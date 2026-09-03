//! **The symbolic identity tier** (ERROR-DESIGN E12): a lane scalar that
//! carries, beside its numeric value, a handle into an expression DAG
//! over the document's continuous-parameter symbols — so a margin whose
//! expression is IDENTICALLY ZERO in the parameters decides
//! [`Sign::Zero`] before any enclosure is consulted, for every parameter
//! value, at any box width.
//!
//! # The defect this closes
//!
//! The kernel's certification population is full of checked IDENTITIES:
//! an edge's endpoint lies ON its carrier, a side plane is cosurface
//! with its neighbour. Their margin is exactly zero in real arithmetic
//! for every parameter value, and their INTERVAL enclosure over a box of
//! width `w` is `[0, c·w]` with `c ≈ 2–4`, because the two sides of the
//! identity reach the funnel as two separately evaluated intervals and
//! interval arithmetic cannot see that the occurrences of the parameter
//! are one number. A leaf goes definite only once its own width is a
//! fraction of ε, so a macroscopic tolerance box refuses all of its mass.
//! No re-association at the decide site can recover it — the dependence
//! was lost upstream — which is why the tier tracks it from the
//! parameter down.
//!
//! # What a symbolic `Zero` claims, and why it is a THEOREM
//!
//! [`Decide::sign_within`] on a [`Sym<T>`] answers `Ok(Sign::Zero)`
//! without consulting the enclosure exactly when two things hold:
//!
//! 1. the value channel **certifies** ([`CertifiedEnclosure`]) — the
//!    computation was defined on the whole input box; and
//! 2. the node's POLYNOMIAL NORMAL FORM over the parameter symbols is
//!    the zero polynomial.
//!
//! The normal form is computed with EXACT rational coefficients and
//! **nothing in it ever reads a value**. Every node denotes a real-valued
//! function of its indeterminates (the parameter symbols, π, and the
//! opaque atoms below), and every scalar operation's value channel
//! encloses that same real. So a form that is zero as a polynomial is
//! zero under every real assignment of its indeterminates — in
//! particular under the actual one, at every parameter point of the box.
//! That is the whole soundness argument, and it is pinned by the
//! `sym_theorem` rows of `geom-core`'s suite.
//!
//! Clause 1 is not decoration. Without it a domain violation
//! (`sqrt(-1) - sqrt(-1)`) would decide `Zero` on an expression that has
//! no real value at all; the gate is the same door
//! [`crate::Interval::sign_within`] refuses at, for the same reason.
//!
//! # The DAG, and what is opaque in it
//!
//! Nodes are `Param(symbol)`, `Lit(f64 bits)`, `Pi`, `Add`/`Sub`/`Mul`/
//! `Neg`/`Powi`, `Div` as `Mul(a, Inv(b))`, and OPAQUE atoms for every
//! other [`Real`] operation (`sqrt`, `abs`, the trigonometric family,
//! `floor`, `min`, `max`, `copysign`, and the span-hull seam), keyed by
//! the normal form of their arguments. Two atoms whose arguments have
//! the same normal form are the SAME indeterminate — which is what lets
//! `sqrt(x² + y²) − sqrt(y² + x²)` cancel — and an atom applied to a
//! zero form folds to the value the function takes at zero where that
//! value is rational (`sqrt 0 = 0`, `cos 0 = 1`, `acos 0 = π/2`).
//!
//! The periodic reducers are deliberately NOT overridden: [`Real`]'s
//! defaulted bodies are a fixed composition of `÷`, [`Real::floor`], `·`
//! and `−`, so they decompose into the DAG on their own and only the
//! `floor` stays opaque. Overriding them would be strictly less
//! cancellation for no gain.
//!
//! **The documented limits.** There is no factoring, no
//! `Inv(b)·b = 1`, and no trigonometric identity: `sin² + cos² − 1` does
//! not decide symbolically, and neither does `(x/y)·y − x`. Each is an
//! opaque atom by construction, which is a limit of the tier and not a
//! bug in it — over-refusal is the safe direction, and every such margin
//! falls to the numeric channel exactly as before.
//!
//! # Node ids are CONTENT HASHES (D9)
//!
//! A node's id is a 128-bit structural hash of `(op, children ids,
//! payload bits)` — never a sequence number. An id is therefore the same
//! under every rayon schedule and every insertion order, structural
//! sharing is free, and two builds of the same expression memoize the
//! same normal form. The hash-consing table is per-leaf-replay
//! ([`with_session`]), holds nothing across leaves, and is dropped with
//! the leaf.
//!
//! Distinct expressions colliding on a 128-bit content hash would be a
//! soundness break; this is the standard hash-consing assumption and it
//! is stated rather than hidden.
//!
//! # Freezing: the budget, and why it is sound
//!
//! A form whose term count or total degree exceeds the session's
//! [`SymBudget`], or whose coefficient arithmetic overflows the in-tree
//! rational, is FROZEN: the node becomes an indeterminate of its own,
//! keyed by its content hash. Cancellation THROUGH a frozen node is
//! lost; soundness is not, because an unknown function of the parameters
//! is exactly what an indeterminate denotes. Two structurally identical
//! frozen nodes still share an id and therefore still cancel. Every
//! freeze is counted ([`SymCounts::frozen`]).
//!
//! **The coefficients are an in-tree dyadic-scaled `i128` rational**
//! rather than `num-rational`/`num-bigint`, which are already in the
//! lock. Two reasons, both about this crate: `geom-core`'s runtime
//! dependency set is `libm` and nothing else, and an arbitrary-precision
//! coefficient has unbounded cost in a test run thousands of times per
//! leaf. The freezing budget exists either way; an overflow freeze is the
//! same sound outcome as a term-count freeze, and it is measured rather
//! than assumed.
//!
//! # No session, no tier
//!
//! Ids are computable without the table, so a [`Sym<T>`] built outside
//! [`with_session`] still carries a deterministic id — the lookup simply
//! misses, the form freezes, and every decision falls to the numeric
//! channel. The tier is never partially on.

use core::cell::RefCell;
use core::ops::{Add, Div, Mul, Neg, Sub};
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use crate::predicate::{Band, Decide, Indeterminate, Sign};
use crate::real::{Bounds, CertifiedEnclosure, Real};
use crate::spline::{KnotVector, SpanLocate, SpanSet};

// ---------------------------------------------------------------- ids

