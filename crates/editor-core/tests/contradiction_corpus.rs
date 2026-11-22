//! **The contradiction instrument over the documents the tier is measured
//! on** (ERROR-DESIGN E12, "the two channels are checked where an answer
//! is returned").
//!
//! A definite numeric sign is the answer and the decision asks the tier
//! nothing about it, so whether the tier's algebra or a registration
//! would have claimed `Zero` on a margin the numbers decided is asked
//! here, after each build, by `geom_core::sym::contradiction` over the
//! decisions the re-valuation recorder kept. The documents are the five
//! `m10_9_pins_interval::measured_studies` (the plate, the annulus, R2's
//! link, bracket and pad), each built in an eager session at the drive's
//! budget, rules and retry ladder at `Sym<Interval>` over its analyzed
//! box at the scale it certifies at, where a report is a soundness defect
//! in one of the two channels (past each ceiling is
//! `sym11_exact_channel_rows`). It asserts it finds nothing.
//!
//! **Why these five, and not the whole corpus or `Sym<f64>`**
//! (measured): the instrument asks the whole ladder of every definite
//! margin, which on the corpus's larger documents is work the decision
//! path never does. `corner_table` at `Sym<f64>` is 30,375 definite
//! decisions, 212 s with no retry ladder and past 11 GB with the drive's;
//! `die` (367,824 decisions) passes 11 GB; the pad at `Sym<f64>` with its
//! variables as symbols passes 12 GB. The rounded event has its planted
//! rows in `geom-core` and `sweep`.
//!
//! In the slow set; ci.yml's `sym instrument` row runs it on a pull
//! request that touches the tier (`crates/geom-core/src/sym*`), and the
//! nightly's full suite runs it at every ε row.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{CancelToken, EvalOptions, Evaluation, ProfileDoc, evaluate};
use geom_core::sym::contradiction::{self, Report};
use geom_core::sym::revalue::record;
use geom_core::{Interval, Sym, SymBudget, SymRules, Tol};

use crate::m10_9_pins_interval::measured_studies;

/// One build of a document at a lane's scalar.
type Build = fn(&ProfileDoc, &EvalOptions, Tol);

fn at_sym_interval(doc: &ProfileDoc, opts: &EvalOptions, tol: Tol) {
    let _: Evaluation<Sym<Interval>> = evaluate(doc, None, &CancelToken::new(), opts, tol);
}

/// `doc` built by `build` under `opts`, inside an eager session at the
/// drive's dials, and the instrument's report of its decisions.
fn instrumented(doc: &ProfileDoc, opts: &EvalOptions, build: Build, tol: Tol) -> (usize, Report) {
    let budget = SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let (out, _) = geom_core::sym::with_session_retry(
        budget,
        SymRules::shipped(),
        editor_core::drive::DEFAULT_SYM_RETRY,
        || {
            let ((), decisions) = record(|| build(doc, opts, tol));
            let report = contradiction::check(&decisions).expect("inside the session");
            (decisions.len(), report)
        },
    );
    out
}

/// Every measured document through `build` under the options `opts_of`
/// gives it: what the instrument found, or a panic naming each document
/// that reported anything.
fn the_measured_documents_contradict_nothing(
    lane: &str,
    build: Build,
    opts_of: impl Fn(&ProfileDoc) -> EvalOptions,
) {
    let tol = Tol::witness();
    let mut found = Vec::new();
    let mut checked = 0;
    for study in measured_studies(tol) {
        let doc = (study.at)(study.certifies_at * tol.eps());
        let t0 = std::time::Instant::now();
        let (decisions, report) = instrumented(&doc, &opts_of(&doc), build, tol);
        println!(
            "  [{lane}] {}: {decisions} decisions, {} definite checked, {} absent, {} free, {} \
             found ({:.2} s)",
            study.name,
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
                study.name, report.absent
            ));
        }
        for c in &report.contradictions {
            found.push(format!(
                "{}: decision {} `{}` decided {:?} and the tier proves it zero ({:?}, {:?} rung, \
                 attempt {})",
                study.name, c.index, c.predicate, c.sign, c.event, c.rung, c.attempt
            ));
        }
    }
    assert!(
        checked > 0,
        "[{lane}] no document decided anything definite"
    );
    assert!(
        found.is_empty(),
        "[{lane}] the instrument found the two channels in contradiction:\n{}",
        found.join("\n")
    );
}

/// At the exact witness, over each document's analyzed box: a report is
/// a soundness defect.
#[test]
fn the_measured_documents_contradict_nothing_at_sym_interval() {
    the_measured_documents_contradict_nothing("Sym<Interval>", at_sym_interval, |doc| {
        EvalOptions {
            param_box: Some(Arc::new(ParamBox::of(&analyzed_box(
                doc,
                &AnalysisPolicy::default(),
            )))),
            ..EvalOptions::default()
        }
    });
}
