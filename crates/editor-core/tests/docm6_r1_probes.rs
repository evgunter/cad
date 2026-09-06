//! R1 review probes for DOCM-6 (not for merge). Each row PRINTS what it
//! measured and asserts only what the review claims.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CancelToken, CapEnd, ContactClass, DocEdit, DocRef,
    DocumentId, EntityKind, EvalOptions, Evaluation, Frame, MateFrame, MatePrimitive, Node,
    ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, SitedRef, StableName,
    assemble, content_pin, evaluate, product_recorded,
};
use fixture::{ang, insert, len, on_frame, scl, step};
use geom_core::Tol;

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
            profile: p,
            distance: len(dz),
        },
    )
}

const PART_BODY: RecipeNodeId = RecipeNodeId(2);

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

fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: PART_BODY,
                path: vec![RoleSeg::Cap(cap)],
            }),
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

fn rest_mate_at(
    a: RecipeNodeId,
    b: RecipeNodeId,
    origin: [f64; 3],
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: SitedRef::at_mint(in_part(a, CapEnd::End)),
        b: SitedRef::at_mint(in_part(b, CapEnd::Start)),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: frame(origin, [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

/// Two cubes of `part`, mated `Rest` at `origin`. `[1,0,1]` is
/// `asm_r2b`'s GRAZING seat: one carrier, no shared area, decidable in
/// neither direction, so the census DECLINES the pair.
fn stand_at(label: &str, part: DocRef, origin: [f64; 3]) -> (ProfileDoc, RecipeNodeId) {
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
            node: rest_mate_at(ids[0], ids[1], origin),
        },
    );
    (doc, mate.expect("the mate inserts"))
}

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

fn variant(r: &Result<editor_core::Assembly<f64>, AssemblyError>) -> String {
    match r {
        Ok(_) => "Ok".to_string(),
        Err(AssemblyError::AtRest { findings }) => format!(
            "AtRest[{}]",
            findings
                .iter()
                .map(|f| format!("{:?}", f.attribution))
                .collect::<Vec<_>>()
                .join(" | ")
        ),
        Err(AssemblyError::Uncertified { findings, .. }) => format!(
            "Uncertified[{}]",
            findings
                .iter()
                .map(|f| format!("{:?}", f.attribution))
                .collect::<Vec<_>>()
                .join(" | ")
        ),
        Err(other) => format!("{other:?}"),
    }
}

/// R1-P1 (C4 / deviation 4): a CARRIED declaration the census merely
/// DECLINES, and nothing else. On the merge base every finding is
/// `Unattributed`, so the `Uncertified` predicate fails and the gate
/// answers `AtRest`; with the widened predicate the same fixture
/// answers `Uncertified`.
#[test]
fn r1_p1_a_carried_decline_alone_changes_the_gates_verdict() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1p1-cube"), Tol::witness());
    let (inner, inner_mate) = stand_at("r1p1-graze", part, [1.0, 0.0, 1.0]);
    let inner_ref = store.insert(inner.clone(), Tol::witness());
    let (outer, _) = row_of("r1p1-row", inner_ref, 1, 4.0);

    let inner_ev = run(&inner, &opts(store.clone()));
    let inner_result = assemble(&inner, &inner_ev, Tol::witness());
    println!(
        "R1-P1 inner (mate {inner_mate:?}): {}",
        variant(&inner_result)
    );

    let ev = run(&outer, &opts(store));
    let result = assemble(&outer, &ev, Tol::witness());
    println!("R1-P1 outer: {}", variant(&result));
    // Is the SAME fixture reachable from Python? `reads_as_prose`
    // rejects a message carrying `" { "`; a decline carries no
    // `FIT_DEFERRAL`, so `carried_declined` may be exercisable there
    // even though `carried_refuted` is not.
    let rendered = result.expect_err("the graze declines").to_string();
    println!(
        "R1-P1 outer message contains \" {{ \": {} ; text: {rendered}",
        rendered.contains(" { ")
    );
}

