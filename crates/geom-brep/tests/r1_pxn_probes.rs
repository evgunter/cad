//! R1 adversarial probes for the M7-8 plane × NURBS edge lane.
//!
//! Attack surface: the between-samples envelope (a wiggle vanishing at
//! the whole sample schedule), the refusal boundary (smallest catching
//! displacement, off-plane and drift directions), the envelope's
//! soundness (certified sup vs dense-sampled true sup), and the
//! transversality boundary (shallower than the planted tangential).
//!
//! **ADOPTED INTO THE SHIPPED SUITE** (R1 MAJOR-1). These began as a
//! reviewer's private probes and are kept as permanent rows because
//! they are the ONLY tests that observe the unit's headline
//! obligation: with the chart-sup decision mutated away (zero the sup
//! before its `decide`), every other M7-8 row stays green at default
//! AND at 1e-12, while
//! [`a_wiggle_vanishing_at_the_whole_schedule_must_refuse`] and
//! [`the_certified_sup_bounds_the_dense_sampled_true_sup`] go red —
//! measured, both directions. The between-samples envelope is what
//! makes a declared carrier certifiable at all, so a refactor that
//! hollows it out must not be able to leave the suite green.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeNurbsLane, PlaneNurbsRefusal};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tolerance, Vec3};
use geom_core::Tol;

fn quarter_cylinder_wall() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 1.0),
    ];
    let w = core::f64::consts::FRAC_1_SQRT_2;
    NurbsSurface::new(ku, kv, control, vec![1.0, 1.0, w, w, 1.0, 1.0]).unwrap()
}

fn transverse_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn segment(a: Point3<f64>, b: Point3<f64>) -> NurbsCurve3<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsCurve3::new(k, vec![a, b], vec![1.0, 1.0]).unwrap()
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A spline carrier that interpolates `(1 + a·sin(32πt), 0, t)` at 257
/// chord points: exactly ON both surfaces at every one of the lane's
/// 33 schedule samples (t = k/32, where sin vanishes to ~1e-15), and
/// displaced by ~`a` radially (in-plane, off-wall) between them.
fn wiggle_carrier(a: f64) -> NurbsCurve3<f64> {
    let n = 257;
    let pts: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            let t = f64::from(i) / f64::from(n - 1);
            Point3::new(
                (32.0 * core::f64::consts::PI * t).sin().mul_add(a, 1.0),
                0.0,
                t,
            )
        })
        .collect();
    let params: Vec<f64> = (0..n).map(|i| f64::from(i) / f64::from(n - 1)).collect();
    NurbsCurve3::interpolate_with_params(&pts, 3, &params).unwrap()
}

/// Dense-sampled true residual of `c` against the wall+plane pair.
fn dense_true_sup(c: &NurbsCurve3<f64>, wall: &NurbsSurface<f64>) -> f64 {
    let mut sup: f64 = 0.0;
    for i in 0..=4096 {
        let t = f64::from(i) / 4096.0;
        let p = c.eval(t);
        let plane_res = p.y.abs();
        let wall_res = match wall.project(p) {
            Ok(pr) => {
                let q = wall.eval(pr.u, pr.v);
                p.distance(q)
            }
            Err(e) => e.last_distance,
        };
        sup = sup.max(plane_res.max(wall_res));
    }
    sup
}

/// ATTACK 1: on-schedule, off-between-samples. If this certifies, the
/// envelope is grid-only and the lane is broken.
#[test]
fn a_wiggle_vanishing_at_the_whole_schedule_must_refuse() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    let a = 1e-3;
    let carrier = wiggle_carrier(a);
    let truth = dense_true_sup(&carrier, &wall);
    match f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band()) {
        Ok(limbs) => panic!(
            "GRID-ONLY HOLE: a carrier displaced {truth:e} m between samples \
             certified with hull_sup {:e}",
            limbs.hull_sup
        ),
        Err(e) => {
            println!("R1 wiggle a={a:e}: true sup {truth:e} -> refused: {e:?}");
            if let PlaneNurbsRefusal::Limb { value, .. } = e {
                assert!(
                    value >= truth * 0.5,
                    "the certified bound must dominate the true displacement: \
                     {value:e} < {truth:e}"
                );
            }
        }
    }
}

/// ATTACK 1c: the PLANE-side envelope in isolation — a carrier
/// wiggling off-plane (+y) between samples with amplitude 1e-5, whose
/// wall-side residual is only ~A²/2 ≈ 5e-11 (inside ε). Only the
/// analytic operand's between-samples bound can catch it.
#[test]
fn an_off_plane_wiggle_must_refuse_via_the_plane_envelope() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    let a = 1e-5;
    let n = 257;
    let pts: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            let t = f64::from(i) / f64::from(n - 1);
            Point3::new(1.0, a * (32.0 * core::f64::consts::PI * t).sin(), t)
        })
        .collect();
    let params: Vec<f64> = (0..n).map(|i| f64::from(i) / f64::from(n - 1)).collect();
    let carrier = NurbsCurve3::interpolate_with_params(&pts, 3, &params).unwrap();
    let truth = dense_true_sup(&carrier, &wall);
    match f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band()) {
        Ok(limbs) => panic!(
            "PLANE-SIDE GRID-ONLY HOLE: off-plane-between-samples carrier \
             (true sup {truth:e}) certified with hull_sup {:e}",
            limbs.hull_sup
        ),
        Err(e) => println!("R1 off-plane wiggle a={a:e}: true sup {truth:e} -> {e:?}"),
    }
}

