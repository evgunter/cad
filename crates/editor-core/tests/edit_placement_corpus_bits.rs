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
    ("die", 206, 0x0201056a7296e3e4),
    ("die", 217, 0x71d406cc0cc60f74),
    ("die", 228, 0xb4521c4484e98391),
    ("die", 239, 0xbc50e76947e3b5a8),
    ("die", 250, 0xaa55c7469581deef),
    ("die", 261, 0x9f673d473197df44),
    ("die", 272, 0x561516e44a0f36da),
    ("die", 283, 0x2137d04fbaf4685e),
    ("die", 294, 0xa21a197e6e5574d8),
    ("die", 305, 0x49fd4369f0cbdfd8),
    ("die", 316, 0x93fefd6b4d5d474f),
    ("die", 327, 0x3d18647827d82df6),
    ("die", 338, 0x69a9ec64ec3b0e00),
    ("die", 349, 0x87b17d8639ceffd0),
    ("die", 360, 0x05ff625695034714),
    ("die", 371, 0xba08f42a4427d5e3),
    ("die", 382, 0xb80d1a785000ecb8),
    ("die", 393, 0x39e3ede2f3293402),
    ("die", 404, 0x0c85be9104c24df4),
    ("die", 415, 0xa4406d9438e02b96),
    ("die", 426, 0x5a207cf08c40ee72),
    ("heat_sink", 74, 0xfb55d8412879ca18),
    ("heat_sink", 85, 0x92eb60d3afb04345),
    ("heat_sink", 96, 0xea2aec91eb7d9502),
    ("heat_sink", 107, 0x5a9cf341e00813d3),
    ("heat_sink", 118, 0xefdee73b23416e58),
    ("kitchen_sink", 95, 0x57e4bc98ea921817),
    ("die_pips", 68, 0x254e1fde3ae81c52),
    ("part_select", 64, 0x784b95a012eb4425),
    ("die_composed", 68, 0x254e1fde3ae81c52),
    ("die_composed_tour", 71, 0x278ec9264a55eb53),
    ("die_composed_tour", 80, 0xc68b4ebcef11034e),
    ("die_composed_tour", 89, 0x26f2f5592c79f5de),
    ("die_composed_tour", 98, 0xefea45a65b75de6f),
    ("die_composed_tour", 107, 0x414ed0544037b7f1),
    ("die_composed_tour", 116, 0xbc96ddf1ad7d1883),
    ("die_composed_tour", 125, 0xfbca381681daeaec),
    ("die_composed_tour", 134, 0x42c0408662f77e72),
    ("die_composed_tour", 143, 0x6fe92fe1ade3384c),
    ("die_composed_tour", 152, 0x510e9c5548a29d4a),
    ("die_composed_tour", 161, 0x08006acfb26dc5c4),
    ("die_composed_tour", 170, 0x4e0fbb7547272e25),
    ("die_composed_tour", 179, 0x3bcf26a6f3d55b24),
    ("die_composed_tour", 188, 0x36f0f07f948ee9b9),
    ("die_composed_tour", 197, 0x59fc7c4d8c858286),
    ("die_composed_tour", 206, 0x5d1e962ffb44a014),
    ("die_composed_tour", 215, 0x303539e23dc0a27e),
    ("die_composed_tour", 224, 0xbdf242c02b54e25a),
    ("die_composed_tour", 233, 0x31de43c353e62943),
    ("die_composed_tour", 242, 0xa04b92cf43847942),
    ("die_composed_tour", 251, 0xa24bf43e171e74fa),
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
