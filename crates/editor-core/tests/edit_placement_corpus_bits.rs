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
    ("die", 13994026987675326066, 0xfd1014f5e20cbf71),
    ("die", 4365837206675677594, 0x944702db7bb6dc1a),
    ("die", 1164309454870877331, 0xecb847c507c3171f),
    ("die", 11546348034858879516, 0x0854bd087dd0322e),
    ("die", 12232369640279693749, 0x95d1475474e38a2f),
    ("die", 7510813398810767164, 0x3c2e8277f9b8fac5),
    ("die", 10570193506000302247, 0x5013af49b5221c10),
    ("die", 16583934643976514817, 0xa50af1e18f159620),
    ("die", 10627962774813810821, 0x518b29c8d7fbda19),
    ("die", 9924431502639197107, 0x208216a86b33c3f4),
    ("die", 543953658387954354, 0xe6ba4abd63d15ada),
    ("die", 15472226987917727993, 0x7b556a3bb218b4dd),
    ("die", 13515225346229264379, 0x09863f51122891b2),
    ("die", 5171531494409520269, 0x55a36a7529bb91a4),
    ("die", 1569351781973311484, 0xd80d5a7c3dfc78d5),
    ("die", 17629021721480173891, 0x068180fc248932fa),
    ("die", 7128419856343990765, 0xd2ee8d9d7e1dfd2d),
    ("die", 3219761458039500975, 0xbb7913c68e4938e6),
    ("die", 4411090495949719371, 0xe404244c6aeb0061),
    ("die", 16630946352607348628, 0x8bcd9616317f79ba),
    ("die", 4518833902769350878, 0xfbea2ec637b5f50a),
    ("heat_sink", 4703293425767977008, 0xd5fe4b48602052dd),
    ("heat_sink", 11646378301052922026, 0xb6b45e184f2f7252),
    ("heat_sink", 17791236936980340351, 0x2a021f4f63c13319),
    ("heat_sink", 6313698637056090348, 0xa264da5e05431294),
    ("heat_sink", 4274860024317929404, 0x3fc7f9fdefb52d5b),
    ("kitchen_sink", 13987405627608502392, 0x02a1dc12d8de1197),
    ("die_pips", 14934167376393906020, 0xb1ffd78a3718bf3a),
    ("part_select", 409985313263932839, 0x6726415cea4993e0),
    ("die_composed", 14934167376393906020, 0xb1ffd78a3718bf3a),
    (
        "die_composed_tour",
        16960854264500600376,
        0x29a2603303df1557,
    ),
    ("die_composed_tour", 4024003207080596246, 0xd21aeb8979acc971),
    (
        "die_composed_tour",
        12820861895341595870,
        0x6175a159d03385ab,
    ),
    ("die_composed_tour", 1969896633244506434, 0x04600238bf5b8612),
    ("die_composed_tour", 7904589757822710700, 0x39cf6cc4cce35cf6),
    ("die_composed_tour", 5778639837828218104, 0xa98005679d71bb6f),
    (
        "die_composed_tour",
        13033851799668782135,
        0x1c1eb755a41cc77c,
    ),
    (
        "die_composed_tour",
        12458354866338640224,
        0x178c4298bb2da35b,
    ),
    ("die_composed_tour", 766812983101097739, 0xa5dce43b6eba5a97),
    ("die_composed_tour", 5918991139494938117, 0xbf7c35b7a6dd4090),
    (
        "die_composed_tour",
        18074796592197805631,
        0x7ffeb9f407173802,
    ),
    (
        "die_composed_tour",
        11394747261841190062,
        0x47ccae30ef2efbb9,
    ),
    (
        "die_composed_tour",
        16579740739993622022,
        0xe0575a561108feb7,
    ),
    (
        "die_composed_tour",
        18208001098458103156,
        0xb6045354b84a0667,
    ),
    ("die_composed_tour", 717334240356059075, 0x505170c5f6f56951),
    ("die_composed_tour", 7298697664680082213, 0x2b22cc4803cfb0f3),
    ("die_composed_tour", 3592163807252616815, 0xce6b46237ac47400),
    (
        "die_composed_tour",
        10938537313497100721,
        0x6e605589c7bf4ebd,
    ),
    ("die_composed_tour", 1599215149336221801, 0x76d794d2ec53732e),
    ("die_composed_tour", 4209480348272499983, 0x16da5a48a973554e),
    (
        "die_composed_tour",
        16667962929149992230,
        0x36e1cec05394a40f,
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
