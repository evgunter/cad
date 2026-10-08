//! JOIN lane probe (temporary): every pose class through `outcome`, face census and mesh.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::PI;

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished};
use topo::{AtRestBody, BooleanDeclarations, BooleanError, BooleanResult};

use crate::common::differential::outcome;

fn ballv(r: f64) -> f64 {
    4.0 / 3.0 * PI * r.powi(3)
}

fn lens(big: f64, small: f64, d: f64) -> f64 {
    if d >= big + small {
        return 0.0;
    }
    if d <= (big - small).abs() {
        return ballv(big.min(small));
    }
    PI * (big + small - d).powi(2)
        * (d * d + 2.0 * d * small - 3.0 * small * small + 2.0 * d * big + 6.0 * small * big
            - 3.0 * big * big)
        / (12.0 * d)
}

fn capv(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

fn place(at: topo::Body<f64>, m: Affine3<f64>, c: Vec3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let at = topo::transform_rigid(&at, &m, tol).unwrap();
    finished("b", topo::transform_rigid(&at, &Affine3::translation(c), tol).unwrap(), tol)
}

fn ball(r: f64, c: Vec3<f64>) -> AtRestBody<f64> {
    place(ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), Tol::witness()), Affine3::identity(), c)
}

/// A `y`-poled ball turned about `z` by `spin`, then local `z` onto `dir`, at `dir·dist`.
fn toward(r: f64, dist: f64, dir: Vec3<f64>, spin: f64) -> AtRestBody<f64> {
    let dir = dir / dir.norm();
    let z = Vec3::new(0.0, 0.0, 1.0);
    let o = Point3::origin();
    let s = Affine3::rotation_about_axis(o, z, spin);
    let ax = z.cross(dir);
    let m = if ax.norm() < 1e-15 {
        if dir.z > 0.0 { Affine3::identity() } else { Affine3::rotation_about_axis(o, Vec3::new(1.0, 0.0, 0.0), PI) }
    } else {
        Affine3::rotation_about_axis(o, ax / ax.norm(), z.dot(dir).clamp(-1.0, 1.0).acos())
    };
    place(ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), Tol::witness()), m * s, dir * dist)
}

fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished("box", brick(x, y, z, Tol::witness()), Tol::witness())
}

fn shape(r: &Result<BooleanResult<f64>, BooleanError>) -> String {
    let tol = Tol::witness();
    let Ok(res) = r else { return String::new() };
    let Some(bb) = res.body() else { return String::new() };
    let body = &bb.body;
    let mut spheres: Vec<(i64, usize)> = Vec::new();
    for (_, fd) in body.faces() {
        if let Some(geom::Surface::Sphere { radius, .. }) = body.get_surface(fd.surface) {
            let k = (radius * 1e6).round() as i64;
            match spheres.iter_mut().find(|(s, _)| *s == k) {
                Some((_, n)) => *n += 1,
                None => spheres.push((k, 1)),
            }
        }
    }
    let mesh = match mesh::tessellate(body, 5e-3, tol) {
        Ok(m) => match mesh::validate::check_mesh(&m) {
            Ok(()) => "ok".to_string(),
            Err(e) => format!("BAD {e:?}"),
        },
        Err(e) => format!("ERR {:.60}", format!("{e:?}")),
    };
    format!(
        "F{} E{} V{} sph{:?} skip{} mesh={mesh}",
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
        spheres.iter().map(|(k, n)| (*k as f64 / 1e6, *n)).collect::<Vec<_>>(),
        bb.naming.merge_skipped.len()
    )
}

fn six(pose: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, (va, vb, sh): (f64, f64, f64)) {
    let (tol, none) = (Tol::witness(), BooleanDeclarations::none());
    for (op, want, r) in [
        ("a∪b", va + vb - sh, topo::union_with(a, b, &none, tol)),
        ("b∪a", va + vb - sh, topo::union_with(b, a, &none, tol)),
        ("a∖b", va - sh, topo::subtract_with(a, b, &none, tol)),
        ("b∖a", vb - sh, topo::subtract_with(b, a, &none, tol)),
        ("a∩b", sh, topo::intersect_with(a, b, &none, tol)),
        ("b∩a", sh, topo::intersect_with(b, a, &none, tol)),
    ] {
        let sh = shape(&r);
        let o = outcome(r, want, tol);
        let o: String = o.split(" t2=").next().unwrap().chars().take(120).collect();
        println!("ROUTE {pose} | {op} | {o} | {sh}");
    }
}

