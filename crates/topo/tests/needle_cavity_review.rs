//! Review probes for PR 5694 (`seam_germ_side`): the needle channel with
//! the fill's face toward it tilted by `s` (the channel's cross-section
//! a trapezoid, `w` at the floor and `w(1+s)` at the ceiling), both union
//! orders, the bar through ∖ and ∩. Prints each outcome; a built body
//! must pass tier 3′, read its roles and hold its closed-form volume.
//! `CAD_TOLERANCE_EPS=<ε> CAD_SEAM_PROBE=1 cargo test -p topo --test all -- --ignored --nocapture needle_cavity_review`

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use geom_core::{Point3, Tol};
use topo::{AtRestBody, BooleanError, BooleanResult, ShellRole};

use crate::common;

fn block(
    tol: Tol,
    r: [[f64; 3]; 3],
    f: impl Fn(f64, f64, f64) -> [f64; 3] + 'static,
) -> AtRestBody<f64> {
    let b = common::mapped_cube::<f64>(
        move |u, v, w| {
            let p = f(u, v, w);
            let at = |k: usize| r[k][0] * p[0] + r[k][1] * p[1] + r[k][2] * p[2];
            Point3::new(at(0), at(1), at(2))
        },
        tol,
    );
    AtRestBody::validate(b, tol).expect("operand at rest")
}

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

fn check(what: &str, r: Result<BooleanResult<f64>, BooleanError>, vol: f64, union: bool, tol: Tol) {
    let res = match r {
        Ok(res) => res,
        Err(e) => {
            println!("{what}: ERR {}", format!("{e:?}").chars().take(220).collect::<String>());
            return;
        }
    };
    let Some(b) = res.body() else {
        println!("{what}: EMPTY (wrong)");
        return;
    };
    let body: &topo::Body<f64> = &b.body;
    let t3 = topo::validate_pseudomanifold(body, &topo::ContactRecords::default(), tol);
    let v = topo::mass_properties(&b.body, tol).map(|m| m.volume);
    let roles = topo::classify_shells(
        &topo::test_support::finished("probe", body.clone(), tol),
        tol,
    )
    .map(|c| c.iter().map(|c| c.role).collect::<Vec<_>>());
    let vol_ok = v.as_ref().is_ok_and(|v| {
        if union {
            (v - vol).abs() < 1e-12
        } else {
            ((v - vol) / vol).abs() < 1e-3
        }
    });
    let roles_ok = if union {
        roles.as_ref().is_ok_and(|r| *r == [ShellRole::Outer, ShellRole::Void])
    } else {
        roles.as_ref().is_ok_and(|r| *r == [ShellRole::Outer])
    };
    println!(
        "{what}: BUILT t3'={} vol={v:?} want={vol:e} vol_ok={vol_ok} roles={roles:?} roles_ok={roles_ok}",
        t3.is_ok()
    );
}

#[test]
#[ignore = "review probe"]
fn needle_cavity_review() {
    let tol = Tol::witness();
    let e = tol.eps();
    for (pose, r) in [("axis", rot([0.0, 0.0, 1.0], 0.0)), ("turned", rot([1.0, 2.0, 3.0], 1.1))] {
        for s in [0.0, 0.5, 3.0, -0.5, -0.9] {
            for m in [10.5, 12.0, 15.0, 30.0] {
                let w = m * e;
                let at = |op: &str| format!("eps {e:e} {pose} s={s} {m}ε {op}");
                let big = block(tol, r, |u, v, w| [-1.0 + 3.0 * u, -1.0 + 3.0 * v, -1.0 + 3.0 * w]);
                let slab = block(tol, r, move |u, v, z| [u, 0.5 + 0.3 * v, 0.5 + w * z]);
                // y0(z) = 0.5 + w + s·(z − 0.5), z in [0.5, 0.5 + w].
                let fill = block(tol, r, move |u, v, z| {
                    let zz = 0.5 + w * z;
                    let y0 = 0.5 + w + s * w * z;
                    [u, y0 + v * (0.8 - y0), zz]
                });
                let area = w * w * (1.0 + s / 2.0);
                let hollow = topo::subtract(&big, &slab, tol).unwrap();
                let hollow = hollow.body().unwrap().body.clone();
                let comp = topo::subtract(&big, &fill, tol).unwrap();
                let comp = comp.body().unwrap().body.clone();
                check(&at("hollow∪fill"), topo::union(&hollow, &fill, tol), 27.0 - area, true, tol);
                check(&at("fill∪hollow"), topo::union(&fill, &hollow, tol), 27.0 - area, true, tol);
                check(&at("slab∖fill"), topo::subtract(&slab, &fill, tol), area, false, tol);
                check(&at("slab∩(big∖fill)"), topo::intersect(&slab, &comp, tol), area, false, tol);
                check(&at("(big∖fill)∩slab"), topo::intersect(&comp, &slab, tol), area, false, tol);
            }
        }
    }
}

