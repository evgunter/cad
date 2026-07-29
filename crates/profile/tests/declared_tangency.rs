//! The #101 declared-tangency discipline, end to end: definite-Zero
//! joint tangency must be declared (point 1), declarations are
//! verified never trusted (point 3), the constructive fillet is the
//! authoring path and is exact on exact inputs (point 4), and
//! same-carrier continuation / transversal joints stay legal
//! undeclared (the free-arc guarantee of point 5).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{bracket, chain, circle_h, profile, quarter_bulge, tol};
use geom_core::Point2;
use profile::{ProfileError, ProfileLoop};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The bracket chain hand-authored WITHOUT declarations: exact
/// line/arc tangency at (1.5, 1) and (1, 1.5) (the #100 constants).
fn undeclared_bracket() -> ProfileLoop<f64> {
    chain(&[
        (0.0, 0.0, 0.0),
        (3.0, 0.0, 0.0),
        (3.0, 1.0, 0.0),
        (1.5, 1.0, -quarter_bulge()),
        (1.0, 1.5, 0.0),
        (1.0, 3.0, 0.0),
        (0.0, 3.0, 0.0),
    ])
}

// ------------------------------------------------- point 1: undeclared --

#[test]
fn undeclared_line_arc_tangency_is_refused_typed() {
    match profile(vec![undeclared_bracket()])
        .validate(tol())
        .expect_err("undeclared tangency must refuse")
    {
        ProfileError::UndeclaredTangency {
            first,
            second,
            joint,
            suggestion,
        } => {
            // The first tangent joint in vertex order is (1.5, 1):
            // vertex 3, between segment 2 (the line arriving) and
            // segment 3 (the fillet arc leaving).
            assert_eq!(joint, 3);
            assert_eq!((first.loop_index, first.segment_index), (0, 2));
            assert_eq!((second.loop_index, second.segment_index), (0, 3));
            assert!(suggestion.contains("declare"), "repair menu: {suggestion}");
            // S6: the definite-tangency arm composes the shared
            // sub-ε_input recourse (two-tolerance principle).
            assert!(
                suggestion.contains(geom_core::COINCIDENCE_RECOURSE),
                "repair menu: {suggestion}"
            );
        }
        other => panic!("expected UndeclaredTangency, got {other:?}"),
    }
}

#[test]
fn declaring_the_joints_repairs_the_refusal() {
    let mut lp = undeclared_bracket();
    lp.tangent_joints = vec![3, 4];
    let vp = profile(vec![lp])
        .validate(tol())
        .expect("declared tangency validates");
    // Canonical start is (0, 0) = input vertex 0, outer loop already
    // counterclockwise: canonical indices coincide with input's.
    assert_eq!(vp.loops()[0].tangent_joints(), &[3, 4]);
}

// ------------------------------------------- point 3: verified, never --

#[test]
fn declared_tangency_on_a_transversal_joint_is_contradicted() {
    // The L-profile's corners are definite right angles.
    let mut lp = common::l_profile();
    lp.tangent_joints = vec![2];
    match profile(vec![lp])
        .validate(tol())
        .expect_err("contradicted declaration must refuse")
    {
        ProfileError::TangencyContradicted {
            joint,
            same_carrier,
            ..
        } => {
            assert_eq!(joint, 2);
            assert!(!same_carrier);
        }
        other => panic!("expected TangencyContradicted, got {other:?}"),
    }
}

#[test]
fn declared_tangency_on_collinear_lines_is_contradicted_as_continuation() {
    // A rectangle with a redundant collinear vertex on the bottom
    // side: legal undeclared (carrier continuation)...
    let redundant = || {
        chain(&[
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (2.0, 0.0, 0.0),
            (2.0, 1.0, 0.0),
            (0.0, 1.0, 0.0),
        ])
    };
    profile(vec![redundant()])
        .validate(tol())
        .expect("collinear continuation is legal undeclared");
    // ...but a tangency declaration there is refused: continuation is
    // not a tangency.
    let mut lp = redundant();
    lp.tangent_joints = vec![1];
    match profile(vec![lp]).validate(tol()).expect_err("continuation") {
        ProfileError::TangencyContradicted {
            joint,
            same_carrier,
            ..
        } => {
            assert_eq!(joint, 1);
            assert!(same_carrier);
        }
        other => panic!("expected same-carrier contradiction, got {other:?}"),
    }
}

