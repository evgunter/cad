//! **The census records the mated seats** (ASSEMBLY-DESIGN A5, D10).
//!
//! A mate places a copy and declares nothing. The product gather's
//! at-rest census records every touch between copies, mated or not, as
//! a row and a record citing it (`topo::record_at_rest`), so every
//! evaluated product carries its seats' records. The at-rest check
//! reads that census's verdict and records nothing of its own.
//!
//! What that buys is the instantiation seam: a sub-assembly's records
//! ride into the consuming document under the graft's own descendant
//! map, exactly as its boolean-discovered records do, and the outer
//! census confirms them again.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Dimension;
use editor_core::Formula;

use editor_core::coincide::{NamedCell, NamedCoincidence, Proof, Rung};
use editor_core::{
    CapEnd, Coord, DocRef, DocumentId, EntityKind, Frame, MateRef, Node, ProfileDoc, RecipeNodeId,
    RoleSeg, StableName, TargetAxis, product_recorded,
};
use editor_core::{Evaluation, Unreplayed};
use fixture::resolver::{PartName, PartStore, in_part, with_resolver};
use fixture::{insert, len, mated_instance, on_frame, run};
use geom_core::Tol;

// ---- Evaluation through the shared part store ----

// ---- Documents ----

/// An axis-aligned block: `[x]×[y]` on the plane at `z0`, extruded
/// `dz`. Its extrude node is the returned id.
fn block(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
        },
    )
}

fn cube_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    block(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    )
}

/// A `Rest` mate declaring `a`'s TOP face against `b`'s BOTTOM face,
/// standing `b` `apart` beside `a` ([`Seat`]). `apart = 0.0` puts `b`'s
/// bottom square on `a`'s top (the unit cube is z ∈ [0,1]); `2.0`
/// leaves the two faces coplanar with a unit of clear air between them.
fn rest_mate(a: impl crate::fixture::Sited, b: impl crate::fixture::Sited, apart: f64) -> Seat {
    Seat {
        mate: fixture::rest(fixture::head(b), fixture::head(a)),
        apart,
        across: [TargetAxis::X, TargetAxis::Y],
    }
}

/// **A rest and how far beside its target it stands**: the copy's
/// own origin slid `apart` along x from the target's in the rested
/// plane. `0.0` stands the two unit cubes square, face to face; `2.0`
/// leaves the faces coplanar with a unit of clear air between their
/// trims, so nothing touches.
struct Seat {
    mate: editor_core::Mate<Formula>,
    apart: f64,
    /// The two slides the mate leaves, across its mated direction: the
    /// first takes `apart`.
    across: [TargetAxis; 2],
}

