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
/// `die` is `0x46ff_fbb3_d481_e812` — the same number
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
const PINNED: &[(&str, u64)] = &[
    ("die", 0xdc9b_8f31_6844_831f),
    ("corner_table", 0x99b9_9cea_1d55_7d6b),
    ("heat_sink", 0x8848_860f_23cd_1004),
    ("crossing_slots", 0x2bdf_1400_8395_80d3),
    ("nested_islands_105", 0xd795_149a_3de1_9e1d),
    ("nested_islands_106_depth1", 0x6131_14eb_ec01_f277),
    ("nested_islands_106_depth2", 0x0b57_8049_4352_573f),
    ("declared_tangency", 0x9669_c317_71c9_2a49),
    ("kitchen_sink", 0xbc5c_5f93_1336_b5f3),
    ("cut_cylinder", 0x41db_1192_9026_3bed),
    ("measured_web", 0x1721_fe2f_f026_bf22),
    ("boss_union", 0xc45a_900d_55ce_dfe2),
    ("die_fillet", 0xf37b_a47b_ed71_31d2),
    ("die_chamfer", 0x33f7_333d_4be4_662e),
    ("die_pips", 0xb9f9_07c4_9bf4_9e03),
    ("heat_sink_fins", 0xd244_b58c_02dc_3b3f),
    ("die_tool", 0xfde1_2e50_663f_f886),
    ("face_sketch", 0xe17f_467e_cf2c_0119),
    // DOCM-2. Two `Part`s of one split and one of a pattern: the
    // projection mints nothing, so every name in the document is the
    // split's, the pattern's, or the union's over them, and the row's
    // arrival moved no other row.
    ("part_select", 0x0020_c610_e95b_464e),
    ("loft_prism", 0x74db_6889_4c07_172b),
    ("die_composed", 0xce99_5734_cf12_d283),
    ("die_composed_tour", 0x527b_0baa_7db2_e7f0),
    ("plate_param", 0x1624_908e_2748_08d7),
    ("kiss_carry", 0xdf57_2c7e_ce54_997d),
    // LIB-TUBE. Both tables are minted by `name_revolve` — the
    // tube doors return `Revolved<T>` and the emitter reads only
    // its maps — so these two rows are the revolve role vocabulary
    // over a body no revolve node built. Their arrival moved no
    // other row, which is the property this table exists to make
    // readable.
    ("tube_ring", 0xa71b_e28f_0bf1_a321),
    ("tube_arc", 0x2af8_6e46_0e5f_188b),
    ("hollow_tube_elbow", 0xafb7_1088_9300_f596),
    ("hollow_tube_ring", 0x7842_e8a7_35aa_eb8d),
    // EDIT-PROGRAM: the one document whose log reshapes a profile
    // under a fillet. Its table is minted over the crease name the
    // door REBOUND, which is the fact this row makes golden; its
    // arrival moved no other row.
    ("reshaped_rod", 0x8379_4d89_2ebf_929a),
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
