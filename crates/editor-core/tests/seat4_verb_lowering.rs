//! **The migrated verbs moved onto the verb substrate and nothing
//! observable moved with them.** These are the pins for that claim —
//! the blend pair's first (SEAT-4), and the boolean's beside them on
//! the same method (SEAT-5).
//!
//! The lowering the blend nodes run through is one generic function
//! driven by a per-verb correspondence (`editor_core::verbs::blend`),
//! the boolean runs through the two-operand lowering driven by its own
//! (`editor_core::verbs::boolean`), the kernel doors are reached
//! through the `verbs` run doors, and every migrated content tag is a
//! function of the kernel's name for the verb rather than a number
//! written inline. Every one of those is a re-plumbing, and a
//! re-plumbing's failure mode is a difference nobody looks for. So:
//!
//! - **The wire format**: a document carrying BOTH blends saves, loads
//!   and re-saves byte-identically, and the bytes carry the same schema
//!   version they did before; a registered boolean document (declared
//!   contact included) does the same.
//! - **The evaluation**: each pinned document's body geometry and name
//!   table digest to committed constants, per document, so a red says
//!   WHICH document moved.
//!
//! # What already covers this, and what these rows add
//!
//! Two corpus-wide goldens overlap this suite deliberately:
//! `m10_p_fence` digests every body POINT's bits across the whole
//! registry, and `lib_g16_corpus_name_digests` digests every document's
//! name tables. Either would have caught a lowering that changed
//! geometry or names — and being corpus-wide, neither says which
//! document did it, neither covers a document that carries both
//! blends at once (the registry has one of each, in separate files),
//! and neither reaches a boolean value's non-body halves (the result
//! classification and the surviving declared contacts, which the
//! digest here feeds). These rows are the per-document form. A red in
//! both places is one fact; a red only here is a migrated-verb one.
//!
//! # Why the digests are eps-independent
//!
//! Nothing rendered from a classification band enters them (the
//! `m10_p_fence` rule): point bits and name spellings only, and no
//! outcome text. The hosted matrix samples one tolerance row per run,
//! so a constant that moved with eps would gate one row in three.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use editor_core::{
    LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId, StableName, persist,
};
use fixture::digest::digest;
use fixture::{len, prism_edges, tol};

/// The cube side and the two blend sizes, all dyadic.
const L: f64 = 1.0;
const R: f64 = 0.125;
const D: f64 = 0.125;

/// **One document carrying both blend nodes.** Two siblings over one
/// cube: the same twelve authored edges, filleted on one branch and
/// chamfered on the other, so a single file exercises both wire
/// spellings and both lowering paths.
fn both_blends() -> BothBlends {
    let mut r = corpus::Recorder::new();
    // The saved SNAPSHOT is the empty document and the log is
    // everything (the recorder's convention), so a load replays the
    // whole recipe through `apply`'s doors — which is what makes the
    // round-trip a test of the wire spelling of every node.
    let snapshot = r.doc.clone();
    let square = LoopProgram::polygon([(0.0, 0.0), (L, 0.0), (L, L), (0.0, L)]).unwrap();
    let profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![square],
        ids: Vec::new(),
        on: None,
    }));
    let cube = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(L),
    });
    let edges: Vec<StableName> = prism_edges(&r.doc, cube, 4);
    let filleted = r.insert(Node::fillet(cube, len(R), edges.clone()));
    let chamfered = r.insert(Node::chamfer(cube, len(D), edges));
    BothBlends {
        snapshot,
        doc: r.doc,
        edits: r.edits,
        blends: [filleted, chamfered],
    }
}

/// The two-blend fixture: what to save, what to evaluate, and the two
/// blend nodes' ids.
struct BothBlends {
    snapshot: ProfileDoc,
    doc: ProfileDoc,
    edits: Vec<editor_core::DocEdit<ProfileProgram>>,
    blends: [RecipeNodeId; 2],
}