#[test]
fn declared_tangency_on_a_cocircular_joint_is_contradicted() {
    // The minimal two-arc circle: joints are cocircular continuation —
    // legal undeclared (asserted all over the corpus), contradicted
    // declared.
    let mut lp = circle_h(0.0, 0.0, 1.0);
    lp.tangent_joints = vec![0];
    match profile(vec![lp]).validate(tol()).expect_err("cocircular") {
        ProfileError::TangencyContradicted { same_carrier, .. } => assert!(same_carrier),
        other => panic!("expected same-carrier contradiction, got {other:?}"),
    }
}

#[test]
fn out_of_range_declaration_is_refused_typed() {
    let mut lp = common::rect(0.0, 0.0, 2.0, 1.0);
    lp.tangent_joints = vec![4];
    match profile(vec![lp]).validate(tol()).expect_err("range") {
        ProfileError::TangentJointOutOfRange {
            loop_index,
            joint,
            count,
        } => {
            assert_eq!((loop_index, joint, count), (0, 4, 4));
        }
        other => panic!("expected TangentJointOutOfRange, got {other:?}"),
    }
}

// ------------------------------------------------- arc/arc tangencies --

/// Two stacked semicircles (externally tangent carriers at (0, 2))
/// closed by a rectangle of lines; joints 0 and 2 are line/arc
/// tangencies (the closing line y = 0 is tangent to the lower carrier
/// at (0, 0) — a cusp, carrier-tangent all the same — and y = 4 to the
/// upper at (0, 4)).
fn s_curve(joints: Vec<usize>) -> ProfileLoop<f64> {
    let mut lp = chain(&[
        (0.0, 0.0, 1.0),
        (0.0, 2.0, -1.0),
        (0.0, 4.0, 0.0),
        (5.0, 4.0, 0.0),
        (5.0, 0.0, 0.0),
    ]);
    lp.tangent_joints = joints;
    lp
}

#[test]
fn external_arc_arc_tangency_must_be_declared() {
    match profile(vec![s_curve(vec![0, 2])])
        .validate(tol())
        .expect_err("undeclared external tangency")
    {
        ProfileError::UndeclaredTangency {
            first,
            second,
            joint,
            ..
        } => {
            assert_eq!(joint, 1);
            assert_eq!(first.segment_index, 0);
            assert_eq!(second.segment_index, 1);
        }
        other => panic!("expected UndeclaredTangency, got {other:?}"),
    }
    profile(vec![s_curve(vec![0, 1, 2])])
        .validate(tol())
        .expect("fully declared S-curve validates");
}

/// A unit semicircle internally tangent at (0, 2) to a radius-2
/// quarter arc (carriers: center (0,1) r 1 inside center (0,0) r 2),
/// closed by transversal lines.
fn internal_tangent_loop(joints: Vec<usize>) -> ProfileLoop<f64> {
    let mut lp = chain(&[
        (0.0, 0.0, 1.0),
        (0.0, 2.0, 1.0 - std::f64::consts::SQRT_2),
        (2.0, 0.0, 0.0),
        (3.0, 0.0, 0.0),
        (3.0, -1.0, 0.0),
        (0.0, -1.0, 0.0),
    ]);
    lp.tangent_joints = joints;
    lp
}

