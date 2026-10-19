//! **The gather mints** (ASSEMBLY-DESIGN A3's "Declaration minting":
//! *evaluation carries each mate's declaration into the evaluated
//! body's contact record set*).
//!
//! Minting is the PRODUCT GATHER's act, so every evaluated product —
//! not only the one `assemble` happens to be looking at — carries its
//! mates' declarations. `assemble` is that product plus the kernel's
//! tier-3′ at-rest door, and mints nothing of its own.
//!
//! What that buys is the instantiation seam: a sub-assembly's mate
//! declarations ride into the consuming document under the graft's own
//! descendant map, exactly as its boolean-discovered records already
//! do. Construction composes; verification runs once, at the outermost
//! gate — and the rows below pin BOTH halves, because a carry that
//! nothing re-verifies would be a way to launder a false declaration
//! across a document boundary.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;
use editor_core::Formula;

use editor_core::{
    AssemblyError, Attribution, CapEnd, ContactClass, Coord, DocEdit, DocRef, DocumentId,
    EntityKind, Frame, MateRef, MintRefusal, Node, ProfileDoc, RecipeNodeId, RoleSeg, SlideAxis,
    StableName, Value, assemble, product_recorded,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
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
            side: ExtrudeSide::Along,
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

/// The same reading, one level deeper: `instance`'s part is ITSELF an
/// assembly, and the face wanted is the cap of the cube inside the
/// sub-instance `sub` of that assembly — an instance is a placement,
/// so its copy carries the face under the instance's own wrapper —
/// `body` the cube part's body.
fn in_part_in_part(
    instance: RecipeNodeId,
    sub: RecipeNodeId,
    body: RecipeNodeId,
    cap: CapEnd,
) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: in_part(sub, body, cap).into(),
        }],
    }
}

/// A `Rest` mate declaring `a`'s TOP face against `b`'s BOTTOM face,
/// standing `b` `apart` beside `a` ([`Seat`]). `apart = 0.0` puts `b`'s
/// bottom square on `a`'s top (the unit cube is z ∈ [0,1]); `2.0`
/// leaves the two faces coplanar with a unit of clear air between them,
/// and the declaration is FALSE.
fn rest_mate(a: StableName, b: StableName, apart: f64) -> Seat {
    classed_mate(a, b, apart, ContactClass::Rest)
}

/// [`rest_mate`]'s shape at an arbitrary class — the totality rows need
/// a `Tangent` declaration, whose class mints no record at rest: `b`'s
/// face's plane on `a`'s, facing it.
fn classed_mate(a: StableName, b: StableName, apart: f64, class: ContactClass) -> Seat {
    Seat {
        mate: fixture::rest(fixture::head(b), fixture::head(a), class),
        apart,
    }
}

/// **A rest and how far beside its target it stands**: the copy's
/// own origin slid `apart` along x from the target's in the rested
/// plane. `0.0` stands the two unit cubes square, face to face; `2.0`
/// leaves the faces coplanar with a unit of clear air between their
/// trims, so the declaration is FALSE.
struct Seat {
    mate: editor_core::Mate<Formula>,
    apart: f64,
}

/// `seat`'s mate added to `placement`'s bundle with the three freedoms
/// a plane mate leaves set: a slide of `apart` along x, none along y,
/// and no spin, so it pins the copy.
fn seated_at(doc: ProfileDoc, placement: RecipeNodeId, seat: Seat) -> (ProfileDoc, MateRef) {
    let (doc, m) = fixture::constrain_at(doc, placement, seat.mate);
    let doc = [
        (Coord::Slide(SlideAxis::X), len(seat.apart)),
        (Coord::Slide(SlideAxis::Y), len(0.0)),
        (Coord::Spin, fixture::ang(0.0)),
    ]
    .into_iter()
    .fold(doc, |doc, (coord, by)| {
        fixture::step(
            doc,
            DocEdit::AddValue {
                placement: m.placement,
                value: Box::new(Value {
                    mate: m.id,
                    coord,
                    by,
                }),
                fresh: Vec::new(),
            },
        )
        .0
    });
    (doc, m)
}

