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
///
/// **Re-pinned for INTENT-LITERALS PR C** (a slot holds a variable):
/// every node is minted from slots that hold variable ids, a typed
/// value's variable drawn from what it holds, so every node id moved
/// and with it every row this hashes. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held untouched across the change.
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
        14934969119361055721,
        0xadfb5bdc342c883a,
    ),
    (
        "die_composed_tour",
        12321946086614061860,
        0x51accfbf63acd9ec,
    ),
    ("die_composed_tour", 1607595739630803563, 0x36c5b0dc980939e7),
    (
        "die_composed_tour",
        12314759081092211246,
        0x0c2a08a4c37ded1b,
    ),
    ("die_composed_tour", 3796523877164864820, 0xfcf5fdb10223fc58),
    (
        "die_composed_tour",
        16354879676066552213,
        0x37d79bdbaa63cc6f,
    ),
    ("die_composed_tour", 1744719274560299663, 0x479d4f3e38d0b028),
    (
        "die_composed_tour",
        17307922903490645166,
        0x265a245ede45b6b8,
    ),
    (
        "die_composed_tour",
        10251832615738266467,
        0xe78d23e2043503c1,
    ),
    (
        "die_composed_tour",
        12913916506033112012,
        0x71a6644ccc9d103b,
    ),
    (
        "die_composed_tour",
        10891647831853294135,
        0xb12b8971dd3e52ad,
    ),
    ("die_composed_tour", 92835628372886175, 0xfc53f654b574e039),
    ("die_composed_tour", 849907914837897822, 0xd15a458df062c5fe),
    (
        "die_composed_tour",
        16188805933911949637,
        0x55efb3eb36e0800d,
    ),
    ("die_composed_tour", 2043580579064484645, 0x312837419a8ab310),
    (
        "die_composed_tour",
        10687756545180916313,
        0xab9273b94ebb7270,
    ),
    (
        "die_composed_tour",
        18252633885557477507,
        0xcbd1647b5eed42a5,
    ),
    ("die_composed_tour", 7756297561168519233, 0x89c390fcbedb300c),
    ("die_composed_tour", 578548427347514355, 0x099379621baeea2f),
    ("die_composed_tour", 7684283349274090191, 0x42b1bac52104ac26),
    ("die_composed_tour", 4091348359908227239, 0x3bacd76869f7dd40),
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
