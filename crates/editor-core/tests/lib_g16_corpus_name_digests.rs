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
        if let Some(v) = ev.value(*id) {
            for (n, e) in v.name_table.iter() {
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
/// Two more rows are worth a reader's second look, and neither is a bug.
///
/// `die` is the same number
/// `m4_pr3_names_ci::DIE_TABLE_DIGEST` carries, because it is the same
/// digest of the same tables. The two pins agreeing is a cross-check,
/// not a duplication: that one covers the die FIXTURE through its own
/// bump rows, this one covers the registry.
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
const PINNED: &[(&str, u64)] = &[
    ("die", 0xddcc3ced224b3abe),
    ("corner_table", 0xbf41aa6a471ab71c),
    ("heat_sink", 0xaacd962298c4a53c),
    ("crossing_slots", 0x502c3f6a3cfb1919),
    ("nested_islands_105", 0xdf4442cd7b688111),
    ("nested_islands_106_depth1", 0xe60447ef73de5f47),
    ("nested_islands_106_depth2", 0x0d0f5287d3949db6),
    ("declared_tangency", 0x370595ad9c1f1053),
    ("kitchen_sink", 0x9b19e7c275c8b587),
    ("cut_cylinder", 0xecad82208268b4e3),
    ("measured_web", 0x04a7a443eb7d0a8d),
    ("boss_union", 0xbb9123c70a8e96d8),
    ("die_fillet", 0x89166ac05d7aa862),
    ("die_chamfer", 0xe4b825bef56c47a6),
    ("die_pips", 0x5f07295c3f1a2633),
    ("heat_sink_fins", 0xf0b12754e2c4efed),
    ("die_tool", 0x1cd399b374c5520c),
    ("face_sketch", 0xac24651484178273),
    ("part_select", 0xbd9176c4c3e50f39),
    ("loft_prism", 0x4413e8ba1b27cd15),
    ("die_composed", 0x690e7c680cebda97),
    ("die_composed_tour", 0x7e7ec4fb3e9e7dbd),
    ("plate_param", 0xb8c9e234c718e107),
    ("kiss_carry", 0x8be2464010f4249d),
    ("tube_ring", 0x0d0d950420bce9dd),
    ("tube_arc", 0x0dc2a35cb5306ecc),
    ("hollow_tube_elbow", 0xa033450e9f76acfa),
    ("hollow_tube_ring", 0x2c05f4d199bc5e59),
    ("reshaped_rod", 0x9cf09f79ade28dac),
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
