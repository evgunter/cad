//! The unit table: symbols, quantities, and exact conversion factors
//! into canonical kernel units — as DATA, shaped after the one prior
//! unit table in the tree (step-import's `units.rs` prefix/factor
//! tables).
//!
//! Factors are canonical-units-per-one-of-this-unit:
//!
//! * `MM`/`CM` are PREFIXED METRES — the factor is the SI prefix
//!   ([`MILLI`], [`CENTI`]), each the correctly-rounded f64 of its
//!   exact power of ten. Lengths take prefixes; angles do not (no
//!   `mdeg` row can exist here) — the census's lengths-take-prefixes
//!   rule, mirrored from step-import's `si_unit_kind`, where a
//!   prefixed `RADIAN` refuses.
//! * `IN` is EXACTLY 25.4 mm (the international inch): the factor is
//!   the correctly-rounded f64 of the exact decimal `0.0254`.
//! * `DEG` is π/180 — INEXACT BY NATURE: π has no finite decimal, so
//!   the stored factor is the f64 rounding of `f64::consts::PI / 180`.
//!   Every degree quantity is therefore canonical-radians data whose
//!   last-ulp identity is defined by this constant (bit-stable: the
//!   constant is a compile-time literal expression, not a runtime
//!   computation).
//! * `M`/`RAD` are the canonical units themselves, factor exactly 1.0
//!   — multiplication by them is the f64 identity, which is what makes
//!   them the formatter's always-exact fallback rendering.

/// The SI prefix factor 10⁻³ (exact decimal, correctly rounded f64) —
/// `MM`'s factor. Prefixes apply to the metre only (module docs).
pub const MILLI: f64 = 1e-3;

/// The SI prefix factor 10⁻² — `CM`'s factor.
pub const CENTI: f64 = 1e-2;

/// What a unit measures — the continuous half of the dimension set
/// (`Count` has no units: a count is a bare integer; `Scalar` is
/// dimensionless by definition).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitQuantity {
    /// A length; factors carry into canonical meters.
    Length,
    /// A plane angle; factors carry into canonical radians.
    Angle,
}


/// One row of the unit table: a display/parse symbol, the quantity it
/// measures, and the exact factor into canonical units.
///
/// **SEALED — a `UnitDef` is a row of [`UNITS`], not a triple of three
/// independent fields (issue #650).** The fields are private and there
/// is no constructor at all — not a public one, not a `#[doc(hidden)]`
/// test-only one. The only values of this type a caller can hold are
/// copies of the table's own rows: [`UNITS`] itself, [`unit_by_symbol`],
/// and whatever hands one back (`Expr::display_unit`).
///
/// **The invariant this buys, which downstream crates rely on: the
/// symbol DETERMINES the other two fields.** A row whose `symbol` is
/// `"mm"` and whose `quantity` is `Angle` is not merely refused, it is
/// unrepresentable. That is what closes #650, where such a row defeated
/// `Expr::literal_with_unit`'s dimension guard — the guard read the
/// CALLER's `quantity`, then stored the table's row unchecked — and
/// produced an `Expr` that serialized into a document editor-core's own
/// load door then refused (`DisplayUnitMismatch`): a round-trip break,
/// not a rejected call. D2 addendum: the answer chosen is "make the
/// illegal state unrepresentable", not row 1's typed error, because the
/// input was never legitimate in the first place.
///
/// The two typed views [`LengthUnit`] and [`AngleUnit`] are sealed the
/// same way and by the same mechanism — each WRAPS a row of this table
/// (issue #669), so "a typed view of a `UnitDef` row" is what the type
/// literally is rather than a claim about how it is used. [`as_length`]
/// and [`as_angle`] are the public route from a row to its view;
/// `LengthUnit::def` / `AngleUnit::def` are the inverse and stay
/// `pub(crate)` only because nothing outside the crate needs them —
/// the seal no longer depends on their visibility.
///
/// **This rustdoc is the one home of that narrative.** Everything else
/// that touches the seal points here rather than restating it.
///
/// The seal is a claim about what COMPILES, so it is pinned by rows
/// that must fail to compile. These are library doctests — they run
/// under `cargo test -p quantity --doc`, so undoing the seal turns them
/// red rather than merely dating a comment.
///
/// #650's literal counterexample no longer builds:
///
/// ```compile_fail
/// let bogus = quantity::UnitDef {
///     symbol: "mm",
///     quantity: quantity::UnitQuantity::Angle,
///     factor: 1.0,
/// };
/// ```
///
/// Nor does the struct-update escape from a real row:
///
/// ```compile_fail
/// let mm = quantity::unit_by_symbol("mm").unwrap();
/// let bogus = quantity::UnitDef { quantity: quantity::UnitQuantity::Angle, ..mm };
/// ```
///
/// Nor the route through the typed views, which since #669 have no
/// public constructor either — so there is no second mint to demote:
///
/// ```compile_fail
/// let bogus = quantity::AngleUnit { symbol: "mm", factor: 1.0 };
/// ```
///
/// A `compile_fail` row proves only that the snippet does not build,
/// not that it fails for the intended reason — a typo would pass it
/// just as well. Each block above therefore has its legal twin here,
/// differing from it by exactly the illegal step:
///
/// ```
/// // Twin of blocks 1 and 2: the path `quantity::UnitDef` names this
/// // exported type, the row and the enum variant exist, and every
/// // field is READABLE through its accessor. Naming the type binds
/// // the twin to it, so a rename or an un-export reddens here rather
/// // than quietly satisfying the `compile_fail` blocks for the wrong
/// // reason.
/// let mm: quantity::UnitDef = quantity::unit_by_symbol("mm").expect("mm is a table row");
/// assert_eq!(mm.symbol(), "mm");
/// assert_eq!(mm.quantity(), quantity::UnitQuantity::Length);
/// assert_eq!(mm.factor(), quantity::MILLI);
/// assert_eq!(quantity::UnitQuantity::Angle, quantity::UnitQuantity::Angle);
///
/// // Twin of block 3: `quantity::AngleUnit` is exported, its two
/// // readers exist, and the ONLY difference from the illegal step is
/// // where the value came from — the table, not a field list.
/// let view: quantity::AngleUnit = quantity::DEG;
/// assert_eq!(view.symbol(), "deg");
/// assert_eq!(view.factor(), core::f64::consts::PI / 180.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitDef {
    symbol: &'static str,
    quantity: UnitQuantity,
    factor: f64,
}

