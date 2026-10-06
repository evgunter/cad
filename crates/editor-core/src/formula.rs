//! **The authored expression** (VARIABLES-DESIGN VR6): what a person
//! or a caller writes into a slot or a definition, before the edit door
//! stores it.
//!
//! A [`Formula`] is the same dimension-checked tree as the stored
//! [`Expr`], with the leaves only an author writes beside the shared
//! ones: a variable by name ([`Formula::named`]). The edit door lowers
//! it against the document's names ([`Formula::lower`]), and the
//! stored form has no name leaf to hold, so a stored document reads
//! every variable by id as a fact of its types.

use crate::doc::VarName;
use crate::expr::{AuthoredLeaf, Dimension, DimensionError, Expr, ExprTree, StoredLeaf, Unlowered};
use crate::var::VarId;

/// The authored expression: the shared leaves, plus a variable by name.
pub type Formula = ExprTree<AuthoredLeaf>;

/// **A name leaf the lowering did not resolve**, the first in
/// pre-order: the name, the dimension the leaf reads it at, and why it
/// stayed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameFault {
    /// The name the leaf reads.
    pub name: VarName,
    /// The dimension the leaf reads it at.
    pub dim: Dimension,
    /// Why it did not lower.
    pub why: Unlowered,
}

impl core::fmt::Display for NameFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { name, dim, why } = self;
        match why {
            Unlowered::Unheld => write!(f, "no variable is named {name}"),
            Unlowered::Kind { var: _, declared } => write!(
                f,
                "the variable named {name} is {} {declared}, read here as {} {dim}",
                declared.article(),
                dim.article()
            ),
        }
    }
}

impl core::error::Error for NameFault {}

// The constructors share names with the std ops traits on purpose, as
// `Expr`'s do.
#[allow(clippy::should_implement_trait)]
impl Formula {
    /// A continuous dimensioned literal in canonical kernel units.
    /// Refuses [`Dimension::Count`] — Count literals are integers
    /// ([`Formula::count`]) — and NON-FINITE values (ruled door 1 of the
    /// non-finite policy: the kernel never produces NaN/inf
    /// legitimately, so recipe data must not admit them; F3's
    /// persist-time refusal then has nothing to catch).
    pub fn literal(value: f64, dim: Dimension) -> Result<Self, DimensionError> {
        Self::literal_leaf(value, dim)
    }

    /// A continuous literal that REMEMBERS the display unit it was
    /// authored in (LIB-SWITCH §4g; the text door's `25 mm` row).
    /// `value` is already canonical (meters/radians) — the parser does
    /// its one multiply before this door. The unit's quantity must
    /// agree with `dim` ([`DimensionError::DisplayUnitMismatch`]);
    /// everything [`Formula::literal`] refuses is refused here too.
    ///
    /// The unit is presentation metadata (DESIGN.md D6): it round-trips
    /// through persistence and feeds the display formatter, but never
    /// enters [`Formula::bit_eq`], [`Formula::literal_bits`], content/naming
    /// keys, or evaluation.
    pub fn literal_with_unit(
        value: f64,
        dim: Dimension,
        unit: quantity::UnitDef,
    ) -> Result<Self, DimensionError> {
        Self::literal_leaf_with_unit(value, dim, unit)
    }

    /// A continuous literal from an AUTHORED length — the value and
    /// the notation it was written in, together
    /// ([`quantity::WrittenLength`]).
    ///
    /// The door library and GUI authoring should reach for. A caller
    /// never spells the dimension — a `WrittenLength` is a length, so
    /// there is no second fact to keep in step — and the literal
    /// ALWAYS remembers a unit, because an authored quantity always
    /// names the one it is written in (`quantity::written`'s module
    /// docs). [`Formula::literal`] remains the door for a value whose
    /// notation is not a CHOICE — it stores the canonical row for the
    /// dimension, `quantity::ONE` for a `Scalar`.
    ///
    /// **`DisplayUnitMismatch` cannot fire here.** A
    /// [`quantity::WrittenLength`] holds a `LengthUnit`, which is an
    /// index into a Length row of the table (#669), so the unit's
    /// quantity agrees with `Dimension::Length` by construction rather
    /// than by a check. The refusal that remains is
    /// [`Formula::literal`]'s: a non-finite value (ruled door 1). The
    /// claim is executed by
    /// `switch_display_units::every_authored_unit_reaches_a_literal_without_a_mismatch`,
    /// not merely stated here.
    ///
    /// # Errors
    ///
    /// [`DimensionError::NonFiniteLiteral`] for a non-finite value.
    pub fn written_length(written: quantity::WrittenLength) -> Result<Self, DimensionError> {
        Self::literal_with_unit(written.meters(), Dimension::Length, written.unit().def())
    }

    /// A continuous literal from an AUTHORED angle —
    /// [`Formula::written_length`]'s mirror, and everything that door's
    /// docs say holds here with `AngleUnit` and `Dimension::Angle`.
    ///
    /// # Errors
    ///
    /// [`DimensionError::NonFiniteLiteral`] for a non-finite value.
    pub fn written_angle(written: quantity::WrittenAngle) -> Result<Self, DimensionError> {
        Self::literal_with_unit(written.radians(), Dimension::Angle, written.unit().def())
    }

