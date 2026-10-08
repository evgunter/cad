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
const PINNED: &[(&str, u32, u64)] = &[
    ("die", 206, 0xf0b50b9715642c98),
    ("die", 217, 0x2f2a645ddaee6b13),
    ("die", 228, 0xaa531a7115113814),
    ("die", 239, 0xb7ffb70cd40228b9),
    ("die", 250, 0xcda40566a5b2b3e3),
    ("die", 261, 0xf654405a50fb4cc0),
    ("die", 272, 0xca845b75efdb25b5),
    ("die", 283, 0xf96037c149fb9aab),
    ("die", 294, 0x2efd0725dfb52e14),
    ("die", 305, 0xf83dcfa80d43619b),
    ("die", 316, 0x53daa57d589e0698),
    ("die", 327, 0xcffa4a9604856a42),
    ("die", 338, 0x0d556a68bc0d2970),
    ("die", 349, 0x81338d658ebde776),
    ("die", 360, 0xc3e06724520a72d5),
    ("die", 371, 0xd910071437ca6e0b),
    ("die", 382, 0xeaf0550348e367ba),
    ("die", 393, 0x5dca085430636c03),
    ("die", 404, 0xb06bd6ae43a375e6),
    ("die", 415, 0x3839975ec5ce75f1),
    ("die", 426, 0xe5eb4a4cfe10d990),
    ("heat_sink", 74, 0x48df56138bf241f6),
    ("heat_sink", 85, 0x7e567dff013de9ed),
    ("heat_sink", 96, 0xc927d865776e2526),
    ("heat_sink", 107, 0x675ff945992755ac),
    ("heat_sink", 118, 0xaab9a522e7252328),
    ("kitchen_sink", 95, 0x2a8daf62ca4812f7),
    ("die_pips", 68, 0xa712acda52d9a777),
    ("part_select", 64, 0x1e5f46b0cd67bc2a),
    ("die_composed", 68, 0xa712acda52d9a777),
    ("die_composed_tour", 71, 0xb1028ec60000ba2f),
    ("die_composed_tour", 80, 0xaba36f1c7e18d4ec),
    ("die_composed_tour", 89, 0x4dcf56d3a8b5af2c),
    ("die_composed_tour", 98, 0xc840c782d735dacb),
    ("die_composed_tour", 107, 0x40d0a11102199dc4),
    ("die_composed_tour", 116, 0x005ced04a2b3775c),
    ("die_composed_tour", 125, 0xee192e7bea1d2e5a),
    ("die_composed_tour", 134, 0x270291002e4d9cf6),
    ("die_composed_tour", 143, 0x26b158b75cbc459e),
    ("die_composed_tour", 152, 0xaffc4b355d854e11),
    ("die_composed_tour", 161, 0xcbc481eed87f6584),
    ("die_composed_tour", 170, 0xa5a9e70bd695b50e),
    ("die_composed_tour", 179, 0x8e3ce753a0a45aa1),
    ("die_composed_tour", 188, 0xaa46d9774fe26936),
    ("die_composed_tour", 197, 0xea32e8a39eedb148),
    ("die_composed_tour", 206, 0xceb228d3ce55ba17),
    ("die_composed_tour", 215, 0x2b7baac472de473c),
    ("die_composed_tour", 224, 0xaf37947017dbc96e),
    ("die_composed_tour", 233, 0xe9039c0d4bcd61fd),
    ("die_composed_tour", 242, 0x4d7f967fd758ba86),
    ("die_composed_tour", 251, 0x7fd767cd65041cbc),
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