impl UnitDef {
    /// The surface symbol (`"mm"`), as parsed and as displayed.
    pub const fn symbol(&self) -> &'static str {
        self.symbol
    }

    /// The quantity this unit measures.
    pub const fn quantity(&self) -> UnitQuantity {
        self.quantity
    }

    /// Canonical units (meters/radians) per one of this unit.
    pub const fn factor(&self) -> f64 {
        self.factor
    }

    /// This row as a [`LengthUnit`], or `None` when the row measures an
    /// angle — the public route from a table row to the typed view that
    /// builds a [`crate::Length`].
    ///
    /// Partial, and necessarily so: the quantity is what decides which
    /// view a row has, and it is data. The refusal is the only place a
    /// dimension is still CHECKED rather than typed, and it can only
    /// answer `None` for a row whose own `quantity` says so — never for
    /// a caller-assembled pair, which no longer exists.
    pub const fn as_length(self) -> Option<LengthUnit> {
        match self.quantity {
            UnitQuantity::Length => Some(LengthUnit(self)),
            UnitQuantity::Angle => None,
        }
    }

    /// This row as an [`AngleUnit`], or `None` when the row measures a
    /// length — the mirror of [`UnitDef::as_length`].
    pub const fn as_angle(self) -> Option<AngleUnit> {
        match self.quantity {
            UnitQuantity::Angle => Some(AngleUnit(self)),
            UnitQuantity::Length => None,
        }
    }
}

/// A length unit — a typed view of a [`UnitDef`] row, so `25.0 * MM`
/// can only build a [`crate::Length`] (never an angle), and the row it
/// views is a row of [`UNITS`].
///
/// **SEALED, by wrapping the row rather than copying it (issue #669) —
/// the narrative lives on [`UnitDef`].** Both halves of the pair are
/// now typed: the DIMENSION by which view this is, the symbol/factor
/// pairing by the table. Obtain one from a unit constant ([`MM`],
/// [`CM`], [`M`], [`IN`]) or from [`UnitDef::as_length`].
///
/// **Every public entry point that takes one trusted the caller's
/// `symbol` and `factor` before the seal**, and this is the whole list:
/// [`crate::fmt_length`], [`crate::Length::in_unit`], and the two `Mul`
/// impls (`f64 * LengthUnit`, `LengthUnit * f64`) — with
/// [`crate::fmt_angle`], [`crate::Angle::in_unit`] and the two mirrored
/// `Mul` impls on [`AngleUnit`]. None of them checks anything now
/// either; what changed is that there is no longer an unchecked value
/// to hand them.
///
/// What that closed, concretely: `25.0 * LengthUnit { symbol: "mm",
/// factor: 1.0 }` was 25 METRES carrying the label `mm`, at the D6
/// typed-units boundary — a wrong VALUE, not a mislabelled string. And
/// it reached a document, since `fmt.rs`'s stated pin is
/// `parse(fmt(x, unit))`: `fmt_length(0.025, LengthUnit { symbol:
/// "deg", factor: 1e-3 })` rendered `"25 deg"`, which parses to an
/// [`crate::Angle`] literal.
///
/// The seal is a claim about what COMPILES, so it is pinned by rows
/// that must fail to compile, each with the legal twin that differs
/// from it by exactly the illegal step.
///
/// The mislabelled-value mint no longer builds:
///
/// ```compile_fail
/// let bogus = quantity::LengthUnit { symbol: "mm", factor: 1.0 };
/// ```
///
/// Nor does the struct-update escape from a real unit constant, which
/// is the form a seal on the constructor alone would leave open:
///
/// ```compile_fail
/// let bogus = quantity::LengthUnit { factor: 1.0, ..quantity::MM };
/// ```
///
/// Nor the cross-quantity mint the formatter round trip turned into an
/// [`crate::Angle`] in a document:
///
/// ```compile_fail
/// let bogus = quantity::LengthUnit { symbol: "deg", factor: 1e-3 };
/// ```
///
/// The legal twin of all three: the type is exported, both readers
/// exist, and the only difference is that the value comes from the
/// table.
///
/// ```
/// let mm: quantity::LengthUnit = quantity::MM;
/// assert_eq!(mm.symbol(), "mm");
/// assert_eq!(mm.factor(), quantity::MILLI);
/// assert_eq!((25.0 * mm).meters(), 0.025);
///
/// // And the row → view door, which is the other way in.
/// let row = quantity::unit_by_symbol("mm").expect("mm is a table row");
/// assert_eq!(row.as_length(), Some(quantity::MM));
/// assert_eq!(row.as_angle(), None);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LengthUnit(UnitDef);

