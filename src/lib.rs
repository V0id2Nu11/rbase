mod base16;
mod base32;
mod base64;
pub use base16::Base16;
pub use base32::{ Base32, Base32Hex };
pub use base64::{ Base64, Base64URL };

pub trait Alphabate {
    fn pad() -> char {
        '='
    }
    fn get_index_from_character(character: char) -> Option<u8>;
    fn get_character_from_index(index: u8) -> Option<char>;
}

pub trait Encode {
    fn encode(data: &[u8]) -> String;
}

pub trait Decode {
    fn decode(code: &str) -> Vec<u8>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // test data
    const BASE16_RESULT: [(&str, &str); 7] = [
        ("", ""), ("f", "66"), ("fo", "666F"), ("foo", "666F6F"),
        ("foob", "666F6F62"), ("fooba", "666F6F6261"),
        ("foobar", "666F6F626172"),
    ];
    const BASE32_RESULT: [(&str, &str); 7] = [
        ("", ""), ("f", "MY======"), ("fo", "MZXQ===="),
        ("foo", "MZXW6==="), ("foob", "MZXW6YQ="), ("fooba", "MZXW6YTB"),
        ("foobar", "MZXW6YTBOI======"),
    ];
    const BASE32_HEX_RESULT: [(&str, &str); 7] = [
        ("", ""), ("f", "CO======"), ("fo", "CPNG===="),
        ("foo", "CPNMU==="), ("foob", "CPNMUOG="), ("fooba", "CPNMUOJ1"),
        ("foobar", "CPNMUOJ1E8======"),
    ];
    const BASE64_RESULT: [(&str, &str); 7] = [
        ("", ""), ("f", "Zg=="), ("fo", "Zm8="),
        ("foo", "Zm9v"), ("foob", "Zm9vYg=="), ("fooba", "Zm9vYmE="),
        ("foobar", "Zm9vYmFy"),
    ];

    #[test]
    fn do_encode() {
        for i in BASE16_RESULT {
            assert_eq!(Base16::encode(i.0.as_bytes()), i.1);
        }
        for i in BASE32_RESULT {
            assert_eq!(Base32::encode(i.0.as_bytes()), i.1);
        }
        for i in BASE32_HEX_RESULT {
            assert_eq!(Base32Hex::encode(i.0.as_bytes()), i.1);
        }
        for i in BASE64_RESULT {
            assert_eq!(Base64::encode(i.0.as_bytes()), i.1);
        }
    }

    #[test]
    fn do_decode() {
        for i in BASE16_RESULT {
            assert_eq!(
                std::str::from_utf8(&Base16::decode(i.1)).unwrap(),
                i.0);
        }
        for i in BASE32_RESULT {
            assert_eq!(
                std::str::from_utf8(&Base32::decode(i.1)).unwrap(),
                i.0);
        }
        for i in BASE32_HEX_RESULT {
            assert_eq!(
                std::str::from_utf8(&Base32Hex::decode(i.1)).unwrap(),
                i.0);
        }
        for i in BASE64_RESULT {
            assert_eq!(
                std::str::from_utf8(&Base64::decode(i.1)).unwrap(),
                i.0);
        }
    }
}
