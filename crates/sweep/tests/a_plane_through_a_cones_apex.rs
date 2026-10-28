//! probe
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use topo::splitting::split;

fn cone() -> topo::AtRestBody<f64> {
    let lp = bulge_loop(
        [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]
            .iter()
            .map(|&(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
    );
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let body = sweep::revolve(
        &vp,
        sweep::RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        },
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    sweep::test_support::finished("operand", body, Tol::witness())
}

#[test]
fn zz_probe_cone_apex() {
    let body = cone();
    let thetas = [
        0.0,
        FRAC_PI_2,
        PI,
        3.0 * FRAC_PI_2,
        0.3,
        1.1,
        2.0,
        2.9,
        4.0,
        5.5,
    ];
    let tilts = [0.0, 1e-3, 0.05, 0.4, 1.0, FRAC_PI_2, 2.0, 3.0, -0.4, -1.2];
    let mut ok = 0;
    let mut bad = 0;
    for a in thetas {
        for t in tilts {
            if std::env::var("ZZ_ONE").is_ok() && (a != 0.3 || t != 0.4) { continue; }
            if t != 0.0 && ![0.0, 0.3, 2.0].contains(&a) {
                continue;
            }
            for s in [1.0, -1.0] {
                let axis = Vec3::new(0.0, 1.0, 0.0);
                let radial = Vec3::new(a.cos(), 0.0, a.sin());
                let ruling = (axis - radial).normalize();
                let outward = (radial + axis).normalize();
                let n = (outward * t.cos() + ruling.cross(outward) * t.sin()) * s;
                let p = topo::test_support::split_plane(
                    Point3::new(0.0, 0.0, 0.0) + radial,
                    n,
                    Tol::witness(),
                );
                match split(&body, &p, Tol::witness()) {
                    Ok(r) => {
                        ok += 1;
                        let v = |b: Option<&topo::AtRestBody<f64>>| {
                            b.map(|b| {
                                topo::mass_properties(
                                    b,
                                    Tol::witness(),
                                    topo::test_support::pinned_resolution(Tol::witness()),
                                )
                                .map(|m| m.enclosure.volume)
                            })
                        };
                        println!(
                            "a={a} t={t} s={s}: OK {:?} {:?}",
                            v(r.above.body()),
                            v(r.below.body())
                        );
                    }
                    Err(e) => {
                        bad += 1;
                        let s = format!("{e:?}");
                        println!("a={a} t={t} s={s}: ERR {}", &s[..s.len().min(700)]);
                    }
                }
            }
        }
    }
    println!("ok {ok} bad {bad}");
}
