//! **The declared ARRIVAL at the seam** (BOOL-12, under the Q1
//! fifth-round ruling with PQ4 reopened for the declared case, and
//! Evan's "join-tangent-to-end" for the tangent member).
//!
//! The seam is the one junction whose arriving leg is the LATER
//! authored one, so every declaration that rides the departing leg
//! elsewhere has no departing leg to ride here. The declaration moves
//! to the TARGET, and these rows pin what that buys and what it costs:
//!
//! - the canonical D-shape closes in BOTH directions, one rotation per
//!   straight closer (`continue_to` where the closing leg also
//!   continues its own run, `line_to` where it departs a corner);
//! - the stadium closes on the tangent member, and its seam joint
//!   carries the declared flag the verify layer re-checks;
//! - UNDECLARED, every one of those seams keeps refusing, and the rows
//!   record which refusal — that is the before half of the red-first;
//! - the check is BANDED with a row on each side and one inside, stated
//!   in multiples of the run's own ε so the source means the same thing
//!   on every tolerance leg;
//! - the LEVER is dimension-honest: the threshold is on the
//!   DISPLACEMENT the misalignment opens at the seam, so it does not
//!   drift with the closing leg's length.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::pinned;
use geom_core::{Point2, Tol};
use profile::{Bulge, ClosedLoop, Open, PathError, Profile, ProfileLoop, SketchPlane, Start};
use std::f64::consts::FRAC_PI_2;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn validate_ok(l: &ProfileLoop<f64>) {
    Profile::new(SketchPlane::xy(), vec![l.clone()])
        .validate(Tol::witness())
        .expect("algebra-lowered loop passes the junction verifier");
}

/// The run's classification band, read from the witness rather than
/// written as a literal.
fn band() -> (f64, f64) {
    let t = Tol::witness();
    (t.eps(), t.k() * t.eps())
}

// ------------------------------------------------------------------
// The canonical D-shape (Evan's fixture), both directions.
// ------------------------------------------------------------------

/// `(0,0) → (0,2) — arc → (0,−2) → (0,−1) → (0,0)`: one semicircular
/// side, one straight side authored as three legs, and the loop's entry
/// at `(0,0)` — a SUBDIVISION point of that straight side. The closing
/// leg continues its own run (so it declares the departure) AND arrives
/// straight into the entry's first side (so the target declares the
/// arrival). Two independent facts, one leg.
fn d_shape_forward(closer: Closer) -> Result<ClosedLoop<f64>, PathError<f64>> {
    let t = Tol::witness();
    let p = Open
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
        .unwrap();
    match closer {
        Closer::Declared => p.continue_to(Start.arrives_straight(), t),
        Closer::Undeclared => p.continue_to(Start, t),
        Closer::UndeclaredLineTo => p.line_to(Start, t),
    }
}

/// The same outline traversed the other way: entry `(0,0)` heading
/// SOUTH, the straight side authored down through its subdivision, the
/// arc back up, and the closing leg departing the arc's end at a CORNER
/// while arriving straight into the entry's first side. This rotation
/// declares the arrival ONLY — there is no departure to declare — and it
/// is why the arrival declaration cannot be folded into `continue_to`.
fn d_shape_reverse(closer: Closer) -> Result<ClosedLoop<f64>, PathError<f64>> {
    let t = Tol::witness();
    let p = Open
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
        .unwrap();
    match closer {
        Closer::Declared => p.line_to(Start.arrives_straight(), t),
        Closer::Undeclared | Closer::UndeclaredLineTo => p.line_to(Start, t),
    }
}

#[derive(Clone, Copy)]
enum Closer {
    Declared,
    Undeclared,
    UndeclaredLineTo,
}

