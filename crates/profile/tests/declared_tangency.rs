//! The #101 declared-tangency discipline, end to end: definite-Zero
//! joint tangency must be declared (point 1), declarations are
//! verified never trusted (point 3), the PATHS fillet is the authoring
//! path and is exact on exact inputs (point 4), and same-carrier
//! continuation / transversal joints stay legal undeclared (the
//! free-arc guarantee of point 5).
//!
//! Two vocabularies meet here. Loops the verify layer must JUDGE —
//! undeclared, contradicted, out-of-range — are raw vertex data,
//! because a declaration that contradicts its own geometry is exactly
//! what the algebra cannot author. Loops the fillet must BUILD go
//! through the §2c surface, where the corner is the carriers'
//! intersection and the anchor-fit refusals are
//! [`profile::PathError`]'s.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{bracket, chain, circle_h, pinned, profile, quarter_bulge, tol};
use geom_core::Point2;
use geom_core::Tol;
use profile::{Open, PathError, ProfileError, ProfileLoop, RawLoop, Start};

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
            assert_eq!(
                suggestion.matches(geom_core::COINCIDENCE_RECOURSE).count(),
                1,
                "repair menu: {suggestion}"
            );
        }
        other => panic!("expected UndeclaredTangency, got {other:?}"),
    }
}

#[test]
fn declaring_the_joints_repairs_the_refusal() {
    let mut lp = undeclared_bracket();
    lp = lp.with_tangent_joints(vec![3, 4]);
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
    lp = lp.with_tangent_joints(vec![2]);
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
    lp = lp.with_tangent_joints(vec![1]);
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
    lp = lp.with_tangent_joints(vec![0]);
    match profile(vec![lp]).validate(tol()).expect_err("cocircular") {
        ProfileError::TangencyContradicted { same_carrier, .. } => assert!(same_carrier),
        other => panic!("expected same-carrier contradiction, got {other:?}"),
    }
}

#[test]
fn out_of_range_declaration_is_refused_typed() {
    let mut lp = common::rect(0.0, 0.0, 2.0, 1.0);
    lp = lp.with_tangent_joints(vec![4]);
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
    lp = lp.with_tangent_joints(joints);
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
    lp = lp.with_tangent_joints(joints);
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
    assert_eq!(lp.vertices()[3].pos().x.to_bits(), 1.5f64.to_bits());
    assert_eq!(lp.vertices()[3].pos().y.to_bits(), 1.0f64.to_bits());
    assert_eq!(lp.vertices()[4].pos().x.to_bits(), 1.0f64.to_bits());
    assert_eq!(lp.vertices()[4].pos().y.to_bits(), 1.5f64.to_bits());
    // The arc bulge is tan(-pi/8) to rounding.
    assert!((lp.vertices()[3].bulge() + quarter_bulge()).abs() < 1e-15);
    // Declares by construction, and the declarations verify.
    assert_eq!(lp.tangent_joints(), vec![3, 4]);
    let vp = profile(vec![lp])
        .validate(tol())
        .expect("fillet-authored bracket validates");
    assert_eq!(vp.loops()[0].tangent_joints(), &[3, 4]);
}

#[test]
fn abandoning_a_fillet_exit_leg_is_contradicted_loudly() {
    // The bracket's tangent points, declared, but with the segment
    // LEAVING T2 = (1, 1.5) pointed somewhere other than along the
    // arc's tangent there: the declaration is contradicted and
    // validation refuses (never silently accepts the wrong intent).
    // Raw data by necessity — the algebra binds an arrival side's
    // direction once, so it cannot author this loop at all.
    let mut lp = chain(&[
        (0.0, 0.0, 0.0),
        (3.0, 0.0, 0.0),
        (3.0, 1.0, 0.0),
        (1.5, 1.0, -quarter_bulge()),
        (1.0, 1.5, 0.0),
        (0.5, 3.0, 0.0), // NOT along the arc's tangent (0, 1)
        (0.0, 3.0, 0.0),
    ]);
    lp = lp.with_tangent_joints(vec![3, 4]);
    match profile(vec![lp]).validate(tol()).expect_err("broken exit") {
        ProfileError::TangencyContradicted { joint, .. } => assert_eq!(joint, 4),
        other => panic!("expected TangencyContradicted, got {other:?}"),
    }
}

