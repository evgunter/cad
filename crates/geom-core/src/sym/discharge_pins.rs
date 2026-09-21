//! **A discharge kind is spelled five times; these rows hold the
//! spellings together** — one row per seam, in the shape of
//! `k-lint`'s `tests/outcome_vocabulary.rs`: enumerate one side's
//! roster, assert each member reaches exactly one member of the other,
//! and that the other side's members the projection does not reach are
//! NAMED here as the ones it cannot.
//!
//! # What happened, and why these rows exist
//!
//! A discharge outcome is spelled five times across two crates —
//! [`SymCounts::registered`](super::SymCounts::registered),
//! [`Discharge::Registered`](super::Discharge::Registered),
//! `report::ShapeOutcome::Registered`, `k_stats::SampleOutcome::Registered`
//! and `k_lint::Scan::registered`. They are five different types by
//! design (a count, a discharge reason, a report row, a K token and a
//! lint column are not one concept), and they must MOVE TOGETHER: a
//! sixth kind that reaches four of the five produces a receipt that
//! adds up over a population one of its columns silently mislabels.
//!
//! The registered-identity door's own five edits were made by hand, in
//! five files, and the order they were made in is why exactly ONE pair
//! was pinned — `SampleOutcome` against `k_lint::ACCEPTED_OUTCOMES`,
//! the seam a `cargo` build cannot see for itself because it crosses a
//! workspace boundary. The other seams a build CAN see, and sees
//! wrongly: adding a variant forces an arm at every `match`, and an arm
//! that folds the new kind into an existing column, row or token
//! compiles and passes.
//!
//! So what these rows assert is not totality — the compiler has that —
//! but INJECTIVITY: each kind reaching a column, a row and a token of
//! its OWN.
//!
//! # Why here
//!
//! [`Discharge`](super::Discharge) is private to `sym`, so an
//! integration suite cannot name it; these two seams are therefore
//! rows inside the library, where both of each seam's sides are
//! visible. `sym.rs` holds the enum and the receipt it increments
//! ([`count_decision`](super::count_decision)); `sym/report.rs` holds
//! the report rows and the `record` that maps a discharge onto one,
//! and `record` is `pub(super)`, so this module — a descendant of
//! `sym` — drives the production mapping rather than a copy.
//!
//! The THIRD seam is not here. `k_stats::SampleOutcome` exists only
//! under `probe`, and a `probe`-gated row inside the library is
//! compiled by CI and run by nothing (the probe sweep invokes
//! `--test all`), so that pin is an integration row —
//! `every_discharge_kind_retags_its_sample_with_a_token_of_its_own`,
//! in the `k_stats_doors` suite — reading
//! [`super::discharge_sample_outcomes`].

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::report::{self, ShapeOutcome};
use super::{Discharge, SymBudget, SymCounts, count_decision, with_session};
use crate::predicate::Sign;

