//! Review 6497 (r1): `cert_spindle` fuzzed beyond the PR's own range —
//! the horn and near-horn, wide apples, and fans as wide as a coarse
//! pole column (`Δu` up to 2), `Δv` up to the apex window.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Point3;
use mesh::cert::cert_spindle;

fn p(major: f64, minor: f64, u: f64, v: f64) -> Point3<f64> {
    let rho = major + minor * v.cos();
    Point3::new(rho * u.cos(), rho * u.sin(), minor * v.sin())
}

fn sheet_dist(major: f64, minor: f64, q: Point3<f64>) -> f64 {
    let (rho, z) = (q.x.hypot(q.y), q.z);
    let v_star = (-major / minor).clamp(-1.0, 1.0).acos();
    let (x, y) = (rho - major, z);
    if y.atan2(x).abs() <= v_star {
        return (x.hypot(y) - minor).abs();
    }
    let h = minor * v_star.sin();
    rho.hypot(z - h).min(rho.hypot(z + h))
}

struct Lcg(u64);
impl Lcg {
    fn unit(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        #[allow(clippy::cast_precision_loss)]
        let x = (self.0 >> 11) as f64 / (1u64 << 53) as f64;
        x
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.unit()
    }
}

fn fuzz(label: &str, ratio: (f64, f64), du_max: f64, same_row: bool) -> (usize, f64) {
    let mut rng = Lcg(0x6497);
    let (mut fails, mut worst_ratio) = (0, 0.0_f64);
    for _ in 0..40_000 {
        let minor = rng.range(0.05, 2.0);
        let major = minor * rng.range(ratio.0, ratio.1);
        let vs = (-major / minor).clamp(-1.0, 1.0).acos();
        let up = rng.unit() < 0.5;
        let at = if up { vs } else { -vs };
        let du = rng.range(0.0, du_max);
        let dv = rng.range(0.0, vs.min(1.0));
        let u0 = rng.range(-3.0, 3.0);
        let row = rng.unit() * dv;
        let mut uv = [[rng.range(-3.0, 3.0), at]; 3];
        for (k, corner) in uv[1..].iter_mut().enumerate() {
            let off = if same_row { row } else { rng.unit() * dv };
            let u = if same_row {
                u0 + du * k as f64
            } else {
                u0 + rng.unit() * du
            };
            *corner = [u, if up { at - off } else { at + off }];
        }
        let cert = cert_spindle(major, minor, uv, [true, false, false]);
        let c = uv.map(|[u, v]| p(major, minor, u, v));
        let n = 10u32;
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
        if !(worst <= cert * (1.0 + 1e-9) + 1e-12) {
            if fails < 3 {
                eprintln!("{label}: R {major} r {minor} uv {uv:?}: {worst} > {cert}");
            }
            fails += 1;
        }
    }
    eprintln!("{label}: {fails} fails, worst distance/cert {worst_ratio}");
    (fails, worst_ratio)
}

#[test]
fn r1_cert_spindle_wider_fuzz() {
    let mut bad = vec![];
    for (label, ratio, du, row) in [
        ("lemon+apple as PR, du 2", (-0.98, 0.98), 2.0, false),
        ("horn", (1.0, 1.0), 0.6, false),
        ("horn du 2", (1.0, 1.0), 2.0, false),
        ("near horn", (0.98, 1.0), 2.0, false),
        ("slim lemon", (-1.0, -0.98), 2.0, false),
        ("fan rows", (-0.98, 1.0), 2.0, true),
    ] {
        let (f, _) = fuzz(label, ratio, du, row);
        if f > 0 {
            bad.push(label);
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}
