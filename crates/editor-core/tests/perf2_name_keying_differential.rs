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
///
/// **Re-pinned for INTENT stage 2 PR C** (the product is the world):
/// every document now places its bodies, so its text holds the
/// placement nodes and their outputs (`roots` is gone from it) and its
/// tables hold each copy's names under its placement. What each
/// document delivers did not move: `intent_s2_c_world`'s migration
/// check holds each product to its pre-C digest.
///
/// **Re-pinned for INTENT stage 2 PR D** (a measure is one primitive):
/// `measured_web` alone. Its measure holds the distance and the web is
/// a definition the assertion reads, so the measure's mint preimage,
/// its id and its saved text moved; every other row held its word, and
/// the id-masked geometry fence held untouched.
///
/// **Re-pinned for INTENT stage 2 PR E** (a selection is a variable):
/// the seven documents that blend, shell, frame on a face or measure —
/// `measured_web`, `die_fillet`, `die_chamfer`, `face_sketch`,
/// `die_composed`, `die_composed_tour` and `reshaped_rod` — now mint a
/// selection variable each such node reads, so their node ids moved and
/// every row naming them; every other row held its word, and the
/// id-masked geometry fence held untouched.
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 0x62b07c7daae52838, 0x3c90ffcf06c6a15a),
    ("corner_table", 0x78adcd98aa349293, 0xf9103519a6680b71),
    ("heat_sink", 0x8beb484b542b2ede, 0x3d384c1be5e5b928),
    ("crossing_slots", 0x64de44e1f3deb528, 0xa470c7d82a485eb0),
    ("nested_islands_105", 0x186b786066f4f607, 0xfa3e0b12b86e1ac9),
    (
        "nested_islands_106_depth1",
        0x884db7159a8d3510,
        0x1b824a12cb1ce265,
    ),
    (
        "nested_islands_106_depth2",
        0x2a835d4731a44d4f,
        0x46a0794448aa5067,
    ),
    ("declared_tangency", 0xe7249c941ad4e6dd, 0x4ef7c39092eca21a),
    ("kitchen_sink", 0x4d54a9f813f26bc2, 0x5afd76e9573a1531),
    ("cut_cylinder", 0x366fa42a35257323, 0x0b1584874c6521a2),
    ("measured_web", 0x687e7d83e3b0d6f0, 0x9bd3a69736aa997b),
    ("boss_union", 0x563816ac9f7adc7e, 0x405ba395ed713022),
    ("die_fillet", 0xa3879e5204227d32, 0xcc827f4ab84ab2f7),
    ("die_chamfer", 0xbcfdc711a014ba7e, 0x955720216d75cf9a),
    ("die_pips", 0x650ac6623aa79fa7, 0xde8030f8ad79858a),
    ("heat_sink_fins", 0x248682fac9898b5a, 0xbdeed2f79c7c0af9),
    ("die_tool", 0xd27c69b347ec03b8, 0x59c24a20b944e29b),
    ("face_sketch", 0x25f0c5c4ab521837, 0x6ff3c25647b7831b),
    ("part_select", 0x2a314c6b9084180d, 0x29cf3119e30ba70a),
    ("loft_prism", 0x9f15f3e0cf1e8ace, 0xc7ef1ffc56e06ac4),
    ("die_composed", 0xf31d891c2ec31d07, 0xeca44d3772d9a79e),
    ("die_composed_tour", 0xe2039e28c41853bf, 0x3004f878021c8d9c),
    ("plate_param", 0x0ec32bf52511b5ea, 0x7fc1f8539afd08a0),
    ("kiss_carry", 0x471eeeaa3049eeab, 0xb459cc9e8e51d5d2),
    ("tube_ring", 0x33da3ceb7454021f, 0x9a8176ec901ca54c),
    ("tube_arc", 0xebcfe2751352a77e, 0xe780b5fe37d515b4),
    ("hollow_tube_elbow", 0xb1be521411cf3616, 0x27d7a90771dfe921),
    ("hollow_tube_ring", 0x46f61613d1f6a156, 0x39c2fbc857b67ef1),
    ("reshaped_rod", 0xf8c2a29b88e6a24a, 0xf096667c4f3cb66c),
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
