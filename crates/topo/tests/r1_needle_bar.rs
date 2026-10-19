//! Review R1, second delta of PR 5119: the needle class under the bandless
//! volume. A bar `(0,1)×(0,w)×(0,w)` at `w ∈ {10.5, 12, 15, 21, 30}ε`, both
//! orientations, at axis, oblique and far-translated poses; the same bar as
//! a cavity of a brick (`brick ∖ bar`), and that body inside out. Printed,
//! not pinned.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::meeting::Pose;
use common::{FaceGeometry, describe_as_intersections, prism_ops};
use geom_core::Tol;
use topo::{AtRestBody, Body, BooleanResult, classify_shells};

fn tol() -> Tol {
    Tol::witness()
}

fn poses() -> Vec<Pose> {
    vec![
        Pose::turn("axis", [0.0, 0.0, 1.0], 0.0, [0.0; 3]),
        Pose::turn("x=y", [0.0, 0.0, 1.0], core::f64::consts::FRAC_PI_4, [0.0; 3]),
        Pose::turn("oblique1", [1.0, 2.0, 3.0], 0.7, [0.3, -0.2, 0.5]),
        Pose::turn("oblique2", [-0.3, 0.9, 0.1], 2.1, [1.7, 0.4, -0.9]),
        Pose::turn("far37", [0.2, -0.5, 0.8], 0.4, [37.0, -21.0, 13.0]),
        Pose::turn("far1e3", [0.577, 0.577, 0.577], 1.234_567, [1000.0, -700.0, 300.0]),
        Pose::turn("axisfar1e3", [0.0, 0.0, 1.0], 0.0, [1000.0, 1000.0, 1000.0]),
        Pose::turn("z0.3", [0.0, 0.0, 1.0], 0.3, [0.0; 3]),
        Pose::turn("z0.3far1e3", [0.0, 0.0, 1.0], 0.3, [1000.0, -1000.0, 1000.0]),
        Pose::turn("z0.3far1e5", [0.0, 0.0, 1.0], 0.3, [1.0e5, -1.0e5, 1.0e5]),
        Pose::turn("x0.2far1e2", [1.0, 0.0, 0.0], 0.2, [100.0, 50.0, -70.0]),
    ]
}

fn prism(profile: &[(f64, f64)], z: (f64, f64), pose: &Pose) -> Option<Body<f64>> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut body = Body::<f64>::new();
        prism_ops(&mut body, profile, z, |x, y, z| pose.at([x, y, z]), FaceGeometry::Certified, tol());
        describe_as_intersections(&mut body, tol());
        body
    }))
    .ok()
}

fn short<E: core::fmt::Debug>(e: &E) -> String {
    format!("{e:?}").chars().take(300).collect()
}

fn verdict(b: Body<f64>) -> String {
    match AtRestBody::validate(b, tol()) {
        Ok(_) => "valid".into(),
        Err(e) => short(&e),
    }
}

#[test]
#[ignore = "review probe"]
fn r1_needle_bars() {
    let e = tol().eps();
    for pose in poses() {
        for m in [10.5, 12.0, 15.0, 21.0, 30.0] {
            let w = m * e;
            let bar = || prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, w), (0.0, w)], (0.0, w), &pose);
            let Some(b) = bar() else {
                eprintln!("{} {m}ε: the bar does not build", pose.label);
                continue;
            };
            let inv = match b.revert() {
                Ok(i) => verdict(i),
                Err(err) => format!("revert refuses {}", short(&err)),
            };
            let plain = verdict(b);
            // The bar as a cavity, and that body inside out.
            let big = prism(&[(-1.0, -1.0), (2.0, -1.0), (2.0, 2.0), (-1.0, 2.0)], (-1.0, 2.0), &pose)
                .map(|b| AtRestBody::validate(b, tol()).unwrap());
            let cav = match (big, bar().map(|b| AtRestBody::validate(b, tol()))) {
                (Some(big), Some(Ok(s))) => match topo::subtract(&big, &s, tol()) {
                    Ok(BooleanResult::Body(r)) => {
                        let roles = classify_shells(&r.body, tol())
                            .map(|c| c.iter().map(|c| format!("{:?}", c.role)).collect::<Vec<_>>())
                            .map_err(|e| short(&e));
                        let shells: Vec<_> = r.body.solids().map(|(_, s)| s.shells.len()).collect();
                        let inv = match r.body.revert() {
                            Ok(i) => verdict(i),
                            Err(err) => format!("revert refuses {}", short(&err)),
                        };
                        format!("cavity built shells {shells:?} roles {roles:?} | inside out: {inv}")
                    }
                    Ok(BooleanResult::Empty) => "cavity empty".into(),
                    Err(err) => format!("cavity REFUSED {}", short(&err)),
                },
                (_, Some(Err(err))) => format!("bar at rest refuses {}", short(&err)),
                _ => "cavity n/a".into(),
            };
            eprintln!("{} {m}ε: plain {plain} | inverted {inv} | {cav}", pose.label);
        }
    }
}
