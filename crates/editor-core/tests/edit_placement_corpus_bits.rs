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
/// moving.
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 8228204478696989143, 0x638d9883d28755ce),
    ("die", 2142085009859690040, 0x07a7406327266dd8),
    ("die", 16641180976449670160, 0x13b1498f8b6dda47),
    ("die", 15122733293216335162, 0xda5ee2bbf7de1a85),
    ("die", 12796615812804999765, 0x10a53b6cead83b77),
    ("die", 16746126762965662293, 0xdff41ba655febb15),
    ("die", 5355727924105142303, 0x0a03ebcc7d61a8ae),
    ("die", 8562494384789451500, 0x3371e95513f6896e),
    ("die", 3059686154848946292, 0xaace545045b588e9),
    ("die", 5696663369424421488, 0x8ddace50794b8647),
    ("die", 11133737891945214133, 0x5ce0d99c7eede90e),
    ("die", 4516141624092323051, 0x07242675c8fdec0c),
    ("die", 5043039883928955296, 0x426c835d9fe42ac2),
    ("die", 18120281526273524681, 0x59a65c450475a9f2),
    ("die", 7264661415334787288, 0x221b9a5e9176707c),
    ("die", 12507770361027217948, 0xb5fb1a469de6b67f),
    ("die", 3462147162018858052, 0xfc688bd0c04179c9),
    ("die", 4753516541850422195, 0x3268a4ac06025781),
    ("die", 11882107217539281851, 0x0111215f26ae1bcc),
    ("die", 7232861933515004077, 0x7bdb8d7c24a447c9),
    ("die", 18105214194993766205, 0x330ade1b6547a7a2),
    ("heat_sink", 10341978869039770069, 0xdc346345bc552a4f),
    ("heat_sink", 556070422258884604, 0x67ce7854a2fbb300),
    ("heat_sink", 6940508960040532169, 0x0c52cb1d24ec7b0f),
    ("heat_sink", 15888990422211570191, 0xa40af663e8021c54),
    ("heat_sink", 15608840332042085708, 0xaeef904430f692ea),
    ("kitchen_sink", 6440793427619528015, 0xb817351db3700562),
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
