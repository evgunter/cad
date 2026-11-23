//! SI unit resolution (M7-2 Leg A): the prefix table as **data**, and
//! the one scale factor every length in a file passes through on its
//! way to kernel meters.
//!
//! The kernel's own writer emits `SI_UNIT($, .METRE.)` — no prefix, so
//! the factor is 1 and nothing moves. A foreign file need not: FreeCAD
//! 1.1.2 writes `SI_UNIT(.MILLI., .METRE.)` on every file it emits, and
//! its declared uncertainty (`1.E-07`) is in those millimeters too.
//! Accepting that is not a special case for millimeters — the ISO
//! 10303-41 `si_prefix` enumeration is a closed sixteen-entry table, so
//! it is looked up as data ([`PREFIX_FACTORS`]) and any member resolves.
//!
//! **What scales**: every length the file states — `CARTESIAN_POINT`
//! coordinates, every radius, and the declared uncertainty that becomes
//! ε_in. **What does not**: `DIRECTION` ratios and the cone's
//! `semi_angle` (dimensionless), a `VECTOR`'s magnitude (checked
//! against 1 as the subset's arc-length parameterization convention,
//! which is a statement about the parameterization, not a length), and
//! B-spline knots (parameter values — scaling a curve's control points
//! scales its locus with its domain left alone).
//!
//! SI angle units are accepted **unprefixed only**. A `.MILLI.
//! .RADIAN.` context is representable in the schema and absent from
//! every file measured; rather than fold a second SI scale into the
//! angle path on speculation, it refuses typed and names the unit.
//!
//! Both corpus claims above — the millimetre-and-`1.E-07` dialect and
//! the absence of a prefixed SI angle — are re-read from the committed
//! files by `tests/freecad.rs`'s
//! `the_committed_freecad_corpus_still_says_what_chart_and_units_quote`
//! (issue #667's Q6), so neither is a sentence that can quietly stop
//! being true. What that row does NOT claim is anything about FreeCAD
//! releases nobody committed a file from.
//!
//! **Conversion-based units (M7-4 Leg B).** The wild's dominant
//! convention is not SI at all: `CONVERSION_BASED_UNIT('INCH', …)` and
//! `('DEGREE', …)`, each naming a `*_MEASURE_WITH_UNIT` that states
//! the conversion *in the file*. Those resolve through
//! [`conversion_kind`] — multiplying the file's own stated factor onto
//! whatever base unit it names — and never through a table of what an
//! inch "is". The consequence for the list above: a `semi_angle` is
//! dimensionless in the sense that no LENGTH scale touches it, but it
//! is an angle, so the file's ANGLE scale does; a degree-context cone
//! whose half-angle were read as radians would be a different cone.

use quantity::Decimal;

use crate::error::StepImportError;

/// The ISO 10303-41 `si_prefix` enumeration and the power of ten each
/// names — the whole closed table, so a prefix is data, never a special
/// case.
pub(crate) const PREFIX_FACTORS: [(&str, i64); 16] = [
    ("EXA", 18),
    ("PETA", 15),
    ("TERA", 12),
    ("GIGA", 9),
    ("MEGA", 6),
    ("KILO", 3),
    ("HECTO", 2),
    ("DECA", 1),
    ("DECI", -1),
    ("CENTI", -2),
    ("MILLI", -3),
    ("MICRO", -6),
    ("NANO", -9),
    ("PICO", -12),
    ("FEMTO", -15),
    ("ATTO", -18),
];

/// The factor for an `si_prefix` enumeration name, or `None` when the
/// name is not one of the sixteen.
pub(crate) fn prefix_factor(name: &str) -> Option<Scale> {
    PREFIX_FACTORS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, k)| Scale::power_of_ten(k))
}

/// **A factor into kernel units, held exactly**: the decimal the
/// file's own numbers and the prefix table's powers of ten make, so a
/// number the file states in its unit is read through the unit door's
/// arithmetic, its decimal times the exact factor rounded once
/// ([`Scale::apply`]), not a float multiply that rounds twice.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Scale(Decimal);

impl Scale {
    /// The identity: the kernel's own units.
    pub(crate) fn one() -> Self {
        Self::power_of_ten(0)
    }

    /// `10^k`.
    pub(crate) fn power_of_ten(k: i64) -> Self {
        let Some(power) = Decimal::parse(&format!("1e{k}")) else {
            unreachable!("1e{k} is a decimal")
        };
        Self(power)
    }

    /// The number a file states, read as its shortest decimal (the
    /// file's own digits, for any it wrote of at most fifteen
    /// figures); `None` where it is not finite.
    pub(crate) fn of_file(value: f64) -> Option<Self> {
        Decimal::of_float(value).map(Self)
    }

