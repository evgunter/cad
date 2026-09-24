//! An `f64` as a refusal sentence can carry it.
//!
//! `f64`'s `Display` is positional at every magnitude: `1e308` renders
//! as a 309-digit integer and `1e-300` as three hundred zeros after the
//! point. A refusal that names a knot, a weight or a parameter near
//! either end of the range would print a number no reader can read, and
//! the domains some refusals exist to report (an overflowing reflection,
//! a width past `f64::MAX`) live exactly there.
//!
//! [`Readable`] is the one rendering: positional (`Display`) on the
//! decades where that is short, scientific (`LowerExp`) outside them.
//! The boundary is std's own for `f64`'s `Debug` — positional for
//! `1e-4 ≤ |x| < 1e16` and for zero — so a knot in `[0, 1]`, a count
//! stored as a float, or a length in metres at modelling scale reads
//! exactly as the bare `{}` it replaces, and `1e308` reads `1e308`.
//! Both arms are shortest round-trip, so no digit the value carries is
//! lost either way. Non-finite values render as `Display` spells them
//! (`inf`, `-inf`, `NaN`).

use core::fmt;

/// The smallest magnitude rendered positionally (std's `Debug`
/// boundary for `f64`).
const POSITIONAL_FLOOR: f64 = 1e-4;
/// The first magnitude rendered scientifically (std's `Debug` boundary
/// for `f64`).
const POSITIONAL_CEILING: f64 = 1e16;

/// An `f64` rendered for a refusal message: positional where that is
/// short, scientific where it is not (the module docs give the
/// boundary). Width and precision flags on the placeholder pass through
/// to whichever of `Display` / `LowerExp` renders the value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Readable(pub f64);

impl fmt::Display for Readable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.0;
        let magnitude = x.abs();
        let positional = !x.is_finite()
            || magnitude == 0.0
            || (POSITIONAL_FLOOR..POSITIONAL_CEILING).contains(&magnitude);
        if positional {
            fmt::Display::fmt(&x, f)
        } else {
            fmt::LowerExp::fmt(&x, f)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Readable;

    fn show(x: f64) -> String {
        Readable(x).to_string()
    }

    /// The decades a knot, a weight or a modelling-scale length lives
    /// in render exactly as the bare `Display` would.
    #[test]
    fn ordinary_values_read_as_display_reads_them() {
        for x in [
            0.0,
            -0.0,
            1.0,
            0.25,
            1.0 / 3.0,
            -2.5,
            0.0001,
            1234.5,
            9_999_999_999_999_998.0,
        ] {
            assert_eq!(show(x), x.to_string(), "{x:e}");
        }
    }

    /// Past either boundary the rendering is scientific, and the
    /// boundary sits where the module docs put it.
    #[test]
    fn values_past_the_boundary_read_scientifically() {
        assert_eq!(show(1e308), "1e308");
        assert_eq!(show(1.5e308), "1.5e308");
        assert_eq!(show(-f64::MAX), "-1.7976931348623157e308");
        assert_eq!(show(1e16), "1e16");
        assert_eq!(show(9.9e-5), "9.9e-5");
        assert_eq!(show(1e-300), "1e-300");
        assert_eq!(show(f64::MIN_POSITIVE / 4.0), "5.562684646268003e-309");
    }

    #[test]
    fn non_finite_values_read_as_display_spells_them() {
        assert_eq!(show(f64::INFINITY), "inf");
        assert_eq!(show(f64::NEG_INFINITY), "-inf");
        assert_eq!(show(f64::NAN), "NaN");
    }

    /// Every finite magnitude renders short: one value per binade,
    /// denormals to `f64::MAX`, at most a sign, seventeen significant
    /// digits, a point and a positional tail or an exponent. The bare
    /// `Display` breaks this at both ends by three hundred characters.
    #[test]
    fn every_binade_renders_short() {
        let mut x = f64::from_bits(1);
        while x.is_finite() {
            for v in [x, -x, x * 1.5] {
                let s = show(v);
                assert!(s.len() <= 24, "{v:e} rendered as {} chars: {s}", s.len());
                assert_eq!(s.parse::<f64>(), Ok(v), "the rendering round-trips");
            }
            x *= 2.0;
        }
    }

    #[test]
    fn precision_flags_reach_the_rendering() {
        assert_eq!(format!("{:.2}", Readable(0.126)), "0.13");
        assert_eq!(format!("{:.1}", Readable(1.26e300)), "1.3e300");
    }
}
