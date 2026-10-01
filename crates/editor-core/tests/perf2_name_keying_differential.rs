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
    ("die", 0x4274_91ac_2352_4321, 0x00fd_dbbe_eb97_fc70),
    ("corner_table", 0xf97c_c313_eaf8_7597, 0x4335_723f_1d58_d318),
    ("heat_sink", 0xb031_55ad_d37a_ec57, 0x71fa_50bb_c29e_9bd3),
    (
        "crossing_slots",
        0x031d_d12a_6e32_316e,
        0x6ba6_ebbc_f121_3524,
    ),
    (
        "nested_islands_105",
        0xa1f4_9aec_e8e6_21b0,
        0x5398_04be_08ed_9967,
    ),
    (
        "nested_islands_106_depth1",
        0xd5d6_5d4c_87a5_eb0f,
        0x728b_2ee3_1976_4b4e,
    ),
    (
        "nested_islands_106_depth2",
        0xc888_c6a0_6cca_30db,
        0x1269_f48b_65d7_840d,
    ),
    (
        "declared_tangency",
        0x79c8_1fb5_5df5_cadb,
        0xe304_39af_b2c6_6981,
    ),
    ("kitchen_sink", 0x4698_9df3_f908_979f, 0x14e0_0b90_cd2d_344a),
    ("cut_cylinder", 0x4606_3982_c4a7_8013, 0x6ca9_c180_a8f2_d5d7),
    ("measured_web", 0x0292_25ef_fc59_32f8, 0x9520_6dbb_eae1_676f),
    ("boss_union", 0x332e_5c74_d979_dc07, 0x03e8_7e71_7ff8_83c1),
    ("die_fillet", 0x5768_aa33_1646_f3f6, 0x3f5a_5812_d028_4be5),
    ("die_chamfer", 0x0f92_58a8_2d15_cdf6, 0x0d0b_45a7_c8c8_ec64),
    ("die_pips", 0x8ea9_6349_4a37_247e, 0x5ebe_7620_2b94_f8b6),
    (
        "heat_sink_fins",
        0x301d_1832_d686_695b,
        0x80fc_54a1_6c48_1499,
    ),
    ("die_tool", 0xfda1_087d_7ee7_3893, 0x9dd9_4c3c_683b_be0f),
    ("face_sketch", 0xe0e2_897c_def1_b44f, 0x5c64_045f_7c3c_f592),
    ("part_select", 0x1dc0_86ec_f1c0_5f3d, 0x0631_355c_5de1_1fa0),
    ("loft_prism", 0x257f_85ed_5c45_9334, 0xc05e_f8f5_5c7b_15a1),
    ("die_composed", 0x2fa1_18f5_b12d_0a6b, 0xddc7_92f7_6b2c_2877),
    (
        "die_composed_tour",
        0xc1ba_18c2_f8f5_4fe5,
        0x8ced_176d_f410_0067,
    ),
    ("plate_param", 0x86d2_0066_0d73_43dd, 0x0a41_acce_777f_31cc),
    ("kiss_carry", 0x2830_4771_94f7_e9b7, 0x9873_9a2f_819a_6c23),
    ("tube_ring", 0x09e2_09f2_3e9c_24d2, 0xc776_ac29_0ad6_0f24),
    ("tube_arc", 0xfc46_548d_4b70_1217, 0x61fb_ae1a_ac8b_abbd),
    (
        "hollow_tube_elbow",
        0xbac8_efd5_93ce_013d,
        0x77ea_aa36_b11b_cd9d,
    ),
    (
        "hollow_tube_ring",
        0x6667_e42e_de38_a3fe,
        0xf126_e0c0_2518_9262,
    ),
    // The first persisted `SetProgram` in the tree: its text digest
    // is the first taken over a log holding one.
    ("reshaped_rod", 0x1e8f_afbc_1909_57b0, 0x1e72_13bc_83bd_8db8),
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
