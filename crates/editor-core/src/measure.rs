//! The measurement vocabulary (ERROR-DESIGN E3, CONTACT-DESIGN C5).
//!
//! A [`Measure`](crate::Node::Measure) node is an operation holding ONE
//! [`MeasurePrimitive`]: a closed-form measurement of two sited
//! references. It defines one scalar output of the primitive's
//! dimension, and that output is **observed** (D10): a function of the
//! built geometry rather than of what was written, read only by an
//! [`Assertion`](crate::Node::Assertion), directly or through a
//! definition.
//!
//! Arithmetic over measured values is an ordinary `Defined` variable
//! over the measures' outputs. [`MeasureExpr`] is the AUTHORED shape of
//! such a measurement — `Expr`'s arithmetic over primitive leaves and
//! value leaves — and nothing stores it: [`crate::measure`] lowers it to
//! one measure per primitive and a [`crate::Formula`] over their
//! outputs, which the reading slot lowers like any other formula.
//!
//! # One lattice, asked rather than restated
//!
//! Dimension checking runs at CONSTRUCTION, exactly as `Expr`'s does,
//! and it runs the SAME rules: [`lattice`] builds probe expressions at
//! the operand dimensions and asks `Expr`'s own smart constructors what
//! comes out. A rule change in `expr.rs` therefore reaches this
//! language automatically; a second copy of the F1 table would drift.

use geom_brep::recourse::{Reading, RefusedArm, SizedDecision, StoredDefinite};
use geom_core::{Decide, SizedPass};

use crate::expr::{Dimension, DimensionError, Expr, MAX_NESTING, Slot};
use crate::node::SitedRef;

/// Which closed-form measurement a measure computes, over which two
/// references `R`.
///
/// A [`crate::Node::Measure`] holds one over [`SitedRef`]s. The authored
/// [`MeasureExpr`] holds them over `u32` indices into the reference
/// list its builder is handed ([`crate::measure`]).
///
/// The v1 carrier scope is stated on each door in
/// [`mod@crate::eval::measure`], where the closed forms live: this
/// enum names WHAT is measured, and a pair of carriers the closed form
/// has no arm for is a typed evaluation refusal
/// ([`MeasureUnsupported`](crate::eval::measure::MeasureUnsupported)), never a
/// guess.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum MeasurePrimitive<R = SitedRef> {
    /// The distance between two referenced entities → [`Dimension::Length`].
    Distance {
        /// The first entity.
        a: R,
        /// The second.
        b: R,
    },
    /// The angle between two referenced entities → [`Dimension::Angle`].
    Angle {
        /// The first entity.
        a: R,
        /// The second.
        b: R,
    },
    /// **The minimum clearance between two selections** →
    /// [`Dimension::Length`] (E3's last v1 primitive, E7's engine).
    ///
    /// Each reference's ENTITY KIND is the selection's face scope: a
    /// reference to a BODY selects all of that body's faces, a
    /// reference to a FACE selects that one. Those are the two scopes
    /// M10-5's `clearance::FaceScope` carries. A reference to anything
    /// else (a vertex, an edge, a datum) refuses typed.
    ///
    /// **Its value is an enclosure, so it exists only where enclosures
    /// do.** At `f64`, `Probe` and `Dual<f64>` the measure has no
    /// value at all and says so
    /// ([`crate::eval::ValuePayload::MeasureUnavailable`]): a station
    /// pair found by a point-scalar search is an upper bound on the
    /// minimum and not the minimum. At `Interval` over a leaf the value
    /// IS `clearance::min_separation`'s bracket.
    MinClearance {
        /// The first selection.
        a: R,
        /// The second.
        b: R,
    },
    /// C5's SIGNED gap between a mating pair → [`Dimension::Length`].
    ///
    /// **Argument order is the mating ROLE**, not a symmetry: `outer`
    /// is the containing carrier (the socket, the bore, the plane the
    /// offset is measured FROM) and `inner` is the contained one (the
    /// ball, the pin). C5's formulas are asymmetric in exactly that
    /// way — `g = R − r − ‖Δc‖` is not `r − R − ‖Δc‖` — so the roles
    /// are authored rather than inferred from which radius is larger.
    Gap {
        /// The containing carrier.
        outer: R,
        /// The contained carrier.
        inner: R,
    },
}

