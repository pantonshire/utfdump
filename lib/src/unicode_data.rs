use core::{fmt, mem, slice};

use tap::Pipe;

const MAGIC_NUMBER: [u8; 8] = *b"UTFDUMP!";

#[derive(Clone, Copy)]
pub struct UnicodeData<'a> {
    group_table: GroupTable<'a>,
    char_table: CharTable<'a>,
    string_table: StringTable<'a>,
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

    pub(crate) fn chars(self) -> CharTable<'a> {
        self.char_table
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
}

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
pub(crate) struct CharTable<'a> {
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
