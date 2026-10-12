//! **The authored expression** (VARIABLES-DESIGN VR6): what a person
//! or a caller writes into a slot or a definition, before the edit door
//! stores it.
//!
//! A [`Formula`] is the same dimension-checked tree as the stored
//! [`Expr`], with the leaves only an author writes beside the shared
//! ones: a variable by name ([`Formula::named`]), a fresh-table entry
//! ([`Formula::fresh`]) and a written quantity ([`Formula::length_in`]
//! and its siblings). The edit door lowers it against the document's
//! names, minting an anonymous free variable for each written quantity
//! ([`Formula::lower_with`]), and the stored form has none of those
//! leaves to hold, so a stored document reads every variable by id and
//! holds no float in an expression, as facts of its types.

use crate::doc::VarName;
use crate::expr::{
    AuthoredLeaf, Dimension, DimensionError, Expr, ExprKind, ExprTree, Quantity, StoredLeaf,
    Terminal, UnitSym, Unlowered,
};
use crate::var::{VarId, VarKind};

/// The authored expression: the shared leaves, plus a variable by name,
/// a fresh-table entry and a written quantity.
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
                crate::sentence::article(&declared.to_string()),
                dim.article()
            ),
        }
    }
}

impl core::error::Error for NameFault {}

/// **A fresh-table read the lowering did not resolve**, the first in
/// pre-order: the index, the dimension the leaf reads it at, and the
/// dimension the entry holds, `None` where the table holds no entry
/// `index`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreshFault {
    /// The table index the leaf reads.
    pub index: u16,
    /// The dimension the leaf reads it at.
    pub dim: Dimension,
    /// The dimension the entry holds, if the table holds it.
    pub held: Option<Dimension>,
}

impl core::fmt::Display for FreshFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { index, dim, held } = self;
        match held {
            None => write!(f, "the edit's fresh table holds no entry {index}"),
            Some(held) => write!(
                f,
                "fresh entry {index} is {} {held}, read here as {} {dim}",
                held.article(),
                dim.article()
            ),
        }
    }
}

/// **Why a formula did not lower**: a name, a fresh-table read, or a
/// written quantity lowered where nothing mints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LowerFault {
    /// A name leaf the scope does not resolve.
    Name(NameFault),
    /// A fresh leaf the edit's table does not resolve.
    Fresh(FreshFault),
    /// A written quantity, read at `dim`, where the lowering mints no
    /// variable for it: only an edit's door mints.
    Quantity {
        /// The dimension the quantity is read at.
        dim: Dimension,
    },
}

impl core::fmt::Display for LowerFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Name(fault) => fault.fmt(f),
            Self::Fresh(fault) => fault.fmt(f),
            Self::Quantity { dim } => write!(
                f,
                "a written {dim} is a variable the edit door mints, and nothing mints here"
            ),
        }
    }
}

impl core::error::Error for LowerFault {}

impl From<NameFault> for LowerFault {
    fn from(fault: NameFault) -> Self {
        Self::Name(fault)
    }
}

/// **What a formula is at a slot's root** (VARIABLES-DESIGN VR4, VR6):
/// what the edit door stores for it.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum SlotRoot<'a> {
    /// A lone variable, by id: the slot reads it.
    Var(VarId),
    /// A lone variable, by name, read at `dim`.
    Name(&'a VarName, Dimension),
    /// A lone fresh-table entry, read at `dim`.
    Fresh(u16, Dimension),
    /// A lone written value: the slot reads a fresh anonymous free
    /// variable holding it.
    Value(crate::doc::FreeVar),
    /// Anything else: the slot reads a fresh anonymous defined
    /// variable.
    Formula,
}

// The constructors share names with the std ops traits on purpose, as
// `Expr`'s do.
#[allow(clippy::should_implement_trait)]
impl Formula {
    /// A written quantity in canonical kernel units, remembering the
    /// canonical unit for its dimension. Refuses [`Dimension::Count`] —
    /// a count is an integer ([`Formula::count`]) — and NON-FINITE
    /// values (ruled door 1 of the non-finite policy: the kernel never
    /// produces NaN/inf legitimately, so recipe data must not admit
    /// them).
    pub fn literal(value: f64, dim: Dimension) -> Result<Self, DimensionError> {
        if dim == Dimension::Count {
            return Err(DimensionError::LiteralCountIsInteger);
        }
        Self::literal_with_unit(value, dim, UnitSym::canonical_for(dim).def())
    }

