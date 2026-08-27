//! **Adopted blinded-review probes for M5 S12** (S11 precedent: the
//! review's adversarial constructions become permanent guards). Four
//! class-boundary attacks on the newly per-class door, plus involution
//! on a body the boolean itself built:
//!
//! - the TORUS gate: `union` has no per-class door of its own, so the
//!   torus must refuse at `reduce::gate_operand_pairs` — this is what scopes
//!   the containment-fallback finding to the sphere class;
//! - the RADIAL POKE: a cylinder wall escaping between its own seam
//!   vertices, the shape that defeats a vertex probe — exact or typed,
//!   never silent;
//! - the HALF-BURIED LOG: the finding's ball shape rebuilt from a
//!   cylinder, to ask whether the OPENED class inherits the silence;
//! - the CONTAINED BOSS: the no-crossings fallback made newly reachable
//!   by opening ∖/∩, where the vertex probe must be sound;
//! - INVOLUTION on a boolean RESULT (mixed senses, both encodings
//!   present in one body).
//!
//! The probes are adopted VERBATIM from the review; only this header
//! and the lint allowance are the adopter's. Each answering row asserts
//! its closed form, so a refusal is honest and a wrong number fails.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Affine3, Point2, Point3, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use std::f64::consts::PI;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::boolean::{BooleanDeclarations, BooleanOp, boolean_op_with};
use topo::{Body, SweepStrategy};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn vol(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness()).unwrap().volume
}

fn boxy(x0: f64, y0: f64, w: f64, h: f64, z0: f64, t: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        [(x0, y0), (x0 + w, y0), (x0 + w, y0 + h), (x0, y0 + h)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(t), Tol::witness())
        .unwrap()
        .body
}

/// 2-arc disc radius `r` centred at origin with seam vertices at
/// angles `phi` and `phi + pi`, extruded z0..z0+len.
fn disc2(r: f64, phi: f64, z0: f64, len: f64) -> Body<f64> {
    let at = |th: f64| p2(r * th.cos(), r * th.sin());
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(phi), 1.0),
        ProfileVertex::new(at(phi + PI), 1.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(len), Tol::witness())
        .unwrap()
        .body
}

/// PROBE 1 (charter A3): a TORUS operand. Union has no per-class door;
/// the gate must refuse it typed (never a number). If it answers, the
/// answer is checked and a silent lie is flagged.
#[test]
fn probe_torus_union_is_never_silently_wrong() {
    // Circle profile centred (1.5, 0) radius 0.4, revolved about y.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(1.1, 0.0), 1.0),
        ProfileVertex::new(p2(1.9, 0.0), 1.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    let torus = revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    let slab = boxy(-3.0, -3.0, 6.0, 6.0, -0.1, 0.2);
    for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
        match boolean_op_with(
            op,
            &slab,
            &torus,
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        ) {
            Err(e) => println!("torus {op:?}: typed refusal: {e:?}"),
            Ok(out) => {
                let v = vol(&out.body().unwrap().body);
                panic!("torus {op:?} ANSWERED: {v} — check for silence");
            }
        }
    }
}

/// PROBE 2 (charter A3, "wall poking out between the seams"): a 2-arc
/// cylinder radius 1.05 with seams on the box diagonals, inside a
/// 2x2x1 box; the wall pokes out all four sides between vertices, cap
/// circles cross the side faces. Every op must be exact or refuse
/// typed.
#[test]
fn probe_cylinder_radial_poke_is_exact_or_typed() {
    let r: f64 = 1.05;
    let a = boxy(-1.0, -1.0, 2.0, 2.0, 0.0, 1.0);
    let b = disc2(r, PI / 4.0, 0.2, 0.6);
    let alpha = (1.0 / r).acos();
    let cap = r * r * (alpha - alpha.sin() * alpha.cos()); // one side's escape segment
    let lens_area = PI * r * r - 4.0 * cap;
    let v_a = 4.0;
    let v_b = PI * r * r * 0.6;
    let v_meet = lens_area * 0.6;
    for (op, expect) in [
        (BooleanOp::Union, v_a + v_b - v_meet),
        (BooleanOp::Subtract, v_a - v_meet),
        (BooleanOp::Intersect, v_meet),
    ] {
        match boolean_op_with(
            op,
            &a,
            &b,
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        ) {
            Err(e) => println!("radial poke {op:?}: typed refusal: {e:?}"),
            Ok(out) => {
                let v = vol(&out.body().unwrap().body);
                assert!(
                    (v - expect).abs() < 1e-9,
                    "radial poke {op:?} SILENTLY WRONG: {v} vs {expect}"
                );
                println!("radial poke {op:?}: exact ({v})");
            }
        }
    }
}