impl<R> MeasurePrimitive<R> {
    /// The F1 dimension this primitive yields, fixed per primitive.
    pub fn dim(&self) -> Dimension {
        match self {
            Self::Distance { .. } | Self::Gap { .. } | Self::MinClearance { .. } => {
                Dimension::Length
            }
            Self::Angle { .. } => Dimension::Angle,
        }
    }

    /// The two references, in argument order.
    pub fn refs(&self) -> [&R; 2] {
        match self {
            Self::Distance { a, b } | Self::Angle { a, b } | Self::MinClearance { a, b } => [a, b],
            Self::Gap { outer, inner } => [outer, inner],
        }
    }

    /// The two references, in argument order, exclusive.
    pub fn refs_mut(&mut self) -> [&mut R; 2] {
        match self {
            Self::Distance { a, b } | Self::Angle { a, b } | Self::MinClearance { a, b } => [a, b],
            Self::Gap { outer, inner } => [outer, inner],
        }
    }

    /// The same primitive over other references, `f` applied to each
    /// in argument order; the first refusal is the answer.
    ///
    /// # Errors
    ///
    /// `f`'s first.
    pub fn try_map<R2, E>(
        &self,
        mut f: impl FnMut(&R) -> Result<R2, E>,
    ) -> Result<MeasurePrimitive<R2>, E> {
        Ok(match self {
            Self::Distance { a, b } => MeasurePrimitive::Distance { a: f(a)?, b: f(b)? },
            Self::Angle { a, b } => MeasurePrimitive::Angle { a: f(a)?, b: f(b)? },
            Self::MinClearance { a, b } => MeasurePrimitive::MinClearance { a: f(a)?, b: f(b)? },
            Self::Gap { outer, inner } => MeasurePrimitive::Gap {
                outer: f(outer)?,
                inner: f(inner)?,
            },
        })
    }

    /// The primitive's name, for diagnostics and the wire.
    pub fn verb(&self) -> &'static str {
        match self {
            Self::Distance { .. } => "distance",
            Self::Angle { .. } => "angle",
            Self::Gap { .. } => "gap",
            Self::MinClearance { .. } => "min_clearance",
        }
    }
}

/// **A measurement as authored**: `Expr`'s arithmetic over primitive
/// leaves, each indexing the reference list the builder is handed
/// ([`crate::measure`]), and value leaves.
///
/// Nothing stores one. The builder inserts one [`crate::Node::Measure`]
/// per primitive, in pre-order, and lowers the arithmetic to a
/// [`crate::Formula`] over their outputs ([`Self::formula`]).
///
/// Private fields and fallible constructors, exactly as [`Expr`]: an
/// ill-dimensioned tree is unrepresentable, so the cached
/// [`Self::dim`] is trustworthy by construction. A measurement nests at
/// most [`MAX_NESTING`] levels, the bound a formula has, a value leaf
/// counting as the formula it holds.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureExpr {
    dim: Dimension,
    nesting: u8,
    kind: MeasureKind,
}

impl Drop for MeasureExpr {
    /// Frees the tree from a heap stack, as [`Expr`]'s drop does.
    fn drop(&mut self) {
        if !matches!(self.kind, MeasureKind::Primitive(_) | MeasureKind::Value(_)) {
            crate::tree::free(self, |e, out| e.kind.detach_children(out));
        }
    }
}

