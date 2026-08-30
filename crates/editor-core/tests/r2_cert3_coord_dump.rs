//! CERT-3 review lane R2 — an independent re-derivation of the m10-p
//! fence's coordinate-dump differential.
//!
//! The fence header records the procedure as "the same corpus walk,
//! dumping every coordinate rather than digesting it, on this tree and
//! on a tree with only those two files reverted". This file is that
//! walk, written from the fence's own `corpus_digest` shape but
//! emitting a line per observable instead of folding into FNV. Run it
//! on the reviewed head and again with `affine.rs`/`mat.rs` reverted,
//! then diff the two outputs.
//!
//! Not a unit deliverable; a reviewer's instrument.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{CancelToken, EvalOptions, NodeResult, ValuePayload, evaluate};
use geom_core::Tol;

/// Dump every observable the fence digests, in the same order.
fn dump<T: editor_core::EvalScalar>(tag: &str, bits: impl Fn(&geom_core::Point3<T>) -> String) {
    let mut coords = 0usize;
    let mut nodes = 0usize;
    for doc in corpus::documents() {
        let ev = evaluate::<T>(
            &doc.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        for (id, result) in ev.nodes.iter() {
            nodes += 1;
            match result {
                NodeResult::Poisoned { through } => {
                    println!("{tag}\t{}\t{}\tPOISONED\t{}", doc.name, id.0, through.0);
                }
                NodeResult::Failed(_) => {
                    println!("{tag}\t{}\t{}\tFAILED", doc.name, id.0);
                }
                NodeResult::Ok(v) => {
                    println!(
                        "{tag}\t{}\t{}\tOK\t{}",
                        doc.name,
                        id.0,
                        v.payload.kind_name()
                    );
                    if let ValuePayload::Body(b) = &v.payload {
                        for (pid, p) in b.points() {
                            coords += 3;
                            println!("{tag}\t{}\t{}\tPT\t{pid:?}\t{}", doc.name, id.0, bits(p));
                        }
                    }
                }
            }
        }
    }
    println!("{tag}\tSUMMARY\tnodes={nodes}\tcoords={coords}");
}

#[test]
fn r2_dump_corpus_coordinates_f64() {
    dump::<f64>("f64", |p| {
        format!(
            "{:016x} {:016x} {:016x} | {:?} {:?} {:?}",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits(),
            p.x,
            p.y,
            p.z
        )
    });
}
