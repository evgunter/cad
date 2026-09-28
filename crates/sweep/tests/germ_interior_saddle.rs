//! **Two cylinder walls that meet in a saddle loop interior to both
//! faces, while crossings exist elsewhere.**
//!
//! The interior-loop class of `germ_interior_oval.rs`, on the cylinder.
//! A partial arc face grazes a rod's wall in one closed loop that
//! touches no edge of either body; a bracket's pin pierces the rod's
//! top cap, so the op has crossings and never reaches the no-crossings
//! fallback's `cylinder_extent_gate`. On main before the guard's
//! cylinder half every op came back a VALID `Seamed` body that was
//! wrong: ∩ kept the pin alone and missed the lens. These rows pin the
//! refusal, the no-crossings control, and the pin-only answer the guard
//! must leave standing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::germ_pair::cyl;

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop, test_support::bulge_loop};
use topo::{Body, BooleanOp};

/// A profile in the `yz` plane at `x = x0`, extruded `dist` along `x`.
fn yz_prism(lp: ProfileLoop<f64>, x0: f64, dist: f64) -> Body<f64> {
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_y(), Vec3::unit_z(), Vec3::unit_x()),
        Vec3::new(x0, 0.0, 0.0),
    ));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("the profile validates");
    sweep::extrude(&vp, sweep::Extrusion::Distance(dist), Tol::witness())
        .expect("the profile extrudes")
        .body
}

/// A 240° arc of the circle about `(y, z) = (1.3, 0)`, `r = 0.5`, from
/// `(1.55, ±0.433)` through `y = 0.8`, closed by a rectangle to `y = 2`
/// and extruded over `x ∈ [−3, 3]`. Its cylinder face is PARTIAL, and
/// its line edges sit at `y = 1.55`, clear of the rod.
fn arc_prism() -> Body<f64> {
    let s = 0.75_f64.sqrt() * 0.5;
    let lp = bulge_loop(vec![
        (Point2::new(1.55, s), 3.0_f64.sqrt()),
        (Point2::new(1.55, -s), 0.0),
        (Point2::new(2.0, -s), 0.0),
        (Point2::new(2.0, s), 0.0),
    ]);
    yz_prism(lp, -3.0, 6.0)
}

/// A planar bracket over `x ∈ [−0.1, 0.1]` whose pin (`y ∈ [−0.1, 0.1]`,
/// `z ∈ [1.8, 2.3]`) pierces the rod's top cap — four pierces, the op's
/// only crossings — and whose leg stands outside the rod.
fn bracket() -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(-0.1, 1.8),
        Point2::new(0.1, 1.8),
        Point2::new(0.1, 2.3),
        Point2::new(1.65, 2.3),
        Point2::new(1.65, 0.2),
        Point2::new(1.85, 0.2),
        Point2::new(1.85, 2.5),
        Point2::new(-0.1, 2.5),
    ]);
    yz_prism(lp, -0.1, 0.2)
}

/// The arc prism and the bracket, one solid.
fn arc_bracket() -> Body<f64> {
    topo::union(&arc_prism(), &bracket(), Tol::witness())
        .expect("the fixture's union answers")
        .body()
        .expect("non-empty")
        .body
        .clone()
}

fn volume(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, Tol::witness())
        .expect("the volume integrates")
        .volume
}

fn every_op(
    a: &Body<f64>,
    b: &Body<f64>,
) -> [(
    BooleanOp,
    &'static str,
    Result<topo::BooleanResult<f64>, topo::BooleanError>,
); 4] {
    let t = Tol::witness();
    [
        (BooleanOp::Union, "A ∪ B", topo::union(a, b, t)),
        (BooleanOp::Intersect, "A ∩ B", topo::intersect(a, b, t)),
        (BooleanOp::Subtract, "A ∖ B", topo::subtract(a, b, t)),
        (BooleanOp::Subtract, "B ∖ A", topo::subtract(b, a, t)),
    ]
}

