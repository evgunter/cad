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
//! **The documented limits.** The normal form is a QUOTIENT of
//! polynomials over the parameter symbols — a field of fractions, not a
//! polynomial ring — so a reciprocal is a first-class part of it and
//! `(x/y)·y − x` DOES decide symbolically: `x/y` is the form `x` over
//! `y`, multiplying by `y` gives `xy/y`, and the difference's numerator
//! is the zero polynomial. (The tier's headline row needs exactly that:
//! an extruded strut's carrier is `origin + (w/‖w‖)·t`, so its endpoint
//! residual is literally `w·(‖w‖·‖w‖⁻¹ − 1)`.) That is the whole of the
//! reciprocal's reach, and it is a NORMAL FORM rather than a rewrite
//! rule: nothing is factored, and no simplification is attempted.
//!
//! What remains outside the DEFAULT tier: no factoring, and no
//! functional identity of any opaque atom. `sin² + cos² − 1` does not
//! decide symbolically, `sqrt(x)·sqrt(x) − x` does not, and `|x| − x` on
//! a nonnegative `x` does not. Each atom is an indeterminate keyed by
//! its argument's form, so two occurrences of ONE atom cancel and
//! nothing else about it is known. These are limits of the tier and not
//! bugs in it — over-refusal is the safe direction, and every such
//! margin falls to the numeric channel exactly as before.
//!
//! # The atom algebra (M10-8), and why it is filed
//!
//! Two BUILDABLE functional-identity rules sit behind the [`SymRules`]
//! dial — **A** `sqrt(X)² = X` and **B** `sin² + cos² = 1` — applied as
//! a reduction over the top RESIDUAL a decide site tests
//! ([`is_identically_zero`]), never during form construction, so a rule
//! can only ever ADD a discharge and never disturb what the plain
//! quotient form already reaches. Both read no value; on small forms
//! they are sound and effective (this module's rule-A/B unit rows).
//!
//! **Rule C** — `sqrt(Q²) = Q` for a `Q` whose sign is certified over
//! the leaf box (clause 3, the one fold that would read a value) — is
//! FILED UNBUILT. Reading a value at the lane scalar needs either
//! `dyn Any` to store the parameter's `T` (the bit-identity-punning
//! discipline forbids it) or the feature-gated `Interval` type (this
//! module is feature-agnostic, and the read must also serve the
//! `f64`/`Probe` lanes). So the shipped and buildable tier reads NO
//! value — E12's original invariant holds literally — and
//! [`SymCounts::sign_gated`] with its K token stands as the reserved
//! instrument for the day a value-reading fold is built within the
//! discipline.
//!
//! All of it — A, B and C — is **filed, not shipped**
//! ([`SymRules::shipped`] is empty), and the reason is measured (M10-8's
//! §1 table, over the two-hole plate and both reviewers' brackets): the
//! arc family the rules target — a swept arc's carrier carries
//! `u_ref·u_ref = (v·v)/sqrt(v·v)²`, which rule A reduces to `1` — lives
//! in forms large enough to FREEZE before a top-residual reduction can
//! reach them, so on all three documents the rules move no ceiling and
//! discharge no decision the plain form did not. An EARLY reduction (per
//! DAG node, before the freeze) does discharge the family and was
//! measured to raise the filleted bracket's whole-certifying box ~10×,
//! but paying a reduction per node is a runaway and letting it replace
//! the plain form downgrades identities the plain form proves. A
//! bounded, non-downgrading early reduction is the mechanism this unit
//! could not land; the default tier stays the M10-7 quotient form, bit
//! for bit.
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
//! **The coefficients are arbitrary-precision dyadic-scaled rationals**
//! ([`Rat`], over `num-bigint`), bounded at [`COEFF_BITS`] bits. They
//! were an in-tree `i128` through M10-7, on the argument that
//! `geom-core`'s runtime dependencies were `libm` alone and that
//! nothing measured was losing a cancellation to the overflow — the
//! whole-box replays reported `frozen: 0` on the bracket because the
//! `Decide` impl skips the form of a margin the numeric channel has
//! already proved non-zero. M10-8 measured the case the whole-box
//! replays cannot see: at a document's NOMINAL, where every identity
//! margin is near zero and every form is built, the plate froze 1,056
//! forms, R2's bracket 1,978 and R1's annulus 1,034 — and the plate's
//! own ceiling residual (`carrier_endpoint_start`, the rim's
//! `‖q − c‖ = r`) is a polynomial of degree 12 in a radius whose nominal
//! is an `f64` literal with a 53-bit mantissa. Three such factors
//! overflow an `i128`; the residual has twelve. The overflow was the
//! freeze, the freeze was the ceiling, and no rule can reach an atom
//! inside a frozen form. The bound keeps the freeze discipline: a
//! coefficient past it is refused exactly as an overflow was, so a
//! blow-up is a counted freeze and never an allocation to the ceiling.
//!
//! # The census: which identity-shaped predicates this tier reaches
//!
//! Two greps over `crates/` and `demos/` — one for the names handed to a
//! funnel door, one for identity/gap-shaped string literals — and their
//! union minus the bare filter words and the test-harness names. **107
//! names.** The rule is written out in
//! `work/cert/symbolic-tier-census.md`, which also carries the full
//! table: one row per name, with its bucket, its evidence and its site.
//! Only the counts and the two families that matter are here.
//!
//! | bucket | count |
//! | --- | --- |
//! | IMPLICIT (S-CERT's frontier) | 4 |
//! | NOT A PREDICATE | 8 |
//! | EXPLICIT | 95 |
//!
//! **107 and not the 66 the previous sweep reported**, because that
//! number is not re-derivable from a rule written down anywhere and this
//! one states its own. The difference is filter width, not new
//! predicates.
//!
//! **IMPLICIT — 4**, and this is the census's load-bearing claim:
//! `ssi_on_locus` and `ssi_on_locus_foot` (a marched intersection
//! point's residual and the foot of its projection),
//! `plane_nurbs_on_locus` (a chart-image foot) and
//! `offset_reanchor_on_carrier` (an offset carrier re-anchored through a
//! solve) — EXACTLY the four S-CERT's frontier item already names, at
//! either filter width. A quantity found by iteration has no expression
//! in the parameters, so no normal form reaches it and its residual
//! widens with the box whatever this tier does.
//!
//! **NOT A PREDICATE — 8.** Seven are `pncad-py` TAG strings for error
//! and enum variants; `carrier_kind` is a diagnostic name on an
//! `Indeterminate` carrying `MarginDiag::Invalid`
//! (`topo/src/boolean/carrier_eq.rs`) — a structure contradiction, with
//! no margin ever classified.
//!
//! **EXPLICIT — 95.** Closed forms in the parameters over analytic
//! carriers. Nine carry a MEASURED symbolic/numeric split from
//! `editor-core/tests/m10_7_census_probe.rs` (at `Sym<Probe>`, through
//! the same funnel, over the M10 fixtures and the tour's plate):
//! `carrier_endpoint_start` and `carrier_endpoint_end` (56/16 each),
//! `carrier_matches_mapped_source` (288/72), `carrier_on_surface_1` and
//! `carrier_on_surface_2` (216/72 each), `carrier_circles_identity`
//! (6/0), `side_cylinders_cosurface` (4/0), `carrier_cyl_axis_parallel`
//! (3/0), and `side_planes_cosurface` at 0/8. The rest carry their site,
//! and many run at `f64` over no parameter box at all — a fact about
//! this repository's fixtures rather than about their margins.
//!
//! `side_planes_cosurface`'s 0/8 is not a miss: consecutive walls of a
//! rectangle are genuinely NOT cosurface, so all eight are definite
//! non-coincidences. Its margin,
//! `perp_dot(normalize(prev.b − prev.a), next.b − prev.a)`
//! (`sweep/src/swept.rs`), is a quotient of polynomials and the form
//! reaches it wherever the walls really are cosurface — which is what
//! `side_cylinders_cosurface` at 4/0 on the plate's holes shows.
//!
//! # What the census CANNOT see, and it is the expensive part
//!
//! Both sweeps filter on WORDS, so a predicate whose name contains none
//! of them is invisible to the instrument however much it decides. Five
//! such names appear in the driver's own K CSV:
//! `newell_plane_residual` (1,584 symbolic decisions),
//! `segment_straightness` (1,650), `witness_at_mid_parameter` (1,377),
//! `dihedral_wedge`, and `arc_diameter_clearance`.
//!
//! Two of them are not a footnote. **`dihedral_wedge` is what sets the
//! slab's certification ceiling** — it lands in the band over a wide box
//! — and `newell_plane_residual`'s INVALID arm is what a one-leaf replay
//! of a still wider box fails on. So the predicate that bounds
//! certification today is one this census was structurally unable to
//! name, which says the instrument answers "which identity-shaped
//! predicates does the tier reach" and NOT "which predicates bound
//! certification". Those are different populations and the second one is
//! `work/m10/real-margin-dependency-widening.md`.
//!
//! # The family this tier MISSES, named (E12's reserve)
//!
//! E12 keeps discharge-by-provenance in reserve "only if the census
//! shows a family the symbolic tier misses". It does, and this is it.
//!
//! An ARC rim's endpoint pinning. `sweep::swept` gives a swept arc the
//! carrier `Circle { center: c, radius: r, u_ref: (q − c).normalize() }`
//! and the certifier checks `‖carrier.eval(0) − q‖ ≤ ε`, which expands
//! to `‖c + (q − c)·r/‖q − c‖ − q‖`. That is zero iff `‖q − c‖ = r`,
//! which is TRUE of the geometry and is not a rational-function identity
//! in the parameters: it needs `sqrt(r²) = r`, a fact about the sign of
//! the radius rather than about algebra. No normal form over a field of
//! fractions can see it, and no budget makes it visible.
//!
//! MEASURED consequence, on the tour's own two-hole plate
//! (`m10_7_census_probe::measure_the_ceiling_on_the_two_hole_plate`):
//! the widest box that certifies whole is `7.81e-7` of the real study
//! with the tier ON and `7.81e-7` with it OFF — unmoved — and the first
//! refusal beyond it is `carrier_endpoint_start` with the enclosure
//! `[0, 1.25e-9]` against a coincidence threshold of `1e-9`. On the
//! straight-walled slab, where no normalization stands between the
//! carrier and its endpoint, the same measurement moves from a
//! half-width of `ε/16` to `0.488` on a `1.0` nominal — a factor of
//! about `8·10^9`.
//!
//! The recourse E12 names is a provenance token ("built as the arc's far
//! endpoint"), discharged structurally and verified at the f64 witness
//! point. It is not taken here.
//!
//! # No session, no tier — and what that does NOT mean
//!
//! Ids are computable without the table, so a [`Sym<T>`] built outside
//! [`with_session`] still carries a deterministic id — the lookup simply
//! misses, the form freezes, and the decision falls to the numeric
//! channel.
//!
//! An earlier draft of this paragraph said "the tier is never partially
//! on", and that sentence was false. A node minted before the session
//! was installed is not IN the session's table, so its form freezes to
//! an indeterminate keyed by its own id — and two occurrences of that
//! same node share the id, so they still cancel: `a − a` decides `Zero`
//! for such an `a`, inside a session that never saw it built. That is
//! SOUND (one id is one expression, so the cancellation is a real
//! theorem about a real subexpression) but it is not "off". The true
//! statement is narrower and it is the one that matters: **a node the
//! session cannot expand contributes an unknown, never a value** — so
//! the tier can only ever discharge FEWER identities than it would with
//! the full table, never more, and mixing a pre-session node into a
//! session's DAG cannot manufacture a theorem that a fully-recorded
//! replay would not also reach.

