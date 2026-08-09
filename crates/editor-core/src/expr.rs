//! The expression sublanguage v1 (spec D4, ratified forks F1 + F7).
//!
//! A small typed AST: dimensioned literals, document-parameter refs,
//! arithmetic, trig, min/max. **No conditionals, no iteration, no
//! user-defined functions** — total by construction (F7); case analysis
//! belongs to structural parameters.
//!
//! Dimension checking happens at expression CONSTRUCTION time via the
//! smart constructors on [`Expr`]; an ill-dimensioned tree is
//! unrepresentable. The F1 restrictive lattice: same-dimension
//! add/sub/min/max; `Mul` requires at least one [`Dimension::Scalar`]
//! operand; `Div` requires a `Scalar` divisor. Same-dimension ratios
//! (Length/Length → Scalar) are REFUSED in v1 — the full rational-
//! exponent lattice is a purely additive future extension, so the
//! refusal forecloses nothing; it is pinned by test.
//!
//! Units erase at the evaluation boundary (GQ5): [`eval`] returns raw
//! `T` in kernel units (meters/radians); display units are document
//! presentation metadata, not this layer's concern.

use geom_core::Real;
use geom_core::predicate::{Band, Decide, Sign};

use crate::doc::ParamName;
use crate::node::{RecipeNodeId, SlotId};

/// The v1 quantity-dimension lattice (ratified F1, GQ5's banked
/// decision): four dimensions, no products of dimensions.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum Dimension {
    /// A length, canonically meters (units erase before kernel `T`).
    Length,
    /// An angle, canonically radians.
    Angle,
    /// A dimensionless INTEGER (structural: pattern counts, indices).
    /// Closed under add/sub/mul/neg/min/max; promotion to `Scalar` is
    /// explicit ([`Expr::count_to_scalar`]), never implicit (spec D4).
    Count,
    /// A dimensionless real.
    Scalar,
}

/// Typed refusal from the construction-time dimension checker (spec D4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DimensionError {
    /// Add/Sub/Min/Max/Atan2 over two different dimensions.
    Mismatch {
        /// The operation that was refused.
        op: &'static str,
        /// Left operand's dimension.
        left: Dimension,
        /// Right operand's dimension.
        right: Dimension,
    },
    /// `Mul` with neither operand `Scalar` (covers Length×Length —
    /// dimension-changing products are v1 refusals, pinned by test).
    MulNeedsScalar {
        /// Left operand's dimension.
        left: Dimension,
        /// Right operand's dimension.
        right: Dimension,
    },
    /// `Div` with a non-`Scalar` divisor (covers Length/Length — the
    /// same-dimension ratio is REFUSED in v1; relaxation is additive).
    DivNeedsScalarDivisor {
        /// Dividend's dimension.
        left: Dimension,
        /// Divisor's dimension.
        right: Dimension,
    },
    /// Trig applied to a non-`Angle` operand.
    TrigNeedsAngle {
        /// The trig operation refused.
        op: &'static str,
        /// The operand's actual dimension.
        found: Dimension,
    },
    /// A `Count` operand where a continuous dimension is required —
    /// implicit Count→Scalar promotion is refused (spec D4); use
    /// [`Expr::count_to_scalar`].
    CountNeedsExplicitPromotion {
        /// The operation that would have promoted implicitly.
        op: &'static str,
    },
    /// A non-`Count` operand to an operation requiring `Count`
    /// ([`Expr::count_to_scalar`]).
    NotCount {
        /// The operand's actual dimension.
        found: Dimension,
    },
    /// A literal constructed with [`Dimension::Count`] — Count literals
    /// are integers, made by [`Expr::count`].
    LiteralCountIsInteger,
    /// A non-finite (NaN/±inf) literal — refused at construction (the
    /// M4 PR 1 review's ruled "door 1": the kernel never produces
    /// non-finite values legitimately, so admitting one into recipe
    /// data would smuggle poison past every downstream check).
    NonFiniteLiteral,
    /// A literal's display unit measures a different quantity than the
    /// literal's dimension (`mm` can only suffix a `Length`; `deg` only
    /// an `Angle`; `Scalar` literals take no unit). LIB-SWITCH §4g: the
    /// display unit is presentation metadata, but a MISMATCHED one is
    /// corrupt data, refused at construction like every other dimension
    /// fault.
    DisplayUnitMismatch {
        /// The dimension the unit's quantity implies.
        unit: Dimension,
        /// The literal's declared dimension.
        literal: Dimension,
    },
    /// A persisted display-unit symbol outside quantity's closed table
    /// (the load door's strict-vocabulary refusal; the wire form stores
    /// the symbol as text).
    UnknownDisplayUnit {
        /// The unrecognized symbol.
        symbol: String,
    },
}

