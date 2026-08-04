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
//! Angle units are accepted **unprefixed only**. A `.MILLI. .RADIAN.`
//! context is representable in the schema and absent from every file
//! measured; rather than fold a second scale into the angle path on
//! speculation, it refuses typed and names the unit.

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

/// What one resolved `SI_UNIT` record measures.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum UnitKind {
    /// A length, carrying the factor into meters (1.0 unprefixed,
    /// 1e-3 for `.MILLI.`).
    Length(f64),
    /// A plane angle in radians (unprefixed).
    Angle,
    /// A solid angle in steradians (unprefixed).
    SolidAngle,
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
        ("RADIAN", None) => Ok(UnitKind::Angle),
        ("STERADIAN", None) => Ok(UnitKind::SolidAngle),
        _ => Err(StepImportError::UnsupportedUnit { id, found: found() }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{PREFIX_FACTORS, UnitKind, prefix_factor, si_unit_kind};

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
            UnitKind::Angle
        );
        assert!(si_unit_kind(1, Some("MILLI"), "RADIAN", found).is_err());
        assert!(si_unit_kind(1, None, "INCH", found).is_err());
    }
}
