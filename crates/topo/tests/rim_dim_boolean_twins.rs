//! Scale-covariance pins for the rim-dimensional audit's topo fixes
//! (see `docs/predicate-dimension-audit.md`).
//!
//! The ε-semantics contract (D4): every decision margin is a POINT
//! DEVIATION in meters, so on two geometrically-similar bodies the
//! margin stream must scale LINEARLY with the model scale — same
//! predicates, same counts, same verdicts, every margin ×1000 between
//! a mm twin and a metre twin. A bare cosine/sine (the audited class
//! (c) defects fixed in this unit: `bool_join_facing`,
//! `bool_strut_order`, `bool_plane_orient`, `pm_census_ee_parallel`,
//! `point_in_loop_arm`, `split_join_frame_arm`) is scale-INVARIANT
//! and an area/volume is scale-QUADRATIC/CUBIC — both break the
//! elementwise ratio this suite asserts.
//!
//! Two boolean configurations at the recording scalar `Probe`:
//! a corner-overlap subtract and a through-pocket subtract (rings).
//! For every predicate fired at both scales the sorted |margin| lists
//! must match elementwise at ratio 1000, EXCEPT the audit's known,
//! documented non-length comparands (the FLAG list below) — those are
//! reported, and any new nonconformer fails the suite (a finding, not
//! noise).
//!
//! # ε-row honesty (the hosted matrix runs this at several ε)
//!
//! The mm twin's ring-winding AREA margins (deferred audit finding F4,
//! `bool_ring_run_winding`: measured 2e-6/6e-6/8e-6 on these
//! fixtures) land INSIDE `Band{ε, Kε}` on coarse rows — at ε = 1e-6
//! the pocket subtract REFUSES typed. That refusal is the CURRENT
//! TRUTH of the deferred defect, so on rows where the band can catch
//! those margins this suite's claim IS the refusal (asserted with
//! F4's signature); the linearity pin runs on the rows where the
//! margins clear. The banked F4+F5 unit (sequenced immediately after
//! the M6-3 merge) retires the refusal arm, at which point the
//! signature assertion here goes red and moves WITH the fix.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::collections::BTreeMap;

use common::prism_z;
use geom_core::Sign;
use geom_core::k_stats::{self, Probe, SampleOutcome};
use topo::{BooleanResult, subtract};

/// Audited, documented non-length comparands still awaiting their own
/// units (docs/predicate-dimension-audit.md FLAG rows). Everything
/// else must scale linearly.
const KNOWN_NONLINEAR: &[&str] = &[
    // F3: flux volume (m³) against the linear band, ops.rs backstop.
    "volume_backstop",
    // F3's second face, MEASURED here: the backstop classifies through
    // a RAW `sign_within` (the audit's one funnel bypass), so on the
    // recording lane its volume margins are logged under whatever
    // predicate name the funnel set LAST — on these fixtures that is
    // certify's `witness_at_mid_parameter`. Executed evidence (ε =
    // 1e-12 row): the "witness" decisive list at the metre scale is
    // exactly the operand/result VOLUME set {1, 1, 3, 8, 8, 16} m³,
    // scaling ×1e-9 (cubic) between the twins. The real
    // mid-parameter-distance samples are coincident residuals (Zero
    // outcome) and never reach the ratio check. Retires with F3.
    "witness_at_mid_parameter",
    // F4: Newell AREA (m²) against the linear band, three sites.
    "bool_ring_run_winding",
    // F2: ray-caster denominators (dimensionless / 1/m).
    "bool_point_in_solid_denom",
    "bool_ray_cylinder_disc",
];

/// Predicates whose DECISION COUNT may differ between the twins.
/// `witness_at_mid_parameter`: the F3 stale-name contamination above
/// also perturbs its sample COUNT (the backstop's bypass decisions
/// ride under its name; measured 102 vs 103 at the default ε row).
/// Anything else with a count mismatch fails the pin.
const KNOWN_SCALE_DEPENDENT_COUNTS: &[&str] = &["witness_at_mid_parameter"];

