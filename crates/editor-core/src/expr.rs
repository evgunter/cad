//! The expression sublanguage v1 (spec D4, ratified forks F1 + F7).
//!
//! A small typed AST: variable readers, exact constants (rationals,
//! integers and `turn`), arithmetic, trig, min/max. The stored form
//! holds no float (VARIABLES-DESIGN VR5): a written value is a variable. **No conditionals, no iteration, no
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
//! One tree serves two forms: the stored [`Expr`] reads every variable
//! by id, and the authored [`crate::Formula`] may also write a name,
//! which the edit door lowers to an id (VR6).
//!
//! Units erase at the evaluation boundary (GQ5): [`eval`] returns raw
//! `T` in kernel units (meters/radians); display units are document
//! presentation metadata, not this layer's concern.

use geom_core::Real;
use geom_core::predicate::{Band, Decide, Sign};

use crate::doc::VarName;
use crate::node::{RecipeNodeId, SlotId};
use crate::var::VarId;

pub use crate::ratio::Ratio;

/// The v1 quantity-dimension lattice (ratified F1, GQ5's banked
/// decision): four dimensions, no products of dimensions.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
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

// The one home of the dimension-in-prose rule for the crate. A
// dimension is a quantity KIND, not an address: it names what a value
// measures, so refusal prose renders it as the common noun a person
// would say ("slot needs a length expression, got an angle") and never
// as the variant identifier. That holds wherever a dimension reaches a
// user — `EditError`, `DimensionError`, `EvalError`, the persist
// checker's faults, the select refusals — and it holds for a dimension
// NAMED in a sentence as much as for one interpolated, so the words are
// lowercase on both sides. Two channels are outside it: a panic or
// assertion addressed to whoever debugs the kernel names the variant
// (the identifier is the thing to grep), and the wire/key encodings
// carry tags, not words.
//
// Two further spellings of this word list exist downstream and are
// deliberate: `pncad-py`'s `errors::dimension_tag` (the FFI tag —
// identical words, pinned equal to this rendering by that crate's
// `dimension_tags_match_the_kernel_prose`) and its `py::value::
// dimension_name` (capitalized, the Python `Measurement` repr). Each
// says so at its own site.
impl core::fmt::Display for Dimension {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Length => "length",
            Self::Angle => "angle",
            Self::Count => "count",
            Self::Scalar => "scalar",
        })
    }
}

impl Dimension {
    /// **Every dimension this enum names**, in declaration order — the
    /// one enumeration, owned where the exhaustive matches live.
    ///
    /// A list cannot be derived from a match in safe Rust, so SOMEONE
    /// writes it by hand; the only question is where. Written here, it
    /// sits in the crate whose exhaustive matches over `Dimension`
    /// (`Display` above, `Dimension::article` below, the checker's own
    /// arms) fail to compile on a dimension added to the lattice — so
    /// the author adding one is already on this page with the list in
    /// front of them, and the `all_is_every_dimension` census
    /// (`tests/m4_pr1_dims.rs`) puts a second visit right beside it.
    /// **Neither forces the edit**: what they force is that the author
    /// is here and has to decide, and the census's own doc measures how
    /// far short of forcing it stops. A copy in a downstream crate gets
    /// not even that. The enum is closed, so a consumer's own
    /// exhaustive match does fence THAT consumer; but nothing ties an
    /// array literal to a variant list, so a downstream list stays the
    /// length it was written at, with no error anywhere and no author
    /// standing over it.
    ///
    /// So this is the list downstream reads instead of writing its own
    /// — the viewer's new-parameter radio row draws one button per
    /// entry — and a consumer that RENDERS it renders declaration
    /// order, which nothing here ranks: this is the lattice's own
    /// order, not a recommendation.
    ///
    /// The words are not here and are not wanted here: a dimension
    /// reaching a user is the `Display` above, which is this crate's
    /// one home for that rule.
    pub const ALL: [Self; 4] = [Self::Length, Self::Angle, Self::Count, Self::Scalar];

    /// The indefinite article agreeing with the `Display` noun, for
    /// the sentence positions that need one. **The value decides it**
    /// — a sentence that hard-codes "a" is wrong for every value whose
    /// noun begins with a vowel, which is how "a angle" and "a edge"
    /// both reached refusal prose. So the sentence writes `"{} {dim}"`
    /// and the value supplies both halves. This is the crate's one
    /// idiom for a rendered kind's article; `EntityKind::article` is
    /// its twin, and covers a whole `StableName` phrase as well.
    pub(crate) fn article(self) -> &'static str {
        match self {
            Self::Length | Self::Count | Self::Scalar => "a",
            Self::Angle => "an",
        }
    }
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
    /// A written quantity constructed with [`Dimension::Count`] — a
    /// count is an integer, made by [`crate::Formula::count`].
    LiteralCountIsInteger,
    /// A non-finite (NaN/±inf) written quantity — refused at
    /// construction (the M4 PR 1 review's ruled "door 1": the kernel
    /// never produces non-finite values legitimately, so admitting one
    /// into recipe data would smuggle poison past every downstream
    /// check).
    NonFiniteLiteral,
    /// A written quantity's display unit measures a different quantity
    /// than its dimension (`mm` can only suffix a `Length`; `deg` only
    /// an `Angle`; a `Scalar` takes no unit). LIB-SWITCH §4g: the
    /// display unit is presentation metadata, but a MISMATCHED one is
    /// corrupt data, refused at construction like every other dimension
    /// fault.
    DisplayUnitMismatch {
        /// The dimension the unit's quantity implies.
        unit: Dimension,
        /// The quantity's declared dimension.
        literal: Dimension,
    },
    /// A rational constant with a zero denominator, or whose reduced
    /// numerator or denominator exceeds 2^53 ([`Ratio`]): both
    /// operands of its one division must be exact doubles.
    ConstantOutOfRange {
        /// The constant as written.
        text: String,
    },
    /// A persisted rational constant that is not in lowest terms: the
    /// load door reads the one spelling each constant has.
    RatioNotReduced {
        /// The numerator as written.
        num: i64,
        /// The denominator as written.
        den: u64,
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
    /// The expression would nest deeper than an expression may
    /// (`expr::MAX_NESTING`): every door that mints an expression
    /// refuses it, so every walk over one fits the smallest stack a
    /// door runs on.
    NestedTooDeep {
        /// The deepest an expression may nest, in levels.
        bound: usize,
    },
}

// LIB-DOORS F6 (reopened on review): a human-readable rendering. The
// comment-style rule applies — each arm states the PROBLEM, not the
// enum's guts; the enum itself remains the machine contract.
impl core::fmt::Display for DimensionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Mismatch { op, left, right } => {
                write!(f, "cannot apply `{op}` to {left} and {right}")
            }
            Self::MulNeedsScalar { left, right } => write!(
                f,
                "multiplication needs a scalar operand ({left} x {right} would change dimension)"
            ),
            Self::DivNeedsScalarDivisor { left, right } => write!(
                f,
                "division needs a scalar divisor ({left} / {right} is refused in v1)"
            ),
            Self::TrigNeedsAngle { op, found } => {
                write!(
                    f,
                    "`{op}` needs an angle operand, got {} {found}",
                    found.article()
                )
            }
            Self::CountNeedsExplicitPromotion { op } => write!(
                f,
                "`{op}` on a count needs an explicit promotion (use count_to_scalar)"
            ),
            Self::NotCount { found } => {
                write!(
                    f,
                    "a count operand is required, got {} {found}",
                    found.article()
                )
            }
            Self::LiteralCountIsInteger => {
                f.write_str("a count literal must be an integer (use Formula::count)")
            }
            Self::NonFiniteLiteral => f.write_str("a literal value must be finite"),
            Self::DisplayUnitMismatch { unit, literal } => write!(
                f,
                "the display unit measures {unit} but the literal is {literal}"
            ),
            Self::ConstantOutOfRange { text } => write!(
                f,
                "the constant {text} is out of range: a constant is a ratio of integers of \
                 at most 2^53, with a non-zero denominator"
            ),
            Self::RatioNotReduced { num, den } => {
                write!(f, "the constant {num}/{den} is not in lowest terms")
            }
            Self::UnknownDisplayUnit { symbol } => {
                write!(f, "unknown display unit {symbol:?}")
            }
            Self::NestedTooDeep { bound } => write!(
                f,
                "the expression nests deeper than {bound} levels. Recourse: regroup it to \
                 nest less; `(a + b) + (c + d)` nests one level less than `a + b + c + d`"
            ),
        }
    }
}

impl core::error::Error for DimensionError {}

/// **Why a name leaf did not lower** ([`crate::Formula::lower`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unlowered {
    /// No variable holds the name.
    Unheld,
    /// The variable holding the name reads at `declared`, not at the
    /// dimension the leaf reads it at.
    Kind {
        /// The variable holding the name.
        var: VarId,
        /// The dimension its kind reads at.
        declared: Dimension,
    },
}

