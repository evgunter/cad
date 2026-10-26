//! Rejection and escalation fixtures: every typed error variant hit by
//! a closed-form profile, with the exact error asserted.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{
    arc_kisses_line, bowtie, chain, circle_h, near_tangent_hole, profile, rect, tangent_hole, tol,
};
use geom_core::Point2;
use geom_core::Tol;
use geom_core::{ErrorTextReading, MarginDiag};

use profile::{
    ArcSweep, Center, ContactKind, EscalationSite, FILLET_ENCLOSING_RECOURSE, Open, PathError,
    ProfileError, SegmentRef, SketchPlane, Start,
};

fn err(p: &profile::Profile<f64>) -> ProfileError {
    p.validate(tol()).expect_err("fixture must be rejected")
}

fn sref(loop_index: usize, segment_index: usize) -> SegmentRef {
    SegmentRef {
        loop_index,
        segment_index,
    }
}

#[test]
fn empty_profile_is_rejected() {
    let p = profile::Profile::<f64>::new(SketchPlane::xy(), vec![]);
    assert_eq!(err(&p), ProfileError::EmptyProfile);
}

/// A plane that is not finite refuses at every validation door, and at
/// `lift_onto`, as `PlaneNotFinite`. Validation decides nothing on the
/// plane, but the canonical form carries it, so a NaN or infinite plane
/// would otherwise be a validated profile with no place in space.
#[test]
fn a_plane_that_is_not_finite_is_rejected() {
    let at = |z: f64| SketchPlane::xy_at(geom_core::Point3::new(0.0, 0.0, z));
    for z in [f64::NAN, f64::INFINITY] {
        let p = profile::Profile::new(at(z), vec![rect(0.0, 0.0, 1.0, 1.0)]);
        assert_eq!(err(&p), ProfileError::PlaneNotFinite, "validate, z = {z}");
        assert_eq!(
            p.validate_recording(tol()).map(|_| ()),
            Err(ProfileError::PlaneNotFinite),
            "validate_recording, z = {z}"
        );
        let finite = profile::Profile::new(at(0.0), vec![rect(0.0, 0.0, 1.0, 1.0)])
            .validate(tol())
            .expect("the finite plane validates");
        assert_eq!(
            finite.lift_onto(at(z)).map(|_| ()),
            Err(ProfileError::PlaneNotFinite),
            "lift_onto, z = {z}"
        );
    }
}

#[test]
fn empty_loop_fails_arity() {
    let p = profile(vec![chain(&[])]);
    assert_eq!(err(&p), ProfileError::EmptyLoop { loop_index: 0 });
}

/// One vertex is a legal arity (D1's full turn), but the bulge door
/// cannot express a full turn: its lowering of the zero chord has
/// radius zero, a segment of no length.
#[test]
fn single_bulge_vertex_is_a_degenerate_segment() {
    let p = profile(vec![chain(&[(0.0, 0.0, 1.0)])]);
    assert_eq!(err(&p), ProfileError::DegenerateSegment(sref(0, 0)));
}

#[test]
fn repeated_vertex_is_a_degenerate_segment() {
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
    ])]);
    assert_eq!(err(&p), ProfileError::DegenerateSegment(sref(0, 1)));
}

#[test]
fn near_coincident_vertices_escalate() {
    // Vertices 5ε apart: inside the sliver band — a typed escalation
    // naming the distance predicate, not a guess either way.
    let eps = tol().eps();
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, 5.0 * eps, 0.0),
        (0.0, 1.0, 0.0),
    ])]);
    match err(&p) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::Segment(sref(0, 1)));
            assert_eq!(source.predicate, Some("vertex_separation"));
        }
        other => panic!("expected escalation, got {other:?}"),
    }
}

#[test]
fn sliver_arc_bulge_escalates() {
    // A unit chord with sagitta 5ε: neither line nor sound arc.
    let eps = tol().eps();
    let p = profile(vec![chain(&[
        (0.0, 0.0, 10.0 * eps), // sagitta = L·b/2 = 5ε
        (1.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
        (0.0, 1.0, 0.0),
    ])]);
    match err(&p) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::Segment(sref(0, 0)));
            assert_eq!(source.predicate, Some("segment_straightness"));
        }
        other => panic!("expected escalation, got {other:?}"),
    }
}

/// **The re-homed fail-loud demo** (LIB-RETTAIL, Ev's ruling on
/// #413). The tour's coda used to build this loop and print the
/// refusal; a broken-on-purpose scene is not a use case, so the
/// contract moved here, where it can be asserted instead of narrated.
///
/// The contract, exactly as the demo stated it: the chain **authors
/// cleanly** — the PATHS lattice's junction checks are LOCAL, and all
/// four of the bowtie's corners are definitely sharp, so nothing
/// refuses at authoring time — and the **profile-level validator** is
/// what catches the crossing, typed, before any sweep can run. That
/// split is the whole point: a local check cannot see a global
/// self-intersection, which is why the validate gate exists.
///
/// The tour's own coordinates (the 2x2 scaling), so the geometry that
/// was demonstrated is the geometry that is pinned.
#[test]
fn the_bowtie_authors_cleanly_and_refuses_at_validation() {
    let authored = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(2.0, 2.0), Tol::witness())
        .and_then(|t| t.line_to(Point2::new(2.0, 0.0), Tol::witness()))
        .and_then(|t| t.line_to(Point2::new(0.0, 2.0), Tol::witness()))
        .and_then(|t| t.line_to(Start, Tol::witness()))
        .expect("every corner is sharp: the lattice's LOCAL checks pass a bowtie");

    let p = profile(vec![authored.into()]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 2),
            kind: ContactKind::Crossing,
        },
        "the validate gate must refuse the crossing typed — never Ok, never a panic"
    );
}

#[test]
fn bowtie_is_a_crossing() {
    let p = profile(vec![bowtie()]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 2),
            kind: ContactKind::Crossing,
        }
    );
}

#[test]
fn overlapping_loops_cross() {
    let p = profile(vec![rect(0.0, 0.0, 2.0, 2.0), rect(1.0, 1.0, 2.0, 2.0)]);
    match err(&p) {
        ProfileError::NonSimple {
            kind: ContactKind::Crossing,
            first,
            second,
        } => {
            assert_eq!(first.loop_index, 0);
            assert_eq!(second.loop_index, 1);
        }
        other => panic!("expected a crossing, got {other:?}"),
    }
}

#[test]
fn hole_crossing_outer_is_non_simple() {
    let p = profile(vec![rect(0.0, 0.0, 4.0, 4.0), rect(3.0, 1.0, 3.0, 1.0)]);
    match err(&p) {
        ProfileError::NonSimple {
            kind: ContactKind::Crossing,
            ..
        } => {}
        other => panic!("expected a crossing, got {other:?}"),
    }
}

#[test]
fn loops_touching_at_a_corner_are_non_simple() {
    let p = profile(vec![rect(0.0, 0.0, 2.0, 2.0), rect(2.0, 2.0, 1.0, 1.0)]);
    match err(&p) {
        ProfileError::NonSimple {
            kind: ContactKind::Touch,
            ..
        } => {}
        other => panic!("expected an endpoint contact, got {other:?}"),
    }
}

#[test]
fn vertex_on_segment_interior_is_non_simple() {
    // (1, 0) is a loop vertex lying on segment 0's interior.
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 0.0),
        (2.0, 2.0, 0.0),
        (1.0, 0.0, 0.0),
    ])]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 2),
            kind: ContactKind::Touch,
        }
    );
}

#[test]
fn collinear_spike_is_an_overlap() {
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
    ])]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 1),
            kind: ContactKind::Overlap,
        }
    );
}

#[test]
fn two_line_digon_is_an_overlap() {
    let p = profile(vec![chain(&[(0.0, 0.0, 0.0), (2.0, 0.0, 0.0)])]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 1),
            kind: ContactKind::Overlap,
        }
    );
}

#[test]
fn doubled_arc_digon_is_an_overlap() {
    // Out along an arc and back along the same arc: identical spans on
    // one carrier (coincident apexes).
    let p = profile(vec![chain(&[(0.0, 0.0, 1.0), (2.0, 0.0, -1.0)])]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 1),
            kind: ContactKind::Overlap,
        }
    );
}

#[test]
fn hole_outside_outer_means_two_outer_loops() {
    let p = profile(vec![rect(0.0, 0.0, 2.0, 2.0), rect(5.0, 0.0, 1.0, 1.0)]);
    assert_eq!(
        err(&p),
        ProfileError::MultipleOuterLoops {
            outer_loops: vec![0, 1],
        }
    );
}

#[test]
fn nesting_deeper_than_holes_is_rejected() {
    let p = profile(vec![
        rect(0.0, 0.0, 10.0, 10.0),
        rect(1.0, 1.0, 8.0, 8.0),
        rect(2.0, 2.0, 6.0, 6.0),
    ]);
    assert_eq!(
        err(&p),
        ProfileError::NestingTooDeep {
            loop_index: 2,
            depth: 2,
        }
    );
}

#[test]
fn internally_tangent_hole_is_a_tangential_contact() {
    assert_eq!(
        err(&tangent_hole()),
        ProfileError::TangentialContact {
            first: sref(0, 0),
            second: sref(1, 0),
        }
    );
}

#[test]
fn arc_kissing_a_line_is_a_tangential_contact() {
    assert_eq!(
        err(&arc_kisses_line()),
        ProfileError::TangentialContact {
            first: sref(0, 0),
            second: sref(0, 3),
        }
    );
}

