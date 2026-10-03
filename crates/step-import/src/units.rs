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

use crate::error::StepImportError;

/// The ISO 10303-41 `si_prefix` enumeration and its decimal factor —
/// the whole closed table, so a prefix is data, never a special case.
/// Factors are written as exact decimal literals (each is the
/// correctly-rounded f64 of its power of ten).
pub(crate) const PREFIX_FACTORS: [(&str, f64); 16] = [
    ("EXA", 1e18),
    ("PETA", 1e15),
    ("TERA", 1e12),
    ("GIGA", 1e9),
    ("MEGA", 1e6),
    ("KILO", 1e3),
    ("HECTO", 1e2),
    ("DECA", 1e1),
    ("DECI", 1e-1),
    ("CENTI", 1e-2),
    ("MILLI", 1e-3),
    ("MICRO", 1e-6),
    ("NANO", 1e-9),
    ("PICO", 1e-12),
    ("FEMTO", 1e-15),
    ("ATTO", 1e-18),
];

/// The factor for an `si_prefix` enumeration name, or `None` when the
/// name is not one of the sixteen.
pub(crate) fn prefix_factor(name: &str) -> Option<f64> {
    PREFIX_FACTORS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, f)| *f)
}

/// What one resolved unit record measures, with the factor that
/// carries it into the kernel's own (meters, radians, steradians).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum UnitKind {
    /// A length, carrying the factor into meters (1.0 unprefixed,
    /// 1e-3 for `.MILLI.`, the file's own 0.0254 for a declared inch).
    Length(f64),
    /// A plane angle, carrying the factor into radians (1.0 for an
    /// unprefixed `.RADIAN.`; for a `CONVERSION_BASED_UNIT('DEGREE',…)`
    /// the file's OWN declared π/180, read off its conversion
    /// expression — M7-4 Leg B).
    Angle(f64),
    /// A solid angle in steradians (unprefixed).
    SolidAngle,
}

impl UnitKind {
    /// The name of the quantity this unit measures — for refusal text
    /// that says which slot a mismatched conversion factor landed in.
    pub(crate) fn quantity(self) -> &'static str {
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
    match (declared, base) {
        ("LENGTH_UNIT", UnitKind::Length(f)) => Ok(UnitKind::Length(value * f)),
        ("PLANE_ANGLE_UNIT", UnitKind::Angle(f)) => Ok(UnitKind::Angle(value * f)),
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
        ("METRE", None) => Ok(UnitKind::Length(1.0)),
        ("METRE", Some(p)) => prefix_factor(p)
            .map(UnitKind::Length)
            .ok_or(StepImportError::UnsupportedUnit { id, found: found() }),
        ("RADIAN", None) => Ok(UnitKind::Angle(1.0)),
        ("STERADIAN", None) => Ok(UnitKind::SolidAngle),
        _ => Err(StepImportError::UnsupportedUnit { id, found: found() }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{PREFIX_FACTORS, UnitKind, conversion_kind, prefix_factor, si_unit_kind};

    #[test]
    fn the_prefix_table_is_the_whole_enumeration_and_reads_as_data() {
        assert_eq!(PREFIX_FACTORS.len(), 16);
        assert_eq!(prefix_factor("MILLI"), Some(1e-3));
        assert_eq!(prefix_factor("KILO"), Some(1e3));
        assert_eq!(prefix_factor("ATTO"), Some(1e-18));
        assert_eq!(prefix_factor("MYRIA"), None, "not an si_prefix member");
    }

    #[test]
    fn lengths_take_prefixes_and_angles_do_not() {
        let found = || "probe".to_owned();
        assert_eq!(
            si_unit_kind(1, Some("MILLI"), "METRE", found).unwrap(),
            UnitKind::Length(1e-3)
        );
        assert_eq!(
            si_unit_kind(1, None, "METRE", found).unwrap(),
            UnitKind::Length(1.0)
        );
        assert_eq!(
            si_unit_kind(1, None, "RADIAN", found).unwrap(),
            UnitKind::Angle(1.0)
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
            conversion_kind(1, "LENGTH_UNIT", 25.4, UnitKind::Length(1e-3), found).unwrap(),
            UnitKind::Length(25.4 * 1e-3)
        );
        assert_eq!(
            conversion_kind(
                1,
                "PLANE_ANGLE_UNIT",
                0.0174532925,
                UnitKind::Angle(1.0),
                found
            )
            .unwrap(),
            UnitKind::Angle(0.0174532925)
        );
        // The identity conversion an Onshape-lineage writer emits for
        // plain metres resolves to plain metres.
        assert_eq!(
            conversion_kind(1, "LENGTH_UNIT", 1.0, UnitKind::Length(1.0), found).unwrap(),
            UnitKind::Length(1.0)
        );
    }

    /// A conversion whose declared quantity and base disagree, or
    /// whose factor is not finite and positive, refuses typed and
    /// names both sides.
    #[test]
    fn a_mismatched_or_unusable_conversion_refuses_typed() {
        let found = || "complex(CONVERSION_BASED_UNIT LENGTH_UNIT NAMED_UNIT)".to_owned();
        let mismatch = conversion_kind(7, "LENGTH_UNIT", 1.0, UnitKind::Angle(1.0), found)
            .unwrap_err()
            .to_string();
        assert!(mismatch.contains("declared as LENGTH_UNIT"), "{mismatch}");
        assert!(mismatch.contains("a plane angle"), "{mismatch}");
        for bad in [0.0, -25.4, f64::NAN, f64::INFINITY] {
            assert!(
                conversion_kind(7, "LENGTH_UNIT", bad, UnitKind::Length(1e-3), found).is_err(),
                "factor {bad} is not a usable scale"
            );
        }
        // Nothing reads a solid angle, so a scaled steradian has no
        // consumer to apply its factor: refused, not silently dropped.
        assert!(conversion_kind(7, "SOLID_ANGLE_UNIT", 2.0, UnitKind::SolidAngle, found).is_err());
        assert!(
            conversion_kind(7, "MASS_UNIT", 1.0, UnitKind::Length(1.0), found).is_err(),
            "a mass conversion is not a quantity this importer measures"
        );
    }
}
