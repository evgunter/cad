//! **A flat cone's rulings and rims image at their distance from the
//! cone.** The cone chart's slant `v` is a length along the generator,
//! and its height is `v·cos α`; reading `v` off a carrier's height alone
//! divides whatever the carrier stands off the cone by `cos α`, about
//! 48 at α = 1.55. The chart image of a ruling or a rim instead takes
//! its slant as the projection on the generator through it, so the
//! image misses the carrier by the carrier's own distance from the cone
//! and its certificate reads that distance.
//!
//! Each row stands a carrier off a cone of half-angle 1.55 by `d =
//! 0.4e-12` along the cone's normal, under the band `(1e-12, 1e-11)` and
//! with `d / cos α` over its escalation edge, and the image certifies
//! with an envelope within a few `u` of the carrier's size of `d`. The
//! band is the row's own, so the row reads the same at every ε.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom-brep/src/pcurve_cache.rs"];

use geom::{Curve3, Surface};
use geom_brep::{PcurveCache, chart_pcurve};
use geom_core::{Band, Point3, Vec3};

const ALPHA: f64 = 1.55;
const OFF: f64 = 0.4e-12;

fn band() -> Band {
    Band::new(1e-12, 1e-11).unwrap()
}

/// The cone, apex at the origin about `+z`; `nappe` picks the side.
fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: ALPHA,
        u_ref: Vec3::unit_x(),
    }
}

/// The point at slant `v` on the generator along `+x`, moved `d` along
/// the cone's outward normal there (`v < 0` is the mirror nappe, its
/// point on `−x`).
fn off_cone(v: f64, d: f64) -> (f64, f64) {
    let (s, c) = ALPHA.sin_cos();
    let n = v.signum();
    // (radius, height): the generator `(s, c)·v`, the normal `(c, −s)`
    // turned to the point's side.
    (v.abs() * s + d * c, v * c - n * d * s)
}

fn certifies(carrier: &Curve3<f64>, t1: f64, what: &str, wrong: &mut Vec<String>) {
    let size = match carrier {
        Curve3::Line { origin, dir } => (*origin + *dir * t1 - Point3::origin()).norm(),
        Curve3::Circle { center, radius, .. } => (*center - Point3::origin()).norm() + radius,
        _ => unreachable!(),
    };
    let floor = OFF + 64.0 * 0.5 * f64::EPSILON * size;
    let got = chart_pcurve(carrier, &cone(), band())
        .and_then(|image| PcurveCache::certify(image, 0.0, t1, carrier, &cone(), band()));
    match got {
        Ok(cache) => {
            let envelope = cache.certificate().envelope;
            if envelope > floor {
                wrong.push(format!("{what}: envelope {envelope:e} over {floor:e}"));
            }
        }
        Err(e) => wrong.push(format!("{what}: {e:?}")),
    }
}

/// A rim at slant ±10 and ±300, `d` off the cone.
#[test]
fn a_rim_off_a_flat_cone_images_at_its_distance() {
    assert!(
        OFF / ALPHA.cos() > band().escalate(),
        "the height read is past the band"
    );
    let mut wrong = Vec::new();
    for v in [10.0, -10.0, 300.0, -300.0] {
        let (rho, h) = off_cone(v, OFF);
        let rim = Curve3::Circle {
            center: Point3::new(0.0, 0.0, h),
            axis: Vec3::unit_z(),
            radius: rho,
            u_ref: Vec3::unit_x(),
        };
        certifies(
            &rim,
            core::f64::consts::PI,
            &format!("rim at v {v}"),
            &mut wrong,
        );
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// A ruling from slant ±1 to ±300 whose far end stands `d` off the cone.
#[test]
fn a_ruling_off_a_flat_cone_images_at_its_distance() {
    let mut wrong = Vec::new();
    for (v0, v1) in [(1.0, 300.0), (-1.0, -300.0)] {
        let (r0, h0) = off_cone(v0, 0.0);
        let (r1, h1) = off_cone(v1, OFF);
        let x = |v: f64, r: f64| r * v.signum();
        let (a, b) = (
            Point3::new(x(v0, r0), 0.0, h0),
            Point3::new(x(v1, r1), 0.0, h1),
        );
        let len = (b - a).norm();
        let ruling = Curve3::Line {
            origin: a,
            dir: (b - a) / len,
        };
        certifies(&ruling, len, &format!("ruling {v0}..{v1}"), &mut wrong);
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
