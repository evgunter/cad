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
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 0x7c0063c6f610214a, 0xb9fd427959fee070),
    ("corner_table", 0xeed262c8399c88d5, 0xa0430bdc34cc13fb),
    ("heat_sink", 0x0bbbfca1fb4af266, 0xdfc93183609f02d8),
    ("crossing_slots", 0x2ed2e98ee9fd3948, 0x77a1ef0adf8fcb04),
    ("nested_islands_105", 0x31b576cdbd937dc7, 0xed62b8e9c476ae64),
    (
        "nested_islands_106_depth1",
        0xcebb92c15da4da36,
        0x15b75bbc9ae7cbd6,
    ),
    (
        "nested_islands_106_depth2",
        0xc221e76a6daca4a5,
        0xa95f171e06017d0b,
    ),
    ("declared_tangency", 0xe7249c941ad4e6dd, 0x4ef7c39092eca21a),
    ("kitchen_sink", 0x37558c266b941c5a, 0xb7b95e5ed05b1b21),
    ("cut_cylinder", 0x366fa42a35257323, 0x0b1584874c6521a2),
    ("measured_web", 0xa33c4004c064975d, 0x1d888af148f0f31e),
    ("boss_union", 0x563816ac9f7adc7e, 0x405ba395ed713022),
    ("die_fillet", 0x2b071b9218e6bd30, 0x15261a579edae7b5),
    ("die_chamfer", 0xacc58f812a61da8c, 0x6c4a019ce8bee6b2),
    ("die_pips", 0x650ac6623aa79fa7, 0xde8030f8ad79858a),
    ("heat_sink_fins", 0x248682fac9898b5a, 0xbdeed2f79c7c0af9),
    ("die_tool", 0x9009ee9ed80f29e7, 0xa3b3702e2dcb61fa),
    ("face_sketch", 0x380d595523d5b0af, 0xacdc1a2b07152fd4),
    ("part_select", 0x5b14de8065f9b1b6, 0x186ec8c1575ee173),
    ("loft_prism", 0x9f15f3e0cf1e8ace, 0xc7ef1ffc56e06ac4),
    ("die_composed", 0x36aba954b728e3e1, 0xbff3453d1a482337),
    ("die_composed_tour", 0xfb8adcf5f6595625, 0x2866e76549815150),
    ("plate_param", 0x4b3a8cc0088d16bd, 0x200677f2f703b7b2),
    ("kiss_carry", 0xd5c8c49fd0f8dffa, 0x206b1da6857dede9),
    ("tube_ring", 0x33da3ceb7454021f, 0x9a8176ec901ca54c),
    ("tube_arc", 0xebcfe2751352a77e, 0xe780b5fe37d515b4),
    ("hollow_tube_elbow", 0xb1be521411cf3616, 0x27d7a90771dfe921),
    ("hollow_tube_ring", 0x46f61613d1f6a156, 0x39c2fbc857b67ef1),
    ("reshaped_rod", 0x0350b6ce38a8ae3f, 0x1bf75f321a768b78),
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