/// A dimension-checked expression tree (ratified F7 shape).
///
/// Construction goes through the smart constructors below, which run
/// the F1 dimension checker; the fields are private so an
/// ill-dimensioned tree cannot be built. The cached [`Self::dim`] is
/// therefore trustworthy by construction.
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    dim: Dimension,
    kind: ExprKind,
}

/// A stored continuous literal: the canonical-units value plus its
/// per-literal DISPLAY unit (LIB-SWITCH §4g, U8b folded into the v4
/// break). The unit is presentation metadata under D7's hard rules —
/// it is EXCLUDED from equality here (so [`Expr::bit_eq`], content
/// keys, and naming keys are all display-unit-blind by construction),
/// excluded from [`Expr::literal_bits`], and ignored by evaluation;
/// the value stays canonical meters/radians regardless.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Lit {
    /// The exact canonical-units value (bit-exact per D7).
    pub(crate) value: f64,
    /// The display unit the literal was authored in, if any (quantity's
    /// closed table; `None` renders canonically).
    pub(crate) display_unit: Option<quantity::UnitDef>,
}

impl PartialEq for Lit {
    /// IEEE-semantic on the VALUE only — the display unit is
    /// presentation metadata and never part of expression identity
    /// (D7; two literals differing only in display unit are the same
    /// expression).
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// The node vocabulary of the AST (private: constructors check dims).
///
/// Child order (the ExprPath byte at each level, spec D5): operands in
/// argument order — 0 = first/only child, 1 = second.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExprKind {
    /// A continuous dimensioned literal, canonical kernel units
    /// (meters/radians); bit-exact f64 storage per D7 replay identity.
    Literal(Lit),
    /// A `Count` literal — exact integer (spec D4: Count is
    /// integer-valued, never a float).
    CountLiteral(i64),
    /// A document-level named-parameter reference, carrying the
    /// dimension the parameter was declared with at construction time
    /// (`apply` re-checks it against the document's table).
    Param(ParamName),
    /// Same-dimension addition.
    Add(Box<Expr>, Box<Expr>),
    /// Same-dimension subtraction.
    Sub(Box<Expr>, Box<Expr>),
    /// Negation (any dimension, including Count).
    Neg(Box<Expr>),
    /// Product; ≥1 operand dimensionless (`Scalar`), or Count×Count.
    Mul(Box<Expr>, Box<Expr>),
    /// Quotient; the divisor must be `Scalar`.
    Div(Box<Expr>, Box<Expr>),
    /// Sine of an `Angle`, yielding `Scalar`.
    Sin(Box<Expr>),
    /// Cosine of an `Angle`, yielding `Scalar`.
    Cos(Box<Expr>),
    /// Tangent of an `Angle`, yielding `Scalar`.
    Tan(Box<Expr>),
    /// Four-quadrant arctangent of same-dimension (y, x), yielding
    /// `Angle`.
    Atan2(Box<Expr>, Box<Expr>),
    /// Same-dimension lattice minimum (a value operation, never
    /// control flow — the AST has no branches; F7).
    Min(Box<Expr>, Box<Expr>),
    /// Same-dimension lattice maximum.
    Max(Box<Expr>, Box<Expr>),
    /// EXPLICIT Count→Scalar promotion (spec D4: never implicit).
    CountToScalar(Box<Expr>),
}

// The arithmetic constructors share names with the std ops traits on
// purpose (they ARE the expression-level add/sub/…), but they cannot
// implement those traits: they are FALLIBLE (the F1 dimension checker
// runs at construction) and associated functions, not methods.
#[allow(clippy::should_implement_trait)]
impl Expr {
    /// This expression's dimension (cached; correct by construction).
    pub fn dim(&self) -> Dimension {
        self.dim
    }

    /// The AST node (persistence's wire conversion reads it; the type
    /// stays crate-private so trees are only built through the
    /// dimension-checking constructors).
    pub(crate) fn kind(&self) -> &ExprKind {
        &self.kind
    }

