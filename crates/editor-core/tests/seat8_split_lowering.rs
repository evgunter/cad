//! **The split moved onto the verb substrate and nothing observable
//! moved with it** — the SEAT-4/5/7 method, for the first verb whose
//! result is two-sided.
//!
//! `Node::Split` builds a `verbs::Split`, runs it through the split
//! door, takes its birth record out of the closed record channel and
//! its two sides out of the door's own out-type, and emits names from
//! the record. A re-plumbing's failure mode is a difference nobody
//! looks for — so:
//!
//! - **The wire format**: a registered document carrying a split AND
//!   two projections off it saves, loads and re-saves byte-identically.
//! - **The evaluation**: every split-carrying corpus document's bodies
//!   and name tables digest to committed constants, per document, so a
//!   red says WHICH document moved — and the digest feeds BOTH sides of
//!   the split's value, with an empty side as its own token.
//! - **The empty side is a channel that can red**: the corpus's three
//!   split documents all cut through their operand, so no registered
//!   evaluation ever takes the `Empty` arm; an in-suite document whose
//!   plane misses the body is pinned so that token is fed by a real
//!   value and not just written into the feed (the lesson the boolean's
//!   typed-empty and contacts channels taught, twice).
//! - **What the projection needs from the two-sided value** is pinned
//!   where the projection reads it: a half selected by ROLE, an empty
//!   half refused typed, a present half handed on as the split's own
//!   body.
//!
//! What already covers this and what these rows add is the same
//! division SEAT-4 recorded: `m10_p_fence` digests every body point's
//! bits corpus-wide and `lib_g16_corpus_name_digests` digests every
//! name table, so either would catch a lowering that changed geometry
//! or names — and neither says which document did it, and neither sees
//! an `Empty` side at all
//! (there is no body to digest and no name to emit for one).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use corpus::{Recorder, eval, failures};
use editor_core::{
    Node, NodeErrorKind, NodeResult, ProfileDoc, RecipeNodeId, SplitHalf, SplitSide, ValuePayload,
    persist,
};
use fixture::digest::digest;
use fixture::{len, square, tol};

/// The registered split documents, by name: a tilted plane through a
/// cylinder (curved section edges), a plane through a box with both
/// halves read by port and unioned back (the DM3 payoff), and the kitchen
/// sink's mid-height cut of a declared union. Every one cuts through
/// its operand: two bodies out, no empty side anywhere — measured by
/// `the_corpus_has_no_split_with_an_empty_side`, not read off.
const SPLIT_DOCUMENTS: [&str; 3] = ["cut_cylinder", "part_select", "kitchen_sink"];

/// Every split value in an evaluation, as `(above is empty, below is
/// empty)`.
fn split_sides(ev: &editor_core::Evaluation<f64>) -> Vec<(bool, bool)> {
    ev.order
        .iter()
        .filter_map(|id| match ev.value(*id).map(|v| &v.payload) {
            Some(ValuePayload::Split { above, below }) => Some((
                matches!(above, SplitSide::Empty),
                matches!(below, SplitSide::Empty),
            )),
            _ => None,
        })
        .collect()
}

/// **The corpus's split documents are exactly [`SPLIT_DOCUMENTS`], and
/// none of their splits has an empty side** — a measurement, so that a
/// fourth registered split, or a plane that stops cutting through, is
/// noticed here rather than read off a comment. The empty-side row
/// below exists BECAUSE this count is zero: the day it is not, the
/// in-suite document is no longer the only input feeding that token
/// and the registered one should carry the pin.
#[test]
fn the_corpus_has_no_split_with_an_empty_side() {
    let mut with_split: Vec<&str> = Vec::new();
    let mut empty_sides = 0usize;
    for doc in corpus::documents() {
        let ev = eval::<f64>(&doc.doc);
        let sides = split_sides(&ev);
        if !sides.is_empty() {
            with_split.push(doc.name);
        }
        empty_sides += sides
            .iter()
            .filter(|(above, below)| *above || *below)
            .count();
    }
    with_split.sort_unstable();
    let mut expected = SPLIT_DOCUMENTS.to_vec();
    expected.sort_unstable();
    assert_eq!(
        with_split, expected,
        "the registry's split-carrying documents are not the ones this suite pins"
    );
    assert_eq!(
        empty_sides, 0,
        "a registered split now has an empty side; move the empty-side pin onto it"
    );
}

