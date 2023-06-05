use core::{fmt, mem, slice, str};

use tap::Pipe;

use crate::character::{
    CharData,
    Category,
    BidiCategory,
    OptionalDecompKind,
    CombiningClass,
    DecompMapping,
};

const MAGIC_NUMBER: [u8; 8] = *b"UTFDUMP!";

pub type StaticUnicodeData = UnicodeData<'static>;

#[derive(Clone, Copy)]
pub struct UnicodeData<'a> {
    group_table: GroupTable<'a>,
    char_table: CharTable<'a>,
    string_table: StringTable<'a>,
}

const UNICODE_DATA_BYTES: &[u8] = include_bytes!(
    concat!(env!("OUT_DIR"), "/unicode_data_encoded")
);

impl UnicodeData<'static> {
    pub fn new() -> Result<Self, UnicodeDataError> {
        Self::from_bytes(UNICODE_DATA_BYTES)
    }
}

impl<'a> UnicodeData<'a> {
    pub(crate) fn from_bytes(bs: &'a [u8]) -> Result<Self, UnicodeDataError> {
        let mut bs = ByteStream(bs);

        if bs.consume(MAGIC_NUMBER.len())? != MAGIC_NUMBER {
            return Err(UnicodeDataError::InvalidHeader);
        }

        let group_table_len = bs.consume_4_byte_len()?;
        let char_table_len = bs.consume_4_byte_len()?;
        let string_table_len = bs.consume_4_byte_len()?;

        let group_table = bs.consume(group_table_len)?.pipe(GroupTable::new)?;
        let char_table = bs.consume(char_table_len)?.pipe(CharTable::new)?;
        let string_table = bs.consume(string_table_len)?.pipe(StringTable::new);
        
        bs.check_empty()?;
        
        Ok(Self { group_table, char_table, string_table })
    }

    pub fn get(self, codepoint: u32) -> Option<CharData<'a>> {
        let entry = self.char_entry_for(codepoint)?;

        let flags_and_categories = entry.flags_and_categories.to_u16();
        let category = Category::decode((flags_and_categories & 0x1f) as u8)?;
        let bidi = BidiCategory::decode(((flags_and_categories >> 5) & 0x1f) as u8)?;
        let decomp_kind = OptionalDecompKind::decode(((flags_and_categories >> 10) & 0x1f) as u8)?;
        let mirrored = (flags_and_categories >> 15) != 0;

        let name = self.string_table.get_u24_le(entry.name)?;
        
        let decomp_value = self.string_table.get_u24_le(entry.decomp);
        let decomp = match (decomp_kind, decomp_value) {
            (OptionalDecompKind::None, _) | (_, None) => None,
            (OptionalDecompKind::Anon, Some(value)) => {
                Some(DecompMapping::new(None, value))
            },
            (OptionalDecompKind::Named(kind), Some(value)) => {
                Some(DecompMapping::new(Some(kind), value))
            },
        };
        
        let numeric = self.string_table.get_u24_le(entry.numeric);
        let old_name = self.string_table.get_u24_le(entry.old_name); 
        let comment = self.string_table.get_u24_le(entry.comment);
        let uppercase = self.string_table.get_u24_le(entry.uppercase);
        let lowercase = self.string_table.get_u24_le(entry.lowercase);
        let titlecase = self.string_table.get_u24_le(entry.titlecase);

        let combining = CombiningClass(entry.combining);

        let decimal_digit = match entry.digit & 0xf {
            0xf => None,
            n => Some(n),
        };

        let digit = match (entry.digit >> 4) & 0xf {
            0xf => None,
            n => Some(n),
        };

        Some(CharData {
            codepoint,
            name,
            category,
            combining,
            bidi,
            decomp,
            decimal_digit,
            digit,
            numeric,
            mirrored,
            old_name,
            comment,
            uppercase,
            lowercase,
            titlecase,
        })
    }

    fn char_entry_for(self, codepoint: u32) -> Option<&'a CharTableEntry> {
        let index = self.group_table
            .char_table_index_for(codepoint)?
            .pipe(usize::try_from)
            .ok()?;

        self.char_table.get(index)
    }
}

