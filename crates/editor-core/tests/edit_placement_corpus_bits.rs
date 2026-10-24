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
///
/// **Re-pinned when a plane pose came to hold its normal and offset
/// alone**: `cut_cylinder`'s two rows alone. Its tool is a tilted plane,
/// and a split reads the plane's point nearest the origin rather than
/// the face's stored point, so its halves' section points moved at the
/// rounding level; every other row held its word.
const PINNED: &[(&str, u32, u64)] = &[
    ("die", 160, 0xda684850d9e84843),
    ("die", 196, 0x400ba696eb63b364),
    ("die", 232, 0xc00f3a398da762a4),
    ("die", 268, 0xb59cf4c874d80c3b),
    ("die", 304, 0x9188e5938dbbd86e),
    ("die", 340, 0x5a836b25ac9a8d64),
    ("die", 390, 0x5cdb51eb74edd571),
    ("die", 440, 0x1f471a0741ecd894),
    ("die", 483, 0xeeac29a7f3f2c11b),
    ("die", 526, 0xd02383aa33213d49),
    ("die", 569, 0xf806f6bbefab58a0),
    ("die", 612, 0x447c791d639dd950),
    ("die", 655, 0x362d3e8f4082315d),
    ("die", 705, 0x0f3b625bbd8e22ed),
    ("die", 755, 0xc1dcb30fe9cb7f70),
    ("die", 805, 0x3492147a18d88d64),
    ("die", 841, 0x8df5bde7bf89b7a5),
    ("die", 877, 0x08a3781fb2911374),
    ("die", 913, 0x85e0574ce3d0e59a),
    ("die", 949, 0x15f2562a9d666f0c),
    ("die", 992, 0xdb0c743390023f83),
    ("die", 1003, 0xd5a4a39edf93865d),
    ("corner_table", 50, 0x5363c160ea615d81),
    ("corner_table", 88, 0x7685fc25f30b242f),
    ("corner_table", 126, 0x67fe3f2ced918fe5),
    ("corner_table", 164, 0xc831b8ff93cae9e5),
    ("corner_table", 178, 0x40aee605c55ce386),
    ("heat_sink", 58, 0x53e981bf35379479),
    ("heat_sink", 77, 0x35c4dd23ad0f058a),
    ("heat_sink", 96, 0x4060da0d05861142),
    ("heat_sink", 115, 0x025d8d3ec4cac274),
    ("heat_sink", 134, 0xc775af31dcaeb133),
    ("heat_sink", 148, 0xfcac9f778f5e1ba0),
    ("heat_sink", 165, 0x9e4d1d8fa9d102f7),
    ("heat_sink", 182, 0x7df2865401bc1049),
    ("heat_sink", 199, 0x6da888c5e08915d4),
    ("heat_sink", 216, 0x735749215b47adee),
    ("heat_sink", 233, 0x9ba8160739283a30),
    ("crossing_slots", 50, 0x7fd1578b2c26cfdf),
    ("crossing_slots", 88, 0x28b54fe0a94e7634),
    ("crossing_slots", 102, 0x6c070664fb32f08a),
    ("nested_islands_105", 69, 0x5018aab606ad399a),
    ("nested_islands_105", 88, 0x43832d225a9b64f9),
    ("nested_islands_105", 126, 0x08040c5f4289c54b),
    ("nested_islands_105", 140, 0xbed7019e8b664e62),
    ("nested_islands_106_depth1", 69, 0x5018aab606ad399a),
    ("nested_islands_106_depth1", 88, 0x43832d225a9b64f9),
    ("nested_islands_106_depth1", 126, 0x05e43d17191df564),
    ("nested_islands_106_depth1", 140, 0x075e94ae32e0ad8c),
    ("nested_islands_106_depth2", 69, 0x5018aab606ad399a),
    ("nested_islands_106_depth2", 88, 0x43832d225a9b64f9),
    ("nested_islands_106_depth2", 126, 0x08040c5f4289c54b),
    ("nested_islands_106_depth2", 164, 0xf0e195e16ccc82c7),
    ("nested_islands_106_depth2", 178, 0x12f84b945867be02),
    ("declared_tangency", 66, 0x0ffe4e74b205caa7),
    ("declared_tangency", 80, 0xaca0c95868276903),
    ("kitchen_sink", 64, 0x3974a4eb836e8abd),
    ("kitchen_sink", 88, 0x2864daf7c9ffbed2),
    ("kitchen_sink", 154, 0xe1216491f96e74f7),
    ("kitchen_sink", 171, 0x8cb0fdde0baa4570),
    ("kitchen_sink", 185, 0x057a1d19c79ac7b1),
    ("kitchen_sink", 199, 0xefb0cc230551a556),
    ("kitchen_sink", 216, 0xf5aa9a72df7131c8),
    ("kitchen_sink", 233, 0x1f7abb8048a85712),
    ("kitchen_sink", 250, 0xd941f44e7597e122),
    ("kitchen_sink", 264, 0x7ee689567bff8c43),
    ("cut_cylinder", 45, 0x449cda974caa08ec),
    ("cut_cylinder", 59, 0x8d0ab69834e92862),
    ("measured_web", 46, 0xe8d3f6e2df174f06),
    ("measured_web", 65, 0x5beac086138b840f),
    ("boss_union", 42, 0x9365a51aaa4d36de),
    ("boss_union", 56, 0x5a878f9868cea6e1),
    ("die_fillet", 30, 0x97017ef426053510),
    ("die_chamfer", 30, 0x24b66a82e2fbeb95),
    ("die_pips", 80, 0x227d68472c197fd3),
    ("die_pips", 93, 0x1badd986a5f430ea),
    ("heat_sink_fins", 34, 0xa2d439d99f78315c),
    ("die_tool", 52, 0x15227afc62b4bc1c),
    ("die_tool", 67, 0xc8ba7bbc49af03ef),
    ("face_sketch", 45, 0x20e419ab4b4c778d),
    ("face_sketch", 73, 0x8b8d9508a9b574ef),
    ("part_select", 49, 0x15dee26ec97362f9),
    ("part_select", 63, 0xcacec981969db428),
    ("symmetric_plate", 38, 0x9df0413c16d73fd7),
    ("loft_prism", 58, 0x4f99387925deaa74),
    ("die_composed", 80, 0x227d68472c197fd3),
    ("die_composed", 97, 0x398da4497fbb327e),
    ("die_composed_tour", 98, 0xe2cf3b8c7da03ee7),
    ("die_composed_tour", 173, 0xa3a6e789a6accf5f),
    ("die_composed_tour", 248, 0x56e59078d4613ecd),
    ("die_composed_tour", 323, 0xfb58a898ca7c468e),
    ("die_composed_tour", 398, 0x94d39e9e53a92eed),
    ("die_composed_tour", 473, 0xee88741153cdfaf3),
    ("die_composed_tour", 548, 0x5acce2950ba5056a),
    ("die_composed_tour", 623, 0x7347a887058b5ae2),
    ("die_composed_tour", 698, 0x7cdcfbcb745a9596),
    ("die_composed_tour", 773, 0x4abd80bab7d80e68),
    ("die_composed_tour", 848, 0xbcd876c1f9819116),
    ("die_composed_tour", 923, 0x060ac331f15f46e3),
    ("die_composed_tour", 998, 0x2418601885802541),
    ("die_composed_tour", 1073, 0x6b26b1282580ce9a),
    ("die_composed_tour", 1148, 0xc15f6c9bc30a5748),
    ("die_composed_tour", 1223, 0x0b67dcf2e6100df1),
    ("die_composed_tour", 1298, 0xd7a20ccdbdb6b35b),
    ("die_composed_tour", 1373, 0xd69e6fe4d2088d20),
    ("die_composed_tour", 1448, 0xc47c56a08f1fba38),
    ("die_composed_tour", 1523, 0x8792811960c28a23),
    ("die_composed_tour", 1598, 0x7e142d4d45d29851),
    ("die_composed_tour", 1621, 0xf2e5cd60a55f60d9),
    ("plate_param", 57, 0xe97586e3da9a8fac),
    ("plate_param", 71, 0x7d6df0dc2e1b05f8),
    ("kiss_carry", 50, 0x624f8b258c9afd9a),
    ("kiss_carry", 88, 0x722ccea6d5c24738),
    ("kiss_carry", 102, 0xccd19ff51b2b0264),
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
