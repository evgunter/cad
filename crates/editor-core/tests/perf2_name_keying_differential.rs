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
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 0x874a19bb0c1cb210, 0xfe1f3779173f909d),
    ("corner_table", 0x1538f5150ad6647a, 0xde5c69b920f6fdea),
    ("heat_sink", 0x5a65_8b2b_816b_abfb, 0x83ab_6af2_145b_adc8),
    ("crossing_slots", 0x531c69e7e9ff59e6, 0xd0f6429faf4510be),
    ("nested_islands_105", 0xa3260846d5b40f15, 0xe9af7ac8f708e855),
    (
        "nested_islands_106_depth1",
        0xa0864e8ffdbe000a,
        0xa830cba1e9f77225,
    ),
    (
        "nested_islands_106_depth2",
        0x1a43336a2d09e635,
        0x5c1219843db607a6,
    ),
    ("declared_tangency", 0x0e962063407e3b05, 0x0634581982b9dfd5),
    ("kitchen_sink", 0x54bb_19b3_5f10_b835, 0xdaa9_41f7_37ec_5c4f),
    ("cut_cylinder", 0x35a106126dc3aca0, 0xf746b3c7d6dda1f9),
    ("measured_web", 0x5f0b35b9a50ae2b4, 0xe039ab7aa5cad1da),
    ("boss_union", 0x25d270ae52dade43, 0x7311f8275a31e470),
    ("die_fillet", 0x8ddc8e8e4cc1cc18, 0x44273f0b972dadee),
    ("die_chamfer", 0x4d9e144d4e8be9b4, 0xed273f48cb5ca907),
    ("die_pips", 0x46ff4311b0616fbd, 0x5e8871c705b54729),
    (
        "heat_sink_fins",
        0x33cf_9cd2_a9d5_348e,
        0xfac2_6885_ef93_a89f,
    ),
    ("die_tool", 0x346c_da78_24bf_a878, 0x814d_4779_2822_0e71),
    ("face_sketch", 0x1a32703bef525f6b, 0x3a0976a80e3f9cbf),
    ("part_select", 0xfc2d_3fc5_a314_fab7, 0xe6a1_919f_be71_88de),
    ("symmetric_plate", 0x4481fd1b6f905ec1, 0xd50cfd94a5cbf8ea),
    ("loft_prism", 0xf84090b7955f6cd0, 0x9d1660151c8302a0),
    ("die_composed", 0xec47aa7753cf019c, 0xc69608fcaa2c548d),
    ("die_composed_tour", 0xfabd8792f7a5faea, 0xa5f5807ff6b4b3c3),
    ("plate_param", 0xaa331954865f8d22, 0x9ee69eeee04c353b),
    ("kiss_carry", 0x92f333305a33713c, 0xd4cd5f6b57205f26),
    ("tube_ring", 0x56fd2415a347d921, 0xab53801334f7ed9b),
    ("tube_arc", 0x3bcb162e9e7923d5, 0x299b0f3445b95489),
    ("hollow_tube_elbow", 0xdbf97cdac2df4d30, 0x88b19605f5015b9c),
    ("hollow_tube_ring", 0xf0a59d29f0a5ef21, 0x743a5c3e6051831a),
    ("reshaped_rod", 0xd37dd1691945dda0, 0x40e1518b88d07303),
    ("pin_in_bore", 0x70c706ddcfaafe79, 0xfc906cdc112c4e68),
    ("axis_projection", 0x9a503b2d07b5e7ba, 0x25ace293e80616db),
    (
        "blocks_on_one_floor",
        0xe3698f46a60e0cf7,
        0x1118cf3b11e89b46,
    ),
    (
        "revolves_on_one_axis",
        0x1a95665af34466c7,
        0xecaa35ae22f6a002,
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