/// The in-band height the line×line rows place an end at: half way
/// across the band, `ε·(1 + K)/2`.
fn line_band() -> f64 {
    let t = tol().get();
    t.eps * ((1.0 + t.k) / 2.0)
}

/// The step fixture of the line×line rows:
/// `(0,0) (2,0) (2,2) (4,2) (4,b) (6,−1) (6,3) (0,3)`, with `(4, b)` in
/// band of segment 0's carrier `y = 0` and two metres past its end.
fn step_chain() -> Vec<(f64, f64, f64)> {
    let b = line_band();
    vec![
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 0.0),
        (2.0, 2.0, 0.0),
        (4.0, 2.0, 0.0),
        (4.0, b, 0.0),
        (6.0, -1.0, 0.0),
        (6.0, 3.0, 0.0),
        (0.0, 3.0, 0.0),
    ]
}

/// **A segment's end in band of another's carrier but past its end
/// along it is no contact, whichever way the loop is authored** (the
/// line×line row of
/// `profile/validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc`).
/// The step fixture validates as authored; re-seamed to start at
/// `(4, 2)`, where the in-band pair is read with the vertical segment
/// first and its own axis puts the gap at `b`, in band; and reversed. In
/// each, `(0,0)` and `(2,0)` lie definitely on one side of the vertical
/// segment's carrier, so the pair is apart however `(4, b)`'s side of
/// `y = 0` reads. At `f64` and `Interval` alike.
#[test]
fn an_end_in_band_of_a_carrier_but_past_its_segment_is_no_contact() {
    let as_authored = step_chain();
    let mut reseamed = as_authored.clone();
    reseamed.rotate_left(3);
    let mut reversed = as_authored.clone();
    reversed.reverse();
    for (name, vs) in [
        ("as authored", as_authored),
        ("re-seamed", reseamed),
        ("reversed", reversed),
    ] {
        validate_both(&profile(vec![chain(&vs)]))
            .unwrap_or_else(|e| panic!("{name}: the end two metres past segment 0: {e:?}"));
    }
}

/// **An end in band of a carrier past its segment, its segment's
/// projection overlapping, is no contact.** The chain
/// `(0,0) (1,0) (1,−1) (3,−1) (2,b) (0.5,1) (−1,1)`: `(2, b)` is in band
/// of segment 0's carrier, a metre past its end, and segment 4 runs back
/// over segment 0 at least `0.55` m above it, so their projections
/// overlap on both axes and no gap separates them. Segment 0's ends lie
/// definitely on one side of segment 4's carrier, which does. At `f64`
/// and `Interval` alike.
#[test]
fn an_end_in_band_of_a_carrier_over_an_overlapping_projection_is_no_contact() {
    let b = line_band();
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, -1.0, 0.0),
        (3.0, -1.0, 0.0),
        (2.0, b, 0.0),
        (0.5, 1.0, 0.0),
        (-1.0, 1.0, 0.0),
    ])]);
    validate_both(&p).expect("segment 0 lies wholly below segment 4's carrier");
}

/// **Two segments in band of one line but a metre apart along it are no
/// contact.** The chain `(0,0) (1,0) (1,−1) (4,−1) (4,b) (2,b) (2,2) (0,2)`:
/// segment 0 runs along `y = 0` to `x = 1`, and segment 4 along `y = b`
/// from `x = 4` back to `x = 2`. Every end of each stands in band of the
/// other's carrier, so no carrier side is definite and nothing but the
/// spans' gap along the line, a metre, separates them. At `f64` and
/// `Interval` alike.
#[test]
fn two_segments_in_band_of_one_line_a_metre_apart_along_it_are_no_contact() {
    let b = line_band();
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, -1.0, 0.0),
        (4.0, -1.0, 0.0),
        (4.0, b, 0.0),
        (2.0, b, 0.0),
        (2.0, 2.0, 0.0),
        (0.0, 2.0, 0.0),
    ])]);
    validate_both(&p).expect("the two segments lie a metre apart along their line");
}

/// The NOTE witness of PR 5536's review: a run along `y = 0.9ε` from
/// `(−2, 0.9ε)`, its last head at `(−0.5, last)`, closed onto `Start`
/// tangent, so the closing leg and segment 0 meet at `Start` through a
/// joint straight within the band.
fn straight_joint_at_start(last: f64) -> profile::Profile<f64> {
    let t = tol();
    let e = t.get().eps;
    let closed = Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .line_to(Point2::new(0.0, 2.0), t)
        .unwrap()
        .line_to(Point2::new(-2.0, 0.9 * e), t)
        .unwrap()
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Point2::new(-0.5, last * e), t)
        .unwrap()
        .continue_to(Start.arrives_tangent(), t)
        .unwrap();
    profile(vec![common::pinned(closed)])
}

/// **A run's end in band of a carrier a metre past its segment is no
/// contact**: [`straight_joint_at_start`] with the last head on the
/// run's line (`0.9ε`). `(1, 0)`, segment 1's start, stands in band of
/// the closing leg's carrier and a metre past its span, and the closing
/// leg lies within the band of segment 0's carrier, so every pair of the
/// simplicity pass reads no contact, `SegmentPair(1, 5)` and
/// `SegmentPair(0, 5)` among them. What escalates is the joint at
/// `Start`, read after the whole pass (`SegmentPair(5, 0)`, the joint's
/// arriving and leaving segments): a joint straight within the band.
#[test]
fn a_run_end_in_band_of_a_carrier_past_its_segment_is_no_contact() {
    match validate_both(&straight_joint_at_start(0.9)) {
        Err(ProfileError::Escalated { site, source }) => {
            assert_eq!(site, EscalationSite::SegmentPair(sref(0, 5), sref(0, 0)));
            assert_eq!(source.predicate, Some("chord_side"));
        }
        other => panic!("only the joint at Start may escalate, got {other:?}"),
    }
}

/// A short segment continuing a long one straight within ε: the loop
/// `(0,0) (1,0) (x, y) (x,1) (0,1)`, segment 1 from `(1, 0)` to `(x, y)`
/// with `y` within the band's zero of `y = 0`.
fn straight_within_eps(x: f64, y: f64) -> profile::Profile<f64> {
    profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (x, y, 0.0),
        (x, 1.0, 0.0),
        (0.0, 1.0, 0.0),
    ])])
}

/// **A short segment continuing a long one straight within ε is
/// simple.** [`straight_within_eps`] at `(1.1, 0.5ε)`, `(1.2, 0.5ε)` and
/// `(1.3, 0.9ε)`, and the first re-seamed to start at `(1, 0)`, so the
/// short segment is read first. Both of the short segment's ends lie
/// within the band's zero of the long one's carrier, so the pair is read
/// collinear along the long one, a touch at their shared vertex, though
/// the long one's far end stands `len₀/len₁` times as far off the short
/// one's carrier, in band. At `f64` and `Interval` alike.
#[test]
fn a_short_segment_continuing_a_long_one_straight_within_eps_is_simple() {
    let e = tol().get().eps;
    for (x, y) in [(1.1, 0.5 * e), (1.2, 0.5 * e), (1.3, 0.9 * e)] {
        validate_both(&straight_within_eps(x, y))
            .unwrap_or_else(|err| panic!("({x}, {y}): {err:?}"));
    }
    let short_first = profile(vec![chain(&[
        (1.0, 0.0, 0.0),
        (1.1, 0.5 * e, 0.0),
        (1.1, 1.0, 0.0),
        (0.0, 1.0, 0.0),
        (0.0, 0.0, 0.0),
    ])]);
    validate_both(&short_first).unwrap_or_else(|err| panic!("the short segment first: {err:?}"));
}

/// **A loop that comes back to a vertex along a straight continuation
/// touches itself there.** `(0,0) (1,0) (1,−1) (1.2,−1) (1.2, 0.5ε) (1,0)`:
/// segment 4 returns to `(1, 0)` within the band's zero of segment 0's
/// carrier, and the closing leg retraces segment 0 back to `(0, 0)`.
/// Read collinear with segment 0, segment 4 touches it at `(1, 0)`,
/// which is not their shared vertex, so the loop is not simple:
/// `NonSimple` with a `Touch`, not an escalation.
#[test]
fn a_loop_returning_to_a_vertex_along_a_straight_continuation_touches() {
    let e = tol().get().eps;
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, -1.0, 0.0),
        (1.2, -1.0, 0.0),
        (1.2, 0.5 * e, 0.0),
        (1.0, 0.0, 0.0),
    ])]);
    match err(&p) {
        ProfileError::NonSimple {
            kind: ContactKind::Touch,
            ..
        } => {}
        other => panic!("the return to (1, 0) must read a touch, got {other:?}"),
    }
}

/// **An adjacent pair through a joint straight within the band touches
/// only at the vertex it shares.** [`straight_joint_at_start`] as the
/// review wrote it (`1.8ε`): segment 0 and the closing leg share
/// `Start`, their far ends stand in band of each other's carrier, and
/// their projections touch at the shared vertex, so neither the gap nor a
/// carrier side decides the pair, and `line_line` escalates on
/// `chord_side`. Their overlap along segment 0 is within the band, so
/// every point they share is within the band of `Start`, which the
/// adjacency discount reads, and the pair pass reads no contact. What
/// escalates is the joint at `Start` (`SegmentPair(5, 0)`), straight
/// within the band, as at `0.9ε`
/// ([`a_run_end_in_band_of_a_carrier_past_its_segment_is_no_contact`]).
#[test]
fn an_adjacent_pair_through_a_joint_straight_within_the_band_escalates_only_at_its_joint() {
    match validate_both(&straight_joint_at_start(1.8)) {
        Err(ProfileError::Escalated { site, source }) => {
            assert_eq!(site, EscalationSite::SegmentPair(sref(0, 5), sref(0, 0)));
            assert_eq!(source.predicate, Some("chord_side"));
        }
        other => panic!("only the joint at Start may escalate, got {other:?}"),
    }
}

