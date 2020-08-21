use data_encoding::{Encoding, Specification};
use lazy_static::lazy_static;

lazy_static! {
    // Base2 (alphabet: 01)
    pub static ref BASE2: Encoding = {
        let mut spec = Specification::new();
        spec.symbols.push_str("01");
        spec.encoding().unwrap()
    };

    // Base8 (alphabet: 01234567)
    pub static ref BASE8: Encoding = {
        let mut spec = Specification::new();
        spec.symbols.push_str("01234567");
        spec.encoding().unwrap()
    };

    // Base32, rfc4648 no padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
    pub static ref BASE32_NOPAD_LOWER: Encoding = {
        let mut spec = Specification::new();
        spec.symbols.push_str("abcdefghijklmnopqrstuvwxyz234567");
        spec.encoding().unwrap()
    };

    // Base32, rfc4648 with padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
    pub static ref BASE32_PAD_LOWER: Encoding = {
        let mut spec = Specification::new();
        spec.symbols.push_str("abcdefghijklmnopqrstuvwxyz234567");
        spec.padding = Some('=');
        spec.encoding().unwrap()
    };

    // Base32hex, rfc4648 no padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
    pub static ref BASE32HEX_NOPAD_LOWER: Encoding = {
        let mut spec = Specification::new();
        spec.symbols.push_str("0123456789abcdefghijklmnopqrstuv");
        spec.encoding().unwrap()
    };

    // Base32hex, rfc4648 with padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
    pub static ref BASE32HEX_PAD_LOWER: Encoding = {
        let mut spec = Specification::new();
        spec.symbols.push_str("0123456789abcdefghijklmnopqrstuv");
        spec.padding = Some('=');
        spec.encoding().unwrap()
    };

    // z-base-32 (used by Tahoe-LAFS) (alphabet: ybndrfg8ejkmcpqxot1uwisza345h769).
    pub static ref BASE32Z: Encoding = {
        let mut spec = Specification::new();
        spec.symbols.push_str("ybndrfg8ejkmcpqxot1uwisza345h769");
        spec.encoding().unwrap()
    };
}

/// Base10 (alphabet: 0123456789)
pub const BASE10: &str = "0123456789";

// Base16 lower hexadecimal (alphabet: 0123456789abcdef)
pub const BASE16_LOWER: Encoding = data_encoding::HEXLOWER;

// Base16 upper hexadecimal (alphabet: 0123456789ABCDEF).
pub const BASE16_UPPER: Encoding = data_encoding::HEXUPPER;

// Base32, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
pub const BASE32_NOPAD_UPPER: Encoding = data_encoding::BASE32_NOPAD;

// Base32, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
pub const BASE32_PAD_UPPER: Encoding = data_encoding::BASE32;

// Base32hex, rfc4648 no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
pub const BASE32HEX_NOPAD_UPPER: Encoding = data_encoding::BASE32HEX_NOPAD;

/// Base32hex, rfc4648 with padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
pub const BASE32HEX_PAD_UPPER: Encoding = data_encoding::BASE32HEX;

// Base58 Flickr's alphabet for creating short urls from photo ids.
pub const BASE58_FLICKR: &str = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

// Base58 Bitcoin's alphabet as defined in their Base58Check encoding.
pub const BASE58_BITCOIN: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// Base64, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
pub const BASE64_NOPAD: Encoding = data_encoding::BASE64_NOPAD;

// Base64, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
pub const BASE64_PAD: Encoding = data_encoding::BASE64;

// Base64 url, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
pub const BASE64URL_NOPAD: Encoding = data_encoding::BASE64URL_NOPAD;

// Base64 url, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
pub const BASE64URL_PAD: Encoding = data_encoding::BASE64URL;
