#![cfg_attr(not(feature = "std"), no_std)]

pub mod character;
pub mod unicode_data;
pub mod utf8;

pub use character::{
    BidiCategory,
    Category,
    CharData,
    CombiningClass,
    DecompKind,
    DecompMapping,
};

pub use unicode_data::{StaticUnicodeData, UnicodeData};
