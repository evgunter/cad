//! Reviewer R1 probes (local, not committed).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom::Surface;
use geom_brep::{PairSection, SectionRead, SsiLane, SectionError, SsiError};
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn cyl(r: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, -3.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}
fn sph(c: [f64; 3], big: f64) -> Surface<f64> {
    Surface::Sphere {
        center: Point3::from_array(c),
        radius: big,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}
fn built(a: &Surface<f64>, b: &Surface<f64>) -> PairSection<f64> {
    match geom_brep::pair_section(a, b, band()) {
        Ok(SectionRead::Built(s)) => s,
        other => panic!("{other:?}"),
    }
}
/// Analytic loop points of cylinder r=`r` about z, sphere centre (d,0,cz) radius big:
/// returns (sign of height, theta, point).
fn analytic(r: f64, d: f64, cz: f64, big: f64, n: usize) -> Vec<(i8, f64, Point3<f64>)> {
    let mut out = Vec::new();
    for k in 0..n {
        let th = -std::f64::consts::PI + (k as f64 + 0.5) * std::f64::consts::TAU / n as f64;
        let h2 = big * big - r * r - d * d + 2.0 * r * d * th.cos();
        if h2 <= 0.0 {
            continue;
        }
        let h = h2.sqrt();
        for s in [1i8, -1] {
            out.push((s, th, Point3::new(r * th.cos(), r * th.sin(), cz + f64::from(s) * h)));
        }
    }
    out
}

/// component_of / foot agree with the analytic loops, both operand orders,
/// down to near the far tangency; feet are cyclically monotone in theta.
#[test]
fn r1_two_loops_assign_and_order() {
    for (label, d, big) in [
        ("two loops", 0.1, 0.7),
        ("two loops, near node", 0.2, 0.71),
        ("two loops, nearer node", 0.2, 0.7005),
    ] {
        let (c, s) = (cyl(0.5), sph([d, 0.0, 0.3], big));
        for swap in [false, true] {
            let sec = if swap { built(&s, &c) } else { built(&c, &s) };
            assert_eq!(sec.components().len(), 2, "{label}");
            let t0 = std::time::Instant::now();
            let car = sec.carry(Some(SsiLane::marched()), band());
            println!("{label} swap={swap}: carry {:?}", t0.elapsed());
            let car = match car {
                Ok(c) => c,
                Err(e) => {
                    println!("{label} swap={swap}: carry refused {e:?}");
                    continue;
                }
            };
            let mut per: [Vec<(f64, f64)>; 2] = [Vec::new(), Vec::new()];
            let mut misses = 0;
            for (sg, th, p) in analytic(0.5, d, 0.3, big, 200) {
                let want = if sg > 0 { 0 } else { 1 };
                match sec.component_of(p, band()) {
                    Ok(k) => assert_eq!(k, want, "{label} swap={swap}: component_of at {th}"),
                    Err(e) => {
                        misses += 1;
                        println!("{label}: component_of refused at θ={th:.4} {e:?}");
                        continue;
                    }
                }
                match car.foot(p, band()) {
                    Ok(f) => {
                        assert_eq!(f.component, want, "{label} swap={swap}: foot at {th}");
                        per[want].push((th, f.t));
                    }
                    Err(e) => {
                        misses += 1;
                        println!("{label} swap={swap}: foot refused at θ={th:.4} {e:?}");
                    }
                }
            }
            for (k, v) in per.iter().enumerate() {
                let span = car.components()[k].span;
                let period = span.1 - span.0;
                // cyclic monotone: count direction reversals of successive dt (mod period)
                let mut ups = 0;
                let mut downs = 0;
                for w in v.windows(2) {
                    let dt = (w[1].1 - w[0].1).rem_euclid(period);
                    if dt < period / 2.0 { ups += 1 } else { downs += 1 }
                }
                println!("{label} swap={swap} comp {k}: n={} ups={ups} downs={downs} span={span:?}", v.len());
                assert!(ups == 0 || downs == 0, "{label} swap={swap}: feet not monotone along loop {k}");
            }
            println!("{label} swap={swap}: misses {misses}");
        }
    }
}

/// One loop: feet monotone along the analytic loop (tau order), both orders.
#[test]
fn r1_one_loop_feet_follow_travel() {
    for (label, c0, big) in [
        ("centre on wall", [0.5, 0.0, 0.0], 0.3),
        ("centre inside", [0.35, 0.0, 0.1], 0.3),
        ("ball wider than wall (near node)", [0.2, 0.0, 0.0], 0.699),
    ] {
        let (c, s) = (cyl(0.5), sph(c0, big));
        for swap in [false, true] {
            let sec = if swap { built(&s, &c) } else { built(&c, &s) };
            assert_eq!(sec.components().len(), 1, "{label}");
            let t0 = std::time::Instant::now();
            let car = sec.carry(Some(SsiLane::marched()), band());
            println!("{label} swap={swap}: carry {:?}", t0.elapsed());
            let car = car.unwrap_or_else(|e| panic!("{label}: {e:?}"));
            let d = c0[0];
            let r = 0.5;
            let th0 = ((r * r + d * d - big * big) / (2.0 * r * d)).acos();
            // tau parametrisation: up side theta in (-th0, th0), then down side back
            let mut feet = Vec::new();
            let n = 120;
            for k in 0..n {
                let tau = (k as f64 + 0.5) * 4.0 * th0 / n as f64;
                let (th, sg) = if tau < 2.0 * th0 { (tau - th0, 1.0) } else { (3.0 * th0 - tau, -1.0) };
                let h2 = big * big - r * r - d * d + 2.0 * r * d * th.cos();
                if h2 <= 0.0 { continue; }
                let p = Point3::new(r * th.cos(), r * th.sin(), c0[2] + sg * h2.sqrt());
                match car.foot(p, band()) {
                    Ok(f) => feet.push(f.t),
                    Err(e) => println!("{label}: foot refused at tau={tau:.4} {e:?}"),
                }
            }
            let span = car.components()[0].span;
            let period = span.1 - span.0;
            let (mut ups, mut downs) = (0, 0);
            for w in feet.windows(2) {
                let dt = (w[1] - w[0]).rem_euclid(period);
                if dt < period / 2.0 { ups += 1 } else { downs += 1 }
            }
            println!("{label} swap={swap}: n={} ups={ups} downs={downs}", feet.len());
            assert!(ups == 0 || downs == 0, "{label}: not monotone");
        }
    }
}

/// Negative / zero radius, both orders, every pub door.
#[test]
fn r1_degenerate_radius_every_door_both_orders() {
    for (label, c, s) in [
        ("neg sphere off-axis", cyl(0.5), sph([0.4, 0.0, 0.0], -0.3)),
        ("neg cyl off-axis", cyl(-0.5), sph([0.4, 0.0, 0.0], 0.3)),
        ("neg sphere coaxial", cyl(0.5), sph([0.0, 0.0, 0.0], -0.7)),
        ("zero sphere", cyl(0.5), sph([0.4, 0.0, 0.0], 0.0)),
        ("zero cyl", cyl(0.0), sph([0.4, 0.0, 0.0], 0.3)),
    ] {
        for (x, y) in [(&c, &s), (&s, &c)] {
            let r = geom_brep::pair_section(x, y, band());
            assert!(matches!(r, Err(SectionError::DegenerateOperand { .. })), "{label} value: {r:?}");
            let dom = geom_brep::ssi::SsiDomain { center: Point3::new(0.0, 0.0, 0.0), half_extent: 2.0, extent: 2.0, floor_scale: 1.0 };
            let m = geom_brep::ssi::cylinder_sphere_ssi(x, y, dom, band());
            assert!(matches!(m, Err(SsiError::RadiusNotPositive { .. })), "{label} march: {m:?}");
        }
        let cs = geom_brep::cylinder_sphere_section(&c, &s, band());
        assert!(matches!(cs, Err(SectionError::DegenerateOperand { .. })), "{label} C5 arm: {cs:?}");
    }
}

/// Offset conditioning: centre far along the axis, tiny offset.
#[test]
fn r1_offset_form_conditioning() {
    let a = Vec3::new(1.0f64, 2.0, 3.0).normalize();
    let perp = a.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
    for far in [1.0, 1e3, 1e6] {
        for off in [1e-3, 1e-6, 1e-8] {
            let w = a * far + perp * off;
            let new = a.cross(w.cross(a));
            let old = w - a * w.dot(a);
            let ax = |v: Vec3<f64>| v.dot(a).abs() / v.norm();
            println!(
                "far={far:e} off={off:e}: |d| new {:.3e} old {:.3e} (true {off:e}); axial/|v| new {:.2e} old {:.2e}",
                new.norm(), old.norm(), ax(new), ax(old)
            );
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Class {
    Empty,
    Lone,
    Pair,
    Touch,
    Tangent,
    Other(String),
}

/// The retired `section_cert::sphere_cylinder` arm, transcribed (sphere is F).
fn old_arm(e: f64, rc: f64, rho: f64, b: Band) -> Class {
    use geom_core::{Margin, Sign};
    let dec = |n: &'static str, m: f64| geom_core::k_stats::decide(n, Margin::of(m), b);
    let q = (e - rc).abs();
    let rows = [("reach", rho - q), ("girdle", e + rc - rho)];
    let mut out = [false; 2];
    for (i, (n, m)) in rows.into_iter().enumerate() {
        match dec(n, m) {
            Ok(Sign::Positive) => out[i] = true,
            Ok(Sign::Negative) => {}
            Ok(Sign::Zero) => {
                if n == "reach" && dec("girdle", e + rc - rho) == Ok(Sign::Positive) {
                    return Class::Touch;
                }
                return Class::Tangent;
            }
            Err(_) => return Class::Tangent,
        }
    }
    match out {
        [false, _] => Class::Empty,
        [true, false] => Class::Pair,
        [true, true] => Class::Lone,
    }
}

fn new_arm(e: f64, rc: f64, rho: f64, swap: bool, b: Band) -> Class {
    use geom_brep::ComponentShape;
    let c = cyl(rc);
    let s = sph([e, 0.0, 0.0], rho);
    let r = if swap { geom_brep::pair_section(&s, &c, b) } else { geom_brep::pair_section(&c, &s, b) };
    match r {
        Ok(SectionRead::Built(sec)) => {
            if sec.tangency().is_some() {
                return match sec.components() {
                    [c] if c.shape == ComponentShape::Point && c.witness.is_some() => Class::Touch,
                    _ => Class::Tangent,
                };
            }
            match sec.components().len() {
                0 => Class::Empty,
                1 => Class::Lone,
                2 => Class::Pair,
                n => Class::Other(format!("{n}")),
            }
        }
        Err(SectionError::Escalated(_) | SectionError::RadiusEscalated { .. }) => Class::Tangent,
        other => Class::Other(format!("{other:?}")),
    }
}

/// Old arm vs new value over a grid including every boundary.
#[test]
fn r1_guard_matches_the_retired_arm() {
    let b = band();
    let (z, k) = (b.zero(), b.escalate());
    println!("band zero {z:e} escalate {k:e}");
    let rc = 0.5;
    let mut es = vec![0.0, 0.3 * z, z, 2.0 * z, 0.5 * (z + k), k, 1.5 * k, 3.0 * k, 1e-3, 0.1, 0.2, 0.5, 0.7, 1.2];
    es.sort_by(f64::total_cmp);
    let mut diffs = 0;
    let mut n = 0;
    for &e in &es {
        // rho around each boundary: |e-r|, e+r, r (coaxial)
        let mut rhos = Vec::new();
        for base in [(e - rc).abs(), e + rc, rc, 0.3, 0.9] {
            for dd in [-3.0 * k, -k, -0.5 * (z + k), -z, -0.3 * z, 0.0, 0.3 * z, z, 0.5 * (z + k), k, 3.0 * k, 0.05, -0.05] {
                let r = base + dd;
                if r > 3.0 * k {
                    rhos.push(r);
                }
            }
        }
        for rho in rhos {
            let old = old_arm(e, rc, rho, b);
            for swap in [false, true] {
                n += 1;
                let new = new_arm(e, rc, rho, swap, b);
                if old != new {
                    diffs += 1;
                    println!("DIFF e={e:e} rho-|e-r|={:e} rho-(e+r)={:e} rho-r={:e} swap={swap}: old {old:?} new {new:?}", rho - (e - rc).abs(), rho - (e + rc), rho - rc);
                }
            }
        }
    }
    println!("{diffs} differences of {n}");
}