#[derive(Clone, Copy, Debug)]
struct GroupTable<'a> {
    entries: &'a [GroupTableEntry],
}

impl<'a> GroupTable<'a> {
    fn new(bs: &'a [u8]) -> Result<Self, UnicodeDataError> {
        if bs.len() % GroupTableEntry::SIZE != 0 {
            return Err(UnicodeDataError::InvalidTableSize);
        }

        let num_entries = bs.len() / GroupTableEntry::SIZE;

        // SAFETY:
        // - The pointer is valid for reads of `num_entries * mem::size_of::<GroupTableEntry>()`
        //   bytes; `num_entries = bs.len() / mem::size_of::<GroupTableEntry>()`, so
        //   `num_entries * mem::size_of::<GroupTableEntry>() <= bs.len()` (the inequality is due
        //   to flooring integer division), and clearly a pointer to `bs` is valid for reads of
        //   <= `bs.len()` bytes.
        //
        // - `u8` and `GroupTableEntry` both have an alignment of 1 (since `GroupTableEntry` is
        //    packed), so the pointer is correctly aligned.
        //
        // - The pointer points to `num_entries` consecutive properly-initialised `GroupTableEntry`
        //   values, as `bs` contains initialised data and `GroupTableEntry` consists only of
        //   arrays of `u8` of varying sizes, for which any bit pattern is valid.
        //
        // - Since we obtained the pointer from an immutable reference `bs`, the data cannot be
        //   mutated by safe code for the duration of the lifetime `'a`.
        //
        // - The total length of the slice does not exceed `isize::MAX`, since it is no larger
        //   than `bs` which is a valid slice and therefore no larger than `isize::MAX`.
        let entries = unsafe {
            slice::from_raw_parts(
                bs.as_ptr() as *const GroupTableEntry,
                num_entries
            )
        };

        Ok(Self { entries })
    }

    // TODO: compare performance of binary search to linear search
    // TODO: fast path for characters before the first group
    fn char_table_index_for(self, codepoint: u32) -> Option<u32> {
        let mut entries = self.entries;
        let mut offset = 0;

        loop {
            if entries.len() == 0 {
                break codepoint.checked_sub(offset);
            }

            let midpoint = entries.len() / 2;
            let entry = &entries[midpoint];
            let start = entry.start.to_u32();
            let end = entry.end.to_u32();
            let total_len_before = entry.total_len_before.to_u32();

            if start <= codepoint && codepoint <= end {
                match entry.kind {
                    GROUP_KIND_USE_PREV_VALUE => {
                        // This group uses the same character data as the codepoint immediately
                        // before the group start (`start - 1`). Subtract `total_len_before`, which
                        // is the total length of all groups before this group, from `start - 1` to
                        // find the index of its character data in the character table.
                        break start
                            .checked_sub(1)
                            .expect("first codepoint for a USE_PREV_VALUE group should always be at least 1")
                            .checked_sub(total_len_before)
                            .expect("computed character data index should not underflow")
                            .pipe(Some)
                    },
                    
                    // If the codepoint is in a group which is not `USE_PREV_VALUE`, we take it to
                    // be a codepoint with no associated character data.
                    _ => break None,
                }
            } else if codepoint > end {
                // Since the `end` is inclusive, the length of the group is calculated as
                // `(end - start) + 1`.
                let group_len = end
                    .checked_sub(start)
                    .expect("group start should be less than or equal to the group end")
                    .checked_add(1)
                    .expect("group length should not overflow a u32");

                // `total_len_before` is the total length of all groups before this group, so we
                // can calculate the total length of all groups up to and including this group by
                // adding `group_len` to it. We assign this to `offset` because this is the group
                // with the largest `end` value that is less than the codepoint, and is therefore
                // the offset that should be used to calculate the character table index for this
                // codepoint in the event that there are no groups with a larger `end` value less
                // than the codepoint and the codepoint is not contained in a group.
                offset = total_len_before
                    .checked_add(group_len)
                    .expect("cumulative group length should not overflow a u32");

                entries = &entries[(midpoint + 1)..];
            } else {
                entries = &entries[..midpoint];
            }
        }
    }
}

