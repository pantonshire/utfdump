use std::{fmt, io::{self, Read}};

use clap::Parser;
use libshire::strings::CappedString;
use tabled::{Tabled, Table, Style};
use utfdump::{CombiningClass, Category, utf8::{Utf8Decode, Utf8Error}, StaticUnicodeData};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Display category names in plain English, rather than using their abbreviated names
    #[clap(short, long, action)]
    full_category_names: bool,
}

fn main() {
    let unicode_data = StaticUnicodeData::new().unwrap();

    let args = Args::parse();

    let input = {
        let mut buf = Vec::<u8>::new();
        let stdin = io::stdin();
        let mut guard = stdin.lock();
        guard.read_to_end(&mut buf)
            .expect("failed to read stdin");
        buf
    };

    let rows = input
        .decode_utf8()
        .map(|c| OutRow::from_char_result(&unicode_data, c, args.full_category_names));

    let table = Table::new(rows)
        .with(Style::modern());

    println!("{}", table);
}

#[derive(Tabled)]
struct OutRow {
    #[tabled(rename = "")]
    display_char: CappedString<8>, 
    #[tabled(rename = "Code")]
    codepoint: Optional<Codepoint>,
    #[tabled(rename = "UTF-8")]
    utf_8_bytes: Utf8Bytes,
    #[tabled(rename = "Name")]
    name: Optional<&'static str>,
    #[tabled(rename = "Category")]
    category: Optional<DisplayCategory>,
    #[tabled(rename = "Combining")]
    char_combining_class: Optional<DisplayCombiningClass>,
}

impl OutRow {
    fn from_char_result(
        unicode_data: &StaticUnicodeData,
        c: Result<char, Utf8Error>,
        full_category_names: bool
    ) -> Self
    {
        match c {
            Ok(c) => Self::from_good_char(unicode_data, c, full_category_names),
            Err(err) => Self::from_bad_char(err),
        }
    }

    fn from_good_char(
        unicode_data: &StaticUnicodeData,
        c: char,
        full_category_names: bool
    ) -> Self
    {
        let mut name = Optional::None;
        let mut category = Optional::None;
        let mut char_combining_class = Optional::None;
        
        let mut combining = false;

        if let Some(char_data) = unicode_data.get(u32::from(c)) {
            name = Optional::Some(char_data.name());
            category = Optional::Some(DisplayCategory {
                category: char_data.category(),
                full_name: full_category_names,
            });

            let ccc = char_data.combining_class();
            char_combining_class = Optional::Some(DisplayCombiningClass { ccc });
            combining = ccc.is_combining();
        }

        let display_char = {
            let mut buf = CappedString::empty();
            if combining {
                buf.push_truncating('\u{25cc}');
            }
            buf.push_truncating(c);
            buf
        };

        Self {
            display_char,
            codepoint: Optional::Some(Codepoint(c)),
            utf_8_bytes: Utf8Bytes::from_char(c),
            name,
            category,
            char_combining_class,
        }
    }

    fn from_bad_char(err: Utf8Error) -> Self {
        let (bad_bytes, _num_bad_bytes, num_consumed_bad_bytes) = err.into_parts();

        Self {
            display_char: CappedString::new_truncating("\u{fffd}"),
            codepoint: Optional::None,
            utf_8_bytes: Utf8Bytes {
                buf: bad_bytes,
                len: num_consumed_bad_bytes,
            },
            name: Optional::Some("<invalid>"),
            category: Optional::None,
            char_combining_class: Optional::None,
        }
    }
}

#[derive(Debug)]
enum Optional<T> {
    Some(T),
    None,
}

impl<T> fmt::Display for Optional<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Some(x) => fmt::Display::fmt(&x, f),
            Self::None => f.write_str("??"),
        }
    }    
}

#[derive(Debug)]
struct Codepoint(char);

impl fmt::Display for Codepoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U+{:04x}", self.0 as u32)
    }
}

#[derive(Debug)]
struct Utf8Bytes {
    buf: [u8; 4],
    len: usize,
}

impl Utf8Bytes {
    fn from_char(c: char) -> Self {
        let mut buf = [0u8; 4];
        let string = c.encode_utf8(&mut buf);
        let len = string.len();
        Self { buf, len }
    }

    fn bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl fmt::Display for Utf8Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes = self.bytes().iter().copied();
        if let Some(b) = bytes.next() {
            write!(f, "0x{:02x}", b)?;
            for b in bytes {
                write!(f, " 0x{:02x}", b)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct DisplayCategory {
    category: Category,
    full_name: bool,
}

impl fmt::Display for DisplayCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.full_name {
            write!(f, "{}", self.category.full_name())
        } else {
            write!(f, "{}", self.category.abbreviation())
        }
    }
}

struct DisplayCombiningClass {
    ccc: CombiningClass,
}

impl fmt::Display for DisplayCombiningClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ccc.name() {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "{}", self.ccc.0),
        }
    }
}
