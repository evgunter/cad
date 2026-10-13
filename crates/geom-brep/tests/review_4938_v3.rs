//! Reviewer probes for PR 4938's pins head: `props_chart_axis_inside`
//! against small cylinders, thin tori, spindle and horn tori, and
//! parallels near a spindle's cusp.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::{band, eps};
use crate::shared::topo::edge;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, curved_face_loops};
use std::f64::consts::TAU;

fn class<X: core::fmt::Debug>(r: &Result<X, PropsError>) -> String {
    match r {
        Ok(_) => "ok".into(),
        Err(PropsError::Escalated { cause, .. }) => {
            let s = format!("{cause:?}");
            let pred = s
                .split("predicate: Some(\"")
                .nth(1)
                .and_then(|t| t.split('"').next())
                .unwrap_or("?")
                .to_string();
            format!("esc {pred}")
        }
        Err(e) => format!("{e:?}").chars().take(70).collect(),
    }
}

fn crim<T: geom_core::Decide>(radius: f64, z: f64, u0: f64, u1: f64) -> LoopEdge<T> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, z),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(radius),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}
fn ruling<T: geom_core::Decide>(radius: f64, u: f64, z0: f64, z1: f64) -> LoopEdge<T> {
    edge(
        Curve3::Line {
            origin: p3(radius * u.cos(), radius * u.sin(), 0.0),
            dir: v3(0.0, 0.0, 1.0),
        },
        z0,
        z1,
        0,
        0,
    )
}
fn par<T: geom_core::Decide>(big: f64, r: f64, v: f64, u0: f64, u1: f64) -> LoopEdge<T> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, r * v.sin()),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(big + r * v.cos()),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}
fn mer<T: geom_core::Decide>(big: f64, r: f64, u: f64, v0: f64, v1: f64) -> LoopEdge<T> {
    edge(
        Curve3::Circle {
            center: p3(big * u.cos(), big * u.sin(), 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: T::from_f64(r),
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        v0,
        v1,
        0,
        0,
    )
}

fn cyl_rect<T: geom_core::Decide>(radius: f64, h: f64) -> String {
    let s = Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: T::from_f64(radius),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let l = vec![
        crim::<T>(radius, 0.0, 0.0, 2.0),
        ruling::<T>(radius, 2.0, 0.0, h),
        crim::<T>(radius, h, 2.0, 0.0),
        ruling::<T>(radius, 0.0, h, 0.0),
    ];
    class(&curved_face_loops(&s, &[&l], true, band()))
}
fn cyl_band<T: geom_core::Decide>(radius: f64, h: f64) -> String {
    let s = Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: T::from_f64(radius),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let (a, b) = (
        vec![crim::<T>(radius, 0.0, 0.0, TAU)],
        vec![crim::<T>(radius, h, TAU, 0.0)],
    );
    class(&curved_face_loops(&s, &[&a, &b], true, band()))
}
fn tor_rect<T: geom_core::Decide>(big: f64, r: f64, v0: f64, v1: f64, u1: f64) -> String {
    let s = Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        major_radius: T::from_f64(big),
        minor_radius: T::from_f64(r),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let l = vec![
        par::<T>(big, r, v0, 0.0, u1),
        mer::<T>(big, r, u1, v0, v1),
        par::<T>(big, r, v1, u1, 0.0),
        mer::<T>(big, r, 0.0, v1, v0),
    ];
    class(&curved_face_loops(&s, &[&l], true, band()))
}

#[test]
fn review_4938_v3_axis_inside_census() {
    let e = eps();
    let line = |name: String, f: String, iv: String| {
        eprintln!("REVIEW4938V3 eps={e:e} {name}: f64 {f} | iv {iv}")
    };
    for k in [
        3.0, 5.0, 8.0, 9.5, 10.5, 11.0, 11.5, 12.0, 12.5, 13.0, 15.0, 20.0, 100.0,
    ] {
        let rad = k * e;
        line(
            format!("cyl rect R={k}ε"),
            cyl_rect::<f64>(rad, 1.0),
            cyl_rect::<geom_core::Interval>(rad, 1.0),
        );
        line(
            format!("cyl band R={k}ε"),
            cyl_band::<f64>(rad, 1.0),
            cyl_band::<geom_core::Interval>(rad, 1.0),
        );
        // A thin torus: minor radius kε, major 1 mm.
        line(
            format!("thin torus r={k}ε"),
            tor_rect::<f64>(1e-3, rad, 0.2, 1.2, 1.0),
            tor_rect::<geom_core::Interval>(1e-3, rad, 0.2, 1.2, 1.0),
        );
        // A spindle torus R=4mm r=5mm: the parallel at v1 has radius kε (near the cusp).
        let (big, r) = (0.004, 0.005);
        let v1 = ((rad - big) / r).acos();
        line(
            format!("spindle parallel ρ={k}ε"),
            tor_rect::<f64>(big, r, 1.0, v1, 1.0),
            tor_rect::<geom_core::Interval>(big, r, 1.0, v1, 1.0),
        );
        // A horn torus R=r=5mm: the parallel at v1 has radius kε.
        let (big, r) = (0.005, 0.005);
        let v1 = ((rad - big) / r).acos();
        line(
            format!("horn parallel ρ={k}ε"),
            tor_rect::<f64>(big, r, 1.0, v1, 1.0),
            tor_rect::<geom_core::Interval>(big, r, 1.0, v1, 1.0),
        );
    }
    // Whole spindle and horn tori faces away from the cusp.
    line(
        "spindle rect mid".into(),
        tor_rect::<f64>(0.004, 0.005, 0.2, 1.5, 1.0),
        tor_rect::<geom_core::Interval>(0.004, 0.005, 0.2, 1.5, 1.0),
    );
    line(
        "horn rect mid".into(),
        tor_rect::<f64>(0.005, 0.005, 0.2, 2.5, 1.0),
        tor_rect::<geom_core::Interval>(0.005, 0.005, 0.2, 2.5, 1.0),
    );
}

#[test]
fn review_4938_v3_cusp_reach() {
    let e = eps();
    for k in [100.0, 1e3, 1e4, 1e5, 1e6, 1e7] {
        let rad = k * e;
        for (name, big, r) in [("spindle", 0.004, 0.005), ("horn", 0.005, 0.005)] {
            let v1 = ((rad - big) / r).acos();
            eprintln!(
                "REVIEW4938V3R eps={e:e} {name} parallel ρ={k}ε ({rad:e} m): f64 {} | iv {}",
                tor_rect::<f64>(big, r, 1.0, v1, 1.0),
                tor_rect::<geom_core::Interval>(big, r, 1.0, v1, 1.0)
            );
        }
    }
}
