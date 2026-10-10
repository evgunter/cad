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
    ("die", 0x83ac922e41d3ba16, 0x0e61bc09ee5e4787),
    ("corner_table", 0xe7742033d714b226, 0x20874cd38312daee),
    ("heat_sink", 0xd1913d4ed7b142ad, 0x02b4d72879733a82),
    ("crossing_slots", 0x1a1d87594aeeaacd, 0x141d42ffe8b5a6dc),
    ("nested_islands_105", 0xb7d1453e8c67b9d8, 0x1b0be3c32cabdfaa),
    (
        "nested_islands_106_depth1",
        0xa6b51042f738e0c3,
        0x7a3afa5549c2cdb6,
    ),
    (
        "nested_islands_106_depth2",
        0x3090a685e0366491,
        0xe51f281917b27d5d,
    ),
    ("declared_tangency", 0x920c4a6edb6d9790, 0xc4414d903d501fb2),
    ("kitchen_sink", 0xac37f84bb8a2cd36, 0x8193bba92d7d87d9),
    ("cut_cylinder", 0x3aa734635633b693, 0x4f8be79a1df60870),
    ("measured_web", 0x8836e05d1305221c, 0x0154c278d25f9609),
    ("boss_union", 0xa8dc1885fbd93a40, 0x5bbee4ef9702e5f4),
    ("die_fillet", 0xf3379766fd19c981, 0xe6711af35741f166),
    ("die_chamfer", 0x952f91a59f667284, 0x246a01bd1fd340d2),
    ("die_pips", 0x0d114d91f2c42469, 0x5aa026872f17bb55),
    ("heat_sink_fins", 0x14f392017fd38b54, 0x0eed3ffacf843bd2),
    ("die_tool", 0xb8b305ee55f32a45, 0xbb83b02a6f1b0b7b),
    ("face_sketch", 0xaac24880d7fbeabb, 0xa79dbdb5044164c2),
    ("part_select", 0x2f5ad217cb6d8521, 0x360e73f35d98e25d),
    ("loft_prism", 0x0563f998f4df486d, 0xb50dc09892c29686),
    // The blend's closing join moved these two tables (the joined host
    // trimlines and their foot); their persisted text did not move.
    ("die_composed", 0x424ddb0cf837da2c, 0xed1fc885ab668a89),
    ("die_composed_tour", 0xbb5c8c41cdde3138, 0xe38a70d44dd71c46),
    ("plate_param", 0xf2f372f6b8661b7f, 0x24f6fab30e0b9f68),
    ("kiss_carry", 0xf527265516b0b730, 0xe5e948dfbb9b93c2),
    ("tube_ring", 0x5b948c82c4cbc95b, 0xd97e281ea06504c6),
    ("tube_arc", 0xe069884a9e190ca8, 0x099c4bb7ea68d552),
    ("hollow_tube_elbow", 0xc6631b2e05b5e513, 0xd78b4922805fef4c),
    ("hollow_tube_ring", 0x9f3fb671109a6d9f, 0xa21aae32a20829d1),
    ("reshaped_rod", 0x13000c3a7fbdb581, 0x01ffc2e219331fec),
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
