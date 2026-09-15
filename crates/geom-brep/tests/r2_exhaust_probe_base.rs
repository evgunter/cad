//! R2 probe (lane `scalar-exhaust-r2`, unit EXHAUST-LANE): the raw
//! receipt / refusal bits at the merge base and at the frozen head, so
//! the two can be diffed line for line. Reads only fields that exist on
//! both sides. Not a permanent row.
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use crate::shared::surf;
use crate::shared::tol::band;
use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, SsiDomain, SsiError};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

pub fn wall() -> NurbsSurface<f64> {
    let cols = [(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)];
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::with_capacity(8);
    for (x, y) in cols {
        control.push(Point3::new(x, y, 0.0));
        control.push(Point3::new(x, y, 0.8));
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap()
}

pub fn plane() -> Surface<f64> {
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

pub fn wall_domain(floor_m: f64) -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.5, 0.0, 0.4),
        half_extent: 2.0,
        extent: 1.5,
        floor_scale: SsiDomain::floor_scale_for(floor_m, band()),
    }
}

pub fn cyl() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.03, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.08,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

pub fn pole_domain() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.03, 0.0, 0.996),
        half_extent: 0.2,
        extent: 0.4,
        floor_scale: 1.0,
    }
}

#[test]
fn r2_raw_bits() {
    let b = band();
    let s = surf::sphere(1.0);
    match ssi::cylinder_sphere_ssi(&cyl(), &s, pole_domain(), b) {
        Ok(o) => {
            let e = o.exhaustiveness;
            println!(
                "BITS R3_OK branches={} seeds={} examined={} excluded={} accounted={} refined={} max_depth={} floor_bits={:#018x} floor={:e}",
                o.branches.len(),
                o.seeds,
                e.examined,
                e.excluded,
                e.accounted,
                e.refined,
                e.max_depth,
                e.floor.to_bits(),
                e.floor
            );
            for br in &o.branches {
                println!(
                    "BITS R3_OK tube_radius_bits={:#018x}",
                    br.certificate.tube_radius.to_bits()
                );
            }
        }
        Err(e) => println!("BITS R3_OK_ERR {e}"),
    }
    let mut d = pole_domain();
    d.floor_scale = SsiDomain::floor_scale_for(0.1, b);
    match ssi::cylinder_sphere_ssi(&cyl(), &s, d, b) {
        Err(SsiError::ExhaustivenessInconclusive {
            cell_width,
            floor,
            examined,
            ..
        }) => println!(
            "BITS R3_REFUSE cw_bits={:#018x} floor_bits={:#018x} examined={} cw={:e} floor={:e}",
            cell_width.to_bits(),
            floor.to_bits(),
            examined,
            cell_width,
            floor
        ),
        Err(e) => println!("BITS R3_REFUSE_OTHER {e}"),
        Ok(_) => println!("BITS R3_REFUSE_OK"),
    }
    let (p, w) = (plane(), wall());
    match ssi::plane_nurbs_ssi(&p, &w, wall_domain(0.05), b) {
        Ok(o) => {
            let e = o.exhaustiveness;
            println!(
                "BITS CH_OK branches={} seeds={} examined={} excluded={} accounted={} refined={} max_depth={} floor_bits={:#018x} floor={:e}",
                o.branches.len(),
                o.seeds,
                e.examined,
                e.excluded,
                e.accounted,
                e.refined,
                e.max_depth,
                e.floor.to_bits(),
                e.floor
            );
            for br in &o.branches {
                println!(
                    "BITS CH_OK tube_radius_bits={:#018x} tube_boxes={} transv_lo_bits={:#018x}",
                    br.certificate.tube_radius.to_bits(),
                    br.certificate.tube_boxes,
                    br.certificate.tube_transversality.to_bits()
                );
            }
        }
        Err(e) => println!("BITS CH_OK_ERR {e}"),
    }
    match ssi::plane_nurbs_ssi(&p, &w, wall_domain(0.5), b) {
        Err(SsiError::ExhaustivenessInconclusive {
            cell_width,
            floor,
            examined,
            ..
        }) => println!(
            "BITS CH_REFUSE cw_bits={:#018x} floor_bits={:#018x} examined={} cw={:e} floor={:e}",
            cell_width.to_bits(),
            floor.to_bits(),
            examined,
            cell_width,
            floor
        ),
        Err(e) => println!("BITS CH_REFUSE_OTHER {e}"),
        Ok(_) => println!("BITS CH_REFUSE_OK"),
    }
}