fn box_at<F: Fn(f64) -> f64>(
    s: &F,
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
) -> topo::Body<Probe> {
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
}

/// Runs both boolean configurations at `scale`, returning the fired
/// predicates with their (outcome, |margin|) streams in recording
/// order — or `None` when the mm-scale F4 refusal row fired (see the
/// module docs; the refusal itself is asserted inside).
fn margins_at(scale: f64) -> Option<BTreeMap<&'static str, Vec<(SampleOutcome, f64)>>> {
    let s = |v: f64| v * scale;
    k_stats::start_recording();
    // Corner overlap: generic crossing subtract.
    let a = box_at(&s, (0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = box_at(&s, (1.0, 3.0), (1.0, 3.0), (1.0, 3.0));
    let r = subtract(&a, &b).expect("corner subtract");
    let BooleanResult::Body(rb) = r else {
        panic!("corner: body out");
    };
    // The pseudomanifold census over the result (empty declarations):
    // the pm_census_* sweeps — including the fixed
    // `pm_census_ee_parallel` — decide every entity pair.
    topo::validate_pseudomanifold(&rb.body, &topo::ContactRecords::default())
        .expect("corner census");
    // Through-pocket: the tool pierces the top and bottom faces, so
    // the result carries ring loops (the point-in-loop lane). On
    // coarse ε rows this subtract REFUSES on the deferred F4 area
    // comparand (module docs) — the three-outcome match below pins
    // that refusal as F4's live signature instead of absorbing it.
    let a2 = box_at(&s, (0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    let b2 = box_at(&s, (1.0, 2.0), (1.0, 2.0), (-1.0, 2.0));
    match subtract(&a2, &b2) {
        Ok(BooleanResult::Body(_)) => {}
        Ok(other) => panic!("pocket: expected a body, got {other:?}"),
        Err(topo::BooleanError::Escalated { diag }) => {
            // Only F4's own signature is an admissible refusal: the
            // ring-winding AREA margin, in-band by ITS OWN payload.
            assert_eq!(
                diag.predicate,
                Some("bool_ring_run_winding"),
                "pocket refusal outside the documented F4 signature: {diag:?}"
            );
            let _ = k_stats::take_samples();
            println!(
                "F4 LIVE SIGNATURE at this ε row (scale {scale:e}): the mm ring-winding \
                 AREA margin refused the pocket subtract in-band — {diag:?}. The banked \
                 F4+F5 unit (after the M6-3 merge) retires this arm; this assertion \
                 then moves with it."
            );
            return None;
        }
        Err(other) => panic!("pocket refusal outside the documented F4 signature: {other:?}"),
    }
    let mut out: BTreeMap<&'static str, Vec<(SampleOutcome, f64)>> = BTreeMap::new();
    for sample in k_stats::take_samples() {
        out.entry(sample.predicate)
            .or_default()
            .push((sample.outcome, sample.margin.abs()));
    }
    Some(out)
}

/// The pin: same predicates, same counts, and every margin pair at
/// ratio exactly `scale_ratio` (rel 1e-9) — the margin stream of a
/// scaled body is the scaled margin stream, predicate by predicate,
/// sample by sample.
#[test]
fn boolean_margin_streams_scale_linearly_with_the_model() {
    let eps = geom_core::Tolerance::get().eps;
    let (Some(mm), Some(m)) = (margins_at(1e-3), margins_at(1.0)) else {
        // The F4 refusal row (module docs): `margins_at` asserted the
        // typed in-band `bool_ring_run_winding` signature — that
        // refusal IS this row's pin; the linearity comparison has no
        // computed twin to run on.
        println!(
            "ε {eps:e}: linearity comparison skipped — the F4 refusal \
             signature is this row's claim"
        );
        return;
    };
    assert_eq!(
        mm.keys().collect::<Vec<_>>(),
        m.keys().collect::<Vec<_>>(),
        "the twins must fire the same predicate set"
    );
    let mut nonconforming: Vec<String> = Vec::new();
    for (pred, mm_list) in &mm {
        let m_list = &m[pred];
        if mm_list.len() != m_list.len() {
            assert!(
                KNOWN_SCALE_DEPENDENT_COUNTS.contains(pred),
                "{pred}: the twins decided different numbers of times \
                 ({} vs {}) and the predicate is not a documented \
                 search-lane loop",
                mm_list.len(),
                m_list.len()
            );
            continue; // streams not comparable elementwise
        }
        // The verdict-stream pin: outcomes are scale-free and must be
        // IDENTICAL sample-for-sample in recording order.
        let outcomes = |l: &[(SampleOutcome, f64)]| l.iter().map(|s| s.0).collect::<Vec<_>>();
        assert_eq!(
            outcomes(mm_list),
            outcomes(m_list),
            "{pred}: the twins must produce identical verdict streams"
        );
        // The margin pin: DECISIVE margins (definite nonzero — actual
        // separations, not coincident-residual fp noise) must scale
        // linearly. Sorted pairing: recording order pairs unrelated
        // sites when a predicate fires on several entities.
        let decisive = |l: &[(SampleOutcome, f64)]| {
            let mut v: Vec<f64> = l
                .iter()
                .filter(|s| {
                    matches!(
                        s.0,
                        SampleOutcome::Definite(Sign::Positive | Sign::Negative)
                    )
                })
                .map(|s| s.1)
                // The CANONICAL SCAFFOLDING CIRCLE
                // (`EdgeCurveSpec::self_loop_circle_at`: radius exactly
                // 1, span exactly (0, τ)) is deliberately scale-free
                // geometry — euler-op self-loop sites certify it at
                // every model scale with the bit-identical
                // `interval_span_forward` margin τ·1 m. Exempt exactly
                // those samples (that predicate, that bit pattern); a
                // real model span colliding with τ to the last bit is
                // not a realizable concern.
                .filter(|margin| {
                    !(*pred == "interval_span_forward" && *margin == core::f64::consts::TAU)
                })
                .collect();
            v.sort_by(f64::total_cmp);
            v
        };
        let mut worst: f64 = 0.0;
        for (a, b) in decisive(mm_list).iter().zip(&decisive(m_list)) {
            worst = worst.max((b / a / 1e3 - 1.0).abs());
        }
        if worst > 1e-9 {
            nonconforming.push(format!("{pred} (worst rel dev {worst:.3e})"));
            println!(
                "{pred} decisive margins:\n  mm: {:?}\n   m: {:?}",
                decisive(mm_list),
                decisive(m_list)
            );
        }
    }
    // The audited FLAG comparands are allowed to break linearity —
    // they are the documented findings, listed so their eventual fix
    // flips them out of this allowlist. Anything else is a NEW
    // dimensional defect and fails loudly.
    let unexpected: Vec<_> = nonconforming
        .iter()
        .filter(|n| !KNOWN_NONLINEAR.iter().any(|k| n.starts_with(k)))
        .collect();
    println!("nonlinear (documented FLAG comparands): {nonconforming:?}");
    assert!(
        unexpected.is_empty(),
        "predicates with non-length-scaling margins outside the audited \
         FLAG list: {unexpected:?}"
    );
    // The fixed class-(c) sites this suite exists to pin: they must
    // FIRE here and hence scale (they passed the loop above). If a
    // refactor stops one from firing, the pin goes vacuous — fail
    // loudly instead so the pin moves with the code.
    for fixed in ["bool_join_facing", "pm_census_ee_parallel"] {
        assert!(
            mm.contains_key(fixed),
            "{fixed}: expected to fire in the twin booleans (pin vacuous)"
        );
        assert!(
            mm[fixed].iter().any(|v| v.1 != 0.0),
            "{fixed}: expected a nonzero margin to pin the metering"
        );
    }
}
