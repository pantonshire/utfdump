use std::sync::OnceLock;

use utfdump::{UnicodeData, CombiningClass, CharData};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct WbgCharData(CharData<'static>);

#[wasm_bindgen]
impl WbgCharData {
    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.0.name().to_owned()
    }

    #[wasm_bindgen]
    pub fn encoded_utf8(&self) -> Option<EncodedCodepoint> {
        let c = char::try_from(self.0.codepoint()).ok()?;
        let mut buf = [0u8; 4];
        let len = c.encode_utf8(&mut buf).len() as u8;
        Some(EncodedCodepoint::new(buf, len))
    }

    #[wasm_bindgen]
    pub fn encoded_utf16_le(&self) -> Option<EncodedCodepoint> {
        let (word_buf, num_words) = self.encoded_utf16()?;
        let mut byte_buf = [0u8; 4];
        for (i, word) in word_buf.iter().take(usize::from(num_words)).enumerate() {
            let le_bytes = word.to_le_bytes();
            byte_buf[(i * 2)..(i * 2 + 2)].copy_from_slice(&le_bytes);
        }
        Some(EncodedCodepoint::new(byte_buf, num_words * 2))
    }

    fn encoded_utf16(&self) -> Option<([u16; 2], u8)> {
        let c = char::try_from(self.0.codepoint()).ok()?;
        let mut word_buf = [0u16; 2];
        let num_words = c.encode_utf16(&mut word_buf).len() as u8;
        Some((word_buf, num_words))
    }

    #[wasm_bindgen]
    pub fn category(&self) -> String {
        self.0.category().abbreviation().to_owned()
    }

    #[wasm_bindgen]
    pub fn category_full(&self) -> String {
        self.0.category().full_name().to_owned()
    }

    #[wasm_bindgen]
    pub fn combining_class(&self) -> u8 {
        self.0.combining_class().0
    }

    #[wasm_bindgen]
    pub fn bidi(&self) -> String {
        self.0.bidi_category().abbreviation().to_owned()
    }

    #[wasm_bindgen]
    pub fn bidi_full(&self) -> String {
        self.0.bidi_category().full_name().to_owned()
    }

    #[wasm_bindgen]
    pub fn numeric_value(&self) -> Option<String> {
        self.0.numeric_value().map(ToOwned::to_owned)
    }

    #[wasm_bindgen]
    pub fn mirrored(&self) -> bool {
        self.0.mirrored()
    }

    #[wasm_bindgen]
    pub fn decomp_string(&self) -> Option<String> {
        self.0.decomp_mapping().map(|d| d.value().to_owned())
    }

    #[wasm_bindgen]
    pub fn uppercase_string(&self) -> Option<String> {
        self.0.uppercase().map(ToOwned::to_owned)
    }

    #[wasm_bindgen]
    pub fn lowercase_string(&self) -> Option<String> {
        self.0.lowercase().map(ToOwned::to_owned)
    }

    #[wasm_bindgen]
    pub fn titlecase_string(&self) -> Option<String> {
        self.0.titlecase().map(ToOwned::to_owned)
    }
}

#[wasm_bindgen]
pub fn combining_class_name(combining_class: u8) -> Option<String> {
    CombiningClass(combining_class)
        .name()
        .map(ToOwned::to_owned)
}

static UNICODE_DATA: OnceLock<UnicodeData> = OnceLock::new();

#[wasm_bindgen]
pub fn codepoint_char_data(codepoint: u32) -> Option<WbgCharData> {
    let unicode_data = UNICODE_DATA.get_or_init(|| {
        UnicodeData::new()
            .unwrap()
    });
    
    unicode_data
        .get(codepoint)
        .map(WbgCharData)
}

#[wasm_bindgen]
pub struct EncodedCodepoint {
    // `wasm-bindgen` unfortunately does not support arrays :(
    pub b0: u8,
    pub b1: u8,
    pub b2: u8,
    pub b3: u8,
    pub len: u8,
}

impl EncodedCodepoint {
    fn new([b0, b1, b2, b3]: [u8; 4], len: u8) -> Self {
        Self { b0, b1, b2, b3, len }
    }
}