/// An angle unit — the typed view for [`crate::Angle`] construction,
/// wrapping a row of [`UNITS`].
///
/// **SEALED for the reason and by the mechanism on [`LengthUnit`]**;
/// obtain one from [`DEG`], [`RAD`], or [`UnitDef::as_angle`].
///
/// The mint that made `90.0 * AngleUnit { symbol: "deg", factor: 1.0 }`
/// ninety RADIANS labelled `deg` no longer builds:
///
/// ```compile_fail
/// let bogus = quantity::AngleUnit { symbol: "deg", factor: 1.0 };
/// ```
///
/// Its legal twin:
///
/// ```
/// let deg: quantity::AngleUnit = quantity::DEG;
/// assert_eq!(deg.symbol(), "deg");
/// assert!(((90.0 * deg).radians() - core::f64::consts::FRAC_PI_2).abs() < 1e-15);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngleUnit(UnitDef);

impl LengthUnit {
    /// The surface symbol (`"mm"`), as displayed and as parsed back.
    pub const fn symbol(self) -> &'static str {
        self.0.symbol
    }

    /// Meters per one of this unit.
    pub const fn factor(self) -> f64 {
        self.0.factor
    }

    /// This unit as a table row — the inverse of
    /// [`UnitDef::as_length`], and total, because the view IS a row.
    /// `pub(crate)` because nothing outside the crate needs it, not
    /// because the seal depends on it: since #669 a `LengthUnit` can
    /// only be a table row, so this cannot mint anything
    /// [`unit_by_symbol`] would not also hand out.
    pub(crate) const fn def(self) -> UnitDef {
        self.0
    }
}

impl AngleUnit {
    /// The surface symbol (`"deg"`), as displayed and as parsed back.
    pub const fn symbol(self) -> &'static str {
        self.0.symbol
    }

    /// Radians per one of this unit.
    pub const fn factor(self) -> f64 {
        self.0.factor
    }

    /// This unit as a table row — see [`LengthUnit::def`].
    pub(crate) const fn def(self) -> UnitDef {
        self.0
    }
}

/// The one place a length row is written down; private, so the six
/// constants below are the only length units that exist.
const fn length(symbol: &'static str, factor: f64) -> LengthUnit {
    LengthUnit(UnitDef {
        symbol,
        quantity: UnitQuantity::Length,
        factor,
    })
}

/// The angle mirror of [`length`].
const fn angle(symbol: &'static str, factor: f64) -> AngleUnit {
    AngleUnit(UnitDef {
        symbol,
        quantity: UnitQuantity::Angle,
        factor,
    })
}

/// Millimeter: a `MILLI`-prefixed metre.
pub const MM: LengthUnit = length("mm", MILLI);

/// Centimeter: a `CENTI`-prefixed metre.
pub const CM: LengthUnit = length("cm", CENTI);

/// Meter — the canonical length unit, factor exactly 1.0.
pub const M: LengthUnit = length("m", 1.0);

/// International inch: EXACTLY 25.4 mm (module docs) — the factor is
/// the correctly-rounded f64 of the exact decimal 0.0254.
pub const IN: LengthUnit = length("in", 0.0254);

/// Degree: π/180 radians — inexact by nature (module docs).
pub const DEG: AngleUnit = angle("deg", core::f64::consts::PI / 180.0);

/// Radian — the canonical angle unit, factor exactly 1.0.
pub const RAD: AngleUnit = angle("rad", 1.0);

/// The whole closed unit table, as data — the expression text parser's
/// suffix vocabulary and the formatter's display vocabulary are both
/// exactly this list.
pub const UNITS: [UnitDef; 6] = [MM.def(), CM.def(), M.def(), IN.def(), DEG.def(), RAD.def()];

/// The table row for a surface symbol, or `None` when the symbol is
/// not one of the six.
pub fn unit_by_symbol(symbol: &str) -> Option<UnitDef> {
    UNITS.iter().find(|u| u.symbol == symbol).copied()
}