#[test]
fn the_d_shape_closes_with_the_declared_straight_arrival() {
    for (name, closed) in [
        ("forward", d_shape_forward(Closer::Declared)),
        ("reverse", d_shape_reverse(Closer::Declared)),
    ] {
        let closed = pinned(closed.unwrap_or_else(|e| panic!("{name}: {e}")));
        validate_ok(&closed);
        assert_eq!(closed.vertices().len(), 4, "{name}");
        // The seam is a declared SUBDIVISION, not a tangent joint: one
        // carrier continues through it, and the #433 ruling says data
        // like that claims no tangency. The arc's two junctions are
        // corners.
        assert!(
            closed.tangent_joints().is_empty(),
            "{name}: {:?}",
            closed.tangent_joints()
        );
    }
}

#[test]
fn the_undeclared_d_shape_seams_keep_refusing() {
    // FORWARD, `continue_to(Start)`: the departure is declared, so the
    // classification reaches the SEAM and refuses there.
    assert!(
        matches!(
            d_shape_forward(Closer::Undeclared),
            Err(PathError::SeamTangent { .. })
        ),
        "{:?}",
        d_shape_forward(Closer::Undeclared)
    );
    // FORWARD, `line_to(Start)`: the closing leg departs a subdivision
    // vertex with a DERIVED direction, so the departure junction
    // refuses first — an ordinary `JunctionTangent`, exactly as it does
    // mid-chain (BOOL-11's addendum). The seam is never reached.
    assert!(
        matches!(
            d_shape_forward(Closer::UndeclaredLineTo),
            Err(PathError::JunctionTangent { .. })
        ),
        "{:?}",
        d_shape_forward(Closer::UndeclaredLineTo)
    );
    // REVERSE: the closing leg departs a CORNER, so the seam is what
    // refuses.
    assert!(
        matches!(
            d_shape_reverse(Closer::Undeclared),
            Err(PathError::SeamTangent { .. })
        ),
        "{:?}",
        d_shape_reverse(Closer::Undeclared)
    );
}

// ------------------------------------------------------------------
// The stadium — the tangent member.
// ------------------------------------------------------------------

/// A stadium: two straight sides and two semicircular caps, tangent at
/// all four joints. The closing cap departs tangent (declared by
/// `.tangent()`, which CONSTRUCTS the arc) and arrives tangent to the
/// entry's outgoing straight (declared by the target, which CHECKS it).
/// One end constructs, the other is checked, so nothing is
/// overdetermined — and the stadium is exactly the non-generic shape a
/// single circular arc CAN serve at both ends.
fn stadium(declared: bool) -> Result<ClosedLoop<f64>, PathError<f64>> {
    let t = Tol::witness();
    let p = Open
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
        .tangent();
    if declared {
        p.tangent_arc_to(Start.arrives_tangent(), t)
    } else {
        p.tangent_arc_to(Start, t)
    }
}

#[test]
fn the_stadium_closes_with_the_declared_tangent_arrival() {
    let closed = pinned(stadium(true).expect("the declared tangent seam closes"));
    validate_ok(&closed);
    assert_eq!(closed.vertices().len(), 4);
    // Joint 0 — the seam — carries the declared flag, and the verify
    // layer above re-checked it. The three interior `.tangent()` joints
    // carry theirs.
    let mut joints = closed.tangent_joints().to_vec();
    joints.sort_unstable();
    assert_eq!(joints, vec![0, 1, 2, 3], "{:?}", closed.tangent_joints());
}

#[test]
fn the_undeclared_stadium_seam_keeps_refusing() {
    // MEASURED BEFORE THE BUILD, and the measurement is why this row
    // exists: the undeclared G1 seam refused, but under the DEPARTURE's
    // name (`JunctionTangent`), because the two arc closers passed
    // `seam: false`. Every seam ARRIVAL now goes through the seam arm,
    // so the refusal names the fact — and its recourse names the
    // declaration, which is a spelling only the seam has.
    let refused = stadium(false);
    assert!(
        matches!(refused, Err(PathError::SeamTangent { .. })),
        "{refused:?}"
    );
    let msg = refused.unwrap_err().to_string();
    assert!(msg.contains("arrives_tangent"), "{msg}");
    assert!(msg.contains("arrives_straight"), "{msg}");
}

