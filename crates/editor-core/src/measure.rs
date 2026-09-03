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
//! geometry. That closure is not reopened here. [`MeasureExpr`] is the
//! strictly larger language that sits ON TOP: the same arithmetic over
//! two leaf kinds — an ordinary [`Expr`], and a [`MeasurePrimitive`]
//! naming a closed-form measurement of entities the NODE references.
//!
//! **"Total" is the word to be careful with, and this header used to
//! use it wrongly.** The AST is total in the sense F7 means: no
//! conditionals, no iteration, no user-defined functions, so every
//! tree terminates. It is NOT total as a function into the finite
//! reals — `Div` is partial, `13 / 0` is `inf`, and that is true of
//! `Expr` too. `Expr` handles it at a door rather than in the type:
//! [`crate::expr::eval`] refuses a non-finite FINAL value. This
//! language shares that exact door
//! ([`crate::expr::refuse_non_finite`], called by
//! [`crate::eval::measure::eval_measure`]) rather than restating the
//! arithmetic and forgetting it — which is what the first draft did,
//! and it shipped an assertion reporting `Holds { measured: inf }`.
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
    /// **The minimum clearance between two selections** →
    /// [`Dimension::Length`] (E3's last v1 primitive, E7's engine).
    ///
    /// The two indices name references exactly as every other
    /// primitive does, and each one's ENTITY KIND is the selection's
    /// face scope: a reference to a BODY selects all of that body's
    /// faces, a reference to a FACE selects that one. Those are the two
    /// scopes M10-5's [`crate::clearance::FaceScope`] carries; a
    /// several-named-faces scope has no spelling here, because a
    /// primitive's arity is fixed at two references and the general
    /// selection vocabulary is the clearance door's own. A reference to
    /// anything else (a vertex, an edge, a datum) refuses typed.
    ///
    /// **Its value is an enclosure, so it exists only where enclosures
    /// do.** At `f64`, `Probe` and `Dual<f64>` the measure has no
    /// value at all and says so
    /// ([`crate::eval::ValuePayload::MeasureUnavailable`]): a station
    /// pair found by a point-scalar search is an upper bound on the
    /// minimum and not the minimum, and reporting one as the measured
    /// value is the degradation E7 forbids by name. At `Interval` over
    /// a leaf the value IS
    /// [`crate::clearance::min_separation`]'s bracket.
    MinClearance {
        /// Index into the node's `refs` of the first selection.
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
            Self::Distance { .. } | Self::Gap { .. } | Self::MinClearance { .. } => {
                Dimension::Length
            }
            Self::Angle { .. } => Dimension::Angle,
        }
    }

    /// The reference indices this primitive reads, in argument order —
    /// the domain every bounds check runs over, so no door has its own
    /// copy of each primitive's arity.
    pub fn refs(self) -> [u32; 2] {
        match self {
            Self::Distance { a, b } | Self::Angle { a, b } | Self::MinClearance { a, b } => [a, b],
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
            Self::MinClearance { .. } => "min_clearance",
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

    /// **Which endpoints of this tree's enclosure are certified for
    /// the subject** ([`Certified`], M10-6/R1).
    ///
    /// Structural, not numeric: it reads the tree's SHAPE, so it is
    /// the same answer at every scalar and over every box, and an
    /// assertion's admissible arms cannot depend on the numbers it is
    /// about to compare.
    pub fn certified(&self) -> Certified {
        if matches!(
            self.kind,
            MeasureKind::Primitive(MeasurePrimitive::MinClearance { .. })
        ) {
            return Certified::LowerBoundOnly;
        }
        let mut prims = Vec::new();
        self.primitives(&mut prims);
        if prims
            .iter()
            .any(|p| matches!(p, MeasurePrimitive::MinClearance { .. }))
        {
            Certified::Neither
        } else {
            Certified::Enclosure
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

/// **Why a measure has no value at the scalar the build ran at**
/// ([`crate::eval::ValuePayload::MeasureUnavailable`]).
///
/// One arm today, and the type exists so the second one — whenever a
/// primitive arrives whose answer some other lane cannot carry — lands
/// as a variant rather than as a second mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasureUnavailableAt {
    /// The primitive's answer is an ENCLOSURE, and this build's scalar
    /// is a point: it has nowhere to put one.
    ///
    /// Not a degradation and not a fallback — the two are the same
    /// thing said twice, which is why this carries the DOOR that can
    /// answer instead of a number: a reader is told where the answer
    /// lives, not handed a worse one.
    NeedsEnclosure {
        /// Which primitive.
        verb: &'static str,
        /// The scalar this build ran at, in its own name
        /// ([`MinClearanceLane::LANE`]).
        scalar: &'static str,
        /// The door that answers it, named so the recourse is in the
        /// refusal rather than in a reader's memory.
        door: &'static str,
    },
}

impl MeasureUnavailableAt {
    /// The primitive whose answer is unavailable — the one word a
    /// goldening form needs, without spelling the whole prose.
    pub fn verb(&self) -> &'static str {
        match self {
            Self::NeedsEnclosure { verb, .. } => verb,
        }
    }
}

impl core::fmt::Display for MeasureUnavailableAt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NeedsEnclosure { verb, scalar, door } => write!(
                f,
                "`{verb}` answers with a certified enclosure, and a {scalar} build has no \
                 channel for one — a point-scalar search finds a pair, and a pair that was \
                 found is an upper bound on the minimum rather than the minimum. Evaluate \
                 the document at the interval scalar over a parameter box, where `{door}` \
                 computes the bracket"
            ),
        }
    }
}

