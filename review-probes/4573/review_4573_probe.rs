//! Reviewer probe (PR 4573): exact identities on ball/box booleans with
//! the ball's chart pole swept over many directions.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled, brick, finished};
use topo::AtRestBody;

fn pole(i: usize) -> Vec3<f64> {
    // golden-spiral directions
    let n = 40.0;
    let k = i as f64 + 0.5;
    let z = 1.0 - 2.0 * k / n;
    let r = (1.0 - z * z).sqrt();
    let phi = k * 2.399963229728653;
    Vec3::new(r * phi.cos(), r * phi.sin(), z)
}

fn run(op: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Result<(f64, f64), String> {
    let tol = Tol::witness();
    let none = topo::BooleanDeclarations::none();
    let r = match op {
        "i" => topo::intersect_with(a, b, &none, tol),
        "s" => topo::subtract_with(a, b, &none, tol),
        _ => topo::subtract_with(b, a, &none, tol),
    }
    .map_err(|e| format!("bool {e:?}"))?;
    let body = r.body().ok_or("no body")?.body.clone();
    let mp = topo::mass_properties(&body, tol).map_err(|e| format!("mp {e:?}"))?;
    let mi = topo::mass_properties(&topo::Body::clone(&body), tol);
    let _ = mi;
    Ok((mp.volume, mp.surface_area))
}

fn sweep_case(name: &str, rr: f64, c: (f64, f64, f64), boxx: (f64, f64), tilt: f64) {
    let tol = Tol::witness();
    let bx = brick(boxx, boxx, boxx, tol);
    let bx = topo::transform_rigid(
        &bx,
        &Affine3::rotation_about_axis(Point3::new(0.5, 0.5, 0.5), Vec3::new(1.0, 2.0, 3.0), tilt),
        tol,
    )
    .unwrap();
    let bx = finished("box", bx, tol);
    let boxv = (boxx.1 - boxx.0).powi(3);
    let ballv = 4.0 / 3.0 * core::f64::consts::PI * rr.powi(3);
    let mut inter = vec![];
    for i in 0..40 {
        let p = pole(i);
        let ball = ball_poled(rr, Vec3::new(c.0, c.1, c.2), p, tol);
        let ball = finished("ball", ball, tol);
        let a = run("i", &bx, &ball);
        let s = run("s", &bx, &ball);
        let t = run("t", &bx, &ball);
        match (&a, &s, &t) {
            (Ok(a), Ok(s), Ok(t)) => {
                let e1 = (a.0 + s.0 - boxv) / boxv;
                let e2 = (a.0 + t.0 - ballv) / ballv;
                println!("{name} pole {i:2} ({:+.3},{:+.3},{:+.3}): I={:.15e} box-id {e1:+.2e} ball-id {e2:+.2e} area {:.12e}", p.x, p.y, p.z, a.0, a.1);
                inter.push((a.0, a.1));
            }
            _ => println!("{name} pole {i:2}: I={a:?} S={s:?} T={t:?}"),
        }
    }
    let (mn, mx) = inter.iter().fold((f64::MAX, f64::MIN), |(l, h), v| (l.min(v.0), h.max(v.0)));
    let (amn, amx) = inter.iter().fold((f64::MAX, f64::MIN), |(l, h), v| (l.min(v.1), h.max(v.1)));
    println!("{name}: {} measured; vol spread {:.2e} rel; area spread {:.2e} rel", inter.len(), (mx - mn) / mx, (amx - amn) / amx);
}

#[test]
fn probe_corner_ball() {
    sweep_case("corner", 0.7, (1.0, 1.0, 1.0), (0.0, 1.0), 0.0);
}
#[test]
fn probe_corner_ball_tilted_box() {
    sweep_case("cornertilt", 0.7, (1.0, 1.0, 1.0), (0.0, 1.0), 0.37);
}
#[test]
fn probe_edge_ball() {
    sweep_case("edge", 0.45, (0.5, 1.0, 1.0), (0.0, 1.0), 0.0);
}
#[test]
fn probe_face_dimple() {
    sweep_case("face", 0.3, (0.5, 0.45, 1.1), (0.0, 1.0), 0.0);
}

#[test]
fn probe_rod_through_ball() {
    // a thin square rod through the ball: sphere faces carry rings
    let tol = Tol::witness();
    let rod = finished("rod", brick((-0.1, 0.15), (-0.12, 0.09), (-2.0, 2.0), tol), tol);
    let rodv = 0.25 * 0.21 * 4.0;
    let ballv = 4.0 / 3.0 * core::f64::consts::PI;
    for i in 0..40 {
        let p = pole(i);
        let ball = finished("ball", ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), p, tol), tol);
        let a = run("i", &rod, &ball);
        let s = run("s", &rod, &ball);
        let t = run("t", &rod, &ball);
        match (&a, &s, &t) {
            (Ok(a), Ok(s), Ok(t)) => println!(
                "rod pole {i:2}: I={:.15e} rod-id {:+.2e} ball-id {:+.2e} areaT {:.12e}",
                a.0, (a.0 + s.0 - rodv) / rodv, (a.0 + t.0 - ballv) / ballv, t.1
            ),
            _ => println!("rod pole {i:2}: I={a:?} S={s:?} T={t:?}"),
        }
    }
}

