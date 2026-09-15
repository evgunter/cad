//! R2 probe (lane `scalar-exhaust-r2`): the head-only surface — lane
//! tag, `floor_meters`, `cell_width_meters`, both `Display`s, the
//! fixture's speed against the retired literal, and the round-trip
//! error in units of ε. Not a permanent row.
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use crate::r2_exhaust_probe_base::{cyl, plane, pole_domain, wall, wall_domain};
use crate::shared::surf;
use crate::shared::tol::band;
use geom_brep::ExhaustLane;
use geom_brep::ssi::{self, SsiDomain, SsiError};

const RETIRED: f64 = 1.130_884_609_498_248;

#[test]
fn r2_head_readings() {
    let b = band();
    let (p, w) = (plane(), wall());
    let out = ssi::plane_nurbs_ssi(&p, &w, wall_domain(0.05), b).expect("healthy chart run");
    let e = out.exhaustiveness;
    let speed = match e.lane {
        ExhaustLane::Chart { speed } => speed.get(),
        ExhaustLane::R3 => panic!("chart run tagged R3"),
    };
    println!(
        "HEAD CHART_SPEED {speed:e} bits={:#018x} retired_bits={:#018x} same_bits={}",
        speed.to_bits(),
        RETIRED.to_bits(),
        speed.to_bits() == RETIRED.to_bits()
    );
    let back = e.floor_meters();
    println!(
        "HEAD CH_OK floor_uv_bits={:#018x} floor_m={back:e} floor_m_bits={:#018x} bare_product_bits={:#018x} rel_err_over_eps={}",
        e.floor.to_bits(),
        back.to_bits(),
        (e.floor * speed).to_bits(),
        (back - 0.05).abs() / (f64::EPSILON * 0.05)
    );
    println!("HEAD DISPLAY_RECEIPT_CHART {e}");
    println!("HEAD DEBUG_RECEIPT_CHART {e:?}");
    let err = ssi::plane_nurbs_ssi(&p, &w, wall_domain(0.5), b).expect_err("clamped chart run");
    if let SsiError::ExhaustivenessInconclusive {
        lane,
        cell_width,
        floor,
        ..
    } = &err
    {
        let sp = match lane {
            ExhaustLane::Chart { speed } => speed.get(),
            ExhaustLane::R3 => f64::NAN,
        };
        println!(
            "HEAD CH_REFUSE speed_same_bits={} cw_m={:e} cw_m_bits={:#018x} bare={:#018x} floor_m={:e} floor_m_bits={:#018x} bare={:#018x} rel_err_over_eps={}",
            sp.to_bits() == speed.to_bits(),
            err.cell_width_meters().unwrap(),
            err.cell_width_meters().unwrap().to_bits(),
            (cell_width * sp).to_bits(),
            err.floor_meters().unwrap(),
            err.floor_meters().unwrap().to_bits(),
            (floor * sp).to_bits(),
            (err.floor_meters().unwrap() - 0.5).abs() / (f64::EPSILON * 0.5)
        );
    }
    println!("HEAD DISPLAY_ERR_CHART {err}");
    println!("HEAD DEBUG_ERR_CHART {err:?}");

    let s = surf::sphere(1.0);
    let o = ssi::cylinder_sphere_ssi(&cyl(), &s, pole_domain(), b).expect("healthy r3 run");
    let e = o.exhaustiveness;
    println!(
        "HEAD R3_OK lane_is_r3={} floor_bits={:#018x} floor_m_bits={:#018x}",
        matches!(e.lane, ExhaustLane::R3),
        e.floor.to_bits(),
        e.floor_meters().to_bits()
    );
    println!("HEAD DISPLAY_RECEIPT_R3 {e}");
    let mut d = pole_domain();
    d.floor_scale = SsiDomain::floor_scale_for(0.1, b);
    let err = ssi::cylinder_sphere_ssi(&cyl(), &s, d, b).expect_err("clamped r3 run");
    println!(
        "HEAD R3_REFUSE cw_m={:?} floor_m={:?} floor_m_bits={:#018x}",
        err.cell_width_meters(),
        err.floor_meters(),
        err.floor_meters().unwrap().to_bits()
    );
    println!("HEAD DISPLAY_ERR_R3 {err}");
    let m = err.floor_meters().unwrap_or(f64::NAN);
    println!("HEAD ERGO floor_meters().unwrap_or(NAN) = {m:e}");
}

/// Limb-3 speed collapse, planted through `R2_PLANT` (a probe-branch
/// edit in `certify.rs`): `zero` forces the closure's `m` to 0, `inf`
/// forces it to +∞. Unset: the untouched path.
#[test]
fn r2_limb3_plant() {
    let b = band();
    let (p, w) = (plane(), wall());
    let plant = std::env::var("R2_PLANT").unwrap_or_default();
    match ssi::plane_nurbs_ssi(&p, &w, wall_domain(0.05), b) {
        Ok(o) => {
            for br in &o.branches {
                println!(
                    "PLANT[{plant}] single-span OK tube_radius={:e} tube_boxes={} transversality_lo={:e} receipt: {}",
                    br.certificate.tube_radius,
                    br.certificate.tube_boxes,
                    br.certificate.tube_transversality,
                    o.exhaustiveness
                );
            }
        }
        Err(e) => println!("PLANT[{plant}] single-span ERR {e}"),
    }
    // A wall with an INTERIOR knot in u, so the span windows select
    // different control-net differences and a pad can change the box.
    let w = {
        use geom::NurbsSurface;
        use geom_core::Point3;
        use geom_core::spline::KnotVector;
        let cols = [(0.0, 0.0), (0.175, 0.07), (0.525, 0.19), (0.875, 0.27), (1.05, 0.30)];
        let ku =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let mut control = Vec::with_capacity(10);
        for (x, y) in cols {
            control.push(Point3::new(x, y, 0.0));
            control.push(Point3::new(x, y, 0.8));
        }
        NurbsSurface::new(ku, kv, control, vec![1.0; 10]).unwrap()
    };
    match ssi::plane_nurbs_ssi(&p, &w, wall_domain(0.05), b) {
        Ok(o) => {
            for br in &o.branches {
                println!(
                    "PLANT[{plant}] OK tube_radius={:e} tube_boxes={} transversality_lo={:e} receipt: {}",
                    br.certificate.tube_radius,
                    br.certificate.tube_boxes,
                    br.certificate.tube_transversality,
                    o.exhaustiveness
                );
            }
        }
        Err(e) => println!("PLANT[{plant}] ERR {e}"),
    }
}
