//! **A cone's rulings and rims image at their distance from the cone.**
//! The cone chart's slant `v` is a length along the generator, and its
//! height is `v·cos α`; reading `v` off a carrier's height alone divides
//! whatever the carrier stands off the cone by `cos α`, about 48 at α =
//! 1.55. The chart image of a ruling or a rim instead takes its slant
//! as the projection on the generator through it, so the image misses
//! the carrier by the carrier's own distance from the generator, and
//! its certificate reads that distance — not the sum of its axial and
//! radial legs, which is `(sin α + cos α)·d`, up to `√2·d` at π/4.
//!
//! Each row stands a carrier `d` off a cone along the cone's normal and
//! asks for an envelope within `d·(1 + 2⁻¹⁰)` plus a few `u` of the
//! carrier's size. The bands are the rows' own, so they read the same
//! at every ε.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom-brep/src/pcurve_cache.rs"];

use geom::{Curve3, Surface};
use geom_brep::{PcurveCache, chart_pcurve};
use geom_core::{Band, Bounds, Interval, Point3, Real, Vec3};

const FLAT: f64 = 1.55;
const FLAT_OFF: f64 = 0.4e-12;
/// Half-angles from a narrow cone through `π/4`, where the two legs of
/// a rim's miss are equal and their sum over-reads most.
const ALPHAS: [f64; 4] = [0.1, 0.5, core::f64::consts::FRAC_PI_4, 1.0];
const OFF: f64 = 4e-9;
const U: f64 = 0.5 * f64::EPSILON;

fn flat_band() -> Band {
    Band::new(1e-12, 1e-11).unwrap()
}

fn band() -> Band {
    Band::new(1e-8, 1e-7).unwrap()
}

/// The cone of half-angle `alpha`, apex at the origin about `+z`.
fn cone(alpha: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    }
}

/// The point at slant `v` on the generator along `+x`, moved `d` along
/// the cone's outward normal there (`v < 0` is the mirror nappe, its
/// point on `−x`), as `(radius, height)`.
fn off_cone(alpha: f64, v: f64, d: f64) -> (f64, f64) {
    let (s, c) = alpha.sin_cos();
    let n = v.signum();
    // The generator `(s, c)·v`, the normal `(c, −s)` turned to the
    // point's side.
    (v.abs() * s + d * c, v * c - n * d * s)
}

/// The rim at slant `v`, `d` off the cone.
fn rim(alpha: f64, v: f64, d: f64) -> (Curve3<f64>, f64) {
    let (rho, h) = off_cone(alpha, v, d);
    let rim = Curve3::Circle {
        center: Point3::new(0.0, 0.0, h),
        axis: Vec3::unit_z(),
        radius: rho,
        u_ref: Vec3::unit_x(),
    };
    (rim, core::f64::consts::TAU)
}

/// The ruling from slant `v0` on the cone to `v1`, whose far end stands
/// `d` off it; and its length.
fn ruling(alpha: f64, (v0, v1): (f64, f64), d: f64) -> (Curve3<f64>, f64) {
    let (r0, h0) = off_cone(alpha, v0, 0.0);
    let (r1, h1) = off_cone(alpha, v1, d);
    let a = Point3::new(r0 * v0.signum(), 0.0, h0);
    let b = Point3::new(r1 * v1.signum(), 0.0, h1);
    let len = (b - a).norm();
    let line = Curve3::Line {
        origin: a,
        dir: (b - a) / len,
    };
    (line, len)
}

fn size(carrier: &Curve3<f64>, t1: f64) -> f64 {
    match carrier {
        Curve3::Line { origin, dir } => (*origin + *dir * t1 - Point3::origin()).norm(),
        Curve3::Circle { center, radius, .. } => (*center - Point3::origin()).norm() + radius,
        _ => unreachable!(),
    }
}

/// The f64 certificate's envelope, within `d·(1 + 2⁻¹⁰) + 16·u·size`.
fn reads_its_distance(
    (carrier, t1): (Curve3<f64>, f64),
    (alpha, d, band): (f64, f64, Band),
    what: &str,
    wrong: &mut Vec<String>,
) {
    let ceiling = d * (1.0 + 2f64.powi(-10)) + 16.0 * U * size(&carrier, t1);
    let got = chart_pcurve(&carrier, &cone(alpha), band)
        .and_then(|image| PcurveCache::certify(image, 0.0, t1, &carrier, &cone(alpha), band));
    match got {
        Ok(cache) => {
            let envelope = cache.certificate().envelope;
            if envelope > ceiling {
                wrong.push(format!(
                    "{what}: envelope {envelope:e} = {:.4}·d, over {ceiling:e}",
                    envelope / d
                ));
            }
        }
        Err(e) => wrong.push(format!("{what}: {e:?}")),
    }
}