    /// A written quantity that REMEMBERS the display unit it was
    /// authored in (LIB-SWITCH §4g; the text door's `25 mm` row).
    /// `value` is already canonical (meters/radians) — the parser does
    /// its one multiply before this door. The unit's quantity must
    /// agree with `dim` ([`DimensionError::DisplayUnitMismatch`]);
    /// everything [`Formula::literal`] refuses is refused here too.
    ///
    /// The unit is presentation metadata (DESIGN.md D6): it round-trips
    /// through persistence and feeds the display formatter, but never
    /// enters [`Formula::bit_eq`], [`Formula::literal_bits`], content
    /// keys, or evaluation.
    pub fn literal_with_unit(
        value: f64,
        dim: Dimension,
        unit: quantity::UnitDef,
    ) -> Result<Self, DimensionError> {
        let unit = UnitSym::checked_for(dim, unit)?;
        if dim == Dimension::Count {
            return Err(DimensionError::LiteralCountIsInteger);
        }
        if !value.is_finite() {
            return Err(DimensionError::NonFiniteLiteral);
        }
        Ok(Self::quantity_leaf(value, dim, unit))
    }

    /// The written quantity `value` in `unit`, read at `dim`, unchecked.
    fn quantity_leaf(value: f64, dim: Dimension, unit: UnitSym) -> Self {
        Self::own_leaf(
            AuthoredLeaf::Quantity(Box::new(Quantity {
                value,
                unit,
                distribution: None,
            })),
            dim,
        )
    }

    /// A written dimensionless value: a `Scalar` quantity with the
    /// dimensionless unit, which the edit door mints a variable for —
    /// at a slot's root as any value, and inside a formula as any
    /// written quantity (VR5, VR6). The exact constant is
    /// [`Formula::ratio`].
    ///
    /// # Errors
    ///
    /// [`DimensionError::NonFiniteLiteral`] for a non-finite value.
    pub fn scalar(value: f64) -> Result<Self, DimensionError> {
        Self::literal(value, Dimension::Scalar)
    }

    /// A written quantity from an AUTHORED length — the value and
    /// the notation it was written in, together
    /// ([`quantity::WrittenLength`]).
    ///
    /// The door library and GUI authoring should reach for. A caller
    /// never spells the dimension — a `WrittenLength` is a length, so
    /// there is no second fact to keep in step — and the quantity
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

    /// A written quantity from an AUTHORED angle —
    /// [`Formula::written_length`]'s mirror, and everything that door's
    /// docs say holds here with `AngleUnit` and `Dimension::Angle`.
    ///
    /// # Errors
    ///
    /// [`DimensionError::NonFiniteLiteral`] for a non-finite value.
    pub fn written_angle(written: quantity::WrittenAngle) -> Result<Self, DimensionError> {
        Self::literal_with_unit(written.radians(), Dimension::Angle, written.unit().def())
    }

    /// A written quantity from a length authored as `value` in
    /// `unit` — exactly
    /// `Formula::written_length(WrittenLength::in_unit(value, unit))`,
    /// the composition an authoring caller holding a number and a unit
    /// writes at every authored length.
    ///
    /// # Errors
    ///
    /// [`DimensionError::NonFiniteLiteral`] for a non-finite value.
    pub fn length_in(value: f64, unit: quantity::LengthUnit) -> Result<Self, DimensionError> {
        Self::written_length(quantity::WrittenLength::in_unit(value, unit))
    }

    /// A written quantity from an angle authored as `value` in
    /// `unit` — [`Formula::length_in`]'s mirror.
    ///
    /// # Errors
    ///
    /// [`DimensionError::NonFiniteLiteral`] for a non-finite value.
    pub fn angle_in(value: f64, unit: quantity::AngleUnit) -> Result<Self, DimensionError> {
        Self::written_angle(quantity::WrittenAngle::in_unit(value, unit))
    }

    /// The integer `value`: a constant inside a formula, and at a
    /// slot's root a written count, which mints a free `Count`
    /// variable (VR6).
    pub fn count(value: i64) -> Self {
        Self::integer(value)
    }

