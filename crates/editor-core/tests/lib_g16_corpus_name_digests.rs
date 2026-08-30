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

mod corpus;
mod fixture;

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
/// edges named through twenty pairwise unions, in one pin.
///
/// Two more rows are worth a reader's second look, and neither is a bug.
///
/// `die` is `0x8d2e4c613057071e` — the same number
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
    ("die", 0x8d2e_4c61_3057_071e),
    ("corner_table", 0x9a0f_6669_27a4_02f7),
    ("heat_sink", 0x348d_1478_bcea_6a5e),
    ("crossing_slots", 0xcd4a_536d_9410_2d32),
    ("nested_islands_105", 0x4f21_dd91_9b3f_cf12),
    ("nested_islands_106_depth1", 0x9655_e39f_84e3_f037),
    ("nested_islands_106_depth2", 0x499e_b7b3_2443_6f91),
    ("declared_tangency", 0xdef9_76c6_1d04_25bb),
    ("kitchen_sink", 0xc624_18ea_8ee0_e5aa),
    ("cut_cylinder", 0xc461_0a86_1d7e_379b),
    ("measured_web", 0x9a4c_06c4_6086_685b),
    ("boss_union", 0xefcb_deb5_ef3a_3873),
    ("die_fillet", 0xb6ab_9ad5_a321_15f2),
    ("die_chamfer", 0xb6ab_9ad5_a321_15f2),
    ("die_pips", 0x4116_0291_2c74_aa6d),
    ("heat_sink_fins", 0xae39_7800_c351_3248),
    ("die_tool", 0x9e24_4be7_b06b_9a40),
    ("loft_prism", 0x7318_e99f_2b22_dafb),
    ("die_composed", 0xc801_9fd6_e360_ce3e),
    ("die_composed_tour", 0x2813_3d38_1e83_f02d),
    ("plate_param", 0x3bfe_3e78_5eec_a227),
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
