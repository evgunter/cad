//! **The cyl×cyl germ arm**: what the intersecting equal-radius
//! cylinder family does at the join, and why the Steinmetz pose is not
//! the fixture it looks like.
//!
//! The family's two walls meet in the two bisector-plane ellipses, and
//! those ellipses are not two disjoint loci. The bisector planes'
//! common line runs through the axes' meeting point `p` along
//! `â₁ × â₂`, is perpendicular to both axes, and therefore meets BOTH
//! walls at `p ± r·n̂`, where `n̂ = unit(â₁ × â₂)` — so the two
//! ellipses CROSS there, at every pose and every angle between the
//! axes. Four arcs, two valence-4 PINCH vertices, always.
//!
//! The UNIT is not decoration: `‖â₁ × â₂‖ = sin θ`, so off the
//! perpendicular pose the raw cross product lands well inside the
//! walls and is not a pinch point at all. Every row below normalizes,
//! and the door's own message says `n̂` for the same reason.
//!
//! Two consequences run through every row below.
//!
//! - **The classic Steinmetz pose puts both operands' seams ON the
//!   pinch points.** An extruded circle's seam rulings sit at azimuth
//!   0 and π of its own chart, which for the axis-aligned pair is
//!   exactly `x = ±1` — the pinch. So the family's most familiar
//!   fixture never reaches the join at all: its seams are TANGENT to
//!   the partner wall and die two layers earlier, at the crossing
//!   layer's tangency door. Turning each operand about its OWN axis
//!   moves the seams off the pinch without moving either SURFACE, and
//!   that pose does reach the join.
//! - **The nodes are built, not refused.** Each germ rides one
//!   ellipse, its branch decided where it is minted, and the join's
//!   bisector lane cuts each wall by that ellipse's plane. A node in
//!   both operands' faces is minted in the reduction as a ring in both
//!   walls, four struts paired across the operands, so every arc of
//!   the section runs node to node and every boolean of the pair builds
//!   at its closed form, for rods and bores in any pairing. A node the
//!   placement cannot certify inside or outside both faces refuses
//!   typed (`NodeUnbuilt`). Branch stubs,
//!   whose nodes miss a face, build without a ring
//!   (`bisector_lane_stubs`); the full family is `bisector_nodes`.
//!
//! Every row is paired with the same fixture under a `transform_rigid`
//! off every axis plane. A pose whose direct-extruded and re-posed
//! copies disagree is a defect by construction, so the pairing is the
//! assertion rather than a convenience.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use sweep::ExtrudeSide;

use crate::common::germ_pair::{cyl, repose, same_door, seams_off_the_pinch, spin, steinmetz};
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::test_support::finished;
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn union_err(a: &Body<f64>, b: &Body<f64>) -> BooleanError {
    let tol = Tol::witness();
    let (a, b) = (
        finished("operand A", a.clone(), tol),
        finished("operand B", b.clone(), tol),
    );
    topo::union(&a, &b, tol).expect_err("this pose refuses")
}

/// Asserts that the refusal of the direct pose and of its re-posed twin
/// are one door ([`same_door`]).
fn assert_same_door(direct: &BooleanError, reposed: &BooleanError, what: &str) {
    assert!(
        same_door(direct, reposed),
        "{what}: direct {direct:?}, re-posed {reposed:?}"
    );
}

/// The single cylinder surface of an operand built by [`cyl`].
fn wall(b: &Body<f64>) -> (Point3<f64>, Vec3<f64>, f64) {
    let mut found = None;
    for (_, s) in b.surfaces() {
        if let topo::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } = s
        {
            assert!(found.is_none(), "the fixture has one wall surface");
            found = Some((*origin, *axis, *radius));
        }
    }
    found.expect("the fixture has a wall")
}

