//! Probe: which edge × cone-face pairs refuse, and where.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::{AtRestBody, Body, BooleanOp};

fn revolved(pts: &[(f64, f64)]) -> Body<f64> {
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn widening() -> AtRestBody<f64> {
    finished(
        "widening",
        revolved(&[(0.0, 0.0), (0.5, 0.0), (1.0, 1.0), (0.0, 1.0)]),
        Tol::witness(),
    )
}

fn narrowing() -> AtRestBody<f64> {
    finished(
        "narrowing",
        revolved(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]),
        Tol::witness(),
    )
}

fn full_cone() -> AtRestBody<f64> {
    finished(
        "cone",
        revolved(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]),
        Tol::witness(),
    )
}

fn moved(b: &Body<f64>, m: &Affine3<f64>) -> AtRestBody<f64> {
    finished(
        "moved",
        topo::transform_rigid(b, m, Tol::witness()).unwrap(),
        Tol::witness(),
    )
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    sweep::test_support::brick(x, y, z, Tol::witness())
}

/// A cube of side `s` centred at `c`, its body diagonal turned onto `+y`.
fn diag_cube(s: f64, c: [f64; 3]) -> AtRestBody<f64> {
    let h = s / 2.0;
    let b = brick((-h, h), (-h, h), (-h, h));
    // Turn (1,1,1)/√3 onto +y: about (1,1,1)×(0,1,0) = (−1,0,1).
    let d = Vec3::new(1.0, 1.0, 1.0) / 3f64.sqrt();
    let angle = d.dot(Vec3::new(0.0, 1.0, 0.0)).acos();
    let k = Vec3::new(-1.0, 0.0, 1.0) / 2f64.sqrt();
    let turn = Affine3::rotation_about_axis(Point3::origin(), k, angle);
    let b = topo::transform_rigid(&b, &turn, Tol::witness()).unwrap();
    moved(&b, &Affine3::translation(Vec3::new(c[0], c[1], c[2])))
}

fn rod(r: f64, y0: f64, h: f64, tilt: f64, at: [f64; 3]) -> AtRestBody<f64> {
    // A z-prism turned onto y, tilted about z.
    let b = sweep::test_support::prism_at(
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        y0,
        h,
        Tol::witness(),
    );
    let onto_y = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(1.0, 0.0, 0.0),
        -core::f64::consts::FRAC_PI_2,
    );
    let b = topo::transform_rigid(&b, &onto_y, Tol::witness()).unwrap();
    let t = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), tilt);
    let b = topo::transform_rigid(&b, &t, Tol::witness()).unwrap();
    moved(&b, &Affine3::translation(Vec3::new(at[0], at[1], at[2])))
}

fn run(op: BooleanOp, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> String {
    let r = match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    };
    match r {
        Ok(out) => match out.body() {
            Some(b) => format!(
                "Ok vol {:.12}",
                topo::mass_properties(&b.body, Tol::witness())
                    .map(|p| p.volume)
                    .unwrap_or(f64::NAN)
            ),
            None => "Ok empty".into(),
        },
        Err(e) => format!("Err {e:?}").chars().take(220).collect(),
    }
}

#[test]
#[ignore = "probe"]
fn probe() {
    let rows: Vec<(&str, AtRestBody<f64>, AtRestBody<f64>)> = vec![
        ("diag cube across widening wall", widening(), diag_cube(0.4, [0.75, 0.5, 0.0])),
        ("diag cube across narrowing wall", narrowing(), diag_cube(0.4, [0.75, 0.5, 0.0])),
        (
            "axis box across widening wall",
            widening(),
            finished("b", brick((0.4, 1.4), (0.25, 0.75), (-0.2, 0.2)), Tol::witness()),
        ),
        (
            "box through cone apex",
            full_cone(),
            finished("b", brick((-0.2, 0.2), (0.8, 1.2), (-0.2, 0.2)), Tol::witness()),
        ),
        (
            "box edge through apex",
            full_cone(),
            finished("b", brick((0.0, 0.4), (1.0, 1.4), (0.0, 0.4)), Tol::witness()),
        ),
        (
            "segment inside both nappes",
            full_cone(),
            finished("b", brick((0.01, 0.03), (0.5, 1.5), (-0.01, 0.01)), Tol::witness()),
        ),
        ("coaxial rod in widening", widening(), rod(0.6, 0.2, 1.2, 0.0, [0.0, 0.0, 0.0])),
        ("coaxial thin rod", widening(), rod(0.3, -0.3, 0.6, 0.0, [0.0, 0.0, 0.0])),
        ("tilted rod", widening(), rod(0.2, -0.5, 2.0, 0.3, [0.5, 0.0, 0.0])),
    ];
    for (label, a, b) in &rows {
        for (op_l, op, x, y) in [
            ("A∪B", BooleanOp::Union, a, b),
            ("A∩B", BooleanOp::Intersect, a, b),
            ("A∖B", BooleanOp::Subtract, a, b),
            ("B∖A", BooleanOp::Subtract, b, a),
        ] {
            eprintln!("{label} | {op_l} | {}", run(op, x, y));
        }
    }
}
