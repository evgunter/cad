//! **The M10-8 measurement harness** — the ONE home of the whole-box
//! probe, the log-bisected ceiling and the degenerate nominal box that
//! the pins, the evidence suite and both reviewers' suites share. Both
//! reviews found these re-derived five times over (their Q1); a copy
//! per suite is a copy per suite of whatever a future change to the
//! drive's whole-box shape has to be made in.
#![cfg(feature = "interval")]
#![allow(dead_code)]

use std::time::Instant;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, AnalyzedBox, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, SymbolicDials, drive};
use geom_core::{SymRules, Tol};

/// The dials with a chosen rule set, the tier on at the shipped budget.
pub(crate) fn dials(rules: SymRules) -> SymbolicDials {
    SymbolicDials {
        rules,
        ..SymbolicDials::default()
    }
}

/// Whether `doc` certifies its WHOLE analyzed box in one leaf under
/// `dials` — `max_depth = 0`, one leaf, the receipt's `certified == 1`.
pub(crate) fn certifies_whole_with(doc: &ProfileDoc, dials: SymbolicDials, tol: Tol) -> bool {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    drive(
        doc,
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
}

/// [`certifies_whole_with`] at the shipped budget under `rules`.
pub(crate) fn certifies_whole(doc: &ProfileDoc, rules: SymRules, tol: Tol) -> bool {
    certifies_whole_with(doc, dials(rules), tol)
}

/// The degenerate box at the nominal: every axis of the analyzed box
/// fixed at its nominal value.
pub(crate) fn nominal_box(analyzed: &AnalyzedBox) -> ParamBox {
    ParamBox::from_axes(
        ParamBox::of(analyzed)
            .axes()
            .keys()
            .map(|n| (n.clone(), BoxAxis::Fixed))
            .collect(),
    )
}

/// The widest whole-certifying scale of a document's study between `lo`
/// and `hi`, by `steps` bisections of the LOG of the scale (the answer
/// spans decades), as the bracket `(certifies, refuses)` with the mean
/// wall time per probe. `(NaN, hi, _)` when even `lo` refuses;
/// `(hi, +inf, _)` when `hi` certifies.
pub(crate) fn ceiling(
    doc_at: &dyn Fn(f64) -> ProfileDoc,
    rules: SymRules,
    tol: Tol,
    lo: f64,
    hi: f64,
    steps: usize,
) -> (f64, f64, f64) {
    let (mut lo, mut hi) = (lo, hi);
    let mut probes = 0.0;
    let mut spent = 0.0;
    let mut probe = |s: f64| {
        let t = Instant::now();
        let ok = certifies_whole(&doc_at(s), rules, tol);
        spent += t.elapsed().as_secs_f64();
        probes += 1.0;
        ok
    };
    if !probe(lo) {
        return (f64::NAN, hi, spent / probes);
    }
    if probe(hi) {
        return (hi, f64::INFINITY, spent / probes);
    }
    for _ in 0..steps {
        let mid = (0.5 * (lo.ln() + hi.ln())).exp();
        if probe(mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo, hi, spent / probes)
}
