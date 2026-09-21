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
/// **What it cannot tell you.** The check below is that the first token
/// opens on an identifier, not that it is the VARIANT's identifier —
/// there is nothing to compare it against. Over a derived `Debug` those
/// are the same thing; over a HAND-WRITTEN one they need not be, and a
/// `Debug` that opens on the type name (or on any other identifier)
/// yields a wrong token here, which a roster then gets "fixed" to match.
/// The weld is green and mirrors nothing. Every enum this is used on in
/// this tree derives `Debug`; nothing here enforces that.
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

/// **The compiler-checked pair the weld runs on**: a wildcard-free
/// `match` over one enum, and that enum's roster of variant
/// identifiers.
///
/// The fields are private, so there are exactly two ways to build one:
/// [`crate::f6_variants!`], from **one list of idents**, and
/// [`Self::hand_written`], for the one enum shape that macro cannot
/// serve. They are the same construction under two names, and the name
/// is the point — it is visible at the call site, so a reader can tell
/// which of the two guarantees below they are looking at.
///
/// As two loose parameters (`fn(&E)` and `&[&str]`) the pair was a
/// convention: neither type says the function is a `match` at all, so a
/// token that was a no-op, or one whose only arm was `_`, passed and
/// left the roster answering to nothing.
///
/// Built by the macro, none of that is expressible:
///
/// 1. The `match` is written by the macro from the idents, so there is
///    no body for an author to leave empty.
/// 2. `_` is not an `ident`, so a wildcard arm is rejected by the
///    macro's own grammar before rustc sees it.
/// 3. A variant added to the enum leaves the generated `match`
///    non-exhaustive (`error[E0004]`) and a misspelt one names no
///    variant (`error[E0599]`), so both halves are rustc's.
/// 4. The roster is `stringify!` over the SAME idents, so it cannot
///    disagree with the patterns — the shape
///    [`crate::roster!`] already uses one file over.
pub struct VariantCensus<E> {
    token: fn(&E),
    all: &'static [&'static str],
}

impl<E> VariantCensus<E> {
    /// [`crate::f6_variants!`]'s entry point, hidden because the macro
    /// is the documented spelling of it. It carries no guarantee of its
    /// own — what the macro buys is what the macro WRITES, and a caller
    /// reaching past it has the same pair [`Self::hand_written`] names.
    #[doc(hidden)]
    #[must_use]
    pub const fn from_variant_idents(token: fn(&E), all: &'static [&'static str]) -> Self {
        Self { token, all }
    }

    /// The census for an enum the asserting crate **cannot** match
    /// exhaustively, where the macro's guarantees are unavailable and
    /// this says so at the call site.
    ///
    /// The live case is a `#[non_exhaustive]` enum from another crate:
    /// rustc forces a catch-all arm, so the `match` no longer stops
    /// compiling when a variant is added, and the roster beside it is
    /// back to being a list kept in step by hand. A site that reaches
    /// for this owes the reader why its enum cannot use
    /// [`crate::f6_variants!`], and what covers the gap instead.
    #[must_use]
    pub const fn hand_written(token: fn(&E), all: &'static [&'static str]) -> Self {
        Self { token, all }
    }

    /// The variant identifiers, for a site that has to ban them
    /// somewhere this module's own assertions do not reach.
    #[must_use]
    pub const fn identifiers(&self) -> &'static [&'static str] {
        self.all
    }
}

