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
    ("die", 0x72f57b94cc9fe884, 0xaae0cb48a6016f0f),
    ("corner_table", 0x414c750a0f2fdc07, 0xd6f34726eeb6a399),
    ("heat_sink", 0x0798e64f202bc208, 0xe4387b8a0d4caf60),
    ("crossing_slots", 0x05af54914b87d25a, 0x268f18ef91ff5589),
    ("nested_islands_105", 0x69de11ecae1431c7, 0x2cf160d4b83e036b),
    (
        "nested_islands_106_depth1",
        0x1d41531ace7d24f9,
        0x3a807d13dbff0dc7,
    ),
    (
        "nested_islands_106_depth2",
        0xf1ce1ddf68891699,
        0xa16b183d8ea9c472,
    ),
    ("declared_tangency", 0x20dcf45fc699b192, 0x8eb4f6d857b96daf),
    ("kitchen_sink", 0xeac5a0519d4e3623, 0x5cfb9e8e5a134138),
    ("cut_cylinder", 0x5c3f85efc061d526, 0x8bac2bef268572bb),
    ("measured_web", 0x6c33c2e52d4ef7a5, 0x7b3589f58b2daaa2),
    ("boss_union", 0x21dd428a780ae942, 0xfa6dd5f60bdab416),
    ("die_fillet", 0x2ea00cf1a1d2af81, 0x90b7b79403a12799),
    ("die_chamfer", 0x433e53c9fa2da26a, 0x0b27f3cfc386cf10),
    ("die_pips", 0x8f19df85580dc63c, 0x7ad001d047bcee7c),
    ("heat_sink_fins", 0x457fa22c8a76c4e6, 0xf6278745e4242ebf),
    ("die_tool", 0x7b989537433a1944, 0x22aede7d6b3e452e),
    ("face_sketch", 0xc14cd1168e01140e, 0x545cf694e5579754),
    ("part_select", 0x5c9f4d3590f76b77, 0x8c46387ff32f6549),
    ("loft_prism", 0xb20be28b52e23953, 0xa895e42c52c56fab),
    ("die_composed", 0x7708886770126871, 0x3c0c5ff9ce8373c0),
    ("die_composed_tour", 0x1d2f73e49cc5370d, 0x5bcab07b467861dc),
    ("plate_param", 0x2e85fa9543827703, 0x668da60cf85cae93),
    ("kiss_carry", 0xefd6a88f4e31f0a0, 0xc2e3b763a435f190),
    ("tube_ring", 0x5b948c82c4cbc95b, 0xd97e281ea06504c6),
    ("tube_arc", 0xe069884a9e190ca8, 0x099c4bb7ea68d552),
    ("hollow_tube_elbow", 0xc6631b2e05b5e513, 0xd78b4922805fef4c),
    ("hollow_tube_ring", 0x9f3fb671109a6d9f, 0xa21aae32a20829d1),
    ("reshaped_rod", 0x2859e67678a96993, 0xaa0a69be3ae30aae),
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
