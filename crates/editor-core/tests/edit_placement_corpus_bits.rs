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
const PINNED: &[(&str, u32, u64)] = &[
    ("die", 58, 0x02ccfe6746763318),
    ("die", 60, 0xf323df7dde9d8c59),
    ("die", 62, 0xe369eff096aea1b4),
    ("die", 64, 0x2b9bec8b2a2ce111),
    ("die", 66, 0x6e3981d3264460de),
    ("die", 68, 0xf1cfd5a7f9238b77),
    ("die", 70, 0x026e84293ab33df2),
    ("die", 72, 0xd2692807dede8a0c),
    ("die", 74, 0x72bd38525dad3799),
    ("die", 76, 0x2ae90b85341ac928),
    ("die", 78, 0x1b0f40c5bd879b57),
    ("die", 80, 0x5ebc778b7fe041dd),
    ("die", 82, 0xa3fb11cb2b394e0f),
    ("die", 84, 0x65414abb85c62859),
    ("die", 86, 0x252432c3d13c2a89),
    ("die", 88, 0xa13e8d46a89d47bd),
    ("die", 90, 0x82cbfc37271c1d51),
    ("die", 92, 0x9ba708105f28f5ab),
    ("die", 94, 0x02fe5e8c4211d531),
    ("die", 96, 0x3d4a790e2d10e043),
    ("die", 98, 0x463ae4bfb348b1a3),
    ("heat_sink", 19, 0xd5d9c40700d9e7fe),
    ("heat_sink", 21, 0x65e374a7b8e6edeb),
    ("heat_sink", 23, 0x1556a15b6d486112),
    ("heat_sink", 25, 0xe6e15ad06e10118c),
    ("heat_sink", 27, 0xb0e2af246a1d1204),
    ("kitchen_sink", 24, 0xecb8d4e9f34ac356),
    ("die_pips", 16, 0xab71e6164a732142),
    ("part_select", 17, 0x59eed151c544b503),
    ("die_composed", 16, 0xab71e6164a732142),
    ("die_composed_tour", 17, 0x3981379782091844),
    ("die_composed_tour", 18, 0xcd7cb53dd49e7f20),
    ("die_composed_tour", 19, 0xbe192ef65dfd7734),
    ("die_composed_tour", 20, 0x8b405a8729bb498b),
    ("die_composed_tour", 21, 0x67037af29a6fee57),
    ("die_composed_tour", 22, 0xa61360a5736d3892),
    ("die_composed_tour", 23, 0x224094ca8fe4f28c),
    ("die_composed_tour", 24, 0xe49f19ec65b5744d),
    ("die_composed_tour", 25, 0x36ba8059eeffc6d1),
    ("die_composed_tour", 26, 0x0a82b75b5c9c9b55),
    ("die_composed_tour", 27, 0x9c642ecb412f9de7),
    ("die_composed_tour", 28, 0x8a962b51640372bc),
    ("die_composed_tour", 29, 0xcbb6cbcc13ea2e85),
    ("die_composed_tour", 30, 0xa3bd48ed216141a8),
    ("die_composed_tour", 31, 0xe5fdeb386fb99746),
    ("die_composed_tour", 32, 0xe94516dec1964f11),
    ("die_composed_tour", 33, 0x7612b357a64cec56),
    ("die_composed_tour", 34, 0x86d736b21cdca4b9),
    ("die_composed_tour", 35, 0x951cac56dcbebc8c),
    ("die_composed_tour", 36, 0xa45b53aca74065b8),
    ("die_composed_tour", 37, 0xd6a71d6008be9801),
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
