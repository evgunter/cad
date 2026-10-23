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
/// Re-pinned for D′3 (a placement copies one source, and the product is
/// a slot reading a `⊔`): a placement's payload is its one `source`,
/// so each placement's id, and the word hashing it, moved; every row's
/// ordinal held. No point moved: `m10_p_fence`'s ids-masked row, with
/// the corpus's `⊔` nodes set aside, is main's pin.
const PINNED: &[(&str, u32, u64)] = &[
    ("die", 160, 0xda597511743b6714),
    ("die", 196, 0x82e8008f318aaf84),
    ("die", 232, 0xb0aa504153b0a85f),
    ("die", 268, 0x25c317b48756e5ff),
    ("die", 304, 0x726ae33103a1933b),
    ("die", 340, 0x090b5abdac644a2a),
    ("die", 390, 0xe2f1ee61afa2663c),
    ("die", 440, 0x6ee218d4825babc8),
    ("die", 483, 0xd48057e0cb10c8f7),
    ("die", 526, 0xaae5a127a973ef75),
    ("die", 569, 0x35ca97a81ab06b5d),
    ("die", 612, 0x306b234c23d01705),
    ("die", 655, 0x59b4de470addb3e5),
    ("die", 705, 0x1f942d1ac908bb0d),
    ("die", 755, 0xd26d7de71f16c4cb),
    ("die", 805, 0xe3939fd6925b2ebf),
    ("die", 841, 0xee22cc5be7598ae5),
    ("die", 877, 0x54bf268991b04092),
    ("die", 913, 0x3f3a22dfda16886b),
    ("die", 949, 0xdc375ee36136ec6a),
    ("die", 992, 0xb3f619576a24d299),
    ("die", 1003, 0xce36057e10eba8ef),
    ("corner_table", 50, 0x9923dd3677718509),
    ("corner_table", 88, 0x7551e6d416292946),
    ("corner_table", 126, 0x163ed3b6931c2671),
    ("corner_table", 164, 0x0fc3122ba02258ff),
    ("corner_table", 178, 0x642ac5fe40e7e355),
    ("heat_sink", 58, 0x02d97828a3c06880),
    ("heat_sink", 77, 0x78a852ddb93e8780),
    ("heat_sink", 96, 0xfcab923e35004328),
    ("heat_sink", 115, 0xb6502f16765b2760),
    ("heat_sink", 134, 0xb8aa947c24ef50ea),
    ("heat_sink", 148, 0x061140ebd9ef708b),
    ("heat_sink", 165, 0xa3bbb3038d90b86c),
    ("heat_sink", 184, 0x5a4d2ea1ed5d173f),
    ("heat_sink", 203, 0x353e38f8b477f7d8),
    ("heat_sink", 222, 0x719188f8c4312607),
    ("heat_sink", 241, 0x140cc17cc8775675),
    ("crossing_slots", 50, 0xe9c826f53710729e),
    ("crossing_slots", 88, 0x9348963338e6758a),
    ("crossing_slots", 102, 0x904c33fc8eea0bb5),
    ("nested_islands_105", 69, 0x3354d08ce26aef48),
    ("nested_islands_105", 88, 0xe671e6ed6bb6d640),
    ("nested_islands_105", 126, 0xb861a9eca4e18273),
    ("nested_islands_105", 140, 0x3907960f08955e14),
    ("nested_islands_106_depth1", 69, 0x3354d08ce26aef48),
    ("nested_islands_106_depth1", 88, 0xe671e6ed6bb6d640),
    ("nested_islands_106_depth1", 126, 0xff157a59bcc8f547),
    ("nested_islands_106_depth1", 140, 0xd1951478c5987332),
    ("nested_islands_106_depth2", 69, 0x3354d08ce26aef48),
    ("nested_islands_106_depth2", 88, 0xe671e6ed6bb6d640),
    ("nested_islands_106_depth2", 126, 0xb861a9eca4e18273),
    ("nested_islands_106_depth2", 164, 0xec576e716c904bf8),
    ("nested_islands_106_depth2", 178, 0x232a75bb4cdc50c8),
    ("declared_tangency", 66, 0xaffe8409f108694e),
    ("declared_tangency", 80, 0xe69c31c4a41b5039),
    ("kitchen_sink", 64, 0x09d42298cd0c8bff),
    ("kitchen_sink", 88, 0x51af90698589deb4),
    ("kitchen_sink", 154, 0xcec676f50eb81faf),
    ("kitchen_sink", 171, 0xd51fc4c429c1259e),
    ("kitchen_sink", 187, 0x96845c8da415d395),
    ("kitchen_sink", 203, 0x449b348cbccdb32c),
    ("kitchen_sink", 222, 0x99d3f542c803df4b),
    ("kitchen_sink", 241, 0xe96e45c56765bbdf),
    ("kitchen_sink", 260, 0xf9a4e89f22897e4e),
    ("kitchen_sink", 276, 0xdc591a56c7ff599f),
    ("cut_cylinder", 45, 0x35b8f56f94f96548),
    ("cut_cylinder", 59, 0xe0ab1d69c2eaa17d),
    ("measured_web", 46, 0xf2d58fd2758bd800),
    ("measured_web", 65, 0x6eec47a879f4d56f),
    ("boss_union", 42, 0x6a320a5b303adad3),
    ("boss_union", 56, 0x15aeeeda52308689),
    ("die_fillet", 30, 0x42f721f4cb1ce71f),
    ("die_chamfer", 30, 0xc8604c07e6cdefa2),
    ("die_pips", 80, 0x4ad3d3e60bd57ffe),
    ("die_pips", 93, 0x4e710c53f3413f61),
    ("heat_sink_fins", 34, 0xbef3724bb3fe2458),
    ("die_tool", 52, 0xfa286d9713395dfc),
    ("die_tool", 67, 0x46a58a450a1c32d8),
    ("face_sketch", 45, 0x5479522dafb03509),
    ("face_sketch", 73, 0xa90619be903ce903),
    ("part_select", 49, 0x396c481bde506ba1),
    ("part_select", 63, 0x3fc6e04b382377ff),
    ("symmetric_plate", 38, 0xe478fc215f55c43e),
    ("loft_prism", 58, 0x5d8180d348ae1f2d),
    ("die_composed", 80, 0x4ad3d3e60bd57ffe),
    ("die_composed", 97, 0x6e9f0fa3459a6d6d),
    ("die_composed_tour", 98, 0x0260e91a51df22df),
    ("die_composed_tour", 173, 0xaae3512975a76aa8),
    ("die_composed_tour", 248, 0x5f8b7741569e227f),
    ("die_composed_tour", 323, 0x96718543eedc0729),
    ("die_composed_tour", 398, 0x101fc3a6a6356a1c),
    ("die_composed_tour", 473, 0x9f6e35a13fff428e),
    ("die_composed_tour", 548, 0x0a26b43613a0905f),
    ("die_composed_tour", 623, 0x7a3badda5626a6a8),
    ("die_composed_tour", 698, 0xebc575ba20596244),
    ("die_composed_tour", 773, 0x5c582ceea634eb60),
    ("die_composed_tour", 848, 0xfb385c62a6fda591),
    ("die_composed_tour", 923, 0xf9113d1e2cc57c34),
    ("die_composed_tour", 998, 0xdb9a0bf93d094b41),
    ("die_composed_tour", 1073, 0x1bd9fe7223fdb5c0),
    ("die_composed_tour", 1148, 0x17bf5c041d105973),
    ("die_composed_tour", 1223, 0x7656de0b697e433b),
    ("die_composed_tour", 1298, 0x7a39488cc8c365f9),
    ("die_composed_tour", 1373, 0x81a5d0bb4e919042),
    ("die_composed_tour", 1448, 0x62002732afc0a854),
    ("die_composed_tour", 1523, 0x662fc28e7e1584d7),
    ("die_composed_tour", 1598, 0x084e4252f61b1595),
    ("die_composed_tour", 1621, 0xd5ec5397c0b43f39),
    ("plate_param", 57, 0x016280c01b409ee8),
    ("plate_param", 71, 0x344d4dccd8dd36c8),
    ("kiss_carry", 50, 0xbcf5318bcea099ec),
    ("kiss_carry", 88, 0xa151f255c2ab1cb0),
    ("kiss_carry", 102, 0x1271f6dece7ed6c0),
    ("tube_ring", 12, 0xd785cea9c3eee2fa),
    ("tube_arc", 14, 0x1ac2adc4a57c1e39),
    ("hollow_tube_elbow", 15, 0x2dd186b29afd0375),
    ("hollow_tube_ring", 13, 0xfb791da54bf880f3),
    ("reshaped_rod", 55, 0xde1ce559b047222b),
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
