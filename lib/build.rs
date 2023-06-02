use std::{env, fs::File, io, path::Path};

const COMPRESSED_DATA_PATH: &str = "../unicode_data_encoded.gz";
const OUT_DATA_PATH: &str = "unicode_data_encoded";

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed={}", COMPRESSED_DATA_PATH);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join(OUT_DATA_PATH);
    let out_data_fd = File::create(out_path)?;
    let mut decoder = flate2::write::GzDecoder::new(out_data_fd);
    
    let mut compressed_data_fd = File::open(COMPRESSED_DATA_PATH)?;

    io::copy(&mut compressed_data_fd, &mut decoder)?;
    decoder.finish()?;

    Ok(())
}