/// How well `mass_properties` and the built bar's corners hold the
/// turned bar: the bar built directly, and each op's corners against the
/// channel's.
#[test]
#[ignore = "review probe"]
fn needle_bar_volume_noise() {
    let tol = Tol::witness();
    let e = tol.eps();
    let r = rot([1.0, 2.0, 3.0], 1.1);
    for m in [10.5, 12.0, 15.0, 30.0] {
        let w = m * e;
        let direct = block(tol, r, move |u, v, z| [u, 0.5 + w * v, 0.5 + w * z]);
        let v = topo::mass_properties(&direct, tol).map(|m| m.volume);
        println!("eps {e:e} {m}ε direct bar vol={v:?} want={:e}", w * w);
        let slab = block(tol, r, move |u, v, z| [u, 0.5 + 0.3 * v, 0.5 + w * z]);
        let fill = block(tol, r, move |u, v, z| {
            let y0 = 0.5 + w;
            [u, y0 + v * (0.8 - y0), 0.5 + w * z]
        });
        let big = block(tol, r, |u, v, w| [-1.0 + 3.0 * u, -1.0 + 3.0 * v, -1.0 + 3.0 * w]);
        let comp = topo::subtract(&big, &fill, tol).unwrap().body().unwrap().body.clone();
        let place = |p: [f64; 3]| {
            let at = |k: usize| r[k][0] * p[0] + r[k][1] * p[1] + r[k][2] * p[2];
            [at(0), at(1), at(2)]
        };
        let channel: Vec<[f64; 3]> = (0..8)
            .map(|i| {
                let pick = |bit: usize, lo: f64, hi: f64| if i >> bit & 1 == 0 { lo } else { hi };
                place([pick(0, 0.0, 1.0), pick(1, 0.5, 0.5 + w), pick(2, 0.5, 0.5 + w)])
            })
            .collect();
        for (op, run) in [
            ("slab∖fill", topo::subtract(&slab, &fill, tol)),
            ("slab∩(big∖fill)", topo::intersect(&slab, &comp, tol)),
            ("(big∖fill)∩slab", topo::intersect(&comp, &slab, tol)),
        ] {
            let Ok(res) = run else {
                println!("eps {e:e} {m}ε {op}: ERR");
                continue;
            };
            let b = &res.body().unwrap().body;
            let body: &topo::Body<f64> = b;
            let dev = body
                .vertex_points()
                .map(|(_, p)| {
                    channel
                        .iter()
                        .map(|q| ((p.x - q[0]).powi(2) + (p.y - q[1]).powi(2) + (p.z - q[2]).powi(2)).sqrt())
                        .fold(f64::INFINITY, f64::min)
                })
                .fold(0.0, f64::max);
            let v = topo::mass_properties(b, tol).map(|m| m.volume);
            println!(
                "eps {e:e} {m}ε {op}: verts={} max corner dev={dev:e} vol={v:?} rel={:e}",
                body.vertex_points().count(),
                (v.as_ref().unwrap() - w * w) / (w * w)
            );
        }
    }
}
