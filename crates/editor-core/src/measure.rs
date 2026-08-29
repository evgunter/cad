//! The measurement sublanguage (ERROR-DESIGN E3, CONTACT-DESIGN C5).
//!
//! A [`Measure`](crate::Node::Measure) node is ONE dimension-generic
//! sink: it denotes no body and evaluates to a typed F1 quantity. The
//! quantity KIND rides the measured expression through the existing
//! [`Dimension`] lattice — there are no per-kind measure node variants,
//! and this module grows the lattice by nothing.
//!
//! # Two layers, deliberately
//!
//! [`Expr`] is a CLOSED world: literals, document parameters and
//! arithmetic, and nothing that reaches out of the document into
//! geometry. That closure is what makes it total, and it is not
//! reopened here. [`MeasureExpr`] is the strictly larger language that
//! sits ON TOP: the same arithmetic over two leaf kinds — an ordinary
//! [`Expr`], and a [`MeasurePrimitive`] naming a closed-form
//! measurement of entities the NODE references.
//!
//! The primitives address their operands by INDEX into the node's
//! `refs: Vec<StableName>`, never by name. Entity references therefore
//! live on the node (the `Node::Fillet` selection precedent — frozen at
//! authoring time, rebindable through the one `Rebind` edit), and the
//! expression stays a pure value: it can be compared, hashed and
//! serialized without ever touching the naming layer.
//!
//! # One lattice, asked rather than restated
//!
//! Dimension checking runs at CONSTRUCTION, exactly as `Expr`'s does,
//! and it runs the SAME rules: [`lattice`] builds probe expressions at
//! the operand dimensions and asks `Expr`'s own smart constructors what
//! comes out. A rule change in `expr.rs` therefore reaches this
//! language automatically; a second copy of the F1 table would drift.

use geom_core::Decide;

use crate::expr::{Dimension, DimensionError, Expr};

/// Which closed-form measurement a leaf computes, and over which of
/// the node's references.
///
/// The v1 carrier scope is stated on each door in
/// [`mod@crate::eval::measure`], where the closed forms live: this
/// enum names WHAT is measured, and a pair of carriers the closed form
/// has no arm for is a typed evaluation refusal
/// ([`MeasureUnsupported`](crate::eval::measure::MeasureUnsupported)), never a
/// guess.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum MeasurePrimitive {
    /// The distance between two referenced entities → [`Dimension::Length`].
    Distance {
        /// Index into the node's `refs` of the first entity.
        a: u32,
        /// Index into the node's `refs` of the second.
        b: u32,
    },
    /// The angle between two referenced entities → [`Dimension::Angle`].
    Angle {
        /// Index into the node's `refs` of the first entity.
        a: u32,
        /// Index into the node's `refs` of the second.
        b: u32,
    },
    /// C5's SIGNED gap between a mating pair → [`Dimension::Length`].
    ///
    /// **Argument order is the mating ROLE**, not a symmetry: `outer`
    /// is the containing carrier (the socket, the bore, the plane the
    /// offset is measured FROM) and `inner` is the contained one (the
    /// ball, the pin). C5's formulas are asymmetric in exactly that
    /// way — `g = R − r − ‖Δc‖` is not `r − R − ‖Δc‖` — so the roles
    /// are authored rather than inferred from which radius is larger,
    /// which would be a decided comparison masquerading as a
    /// definition.
    Gap {
        /// Index into the node's `refs` of the containing carrier.
        outer: u32,
        /// Index into the node's `refs` of the contained carrier.
        inner: u32,
    },
}

impl MeasurePrimitive {
    /// The F1 dimension this primitive yields. Fixed per primitive —
    /// the whole point of E3's "the quantity kind rides the
    /// expression".
    pub fn dim(self) -> Dimension {
        match self {
            Self::Distance { .. } | Self::Gap { .. } => Dimension::Length,
            Self::Angle { .. } => Dimension::Angle,
        }
    }