/// A dimension-checked expression tree (ratified F7 shape).
///
/// Construction goes through the smart constructors below, which run
/// the F1 dimension checker and the nesting bound; the fields are
/// private so an ill-dimensioned or over-deep tree cannot be built.
/// The cached [`Self::dim`] is therefore trustworthy by construction.
///
/// **An expression nests at most 128 levels** along its longest chain
/// from the root to a leaf, and a constructor that would pass that
/// refuses with [`DimensionError::NestedTooDeep`]. The operators
/// associate to the left, so a flat chain of more than 128 terms
/// (`a + b + …`) refuses; grouped (`(a + b) + (c + d)`), the same terms
/// nest less.
///
/// **Two forms share the tree**, and differ only in the leaves `L`
/// adds to the shared ones: [`Expr`] is the stored form and adds none,
/// [`crate::Formula`] is the authored form and adds the leaves only an
/// author writes (VARIABLES-DESIGN VR6). The edit door lowers the one
/// to the other, so a stored document cannot hold an authored leaf.
#[derive(Clone, PartialEq)]
pub struct ExprTree<L: LeafSet> {
    dim: Dimension,
    /// How many levels the tree nests, this node included (a leaf is
    /// 1); never above [`MAX_NESTING`].
    nesting: u8,
    kind: ExprKind<L>,
}

/// The stored expression: no leaf beyond the shared ones, so every
/// variable it reads is read by id.
pub type Expr = ExprTree<StoredLeaf>;

impl<L: LeafSet> core::fmt::Debug for ExprTree<L> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { dim, nesting, kind } = self;
        f.debug_struct(L::FORM)
            .field("dim", dim)
            .field("nesting", nesting)
            .field("kind", kind)
            .finish()
    }
}

mod sealed {
    pub trait Sealed {}
}

/// **The leaves one form of [`ExprTree`] adds** to the shared ones —
/// sealed: the forms are [`Expr`] and [`crate::Formula`], and no other.
pub trait LeafSet: Clone + core::fmt::Debug + PartialEq + sealed::Sealed {
    /// The form's type name, as `Debug` writes it.
    const FORM: &'static str;
    /// The text of one such leaf, read at `dim` ([`unparse`]).
    fn write(&self, dim: Dimension, out: &mut String);
    /// Pushes the bits of every float the leaf holds
    /// ([`ExprTree::literal_bits`]).
    fn bits(&self, out: &mut Vec<u64>);
    /// Whether the leaf's text opens with a minus sign, so it binds as
    /// a negation does ([`unparse`]).
    fn negative(&self) -> bool;
    /// Whether the leaf's text is a number, whose sign a minus written
    /// before it is read as ([`unparse`]).
    fn numeric(&self) -> bool;
    /// The leaf as an author writes it.
    fn authored(&self) -> AuthoredLeaf;
    /// The leaf's value, read at the continuous dimension `dim`.
    ///
    /// # Errors
    ///
    /// A leaf only the edit door resolves ([`EvalError::Unlowered`]).
    fn value<T: Real>(&self, dim: Dimension) -> Result<T, EvalError>;
    /// The leaf's value as a count.
    ///
    /// # Errors
    ///
    /// A leaf only the edit door resolves ([`EvalError::Unlowered`]), or
    /// a continuous one.
    fn count(&self, dim: Dimension) -> Result<i64, EvalError>;
}

/// **What a node's slot holds**, in either form: the stored
/// [`VarId`] (VARIABLES-DESIGN VR4), or the authored [`crate::Formula`]
/// an edit carries. Sealed, as [`LeafSet`] is; the node, the program
/// and the measure are generic over it, so one declaration serves the
/// form a door is handed and the form the document stores. A stored
/// slot carries no dimension of its own: its address fixes the one it
/// is read at, and its variable's kind the one it holds.
pub trait Slot:
    Clone
    + core::fmt::Debug
    + PartialEq
    + serde::Serialize
    + for<'de> serde::Deserialize<'de>
    + sealed::Sealed
{
    /// The variables it reads by id, in pre-order.
    fn var_ids(&self, out: &mut Vec<VarId>);
    /// How many levels the value nests, itself included: a stored
    /// slot is one leaf.
    fn nesting(&self) -> usize;
    /// Bit-semantic equality (D7): `PartialEq`, with every float the
    /// value holds compared by its bits.
    fn bit_eq(&self, other: &Self) -> bool;
}

impl<L: LeafSet> sealed::Sealed for ExprTree<L> {}

impl<L: LeafSet> Slot for ExprTree<L>
where
    Self: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    fn var_ids(&self, out: &mut Vec<VarId>) {
        let mut reads = Vec::new();
        ExprTree::var_reads(self, &mut reads);
        out.extend(reads.into_iter().map(|(var, _)| var));
    }
    fn nesting(&self) -> usize {
        usize::from(self.nesting)
    }
    fn bit_eq(&self, other: &Self) -> bool {
        ExprTree::bit_eq(self, other)
    }
}

impl sealed::Sealed for VarId {}

impl Slot for VarId {
    fn var_ids(&self, out: &mut Vec<VarId>) {
        out.push(*self);
    }
    fn nesting(&self) -> usize {
        1
    }
    fn bit_eq(&self, other: &Self) -> bool {
        self == other
    }
}

/// The stored form's own leaves: there are none, so a stored tree
/// holds only the shared ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredLeaf {}

impl sealed::Sealed for StoredLeaf {}

impl LeafSet for StoredLeaf {
    const FORM: &'static str = "Expr";
    fn write(&self, _dim: Dimension, _out: &mut String) {
        match *self {}
    }
    fn bits(&self, _out: &mut Vec<u64>) {
        match *self {}
    }
    fn negative(&self) -> bool {
        match *self {}
    }
    fn numeric(&self) -> bool {
        match *self {}
    }
    fn authored(&self) -> AuthoredLeaf {
        match *self {}
    }
    fn value<T: Real>(&self, _dim: Dimension) -> Result<T, EvalError> {
        match *self {}
    }
    fn count(&self, _dim: Dimension) -> Result<i64, EvalError> {
        match *self {}
    }
}

/// **A written quantity** (VARIABLES-DESIGN VR6): a value in canonical
/// kernel units, the unit it was written in, and an optional
/// distribution. The dimension is the tree's, at the leaf. The edit
/// door mints an anonymous free variable holding it, so a document
/// holds it as that variable and never as a float in an expression.
#[derive(Debug, Clone, Copy)]
pub struct Quantity {
    /// The exact canonical-units value.
    pub(crate) value: f64,
    /// The unit it was written in (presentation, D6).
    pub(crate) unit: UnitSym,
    /// Its uncertainty, if the author wrote one.
    pub(crate) distribution: Option<crate::distribution::Distribution>,
}

impl Quantity {
    /// The exact canonical-units value.
    #[must_use]
    pub fn value(&self) -> f64 {
        self.value
    }

    /// The unit it was written in.
    #[must_use]
    pub fn unit(&self) -> quantity::UnitDef {
        self.unit.def()
    }

    /// Its distribution, if one was written.
    #[must_use]
    pub fn distribution(&self) -> Option<&crate::distribution::Distribution> {
        self.distribution.as_ref()
    }

    /// The free variable the edit door mints for it, read at `dim`.
    pub(crate) fn free_var(&self, dim: Dimension) -> crate::doc::FreeVar {
        crate::doc::FreeVar::Continuous {
            dim,
            value: self.value,
            display_unit: self.unit,
            distribution: self.distribution,
        }
    }
}

impl PartialEq for Quantity {
    /// IEEE-semantic on the value and the distribution; the display
    /// unit is presentation metadata and never part of a formula's
    /// identity (DESIGN.md D6).
    fn eq(&self, other: &Self) -> bool {
        let Self {
            value,
            unit: _,
            distribution,
        } = self;
        *value == other.value && *distribution == other.distribution
    }
}

/// The authored form's own leaves: what an author writes and the edit
/// door resolves.
#[derive(Debug, Clone, PartialEq)]
pub enum AuthoredLeaf {
    /// A variable by NAME, which the edit door lowers to a reader of
    /// the variable the document names so.
    Name(VarName),
    /// Entry `i` of the edit's fresh table: a variable the edit mints,
    /// which the door lowers to a reader of the id it minted.
    Fresh(u16),
    /// A written quantity, which the door lowers to a reader of the
    /// anonymous free variable it mints for it (boxed, so a leaf costs
    /// what a name costs).
    Quantity(Box<Quantity>),
}

impl sealed::Sealed for AuthoredLeaf {}

impl LeafSet for AuthoredLeaf {
    const FORM: &'static str = "Formula";
    fn write(&self, dim: Dimension, out: &mut String) {
        match self {
            Self::Name(name) => out.push_str(name.as_str()),
            Self::Fresh(index) => {
                use core::fmt::Write as _;
                let _ = write!(out, "fresh[{index}]");
            }
            Self::Quantity(q) => out.push_str(&write_quantity(q.value, q.unit, dim)),
        }
    }
    fn bits(&self, out: &mut Vec<u64>) {
        if let Self::Quantity(q) = self {
            out.push(q.value.to_bits());
        }
    }
    fn negative(&self) -> bool {
        matches!(self, Self::Quantity(q) if q.value.is_sign_negative())
    }
    fn numeric(&self) -> bool {
        matches!(self, Self::Quantity(_))
    }
    fn authored(&self) -> AuthoredLeaf {
        self.clone()
    }
    fn value<T: Real>(&self, dim: Dimension) -> Result<T, EvalError> {
        match unlowered(self, dim) {
            Ok(fault) => Err(EvalError::Unlowered(fault)),
            Err(q) => Ok(T::from_f64(q.value)),
        }
    }
    fn count(&self, dim: Dimension) -> Result<i64, EvalError> {
        match unlowered(self, dim) {
            Ok(fault) => Err(EvalError::Unlowered(fault)),
            Err(_) => Err(EvalError::ContinuousExprInCountEval { found: dim }),
        }
    }
}

