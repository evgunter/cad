//! Document identity and content pins (ASM-1, ASSEMBLY-DESIGN A4).
//!
//! Identity ≠ pin: a [`DocumentId`] answers "which part" and survives
//! every edit; a [`ContentPin`] answers "which version of it" and is
//! the SHA-256 of the document's canonical semantic bytes
//! ([`crate::persist::canonical_bytes`]). [`DocRef`] pairs the two —
//! the value cross-document references carry.

use core::fmt;

use serde::de::Error as _;
use sha2::{Digest, Sha256};

/// A document's stable identity — AUTHORED data supplied at
/// construction, never minted from ambient randomness inside this
/// crate (the kernel is deterministic by construction; D9's
/// reproducible save bytes depend on it). Deterministic callers use
/// [`DocumentId::derive`]; interactive authoring mints random ids in
/// the document layer (`pncad`), outside this crate.
///
/// Identity survives every edit; nothing about content constrains it.
/// Workspace-level uniqueness is the store's invariant, enforced at
/// scan time, not by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DocumentId(pub u128);

impl DocumentId {
    /// Deterministic id from a human label: the first 16 bytes of
    /// SHA-256 of the label's UTF-8 bytes, big-endian. Same label,
    /// same id, on every platform — what corpus, demos, and tests
    /// use so regeneration stays byte-identical (D9).
    pub fn derive(label: &str) -> Self {
        let digest = Sha256::digest(label.as_bytes());
        let mut first = [0u8; 16];
        first.copy_from_slice(&digest[..16]);
        Self(u128::from_be_bytes(first))
    }

    /// The canonical text form: exactly 32 lowercase hex digits.
    pub fn hex(&self) -> String {
        format!("{:032x}", self.0)
    }

    /// Parses the canonical text form — exactly 32 lowercase hex
    /// digits, anything else refused (`None`). The strict spelling is
    /// what the save header and serde forms emit, so a non-canonical
    /// spelling is a tampered or foreign file, not data.
    pub fn parse_hex(text: &str) -> Option<Self> {
        if text.len() != 32 || !text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return None;
        }
        u128::from_str_radix(text, 16).ok().map(Self)
    }
}

impl fmt::Display for DocumentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

impl serde::Serialize for DocumentId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.hex())
    }
}

impl<'de> serde::Deserialize<'de> for DocumentId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        Self::parse_hex(&text).ok_or_else(|| {
            D::Error::custom(format!(
                "document id must be exactly 32 lowercase hex digits, got {text:?}"
            ))
        })
    }
}

/// A document version's content pin: the 32-byte SHA-256 of the
/// canonical semantic bytes ([`crate::persist::canonical_bytes`]).
/// The pin IS version identity (ASSEMBLY-DESIGN A4), so collision
/// resistance is required here.
///
/// Deliberately a SEPARATE vocabulary from the in-process memo's FNV
/// [`crate::ContentKey`]: the memo key hashes evaluated inputs
/// (resolved bits, ambient ε) with a non-collision-resistant hasher
/// and never leaves the process; the pin hashes authored canonical
/// bytes and crosses files. Do not unify them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContentPin(pub [u8; 32]);

impl ContentPin {
    /// The pin of the given canonical bytes.
    pub fn of_bytes(bytes: &[u8]) -> Self {
        Self(Sha256::digest(bytes).into())
    }

    /// The canonical text form: exactly 64 lowercase hex digits.
    pub fn hex(&self) -> String {
        let mut out = String::with_capacity(64);
        for b in self.0 {
            use fmt::Write as _;
            // Infallible on String; surfaced not swallowed.
            let _ = write!(out, "{b:02x}");
        }
        out
    }

    /// Parses the canonical text form — exactly 64 lowercase hex
    /// digits, anything else refused (`None`).
    pub fn parse_hex(text: &str) -> Option<Self> {
        if text.len() != 64 || !text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return None;
        }
        let mut out = [0u8; 32];
        for (i, chunk) in text.as_bytes().chunks_exact(2).enumerate() {
            let s = core::str::from_utf8(chunk).ok()?;
            out[i] = u8::from_str_radix(s, 16).ok()?;
        }
        Some(Self(out))
    }
}

impl fmt::Display for ContentPin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.hex())
    }
}

impl serde::Serialize for ContentPin {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.hex())
    }
}

impl<'de> serde::Deserialize<'de> for ContentPin {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        Self::parse_hex(&text).ok_or_else(|| {
            D::Error::custom(format!(
                "content pin must be exactly 64 lowercase hex digits, got {text:?}"
            ))
        })
    }
}

/// A cross-document reference: (which part, which version of it) —
/// the value future `InstantiatePart` nodes carry (ASSEMBLY-DESIGN
/// A4's Cargo.lock semantics: edits to the referenced document never
/// retarget this; moving the pin is a recorded edit).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct DocRef {
    /// The referenced document's stable identity.
    pub id: DocumentId,
    /// The pinned version: the content pin the reference expects the
    /// resolved document to hash to.
    pub pin: ContentPin,
}

/// How many pin hex digits [`DocRef`]'s `Display` shows — enough to
/// be humanly distinguishing, deliberately not the whole pin (the
/// full 64 digits are data, not prose; use the fields for data).
const PIN_PREFIX_HEX: usize = 12;

impl fmt::Display for DocRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pin = self.pin.hex();
        // The hex string is 64 ASCII digits, so the slice is exact.
        write!(f, "{}@{}", self.id, &pin[..PIN_PREFIX_HEX])
    }
}