/// A short closing leg arriving at `(0, 0)` from `(−0.1, h)`: the loop
/// `(0,0) (1,0) (1,1) (−0.1,1) (−0.1,h)`. With `h` in band, the closing
/// leg's far end stands in band of segment 0's carrier while `(1, 0)`
/// stands `10·h` off the leg's, definitely, so the joint at `(0, 0)` is a
/// corner and the pair's sides decide nothing.
fn short_leg_into_a_corner(h: f64) -> profile::Profile<f64> {
    profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
        (-0.1, 1.0, 0.0),
        (-0.1, h, 0.0),
    ])])
}

/// **An adjacent pair whose side reads in band escalates at its joint,
/// not as a pair.** [`short_leg_into_a_corner`] at `in_band()`, and
/// re-seamed to start at the leg's far end, so the short leg is the
/// pair's first segment and the overlap is read along it. On main the
/// pair escalated on `chord_side`. Its overlap is within the band, so the
/// pair pass reads no contact, and the joint at `(0, 0)` escalates on
/// `path_junction_turn`: the turn levered by the shorter leg is the same
/// in-band displacement the leg's far end stands off segment 0's carrier.
#[test]
fn an_adjacent_pair_with_an_in_band_side_escalates_at_its_joint() {
    let h = in_band();
    let short_first = profile(vec![chain(&[
        (-0.1, h, 0.0),
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
        (-0.1, 1.0, 0.0),
    ])]);
    for (name, p, joint) in [
        ("the corner first", short_leg_into_a_corner(h), (4, 0)),
        ("the short leg first", short_first, (0, 1)),
    ] {
        match validate_both(&p) {
            Err(ProfileError::Escalated { site, source }) => {
                assert_eq!(
                    site,
                    EscalationSite::SegmentPair(sref(0, joint.0), sref(0, joint.1)),
                    "{name}"
                );
                assert_eq!(source.predicate, Some("path_junction_turn"), "{name}");
            }
            other => panic!("{name}: only the joint may escalate, got {other:?}"),
        }
    }
}

/// **A pair that shares no vertex still escalates on an in-band side.**
/// The square `(0,0) (1,0) (1,−1) (0,−1)` and a triangle whose first edge
/// starts half the band's zero past the square's `(1, 0)` and runs on to
/// `(3, h)`, `h` in band: the two edges' projections touch within the
/// band, and their sides read in band. The shared-end touch is the
/// adjacent pair's, read at a vertex both segments store, so this pair,
/// across two loops, escalates as before.
#[test]
fn a_pair_sharing_no_vertex_still_escalates_on_an_in_band_side() {
    let t = tol().get();
    let p = profile(vec![
        chain(&[
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (1.0, -1.0, 0.0),
            (0.0, -1.0, 0.0),
        ]),
        chain(&[
            (1.0 + 0.5 * t.eps, 0.0, 0.0),
            (3.0, in_band(), 0.0),
            (2.0, 1.0, 0.0),
        ]),
    ]);
    match err(&p) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::SegmentPair(sref(0, 0), sref(1, 0)));
            assert_eq!(source.predicate, Some("chord_side"));
        }
        other => panic!("the pair across two loops must escalate, got {other:?}"),
    }
}

/// **A crossing the pass reads after an adjacent pair's in-band side is
/// the refusal.** `(0,0) (1,0) (1,1) (1.2,0.5) (−0.1,h)`, `h` in band:
/// the closing leg and segment 0 meet only at `(0, 0)`, read before the
/// crossing of segments 1 and 3 in pass order. On main the pair
/// escalated on `chord_side` and the crossing went unread.
#[test]
fn a_crossing_after_an_adjacent_pairs_in_band_side_refuses() {
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
        (1.2, 0.5, 0.0),
        (-0.1, in_band(), 0.0),
    ])]);
    match err(&p) {
        ProfileError::NonSimple {
            first,
            second,
            kind: ContactKind::Crossing,
        } => assert_eq!((first, second), (sref(0, 1), sref(0, 3))),
        other => panic!("segments 1 and 3 cross, got {other:?}"),
    }
}

/// One circle, its lowest point an in-band height above the bottom
/// line, carrying either of two arcs between `(3, 3)` and `(1, 3)`:
/// the major arc dips through that point and escalates on the carrier
/// clearance; the minor arc bulges up, away from it, and is 3 clear of
/// the line, so the same carrier clearance is no contact of the
/// segments' and the loop validates.
#[test]
fn an_in_band_line_circle_clearance_escalates_only_where_the_arc_holds_the_graze() {
    let t = tol().get();
    let band = t.eps * ((1.0 + t.k) / 2.0);
    // Half-chord 1, so the bulge is the sagitta: the major arc's
    // sagitta 3 − band puts the carrier's lowest point at y = band, and
    // its complement on the same carrier has bulge 1/(3 − band).
    let sagging = 3.0 - band;
    let with_arc = |bulge: f64| {
        profile(vec![chain(&[
            (0.0, 0.0, 0.0),
            (4.0, 0.0, 0.0),
            (4.0, 3.0, 0.0),
            (3.0, 3.0, bulge),
            (1.0, 3.0, 0.0),
            (0.0, 3.0, 0.0),
        ])])
    };
    match err(&with_arc(-sagging)) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::SegmentPair(sref(0, 0), sref(0, 3)));
            assert_eq!(source.predicate, Some("carrier_line_circle"));
        }
        other => panic!("the arc through the graze must escalate, got {other:?}"),
    }
    with_arc(1.0 / sagging)
        .validate(tol())
        .expect("the arc off the graze is clear of the line");
}

/// The in-band height the rows below place a carrier clearance at:
/// half way across the band, `ε·(1 + K)/2`.
fn in_band() -> f64 {
    let t = tol().get();
    t.eps * ((1.0 + t.k) / 2.0)
}

/// **A line that ends short of an in-band graze is no contact** (the
/// line×circle row of
/// `profile/validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc`).
/// The major arc from `(3, 3)` to `(1, 3)` dips to an in-band height
/// above `y = 0` at `x = 2`; the line along `y = 0` ends at `x = 1`,
/// about `0.28` m from it. On main the pair pass escalated
/// `SegmentPair(0, 6)` on `carrier_line_circle`: the arc holds the
/// graze, so the arc's side cannot clear it. The line's side does: the
/// centre's foot `(2, 0)` is past the line's end, and both its ends are
/// outside the circle. The loop validates, at `f64` and `Interval`.
#[test]
fn a_line_ending_short_of_an_in_band_graze_is_no_contact() {
    let b = in_band();
    let p = profile(vec![chain(&[
        (-1.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, -1.0, 0.0),
        (5.0, -1.0, 0.0),
        (5.0, 5.0, 0.0),
        (3.0, 5.0, 0.0),
        (3.0, 3.0, -(3.0 - b)),
        (1.0, 3.0, 0.0),
        (-1.0, 3.0, 0.0),
    ])]);
    validate_both(&p).expect("the line ends a metre short of the graze");
}

/// Two half-disc holes in a square, whose circles, of radius 1 about
/// `(0, 0)` and `(2 + gap, 0)`, are externally tangent within the band
/// at `(1, 0)`: `facing` puts each arc on the side toward the other
/// (through the near-tangency), otherwise each is on its far side.
fn half_discs_outside(gap: f64, facing: bool) -> profile::Profile<f64> {
    let x = 2.0 + gap;
    let bulge = if facing { -1.0 } else { 1.0 };
    profile(vec![
        rect(-5.0, -5.0, 10.0, 10.0),
        chain(&[(0.0, 1.0, bulge), (0.0, -1.0, 0.0)]),
        chain(&[(x, -1.0, 0.0), (x, 1.0, -bulge)]),
    ])
}

/// **Arcs on the far sides of an in-band external tangency are no
/// contact** (the arc×arc row of the item above). The circles' external
/// clearance is in band, and on main the pass escalated
/// `SegmentPair((0, 0), (1, 1))` on `carrier_circles_external`, though
/// the arcs stand two metres apart. Each arc excludes its point facing
/// the other centre, and its ends lie outside the other circle, so the
/// pair validates, at `f64` and `Interval`.
#[test]
fn arcs_away_from_an_in_band_external_tangency_are_no_contact() {
    validate_both(&half_discs_outside(in_band(), false)).expect("the arcs stand two metres apart");
}

/// **One arc away from an in-band external tangency is enough.** As
/// [`arcs_away_from_an_in_band_external_tangency_are_no_contact`], but
/// the second hole's arc faces the near-tangency at `(1, 0)`, so only the
/// first arc is clear of the other's circle, and that suffices: the pair
/// validates, where reading only the second arc's certificate would
/// escalate `SegmentPair((1, 0), (2, 1))` (the FULL review's witness,
/// PR 5552).
#[test]
fn one_arc_away_from_an_in_band_external_tangency_is_no_contact() {
    let x = 2.0 + in_band();
    let p = profile(vec![
        rect(-5.0, -5.0, 10.0, 10.0),
        chain(&[(0.0, 1.0, 1.0), (0.0, -1.0, 0.0)]),
        chain(&[(x, -1.0, 0.0), (x, 1.0, 1.0)]),
    ]);
    validate_both(&p).expect("the first arc stands a metre and more from the second circle");
}