/// [`seated_at`] on the placement the seat's `on` reads.
fn seated(doc: ProfileDoc, seat: Seat) -> (ProfileDoc, MateRef) {
    let placement = fixture::placed_by(&doc, &seat.mate);
    seated_at(doc, placement, seat)
}

/// A reference that resolves to NO product face: the part's body has
/// caps, but node 99 does not exist in it at all. The constraint is
/// admitted — the door reads the name, not the part's faces — and the
/// placement it is in refuses at evaluation.
fn dangling(instance: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId::new(0, 99),
                path: vec![RoleSeg::Cap(CapEnd::End)],
            }
            .into(),
        }],
    }
}

/// **Issue 946's inner document**: a stand whose validity DEPENDS on
/// its mate — two cubes seated one on the other, the contact declared
/// by the mate and by nothing else. Its own `assemble` is green at
/// `apart = 0.0`; standing the upper cube beside the lower one, the
/// declaration is false ([`Seat`]). `body` is `part`'s body.
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
    classed_stand(label, part, body, apart, ContactClass::Rest, store)
}

/// [`stand`] with its mate declaring `class`.
fn classed_stand(
    label: &str,
    part: DocRef,
    body: RecipeNodeId,
    apart: f64,
    class: ContactClass,
    store: &PartStore,
) -> (ProfileDoc, Vec<RecipeNodeId>, MateRef) {
    let (doc, ids) = seated_row(label, part, 2, store);
    let (doc, mate) = seated(
        doc,
        classed_mate(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[1], body, CapEnd::Start),
            apart,
            class,
        ),
    );
    (doc, ids, mate)
}

/// `count` instances of `part`: the first pinned to the world, the rest
/// with empty bundles, each a loose copy until a mate places it.
fn seated_row(
    label: &str,
    part: DocRef,
    count: usize,
    store: &PartStore,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (mut doc, first) = fixture::insert_pinned(
        doc,
        Node::instantiate_part(part),
        &with_resolver(store.clone()),
    );
    let mut ids = vec![first];
    for _ in 1..count {
        let (next, id) = insert(doc, mated_instance(part));
        doc = next;
        ids.push(id);
    }
    (doc, ids)
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
    (doc, ids)
}

/// Every finding the gate raised, rendered — from either refusing arm,
/// so a row asserting about the finding SET need not know which side of
/// the split its fixture lands on.
fn findings(result: &Result<editor_core::Assembly<f64>, AssemblyError>) -> Vec<String> {
    match result {
        Ok(_) => Vec::new(),
        Err(AssemblyError::AtRest { findings } | AssemblyError::Uncertified { findings, .. }) => {
            findings.iter().map(|f| format!("{:?}", f.error)).collect()
        }
        Err(other) => panic!("expected an at-rest verdict, got {other}"),
    }
}

// ---- The seam ----

/// INVARIANT (**issue 946, the row it is about**): an inner document
/// whose validity depends on its mates, instantiated ×3, is valid in
/// the outer document — because the gather MINTED the inner
/// declarations and the seam carried them under the graft's descendant
/// map, exactly as it already carried boolean-discovered records.
///
/// Before minting moved into the gather this refused
/// `UnrecordedContact` at the outer gate: the inner seat is a real
/// contact of the outer product, and the only thing that ever declared
/// it — the inner mate — was minted by a door the seam does not call.
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
        "one carried declaration per stand — the inner mate's, re-keyed \
         onto the aggregate"
    );
    assert!(
        gathered.minted.is_empty(),
        "the OUTER document declares nothing of its own: {:?}",
        gathered.minted
    );

    let result = assemble(&outer, &ev, Tol::witness());
    let raised = findings(&result);
    assert!(
        raised.is_empty(),
        "the outer gate certifies a row of correctly-mated stands: {raised:?}"
    );
    assert!(!ids.is_empty());
}

/// INVARIANT: the carry is not one level deep. Outer–mid–inner: the
/// cube's seat is declared at the innermost document and survives two
/// grafts, because each level's gather mints and each seam carries what
/// its level's gather produced.
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
        "one carried declaration per stand, through two seams"
    );
    let raised = findings(&assemble(&outer, &ev, Tol::witness()));
    assert!(
        raised.is_empty(),
        "and the outermost gate certifies the lot: {raised:?}"
    );
}