/// A DAG node's identity: the 128-bit structural content hash of
/// `(op, children ids, payload bits)` (module docs).
///
/// Never a sequence number, so it is stable across rayon schedules,
/// insertion orders and repeats — D9 for free.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymId(u128);

impl SymId {
    /// The id of a leaf minted with no session installed. Reserved: the
    /// mixer never produces it, so it can never collide with a node.
    const UNRECORDED: Self = Self(0);

    /// The id's bits — for a determinism check that has to compare two
    /// DAGs without owning their nodes.
    #[must_use]
    pub fn bits(self) -> u128 {
        self.0
    }
}

/// A 128-bit FNV-1a over the little-endian bytes of the words fed to it
/// — a fixed, platform-independent mixer, which is what D9 asks of an
/// identity that has to agree across builds.
struct Hash128(u128);

impl Hash128 {
    const OFFSET: u128 = 0x6c62_272e_07bb_0142_62b8_2175_6295_c58d;
    const PRIME: u128 = 0x0000_0000_0100_0000_0000_0000_0000_013b;

    fn new() -> Self {
        Self(Self::OFFSET)
    }

    fn word(mut self, w: u64) -> Self {
        for byte in w.to_le_bytes() {
            self.0 ^= u128::from(byte);
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
        self
    }

    fn wide(self, w: u128) -> Self {
        self.word(w as u64).word((w >> 64) as u64)
    }

    /// The digest, with zero folded away: `SymId(0)` is reserved for the
    /// unrecorded leaf, so no real node may claim it.
    fn finish(self) -> u128 {
        if self.0 == 0 { Self::OFFSET } else { self.0 }
    }
}

/// The symbol a document parameter enters the DAG as: a hash of its
/// name, so two evaluations of the same document agree on it without
/// carrying a string into a `Copy` scalar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParamSymbol(u64);

impl ParamSymbol {
    /// The symbol for a parameter name.
    #[must_use]
    pub fn of(name: &str) -> Self {
        let mut h = Hash128::new().word(0x5359_4d5f_5041_5241);
        for b in name.as_bytes() {
            h = h.word(u64::from(*b));
        }
        Self(h.finish() as u64)
    }
}

// -------------------------------------------------------------- nodes

/// What one DAG node computes. Everything outside the ring operations
/// is an OPAQUE atom (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SymOp {
    Param,
    Lit,
    Pi,
    Add,
    Sub,
    Mul,
    Neg,
    Powi,
    /// `1/x` — an atom, so `Inv(b)·b` does not fold to one.
    Inv,
    Sqrt,
    Abs,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Floor,
    Atan2,
    Min,
    Max,
    Copysign,
    /// The multi-span enclosure hull ([`SpanLocate::enclosure_hull`]).
    /// Keyed by CHILD IDS rather than by their normal forms, because a
    /// hull is a function of the operands' ENCLOSURES and not of the
    /// reals they stand for — two expressions with equal normal forms
    /// can carry different enclosures, so keying it by form would claim
    /// an equality that does not hold.
    Hull,
}

impl SymOp {
    /// The op's tag in the content hash — an explicit number per
    /// variant, so reordering the enum cannot silently re-key a DAG.
    fn tag(self) -> u64 {
        match self {
            Self::Param => 1,
            Self::Lit => 2,
            Self::Pi => 3,
            Self::Add => 4,
            Self::Sub => 5,
            Self::Mul => 6,
            Self::Neg => 7,
            Self::Powi => 8,
            Self::Inv => 9,
            Self::Sqrt => 10,
            Self::Abs => 11,
            Self::Sin => 12,
            Self::Cos => 13,
            Self::Tan => 14,
            Self::Asin => 15,
            Self::Acos => 16,
            Self::Atan => 17,
            Self::Floor => 18,
            Self::Atan2 => 19,
            Self::Min => 20,
            Self::Max => 21,
            Self::Copysign => 22,
            Self::Hull => 23,
        }
    }

    /// How many of the node's two child slots this op reads.
    fn arity(self) -> usize {
        match self {
            Self::Param | Self::Lit | Self::Pi => 0,
            Self::Neg
            | Self::Powi
            | Self::Inv
            | Self::Sqrt
            | Self::Abs
            | Self::Sin
            | Self::Cos
            | Self::Tan
            | Self::Asin
            | Self::Acos
            | Self::Atan
            | Self::Floor => 1,
            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Atan2
            | Self::Min
            | Self::Max
            | Self::Copysign
            | Self::Hull => 2,
        }
    }
}

/// One DAG node: its op, its payload bits (`Lit`'s float bits,
/// `Param`'s symbol, `Powi`'s exponent) and up to two children.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SymNode {
    op: SymOp,
    payload: u64,
    kids: [SymId; 2],
}

impl SymNode {
    fn id(&self) -> SymId {
        SymId(
            Hash128::new()
                .word(self.op.tag())
                .word(self.payload)
                .wide(self.kids[0].0)
                .wide(self.kids[1].0)
                .finish(),
        )
    }
}

// ---------------------------------------------------------- rationals

/// An exact rational `num / den · 2^exp2`, with `num`/`den` odd and
/// coprime and `den > 0` — the normal form's coefficient.
///
/// The power of two is factored out rather than left in the pair
/// because every `f64` literal IS `m · 2^e`: keeping `e` in its own
/// field leaves the odd part alone, so the round constants a recipe is
/// full of (`1`, `½`, `2`, `¼`) never grow the `i128` at all and the
/// budget bites on genuine term growth instead of on scaling.
///
/// Every operation is CHECKED and answers `None` on overflow, which the
/// caller turns into a freeze (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rat {
    num: i128,
    den: i128,
    exp2: i32,
}

/// The greatest common divisor of two non-negative `i128`s.
fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// Strips factors of two out of `v`, answering the odd part and how many
/// were removed. `v == 0` keeps zero and removes none.
fn strip_twos(v: i128) -> (i128, u32) {
    if v == 0 {
        return (0, 0);
    }
    let k = v.trailing_zeros();
    (v >> k, k)
}

impl Rat {
    const ZERO: Self = Self {
        num: 0,
        den: 1,
        exp2: 0,
    };

