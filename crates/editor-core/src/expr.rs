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
    ///
    /// Raised at exactly one place — `persist::wire`'s rebuild, where
    /// the symbol arrives as a STRING out of a file and
    /// `quantity::unit_by_symbol` can genuinely fail. Construction
    /// cannot raise it: since #650 sealed `quantity::UnitDef`, every
    /// row a caller can hold is a table row.
    UnknownDisplayUnit {
        /// The unrecognized symbol.
        symbol: String,
    },
}

// LIB-DOORS F6 (reopened on review): a human-readable rendering. The
// comment-style rule applies — each arm states the PROBLEM, not the
// enum's guts; the enum itself remains the machine contract.
impl core::fmt::Display for DimensionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Mismatch { op, left, right } => {
                write!(f, "cannot apply `{op}` to {left:?} and {right:?}")
            }
            Self::MulNeedsScalar { left, right } => write!(
                f,
                "multiplication needs a Scalar operand ({left:?} x {right:?} would change dimension)"
            ),
            Self::DivNeedsScalarDivisor { left, right } => write!(
                f,
                "division needs a Scalar divisor ({left:?} / {right:?} is refused in v1)"
            ),
            Self::TrigNeedsAngle { op, found } => {
                write!(f, "`{op}` needs an Angle operand, got {found:?}")
            }
            Self::CountNeedsExplicitPromotion { op } => write!(
                f,
                "`{op}` on a Count needs an explicit promotion (use count_to_scalar)"
            ),
            Self::NotCount { found } => {
                write!(f, "a Count operand is required, got {found:?}")
            }
            Self::LiteralCountIsInteger => {
                f.write_str("a count literal must be an integer (use Expr::count)")
            }
            Self::NonFiniteLiteral => f.write_str("a literal value must be finite"),
            Self::DisplayUnitMismatch { unit, literal } => write!(
                f,
                "the display unit measures {unit:?} but the literal is {literal:?}"
            ),
            Self::UnknownDisplayUnit { symbol } => {
                write!(f, "unknown display unit {symbol:?}")
            }
        }
    }
}

impl core::error::Error for DimensionError {}

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

/// The stored display-unit CODE — quantity's closed table as a one-
/// byte identity (the spec's "U8a's unit type/code" read at its word:
/// storing the 32-byte [`quantity::UnitDef`] row inline grew every
/// `Expr` by ~40 bytes and tripped `large_enum_variant` on
/// `DocEdit::InsertNode`; the ROW is derivable from the identity, so
/// the identity is what is stored — resolved back through
/// [`Lit::unit_def`] at every read. That measurement is pinned by the
/// `size_of::<Lit>()` assertion below.)
///
/// The identity is the row's POSITION in [`quantity::UNITS`], not a
/// second spelling of the table's units: no unit symbol is written as CODE
/// anywhere in this crate's `src`, and both directions here go
/// through the table, so there is no mirror to hand-sync.
///
/// What that buys, exactly — the promise is narrower than "no edit
/// anywhere", and this crate's own tests say so. Re-checked against the
/// suites as they stand:
///
/// * **Reordered** in `quantity`: no edit at all, in `src` or in the
///   suites. Nothing here holds an opinion about the order —
///   `switch_display_units.rs`'s wire golden deliberately compares
///   membership as a SET so that stays true. (It is not silent either:
///   `quantity`'s own suite pins every symbol IN ORDER, so a reorder
///   is a decision taken there rather than a surprise here.)
/// * **Added**: no edit to `src`. In the suites,
///   `switch_display_units.rs`'s wire golden goes red — deliberately,
///   so a new unit cannot land unpinned. `tests/u8a_parse.rs`'s two
///   proptest generators enumerate the symbols by hand and do NOT
///   go red; they silently under-cover, so they want an edit that
///   nothing announces.
/// * **Renamed**: no edit to `src`. `u8a_parse.rs`'s generators go red
///   (the old symbol stops parsing), and so do
///   `switch_display_units.rs`'s golden and its `table_row` fixtures.
///
/// The index carries **no compatibility contract**: it is never
/// persisted (the wire stores the SYMBOL — `persist::wire`), never
/// enters expression identity, keys or [`Expr::literal_bits`] (D7),
/// and is minted afresh at every construction and load.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct UnitSym(u8);

