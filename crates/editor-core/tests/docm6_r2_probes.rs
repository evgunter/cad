//! DOCM-6 R2 review probes — assemblies the implementer did not choose.
//!
//! - P1: a FOUR-level tree (parts in sub-assemblies in a mid in an
//!   outer in an outermost), two different parts, an instance repeated,
//!   a sub-assembly instantiated twice, a mid-level mate of the mid's
//!   own. Oracle for the re-keying: the product's NAME TABLE, not the
//!   record set — each carried row's two faces must be exactly what
//!   the route-wrapped inner reference names resolve to.
//! - P2: two instances of ONE document at the SAME placement, and a
//!   memo re-evaluation: rows differ in `through`, agree in
//!   `declaration` (mate, names, class), and no `via` aliases.
//! - P3: the boolean-over-an-instance bound the PR discloses, built.
//! - P4: a carried DECLINE reaches `Uncertified` (deviation 4).
//! - P5: an ANGULAR contradiction carries no `Fit { gap }` steer, so a
//!   Python `carried_refuted` row is writable.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, Assembly, AssemblyError, Attribution, AxisSense, BooleanOp, CancelToken, CapEnd,
    CarriedRelation, ContactClass, DocEdit, DocRef, DocumentId, EntityKey, EntityKind, Entry,
    EvalOptions, Evaluation, Frame, MateFrame, MatePrimitive, Node, ProfileDoc, RecipeNodeId,
    ResolveFailure, ResolveFault, RoleSeg, SitedRef, StableName, assemble, content_pin, evaluate,
    product_recorded,
};
use fixture::{insert, len, on_frame, step};
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

fn run_after(doc: &ProfileDoc, prior: &Evaluation<f64>, o: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, Some(prior), &CancelToken::new(), o, Tol::witness())
}

/// A block part: `w` × `d` footprint, extruded `h`. Three nodes, so
/// the body's names carry node 2.
fn block_part(label: &str, w: f64, d: f64, h: f64) -> ProfileDoc {
    let (doc, profile) = on_frame(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, d), (0.0, d)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
        },
    );
    doc
}

const PART_BODY: RecipeNodeId = RecipeNodeId(2);

fn wrap(node: RecipeNodeId, inner: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::InPart {
            of: Box::new(inner),
        }],
    }
}

fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    wrap(
        instance,
        StableName {
            kind: EntityKind::Face,
            node: PART_BODY,
            path: vec![RoleSeg::Cap(cap)],
        },
    )
}

fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    }
}

fn mate_node(
    a: StableName,
    b: StableName,
    class: ContactClass,
    a_frame: MateFrame,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: SitedRef::at_mint(a),
        b: SitedRef::at_mint(b),
        class,
        alignment: Alignment {
            a: a_frame,
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

fn place(doc: ProfileDoc, node: RecipeNodeId, at: [f64; 3]) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetPlacement {
            node,
            frame: Frame::translation(at),
        },
    )
    .0
}

fn findings_of(result: &Result<Assembly<f64>, AssemblyError>) -> Vec<editor_core::AtRestFinding> {
    match result {
        Err(AssemblyError::AtRest { findings } | AssemblyError::Uncertified { findings, .. }) => {
            findings.clone()
        }
        _ => Vec::new(),
    }
}

/// The face a route-wrapped name resolves to in `gathered`'s table,
/// or a panic naming the miss.
fn face_named(gathered: &editor_core::Product<f64>, name: &StableName) -> topo::FaceKey {
    match gathered.names.lookup(name) {
        Some(Entry::Unique(ent)) => match ent.key {
            EntityKey::Face(f) => f,
            other => panic!("{name:?} names {other:?}, not a face"),
        },
        other => panic!("{name:?} resolves to {other:?} in the product"),
    }
}

