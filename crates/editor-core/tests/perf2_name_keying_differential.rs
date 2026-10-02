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
    ("die", 0xd1cc_8c19_f124_cbbf, 0x2d79_2f4f_271d_c54f),
    ("corner_table", 0x7d05_432f_1a1a_4894, 0x5a00_496e_8787_6ae2),
    ("heat_sink", 0x28ba_50de_3b90_2084, 0xf7fd_83b0_fca3_370e),
    (
        "crossing_slots",
        0xc969_258a_0395_41bc,
        0x64b9_6d1e_933b_1c21,
    ),
    (
        "nested_islands_105",
        0x35a9_67b0_0673_48b7,
        0x45d0_4774_d079_8936,
    ),
    (
        "nested_islands_106_depth1",
        0x069a_20c1_c82a_2218,
        0xd2b2_0938_bb16_d76e,
    ),
    (
        "nested_islands_106_depth2",
        0x3a71_1832_a97b_e1f0,
        0xbe0d_704b_4ce9_0d95,
    ),
    (
        "declared_tangency",
        0xc90d_58b7_5c3f_875e,
        0x4e69_10fc_715e_ef89,
    ),
    ("kitchen_sink", 0x1159_6625_1d01_4179, 0xf50e_0207_a703_3430),
    ("cut_cylinder", 0x5f67_3a87_e7ea_6984, 0x6239_1f8f_1538_f8db),
    ("measured_web", 0x2a96_962a_ee9a_d45b, 0xedbc_0df0_0920_68cf),
    ("boss_union", 0x31bb_b49d_aa55_2605, 0x6837_db81_1849_c2d6),
    ("die_fillet", 0x80bc_cae8_01f5_7708, 0xc6ee_3eee_4b29_ce94),
    ("die_chamfer", 0x8fc8_1dcb_9e22_0258, 0x2447_c27a_1e7f_2a10),
    ("die_pips", 0xafdb_45e8_5729_c5df, 0x5456_2592_0506_43c9),
    (
        "heat_sink_fins",
        0xb4a5_1dc7_e10b_6d18,
        0x1b6d_a629_1389_38da,
    ),
    ("die_tool", 0xa2e3_9e62_cdc3_3f64, 0x352a_cfc6_b9df_8535),
    ("face_sketch", 0xf41b_1f6b_e4f0_070c, 0x5144_bc17_7e78_f138),
    ("part_select", 0x6d5d_93b0_a764_b3aa, 0xb98b_a0c8_9d26_1cf4),
    ("loft_prism", 0x257f_85ed_5c45_9334, 0x0158_faf3_57ab_36cd),
    ("die_composed", 0xbc3a_76d0_0ee1_322e, 0x9074_f839_0457_7d98),
    (
        "die_composed_tour",
        0xd829_7f54_7c29_ed25,
        0xc902_de7e_e30b_f314,
    ),
    ("plate_param", 0x7f28_a99f_b924_0164, 0x9b2a_d3b8_b103_6f22),
    ("kiss_carry", 0x89fd_2a34_ccc4_52ce, 0x089c_3648_e9ce_f31e),
    ("tube_ring", 0x09e2_09f2_3e9c_24d2, 0x589c_d491_67f6_6790),
    ("tube_arc", 0xfc46_548d_4b70_1217, 0x2f21_9c2d_495b_8fe9),
    (
        "hollow_tube_elbow",
        0xbac8_efd5_93ce_013d,
        0x2957_fc24_ade4_c889,
    ),
    (
        "hollow_tube_ring",
        0x6667_e42e_de38_a3fe,
        0x95e1_b5c1_fa10_8ede,
    ),
    // The first persisted `SetProgram` in the tree: its text digest
    // is the first taken over a log holding one.
    ("reshaped_rod", 0xc6ae_3f70_c2e9_2fa4, 0x058b_6cb5_09cb_206c),
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