// The code is one byte, so the table it indexes must fit in one. Six
// rows today; a table that grew past 255 rows fails the BUILD here.
// Nothing would truncate without it — `from_def`'s `u8::try_from`
// refuses — but that refusal is an `unreachable!`, i.e. a crash rather
// than a wrong answer. Failing the BUILD is the only answer that is
// neither.
const _: () = assert!(
    quantity::UNITS.len() <= u8::MAX as usize,
    "the display-unit code is one byte: this table has outgrown its code space"
);

impl UnitSym {
    /// The table row this code names.
    fn def(self) -> quantity::UnitDef {
        // Total: the index is minted only by `from_def`, as a position
        // in the very table indexed here, and the field is private to
        // this module, so out of range is unconstructable — this is
        // not `Span`'s shape (S14), where a `pub` type with a `pub`
        // constructor made the invalid state reachable by misuse.
        // Out of range would therefore be a kernel bug observable in a
        // branch: D2 addendum row 4, `unreachable!` with a message
        // rather than `index out of bounds: the len is 6 ...`.
        let Some(row) = quantity::UNITS.get(usize::from(self.0)) else {
            unreachable!(
                "display-unit code {} is not a row of quantity::UNITS ({} rows), yet the \
                 code is minted only by `from_def`, as a position in this very table, and \
                 `UnitSym`'s field is private to this module",
                self.0,
                quantity::UNITS.len()
            )
        };
        *row
    }

    /// The code for a table row, by symbol — TOTAL, exactly as
    /// [`Self::def`] is total in the other direction.
    ///
    /// **Symbol-keyed is sufficient because the symbol DETERMINES the
    /// row (issue #650, closed structurally).** Every `UnitDef` a
    /// caller can hold is a COPY OF A TABLE ROW — the seal, and why no
    /// whole-row re-check was added here, are stated once on
    /// [`quantity::UnitDef`]'s rustdoc. So matching on `symbol` alone
    /// selects the row the caller already had, and it always finds one.
    ///
    /// Both impossible branches take D2 addendum row 4, the same answer
    /// [`Self::def`] takes for its unconstructable index: a check for a
    /// state the type system excludes is dead code pretending to be a
    /// guard, so the state is announced as a kernel bug rather than
    /// carried as a typed refusal a caller could believe in.
    ///
    /// [`DimensionError::UnknownDisplayUnit`] is NOT dead — it is
    /// raised by `persist::wire`, where a display-unit SYMBOL arrives
    /// as a string out of a file and `quantity::unit_by_symbol` really
    /// can fail. That is the one reachable, input-driven off-table
    /// case, and it keeps its typed refusal (D2 addendum row 1). What
    /// went away is the CONSTRUCTION site of that variant, which could
    /// only fire for a `UnitDef` no caller can build.
    fn from_def(u: &quantity::UnitDef) -> Self {
        let Some(i) = quantity::UNITS
            .iter()
            .position(|row| row.symbol() == u.symbol())
        else {
            unreachable!(
                "display unit {:?} is not a row of quantity::UNITS ({} rows), yet UnitDef is \
                 sealed and every row a caller can hold is a copy of one",
                u.symbol(),
                quantity::UNITS.len()
            )
        };
        // The const assertion above bounds the table at 255 rows, so
        // this conversion cannot refuse either — same row 4.
        let Ok(code) = u8::try_from(i) else {
            unreachable!(
                "quantity::UNITS is pinned to at most u8::MAX rows, yet row {i} has no one-byte code"
            )
        };
        Self(code)
    }
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
    /// The display unit the literal was authored in, if any (a code
    /// into quantity's closed table; `None` renders canonically).
    pub(crate) display_unit: Option<UnitSym>,
}

