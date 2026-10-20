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
///
/// **Re-pinned for INTENT stage 3 C** (a mate relates two poses), every
/// row, the persisted text with it (it spells the pin): each document
/// places its copy where it stands by a loose `Place` and the named
/// interim's world pin (`DocEdit::PinWhereItStands`), where one edit
/// minted the placement holding its world pin, so the placement's id
/// and every id drawn after it moved. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// **Re-pinned for a seam citing the parent face** (N3): `crossing_slots`'
/// table only, whose two seams beside the merged floor cite its
/// `Merged` parent where they cited one floor; its persisted text held.
///
/// **Re-pinned for INTENT stage 3 D** (Transform retires into a
/// placement), every row, the persisted text with it: each document was
/// re-spelled without `Datum` or `Transform` (a construction is built in
/// its own coordinates and placed into the world by a `Place`), so its
/// nodes, their mint preimages, the names they mint and the saved text
/// all moved.
///
/// **Re-pinned merging main into D**: `corner_table`'s and
/// `declared_tangency`'s persisted text, their tables held — each
/// placement's values are written from the evaluated geometry its mate
/// reads, which main's merge moved by rounding (the chord's director
/// spelled by its components, the union's fold); `kitchen_sink`'s
/// tables and text, its lone block's copies hung below the world's xy as
/// its retired `SetExtrudeSide` edit had them.
///
/// **Re-pinned for the identity bundle** (FORK-REST): a copy standing
/// where its body stands is placed by no mate and the constant `0` on
/// every freedom, written as one `AddValues`, which mints a different number of ids
/// from the plane mate and three values it replaces, so every node
/// minted after a stand moved its id. The persisted text moved in every
/// document holding a stand, the bundle it saves being new; the name
/// tables moved in the five documents that mint a node after a stand
/// (`heat_sink`, `declared_tangency`, `kitchen_sink`, `cut_cylinder`,
/// `part_select`), as `lib_g16`'s did.
///
/// Re-pinned for exact turns (INTENT stage 3 D), name tables and
/// persisted text: `die_tool` (its ball's exact heading lets the
/// placement gesture write its spin as the constant `0`, one variable
/// fewer, so every later id moved); `die_pips`, `die_composed` and the
/// tour die (pips standing on lines stated from the cube's corner); and
/// `kitchen_sink` (its revolve upright by a bundle with no mate spun
/// `turn/4`, its split's plane and pattern's axis read off its faces).
/// Every other document held, across main's merge at `557e581a64` too.
///
/// `die_tool` moved again when its master ball came to stand on the
/// cube's corner frame (`corner_frame`) by an axis mate whose slide and
/// spin are the constant `0`, where `place_against` had placed it by a
/// read-back frame that minted 42 more ids, and its explicit rule's
/// frames came to be built from the fraction of a turn each is. Every
/// other document held.
///
/// `tube_arc`'s and `hollow_tube_elbow`'s persisted text moved when each
/// tube came to stand about the world's `+y` by a bundle with no mate
/// spun `−turn/4`, where it had stood where its body stands; their name
/// tables held, and so did every other document.
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 0xb4db9802b0b606f7, 0x8af513933244b575),
    ("corner_table", 0x45e0ac4118bb214d, 0x8716f69f07ef2d45),
    ("heat_sink", 0x1d6578a7bacddcf9, 0x222137d882b50b9d),
    ("crossing_slots", 0xa509a00ec2edc86f, 0x494514f9adbb072f),
    ("nested_islands_105", 0x6c1987abab93a799, 0xb8db29aa6a130e58),
    (
        "nested_islands_106_depth1",
        0x2e52dc2c55161a19,
        0x88fdb36877d8f89b,
    ),
    (
        "nested_islands_106_depth2",
        0x671438aa82666f7f,
        0x9e765a7498cfa656,
    ),
    ("declared_tangency", 0x51206b9c784061eb, 0x303c312bc828657a),
    ("kitchen_sink", 0xe18313feb241d472, 0x53c127394e06a682),
    ("cut_cylinder", 0xe723fe79aa24b696, 0x7dff515702541057),
    ("measured_web", 0xf21138cd48fb53df, 0x66b24d0b5e0fd191),
    ("boss_union", 0xd03552cff4cfc65b, 0x82fc0e1b54a1f27f),
    ("die_fillet", 0xa96f8a70800a1e51, 0x9277c3dbd74f7da5),
    ("die_chamfer", 0x370ebad78cec9015, 0x18dab54353cab960),
    ("die_pips", 0x1fff1b83a30b5c99, 0x0d819577eaa47983),
    ("heat_sink_fins", 0xd2143bd13f9aeb93, 0xd0d0bd7b24a38c2d),
    ("die_tool", 0x5f3d123cc3ff8f9a, 0xbd5ef3af13e3a895),
    ("face_sketch", 0x9cff6e4f5211da3d, 0x8bd8c95b2263b17d),
    ("part_select", 0x295f529b85ed0c9b, 0xdeb5a09d1f184d27),
    ("loft_prism", 0x9cb4bf86a35998f3, 0x49d4f40ea014df5c),
    ("die_composed", 0xf30def740ea93329, 0x540b83453b9c46a2),
    ("die_composed_tour", 0x4fa390ec962ef734, 0x6aaf8417114b0da9),
    ("plate_param", 0x1374cf899f16a081, 0xc61e5981d4483dad),
    ("kiss_carry", 0x94d99109484dbaad, 0x0b50925beeee0a40),
    ("tube_ring", 0xb1623d89f704044f, 0xf71602ffe004bfc3),
    ("tube_arc", 0xe566cd7d88cf25b3, 0x893551b32708fee4),
    ("hollow_tube_elbow", 0x469aba62d2ad9f95, 0x9698badaf1c676e2),
    ("hollow_tube_ring", 0x7d6efd15feea4270, 0xe910e65bbd5c2740),
    ("reshaped_rod", 0x2c0687528028d32a, 0x12c682ed7bc2e302),
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