    /// Reduces `num / den · 2^exp2` to the canonical shape.
    fn new(num: i128, den: i128, exp2: i32) -> Option<Self> {
        if den == 0 {
            return None;
        }
        if num == 0 {
            return Some(Self::ZERO);
        }
        let (num, den) = if den < 0 {
            (num.checked_neg()?, den.checked_neg()?)
        } else {
            (num, den)
        };
        let g = gcd(num.unsigned_abs() as i128, den);
        let (num, den) = (num / g, den / g);
        let (num, nz) = strip_twos(num);
        let (den, dz) = strip_twos(den);
        let exp2 = exp2
            .checked_add(i32::try_from(nz).ok()?)?
            .checked_sub(i32::try_from(dz).ok()?)?;
        Some(Self { num, den, exp2 })
    }

    /// The exact value of a finite `f64`; `None` for a non-finite one
    /// (which cannot be a coefficient of a real polynomial).
    fn of_f64(x: f64) -> Option<Self> {
        if !x.is_finite() {
            return None;
        }
        if x == 0.0 {
            return Some(Self::ZERO);
        }
        let bits = x.to_bits();
        let sign = if bits >> 63 == 1 { -1i128 } else { 1i128 };
        let raw_exp = ((bits >> 52) & 0x7ff) as i32;
        let frac = (bits & 0x000f_ffff_ffff_ffff) as i128;
        // Subnormals carry no implicit leading bit and sit one exponent
        // step above what the biased field alone would say.
        let (mantissa, exp) = if raw_exp == 0 {
            (frac, -1074)
        } else {
            (frac | (1i128 << 52), raw_exp - 1075)
        };
        Self::new(sign * mantissa, 1, exp)
    }

    fn is_zero(self) -> bool {
        self.num == 0
    }

    fn add(self, other: Self) -> Option<Self> {
        if self.is_zero() {
            return Some(other);
        }
        if other.is_zero() {
            return Some(self);
        }
        // Align on the smaller exponent, shifting the other numerator up.
        let (lo, hi) = (self.exp2.min(other.exp2), self.exp2.max(other.exp2));
        let shift = u32::try_from(hi.checked_sub(lo)?).ok()?;
        let scale = 1i128.checked_shl(shift)?;
        let (a, b) = if self.exp2 <= other.exp2 {
            (self, Self { num: other.num.checked_mul(scale)?, ..other })
        } else {
            (Self { num: self.num.checked_mul(scale)?, ..self }, other)
        };
        let num = a
            .num
            .checked_mul(b.den)?
            .checked_add(b.num.checked_mul(a.den)?)?;
        Self::new(num, a.den.checked_mul(b.den)?, lo)
    }

    fn neg(self) -> Option<Self> {
        Some(Self {
            num: self.num.checked_neg()?,
            ..self
        })
    }

    fn mul(self, other: Self) -> Option<Self> {
        if self.is_zero() || other.is_zero() {
            return Some(Self::ZERO);
        }
        Self::new(
            self.num.checked_mul(other.num)?,
            self.den.checked_mul(other.den)?,
            self.exp2.checked_add(other.exp2)?,
        )
    }

    /// Feeds the coefficient to a content hash (the atom-keying digest).
    fn feed(self, h: Hash128) -> Hash128 {
        h.wide(self.num as u128).wide(self.den as u128).word(u64::from(self.exp2 as u32))
    }
}

// ----------------------------------------------------------- the form

/// A monomial: indeterminate ids with their exponents, sorted by id and
/// carrying no zero exponent. The empty vector is the constant monomial.
type Mono = Vec<(u128, u32)>;

/// The polynomial normal form over the parameter symbols, π and the
/// opaque atoms — with exact rational coefficients, and no zero
/// coefficient stored, so **the form is zero iff it has no terms**.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct Poly {
    terms: BTreeMap<Mono, Rat>,
}

impl Poly {
    fn zero() -> Self {
        Self::default()
    }

    fn constant(c: Rat) -> Self {
        let mut terms = BTreeMap::new();
        if !c.is_zero() {
            terms.insert(Mono::new(), c);
        }
        Self { terms }
    }

    /// The form of a single indeterminate, coefficient one.
    fn indet(id: u128) -> Self {
        let mut terms = BTreeMap::new();
        terms.insert(
            vec![(id, 1)],
            Rat {
                num: 1,
                den: 1,
                exp2: 0,
            },
        );
        Self { terms }
    }

    fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    /// The largest total degree of any term (zero for the zero form).
    fn degree(&self) -> u32 {
        self.terms
            .keys()
            .map(|m| m.iter().map(|(_, e)| *e).sum::<u32>())
            .max()
            .unwrap_or(0)
    }

    fn insert(&mut self, mono: Mono, c: Rat) -> Option<()> {
        if c.is_zero() {
            return Some(());
        }
        match self.terms.remove(&mono) {
            None => {
                self.terms.insert(mono, c);
            }
            Some(existing) => {
                let sum = existing.add(c)?;
                if !sum.is_zero() {
                    self.terms.insert(mono, sum);
                }
            }
        }
        Some(())
    }

    fn add(&self, other: &Self) -> Option<Self> {
        let mut out = self.clone();
        for (m, c) in &other.terms {
            out.insert(m.clone(), *c)?;
        }
        Some(out)
    }

    fn neg(&self) -> Option<Self> {
        let mut out = Self::zero();
        for (m, c) in &self.terms {
            out.terms.insert(m.clone(), c.neg()?);
        }
        Some(out)
    }

    fn mul(&self, other: &Self) -> Option<Self> {
        let mut out = Self::zero();
        for (ma, ca) in &self.terms {
            for (mb, cb) in &other.terms {
                out.insert(mono_mul(ma, mb)?, ca.mul(*cb)?)?;
            }
        }
        Some(out)
    }

    /// The form's canonical digest — the key an opaque atom is minted
    /// under, so two atoms with equal-form arguments are one
    /// indeterminate.
    fn digest(&self) -> u128 {
        let mut h = Hash128::new().word(0x504f_4c59_4e46_524d);
        for (m, c) in &self.terms {
            h = h.word(m.len() as u64);
            for (id, e) in m {
                h = h.wide(*id).word(u64::from(*e));
            }
            h = c.feed(h);
        }
        h.finish()
    }
}

