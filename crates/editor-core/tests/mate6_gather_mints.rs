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

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AssemblyError, Attribution, AxisSense, CancelToken, CapEnd, ContactClass, DocEdit,
    DocRef, DocumentId, EntityKind, EvalOptions, Evaluation, Frame, MateFrame, MatePrimitive, Node,
    ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, StableName, assemble,
    content_pin, evaluate, product_recorded,
};
use fixture::{desc, insert, len, step};
use geom_core::Tol;

// ---- The stub store (ASM-2A/R2a's shape) ----

#[derive(Debug, Default, Clone)]
struct StubStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl StubStore {
    fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl editor_core::PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        if content_pin(doc, Tol::witness()).expect("the pin computes") != doc_ref.pin {
            return Err(fail(ResolveFault::PinMismatch, "the pin does not hold"));
        }
        Ok(doc.clone())
    }
}

fn opts(store: StubStore) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    }
}

fn run(doc: &ProfileDoc, o: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), o, Tol::witness())
}

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
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

/// A one-block part document: `[0,1]³`. Its extrude is node 1.
fn cube_part(label: &str) -> ProfileDoc {
    let (doc, _) = block(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    doc
}

/// A face of `instance`'s part product, named through the instance
/// qualifier (A12's reading-edge head), where the part's own face is
/// the cap of ITS node 1.
fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

/// The same reading, one level deeper: `instance`'s part is ITSELF an
/// assembly, and the face wanted is the cap of the cube inside the
/// sub-instance `sub` of that assembly.
fn in_part_in_part(instance: RecipeNodeId, sub: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(in_part(sub, cap)),
        }],
    }
}

fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    }
}

/// A `Rest` mate declaring `a`'s TOP face against `b`'s BOTTOM face,
/// seating `b` at height `seat` by frame coincidence. `seat = 1.0`
/// puts `b`'s bottom exactly on `a`'s top (the unit cube is z ∈ [0,1]);
/// anything larger leaves a definite gap and the declaration is FALSE.
fn rest_mate(a: StableName, b: StableName, seat: f64) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: frame([0.0, 0.0, seat], [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

/// **Issue 946's inner document**: a stand whose validity DEPENDS on
/// its mate — two cubes seated one on the other, the contact declared
/// by the mate and by nothing else. Its own `assemble` is green at
/// `seat = 1.0`; at a larger seat the declaration is false.
///
/// Returns the document, its two instance ids, and its mate's id.
fn stand(label: &str, part: DocRef, seat: f64) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(doc, Node::instantiate_part(part));
        doc = next;
        ids.push(id);
    }
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: rest_mate(
                in_part(ids[0], CapEnd::Top),
                in_part(ids[1], CapEnd::Bottom),
                seat,
            ),
        },
    );
    (doc, ids, mate.expect("the mate mints"))
}

/// `count` instances of `part`, each displaced `spacing * i` along +x
/// so the copies stay disjoint (ASM-2B row 4's shape).
fn row_of(
    label: &str,
    part: DocRef,
    count: usize,
    spacing: f64,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..count {
        let (next, id) = insert(doc, Node::instantiate_part(part));
        doc = next;
        if i > 0 {
            #[allow(clippy::cast_precision_loss)]
            let dx = spacing * i as f64;
            let (next, _) = step(
                doc,
                DocEdit::SetPlacement {
                    node: id,
                    frame: Frame::translation([dx, 0.0, 0.0]),
                },
            );
            doc = next;
        }
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
/// `UndeclaredContact` at the outer gate: the inner seat is a real
/// contact of the outer product, and the only thing that ever declared
/// it — the inner mate — was minted by a door the seam does not call.
#[test]
fn three_identical_stands_in_a_row_carry_their_inner_declarations() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("mate6-cube"), Tol::witness());
    let (inner, _, _) = stand("mate6-stand", part, 1.0);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, ids) = row_of("mate6-row", inner_ref, 3, 4.0);

    let ev = run(&outer, &opts(store));
    let gathered = product_recorded(&outer, &ev, Tol::witness()).expect("the row gathers");
    assert_eq!(
        gathered.body.solids().count(),
        6,
        "three stands of two cubes each"
    );
    assert_eq!(
        gathered.contacts.patches.len(),
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
    let mut store = StubStore::default();
    let part = store.insert(cube_part("mate6-deep-cube"), Tol::witness());
    let (inner, _, _) = stand("mate6-deep-stand", part, 1.0);
    let inner_ref = store.insert(inner, Tol::witness());
    // The MID document is a pair of stands, side by side and disjoint.
    let (mid, _) = row_of("mate6-deep-mid", inner_ref, 2, 4.0);
    let mid_ref = store.insert(mid, Tol::witness());
    let (outer, _) = row_of("mate6-deep-outer", mid_ref, 2, 12.0);

    let ev = run(&outer, &opts(store));
    let gathered = product_recorded(&outer, &ev, Tol::witness()).expect("the deep row gathers");
    assert_eq!(gathered.body.solids().count(), 8, "two mids of two stands");
    assert_eq!(
        gathered.contacts.patches.len(),
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
/// declares a seat its geometry does not make — the two cubes are a
/// half-unit apart — gathers and instantiates fine, and the outer
/// census REFUTES the carried declaration, never passes it silently.
///
/// **Which refuting arm** is the geometry's to choose, and this row
/// reads the one that fires rather than naming it in advance: parallel
/// faces a definite half-unit apart are counter-EVIDENCE, so the census
/// answers `ContactContradicted` — the stronger sibling of
/// `StaleContactDeclaration`, which is the *absence* of a witness. Both
/// are the refuting direction and both are findings against the
/// document; a row that demanded the weaker one would be asserting the
/// fixture, not the invariant.
///
/// The finding is unattributed at this gate, and the row says so rather
/// than pretending otherwise: attribution is by arena key against what
/// THIS document minted ([`Attribution`]'s contract), and a carried
/// declaration was minted by another document, whose bookkeeping does
/// not cross the seam. The kernel record does; the mate's name does not.
#[test]
fn a_carried_declaration_the_outer_geometry_refutes_is_refuted_loudly() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("mate6-gap-cube"), Tol::witness());
    let (inner, _, _) = stand("mate6-gap-stand", part, 1.5);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("mate6-gap-row", inner_ref, 1, 4.0);

    let ev = run(&outer, &opts(store));
    let result = assemble(&outer, &ev, Tol::witness());
    let raised = findings(&result);
    assert!(
        raised
            .iter()
            .any(|f| f.contains("ContactContradicted") || f.contains("StaleContactDeclaration")),
        "the outer census re-verifies what it consumed: {raised:?}"
    );
    let Err(AssemblyError::AtRest { findings }) = &result else {
        panic!("a refuted declaration is a finding against the document: {result:?}");
    };
    assert!(
        findings
            .iter()
            .all(|f| matches!(f.attribution, Attribution::Unattributed)),
        "a carried declaration names no mate of THIS document"
    );
}