    /// A continuous dimensioned literal in canonical kernel units.
    /// Refuses [`Dimension::Count`] — Count literals are integers
    /// ([`Expr::count`]) — and NON-FINITE values (ruled door 1 of the
    /// non-finite policy: the kernel never produces NaN/inf
    /// legitimately, so recipe data must not admit them; F3's
    /// persist-time refusal then has nothing to catch).
    pub fn literal(value: f64, dim: Dimension) -> Result<Self, DimensionError> {
        if dim == Dimension::Count {
            return Err(DimensionError::LiteralCountIsInteger);
        }
        if !value.is_finite() {
            return Err(DimensionError::NonFiniteLiteral);
        }
        Ok(Self {
            dim,
            kind: ExprKind::Literal(Lit {
                value,
                display_unit: None,
            }),
        })
    }

    /// A continuous literal that REMEMBERS the display unit it was
    /// authored in (LIB-SWITCH §4g; the text door's `25 mm` row).
    /// `value` is already canonical (meters/radians) — the parser does
    /// its one multiply before this door. The unit's quantity must
    /// agree with `dim` ([`DimensionError::DisplayUnitMismatch`]);
    /// everything [`Expr::literal`] refuses is refused here too.
    ///
    /// The unit is presentation metadata (D7): it round-trips through
    /// persistence and feeds the display formatter, but never enters
    /// [`Expr::bit_eq`], [`Expr::literal_bits`], content/naming keys,
    /// or evaluation.
    pub fn literal_with_unit(
        value: f64,
        dim: Dimension,
        unit: quantity::UnitDef,
    ) -> Result<Self, DimensionError> {
        let unit_dim = match unit.quantity {
            quantity::UnitQuantity::Length => Dimension::Length,
            quantity::UnitQuantity::Angle => Dimension::Angle,
        };
        if unit_dim != dim {
            return Err(DimensionError::DisplayUnitMismatch {
                unit: unit_dim,
                literal: dim,
            });
        }
        // Run literal()'s refusal doors, then attach the unit.
        let mut e = Self::literal(value, dim)?;
        if let ExprKind::Literal(ref mut lit) = e.kind {
            lit.display_unit = Some(unit);
        }
        Ok(e)
    }

    /// The display unit of a LITERAL expression, if one is stored
    /// (`None` for every other kind and for canonically-authored
    /// literals). The formatter's read side (§4g).
    pub fn display_unit(&self) -> Option<quantity::UnitDef> {
        match &self.kind {
            ExprKind::Literal(lit) => lit.display_unit,
            _ => None,
        }
    }

    /// A literal's exact canonical-units value (`None` for non-literal
    /// kinds) — with [`Expr::display_unit`], the display formatter's
    /// complete read surface.
    pub fn literal_value(&self) -> Option<f64> {
        match &self.kind {
            ExprKind::Literal(lit) => Some(lit.value),
            _ => None,
        }
    }

    /// A `Count` literal — an exact integer.
    pub fn count(value: i64) -> Self {
        Self {
            dim: Dimension::Count,
            kind: ExprKind::CountLiteral(value),
        }
    }

    /// A document-parameter reference, recording the dimension the
    /// parameter is declared with; `apply` re-checks the record against
    /// the document's table (spec D6).
    pub fn param(name: ParamName, dim: Dimension) -> Self {
        Self {
            dim,
            kind: ExprKind::Param(name),
        }
    }

    fn same_dim(
        op: &'static str,
        a: Expr,
        b: Expr,
        make: fn(Box<Expr>, Box<Expr>) -> ExprKind,
    ) -> Result<Self, DimensionError> {
        if a.dim != b.dim {
            return Err(DimensionError::Mismatch {
                op,
                left: a.dim,
                right: b.dim,
            });
        }
        Ok(Self {
            dim: a.dim,
            kind: make(Box::new(a), Box::new(b)),
        })
    }

    /// Same-dimension addition (Count included: Count is closed under
    /// add/sub/mul/neg/min/max, spec D4).
    pub fn add(a: Expr, b: Expr) -> Result<Self, DimensionError> {
        Self::same_dim("add", a, b, ExprKind::Add)
    }

    /// Same-dimension subtraction.
    pub fn sub(a: Expr, b: Expr) -> Result<Self, DimensionError> {
        Self::same_dim("sub", a, b, ExprKind::Sub)
    }

