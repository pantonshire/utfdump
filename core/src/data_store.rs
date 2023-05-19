use std::{collections::{HashMap, hash_map}, error, fmt, ops::Range};

use crate::{
    char_data::{CharData, Category, CombiningClass},
    string_table::{StringTableBufError, StringTableBuf, StringTable},
};

const DATA_ENTRY_SIZE: usize = 8;

const DATA_INIT_FLAG: u8 = 1;
const DATA_REPEATED_FLAG: u8 = 2;

fn encode_char_data(
    name_index: u32,
    category: Category,
    combining_class: CombiningClass,
    repeated: bool
) -> [u8; DATA_ENTRY_SIZE]
{
    let mut buf = [0u8; DATA_ENTRY_SIZE];

    buf[0] |= DATA_INIT_FLAG;

    if repeated {
        buf[0] |= DATA_REPEATED_FLAG;
    }

    buf[1..5].copy_from_slice(&name_index.to_le_bytes());
    buf[5] = category.byte_repr();
    buf[6] = combining_class.0;

    buf
}

fn decode_char_data(bytes: [u8; DATA_ENTRY_SIZE])
    -> Option<(u32, Category, CombiningClass, bool)>
{
    let flags = bytes[0];
    
    if flags & DATA_INIT_FLAG == 0 {
        return None;
    }

    let name_index = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
    let category = Category::from_byte(bytes[5])?;
    let combining_class = CombiningClass(bytes[6]);
    let repeated = flags & DATA_REPEATED_FLAG != 0;

    Some((name_index, category, combining_class, repeated))
}

pub struct DataStoreBuf {
    data: Vec<u8>,
    strings: StringTableBuf,
    strings_map: HashMap<String, u32>,
}

impl DataStoreBuf {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            strings: StringTableBuf::new(),
            strings_map: HashMap::new(),
        }
    }

    pub fn as_ref_type(&self) -> DataStore {
        DataStore { data: &self.data, strings: &*self.strings }
    }

    pub fn insert(&mut self, char_data: CharData, range: Range<u32>) -> Result<(), DataBufError> {
        if range.is_empty() {
            return Ok(());
        }

        let repeated = range.end
            .checked_sub(range.start)
            .map(|len| len > 1)
            .unwrap_or(false);

        let range = {
            let start = usize::try_from(range.start)
                .map_err(|_| DataBufError::DataOutOfCapacity)?
                .checked_mul(DATA_ENTRY_SIZE)
                .ok_or(DataBufError::DataOutOfCapacity)?;
            let end = usize::try_from(range.end)
                .map_err(|_| DataBufError::DataOutOfCapacity)?
                .checked_mul(DATA_ENTRY_SIZE)
                .ok_or(DataBufError::DataOutOfCapacity)?;
            start..end
        };

        if let Some(extra_capacity_needed) = range.end.checked_sub(self.data.len()) {
            self.data.try_reserve(extra_capacity_needed)
                .map_err(|_| DataBufError::DataOutOfCapacity)?;
        }

        let name_index = self.add_string(char_data.name().to_owned())?;

        let encoded_char_data = encode_char_data(
            name_index,
            char_data.category(),
            char_data.combining_class(),
            repeated
        );

        if self.data.len() < range.end {
            // Using 0 means that the DATA_INIT_FLAG won't be set, so these won't be valid entries.
            self.data.resize(range.end, 0);
        }

        for i in range.step_by(DATA_ENTRY_SIZE) {
            self.data[i..(i + DATA_ENTRY_SIZE)].copy_from_slice(&encoded_char_data);
        }

        Ok(())
    }

    fn add_string(&mut self, name: String) -> Result<u32, DataBufError> {
        match self.strings_map.entry(name) {
            hash_map::Entry::Occupied(entry) => Ok(*entry.get()),
            hash_map::Entry::Vacant(entry) => {
                let index = self.strings.push(entry.key())?;
                entry.insert(index);
                Ok(index)
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct DataStore<'a> {
    data: &'a [u8],
    strings: &'a StringTable,
}

impl<'a> DataStore<'a> {
    pub fn get(self, codepoint: char) -> Option<CharData<'a>> {
        let index = usize::try_from(u32::from(codepoint)).ok()?;
        let start = index.checked_mul(DATA_ENTRY_SIZE)?;
        let end = start.checked_add(DATA_ENTRY_SIZE)?;
        let encoded = self.data.get(start..end)?;
        let (name_index, category, ccc, _repeated) = decode_char_data(encoded.try_into().unwrap())?;
        let name = self.strings.get(name_index)?;
        Some(CharData::from_parts(name, category, ccc))
    }

    pub fn to_bytes(self) -> Option<([u8; 4], [&'a [u8]; 2])> {
        let strings = self.strings.to_bytes();
        let strings_len = u32::try_from(strings.len())
            .ok()?
            .to_le_bytes();
        Some((strings_len, [strings, self.data]))
    }

    pub fn from_bytes(bytes: &'a [u8]) -> Option<Self> {
        let strings_len = usize::try_from(
            u32::from_le_bytes(bytes.get(..4)?.try_into().unwrap())
        ).ok()?;
        let strings = StringTable::from_bytes(bytes.get(4..(4 + strings_len))?);
        let data = bytes.get((4 + strings_len)..)?;
        Some(Self { data, strings })
    }
}

#[derive(Debug)]
pub enum DataBufError {
    DataOutOfCapacity,
    StringsMapOutOfCapacity,
    StringTable(StringTableBufError),
}

impl fmt::Display for DataBufError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DataOutOfCapacity => write!(f, "data buf out of capacity"),
            Self::StringsMapOutOfCapacity => write!(f, "strings map out of capacity"),
            Self::StringTable(err) => write!(f, "string table error: {}", err),
        }
    }
}

impl error::Error for DataBufError {}

impl From<StringTableBufError> for DataBufError {
    fn from(err: StringTableBufError) -> Self {
        Self::StringTable(err)
    }
}
