//! **Adopted blinded-review probes for M6 unit 1's rider** (charter
//! E), adopted verbatim per the standing review policy: falsification
//! attempt on `circle_residual_extremes` — the enclosure must never
//! be beaten by dense sampling (a bound below truth would be a MAJOR
//! unsoundness), and a positive `max(lo, -hi)` margin must imply the
//! circle never changes side across the whole carrier (the semantics
//! the boolean's definite-miss arm rests on). 3000 + 20000 seeded
//! pairs, spheres and cylinders, adversarial near-tangent families.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{circle_residual_extremes, implicit_residual};
use geom_core::{Point3, Vec3};
use geom_surfaces::Surface;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
            );
            let n = v.norm();
            if n > 1e-3 {
                return v / n;
            }
        }
    }
}

fn frame(rng: &mut Rng) -> (Vec3<f64>, Vec3<f64>) {
    let a = rng.unit();
    let mut u = rng.unit();
    u = (u - a * a.dot(u)).normalize();
    (a, u)
}

#[test]
fn enclosure_never_beaten_by_dense_sampling() {
    let mut rng = Rng(0x1234_5678_9ABC_DEF1);
    let n_samples = 20_000usize;
    let mut worst_slack = f64::INFINITY;
    for case in 0..3000 {
        let (axis, u_ref) = frame(&mut rng);
        let center = Point3::new(
            rng.range(-2.0, 2.0),
            rng.range(-2.0, 2.0),
            rng.range(-2.0, 2.0),
        );
        let radius = rng.range(1e-4, 3.0);
        let s: Surface<f64> = match case % 3 {
            0 => {
                let (sa, _) = frame(&mut rng);
                Surface::Sphere {
                    center: Point3::new(
                        rng.range(-2.0, 2.0),
                        rng.range(-2.0, 2.0),
                        rng.range(-2.0, 2.0),
                    ),
                    radius: rng.range(1e-3, 2.5),
                    axis: sa,
                    u_ref: Vec3::unit_x(),
                }
            }
            1 => {
                let (ca, _) = frame(&mut rng);
                Surface::Cylinder {
                    origin: Point3::new(
                        rng.range(-2.0, 2.0),
                        rng.range(-2.0, 2.0),
                        rng.range(-2.0, 2.0),
                    ),
                    axis: ca,
                    radius: rng.range(1e-3, 2.5),
                    u_ref: Vec3::unit_x(),
                }
            }
            _ => {
                // adversarial cylinder families: near-coaxial, contained,
                // near-tangent
                let jitter = rng.range(0.0, 0.05);
                let ca = (axis + Vec3::new(jitter, jitter / 2.0, 0.0)).normalize();
                Surface::Cylinder {
                    origin: center + axis * rng.range(-0.5, 0.5) + u_ref * rng.range(0.0, 0.02),
                    axis: ca,
                    radius: if case % 2 == 0 {
                        radius // near-tangent internal
                    } else {
                        rng.range(1e-3, 2.5)
                    },
                    u_ref: Vec3::unit_x(),
                }
            }
        };
        let Some((lo, hi)) = circle_residual_extremes(&s, center, axis, radius, u_ref) else {
            panic!("closed form expected for sphere/cylinder");
        };
        assert!(lo <= hi, "malformed range case {case}: [{lo}, {hi}]");
        let v = axis.cross(u_ref);
        let mut smin = f64::INFINITY;
        let mut smax = f64::NEG_INFINITY;
        for i in 0..n_samples {
            let t = core::f64::consts::TAU * (i as f64) / (n_samples as f64);
            let p = center + (u_ref * t.cos() + v * t.sin()) * radius;
            let r = implicit_residual(&s, p);
            smin = smin.min(r);
            smax = smax.max(r);
        }
        let tol = 1e-9 * (1.0 + smax.abs().max(smin.abs()));
        assert!(
            lo <= smin + tol,
            "case {case}: lo {lo} beats sampled min {smin} on {s:?} (UNSOUND)"
        );
        assert!(
            hi >= smax - tol,
            "case {case}: hi {hi} beats sampled max {smax} on {s:?} (UNSOUND)"
        );
        // Sphere arm claims exactness: check tightness there.
        if matches!(s, Surface::Sphere { .. }) {
            let span = (hi - lo).max(1e-12);
            assert!(
                (smin - lo).abs() / span < 1e-4 && (hi - smax).abs() / span < 1e-4,
                "case {case}: sphere arm not tight: [{lo},{hi}] vs sampled [{smin},{smax}]"
            );
        }
        worst_slack = worst_slack.min((smin - lo).min(hi - smax));
    }
    println!(
        "rider falsifier: 3000 pairs x {n_samples} samples, no enclosure violation; min slack {worst_slack:.3e}"
    );
}

/// The margin the boolean arm uses: max(lo, -hi) > 0 must imply the
/// sampled residual never changes sign across the circle.
#[test]
fn positive_margin_implies_one_sided() {
    let mut rng = Rng(0xFEED_FACE_CAFE_BEEF);
    let mut one_sided_checked = 0u32;
    for _ in 0..20_000 {
        let (axis, u_ref) = frame(&mut rng);
        let center = Point3::new(
            rng.range(-2.0, 2.0),
            rng.range(-2.0, 2.0),
            rng.range(-2.0, 2.0),
        );
        let radius = rng.range(1e-3, 2.0);
        let (sa, _) = frame(&mut rng);
        let s: Surface<f64> = if rng.next() < 0.5 {
            Surface::Sphere {
                center: Point3::new(
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                ),
                radius: rng.range(1e-2, 2.0),
                axis: sa,
                u_ref: Vec3::unit_x(),
            }
        } else {
            Surface::Cylinder {
                origin: Point3::new(
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                ),
                axis: sa,
                radius: rng.range(1e-2, 2.0),
                u_ref: Vec3::unit_x(),
            }
        };
        let (lo, hi) = circle_residual_extremes(&s, center, axis, radius, u_ref).unwrap();
        let margin = lo.max(-hi);
        if margin <= 0.0 {
            continue;
        }
        one_sided_checked += 1;
        let v = axis.cross(u_ref);
        let mut pos = false;
        let mut neg = false;
        for i in 0..4096 {
            let t = core::f64::consts::TAU * (i as f64) / 4096.0;
            let p = center + (u_ref * t.cos() + v * t.sin()) * radius;
            let r = implicit_residual(&s, p);
            pos |= r > 0.0;
            neg |= r < 0.0;
        }
        assert!(
            !(pos && neg),
            "margin {margin} > 0 but the circle straddles the surface: {s:?}"
        );
    }
    println!("one-sided semantic held on {one_sided_checked} positive-margin pairs");
}
