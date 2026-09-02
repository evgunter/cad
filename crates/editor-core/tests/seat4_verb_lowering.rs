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

mod corpus;
mod fixture;

use editor_core::{
    DocEdit, LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId, StableName, persist,
};
use fixture::{len, prism_edges};
use geom_core::Tol;
use profile::SketchPlane;

fn tol() -> Tol {
    Tol::witness()
}

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
        plane: SketchPlane::xy(),
        loops: vec![square],
    }));
    let cube = r.insert(Node::Extrude {
        profile,
        distance: len(L),
    });
    let edges: Vec<StableName> = prism_edges(cube, 4);
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
    edits: Vec<DocEdit<ProfileProgram>>,
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
        let value = ev.value(id).expect("the blend node produced a value");
        assert!(
            value.name_table.iter().count() > 0,
            "node {id:?} produced an empty name table"
        );
    }
    // The two branches are different geometry under different node ids,
    // so their tables share no name: this is what "the discrimination
    // is the minting id" means, executed.
    let names = |id: RecipeNodeId| -> std::collections::BTreeSet<String> {
        ev.value(id)
            .unwrap()
            .name_table
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

/// FNV-1a 64 over a document's evaluated name tables and values —
/// **every channel the migrated lowerings write**, in one number per
/// document.
///
/// # What this covers, and how that set was chosen
///
/// Not "everything observable": the channels THESE lowerings can move,
/// enumerated off the lowerings' own bodies. `wire_blend` writes
/// exactly four things — the name table the emitter returns, the body
/// the kernel verb returns, the provenance stamp `stamp_minted`
/// applies to that body, and (on the refusal path) a typed error.
/// `wire_boolean` writes the same first three plus the boolean VALUE's
/// other halves — the result classification, the surviving declared
/// contacts, and the typed empty success — and all of those are here
/// too (the `Boolean` payload arm). The refusal path is NOT, and that
/// is a real hole, stated: a refusal payload's spelling and the
/// verdict logs are outside this digest, so a change that only altered
/// which `NodeErrorKind` came back would pass it — the boolean's
/// undeclared-contact menu lift included.
/// `both_blends_evaluate_in_one_document` covers only that the success
/// path stays a success.
///
/// # Why each half is load-bearing, measured rather than assumed
///
/// - **Point bits alone are not enough.** A unit cube filleted at
///   radius `r` and a unit cube chamfered at setback `r` have the same
///   twenty-four vertex positions to the bit, differing only in whether
///   the faces between them are cylinders and spheres or planes. A
///   points-only digest gave the two documents ONE identical number and
///   could not tell this unit's two verbs apart at all.
/// - **Carriers alone are not enough either.** Face carriers were the
///   first fix, and `stamp_minted` — the line that gives every surface,
///   curve and point this blend mints its `GeomSource`, and the thing
///   that makes a downstream reference into a blended body resolvable —
///   could be DELETED from `wire_blend` with this digest and the whole
///   891-row editor-core suite still green. So the three provenance
///   source tables are fed here, and so are the edge curve carriers
///   (faces reach surfaces; nothing reached the curve arena).
///
/// Deleting `stamp_minted` now fails this row. That is the red-first
/// evidence for the sentence above, and it is why the constants below
/// are this PR's own mint rather than the pre-change tree's.
///
/// Geometry and provenance enter through `Debug`, whose `f64` rendering
/// is the shortest round-tripping decimal: a bijection with the bits
/// for every finite value, `-0.0` included.
fn digest(ev: &editor_core::Evaluation<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |bytes: &[u8]| {
        for b in bytes {
            h ^= u64::from(*b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for id in &ev.order {
        feed(format!("#{id:?}").as_bytes());
        let Some(value) = ev.value(*id) else { continue };
        for (name, entry) in value.name_table.iter() {
            feed(format!("{name:?}={entry:?}").as_bytes());
        }
        feed(value.payload.kind_name().as_bytes());
        if let editor_core::ValuePayload::Body(body) = &value.payload {
            feed_body(&mut feed, body);
        } else if let editor_core::ValuePayload::Boolean(bv) = &value.payload {
            // The boolean value's three halves — `wire_boolean` writes
            // all of them: the stamped body, the result
            // classification, and the surviving declared contacts (the
            // tier-3′ currency downstream ops re-enter). A lowering
            // that dropped the contact carry or misfiled the kind
            // would move nothing in any body arena and EVERYTHING
            // here. The typed empty is fed as its own token so a
            // result that vanished cannot alias one that never ran.
            match bv {
                editor_core::BooleanValue::Body {
                    body,
                    kind,
                    contacts,
                } => {
                    feed(format!("{kind:?}{contacts:?}").as_bytes());
                    feed_body(&mut feed, body);
                }
                editor_core::BooleanValue::Empty => feed(b"empty"),
            }
        }
    }
    h
}

/// The body half of [`digest`], byte-for-byte the SEAT-4 feed: points
/// with their provenance stamps, the curve and surface arenas with
/// theirs, the topology's attachment both ways, and the entity census.
fn feed_body(feed: &mut impl FnMut(&[u8]), body: &topo::Body<f64>) {
    // Points: bits, then the provenance stamp on the same key.
    for (key, p) in body.points() {
        for c in [p.x, p.y, p.z] {
            feed(&c.to_bits().to_be_bytes());
        }
        feed(format!("{key:?}<-{:?}", body.point_source(key)).as_bytes());
    }
    // Curves and surfaces: the arenas themselves plus their
    // stamps. A face reaches its surface below, but nothing
    // reached the curve arena before this, and neither arena's
    // SOURCE was reachable at all.
    for (key, curve) in body.curves() {
        feed(format!("{key:?}{curve:?}<-{:?}", body.curve_source(key)).as_bytes());
    }
    for (key, surface) in body.surfaces() {
        feed(format!("{key:?}{surface:?}<-{:?}", body.surface_source(key)).as_bytes());
    }
    // The topology's attachment to that geometry, both ways: a
    // face's carrier and an edge's curve. A re-plumbing that
    // kept every arena and re-pointed the topology at it moves
    // these and nothing above.
    for (key, face) in body.faces() {
        let surface = body
            .get_surface(face.surface)
            .expect("a face has a carrier");
        feed(format!("{key:?}{surface:?}").as_bytes());
    }
    for (key, edge) in body.edges() {
        let curve = body
            .get_curve_geom(edge.curve)
            .expect("an edge has a curve");
        feed(format!("{key:?}{curve:?}").as_bytes());
    }
    feed(
        format!(
            "V{}E{}F{}",
            body.vertices().count(),
            body.edges().count(),
            body.faces().count()
        )
        .as_bytes(),
    );
}

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
#[test]
fn the_blend_documents_evaluate_to_their_committed_digests() {
    for (name, want) in [
        ("die_fillet", 0xc88b_608e_e0eb_be22_u64),
        ("die_chamfer", 0x0d1a_ec94_58b0_afd6),
    ] {
        let doc = corpus::documents()
            .into_iter()
            .find(|d| d.name == name)
            .expect("the document is registered");
        let ev = corpus::eval::<f64>(&doc.doc);
        let got = digest(&ev);
        println!("seat4 {name}: {got:#018x}");
        assert_eq!(got, want, "{name}'s evaluation moved — body or name table");
    }
}

/// **A registered boolean document's bytes survive the migration**:
/// save → load → save reproduces the file exactly. The document is
/// `crossing_slots` — two subtracts, one carrying a `Declare` operand —
/// so the wire spelling under pin includes the boolean node's whole
/// payload: the op, both operand edges and the declare edge. The
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
/// `crossing_slots` runs subtract twice, once with a DECLARED rest
/// contact (so `resolve_declarations`' face-pair arm exercises under
/// pin) and once undeclared; `heat_sink` runs the union chain;
/// `kiss_carry` is the one whose boolean values carry NON-EMPTY
/// surviving contacts (a discovered corner kiss, then the same record
/// re-entered through `resolve_declarations`' carried-v-v arm). The
/// digest feeds the boolean value's kind and contacts beside the
/// stamped body and the name table, so the constants cover exactly
/// what `wire_boolean` writes.
///
/// `kiss_carry` is load-bearing for the contacts half, MEASURED: on
/// the other two documents alone, replacing the lowering's contact
/// carry with `ContactRecords::default()` leaves every constant
/// standing (their surviving records are empty — declared REST
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
#[test]
fn the_boolean_documents_evaluate_to_their_committed_digests() {
    for (name, want) in [
        ("crossing_slots", 0x7865_325e_8719_d6a0_u64),
        ("heat_sink", 0x4c79_8719_cbc2_5c5a),
        ("kiss_carry", 0x6dd7_1fcd_ed94_9fff),
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
        assert_eq!(
            got, want,
            "{name}'s evaluation moved — body, value or name table"
        );
    }
}