/// **Arcs that face an in-band external tangency still escalate.** The
/// control for the row above: each arc holds the near-tangency at
/// `(1, 0)`, so neither is clear of the other's circle, and the
/// clearance's in-band reading is the pair's verdict.
#[test]
fn arcs_facing_an_in_band_external_tangency_escalate() {
    match err(&half_discs_outside(in_band(), true)) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::SegmentPair(sref(1, 0), sref(2, 1)));
            assert_eq!(source.predicate, Some("carrier_circles_external"));
        }
        other => panic!("the facing arcs must escalate, got {other:?}"),
    }
}

/// A circle of radius 1 about `(1 + gap, 0)` inside one of radius 2
/// about the origin, internally tangent within the band at `(2, 0)`.
/// The small one's half-disc, its arc through `(gap, 0)` away from the
/// tangency, as a hole: in the big circle's segment right of `x = −1`
/// (`outer_holds`, its arc through the tangency), or in a square beside
/// the big circle's cap left of `x = −1` (its arc through `(−2, 0)`).
fn half_discs_inside(gap: f64, outer_holds: bool) -> profile::Profile<f64> {
    let x = 1.0 + gap;
    let hole = chain(&[(x, 1.0, 1.0), (x, -1.0, 0.0)]);
    let h = 3f64.sqrt();
    if outer_holds {
        // Sagitta 3 on half-chord √3.
        profile(vec![chain(&[(-1.0, -h, h), (-1.0, h, 0.0)]), hole])
    } else {
        // Sagitta 1 on half-chord √3.
        let cap = chain(&[(-1.0, h, 1.0 / h), (-1.0, -h, 0.0)]);
        profile(vec![rect(-5.0, -5.0, 10.0, 10.0), cap, hole])
    }
}

/// **Arcs away from an in-band internal tangency are no contact** (the
/// item's arc×arc row, its internal sibling). The internal clearance is
/// in band, and on main the pass escalated on
/// `carrier_circles_internal`. Where the big arc holds the tangency,
/// the small arc clears it from inside: it excludes its point farthest
/// from the big centre, and its ends lie inside the big circle. Where
/// neither holds it, the big arc clears it from outside. Both validate,
/// at `f64` and `Interval`.
#[test]
fn arcs_away_from_an_in_band_internal_tangency_are_no_contact() {
    for outer_holds in [true, false] {
        validate_both(&half_discs_inside(in_band(), outer_holds)).unwrap_or_else(|e| {
            panic!(
                "the small arc keeps a metre off the tangency (outer_holds {outer_holds}): {e:?}"
            )
        });
    }
}

/// Two holes in a square, one touching the other at a vertex that both
/// of its segments leave nearly tangent to the other's edge. Hole 1
/// runs down the unit circle centred (0, 1) to `E = (−s, s²/2)`, which
/// stands within ε of the line y = 0, `s = 3Kε` short of the circle's
/// tangency point with it; it leaves `E` on the circle of radius ½
/// tangent to y = 0 a further `s/√2` to the left, so each of its arcs
/// misses its own tangency point with y = 0 by more than Kε. Its last
/// side closes along y = 1, tangent to the small circle at its top.
/// Hole 2 lies under y = 0: the rectangle below it, or (`under = Some(R)`)
/// a region capped by an arc of the circle of radius `R` tangent to
/// y = 0 from below at `x = −3.5Kε`, between the two circles' own
/// tangency points with it, where each arc of hole 1 definitely misses
/// its tangency with the cap's circle.
fn holes_touching_between_tangent_arcs(under: Option<f64>) -> profile::Profile<f64> {
    use geom_core::Arc2;
    use profile::{ProfileLoop, RawLoop, Segment};
    let t = tol().get();
    let s = 3.0 * t.k * t.eps;
    let touch = Point2::new(-s, s * s / 2.0);
    let small = Point2::new(-s - (touch.y - touch.y * touch.y).sqrt(), 0.5);
    let top = Point2::new(small.x, 1.0);
    let angle = |c: Point2<f64>, p: Point2<f64>| (p.y - c.y).atan2(p.x - c.x);
    let big = Point2::new(0.0, 1.0);
    let down = (angle(big, touch) - std::f64::consts::PI).rem_euclid(std::f64::consts::TAU);
    let up = (angle(small, top) - angle(small, touch)).rem_euclid(std::f64::consts::TAU);
    let arc = |centre, radius, sweep| {
        Segment::Arc(Arc2 {
            centre,
            radius,
            sweep,
        })
    };
    let hole = ProfileLoop::new([
        (Point2::new(-1.0, 1.0), arc(big, 1.0, down)),
        (touch, arc(small, 0.5, up)),
        (top, Segment::Line),
    ]);
    let under = match under {
        None => rect(-1.0, -1.0, 2.0, 1.0),
        Some(r) => {
            let x = -3.5 * t.k * t.eps;
            let y = (r * r - 1.0).sqrt() - r;
            let cap = arc(Point2::new(x, -r), r, 2.0 * (1.0 / r).asin());
            ProfileLoop::new([
                (Point2::new(x + 1.0, y), cap),
                (Point2::new(x - 1.0, y), Segment::Line),
                (Point2::new(x - 1.0, -3.0), Segment::Line),
                (Point2::new(x + 1.0, -3.0), Segment::Line),
            ])
        }
    };
    profile(vec![rect(-5.0, -5.0, 10.0, 10.0), hole, under])
}

/// **A touch between two holes, at a vertex whose two segments each
/// miss their tangency with the other hole's edge, is refused** at
/// `f64` and at `Interval`, against a line (the rectangle's top) and
/// against an arc (a cap tangent to y = 0 from below). The touching
/// vertex stands within 4.5·K²ε² of the other edge. The cap is moved
/// off the origin because one tangent there escalates instead: its
/// tangency with the small circle falls within Kε of `E`, where the
/// small arc's span reads it in band.
#[test]
fn holes_touching_at_a_vertex_between_two_tangent_arcs_are_non_simple() {
    for (under, edge) in [(None, 2), (Some(2.0), 0)] {
        let p = holes_touching_between_tangent_arcs(under);
        let want = ProfileError::NonSimple {
            first: sref(1, 0),
            second: sref(2, edge),
            kind: ContactKind::Touch,
        };
        assert_eq!(err(&p), want, "f64, under {under:?}");
        assert_eq!(
            common::lift::<geom_core::Interval>(&p)
                .validate(tol())
                .map(|_| ())
                .expect_err("the Interval profile must be rejected"),
            want,
            "Interval, under {under:?}"
        );
    }
}

/// Two unit circles crossing at the origin at the angle `phi`, both
/// heading +x there clockwise: a dome whose left leg runs down the
/// first to `end` (arc length) short of the crossing, a connector to
/// the point `end` past it on the second, the right leg down the
/// second, and a chord closing underneath. The legs are non-adjacent
/// and stand ≈ 2·`end` apart across the crossing. It is `seg`'s
/// `in_band_rows` crossing read as a whole profile; a `tests/` binary
/// cannot name that module's builders.
fn dome_across_a_shallow_crossing(phi: f64, end: f64) -> profile::Profile<f64> {
    use geom_core::Arc2;
    use profile::{ProfileLoop, RawLoop, Segment};
    let quarter = std::f64::consts::FRAC_PI_2;
    let first = Point2::new(0.0, -1.0);
    let second = Point2::new(phi.sin(), -phi.cos());
    let on = |c: Point2<f64>, angle: f64| Point2::new(c.x + angle.cos(), c.y + angle.sin());
    let arc = |centre, sweep| {
        Segment::Arc(Arc2 {
            centre,
            radius: 1.0,
            sweep,
        })
    };
    profile(vec![ProfileLoop::new([
        (on(first, 2.0 * quarter), arc(first, -(quarter - end))),
        (on(first, quarter + end), Segment::Line),
        (
            on(second, quarter + phi - end),
            arc(second, -(quarter - end)),
        ),
        (on(second, phi), Segment::Line),
    ])])
}

/// **Two legs whose carriers cross at a shallow angle, stopping short
/// of the crossing on opposite sides, validate** at `f64` and at
/// `Interval`, at every ε. The legs stand 1.6Kε apart; each span reads
/// the crossing in band, 0.8Kε past its end, and each leg's end stands
/// within ε of the other's carrier (the turn is 0.03 rad), so the ends
/// settle the pair. This is the arc × arc fillet corner at a small
/// turn, its fillet replaced by its chord.
#[test]
fn legs_stopping_short_either_side_of_a_shallow_crossing_validate() {
    let t = tol().get();
    let p = dome_across_a_shallow_crossing(0.03, 0.8 * t.k * t.eps);
    p.validate(tol())
        .unwrap_or_else(|e| panic!("f64: the dome validates, got {e}"));
    common::lift::<geom_core::Interval>(&p)
        .validate(tol())
        .unwrap_or_else(|e| panic!("Interval: the dome validates, got {e}"));
}