use core::cell::{Cell, RefCell};
use core::ops::{Add, Div, Mul, Neg, Sub};
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive};

use crate::predicate::{Band, Decide, Indeterminate, MarginDiag, Sign};
use crate::real::{Bounds, CertifiedEnclosure, Real};
use crate::spline::{KnotVector, SpanLocate, SpanSet};

/// The atom algebra: the rule A/B reductions over a residual.
#[path = "sym/algebra.rs"]
mod algebra;
/// The shape report — the instrument that says, per decide site that
/// stayed numeric, what blocked it.
#[path = "sym/report.rs"]
pub mod report;
/// Rule C: the polynomial square root and the clause-3 fold, with the
/// one value read the tier makes (a parameter bracket in the ring).
#[path = "sym/signed.rs"]
mod signed;

// ---------------------------------------------------------------- ids

/// A DAG node's identity: the 128-bit structural content hash of
/// `(op, children ids, payload bits)` (module docs).
///
/// Never a sequence number, so it is stable across rayon schedules,
/// insertion orders and repeats — D9 for free.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymId(u128);

impl SymId {
    /// The **empty child slot**, and nothing else. Reserved: the mixer
    /// never produces it, so it can never collide with a node's id.
    ///
    /// It used to be a second thing as well — the id every
    /// [`Sym::opaque`] value carried — and that was a soundness defect,
    /// because one id means one expression: `opaque(1.0) - opaque(2.0)`
    /// hashed to a difference of a node with ITSELF, whose form is the
    /// zero polynomial, and `sign_within` answered `Zero` on two values
    /// that are not equal. Two untracked reals are two unknowns, so each
    /// opaque value now mints its OWN indeterminate ([`SymOp::Opaque`])
    /// and this constant names one thing.
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
///
/// **The third FNV in this tree, and deliberately not shared with the
/// other two.** `editor_core::eval::memo` runs two 64-bit FNV lanes for
/// evaluation CONTENT KEYS, and `editor_core::stackup`'s `Digest` runs
/// one for a pairing COMPARISON. All three are FNV-1a because FNV-1a is
/// a dozen lines with no dependency and a fixed spec, which is the
/// property each of them wants; that is a shared REASON, not shared
/// code. They are not one type because they answer to different
/// contracts: this one is a node identity that must agree across
/// processes and builds forever (a change to it changes every id and
/// every memoized form), the memo's keys never leave the process, and
/// the stackup digest is explicitly "never a content key". Hoisting
/// them together would put the loosest contract and the strictest one
/// behind one name, and `geom-core` cannot depend on `editor-core` in
/// any case. Said here so the duplication is a decision rather than an
/// oversight.
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
    /// **One untracked real**, minted by [`Sym::opaque`]: a value whose
    /// expression this session did not build, carrying a per-session
    /// sequence number in its payload so that each such value is its
    /// OWN unknown. Never keyed by the value's bits — two intervals
    /// that happen to be equal are still two different reals, and
    /// keying by bits would re-introduce the false theorem this op
    /// exists to prevent.
    Opaque,
    Lit,
    Pi,
    Add,
    Sub,
    Mul,
    Neg,
    /// An integer power, **EXPANDED into the form** rather than kept as
    /// an opaque atom.
    ///
    /// The unit's spec listed `powi` among the opaque atoms and this is
    /// a deliberate departure from it, disclosed as a deviation: an
    /// integer power of a rational function IS a rational function, so
    /// expanding it costs nothing in soundness and buys every
    /// cancellation that runs through a square. It is what makes a
    /// SQUARED DISTANCE cancel — `‖a − b‖²` reaching the form as a sum
    /// of squares rather than as an unknown — and squared distances are
    /// most of what the certification identities are written in. The
    /// expansion is budget-checked at every step ([`powi_form`]), so a
    /// large exponent freezes rather than allocating its way to the
    /// ceiling.
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
            Self::Opaque => 24,
        }
    }

    /// How many of the node's two child slots this op reads.
    fn arity(self) -> usize {
        match self {
            Self::Param | Self::Opaque | Self::Lit | Self::Pi => 0,
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

/// **The coefficient integer: an `i128` inline, a `BigInt` only past
/// it.** The ring is arbitrary-precision under [`COEFF_BITS`], but the
/// overwhelming majority of a document's coefficients fit a machine
/// word — the round constants, the small integers, the products that
/// used to fit an `i128` — and measured, a `BigInt` for every one of
/// them cost 4× per leaf on R2's bracket and 100× on a nominal replay
/// (heap traffic, not arithmetic). So every operation runs the checked
/// `i128` path first and promotes to a heap integer only on overflow,
/// and every result that fits demotes back, which keeps the
/// representation canonical (one value, one variant) so equality and
/// the digest read it directly.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Int {
    Small(i128),
    Big(Box<BigInt>),
}

impl Int {
    fn zero() -> Self {
        Self::Small(0)
    }

    fn one() -> Self {
        Self::Small(1)
    }

    /// Canonical: a big integer that fits an `i128` is `Small`.
    fn from_big(b: BigInt) -> Self {
        match i128::try_from(&b) {
            Ok(v) => Self::Small(v),
            Err(_) => Self::Big(Box::new(b)),
        }
    }

    fn big(&self) -> BigInt {
        match self {
            Self::Small(v) => BigInt::from(*v),
            Self::Big(b) => (**b).clone(),
        }
    }

    fn is_zero(&self) -> bool {
        matches!(self, Self::Small(0))
    }

    fn is_one(&self) -> bool {
        matches!(self, Self::Small(1))
    }

    fn is_negative(&self) -> bool {
        match self {
            Self::Small(v) => *v < 0,
            Self::Big(b) => b.is_negative(),
        }
    }

    fn bits(&self) -> u64 {
        match self {
            Self::Small(v) => u64::from(128 - v.unsigned_abs().leading_zeros()),
            Self::Big(b) => b.bits(),
        }
    }

    fn neg(&self) -> Self {
        match self {
            Self::Small(v) => match v.checked_neg() {
                Some(n) => Self::Small(n),
                None => Self::from_big(-BigInt::from(*v)),
            },
            Self::Big(b) => Self::from_big(-(**b).clone()),
        }
    }

    fn abs(&self) -> Self {
        if self.is_negative() {
            self.neg()
        } else {
            self.clone()
        }
    }

    fn add(&self, o: &Self) -> Self {
        if let (Self::Small(a), Self::Small(b)) = (self, o)
            && let Some(v) = a.checked_add(*b)
        {
            return Self::Small(v);
        }
        Self::from_big(self.big() + o.big())
    }

    fn mul(&self, o: &Self) -> Self {
        if let (Self::Small(a), Self::Small(b)) = (self, o)
            && let Some(v) = a.checked_mul(*b)
        {
            return Self::Small(v);
        }
        Self::from_big(self.big() * o.big())
    }

    fn shl(&self, k: usize) -> Self {
        if let Self::Small(a) = self
            && k < 127
            && let Some(v) = a.checked_mul(1i128 << k)
        {
            return Self::Small(v);
        }
        Self::from_big(self.big() << k)
    }

    /// The greatest common divisor of the magnitudes (positive).
    fn gcd(&self, o: &Self) -> Self {
        if let (Self::Small(a), Self::Small(b)) = (self, o) {
            let g = gcd_u128(a.unsigned_abs(), b.unsigned_abs());
            return match i128::try_from(g) {
                Ok(v) => Self::Small(v),
                Err(_) => Self::from_big(BigInt::from(g)),
            };
        }
        Self::from_big(self.big().gcd(&o.big()))
    }

    /// Exact division by a divisor known to divide.
    fn div_exact(&self, d: &Self) -> Self {
        if let (Self::Small(a), Self::Small(b)) = (self, d)
            && let Some(v) = a.checked_div(*b)
        {
            return Self::Small(v);
        }
        Self::from_big(self.big() / d.big())
    }

    /// The odd part and the number of twos stripped (`0` keeps zero).
    fn strip_twos(&self) -> (Self, u64) {
        match self {
            Self::Small(0) => (Self::Small(0), 0),
            Self::Small(v) => {
                let k = v.trailing_zeros();
                (Self::Small(v >> k), u64::from(k))
            }
            Self::Big(b) => {
                let k = b.trailing_zeros().unwrap_or(0);
                (Self::from_big((**b).clone() >> k), k)
            }
        }
    }

    /// `Some(r)` iff `r·r == self` exactly, for `self >= 0`.
    fn isqrt_exact(&self) -> Option<Self> {
        if self.is_negative() {
            return None;
        }
        if let Self::Small(v) = self {
            let r = isqrt_u128(v.unsigned_abs())?;
            return i128::try_from(r).ok().map(Self::Small);
        }
        let b = self.big();
        let r = b.sqrt();
        (&r * &r == b).then(|| Self::from_big(r))
    }

    /// A rounded `f64` (at most an ulp off), or `None` past the range.
    fn to_f64(&self) -> Option<f64> {
        match self {
            Self::Small(v) => Some(*v as f64),
            Self::Big(b) => b.to_f64(),
        }
    }

    /// Feeds the integer to a content hash: the sign and the digits.
    fn feed(&self, h: Hash128) -> Hash128 {
        match self {
            Self::Small(v) => h.word(0).wide(*v as u128),
            Self::Big(b) => {
                let (_, digits) = b.to_u32_digits();
                let mut h = h.word(u64::from(b.is_negative())).word(digits.len() as u64);
                for d in digits {
                    h = h.word(u64::from(d));
                }
                h
            }
        }
    }
}

impl core::fmt::Display for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Small(v) => write!(f, "{v}"),
            Self::Big(b) => write!(f, "{b}"),
        }
    }
}

