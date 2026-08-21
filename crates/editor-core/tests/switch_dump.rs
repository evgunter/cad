//! **LIB-SWITCH §8 acceptance instrument: the merge-base payload
//! diff.** Dumps every corpus document's evaluated GEOMETRY payloads
//! (bodies, validated profiles, datums — Debug is shortest-round-trip,
//! bit-faithful) to `target/switch-dump/<doc>.txt`, and every node's
//! NAME TABLE (names only, in table order) to
//! `target/switch-dump/<doc>.names.txt` — separate files so geometry
//! identity and naming identity diff independently: program-anchored
//! naming may legitimately RENAME (a reversed or rotated loop's
//! program indices) where geometry is bit-identical, and a rename
//! must be a stated finding, never hidden (PR #291 review MINOR-1).
//! Run here and in a scratch worktree at the merge-base (with the
//! pre-switch payload spelling), then diff: geometry byte-identical
//! per doc EXCEPT the §5-1 re-authored documents is the acceptance
//! bar. Ignored by default — run with `--ignored`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{CancelToken, EvalOptions, NodeResult, ValuePayload, evaluate};
use geom_core::Tol;

#[test]
#[ignore = "the merge-base diff instrument; run explicitly"]
fn dump_corpus_payloads() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/switch-dump");
    std::fs::create_dir_all(&dir).unwrap();
    for d in corpus::documents() {
        let ev = evaluate::<f64>(&d.doc, None, &CancelToken::new(), &EvalOptions::default(), Tol::witness());
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
        let mut names = String::new();
        for id in &ev.order {
            names.push_str(&format!("== node {id:?}\n"));
            if let Some(NodeResult::Ok(v)) = ev.nodes.get(id) {
                for (n, e) in v.name_table.iter() {
                    // Name AND binding: arena keys are D9-deterministic
                    // for identical geometry, so a name-to-entity SWAP
                    // (the rename class) shows even when the name SET
                    // is unchanged.
                    names.push_str(&format!("{n:?} -> {e:?}\n"));
                }
            }
        }
        std::fs::write(dir.join(format!("{}.names.txt", d.name)), names).unwrap();
    }
}
