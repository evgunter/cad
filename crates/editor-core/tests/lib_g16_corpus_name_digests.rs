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
    ("die", 0x80fb24a6f1ae9ba2),
    ("corner_table", 0x08907ad083e7cdfb),
    ("heat_sink", 0x18913f722d28ae07),
    ("crossing_slots", 0xfd6f240f452bcee0),
    ("nested_islands_105", 0x32d2c78368ef35fc),
    ("nested_islands_106_depth1", 0xb47e07297675a420),
    ("nested_islands_106_depth2", 0xd8550b1d0c9ed5c7),
    ("declared_tangency", 0xca14cb18cffacb24),
    ("kitchen_sink", 0x1a5debaa269a544b),
    ("cut_cylinder", 0x22a0d03fcb78332b),
    ("measured_web", 0x6962a39b5e2dc962),
    ("boss_union", 0xe9fcaaa450bac745),
    ("die_fillet", 0xdb8817442b7968f9),
    ("die_chamfer", 0xdb8817442b7968f9),
    ("die_pips", 0x12e9579484bded0f),
    ("heat_sink_fins", 0xce2cfbcca8976ae0),
    ("die_tool", 0x37d3cb415ee63ff6),
    ("face_sketch", 0x19e0a31da991f68f),
    ("part_select", 0x95abbfc3d7d34c69),
    ("symmetric_plate", 0x9268c17572a48e9a),
    ("loft_prism", 0x0f4f5fb2ac6ac7ac),
    ("die_composed", 0xdfdcbea055159d9d),
    ("die_composed_tour", 0x41726d4e2a284da9),
    ("plate_param", 0x89d1d5f269b1e2cf),
    ("kiss_carry", 0xc70e5bde21f51896),
    ("tube_ring", 0xb33f785634fb18d3),
    ("tube_arc", 0x241735bd4f9c6219),
    ("hollow_tube_elbow", 0xebcf59f4b929c23d),
    ("hollow_tube_ring", 0xc8b8154003bbf363),
    ("reshaped_rod", 0xbadc7b4caa3f991a),
    ("pin_in_bore", 0x608f7fa16579fe28),
    ("axis_projection", 0x6b49b2843959b06d),
    ("blocks_on_one_floor", 0x8a79301884779812),
    ("revolves_on_one_axis", 0x22ae7f88d671b250),
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
