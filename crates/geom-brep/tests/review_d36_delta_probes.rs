//! **Delta-review probes for PCERT D36's fix pass** (`10a74f5`):
//! `cone_conic_incidence` against built cones, and the rerouted rim
//! gate. Each row prints what it measured and asserts what the
//! reviewer expects, so a row that changes verdict goes red.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::{PcurveCertifyError, UncoveredClass, chart_pcurve};
use geom_core::{Interval, Point3, Real, Vec3};

fn cone<T: Real>(alpha: f64) -> Surface<T> {
    let f = T::from_f64;
    Surface::Cone {
        apex: Point3::new(f(0.0), f(0.0), f(0.0)),
        axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
        half_angle: f(alpha),
        u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
    }
}

/// Worst distance from the DOUBLE cone (both nappes), and the min
/// and centre `|z|` of the carrier.
fn off_double_cone(c: &Curve3<f64>, alpha: f64) -> (f64, f64) {
    let mut worst = 0.0_f64;
    let mut zmin = f64::INFINITY;
    for i in 0..4096 {
        let p = c.eval(f64::from(i) * core::f64::consts::TAU / 4096.0);
        worst = worst.max((p.x.hypot(p.y) * alpha.cos() - p.z.abs() * alpha.sin()).abs());
        zmin = zmin.min(p.z.abs());
    }
    (worst, zmin)
}

/// The cone's section by the plane through `(0,0,h)` with normal tilted
/// `theta` toward `+x` (same construction as `chart_incidence::section`).
fn section(alpha: f64, h: f64, theta: f64) -> (Point3<f64>, Vec3<f64>, Vec3<f64>, f64, f64) {
    let t = alpha.tan().powi(2);
    let (s, c) = theta.sin_cos();
    let a = c * c - t * s * s;
    let u = Vec3::new(c, 0.0, -s);
    let centre = Point3::new(0.0, 0.0, h) + u * (-t * h * s / a);
    let k = h * alpha.tan() * c;
    (centre, Vec3::new(s, 0.0, c), u, k / a, k / a.sqrt())
}

fn ellipse(sec: (Point3<f64>, Vec3<f64>, Vec3<f64>, f64, f64)) -> Curve3<f64> {
    let (center, axis, u_ref, major, minor) = sec;
    Curve3::Ellipse {
        center,
        axis,
        major,
        minor,
        u_ref,
    }
}

fn rim(alpha: f64, h: f64, tilt: f64) -> Curve3<f64> {
    let (s, c) = tilt.sin_cos();
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, h),
        axis: Vec3::new(s, 0.0, c),
        radius: h.abs() * alpha.tan(),
        u_ref: Vec3::new(c, 0.0, -s),
    }
}

fn verdict<T: geom_core::Decide>(c: &Curve3<T>, s: &Surface<T>) -> &'static str {
    match chart_pcurve(c, s, band()) {
        Ok(_) => "image",
        Err(PcurveCertifyError::UnsupportedCarrier {
            class: UncoveredClass::ConeSection,
            ..
        }) => "uncovered:ConeSection",
        Err(PcurveCertifyError::UnsupportedCarrier { .. }) => "uncovered:other",
        Err(PcurveCertifyError::CarrierOffChart { .. }) => "off-chart",
        Err(PcurveCertifyError::Escalated { .. }) => "escalated",
        Err(_) => "other",
    }
}

fn lift(c: &Curve3<f64>) -> Curve3<Interval> {
    let f = Interval::from_f64;
    let p = |p: Point3<f64>| Point3::new(f(p.x), f(p.y), f(p.z));
    let v = |v: Vec3<f64>| Vec3::new(f(v.x), f(v.y), f(v.z));
    match *c {
        Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => Curve3::Ellipse {
            center: p(center),
            axis: v(axis),
            major: f(major),
            minor: f(minor),
            u_ref: v(u_ref),
        },
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => Curve3::Circle {
            center: p(center),
            axis: v(axis),
            radius: f(radius),
            u_ref: v(u_ref),
        },
        _ => unreachable!(),
    }
}

