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
/// and whatever hands one back (`Expr::display_unit`). The
/// `LengthUnit::def` /
/// `AngleUnit::def` views that BUILD the table are `pub(crate)` for
/// the same reason — `LengthUnit`'s fields are public, so a public
/// `def()` would be a second mint that could stamp any symbol onto any
/// quantity.
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
/// Nor the route through the typed views, whose own fields are still
/// public — an [`AngleUnit`] with any symbol builds fine, but `def()`
/// is `pub(crate)`, so it is not a second mint. Closing that one is
/// what makes the seal a seal rather than a rename:
///
/// ```compile_fail
/// let bogus = quantity::AngleUnit { symbol: "mm", factor: 1.0 }.def();
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
/// // Twin of block 3: the typed view with those exact field values
/// // BUILDS — its fields are public. Only `.def()` is out of reach,
/// // which is the whole of what block 3 pins.
/// let view = quantity::AngleUnit { symbol: "mm", factor: 1.0 };
/// assert_eq!(view.symbol, "mm");
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
}

/// A length unit — a typed view of a [`UnitDef`] row, so `25.0 * MM`
/// can only build a [`crate::Length`] (never an angle).
///
/// **NOT sealed, and asymmetrically so — issue #669.** [`UnitDef`]'s
/// rows are sealed; a *view* of one is not. The DIMENSION is typed and
/// enforced (this type builds a `Length`, full stop); the `symbol` and
/// the `factor` are free, and every public entry point that takes one
/// trusts them: [`crate::fmt_length`] / [`crate::fmt_angle`],
/// [`crate::Length::in_unit`] / [`crate::Angle::in_unit`], and the
/// `Mul` impls.
///
/// The consequence is a wrong VALUE, not a mislabelled string:
/// `25.0 * LengthUnit { symbol: "mm", factor: 1.0 }` is 25 METRES
/// carrying the label `mm`, at the D6 typed-units boundary. And it does
/// reach a document: `fmt.rs`'s stated pin is `parse(fmt(x, unit))`, so
/// `fmt_length(0.025, LengthUnit { symbol: "deg", factor: 1e-3 })` —
/// `Ok("25 deg")` — is parser input by design, and `25 deg` parses to
/// an [`crate::Angle`] literal that enters an `Expr` and a document.
///
/// #669 is the schedule: seal these two the way [`UnitDef`] was sealed,
/// which needs a `UnitDef → LengthUnit` conversion and edits in
/// `pncad-py`. Not done here because #650's fix did not require it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LengthUnit {
    /// The surface symbol.
    pub symbol: &'static str,
    /// Meters per one of this unit.
    pub factor: f64,
}

/// An angle unit — the typed view for [`crate::Angle`] construction.
///
/// **NOT sealed — issue #669**, for the reason and with the
/// consequences on [`LengthUnit`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngleUnit {
    /// The surface symbol.
    pub symbol: &'static str,
    /// Radians per one of this unit.
    pub factor: f64,
}

impl LengthUnit {
    /// This unit as a table row — `pub(crate)`: this is the mint that
    /// BUILDS [`UNITS`], and `LengthUnit`'s own fields are public, so
    /// a public `def()` would re-open the [`UnitDef`] seal by letting
    /// a caller stamp any symbol onto `Length` (issue #650). Callers
    /// outside the crate reach a row through [`unit_by_symbol`].
    pub(crate) const fn def(self) -> UnitDef {
        UnitDef {
            symbol: self.symbol,
            quantity: UnitQuantity::Length,
            factor: self.factor,
        }
    }
}

impl AngleUnit {
    /// This unit as a table row — `pub(crate)` for the reason on
    /// [`LengthUnit::def`].
    pub(crate) const fn def(self) -> UnitDef {
        UnitDef {
            symbol: self.symbol,
            quantity: UnitQuantity::Angle,
            factor: self.factor,
        }
    }
}

/// Millimeter: a `MILLI`-prefixed metre.
pub const MM: LengthUnit = LengthUnit {
    symbol: "mm",
    factor: MILLI,
};

/// Centimeter: a `CENTI`-prefixed metre.
pub const CM: LengthUnit = LengthUnit {
    symbol: "cm",
    factor: CENTI,
};

/// Meter — the canonical length unit, factor exactly 1.0.
pub const M: LengthUnit = LengthUnit {
    symbol: "m",
    factor: 1.0,
};

/// International inch: EXACTLY 25.4 mm (module docs) — the factor is
/// the correctly-rounded f64 of the exact decimal 0.0254.
pub const IN: LengthUnit = LengthUnit {
    symbol: "in",
    factor: 0.0254,
};

/// Degree: π/180 radians — inexact by nature (module docs).
pub const DEG: AngleUnit = AngleUnit {
    symbol: "deg",
    factor: core::f64::consts::PI / 180.0,
};

/// Radian — the canonical angle unit, factor exactly 1.0.
pub const RAD: AngleUnit = AngleUnit {
    symbol: "rad",
    factor: 1.0,
};

/// The whole closed unit table, as data — the expression text parser's
/// suffix vocabulary and the formatter's display vocabulary are both
/// exactly this list.
pub const UNITS: [UnitDef; 6] = [MM.def(), CM.def(), M.def(), IN.def(), DEG.def(), RAD.def()];

/// The table row for a surface symbol, or `None` when the symbol is
/// not one of the six.
pub fn unit_by_symbol(symbol: &str) -> Option<UnitDef> {
    UNITS.iter().find(|u| u.symbol == symbol).copied()
}