#[test]
fn fillet_of_an_acute_corner_validates_at_run_eps() {
    // Non-right angle (60-degree turn at the derived corner (2, 0)):
    // the closed form's sqrt path, still definite-Zero tangency at the
    // run's eps. The incoming ray leaves (0, 0) toward +x; the arrival
    // side runs toward (3, √3) and ends at its own anchor there.
    let sqrt3 = 3.0f64.sqrt();
    let lp = pinned(
        Open.at(p2(0.0, 0.0))
            .toward(1.0, 0.0, Tol::witness())
            .expect("the incoming ray runs +x")
            .fillet(0.25, Tol::witness())
            .expect("positive radius")
            .toward(1.0, sqrt3, Tol::witness())
            .expect("the arrival side runs toward (3, √3)")
            .to(p2(3.0, sqrt3), Tol::witness())
            .expect("acute fillet fits")
            .line_to(p2(0.0, 2.5), Tol::witness())
            .expect("the far side")
            .line_to(Start, Tol::witness())
            .expect("the straight seam closes"),
    );
    profile(vec![lp])
        .validate(tol())
        .expect("acute fillet validates");
}

// ------------------------------- the fillet leg-fit gate (MAJOR-1) --

#[test]
fn oversized_fillet_radius_is_refused_typed_both_legs() {
    // The review's MAJOR-1 counterexample, inverted into the pin:
    // r = 10 on legs of length 2 and 3 used to validate green with an
    // arc that never approached the corner (T1 = (3,-8)). The corner
    // (3, 2) is the carriers' intersection; the anchors that pin the
    // two extents are the ray's origin (3, 0) and the arrival's own
    // anchor (0, 2).
    match Open
        .at(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0), Tol::witness())
        .expect("bottom side")
        .toward(0.0, 1.0, Tol::witness())
        .expect("the incoming ray runs +y")
        .fillet(10.0, Tol::witness())
        .expect("positive radius")
        .toward(-1.0, 0.0, Tol::witness())
        .expect("the arrival side runs −x")
        .to(p2(0.0, 2.0), Tol::witness())
        .expect_err("oversized radius must refuse")
    {
        PathError::AnchorOutsideTrimmedExtent {
            side,
            carrier,
            setback,
            available,
        } => {
            assert_eq!(carrier, profile::FilletLegCarrier::Line);
            // Incoming→outgoing gate order: the shorter incoming side
            // reports first; right angle + dyadic legs: exact values.
            assert_eq!(side, profile::FilletLeg::Incoming);
            assert_eq!(setback, 10.0);
            assert_eq!(available, 2.0);
        }
        other => panic!("expected AnchorOutsideTrimmedExtent, got {other:?}"),
    }
}

#[test]
fn oversized_fillet_radius_is_refused_for_one_overrun_leg() {
    // Incoming side (3) fits; the arrival side (2) overruns at
    // r = 2.5, so the refusal names the outgoing anchor.
    match Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .expect("the incoming ray runs +x")
        .fillet(2.5, Tol::witness())
        .expect("positive radius")
        .toward(0.0, 1.0, Tol::witness())
        .expect("the arrival side runs +y")
        .to(p2(3.0, 2.0), Tol::witness())
        .expect_err("outgoing overrun must refuse")
    {
        PathError::AnchorOutsideTrimmedExtent {
            side,
            carrier,
            setback,
            available,
        } => {
            assert_eq!(carrier, profile::FilletLegCarrier::Line);
            assert_eq!(side, profile::FilletLeg::Outgoing);
            assert_eq!(setback, 2.5);
            assert_eq!(available, 2.0);
        }
        other => panic!("expected AnchorOutsideTrimmedExtent, got {other:?}"),
    }
}