    /// Negation — any dimension (Count stays Count).
    pub fn neg(a: Expr) -> Self {
        Self {
            dim: a.dim,
            kind: ExprKind::Neg(Box::new(a)),
        }
    }

    /// Product. Permitted (F1): Count×Count → Count; otherwise at
    /// least one operand `Scalar`, result the other's dimension.
    /// Length×Length is a typed refusal (dimension-changing products
    /// are out of the v1 lattice; relaxation is additive). A single
    /// Count operand mixed with a continuous one is refused — promote
    /// explicitly via [`Expr::count_to_scalar`].
    pub fn mul(a: Expr, b: Expr) -> Result<Self, DimensionError> {
        use Dimension::{Count, Scalar};
        let dim = match (a.dim, b.dim) {
            (Count, Count) => Count,
            (Count, _) | (_, Count) => {
                return Err(DimensionError::CountNeedsExplicitPromotion { op: "mul" });
            }
            (Scalar, d) | (d, Scalar) => d,
            (l, r) => return Err(DimensionError::MulNeedsScalar { left: l, right: r }),
        };
        Ok(Self {
            dim,
            kind: ExprKind::Mul(Box::new(a), Box::new(b)),
        })
    }

    /// Quotient. The divisor must be `Scalar` (F1): Length/Length —
    /// the same-dimension ratio — is a typed refusal in v1 (pinned by
    /// test; relaxing to ratios later is purely additive). Count is
    /// not closed under division (spec D4 lists add/sub/mul/min/max),
    /// so any Count operand is refused — promote explicitly first.
    pub fn div(a: Expr, b: Expr) -> Result<Self, DimensionError> {
        use Dimension::{Count, Scalar};
        if a.dim == Count || b.dim == Count {
            return Err(DimensionError::CountNeedsExplicitPromotion { op: "div" });
        }
        if b.dim != Scalar {
            return Err(DimensionError::DivNeedsScalarDivisor {
                left: a.dim,
                right: b.dim,
            });
        }
        Ok(Self {
            dim: a.dim,
            kind: ExprKind::Div(Box::new(a), Box::new(b)),
        })
    }

    fn trig(
        op: &'static str,
        a: Expr,
        make: fn(Box<Expr>) -> ExprKind,
    ) -> Result<Self, DimensionError> {
        if a.dim != Dimension::Angle {
            return Err(DimensionError::TrigNeedsAngle { op, found: a.dim });
        }
        Ok(Self {
            dim: Dimension::Scalar,
            kind: make(Box::new(a)),
        })
    }

    /// Sine of an `Angle` → `Scalar` (spec D4).
    pub fn sin(a: Expr) -> Result<Self, DimensionError> {
        Self::trig("sin", a, ExprKind::Sin)
    }

    /// Cosine of an `Angle` → `Scalar`.
    pub fn cos(a: Expr) -> Result<Self, DimensionError> {
        Self::trig("cos", a, ExprKind::Cos)
    }

    /// Tangent of an `Angle` → `Scalar`.
    pub fn tan(a: Expr) -> Result<Self, DimensionError> {
        Self::trig("tan", a, ExprKind::Tan)
    }

    /// Four-quadrant arctangent `atan2(y, x)` → `Angle` (spec D4).
    /// Operands must share one continuous dimension (their common
    /// scale cancels in the true ratio); Count operands are refused —
    /// promote explicitly.
    pub fn atan2(y: Expr, x: Expr) -> Result<Self, DimensionError> {
        if y.dim == Dimension::Count || x.dim == Dimension::Count {
            return Err(DimensionError::CountNeedsExplicitPromotion { op: "atan2" });
        }
        if y.dim != x.dim {
            return Err(DimensionError::Mismatch {
                op: "atan2",
                left: y.dim,
                right: x.dim,
            });
        }
        Ok(Self {
            dim: Dimension::Angle,
            kind: ExprKind::Atan2(Box::new(y), Box::new(x)),
        })
    }

    /// Same-dimension lattice minimum (value operation, never control
    /// flow — the AST has no branches, F7; Count included).
    pub fn min(a: Expr, b: Expr) -> Result<Self, DimensionError> {
        Self::same_dim("min", a, b, ExprKind::Min)
    }

    /// Same-dimension lattice maximum.
    pub fn max(a: Expr, b: Expr) -> Result<Self, DimensionError> {
        Self::same_dim("max", a, b, ExprKind::Max)
    }