/// The name-table oracle: each carried row's faces are EXACTLY the
/// faces its route-wrapped inner references name in the product.
fn assert_rows_match_names(gathered: &editor_core::Product<f64>) {
    for row in &gathered.carried {
        let route = |inner: &StableName| {
            let mut name = inner.clone();
            for &node in row.via.iter().rev() {
                name = wrap(node, name);
            }
            wrap(row.through, name)
        };
        let want = (
            face_named(gathered, &route(&row.declaration.a)),
            face_named(gathered, &route(&row.declaration.b)),
        );
        assert_eq!(
            row.declaration.faces, want,
            "row {row:?} is not keyed to the faces its own references name"
        );
    }
}

/// Sub-assembly S1: a cube resting on a 2×2×0.5 slab, at the corner.
fn s1(store: &mut StubStore, label: &str) -> (DocRef, DocumentId, RecipeNodeId) {
    let slab = store.insert(block_part(&format!("{label}-slab"), 2.0, 2.0, 0.5), Tol::witness());
    let cube = store.insert(block_part(&format!("{label}-cube"), 1.0, 1.0, 1.0), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, s) = insert(doc, Node::instantiate_part(slab));
    let (doc, c) = insert(doc, Node::instantiate_part(cube));
    let (doc, mate) = insert(
        doc,
        mate_node(
            in_part(s, CapEnd::End),
            in_part(c, CapEnd::Start),
            ContactClass::Rest,
            frame([0.0, 0.0, 0.5], [0.0, 0.0, 1.0]),
        ),
    );
    let id = doc.id();
    (store.insert(doc, Tol::witness()), id, mate)
}

/// Sub-assembly S2: two cubes stacked, plus an unmated slab off to
/// the side. Returns the top cube's instance id too.
fn s2(store: &mut StubStore, label: &str) -> (DocRef, DocumentId, RecipeNodeId, RecipeNodeId) {
    let cube = store.insert(block_part(&format!("{label}-cube"), 1.0, 1.0, 1.0), Tol::witness());
    let slab = store.insert(block_part(&format!("{label}-slab"), 2.0, 2.0, 0.5), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, c0) = insert(doc, Node::instantiate_part(cube));
    let (doc, c1) = insert(doc, Node::instantiate_part(cube));
    let (doc, mate) = insert(
        doc,
        mate_node(
            in_part(c0, CapEnd::End),
            in_part(c1, CapEnd::Start),
            ContactClass::Rest,
            frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]),
        ),
    );
    let (doc, s) = insert(doc, Node::instantiate_part(slab));
    let doc = place(doc, s, [5.0, 0.0, 0.0]);
    let id = doc.id();
    (store.insert(doc, Tol::witness()), id, mate, c1)
}

struct Mid {
    doc_ref: DocRef,
    id: DocumentId,
    mate: RecipeNodeId,
    s1a: RecipeNodeId,
    s1b: RecipeNodeId,
    s2: RecipeNodeId,
    s1_id: DocumentId,
    s1_mate: RecipeNodeId,
    s2_id: DocumentId,
    s2_mate: RecipeNodeId,
}

/// The mid document: S1 twice, S2 once, a bare cube twice, and a
/// third cube MATED by the mid's own mate onto S2's top cube.
fn mid(store: &mut StubStore, label: &str) -> Mid {
    let (s1_ref, s1_id, s1_mate) = s1(store, &format!("{label}-s1"));
    let (s2_ref, s2_id, s2_mate, s2_top) = s2(store, &format!("{label}-s2"));
    let cube = store.insert(block_part(&format!("{label}-cube"), 1.0, 1.0, 1.0), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, s1a) = insert(doc, Node::instantiate_part(s1_ref));
    let (doc, s1b) = insert(doc, Node::instantiate_part(s1_ref));
    let doc = place(doc, s1b, [10.0, 0.0, 0.0]);
    let (doc, s2i) = insert(doc, Node::instantiate_part(s2_ref));
    let doc = place(doc, s2i, [20.0, 0.0, 0.0]);
    let (doc, c1) = insert(doc, Node::instantiate_part(cube));
    let doc = place(doc, c1, [30.0, 0.0, 0.0]);
    let (doc, c2) = insert(doc, Node::instantiate_part(cube));
    let doc = place(doc, c2, [40.0, 0.0, 0.0]);
    let (doc, c3) = insert(doc, Node::instantiate_part(cube));
    let (doc, mate) = insert(
        doc,
        mate_node(
            wrap(s2i, in_part(s2_top, CapEnd::End)),
            in_part(c3, CapEnd::Start),
            ContactClass::Rest,
            frame([0.0, 0.0, 2.0], [0.0, 0.0, 1.0]),
        ),
    );
    let id = doc.id();
    Mid {
        doc_ref: store.insert(doc, Tol::witness()),
        id,
        mate,
        s1a,
        s1b,
        s2: s2i,
        s1_id,
        s1_mate,
        s2_id,
        s2_mate,
    }
}

