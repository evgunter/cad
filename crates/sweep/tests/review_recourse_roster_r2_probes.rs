//! Review probes for the recourse roster (R2).
//!
//! **The validator's door ends by the gate and notes a near-tangency.**
//! `ProfileError::Escalated` ends each gate as its row in `profile`'s
//! closed `Gate` table says; its near-tangency addendum appends a site
//! note for exactly three carrier gates at a segment pair, and nothing
//! otherwise. An escalation under an unknown name cannot be built at
//! either profile door: each carries a `Gate`, as the blend door carries
//! a `BlendDecision`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, COINCIDENCE_RECOURSE, Indeterminate, MarginDiag, Tol};
use profile::{EscalationSite, Gate, ProfileError, SegmentRef};

/// An escalation of `gate`, its margin inside the run's band.
fn escalation(gate: Gate) -> Indeterminate {
    let band = Band::linear(Tol::witness()).expect("the run's band forms");
    Indeterminate {
        margin: MarginDiag::value((band.zero() + band.escalate()) / 2.0),
        band,
        predicate: Some(gate.predicate()),
        terminal_sliver: false,
    }
}

/// **The validator's door ends by the decision's row and notes a
/// near-tangency.**
///
/// Every gate ends in the geometry lever, and the site note is appended
/// only at a segment pair for the three carrier gates.
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
    for (gate, noted) in [
        (Gate::CarrierLineCircle, true),
        (Gate::CarrierCirclesExternal, true),
        (Gate::CarrierCirclesInternal, true),
        (Gate::CarrierCirclesIdentity, false),
        (Gate::PathJunctionTurn, false),
    ] {
        let text = ProfileError::Escalated {
            site: pair,
            gate,
            source: escalation(gate),
        }
        .to_string();
        assert!(
            text.starts_with("between loop 0 segment 0 and loop 0 segment 1, "),
            "{text}"
        );
        assert!(text.contains(COINCIDENCE_RECOURSE), "{text}");
        assert!(
            text.contains(gate.subject()),
            "the gate's own subject: {text}"
        );
        assert_eq!(text.contains(NOTE), noted, "{text}");
    }
    // Away from a segment pair the note never appears, even for a
    // carrier gate: the key is (site, gate), not the gate alone.
    let text = ProfileError::Escalated {
        site: EscalationSite::Loop { loop_index: 0 },
        gate: Gate::CarrierLineCircle,
        source: escalation(Gate::CarrierLineCircle),
    }
    .to_string();
    assert!(!text.contains(NOTE), "{text}");
}
