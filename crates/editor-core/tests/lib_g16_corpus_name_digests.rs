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
/// far side, which the join used to refuse and the declared-REST zip
/// then built. The table is the box's — one body, six faces (the four
/// sides each a `Merged` of the two halves' fragments), sixteen edges
/// and twelve vertices, every name a `FromA`/`FromB` lineage — and
/// the persisted text did not move (`perf2_name_keying_differential`'s
/// second column).
const PINNED: &[(&str, u64)] = &[
    ("die", 0xa2b2_a066_44d5_b41a),
    ("corner_table", 0x3799_5a30_3006_7754),
    ("heat_sink", 0xc27b_6076_aa92_b048),
    ("crossing_slots", 0x045a_a35f_7ffb_0917),
    ("nested_islands_105", 0x78ec_775b_bb90_0ef0),
    ("nested_islands_106_depth1", 0x95e1_84f1_6732_94ad),
    ("nested_islands_106_depth2", 0xc372_da0a_3e72_3723),
    ("declared_tangency", 0x10e3_3436_e0dd_f2ca),
    ("kitchen_sink", 0xeac0_dc07_2a2b_518d),
    ("cut_cylinder", 0x4fc1_3f27_d303_0751),
    ("measured_web", 0x6a3e_d351_0833_d5e8),
    ("boss_union", 0x0c9a_9265_78cb_ccf6),
    ("die_fillet", 0x9604_14fb_3d8d_dbf8),
    ("die_chamfer", 0x6ec4_d463_dbda_f46c),
    ("die_pips", 0x0c4b_f7fa_1d64_a3ee),
    ("heat_sink_fins", 0xde1b_5e70_e134_c51f),
    ("die_tool", 0x3cfd_3326_58c3_914f),
    ("face_sketch", 0x8969_aadc_d370_4777),
    // DOCM-2. Two `Part`s of one split and one of a pattern: the
    // projection mints nothing, so every name in the document is the
    // split's, the pattern's, or the union's over them, and the row's
    // arrival moved no other row.
    ("part_select", 0x0856_e02c_2e9d_1b7b),
    ("loft_prism", 0x74db_6889_4c07_172b),
    ("die_composed", 0xab17_b650_d73d_ea22),
    ("die_composed_tour", 0x4285_e851_34e0_a537),
    ("plate_param", 0xf4e8_8394_a29a_4348),
    ("kiss_carry", 0xbfa2_4a45_375c_3a04),
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
    ("reshaped_rod", 0x6bd0_82fa_e3bc_3bde),
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
