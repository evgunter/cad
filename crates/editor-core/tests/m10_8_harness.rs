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

/// **The dials of a RULES differential**: the tier on at the shipped
/// budget, `rules` as chosen, and NO retry ladder
/// (`geom_core::SymRetry::none`).
///
/// A rules differential measures what one rule set reaches against
/// another, and the shipped ladder (`drive::DEFAULT_SYM_RETRY`, in
/// `SymbolicDials::default()`) is a second rule set run into each
/// side's refusals — its first attempt shuts rule G, so a row comparing
/// rule G on against rule G off with the ladder on both sides would
/// read rule G's cost as recovered and stay green. So every
/// differential in this crate's suites takes its dials from here, and
/// [`split_at_the_nominal`] is the same principle for a replay. A row
/// that wants the tier a drive ships sets `retry` itself and says so.
pub(crate) fn dials(rules: SymRules) -> SymbolicDials {
    SymbolicDials {
        rules,
        retry: geom_core::SymRetry::none(),
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

/// **THE PER-PREDICATE SPLIT of one replay**: `predicate -> [theorem,
/// sign-gated, registered, numeric]`, every predicate that decided at
/// all. The spelling of the DISCHARGE table that the pins, the
/// evidence rows and the probes read; a pin asserts it whole
/// (`assert_split`).
///
/// It collapses everything the numeric channel answered into one
/// column, which is the right shape for a claim about what the TIER
/// discharged and the wrong shape for a claim about what BLOCKED:
/// [`blocked`] is that table, and the two are siblings over the same
/// `shapes` rather than one table with a column nobody reads.
pub(crate) fn split(shapes: &[DecisionShape]) -> BTreeMap<&'static str, [u64; 4]> {
    let mut table: BTreeMap<&'static str, [u64; 4]> = BTreeMap::new();
    for s in shapes {
        let row = table.entry(s.predicate).or_default();
        row[match s.outcome {
            ShapeOutcome::Theorem => 0,
            ShapeOutcome::SignGated => 1,
            ShapeOutcome::Registered => 2,
            _ => 3,
        }] += 1;
    }
    table
}

/// **THE PER-PREDICATE BLOCKED TABLE of one replay**: for every
/// predicate the numeric channel could not decide at least once,
/// `(invalid, indeterminate, all)` — a clause-1 domain violation
/// (`Decide for Sym<T>`'s `Invalid` arm, where the tier is never
/// asked), an enclosure the band could not classify, and how many
/// decisions that predicate made in all.
///
/// The `all` column is what keeps a zero honest: a replay escalates at
/// its first blocked predicate and STOPS, so an `invalid` of zero is
/// zero over the decisions that were seen, not over the document.
pub(crate) fn blocked(shapes: &[DecisionShape]) -> BTreeMap<&'static str, (usize, usize, usize)> {
    let mut table: BTreeMap<&'static str, (usize, usize, usize)> = BTreeMap::new();
    for s in shapes {
        let row = table.entry(s.predicate).or_default();
        row.2 += 1;
        match s.outcome {
            ShapeOutcome::Invalid => row.0 += 1,
            ShapeOutcome::Indeterminate => row.1 += 1,
            _ => {}
        }
    }
    table.retain(|_, (invalid, indeterminate, _)| *invalid > 0 || *indeterminate > 0);
    table
}

/// The first `n` CHARACTERS of a rendering, with the full length said:
/// a form that reaches the budget renders to megabytes, and what a
/// reader needs is its head. Counted in characters at both ends — a
/// residual carries `·`, `√` and `−`, so a byte length beside a
/// character cut is two different numbers.
pub(crate) fn head(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        return s.to_owned();
    }
    let cut: String = s.chars().take(n).collect();
    format!("{cut}… [{} chars]", s.chars().count())
}

/// The split of `doc`'s NOMINAL replay under `rules` ([`split`] over
/// [`nominal_box`]), the tier making ONE attempt per rung.
///
/// **No retry ladder**, deliberately: every caller of this door is a
/// RULES differential — what one rule reaches on one document — and a
/// ladder installed here would put a second rule set inside both sides
/// of it. [`split_at_the_nominal_retried`] is the door for a row that
/// wants the tier a drive actually runs.
pub(crate) fn split_at_the_nominal(
    doc: &ProfileDoc,
    rules: SymRules,
    tol: Tol,
) -> BTreeMap<&'static str, [u64; 4]> {
    split_at_the_nominal_retried(doc, rules, geom_core::SymRetry::none(), tol)
}

/// [`split_at_the_nominal`] with a RETRY LADDER installed
/// (`geom_core::SymRetry`) — the tier a drive runs, dials and all.
pub(crate) fn split_at_the_nominal_retried(
    doc: &ProfileDoc,
    rules: SymRules,
    retry: geom_core::SymRetry,
    tol: Tol,
) -> BTreeMap<&'static str, [u64; 4]> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let (shapes, _, _) = crate::m10_8_arc_family_interval::replay_retried(
        doc,
        &nominal_box(&analyzed),
        rules,
        retry,
        tol,
    );
    split(&shapes)
}

/// **Asserts a split WHOLE**: every listed predicate at its listed
/// counts, and no predicate outside the list — one that appears reds
/// naming it, so a pin holds the whole table and not the rows it
/// remembered to name. Zero-count predicates never appear in a split,
/// so the list is exactly the predicates that decided.
#[allow(clippy::panic)]
pub(crate) fn assert_split(
    name: &str,
    table: &BTreeMap<&'static str, [u64; 4]>,
    expected: &[(&str, [u64; 4])],
) {
    // EVERY row that moved, in one panic. A per-row `assert_eq!` stops
    // at the first, so re-pinning a table that moved on six rows cost
    // six runs of a suite that takes minutes — and each run told the
    // reader one sixth of what had happened.
    let mut moved: Vec<String> = Vec::new();
    for (pred, want) in expected {
        let got = table
            .get(pred)
            .copied()
            .unwrap_or_else(|| panic!("{name}: no {pred} decisions — the pinned table lists it"));
        if got != *want {
            moved.push(format!("{pred} {want:?} -> {got:?}"));
        }
    }
    assert!(
        moved.is_empty(),
        "{name}: these moved at the nominal (theorem/gated/registered/numeric); if a rule \
         took them, re-pin and say which: {moved:?}"
    );
    let unlisted: Vec<String> = table
        .iter()
        .filter(|(p, _)| !expected.iter().any(|(e, _)| e == *p))
        .map(|(p, row)| format!("{p} {row:?}"))
        .collect();
    assert!(
        unlisted.is_empty(),
        "{name}: predicates decided that the pinned table does not list: {unlisted:?}"
    );
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