    /// This lone written quantity carrying `distribution` (ERROR-DESIGN
    /// E1/E2): the variable the door mints for it carries it. A lone
    /// rational constant given a distribution becomes the written value
    /// it equals, since a toleranced number is a value and not a
    /// constant; given none, it stays the constant.
    ///
    /// # Errors
    ///
    /// [`crate::DistributionRefusal::CountHasNoAnnotation`] for a lone
    /// integer, [`crate::DistributionRefusal::NotAWrittenValue`] where
    /// this is not one written quantity or number, and
    /// [`crate::DistributionRefusal::Invalid`] for a distribution that
    /// breaks an E2 invariant.
    pub fn with_distribution(
        mut self,
        distribution: Option<crate::distribution::Distribution>,
    ) -> Result<Self, crate::DistributionRefusal> {
        if let Some(d) = &distribution
            && let Err(fault) = d.check()
        {
            return Err(crate::DistributionRefusal::Invalid { fault });
        }
        match self.kind_mut() {
            ExprKind::Leaf(AuthoredLeaf::Quantity(q)) => {
                q.distribution = distribution;
                Ok(self)
            }
            ExprKind::Ratio(_) if distribution.is_none() => Ok(self),
            // A toleranced number is a written value, not a constant.
            &mut ExprKind::Ratio(ratio) => {
                let mut value = Self::quantity_leaf(
                    ratio.eval(),
                    Dimension::Scalar,
                    UnitSym::canonical_for(Dimension::Scalar),
                );
                if let ExprKind::Leaf(AuthoredLeaf::Quantity(q)) = value.kind_mut() {
                    q.distribution = distribution;
                }
                Ok(value)
            }
            ExprKind::Integer(_) => Err(crate::DistributionRefusal::CountHasNoAnnotation),
            _ => Err(crate::DistributionRefusal::NotAWrittenValue),
        }
    }

    /// This lone written quantity with its distribution set to
    /// `distribution`, unchecked: the door that mints its variable
    /// checks it.
    ///
    /// # Errors
    ///
    /// [`crate::DistributionRefusal::NotAWrittenValue`] where this is
    /// not one written quantity and `distribution` is one — there is
    /// nothing to carry it.
    pub(crate) fn carrying(
        mut self,
        distribution: Option<crate::distribution::Distribution>,
    ) -> Result<Self, crate::DistributionRefusal> {
        match self.kind_mut() {
            ExprKind::Leaf(AuthoredLeaf::Quantity(q)) => q.distribution = distribution,
            _ if distribution.is_some() => {
                return Err(crate::DistributionRefusal::NotAWrittenValue);
            }
            _ => {}
        }
        Ok(self)
    }

    /// Pushes the `f64` BITS of every written quantity's value, in
    /// pre-order (children in [`Formula::child`] order): the
    /// bit-semantic comparison substrate (spec D7).
    pub fn literal_bits(&self, out: &mut Vec<u64>) {
        self.own_bits(out);
    }

    /// The written quantity this formula is, where it is one alone.
    pub fn as_quantity(&self) -> Option<&Quantity> {
        match self.kind() {
            ExprKind::Leaf(AuthoredLeaf::Quantity(q)) => Some(q),
            _ => None,
        }
    }

    /// The exact canonical-units value of a lone written quantity, or
    /// of a lone dimensionless number (its correctly-rounded double);
    /// `None` for any other formula — with [`Formula::display_unit`],
    /// the display formatter's read surface.
    pub fn literal_value(&self) -> Option<f64> {
        match self.kind() {
            ExprKind::Leaf(AuthoredLeaf::Quantity(q)) => Some(q.value()),
            ExprKind::Ratio(ratio) => Some(ratio.eval()),
            _ => None,
        }
    }

    /// The display unit of a lone written quantity, and the
    /// dimensionless row for a lone dimensionless number — `None` for
    /// every other formula, because only a value is WRITTEN in a unit.
    pub fn display_unit(&self) -> Option<quantity::UnitDef> {
        match self.kind() {
            ExprKind::Leaf(AuthoredLeaf::Quantity(q)) => Some(q.unit()),
            ExprKind::Ratio(_) => Some(UnitSym::canonical_for(Dimension::Scalar).def()),
            _ => None,
        }
    }

    /// A variable by name, read at `dim`: lowered to a reader of the
    /// variable the document names so ([`Formula::lower`]).
    pub fn named(name: VarName, dim: Dimension) -> Self {
        Self::own_leaf(AuthoredLeaf::Name(name), dim)
    }

    /// Entry `index` of the edit's fresh table, read at `dim`: lowered
    /// to a reader of the variable the edit mints for that entry.
    pub fn fresh(index: u16, dim: Dimension) -> Self {
        Self::own_leaf(AuthoredLeaf::Fresh(index), dim)
    }

