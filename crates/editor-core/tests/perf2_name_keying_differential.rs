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
    ("die", 0xb374_4362_5948_39b7, 0x761a_f473_27a2_3b44),
    ("corner_table", 0x7eb2_f246_c561_7112, 0x0a37_a848_31df_bf60),
    ("heat_sink", 0x2878_2ee0_37ac_5811, 0x3375_3cf0_fdb6_5919),
    ("crossing_slots", 0xe5c7_fa72_79b3_7f62, 0xbf26_b5e4_84a9_ad0a),
    ("nested_islands_105", 0xe39a_4614_ac9d_1714, 0x1c75_f4e0_cce6_e417),
    ("nested_islands_106_depth1", 0x9308_8c0a_9258_4a75, 0xf822_ee43_3d39_ee1a),
    ("nested_islands_106_depth2", 0xd9c8_4a3a_bb4f_6102, 0xeea6_6bf6_d480_7d43),
    ("declared_tangency", 0x30be_3e49_eeb2_29c2, 0xe7ed_ae6e_23c3_1324),
    ("kitchen_sink", 0x0369_ac51_745c_05bf, 0x1e96_2ad9_9ae0_695b),
    ("cut_cylinder", 0x85fe_0bc1_dca6_e66f, 0x1acb_c031_25df_2455),
    ("measured_web", 0x1d3e_d965_4f21_be77, 0x2dcf_01be_e7f3_2f27),
    ("boss_union", 0x5b9a_c367_89e7_cb9d, 0xb687_201a_05b6_6f05),
    ("die_fillet", 0x57a2_a97c_910f_55c9, 0x5e5d_220c_5da9_b301),
    ("die_chamfer", 0x57a2_a97c_910f_55c9, 0xb433_5ab7_38ef_ddf2),
    ("die_pips", 0xf50a_0866_b9f3_6327, 0xb3b5_8824_7df9_e4b3),
    ("heat_sink_fins", 0x3d11_1f6c_5b0b_2f5d, 0x805b_442f_7fea_41cc),
    ("die_tool", 0x5f4d_0f55_51bc_c9c2, 0xad5d_d38c_d57b_202d),
    ("face_sketch", 0x3daa_e163_9d28_6ce0, 0x2fd6_61aa_8fe9_a4ef),
    ("part_select", 0x4c2a_35db_6554_94de, 0xb6d8_fba1_2403_14c9),
    ("loft_prism", 0x1732_9dcf_7d73_6ff8, 0x154a_59f7_74bc_9fdd),
    ("die_composed", 0xba2f_4cf0_589e_12af, 0xf6bf_0e99_9ea5_ff48),
    ("die_composed_tour", 0xad3e_4226_8cc8_1adf, 0x166a_646f_2bce_1e04),
    ("plate_param", 0xc9c5_9dfd_d322_3b42, 0xde95_ae41_3e46_e947),
    ("kiss_carry", 0xd1f5_3821_c576_c77a, 0x7470_5d73_a4cf_11b0),
    ("tube_ring", 0xafab_a990_cf48_8327, 0xfa5c_b8c9_1200_9d34),
    ("tube_arc", 0x916d_ce74_bf3e_314d, 0xdb49_2b2d_e26e_db41),
    ("hollow_tube_elbow", 0x228e_a8ef_876d_7cf7, 0xa603_e468_f99b_e976),
    ("hollow_tube_ring", 0x2124_0759_7a30_b1cf, 0x9042_40e2_bb0f_c1b0),
    // The first persisted `SetProgram` in the tree: its text digest
    // is the first taken over a log holding one.
    ("reshaped_rod", 0xf59c_be60_a9b4_eb8d, 0x8681_dccd_c405_47ed),
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