    /// The reference indices this primitive reads, in argument order —
    /// the domain every bounds check runs over, so no door has its own
    /// copy of each primitive's arity.
    pub fn refs(self) -> [u32; 2] {
        match self {
            Self::Distance { a, b } | Self::Angle { a, b } => [a, b],
            Self::Gap { outer, inner } => [outer, inner],
        }
    }

    /// The primitive's name, for diagnostics and the wire-free
    /// content-key tag's human twin.
    pub fn verb(self) -> &'static str {
        match self {
            Self::Distance { .. } => "distance",
            Self::Angle { .. } => "angle",
            Self::Gap { .. } => "gap",
        }
    }
}

/// A dimension-checked measurement expression: `Expr`'s arithmetic over
/// a primitive leaf.
///
/// Private fields and fallible constructors, exactly as [`Expr`]: an
/// ill-dimensioned tree is unrepresentable, so the cached
/// [`Self::dim`] is trustworthy by construction.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureExpr {
    dim: Dimension,
    kind: MeasureKind,
}

/// The measurement AST. Crate-private so trees are only built through
/// the dimension-checking constructors.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MeasureKind {
    /// A closed-form measurement of the node's referenced entities.
    Primitive(MeasurePrimitive),
    /// An ordinary document expression — literals, parameters, and the
    /// whole `Expr` arithmetic beneath them.
    Value(Expr),
    /// Same-dimension addition.
    Add(Box<MeasureExpr>, Box<MeasureExpr>),
    /// Same-dimension subtraction.
    Sub(Box<MeasureExpr>, Box<MeasureExpr>),
    /// Negation (any dimension).
    Neg(Box<MeasureExpr>),
    /// Product; the F1 rule (≥1 `Scalar` operand).
    Mul(Box<MeasureExpr>, Box<MeasureExpr>),
    /// Quotient; the divisor must be `Scalar`.
    Div(Box<MeasureExpr>, Box<MeasureExpr>),
    /// Same-dimension lattice minimum.
    Min(Box<MeasureExpr>, Box<MeasureExpr>),
    /// Same-dimension lattice maximum.
    Max(Box<MeasureExpr>, Box<MeasureExpr>),
}

/// The binary operations this language shares with [`Expr`]. Naming
/// them lets [`lattice`] ask `Expr`'s own constructor for the result
/// dimension instead of restating the F1 table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
}

/// **The F1 lattice, asked rather than restated.**
///
/// Builds one probe expression per operand dimension and runs the real
/// `Expr` constructor over them: whatever dimension comes out is this
/// language's, and whatever [`DimensionError`] comes out is this
/// language's refusal, in the same words a document expression would
/// have earned. Probe construction is total for every `Dimension` —
/// `Expr::literal` refuses only `Count` (which takes `Expr::count`)
/// and non-finite values (1.0 is finite) — so the impossible branch is
/// announced as the kernel bug it would be rather than carried as a
/// refusal a caller could believe in.
fn lattice(op: Binop, left: Dimension, right: Dimension) -> Result<Dimension, DimensionError> {
    fn probe(dim: Dimension) -> Expr {
        if dim == Dimension::Count {
            return Expr::count(1);
        }
        match Expr::literal(1.0, dim) {
            Ok(e) => e,
            Err(refusal) => unreachable!(
                "a unit literal at {dim:?} is constructible — `Expr::literal` refuses only \
                 Count (taken above) and non-finite values — yet it refused: {refusal}"
            ),
        }
    }
    let (a, b) = (probe(left), probe(right));
    match op {
        Binop::Add => Expr::add(a, b),
        Binop::Sub => Expr::sub(a, b),
        Binop::Mul => Expr::mul(a, b),
        Binop::Div => Expr::div(a, b),
        Binop::Min => Expr::min(a, b),
        Binop::Max => Expr::max(a, b),
    }
    .map(|e| e.dim())
}

// The arithmetic constructors share names with the std ops traits for
// the reason `Expr`'s do (they ARE this language's add/sub/...), and
// they cannot implement them for the same reason: they are FALLIBLE
// (the F1 checker runs at construction) and associated functions, not
// methods.
#[allow(clippy::should_implement_trait)]
impl MeasureExpr {
    /// This expression's dimension (cached; correct by construction).
    pub fn dim(&self) -> Dimension {
        self.dim
    }