const GROUP_KIND_USE_PREV_VALUE: u8 = 1;

#[derive(Debug)]
#[repr(C, packed)]
struct GroupTableEntry {
    start: U32Le,
    end: U32Le,
    total_len_before: U32Le,
    kind: u8,
}

impl GroupTableEntry {
    const SIZE: usize = mem::size_of::<Self>();
}

#[derive(Debug)]
#[derive(Clone, Copy)]
struct CharTable<'a> {
    entries: &'a [CharTableEntry],
}

impl<'a> CharTable<'a> {
    fn new(bs: &'a [u8]) -> Result<Self, UnicodeDataError> {
        if bs.len() % CharTableEntry::SIZE != 0 {
            return Err(UnicodeDataError::InvalidTableSize);
        }

        let num_entries = bs.len() / CharTableEntry::SIZE;

        // SAFETY:
        // - The pointer is valid for reads of `num_entries * mem::size_of::<CharTableEntry>()`
        //   bytes; `num_entries = bs.len() / mem::size_of::<CharTableEntry>()`, so
        //   `num_entries * mem::size_of::<CharTableEntry>() <= bs.len()` (the inequality is due
        //   to flooring integer division), and clearly a pointer to `bs` is valid for reads of
        //   <= `bs.len()` bytes.
        //
        // - `u8` and `CharTableEntry` both have an alignment of 1 (since `CharTableEntry` is
        //    packed), so the pointer is correctly aligned.
        //
        // - The pointer points to `num_entries` consecutive properly-initialised `CharTableEntry`
        //   values, as `bs` contains initialised data and `CharTableEntry` consists only of
        //   arrays of `u8` of varying sizes, for which any bit pattern is valid.
        //
        // - Since we obtained the pointer from an immutable reference `bs`, the data cannot be
        //   mutated by safe code for the duration of the lifetime `'a`.
        //
        // - The total length of the slice does not exceed `isize::MAX`, since it is no larger
        //   than `bs` which is a valid slice and therefore no larger than `isize::MAX`.
        let entries = unsafe {
            slice::from_raw_parts(
                bs.as_ptr() as *const CharTableEntry,
                num_entries
            )
        };
        
        Ok(Self { entries })
    }

    fn get(self, i: usize) -> Option<&'a CharTableEntry> {
        self.entries.get(i)
    }
}

#[derive(Debug)]
#[repr(C, packed)]
struct CharTableEntry {
    flags_and_categories: U16Le,
    name: U24Le,
    decomp: U24Le,
    numeric: U24Le,
    old_name: U24Le,
    comment: U24Le,
    uppercase: U24Le,
    lowercase: U24Le,
    titlecase: U24Le,
    combining: u8,
    digit: u8,
}

impl CharTableEntry {
    const SIZE: usize = mem::size_of::<Self>();
}

#[derive(Clone, Copy)]
struct StringTable<'a> {
    inner: &'a [u8],
}

impl<'a> StringTable<'a> {
    fn new(bs: &'a [u8]) -> Self {
        Self { inner: bs }
    }

    fn get(self, i: usize) -> Option<&'a str> {
        let len = usize::from(*self.inner.get(i)?);
        
        let str_start = i.checked_add(1)?;
        let str_end = str_start.checked_add(len)?;

        self.inner.get(str_start..str_end)
            .and_then(|s| str::from_utf8(s).ok())
    }

    fn get_u24_le(self, i: U24Le) -> Option<&'a str> {
        const NIL_INDEX_PATTERN: [u8; 3] = [0xff; 3];
        
        if i.0 == NIL_INDEX_PATTERN {
            return None;
        }

        i.to_usize().and_then(|i| self.get(i))
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct U16Le([u8; 2]);

impl U16Le {
    fn to_u16(self) -> u16 {
        u16::from_le_bytes(self.0)
    }
}

impl fmt::Debug for U16Le {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_u16(), f)
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct U24Le([u8; 3]);

