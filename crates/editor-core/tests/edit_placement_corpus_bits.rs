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
const PINNED: &[(&str, u32, u64)] = &[
    ("die", 160, 0xd5d0ffc6dba4af42),
    ("die", 196, 0x47ef0c99b04b1704),
    ("die", 232, 0x75c11b111242d094),
    ("die", 268, 0xa63089cd39b370da),
    ("die", 304, 0x82e0972577bb08d2),
    ("die", 340, 0x8b6f06e186065cd5),
    ("die", 390, 0x3f50826458633258),
    ("die", 440, 0xbc6dbf817eb86745),
    ("die", 483, 0x1d21186b069e48d9),
    ("die", 526, 0xb40a21eb4a50de5c),
    ("die", 569, 0x27f97ec138b85dc8),
    ("die", 612, 0x077673a898ed89b5),
    ("die", 655, 0x78da254bacce1400),
    ("die", 705, 0xf7468ab79756a82e),
    ("die", 755, 0xca67eb7c438fc059),
    ("die", 805, 0x7aa3675a6321f4d8),
    ("die", 841, 0x0d80ff8c75ec0131),
    ("die", 877, 0xb7e54b66979aee86),
    ("die", 913, 0x3f7759332e47499c),
    ("die", 949, 0x202ce95bdbb0a38f),
    ("die", 992, 0x0cfc0ccd50468e6c),
    ("die", 1003, 0xbeaa262285c58b71),
    ("corner_table", 50, 0xfffc84b637d6a7d1),
    ("corner_table", 88, 0x8a5e3647e95403a3),
    ("corner_table", 126, 0x75d242e631400cab),
    ("corner_table", 164, 0x659469f5e93af449),
    ("corner_table", 181, 0xa9494374b65ef0fa),
    ("heat_sink", 58, 0xcf40f533d2f42a71),
    ("heat_sink", 77, 0xfbcd03e99d9b83a6),
    ("heat_sink", 96, 0x2bb20b78c9becfc7),
    ("heat_sink", 115, 0x41f84e09d49e7fe2),
    ("heat_sink", 134, 0x2a24e23e79054dec),
    ("heat_sink", 148, 0xf144c51984de2d79),
    ("heat_sink", 167, 0x10d4d8efd6ac2be7),
    ("heat_sink", 184, 0x99d3139708ec4778),
    ("heat_sink", 201, 0xaaa0185649ef0b04),
    ("heat_sink", 218, 0xec3590c2ad9b29fd),
    ("heat_sink", 235, 0x044f81ef7aa3e95d),
    ("crossing_slots", 50, 0x7357301d1694aa5d),
    ("crossing_slots", 88, 0x3afb223d12afec27),
    ("crossing_slots", 102, 0xd965217b8d300cdb),
    ("nested_islands_105", 69, 0x51a518d81f2453db),
    ("nested_islands_105", 88, 0x80ad4c71d417a8ba),
    ("nested_islands_105", 126, 0x1d9dc53953a501a1),
    ("nested_islands_105", 140, 0x06a4511caa8a69ba),
    ("nested_islands_106_depth1", 69, 0x51a518d81f2453db),
    ("nested_islands_106_depth1", 88, 0x80ad4c71d417a8ba),
    ("nested_islands_106_depth1", 126, 0xf85d14c7553fcfa8),
    ("nested_islands_106_depth1", 140, 0x72ca0cbe36c413af),
    ("nested_islands_106_depth2", 69, 0x51a518d81f2453db),
    ("nested_islands_106_depth2", 88, 0x80ad4c71d417a8ba),
    ("nested_islands_106_depth2", 126, 0x1d9dc53953a501a1),
    ("nested_islands_106_depth2", 164, 0x9fb330b6f8ee464e),
    ("nested_islands_106_depth2", 178, 0xaa80e813ed23b89f),
    ("declared_tangency", 66, 0xf66043dd4d17019c),
    ("declared_tangency", 83, 0x7f46ac8a5a041823),
    ("kitchen_sink", 64, 0xdbfa860d06d7fca5),
    ("kitchen_sink", 88, 0x3a87593c0cff0cc0),
    ("kitchen_sink", 155, 0xe450d9b16dd62ab7),
    ("kitchen_sink", 171, 0x0547313954ba6f6d),
    ("kitchen_sink", 183, 0x590ed8cf115ad2a4),
    ("kitchen_sink", 197, 0x7e21aee3bf2ef3a0),
    ("kitchen_sink", 214, 0x6ed7a3c649c801ee),
    ("kitchen_sink", 231, 0x089f5987459692c8),
    ("kitchen_sink", 248, 0x2697424b003c9e57),
    ("kitchen_sink", 262, 0xfe724383d1d7899b),
    ("cut_cylinder", 45, 0x7ce3f4275d2a40a6),
    ("cut_cylinder", 59, 0x092e7d97c73298aa),
    ("measured_web", 46, 0x0c16e9baa3e83ea5),
    ("measured_web", 65, 0x3a2887899f7ffb3c),
    ("boss_union", 42, 0x97e49e2f52580427),
    ("boss_union", 56, 0xd6cf01da05757ea2),
    ("die_fillet", 30, 0x02513ff88f9c0bfa),
    ("die_chamfer", 30, 0xa0c8dcc7d6b790ec),
    ("die_pips", 80, 0x72df4501b5207a4a),
    ("die_pips", 93, 0xae2c80640ffd2b36),
    ("heat_sink_fins", 36, 0x1f75b05a9753a2f4),
    ("die_tool", 52, 0x0ba6eb2612c51f0e),
    ("die_tool", 67, 0xa242668b2db9d56a),
    ("face_sketch", 45, 0xb2120ea6ad9fd5de),
    ("face_sketch", 73, 0x81eb03461c9d99e0),
    ("part_select", 49, 0x7436ad8a8711203e),
    ("part_select", 63, 0x6310ca5aabf1b799),
    ("symmetric_plate", 44, 0x34084bd173339786),
    ("loft_prism", 58, 0x74c3cf2b7fee9584),
    ("die_composed", 80, 0x72df4501b5207a4a),
    ("die_composed", 97, 0x258708453786d16d),
    ("die_composed_tour", 98, 0xbe22ebf6b239457f),
    ("die_composed_tour", 173, 0x3000f459889479a6),
    ("die_composed_tour", 248, 0xf5c16f5ce77a546c),
    ("die_composed_tour", 323, 0x9c9a93eacee38dc6),
    ("die_composed_tour", 398, 0x993f3276bbc3f363),
    ("die_composed_tour", 473, 0x15ca8de1ab717c5c),
    ("die_composed_tour", 548, 0x9e7122e37adb60d2),
    ("die_composed_tour", 623, 0x8e27c11c25da5f24),
    ("die_composed_tour", 698, 0x48927a4a5b46c302),
    ("die_composed_tour", 773, 0xa45ab0b30b8fb7ec),
    ("die_composed_tour", 848, 0x100b90a5d528ac98),
    ("die_composed_tour", 923, 0x8b3234ec581b030b),
    ("die_composed_tour", 998, 0x781de1fa12b2a2c8),
    ("die_composed_tour", 1073, 0x4f76c9a7fb3a9ec3),
    ("die_composed_tour", 1148, 0x573dcbc8af86cd72),
    ("die_composed_tour", 1223, 0x4e24f130b5fda24a),
    ("die_composed_tour", 1298, 0xe4f43455b83b71f8),
    ("die_composed_tour", 1373, 0x4dfa814beb856de8),
    ("die_composed_tour", 1448, 0x257e07332255bc3f),
    ("die_composed_tour", 1523, 0xe796813147443f27),
    ("die_composed_tour", 1598, 0xa9304231a123f145),
    ("die_composed_tour", 1621, 0x4be208f0c8787fd0),
    ("plate_param", 57, 0xf502ce8bceb22ec4),
    ("plate_param", 71, 0x5c42776174d01720),
    ("kiss_carry", 50, 0xb70cde5510b8e057),
    ("kiss_carry", 88, 0xe2872ec7128bf74f),
    ("kiss_carry", 102, 0x2de414a51db0b405),
    ("tube_ring", 12, 0x74de1186f8f9baf9),
    ("tube_arc", 14, 0x0c98a3cbaaaf6ebf),
    ("hollow_tube_elbow", 15, 0x97e793ddd9e3ef94),
    ("hollow_tube_ring", 13, 0xf0c417be91c420be),
    ("reshaped_rod", 55, 0xf6a593d3a7dd9495),
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
