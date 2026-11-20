//! **Measurement instruments for the chart-coherence resident's
//! default** (`work/tesslie/chart-coherence-ships-off-and-nothing-
//! schedules-turning-it-on.md`). Both rows PRINT and assert nothing
//! about the clock, so both are `#[ignore]`d:
//!
//! ```text
//! cargo nextest run -p editor-core --test all tesslie_coherence_cost \
//!     --run-ignored only --no-capture
//! ```
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::{Duration, Instant};

use crate::corpus;
use editor_core::{
    Advisory, CheckEvidence, CheckId, ChecksConfig, Evaluation, Severity, Subject,
    product_recorded, run_checks, run_checks_on,
};
use geom_core::Tol;

const REPS: usize = 5;

fn median(mut xs: Vec<Duration>) -> f64 {
    xs.sort();
    xs[xs.len() / 2].as_secs_f64() * 1e3
}

fn all_off() -> ChecksConfig {
    ChecksConfig {
        connectedness: Severity::Off,
        chart_coherence: Advisory::Off,
        unproven_coincidence: Advisory::Off,
        at_rest: Severity::Off,
        unused: Advisory::Off,
        ..ChecksConfig::default()
    }
}

/// The resident's cost at the registry split's pinned point (the
/// corpus heat sink at 160 fins), beside the registry's own figure.
#[test]
#[ignore = "measurement; run command in the module docs"]
fn coherence_cost_at_the_pinned_point() {
    let tol = Tol::witness();
    let doc = corpus::heatsink::placed_fins(160);
    let ev: Evaluation<f64> = corpus::eval(&doc);
    let subject = product_recorded(&doc, &ev, tol).expect("the heat sink gathers");
    let stop = editor_core::Stop::default();
    let only = ChecksConfig {
        chart_coherence: Advisory::Warn,
        ..all_off()
    };
    let with = ChecksConfig {
        chart_coherence: Advisory::Warn,
        ..ChecksConfig::default()
    };
    let cfgs = [
        ("default (coherence Off)", ChecksConfig::default()),
        ("default + coherence Warn", with),
        ("all Off", all_off()),
        ("coherence Warn alone", only),
    ];
    for (name, cfg) in &cfgs {
        let mut ts = Vec::new();
        let mut n = 0;
        for _ in 0..REPS {
            let t0 = Instant::now();
            let r = run_checks_on(&doc, &ev, Subject::Product(&subject), cfg, &stop, tol)
                .expect("the registry runs");
            ts.push(t0.elapsed());
            n = r
                .findings
                .iter()
                .filter(|f| f.check == CheckId::ChartCoherence)
                .count();
        }
        println!(
            "PINNED {name}: checks_ms={:.3} coherence_findings={n}",
            median(ts)
        );
    }
}

/// Per corpus document: the resident's findings and unexamined loops at
/// `Warn`, and its time against the all-off registry.
#[test]
#[ignore = "measurement; run command in the module docs"]
fn coherence_census_over_the_corpus() {
    let tol = Tol::witness();
    let stop = editor_core::Stop::default();
    let only = ChecksConfig {
        chart_coherence: Advisory::Warn,
        ..all_off()
    };
    let (mut docs_firing, mut total_f, mut total_u) = (0, 0, 0);
    let docs = corpus::documents();
    let n_docs = docs.len();
    for d in docs {
        let ev: Evaluation<f64> = corpus::eval(&d.doc);
        let mut on = Vec::new();
        let mut off = Vec::new();
        let mut report = None;
        for _ in 0..REPS {
            let t0 = Instant::now();
            let _ = run_checks(&d.doc, &ev, &all_off(), &stop, tol);
            off.push(t0.elapsed());
            let t0 = Instant::now();
            let r = run_checks(&d.doc, &ev, &only, &stop, tol);
            on.push(t0.elapsed());
            report = Some(r);
        }
        let r = match report.unwrap() {
            Ok(r) => r,
            Err(e) => {
                println!("CORPUS {}: registry refused: {e:?}", d.name);
                continue;
            }
        };
        let (mut f, mut u, mut why) = (0, 0, Vec::new());
        for x in &r.findings {
            match &x.evidence {
                CheckEvidence::ChartCoherence { finding } => {
                    f += 1;
                    why.push(format!(
                        "{:?} metres={:e}",
                        finding.condition, finding.metres
                    ));
                }
                CheckEvidence::ChartCoherenceUnexamined { unexamined } => {
                    u += 1;
                    why.push(format!("{:?}", unexamined.why));
                }
                _ => {}
            }
        }
        if f + u > 0 {
            docs_firing += 1;
        }
        total_f += f;
        total_u += u;
        println!(
            "CORPUS {}: findings={f} unexamined={u} on_ms={:.3} off_ms={:.3} placements={}",
            d.name,
            median(on),
            median(off),
            d.doc.placements().len()
        );
        for w in why {
            println!("    {w}");
        }
    }
    println!(
        "CORPUS TOTAL: docs={n_docs} firing={docs_firing} findings={total_f} unexamined={total_u}"
    );
}
