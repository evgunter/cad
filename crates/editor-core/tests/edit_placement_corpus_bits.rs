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
    ("die", 17061346926331576421, 0xb12689d9d6bb51f3),
    ("die", 4423626361224294612, 0xdeb138cdc9cfa695),
    ("die", 10131222622516898457, 0xc7f04eda6a42d786),
    ("die", 12669634588876365626, 0x543f283eaf6fae32),
    ("die", 2099981066364238522, 0xa8d474f3cc5e4c96),
    ("die", 240260849689023890, 0xcc5473c79a9365f3),
    ("die", 12917214618081107605, 0x8ef4423d3a981662),
    ("die", 7858135498697945409, 0x4b3ce84cc3c859de),
    ("die", 12295462030928804260, 0xdb67e1abe922e287),
    ("die", 9460317804019271151, 0x1635923d57a73e15),
    ("die", 11045762232025114146, 0xe096ec5bc9fd5201),
    ("die", 12582636751142469999, 0x66cfa5e507bfc20d),
    ("die", 2619103930213709022, 0x74528629999c25e2),
    ("die", 7386413201909763574, 0xd2bc18310d6b31ca),
    ("die", 6980447468857588919, 0xe9430c658c823016),
    ("die", 6293367356075393044, 0x6aaecf2ecb84f4cc),
    ("die", 10305592442725763986, 0xad79a147d673e795),
    ("die", 6583090690359395054, 0x5594cb73d4c3f3f9),
    ("die", 7521805525117394705, 0xd4e5c1d5e91a442d),
    ("die", 9760567267691596174, 0xc3774c552338fe17),
    ("heat_sink", 10341978869039770069, 0xdc346345bc552a4f),
    ("heat_sink", 14296322084079648162, 0x6cc9faee5cfbff76),
    ("heat_sink", 12184786657845624742, 0x2503ab5e5ce94396),
    ("heat_sink", 17536180939684637166, 0x23ff8b267714897e),
    ("heat_sink", 1277121435761408711, 0x56d859c6f87fe8ab),
    ("kitchen_sink", 16425910021445123081, 0x93d1c126848a5354),
    ("die_pips", 2033039845984006919, 0xc6134ff6662bf60b),
    ("part_select", 1775943491813408770, 0x47178339ad862e3d),
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