/// R1-P2 (C2's bound): the same carried declaration reaching the
/// product through a BOOLEAN over the instance. The records survive the
/// remap; the declaration rows do not.
#[test]
fn r1_p2_a_boolean_over_an_instance_drops_the_declaration_rows() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1p2-cube"), Tol::witness());
    let (inner, inner_mate) = stand_at("r1p2-pen", part, [0.0, 0.0, 0.5]);
    let inner_ref = store.insert(inner, Tol::witness());

    let (direct, _) = row_of("r1p2-direct", inner_ref, 1, 4.0);
    let ev = run(&direct, &opts(store.clone()));
    let direct_gathered = product_recorded(&direct, &ev, Tol::witness()).expect("gathers");
    println!(
        "R1-P2 direct carried rows: {} ; gate: {}",
        direct_gathered.carried.len(),
        variant(&assemble(&direct, &ev, Tol::witness()))
    );

    let mut doc = ProfileDoc::empty(DocumentId::derive("r1p2-boolean"), Tol::witness());
    let (next, inst) = insert(doc, Node::instantiate_part(inner_ref));
    doc = next;
    let (next, far) = block(doc, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    doc = next;
    let (doc, _) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: inst,
            b: far,
            declare: None,
        },
    );
    let ev = run(&doc, &opts(store));
    match product_recorded(&doc, &ev, Tol::witness()) {
        Ok(g) => println!(
            "R1-P2 boolean carried rows: {} ; patches: {} ; gate: {} (mate {inner_mate:?})",
            g.carried.len(),
            g.contacts.patches.len(),
            variant(&assemble(&doc, &ev, Tol::witness()))
        ),
        Err(e) => println!(
            "R1-P2 boolean gather refused: {e:?}; node errors: {:?}",
            (0..6)
                .map(|n| format!("{:?}", ev.node_error(RecipeNodeId(n))))
                .collect::<Vec<_>>()
        ),
    }
}

/// R1-P3 (per-lane ii): two instances of ONE part in one document carry
/// two rows with different `through` AND different re-keyed faces, and
/// the shared `Arc` never aliases the route.
#[test]
fn r1_p3_two_instances_of_one_part_carry_two_distinct_rows() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1p3-cube"), Tol::witness());
    let (inner, inner_mate) = stand_at("r1p3-stand", part, [0.0, 0.0, 1.0]);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, instances) = row_of("r1p3-row", inner_ref, 2, 4.0);

    let ev = run(&outer, &opts(store));
    let g = product_recorded(&outer, &ev, Tol::witness()).expect("gathers");
    println!("R1-P3 rows: {:?}", g.carried);
    assert_eq!(g.carried.len(), 2);
    assert_eq!(g.carried[0].through, instances[0]);
    assert_eq!(g.carried[1].through, instances[1]);
    assert_eq!(g.carried[0].declaration.mate, inner_mate);
    assert_eq!(g.carried[1].declaration.mate, inner_mate);
    assert!(g.carried[0].via.is_empty() && g.carried[1].via.is_empty());
    assert_ne!(
        g.carried[0].declaration.faces, g.carried[1].declaration.faces,
        "two instances re-key onto DIFFERENT aggregate faces"
    );
}

/// R1-P4: a `Transform` over an instance. The PR's sweep says it drops
/// records and rows alike; the row measures both.
#[test]
fn r1_p4_a_transform_over_an_instance_drops_records_and_rows_alike() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1p4-cube"), Tol::witness());
    let (inner, _) = stand_at("r1p4-stand", part, [0.0, 0.0, 1.0]);
    let inner_ref = store.insert(inner, Tol::witness());

    let mut doc = ProfileDoc::empty(DocumentId::derive("r1p4-row"), Tol::witness());
    let (next, inst) = insert(doc, Node::instantiate_part(inner_ref));
    doc = next;
    let (doc, _) = insert(
        doc,
        Node::Transform {
            input: inst,
            translation: [len(3.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let ev = run(&doc, &opts(store));
    match product_recorded(&doc, &ev, Tol::witness()) {
        Ok(g) => println!(
            "R1-P4 transform: carried {} ; patches {} ; gate {}",
            g.carried.len(),
            g.contacts.patches.len(),
            variant(&assemble(&doc, &ev, Tol::witness()))
        ),
        Err(e) => println!("R1-P4 transform gather refused: {e:?}"),
    }
}

/// R1-P5 (the `reads_as_prose` finding): the outer gate's own `Display`
/// for a refuted declared contact carries `topo::FIT_DEFERRAL`, whose
/// text contains the struct fingerprint `" { "` that
/// `pncad_py::errors::reads_as_prose` rejects. Measured here rather
/// than through Python so the same row runs on the merge base.
#[test]
fn r1_p5_a_contradicted_contact_renders_the_struct_fingerprint() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("r1p5-cube"), Tol::witness());
    let (inner, _) = stand_at("r1p5-pen", part, [0.0, 0.0, 0.5]);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("r1p5-row", inner_ref, 1, 4.0);
    let ev = run(&outer, &opts(store));
    let rendered = assemble(&outer, &ev, Tol::witness())
        .expect_err("the penetrating declaration is refuted")
        .to_string();
    println!(
        "R1-P5 contains \" {{ \": {} ; contains FIT_DEFERRAL: {}",
        rendered.contains(" { "),
        rendered.contains("Fit { gap }")
    );
}
