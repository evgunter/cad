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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitDef {
    /// The surface symbol (`"mm"`), as parsed and as displayed.
    pub symbol: &'static str,
    /// The quantity this unit measures.
    pub quantity: UnitQuantity,
    /// Canonical units (meters/radians) per one of this unit.
    pub factor: f64,
}

/// A length unit — a typed view of a [`UnitDef`] row, so `25.0 * MM`
/// can only build a [`crate::Length`] (never an angle).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LengthUnit {
    /// The surface symbol.
    pub symbol: &'static str,
    /// Meters per one of this unit.
    pub factor: f64,
}

/// An angle unit — the typed view for [`crate::Angle`] construction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngleUnit {
    /// The surface symbol.
    pub symbol: &'static str,
    /// Radians per one of this unit.
    pub factor: f64,
}

impl LengthUnit {
    /// This unit as a table row.
    pub const fn def(self) -> UnitDef {
        UnitDef {
            symbol: self.symbol,
            quantity: UnitQuantity::Length,
            factor: self.factor,
        }
    }
}

impl AngleUnit {
    /// This unit as a table row.
    pub const fn def(self) -> UnitDef {
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
pub const UNITS: [UnitDef; 6] = [
    MM.def(),
    CM.def(),
    M.def(),
    IN.def(),
    DEG.def(),
    RAD.def(),
];

/// The table row for a surface symbol, or `None` when the symbol is
/// not one of the six.
pub fn unit_by_symbol(symbol: &str) -> Option<UnitDef> {
    UNITS.iter().find(|u| u.symbol == symbol).copied()
}
