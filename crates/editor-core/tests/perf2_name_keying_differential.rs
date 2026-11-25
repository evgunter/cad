//! **The naming differential**: per corpus document, ONE digest of every
//! node's name table in BOTH directions and ONE digest of the document's
//! persisted text — the pair a change to how names are KEYED must not
//! move.
//!
//! # What it covers that the neighbours do not
//!
//! `lib_g16_corpus_name_digests` digests the FORWARD table (name → entry)
//! of every registered document, which is the surface an emitter change
//! moves. It says nothing about the REVERSE direction (entity → name),
//! and the reverse map is what hit-testing reads, so a change that
//! re-keys the table can leave every forward row intact and still hand a
//! different name back for an entity. This file's first digest feeds both
//! directions of every node's table, so the two maps are pinned as a
//! pair rather than one of them twice.
//!
//! The second digest is the other half of the same claim. A
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
//! It is a golden in the ordinary sense (`docs/prompts/implementer-discipline.md`
//! §3): the question is whether the NEW names are right, never how to
//! get the old number back. A document added to or removed from the
//! registry moves its own row and no other.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::persist;
use editor_core::test_support::sans_epsilon;
use geom_core::Tol;

use crate::corpus;

/// FNV-1a 64 — the same mixer `m4_pr3_names_ci` and
/// `lib_g16_corpus_name_digests` use, so a number here is comparable
/// with a number there by construction rather than by convention.
struct Fnv(u64);

impl Fnv {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }

    fn feed(&mut self, s: &str) {
        for b in s.bytes() {
            self.0 ^= u64::from(b);
            self.0 = self.0.wrapping_mul(0x1000_0000_01b3);
        }
    }
}

/// Both directions of every node's table, in evaluation order.
///
/// The reverse direction is fed through [`editor_core::names::NameTable::name_of`]
/// over the forward table's own entities, so a row that answers in one
/// direction and not the other moves the digest instead of being
/// silently skipped.
fn tables_digest(ev: &editor_core::Evaluation<f64>) -> u64 {
    let mut h = Fnv::new();
    for id in &ev.order {
        h.feed(&format!("#{id:?}"));
        for t in crate::fixture::tables(ev, *id) {
            perf2_table_digest(&mut h, t);
        }
    }
    h.0
}

/// One table's rows into `h`, forward then reverse.
fn perf2_table_digest(h: &mut Fnv, t: &editor_core::NameTable) {
    for (n, e) in t.iter() {
        h.feed(&format!("fwd {n:?}={e:?};"));
    }
    for (_, e) in t.iter() {
        for ent in match e {
            editor_core::names::Entry::Unique(one) => vec![*one],
            editor_core::names::Entry::Tied(many) => many.clone(),
        } {
            h.feed(&format!(
                "rev {ent:?}={:?};",
                t.name_of(&ent).map(|n| format!("{n:?}"))
            ));
        }
    }
}

/// `(document, both-direction name digest, persisted-text digest)`.
///
/// A row's first word moves with its document's names and ids, its
/// second with its persisted text, and a row is added with every
/// document the registry gains. The PR that moves a row says which
/// documents moved and why.
/// [`every_corpus_documents_name_tables_and_persisted_text_are_pinned`]
/// reads it.
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 0xecd52c78fea995a5, 0x591ac2ad8feccdf6),
    ("corner_table", 0x4e034da469bc27c2, 0x2e345b0dccdd4c24),
    ("heat_sink", 0x6b9870e7cdf512fb, 0x9c1db942bfc15943),
    ("crossing_slots", 0x1d1db1973ecea6d5, 0xe7e67940c2c33db5),
    ("nested_islands_105", 0xcdcfa201f726b886, 0x3b6e3e51c3dcfbca),
    (
        "nested_islands_106_depth1",
        0xa18f3b3003aa1f53,
        0xa372797c8f7d003a,
    ),
    (
        "nested_islands_106_depth2",
        0x4d05c343f5465689,
        0xbdd86e4801c44254,
    ),
    ("declared_tangency", 0x02afb71c6f997726, 0xf9697861397eb937),
    ("kitchen_sink", 0x88803e8bbf21187b, 0x8168f61c6f47fce7),
    ("cut_cylinder", 0xd97c080b5a539bbb, 0x193416e7984f6d8e),
    ("measured_web", 0xbc08efa7206efdd2, 0xae7444dea631555e),
    ("boss_union", 0x5b69aa2230051b34, 0x4cdf20eb05c5c85f),
    ("die_fillet", 0x5aa9c8b27756477a, 0xbff1369360505793),
    ("die_chamfer", 0x6b467646ba746fd3, 0x302a655fce2975cc),
    ("die_pips", 0xab1993865ca107a7, 0x490310648b48f9b8),
    ("heat_sink_fins", 0x3a7bf5baaa100e36, 0xbc6967bf628b4e61),
    ("die_tool", 0xae4e6ea460bfec6c, 0x1167d41d3a76c3b1),
    ("face_sketch", 0x66a7e13b6f73e1bb, 0xf42662938388297d),
    ("part_select", 0x3bb5821be1122397, 0x9fac3f88d00aa799),
    ("symmetric_plate", 0xe4fad153d1b0cdb3, 0x18e24c09ce1be59c),
    ("loft_prism", 0xb08a6a63c44a8590, 0xfa174829a70971f4),
    ("die_composed", 0x0ff2f36548f1e595, 0x452d7508ac07a3da),
    ("die_composed_tour", 0xa86b6f54352bb179, 0x6cc4d6d010619909),
    ("plate_param", 0xb7ba781c4227180e, 0x60c6cebb721190be),
    ("kiss_carry", 0x64c8785488d77a3f, 0xa50cf2d76a2f2369),
    ("tube_ring", 0x9bb47234553e3786, 0x03867bbe79cf2a84),
    ("tube_arc", 0xef8fdffb339a0c0b, 0xcdfea7dc50e83443),
    ("hollow_tube_elbow", 0xf47cb8f1a229d3c3, 0x786e99c0281f296b),
    ("hollow_tube_ring", 0x33ed8cef64654256, 0x6a440ece2cb63056),
    ("reshaped_rod", 0x1259bae421f75f0b, 0x022747dedfbeb84c),
    ("pin_in_bore", 0x70ec03aec6b0267f, 0x6841e68569e65434),
    ("axis_projection", 0xff397bc6fe0bf56c, 0xc532d4c258eb0aa8),
    (
        "blocks_on_one_floor",
        0x9173dd51814c576d,
        0x5ade445df825d7b4,
    ),
    (
        "revolves_on_one_axis",
        0xa16460f006abd44d,
        0x34e0902b0883c79c,
    ),
];

#[test]
fn every_corpus_documents_name_tables_and_persisted_text_are_pinned() {
    let tol = Tol::witness();
    let got: Vec<(String, u64, u64)> = corpus::documents()
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
            let mut h = Fnv::new();
            h.feed(&sans_epsilon(&text));
            (d.name.to_owned(), tables_digest(&ev), h.0)
        })
        .collect();
    let want: Vec<(String, u64, u64)> = PINNED
        .iter()
        .map(|(n, a, b)| ((*n).to_owned(), *a, *b))
        .collect();
    let render = |v: &[(String, u64, u64)]| {
        v.iter()
            .map(|(n, a, b)| format!("    (\"{n}\", 0x{a:016x}, 0x{b:016x}),"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(
        got,
        want,
        "the corpus's name tables or its persisted text moved. Decide \
         whether the NEW names are right; if they are, this is the fresh \
         table:\n{}",
        render(&got)
    );
}