    /// EXPLICIT Count→Scalar promotion (spec D4: never implicit).
    /// Refuses non-Count operands.
    pub fn count_to_scalar(a: Expr) -> Result<Self, DimensionError> {
        if a.dim != Dimension::Count {
            return Err(DimensionError::NotCount { found: a.dim });
        }
        Ok(Self {
            dim: Dimension::Scalar,
            kind: ExprKind::CountToScalar(Box::new(a)),
        })
    }

    /// The child at ExprPath index `i` (spec D5: operands in argument
    /// order), or `None` past the arity.
    pub fn child(&self, i: u8) -> Option<&Expr> {
        use ExprKind as K;
        match (&self.kind, i) {
            (
                K::Add(a, _)
                | K::Sub(a, _)
                | K::Mul(a, _)
                | K::Div(a, _)
                | K::Atan2(a, _)
                | K::Min(a, _)
                | K::Max(a, _),
                0,
            ) => Some(a),
            (
                K::Add(_, b)
                | K::Sub(_, b)
                | K::Mul(_, b)
                | K::Div(_, b)
                | K::Atan2(_, b)
                | K::Min(_, b)
                | K::Max(_, b),
                1,
            ) => Some(b),
            (K::Neg(a) | K::Sin(a) | K::Cos(a) | K::Tan(a) | K::CountToScalar(a), 0) => Some(a),
            _ => None,
        }
    }

    /// The subtree at an AST path (a chain of [`Expr::child`] steps);
    /// `None` if the path runs off the tree.
    pub fn descend(&self, path: &[u8]) -> Option<&Expr> {
        path.iter().try_fold(self, |e, &i| e.child(i))
    }

    /// The parameter names this expression references, with their
    /// recorded dimensions (used by `apply`'s re-check, spec D6).
    pub fn param_refs(&self, out: &mut Vec<(ParamName, Dimension)>) {
        use ExprKind as K;
        match &self.kind {
            K::Param(name) => out.push((name.clone(), self.dim)),
            K::Literal(_) | K::CountLiteral(_) => {}
            K::Neg(a) | K::Sin(a) | K::Cos(a) | K::Tan(a) | K::CountToScalar(a) => {
                a.param_refs(out);
            }
            K::Add(a, b)
            | K::Sub(a, b)
            | K::Mul(a, b)
            | K::Div(a, b)
            | K::Atan2(a, b)
            | K::Min(a, b)
            | K::Max(a, b) => {
                a.param_refs(out);
                b.param_refs(out);
            }
        }
    }

    /// Pushes every continuous literal's `f64` BITS in deterministic
    /// traversal order (pre-order, children in [`Expr::child`] order)
    /// — the bit-semantic comparison substrate (spec D7: replay is
    /// bit-identical, so the comparators must not be bit-blind).
    pub fn literal_bits(&self, out: &mut Vec<u64>) {
        use ExprKind as K;
        match &self.kind {
            K::Literal(lit) => out.push(lit.value.to_bits()),
            K::CountLiteral(_) | K::Param(_) => {}
            K::Neg(a) | K::Sin(a) | K::Cos(a) | K::Tan(a) | K::CountToScalar(a) => {
                a.literal_bits(out);
            }
            K::Add(a, b)
            | K::Sub(a, b)
            | K::Mul(a, b)
            | K::Div(a, b)
            | K::Atan2(a, b)
            | K::Min(a, b)
            | K::Max(a, b) => {
                a.literal_bits(out);
                b.literal_bits(out);
            }
        }
    }

    /// Bit-semantic equality (M4 PR 1 review non-blocker): structural
    /// equality with float literals compared by BITS — `0.0` and
    /// `-0.0` are DIFFERENT expressions here, unlike `PartialEq`
    /// (which stays IEEE-semantic). NaN cannot occur in a stored
    /// expression (door 1 refuses non-finite literals), so
    /// `PartialEq` + aligned bit vectors is exact: when `self ==
    /// other`, both trees have identical shape, so the traversals
    /// align literal-for-literal.
    pub fn bit_eq(&self, other: &Expr) -> bool {
        if self != other {
            return false;
        }
        let (mut a, mut b) = (Vec::new(), Vec::new());
        self.literal_bits(&mut a);
        other.literal_bits(&mut b);
        a == b
    }