/// ATTACK 1b: envelope soundness on the ACCEPT side — a certifying
/// carrier's certified sup must bound its dense-sampled true sup.
#[test]
fn the_certified_sup_bounds_the_dense_sampled_true_sup() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    for a in [0.0, 1e-13, 1e-12] {
        let carrier = if a == 0.0 {
            segment(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0))
        } else {
            wiggle_carrier(a)
        };
        let truth = dense_true_sup(&carrier, &wall);
        match f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band()) {
            Ok(limbs) => {
                println!(
                    "R1 sup-bound a={a:e}: certified {:e} vs true {truth:e}",
                    limbs.hull_sup
                );
                assert!(
                    limbs.hull_sup >= truth * 0.99,
                    "UNSOUND: certified sup {:e} below the true sup {truth:e}",
                    limbs.hull_sup
                );
            }
            Err(e) => println!("R1 sup-bound a={a:e}: refused {e:?} (true {truth:e})"),
        }
    }
}

/// ATTACK 2: the refusal boundary — uniform radial (in-plane) and
/// off-plane displacements, scanned down from 1e-3. Every outcome must
/// be typed; refusals must carry a bound of the displacement's order.
#[test]
fn displacement_scan_finds_the_refusal_boundary_typed() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    let eps = Tol::witness().get().eps;
    for dir in ["radial(+x, on-plane)", "off-plane(+y)"] {
        let mut smallest_refused = f64::INFINITY;
        let mut largest_certified: f64 = 0.0;
        for k in 3..14 {
            let d = 10f64.powi(-k);
            let carrier = if dir.starts_with("radial") {
                segment(
                    Point3::new(1.0 + d, 0.0, 0.0),
                    Point3::new(1.0 + d, 0.0, 1.0),
                )
            } else {
                segment(Point3::new(1.0, d, 0.0), Point3::new(1.0, d, 1.0))
            };
            match f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band()) {
                Ok(l) => {
                    largest_certified = largest_certified.max(d);
                    println!("R1 scan {dir} d={d:e}: CERTIFIES hull {:e}", l.hull_sup);
                    assert!(
                        d <= eps * 1.01,
                        "a displacement past ε certified: d={d:e} eps={eps:e}"
                    );
                }
                Err(PlaneNurbsRefusal::Limb { limb, value }) => {
                    smallest_refused = smallest_refused.min(d);
                    println!("R1 scan {dir} d={d:e}: Limb {} = {value:e}", limb.name());
                    assert!(value > 0.1 * d, "bound must be of the displacement's order");
                }
                Err(e) => println!("R1 scan {dir} d={d:e}: {e:?}"),
            }
        }
        println!(
            "R1 scan {dir}: smallest refused {smallest_refused:e}, \
             largest certified {largest_certified:e}"
        );
    }
}

/// ATTACK 2b: tangential drift ALONG the intersection — a subsegment
/// of the true locus. The LANE certifies it (it is on both surfaces);
/// the segment-identity duty belongs to the door's endpoint checks.
/// Recorded so the division of labour is explicit.
#[test]
fn a_drifted_subsegment_of_the_true_locus_certifies_at_the_lane() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    let carrier = segment(Point3::new(1.0, 0.0, 0.2), Point3::new(1.0, 0.0, 0.9));
    let r = f64::plane_nurbs_limbs(&carrier, &plane, &wall, 0.7, band());
    println!("R1 drift subsegment: {r:?}");
    assert!(r.is_ok(), "a true-locus subsegment is on both surfaces");
}

/// ATTACK 4: the transversality boundary, shallower than the planted
/// 0° tangential — a plane through the ruling tilted α off the tangent
/// plane. Every outcome must be typed; no panic, no silent accept in
/// the sub-ε regime.
#[test]
fn near_tangential_scan_is_typed_at_every_angle() {
    let wall = quarter_cylinder_wall();
    let carrier = segment(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0));
    let eps = Tol::witness().get().eps;
    for k in [1, 2, 3, 6, 8, 9, 10, 12] {
        let alpha = 10f64.powi(-k);
        // Normal (cos α, sin α, 0), plane through (1,0,0) containing z:
        // sin θ against the wall's radial normal is sin α.
        let plane = Surface::Plane {
            origin: Point3::new(1.0, 0.0, 0.0),
            normal: Vec3::new(alpha.cos(), alpha.sin(), 0.0),
            u_ref: Vec3::new(0.0, 0.0, 1.0),
        };
        match f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band()) {
            Ok(l) => {
                println!(
                    "R1 tangency a={alpha:e}: certifies, sin={:e}",
                    l.min_sin_theta
                );
                assert!(
                    alpha.sin() > eps,
                    "sub-ε transversality silently accepted at α={alpha:e}"
                );
            }
            Err(e) => println!("R1 tangency a={alpha:e}: {e:?}"),
        }
    }
}
