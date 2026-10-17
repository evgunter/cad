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
        for t in crate::fixture::tables(ev, *id) {
            perf2_table_digest(&mut h, t);
        }
    }
    h.0
}

/// One table's rows into `h`, forward then reverse.
fn perf2_table_digest(h: &mut Fnv, t: &editor_core::NameTable) {
    for (n, e) in t.iter() {
        h.feed(&format!("fwd {n:?}={e:?};"));
    }
    for (_, e) in t.iter() {
        for ent in match e {
            editor_core::names::Entry::Unique(one) => vec![*one],
            editor_core::names::Entry::Tied(many) => many.clone(),
        } {
            h.feed(&format!(
                "rev {ent:?}={:?};",
                t.name_of(&ent).map(|n| format!("{n:?}"))
            ));
        }
    }
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
///
/// **Re-pinned for INTENT stage 3 PR A** (a revolve's axis is a line on
/// the node): the five documents that revolve — `kitchen_sink`,
/// `die_pips`, `die_tool`, `die_composed` and `die_composed_tour` —
/// hold no in-plane axis datum,
/// so their later node ids moved and every row naming them; every other
/// row held its word, and no point moved. Re-taken merged over
/// FORK-DM4 unit 1 (PR 4527), whose names keyed by read had moved the
/// same rows on main.
///
/// **Re-pinned for the killed-lineage chase** (`emit_topo`'s
/// `KilledSplits`): `crossing_slots` only. Its second slot's bottom rim
/// reaches the result in pieces split off around stretches the subtract
/// killed. Those pieces were read as chords, and one was named a seam of
/// the first slot's floor and the second slot's wall. They are now
/// pieces of that rim. Two floor corners moved with them: one from a
/// junction of three seam lines to a crossing of the rim, and one from
/// a face crossing to the seam vertex of two edges that touch. Every
/// other document held its word.
///
/// **Re-pinned for the vertex run** (N2: an `EdgeCrossing` is a run of
/// (line, sense) in name order): `corner_table` and `crossing_slots`,
/// the two documents whose pair booleans meet edge to edge. Their
/// tables are the old ones with each edge × edge vertex re-spelled: read
/// as a set of (line, sense), every run is the pair it replaced, and no
/// other row moved; the persisted text did not move.
/// **Re-pinned for the output stage's killed lineage** (`topo`'s
/// output stage copies the records of the edges its merge kills into
/// `BooleanNaming::killed_lineage`, which `KilledSplits` reads):
/// `crossing_slots` only. The second subtract's merge of the two flush
/// floors kills one stretch of the first cavity's bottom rim and keeps
/// the next, whose lineage stopped at the stretch killed, so it was
/// named a seam of the cavity's wall and the slot's floor. It is now a
/// piece of that rim, and the floor corner where the slot's rim meets
/// the cavity's wall moved from a touch of the wall to a crossing of the
/// two rims, as the other three such corners are named. Every
/// other document held its word, and the persisted text did not move.
///
/// **Re-pinned for INTENT stage 3 B** (a placement is the bundle of
/// mates), every row, the persisted text with it (it spells `Place` and
/// its constraint): a placement is a `Place` holding a world pin where
/// it was a `PlaceInWorld`, so its stored node, and with it its mint
/// preimage, moved, and every id the mint chain draws after it. No
/// outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// **Re-pinned for FORK-DM4 unit 2** (a name is scoped by the variable
/// holding its body): `cut_cylinder`, `kitchen_sink` and `part_select`,
/// the three documents holding a split. A split's two halves are two
/// tables and its names spell no half, so each row walks both tables of
/// the half-free names; `part_select`'s union, which declares the rest
/// between the halves by the section face, also took a new id. Every
/// other row held its word, and the id-masked geometry fence held.
/// The persisted text moved in `kitchen_sink` (its appearance key is
/// now a body read and a name) and `part_select` (its union's declared
/// names); `cut_cylinder`'s held.
///
/// Re-taken merged over INTENT stage 3 PR A (a revolve's axis is a
/// line on the node), which moved `kitchen_sink` too: its merged row is
/// neither tree's value; every other row is the one of the two trees
/// that moved it.
///
/// **Re-taken merged over INTENT stage 4 F** (declared pairs retire):
/// main's re-bless and this unit's each moved these rows, so the merged
/// rows are neither tree's value; the id-masked geometry fence held.
///
/// **Re-taken merged over INTENT stage 3 B** (#5232, a placement is the
/// bundle of mates), which re-pinned every row by its new mint chain:
/// the merged rows are main's values but for the three split documents,
/// `cut_cylinder`, `kitchen_sink` and `part_select`, which this unit
/// moves, and whose merged values are neither tree's.
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 0x9f0d34d7a07b5474, 0x9f03cc351319921e),
    ("corner_table", 0x366cfaeb1b511dd4, 0x1df2baaa0b4cfe02),
    ("heat_sink", 0xaf8df354a89b9b7e, 0x30b1df72ae4fae7d),
    ("crossing_slots", 0x892956236a7342ad, 0x0e714c597bb9a2ab),
    ("nested_islands_105", 0x040165b6681f1e32, 0x031a21c37ea524aa),
    (
        "nested_islands_106_depth1",
        0xc61b4696df6c3fa7,
        0xffc1f6d8c480cab6,
    ),
    (
        "nested_islands_106_depth2",
        0xa2cd273b9315a2c9,
        0xa529e5f0b9bc012b,
    ),
    ("declared_tangency", 0xb33609b3c74c8491, 0x653d3e8e9ab0edf6),
    ("kitchen_sink", 0x921af29a8a392e9f, 0x1a3281466a9c6d8e),
    ("cut_cylinder", 0x62aa716fe168434c, 0x83a2ba14978f0b0b),
    ("measured_web", 0x0f7ebbba863dbd6a, 0x5132b64f2ec245cb),
    ("boss_union", 0x51a5d345d1924d24, 0x6632d620a8ff5b33),
    ("die_fillet", 0xc19b1097f6f7ebf3, 0x8f0afb80b53dcc66),
    ("die_chamfer", 0xdc5fe2d6d7033c68, 0x11deb8f300eb8cc2),
    ("die_pips", 0x9fdb4566dce6882d, 0x24811dedc73230f4),
    ("heat_sink_fins", 0xc9f255e5f344b455, 0xb1c52ca0771195f8),
    ("die_tool", 0xebe33ebbcca8c1ec, 0xbee2f8e89c69fd49),
    ("face_sketch", 0x8d423ef73f55b3d5, 0x0768994dd185306a),
    ("part_select", 0x618c686b956524f0, 0x15b44130613d39d6),
    ("loft_prism", 0x6ce9439a092a1304, 0x60fd222c8b71e3d6),
    ("die_composed", 0xe4fb7f068ed46951, 0xc8972f50fe776583),
    ("die_composed_tour", 0x8f6423172611a30d, 0x79a850824cb49478),
    ("plate_param", 0x9459702a72a7f5ca, 0x011fbbc20b608e9d),
    ("kiss_carry", 0x8a551364760f695c, 0x2982dd01cd27503c),
    ("tube_ring", 0xfa25d81018b2cdae, 0x57ec438e6ce81565),
    ("tube_arc", 0xf5790d82c541a72c, 0xb325d4a2f7761606),
    ("hollow_tube_elbow", 0x4efec1bb11495af2, 0xbf62e3fb1b94b9f5),
    ("hollow_tube_ring", 0x0286f6f224472673, 0x0e161b68e78729c2),
    ("reshaped_rod", 0xe2c27ef63b18e8c4, 0x6e198d887a4f24df),
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
