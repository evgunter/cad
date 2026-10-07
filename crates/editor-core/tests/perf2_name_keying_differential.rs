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
    ("die", 0x8ab08c05fe993da9, 0x4c43c1c03b089fd2),
    ("corner_table", 0x9d2f5f1603602c56, 0xc2d7dedb3c36a6e4),
    ("heat_sink", 0x8a53a2941c87adb9, 0xb785b404a1230810),
    ("crossing_slots", 0x1ae12b85eed61c34, 0xd85829bb66757610),
    ("nested_islands_105", 0xc48192aa4bf99150, 0xdb0ed0e76cc34161),
    (
        "nested_islands_106_depth1",
        0x8fd0a4c983008268,
        0xbbd68c27d9103067,
    ),
    (
        "nested_islands_106_depth2",
        0x65cde390f37d26e7,
        0x031f3be43c90d6d3,
    ),
    ("declared_tangency", 0xc90d58b75c3f875e, 0x34480000fcfb072d),
    ("kitchen_sink", 0xd6a09b98b12402ae, 0x62e6c285505c3cbc),
    ("cut_cylinder", 0x2daaea0d71777140, 0xd3a184191a4cc19b),
    ("measured_web", 0x28410fa5c9c4a70e, 0xcc65f02302d733d8),
    ("boss_union", 0xc75c5c25a64b6a1e, 0x7116081fbd2960a3),
    ("die_fillet", 0x80bccae801f57708, 0x6dcb21d6a6cfbfb8),
    ("die_chamfer", 0x8fc81dcb9e220258, 0x9479dab36a2dfdd4),
    ("die_pips", 0xf01f33f4c29646ae, 0x8faf89652902c1b3),
    ("heat_sink_fins", 0xd060670721a24e2d, 0x66e8a0d7e27e882a),
    ("die_tool", 0x269d714cc223116d, 0xca66e87fa655f537),
    ("face_sketch", 0xf41b1f6be4f0070c, 0xd87484a3ac1aa80c),
    ("part_select", 0x8c136e00b387d811, 0x3cd65780f1b6ad91),
    ("loft_prism", 0x257f85ed5c459334, 0x9e2948649b46cf41),
    ("die_composed", 0xadc9664877ea952e, 0xfc1955ee3b9135d7),
    ("die_composed_tour", 0x3f525bbd4a0d3a5c, 0x376b4b4328ef8916),
    ("plate_param", 0x153428ca03f2a385, 0x73c5ccb28f3fa1e9),
    ("kiss_carry", 0xe8cc0f244f229297, 0xde5cde280a9bedd4),
    ("tube_ring", 0x09e209f23e9c24d2, 0xc3223402510c7f54),
    ("tube_arc", 0xfc46548d4b701217, 0x994505ee78ced28d),
    ("hollow_tube_elbow", 0xbac8efd593ce013d, 0x33127c10980e382d),
    ("hollow_tube_ring", 0x6667e42ede38a3fe, 0xe60a6a6f441ebab6),
    ("reshaped_rod", 0xc6ae3f70c2e92fa4, 0xfb46b7183a42a1c0),
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
