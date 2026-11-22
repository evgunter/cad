//! A number as text a person reads.
//!
//! Module kind: **vocabulary** (`crates/viewer/README.md`, Module
//! boundaries): the chrome's doors onto `geom_core::Told`, which is the
//! one rule for a number told to a person
//! (`crates/geom-core/README.md`, "A number told to a person"), and
//! the width of the widest text they return ([`widest_render`]) for a
//! box that has to hold one.
//!
//! # The rule, and which door says what
//!
//! A told quantity is held in metres or radians with its role and its
//! dimension, and is rendered in a notation. So a door here takes the
//! canonical value and the unit it is read in, never a number already
//! divided by a factor: the grid (four figures, and for a length a
//! tenth of ε, which is a length whatever the unit) and the unit word
//! are the quantity's.
//!
//! - [`computed`] and [`computed_number`]: a value nobody wrote (a
//!   driven slot, a measure, a display tolerance), on the grid.
//! - [`bare`]: the same in the canonical unit, for a sentence that
//!   words the unit itself.
//! - [`field_number`] and [`names`]: the fields' door, which has no
//!   unit in hand (see [`field_number`]).
//!
//! No door renders a nonzero value as zero: the grid is relative below
//! its cap and has no floor.
//!
//! # The character bound
//!
//! A sentence is as wide as it needs to be; a box is not. Every text
//! these doors return for one number is at most [`MAX_CHARS`]
//! characters, sign aside, because `geom_core::Told` spells a number
//! in the notation `geom_core::Readable` picks by magnitude: decimal
//! between `1e-4` and `1e16`, scientific outside, and never more than
//! seventeen significant figures. The bound is `geom-core`'s
//! (`geom_core::WIDEST_NUMBER`, pinned there over every role and
//! unit), read here and not restated.

use pncad::geom_core::{self, Tell, Told};
use pncad::quantity::{Notation, UnitDef, UnitQuantity};

/// The longest spelling of one number these doors return, in
/// characters, sign aside (`geom_core::WIDEST_NUMBER` counts the sign).
///
/// A box that meets it shows every text this module returns. A box
/// narrower than this clips, and a clipped render reads as a different
/// value: that is the box's number to meet, not this one's to lower.
///
/// A sentence is bounded by its region instead:
/// `crate::widgets::message`'s *Characters or width* states the rule.
pub const MAX_CHARS: usize = geom_core::WIDEST_NUMBER - 1;

/// Every character a number longer than four characters is spelled in:
/// the digits, the point, the sign and the exponent's `e`. The
/// non-finite spellings (`NaN`, `inf`, `-inf`) are the only ones with
/// other letters, and none is wider than four widest glyphs.
pub const GLYPHS: &str = "0123456789.-e";

/// **How wide the widest text one number renders as is**, in a font
/// whose advance for a character is `advance(character)`:
/// [`MAX_CHARS`] and the sign, each at the widest of [`GLYPHS`].
pub fn widest_render(mut advance: impl FnMut(char) -> f32) -> f32 {
    let widest = GLYPHS.chars().map(&mut advance).fold(0.0, f32::max);
    (MAX_CHARS + 1) as f32 * widest
}

/// The dimension a value read in `unit` has.
fn dimension(unit: UnitDef) -> geom_core::Dimension {
    geom_core::Dimension::of(unit.quantity())
}

/// A canonical value nobody wrote, told in `unit` with its unit word.
///
/// Where `unit` cannot say the value (its factor carries it out of the
/// format) the text is in the canonical unit and says so: `1e306 m`
/// for a length a millimetre reader asked about.
pub fn computed(canonical: f64, unit: UnitDef) -> String {
    Told::computed(&canonical, dimension(unit))
        .told_in(Notation::CANONICAL.with(unit))
        .to_string()
}

/// [`computed`]'s number alone, for a surface that writes the unit
/// itself; `None` where `unit` cannot say the value, since a number
/// with the wrong unit beside it is a different value.
pub fn computed_number(canonical: f64, unit: UnitDef) -> Option<String> {
    Told::computed(&canonical, dimension(unit))
        .number_in(Notation::CANONICAL.with(unit))
        .map(|number| number.to_string())
}

/// A value the chrome computed, told on the grid in the canonical unit
/// of `quantity` and without that unit's word, for a sentence that
/// words the unit itself: `NaN` and `inf` for a value that is neither
/// a number nor finite.
pub fn bare(value: f64, quantity: UnitQuantity) -> String {
    canonical_number(&Told::computed(&value, geom_core::Dimension::of(quantity)))
}

/// A quantity's number in the canonical unit, which says every
/// quantity.
fn canonical_number(told: &Told) -> String {
    told.number_in(Notation::CANONICAL)
        .map_or_else(|| told.to_string(), |number| number.to_string())
}

/// The number a field holds, as the field shows it.
///
/// **A field's formatter has the number and not its unit**: `egui`
/// hands a `DragValue`'s formatter the `f64` the widget is bound to,
/// which is the canonical value already divided by its unit's factor.
/// So this door cannot say what the quantity is, and tells the number
/// as a computed length in the canonical unit: four figures, and a
/// tenth of ε of the WRITTEN number. Every length unit's factor is at
/// most one, so that is at least as fine as the rule's grid for a
/// length and finer than it for an angle or a ratio. Threading the
/// unit to the formatter is
/// `work/vgeom/the-field-door-tells-a-written-number-with-no-unit-in-hand.md`.
pub fn field_number(value: f64) -> String {
    canonical_number(&as_field(value))
}