    /// What this formula is at a slot's root: what the door stores.
    pub(crate) fn slot_root(&self) -> SlotRoot<'_> {
        use crate::expr::ExprKind as K;
        match self.kind() {
            K::Var(var) => SlotRoot::Var(*var),
            K::Leaf(AuthoredLeaf::Name(name)) => SlotRoot::Name(name, self.dim()),
            K::Leaf(AuthoredLeaf::Fresh(index)) => SlotRoot::Fresh(*index, self.dim()),
            K::Leaf(AuthoredLeaf::Quantity(q)) => SlotRoot::Value(q.free_var(self.dim())),
            // A lone number is a typed value, not a constant (Q1).
            K::Integer(value) => SlotRoot::Value(crate::doc::FreeVar::Count { value: *value }),
            K::Ratio(ratio) => SlotRoot::Value(crate::doc::FreeVar::Continuous {
                dim: Dimension::Scalar,
                value: ratio.eval(),
                display_unit: UnitSym::canonical_for(Dimension::Scalar),
                distribution: None,
            }),
            _ => SlotRoot::Formula,
        }
    }

    /// The names this formula reads, with the dimension each is read
    /// at, in pre-order.
    pub fn named_reads(&self, out: &mut Vec<(VarName, Dimension)>) {
        self.visit_leaves(&mut |leaf, dim| {
            if let AuthoredLeaf::Name(name) = leaf {
                out.push((name.clone(), dim));
            }
        });
    }

    /// **This formula with every name `scope` lowers lowered**, the
    /// rest left as written: a name read at the kind of the variable
    /// `scope` names so is that variable's reader, and any other name
    /// stays ([`Formula::lower`]'s rule, partial). What a refusal
    /// speaks a node by, so a node authored by name and the same node
    /// authored by id speak one id however much of it lowers. A fresh
    /// leaf stays as written: nothing is minted yet.
    pub(crate) fn lower_held(
        &self,
        scope: &impl Fn(&VarName) -> Option<(VarId, VarKind)>,
    ) -> Formula {
        let Ok(held) = self.try_map_leaves(&mut |leaf, dim| {
            Ok::<_, core::convert::Infallible>(match leaf {
                AuthoredLeaf::Name(name) => match scope(name) {
                    Some((var, declared)) if declared.dimension() == Some(dim) => {
                        Formula::var(var, dim)
                    }
                    _ => Formula::named(name.clone(), dim),
                },
                AuthoredLeaf::Fresh(index) => Formula::fresh(*index, dim),
                AuthoredLeaf::Quantity(q) => {
                    Formula::own_leaf(AuthoredLeaf::Quantity(q.clone()), dim)
                }
            })
        });
        held
    }

    /// **The written quantities this formula holds**, with the
    /// dimension each is read at, in pre-order: what the edit door
    /// mints, one anonymous free variable each, before it lowers the
    /// formula ([`Self::lower_with`]).
    pub(crate) fn quantities(&self) -> Vec<crate::doc::FreeVar> {
        let mut out = Vec::new();
        self.visit_leaves(&mut |leaf, dim| {
            if let AuthoredLeaf::Quantity(q) = leaf {
                out.push(q.free_var(dim));
            }
        });
        out
    }

    /// **The variables this formula reads once lowered**, with the
    /// dimension each is read at, in pre-order: every reader by id, and
    /// every name and fresh entry that lowers, read as the variable it
    /// lowers to. A written quantity reads nothing yet: the variable it
    /// lowers to is minted for it.
    pub(crate) fn lowered_reads(
        &self,
        scope: &impl Fn(&VarName) -> Option<(VarId, VarKind)>,
        fresh: &[(VarId, Dimension)],
    ) -> Vec<(VarId, Dimension)> {
        let mut out = Vec::new();
        self.visit_terminals(&mut |terminal, dim| match terminal {
            Terminal::Var(var) => out.push((var, dim)),
            Terminal::Leaf(leaf) => out.extend(
                leaf_fault(leaf, dim, scope, fresh)
                    .ok()
                    .map(|var| (var, dim)),
            ),
            Terminal::Constant => {}
        });
        out
    }

    /// **The first leaf only a document's edit door resolves**, in
    /// pre-order — a name, or a fresh-table read — as it refuses where
    /// no name is held and no table is read; `None` for a formula that
    /// evaluates as it stands ([`crate::eval`]).
    #[must_use]
    pub fn unresolvable(&self) -> Option<LowerFault> {
        self.unresolved(&|_| None, &[])
    }

    /// **The first name or fresh read that does not lower**, in
    /// pre-order — what [`Self::lower_with`] would refuse, asked before
    /// anything is minted for the formula's quantities.
    pub(crate) fn unresolved(
        &self,
        scope: &impl Fn(&VarName) -> Option<(VarId, VarKind)>,
        fresh: &[(VarId, Dimension)],
    ) -> Option<LowerFault> {
        let mut first = None;
        self.visit_leaves(&mut |leaf, dim| {
            if first.is_none() && !matches!(leaf, AuthoredLeaf::Quantity(_)) {
                first = leaf_fault(leaf, dim, scope, fresh).err();
            }
        });
        first
    }

    /// **The stored expression this formula lowers to** in `scope`: every
    /// name leaf a reader of the variable `scope` resolves it to, where
    /// that variable's kind is the dimension the leaf reads it at — the
    /// one lowering rule. Nothing is minted, so neither a fresh-table
    /// read nor a written quantity lowers ([`Self::lower_with`]).
    ///
    /// # Errors
    ///
    /// The first leaf, in pre-order, that does not lower.
    pub fn lower(
        &self,
        scope: &impl Fn(&VarName) -> Option<(VarId, VarKind)>,
    ) -> Result<Expr, LowerFault> {
        self.lower_with(scope, &[], &[])
    }

    /// [`Self::lower`], each fresh leaf `i` a reader of `fresh[i]`, the
    /// variable the edit minted for entry `i`, with its dimension, and
    /// the `k`-th written quantity, in pre-order, a reader of
    /// `minted[k]`, the variable the door minted for it
    /// ([`Self::quantities`]).
    ///
    /// # Errors
    ///
    /// The first leaf, in pre-order, that does not lower.
    pub(crate) fn lower_with(
        &self,
        scope: &impl Fn(&VarName) -> Option<(VarId, VarKind)>,
        fresh: &[(VarId, Dimension)],
        minted: &[VarId],
    ) -> Result<Expr, LowerFault> {
        let mut quantities = minted.iter();
        self.try_map_leaves(&mut |leaf, dim| match leaf {
            AuthoredLeaf::Quantity(_) => quantities
                .next()
                .map(|&var| Expr::var(var, dim))
                .ok_or(LowerFault::Quantity { dim }),
            other => leaf_fault(other, dim, scope, fresh).map(|var| Expr::var(var, dim)),
        })
    }
}

