use core::iter::Peekable;

pub trait ToByte {
    fn to_byte(self) -> u8;

    fn as_byte(&self) -> u8;
}

impl ToByte for u8 {
    fn to_byte(self) -> u8 {
        self
    }

    fn as_byte(&self) -> u8 {
        *self
    }
}

impl<'a, B> ToByte for &'a B
where
    B: ToByte,
{
    fn to_byte(self) -> u8 {
        <B as ToByte>::as_byte(self)
    }

    fn as_byte(&self) -> u8 {
        <Self as ToByte>::to_byte(*self)
    }
}

pub trait Utf8Decode {
    type Iter: Iterator<Item = Self::Byte>;
    type Byte: ToByte;

    fn decode_utf8(self) -> Utf8Decoder<Self::Iter, Self::Byte>;
}

impl<T, B> Utf8Decode for T
where
    T: IntoIterator<Item = B>,
    B: ToByte,
{
    type Iter = <T as IntoIterator>::IntoIter;
    type Byte = B;

    fn decode_utf8(self) -> Utf8Decoder<Self::Iter, B> {
        Utf8Decoder::new(self.into_iter())
    }
}

// https://encoding.spec.whatwg.org/#utf-8-decoder
pub struct Utf8Decoder<I, B>
where
    I: Iterator<Item = B>,
    B: ToByte,
{
    bytes: Peekable<I>,
}

impl<I, B> Utf8Decoder<I, B>
where
    I: Iterator<Item = B>,
    B: ToByte,
{
    fn new(bytes: I) -> Self {
        Self {
            bytes: bytes.peekable(),
        }
    }
}

impl<I, B> Iterator for Utf8Decoder<I, B>
where
    I: Iterator<Item = B>,
    B: ToByte,
{
    type Item = Result<char, Utf8Error>;

    fn next(&mut self) -> Option<Self::Item> {
        const DEFAULT_BOUNDARIES: (u8, u8) = (0x80, 0xbf);
        
        // Keep track of the bytes we have seen so far, so that if there is an error we can return
        // the problematic bytes. There is no need for a variable to store the number of bytes we
        // have put into this array, since we can always work it out from other sources.
        let mut bytes_seen = [0u8; 4];

        let mut codepoint: u32;
        let bytes_needed: u8;
        let mut lower_boundary: u8;
        let mut upper_boundary: u8;

        let first_byte = self.bytes.next()?.to_byte();
        bytes_seen[0] = first_byte;

        match first_byte {
            byte @ 0x00..=0x7f => {
                return Some(Ok(char::from(byte)));
            },

            byte @ 0xc2..=0xdf => {
                bytes_needed = 1;
                codepoint = u32::from(byte & 0x1f) << 6;
                (lower_boundary, upper_boundary) = DEFAULT_BOUNDARIES;
            },

            byte @ 0xe0..=0xef => {
                bytes_needed = 2;
                codepoint = u32::from(byte & 0x0f) << 12;
                (lower_boundary, upper_boundary) = match byte {
                    0xe0 => (0xa0, 0xbf),
                    0xed => (0x80, 0x9f),
                    _ => DEFAULT_BOUNDARIES,
                };
            },

            byte @ 0xf0..=0xf4 => {
                bytes_needed = 3;
                codepoint = u32::from(byte & 0x07) << 18;
                (lower_boundary, upper_boundary) = match byte {
                    0xf0 => (0x90, 0xbf),
                    0xf4 => (0x80, 0x8f),
                    _ => DEFAULT_BOUNDARIES,
                };
            },

            _ => {
                return Some(Err(Utf8Error {
                    bad_bytes: bytes_seen,
                    num_bad_bytes: 1,
                    num_consumed_bad_bytes: 1,
                }));
            },
        }

        for i in 0..bytes_needed {
            // Peek the byte rather than consuming it; the specification says we should not consume
            // the byte here if it is not between the upper and lower boundaries.
            let byte = match self.bytes.peek() {
                Some(byte) => byte.as_byte(),
                None => return Some(Err(Utf8Error {
                    bad_bytes: bytes_seen,
                    num_bad_bytes: usize::from(i) + 1,
                    num_consumed_bad_bytes: usize::from(i),
                })),
            };

            bytes_seen[usize::from(i) + 1] = byte;
            
            if !(lower_boundary..=upper_boundary).contains(&byte) {
                return Some(Err(Utf8Error {
                    bad_bytes: bytes_seen,
                    num_bad_bytes: usize::from(i) + 2,
                    num_consumed_bad_bytes: usize::from(i) + 1,
                }));
            }

            // Consume the byte we peeked.
            self.bytes.next();

            (lower_boundary, upper_boundary) = DEFAULT_BOUNDARIES;

            // OR the 6 least significant bits into the codepoint.
            codepoint |= u32::from(byte & 0x3f) << (6 * (bytes_needed - i - 1));
        }

        let codepoint = char::try_from(codepoint)
            .unwrap();

        Some(Ok(codepoint))
    }
}

