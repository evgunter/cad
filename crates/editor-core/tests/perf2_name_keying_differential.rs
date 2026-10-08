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
    ("die", 0x84f08a6c44f7f37c, 0xa7bff3f84b46e872),
    ("corner_table", 0x3656ce5197180597, 0x5dff4459175cf0c7),
    ("heat_sink", 0x4eff95834e05797d, 0x78329872187eb8c9),
    ("crossing_slots", 0x00652ced9a4813ea, 0xd70d769ecc083692),
    ("nested_islands_105", 0xcdb0d34cc9de8a5a, 0xfe7349a091221711),
    (
        "nested_islands_106_depth1",
        0x9cb3f7eba80e3bff,
        0xb220aea8058cbc64,
    ),
    (
        "nested_islands_106_depth2",
        0x7c8c406a80736f0b,
        0x77a8c81af1a14efd,
    ),
    ("declared_tangency", 0x1d7bca5d509d8ab8, 0xd9b872b5a8e507ac),
    ("kitchen_sink", 0xb394f9860ea97030, 0x8fb00396e6d007db),
    ("cut_cylinder", 0x93806d6d14b22ab3, 0x1f69aa9d1ff439de),
    ("measured_web", 0x7b04205ea105cc7a, 0xec021418f836d995),
    ("boss_union", 0xccd5072e369b35f8, 0x58a78ee8622b36a0),
    ("die_fillet", 0xe70763cd2206838d, 0xdf1ac7f83aef13fd),
    ("die_chamfer", 0x41ea228623983d5a, 0xca07acef459175e7),
    ("die_pips", 0x50e4d9e7de620319, 0x0fd37aee98ccddc9),
    ("heat_sink_fins", 0x37d72eb789fdcae4, 0x9a48e9b9cca7d72e),
    ("die_tool", 0x229829a20453cc63, 0xec0c1a3d4f281945),
    ("face_sketch", 0xf005920477b4c45e, 0x90117e933a0c34bf),
    ("part_select", 0x9fb18f5c9ee777d6, 0x9ba4cfe4e03a7518),
    ("loft_prism", 0x7b0ca0c00123a939, 0xfde137cf1cc93547),
    ("die_composed", 0xaa5ae30b551d4077, 0xcc94fd919193b59c),
    ("die_composed_tour", 0x7a41702956b74775, 0x6b565d15813ebd48),
    ("plate_param", 0x1493b9efc6413f43, 0x039294a52b3467c2),
    ("kiss_carry", 0xb8255010a6fcce3f, 0x4028c12c64ae3f0d),
    ("tube_ring", 0xbda0bb3a62bbb1ec, 0x4231e1ee994d3a8d),
    ("tube_arc", 0xee8a27665a0e0d70, 0x33c3d91c6761cdcd),
    ("hollow_tube_elbow", 0x33be393a25268459, 0x4743af5134649243),
    ("hollow_tube_ring", 0xb7a64e25993c0122, 0x07c7cb5fed971359),
    ("reshaped_rod", 0xa8e1434396635f4f, 0x4cbc27516c028948),
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
