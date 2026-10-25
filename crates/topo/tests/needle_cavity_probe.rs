//! **The needle cavity's measuring probe** (FLUSH,
//! `a-needle-cavity-refuses-at-the-joins-section-loop-before-any-read`):
//! every boolean the pin `shell_role_reader::a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void`
//! reads, and the plain `brick ∖ bar` the row first named, printed per
//! width, pose and ε. It asserts nothing; run it at each ε with
//! `CAD_TOLERANCE_EPS=<ε> cargo nextest run -p topo --release --run-ignored only needle_cavity_probe --no-capture`.
//! `tilted` turns each operand's corners; `moved` turns the
//! axis-aligned operands of the union with `transform_rigid`.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use geom_core::{Point3, Tol};
use topo::{AtRestBody, BooleanError, BooleanResult};

use crate::common;

fn rot(axis: [f64; 3], ang: f64) -> [[f64; 3]; 3] {
    let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let (x, y, z) = (axis[0] / n, axis[1] / n, axis[2] / n);
    let (s, c) = ang.sin_cos();
    let k = 1.0 - c;
    [
        [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
        [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
        [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
    ]
}

fn boxed(
    r: [[f64; 3]; 3],
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
    tol: Tol,
) -> AtRestBody<f64> {
    let b = common::mapped_cube::<f64>(
        move |u, v, w| {
            let p = [
                x.0 + u * (x.1 - x.0),
                y.0 + v * (y.1 - y.0),
                z.0 + w * (z.1 - z.0),
            ];
            let at = |k: usize| r[k][0] * p[0] + r[k][1] * p[1] + r[k][2] * p[2];
            Point3::new(at(0), at(1), at(2))
        },
        tol,
    );
    AtRestBody::validate(b, tol).expect("operand at rest")
}

fn show(r: &Result<BooleanResult<f64>, BooleanError>) -> String {
    match r {
        Ok(res) => match res.body() {
            Some(b) => format!(
                "built shells/solid {:?}",
                b.body
                    .solids()
                    .map(|(_, s)| s.shells.len())
                    .collect::<Vec<_>>()
            ),
            None => "empty".into(),
        },
        Err(e) => format!("ERR {e:?}").chars().take(300).collect(),
    }
}

/// `(big ∖ slab) ∪ fill`, shown.
fn union_of_the_hollow(
    big: &AtRestBody<f64>,
    slab: &AtRestBody<f64>,
    fill: &AtRestBody<f64>,
    tol: Tol,
) -> String {
    match topo::subtract(big, slab, tol) {
        Ok(h) => show(&topo::union(&h.body().expect("a hollow").body, fill, tol)),
        Err(e) => format!("brick ∖ slab ERR {e:?}"),
    }
}

#[test]
#[ignore = "probe: CAD_TOLERANCE_EPS=<ε> cargo nextest run -p topo --release --run-ignored only needle_cavity_probe --no-capture"]
fn needle_cavity_probe() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let band = geom_core::Band::linear(tol).expect("the run's band");
    let axis = geom_core::UnitVec3::new(geom_core::Vec3::new(1.0, 2.0, 3.0), "probe", band)
        .expect("a unit axis");
    let motion = geom_core::RigidMotion::rotation(axis, 1.1).expect("a finite angle");
    let moved = |b: &AtRestBody<f64>| {
        AtRestBody::validate(topo::transform_rigid(b, &motion, tol).expect("moves"), tol)
            .expect("moved operand at rest")
    };
    for (pose, r) in [
        ("axis", rot([0.0, 0.0, 1.0], 0.0)),
        ("tilted", rot([1.0, 2.0, 3.0], 1.1)),
    ] {
        for m in [10.5, 12.0, 15.0, 21.0, 30.0] {
            let w = m * eps;
            let at = |op: &str| format!("eps {eps:e} {pose} {m}ε  {op}:");
            let big = boxed(r, (-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), tol);
            let bar = boxed(r, (0.0, 1.0), (0.5, 0.5 + w), (0.5, 0.5 + w), tol);
            let slab = boxed(r, (0.0, 1.0), (0.5, 0.8), (0.5, 0.5 + w), tol);
            let fill = boxed(r, (0.0, 1.0), (0.5 + w, 0.8), (0.5, 0.5 + w), tol);
            println!(
                "{} {}",
                at("big∖bar"),
                show(&topo::subtract(&big, &bar, tol))
            );
            println!(
                "{} {}",
                at("bar∩big"),
                show(&topo::intersect(&bar, &big, tol))
            );
            println!("{} {}", at("big∪bar"), show(&topo::union(&big, &bar, tol)));
            println!(
                "{} {}",
                at("slab∖fill"),
                show(&topo::subtract(&slab, &fill, tol))
            );
            let cut = match topo::subtract(&big, &fill, tol) {
                Ok(c) => show(&topo::intersect(
                    &slab,
                    &c.body().expect("a complement").body,
                    tol,
                )),
                Err(e) => format!("brick ∖ fill ERR {e:?}"),
            };
            println!("{} {cut}", at("slab∩(big∖fill)"));
            println!(
                "{} {}",
                at("hollow∪fill"),
                union_of_the_hollow(&big, &slab, &fill, tol)
            );
            if pose == "axis" {
                println!(
                    "eps {eps:e} moved {m}ε  hollow∪fill: {}",
                    union_of_the_hollow(&moved(&big), &moved(&slab), &moved(&fill), tol)
                );
            }
        }
    }
}
