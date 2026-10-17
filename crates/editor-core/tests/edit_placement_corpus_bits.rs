//! **Every corpus `Transform` places its body by the same bits** — the
//! value channel of each `Node::Transform` in the registry, pinned per
//! node against digests taken on the merge base before the node held a
//! `Placement`.
//!
//! The digest is the fixture's value channel (`value_digest_nodes`):
//! the placed bodies' arena counts and every stored point, so a motion
//! that moved one bit of one coordinate moves the word. A golden in the
//! ordinary sense: when one moves, the question is whether the new
//! motion is right, never how to restore the old number.

use crate::corpus;
use crate::fixture::value_channel::value_digest_nodes;
use corpus::{eval, failures};
use editor_core::Node;

/// `(document, node's mint ordinal, digest)` for every `Transform` the
/// registry holds, in registry then document order, the same at the
/// default, `1e-6` and `1e-12` ε rows. The digest feeds the node id, so
/// a re-minted id moves the word with no geometry moving; so does a
/// point landing in another arena slot.
///
/// Re-pinned when declaring a variable began minting its id: the rows of
/// the four documents that declare one (`die`, `heat_sink`,
/// `kitchen_sink`, `part_select`) moved with their re-minted node ids,
/// and every other row held its word.
///
/// **Re-pinned for INTENT-LITERALS PR C** (a slot holds a variable):
/// every node is minted from slots that hold variable ids, a typed
/// value's variable drawn from what it holds, so every node id moved
/// and with it every row this hashes. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held untouched across the change.
///
/// **Re-pinned when an id became its mint ordinal and its digest**: a
/// row names its node by ordinal, and every word moved with the id it
/// hashes; the ids-masked geometry fence held.
///
/// **Re-pinned when a selection became a variable**: the tour document
/// mints one selection ahead of its transforms, so each of its rows'
/// ordinals moved by one and its words with them; every other row held,
/// and so did the ids-masked geometry fence.
///
/// **Re-pinned for INTENT stage 3 PR A** (a revolve's axis is a line on
/// the node): `kitchen_sink`, `die_pips`, `die_composed` and the tour
/// document hold no in-plane axis datum, so their later ids moved —
/// ordinals by two where the datum and its frame read went — and every
/// row naming them; every other row held its word, and the ids-masked
/// geometry fence held. Taken merged over FORK-DM4 unit 1 (PR 4527).
const PINNED: &[(&str, u32, u64)] = &[
    ("die", 206, 0x1d1f8e2cc2c77841),
    ("die", 217, 0x3785f6c2ba33c653),
    ("die", 228, 0x3363eb4ff73f9450),
    ("die", 239, 0x7c530731603cf65d),
    ("die", 250, 0x16a67e6236d9b604),
    ("die", 261, 0x45be4dfc27529307),
    ("die", 272, 0xbe821eb0e7022f3d),
    ("die", 283, 0x1913295a11edfb23),
    ("die", 294, 0x7b63f6965d66520b),
    ("die", 305, 0x6ccbe66957580fd1),
    ("die", 316, 0x86490582263f56a0),
    ("die", 327, 0xcb52fa219a088672),
    ("die", 338, 0x6595f3d39824a5c4),
    ("die", 349, 0xeb30370b669c0ef6),
    ("die", 360, 0x196dc6fb4191848f),
    ("die", 371, 0x691c8c2dadb72338),
    ("die", 382, 0x40f78ac17a0954c5),
    ("die", 393, 0x680102bdf5bb645a),
    ("die", 404, 0x348213346708f0f1),
    ("die", 415, 0x79dacbb4b3757f7d),
    ("die", 426, 0x07a07e93956ce3bd),
    ("heat_sink", 74, 0x489cb40be1b10601),
    ("heat_sink", 85, 0xc48f84c3a2ca7050),
    ("heat_sink", 96, 0x861a4c449370dbcd),
    ("heat_sink", 107, 0x3c57788966584962),
    ("heat_sink", 118, 0xb3a54c8e814e0680),
    ("kitchen_sink", 95, 0x804d2736f9974e1d),
    ("die_pips", 65, 0xe4cf3b1479e5bd07),
    ("part_select", 60, 0x36f2cf9c61b0b7b5),
    ("die_composed", 65, 0xe4cf3b1479e5bd07),
    ("die_composed_tour", 69, 0xf26b930607f0d71f),
    ("die_composed_tour", 78, 0x0f065bc442238444),
    ("die_composed_tour", 87, 0x20990428e4bfbe29),
    ("die_composed_tour", 96, 0x840174787a213b73),
    ("die_composed_tour", 105, 0x4475498bd52abc42),
    ("die_composed_tour", 114, 0x480a23c6ceffba0b),
    ("die_composed_tour", 123, 0xc9e87da066fc9b4a),
    ("die_composed_tour", 132, 0x55087709634b3feb),
    ("die_composed_tour", 141, 0xf794f7bc5f7ae5eb),
    ("die_composed_tour", 150, 0x4026e2a242742308),
    ("die_composed_tour", 159, 0xc04428db66a1bff3),
    ("die_composed_tour", 168, 0x98c72e3a322bb58e),
    ("die_composed_tour", 177, 0x17e9fc4185377444),
    ("die_composed_tour", 186, 0xa2ca63c2993448a8),
    ("die_composed_tour", 195, 0x3fa325a9c74bc6b0),
    ("die_composed_tour", 204, 0x7237eff2c8caab01),
    ("die_composed_tour", 213, 0x09aaa94f9d2f8909),
    ("die_composed_tour", 222, 0x4576951c362850f0),
    ("die_composed_tour", 231, 0x003cd7e8f1876425),
    ("die_composed_tour", 240, 0xadd664dba16453ae),
    ("die_composed_tour", 249, 0x4bd1dcb8a1b5255d),
];

#[test]
fn every_corpus_transform_places_its_body_by_the_pinned_bits() {
    let mut got: Vec<(&'static str, u32, u64)> = Vec::new();
    for doc in corpus::documents() {
        let transforms: Vec<_> = doc
            .doc
            .ids()
            .iter()
            .copied()
            .filter(|id| matches!(doc.doc.node(*id), Some(Node::Transform { .. })))
            .collect();
        if transforms.is_empty() {
            continue;
        }
        let ev = eval::<f64>(&doc.doc);
        let bad = failures(&ev);
        assert!(bad.is_empty(), "{} failed to evaluate: {bad:?}", doc.name);
        for (id, word, fed) in value_digest_nodes(&ev) {
            if transforms.contains(&id) {
                assert!(fed > 0, "{} node {}: nothing was digested", doc.name, id);
                got.push((doc.name, id.0.ordinal(), word));
            }
        }
    }
    for (name, id, word) in &got {
        println!("    (\"{name}\", {id}, {word:#018x}),");
    }
    assert_eq!(
        got.as_slice(),
        PINNED,
        "a corpus transform's placed body moved"
    );
}