/// The greatest common divisor of two magnitudes.
fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// `Some(r)` iff `r·r == n` exactly.
fn isqrt_u128(n: u128) -> Option<u128> {
    if n < 2 {
        return Some(n);
    }
    let mut x = (n as f64).sqrt() as u128;
    if x == 0 {
        x = 1;
    }
    for _ in 0..8 {
        x = (x + n / x) / 2;
    }
    while x.checked_mul(x).is_none_or(|s| s > n) {
        x -= 1;
    }
    while (x + 1).checked_mul(x + 1).is_some_and(|s| s <= n) {
        x += 1;
    }
    // Integer arithmetic, spelled as a power so the interval-square
    // gate does not read it as an enclosure product.
    (x.checked_pow(2) == Some(n)).then_some(x)
}

/// An exact rational `num / den · 2^exp2`, with `num`/`den` odd and
/// coprime and `den > 0` — the normal form's coefficient.
///
/// The power of two is factored out rather than left in the pair
/// because every `f64` literal IS `m · 2^e`: keeping `e` in its own
/// field leaves the odd part alone, so the round constants a recipe is
/// full of (`1`, `½`, `2`, `¼`) never grow the integers at all.
///
/// **The integers are arbitrary-precision** (M10-8). They were `i128`,
/// and that was measured to be the arc family's freeze: a document's
/// dimensions are `f64` literals with 53-bit mantissas, so the product
/// of THREE of them overflows an `i128`, and every polynomial of degree
/// three or more in a parameter with such a nominal froze — which is
/// what the plate's rim residual is (`sqrt(…)^12`). The size discipline
/// the `i128` gave for free is kept explicitly: an integer past
/// [`COEFF_BITS`] is refused by [`Rat::new`] and the caller freezes, so
/// a coefficient blow-up is still a bounded cost, not an allocation to
/// the ceiling. Every operation is CHECKED and answers `None` on that
/// bound (module docs).
#[derive(Clone, Debug, PartialEq, Eq)]
struct Rat {
    num: Int,
    den: Int,
    exp2: i32,
}

/// The most bits either integer of a coefficient may carry before the
/// coefficient is refused and its form freezes — a COST dial as much as
/// a discipline, and set by measurement. At 4096 bits nothing on R2's
/// bracket froze and one leaf replay took 229 s against M10-7's 5.9 s:
/// with the constant fold on, every coefficient is a product of
/// dimensions' 53-bit mantissas and the forms that used to overflow an
/// `i128` grew instead to the term budget with thousand-bit
/// coefficients, and BigInt arithmetic on those is the whole cost. At
/// 256 bits — twice the `i128` the ring replaced — the bracket's and
/// the annulus's ceilings still move by the factors measured at
/// `i128`, the worst forms freeze again, and the plate's rim residual
/// (degree 12 in a 53-bit nominal, ~640 bits) does NOT fit: that is
/// the measured trade, recorded on
/// `work/m10/plate-rim-residual-needs-the-wide-coefficient-ring`.
const COEFF_BITS: u64 = 256;

impl Rat {
    fn zero() -> Self {
        Self {
            num: Int::zero(),
            den: Int::one(),
            exp2: 0,
        }
    }

    fn one() -> Self {
        Self {
            num: Int::one(),
            den: Int::one(),
            exp2: 0,
        }
    }

    /// `num / den · 2^exp2` from machine integers — the door literals
    /// and small constants come through.
    fn new(num: i128, den: i128, exp2: i32) -> Option<Self> {
        Self::from_parts(Int::Small(num), Int::Small(den), exp2)
    }

    /// Reduces `num / den · 2^exp2` to the canonical shape, refusing a
    /// zero denominator and an integer past [`COEFF_BITS`].
    fn from_parts(num: Int, den: Int, exp2: i32) -> Option<Self> {
        if den.is_zero() {
            return None;
        }
        if num.is_zero() {
            return Some(Self::zero());
        }
        let (num, den) = if den.is_negative() {
            (num.neg(), den.neg())
        } else {
            (num, den)
        };
        let g = num.gcd(&den);
        let (num, den) = if g.is_one() {
            (num, den)
        } else {
            (num.div_exact(&g), den.div_exact(&g))
        };
        let (num, nz) = num.strip_twos();
        let (den, dz) = den.strip_twos();
        let exp2 = exp2
            .checked_add(i32::try_from(nz).ok()?)?
            .checked_sub(i32::try_from(dz).ok()?)?;
        if num.bits() > COEFF_BITS || den.bits() > COEFF_BITS {
            return None;
        }
        Some(Self { num, den, exp2 })
    }

