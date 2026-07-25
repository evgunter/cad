//! Hex-string encoding for opaque byte payloads (spec D3: witness
//! bytes persist bit-exactly as text; hex chosen over base64 —
//! greppable, no padding cases, canonical lowercase). A `#[serde(with
//! = ...)]` module: non-JSON-specific, byte-exact both ways, and
//! strict on load (odd length or a non-hex digit refuses typed).

use serde::{Deserialize, Deserializer, Serializer, de::Error as _};

/// Serializes `bytes` as a lowercase hex string.
///
/// # Errors
///
/// Only the underlying serializer's own errors.
pub fn serialize<S: Serializer>(bytes: &[u8], ser: S) -> Result<S::Ok, S::Error> {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write as _;
        // Infallible for String; the map keeps the signature honest.
        write!(&mut s, "{b:02x}").map_err(serde::ser::Error::custom)?;
    }
    ser.serialize_str(&s)
}

/// Deserializes a lowercase-or-uppercase hex string into bytes.
///
/// # Errors
///
/// A typed refusal on odd length or any non-hex character (no silent
/// best-effort loads, spec D6.3).
pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Vec<u8>, D::Error> {
    let s = String::deserialize(de)?;
    if !s.len().is_multiple_of(2) {
        return Err(D::Error::custom(format!(
            "hex byte string has odd length {}",
            s.len()
        )));
    }
    let nib = |c: u8| -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(c - b'a' + 10),
            b'A'..=b'F' => Some(c - b'A' + 10),
            _ => None,
        }
    };
    s.as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let (hi, lo) = (nib(pair[0]), nib(pair[1]));
            match (hi, lo) {
                (Some(h), Some(l)) => Ok((h << 4) | l),
                _ => Err(D::Error::custom(format!(
                    "non-hex byte pair {:?} in byte string",
                    String::from_utf8_lossy(pair)
                ))),
            }
        })
        .collect()
}