#[test]
fn internal_arc_arc_tangency_must_be_declared() {
    match profile(vec![internal_tangent_loop(vec![])])
        .validate(tol())
        .expect_err("undeclared internal tangency")
    {
        ProfileError::UndeclaredTangency { joint, .. } => assert_eq!(joint, 1),
        other => panic!("expected UndeclaredTangency, got {other:?}"),
    }
    profile(vec![internal_tangent_loop(vec![1])])
        .validate(tol())
        .expect("declared internal tangency validates");
}

// ----------------------------------------- point 4: the constructor --

#[test]
fn fillet_computes_exact_tangent_points_and_declares() {
    let lp = bracket();
    // Right-angle corner, dyadic legs: T1/T2 are bit-exact.
    assert_eq!(lp.vertices[3].pos.x.to_bits(), 1.5f64.to_bits());
    assert_eq!(lp.vertices[3].pos.y.to_bits(), 1.0f64.to_bits());
    assert_eq!(lp.vertices[4].pos.x.to_bits(), 1.0f64.to_bits());
    assert_eq!(lp.vertices[4].pos.y.to_bits(), 1.5f64.to_bits());
    // The arc bulge is tan(-pi/8) to rounding.
    assert!((lp.vertices[3].bulge + quarter_bulge()).abs() < 1e-15);
    // Declares by construction, and the declarations verify.
    assert_eq!(lp.tangent_joints, vec![3, 4]);
    let vp = profile(vec![lp])
        .validate(tol())
        .expect("fillet-authored bracket validates");
    assert_eq!(vp.loops()[0].tangent_joints(), &[3, 4]);
}

#[test]
fn abandoning_a_fillet_exit_leg_is_contradicted_loudly() {
    // fillet(corner, next, r) declares the T2 joint; leaving T2
    // toward anywhere but `next` breaks the declared tangency and
    // validation refuses (never silently accepts the wrong intent).
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 1.0))
        .fillet(p2(1.0, 1.0), p2(1.0, 3.0), 0.5)
        .expect("fillet fits")
        .line_to(p2(0.5, 3.0)) // NOT toward (1, 3)
        .line_to(p2(0.0, 3.0))
        .close();
    match profile(vec![lp]).validate(tol()).expect_err("broken exit") {
        ProfileError::TangencyContradicted { joint, .. } => assert_eq!(joint, 4),
        other => panic!("expected TangencyContradicted, got {other:?}"),
    }
}

#[test]
fn fillet_of_an_acute_corner_validates_at_run_eps() {
    // Non-right angle (60-degree turn at (2,0)): the closed form's
    // sqrt path, still definite-Zero tangency at the run's eps.
    let sqrt3 = 3.0f64.sqrt();
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .fillet(p2(2.0, 0.0), p2(3.0, sqrt3), 0.25)
        .expect("fillet fits")
        .line_to(p2(3.0, sqrt3))
        .line_to(p2(0.0, 2.5))
        .close();
    profile(vec![lp])
        .validate(tol())
        .expect("acute fillet validates");
}

// ------------------------------- the fillet leg-fit gate (MAJOR-1) --

