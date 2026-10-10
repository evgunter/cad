//! D1's profile tangency, end to end: the tangent-joint set is derived
//! at validation, never stored — from the joints the lattice's
//! constructors made, each verified and never trusted, and from the
//! junctions validation decides Zero, each a tangent joint all the same
//! and recorded ([`profile::ValidatedLoop::decided_joints`]); the PATHS
//! fillet is exact on exact inputs; transversal joints are free.
//!
//! Two vocabularies meet here. Loops whose junctions validation must
//! DECIDE are raw vertex data, which no constructor built. Loops whose
//! constructed joints it must VERIFY are the lattice's
//! ([`profile::ConstructedLoop`]), or, where the geometry contradicts a
//! construction the lattice never makes, its fixture door. Loops the
//! fillet must BUILD go through the §2c surface, where the corner is
//! the carriers' intersection and the anchor-fit refusals are
//! [`profile::PathError`]'s.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{
    bracket_constructed, chain, circle_h, pinned, pinned_constructed, profile, quarter_bulge, tol,
};
use geom_core::Point2;
use geom_core::Tol;
use profile::{
    ConstructedLoop, ConstructedProfile, JointCarriers, Open, PathError, ProfileError, ProfileLoop,
    SketchPlane, Start, ValidatedProfile,
};

