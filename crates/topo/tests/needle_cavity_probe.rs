//! Probe: the needle cavity at the join's section loop (measure only).

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

#[test]
#[ignore = "probe: cargo nextest run -p topo --run-ignored only needle_cavity_probe --no-capture"]
fn needle_cavity_probe() {
    {
        let tol = Tol::witness();
        let eps = tol.eps();
        for (pose, r) in [
            ("axis", rot([0.0, 0.0, 1.0], 0.0)),
            ("tilted", rot([1.0, 2.0, 3.0], 1.1)),
        ]
        .into_iter()
        .take(if std::env::var_os("NEEDLE_AXIS").is_some() {
            1
        } else {
            2
        }) {
            for m in std::env::var("NEEDLE_M")
                .map(|v| vec![v.parse::<f64>().unwrap()])
                .unwrap_or(vec![10.5, 12.0, 15.0, 21.0, 30.0])
            {
                let w = m * eps;
                let big = boxed(r, (-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), tol);
                let bar = boxed(r, (0.0, 1.0), (0.5, 0.5 + w), (0.5, 0.5 + w), tol);
                let slab = boxed(r, (0.0, 1.0), (0.5, 0.8), (0.5, 0.5 + w), tol);
                let fill = boxed(r, (0.0, 1.0), (0.5 + w, 0.8), (0.5, 0.5 + w), tol);
                println!(
                    "eps {eps:e} {pose} {m}ε  big∖bar: {}",
                    show(&topo::subtract(&big, &bar, tol))
                );
                println!(
                    "eps {eps:e} {pose} {m}ε  bar∩big: {}",
                    show(&topo::intersect(&bar, &big, tol))
                );
                println!(
                    "eps {eps:e} {pose} {m}ε  big∪bar: {}",
                    show(&topo::union(&big, &bar, tol))
                );
                let mv = |b: AtRestBody<f64>| {
                    let axis = geom_core::UnitVec3::new(
                        geom_core::Vec3::new(1.0, 2.0, 3.0),
                        "probe",
                        geom_core::Band::linear(tol).unwrap(),
                    )
                    .unwrap();
                    let m = geom_core::RigidMotion::rotation(axis, 1.1).unwrap();
                    AtRestBody::validate(topo::transform_rigid(&b, &m, tol).unwrap(), tol).unwrap()
                };
                if pose == "axis" {
                    let (big, slab, fill) = (mv(big.clone()), mv(slab.clone()), mv(fill.clone()));
                    match topo::subtract(&big, &slab, tol) {
                        Ok(h) => {
                            let hollow = h.body().unwrap().body.clone();
                            println!(
                                "eps {eps:e} moved {m}ε  hollow∪fill: {}",
                                show(&topo::union(&hollow, &fill, tol))
                            );
                        }
                        Err(e) => println!("eps {eps:e} moved {m}ε  big∖slab ERR {e:?}"),
                    }
                }
                println!(
                    "eps {eps:e} {pose} {m}ε  slab∖fill: {}",
                    show(&topo::subtract(&slab, &fill, tol))
                );
                match topo::subtract(&big, &fill, tol) {
                    Ok(c) => {
                        let comp = c.body().unwrap().body.clone();
                        println!(
                            "eps {eps:e} {pose} {m}ε  slab∩(big∖fill): {}",
                            show(&topo::intersect(&slab, &comp, tol))
                        );
                    }
                    Err(e) => println!("eps {eps:e} {pose} {m}ε  big∖fill ERR {e:?}"),
                }
                match topo::subtract(&big, &slab, tol) {
                    Ok(h) => {
                        let hollow = h.body().unwrap().body.clone();
                        println!(
                            "eps {eps:e} {pose} {m}ε  hollow∪fill: {}",
                            show(&topo::union(&hollow, &fill, tol))
                        );
                    }
                    Err(e) => println!("eps {eps:e} {pose} {m}ε  big∖slab ERR {e:?}"),
                }
            }
        }
    }
}
