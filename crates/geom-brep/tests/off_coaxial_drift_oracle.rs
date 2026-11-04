//! **The circle arm's `kappa_drift` against the sampled drift of
//! `κ_rel`**, on near-coaxial torus and cone poses: the oracle of
//! `work/tang/the-off-coaxial-torus-and-cone-drift-bound-has-no-soundness-argument.md`.
//!
//! Off coaxial, the torus and cone arms charge `8·dev/r²`, an accounting
//! derived for a constant Hessian; the third-derivative term is not in
//! it. Every span of the 9-sample schedule is walked at 64 points, and
//! `|κ_rel(t) − κ_rel(tᵢ)|` (`tᵢ` the span's left sample, the tube's own
//! comparison) must stay within the drift the arm states, at `f64` and,
//! as a definite lower bound against the bound's lower end, at
//! `Interval`.
//!
//! Ignored: it is evidence for the row, not a gate on a derivation that
//! does not exist yet. Run it with
//! `cargo nextest run -p geom-brep --features test-support --run-ignored only -E 'test(off_coaxial_drift)'`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

const SPANS: u32 = 8;
const PER_SPAN: u32 = 64;

fn perp(x: Vec3<f64>, a: Vec3<f64>) -> Vec3<f64> {
    x - a * x.dot(a)
}

/// The worst sampled `|Δκ_rel|` over a span from its left sample, as a
/// fraction of the stated `kappa_drift`, at `f64`.
fn ratio_f64(s1: &Surface<f64>, s2: &Surface<f64>, c: &Curve3<f64>, t1: f64) -> f64 {
    let drift = geom_brep::tangent_span_bounds(s1, s2, c, 0.0, t1)
        .unwrap()
        .kappa_drift;
    let step = t1 / f64::from(SPANS);
    let kappa = |t: f64| {
        let (p, tau) = c.ders1(t);
        geom_brep::tangent_jet(s1, s2, p, tau).kappa_rel
    };
    let mut worst: f64 = 0.0;
    for i in 0..SPANS {
        let ti = step * f64::from(i);
        let ki = kappa(ti);
        for k in 1..=PER_SPAN {
            let t = ti + step * f64::from(k) / f64::from(PER_SPAN);
            worst = worst.max((kappa(t) - ki).abs());
        }
    }
    worst / drift
}

/// As [`ratio_f64`] at `Interval`: a definite lower bound on `|Δκ_rel|`
/// over the lower end of the stated drift.
fn ratio_interval(s1: &Surface<f64>, s2: &Surface<f64>, c: &Curve3<f64>, t1: f64) -> f64 {
    let lift = Interval::from_f64;
    let (s1, s2, c) = (s1.map_scalar(lift), s2.map_scalar(lift), c.map_scalar(lift));
    let drift = geom_brep::tangent_span_bounds(&s1, &s2, &c, lift(0.0), lift(t1))
        .unwrap()
        .kappa_drift;
    let step = t1 / f64::from(SPANS);
    let kappa = |t: f64| {
        let (p, tau) = c.ders1(lift(t));
        geom_brep::tangent_jet(&s1, &s2, p, tau).kappa_rel
    };
    let mut worst: f64 = 0.0;
    for i in 0..SPANS {
        let ti = step * f64::from(i);
        let ki = kappa(ti);
        for k in 1..=PER_SPAN {
            let d = kappa(ti + step * f64::from(k) / f64::from(PER_SPAN)) - ki;
            let definite = if d.lo() > 0.0 {
                d.lo()
            } else if d.hi() < 0.0 {
                -d.hi()
            } else {
                0.0
            };
            worst = worst.max(definite);
        }
    }
    worst / drift.lo()
}