/// `seat`'s mate added to `placement`'s bundle with the three freedoms
/// a plane mate leaves set: a slide of `apart` along x, none along y,
/// and no spin, so it pins the copy.
fn seated_at(doc: ProfileDoc, placement: RecipeNodeId, seat: Seat) -> (ProfileDoc, MateRef) {
    let (doc, m) = crate::fixture::mated(
        doc,
        placement,
        seat.mate,
        &[
            (Coord::Slide(seat.across[0]), len(seat.apart)),
            (
                Coord::Slide(seat.across[1]),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    (doc, m)
}

/// [`seated_at`] on the placement the seat's `on` reads.
fn seated(doc: ProfileDoc, seat: Seat) -> (ProfileDoc, MateRef) {
    let placement = fixture::placed_by(&doc, &seat.mate);
    seated_at(doc, placement, seat)
}

/// **Issue 946's inner document**: a stand of two cubes seated one on
/// the other by its mate. Its own `assemble` is green at `apart = 0.0`;
/// at `2.0` the upper cube stands beside the lower one ([`Seat`]).
/// `body` is `part`'s body.
///
/// Returns the document, its two instance ids (the lower pinned to the
/// world, the upper placed by the mate), and its mate's address.
fn stand(
    label: &str,
    part: DocRef,
    body: RecipeNodeId,
    apart: f64,
    store: &PartStore,
) -> (ProfileDoc, Vec<RecipeNodeId>, MateRef) {
    let (doc, ids) = seated_row(label, part, 2, store);
    let (doc, mate) = seated(
        doc,
        rest_mate(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[1], body, CapEnd::Start),
            apart,
        ),
    );
    (doc, ids, mate)
}

/// `count` instances of `part`: the first pinned to the world, the rest
/// standing in the world by the identity bundle until a mate places it.
fn seated_row(
    label: &str,
    part: DocRef,
    count: usize,
    store: &PartStore,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (mut doc, first) = fixture::insert_pinned(
        doc,
        crate::fixture::mated_instance(part),
        &with_resolver(store.clone()),
    );
    let mut ids = vec![first];
    for _ in 1..count {
        let (next, id) = insert(doc, mated_instance(part));
        doc = next;
        ids.push(id);
    }
    (crate::fixture::assemble_world_copies(doc), ids)
}

/// `count` instances of `part`, each pinned `spacing * i` along +x so
/// the copies stay disjoint (ASM-2B row 4's shape).
fn row_of(
    label: &str,
    part: DocRef,
    count: usize,
    spacing: f64,
    store: &PartStore,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let o = with_resolver(store.clone());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..count {
        let (next, id) = insert(doc, mated_instance(part));
        #[allow(clippy::cast_precision_loss)]
        let dx = spacing * i as f64;
        doc = fixture::pin(next, id, &Frame::translation([dx, 0.0, 0.0]), &o);
        ids.push(id);
    }
    (crate::fixture::assemble_world_copies(doc), ids)
}

/// Every at-rest finding over `doc`, rendered.
fn findings(doc: &ProfileDoc, ev: &Evaluation<f64>) -> Vec<String> {
    crate::fixture::at_rest(doc, ev)
        .unwrap_or_else(|e| panic!("expected the product to gather, got {e}"))
        .iter()
        .map(|f| format!("{:?}", f.evidence))
        .collect()
}

// ---- The seam ----

/// INVARIANT (**issue 946, the row it is about**): an inner stand,
/// instantiated ×3, is valid in the outer document — because the inner
/// gather's census RECORDED its seat and the seam carried the record
/// under the graft's descendant map, exactly as it carries
/// boolean-discovered records. Without the record the inner seat is a
/// touch inside one copy of the outer product, which no record backs
/// (D1 (iii)).
#[test]
fn three_identical_stands_in_a_row_carry_their_inner_declarations() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-cube"), Tol::witness());
    let (inner, _, _) = stand("mate6-stand", part, body, 0.0, &store);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, ids) = row_of("mate6-row", inner_ref, 3, 4.0, &store);

    let ev = run(&outer, &with_resolver(store));
    let gathered = product_recorded(&outer, &ev, Tol::witness()).expect("the row gathers");
    assert_eq!(
        gathered.body.solids().count(),
        6,
        "three stands of two cubes each"
    );
    assert_eq!(
        gathered.contacts.face_face().count(),
        3,
        "one carried record per stand — the inner census's, re-keyed onto \
         the aggregate"
    );

    let raised = findings(&outer, &ev);
    assert!(
        raised.is_empty(),
        "the outer at-rest check reports nothing of a row of correctly-mated stands: {raised:?}"
    );
    assert!(!ids.is_empty());
}

/// INVARIANT: the carry is not one level deep. Outer–mid–inner: the
/// cube's seat is recorded at the innermost document and survives two
/// grafts, because each seam carries what its level's gather recorded.
#[test]
fn the_carry_survives_a_second_nesting_level() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-deep-cube"), Tol::witness());
    let (inner, _, _) = stand("mate6-deep-stand", part, body, 0.0, &store);
    let inner_ref = store.insert(inner, Tol::witness());
    // The MID document is a pair of stands, side by side and disjoint.
    let (mid, _) = row_of("mate6-deep-mid", inner_ref, 2, 4.0, &store);
    let mid_ref = store.insert(mid, Tol::witness());
    let (outer, _) = row_of("mate6-deep-outer", mid_ref, 2, 12.0, &store);

    let ev = run(&outer, &with_resolver(store));
    let gathered = product_recorded(&outer, &ev, Tol::witness()).expect("the deep row gathers");
    assert_eq!(gathered.body.solids().count(), 8, "two mids of two stands");
    assert_eq!(
        gathered.contacts.face_face().count(),
        4,
        "one carried record per stand, through two seams"
    );
    let raised = findings(&outer, &ev);
    assert!(
        raised.is_empty(),
        "and the outermost at-rest check reports nothing: {raised:?}"
    );
}

// ---- A mate states no contact ----

/// INVARIANT: a mate places and declares nothing. A stand whose mate
/// leaves the upper cube a unit of clear air beside the lower one has no
/// touch for the census to record, and its check reports nothing: no
/// contact is asserted that the geometry would have to refute.
#[test]
fn a_mate_that_leaves_clear_air_records_nothing_and_refuses_nothing() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-gap-cube"), Tol::witness());
    let (doc, _, _) = stand("mate6-gap-stand", part, body, 2.0, &store);
    let ev = run(&doc, &with_resolver(store));
    let gathered = product_recorded(&doc, &ev, Tol::witness()).expect("the stand gathers");
    assert_eq!(
        gathered.contacts.len(),
        0,
        "no touch, no record: {:?}",
        gathered.contacts
    );
    let raised = findings(&doc, &ev);
    assert!(raised.is_empty(), "nothing to refute: {raised:?}");
}