/// The product of two monomials, refusing an exponent overflow.
fn mono_mul(a: &Mono, b: &Mono) -> Option<Mono> {
    let mut out: Mono = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        match (a.get(i), b.get(j)) {
            (Some(&(ia, ea)), Some(&(ib, eb))) if ia == ib => {
                out.push((ia, ea.checked_add(eb)?));
                i += 1;
                j += 1;
            }
            (Some(&(ia, ea)), Some(&(ib, _))) if ia < ib => {
                out.push((ia, ea));
                i += 1;
            }
            (Some(_), Some(&(ib, eb))) => {
                out.push((ib, eb));
                j += 1;
            }
            (Some(&(ia, ea)), None) => {
                out.push((ia, ea));
                i += 1;
            }
            (None, Some(&(ib, eb))) => {
                out.push((ib, eb));
                j += 1;
            }
            (None, None) => break,
        }
    }
    Some(out)
}

// --------------------------------------------------------- the session

/// The freezing budget: the size a normal form may reach before the node
/// is frozen into an indeterminate of its own (module docs).
///
/// A run dial, not a constant of nature — the driver carries it, and the
/// frozen count is the evidence for whatever it is set to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymBudget {
    /// The most terms one form may hold.
    pub max_terms: usize,
    /// The largest total degree one form may reach.
    pub max_degree: u32,
}

impl SymBudget {
    /// A budget of zero terms: every form freezes, so every decision
    /// falls to the numeric channel. The differential that shows the
    /// tier's only effect is the identities it discharges.
    #[must_use]
    pub fn none() -> Self {
        Self {
            max_terms: 0,
            max_degree: 0,
        }
    }
}

/// What one session's decisions came to — the E12 receipt (`symbolic`
/// against `numeric`) and the honesty column beside it (`frozen`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SymCounts {
    /// Decisions answered `Zero` by the symbolic tier.
    pub symbolic_zero: u64,
    /// Decisions handed to the numeric channel.
    pub numeric: u64,
    /// Nodes frozen into indeterminates (a budget or an overflow).
    pub frozen: u64,
}

impl SymCounts {
    /// The two decision counts added together.
    #[must_use]
    pub fn decisions(&self) -> u64 {
        self.symbolic_zero + self.numeric
    }

    /// Adds another session's counts into this one.
    pub fn absorb(&mut self, other: Self) {
        self.symbolic_zero += other.symbolic_zero;
        self.numeric += other.numeric;
        self.frozen += other.frozen;
    }
}

/// A hasher for keys that ARE hashes: it takes the low 64 bits verbatim.
/// Deterministic and allocation-free; the map is never iterated, so no
/// ordering claim rides on it.
#[derive(Default)]
struct IdHasher(u64);

impl core::hash::Hasher for IdHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.0 = self.0.rotate_left(8) ^ u64::from(*b);
        }
    }

    fn write_u128(&mut self, i: u128) {
        self.0 = i as u64;
    }
}

type IdMap<V> = HashMap<SymId, V, core::hash::BuildHasherDefault<IdHasher>>;

/// One leaf replay's DAG: the hash-consing table, the memoized forms and
/// the counts. Dropped with the leaf; nothing is shared across leaves.
struct Session {
    budget: SymBudget,
    nodes: IdMap<SymNode>,
    forms: IdMap<Rc<Poly>>,
    counts: SymCounts,
}

thread_local! {
    /// The installed session, if any (module docs: no session, no tier).
    static SESSION: RefCell<Option<Session>> = const { RefCell::new(None) };
}

/// Runs `f` with a fresh symbolic session installed on this thread,
/// answering its result beside the session's counts.
///
/// The table is per-call and dropped at the end of it, which is what
/// "one hash-consing table per leaf replay" means: a leaf's nodes never
/// reach another leaf, and the counts are that leaf's own. Nesting is
/// refused rather than silently flattened — an inner session would count
/// a different leaf's decisions into the outer one's receipt.
pub fn with_session<R>(budget: SymBudget, f: impl FnOnce() -> R) -> (R, SymCounts) {
    let nested = SESSION.with(|s| s.borrow().is_some());
    // A nested call in a release build runs with the OUTER session,
    // which is sound — ids are content hashes and the table is keyed by
    // them — and only muddles whose receipt the decisions land in.
    debug_assert!(!nested, "symbolic sessions do not nest");
    if nested {
        return (f(), SymCounts::default());
    }
    SESSION.with(|s| {
        *s.borrow_mut() = Some(Session {
            budget,
            nodes: IdMap::default(),
            forms: IdMap::default(),
            counts: SymCounts::default(),
        });
    });
    let out = f();
    let counts = SESSION
        .with(|s| s.borrow_mut().take())
        .map_or_else(SymCounts::default, |s| s.counts);
    (out, counts)
}

/// The counts so far in the installed session (`None` outside one) — the
/// door a driver reads mid-replay when it prices a leaf.
#[must_use]
pub fn session_counts() -> Option<SymCounts> {
    SESSION.with(|s| s.borrow().as_ref().map(|s| s.counts))
}

/// Records `node` in the installed session and answers its id. Outside a
/// session the id is still computed — it is a pure function of the node
/// — and nothing is stored.
fn intern(node: SymNode) -> SymId {
    let id = node.id();
    SESSION.with(|s| {
        if let Some(sess) = s.borrow_mut().as_mut() {
            sess.nodes.entry(id).or_insert(node);
        }
    });
    id
}

// ---------------------------------------------------- the normal form

/// The indeterminate π enters the form as: a fixed key, so `τ − 2π`
/// cancels while nothing reads a value of π anywhere.
const INDET_PI: u128 = 0x5049_5f49_4e44_4554_5f5f_5f5f_5f5f_5f5f;

/// The indeterminate a parameter symbol enters the form as.
fn indet_param(symbol: u64) -> u128 {
    Hash128::new().word(0x5041_5241_4d5f_494e).word(symbol).finish()
}

/// The indeterminate an OPAQUE atom enters the form as: its op tag, its
/// payload and the DIGESTS of its arguments' normal forms — so two
/// atoms whose arguments are the same real are one indeterminate.
fn indet_atom(tag: u64, payload: u64, args: &[u128]) -> u128 {
    let mut h = Hash128::new().word(0x4154_4f4d_5f49_4e44).word(tag).word(payload);
    for d in args {
        h = h.wide(*d);
    }
    h.finish()
}