/// **A scalar that can carry a `min_clearance` answer** — the third
/// lane seam, beside [`crate::analysis::AxisScalar`] (the box axis in)
/// and [`crate::analysis::SeedScalar`] (the derivative seed in).
///
/// [`Self::min_separation`] answers the bracket as a value of `Self`,
/// or `None` when this scalar has no such value. The trio share one
/// shape on purpose: a per-scalar CAPABILITY, expressed at compile time
/// behind a scalar-free door, whose `None` is a typed refusal at the
/// call site rather than a quietly degraded answer.
///
/// The engine that computes it is the interval lane's
/// ([`crate::clearance::min_separation`]) and so is the only `Some`.
pub trait MinClearanceLane: geom_core::Real {
    /// This lane's own name, for the refusal that names it.
    const LANE: &'static str;

    /// The minimum separation between two resolved selections, or
    /// `None` when this scalar cannot carry an enclosure.
    ///
    /// # Errors
    ///
    /// The engine's own typed refusal, carried by class name and
    /// payload ([`MinClearanceRefusal`]) rather than by its own type,
    /// which lives behind the `interval` feature this door does not.
    fn min_separation(
        a: &MinClearanceOperand<'_, Self>,
        b: &MinClearanceOperand<'_, Self>,
    ) -> Option<Result<Self, MinClearanceRefusal>>;
}

/// One side of a [`MeasurePrimitive::MinClearance`], resolved: the body
/// the reference landed in, where it was read, and the faces its entity
/// kind selects.
///
/// The resolution is the evaluator's — it already walked the N5 ladder
/// to read the carrier of every other primitive's reference — so this
/// carries the ANSWER of that walk and no naming machinery.
pub struct MinClearanceOperand<'b, T: geom_core::Real> {
    /// The node the body was read at.
    pub at: crate::node::RecipeNodeId,
    /// Which of that node's output bodies.
    pub index: u32,
    /// The body itself, at this lane's scalar.
    pub body: &'b topo::Body<T>,
    /// The faces in scope: every face of the body for a body-kind
    /// reference, the one face for a face-kind reference.
    pub faces: Vec<topo::entity::FaceKey>,
}

/// The clearance engine's refusal, carried across the feature boundary
/// by class name and payload.
///
/// The engine's own `ClearanceRefusal` lives behind the `interval`
/// feature and this door does not, so the two halves it renders — the
/// stable class name and the evidence — travel instead of the enum.
/// They are the same two halves the goldening form prints, through the
/// engine's own `name()` and `payload()`, so a refusal reads the same
/// here as it does in a clearance report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinClearanceRefusal {
    /// The refusal's stable class name.
    pub class: &'static str,
    /// Its evidence, rendered.
    pub payload: String,
}

