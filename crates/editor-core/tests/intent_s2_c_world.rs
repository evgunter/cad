//! INTENT stage 2 PR C: the product is the world
//! (`docs/INTENT-STAGE2-SPEC.md` §4, §9 rows 6–12).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::value_channel::body_digest;
use editor_core::{ProfileDoc, product};
use geom_core::Tol;

/// The committed files test 6 regenerates, relative to the repo root.
const FILES: [&str; 5] = [
    "crates/editor-core/tests/golden/golden.cad",
    "crates/editor-core/tests/corpus/die_tool.pncad",
    "crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
    "crates/viewer/tests/gallery_ring.pncad",
    "crates/pncad/tests/plate_param.pncad",
];

/// Every document test 6 compares: the corpus, then the files.
fn documents() -> Vec<(String, ProfileDoc)> {
    let mut out: Vec<(String, ProfileDoc)> = corpus::documents()
        .into_iter()
        .map(|d| (d.name.to_string(), d.doc))
        .collect();
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for file in FILES {
        let text = std::fs::read_to_string(here.join(file)).expect("reads");
        let doc = editor_core::persist::load(&text, Tol::witness())
            .unwrap_or_else(|e| panic!("{file} loads: {e}"))
            .doc;
        out.push((file.to_string(), doc));
    }
    out
}

/// One document's product, as a digest word and the count of words
/// fed, or the refusal's words.
fn product_row(doc: &ProfileDoc) -> String {
    let ev = corpus::eval::<f64>(doc);
    match product(doc, &ev, Tol::witness()) {
        Ok(body) => {
            let (word, fed) = body_digest(&body);
            format!("{word:016x}/{fed}")
        }
        Err(e) => format!("refused {:?}", e.kind()),
    }
}

/// Prints every document's product row: the record test 6 compares
/// against was taken by this at the base.
#[test]
#[ignore = "a probe: prints the product rows"]
fn print_product_rows() {
    for (name, doc) in documents() {
        println!("(\"{name}\", \"{}\"),", product_row(&doc));
    }
}

#[test]
#[ignore = "a probe"]
fn print_roots() {
    for (name, doc) in documents() {
        let ids = doc.ids();
        let rows: Vec<String> = doc
            .roots()
            .iter()
            .map(|r| {
                let at = ids.iter().position(|i| i == r).unwrap();
                format!("{}#{at}", corpus::node_kind(doc.node(*r).unwrap()))
            })
            .collect();
        println!("{name}: {}", rows.join(" "));
    }
}