/// The value an opaque UNARY atom takes at argument zero, where that
/// value is expressible in the form's own vocabulary — the fold that
/// lets `‖a − b‖` decide `Zero` when `a − b` does, which is the shape
/// most of the kernel's identity margins arrive in (`Margin::of` of a
/// distance is a `sqrt`).
fn unary_at_zero(op: SymOp) -> Option<Poly> {
    let one = Rat {
        num: 1,
        den: 1,
        exp2: 0,
    };
    match op {
        SymOp::Sqrt
        | SymOp::Abs
        | SymOp::Sin
        | SymOp::Tan
        | SymOp::Asin
        | SymOp::Atan
        | SymOp::Floor
        | SymOp::Neg => Some(Poly::zero()),
        SymOp::Cos => Some(Poly::constant(one)),
        // acos 0 = π/2 — expressible, because π is an indeterminate of
        // the form rather than a number.
        SymOp::Acos => {
            let mut p = Poly::indet(INDET_PI);
            let half = Rat::new(1, 2, 0)?;
            for c in p.terms.values_mut() {
                *c = c.mul(half)?;
            }
            Some(p)
        }
        // 1/0 is not a real; the numeric channel owns that refusal.
        _ => None,
    }
}

/// Whether a form is inside the session's freezing budget.
fn within(budget: SymBudget, p: &Poly) -> bool {
    p.terms.len() <= budget.max_terms && p.degree() <= budget.max_degree
}

/// `base^n` for `n >= 0`, budget-checked at every step so a large
/// exponent freezes rather than allocating its way to the ceiling.
fn powi_form(base: &Poly, n: u32, budget: SymBudget) -> Option<Poly> {
    let mut acc = Poly::constant(Rat {
        num: 1,
        den: 1,
        exp2: 0,
    });
    for _ in 0..n {
        acc = acc.mul(base)?;
        if !within(budget, &acc) {
            return None;
        }
    }
    Some(acc)
}

/// The form of one node, given its children's forms — `None` for
/// anything the caller must freeze (an overflow, a budget, an
/// unrepresentable literal).
fn combine(node: &SymNode, kids: [&Poly; 2], budget: SymBudget) -> Option<Poly> {
    let (a, b) = (kids[0], kids[1]);
    let atom1 = |op: SymOp| {
        if a.is_zero() {
            if let Some(p) = unary_at_zero(op) {
                return Some(p);
            }
        }
        Some(Poly::indet(indet_atom(op.tag(), node.payload, &[a.digest()])))
    };
    let atom2 = |op: SymOp| {
        Some(Poly::indet(indet_atom(
            op.tag(),
            node.payload,
            &[a.digest(), b.digest()],
        )))
    };
    match node.op {
        SymOp::Param => Some(Poly::indet(indet_param(node.payload))),
        SymOp::Lit => Rat::of_f64(f64::from_bits(node.payload)).map(Poly::constant),
        SymOp::Pi => Some(Poly::indet(INDET_PI)),
        SymOp::Add => a.add(b),
        SymOp::Sub => a.add(&b.neg()?),
        SymOp::Mul => a.mul(b),
        SymOp::Neg => a.neg(),
        SymOp::Powi => {
            let n = node.payload as u32 as i32;
            match u32::try_from(n) {
                Ok(n) => powi_form(a, n, budget),
                // A negative power is `Inv` of a positive one, and `Inv`
                // is an atom: keyed by the exponent and the base's form.
                Err(_) => atom1(SymOp::Powi),
            }
        }
        SymOp::Inv
        | SymOp::Sqrt
        | SymOp::Abs
        | SymOp::Sin
        | SymOp::Cos
        | SymOp::Tan
        | SymOp::Asin
        | SymOp::Acos
        | SymOp::Atan
        | SymOp::Floor => atom1(node.op),
        // min(0, 0) and max(0, 0) are zero; a one-sided zero says
        // nothing, so only the both-zero fold is taken.
        SymOp::Min | SymOp::Max => {
            if a.is_zero() && b.is_zero() {
                Some(Poly::zero())
            } else {
                atom2(node.op)
            }
        }
        // copysign carries `a`'s MAGNITUDE, so a zero first argument is
        // zero whatever the sign argument does (±0 is one real).
        SymOp::Copysign => {
            if a.is_zero() {
                Some(Poly::zero())
            } else {
                atom2(node.op)
            }
        }
        // atan2(0, x) is 0 or π depending on the sign of x — not a fold
        // the form can take without reading a value.
        SymOp::Atan2 => atom2(node.op),
        // Keyed by the CHILD IDS, never by their forms (the op's docs).
        SymOp::Hull => Some(Poly::indet(
            Hash128::new()
                .word(SymOp::Hull.tag())
                .wide(node.kids[0].bits())
                .wide(node.kids[1].bits())
                .finish(),
        )),
    }
}

/// The normal form of `root`, computed and memoized inside `sess`.
///
/// Iterative rather than recursive: an evaluation's DAG is as deep as
/// its expression tree, and a leaf replay's is thousands of nodes.
/// Termination is structural — a node's id is a hash of its children's
/// ids, so a cycle would need a hash preimage — and every popped id
/// leaves a form behind, so each is visited at most twice.
fn form_in(sess: &mut Session, root: SymId) -> Rc<Poly> {
    let frozen = |sess: &mut Session, id: SymId| -> Rc<Poly> {
        sess.counts.frozen += 1;
        Rc::new(Poly::indet(id.bits()))
    };
    let mut stack = vec![(root, false)];
    while let Some((id, expanded)) = stack.pop() {
        if sess.forms.contains_key(&id) {
            continue;
        }
        let Some(node) = sess.nodes.get(&id).copied() else {
            // Not in this session's table: an unrecorded leaf, or a node
            // minted before the session was installed. An unknown
            // function of the parameters is exactly an indeterminate.
            let f = frozen(sess, id);
            sess.forms.insert(id, f);
            continue;
        };
        let arity = node.op.arity();
        if !expanded {
            let pending: Vec<SymId> = node.kids[..arity]
                .iter()
                .copied()
                .filter(|k| !sess.forms.contains_key(k))
                .collect();
            if !pending.is_empty() {
                stack.push((id, true));
                stack.extend(pending.into_iter().map(|k| (k, false)));
                continue;
            }
        }
        let empty = Poly::zero();
        let ka = node.kids[0];
        let kb = node.kids[1];
        let fa = if arity >= 1 {
            sess.forms.get(&ka).cloned()
        } else {
            None
        };
        let fb = if arity >= 2 {
            sess.forms.get(&kb).cloned()
        } else {
            None
        };
        let kids = [
            fa.as_deref().unwrap_or(&empty),
            fb.as_deref().unwrap_or(&empty),
        ];
        let budget = sess.budget;
        let made = combine(&node, kids, budget).filter(|p| within(budget, p));
        drop((fa, fb));
        let f = match made {
            Some(p) => Rc::new(p),
            None => frozen(sess, id),
        };
        sess.forms.insert(id, f);
    }
    sess.forms
        .get(&root)
        .cloned()
        .unwrap_or_else(|| Rc::new(Poly::indet(root.bits())))
}

