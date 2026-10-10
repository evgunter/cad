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
const PINNED: &[(&str, u32, u64)] = &[
    ("die", 206, 0x1d1f8e2cc2c77841),
    ("die", 217, 0x6958a2d0fdcfb47d),
    ("die", 228, 0x9b49f2b1dc1261aa),
    ("die", 239, 0x878d62c9a1931ec0),
    ("die", 250, 0x9b9a6d8fb0fb97e7),
    ("die", 261, 0x423b31e397a31d8b),
    ("die", 272, 0x7ac9465b623fc976),
    ("die", 283, 0xadf38fb58a48418d),
    ("die", 294, 0xf6b6c97b9ae7c5e3),
    ("die", 305, 0xb24659281728fd10),
    ("die", 316, 0x2d7a3e9304c5d101),
    ("die", 327, 0x1e53907147ad5ced),
    ("die", 338, 0x29c09885cbc05810),
    ("die", 349, 0x391784da34cc11ae),
    ("die", 360, 0x992436a0ed5ebde0),
    ("die", 371, 0x502eaefc277902fe),
    ("die", 382, 0x6612b5e851c5a0f9),
    ("die", 393, 0xe66392f9a21a9d86),
    ("die", 404, 0xd474712e508fe457),
    ("die", 415, 0x69c83bb2a36ae00b),
    ("die", 426, 0x09ae1d9c677084b6),
    ("heat_sink", 74, 0x489cb40be1b10601),
    ("heat_sink", 85, 0x682100089a494dff),
    ("heat_sink", 96, 0xda08377852891629),
    ("heat_sink", 107, 0xda6f32bd53b81eee),
    ("heat_sink", 118, 0xc807937c5465ee9b),
    ("kitchen_sink", 95, 0xcbb9a35fb5efdfb9),
    ("die_pips", 66, 0x3d71a421838e2998),
    ("part_select", 64, 0xeb23db4f7e1e1b46),
    ("die_composed", 66, 0x3d71a421838e2998),
    ("die_composed_tour", 70, 0x39eec164647550ed),
    ("die_composed_tour", 79, 0xfc601184b6fc2fbf),
    ("die_composed_tour", 88, 0xd3f0085f4000e2a7),
    ("die_composed_tour", 97, 0x788d10cc494d92a3),
    ("die_composed_tour", 106, 0x61ecd055b6d6ac98),
    ("die_composed_tour", 115, 0xa84c1386e8975904),
    ("die_composed_tour", 124, 0xe5c58557ae0952af),
    ("die_composed_tour", 133, 0x06706c8ffe901572),
    ("die_composed_tour", 142, 0xceebc2885fb47bc7),
    ("die_composed_tour", 151, 0x1353af534d629e31),
    ("die_composed_tour", 160, 0x44345bd453aa5c84),
    ("die_composed_tour", 169, 0x1e93a5002169d373),
    ("die_composed_tour", 178, 0x090d47379b784835),
    ("die_composed_tour", 187, 0xdd1947903d37f26e),
    ("die_composed_tour", 196, 0x22c7d552b7124fef),
    ("die_composed_tour", 205, 0x963d7364ae8c897a),
    ("die_composed_tour", 214, 0x770471b37929ce4d),
    ("die_composed_tour", 223, 0x1452417a62d0fb02),
    ("die_composed_tour", 232, 0x12eda247c5a28e71),
    ("die_composed_tour", 241, 0xfc3510c67d5d1089),
    ("die_composed_tour", 250, 0x213b8c3b14c19ff2),
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
