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
    ("die", 0xc4c0_1c98_c637_b0af, 0xb868_310d_6d83_d180),
    ("corner_table", 0x0fba_accb_8d1d_2446, 0xc2d7_dedb_3c36_a6e4),
    ("heat_sink", 0xbbb8_b524_ae03_22e1, 0x538c_d43d_b678_38f0),
    (
        "crossing_slots",
        0xa778_9570_3adf_8a9a,
        0xd858_29bb_6675_7610,
    ),
    (
        "nested_islands_105",
        0xbe40_0369_f69e_fce4,
        0xdb0e_d0e7_6cc3_4161,
    ),
    (
        "nested_islands_106_depth1",
        0x5585_9b4d_f393_5fe8,
        0xbbd6_8c27_d910_3067,
    ),
    (
        "nested_islands_106_depth2",
        0xd314_04ee_630f_85bd,
        0x031f_3be4_3c90_d6d3,
    ),
    (
        "declared_tangency",
        0xc90d_58b7_5c3f_875e,
        0x3448_0000_fcfb_072d,
    ),
    ("kitchen_sink", 0xaf57_998d_6b35_9d4e, 0xe833_ec08_59b1_4a59),
    ("cut_cylinder", 0x5f67_3a87_e7ea_6984, 0xd3a1_8419_1a4c_c19b),
    ("measured_web", 0x0d7d_d459_34a2_9744, 0x31bb_9cec_201b_92e6),
    ("boss_union", 0x9869_df73_e15f_370c, 0x7116_081f_bd29_60a3),
    ("die_fillet", 0x80bc_cae8_01f5_7708, 0x6dcb_21d6_a6cf_bfb8),
    ("die_chamfer", 0x8fc8_1dcb_9e22_0258, 0x9479_dab3_6a2d_fdd4),
    ("die_pips", 0xe00b_6b9d_9055_9f08, 0x8faf_8965_2902_c1b3),
    (
        "heat_sink_fins",
        0x3856_85f6_1e66_7868,
        0xc1a3_ff0c_42c8_dd86,
    ),
    ("die_tool", 0xd5ba_730b_96ef_7e5b, 0xca66_e87f_a655_f537),
    ("face_sketch", 0xf41b_1f6b_e4f0_070c, 0xd874_84a3_ac1a_a80c),
    ("part_select", 0xa8d8_a304_a597_36b7, 0x0db3_8837_a5af_b52d),
    ("loft_prism", 0x257f_85ed_5c45_9334, 0x9e29_4864_9b46_cf41),
    ("die_composed", 0x3676_2319_e306_42fe, 0xfc19_55ee_3b91_35d7),
    (
        "die_composed_tour",
        0x2100_04d1_be49_55c0,
        0x376b_4b43_28ef_8916,
    ),
    ("plate_param", 0x047c_8706_0db8_aa18, 0x5239_724b_32b8_b18a),
    ("kiss_carry", 0xebc2_712d_6230_38c7, 0xde5c_de28_0a9b_edd4),
    ("tube_ring", 0x09e2_09f2_3e9c_24d2, 0xc322_3402_510c_7f54),
    ("tube_arc", 0xfc46_548d_4b70_1217, 0x9945_05ee_78ce_d28d),
    (
        "hollow_tube_elbow",
        0xbac8_efd5_93ce_013d,
        0x3312_7c10_980e_382d,
    ),
    (
        "hollow_tube_ring",
        0x6667_e42e_de38_a3fe,
        0xe60a_6a6f_441e_bab6,
    ),
    // The first persisted `SetProgram` in the tree: its text digest
    // is the first taken over a log holding one.
    ("reshaped_rod", 0xc6ae_3f70_c2e9_2fa4, 0xfb46_b718_3a42_a1c0),
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