    /// A copy of `self` with the subtree at `path` replaced by `new`,
    /// re-running the dimension checker on every rebuilt ancestor (the
    /// replacement may change a subtree's dimension; ancestors must
    /// still type-check). `None` if the path runs off the tree.
    pub fn with_replaced(&self, path: &[u8], new: Expr) -> Option<Result<Expr, DimensionError>> {
        use ExprKind as K;
        let Some((&i, rest)) = path.split_first() else {
            return Some(Ok(new));
        };
        let child = self.child(i)?;
        let rebuilt = match child.with_replaced(rest, new)? {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        };
        // Re-run the smart constructor for this node with the rebuilt
        // child in position `i` (sibling clones keep their checked dims).
        let other = |b: &Expr| b.clone();
        let res = match (&self.kind, i) {
            (K::Add(_, b), 0) => Self::add(rebuilt, other(b)),
            (K::Add(a, _), 1) => Self::add(other(a), rebuilt),
            (K::Sub(_, b), 0) => Self::sub(rebuilt, other(b)),
            (K::Sub(a, _), 1) => Self::sub(other(a), rebuilt),
            (K::Mul(_, b), 0) => Self::mul(rebuilt, other(b)),
            (K::Mul(a, _), 1) => Self::mul(other(a), rebuilt),
            (K::Div(_, b), 0) => Self::div(rebuilt, other(b)),
            (K::Div(a, _), 1) => Self::div(other(a), rebuilt),
            (K::Atan2(_, b), 0) => Self::atan2(rebuilt, other(b)),
            (K::Atan2(a, _), 1) => Self::atan2(other(a), rebuilt),
            (K::Min(_, b), 0) => Self::min(rebuilt, other(b)),
            (K::Min(a, _), 1) => Self::min(other(a), rebuilt),
            (K::Max(_, b), 0) => Self::max(rebuilt, other(b)),
            (K::Max(a, _), 1) => Self::max(other(a), rebuilt),
            (K::Neg(_), 0) => Ok(Self::neg(rebuilt)),
            (K::Sin(_), 0) => Self::sin(rebuilt),
            (K::Cos(_), 0) => Self::cos(rebuilt),
            (K::Tan(_), 0) => Self::tan(rebuilt),
            (K::CountToScalar(_), 0) => Self::count_to_scalar(rebuilt),
            _ => return None,
        };
        Some(res)
    }
}

/// The address of an expression subtree inside a document (spec D5,
/// F7's "GeomSource's missing type"): a node, a NAMED slot (never an
/// index), and a chain of AST-child indices. Stable under edits to
/// other expressions and to unrelated subtrees by construction.
///
/// **Staleness caveat (M4 PR 1 review, non-blocker)**: within its OWN
/// slot a path is positional — a same-slot edit that replaces an
/// ANCESTOR of the referent (or the whole slot) with a same-shape
/// expression silently re-points an old path at a different
/// subexpression; v1 has no slot generation/version to detect this.
/// Consumers must re-derive their paths after any same-slot edit;
/// PR 5's GeomSource must NOT assume same-slot staleness is
/// detectable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExprPath {
    /// The recipe node owning the expression slot.
    pub node: RecipeNodeId,
    /// The named slot on that node (per-node-type enum, spec D5).
    pub slot: SlotId,
    /// AST-child indices from the slot's root ([`Expr::child`] order);
    /// empty addresses the whole slot expression.
    pub path: Vec<u8>,
}

/// A document-parameter value bound for evaluation (spec D4). The
/// scalar type is generic: the document stores exact `f64`/`i64`;
/// [`crate::Doc::param_env`] embeds them into any [`Real`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParamValue<T> {
    /// A continuous value with its declared dimension (kernel units).
    Continuous {
        /// The parameter's declared dimension (never `Count`).
        dim: Dimension,
        /// The value, canonical kernel units.
        value: T,
    },
    /// An exact integer `Count` value.
    Count(i64),
}

impl<T> ParamValue<T> {
    /// The bound value's dimension.
    pub fn dim(&self) -> Dimension {
        match self {
            Self::Continuous { dim, .. } => *dim,
            Self::Count(_) => Dimension::Count,
        }
    }
}

/// The name→value environment [`eval`] and [`eval_count`] read
/// parameter refs from (spec D4's `params`).
#[derive(Debug, Clone, PartialEq)]
pub struct ParamEnv<T> {
    /// The bindings, by parameter name.
    pub bindings: std::collections::BTreeMap<ParamName, ParamValue<T>>,
}

