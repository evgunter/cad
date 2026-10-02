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
//! - **The join has no frame for the pair.** A germ-pair frame is one
//!   conic's centre and axis, and a self-crossing ellipse pair is not
//!   one conic. The dispatch reads WHICH of the two shapes the locus
//!   has off the lowered parameter-identity channel — `Declared` for
//!   the equal-radius pinch, `None` for the open question — but
//!   neither answer yields one conic, so it has no frame to hand over
//!   either way. It refuses typed at a door that names the pinch. Walking the section across
//!   a pinch is a chord lane this tree does not have — the plane-side
//!   `BoolPlanar` chord and the plane-carrying `Split` context are
//!   both premised on one member of the pair being a PLANE — and that
//!   lane is not this unit's to invent.
//!
//! Every row is paired with the same fixture under a `transform_rigid`
//! off every axis plane. A pose whose direct-extruded and re-posed
//! copies disagree is a defect by construction, so the pairing is the
//! assertion rather than a convenience.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::germ_pair::{cyl, repose, same_door, seams_off_the_pinch, spin, steinmetz};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn union_err(a: &Body<f64>, b: &Body<f64>) -> BooleanError {
    topo::union(a, b, Tol::witness()).expect_err("this family has no join arm")
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

/// **The row this unit exists for.** Turn each operand about its own
/// axis — which changes neither SURFACE, only where its chart seam
/// falls — and the same two solids reach the JOIN: the crossing layer
/// finds the seam rulings' four wall crossings, splits both operands,
/// and the germ pair that comes out is cylinder × cylinder.
///
/// The join then refuses at the door that names the pinch. That is the
/// honest destination and not a shortfall: the section is two ellipses
/// crossing at two points, a germ-pair FRAME is one conic's centre and
/// axis, and no amount of dispatch work makes a self-crossing pair into
/// one conic. What would serve it is a chord lane that walks a
/// self-intersecting section, on two WALL sides — a lane this tree does
/// not have in any form (its two curved chord lanes both require one
/// member of the pair to be a plane).
#[test]
fn seams_off_the_pinch_reach_the_join_and_name_it() {
    let (a, b) = seams_off_the_pinch(1.2, PI / 4.0);
    let err = union_err(&a, &b);
    assert!(
        matches!(err, BooleanError::GermFrameCylinderPinch { .. }),
        "expected the germ frame's pinch door, got {err:?}"
    );
    // The message must name the crossing axes and stay true for both
    // radius cases (the raise site never compares radii), and end on
    // what the person can do. The pinch points' formula (the UNIT
    // cross product) is the variant's rustdoc, not the sentence.
    let text = format!("{err}");
    for want in [
        "two cylinder walls whose axes cross",
        "whether their radii are equal",
        "Recourse: reshape the parts",
    ] {
        assert!(text.contains(want), "the door must say {want:?}: {text}");
    }
    assert_same_door(
        &err,
        &union_err(&repose(&a), &repose(&b)),
        "the re-posed pose must reach the same door",
    );
}

/// **The family, not one pose.** Every intersecting equal-radius pose
/// this suite can author is run at both scales and every seam angle,
/// against its own re-posed twin. Two things are asserted at once: the
/// answers agree pose for pose (the re-pose row, generalized), and
/// every answer is a TYPED door rather than a wrong body — the family
/// has no union at this head and none of these rows may quietly grow
/// one.
///
/// Every pose, at every height, reaches the join's pinch door: with
/// the seams off the pinch the crossings are found and the sector
/// sides certify, whatever the operand's height.
#[test]
fn every_pose_of_the_family_answers_typed_and_pose_independently() {
    for h in [1.05_f64, 1.2, 1.5, 2.0] {
        for deg in [15.0_f64, 30.0, 45.0, 60.0, 75.0] {
            let (a, b) = seams_off_the_pinch(h, deg.to_radians());
            let err = union_err(&a, &b);
            assert!(
                matches!(err, BooleanError::GermFrameCylinderPinch { .. }),
                "h = {h}, {deg}°: the family must refuse at the pinch door, got {err:?}"
            );
            assert_same_door(
                &err,
                &union_err(&repose(&a), &repose(&b)),
                &format!("h = {h}, {deg}°: the re-posed twin must answer identically"),
            );
        }
    }
}

/// **The differentials the fences promise.** None of the three poses
/// reaches the equal-radius ARM, which is what says that arm is a
/// statement about intersecting equal-radius axes and not about
/// cylinder pairs at large.
///
/// - Unequal radii: the axes still intersect, so the frame dispatch
///   names the pinch door, but with no radius evidence (`None`): the
///   door names neither shape behind those axes and the locus, a space
///   quartic, routes the general rung.
/// - Skew axes: the locus is a space quartic, canal territory; the
///   general rung has not retired, and the dispatch has no arm
///   (`GermFrameUnsupported`). Its verdict on this pose is pinned at
///   both radii by
///   `the_non_parallel_cylinder_pair_splits_on_coplanarity_alone`
///   (`boolean::join`).
/// - Parallel equal radii: the crossing events are a rim CIRCLE against
///   a wall, whose parameters are the roots of a degree-2 trigonometric
///   polynomial. No root lane for that exists anywhere in this tree, so
///   the crossing layer refuses.
#[test]
fn the_fenced_poses_keep_their_own_doors() {
    let a = cyl(1.0, 2.0);

    let unequal = spin(&cyl(0.6, 2.0), Vec3::new(1.0, 0.0, 0.0), PI / 2.0);
    let e = union_err(&a, &unequal);
    assert!(
        matches!(
            e,
            BooleanError::GermFrameCylinderPinch {
                evidence: geom_brep::RadiusEvidence::None,
                ..
            }
        ),
        "unequal radii: the pinch door on the axis relation alone, got {e:?}"
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
        &Affine3::translation(Vec3::new(0.35, 0.0, 0.0)),
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

    // Parallel axes, walls definitely crossing: the rim circle row.
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(1.2, 0.0), 1.0, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -2.0)));
    let parallel = extrude(
        &Profile::new(plane, vec![lp.into()]).validate(tol).unwrap(),
        Extrusion::Distance(4.0),
        tol,
    )
    .unwrap()
    .body;
    let e = union_err(&a, &parallel);
    assert!(
        matches!(e, BooleanError::CurvedPierceUnsupported { .. }),
        "parallel-equal-r: the rim circle has no root lane, got {e:?}"
    );
    assert_same_door(
        &e,
        &union_err(&repose(&a), &repose(&parallel)),
        "parallel-equal-r",
    );
}