/// PROBE 3 (charter A3, "endcap-wise"): a horizontal log half-buried
/// in a slab exactly like the finding's ball — wall leaves through
/// top AND bottom — but a cylinder's boundary circles must poke with
/// it. Exact or typed; silence is the MAJOR.
#[test]
fn probe_horizontal_log_halfburied_is_exact_or_typed() {
    let slab = boxy(0.0, 0.0, 4.0, 4.0, 0.0, 1.0);
    // Vertical 2-arc cylinder r=0.7, then rotate about x so the axis
    // runs along y at z = 0.5, spanning y in [0.5, 3.5].
    let log0 = disc2(0.7, 0.0, 0.0, 3.0);
    let rot = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        -PI / 2.0,
    );
    let log1 = topo::transform_rigid(&log0, &rot, Tol::witness()).unwrap();
    let log = topo::transform_rigid(
        &log1,
        &Affine3::translation(Vec3::new(2.0, 0.5, 0.5)),
        Tol::witness(),
    )
    .unwrap();
    let r: f64 = 0.7;
    let beta = (0.5 / r).acos();
    let seg = r * r * (beta - beta.sin() * beta.cos()); // area beyond each slab plane
    let meet_area = PI * r * r - 2.0 * seg;
    let v_meet = meet_area * 3.0;
    let v_a = 16.0;
    let v_b = PI * r * r * 3.0;
    for (op, expect) in [
        (BooleanOp::Union, v_a + v_b - v_meet),
        (BooleanOp::Subtract, v_a - v_meet),
        (BooleanOp::Intersect, v_meet),
    ] {
        match boolean_op_with(
            op,
            &slab,
            &log,
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        ) {
            Err(e) => println!("log {op:?}: typed refusal: {e:?}"),
            Ok(out) => {
                let v = vol(&out.body().unwrap().body);
                assert!(
                    (v - expect).abs() < 1e-9,
                    "log {op:?} SILENTLY WRONG: {v} vs {expect}"
                );
                println!("log {op:?}: exact ({v})");
            }
        }
    }
}

/// PROBE 5 (charter A3): the opened door makes the no-crossings
/// FALLBACK newly reachable for cylinder operands on subtract and
/// intersect (a wholly-contained boss). The vertex probe must be
/// SOUND here: the boss's seam vertices are honest witnesses because
/// a contained-cylinder verdict cannot be defeated between vertices
/// without a surface crossing, which the frontier door catches.
#[test]
fn probe_contained_cylinder_reaches_the_fallback_soundly() {
    let a = boxy(0.0, 0.0, 3.0, 3.0, 0.0, 1.0);
    let b = disc2(0.3, 0.0, 0.3, 0.4); // wholly interior at (0,0)?? centred origin — move it
    let b = topo::transform_rigid(
        &b,
        &Affine3::translation(Vec3::new(1.5, 1.5, 0.0)),
        Tol::witness(),
    )
    .unwrap();
    for (op, expect) in [
        (BooleanOp::Intersect, PI * 0.09 * 0.4),
        (BooleanOp::Subtract, 9.0 - PI * 0.09 * 0.4),
        (BooleanOp::Union, 9.0),
    ] {
        match boolean_op_with(
            op,
            &a,
            &b,
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        ) {
            Err(e) => println!("contained boss {op:?}: typed refusal: {e:?}"),
            Ok(out) => {
                let v = vol(&out.body().unwrap().body);
                assert!(
                    (v - expect).abs() < 1e-9,
                    "contained boss {op:?} SILENTLY WRONG: {v} vs {expect}"
                );
                println!("contained boss {op:?}: exact ({v})");
            }
        }
    }
}

/// PROBE 4 (charter B): involution on an ASYMMETRIC mixed body that a
/// boolean built (planar caps + convex boss wall + concave hole wall,
/// mixed senses) — both encodings must each flip exactly once, twice.
#[test]
fn probe_involution_on_a_boolean_result_body() {
    let plate = boxy(0.0, 0.0, 3.0, 3.0, 0.0, 0.8);
    let boss = {
        let at = |th: f64| p2(1.2 + 0.35 * th.cos(), 1.7 + 0.35 * th.sin());
        let lp = ProfileLoop::new(
            (0..3)
                .map(|i| ProfileVertex::new(at(2.0 * PI * i as f64 / 3.0), (PI / 6.0).tan()))
                .collect(),
        );
        let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.3)));
        let profile = Profile::new(plane, vec![lp])
            .validate(Tol::witness())
            .unwrap();
        extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
            .unwrap()
            .body
    };
    let holed = topo::subtract(&plate, &boss, Tol::witness())
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    let senses: Vec<bool> = holed.faces().map(|(_, f)| f.sense).collect();
    assert!(
        senses.contains(&false),
        "blind hole wall should be sense: false"
    );
    let v = vol(&holed);
    let original = format!("{holed:?}");
    let rev = holed.revert().unwrap();
    assert_eq!(vol(&rev).to_bits(), (-v).to_bits(), "volume bit-negated");
    assert_eq!(
        topo::validate_geometric(&rev, Tol::witness()),
        Err(vec![topo::ValidationError::NegativeVolume])
    );
    assert_eq!(
        format!("{:?}", rev.revert().unwrap()),
        original,
        "bitwise involution"
    );
}