/// The measurement AST. Private so trees are only built through the
/// dimension-checking constructors.
#[derive(Debug, Clone, PartialEq)]
enum MeasureKind {
    /// A closed-form measurement of the builder's references.
    Primitive(MeasurePrimitive<u32>),
    /// An ordinary formula.
    Value(crate::Formula),
    Add(Box<MeasureExpr>, Box<MeasureExpr>),
    Sub(Box<MeasureExpr>, Box<MeasureExpr>),
    Neg(Box<MeasureExpr>),
    Mul(Box<MeasureExpr>, Box<MeasureExpr>),
    Div(Box<MeasureExpr>, Box<MeasureExpr>),
    Min(Box<MeasureExpr>, Box<MeasureExpr>),
    Max(Box<MeasureExpr>, Box<MeasureExpr>),
}

impl MeasureKind {
    /// Moves this node's children onto `out`, leaving a leaf behind.
    fn detach_children(&mut self, out: &mut Vec<MeasureExpr>) {
        let leaf = MeasureKind::Primitive(MeasurePrimitive::Distance { a: 0, b: 0 });
        match core::mem::replace(self, leaf) {
            MeasureKind::Add(a, b)
            | MeasureKind::Sub(a, b)
            | MeasureKind::Mul(a, b)
            | MeasureKind::Div(a, b)
            | MeasureKind::Min(a, b)
            | MeasureKind::Max(a, b) => {
                out.push(*a);
                out.push(*b);
            }
            MeasureKind::Neg(a) => out.push(*a),
            MeasureKind::Primitive(_) | MeasureKind::Value(_) => {}
        }
    }

    /// How many levels the node's children nest, the deeper one's.
    fn below(&self) -> u8 {
        match self {
            MeasureKind::Add(a, b)
            | MeasureKind::Sub(a, b)
            | MeasureKind::Mul(a, b)
            | MeasureKind::Div(a, b)
            | MeasureKind::Min(a, b)
            | MeasureKind::Max(a, b) => a.nesting.max(b.nesting),
            MeasureKind::Neg(a) => a.nesting,
            MeasureKind::Primitive(_) | MeasureKind::Value(_) => 0,
        }
    }
}

/// The binary operations this language shares with [`Expr`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
}

