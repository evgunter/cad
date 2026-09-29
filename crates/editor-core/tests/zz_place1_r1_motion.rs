//! place1-r1 probe: evaluate every corpus document at f64 and at
//! Interval so the PROBE_MOTION hook in wire_transform prints every
//! transform's motion bits. Diffed base vs head.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::corpus;
use geom_core::Interval;

#[test]
fn place1_r1_print_every_corpus_motion() {
    for doc in corpus::documents() {
        eprintln!("PROBE_DOC {}", doc.name);
        let _ = corpus::eval::<f64>(&doc.doc);
        let _ = corpus::eval::<Interval>(&doc.doc);
    }
}
