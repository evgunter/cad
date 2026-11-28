//! Review probe (PR 7710): a differential over the corpus's coincidence
//! and Boolean rows. Writes every document's per-node outcome, every
//! row's relation, site, margin and discharge, and the door's proof of
//! each, at `f64`, and the per-node outcome and rows at `Interval`, to
//! the file `REVIEW_DIFF_OUT` names. Run on two trees and diff.
use super::*;
use std::fmt::Write as _;

fn docs() -> Vec<crate::corpus::CorpusDoc> {
    let mut all = crate::corpus::documents();
    all.extend(crate::corpus::stage6_beside_the_registry());
    all.push(crate::corpus::blocks_on_one_floor::document());
    all.push(crate::corpus::blocks_on_one_floor::members_exchanged());
    all
}

fn one(cd: &crate::corpus::CorpusDoc) -> String {
    let mut out = String::new();
    let ev = run(&cd.doc);
    for f in crate::corpus::failures(&ev) {
        writeln!(out, "  f64 failure {f}").unwrap();
    }
    for (node, rows, proofs) in proved_rows(&cd.doc, &ev) {
        for (row, proof) in rows.iter().zip(&proofs) {
            writeln!(
                out,
                "  f64 {node:?} {:?} {:?} {:?} {:?} margin={:?} proof={}",
                row.origin,
                row.relation,
                row.site,
                row.discharge,
                row.margin,
                proof_words(proof)
            )
            .unwrap();
        }
    }
    let iv = crate::corpus::eval::<geom_core::Interval>(&cd.doc);
    for f in crate::corpus::failures(&iv) {
        writeln!(out, "  iv failure {f}").unwrap();
    }
    for &node in &iv.order {
        if let Some(v) = iv.value(node) {
            for row in v.coincidences.iter() {
                writeln!(
                    out,
                    "  iv {node:?} {:?} {:?} {:?} {:?} margin={:?}",
                    row.origin, row.relation, row.site, row.discharge, row.margin
                )
                .unwrap();
            }
        }
    }
    out
}

#[test]
#[ignore = "review probe: run by name"]
fn review_one_plane_corpus_differential() {
    let path = std::env::var("REVIEW_DIFF_OUT").expect("REVIEW_DIFF_OUT");
    let mut out = String::new();
    for cd in docs() {
        let name = cd.name;
        let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| one(&cd)));
        match got {
            Ok(text) => writeln!(out, "== {name}\n{text}").unwrap(),
            Err(_) => writeln!(out, "== {name}\n  PANICKED").unwrap(),
        }
    }
    std::fs::write(path, out).unwrap();
}