fn refuses_at_the_cylinder_half(a: &Body<f64>, b: &Body<f64>, what: &str) {
    let mut wrong = Vec::new();
    for (op, name, r) in every_op(a, b) {
        match r {
            Err(topo::BooleanError::CurvedPairUnsupported {
                op: Some(o),
                kind: geom_brep::SurfaceKind::Cylinder,
                other_kind: geom_brep::SurfaceKind::Cylinder,
                ..
            }) if o == op => {}
            Err(e) => wrong.push(format!("{name}: refused, but not by the guard: {e:?}")),
            Ok(r) => wrong.push(format!(
                "{name}: answered {:?}, volume {:?}",
                r.body().map(|x| x.kind),
                r.body().map(|x| volume(&x.body))
            )),
        }
    }
    assert!(wrong.is_empty(), "{what}:\n{}", wrong.join("\n"));
}

/// **The live case.** The saddle loop (`y ∈ [0.8, 1]`, `|x| ≤ 0.954`,
/// `|z| ≤ 0.5`) is interior to the rod's wall and to the arc face, and
/// no event marks it. `(0, 0.9, 0)` lies inside both operands; before
/// the guard's cylinder half ∩ came back as the pin alone (`0.008`,
/// against the closed form `0.008 + 0.0820944`), and every other op
/// was wrong by the same lens.
#[test]
fn a_cylinder_saddle_behind_a_pin_refuses_every_op() {
    let (a, b) = (cyl(1.0, 2.0), arc_bracket());
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's band");
    let q = Point3::new(0.0, 0.9, 0.0);
    for (name, body) in [("rod", &a), ("arc bracket", &b)] {
        assert!(
            matches!(
                topo::point_in_solid(body, q, band, Tol::witness()),
                Ok(topo::SolidContainment::In)
            ),
            "the witness point is inside the {name}"
        );
    }
    refuses_at_the_cylinder_half(&a, &b, "rod and arc bracket");
}

/// **The same saddle with the axes crossing at an angle**: the arc
/// bracket rotated `0.15` rad about `y`. Before the guard's cylinder
/// half every op again answered a valid wrong body.
#[test]
fn a_tilted_cylinder_saddle_behind_a_pin_refuses_every_op() {
    let a = cyl(1.0, 2.0);
    let b = topo::transform_rigid(
        &arc_bracket(),
        &Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), 0.15),
        Tol::witness(),
    )
    .expect("the bracket tilts");
    refuses_at_the_cylinder_half(&a, &b, "rod and tilted arc bracket");
}

/// **The no-crossings control.** A full rod at `y = 1.3` meets the wall
/// in the same saddle loop and nowhere else: no crossings, so the op
/// takes the fallback and `cylinder_extent_gate` refuses it there.
#[test]
fn the_saddle_without_a_pin_refuses_at_the_fallback() {
    let a = cyl(1.0, 2.0);
    let rod = topo::transform_rigid(
        &cyl(0.5, 3.0),
        &Affine3::from_parts(
            Affine3::rotation_about_axis(
                Point3::origin(),
                Vec3::unit_y(),
                std::f64::consts::FRAC_PI_2,
            )
            .linear,
            Vec3::new(0.0, 1.3, 0.0),
        ),
        Tol::witness(),
    )
    .expect("the rod moves");
    for (_, name, r) in every_op(&a, &rod) {
        assert!(
            matches!(r, Err(topo::BooleanError::FallbackExtentUnsupported { .. })),
            "rod and full rod, {name}: {r:?}"
        );
    }
}

/// **What the cylinder half must leave answering.** The bracket alone:
/// its pin pierces the rod's top cap and nothing else meets. Its planar
/// faces stand parallel to the rod's axis or cut the carrier in a
/// circle outside themselves, which the plane clause certifies, so ∩ is
/// the pin's cube `0.2³` and ∪ the two volumes less it.
#[test]
fn the_pin_alone_still_answers_its_closed_form() {
    let (a, b) = (cyl(1.0, 2.0), bracket());
    let pin = 0.2_f64.powi(3);
    let (va, vb) = (volume(&a), volume(&b));
    for (_, name, r) in every_op(&a, &b) {
        let r = r.unwrap_or_else(|e| panic!("{name}: {e:?}"));
        let body = &r.body().expect("non-empty").body;
        assert_eq!(
            topo::validate_geometric(body, Tol::witness()),
            Ok(()),
            "{name}"
        );
        let want = match name {
            "A ∪ B" => va + vb - pin,
            "A ∩ B" => pin,
            "A ∖ B" => va - pin,
            _ => vb - pin,
        };
        let got = volume(body);
        assert!(
            (got - want).abs() <= 1e-9 * want.max(1.0),
            "{name}: {got} against the closed form {want}"
        );
    }
}
