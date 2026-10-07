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
        let Some(v) = ev.value(*id) else { continue };
        for (n, e) in v.name_table.iter() {
            h.feed(&format!("fwd {n:?}={e:?};"));
        }
        for (n, e) in v.name_table.iter() {
            for ent in match e {
                editor_core::names::Entry::Unique(one) => vec![*one],
                editor_core::names::Entry::Tied(many) => many.clone(),
            } {
                h.feed(&format!(
                    "rev {ent:?}={:?};",
                    v.name_table.name_of(&ent).map(|n| format!("{n:?}"))
                ));
            }
            let _ = n;
        }
    }
    h.0
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
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 0x5c9ef047b273d5d4, 0x50b5f9d528f59a8b),
    ("corner_table", 0xacacce09efbb137e, 0x8f4d1bd35d9bdfc2),
    ("heat_sink", 0x22689cae1eaf11f2, 0x2c8643d66aee5fae),
    ("crossing_slots", 0xaf52769734b8cef4, 0xec49ab486ff5fdba),
    ("nested_islands_105", 0xe02ce72d10991cfe, 0x47b7a77daaad577f),
    (
        "nested_islands_106_depth1",
        0xa6437a628271e39f,
        0x2d82c8ca2451e73a,
    ),
    (
        "nested_islands_106_depth2",
        0x5b29afd4f3605714,
        0x020ccf3fe34623b6,
    ),
    ("declared_tangency", 0x67d47c203ff81b7c, 0xacf4c4fa7a636f36),
    ("kitchen_sink", 0x0ccfd4bb64094a2a, 0x2bf948bcb3c0c6a6),
    ("cut_cylinder", 0xd714ca44afe8f765, 0xaa8f34414d946441),
    ("measured_web", 0x98d21c45d5df0e04, 0xdcafdb68882eaf21),
    ("boss_union", 0xcaef6fafd6ad6df9, 0xc797d4025fe0cd84),
    ("die_fillet", 0x67603beb1b6e3940, 0x0d0057acbf46a045),
    ("die_chamfer", 0x77dc4e1b4a7b3c89, 0x11d7515bcfaeb0cc),
    ("die_pips", 0xfe214d7a8daf7d72, 0x54538e07b320410f),
    ("heat_sink_fins", 0xe4cfc859b725f565, 0x13de350837f4c038),
    ("die_tool", 0x27faa9c36c1d9e08, 0x160f6f8cc0feb7c2),
    ("face_sketch", 0xc7768e937f268fa0, 0x64c8d61c0457da80),
    ("part_select", 0x2d22815beaf59faf, 0x7807b28f61e1a58c),
    ("loft_prism", 0x9171966a14aba8a3, 0x9b97831e6cc32eb7),
    ("die_composed", 0x28c8b2ee6a253a9d, 0x4b8b2c410bf0deff),
    ("die_composed_tour", 0x9877f22e915d6dbf, 0xc77f26a74ca0517a),
    ("plate_param", 0x79964eb05ca55fbd, 0xbc15e7f652915e2a),
    ("kiss_carry", 0x444bc4d37d8298fd, 0x5375c22f1ea3f69a),
    ("tube_ring", 0x4acd213e945d0068, 0x8a1523a71529d363),
    ("tube_arc", 0x6baeae612405ba12, 0x3792295776a08510),
    ("hollow_tube_elbow", 0x52b99e411f2546d4, 0x753b926c5cbf2a3e),
    ("hollow_tube_ring", 0x759516eee400e94f, 0x10e9faa4ef320aaf),
    ("reshaped_rod", 0xfcbee1a3a44f483d, 0x50f587f763476cba),
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