/// **The wire format is untouched**: save → load → save reproduces the
/// bytes exactly, for a document carrying a fillet and a chamfer.
///
/// Byte equality is the whole assertion. A schema bump, a field rename,
/// a reordered payload or a changed number format each break it, and
/// none of them would be visible in an evaluation digest.
#[test]
fn a_fillet_and_chamfer_document_round_trips_byte_identical() {
    let fixture = both_blends();
    let first =
        persist::save(&fixture.snapshot, &fixture.edits, tol()).expect("the document saves");
    let loaded = persist::load(&first, tol()).expect("its own bytes load back");
    assert_eq!(loaded.edits, fixture.edits, "the edit log did not survive");
    assert!(
        loaded.doc.bit_eq(&fixture.doc),
        "the replayed document is not bit-identical to the authored one"
    );
    let second = persist::save(&loaded.snapshot, &loaded.edits, tol())
        .expect("the loaded document re-saves");
    assert_eq!(
        first, second,
        "a fillet+chamfer document does not round-trip byte-identically"
    );
    // The header the bytes carry, asserted separately: the format has
    // no version line (the persist module docs say why), so the whole
    // header is the document's `id:` line.
    assert!(
        first.starts_with(&format!("id: {}\n", fixture.doc.id())),
        "the saved header is not the document's id line"
    );
}

/// **Both blend nodes evaluate**, in one document, through the one
/// generic lowering — the fillet under a fillet's node id and the
/// chamfer under a chamfer's, each with a full name table.
#[test]
fn both_blends_evaluate_in_one_document() {
    let fixture = both_blends();
    let [filleted, chamfered] = fixture.blends;
    let ev = corpus::eval::<f64>(&fixture.doc);
    let failures = corpus::failures(&ev);
    assert!(
        failures.is_empty(),
        "the two-blend document failed: {failures:?}"
    );
    for id in [filleted, chamfered] {
        let table = ev.table_at(id).expect("the blend node produced a value");
        assert!(
            table.iter().count() > 0,
            "node {id:?} produced an empty name table"
        );
    }
    // The two branches are different geometry under different node ids,
    // so their tables share no name: this is what "the discrimination
    // is the minting id" means, executed.
    let names = |id: RecipeNodeId| -> std::collections::BTreeSet<String> {
        ev.table_at(id)
            .unwrap()
            .iter()
            .map(|(n, _)| format!("{n:?}"))
            .collect()
    };
    let (a, b) = (names(filleted), names(chamfered));
    assert!(
        a.intersection(&b).next().is_none(),
        "the fillet's and chamfer's tables collide"
    );
}

// The digest these rows pin with is `fixture::digest::digest` — ONE feed
// for every verb-migration suite, with what it covers (and the measured
// reasons each half is load-bearing, first found here at SEAT-4) stated
// at that home. The constants below are this suite's own.

