//! **Two cylinder walls that meet in a saddle loop interior to both
//! faces, while crossings exist elsewhere.**
//!
//! The interior-loop class of `germ_interior_oval.rs`, on the cylinder.
//! A partial arc face grazes a rod's wall in one closed loop that
//! touches no edge of either body; a bracket's pin pierces the rod's
//! top cap, so the op has crossings and never reaches the no-crossings
//! fallback. Before the interior-loop guard covered the cylinder, every
//! op came back a VALID `Seamed` body that was wrong: ∩ kept the pin
//! alone and missed the lens. The section certificate classifies the
//! pair as the cylinder pair table's middle row — one null saddle loop
//! — whose witness lies strictly inside both faces with no event on the
//! pair: a certified interior loop (R-loop). These rows pin the
//! refusal, its payload and verdict, the no-crossings control, and the
//! pin-only answer the guard must leave standing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::germ_pair::cyl;
use sweep::ExtrudeSide;

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop, test_support::bulge_loop};
use sweep::test_support::finished;
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
    sweep::extrude(
        &vp,
        sweep::Extrusion::Distance {
            depth: dist,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
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
    let t = Tol::witness();
    let prism = finished("the arc prism", arc_prism(), t);
    let bracket = finished("the bracket", bracket(), t);
    topo::union(&prism, &bracket, t)
        .expect("the fixture's union answers")
        .body()
        .expect("non-empty")
        .body
        .clone()
        .into_body()
}

fn volume(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, Tol::witness())
        .expect("the volume integrates")
        .volume
}

/// One op's row: the op, its name, and what it returned.
type OpRow = (
    BooleanOp,
    &'static str,
    Result<topo::BooleanResult<f64>, topo::BooleanError>,
);

/// Every op on `a` and `b`, each finished once as an operand.
fn every_op(a: &Body<f64>, b: &Body<f64>) -> [OpRow; 4] {
    let t = Tol::witness();
    let a = &finished("operand A", a.clone(), t);
    let b = &finished("operand B", b.clone(), t);
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
                site: topo::PairRefusalSite::InteriorLoopGuard,
                kind: geom::SurfaceKind::Cylinder,
                other_kind: geom::SurfaceKind::Cylinder,
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
/// takes the fallback, and its section pass refuses the loop there as
/// R-loop. So does the arc prism alone, without its bracket.
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
    for (what, b) in [("full rod", rod), ("arc prism", arc_prism())] {
        for (_, name, r) in every_op(&a, &b) {
            match r {
                Err(topo::BooleanError::FallbackExtentUnsupported { what: why, .. }) => {
                    assert!(
                        why.contains("closed loop interior to both faces"),
                        "rod and {what}, {name}: {why}"
                    );
                }
                other => panic!("rod and {what}, {name}: {other:?}"),
            }
        }
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

fn cylinder_faces(b: &Body<f64>) -> Vec<topo::FaceKey> {
    b.faces()
        .filter(|(_, fd)| {
            matches!(
                b.get_surface(fd.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// **The refusal names both walls, and the certificate's verdict is a
/// certified interior loop.** Every op's payload names the rod's wall
/// and the arc face, from its first operand's side, and the crossings
/// path's per-pair verdict for that pair is R-loop. Red against the
/// middle row read as two thin-essential loops (W2 would clear it on the
/// arc face), against the no-event decision answering `In`-both as
/// clear, and against W4 reading the op's events rather than the
/// pair's.
#[test]
fn the_saddle_is_a_certified_interior_loop_naming_both_walls() {
    let tilted = topo::transform_rigid(
        &arc_bracket(),
        &Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), 0.15),
        Tol::witness(),
    )
    .expect("the bracket tilts");
    for (what, b) in [("straight", arc_bracket()), ("tilted", tilted)] {
        let a = cyl(1.0, 2.0);
        let (wall_a, wall_b) = (cylinder_faces(&a), cylinder_faces(&b));
        assert_eq!(wall_b.len(), 1, "{what}: the bracket's one arc face");
        for (op, name, r) in every_op(&a, &b) {
            let a_first = !name.starts_with('B');
            let (first, second) = if a_first {
                (&wall_a, &wall_b)
            } else {
                (&wall_b, &wall_a)
            };
            match r {
                Err(topo::BooleanError::CurvedPairUnsupported {
                    op: Some(o),
                    site: topo::PairRefusalSite::InteriorLoopGuard,
                    operand: topo::Operand::A,
                    face,
                    other_face,
                    ..
                }) => {
                    assert_eq!(o, op, "{what}, {name}");
                    assert!(
                        first.contains(&face) && second.contains(&other_face),
                        "{what}, {name}: names {face:?} × {other_face:?}"
                    );
                }
                other => panic!("{what}, {name}: {other:?}"),
            }
        }
        let verdicts = topo::test_support::section_report(BooleanOp::Union, &a, &b, Tol::witness())
            .expect("the reduction runs");
        assert!(
            verdicts
                .iter()
                .any(|(fa, fb, v)| wall_a.contains(fa) && *fb == wall_b[0] && v == "Err(Loop)"),
            "{what}: {verdicts:?}"
        );
    }
}