/// P1: the four-level tree. Rows one for one with the inner minted
/// sets, `of` naming the MINTING document, `via` nearest first, and
/// every face pair the name table's own answer.
#[test]
fn p1_four_levels_every_row_is_keyed_to_what_its_names_resolve_to() {
    let mut store = StubStore::default();
    let m = mid(&mut store, "r2-p1-mid");
    let cube = store.insert(block_part("r2-p1-cube", 1.0, 1.0, 1.0), Tol::witness());
    // Outer: a bare cube FIRST so the mid instances' ids (1, 2) do not
    // all coincide with the mid's own sub-instance ids (0, 1, 2).
    let outer = ProfileDoc::empty(DocumentId::derive("r2-p1-outer"), Tol::witness());
    let (outer, oc) = insert(outer, Node::instantiate_part(cube));
    let outer = place(outer, oc, [0.0, -50.0, 0.0]);
    let (outer, om1) = insert(outer, Node::instantiate_part(m.doc_ref));
    let (outer, om2) = insert(outer, Node::instantiate_part(m.doc_ref));
    let outer = place(outer, om2, [0.0, 100.0, 0.0]);
    let outer_id = outer.id();

    let ev = run(&outer, &opts(store.clone()));
    let gathered = product_recorded(&outer, &ev, Tol::witness()).expect("the outer gathers");
    let per_mid = |om: RecipeNodeId| {
        vec![
            (om, m.id, vec![], m.mate),
            (om, m.s1_id, vec![m.s1a], m.s1_mate),
            (om, m.s1_id, vec![m.s1b], m.s1_mate),
            (om, m.s2_id, vec![m.s2], m.s2_mate),
        ]
    };
    let mut expected = per_mid(om1);
    expected.extend(per_mid(om2));
    assert_eq!(
        gathered
            .carried
            .iter()
            .map(|c| (c.through, c.of, c.via.clone(), c.declaration.mate))
            .collect::<Vec<_>>(),
        expected,
        "outer rows"
    );
    assert!(gathered.minted.is_empty() && gathered.carried_unminted.is_empty());
    assert!(
        gathered.carried.iter().all(|c| c.of != outer_id),
        "no row is `of` the document that merely passed it"
    );
    assert_rows_match_names(&gathered);
    let outer_result = assemble(&outer, &ev, Tol::witness());
    assert!(outer_result.is_ok(), "the outer certifies: {outer_result:?}");

    // Level four: the outer instantiated once more. `via` gains a
    // second entry, nearest first.
    let outer_ref = store.insert(outer, Tol::witness());
    let top = ProfileDoc::empty(DocumentId::derive("r2-p1-top"), Tol::witness());
    let (top, t) = insert(top, Node::instantiate_part(outer_ref));
    let ev = run(&top, &opts(store));
    let gathered = product_recorded(&top, &ev, Tol::witness()).expect("the top gathers");
    let mut expected = Vec::new();
    for om in [om1, om2] {
        expected.push((t, m.id, vec![om], m.mate));
        expected.push((t, m.s1_id, vec![om, m.s1a], m.s1_mate));
        expected.push((t, m.s1_id, vec![om, m.s1b], m.s1_mate));
        expected.push((t, m.s2_id, vec![om, m.s2], m.s2_mate));
    }
    assert_eq!(
        gathered
            .carried
            .iter()
            .map(|c| (c.through, c.of, c.via.clone(), c.declaration.mate))
            .collect::<Vec<_>>(),
        expected,
        "top rows"
    );
    assert_rows_match_names(&gathered);
    assert!(assemble(&top, &ev, Tol::witness()).is_ok());
}

