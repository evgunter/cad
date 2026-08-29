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
//! * `PI` is the half-turn, factor `f64::consts::PI` — the unit that
//!   is not a unit. It measures an angle exactly as `DEG` does and is
//!   carried here for the same reason every other row is: a literal
//!   REMEMBERS the unit it was authored in, so `0.5 pi` is how a user
//!   says "write this angle as a multiple of π" and get it back that
//!   way. Nothing downstream distinguishes it from `DEG` — an angle is
//!   canonical radians whichever row produced it.
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
/// The two typed views [`LengthUnit`] and [`AngleUnit`] are sealed by a
/// stronger mechanism (issue #669): each is a private INDEX into
/// [`UNITS`], so "is a row of the table" is what the value is, not a
/// convention about how it is built. [`UnitDef::as_length`] and
/// [`UnitDef::as_angle`] are the public route from a row to its view;
/// [`LengthUnit::def`] / [`AngleUnit::def`] are the inverse, and are
/// public because the seal never depended on their visibility — a view
/// IS an index into this table, so handing back the row it indexes
/// cannot produce a pairing the table does not have.
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
/// Both are refused on PRIVACY (`E0451`), which is the property that
/// has to hold: they redden the moment a field is made public, and a
/// row that merely named a field that does not exist would not.
///
/// A `compile_fail` row proves only that the snippet does not build,
/// not that it fails for the intended reason — a typo would pass it
/// just as well. Each block above therefore has its legal twin here,
/// differing from it in exactly one respect: the twin never names a
/// field.
///
/// ```
/// // The path `quantity::UnitDef` names this exported type, the row
/// // and the enum variant exist, and every field is READABLE through
/// // its accessor. Naming the type binds the twin to it, so a rename
/// // or an un-export reddens here rather than quietly satisfying the
/// // `compile_fail` blocks for the wrong reason.
/// let mm: quantity::UnitDef = quantity::unit_by_symbol("mm").expect("mm is a table row");
/// assert_eq!(mm.symbol(), "mm");
/// assert_eq!(mm.quantity(), quantity::UnitQuantity::Length);
/// assert_eq!(mm.factor(), quantity::MILLI);
/// assert_eq!(quantity::UnitQuantity::Angle, quantity::UnitQuantity::Angle);
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
    /// view a row has, and it is data. The refusal can only answer
    /// `None` for a row whose own `quantity` says so — never for a
    /// caller-assembled pair, which no longer exists.
    pub const fn as_length(self) -> Option<LengthUnit> {
        match self.quantity {
            UnitQuantity::Length => Some(LengthUnit::of_row(self.symbol)),
            UnitQuantity::Angle => None,
        }
    }

    /// This row as an [`AngleUnit`], or `None` when the row measures a
    /// length — the mirror of [`UnitDef::as_length`].
    pub const fn as_angle(self) -> Option<AngleUnit> {
        match self.quantity {
            UnitQuantity::Angle => Some(AngleUnit::of_row(self.symbol)),
            UnitQuantity::Length => None,
        }
    }
}

/// The whole closed unit table, as data — the expression text parser's
/// suffix vocabulary and the formatter's display vocabulary are both
/// exactly this list, and a typed view is an index into it.
pub const UNITS: [UnitDef; 7] = [
    // Millimeter: a `MILLI`-prefixed metre.
    UnitDef {
        symbol: "mm",
        quantity: UnitQuantity::Length,
        factor: MILLI,
    },
    // Centimeter: a `CENTI`-prefixed metre.
    UnitDef {
        symbol: "cm",
        quantity: UnitQuantity::Length,
        factor: CENTI,
    },
    // Meter — the canonical length unit, factor exactly 1.0.
    UnitDef {
        symbol: "m",
        quantity: UnitQuantity::Length,
        factor: 1.0,
    },
    // International inch: EXACTLY 25.4 mm (module docs) — the factor
    // is the correctly-rounded f64 of the exact decimal 0.0254.
    UnitDef {
        symbol: "in",
        quantity: UnitQuantity::Length,
        factor: 0.0254,
    },
    // Degree: π/180 radians — inexact by nature (module docs).
    UnitDef {
        symbol: "deg",
        quantity: UnitQuantity::Angle,
        factor: core::f64::consts::PI / 180.0,
    },
    // Radian — the canonical angle unit, factor exactly 1.0.
    UnitDef {
        symbol: "rad",
        quantity: UnitQuantity::Angle,
        factor: 1.0,
    },
    // Half-turn: π radians — the "unit" that is a notation (module
    // docs). Inexact for the same reason `deg` is, and by the same
    // constant.
    UnitDef {
        symbol: "pi",
        quantity: UnitQuantity::Angle,
        factor: core::f64::consts::PI,
    },
];

