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
    ("die", 0x619c_65cf_1b9f_6647, 0x4de3_566d_10fa_5261),
    ("corner_table", 0x86cb_184c_3481_9f06, 0x9d0e_4973_f3c6_108d),
    ("heat_sink", 0x5080_f5fb_2ada_0b49, 0xf453_ee0e_8341_bbdf),
    (
        "crossing_slots",
        0x6176_811f_4981_a546,
        0x155b_232b_5fe0_0fe9,
    ),
    (
        "nested_islands_105",
        0x39df_e100_14fd_a828,
        0x6827_74c8_5b27_3943,
    ),
    (
        "nested_islands_106_depth1",
        0xf34b_8b59_778e_d4a7,
        0xe794_478c_08fd_16a6,
    ),
    (
        "nested_islands_106_depth2",
        0x8618_7586_0d06_5792,
        0x763f_02d8_4e26_1c42,
    ),
    (
        "declared_tangency",
        0x43f8_d22b_4a06_bef8,
        0xb740_afe6_3ac0_3b9e,
    ),
    ("kitchen_sink", 0xee5d_6ad7_b931_ff55, 0x2673_3afa_01c0_7128),
    ("cut_cylinder", 0xe0d5_2931_ec58_c9b1, 0xa4f1_a604_21f6_2494),
    ("measured_web", 0x8f05_d4ee_185c_a667, 0x9620_7fc9_b6fc_5050),
    ("boss_union", 0xd970_c774_1a5d_58dd, 0xf0b7_3067_97fb_9e6a),
    ("die_fillet", 0x147d_a6c2_0018_91ff, 0xf9cb_f3b8_9f20_15be),
    ("die_chamfer", 0x147d_a6c2_0018_91ff, 0x6dea_76ae_1356_c167),
    ("die_pips", 0x3d27_f3a0_918b_8c3d, 0x95ca_004d_9b43_7b93),
    (
        "heat_sink_fins",
        0xebd6_be2c_e43f_4e35,
        0xbf51_2e73_116f_b9ed,
    ),
    ("die_tool", 0xc99f_2e08_8b24_8498, 0xeef7_257f_438c_2f0d),
    ("face_sketch", 0x200b_b0eb_0e7e_e54e, 0xbd64_499b_ba26_f923),
    ("part_select", 0x114a_9d82_a993_9e00, 0x5c7c_d17b_9981_28e3),
    ("loft_prism", 0xc6db_7be2_9eb6_dfc8, 0xe546_02da_c16f_fa75),
    ("die_composed", 0x81ca_737b_5ccc_97c7, 0xd7b4_ab3e_23ae_56ec),
    (
        "die_composed_tour",
        0xb5b2_4244_d25d_310b,
        0xeb72_eb6d_b988_d8d8,
    ),
    ("plate_param", 0xc650_c981_5207_32b4, 0xcc04_5e8e_2d92_2a98),
    ("kiss_carry", 0x71d9_43b3_9ed1_9338, 0xf3ee_97eb_9f50_c8bb),
    ("tube_ring", 0x4fe2_260e_a6e8_0fd3, 0x3285_e0b9_de61_a44c),
    ("tube_arc", 0xd83d_2ab3_aebe_7035, 0x0af4_9808_bde0_8619),
    (
        "hollow_tube_elbow",
        0x660f_7e52_b862_5c71,
        0xf64d_52cf_0cd9_81ae,
    ),
    (
        "hollow_tube_ring",
        0x8b51_65bd_2a3d_89c3,
        0xc2b6_c5f9_eb37_f948,
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