/// The bracket chain hand-authored as a table: exact line/arc tangency
/// at (1.5, 1) and (1, 1.5) (the #100 constants), which no constructor
/// made.
fn table_bracket() -> ProfileLoop<f64> {
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

/// Validates loops the lattice constructed (or its fixture door's).
fn constructed(loops: Vec<ConstructedLoop<f64>>) -> Result<ValidatedProfile<f64>, ProfileError> {
    ConstructedProfile::new(SketchPlane::xy(), loops).validate(tol())
}

/// Loop 0's decided joints, as `(joint, carriers)`.
fn decided(vp: &ValidatedProfile<f64>) -> Vec<(usize, JointCarriers)> {
    vp.loops()[0]
        .decided_joints()
        .iter()
        .map(|d| (d.joint, d.carriers))
        .collect()
}

// -------------------------------------------- decided from values --

/// **A tangency no constructor made is a tangent joint all the same,
/// and recorded** (spec §11 row 21, its second half). The table bracket
/// builds, its two joints are in the derived set, and each is recorded
/// with the Zero margin that decided it.
///
/// Red if the gate refuses a value-decided tangency, or derives the
/// joints without recording them.
#[test]
fn a_tangency_no_constructor_made_is_derived_and_recorded() {
    let vp = profile(vec![table_bracket()])
        .validate(tol())
        .expect("a value-decided tangency validates");
    // Canonical start is (0, 0) = input vertex 0, outer loop already
    // counterclockwise: canonical indices coincide with input's.
    assert_eq!(vp.loops()[0].tangent_joints(), &[3, 4]);
    assert_eq!(
        decided(&vp),
        vec![(3, JointCarriers::Tangent), (4, JointCarriers::Tangent)]
    );
    for d in vp.loops()[0].decided_joints() {
        let margin = d.margin.diagnostic_f64_for_error_text().value();
        assert!(
            margin.is_some_and(|m| m.abs() <= tol().eps()),
            "joint {}: the margin that decided it, in the Zero band: {margin:?}",
            d.joint
        );
    }
}

// ------------------------------------- constructed: verified, never --

/// **A constructed joint the geometry contradicts refuses** (spec §11
/// row 21, its third half): the L-profile's corners are definite right
/// angles, so a construction claiming one tangent is a lie validation
/// catches. The same table with no construction is free geometry.
///
/// Red if a constructed joint is trusted rather than verified.
#[test]
fn a_constructed_joint_the_carriers_cross_is_contradicted() {
    match constructed(vec![ConstructedLoop::fixture(common::l_profile(), vec![2])])
        .expect_err("a contradicted construction must refuse")
    {
        ProfileError::TangencyContradicted { joint, .. } => assert_eq!(joint, 2),
        other => panic!("expected TangencyContradicted, got {other:?}"),
    }
    let free = profile(vec![common::l_profile()])
        .validate(tol())
        .expect("transversal joints are free geometry");
    assert!(free.loops()[0].tangent_joints().is_empty());
}

/// **One carrier continuing is a tangent joint** (every zero-turn
/// joint is, Ev in-chat 2026-09-02): a rectangle with a redundant
/// collinear vertex on its bottom side. As a table the joint is decided
/// and recorded; as a construction it is verified and not recorded.
#[test]
fn a_collinear_joint_is_a_tangent_joint_decided_or_constructed() {
    let redundant = || {
        chain(&[
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (2.0, 0.0, 0.0),
            (2.0, 1.0, 0.0),
            (0.0, 1.0, 0.0),
        ])
    };
    let table = profile(vec![redundant()])
        .validate(tol())
        .expect("collinear continuation validates");
    assert_eq!(table.loops()[0].tangent_joints(), &[1]);
    assert_eq!(decided(&table), vec![(1, JointCarriers::Same)]);
    let built = constructed(vec![ConstructedLoop::fixture(redundant(), vec![1])])
        .expect("a constructed collinear joint is a tangent joint");
    assert_eq!(built.loops()[0].tangent_joints(), &[1]);
    assert!(decided(&built).is_empty(), "a construction is not recorded");
}

/// **The minimal two-arc circle**: its joints are one carrier
/// continuing. The `circle` form constructs them; the same table no
/// constructor built decides and records them.
#[test]
fn a_cocircular_joint_is_a_tangent_joint_decided_or_constructed() {
    let table = profile(vec![circle_h(0.0, 0.0, 1.0)])
        .validate(tol())
        .expect("the two-arc circle validates");
    assert_eq!(table.loops()[0].tangent_joints(), &[0, 1]);
    assert_eq!(
        decided(&table),
        vec![(0, JointCarriers::Same), (1, JointCarriers::Same)]
    );
    let form = profile::circle(Point2::new(0.0, 0.0), 1.0, tol()).expect("a circle");
    assert_eq!(form.loop_.constructed_joints(), &[0, 1]);
    let built = constructed(vec![form.loop_]).expect("the circle form validates");
    assert_eq!(built.loops()[0].tangent_joints(), &[0, 1]);
    assert!(
        decided(&built).is_empty(),
        "the form's joints are constructed"
    );
}

/// A constructed joint must name a vertex of its loop. The lattice
/// never mints one that does not, so the row is the fixture door's.
///
/// Red if the range check is dropped (the index would be read as no
/// joint at all, or panic).
#[test]
fn an_out_of_range_constructed_joint_is_refused_typed() {
    let lp = ConstructedLoop::fixture(common::rect(0.0, 0.0, 2.0, 1.0), vec![4]);
    match constructed(vec![lp]).expect_err("range") {
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
fn s_curve() -> ProfileLoop<f64> {
    chain(&[
        (0.0, 0.0, 1.0),
        (0.0, 2.0, -1.0),
        (0.0, 4.0, 0.0),
        (5.0, 4.0, 0.0),
        (5.0, 0.0, 0.0),
    ])
}

/// External arc/arc tangency (input joint 1), with the two line/arc
/// ones: every one decided and recorded. The loop is authored
/// clockwise, so canonicalization reverses it and input joint j is
/// canonical joint (5 − j) mod 5: {0, 1, 2} reads {0, 4, 3}.
#[test]
fn external_arc_arc_tangency_is_decided_and_recorded() {
    let vp = profile(vec![s_curve()])
        .validate(tol())
        .expect("the S-curve validates");
    assert_eq!(vp.loops()[0].tangent_joints(), &[0, 3, 4]);
    assert_eq!(
        decided(&vp),
        vec![
            (0, JointCarriers::Tangent),
            (3, JointCarriers::Tangent),
            (4, JointCarriers::Tangent)
        ]
    );
}

/// A unit semicircle internally tangent at (0, 2) to a radius-2
/// quarter arc (carriers: center (0,1) r 1 inside center (0,0) r 2),
/// closed by transversal lines.
fn internal_tangent_loop() -> ProfileLoop<f64> {
    chain(&[
        (0.0, 0.0, 1.0),
        (0.0, 2.0, 1.0 - std::f64::consts::SQRT_2),
        (2.0, 0.0, 0.0),
        (3.0, 0.0, 0.0),
        (3.0, -1.0, 0.0),
        (0.0, -1.0, 0.0),
    ])
}

/// Internal arc/arc tangency (at (0, 2)): decided and recorded.
#[test]
fn internal_arc_arc_tangency_is_decided_and_recorded() {
    let vp = profile(vec![internal_tangent_loop()])
        .validate(tol())
        .expect("the internal tangency validates");
    let kiss = vp.loops()[0]
        .vertices()
        .iter()
        .position(|v| v.x == 0.0 && v.y == 2.0)
        .expect("the kiss survives canonicalization");
    assert!(vp.loops()[0].tangent_joints().contains(&kiss));
    assert!(decided(&vp).contains(&(kiss, JointCarriers::Tangent)));
}

// ----------------------------------------- point 4: the constructor --

/// **A constructor's joint is in the set and not recorded** (spec §11
/// row 21, its first half): the fillet makes its two joints tangent,
/// validation verifies them, and nothing is decided from values.
///
/// Red if a constructed joint is recorded (a decided joint appears).
#[test]
fn fillet_computes_exact_tangent_points_and_constructs_its_joints() {
    let lp = bracket_constructed();
    // Right-angle corner, dyadic legs: T1/T2 are bit-exact.
    assert_eq!(lp.vertices()[3].x.to_bits(), 1.5f64.to_bits());
    assert_eq!(lp.vertices()[3].y.to_bits(), 1.0f64.to_bits());
    assert_eq!(lp.vertices()[4].x.to_bits(), 1.0f64.to_bits());
    assert_eq!(lp.vertices()[4].y.to_bits(), 1.5f64.to_bits());
    // The arc's sweep reads as the bulge tan(-pi/8) to rounding.
    assert!((crate::common::quarter_tan(&lp.segments()[3]) + quarter_bulge()).abs() < 1e-15);
    // Constructs its two joints, and validation verifies them.
    assert_eq!(lp.constructed_joints(), &[3, 4]);
    let vp = constructed(vec![lp]).expect("fillet-authored bracket validates");
    assert_eq!(vp.loops()[0].tangent_joints(), &[3, 4]);
    assert!(decided(&vp).is_empty(), "a construction is not recorded");
}

#[test]
fn abandoning_a_fillet_exit_leg_is_contradicted_loudly() {
    // The bracket's tangent points, constructed, but with the segment
    // LEAVING T2 = (1, 1.5) pointed somewhere other than along the
    // arc's tangent there: the construction is contradicted and
    // validation refuses (never silently accepts the wrong intent).
    // The fixture door by necessity — the algebra binds an arrival
    // side's direction once, so it cannot author this loop at all.
    let broken = || {
        chain(&[
            (0.0, 0.0, 0.0),
            (3.0, 0.0, 0.0),
            (3.0, 1.0, 0.0),
            (1.5, 1.0, -quarter_bulge()),
            (1.0, 1.5, 0.0),
            (0.5, 3.0, 0.0), // NOT along the arc's tangent (0, 1)
            (0.0, 3.0, 0.0),
        ])
    };
    match constructed(vec![ConstructedLoop::fixture(broken(), vec![3, 4])])
        .expect_err("broken exit")
    {
        ProfileError::TangencyContradicted { joint, .. } => assert_eq!(joint, 4),
        other => panic!("expected TangencyContradicted, got {other:?}"),
    }
    // The same table no constructor built: joint 3 is decided tangent,
    // and joint 4 is a corner.
    let table = profile(vec![broken()])
        .validate(tol())
        .expect("as a table, the broken exit is a corner");
    assert_eq!(decided(&table), vec![(3, JointCarriers::Tangent)]);
}

#[test]
fn fillet_of_an_acute_corner_validates_at_run_eps() {
    // Non-right angle (60-degree turn at the derived corner (2, 0)):
    // the closed form's sqrt path, still definite-Zero tangency at the
    // run's eps. The incoming ray leaves (0, 0) toward +x; the arrival
    // side runs toward (3, √3) and ends at its own anchor there.
    let sqrt3 = 3.0f64.sqrt();
    let lp = pinned(
        Open.at(Point2::new(0.0, 0.0))
            .toward(1.0, 0.0, Tol::witness())
            .expect("the incoming ray runs +x")
            .fillet(0.25, Tol::witness())
            .expect("positive radius")
            .toward(1.0, sqrt3, Tol::witness())
            .expect("the arrival side runs toward (3, √3)")
            .to(Point2::new(3.0, sqrt3), Tol::witness())
            .expect("acute fillet fits")
            .line_to(Point2::new(0.0, 2.5), Tol::witness())
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
    let err = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(3.0, 0.0), Tol::witness())
        .expect("bottom side")
        .toward(0.0, 1.0, Tol::witness())
        .expect("the incoming ray runs +y")
        .fillet(10.0, Tol::witness())
        .expect("positive radius")
        .toward(-1.0, 0.0, Tol::witness())
        .expect("the arrival side runs −x")
        .to(Point2::new(0.0, 2.0), Tol::witness())
        .expect_err("oversized radius must refuse");
    // A straight carrier pair derives ONE corner, so the envelope
    // carries one entry and it is the fit refusal.
    assert_eq!(
        common::corners(&err).len(),
        1,
        "a straight pair derives one corner: {err:?}"
    );
    let Some((side, carrier, setback, available)) = common::anchor_fit(&err) else {
        panic!("expected an anchor-fit entry, got {err:?}")
    };
    assert_eq!(carrier, profile::FilletLegCarrier::Line);
    // Incoming→outgoing gate order: the shorter incoming side
    // reports first; right angle + dyadic legs: exact values.
    assert_eq!(side, profile::FilletLeg::Incoming);
    assert_eq!(setback, 10.0);
    assert_eq!(available, 2.0);
}

#[test]
fn oversized_fillet_radius_is_refused_for_one_overrun_leg() {
    // Incoming side (3) fits; the arrival side (2) overruns at
    // r = 2.5, so the refusal names the outgoing anchor.
    let err = Open
        .at(Point2::new(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .expect("the incoming ray runs +x")
        .fillet(2.5, Tol::witness())
        .expect("positive radius")
        .toward(0.0, 1.0, Tol::witness())
        .expect("the arrival side runs +y")
        .to(Point2::new(3.0, 2.0), Tol::witness())
        .expect_err("outgoing overrun must refuse");
    assert_eq!(
        common::corners(&err).len(),
        1,
        "a straight pair derives one corner: {err:?}"
    );
    let Some((side, carrier, setback, available)) = common::anchor_fit(&err) else {
        panic!("expected an anchor-fit entry, got {err:?}")
    };
    assert_eq!(carrier, profile::FilletLegCarrier::Line);
    assert_eq!(side, profile::FilletLeg::Outgoing);
    assert_eq!(setback, 2.5);
    assert_eq!(available, 2.0);
}

#[test]
fn largest_fitting_radius_succeeds_with_exact_tangency() {
    // Boundary: r = 2 on a right angle consumes the incoming side
    // EXACTLY (setback = extent = 2, margin bit-zero). The arc springs
    // directly off the ray's origin (no zero-length lead-in line, no
    // construction there -- that joint is a genuine transversal
    // corner), and the strictly-interior outgoing tangent point is
    // constructed and verifies.
    let lp = pinned_constructed(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(3.0, 0.0), Tol::witness())
            .expect("bottom side")
            .toward(0.0, 1.0, Tol::witness())
            .expect("the incoming ray runs +y")
            .fillet(2.0, Tol::witness())
            .expect("positive radius")
            .toward(-1.0, 0.0, Tol::witness())
            .expect("the arrival side runs −x")
            .to(Point2::new(0.0, 2.0), Tol::witness())
            .expect("exact-fit radius must succeed")
            .line_to(Start, Tol::witness())
            .expect("the straight seam closes"),
    );
    // 4 vertices: (0,0), (3,0) [arc springs here], T2, (0,2).
    assert_eq!(lp.vertices().len(), 4);
    assert_eq!(lp.constructed_joints(), &[2]);
    let vp = constructed(vec![lp]).expect("exact-fit fillet validates with verified tangency");
    assert_eq!(vp.loops()[0].tangent_joints(), &[2]);
}

#[test]
fn doubled_back_fillet_corner_refuses_as_parallel_carriers() {
    // phi = pi: the arrival carrier is the incoming ray's own line,
    // traversed backwards. There is no corner to cut, and the turn
    // gate says so structurally — before any closed form is asked to
    // divide by a vanishing turn.
    match Open
        .at(Point2::new(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .expect("the incoming ray runs +x")
        .fillet(0.5, Tol::witness())
        .expect("positive radius")
        .toward(-1.0, 0.0, Tol::witness())
        .expect("the arrival side runs back along it")
        .to(Point2::new(1.0, 0.0), Tol::witness())
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
    Open.at(Point2::new(0.0, 4.0))
        .angle(-std::f64::consts::FRAC_PI_2, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, tol())
        .unwrap()
        .tangent_arc_to(Point2::new(0.0, 0.0), tol())
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol())
        .unwrap()
}

/// **The authoring door for the reverse-tangent junction** (D1's
/// wedge-0/2π arm, issue 941 item 2): the verb authors the reverse
/// exactly and constructs the joint, and the profile gate, which reads
/// carrier tangency (a question with no direction in it), verifies it
/// and records nothing.
///
/// The other half is the same table with the provenance given up: the
/// tangency is real geometry, so the gate decides it from values and
/// records it, a cusp all the same.
#[test]
fn the_cusp_verb_constructs_a_joint_the_gate_would_otherwise_record() {
    let built = pinned_constructed(lune());
    assert_eq!(
        built.constructed_joints(),
        &[2],
        "the kiss, constructed once"
    );
    let vp = constructed(vec![built.clone()]).expect("a constructed cusp joint validates");
    assert_eq!(vp.loops()[0].cusp_joints(), &[2]);
    assert!(decided(&vp).is_empty(), "a construction is not recorded");
    let table = profile(vec![built.into_loop()])
        .validate(tol())
        .expect("the same table validates");
    assert_eq!(table.loops()[0].cusp_joints(), &[2]);
    assert_eq!(decided(&table), vec![(2, JointCarriers::Tangent)]);
}

/// The junction is EXACT, not merely inside the band: the kiss lands
/// on the authored point bit for bit, and the two arcs meeting there
/// wind opposite ways — one lip of a lune, not a smooth join.
#[test]
fn the_cusp_junction_is_exact_and_the_two_carriers_oppose() {
    let lp = pinned(lune());
    let v = lp.vertices();
    assert_eq!(v[2].x.to_bits(), 0.0f64.to_bits());
    assert_eq!(v[2].y.to_bits(), 0.0f64.to_bits());
    let inner = crate::common::quarter_tan(&lp.segments()[1]);
    let outer = crate::common::quarter_tan(&lp.segments()[2]);
    assert!(
        inner * outer < 0.0,
        "the lune's two arcs must wind opposite ways: {inner} vs {outer}"
    );
}

/// **No coverage-corpus row leaves a joint to decide**: each loop of
/// the shared coverage corpus — one per verb row — that validates
/// decides none of its joints from values, so no verb row's own
/// construction leaks a tangency. The corpus has no ε-scale junction,
/// and this says nothing about one: the lattice builds a junction its
/// turn band reads as a corner and validation decides Zero
/// (`coincidence_door.rs`'s profile-junction rows), and that joint is
/// recorded. The rows refused for their shape are pinned by index, so
/// a verb row that starts refusing reds here instead of dropping out.
///
/// Red if a corpus row's construction leaves a zero-turn joint
/// unconstructed (validation would decide and record it), or the set
/// of refusing rows moves.
#[test]
fn no_coverage_corpus_row_leaves_a_joint_to_decide() {
    // Row 9, the two arc-mode legs, covers verbs rather than a region:
    // its loop crosses itself.
    const REFUSED: &[(usize, &str)] = &[(9, "NonSimple")];
    let mut refused = Vec::new();
    for (i, closed) in common::coverage_corpus().into_iter().enumerate() {
        let vp = match constructed(vec![closed.loop_]) {
            Ok(vp) => vp,
            Err(e) => {
                let said = format!("{e:?}");
                let kind = said.split([' ', '{', '(']).next().unwrap_or("").to_owned();
                refused.push((i, kind));
                continue;
            }
        };
        assert!(
            vp.loops()[0].decided_joints().is_empty(),
            "corpus row {i}: a joint the lattice built was decided from values: {:?}",
            vp.loops()[0].decided_joints()
        );
    }
    let refused: Vec<(usize, &str)> = refused.iter().map(|(i, k)| (*i, k.as_str())).collect();
    assert_eq!(refused, REFUSED, "the corpus rows refused for their shape");
}

/// **Arc extension constructs no joint at the tip, and leaves none to
/// decide.** The boss chain of `path/family.rs`'s module doc: a
/// fillet-arc arrival onto the circle about (7, 0), then an
/// `arc_fillet` whose `Radius` names that circle's own radius, so the
/// derived carrier IS the arriving one and the arriving leg extends to
/// the trim point instead of meeting a new carrier at the tip.
///
/// Red if the extension leaves a vertex at the tip that validation
/// decides one carrier continuing (a decided joint appears).
#[test]
fn an_arc_extension_leaves_no_joint_to_decide() {
    let t = Tol::witness();
    let closed = Open
        .at(Point2::new(5.05, -1.6))
        .toward(2.1_f64, 0.8, t)
        .unwrap()
        .fillet_arc(
            0.5,
            profile::Center {
                c: Point2::new(7.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: Point2::new(8.5, 0.0),
            },
            t,
        )
        .unwrap()
        .arc_fillet(
            profile::Radius {
                r: 1.5,
                side: profile::ArcSide::Left,
            },
            0.5,
            t,
        )
        .expect("the arrival carrier's own radius extends it")
        .at(Point2::new(4.05, 1.35), t)
        .unwrap()
        .toward(-4.1, 0.3, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .line_to(Start, t)
        .expect("the chain closes");
    let vp = constructed(vec![closed.loop_]).expect("the boss chain validates");
    assert!(
        decided(&vp).is_empty(),
        "every tangent joint of the boss chain is constructed: {:?}",
        decided(&vp)
    );
}