// Manual impl: the derive would demand `T: Default`, which certified
// scalars (Interval) deliberately do not provide.
impl<T> Default for ParamEnv<T> {
    fn default() -> Self {
        Self {
            bindings: std::collections::BTreeMap::new(),
        }
    }
}

/// Typed evaluation failure (spec D4). Numeric-domain issues (division
/// by zero, out-of-domain trig) are NOT errors here: they follow the
/// kernel's poison-value policy through `T` (`geom-core::real` module
/// docs) — the evaluator has no branches to hide them behind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    /// A parameter ref with no binding in the environment.
    UnknownParam(ParamName),
    /// A parameter bound with a different dimension than the ref
    /// recorded at construction.
    ParamDimensionMismatch {
        /// The parameter name.
        name: ParamName,
        /// The dimension the expression's ref recorded.
        expected: Dimension,
        /// The dimension the environment bound.
        found: Dimension,
    },
    /// [`eval`] applied to a `Count`-dimension expression — Count
    /// evaluates exactly via [`eval_count`]; promotion to `T` is only
    /// through the explicit [`Expr::count_to_scalar`] node (spec D4).
    CountExprInContinuousEval,
    /// [`eval_count`] applied to a non-`Count` expression.
    ContinuousExprInCountEval {
        /// The expression's actual dimension.
        found: Dimension,
    },
    /// Exact integer Count arithmetic overflowed `i64` (fail-loud:
    /// wrapping would fabricate a count).
    CountOverflow,
    /// A `CountToScalar` promotion of a count outside `i32` range
    /// (fail-loud; the `f64` embedding goes through `i32::try_from` +
    /// the exact `f64::from(i32)`, so out-of-range counts — which are
    /// structurally absurd anyway — are typed refusals, never inexact
    /// casts; ruled at the M4 PR 1 review, replacing the ±2⁵³ guard).
    CountToScalarOutOfRange(i64),
    /// The evaluated result was NON-FINITE (ruled door 2 of the
    /// non-finite policy): with door 1 refusing non-finite literals
    /// and doc params, a non-finite RESULT means the arithmetic
    /// itself overflowed or hit a pole (1/0, 0/0) — refused at the
    /// eval boundary rather than flowed into geometry. Context: the
    /// caller supplied the expression being evaluated ([`eval`]'s
    /// argument identifies it; PR 2's evaluation service attaches
    /// node/slot when it evaluates document slots). At certified
    /// scalars this refuses POISON (NaI/empty/`Trv`-decorated
    /// enclosures); legitimately unbounded-but-valid enclosures pass
    /// (boundedness is the `Com`-decoration's business, not this
    /// door's).
    NonFiniteResult,
}

/// Evaluate a continuous expression to a raw `T` in kernel units —
/// units erase at this boundary (GQ5). Generic over the scalar (spec
/// D4, the banked scalar-genericity principle); there are no raw
/// comparisons on control-flow paths because the AST has no branches.
///
/// The bound is [`Decide`] (= `Real` + the one sanctioned door from
/// values to decisions, spec D1's "for `Real`/`Decide`"): the ruled
/// door-2 finiteness check on the FINAL value is a reified decision,
/// so it goes through `sign_within`, never a raw comparison. The
/// recursive evaluation itself needs only `Real` (see `eval_inner`).
pub fn eval<T: Decide>(expr: &Expr, params: &ParamEnv<T>) -> Result<T, EvalError> {
    let value = eval_inner(expr, params)?;
    // Door 2: probe = value * 0 is EXACTLY zero for every finite
    // value and poison (NaN / empty / Trv) otherwise, so any valid
    // band classifies it identically — Zero passes, everything else
    // is a non-finite result. Band construction with these constants
    // cannot fail; the `else` arm is unreachable but typed (no
    // panic paths in this crate).
    let Ok(band) = Band::new(1e-100, 1e-50) else {
        return Err(EvalError::NonFiniteResult);
    };
    match (value * T::zero()).sign_within(band) {
        Ok(Sign::Zero) => Ok(value),
        _ => Err(EvalError::NonFiniteResult),
    }
}

