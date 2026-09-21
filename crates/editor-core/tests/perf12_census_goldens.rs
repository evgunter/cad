//! **The at-rest census verdict golden**: every registered corpus
//! document and the heat sink driven to 10/40/160 fins, each gathered
//! into its product and put through the tier-3′ door
//! (`AtRestPolicy::gate_at_rest_declared`), with the WHOLE error vector
//! — kinds, entities, order, witnesses — serialized and compared
//! byte-exact against a committed golden per ε row.
//!
//! What it pins is the census's verdict as a function of the body: a
//! change to WHICH pairs the sweeps examine (a candidate pre-filter)
//! must leave every row of this file byte-identical, because the
//! verdict is a function of exact tests only and the filter may drop
//! nothing the exact sweep decides. The K-funnel's verdict LOG is
//! deliberately not in the golden: it records which predicates ran, and
//! a pre-filter exists to run fewer (`m5_pr8_bvh_diff` scrubs the same
//! field for the same reason).
//!
//! # Re-blessing
//!
//! Run with `PERF12_BLESS_CENSUS=1` at each ε row
//! (`CAD_TOLERANCE_EPS=1e-6`, default, `1e-12`), inspect the diff, and
//! commit it WITH the change it records — a moved row is the census
//! deciding differently, which the PR body must state.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use corpus::{documents, eval, failures};
use editor_core::{DocEdit, DocParam, ParamName, ProfileDoc, apply, assemble, product_recorded};
use geom_core::Tol;
use topo::AtRestPolicy;

/// The committed goldens, one per ε row `ci-filter.py` draws — keyed by
/// `format!("{:e}", eps)`.
const CENSUS_GOLDENS: &[(&str, &str, &str)] = &[
    (
        "1e-6",
        include_str!("golden/perf12_census_1e-6.txt"),
        "tests/golden/perf12_census_1e-6.txt",
    ),
    (
        "1e-9",
        include_str!("golden/perf12_census_1e-9.txt"),
        "tests/golden/perf12_census_1e-9.txt",
    ),
    (
        "1e-12",
        include_str!("golden/perf12_census_1e-12.txt"),
        "tests/golden/perf12_census_1e-12.txt",
    ),
];

/// The fin counts the heat sink is driven to — the finding's three
/// measured sizes (`work/perf/assemble-aggregate-census-is-quadratic-in-solids.md`).
const FINS: [i64; 3] = [10, 40, 160];

/// The corpus heat sink with its fin count driven to `fins`.
fn heatsink_at(fins: i64) -> ProfileDoc {
    let entry = documents()
        .into_iter()
        .find(|d| d.name == "heat_sink")
        .expect("the corpus carries the heat sink");
    apply(
        &entry.doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("fins"),
            value: DocParam::Count { value: fins },
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the fin count is a document parameter")
    .doc
}

/// One document's row: the product's size, `assemble`'s outcome, and
/// the census's error vector one finding per line.
fn row(label: &str, doc: &ProfileDoc, out: &mut String) {
    let tol = Tol::witness();
    let ev = eval::<f64>(doc);
    let bad = failures(&ev);
    assert!(
        bad.is_empty(),
        "{label}: evaluation failed:\n{}",
        bad.join("\n")
    );
    out.push_str(&format!("## {label}\n"));
    let subject = match product_recorded(doc, &ev, tol) {
        Ok(p) => p,
        Err(e) => {
            out.push_str(&format!("gather: Err({e:?})\n"));
            return;
        }
    };
    out.push_str(&format!(
        "solids={} faces={} edges={} vertices={}\n",
        subject.body.solids().count(),
        subject.body.faces().count(),
        subject.body.edges().count(),
        subject.body.vertices().count(),
    ));
    // `assemble`'s verdict, by variant: the census's findings ride the
    // error below, so only the SHAPE of the outcome is recorded here.
    match assemble(doc, &ev, tol) {
        Ok(_) => out.push_str("assemble: Ok\n"),
        Err(e) => {
            let s = format!("{e:?}");
            let variant = s
                .split(|c: char| !c.is_alphanumeric())
                .next()
                .unwrap_or("?");
            out.push_str(&format!("assemble: Err({variant})\n"));
        }
    }
    match <f64 as AtRestPolicy>::gate_at_rest_declared(&subject.body, &subject.contacts, tol) {
        Ok(_) => out.push_str("census: Ok\n"),
        Err(errors) => {
            out.push_str(&format!("census: {} findings\n", errors.len()));
            for e in &errors {
                out.push_str(&format!("{e:?}\n"));
            }
        }
    }
}

/// Every row, in a fixed order: the registry's order, then the fin
/// sweep ascending.
fn census_text() -> String {
    let mut out = String::new();
    for d in documents() {
        row(d.name, &d.doc, &mut out);
    }
    for fins in FINS {
        let doc = heatsink_at(fins);
        row(&format!("heat_sink@{fins}"), &doc, &mut out);
    }
    out
}

#[test]
fn the_at_rest_census_verdicts_are_goldened_bit_exact() {
    let eps = format!("{:e}", Tol::witness().eps());
    let Some(&(_, golden, path)) = CENSUS_GOLDENS.iter().find(|(row, _, _)| *row == eps) else {
        // An unblessed ε FAILS (the `m10_6` posture): a row that
        // compares nothing and passes is indistinguishable from one
        // that checked something.
        panic!(
            "no committed census golden for eps={eps}; the blessed rows are {}. Bless it with \
             PERF12_BLESS_CENSUS=1 and commit the file WITH the change it records.",
            CENSUS_GOLDENS
                .iter()
                .map(|(r, _, _)| *r)
                .collect::<Vec<_>>()
                .join(", ")
        );
    };
    let text = census_text();
    if std::env::var("PERF12_BLESS_CENSUS").is_ok() {
        std::fs::write(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path),
            &text,
        )
        .expect("bless writes");
        panic!(
            "census golden for eps={eps} re-blessed — commit the file WITH the change it \
             records, then rerun without the env var"
        );
    }
    if text != golden {
        // Name the first row that moved rather than dumping two files.
        let first = text
            .lines()
            .zip(golden.lines())
            .position(|(a, b)| a != b)
            .map_or_else(
                || "a length change".to_string(),
                |i| format!("line {}", i + 1),
            );
        panic!(
            "the at-rest census verdicts drifted from their committed golden at {first} \
             (eps={eps}): the census is deciding differently. Read the diff, decide whether \
             the new verdict is right, and re-bless deliberately (PERF12_BLESS_CENSUS=1)."
        );
    }
}