fn unit() -> AtRestBody<f64> {
    ball(1.0, Vec3::new(0.0, 0.0, 0.0))
}

#[test]
#[ignore = "probe"]
fn route_probe() {
    let tol = Tol::witness();
    let z = Vec3::new(0.0, 0.0, 1.0);
    six("plain", &unit(), &ball(0.3, Vec3::new(0.0, 0.0, 0.95)), (ballv(1.0), ballv(0.3), lens(1.0, 0.3, 0.95)));
    let lu = topo::union(&ball(1.0, Vec3::new(2.0, 2.0, 0.5)), &ball(1.0, Vec3::new(3.4, 2.0, 0.5)), tol).unwrap().body().unwrap().body.clone();
    six("lens-union", &lu, &ball(0.3, Vec3::new(2.0, 2.0, 1.45)), (2.0 * ballv(1.0) - lens(1.0, 1.0, 1.4), ballv(0.3), lens(1.0, 0.3, 0.95)));
    for (r, d) in [(0.05, 1.0), (0.7, 1.0), (1.5, 1.5), (1.0, 1.0)] {
        six(&format!("line r{r} d{d}"), &unit(), &ball(r, Vec3::new(0.0, 0.0, d)), (ballv(1.0), ballv(r), lens(1.0, r, d)));
    }
    let c = Vec3::new(0.02, 0.01, 0.9);
    six("tilted", &unit(), &ball(0.3, c), (ballv(1.0), ballv(0.3), lens(1.0, 0.3, c.norm())));
    let o = Vec3::new(0.0, 0.0, 0.0);
    let big = topo::subtract(&ball(1.0, o), &boxed((0.9, 2.0), (-2.0, 2.0), (-2.0, 2.0)), tol).unwrap().body().unwrap().body.clone();
    let small = topo::subtract(&ball(0.3, Vec3::new(0.0, 0.0, 1.2)), &boxed((0.25, 2.0), (-2.0, 2.0), (-2.0, 3.0)), tol).unwrap().body().unwrap().body.clone();
    six("trimmed2", &big, &small, (ballv(1.0) - capv(1.0, 0.1), ballv(0.3) - capv(0.3, 0.05), lens(1.0, 0.3, 1.2)));
    let on_y = |r: f64, y: f64| ball(r, Vec3::new(0.0, y, 0.0));
    let lz = topo::intersect(&on_y(1.0, 0.0), &on_y(0.8, 1.4), tol).unwrap().body().unwrap().body.clone();
    let (s20, c20) = 20f64.to_radians().sin_cos();
    let dir = Vec3::new(0.0, c20, s20);
    let to_top = (dir * 0.93 - Vec3::new(0.0, 1.4, 0.0)).norm();
    six("orbit-lens", &lz, &toward(0.2, 0.93, dir, 0.0), (lens(1.0, 0.8, 1.4), ballv(0.2), lens(1.0, 0.2, 0.93) + lens(0.8, 0.2, to_top) - ballv(0.2)));
    for spin in [0.5, 1.6] {
        six(&format!("nonpar spin{spin}"), &unit(), &toward(0.3, 0.95, z, spin), (ballv(1.0), ballv(0.3), lens(1.0, 0.3, 0.95)));
    }
    let tdir = Vec3::new(0.3, 0.2, 0.93);
    six("nonpar tilt-line", &unit(), &toward(0.3, 0.95, tdir, 0.7), (ballv(1.0), ballv(0.3), lens(1.0, 0.3, 0.95)));
    // Near a pole of the unit ball (y): a small ball whose circle passes δ from the pole.
    let r = 0.01f64;
    let theta = (1.0 - r * r / 2.0).acos();
    for delta in [1e-5, 1e-7] {
        let a = theta + delta;
        six(&format!("pole δ{delta:e}"), &unit(), &toward(r, 1.0, Vec3::new(0.0, a.cos(), a.sin()), 0.0), (ballv(1.0), ballv(r), lens(1.0, r, 1.0)));
    }
    six("tiny r1e-3 on sphere", &unit(), &toward(1e-3, 1.0, z, 0.0), (ballv(1.0), ballv(1e-3), lens(1.0, 1e-3, 1.0)));
    for (name, d) in [("r50 int", 49.0 + 2e-6), ("r50 ext", 51.0 - 2e-6)] {
        six(name, &unit(), &ball(50.0, Vec3::new(0.0, 0.0, d)), (ballv(1.0), ballv(50.0), lens(1.0, 50.0, d)));
    }
    six("r1000", &unit(), &ball(1000.0, Vec3::new(0.0, 0.0, 1000.5)), (ballv(1.0), ballv(1000.0), lens(1.0, 1000.0, 1000.5)));
    let two = |d2: Vec3<f64>| topo::union(&toward(0.3, 0.95, z, 0.0), &toward(0.3, 0.95, d2, 0.0), tol).unwrap().body().unwrap().body.clone();
    six("three +z-z", &unit(), &two(-z), (ballv(1.0), 2.0 * ballv(0.3), 2.0 * lens(1.0, 0.3, 0.95)));
    let tilt = Vec3::new(40f64.to_radians().sin(), 0.0, 40f64.to_radians().cos());
    let _ = two;
    let t2 = topo::union(&toward(0.3, 0.95, z, 0.0), &toward(0.3, 0.95, -tilt, 0.0), tol).unwrap().body().unwrap().body.clone();
    six("three +z,-tilt", &unit(), &t2, (ballv(1.0), 2.0 * ballv(0.3), 2.0 * lens(1.0, 0.3, 0.95)));
    let slab_ball = topo::union(&boxed((-3.0, 3.0), (-3.0, 3.0), (0.5, 3.0)), &ball(0.3, Vec3::new(0.0, 0.0, -0.95)), tol).unwrap().body().unwrap().body.clone();
    six("plane+sphere par", &unit(), &slab_ball, (ballv(1.0), 90.0 + ballv(0.3), capv(1.0, 0.5) + lens(1.0, 0.3, 0.95)));
    let slab_x = topo::union(&boxed((-3.0, 3.0), (-3.0, 3.0), (0.5, 3.0)), &toward(0.3, 0.95, Vec3::new(1.0, 0.0, -0.2), 0.0), tol).unwrap().body().unwrap().body.clone();
    six("plane+sphere nonpar", &unit(), &slab_x, (ballv(1.0), 90.0 + ballv(0.3), capv(1.0, 0.5) + lens(1.0, 0.3, 0.95)));
}

