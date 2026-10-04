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
///
/// Re-pinned when declaring a variable began minting its id: the rows of
/// the four documents that declare one (`die`, `heat_sink`,
/// `kitchen_sink`, `part_select`) moved with their re-minted node ids,
/// and every other row held its word.
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 13630649694864477172, 0x862739c6e706173b),
    ("die", 16428901414727891508, 0xc32ae2f19f5623a5),
    ("die", 13017228998170325707, 0xab3de6fe2b48b6b0),
    ("die", 3890541276213278604, 0xd47226a646622d18),
    ("die", 4139809199485132088, 0xf2470d4a0a29f1a9),
    ("die", 14815247063147112624, 0xcd8b6eb973a26679),
    ("die", 695421684268876998, 0xb68ceb5ca815f842),
    ("die", 12194047926463429064, 0x54995eeaa2a8248c),
    ("die", 12679145450045573974, 0xa38148e387b70c34),
    ("die", 3326898134111483112, 0x8692d4ee62ecd989),
    ("die", 5301243562161008580, 0xb0519db5b6f960ce),
    ("die", 16203765943255675870, 0xec97cce201db180e),
    ("die", 2462385774981593218, 0x720a8355800ce2da),
    ("die", 3597029350192660837, 0x8081ecd2b6996cbd),
    ("die", 6442737403409163554, 0x7a096da8b77b0f4f),
    ("die", 6682164989386179550, 0x5fa84be1c8391f6b),
    ("die", 7147838289019486601, 0x2a7235529cd9c501),
    ("die", 10757242513446565459, 0xdc6dea0a8a14df65),
    ("die", 12185040977992349792, 0xf334ca1ba98d599f),
    ("die", 8751039117019305883, 0xebb45336d16e52e4),
    ("die", 8417215917878320739, 0xeda43dfd37bd778d),
    ("heat_sink", 11235307439961258166, 0x6529b23ba091437f),
    ("heat_sink", 15542140952337107891, 0x8efb41c4380f0f0a),
    ("heat_sink", 20554895330027894, 0x60d598c3d6e13dcc),
    ("heat_sink", 651828539818508088, 0xc2ba04c48da2e06f),
    ("heat_sink", 3498821325472730611, 0xac9344d5ed00516b),
    ("kitchen_sink", 4614143121414073456, 0xd4e8daf8c4606505),
    ("die_pips", 14934167376393906020, 0xb1ffd78a3718bf3a),
    ("part_select", 9973078918449237802, 0x26c213b86f999c0b),
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