/// **The Steinmetz pair dies at its seams, and the seams die ON the
/// pinch points.** The row asserts three things together, because
/// separately none of them says what happened: the refusal is the
/// crossing layer's, the edge it names is a straight seam ruling, and
/// that ruling touches the partner wall exactly at `p − r·n̂`,
/// `n̂ = unit(â₁ × â₂)` —
/// one of the two points where the section crosses itself.
///
/// So the classic fixture is not a join question at all at this head.
/// It is a tangency, and a tangency ties every first-order datum the
/// pierce machinery reads.
#[test]
fn the_steinmetz_seams_are_tangent_at_the_sections_pinch_points() {
    let (a, b) = steinmetz(2.0);
    let err = union_err(&a, &b);
    let BooleanError::CurvedPierceUnsupported { operand, edge, .. } = &err else {
        panic!("expected the crossing layer's door, got {err:?}");
    };
    assert_eq!(*operand, topo::Operand::A, "A's seam is the raiser");

    let Some(topo::CurveGeom::Certified(c)) =
        a.get_edge(*edge).and_then(|e| a.get_curve_geom(e.curve))
    else {
        panic!("the named edge has no certified curve");
    };
    let topo::Curve3::Line { origin, dir } = *c.carrier() else {
        panic!("a seam ruling is a Line carrier; got {:?}", c.carrier());
    };

    // The two pinch points, from the axes alone.
    let (_, a1, r) = wall(&a);
    let (o2, a2, r2) = wall(&b);
    assert!((r - r2).abs() < 1e-15, "the fixture is equal-radius");
    let n = a1.cross(a2).normalize();
    // The axes meet at the origin by construction (both operands are
    // built centred there), so `p` is the origin.
    let p = Point3::new(0.0, 0.0, 0.0);
    let pinches = [p + n * r, p - n * r];

    // The seam's closest approach to B's axis, in closed form: the
    // ruling is axis-parallel to A, so its residual against B's wall is
    // a parabola in the span parameter and its vertex is the foot.
    let w = origin - o2;
    let perp = |v: Vec3<f64>| v - a2 * v.dot(a2);
    let (wp, dp) = (perp(w), perp(dir));
    let t = -wp.dot(dp) / dp.norm_squared();
    let touch = origin + dir * t;
    let gap = perp(touch - o2).norm() - r;
    assert!(
        gap.abs() < 1e-15,
        "the seam is tangent to the partner wall, not crossing it: gap {gap}"
    );
    let d = pinches
        .iter()
        .map(|q| (touch - *q).norm())
        .fold(f64::MAX, f64::min);
    assert!(
        d < 1e-15,
        "the tangency sits at a pinch point p ± r·n̂, n̂ = unit(â₁ × â₂); it is {d} away"
    );
}

/// The same pair re-posed answers the same door, key for key. A green
/// direct-extruded row beside a differing transformed row would be a
/// defect by construction: `transform_rigid` moves no contact.
#[test]
fn the_steinmetz_pair_answers_identically_under_a_rigid_re_pose() {
    let (a, b) = steinmetz(2.0);
    assert_same_door(
        &union_err(&a, &b),
        &union_err(&repose(&a), &repose(&b)),
        "the re-posed Steinmetz pair must answer exactly what the direct-extruded one does",
    );
}

/// The four operations of a seams-off pose, `[∪, ∩, A∖B, B∖A]`, each
/// built, as volumes with their certified half-widths.
fn four(a: &Body<f64>, b: &Body<f64>, what: &str) -> [(f64, f64); 4] {
    let tol = Tol::witness();
    let (a, b) = (
        finished("operand A", a.clone(), tol),
        finished("operand B", b.clone(), tol),
    );
    let built = |r: Result<topo::BooleanResult<f64>, BooleanError>, op: &str| match r {
        Ok(topo::BooleanResult::Body(body)) => {
            let m =
                topo::mass_properties(&body.body, tol, topo::test_support::pinned_resolution(tol))
                    .unwrap();
            (m.enclosure.volume, m.enclosure.volume_pad)
        }
        other => panic!("{what}, {op}: the family builds, got {other:?}"),
    };
    [
        built(topo::union(&a, &b, tol), "∪"),
        built(topo::intersect(&a, &b, tol), "∩"),
        built(topo::subtract(&a, &b, tol), "A∖B"),
        built(topo::subtract(&b, &a, tol), "B∖A"),
    ]
}

