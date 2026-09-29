//! **A conic edge lying in a plane face's plane and running inside it.**
//!
//! The sweep gives such an edge endpoint treatment only and leaves its
//! interior to the neighbour faces, as it does a coplanar line. These
//! rows hold the op-level consequence on operands whose conic edge lies
//! in a box's face plane:
//!
//! - a cylinder whose cap is coplanar with the box's top, the box's
//!   boundary crossing the rim (the cap beside the rim is a coplanar
//!   neighbour; today every op refuses `UndeclaredCoincidence`);
//! - a donut revolved with its seam parallels at the equators, the box
//!   top in the equator plane holding an arc of the outer equator (the
//!   neighbours are the two torus faces; today ∪ refuses
//!   `CurvedSectorSideUnsupported` and ∖, ∩ refuse the torus at their
//!   roster, `CurvedPairUnsupported`);
//! - a tube whose outer wall is two faces meeting in a circle, the box
//!   top in that circle's plane holding an arc of it, or all of it
//!   (today every op refuses at the join);
//! - a die pip whose ball is poled along `y`, so its seam meridian and
//!   both poles lie in the cube's top face (today every op refuses at
//!   the join's tilted plane×sphere section; before the sweep recorded
//!   the poles, the no-crossings fallback re-charted the ball and ∖
//!   answered its closed form).
//!
//! Each op must refuse typed or answer its closed-form volume with
//! `point_in_solid` agreeing on its set membership at witness points.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::germ_pair::cyl;
use crate::revolve_common::{axis_y, validated};

use geom_core::{Affine3, Band, Mat3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop, test_support::bulge_loop};
use sweep::{Revolution, revolve};
use topo::{Body, SolidContainment};

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

fn revolved(lp: ProfileLoop<f64>) -> Body<f64> {
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the profile revolves")
    .body
}

/// The donut `R = 2`, `r = 1/2` about `y`, its meridian authored as two
/// half circles joined at `ρ = 1.5` and `ρ = 2.5`, so its seam
/// parallels are the two equators in the plane `y = 0`.
fn equator_donut() -> Body<f64> {
    revolved(bulge_loop(vec![
        (Point2::new(2.5, 0.0), 1.0),
        (Point2::new(1.5, 0.0), 1.0),
    ]))
}

/// The tube `0.5 ≤ ρ ≤ 1`, `y ∈ [−1, 1]` about `y`, its outer wall
/// authored as two faces meeting in the circle `ρ = 1`, `y = 0`.
fn strutted_tube() -> Body<f64> {
    revolved(ProfileLoop::polygon([
        Point2::new(0.5, -1.0),
        Point2::new(1.0, -1.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.5, 1.0),
    ]))
}

/// A radius-0.3 ball poled along `y`, centred at `(0.5, 0.5, 1)`: its
/// seam meridian lies in the plane `z = 1`.
fn y_poled_pip() -> Body<f64> {
    let ball = revolved(bulge_loop(vec![
        (Point2::new(0.0, -0.3), 1.0),
        (Point2::new(0.0, 0.3), 0.0),
    ]));
    topo::transform_rigid(
        &ball,
        &Affine3::translation(Vec3::new(0.5, 0.5, 1.0)),
        Tol::witness(),
    )
    .expect("the ball moves to its pip")
}

/// `∫ √(1 − s²) ds` over `[−c, c]`.
fn disc_band(c: f64) -> f64 {
    c * (1.0 - c * c).sqrt() + c.asin()
}

/// The donut's material below `y = 0` over an `xz` window: the tube's
/// half height `√(r² − (ρ − R)²)` integrated by the midpoint rule.
fn donut_under_window(x: (f64, f64), z: (f64, f64), n: usize) -> f64 {
    let (dx, dz) = ((x.1 - x.0) / n as f64, (z.1 - z.0) / n as f64);
    let mut sum = 0.0;
    for i in 0..n {
        for j in 0..n {
            let (px, pz) = (x.0 + (i as f64 + 0.5) * dx, z.0 + (j as f64 + 0.5) * dz);
            let d = (px * px + pz * pz).sqrt() - 2.0;
            sum += (0.25 - d * d).max(0.0).sqrt();
        }
    }
    sum * dx * dz
}

