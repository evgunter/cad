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
    ("die", 206, 0x1d1f8e2cc2c77841),
    ("die", 217, 0xd99f3cebb7d169d6),
    ("die", 228, 0xfb602cf4fd1d5d33),
    ("die", 239, 0x75b75f95cc5bc1e2),
    ("die", 250, 0x7239c65c7625c19e),
    ("die", 261, 0xd28fffff8fdf024f),
    ("die", 272, 0xc8cabba90c56cb8f),
    ("die", 283, 0xdea59eaf84b2f195),
    ("die", 294, 0x7ebd906f8503ac6c),
    ("die", 305, 0xc555714fff562b9a),
    ("die", 316, 0x511783b475c1e1cb),
    ("die", 327, 0x7d804bee5084c98a),
    ("die", 338, 0x52a9ec735d933e09),
    ("die", 349, 0x871f863f3a31b943),
    ("die", 360, 0x98c506ae7d2dc942),
    ("die", 371, 0x220b062256ba0e0b),
    ("die", 382, 0xf8028f3362e3af28),
    ("die", 393, 0x3e63597491a6c28d),
    ("die", 404, 0xb923c7317b668b90),
    ("die", 415, 0xb4d2d63ba67c19e8),
    ("die", 426, 0xe89cadb8eff3bdc7),
    ("heat_sink", 74, 0x489cb40be1b10601),
    ("heat_sink", 85, 0x88560fcd6437282e),
    ("heat_sink", 96, 0x5a875864568067bd),
    ("heat_sink", 107, 0x93f832e88b4ee62f),
    ("heat_sink", 118, 0xd6b1054fe079adfd),
    ("kitchen_sink", 95, 0x6c3d200d1683b9be),
    ("die_pips", 68, 0xa472d779713b2147),
    ("part_select", 64, 0xd5fdf5f3da68de4b),
    ("die_composed", 68, 0xa472d779713b2147),
    ("die_composed_tour", 71, 0x9d947d1d538118e1),
    ("die_composed_tour", 80, 0xcebbf7bd9dca1004),
    ("die_composed_tour", 89, 0x82372caa3c1efd3a),
    ("die_composed_tour", 98, 0xb04c5e24300ad66c),
    ("die_composed_tour", 107, 0xaf9e0e368925f49d),
    ("die_composed_tour", 116, 0xe670ebd78d41a234),
    ("die_composed_tour", 125, 0x2339fcb3df84cb92),
    ("die_composed_tour", 134, 0x26e2629e752d9470),
    ("die_composed_tour", 143, 0x9e1e27abdb9171be),
    ("die_composed_tour", 152, 0x51b38f38720c1cef),
    ("die_composed_tour", 161, 0x7f50f0145ce32774),
    ("die_composed_tour", 170, 0x6bc53ad45356b0f1),
    ("die_composed_tour", 179, 0xa95c57173aa995cb),
    ("die_composed_tour", 188, 0x9faacde8163a09e2),
    ("die_composed_tour", 197, 0xbbeced20116063c9),
    ("die_composed_tour", 206, 0xad1f83c79c586f71),
    ("die_composed_tour", 215, 0xdbb01551c8af2beb),
    ("die_composed_tour", 224, 0xdef4747660d91bf6),
    ("die_composed_tour", 233, 0x2879f7909c546dc2),
    ("die_composed_tour", 242, 0x86dd6414ec8103af),
    ("die_composed_tour", 251, 0x4f9abaac2e3fb7dc),
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
