//! **A rim arc whose first plane root lands outside the face and whose
//! second lands inside it.**
//!
//! A is the unit cylinder `z ∈ [−1, 1]`, spun about its axis so the
//! rim's seam vertex moves; B is a thin box whose `y = 0.5` face covers
//! only `x ≤ 0`. Each rim arc crosses the plane `y = 0.5` twice, at
//! `x = ±√3/2`, and only the `x < 0` crossing is inside B's face. At the
//! spins below the arc's parameter order puts the outside crossing
//! first, so the sweep has to examine past it to find the one that
//! matters. B's vertical edges pierce A's caps, so every op here has
//! crossings elsewhere too.
//!
//! Every op answers its closed form, and `point_in_solid` agrees with
//! the op's set membership at witness points on both sides of each
//! boundary, including beside the crossing the second root names.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::germ_pair::{cyl, spin};

use geom_core::{Affine3, Band, Mat3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use topo::{Body, SolidContainment};

/// The spins at which the rim arc meets the outside root first.
const SPINS: [f64; 3] = [0.0, 0.3, 3.0];

fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(x.0, y.0),
        Point2::new(x.1, y.0),
        Point2::new(x.1, y.1),
        Point2::new(x.0, y.1),
    ]);
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Vec3::new(0.0, 0.0, z.0),
    ));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("the box profile validates");
    sweep::extrude(&vp, sweep::Extrusion::Distance(z.1 - z.0), Tol::witness())
        .expect("the box extrudes")
        .body
}

fn volume(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, Tol::witness())
        .expect("the volume integrates")
        .volume
}

/// `∫ √(1 − y²) dy` from `0.5` to `0.7`: the antiderivative
/// `(y√(1 − y²) + asin y)/2`.
fn disc_strip() -> f64 {
    let f = |y: f64| (y * (1.0 - y * y).sqrt() + y.asin()) / 2.0;
    f(0.7) - f(0.5)
}

/// Where a witness point sits relative to the two operands.
#[derive(Clone, Copy)]
struct Witness {
    q: Point3<f64>,
    in_a: bool,
    in_b: bool,
}

struct Fixture {
    /// B's `x` window; its upper end is `0` or `−0.3`.
    x: (f64, f64),
    witnesses: [Witness; 5],
}

impl Fixture {
    fn b(&self) -> Body<f64> {
        boxed(self.x, (0.5, 0.7), (-1.5, 1.5))
    }
    /// The overlap: the strip `y ∈ [0.5, 0.7]` of the unit disc left of
    /// `x = x₁` (the disc's chord there is wider than `|x₁|` over the
    /// whole strip, since `√(1 − 0.7²) > 0.3`), times A's height 2.
    fn overlap(&self) -> f64 {
        2.0 * (disc_strip() + 0.2 * self.x.1)
    }
    fn vol_b(&self) -> f64 {
        (self.x.1 - self.x.0) * 0.2 * 3.0
    }
}

fn w(x: f64, y: f64, z: f64, in_a: bool, in_b: bool) -> Witness {
    Witness {
        q: Point3::new(x, y, z),
        in_a,
        in_b,
    }
}

fn fixtures() -> [Fixture; 2] {
    [
        Fixture {
            x: (-1.5, 0.0),
            witnesses: [
                // Both, beside the crossing the second root names.
                w(-0.8, 0.55, 0.9, true, true),
                // B only, just past the rim there.
                w(-0.85, 0.6, 0.9, false, true),
                // B only, above A's cap.
                w(-0.2, 0.6, 1.3, false, true),
                // A only, beside the crossing the first root names.
                w(0.8, 0.55, 0.9, true, false),
                // Neither.
                w(-1.2, 0.4, 0.0, false, false),
            ],
        },
        Fixture {
            x: (-1.5, -0.3),
            witnesses: [
                w(-0.8, 0.55, 0.9, true, true),
                w(-0.85, 0.6, 0.9, false, true),
                w(-0.5, 0.6, 1.3, false, true),
                // A only, in the strip right of B's `x = −0.3` face.
                w(-0.1, 0.6, 0.5, true, false),
                w(-1.2, 0.4, 0.0, false, false),
            ],
        },
    ]
}

#[test]
fn every_op_answers_its_closed_form_when_the_outside_root_comes_first() {
    let band = Band::linear(Tol::witness()).unwrap();
    let vol_a = 2.0 * std::f64::consts::PI;
    let mut failures = Vec::new();
    for fx in fixtures() {
        let b = fx.b();
        let ov = fx.overlap();
        for s in SPINS {
            let a = spin(&cyl(1.0, 1.0), Vec3::unit_z(), s);
            let rows: [(&str, _, f64, fn(bool, bool) -> bool); 4] = [
                (
                    "A ∪ B",
                    topo::union(&a, &b, Tol::witness()),
                    vol_a + fx.vol_b() - ov,
                    |a, b| a || b,
                ),
                (
                    "A ∩ B",
                    topo::intersect(&a, &b, Tol::witness()),
                    ov,
                    |a, b| a && b,
                ),
                (
                    "A ∖ B",
                    topo::subtract(&a, &b, Tol::witness()),
                    vol_a - ov,
                    |a, b| a && !b,
                ),
                (
                    "B ∖ A",
                    topo::subtract(&b, &a, Tol::witness()),
                    fx.vol_b() - ov,
                    |a, b| b && !a,
                ),
            ];
            for (op, r, want, member) in rows {
                let what = format!("x ∈ {:?}, spin {s}: {op}", fx.x);
                let r = match r {
                    Ok(r) => r,
                    Err(e) => {
                        failures.push(format!("{what}: refused {e:?}"));
                        continue;
                    }
                };
                let Some(body) = r.body() else {
                    failures.push(format!("{what}: came back empty"));
                    continue;
                };
                let body = &body.body;
                if let Err(e) = topo::validate_geometric(body, Tol::witness()) {
                    failures.push(format!("{what}: invalid body {e:?}"));
                }
                let got = volume(body);
                if (got - want).abs() > 1e-9 * want.abs().max(1.0) {
                    failures.push(format!("{what}: volume {got} against the closed form {want}"));
                }
                for wt in fx.witnesses {
                    let expect = if member(wt.in_a, wt.in_b) {
                        SolidContainment::In
                    } else {
                        SolidContainment::Out
                    };
                    let got = topo::point_in_solid(body, wt.q, band, Tol::witness());
                    if !matches!(got, Ok(c) if c == expect) {
                        failures.push(format!("{what}: point_in_solid{:?} = {got:?}", wt.q));
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