/// **The wire format is untouched**: `part_select` — a split with BOTH
/// halves read by port and unioned — saves, loads and re-saves
/// byte-identically. The corpus-wide round-trip covers the
/// same bytes; this is the per-document form beside the digest rows,
/// so a red here names the split rather than the registry.
#[test]
fn a_split_document_read_by_port_round_trips_byte_identical() {
    let doc = corpus::documents()
        .into_iter()
        .find(|d| d.name == "part_select")
        .expect("the document is registered");
    let snapshot = ProfileDoc::empty_derived("seat8_split_roundtrip", tol());
    let first = persist::save(&snapshot, &doc.edits, tol()).expect("the document saves");
    let loaded = persist::load(&first, tol()).expect("its own bytes load back");
    assert_eq!(loaded.edits, doc.edits, "the edit log did not survive");
    let second = persist::save(&loaded.snapshot, &loaded.edits, tol())
        .expect("the loaded document re-saves");
    assert_eq!(
        first, second,
        "a split document does not round-trip byte-identically"
    );
}

// The digest these rows pin with is `fixture::digest::digest` — ONE feed
// for every verb-migration suite, stated at that home; the `Split` arm
// (each side under its ROLE token, an empty side as its own token) is
// this unit's addition to it.

/// **The registered split documents' evaluations are bit-identical**
/// through the verb lowering — bodies on both sides and name tables —
/// one committed number each.
///
/// The digest feeds every node id, each body's `Debug` (its carriers'
/// stored fields and edge descriptions included) and each name table
/// (`fixture::digest`'s module docs), so a re-minted id, a renamed field
/// or a moved bit moves it. They are goldens in the ordinary sense —
/// when one moves the question is whether the new behaviour is right,
/// never how to restore the old number.
///
/// RE-BLESSED for INTENT stage 3 B (a placement is the bundle of
/// mates): a placement is a `Place` holding a world pin where it was a
/// `PlaceInWorld`, so its stored node, and with it its mint preimage,
/// moved, and every id the mint chain draws after it. No outcome or
/// point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// The numbers were taken on this branch and re-taken on the extracted
/// merge base with this file and the shared feed copied onto it; all
/// three reproduce there. That differential is what "nothing observable moved" means;
/// without it the constants would only say the branch agrees with
/// itself. They are goldens in the ordinary sense — when one moves the
/// question is whether the new behaviour is right, never how to restore
/// the old number.
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
/// RE-BLESSED, `part_select` ONLY, where JOIN-1's locus matching met
/// main's shared copy points: each moved it alone — the split halves'
/// union builds in the chord join (JOIN-1), and
/// an op's copies of one vertex share its point, so the arena order the
/// digest hashes moved (main) — and the merged tree is neither value.
///
/// RE-BLESSED, `part_select` and `kitchen_sink` only, when declaring a
/// variable began minting its id on the document's chain: every node
/// minted after a declare was renumbered, and this digest feeds ids.
/// The id-free body rows (`m4_pr8_corpus`'s exact mass pins,
/// `m5_pr8_bvh_diff`) held untouched, and every row of a document that
/// declares nothing held its word.
///
/// RE-BLESSED for INTENT-LITERALS PR C (a slot holds a variable): every
/// node is minted from slots holding variable ids, so every id moved
/// and this digest feeds ids. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held untouched.
///
/// RE-BLESSED, `cut_cylinder` only, when a chart image's flag became
/// `wrap` (the wrap edge, D1): the digest feeds each curve's `Debug`,
/// whose field name moved; with `wrap: ` read back as `seam: ` the feed
/// reproduces every old constant, so no evaluation moved.
///
/// RE-BLESSED for INTENT-LITERALS PR D (`Expr` holds no float):
/// `kitchen_sink` alone, whose formulas hold written quantities that
/// now mint variables of their own, so its ids moved. No outcome or
/// point moved (the id-free fence held).
///
/// RE-BLESSED, `cut_cylinder` only, when a swept point's description began
/// carrying a `geom_brep::SweepRange` (`range`) beside its angle or
/// vector, and a restricted one kept its placement instead of composing
/// the split's motion into it: the digest feeds each curve's `Debug`,
/// and these are the documents whose bodies store a swept-point
/// description, split or whole.
///
/// RE-BLESSED for INTENT stage 2 PR C (the product is the world):
/// each document now places its bodies, and every placement is a node
/// with a value and a name table of its own, so the evaluation this
/// digest walks holds those copies. What each document delivers did
/// not move: `intent_s2_c_world`'s migration check holds each product
/// to its pre-C digest.
///
/// RE-BLESSED, `cut_cylinder` only, when restriction
/// moved onto the description as a whole (`MappedCurve { source,
/// range }`): every curve's `Debug` now nests its source under
/// `source` beside one `range`, and these are the documents whose
/// bodies store a sketch pushforward. No point moved (`m10_p_fence`'s
/// f64 and Interval rows held), and no name table did.
///
/// RE-BLESSED, all three, for INTENT stage 4 E (booleans glue on Zero):
/// a body no longer carries provenance side tables (`GeomSource`
/// stamps, field and axis sources), and the digest feeds each body's
/// `Debug`. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// RE-BLESSED, `kitchen_sink` only, when a revolve's turned vertex came
/// to turn its offset from the axis: 8 of its points moved by at most 2
/// ulps (`m10_p_fence`'s header has the dump), and no name table did.
///
/// RE-BLESSED when an intrinsic edge description's two surfaces became a
/// set (`geom_brep::SurfacePair`), re-taken on the tree holding every
/// change above: the digest feeds each edge's description `Debug`,
/// which spells `pair: {a, b}` in key order where it spelled `s1`, `s2`
/// in the builder's order. No body moved: `m4_pr8_corpus`'s exact mass
/// pins and `m5_pr8_bvh_diff` held.
///
/// RE-BLESSED, `cut_cylinder` only, when a split's section face came to
/// take its chart axis as its first chord projected into the plane: the
/// tilted section's `u_ref` moved by one ulp in `z`, and nothing else did.
///
/// RE-BLESSED, `part_select` only, when its union came to read the
/// split's two ports where it read a `Part` of each half: the two
/// projections are gone from the evaluation this digest walks, and the
/// union keys its members by the ports. `m4_pr8_corpus`'s exact mass
/// pins held.
///
/// RE-BLESSED, `kitchen_sink` only, for INTENT stage 3 A (a revolve's
/// axis is a line on the node): its in-plane axis datum left, so every
/// later node id re-mints and the name table with it. No outcome or
/// point moved: the id-free fence with the pointless datum outcomes
/// dropped is equal on the base tree and this one. Re-taken merged over
/// FORK-DM4 unit 1 (PR 4527) and the split-half retirement (PR 4618).
///
/// RE-BLESSED, `part_select` only, where the at-rest records' cell-pair
/// shape met the split's ports: each moved it alone, and the merged
/// tree is neither value. `m4_pr8_corpus`'s exact mass pins,
/// `m5_pr8_bvh_diff` and `m10_p_fence`'s id-masked geometry held.
///
/// RE-BLESSED, `kitchen_sink`, beside `part_select` above, merging main's INTENT stage 3 A
/// re-pin with the at-rest records' cell-pair shape: each moved it
/// alone, and the merged tree is neither value. `m4_pr8_corpus`'s exact
/// mass pins, `m5_pr8_bvh_diff` and `m10_p_fence`'s id-masked geometry
/// held.
///
/// RE-BLESSED, all three, merged over the two re-pins above, when a name came to be scoped by the variable
/// holding its body (FORK-DM4 unit 2): a split's halves are two tables
/// and its names spell no half, which is what this digest's name table
/// feed reads; `part_select`'s union, declared by the half-free section
/// face, took a new id too. No body moved: the id-masked geometry fence
/// held.
///
/// Re-taken merged over main at `8612ce48e8` (main's split naming,
/// killed lineage and vertex runs, over this unit's per-half tables):
/// `cut_cylinder`'s merged digest is neither tree's, and `part_select`'s
/// and `kitchen_sink`'s are this unit's.
///
/// Re-taken merged over INTENT stage 3 B (#5232, a placement is the
/// bundle of mates), which re-blessed these rows by its new mint chain:
/// `cut_cylinder`, `part_select`, `kitchen_sink` moved by this unit as well, and
/// their merged values are neither tree's; every other row is main's.
///
/// RE-BLESSED, every row, for INTENT stage 3 C (a mate relates two
/// poses): each document places its copy where it stands by a loose
/// `Place` and the named interim's world pin, where one edit minted
/// the placement holding its world pin, so the placement's id and every
/// id drawn after it moved. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held.
///
/// RE-BLESSED, `cut_cylinder` only, when a harmonic row's windings
/// became stored structure (PCERT B2a, PR 5244): the stored pcurve is
/// two channels where it was one flat tuple, and the digest hashes it.
/// RE-BLESSED, `cut_cylinder` only, when each split half came to end
/// with maximal faces and to re-describe the boundary of a curved run
/// its merge records as skipped (PR 5334): the tilted cut leaves both
/// half-walls' pieces in each half, and the re-description's pass over
/// their boundary edges re-stores those edges' stored rows, which this
/// digest hashes. Every edge description's `Debug` is unchanged, and no
/// point moved (`m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held).
///
/// RE-BLESSED, every row, for INTENT stage 3 D (Transform retires into
/// a placement): each document was re-spelled without `Datum` or
/// `Transform` (a construction is built in its own coordinates and
/// placed into the world by a `Place`), so its nodes, their ids and the
/// names they mint moved.
///
/// RE-BLESSED, `cut_cylinder` and `kitchen_sink`, when a placement whose
/// bundle reads a declared or toleranced variable came to run its map
/// at the nominal (C-5 narrowed, the orchestrator's ruling): each
/// document's such placement places its copy by its solved map where it
/// was snapped to no map. Measured with the narrowing switched off and
/// on: off, both digests are the ones pinned before; on, no outcome and
/// no point of the corpus moves at `f64` (`m10_p_fence`'s header), so
/// what moved is the placed body's `Debug` past its points.
/// `part_select` held.
///
/// RE-BLESSED merging main into D: `cut_cylinder`, which main's PCERT
/// B2a and PR 5334 re-blessed above, takes both trees' moves, so its
/// value is neither tree's; `kitchen_sink` with its lone block's copies
/// hung below the world's xy as its retired `SetExtrudeSide` edit had
/// them. `part_select` held.
///
/// RE-BLESSED, every row, for the identity bundle (FORK-REST): a copy
/// standing where its body stands is placed by no mate and the constant
/// `0` on every freedom, written as one `AddValues`, which mints a
/// different number of ids from the plane mate and three values it replaces. So every
/// node minted after a stand moved its id, and this digest feeds ids.
/// Where C-5's narrowing ran a standing copy's map, it is placed by no
/// map again. Every standing copy in the corpus is bit-identical to its
/// source (measured with a lane probe over `body_digest`).
///
/// RE-BLESSED, `kitchen_sink` only, for exact turns (INTENT stage 3 D):
/// a turn written as a fraction of a turn takes its sine and cosine from
/// the fraction, so its revolve's heading `turn/4` puts the axis on the
/// profile's `y` exactly; its revolve stands upright by a bundle with no
/// mate spun `turn/4` (`fixture::upright_at`) where the retired place-at helper wrote an
/// `Axis` mate onto a world read spun by turns read off by `atan2`; and
/// its split's plane and its circular pattern's axis are read off its
/// faces (`fixture::plane_on`, `axis_on`) where they were frames moved by
/// rounded turns. On the merged tree with the exact heading alone it
/// read `0xcd7f_f823_b396_33fc`. The other rows held across main's merge
/// at `557e581a64` and this lane.
///
/// Re-measured merging main at `be82e5ad6b` into D, `cut_cylinder`,
/// `part_select` and `kitchen_sink`: main's N3 names each seam side and
/// crossing line with the read it came in through, and in D a later
/// operand is a placed copy, so the names it spells moved under both
/// changes and the value is neither tree's. No other row moved.
///
/// RE-BLESSED, every row, for a minted reference that comes from the
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
/// the corpus's `⊔` nodes set aside, is main's pin. `cut_cylinder` and
/// `kitchen_sink` were measured again with main's plane pose merged in
/// (main, `34a2e26e61`, which reds both rows on its own), and carry both
/// changes.
///
/// RE-BLESSED, every row, for a certificate that keeps its turn: the
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
/// RE-BLESSED, `kitchen_sink` only, for INTENT stage 3 E: the digest
/// feeds each body where it stands, composed through `in_space`, and
/// every other row held main's value. The kitchen sink's turned copy
/// (node 171) composes one map where it was rebuilt by two, and one of
/// its planes' normals reads `-0.0` where it read `0.0` in x, and so
/// does the product `⊔` (node 284), which reads that copy through its
/// map; every point, curve and face holds main's bits (measured over
/// main's merge at `1f851964`). Re-taken over main's merge at
/// `cd5f9dee`, whose word is `0x2cab_0c2d_cbd8_4ea8`: with each copy
/// rebuilt at `place` as before E, this branch reads main's word.
///
/// RE-BLESSED when `Intersection`'s witness became its selector
/// (`selector: Witness(p)` where it spelled `witness: p`), which changes
/// the edge curve's `Debug` the digest feeds. With that `Debug` written in
/// main's shape every digest here reproduces main's value at all three
/// ε rows, so nothing else moved.
#[test]
fn the_split_documents_evaluate_to_their_committed_digests() {
    let mut moved: Vec<String> = Vec::new();
    for (name, want) in [
        ("cut_cylinder", 0x863c_8c78_7943_b812_u64),
        ("part_select", 0xbca1_dc45_d456_c7a7),
        ("kitchen_sink", 0xd8b7_373c_ad55_7d3c),
    ] {
        assert!(SPLIT_DOCUMENTS.contains(&name));
        let doc = corpus::documents()
            .into_iter()
            .find(|d| d.name == name)
            .expect("the document is registered");
        let ev = eval::<f64>(&doc.doc);
        let bad = failures(&ev);
        assert!(bad.is_empty(), "{name} failed to evaluate: {bad:?}");
        let split = ev
            .order
            .iter()
            .copied()
            .find(|id| {
                matches!(
                    ev.value(*id).map(|v| &v.payload),
                    Some(ValuePayload::Split { .. })
                )
            })
            .expect("the document carries a split value");
        let Some(ValuePayload::Split { above, below }) = ev.value(split).map(|v| &v.payload) else {
            unreachable!("found above");
        };
        assert!(
            matches!((above, below), (SplitSide::Body(_), SplitSide::Body(_))),
            "{name}'s plane no longer cuts through its operand"
        );
        let got = digest(&ev);
        println!("seat8 {name}: {got:#018x}");
        if got != want {
            moved.push(format!("{name}: {got:#018x} (want {want:#018x})"));
        }
    }
    assert!(
        moved.is_empty(),
        "these documents' evaluations moved — a side or the name table:\n{}",
        moved.join("\n")
    );
}