    /// A continuous literal from a length authored as `value` in
    /// `unit` — exactly
    /// `Formula::written_length(WrittenLength::in_unit(value, unit))`,
    /// the composition an authoring caller holding a number and a unit
    /// writes at every authored length.
    ///
    /// Sugar over [`Formula::written_length`] and
    /// [`quantity::WrittenLength::in_unit`], and nothing besides: it
    /// stores the notation the same way, refuses exactly what
    /// `written_length` refuses, and mints no type of its own. The two
    /// halves stay the doors — reach for them when the
    /// [`quantity::WrittenLength`] is already in hand.
    ///
    /// # Errors
    ///
    /// [`DimensionError::NonFiniteLiteral`] for a non-finite value.
    pub fn length_in(value: f64, unit: quantity::LengthUnit) -> Result<Self, DimensionError> {
        Self::written_length(quantity::WrittenLength::in_unit(value, unit))
    }

    /// A continuous literal from an angle authored as `value` in
    /// `unit` — [`Formula::length_in`]'s mirror, exactly
    /// `Formula::written_angle(WrittenAngle::in_unit(value, unit))`, and
    /// everything that door's docs say holds here with an
    /// [`quantity::AngleUnit`].
    ///
    /// # Errors
    ///
    /// [`DimensionError::NonFiniteLiteral`] for a non-finite value.
    pub fn angle_in(value: f64, unit: quantity::AngleUnit) -> Result<Self, DimensionError> {
        Self::written_angle(quantity::WrittenAngle::in_unit(value, unit))
    }
    /// A `Count` literal — an exact integer.
    pub fn count(value: i64) -> Self {
        Self::count_leaf(value)
    }

    /// A variable by name, read at `dim`: lowered to a reader of the
    /// variable the document names so ([`Formula::lower`]).
    pub fn named(name: VarName, dim: Dimension) -> Self {
        Self::own_leaf(AuthoredLeaf::Name(name), dim)
    }

    /// The names this formula reads, with the dimension each is read
    /// at, in pre-order.
    pub fn named_reads(&self, out: &mut Vec<(VarName, Dimension)>) {
        self.visit_leaves(&mut |AuthoredLeaf::Name(name), dim| out.push((name.clone(), dim)));
    }

    /// **This formula with every name `scope` lowers lowered**, the
    /// rest left as written: a name read at the kind of the variable
    /// `scope` names so is that variable's reader, and any other name
    /// stays ([`Formula::lower`]'s rule, partial). What a refusal
    /// speaks a node by, so a node authored by name and the same node
    /// authored by id speak one id however much of it lowers.
    pub(crate) fn lower_held(
        &self,
        scope: &impl Fn(&VarName) -> Option<(VarId, Dimension)>,
    ) -> Formula {
        let Ok(held) = self.try_map_leaves(&mut |AuthoredLeaf::Name(name), dim| {
            Ok::<_, core::convert::Infallible>(match scope(name) {
                Some((var, declared)) if declared == dim => Formula::var(var, dim),
                _ => Formula::named(name.clone(), dim),
            })
        });
        held
    }

    /// **The stored expression this formula lowers to** in `scope`: every
    /// name leaf a reader of the variable `scope` resolves it to, where
    /// that variable's kind is the dimension the leaf reads it at — the
    /// one lowering rule. Nothing is minted.
    ///
    /// # Errors
    ///
    /// The first name leaf, in pre-order, that `scope` does not hold or
    /// holds at another kind ([`NameFault`]).
    pub fn lower(
        &self,
        scope: &impl Fn(&VarName) -> Option<(VarId, Dimension)>,
    ) -> Result<Expr, NameFault> {
        self.try_map_leaves(&mut |leaf, dim| {
            let AuthoredLeaf::Name(name) = leaf;
            match scope(name) {
                Some((var, declared)) if declared == dim => Ok(Expr::var(var, dim)),
                Some((var, declared)) => Err(NameFault {
                    name: name.clone(),
                    dim,
                    why: Unlowered::Kind { var, declared },
                }),
                None => Err(NameFault {
                    name: name.clone(),
                    dim,
                    why: Unlowered::Unheld,
                }),
            }
        })
    }
}

/// **Re-authoring**: a stored expression is a formula with no name
/// leaf, reading every variable by id.
impl From<&Expr> for Formula {
    fn from(stored: &Expr) -> Self {
        let Ok(formula) = stored.try_map_leaves(&mut |leaf: &StoredLeaf,
                                                      _|
         -> Result<
            Self,
            core::convert::Infallible,
        > { match *leaf {} });
        formula
    }
}

impl From<Expr> for Formula {
    fn from(stored: Expr) -> Self {
        Self::from(&stored)
    }
}

/// **A stored expression equals the formula that re-authors it**: the
/// same tree, reading the same variables by id and holding no name.
/// The comparison `PartialEq` makes within either form (IEEE on
/// literals, display units unread).
impl PartialEq<Formula> for Expr {
    fn eq(&self, formula: &Formula) -> bool {
        Self::try_from(formula).is_ok_and(|stored| *self == stored)
    }
}

impl PartialEq<Expr> for Formula {
    fn eq(&self, stored: &Expr) -> bool {
        stored == self
    }
}

/// A formula with no name leaf is already stored: the lowering in a
/// scope that holds no name.
impl TryFrom<&Formula> for Expr {
    type Error = NameFault;
    fn try_from(formula: &Formula) -> Result<Self, NameFault> {
        formula.lower(&|_| None)
    }
}

impl TryFrom<Formula> for Expr {
    type Error = NameFault;
    fn try_from(formula: Formula) -> Result<Self, NameFault> {
        Self::try_from(&formula)
    }
}

/// The formula's text ([`crate::unparse`]), a reader by id written as
/// its full id.
impl core::fmt::Display for Formula {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&crate::expr::unparse(self, &|_| None))
    }
}
