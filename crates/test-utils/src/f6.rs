//! **F6 — a façade-carried refusal renders as a sentence, and never as
//! its own `Debug` dump** (the ratified `Display` contract, #1111).
//!
//! The predicate lives here, once, so that a clause added to F6 reaches
//! every suite that asserts it rather than one of N copies
//! (`work/view/f6-display-predicate-is-spelled-three-times-with-no-home`).
//!
//! The variant identifier is read off the VALUE
//! ([`variant_identifier`]), so it cannot fall behind a rename. The
//! other two fingerprints are the caller's, because only the caller
//! knows which enum the value belongs to (`dumps`, the roster of
//! SIBLING identifiers a rendering must not leak) and which field
//! punctuation would be a dump rather than a door's own prose
//! (`fields`).
//!
//! **Why `fields` is not derived from the payload's `Debug`.** It was
//! tried: scanning `format!("{err:?}")` for `field: ` tokens and
//! banning each from the rendering. It false-positives on a door whose
//! PREFIX is a field name — `MeshPickError::PositionOutOfRange`'s own
//! sentence opens "pick index: triangle 5 of patch 1 …" while its
//! payload has an `index` field, and that sentence is prose, not a
//! dump. A derivation that has to be exempted per site is a hand list
//! wearing a derivation's clothes, so the roster stays the caller's and
//! stays small.

use core::fmt::{Debug, Display};

/// The variant identifier of a value, read off its own derived
/// `Debug`: everything before the delimiter a derived `Debug` puts
/// after the name — a space for a struct variant, `(` for a tuple
/// variant, and end of string for a unit variant.
///
/// **Reading it off the value is the point.** A name typed into a
/// `match` arm beside a pattern is a second spelling nothing checks:
/// rustc checks the pattern and never the string, so a variant renamed
/// in `src/` forces the pattern to change and leaves the string saying
/// the old name. A ban list built from such strings then bans an
/// identifier no rendering can produce, and reads as a guard while
/// guarding nothing.
///
/// # Panics
///
/// If the value's `Debug` does not open on an identifier. A derived
/// `Debug` always does; a hand-written one need not, and this says so
/// rather than returning a fragment of somebody's prose.
#[must_use]
pub fn variant_identifier<E: Debug>(err: &E) -> String {
    let debug = format!("{err:?}");
    let name = debug.split([' ', '(', '{']).next().unwrap_or_default();
    assert!(
        name.starts_with(|c: char| c.is_alphabetic() || c == '_'),
        "{debug:?} does not open on a variant identifier — this reads a DERIVED `Debug`, \
         and a hand-written one need not carry the name"
    );
    name.to_string()
}

/// Asserts the F6 shape over one rendering: the wanted content is
/// present, no banned variant identifier leaks, no `Debug` field
/// punctuation leaks, and the sentence is not simply the dump.
///
/// `dumps` is the roster of variant identifiers this rendering must not
/// contain — the enum's own, so that a rendering which leaks a SIBLING
/// arm's name fails here too, not just one that leaks its own.
/// `fields` are the `Debug` field-name tokens (`"node:"`) that would be
/// a dump in this enum's sentences; `{` is universal and is checked
/// whatever the caller passes.
///
/// # Panics
///
/// On any of the four, naming the rendering and what was wrong with it.
pub fn assert_f6<E: Debug + Display>(err: &E, wants: &[&str], dumps: &[&str], fields: &[&str]) {
    let shown = err.to_string();
    for want in wants {
        assert!(
            shown.contains(want),
            "{err:?} renders as {shown:?}, missing {want:?}"
        );
    }
    for dump in dumps {
        assert!(
            !shown.contains(dump),
            "{err:?} renders as {shown:?} — that is the variant name, i.e. a struct dump"
        );
    }
    assert!(
        !shown.contains('{') && !fields.iter().any(|field| shown.contains(field)),
        "{err:?} renders as {shown:?} — that is Debug punctuation, not a sentence"
    );
    assert_ne!(shown, format!("{err:?}"));
}

#[cfg(test)]
mod tests {
    use super::{assert_f6, variant_identifier};

    #[derive(Debug)]
    enum Shape {
        Unit,
        Tuple(u32),
        Struct { node: u32, name: &'static str },
    }

    impl core::fmt::Display for Shape {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::Unit => f.write_str("nothing was passed"),
                Self::Tuple(n) => write!(f, "node {n} answered nothing"),
                Self::Struct { node, name } => write!(f, "node {node} has no {name}"),
            }
        }
    }

    /// The identifier comes off the value for all three variant shapes
    /// a derived `Debug` can produce.
    #[test]
    fn a_variant_identifier_is_read_off_every_debug_shape() {
        assert_eq!(variant_identifier(&Shape::Unit), "Unit");
        assert_eq!(variant_identifier(&Shape::Tuple(4)), "Tuple");
        assert_eq!(
            variant_identifier(&Shape::Struct {
                node: 4,
                name: "face"
            }),
            "Struct"
        );
    }

    /// The shape passes on prose and fails on the dump — including on a
    /// sibling arm's identifier, which is the half a per-arm check
    /// cannot see.
    #[test]
    fn the_shape_passes_prose_and_fails_a_dump() {
        let dumps = ["Unit", "Tuple", "Struct"];
        let fields = ["node:", "name:"];
        assert_f6(&Shape::Tuple(4), &["node 4"], &dumps, &fields);

        let leaks_a_sibling = std::panic::catch_unwind(|| {
            assert_f6(
                &Shape::Struct {
                    node: 4,
                    name: "Tuple",
                },
                &[],
                &dumps,
                &fields,
            );
        });
        assert!(leaks_a_sibling.is_err(), "a sibling identifier is a dump");

        let leaks_a_field = std::panic::catch_unwind(|| {
            assert_f6(
                &Shape::Struct {
                    node: 4,
                    name: "node: 4",
                },
                &[],
                &dumps,
                &fields,
            );
        });
        assert!(leaks_a_field.is_err(), "field punctuation is a dump");
    }
}