/// Asserts a perpendicular pair of half-height `h ≥ 1` against its
/// closed forms: the intersection is the bicylinder, `16/3`, whole
/// inside both operands, each of volume `2πh`.
fn assert_bicylinder_forms(got: [(f64, f64); 4], h: f64, what: &str) {
    let (v, cap) = (2.0 * PI * h, 16.0 / 3.0);
    let want = [2.0 * v - cap, cap, v - cap, v - cap];
    for (op, ((g, pad), w)) in ["∪", "∩", "A∖B", "B∖A"]
        .into_iter()
        .zip(got.into_iter().zip(want))
    {
        assert!(
            (g - w).abs() <= pad + 1e-12,
            "{what}, {op}: volume {g} ± {pad}, want {w} (off by {:e})",
            g - w
        );
    }
}

/// **The row this unit exists for.** Turn each operand about its own
/// axis — which changes neither SURFACE, only where its chart seam
/// falls — and the same two solids reach the JOIN: the crossing layer
/// finds the seam rulings' four wall crossings, splits both operands,
/// and the germ pair that comes out is cylinder × cylinder.
///
/// Both nodes lie inside both walls, so each is minted as a ring in
/// both, and every operation builds at the bicylinder's closed forms,
/// in the direct pose and its re-posed twin.
#[test]
fn seams_off_the_pinch_build_every_operation_at_the_closed_form() {
    let h = 1.2;
    let (a, b) = seams_off_the_pinch(h, PI / 4.0);
    assert_bicylinder_forms(four(&a, &b, "direct"), h, "direct");
    assert_bicylinder_forms(four(&repose(&a), &repose(&b), "re-posed"), h, "re-posed");
}

/// **The family, not one pose.** Every intersecting equal-radius pose
/// this suite can author with its seams off the nodes is run at every
/// height and seam angle, against its own re-posed twin, and every
/// operation builds at the closed forms: with the seams off the nodes
/// the crossings are found and the sector sides certify, whatever the
/// operand's height.
#[test]
fn every_pose_of_the_family_builds_at_its_closed_form() {
    for h in [1.05_f64, 1.2, 1.5, 2.0] {
        for deg in [15.0_f64, 30.0, 45.0, 60.0, 75.0] {
            let (a, b) = seams_off_the_pinch(h, deg.to_radians());
            let what = format!("h = {h}, {deg}°");
            assert_bicylinder_forms(four(&a, &b, &what), h, &what);
            let what = format!("{what}, re-posed");
            assert_bicylinder_forms(four(&repose(&a), &repose(&b), &what), h, &what);
        }
    }
}