/// INVARIANT: a document with no mates is untouched by any of this —
/// the gather's record set is exactly what its sources carried up.
#[test]
fn a_document_with_no_mates_gathers_exactly_what_it_did_before() {
    let mut store = PartStore::default();
    let (part, _) = store.insert_part(cube_part("mate6-bare-cube"), Tol::witness());
    let (doc, _) = row_of("mate6-bare-row", part, 3, 4.0, &store);

    let ev = run(&doc, &with_resolver(store));
    let gathered = product_recorded(&doc, &ev, Tol::witness()).expect("the bare row gathers");
    assert_eq!(gathered.body.solids().count(), 3);
    assert_eq!(
        gathered.contacts,
        topo::ContactRecords::default(),
        "no touch, no records"
    );
}

/// INVARIANT: the census records each mated seat between copies as one
/// face–face record and nothing else.
#[test]
fn the_census_records_each_mated_seat_as_one_face_pair() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-shape-cube"), Tol::witness());
    let (doc, ids) = seated_row("mate6-shape-row", part, 3, &store);
    let (doc, _) = seated(
        doc,
        rest_mate(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[1], body, CapEnd::Start),
            0.0,
        ),
    );
    let (doc, _) = seated(
        doc,
        rest_mate(
            in_part(ids[1], body, CapEnd::End),
            in_part(ids[2], body, CapEnd::Start),
            0.0,
        ),
    );

    let ev = run(&doc, &with_resolver(store));
    let gathered = product_recorded(&doc, &ev, Tol::witness()).expect("the gather stands");
    assert_eq!(
        gathered.contacts.face_face().count(),
        2,
        "one face–face record per seat: {:?}",
        gathered.contacts
    );
    assert_eq!(gathered.contacts.len(), 2, "{:?}", gathered.contacts);
}

// ---- What a record cites ----

