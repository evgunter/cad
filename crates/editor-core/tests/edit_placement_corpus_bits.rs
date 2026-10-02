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
    ("die", 6430079084679823177, 0x31d8f2c18ba202a0),
    ("die", 1026737612461335188, 0x242271aa52b449ad),
    ("die", 11663644381132420044, 0xd0c4d31ad850cd5f),
    ("die", 6551389482247432038, 0x2fc7ccfe874d8542),
    ("die", 12322422269646474671, 0x66af6b3846faba1d),
    ("die", 10449394397978734407, 0x4ad0a9b145dfa8a7),
    ("die", 16828923182902413872, 0x65ceb55bbd2da0cd),
    ("die", 16238774302290583011, 0x96a0997ea5f507d2),
    ("die", 1531214353026347697, 0x4dfc2159fc85ce2e),
    ("die", 16100331003521135017, 0xea5003a6d30c8f72),
    ("die", 2577355380310980068, 0xae0cd02ae1ecbfe9),
    ("die", 3841714464349156286, 0x380e0b11fcf3da80),
    ("die", 17672577931966138572, 0xd7d48088d53396c5),
    ("die", 11168948534305298090, 0x62ecff216d6699cb),
    ("die", 192556067493213873, 0x4cbae73109dccc8e),
    ("die", 16337580667634619768, 0x8a242c5c08253d38),
    ("die", 11157589262949597284, 0xb873c7631bc3c794),
    ("die", 1623252865700918811, 0xe33caf4270fdce2a),
    ("die", 10544574991883998577, 0x69e14af4ec5ef0a1),
    ("die", 17298582033192872292, 0xafcf264b78ce762d),
    ("heat_sink", 4703293425767977008, 0xd5fe4b48602052dd),
    ("heat_sink", 7523799715572701005, 0x23838696373b3621),
    ("heat_sink", 4236563546953780564, 0x9f13fc1e6acf6e24),
    ("heat_sink", 14094148100680571469, 0x26ca937457666fb3),
    ("heat_sink", 8359137903314853267, 0x02f2eff161c65719),
    ("kitchen_sink", 9091264090001667970, 0x255aba885b57a2ec),
    ("die_pips", 14934167376393906020, 0xb1ffd78a3718bf3a),
    ("part_select", 6792915809084376394, 0x2bc963e94e02630a),
    ("die_composed", 14934167376393906020, 0xb1ffd78a3718bf3a),
    ("die_composed_tour", 16960854264500600376, 0x29a2603303df1557),
    ("die_composed_tour", 4024003207080596246, 0xd21aeb8979acc971),
    ("die_composed_tour", 12820861895341595870, 0x6175a159d03385ab),
    (
        "die_composed_tour",
        1969896633244506434,
        0x04600238bf5b8612,
    ),
    ("die_composed_tour", 7904589757822710700, 0x39cf6cc4cce35cf6),
    ("die_composed_tour", 5778639837828218104, 0xa98005679d71bb6f),
    ("die_composed_tour", 13033851799668782135, 0x1c1eb755a41cc77c),
    (
        "die_composed_tour",
        12458354866338640224,
        0x178c4298bb2da35b,
    ),
    (
        "die_composed_tour",
        766812983101097739,
        0xa5dce43b6eba5a97,
    ),
    (
        "die_composed_tour",
        5918991139494938117,
        0xbf7c35b7a6dd4090,
    ),
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
    ("die_composed_tour", 16579740739993622022, 0xe0575a561108feb7),
    (
        "die_composed_tour",
        18208001098458103156,
        0xb6045354b84a0667,
    ),
    ("die_composed_tour", 717334240356059075, 0x505170c5f6f56951),
    (
        "die_composed_tour",
        7298697664680082213,
        0x2b22cc4803cfb0f3,
    ),
    (
        "die_composed_tour",
        3592163807252616815,
        0xce6b46237ac47400,
    ),
    ("die_composed_tour", 10938537313497100721, 0x6e605589c7bf4ebd),
    ("die_composed_tour", 1599215149336221801, 0x76d794d2ec53732e),
    (
        "die_composed_tour",
        4209480348272499983,
        0x16da5a48a973554e,
    ),
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
