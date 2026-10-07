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
///
/// **Re-pinned for INTENT-LITERALS PR D** (`Expr` holds no float):
/// `kitchen_sink` alone, whose formulas hold written quantities that
/// now mint variables of their own; every other row held its word.
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 7737271460520352144, 0xe80400cf98834e8a),
    ("die", 17378071859743756397, 0x0ca5a266b8fb4528),
    ("die", 721692123010527227, 0x3a0dae2b13f2d216),
    ("die", 14971010633686759539, 0xe6d650f97a27a2e0),
    ("die", 14052667631799044213, 0x90de49f35944d52a),
    ("die", 1311043390687188205, 0xa6f465265658a511),
    ("die", 17515496266726983249, 0x61b6164116b15b59),
    ("die", 15243212580900555671, 0x540000099c4dc7c6),
    ("die", 3387376929605905626, 0xe8d7e3da1519ac63),
    ("die", 13345870966724836724, 0xd950372607955122),
    ("die", 13813147807747047695, 0x38c13ff4da8bceee),
    ("die", 15993098211746638895, 0xa40aae30682cd6e1),
    ("die", 7791515320851863244, 0x68c28daa7a6c3184),
    ("die", 11319635659781039511, 0xaa11935fcbfdf4a8),
    ("die", 11489815378188868632, 0xf6577309fc3143e0),
    ("die", 10844839672210544606, 0x5c53ee03dce6778f),
    ("die", 7309163247844024614, 0x34add9124d6449ac),
    ("die", 13379572687054000213, 0x29d83765cddaf123),
    ("die", 4919800054970687188, 0xe69aac4fac6aa42d),
    ("die", 2910535887588818879, 0xb31383975e68e1e7),
    ("die", 3378863270133240766, 0x1605740be9499566),
    ("heat_sink", 11865892429783732516, 0xa9b6150dd1fc2ef4),
    ("heat_sink", 16783533055005839466, 0x3fe976906e925f90),
    ("heat_sink", 5378097503098638365, 0x53f657e4f6c62101),
    ("heat_sink", 4217065770962140755, 0x8d91d0b1081ae001),
    ("heat_sink", 6728457609622360334, 0x7ff3479ca4d55648),
    ("kitchen_sink", 1002884793474252668, 0x1545911d0f1bbf51),
    ("die_pips", 3616840321126588636, 0x8ace02c55b153ded),
    ("part_select", 9397978846600865866, 0xa6a76d525e40e428),
    ("die_composed", 3616840321126588636, 0x8ace02c55b153ded),
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