/// **The F1 lattice, asked rather than restated**: the real `Expr`
/// constructor run over probe readers at the operand dimensions.
fn lattice(op: Binop, left: Dimension, right: Dimension) -> Result<Dimension, DimensionError> {
    let probe = |dim| Expr::var(crate::var::VarId::new(0, 0), dim);
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
// the reason `Expr`'s do, and cannot implement them for the same
// reason: they are fallible associated functions.
#[allow(clippy::should_implement_trait)]
impl MeasureExpr {
    /// This expression's dimension (cached; correct by construction).
    pub fn dim(&self) -> Dimension {
        self.dim
    }

    /// A closed-form measurement leaf.
    pub fn primitive(p: MeasurePrimitive<u32>) -> Self {
        Self {
            dim: p.dim(),
            nesting: 1,
            kind: MeasureKind::Primitive(p),
        }
    }

    /// An ordinary formula as a leaf — a literal, a variable, a whole
    /// arithmetic subtree of them.
    pub fn value(e: crate::Formula) -> Self {
        let Ok(nesting) = u8::try_from(e.nesting()) else {
            unreachable!(
                "a formula nests {} levels, past the bound of {MAX_NESTING} its every \
                 constructor holds it to",
                e.nesting()
            )
        };
        Self {
            dim: e.dim(),
            nesting,
            kind: MeasureKind::Value(e),
        }
    }

    fn over(dim: Dimension, kind: MeasureKind) -> Result<Self, DimensionError> {
        Ok(Self {
            dim,
            nesting: crate::expr::nesting_over(kind.below())?,
            kind,
        })
    }

    fn binary(
        op: Binop,
        a: Self,
        b: Self,
        make: fn(Box<Self>, Box<Self>) -> MeasureKind,
    ) -> Result<Self, DimensionError> {
        let dim = lattice(op, a.dim, b.dim)?;
        Self::over(dim, make(Box::new(a), Box::new(b)))
    }

    /// Same-dimension addition.
    pub fn add(a: Self, b: Self) -> Result<Self, DimensionError> {
        Self::binary(Binop::Add, a, b, MeasureKind::Add)
    }

    /// Same-dimension subtraction.
    pub fn sub(a: Self, b: Self) -> Result<Self, DimensionError> {
        Self::binary(Binop::Sub, a, b, MeasureKind::Sub)
    }

    /// Negation — any dimension.
    ///
    /// # Errors
    ///
    /// [`DimensionError::NestedTooDeep`] alone, as [`Expr::neg`].
    pub fn neg(a: Self) -> Result<Self, DimensionError> {
        Self::over(a.dim, MeasureKind::Neg(Box::new(a)))
    }

    /// Product; at least one operand `Scalar` (F1).
    pub fn mul(a: Self, b: Self) -> Result<Self, DimensionError> {
        Self::binary(Binop::Mul, a, b, MeasureKind::Mul)
    }

    /// Quotient; the divisor must be `Scalar` (F1).
    pub fn div(a: Self, b: Self) -> Result<Self, DimensionError> {
        Self::binary(Binop::Div, a, b, MeasureKind::Div)
    }

    /// Same-dimension lattice minimum.
    pub fn min(a: Self, b: Self) -> Result<Self, DimensionError> {
        Self::binary(Binop::Min, a, b, MeasureKind::Min)
    }

    /// Same-dimension lattice maximum.
    pub fn max(a: Self, b: Self) -> Result<Self, DimensionError> {
        Self::binary(Binop::Max, a, b, MeasureKind::Max)
    }

    /// **The one primitive this expression is**, when it is nothing
    /// else: no value leaf and no arithmetic. Such a measurement is one
    /// `Measure` node, and its value is that node's output.
    pub fn as_primitive(&self) -> Option<MeasurePrimitive<u32>> {
        match self.kind {
            MeasureKind::Primitive(p) => Some(p),
            _ => None,
        }
    }

    /// Every primitive in the tree, in pre-order: the order the builder
    /// inserts their measures in.
    pub fn primitives(&self) -> Vec<MeasurePrimitive<u32>> {
        let mut out = Vec::new();
        let mut stack = vec![self];
        while let Some(e) = stack.pop() {
            match &e.kind {
                MeasureKind::Primitive(p) => out.push(*p),
                MeasureKind::Value(_) => {}
                MeasureKind::Neg(a) => stack.push(a),
                MeasureKind::Add(a, b)
                | MeasureKind::Sub(a, b)
                | MeasureKind::Mul(a, b)
                | MeasureKind::Div(a, b)
                | MeasureKind::Min(a, b)
                | MeasureKind::Max(a, b) => {
                    stack.push(b);
                    stack.push(a);
                }
            }
        }
        out
    }

    /// The names this expression's value leaves read, with the
    /// dimension each is read at ([`crate::Formula::named_reads`]
    /// lifted to this language).
    pub fn named_reads(&self, out: &mut Vec<(crate::doc::VarName, Dimension)>) {
        let mut stack = vec![self];
        while let Some(e) = stack.pop() {
            match &e.kind {
                MeasureKind::Primitive(_) => {}
                MeasureKind::Value(v) => v.named_reads(out),
                MeasureKind::Neg(a) => stack.push(a),
                MeasureKind::Add(a, b)
                | MeasureKind::Sub(a, b)
                | MeasureKind::Mul(a, b)
                | MeasureKind::Div(a, b)
                | MeasureKind::Min(a, b)
                | MeasureKind::Max(a, b) => {
                    stack.push(b);
                    stack.push(a);
                }
            }
        }
    }

    /// **This measurement as a formula over measured values**: primitive
    /// `i` (in [`Self::primitives`]' pre-order) read as `outputs[i]`,
    /// every value leaf and operator kept.
    ///
    /// # Panics
    ///
    /// When `outputs` is shorter than the tree's primitives: the
    /// builder hands it one output per primitive it inserted.
    pub fn formula(&self, outputs: &[crate::var::VarId]) -> crate::Formula {
        use crate::Formula as F;
        use crate::tree::{Operands as O, Visit};
        let mut next = 0usize;
        let built: Result<F, DimensionError> = crate::tree::fold(
            self,
            |e| {
                Ok(match &e.kind {
                    MeasureKind::Primitive(p) => {
                        let Some(&var) = outputs.get(next) else {
                            unreachable!(
                                "primitive {next} of a measurement read past the {} outputs its \
                                 builder minted, one per primitive",
                                outputs.len()
                            )
                        };
                        next += 1;
                        Visit::Value(F::var(var, p.dim()))
                    }
                    MeasureKind::Value(v) => Visit::Value(v.clone()),
                    MeasureKind::Neg(a) => Visit::One(a),
                    MeasureKind::Add(a, b)
                    | MeasureKind::Sub(a, b)
                    | MeasureKind::Mul(a, b)
                    | MeasureKind::Div(a, b)
                    | MeasureKind::Min(a, b)
                    | MeasureKind::Max(a, b) => Visit::Two(a, b),
                })
            },
            |e, operands| match (&e.kind, operands) {
                (MeasureKind::Neg(_), O::One(a)) => F::neg(a),
                (MeasureKind::Add(..), O::Two(a, b)) => F::add(a, b),
                (MeasureKind::Sub(..), O::Two(a, b)) => F::sub(a, b),
                (MeasureKind::Mul(..), O::Two(a, b)) => F::mul(a, b),
                (MeasureKind::Div(..), O::Two(a, b)) => F::div(a, b),
                (MeasureKind::Min(..), O::Two(a, b)) => F::min(a, b),
                (MeasureKind::Max(..), O::Two(a, b)) => F::max(a, b),
                _ => unreachable!(
                    "a leaf is valued when visited, and an operator combined from as many \
                     operands as it has children"
                ),
            },
        );
        match built {
            Ok(formula) => formula,
            Err(e) => unreachable!(
                "a measurement's arithmetic was dimension-checked and bounded at construction \
                 by the formula constructors' own rules, yet lowering it refused: {e}"
            ),
        }
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
        /// ([`geom_core::Real::NAME`]).
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
                "`{verb}` answers with a certified enclosure, which only the {interval} \
                 scalar computes; a {scalar} point search finds an upper bound on the minimum, \
                 not the minimum. Recourse: evaluate the document at the {interval} scalar \
                 over a parameter box, where `{door}` computes the bracket",
                interval = <geom_core::Interval as geom_core::Real>::NAME,
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
/// (`clearance::min_separation`) and so is the only `Some`.
pub trait MinClearanceLane: geom_core::Real {
    /// The minimum separation between two resolved selections, or
    /// `None` when this scalar cannot carry an enclosure.
    ///
    /// # Errors
    ///
    /// The engine's own typed refusal
    /// ([`crate::clearance::ClearanceRefusal`]), carried unaltered.
    fn min_separation(
        a: &MinClearanceOperand<'_, Self>,
        b: &MinClearanceOperand<'_, Self>,
    ) -> Option<Result<Self, crate::clearance::ClearanceRefusal>>;
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

/// A point scalar has no enclosure to answer with. The whole content of
/// the trait, at the lane where it bites.
impl MinClearanceLane for f64 {
    fn min_separation(
        _a: &MinClearanceOperand<'_, Self>,
        _b: &MinClearanceOperand<'_, Self>,
    ) -> Option<Result<Self, crate::clearance::ClearanceRefusal>> {
        None
    }
}

/// The recording scalar is `f64` with a sink attached, so it carries
/// exactly what `f64` carries — here, nothing.
#[cfg(feature = "probe")]
impl MinClearanceLane for geom_core::Probe {
    fn min_separation(
        _a: &MinClearanceOperand<'_, Self>,
        _b: &MinClearanceOperand<'_, Self>,
    ) -> Option<Result<Self, crate::clearance::ClearanceRefusal>> {
        None
    }
}

/// **A dual does not certify** (the D1 ruling, unmoved). Its value
/// channel is whatever it is built over, and a `Dual<Interval>`'s
/// enclosure would be a certified answer arriving through a type E9
/// and D1 keep out of the certifying lanes — so the whole family
/// answers `None`, and a document measured for sensitivities reports
/// the same typed absence a plain f64 build does.
impl<T> MinClearanceLane for geom_core::Dual<T>
where
    geom_core::Dual<T>: geom_core::Real,
{
    fn min_separation(
        _a: &MinClearanceOperand<'_, Self>,
        _b: &MinClearanceOperand<'_, Self>,
    ) -> Option<Result<Self, crate::clearance::ClearanceRefusal>> {
        None
    }
}

/// The interval lane, and the only one that answers: the engine's own
/// bracket, at the shipped dials.
impl MinClearanceLane for geom_core::Interval {
    fn min_separation(
        a: &MinClearanceOperand<'_, Self>,
        b: &MinClearanceOperand<'_, Self>,
    ) -> Option<Result<Self, crate::clearance::ClearanceRefusal>> {
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
            .map(|m| m.enclosure()),
        )
    }
}

/// **The symbolic tier has no clearance lane, and the absence is a
/// DISCLOSED limitation rather than a design position** (E12's unit,
/// deviation D3; issue `symbolic-tier-and-clearance-engine`).
///
/// Every other lane the tier composes with is scalar-generic and runs
/// at `Sym<T>` unaltered. This one is not: `clearance`'s engine
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
    fn min_separation(
        _a: &MinClearanceOperand<'_, Self>,
        _b: &MinClearanceOperand<'_, Self>,
    ) -> Option<Result<Self, crate::clearance::ClearanceRefusal>> {
        None
    }
}

/// Which way an [`Assertion`](crate::Node::Assertion) constrains its
/// measure (E10).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
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

impl<T> AssertionVerdict<T> {
    /// The same verdict with both numbers taken through `f` — the door
    /// a lane that evaluated at a WRAPPED scalar reports through, so a
    /// consumer sees the enclosure and not the wrapper.
    pub fn map<U>(self, f: impl Fn(T) -> U) -> AssertionVerdict<U> {
        match self {
            Self::Holds { measured, bound } => AssertionVerdict::Holds {
                measured: f(measured),
                bound: f(bound),
            },
            Self::Violated { measured, bound } => AssertionVerdict::Violated {
                measured: f(measured),
                bound: f(bound),
            },
            Self::Unevaluated { reason } => AssertionVerdict::Unevaluated { reason },
        }
    }
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
// `Eq` is not derived: the undecided arm keeps the escalation, whose
// reporting margin is floating point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnevaluatedReason {
    /// The margin between measured and bound landed in the sliver
    /// band: the run's tolerance cannot separate them, and guessing a
    /// side would manufacture the certainty the band exists to deny.
    ///
    /// **It keeps the escalation**, so the refusal ends through
    /// [`ASSERT_BOUND_DECISION`] with the value its own margin gives
    /// (D4 ¶1 (i)) rather than offering an unvalued tighten that a
    /// poisoned margin and an enclosure straddling zero would both
    /// receive.
    Indeterminate {
        /// The escalation of the measured-versus-bound decision, with
        /// the reporting margin it was classified on.
        cause: geom_core::Indeterminate,
    },
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
        /// The tracker item the refusal points a reader at. **Not the
        /// item whose fix retires it** — see [`WINDOW_TIGHTENING`],
        /// which narrows the superset on the clearance sweep's side
        /// and retires nothing here.
        recourse: &'static str,
    },
}

/// **The tracker item this refusal points a reader at**, and which
/// retires nothing on this path: a carrier window cut to the chart
/// boundary of its trimmed face. Named from the type so the pointer
/// travels with the refusal instead of living in a reader's memory.
///
/// **What it bought, and where.** The clearance SWEEP's windows are
/// cut and its cells dropped. The path that raises
/// [`UnevaluatedReason::WindowSuperset`] is
/// `min_clearance -> clearance::min_separation`, and **no window on
/// that path is tightened at all**: minting a description inside an
/// evaluation records the boundary walk's funnel rows in the leaf's
/// census while the `f64` witness build never walks, so every leaf of
/// a drive over such a document refuses `flip_crossing`
/// (`work/trim/min-separation-tightening-crosses-the-drive.md`).
///
/// **And it would not retire the refusal even there.** The refusal
/// retires when `m = M` — when the set the engine measures over is the
/// trimmed face exactly. A tightened window is not that: a cell
/// straddling the described boundary, or lying within `K · ε` of it,
/// is KEPT, because the description is a certificate and every
/// rounding in it keeps the cell. So `hi` would remain a minimum over
/// a superset and the two unsound arms would still refuse. The
/// recourse that retires them is exact-region cells,
/// `work/trim/exact-region-cells-for-lower-bound-only.md`.
pub const WINDOW_TIGHTENING: &str = "work/trim/clearance-window-tightening-needs-chart-boundary.md";

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
/// rather than answering; [`WINDOW_TIGHTENING`] narrows the superset
/// on the clearance sweep's side, leaves this path's windows
/// untightened, and retires the refusal in neither place.
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
            Self::Indeterminate { cause } => write!(
                f,
                "the measured value and the bound are not separated at this tolerance, so the \
                 assertion has no verdict. {}",
                ASSERT_BOUND_DECISION.recourse(RefusedArm::Undecided(cause), Reading::AtRest)
            ),
            Self::MeasureUnavailable(why) => write!(f, "there is no measured value: {why}"),
            Self::WindowSuperset {
                verb,
                endpoint,
                recourse,
            } => write!(
                f,
                "this verdict would be read off the {endpoint} end of an enclosure `{verb}` \
                 certifies only over carrier windows, a superset of the faces the measure \
                 names. The opposite verdict on this bound still gates. Recourse: {recourse}"
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
            verb: "min_clearance",
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
        Err(cause) => AssertionVerdict::Unevaluated {
            reason: UnevaluatedReason::Indeterminate { cause },
        },
    }
}