/// **The differentials the fences promise.** None of the three poses
/// reaches the equal-radius ARM, which is what says that arm is a
/// statement about intersecting equal-radius axes and not about
/// cylinder pairs at large.
///
/// - Unequal radii: the axes still intersect, but the radii decide
///   unequal, so the locus is a space quartic, the skew pair's class:
///   the dispatch has no arm (`GermFrameUnsupported`).
/// - Skew axes: the locus is a space quartic, canal territory; the
///   general rung has not retired, and the dispatch has no arm
///   (`GermFrameUnsupported`). Its verdict on this pose is pinned at
///   both radii by
///   `the_non_parallel_cylinder_pair_splits_on_coplanarity_then_radius`
///   (`boolean::join`).
/// - Parallel equal radii: the crossing events are a rim CIRCLE against
///   a wall, certified by the circle × cylinder root lane; at one height
///   the two pairs of cap discs then overlap in their planes, one plane
///   each by margin, which the boolean glues: the union builds at its
///   closed form, in both poses.
#[test]
fn the_fenced_poses_keep_their_own_outcomes() {
    let a = cyl(1.0, 2.0);

    let unequal = spin(&cyl(0.6, 2.0), Vec3::new(1.0, 0.0, 0.0), PI / 2.0);
    let e = union_err(&a, &unequal);
    assert!(
        matches!(
            e,
            BooleanError::GermFrameUnsupported {
                a_kind: geom::SurfaceKind::Cylinder,
                b_kind: geom::SurfaceKind::Cylinder,
                ..
            }
        ),
        "unequal radii: the space quartic has no section arm, got {e:?}"
    );
    assert_same_door(
        &e,
        &union_err(&repose(&a), &repose(&unequal)),
        "unequal radii",
    );

    // Displaced along the common perpendicular `â₁ × â₂ = x̂`: that is
    // the ONE direction that separates the two axes. Sliding the
    // partner along either axis leaves the lines meeting, which is what
    // the frame dispatch's margin measures.
    let skew = topo::transform_rigid(
        &spin(&cyl(1.0, 2.0), Vec3::new(1.0, 0.0, 0.0), PI / 2.0),
        &geom_core::RigidMotion::translation(Vec3::new(0.35, 0.0, 0.0)).unwrap(),
        Tol::witness(),
    )
    .unwrap();
    let e = union_err(&a, &skew);
    assert!(
        matches!(
            e,
            BooleanError::GermFrameUnsupported {
                a_kind: geom::SurfaceKind::Cylinder,
                b_kind: geom::SurfaceKind::Cylinder,
                ..
            }
        ),
        "skew axes: no section arm, got {e:?}"
    );
    assert_same_door(&e, &union_err(&repose(&a), &repose(&skew)), "skew axes");

    // Parallel axes, walls definitely crossing: the rim circle row,
    // whose cap discs are coplanar and glue.
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(1.2, 0.0), 1.0, tol).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, -2.0));
    let parallel = extrude(
        &Profile::new(plane, vec![lp.into()]).validate(tol).unwrap(),
        Extrusion::Distance {
            depth: 4.0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    // The overlap of two unit discs whose centres are 1.2 apart, four
    // high.
    let lens = 2.0 * 0.6_f64.acos() - 0.6 * (4.0_f64 - 1.44).sqrt();
    let want = 4.0 * (2.0 * PI - lens);
    for (pose, a, b) in [
        ("direct", a.clone(), parallel.clone()),
        ("re-posed", repose(&a), repose(&parallel)),
    ] {
        let (a, b) = (finished("operand A", a, tol), finished("operand B", b, tol));
        let built = topo::union(&a, &b, tol);
        let Ok(topo::BooleanResult::Body(bb)) = &built else {
            panic!("parallel-equal-r, {pose}: the union builds: {built:?}");
        };
        let v = topo::test_support::pinned_volume("the measured body", &bb.body, tol).volume;
        assert!(
            (v - want).abs() < 1e-9,
            "parallel-equal-r, {pose}: {v} vs the closed form {want}"
        );
    }
}

/// **`same_door` matches an unglued coincidence arm for arm.** A
/// margin just inside the zero band and one just past it into the
/// ambiguity band lie closer than a band-width, yet one was decided
/// zero and the other refused in band: two doors. Two margins inside
/// the zero band are one.
#[test]
fn same_door_tells_a_decided_zero_from_a_coincidence_in_band() {
    let band = geom_core::Band::linear(Tol::witness()).expect("a linear band");
    let refusal = |m: f64| BooleanError::Escalated {
        decision: topo::BooleanDecision::Coincidence(
            topo::Coincide::Carriers,
            topo::DeclarationRead::Moot,
        ),
        diag: geom_core::Indeterminate {
            margin: geom_core::MarginDiag::value(m * band.zero()),
            band,
            predicate: Some("carrier_unglued_coincidence"),
            terminal_sliver: false,
        },
    };
    assert!(
        !same_door(&refusal(0.9), &refusal(1.1)),
        "a decided zero and an in-band margin are two doors"
    );
    assert!(
        same_door(&refusal(0.9), &refusal(0.2)),
        "two decided zeros a band-width apart at most are one door"
    );
}
