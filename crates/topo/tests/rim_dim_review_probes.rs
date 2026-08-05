//! ADVERSARIAL REVIEW PROBES (authored on branch review/rim-dim,
//! ADOPTED BY MERGE into the unit — authorship kept): which of the
//! audit's FIXED predicates actually fire in the twin boolean
//! configurations (coverage-vacuity check), and the linearity pins
//! for the sites the twin configs leave silent (flush-face subtract →
//! `bool_plane_orient`; oblique split → `split_join_frame_arm`,
//! `split_section_area`). `bool_strut_order` stays SILENT here too —
//! it is verified by code-read + suites-green only (rare germ-fan
//! lane), stated in the audit doc's row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::collections::BTreeMap;

use common::prism_z;
use geom_core::k_stats::{self, Probe};
use topo::{BooleanResult, subtract};

const FIXED: &[&str] = &[
    "props_rim_level_group",
    "props_rim_side",
    "bool_join_facing",
    "bool_strut_order",
    "bool_plane_orient",
    "pm_census_ee_parallel",
    "point_in_loop_arm",
    "split_join_frame_arm",
    "split_section_area",
];

#[test]
fn which_fixed_predicates_fire_in_the_twin_configs() {
    let s = |v: f64| v * 1e-3;
    let bx = |x: (f64, f64), y: (f64, f64), z: (f64, f64)| {
        prism_z::<Probe>(
            &[
                (Probe(s(x.0)).0, Probe(s(y.0)).0),
                (s(x.1), s(y.0)),
                (s(x.1), s(y.1)),
                (s(x.0), s(y.1)),
            ],
            s(z.0),
            s(z.1),
        )
        .body
    };
    k_stats::start_recording();
    let a = bx((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = bx((1.0, 3.0), (1.0, 3.0), (1.0, 3.0));
    let r = subtract(&a, &b).expect("corner subtract");
    let BooleanResult::Body(rb) = r else {
        panic!("corner: body out");
    };
    topo::validate_pseudomanifold(&rb.body, &topo::ContactRecords::default()).expect("census");
    let a2 = bx((0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    let b2 = bx((1.0, 2.0), (1.0, 2.0), (-1.0, 2.0));
    // ε-row honesty (see rim_dim_boolean_twins module docs): on coarse
    // ε rows this mm pocket subtract refuses on the deferred F4 area
    // comparand — for this diagnostic printer, tolerate exactly that
    // signature.
    match subtract(&a2, &b2) {
        Ok(_) => {}
        Err(topo::BooleanError::Escalated { diag })
            if diag.predicate == Some("bool_ring_run_winding") =>
        {
            println!("pocket subtract refused on the F4 signature at this ε row: {diag:?}");
        }
        Err(other) => panic!("pocket subtract failed outside the F4 signature: {other:?}"),
    }
    let mut counts: BTreeMap<&'static str, (usize, usize)> = BTreeMap::new();
    for sample in k_stats::take_samples() {
        let e = counts.entry(sample.predicate).or_default();
        e.0 += 1;
        if sample.margin != 0.0 {
            e.1 += 1;
        }
    }
    for f in FIXED {
        match counts.get(f) {
            Some((n, nz)) => println!("FIRED {f}: {n} samples, {nz} nonzero"),
            None => println!("SILENT {f}"),
        }
    }
}

use geom_core::Sign;
use geom_core::k_stats::SampleOutcome;
use topo::{SplitPlane, split};

/// Linearity probe for the fixed sites the twin pin leaves SILENT:
/// a flush-face subtract (coplanar lane -> bool_plane_orient) and an
/// oblique plane split (split_join_frame_arm, split_section_area).
#[test]
fn silent_fixed_predicates_scale_linearly() {
    let run = |scale: f64| {
        let s = |v: f64| v * scale;
        let bx = |x: (f64, f64), y: (f64, f64), z: (f64, f64)| {
            prism_z::<Probe>(
                &[
                    (s(x.0), s(y.0)),
                    (s(x.1), s(y.0)),
                    (s(x.1), s(y.1)),
                    (s(x.0), s(y.1)),
                ],
                s(z.0),
                s(z.1),
            )
            .body
        };
        k_stats::start_recording();
        // Flush: b's x-max face lies IN a's x=2 face plane. Undeclared,
        // so the op REFUSES typed at bool_plane_offset — but the
        // bool_plane_orient sample fires (and records) first, which is
        // all this probe harvests.
        let a = bx((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
        let b = bx((1.0, 2.0), (0.5, 1.5), (0.5, 1.5));
        assert!(subtract(&a, &b).is_err(), "undeclared flush must refuse");
        // Oblique split of a cube.
        let body = bx((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)).clone();
        let n = geom_core::Vec3::new(Probe(1.0 / 3.0), Probe(2.0 / 3.0), Probe(2.0 / 3.0));
        let plane = SplitPlane {
            origin: geom_core::Point3::new(Probe(s(1.0)), Probe(s(1.0)), Probe(s(1.0))),
            normal: n,
        };
        split(&body, &plane).expect("oblique split");
        let mut out: BTreeMap<&'static str, Vec<f64>> = BTreeMap::new();
        for sample in k_stats::take_samples() {
            if matches!(
                sample.outcome,
                SampleOutcome::Definite(Sign::Positive | Sign::Negative)
            ) && sample.margin != 0.0
            {
                out.entry(sample.predicate)
                    .or_default()
                    .push(sample.margin.abs());
            }
        }
        for v in out.values_mut() {
            v.sort_by(f64::total_cmp);
        }
        out
    };
    let mm = run(1e-3);
    let m = run(1.0);
    for pred in [
        "bool_plane_orient",
        "split_join_frame_arm",
        "split_section_area",
        "bool_strut_order",
    ] {
        match (mm.get(pred), m.get(pred)) {
            (Some(a), Some(b)) => {
                assert_eq!(a.len(), b.len(), "{pred} count mismatch");
                let mut worst: f64 = 0.0;
                for (x, y) in a.iter().zip(b) {
                    worst = worst.max((y / x / 1e3 - 1.0).abs());
                }
                println!(
                    "FIRED {pred}: {} decisive, worst rel dev {worst:.3e}",
                    a.len()
                );
                assert!(worst < 1e-9, "{pred} margins do not scale linearly");
            }
            _ => println!("SILENT {pred} in flush+split configs"),
        }
    }
}