/// **The identity test**: is this node's expression identically zero in
/// the parameters?
///
/// `false` outside a session, and `false` at a zero-term budget — which
/// is the tier switched off inside the scalar, so nothing is asked of
/// the DAG and every decision is the numeric one.
fn is_identically_zero(id: SymId) -> bool {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let Some(sess) = slot.as_mut() else {
            return false;
        };
        if sess.budget.max_terms == 0 {
            return false;
        }
        form_in(sess, id).is_zero()
    })
}

/// Records how one decision was answered, for the session's receipt.
fn count_decision(symbolic: bool) {
    SESSION.with(|s| {
        if let Some(sess) = s.borrow_mut().as_mut() {
            if symbolic {
                sess.counts.symbolic_zero += 1;
            } else {
                sess.counts.numeric += 1;
            }
        }
    });
}

// --------------------------------------------------------- the scalar

/// **The symbolic tier's lane scalar**: the value `T` computes as
/// today, plus a handle into the expression DAG (module docs).
///
/// Every [`Real`] operation computes the value at `T` VERBATIM — so a
/// `Sym<T>` run is bit-identical to a `T` run in its numeric channel by
/// construction — and mints one content-hashed node beside it. Only
/// [`Decide::sign_within`] behaves differently, and only in the one
/// direction E12 sanctions: a margin whose expression is identically
/// zero answers `Zero` without consulting the enclosure.
///
/// `Copy`, because the handle is an id and evaluation code is
/// arithmetic-dense (the same reason [`Real`] demands it).
#[derive(Clone, Copy, Debug)]
pub struct Sym<T> {
    /// The numeric channel — the value a plain `T` run would carry.
    pub value: T,
    node: SymId,
}

impl<T> Sym<T> {
    /// This value's DAG node.
    #[must_use]
    pub fn node(self) -> SymId {
        self.node
    }

    /// A value carrying NO tracked expression: the id is the reserved
    /// unrecorded one, so its form freezes and every decision on it is
    /// the numeric one.
    ///
    /// The door for a lane that legitimately has no expression to track
    /// — a bracket handed back by an engine that ran at another scalar
    /// — where fabricating a node would claim an algebraic relationship
    /// that was never computed.
    #[must_use]
    pub fn opaque(value: T) -> Self {
        Self {
            value,
            node: SymId::UNRECORDED,
        }
    }

    /// A value bound as the document PARAMETER `symbol` — the one door
    /// that introduces an indeterminate.
    #[must_use]
    pub fn param(symbol: ParamSymbol, value: T) -> Self {
        Self {
            value,
            node: intern(SymNode {
                op: SymOp::Param,
                payload: symbol.0,
                kids: [SymId::UNRECORDED; 2],
            }),
        }
    }

    /// Mints the node for a nullary op.
    fn nullary(value: T, op: SymOp, payload: u64) -> Self {
        Self {
            value,
            node: intern(SymNode {
                op,
                payload,
                kids: [SymId::UNRECORDED; 2],
            }),
        }
    }

    /// Mints the node for a one-child op.
    fn unary(self, value: T, op: SymOp, payload: u64) -> Self {
        Sym {
            value,
            node: intern(SymNode {
                op,
                payload,
                kids: [self.node, SymId::UNRECORDED],
            }),
        }
    }

    /// Mints the node for a two-child op.
    fn binary(self, other: Self, value: T, op: SymOp) -> Self {
        Sym {
            value,
            node: intern(SymNode {
                op,
                payload: 0,
                kids: [self.node, other.node],
            }),
        }
    }
}

impl<T: Real> Add for Sym<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.binary(rhs, self.value + rhs.value, SymOp::Add)
    }
}

impl<T: Real> Sub for Sym<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.binary(rhs, self.value - rhs.value, SymOp::Sub)
    }
}

impl<T: Real> Mul for Sym<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.binary(rhs, self.value * rhs.value, SymOp::Mul)
    }
}

/// Division mints `a · Inv(b)` (module docs): `Inv` is an opaque atom,
/// so `(a/b)·b` does not fold back to `a` and the tier claims nothing
/// about it.
impl<T: Real> Div for Sym<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let inv = rhs.unary(rhs.value, SymOp::Inv, 0);
        self.binary(inv, self.value / rhs.value, SymOp::Mul)
    }
}

impl<T: Real> Neg for Sym<T> {
    type Output = Self;

    fn neg(self) -> Self {
        self.unary(-self.value, SymOp::Neg, 0)
    }
}

impl<T: Real> Real for Sym<T> {
    fn from_f64(x: f64) -> Self {
        Self::nullary(T::from_f64(x), SymOp::Lit, x.to_bits())
    }

    fn zero() -> Self {
        // The same node a `from_f64(0.0)` mints, so the two spellings
        // of the additive identity share one id.
        Self::nullary(T::zero(), SymOp::Lit, 0f64.to_bits())
    }

    fn one() -> Self {
        Self::nullary(T::one(), SymOp::Lit, 1f64.to_bits())
    }

    fn pi() -> Self {
        Self::nullary(T::pi(), SymOp::Pi, 0)
    }

    /// τ's VALUE is `T::tau()` — the base scalar's own constant, so the
    /// numeric channel is untouched — while its NODE is `2·π`, which is
    /// what τ is as a real. Both enclose the same real number, which is
    /// all the identity test ever claims.
    fn tau() -> Self {
        let two = Self::from_f64(2.0);
        let pi = Self::pi();
        Sym {
            value: T::tau(),
            node: intern(SymNode {
                op: SymOp::Mul,
                payload: 0,
                kids: [two.node, pi.node],
            }),
        }
    }

    fn sqrt(self) -> Self {
        self.unary(self.value.sqrt(), SymOp::Sqrt, 0)
    }