/// Why an authored leaf does not lower outside a document and an
/// edit: a name held by nothing, a fresh entry in no table.
fn unlowered(leaf: &AuthoredLeaf, dim: Dimension) -> Result<crate::LowerFault, Quantity> {
    match leaf {
        AuthoredLeaf::Name(name) => Ok(crate::LowerFault::Name(crate::NameFault {
            name: name.clone(),
            dim,
            why: Unlowered::Unheld,
        })),
        &AuthoredLeaf::Fresh(index) => Ok(crate::LowerFault::Fresh(crate::FreshFault {
            index,
            dim,
            held: None,
        })),
        AuthoredLeaf::Quantity(q) => Err(**q),
    }
}

/// **One leaf of a tree**, as [`ExprTree::visit_terminals`] hands it.
pub(crate) enum Terminal<'a, L> {
    /// A reader of a variable, by id.
    Var(VarId),
    /// A leaf of the form's own set.
    Leaf(&'a L),
    /// A constant.
    Constant,
}

/// **How deep an expression may nest**: the longest chain of nodes
/// from the root to a leaf, both ends included. The one bound on an
/// [`Expr`] and on a [`crate::MeasureExpr`], read by every door that
/// mints one: the smart constructors refuse past it
/// ([`DimensionError::NestedTooDeep`]), the text parser builds through
/// them, and the load door refuses a file nested deeper than the bound
/// lets a saved one be (`persist::nesting`).
///
/// It exists so that every walk over an expression that recurses once
/// per level fits the smallest stack a door runs on: serde's in the
/// save, load and content-pin doors, the derived impls, and this
/// crate's own readers of the tree. The evaluators, the parser and
/// `Drop` keep their own stack ([`crate::tree`]). A long expression
/// regroups under it: `a + b + c + d` nests four levels and
/// `(a + b) + (c + d)` three, so the bound limits how a long sum is
/// grouped, not how many terms it has — but a flat chain of more than
/// 128 terms refuses.
pub(crate) const MAX_NESTING: usize = 128;

const _: () = assert!(
    MAX_NESTING <= u8::MAX as usize,
    "an expression's nesting is stored in one byte"
);

/// The nesting of an operator node whose deepest child nests `below`
/// levels, refused past [`MAX_NESTING`]: the one check both expression
/// languages' constructors make.
pub(crate) fn nesting_over(below: u8) -> Result<u8, DimensionError> {
    match below.checked_add(1) {
        Some(nesting) if usize::from(nesting) <= MAX_NESTING => Ok(nesting),
        _ => Err(DimensionError::NestedTooDeep { bound: MAX_NESTING }),
    }
}

/// The stored display-unit CODE — quantity's closed table as a one-
/// byte identity: the 32-byte [`quantity::UnitDef`] row is derivable
/// from the identity, so the identity is what is stored and resolved
/// back through [`UnitSym::def`] at every read.
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
/// persisted, never enters a formula's identity, keys or
/// [`ExprTree::literal_bits`] (D7), and is minted afresh at every
/// construction and load.
///
/// **Serialization goes through the SYMBOL, never the index**, and the
/// impls below are what keep that true now that the type is public and
/// two carriers store it: `persist::wire`'s `WireFormula::Quantity`
/// writes the symbol as its own field, and [`crate::FreeVar::Continuous`]
/// writes one through this type's `Serialize`. Both read back through
/// [`quantity::unit_by_symbol`], so a table REORDER still moves no
/// byte of any file.
///
/// Public because a document parameter's declaration carries one
/// ([`crate::FreeVar::Continuous`]'s `display_unit`) and an enum
/// variant's fields are public with it. The FIELD stays private to
/// this module, which is what every totality argument above rests on —
/// nothing about the seal depended on the type's visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitSym(u8);

impl serde::Serialize for UnitSym {
    /// As the row's surface SYMBOL — the wire spelling, for the reason
    /// the rustdoc above gives.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.def().symbol())
    }
}

impl<'de> serde::Deserialize<'de> for UnitSym {
    /// Back through [`quantity::unit_by_symbol`], the same closed-table
    /// lookup `persist::wire`'s rebuild runs.
    ///
    /// An off-table symbol refuses HERE, at the token, because that is
    /// the only fact this layer can see; whether the unit agrees with
    /// the DIMENSION it was stored beside is a document invariant, and
    /// it is checked where document invariants are, in the shared
    /// save/load validator (`persist::check::first_display_unit_fault`)
    /// — typed, and symmetric across both doors rather than load-only.
    ///
    /// The refusal is the SAME fact `persist::wire`'s rebuild raises
    /// for an off-table symbol on an expression literal, so it leaves
    /// by the same channel and reaches a caller as the same
    /// `PersistError::Dimension`. Otherwise one fault would cross one
    /// door under two classes with contradictory recourse: this route
    /// used to answer "regenerate the file from its source recipe",
    /// which is advice that reproduces the refusal.
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let symbol = <String as serde::Deserialize>::deserialize(deserializer)?;
        match quantity::unit_by_symbol(&symbol) {
            Some(row) => Ok(Self::from_def(&row)),
            None => {
                // The typed refusal leaves through the frame; the serde
                // message is the human half of the same fact
                // (`persist::refusal`, which lists this recorder).
                crate::persist::refusal::record(&DimensionError::UnknownDisplayUnit {
                    symbol: symbol.clone(),
                });
                Err(serde::de::Error::custom(format!(
                    "display unit {symbol:?} is not one of the {} rows quantity::UNITS carries",
                    quantity::UNITS.len()
                )))
            }
        }
    }
}

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
    pub fn def(self) -> quantity::UnitDef {
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

    /// The dimension this unit MEASURES — the other half of the
    /// pairing [`Self::canonical_for`] makes, read in the opposite
    /// direction.
    ///
    /// **The one place that reading is spelled**, and every caller that
    /// needs it asks here rather than re-laddering it: the expression
    /// TEXT door (`parse`, on a suffix), [`crate::Formula::literal_with_unit`] at
    /// construction, [`crate::FreeVar::with_display_unit`] at the
    /// variable's notation door, the declare and define doors' definition
    /// check (`edit.rs`'s `check_var_def`), and the save/load
    /// validator's variable walk (`persist::check`). Callers restating one `match` are that
    /// many chances for them to disagree about what `mm` measures —
    /// and the parser's copy was worse than a duplicate, because the
    /// dimension it derived was then handed to a door that derives the
    /// same thing from the same unit to check the two against each
    /// other.
    ///
    /// Total: the table's quantity column has three rows and
    /// [`Dimension`] has a variant for each. `Count` is not among them
    /// — a count is an integer and names no notation — which is why a
    /// `Count` parameter has no unit door rather than a unit that
    /// measures counts.
    pub fn measures(self) -> Dimension {
        match self.def().quantity() {
            quantity::UnitQuantity::Length => Dimension::Length,
            quantity::UnitQuantity::Angle => Dimension::Angle,
            quantity::UnitQuantity::Scalar => Dimension::Scalar,
        }
    }

    /// The unit a value of `dim` is written in when nothing else was
    /// authored: metres, radians, or the dimensionless row.
    ///
    /// This is where "canonical" stops being a reader's guess and
    /// becomes a stored fact.
    ///
    /// **Total, including `Count`** — deliberately, though a count has
    /// no notation and the table no row for one. [`crate::Formula::literal`]
    /// refuses `Count` before ever reaching here, so no LITERAL takes
    /// that arm; what can is [`crate::FreeVar::continuous`], whose
    /// `dim` is a caller's argument and whose `Count` spelling is a
    /// corrupt parameter the edit door refuses typed
    /// (`EditError::ContinuousVarCannotBeCount`). Panicking here
    /// would replace that typed refusal with a crash on the way to it,
    /// which is the wrong trade: the answer is the dimensionless row,
    /// nothing ever renders it (a count is an integer), and the
    /// refusal still happens where it always did.
    pub fn canonical_for(dim: Dimension) -> Self {
        let row = match dim {
            Dimension::Length => quantity::M.def(),
            Dimension::Angle => quantity::RAD.def(),
            Dimension::Scalar | Dimension::Count => quantity::ONE.def(),
        };
        Self::from_def(&row)
    }

    /// The symbol for an AUTHORED unit on a value of dimension `dim`.
    ///
    /// The one home of "a unit measures what its value holds". Every
    /// door that attaches a notation a caller CHOSE — the literal
    /// constructor [`crate::Formula::literal_with_unit`] and
    /// [`crate::RecordedNotation::set`], which writes one down before
    /// any literal exists — asks this, so the two cannot come to
    /// disagree about which pairings are legal. [`Self::canonical_for`]
    /// is the same relation in the other direction: the unit a
    /// dimension picks when nobody chose one.
    ///
    /// # Errors
    ///
    /// [`DimensionError::DisplayUnitMismatch`] when the unit's quantity
    /// is not `dim` (a `mm` on a bulge).
    pub(crate) fn checked_for(
        dim: Dimension,
        unit: quantity::UnitDef,
    ) -> Result<Self, DimensionError> {
        // Total since the #650 seal: a `UnitDef` is a table row, so
        // it has a code (see `UnitSym::from_def`).
        let sym = Self::from_def(&unit);
        let measured = sym.measures();
        if measured != dim {
            return Err(DimensionError::DisplayUnitMismatch {
                unit: measured,
                literal: dim,
            });
        }
        Ok(sym)
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
    pub fn from_def(u: &quantity::UnitDef) -> Self {
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

/// The two-operand [`ExprKind`] variants, as a PATTERN taking the two
/// operand sub-patterns.
///
/// Four matches partition `ExprKind` by arity — `Expr::child`'s two
/// arms, `var_reads` and `literal_bits` — and each wrote the same
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
        ExprKind::Ratio(_)
            | ExprKind::Integer(_)
            | ExprKind::Turn
            | ExprKind::Var(_)
            | ExprKind::Leaf(_)
    };
}

/// The node vocabulary of the AST (private: constructors check dims).
///
/// Child order (the ExprPath byte at each level, spec D5): operands in
/// argument order — 0 = first/only child, 1 = second.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExprKind<L: LeafSet> {
    /// An exact rational constant, dimension `Scalar` (VR5).
    Ratio(Ratio),
    /// An exact integer constant, dimension `Count` (spec D4: Count is
    /// integer-valued, never a float).
    Integer(i64),
    /// One full rotation, dimension `Angle`: the exact constant a right
    /// angle is a quarter of (VR5).
    Turn,
    /// A reader of a document variable, by identity: the stored form.
    /// The leaf's `dim` caches the variable's kind, which cannot change
    /// (VR3); the doors re-check the cache against the table.
    Var(VarId),
    /// A leaf of the form's own vocabulary ([`LeafSet`]).
    Leaf(L),
    /// Same-dimension addition.
    Add(Box<ExprTree<L>>, Box<ExprTree<L>>),
    /// Same-dimension subtraction.
    Sub(Box<ExprTree<L>>, Box<ExprTree<L>>),
    /// Negation (any dimension, including Count).
    Neg(Box<ExprTree<L>>),
    /// Product; ≥1 operand dimensionless (`Scalar`), or Count×Count.
    Mul(Box<ExprTree<L>>, Box<ExprTree<L>>),
    /// Quotient; the divisor must be `Scalar`.
    Div(Box<ExprTree<L>>, Box<ExprTree<L>>),
    /// Sine of an `Angle`, yielding `Scalar`.
    Sin(Box<ExprTree<L>>),
    /// Cosine of an `Angle`, yielding `Scalar`.
    Cos(Box<ExprTree<L>>),
    /// Tangent of an `Angle`, yielding `Scalar`.
    Tan(Box<ExprTree<L>>),
    /// Four-quadrant arctangent of same-dimension (y, x), yielding
    /// `Angle`.
    Atan2(Box<ExprTree<L>>, Box<ExprTree<L>>),
    /// Same-dimension lattice minimum (a value operation, never
    /// control flow — the AST has no branches; F7).
    Min(Box<ExprTree<L>>, Box<ExprTree<L>>),
    /// Same-dimension lattice maximum.
    Max(Box<ExprTree<L>>, Box<ExprTree<L>>),
    /// EXPLICIT Count→Scalar promotion (spec D4: never implicit).
    CountToScalar(Box<ExprTree<L>>),
}