/// A rim at slant ±10 and ±300 on the flat cone, `d` off it.
#[test]
fn a_rim_off_a_flat_cone_images_at_its_distance() {
    assert!(
        FLAT_OFF / FLAT.cos() > flat_band().escalate(),
        "the height read is past the band"
    );
    let mut wrong = Vec::new();
    for v in [10.0, -10.0, 300.0, -300.0] {
        reads_its_distance(
            rim(FLAT, v, FLAT_OFF),
            (FLAT, FLAT_OFF, flat_band()),
            &format!("rim at v {v}"),
            &mut wrong,
        );
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// A ruling from slant ±1 to ±300 on the flat cone whose far end stands
/// `d` off it.
#[test]
fn a_ruling_off_a_flat_cone_images_at_its_distance() {
    let mut wrong = Vec::new();
    for span in [(1.0, 300.0), (-1.0, -300.0)] {
        reads_its_distance(
            ruling(FLAT, span, FLAT_OFF),
            (FLAT, FLAT_OFF, flat_band()),
            &format!("ruling {span:?}"),
            &mut wrong,
        );
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// A rim `d` off a cone along its normal misses by `d`, its axial and
/// radial legs `d·sin α` and `d·cos α` read as one displacement: their
/// sum is `1.095·d` at α = 0.1 and `√2·d` at π/4.
#[test]
fn a_rim_off_a_cone_reads_its_two_legs_as_one_distance() {
    let mut wrong = Vec::new();
    for alpha in ALPHAS {
        for v in [10.0, -10.0, 300.0, -300.0] {
            reads_its_distance(
                rim(alpha, v, OFF),
                (alpha, OFF, band()),
                &format!("rim at α {alpha}, v {v}"),
                &mut wrong,
            );
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// A ruling whose far end stands `d` off a cone along its normal misses
/// by `d` at the far end.
#[test]
fn a_ruling_off_a_cone_images_at_its_distance() {
    let mut wrong = Vec::new();
    for alpha in ALPHAS {
        for span in [(1.0, 300.0), (-1.0, -300.0)] {
            reads_its_distance(
                ruling(alpha, span, OFF),
                (alpha, OFF, band()),
                &format!("ruling at α {alpha}, {span:?}"),
                &mut wrong,
            );
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// A ruling on a narrow cone certifies at `Interval` with an envelope
/// whose enclosure stays within `24·u` of its size: the distance off
/// the generator is read in its radial-sized parts, never as the
/// difference of two length-sized vectors (`x − (x·ĝ)·ĝ`), whose
/// enclosure is twice as wide on a narrow cone.
#[test]
fn a_ruling_on_a_narrow_cone_encloses_its_envelope_tightly() {
    let mut wrong = Vec::new();
    for alpha in [0.1, 0.5] {
        for span in [(1.0, 100.0), (-1.0, -100.0)] {
            let (line, t1) = ruling(alpha, span, 0.0);
            let ceiling = 24.0 * U * size(&line, t1);
            let line = line.map_scalar(Interval::from_f64);
            let surface = cone(alpha).map_scalar(Interval::from_f64);
            let got = chart_pcurve(&line, &surface, band()).and_then(|image| {
                PcurveCache::certify(
                    image,
                    Interval::from_f64(0.0),
                    Interval::from_f64(t1),
                    &line,
                    &surface,
                    band(),
                )
            });
            match got {
                Ok(cache) => {
                    let envelope = cache.certificate().envelope;
                    let width = envelope.hi() - envelope.lo();
                    if width > ceiling {
                        wrong.push(format!(
                            "α {alpha}, {span:?}: width {width:e} = {:.1}·u·size, over {ceiling:e}",
                            width / (ceiling / 24.0)
                        ));
                    }
                }
                Err(e) => wrong.push(format!("α {alpha}, {span:?}: {e:?}")),
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
