use std::{env, fs::File, io::{BufReader, BufRead, Write}, path::Path};

use utfdump_core::{CharData, DataStoreBuf};

const UNICODE_DATA_PATH: &str = "unicode_data_latest.txt";
const OUT_DATA_PATH: &str = "unicode_data_encoded";

fn main() {
    println!("cargo:rerun-if-changed={}", UNICODE_DATA_PATH);
    
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join(OUT_DATA_PATH);

    let data_file = File::open(UNICODE_DATA_PATH)
        .expect("failed to open unicode data file");

    let buf_reader = BufReader::new(data_file);
 
    let mut data = DataStoreBuf::new();
    let mut start_codepoint = None;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let (codepoint, char_data) = CharData::from_row(&line).unwrap();
        
        match start_codepoint {
            Some(start_codepoint_inner) => {
                let prefix = char_data.name()
                    .strip_suffix(", Last>")
                    .expect("expected end of codepoint block");

                let name = {
                    let mut buf = String::with_capacity(prefix.len() + 1);
                    buf.push_str(prefix);
                    buf.push('>');
                    buf
                };

                let char_data = char_data.with_name(&name);

                data.insert(char_data, start_codepoint_inner..(codepoint + 1))
                    .unwrap();

                start_codepoint = None;
            },

            None => {
                if char_data.name().ends_with(", First>") {
                    start_codepoint = Some(codepoint);
                } else {
                    data.insert(char_data, codepoint..(codepoint + 1))
                        .unwrap();
                }
            },
        }
    }

    let (strings_len, [strings, data]) = data
        .as_ref_type()
        .to_bytes()
        .unwrap();

    let mut out_file = File::create(&out_path)
        .expect("failed to open output file");

    out_file.write_all(&strings_len).unwrap();
    out_file.write_all(strings).unwrap();
    out_file.write_all(data).unwrap();

    drop(out_file);
}