/// The misses applied to a coaxial carrier: its centre offset by `δ`,
/// its axis tilted by `δ`, and both.
fn misses() -> Vec<(String, Vec3<f64>, Vec3<f64>)> {
    let z = Vec3::new(0.0, 0.0, 1.0);
    let mut out = Vec::new();
    for delta in [1e-3, 1e-4, 1e-7] {
        let shift = Vec3::new(delta, -0.5 * delta, 0.7 * delta);
        let tilt = Vec3::new(delta, 0.3 * delta, 1.0).normalize();
        out.push((format!("offset {delta:e}"), shift, z));
        out.push((format!("tilt {delta:e}"), Vec3::new(0.0, 0.0, 0.0), tilt));
        out.push((format!("offset+tilt {delta:e}"), shift, tilt));
    }
    out
}

/// A coaxial pose: its label, the other surface, the torus or cone,
/// and the carrier's centre and radius.
type Pose = (String, Surface<f64>, Surface<f64>, Point3<f64>, f64);

#[test]
#[ignore = "evidence for a filed row; run command in the module doc"]
fn off_coaxial_drift_stays_within_the_stated_bound() {
    let tau = 2.0 * core::f64::consts::PI;
    let z = Vec3::new(0.0, 0.0, 1.0);
    let x = Vec3::new(1.0, 0.0, 0.0);
    let o = Point3::new(0.0, 0.0, 0.0);
    let mut poses: Vec<Pose> = Vec::new();
    for (big, r) in [(1.0, 0.2), (0.3, 0.2), (0.21, 0.2), (5.0, 0.1)] {
        let torus = Surface::Torus {
            center: o,
            axis: z,
            major_radius: big,
            minor_radius: r,
            u_ref: x,
        };
        let top = Point3::new(0.0, 0.0, r);
        poses.push((
            format!("torus R {big} r {r} / top plane"),
            Surface::Plane {
                origin: top,
                normal: z,
                u_ref: x,
            },
            torus.clone(),
            top,
            big,
        ));
        for (side, rho) in [("outer", big + r), ("inner", big - r)] {
            poses.push((
                format!("torus R {big} r {r} / {side} cylinder"),
                Surface::Cylinder {
                    origin: o,
                    axis: z,
                    radius: rho,
                    u_ref: x,
                },
                torus.clone(),
                o,
                rho,
            ));
        }
    }
    for degrees in [5.0f64, 30.0, 80.0] {
        let alpha = degrees.to_radians();
        let (sin, cos) = alpha.sin_cos();
        poses.push((
            format!("cone {degrees}° / inscribed sphere"),
            Surface::Sphere {
                center: Point3::new(0.0, 0.0, 1.0),
                radius: sin,
                axis: z,
                u_ref: x,
            },
            Surface::Cone {
                apex: o,
                axis: z,
                half_angle: alpha,
                u_ref: x,
            },
            Point3::new(0.0, 0.0, cos * cos),
            sin * cos,
        ));
    }
    let mut worst = (0.0f64, String::new());
    let mut rows = 0;
    for (label, other, round, centre, radius) in &poses {
        for (miss, shift, axis) in misses() {
            let carrier = Curve3::circle(*centre + shift, axis, *radius, perp(x, axis).normalize());
            for span in [tau, tau / 16.0] {
                for (first, second, order) in [(other, round, "→"), (round, other, "←")] {
                    for (ratio, scalar) in [
                        (ratio_f64(first, second, &carrier, span), "f64"),
                        (ratio_interval(first, second, &carrier, span), "Interval"),
                    ] {
                        rows += 1;
                        if ratio > worst.0 {
                            worst = (
                                ratio,
                                format!("{label} {order}, {miss}, span {span:.3}, {scalar}"),
                            );
                        }
                    }
                }
            }
        }
    }
    println!(
        "{rows} rows; worst sampled/stated drift {:.4} at {}",
        worst.0, worst.1
    );
    assert!(
        worst.0 <= 1.0,
        "the sampled κ_rel drift exceeds the stated bound: ratio {:.4} at {}",
        worst.0,
        worst.1
    );
}