    /// The AST node (the persistence and key layers read it).
    pub(crate) fn kind(&self) -> &MeasureKind {
        &self.kind
    }

    /// A closed-form measurement leaf.
    pub fn primitive(p: MeasurePrimitive) -> Self {
        Self {
            dim: p.dim(),
            kind: MeasureKind::Primitive(p),
        }
    }

    /// An ordinary document expression as a leaf — a literal bound, a
    /// parameter, a whole arithmetic subtree of them.
    pub fn value(e: Expr) -> Self {
        Self {
            dim: e.dim(),
            kind: MeasureKind::Value(e),
        }
    }

    fn binary(
        op: Binop,
        a: MeasureExpr,
        b: MeasureExpr,
        make: fn(Box<MeasureExpr>, Box<MeasureExpr>) -> MeasureKind,
    ) -> Result<Self, DimensionError> {
        let dim = lattice(op, a.dim, b.dim)?;
        Ok(Self {
            dim,
            kind: make(Box::new(a), Box::new(b)),
        })
    }

    /// Same-dimension addition.
    pub fn add(a: MeasureExpr, b: MeasureExpr) -> Result<Self, DimensionError> {
        Self::binary(Binop::Add, a, b, MeasureKind::Add)
    }

    /// Same-dimension subtraction.
    pub fn sub(a: MeasureExpr, b: MeasureExpr) -> Result<Self, DimensionError> {
        Self::binary(Binop::Sub, a, b, MeasureKind::Sub)
    }

    /// Negation — any dimension.
    pub fn neg(a: MeasureExpr) -> Self {
        Self {
            dim: a.dim,
            kind: MeasureKind::Neg(Box::new(a)),
        }
    }

    /// Product; at least one operand `Scalar` (F1).
    pub fn mul(a: MeasureExpr, b: MeasureExpr) -> Result<Self, DimensionError> {
        Self::binary(Binop::Mul, a, b, MeasureKind::Mul)
    }

    /// Quotient; the divisor must be `Scalar` (F1).
    pub fn div(a: MeasureExpr, b: MeasureExpr) -> Result<Self, DimensionError> {
        Self::binary(Binop::Div, a, b, MeasureKind::Div)
    }

    /// Same-dimension lattice minimum.
    pub fn min(a: MeasureExpr, b: MeasureExpr) -> Result<Self, DimensionError> {
        Self::binary(Binop::Min, a, b, MeasureKind::Min)
    }

    /// Same-dimension lattice maximum.
    pub fn max(a: MeasureExpr, b: MeasureExpr) -> Result<Self, DimensionError> {
        Self::binary(Binop::Max, a, b, MeasureKind::Max)
    }

    /// Every primitive in the tree, in pre-order — the domain of the
    /// node door's bounds check and of the wire door's re-check, so
    /// the two cannot disagree about which indices a tree reads.
    pub fn primitives(&self, out: &mut Vec<MeasurePrimitive>) {
        match &self.kind {
            MeasureKind::Primitive(p) => out.push(*p),
            MeasureKind::Value(_) => {}
            MeasureKind::Neg(a) => a.primitives(out),
            MeasureKind::Add(a, b)
            | MeasureKind::Sub(a, b)
            | MeasureKind::Mul(a, b)
            | MeasureKind::Div(a, b)
            | MeasureKind::Min(a, b)
            | MeasureKind::Max(a, b) => {
                a.primitives(out);
                b.primitives(out);
            }
        }
    }