impl U24Le {
    fn to_u32(self) -> u32 {
        let mut buf = [0u8; 4];
        (&mut buf[..3]).copy_from_slice(&self.0);
        u32::from_le_bytes(buf)
    }

    fn to_usize(self) -> Option<usize> {
        usize::try_from(self.to_u32()).ok()
    }
}

impl fmt::Debug for U24Le {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_u32(), f)
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct U32Le([u8; 4]);

impl U32Le {
    fn to_u32(self) -> u32 {
        u32::from_le_bytes(self.0)
    }
}

impl fmt::Debug for U32Le {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_u32(), f)
    }
}

struct ByteStream<'a>(&'a [u8]);

impl<'a> ByteStream<'a> {
    fn consume(&mut self, n: usize) -> Result<&'a [u8], UnicodeDataError> {
        if n > self.0.len() {
            return Err(UnicodeDataError::InsufficientBytes);
        }

        let consumed = &self.0[..n];
        self.0 = &self.0[n..];
        Ok(consumed)
    }

    fn consume_4_byte_len(&mut self) -> Result<usize, UnicodeDataError> {
        self.consume(4)?
            .pipe(<[u8; 4]>::try_from)
            .unwrap()
            .pipe(u32::from_le_bytes)
            .pipe(usize::try_from)
            .map_err(|_| UnicodeDataError::OutOfBounds)
    }

    fn check_empty(&self) -> Result<(), UnicodeDataError> {
        self.0
            .is_empty()
            .then_some(())
            .ok_or(UnicodeDataError::LeftoverBytes)
    }
}

#[derive(Debug)]
pub enum UnicodeDataError {
    InvalidHeader,
    InsufficientBytes,
    OutOfBounds,
    LeftoverBytes,
    InvalidTableSize,
}

impl fmt::Display for UnicodeDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHeader => write!(f, "invalid header"),
            Self::InsufficientBytes => write!(f, "fewer bytes than expected"),
            Self::OutOfBounds => write!(f, "index out of bounds"),
            Self::LeftoverBytes => write!(f, "unexpected bytes found after expected end of data"),
            Self::InvalidTableSize => write!(f, "invalid table size"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UnicodeData;

    #[test]
    fn test_data_decode() {
        let data = UnicodeData::new().unwrap();

        assert_eq!(data.get(0x0).unwrap().name(), "<control>");
        assert_eq!(data.get(0x0).unwrap().unicode_1_name(), Some("NULL"));
        assert_eq!(data.get(0x1).unwrap().name(), "<control>");
        assert_eq!(data.get(0x1).unwrap().unicode_1_name(), Some("START OF HEADING"));
        assert_eq!(data.get(0x2).unwrap().name(), "<control>");
        assert_eq!(data.get(0x2).unwrap().unicode_1_name(), Some("START OF TEXT"));

        assert_eq!(data.get(0x377).unwrap().name(), "GREEK SMALL LETTER PAMPHYLIAN DIGAMMA");
        assert!(data.get(0x378).is_none());
        assert!(data.get(0x379).is_none());
        assert_eq!(data.get(0x37a).unwrap().name(), "GREEK YPOGEGRAMMENI");

        assert_eq!(data.get(0x33ff).unwrap().name(), "SQUARE GAL");
        assert_eq!(data.get(0x3400).unwrap().name(), "CJK Ideograph Extension A");
        assert_eq!(data.get(0x3401).unwrap().name(), "CJK Ideograph Extension A");
        assert_eq!(data.get(0x3402).unwrap().name(), "CJK Ideograph Extension A");
        assert_eq!(data.get(0x4dbe).unwrap().name(), "CJK Ideograph Extension A");
        assert_eq!(data.get(0x4dbf).unwrap().name(), "CJK Ideograph Extension A");
        assert_eq!(data.get(0x4dc0).unwrap().name(), "HEXAGRAM FOR THE CREATIVE HEAVEN");

        assert_eq!(data.get(0x1039f).unwrap().name(), "UGARITIC WORD DIVIDER");
    }
}
