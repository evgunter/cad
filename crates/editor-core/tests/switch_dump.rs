//! **LIB-SWITCH §8 acceptance instrument: the merge-base payload
//! diff.** Dumps every corpus document's evaluated GEOMETRY payloads
//! (bodies, validated profiles, datums — Debug is shortest-round-trip,
//! bit-faithful) to `target/switch-dump/<doc>.txt`. Run here and in a
//! scratch worktree at the merge-base (with the pre-switch payload
//! spelling), then diff: byte-identical files per doc EXCEPT the §5-1
//! re-authored documents is the acceptance bar. Ignored by default —
//! run with `--ignored` (it spends a full corpus evaluation).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{CancelToken, EvalOptions, NodeResult, ValuePayload, evaluate};

#[test]
#[ignore = "the merge-base diff instrument; run explicitly"]
fn dump_corpus_payloads() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/switch-dump");
    std::fs::create_dir_all(&dir).unwrap();
    for d in corpus::documents() {
        let ev = evaluate::<f64>(&d.doc, None, &CancelToken::new(), &EvalOptions::default());
        let mut out = String::new();
        for id in &ev.order {
            out.push_str(&format!("== node {id:?}\n"));
            match ev.nodes.get(id) {
                Some(NodeResult::Ok(v)) => match &v.payload {
                    ValuePayload::Profile(p) => {
                        // Geometry only: the validated profile (the
                        // naming anchor is new machinery, diffed via
                        // the name-table row below instead).
                        out.push_str(&format!("{:?}\n", p.validated));
                    }
                    other => out.push_str(&format!("{other:?}\n")),
                },
                Some(NodeResult::Failed(e)) => out.push_str(&format!("FAILED {e:?}\n")),
                Some(NodeResult::Poisoned { through }) => {
                    out.push_str(&format!("POISONED {through:?}\n"));
                }
                None => out.push_str("MISSING\n"),
            }
        }
        std::fs::write(dir.join(format!("{}.txt", d.name)), out).unwrap();
    }
}