/// Whether `spelling` names `value` as a field shows it: it reads as a
/// number that [`field_number`] spells the same way.
///
/// Width is no part of it: a caller judging a spelling someone else
/// chose is asking only whether that text names this value.
pub fn names(spelling: &str, value: f64) -> bool {
    spelling
        .parse::<f64>()
        .is_ok_and(|read| as_field(read) == as_field(value))
}

/// A field's number as the told quantity [`field_number`] renders.
fn as_field(value: f64) -> Told {
    Told::computed(&value, geom_core::Dimension::Length)
}

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use super::{MAX_CHARS, bare, computed, computed_number, field_number, names};
    use quantity::{DEG, IN, M, MM, ONE, PI, UnitQuantity};

    #[test]
    fn a_computed_value_reads_in_its_unit_on_the_grid() {
        assert_eq!(computed(0.025, MM.def()), "25 mm");
        assert_eq!(computed(1.5, M.def()), "1.5 m");
        assert_eq!(computed(0.0254, IN.def()), "1 in");
        assert_eq!(computed(core::f64::consts::FRAC_PI_2, DEG.def()), "90 deg");
        assert_eq!(computed(0.1 + 0.2, ONE.def()), "0.3");
        // A tenth of ε is 1e-7 mm: the micrometre a length holds is kept,
        // and float noise below the grid is not.
        assert_eq!(computed(1.000_001, MM.def()), "1000.001 mm");
        assert_eq!(computed(0.008_000_000_000_000_002, MM.def()), "8 mm");
        assert_eq!(computed(3.746_123_456e-7, MM.def()), "0.0003746 mm");
        assert_eq!(computed(0.0, MM.def()), "0 mm");
    }

    #[test]
    fn a_value_the_unit_cannot_say_reads_in_the_canonical_unit() {
        assert_eq!(computed(1.0e306, MM.def()), "1e306 m");
        assert_eq!(computed_number(1.0e306, MM.def()), None);
        assert_eq!(computed(5.0e-324, PI.def()), "5e-324 rad");
        assert_eq!(computed_number(5.0e-324, PI.def()), None);
        assert_eq!(computed_number(0.025, MM.def()).as_deref(), Some("25"));
        assert_eq!(computed_number(0.25, ONE.def()).as_deref(), Some("0.25"));
    }

    #[test]
    fn a_bare_number_is_told_on_its_quantitys_grid() {
        assert_eq!(bare(0.1 + 0.2, UnitQuantity::Scalar), "0.3");
        assert_eq!(bare(3.300_000_000_000_000_3, UnitQuantity::Angle), "3.3");
        assert_eq!(bare(1_234.500_000_1, UnitQuantity::Length), "1234.5000001");
        assert_eq!(bare(1_234.500_000_1, UnitQuantity::Angle), "1235");
        assert_eq!(bare(-0.0, UnitQuantity::Length), "0");
        assert_eq!(bare(f64::NAN, UnitQuantity::Length), "NaN");
        assert_eq!(bare(f64::INFINITY, UnitQuantity::Angle), "inf");
    }

    /// No nonzero value reads as zero, every render reads back within
    /// the coarser of a tenth of ε and four figures of the value, and
    /// none passes the character bound.
    #[test]
    fn a_field_number_names_its_value_within_the_bound() {
        let mut value = 1.0e-320_f64;
        while value < 1.0e300 {
            for signed in [value, -value] {
                let text = field_number(signed);
                let read: f64 = text
                    .parse()
                    .unwrap_or_else(|error| panic!("{signed} renders as {text}: {error}"));
                assert!(read != 0.0, "{signed} renders as {text}");
                assert!(
                    (read - signed).abs() <= 1.0e-10_f64.max(signed.abs() * 5.0e-4),
                    "{signed} renders as {text}"
                );
                assert!(
                    text.trim_start_matches('-').chars().count() <= MAX_CHARS,
                    "{signed} renders as {text}, past the {MAX_CHARS} character bound"
                );
                assert!(names(&text, signed), "{text} does not name {signed}");
            }
            value *= 1.5;
        }
        assert_eq!(field_number(0.0), "0");
        assert_eq!(field_number(1000.001), "1000.001");
        assert_eq!(field_number(1024.0), "1024");
        assert_eq!(field_number(0.0003746), "0.0003746");
        assert_eq!(field_number(4.0e-7), "4e-7");
        // Past the last four-figure decimal the format holds, the grid
        // point below.
        assert_eq!(field_number(f64::MAX), "1.797e308");
    }

    #[test]
    fn a_spelling_names_a_value_it_is_told_as() {
        assert!(names("1000.001", 1000.001));
        assert!(names("0.3", 0.1 + 0.2));
        assert!(!names("1000.0", 1000.001));
        assert!(!names("0.000", 4.0e-7));
        assert!(names("inf", f64::INFINITY));
        assert_eq!(field_number(f64::INFINITY), "inf");
        assert!(!names("not a number", 1.0));
    }
}
