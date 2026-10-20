//! **Every corpus placement places its body by the same bits** — the
//! value channel of each `Node::Place` in the registry, pinned per node.
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

/// `(document, node's mint ordinal, digest)` for every placement the
/// registry holds, in registry then document order, the same at the
/// default, `1e-6` and `1e-12` ε rows. The digest feeds the node id, so
/// a re-minted id moves the word with no geometry moving; so does a
/// point landing in another arena slot.
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
/// **Re-pinned when an id became its mint ordinal and its digest**: a
/// row names its node by ordinal, and every word moved with the id it
/// hashes; the ids-masked geometry fence held.
///
/// **Re-pinned when a selection became a variable**: the tour document
/// mints one selection ahead of its transforms, so each of its rows'
/// ordinals moved by one and its words with them; every other row held,
/// and so did the ids-masked geometry fence.
///
/// **Re-pinned for INTENT stage 3 PR A** (a revolve's axis is a line on
/// the node): `kitchen_sink`, `die_pips`, `die_composed` and the tour
/// document hold no in-plane axis datum, so their later ids moved —
/// ordinals by two where the datum and its frame read went — and every
/// row naming them; every other row held its word, and the ids-masked
/// geometry fence held. Taken merged over FORK-DM4 unit 1 (PR 4527).
///
/// **Re-pinned for FORK-DM4 unit 2** (a name is scoped by the variable
/// holding its body): `part_select`'s row alone. Its union declares the
/// rest between the split's halves by the section face, which no longer
/// spells the half, so the union and every node minted after it took
/// new ids; the transform's word hashes its id. The id-masked geometry
/// fence held.
///
/// **Re-pinned for INTENT stage 3 D** (Transform retires into a
/// placement): a rigid motion of a body that was a `Transform` is a
/// `Place`, and a construction that stood on a moved datum frame is
/// built in its own coordinates and placed where the frame stood, so the
/// registry holds 118 placements where it held 51 and every document
/// has rows. The documents were re-spelled without `Datum` or
/// `Transform`, so every id moved and every word with it.
///
/// **Re-pinned merging main into D**: `declared_tangency`'s placement
/// moved with main's chord director (its components, not `atan2`); the
/// kitchen sink's eight with its lone block's copies hung below the
/// world's xy as its retired `SetExtrudeSide` edit had them (placed at
/// a pose, which mints one id fewer than standing, so the ordinals after
/// them moved by one); `corner_table`'s
/// one with its union's arena order under main's union fold, its points
/// at their exact values. Every other row held.
const PINNED: &[(&str, u32, u64)] = &[
    ("die", 122, 0xedc88b0c9768a183),
    ("die", 137, 0x3b18c1950c918b57),
    ("die", 152, 0x63812ab142fdb33b),
    ("die", 167, 0x752236e951e8d166),
    ("die", 182, 0xce61383f419e19a0),
    ("die", 197, 0xf8988f096b4ac0c3),
    ("die", 212, 0xfc85e4e82783110e),
    ("die", 265, 0x3d1a619fec2796f2),
    ("die", 318, 0xe23e2eea32c67f67),
    ("die", 413, 0xc6677290692adc1f),
    ("die", 508, 0x9ae5ed143907df05),
    ("die", 603, 0x14f14caa34f108f4),
    ("die", 698, 0x6ac695d01e8e955e),
    ("die", 793, 0x22449ee2ec425e28),
    ("die", 888, 0x6b9baaf0412f4cc6),
    ("die", 983, 0xc3d6c3b5776da00c),
    ("die", 1078, 0x866932a5d58dea40),
    ("die", 1145, 0x6def74aef724f900),
    ("die", 1212, 0x48484b3ec98c1577),
    ("die", 1279, 0x3ea4b8a2bf1015fe),
    ("die", 1346, 0xb6b6fd692089d187),
    ("die", 1366, 0xd1ffac72b054e934),
    ("corner_table", 37, 0x56f145c8b37bfabf),
    ("corner_table", 74, 0xa47b8bb17613453b),
    ("corner_table", 111, 0x2f6fbc248977629d),
    ("corner_table", 148, 0xecbb08f6357cbe88),
    ("corner_table", 167, 0xbc01ca62da96a73b),
    ("heat_sink", 45, 0x59eb201168d53e62),
    ("heat_sink", 64, 0x6feed66357f7b1f9),
    ("heat_sink", 83, 0x844acb67c35c5496),
    ("heat_sink", 102, 0xde9f81c590c8c25f),
    ("heat_sink", 121, 0x89c3ef16b6713708),
    ("heat_sink", 140, 0x061244a94671bfea),
    ("heat_sink", 155, 0x0558f65f45d794e1),
    ("heat_sink", 172, 0xce5fccdf45b1231c),
    ("heat_sink", 189, 0x5594dced2c6a56e1),
    ("heat_sink", 206, 0xf9b6f3020c19795e),
    ("heat_sink", 223, 0x8bf4ab713219cf96),
    ("crossing_slots", 37, 0xb33e8149db30dd57),
    ("crossing_slots", 74, 0x532a33f7f1dbae57),
    ("crossing_slots", 93, 0x02461850bc4c912d),
    ("nested_islands_105", 55, 0x88e7f1ef719a5cf8),
    ("nested_islands_105", 74, 0x7a6913ba24a39ec8),
    ("nested_islands_105", 111, 0xb8efb01e9a0de104),
    ("nested_islands_105", 130, 0x4c418b25a00ecb5f),
    ("nested_islands_106_depth1", 55, 0x88e7f1ef719a5cf8),
    ("nested_islands_106_depth1", 74, 0x7a6913ba24a39ec8),
    ("nested_islands_106_depth1", 111, 0x97bbeb3fe3e04923),
    ("nested_islands_106_depth1", 130, 0x800549462906b820),
    ("nested_islands_106_depth2", 55, 0x88e7f1ef719a5cf8),
    ("nested_islands_106_depth2", 74, 0x7a6913ba24a39ec8),
    ("nested_islands_106_depth2", 111, 0xb8efb01e9a0de104),
    ("nested_islands_106_depth2", 148, 0xa7a414ba8b756515),
    ("nested_islands_106_depth2", 167, 0xe3280332ac0be665),
    ("declared_tangency", 58, 0x7e78a08f2a744c2f),
    ("declared_tangency", 70, 0xc73761c9b161c710),
    ("kitchen_sink", 55, 0x81164dd184014339),
    ("kitchen_sink", 99, 0x8b388459d09064ab),
    ("kitchen_sink", 180, 0xad33562c97bbee17),
    ("kitchen_sink", 196, 0x54439491846c821b),
    ("kitchen_sink", 211, 0x3440a00ca81668f3),
    ("kitchen_sink", 225, 0xef65c21d15d5ffb3),
    ("kitchen_sink", 240, 0x9ae6404d2f954552),
    ("kitchen_sink", 257, 0x8f7d24b7aa7142c2),
    ("kitchen_sink", 274, 0xcec20ceb3220531b),
    ("kitchen_sink", 288, 0x7f7bb9fc49a72069),
    ("cut_cylinder", 38, 0x15a3fea54fe3b8ad),
    ("cut_cylinder", 50, 0x71f9ca7767225a01),
    ("measured_web", 36, 0xa5bbabae356adcca),
    ("measured_web", 56, 0x5d19d653bc1dae6e),
    ("boss_union", 29, 0x23bc53fbff8503fc),
    ("boss_union", 48, 0x8da75c0cefa2f056),
    ("die_fillet", 23, 0xaee8dc9b01474e9e),
    ("die_chamfer", 23, 0x2fb1046258e6be88),
    ("die_pips", 36, 0x7fffc8bf1d01dcef),
    ("die_pips", 127, 0x716fd38f925cf753),
    ("heat_sink_fins", 27, 0xb641d0c4ac52804e),
    ("die_tool", 36, 0xe3f2cb363a5ebe7c),
    ("die_tool", 101, 0x085fd31fa66d7f4f),
    ("face_sketch", 37, 0x7786f1eb617ac49b),
    ("face_sketch", 53, 0xd5e58b4c5b896c38),
    ("part_select", 42, 0x82790a172f72402b),
    ("part_select", 54, 0x8a691aaa7d6b2df0),
    ("loft_prism", 51, 0x7b39a360638cd8a1),
    ("die_composed", 36, 0x7fffc8bf1d01dcef),
    ("die_composed", 131, 0xeecdcffddc410554),
    ("die_composed_tour", 40, 0x3c9c5235605a2430),
    ("die_composed_tour", 129, 0xf3dbcf23016794e0),
    ("die_composed_tour", 218, 0x8dc864c8f1079182),
    ("die_composed_tour", 307, 0x2999d534f8bc29a8),
    ("die_composed_tour", 396, 0xab435d31fcb86d78),
    ("die_composed_tour", 485, 0x0c501f86f440d690),
    ("die_composed_tour", 574, 0x3dd96f50171d7cff),
    ("die_composed_tour", 663, 0x5bd88bc8c9490a7a),
    ("die_composed_tour", 738, 0x2b02057eecc348b9),
    ("die_composed_tour", 813, 0x855f4161fbe84373),
    ("die_composed_tour", 916, 0xab7d55c1c1738e78),
    ("die_composed_tour", 1019, 0xc04b5a409a27f3e4),
    ("die_composed_tour", 1122, 0x6c853e3968bd849d),
    ("die_composed_tour", 1225, 0x67e66154307e23c6),
    ("die_composed_tour", 1328, 0x4eec31ac2fa7b2c4),
    ("die_composed_tour", 1417, 0x1cd24d39f0a5096d),
    ("die_composed_tour", 1506, 0x1d93f1e5f179f538),
    ("die_composed_tour", 1595, 0xa3ea0b03525d7420),
    ("die_composed_tour", 1684, 0xa7c191c9d0b0353d),
    ("die_composed_tour", 1773, 0x717b58958454f3d8),
    ("die_composed_tour", 1862, 0x1285c0956f81483a),
    ("die_composed_tour", 1963, 0xe61bac2c922a6477),
    ("plate_param", 44, 0xb131b90c7283e469),
    ("plate_param", 63, 0x778c16d8f77b1175),
    ("kiss_carry", 37, 0x5327fb974bfe45c1),
    ("kiss_carry", 74, 0x21f1f9dbf876d243),
    ("kiss_carry", 93, 0xf9d9270ccbfdcf21),
    ("tube_ring", 5, 0x28708aadf200c24a),
    ("tube_arc", 7, 0xda5de013096cb80d),
    ("hollow_tube_elbow", 8, 0x195ecd4525de4b1e),
    ("hollow_tube_ring", 6, 0xc17178d9cde4a81e),
    ("reshaped_rod", 48, 0x6713d9dd15ba1cb2),
];

#[test]
fn every_corpus_placement_places_its_body_by_the_pinned_bits() {
    let mut got: Vec<(&'static str, u32, u64)> = Vec::new();
    for doc in corpus::documents() {
        let placements: Vec<_> = doc
            .doc
            .ids()
            .iter()
            .copied()
            .filter(|id| matches!(doc.doc.node(*id), Some(Node::Place { .. })))
            .collect();
        if placements.is_empty() {
            continue;
        }
        let ev = eval::<f64>(&doc.doc);
        let bad = failures(&ev);
        assert!(bad.is_empty(), "{} failed to evaluate: {bad:?}", doc.name);
        for (id, word, fed) in value_digest_nodes(&ev) {
            if placements.contains(&id) {
                assert!(fed > 0, "{} node {}: nothing was digested", doc.name, id);
                got.push((doc.name, id.0.ordinal(), word));
            }
        }
    }
    for (name, id, word) in &got {
        println!("    (\"{name}\", {id}, {word:#018x}),");
    }
    assert_eq!(
        got.as_slice(),
        PINNED,
        "a corpus placement's placed body moved"
    );
}