/// A figure-eight: a near-full arc (2π − 0.011 rad) and a small arc
/// whose carriers cross twice, both ends of the near-full arc lying
/// within ε of the small arc's carrier beside the crossing the arcs do
/// not hold, while the arcs themselves cross at the other, 2.7e-3
/// inside the large arc and 1.3e-3 inside the small one, where the
/// large arc's compressed `arc_span` reads it in band. Lines join the
/// ends: E1₀ → arc1 → E1₁ → E2₁ → arc2 reversed → E2₀ → W → E1₀. The
/// figure is drawn for ε = 1e-6 and scaled with ε, so every row reads
/// the same figure.
fn figure_eight_across_the_other_crossing(scale: f64) -> profile::Profile<f64> {
    use geom_core::Arc2;
    use profile::{ProfileLoop, RawLoop, Segment};
    let pt = |x: f64, y: f64| Point2::new(x * scale, y * scale);
    let (c1, r1) = (
        pt(-1.872513274081899, 2.577825301309983),
        1.6092706295113095,
    );
    let (start1, sweep1) = (-1.0062495826666513, -6.272141383775728);
    let (c2, r2) = (
        pt(-0.970460427436857, 1.1507809548491568),
        0.07898190310093273,
    );
    let (start2, sweep2) = (2.1355617127241118, 3.9727669712056928);
    let on = |c: Point2<f64>, r: f64, angle: f64| {
        Point2::new(c.x + r * scale * angle.cos(), c.y + r * scale * angle.sin())
    };
    let arc = |centre, radius: f64, sweep| {
        Segment::Arc(Arc2 {
            centre,
            radius: radius * scale,
            sweep,
        })
    };
    let (e10, e11) = (on(c1, r1, start1), on(c1, r1, start1 + sweep1));
    let (e20, e21) = (on(c2, r2, start2), on(c2, r2, start2 + sweep2));
    let w = Point2::new(0.5 * (e20.x + e10.x) - 1e-3 * scale, 0.5 * (e20.y + e10.y));
    profile(vec![ProfileLoop::new([
        (e10, arc(c1, r1, sweep1)),
        (e11, Segment::Line),
        (e21, arc(c2, r2, -sweep2)),
        (e20, Segment::Line),
        (w, Segment::Line),
    ])])
}

/// **A figure-eight whose crossing a near-full arc reads in band does
/// not validate**, at `f64` and at `Interval`, at every ε. The large
/// arc's end nearest the crossing lies within ε of the small arc's
/// carrier, but beside the carriers' other crossing, so it stands in
/// that crossing's stretch, not this one's, and cannot settle it.
#[test]
fn a_crossing_read_in_band_is_not_settled_by_an_end_beside_the_other_crossing() {
    let p = figure_eight_across_the_other_crossing(tol().eps() / 1e-6);
    let f64_read = p.validate(tol()).map(|_| ());
    let interval_read = common::lift::<geom_core::Interval>(&p)
        .validate(tol())
        .map(|_| ());
    println!("f64: {f64_read:?}\nInterval: {interval_read:?}");
    assert!(f64_read.is_err(), "f64: the figure-eight must not validate");
    assert!(
        interval_read.is_err(),
        "Interval: the figure-eight must not validate"
    );
}

/// The unit circle about the origin and the circle about `(d, 0)` of
/// radius `1 − d + g`: carriers `d` apart whose internal clearance is
/// `g`, so within the band of tangency at angle 0 for `|g| ≤ ε`, and
/// apart by `d(1 − cos θ) − g` at angle `θ`.
fn near_concentric(d: f64, g: f64) -> ((Point2<f64>, f64), (Point2<f64>, f64)) {
    (
        (Point2::new(0.0, 0.0), 1.0),
        (Point2::new(d, 0.0), 1.0 - d + g),
    )
}

fn rim((c, r): (Point2<f64>, f64), angle: f64) -> Point2<f64> {
    Point2::new(c.x + r * angle.cos(), c.y + r * angle.sin())
}

/// A hole bounded by the arc of `circle` from angle `from` through
/// `sweep` and its chord.
fn cap_hole(circle: (Point2<f64>, f64), from: f64, sweep: f64) -> profile::ProfileLoop<f64> {
    use geom_core::Arc2;
    use profile::{ProfileLoop, RawLoop, Segment};
    let arc = Segment::Arc(Arc2 {
        centre: circle.0,
        radius: circle.1,
        sweep,
    });
    ProfileLoop::new([
        (rim(circle, from), arc),
        (rim(circle, from + sweep), Segment::Line),
    ])
}

/// A hole outside `circle`: its arc clockwise from angle `hi` to `lo`,
/// closed by lines out to x = 3.
fn outside_hole(circle: (Point2<f64>, f64), hi: f64, lo: f64) -> profile::ProfileLoop<f64> {
    use geom_core::Arc2;
    use profile::{ProfileLoop, RawLoop, Segment};
    let arc = Segment::Arc(Arc2 {
        centre: circle.0,
        radius: circle.1,
        sweep: lo - hi,
    });
    let (top, bottom) = (rim(circle, hi), rim(circle, lo));
    ProfileLoop::new([
        (top, arc),
        (bottom, Segment::Line),
        (Point2::new(3.0, bottom.y), Segment::Line),
        (Point2::new(3.0, top.y), Segment::Line),
    ])
}

/// Validates `p` at `f64` and at `Interval`, which must agree up to an
/// escalation's margin.
fn validate_both(p: &profile::Profile<f64>) -> Result<(), ProfileError> {
    let verdict = |read: &Result<(), ProfileError>| match read {
        Err(ProfileError::Escalated { site, source }) => format!("{site:?} {:?}", source.predicate),
        other => format!("{other:?}"),
    };
    let at_f64 = p.validate(tol()).map(|_| ());
    let at_interval = common::lift::<geom_core::Interval>(p)
        .validate(tol())
        .map(|_| ());
    assert_eq!(
        verdict(&at_f64),
        verdict(&at_interval),
        "f64 and Interval disagree"
    );
    at_f64
}

/// **Two arcs on near-concentric carriers decided internally tangent
/// read their tangency on each carrier, not at the radical foot.**
///
/// With centres `d` apart and internal clearance `g`, the radical foot
/// stands about `g/d` off both unit-scale circles: 0.029 at `d = 17ε`,
/// `g = 0.5ε`, at every ε. Each hole's arc stops `s` short of angle 0,
/// so the arcs are `2s` apart, centimetres at most, and none of them
/// may refuse. The second table puts the arcs within ε of each other
/// along the stretch where the carriers stay within the band, which
/// reaches far from angle 0 when `d` is a few ε: there, a hole inside
/// the unit circle and a hole outside the other circle, with the arcs
/// holding angle 0 or not, must refuse: a tangency, an end's touch, or
/// an end read in band of the other carrier or the other span. Where the
/// carriers cross (`g > 0`) they read as two crossings 0.24 rad either
/// side of angle 0, not as one tangency.
#[test]
fn near_concentric_arcs_read_their_tangency_on_each_carrier() {
    let eps = tol().eps();
    for (d, g, s) in [
        (17.0, 0.5, 0.005),
        (17.0, 0.5, 0.01),
        (17.0, 0.5, 0.014),
        (34.0, 0.9, 0.01),
        (17.0, -0.5, 0.01),
        (1000.0, 0.5, 0.001),
    ] {
        let (one, two) = near_concentric(d * eps, g * eps);
        let p = profile(vec![
            rect(-5.0, -5.0, 10.0, 10.0),
            cap_hole(one, s, 2.0),
            cap_hole(two, -2.0 - s, 2.0),
        ]);
        assert_eq!(
            validate_both(&p),
            Ok(()),
            "arcs {:e} apart, d = {d}ε, g = {g}ε",
            2.0 * s
        );
    }
    let pair = (sref(1, 0), sref(2, 0));
    let tangent = || ProfileError::TangentialContact {
        first: pair.0,
        second: pair.1,
    };
    let touch = || ProfileError::NonSimple {
        first: pair.0,
        second: pair.1,
        kind: ContactKind::Touch,
    };
    let crossing = || ProfileError::NonSimple {
        first: pair.0,
        second: pair.1,
        kind: ContactKind::Crossing,
    };
    for (d, g, (from, sweep), (hi, lo), want) in [
        (17.0, 0.5, (-0.2, 0.4), (0.2, -0.2), Err("arc_span")),
        (17.0, -0.5, (-0.2, 0.4), (0.2, -0.2), Ok(tangent())),
        (17.0, 0.5, (0.1, 0.4), (0.3, 0.05), Ok(crossing())),
        (17.0, 0.5, (-0.3, 0.2), (0.2, -0.2), Ok(touch())),
        (17.0, 0.5, (-0.2, 0.4), (0.3, 0.1), Ok(touch())),
        (17.0, -0.5, (0.1, 0.4), (0.3, 0.05), Err("circle_side")),
        (17.0, -0.5, (-0.3, 0.2), (-0.05, -0.25), Err("circle_side")),
        (34.0, 0.9, (0.05, 0.1), (0.12, 0.08), Ok(touch())),
    ] {
        let (one, two) = near_concentric(d * eps, g * eps);
        let p = profile(vec![
            rect(-5.0, -5.0, 10.0, 10.0),
            cap_hole(one, from, sweep),
            outside_hole(two, hi, lo),
        ]);
        let label = format!("d = {d}ε, g = {g}ε, arc {from}+{sweep} against {hi}..{lo}");
        match (validate_both(&p), want) {
            (Err(got), Ok(want)) => assert_eq!(got, want, "{label}"),
            (Err(ProfileError::Escalated { site, source }), Err(predicate)) => {
                assert_eq!(site, EscalationSite::SegmentPair(pair.0, pair.1), "{label}");
                assert_eq!(source.predicate, Some(predicate), "{label}");
            }
            (got, _) => panic!("{label}: {got:?}"),
        }
    }
}

/// One loop: an arc of `one` into the joint at its polar angle `at`, an
/// arc of `two` out of it through `sweep` (back the way the first came
/// at a cusp, on round at a smooth joint), and lines home.
fn joint_loop(
    (one, two): ((Point2<f64>, f64), (Point2<f64>, f64)),
    at: f64,
    sweep: f64,
) -> profile::Profile<f64> {
    use geom_core::Arc2;
    use profile::{ProfileLoop, RawLoop, Segment};
    let arc = |(centre, radius): (Point2<f64>, f64), sweep| {
        Segment::Arc(Arc2 {
            centre,
            radius,
            sweep,
        })
    };
    let mut vertices = vec![
        (rim(one, at - 2.0), arc(one, 2.0)),
        (rim(one, at), arc(two, sweep)),
        (rim(two, at + sweep), Segment::Line),
    ];
    if sweep > 0.0 {
        vertices.push((Point2::new(0.0, 0.0), Segment::Line));
    }
    profile(vec![ProfileLoop::new(vertices)])
}