/// A witness point and its membership in each operand.
type Witness = ([f64; 3], bool, bool);

struct Fixture {
    name: &'static str,
    a: Body<f64>,
    b: Body<f64>,
    vol_a: f64,
    vol_b: f64,
    overlap: f64,
    /// The volume row's absolute tolerance: the quadrature's where the
    /// overlap is integrated numerically.
    abs: f64,
    witnesses: Vec<Witness>,
}

const DONUT_WINDOW: ((f64, f64), (f64, f64)) = ((-0.4, 0.4), (2.2, 3.0));

fn fixtures() -> Vec<Fixture> {
    vec![
        Fixture {
            name: "cylinder cap in the box top",
            a: cyl(1.0, 0.5),
            b: boxed((0.0, 2.0), (-0.5, 0.5), (-1.0, 0.5)),
            vol_a: PI,
            vol_b: 3.0,
            overlap: disc_band(0.5),
            abs: 1e-9,
            witnesses: vec![
                ([0.5, 0.0, 0.0], true, true),
                ([-0.5, 0.0, 0.0], true, false),
                ([1.5, 0.0, 0.0], false, true),
                ([0.5, 0.8, 0.0], true, false),
                ([0.5, 0.0, 0.7], false, false),
            ],
        },
        Fixture {
            name: "outer equator arc in the box top",
            a: equator_donut(),
            b: boxed(DONUT_WINDOW.0, (-1.0, 0.0), DONUT_WINDOW.1),
            vol_a: PI * PI,
            vol_b: 0.64,
            overlap: donut_under_window(DONUT_WINDOW.0, DONUT_WINDOW.1, 1000),
            abs: 1e-4,
            witnesses: vec![
                ([0.0, -0.2, 2.5], true, true),
                ([0.0, -0.2, 2.25], true, true),
                ([0.0, 0.2, 2.5], true, false),
                ([0.0, -0.8, 2.5], false, true),
                ([0.0, -0.2, 3.2], false, false),
            ],
        },
        Fixture {
            name: "tube strut arc in the box top",
            a: strutted_tube(),
            b: boxed((-0.3, 0.3), (-2.0, 0.0), (0.8, 1.3)),
            vol_a: 1.5 * PI,
            vol_b: 0.6,
            overlap: disc_band(0.3) - 0.48,
            abs: 1e-9,
            witnesses: vec![
                ([0.0, -0.5, 0.9], true, true),
                ([0.0, 0.5, 0.9], true, false),
                ([0.0, -0.5, 1.2], false, true),
                ([0.0, 0.5, 1.2], false, false),
            ],
        },
        Fixture {
            name: "whole tube strut in the box top",
            a: strutted_tube(),
            b: boxed((-1.5, 1.5), (-2.0, 0.0), (-1.5, 1.5)),
            vol_a: 1.5 * PI,
            vol_b: 18.0,
            overlap: 0.75 * PI,
            abs: 1e-9,
            witnesses: vec![
                ([0.0, -0.5, 0.75], true, true),
                ([0.0, 0.5, 0.75], true, false),
                ([0.0, -0.5, 0.0], false, true),
                ([0.0, 0.5, 0.0], false, false),
            ],
        },
        Fixture {
            name: "y-poled pip on the cube's top face",
            a: sweep::test_support::cube(1.0, Tol::witness()),
            b: y_poled_pip(),
            vol_a: 1.0,
            vol_b: 4.0 / 3.0 * PI * 0.027,
            overlap: 2.0 / 3.0 * PI * 0.027,
            abs: 1e-9,
            witnesses: vec![
                ([0.5, 0.5, 0.9], true, true),
                ([0.5, 0.5, 0.5], true, false),
                ([0.5, 0.5, 1.1], false, true),
                ([0.5, 0.5, 1.5], false, false),
            ],
        },
    ]
}

