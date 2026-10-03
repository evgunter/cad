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
    ("die", 11402305577101294191, 0xfa2e5845bcb0e619),
    ("die", 10676785139024760638, 0xc0e1c2339fa3ac3d),
    ("die", 6285677863219434555, 0xaacdf817dc11154a),
    ("die", 11182270308134637922, 0x49fdfc961d5a60b5),
    ("die", 3028539478167776908, 0x1e44d63b5ebdad9f),
    ("die", 9864336175432859418, 0x3de5f7f249aef570),
    ("die", 5693082155425590476, 0xdbff92e86ce293ad),
    ("die", 14174140763150234211, 0xfc418684e4630da9),
    ("die", 18245451659192426056, 0xb03aceba4527d5fd),
    ("die", 11936417610758949048, 0x80b2e30cdf644a9c),
    ("die", 6619702148143929121, 0x280365244e24f5cb),
    ("die", 7844317743803437933, 0x00be94bf6c929673),
    ("die", 17219235896402659372, 0x534d11543c4dc51f),
    ("die", 3216956210187592142, 0xf3d9f6306a06cc12),
    ("die", 1998696390944862269, 0x1c28bb82e4591df6),
    ("die", 493191438212165245, 0xc8e6ab2195c986ed),
    ("die", 3526311439032985959, 0x1955acfa31b8ba26),
    ("die", 3116061997226106248, 0x7f018d96c184c3ab),
    ("die", 448649542752672983, 0xf366978950a02644),
    ("die", 15344748963357690921, 0x802834e121bb0c63),
    ("die", 3976450572082682283, 0x5e0ca58e198de221),
    ("heat_sink", 17014072853663087232, 0x69cbfdb672be4876),
    ("heat_sink", 9145234564587447566, 0xc758d94b7018f6d2),
    ("heat_sink", 6296479954471476984, 0x6974e1dce856c618),
    ("heat_sink", 12264833463194966517, 0x6125231ce917f4fe),
    ("heat_sink", 7823242965927508360, 0x5ce09f6ebd8cc3df),
    ("kitchen_sink", 6324056852682960335, 0x702ddb60fbd8c265),
    ("die_pips", 14934167376393906020, 0xb1ffd78a3718bf3a),
    ("part_select", 1219802557420740300, 0x83f81c93b22c64eb),
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