// ---- Re-verification, with teeth ----

/// INVARIANT (**the ruling's soundness clause**): carrying a
/// declaration is not trusting it. An inner document whose OWN mate
/// declares a seat its geometry does not make — the two cubes' faces
/// coplanar with a unit of clear air between them — gathers and
/// instantiates fine, and the outer census REFUTES the carried
/// declaration, never passes it silently.
///
/// **Which refuting arm** is the geometry's to choose, and this row
/// reads the one that fires rather than naming it in advance: a rest
/// makes its two faces coplanar, so no counter-evidence separates their
/// carriers, and what refutes is the *absence* of a witness — one
/// carrier whose trims are definitely disjoint, `StaleContactDeclaration`.
/// A row that demanded another arm would be asserting the fixture, not
/// the invariant.
///
/// The finding NAMES the mate that authored the declaration, and the
/// document and instance it reached this gate through: attribution is
/// by arena key against every declaration of the tree, this document's
/// own and its parts' alike, and a part's rows cross the seam keyed by
/// the graft exactly as its records do. Naming a mate is not trusting
/// it — the verdict above is still the kernel's, taken here, once.
#[test]
fn a_carried_declaration_the_outer_geometry_refutes_is_refuted_loudly() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-gap-cube"), Tol::witness());
    let (inner, _, inner_mate) = stand("mate6-gap-stand", part, body, 2.0, &store);
    let inner_id = inner.id();
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, instances) = row_of("mate6-gap-row", inner_ref, 1, 4.0, &store);

    let ev = run(&outer, &with_resolver(store));
    let result = assemble(&outer, &ev, Tol::witness());
    let raised = findings(&result);
    // The arm that FIRES is asserted, not "either refuting arm": the
    // invariant this row defends is that the outer census re-verifies
    // what it consumed, and that is what makes the refutation loud
    // either way — but faces on one carrier whose trims are a unit
    // apart have no witness, so `StaleContactDeclaration` is the arm
    // the kernel's own vocabulary assigns it.
    assert!(
        raised.iter().any(|f| f.contains("StaleContactDeclaration")),
        "the outer census re-verifies what it consumed, and finds no witness: {raised:?}"
    );
    let Err(AssemblyError::AtRest { findings }) = &result else {
        panic!("a refuted declaration is a finding against the document: {result:?}");
    };
    assert!(
        findings.iter().any(|f| matches!(
            &f.attribution,
            Some(Attribution::Carried {
                route,
                declaration,
                relation: editor_core::Relation::Refuted,
                ..
            }) if route.through == instances[0]
                && route.of == inner_id
                && route.via.is_empty()
                && declaration.mate == inner_mate
        )),
        "the refuted carried declaration names its mate, its document and \
         the instance it arrived through: {findings:?}"
    );
    assert!(
        findings.iter().all(|f| f.attribution.is_some()),
        "and nothing here is anonymous any more: {findings:?}"
    );
}

/// INVARIANT: an OUTER document's own mate is still attributed. The
/// gather minted it, so `assemble` reads it from the product rather
/// than making it — and the gate's finding still names the mate that
/// authored the declaration it refutes.
#[test]
fn an_outer_mate_the_geometry_refutes_is_refuted_naming_its_mate() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-outer-cube"), Tol::witness());
    let (inner, subs, _) = stand("mate6-outer-stand", part, body, 0.0, &store);
    let inner_ref = store.insert(inner, Tol::witness());

    // Two stands, and an OUTER mate resting the second stand's lower
    // cube on the first stand's upper cube's plane — two units beside it,
    // so the declaration is false of the geometry it places.
    let (doc, ids) = seated_row("mate6-outer-row", inner_ref, 2, &store);
    let (doc, mate) = seated(
        doc,
        rest_mate(
            in_part_in_part(ids[0], subs[1], body, CapEnd::End),
            in_part_in_part(ids[1], subs[0], body, CapEnd::Start),
            2.0,
        ),
    );

    let ev = run(&doc, &with_resolver(store));
    let gathered = product_recorded(&doc, &ev, Tol::witness()).expect("the pair gathers");
    assert_eq!(
        gathered.minted.iter().map(|m| m.mate).collect::<Vec<_>>(),
        vec![mate],
        "the GATHER minted the outer mate's declaration"
    );

    let result = assemble(&doc, &ev, Tol::witness());
    let Err(AssemblyError::AtRest { findings }) = &result else {
        panic!("the false outer seat is a finding against the document: {result:?}");
    };
    assert!(
        findings
            .iter()
            .any(|f| matches!(&f.attribution, Some(Attribution::Refuted(m)) if m.mate == mate)),
        "naming the mate that declared it: {:?}",
        findings.iter().map(|f| &f.attribution).collect::<Vec<_>>()
    );
}