/// The recursive evaluation core — `Real` only (no decisions inside:
/// poison FLOWS through values per the kernel policy; the single
/// refusal door is [`eval`]'s final check).
fn eval_inner<T: Real>(expr: &Expr, params: &ParamEnv<T>) -> Result<T, EvalError> {
    use ExprKind as K;
    if expr.dim == Dimension::Count {
        return Err(EvalError::CountExprInContinuousEval);
    }
    match &expr.kind {
        // The display unit is presentation metadata: evaluation reads
        // only the canonical value (D7; LIB-SWITCH §4g).
        K::Literal(lit) => Ok(T::from_f64(lit.value)),
        K::CountLiteral(_) => Err(EvalError::CountExprInContinuousEval),
        K::Param(name) => match params.bindings.get(name) {
            None => Err(EvalError::UnknownParam(name.clone())),
            Some(ParamValue::Continuous { dim, value }) if *dim == expr.dim => Ok(*value),
            Some(bound) => Err(EvalError::ParamDimensionMismatch {
                name: name.clone(),
                expected: expr.dim,
                found: bound.dim(),
            }),
        },
        K::Add(a, b) => Ok(eval_inner(a, params)? + eval_inner(b, params)?),
        K::Sub(a, b) => Ok(eval_inner(a, params)? - eval_inner(b, params)?),
        K::Neg(a) => Ok(-eval_inner(a, params)?),
        K::Mul(a, b) => Ok(eval_inner(a, params)? * eval_inner(b, params)?),
        K::Div(a, b) => Ok(eval_inner(a, params)? / eval_inner(b, params)?),
        K::Sin(a) => Ok(eval_inner(a, params)?.sin()),
        K::Cos(a) => Ok(eval_inner(a, params)?.cos()),
        K::Tan(a) => Ok(eval_inner(a, params)?.tan()),
        K::Atan2(y, x) => Ok(eval_inner(y, params)?.atan2(eval_inner(x, params)?)),
        K::Min(a, b) => Ok(eval_inner(a, params)?.min(eval_inner(b, params)?)),
        K::Max(a, b) => Ok(eval_inner(a, params)?.max(eval_inner(b, params)?)),
        K::CountToScalar(a) => {
            let n = eval_count(a, params)?;
            // i32::try_from is total on i64 (no abs, no panic —
            // i64::MIN is a typed refusal); f64::from(i32) is exact.
            let small = i32::try_from(n).map_err(|_| EvalError::CountToScalarOutOfRange(n))?;
            Ok(T::from_f64(f64::from(small)))
        }
    }
}

/// Evaluate a `Count` expression to an exact `i64` (spec D4: Count is
/// integer-valued; arithmetic is checked, overflow a typed error).
pub fn eval_count<T>(expr: &Expr, params: &ParamEnv<T>) -> Result<i64, EvalError> {
    use ExprKind as K;
    if expr.dim != Dimension::Count {
        return Err(EvalError::ContinuousExprInCountEval { found: expr.dim });
    }
    let checked = |r: Option<i64>| r.ok_or(EvalError::CountOverflow);
    match &expr.kind {
        K::CountLiteral(n) => Ok(*n),
        K::Param(name) => match params.bindings.get(name) {
            None => Err(EvalError::UnknownParam(name.clone())),
            Some(ParamValue::Count(n)) => Ok(*n),
            Some(bound) => Err(EvalError::ParamDimensionMismatch {
                name: name.clone(),
                expected: Dimension::Count,
                found: bound.dim(),
            }),
        },
        K::Add(a, b) => checked(eval_count(a, params)?.checked_add(eval_count(b, params)?)),
        K::Sub(a, b) => checked(eval_count(a, params)?.checked_sub(eval_count(b, params)?)),
        K::Neg(a) => checked(eval_count(a, params)?.checked_neg()),
        K::Mul(a, b) => checked(eval_count(a, params)?.checked_mul(eval_count(b, params)?)),
        K::Min(a, b) => Ok(eval_count(a, params)?.min(eval_count(b, params)?)),
        K::Max(a, b) => Ok(eval_count(a, params)?.max(eval_count(b, params)?)),
        // Construction makes these unrepresentable at Count dimension.
        K::Literal(_)
        | K::Div(..)
        | K::Sin(_)
        | K::Cos(_)
        | K::Tan(_)
        | K::Atan2(..)
        | K::CountToScalar(_) => Err(EvalError::ContinuousExprInCountEval { found: expr.dim }),
    }
}
