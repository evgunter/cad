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

/// `(document, node id, digest)` for every `Transform` the registry
/// holds, in registry then document order — taken on `569540027`, and
/// the same at the default, `1e-6` and `1e-12` ε rows. The digest feeds
/// the node id, so a re-minted id moves the word with no geometry
/// moving; so does a point landing in another arena slot.
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 8228204478696989143, 0x638d9883d28755ce),
    ("die", 2400481935357007123, 0x0818d4778f4b8ee5),
    ("die", 3060309163839197816, 0x94cd4d9d15b95ca4),
    ("die", 3920751703889627194, 0x285e3f05a256ec4d),
    ("die", 98632764035910379, 0xf57f13e7421990f7),
    ("die", 13901143140265862892, 0x2b49f58abc76dffd),
    ("die", 9164616238925750999, 0xcb76809b80500431),
    ("die", 8305651701448295517, 0x538d33b57bc80408),
    ("die", 3381412353602557870, 0x16ce0c2cf6379c71),
    ("die", 17621322833503556337, 0xdfbbc5c4926b6b3f),
    ("die", 11248003359211628662, 0x0b03ae0dae0d1bd2),
    ("die", 4295652183230529122, 0x0e4782920f47249f),
    ("die", 12304741202878611330, 0x96b670a522a3b428),
    ("die", 7407374195616426729, 0x9bb586692b66fb52),
    ("die", 598352350308624190, 0x244307fba821c1b5),
    ("die", 11503673492551619118, 0xa4ae522f6a538abe),
    ("die", 14799717778164960586, 0x736114f592f5d1ed),
    ("die", 6999945651804675608, 0x0891375313dc7dff),
    ("die", 6213857481446779077, 0xf075b7fd6741946d),
    ("die", 4076862306422428900, 0x9c68131eec34267f),
    ("die", 15730739499188311629, 0xdfbc79cfd44eb7f8),
    ("heat_sink", 10341978869039770069, 0xdc346345bc552a4f),
    ("heat_sink", 556070422258884604, 0x67ce7854a2fbb300),
    ("heat_sink", 6940508960040532169, 0x0c52cb1d24ec7b0f),
    ("heat_sink", 15888990422211570191, 0xa40af663e8021c54),
    ("heat_sink", 15608840332042085708, 0xaeef904430f692ea),
    ("kitchen_sink", 2043405767949173533, 0xa7f589fbce3e77dc),
    ("die_pips", 2033039845984006919, 0xc6134ff6662bf60b),
    ("part_select", 13354542183570047136, 0xa46b1051cba9c491),
    ("die_composed", 2033039845984006919, 0xc6134ff6662bf60b),
    ("die_composed_tour", 9856071053958488618, 0x6fccef3b5090a60f),
    ("die_composed_tour", 3442839592220679850, 0x33800233f84c58cf),
    ("die_composed_tour", 5402420134371095911, 0x3b41119e1c5ec198),
    (
        "die_composed_tour",
        11930017114504326636,
        0x39b3ac07ca81e62e,
    ),
    ("die_composed_tour", 5407550019308273576, 0x8f9febf420aa8617),
    ("die_composed_tour", 6906824245305636669, 0x1e014ce21d52bdb1),
    ("die_composed_tour", 7630893841442318112, 0x4306efe8fddd1d83),
    (
        "die_composed_tour",
        12319722009332618453,
        0x1202e51b0997d6e0,
    ),
    (
        "die_composed_tour",
        15823547331409877257,
        0xce7b2f102ab2c843,
    ),
    (
        "die_composed_tour",
        13906050342778285137,
        0x3542cb38cc2d9fa2,
    ),
    (
        "die_composed_tour",
        13802394023050550899,
        0x5d801d9b8f114336,
    ),
    (
        "die_composed_tour",
        11552791030996529637,
        0x816d518d2ab8abe2,
    ),
    ("die_composed_tour", 9725959020143702077, 0x50279ac662d8788f),
    (
        "die_composed_tour",
        17664292210884808352,
        0xc70c3aa5ce4c6e3e,
    ),
    ("die_composed_tour", 321870594419961657, 0x252c74ed1d837eb3),
    (
        "die_composed_tour",
        13278526489901090031,
        0x4afa527d14824d0a,
    ),
    (
        "die_composed_tour",
        10626493373964279257,
        0x3691386f798f58ce,
    ),
    ("die_composed_tour", 624723659480373324, 0x18aa6798eb6ecf01),
    ("die_composed_tour", 9023552072712435283, 0x5e24ee5ba57a0be5),
    (
        "die_composed_tour",
        15507196318239516304,
        0x029165cb7ace0ffd,
    ),
    (
        "die_composed_tour",
        12714632731979897124,
        0x94254990d227c3b8,
    ),
];

#[test]
fn every_corpus_transform_places_its_body_by_the_pinned_bits() {
    let mut got: Vec<(&'static str, u64, u64)> = Vec::new();
    for doc in corpus::documents() {
        let transforms: Vec<_> = doc
            .doc
            .order()
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
                assert!(fed > 0, "{} node {}: nothing was digested", doc.name, id.0);
                got.push((doc.name, id.0, word));
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
