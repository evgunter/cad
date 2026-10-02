//! **A rim circle crossing a cylinder wall**, end to end: the poses the
//! circle × cylinder root lane (`topo::boolean::circle_cylinder`)
//! settles, and the door each one reaches next.
//!
//! - **A crossing of the CARRIER outside the wall's trim builds.** A
//!   D-shaped prism beside a cylinder, its half-round wall turned away:
//!   the cylinder's rim circles cross the prism's wall carrier on the
//!   side the wall face does not cover, and the certified roots place
//!   both crossings outside its trim — no event, where the crossing
//!   layer used to keep its pierce door. Square to the axes (the lane's
//!   square arm) and with the prism tilted (its half-angle
//!   arm), every boolean builds and meters at the closed form.
//! - **A genuine pierce reaches the pierce ring.** Two parallel
//!   equal-radius cylinders staggered in height: each rim circle pierces
//!   the other wall, the lane certifies where, the pierce's sector side
//!   certifies, and the pierced wall face carries the pierce ring, which
//!   joins; the walls' own pair, which meets along rulings, has no join
//!   arm.
//!
//! #347's own poses — one height, coaxial, Steinmetz — are pinned with
//! their doors in `verbs_cylcyl_probe.rs` and `verbs_germarms2.rs`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError, BooleanOp, SweepStrategy};

/// The cylinder of radius `r` about `(cx, cy)`, `z ∈ [z0, z1]`, through
/// the public circle and extrude doors.
fn cyl(cx: f64, cy: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(cx, cy), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

/// The D-shaped prism: the half disc of radius `r` about `(d, 0)` on
/// the `+x` side of its flat `x = d`, `z ∈ [z0, z1]`.
fn d_prism(d: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = bulge_loop(vec![(Point2::new(d, -r), 1.0), (Point2::new(d, r), 0.0)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

/// `b` turned by `deg` about the line through `(0, 0, 1)` along `x`.
fn tilt(b: &Body<f64>, deg: f64) -> Body<f64> {
    let turn = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        deg.to_radians(),
    );
    topo::transform_rigid(b, &turn, Tol::witness()).unwrap()
}

/// The area of the unit disc's segment beyond the chord at `x = a`.
fn segment(a: f64) -> f64 {
    a.acos() - a * (1.0 - a * a).sqrt()
}

fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<Body<f64>, BooleanError> {
    let tol = Tol::witness();
    let out = match op {
        BooleanOp::Union => topo::union(a, b, tol),
        BooleanOp::Intersect => topo::intersect(a, b, tol),
        BooleanOp::Subtract => topo::subtract(a, b, tol),
    }?;
    Ok(out
        .body()
        .unwrap_or_else(|| panic!("{op:?} came back empty"))
        .body
        .clone())
}

/// The datum a refusal past the crossing layer does not carry: how many
/// (edge, face) pairs the sweep accepted an event on with a CIRCLE
/// carrier against a CYLINDER face, in `a`'s edges against `b`'s faces
/// and in `b`'s against `a`'s — the circle × cylinder cell, firing.
fn circle_wall_events(a: &Body<f64>, b: &Body<f64>) -> (usize, usize) {
    let (ab, ba) = topo::sweep_traces(a, b, SweepStrategy::Realized, None, Tol::witness())
        .unwrap_or_else(|e| panic!("the sweep refused: {e:?}"));
    let count = |x: &Body<f64>, y: &Body<f64>, trace: &topo::SweepTrace| {
        trace
            .accepted
            .iter()
            .filter(|(e, f)| {
                let circle = x
                    .get_edge(*e)
                    .and_then(|e| x.get_curve_geom(e.curve))
                    .and_then(topo::CurveGeom::certified)
                    .is_some_and(|c| matches!(c.carrier(), topo::Curve3::Circle { .. }));
                let wall = matches!(
                    y.get_face(*f).and_then(|f| y.get_surface(f.surface)),
                    Some(topo::Surface::Cylinder { .. })
                );
                circle && wall
            })
            .count()
    };
    (count(a, b, &ab), count(b, a, &ba))
}

/// Every validation tier, then the volume against `expected`.
fn assert_body(label: &str, body: &Body<f64>, expected: f64) {
    let tol = Tol::witness();
    assert_eq!(topo::validate(body), Ok(()), "{label}: validate");
    assert_eq!(
        topo::validate_closed(body),
        Ok(()),
        "{label}: validate_closed"
    );
    assert_eq!(
        topo::validate_geometric(body, tol),
        Ok(()),
        "{label}: validate_geometric"
    );
    let v = topo::mass_properties(body, tol)
        .unwrap_or_else(|e| panic!("{label}: mass properties, got {e:?}"))
        .volume;
    assert!(
        (v - expected).abs() <= 1e-9 * expected.max(1.0),
        "{label}: volume {v} against the closed form {expected}"
    );
}

/// **The D-prism beside the cylinder builds under every boolean.** The
/// unit cylinder `A` (`z ∈ [0, 2]`) against a D-prism whose flat
/// `x = d` cuts it and whose half-round wall stands clear of it, tall
/// enough that `A`'s whole segment beyond the flat lies inside the
/// prism: the common part is that segment times `A`'s height, whatever
/// the prism's radius and tilt (each pose is chosen so; the tilts turn
/// the prism about a line along `x`, which keeps its flat in `x = d`).
///
/// `A`'s rim circles cross the prism's wall CARRIER at `x < d`, beside
/// the flat, which the wall face does not cover: the root lane places
/// both crossings outside its trim. Square to the axes that is the
/// lane's square arm, tilted its half-angle quartic.
#[test]
fn a_d_prism_beside_a_cylinder_builds_under_every_boolean() {
    let a = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let va = 2.0 * PI;
    let mut poses = Vec::new();
    for d in [0.2, 0.5, 0.8] {
        let vb = PI / 2.0 * 3.0;
        poses.push((format!("square, d {d}"), d_prism(d, 1.0, -0.5, 2.5), d, vb));
    }
    for (r, deg) in [(1.2, 10.0), (1.4, 30.0), (1.4, 45.0)] {
        let b = tilt(&d_prism(0.5, r, -1.5, 3.5), deg);
        let vb = PI * r * r / 2.0 * 5.0;
        poses.push((format!("tilted {deg}°, r {r}"), b, 0.5, vb));
    }
    for (label, b, d, vb) in &poses {
        let common = segment(*d) * 2.0;
        for (op, expected) in [
            (BooleanOp::Union, va + vb - common),
            (BooleanOp::Subtract, va - common),
            (BooleanOp::Intersect, common),
        ] {
            let label = format!("{label}, {op:?}");
            let body = run(op, &a, b).unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
            assert_body(&label, &body, expected);
        }
    }
}

/// **Two parallel equal-radius cylinders that pierce stop at their
/// walls' pair.** Staggered in height, each rim circle crosses the other
/// wall inside its trim: a pierce, certified by the root lane, whose
/// sector side certifies. The pierced wall face then carries the pierce
/// ring, which joins, at every offset from deep overlap to a thin lens;
/// the two walls meet along rulings, a cylinder × cylinder germ pair the
/// join has no arm for, refused naming A's wall. The sweep's trace is
/// asked which events it took: each operand's rim circles on the
/// other's wall.
#[test]
fn parallel_cylinders_that_pierce_stop_at_the_wall_pair() {
    let a = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    for d in [0.3, 0.8, 1.2, 1.6, 1.9] {
        let b = cyl(d, 0.0, 1.0, 0.5, 2.5);
        let (ab, ba) = circle_wall_events(&a, &b);
        assert!(
            ab > 0 && ba > 0,
            "d {d}: each rim circle meets the other wall: {ab} + {ba}"
        );
        for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
            let err = run(op, &a, &b).expect_err("no join arm for the wall pair");
            assert!(
                matches!(
                    err,
                    BooleanError::CurvedBooleanUnsupported {
                        operand: topo::Operand::A,
                        kind: geom::SurfaceKind::Cylinder,
                        ..
                    }
                ),
                "d {d}, {op:?}: expected the wall pair's door, got {err:?}"
            );
        }
    }
}

/// **A tilted rod through a rim takes the half-angle arm and reaches the
/// germ frame.** A thin rod tilted 30° off the cylinder's axis passes
/// through its top rim: the rim circle against the rod's wall is a
/// degree-2 residual the ladder certifies, the pierce's sector side
/// certifies, and the two walls' axes are skew, a pair the germ-frame
/// dispatch has no arm for.
#[test]
fn a_tilted_rod_through_a_rim_reaches_the_germ_frame() {
    let a = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let turn = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        PI / 6.0,
    );
    let rod = topo::transform_rigid(&cyl(0.0, 0.0, 0.3, -1.5, 1.5), &turn, Tol::witness()).unwrap();
    let rod = topo::transform_rigid(
        &rod,
        &Affine3::translation(Vec3::new(0.95, 0.1, 2.0)),
        Tol::witness(),
    )
    .unwrap();
    let (ab, _) = circle_wall_events(&a, &rod);
    assert!(ab > 0, "the cylinder's rim circle meets the rod's wall");
    for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
        let err = run(op, &a, &rod).expect_err("no germ frame for skew cylinders");
        assert!(
            matches!(
                err,
                BooleanError::GermFrameUnsupported {
                    a_kind: geom::SurfaceKind::Cylinder,
                    b_kind: geom::SurfaceKind::Cylinder,
                    ..
                }
            ),
            "{op:?}: expected the germ-frame door, got {err:?}"
        );
    }
}

