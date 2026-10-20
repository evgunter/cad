//! **Per-document NAME-TABLE digests over the whole corpus registry,
//! pinned** — adopted from the LIB-G16 R2 review probe, which printed
//! them for a cross-tree comparison and so expired with it
//! (implementer-discipline §8).
//!
//! What earns it a permanent seat is the hole it fills. Two corpus-wide
//! goldens already exist and neither is this one: `m10_p_fence` pins
//! every body POINT's bits and says nothing about names, and
//! `m4_pr3_names_ci` pins a name digest for the die FIXTURE only. So
//! until now no committed number covered "the registry's name tables",
//! which is exactly the surface an emitter change moves — LIB-G16
//! re-shaped `emit_fillet` onto the shared tie deferral and had to
//! measure that claim by hand, against a checkout of main, because
//! nothing in the tree would have caught it.
//!
//! It is a golden in the ordinary sense: when one of these moves the
//! question is whether the NEW names are right, never how to restore
//! the old number. A document added to or removed from the registry
//! moves its own row and no other, which is what makes a diff here
//! readable.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

/// FNV-1a 64 over the tables' deterministic Debug encoding — copied
/// from `m4_pr3_names_ci.rs::digest` so the number is comparable.
fn digest(ev: &editor_core::Evaluation<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |s: &str| {
        for b in s.bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for id in &ev.order {
        feed(&format!("#{id:?}"));
        for t in crate::fixture::tables(ev, *id) {
            for (n, e) in t.iter() {
                feed(&format!("{n:?}={e:?};"));
            }
        }
    }
    h
}

/// The pinned digest per registered document, in registry order.
///
/// `die_composed_tour` is the row this file was built to be able to
/// take. The measurement that motivated the gate — "do two documents
/// with the same recipe have the same name table" — had to be made by
/// hand against the demo tour's composed die, because that document
/// lived only in `demos/tour` and no registered document was its
/// equal. LIB-CORPUS-DIE registered it (as committed bytes the tour
/// regenerates — `corpus::die_composed_tour`), so the hand
/// measurement is now this number: forty-two rim arcs and twelve box
/// edges, in one pin.
///
/// **That number moved once, at DOCM-3.** (`corner_table`'s moved
/// when a merged face's constituent set became flat: its legs' wall
/// merges list the walls, not the earlier merge — one of them a
/// merge over a merged face carried through two untouched booleans,
/// the pass-through shape — pinned by
/// `docm8_flat_merged::a_boolean_over_a_boolean_mints_a_flat_merged_row_and_replays`.)
/// The die's cutting tool is one `Node::Union` over its 21 pips
/// now instead of twenty chained pairwise unions, so every name in it
/// below the tool is `FromMember { member, of }` — one segment, naming
/// the pip — where it used to be a `FromA`/`FromB` descent as deep as
/// the pip's position in the chain. The names are the whole of what
/// moved: `docm3_union::the_dies_union_is_the_chain_it_replaced`
/// asserts the tool's body is bit-identical to the chain's, face for
/// face and description for description, in one document at one
/// scalar. Every other row above and below is byte-identical, which is
/// the receipt that the change is local to the die.
///
/// The digest encodes each name's entity with its slot VERSION, so a
/// change in how many keys an arena has spent before a mint moves a
/// row with no name changing. A graft spends one key per dead
/// ancestor its forwarded records name (`topo`'s combine door), which
/// is why the documents whose booleans graft a split operand carry
/// versions that no name reads.
///
/// `measured_web` (M10-2) was ADDED to this table when its document
/// joined the registry, and the add is the header's "one row moves"
/// claim being MEASURED rather than restated: the re-cut printed
/// nineteen numbers identical to the ones already pinned here and one
/// new row. A measurement sink denotes no body and mints no name of
/// its own, so it moves neither the geometry fence nor any other
/// document's names.
///
/// `measured_web`'s row moved, alone, when a measure became one
/// primitive (INTENT stage 2 PR D): its measure's mint preimage moved
/// with the node's shape, and with it the ids its tables are keyed by.
///
/// `die_composed` and `die_composed_tour` are the only registered
/// documents that carve a CLOSED chain, so they are the only two whose
/// tables carry the rim-phase roles at all (four band trimlines and
/// eighty-four respectively; every other row's tables have none). A
/// change to the rim vocabulary therefore moves exactly these two
/// numbers, and a change that moves a third is not about rims.
///
/// That census is not prose to be trusted: it is computed by
/// `blend5_r1_probes::the_recorded_band_trim_counts_are_executable`
/// (and again by `blend5_r2_probes`'s), which walks the same registry
/// and fails if any of the three numbers drifts. Read the claim here,
/// believe it there.
///
/// `die_fillet` and `die_chamfer` differ in their blend's minting node
/// and nothing else, and that is what RECIPE-DOORS D3 says should
/// happen. The two documents are the same three-node recipe with the
/// blend swapped, so D3 having ruled the role vocabulary shared, the
/// blend composes the same `RoleSeg`s off the same upstream names. Its
/// node id is minted from the node itself (N1), so a fillet and a
/// chamfer mint two ids and their names differ in `node` alone.
/// `emit_fillet.rs`'s tie probe asserts the within-one-document case,
/// where the two nodes differ and the names must be disjoint.
///
/// **`part_select` moved at JOIN-1's fix pass** (PR 3790), alone. Its
/// union of the two split halves now builds through the chord join: the
/// halves' side faces meet along edges of both solids, coplanar on the
/// far side. The table is the box's — one body, six faces (the four
/// sides each a `Merged` of the two halves' fragments), sixteen edges
/// and twelve vertices, every name a `FromA`/`FromB` lineage — and
/// the persisted text did not move (`perf2_name_keying_differential`'s
/// second column).
///
/// **Re-pinned for INTENT-LITERALS PR C** (a slot holds a variable):
/// every node is minted from slots that hold variable ids, a typed
/// value's variable drawn from what it holds, so every node id moved
/// and with it every row this hashes. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held untouched across the change.
///
/// **Re-pinned for INTENT-LITERALS PR D** (`Expr` holds no float):
/// `kitchen_sink` alone, whose formulas hold written quantities that
/// now mint variables of their own; every other row held its word.
///
/// **Re-pinned for INTENT stage 2 PR C** (the product is the world):
/// each document now places its bodies, and every placement is a node
/// with a name table of its own, its copy's names under it, so every
/// row holds those tables too. What each document delivers did not
/// move: `intent_s2_c_world`'s migration check holds each product to
/// its pre-C digest.
///
/// **Re-pinned for INTENT stage 2 PR E** (a selection is a variable):
/// the seven documents that blend, shell, frame on a face or measure —
/// `measured_web`, `die_fillet`, `die_chamfer`, `face_sketch`,
/// `die_composed`, `die_composed_tour` and `reshaped_rod` — now mint a
/// selection variable each such node reads, so their node ids moved and
/// every row naming them; every other row held its word, and the
/// id-masked geometry fence held untouched.
///
/// **Re-pinned for INTENT stage 3 PR A** (a revolve's axis is a line on
/// the node): the five documents that revolve — `kitchen_sink`,
/// `die_pips`, `die_tool`, `die_composed` and `die_composed_tour` —
/// hold no in-plane axis datum, so their later node ids moved and every
/// name holding one; every other row held its word. Re-taken merged
/// over FORK-DM4 unit 1 (PR 4527), whose names keyed by read had moved
/// the same rows on main.
///
/// **Re-pinned for the killed-lineage chase** (`emit_topo`'s
/// `KilledSplits`): `crossing_slots` only. Its second slot's bottom rim
/// reaches the result in pieces split off around stretches the subtract
/// killed. Those pieces were read as chords, and one was named a seam of
/// the first slot's floor and the second slot's wall. They are now
/// pieces of that rim. Two floor corners moved with them: one from a
/// junction of three seam lines to a crossing of the rim, and one from
/// a face crossing to the seam vertex of two edges that touch. Every
/// other document held its word.
///
/// **Re-pinned for the vertex run** (N2: an `EdgeCrossing` is a run of
/// (line, sense) in name order): `corner_table` and `crossing_slots`,
/// the two documents whose pair booleans meet edge to edge. Their
/// tables are the old ones with each edge × edge vertex re-spelled: read
/// as a set of (line, sense), every run is the pair it replaced, and no
/// other row moved.
/// **Re-pinned for the output stage's killed lineage** (`topo`'s
/// output stage copies the records of the edges its merge kills into
/// `BooleanNaming::killed_lineage`, which `KilledSplits` reads):
/// `crossing_slots` only. The second subtract's merge of the two flush
/// floors kills one stretch of the first cavity's bottom rim and keeps
/// the next, whose lineage stopped at the stretch killed, so it was
/// named a seam of the cavity's wall and the slot's floor. It is now a
/// piece of that rim, and the floor corner where the slot's rim meets
/// the cavity's wall moved from a touch of the wall to a crossing of the
/// two rims, as the other three such corners are named. Every
/// other document held its word.
///
/// **Re-pinned for FORK-DM4 unit 2** (a name is scoped by the variable
/// holding its body): `cut_cylinder`, `kitchen_sink` and `part_select`,
/// the three documents holding a split. A split's two halves are two
/// tables and its names spell no half, so each row walks both tables of
/// the half-free names; `part_select`'s union, which declares the rest
/// between the halves by the section face, also took a new id. Every
/// other row held its word, and the id-masked geometry fence held.
///
/// Re-taken merged over INTENT stage 3 PR A (a revolve's axis is a
/// line on the node), which moved `kitchen_sink` too: its merged row is
/// neither tree's value; every other row is the one of the two trees
/// that moved it.
///
/// **Re-taken merged over INTENT stage 4 F** (declared pairs retire):
/// main's re-bless and this unit's each moved these rows, so the merged
/// rows are neither tree's value; the id-masked geometry fence held.
///
/// **Re-taken merged over INTENT stage 3 B** (#5232, a placement is the
/// bundle of mates), which re-pinned every row by its new mint chain:
/// the merged rows are main's values but for the three split documents,
/// `cut_cylinder`, `kitchen_sink` and `part_select`, which this unit
/// moves, and whose merged values are neither tree's.
///
/// **Re-pinned for INTENT stage 3 C** (a mate relates two poses), every
/// row: each document places its copy where it stands by a loose
/// `Place` and the named interim's world pin, where one `place_in_world`
/// edit minted the placement holding its world pin. The placement's mint
/// preimage moved, and every id drawn after it. No outcome or point
/// moved: `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// **Re-pinned for a seam citing the parent face** (N3: at every
/// boolean a seam cites, as each side, the parent of the face beside
/// it): `crossing_slots` only. Its second subtract merges the two flush
/// floors, and the two seams where the first slot's side walls meet the
/// merged floor cited the second slot's floor alone; they now cite the
/// merged floor's `Merged` parent, in the subtract and its world copy.
/// No other row moved.
///
/// **Re-pinned for INTENT stage 3 D** (Transform retires into a
/// placement), every row: each document was re-spelled without `Datum`
/// or `Transform` (a construction is built in its own coordinates and
/// placed into the world by a `Place`), so its nodes, their mint
/// preimages and every name they mint moved.
///
/// **Re-pinned, `kitchen_sink` only**, when its lone block's copies came
/// to hang below the world's xy as its retired `SetExtrudeSide` edit had
/// them: placed at a pose, the two mint one id fewer than standing did,
/// so every name minted after them moved.
///
/// **Re-pinned for the identity bundle** (FORK-REST), `heat_sink`,
/// `declared_tangency`, `kitchen_sink`, `cut_cylinder` and
/// `part_select`: a copy standing where its body stands is placed by no
/// mate and the constant `0` on every freedom, written as one
/// `AddValues`, which mints a different number of ids from the plane mate and three
/// values it replaces, so every node minted after a stand moved its id:
/// exactly the documents that mint a node after a stand
/// (`cut_cylinder`'s second half stands after its first). Every
/// standing copy is bit-identical to its source.
///
/// Re-pinned for exact turns (INTENT stage 3 D): `die_tool`, whose
/// ball's heading `turn/4` now reads its axis exactly, so the placement
/// gesture writes its spin as the constant `0` where it wrote a typed
/// `6.1e-17 rad`, one variable fewer, and every later id moved; the pips
/// of `die_pips`, `die_composed` and the tour die, which stand on lines
/// stated from the cube's corner; and `kitchen_sink`, whose revolve
/// stands upright by a bundle with no mate spun `turn/4` and whose
/// split's plane and pattern's axis are read off its faces. Every other
/// document held, across main's merge at `557e581a64` too.
///
/// `die_tool` moved again when its master ball came to stand on the
/// cube's corner frame (`corner_frame`) by an axis mate whose slide and
/// spin are the constant `0`, where `place_against` had placed it by a
/// read-back frame that minted 42 more ids, and its explicit rule's
/// frames came to be built from the fraction of a turn each is. Every
/// other document held.
///
/// Re-pinned merging main at `be82e5ad6b` into D, `die`,
/// `crossing_slots`, the three `nested_islands`, `kitchen_sink`,
/// `cut_cylinder`, `die_pips`, `die_tool`, `part_select`,
/// `die_composed` and the tour die: main's N3 names each seam side and
/// crossing line with the read it came in through, and in D a later
/// operand is a placed copy, so the names those documents' booleans
/// spell moved under both changes and each value is neither tree's.
const PINNED: &[(&str, u64)] = &[
    ("die", 0x518b99d8559ab074),
    ("corner_table", 0x29800dc909826450),
    ("heat_sink", 0x99d838be7a738b42),
    ("crossing_slots", 0xc888df0d2e771787),
    ("nested_islands_105", 0x31c5a36546704c22),
    ("nested_islands_106_depth1", 0x55267bf7d8792969),
    ("nested_islands_106_depth2", 0xb535ff1ee5840578),
    ("declared_tangency", 0x57d4ee11deccc368),
    ("kitchen_sink", 0x82d15219141de51a),
    ("cut_cylinder", 0x52479871c2112751),
    ("measured_web", 0x44c4e0d24d38b03c),
    ("boss_union", 0x52aab550bf0ac788),
    ("die_fillet", 0xc57e5803d9b2429a),
    ("die_chamfer", 0x07195df35d2892cc),
    ("die_pips", 0x302a1b3ab51621f3),
    ("heat_sink_fins", 0xc11b5c13358b6b56),
    ("die_tool", 0xe589dd2a21a2f936),
    ("face_sketch", 0xe1162804a5269b4c),
    ("part_select", 0xb7a13d41d1e319e8),
    ("loft_prism", 0x5a473fb46c0284ce),
    ("die_composed", 0x47b37cc0fafd5023),
    ("die_composed_tour", 0xab38ee0562a0d20e),
    ("plate_param", 0x91ce84257aef28bf),
    ("kiss_carry", 0x800de290a5901f2f),
    ("tube_ring", 0xc7e2d65e798931e7),
    ("tube_arc", 0x74d5ab449e276df3),
    ("hollow_tube_elbow", 0xcf9e4a2897047237),
    ("hollow_tube_ring", 0xadf6b4934f02aa7b),
    ("reshaped_rod", 0x8213e4d78f6b4396),
];

#[test]
fn every_corpus_documents_name_tables_are_golden() {
    let got: Vec<(String, u64)> = corpus::documents()
        .iter()
        .map(|d| {
            let ev = corpus::eval::<f64>(&d.doc);
            (d.name.to_owned(), digest(&ev))
        })
        .collect();
    let want: Vec<(String, u64)> = PINNED.iter().map(|(n, h)| ((*n).to_owned(), *h)).collect();
    let render = |v: &[(String, u64)]| {
        v.iter()
            .map(|(n, h)| format!("    (\"{n}\", 0x{h:016x}),"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(
        got,
        want,
        "the corpus's name tables moved. Decide whether the NEW names \
         are right; if they are, this is the fresh table:\n{}",
        render(&got)
    );
}
