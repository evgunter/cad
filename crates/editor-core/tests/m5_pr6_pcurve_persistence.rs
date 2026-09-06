//! **M5 PR 6, the persistence posture, stated honestly** (spec §6).
//!
//! Pcurve caches are **re-derived on load, never serialized**. That is
//! not a compromise, it is what D6.1 persistence already is: a document
//! is a RECIPE (its edit log / snapshot), and loading replays it
//! through evaluation — so the C5 splitting lane re-runs and re-mints
//! every cache from the same deterministic pipeline. No pcurve-shaped
//! field enters the schema, no schema version moves, and there is no
//! stale-cache class to invalidate (C4's content-keyed cache transfer
//! stays banked, and nothing here needs it).
//!
//! This file pins the posture in both directions: the persisted bytes
//! contain nothing pcurve-shaped, and the reloaded document's caches
//! are bit-identical to the original's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use editor_core::{
    CancelToken, EvalOptions, NodeResult, ProfileDoc, SplitSide, ValuePayload, evaluate, load, save,
};
use geom_core::Tol;
use topo::Body;

/// Every body in an evaluation, with its node, in evaluation order.
fn bodies(ev: &editor_core::Evaluation<f64>) -> Vec<Body<f64>> {
    let mut out = Vec::new();
    for id in &ev.order {
        let Some(NodeResult::Ok(v)) = ev.nodes.get(id) else {
            continue;
        };
        match &v.payload {
            ValuePayload::Body(b) => out.push((**b).clone()),
            ValuePayload::Split { above, below } => {
                for side in [above, below] {
                    if let SplitSide::Body(b) = side {
                        out.push((**b).clone());
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// A stable, bit-faithful dump of a body's stored pcurve caches.
fn cache_dump(bodies: &[Body<f64>]) -> Vec<String> {
    let mut out = Vec::new();
    for (i, b) in bodies.iter().enumerate() {
        for (he, cache) in b.pcurves() {
            out.push(format!(
                "{i}|{he:?}|{:?}|{:?}|{:?}",
                cache.pcurve(),
                cache.params(),
                cache.certificate()
            ));
        }
    }
    out
}

fn evaluated(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The shape (i) corpus document (`cut_cylinder`) really does carry
/// stored pcurves after evaluation — otherwise the rows below would be
/// vacuous.
#[test]
fn the_cut_cylinder_corpus_document_carries_caches() {
    let doc = corpus::cut_cylinder::document();
    let ev = evaluated(&doc.doc);
    let dump = cache_dump(&bodies(&ev));
    assert!(
        !dump.is_empty(),
        "the tilted cut's curved faces must carry certified pcurves"
    );
}

/// D6.1 round trip: save, load, re-evaluate — the caches come back
/// bit-identical, **re-derived** rather than read back.
#[test]
fn a_round_trip_re_derives_bit_identical_caches() {
    let doc = corpus::cut_cylinder::document();
    let before = cache_dump(&bodies(&evaluated(&doc.doc)));

    // The SNAPSHOT shape (empty log): `load` replays the log ON TOP of
    // the snapshot, so passing both would evaluate the document twice.
    let bytes = save(&doc.doc, &[], Tol::witness()).unwrap();
    let loaded = load(&bytes, Tol::witness()).unwrap();
    let after = cache_dump(&bodies(&evaluated(&loaded.doc)));

    assert_eq!(before, after, "caches must replay bit-identically");
    assert!(!after.is_empty());
}

/// And the posture itself: nothing pcurve-shaped is in the bytes. The
/// schema is untouched by this PR — a reader that never heard of
/// pcurves loads these documents unchanged.
#[test]
fn the_persisted_bytes_carry_nothing_pcurve_shaped() {
    let doc = corpus::cut_cylinder::document();
    let text = save(&doc.doc, &doc.edits, Tol::witness()).unwrap();
    for needle in ["pcurve", "Pcurve", "Harmonic", "chart"] {
        assert!(
            !text.contains(needle),
            "the schema must stay recipe-level: found {needle:?}"
        );
    }
}
