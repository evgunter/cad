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
/// spin are the constant `0`, where the retired place-against helper had turned it by
/// `atan2`'s rounded quarter turns. Its explicit rule's frames are now
/// built from the fraction of a turn each is
/// (`Frame::rotate_by_turns_then_translate`). The rows are the ball's
/// copy and the product. The product's ordinal fell from 101 to 59:
/// the master's placement mints 42 fewer ids than the retired place-against helper's
/// did.
///
/// `tube_arc`'s and `hollow_tube_elbow`'s rows moved when each tube,
/// built about its own `z`, came to stand about the world's `+y` again
/// as its retired spine frame had it: a bundle with no mate spun
/// `−turn/4` about `x`, so its world copy is where it stood before D.
///
/// Merging main at `be82e5ad6b` into D moved `die_composed`'s row and
/// the tour document's last: each is the world placement of the
/// filleted die, minted after the fillet, whose stored selection names
/// the pip rim by seam sides carried in through their reads (N3,
/// main's); the fillet's id moved with its payload, and the
/// placement's with it. Neither tree moved these words alone; every
/// other row held.
///
/// Re-pinned, `die` alone, when its pips came to stand on frames stated
/// from the cube's corner by whole quarter turns and dyadic slides (a
/// frame mate each, `fixture::square_frame`), where the retired place-against helper had
/// written `atan2`'s rounded turns: the die's world vertices are the
/// merge base's (`f79829b57d`) bit for bit, 176 of 176, and its
/// placements, their ids and the names under them are new.
///
/// Re-pinned for D′1 (a placement names its target space): each
/// construction defines its space beside its body and every bundle is
/// written whole, so ids, and the names spelled with them, moved in
/// every document; `m10_p_fence`'s ids-masked geometry fence held.
///
/// **Re-pinned for PR 5616** (a mirror's seam is the plane's face read
/// twice), taken merged over D′2 (PR 5613, mates read geometry), which
/// left this row red on main: D′2 moved every corpus document's ids,
/// and this PR's `test_support::plane_on` reads `symmetric_plate`'s
/// seam face plainly, so its ids moved too. Ids only:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held unmoved.
const PINNED: &[(&str, u32, u64)] = &[
    ("die", 160, 0x7a418db6ed3af4d3),
    ("die", 196, 0xee8cd1d99dc4ae36),
    ("die", 232, 0x96b12f961d2f0ec4),
    ("die", 268, 0x20adeb643e9e7b5b),
    ("die", 304, 0xf35414d217233519),
    ("die", 340, 0x7ab05749307150f6),
    ("die", 390, 0x3492f1391f2115a8),
    ("die", 440, 0x03668706dc37e793),
    ("die", 483, 0xf52f42abf425360b),
    ("die", 526, 0xcdbc7966b7d06d13),
    ("die", 569, 0xfff7ee3d2d1e241f),
    ("die", 612, 0x64aa9037d9b574a0),
    ("die", 655, 0xae4d7ec18f651b5f),
    ("die", 705, 0x4a4dd79046bf42cd),
    ("die", 755, 0xc6d11d227ff848dc),
    ("die", 805, 0x7e8b2daaf61a975f),
    ("die", 841, 0x320416652727d518),
    ("die", 877, 0xd3c8d9f3cab11f44),
    ("die", 913, 0x7586108305acccac),
    ("die", 949, 0xa583ce4601b62b29),
    ("die", 992, 0xdd389f1e0bd7bb8f),
    ("die", 1003, 0xa45a2ae3ea716d8f),
    ("corner_table", 50, 0xf77b6d2c94949da8),
    ("corner_table", 88, 0x95d65c6ec1ec735e),
    ("corner_table", 126, 0x5d177cfe352f6c6f),
    ("corner_table", 164, 0x1d7276f883545a68),
    ("corner_table", 178, 0x84d35dabb901add8),
    ("heat_sink", 58, 0x38cd1f550704f664),
    ("heat_sink", 77, 0x78e9146f9340667d),
    ("heat_sink", 96, 0x656b55e79349779d),
    ("heat_sink", 115, 0x4bce729014f687b5),
    ("heat_sink", 134, 0x435f32fc4b9e7fe7),
    ("heat_sink", 148, 0x526259c2c7a91f94),
    ("heat_sink", 165, 0xd736b428d49d2f76),
    ("heat_sink", 182, 0x63253e7cf55debe0),
    ("heat_sink", 199, 0x1ed81e91ccfb8562),
    ("heat_sink", 216, 0xe8e4b42e135d2850),
    ("heat_sink", 233, 0xc42a28de45d8976c),
    ("crossing_slots", 50, 0x0e1c70565d55c1f5),
    ("crossing_slots", 88, 0x871706fb5fa5d7ef),
    ("crossing_slots", 102, 0x28f40bcc1f915ab5),
    ("nested_islands_105", 69, 0x2290c7ada628d094),
    ("nested_islands_105", 88, 0xe855dcda5086c401),
    ("nested_islands_105", 126, 0x556a2d0889e19e1a),
    ("nested_islands_105", 140, 0x5c9c95570ace9075),
    ("nested_islands_106_depth1", 69, 0x2290c7ada628d094),
    ("nested_islands_106_depth1", 88, 0xe855dcda5086c401),
    ("nested_islands_106_depth1", 126, 0xd7d8ddfbe1c7d1ac),
    ("nested_islands_106_depth1", 140, 0xc23a4da4407c2b68),
    ("nested_islands_106_depth2", 69, 0x2290c7ada628d094),
    ("nested_islands_106_depth2", 88, 0xe855dcda5086c401),
    ("nested_islands_106_depth2", 126, 0x556a2d0889e19e1a),
    ("nested_islands_106_depth2", 164, 0xf983297c52c3442c),
    ("nested_islands_106_depth2", 178, 0x1382df4ec72cc02c),
    ("declared_tangency", 66, 0x0ffe4e74b205caa7),
    ("declared_tangency", 80, 0xaca0c95868276903),
    ("kitchen_sink", 64, 0xb335d6519f0a53cd),
    ("kitchen_sink", 88, 0x4cc482412ed6ecb6),
    ("kitchen_sink", 154, 0x1554c30d1157a3ab),
    ("kitchen_sink", 171, 0xd1fd153f6339bdb1),
    ("kitchen_sink", 185, 0x92808eedcfd41991),
    ("kitchen_sink", 199, 0x3a5dbc07a4458e10),
    ("kitchen_sink", 216, 0x898bf86ae6d2a640),
    ("kitchen_sink", 233, 0x1c57c9a887162282),
    ("kitchen_sink", 250, 0xebb4a065eb6dafe3),
    ("kitchen_sink", 264, 0xdd794aba8c9e3e0d),
    ("cut_cylinder", 45, 0x2d323075ffb3ef6b),
    ("cut_cylinder", 59, 0x2372040a2a3953ed),
    ("measured_web", 46, 0xaa3af5fc6b752068),
    ("measured_web", 65, 0x600af1ccf8d3e908),
    ("boss_union", 42, 0xf652865bea3769ed),
    ("boss_union", 56, 0xe8541e2df6d7699f),
    ("die_fillet", 30, 0x97017ef426053510),
    ("die_chamfer", 30, 0x24b66a82e2fbeb95),
    ("die_pips", 80, 0x07288413724c102a),
    ("die_pips", 93, 0x5f71fcf25fc4f3f2),
    ("heat_sink_fins", 34, 0xa2d439d99f78315c),
    ("die_tool", 52, 0x24487c898c7e3061),
    ("die_tool", 67, 0xb7645becb31601bd),
    ("face_sketch", 45, 0x20e419ab4b4c778d),
    ("face_sketch", 73, 0xe949535e414f3dc8),
    ("part_select", 49, 0x15dee26ec97362f9),
    ("part_select", 63, 0xcacec981969db428),
    ("symmetric_plate", 38, 0x9df0413c16d73fd7),
    ("loft_prism", 58, 0x4f99387925deaa74),
    ("die_composed", 80, 0x07288413724c102a),
    ("die_composed", 97, 0x1a16b746c4989715),
    ("die_composed_tour", 98, 0xdf4dc184476e760b),
    ("die_composed_tour", 173, 0x40676d344a43663a),
    ("die_composed_tour", 248, 0xbc5efef4dd3951ee),
    ("die_composed_tour", 323, 0xf2c7a403590abaab),
    ("die_composed_tour", 398, 0xb7988b63e673f1f6),
    ("die_composed_tour", 473, 0xdc198471f5d1f695),
    ("die_composed_tour", 548, 0xc23208d6ec3784ae),
    ("die_composed_tour", 623, 0x583b064e06bd2460),
    ("die_composed_tour", 698, 0x3dc35f4d7dbda4f4),
    ("die_composed_tour", 773, 0x6ee023496597ca7f),
    ("die_composed_tour", 848, 0x17c1a563df000d82),
    ("die_composed_tour", 923, 0x56a476e06b869959),
    ("die_composed_tour", 998, 0xf09fd67adf130b26),
    ("die_composed_tour", 1073, 0x86d2a1fe41a9e650),
    ("die_composed_tour", 1148, 0xe06c681d7f6944e5),
    ("die_composed_tour", 1223, 0x5006d1fe44457dd6),
    ("die_composed_tour", 1298, 0xf94948f5eb8699c0),
    ("die_composed_tour", 1373, 0xe659f62883e7c569),
    ("die_composed_tour", 1448, 0xc07596fde4502d13),
    ("die_composed_tour", 1523, 0x5369be78ad74595c),
    ("die_composed_tour", 1598, 0xbbacc0da9a1b1158),
    ("die_composed_tour", 1621, 0xf91c539578f54d17),
    ("plate_param", 57, 0x0a2240ab7af16e1b),
    ("plate_param", 71, 0xf8defd3d2d56bb30),
    ("kiss_carry", 50, 0xd6168a6437214201),
    ("kiss_carry", 88, 0x7703545862557737),
    ("kiss_carry", 102, 0xbeedf13faeb31ef3),
    ("tube_ring", 12, 0x86c62dc1f4635d8c),
    ("tube_arc", 14, 0x0579833020a53684),
    ("hollow_tube_elbow", 15, 0x71484ff9db120ecf),
    ("hollow_tube_ring", 13, 0x0d8427bce16bc974),
    ("reshaped_rod", 55, 0xa30afccc4fc02604),
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