/// **A cusp on near-concentric carriers refuses the crossing its arcs
/// make away from the joint.** Carriers `d` apart whose internal
/// clearance `g` is within ε stay within the band for about
/// `√(2(ε + g)/d)` rad either side of the tangency direction, and for
/// `g > 0` cross at `±√(2g/d)`: 0.24 rad out at `d = 17ε`, `g = 0.5ε`.
/// Two arcs leaving a joint there the same way both hold one crossing,
/// so the pair crosses, however near the tangency direction the joint
/// stands; two leaving it opposite ways meet only at the joint, a
/// smooth tangent joint that validates. The joint's arcs are adjacent,
/// so a contact recorded at the joint is the shared vertex's to
/// discount: the smooth rows hold the tangency's contact there, and the
/// cusp rows hold the crossing apart from it.
#[test]
fn a_cusp_on_near_concentric_carriers_refuses_its_crossing() {
    let eps = tol().eps();
    let crossing = Err(ProfileError::NonSimple {
        first: sref(0, 0),
        second: sref(0, 1),
        kind: ContactKind::Crossing,
    });
    for (d, g, at, sweep, want) in [
        (17.0, 0.5, 0.0, -1.6, crossing.clone()),
        (17.0, 0.9, 0.0, -1.6, crossing.clone()),
        (34.0, 0.5, 0.0, -1.2, crossing.clone()),
        (100.0, 0.5, 0.0, -0.8, crossing.clone()),
        (1000.0, 0.5, 0.0, -0.3, crossing.clone()),
        (17.0, 0.5, -10.0, -1.6, crossing.clone()),
        (17.0, 0.5, 10.0, -1.6, crossing.clone()),
        (1000.0, 0.5, -10.0, -0.3, crossing.clone()),
        (17.0, 0.5, 0.0, 1.6, Ok(())),
        (17.0, -0.5, 0.0, 1.6, Ok(())),
        (1000.0, 0.5, -10.0, 1.6, Ok(())),
    ] {
        let p = joint_loop(near_concentric(d * eps, g * eps), at * eps, sweep);
        assert_eq!(
            validate_both(&p),
            want,
            "d = {d}ε, g = {g}ε, joint at {at}ε, second arc through {sweep}"
        );
    }
}

/// **A compound joint escalates only where its crossing pair and its
/// tangency disagree.** Carriers 0.5 to 0.01 apart, internally tangent
/// at the joint to within ε, cut a chord whose sagitta is about
/// `g·r/d`: within the band at `d = 0.1`, `g = 0.2ε`. Read as a
/// tangency or as the two crossings, a smooth joint's arcs meet only at
/// the joint, so it validates whichever the chord is. A cusp's arcs
/// both hold a crossing near the joint: as two crossings they cross
/// there, and as a tangency they meet only at the joint, so the cusp
/// escalates. Past the band the cusp
/// crosses, and a nested pair (`g < 0`) reads its tangency without
/// asking. The `d = 0.01`, `g = 0.03ε` cusp is the band's lower edge:
/// a sagitta of 3ε, which a chord read a fifth as deep calls tangent.
#[test]
fn a_compound_joint_escalates_only_where_its_readings_disagree() {
    let eps = tol().eps();
    let pair = (sref(0, 0), sref(0, 1));
    let crossing = || ProfileError::NonSimple {
        first: pair.0,
        second: pair.1,
        kind: ContactKind::Crossing,
    };
    let (smooth, cusp) = (1.0, -0.5);
    for (d, g, sweep, want) in [
        (0.5, 0.9, smooth, None),
        (0.5, -0.9, smooth, None),
        (0.1, 0.2, smooth, None),
        (0.1, 0.5, smooth, None),
        (0.1, 0.9, smooth, None),
        (0.1, -0.2, smooth, None),
        (0.1, -0.5, smooth, None),
        (0.1, -0.9, smooth, None),
        (0.01, 0.03, smooth, None),
        (0.5, 0.9, cusp, Some(Err("carrier_circles_chord"))),
        (0.1, 0.2, cusp, Some(Err("carrier_circles_chord"))),
        (0.1, 0.9, cusp, Some(Err("carrier_circles_chord"))),
        (0.01, 0.03, cusp, Some(Err("carrier_circles_chord"))),
        (0.01, 0.2, cusp, Some(Ok(crossing()))),
    ] {
        let p = joint_loop(near_concentric(d, g * eps), 0.0, sweep);
        let label = format!("d = {d}, g = {g}ε, second arc through {sweep}");
        match (validate_both(&p), want) {
            (Ok(()), None) => {}
            (Err(got), Some(Ok(want))) => assert_eq!(got, want, "{label}"),
            (Err(ProfileError::Escalated { site, source }), Some(Err(predicate))) => {
                assert_eq!(site, EscalationSite::SegmentPair(pair.0, pair.1), "{label}");
                assert_eq!(source.predicate, Some(predicate), "{label}");
            }
            (got, _) => panic!("{label}: {got:?}"),
        }
    }
}

/// **A tangent cusp's far end draws its line, and nothing reads how
/// long its ε-stretch runs.** The carriers of `d = 17ε`, `g = −0.5ε`
/// are nested and meet only at the joint, and stay within ε of each
/// other out to about 0.24 rad either side of it. The second arc leaves
/// the joint back the way the first came and stops at `sweep`; the
/// line home starts at its far end. That end's gap to the first arc is
/// about `(0.5 + 8.5·sweep²)ε`, so the pair (first arc, line home)
/// touches while it is within ε, escalates while it is in band and
/// validates once it is definite: the three arms of
/// `[topology.census]`, at every ε, whatever the stretch before it.
#[test]
fn a_tangent_cusps_far_end_draws_its_line() {
    let eps = tol().eps();
    let touch = ProfileError::NonSimple {
        first: sref(0, 0),
        second: sref(0, 2),
        kind: ContactKind::Touch,
    };
    for (sweep, want) in [
        (0.05, Err(Ok(touch.clone()))),
        (0.1, Err(Ok(touch.clone()))),
        (0.15, Err(Ok(touch.clone()))),
        (0.2, Err(Ok(touch.clone()))),
        (0.3, Err(Err("line_span"))),
        (0.5, Err(Err("line_span"))),
        (0.7, Err(Err("line_span"))),
        (1.0, Err(Err("circle_side"))),
        (1.3, Ok(())),
        (1.4, Ok(())),
        (1.5, Ok(())),
        (1.6, Ok(())),
    ] {
        let p = joint_loop(near_concentric(17.0 * eps, -0.5 * eps), 0.0, -sweep);
        let label = format!("second arc through −{sweep}");
        match (validate_both(&p), want) {
            (Ok(()), Ok(())) => {}
            (Err(got), Err(Ok(want))) => assert_eq!(got, want, "{label}"),
            (Err(ProfileError::Escalated { site, source }), Err(Err(predicate))) => {
                assert_eq!(
                    site,
                    EscalationSite::SegmentPair(sref(0, 0), sref(0, 2)),
                    "{label}"
                );
                assert_eq!(source.predicate, Some(predicate), "{label}");
            }
            (got, _) => panic!("{label}: {got:?}"),
        }
    }
}

/// **A tangent cusp on near-concentric carriers validates however long
/// its ε-stretch** (Ev's ruling on #5354). Each loop is `joint_loop`'s:
/// the second arc runs back past the stretch, so its far end is
/// definite, and the joint is a decided tangent cusp, its turn read
/// Zero. `g = 0` is pinned at 1e-6 only: at 1e-9 and 1e-12 the stored
/// radius rounds `g` to a few ulps, which the pair pass reads as a
/// chord in band or a crossing.
#[test]
fn a_tangent_cusp_on_near_concentric_carriers_validates_its_long_stretch() {
    let eps = tol().eps();
    let mut rows = vec![
        (17.0 * eps, -0.5 * eps, -1.6, "d = 17ε, g = −0.5ε"),
        (0.1, -0.5 * eps, -0.5, "d = 0.1, g = −0.5ε"),
    ];
    if format!("{eps:e}") == "1e-6" {
        rows.push((17.0 * eps, 0.0, -1.6, "d = 17ε, g = 0"));
    }
    for (d, g, sweep, label) in rows {
        let p = joint_loop(near_concentric(d, g), 0.0, sweep);
        assert_eq!(validate_both(&p), Ok(()), "{label}");
        let v = p.validate(tol()).expect("validated above");
        let lp = &v.loops()[0];
        assert_eq!(
            (lp.tangent_joints(), lp.cusp_joints()),
            (&[1][..], &[1][..]),
            "{label}: the joint is a tangent cusp"
        );
    }
}

