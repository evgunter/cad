//! Scratch probe for the M10-7 lane: what the symbolic tier does on the
//! M10-3 fixtures. Evidence-only — it prints and asserts nothing that
//! gates.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, SymbolicDials, drive};
use geom_core::Tol;

use crate::m10_3_driver_interval::slab;

#[test]
#[ignore = "evidence-only probe: prints the tier's receipt"]
fn probe_the_tier_on_the_slab() {
    let _ = fixture::len(1.0);
    for half in [Tol::witness().eps() / 16.0, 0.05] {
        let doc = slab(1.0, half);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        for dials in [SymbolicDials::default(), SymbolicDials::off()] {
            let v = drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_leaves: 8,
                    symbolic: dials,
                    ..DriveConfig::default()
                },
                Tol::witness(),
            )
            .unwrap();
            println!(
                "half={half:e} enabled={} certified={} refused={} decisions={:?}",
                dials.enabled,
                v.receipt().certified,
                v.receipt().refused,
                v.decisions()
            );
        }
    }
}

/// Which predicate is still indeterminate over the macroscopic box.
#[test]
#[ignore = "evidence-only probe: names the surviving indeterminacies"]
fn probe_which_predicate_still_widens() {
    use editor_core::analysis::{ParamBox, param_env_over};
    use editor_core::{CancelToken, EvalOptions, NodeResult, ProfileLift, evaluate};
    use std::sync::Arc;

    let doc = slab(1.0, 0.05);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = ParamBox::of(&analyzed);
    let _ = param_env_over::<geom_core::Interval, _>(&doc, &box_);
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_)),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    let (ev, counts) = geom_core::sym::with_session(
        geom_core::SymBudget {
            max_terms: 1_000_000,
            max_degree: 100_000,
        },
        || {
            let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
                evaluate(&doc, None, &CancelToken::new(), &opts, Tol::witness());
            ev
        },
    );
    println!("counts {counts:?}");
    for (id, r) in &ev.nodes {
        if let NodeResult::Failed(e) = r {
            println!("node {id:?} FAILED: {}", e.kind);
        }
    }
}

/// Which budget the macroscopic slab actually needs.
#[test]
#[ignore = "evidence-only probe: sweeps the freezing budget"]
fn probe_the_budget_the_slab_needs() {
    use editor_core::analysis::ParamBox;
    use editor_core::{CancelToken, EvalOptions, NodeResult, ProfileLift, evaluate};
    use std::sync::Arc;

    let doc = slab(1.0, 0.05);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = ParamBox::of(&analyzed);
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_)),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    for max_degree in [8u32, 16, 24, 32, 48, 64, 128, 256] {
        for max_terms in [64usize, 256, 1024, 4096] {
            let (ev, counts) = geom_core::sym::with_session(
                geom_core::SymBudget {
                    max_terms,
                    max_degree,
                },
                || {
                    let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
                        evaluate(&doc, None, &CancelToken::new(), &opts, Tol::witness());
                    ev
                },
            );
            let failed = ev
                .nodes
                .values()
                .filter(|r| matches!(r, NodeResult::Failed(_)))
                .count();
            println!(
                "degree={max_degree:>4} terms={max_terms:>5} failed={failed} sym={} num={} frozen={}",
                counts.symbolic_zero, counts.numeric, counts.frozen
            );
        }
    }
}

/// Does a `Sym<Interval>` replay produce the same per-node content keys
/// as a plain `Interval` replay over the same box?
#[test]
#[ignore = "evidence-only probe: key differential across the tier"]
fn probe_key_differential() {
    use editor_core::analysis::ParamBox;
    use editor_core::{CancelToken, EvalOptions, ProfileLift, evaluate};
    use std::sync::Arc;

    for half in [Tol::witness().eps() / 8.0, 0.05] {
        let doc = slab(1.0, half);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let box_ = ParamBox::of(&analyzed);
        let opts = EvalOptions {
            param_box: Some(Arc::new(box_)),
            profile_lift: ProfileLift::Guided,
            ..EvalOptions::default()
        };
        let plain: editor_core::Evaluation<geom_core::Interval> =
            evaluate(&doc, None, &CancelToken::new(), &opts, Tol::witness());
        let (symb, _) = geom_core::sym::with_session(
            geom_core::SymBudget {
                max_terms: 4096,
                max_degree: 128,
            },
            || {
                let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
                    evaluate(&doc, None, &CancelToken::new(), &opts, Tol::witness());
                ev
            },
        );
        for id in &plain.order {
            let a = plain.value(*id).map(|v| v.content_key);
            let b = symb.value(*id).map(|v| v.content_key);
            println!(
                "half={half:e} node {id:?} plain={} sym={} same={}",
                a.is_some(),
                b.is_some(),
                a == b
            );
        }
    }
}

// ------------------------------------------- what the tier COSTS (D12)

/// EVIDENCE-ONLY, and the source of deviation D12's numbers: what one
/// leaf replay and one drive cost with the tier on against with it off,
/// and how many forms freeze on CURVED geometry.
///
/// Run it, do not read it: the numbers move with the machine. What the
/// PR quotes is a RATIO measured in one process, which is the part that
/// travels.
#[test]
#[ignore = "evidence-only: prints the tier's cost and freeze counts"]
fn m10_7_what_the_tier_costs() {
    use std::time::Instant;
    let tol = Tol::witness();

    let drive_cost = |name: &str, doc: &editor_core::ProfileDoc, leaves: usize| {
        let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
        for (lane, dials) in [
            ("on", SymbolicDials::default()),
            ("off", SymbolicDials::off()),
        ] {
            let cfg = DriveConfig {
                max_leaves: leaves,
                symbolic: dials,
                ..DriveConfig::default()
            };
            let t0 = Instant::now();
            let v = drive(doc, &analyzed, &cfg, tol);
            let dt = t0.elapsed();
            match v {
                Ok(v) => println!(
                    "COST {name} leaves={leaves} tier={lane}: {dt:?} certified={} refused={} \
                     decisions={:?}",
                    v.receipt().certified,
                    v.receipt().refused,
                    v.decisions()
                ),
                Err(e) => println!("COST {name} leaves={leaves} tier={lane}: {dt:?} refused {e}"),
            }
        }
    };

    // The slab, at the macroscopic box the unit's headline row uses.
    let doc = slab(1.0, 0.05);
    drive_cost("slab/macroscopic", &doc, 32);
    drive_cost("slab/256", &doc, 256);

    // CURVED geometry — where the freezes come from `i128` coefficient
    // overflow rather than from the dials, which is D6's gap.
    #[cfg(feature = "probe")]
    {
        let (bracket, _, _) = crate::m10_7_r1_probes_interval::bracket_pub(0.5e-3, false, tol);
        drive_cost("bracket/curved", &bracket, 32);
        for max_terms in [4096usize, 1 << 20] {
            let analyzed = analyzed_box(&bracket, &AnalysisPolicy::default());
            let cfg = DriveConfig {
                max_leaves: 32,
                symbolic: SymbolicDials {
                    max_terms,
                    max_degree: 4096,
                    ..SymbolicDials::default()
                },
                ..DriveConfig::default()
            };
            match drive(&bracket, &analyzed, &cfg, tol) {
                Ok(v) => println!(
                    "FREEZE bracket/curved terms={max_terms} degree=4096: decisions={:?}",
                    v.decisions()
                ),
                Err(e) => println!("FREEZE bracket/curved terms={max_terms}: refused {e}"),
            }
        }
    }
}