/// The other direction round the same stadium — the caps swap roles, so
/// the closing arc is the other one and the entry's first side is the
/// other straight.
#[test]
fn the_stadium_closes_in_the_other_direction_too() {
    let t = Tol::witness();
    let closed = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(p2(2.0, -2.0), t)
        .unwrap()
        .tangent()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(Start.arrives_tangent(), t)
        .expect("the mirrored stadium closes too");
    let closed = pinned(closed);
    validate_ok(&closed);
    assert_eq!(closed.vertices().len(), 4);
}

/// **Both ends tangent by DEMAND, where no circular arc can serve**: the
/// departure tangency is declared and constructs the arc, the arrival
/// tangency is declared and is checked — and the check fails, because a
/// circular arc does not generically carry both. The refusal names the
/// seam FILLET, which constructs both.
#[test]
fn a_both_ends_tangent_seam_no_arc_can_serve_names_the_fillet() {
    let t = Tol::witness();
    let refused = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(Start.arrives_tangent(), t);
    assert!(
        matches!(refused, Err(PathError::SeamArrivalOffDirection { .. })),
        "{refused:?}"
    );
    let msg = refused.unwrap_err().to_string();
    assert!(msg.contains("fillet(r)"), "{msg}");
    assert!(msg.contains("arrives_tangent"), "{msg}");
}

// ------------------------------------------------------------------
// The band, and the lever.
// ------------------------------------------------------------------

/// A closed outline whose closing leg departs a CORNER at `(off, −arm)`
/// and arrives at the entry declaring a straight arrival. The entry's
/// outgoing direction is exactly north, so the arrival's levered miss is
/// `−off` EXACTLY, whatever `arm` is: the leg's direction is
/// `(−off, arm)/L`, its turn against north is `−off/L`, and the lever is
/// `L`. That identity is what makes the two rows below mean what they
/// say.
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

#[test]
fn the_declared_arrival_is_banded_on_both_sides() {
    let (eps, k_eps) = band();
    // Below ε_precision: the arrival and the entry's outgoing direction
    // are the same direction at the precision anything here
    // represents — the declaration is consistent, and the loop closes.
    let closed = tilted_close(0.01 * eps, 1.0).expect("a sub-ε miss is accepted");
    validate_ok(&closed.loop_);
    // Inside the band: nothing is decidable, so it ESCALATES rather
    // than guessing which side of the line the author meant.
    let escalated = tilted_close(3.0 * eps, 1.0);
    assert!(
        matches!(escalated, Err(PathError::Escalated { .. })),
        "{escalated:?}"
    );
    // Past ε_input: definitely different directions, so the authored
    // data contradicts the authored intent and refuses TYPED, with the
    // miss and its lever in the payload.
    let refused = tilted_close(100.0 * k_eps, 1.0);
    match refused {
        Err(PathError::SeamArrivalOffDirection { margin, arm, .. }) => {
            assert!(
                (margin.abs() - 100.0 * k_eps).abs() < 1e-9 * k_eps.max(1.0),
                "margin {margin} is not the authored miss {}",
                100.0 * k_eps
            );
            assert!(arm > 0.0, "the lever is the arriving leg's arm: {arm}");
        }
        other => panic!("{other:?}"),
    }
}