/// **The existing blend documents' evaluations are bit-identical**,
/// body and name table, one committed number each.
///
/// The numbers were taken on this branch and re-taken on a PRE-CHANGE
/// tree with this same file copied onto it — the whole suite, this row
/// and the round-trip both, passes unchanged there. That differential
/// is what "nothing observable moved" means here; without it the
/// constants would only say the branch agrees with itself.
///
/// They are goldens in the ordinary sense — when one moves the question
/// is whether the new behaviour is right, never how to restore the old
/// number.
///
/// RE-BLESSED for the sketch frame, and the differential above no
/// longer stands behind these numbers: a profile's plane became a
/// document node, so each of these documents gained frames and every
/// later node was renumbered. The digest feeds each node's id and each
/// `StableName` — which carries the id of the node that minted it — so
/// it moves for renumbering alone. What did NOT move is the body half:
/// the corpus's exact mass pins (`m4_pr8_corpus::exact_mass_pins_hold`)
/// and the realized-vs-idealized bit equality (`m5_pr8_bvh_diff`) were
/// green across this change untouched, and those are id-free.
///
/// RE-BLESSED for the axis-order orthonormal basis: the digest feeds
/// each surface's `Debug`, and every planar carrier's stored `u_ref`
/// is now `e_z × n` or `e_y × n` — whichever axis the comparison
/// `|n.z| ≤ max(|n.x|, |n.y|)/2` picks — divided by its own length.
/// The plane's LOCUS did not move — origin and normal are
/// bit-identical, which the STEP fixtures' record-level diff shows
/// directly — and the id-free body rows (`m4_pr8_corpus`'s exact mass
/// pins, `m5_pr8_bvh_diff`'s realized-vs-idealized bit equality) were
/// green across the change untouched.
///
/// Re-blessed again when step ids became digests of the document's mint
/// chain: the names spell different ids, and the same id-free pins held. And again when node ids moved onto that mint, for the same reason
/// and with the same pins holding.
///
/// RE-BLESSED for INTENT-LITERALS PR C (a slot holds a variable): every
/// node is minted from slots holding variable ids, so every id moved
/// and this digest feeds ids. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held untouched.
///
/// RE-BLESSED for INTENT stage 2 PR C (the product is the world):
/// each document now places its bodies, and every placement is a node
/// with a value and a name table of its own, so the evaluation this
/// digest walks holds those copies. What each document delivers did
/// not move: `intent_s2_c_world`'s migration check holds each product
/// to its pre-C digest.
///
/// RE-BLESSED, every row that moved, for INTENT stage 4 E (booleans
/// glue on Zero): a body no longer carries provenance side tables
/// (`GeomSource` stamps, field and axis sources), and the digest feeds
/// each body's `Debug`. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// RE-BLESSED for INTENT stage 2 PR E (a selection is a variable): each
/// blend reads a selection variable its insert mints, so the blend's id
/// moved and every name it mints with it. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held untouched. Merged over stage 4 E, the two numbers were re-taken
/// once more, where both changes meet.
///
/// RE-BLESSED when an intrinsic edge description's two surfaces became a
/// set (`geom_brep::SurfacePair`), re-taken on the tree holding every
/// change above: the digest feeds each edge's description `Debug`,
/// which spells `pair: {a, b}` in key order where it spelled `s1`, `s2`
/// in the builder's order. No body moved: `m4_pr8_corpus`'s exact mass
/// pins and `m5_pr8_bvh_diff` held.
///
/// RE-BLESSED for INTENT stage 3 B (a placement is the bundle of
/// mates): a placement is a `Place` holding a world pin where it was a
/// `PlaceInWorld`, so its stored node, and with it its mint preimage,
/// moved, and every id the mint chain draws after it. No outcome or
/// point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// RE-BLESSED, every row, for INTENT stage 3 C (a mate relates two
/// poses): each document places its copy where it stands by a loose
/// `Place` and the named interim's world pin, where one edit minted
/// the placement holding its world pin, so the placement's id and every
/// id drawn after it moved. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// RE-BLESSED, every row, for INTENT stage 3 D (Transform retires into
/// a placement): each document was re-spelled without `Datum` or
/// `Transform` (a construction is built in its own coordinates and
/// placed into the world by a `Place`), so its nodes, their ids and the
/// names they mint moved.
///
/// RE-BLESSED, both rows, for a minted reference that comes from the
/// inputs (PR #5547): the digest feeds each plane's `u_ref`, which a
/// cap now takes from the sketch's `x` axis and an extruded wall from
/// its segment, where `orthonormal_basis` picked it off the axes. No
/// outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// RE-BLESSED for D′1 (a placement names its target space): each
/// construction defines its space beside its body and every bundle is
/// written whole, so ids moved, and with them the names and value
/// tokens this digest feeds; `m10_p_fence`'s ids-masked geometry fence
/// held, so no point moved.
///
/// RE-BLESSED for D′3 (a placement copies one source, and the product is
/// a slot reading a `⊔`): a copy carries its source's names with no
/// `Placed` layer, and a `⊔` keys each member's names by its read, so
/// the name tables this digest feeds moved, and the `⊔` node moved the
/// ids after it. No point moved: `m10_p_fence`'s ids-masked row, with
/// the corpus's `⊔` nodes set aside, is main's pin.
///
/// RE-BLESSED, both rows, for a certificate that keeps its turn: the
/// digest feeds each edge curve's `Debug`, which now spells the
/// certificate's `turn`. No outcome or point moved.
///
/// RE-BLESSED, the rows that moved, over main's merge into D′3 at
/// `1235f4c050`: D′2's moves (main red, filed as
/// `main-red-the-seat-and-name-table-goldens-moved-at-dprime2`),
/// stage 4 I's ids and the walls' reads, beneath
/// D′3's names and ids. `m10_p_fence`'s ids-masked row held.
///
/// RE-BLESSED for limb 2 for every carrier (PR 5728): check 2's turn
/// moved from the edge's `Certificate` onto the edge, which changes the
/// edge curve's `Debug` the digest feeds. With that `Debug` written in
/// main's shape every digest here reproduces main's value at all three
/// ε rows, so nothing else moved.
///
/// RE-BLESSED when `Intersection`'s witness became its selector
/// (`selector: Witness(p)` where it spelled `witness: p`), which changes
/// the edge curve's `Debug` the digest feeds. With that `Debug` written in
/// main's shape every digest here reproduces main's value at all three
/// ε rows, so nothing else moved.
#[test]
fn the_blend_documents_evaluate_to_their_committed_digests() {
    let mut moved: Vec<String> = Vec::new();
    for (name, want) in [
        ("die_fillet", 0xa849_edde_6a1d_0e11_u64),
        ("die_chamfer", 0xc9bc_588c_0cb6_a135),
    ] {
        let doc = corpus::documents()
            .into_iter()
            .find(|d| d.name == name)
            .expect("the document is registered");
        let ev = corpus::eval::<f64>(&doc.doc);
        let got = digest(&ev);
        println!("seat4 {name}: {got:#018x}");
        if got != want {
            moved.push(format!("{name}: {got:#018x} (want {want:#018x})"));
        }
    }
    assert!(
        moved.is_empty(),
        "these documents' evaluations moved — body or name table:\n{}",
        moved.join("\n")
    );
}

