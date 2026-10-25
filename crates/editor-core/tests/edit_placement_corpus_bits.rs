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
    ("die", 160, 0xe9f7cbc3cb49c960),
    ("die", 196, 0x0cdb210901d39a7d),
    ("die", 232, 0x6ed9918813956dc9),
    ("die", 268, 0xd1e833fc8e71a13a),
    ("die", 304, 0x889ec38b113d965d),
    ("die", 340, 0x34f70041f7378045),
    ("die", 390, 0xa9db9e29aed28b53),
    ("die", 440, 0x7227058745e474f5),
    ("die", 483, 0x9f8e1b124ff76d66),
    ("die", 526, 0x7ee74075ab6a9ed4),
    ("die", 569, 0xd31be82d2478b68b),
    ("die", 612, 0x8f3a607cdb0f1433),
    ("die", 655, 0x5eb34263d9e06b99),
    ("die", 705, 0x8e19d7ee66c62800),
    ("die", 755, 0x7162f173d6c1bc7f),
    ("die", 805, 0xaaa25d73ca3ce557),
    ("die", 841, 0x5c6e49e6ffdb0e4f),
    ("die", 877, 0x80b877f4985aa924),
    ("die", 913, 0x3e024d32b4b380e6),
    ("die", 949, 0xfe0ad1a034a0859c),
    ("die", 992, 0xe435dc8fe2a5406c),
    ("die", 1003, 0xd7516581103423fc),
    ("corner_table", 50, 0x1fddb82b14fe0a33),
    ("corner_table", 88, 0xea2ae257df090ef1),
    ("corner_table", 126, 0x4591d66d4b8f4dc6),
    ("corner_table", 164, 0x06e25f0655996d2d),
    ("corner_table", 178, 0x755b55de0609fc82),
    ("heat_sink", 58, 0xf8c1d027064d9769),
    ("heat_sink", 77, 0x6d1a6eed895c6f4f),
    ("heat_sink", 96, 0xac8d488f74e65c85),
    ("heat_sink", 115, 0x2f9958c8b3500a1a),
    ("heat_sink", 134, 0x5e74602b4182442f),
    ("heat_sink", 148, 0x6f686c5449490f93),
    ("heat_sink", 165, 0x6f31f5808ebf6fb9),
    ("heat_sink", 182, 0xf7c8013158111c1f),
    ("heat_sink", 199, 0x60f3b3046676ecdb),
    ("heat_sink", 216, 0x53dc9442757d1971),
    ("heat_sink", 233, 0xb77cf705a36b354b),
    ("crossing_slots", 50, 0x502731456c1e624a),
    ("crossing_slots", 88, 0x3f1a90a1e700ec98),
    ("crossing_slots", 102, 0xed0a0285c97d8881),
    ("nested_islands_105", 69, 0xc04b538744eb2b30),
    ("nested_islands_105", 88, 0x634c78d831e7c576),
    ("nested_islands_105", 126, 0xbb30c056c67c3801),
    ("nested_islands_105", 140, 0x36b3b1167473e9b7),
    ("nested_islands_106_depth1", 69, 0xc04b538744eb2b30),
    ("nested_islands_106_depth1", 88, 0x634c78d831e7c576),
    ("nested_islands_106_depth1", 126, 0x34da8113f30b1869),
    ("nested_islands_106_depth1", 140, 0xb283d8cac2a666d9),
    ("nested_islands_106_depth2", 69, 0xc04b538744eb2b30),
    ("nested_islands_106_depth2", 88, 0x634c78d831e7c576),
    ("nested_islands_106_depth2", 126, 0xbb30c056c67c3801),
    ("nested_islands_106_depth2", 164, 0x685728b78a2558e0),
    ("nested_islands_106_depth2", 178, 0x02ac4a0a3e29b4eb),
    ("declared_tangency", 66, 0x0ffe4e74b205caa7),
    ("declared_tangency", 80, 0xaca0c95868276903),
    ("kitchen_sink", 64, 0x5ee49ed391a58d43),
    ("kitchen_sink", 88, 0x78da985acd7ca8d6),
    ("kitchen_sink", 154, 0xc9144bb436d50356),
    ("kitchen_sink", 171, 0xdbb59497dab2d202),
    ("kitchen_sink", 185, 0xb4b86acb52250ada),
    ("kitchen_sink", 199, 0x02ad4b233b29b3a5),
    ("kitchen_sink", 216, 0xefaddfdb27c82b0d),
    ("kitchen_sink", 233, 0x3af78a62e25395b9),
    ("kitchen_sink", 250, 0x6634ed89ee27278a),
    ("kitchen_sink", 264, 0xbf21aaf447d70f48),
    ("cut_cylinder", 45, 0x449cda974caa08ec),
    ("cut_cylinder", 59, 0x8d0ab69834e92862),
    ("measured_web", 46, 0xa4d6adde39527231),
    ("measured_web", 65, 0x25ed2f8c6497d927),
    ("boss_union", 42, 0xd961ca079d1400b8),
    ("boss_union", 56, 0xc1742ee5879536a1),
    ("die_fillet", 30, 0x97017ef426053510),
    ("die_chamfer", 30, 0x24b66a82e2fbeb95),
    ("die_pips", 80, 0x0ecf164e46177e2b),
    ("die_pips", 93, 0xdaaed2353e3f8794),
    ("heat_sink_fins", 34, 0xa2d439d99f78315c),
    ("die_tool", 52, 0xfc2a92c039fbd13f),
    ("die_tool", 67, 0xd5e01ca3bf16d5e9),
    ("face_sketch", 45, 0x20e419ab4b4c778d),
    ("face_sketch", 73, 0x1c9f92d11c67f0b8),
    ("part_select", 49, 0x15dee26ec97362f9),
    ("part_select", 63, 0xcacec981969db428),
    ("symmetric_plate", 38, 0x9df0413c16d73fd7),
    ("loft_prism", 58, 0x4f99387925deaa74),
    ("die_composed", 80, 0x0ecf164e46177e2b),
    ("die_composed", 97, 0x9de1fd7baf6ac605),
    ("die_composed_tour", 98, 0x2f91a371516704c7),
    ("die_composed_tour", 173, 0xfc645add656f7ae5),
    ("die_composed_tour", 248, 0x79de6e880cc1ac03),
    ("die_composed_tour", 323, 0x6046e481593f631e),
    ("die_composed_tour", 398, 0x97e678ca4a05fbda),
    ("die_composed_tour", 473, 0x9e5b8f2331a4096c),
    ("die_composed_tour", 548, 0x57eee665e42ed1cf),
    ("die_composed_tour", 623, 0x3de59ab627776f15),
    ("die_composed_tour", 698, 0xab93ff42fe7b966c),
    ("die_composed_tour", 773, 0x01a50c12937b270d),
    ("die_composed_tour", 848, 0x7b5b2d92ac2265cd),
    ("die_composed_tour", 923, 0xb0ea738585fd6fc4),
    ("die_composed_tour", 998, 0x3023304745b8aeb6),
    ("die_composed_tour", 1073, 0xc84729dacd95824b),
    ("die_composed_tour", 1148, 0x1db986eea897b943),
    ("die_composed_tour", 1223, 0xb6970c835b4e6d78),
    ("die_composed_tour", 1298, 0x8d50e4241252ed67),
    ("die_composed_tour", 1373, 0xce15c8a9a52b6355),
    ("die_composed_tour", 1448, 0xe7448237b58b5d7b),
    ("die_composed_tour", 1523, 0x9fd33b723af23c05),
    ("die_composed_tour", 1598, 0x003ba4583ed17eeb),
    ("die_composed_tour", 1621, 0x52b1d8ecd9dbf965),
    ("plate_param", 57, 0x59c0c9e0f1747b04),
    ("plate_param", 71, 0xdc7406ee97caebdb),
    ("kiss_carry", 50, 0x62f04a9f06fae599),
    ("kiss_carry", 88, 0x2d5f19ed416b38ee),
    ("kiss_carry", 102, 0x23b1e8ce051fc7a9),
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
