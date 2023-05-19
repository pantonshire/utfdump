use std::{fmt, error, str, ops::Deref};

/// A view into a [`StringTableBuf`](StringTableBuf). The table stores a collection of strings
/// contiguously, with each string being prefixed by its length in bytes.
#[repr(transparent)]
pub struct StringTable {
    bytes: [u8],
}

impl StringTable {
    pub fn from_bytes(bytes: &[u8]) -> &Self {
        // SAFETY:
        // `StringTable` uses `repr(transparent)`, so it has the same memory layout as `[u8]`.
        unsafe { &*(bytes as *const [u8] as *const Self) }
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Attempt to retrieve the string at the given byte offset in the table. The given index must
    /// be the start of a table entry; providing any other index may result in an error or an
    /// unintended string.
    /// 
    /// Note that the string table does not have a sure-fire mechanism for detecting whether the
    /// given index is valid, so providing an invalid index may not always result in an error; the
    /// bytes starting at the invalid index may be incorrectly interpreted as a valid table entry.
    /// However, this will never result in unsoundness, and thus the function is not marked as
    /// unsafe; it is checked that the resulting string is valid UTF-8.
    pub fn get(&self, index: u32) -> Option<&str> {
        let index = usize::try_from(index).ok()?;
        let len = *self.bytes.get(index)?;
        let bytes = self.bytes.get((index + 1)..(index + 1 + usize::from(len)))?;
        str::from_utf8(bytes).ok()
    }
}

/// An owned [`StringTable`](StringTable). Stores a collection of strings contiguously, with each
/// string being prefixed by its length in bytes.
pub struct StringTableBuf {
    buf: Vec<u8>,
}

impl StringTableBuf {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// Append the given string to the table, returning the byte offset in the table at which it
    /// was stored. This byte offset can then be used to retrieve the string from the table later,
    /// via `StringTable::get`.
    pub fn push(&mut self, s: &str) -> Result<u32, StringTableBufError> {
        let len = u8::try_from(s.len())
            .map_err(|_| StringTableBufError::StringTooLong)?;

        let index = u32::try_from(self.buf.len())
            .map_err(|_| StringTableBufError::OutOfCapacity)?;

        self.buf.try_reserve(s.len() + 1)
            .map_err(|_| StringTableBufError::OutOfCapacity)?;

        self.buf.push(len);
        self.buf.extend(s.bytes());
        
        Ok(index)
    }
}

impl AsRef<StringTable> for StringTableBuf {
    fn as_ref(&self) -> &StringTable {
        self
    }
}

impl Deref for StringTableBuf {
    type Target = StringTable;

    fn deref(&self) -> &Self::Target {
        StringTable::from_bytes(&self.buf)
    }
}

#[derive(Debug)]
pub enum StringTableBufError {
    StringTooLong,
    OutOfCapacity,
}

impl fmt::Display for StringTableBufError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StringTooLong => write!(f, "string too long to add to table"),
            Self::OutOfCapacity => write!(f, "string table out of capacity"),
        }
    }
}

impl error::Error for StringTableBufError {}