// ---- What `assemble` retains ----

/// INVARIANT: `assemble` = product + tier-3′. The record set it gates
/// is the GATHER's, unchanged — no second minting pass adds to it, so a
/// declaration appears exactly once however many doors read it.
#[test]
fn assemble_gates_the_gathers_own_record_set_and_mints_nothing() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-once-cube"), Tol::witness());
    let (doc, _, mate) = stand("mate6-once-stand", part, body, 0.0, &store);

    let ev = run(&doc, &with_resolver(store));
    let gathered = product_recorded(&doc, &ev, Tol::witness()).expect("the stand gathers");
    assert_eq!(
        gathered.contacts.face_face().count(),
        1,
        "minted once by the gather"
    );
    assert_eq!(
        gathered.minted.iter().map(|m| m.mate).collect::<Vec<_>>(),
        vec![mate]
    );
    let assembly = assemble(&doc, &ev, Tol::witness()).expect("the stand assembles");
    assert_eq!(
        assembly.contacts, gathered.contacts,
        "and `assemble` gates that set verbatim"
    );
    assert_eq!(
        assembly.minted, gathered.minted,
        "reading the gather's rows rather than minting its own"
    );
}

/// INVARIANT: a document with no mates is untouched by any of this —
/// the gather's record set is exactly what its sources carried up, and
/// its minted list is empty.
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
        "no mates, no records"
    );
    assert!(gathered.minted.is_empty());
    assert!(gathered.unminted.is_empty());
}

/// INVARIANT (**the premise `attribute`'s `StaleContactDeclaration` arm
/// reasons from**): what this door mints is face–face records and
/// nothing else, so no edge–face staleness names a mate's record — the
/// claim is a row rather than a paragraph. (A record's two faces are
/// distinct by its type; `mint` pins the same of the row it writes with
/// its own `debug_assert`, which this row also exercises, tests being a
/// debug build.)
#[test]
fn mint_makes_face_face_records_and_nothing_else() {
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
    assert_eq!(gathered.minted.len(), 2, "both mates minted");
    assert_eq!(
        gathered.contacts.face_face().count(),
        2,
        "a mate authors face–face records only: {:?}",
        gathered.contacts
    );
    assert_eq!(gathered.contacts.len(), 2, "{:?}", gathered.contacts);
    for m in &gathered.minted {
        assert_ne!(m.faces.0, m.faces.1, "and so does the row that records it");
    }
}

/// INVARIANT (**the split between the two doors**): a class the table
/// gives no at-rest record does not make the PRODUCT refuse. Minting is
/// the gather's act; whether a declared class carries a record AT REST
/// is the at-rest door's question, and `assemble` is where it refuses —
/// so a `Tangent` mate still shows its geometry and still refuses at
/// the gate, naming the class.
#[test]
fn a_class_with_no_at_rest_record_refuses_at_the_gate_not_at_the_gather() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-tangent-cube"), Tol::witness());
    // The stand, seated by a Tangent mate rather than a Rest one.
    let (doc, _, tangent) = classed_stand(
        "mate6-tangent-stand",
        part,
        body,
        0.0,
        ContactClass::Tangent,
        &store,
    );

    let ev = run(&doc, &with_resolver(store));
    let gathered = product_recorded(&doc, &ev, Tol::witness())
        .expect("the gather answers with the geometry, not a refusal");
    assert_eq!(gathered.body.solids().count(), 2);
    assert_eq!(
        gathered.unminted.len(),
        1,
        "and records the one declaration it could not mint: {:?}",
        gathered.unminted
    );
    match assemble(&doc, &ev, Tol::witness()) {
        Err(AssemblyError::Mint { refusals }) => match refusals.as_slice() {
            [MintRefusal::NoAtRestRecord { class, mate, .. }] => {
                assert_eq!(*class, ContactClass::Tangent);
                assert_eq!(*mate, tangent, "naming the mate that declared it");
            }
            rows => panic!("one mate refused, so one row: {rows:?}"),
        },
        other => panic!("the at-rest door refuses a Tangent: {other:?}"),
    }
}

