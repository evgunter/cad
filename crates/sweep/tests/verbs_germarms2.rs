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
//!   one conic; the frame dispatch is keyed on surface KINDS alone, so
//!   it has no point with which to select a branch either. It refuses
//!   typed at a door that names the pinch. Walking the section across
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

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

/// A cylinder about `z`, radius `r`, `z ∈ [−h, h]`, through the public
/// extrude door.
fn cyl(r: f64, h: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -h)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(2.0 * h), tol)
        .unwrap()
        .body
}

fn spin(b: &Body<f64>, axis: Vec3<f64>, angle: f64) -> Body<f64> {
    topo::transform_rigid(
        b,
        &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
        Tol::witness(),
    )
    .unwrap()
}

/// **The re-pose**: a rotation about `(1,2,3)` by 0.7 rad followed by a
/// translation off every axis plane. Nothing about the configuration
/// changes — the same two solids, the same contacts — so every row's
/// re-posed twin must answer exactly what its direct copy answers.
fn repose(b: &Body<f64>) -> Body<f64> {
    let r = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        0.7,
    );
    topo::transform_rigid(
        b,
        &Affine3::from_parts(r.linear, r.translation + Vec3::new(0.3, -0.45, 0.6)),
        Tol::witness(),
    )
    .unwrap()
}

/// The classic Steinmetz pair: equal radii, perpendicular intersecting
/// axes, both seams on the pinch points.
fn steinmetz(h: f64) -> (Body<f64>, Body<f64>) {
    (
        cyl(1.0, h),
        spin(&cyl(1.0, h), Vec3::new(1.0, 0.0, 0.0), PI / 2.0),
    )
}

/// The same SURFACES with both seams turned off the pinch: each
/// operand is spun about its own axis, which a cylinder of revolution
/// is invariant under. Only the charts move.
fn seams_off_the_pinch(h: f64, phi: f64) -> (Body<f64>, Body<f64>) {
    let (a, b) = steinmetz(h);
    (
        spin(&a, Vec3::new(0.0, 0.0, 1.0), phi),
        spin(&b, Vec3::new(0.0, 1.0, 0.0), phi),
    )
}

fn union_err(a: &Body<f64>, b: &Body<f64>) -> BooleanError {
    topo::union(a, b, Tol::witness()).expect_err("this family has no join arm")
}

/// A one-line discriminant of a refusal: the variant plus the keys it
/// names. Two poses of the same configuration must produce the same
/// string.
fn door(e: &BooleanError) -> String {
    format!("{e:?}")
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
    assert_eq!(
        door(&union_err(&a, &b)),
        door(&union_err(&repose(&a), &repose(&b))),
        "the re-posed Steinmetz pair must answer exactly what the direct-extruded one does"
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
    // The message must carry the pinch, not merely the kind pair: this
    // refusal is the unit's whole finding and a reader who only sees
    // the variant learns nothing.
    let text = format!("{err}");
    // The pinned formula is the UNIT one. `‖a1 x a2‖ = sin θ`, so the
    // raw cross product names the pinch points only on the
    // perpendicular pose — at 20° on a 0.9 m fixture it is 0.59 m off.
    // A message that wrote the raw product would be a wrong recipe
    // handed to the reader, so the string is pinned, not paraphrased.
    for want in ["pinch", "p ± r·n̂ where n̂ = unit(a1 x a2)", "never inferred"] {
        assert!(text.contains(want), "the door must say {want:?}: {text}");
    }
    assert_eq!(
        door(&err),
        door(&union_err(&repose(&a), &repose(&b))),
        "the re-posed pose must reach the same door"
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
/// The doors themselves differ by pose and are recorded rather than
/// forced: with the seams off the pinch and the operands short enough
/// for the sector-side curvature charge, a pose reaches the join's
/// pinch door; a taller operand's pierce fragments outrun that charge
/// and stop one layer earlier, at `CurvedSectorSideUnsupported`, whose
/// recourse is the second-order sector trilean and not an arm.
#[test]
fn every_pose_of_the_family_answers_typed_and_pose_independently() {
    let mut reached_the_join = 0;
    for h in [1.05_f64, 1.2, 1.5, 2.0] {
        for deg in [15.0_f64, 30.0, 45.0, 60.0, 75.0] {
            let (a, b) = seams_off_the_pinch(h, deg.to_radians());
            let err = union_err(&a, &b);
            assert!(
                matches!(
                    err,
                    BooleanError::GermFrameCylinderPinch { .. }
                        | BooleanError::CurvedSectorSideUnsupported { .. }
                ),
                "h = {h}, {deg}°: the family must refuse typed, got {err:?}"
            );
            if matches!(err, BooleanError::GermFrameCylinderPinch { .. }) {
                reached_the_join += 1;
            }
            assert_eq!(
                door(&err),
                door(&union_err(&repose(&a), &repose(&b))),
                "h = {h}, {deg}°: the re-posed twin must answer identically"
            );
        }
    }
    // The sweep must actually exercise the join door rather than only
    // the earlier one. HOW MANY poses do is a fixture property that
    // moves with the tolerance row — the sector-side curvature charge
    // is what decides it — so the row asserts that the door is reached,
    // not a count.
    assert!(
        reached_the_join >= 1,
        "no pose of the family reached the join door"
    );
}

/// **The differentials the fences promise.** None of the three poses
/// reaches a JOIN door at all, so none of them can inherit the pinch
/// door — which is what says the new arm is a statement about
/// intersecting equal-radius axes and not about cylinder pairs at
/// large.
///
/// - Unequal radii: no equal-radius section exists, and the germ pair
///   the join would need is never minted.
/// - Skew axes: the locus is a space quartic, canal territory; the
///   general rung has not retired. The dispatch's own verdict on this
///   pose is pinned exactly, at both radii, by
///   `the_non_parallel_cylinder_pair_splits_on_coplanarity_alone`
///   (`boolean::join`) — this row's job is that the pose never gets
///   that far.
/// - Parallel equal radii: the crossing events are a rim CIRCLE against
///   a wall, whose parameters are the roots of a degree-2 trigonometric
///   polynomial. No root lane for that exists anywhere in this tree, so
///   this row is untouched by this unit and says so.
///
/// **Which crossing-layer door each pose takes is not the assertion.**
/// A pierce that is never found and a pierce whose sector sides cannot
/// be certified against the wall's curvature are both the crossing
/// layer refusing, and which of the two a pose lands on moves with its
/// lever arms and with the tolerance row. Pinning the exact variant
/// here would be pinning the fixture, not the fence.
#[test]
fn the_fenced_poses_keep_their_own_doors() {
    // Both crossing-layer doors, and neither is a join door.
    fn short_of_the_join(name: &str, e: &BooleanError) {
        assert!(
            matches!(
                e,
                BooleanError::CurvedPierceUnsupported { .. }
                    | BooleanError::CurvedSectorSideUnsupported { .. }
            ),
            "{name}: expected a crossing-layer door, got {e:?}"
        );
    }

    let a = cyl(1.0, 2.0);

    let unequal = spin(&cyl(0.6, 2.0), Vec3::new(1.0, 0.0, 0.0), PI / 2.0);
    let e = union_err(&a, &unequal);
    short_of_the_join("unequal radii", &e);
    assert_eq!(door(&e), door(&union_err(&repose(&a), &repose(&unequal))));

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
    short_of_the_join("skew axes", &e);
    assert_eq!(door(&e), door(&union_err(&repose(&a), &repose(&skew))));

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
    short_of_the_join("parallel-equal-r", &e);
    assert_eq!(door(&e), door(&union_err(&repose(&a), &repose(&parallel))));
}