/// **A mated seat's record cites the census's row, named in its copies,
/// and the door proves it by the mate.** Two cubes of one part, the
/// second rested on the first by a plane mate and its three values: the
/// product holds one face–face record, citing its row 0, the at-rest
/// census's decision that the two seated caps rest on one carrier. The
/// row names each cap in its copy, as
/// that instance's output names it. The two caps are the faces whose
/// planes the mate equates, so the door's first rung proves the row
/// (`[coincidence.structural]`). The two cubes stand flush on all four
/// sides, and each side is a flush pair the census records as a row of
/// its own (`[assertions]`); the mate equates no side's plane, so
/// `unproven-coincidence` reports those four and nothing else.
///
/// Red if the census makes a record citing nothing the product holds,
/// names a cap anywhere but in its copy, or the door leaves the mate's
/// own seat unproven.
#[test]
fn a_mated_record_cites_the_census_row_the_lint_reports() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-cites-cube"), Tol::witness());
    let (stand, ids, mate) = stand("mate6-cites-stand", part, body, 0.0, &store);
    let ev = run(&stand, &with_resolver(store));
    let product = product_recorded(&stand, &ev, Tol::witness()).expect("the stand gathers");
    assert_eq!(
        product.contacts.face_face().count(),
        1,
        "one seat, one record"
    );
    assert_eq!(
        product.contacts.rows()[0].cites,
        topo::Cites::decided(0),
        "the record cites the product's row 0"
    );
    let (seats, flush) = seat_and_flush(&product.coincidences);
    let [row] = seats[..] else {
        panic!("one seat row: {:?}", product.coincidences);
    };
    assert_eq!(flush.len(), 4, "the four flush sides: {flush:?}");
    assert_eq!(row.row.site, topo::DecisionSite::CensusAtRest);
    assert_eq!(row.row.relation, topo::Relation::SameOpposite);
    let mut inputs: Vec<editor_core::VarId> = row
        .row
        .cells
        .iter()
        .map(|cell| match cell {
            editor_core::NamedCell::Entity { input, .. } => *input,
            other => panic!("a census cell is an entity: {other:?}"),
        })
        .collect();
    inputs.sort();
    let mut copies: Vec<editor_core::VarId> = ids
        .iter()
        .map(|&id| stand.output(id, 0).expect("an instance defines its copies"))
        .collect();
    copies.sort();
    assert_eq!(inputs, copies, "each cap is named in its own copy");
    let report = editor_core::run_checks(
        &stand,
        &ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("checks run over the stand");
    let unproven: Vec<_> = report
        .findings
        .iter()
        .filter_map(|f| match &f.evidence {
            editor_core::CheckEvidence::UnprovenCoincidence { row, .. }
                if f.check == editor_core::CheckId::UnprovenCoincidence =>
            {
                Some(row.relation)
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        unproven,
        vec![topo::Relation::SameOriented; 4],
        "the mate's own seat is proven, and the four flush sides are not: {report}"
    );
    assert_eq!(
        door(&stand, mate.placement, &row.row, &ev),
        Proof::Structural(Rung::SameConstruction),
        "by the first rung, the mate's statement"
    );
}

// ---- What the door reads a seat by ----

/// A product's at-rest rows split into the seat's (two faces decided one
/// carrier, opposed) and the flush pairs' (one carrier, aligned).
fn seat_and_flush(
    rows: &[editor_core::AtRestRow],
) -> (Vec<&editor_core::AtRestRow>, Vec<&editor_core::AtRestRow>) {
    let (seats, rest): (Vec<_>, Vec<_>) = rows
        .iter()
        .partition(|r| r.row.relation == topo::Relation::SameOpposite);
    assert!(
        rest.iter()
            .all(|r| r.row.relation == topo::Relation::SameOriented),
        "every other row is a flush pair: {rest:?}"
    );
    (seats, rest)
}

/// **The door's one proof of `row`**, decided at `at` with `ev`'s
/// resolver.
fn door(doc: &ProfileDoc, at: RecipeNodeId, row: &NamedCoincidence, ev: &Evaluation<f64>) -> Proof {
    let mut proofs = editor_core::coincide::prove(
        doc,
        &[(at, std::slice::from_ref(row))],
        ev.resolver(),
        Tol::witness(),
        &editor_core::CancelToken::new(),
    )
    .expect("an uncanceled door");
    proofs.remove(0).remove(0)
}

/// What the replay made of an unproven row, `None` for a proven one.
fn replayed(proof: &Proof) -> Option<(Unreplayed, editor_core::coincide::Recourse)> {
    match proof {
        Proof::Unproven { residual, recourse } => Some((residual.replay, *recourse)),
        Proof::Structural(_) => None,
    }
}

/// **The cap `cap` of the cube in `instance`'s copy**, as the at-rest
/// census names it: in the instance's output, by the name the product's
/// `⊔` keys it with, the copy's read.
fn copy_cap(
    doc: &ProfileDoc,
    instance: RecipeNodeId,
    body: RecipeNodeId,
    cap: CapEnd,
) -> NamedCell {
    let input = doc
        .output(instance, 0)
        .expect("an instance defines its copies");
    let of = in_part(instance, body, cap).name;
    let (assembled, _) = doc
        .product()
        .and_then(|product| doc.defined_by(product))
        .expect("the stand's product is the `⊔` of its cubes");
    NamedCell::Entity {
        input,
        name: StableName {
            kind: of.kind,
            node: assembled,
            path: vec![RoleSeg::From {
                read: input,
                of: of.into(),
            }],
        },
    }
}

/// **A vertex of the cap `end` of the cube in `instance`'s copy**, as
/// the copy's table names it.
fn cap_vertex(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    instance: RecipeNodeId,
    end: CapEnd,
) -> NamedCell {
    fn minted(name: &StableName) -> &StableName {
        name
    }
    let input = doc
        .output(instance, 0)
        .expect("an instance defines its copies");
    let name = ev
        .table(input)
        .expect("the copy evaluates")
        .iter()
        .map(|(name, _)| name)
        .find(|name| {
            name.kind == EntityKind::Vertex
                && matches!(minted(name).path.as_slice(), [RoleSeg::CapVertex(e, _)] if *e == end)
        })
        .expect("the copy names its cap's vertices")
        .clone();
    NamedCell::Entity { input, name }
}

/// **An at-rest census row between `cells`**, decided `relation`, as
/// the door is handed one.
fn at_rest_row(cells: Vec<NamedCell>, relation: topo::Relation) -> NamedCoincidence {
    let margin = geom_core::MarginDiag::value(0.0);
    NamedCoincidence {
        cells,
        relation,
        site: topo::DecisionSite::CensusAtRest,
        margin,
        discharge: topo::Discharge::of(margin),
        origin: editor_core::coincide::RowOrigin::Node,
    }
}

/// [`stand`] with the stand's mate as `mate` writes it, and the plane
/// mate's three values.
fn stand_by(
    label: &str,
    part: DocRef,
    store: &PartStore,
    across: [TargetAxis; 2],
    mate: impl FnOnce(&[RecipeNodeId]) -> editor_core::Mate<Formula>,
) -> (ProfileDoc, Vec<RecipeNodeId>, MateRef) {
    let (doc, ids) = seated_row(label, part, 2, store);
    let seat = Seat {
        mate: mate(&ids),
        apart: 0.0,
        across,
    };
    let (doc, mate) = seated(doc, seat);
    (doc, ids, mate)
}

/// **The first rung reads a seat by the mate that states it, and no
/// other row by it** (`[coincidence.structural]`). Over [`stand`]'s two
/// cubes, the census's row of the mate's seat is proven by the first
/// rung. Each row below is NOT the mate's statement, so the first rung
/// leaves it and the replay reads it:
///
/// - **cells that are not the mate's two faces**: the upper cube's top
///   against the lower cube's top. The replay's carrier verdict decides
///   two planes a unit apart, so it takes another decision.
/// - **a sense the mate does not state**: the seat's two caps decided
///   `SameOriented`, against the rest's flip. The replay's verdict
///   decides them `SameOpposite`, which is not the row's relation.
/// - **a vertex of a cap the mate does not seat**: the upper cube's top
///   vertex laid on the lower cube's top. A vertex of the seated cap is
///   the mate's statement, since the extrude mints it on that cap's
///   plane; one of the other cap is not, and an at-rest row of a vertex
///   has no form the replay reads.
/// - **a mate that states the other sense**: a stand whose mate equates
///   the two caps' planes with no flip, and the census's
///   `SameOpposite` row of those caps.
/// - **a standoff**: a stand whose mate's target is the lower cap's
///   flipped plane slid along its normal by a length, here `0`. The mate
///   equates no face's plane; the gap is a variable, so the replay reads
///   the seat as a coincidence of values (`Numeric`).
///
/// A mate whose bundle does not pin is no row here: every bundle pins
/// its copy, and one that does not refuses at the door (`Unpinned`).
///
/// Red if the first rung reads any row a mate does not state as one, or
/// stops reading the mate's own seat.
#[test]
fn a_mate_states_its_own_seat_and_no_other_row() {
    use topo::Relation::{SameOpposite, SameOriented};
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-states-cube"), Tol::witness());
    let (doc, ids, mate) = stand("mate6-states-stand", part, body, 0.0, &store);
    let ev = run(&doc, &with_resolver(store.clone()));
    let product = product_recorded(&doc, &ev, Tol::witness()).expect("the stand gathers");
    let (seats, _) = seat_and_flush(&product.coincidences);
    let [seat] = seats[..] else {
        panic!("one seat row: {:?}", product.coincidences);
    };
    let (upper_bottom, lower_top) = (
        copy_cap(&doc, ids[1], body, CapEnd::Start),
        copy_cap(&doc, ids[0], body, CapEnd::End),
    );
    assert_eq!(
        seat.row.cells,
        vec![lower_top.clone(), upper_bottom.clone()],
        "the premise: the census's row is the mate's two caps, in gather order"
    );
    assert_eq!(
        door(&doc, mate.placement, &seat.row, &ev),
        Proof::Structural(Rung::SameConstruction),
        "the mate's seat"
    );
    assert_eq!(
        door(
            &doc,
            mate.placement,
            &at_rest_row(vec![upper_bottom.clone(), lower_top.clone()], SameOpposite),
            &ev
        ),
        Proof::Structural(Rung::SameConstruction),
        "the mate's seat, its cells in the other order"
    );
    let upper_top = copy_cap(&doc, ids[1], body, CapEnd::End);
    assert_eq!(
        replayed(&door(
            &doc,
            mate.placement,
            &at_rest_row(vec![upper_top, lower_top.clone()], SameOpposite),
            &ev
        )),
        Some((
            Unreplayed::OtherDecisions,
            editor_core::coincide::Recourse::Assert
        )),
        "cells that are not the mate's two faces"
    );
    assert_eq!(
        replayed(&door(
            &doc,
            mate.placement,
            &at_rest_row(vec![upper_bottom, lower_top.clone()], SameOriented),
            &ev
        )),
        Some((
            Unreplayed::OtherDecisions,
            editor_core::coincide::Recourse::Assert
        )),
        "a sense the mate does not state: the verdict decides the caps opposite"
    );
    let (upper_rim, upper_top_rim) = (
        cap_vertex(&doc, &ev, ids[1], CapEnd::Start),
        cap_vertex(&doc, &ev, ids[1], CapEnd::End),
    );
    assert_eq!(
        door(
            &doc,
            mate.placement,
            &at_rest_row(
                vec![upper_rim, lower_top.clone()],
                topo::Relation::OnCarrier
            ),
            &ev
        ),
        Proof::Structural(Rung::SameConstruction),
        "a vertex of the mate's seated cap lies on its target, which the extrude mints it on"
    );
    assert_eq!(
        replayed(&door(
            &doc,
            mate.placement,
            &at_rest_row(vec![upper_top_rim, lower_top], topo::Relation::OnCarrier),
            &ev
        )),
        Some((
            Unreplayed::NotReplayed,
            editor_core::coincide::Recourse::Assert
        )),
        "a vertex of the other cap is no cell of the mate's faces"
    );

    // A stand whose mate states the other sense, and a standoff: each
    // read against the census row of its own two caps.
    // Unflipped, the bottom cap faces the top's normal: the copy stands
    // upended, its `z` along `−z`.
    let plain = |on: PartName, to: PartName| editor_core::Mate {
        on: fixture::plane(fixture::head(on)),
        to: fixture::plane(fixture::head(to)),
        orient: editor_core::Orientation::new(
            editor_core::SignedAxis::PlusX,
            editor_core::SignedAxis::MinusZ,
        )
        .expect("x and z lie along two axes"),
    };
    let standoff = |on: PartName, to: PartName| {
        use editor_core::pose::{PoseDef, PoseRole, RoleRead};
        let up = || fixture::flip(fixture::plane(fixture::head(to.clone())));
        editor_core::Mate {
            on: fixture::plane(fixture::head(on)),
            to: editor_core::Operand::Pose(Box::new(PoseDef::Slide {
                pose: RoleRead::plain(up()),
                along: RoleRead::through(up(), PoseRole::Normal),
                by: len(0.0),
            })),
            orient: editor_core::Orientation::IDENTITY,
        }
    };
    type Writes<'a> = &'a dyn Fn(PartName, PartName) -> editor_core::Mate<Formula>;
    let cases: [(&str, Writes<'_>, Unreplayed); 2] = [
        ("mate6-states-unflipped", &plain, Unreplayed::OtherDecisions),
        ("mate6-states-standoff", &standoff, Unreplayed::Numeric),
    ];
    for (label, writes, want) in cases {
        let (doc, ids, mate) =
            stand_by(label, part, &store, [TargetAxis::X, TargetAxis::Y], |ids| {
                writes(
                    in_part(ids[1], body, CapEnd::Start),
                    in_part(ids[0], body, CapEnd::End),
                )
            });
        let ev = run(&doc, &with_resolver(store.clone()));
        let row = at_rest_row(
            vec![
                copy_cap(&doc, ids[1], body, CapEnd::Start),
                copy_cap(&doc, ids[0], body, CapEnd::End),
            ],
            SameOpposite,
        );
        let proof = door(&doc, mate.placement, &row, &ev);
        assert_eq!(
            replayed(&proof).map(|(replay, _)| replay),
            Some(want),
            "{label}: {proof:?}"
        );
    }
}

/// **A row at rest the first rung leaves is read off the replay's
/// copies** (`[coincidence.structural]`'s second rung). Two copies of
/// one cube part, each standing where its body stands (the identity
/// bundle), and an at-rest row between their tops, which no mate
/// states:
///
/// - **two copies of one version** read its variables as one set of
///   symbols, so the kernel's carrier verdict on the replay's two tops
///   proves them one carrier;
/// - **copies of two versions of one document** each bind their
///   variables under their own pin (`analysis::symbol_scope`), through
///   the resolved part's evaluation, so the tops are a coincidence of
///   values (`Numeric`): two heights equal at their values;
/// - **a door handed no resolver** builds neither copy, and says the
///   row stands on a part (`ThroughAPart`), not that it was not read.
///
/// Red if the at-rest arm proves nothing, if a resolved part's variables
/// lose their pin on the way into its evaluation, or if a copy the
/// replay did not build reads as a row it did not read.
#[test]
fn a_row_at_rest_no_mate_states_is_read_off_the_replays_copies() {
    let mut store = PartStore::default();
    let (v1, body) = store.insert_part(cube_part("mate6-copies-cube"), Tol::witness());
    let relabelled = fixture::step(
        store.doc(v1.id),
        editor_core::DocEdit::SetLabel {
            node: body,
            label: Some(editor_core::Label::new("cube").expect("a label")),
        },
    )
    .0;
    let v2 = DocRef {
        id: v1.id,
        pin: editor_core::content_pin(&relabelled, Tol::witness()).expect("the pin computes"),
    };
    assert_ne!(v1.pin, v2.pin, "the premise: two versions of one document");
    let versions = Versions(
        [(v1, store.doc(v1.id)), (v2, relabelled)]
            .into_iter()
            .collect(),
    );
    let opts = editor_core::EvalOptions {
        resolver: Some(std::sync::Arc::new(versions)),
        ..editor_core::EvalOptions::default()
    };
    let stood = |label: &str, refs: [DocRef; 2]| {
        let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let mut ids = Vec::new();
        for r in refs {
            let (next, id) = insert(doc, mated_instance(r));
            doc = fixture::written(next, id, editor_core::stand());
            ids.push(id);
        }
        let row = at_rest_row(
            ids.iter()
                .map(|&id| copy_cap(&doc, id, body, CapEnd::End))
                .collect(),
            topo::Relation::SameOriented,
        );
        (doc, ids, row)
    };
    let (doc, ids, row) = stood("mate6-copies-one-version", [v1, v1]);
    let ev = run(&doc, &opts);
    assert_eq!(
        door(&doc, ids[0], &row, &ev),
        Proof::Structural(Rung::CarrierVerdict),
        "two copies of one version"
    );
    let unresolved = editor_core::coincide::prove(
        &doc,
        &[(ids[0], std::slice::from_ref(&row))],
        None,
        Tol::witness(),
        &editor_core::CancelToken::new(),
    )
    .expect("an uncanceled door");
    assert_eq!(
        replayed(&unresolved[0][0]),
        Some((
            Unreplayed::ThroughAPart,
            editor_core::coincide::Recourse::Assert
        )),
        "a door handed no resolver"
    );
    let (doc, ids, row) = stood("mate6-copies-two-versions", [v1, v2]);
    let ev = run(&doc, &opts);
    assert_eq!(
        replayed(&door(&doc, ids[0], &row, &ev)).map(|(replay, _)| replay),
        Some(Unreplayed::Numeric),
        "copies of two versions"
    );
}

/// A resolver holding several versions of a document, each by its pin.
#[derive(Debug)]
struct Versions(std::collections::BTreeMap<DocRef, ProfileDoc>);

impl editor_core::PartResolver for Versions {
    fn resolve(
        &self,
        doc_ref: &DocRef,
        _tol: Tol,
    ) -> Result<ProfileDoc, editor_core::ResolveFailure> {
        self.0
            .get(doc_ref)
            .cloned()
            .ok_or_else(|| editor_core::ResolveFailure {
                fault: editor_core::ResolveFault::Unresolved,
                message: "no such version".to_string(),
            })
    }
}

/// **Across the instantiate seam a record cites the part's record**:
/// the instance's patch cites record 0 of its one input, the part, and
/// the row's product, whose one placement is that instance, cites the
/// same record of the same part.
/// Red if the seam carries the part's citations unchanged (they name
/// the part's rows, which the instance does not hold).
#[test]
fn an_instances_record_cites_the_parts_record() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-seam-cube"), Tol::witness());
    let (stand, _, _) = stand("mate6-seam-stand", part, body, 0.0, &store);
    let stand = crate::fixture::assemble_world_copies(stand);
    let stand_ref = store.insert(stand, Tol::witness());
    let (outer, instances) = row_of("mate6-seam-row", stand_ref, 1, 4.0, &store);
    let ev = run(&outer, &with_resolver(store));
    let value = ev.value(instances[0]).expect("the instance evaluates");
    assert_eq!(
        value.carry.contacts.rows()[0].cites,
        topo::Cites::one(topo::Backing::Carried {
            input: 0,
            record: 0
        })
    );
    assert_eq!(
        value.carry.cited_inputs.as_ref(),
        [editor_core::CitedInput::Part(stand_ref)]
    );
    let product = product_recorded(&outer, &ev, Tol::witness()).expect("the row gathers");
    assert_eq!(
        product.contacts.rows()[0].cites,
        topo::Cites::one(topo::Backing::Carried {
            input: 0,
            record: 0
        })
    );
    // The instance is the product's one placement, so the product
    // cites its records by the part the instance references.
    assert_eq!(
        product.cited_inputs,
        vec![editor_core::CitedInput::Part(stand_ref)],
        "one input: the instance's part"
    );
}

/// **The world's census reads only the copies the world holds** (F's Q8
/// rule): a placed pair — a base pinned to the world and a cube seated on
/// it — and a loose pair, a cube standing in a `new_space()` and a second
/// seated on it. The world's product records the placed pair's seat
/// alone; the loose pair's copies are in no world product.
///
/// Red if the world's census records a touch of copies it does not hold
/// (two records), or misses the placed seat (none).
#[test]
fn the_world_records_the_placed_seat_and_not_the_loose_one() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-world-skip-cube"), Tol::witness());
    let (doc, ids) = seated_row("mate6-world-skip-row", part, 2, &store);
    let (doc, loose) = insert(doc, mated_instance(part));
    let (doc, _) = fixture::into_new_space(doc, loose);
    let (doc, on_loose) = insert(doc, mated_instance(part));
    let (doc, _) = seated(
        doc,
        rest_mate(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[1], body, CapEnd::Start),
            0.0,
        ),
    );
    let (doc, _) = seated(
        doc,
        rest_mate(
            in_part(loose, body, CapEnd::End),
            in_part(on_loose, body, CapEnd::Start),
            0.0,
        ),
    );

    let ev = run(&doc, &with_resolver(store));
    let gathered = product_recorded(&doc, &ev, Tol::witness()).expect("the gather stands");
    assert_eq!(
        gathered.contacts.face_face().count(),
        1,
        "the seat between placed copies, and only it: {:?}",
        gathered.contacts
    );
}

