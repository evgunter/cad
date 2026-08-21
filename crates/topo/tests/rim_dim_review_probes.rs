//! ADVERSARIAL REVIEW PROBES (authored on branch review/rim-dim,
//! ADOPTED BY MERGE into the unit — authorship kept): which of the
//! audit's FIXED predicates actually fire in the twin boolean
//! configurations (coverage-vacuity check), and the linearity pins
//! for the sites the twin configs leave silent (flush-face subtract →
//! `bool_plane_orient`; oblique split → `split_join_frame_arm`,
//! `split_section_area`). `bool_strut_order` stays SILENT here too —
//! it is verified by code-read + suites-green only (rare germ-fan
//! lane), stated in the audit doc's row.
//!
//! **NO TEST IN THIS FILE IS EXECUTED BY CI.** The probe suites CI runs are
//! rostered in `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run
//! by `scripts/k_probe_sweep.sh`; this one is on neither list, so nothing
//! here can go red on a merge and its assertions are evidence for a reader
//! rather than a gate. By hand:
//! `cargo test -p topo --features probe --test all -- rim_dim_review_probes::`.

#![cfg(feature = "probe")]
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
    // The F3+F4 unit's sites. `witness_at_mid_parameter` rides along
    // as the CONTROL: it is not a fixed site, it is the predicate F3's
    // funnel bypass was stealing attribution from, so its count here is
    // the before/after evidence.
    "bool_ring_run_winding",
    "volume_backstop",
    "volume_backstop_violation",
    "volume_backstop_operand",
    "witness_at_mid_parameter",
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
    let r = subtract(&a, &b, Tol::witness()).expect("corner subtract");
    let BooleanResult::Body(rb) = r else {
        panic!("corner: body out");
    };
    topo::validate_pseudomanifold(&rb.body, &topo::ContactRecords::default(), Tol::witness())
        .expect("census");
    let a2 = bx((0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    let b2 = bx((1.0, 2.0), (1.0, 2.0), (-1.0, 2.0));
    // The F4 fix (see rim_dim_boolean_twins module docs) retired this
    // configuration's in-band refusal on coarse ε rows: the winding is
    // metered to its mean width, so the mm pocket subtract computes at
    // every ε. The signature tolerance this printer carried is gone
    // with it — a refusal here is now a finding.
    subtract(&a2, &b2, Tol::witness()).expect("pocket subtract");
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
    // The F3 attribution pin (the funnel-bypass retirement). Before the
    // fix this census read `witness_at_mid_parameter 123 samples, 5
    // nonzero` and no `volume_backstop*` rows at all: the backstop's
    // raw `sign_within` left the recorder's name unset, so its VOLUME
    // margins were filed under whichever predicate had decided last.
    // Now the volume gates carry their own rows and every remaining
    // witness sample is a genuine coincident mid-parameter residual —
    // a nonzero one would mean some margin is riding under this name
    // again.
    let witness = counts
        .get("witness_at_mid_parameter")
        .copied()
        .expect("witness_at_mid_parameter fires in these configs");
    assert_eq!(
        witness.1, 0,
        "stale-name attribution is back: {} of {} witness_at_mid_parameter \
         samples carry a nonzero margin (its own samples are coincident \
         residuals; nonzero ones belonged to the volume backstop)",
        witness.1, witness.0
    );
    for gate in [
        "volume_backstop",
        "volume_backstop_violation",
        "volume_backstop_operand",
    ] {
        assert!(
            counts.get(gate).is_some_and(|c| c.1 > 0),
            "{gate}: expected nonzero-margin samples under its OWN name \
             (the F3 routing pin is vacuous otherwise)"
        );
    }
    // Deviation 2, pinned rather than asserted in prose: the bound check
    // RUNS on these mm-scale operands. Pre-F3 the raw m³ comparand put
    // a 2 mm cube's 8e-9 m³ inside the default band, read that as "not
    // certifiably bounded", and skipped the bound entirely — only 1 of
    // the 3 mm-scale checks ran. Both arms of both checks must fire now
    // (2 bound checks × 2 arms = 4 samples, 2 under each name).
    for arm in ["volume_backstop", "volume_backstop_violation"] {
        assert_eq!(
            counts.get(arm).map(|c| c.0),
            Some(2),
            "{arm}: both mm-scale bound checks must reach this arm — a \
             count of 1 is the pre-F3 silent skip coming back"
        );
    }
}

use geom_core::Sign;
use geom_core::Tol;
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
        assert!(
            subtract(&a, &b, Tol::witness()).is_err(),
            "undeclared flush must refuse"
        );
        // Oblique split of a cube.
        let body = bx((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)).clone();
        let n = geom_core::Vec3::new(Probe(1.0 / 3.0), Probe(2.0 / 3.0), Probe(2.0 / 3.0));
        let plane = SplitPlane {
            origin: geom_core::Point3::new(Probe(s(1.0)), Probe(s(1.0)), Probe(s(1.0))),
            normal: n,
        };
        split(&body, &plane, Tol::witness()).expect("oblique split");
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