/// **A rim circle TANGENT to a parallel wall, or crossing or clearing
/// it by less than the zero band, keeps the pierce door.** Two parallel
/// unit cylinders `2 + δ` apart, staggered, `|δ|` at most half the zero
/// band: each rim circle's extreme residual against the other wall is
/// `δ`. The square arm decides on that exact extreme, puts it in the
/// zero band and escalates to `Uncertain`, and the crossing layer keeps
/// its door, naming a rim circle. The half-angle ladder does not decide
/// on the residual's range and can certify an in-band configuration as a
/// miss (`work/germ/the-half-angle-ladder-certifies-in-band-configurations.md`),
/// so this row is what goes red if circles square to the axis are routed
/// to the ladder.
#[test]
fn a_rim_circle_within_the_band_of_a_parallel_wall_keeps_the_pierce_door() {
    let half_zero = geom_core::Band::linear(Tol::witness()).unwrap().zero() / 2.0;
    let a = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    for delta in [0.0, -half_zero, half_zero] {
        let b = cyl(2.0 + delta, 0.0, 1.0, 0.5, 2.5);
        for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
            let err = run(op, &a, &b).expect_err("an in-band contact is not a crossing");
            let BooleanError::CurvedPierceUnsupported { operand, edge, .. } = &err else {
                panic!("δ {delta}, {op:?}: expected the curved pierce door, got {err:?}");
            };
            let body = if *operand == topo::Operand::A { &a } else { &b };
            let carrier = body
                .get_edge(*edge)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(topo::CurveGeom::certified)
                .map(|c| c.carrier().clone());
            assert!(
                matches!(carrier, Some(topo::Curve3::Circle { .. })),
                "δ {delta}, {op:?}: the refusing edge is a rim circle: {carrier:?}"
            );
        }
    }
}
