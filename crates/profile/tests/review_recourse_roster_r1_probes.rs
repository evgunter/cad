//! **What the recourse roster does not pin: which sentence a routed
//! name gets.**
//!
//! The roster asserts that every decided name is *routed or listed*. It
//! measures that by asking whether the rendered refusal carries the gap
//! sentence, so a name wired to ANOTHER layer's sentence is still
//! "routed" and the roster stays green. The row here pins the
//! pairing: each layer of `PathError::Escalated`'s dispatch owns a
//! sentence, and every name in that layer renders that sentence and no
//! other layer's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Indeterminate, MarginDiag, Tol};
use profile::PathError;

/// The layers of the dispatch, each with the opening of the sentence it
/// owns. A name renders its own layer's opening and no other's.
const LAYERS: &[(&str, &[&str])] = &[
    (
        "the fillet at this corner is undecided:",
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
        "the fillet arc about to be stored is undecided:",
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
///
/// The table is also held COMPLETE against the crate's own decided
/// names: a name the door routes that no layer here claims reds, so a
/// gate cannot be given a sentence without being paired with one.
#[test]
fn every_routed_name_renders_the_sentence_its_own_layer_owns() {
    let mut paired: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    for (opening, names) in LAYERS {
        for name in *names {
            assert!(
                paired.insert(name),
                "`{name}` is in two layers of the table"
            );
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
    let src = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    for name in test_utils::source::predicate_census(&src, profile_carriers()).names {
        let name: &'static str = Box::leak(name.into_boxed_str());
        let routed = !rendered(name).starts_with("escalated at the path door:");
        assert_eq!(
            routed,
            paired.contains(name),
            "`{name}` is {} by the door and {} in this table; it renders: {}",
            if routed { "routed" } else { "unrouted" },
            if paired.contains(name) {
                "paired"
            } else {
                "unpaired"
            },
            rendered(name)
        );
    }
}

/// The carriers `recourse_roster` declares — read from that one home,
/// so the completeness half of the pairing row and the roster can never
/// cover different name sets (the delta re-verification found this
/// list spelled twice).
fn profile_carriers() -> &'static [test_utils::source::NameCarrier] {
    crate::recourse_roster::CARRIERS
}