// ---- Totality: the guard on `mint`'s two `continue`s ----
//
// `mint` is TOTAL over the document's live mates: a mate it cannot mint
// yields a `MintRefusal` row and the walk CONTINUES. That is the change
// this branch makes, and it is only a claim if a row can tell it apart
// from the stop-at-first-bad-mate walk it replaced.
//
// Every row ABOVE is blind to the difference, by construction: each
// places its unmintable mate LAST in document order, so a walk that
// stops there has already minted everything a total walk would. The
// three rows below are the guard. Each puts its bad mate BEFORE a good
// one and asserts the good one's record survives: a constraint whose
// reference resolves to nothing (its placement fails, so it is put out
// of the product, where its failure stops nothing), and the unmintable
// class, which mints a row and continues.

/// GUARD (`mint`'s reference arm, restated over a loose copy): a
/// constraint whose side resolves to no face fails its placement, and a
/// failed placement in the product refuses the gather. Its placement is
/// put out of the product here — its target is a copy no bundle pins —
/// so its failure stops nothing: the mate authored after it still
/// mints, and the bad one's refusal is its placement's, at evaluation.
#[test]
fn a_dangling_reference_before_a_good_mate_does_not_swallow_it() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-tot-ref-cube"), Tol::witness());
    let (doc, ids) = seated_row("mate6-tot-ref-row", part, 4, &store);
    // `ids[3]` stays loose: the copy the bad constraint places is in its
    // space, not the world.
    let (doc, bad) = seated_at(
        doc,
        ids[1],
        rest_mate(dangling(ids[3]), in_part(ids[1], body, CapEnd::Start), 0.0),
    );
    let (doc, good) = seated(
        doc,
        rest_mate(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[2], body, CapEnd::Start),
            0.0,
        ),
    );

    let ev = run(&doc, &with_resolver(store));
    assert!(
        ev.node_error(bad.placement).is_some(),
        "the bad reference fails its placement"
    );
    let gathered = product_recorded(&doc, &ev, Tol::witness()).expect("the gather stands");
    assert_eq!(
        gathered.minted.iter().map(|m| m.mate).collect::<Vec<_>>(),
        vec![good],
        "the mate AFTER the bad reference still minted"
    );
    assert_eq!(
        gathered.contacts.face_face().count(),
        1,
        "and its record is in the set the gate will read"
    );
    assert!(
        gathered.unminted.is_empty(),
        "a constraint off the product is asked nothing by its gather: {:?}",
        gathered.unminted
    );
}

/// GUARD (`mint`'s class arm): the same claim for the other `continue` —
/// a class the table gives no at-rest record is recorded and stepped
/// over, and the mate authored after it still mints.
#[test]
fn an_unmintable_class_before_a_good_mate_does_not_swallow_it() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-tot-class-cube"), Tol::witness());
    let (doc, ids) = seated_row("mate6-tot-class-row", part, 3, &store);
    // Non-touching (two units beside) so the Tangent places its cube
    // without seating it: this row is about the mint walk, not about
    // geometry.
    let (doc, bad) = seated(
        doc,
        classed_mate(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[1], body, CapEnd::Start),
            2.0,
            ContactClass::Tangent,
        ),
    );
    let (doc, good) = seated(
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
        gathered.minted.iter().map(|m| m.mate).collect::<Vec<_>>(),
        vec![good],
        "the mate AFTER the unmintable class still minted"
    );
    assert_eq!(gathered.contacts.face_face().count(), 1);
    assert_eq!(
        gathered
            .unminted
            .iter()
            .map(MintRefusal::mate)
            .collect::<Vec<_>>(),
        vec![bad]
    );
    assert!(
        matches!(gathered.unminted[0], MintRefusal::NoAtRestRecord { .. }),
        "recorded under the class arm: {:?}",
        gathered.unminted
    );
}

