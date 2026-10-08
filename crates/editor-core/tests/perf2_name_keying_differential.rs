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
    ("die", 0x338de96d109c68d6, 0x02a849a19f11c47b),
    ("corner_table", 0xdc180b4855b385eb, 0x2786e90e8f1f03ae),
    ("heat_sink", 0xa31d67162503e0a0, 0x3458acc9a5cdb9c1),
    ("crossing_slots", 0x786ec2aa6cd6f31d, 0x3e27e42020659db9),
    ("nested_islands_105", 0x699a7e9861d9fbcf, 0xcd629b234cb37c71),
    (
        "nested_islands_106_depth1",
        0xd1ac845896039780,
        0xddd3eae4791f911e,
    ),
    (
        "nested_islands_106_depth2",
        0xbea03a2a61deac13,
        0xdfe4af2d569d2c85,
    ),
    ("declared_tangency", 0xfc7b28712fed8673, 0x851c55000c08046e),
    ("kitchen_sink", 0xeaf798783ef4d866, 0x95869081551cc1eb),
    ("cut_cylinder", 0xb3890612dae55e29, 0xef1ee738c04b2ca4),
    ("measured_web", 0x569697021a9aa942, 0xfb462a739968f7f8),
    ("boss_union", 0x927fa27372fd6c91, 0x252770f97676df66),
    ("die_fillet", 0xc678f5aece31a09d, 0x651d829811fbc24c),
    ("die_chamfer", 0x0212d10547cdc260, 0x7b8b0c39477dad2f),
    ("die_pips", 0x90ad6c75b96ccc8b, 0xa4f5c32c23fe9c1f),
    ("heat_sink_fins", 0xb1f121110375a178, 0x823d66da52c5dcbe),
    ("die_tool", 0xe2cd118b8808988b, 0x345a1d0238beff78),
    ("face_sketch", 0x2dea0cee0f356eaa, 0x9cb5bd6899c51794),
    ("part_select", 0x5a61df6418fbbfa9, 0x6c964587fb3e6bbb),
    ("loft_prism", 0x8efcda4a34e94e1b, 0xafa2e0efb199df49),
    ("die_composed", 0x7709c77e13654462, 0x1a119d835d2cb0da),
    ("die_composed_tour", 0x2c7bad55075e8e50, 0xef95a0bcd48c700c),
    ("plate_param", 0xde244f7b26adac55, 0x394fa229c9ccef32),
    ("kiss_carry", 0x9cd94d1fdca40b21, 0xd0fa6e10330d412c),
    ("tube_ring", 0xfc48d7674b9d4ead, 0x31fcd0605dd5bdb8),
    ("tube_arc", 0x86720d782916e281, 0x6b7c0130d7731855),
    ("hollow_tube_elbow", 0xec550b32dcda1bf1, 0x9fceb270956ff222),
    ("hollow_tube_ring", 0x6ab51b93ed8fd4ae, 0x0b804cfd3cc86f80),
    ("reshaped_rod", 0x2be01e7be7a81fae, 0x30fd536cacf702a4),
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
