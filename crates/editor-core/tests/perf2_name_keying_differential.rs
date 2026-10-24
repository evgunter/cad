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
/// spin are the constant `0`, where the retired place-against helper had placed it by a
/// read-back frame that minted 42 more ids, and its explicit rule's
/// frames came to be built from the fraction of a turn each is. Every
/// other document held.
///
/// `tube_arc`'s and `hollow_tube_elbow`'s persisted text moved when each
/// tube came to stand about the world's `+y` by a bundle with no mate
/// spun `−turn/4`, where it had stood where its body stands; their name
/// tables held, and so did every other document.
///
/// **Re-pinned for a seam's sides and a crossing's lines named in the
/// boolean's own table, in name order** (N3): the tables of the nine
/// documents holding a subtract; the persisted text of `die_composed`
/// and `die_composed_tour`, whose stored selections name their pip rims
/// by the subtract's seams, and the tour's ids after its fillets with
/// them. No point moved
/// (`m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`).
///
/// Re-pinned merging main at `be82e5ad6b` into D, `die`,
/// `crossing_slots`, the three `nested_islands`, `kitchen_sink`,
/// `cut_cylinder`, `die_pips`, `die_tool`, `part_select`,
/// `die_composed` and the tour die: main's N3 names each seam side and
/// crossing line with the read it came in through, and in D a later
/// operand is a placed copy, so the names those documents' booleans
/// spell moved under both changes and each value is neither tree's.
/// The persisted text moved in `die_composed` and the tour die alone:
/// their fillets store the pip rim's names.
///
/// Re-pinned, `die` alone, when its pips came to stand on frames stated
/// from the cube's corner by whole quarter turns and dyadic slides (a
/// frame mate each, `fixture::square_frame`), where the retired place-against helper had
/// written `atan2`'s rounded turns: the die's world vertices are the
/// merge base's (`f79829b57d`) bit for bit, 176 of 176, and its
/// placements, their ids and the names under them are new.
///
/// Re-pinned for D′1 (a placement names its target space): each
/// construction defines its space beside its body and every bundle is
/// written whole, so ids, the names spelled with them and the persisted
/// text (a `target` on every placement, a `space` row per construction)
/// moved in every document; `m10_p_fence`'s ids-masked geometry fence
/// held.
///
/// **Re-pinned for PR 5616** (a mirror's seam is the plane's face read
/// twice), taken merged over D′2 (PR 5613, mates read geometry), which
/// left this row red on main: D′2 moved every corpus document's ids,
/// and this PR's `test_support::plane_on` reads `symmetric_plate`'s
/// seam face plainly, so its ids moved too. Ids only:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held unmoved.
///
/// **Re-pinned for D′3** (a placement copies one source, the product is
/// a slot): a copy carries its source's names verbatim, with no layer
/// naming a placement, each product reads a `⊔` of its world copies, and
/// every id drawn after one moved. Names and ids only:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// holds with the `⊔` nodes set aside.
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 0xe89b307d08b8814d, 0x2fcc63c2e71cf7c3),
    ("corner_table", 0xcded7ceb0d2fa021, 0x5062d71cc9ac91ca),
    ("heat_sink", 0x2762221dff53d1ed, 0x6153160cd6b25035),
    ("crossing_slots", 0x819b5589de335123, 0x06a34fcd9ffb534a),
    ("nested_islands_105", 0xb808da3cc1580432, 0xee9cfae3a4c74cc8),
    (
        "nested_islands_106_depth1",
        0x7cc49f167c6f18a4,
        0x72eab774a09508e4,
    ),
    (
        "nested_islands_106_depth2",
        0x0ff02616ad0fcca9,
        0x0c8b65dbfa006f99,
    ),
    ("declared_tangency", 0x0e962063407e3b05, 0x0634581982b9dfd5),
    ("kitchen_sink", 0xf2c6cc5000df2b08, 0xa5a96d42df5908a5),
    ("cut_cylinder", 0x35a106126dc3aca0, 0xf746b3c7d6dda1f9),
    ("measured_web", 0x77c748bf4d7c5268, 0x2b77dfdce2a55c31),
    ("boss_union", 0x68a6edee2959e2e9, 0xa68c183852a590be),
    ("die_fillet", 0x8ddc8e8e4cc1cc18, 0x44273f0b972dadee),
    ("die_chamfer", 0x4d9e144d4e8be9b4, 0xed273f48cb5ca907),
    ("die_pips", 0xd22e78db2937ae80, 0xdd1a39df0899a8e6),
    ("heat_sink_fins", 0x9c34cfa0fb53a7b7, 0x08d941cbe7d15871),
    ("die_tool", 0x0670c0d4ee7e0588, 0x20bef543c53d54c8),
    ("face_sketch", 0xef2092a04cea77eb, 0xb221cfbfe1b6a37b),
    ("part_select", 0x1e9dd41792e566e6, 0xececff8c053acde3),
    ("symmetric_plate", 0x4481fd1b6f905ec1, 0xd50cfd94a5cbf8ea),
    ("loft_prism", 0xf84090b7955f6cd0, 0x9d1660151c8302a0),
    ("die_composed", 0xcddf06ccb7838c85, 0xcab270881e528d7f),
    ("die_composed_tour", 0x6b9947c5fd117525, 0xcb372e398a5b01b0),
    ("plate_param", 0x15f4a7d31d3a282f, 0x14add331dec4d188),
    ("kiss_carry", 0x8fe6a1043c38b6c7, 0xae53d94a04fccd65),
    ("tube_ring", 0x56fd2415a347d921, 0xab53801334f7ed9b),
    ("tube_arc", 0x3bcb162e9e7923d5, 0x299b0f3445b95489),
    ("hollow_tube_elbow", 0xdbf97cdac2df4d30, 0x88b19605f5015b9c),
    ("hollow_tube_ring", 0xf0a59d29f0a5ef21, 0x743a5c3e6051831a),
    ("reshaped_rod", 0xd37dd1691945dda0, 0x40e1518b88d07303),
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
