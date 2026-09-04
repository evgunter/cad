//! DOCM-4 review lane R1 — `reused` on the bench corpus, per document,
//! printed so the same file runs at the merge base and at the head.
//! Standalone (not mounted in `all.rs`) so it compiles at both.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code, unused_imports)]

#[path = "fixture/mod.rs"]
mod fixture;
#[path = "corpus/mod.rs"]
mod corpus;

use editor_core::{CancelToken, EvalOptions, evaluate};
use geom_core::Tol;

#[test]
fn r1_reused_dump() {
    let mut total_bump = 0;
    let mut total_same = 0;
    for d in corpus::documents() {
        let full = corpus::eval::<f64>(&d.doc);
        let same = evaluate::<f64>(
            &d.doc,
            Some(&full),
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let bumped = d.bumped();
        let after = evaluate::<f64>(
            &bumped,
            Some(&full),
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        println!(
            "REUSED {} same={} bump={} recomputed={} len={}",
            d.name,
            same.reused,
            after.reused,
            after.recomputed,
            d.len()
        );
        total_bump += after.reused;
        total_same += same.reused;
    }
    println!("TOTAL same={total_same} bump={total_bump}");
}
