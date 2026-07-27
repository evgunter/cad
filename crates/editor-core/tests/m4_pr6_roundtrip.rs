//! M4 PR 6 spec D6.1 — the save/load/replay-identity CI row.
//!
//! The fixtures ARE the M4 PR 8a Band 4 corpus (`corpus/mod.rs`), each
//! in both persisted shapes: as a pure edit LOG over the empty
//! snapshot (so `load` must replay the entire document through
//! `apply`'s doors) and as a bare SNAPSHOT with no log. Every fixture
//! round-trips BIT-identically (snapshot, edit log, canonical bytes)
//! AND replays to a bit-identical evaluation — name tables, bodies
//! (arena floats via Debug's shortest-round-trip encoding, which is
//! bit-faithful for the finite values documents carry), verdict logs,
//! content keys. The CI matrix runs this file at ε ∈ {1e-6, 1e-9,
//! 1e-12} and under the interval feature
//! (`m4_pr6_roundtrip_interval` wraps the same corpus).
//!
//! D2 lives in `m4_pr6_floats.rs` (the bit-level float property row).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{
    CancelToken, DocEdit, EvalOptions, ProfileDesc, ProfileDoc, apply, evaluate, load, save,
};

use corpus::documents;

/// The whole-evaluation bit fingerprint: `Debug` of every node result
/// (payload arenas, name tables, verdicts, content keys) plus the
/// appearance resolution. `Debug` prints floats shortest-round-trip
/// (bit-faithful for finite values, sign of zero included).
fn eval_fingerprint(doc: &ProfileDoc) -> String {
    let ev = evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default());
    format!("{:?}|{:?}|{:?}", ev.order, ev.nodes, ev.appearance)
}

/// One persisted fixture: (label, snapshot, edit log, whether to spend
/// an evaluation on it). Both shapes of a document replay to the SAME
/// state, so the (expensive) evaluation fingerprint is checked once per
/// document — on the log shape, the one with more machinery between
/// the bytes and the state.
type Fixture = (String, ProfileDoc, Vec<DocEdit<ProfileDesc>>, bool);

fn files() -> Vec<Fixture> {
    let mut out = Vec::new();
    for d in documents() {
        out.push((
            format!("{}_log", d.name),
            ProfileDoc::empty(),
            d.edits.clone(),
            true,
        ));
        out.push((format!("{}_snapshot", d.name), d.doc, Vec::new(), false));
    }
    out
}

#[test]
fn save_load_replay_identity() {
    for (label, snapshot, edits, check_eval) in files() {
        // The expected current state: snapshot + edits.
        let mut expected = snapshot.clone();
        for edit in &edits {
            expected = apply(&expected, edit).expect("corpus edit").doc;
        }
        let text = save(&snapshot, &edits).expect("save");
        let loaded = load(&text).unwrap_or_else(|e| panic!("{label}: load refused: {e:?}"));
        assert!(
            loaded.snapshot.bit_eq(&snapshot),
            "{label}: snapshot round-trip not bit-identical"
        );
        assert_eq!(loaded.edits, edits, "{label}: edit log round-trip");
        assert!(
            loaded.doc.bit_eq(&expected),
            "{label}: replayed document not bit-identical"
        );
        // Canonical bytes: re-saving the loaded state reproduces the
        // file exactly (BTreeMap ordering + ryu canonical floats).
        let resaved = save(&loaded.snapshot, &loaded.edits).expect("re-save");
        assert_eq!(resaved, text, "{label}: save bytes not canonical");
        // Replay identity: evaluation of the loaded document is
        // bit-identical — name tables, bodies, verdicts, keys.
        if check_eval {
            assert_eq!(
                eval_fingerprint(&expected),
                eval_fingerprint(&loaded.doc),
                "{label}: evaluation fingerprint drifted across save/load"
            );
        }
    }
}

#[test]
fn loaded_metadata_round_trips_structurally() {
    // Pass-through interop (D7): the loader reproduces the exact
    // canonical tree, bits and all, without interpreting it.
    let ks = corpus::sink::document();
    let text = save(&ks.doc, &[]).expect("save");
    let loaded = load(&text).expect("load");
    let attributed = ks
        .doc
        .appearance()
        .keys()
        .find(|n| {
            ks.doc
                .appearance_of(n)
                .is_some_and(|r| r.metadata.contains_key("tool.example/annotation"))
        })
        .expect("the annotated name survived")
        .clone();
    let rec = loaded
        .doc
        .appearance_of(&attributed)
        .expect("record survived");
    assert_eq!(
        rec.metadata["tool.example/annotation"],
        corpus::sink::meta_tree()
    );
}