impl<L: LeafSet> ExprKind<L> {
    /// Moves this node's children onto `out`, leaving a leaf behind.
    fn detach_children(&mut self, out: &mut Vec<ExprTree<L>>) {
        match core::mem::replace(self, ExprKind::Integer(0)) {
            binary_kind!(a, b) => {
                out.push(*a);
                out.push(*b);
            }
            unary_kind!(a) => out.push(*a),
            leaf_kind!() => {}
        }
    }
}

impl<L: LeafSet> Drop for ExprTree<L> {
    /// Frees the tree from a heap stack (`crate::tree::free`), so no
    /// drop recurses.
    fn drop(&mut self) {
        if !matches!(self.kind, leaf_kind!()) {
            crate::tree::free(self, |e, out| e.kind.detach_children(out));
        }
    }
}

// The arithmetic constructors share names with the std ops traits on
// purpose (they ARE the expression-level add/sub/…), but they cannot
// implement those traits: they are FALLIBLE (the F1 dimension checker
// runs at construction) and associated functions, not methods.
#[allow(clippy::should_implement_trait)]
impl<L: LeafSet> ExprTree<L> {
    /// This expression's dimension (cached; correct by construction).
    pub fn dim(&self) -> Dimension {
        self.dim
    }

    /// The AST node (persistence's wire conversion reads it; the type
    /// stays crate-private so trees are only built through the
    /// dimension-checking constructors).
    pub(crate) fn kind(&self) -> &ExprKind<L> {
        &self.kind
    }

    /// The AST node, to rewrite a leaf in place (a leaf's dimension
    /// and nesting do not move).
    pub(crate) fn kind_mut(&mut self) -> &mut ExprKind<L> {
        &mut self.kind
    }

    /// The variable this tree is, where it is one lone reader.
    pub fn as_var(&self) -> Option<VarId> {
        match self.kind {
            ExprKind::Var(var) => Some(var),
            _ => None,
        }
    }

    /// The constant `ratio`, dimension `Scalar`.
    pub(crate) fn ratio_leaf(ratio: Ratio) -> Self {
        Self::leaf(Dimension::Scalar, ExprKind::Ratio(ratio))
    }

    /// The integer constant `value`, dimension `Count`.
    pub(crate) fn integer_leaf(value: i64) -> Self {
        Self::leaf(Dimension::Count, ExprKind::Integer(value))
    }

    /// One full rotation, dimension `Angle`.
    pub(crate) fn turn_leaf() -> Self {
        Self::leaf(Dimension::Angle, ExprKind::Turn)
    }

    /// A leaf of the form's own vocabulary, read at `dim`.
    pub(crate) fn own_leaf(own: L, dim: Dimension) -> Self {
        Self::leaf(dim, ExprKind::Leaf(own))
    }

    /// A reader of the variable `var`, read at `dim` (the variable's
    /// kind; the doors re-check it against the document's table).
    pub fn var(var: VarId, dim: Dimension) -> Self {
        Self::leaf(dim, ExprKind::Var(var))
    }

    fn leaf(dim: Dimension, kind: ExprKind<L>) -> Self {
        Self {
            dim,
            nesting: 1,
            kind,
        }
    }

    /// An operator node over the children `kind` holds, refused when it
    /// would nest past [`MAX_NESTING`].
    fn over(dim: Dimension, kind: ExprKind<L>) -> Result<Self, DimensionError> {
        let below = match &kind {
            binary_kind!(a, b) => a.nesting.max(b.nesting),
            unary_kind!(a) => a.nesting,
            leaf_kind!() => 0,
        };
        Ok(Self {
            dim,
            nesting: nesting_over(below)?,
            kind,
        })
    }

    fn same_dim(
        op: &'static str,
        a: Self,
        b: Self,
        make: fn(Box<Self>, Box<Self>) -> ExprKind<L>,
    ) -> Result<Self, DimensionError> {
        if a.dim != b.dim {
            return Err(DimensionError::Mismatch {
                op,
                left: a.dim,
                right: b.dim,
            });
        }
        Self::over(a.dim, make(Box::new(a), Box::new(b)))
    }

    /// Same-dimension addition (Count included: Count is closed under
    /// add/sub/mul/neg/min/max, spec D4).
    pub fn add(a: Self, b: Self) -> Result<Self, DimensionError> {
        Self::same_dim("add", a, b, ExprKind::Add)
    }

    /// Same-dimension subtraction.
    pub fn sub(a: Self, b: Self) -> Result<Self, DimensionError> {
        Self::same_dim("sub", a, b, ExprKind::Sub)
    }

    /// Negation — any dimension (Count stays Count).
    ///
    /// # Errors
    ///
    /// [`DimensionError::NestedTooDeep`] alone: negation is total over
    /// every dimension, so only the nesting bound refuses it.
    pub fn neg(a: Self) -> Result<Self, DimensionError> {
        Self::over(a.dim, ExprKind::Neg(Box::new(a)))
    }

    /// Product. Permitted (F1): Count×Count → Count; otherwise at
    /// least one operand `Scalar`, result the other's dimension.
    /// Length×Length is a typed refusal (dimension-changing products
    /// are out of the v1 lattice; relaxation is additive). A single
    /// Count operand mixed with a continuous one is refused — promote
    /// explicitly via [`Expr::count_to_scalar`].
    pub fn mul(a: Self, b: Self) -> Result<Self, DimensionError> {
        use Dimension::{Count, Scalar};
        let dim = match (a.dim, b.dim) {
            (Count, Count) => Count,
            (Count, _) | (_, Count) => {
                return Err(DimensionError::CountNeedsExplicitPromotion { op: "mul" });
            }
            (Scalar, d) | (d, Scalar) => d,
            (l, r) => return Err(DimensionError::MulNeedsScalar { left: l, right: r }),
        };
        Self::over(dim, ExprKind::Mul(Box::new(a), Box::new(b)))
    }

