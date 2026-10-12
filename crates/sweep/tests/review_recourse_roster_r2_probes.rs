//! Review probes for the recourse roster (R2).
//!
//! Two pins the roster suites leave to inference, each measured by
//! rendering a door's own refusal and reading the text:
//!
//! - **the path door renders the one gap sentence for an unknown
//!   name**, named or nameless. The blend door has no such arm: its
//!   escalation carries a closed decision, as the tube's and the revolve
//!   clearance's do, so no name reaches it
//!   unrouted.
//! - **the validator's door routes no recourse by name.**
//!   `ProfileError::Escalated` carries the other predicate-keyed table
//!   in `profile` (the near-tangency addendum): it appends a site note
//!   for exactly three carrier names at a segment pair, and its default
//!   appends nothing. The shared recourse rides `{source}` for every
//!   name, so an unknown name is neither categorised nor told the
//!   table has a hole — there is no table of recourses to have one.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, COINCIDENCE_RECOURSE, Indeterminate, MarginDiag, MissingRecourse, Tol};
use profile::{EscalationSite, PathError, ProfileError, SegmentRef};

/// An escalation carrying `name`, its margin inside the run's band.
fn escalation(name: Option<&'static str>) -> Indeterminate {
    let band = Band::linear(Tol::witness()).expect("the run's band forms");
    Indeterminate {
        margin: MarginDiag::value((band.zero() + band.escalate()) / 2.0),
        band,
        predicate: name,
        terminal_sliver: false,
    }
}

const UNKNOWN: &str = "roster_unknown_probe";

/// **An unknown name renders the one gap sentence at the path door —
/// and the same for an escalation carrying no name at all.**
///
/// The sentence is the tail of the refusal, byte for byte, the home
/// every name-keyed table's fall-through composes; the shared
/// coincidence recourse still arrives ahead of it through `{source}`
/// (the class `every-escalation-carries-the-coincidence-recourse-first`
/// names, which the path door's junction keys keep on purpose); and the
/// door asserts no category. The blend door is not a leg here: its
/// `Escalated` carries a `BlendDecision`, so a name it does not route
/// cannot be built.
#[test]
fn the_path_door_renders_the_gap_sentence_for_an_unknown_name() {
    for name in [Some(UNKNOWN), None] {
        let sentence = MissingRecourse(name).to_string();
        let path = PathError::<f64>::Escalated {
            source: escalation(name),
        }
        .to_string();
        assert!(
            path.ends_with(&sentence),
            "the refusal ends in the one gap sentence: {path}"
        );
        assert!(
            path.contains(COINCIDENCE_RECOURSE),
            "the shared recourse still rides the payload: {path}"
        );
        assert!(
            !path.contains("at this junction"),
            "no category over an unknown name: {path}"
        );
        assert!(
            path.starts_with("an unnamed decision is too close to call: "),
            "the door names what escalated ahead of the payload: {path}"
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
            text.starts_with(
                "validation escalated between loop 0 segment 0 and loop 0 segment 1: "
            ),
            "{text}"
        );
        assert!(text.contains(COINCIDENCE_RECOURSE), "{text}");
        assert!(
            !text.contains("no recourse is recorded"),
            "no gap sentence: {text}"
        );
        // The door states each name's own subject, and a name it has no
        // words for as unnamed, never another name's.
        assert_eq!(
            text.contains(geom_core::UNNAMED_DECISION),
            name == UNKNOWN,
            "{text}"
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
