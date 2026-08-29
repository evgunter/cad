//! Content keys — the memoization currency (spec D4, the banked
//! principle): `content_key(node) = hash(op kind, structural params,
//! evaluated expression values AS BITS, upstream content keys)`, plus
//! the ambient tolerance (ε and k enter every decision the ops make,
//! so they are inputs by D9's replay identity). Bit-exact floats in,
//! so same key ⇒ same inputs ⇒ (D9 determinism) same output: the key
//! IS the correctness proof for reuse.
//!
//! # Hasher choice (spec D4)
//!
//! FNV-1a, 64-bit, run TWICE with the two documented offset bases
//! below to form a 128-bit key — stable across runs, platforms, and
//! Rust versions (the std hasher is randomly seeded per-process and
//! explicitly unstable — D9 forbids it here). FNV is not
//! collision-resistant against adversaries; the threat model is
//! accidental collision between a document's own node inputs, where
//! 128 bits of independent-basis FNV is far beyond the DAG sizes this
//! layer sees. Revisit alongside PR 6 persistence if keys ever go
//! cross-session.

use geom_core::Real;

/// A node's input-content hash (spec D2/D4). Equal keys certify equal
/// inputs (up to 128-bit hash collision); the evaluator reuses the
/// prior value without re-running the op.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContentKey(pub u128);

/// The two FNV-1a states (standard 64-bit offset basis, and the same
/// basis XOR a documented constant so the halves are independent
/// walks; both use the standard FNV prime).
#[derive(Debug, Clone)]
pub struct KeyHasher {
    lo: u64,
    hi: u64,
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
/// Documented second-basis tweak (arbitrary odd constant, fixed
/// forever — changing it invalidates every stored key).
const HI_TWEAK: u64 = 0x9e37_79b9_7f4a_7c15;

impl KeyHasher {
    /// A fresh hasher at the documented bases.
    pub fn new() -> Self {
        Self {
            lo: FNV_OFFSET,
            hi: FNV_OFFSET ^ HI_TWEAK,
        }
    }

    /// Feeds one u64 (little-endian byte walk, both lanes).
    pub fn write_u64(&mut self, x: u64) {
        for b in x.to_le_bytes() {
            self.lo = (self.lo ^ u64::from(b)).wrapping_mul(FNV_PRIME);
            self.hi = (self.hi ^ u64::from(b)).wrapping_mul(FNV_PRIME);
        }
    }

    /// Feeds a tag byte (domain separation between record kinds).
    pub fn write_tag(&mut self, tag: u8) {
        self.write_u64(u64::from(tag));
    }

    /// Feeds an i64 (two's-complement bits).
    pub fn write_i64(&mut self, x: i64) {
        self.write_u64(x as u64);
    }

    /// Feeds an f64's exact bits.
    pub fn write_f64_bits(&mut self, x: f64) {
        self.write_u64(x.to_bits());
    }

    /// Feeds a prior content key (upstream link).
    pub fn write_key(&mut self, key: ContentKey) {
        self.write_u64(key.0 as u64);
        self.write_u64((key.0 >> 64) as u64);
    }

    /// Feeds UTF-8 bytes, length-prefixed.
    pub fn write_str(&mut self, s: &str) {
        self.write_u64(s.len() as u64);
        for b in s.as_bytes() {
            self.write_u64(u64::from(*b));
        }
    }

    /// Feeds raw bytes, length-prefixed (opaque payloads — the
    /// witness datum's exact representation, M4 PR 4).
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.write_u64(bytes.len() as u64);
        for b in bytes {
            self.write_u64(u64::from(*b));
        }
    }

    /// The finished key.
    pub fn finish(&self) -> ContentKey {
        ContentKey((u128::from(self.hi) << 64) | u128::from(self.lo))
    }
}

impl Default for KeyHasher {
    fn default() -> Self {
        Self::new()
    }
}

/// The bit representation of an evaluated scalar, fed to content keys
/// (spec D4: "evaluated expression values AS BITS"). Implemented for
/// the two scalars evaluation instantiates at; a new scalar joins by
/// stating its exact representation here.
pub trait ContentBits: Real {
    /// Feed this value's exact representation to the hasher.
    fn feed(&self, h: &mut KeyHasher);
}

impl ContentBits for f64 {
    fn feed(&self, h: &mut KeyHasher) {
        h.write_f64_bits(*self);
    }
}

/// RETIREMENT-SCHEDULED bit-identity consumer (DESIGN.md M4; the
/// ci.yml tripwire allowlists this file): unlike the coincidence
/// checkers the retirement targets, this site HASHES the exact
/// representation into a content key (spec D4 mandates "evaluated
/// expression values AS BITS") and never compares values by bits.
/// When provenance-based naming retires the channel, this feed moves
/// to whatever exact-representation door replaces `repr_bits`.
#[cfg(feature = "interval")]
impl ContentBits for geom_core::Interval {
    fn feed(&self, h: &mut KeyHasher) {
        let (lo, hi, dec) = self.repr_bits();
        h.write_u64(lo);
        h.write_u64(hi);
        h.write_u64(u64::from(dec));
    }
}
