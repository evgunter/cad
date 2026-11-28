//! **The naming differential**: per corpus document, every node's name
//! table read in BOTH directions, and ONE digest of the document's
//! persisted text — the pair a change to how names are KEYED must not
//! move.
//!
//! # What it covers that the neighbours do not
//!
//! `lib_g16_corpus_name_digests` digests the FORWARD table (name → entry)
//! of every registered document, which is the surface an emitter change
//! moves. It says nothing about the REVERSE direction (entity → name),
//! and the reverse map is what hit-testing reads, so a change that
//! re-keys the table can leave every forward row intact and still hand
//! back, for an entity, a name that does not denote it. This file reads
//! every entity of every forward row back through the reverse map and
//! asks that the name it gets denotes that entity.
//!
//! The digest is the other half of the same claim. A
//! [`editor_core::names::StableName`] is PERSISTED — declarations and the
//! appearance store carry names into the saved text, structurally — so a
//! representation change is observable on disk as well as in memory. The
//! row saves every registered document, pins the text's digest, and
//! reloads and re-saves it to assert the file round-trips to the same
//! bytes.
//!
//! ε is the ONE line excluded from the persisted digest, for the reason
//! `pncad::tests::plate_param_authors_facade_only_and_its_saved_text_is_pinned`
//! states: CI's eps rows sweep the ambient tolerance by design, so the
//! `"epsilon":` line legitimately varies per run while every other byte
//! must not.
//!
//! # What it does NOT pin
//!
//! Cost. The keying this row was written to hold still was changed so
//! that the naming table stops paying descent depth, and what is left
//! is Θ(names emitted) at roughly 0.7 µs per named entity — short of
//! the share the change was aimed at, and tracked as
//! `naming-a-boolean-chain-is-theta-names-per-step` with the three
//! leads that would close it. This file is the correctness half: it
//! says the names did not move, and says nothing about what they cost.
//!
//! # When one moves
//!
//! The text digest is a golden in the ordinary sense
//! (`docs/prompts/implementer-discipline.md` §3): the question is
//! whether the NEW text is right, never how to get the old number back.
//! A document added to or removed from the registry moves its own row
//! and no other.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::persist;
use editor_core::test_support::sans_epsilon;
use geom_core::Tol;

use crate::corpus;

/// Every node's table read both ways: each entity of each forward row,
/// handed to [`editor_core::names::NameTable::name_of`], answers a name
/// whose forward row holds that entity. Returns the rows that do not.
fn reverse_misreads(ev: &editor_core::Evaluation<f64>) -> Vec<String> {
    let mut wrong = Vec::new();
    for id in &ev.order {
        for t in crate::fixture::tables(ev, *id) {
            for (n, e) in t.iter() {
                let entities = match e {
                    editor_core::names::Entry::Unique(one) => vec![*one],
                    editor_core::names::Entry::Tied(many) => many.clone(),
                };
                for ent in entities {
                    let back = t.name_of(&ent);
                    let holds = back.and_then(|b| t.lookup(b)).is_some_and(|e| match e {
                        editor_core::names::Entry::Unique(one) => *one == ent,
                        editor_core::names::Entry::Tied(many) => many.contains(&ent),
                    });
                    if !holds {
                        wrong.push(format!("{n:?} names {ent:?}, which reads back as {back:?}"));
                    }
                }
            }
        }
    }
    wrong
}

/// `(document, persisted-text digest)`.
///
/// A row moves with its document's persisted text, and a row is added
/// with every document the registry gains. The PR that moves a row says which
/// documents moved and why.
/// [`every_corpus_documents_names_read_back_and_its_persisted_text_is_pinned`]
/// reads it.
const PINNED: &[(&str, u64)] = &[
    ("die", 0x2054dfbda20efbd6),
    ("corner_table", 0x2e345b0dccdd4c24),
    ("heat_sink", 0x3cc2_a010_a541_a4bb),
    ("crossing_slots", 0x2b824fc243a3ab59),
    ("nested_islands_105", 0x6935a2dd3d8972f3),
    ("nested_islands_106_depth1", 0xe3abcbda630addfd),
    ("nested_islands_106_depth2", 0x9d72d939da0fde76),
    ("declared_tangency", 0xf9697861397eb937),
    ("kitchen_sink", 0x5d97_8ba9_76e7_b29b),
    ("cut_cylinder", 0x193416e7984f6d8e),
    ("measured_web", 0xae7444dea631555e),
    ("boss_union", 0x4cdf20eb05c5c85f),
    ("die_fillet", 0xbff1369360505793),
    ("die_chamfer", 0x302a655fce2975cc),
    ("die_pips", 0x91d7debf3d330210),
    ("heat_sink_fins", 0x5d40_8a1e_31f3_74aa),
    ("die_tool", 0xe77fe13851e627a8),
    ("face_sketch", 0xf42662938388297d),
    ("part_select", 0x61af_eff2_5d8e_245f),
    ("symmetric_plate", 0x18e24c09ce1be59c),
    ("loft_prism", 0xfa174829a70971f4),
    ("die_composed", 0x4bb78d3b3dc98a48),
    ("die_composed_tour", 0x5a31478c50d79e90),
    ("plate_param", 0x60c6cebb721190be),
    ("kiss_carry", 0xa50cf2d76a2f2369),
    ("tube_ring", 0x03867bbe79cf2a84),
    ("tube_arc", 0xcdfea7dc50e83443),
    ("hollow_tube_elbow", 0x786e99c0281f296b),
    ("hollow_tube_ring", 0x6a440ece2cb63056),
    ("reshaped_rod", 0x022747dedfbeb84c),
    ("pin_in_bore", 0x6841e68569e65434),
    ("axis_projection", 0xc532d4c258eb0aa8),
    ("blocks_on_one_floor", 0x5ade445df825d7b4),
    ("revolves_on_one_axis", 0x34e0902b0883c79c),
];

#[test]
fn every_corpus_documents_names_read_back_and_its_persisted_text_is_pinned() {
    let tol = Tol::witness();
    let got: Vec<(String, u64)> = corpus::documents()
        .iter()
        .map(|d| {
            let ev = corpus::eval::<f64>(&d.doc);
            let text =
                persist::save(&d.doc, tol).unwrap_or_else(|e| panic!("{} saves: {e:?}", d.name));
            // Save → load → save is the file's own fixed point: a
            // representation change that survives serialization but not
            // deserialization lands here rather than on a user's disk.
            let back =
                persist::load(&text, tol).unwrap_or_else(|e| panic!("{} reloads: {e:?}", d.name));
            let again =
                persist::save(&back, tol).unwrap_or_else(|e| panic!("{} re-saves: {e:?}", d.name));
            assert_eq!(
                text, again,
                "{} does not round-trip to the same bytes",
                d.name
            );
            let wrong = reverse_misreads(&ev);
            assert!(
                wrong.is_empty(),
                "{}: the reverse map hands back a name that does not denote the entity:\n{}",
                d.name,
                wrong.join("\n")
            );
            let mut h = crate::fixture::digest::Fnv::new();
            h.feed(sans_epsilon(&text).as_bytes());
            (d.name.to_owned(), h.finish())
        })
        .collect();
    let want: Vec<(String, u64)> = PINNED.iter().map(|(n, b)| ((*n).to_owned(), *b)).collect();
    let render = |v: &[(String, u64)]| {
        v.iter()
            .map(|(n, b)| format!("    (\"{n}\", 0x{b:016x}),"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(
        got,
        want,
        "the corpus's persisted text moved. Decide whether the NEW text is \
         right; if it is, this is the fresh table:\n{}",
        render(&got)
    );
}