    /// Every VALUE leaf, in pre-order — the deterministic order the
    /// evaluator resolves them in and the order the evaluation walk
    /// consumes them in.
    ///
    /// One order, two consumers, exactly as `Node::slots()` is one
    /// order for the content key and the op wiring: the leaves are
    /// evaluated once, their bits feed the key, and the same vector
    /// feeds the arithmetic. Two walks would be two chances to
    /// disagree about which leaf is which.
    pub fn value_leaves<'e>(&'e self, out: &mut Vec<&'e Expr>) {
        match &self.kind {
            MeasureKind::Primitive(_) => {}
            MeasureKind::Value(e) => out.push(e),
            MeasureKind::Neg(a) => a.value_leaves(out),
            MeasureKind::Add(a, b)
            | MeasureKind::Sub(a, b)
            | MeasureKind::Mul(a, b)
            | MeasureKind::Div(a, b)
            | MeasureKind::Min(a, b)
            | MeasureKind::Max(a, b) => {
                a.value_leaves(out);
                b.value_leaves(out);
            }
        }
    }

    /// The document parameters this expression references, with their
    /// recorded dimensions — the `Expr::param_refs` contract lifted to
    /// this language, so `apply`'s re-check reaches measure nodes too.
    pub fn param_refs(&self, out: &mut Vec<(crate::doc::ParamName, Dimension)>) {
        match &self.kind {
            MeasureKind::Primitive(_) => {}
            MeasureKind::Value(e) => e.param_refs(out),
            MeasureKind::Neg(a) => a.param_refs(out),
            MeasureKind::Add(a, b)
            | MeasureKind::Sub(a, b)
            | MeasureKind::Mul(a, b)
            | MeasureKind::Div(a, b)
            | MeasureKind::Min(a, b)
            | MeasureKind::Max(a, b) => {
                a.param_refs(out);
                b.param_refs(out);
            }
        }
    }

    /// Every embedded value leaf's float literal BITS, pre-order — the
    /// bit-semantic comparison substrate (D7), delegating each leaf to
    /// [`Expr::literal_bits`] rather than re-walking `Expr`.
    pub fn literal_bits(&self, out: &mut Vec<u64>) {
        match &self.kind {
            MeasureKind::Primitive(_) => {}
            MeasureKind::Value(e) => e.literal_bits(out),
            MeasureKind::Neg(a) => a.literal_bits(out),
            MeasureKind::Add(a, b)
            | MeasureKind::Sub(a, b)
            | MeasureKind::Mul(a, b)
            | MeasureKind::Div(a, b)
            | MeasureKind::Min(a, b)
            | MeasureKind::Max(a, b) => {
                a.literal_bits(out);
                b.literal_bits(out);
            }
        }
    }

    /// Bit-semantic equality (D7): structural equality with float
    /// literals compared by BITS, exactly as [`Expr::bit_eq`].
    pub fn bit_eq(&self, other: &MeasureExpr) -> bool {
        if self != other {
            return false;
        }
        let (mut a, mut b) = (Vec::new(), Vec::new());
        self.literal_bits(&mut a);
        other.literal_bits(&mut b);
        a == b
    }
}

/// Which way an [`Assertion`](crate::Node::Assertion) constrains its
/// measure (E10).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum AssertionDir {
    /// The measured quantity must be at least the bound.
    AtLeast,
    /// The measured quantity must be at most the bound.
    AtMost,
}

impl AssertionDir {
    /// The relation as it reads in a report.
    pub fn symbol(self) -> &'static str {
        match self {
            Self::AtLeast => ">=",
            Self::AtMost => "<=",
        }
    }
}

/// **An assertion's evaluated verdict — REPORT ONLY (E10 v1).**
///
/// This is the whole of an assertion's product. Nothing downstream
/// reads it: no gate consults it, no op takes it as an operand, and no
/// product or export changes shape because of it. That is structural
/// rather than promised — an assertion node denotes no body, so the
/// root gather skips it exactly as it skips a declaration, and its
/// value payload is not an admissible operand for any op in the
/// vocabulary. A `Violated` verdict is therefore a fact a REPORT reads,
/// and a gating mode is additive policy nobody has ratified.
#[derive(Debug, Clone, PartialEq)]
pub enum AssertionVerdict<T> {
    /// The measured value satisfies the bound.
    Holds {
        /// What the measure evaluated to.
        measured: T,
        /// What the bound evaluated to.
        bound: T,
    },
    /// The measured value violates the bound — BOTH numbers, because a
    /// verdict without them cannot be acted on.
    Violated {
        /// What the measure evaluated to.
        measured: T,
        /// What the bound evaluated to.
        bound: T,
    },
    /// No verdict is available: the comparison itself could not be
    /// decided at the run's tolerance, so the assertion says so rather
    /// than picking a side.
    Unevaluated {
        /// Why, in the reporting layer's own words.
        reason: UnevaluatedReason,
    },
}

