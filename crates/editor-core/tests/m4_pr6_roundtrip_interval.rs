//! M4 PR 6 spec D6.1, Interval lane: save/load/replay-identity with
//! the evaluation instantiated at the certified interval scalar. The
//! document layer is scalar-independent (stored exact f64 embeds via
//! `from_f64`), so the pin here is that a LOADED document's Interval
//! evaluation is bit-identical to the original's — enclosures, name
//! tables, verdicts and all (Debug encodes interval endpoints
//! shortest-round-trip, bit-faithfully).
//!
//! M4 PR 8a: the fixture is the whole Band 4 corpus, not just the die.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use editor_core::{CancelToken, EvalOptions, ProfileDoc, evaluate, load, save};
use geom_core::Interval;

use corpus::documents;
use geom_core::Tol;

fn fingerprint(label: &str, doc: &ProfileDoc) -> String {
    let ev = evaluate::<Interval>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    // #117: fingerprint identity is blind to evaluation health — assert
    // green so a sick fixture fails loudly (rationale in the f64 lane,
    // m4_pr6_roundtrip.rs).
    let bad = corpus::failures(&ev);
    assert!(
        bad.is_empty(),
        "{label}: persistence fixture did not evaluate green (#117):\n{}",
        bad.join("\n")
    );
    format!("{:?}|{:?}|{:?}", ev.order, ev.nodes, ev.appearance)
}

#[test]
fn interval_replay_identity_across_save_load() {
    for d in documents() {
        let text = save(&d.doc, &[], Tol::witness()).expect("save");
        let loaded = load(&text, Tol::witness()).expect("load");
        assert!(
            loaded.doc.bit_eq(&d.doc),
            "{}: round-trip bit identity",
            d.name
        );
        assert_eq!(
            fingerprint(d.name, &d.doc),
            fingerprint(d.name, &loaded.doc),
            "{}: Interval evaluation fingerprint drifted across save/load",
            d.name
        );
    }
}