#[test]
fn largest_fitting_radius_succeeds_with_exact_tangency() {
    // Boundary: r = 2 on a right angle consumes the incoming side
    // EXACTLY (setback = extent = 2, margin bit-zero). The arc springs
    // directly off the ray's origin (no zero-length lead-in line, no
    // declaration there -- that joint is a genuine transversal
    // corner), and the strictly-interior outgoing tangent point is
    // declared and verifies.
    let lp = pinned(
        Open.at(p2(0.0, 0.0))
            .line_to(p2(3.0, 0.0), Tol::witness())
            .expect("bottom side")
            .toward(0.0, 1.0, Tol::witness())
            .expect("the incoming ray runs +y")
            .fillet(2.0, Tol::witness())
            .expect("positive radius")
            .toward(-1.0, 0.0, Tol::witness())
            .expect("the arrival side runs −x")
            .to(p2(0.0, 2.0), Tol::witness())
            .expect("exact-fit radius must succeed")
            .line_to(Start, Tol::witness())
            .expect("the straight seam closes"),
    );
    // 4 vertices: (0,0), (3,0) [arc springs here], T2, (0,2).
    assert_eq!(lp.vertices().len(), 4);
    assert_eq!(lp.tangent_joints(), vec![2]);
    let vp = profile(vec![lp])
        .validate(tol())
        .expect("exact-fit fillet validates with verified tangency");
    assert_eq!(vp.loops()[0].tangent_joints(), &[2]);
}

#[test]
fn doubled_back_fillet_corner_refuses_as_parallel_carriers() {
    // phi = pi: the arrival carrier is the incoming ray's own line,
    // traversed backwards. There is no corner to cut, and the turn
    // gate says so structurally — before any closed form is asked to
    // divide by a vanishing turn.
    match Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .expect("the incoming ray runs +x")
        .fillet(0.5, Tol::witness())
        .expect("positive radius")
        .toward(-1.0, 0.0, Tol::witness())
        .expect("the arrival side runs back along it")
        .to(p2(1.0, 0.0), Tol::witness())
        .expect_err("doubled-back corner must refuse")
    {
        PathError::NoCornerForFillet {
            reason: profile::path::PathNoCornerReason::CarriersParallel,
            radius,
        } => assert_eq!(radius, 0.5),
        other => panic!("expected CarriersParallel, got {other:?}"),
    }
}

// --------------------------------------- the cusp door (issue 941 item 2) --

/// The lune between two internally tangent circles, cut on the y axis:
/// the cross-section of D1's own kissing-cylinders figure, and the
/// profile a cusp-edged solid is swept from. The junction at the kiss
/// is authored by `.cusp()` — `.tangent()`'s mirror, departing along
/// the negated incoming ray — and every other corner is a right angle.
fn lune() -> profile::ClosedLoop<f64> {
    Open.at(p2(0.0, 4.0))
        .angle(-std::f64::consts::FRAC_PI_2, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, tol())
        .unwrap()
        .tangent_arc_to(p2(0.0, 0.0), tol())
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol())
        .unwrap()
}

/// **The authoring door for the reverse-tangent junction** (D1's
/// wedge-0/2π arm, issue 941 item 2): the verb authors the reverse
/// exactly and DECLARES the joint, and the profile data gate — which
/// judges declared joints by carrier tangency, a question with no
/// direction in it — accepts the declaration as it stands.
///
/// The red half is the same loop with the declaration removed: the
/// tangency is real geometry, so the gate refuses it undeclared
/// exactly as it refuses a same-direction one. What the verb supplies
/// is the intent, which no margin can.
#[test]
fn the_cusp_verb_declares_a_joint_the_data_gate_already_accepts() {
    let lp = pinned(lune());
    assert_eq!(lp.tangent_joints(), &[2], "the kiss, declared once");
    profile(vec![lp.clone()])
        .validate(tol())
        .expect("a declared cusp joint validates");
    match profile(vec![lp.with_tangent_joints(Vec::new())])
        .validate(tol())
        .expect_err("an undeclared cusp joint must refuse")
    {
        ProfileError::UndeclaredTangency { joint, .. } => assert_eq!(joint, 2),
        other => panic!("expected UndeclaredTangency, got {other:?}"),
    }
}

/// The junction is EXACT, not merely inside the band: the kiss lands
/// on the authored point bit for bit, and the two arcs meeting there
/// wind opposite ways — one lip of a lune, not a smooth join.
#[test]
fn the_cusp_junction_is_exact_and_the_two_carriers_oppose() {
    let lp = pinned(lune());
    let v = lp.vertices();
    assert_eq!(v[2].pos().x.to_bits(), 0.0f64.to_bits());
    assert_eq!(v[2].pos().y.to_bits(), 0.0f64.to_bits());
    let inner = v[1].bulge();
    let outer = v[2].bulge();
    assert!(
        inner * outer < 0.0,
        "the lune's two arcs must wind opposite ways: {inner} vs {outer}"
    );
}