/// The funnel site name of the assertion comparison. A roster carrier
/// (`docs/K-REPORT.md`) rather than a literal at the decide site.
pub const ASSERT_BOUND: &str = "assert_bound";

/// **The assertion's own decision**, as the one ending table reads it
/// (D4 ¶1 (i)): the comparand is `measured − bound` and BOTH definite
/// signs are verdicts — at the bound exactly a non-strict relation holds
/// — so the only refused arm is the undecided one, and the lever is the
/// bound the document names. A smaller tolerance decides an in-band
/// margin, so the offer is valued from that margin; a straddling
/// enclosure and an unreadable margin get the lever alone, which is the
/// defect this spelling closes.
pub const ASSERT_BOUND_DECISION: SizedDecision = SizedDecision {
    lever: "move the bound",
    size: "difference",
    passes: SizedPass::AnySign,
    // The stored description the lever edits IS the asserted bound.
    stored: StoredDefinite::Lever,
    at_zero: None,
};

#[cfg(test)]
mod tests {
    use super::*;

    /// The authored measurement's pre-order walk and its `Drop` cost the
    /// stack nothing per level: a million levels, built past the
    /// constructors, walk and free on the wasm32 stack.
    #[test]
    fn the_measurement_walk_and_drop_keep_their_own_stack() {
        test_utils::own_thread::on_the_smallest_stack(|| {
            let leaf = MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 1, inner: 0 });
            let deep = crate::tree::raw_chain(leaf, 1_000_000, |e| MeasureExpr {
                dim: e.dim,
                nesting: u8::MAX,
                kind: MeasureKind::Neg(Box::new(e)),
            });
            assert_eq!(
                deep.primitives(),
                vec![MeasurePrimitive::Gap { outer: 1, inner: 0 }],
                "the one primitive is met once"
            );
            drop(deep);
        });
    }
}