/// (a) The stored cone is the complete double cone (`v < 0` is the
/// mirror nappe, `geom/src/surfaces.rs`), so a mirror-nappe conic IS on
/// the chart: a mirror rim images and a mirror section is uncovered.
#[test]
fn mirror_nappe_conics_are_on_the_double_cone_chart() {
    let alpha = 0.5;
    let r = rim(alpha, -2.0, 0.0);
    let (d, _) = off_double_cone(&r, alpha);
    let v = verdict(&r, &cone(alpha));
    eprintln!("mirror rim: dist {d:e} -> {v}");
    assert_eq!(v, "image");
    let e = ellipse(section(alpha, -2.0, 0.3));
    let (d, _) = off_double_cone(&e, alpha);
    let v = verdict(&e, &cone(alpha));
    eprintln!("mirror section: dist {d:e} -> {v}");
    assert!(d < 1e-13);
    assert_eq!(v, "uncovered:ConeSection");
}

/// (b) + claim 2: true rims, tilted rims, needle / wide sections and
/// the same sections shifted by 1e-6 — f64 and Interval.
#[test]
fn rims_and_sections_across_half_angles() {
    let e = eps();
    for alpha in [0.05_f64, 0.5, 1.2, 1.55] {
        for tilt in [0.0, 1e-10, 1e-9, 1e-8, 1e-6] {
            let c = rim(alpha, 1.0, tilt);
            let (d, _) = off_double_cone(&c, alpha);
            let v = verdict(&c, &cone(alpha));
            let vi = verdict(&lift(&c), &cone::<Interval>(alpha));
            eprintln!(
                "alpha {alpha} rim tilt {tilt:e}: dist/eps {:.3} -> {v} / iv {vi}",
                d / e
            );
            if d < 0.5 * e {
                assert!(v != "off-chart", "on-cone rim called off-chart");
            }
            if d > 20.0 * e {
                assert_eq!(v, "off-chart", "off-cone rim let through");
            }
        }
        // A tilted section: theta must keep A = cos²θ − tan²α·sin²θ > 0.
        let theta = 0.3 * (1.0 / alpha.tan()).atan();
        let sec = section(alpha, 1.0, theta);
        let on = ellipse(sec);
        let (d, zmin) = off_double_cone(&on, alpha);
        let v = verdict(&on, &cone(alpha));
        let vi = verdict(&lift(&on), &cone::<Interval>(alpha));
        eprintln!(
            "alpha {alpha} section theta {theta:.4}: dist {d:e} zc {:.3} zmin {zmin:.3} -> {v} / iv {vi}",
            sec.0.z
        );
        assert_eq!(v, "uncovered:ConeSection");
        assert_eq!(vi, "uncovered:ConeSection");
        for shift in [Vec3::new(0.0, 1e-6, 0.0), Vec3::new(0.0, 0.0, 1e-6)] {
            let moved = ellipse((sec.0 + shift, sec.1, sec.2, sec.3, sec.4));
            let (d, _) = off_double_cone(&moved, alpha);
            let v = verdict(&moved, &cone(alpha));
            eprintln!("  shifted {shift:?}: dist/eps {:.1} -> {v}", d / e);
            assert_eq!(v, "off-chart");
        }
    }
}

/// (c) The lever is the CENTRE's height `|w_z|`, while the distance
/// at a point is `Q / (2 sin α |p_z|)`. On an elongated (near-parabolic)
/// section `z_min ≪ z_c`, so the meter under-reads the distance near
/// the low vertex by `z_c / z_min`. Shift the section off the cone
/// along its own plane normal and record the worst true distance the
/// incidence test still reads as ZERO (uncovered).
#[test]
fn elongated_section_meter_under_reads_near_the_low_vertex() {
    let e = eps();
    let alpha = 0.5;
    let crit = (1.0 / alpha.tan()).atan();
    let mut worst_uncovered: f64 = 0.0;
    for frac in [0.3_f64, 0.9, 0.99, 0.999] {
        let theta = frac * crit;
        let sec = section(alpha, 1.0, theta);
        let base = ellipse(sec);
        let (_, zmin) = off_double_cone(&base, alpha);
        let zc = sec.0.z;
        for k in 0..60 {
            let delta = e * 1.25_f64.powi(k) * 1e-2;
            let moved = ellipse((sec.0 + sec.1 * delta, sec.1, sec.2, sec.3, sec.4));
            let (d, _) = off_double_cone(&moved, alpha);
            let v = verdict(&moved, &cone(alpha));
            if v.starts_with("uncovered") {
                worst_uncovered = worst_uncovered.max(d / e);
            }
            if k % 6 == 0 {
                eprintln!(
                    "frac {frac} zc/zmin {:.1}: delta/eps {:.3} dist/eps {:.3} -> {v}",
                    zc / zmin,
                    delta / e,
                    d / e
                );
            }
        }
    }
    eprintln!("worst true distance read uncovered: {worst_uncovered:.2} eps");
}