    /// This factor times `other`, exactly.
    pub(crate) fn times(&self, other: &Self) -> Self {
        Self(self.0.times(&other.0))
    }

    /// `raw`, a number the file states in this unit, in kernel units:
    /// its decimal times this factor, rounded once. The identity moves
    /// no bits, and a value that is not finite is itself.
    pub(crate) fn apply(&self, raw: f64) -> f64 {
        if *self == Self::one() {
            return raw;
        }
        Decimal::of_float(raw).map_or(raw, |number| number.times_to_f64(&self.0))
    }
}

/// What one resolved unit record measures, with the factor that
/// carries it into the kernel's own (meters, radians, steradians).
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum UnitKind {
    /// A length, carrying the factor into meters (1.0 unprefixed,
    /// 1e-3 for `.MILLI.`, the file's own 0.0254 for a declared inch).
    Length(Scale),
    /// A plane angle, carrying the factor into radians (1.0 for an
    /// unprefixed `.RADIAN.`; for a `CONVERSION_BASED_UNIT('DEGREE',…)`
    /// the file's OWN declared π/180, read off its conversion
    /// expression — M7-4 Leg B).
    Angle(Scale),
    /// A solid angle in steradians (unprefixed).
    SolidAngle,
}

impl UnitKind {
    /// The name of the quantity this unit measures — for refusal text
    /// that says which slot a mismatched conversion factor landed in.
    pub(crate) fn quantity(&self) -> &'static str {
        match self {
            Self::Length(_) => "a length",
            Self::Angle(_) => "a plane angle",
            Self::SolidAngle => "a solid angle",
        }
    }
}

/// Scales a `CONVERSION_BASED_UNIT`'s declared factor onto the base
/// unit it converts from: `INCH = 25.4 MILLIMETRE` resolves to
/// `Length(25.4 * 1e-3)`, `DEGREE = 0.017453… RADIAN` to
/// `Angle(0.017453…)`.
///
/// **The factor comes from the file, never from a table.** A reader
/// that "knows" an inch is 25.4 mm silently accepts a file that says
/// otherwise and imports it at the wrong size; this one multiplies
/// through whatever the conversion expression states, so a file's own
/// arithmetic is the only authority. `declared` is the quantity the
/// complex claims to be (its `LENGTH_UNIT()` / `PLANE_ANGLE_UNIT()`
/// component) and `base` is what the factor's measure resolved to:
/// they must agree, or the record is malformed rather than merely
/// unsupported, and it refuses naming both.
pub(crate) fn conversion_kind(
    id: u64,
    declared: &str,
    value: f64,
    base: UnitKind,
    found: impl Fn() -> String,
) -> Result<UnitKind, StepImportError> {
    if !(value.is_finite() && value > 0.0) {
        return Err(StepImportError::UnsupportedUnit {
            id,
            found: format!(
                "{} with conversion factor {value}, which is not finite and positive",
                found()
            ),
        });
    }
    let mismatch = |base: UnitKind| StepImportError::UnsupportedUnit {
        id,
        found: format!(
            "{} declared as {declared} but converting from {}",
            found(),
            base.quantity()
        ),
    };
    let Some(stated) = Scale::of_file(value) else {
        unreachable!("a finite conversion factor is a decimal")
    };
    match (declared, base) {
        ("LENGTH_UNIT", UnitKind::Length(f)) => Ok(UnitKind::Length(stated.times(&f))),
        ("PLANE_ANGLE_UNIT", UnitKind::Angle(f)) => Ok(UnitKind::Angle(stated.times(&f))),
        // A solid-angle conversion carries no factor anywhere in this
        // importer (nothing reads a solid angle), so only the identity
        // resolves; a scaled steradian refuses rather than resolving
        // to a factor no consumer would apply.
        ("SOLID_ANGLE_UNIT", UnitKind::SolidAngle) if value == 1.0 => Ok(UnitKind::SolidAngle),
        (_, base) => Err(mismatch(base)),
    }
}

