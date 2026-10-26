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
///
/// **Re-pinned when the value chart came to read a written
/// orientation**: every mate carries its orientation, which the minting
/// edits' bytes hold, so the ids after a mate, and the names spelled
/// with them, moved in the seventeen documents holding a mate. The
/// ids-masked fence moved in `die_composed_tour` alone, its ±x faces'
/// pip seams turned a quarter about the pips' axes.
const PINNED: &[(&str, u32, u64)] = &[
    ("die", 160, 0xa846f87f516b0715),
    ("die", 196, 0xf003f9b6c61ce70a),
    ("die", 232, 0x8a71e167b73c9a71),
    ("die", 268, 0x15635bee1bdf1a6d),
    ("die", 304, 0x2717c780c7c7bea2),
    ("die", 340, 0x088237a6377f7c3d),
    ("die", 390, 0x921ce441162e38a1),
    ("die", 440, 0x2a1cc090b4f23e32),
    ("die", 483, 0x968012e36975a551),
    ("die", 526, 0xd3dd0c1d9ebc989f),
    ("die", 569, 0x12786b2d8e9d7900),
    ("die", 612, 0x9eb2fa3cf78ae990),
    ("die", 655, 0x492d9f73da2acf29),
    ("die", 705, 0xb464f1932ccfc5b5),
    ("die", 755, 0x82d87b0e907ab00e),
    ("die", 805, 0xd65e10c5667816c3),
    ("die", 841, 0xe71af14f52e092c0),
    ("die", 877, 0x62ba35428a9adab2),
    ("die", 913, 0xd4ea311b503dc64e),
    ("die", 949, 0x2a9c04fcb2df640b),
    ("die", 992, 0xea7c226b22c2c54a),
    ("die", 1003, 0xebb798d2bb663b20),
    ("corner_table", 50, 0x3f04793dc5b873f7),
    ("corner_table", 88, 0x03a84c31e746b890),
    ("corner_table", 126, 0x3297dceba3e72880),
    ("corner_table", 164, 0x21dfdc31b44c6047),
    ("corner_table", 178, 0x4a2246da0d385435),
    ("heat_sink", 58, 0xa1cd9c678b19bdd5),
    ("heat_sink", 77, 0x75cd12ccf177455a),
    ("heat_sink", 96, 0x6b839c8430c0f9b1),
    ("heat_sink", 115, 0x60e24c85c2144c54),
    ("heat_sink", 134, 0x1a42a07310e26d0a),
    ("heat_sink", 148, 0xef15c731adbb4cd1),
    ("heat_sink", 165, 0x829e4b8e2c390180),
    ("heat_sink", 182, 0xbacb2346ca1dc863),
    ("heat_sink", 199, 0x10001175f6d54f74),
    ("heat_sink", 216, 0xa5a4d2ec761077ff),
    ("heat_sink", 233, 0xffb706ef9637f478),
    ("crossing_slots", 50, 0x71651d39dcbae63b),
    ("crossing_slots", 88, 0x01168a197af2291f),
    ("crossing_slots", 102, 0xba9ff82f4c9ec1fc),
    ("nested_islands_105", 69, 0xb2b35f0ba0195d26),
    ("nested_islands_105", 88, 0x3a228f47f9a6a339),
    ("nested_islands_105", 126, 0xbf1d5a11b6f10dd3),
    ("nested_islands_105", 140, 0x87d17c88f4571f9c),
    ("nested_islands_106_depth1", 69, 0xb2b35f0ba0195d26),
    ("nested_islands_106_depth1", 88, 0x3a228f47f9a6a339),
    ("nested_islands_106_depth1", 126, 0x0285d35547a83419),
    ("nested_islands_106_depth1", 140, 0x45ec270e4f0938a9),
    ("nested_islands_106_depth2", 69, 0xb2b35f0ba0195d26),
    ("nested_islands_106_depth2", 88, 0x3a228f47f9a6a339),
    ("nested_islands_106_depth2", 126, 0xbf1d5a11b6f10dd3),
    ("nested_islands_106_depth2", 164, 0x9ab325ea20c29cf0),
    ("nested_islands_106_depth2", 178, 0x76d5535cf04ead8b),
    ("declared_tangency", 66, 0x0ffe4e74b205caa7),
    ("declared_tangency", 80, 0xaca0c95868276903),
    ("kitchen_sink", 64, 0x9790c785b6e33e5d),
    ("kitchen_sink", 88, 0xd86cccb679a189e5),
    ("kitchen_sink", 154, 0xf6d999c7a1066822),
    ("kitchen_sink", 171, 0x35448ec7b66c5b65),
    ("kitchen_sink", 185, 0x38aad13235576fd9),
    ("kitchen_sink", 199, 0x05d376c9f783d34f),
    ("kitchen_sink", 216, 0x5b75655031837cd4),
    ("kitchen_sink", 233, 0x4f366e1c9715e543),
    ("kitchen_sink", 250, 0x732df65a1d7a02c1),
    ("kitchen_sink", 264, 0xea7f96fbaf5f4e97),
    ("cut_cylinder", 45, 0x449cda974caa08ec),
    ("cut_cylinder", 59, 0x8d0ab69834e92862),
    ("measured_web", 46, 0x45b5a023144f85a8),
    ("measured_web", 65, 0xad2c5e0884a59ab8),
    ("boss_union", 42, 0x24ec5b5506ab4e61),
    ("boss_union", 56, 0x805740c20d93919a),
    ("die_fillet", 30, 0x97017ef426053510),
    ("die_chamfer", 30, 0x24b66a82e2fbeb95),
    ("die_pips", 80, 0x7850e41eb837bfef),
    ("die_pips", 93, 0x52c898f5f0c59381),
    ("heat_sink_fins", 34, 0xa2d439d99f78315c),
    ("die_tool", 52, 0x237a8df5fcc6c3e3),
    ("die_tool", 67, 0x06c89b9c0e872b13),
    ("face_sketch", 45, 0x20e419ab4b4c778d),
    ("face_sketch", 73, 0xc161c335aa39a247),
    ("part_select", 49, 0x15dee26ec97362f9),
    ("part_select", 63, 0xcacec981969db428),
    ("symmetric_plate", 38, 0x9df0413c16d73fd7),
    ("loft_prism", 58, 0x4f99387925deaa74),
    ("die_composed", 80, 0x7850e41eb837bfef),
    ("die_composed", 97, 0xa70a0bcd299b8cdd),
    ("die_composed_tour", 98, 0x9af0c9aed04170ba),
    ("die_composed_tour", 173, 0x63546cbc69d2b54e),
    ("die_composed_tour", 248, 0x97613843fade15ba),
    ("die_composed_tour", 323, 0xab0120e597fdd609),
    ("die_composed_tour", 398, 0x3a93716f54954cad),
    ("die_composed_tour", 473, 0xe8db4604602cd2b4),
    ("die_composed_tour", 548, 0x1ff8d4834d824409),
    ("die_composed_tour", 623, 0xaef95ca734343fe3),
    ("die_composed_tour", 698, 0xc3bfe04950917994),
    ("die_composed_tour", 773, 0x2e421cc24cac4f7d),
    ("die_composed_tour", 848, 0x107b52dfe4992ba6),
    ("die_composed_tour", 923, 0xfab36c80b03a1a5b),
    ("die_composed_tour", 998, 0x336bcdc19f67299a),
    ("die_composed_tour", 1073, 0x0dfff43c62f267e6),
    ("die_composed_tour", 1148, 0xf6d67153b8a9d6e0),
    ("die_composed_tour", 1223, 0x32ac526b3c40009e),
    ("die_composed_tour", 1298, 0xec0dbf7540a1f8a6),
    ("die_composed_tour", 1373, 0x5f581ad3b318db87),
    ("die_composed_tour", 1448, 0x84b26a8cfc5e9be8),
    ("die_composed_tour", 1523, 0x678dd86da9313a64),
    ("die_composed_tour", 1598, 0xf8598716972686e1),
    ("die_composed_tour", 1621, 0x3a81855d02aa9efe),
    ("plate_param", 57, 0xf570ce21f3b9e6b9),
    ("plate_param", 71, 0x2594a344df30646e),
    ("kiss_carry", 50, 0xf748108def3e94e4),
    ("kiss_carry", 88, 0x91f3d076b3234466),
    ("kiss_carry", 102, 0xa57520aa05acbb90),
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