/// The coefficients' worst `|k_i| / (2 sin α |w_z|)`, re-derived here
/// from the docstring (`pcurve_cache.rs` `cone_conic_incidence`).
fn meter(c: &Curve3<f64>, alpha: f64) -> f64 {
    let (center, axis, major, minor, u_ref) = match *c {
        Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => (center, axis, major, minor, u_ref),
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => (center, axis, radius, radius, u_ref),
        _ => unreachable!(),
    };
    let w = center - Point3::new(0.0, 0.0, 0.0);
    let a = u_ref * major;
    let b = axis.cross(u_ref) * minor;
    let (wz, az, bz) = (w.z, a.z, b.z);
    let c2 = alpha.cos().powi(2);
    let ks = [
        c2 * (w.dot(w) + (a.dot(a) + b.dot(b)) * 0.5) - wz * wz - (az * az + bz * bz) * 0.5,
        (c2 * w.dot(a) - wz * az) * 2.0,
        (c2 * w.dot(b) - wz * bz) * 2.0,
        (c2 * (a.dot(a) - b.dot(b)) - (az * az - bz * bz)) * 0.5,
        c2 * a.dot(b) - az * bz,
    ];
    ks.iter().fold(0.0_f64, |m, k| m.max(k.abs())) / (2.0 * alpha.sin() * wz.abs())
}

/// (c) adversarial: random small perturbations of an elongated
/// section's 8 parameters, scaled so the incidence meter reads 0.9 ε
/// (inside the zero half), and the worst true distance among those
/// the kernel then calls uncovered.
#[test]
fn elongated_section_adversarial_perturbation() {
    let e = eps();
    let alpha = 0.5;
    let crit = (1.0 / alpha.tan()).atan();
    let mut seed: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut rnd = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
    };
    for frac in [0.3_f64, 0.9, 0.99, 0.999] {
        let (c0, n0, u0, ma0, mi0) = section(alpha, 1.0, frac * crit);
        let base = ellipse((c0, n0, u0, ma0, mi0));
        let (_, zmin) = off_double_cone(&base, alpha);
        let mut worst = (0.0_f64, String::new());
        for _ in 0..3000 {
            let dir: [f64; 8] = core::array::from_fn(|_| rnd());
            let make = |s: f64| {
                let om = Vec3::new(dir[3], dir[4], dir[5]) * s;
                let n = (n0 + om.cross(n0)).normalize();
                let u = u0 + om.cross(u0);
                let u = (u - n * u.dot(n)).normalize();
                ellipse((
                    c0 + Vec3::new(dir[0], dir[1], dir[2]) * s,
                    n,
                    u,
                    ma0 + dir[6] * s,
                    mi0 + dir[7] * s,
                ))
            };
            let m1 = meter(&make(1e-7), alpha) / 1e-7;
            let s = 0.9 * e / m1;
            let c = make(s);
            let v = verdict(&c, &cone(alpha));
            let (d, _) = off_double_cone(&c, alpha);
            if v.starts_with("uncovered") && d / e > worst.0 {
                worst = (d / e, format!("meter/eps {:.2}", meter(&c, alpha) / e));
            }
        }
        eprintln!(
            "frac {frac} zc/zmin {:.1}: worst true dist read UNCOVERED = {:.1} eps ({})",
            c0.z / zmin,
            worst.0,
            worst.1
        );
        // Red on 10a74f5 at zc/zmin ≳ 40: past the sliver (K·ε), an
        // off-cone ellipse is excused as an uncovered section.
        assert!(
            worst.0 < 10.0,
            "frac {frac}: {:.1} eps off read uncovered",
            worst.0
        );
    }
}