#[test]
fn oversized_fillet_radius_is_refused_typed_both_legs() {
    // The review's MAJOR-1 counterexample, inverted into the pin:
    // r = 10 on legs of length 2 and 3 used to validate green with an
    // arc that never approached the corner (T1 = (3,-8)).
    match ProfileLoop::builder(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .fillet(p2(3.0, 2.0), p2(0.0, 2.0), 10.0)
        .expect_err("oversized radius must refuse")
    {
        ProfileError::FilletDoesNotFit {
            leg,
            setback,
            leg_length,
        } => {
            // Incoming→outgoing gate order: the shorter incoming leg
            // reports first; right angle + dyadic legs: exact values.
            assert_eq!(leg, profile::FilletLeg::Incoming);
            assert_eq!(setback, 10.0);
            assert_eq!(leg_length, 2.0);
        }
        other => panic!("expected FilletDoesNotFit, got {other:?}"),
    }
}

#[test]
fn oversized_fillet_radius_is_refused_for_one_overrun_leg() {
    // Incoming leg (3) fits; outgoing leg (2) overruns at r = 2.5.
    match ProfileLoop::builder(p2(0.0, 0.0))
        .fillet(p2(3.0, 0.0), p2(3.0, 2.0), 2.5)
        .expect_err("outgoing overrun must refuse")
    {
        ProfileError::FilletDoesNotFit {
            leg,
            setback,
            leg_length,
        } => {
            assert_eq!(leg, profile::FilletLeg::Outgoing);
            assert_eq!(setback, 2.5);
            assert_eq!(leg_length, 2.0);
        }
        other => panic!("expected FilletDoesNotFit, got {other:?}"),
    }
}

#[test]
fn zero_radius_fillet_is_a_degenerate_segment_at_validation() {
    // r = 0: setback 0 fits both legs (the gate passes); T1 = T2 =
    // corner and the zero-length arc is refused typed downstream.
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 1.0))
        .fillet(p2(1.0, 1.0), p2(1.0, 3.0), 0.0)
        .expect("zero radius passes the fit gate")
        .line_to(p2(1.0, 3.0))
        .line_to(p2(0.0, 3.0))
        .close();
    match profile(vec![lp]).validate(tol()).expect_err("degenerate") {
        ProfileError::DegenerateSegment(_) => {}
        other => panic!("expected DegenerateSegment, got {other:?}"),
    }
}

#[test]
fn largest_fitting_radius_succeeds_with_exact_tangency() {
    // Boundary: r = 2 on a right angle consumes the incoming leg
    // EXACTLY (setback = leg = 2, margin bit-zero). The arc springs
    // directly off the chain head (no zero-length lead-in line, no
    // declaration there -- the head joint is a genuine transversal
    // corner), and the strictly-interior outgoing tangent point is
    // declared and verifies.
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .fillet(p2(3.0, 2.0), p2(0.0, 2.0), 2.0)
        .expect("exact-fit radius must succeed")
        .line_to(p2(0.0, 2.0))
        .close();
    // 4 vertices: (0,0), (3,0) [arc springs here], T2, (0,2).
    assert_eq!(lp.vertices.len(), 4);
    assert_eq!(lp.tangent_joints, vec![2]);
    let vp = profile(vec![lp])
        .validate(tol())
        .expect("exact-fit fillet validates with verified tangency");
    assert_eq!(vp.loops()[0].tangent_joints(), &[2]);
}

#[test]
fn negative_fillet_radius_is_refused_downstream() {
    // r < 0 puts both tangent points beyond the corner (setback < 0
    // passes the overrun gate -- it is not an overrun); the resulting
    // doubled-back geometry fails simplicity typed. Pinned as
    // documented behavior.
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 1.0))
        .fillet(p2(1.0, 1.0), p2(1.0, 3.0), -0.5)
        .expect("negative radius passes the fit gate")
        .line_to(p2(1.0, 3.0))
        .line_to(p2(0.0, 3.0))
        .close();
    match profile(vec![lp]).validate(tol()).expect_err("negative r") {
        ProfileError::NonSimple {
            kind: profile::ContactKind::Crossing,
            ..
        } => {}
        other => panic!("expected NonSimple crossing, got {other:?}"),
    }
}

#[test]
fn doubled_back_fillet_corner_escalates_at_the_constructor() {
    // phi = pi: the closed form is 0/0 -> NaN; the fit gate's margin
    // poisons and the CONSTRUCTOR refuses typed (site: Fillet).
    match ProfileLoop::builder(p2(0.0, 0.0))
        .fillet(p2(2.0, 0.0), p2(1.0, 0.0), 0.5)
        .expect_err("doubled-back corner must escalate")
    {
        ProfileError::Escalated {
            site: profile::EscalationSite::Fillet,
            ..
        } => {}
        other => panic!("expected fillet-site escalation, got {other:?}"),
    }
}
