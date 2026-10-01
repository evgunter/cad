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
    ("die", 0xb894_8d14_7ea8_f683, 0xf028_c2e9_7c68_c9cb),
    ("corner_table", 0x5049_8282_2abc_6576, 0xe1af_e304_cb74_b58d),
    ("heat_sink", 0x3f75_2e44_c653_a195, 0xc121_bd20_aa40_3afb),
    (
        "crossing_slots",
        0xda43_6c13_abf1_d594,
        0x8699_30d6_d9f4_5832,
    ),
    (
        "nested_islands_105",
        0x744f_c58b_23ea_7a32,
        0xa54b_1d87_1ad6_144e,
    ),
    (
        "nested_islands_106_depth1",
        0x0b43_088d_fcfc_dc99,
        0x93de_1cd7_5952_da4f,
    ),
    (
        "nested_islands_106_depth2",
        0x3adb_b7f9_21b4_8ace,
        0xa58c_369b_0600_ab56,
    ),
    (
        "declared_tangency",
        0x8d76_33c1_671d_4a60,
        0x70a2_f947_1ec9_50dc,
    ),
    ("kitchen_sink", 0x90ac_7090_7b34_a265, 0xf721_98a2_1cd8_727c),
    ("cut_cylinder", 0x834e_c919_bea5_e627, 0x031f_091f_86a3_bbf2),
    ("measured_web", 0x08a9_cca4_099a_f6eb, 0x0f41_ad3c_a6aa_7857),
    ("boss_union", 0x3ae1_d9ac_7a23_a219, 0x9ec6_3539_b5fa_b0ad),
    ("die_fillet", 0x0870_50ac_c9dd_c471, 0x342c_e205_8a5c_4b86),
    ("die_chamfer", 0x0870_50ac_c9dd_c471, 0x044e_6dd6_a945_e3cb),
    ("die_pips", 0xf92a_497e_183a_d145, 0x9329_0046_2b0f_d692),
    (
        "heat_sink_fins",
        0x9ab9_8e4e_263d_f73d,
        0xba6c_7b5b_0305_528b,
    ),
    ("die_tool", 0xdaa2_9bd7_da39_9be8, 0x870f_c3b5_cad7_3495),
    ("face_sketch", 0xaebd_8352_c897_9f38, 0x0e14_fd88_e2a3_89f0),
    ("part_select", 0x1bda_a086_1125_bace, 0xa3d4_56af_fd7a_fca2),
    ("loft_prism", 0x2c5c_54f7_0e9b_3170, 0x7114_73e9_367c_ca4a),
    ("die_composed", 0xdc85_e49e_ad92_0861, 0xb9bc_3e41_cb1d_1f0d),
    (
        "die_composed_tour",
        0xf33d_572d_3f14_cc11,
        0x53eb_2155_9ec9_d281,
    ),
    ("plate_param", 0x6531_364f_7c5b_9574, 0x299b_671a_375b_e29f),
    ("kiss_carry", 0x31d0_3adb_a450_3576, 0x97c1_41bc_2508_3e86),
    ("tube_ring", 0xafab_a990_cf48_8327, 0xb556_1722_b63b_c370),
    ("tube_arc", 0x916d_ce74_bf3e_314d, 0x55a7_e98c_6d79_442d),
    (
        "hollow_tube_elbow",
        0x228e_a8ef_876d_7cf7,
        0x98d1_e331_2bc3_d932,
    ),
    (
        "hollow_tube_ring",
        0x2124_0759_7a30_b1cf,
        0x7209_c7e6_d54e_e134,
    ),
    // The first persisted `SetProgram` in the tree: its text digest
    // is the first taken over a log holding one.
    ("reshaped_rod", 0xdfd7_8c54_df06_ab85, 0xc406_afe8_6182_63a7),
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