/// P2: two instances of ONE document at the SAME placement, and the
/// memo. Rows differ in `through` only; a re-evaluation from the
/// prior serves the same rows.
#[test]
fn p2_repeated_instance_at_one_placement_and_the_memo() {
    let mut store = StubStore::default();
    let m = mid(&mut store, "r2-p2-mid");
    let outer = ProfileDoc::empty(DocumentId::derive("r2-p2-outer"), Tol::witness());
    let (outer, om1) = insert(outer, Node::instantiate_part(m.doc_ref));
    let (outer, om2) = insert(outer, Node::instantiate_part(m.doc_ref));
    let o = opts(store);
    let ev = run(&outer, &o);
    let gathered = product_recorded(&outer, &ev, Tol::witness()).expect("gathers");
    assert_eq!(gathered.carried.len(), 8);
    let (first, second) = gathered.carried.split_at(4);
    for (a, b) in first.iter().zip(second) {
        assert_eq!((a.through, b.through), (om1, om2));
        assert_eq!((a.of, &a.via), (b.of, &b.via), "the route below differs in nothing");
        assert_eq!(
            (&a.declaration.mate, &a.declaration.a, &a.declaration.b, a.declaration.class),
            (&b.declaration.mate, &b.declaration.a, &b.declaration.b, b.declaration.class),
        );
        assert_ne!(a.declaration.faces, b.declaration.faces, "two instances, two face pairs");
    }
    assert_rows_match_names(&gathered);

    // The memo: every node reused, rows identical.
    let ev2 = run_after(&outer, &ev, &o);
    assert!(ev2.reused > 0 && ev2.recomputed == 0, "{} / {}", ev2.reused, ev2.recomputed);
    let again = product_recorded(&outer, &ev2, Tol::witness()).expect("gathers again");
    assert_eq!(again.carried, gathered.carried);
}

/// The mate suites' stand: two cubes, one mate at `seat` with the
/// a-frame axis `axis`.
fn stand(
    store: &mut StubStore,
    label: &str,
    seat: [f64; 3],
    axis: [f64; 3],
) -> (DocRef, DocumentId, RecipeNodeId) {
    let cube = store.insert(block_part(&format!("{label}-cube"), 1.0, 1.0, 1.0), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, c0) = insert(doc, Node::instantiate_part(cube));
    let (doc, c1) = insert(doc, Node::instantiate_part(cube));
    let (doc, mate) = insert(
        doc,
        mate_node(
            in_part(c0, CapEnd::End),
            in_part(c1, CapEnd::Start),
            ContactClass::Rest,
            frame(seat, axis),
        ),
    );
    let id = doc.id();
    (store.insert(doc, Tol::witness()), id, mate)
}

/// P3: the disclosed bound, built. An instance that CARRIES a
/// declaration is a product of at least two solids (a mate is between
/// two instances), and the pair boolean refuses a multi-solid operand
/// — at every seat: penetrating, resting, gapped. So no declaration
/// row reaches a boolean today, and the boolean's own `contacts` are
/// its `Declare` input's pairs, which no mate authored.
#[test]
fn p3_no_carried_declaration_reaches_a_boolean_operand() {
    for (label, seat) in [("pen", 0.5), ("rest", 1.0), ("gap", 1.5)] {
        let mut store = StubStore::default();
        let (inner, _, _) = stand(
            &mut store,
            &format!("r2-p3-{label}"),
            [0.0, 0.0, seat],
            [0.0, 0.0, 1.0],
        );
        let cube = store.insert(
            block_part(&format!("r2-p3-{label}-cube"), 1.0, 1.0, 1.0),
            Tol::witness(),
        );
        let doc = ProfileDoc::empty(
            DocumentId::derive(&format!("r2-p3-{label}-outer")),
            Tol::witness(),
        );
        let (doc, i) = insert(doc, Node::instantiate_part(inner));
        let (doc, c) = insert(doc, Node::instantiate_part(cube));
        let doc = place(doc, c, [50.0, 0.0, 0.0]);
        let (doc, u) = insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a: i,
                b: c,
                declare: None,
            },
        );
        let ev = run(&doc, &opts(store));
        let outcome = format!("{:?}", ev.result(u));
        println!("P3 {label}: {}", outcome.chars().take(200).collect::<String>());
        assert!(
            outcome.contains("Failed") && outcome.contains("not one solid"),
            "the boolean refuses the two-solid instance: {outcome}"
        );
        let result = assemble(&doc, &ev, Tol::witness());
        assert!(
            matches!(&result, Err(AssemblyError::Product(_))),
            "and the gate refuses on the root: {result:?}"
        );
    }
}

