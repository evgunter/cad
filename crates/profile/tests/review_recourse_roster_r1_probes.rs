//! **Two things the recourse roster does not pin: which sentence a
//! routed name gets, and the spelling its reader can read.**
//!
//! The roster asserts that every decided name is *routed or listed*. It
//! measures that by asking whether the rendered refusal carries the gap
//! sentence, so a name wired to ANOTHER layer's sentence is still
//! "routed" and the roster stays green. The first row here pins the
//! pairing: each layer of `PathError::Escalated`'s dispatch owns a
//! sentence, and every name in that layer renders that sentence and no
//! other layer's.
//!
//! The second row pins the roster reader's own precondition. Its
//! `decide*` scan admits a token whose suffix is alphanumeric and skips
//! everything else, so `decide::<f64>("…")` — an ordinary spelling of a
//! generic call — is neither read as a name nor recorded as an indirect
//! site. The reader is only fail-loud while nothing in `src` is spelled
//! that way, and that is what this row measures.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Indeterminate, MarginDiag, Tol};
use profile::PathError;

/// The layers of the dispatch, each with the opening of the sentence it
/// owns. A name renders its own layer's opening and no other's.
const LAYERS: &[(&str, &[&str])] = &[
    (
        "resolving the fillet at this corner,",
        &[
            "fillet_corner_arm",
            "fillet_corner_turn",
            "fillet_enclosing_carrier",
            "fillet_leg_fit",
            "fillet_leg_reach",
            "fillet_offset_circles_external",
            "fillet_offset_circles_internal",
            "fillet_offset_lever",
            "fillet_offset_line_circle",
        ],
    ),
    (
        "the declared straight continuation's target",
        &["path_continuation_target_offset"],
    ),
    (
        "the declared seam arrival",
        &["path_seam_arrival_turn", "path_seam_arrival_side"],
    ),
    (
        "an authored leg extent could not be told from zero",
        &["path_leg_length"],
    ),
    (
        "reading back the fillet arc this door is about to store,",
        &[
            "arc_diameter_clearance",
            "carrier_circles_external",
            "carrier_circles_identity",
            "carrier_circles_internal",
            "carrier_line_circle",
            "chord_side",
            "segment_straightness",
            "vertex_separation",
        ],
    ),
    (
        "path junction classification:",
        &["path_junction_side", "path_junction_turn"],
    ),
];

fn rendered(name: &'static str) -> String {
    let band = Band::linear(Tol::witness()).expect("the run's band forms");
    PathError::<f64>::Escalated {
        source: Indeterminate {
            margin: MarginDiag::Value((band.zero() + band.escalate()) / 2.0),
            band,
            predicate: Some(name),
        },
    }
    .to_string()
}

/// **A routed name renders the sentence its own layer owns.**
///
/// The roster row beside this one measures routed-or-listed: a name
/// rewired from one layer to another stays routed, so it stays green.
/// The pairing is what a reader of the refusal actually depends on —
/// the leg-extent lever is not the stored form's, and the fillet
/// corner's is neither — so it is pinned here, name by name.
#[test]
fn every_routed_name_renders_the_sentence_its_own_layer_owns() {
    for (opening, names) in LAYERS {
        for name in *names {
            let text = rendered(name);
            assert!(
                text.starts_with(opening),
                "`{name}` should open with {opening:?}; it renders: {text}"
            );
            for (other, _) in LAYERS {
                assert!(
                    other == opening || !text.starts_with(other),
                    "`{name}` renders another layer's sentence: {text}"
                );
            }
        }
    }
}

/// **No `decide` call in this crate's `src` is spelled with a
/// turbofish.**
///
/// `recourse_roster`'s reader scans for a `decide` token whose suffix up
/// to the paren is alphanumeric. `decide::<f64>(…)` fails that test and
/// the call is skipped entirely — neither a name nor an indirect site —
/// so a gate spelled that way reaches the door's fall-through with no
/// row going red. While this holds, the roster's completeness claim
/// holds with it.
#[test]
fn no_decide_call_in_src_is_spelled_with_a_turbofish() {
    let src = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut found = Vec::new();
    for path in test_utils::source::rust_sources(&src) {
        let text = std::fs::read_to_string(&path).expect("a readable source file");
        let code = test_utils::source::code_and_literals(&text);
        if code.contains("decide::<") || code.contains("decide ::<") {
            found.push(path.display().to_string());
        }
    }
    assert!(
        found.is_empty(),
        "a `decide` call is spelled with a turbofish, which the recourse roster's reader \
         skips without recording: {found:?}"
    );
}
