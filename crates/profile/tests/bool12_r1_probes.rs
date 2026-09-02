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

/// **R1 MINOR, FIXED — the row flipped.** The in-band arm of the new
/// keys used to fall through `PathError::Escalated`'s `_` arm and
/// inherit the shared junction tail — "declare the coincidence, move
/// the geometry, or LOWER the tolerance" — where at a DECLARED arrival
/// the first lever is already pulled and the third points the wrong
/// way. The two keys now compose their own recourse, as BOOL-11 did for
/// `path_continuation_target_offset`, and this row asserts the
/// composition rather than the inheritance.
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
    // The two halves of the inherited template are GONE.
    assert!(!msg.contains("path junction classification"), "{msg}");
    assert!(!msg.contains("declare the coincidence"), "{msg}");
    // And the composed recourse points the right way: the declaration
    // is already made, and a LARGER tolerance is what admits the miss.
    assert!(msg.contains("the declaration is the target"), "{msg}");
    assert!(msg.contains("widen the input tolerance"), "{msg}");
    assert!(
        msg.contains("LOWERING the tolerance is the wrong direction"),
        "{msg}"
    );
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
/// tangency it declares nothing about, and at the time of the review
/// the only spelling of this seam — a straight leg arriving G1 into an
/// arc — was unrepresentable: `line_to(Start.arrives_tangent())` had no
/// impl.
///
/// **RULED (Evan, in-chat, 2026-09-02), and the finding is answered in
/// BOTH halves.** The arrival token classifies the JOINT, not the leg,
/// so `line_to(Start.arrives_tangent())` is now a spelling and the
/// second half of this finding is gone. And nothing at the seam may
/// consult the following carrier, so the first half is DELIBERATE: the
/// lattice cannot see that the entry's first side is an arc, the
/// declared-straight close therefore lands, and the DATA gate — the
/// layer that owns materialized carriers — is what refuses it. This row
/// pins that division of labour, then runs the recourse and requires it
/// to close and validate.
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
    // Declared "straight": the algebra closes, declaring nothing —
    // `arrives_straight` says the seam is a SUBDIVISION joint, and the
    // lattice cannot see that the following carrier is an arc.
    let closed = fan(true).expect("the algebra accepts the declared straight arrival");
    assert!(closed.loop_.tangent_joints().is_empty());
    // ...and the DATA gate refuses the line-onto-arc tangency at joint
    // 0: the data says two carriers meet where the declaration said one
    // continued.
    let verdict = validate(&closed);
    println!("R1: straight arrival into an arc first side -> {verdict:?}");
    assert!(
        matches!(
            verdict,
            Err(ProfileError::UndeclaredTangency { joint: 0, .. })
        ),
        "{verdict:?}"
    );

    // THE RECOURSE, and the ruling's own point: the same straight
    // closer declaring a TANGENT joint closes AND validates.
    let g1 = Open
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
        .unwrap()
        .line_to(Start.arrives_tangent(), t)
        .expect("a straight leg may declare a TANGENT seam joint");
    assert_eq!(g1.loop_.tangent_joints(), &[0]);
    validate(&g1).expect("the declared G1 seam validates");
}

/// **R1 MINOR, FIXED — the row flipped.** `Start.arrives_tangent()` on
/// a closing arc COCIRCULAR with the entry's first arc is declared
/// tangency onto carrier IDENTITY — §4 item 4's refusal
/// (`SameCarrierJunction`) at every other junction, and the fifth-round
/// ruling's "declared-tangency-onto-identity all keep refusing". The
/// seam check now READS the entry's first side (`Core::first_side`) and
/// refuses there, instead of closing and leaving the data gate to say
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
    let closed = closed.expect("the algebra closes: it cannot see the following carrier");
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

    // The RECOURSE: the same closing arc declaring a SUBDIVISION joint
    // instead — which is what a cocircular seam actually is — closes,
    // declares nothing, and validates.
    let sub = Open
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
        .tangent_arc_to(Start.arrives_straight(), t)
        .expect("a closing arc may declare a SUBDIVISION seam joint");
    assert!(sub.loop_.tangent_joints().is_empty());
}

// ------------------------------------------------------------------
// Claim 6/12: `SeamTangent`'s recourse from the sharp-arc closer.
// ------------------------------------------------------------------

/// **R1 MINOR, FIXED by giving the verb the spelling.** The seam-flag
/// flip on `arc_to_start` makes a tangent sharp-arc seam refuse
/// `SeamTangent`, whose message names `Start.arrives_tangent()` — a
/// target `arc_to` could not take. Rather than fork the message by
/// closer, `Bulge` gained the declaration: the bulge already fixes the
/// arc's end tangent, so the CHECK form applies unchanged. This row now
/// runs the recourse the message gives and requires it to CLOSE.
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
    // The recourse is real: the same chain closes with it.
    let closed = Open
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
        .arc_to(
            Bulge {
                p: Start.arrives_tangent(),
                b: 1.0,
            },
            t,
        )
        .expect("the message's own recourse closes the loop");
    // The seam is a declared G1 joint, so joint 0 carries the flag and
    // the gate re-checks it.
    assert!(closed.loop_.tangent_joints().contains(&0));
    validate(&closed).expect("the declared sharp-arc seam validates");
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
            Err(PathError::SeamArrivalOffDirection { margin, arm, .. }) => {
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