    fn abs(self) -> Self {
        self.unary(self.value.abs(), SymOp::Abs, 0)
    }

    fn is_poison(self) -> bool {
        self.value.is_poison()
    }

    fn powi(self, n: i32) -> Self {
        self.unary(self.value.powi(n), SymOp::Powi, u64::from(n as u32))
    }

    fn sin_cos(self) -> (Self, Self) {
        let (s, c) = self.value.sin_cos();
        (
            self.unary(s, SymOp::Sin, 0),
            self.unary(c, SymOp::Cos, 0),
        )
    }

    fn tan(self) -> Self {
        self.unary(self.value.tan(), SymOp::Tan, 0)
    }

    fn asin(self) -> Self {
        self.unary(self.value.asin(), SymOp::Asin, 0)
    }

    fn acos(self) -> Self {
        self.unary(self.value.acos(), SymOp::Acos, 0)
    }

    fn atan(self) -> Self {
        self.unary(self.value.atan(), SymOp::Atan, 0)
    }

    fn atan2(self, x: Self) -> Self {
        self.binary(x, self.value.atan2(x.value), SymOp::Atan2)
    }

    fn min(self, other: Self) -> Self {
        self.binary(other, self.value.min(other.value), SymOp::Min)
    }

    fn max(self, other: Self) -> Self {
        self.binary(other, self.value.max(other.value), SymOp::Max)
    }

    fn floor(self) -> Self {
        self.unary(self.value.floor(), SymOp::Floor, 0)
    }

    fn copysign(self, sign: Self) -> Self {
        self.binary(sign, self.value.copysign(sign.value), SymOp::Copysign)
    }
}

/// The bracket is the value channel's, verbatim: the DAG carries no
/// numbers of its own and is never read here.
impl<T: Bounds> Bounds for Sym<T> {
    fn lo(self) -> f64 {
        Bounds::lo(self.value)
    }

    fn hi(self) -> f64 {
        Bounds::hi(self.value)
    }
}

/// Certification delegates: whether the computation was defined on the
/// whole box is a question about the numeric channel, and the symbolic
/// tier neither widens nor narrows the answer.
impl<T: CertifiedEnclosure> CertifiedEnclosure for Sym<T> {
    fn certified_bracket(self) -> Option<(f64, f64)> {
        self.value.certified_bracket()
    }
}

/// Span selection is STRUCTURE selection and reads the value channel;
/// the hull mints a `Hull` node keyed by the two operands' ids (never by
/// their forms — see [`SymOp::Hull`]).
impl<T: SpanLocate> SpanLocate for Sym<T> {
    fn locate_spans(self, knots: &KnotVector) -> SpanSet {
        self.value.locate_spans(knots)
    }

    fn enclosure_hull(self, other: Self) -> Self {
        self.binary(other, self.value.enclosure_hull(other.value), SymOp::Hull)
    }
}

