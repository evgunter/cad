//! Review R2 probe for PR 6497 (not for merge): `cert_spindle` fuzz on
//! the horn, near-horn and wide apples, and wider fan triangles.
#![allow(clippy::unwrap_used, clippy::panic, clippy::print_stderr)]
use core::f64::consts::PI;
use geom_core::Point3;

fn torus_point(major: f64, minor: f64, u: f64, v: f64) -> Point3<f64> {
    let rho = major + minor * v.cos();
    Point3::new(rho * u.cos(), rho * u.sin(), minor * v.sin())
}

fn sheet_dist(major: f64, minor: f64, p: Point3<f64>) -> f64 {
    let (rho, z) = (p.x.hypot(p.y), p.z);
    let v_star = (-major / minor).clamp(-1.0, 1.0).acos();
    let (x, y) = (rho - major, z);
    if y.atan2(x).abs() <= v_star {
        return (x.hypot(y) - minor).abs();
    }
    let h = minor * v_star.sin();
    rho.hypot(z - h).min(rho.hypot(z + h))
}

#[test]
fn probe_cert_spindle_fuzz_wide() {
    let mut rng = test_utils::fuzz::start("r2_cert_spindle");
    let mut worst_ratio: f64 = 0.0;
    let mut fails = 0usize;
    let mut first = None;
    for _ in 0..200_000 {
        let minor = rng.range(0.05, 2.0);
        let pick = rng.unit();
        let major = if pick < 0.2 {
            minor // the horn
        } else if pick < 0.3 {
            minor * (1.0 - 1e-9)
        } else if pick < 0.4 {
            -minor * (1.0 - 1e-6) // the needle lemon
        } else {
            minor * rng.range(-0.999, 0.999)
        };
        let vs = (-major / minor).clamp(-1.0, 1.0).acos();
        let up = rng.unit() < 0.5;
        let at = if up { vs } else { -vs };
        let (du, dv) = (rng.range(0.0, 1.5), rng.range(0.0, vs.min(1.5)));
        let u0 = rng.range(-PI, PI);
        let mut uv = [[rng.range(-PI, PI), at]; 3];
        for corner in &mut uv[1..] {
            let off = rng.unit() * dv;
            *corner = [u0 + rng.unit() * du, if up { at - off } else { at + off }];
        }
        let apex_slot = (rng.unit() * 3.0) as usize % 3;
        uv.swap(0, apex_slot);
        let mut apex = [false; 3];
        apex[apex_slot] = true;
        let cert = mesh::cert::cert_spindle(major, minor, uv, apex);
        if !cert.is_finite() {
            continue;
        }
        let c = uv.map(|[u, v]| torus_point(major, minor, u, v));
        let n = 12u32;
        let mut worst: f64 = 0.0;
        for i in 0..=n {
            for j in 0..=(n - i) {
                let (li, lj) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                let lk = 1.0 - li - lj;
                let q = Point3::new(
                    c[0].x * li + c[1].x * lj + c[2].x * lk,
                    c[0].y * li + c[1].y * lj + c[2].y * lk,
                    c[0].z * li + c[1].z * lj + c[2].z * lk,
                );
                worst = worst.max(sheet_dist(major, minor, q));
            }
        }
        if worst > 1e-12 {
            worst_ratio = worst_ratio.max(worst / cert);
        }
        if worst > cert * (1.0 + 1e-9) + 1e-12 {
            fails += 1;
            if first.is_none() {
                first = Some(format!(
                    "R {major} r {minor} uv {uv:?} apex {apex:?}: {worst} > {cert}"
                ));
            }
        }
    }
    eprintln!("worst dist/cert {worst_ratio}; fails {fails}; first {first:?}");
    assert_eq!(fails, 0, "{first:?}");
}