/// A unit cube on its own xy, the plane at height `z` with `+z` normal
/// read off its geometry, and the split of the one by the other — returns the
/// recorder and the split node.
fn cube_split_at(z: f64) -> (Recorder, RecipeNodeId) {
    let mut r = Recorder::new();
    let profile = r.profile(
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.5, 0.5, 0.5)],
    );
    let cube = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(1.0),
    });
    let tool = crate::fixture::plane_on(&r.doc, cube, [0.0, 0.0, z], [0.0, 0.0, 1.0]);
    let split = r.insert(Node::Split {
        target: cube.into(),
        tool,
    });
    (r, split)
}

/// **The empty side is pinned by an input that PRODUCES it** — a plane
/// clear of the cube, authored in-suite because no registered document
/// has one and the empty path needs none of the corpus's other rows
/// (no mass to pin on a side with no material).
///
/// The value is asserted to actually BE `Empty` above and a body below
/// before the constant is checked, so the row cannot go vacuous if the
/// fixture drifts. With that, perturbing the digest's empty-side token
/// reds THIS row while the three document constants stand, and a
/// lowering that turned the typed empty into anything else — or an
/// empty side into a phantom body — moves this number.
///
/// empty side into a phantom body — moves this number. The constant
/// reproduces on the extracted merge base (the empty path predates the
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
/// RE-BLESSED when a name came to be scoped by the variable holding its
/// body (FORK-DM4 unit 2): the split's one table became a table per
/// half, the empty half's empty, and its names spell no half.
///
/// RE-BLESSED for INTENT stage 3 D (Transform retires into a
/// placement): the split's tool is a plane read off the cube's own
/// geometry (`fixture::plane_on`) where it was a `Datum::Plane`, and the
/// profile carries no frame, so its nodes, their ids and the names they
/// mint moved.
///
/// RE-BLESSED for exact turns (INTENT stage 3 D): the tool is the
/// cube's end cap's plane slid along its normal (`fixture::plane_on`
/// reads a face with the stated normal) where it was a frame read off
/// the cube's edges moved by turns read off by `atan2`, so its pose's
/// nodes and the names the split mints moved.
///
/// RE-BLESSED when a minted plane's `u_ref` became a direction its
/// construction holds: the cube's caps are charted along its sketch's
/// `x` axis and its walls along their swept segments, where
/// `orthonormal_basis` picked both off the axes. No plane moved as a
/// point set.
///
/// RE-BLESSED for D′1 (a placement names its target space): each
/// construction defines its space beside its body, and the content key
/// holds a placement's target (`VERSION` 18), so ids drawn after a
/// construction and every value token moved. No point moved.
///
/// RE-BLESSED for a certificate that keeps its turn: the digest feeds
/// each edge curve's `Debug`, which now spells the certificate's `turn`.
/// No outcome or point moved.
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
fn a_split_with_an_empty_side_evaluates_to_its_committed_digest() {
    let (r, split) = cube_split_at(5.0);
    let ev = eval::<f64>(&r.doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "the fixture evaluates: {bad:?}");
    let Some(ValuePayload::Split { above, below }) = ev.value(split).map(|v| &v.payload) else {
        panic!("the split node has no split value");
    };
    assert!(
        matches!(above, SplitSide::Empty),
        "a plane above the cube leaves nothing above it; the fixture no longer produces the empty side"
    );
    assert!(
        matches!(below, SplitSide::Body(_)),
        "everything is below the plane"
    );
    let got = digest(&ev);
    println!("seat8 empty_side: {got:#018x}");
    assert_eq!(
        got, 0x883a_4673_251b_8566,
        "the empty-side evaluation moved — side token, body or name table"
    );
}