// PR #291 MAJOR-2, as a compile-time row rather than a remembered
// measurement: `Lit` stores the one-byte CODE, never the row.
// Inlining `quantity::UnitDef` (32 bytes) into this struct took it to
// 40 and grew every `Expr` with it, tripping `large_enum_variant` on
// `DocEdit::InsertNode`.
//
// What this pin adds, stated precisely: the ORIGINAL detector is
// still armed — `large_enum_variant` is default-on in clippy's `perf`
// group and CI runs `cargo clippy --workspace --all-targets -D
// warnings` — so a repeat was never entirely unguarded. What is
// unguarded is the MARGIN. Whether a regrowth re-crosses that lint's
// 200-byte threshold depends on `DocEdit`'s size, which nothing
// tracks, and `Lit` is a struct, so it trips no enum lint on its own.
// This assertion moves the guard onto the thing that actually
// regressed, and makes it exact rather than threshold-dependent.
//
// It pins the PADDED size, and claims no more: re-inlining the row
// goes red here, and so does any growth past 16 bytes, but the six
// padding bytes beside the one-byte code are free (adding a
// `[u8; 6]` field here still compiles).
//
// (`Expr` itself is not pinned: its size is the largest `ExprKind`
// variant and moves for unrelated reasons. `Lit` is where the
// regression would enter.)
const _: () = assert!(
    core::mem::size_of::<Lit>() == 16,
    "a literal is one f64 plus a one-byte display-unit code"
);

impl Lit {
    /// The stored unit's table row, if any.
    pub(crate) fn unit_def(&self) -> Option<quantity::UnitDef> {
        self.display_unit.map(UnitSym::def)
    }
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

/// The two-operand [`ExprKind`] variants, as a PATTERN taking the two
/// operand sub-patterns.
///
/// Four matches partition `ExprKind` by arity — `Expr::child`'s two
/// arms, `param_refs` and `literal_bits` — and each wrote the same
/// seven names out. Sharing them as patterns keeps every one of those
/// matches exhaustive: a new variant absent from this macro breaks all
/// four builds, and its arity is one decision at one site.
macro_rules! binary_kind {
    ($a:pat, $b:pat) => {
        ExprKind::Add($a, $b)
            | ExprKind::Sub($a, $b)
            | ExprKind::Mul($a, $b)
            | ExprKind::Div($a, $b)
            | ExprKind::Atan2($a, $b)
            | ExprKind::Min($a, $b)
            | ExprKind::Max($a, $b)
    };
}

/// The one-operand [`ExprKind`] variants — see [`binary_kind`].
macro_rules! unary_kind {
    ($a:pat) => {
        ExprKind::Neg($a)
            | ExprKind::Sin($a)
            | ExprKind::Cos($a)
            | ExprKind::Tan($a)
            | ExprKind::CountToScalar($a)
    };
}

/// The zero-operand (leaf) [`ExprKind`] variants — see [`binary_kind`].
macro_rules! leaf_kind {
    () => {
        ExprKind::Literal(_) | ExprKind::CountLiteral(_) | ExprKind::Param(_)
    };
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
        let unit_dim = match unit.quantity() {
            quantity::UnitQuantity::Length => Dimension::Length,
            quantity::UnitQuantity::Angle => Dimension::Angle,
        };
        if unit_dim != dim {
            return Err(DimensionError::DisplayUnitMismatch {
                unit: unit_dim,
                literal: dim,
            });
        }
        // Total since the #650 seal: a `UnitDef` is a table row, so
        // it has a code (see `UnitSym::from_def`).
        let sym = UnitSym::from_def(&unit);
        // Run literal()'s refusal doors, then attach the unit.
        let mut e = Self::literal(value, dim)?;
        if let ExprKind::Literal(ref mut lit) = e.kind {
            lit.display_unit = Some(sym);
        }
        Ok(e)
    }