#[test]
#[ignore = "probe"]
fn route_r50() {
    let (tol, none) = (Tol::witness(), BooleanDeclarations::none());
    for (name, d) in [("r50 int", 49.0 + 2e-6), ("r50 ext", 51.0 - 2e-6)] {
        let (a, b) = (unit(), ball(50.0, Vec3::new(0.0, 0.0, d)));
        let sh = lens(1.0, 50.0, d);
        for (op, want, r) in [
            ("a∪b", ballv(1.0) + ballv(50.0) - sh, topo::union_with(&a, &b, &none, tol)),
            ("a∖b", ballv(1.0) - sh, topo::subtract_with(&a, &b, &none, tol)),
            ("a∩b", sh, topo::intersect_with(&a, &b, &none, tol)),
        ] {
            if let Err(e) = &r { println!("R50 {name} {op} ERR {e:?}"); continue; }
            let bb = r.as_ref().unwrap().body().unwrap().body.clone();
            let far = finished("far", brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol), tol);
            println!("R50 {name} {op} far-union {:?}", topo::union(&bb, &far, tol).err());
            println!("R50 {name} {op} {}", outcome(r, want, tol));
        }
    }
}

#[test]
#[ignore = "probe"]
fn route_interval() {
    use crate::common::interval::iv;
    let tol = Tol::witness();
    let ball_iv = |r: f64, z: f64| finished("b", ball_poled_y(iv(r), Vec3::new(iv(0.0), iv(0.0), iv(z)), tol), tol);
    let (a, b) = (ball_iv(1.0, 0.0), ball_iv(0.3, 0.95));
    for (op, out) in [
        ("a∪b", topo::union(&a, &b, tol)),
        ("a∖b", topo::subtract(&a, &b, tol)),
        ("b∖a", topo::subtract(&b, &a, tol)),
        ("a∩b", topo::intersect(&a, &b, tol)),
    ] {
        println!("IV eps={} {op} {}", tol.eps(), match out { Ok(_) => "built".to_string(), Err(e) => format!("{:.200}", format!("{e:?}")) });
    }
}