/// The typed root cause of one node, or a loud failure.
fn error_of(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> &NodeErrorKind {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(e)) => &e.kind,
        other => panic!("node {id:?} did not fail typed: {other:?}"),
    }
}

/// **What a read of a half needs from the two-sided value, pinned where
/// it reads it.** Each port is one half by ROLE — above or below the
/// plane's normal (`SplitHalf::port`) — a read of an empty half refuses
/// typed (`EmptyHalf`, naming the split and the side), and a present
/// half is read as the split's own side. Those three are the contract
/// the split's out-type carries two role-tagged `Body | Empty` sides
/// for, and the reason it is not a list of bodies or a body with a side
/// marker.
#[test]
fn a_read_of_a_half_reads_the_two_sided_value_by_role() {
    let (mut r, split) = cube_split_at(5.0);
    // A copy of each half, loose: a reader of the half's value.
    let still = |half: SplitHalf| {
        crate::fixture::stood(editor_core::BodyRead::plain(editor_core::Operand::output(
            split,
            half.port(),
        )))
    };
    let above = r.insert(still(SplitHalf::Above));
    let below = r.insert(still(SplitHalf::Below));
    let ev = eval::<f64>(&r.doc);
    assert!(
        matches!(
            error_of(&ev, above),
            NodeErrorKind::EmptyHalf { input, half: SplitHalf::Above } if *input == split
        ),
        "the empty half must refuse typed, naming the split and the side: {:?}",
        error_of(&ev, above)
    );
    let Some(ValuePayload::Split {
        below: SplitSide::Body(side),
        ..
    }) = ev.value(split).map(|v| &v.payload)
    else {
        panic!("the split's below side is a body");
    };
    let read = match ev.value(below).map(|v| &v.payload) {
        Some(ValuePayload::Body(body)) => body,
        Some(ValuePayload::Instances(copies)) => {
            let [read] = &copies[..] else {
                panic!("one copy of the half: {}", copies.len());
            };
            read
        }
        _ => panic!("the present half is read: {:?}", ev.result(below).is_some()),
    };
    assert_eq!(
        (
            read.body().faces().count(),
            read.body().edges().count(),
            read.body().vertices().count()
        ),
        (
            side.body().faces().count(),
            side.body().edges().count(),
            side.body().vertices().count()
        ),
        "the reader holds the split's own side"
    );
}
