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
///
/// **Re-pinned for the identity bundle** (FORK-REST): a copy standing
/// where its body stands is placed by no mate and the constant `0` on
/// every freedom, written as one `AddValues`, which mints a different number of ids
/// from the plane mate and three values it replaces, so every node
/// minted after a stand moved its id, and this digest feeds ids: the
/// placements after a stand in `heat_sink`, `declared_tangency`,
/// `kitchen_sink`, `cut_cylinder` and `part_select`, whose ordinals moved.
/// Every standing copy is bit-identical to its source.
///
/// **Re-pinned for exact turns** (INTENT stage 3 D): a turn written as a
/// fraction of a turn takes its sine and cosine from the fraction, so a
/// revolve whose heading is `turn/4` has its axis on its profile's `y`
/// exactly, and the placements of such a revolve, or of a pose spun by
/// `turn/4`, carry exact turns. `die_tool`'s two rows moved by that
/// alone. The writers that mean an axis-aligned frame state it: the
/// pips of `die_pips` and `die_composed` (two rows each) and of the
/// tour die (22) stand on lines stated from the cube's corner, and the
/// kitchen sink's revolve stands upright by a bundle with no mate spun
/// `turn/4`, its split's plane and its pattern's axis read off its faces
/// (nine rows). Every other row held, across main's merge at
/// `557e581a64` too.
///
/// `die_tool`'s two rows moved again when its master ball came to
/// stand on the cube's corner frame by an axis mate whose slide and
/// spin are the constant `0`, where `place_against` had turned it by
/// `atan2`'s rounded quarter turns. Its explicit rule's frames are now
/// built from the fraction of a turn each is
/// (`Frame::rotate_by_turns_then_translate`). The rows are the ball's
/// copy and the product. The product's ordinal fell from 101 to 59:
/// the master's placement mints 42 fewer ids than `place_against`'s
/// did.
///
/// `tube_arc`'s and `hollow_tube_elbow`'s rows moved when each tube,
/// built about its own `z`, came to stand about the world's `+y` again
/// as its retired spine frame had it: a bundle with no mate spun
/// `−turn/4` about `x`, so its world copy is where it stood before D.
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
    ("heat_sink", 157, 0x6519f38af27728a2),
    ("heat_sink", 174, 0xdb9ce54b22fc3bdf),
    ("heat_sink", 191, 0x151fccc34c36966e),
    ("heat_sink", 208, 0xa6aa8944480d3fc9),
    ("heat_sink", 225, 0xd8557bc183d8d5d2),
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
    ("declared_tangency", 72, 0x649e278463aeac31),
    ("kitchen_sink", 55, 0x81164dd184014339),
    ("kitchen_sink", 79, 0x1c3f92b6b0bea147),
    ("kitchen_sink", 144, 0x134a9422371eccdd),
    ("kitchen_sink", 160, 0xa09e4da0f77f9429),
    ("kitchen_sink", 173, 0x0474d4565c93d5e3),
    ("kitchen_sink", 187, 0xf67752f3182120e6),
    ("kitchen_sink", 204, 0x221c396cd531c0f2),
    ("kitchen_sink", 221, 0x6613cd214ec834c4),
    ("kitchen_sink", 238, 0x608bb95e5bb420d8),
    ("kitchen_sink", 252, 0x21fedfe14f7bac56),
    ("cut_cylinder", 38, 0x15a3fea54fe3b8ad),
    ("cut_cylinder", 52, 0xce579141ff4b8726),
    ("measured_web", 36, 0xa5bbabae356adcca),
    ("measured_web", 56, 0x5d19d653bc1dae6e),
    ("boss_union", 29, 0x23bc53fbff8503fc),
    ("boss_union", 48, 0x8da75c0cefa2f056),
    ("die_fillet", 23, 0xaee8dc9b01474e9e),
    ("die_chamfer", 23, 0x2fb1046258e6be88),
    ("die_pips", 36, 0x9799e9408a965673),
    ("die_pips", 85, 0xabdcdd68d6103dcc),
    ("heat_sink_fins", 27, 0xb641d0c4ac52804e),
    ("die_tool", 36, 0x3ff90e3ab6223307),
    ("die_tool", 59, 0xe1197d85a85e8d88),
    ("face_sketch", 37, 0x7786f1eb617ac49b),
    ("face_sketch", 53, 0xd5e58b4c5b896c38),
    ("part_select", 42, 0x82790a172f72402b),
    ("part_select", 56, 0xd6c1e39940b5f496),
    ("loft_prism", 51, 0x7b39a360638cd8a1),
    ("die_composed", 36, 0x9799e9408a965673),
    ("die_composed", 89, 0x4bca5573dec89b94),
    ("die_composed_tour", 40, 0xa6ebce75136e3f6c),
    ("die_composed_tour", 101, 0x3cfecee20be34e65),
    ("die_composed_tour", 176, 0x298467248d835200),
    ("die_composed_tour", 251, 0xe42829bb9f30f475),
    ("die_composed_tour", 326, 0xda0ea6e70a2c7e79),
    ("die_composed_tour", 401, 0x676cf08c5458dfca),
    ("die_composed_tour", 476, 0xf32a0f21c91d27b1),
    ("die_composed_tour", 551, 0xb099dfebc9bc425e),
    ("die_composed_tour", 626, 0xc86bbbd6130f79b1),
    ("die_composed_tour", 701, 0xc4165615472b34d8),
    ("die_composed_tour", 776, 0xdd0d89f19bb05a89),
    ("die_composed_tour", 851, 0x1b18a7ca52bc0351),
    ("die_composed_tour", 926, 0xe9dc547d60f518df),
    ("die_composed_tour", 1001, 0xbb2dd41e075deea7),
    ("die_composed_tour", 1076, 0x7772470d20d26dec),
    ("die_composed_tour", 1151, 0x2a03587052a71a2b),
    ("die_composed_tour", 1226, 0xeb45494e859ee144),
    ("die_composed_tour", 1301, 0x195d14ca4037e882),
    ("die_composed_tour", 1376, 0xe8227e6c4f410265),
    ("die_composed_tour", 1451, 0x6b1a01b34ed6d881),
    ("die_composed_tour", 1526, 0xa264fe57d2a657e2),
    ("die_composed_tour", 1613, 0x06f090e809052b4d),
    ("plate_param", 44, 0xb131b90c7283e469),
    ("plate_param", 63, 0x778c16d8f77b1175),
    ("kiss_carry", 37, 0x5327fb974bfe45c1),
    ("kiss_carry", 74, 0x21f1f9dbf876d243),
    ("kiss_carry", 93, 0xf9d9270ccbfdcf21),
    ("tube_ring", 5, 0x28708aadf200c24a),
    ("tube_arc", 7, 0x78b0777814e2560d),
    ("hollow_tube_elbow", 8, 0x34e28ea09a5238fe),
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
