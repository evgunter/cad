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
//! # The F3+F4 flip — THIS UNIT is the one the old comments promised
//!
//! This suite's ε-row structure used to encode two deferred findings.
//! Both are now FIXED, and the pins moved with them:
//!
//! - **F4** (`bool_ring_run_winding`): the mm twin's ring-winding AREA
//!   margins (measured 2e-6/6e-6/8e-6 here) landed INSIDE
//!   `Band{ε, Kε}` on coarse rows, and at ε = 1e-6 the pocket subtract
//!   REFUSED typed — a real mm-scale boolean refusal on a hosted CI
//!   row, which this suite pinned as F4's live signature with a
//!   three-outcome match. The predicate now decides `2A/P` (the run's
//!   MEAN WIDTH, a length) at all three of its sites, so at mm scale
//!   those margins are 5e-4 / 7.5e-4 / 1e-3 m (measured here) —
//!   decisively out of every band in the matrix, and at the mm twin's
//!   own feature scale instead of quadratically below it. The refusal
//!   arm and the `Option` return that carried it are GONE: the pocket
//!   subtract computes on every row, and `bool_ring_run_winding` is
//!   pinned LINEAR below.
//! - **F3** (`volume_backstop`): the backstop's raw `sign_within`
//!   bypass is retired — both its gates decide through the funnel,
//!   under `volume_backstop` / `volume_backstop_operand`, metered to
//!   lengths (ΔV over the compared bodies' summed surface area;
//!   measured here 1.0e-5 / 2.1e-5 m and 2.1e-4 / 3.3e-4 m at mm
//!   scale). The stale-name contamination of `witness_at_mid_parameter`
//!   that this suite MEASURED and allowlisted is therefore gone — and
//!   the proof is that its decisive list is now EMPTY at both scales
//!   (the {1, 1, 3, 8, 8, 16} m³ "witness distances" were the whole of
//!   it; its own samples are coincident residuals, as the old comment
//!   claimed). Both its nonlinearity entry and its scale-dependent-count
//!   exemption are dropped, so the pin now holds it to the full
//!   elementwise claim.
//!
//! **NO TEST IN THIS FILE IS EXECUTED BY CI.** The probe suites CI runs are
//! rostered in `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run
//! by `scripts/k_probe_sweep.sh`; this one is on neither list, so nothing
//! here can go red on a merge and its assertions are evidence for a reader
//! rather than a gate. By hand:
//! `cargo test -p topo --features probe --test all -- rim_dim_boolean_twins::`.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::collections::BTreeMap;

use common::prism_z;
use geom_core::Sign;
use geom_core::Tol;
use geom_core::k_stats::{self, Probe, SampleOutcome};
use topo::{BooleanResult, subtract};

/// Audited, documented non-length comparands still awaiting their own
/// units (docs/predicate-dimension-audit.md FLAG rows). Everything
/// else must scale linearly.
/// (F3 and F4 were on this list until this unit; the module docs
/// record what they were and what retired them.)
const KNOWN_NONLINEAR: &[&str] = &[
    // F2: ray-caster denominators (dimensionless / 1/m).
    "bool_point_in_solid_denom",
    "bool_ray_cylinder_disc",
];

/// Predicates whose DECISION COUNT may differ between the twins.
/// EMPTY since the F3 fix: the only entry was
/// `witness_at_mid_parameter`, whose 102-vs-103 mismatch was not its
/// own — it was the volume backstop's bypassed decisions riding under
/// its name (module docs). With the bypass routed through the funnel,
/// every predicate's twin counts must match, and a mismatch is a
/// finding.
const KNOWN_SCALE_DEPENDENT_COUNTS: &[&str] = &[];

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
/// order. Total since the F4 fix: both subtracts COMPUTE at every ε
/// row in the matrix (module docs — this used to return `None` on the
/// rows where the mm pocket subtract refused in-band).
fn margins_at(scale: f64) -> BTreeMap<&'static str, Vec<(SampleOutcome, f64)>> {
    let s = |v: f64| v * scale;
    k_stats::start_recording();
    // Corner overlap: generic crossing subtract.
    let a = box_at(&s, (0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = box_at(&s, (1.0, 3.0), (1.0, 3.0), (1.0, 3.0));
    let r = subtract(&a, &b, Tol::witness()).expect("corner subtract");
    let BooleanResult::Body(rb) = r else {
        panic!("corner: body out");
    };
    // The pseudomanifold census over the result (empty declarations):
    // the pm_census_* sweeps — including the fixed
    // `pm_census_ee_parallel` — decide every entity pair.
    topo::validate_pseudomanifold(&rb.body, &topo::ContactRecords::default(), Tol::witness())
        .expect("corner census");
    // Through-pocket: the tool pierces the top and bottom faces, so
    // the result carries ring loops (the point-in-loop lane). This is
    // the configuration whose mm twin refused in-band on F4's area
    // comparand at ε = 1e-6; with the winding metered to a mean width
    // it computes at every ε row, so ANY refusal here is now a finding.
    let a2 = box_at(&s, (0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    let b2 = box_at(&s, (1.0, 2.0), (1.0, 2.0), (-1.0, 2.0));
    match subtract(&a2, &b2, Tol::witness()) {
        Ok(BooleanResult::Body(_)) => {}
        Ok(other) => panic!("pocket: expected a body, got {other:?}"),
        Err(other) => panic!(
            "pocket subtract refused at scale {scale:e} — the F3+F4 fixes retired \
             every known in-band refusal on this configuration: {other:?}"
        ),
    }
    let mut out: BTreeMap<&'static str, Vec<(SampleOutcome, f64)>> = BTreeMap::new();
    for sample in k_stats::take_samples() {
        out.entry(sample.predicate)
            .or_default()
            .push((sample.outcome, sample.margin.abs()));
    }
    out
}

/// The pin: same predicates, same counts, and every margin pair at
/// ratio exactly `scale_ratio` (rel 1e-9) — the margin stream of a
/// scaled body is the scaled margin stream, predicate by predicate,
/// sample by sample.
#[test]
fn boolean_margin_streams_scale_linearly_with_the_model() {
    let eps = geom_core::Tol::witness().get().eps;
    // Since the F3+F4 fixes there is no skip arm: EVERY ε row in the
    // hosted matrix runs the full elementwise comparison below.
    println!("ε {eps:e}: both twins computed — running the full linearity pin");
    let (mm, m) = (margins_at(1e-3), margins_at(1.0));
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
    //
    // `bool_ring_run_winding` (F4) and the two `volume_backstop*`
    // gates (F3) are on this list BECAUSE of this unit: their
    // presence here is what makes their absence from KNOWN_NONLINEAR
    // a claim rather than a silence — they fire, and they scale.
    //
    // `bool_join_chord` is here for the ASSIGNMENT, not the metering:
    // it decides the germ-chord LENGTH, so its samples are the join
    // family's model-scale distances, while `bool_join_nearest`
    // decides a DIFFERENCE of two such lengths and on these fixtures
    // every incumbent comparison ties at exactly 0. Swap the two names
    // and the nonzero-margin assertion below goes red.
    for fixed in [
        "bool_join_chord",
        "bool_join_facing",
        "pm_census_ee_parallel",
        "bool_ring_run_winding",
        "volume_backstop",
        "volume_backstop_operand",
    ] {
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