/// **The lever is dimension-honest, and this row FAILS without it.**
/// The threshold is on the DISPLACEMENT the misalignment opens at the
/// seam, not on the angle, so holding the displacement fixed and
/// changing the closing leg's length leaves the verdict alone — while
/// the ANGLE those rows carry differs by the length ratio.
///
/// The first two arms alone did NOT pin the lever: R2 proved by
/// mutation that replacing both `Margin::levered(x, arm)` with
/// `Margin::of(x)` left every shipped row green, because at arms 1 and
/// 4 the levered and unlevered margins land on the same side of the
/// band. What separates the two designs is an arm FAR from 1, and the
/// third arm here is that separator: a 1000 m closing leg missing by
/// `100·ε` of displacement subtends an angle of `0.1·ε`, which an
/// unlevered check calls Zero and ACCEPTS. (R2's own row states the
/// same separation from the falsification side; the degenerate-lever
/// row separates it from the other end, where the arm is sub-ε.)
#[test]
fn the_levered_threshold_does_not_drift_with_leg_length() {
    let (eps, k_eps) = band();
    for arm in [1.0, 4.0] {
        assert!(
            tilted_close(0.01 * eps, arm).is_ok(),
            "accepted at arm {arm}"
        );
        let refused = tilted_close(100.0 * k_eps, arm);
        match refused {
            Err(PathError::SeamArrivalOffDirection {
                margin, arm: lever, ..
            }) => {
                assert!(
                    (margin.abs() - 100.0 * k_eps).abs() < 1e-9 * k_eps.max(1.0),
                    "arm {arm}: margin {margin}"
                );
                // The lever really is the leg, so the ANGLE differs
                // between the two rows even though the verdict does not.
                assert!(
                    (lever - (arm * arm + (100.0 * k_eps).powi(2)).sqrt()).abs() < 1e-9,
                    "arm {arm}: lever {lever}"
                );
            }
            other => panic!("arm {arm}: {other:?}"),
        }
    }
    // THE SEPARATOR. A displacement past ε_input on a very long leg is
    // an angle far below ε. Levered, it refuses; unlevered, it closes —
    // so this arm is what the lever buys, and dropping the lever reds
    // exactly here.
    let long = tilted_close(100.0 * eps, 1000.0);
    match long {
        Err(PathError::SeamArrivalOffDirection { margin, arm, .. }) => {
            assert!(
                (margin.abs() - 100.0 * eps).abs() <= 1e-2 * 100.0 * eps,
                "margin {margin} is not the authored displacement"
            );
            assert!(
                margin.abs() / arm < eps,
                "the ANGLE is sub-eps by construction: {} ",
                margin.abs() / arm
            );
        }
        other => panic!("the long-arm miss must refuse: {other:?}"),
    }
}

/// An arrival that REVERSES the entry's outgoing direction has a
/// near-zero turn too, so the check cannot stop at the turn. It is a
/// cusp, and it refuses under the cusp's own name at the seam exactly as
/// at any other junction — one fact, one refusal.
#[test]
fn a_reversed_declared_arrival_is_a_cusp() {
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
    assert!(
        matches!(refused, Err(PathError::JunctionCusp { .. })),
        "{refused:?}"
    );
}

/// The declaration is what separates the two spellings, and nothing
/// else: the SAME geometry closes with the arrival declared and refuses
/// without it. Nothing is inferred from a value — the kernel checks what
/// the target said.
#[test]
fn the_declaration_is_what_separates_the_two_seam_spellings() {
    assert!(d_shape_forward(Closer::Declared).is_ok());
    assert!(matches!(
        d_shape_forward(Closer::Undeclared),
        Err(PathError::SeamTangent { .. })
    ));
    assert!(stadium(true).is_ok());
    assert!(matches!(stadium(false), Err(PathError::SeamTangent { .. })));
}

/// The seam flag flipped on BOTH arc closers, so the SHARP arc seam
/// names the same fact: a closing `arc_to(Bulge { p: Start, .. })`
/// whose end tangent lands on the entry's outgoing direction is an
/// undeclared tangent SEAM, not an undeclared tangent departure. The
/// mirrored bulge arrives REVERSED at the same seam and stays a cusp,
/// which is the arm the flag does not touch.
#[test]
fn a_sharp_arc_seam_that_arrives_tangent_refuses_as_a_seam() {
    let t = Tol::witness();
    let close = |b: f64| {
        Open.at(p2(0.0, 0.0))
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
            .arc_to(Bulge { p: Start, b }, t)
    };
    assert!(
        matches!(close(1.0), Err(PathError::SeamTangent { .. })),
        "{:?}",
        close(1.0)
    );
    assert!(
        matches!(close(-1.0), Err(PathError::JunctionCusp { .. })),
        "{:?}",
        close(-1.0)
    );
}
