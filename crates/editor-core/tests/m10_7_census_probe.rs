//! **M10-7's honesty instruments, as runnable evidence.**
//!
//! Two probes, both `#[ignore]`d because they REPORT rather than gate
//! ([[test-suite-cost]]: a row that only prints cannot fail, so it must
//! not sit in the ε matrix):
//!
//! - `census_which_predicates_decide_symbolically` names, per funnel
//!   predicate, how many of its decisions the symbolic identity tier
//!   answered — the evidence column of the census table in
//!   `geom_core::sym`'s module docs;
//! - `measure_the_ceiling_on_the_two_hole_plate` bisects the widest box
//!   of the tour's own plate that certifies, and names what refuses
//!   first beyond it.
//!
//! Run them:
//!
//! ```sh
//! cargo test -p editor-core --features probe,interval --test all -- \
//!   m10_7_census_probe:: --ignored --nocapture
//! ```
#![cfg(all(feature = "interval", feature = "probe"))]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{
    DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS, DriveConfig, SymbolicDials, drive,
};
use editor_core::{CancelToken, EvalOptions, ProfileLift, evaluate};
use geom_core::k_stats::{SampleOutcome, start_recording, take_samples};
use geom_core::{SymBudget, Tol};

use crate::m10_3_driver_interval::slab;
use crate::m10_7_plate::plate;

fn budget() -> SymBudget {
    SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    }
}

/// The per-predicate split, at the DEGENERATE box on the nominal.
///
/// `Probe` is a point scalar — `AxisScalar::axis` refuses a widened
/// span at it — so the recording lane samples a POINT, exactly as the
/// driver's `KProbe::CertifiedMidpoints` replay does. The parameters
/// are still SYMBOLS there (`axis_named` mints one per parameter
/// whatever its width), so the identity test answers the same question
/// it answers over a box: is this margin's expression identically zero
/// in the parameters. What a point cannot show is the WIDENING, which
/// is the driver's own rows' business.
fn split(
    doc: &editor_core::ProfileDoc,
    tol: Tol,
) -> BTreeMap<&'static str, (u64, u64)> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let nominal = ParamBox::from_axes(
        ParamBox::of(&analyzed)
            .axes()
            .keys()
            .map(|n| (n.clone(), BoxAxis::Fixed))
            .collect(),
    );
    let opts = EvalOptions {
        param_box: Some(Arc::new(nominal)),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    start_recording();
    let _ = geom_core::sym::with_session(budget(), || {
        let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Probe>> =
            evaluate(doc, None, &CancelToken::new(), &opts, tol);
        ev
    });
    let mut out: BTreeMap<&'static str, (u64, u64)> = BTreeMap::new();
    for s in take_samples() {
        let row = out.entry(s.predicate).or_default();
        match s.outcome {
            SampleOutcome::SymbolicZero => row.0 += 1,
            _ => row.1 += 1,
        }
    }
    out
}

/// **The census's evidence column**: which funnel predicates the tier
/// actually discharges, on the M10 corpus fixtures and the tour's plate.
///
/// EVIDENCE-ONLY (it prints). `Sym<Probe>` is the recording scalar with
/// the tier in front of it, so every row is a sample the SAME funnel
/// recorded — no second channel.
#[test]
#[ignore = "evidence-only: prints the per-predicate symbolic/numeric split"]
fn census_which_predicates_decide_symbolically() {
    let tol = Tol::witness();
    let mut total: BTreeMap<&'static str, (u64, u64)> = BTreeMap::new();
    let mut add = |name: &str, doc: &editor_core::ProfileDoc| {
        let rows = split(doc, tol);
        println!("== {name}");
        for (pred, (sym, num)) in &rows {
            println!("   {pred:<40} symbolic={sym:<6} numeric={num}");
            let e = total.entry(pred).or_default();
            e.0 += sym;
            e.1 += num;
        }
    };
    add("slab(1.0, 0.05)", &slab(1.0, 0.05));
    add("two_hole_plate", &plate(5.0e-5, 1.0e-5, tol).0);
    println!("== TOTAL over both documents");
    for (pred, (sym, num)) in &total {
        println!("   {pred:<40} symbolic={sym:<6} numeric={num}");
    }
    println!(
        "   {} predicate names seen; {} of them decide symbolically at least once",
        total.len(),
        total.values().filter(|(s, _)| *s > 0).count()
    );
}

/// **The re-measured ceiling, on the tour's own two-hole plate.**
///
/// EVIDENCE-ONLY (it prints the number, the tier-off comparison, and
/// what refuses first beyond it). The measurement is the WIDEST BOX THAT
/// CERTIFIES WHOLE — `max_depth = 0`, so exactly one leaf and no
/// subdivision — which is the same shape `m10_3_driver_interval`'s own
/// width rows measure, and the only shape whose answer is about the
/// enclosure rather than about the leaf budget.
///
/// The scale is on the study's own tolerances (±0.05 mm on the spacing,
/// σ = 0.01 mm on each radius, scaled together), so `x1` IS the real
/// study and the number reads as "what fraction of a real study's box
/// certifies in one leaf".
#[test]
#[ignore = "evidence-only: prints the measured ceiling and its first refusal"]
fn measure_the_ceiling_on_the_two_hole_plate() {
    let tol = Tol::witness();
    let doc_at = |scale: f64| plate(5.0e-5 * scale, 1.0e-5 * scale, tol).0;
    let certifies_whole = |scale: f64, dials: SymbolicDials| {
        let doc = doc_at(scale);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_depth: 0,
                max_leaves: 1,
                symbolic: dials,
                ..DriveConfig::default()
            },
            tol,
        )
        .is_ok_and(|v| v.receipt().certified == 1)
    };
    let ceiling = |dials: SymbolicDials| -> f64 {
        let (mut lo, mut hi) = (1.0e-12, 1.0);
        assert!(certifies_whole(lo, dials), "the bracket's floor certifies");
        assert!(!certifies_whole(hi, dials), "the real study does not");
        // Bisect the LOG of the scale: the answer spans decades.
        for _ in 0..30 {
            let mid = (0.5 * (lo.ln() + hi.ln())).exp();
            if certifies_whole(mid, dials) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        lo
    };
    let off = ceiling(SymbolicDials::off());
    let on = ceiling(SymbolicDials::default());
    println!("   TIER OFF: the widest whole-certifying box is x{off:e} of the real study");
    println!("   TIER ON : the widest whole-certifying box is x{on:e} of the real study");
    println!("   the ceiling moved by a factor of {:e}", on / off);

    // What refuses FIRST beyond it, named — the leaf replayed directly
    // so the predicate is in the message rather than behind a `Bisect`.
    let beyond = on * 2.0;
    let doc = doc_at(beyond);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    let (ev, counts) = geom_core::sym::with_session(budget(), || {
        let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
            evaluate(&doc, None, &CancelToken::new(), &opts, tol);
        ev
    });
    println!("   at x{beyond:e} the leaf replay decides {counts:?}");
    for id in &ev.order {
        if let Some(editor_core::NodeResult::Failed(e)) = ev.result(*id) {
            println!("   FIRST REFUSAL beyond the ceiling: node {} — {}", id.0, e.kind);
            break;
        }
    }
}
