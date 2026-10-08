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
    ("die", 185, 0xdee2c19ff64f28b9),
    ("die", 194, 0xb8755a5339d2256f),
    ("die", 203, 0x3af4fad9d1ee2e18),
    ("die", 212, 0x756014b1e7246fd7),
    ("die", 221, 0x527c5ad82a25b16b),
    ("die", 230, 0xc1e2926144810074),
    ("die", 239, 0x1726ec2c933d8211),
    ("die", 248, 0xeb1f03f0baf1903e),
    ("die", 257, 0xaf8e226570f53545),
    ("die", 266, 0xbb3ee819c6be1747),
    ("die", 275, 0xd087603ca5e61231),
    ("die", 284, 0x17a87e9cff95f8c0),
    ("die", 293, 0xf44a1575d32398af),
    ("die", 302, 0x7a9f0af6193270ba),
    ("die", 311, 0x1307bfa613d87b9d),
    ("die", 320, 0x0426999c2d4bcd7b),
    ("die", 329, 0x6f3318f3ece33654),
    ("die", 338, 0x21069cba4fb09cc4),
    ("die", 347, 0x0a52dda7d32f69d4),
    ("die", 356, 0x45105da9d4160dfb),
    ("die", 365, 0x92f7e0c80213f199),
    ("heat_sink", 67, 0xe27c5c3a8c987274),
    ("heat_sink", 76, 0xde6ef8852a65750b),
    ("heat_sink", 85, 0x08c46d2d7cc96330),
    ("heat_sink", 94, 0x1464c7b232f99479),
    ("heat_sink", 103, 0x64e2ae5704de05dc),
    ("kitchen_sink", 83, 0xcaf9146a1c21014b),
    ("die_pips", 60, 0x5c1ae6cb8f222530),
    ("part_select", 53, 0xc8fcc5a141cc196e),
    ("die_composed", 60, 0x5c1ae6cb8f222530),
    ("die_composed_tour", 62, 0xd5054f41c4f8fdc2),
    ("die_composed_tour", 70, 0xececb391e114e88d),
    ("die_composed_tour", 78, 0x048d1a647e6074a9),
    ("die_composed_tour", 86, 0xcde16263d33d9efa),
    ("die_composed_tour", 94, 0xdf031d67946aabb6),
    ("die_composed_tour", 102, 0xfd7e7ab9b654e578),
    ("die_composed_tour", 110, 0xcb2adeaf8484a581),
    ("die_composed_tour", 118, 0xf910f862faccb718),
    ("die_composed_tour", 126, 0x4e684c9837851457),
    ("die_composed_tour", 134, 0xba60b5b64dc30ced),
    ("die_composed_tour", 142, 0x47ff1bb39c31a8c9),
    ("die_composed_tour", 150, 0x1af4f7c9f17ae1bf),
    ("die_composed_tour", 158, 0x4d05cf8aa85a5b88),
    ("die_composed_tour", 166, 0xfac79734855180d2),
    ("die_composed_tour", 174, 0xd7ac74d70c886829),
    ("die_composed_tour", 182, 0xe8d5beb41b7cab94),
    ("die_composed_tour", 190, 0xca3b6b0d57ab322b),
    ("die_composed_tour", 198, 0x6af551cafd3e1121),
    ("die_composed_tour", 206, 0x09d8c27cd94282bc),
    ("die_composed_tour", 214, 0x1153f76a856917fb),
    ("die_composed_tour", 222, 0xd9464f69c0e94508),
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
