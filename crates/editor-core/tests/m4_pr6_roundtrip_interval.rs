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

mod corpus;
mod fixture;

use editor_core::{CancelToken, EvalOptions, ProfileDoc, evaluate, load, save};
use geom_core::Interval;

use corpus::documents;

fn fingerprint(doc: &ProfileDoc) -> String {
    let ev = evaluate::<Interval>(doc, None, &CancelToken::new(), &EvalOptions::default());
    format!("{:?}|{:?}|{:?}", ev.order, ev.nodes, ev.appearance)
}

#[test]
fn interval_replay_identity_across_save_load() {
    for d in documents() {
        let text = save(&d.doc, &[]).expect("save");
        let loaded = load(&text).expect("load");
        assert!(
            loaded.doc.bit_eq(&d.doc),
            "{}: round-trip bit identity",
            d.name
        );
        assert_eq!(
            fingerprint(&d.doc),
            fingerprint(&loaded.doc),
            "{}: Interval evaluation fingerprint drifted across save/load",
            d.name
        );
    }
}