/// **One ident per variant, feeding the `match` and the roster
/// together** — the [`VariantCensus`] `assert_f6_every_variant` takes.
///
/// ```ignore
/// test_utils::f6_variants! {
///     /// What this enum is, for a reader.
///     const CONTACT_REFUSAL: ContactRefusal =
///         [Contradicted, Escalated, Undeclared, NotCertifiable];
/// }
/// ```
///
/// The enum is named by a bare ident, so it must be in scope; each
/// variant is a bare ident, and the macro writes `Ty::Variant { .. }`
/// for it, which is a legal pattern for a unit, tuple or struct
/// variant alike. What that buys is in [`VariantCensus`]'s own docs.
#[macro_export]
macro_rules! f6_variants {
    ($(#[$meta:meta])* $vis:vis const $name:ident : $ty:ident = [$($variant:ident),+ $(,)?];) => {
        $(#[$meta])*
        $vis const $name: $crate::f6::VariantCensus<$ty> = {
            fn token(value: &$ty) {
                match value {
                    $($ty::$variant { .. } => (),)+
                }
            }
            $crate::f6::VariantCensus::from_variant_idents(
                token,
                &[$(stringify!($variant)),+],
            )
        };
    };
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
/// **What the two halves guarantee.** [`VariantCensus`]'s `match` can
/// only stop compiling: it names no identifiers, so it is not itself a
/// census. The set difference below is the census — every identifier
/// in the roster must be produced by some case's own `Debug`
/// ([`variant_identifier`]) and every case's must be in the roster, so
/// nothing here trusts a string typed beside a pattern, which rustc
/// never checks.
///
/// **What a variant added tomorrow costs.** The generated `match` stops
/// compiling, so the author writes the new ident into the
/// [`crate::f6_variants!`] block — and that block is the roster too, so
/// the identifier is now declared and rendered by no case, and this
/// reports it. The author cannot add the arm and stop: there is no
/// second list to forget. That was the hole while the token and the
/// roster were two parameters, and closing it is why they are one
/// value.
///
/// **What is still not welded.** The identifiers this compares the
/// roster against come from [`variant_identifier`], which reads a
/// DERIVED `Debug` and cannot tell it from a hand-written one — see its
/// docs. `fields` is a hand-written mirror of
/// the enum's payload field names — see this module's docs for why it
/// is not derived — and `also_banned` is whatever the caller passes.
/// Neither is checked against anything, and a case list that renders
/// one variant twice is indistinguishable here from one that renders it
/// once: the weld says every variant is REACHED, never that each is
/// reached once. **A site that adopts this points here rather than
/// restating it**, so those have one description that cannot drift from
/// the mechanism.
///
/// `also_banned` carries identifiers from OTHER enums that a rendering
/// must not leak either; `fields` is this enum's field punctuation, as
/// for [`assert_f6`].
///
/// # Panics
///
/// On any F6 violation in any case, on a case that names no content to
/// look for, and on a roster that disagrees with the cases in either
/// direction.
pub fn assert_f6_every_variant<E: Debug + Display>(
    cases: &[(E, Vec<&str>)],
    census: &VariantCensus<E>,
    also_banned: &[&str],
    fields: &[&str],
) {
    let all = census.all;
    // The anti-vacuity floor. Everything below is a claim about the
    // cases, and over no cases — or over cases that ask for no content
    // — all of it is vacuously true while still reporting that every
    // variant is reached. `wants` is where the content claim lives, so
    // it is where the floor goes.
    assert!(
        !cases.is_empty(),
        "an F6 census over NO cases asserts nothing about {all:?} and still reports every \
         variant reached — give the enum its cases"
    );
    for (err, wants) in cases {
        assert!(
            !wants.is_empty() && wants.iter().all(|want| !want.is_empty()),
            "{err:?}'s case names no content to look for, so the F6 shape over it says \
             only that its rendering is not the dump — name at least one phrase the \
             sentence must contain"
        );
    }
    let dumps: Vec<&str> = all.iter().chain(also_banned).copied().collect();
    for (err, wants) in cases {
        (census.token)(err);
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
    use super::{VariantCensus, assert_f6, assert_f6_every_variant, variant_identifier};
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
        let dumps = SHAPE.identifiers();
        let fields = ["node:", "name:"];
        assert_f6(&Shape::Tuple(4), &["node 4"], dumps, &fields);

        let leaks_a_sibling = std::panic::catch_unwind(|| {
            assert_f6(
                &Shape::Struct {
                    node: 4,
                    name: "Tuple",
                },
                &[],
                dumps,
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
                dumps,
                &fields,
            );
        });
        assert!(leaks_a_field.is_err(), "field punctuation is a dump");
    }

    crate::f6_variants! {
        /// `Shape`'s census: the `match` rustc checks and the roster
        /// the weld compares, from one list of idents.
        const SHAPE: Shape = [Unit, Tuple, Struct];
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

    /// The roster the macro writes is the identifier each value's own
    /// `Debug` produces — the half `stringify!` could get wrong on its
    /// own, and the reason the roster is not typed a second time.
    #[test]
    fn the_macro_roster_is_what_each_values_own_debug_produces() {
        let rendered: Vec<String> = shape_cases()
            .iter()
            .map(|(err, _)| variant_identifier(err))
            .collect();
        assert_eq!(rendered, SHAPE.identifiers());
    }

    /// The weld reds when the roster and the rendered cases are not the
    /// same set — which is the half the `match` cannot check, because
    /// rustc never reads a string typed beside a pattern.
    #[test]
    fn the_roster_is_welded_to_the_cases_in_both_directions() {
        let fields = ["node:", "name:"];
        assert_f6_every_variant(&shape_cases(), &SHAPE, &[], &fields);

        // A variant the enum has and the cases do not reach. This is
        // the live direction under a macro-built census: the roster
        // grows with the `match`, so what can go short is the cases.
        let mut short = shape_cases();
        short.pop();
        let said = caught(|| {
            assert_f6_every_variant(&short, &SHAPE, &[], &fields);
        })
        .expect("a rostered variant no case renders is a gap in the cases");
        assert!(said.contains("\"Struct\""), "the report names it: {said}");

        // The other direction, reachable only through the door for an
        // enum that cannot be matched exhaustively. A roster built by
        // hand can fall short of the cases in both directions at once;
        // one built by `f6_variants!` cannot fall short at all.
        let by_hand = VariantCensus::hand_written(SHAPE.token, &["Unit", "Strukt"]);
        let said = caught(|| {
            assert_f6_every_variant(&shape_cases(), &by_hand, &[], &fields);
        })
        .expect("a hand-built roster can disagree with the cases in both directions");
        assert!(
            said.contains("\"Strukt\"")
                && said.contains("\"Struct\"")
                && said.contains("\"Tuple\""),
            "both directions are named: {said}"
        );
    }

    /// The anti-vacuity floor: a census over no cases, or over cases
    /// that ask for no content, reported "every variant reached" while
    /// asserting nothing about any rendering.
    #[test]
    fn a_census_that_asserts_no_content_is_a_violation_not_a_green() {
        let empty: Vec<(Shape, Vec<&str>)> = vec![];
        let said = caught(|| {
            assert_f6_every_variant(&empty, &SHAPE, &[], &[]);
        })
        .expect("no cases is no assertion");
        assert!(said.contains("NO cases"), "{said}");

        let no_content = vec![(Shape::Unit, vec![]), (Shape::Tuple(4), vec![""])];
        let said = caught(|| {
            assert_f6_every_variant(&no_content, &SHAPE, &[], &[]);
        })
        .expect("a case with no content named is no assertion");
        assert!(said.contains("names no content"), "{said}");
    }
}
