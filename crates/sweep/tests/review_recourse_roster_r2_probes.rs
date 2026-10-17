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
//! - **the validator's door ends by the decision's row and notes a
//!   near-tangency.** `ProfileError::Escalated` ends each name as its
//!   row in `profile`'s `DECISION_ENDINGS` says, and names the hole for
//!   a name with no row, as the path door does; its near-tangency
//!   addendum appends a site note for exactly three carrier names at a
//!   segment pair, and nothing otherwise.

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
/// coincidence recourse still arrives ahead of it in the ending; and the
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
            "the shared recourse still rides the ending: {path}"
        );
        assert!(
            !path.contains("at this junction"),
            "no category over an unknown name: {path}"
        );
        assert!(
            path.starts_with("an unnamed decision is undecided: "),
            "the door names what escalated ahead of the payload: {path}"
        );
    }
}

/// **The validator's door ends by the decision's row and notes a
/// near-tangency.**
///
/// Every name ends in the geometry lever; only a name with no row names
/// the hole, and the site note is appended only at a segment pair for
/// the three carrier names.
#[test]
fn the_validator_door_ends_by_the_row_and_notes_a_near_tangency() {
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
            text.starts_with("between loop 0 segment 0 and loop 0 segment 1, "),
            "{text}"
        );
        assert!(text.contains(COINCIDENCE_RECOURSE), "{text}");
        assert_eq!(
            text.contains("no recourse is recorded"),
            name == UNKNOWN,
            "a gap sentence for the unknown name alone: {text}"
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