pub struct Utf8Error {
    bad_bytes: [u8; 4],
    num_bad_bytes: usize,
    num_consumed_bad_bytes: usize,
}

impl Utf8Error {
    pub fn bytes(&self) -> &[u8] {
        &self.bad_bytes[..self.num_bad_bytes]
    }

    // FIXME: return some type with u8 array + length
    pub fn into_parts(self) -> ([u8; 4], usize, usize) {
        (self.bad_bytes, self.num_bad_bytes, self.num_consumed_bad_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::Utf8Decode;

    #[derive(PartialEq, Eq, Debug)]
    struct Invalid;

    const INVALID: Result<char, Invalid> = Err(Invalid);

    #[test]
    fn test_utf8_decoder() {
        assert_decodes_to(&[
            0x68, 0x65, 0x6c, 0x6c, 0x6f
        ], &[
            Ok('h'), Ok('e'), Ok('l'), Ok('l'), Ok('o')
        ]);
    
        assert_decodes_to(&[
            0xce, 0xba, 0xe1, 0xbd, 0xb9, 0xcf, 0x83, 0xce, 0xbc, 0xce, 0xb5
        ], &[
            Ok('κ'), Ok('ό'), Ok('σ'), Ok('μ'), Ok('ε')
        ]);

        assert_decodes_to(&[
            0xf0, 0x9f, 0x8f, 0xb3, 0xef, 0xb8, 0x8f, 0xe2, 0x80, 0x8d, 0xe2, 0x9a, 0xa7, 0xef,
            0xb8, 0x8f
        ], &[
            Ok('\u{1f3f3}'), Ok('\u{fe0f}'), Ok('\u{200d}'), Ok('\u{26a7}'), Ok('\u{fe0f}')
        ]);

        assert_decodes_to(&[
            0xce, 0x61
        ], &[
            INVALID, Ok('a')
        ]);

        assert_decodes_to(&[
            0xce, 0xc2
        ], &[
            INVALID, INVALID
        ]);

        assert_decodes_to(&[
            0x80
        ], &[
            INVALID
        ]);

        assert_decodes_to(&[
            0x80, 0x80
        ], &[
            INVALID, INVALID
        ]);

        assert_decodes_to(&[
            0xed, 0x86, 0xad, 0xed, 0xba, 0xad
        ], &[
            Ok('\u{d1ad}'), INVALID, INVALID, INVALID
        ]);
    }

    fn assert_decodes_to(bytes: &[u8], expected: &[Result<char, Invalid>]) {
        let mut decoded = bytes.decode_utf8();

        for expected_char in expected {
            let decoded_char = decoded.next().map(|res| res.map_err(|_| Invalid));
            assert_eq!(decoded_char.as_ref(), Some(expected_char));
        }

        assert!(decoded.next().is_none());
    }
}