    /// Quotient. The divisor must be `Scalar` (F1): Length/Length —
    /// the same-dimension ratio — is a typed refusal in v1 (pinned by
    /// test; relaxing to ratios later is purely additive). Count is
    /// not closed under division (spec D4 lists add/sub/mul/min/max),
    /// so any Count operand is refused — promote explicitly first.
    pub fn div(a: Self, b: Self) -> Result<Self, DimensionError> {
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
        Self::over(a.dim, ExprKind::Div(Box::new(a), Box::new(b)))
    }

    fn trig(
        op: &'static str,
        a: Self,
        make: fn(Box<Self>) -> ExprKind<L>,
    ) -> Result<Self, DimensionError> {
        if a.dim != Dimension::Angle {
            return Err(DimensionError::TrigNeedsAngle { op, found: a.dim });
        }
        Self::over(Dimension::Scalar, make(Box::new(a)))
    }

    /// Sine of an `Angle` → `Scalar` (spec D4).
    pub fn sin(a: Self) -> Result<Self, DimensionError> {
        Self::trig("sin", a, ExprKind::Sin)
    }

    /// Cosine of an `Angle` → `Scalar`.
    pub fn cos(a: Self) -> Result<Self, DimensionError> {
        Self::trig("cos", a, ExprKind::Cos)
    }

    /// Tangent of an `Angle` → `Scalar`.
    pub fn tan(a: Self) -> Result<Self, DimensionError> {
        Self::trig("tan", a, ExprKind::Tan)
    }

    /// Four-quadrant arctangent `atan2(y, x)` → `Angle` (spec D4).
    /// Operands must share one continuous dimension (their common
    /// scale cancels in the true ratio); Count operands are refused —
    /// promote explicitly.
    pub fn atan2(y: Self, x: Self) -> Result<Self, DimensionError> {
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
        Self::over(Dimension::Angle, ExprKind::Atan2(Box::new(y), Box::new(x)))
    }

    /// Same-dimension lattice minimum (value operation, never control
    /// flow — the AST has no branches, F7; Count included).
    pub fn min(a: Self, b: Self) -> Result<Self, DimensionError> {
        Self::same_dim("min", a, b, ExprKind::Min)
    }

    /// Same-dimension lattice maximum.
    pub fn max(a: Self, b: Self) -> Result<Self, DimensionError> {
        Self::same_dim("max", a, b, ExprKind::Max)
    }

    /// EXPLICIT Count→Scalar promotion (spec D4: never implicit).
    /// Refuses non-Count operands.
    pub fn count_to_scalar(a: Self) -> Result<Self, DimensionError> {
        if a.dim != Dimension::Count {
            return Err(DimensionError::NotCount { found: a.dim });
        }
        Self::over(Dimension::Scalar, ExprKind::CountToScalar(Box::new(a)))
    }