    /// The display unit of a LITERAL expression, if one is stored
    /// (`None` for every other kind and for canonically-authored
    /// literals). The formatter's read side (§4g).
    pub fn display_unit(&self) -> Option<quantity::UnitDef> {
        match &self.kind {
            ExprKind::Literal(lit) => lit.unit_def(),
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
        match (&self.kind, i) {
            (binary_kind!(a, _), 0) | (unary_kind!(a), 0) => Some(a),
            (binary_kind!(_, b), 1) => Some(b),
            // EXHAUSTIVE on the KIND axis, open on the index axis: a
            // new variant must be given an arity here or the compile
            // breaks. A wildcard would give it arity ZERO silently, and
            // `descend`/`ExprPath` would then walk off the tree at a
            // node that does have children.
            (binary_kind!(_, _) | unary_kind!(_) | leaf_kind!(), _) => None,
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
        match &self.kind {
            ExprKind::Param(name) => out.push((name.clone(), self.dim)),
            ExprKind::Literal(_) | ExprKind::CountLiteral(_) => {}
            unary_kind!(a) => a.param_refs(out),
            binary_kind!(a, b) => {
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
        match &self.kind {
            ExprKind::Literal(lit) => out.push(lit.value.to_bits()),
            ExprKind::CountLiteral(_) | ExprKind::Param(_) => {}
            unary_kind!(a) => a.literal_bits(out),
            binary_kind!(a, b) => {
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
            // EXHAUSTIVE on the KIND axis (the `child` rule): a new
            // variant must be given a rebuild here or the compile
            // breaks. A wildcard would refuse to rebuild it — an edit
            // to a valid path silently reported as off-tree.
            (binary_kind!(_, _) | unary_kind!(_) | leaf_kind!(), _) => return None,
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

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM and, where the fault has a lever, its one recourse. The
// enum stays the machine contract; composing layers (the evaluation
// service's `Expr` slot arm) FORWARD this rendering rather than
// re-stating it.
impl core::fmt::Display for EvalError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownParam(name) => write!(
                f,
                "parameter {:?} has no binding in the evaluation environment — declare \
                 the document parameter or fix the reference",
                name.0
            ),
            Self::ParamDimensionMismatch {
                name,
                expected,
                found,
            } => write!(
                f,
                "parameter {:?} is referenced as {expected:?} but bound as {found:?}",
                name.0
            ),
            Self::CountExprInContinuousEval => f.write_str(
                "a Count expression does not evaluate continuously — promote it \
                 explicitly through count_to_scalar",
            ),
            Self::ContinuousExprInCountEval { found } => write!(
                f,
                "a {found:?} expression does not evaluate as a Count — counts are exact \
                 and never inferred from a continuous value"
            ),
            Self::CountOverflow => {
                f.write_str("exact Count arithmetic overflowed (a count never wraps)")
            }
            Self::CountToScalarOutOfRange(count) => write!(
                f,
                "count {count} is outside the exactly-promotable range — a promoted \
                 count must fit i32 so its f64 embedding is exact"
            ),
            Self::NonFiniteResult => f.write_str(
                "the evaluated result is not finite — the arithmetic overflowed or hit \
                 a pole (1/0, 0/0); fix the expression or the values feeding it",
            ),
        }
    }
}

impl core::error::Error for EvalError {}

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
    refuse_non_finite(eval_inner(expr, params)?)
}

/// **Door 2, as a shared door.** The ruled non-finite check on a
/// FINAL evaluated value: `value * 0` is EXACTLY zero for every finite
/// value and poison (NaN / empty / Trv) otherwise, so any valid band
/// classifies it identically — Zero passes, everything else is a
/// non-finite result.
///
/// It lives apart from [`eval`] because [`eval`] is not the only
/// evaluator of this crate's expression arithmetic: the measurement
/// sublanguage ([`crate::measure`]) evaluates its own tree, and a
/// second copy of this door would be a second chance to forget it —
/// which is exactly what happened, and what shipped a
/// `Holds { measured: inf }` verdict. Any evaluator that produces a
/// value a caller will believe passes it through here.
///
/// Band construction with these constants cannot fail; the `else` arm
/// is unreachable but typed (no panic paths in this crate).
pub(crate) fn refuse_non_finite<T: Decide>(value: T) -> Result<T, EvalError> {
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
