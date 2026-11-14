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

/// The saved text with its one `"epsilon":` line removed — the line CI's
/// eps rows sweep. Exactly one must be present: a missing or duplicated
/// ε line is damage, not sweep variance.
fn sans_epsilon(t: &str) -> String {
    let (kept, excluded): (Vec<&str>, Vec<&str>) = t
        .lines()
        .partition(|l| !l.trim_start().starts_with("\"epsilon\":"));
    assert_eq!(
        excluded.len(),
        1,
        "expected exactly one \"epsilon\" line, found {}",
        excluded.len()
    );
    kept.join("\n")
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
    ("die", 0xecd52c78fea995a5, 0x216bc661fe8d623f),
    ("corner_table", 0x4e034da469bc27c2, 0x7e9c3a46127def29),
    ("heat_sink", 0x6b9870e7cdf512fb, 0x3ef215a2407c3a78),
    ("crossing_slots", 0x1d1db1973ecea6d5, 0x9a1b58694bb4d50e),
    ("nested_islands_105", 0xcdcfa201f726b886, 0xb6e98a2fef9a7683),
    (
        "nested_islands_106_depth1",
        0xa18f3b3003aa1f53,
        0xa3a3df12e81fed73,
    ),
    (
        "nested_islands_106_depth2",
        0x4d05c343f5465689,
        0xa0efcd0678503b59,
    ),
    ("declared_tangency", 0x02afb71c6f997726, 0x6fed7a61290973ac),
    ("kitchen_sink", 0x88803e8bbf21187b, 0x30e98448f6842a9c),
    ("cut_cylinder", 0xd97c080b5a539bbb, 0x2c24c48cb0bec4a7),
    ("measured_web", 0xbc08efa7206efdd2, 0x023ae36db53ddcf7),
    ("boss_union", 0x5b69aa2230051b34, 0x6686109354925f04),
    ("die_fillet", 0x5aa9c8b27756477a, 0x1f533b6b0e9eec08),
    ("die_chamfer", 0x6b467646ba746fd3, 0xdd5ec23f7efcf701),
    ("die_pips", 0x46f1922773f45023, 0x8f3f511ba32772dd),
    ("heat_sink_fins", 0x3a7bf5baaa100e36, 0xc5adeae37c02cbca),
    ("die_tool", 0x075ec898e3aaa1e0, 0x6bf8a06c8a0be61a),
    ("face_sketch", 0x66a7e13b6f73e1bb, 0xa9e4d5eac3818d06),
    ("part_select", 0x3bb5821be1122397, 0xa3f6f7ba47cd0cd2),
    ("symmetric_plate", 0xe4fad153d1b0cdb3, 0x493f4e6599815211),
    ("loft_prism", 0xb08a6a63c44a8590, 0xed67b7055477b839),
    ("die_composed", 0xa22d8b5a800f3567, 0x249d4cb20c47da3d),
    ("die_composed_tour", 0x37df3208ce3cf433, 0xc0ef119fcbe3ff63),
    ("plate_param", 0xb7ba781c4227180e, 0x5b7084b281858497),
    ("kiss_carry", 0x64c8785488d77a3f, 0x10656ff0741413a2),
    ("tube_ring", 0x9bb47234553e3786, 0xfc38b9833a2d3549),
    ("tube_arc", 0xef8fdffb339a0c0b, 0x7a64482a3884b378),
    ("hollow_tube_elbow", 0xf47cb8f1a229d3c3, 0xafa9ef32292e5bf0),
    ("hollow_tube_ring", 0x33ed8cef64654256, 0x7048bf1fc3eab3df),
    ("reshaped_rod", 0x1259bae421f75f0b, 0xa00999df9f994a81),
    ("pin_in_bore", 0x6199b58675e64253, 0xcd19e6b8bf7b2d79),
    ("axis_projection", 0xff397bc6fe0bf56c, 0xc9a7e7679ebcc9cd),
    (
        "blocks_on_one_floor",
        0x9173dd51814c576d,
        0xb2e7d45448d07bf9,
    ),
    (
        "revolves_on_one_axis",
        0xa16460f006abd44d,
        0xd1fa6cb10cea6811,
    ),
];

#[test]
fn every_corpus_documents_name_tables_and_persisted_text_are_pinned() {
    let tol = Tol::witness();
    let got: Vec<(String, u64, u64)> = corpus::documents()
        .iter()
        .map(|d| {
            let ev = corpus::eval::<f64>(&d.doc);
            let text = persist::save(&d.doc, &[], tol)
                .unwrap_or_else(|e| panic!("{} saves: {e:?}", d.name));
            // Save → load → save is the file's own fixed point: a
            // representation change that survives serialization but not
            // deserialization lands here rather than on a user's disk.
            let back =
                persist::load(&text, tol).unwrap_or_else(|e| panic!("{} reloads: {e:?}", d.name));
            let again = persist::save(&back.snapshot, &back.edits, tol)
                .unwrap_or_else(|e| panic!("{} re-saves: {e:?}", d.name));
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