/// **A cusp on crossing carriers 95ε apart validates at 1e-12** (Ev's
/// ruling on #5354: the profile is valid). The third review of #5175's
/// witness: two arcs of radius 2.71 whose centres stand 95ε apart and
/// whose radii differ by about as much, joined at one of the carriers'
/// crossings, the second arc running back within ε of the first for
/// 0.13 rad. The joint's turn levered by the shorter leg's arm reads
/// about 0.01ε, a tangent cusp, at `f64` and `Interval`; whether it is
/// a legal edge is the sweep's to say.
#[test]
fn a_cusp_on_crossing_carriers_95_eps_apart_validates() {
    use geom_core::Arc2;
    use profile::{ProfileLoop, RawLoop, Segment};
    if format!("{:e}", tol().eps()) != "1e-12" {
        test_utils::vacuity::stood_down(
            "the 95ε witness",
            "its numbers are 1e-12's; at a wider ε its carriers are one, so no turn is read",
        );
        return;
    }
    let arc = |centre: Point2<f64>, radius, sweep| {
        Segment::Arc(Arc2 {
            centre,
            radius,
            sweep,
        })
    };
    let (c1, r1) = (
        Point2::new(0.8339940436193314, 0.6128195379062746),
        2.709842185979272,
    );
    let (c2, r2) = (
        Point2::new(0.8339940436636458, 0.6128195379906916),
        2.709842185883936,
    );
    let (start, sweep1, sweep2) = (-5.195849007589949, 6.272685487203751, -1.409459670080262);
    let joint = rim((c1, r1), start + sweep1);
    let from = (joint.y - c2.y).atan2(joint.x - c2.x);
    let p = profile(vec![ProfileLoop::new([
        (rim((c1, r1), start), arc(c1, r1, sweep1)),
        (joint, arc(c2, r2, sweep2)),
        (rim((c2, r2), from + sweep2), Segment::Line),
    ])]);
    assert_eq!(validate_both(&p), Ok(()), "the 95ε witness validates");
    let v = p.validate(tol()).expect("validated above");
    let lp = &v.loops()[0];
    assert_eq!(
        (lp.tangent_joints(), lp.cusp_joints()),
        (&[1][..], &[1][..]),
        "the joint is a tangent cusp"
    );
}

/// One loop: an arc of `one` from `at − 1` into the joint at its polar
/// angle `at`, an arc of `two` out of it through `sweep`, and a line
/// home; the second arc starts at the joint's own polar angle about
/// `two`'s centre, so `at` may stand anywhere on both carriers.
fn crossing_loop(
    (one, two): ((Point2<f64>, f64), (Point2<f64>, f64)),
    at: f64,
    sweep: f64,
) -> profile::Profile<f64> {
    use geom_core::Arc2;
    use profile::{ProfileLoop, RawLoop, Segment};
    let arc = |(centre, radius): (Point2<f64>, f64), sweep| {
        Segment::Arc(Arc2 {
            centre,
            radius,
            sweep,
        })
    };
    let joint = rim(one, at);
    let from = (joint.y - two.0.y).atan2(joint.x - two.0.x);
    profile(vec![ProfileLoop::new([
        (rim(one, at - 1.0), arc(one, 1.0)),
        (joint, arc(two, sweep)),
        (rim(two, from + sweep), Segment::Line),
    ])])
}

/// **A joint at a crossing of near-concentric carriers is a corner, not
/// a tangent cusp.** Carriers `d` apart with internal clearance
/// `g = 0.5ε` cross where a vertex on both turns by about
/// `√(2gd)`: 3e-7 rad at `d = 1e-4` to 2e-5 rad at `d = 0.3`, a
/// definite displacement at the unit arm. The carriers' clearance is
/// within ε, which once recorded these joints as decided tangent cusps
/// (back the way the first arc came) or smooth tangent joints; the
/// joint pass reads the turn, so each validates with no tangent joint,
/// both ways out. The rows are the default ε's; at 1e-6 the turn at
/// `d = 1e-4` reads in band, and at 1e-12 the pair pass reads the
/// `d = 0.1` and `0.3` cusps' chord in band, so each row runs only
/// where it holds.
#[test]
fn a_joint_at_a_crossing_of_near_concentric_carriers_is_a_corner() {
    let eps = tol().eps();
    let row = format!("{eps:e}");
    let ds: &[f64] = match row.as_str() {
        "1e-9" => &[0.1, 0.3, 0.01, 1e-4],
        "1e-6" => &[0.1, 0.3, 0.01],
        "1e-12" => &[0.01, 1e-4],
        _ => &[0.01],
    };
    for &d in ds {
        let (one, two) = near_concentric(d, 0.5 * eps);
        let at = -((1.0 + d * d - two.1 * two.1) / (2.0 * d)).acos();
        for sweep in [-0.8, 0.8] {
            let label = format!("d = {d}, joint at {at:e} rad, second arc through {sweep}");
            let p = crossing_loop((one, two), at, sweep);
            assert_eq!(validate_both(&p), Ok(()), "{label}");
            let v = p.validate(tol()).expect("validated above");
            assert!(
                v.loops()[0].tangent_joints().is_empty(),
                "{label}: the joint is a corner, got tangent joints {:?}",
                v.loops()[0].tangent_joints()
            );
        }
    }
}

/// **A cusp whose crossing one arc's span reads in band escalates.**
/// Carriers 0.015 apart cut a chord whose sagitta, 1.5ε, is within the
/// band, so the crossings stand `√(3ε)` rad either side of the tangency
/// direction. The joint stands 10ε along toward one of them, and the
/// second arc leaves it back the way the first came, round to just
/// past the joint: a gap wide enough to read as an arc, narrow enough
/// that its span reads the crossing in band. The carriers stay within ε
/// from the joint to the crossing, so the joint is on both of them, but
/// a touch there is the shared vertex's: the crossing escalates rather
/// than settle on it.
#[test]
fn a_cusp_whose_crossing_one_arc_reads_in_band_escalates() {
    use geom_core::Arc2;
    use profile::{ProfileLoop, RawLoop, Segment};
    let eps = tol().eps();
    let (d, sagitta) = (0.015, 1.5 * eps);
    let (one, two) = near_concentric(d, sagitta * d);
    let arc = |(centre, radius): (Point2<f64>, f64), sweep| {
        Segment::Arc(Arc2 {
            centre,
            radius,
            sweep,
        })
    };
    let v = rim(one, -10.0 * eps);
    let from = (v.y - two.0.y).atan2(v.x - two.0.x);
    let sweep = -(std::f64::consts::TAU - 1.05 * (160.0 * eps).sqrt());
    let out = |angle: f64| rim((Point2::new(0.0, 0.0), 2.0), angle);
    let p = profile(vec![ProfileLoop::new([
        (rim(one, -10.0 * eps - 0.34), arc(one, 0.34)),
        (v, arc(two, sweep)),
        (rim(two, from + sweep), Segment::Line),
        (out(from + sweep), Segment::Line),
        (out(-0.34), Segment::Line),
    ])]);
    match validate_both(&p) {
        Err(ProfileError::Escalated { site, source }) => {
            assert_eq!(site, EscalationSite::SegmentPair(sref(0, 0), sref(0, 1)));
            assert_eq!(source.predicate, Some("carrier_circles_chord"));
        }
        got => panic!("the cusp's crossing should escalate: {got:?}"),
    }
}

#[test]
fn near_tangent_hole_escalates_on_the_internal_clearance() {
    match err(&near_tangent_hole(tol().eps())) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::SegmentPair(sref(0, 0), sref(1, 0)));
            assert_eq!(source.predicate, Some("carrier_circles_internal"));
            match source.margin.diagnostic_f64_for_error_text() {
                ErrorTextReading::Value(m) => {
                    // The clearance is −5ε (up to the cancellation ulp).
                    let eps = tol().eps();
                    assert!(
                        (m + 5.0 * eps).abs() < 0.5 * eps,
                        "margin {m:e} should be ≈ −5ε"
                    );
                }
                other => panic!("expected an in-band value margin, got {other:?}"),
            }
        }
        other => panic!("expected escalation, got {other:?}"),
    }
}

#[test]
fn near_full_arc_is_a_typed_rejection() {
    // Half-gap delta sized so the diameter clearance 2r(1 - cos(delta/2))
    // ~ delta^2/4 lands at 0.5 eps: within tolerance of a full circle
    // (M2 PR 2 review SHOULD-2 fix). r = 1 keeps rounding noise (~1e-15)
    // far below every CI row's eps.
    let eps = tol().eps();
    let delta = (2.0 * eps).sqrt();
    let bulge = 1.0 / (delta / 2.0).tan();
    let (s, c) = delta.sin_cos();
    let p = profile(vec![chain(&[(c, s, bulge), (c, -s, 0.0)])]);
    assert_eq!(err(&p), ProfileError::NearFullArc(sref(0, 0)));
}

#[test]
fn almost_near_full_arc_escalates_on_diameter_clearance() {
    // Same construction with clearance ~ 5 eps: inside the ambiguity
    // band, so the gate escalates naming its predicate.
    let eps = tol().eps();
    let delta = (20.0 * eps).sqrt();
    let bulge = 1.0 / (delta / 2.0).tan();
    let (s, c) = delta.sin_cos();
    let p = profile(vec![chain(&[(c, s, bulge), (c, -s, 0.0)])]);
    match err(&p) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::Segment(sref(0, 0)));
            assert_eq!(source.predicate, Some("arc_diameter_clearance"));
        }
        other => panic!("expected diameter-clearance escalation, got {other:?}"),
    }
}

#[test]
fn nan_coordinates_poison_to_a_typed_error() {
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (f64::NAN, 0.0, 0.0),
        (1.0, 1.0, 0.0),
    ])]);
    match err(&p) {
        ProfileError::Escalated { source, .. } => {
            assert_eq!(source.margin, MarginDiag::INVALID);
        }
        other => panic!("expected poison escalation, got {other:?}"),
    }
}