/// Why an assertion produced no verdict.
///
/// The upstream-failure lanes are NOT here: a failed or poisoned
/// measure poisons its assertion through the ordinary DAG edge (F2),
/// so the assertion has no value at all rather than an
/// `Unevaluated` one. What is left is the case where both operands
/// evaluated and the COMPARISON is the thing that could not be
/// decided.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnevaluatedReason {
    /// The margin between measured and bound landed in the sliver
    /// band: the run's tolerance cannot separate them, and guessing a
    /// side would manufacture the certainty the band exists to deny.
    Indeterminate,
}

impl core::fmt::Display for UnevaluatedReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Indeterminate => f.write_str(
                "the measured value and the bound are not separated at this run's tolerance, \
                 so the assertion has no verdict — tighten the tolerance or move the bound",
            ),
        }
    }
}

impl<T> AssertionVerdict<T> {
    /// Does the assertion hold? `None` when there is no verdict — the
    /// three states stay three at every reader, so nothing collapses
    /// `Unevaluated` into a silent pass.
    pub fn holds(&self) -> Option<bool> {
        match self {
            Self::Holds { .. } => Some(true),
            Self::Violated { .. } => Some(false),
            Self::Unevaluated { .. } => None,
        }
    }

    /// The verdict's state as a word, for reports.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Holds { .. } => "Holds",
            Self::Violated { .. } => "Violated",
            Self::Unevaluated { .. } => "Unevaluated",
        }
    }
}

/// **The signed comparison an assertion makes**, decided through the
/// one `k_stats` funnel under the existing `assert_bound` predicate
/// name.
///
/// The comparand is `measured − bound` for `AtLeast` and its negation
/// for `AtMost`, in the MEASURE's own dimension.
///
/// # Dimension (audit F16, `docs/predicate-dimension-audit.md`)
///
/// Length measures make that comparand honest metres against the
/// linear band; Angle measures make it RADIANS against the same band,
/// which is the audit's dimensionless-comparand shape. E3 forecloses
/// the obvious repair — a lever arm would need a chosen length scale,
/// which it rejects by name — so the site is FLAGGED, not cast, and
/// the row argues it.
///
/// An `Indeterminate` escalation becomes
/// [`AssertionVerdict::Unevaluated`] rather than a node failure: a
/// bound the run cannot separate from the measurement is a fact about
/// the report, not a broken document.
pub(crate) fn decide_assertion<T: Decide>(
    measured: T,
    bound: T,
    dir: AssertionDir,
    band: geom_core::Band,
) -> AssertionVerdict<T> {
    let comparand = match dir {
        AssertionDir::AtLeast => measured - bound,
        AssertionDir::AtMost => bound - measured,
    };
    match geom_core::k_stats::decide_flagged(ASSERT_BOUND, comparand, band, "F16") {
        // At the bound exactly, a non-strict relation holds.
        Ok(geom_core::Sign::Positive | geom_core::Sign::Zero) => {
            AssertionVerdict::Holds { measured, bound }
        }
        Ok(geom_core::Sign::Negative) => AssertionVerdict::Violated { measured, bound },
        Err(_) => AssertionVerdict::Unevaluated {
            reason: UnevaluatedReason::Indeterminate,
        },
    }
}

/// The funnel site name of the assertion comparison. A roster carrier
/// (`docs/K-REPORT.md`) rather than a literal at the decide site.
pub const ASSERT_BOUND: &str = "assert_bound";
