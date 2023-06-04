pub mod character;
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
    fn test_data_decode() {
        let data = unicode_data::UnicodeData::from_bytes(UNICODE_DATA_BYTES)
            .unwrap();

        assert_eq!(data.get(0x0).unwrap().name(), "<control>");
        assert_eq!(data.get(0x0).unwrap().old_name(), Some("NULL"));
        assert_eq!(data.get(0x1).unwrap().name(), "<control>");
        assert_eq!(data.get(0x1).unwrap().old_name(), Some("START OF HEADING"));
        assert_eq!(data.get(0x2).unwrap().name(), "<control>");
        assert_eq!(data.get(0x2).unwrap().old_name(), Some("START OF TEXT"));

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