/// **The name of `body`'s entity `seg` in `instance`'s copy**: the
/// part's own name, which the instance's copy carries verbatim, read at
/// the instance ([`fixture::resolver::in_part_of`]).
fn in_part_entity(
    instance: RecipeNodeId,
    body: RecipeNodeId,
    kind: EntityKind,
    seg: RoleSeg,
) -> PartName {
    fixture::resolver::in_part_of(kind, instance, body, seg)
}

/// **A mate's wall states the rim on it** (`[coincidence.structural]`):
/// two cubes of one part, the upper's `x = 0` wall rested on the
/// lower's `x = 1` wall. The upper's end rim over that wall lies on the
/// wall it bounds, so a row laying it on the lower's wall is the mate's
/// statement, proven by the first rung. A rim over another wall is no
/// cell of the mate's faces, and a standoff (the lower wall's plane slid
/// `0` along its normal) equates no face's plane, so neither is the
/// mate's statement.
#[test]
fn a_mates_wall_states_the_rim_on_it_and_a_standoff_states_nothing() {
    let mut store = PartStore::default();
    let (part_doc, part_body) = cube_part("mate6-wall-cube");
    // Segment 1 runs along x = 1 and segment 3 along x = 0 (the square's
    // corner order).
    let (east, west, south) = (
        fixture::wall(&part_doc, part_body, 1),
        fixture::wall(&part_doc, part_body, 3),
        fixture::wall(&part_doc, part_body, 0),
    );
    let rim = |seg: &RoleSeg| match seg {
        RoleSeg::Lateral(run) => RoleSeg::RimEdge(CapEnd::End, run.clone()),
        other => panic!("a wall, got {other:?}"),
    };
    let (part, body) = store.insert_part((part_doc, part_body), Tol::witness());
    let standoff = |to: PartName| {
        use editor_core::pose::{PoseDef, PoseRole, RoleRead};
        let up = || fixture::flip(fixture::plane(fixture::head(to.clone())));
        editor_core::Operand::Pose(Box::new(PoseDef::Slide {
            pose: RoleRead::plain(up()),
            along: RoleRead::through(up(), PoseRole::Normal),
            by: len(0.0),
        }))
    };
    for (label, stated) in [("mate6-wall-rest", true), ("mate6-wall-standoff", false)] {
        // The walls face `±x`, which the mate takes: the slides across it.
        let (doc, ids, mate) =
            stand_by(label, part, &store, [TargetAxis::Y, TargetAxis::Z], |ids| {
                let (on, to) = (
                    in_part_entity(ids[1], body, EntityKind::Face, west.clone()),
                    in_part_entity(ids[0], body, EntityKind::Face, east.clone()),
                );
                if stated {
                    fixture::rest(fixture::head(on), fixture::head(to))
                } else {
                    editor_core::Mate {
                        on: fixture::plane(fixture::head(on)),
                        to: standoff(to),
                        orient: editor_core::Orientation::IDENTITY,
                    }
                }
            });
        let ev = run(&doc, &with_resolver(store.clone()));
        let cell = |instance: RecipeNodeId, kind, seg: RoleSeg| {
            let input = doc
                .output(instance, 0)
                .expect("an instance defines its copies");
            let name = in_part_entity(instance, body, kind, seg).name;
            assert!(
                ev.table(input).is_some_and(|t| t.lookup(&name).is_some()),
                "{label}: the premise, the copy names {name:?}"
            );
            NamedCell::Entity { input, name }
        };
        let lower_east = cell(ids[0], EntityKind::Face, east.clone());
        let on_wall = door(
            &doc,
            mate.placement,
            &at_rest_row(
                vec![
                    cell(ids[1], EntityKind::Edge, rim(&west)),
                    lower_east.clone(),
                ],
                topo::Relation::OnCarrier,
            ),
            &ev,
        );
        assert_eq!(
            on_wall == Proof::Structural(Rung::SameConstruction),
            stated,
            "{label}: the rim on the mated wall: {on_wall:?}"
        );
        let off_wall = door(
            &doc,
            mate.placement,
            &at_rest_row(
                vec![cell(ids[1], EntityKind::Edge, rim(&south)), lower_east],
                topo::Relation::OnCarrier,
            ),
            &ev,
        );
        assert_ne!(
            off_wall,
            Proof::Structural(Rung::SameConstruction),
            "{label}: a rim over another wall"
        );
    }
}
