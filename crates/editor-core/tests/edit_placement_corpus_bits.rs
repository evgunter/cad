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
    ("die", 16124493075523318136, 0x20dcec922512e721),
    ("die", 15974345893496838687, 0x2b5e8535b5928129),
    ("die", 8713799178238569587, 0x40f15921f4aa9fd9),
    ("die", 14069951386207725068, 0x4b6172b00699231f),
    ("die", 9341519010300021978, 0xe0504c65f913c734),
    ("die", 6657832975466258974, 0x6d96e0650ba4ac8e),
    ("die", 13880924209827505905, 0x1bf26692595ef5d2),
    ("die", 10745648360525432012, 0xea8b398dcd8c1094),
    ("die", 13063684928208796875, 0x5c8ab1c0ded6d1a9),
    ("die", 5134244361962535213, 0x5e0499cef23fd1d7),
    ("die", 13477449286911829538, 0x80721406edfa429b),
    ("die", 1312867628974718537, 0xc177abe7e5c5d348),
    ("die", 5073302808294264834, 0xa5405e060775b081),
    ("die", 16414274619086927742, 0xf3d1c9f933d013bd),
    ("die", 6510121927585596402, 0x14306374c575a279),
    ("die", 6688391033139309776, 0xe7491c1de4243451),
    ("die", 7037305691044043822, 0xc5bd3ee8a5a2317f),
    ("die", 17005134172494295408, 0xbff05e2e708045c7),
    ("die", 3084123221246772465, 0x9287b620733dbb21),
    ("die", 9389799721170207125, 0xefd0d14caf312fa2),
    ("die", 8103816150378695923, 0x5ab6af8fc1f2852a),
    ("heat_sink", 11235307439961258166, 0x6529b23ba091437f),
    ("heat_sink", 15542140952337107891, 0x8efb41c4380f0f0a),
    ("heat_sink", 20554895330027894, 0x60d598c3d6e13dcc),
    ("heat_sink", 651828539818508088, 0xc2ba04c48da2e06f),
    ("heat_sink", 3498821325472730611, 0xac9344d5ed00516b),
    ("kitchen_sink", 5723596197250949099, 0x87bd1b4a7509c035),
    ("die_pips", 14934167376393906020, 0xb1ffd78a3718bf3a),
    ("part_select", 14808078435937784354, 0xa5cc2cbb0aaa7a98),
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