/// INVARIANT: an OUTER document's own mate is still attributed. The
/// gather minted it, so `assemble` reads it from the product rather
/// than making it — and the gate's finding still names the mate that
/// authored the declaration it refutes.
#[test]
fn an_outer_mate_the_geometry_refutes_is_refuted_naming_its_mate() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("mate6-outer-cube"), Tol::witness());
    let (inner, subs, _) = stand("mate6-outer-stand", part, 1.0);
    let inner_ref = store.insert(inner, Tol::witness());

    // Two stands, and an OUTER mate seating the second stand's lower
    // cube on the first stand's upper cube — at a seat half a unit too
    // high, so the declaration is false of the geometry it places.
    let mut doc = ProfileDoc::empty(DocumentId::derive("mate6-outer-row"), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(doc, Node::instantiate_part(inner_ref));
        doc = next;
        ids.push(id);
    }
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: rest_mate(
                in_part_in_part(ids[0], subs[1], CapEnd::Top),
                in_part_in_part(ids[1], subs[0], CapEnd::Bottom),
                2.5,
            ),
        },
    );
    let mate = mate.expect("the outer mate mints");

    let ev = run(&doc, &opts(store));
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
            .any(|f| matches!(&f.attribution, Attribution::Refuted(m) if m.mate == mate)),
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
    let mut store = StubStore::default();
    let part = store.insert(cube_part("mate6-once-cube"), Tol::witness());
    let (doc, _, mate) = stand("mate6-once-stand", part, 1.0);

    let ev = run(&doc, &opts(store));
    let gathered = product_recorded(&doc, &ev, Tol::witness()).expect("the stand gathers");
    assert_eq!(
        gathered.contacts.patches.len(),
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
    let mut store = StubStore::default();
    let part = store.insert(cube_part("mate6-bare-cube"), Tol::witness());
    let (doc, _) = row_of("mate6-bare-row", part, 3, 4.0);

    let ev = run(&doc, &opts(store));
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

/// INVARIANT (**the split between the two doors**): a class the table
/// gives no at-rest record does not make the PRODUCT refuse. Minting is
/// the gather's act; whether a declared class carries a record AT REST
/// is the at-rest door's question, and `assemble` is where it refuses —
/// so a `Tangent` mate still shows its geometry and still refuses at
/// the gate, naming the class.
#[test]
fn a_class_with_no_at_rest_record_refuses_at_the_gate_not_at_the_gather() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("mate6-tangent-cube"), Tol::witness());
    let (doc, ids, _) = stand("mate6-tangent-stand", part, 1.0);
    let mut node = rest_mate(
        in_part(ids[0], CapEnd::Top),
        in_part(ids[1], CapEnd::Bottom),
        1.0,
    );
    if let Node::Mate { class, .. } = &mut node {
        *class = ContactClass::Tangent;
    }
    // Replace the stand's Rest mate with a Tangent one by authoring a
    // second document holding only the tangent declaration.
    let (doc, tangent) = step(doc, DocEdit::InsertNode { node });
    let tangent = tangent.expect("the tangent mate mints");

    let ev = run(&doc, &opts(store));
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
        Err(AssemblyError::NoAtRestRecord { class, mate, .. }) => {
            assert_eq!(class, ContactClass::Tangent);
            assert_eq!(mate, tangent, "naming the mate that declared it");
        }
        other => panic!("the at-rest door refuses a Tangent: {other:?}"),
    }
}
