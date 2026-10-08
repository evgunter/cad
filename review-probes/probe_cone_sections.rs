//! REVIEW PROBE (PR 4343 review, not for merge): exact oblique sections
//! of one cone nappe against the conic × quadric door.
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp, clippy::print_stderr)]
use super::*;
use geom_core::{Point3, Tol, Vec3};

fn unit_perp(a: Vec3<f64>) -> Vec3<f64> {
    let k = if a.x.abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
    a.cross(k).normalize()
}

#[test]
#[ignore = "review probe"]
fn probe_oblique_cone_sections() {
    let mut rng = test_utils::fuzz::start("review::cone_sections");
    let unit = |rng: &mut test_utils::fuzz::Rng| loop {
        let v = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
        let n = v.norm();
        if (1e-3..=1.0).contains(&n) { break v / n; }
    };
    let band = Band::linear_at(Tol::witness(), 1e-9).unwrap();
    let (mut total, mut unc, mut on, mut other) = (0, 0, 0, 0);
    let mut by_km = [[0usize; 2]; 9];
    for scale in [1e-3, 1.0, 1e3] {
        for _ in 0..400 {
            let alpha: f64 = rng.range(0.1, 1.4);
            let k = alpha.tan();
            let km = rng.range(0.0, 0.9);
            let m = km / k;
            let z0 = scale * rng.range(1.0, 2.0);
            // the cone frame
            let a = unit(&mut rng);
            let ex = unit_perp(a);
            let ey = a.cross(ex);
            let apex = Point3::new(0.0, 0.0, 0.0) + unit(&mut rng) * scale * rng.range(0.0, 3.0);
            let big_a = 1.0 - km * km;
            let xc = k * k * m * z0 / big_a;
            let s = (1.0 + m * m).sqrt();
            let maj = k * z0 * s / big_a;
            let min = k * z0 / big_a.sqrt();
            let center = apex + ex * xc + a * (z0 + m * xc);
            let normal = (a - ex * m) / s;
            let u = (ex + a * m) / s;
            let (major, minor, u_ref) = if maj >= min { (maj, min, u) } else { (min, maj, normal.cross(u)) };
            let carrier = geom::Curve3::Ellipse { center, axis: normal, major, minor, u_ref };
            let cone = geom::Surface::Cone { apex, axis: a, half_angle: alpha, u_ref: ex };
            let got = conic_quadric_roots(&carrier, 0.3, 2.1, &cone, band);
            total += 1;
            let bucket = ((km / 0.1) as usize).min(8);
            by_km[bucket][0] += 1;
            match got {
                Ok(CircleRoots::OnSurface) => on += 1,
                Ok(CircleRoots::Uncertain) => { unc += 1; by_km[bucket][1] += 1; }
                ref g => { other += 1; eprintln!("other: scale {scale} alpha {alpha} km {km}: {g:?}"); }
            }
        }
    }
    eprintln!("CONE PROBE total {total} OnSurface {on} Uncertain {unc} other {other}");
    for (i, [n, u]) in by_km.iter().enumerate() {
        eprintln!("  k·m in [{:.1},{:.1}): {u}/{n} Uncertain", i as f64 * 0.1, (i + 1) as f64 * 0.1);
    }
}
