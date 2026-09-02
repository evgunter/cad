//! R1 review probes for BOOL-12 (PR #1573, frozen head 50740f965).
//!
//! Each row pins something the review measured; a row whose assertion
//! documents a DEFECT says so in its doc comment, so that the fix pass
//! can flip it rather than delete it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Bulge, ClosedLoop, Open, PathError, Profile, ProfileError, SketchPlane, Start};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_8, PI};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn validate(l: &ClosedLoop<f64>) -> Result<(), ProfileError> {
    Profile::new(SketchPlane::xy(), vec![l.loop_.clone()])
        .validate(Tol::witness())
        .map(|_| ())
}

fn band() -> (f64, f64) {
    let t = Tol::witness();
    (t.eps(), t.k() * t.eps())
}

/// The PR's `tilted_close` fixture, verbatim: the closing leg departs a
/// corner at `(off, -arm)` and arrives at the entry declaring a
/// straight arrival; the levered miss is exactly `off`.
fn tilted_close(off: f64, arm: f64) -> Result<ClosedLoop<f64>, PathError<f64>> {
    let t = Tol::witness();
    Open.at(p2(0.0, 0.0))
        .angle(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(2.0 + arm, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0 + off, t)
        .unwrap()
        .line_to(Start.arrives_straight(), t)
}

// ------------------------------------------------------------------
// Claim 11: the escalation's recourse direction.
// ------------------------------------------------------------------

/// **DEFECT (R1 MINOR).** The in-band arm of the new keys falls through
/// `PathError::Escalated`'s `_` arm and inherits the shared junction
/// tail — "declare the coincidence, move the geometry, or LOWER the
/// tolerance". At a DECLARED arrival the first lever is already pulled
/// (the target is the declaration) and the third points the wrong way
/// (a LARGER tolerance admits the miss; the definite arm's own message
/// says so). BOOL-11 made exactly this correction for
/// `path_continuation_target_offset`; the two new keys did not get it.
#[test]
fn r1_the_seam_arrival_escalation_inherits_the_junction_tail() {
    let (eps, _) = band();
    let escalated = tilted_close(3.0 * eps, 1.0);
    let err = match escalated {
        Err(e @ PathError::Escalated { .. }) => e,
        other => panic!("expected an escalation in the band: {other:?}"),
    };
    let msg = err.to_string();
    println!("R1: seam-arrival escalation -> {msg}");
    assert!(msg.contains("path_seam_arrival_turn"), "{msg}");
    // The two halves of the inherited template, pinned so the fix pass
    // sees them go.
    assert!(msg.contains("path junction classification"), "{msg}");
    assert!(msg.contains("lower the tolerance"), "{msg}");
    assert!(msg.contains("declare the coincidence"), "{msg}");
}

// ------------------------------------------------------------------
// Claim 3: the two declarations really are independent — the cross
// cases refuse, so neither rotation of the D-shape can use the other's
// verb.
// ------------------------------------------------------------------

#[test]
fn r1_the_d_shape_rotations_cannot_swap_verbs() {
    let t = Tol::witness();
    // Forward, with `line_to` instead of `continue_to`: the departure
    // at (0,-1) is a DERIVED collinear direction, so the departure
    // refuses before the seam is reached.
    let forward_line_to = Open
        .at(p2(0.0, 0.0))
        .angle(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .arc_to(
            Bulge {
                p: p2(0.0, -2.0),
                b: 1.0,
            },
            t,
        )
        .unwrap()
        .line_to(p2(0.0, -1.0), t)
        .unwrap()
        .line_to(Start.arrives_straight(), t);
    assert!(
        matches!(forward_line_to, Err(PathError::JunctionTangent { .. })),
        "{forward_line_to:?}"
    );
    // Reverse, with `continue_to` instead of `line_to`: the closing leg
    // departs the arc's end at a CORNER, so `Start` is off its ray.
    let reverse_continue_to = Open
        .at(p2(0.0, 0.0))
        .angle(-FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(0.0, -2.0), t)
        .unwrap()
        .arc_to(
            Bulge {
                p: p2(0.0, 2.0),
                b: 1.0,
            },
            t,
        )
        .unwrap()
        .continue_to(Start.arrives_straight(), t);
    assert!(
        matches!(
            reverse_continue_to,
            Err(PathError::ContinuationTargetOffRay { .. })
        ),
        "{reverse_continue_to:?}"
    );
}

// ------------------------------------------------------------------
// Claims 4/5: the arrival check is ENTRY-SIDE CARRIER-BLIND.
// ------------------------------------------------------------------

/// **FINDING (R1 MINOR, design surface).** `Start.arrives_straight()`
/// is documented as "the seam is a declared SUBDIVISION point of one
/// carrier", but the check compares DIRECTIONS only: nothing verifies
/// the entry's first side is a LINE. With an ARC first side whose
/// start tangent the closing line meets, the algebra CLOSES with
/// `tangent_joints` empty — and the loop is a G1 line→arc joint with
/// no declaration, which the data gate then refuses
/// `UndeclaredTangency`. So the "straight" token has authored a
/// tangency it declares nothing about, and the only spelling of this
/// seam (a straight leg arriving G1 into an arc) is unrepresentable:
/// `line_to(Start.arrives_tangent())` has no impl by design.
///
/// Fixture: a quarter-circle fan. Entry (0,0); first side the CCW
/// quarter arc about (0,1) to (1,1) — departing EAST; then west to
/// (-1,1), south to (-1,0), and the closing line east into (0,0).
#[test]
fn r1_a_straight_arrival_into_an_arc_first_side_is_an_undeclared_tangency_at_the_gate() {
    let t = Tol::witness();
    let fan = |declared: bool| {
        let p = Open
            .at(p2(0.0, 0.0))
            .arc_to(
                Bulge {
                    p: p2(1.0, 1.0),
                    b: FRAC_PI_8.tan(),
                },
                t,
            )
            .unwrap()
            .line_to(p2(-1.0, 1.0), t)
            .unwrap()
            .line_to(p2(-1.0, 0.0), t)
            .unwrap();
        if declared {
            p.line_to(Start.arrives_straight(), t)
        } else {
            p.line_to(Start, t)
        }
    };
    // Undeclared: the seam refuses as before.
    assert!(
        matches!(fan(false), Err(PathError::SeamTangent { .. })),
        "{:?}",
        fan(false)
    );
    // Declared "straight": the algebra closes, declaring nothing.
    let closed = fan(true).expect("the algebra accepts the declared straight arrival");
    assert!(closed.loop_.tangent_joints().is_empty());
    // ...and the data gate refuses the undeclared line→arc tangency at
    // joint 0. The algebra is not the insurance it says it is here.
    let verdict = validate(&closed);
    println!("R1: straight arrival into an arc first side -> {verdict:?}");
    assert!(
        matches!(
            verdict,
            Err(ProfileError::UndeclaredTangency { joint: 0, .. })
        ),
        "{verdict:?}"
    );
}

/// **FINDING (R1 MINOR, same class).** `Start.arrives_tangent()` on a
/// closing arc COCIRCULAR with the entry's first arc is declared
/// tangency onto carrier IDENTITY — §4 item 4's refusal
/// (`SameCarrierJunction`) at every other junction, and the fifth-round
/// ruling's "declared-tangency-onto-identity all keep refusing". The
/// seam arrival check cannot see the entry's carrier, so the algebra
/// CLOSES and declares joint 0 tangent; the data gate then refuses
/// `TangencyContradicted { same_carrier: true }`.
///
/// Fixture: entry (1,0) heading north; a 3/4 unit circle to (0,-1);
/// a chord to (1/√2, -1/√2); a sharp `.angle(π/4)` departure whose
/// tangent arc through (1,0) is the unit circle again.
#[test]
fn r1_a_cocircular_declared_tangent_arrival_is_carrier_identity_the_algebra_misses() {
    let t = Tol::witness();
    let c = FRAC_PI_4.cos();
    let closed = Open
        .at(p2(1.0, 0.0))
        .arc_to(
            Bulge {
                p: p2(0.0, -1.0),
                b: (3.0 * FRAC_PI_8).tan(),
            },
            t,
        )
        .unwrap()
        .line_to(p2(c, -c), t)
        .unwrap()
        .angle(FRAC_PI_4, t)
        .unwrap()
        .tangent_arc_to(Start.arrives_tangent(), t);
    let closed = match closed {
        Ok(c) => c,
        Err(e) => panic!("R1 expected the algebra to close (the finding): {e:?} / {e}"),
    };
    assert!(closed.loop_.tangent_joints().contains(&0));
    let verdict = validate(&closed);
    println!("R1: cocircular declared tangent seam -> {verdict:?}");
    assert!(
        matches!(
            verdict,
            Err(ProfileError::TangencyContradicted {
                joint: 0,
                same_carrier: true,
                ..
            })
        ),
        "{verdict:?}"
    );
}

// ------------------------------------------------------------------
// Claim 6/12: `SeamTangent`'s recourse from the sharp-arc closer.
// ------------------------------------------------------------------

/// **FINDING (R1 MINOR).** The seam-flag flip on `arc_to_start` makes a
/// tangent sharp-arc seam refuse `SeamTangent`, whose message tells the
/// author to write `Start.arrives_tangent()` — a target `arc_to` cannot
/// take (no impl; the PR's §9 defers the declaration for this verb).
/// The reader is sent to a spelling the verb does not have, which is
/// the failure the PR's own §0.1 corrected in the other direction.
#[test]
fn r1_the_sharp_arc_seam_is_told_to_use_a_target_it_cannot_take() {
    let t = Tol::witness();
    let refused = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .arc_to(Bulge { p: Start, b: 1.0 }, t);
    let err = match refused {
        Err(e @ PathError::SeamTangent { .. }) => e,
        other => panic!("{other:?}"),
    };
    let msg = err.to_string();
    println!("R1: sharp-arc seam -> {msg}");
    assert!(msg.contains("Start.arrives_tangent()"), "{msg}");
}

// ------------------------------------------------------------------
// Claim 4: the lever, re-measured at a third arm and with the payload
// sign.
// ------------------------------------------------------------------

#[test]
fn r1_the_levered_verdict_holds_at_arm_ten_and_the_margin_is_signed() {
    let (eps, k_eps) = band();
    assert!(tilted_close(0.01 * eps, 10.0).is_ok());
    assert!(matches!(
        tilted_close(3.0 * eps, 10.0),
        Err(PathError::Escalated { .. })
    ));
    for sign in [1.0, -1.0] {
        match tilted_close(sign * 100.0 * k_eps, 10.0) {
            Err(PathError::SeamArrivalOffDirection { margin, arm }) => {
                assert!((margin.abs() - 100.0 * k_eps).abs() < 1e-9 * k_eps.max(1.0));
                assert!((arm - 10.0).abs() < 1e-6, "{arm}");
                println!("R1: off={sign:+}·100Kε at arm 10 -> margin {margin:e}");
            }
            other => panic!("{other:?}"),
        }
    }
}

// ------------------------------------------------------------------
// Claim 5: the stadium under the construct-from-arrival form.
// ------------------------------------------------------------------

/// The arc through the departure point `(0,2)` and `Start = (0,0)` with
/// end tangent `Start.dir = east` is the unit semicircle about `(0,1)`
/// — whose START tangent at `(0,2)` is WEST, i.e. exactly the incoming
/// straight's direction. So the departure the construction derives is
/// tangent to the incoming leg: undeclared, `JunctionTangent`;
/// declared, identity of the two tangencies at a corner-less junction.
/// The PR's deviation reason holds: measured here with the algebra's
/// own classifier on the derived departure.
#[test]
fn r1_the_construct_from_arrival_form_would_derive_a_tangent_departure_on_the_stadium() {
    let t = Tol::witness();
    // Reproduce the derived departure through the existing verbs: the
    // arc from (0,2) that arrives at (0,0) heading east is the one
    // departing (0,2) heading WEST, so author that departure as an
    // AUTHORED angle and let the classifier judge it against the
    // incoming west-bound straight.
    let refused = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(p2(2.0, 2.0), t)
        .unwrap()
        .tangent()
        .line(2.0, t)
        .unwrap()
        .angle(PI, t);
    assert!(
        matches!(refused, Err(PathError::JunctionTangent { .. })),
        "{refused:?}"
    );
}

// ------------------------------------------------------------------
// Q4 (style lane): a premise the unit invalidated one layer down.
// ------------------------------------------------------------------

/// **FORWARD OBSERVATION (R1 NOTE; lift.rs is out of fence).**
/// `LiftRefusal::AllJointsDeclared` says an all-tangent loop has "no
/// sharp joint to seam the chain at" and names the seam fillet as the
/// only spelling. This unit made that premise false: the stadium
/// authors through `tangent_arc_to(Start.arrives_tangent())` with all
/// four joints declared, and the lift layer still refuses to lift the
/// very loop the algebra just produced.
#[test]
fn r1_the_lift_layer_still_refuses_the_all_tangent_loop_the_algebra_now_authors() {
    let t = Tol::witness();
    let closed = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(p2(2.0, 2.0), t)
        .unwrap()
        .tangent()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(Start.arrives_tangent(), t)
        .expect("the stadium closes");
    let lifted = profile::lift(&closed.loop_, t);
    println!("R1: lift(stadium) -> {lifted:?}");
    assert!(
        matches!(
            lifted,
            Err(profile::LiftRefusal::AllJointsDeclared { joints: 4 })
        ),
        "{lifted:?}"
    );
}

/// **FINDING (R1 MINOR, class of the PR's own §0.1).** A REVERSED
/// declared arrival refuses `JunctionCusp`, whose recourse is the
/// departure's — author the cusp with `.cusp()`. At the seam the
/// arriving leg is the later-authored one and the entry cannot carry
/// `.cusp()` (§2's entry rule), so the reader is sent to a spelling the
/// seam does not have — the exact defect the PR corrected for the
/// tangent arm (`JunctionTangent` → `SeamTangent`) and left in the cusp
/// arm, at both `seam_arrival_check` and `junction_check(seam = true)`.
#[test]
fn r1_the_seam_cusp_names_a_departure_recourse() {
    let t = Tol::witness();
    let refused = Open
        .at(p2(0.0, 0.0))
        .angle(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .turn(-FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .turn(-FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .turn(-FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .continue_to(Start.arrives_straight(), t);
    let err = match refused {
        Err(e @ PathError::JunctionCusp { .. }) => e,
        other => panic!("{other:?}"),
    };
    let msg = err.to_string();
    println!("R1: reversed declared arrival -> {msg}");
    assert!(msg.contains(".cusp()"), "{msg}");
}