/// **A registered boolean document's bytes survive the migration**:
/// save → load → save reproduces the file exactly. The document is
/// `crossing_slots` — two subtracts — so the wire spelling under pin
/// includes the boolean node's whole payload: the op and both operand
/// edges. The
/// corpus-wide round-trip covers the same bytes; this is the
/// per-document form beside the digest row, so a red here names the
/// boolean rather than the registry.
#[test]
fn a_boolean_document_round_trips_byte_identical() {
    let doc = corpus::documents()
        .into_iter()
        .find(|d| d.name == "crossing_slots")
        .expect("the document is registered");
    let snapshot = ProfileDoc::empty_derived("seat5_boolean_roundtrip", tol());
    let first = persist::save(&snapshot, &doc.edits, tol()).expect("the document saves");
    let loaded = persist::load(&first, tol()).expect("its own bytes load back");
    assert_eq!(loaded.edits, doc.edits, "the edit log did not survive");
    let second = persist::save(&loaded.snapshot, &loaded.edits, tol())
        .expect("the loaded document re-saves");
    assert_eq!(
        first, second,
        "a boolean document does not round-trip byte-identically"
    );
}

/// **The existing boolean documents' evaluations are bit-identical**
/// through the two-operand verb lowering — the SEAT-4 differential
/// method on the boolean's own channels.
///
/// The three documents split the semantics between them:
/// `crossing_slots` runs subtract twice, the second against a floor
/// coplanar with the first cavity's, a rest contact the subtract glues;
/// `heat_sink` runs the union chain; `kiss_carry` is the one whose
/// boolean values carry NON-EMPTY surviving contacts (a corner kiss the
/// first union discovers, then the same record the second union carries
/// from its operand). The digest feeds the boolean value's kind and
/// contacts beside the stamped body and the name table, so the
/// constants cover exactly what the pair lowerings write.
///
/// `kiss_carry` is load-bearing for the contacts half, MEASURED: on
/// the other two documents alone, replacing the lowering's contact
/// carry with `ContactRecords::default()` leaves every constant
/// standing (their surviving records are empty — glued REST
/// contacts are consumed into seam structure), so the channel was fed
/// but dead. With `kiss_carry` pinned that same mutation reds its
/// row (the other two constants stand, re-measured); deleting the
/// `stamp_minted` write reds the suite at `crossing_slots` already.
///
/// All three numbers were taken on this branch and re-taken on a
/// PRE-CHANGE tree (extracted main, with this file and the
/// `kiss_carry` corpus files copied onto it) — the whole suite passes
/// unchanged there, `kiss_carry`'s row included, since the document
/// authors through doors the migration did not add. That differential
/// is what "nothing observable moved" means here; without it the
/// constants would only say the branch agrees with itself.
///
/// RE-BLESSED for the sketch frame, and the differential above no
/// longer stands behind these numbers: a profile's plane became a
/// document node, so each of these documents gained frames and every
/// later node was renumbered. The digest feeds each node's id and each
/// `StableName` — which carries the id of the node that minted it — so
/// it moves for renumbering alone. What did NOT move is the body half:
/// the corpus's exact mass pins (`m4_pr8_corpus::exact_mass_pins_hold`)
/// and the realized-vs-idealized bit equality (`m5_pr8_bvh_diff`) were
/// green across this change untouched, and those are id-free.
///
/// RE-BLESSED for the axis-order orthonormal basis: the digest feeds
/// each surface's `Debug`, and every planar carrier's stored `u_ref`
/// is now `e_z × n` or `e_y × n` — whichever axis the comparison
/// `|n.z| ≤ max(|n.x|, |n.y|)/2` picks — divided by its own length.
/// The plane's LOCUS did not move — origin and normal are
/// bit-identical, which the STEP fixtures' record-level diff shows
/// directly — and the id-free body rows (`m4_pr8_corpus`'s exact mass
/// pins, `m5_pr8_bvh_diff`'s realized-vs-idealized bit equality) were
/// green across the change untouched.
///
/// Re-blessed again when step ids became digests of the document's mint
/// chain: the names spell different ids, and the same id-free pins held. And again when node ids moved onto that mint, for the same reason
/// and with the same pins holding.
///
/// RE-BLESSED, `heat_sink` only, when declaring a variable began
/// minting its id on the document's chain: every node minted after a
/// declare was renumbered, and this digest feeds ids. The id-free body
/// rows (`m4_pr8_corpus`'s exact mass pins, `m5_pr8_bvh_diff`) held
/// untouched, and every row of a document that declares nothing held
/// its word.
///
/// RE-BLESSED for INTENT-LITERALS PR C (a slot holds a variable): every
/// node is minted from slots holding variable ids, so every id moved
/// and this digest feeds ids. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held untouched.
///
/// RE-BLESSED for INTENT stage 2 PR C (the product is the world):
/// each document now places its bodies, and every placement is a node
/// with a value and a name table of its own, so the evaluation this
/// digest walks holds those copies. What each document delivers did
/// not move: `intent_s2_c_world`'s migration check holds each product
/// to its pre-C digest.
///
/// RE-BLESSED, every row that moved, for INTENT stage 4 E (booleans
/// glue on Zero): a body no longer carries provenance side tables
/// (`GeomSource` stamps, field and axis sources), and the digest feeds
/// each body's `Debug`. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// RE-BLESSED when an intrinsic edge description's two surfaces became a
/// set (`geom_brep::SurfacePair`), re-taken on the tree holding every
/// change above: the digest feeds each edge's description `Debug`,
/// which spells `pair: {a, b}` in key order where it spelled `s1`, `s2`
/// in the builder's order. No body moved: `m4_pr8_corpus`'s exact mass
/// pins and `m5_pr8_bvh_diff` held.
///
/// RE-BLESSED, every row, for CONTACTHOLD's at-rest record (contact
/// records are one list of unordered cell pairs): the digest feeds each
/// value's `Debug`, and the records' `Debug` shape moved from seven
/// per-kind lists to one list of canonical pairs. No outcome, count or
/// point moved: every other row of these suites held.
///
/// RE-BLESSED, `crossing_slots` only, for the killed-lineage chase
/// (`emit_topo`'s `KilledSplits`): its second slot's bottom-rim pieces,
/// split off around stretches the subtract killed, are named as pieces
/// of that rim rather than as chords, and two floor corners' vertex
/// names moved with them. The digest feeds the name table. No body
/// moved.
///
/// RE-BLESSED, `crossing_slots` only, for the output stage's killed
/// lineage (the records of the edges the merge kills, copied into
/// `BooleanNaming::killed_lineage`): a stretch of the first cavity's
/// bottom rim the merge left is named as a piece of that rim rather
/// than as a seam, and one floor corner moved from a touch to a
/// crossing of the two rims. The digest feeds the name table. No body
/// moved.
///
/// RE-BLESSED for INTENT stage 3 B (a placement is the bundle of
/// mates): a placement is a `Place` holding a world pin where it was a
/// `PlaceInWorld`, so its stored node, and with it its mint preimage,
/// moved, and every id the mint chain draws after it. No outcome or
/// point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// RE-BLESSED, every row, for INTENT stage 3 C (a mate relates two
/// poses): each document places its copy where it stands by a loose
/// `Place` and the named interim's world pin, where one edit minted
/// the placement holding its world pin, so the placement's id and every
/// id drawn after it moved. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// RE-BLESSED, `crossing_slots` only, for a seam citing the parent face
/// (N3): the two seams beside its merged floor cite the floor's `Merged`
/// parent where they cited one floor. The digest feeds the name table.
/// No body moved.
///
/// RE-BLESSED, every row, for INTENT stage 3 D (Transform retires into
/// a placement): each document was re-spelled without `Datum` or
/// `Transform` (a construction is built in its own coordinates and
/// placed into the world by a `Place`), so its nodes, their ids and the
/// names they mint moved.
///
/// RE-BLESSED, `heat_sink` only, for the identity bundle (FORK-REST): a
/// copy standing where its body stands is placed by no mate and the
/// constant `0` on every freedom, written as one `AddValues`, which
/// mints a different number of ids from the plane mate and three values it replaces.
/// So every node minted after a stand moved its id, and this digest
/// feeds ids. Every standing copy in the corpus is bit-identical to its
/// source (measured with a lane probe over `body_digest`).
///
/// RE-BLESSED, `crossing_slots` only, for a seam's sides and a
/// crossing's lines named in the subtract's own table, in name order
/// (N3). The digest feeds the name table. No body moved
/// (`m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held).
/// Re-measured merging main at `be82e5ad6b` into D, `crossing_slots`:
/// main's N3 names each seam side and crossing line with the read it
/// came in through, and in D a later operand is a placed copy, so the
/// names it spells moved under both changes and the value is neither
/// tree's. No other row moved.
///
/// RE-BLESSED, `crossing_slots`, `heat_sink` and `kiss_carry`, for a
/// minted reference that comes from the inputs (PR #5547): the digest
/// feeds each plane's `u_ref`, which a cap now takes from the sketch's
/// `x` axis and an extruded wall from its segment, where
/// `orthonormal_basis` picked it off the axes. No outcome or point
/// moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// RE-BLESSED for D′1 (a placement names its target space): each
/// construction defines its space beside its body and every bundle is
/// written whole, so ids moved, and with them the names and value
/// tokens this digest feeds; `m10_p_fence`'s ids-masked geometry fence
/// held, so no point moved.
///
/// RE-BLESSED for D′3 (a placement copies one source, and the product is
/// a slot reading a `⊔`): a copy carries its source's names with no
/// `Placed` layer, and a `⊔` keys each member's names by its read, so
/// the name tables this digest feeds moved, and the `⊔` node moved the
/// ids after it. No point moved: `m10_p_fence`'s ids-masked row, with
/// the corpus's `⊔` nodes set aside, is main's pin.
///
/// RE-BLESSED, every row, for a certificate that keeps its turn: the
/// digest feeds each edge curve's `Debug`, which now spells the
/// certificate's `turn`. No outcome or point moved.
///
/// RE-BLESSED, `crossing_slots` only, for a pair boolean's `Borders`
/// walls citing their parent as the boolean's own table names it (the
/// `From` each came in through): the second subtract's divided pieces'
/// names moved. No outcome or point moved.
///
/// RE-BLESSED, `crossing_slots` only, for a run line's sense that is no
/// longer optional (an orientation is read, never chosen): the
/// subtracts' `EdgeCrossing` names spell each line's sense bare. No
/// outcome or point moved.
///
/// RE-BLESSED, the rows that moved, over main's merge into D′3 at
/// `1235f4c050`: D′2's moves (main red, filed as
/// `main-red-the-seat-and-name-table-goldens-moved-at-dprime2`),
/// stage 4 I's ids and the walls' reads, beneath
/// D′3's names and ids. `m10_p_fence`'s ids-masked row held.
///
/// Measured again for D′3 over main's merge at `4c341b9225`, which
/// brings the run line's bare sense (above) beneath D′3's names and ids;
/// the ids-masked fence held.
///
/// RE-BLESSED for limb 2 for every carrier (PR 5728): check 2's turn
/// moved from the edge's `Certificate` onto the edge, which changes the
/// edge curve's `Debug` the digest feeds. With that `Debug` written in
/// main's shape every digest here reproduces main's value at all three
/// ε rows, so nothing else moved.
///
/// RE-BLESSED when `Intersection`'s witness became its selector
/// (`selector: Witness(p)` where it spelled `witness: p`), which changes
/// the edge curve's `Debug` the digest feeds. With that `Debug` written in
/// main's shape every digest here reproduces main's value at all three
/// ε rows, so nothing else moved.
#[test]
fn the_boolean_documents_evaluate_to_their_committed_digests() {
    let mut moved: Vec<String> = Vec::new();
    for (name, want) in [
        ("crossing_slots", 0xa187_0bd9_e3b5_134f_u64),
        ("heat_sink", 0xa26d_5873_8dc7_6915),
        ("kiss_carry", 0x2bcf_1c97_70eb_fb7a),
    ] {
        let doc = corpus::documents()
            .into_iter()
            .find(|d| d.name == name)
            .expect("the document is registered");
        let ev = corpus::eval::<f64>(&doc.doc);
        let failures = corpus::failures(&ev);
        assert!(
            failures.is_empty(),
            "{name} failed to evaluate: {failures:?}"
        );
        let got = digest(&ev);
        println!("seat5 {name}: {got:#018x}");
        if got != want {
            moved.push(format!("{name}: {got:#018x} (want {want:#018x})"));
        }
    }
    assert!(
        moved.is_empty(),
        "these documents' evaluations moved — body, value or name table:\n{}",
        moved.join("\n")
    );
}