    /// The child at ExprPath index `i` (spec D5: operands in argument
    /// order), or `None` past the arity.
    pub fn child(&self, i: u8) -> Option<&Self> {
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
    pub fn descend(&self, path: &[u8]) -> Option<&Self> {
        path.iter().try_fold(self, |e, &i| e.child(i))
    }

    /// The variables this expression reads, with the dimension each
    /// leaf reads it at, in pre-order.
    pub fn var_reads(&self, out: &mut Vec<(VarId, Dimension)>) {
        match &self.kind {
            ExprKind::Var(var) => out.push((*var, self.dim)),
            ExprKind::Ratio(_) | ExprKind::Integer(_) | ExprKind::Turn | ExprKind::Leaf(_) => {}
            unary_kind!(a) => a.var_reads(out),
            binary_kind!(a, b) => {
                a.var_reads(out);
                b.var_reads(out);
            }
        }
    }

    /// Whether this expression reads the variable `var`.
    #[must_use]
    pub fn reads(&self, var: VarId) -> bool {
        let mut reads = Vec::new();
        self.var_reads(&mut reads);
        reads.iter().any(|&(read, _)| read == var)
    }

    /// Re-point every reader of a key of `map` at its value; a reader
    /// of any other variable is untouched.
    pub fn remap_vars(&mut self, map: &std::collections::BTreeMap<VarId, VarId>) {
        match &mut self.kind {
            ExprKind::Var(var) => {
                if let Some(&to) = map.get(var) {
                    *var = to;
                }
            }
            ExprKind::Ratio(_) | ExprKind::Integer(_) | ExprKind::Turn | ExprKind::Leaf(_) => {}
            unary_kind!(a) => a.remap_vars(map),
            binary_kind!(a, b) => {
                a.remap_vars(map);
                b.remap_vars(map);
            }
        }
    }

    /// Pushes the `f64` BITS of every value this form's own leaves
    /// hold, in pre-order (children in [`Expr::child`] order): the
    /// bit-semantic comparison substrate (spec D7: replay is
    /// bit-identical, so the comparators must not be bit-blind). A
    /// stored expression holds no float, so it pushes nothing.
    pub fn literal_bits(&self, out: &mut Vec<u64>) {
        self.visit_leaves(&mut |leaf, _| leaf.bits(out));
    }

    /// Bit-semantic equality (M4 PR 1 review non-blocker): structural
    /// equality with every float a leaf holds compared by BITS — `0.0`
    /// and `-0.0` are DIFFERENT formulas here, unlike `PartialEq`
    /// (which stays IEEE-semantic). NaN cannot occur (door 1 refuses
    /// non-finite values), so `PartialEq` + aligned bit vectors is
    /// exact: when `self == other`, both trees have identical shape, so
    /// the traversals align leaf for leaf.
    pub fn bit_eq(&self, other: &Self) -> bool {
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
    pub fn with_replaced(&self, path: &[u8], new: Self) -> Option<Result<Self, DimensionError>> {
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
        let other = |b: &Self| b.clone();
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
            (K::Neg(_), 0) => Self::neg(rebuilt),
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

impl<L: LeafSet> ExprTree<L> {
    /// **This tree with readers replaced**: every reader of a variable
    /// `f` answers for replaced by the answer, each operator above one
    /// rebuilt through its checking constructor.
    ///
    /// # Errors
    ///
    /// The constructors' first refusal, which is
    /// [`DimensionError::NestedTooDeep`] where a replacement deepens the
    /// tree past [`MAX_NESTING`]; or `f`'s answer at another dimension
    /// than the reader's.
    pub fn substitute_vars(
        &self,
        f: &mut impl FnMut(VarId) -> Option<Self>,
    ) -> Result<Self, DimensionError> {
        use ExprKind as K;
        let mut go = |e: &Self| e.substitute_vars(f);
        match &self.kind {
            K::Var(var) => match f(*var) {
                Some(by) if by.dim == self.dim => Ok(by),
                Some(by) => Err(DimensionError::Mismatch {
                    op: "substitute",
                    left: self.dim,
                    right: by.dim,
                }),
                None => Ok(self.clone()),
            },
            K::Ratio(_) | K::Integer(_) | K::Turn | K::Leaf(_) => Ok(self.clone()),
            K::Add(a, b) => Self::add(go(a)?, go(b)?),
            K::Sub(a, b) => Self::sub(go(a)?, go(b)?),
            K::Mul(a, b) => Self::mul(go(a)?, go(b)?),
            K::Div(a, b) => Self::div(go(a)?, go(b)?),
            K::Atan2(a, b) => Self::atan2(go(a)?, go(b)?),
            K::Min(a, b) => Self::min(go(a)?, go(b)?),
            K::Max(a, b) => Self::max(go(a)?, go(b)?),
            K::Neg(a) => Self::neg(go(a)?),
            K::Sin(a) => Self::sin(go(a)?),
            K::Cos(a) => Self::cos(go(a)?),
            K::Tan(a) => Self::tan(go(a)?),
            K::CountToScalar(a) => Self::count_to_scalar(go(a)?),
        }
    }
}

impl<L: LeafSet> ExprTree<L> {
    /// **Every leaf of this form's own set**, with the dimension it is
    /// read at, in pre-order: what a reader of the leaves asks, where
    /// [`Self::try_map_leaves`] rewrites them.
    ///
    /// Recursion is bounded by [`MAX_NESTING`], as every walk over a
    /// constructed tree is.
    pub(crate) fn visit_leaves(&self, visit: &mut impl FnMut(&L, Dimension)) {
        use ExprKind as K;
        match &self.kind {
            K::Leaf(leaf) => visit(leaf, self.dim),
            K::Ratio(_) | K::Integer(_) | K::Turn | K::Var(_) => {}
            K::Add(a, b)
            | K::Sub(a, b)
            | K::Mul(a, b)
            | K::Div(a, b)
            | K::Atan2(a, b)
            | K::Min(a, b)
            | K::Max(a, b) => {
                a.visit_leaves(visit);
                b.visit_leaves(visit);
            }
            K::Neg(a) | K::Sin(a) | K::Cos(a) | K::Tan(a) | K::CountToScalar(a) => {
                a.visit_leaves(visit);
            }
        }
    }

    /// **Every leaf of this tree**, with the dimension it is read at,
    /// in pre-order: a reader by id, a leaf of this form's own set, or a
    /// constant.
    ///
    /// Recursion is bounded by [`MAX_NESTING`], as every walk over a
    /// constructed tree is.
    pub(crate) fn visit_terminals(&self, visit: &mut impl FnMut(Terminal<'_, L>, Dimension)) {
        use ExprKind as K;
        match &self.kind {
            K::Leaf(leaf) => visit(Terminal::Leaf(leaf), self.dim),
            K::Var(var) => visit(Terminal::Var(*var), self.dim),
            K::Ratio(_) | K::Integer(_) | K::Turn => visit(Terminal::Constant, self.dim),
            binary_kind!(a, b) => {
                a.visit_terminals(visit);
                b.visit_terminals(visit);
            }
            unary_kind!(a) => a.visit_terminals(visit),
        }
    }

    /// **This tree in another form**: every leaf of this form's own
    /// vocabulary rewritten by `own` (handed the dimension it reads at),
    /// every shared leaf and operator kept, with its dimension and
    /// nesting. The first refusal of `own`, in pre-order, is the
    /// answer.
    ///
    /// Recursion is bounded by [`MAX_NESTING`], as every walk over a
    /// constructed tree is.
    pub(crate) fn try_map_leaves<M: LeafSet, E>(
        &self,
        own: &mut impl FnMut(&L, Dimension) -> Result<ExprTree<M>, E>,
    ) -> Result<ExprTree<M>, E> {
        use ExprKind as K;
        let mut map = |e: &Self| e.try_map_leaves(own).map(Box::new);
        let kind = match &self.kind {
            K::Leaf(leaf) => return own(leaf, self.dim),
            K::Ratio(r) => K::Ratio(*r),
            K::Integer(n) => K::Integer(*n),
            K::Turn => K::Turn,
            K::Var(var) => K::Var(*var),
            K::Add(a, b) => K::Add(map(a)?, map(b)?),
            K::Sub(a, b) => K::Sub(map(a)?, map(b)?),
            K::Mul(a, b) => K::Mul(map(a)?, map(b)?),
            K::Div(a, b) => K::Div(map(a)?, map(b)?),
            K::Atan2(a, b) => K::Atan2(map(a)?, map(b)?),
            K::Min(a, b) => K::Min(map(a)?, map(b)?),
            K::Max(a, b) => K::Max(map(a)?, map(b)?),
            K::Neg(a) => K::Neg(map(a)?),
            K::Sin(a) => K::Sin(map(a)?),
            K::Cos(a) => K::Cos(map(a)?),
            K::Tan(a) => K::Tan(map(a)?),
            K::CountToScalar(a) => K::CountToScalar(map(a)?),
        };
        Ok(ExprTree {
            dim: self.dim,
            nesting: self.nesting,
            kind,
        })
    }
}

// The constants both forms share.
impl<L: LeafSet> ExprTree<L> {
    /// This tree as an author writes it: every leaf kept.
    pub(crate) fn to_formula(&self) -> crate::Formula {
        let Ok(formula) = self.try_map_leaves(&mut |leaf, dim| {
            Ok::<_, core::convert::Infallible>(crate::Formula::own_leaf(leaf.authored(), dim))
        });
        formula
    }

    /// The exact rational constant `num / den`, dimension `Scalar`.
    ///
    /// # Errors
    ///
    /// [`Ratio::new`]'s.
    pub fn ratio(num: i64, den: u64) -> Result<Self, DimensionError> {
        Ratio::new(num, den).map(Self::ratio_leaf)
    }

    /// The exact integer constant `value`, dimension `Count`.
    pub fn integer(value: i64) -> Self {
        Self::integer_leaf(value)
    }

    /// One full rotation, dimension `Angle`: `turn / 4` is a right
    /// angle.
    pub fn turn() -> Self {
        Self::turn_leaf()
    }

    /// The constant this tree is, where it is one lone rational.
    pub fn as_ratio(&self) -> Option<Ratio> {
        match self.kind {
            ExprKind::Ratio(r) => Some(r),
            _ => None,
        }
    }

    /// The integer this tree is, where it is one lone integer constant.
    pub fn as_integer(&self) -> Option<i64> {
        match self.kind {
            ExprKind::Integer(n) => Some(n),
            _ => None,
        }
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
/// [`crate::Doc::var_env`] embeds them into any [`Real`].
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

/// The id→value environment [`eval`] and [`eval_count`] read variable
/// readers from (spec D4's `params`).
#[derive(Debug, Clone, PartialEq)]
pub struct VarEnv<T> {
    /// The bindings, by variable.
    pub bindings: std::collections::BTreeMap<VarId, ParamValue<T>>,
    /// The defined variables whose definition refused, by variable: a
    /// reader of one refuses [`EvalError::DefinitionRefused`] with
    /// this refusal as its source — or, for one in [`Self::written`],
    /// with the refusal itself.
    pub refused: std::collections::BTreeMap<VarId, EvalError>,
    /// The defined variables a document holds with no name: each is a
    /// formula as it was written at the slot that reads it, so a
    /// refusal of its definition is the slot's own refusal, not a
    /// variable's the person never named.
    pub written: std::collections::BTreeSet<VarId>,
}

impl<T> VarEnv<T> {
    /// The binding of `var`, or why it has none.
    pub(crate) fn binding(&self, var: VarId) -> Result<&ParamValue<T>, EvalError> {
        match self.bindings.get(&var) {
            Some(bound) => Ok(bound),
            None => Err(match self.refused.get(&var) {
                Some(source) if self.written.contains(&var) => source.clone(),
                Some(source) => EvalError::DefinitionRefused {
                    var,
                    source: Box::new(source.clone()),
                },
                None => EvalError::UnresolvedVar { var },
            }),
        }
    }
}

// Manual impl: the derive would demand `T: Default`, which certified
// scalars (Interval) deliberately do not provide.
impl<T> Default for VarEnv<T> {
    fn default() -> Self {
        Self {
            bindings: std::collections::BTreeMap::new(),
            refused: std::collections::BTreeMap::new(),
            written: std::collections::BTreeSet::new(),
        }
    }
}

/// Typed evaluation failure (spec D4). Numeric-domain issues (division
/// by zero, out-of-domain trig) are NOT errors here: they follow the
/// kernel's poison-value policy through `T` (`geom-core::real` module
/// docs) — the evaluator has no branches to hide them behind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    /// A reader of a variable with no binding in the environment: a
    /// variable the document deleted (VR7: its readers stay, unresolved),
    /// or one a hand-built environment left out.
    UnresolvedVar {
        /// The variable read.
        var: VarId,
    },
    /// A reader of a defined variable whose definition refused: the
    /// definition's own refusal is `source`. The reader's address is
    /// the wrapper's, as for [`Self::VarKindMismatch`].
    DefinitionRefused {
        /// The defined variable read.
        var: VarId,
        /// Why its definition refused.
        source: Box<EvalError>,
    },
    /// A reader whose cached kind disagrees with the dimension the
    /// environment bound the variable at.
    ///
    /// The kind fact at EVALUATION, where the edit and load doors
    /// spell it `{Slot,Payload}VarKind`. It is named by the fact alone
    /// because the address is not this arm's to carry: the wrapper
    /// supplies it ([`crate::eval::NodeErrorKind::Expr`] a node and a
    /// slot, [`crate::eval::NodeErrorKind::PayloadExpr`] a node and a
    /// payload), and it forwards this refusal unaltered.
    VarKindMismatch {
        /// The variable read.
        var: VarId,
        /// The dimension the environment bound.
        bound: Dimension,
        /// The dimension the reader reads at.
        read: Dimension,
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
    /// scalars this refuses what may not certify (NaI, empty and
    /// `Trv`-decorated enclosures); legitimately unbounded-but-valid
    /// enclosures pass (boundedness is the `Com`-decoration's business,
    /// not this door's).
    NonFiniteResult,
    /// A leaf of an authored formula that only the edit door resolves —
    /// a name, or a fresh-table entry — evaluated outside it: lower the
    /// formula in a document first.
    Unlowered(crate::LowerFault),
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM and, where the fault has a lever, its one recourse. The
// enum stays the machine contract; composing layers (the evaluation
// service's `Expr` slot arm) FORWARD this rendering rather than
// re-stating it.
impl core::fmt::Display for EvalError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnresolvedVar { var } => write!(
                f,
                "variable {var} has no binding in the evaluation environment — it was \
                 deleted, or never declared here; point the reader at a live variable"
            ),
            Self::DefinitionRefused { var, source } => {
                write!(f, "the definition of variable {var} refused: {source}")
            }
            Self::VarKindMismatch { var, bound, read } => {
                write!(f, "variable {var} is read as {read} but bound as {bound}")
            }
            Self::CountExprInContinuousEval => f.write_str(
                "a count expression does not evaluate continuously — promote it \
                 explicitly through count_to_scalar",
            ),
            Self::ContinuousExprInCountEval { found } => write!(
                f,
                "{} {found} expression does not evaluate as a count — counts are exact \
                 and never inferred from a continuous value",
                found.article()
            ),
            Self::CountOverflow => {
                f.write_str("exact count arithmetic overflowed (a count never wraps)")
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
            Self::Unlowered(fault) => write!(
                f,
                "{fault}, and only a document's edit door resolves it; evaluate the formula \
                 as the document lowers it"
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
/// evaluation itself needs only `Real` (see `eval_inner`).
pub fn eval<T: Decide>(expr: &ExprTree<impl LeafSet>, params: &VarEnv<T>) -> Result<T, EvalError> {
    refuse_non_finite(eval_inner(expr, params)?)
}

/// **A slot's variable evaluated** (VARIABLES-DESIGN VR4): its binding
/// in `params`, read at `dim`, the dimension the slot's address reads
/// it at — exactly [`eval`] of a lone reader of `var`.
///
/// # Errors
///
/// [`eval`]'s, of that reader.
pub fn eval_var<T: Decide>(var: VarId, dim: Dimension, params: &VarEnv<T>) -> Result<T, EvalError> {
    eval(&Expr::var(var, dim), params)
}

/// **A structural slot's variable evaluated**: [`eval_count`] of a lone
/// `Count` reader of `var`.
///
/// # Errors
///
/// [`eval_count`]'s, of that reader.
pub fn eval_var_count<T>(var: VarId, params: &VarEnv<T>) -> Result<i64, EvalError> {
    eval_count(&Expr::var(var, Dimension::Count), params)
}

/// **Door 2, as a shared door.** The ruled non-finite check on a
/// FINAL evaluated value: `value * 0` is EXACTLY zero for every finite
/// value and NaN or refused (NaN / empty / Trv) otherwise, so any valid
/// band classifies it identically — Zero passes, everything else is a
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
///
/// **Named, through the funnel.** This decision used to call
/// `sign_within` directly, outside any named `classify` — and because
/// the recorder's name channel was never reset, every K sample it
/// recorded was charged to whichever predicate had classified LAST
/// (1,054 samples in the corpus sweep at ε = 1e-6, found when M10-8
/// scoped the name). It goes through the recorder's named evaluator
/// door now (`k_stats::check_unlogged`) — its samples carry its own
/// name, and it stays out of the VERDICT log, which is the verdict-diff
/// engine's row-for-row comparison of certification predicates between
/// the witness and a leaf: this check fires once per expression
/// evaluation, a count the two lanes do not share, and logging it
/// refused every M10-6 min-clearance box on a vector mismatch with no
/// geometry changed. The ledger row says why no `Margin` door fits:
/// `value · 0` carries `value`'s dimension, whatever that is.
pub(crate) fn refuse_non_finite<T: Decide>(value: T) -> Result<T, EvalError> {
    let Ok(band) = Band::new(1e-100, 1e-50) else {
        return Err(EvalError::NonFiniteResult);
    };
    match geom_core::k_stats::check_unlogged("expr_non_finite", value * T::zero(), band, "F18") {
        Ok(Sign::Zero) => Ok(value),
        _ => Err(EvalError::NonFiniteResult),
    }
}

/// The evaluation core — `Real` only (no decisions inside: poison FLOWS
/// through values per the kernel policy; the single refusal door is
/// [`eval`]'s final check). The walk keeps its own stack
/// ([`crate::tree::fold`]), so how deep the expression nests costs the
/// thread's stack nothing.
fn eval_inner<T: Real, L: LeafSet>(root: &ExprTree<L>, params: &VarEnv<T>) -> Result<T, EvalError> {
    use crate::tree::{Operands as O, Visit};
    use ExprKind as K;
    crate::tree::fold(
        root,
        |expr| {
            if expr.dim == Dimension::Count {
                return Err(EvalError::CountExprInContinuousEval);
            }
            Ok(match &expr.kind {
                K::Ratio(r) => Visit::Value(r.eval()),
                K::Turn => Visit::Value(T::tau()),
                K::Integer(_) => return Err(EvalError::CountExprInContinuousEval),
                K::Var(var) => match params.binding(*var)? {
                    ParamValue::Continuous { dim, value } if *dim == expr.dim => {
                        Visit::Value(*value)
                    }
                    bound => {
                        return Err(EvalError::VarKindMismatch {
                            var: *var,
                            bound: bound.dim(),
                            read: expr.dim,
                        });
                    }
                },
                K::Leaf(own) => Visit::Value(own.value(expr.dim)?),
                K::CountToScalar(a) => {
                    let n = eval_count(a, params)?;
                    // i32::try_from is total on i64 (no abs, no panic —
                    // i64::MIN is a typed refusal); f64::from(i32) is exact.
                    let small =
                        i32::try_from(n).map_err(|_| EvalError::CountToScalarOutOfRange(n))?;
                    Visit::Value(T::from_f64(f64::from(small)))
                }
                binary_kind!(a, b) => Visit::Two(a, b),
                K::Neg(a) | K::Sin(a) | K::Cos(a) | K::Tan(a) => Visit::One(a),
            })
        },
        |expr, operands| {
            Ok(match (&expr.kind, operands) {
                (K::Neg(_), O::One(a)) => -a,
                (K::Sin(_), O::One(a)) => a.sin(),
                (K::Cos(_), O::One(a)) => a.cos(),
                (K::Tan(_), O::One(a)) => a.tan(),
                (K::Add(..), O::Two(a, b)) => a + b,
                (K::Sub(..), O::Two(a, b)) => a - b,
                (K::Mul(..), O::Two(a, b)) => a * b,
                (K::Div(..), O::Two(a, b)) => a / b,
                (K::Atan2(..), O::Two(y, x)) => y.atan2(x),
                (K::Min(..), O::Two(a, b)) => a.min(b),
                (K::Max(..), O::Two(a, b)) => a.max(b),
                (kind, _) => unreachable!(
                    "{kind:?} is valued when visited, or combined from as many operands as it \
                     has children"
                ),
            })
        },
    )
}

/// Evaluate a `Count` expression to an exact `i64` (spec D4: Count is
/// integer-valued; arithmetic is checked, overflow a typed error). The
/// walk keeps its own stack, as [`eval`]'s does.
pub fn eval_count<T>(root: &ExprTree<impl LeafSet>, params: &VarEnv<T>) -> Result<i64, EvalError> {
    use crate::tree::{Operands as O, Visit};
    use ExprKind as K;
    let checked = |r: Option<i64>| r.ok_or(EvalError::CountOverflow);
    crate::tree::fold(
        root,
        |expr| {
            if expr.dim != Dimension::Count {
                return Err(EvalError::ContinuousExprInCountEval { found: expr.dim });
            }
            Ok(match &expr.kind {
                K::Integer(n) => Visit::Value(*n),
                K::Var(var) => match params.binding(*var)? {
                    ParamValue::Count(n) => Visit::Value(*n),
                    bound => {
                        return Err(EvalError::VarKindMismatch {
                            var: *var,
                            bound: bound.dim(),
                            read: Dimension::Count,
                        });
                    }
                },
                K::Leaf(own) => Visit::Value(own.count(expr.dim)?),
                K::Add(a, b) | K::Sub(a, b) | K::Mul(a, b) | K::Min(a, b) | K::Max(a, b) => {
                    Visit::Two(a, b)
                }
                K::Neg(a) => Visit::One(a),
                // Construction makes these unrepresentable at Count dimension.
                K::Ratio(_)
                | K::Turn
                | K::Div(..)
                | K::Sin(_)
                | K::Cos(_)
                | K::Tan(_)
                | K::Atan2(..)
                | K::CountToScalar(_) => {
                    return Err(EvalError::ContinuousExprInCountEval { found: expr.dim });
                }
            })
        },
        |expr, operands| match (&expr.kind, operands) {
            (K::Neg(_), O::One(a)) => checked(a.checked_neg()),
            (K::Add(..), O::Two(a, b)) => checked(a.checked_add(b)),
            (K::Sub(..), O::Two(a, b)) => checked(a.checked_sub(b)),
            (K::Mul(..), O::Two(a, b)) => checked(a.checked_mul(b)),
            (K::Min(..), O::Two(a, b)) => Ok(a.min(b)),
            (K::Max(..), O::Two(a, b)) => Ok(a.max(b)),
            (kind, _) => unreachable!(
                "{kind:?} is refused or valued when visited, or combined from as many operands \
                 as it has children"
            ),
        },
    )
}

/// Binding level of a rendered expression, in the text grammar's own
/// terms (`parse`'s module docs): sums bind loosest, then products,
/// then unary minus, and a leaf or a call is an atom that no
/// surrounding operator can split.
///
/// Used only to decide parentheses, so the numbers matter only in
/// their order.
const PREC_SUM: u8 = 1;
/// See [`PREC_SUM`].
const PREC_PRODUCT: u8 = 2;
/// See [`PREC_SUM`].
const PREC_UNARY: u8 = 3;
/// See [`PREC_SUM`].
const PREC_ATOM: u8 = 4;

/// The binding level [`unparse`] renders `expr` at.
///
/// A literal whose value is NEGATIVE renders with a leading `-` and
/// therefore binds as a negation does, not as an atom — the sign is
/// part of the emitted text, and whoever is deciding parentheses has
/// to see it.
fn precedence<L: LeafSet>(expr: &ExprTree<L>) -> u8 {
    match &expr.kind {
        ExprKind::Ratio(r) if r.is_negative() => PREC_UNARY,
        ExprKind::Integer(n) if *n < 0 => PREC_UNARY,
        ExprKind::Leaf(own) if own.negative() => PREC_UNARY,
        ExprKind::Add(..) | ExprKind::Sub(..) => PREC_SUM,
        ExprKind::Mul(..) | ExprKind::Div(..) => PREC_PRODUCT,
        ExprKind::Neg(_) => PREC_UNARY,
        ExprKind::Ratio(_)
        | ExprKind::Integer(_)
        | ExprKind::Turn
        | ExprKind::Var(_)
        | ExprKind::Leaf(_)
        | ExprKind::Sin(_)
        | ExprKind::Cos(_)
        | ExprKind::Tan(_)
        | ExprKind::Atan2(..)
        | ExprKind::Min(..)
        | ExprKind::Max(..)
        | ExprKind::CountToScalar(_) => PREC_ATOM,
    }
}

/// The expression TEXT door OUTWARD (issue #1103): source text that
/// [`crate::parse_formula`] reads back as this very expression.
///
/// **The contract is the round trip, structurally**: for every `e`
/// the constructors admit, `parse_formula(&unparse(&e, names), params)` is
/// [`ExprTree::bit_eq`] to `e` — same tree, so the same nesting, and the
/// same value BITS — and its quantities remember the same display
/// units (which `bit_eq` deliberately does not compare, being
/// presentation metadata). Parentheses are emitted where and only
/// where the grammar needs them to reproduce the same tree, which is
/// stricter than "the same value": the parser is left-associative, so
/// `Add(a, Add(b, c))` is parenthesised even though `a + b + c`
/// evaluates identically.
///
/// A written quantity is written in the display unit it REMEMBERS
/// ([`crate::Formula::display_unit`]) — through
/// [`quantity::fmt_length`]/[`quantity::fmt_angle`], whose own pin is
/// that the digits multiply back to the exact bits, and which fall back
/// to the canonical unit for the values that have no preimage in the
/// asked-for one. A `Scalar` one takes no unit and is written with a
/// decimal point or an exponent, because a BARE integer is this
/// grammar's spelling of a `Count`; it is one no constant in range
/// spells exactly ([`crate::Formula::number`]), so its digits read back
/// as a written value. A rational constant is written as
/// a decimal where its denominator is a product of twos and fives and
/// as `p/q` otherwise ([`Ratio`]'s `Display`), and one full rotation
/// as `turn`.
///
/// A negative number is written with its sign (`-25 mm`), which the
/// parser reads as the number's own; the negation of a non-negative
/// number is therefore bracketed (`-(25 mm)`), the one place a bracket
/// around an atom is needed.
///
/// A reader writes the name `names` gives its variable, so the text
/// parses back to a name leaf that lowers, against the same names, to
/// the same reader. A reader `names` has no name for writes
/// `#<16 hex>`, its full id, which the parser refuses.
pub fn unparse<'n, L: LeafSet>(
    expr: &ExprTree<L>,
    names: &impl Fn(VarId) -> Option<&'n VarName>,
) -> String {
    let mut out = String::new();
    write_expr(expr, names, &mut out);
    out
}

/// The names a rendering reads its readers' text from.
type Names<'a, 'n> = &'a dyn Fn(VarId) -> Option<&'n VarName>;

/// `expr`, wrapped in parentheses unless it already binds at least as
/// tightly as `needs`.
fn write_nested<L: LeafSet>(expr: &ExprTree<L>, needs: u8, names: Names<'_, '_>, out: &mut String) {
    if precedence(expr) < needs {
        out.push('(');
        write_expr(expr, names, out);
        out.push(')');
    } else {
        write_expr(expr, names, out);
    }
}

/// A binary infix rendering at binding level `level`.
///
/// The RIGHT operand is parenthesised one level tighter than the left.
/// That asymmetry is the parser's left-associativity: `a - (b - c)`
/// and `a - b - c` are different trees, so a right operand binding at
/// its parent's own level has to be bracketed even where arithmetic
/// would not care.
fn write_infix<L: LeafSet>(
    left: &ExprTree<L>,
    op: &str,
    right: &ExprTree<L>,
    level: u8,
    names: Names<'_, '_>,
    out: &mut String,
) {
    write_nested(left, level, names, out);
    out.push(' ');
    out.push_str(op);
    out.push(' ');
    write_nested(right, level + 1, names, out);
}

/// A call in the parser's own spelling — the arguments are delimited,
/// so no argument is ever parenthesised.
fn write_call<L: LeafSet>(
    name: &str,
    args: &[&ExprTree<L>],
    names: Names<'_, '_>,
    out: &mut String,
) {
    out.push_str(name);
    out.push('(');
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        write_expr(arg, names, out);
    }
    out.push(')');
}

/// The rendering proper (see [`unparse`] for the contract).
fn write_expr<L: LeafSet>(expr: &ExprTree<L>, names: Names<'_, '_>, out: &mut String) {
    use ExprKind as K;
    match &expr.kind {
        K::Ratio(r) => out.push_str(&r.to_string()),
        K::Integer(n) => out.push_str(&n.to_string()),
        K::Turn => out.push_str("turn"),
        K::Leaf(own) => own.write(expr.dim, out),
        // A reader the names cannot speak — an anonymous variable, or
        // one the document no longer holds — writes its full id, which
        // the parser does not read: such text names no variable.
        K::Var(var) => match names(*var) {
            Some(name) => out.push_str(name.as_str()),
            None => out.push_str(&var.to_string()),
        },
        K::Add(a, b) => write_infix(a, "+", b, PREC_SUM, names, out),
        K::Sub(a, b) => write_infix(a, "-", b, PREC_SUM, names, out),
        K::Mul(a, b) => write_infix(a, "*", b, PREC_PRODUCT, names, out),
        K::Div(a, b) => write_infix(a, "/", b, PREC_PRODUCT, names, out),
        K::Neg(a) => {
            out.push('-');
            let numeric = match &a.kind {
                K::Ratio(_) | K::Integer(_) => true,
                K::Leaf(own) => own.numeric(),
                _ => false,
            };
            if numeric && precedence(a) == PREC_ATOM {
                out.push('(');
                write_expr(a, names, out);
                out.push(')');
            } else {
                write_nested(a, PREC_UNARY, names, out);
            }
        }
        K::Sin(a) => write_call("sin", &[a], names, out),
        K::Cos(a) => write_call("cos", &[a], names, out),
        K::Tan(a) => write_call("tan", &[a], names, out),
        K::CountToScalar(a) => write_call("scalar", &[a], names, out),
        K::Atan2(y, x) => write_call("atan2", &[y, x], names, out),
        K::Min(a, b) => write_call("min", &[a, b], names, out),
        K::Max(a, b) => write_call("max", &[a, b], names, out),
    }
}

/// One written quantity, in the unit it remembers.
fn write_quantity(value: f64, unit: UnitSym, dim: Dimension) -> String {
    let remembered = unit.def();
    let formatted = match dim {
        // The stored unit and the dimension agree by construction —
        // `Formula::literal_with_unit` checks the pairing and `literal`
        // supplies the canonical row — so the typed view is there, and
        // its absence is D2 addendum row 4 rather than a default: a
        // quiet fallback here would render a corrupt literal as if it
        // were fine.
        Dimension::Length => {
            let Some(unit) = remembered.as_length() else {
                unreachable!(
                    "a Length literal remembers {:?}, which measures {:?}",
                    remembered.symbol(),
                    remembered.quantity()
                )
            };
            quantity::fmt_length(value, unit)
        }
        Dimension::Angle => {
            let Some(unit) = remembered.as_angle() else {
                unreachable!(
                    "an Angle literal remembers {:?}, which measures {:?}",
                    remembered.symbol(),
                    remembered.quantity()
                )
            };
            quantity::fmt_angle(value, unit)
        }
        // The dimensionless row's symbol is empty, so there is no
        // suffix to append and no formatter to route through: a
        // dimensionless real is its own text. It must not render bare
        // DIGITS, though — `2` is a `Count` in this grammar and `2.0`
        // is a `Scalar`. `{:?}` is the shortest form that reads back to
        // the same bits and always carries a `.` or an `e`, so it
        // settles the round trip and the dimension together.
        Dimension::Scalar => return format!("{value:?}"),
        // Unconstructable (D2 addendum row 4): `Formula::literal` refuses
        // `Count`, and `ExprKind::Literal` is minted nowhere else.
        Dimension::Count => {
            unreachable!("a count is an integer constant (ExprKind::Integer), never a quantity")
        }
    };
    match formatted {
        Ok(text) => text,
        // Same row: door 1 refuses a non-finite literal at
        // construction, and non-finiteness is the formatter's only
        // refusal.
        Err(error) => unreachable!(
            "a stored literal is finite by construction, yet the display formatter refused: \
             {error}"
        ),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use super::*;

    use crate::tree::raw_chain;

    /// A negation over `e`, built past the constructors, which refuse
    /// it past the bound.
    fn raw_neg(e: Expr) -> Expr {
        Expr {
            dim: e.dim,
            nesting: u8::MAX,
            kind: ExprKind::Neg(Box::new(e)),
        }
    }

    /// The evaluators and `Drop` cost the stack nothing per level: a
    /// tree a million levels deep evaluates and frees on the wasm32
    /// stack, where one frame per level would exhaust it a thousand
    /// times over.
    #[test]
    fn evaluation_and_drop_keep_their_own_stack() {
        test_utils::own_thread::on_the_smallest_stack(|| {
            let levels = 1_000_000;
            let half = Expr::ratio(1, 2).expect("a constant in range");
            let deep = raw_chain(half, levels, raw_neg);
            assert_eq!(eval(&deep, &VarEnv::<f64>::default()), Ok(-0.5));
            let counted = raw_chain(Expr::integer(3), levels, raw_neg);
            assert_eq!(eval_count(&counted, &VarEnv::<f64>::default()), Ok(-3));
            drop((deep, counted));
        });
    }
}
