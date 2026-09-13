//! **The M10-8 measurement harness** — the ONE home of the whole-box
//! probe, the log-bisected ceiling and the degenerate nominal box that
//! the pins, the evidence suite and both reviewers' suites share. Both
//! reviews found these re-derived five times over (their Q1); a copy
//! per suite is a copy per suite of whatever a future change to the
//! drive's whole-box shape has to be made in.
#![cfg(feature = "interval")]
#![allow(dead_code)]

use std::time::Instant;

use std::collections::BTreeMap;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, AnalyzedBox, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, SymbolicDials, drive};
use geom_core::sym::report::{DecisionShape, ShapeOutcome};
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

/// One predicate over the band in a replay: its name, the widest
/// enclosure the band could not classify, how many of its decisions
/// were over the band, and how many it made in all.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct OverBand {
    pub(crate) predicate: &'static str,
    pub(crate) enclosure: (f64, f64),
    pub(crate) over: usize,
    pub(crate) all: usize,
}

/// **THE OVER-BAND SET of one replay**: every predicate with at least
/// one decision the band could not classify, the widest such enclosure,
/// and the counts — sorted widest-band first, so the predicate furthest
/// over the band is the one a slightly narrower study would still be
/// stopped by.
///
/// This is what a BOUND is, and it is the only spelling the tree uses.
/// A drive stops at its FIRST refusal, and at any scale past the
/// ceiling several predicates can be over the band at once, so the
/// name a drive reports is a fact about evaluation ORDER (validation
/// before certification, the sample schedule in order) and not about
/// the document. The set, read at the refusing end of the bisection
/// bracket ([`bound`]), is a fact about the document.
pub(crate) fn over_band_set(shapes: &[DecisionShape]) -> Vec<OverBand> {
    let mut out: BTreeMap<&'static str, OverBand> = BTreeMap::new();
    for s in shapes {
        let e = out.entry(s.predicate).or_insert(OverBand {
            predicate: s.predicate,
            enclosure: (f64::INFINITY, f64::NEG_INFINITY),
            over: 0,
            all: 0,
        });
        e.all += 1;
        if !matches!(
            s.outcome,
            ShapeOutcome::Indeterminate | ShapeOutcome::Invalid
        ) {
            continue;
        }
        e.over += 1;
        if let Some((lo, hi)) = s.enclosure {
            e.enclosure.0 = e.enclosure.0.min(lo);
            e.enclosure.1 = e.enclosure.1.max(hi);
        }
    }
    let mut rows: Vec<OverBand> = out.into_values().filter(|e| e.over > 0).collect();
    rows.sort_by(|a, b| {
        (b.enclosure.1 - b.enclosure.0)
            .partial_cmp(&(a.enclosure.1 - a.enclosure.0))
            .unwrap_or(core::cmp::Ordering::Equal)
    });
    rows
}

/// **WHAT BOUNDS A DOCUMENT**: its whole-certifying bracket
/// (`(certifies, refuses)`, [`ceiling`]) and the over-band set at the
/// REFUSING END of that bracket — ceiling + δ, never a multiple of it.
/// `None` for the set when the bracket has no finite refusing end.
pub(crate) fn bound(
    doc_at: &dyn Fn(f64) -> ProfileDoc,
    rules: SymRules,
    tol: Tol,
    lo: f64,
    hi: f64,
    steps: usize,
) -> (f64, f64, Option<Vec<OverBand>>) {
    let (lo, hi, _) = ceiling(doc_at, rules, tol, lo, hi, steps);
    if !(lo.is_finite() && hi.is_finite()) {
        return (lo, hi, None);
    }
    let doc = doc_at(hi);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let (shapes, _, _) =
        crate::m10_8_arc_family_interval::replay(&doc, &ParamBox::of(&analyzed), rules, tol);
    (lo, hi, Some(over_band_set(&shapes)))
}

/// One line per over-band predicate, for the evidence rows.
pub(crate) fn render_over_band(set: &[OverBand]) -> String {
    set.iter()
        .map(|e| {
            format!(
                "OVER BAND {:<34} [{:>11.4e},{:>11.4e}] {:>3}/{:<4}",
                e.predicate, e.enclosure.0, e.enclosure.1, e.over, e.all
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