/// (d) A row only `k₀` decides: a wide-cone circle ⊥ the axis whose
/// centre is 10 ε off it (failing the centring gate, so the incidence
/// test runs) and whose radius is 1e-3 too large. Every other
/// coefficient reads inside ε, so dropping `k₀` turns OFF-CHART into
/// uncovered. No committed row has this shape.
#[test]
fn a_radius_off_circle_is_decided_by_k0_alone() {
    let alpha = 1.55_f64;
    let c = Curve3::Circle {
        center: Point3::new(10.0 * eps(), 0.0, 1.0),
        axis: Vec3::unit_z(),
        radius: alpha.tan() + 1e-3,
        u_ref: Vec3::unit_x(),
    };
    let v = verdict(&c, &cone(alpha));
    eprintln!("k0-only circle -> {v}");
    assert_eq!(v, "off-chart");
}

/// Claim 3: a parallel at the tube's top with its centre `δ` off the
/// axis is within `δ²/2r` of the torus; away from the top it leaves at
/// first order. Both read uncovered (`TorusGeneralCircle`).
#[test]
fn torus_off_axis_parallels() {
    let (big, small) = (1.0_f64, 0.1_f64);
    let torus = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: big,
        minor_radius: small,
        u_ref: Vec3::unit_x(),
    };
    for (z, rho, delta) in [
        (small, big, 1e-6),
        (small, big, 1e-4),
        (0.0, big + small, 1e-4),
    ] {
        let c = Curve3::Circle {
            center: Point3::new(delta, 0.0, z),
            axis: Vec3::unit_z(),
            radius: rho,
            u_ref: Vec3::unit_x(),
        };
        let d = (0..4096)
            .map(|i| {
                let p = c.eval(f64::from(i) * core::f64::consts::TAU / 4096.0);
                ((p.x.hypot(p.y) - big).hypot(p.z) - small).abs()
            })
            .fold(0.0_f64, f64::max);
        let got = chart_pcurve(&c, &torus, band());
        let class = match &got {
            Err(PcurveCertifyError::UnsupportedCarrier { class, .. }) => format!("{class:?}"),
            other => format!("{other:?}").chars().take(60).collect(),
        };
        eprintln!(
            "z {z} rho {rho} delta {delta:e}: dist/eps {:.3e} -> {class}",
            d / eps()
        );
        assert_eq!(class, "TorusGeneralCircle");
    }
}

/// Claim 2 adversarially: random small perturbations of a true rim
/// (centre, tilt, radius) scaled so the incidence meter reads 0.9 ε,
/// and the worst true distance among those then called uncovered or
/// imaged.
#[test]
fn rim_adversarial_perturbation() {
    let e = eps();
    let mut seed: u64 = 0x2545_F491_4F6C_DD1D;
    let mut rnd = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
    };
    for alpha in [0.05_f64, 0.5, 1.2, 1.55] {
        let mut worst = [0.0_f64; 3];
        for _ in 0..3000 {
            let dir: [f64; 6] = core::array::from_fn(|_| rnd());
            let make = |s: f64| {
                let n = Vec3::new(dir[3] * s, dir[4] * s, 1.0).normalize();
                let u = Vec3::unit_x();
                let u = (u - n * u.dot(n)).normalize();
                Curve3::Circle {
                    center: Point3::new(dir[0] * s, dir[1] * s, 1.0 + dir[2] * s),
                    axis: n,
                    radius: alpha.tan() + dir[5] * s,
                    u_ref: u,
                }
            };
            let m1 = meter(&make(1e-7), alpha) / 1e-7;
            let c = make(0.9 * e / m1);
            let v = verdict(&c, &cone(alpha));
            let (d, _) = off_double_cone(&c, alpha);
            let slot = match v {
                "image" => 0,
                "uncovered:ConeSection" => 1,
                _ => 2,
            };
            worst[slot] = worst[slot].max(d / e);
        }
        eprintln!(
            "alpha {alpha}: worst dist/eps imaged {:.2}, uncovered {:.2}, other {:.2}",
            worst[0], worst[1], worst[2]
        );
    }
}