/// **The whole of the tier's effect on decision-making** (E12): the
/// symbolic step happens INSIDE the scalar, so `k_stats::decide`, its
/// private `classify` and every funnel site are untouched by
/// construction.
///
/// The two clauses of the theorem, in order:
///
/// 1. `certified_bracket()` — the computation was defined on the whole
///    input box. Without it `sqrt(-1) − sqrt(-1)` would decide `Zero`
///    on an expression with no real value; this is the same door
///    [`crate::Interval::sign_within`] refuses at, consulted for the
///    same reason and BEFORE the identity test rather than after.
/// 2. the node's normal form is the zero polynomial, computed with exact
///    rational coefficients from the parameter symbols down. Nothing in
///    that computation reads a value.
///
/// Together they say: the margin is a real number, and that real number
/// is zero at every parameter point of the box. `Sign::Zero` is then a
/// theorem, not a measurement — which is why no band is consulted and
/// why the answer does not depend on the box's width.
///
/// Everything else is `T::sign_within` verbatim.
impl<T: Decide + CertifiedEnclosure> Decide for Sym<T> {
    fn sign_within(self, band: Band) -> Result<Sign, Indeterminate> {
        if self.value.certified_bracket().is_some() && is_identically_zero(self.node) {
            count_decision(true);
            #[cfg(feature = "probe")]
            crate::k_stats::record_symbolic_zero(self.value.certified_bracket(), band);
            return Ok(Sign::Zero);
        }
        count_decision(false);
        self.value.sign_within(band)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::predicate::Margin;
    use crate::tolerance::Tol;

    fn budget() -> SymBudget {
        SymBudget {
            max_terms: 4096,
            max_degree: 16,
        }
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness tolerance has a linear band")
    }

    /// The parameter, at `f64`: a point value with a symbol on it.
    fn p(name: &str, v: f64) -> Sym<f64> {
        Sym::param(ParamSymbol::of(name), v)
    }

    fn decides_zero(m: Sym<f64>) -> bool {
        crate::k_stats::decide("sym_test", Margin::of(m), band()) == Ok(Sign::Zero)
    }

    #[test]
    fn a_literal_difference_is_the_zero_form() {
        let (out, counts) = with_session(budget(), || {
            let x = p("w", 0.37);
            let a = x + Sym::from_f64(2.0) * x;
            let b = Sym::from_f64(3.0) * x;
            decides_zero(a - b)
        });
        assert!(out, "3x written two ways is the same polynomial");
        assert_eq!(counts.symbolic_zero, 1);
        assert_eq!(counts.numeric, 0);
    }

    /// The shape most identity margins arrive in: a NORM of a vector
    /// that is componentwise zero. `sqrt` is an opaque atom, so this
    /// only works because an atom over a zero form folds to `f(0)`.
    #[test]
    fn a_norm_of_a_zero_vector_decides_symbolically() {
        let (out, _) = with_session(budget(), || {
            let t = p("depth", 0.5);
            let one = Sym::from_f64(1.0);
            // (P + t·d) − (P + t·d), componentwise, then the norm.
            let comp = |k: f64| {
                let base = Sym::from_f64(k);
                let a = base + t * one;
                let b = base + one * t;
                a - b
            };
            let (x, y, z) = (comp(3.0), comp(-1.5), comp(0.0));
            let n = (x * x + y * y + z * z).sqrt();
            decides_zero(n)
        });
        assert!(out, "the norm of a componentwise-zero vector is zero");
    }

    /// A COINCIDENCE at the nominal is not an identity: two segments
    /// collinear at `p = 0` only. It never decides symbolically, at any
    /// parameter value.
    #[test]
    fn a_coincidence_at_the_nominal_never_decides_symbolically() {
        for v in [0.0, 1e-12, 0.5] {
            let (_, counts) = with_session(budget(), || {
                let x = p("w", v);
                decides_zero(x * x)
            });
            // At v = 0 the NUMERIC channel answers `Zero`, and that is
            // the point: a coincidence is decided by the enclosure, at
            // the width the enclosure has, and widens with the box. The
            // tier never claims it.
            assert_eq!(counts.symbolic_zero, 0, "at {v}");
            assert_eq!(counts.numeric, 1, "at {v}");
        }
    }

    /// The cross product of a direction with itself: zero in every
    /// component, by different routes through the same symbols.
    #[test]
    fn a_self_cross_product_is_identically_zero() {
        let (out, _) = with_session(budget(), || {
            let d = [p("a", 0.3), p("b", -0.7), p("c", 0.1)];
            let cross = [
                d[1] * d[2] - d[2] * d[1],
                d[2] * d[0] - d[0] * d[2],
                d[0] * d[1] - d[1] * d[0],
            ];
            cross.into_iter().all(decides_zero)
        });
        assert!(out, "d x d is the zero vector, symbolically");
    }

    /// π's own identity: `τ − 2π` is zero as a real, and the form says
    /// so without reading either constant's value.
    #[test]
    fn tau_is_two_pi_in_the_form() {
        let (out, _) = with_session(budget(), || {
            decides_zero(<Sym<f64> as Real>::tau() - Sym::from_f64(2.0) * <Sym<f64> as Real>::pi())
        });
        assert!(out);
    }

    /// The documented limits, pinned as limits: no `Inv(b)·b`, no
    /// trigonometric identity. Both fall to the numeric channel.
    #[test]
    fn the_opaque_atoms_are_opaque() {
        let (_, counts) = with_session(budget(), || {
            let x = p("w", 3.0);
            let y = p("h", 2.0);
            let inv = (x / y) * y - x;
            let (s, c) = x.sin_cos();
            let pyth = s * s + c * c - Sym::from_f64(1.0);
            (decides_zero(inv), decides_zero(pyth))
        });
        // Both are numerically zero at this point — the numeric channel
        // answers them, as it always did. What is pinned is that the
        // TIER claims neither: `Inv` and the trigonometric pair are
        // opaque atoms and no form of theirs is the zero polynomial.
        assert_eq!(counts.symbolic_zero, 0, "no factoring, no trig identities");
        assert_eq!(counts.numeric, 2);
    }

    /// A poisoned expression never decides `Zero` however zero its form
    /// is: clause 1 of the theorem, on the value channel.
    #[test]
    fn a_domain_violation_never_certifies_symbolically() {
        let (out, _) = with_session(budget(), || {
            let neg = Sym::from_f64(-1.0);
            let r = neg.sqrt();
            decides_zero(r - r)
        });
        assert!(!out, "sqrt(-1) - sqrt(-1) is not a certified zero");
    }

    /// A budget of zero terms is the tier switched off inside the
    /// scalar: nothing is asked of the DAG at all.
    #[test]
    fn a_zero_budget_decides_everything_numerically() {
        let (_, counts) = with_session(SymBudget::none(), || {
            let x = p("w", 0.37);
            decides_zero(x - x)
        });
        assert_eq!(counts.symbolic_zero, 0);
        assert_eq!(counts.numeric, 1);
        assert_eq!(counts.frozen, 0, "nothing is even computed");
    }

    /// Freezing is SOUND, not silent: a form driven past the term
    /// budget decides numerically and the freeze is counted.
    #[test]
    fn an_over_budget_form_freezes_and_is_counted() {
        let tight = SymBudget {
            max_terms: 2,
            max_degree: 16,
        };
        let (out, counts) = with_session(tight, || {
            let (x, y, z) = (p("a", 1.0), p("b", 2.0), p("c", 3.0));
            let wide = x + y + z;
            // The sum has three terms: over budget, so it freezes into
            // an atom. The DIFFERENCE of two identical frozen atoms is
            // still zero, which is sound — same node, same real.
            let out = decides_zero(wide - wide);
            (out, session_counts())
        });
        assert!(out.0, "identical frozen nodes still cancel");
        assert!(counts.frozen >= 1, "the freeze is counted: {counts:?}");
        assert_eq!(
            out.1.map(|c| c.frozen),
            Some(counts.frozen),
            "the mid-replay door reports the same freezes the session ends with"
        );
    }

    /// D9: the node ids are content hashes, so two sessions building the
    /// same expression in different orders agree bit for bit, and no
    /// table is shared between them.
    #[test]
    fn node_ids_are_bit_identical_across_sessions_and_orders() {
        let build_forward = || {
            let x = p("w", 1.0);
            let y = p("h", 2.0);
            (x * y + x).node().bits()
        };
        let build_backward = || {
            let y = p("h", 2.0);
            let x = p("w", 1.0);
            let m = x * y;
            (m + x).node().bits()
        };
        let (a, _) = with_session(budget(), build_forward);
        let (b, _) = with_session(budget(), build_backward);
        let (c, _) = with_session(SymBudget::none(), build_forward);
        assert_eq!(a, b);
        assert_eq!(a, c, "the id does not depend on the budget");
        // And outside any session at all.
        assert_eq!(a, build_forward());
    }

    /// The rational is exact on every `f64` it accepts, and refuses the
    /// ones that are not real numbers.
    #[test]
    fn the_rational_embeds_floats_exactly() {
        for x in [0.0, 1.0, -0.5, 0.1, 3.1e-3, f64::MIN_POSITIVE] {
            let r = Rat::of_f64(x).expect("a finite float is a dyadic rational");
            // num/den · 2^exp2 back to a float, when the parts are small
            // enough for the round trip to be exact.
            if r.num.unsigned_abs() < (1 << 53) && r.den == 1 {
                let back = (r.num as f64) * 2f64.powi(r.exp2);
                assert_eq!(back.to_bits(), x.to_bits(), "round trip of {x}");
            }
        }
        assert!(Rat::of_f64(f64::NAN).is_none());
        assert!(Rat::of_f64(f64::INFINITY).is_none());
    }
}
