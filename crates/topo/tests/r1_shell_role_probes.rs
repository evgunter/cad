//! Review R1 (FLUSH shell-role reader) probes: oblique slabs at and just
//! above Kε, sharp wedges and sliver tets whose faces all meet, and the
//! cost of a ray per face on a many-sided coin. Printed, not pinned.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::meeting::{Pose, apex_pyramid_kept};
use common::{FaceGeometry, describe_as_intersections, prism_ops};
use geom_core::{Point3, Tol};
use topo::{AtRestBody, Body, BooleanResult, classify_shells};

fn tol() -> Tol {
    Tol::witness()
}

fn poses() -> Vec<Pose> {
    vec![
        Pose::turn("axis", [0.0, 0.0, 1.0], 0.0, [0.0; 3]),
        Pose::turn(
            "x=y",
            [0.0, 0.0, 1.0],
            core::f64::consts::FRAC_PI_4,
            [0.0; 3],
        ),
        Pose::turn("oblique1", [1.0, 2.0, 3.0], 0.7, [0.3, -0.2, 0.5]),
        Pose::turn("oblique2", [-0.3, 0.9, 0.1], 2.1, [1.7, 0.4, -0.9]),
        Pose::turn("oblique3", [0.577, 0.577, 0.577], 1.234_567, [0.0; 3]),
        Pose::turn("far", [0.2, -0.5, 0.8], 0.4, [37.0, -21.0, 13.0]),
    ]
}

/// A prism over `profile` in (x, y), z in `z`, placed by `pose`.
fn prism(profile: &[(f64, f64)], z: (f64, f64), pose: &Pose) -> Body<f64> {
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        profile,
        z,
        |x, y, z| pose.at([x, y, z]),
        FaceGeometry::Certified,
        tol(),
    );
    describe_as_intersections(&mut body, tol());
    body
}

fn short<E: core::fmt::Debug>(e: &E) -> String {
    let s = format!("{e:?}");
    s.chars().take(260).collect()
}

fn rest(body: Body<f64>) -> Result<AtRestBody<f64>, String> {
    AtRestBody::validate(body, tol()).map_err(|e| short(&e))
}

#[test]
#[ignore = "review probe"]
fn r1_oblique_slabs_at_k_eps() {
    let (e, k) = (tol().eps(), tol().k());
    eprintln!("eps {e:e} k {k}");
    for pose in poses() {
        for m in [1.0, 1.000_001, 1.001, 1.05, 1.5] {
            let t = m * k * e;
            let slab = || {
                prism(
                    &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
                    (0.0, t),
                    &pose,
                )
            };
            let Ok(b) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(slab)) else {
                eprintln!("{} m={m}: the prism does not build", pose.label);
                continue;
            };
            let lone = rest(b);
            let lone_s = match &lone {
                Ok(b) => format!(
                    "ok, classify {:?}",
                    classify_shells(b, tol())
                        .map(|c| c.iter().map(|c| c.role).collect::<Vec<_>>())
                        .map_err(|e| short(&e))
                ),
                Err(e) => format!("REFUSED {e}"),
            };
            // A cavity: a big brick less the slab (posed brick).
            let big = rest(prism(
                &[(-1.0, -1.0), (2.0, -1.0), (2.0, 2.0), (-1.0, 2.0)],
                (-1.0, 2.0),
                &pose,
            ))
            .unwrap();
            let cav = match &lone {
                Ok(s) => match topo::subtract(&big, s, tol()) {
                    Ok(BooleanResult::Body(r)) => format!(
                        "cavity ok shells {:?}",
                        r.body
                            .solids()
                            .map(|(_, s)| s.shells.len())
                            .collect::<Vec<_>>()
                    ),
                    Ok(BooleanResult::Empty) => "cavity empty".into(),
                    Err(e) => format!("cavity REFUSED {}", short(&e)),
                },
                Err(_) => "-".into(),
            };
            eprintln!("{} m={m}: lone {lone_s} | {cav}", pose.label);
        }
    }
}

#[test]
#[ignore = "review probe"]
fn r1_sharp_wedges_and_sliver_tets() {
    let (e, k) = (tol().eps(), tol().k());
    let ke = k * e;
    for pose in poses().into_iter().take(3) {
        for m in [2.0, 3.0, 5.0, 10.0, 30.0, 100.0, 1000.0] {
            let h = m * ke;
            // A knife wedge: triangle (0,0),(1,0),(1,h) extruded z 0..1.
            let w = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, h)], (0.0, 1.0), &pose)
            }))
            .map_err(|_| "the prism does not build".to_owned())
            .and_then(rest);
            // A sliver tet: flat base triangle, apex h above an interior point.
            let base = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
            let _ = &base;
            let tet = apex_pyramid_kept(
                &[[0.0, 0.0, -h], [1.0, 0.0, -h], [0.0, 1.0, -h]],
                &pose,
                tol(),
            );
            eprintln!(
                "{} h={m}Kε: wedge {} | tet {}",
                pose.label,
                w.map(|_| "ok".to_owned())
                    .unwrap_or_else(|e| format!("REFUSED {e}")),
                tet.map(|_| "ok".to_owned())
                    .unwrap_or_else(|e| format!("REFUSED {}", short(&e))),
            );
        }
    }
}

#[test]
#[ignore = "review probe"]
fn r1_many_sided_coin_cost() {
    let ke = tol().k() * tol().eps();
    let pose = Pose::turn("oblique1", [1.0, 2.0, 3.0], 0.7, [0.3, -0.2, 0.5]);
    for n in [64usize, 256, 1024] {
        let ring: Vec<(f64, f64)> = (0..n)
            .map(|i| {
                let a = core::f64::consts::TAU * i as f64 / n as f64;
                (a.cos(), a.sin())
            })
            .collect();
        for (what, t) in [
            ("thick", 0.5),
            ("thin 0.5Kε", 0.5 * ke),
            ("thin 2Kε", 2.0 * ke),
        ] {
            let start = std::time::Instant::now();
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                prism(&ring, (0.0, t), &pose)
            }))
            .map_err(|_| "the prism does not build".to_owned())
            .and_then(rest);
            eprintln!(
                "coin n={n} {what}: {:?} in {:.2}s",
                r.as_ref()
                    .map(|_| ())
                    .map_err(|e| e.chars().take(120).collect::<String>()),
                start.elapsed().as_secs_f64()
            );
        }
    }
    let _ = Point3::new(0.0, 0.0, 0.0);
}

/// A boolean whose result is a needle `w × w` across, every pair of its
/// cells `w` apart: how the door types it.
#[test]
#[ignore = "review probe"]
fn r1_a_needle_result_at_the_door() {
    let ke = tol().k() * tol().eps();
    for pose in poses().into_iter().take(3) {
        for m in [1.2, 1.5, 2.0, 3.0, 5.0] {
            let w = m * ke;
            let a = rest(prism(&[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)], (0.0, 1.0), &pose)).unwrap();
            let b = rest(prism(
                &[(-1.0, 1.0 - w), (3.0, 1.0 - w), (3.0, 2.0), (-1.0, 2.0)],
                (1.0 - w, 2.0),
                &pose,
            ))
            .unwrap();
            let r = match topo::intersect(&a, &b, tol()) {
                Ok(BooleanResult::Body(_)) => "built".to_owned(),
                Ok(BooleanResult::Empty) => "empty".to_owned(),
                Err(e) => short(&e),
            };
            eprintln!("{} needle w={m}Kε: {r}", pose.label);
        }
    }
}
