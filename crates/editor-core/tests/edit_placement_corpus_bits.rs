//! **Every corpus placement places its body by the same bits** — the
//! value channel of each `Node::Place` in the registry, pinned per node.
//!
//! The digest is the fixture's value channel (`placed_body_digest`):
//! the placed bodies' arena counts and every stored point where the
//! copy stands, composed into its space through `in_space` (a copy is
//! its source's body under its frame), so a motion that moved one bit
//! of one coordinate moves the word. A golden in the
//! ordinary sense: when one moves, the question is whether the new
//! motion is right, never how to restore the old number.

use crate::corpus;
use crate::fixture::value_channel::placed_body_digest;
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
/// **Read where each copy stands** (INTENT stage 3 E): a copy is its
/// source's body under its frame, so the word digests the copy composed
/// into its space through `in_space`. 118 of the 119 words held: each
/// copy composes to the bits its rebuilt copy had. The tour die's last
/// row (`1621`, the identity copy of the filleted die) moved: its 15
/// pip-floor points read `0.04999999999999993` where they read
/// `0.04999999999999996` (four ulps), because the subtract that cut the
/// pips computes in the die's frame and reads each pip, a copy, through
/// one composed map where each was rebuilt once per placement before.
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
/// Re-pinned for D′3 (a placement copies one source, and the product is
/// a slot reading a `⊔`): a placement's payload is its one `source`,
/// so each placement's id, and the word hashing it, moved; every row's
/// ordinal held. No point moved: `m10_p_fence`'s ids-masked row, with
/// the corpus's `⊔` nodes set aside, is main's pin. Measured again
/// with main's plane pose merged in, where `cut_cylinder`'s two rows
/// carry both changes. Measured again over main's merge at
/// `1235f4c050`, which brings stage 4 I's re-pin of every row
/// (`f077063267`, ids only): D′3's moved ids land over main's, so every
/// row that moved on either side moved here; the ids-masked fence held.
const PINNED: &[(&str, u32, u64)] = &[
    ("die", 160, 0x8ad5a9a62d197bf9),
    ("die", 196, 0xda0f5127c3d2cad7),
    ("die", 232, 0x427af8584a557e34),
    ("die", 268, 0x74d4eec642c7f152),
    ("die", 304, 0x88c22d734dccf045),
    ("die", 340, 0x4e7c26fb78519d07),
    ("die", 390, 0x9e99dd5dac6fb892),
    ("die", 440, 0xa22b02cf1820fabb),
    ("die", 483, 0xc9ec16d48e4b89c1),
    ("die", 526, 0x8a9edb5eaef56af3),
    ("die", 569, 0x20206f4984734ce5),
    ("die", 612, 0xb7146d78dc95d2ef),
    ("die", 655, 0x0adf8278a8248f08),
    ("die", 705, 0xc55d117018c6a7af),
    ("die", 755, 0x7c15110c8d1f2958),
    ("die", 805, 0xe7e379fd33c4d1d6),
    ("die", 841, 0x0e32a35f3f175fec),
    ("die", 877, 0xd0cf0406704f6e34),
    ("die", 913, 0x3b83b6c4e7469dc7),
    ("die", 949, 0x703e17fd3cc5a8c9),
    ("die", 992, 0x1d10a921ae89a260),
    ("die", 1003, 0x5b58f7d98f278288),
    ("corner_table", 50, 0x4fe72e606fdffb4e),
    ("corner_table", 88, 0x2b1789e6f76d3e80),
    ("corner_table", 126, 0xdae024ffa853b4a8),
    ("corner_table", 164, 0xde818a7ed2dffbe0),
    ("corner_table", 178, 0x24d967f733c4adae),
    ("heat_sink", 58, 0x9711af8af76b478a),
    ("heat_sink", 77, 0x6973a29aa2a26ac3),
    ("heat_sink", 96, 0x3e252c45798c685b),
    ("heat_sink", 115, 0xf41dc573cabf3016),
    ("heat_sink", 134, 0x04aa87b8e1a95a97),
    ("heat_sink", 148, 0xaa045fca34319f46),
    ("heat_sink", 165, 0x8e39358abb66604d),
    ("heat_sink", 184, 0xdd8ad9d32310298d),
    ("heat_sink", 203, 0x2607e4936a533e3e),
    ("heat_sink", 222, 0x40d17e0620fe8e32),
    ("heat_sink", 241, 0x3bfb93b9171e6bce),
    ("crossing_slots", 50, 0x9957a2d2e0ca5bf4),
    ("crossing_slots", 88, 0x275e18247bbf3514),
    ("crossing_slots", 102, 0xf86afba3286d7167),
    ("nested_islands_105", 69, 0x23455bbb9b8d69a1),
    ("nested_islands_105", 88, 0x860c85c5801e4fa9),
    ("nested_islands_105", 126, 0x4e502aa5dbc24393),
    ("nested_islands_105", 140, 0x2f1791921e8a1403),
    ("nested_islands_106_depth1", 69, 0x23455bbb9b8d69a1),
    ("nested_islands_106_depth1", 88, 0x860c85c5801e4fa9),
    ("nested_islands_106_depth1", 126, 0x5f6feadfbcb3110f),
    ("nested_islands_106_depth1", 140, 0xdb5baaf9671cb660),
    ("nested_islands_106_depth2", 69, 0x23455bbb9b8d69a1),
    ("nested_islands_106_depth2", 88, 0x860c85c5801e4fa9),
    ("nested_islands_106_depth2", 126, 0x4e502aa5dbc24393),
    ("nested_islands_106_depth2", 164, 0x8179ee64a090f5c7),
    ("nested_islands_106_depth2", 178, 0x44d4ddbaa0e58ffe),
    ("declared_tangency", 66, 0xaffe8409f108694e),
    ("declared_tangency", 80, 0xe69c31c4a41b5039),
    ("kitchen_sink", 64, 0x06a1f2bc9e6c8c85),
    ("kitchen_sink", 88, 0x90a21f12e475193b),
    ("kitchen_sink", 154, 0x12b3a860a43c38b9),
    ("kitchen_sink", 171, 0x52915acf5d9bcb32),
    ("kitchen_sink", 187, 0xa80a3071683ae65d),
    ("kitchen_sink", 203, 0x6a6e2a23e21b9461),
    ("kitchen_sink", 222, 0x99706b697922f3e0),
    ("kitchen_sink", 241, 0xe7a9068c291c4f05),
    ("kitchen_sink", 260, 0x5b0473a795bf3475),
    ("kitchen_sink", 276, 0xc3804e93648f0fdd),
    ("cut_cylinder", 45, 0xd8d0024afc3a0827),
    ("cut_cylinder", 59, 0x27503802b18ddfb2),
    ("measured_web", 46, 0x32701a4d363829cd),
    ("measured_web", 65, 0xc92471091171df5d),
    ("boss_union", 42, 0x9ca8ff441bbf64c4),
    ("boss_union", 56, 0xf26e981bdc04b3a0),
    ("die_fillet", 30, 0x42f721f4cb1ce71f),
    ("die_chamfer", 30, 0xc8604c07e6cdefa2),
    ("die_pips", 80, 0x8a354a31fb1b4b75),
    ("die_pips", 93, 0x23628454fca46580),
    ("heat_sink_fins", 34, 0xbef3724bb3fe2458),
    ("die_tool", 52, 0x2c3bff9019326ace),
    ("die_tool", 67, 0xc2997e5289a6f0b8),
    ("face_sketch", 45, 0x5479522dafb03509),
    ("face_sketch", 73, 0x9bbde5599727ac2c),
    ("part_select", 49, 0x396c481bde506ba1),
    ("part_select", 63, 0x3fc6e04b382377ff),
    ("symmetric_plate", 38, 0xe478fc215f55c43e),
    ("loft_prism", 58, 0x5d8180d348ae1f2d),
    ("die_composed", 80, 0x8a354a31fb1b4b75),
    ("die_composed", 97, 0x4d31007d4d2b4b21),
    ("die_composed_tour", 98, 0xaddb4489dde41d99),
    ("die_composed_tour", 173, 0x4c241e3fa6cf2a94),
    ("die_composed_tour", 248, 0x8fd9b68bd0594a41),
    ("die_composed_tour", 323, 0x4ab38e565bcd2c03),
    ("die_composed_tour", 398, 0xea49bf2bda71171d),
    ("die_composed_tour", 473, 0xe4aaa310b5af69ed),
    ("die_composed_tour", 548, 0xcca5aae2d42e2229),
    ("die_composed_tour", 623, 0x2e91d00e4687d40a),
    ("die_composed_tour", 698, 0xd453298e0e488f8f),
    ("die_composed_tour", 773, 0x6351be13e63110c8),
    ("die_composed_tour", 848, 0xb814813fbac3848c),
    ("die_composed_tour", 923, 0xa78feffca0c56be5),
    ("die_composed_tour", 998, 0x90559dd91af6faff),
    ("die_composed_tour", 1073, 0x035b368103b7800c),
    ("die_composed_tour", 1148, 0x7e27a6b6ea2fe09a),
    ("die_composed_tour", 1223, 0x5fa2ef8bb25dcc63),
    ("die_composed_tour", 1298, 0x165a7f798d97c9da),
    ("die_composed_tour", 1373, 0x6da9e7044d51e92e),
    ("die_composed_tour", 1448, 0xc66115a1e4d305d5),
    ("die_composed_tour", 1523, 0x556594d2d46704dd),
    ("die_composed_tour", 1598, 0x999c5c5df88486f3),
    ("die_composed_tour", 1621, 0xa6651d59609d4ed0),
    ("plate_param", 57, 0x450f2054aa0c334b),
    ("plate_param", 71, 0x285ae066b3d4d3d8),
    ("kiss_carry", 50, 0xe7f91a43ad64c21d),
    ("kiss_carry", 88, 0x66c2b61869279af2),
    ("kiss_carry", 102, 0x9694f9baccb999a6),
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
        for id in ev
            .order
            .iter()
            .copied()
            .filter(|id| placements.contains(id))
        {
            let (word, fed) = placed_body_digest(&ev, id).unwrap_or_else(|| {
                panic!("{} node {id}: a placement's value is one body", doc.name)
            });
            assert!(fed > 0, "{} node {}: nothing was digested", doc.name, id);
            got.push((doc.name, id.0.ordinal(), word));
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