/// GUARD (the rows, and their ORDER): a document that refuses twice
/// with a good mate between them. The refusal rows are the whole set, in
/// DOCUMENT ORDER — which is what makes `assemble`'s "raise every one"
/// a statement about the document rather than about the walk.
///
/// This is the row the stop-at-first-bad-mate walk cannot pass on any
/// assertion: it would mint nothing, record one refusal, and never reach
/// the second. A constraint whose reference resolves to nothing fails
/// its placement rather than minting a row; one is here too, on a copy
/// out of the product, and its refusal is its placement's at evaluation
/// while the rows stand.
#[test]
fn every_unmintable_mate_gets_its_row_in_document_order() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-tot-order-cube"), Tol::witness());
    let (doc, ids) = seated_row("mate6-tot-order-row", part, 6, &store);
    let (doc, first_bad) = seated(
        doc,
        classed_mate(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[1], body, CapEnd::Start),
            2.0,
            ContactClass::Tangent,
        ),
    );
    let (doc, good) = seated(
        doc,
        rest_mate(
            in_part(ids[1], body, CapEnd::End),
            in_part(ids[2], body, CapEnd::Start),
            0.0,
        ),
    );
    // `ids[5]` stays loose, so the copy this bad reference places is out
    // of the product.
    let (doc, dangling_mate) = seated_at(
        doc,
        ids[4],
        rest_mate(dangling(ids[5]), in_part(ids[4], body, CapEnd::Start), 0.0),
    );
    let (doc, second_bad) = seated(
        doc,
        classed_mate(
            in_part(ids[2], body, CapEnd::End),
            in_part(ids[3], body, CapEnd::Start),
            2.0,
            ContactClass::Tangent,
        ),
    );

    let ev = run(&doc, &with_resolver(store));
    assert!(
        ev.node_error(dangling_mate.placement).is_some(),
        "the bad reference fails its placement"
    );
    let gathered = product_recorded(&doc, &ev, Tol::witness()).expect("the gather stands");
    assert_eq!(
        gathered.minted.iter().map(|m| m.mate).collect::<Vec<_>>(),
        vec![good],
        "the good mate between two bad ones still minted"
    );
    assert_eq!(gathered.contacts.face_face().count(), 1);
    assert_eq!(
        gathered
            .unminted
            .iter()
            .map(MintRefusal::mate)
            .collect::<Vec<_>>(),
        vec![first_bad, second_bad],
        "both refusals, in document order"
    );
    assert!(
        gathered
            .unminted
            .iter()
            .all(|row| matches!(row, MintRefusal::NoAtRestRecord { .. })),
        "each under the class arm: {:?}",
        gathered.unminted
    );
    // And the door that raises reads the WHOLE set, in document order:
    // an author with two broken mates has two repairs, and a verdict
    // naming one of them makes the second a second evaluation.
    match assemble(&doc, &ev, Tol::witness()) {
        Err(AssemblyError::Mint { refusals }) => {
            assert_eq!(
                refusals.iter().map(MintRefusal::mate).collect::<Vec<_>>(),
                vec![first_bad, second_bad],
                "every refusal, in document order"
            );
            let rendered = AssemblyError::Mint { refusals }.to_string();
            let said = |mate: MateRef| {
                format!(
                    "constraint {} of node {}",
                    mate.id,
                    test_utils::refusal::tag(mate.placement.0.digest())
                )
            };
            assert!(
                rendered.contains(&said(first_bad)) && rendered.contains(&said(second_bad)),
                "and both are in the one message: {rendered:?}"
            );
        }
        other => panic!("expected every refusal raised, got {other:?}"),
    }
}

