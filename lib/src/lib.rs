pub mod unicode_data;
pub mod utf8;

// pub use utfdump_core::{CharData, Category, CombiningClass};

// use once_cell::sync::Lazy;
// use utfdump_core::data_store::DataStore;

// const UNICODE_DATA_BYTES: &[u8] = include_bytes!(
//     concat!(env!("OUT_DIR"), "/unicode_data_encoded")
// );

// static UNICODE_DATA: Lazy<DataStore> = Lazy::new(|| {
//     DataStore::from_bytes(UNICODE_DATA_BYTES).unwrap()
// });

// pub fn char_data(c: char) -> Option<CharData<'static>> {
//     UNICODE_DATA.get(c)
// }

const UNICODE_DATA_BYTES: &[u8] = include_bytes!(
    concat!(env!("OUT_DIR"), "/unicode_data_encoded")
);

#[cfg(test)]
mod tests {
    use crate::{UNICODE_DATA_BYTES, unicode_data};

    #[test]
    fn test_encoded_data() {
        let data = unicode_data::UnicodeData::from_bytes(UNICODE_DATA_BYTES)
            .unwrap();

        println!("{:#?}", data.groups());
    }
}
