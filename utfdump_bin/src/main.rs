use std::{fmt, io::{self, Read}};

use clap::Parser;
use libshire::strings::CappedString;
use tabled::{Tabled, Table, Style};

use utfdump_core::{chardata::Category, encoded::Data};

const UNICODE_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/unicode_data_encoded"));

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, action)]
    full_category_names: bool,
}

fn main() {
    let args = Args::parse();

    let data = Data::<'static>::from_bytes(UNICODE_DATA).unwrap();

    let input = {
        let mut buf = Vec::<u8>::new();
        let stdin = io::stdin();
        let mut guard = stdin.lock();
        guard.read_to_end(&mut buf)
            .expect("failed to read stdin");
        // TODO: just skip over invalid utf-8 characters
        String::from_utf8(buf)
            .expect("invalid utf-8")
    };

    let rows = input
        .chars()
        .map(|c| {
            let mut name = Optional::None;
            let mut category = Optional::None;
            let mut char_combining_class = Optional::None;
            
            let mut combining = false;

            if let Some(char_data) = data.get(c as u32) {
                name = Optional::Some(char_data.name());
                category = Optional::Some(DisplayCategory {
                    category: char_data.category(),
                    full_name: args.full_category_names,
                });

                let ccc = char_data.ccc();
                char_combining_class = Optional::Some(ccc);
                combining = ccc != 0;
            }

            let display_char = {
                let mut buf = CappedString::empty();
                if combining {
                    buf.push_truncating('\u{25cc}');
                }
                buf.push_truncating(c);
                buf
            };

            OutRow {
                display_char,
                codepoint: Codepoint(c),
                utf_8_bytes: Utf8Bytes(c),
                name,
                category,
                char_combining_class,
            }
        });

    let table = Table::new(rows)
        .with(Style::modern());

    println!("{}", table);
}

#[derive(Tabled)]
struct OutRow {
    #[tabled(rename = "")]
    display_char: CappedString<8>, 
    #[tabled(rename = "Code")]
    codepoint: Codepoint,
    #[tabled(rename = "UTF-8")]
    utf_8_bytes: Utf8Bytes,
    #[tabled(rename = "Name")]
    name: Optional<&'static str>,
    #[tabled(rename = "Category")]
    category: Optional<DisplayCategory>,
    #[tabled(rename = "Combining")]
    char_combining_class: Optional<u8>,
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
struct Utf8Bytes(char);

impl fmt::Display for Utf8Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0u8; 4];
        let s = self.0.encode_utf8(&mut buf);
        let mut bytes = s.bytes();
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
            write!(f, "{}", self.category.abbr())
        }
    }
}

