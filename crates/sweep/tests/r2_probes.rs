//! Review r2 of PR 4399: probe families past the PR's own poses. One
//! line per run through `differential::outcome`; `R2_MESH=1` also
//! tessellates every body that builds. Run with `--nocapture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, SQRT_2};

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::test_support::{ball_poled_y, ball_poled_z, brick, extruded, finished, sketch_at};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

use crate::common::differential::{every_op_both_orders, outcome};
use crate::common::oracles::{ball_volume, cap_volume};

fn tol() -> Tol {
    Tol::witness()
}

fn line(label: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) -> String {
    let mesh = if std::env::var("R2_MESH").is_ok() {
        match r.as_ref().map(BooleanResult::body) {
            Ok(Some(bb)) => match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                mesh::tessellate(&bb.body, 5e-3, tol())
            })) {
                Err(_) => " mesh=PANIC".to_string(),
                Ok(Ok(m)) => match mesh::validate::check_mesh(&m) {
                    Ok(()) => " mesh=ok".to_string(),
                    Err(e) => format!(" mesh=BADCHECK {e:?}"),
                },
                Ok(Err(e)) => {
                    let s = format!("{e:?}");
                    format!(" mesh=refused {}", s.chars().take(60).collect::<String>())
                }
            },
            _ => String::new(),
        }
    } else {
        String::new()
    };
    let o = outcome(r, want, tol());
    let s = format!("R2 {label}: {o}{mesh}");
    println!("{s}");
    s
}

fn keep(label: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) -> Option<AtRestBody<f64>> {
    let b = r.as_ref().ok().and_then(BooleanResult::body).map(|bb| bb.body.clone());
    line(label, r, want);
    b
}

fn six(name: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, v: (f64, f64, f64)) {
    for (op, r, want) in every_op_both_orders(a, b, v, tol()) {
        line(&format!("{name}, {op}"), r, want);
    }
}

fn rigid(b: &Body<f64>, m: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, m, tol()).unwrap()
}

fn fin(b: Body<f64>) -> AtRestBody<f64> {
    finished("probe operand", b, tol())
}

/// The z tube of radius `a` over `[z0, z0 + len]`.
fn tube(a: f64, z0: f64, len: f64) -> Body<f64> {
    let c = profile::circle(Point2::new(0.0, 0.0), a, tol()).unwrap();
    extruded(sketch_at(z0), vec![c.into()], len, tol())
}

/// An annulus `[ri, ro]` extruded over `[z0, z0 + len]`.
fn pipe(ri: f64, ro: f64, z0: f64, len: f64) -> Body<f64> {
    let v = |x: f64, b: f64| (Point2::new(x, 0.0), b);
    let outer = bulge_loop(vec![v(ro, 1.0), v(-ro, 1.0)]);
    let inner = bulge_loop(vec![v(ri, -1.0), v(-ri, -1.0)]);
    extruded(sketch_at(z0), vec![outer, inner], len, tol())
}

/// A meridian polygon in the sketch xy plane revolved about sketch y.
fn rev_y(pts: &[(f64, f64)]) -> Body<f64> {
    rev_y_bulged(&pts.iter().map(|&(x, y)| (x, y, 0.0)).collect::<Vec<_>>())
}

fn rev_y_bulged(pts: &[(f64, f64, f64)]) -> Body<f64> {
    let p = Profile::new(
        SketchPlane::xy(),
        vec![bulge_loop(
            pts.iter().map(|&(x, y, b)| (Point2::new(x, y), b)).collect(),
        )],
    )
    .validate(tol())
    .unwrap();
    revolve(
        &p,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol(),
    )
    .unwrap()
    .body
}

fn ball_z() -> AtRestBody<f64> {
    fin(ball_poled_z(SQRT_2, Vec3::zero(), tol()))
}

fn place(tilt: f64, turn: f64) -> Affine3<f64> {
    let o = Point3::origin();
    Affine3::rotation_about_axis(o, Vec3::unit_z(), turn)
        * Affine3::rotation_about_axis(o, Vec3::unit_x(), tilt)
}

/// A tube of radius `a`, rim on the √2 ball, rim offset `dz` along its
/// axis, length `len`, placed: (body, volume, shared with the ball).
fn tube_on_ball(a: f64, len: f64, tilt: f64, turn: f64, dz: f64) -> (AtRestBody<f64>, f64, f64) {
    let z0 = (2.0 - a * a).sqrt();
    let t = rigid(&tube(a, z0 + dz, len), &place(tilt, turn));
    let shared = if dz >= 0.0 {
        cap_volume(SQRT_2, SQRT_2 - z0 - dz)
    } else {
        cap_volume(SQRT_2, SQRT_2 - z0) + PI * a * a * (-dz)
    };
    (fin(t), PI * a * a * len, shared)
}

