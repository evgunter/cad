//! R1 review probe — the merge-base differential, written so the SAME
//! file compiles at the merge base and at the head. It reads only the
//! fields both trees have (`Exhaustiveness::{examined, excluded,
//! accounted, refined, max_depth, floor}` and the refusal's
//! `cell_width` / `floor`) and prints their BITS.
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use crate::shared::tol::band;
use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, SsiDomain, SsiError};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

fn wall_from_cols(cols: [(f64, f64); 4]) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::with_capacity(8);
    for (x, y) in cols {
        control.push(Point3::new(x, y, 0.0));
        control.push(Point3::new(x, y, 0.8));
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap()
}

fn certifiable_wall() -> NurbsSurface<f64> {
    wall_from_cols([(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)])
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

fn sphere() -> Surface<f64> {
    crate::shared::surf::sphere(1.0)
}

fn threaded_cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.03, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.08,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// Every lane-carrying length the two doors can produce on this tree,
/// as bits. Compare the printed block at merge base and at head.
#[test]
fn r1_bit_census() {
    let (p, w) = (cutting_plane(), certifiable_wall());
    let dom = |m: f64| SsiDomain {
        floor_scale: SsiDomain::floor_scale_for(m, band()),
        ..wall_domain()
    };

    // -- chart lane, the receipt (floor 0.05 m)
    let out = ssi::plane_nurbs_ssi(&p, &w, dom(0.05), band()).expect("chart run certifies");
    let e = out.exhaustiveness;
    println!(
        "CHART-RECEIPT floor_bits={:#018x} floor={:e} examined={} excluded={} accounted={} \
         refined={} max_depth={} branches={} seeds={}",
        e.floor.to_bits(),
        e.floor,
        e.examined,
        e.excluded,
        e.accounted,
        e.refined,
        e.max_depth,
        out.branches.len(),
        out.seeds
    );

    // -- chart lane, the refusal (floor 0.5 m)
    match ssi::plane_nurbs_ssi(&p, &w, dom(0.5), band()) {
        Err(SsiError::ExhaustivenessInconclusive {
            cell_width,
            floor,
            examined,
            ..
        }) => println!(
            "CHART-REFUSAL cell_width_bits={:#018x} floor_bits={:#018x} cell_width={:e} \
             floor={:e} examined={}",
            cell_width.to_bits(),
            floor.to_bits(),
            cell_width,
            floor,
            examined
        ),
        other => panic!("chart refusal expected, got {other:?}", other = other.map(|_| ())),
    }

    // -- R3 lane, the refusal
    let (s, c) = (sphere(), threaded_cylinder());
    let mut d = SsiDomain {
        center: Point3::new(0.03, 0.0, 0.996),
        half_extent: 0.2,
        extent: 0.4,
        floor_scale: 1.0,
    };
    d.floor_scale = SsiDomain::floor_scale_for(0.1, band());
    match ssi::cylinder_sphere_ssi(&c, &s, d, band()) {
        Err(SsiError::ExhaustivenessInconclusive {
            cell_width,
            floor,
            examined,
            ..
        }) => println!(
            "R3-REFUSAL cell_width_bits={:#018x} floor_bits={:#018x} cell_width={:e} \
             floor={:e} examined={}",
            cell_width.to_bits(),
            floor.to_bits(),
            cell_width,
            floor,
            examined
        ),
        Err(other) => println!("R3-REFUSAL stood down: {other}"),
        Ok(_) => panic!("R3 floor-clamped run must refuse"),
    }

    // -- R3 lane, the receipt on the full slab
    match ssi::cylinder_sphere_ssi(&c, &sphere(), slab_dom(), band()) {
        Ok(o) => {
            let e = o.exhaustiveness;
            println!(
                "R3-RECEIPT floor_bits={:#018x} floor={:e} examined={} excluded={} \
                 accounted={} refined={} max_depth={} branches={}",
                e.floor.to_bits(),
                e.floor,
                e.examined,
                e.excluded,
                e.accounted,
                e.refined,
                e.max_depth,
                o.branches.len()
            );
        }
        Err(other) => println!("R3-RECEIPT stood down: {other}"),
    }
}

fn slab_dom() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 1.5,
        extent: 2.0,
        floor_scale: 1.0,
    }
}
