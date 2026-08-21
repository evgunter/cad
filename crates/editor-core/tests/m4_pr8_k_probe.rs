//! M4 PR 8b spec D3 — the K-telemetry Probe run over the Band 4
//! corpus (the harness the K-REPORT M3 addendum recorded as missing).
//!
//! Mechanics are the M2 report's, unchanged (docs/K-REPORT.md
//! "Collection method for the future run"): every corpus document is
//! evaluated end-to-end at the recording scalar `Probe` — decisions
//! bit-identical to f64, every `k_stats::decide` classification
//! recorded — then the result body runs the same tier-1/closed
//! validation and mass-properties pass the corpus rows run, and every
//! `MarginSample` dumps as CSV. One process per ε (`Tolerance` is a
//! OnceLock):
//!
//! ```sh
//! CAD_TOLERANCE_EPS=1e-9 CAD_K_REPORT_OUT=/tmp/corpus-eps-1e-9.csv \
//!   cargo test -p editor-core --test m4_pr8_k_probe -- --ignored --nocapture
//! ```
//!
//! Without `CAD_K_REPORT_OUT` the CSV goes to stdout. Columns are the
//! M2 file convention: `shape,predicate,margin,band_zero,
//! band_escalate,outcome`, with shapes namespaced `corpus/<doc>` so a
//! merged corpus+demos CSV stays attributable (the demo sweep — the
//! tour binary's `k-probe` mode — namespaces `demo/<scene>`).
//! `scripts/k_probe_sweep.sh` runs both and merges; the committed
//! baseline lives in `docs/k-report-data/m7-eps-<ε>.csv.gz`
//! (the m4-/m5- rows stay committed as the historical record).
//!
//! Both tests here are Probe-lane by construction (the dump run and the
//! standing bit-identity pin), so the whole file is gated on the `probe`
//! feature — nothing non-Probe is lost from the default build.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use std::io::Write as _;

use geom_core::k_stats::{self, MarginSample, Probe, SampleOutcome};
use geom_core::{Sign, Tolerance};
use topo::{mass_properties, validate, validate_closed};

use corpus::{body_of, documents, eval, failures};

fn outcome_str(o: SampleOutcome) -> &'static str {
    match o {
        SampleOutcome::Definite(Sign::Negative) => "negative",
        SampleOutcome::Definite(Sign::Zero) => "zero",
        SampleOutcome::Definite(Sign::Positive) => "positive",
        SampleOutcome::Indeterminate => "indeterminate",
        SampleOutcome::Invalid => "invalid",
    }
}

/// One document's Probe sweep: evaluation (sequential — the sample
/// sink is thread-local) plus the corpus rows' body pass.
fn run_doc(d: &corpus::CorpusDoc) -> Vec<MarginSample> {
    k_stats::start_recording();
    let ev = eval::<Probe>(&d.doc);
    let bad = failures(&ev);
    assert!(
        bad.is_empty(),
        "{}: corpus document must evaluate green at Probe. Nothing here \
         compares Probe against f64, so a red is either a Probe-lane \
         divergence or an f64-lane break — the f64 corpus rows say \
         which:\n{}",
        d.name,
        bad.join("\n")
    );
    if let Some(result) = d.result {
        let body = body_of(&ev, result);
        validate(body).expect("tier 1");
        validate_closed(body).expect("closed");
        mass_properties(body).expect("mass properties");
    }
    k_stats::take_samples()
}

/// The dump entry point (ignored: run explicitly, one process per ε).
#[test]
#[ignore = "K-telemetry collection run; one process per eps (see module docs)"]
fn dump_corpus_k_samples() {
    let eps = Tolerance::get().eps;
    let mut csv = String::from("shape,predicate,margin,band_zero,band_escalate,outcome\n");
    let mut total = 0usize;
    let mut unnamed = 0usize;
    for d in documents() {
        let samples = run_doc(&d);
        total += samples.len();
        for s in &samples {
            if s.predicate == "<unnamed>" {
                unnamed += 1;
            }
            csv.push_str(&format!(
                "corpus/{},{},{:e},{:e},{:e},{}\n",
                d.name,
                s.predicate,
                s.margin,
                s.band_zero,
                s.band_escalate,
                outcome_str(s.outcome)
            ));
        }
    }
    assert_eq!(
        unnamed, 0,
        "<unnamed> must be unreachable from shipped decide paths"
    );
    eprintln!("k_probe(corpus): eps={eps:e}, {total} samples");
    match std::env::var("CAD_K_REPORT_OUT") {
        Ok(path) => {
            let mut f = std::fs::File::create(&path).expect("create CAD_K_REPORT_OUT");
            f.write_all(csv.as_bytes()).expect("write csv");
            eprintln!("k_probe(corpus): wrote {path}");
        }
        Err(_) => print!("{csv}"),
    }
}

/// The whole corpus evaluates green at `Probe` — one-sided, and that is
/// the whole of the assertion.
///
/// The property this reaches for is that the recording scalar is a
/// WRAPPER and not a second arithmetic, i.e. that its decisions are
/// bit-identical to f64's. **Nothing here compares the two**, and no
/// test in this tree does; greenness at `Probe` is evidence for that
/// claim, not a check of it. Greenness is also tolerance-dependent, so
/// this says what it says at one ε.
///
/// Runs under the DEFAULT selection, which `scripts/k_probe_sweep.sh`
/// invokes once per rostered suite. `run_doc` inside the `#[ignore]`d
/// dump beside it asserts the same predicate over the same documents at
/// all three ε, so what this row adds is that its body executes at all
/// — not the property.
#[test]
fn corpus_evaluates_green_at_probe() {
    for d in documents() {
        let ev = eval::<Probe>(&d.doc);
        let bad = failures(&ev);
        assert!(
            bad.is_empty(),
            "{}: not green at Probe:\n{}",
            d.name,
            bad.join("\n")
        );
    }
}
