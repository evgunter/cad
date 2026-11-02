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
/// The digest encodes each name's entity with its slot VERSION, so a
/// change in how many keys an arena has spent before a mint moves a
/// row with no name changing. A graft spends one key per dead
/// ancestor its forwarded records name (`topo`'s combine door), which
/// is why the documents whose booleans graft a split operand carry
/// versions that no name reads.
///
/// `die_composed` and `die_composed_tour` are the only registered
/// documents that carve a CLOSED chain, so they are the only two whose
/// tables carry the rim-phase roles at all (eight band-trim names and
/// 168 respectively, each said in the blend's table and its world copy's; every other row's tables have none). A
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
/// A row moves when its document's names or the ids they are keyed by
/// move, and a row is added with every document the registry gains;
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// tells a move of names and ids alone from one of geometry. The PR
/// that moves a row says which documents moved and why.
const PINNED: &[(&str, u64)] = &[
    ("die", 0x842c07a9245e3188),
    ("corner_table", 0x9f694f6348a96dd6),
    ("heat_sink", 0x7956_4324_d870_c32d),
    ("crossing_slots", 0xe8dc66f2cb5166ab),
    ("nested_islands_105", 0x15d51d7c25dfe6df),
    ("nested_islands_106_depth1", 0x4677e01cb07a25ee),
    ("nested_islands_106_depth2", 0x664fe6e3cc64f8a7),
    ("declared_tangency", 0x10186938bc97ccdd),
    ("kitchen_sink", 0xfef3_da00_18d0_b020),
    ("cut_cylinder", 0xa78c6dd27d0a7322),
    ("measured_web", 0xa60a9fdbd8576659),
    ("boss_union", 0x60c0307b55a8a5cc),
    ("die_fillet", 0xd97598cb5c09a14e),
    ("die_chamfer", 0x5cca19013fc9edde),
    ("die_pips", 0xf0816cf81d5e9447),
    ("heat_sink_fins", 0xb4a7_785a_a697_7bd9),
    ("die_tool", 0xe198_b7d8_af7e_a0f0),
    ("face_sketch", 0xd0c00189b535698d),
    ("part_select", 0xc004_f04b_3c3b_bbd7),
    ("symmetric_plate", 0xab53c12868591a5e),
    ("loft_prism", 0xae293089cc9c899a),
    ("die_composed", 0x790583813e1af47c),
    ("die_composed_tour", 0xebe1a3b276f44b4b),
    ("plate_param", 0xee0a9a914d168a89),
    ("kiss_carry", 0x311483e921dfa423),
    ("tube_ring", 0x36e4e9fd72573d79),
    ("tube_arc", 0x27e56612927be491),
    ("hollow_tube_elbow", 0x910cc66850c7e31f),
    ("hollow_tube_ring", 0xaffef3b5e782e58d),
    ("reshaped_rod", 0x408e1fae110b5bc4),
    ("pin_in_bore", 0x4efadfbeeee4da76),
    ("axis_projection", 0xc9886636e6ced645),
    ("blocks_on_one_floor", 0x47c47127b70fd891),
    ("revolves_on_one_axis", 0x4072475117d1226a),
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