// ---- What a record cites ----

/// **A mated record cites the census's row, named in its copies, and
/// the lint reports it.** Two cubes of one part, the second rested on
/// the first by a plane mate and its three values: the product holds one
/// face–face record, citing its row 0, the at-rest census's decision that
/// the two seated caps rest on one carrier, kept beside the mate whose
/// pair it decided. The row names each cap in its copy, as that
/// instance's output names it. `unproven-coincidence` reports the row
/// against the mate, not replayed: the door builds no placed copies at
/// `Sym` ([`editor_core::coincide::REPLAYS_PLACED_COPIES`], off until
/// `work/intent/a-mate-placed-seat-is-unproven-at-the-doors-budget.md`),
/// so it has no form of the row and its recourse is an assertion.
///
/// Red if the mint makes a record citing nothing the product holds,
/// names a cap anywhere but in its copy, or the lint stays silent.
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
        "one mate, one record"
    );
    assert_eq!(
        product.contacts.rows()[0].cites,
        topo::Cites::decided(0),
        "the record cites the product's row 0"
    );
    let [row] = product.coincidences.as_slice() else {
        panic!("one row: {:?}", product.coincidences);
    };
    assert_eq!(
        row.by,
        editor_core::AtRestBy::Mate(mate),
        "the row is the mate's pair's"
    );
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
    assert!(
        product.refused_at_rest.is_empty(),
        "{:?}",
        product.refused_at_rest
    );
    let report = editor_core::run_checks(
        &stand,
        &ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("checks run over the stand");
    assert!(
        report.findings.iter().any(|f| matches!(
            &f.evidence,
            editor_core::CheckEvidence::UnprovenCoincidence {
                residual,
                recourse: editor_core::coincide::Recourse::Assert,
                ..
            } if residual.replay == editor_core::Unreplayed::NotReplayed
        ) && f.subject
            == editor_core::FindingSubject::Node(mate.placement)),
        "the census's row is reported against its mate, not replayed: {report}"
    );
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
    let stand_ref = store.insert(stand, Tol::witness());
    let (outer, instances) = row_of("mate6-seam-row", stand_ref, 1, 4.0, &store);
    let ev = run(&outer, &with_resolver(store));
    let value = ev.value(instances[0]).expect("the instance evaluates");
    assert_eq!(
        value.contacts.rows()[0].cites,
        topo::Cites::one(topo::Backing::Carried {
            input: 0,
            record: 0
        })
    );
    assert_eq!(
        value.cited_inputs.as_ref(),
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

/// **The gather mints only the mates of the space it gathers** (F's Q8
/// rule, landing with the world): a placed pair — a base pinned to the
/// world and a cube seated on it — and a loose pair, a cube no bundle
/// pins and a second seated on it. The world's gather mints exactly the
/// placed pair's mate, and the loose pair's mate states nothing (no
/// record, no refusal), because its copies are in no product.
///
/// Red if the skip swallows a mate whose copies ARE in the world
/// (nothing minted), or stops skipping the loose one (two minted, or a
/// refusal for a face the product does not hold).
#[test]
fn the_gate_skips_a_mate_on_a_loose_copy_and_mints_the_placed_ones() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("mate6-world-skip-cube"), Tol::witness());
    let (doc, ids) = seated_row("mate6-world-skip-row", part, 2, &store);
    let (doc, loose) = insert(doc, mated_instance(part));
    let (doc, on_loose) = insert(doc, mated_instance(part));
    let (doc, placed_mate) = seated(
        doc,
        rest_mate(
            in_part(ids[0], body, CapEnd::End),
            in_part(ids[1], body, CapEnd::Start),
            0.0,
        ),
    );
    let (doc, loose_mate) = seated(
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
        gathered.minted.iter().map(|m| m.mate).collect::<Vec<_>>(),
        vec![placed_mate],
        "the mate between placed copies mints, and only it"
    );
    assert!(
        gathered.unminted.is_empty(),
        "the mate between loose copies refuses nothing: {:?}",
        gathered.unminted
    );
    crate::fixture::assert_mints_nothing(&doc, &ev, loose_mate);
}
