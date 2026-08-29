//! **R1 evidence-only differential harness** (M10-1 review, claim 1):
//! prints a bit-level digest of every corpus document's f64 evaluation
//! so the SAME harness can be run at the PR head and at its merge base
//! and the outputs diffed. Revision-neutral by construction: it names
//! no M10-1 API.
//!
//! EVIDENCE, NOT A GATE (`memories/test-suite-cost.md`, one-shot
//! comparison artefacts): the single assertion is green-ness; the
//! digest is the payload, read by the reviewer who diffs the two runs.
//! Retire this file once the M10-1 comparison is taken, or name the
//! next comparison that wants it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use corpus::{body_of, documents, eval, failures};
use geom_core::Tol;
use topo::mass_properties;

#[test]
fn print_corpus_f64_digest() {
    for d in documents() {
        let ev = eval::<f64>(&d.doc);
        assert!(
            failures(&ev).is_empty(),
            "{}: corpus must be green for the digest to mean anything",
            d.name
        );
        let (volume, area) = match d.result {
            None => (0u64, 0u64),
            Some(id) => {
                let m = mass_properties(body_of(&ev, id), Tol::witness()).expect("props");
                (m.volume.to_bits(), m.surface_area.to_bits())
            }
        };
        println!(
            "R1DIGEST {} nodes={} order={} vol={:016x} area={:016x}",
            d.name,
            d.doc.len(),
            ev.order.len(),
            volume,
            area
        );
    }
}
