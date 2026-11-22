//! Reviewer probes for the decide-path unit over documents: the
//! contradiction instrument over corpus documents the PR's lane leaves
//! out, by name (`R1_DOCS`, comma-separated, or `all`), lane (`R1_LANE`
//! = `interval` | `f64`) and ladder (`R1_RETRY` = `1` for the drive's).
//! It asserts nothing: run with `--ignored --nocapture` and read it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{CancelToken, EvalOptions, Evaluation, ProfileDoc, evaluate};
use geom_core::sym::contradiction;
use geom_core::sym::revalue::record;
use geom_core::{Interval, Sym, SymBudget, SymRetry, SymRules, Tol};

use crate::corpus;

fn build(doc: &ProfileDoc, lane: &str) {
    let opts = EvalOptions::default();
    let tol = Tol::witness();
    if lane == "f64" {
        let _: Evaluation<Sym<f64>> = evaluate(doc, None, &CancelToken::new(), &opts, tol);
    } else {
        let _: Evaluation<Sym<Interval>> = evaluate(doc, None, &CancelToken::new(), &opts, tol);
    }
}

#[test]
#[ignore = "a measurement: R1_DOCS=<names|all> R1_LANE=<interval|f64> R1_RETRY=<0|1>"]
fn r1_the_instrument_over_named_corpus_documents() {
    let docs = std::env::var("R1_DOCS").unwrap_or_default();
    let lane = std::env::var("R1_LANE").unwrap_or_else(|_| "interval".to_owned());
    let retry = if std::env::var("R1_RETRY").as_deref() == Ok("1") {
        editor_core::drive::DEFAULT_SYM_RETRY
    } else {
        SymRetry::none()
    };
    let budget = SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    for cd in corpus::documents() {
        if docs != "all" && !docs.split(',').any(|n| n == cd.name) {
            continue;
        }
        let t0 = std::time::Instant::now();
        let ((n, built, report), counts) =
            geom_core::sym::with_session_retry(budget, SymRules::shipped(), retry, || {
                let ((), decisions) = record(|| build(&cd.doc, &lane));
                let built = t0.elapsed().as_secs_f64();
                (decisions.len(), built, contradiction::check(&decisions))
            });
        let report = report.expect("in session");
        println!(
            "r1 [{lane}] {}: {n} decisions, built {built:.2} s, checked {}, absent {}, free {}, \
             found {}, total {:.2} s, {counts:?}",
            cd.name,
            report.checked,
            report.absent,
            report.free,
            report.contradictions.len(),
            t0.elapsed().as_secs_f64()
        );
        for c in report.contradictions.iter().take(5) {
            println!(
                "   `{}` {:?} {:?} {:?} attempt {}",
                c.predicate, c.sign, c.event, c.rung, c.attempt
            );
        }
    }
}
