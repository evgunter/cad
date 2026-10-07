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
///
/// **Re-pinned for INTENT-LITERALS PR C** (a slot holds a variable):
/// every node is minted from slots that hold variable ids, a typed
/// value's variable drawn from what it holds, so every node id moved
/// and with it every row this hashes. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held untouched across the change.
///
/// **Re-pinned for INTENT-LITERALS PR D** (`Expr` holds no float):
/// `kitchen_sink` alone, whose formulas hold written quantities that
/// now mint variables of their own; every other row held its word.
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 0x9337a274a3acb726, 0x46506aec78b12bc0),
    ("corner_table", 0x91e1ede9fb20d08b, 0xd814d9aa6a09ba37),
    ("heat_sink", 0xbc971878ad01ad4f, 0x0d2f9ea83e6390ea),
    ("crossing_slots", 0x506f21b41842b002, 0x41f43ec6e4ae4de0),
    ("nested_islands_105", 0x5e01753841ed22be, 0x81ef8c83e1fd74a1),
    (
        "nested_islands_106_depth1",
        0xb18da35d1da4cdd3,
        0xb010532485d9bc04,
    ),
    (
        "nested_islands_106_depth2",
        0x076cc36bdc6d9fc6,
        0xd035d9e7fe10f038,
    ),
    ("declared_tangency", 0x1d76ad1c387f0f76, 0x9fd2c3709cf881a5),
    ("kitchen_sink", 0xcd73538f89ef4d39, 0xd784b5102147178d),
    ("cut_cylinder", 0xa9b02a216a10c201, 0x5134a93fc0fc00d8),
    ("measured_web", 0x6fe2b289032f2c02, 0xaaf9fe407ce9511d),
    ("boss_union", 0xe73757c4d5815f2b, 0xcba022cd02d011f3),
    ("die_fillet", 0x97d6ecd741693786, 0x8b78a712acf38b22),
    ("die_chamfer", 0xf00f3fd78bb2ead3, 0x2285448f7ae839e6),
    ("die_pips", 0xb5417c3954c61ef7, 0xdf770974aa1204b8),
    ("heat_sink_fins", 0xd7757192afe26a4f, 0x5f0076d10dd022a2),
    ("die_tool", 0xab6616a11d2c7691, 0xfbffa4b785ea0de1),
    ("face_sketch", 0x8ccaad08895bb12c, 0xff1700ab87b58a6a),
    ("part_select", 0xc8a2a1894e525e62, 0x5fea8c89f9831517),
    ("loft_prism", 0x0f50d336364203e7, 0xb8f996dbcece94ea),
    ("die_composed", 0xd71d9de7ef02de8d, 0x55330a4e993e67ab),
    ("die_composed_tour", 0xe9b8bf99fa05fcc2, 0xf529e5cfd2086ec3),
    ("plate_param", 0x2ec407c2cbc0d852, 0x16fe73eeaffd2d85),
    ("kiss_carry", 0x15695353d45cb97b, 0x20dab83b21e4350b),
    ("tube_ring", 0x5623ee546b6a995e, 0x6a494d3a6e3b179d),
    ("tube_arc", 0x1ff5dd9cf6ef9b39, 0xaf67d49d9095538a),
    ("hollow_tube_elbow", 0x51d74e8c862056d6, 0x5dc45f5fef52f0bc),
    ("hollow_tube_ring", 0xa203ff036995c601, 0xc09849d6a42c7829),
    ("reshaped_rod", 0x42f0ed7b404add2f, 0x537872614d22b71d),
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