fn frustum_volume(h: f64, r0: f64, r1: f64) -> f64 {
    PI * h * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0
}

#[test]
fn r2_probe_battery() {
    let ball = ball_z();
    let vball = ball_volume(SQRT_2);

    // F1 near the equator, F2 downward, F2b band offsets.
    let tubes: &[(&str, f64, f64, f64, f64, f64)] = &[
        ("F1 a=1.2", 1.2, 2.0, 0.0, 0.0, 0.0),
        ("F1 a=1.4", 1.4, 2.0, 0.0, 0.0, 0.0),
        ("F1 a=1.41", 1.41, 2.0, 0.0, 0.0, 0.0),
        ("F1 a=1.414", 1.414, 2.0, 0.0, 0.0, 0.0),
        ("F1 a=1.4142", 1.4142, 2.0, 0.0, 0.0, 0.0),
        ("F1 a=1.4 tilt .3 turn .7", 1.4, 2.0, 0.3, 0.7, 0.0),
        ("F2 down a=1", 1.0, 2.0, PI, 0.0, 0.0),
        ("F2 down a=0.5", 0.5, 2.0, PI, 0.0, 0.0),
        ("F2 down a=1.4", 1.4, 2.0, PI, 0.0, 0.0),
        ("F2 down tilt pi-.4 turn .7", 1.0, 2.0, PI - 0.4, 0.7, 0.0),
        ("F2 down turn pi/2", 1.0, 2.0, PI, FRAC_PI_2, 0.0),
        ("F2b dz=+1e-12", 1.0, 2.0, 0.0, 0.0, 1e-12),
        ("F2b dz=-1e-12", 1.0, 2.0, 0.0, 0.0, -1e-12),
        ("F2b dz=+3e-10", 1.0, 2.0, 0.0, 0.0, 3e-10),
        ("F2b dz=-3e-10", 1.0, 2.0, 0.0, 0.0, -3e-10),
        ("F2b dz=+1e-7", 1.0, 2.0, 0.0, 0.0, 1e-7),
        ("F2b dz=-1e-7", 1.0, 2.0, 0.0, 0.0, -1e-7),
        ("F2b tilt .5 dz=+3e-10", 0.7, 2.0, 0.5, 0.3, 3e-10),
        ("F2b tilt .5 dz=-3e-10", 0.7, 2.0, 0.5, 0.3, -3e-10),
    ];
    for &(name, a, len, tilt, turn, dz) in tubes {
        let (t, vt, sh) = tube_on_ball(a, len, tilt, turn, dz);
        six(name, &t, &ball, (vt, vball, sh));
    }

    // F3 frusta and cones, revolved about y, against the y-poled ball
    // (their seams share the ball's seam half-plane) and turned off it.
    let bally = fin(ball_poled_y(SQRT_2, Vec3::zero(), tol()));
    let cap1 = cap_volume(SQRT_2, SQRT_2 - 1.0);
    let cones: &[(&str, Vec<(f64, f64)>, f64, f64)] = &[
        (
            "F3 frustum widening",
            vec![(0.0, 1.0), (1.0, 1.0), (1.5, 2.0), (0.0, 2.0)],
            frustum_volume(1.0, 1.0, 1.5),
            cap1,
        ),
        (
            "F3 cone to apex",
            vec![(0.0, 1.0), (1.0, 1.0), (0.0, 3.0)],
            frustum_volume(2.0, 1.0, 0.0),
            cap1,
        ),
        (
            "F3 frustum narrowing slow",
            vec![(0.0, 1.0), (1.0, 1.0), (0.6, 2.0), (0.0, 2.0)],
            frustum_volume(1.0, 1.0, 0.6),
            cap1,
        ),
        (
            "F3 cone tangent to the ball",
            vec![(0.0, 1.0), (1.0, 1.0), (0.0, 2.0)],
            frustum_volume(1.0, 1.0, 0.0),
            cap1,
        ),
        (
            "F3 cone inside the cap",
            vec![(0.0, 1.0), (1.0, 1.0), (0.0, 1.3)],
            frustum_volume(0.3, 1.0, 0.0),
            frustum_volume(0.3, 1.0, 0.0),
        ),
        (
            "F3 frustum downward",
            vec![(0.0, -2.0), (1.5, -2.0), (1.0, -1.0), (0.0, -1.0)],
            frustum_volume(1.0, 1.0, 1.5),
            cap1,
        ),
    ];
    for (name, pts, v, sh) in cones {
        let c = rev_y(pts);
        six(name, &fin(c.clone()), &bally, (*v, vball, *sh));
        let turned = rigid(
            &c,
            &Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), 0.4),
        );
        six(&format!("{name} turned"), &fin(turned), &bally, (*v, vball, *sh));
    }

    // F4 a ball's seam meridians lying in a cone face: the z-poled
    // ball's seams are the great circle in xz; a y-axis frustum through
    // it crosses the sphere there transversally and nowhere else.
    let r0 = 1.5 * SQRT_2;
    let r1 = 0.5 * SQRT_2;
    let fr = rev_y(&[(0.0, -1.0), (r0, -1.0), (r1, 1.0), (0.0, 1.0)]);
    let vfr = frustum_volume(2.0, r0, r1);
    let sh4 = frustum_volume(1.0, SQRT_2, r1) + PI * (2.0 - 1.0 / 3.0);
    six("F4 seam meridian in a cone", &fin(fr.clone()), &ball, (vfr, vball, sh4));
    let fr_t = rigid(
        &fr,
        &Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), 0.3),
    );
    six("F4 seam meridian in a cone, cone turned", &fin(fr_t), &ball, (vfr, vball, sh4));

    // F5 two tubes ending on one ball, sequentially.
    let (up, vu, su) = tube_on_ball(1.0, 2.0, 0.0, 0.0, 0.0);
    let (dn, vd, sd) = tube_on_ball(1.0, 2.0, PI, 0.0, 0.0);
    let (sk, vs, ss) = tube_on_ball(0.3, 1.0, 1.3, FRAC_PI_2, 0.0);
    for (name, second, v2, s2) in [("F5 up+down", &dn, vd, sd), ("F5 up+skew", &sk, vs, ss)] {
        let u1 = topo::union(&ball, &up, tol());
        if let Some(b1) = keep(&format!("{name}: ball ∪ up"), u1, vball + vu - su) {
            line(
                &format!("{name}: (ball ∪ up) ∪ second"),
                topo::union(&b1, second, tol()),
                vball + vu - su + v2 - s2,
            );
            line(
                &format!("{name}: second ∪ (ball ∪ up)"),
                topo::union(second, &b1, tol()),
                vball + vu - su + v2 - s2,
            );
            line(
                &format!("{name}: (ball ∪ up) ∩ second"),
                topo::intersect(&b1, second, tol()),
                s2,
            );
            line(
                &format!("{name}: (ball ∪ up) ∖ second"),
                topo::subtract(&b1, second, tol()),
                vball + vu - su - s2,
            );
            line(
                &format!("{name}: second ∖ (ball ∪ up)"),
                topo::subtract(second, &b1, tol()),
                v2 - s2,
            );
        }
        let d1 = topo::subtract(&ball, &up, tol());
        if let Some(b1) = keep(&format!("{name}: ball ∖ up"), d1, vball - su) {
            line(
                &format!("{name}: (ball ∖ up) ∖ second"),
                topo::subtract(&b1, second, tol()),
                vball - su - s2,
            );
            line(
                &format!("{name}: (ball ∖ up) ∪ second"),
                topo::union(&b1, second, tol()),
                vball - su + v2 - s2,
            );
            line(
                &format!("{name}: (ball ∖ up) ∩ second"),
                topo::intersect(&b1, second, tol()),
                s2,
            );
        }
    }

    // F6 a plane through both: the union cut by half-spaces, and the
    // tube on a half-ball whose flat face its rim crosses.
    let u = topo::union(&up, &ball, tol());
    let vu_all = vball + vu - su;
    if let Some(ub) = keep("F6 tube ∪ ball", u, vu_all) {
        for (name, h, want) in [
            ("x≥0", brick((0.0, 5.0), (-5.0, 5.0), (-5.0, 5.0), tol()), vu_all / 2.0),
            ("y≥0", brick((-5.0, 5.0), (0.0, 5.0), (-5.0, 5.0), tol()), vu_all / 2.0),
            ("z≤2", brick((-5.0, 5.0), (-5.0, 5.0), (-5.0, 2.0), tol()), vu_all - PI),
        ] {
            let h = fin(h);
            line(&format!("F6 (tube ∪ ball) ∩ {name}"), topo::intersect(&ub, &h, tol()), want);
            line(&format!("F6 {name} ∩ (tube ∪ ball)"), topo::intersect(&h, &ub, tol()), want);
            line(&format!("F6 (tube ∪ ball) ∖ {name}"), topo::subtract(&ub, &h, tol()), vu_all - want);
        }
    }
    for (name, h) in [
        ("x≥0", brick((0.0, 5.0), (-5.0, 5.0), (-5.0, 5.0), tol())),
        ("y≥0", brick((-5.0, 5.0), (0.0, 5.0), (-5.0, 5.0), tol())),
    ] {
        let hb = topo::intersect(&ball, &fin(h), tol());
        if let Some(hb) = keep(&format!("F6 half-ball {name}"), hb, vball / 2.0) {
            six(&format!("F6 tube on half-ball {name}"), &up, &hb, (vu, vball / 2.0, su / 2.0));
        }
    }

    // F7 a square prism inscribed in a cylinder: its four vertical
    // edges lie in the cylinder's wall (one-sided line germs).
    let sq = extruded(
        sketch_at(-1.0),
        vec![bulge_loop(vec![
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
            (Point2::new(-1.0, 0.0), 0.0),
            (Point2::new(0.0, -1.0), 0.0),
        ])],
        4.0,
        tol(),
    );
    let cyl = tube(1.0, 0.0, 2.0);
    six("F7 inscribed square in a rod", &fin(sq.clone()), &fin(cyl.clone()), (8.0, 2.0 * PI, 4.0));
    let sq_t = rigid(&sq, &Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), 0.3));
    six("F7 inscribed square turned .3", &fin(sq_t), &fin(cyl), (8.0, 2.0 * PI, 4.0));
    // The frustum's seam rulings, and the tube's seam line, in a brick's
    // face plane through the axis.
    let fr2 = rev_y(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]);
    let vfr2 = frustum_volume(1.0, 1.0, 0.5);
    six(
        "F7 frustum ∩ z≤0 half-space (seam rulings in its plane)",
        &fin(fr2),
        &fin(brick((-5.0, 5.0), (-5.0, 5.0), (-5.0, 0.0), tol())),
        (vfr2, 500.0, vfr2 / 2.0),
    );
    six(
        "F7 tube ∩ y≤0 half-space",
        &fin(tube(1.0, 0.0, 2.0)),
        &fin(brick((-5.0, 5.0), (-5.0, 0.0), (-5.0, 5.0), tol())),
        (2.0 * PI, 500.0, PI),
    );
    six(
        "F7 ball ∩ y≤0 half-space (seam meridians in its plane)",
        &ball,
        &fin(brick((-5.0, 5.0), (-5.0, 0.0), (-5.0, 5.0), tol())),
        (vball, 500.0, vball / 2.0),
    );

    // F8 a tube ending on a torus whose rim plane meets it in a second
    // circle.
    let (rr, r, th) = (2.0_f64, 1.0_f64, PI / 3.0);
    let torus = rev_y_bulged(&[(rr - r, 0.0, 1.0), (rr + r, 0.0, 1.0)]);
    let (a, y0) = (rr + r * th.cos(), r * th.sin());
    let tor_t = rev_y(&[(0.0, y0), (a, y0), (a, 2.0), (0.0, 2.0)]);
    let seg = r * r / 2.0 * (FRAC_PI_2 - (y0 / r).asin()) - y0 / 2.0 * (r * r - y0 * y0).sqrt();
    six(
        "F8 tube ending on a torus",
        &fin(tor_t),
        &fin(torus),
        (PI * a * a * (2.0 - y0), 2.0 * PI * PI * rr * r * r, 4.0 * PI * rr * seg),
    );

    // F9 coaxial walls: a rod in a pipe's bore of its own radius.
    let rod = fin(tube(1.0, 0.0, 4.0));
    let p = fin(pipe(1.0, 2.0, 1.0, 2.0));
    six("F9 rod through a pipe's bore", &rod, &p, (4.0 * PI, 6.0 * PI, 0.0));
    let rod2 = fin(tube(1.0, 0.0, 2.0));
    six("F9 rod ending inside a pipe's bore", &rod2, &p, (2.0 * PI, 6.0 * PI, 0.0));
    let p3 = fin(pipe(1.0, 2.0, 2.0, 1.0));
    six("F9 rod ending at a pipe's bore rim", &rod2, &p3, (2.0 * PI, 3.0 * PI, 0.0));
}
