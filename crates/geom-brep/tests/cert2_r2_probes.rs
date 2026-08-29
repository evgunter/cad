//! CERT-2 adversarial review (R2) probes — NOT part of the unit.
//!
//! Drives `plane_nurbs_ssi` through the PUBLIC door with degenerate
//! walls authored here, and reports what the door's refusals actually
//! read like to a consumer. Printing rows, not pinning rows: the point
//! is the text a caller sees.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_brep::ssi;
use geom_brep::ssi::{SsiDomain, SsiError};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn cutting_plane() -> Surface<f64> {
    let n = Vec3::new(0.0, 0.25, 1.0);
    let n = n / n.norm();
    let u = Vec3::new(1.0, 0.0, 0.0);
    let u = (u - n * u.dot(n)) / (u - n * u.dot(n)).norm();
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.4),
        normal: n,
        u_ref: u,
    }
}

fn wall_domain() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.5, 0.0, 0.4),
        half_extent: 2.0,
        extent: 1.5,
        floor_scale: 1.0,
    }
}

/// The substrate wall, scaled by `s` and with every weight set to `w`.
fn wall(s: f64, w: f64) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::with_capacity(8);
    for (x, y) in [(0.0, 0.0), (0.35, 0.18), (0.70, -0.12), (1.05, 0.04)] {
        control.push(Point3::new(x * s, y * s, 0.0));
        control.push(Point3::new(x * s, y * s, 0.8 * s));
    }
    NurbsSurface::new(ku, kv, control, vec![w; 8]).unwrap()
}

fn say(label: &str, r: Result<impl std::fmt::Debug, SsiError>) {
    match r {
        Ok(_) => println!("[door] {label:38} => Ok (SILENT)"),
        Err(e) => println!("[door] {label:38} => {e}"),
    }
}

/// PROBE A — overflowing nets, the magnitude ladder D286/§4 argues about.
#[test]
fn r2_door_overflowing_nets() {
    for s in [1.0e100, 1.0e150, 1.0e169, 1.0e200, 1.0e300, 1.0e308] {
        say(
            &format!("net scale {s:e}, unit weights"),
            ssi::plane_nurbs_ssi(&cutting_plane(), &wall(s, 1.0), wall_domain(), band()),
        );
    }
}

/// PROBE B — the underflow side: subnormal and near-subnormal weights.
#[test]
fn r2_door_subnormal_weights() {
    for w in [
        f64::from_bits(1),
        f64::from_bits(1 << 20),
        f64::MIN_POSITIVE,
        1.0e-300,
        1.0e-10,
    ] {
        say(
            &format!("unit net, weight {w:e}"),
            ssi::plane_nurbs_ssi(&cutting_plane(), &wall(1.0, w), wall_domain(), band()),
        );
    }
}

/// PROBE C — a degenerate wall in the zero-radius spirit: the net
/// collapses to a point, so the chart speed is exactly zero, and a net
/// whose spread is so small the speed is subnormal-but-positive.
#[test]
fn r2_door_collapsed_and_tiny_walls() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    for (label, s) in [
        ("collapsed to a point (s=0)", 0.0),
        ("spread 1e-320 (subnormal speed?)", 1.0e-320),
        ("spread 1e-300", 1.0e-300),
        ("spread 1e-160", 1.0e-160),
    ] {
        let mut control = Vec::with_capacity(8);
        for (x, y) in [(0.0, 0.0), (0.35, 0.18), (0.70, -0.12), (1.05, 0.04)] {
            control.push(Point3::new(x * s, y * s, 0.0));
            control.push(Point3::new(x * s, y * s, 0.8 * s));
        }
        let w = NurbsSurface::new(ku.clone(), kv.clone(), control, vec![1.0; 8]).unwrap();
        say(
            label,
            ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()),
        );
    }
}

/// PROBE D — issue 1218's claim, checked at the door: a NON-FINITE
/// PLANE against a perfectly ordinary wall.
#[test]
fn r2_door_nonfinite_plane_blames_the_wall() {
    for (label, bad) in [
        ("plane origin +inf", f64::INFINITY),
        ("plane origin NaN", f64::NAN),
    ] {
        let p = Surface::Plane {
            origin: Point3::new(bad, 0.0, 0.4),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        say(
            label,
            ssi::plane_nurbs_ssi(&p, &wall(1.0, 1.0), wall_domain(), band()),
        );
    }
}

/// PROBE E — issue 1219's premise, checked in the tree rather than
/// argued: does `curvature_lever_arm` actually return poison for the
/// kinds the issue names?
#[test]
fn r2_lever_arm_poison_premise() {
    // Reached only through the public surface enum; the ℝ³ pair is
    // lane-gated, so this probe reads the source claim instead.
    let src = include_str!("../src/implicit.rs");
    let has = src.contains("curvature_lever_arm");
    println!("[premise] implicit.rs mentions curvature_lever_arm: {has}");
}

/// PROBE F — a uniform weight scale is a GEOMETRIC NO-OP on a rational
/// surface (all weights equal ⇒ the same point set). Does the door
/// agree?
#[test]
fn r2_door_uniform_weight_is_a_geometric_noop() {
    for w in [1.0, 1.0e-1, 1.0e-4, 1.0e-8, 1.0e-10, 1.0e2, 1.0e8] {
        say(
            &format!("SAME SURFACE, uniform weight {w:e}"),
            ssi::plane_nurbs_ssi(&cutting_plane(), &wall(1.0, w), wall_domain(), band()),
        );
    }
}

/// PROBE G — hunt for a door-reachable SUBNORMAL-but-positive chart
/// speed, the band the new march guard admits and cannot use.
#[test]
fn r2_door_subnormal_chart_speed_ladder() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    for e in [-160i32, -200, -250, -290, -300, -305, -308, -310, -315] {
        let s = 10f64.powi(e);
        let mut control = Vec::with_capacity(8);
        for (x, y) in [(0.0, 0.0), (0.35, 0.18), (0.70, -0.12), (1.05, 0.04)] {
            control.push(Point3::new(x * s, y * s, 0.0));
            control.push(Point3::new(x * s, y * s, 0.8 * s));
        }
        let w = NurbsSurface::new(ku.clone(), kv.clone(), control, vec![1.0; 8]).unwrap();
        say(
            &format!("net spread 1e{e}"),
            ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()),
        );
    }
}