/// P4: a carried DECLINE — the grazing pair the census cannot decide
/// — reaches the frontier arm under its own name (deviation 4).
#[test]
fn p4_a_carried_decline_reaches_uncertified() {
    let mut store = StubStore::default();
    let (graze, graze_id, graze_mate) =
        stand(&mut store, "r2-p4-graze", [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]);
    let inner = store.docs[&graze_id].clone();
    let inner_result = assemble(&inner, &run(&inner, &opts(store.clone())), Tol::witness());
    assert!(
        matches!(&inner_result, Err(AssemblyError::Uncertified { .. })),
        "the inner alone is the frontier: {inner_result:?}"
    );
    let doc = ProfileDoc::empty(DocumentId::derive("r2-p4-outer"), Tol::witness());
    let (doc, i) = insert(doc, Node::instantiate_part(graze));
    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    println!("P4: {}", result.as_ref().err().map(ToString::to_string).unwrap_or_default());
    assert!(
        matches!(&result, Err(AssemblyError::Uncertified { .. })),
        "head: the carried decline is the frontier: {result:?}"
    );
    let findings = findings_of(&result);
    assert!(findings.iter().all(|f| matches!(
        &f.attribution,
        Attribution::Carried { through, of, relation: CarriedRelation::Declined, declaration, .. }
            if *through == i && *of == graze_id && declaration.mate == graze_mate
    )), "{findings:?}");
}

/// P5: an ANGULAR contradiction — the a-frame axis tilted — carries
/// no `Fit { gap }` steer, so its message reads as prose and a Python
/// `carried_refuted` row is writable today.
#[test]
fn p5_an_angular_carried_refutation_reads_as_prose() {
    let mut store = StubStore::default();
    let (tilted, tilted_id, _) = stand(
        &mut store,
        "r2-p5-tilted",
        [0.0, 0.0, 1.0],
        [0.0, 0.5, 0.866_025_403_784_438_6],
    );
    let doc = ProfileDoc::empty(DocumentId::derive("r2-p5-outer"), Tol::witness());
    let (doc, _) = insert(doc, Node::instantiate_part(tilted));
    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    let text = result.as_ref().err().map(ToString::to_string).unwrap_or_default();
    println!("P5: {text}");
    let findings = findings_of(&result);
    assert!(
        findings.iter().any(|f| matches!(
            &f.attribution,
            Attribution::Carried { of, relation: CarriedRelation::Refuted, .. } if *of == tilted_id
        )),
        "{findings:?}"
    );
    assert!(
        !text.contains(" { "),
        "no struct fingerprint, so `reads_as_prose` holds and `typed_err` does not panic"
    );
    // The penetrating seat, by contrast, carries the steer.
    let mut store = StubStore::default();
    let (pen, _, _) = stand(&mut store, "r2-p5-pen", [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let doc = ProfileDoc::empty(DocumentId::derive("r2-p5-pen-outer"), Tol::witness());
    let (doc, _) = insert(doc, Node::instantiate_part(pen));
    let ev = run(&doc, &opts(store));
    let text = assemble(&doc, &ev, Tol::witness())
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default();
    assert!(text.contains(" { "), "the offset contradiction carries `Fit {{ gap }}`: {text}");
}