    /// The exact value of a finite `f64`; `None` for a non-finite one
    /// (which cannot be a coefficient of a real polynomial).
    fn of_f64(x: f64) -> Option<Self> {
        if !x.is_finite() {
            return None;
        }
        if x == 0.0 {
            return Some(Self::zero());
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

    fn is_zero(&self) -> bool {
        self.num.is_zero()
    }

    fn is_negative(&self) -> bool {
        self.num.is_negative()
    }

    fn add(&self, other: &Self) -> Option<Self> {
        if self.is_zero() {
            return Some(other.clone());
        }
        if other.is_zero() {
            return Some(self.clone());
        }
        // Align on the smaller exponent, shifting the other numerator up.
        let lo = self.exp2.min(other.exp2);
        let shift = |r: &Self| -> Option<Int> {
            let k = usize::try_from(r.exp2.checked_sub(lo)?).ok()?;
            if k as u64 > COEFF_BITS {
                return None;
            }
            Some(r.num.shl(k))
        };
        let (a, b) = (shift(self)?, shift(other)?);
        let num = a.mul(&other.den).add(&b.mul(&self.den));
        Self::from_parts(num, self.den.mul(&other.den), lo)
    }

    fn neg(&self) -> Option<Self> {
        Some(Self {
            num: self.num.neg(),
            den: self.den.clone(),
            exp2: self.exp2,
        })
    }

    fn abs(&self) -> Self {
        Self {
            num: self.num.abs(),
            den: self.den.clone(),
            exp2: self.exp2,
        }
    }

    fn mul(&self, other: &Self) -> Option<Self> {
        if self.is_zero() || other.is_zero() {
            return Some(Self::zero());
        }
        Self::from_parts(
            self.num.mul(&other.num),
            self.den.mul(&other.den),
            self.exp2.checked_add(other.exp2)?,
        )
    }

    /// The reciprocal; `None` for zero.
    fn recip(&self) -> Option<Self> {
        Self::from_parts(self.den.clone(), self.num.clone(), self.exp2.checked_neg()?)
    }

    /// The EXACT square root of a non-negative rational, or `None`
    /// where it is not rational (rule A0's coefficient fold and rule
    /// C's polynomial root both need exactly this). `num/den · 2^e` with
    /// `e` made even by moving one factor of two into `num`; the root is
    /// `isqrt(num)/isqrt(den) · 2^(e/2)` when both are exact.
    fn sqrt_exact(&self) -> Option<Self> {
        if self.is_negative() {
            return None;
        }
        if self.is_zero() {
            return Some(Self::zero());
        }
        let (num, exp2) = if self.exp2 % 2 != 0 {
            (self.num.shl(1), self.exp2.checked_sub(1)?)
        } else {
            (self.num.clone(), self.exp2)
        };
        let sn = num.isqrt_exact()?;
        let sd = self.den.isqrt_exact()?;
        Self::from_parts(sn, sd, exp2 / 2)
    }

    /// A conservative `f64` bracket of the value — the two rounded
    /// conversions and the division each cost at most an ulp, and the
    /// bracket is opened by four on each side. `None` when the power of
    /// two is out of `f64`'s range (a flushed zero would not be
    /// conservative).
    fn f64_bracket(&self) -> Option<(f64, f64)> {
        if self.exp2.abs() > 1000 {
            return None;
        }
        let v = self.num.to_f64()? / self.den.to_f64()? * 2f64.powi(self.exp2);
        if !v.is_finite() {
            return None;
        }
        let (mut lo, mut hi) = (v, v);
        for _ in 0..4 {
            lo = lo.next_down();
            hi = hi.next_up();
        }
        Some((lo, hi))
    }

    /// Feeds the coefficient to a content hash (the atom-keying digest):
    /// both integers and the exponent.
    fn feed(&self, h: Hash128) -> Hash128 {
        self.den
            .feed(self.num.feed(h))
            .word(u64::from(self.exp2 as u32))
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

    fn one() -> Self {
        Self::constant(Rat::one())
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
        terms.insert(vec![(id, 1)], Rat::one());
        Self { terms }
    }

    fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    /// The value of a CONSTANT polynomial (no indeterminate).
    fn as_constant(&self) -> Option<Rat> {
        match self.terms.len() {
            0 => Some(Rat::zero()),
            1 => {
                let (m, c) = self.terms.iter().next()?;
                m.is_empty().then(|| c.clone())
            }
            _ => None,
        }
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
                let sum = existing.add(&c)?;
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
            out.insert(m.clone(), c.clone())?;
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

    /// The product, or `None` for the caller to freeze.
    ///
    /// **Refused BEFORE it is built**, on bounds that cost nothing to
    /// compute: the product has at most `|a|*|b|` terms and degree
    /// exactly `deg(a) + deg(b)`. The first version built the whole
    /// product and let [`within`] reject it afterwards, which is how a
    /// single multiplication came to take 10.8 s on a reviewer's
    /// bracket — the work was done and then thrown away. Freezing is
    /// the same outcome either way; only the bill differs.
    ///
    /// The term bound is an UPPER one (colliding monomials merge), so a
    /// product whose terms would have collided down under the budget is
    /// refused where the old code would have kept it. That is a real
    /// difference and it is measured rather than assumed: on the M10-3
    /// slab and the tour's plate the frozen counts are unchanged, so
    /// nothing the shipped fixtures rely on sat in that gap. The degree
    /// bound is exact for the leading monomial in every case the form
    /// reaches, cancellation of a whole leading term needing coefficient
    /// cancellation that a product of two nonzero polynomials over a
    /// field does not produce.
    fn mul(&self, other: &Self, budget: SymBudget) -> Option<Self> {
        if self.terms.len().checked_mul(other.terms.len())? > budget.max_terms
            || self.degree().checked_add(other.degree())? > budget.max_degree
        {
            return None;
        }
        let mut out = Self::zero();
        for (ma, ca) in &self.terms {
            for (mb, cb) in &other.terms {
                out.insert(mono_mul(ma, mb)?, ca.mul(cb)?)?;
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
    /// Decisions answered `Zero` by the symbolic tier as unconditional
    /// theorems — the normal form read no value.
    pub symbolic_zero: u64,
    /// Decisions answered `Zero` through a clause-3 fold (rule C,
    /// [`SymRules::signed_root`]) — a theorem CONDITIONAL on a sign
    /// read over the leaf's box, the one value the tier reads
    /// ([`signed`]). Kept apart from `symbolic_zero` because the two
    /// claims differ in kind; the matching K token is `sign_gated`.
    pub sign_gated: u64,
    /// Decisions handed to the numeric channel.
    pub numeric: u64,
    /// Nodes frozen into indeterminates (a budget or an overflow).
    pub frozen: u64,
}

impl SymCounts {
    /// The three decision counts added together.
    #[must_use]
    pub fn decisions(&self) -> u64 {
        self.symbolic_zero + self.sign_gated + self.numeric
    }

    /// Adds another session's counts into this one.
    pub fn absorb(&mut self, other: Self) {
        self.symbolic_zero += other.symbolic_zero;
        self.sign_gated += other.sign_gated;
        self.numeric += other.numeric;
        self.frozen += other.frozen;
    }
}

/// **The atom-algebra dials** — the three rewrite rules the normal form
/// applies to its opaque atoms, each switchable so that its effect on
/// a document is a measurement rather than an assumption, and so that
/// all three off is the plain quotient form bit for bit.
///
/// Every rule is an equality of reals under clause 1 of the theorem
/// ([`Decide::sign_within`]'s docs), so a zero reached through any of
/// them is still a zero of the real margin; what a rule can cost is
/// only a cancellation it fails to find. The rules are named A, B and C
/// where the tier's module docs discuss them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymRules {
    /// **A — `sqrt(X)² = X`.** An even power of a `sqrt` atom reduces
    /// to the power of its argument form. Pure algebra: sound for every
    /// real `X ≥ 0`, and `X ≥ 0` holds wherever the atom has a real
    /// value, which clause 1 guarantees before the identity test is
    /// ever asked.
    pub sqrt_square: bool,
    /// **B — `sin(θ)² + cos(θ)² = 1`** for atoms of ONE argument form.
    /// An even power of a `sin` atom rewrites to the same power of
    /// `1 − cos²` of the same argument, so any polynomial in the two
    /// that lies in the ideal of the Pythagorean identity reduces to
    /// zero. Unconditional.
    pub pythagoras: bool,
    /// **A0 — the exact constant fold**: `sqrt(c)` and `abs(c)` of a
    /// CONSTANT form whose value is a perfect-square rational (`sqrt`)
    /// or any rational (`abs`) fold to the exact rational, in the PLAIN
    /// form. No value is read — the argument is a literal of the form
    /// itself — and the fold is trivially sound (a constant atom
    /// replaced by the constant it denotes), so it cannot cost a
    /// cancellation: any zero the opaque form reaches, the folded form
    /// reaches. It is what relieves the arc family's freezes: the
    /// blocking residuals were products of `sqrt(1)^58` and `sqrt` of
    /// exact-square dyadic constants ([`Self::shipped`]).
    pub const_fold: bool,
    /// **The EARLY walk**: a SECOND memo built ALONGSIDE the plain
    /// form — never replacing it — in which rule C's fold runs at each
    /// `sqrt`/`abs` node (and, with [`Self::early_ab`], rules A/B run
    /// per node too). A decision is asked of the plain form first, so a
    /// plain theorem is never re-labelled; the early form can only ADD
    /// a discharge. Rule C rides this walk exclusively, because the
    /// atoms it folds sit nested inside other atoms' arguments, out of
    /// a top-residual reduction's reach. Its cost is a second walk of
    /// the DAG per decision the plain form did not answer, memoized per
    /// leaf.
    pub early: bool,
    /// **Rules A/B PER NODE in the early walk**, under a small step cap
    /// ([`EARLY_STEPS`]) with the un-reduced form kept where a
    /// reduction does not fit. Measured expensive rather than a
    /// runaway — and, on the BigInt ring, expensive enough not to ship:
    /// the plate's nominal replay went from 0.6 s to 138 s with it on,
    /// because with nothing freezing every node's reduction is real
    /// work ([`Self::shipped`]). Needs `early`.
    pub early_ab: bool,
    /// **C — `sqrt(X) = R` where `X = R²` as forms and `R` has a
    /// certified sign over the leaf's box** (and `abs(R) = ±R`
    /// likewise): clause 3 of the theorem, the one rule that reads a
    /// value. [`signed`] is the whole of how the value is read — the
    /// parameter brackets the analysis box already holds, enclosed in
    /// the ring — and why a zero reached through it is counted
    /// `sign_gated` rather than `symbolic_zero`. Needs `early`.
    pub signed_root: bool,
}

impl SymRules {
    /// Every rule on — the full set, for measuring what each can reach.
    #[must_use]
    pub const fn all() -> Self {
        Self {
            sqrt_square: true,
            pythagoras: true,
            const_fold: true,
            early: true,
            early_ab: true,
            signed_root: true,
        }
    }

    /// **The shipped set** — SHIPPED_DOCS_PLACEHOLDER
    #[must_use]
    pub const fn shipped() -> Self {
        Self {
            sqrt_square: false,
            pythagoras: false,
            const_fold: true,
            early: false,
            early_ab: false,
            signed_root: false,
        }
    }

    /// Every rule off: the quotient normal form with every atom opaque,
    /// which is the tier exactly as it stood before the atom algebra.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            sqrt_square: false,
            pythagoras: false,
            const_fold: false,
            early: false,
            early_ab: false,
            signed_root: false,
        }
    }
}

impl Default for SymRules {
    fn default() -> Self {
        Self::shipped()
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

/// A map keyed by an INDETERMINATE id (a 128-bit digest, so the same
/// verbatim hasher serves).
type IndetMap<V> = HashMap<u128, V, core::hash::BuildHasherDefault<IdHasher>>;

/// What one opaque atom is: its op and the forms of its arguments —
/// what rule A needs (`sqrt`'s argument), what rule B needs (a `sin`'s
/// argument digest names its `cos` twin), and what the shape report
/// renders.
struct AtomInfo {
    op: SymOp,
    /// R2's experiment: the node payload the atom's id was keyed with,
    /// without which a rule that rewrites an atom's ARGUMENT cannot
    /// re-mint the atom's id.
    payload: u64,
    args: [Option<Rc<Form>>; 2],
}

/// One leaf replay's DAG: the hash-consing table, the memoized forms and
/// the counts. Dropped with the leaf; nothing is shared across leaves.
struct Session {
    budget: SymBudget,
    rules: SymRules,
    nodes: IdMap<SymNode>,
    /// The PLAIN quotient forms — every atom opaque, rule A0 only (the
    /// constant fold, which cannot cost a cancellation). Rules A/B are
    /// applied afterwards over the top residual ([`algebra::reduce`])
    /// and per node in `forms_early`; nothing ruled is memoized here.
    forms: IdMap<Rc<Form>>,
    /// The EARLY-reduced forms (`SymRules::early`), a second memo
    /// beside the plain one.
    forms_early: IdMap<Rc<Form>>,
    /// The `f64` bracket of each document parameter this leaf was
    /// evaluated over, by the parameter's indeterminate id — recorded
    /// by [`Sym::param_over`], read only by rule C ([`signed`]).
    params: IndetMap<(f64, f64)>,
    /// Every opaque atom minted so far, by its indeterminate id.
    atoms: IndetMap<AtomInfo>,
    counts: SymCounts,
}

thread_local! {
    /// The installed session, if any (module docs: no session, no tier).
    static SESSION: RefCell<Option<Session>> = const { RefCell::new(None) };

    /// **How many opaque values this leaf replay has minted** — the
    /// payload [`Sym::opaque`] stamps into its node, so that each
    /// untracked real is its own indeterminate.
    ///
    /// D9, argued here rather than assumed. A sequence number is only
    /// deterministic if the order that advances it is; this one is
    /// advanced by the ORDER A LEAF MINTS ITS OPAQUE VALUES, which is
    /// the order the evaluation service walks that leaf's recipe — a
    /// fixed, single-threaded walk per leaf, the same one whose node
    /// ids D9 already rests on. [`with_session`] resets it around every
    /// replay, so a leaf's sequence starts at 0 no matter which leaf
    /// ran before it or on which rayon worker, and two replays of the
    /// same leaf mint the same ids. The counter never crosses a leaf
    /// boundary, which is the property that makes it safe: it is
    /// per-replay state living beside a per-replay table.
    static OPAQUE_SEQ: Cell<u64> = const { Cell::new(0) };
}

/// Restores [`OPAQUE_SEQ`] when a [`with_session`] call leaves, by any
/// path including a panic — the counter is per-replay state, so a
/// session that unwound without restoring it would hand the next leaf a
/// sequence starting mid-count and break the D9 claim on `opaque`'s ids.
struct OpaqueSeqGuard(u64);

impl Drop for OpaqueSeqGuard {
    fn drop(&mut self) {
        OPAQUE_SEQ.set(self.0);
    }
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
    with_session_rules(budget, SymRules::shipped(), f)
}

/// [`with_session`] with the atom-algebra dials chosen ([`SymRules`]);
/// `with_session` is this at [`SymRules::shipped`] — ONE default, so
/// every legacy caller runs the shipped tier and nothing else.
pub fn with_session_rules<R>(
    budget: SymBudget,
    rules: SymRules,
    f: impl FnOnce() -> R,
) -> (R, SymCounts) {
    let nested = SESSION.with(|s| s.borrow().is_some());
    // The opaque sequence is per-replay state, restored on the way out
    // so a nested or sequential call cannot inherit a partial count
    // (`OPAQUE_SEQ`'s docs carry the D9 argument).
    let _restore = OpaqueSeqGuard(OPAQUE_SEQ.replace(0));
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
            rules,
            nodes: IdMap::default(),
            forms: IdMap::default(),
            forms_early: IdMap::default(),
            params: IndetMap::default(),
            atoms: IndetMap::default(),
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

/// The indeterminate ONE OPAQUE VALUE enters the form as, keyed by its
/// per-replay sequence number — a different key per call, which is what
/// makes two untracked reals two unknowns rather than one.
fn indet_opaque(seq: u64) -> u128 {
    Hash128::new()
        .word(0x4f50_4151_5545_5f5f)
        .word(seq)
        .finish()
}

/// The indeterminate a parameter symbol enters the form as.
fn indet_param(symbol: u64) -> u128 {
    Hash128::new()
        .word(0x5041_5241_4d5f_494e)
        .word(symbol)
        .finish()
}

/// The indeterminate an OPAQUE atom enters the form as: its op tag, its
/// payload and the DIGESTS of its argument forms — so two atoms whose
/// arguments are the same rational function are one indeterminate.
fn indet_atom(tag: u64, payload: u64, args: &[u128]) -> u128 {
    let mut h = Hash128::new()
        .word(0x4154_4f4d_5f49_4e44)
        .word(tag)
        .word(payload);
    for d in args {
        h = h.wide(*d);
    }
    h.finish()
}

/// **The normal form: a quotient of two polynomials** over the parameter
/// symbols, π and the opaque atoms — with exact rational coefficients,
/// a denominator that is never the zero polynomial, and NO common-factor
/// cancellation.
///
/// # Why a quotient and not a polynomial
///
/// Division is not decoration in this kernel's identities. An extruded
/// strut's carrier is `origin + (w/‖w‖)·t` metered by `t ∈ [0, ‖w‖]`, so
/// the endpoint-pinning residual `carrier.eval(t₁) − end` is literally
/// `w·(‖w‖ · ‖w‖⁻¹ − 1)`: with the reciprocal held opaque the residual is
/// not the zero form, the tier discharges the rest of the identity
/// population and the macroscopic box still refuses. Measured on
/// `m10_3_driver_interval`'s slab at a ±0.05 band: 945 identities
/// discharged, `carrier_endpoint_end` still indeterminate at `[0, 0.21]`.
///
/// A quotient is not a simplification RULE bolted on — it is the normal
/// form of the field of fractions, reached by the same construction the
/// polynomial form is: `a/b + c/d = (ad + cb)/(bd)`, `(a/b)⁻¹ = b/a`.
/// Nothing is factored, `sqrt` and the transcendentals stay opaque
/// atoms, and no identity is asserted about them.
///
/// # Why the zero test stays a theorem
///
/// The form is zero **iff its NUMERATOR is the zero polynomial**. As a
/// rational function that is exactly zero; as a real number at the box's
/// actual parameter point it is `p(x)/q(x) = 0` PROVIDED `q(x) ≠ 0`, and
/// clause 1 of [`Decide::sign_within`]'s test already guarantees that: a
/// division by an enclosure containing zero is undefined there, so the
/// interval decoration drops to `Trv`, `Trv` propagates, and
/// `certified_bracket()` refuses the margin before the identity test is
/// ever asked. The clause was there for `sqrt(-1)`; it covers `1/0` by
/// the same sentence.
///
/// # What is deliberately absent
///
/// No common-factor cancellation, so `x²/x` and `x/1` are DIFFERENT
/// forms and an atom over one does not cancel against an atom over the
/// other. That is the conservative direction — a missed cancellation is
/// a numeric decision, which is what the kernel did before the tier
/// existed — and it keeps the form's cost linear in the expression
/// rather than in a polynomial GCD.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Form {
    num: Poly,
    den: Poly,
    /// **This form is not a rational function of the parameters** — it
    /// was built through a division by the ZERO polynomial, so it has
    /// no value anywhere and nothing derived from it may be declared
    /// identically zero.
    ///
    /// Why a poison and not a freeze (the reviewer's row
    /// `atan(1/(x-x)) - atan(1/(x-x))`). Freezing turns an
    /// unrepresentable subexpression into an INDETERMINATE keyed by its
    /// node id, and two occurrences of one node share that id — so the
    /// two `atan`s would be one unknown `u`, `u - u` would be the zero
    /// polynomial, and the tier would answer `Zero` for an expression
    /// with no real value. Freezing is the right answer for something
    /// the form cannot REPRESENT; it is the wrong answer for something
    /// that does not EXIST. Poison propagates through every combinator
    /// and makes [`Form::is_zero`] false, so the theorem's clause 2
    /// cannot be satisfied through it.
    ///
    /// This is the form-side half of clause 1. The value-side half
    /// (`MarginDiag::Invalid` from the numeric channel) catches a
    /// domain violation the SCALAR can see — an uncertified interval,
    /// a NaN. It cannot see this one: at `f64` the whole expression
    /// evaluates to a finite `0.0`, because `1/0` is `+inf`,
    /// `atan(+inf)` is `pi/2`, and the difference is an honest zero.
    /// The two halves catch different things and both are needed.
    poisoned: bool,
    /// **This form was built through a clause-3 fold** (rule C,
    /// [`signed`]): it is equal to the expression at every point of the
    /// leaf's box rather than identically in the parameters, so a zero
    /// reached through it is `sign_gated`, not `symbolic_zero`. Sticky
    /// through every combinator, like the poison flag.
    gated: bool,
}

impl Form {
    fn poly(num: Poly) -> Self {
        Self::quotient(num, Poly::one())
    }

    fn quotient(num: Poly, den: Poly) -> Self {
        Self {
            num,
            den,
            poisoned: false,
            gated: false,
        }
    }

    fn zero() -> Self {
        Self::poly(Poly::zero())
    }

    /// The form of an expression with no value: see [`Form::poisoned`].
    /// Its numerator is deliberately the ONE polynomial, so that a
    /// reader who ignores the flag still never reads it as a zero.
    fn poison() -> Self {
        Self {
            num: Poly::one(),
            den: Poly::one(),
            poisoned: true,
            gated: false,
        }
    }

    /// Whether either operand is poison — the propagation rule, in one
    /// place so no combinator can forget it.
    fn tainted(&self, other: &Self) -> bool {
        self.poisoned || other.poisoned
    }

    fn is_zero(&self) -> bool {
        !self.poisoned && self.num.is_zero()
    }

    fn add(&self, other: &Self, budget: SymBudget) -> Option<Self> {
        if self.tainted(other) {
            return Some(Self::poison());
        }
        // A shared denominator adds numerators, which is both cheaper
        // and tighter against the budget than cross-multiplying two
        // copies of the same polynomial.
        if self.den == other.den {
            return Some(Self {
                num: self.num.add(&other.num)?,
                den: self.den.clone(),
                poisoned: false,
                gated: self.gated || other.gated,
            });
        }
        Some(Self {
            num: self
                .num
                .mul(&other.den, budget)?
                .add(&other.num.mul(&self.den, budget)?)?,
            den: self.den.mul(&other.den, budget)?,
            poisoned: false,
            gated: self.gated || other.gated,
        })
    }

    fn neg(&self) -> Option<Self> {
        if self.poisoned {
            return Some(Self::poison());
        }
        Some(Self {
            num: self.num.neg()?,
            den: self.den.clone(),
            poisoned: false,
            gated: self.gated,
        })
    }

    fn mul(&self, other: &Self, budget: SymBudget) -> Option<Self> {
        if self.tainted(other) {
            return Some(Self::poison());
        }
        Some(Self {
            num: self.num.mul(&other.num, budget)?,
            den: self.den.mul(&other.den, budget)?,
            poisoned: false,
            gated: self.gated || other.gated,
        })
    }

    /// The reciprocal — POISON when the numerator is identically zero,
    /// which is not a rational function and has no value at any point.
    fn recip(&self) -> Option<Self> {
        if self.poisoned || self.num.is_zero() {
            return Some(Self::poison());
        }
        Some(Self {
            num: self.den.clone(),
            den: self.num.clone(),
            poisoned: false,
            gated: self.gated,
        })
    }

    /// The form's canonical digest — the key an opaque atom is minted
    /// under, so two atoms with equal-form arguments are one
    /// indeterminate. The poison flag is part of it, so an atom over a
    /// poisoned argument is never keyed as one over a clean argument;
    /// the atom itself is poisoned too, which is the load-bearing half.
    fn digest(&self) -> u128 {
        Hash128::new()
            .word(0x464f_524d_5f4e_4652)
            .word(u64::from(self.poisoned) | (u64::from(self.gated) << 1))
            .wide(self.num.digest())
            .wide(self.den.digest())
            .finish()
    }
}

/// The value an opaque UNARY atom takes at argument zero, where that
/// value is expressible in the form's own vocabulary — the fold that
/// lets `‖a − b‖` decide `Zero` when `a − b` does, which is the shape
/// most of the kernel's identity margins arrive in (`Margin::of` of a
/// distance is a `sqrt`).
fn unary_at_zero(op: SymOp) -> Option<Form> {
    match op {
        SymOp::Sqrt
        | SymOp::Abs
        | SymOp::Sin
        | SymOp::Tan
        | SymOp::Asin
        | SymOp::Atan
        | SymOp::Floor => Some(Form::zero()),
        SymOp::Cos => Some(Form::poly(Poly::one())),
        // acos 0 = π/2 — expressible, because π is an indeterminate of
        // the form rather than a number.
        SymOp::Acos => {
            let mut p = Poly::indet(INDET_PI);
            let half = Rat::new(1, 2, 0)?;
            for c in p.terms.values_mut() {
                *c = c.mul(&half)?;
            }
            Some(Form::poly(p))
        }
        // 1/0 is not a real; the numeric channel owns that refusal.
        _ => None,
    }
}

/// Whether a form is inside the session's freezing budget — both halves
/// of the quotient, because a denominator that grows without bound costs
/// exactly what a numerator does.
fn within(budget: SymBudget, f: &Form) -> bool {
    let ok = |p: &Poly| p.terms.len() <= budget.max_terms && p.degree() <= budget.max_degree;
    ok(&f.num) && ok(&f.den)
}

/// `base^n` for `n >= 0`, budget-checked at every step so a large
/// exponent freezes rather than allocating its way to the ceiling.
fn powi_form(base: &Form, n: u32, budget: SymBudget) -> Option<Form> {
    // `x^0` is 1 only where `x` has a value; a poisoned base has none,
    // so the exponent cannot rescue it.
    if base.poisoned {
        return Some(Form::poison());
    }
    let mut acc = Form::poly(Poly::one());
    for _ in 0..n {
        acc = acc.mul(base, budget)?;
        if !within(budget, &acc) {
            return None;
        }
    }
    Some(acc)
}

/// The PLAIN form of one node, given its children's forms — every atom
/// opaque, no rule applied — `None` for anything the caller must freeze
/// (an overflow, a budget, an unrepresentable literal, a reciprocal of
/// the zero form).
///
/// The atom algebra is NOT here: it runs later, once, over the top
/// residual ([`algebra::reduce`]), so it can never disturb a
/// cancellation the plain form already reaches. Every atom this mints
/// is recorded in the session ([`Session::atoms`]) so that reduction
/// can look its argument form back up.
fn combine(node: &SymNode, kids: [&Form; 2], sess: &mut Session, early: bool) -> Option<Form> {
    let (a, b) = (kids[0], kids[1]);
    let budget = sess.budget;
    // A0 applies in BOTH walks: it replaces a constant atom by the
    // constant it denotes, which cannot cost a cancellation
    // (`SymRules::const_fold`).
    let a0 = sess.rules.const_fold;
    // Rule C applies in the EARLY walk only (`SymRules::signed_root`).
    let c = early && sess.rules.signed_root;
    // An atom over a gated argument is gated: it stands for the value
    // of a form that is only box-wise equal to the expression.
    let gate = |mut f: Form| {
        f.gated |= a.gated;
        f
    };
    let atom1 = |op: SymOp, sess: &mut Session| {
        // A function OF an expression with no value has no value
        // either, and `a.is_zero()` is already false for a poisoned
        // argument, so the at-zero fold cannot fire on one.
        if a.poisoned {
            return Some(Form::poison());
        }
        if a.is_zero()
            && let Some(f) = unary_at_zero(op)
        {
            return Some(gate(f));
        }
        let id = indet_atom(op.tag(), node.payload, &[a.digest()]);
        sess.atoms.entry(id).or_insert_with(|| AtomInfo {
            op,
            payload: node.payload,
            args: [Some(Rc::new(a.clone())), None],
        });
        Some(gate(Form::poly(Poly::indet(id))))
    };
    match node.op {
        SymOp::Param => Some(Form::poly(Poly::indet(indet_param(node.payload)))),
        // One untracked real: its OWN indeterminate, keyed by the
        // sequence number the node carries (`SymOp::Opaque`'s docs).
        SymOp::Opaque => Some(Form::poly(Poly::indet(indet_opaque(node.payload)))),
        SymOp::Lit => {
            Rat::of_f64(f64::from_bits(node.payload)).map(|c| Form::poly(Poly::constant(c)))
        }
        SymOp::Pi => Some(Form::poly(Poly::indet(INDET_PI))),
        SymOp::Add => a.add(b, budget),
        SymOp::Sub => a.add(&b.neg()?, budget),
        SymOp::Mul => a.mul(b, budget),
        SymOp::Neg => a.neg(),
        SymOp::Inv => a.recip(),
        SymOp::Powi => {
            let n = node.payload as u32 as i32;
            match u32::try_from(n) {
                Ok(n) => powi_form(a, n, budget),
                Err(_) => powi_form(&a.recip()?, n.unsigned_abs(), budget),
            }
        }
        // A0: a sqrt/abs of a CONSTANT form folds exactly; then rule C
        // (early walk): a sqrt of a perfect square, or an abs, of a
        // form with a certified sign folds to the signed root.
        SymOp::Sqrt | SymOp::Abs if (a0 || c) && !a.poisoned => {
            let folded = (|| {
                if !a0 {
                    return None;
                }
                let n = a.num.as_constant()?;
                let d = a.den.as_constant()?;
                let c = n.mul(&d.recip()?)?;
                match node.op {
                    SymOp::Sqrt => c.sqrt_exact(),
                    _ => Some(c.abs()),
                }
            })();
            match folded {
                Some(k) => Some(gate(Form::poly(Poly::constant(k)))),
                None if c => match signed::fold(node.op, a, &sess.params, budget) {
                    Some(f) => Some(gate(f)),
                    None => atom1(node.op, sess),
                },
                None => atom1(node.op, sess),
            }
        }
        SymOp::Sqrt
        | SymOp::Abs
        | SymOp::Sin
        | SymOp::Cos
        | SymOp::Tan
        | SymOp::Asin
        | SymOp::Acos
        | SymOp::Atan
        | SymOp::Floor => atom1(node.op, sess),
        SymOp::Atan2 | SymOp::Min | SymOp::Max | SymOp::Copysign => {
            if a.tainted(b) {
                return Some(Form::poison());
            }
            // min(0, 0) and max(0, 0) are zero; a one-sided zero says
            // nothing, so only the both-zero fold is taken. copysign
            // carries `a`'s MAGNITUDE, so a zero first argument is zero
            // whatever the sign argument does (±0 is one real).
            // atan2(0, x) is 0 or π depending on the sign of x — not a
            // fold the form can take without reading a value.
            let folds = match node.op {
                SymOp::Min | SymOp::Max => a.is_zero() && b.is_zero(),
                SymOp::Copysign => a.is_zero(),
                _ => false,
            };
            if folds {
                let mut z = Form::zero();
                z.gated = a.gated || b.gated;
                return Some(z);
            }
            let id = indet_atom(node.op.tag(), node.payload, &[a.digest(), b.digest()]);
            sess.atoms.entry(id).or_insert_with(|| AtomInfo {
                op: node.op,
                payload: node.payload,
                args: [Some(Rc::new(a.clone())), Some(Rc::new(b.clone()))],
            });
            let mut f = Form::poly(Poly::indet(id));
            f.gated = a.gated || b.gated;
            Some(f)
        }
        // Keyed by the CHILD IDS, never by their forms (the op's docs).
        // A hull of something with no value has none either, so the
        // poison crosses this door like every other.
        SymOp::Hull if a.tainted(b) => Some(Form::poison()),
        SymOp::Hull => {
            let mut f = Form::poly(Poly::indet(
                Hash128::new()
                    .word(SymOp::Hull.tag())
                    .wide(node.kids[0].bits())
                    .wide(node.kids[1].bits())
                    .finish(),
            ));
            f.gated = a.gated || b.gated;
            Some(f)
        }
    }
}

/// The normal form of `root`, computed into `memo` inside `sess` — the
/// one walk both memos share, `early` choosing which.
///
/// **Two memos per session, and the distinction is the whole
/// architecture of the atom algebra.** The PLAIN walk (`early = false`,
/// every atom opaque, rule A0 only) builds the quotient normal form as
/// the tier stood before the algebra plus the constant fold; it is what
/// a decision is FIRST tested against, and a plain form that is zero is
/// an unconditional theorem. The EARLY walk (`early = true`,
/// `SymRules::early`) applies rules A/B per node under [`EARLY_STEPS`]
/// and rule C's fold at each `sqrt`/`abs` — ALONGSIDE the plain memo,
/// never replacing it, so a rule can only ADD a discharge and never
/// re-label one the plain form reached. That split is measured, not
/// assumed: the first cut of this unit let a ruled form REPLACE the
/// plain one and lost an `arc_span` cancellation and a straight edge's
/// endpoint theorem to it.
///
/// Iterative rather than recursive: an evaluation's DAG is as deep as
/// its expression tree, and a leaf replay's is thousands of nodes.
/// Termination is structural — a node's id is a hash of its children's
/// ids, so a cycle would need a hash preimage — and every popped id
/// leaves a form behind, so each is visited at most twice. Either walk
/// is the O(dag) construction; the early walk's per-node reduction is
/// bounded by its step cap, so its cost is a constant factor over the
/// plain walk, measured per document in `SymRules::shipped`'s docs.
fn form_in(sess: &mut Session, memo: &mut IdMap<Rc<Form>>, root: SymId, early: bool) -> Rc<Form> {
    let frozen = |sess: &mut Session, id: SymId| -> Rc<Form> {
        if !early {
            sess.counts.frozen += 1;
        }
        Rc::new(Form::poly(Poly::indet(id.bits())))
    };
    let mut stack = vec![(root, false)];
    while let Some((id, expanded)) = stack.pop() {
        if memo.contains_key(&id) {
            continue;
        }
        let Some(node) = sess.nodes.get(&id).copied() else {
            // Not in this session's table: an unrecorded leaf, or a node
            // minted before the session was installed. An unknown
            // function of the parameters is exactly an indeterminate.
            let f = frozen(sess, id);
            memo.insert(id, f);
            continue;
        };
        let arity = node.op.arity();
        if !expanded {
            let pending: Vec<SymId> = node.kids[..arity]
                .iter()
                .copied()
                .filter(|k| !memo.contains_key(k))
                .collect();
            if !pending.is_empty() {
                stack.push((id, true));
                stack.extend(pending.into_iter().map(|k| (k, false)));
                continue;
            }
        }
        let empty = Form::zero();
        let fa = if arity >= 1 {
            memo.get(&node.kids[0]).cloned()
        } else {
            None
        };
        let fb = if arity >= 2 {
            memo.get(&node.kids[1]).cloned()
        } else {
            None
        };
        let budget = sess.budget;
        let made = {
            let kids = [
                fa.as_deref().unwrap_or(&empty),
                fb.as_deref().unwrap_or(&empty),
            ];
            let combined = combine(&node, kids, sess, early);
            // The per-node A/B reduction (`SymRules::early_ab`),
            // bounded, falling back to the un-reduced form when it does
            // not fit.
            let combined = if early && sess.rules.early_ab {
                combined.map(|f| {
                    algebra::reduce_steps(&f, sess.rules, budget, &sess.atoms, EARLY_STEPS)
                        .filter(|g| within(budget, g))
                        .unwrap_or(f)
                })
            } else {
                combined
            };
            combined.filter(|f| within(budget, f))
        };
        drop((fa, fb));
        let f = match made {
            Some(p) => Rc::new(p),
            None => frozen(sess, id),
        };
        memo.insert(id, f);
    }
    memo.get(&root)
        .cloned()
        .unwrap_or_else(|| Rc::new(Form::poly(Poly::indet(root.bits()))))
}

/// The plain quotient form of `root` — every atom opaque, no rule
/// applied, no value read. Memoized in the session's persistent table.
fn plain_form(sess: &mut Session, root: SymId) -> Rc<Form> {
    let mut memo = core::mem::take(&mut sess.forms);
    let out = form_in(sess, &mut memo, root, false);
    sess.forms = memo;
    out
}

/// The most rule-A/B substitutions the early walk takes per node
/// before it gives the un-reduced form back — the bound that makes the
/// per-node reduction a fixed cost rather than a pass over the form.
const EARLY_STEPS: usize = 8;

/// The early-reduced form of `root` (`SymRules::early`), memoized in
/// its own table beside the plain one: the same walk as
/// [`plain_form`], with rules A/B applied per node under
/// [`EARLY_STEPS`] and rule C's fold at each `sqrt`/`abs`.
fn early_form(sess: &mut Session, root: SymId) -> Rc<Form> {
    let mut memo = core::mem::take(&mut sess.forms_early);
    let out = form_in(sess, &mut memo, root, true);
    sess.forms_early = memo;
    out
}

/// How the symbolic tier discharged a decision.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Discharge {
    /// An unconditional theorem: the form is the zero polynomial, no
    /// value read (`symbolic_zero`).
    Theorem,
    /// A theorem conditional on a clause-3 sign read over the leaf's
    /// box — the form is zero and was built through rule C's fold
    /// (`sign_gated`).
    SignGated,
}

/// **The identity test**: is this node's expression identically zero in
/// the parameters — as an unconditional theorem, or as one conditional
/// on a certified sign?
///
/// Three tiers, in the order that keeps a stronger claim from being
/// re-labelled as a weaker one: the PLAIN form (every atom opaque,
/// rules A0 only) is the zero polynomial — a theorem; the EARLY form
/// (rules A/B per node, rule C's fold) is zero — a theorem if no fold
/// took part, `SignGated` if one did; the top residual reduces to zero
/// under rules A/B — a theorem. Each tier is memoized per session, so
/// a decision pays the walk it needs once.
///
/// `None` outside a session, and at a zero-term budget — the tier
/// switched off inside the scalar.
fn discharge(id: SymId) -> Option<Discharge> {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut()?;
        if sess.budget.max_terms == 0 {
            return None;
        }
        let plain = plain_form(sess, id);
        if plain.is_zero() {
            return Some(Discharge::Theorem);
        }
        let rules = sess.rules;
        if rules.early {
            let e = early_form(sess, id);
            if e.is_zero() {
                return Some(if e.gated {
                    Discharge::SignGated
                } else {
                    Discharge::Theorem
                });
            }
        }
        if !(rules.sqrt_square || rules.pythagoras) {
            return None;
        }
        // Rules A and B (unconditional) over the residual, once.
        algebra::reduce(&plain, rules, sess.budget, &sess.atoms)
            .as_ref()
            .is_some_and(|f| f.is_zero())
            .then_some(Discharge::Theorem)
    })
}

/// Records how one decision was answered, for the session's receipt.
fn count_decision(discharge: Option<Discharge>) {
    SESSION.with(|s| {
        if let Some(sess) = s.borrow_mut().as_mut() {
            match discharge {
                Some(Discharge::Theorem) => sess.counts.symbolic_zero += 1,
                Some(Discharge::SignGated) => sess.counts.sign_gated += 1,
                None => sess.counts.numeric += 1,
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

    /// A value carrying NO tracked expression: **its own fresh
    /// indeterminate**, so its form is an unknown and every decision
    /// that depends on it is the numeric one.
    ///
    /// The door for a lane that legitimately has no expression to track
    /// — a bracket handed back by an engine that ran at another scalar
    /// — where fabricating a node would claim an algebraic relationship
    /// that was never computed.
    ///
    /// **Each call mints a DIFFERENT unknown, and that is the whole
    /// point.** Two untracked reals are two unknowns: `x` and `y` with
    /// nothing known about either. If they shared an id they would be
    /// one unknown, `x − x` would be the zero polynomial, and
    /// `opaque(1.0) − opaque(2.0)` would decide `Zero` — a theorem
    /// about two values that are not equal. They are also never keyed
    /// by the VALUE: two enclosures that happen to be bit-equal are
    /// still two separate reals, and keying by bits would say they are
    /// one.
    ///
    /// The sequence that separates them is [`OPAQUE_SEQ`], whose docs
    /// carry the D9 argument for why a counter here is still
    /// schedule-independent.
    #[must_use]
    pub fn opaque(value: T) -> Self {
        let seq = OPAQUE_SEQ.with(|c| {
            let n = c.get();
            c.set(n.wrapping_add(1));
            n
        });
        Self::nullary(value, SymOp::Opaque, seq)
    }

    /// A value bound as the document PARAMETER `symbol` — the one door
    /// that introduces an indeterminate. No bracket is recorded, so
    /// rule C ([`signed`]) can fold nothing over this parameter; a
    /// caller holding the box's bounds uses [`Self::param_over`].
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

    /// [`Self::param`] over the bracket `[lo, hi]` the value was built
    /// from — the analysis box's own two `f64`s, which the caller that
    /// mints a parameter axis already holds. The bracket is recorded in
    /// the installed session for rule C's sign read ([`signed`]); it is
    /// the ONLY value the symbolic tier ever reads, and it is read as
    /// two floats through a ring enclosure, never as the lane scalar.
    /// Outside a session the bracket is dropped and this is `param`.
    #[must_use]
    pub fn param_over(symbol: ParamSymbol, value: T, lo: f64, hi: f64) -> Self {
        SESSION.with(|s| {
            if let Some(sess) = s.borrow_mut().as_mut() {
                sess.params.insert(indet_param(symbol.0), (lo, hi));
            }
        });
        Self::param(symbol, value)
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
        (self.unary(s, SymOp::Sin, 0), self.unary(c, SymOp::Cos, 0))
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
/// 1. **the computation was defined on the whole input box**, checked
///    in TWO places because one scalar cannot see both halves of it.
///
///    *The value side.* [`MarginDiag::Invalid`] is the arm every scalar
///    returns for a domain violation it can see —
///    [`crate::Interval::sign_within`] for an uncertified enclosure,
///    `f64` and [`crate::Probe`] for NaN — so the numeric channel
///    already answers that question and this impl needs no bracket door
///    of its own. Without it, `sqrt(-1) − sqrt(-1)` decides `Zero` on an
///    expression with no real value.
///
///    *The form side*, and it is not decoration:
///    **[`Form::poisoned`]**. At `f64` and `Probe` the only thing
///    `Invalid` catches is NaN, and a violation can hide behind a
///    perfectly finite value — `atan(1/(x−x)) − atan(1/(x−x))` is
///    `atan(+inf) − atan(+inf)` = `π/2 − π/2` = a finite `0.0`, with no
///    real behind it anywhere. So a form built through a division by the
///    ZERO polynomial is poisoned, the poison propagates through every
///    combinator, and a poisoned form is never zero. That is the half
///    the value channel structurally cannot supply at a point scalar.
///
///    (At `Interval` the same expression violates the domain visibly —
///    the division is empty and the decoration drops — so the value side
///    catches it there. Both halves are present at every base scalar
///    because neither one is sufficient at all of them.)
/// 2. **the node's normal form is the zero polynomial**, computed with
///    exact rational coefficients from the parameter symbols down.
///    Nothing in that computation reads a value.
///
/// Together they say: the margin is a real number, and that real number
/// is zero at every parameter point of the box. `Sign::Zero` is then a
/// theorem, not a measurement — which is why no band is consulted and
/// why the answer does not depend on the box's width.
///
/// **The numeric channel runs FIRST, always**, which costs one
/// `sign_within` on the symbolic path and buys two things: clause 1 above
/// and an honest K sample. At `Probe` the base scalar records the margin
/// it classified before this impl overrides the answer, so the funnel's
/// sample carries a real number and is merely RE-TAGGED
/// ([`crate::k_stats`]'s `retag_at`) rather than replaced
/// by one with no margin in it — at the index taken BEFORE the base
/// scalar ran, so the row re-tagged is this decision's own.
///
/// **And a definite non-zero numeric answer short-circuits the form.**
/// A certified enclosure that excludes zero is a proof that the margin
/// is not zero, so no normal form over the parameters can be the zero
/// polynomial — asking for one is work whose answer is already known.
/// Building it was a measurable share of the tier's cost (a reviewer
/// clocked one leaf replay at 57 ms against 1.4 ms numeric), and the
/// forms skipped here are exactly the expensive ones: the margins that
/// are NOT identities, which is most of them. A debug assertion keeps
/// the shortcut honest — if a form ever IS zero under a definite
/// numeric sign, the two channels contradict each other and that is a
/// soundness bug in one of them, not a fast path to take quietly.
///
/// Everything else is `T::sign_within` verbatim.
impl<T: Decide> Decide for Sym<T> {
    fn sign_within(self, band: Band) -> Result<Sign, Indeterminate> {
        // Where this decision's own K sample will land, read before the
        // base scalar records it (`k_stats::sink_mark`).
        #[cfg(feature = "probe")]
        let mark = crate::k_stats::sink_mark();
        let numeric = self.value.sign_within(band);
        let domain_violation =
            matches!(&numeric, Err(e) if matches!(e.margin, MarginDiag::Invalid));
        let definitely_nonzero = matches!(&numeric, Ok(Sign::Positive | Sign::Negative));
        if definitely_nonzero {
            debug_assert!(
                discharge(self.node).is_none(),
                "the numeric channel proved this margin nonzero and the form says it is                  identically zero: the two channels contradict each other"
            );
            count_decision(None);
            report::record(&numeric, None, None);
            return numeric;
        }
        let symbolic = if domain_violation {
            None
        } else {
            discharge(self.node)
        };
        count_decision(symbolic);
        if let Some(how) = symbolic {
            #[cfg(feature = "probe")]
            crate::k_stats::retag_at(
                mark,
                match how {
                    Discharge::Theorem => crate::k_stats::SampleOutcome::SymbolicZero,
                    Discharge::SignGated => crate::k_stats::SampleOutcome::SignGated,
                },
            );
            report::record(&numeric, Some(how), None);
            return Ok(Sign::Zero);
        }
        // The shape report wants the residual that BLOCKED — rendered
        // only when the instrument is installed, so an ordinary replay
        // never pays for it.
        if report::active() {
            let text = report::render_node(self.node);
            report::record(&numeric, None, text);
        }
        numeric
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::predicate::Margin;
    use crate::tolerance::Tol;

    /// **The SHIPPED budget**, so these rows exercise the dials a drive
    /// actually runs at. They used to use `max_degree: 16` while the
    /// shipped value was 128 — a unit test that never touched the
    /// configuration it was defending.
    fn budget() -> SymBudget {
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
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

    /// The documented limits, pinned as limits: no factoring past the
    /// quotient, and no trigonometric identity beyond rule B. `sin(2θ)
    /// − 2·sinθ·cosθ` is not the zero form — `sin(2θ)` is an atom of
    /// another argument and nothing relates it to the pair — and with
    /// the rules OFF the Pythagorean pair is not either, which is what
    /// makes `SymRules::none` the pre-algebra tier.
    #[test]
    fn the_opaque_atoms_are_opaque() {
        let (_, counts) = with_session(budget(), || {
            let x = p("w", 3.0);
            let (s, c) = x.sin_cos();
            let (s2, _) = (x + x).sin_cos();
            decides_zero(s2 - Sym::from_f64(2.0) * s * c)
        });
        // Numerically zero at this point — the numeric channel answers
        // it, as it always did. What is pinned is that the TIER claims
        // nothing: the double-angle identity is outside every rule.
        assert_eq!(counts.symbolic_zero, 0, "no double-angle identity");
        assert_eq!(counts.sign_gated, 0);
        assert_eq!(counts.numeric, 1);
        let (_, counts) = with_session_rules(budget(), SymRules::none(), || {
            let x = p("w", 3.0);
            let (s, c) = x.sin_cos();
            decides_zero(s * s + c * c - Sym::from_f64(1.0))
        });
        assert_eq!(counts.symbolic_zero, 0, "rule B off: both atoms opaque");
        assert_eq!(counts.numeric, 1);
    }

    /// **Rule B**: the Pythagorean pair of ONE argument form is the zero
    /// form, whatever the argument; of two different arguments it is
    /// not.
    #[test]
    fn the_pythagorean_pair_of_one_argument_is_a_theorem() {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let x = p("w", 3.0);
            let y = p("h", 0.25);
            let (s, c) = (x * y + Sym::from_f64(2.0)).sin_cos();
            let same = decides_zero(s * s + c * c - Sym::from_f64(1.0));
            let (s2, _) = y.sin_cos();
            let mixed = decides_zero(s2 * s2 + c * c - Sym::from_f64(1.0));
            (same, mixed)
        });
        assert!(out.0, "sin²θ + cos²θ − 1 is the zero form");
        assert_eq!(counts.symbolic_zero, 1, "{counts:?}");
        assert_eq!(counts.sign_gated, 0);
        // The mixed pair decides NUMERICALLY (it is not a theorem, and
        // at this point it is not zero either).
        assert_eq!(counts.numeric, 1, "{counts:?}");
    }

    /// **Rule A**: an even power of a `sqrt` atom is its argument, so
    /// `sqrt(X)·sqrt(X) − X` and `sqrt(X)³ − X·sqrt(X)` are theorems.
    #[test]
    fn a_square_root_squared_is_its_argument() {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let (x, y) = (p("w", 3.0), p("h", 0.25));
            let arg = x * x + y * y + Sym::from_f64(1.0);
            let s = arg.sqrt();
            let a = decides_zero(s * s - arg);
            let b = decides_zero(s * s * s - arg * s);
            let c = decides_zero(s.powi(2) - arg);
            (a, b, c)
        });
        assert_eq!(out, (true, true, true));
        assert_eq!(counts.symbolic_zero, 3, "{counts:?}");
        assert_eq!(counts.sign_gated, 0, "rule A reads no value");
    }

    /// A parameter with its bracket recorded, at `f64` — the door rule C
    /// reads through ([`Sym::param_over`]).
    fn p_over(name: &str, v: f64, lo: f64, hi: f64) -> Sym<f64> {
        Sym::param_over(ParamSymbol::of(name), v, lo, hi)
    }

    /// **Rule C, clause 3: `sqrt(r²) − r` is a theorem CONDITIONAL on
    /// `r`'s sign**, and is counted as one. With `r`'s bracket strictly
    /// positive the fold takes `sqrt(r²) → r` and the decision is
    /// `sign_gated` — never `symbolic_zero`, because it holds on the box
    /// and not identically. `abs(r) − r` folds the same way. The
    /// residual sits one power below rule A (`sqrt` to the FIRST
    /// power), which is why an unconditional rule cannot reach it.
    #[test]
    fn rule_c_discharges_a_signed_root_as_sign_gated() {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = p_over("r", 1.25e-3, 1.0e-3, 2.0e-3);
            let sq = decides_zero((r * r).sqrt() - r);
            let abs = decides_zero(r.abs() - r);
            (sq, abs)
        });
        assert_eq!(out, (true, true));
        assert_eq!(
            counts.sign_gated, 2,
            "both are clause-3 theorems: {counts:?}"
        );
        assert_eq!(
            counts.symbolic_zero, 0,
            "and neither is an unconditional one"
        );
        assert_eq!(counts.numeric, 0);
    }

    /// **Rule C's negative sign**: `sqrt(r²) + r` folds when `r` is
    /// DEFINITELY negative (`sqrt(r²) = −r` there), and not otherwise.
    #[test]
    fn rule_c_folds_the_negated_root_under_a_negative_sign() {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = p_over("r", -1.25e-3, -2.0e-3, -1.0e-3);
            (
                decides_zero((r * r).sqrt() + r),
                decides_zero(r.abs() + r),
                // The same residual with the WRONG sign is not zero, and
                // the fold does not make it one: it decides numerically.
                decides_zero((r * r).sqrt() - r),
            )
        });
        assert_eq!(out, (true, true, false));
        assert_eq!(counts.sign_gated, 2, "{counts:?}");
        assert_eq!(counts.numeric, 1);
    }

    /// **Rule C's refusals**: a bracket that STRADDLES zero never folds
    /// (the sign is not certified), a parameter with no bracket
    /// recorded never folds, and with the rule off the atom stays
    /// opaque — in every case the decision is the numeric channel's
    /// own and `sign_gated` stays zero.
    #[test]
    fn rule_c_never_folds_without_a_certified_sign() {
        // Straddling.
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = p_over("r", 1.25e-3, -1.0e-3, 2.0e-3);
            (decides_zero((r * r).sqrt() - r), decides_zero(r.abs() - r))
        });
        assert_eq!(out, (true, true), "numerically zero at the point");
        assert_eq!(
            counts.sign_gated, 0,
            "a straddling bracket folds nothing: {counts:?}"
        );
        assert_eq!(counts.numeric, 2);
        // A zero endpoint is not strictly signed.
        let (_, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = p_over("r", 1.25e-3, 0.0, 2.0e-3);
            decides_zero((r * r).sqrt() - r)
        });
        assert_eq!(counts.sign_gated, 0, "{counts:?}");
        // No bracket at all (`Sym::param`).
        let (_, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = p("r", 1.25e-3);
            decides_zero((r * r).sqrt() - r)
        });
        assert_eq!(counts.sign_gated, 0, "no bracket, no read: {counts:?}");
        assert_eq!(counts.numeric, 1);
        // The rule off (the shipped set is measured, not assumed:
        // `SymRules::shipped`'s docs).
        let (_, counts) = with_session_rules(
            budget(),
            SymRules {
                signed_root: false,
                ..SymRules::all()
            },
            || {
                let r = p_over("r", 1.25e-3, 1.0e-3, 2.0e-3);
                decides_zero((r * r).sqrt() - r)
            },
        );
        assert_eq!(counts.sign_gated, 0, "{counts:?}");
        assert_eq!(counts.numeric, 1);
    }

    /// **A plain theorem is never re-labelled by rule C.** The early
    /// walk runs ALONGSIDE the plain form, and a decision the plain form
    /// answers is `symbolic_zero` even when a gated fold would also have
    /// reached it.
    #[test]
    fn a_plain_theorem_stays_unconditional_beside_rule_c() {
        let (_, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = p_over("r", 1.25e-3, 1.0e-3, 2.0e-3);
            // `sqrt(r²)·sqrt(r²) − r²`: rule A reaches it in the early
            // walk too, and the plain form does not — but with r's sign
            // certified the early walk's FIRST fold is C's, so this is
            // gated; the plain-zero row below is the one that must not
            // be.
            decides_zero((r * r).sqrt() - (r * r).sqrt());
        });
        assert_eq!(
            counts.symbolic_zero, 1,
            "x − x is the zero form: {counts:?}"
        );
        assert_eq!(counts.sign_gated, 0);
    }

    /// **The candidate shape the plate's ceiling has**: `sqrt(X) − R`
    /// with `X = R²` as forms where `X` is NOT a syntactic square —
    /// `(a + 2r)²` expanded to `a² + 4ar + 4r²` under the root — folds
    /// under rule C when `a + 2r` has a certified sign. This is
    /// `‖q − c‖ = r` with the endpoint at `c + r·(1, 0)` scaled by 2.
    #[test]
    fn rule_c_recovers_the_root_of_an_expanded_square() {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let a = p_over("a", 0.5, 0.25, 0.75);
            let r = p_over("r", 1.25e-3, 1.0e-3, 2.0e-3);
            let x = a * a + Sym::from_f64(4.0) * a * r + Sym::from_f64(4.0) * r * r;
            decides_zero(x.sqrt() - (a + Sym::from_f64(2.0) * r))
        });
        assert!(out);
        assert_eq!(counts.sign_gated, 1, "{counts:?}");
        assert_eq!(counts.symbolic_zero, 0);
    }

    /// **Division is IN the normal form** (the quotient of polynomials,
    /// not an opaque reciprocal): `(x/y)·y − x` is the zero form.
    ///
    /// This is the shape the kernel's own endpoint-pinning identity
    /// arrives in — an extruded strut's carrier is metered in metres and
    /// its direction normalized, so the residual is
    /// `w·(‖w‖ · ‖w‖⁻¹ − 1)` — and holding the reciprocal opaque leaves
    /// exactly that identity undischarged.
    #[test]
    fn a_reciprocal_cancels_because_the_form_is_a_quotient() {
        let (_, counts) = with_session(budget(), || {
            let x = p("w", 3.0);
            let y = p("h", 2.0);
            decides_zero((x / y) * y - x);
            // And the shape the kernel actually builds: a direction
            // normalized by a norm, re-metered by the same norm.
            let w = [p("a", 0.0), p("b", 0.0), p("d", 1.0)];
            let n = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
            let residual = w[2] / n * n - w[2];
            decides_zero(residual)
        });
        assert_eq!(counts.symbolic_zero, 2, "{counts:?}");
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

    /// **The coefficient ring's bound is a freeze, not a panic**: an
    /// alignment or a product that would need more than `COEFF_BITS`
    /// bits answers `None` — exactly what an `i128` overflow answered —
    /// and everything under the bound is exact.
    #[test]
    fn an_alignment_past_the_coefficient_bound_freezes() {
        let big = Rat::new(1, 1, 5000).unwrap();
        let small = Rat::new(1, 1, -5000).unwrap();
        assert!(
            big.add(&small).is_none(),
            "a 10 000-bit shift is past the bound"
        );
        assert!(big.add(&big).is_some(), "aligned already: exact");
        // A product whose odd part crosses the bound is refused too, and
        // one just under it is exact.
        let m = Rat::of_f64(0.1).unwrap();
        let mut acc = Rat::one();
        let mut steps = 0;
        while let Some(next) = acc.mul(&m) {
            acc = next;
            steps += 1;
            assert!(steps < 100, "0.1^k must cross the bound before k = 100");
        }
        assert!(
            steps >= 4,
            "0.1 carries 53 odd bits, so 4 factors fit: {steps}"
        );
    }

    /// The rational is exact on every `f64` it accepts, and refuses the
    /// ones that are not real numbers.
    #[test]
    fn the_rational_embeds_floats_exactly() {
        for x in [0.0, 1.0, -0.5, 0.1, 3.1e-3, f64::MIN_POSITIVE] {
            let r = Rat::of_f64(x).expect("a finite float is a dyadic rational");
            // num/den · 2^exp2 back to a float, when the parts are small
            // enough for the round trip to be exact.
            if r.num.bits() <= 53 && r.den.is_one() {
                let back = r.num.to_f64().unwrap() * 2f64.powi(r.exp2);
                assert_eq!(back.to_bits(), x.to_bits(), "round trip of {x}");
            }
        }
        assert!(Rat::of_f64(f64::NAN).is_none());
        assert!(Rat::of_f64(f64::INFINITY).is_none());
    }
}
