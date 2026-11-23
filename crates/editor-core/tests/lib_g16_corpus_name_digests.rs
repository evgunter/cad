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

/// FNV-1a 64 over every node's names by what they say
/// (`fixture::digest::name_shapes`): each name, its node ids spelled by
/// place, with its entry's shape — no arena key, no mint id and no
/// geometry.
fn digest(ev: &editor_core::Evaluation<f64>) -> u64 {
    let mut h = crate::fixture::digest::Fnv::new();
    for (place, id) in ev.order.iter().enumerate() {
        h.feed(format!("@node{place}").as_bytes());
        for row in crate::fixture::digest::name_shapes(ev, *id) {
            h.feed(row.as_bytes());
            h.feed(b";");
        }
    }
    h.finish()
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
/// The digest reads each name by what it says and its entry's shape,
/// with no arena key, mint id or geometry, so a renumbered arena, a
/// re-minted id or a moved body moves no row; a renamed role, a new
/// name or a new tie does. Which cell a name denotes is the seat
/// digests' (`fixture::digest::digest`), which read it by its geometry.
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
/// place, those are the same names.
/// `emit_fillet.rs`'s tie probe asserts the within-one-document case,
/// where the two nodes differ and the names must be disjoint.
///
/// A row moves when its document's names move, and a row is added with every document the registry gains.
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
    ("declared_tangency", 0x801bb472bc13a60f),
    ("kitchen_sink", 0x60a8cfec2a0736da),
    ("cut_cylinder", 0x2b1280e5d2009904),
    ("measured_web", 0x0e5af2c65b20a2d4),
    ("boss_union", 0x5c87046d85b46fa4),
    ("die_fillet", 0x60ec1e008f9a7ffd),
    ("die_chamfer", 0x58ea63f980677f79),
    ("die_pips", 0xac7ac80084e99f97),
    ("heat_sink_fins", 0x939c19aa07a4d2f8),
    ("die_tool", 0x4b864bc39047c7d8),
    ("face_sketch", 0xcd361cba2bb30beb),
    ("part_select", 0xc4ceeea813f10014),
    ("symmetric_plate", 0xdf1929b1ed882a22),
    ("loft_prism", 0xac39b52502b5baac),
    ("die_composed", 0x0efd880b8498d209),
    ("die_composed_tour", 0x8d5219bde5b95a24),
    ("plate_param", 0x4440fa5ac5a1f585),
    ("kiss_carry", 0xdc2119ead2c033b7),
    ("tube_ring", 0xb02294432b000ded),
    ("tube_arc", 0x116fd349c121b24d),
    ("hollow_tube_elbow", 0x2c07d9c69f7f62ed),
    ("hollow_tube_ring", 0x6bc98e0b162fc4e1),
    ("reshaped_rod", 0x9258d736e1a72d63),
    ("pin_in_bore", 0x7ab4623fecf03f7a),
    ("axis_projection", 0x49578952ed3b7ad3),
    ("blocks_on_one_floor", 0xff47ce03eb1dde44),
    ("revolves_on_one_axis", 0x4f4f95f1259b45fc),
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
