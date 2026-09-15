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

// The weld REPORTS a drifted roster by panicking, deliberately: this is
// an assertion helper, and a set difference that returned an error for a
// caller to ignore would be the failure it exists to prevent. The
// workspace's no-panic rule is about production code, and no production
// manifest names this crate (see the crate docs).
#![allow(clippy::panic)]

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

/// Runs the F6 shape over one error enum's whole case list, with the
/// enum's own variant identifiers as the ban list, and reports any
/// variant the cases do not reach.
///
/// **This is the weld's one home.** A suite that spells the pair out
/// again — an exhaustiveness `match` beside a roster beside a set
/// comparison — is a second copy of the mechanism kept in step by
/// hand, which is the defect the mechanism exists to catch.
///
/// **What each half actually guarantees.** `exhaustive` is a
/// wildcard-free `match` over the enum and NOTHING else: it names no
/// identifiers, so the only thing it can do is stop compiling. That is
/// its whole job — a variant added to the enum, or renamed, leaves the
/// `match` non-exhaustive and forces the author to open the suite. It
/// is not itself a census, because the compiler cannot tell whether the
/// author then did the right thing. `all` is the identifier roster,
/// written out; the set difference below is what welds it. Every
/// identifier in `all` must be produced by some case's own `Debug`
/// ([`variant_identifier`]) and every case's must be in `all`, so the
/// roster cannot drift in either direction and a MISSPELLING in it
/// fails — nothing here trusts a string typed beside a pattern, which
/// rustc never checks.
///
/// **The one hole, stated.** An author who adds a variant, adds its arm
/// to `exhaustive` — which the compiler makes them do — and then adds
/// NEITHER a case NOR an `all` entry is not caught: nothing renders the
/// variant, so nothing contradicts a roster that never grew. The
/// compile error is what stands between that and an accident; closing
/// it would need the variant list itself to be derivable, which safe
/// Rust does not offer without a macro or a derive over a type the
/// asserting crate does not own. **A site that adopts this points
/// here rather than restating it**, so the hole has one description
/// that cannot drift from the mechanism.
///
/// `also_banned` carries identifiers from OTHER enums that a rendering
/// must not leak either; `fields` is this enum's field punctuation, as
/// for [`assert_f6`].
///
/// # Panics
///
/// On any F6 violation in any case, and on a roster that disagrees with
/// the cases in either direction.
pub fn assert_f6_every_variant<E: Debug + Display>(
    cases: &[(E, Vec<&str>)],
    exhaustive: fn(&E),
    all: &[&str],
    also_banned: &[&str],
    fields: &[&str],
) {
    let dumps: Vec<&str> = all.iter().chain(also_banned).copied().collect();
    for (err, wants) in cases {
        exhaustive(err);
        assert_f6(err, wants, &dumps, fields);
    }
    let covered_words: Vec<String> = cases
        .iter()
        .map(|(err, _)| variant_identifier(err))
        .collect();
    let covered: Vec<&str> = covered_words.iter().map(String::as_str).collect();
    if let Some(report) = crate::census::set_difference(
        all,
        &covered,
        // The enum is named by its own roster rather than by
        // `type_name`: the punning tripwire
        // (`scripts/gates/bit-identity-punning.sh`) forbids reaching
        // `core::any` outside `geom-core/src/bit_identity.rs`, and the
        // roster is the caller's data, so it cannot fall behind a
        // rename the way a hand-typed subject would.
        &format!("the identifier roster {all:?} and its rendered cases disagree"),
        "rendered by a case and absent from the roster — add it, spelled as `Debug` renders it",
        "in the roster and rendered by no case — give it a case, or fix its spelling",
    ) {
        panic!("{report}");
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{assert_f6, assert_f6_every_variant, variant_identifier};
    use crate::panic_capture::caught;

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

    /// `Shape`'s exhaustiveness token: no wildcard arm, no strings.
    fn shape_is_exhaustive(s: &Shape) {
        match s {
            Shape::Unit | Shape::Tuple(..) | Shape::Struct { .. } => (),
        }
    }

    fn shape_cases() -> Vec<(Shape, Vec<&'static str>)> {
        vec![
            (Shape::Unit, vec!["nothing was passed"]),
            (Shape::Tuple(4), vec!["node 4"]),
            (
                Shape::Struct {
                    node: 4,
                    name: "face",
                },
                vec!["no face"],
            ),
        ]
    }

    /// The weld reds in BOTH directions and green only when the roster
    /// and the rendered cases are the same set — which is the half the
    /// exhaustiveness token cannot check, because rustc never reads a
    /// string typed beside a pattern.
    #[test]
    fn the_roster_is_welded_to_the_cases_in_both_directions() {
        let all = ["Unit", "Tuple", "Struct"];
        let fields = ["node:", "name:"];
        assert_f6_every_variant(&shape_cases(), shape_is_exhaustive, &all, &[], &fields);

        let said = caught(|| {
            assert_f6_every_variant(
                &shape_cases(),
                shape_is_exhaustive,
                &["Unit", "Tuple"],
                &[],
                &fields,
            );
        })
        .expect("a case rendered by no roster entry is a drifted roster");
        assert!(said.contains("\"Struct\""), "the report names it: {said}");

        let said = caught(|| {
            assert_f6_every_variant(
                &shape_cases(),
                shape_is_exhaustive,
                &["Unit", "Tuple", "Strukt"],
                &[],
                &fields,
            );
        })
        .expect("a misspelt roster entry is witnessed by no case");
        assert!(
            said.contains("\"Strukt\"") && said.contains("\"Struct\""),
            "both directions are named: {said}"
        );
    }
}