impl core::fmt::Display for MinClearanceRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "the clearance engine refused `{}`", self.class)?;
        if !self.payload.is_empty() {
            write!(f, " ({})", self.payload)?;
        }
        Ok(())
    }
}

impl core::error::Error for MinClearanceRefusal {}

/// A point scalar has no enclosure to answer with. The whole content of
/// the trait, at the lane where it bites.
impl MinClearanceLane for f64 {
    const LANE: &'static str = "f64";

    fn min_separation(
        _a: &MinClearanceOperand<'_, Self>,
        _b: &MinClearanceOperand<'_, Self>,
    ) -> Option<Result<Self, MinClearanceRefusal>> {
        None
    }
}

/// The recording scalar is `f64` with a sink attached, so it carries
/// exactly what `f64` carries — here, nothing.
#[cfg(feature = "probe")]
impl MinClearanceLane for geom_core::Probe {
    const LANE: &'static str = "Probe";

    fn min_separation(
        _a: &MinClearanceOperand<'_, Self>,
        _b: &MinClearanceOperand<'_, Self>,
    ) -> Option<Result<Self, MinClearanceRefusal>> {
        None
    }
}

/// **A dual does not certify** (the D1 ruling, unmoved). Its value
/// channel is whatever it is built over, and a `Dual<Interval>`'s
/// enclosure would be a certified answer arriving through a type E9
/// and D1 keep out of the certifying lanes — so the whole family
/// answers `None`, and a document measured for sensitivities reports
/// the same typed absence a plain f64 build does.
impl<T: geom_core::Real> MinClearanceLane for geom_core::Dual<T>
where
    geom_core::Dual<T>: geom_core::Real,
{
    const LANE: &'static str = "Dual";

    fn min_separation(
        _a: &MinClearanceOperand<'_, Self>,
        _b: &MinClearanceOperand<'_, Self>,
    ) -> Option<Result<Self, MinClearanceRefusal>> {
        None
    }
}

/// The interval lane, and the only one that answers: the engine's own
/// bracket, at the shipped dials.
#[cfg(feature = "interval")]
impl MinClearanceLane for geom_core::Interval {
    const LANE: &'static str = "Interval";

    fn min_separation(
        a: &MinClearanceOperand<'_, Self>,
        b: &MinClearanceOperand<'_, Self>,
    ) -> Option<Result<Self, MinClearanceRefusal>> {
        fn side<'b>(
            o: &MinClearanceOperand<'b, geom_core::Interval>,
        ) -> crate::clearance::MinSepSelection<'b> {
            crate::clearance::MinSepSelection {
                at: o.at,
                index: o.index,
                body: o.body,
                faces: o.faces.clone(),
            }
        }
        Some(
            crate::clearance::min_separation(
                &side(a),
                &side(b),
                crate::clearance::MinSeparationConfig::default(),
            )
            .map(|m| m.enclosure())
            .map_err(|r| MinClearanceRefusal {
                class: r.name(),
                payload: r.payload(),
            }),
        )
    }
}

