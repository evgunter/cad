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
/// `0.04999999999999996` (four ulps): the pips' union computes in its
/// first pip's frame and reads every other pip through `relative`, an
/// inverse and a composition where each pip was rebuilt into the world
/// before, and the cut, both fillets and this copy carry that rounding.
/// Over main's merge at `1f851964` (whose word here is
/// `0xf44c58129da3f0a3`) the same fifteen points move it, and a dump of
/// the corpus composed through `in_space` against `1f851964` differs
/// at those points alone.
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
    ("die", 160, 0x0b0e9ff22465e66d),
    ("die", 196, 0x36ae6d4eacb134a2),
    ("die", 232, 0xade01fc71fecc8c4),
    ("die", 268, 0x83e31b937b9e4b17),
    ("die", 304, 0xdf091afad5632aaf),
    ("die", 340, 0x05108b9159df8f6a),
    ("die", 390, 0xd4ab9db065356fa0),
    ("die", 440, 0xba489e8613e40e0f),
    ("die", 483, 0x285499e405e339a4),
    ("die", 526, 0x2539bc65ce72a157),
    ("die", 569, 0x05ff7f68ee8b3212),
    ("die", 612, 0x5a8cb8c0a3db21a1),
    ("die", 655, 0x69f11c5f7f55a79d),
    ("die", 705, 0x52c988dfff791cf3),
    ("die", 755, 0x7d57f2f120d079b8),
    ("die", 805, 0x6448eeb904c737bd),
    ("die", 841, 0xc1b3a36fb5f36d40),
    ("die", 877, 0xb72ac5545504a131),
    ("die", 913, 0x45447f76a6e2fae5),
    ("die", 949, 0x34b467d2e03befed),
    ("die", 992, 0x35125a681f43d0bc),
    ("die", 1003, 0x25a76c8281a1b1f9),
    ("corner_table", 50, 0xb3e05892034a8894),
    ("corner_table", 88, 0xab90afdd20f8888a),
    ("corner_table", 126, 0x4583d0c1f816ef5f),
    ("corner_table", 164, 0x09b94c717384b795),
    ("corner_table", 178, 0xe601a4b216e254c4),
    ("heat_sink", 58, 0x11dd889726803877),
    ("heat_sink", 77, 0x05d9cf7a847821ea),
    ("heat_sink", 96, 0xcf04611657b094ce),
    ("heat_sink", 115, 0x8759dbfcddefd29f),
    ("heat_sink", 134, 0xbe6cc3fa67531d7f),
    ("heat_sink", 148, 0x9f76dec781b9fe27),
    ("heat_sink", 165, 0x503de8b41b22ec0b),
    ("heat_sink", 184, 0x8e663d380e070441),
    ("heat_sink", 203, 0xa35f9e9f76df4fd0),
    ("heat_sink", 222, 0x96ab6bcb812e763b),
    ("heat_sink", 241, 0xe4e5a9e7004a32b9),
    ("crossing_slots", 50, 0xc1dd3b0e9159b3a2),
    ("crossing_slots", 88, 0x09f9ba1c9f871dc9),
    ("crossing_slots", 102, 0xf97de0f577170fca),
    ("nested_islands_105", 69, 0x6304f06e5bb756d5),
    ("nested_islands_105", 88, 0xeed8a5d01e758619),
    ("nested_islands_105", 126, 0x771b7d779f235ef2),
    ("nested_islands_105", 140, 0x71cee5210c7dc40c),
    ("nested_islands_106_depth1", 69, 0x6304f06e5bb756d5),
    ("nested_islands_106_depth1", 88, 0xeed8a5d01e758619),
    ("nested_islands_106_depth1", 126, 0x47a8f7cb35d5aaf8),
    ("nested_islands_106_depth1", 140, 0x82a92357de7674f6),
    ("nested_islands_106_depth2", 69, 0x6304f06e5bb756d5),
    ("nested_islands_106_depth2", 88, 0xeed8a5d01e758619),
    ("nested_islands_106_depth2", 126, 0x771b7d779f235ef2),
    ("nested_islands_106_depth2", 164, 0x2be409a84fc4c076),
    ("nested_islands_106_depth2", 178, 0xcf74d9a8e2cdde5a),
    ("declared_tangency", 66, 0xaffe8409f108694e),
    ("declared_tangency", 80, 0xe69c31c4a41b5039),
    ("kitchen_sink", 64, 0x685fccab23b8deed),
    ("kitchen_sink", 88, 0x536f61f8dfa6c306),
    ("kitchen_sink", 154, 0x5cfb41402707ee2e),
    ("kitchen_sink", 171, 0x78713ac91fff850a),
    ("kitchen_sink", 187, 0x0cd8518643d0a824),
    ("kitchen_sink", 203, 0x16c4fc80ca3bfe66),
    ("kitchen_sink", 222, 0x482674440cd1a5ec),
    ("kitchen_sink", 241, 0x388bc68662841ffd),
    ("kitchen_sink", 260, 0xce1c6ed64b31fe24),
    ("kitchen_sink", 276, 0x2e730dc4beaebf66),
    ("cut_cylinder", 45, 0xd8d0024afc3a0827),
    ("cut_cylinder", 59, 0x27503802b18ddfb2),
    ("measured_web", 46, 0x4c51d7b60b614057),
    ("measured_web", 65, 0xf81a799da37189fd),
    ("boss_union", 42, 0x6e5420d9163c5bc9),
    ("boss_union", 56, 0x080ea9b86ae95fdf),
    ("die_fillet", 30, 0x42f721f4cb1ce71f),
    ("die_chamfer", 30, 0xc8604c07e6cdefa2),
    ("die_pips", 80, 0xff6f037412ee5476),
    ("die_pips", 93, 0x58a465bde703bc05),
    ("heat_sink_fins", 34, 0xbef3724bb3fe2458),
    ("die_tool", 52, 0x403ad616dad19725),
    ("die_tool", 67, 0xa77e80363036d043),
    ("face_sketch", 45, 0x5479522dafb03509),
    ("face_sketch", 73, 0xb405e06c7eae44c7),
    ("part_select", 49, 0x396c481bde506ba1),
    ("part_select", 63, 0x3fc6e04b382377ff),
    ("symmetric_plate", 38, 0xe478fc215f55c43e),
    ("loft_prism", 58, 0x5d8180d348ae1f2d),
    ("die_composed", 80, 0xff6f037412ee5476),
    ("die_composed", 97, 0x2640176a3e9041ba),
    ("die_composed_tour", 98, 0xa0a74608b70723db),
    ("die_composed_tour", 173, 0x2ca172ee1cef1b30),
    ("die_composed_tour", 248, 0x66902aa93fc4c70b),
    ("die_composed_tour", 323, 0x3f2092e47051df17),
    ("die_composed_tour", 398, 0xe9a92532a216393c),
    ("die_composed_tour", 473, 0x14a2a7129245b74c),
    ("die_composed_tour", 548, 0x93ba74404defc365),
    ("die_composed_tour", 623, 0x408fa8a73798ef07),
    ("die_composed_tour", 698, 0x3d2cbcd40c7f0f66),
    ("die_composed_tour", 773, 0x2358428c37439ee7),
    ("die_composed_tour", 848, 0x67d1dedd500854e8),
    ("die_composed_tour", 923, 0x56db65c91efa9327),
    ("die_composed_tour", 998, 0x27c5b89bd5db4eff),
    ("die_composed_tour", 1073, 0x3af042dec7672506),
    ("die_composed_tour", 1148, 0x5ca7a7aad7a3f3f1),
    ("die_composed_tour", 1223, 0x91dce51169a3f75a),
    ("die_composed_tour", 1298, 0xc1af7d17be5bfec8),
    ("die_composed_tour", 1373, 0x7dcf43928e3b9b63),
    ("die_composed_tour", 1448, 0x0f438d0c1168ff00),
    ("die_composed_tour", 1523, 0xe6e314c7e5f1b012),
    ("die_composed_tour", 1598, 0xa2bc43819236825f),
    ("die_composed_tour", 1621, 0xf9dfe49c1f8fb48f),
    ("plate_param", 57, 0xabb2f540f547e688),
    ("plate_param", 71, 0xfa3a93f364562949),
    ("kiss_carry", 50, 0x88fe841c789d0440),
    ("kiss_carry", 88, 0x9477a514b0d5087c),
    ("kiss_carry", 102, 0xbe14d499184f60f1),
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
            let digest = placed_body_digest(&ev, id);
            assert!(
                digest.is_some(),
                "{} node {id}: a placement's value is one body",
                doc.name
            );
            let (word, fed) = digest.unwrap_or_default();
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
