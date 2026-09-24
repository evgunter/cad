//! Review probes for the recourse roster (R2).
//!
//! Three pins the roster suites leave to inference, each measured by
//! rendering a door's own refusal and reading the text:
//!
//! - **the two routing doors render ONE gap sentence for one unknown
//!   name.** Each roster suite reads its own door; this crate can see
//!   both, so the sameness is measured here rather than argued from the
//!   shared home — for a named escalation and for a nameless one.
//! - **the validator's door routes no recourse by name.**
//!   `ProfileError::Escalated` carries the other predicate-keyed table
//!   in `profile` (the near-tangency addendum): it appends a site note
//!   for exactly three carrier names at a segment pair, and its default
//!   appends nothing. The shared recourse rides `{source}` for every
//!   name, so an unknown name is neither categorised nor told the
//!   table has a hole — there is no table of recourses to have one.
//! - **the tube door's predicate-keyed table routes a DOOR NAME, not a
//!   recourse.** `HOLLOW_PREDICATES` picks which of the two tube doors
//!   a wall escalation names; every other name, and no name, reads
//!   "tube door", with the shared recourse and no gap sentence.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, COINCIDENCE_RECOURSE, Indeterminate, MarginDiag, MissingRecourse, Tol};
use profile::{EscalationSite, PathError, ProfileError, SegmentRef};
use sweep::TubeError;
use sweep::blend::{BlendError, BlendSite};

/// An escalation carrying `name`, its margin inside the run's band.
fn escalation(name: Option<&'static str>) -> Indeterminate {
    let band = Band::linear(Tol::witness()).expect("the run's band forms");
    Indeterminate {
        margin: MarginDiag::Value((band.zero() + band.escalate()) / 2.0),
        band,
        predicate: name,
    }
}

const UNKNOWN: &str = "roster_unknown_probe";

/// **One unknown name, one sentence, at both doors — and the same for
/// an escalation carrying no name at all.**
///
/// The sentence is the tail of each refusal, byte for byte, so the two
/// doors cannot answer the same question differently; the shared
/// coincidence recourse still arrives ahead of it through `{source}`
/// (the class `every-escalation-carries-the-coincidence-recourse-first`
/// names, unchanged here); and the profile door asserts no category.
#[test]
fn the_two_doors_render_one_gap_sentence_for_one_unknown_name() {
    for name in [Some(UNKNOWN), None] {
        let sentence = MissingRecourse(name).to_string();
        let path = PathError::<f64>::Escalated {
            source: escalation(name),
        }
        .to_string();
        let blend = BlendError::Escalated {
            site: BlendSite::Chain,
            source: escalation(name),
        }
        .to_string();
        for text in [&path, &blend] {
            assert!(
                text.ends_with(&sentence),
                "the refusal ends in the one gap sentence: {text}"
            );
            assert!(
                text.contains(COINCIDENCE_RECOURSE),
                "the shared recourse still rides the payload: {text}"
            );
        }
        assert!(
            !path.contains("path junction classification"),
            "no category over an unknown name: {path}"
        );
        // Both doors name where the escalation happened before the
        // payload. `BlendError::Escalated` carries a site field and
        // names it; `PathError::Escalated` carries none, so the door
        // names itself.
        assert!(
            path.starts_with("escalated at the path door: ") && blend.starts_with("escalated at "),
            "each door names a site ahead of the payload: {path} / {blend}"
        );
    }
}

/// **The validator's door routes no recourse by name.**
///
/// Its predicate-keyed table decides only whether a site note is
/// appended, and only at a segment pair for the three carrier names;
/// every name — routed nowhere, because there is nowhere to route it —
/// renders the shared recourse whole, never the gap sentence and never
/// a category.
#[test]
fn the_validator_door_appends_a_site_note_and_routes_nothing() {
    let pair = EscalationSite::SegmentPair(
        SegmentRef {
            loop_index: 0,
            segment_index: 0,
        },
        SegmentRef {
            loop_index: 0,
            segment_index: 1,
        },
    );
    const NOTE: &str = "near-tangency:";
    for (name, noted) in [
        ("carrier_line_circle", true),
        ("carrier_circles_external", true),
        ("carrier_circles_internal", true),
        ("carrier_circles_identity", false),
        ("path_junction_turn", false),
        (UNKNOWN, false),
    ] {
        let text = ProfileError::Escalated {
            site: pair,
            source: escalation(Some(name)),
        }
        .to_string();
        assert!(
            text.starts_with(&format!(
                "validation escalated between loop 0 segment 0 and loop 0 segment 1: \
                 predicate '{name}' indeterminate:"
            )),
            "{text}"
        );
        assert!(text.contains(COINCIDENCE_RECOURSE), "{text}");
        assert!(
            !text.contains("no recourse is recorded") && !text.contains("junction classification"),
            "neither a gap sentence nor a category: {text}"
        );
        assert_eq!(text.contains(NOTE), noted, "{text}");
    }
    // Away from a segment pair the note never appears, even for a
    // carrier name: the key is (site, name), not the name alone.
    let text = ProfileError::Escalated {
        site: EscalationSite::Loop { loop_index: 0 },
        source: escalation(Some("carrier_line_circle")),
    }
    .to_string();
    assert!(!text.contains(NOTE), "{text}");
}

/// **The tube's table routes which tube it names, never a recourse.**
///
/// The three wall names read "the hollow tube"; every other name, and
/// a nameless escalation, reads "the tube" — the honest answer for a
/// predicate both tube doors can reach — and every one of them renders
/// the shared recourse whole, with no gap sentence.
#[test]
fn the_tube_door_routes_a_door_name_and_never_a_recourse() {
    for (name, door) in [
        ("tube_wall", "the hollow tube"),
        ("tube_wall_bore", "the hollow tube"),
        ("tube_wall_gap", "the hollow tube"),
        ("tube_frame_unit", "the tube"),
        ("tube_window_span", "the tube"),
        (UNKNOWN, "the tube"),
    ] {
        let text = TubeError::Escalated {
            source: escalation(Some(name)),
        }
        .to_string();
        assert!(
            text.starts_with(&format!(
                "{door} escalated: predicate '{name}' indeterminate:"
            )),
            "{text}"
        );
        assert!(
            text.contains(COINCIDENCE_RECOURSE) && !text.contains("no recourse is recorded"),
            "{text}"
        );
    }
    let text = TubeError::Escalated {
        source: escalation(None),
    }
    .to_string();
    assert!(
        text.starts_with("the tube escalated: sign indeterminate:"),
        "{text}"
    );
}