/// **The symbolic tier has no clearance lane, and the absence is a
/// DISCLOSED limitation rather than a design position** (E12's unit,
/// deviation D3; issue 1276).
///
/// Every other lane the tier composes with is scalar-generic and runs
/// at `Sym<T>` unaltered. This one is not: [`crate::clearance`]'s engine
/// is written at [`geom_core::Interval`] concretely — its selection type
/// borrows a `&Body<Interval>` and its inner subdivision is spelled in
/// that type — so the door cannot be handed a `Body<Sym<Interval>>`, and
/// stripping one would need a scalar remap of a whole body, which no
/// door in `topo` offers today.
///
/// `None` is therefore the honest answer, and it behaves exactly as the
/// point scalars' `None` does: `min_clearance` refuses TYPED at this
/// lane, naming it, instead of reporting a number it did not compute. A
/// document carrying that measure drives with the symbolic tier off
/// (`DriveConfig::symbolic.enabled = false`) and says so; nothing
/// silently degrades.
impl<T: MinClearanceLane> MinClearanceLane for geom_core::Sym<T>
where
    geom_core::Sym<T>: geom_core::Real,
{
    const LANE: &'static str = "Sym";

    fn min_separation(
        _a: &MinClearanceOperand<'_, Self>,
        _b: &MinClearanceOperand<'_, Self>,
    ) -> Option<Result<Self, MinClearanceRefusal>> {
        None
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
/// The upstream-FAILURE lanes are NOT here: a failed or poisoned
/// measure poisons its assertion through the ordinary DAG edge (F2),
/// so the assertion has no value at all rather than an
/// `Unevaluated` one. What is left is the two cases where the measure
/// node itself came out fine and the COMPARISON still has no answer:
/// the margin was undecidable, or there was no measured value to
/// compare — a typed absence
/// ([`crate::eval::ValuePayload::MeasureUnavailable`]), which is a
/// value and not a failure, and therefore reaches here rather than
/// poisoning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnevaluatedReason {
    /// The margin between measured and bound landed in the sliver
    /// band: the run's tolerance cannot separate them, and guessing a
    /// side would manufacture the certainty the band exists to deny.
    Indeterminate,
    /// The measure has no value at this build's scalar, and says why.
    /// E10's third state used for exactly what it is for: the
    /// requirement is recorded, the run cannot answer it, and neither
    /// half of that is hidden.
    MeasureUnavailable(MeasureUnavailableAt),
    /// **The verdict would have been read off an endpoint this run
    /// certifies for the CARRIER rather than for the thing the measure
    /// names** (M10-6; R1's MAJOR). See [`Certified`] for the table of
    /// which arm reads which endpoint and why only two of the four are
    /// sound.
    WindowSuperset {
        /// The primitive that brought the superset in.
        verb: &'static str,
        /// The endpoint the arm would have read.
        endpoint: &'static str,
        /// The tracker item whose fix retires this refusal.
        recourse: &'static str,
    },
}

/// **The tracker item that retires [`UnevaluatedReason::WindowSuperset`]**:
/// tightening a carrier window to its trimmed face needs the trim
/// boundary in chart coordinates. Named from the type so the recourse
/// travels with the refusal instead of living in a reader's memory.
pub const WINDOW_TIGHTENING: &str = "work/m10/clearance-window-tightening-needs-chart-boundary.md";

/// **How much of a measured enclosure is certified for the thing the
/// measure NAMES**, as against the carrier the engine subdivided.
///
/// Every measure but `min_clearance` answers about its own subject, so
/// both endpoints are the subject's and an assertion may read either.
/// `min_clearance` does not: M10-5's engine subdivides carrier
/// WINDOWS, a disclosed SUPERSET of the trimmed faces. Writing `m` for
/// the window separation and `M` for the faces', `m ≤ M` pointwise —
/// so a lower bound on `m` is a lower bound on `M`, while an attained
/// window distance is NOT an upper bound on `M`. On an L-shaped cap
/// the engine finds a window pair straight across the notch that
/// neither face occupies, and reports it.
///
/// The endpoints are therefore not interchangeable, and which VERDICT
/// an assertion may reach depends on which endpoint its arm reads:
///
/// | direction | verdict | endpoint | sound for the faces? |
/// | --- | --- | --- | --- |
/// | `AtLeast c` | `Holds` | `lo` | yes — `M ≥ m ≥ lo ≥ c` |
/// | `AtLeast c` | `Violated` | `hi` | **no** |
/// | `AtMost c` | `Violated` | `lo` | yes — `M ≥ m ≥ lo > c` |
/// | `AtMost c` | `Holds` | `hi` | **no** |
///
/// The two unsound arms refuse [`UnevaluatedReason::WindowSuperset`]
/// rather than answering; [`WINDOW_TIGHTENING`] retires the refusal.
/// Both gating directions survive: a clearance requirement (`AtLeast`)
/// still certifies, and a maximum-gap requirement (`AtMost`) still
/// fails loudly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Certified {
    /// Both endpoints are the subject's — every tree that reads no
    /// `min_clearance`.
    Enclosure,
    /// The LOWER endpoint only: the tree is exactly one
    /// `min_clearance` primitive, so `lo` bounds the faces from below
    /// and `hi` is the carrier's alone.
    LowerBoundOnly,
    /// NEITHER endpoint, because a `min_clearance` sits under
    /// arithmetic that can carry it to either end (a `Neg`, a `Sub`
    /// with it on the right, a `Min`/`Max` against something else).
    /// Refusing the whole assertion is the reading that needs no
    /// per-operator argument; no document this unit ships takes it.
    Neither,
}

