//! origin-graft-r1 review probe (PR 3413): dump every corpus document's
//! name tables to `$GRAFT_R1_DUMP` for a cross-tree diff.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use std::fmt::Write as _;

#[test]
fn graft_r1_dump_corpus_name_tables() {
    let Ok(path) = std::env::var("GRAFT_R1_DUMP") else { return };
    let mut out = String::new();
    for d in corpus::documents() {
        let ev = corpus::eval::<f64>(&d.doc);
        for id in &ev.order {
            if let Some(v) = ev.value(*id) {
                for (n, e) in v.name_table.iter() {
                    writeln!(out, "{}\t{id:?}\t{n:?}\t{e:?}", d.name).unwrap();
                }
            }
        }
    }
    std::fs::write(path, out).unwrap();
}
