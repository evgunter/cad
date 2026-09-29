//! place1-r2 review probe: prints every corpus transform's MOTION (via
//! the PLACE1_R2_MOTION instrumentation) at f64 and in the interval lane,
//! so base and head can be diffed by bits.

use crate::corpus;
use editor_core::Node;
use geom_core::Interval;

#[test]
fn place1_r2_print_every_corpus_motion() {
    for doc in corpus::documents() {
        let has = doc
            .doc
            .order()
            .iter()
            .any(|id| matches!(doc.doc.node(*id), Some(Node::Transform { .. })));
        if !has {
            continue;
        }
        eprintln!("DOC {} f64", doc.name);
        let ev = corpus::eval::<f64>(&doc.doc);
        assert!(corpus::failures(&ev).is_empty());
        eprintln!("DOC {} interval", doc.name);
        let ev = corpus::eval::<Interval>(&doc.doc);
        let bad = corpus::failures(&ev);
        eprintln!("DOC {} interval failures {}", doc.name, bad.len());
    }
}
