//! **The contradiction instrument over the document corpus** (ERROR-DESIGN
//! E12, "the two channels are checked where an answer is returned").
//!
//! A definite numeric sign is the answer and the decision asks the tier
//! nothing about it, so whether the tier's algebra or a registration
//! would have claimed `Zero` on a margin the numbers decided is asked
//! here, after each build, by `geom_core::sym::contradiction` over the
//! decisions the re-valuation recorder kept: every corpus document,
//! evaluated with its variables as symbols in an eager session at the
//! drive's budget, rules and retry ladder, at `Sym<Interval>` (where a
//! report is a soundness defect in one of the two channels) and at
//! `Sym<f64>` (where it is a rounded reading the band could not save,
//! or a defect). Each asserts it finds nothing.
//!
//! In the slow set; ci.yml's `sym instrument` row runs it on a pull
//! request that touches the tier (`crates/geom-core/src/sym*`), and the
//! nightly's full suite runs it at every ε row.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{CancelToken, EvalOptions, Evaluation, EvalScalar, ProfileDoc, evaluate};
use geom_core::sym::contradiction::{self, Report};
use geom_core::sym::revalue::record;
use geom_core::{Interval, Sym, SymBudget, SymRules, Tol};

use crate::corpus;

/// `doc` evaluated at `T` with its variables as symbols, inside a session
/// at the drive's dials, and the instrument's report of its decisions.
fn instrumented<T>(doc: &ProfileDoc) -> (usize, Report)
where
    T: EvalScalar,
{
    let opts = EvalOptions {
        symbols: true,
        ..EvalOptions::default()
    };
    let budget = SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let ((decisions, report), _) = geom_core::sym::with_session_retry(
        budget,
        SymRules::shipped(),
        editor_core::drive::DEFAULT_SYM_RETRY,
        || {
            let (_, decisions) = record(|| {
                let ev: Evaluation<T> =
                    evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness());
                ev.order.len()
            });
            let report = contradiction::check(&decisions).expect("inside the session");
            (decisions.len(), report)
        },
    );
    (decisions, report)
}

/// Every corpus document at `T`: what the instrument found, document by
/// document, or a panic naming each document that reported anything.
fn the_corpus_contradicts_nothing_at<T: EvalScalar>(lane: &str) {
    let mut found = Vec::new();
    let mut checked = 0;
    for cd in corpus::documents() {
        let t0 = std::time::Instant::now();
        let (decisions, report) = instrumented::<T>(&cd.doc);
        println!(
            "  [{lane}] {}: {decisions} decisions, {} definite checked, {} absent, {} free, {} \
             found ({:.2} s)",
            cd.name,
            report.checked,
            report.absent,
            report.free,
            report.contradictions.len(),
            t0.elapsed().as_secs_f64()
        );
        checked += report.checked;
        if report.absent != 0 {
            found.push(format!(
                "{}: {} definite decisions on nodes the session does not hold",
                cd.name, report.absent
            ));
        }
        for c in &report.contradictions {
            found.push(format!(
                "{}: decision {} `{}` decided {:?} and the tier proves it zero ({:?}, {:?} rung, \
                 attempt {})",
                cd.name, c.index, c.predicate, c.sign, c.event, c.rung, c.attempt
            ));
        }
    }
    assert!(checked > 0, "[{lane}] the corpus decided nothing definite");
    assert!(
        found.is_empty(),
        "[{lane}] the instrument found the two channels in contradiction:\n{}",
        found.join("\n")
    );
}

/// At the exact witness: a report is a soundness defect.
#[test]
fn the_corpus_contradicts_nothing_at_sym_interval() {
    the_corpus_contradicts_nothing_at::<Sym<Interval>>("Sym<Interval>");
}

/// At the point witness: a report is a rounded reading the band could not
/// save, or a defect; at the corpus's nominals neither is expected.
#[test]
fn the_corpus_contradicts_nothing_at_sym_f64() {
    the_corpus_contradicts_nothing_at::<Sym<f64>>("Sym<f64>");
}
