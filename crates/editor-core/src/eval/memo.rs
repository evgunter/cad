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
///
/// Deliberately a SEPARATE vocabulary from the cross-file
/// [`crate::ContentPin`] (ASM-1 D-2): the pin is collision-resistant
/// SHA-256 over a document's canonical authored bytes and IS version
/// identity; this key hashes one node's evaluated inputs (resolved
/// bits, ambient ε) with FNV and never leaves the process. Do not
/// unify them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContentKey(pub u128);

/// A node's NAMING key (issue #95, disposition 2 — M4 PR 5):
/// `naming_key(N) = H(content_key(N), [(input_id_i,
/// naming_key(input_i)), …])` — computed like a content key but
/// INCLUDING input node ids, composing recursively through the DAG.
/// Names embed minting node ids (N1) while content keys exclude them
/// (D8), so the naming half of a memoized value is a pure function of
/// THIS key, not of the content key; a memo hit additionally requires
/// naming-key equality, else the op re-runs (geometry bit-identical
/// by D9, names honestly re-derived). The recursion is what catches
/// the grandparent case: re-pointing an input two hops up changes
/// every downstream naming key even where contents agree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NamingKey(pub u128);

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
/// the scalars evaluation instantiates at (f64, Interval, the
/// K-telemetry `Probe`, and `Dual` over any of them); a new scalar
/// joins by stating its exact representation here.
///
/// # `Dual` feeds BOTH channels — the memo's seed-soundness law
///
/// A dual's exact representation is its two channels, and both feed
/// (`docs/DUAL-DESIGN.md` DL2): the value channel's representation,
/// then the tangent channel's, each through the base scalar's own
/// `feed`, separated by position exactly as every other multi-field
/// feed. No explicit seed identifier enters the key because none is
/// needed — the memo principle is *same key ⇒ same input bits ⇒ (D9)
/// same output* (module docs), and feeding both channels extends it
/// verbatim: under two different seeded parameters, a node downstream
/// of a seed differs in tangent bits (distinct keys, no cross-pass
/// contamination), while a node downstream of neither carries
/// identical value+tangent bits in both passes (same key, and the
/// reuse is sound — bit-equal inputs, deterministic dual ops). A
/// value-channel-only feed would collide two seeds' passes on every
/// node the moment a prior is threaded.
///
/// The impl is a compiler fact a doctest keeps honest, beside the
/// base-scalar companion, so the pair measures a real bound:
///
/// ```
/// fn feeds_content_bits<T: editor_core::eval::ContentBits>(_t: T) {}
/// feeds_content_bits(geom_core::Dual64::constant(1.0));
/// feeds_content_bits(1.0_f64);
/// ```
pub trait ContentBits: Real {
    /// Feed this value's exact representation to the hasher.
    fn feed(&self, h: &mut KeyHasher);
}

impl ContentBits for f64 {
    fn feed(&self, h: &mut KeyHasher) {
        h.write_f64_bits(*self);
    }
}

/// Both channels, value first (trait docs): a dual's exact
/// representation IS the pair, and the tangent bits are what keep two
/// seeds' passes out of each other's memo entries. The `Real` bound on
/// the where-clause is the trait's own (`ContentBits: Real`), stated
/// on `Dual<T>` because the base scalar alone does not imply it.
impl<T: ContentBits> ContentBits for geom_core::Dual<T>
where
    geom_core::Dual<T>: Real,
{
    fn feed(&self, h: &mut KeyHasher) {
        self.value.feed(h);
        self.deriv.feed(h);
    }
}

/// The K-telemetry recording scalar (M4 PR 8b): `Probe` is a
/// transparent `f64` whose every operation delegates exactly, so its
/// exact representation IS the wrapped f64's bits. This impl is what
/// lets the K-telemetry probe run a whole document evaluation at
/// `T = Probe` (the M2 report's collection mechanics over the Band 4
/// corpus); it feeds bits identical to the f64 lane by construction.
#[cfg(feature = "probe")]
impl ContentBits for geom_core::Probe {
    fn feed(&self, h: &mut KeyHasher) {
        h.write_f64_bits(self.0);
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
