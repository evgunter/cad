//! M4 PR 6 spec D6.1 — the save/load/replay-identity CI row.
//!
//! The fixtures ARE the M4 PR 8a Band 4 corpus (`corpus/mod.rs`). Every
//! document round-trips BIT-identically (the document, canonical bytes)
//! AND evaluates bit-identically once loaded — name tables, bodies
//! (arena floats via Debug's shortest-round-trip encoding, which is
//! bit-faithful for the finite values documents carry), verdict logs,
//! content keys. The CI matrix runs this file at ε ∈ {1e-6, 1e-9,
//! 1e-12}.
//!
//! Every evaluated fixture must also be GREEN (#117): fingerprint
//! identity alone is blind to evaluation health, so a sick fixture
//! would otherwise round-trip perfectly while proving almost nothing.
//!
//! D2 lives in `m4_pr6_floats.rs` (the bit-level float property row).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use editor_core::{CancelToken, EvalOptions, ProfileDoc, evaluate, load, save};

use corpus::documents;
use geom_core::Tol;

/// The whole-evaluation bit fingerprint: `Debug` of every node result
/// (payload arenas, name tables, verdicts, content keys) plus the
/// appearance resolution. `Debug` prints floats shortest-round-trip
/// (bit-faithful for finite values, sign of zero included).
fn eval_fingerprint(label: &str, doc: &ProfileDoc) -> String {
    let ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    // #117: fingerprint identity is BLIND to evaluation health — a sick
    // document (Failed/Poisoned nodes) round-trips just as bit-perfectly
    // as a green one, which is exactly how PR 6's legacy kitchen-sink
    // fixture carried 2 Failed + 4 Poisoned of 20 nodes through two
    // passing persistence rows. Assert green here so any future sick
    // fixture fails loudly instead of proving less than the row appears
    // to.
    let bad = corpus::failures(&ev);
    assert!(
        bad.is_empty(),
        "{label}: persistence fixture did not evaluate green (#117 — \
         fingerprint comparison cannot see evaluation health):\n{}",
        bad.join("\n")
    );
    format!("{:?}|{:?}|{:?}", ev.order, ev.nodes, ev.appearance)
}

#[test]
fn save_load_replay_identity() {
    for d in documents() {
        let text = save(&d.doc, Tol::witness()).expect("save");
        let loaded = load(&text, Tol::witness())
            .unwrap_or_else(|e| panic!("{}: load refused: {e:?}", d.name));
        assert!(
            loaded.bit_eq(&d.doc),
            "{}: document round-trip not bit-identical",
            d.name
        );
        // Canonical bytes: re-saving the loaded state reproduces the
        // file exactly (BTreeMap ordering + ryu canonical floats).
        let resaved = save(&loaded, Tol::witness()).expect("re-save");
        assert_eq!(resaved, text, "{}: save bytes not canonical", d.name);
        // Replay identity: evaluation of the loaded document is
        // bit-identical — name tables, bodies, verdicts, keys.
        assert_eq!(
            eval_fingerprint(d.name, &d.doc),
            eval_fingerprint(d.name, &loaded),
            "{}: evaluation fingerprint drifted across save/load",
            d.name
        );
    }
}

#[test]
fn loaded_metadata_round_trips_structurally() {
    // Pass-through interop (D7): the loader reproduces the exact
    // canonical tree, bits and all, without interpreting it.
    let ks = corpus::sink::document();
    let text = save(&ks.doc, Tol::witness()).expect("save");
    let loaded = load(&text, Tol::witness()).expect("load");
    let attributed = ks
        .doc
        .appearance()
        .keys()
        .find(|n| {
            ks.doc
                .appearance()
                .get(n)
                .is_some_and(|r| r.metadata.contains_key("tool.example/annotation"))
        })
        .expect("the annotated name survived")
        .clone();
    let rec = loaded
        .appearance()
        .get(&attributed)
        .expect("record survived");
    assert_eq!(
        rec.metadata["tool.example/annotation"],
        corpus::sink::meta_tree()
    );
}