#[test]
fn nan_bulge_poisons_to_a_typed_error() {
    let p = profile(vec![chain(&[
        (0.0, 0.0, f64::NAN),
        (1.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
    ])]);
    match err(&p) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::Segment(sref(0, 0)));
            assert_eq!(source.margin, MarginDiag::INVALID);
            assert_eq!(source.predicate, Some("arc_sweep_length"));
        }
        other => panic!("expected poison escalation, got {other:?}"),
    }
}

#[test]
fn infinite_coordinates_fail_loudly_without_panicking() {
    for bad in [f64::INFINITY, f64::NEG_INFINITY] {
        let p = profile(vec![chain(&[
            (0.0, 0.0, 0.0),
            (bad, 0.0, 0.0),
            (1.0, 1.0, 0.0),
        ])]);
        // Whatever the classification path, it must be a typed error,
        // never a panic and never an accept (∞ chords poison derived
        // quantities like the unit direction).
        assert!(p.validate(tol()).is_err());
    }
}

#[test]
fn garbage_fuzz_never_panics() {
    // A deterministic little garbage battery: special values in every
    // slot, including poison bulges on degenerate chains.
    let specials = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        f64::NAN,
        f64::INFINITY,
        1e300,
        -5e-324,
    ];
    for (i, &a) in specials.iter().enumerate() {
        for (j, &b) in specials.iter().enumerate() {
            let p = profile(vec![chain(&[
                (a, b, specials[(i + j) % specials.len()]),
                (b, a, 1.0),
                (1.0, 0.5, a),
            ])]);
            let _ = p.validate(tol());
        }
    }
}

#[test]
fn error_display_is_actionable() {
    // S6 (two-tolerance, D4 ¶1 addendum): the exactly-tangent and the
    // in-band refusals are one user situation — both carry the shared
    // recourse fragment, margins ride as payload.
    let e = err(&tangent_hole());
    let msg = e.to_string();
    assert!(msg.contains("tangential contact"), "{msg}");
    assert!(msg.contains("loop 0 segment 0"), "{msg}");
    assert!(msg.contains("loop 1 segment 0"), "{msg}");
    assert_eq!(
        msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
        1,
        "{msg}"
    );

    let e = err(&near_tangent_hole(tol().eps()));
    let msg = e.to_string();
    assert!(
        format!("{e:?}").contains("carrier_circles_internal"),
        "{e:?}"
    );
    assert!(msg.contains("ambiguity band"), "{msg}");
    assert_eq!(
        msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
        1,
        "{msg}"
    );
}

#[test]
fn cw_and_ccw_inputs_reject_identically() {
    // Winding is invisible to errors too: the bowtie reversed reports
    // the same crossing (segment indices mirror deterministically).
    let fwd = err(&profile(vec![bowtie()]));
    let rev = err(&profile(vec![bowtie().reversed()]));
    match (fwd, rev) {
        (ProfileError::NonSimple { kind: k1, .. }, ProfileError::NonSimple { kind: k2, .. }) => {
            assert_eq!(k1, ContactKind::Crossing);
            assert_eq!(k2, ContactKind::Crossing);
        }
        other => panic!("expected two crossings, got {other:?}"),
    }
}

#[test]
fn concentric_circles_of_close_radii_escalate_as_sliver_annulus() {
    // Radii 5ε apart, concentric: the loci sit inside each other's
    // sliver band everywhere — the identity margin escalates.
    let eps = tol().eps();
    let p = profile(vec![
        circle_h(0.0, 0.0, 1.0),
        circle_h(0.0, 0.0, 1.0 + 5.0 * eps),
    ]);
    match err(&p) {
        ProfileError::Escalated { source, .. } => {
            assert_eq!(source.predicate, Some("carrier_circles_identity"));
        }
        other => panic!("expected identity escalation, got {other:?}"),
    }
}

/// **The enclosing gate's IN-BAND arm, both halves.** ρ = R − σ·τ·r is a
/// length like any other gate's margin, so a fillet radius within the
/// ambiguity band of a leg's carrier radius leaves the class question
/// itself undecidable — and the kernel says so rather than picking a
/// side of it.
///
/// Two things are covered here because they are one situation rendered
/// at two doors: the authoring door escalates with the predicate named
/// (so a driver bisecting a parameter box knows which decision went
/// unconfirmed), and the refusal the caller reads carries the same
/// sentence its definite sibling
/// [`profile::path::CornerReason::EnclosesLegCarrier`] carries
/// (D4 ¶1 (iv)).
#[test]
fn a_radius_within_the_band_of_a_carrier_radius_escalates_the_enclosing_gate() {
    let eps = tol().eps();
    // Two unit lobes whose crossing is a real corner, and a radius 5ε
    // above the lobe radius: rho = -5eps, inside (eps, K*eps).
    let tip = 0.75f64.sqrt();
    let err = Open
        .arc_fillet_arc(
            Center {
                c: Point2::new(-0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: Point2::new(0.0, -tip),
            },
            1.0 + 5.0 * eps,
            Center {
                c: Point2::new(0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .expect_err("a radius inside the band of the carrier radius cannot be classified");
    match &err {
        PathError::Escalated { source } => assert_eq!(
            source.predicate,
            Some("fillet_enclosing_carrier"),
            "the escalation must name the gate that could not be classified"
        ),
        other => panic!("expected an escalation, got {other:?}"),
    }
    // The rider: the in-band arm renders the same recourse as the
    // definite refusal, and — like that refusal — it endorses no radius
    // it cannot vouch for.
    // Named, not restated: the sentence is exported under `test-support`
    // precisely so an assertion agrees with the code rather than with a
    // copy of it.
    let rendered = err.to_string();
    assert!(
        rendered.contains(FILLET_ENCLOSING_RECOURSE),
        "the enclosing recourse is missing: {rendered}"
    );
}

/// A two-segment table: a counterclockwise half turn about the origin of
/// stored radius `r`, from `(r + off, 0)` to `(−r, 0)`, and the line
/// back. `off` puts the start `off` off the carrier.
fn half_turn_table(r: f64, off: f64) -> profile::Profile<f64> {
    use geom_core::Arc2;
    use profile::{ProfileLoop, RawLoop, Segment};
    let arc = Arc2 {
        centre: Point2::new(0.0, 0.0),
        radius: r,
        sweep: std::f64::consts::PI,
    };
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new([
        (Point2::new(r + off, 0.0), Segment::Arc(arc)),
        (Point2::new(-r, 0.0), Segment::Line),
    ]);
    profile(vec![lp])
}

/// **A table arc read off its vertex below the scene's resolution is
/// refused as unreadable, not as inconsistent.** The radius is the
/// power of two whose ulp is at least 4·K·ε at the running ε, so
/// `f64` rounding at that magnitude is past the escalation band, and
/// the start sits one ulp off the carrier: `arc_start_on_carrier`
/// reads a definite difference, and `arc_carrier_resolution` says the
/// scene cannot read it. The same one-ulp-scale offset (2·K·ε) at
/// radius 1 is a real inconsistency, the control.
#[test]
fn a_table_arc_off_its_vertex_below_the_scene_resolution_is_unreadable_not_inconsistent() {
    let band = geom_core::Band::linear(tol()).expect("the suite's band");
    let floor = 4.0 * band.escalate();
    let mut r = 1.0_f64;
    while r * f64::EPSILON < floor {
        r *= 2.0;
    }
    let ulp = r * f64::EPSILON;
    assert!(ulp >= floor && ulp > band.escalate());
    match err(&half_turn_table(r, ulp)) {
        ProfileError::ArcBelowSceneResolution { at, check, .. } => {
            assert_eq!(at, sref(0, 0));
            assert_eq!(check, profile::ArcCheck::OnCarrier);
        }
        other => panic!("expected the scene-resolution refusal at r = {r:e}, got {other:?}"),
    }
    match err(&half_turn_table(1.0, 2.0 * band.escalate())) {
        ProfileError::InconsistentArc { at, check } => {
            assert_eq!(at, sref(0, 0));
            assert_eq!(check, profile::ArcCheck::OnCarrier);
        }
        other => panic!("expected a real inconsistency at r = 1, got {other:?}"),
    }
}

/// **A small arc crossing a large one is a crossing, at any ratio of
/// their radii.** A half disc of radius 100, from whose left corner a
/// chord runs up to a short arc of radius 1000ε centred on the half
/// disc's arc. The short arc spans 0.4 rad about the crossing of the
/// two carriers, so it crosses the half disc's arc once, deep inside
/// both, while its ends stand a fifth of its radius either side of
/// it; two lines outside the disc close the loop at the right corner.
/// The radius ratio is 10⁸ at ε = 1e-9 and 10¹¹ at 1e-12.
#[test]
fn a_small_arc_crossing_a_large_one_is_non_simple() {
    use geom_core::Arc2;
    use profile::{ProfileLoop, RawLoop, Segment};
    let (big, small) = (100.0, 1e3 * tol().eps());
    let centre = Point2::new(0.0, big);
    let at = |t: f64| Point2::new(small * t.cos(), big + small * t.sin());
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new([
        (
            Point2::new(big, 0.0),
            Segment::Arc(Arc2 {
                centre: Point2::new(0.0, 0.0),
                radius: big,
                sweep: std::f64::consts::PI,
            }),
        ),
        (Point2::new(-big, 0.0), Segment::Line),
        (
            at(-0.2),
            Segment::Arc(Arc2 {
                centre,
                radius: small,
                sweep: 0.4,
            }),
        ),
        (at(0.2), Segment::Line),
        (Point2::new(2.0 * big, 2.0 * big), Segment::Line),
    ]);
    assert_eq!(
        err(&profile(vec![lp])),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 2),
            kind: ContactKind::Crossing,
        }
    );
}
