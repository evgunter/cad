//! The curved pierce/split substrate's own rows: what the planar
//! point-in-face walk does with a boundary it cannot polygonize.
//!
//! A cylinder cap is bounded by two rim semicircles and has TWO
//! vertices, so the ray-parity walk's polygon through them is the
//! disc's diameter — a segment of zero area. Every interior point of
//! the cap therefore read `Out`, and the chord rows above that walk
//! read every point ON the diameter as if it lay on the rim edge. The
//! rows here are the consequences, metered rather than asserted:
//! `is_ok` would have passed on the wrong body that motivated them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn cyl(r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(p2(0.0, 0.0), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

fn boxx(x0: f64, x1: f64, y0: f64, y1: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp: ProfileLoop<f64> = RawLoop::polygon([p2(x0, y0), p2(x1, y0), p2(x1, y1), p2(x0, y1)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

/// **The wrong answer this substrate closes.** A box driven up through
/// a cylinder's cap crosses that cap in four places, all strictly
/// inside the disc and none of them on its two-vertex polygon. The
/// walk found no event at all, the operands read as disjoint, and the
/// union came back OK with the overlap counted TWICE.
///
/// The yardstick is kept because it is what makes the row falsifiable.
/// `7.003185307179585` was the answer and `6.643185307179586` is the
/// truth — the difference is exactly the buried 0.36 of box. Both are
/// MEASURED values, not closed forms: `π·1²·2 + 0.6·0.6·2` evaluated
/// in `f64` differs from the first in its last bit, and quoting the
/// expression as if it produced the literal would be quoting a number
/// this kernel never returned.
///
/// It is a typed refusal now, from the JOIN layer: the events are
/// found, both bodies are split, and what has no arm is the join of a
/// pierce ring in an arc-bounded planar face. A refusal is not the
/// answer this case deserves, but it is honest, and the wrong volume
/// is gone.
#[test]
fn a_box_driven_through_a_cap_no_longer_unions_as_two_disjoint_solids() {
    let tol = Tol::witness();
    let a = cyl(1.0, 0.0, 2.0);
    let b = boxx(-0.3, 0.3, -0.3, 0.3, 1.0, 3.0);
    let err = match topo::union(&a, &b, tol) {
        Err(e) => e,
        Ok(topo::BooleanResult::Body(out)) => {
            let v = topo::mass_properties(&out.body, tol).unwrap().volume;
            panic!(
                "the cap crossing must not be silent: volume {v} \
                 (the old wrong answer was 7.003185307179585, truth \
                 6.643185307179586)"
            );
        }
        Ok(other) => panic!("expected one solid or a typed refusal, got {other:?}"),
    };
    assert!(
        matches!(err, BooleanError::Join(_)),
        "the crossing layer passes it; the join layer owns what is left: {err:?}"
    );
}

/// The disc row's OUT direction, at the same reach: a box standing
/// clear of the disc crosses the cap's PLANE outside its boundary, and
/// that must mint no event — the operands stay disjoint and the
/// volumes add. Reach is deliberate (the boxes' padded extents
/// overlap), so the pair is examined rather than pruned.
#[test]
fn a_crossing_outside_the_disc_mints_no_event() {
    let tol = Tol::witness();
    let a = cyl(1.0, 0.0, 2.0);
    let b = boxx(1.05, 2.0, -0.5, 0.5, 1.0, 3.0);
    let topo::BooleanResult::Body(out) = topo::union(&a, &b, tol).expect("no crossing to route")
    else {
        panic!("two clear solids union into a two-shell body");
    };
    assert_eq!(out.body.shells().count(), 2);
    let v = topo::mass_properties(&out.body, tol).unwrap().volume;
    let truth = PI * 2.0 + 0.95 * 1.0 * 2.0;
    assert!((v - truth).abs() < 1e-12, "{v} vs {truth}");
}

/// The SOLID-containment sibling of the class, measured rather than
/// assumed: the ray lane reads a planar face's trim through the same
/// polygon walk (`solid_contain::point_in_face`), so a ray leaving
/// through a cap is a candidate for the same silence. A box buried in
/// a cylinder has no boundary crossing at all, so the operands are
/// classified by containment alone, and the union is the cylinder.
#[test]
fn a_box_buried_in_a_cylinder_unions_to_the_cylinder() {
    let tol = Tol::witness();
    let a = cyl(1.0, 0.0, 2.0);
    let b = boxx(-0.3, 0.3, -0.3, 0.3, 0.5, 1.5);
    let topo::BooleanResult::Body(out) = topo::union(&a, &b, tol).expect("containment decides")
    else {
        panic!("a buried box unions into one solid");
    };
    let v = topo::mass_properties(&out.body, tol).unwrap().volume;
    assert!((v - PI * 2.0).abs() < 1e-12, "{v} vs {}", PI * 2.0);
}

/// **The ring half of the disc class, isolated** — and the clearest
/// single result the substrate has, so it gets its own row rather than
/// a sentence.
///
/// A face's RINGS go through the same walk as its outer loop, so a
/// circular hole was invisible in exactly the way a cap was: the
/// ring's polygon through its two vertices is the hole's own diameter,
/// and every point inside the hole read as inside the face. A tube
/// cannot show it — a tube's outer loop is also a circle, and the two
/// errors cancel — so this is a SQUARE plate (outer loop a polygon,
/// always right) with a CIRCULAR hole, and a box driven straight down
/// the hole. The box meets no plate material at all; the operands are
/// honestly disjoint and the volumes add exactly.
///
/// Adopted from the R1 review probe branch, whose author observed that
/// nothing in the unit demonstrated the ring half.
#[test]
fn a_box_down_a_circular_hole_in_a_square_plate_sees_the_hole() {
    let tol = Tol::witness();
    let hole = profile::circle(p2(0.0, 0.0), 0.5, tol).unwrap();
    let plate = {
        let outer: ProfileLoop<f64> =
            RawLoop::polygon([p2(-2.0, -2.0), p2(2.0, -2.0), p2(2.0, 2.0), p2(-2.0, 2.0)]);
        let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.0)));
        let profile = Profile::new(plane, vec![outer, hole.into()])
            .validate(tol)
            .unwrap();
        extrude(&profile, Extrusion::Distance(1.0), tol)
            .unwrap()
            .body
    };
    let boss = boxx(-0.2, 0.2, -0.2, 0.2, 0.5, 2.0);
    let topo::BooleanResult::Body(out) =
        topo::union(&plate, &boss, tol).expect("the hole is empty; nothing to route")
    else {
        panic!("a plate and a box down its hole are two disjoint shells");
    };
    assert_eq!(out.body.shells().count(), 2, "the box touches no material");
    let v = topo::mass_properties(&out.body, tol).unwrap().volume;
    let truth = (16.0 - PI * 0.25) * 1.0 + 0.4 * 0.4 * 1.5;
    assert!((v - truth).abs() < 1e-12, "{v} vs {truth}");
}