// A typed view is one byte, so the table it indexes must fit in one —
// the same bound `editor_core::expr::UnitSym` asserts over the same
// table. A table grown past 255 rows fails the BUILD here rather than
// truncating an index.
const _: () = assert!(
    UNITS.len() <= u8::MAX as usize,
    "a typed unit view is one byte: this table has outgrown its index space"
);

/// Byte-wise `str` equality, because `==` is not available in a const
/// fn on this toolchain.
const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// The position of `symbol` in [`UNITS`] — the ONE mint for a typed
/// view, and the reason `LengthUnit`/`AngleUnit` cannot name a unit the
/// table does not have.
///
/// Two callers, both of which make the miss unreachable rather than
/// merely unlikely: the unit constants, where a miss is a
/// compile-time const-eval failure, and [`UnitDef::as_length`] /
/// [`UnitDef::as_angle`], where the argument is a sealed row and so is
/// a copy of an entry of this very array. D2 addendum row 4 — a kernel
/// bug observable in a branch, announced, never discarded. Message-less
/// because a const fn cannot format one, so the invariant is stated
/// here instead of at the macro.
const fn row_index(symbol: &str) -> u8 {
    let mut i = 0;
    while i < UNITS.len() {
        if str_eq(UNITS[i].symbol, symbol) {
            return i as u8;
        }
        i += 1;
    }
    unreachable!()
}

/// A length unit — a typed view of a [`UnitDef`] row, so `25.0 * MM`
/// can only build a [`crate::Length`] (never an angle), and the row it
/// views is a row of [`UNITS`] **by construction**: the value is that
/// row's index and nothing else.
///
/// **SEALED (issue #669) — the narrative lives on [`UnitDef`].** Both
/// halves of the pair are typed: the DIMENSION by which view this is,
/// the symbol/factor pairing by the index. Obtain one from a unit
/// constant ([`MM`], [`CM`], [`M`], [`IN`]) or from
/// [`UnitDef::as_length`].
///
/// The seal has two halves with different mechanisms, and it is worth
/// saying which pins which. **Outside the crate** the field is private,
/// so no value can be built at all — pinned by the `compile_fail` rows
/// below. **Inside the crate** the type lives behind a private module,
/// so the only mints are `of_row` and the two `UnitDef::as_*` doors,
/// and `of_row`'s two refusals are const-evaluated: a symbol [`UNITS`]
/// does not have, or a row of the wrong quantity, fails the BUILD at
/// the constant that names it. Nothing here rests on a convention that
/// this file happens to keep.
///
/// **Eight public functions apply a unit's factor and print its symbol
/// without checking either** — which is why the pairing has to be typed
/// rather than validated. Four take a `LengthUnit`: [`crate::fmt_length`],
/// [`crate::Length::in_unit`], and the two `Mul` impls
/// (`f64 * LengthUnit` and `LengthUnit * f64`, the two spellings of one
/// multiply); [`AngleUnit`] mirrors all four. The `Mul` impls ARE the
/// D6 typed-units boundary, and `fmt.rs`'s `parse(fmt(x, unit))` pin
/// makes the formatter's suffix parser input, so a symbol paired with a
/// foreign factor would be a wrong VALUE reaching an `Expr` and a
/// document — not a display string. Issue #669 has the executed
/// reproductions.
///
/// The seal is a claim about what COMPILES, so it is pinned by rows
/// that must fail to compile — and each pins a PRIVACY refusal, so that
/// opening the field reddens it. A row that named a field this type
/// does not have would fail forever, however open the seal became, and
/// is worth nothing.
///
/// The index cannot be supplied:
///
/// ```compile_fail
/// let bogus = quantity::LengthUnit(0);
/// ```
///
/// Nor read off a real constant and re-used:
///
/// ```compile_fail
/// let stolen = quantity::MM.0;
/// ```
///
/// Nor written through:
///
/// ```compile_fail
/// let mut mm = quantity::MM;
/// mm.0 = 4;
/// ```
///
/// The legal twin of all three: the type is exported, both readers
/// exist, and the twin differs from each block above in exactly one
/// respect — it never names the field.
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
pub use view::LengthUnit;