/// **The variable a name or fresh leaf lowers to**, read at `dim`, or
/// why it does not.
fn leaf_fault(
    leaf: &AuthoredLeaf,
    dim: Dimension,
    scope: &impl Fn(&VarName) -> Option<(VarId, VarKind)>,
    fresh: &[(VarId, Dimension)],
) -> Result<VarId, LowerFault> {
    match leaf {
        AuthoredLeaf::Name(name) => match scope(name) {
            Some((var, declared)) if declared.dimension() == Some(dim) => Ok(var),
            Some((var, declared)) => Err(LowerFault::Name(NameFault {
                name: name.clone(),
                dim,
                why: Unlowered::Kind { var, declared },
            })),
            None => Err(LowerFault::Name(NameFault {
                name: name.clone(),
                dim,
                why: Unlowered::Unheld,
            })),
        },
        &AuthoredLeaf::Fresh(index) => match fresh.get(usize::from(index)) {
            Some(&(var, held)) if held == dim => Ok(var),
            held => Err(LowerFault::Fresh(FreshFault {
                index,
                dim,
                held: held.map(|&(_, held)| held),
            })),
        },
        AuthoredLeaf::Quantity(_) => Err(LowerFault::Quantity { dim }),
    }
}

/// **Re-authoring**: a stored expression is a formula with no authored
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
/// same tree, reading the same variables by id and holding no authored
/// leaf.
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

/// A formula with no authored leaf is already stored: the lowering in
/// a scope that holds no name, minting nothing.
impl TryFrom<&Formula> for Expr {
    type Error = LowerFault;
    fn try_from(formula: &Formula) -> Result<Self, LowerFault> {
        formula.lower(&|_| None)
    }
}

impl TryFrom<Formula> for Expr {
    type Error = LowerFault;
    fn try_from(formula: Formula) -> Result<Self, LowerFault> {
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
