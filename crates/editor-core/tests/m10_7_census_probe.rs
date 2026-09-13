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
//!   of the tour's own plate that certifies, and reads what bounds it
//!   as the over-band SET at the bracket's refusing end (ceiling + δ).
//!
//! **NO TEST IN THIS FILE IS EXECUTED BY CI**, and that is deliberate
//! rather than a filter's accident: both rows are `#[ignore]`d evidence
//! probes that print and assert nothing a gate could read, and the whole
//! file is behind `#![cfg(all(feature = "interval", feature = "probe"))]`
//! besides. They are run by hand when the census or the ceiling has to be
//! re-measured — which is a unit's act, not a per-run one — and their
//! output is quoted where it is read: `geom_core::sym`'s module docs and
//! M10-7's PR body. `scripts/k_probe_sweep.sh` deliberately does not
//! roster them: neither writes a K CSV, so a sweep that ran them would
//! collect nothing.
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
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS, SymbolicDials};
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
fn split(doc: &editor_core::ProfileDoc, tol: Tol) -> BTreeMap<&'static str, (u64, u64)> {
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
/// the over-band SET at ceiling + δ). The measurement is the WIDEST BOX THAT
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
#[ignore = "evidence-only: prints the measured ceiling and the over-band set at ceiling + δ"]
fn measure_the_ceiling_on_the_two_hole_plate() {
    let tol = Tol::witness();
    let doc_at = |scale: f64| plate(5.0e-5 * scale, 1.0e-5 * scale, tol).0;
    let certifies_whole = |scale: f64, dials: SymbolicDials| {
        crate::m10_8_harness::certifies_whole_with(&doc_at(scale), dials, tol)
    };
    let ceiling = |dials: SymbolicDials| -> (f64, f64) {
        let (mut lo, mut hi) = (1.0e-12, 1.0);
        assert!(certifies_whole(lo, dials), "the bracket's floor certifies");
        // Under M10-10's tier the real study is past the ceiling but
        // not by decades; the search still starts at the study itself.
        if certifies_whole(hi, dials) {
            hi = 10.0;
        }
        assert!(!certifies_whole(hi, dials), "the bracket's top refuses");
        // Bisect the LOG of the scale: the answer spans decades.
        for _ in 0..30 {
            let mid = (0.5 * (lo.ln() + hi.ln())).exp();
            if certifies_whole(mid, dials) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (lo, hi)
    };
    let (off, _) = ceiling(SymbolicDials::off());
    let (on, on_hi) = ceiling(SymbolicDials::default());
    println!("   TIER OFF: the widest whole-certifying box is x{off:e} of the real study");
    println!("   TIER ON : the widest whole-certifying box is x{on:e} of the real study");
    println!("   the ceiling moved by a factor of {:e}", on / off);

    // What BOUNDS it: the over-band SET at the refusing end of the
    // bracket (ceiling + δ), never one drive's first refusal at a
    // multiple of the ceiling — past the ceiling several predicates
    // are over the band at once and the first name is evaluation
    // order (`work/m10/first-refusal-at-twice-the-ceiling-is-an-order-artefact`).
    let doc = doc_at(on_hi);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let (shapes, refusal, counts) = crate::m10_8_arc_family_interval::replay(
        &doc,
        &ParamBox::of(&analyzed),
        geom_core::SymRules::shipped(),
        tol,
    );
    println!(
        "   at x{on_hi:e} (ceiling + δ) the leaf replay decides {counts:?}; the drive stops at {refusal:?}"
    );
    println!(
        "   OVER THE BAND at ceiling + δ:\n{}",
        crate::m10_8_harness::render_over_band(&crate::m10_8_harness::over_band_set(&shapes))
    );
}