/// One op: its name, its answer, its closed-form volume, and its set
/// membership as a function of membership in A and in B.
type Row = (
    &'static str,
    Result<topo::BooleanResult<f64>, topo::BooleanError>,
    f64,
    fn(bool, bool) -> bool,
);

#[test]
fn every_op_refuses_or_answers_its_closed_form() {
    let band = Band::linear(Tol::witness()).unwrap();
    let tol = Tol::witness();
    let mut failures = Vec::new();
    let mut refusals = Vec::new();
    for fx in fixtures() {
        let (a, b, ov) = (&fx.a, &fx.b, fx.overlap);
        let rows: [Row; 3] = [
            (
                "A ∪ B",
                topo::union(a, b, tol),
                fx.vol_a + fx.vol_b - ov,
                |a, b| a || b,
            ),
            ("A ∩ B", topo::intersect(a, b, tol), ov, |a, b| a && b),
            (
                "A ∖ B",
                topo::subtract(a, b, tol),
                fx.vol_a - ov,
                |a, b| a && !b,
            ),
        ];
        for (op, r, want, member) in rows {
            let what = format!("{}: {op}", fx.name);
            let r = match r {
                Ok(r) => r,
                Err(e) => {
                    refusals.push(format!("{what}: {:?}", e.kind()));
                    continue;
                }
            };
            let Some(body) = r.body() else {
                failures.push(format!("{what}: came back empty"));
                continue;
            };
            let body = &body.body;
            if let Err(e) = topo::validate_geometric(body, tol) {
                failures.push(format!("{what}: invalid body {e:?}"));
            }
            let got = topo::mass_properties(body, tol)
                .expect("the volume integrates")
                .volume;
            if (got - want).abs() > fx.abs {
                failures.push(format!(
                    "{what}: volume {got} against the closed form {want}"
                ));
            }
            for &([x, y, z], in_a, in_b) in &fx.witnesses {
                let expect = if member(in_a, in_b) {
                    SolidContainment::In
                } else {
                    SolidContainment::Out
                };
                let q = Point3::new(x, y, z);
                let got = topo::point_in_solid(body, q, band, tol);
                if !matches!(got, Ok(c) if c == expect) {
                    failures.push(format!("{what}: point_in_solid{q:?} = {got:?}"));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{}\n(refused: {refusals:?})",
        failures.join("\n")
    );
}

/// The closed forms the row above holds an answer to, checked against
/// the operands' own volumes, so an answer is not measured against a
/// wrong constant while every op refuses; and the quadrature against
/// itself at twice the resolution, inside the row's tolerance.
#[test]
fn the_closed_forms_hold_on_the_operands() {
    let tol = Tol::witness();
    for fx in fixtures() {
        for (which, body, want) in [("A", &fx.a, fx.vol_a), ("B", &fx.b, fx.vol_b)] {
            let got = topo::mass_properties(body, tol)
                .expect("the volume integrates")
                .volume;
            assert!(
                (got - want).abs() < 1e-9 * want.max(1.0),
                "{}: {which} volume {got} against {want}",
                fx.name
            );
        }
        assert!(
            fx.overlap > 0.0 && fx.overlap < fx.vol_a.min(fx.vol_b),
            "{}: the overlap {} lies strictly inside both",
            fx.name,
            fx.overlap
        );
    }
    let coarse = donut_under_window(DONUT_WINDOW.0, DONUT_WINDOW.1, 1000);
    let fine = donut_under_window(DONUT_WINDOW.0, DONUT_WINDOW.1, 2000);
    assert!(
        (coarse - fine).abs() < 1e-5,
        "the quadrature has converged: {coarse} against {fine}"
    );
}
