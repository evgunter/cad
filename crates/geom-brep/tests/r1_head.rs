//! R1 review probe — head-only: the lane tag's readings as a user sees
//! them, the fixture's certified chart speed against the retired
//! literal, and how much of the `4·ε` FLOOR-TIE slack the round trip
//! actually uses.
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use crate::r1_diff::{certifiable_wall, cutting_plane, sphere, threaded_cylinder, wall_domain};
use crate::shared::tol::band;
use geom_brep::ssi::{self, SsiDomain, SsiError};
use geom_brep::{ExhaustLane, Exhaustiveness, SsiOutcome};
use geom_core::Point3;

/// The literal `m5_pr7_ssi.rs` retired.
const RETIRED_WALL_CHART_SPEED: f64 = 1.130_884_609_498_248;

fn lane_speed(e: &Exhaustiveness) -> Option<f64> {
    match e.lane {
        ExhaustLane::R3 => None,
        ExhaustLane::Chart { speed } => Some(speed.get()),
    }
}

/// **A user's plane × NURBS wall run, both ways, on both lanes.**
#[test]
fn r1_end_to_end_as_a_user() {
    let (p, w) = (cutting_plane(), certifiable_wall());
    let dom = |m: f64| SsiDomain {
        floor_scale: SsiDomain::floor_scale_for(m, band()),
        ..wall_domain()
    };

    // ---- the run that CERTIFIES, with a receipt.
    let out: SsiOutcome = ssi::plane_nurbs_ssi(&p, &w, dom(0.05), band()).unwrap();
    let e = out.exhaustiveness;
    println!("RECEIPT Display: {e}");
    println!("RECEIPT Debug:   {e:?}");
    println!(
        "RECEIPT floor={:e}  floor_meters()={:e}  speed={:?}",
        e.floor,
        e.floor_meters(),
        lane_speed(&e)
    );
    let s = lane_speed(&e).expect("the chart door reports the chart lane");
    println!(
        "SPEED  reported={:.17e}  retired_literal={:.17e}  bits_equal={}",
        s,
        RETIRED_WALL_CHART_SPEED,
        s.to_bits() == RETIRED_WALL_CHART_SPEED.to_bits()
    );
    assert_eq!(
        s.to_bits(),
        RETIRED_WALL_CHART_SPEED.to_bits(),
        "the kernel's rate must be the literal the test retired"
    );
    // How much of the 4·eps slack the FLOOR-TIE round trip uses.
    let back = e.floor_meters();
    println!(
        "SLACK  receipt: |back-0.05| = {:e}  = {:.3} x (eps*0.05)",
        (back - 0.05).abs(),
        (back - 0.05).abs() / (f64::EPSILON * 0.05)
    );
    // The same floor without the reconstitution through floor_scale_for:
    let pure = (0.05_f64 / s) * s;
    println!(
        "SLACK  pure to_param/to_meters round trip on 0.05: |x-0.05| = {:e} = {:.3} x (eps*0.05)",
        (pure - 0.05).abs(),
        (pure - 0.05).abs() / (f64::EPSILON * 0.05)
    );

    // ---- the run that REFUSES.
    match ssi::plane_nurbs_ssi(&p, &w, dom(0.5), band()) {
        Err(ref err @ SsiError::ExhaustivenessInconclusive { cell_width, floor, .. }) => {
            println!("REFUSAL Display: {err}");
            println!("REFUSAL Debug:   {err:?}");
            println!(
                "REFUSAL raw cell_width={cell_width:e} floor={floor:e} \
                 cell_width_meters={:?} floor_meters={:?}",
                err.cell_width_meters(),
                err.floor_meters()
            );
            let back = err.floor_meters().unwrap();
            println!(
                "SLACK  refusal: |back-0.5| = {:e} = {:.3} x (eps*0.5)",
                (back - 0.5).abs(),
                (back - 0.5).abs() / (f64::EPSILON * 0.5)
            );
        }
        other => panic!("expected the chart refusal, got {:?}", other.map(|_| ())),
    }

    // ---- the R3 lane, for the contrast a caller sees.
    let (s3, c3) = (sphere(), threaded_cylinder());
    let mut d = SsiDomain {
        center: Point3::new(0.03, 0.0, 0.996),
        half_extent: 0.2,
        extent: 0.4,
        floor_scale: 1.0,
    };
    d.floor_scale = SsiDomain::floor_scale_for(0.1, band());
    match ssi::cylinder_sphere_ssi(&c3, &s3, d, band()) {
        Err(ref err @ SsiError::ExhaustivenessInconclusive { .. }) => {
            println!("R3 REFUSAL Display: {err}");
            println!(
                "R3 REFUSAL floor_meters={:?} cell_width_meters={:?}",
                err.floor_meters(),
                err.cell_width_meters()
            );
        }
        other => println!("R3 refusal stood down: {:?}", other.map(|_| ())),
    }
    match ssi::cylinder_sphere_ssi(&c3, &sphere(), crate::r1_diff::slab_dom(), band()) {
        Ok(o) => println!("R3 RECEIPT Display: {}", o.exhaustiveness),
        Err(e) => println!("R3 receipt stood down: {e}"),
    }
}