/// An angle unit — the typed view for [`crate::Angle`] construction, an
/// index into [`UNITS`].
///
/// **SEALED for the reason and by the mechanism on [`LengthUnit`]**;
/// obtain one from [`DEG`], [`RAD`], or [`UnitDef::as_angle`]. The same
/// three privacy refusals pin it, so the two types stay symmetric.
///
/// ```compile_fail
/// let bogus = quantity::AngleUnit(4);
/// ```
///
/// ```compile_fail
/// let stolen = quantity::DEG.0;
/// ```
///
/// ```compile_fail
/// let mut deg = quantity::DEG;
/// deg.0 = 5;
/// ```
///
/// Their legal twin:
///
/// ```
/// let deg: quantity::AngleUnit = quantity::DEG;
/// assert_eq!(deg.symbol(), "deg");
/// assert!(((90.0 * deg).radians() - core::f64::consts::FRAC_PI_2).abs() < 1e-15);
/// let row = quantity::unit_by_symbol("deg").expect("deg is a table row");
/// assert_eq!(row.as_angle(), Some(quantity::DEG));
/// assert_eq!(row.as_length(), None);
/// ```
pub use view::AngleUnit;

/// The typed views live behind a private module boundary, so the mints
/// below are the only code ANYWHERE — inside this file included — that
/// can build one. That is what makes "a `LengthUnit` is a `Length` row
/// of [`UNITS`]" a compiler-enforced invariant rather than a convention
/// this file keeps: a hand-written `LengthUnit(4)` does not compile
/// here either.
mod view {
    use super::{UNITS, UnitDef, UnitQuantity, row_index};

    /// See [`super::LengthUnit`].
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct LengthUnit(u8);

    /// See [`super::AngleUnit`].
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct AngleUnit(u8);

    impl LengthUnit {
        /// The length view of the [`UNITS`] row named `symbol` — the
        /// ONE mint. Both refusals are const-evaluated at every
        /// caller: a symbol the table does not have and a row of the
        /// wrong quantity are compile errors, so neither a seventh
        /// unit nor a mislabelled one can be written.
        pub(super) const fn of_row(symbol: &str) -> Self {
            let i = row_index(symbol);
            assert!(
                matches!(UNITS[i as usize].quantity(), UnitQuantity::Length),
                "a LengthUnit can only view a Length row of UNITS"
            );
            Self(i)
        }

        /// The surface symbol (`"mm"`), as displayed and as parsed back.
        pub const fn symbol(self) -> &'static str {
            self.def().symbol()
        }

        /// Meters per one of this unit.
        pub const fn factor(self) -> f64 {
            self.def().factor()
        }

        /// The table row this view indexes — the inverse of
        /// [`UnitDef::as_length`], and total, because the view IS an
        /// index. Public: it can only ever answer a row of [`UNITS`],
        /// which is what the seal is about.
        pub const fn def(self) -> UnitDef {
            UNITS[self.0 as usize]
        }
    }

    impl AngleUnit {
        /// The angle mirror of [`LengthUnit::of_row`].
        pub(super) const fn of_row(symbol: &str) -> Self {
            let i = row_index(symbol);
            assert!(
                matches!(UNITS[i as usize].quantity(), UnitQuantity::Angle),
                "an AngleUnit can only view an Angle row of UNITS"
            );
            Self(i)
        }

        /// The surface symbol (`"deg"`), as displayed and as parsed back.
        pub const fn symbol(self) -> &'static str {
            self.def().symbol()
        }

        /// Radians per one of this unit.
        pub const fn factor(self) -> f64 {
            self.def().factor()
        }

        /// The table row this view indexes — see [`LengthUnit::def`].
        pub const fn def(self) -> UnitDef {
            UNITS[self.0 as usize]
        }
    }
}

/// Millimeter: a `MILLI`-prefixed metre.
pub const MM: LengthUnit = LengthUnit::of_row("mm");

/// Centimeter: a `CENTI`-prefixed metre.
pub const CM: LengthUnit = LengthUnit::of_row("cm");

/// Meter — the canonical length unit, factor exactly 1.0.
pub const M: LengthUnit = LengthUnit::of_row("m");

/// International inch: EXACTLY 25.4 mm (module docs).
pub const IN: LengthUnit = LengthUnit::of_row("in");

/// Degree: π/180 radians — inexact by nature (module docs).
pub const DEG: AngleUnit = AngleUnit::of_row("deg");

/// Radian — the canonical angle unit, factor exactly 1.0.
pub const RAD: AngleUnit = AngleUnit::of_row("rad");

/// Half-turn: π radians, so `0.5 * PI` is a right angle and `2.0 * PI`
/// a full turn. The notation-as-a-unit row (module docs).
pub const PI: AngleUnit = AngleUnit::of_row("pi");

/// The table row for a surface symbol, or `None` when the symbol is
/// not one the table carries.
pub fn unit_by_symbol(symbol: &str) -> Option<UnitDef> {
    UNITS.iter().find(|u| u.symbol == symbol).copied()
}
