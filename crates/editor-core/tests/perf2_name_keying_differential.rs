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
    ("die", 0xf293_9f45_51a6_1281, 0x5622_3599_4665_e78b),
    ("corner_table", 0xec6c_5081_1ad3_dbc9, 0xbd34_cacd_ad52_06b1),
    ("heat_sink", 0x1acc_9e97_efef_374e, 0x02bf_b3a2_7673_24a7),
    (
        "crossing_slots",
        0x3ffd_7206_80d8_e1ed,
        0x3cf7_60e4_09b5_cce1,
    ),
    (
        "nested_islands_105",
        0x9089_435c_7b39_f84e,
        0xd7d8_72d3_0604_66d1,
    ),
    (
        "nested_islands_106_depth1",
        0x40e6_6d23_c1ce_78fa,
        0x27ef_7b5a_5f5f_85eb,
    ),
    (
        "nested_islands_106_depth2",
        0x33d3_0d4a_6f40_80d2,
        0x8acd_6604_c9cc_805e,
    ),
    (
        "declared_tangency",
        0x79c8_1fb5_5df5_cadb,
        0x18b6_9635_22a2_ed6d,
    ),
    ("kitchen_sink", 0x0091_d99d_4e53_b7b4, 0x4c53_3f34_73c8_9561),
    ("cut_cylinder", 0x4606_3982_c4a7_8013, 0x4c5d_391b_6548_58f3),
    ("measured_web", 0x0292_25ef_fc59_32f8, 0x6a46_1194_9a85_aa8b),
    ("boss_union", 0xd7c0_1ebb_bd1a_37f2, 0x6e39_ba62_7549_a076),
    ("die_fillet", 0x5768_aa33_1646_f3f6, 0xfea4_4152_1375_ae51),
    ("die_chamfer", 0x0f92_58a8_2d15_cdf6, 0xeb85_0ef2_45d4_ced0),
    ("die_pips", 0x1cc8_a6b6_c7cb_0428, 0x8a11_608e_d9f1_7afe),
    (
        "heat_sink_fins",
        0x301d_1832_d686_695b,
        0xc2ea_ac9d_e0e4_2665,
    ),
    ("die_tool", 0x3083_4f4a_8dbb_31e7, 0x9cbc_b6a9_2205_e35c),
    ("face_sketch", 0xe0e2_897c_def1_b44f, 0x62de_3809_4f19_178e),
    ("part_select", 0x1329_ff16_58c1_e514, 0x1732_a842_6424_e947),
    ("loft_prism", 0x257f_85ed_5c45_9334, 0x0158_faf3_57ab_36cd),
    ("die_composed", 0xd698_2b75_9e6d_35f9, 0x0018_e552_0174_d390),
    (
        "die_composed_tour",
        0x19a0_2d9c_ca86_fbbd,
        0x8c76_4e40_8d74_6427,
    ),
    ("plate_param", 0xd500_d4d9_18a0_9e2b, 0xa490_b6c5_2d2b_ce14),
    ("kiss_carry", 0xdd6a_08e0_5c50_a070, 0x4d96_ab37_c9e8_1562),
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
    ("reshaped_rod", 0x1e8f_afbc_1909_57b0, 0x54cd_90a4_1cac_9b94),
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