impl Certified {
    /// Whether an arm reading `endpoint` may answer.
    fn admits(self, upper: bool) -> bool {
        match self {
            Self::Enclosure => true,
            Self::LowerBoundOnly => !upper,
            Self::Neither => false,
        }
    }
}

impl core::fmt::Display for UnevaluatedReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Indeterminate => f.write_str(
                "the measured value and the bound are not separated at this run's tolerance, \
                 so the assertion has no verdict — tighten the tolerance or move the bound",
            ),
            Self::MeasureUnavailable(why) => write!(f, "there is no measured value: {why}"),
            Self::WindowSuperset {
                verb,
                endpoint,
                recourse,
            } => write!(
                f,
                "this verdict would be read off the enclosure's {endpoint} endpoint, which \
                 `{verb}` certifies only over the carrier WINDOWS — a superset of the trimmed \
                 faces the measure names — so a window pair neither face occupies could decide \
                 it. The opposite verdict on this same bound is still available and still \
                 gates. Recourse: {recourse}"
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
/// # Which arms may answer (M10-6/R1)
///
/// `certified` says which endpoints of `measured` belong to the thing
/// the measure NAMES; [`Certified`] carries the table and the
/// argument. The funnel runs either way — the decision is the same one
/// at the same site, and demoting it after the fact rather than
/// branching before it keeps `assert_bound`'s k-population complete,
/// so the E6 telemetry still sees every comparison the document asked
/// for. What changes is only whether the verdict it reached is one
/// this run may report.
pub(crate) fn decide_assertion<T: Decide>(
    measured: T,
    bound: T,
    dir: AssertionDir,
    band: geom_core::Band,
    certified: Certified,
) -> AssertionVerdict<T> {
    let comparand = match dir {
        AssertionDir::AtLeast => measured - bound,
        AssertionDir::AtMost => bound - measured,
    };
    // Which END of the enclosure each arm reads. `AtLeast` decides
    // `Holds` off the smallest the measure can be and `Violated` off
    // the largest; `AtMost` is the mirror.
    let refuse = |upper: bool| AssertionVerdict::Unevaluated {
        reason: UnevaluatedReason::WindowSuperset {
            verb: MeasurePrimitive::MinClearance { a: 0, b: 0 }.verb(),
            endpoint: if upper { "upper" } else { "lower" },
            recourse: WINDOW_TIGHTENING,
        },
    };
    match geom_core::k_stats::decide_flagged(ASSERT_BOUND, comparand, band, "F16") {
        // At the bound exactly, a non-strict relation holds.
        Ok(geom_core::Sign::Positive | geom_core::Sign::Zero) => {
            let upper = matches!(dir, AssertionDir::AtMost);
            if certified.admits(upper) {
                AssertionVerdict::Holds { measured, bound }
            } else {
                refuse(upper)
            }
        }
        Ok(geom_core::Sign::Negative) => {
            let upper = matches!(dir, AssertionDir::AtLeast);
            if certified.admits(upper) {
                AssertionVerdict::Violated { measured, bound }
            } else {
                refuse(upper)
            }
        }
        Err(_) => AssertionVerdict::Unevaluated {
            reason: UnevaluatedReason::Indeterminate,
        },
    }
}

/// The funnel site name of the assertion comparison. A roster carrier
/// (`docs/K-REPORT.md`) rather than a literal at the decide site.
pub const ASSERT_BOUND: &str = "assert_bound";