/// Every receipt column, with its name.
///
/// A DESTRUCTURE and not a field list: a column added to
/// [`SymCounts`] is a compile error here until this row says which
/// side of the partition below it is on.
fn columns(counts: SymCounts) -> [(&'static str, u64); 7] {
    let SymCounts {
        symbolic_zero,
        sign_gated,
        registered,
        registrations_refused,
        registrations_contradicted,
        numeric,
        frozen,
    } = counts;
    [
        ("symbolic_zero", symbolic_zero),
        ("sign_gated", sign_gated),
        ("registered", registered),
        ("registrations_refused", registrations_refused),
        ("registrations_contradicted", registrations_contradicted),
        ("numeric", numeric),
        ("frozen", frozen),
    ]
}

/// **The receipt columns a [`Discharge`] cannot reach**, named so that
/// the row below says what it excludes rather than passing over it:
/// `numeric` is the column a decision with NO discharge lands in (the
/// last assertion of the row pins that); `registrations_refused` and
/// `registrations_contradicted` count events at the registry door and
/// at a numeric contradiction, neither of which is a decision being
/// discharged; `frozen` counts nodes, not decisions.
const NOT_A_DISCHARGE_KIND: [&str; 4] = [
    "registrations_refused",
    "registrations_contradicted",
    "numeric",
    "frozen",
];

/// A session exists here only to hold a receipt — no form is built and
/// no decision is asked, so the budget is the one that builds nothing.
fn counts_for(discharge: Option<Discharge>) -> SymCounts {
    let (_, counts) = with_session(SymBudget::none(), || count_decision(discharge));
    counts
}

fn moved(counts: SymCounts) -> Vec<(&'static str, u64)> {
    columns(counts)
        .into_iter()
        .filter(|(_, n)| *n != 0)
        .collect()
}

/// **SEAM 1 — `Discharge` against `SymCounts`.** Every discharge kind
/// increments exactly one receipt column, by one, and no two kinds
/// increment the same one; every column that is not named in
/// [`NOT_A_DISCHARGE_KIND`] is reached by exactly one kind.
#[test]
fn every_discharge_kind_increments_one_receipt_column_of_its_own() {
    let mut reached: Vec<(String, &'static str)> = Vec::new();
    for kind in Discharge::all() {
        let hits = moved(counts_for(Some(kind)));
        assert_eq!(
            hits.len(),
            1,
            "one decision discharged as {kind:?} moved {} receipt columns, not one — a kind \
             that increments two columns double-counts the session and one that increments \
             none is a discharge the receipt does not report at all: {hits:?}",
            hits.len()
        );
        assert_eq!(
            hits[0].1, 1,
            "one decision discharged as {kind:?} moved `{}` by {}, not by one",
            hits[0].0, hits[0].1
        );
        reached.push((format!("{kind:?}"), hits[0].0));
    }

    let mut hit: Vec<&str> = reached.iter().map(|(_, column)| *column).collect();
    let before = hit.len();
    hit.sort_unstable();
    hit.dedup();
    assert_eq!(
        hit.len(),
        before,
        "two discharge kinds increment the SAME receipt column, so the receipt still adds up \
         while the population one of its columns names is wrong — which is the whole failure \
         a sixth kind wired into one side of this seam produces: {reached:?}"
    );

    for (column, _) in columns(SymCounts::default()) {
        let kinds = reached.iter().filter(|(_, c)| *c == column).count();
        let excluded = NOT_A_DISCHARGE_KIND.contains(&column);
        assert_eq!(
            kinds == 1,
            !excluded,
            "`{column}` is reached by {kinds} discharge kind(s) and is {} — either a \
             discharge-kind column no `Discharge` reaches, or a column this row excludes \
             that one does: {reached:?}",
            if excluded {
                "named in NOT_A_DISCHARGE_KIND"
            } else {
                "not named in NOT_A_DISCHARGE_KIND"
            }
        );
    }

    let numeric = moved(counts_for(None));
    assert_eq!(
        numeric,
        vec![("numeric", 1)],
        "a decision with no discharge is what `numeric` counts, and nothing else moves with \
         it — the claim that lets this row exclude that column"
    );
}

/// Every report row, once — the roster the seam-2 row reads its
/// excluded rows off. The classifying `match` is exhaustive, so a new
/// [`ShapeOutcome`] is a compile error here until this row says
/// whether a discharge produces it.
fn report_rows() -> [(ShapeOutcome, bool); 9] {
    [
        ShapeOutcome::Theorem,
        ShapeOutcome::SignGated,
        ShapeOutcome::Registered,
        ShapeOutcome::Definite(Sign::Negative),
        ShapeOutcome::Definite(Sign::Zero),
        ShapeOutcome::Definite(Sign::Positive),
        ShapeOutcome::NumericZero,
        ShapeOutcome::Indeterminate,
        ShapeOutcome::Invalid,
    ]
    .map(|row| {
        let from_a_discharge = match row {
            ShapeOutcome::Theorem | ShapeOutcome::SignGated | ShapeOutcome::Registered => true,
            // The rows the numeric channel answers, and the one a
            // domain violation answers: no `Discharge` reaches them,
            // and the row below asserts that none does.
            ShapeOutcome::Definite(_)
            | ShapeOutcome::NumericZero
            | ShapeOutcome::Indeterminate
            | ShapeOutcome::Invalid => false,
        };
        (row, from_a_discharge)
    })
}

/// **SEAM 2 — `Discharge` against `ShapeOutcome`.** Every discharge
/// kind records a report row of its own, and the rows a discharge
/// produces are exactly the three the roster above classifies as
/// such — the report's other rows come from nothing but the numeric
/// channel.
#[test]
fn every_discharge_kind_records_a_report_row_of_its_own() {
    report::start_shape_report();
    let kinds = Discharge::all();
    for kind in kinds {
        // The numeric side of the pair is ignored by every discharge
        // arm of `record`'s match; a definite sign here is the reading
        // that would mask a mis-wired arm rather than agree with it.
        report::record(&Ok(Sign::Positive), Some(kind), None, None);
    }
    let rows: Vec<ShapeOutcome> = report::take_shape_report()
        .into_iter()
        .map(|shape| shape.outcome)
        .collect();

    assert_eq!(
        rows.len(),
        kinds.len(),
        "{} discharge kinds recorded {} report rows: {rows:?}",
        kinds.len(),
        rows.len()
    );
    for (i, row) in rows.iter().enumerate() {
        assert!(
            !rows[..i].contains(row),
            "{:?} and {:?} both record {row:?}, so the shape report cannot tell the two \
             claims apart — a reader counting a document's registered discharges would \
             count them as theorems: {rows:?}",
            kinds[i],
            kinds[rows.iter().position(|r| r == row).unwrap()]
        );
    }
    for (row, from_a_discharge) in report_rows() {
        assert_eq!(
            rows.contains(&row),
            from_a_discharge,
            "{row:?} is {} and was {} by a discharge — a discharge row no kind produces is a \
             row nothing can write, and a numeric row a discharge produces is the tier \
             claiming a decision it did not make: {rows:?}",
            if from_a_discharge {
                "a discharge row"
            } else {
                "not a discharge row"
            },
            if rows.contains(&row) {
                "recorded"
            } else {
                "not recorded"
            }
        );
    }
}