/// Reads one `SI_UNIT(prefix, name)` pair into its [`UnitKind`], or
/// refuses typed naming `id` (module docs: prefixed lengths resolve,
/// prefixed angles refuse).
pub(crate) fn si_unit_kind(
    id: u64,
    prefix: Option<&str>,
    name: &str,
    found: impl FnOnce() -> String,
) -> Result<UnitKind, StepImportError> {
    match (name, prefix) {
        ("METRE", None) => Ok(UnitKind::Length(Scale::one())),
        ("METRE", Some(p)) => prefix_factor(p)
            .map(UnitKind::Length)
            .ok_or(StepImportError::UnsupportedUnit { id, found: found() }),
        ("RADIAN", None) => Ok(UnitKind::Angle(Scale::one())),
        ("STERADIAN", None) => Ok(UnitKind::SolidAngle),
        _ => Err(StepImportError::UnsupportedUnit { id, found: found() }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{PREFIX_FACTORS, Scale, UnitKind, conversion_kind, prefix_factor, si_unit_kind};

    fn scale(text: &str) -> Scale {
        Scale(quantity::Decimal::parse(text).unwrap())
    }

    #[test]
    fn the_prefix_table_is_the_whole_enumeration_and_reads_as_data() {
        assert_eq!(PREFIX_FACTORS.len(), 16);
        assert_eq!(prefix_factor("MILLI"), Some(scale("0.001")));
        assert_eq!(prefix_factor("KILO"), Some(scale("1000")));
        assert_eq!(prefix_factor("ATTO"), Some(scale("1e-18")));
        assert_eq!(prefix_factor("MYRIA"), None, "not an si_prefix member");
    }

    #[test]
    fn lengths_take_prefixes_and_angles_do_not() {
        let found = || "probe".to_owned();
        assert_eq!(
            si_unit_kind(1, Some("MILLI"), "METRE", found).unwrap(),
            UnitKind::Length(scale("1e-3"))
        );
        assert_eq!(
            si_unit_kind(1, None, "METRE", found).unwrap(),
            UnitKind::Length(Scale::one())
        );
        assert_eq!(
            si_unit_kind(1, None, "RADIAN", found).unwrap(),
            UnitKind::Angle(Scale::one())
        );
        assert!(si_unit_kind(1, Some("MILLI"), "RADIAN", found).is_err());
        assert!(si_unit_kind(1, None, "INCH", found).is_err());
    }

    /// Leg B: the conversion factor is the file's arithmetic, carried
    /// through whatever base unit it names — an inch declared over
    /// millimetres lands at 0.0254 m without a table lookup, and a
    /// file that declared its inch differently would import at ITS
    /// number, not ours.
    #[test]
    fn a_conversion_factor_multiplies_through_the_base_it_names() {
        let found = || "complex(CONVERSION_BASED_UNIT LENGTH_UNIT NAMED_UNIT)".to_owned();
        assert_eq!(
            conversion_kind(
                1,
                "LENGTH_UNIT",
                25.4,
                UnitKind::Length(scale("1e-3")),
                found
            )
            .unwrap(),
            UnitKind::Length(scale("0.0254"))
        );
        assert_eq!(
            conversion_kind(
                1,
                "PLANE_ANGLE_UNIT",
                0.0174532925,
                UnitKind::Angle(Scale::one()),
                found
            )
            .unwrap(),
            UnitKind::Angle(scale("0.0174532925"))
        );
        // The identity conversion an Onshape-lineage writer emits for
        // plain metres resolves to plain metres.
        assert_eq!(
            conversion_kind(1, "LENGTH_UNIT", 1.0, UnitKind::Length(Scale::one()), found).unwrap(),
            UnitKind::Length(Scale::one())
        );
    }

    /// A conversion whose declared quantity and base disagree, or
    /// whose factor is not finite and positive, refuses typed and
    /// names both sides.
    #[test]
    fn a_mismatched_or_unusable_conversion_refuses_typed() {
        let found = || "complex(CONVERSION_BASED_UNIT LENGTH_UNIT NAMED_UNIT)".to_owned();
        let mismatch = conversion_kind(7, "LENGTH_UNIT", 1.0, UnitKind::Angle(Scale::one()), found)
            .unwrap_err()
            .render();
        assert!(mismatch.contains("declared as LENGTH_UNIT"), "{mismatch}");
        assert!(mismatch.contains("a plane angle"), "{mismatch}");
        for bad in [0.0, -25.4, f64::NAN, f64::INFINITY] {
            assert!(
                conversion_kind(
                    7,
                    "LENGTH_UNIT",
                    bad,
                    UnitKind::Length(scale("1e-3")),
                    found
                )
                .is_err(),
                "factor {bad} is not a usable scale"
            );
        }
        // Nothing reads a solid angle, so a scaled steradian has no
        // consumer to apply its factor: refused, not silently dropped.
        assert!(conversion_kind(7, "SOLID_ANGLE_UNIT", 2.0, UnitKind::SolidAngle, found).is_err());
        assert!(
            conversion_kind(7, "MASS_UNIT", 1.0, UnitKind::Length(Scale::one()), found).is_err(),
            "a mass conversion is not a quantity this importer measures"
        );
    }
}
