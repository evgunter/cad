//! **Per-document NAME-TABLE digests over the whole corpus registry,
//! pinned** — adopted from the LIB-G16 R2 review probe, which printed
//! them for a cross-tree comparison and so expired with it
//! (`memories/test-suite-cost.md`).
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
/// **That number moved once, at DOCM-3, and it is the only row that
/// did.** The die's cutting tool is one `Node::Union` over its 21 pips
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
/// Two more rows are worth a reader's second look, and neither is a bug.
///
/// `die` is `0xd77331053f8a87fe` — the same number
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
/// `die_fillet` and `die_chamfer` are IDENTICAL, and that is what
/// RECIPE-DOORS D3 says should happen. The two documents are the same
/// three-node recipe with the blend swapped, so the blend mints under
/// the same node id (2) and — D3 having ruled the role vocabulary
/// shared — composes the same `RoleSeg`s off the same upstream names.
/// The names are therefore equal, which costs nothing: a `StableName`
/// is scoped to its own document, and WITHIN either document every
/// name is still unique. What distinguishes a chamfer's blend from a
/// fillet's is the minting node, and here the two nodes live in
/// different documents. `emit_fillet.rs`'s tie probe asserts the
/// within-one-document case, where the two nodes differ and the names
/// must be disjoint.
const PINNED: &[(&str, u64)] = &[
    ("die", 0xfdf3_d13d_4782_a4e5),
    ("corner_table", 0x9418_4d9f_f5bb_667e),
    ("heat_sink", 0xa29d_1f9f_979d_097e),
    ("crossing_slots", 0xcdd4_d506_fa1a_ab9b),
    ("nested_islands_105", 0xb7d4_5b86_990b_53a8),
    ("nested_islands_106_depth1", 0x56f3_2cd0_db15_c3fa),
    ("nested_islands_106_depth2", 0x81bf_a4ec_3d11_bd26),
    ("declared_tangency", 0xc64d_f2c5_c3b8_6599),
    // Moved by the in-plane revolve axis, and the ONLY row that did.
    // `kitchen_sink` shared one `Datum::Axis` between a circular
    // pattern and a revolve; those are two node kinds now — a pattern
    // turns a body about a world line, a revolve turns a sketch about a
    // line in its own plane — so the document authors both and every
    // node after the new one is renumbered.
    //
    // The three die documents mint their axis in the same edit as
    // before, one position later (after the frame it names), and a
    // datum mints no names, so the swap moves no id that any name
    // holds: their rows are byte-identical. That is what this
    // per-document instrument is for.
    ("kitchen_sink", 0x27bd_57ed_25e7_e4b3),
    ("cut_cylinder", 0x50ce_47ef_bede_96f7),
    ("measured_web", 0x57a8_bd3d_9ee5_80c8),
    ("boss_union", 0xd2f8_79b5_6cd2_0efa),
    ("die_fillet", 0x34ae_aabf_d65b_e042),
    ("die_chamfer", 0x34ae_aabf_d65b_e042),
    ("die_pips", 0xc1d7_f994_65ee_de1d),
    ("heat_sink_fins", 0x774b_b1fa_e9c3_ea5a),
    ("die_tool", 0x8842_6c6d_a225_7e5c),
    ("face_sketch", 0x9686_16cf_e0bc_7e15),
    // DOCM-2. Two `Part`s of one split and one of a pattern: the
    // projection mints nothing, so every name in the document is the
    // split's, the pattern's, or the union's over them, and the row's
    // arrival moved no other row.
    ("part_select", 0xef49_6789_be45_f431),
    ("loft_prism", 0x28f4_e9c8_5810_f1a9),
    ("die_composed", 0xf533_e226_f499_6617),
    ("die_composed_tour", 0x10a8_8610_60a2_49eb),
    ("plate_param", 0xf74f_e1b0_968d_d6e8),
    ("kiss_carry", 0x4c48_320f_0668_6632),
    // LIB-TUBE. Both tables are minted by `name_revolve` — the
    // tube doors return `Revolved<T>` and the emitter reads only
    // its maps — so these two rows are the revolve role vocabulary
    // over a body no revolve node built. Their arrival moved no
    // other row, which is the property this table exists to make
    // readable.
    ("tube_ring", 0x1293_7fbf_295c_f16c),
    ("hollow_tube_elbow", 0x98e4_97a0_679c_33ad),
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