#[test]
fn probe_thin_cap_precision() {
    let tol = Tol::witness();
    let r = 1.0;
    for &h in &[1e-1, 1e-2, 1e-3, 1e-4, 1e-5] {
        for (pname, p) in [("axial", Vec3::new(0.0, 0.0, 1.0)), ("tilted", Vec3::new(0.3, 0.2, 0.93))] {
            let ball = finished("ball", ball_poled(r, Vec3::new(0.0, 0.0, 0.0), p, tol), tol);
            let slab = finished("slab", brick((-2.0, 2.0), (-2.0, 2.0), (1.0 - h, 3.0), tol), tol);
            match run("i", &slab, &ball) {
                Ok((v, a)) => {
                    let ev = core::f64::consts::PI * h * h * (3.0 * r - h) / 3.0;
                    let ea = 2.0 * core::f64::consts::PI * r * h + core::f64::consts::PI * (2.0 * r * h - h * h);
                    println!("cap h={h:.0e} {pname}: vol rel {:+.3e} area rel {:+.3e}", (v - ev) / ev, (a - ea) / ea);
                }
                Err(e) => println!("cap h={h:.0e} {pname}: {e}"),
            }
        }
    }
}

#[test]
fn probe_flipped_holed_and_one_loop_faces() {
    let tol = Tol::witness();
    let none = topo::BooleanDeclarations::none();
    let rod = finished("rod", brick((-0.1, 0.15), (-0.12, 0.09), (-2.0, 2.0), tol), tol);
    let mut tally = std::collections::BTreeMap::<String, u32>::new();
    for i in 0..40 {
        let p = pole(i);
        let ball = finished("ball", ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), p, tol), tol);
        let body = topo::subtract_with(&ball, &rod, &none, tol).unwrap().body().unwrap().body.clone();
        for (k, f) in body.faces() {
            if !matches!(body.get_surface(f.surface), Some(geom::Surface::Sphere { .. })) { continue; }
            let kind = if f.rings.is_empty() { "one-loop" } else { "holed" };
            let flipped = body.flipped_face_sense_for_tests(k).unwrap();
            let v = topo::validate_geometric(&flipped, tol);
            let mp = topo::mass_properties(&flipped, tol);
            let key = format!(
                "{kind}: validate={} mp={}",
                match &v { Ok(()) => "OK(!)".to_string(), Err(e) => format!("{:?}", e.iter().map(|x| format!("{x:?}").split(' ').next().unwrap().to_string()).collect::<Vec<_>>()) },
                match &mp { Ok(m) => format!("Ok(vol {:.6})", m.volume), Err(topo::MassPropsError::Face { source, .. }) => format!("{source:?}"), Err(e) => format!("{e:?}").chars().take(60).collect() }
            );
            *tally.entry(key).or_default() += 1;
        }
    }
    for (k, v) in &tally { println!("{v:4} {k}"); }
}

#[test]
fn probe_dimple_chart() {
    let tol = Tol::witness();
    let none = topo::BooleanDeclarations::none();
    let slab = finished("slab", brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol), tol);
    let at = sweep::test_support::ball_poled_y(0.6, Vec3::new(0.0, 0.0, 0.0), tol);
    let moved = topo::transform_rigid(&at, &Affine3::translation(Vec3::new(2.0, 2.0, 1.3)), tol).unwrap();
    let ball = finished("ball", moved, tol);
    let body = topo::subtract_with(&slab, &ball, &none, tol).unwrap().body().unwrap().body.clone();
    for (k, f) in body.faces() {
        if let Some(geom::Surface::Sphere { center, axis, .. }) = body.get_surface(f.surface) {
            println!("dimple face {k:?} centre {center:?} axis {axis:?} sense {}", f.sense);
            let (edges, _) = topo::props::loop_edges(&body, f.outer).unwrap();
            for e in &edges { println!("   edge t {}..{} fwd {} carrier {:?}", e.t0, e.t1, e.forward, e.carrier); }
        }
    }
}

#[test]
fn probe_large_ball_union() {
    let tol = Tol::witness();
    let none = topo::BooleanDeclarations::none();
    let d = 51.0 - 2e-6;
    let a = finished("a", ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), tol), tol);
    let b = finished("b", ball_poled(50.0, Vec3::new(0.0, 0.0, d), Vec3::new(0.0, 1.0, 0.0), tol), tol);
    let bv = |r: f64| 4.0 / 3.0 * core::f64::consts::PI * r * r * r;
    let (r1, r2) = (1.0f64, 50.0f64);
    let lens = core::f64::consts::PI * (r1 + r2 - d).powi(2)
        * (d * d + 2.0 * d * r2 - 3.0 * r2 * r2 + 2.0 * d * r1 + 6.0 * r2 * r1 - 3.0 * r1 * r1) / (12.0 * d);
    let want = bv(1.0) + bv(50.0) - lens;
    for (n, out) in [("a∪b", topo::union_with(&a, &b, &none, tol)), ("b∪a", topo::union_with(&b, &a, &none, tol))] {
        match out {
            Ok(r) => {
                let body = r.body().unwrap().body.clone();
                let v = topo::validate_geometric(&body, tol);
                let mp = topo::mass_properties(&body, tol).map(|m| m.volume);
                println!("{n}: validate {:?} vol {:?} want {want} err {:?} lens {lens:e}", v.is_ok(), mp, mp.as_ref().map(|x| (x - want) / want));
            }
            Err(e) => println!("{n}: Err {e:?}"),
        }
    }
}
