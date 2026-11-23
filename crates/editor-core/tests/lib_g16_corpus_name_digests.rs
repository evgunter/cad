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

/// FNV-1a 64 over every node's name rows, key-free and id-free
/// (`fixture::digest::name_rows`): each name, its node ids spelled by
/// place, with the cell it denotes read by its geometry.
fn digest(ev: &editor_core::Evaluation<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |s: &str| {
        for b in s.bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for (place, id) in ev.order.iter().enumerate() {
        feed(&format!("@node{place}"));
        for row in crate::fixture::digest::name_rows(ev, *id) {
            feed(&row);
            feed(";");
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
/// The digest reads each name's cell by its geometry and no arena key
/// or mint id, so a renumbered arena or a re-minted id moves no row; a
/// name that comes to denote another cell does.
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
/// chamfer mint two ids and their names differ in `node` alone; read by
/// place, those are the same, and the two rows differ in the cells the
/// blends leave.
/// `emit_fillet.rs`'s tie probe asserts the within-one-document case,
/// where the two nodes differ and the names must be disjoint.
///
/// A row moves when its document's names, or the cells they denote,
/// move, and a row is added with every document the registry gains.
/// The PR that moves a row says which documents moved and why.
/// [`every_corpus_documents_name_tables_are_golden`] reads it.
const PINNED: &[(&str, u64)] = &[
    ("die", 0x8b4f32f45c26070a),
    ("corner_table", 0xe3a87a6982bc6f1a),
    ("heat_sink", 0x63422fbb61ad8eb6),
    ("crossing_slots", 0xd65510445840f456),
    ("nested_islands_105", 0x8d5e2081324eb686),
    ("nested_islands_106_depth1", 0x07b1e27e0e69810a),
    ("nested_islands_106_depth2", 0x3b546f123501bf37),
    ("declared_tangency", 0x38bef38f6ffefb97),
    ("kitchen_sink", 0x79cdbbebeab4bb70),
    ("cut_cylinder", 0x310ab525a9a0312a),
    ("measured_web", 0xae9773711a98fc0a),
    ("boss_union", 0x1888bdc6ea9b9cb4),
    ("die_fillet", 0x08698875ca4bd22d),
    ("die_chamfer", 0x58ea63f980677f79),
    ("die_pips", 0x34c1d605e9487307),
    ("heat_sink_fins", 0x939c19aa07a4d2f8),
    ("die_tool", 0xbafef329379ab278),
    ("face_sketch", 0xcd361cba2bb30beb),
    ("part_select", 0xc4ceeea813f10014),
    ("symmetric_plate", 0xdf1929b1ed882a22),
    ("loft_prism", 0xac39b52502b5baac),
    ("die_composed", 0xab604fd94c44c473),
    ("die_composed_tour", 0x094442db396d5fa0),
    ("plate_param", 0xd2253e20335d09c9),
    ("kiss_carry", 0xdc2119ead2c033b7),
    ("tube_ring", 0xf6c2f84fb176dc19),
    ("tube_arc", 0xb702ecd65b1e6325),
    ("hollow_tube_elbow", 0x1cac9ccda83279c1),
    ("hollow_tube_ring", 0x44e374b4092520b5),
    ("reshaped_rod", 0x477a7f0adcf80a91),
    ("pin_in_bore", 0x7bf60819fcf9d2b4),
    ("axis_projection", 0x1a8e0ad2da16e02b),
    ("blocks_on_one_floor", 0xff47ce03eb1dde44),
    ("revolves_on_one_axis", 0x4765f13f6efd69de),
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