/// **The typed empty success is pinned by an input that PRODUCES it** —
/// a disjoint intersect, authored in-suite because the corpus has no
/// empty-boolean document and the empty path needs none of the corpus's
/// other rows (no body to round-trip eps rows over, no mass to pin).
///
/// This row exists because the digest's `b"empty"` token was measured
/// FED BUT DEAD — the same class as the contacts channel before
/// `kiss_carry`: no pinned document ever took the `Empty` arm, so
/// perturbing the token left every constant standing. The value under
/// pin is asserted to actually BE `BooleanValue::Empty` first, so the
/// constant cannot go vacuous if the fixture drifts; with that,
/// perturbing the token reds THIS row while the three document
/// constants stand (measured), and a lowering that turned the typed
/// empty into anything else — or an empty result into a phantom body —
/// moves this number. The constant reproduces on the same extracted
/// pre-change tree as the document rows (the empty path predates the
/// migration), so it is a differential pin, not a self-agreement.
///
/// RE-BLESSED for the axis-order orthonormal basis: the digest feeds
/// each surface's `Debug`, and every planar carrier's stored `u_ref`
/// is now `e_z × n` or `e_y × n` — whichever axis the comparison
/// `|n.z| ≤ max(|n.x|, |n.y|)/2` picks — divided by its own length.
/// The plane's LOCUS did not move — origin and normal are
/// bit-identical, which the STEP fixtures' record-level diff shows
/// directly — and the id-free body rows (`m4_pr8_corpus`'s exact mass
/// pins, `m5_pr8_bvh_diff`'s realized-vs-idealized bit equality) were
/// green across the change untouched.
///
/// RE-BLESSED for INTENT-LITERALS PR C (a slot holds a variable): every
/// node is minted from slots holding variable ids, so every id moved
/// and this digest feeds ids. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held untouched.
///
/// RE-BLESSED when an intrinsic edge description's two surfaces became a
/// set (`geom_brep::SurfacePair`), re-taken on the tree holding every
/// change above: the digest feeds each edge's description `Debug`,
/// which spells `pair: {a, b}` in key order where it spelled `s1`, `s2`
/// in the builder's order. No body moved: `m4_pr8_corpus`'s exact mass
/// pins and `m5_pr8_bvh_diff` held.
///
/// RE-BLESSED for INTENT stage 3 D (Transform retires into a
/// placement): the fixture's profiles carry no frame and its second
/// operand is placed against the first's root by a `Place`, so its
/// nodes, their ids and the names they mint moved.
///
/// RE-BLESSED for FORK-REST: only the identity bundle as written places
/// by no map, so the copy placed against `a` at the identity by a plane
/// mate runs its map, which comes out as the exact identity and which
/// the at-rest snap had placed by no map.
///
/// RE-BLESSED when a minted plane's `u_ref` became a direction its
/// construction holds: the two prisms' caps are charted along their
/// sketch's `x` axis and their walls along their swept segments, where
/// `orthonormal_basis` picked both off the axes. No plane moved as a
/// point set.
///
/// RE-BLESSED for D′1 (a placement names its target space): each
/// construction defines its space beside its body, and the content key
/// holds a placement's target (`VERSION` 18), so ids drawn after a
/// construction and every value token moved. No point moved.
///
/// RE-BLESSED for D′2 (a value names no mate): the second operand's
/// bundle is spelled in the new vocabulary and the content key's
/// `VERSION` moved, so its ids and the names under them moved. Every
/// node's kind and vertex bits are main's (`be9b907694`).
///
/// RE-BLESSED for D′3 (a placement copies one source, and a copy carries
/// its source's names): the second operand's copy names its faces as
/// its prism does, with no `Placed` qualifier, so its name table and
/// the intersect's moved. No body and no vertex moved.
///
/// RE-BLESSED for the merge over ATREST stage 4 I (PR 5637): the
/// content key's `VERSION` moved to 21 (a mate writes no class, and this
/// branch's own bump beside it), so every value token moved. No body and
/// no vertex moved.
///
/// RE-BLESSED for a certificate that keeps its turn: the digest feeds
/// each edge curve's `Debug`, which now spells the certificate's `turn`.
/// No outcome or point moved. Re-measured with both merged.
///
/// RE-BLESSED for limb 2 for every carrier (PR 5728): check 2's turn
/// moved from the edge's `Certificate` onto the edge, which changes the
/// edge curve's `Debug` the digest feeds. With that `Debug` written in
/// main's shape every digest here reproduces main's value at all three
/// ε rows, so nothing else moved.
///
/// RE-BLESSED when `Intersection`'s witness became its selector
/// (`selector: Witness(p)` where it spelled `witness: p`), which changes
/// the edge curve's `Debug` the digest feeds. With that `Debug` written in
/// main's shape the digest reproduces main's value at all three ε rows,
/// so nothing else moved.
#[test]
fn an_empty_boolean_evaluates_to_its_committed_digest() {
    let mut r = corpus::Recorder::new();
    let square =
        |x0: f64| LoopProgram::polygon([(x0, 0.0), (x0 + L, 0.0), (x0 + L, L), (x0, L)]).unwrap();
    let pa = r.insert(Node::Profile(ProfileProgram {
        loops: vec![square(0.0)],
        ids: Vec::new(),
        on: None,
    }));
    let a = r.insert(Node::Extrude {
        profile: pa.into(),
        distance: len(L),
    });
    let pb = r.insert(Node::Profile(ProfileProgram {
        loops: vec![square(3.0)],
        ids: Vec::new(),
        on: None,
    }));
    let b = r.insert(Node::Extrude {
        profile: pb.into(),
        distance: len(L),
    });
    let b = r.against(a, b, &editor_core::Frame::IDENTITY);
    let boolean = r.insert(Node::Intersect {
        members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
    });
    let ev = corpus::eval::<f64>(&r.doc);
    let failures = corpus::failures(&ev);
    assert!(failures.is_empty(), "the fixture evaluates: {failures:?}");
    assert!(
        matches!(
            &ev.value(boolean).expect("the boolean has a value").payload,
            editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Empty)
        ),
        "a disjoint intersect is the typed empty; the fixture no longer produces it"
    );
    let got = digest(&ev);
    println!("seat5 empty_intersect: {got:#018x}");
    assert_eq!(
        got, 0x53a4_d4cd_70d2_b713,
        "the empty-boolean evaluation moved — value token, bodies or name tables"
    );
}
