//! ASM-2A acceptance — `InstantiatePart`, single-solid parts.
//!
//! The kernel-layer half of the eight spec rows: everything reachable
//! with a STUB resolver, which is the substrate D-3 promises ("kernel
//! tests use a stub resolver"). The document-layer half — a real
//! `Workspace` on disk, the pin gate and the ε seam observed
//! end-to-end, and D9 across two fresh processes — lives in
//! `pncad`'s own suite, because the store is that layer's.
//!
//! Undo note, as in `asm_roots`: this layer has no undo STACK. `apply`
//! is pure and undo is keeping prior values, so "undo restores" is
//! checked the way the layer offers it — the pre-edit document, held
//! across the edit, is unchanged.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    CancelToken, CarriedIn, ContentPin, DocEdit, DocRef, DocumentId, EditError, EvalOptions,
    Evaluation, Frame, Node, NodeErrorKind, NodeResult, PartFault, PartResolver, PersistError,
    ProductErrorKind, ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg,
    SnapshotError, StableName, content_pin, evaluate, load, product, product_named, save,
};
use fixture::{ang, insert, len, on_frame, scl, square, step};
use geom_core::Tol;

// ---- The stub store ----

/// A resolver over an in-memory map, verifying the pin exactly as the
/// document layer's does — the point of the stub is to have no FILES,
/// not to have no gate.
#[derive(Debug, Default)]
struct StubStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
    /// Documents this store hands back with a deliberately WRONG
    /// tolerance classification, standing in for a bank document whose
    /// recorded ε does not reconcile.
    eps_seam: BTreeMap<DocumentId, ()>,
}

impl StubStore {
    fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        if self.eps_seam.contains_key(&doc_ref.id) {
            return Err(fail(
                ResolveFault::EpsilonSeam,
                "the referenced document records a different tolerance",
            ));
        }
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        let found = content_pin(doc, Tol::witness()).expect("the pin computes");
        if found != doc_ref.pin {
            return Err(fail(ResolveFault::PinMismatch, "the pin does not hold"));
        }
        Ok(doc.clone())
    }
}

fn with_resolver(store: StubStore) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    }
}

fn run(doc: &editor_core::ProfileDoc, opts: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), opts, Tol::witness())
}

/// A one-solid part document: a `side`-wide square extruded 1 tall,
/// centered at `cx`.
fn part(label: &str, cx: f64, side: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, side / 2.0)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    doc
}

/// A part whose product root is a BOOLEAN: a 3x3x0.8 plate with a
/// square boss sketched at z = 0.3 and extruded 1.0, poking out of the
/// top (the `corpus::boss` shape, squared). Strictly interior in x/y,
/// so no face of the boss is coincident with one of the plate's — the
/// union is a genuine one-solid result with nothing declared.
fn boolean_part(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, plate_p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)]],
    );
    let (doc, plate) = insert(
        doc,
        Node::Extrude {
            profile: plate_p,
            distance: len(0.8),
        },
    );
    let (doc, boss_p) = on_frame(
        doc,
        [0.0, 0.0, 0.3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(1.2, 1.7, 0.35)],
    );
    let (doc, boss) = insert(
        doc,
        Node::Extrude {
            profile: boss_p,
            distance: len(1.0),
        },
    );
    let (doc, _) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: plate,
            b: boss,
            declare: None,
        },
    );
    doc
}

/// Two disjoint blocks in one document — a TWO-solid product: what
/// row 5d refused before ASM-2B lifted that door, and instantiates now.
fn two_solid_part(label: &str) -> ProfileDoc {
    let doc = part(label, 0.0, 1.0);
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(10.0, 0.0, 0.5)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    doc
}

/// An assembly document instantiating `refs`, in order, each a root.
fn assembly(label: &str, refs: &[DocRef]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for r in refs {
        let (next, id) = insert(doc, Node::instantiate_part(*r));
        doc = next;
        ids.push(id);
    }
    (doc, ids)
}

fn volume(body: &topo::Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness())
        .expect("mass properties")
        .volume
}

fn part_fault(ev: &Evaluation<f64>, node: RecipeNodeId) -> PartFault {
    match ev.result(node) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Part { fault, .. } => fault.clone(),
            other => panic!("expected a Part refusal, got {other:?}"),
        },
        other => panic!("expected a failed node, got {other:?}"),
    }
}

/// The least x over a body's vertices — the position probe rows 1 and
/// 4 need. Exact (no arithmetic beyond the comparison), so a rigid
/// translation moves it by exactly the translation.
fn min_x(body: &topo::Body<f64>) -> f64 {
    body.vertices()
        .filter_map(|(_, v)| body.get_point(v.point))
        .map(|p| p.x)
        .fold(f64::INFINITY, f64::min)
}

// ---- Row 1: two instances at different frames ----

/// Row 1 (kernel half) — an assembly of TWO instances of one part at
/// different frames evaluates to a 2-solid product whose volume is
/// exactly 2× the part's, in root order.
#[test]
fn row1_two_instances_gather_into_a_two_solid_product() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-r1-part", 0.0, 1.0), Tol::witness());
    let opts = with_resolver(store);

    let (doc, ids) = assembly("asm2a-r1-asm", &[doc_ref, doc_ref]);
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[1],
            frame: Frame::translation([5.0, 0.0, 0.0]),
        },
    );

    let ev = run(&doc, &opts);
    let body = product(&doc, &ev, Tol::witness()).expect("the product gathers");
    assert_eq!(body.solids().count(), 2, "one solid per instance");

    // The part's own product, for the volume comparison.
    let part_doc = part("asm2a-r1-part", 0.0, 1.0);
    let part_ev = run(&part_doc, &EvalOptions::default());
    let part_body = product(&part_doc, &part_ev, Tol::witness()).expect("the part's product");
    assert_eq!(
        volume(&body).to_bits(),
        (2.0 * volume(&part_body)).to_bits(),
        "volume is bit-exactly twice the part's"
    );

    // Solid order IS root order: the first root's solid sits at x = 0,
    // the second's at x = 5.
    let ev2 = run(&doc, &opts);
    let body2 = product(&doc, &ev2, Tol::witness()).expect("the product gathers again");
    assert_eq!(
        volume(&body).to_bits(),
        volume(&body2).to_bits(),
        "two evaluations agree bit for bit (D9)"
    );
    let placed_x = |node: RecipeNodeId| match ev.value(node).map(|v| &v.payload) {
        Some(editor_core::ValuePayload::Body(b)) => min_x(b),
        other => panic!("an instance's value is a body, got {other:?}"),
    };
    assert!((placed_x(ids[0]) + 0.5).abs() < 1e-12);
    assert!((placed_x(ids[1]) - 4.5).abs() < 1e-12);
}

// ---- Row 2: the memo ----

/// Row 2 — the referenced document evaluates ONCE for two instances.
/// A counter, not a timing claim.
#[test]
fn row2_one_part_two_instances_one_evaluation() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-r2-part", 0.0, 1.0), Tol::witness());
    let opts = with_resolver(store);
    let (doc, ids) = assembly("asm2a-r2-asm", &[doc_ref, doc_ref, doc_ref]);
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[2],
            frame: Frame::translation([9.0, 0.0, 0.0]),
        },
    );

    let ev = run(&doc, &opts);
    assert_eq!(
        ev.part_evaluations, 1,
        "three instances of one part evaluate it once"
    );
    assert_eq!(
        product(&doc, &ev, Tol::witness())
            .expect("gathers")
            .solids()
            .count(),
        3,
        "sharing the evaluation does not share the SOLIDS"
    );

    // Two DISTINCT parts are two evaluations — the counter counts
    // references, not calls.
    let mut store = StubStore::default();
    let a = store.insert(part("asm2a-r2-a", 0.0, 1.0), Tol::witness());
    let b = store.insert(part("asm2a-r2-b", 0.0, 2.0), Tol::witness());
    let opts = with_resolver(store);
    let (doc, ids) = assembly("asm2a-r2-two", &[a, b]);
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[1],
            frame: Frame::translation([20.0, 0.0, 0.0]),
        },
    );
    assert_eq!(run(&doc, &opts).part_evaluations, 2);
}

/// **The instantiate node's verdict log is its own op's, and it is the
/// same log whichever instance actually ran the part.** The op
/// evaluates the referenced document INSIDE its own verdict bracket
/// (the part cache's miss path), so the first instance's bracket has a
/// whole nested evaluation opening and closing brackets under it; the
/// second instance is a cache hit and runs none. Both frames must come
/// back holding exactly the instantiate op's own decisions — the
/// placement and the geometric validation of the placed body — which
/// is what "the verdicts a node's op produced" means when the op's
/// work includes another document: that document's decisions are
/// recorded on ITS nodes, in the nested evaluation, and never in the
/// instantiator's frame.
///
/// A log that is empty at the first instance and populated at the
/// second is the signature of a nested bracket destroying its parent's
/// frame; one that is LARGER at the first is the miss path's work —
/// the part's own decisions — landing on whichever instance ran it.
#[test]
fn the_instantiate_node_records_its_own_decisions_whichever_instance_ran_the_part() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-vlog-part", 0.0, 1.0), Tol::witness());
    let opts = with_resolver(store);
    let (doc, ids) = assembly("asm2a-vlog-asm", &[doc_ref, doc_ref]);
    // Both instances placed, so both ops do the same work on the part
    // they share: an identity placement is applied by doing nothing,
    // and an op that does nothing decides nothing.
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[0],
            frame: Frame::translation([0.0, 9.0, 0.0]),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[1],
            frame: Frame::translation([9.0, 0.0, 0.0]),
        },
    );
    let ev = run(&doc, &opts);
    assert_eq!(ev.part_evaluations, 1, "the second instance is a cache hit");
    let log = |id: RecipeNodeId| {
        ev.result(id)
            .and_then(NodeResult::value)
            .expect("the instance evaluates")
            .verdicts
            .clone()
    };
    let (first, second) = (log(ids[0]), log(ids[1]));
    // The part's own decisions are the part document's, not the
    // instantiator's: evaluated directly, the part records far more
    // than the op that places it does.
    let part_doc = part("asm2a-vlog-part", 0.0, 1.0);
    let direct = run(&part_doc, &EvalOptions::default());
    let direct_total: usize = direct
        .nodes
        .values()
        .filter_map(NodeResult::value)
        .map(|v| v.verdicts.len())
        .sum();
    assert!(
        !first.is_empty(),
        "the instance that ran the part records its own op's decisions: {} verdicts at the \
         first instance, {} at the second, {direct_total} on the part evaluated directly",
        first.len(),
        second.len()
    );
    assert_eq!(
        first,
        second,
        "the two instances' ops make the same decisions: {} vs {} verdicts",
        first.len(),
        second.len()
    );
    // The counts are literals on purpose: a row that only compares the
    // two instances passes when both lose the same decisions. 466 is
    // the placing op's own log on this part (placement + validation of
    // the placed body); 726 is the part's, on its own nodes — its
    // profile's pre-pass on the Profile node's log, decided once (the
    // pinned lift reuses the pre-pass's validated form).
    assert_eq!(first.len(), 466, "the instantiate op's own decisions");
    assert_eq!(direct_total, 726, "the part's decisions on its own nodes");
}

// ---- Row 3: instance-qualified naming ----

fn instance_names(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<StableName> {
    ev.value(node)
        .expect("the instance evaluated")
        .name_table
        .iter()
        .map(|(n, _)| n.clone())
        .collect()
}

/// Row 3 — both copies carry instance-qualified names; every name is
/// distinct across the two; each resolves to ITS OWN copy's geometry
/// (the cross-wiring probe: moving one instance moves only its own
/// names' entities).
#[test]
fn row3_instance_qualified_names_are_distinct_and_resolve_to_their_own_copy() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-r3-part", 0.0, 1.0), Tol::witness());
    let opts = with_resolver(store);
    let (doc, ids) = assembly("asm2a-r3-asm", &[doc_ref, doc_ref]);
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[1],
            frame: Frame::translation([5.0, 0.0, 0.0]),
        },
    );
    let ev = run(&doc, &opts);

    let a = instance_names(&ev, ids[0]);
    let b = instance_names(&ev, ids[1]);
    assert!(!a.is_empty(), "an instance has names at all");
    assert_eq!(a.len(), b.len(), "the two instances name the same entities");
    for name in &a {
        assert!(!b.contains(name), "no name is shared between instances");
    }
    // Every non-body name is an `InPart` wrapper minted AT the instance
    // node; the body name is the instance's own output.
    for (names, node) in [(&a, ids[0]), (&b, ids[1])] {
        for name in names {
            assert_eq!(name.node, node, "names are minted at the instance");
            match &name.path[..] {
                [RoleSeg::InPart { of }] => {
                    assert_eq!(of.kind, name.kind, "the wrapper preserves the kind");
                }
                [RoleSeg::OutputBody] => {
                    assert_eq!(name.kind, editor_core::EntityKind::Body);
                }
                other => panic!("unexpected instance role path {other:?}"),
            }
        }
    }

    // Cross-wiring probe: the SAME part-local vertex name, wrapped
    // under each instance, must land in that instance's OWN copy. If
    // the tables were cross-wired the two positions would agree.
    let a_vertex = a
        .iter()
        .find(|n| n.kind == editor_core::EntityKind::Vertex)
        .expect("a vertex name");
    let RoleSeg::InPart { of } = &a_vertex.path[0] else {
        panic!("an instance's vertex name wraps a part-local one");
    };
    let under = |node: RecipeNodeId| StableName {
        kind: a_vertex.kind,
        node,
        path: vec![RoleSeg::InPart { of: of.clone() }],
    };
    let pa = editor_core::vertex_position(&ev, ids[0], &under(ids[0])).expect("resolves");
    let pb = editor_core::vertex_position(&ev, ids[1], &under(ids[1])).expect("resolves");
    assert!(
        (pb.x - pa.x - 5.0).abs() < 1e-12,
        "one part-local name lands 5 apart under the two instances"
    );
    // And a name minted at one instance does not resolve in the other's
    // value: identity is per instance, not per part.
    assert!(
        editor_core::vertex_position(&ev, ids[1], &under(ids[0])).is_err(),
        "instance 0's name is not instance 1's"
    );
}

/// Row 3 (persistence half) — instance-qualified names round-trip
/// save/load: an appearance attribute keyed by one survives the wire.
#[test]
fn row3_instance_qualified_names_round_trip_persistence() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-r3p-part", 0.0, 1.0), Tol::witness());
    let opts = with_resolver(store);
    let (doc, ids) = assembly("asm2a-r3p-asm", &[doc_ref]);
    let ev = run(&doc, &opts);
    let name = instance_names(&ev, ids[0])
        .into_iter()
        .find(|n| n.kind == editor_core::EntityKind::Face)
        .expect("a face name");

    let (doc, _) = step(
        doc,
        DocEdit::SetAppearance {
            name: name.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8 {
                r: 1,
                g: 2,
                b: 3,
                a: 255,
            }),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("loads").doc;
    assert!(
        loaded.appearance().keys().any(|k| *k == name),
        "an instance-qualified name survives the wire verbatim"
    );
    assert!(loaded.bit_eq(&doc), "the whole document round-trips");
}

// ---- Row 4: SetPlacement ----

/// Row 4 — the placement moves the copy without changing its volume;
/// the pre-edit document is untouched (undo = keeping the prior value);
/// an improper frame refuses typed NAMING the R4 prerequisite; a
/// non-instance target refuses typed.
#[test]
fn row4_set_placement_moves_undoes_and_refuses() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-r4-part", 0.0, 1.0), Tol::witness());
    let opts = with_resolver(store);
    let (doc, ids) = assembly("asm2a-r4-asm", &[doc_ref]);

    let before = run(&doc, &opts);
    let before_body = product(&doc, &before, Tol::witness()).expect("gathers");
    assert!(doc.placement(ids[0]).is_identity_bits());

    let (moved, _) = step(
        doc.clone(),
        DocEdit::SetPlacement {
            node: ids[0],
            frame: Frame::translation([7.0, 0.0, 0.0]),
        },
    );
    let after = run(&moved, &opts);
    let after_body = product(&moved, &after, Tol::witness()).expect("gathers");
    assert_eq!(
        volume(&before_body).to_bits(),
        volume(&after_body).to_bits(),
        "a rigid placement preserves volume bit for bit"
    );
    assert!(
        (min_x(&after_body) - min_x(&before_body) - 7.0).abs() < 1e-9,
        "the copy moved by the frame's translation"
    );

    // Undo, as this layer offers it: the prior document is unchanged.
    assert!(doc.placement(ids[0]).is_identity_bits(), "undo restores");
    assert!(doc.placements().is_empty());

    // An improper frame (a mirror) refuses, naming R4's prerequisite.
    let mirror = Frame {
        columns: [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0, 0.0, 0.0],
    };
    match editor_core::apply(
        &doc,
        &DocEdit::SetPlacement {
            node: ids[0],
            frame: mirror,
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(
            e @ EditError::ImproperPlacement {
                node, determinant, ..
            },
        ) => {
            assert_eq!(node, ids[0]);
            assert!(determinant < 0.0);
            let rendered = e.to_string();
            assert!(
                rendered.contains("admitted only behind the equivariance audit"),
                "the refusal names the prerequisite: {rendered}"
            );
        }
        other => panic!("an improper frame must refuse, got {other:?}"),
    }

    // A non-instance target refuses typed.
    let part_doc = part("asm2a-r4-nontarget", 0.0, 1.0);
    let target = part_doc.order()[0];
    match editor_core::apply(
        &part_doc,
        &DocEdit::SetPlacement {
            node: target,
            frame: Frame::translation([1.0, 0.0, 0.0]),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::PlacementOnNonInstance { node }) => assert_eq!(node, target),
        other => panic!("a non-instance target must refuse, got {other:?}"),
    }
}

/// Row 4 (registry hygiene) — deleting an instance takes its placement
/// with it; the save validator would refuse a stranded row.
#[test]
fn row4_deleting_an_instance_clears_its_placement() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-r4d-part", 0.0, 1.0), Tol::witness());
    let _ = with_resolver(store);
    let (doc, ids) = assembly("asm2a-r4d-asm", &[doc_ref, doc_ref]);
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[0],
            frame: Frame::translation([3.0, 0.0, 0.0]),
        },
    );
    assert_eq!(doc.placements().len(), 1);
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: ids[0] });
    assert!(
        doc.placements().is_empty(),
        "the placement dies with its instance"
    );
}

// ---- Row 5: the refusals, each its own row ----

/// Row 5a — no resolver: the seam cannot be crossed, and the node says
/// so rather than pretending the part is empty.
#[test]
fn row5a_no_resolver_refuses_typed() {
    let doc_ref = DocRef {
        id: DocumentId::derive("asm2a-r5a"),
        pin: ContentPin([0u8; 32]),
    };
    let (doc, ids) = assembly("asm2a-r5a-asm", &[doc_ref]);
    let ev = run(&doc, &EvalOptions::default());
    assert_eq!(part_fault(&ev, ids[0]), PartFault::NoResolver);
}

/// Row 5b — A4's pin gate, observed END TO END: the part is edited
/// after the reference was pinned, so the stale pin refuses at
/// evaluation. References are never retargeted silently.
#[test]
fn row5b_stale_pin_refuses_at_evaluate() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-r5b-part", 0.0, 1.0), Tol::witness());
    // The part document is edited AFTER the reference was pinned.
    let edited = part("asm2a-r5b-part", 0.0, 2.0);
    assert_ne!(
        content_pin(&edited, Tol::witness()).expect("pins"),
        doc_ref.pin,
        "the edit really moved the pin"
    );
    store.docs.insert(edited.id(), edited);
    let opts = with_resolver(store);

    let (doc, ids) = assembly("asm2a-r5b-asm", &[doc_ref]);
    let ev = run(&doc, &opts);
    assert!(
        matches!(
            part_fault(&ev, ids[0]),
            PartFault::Unresolved {
                fault: ResolveFault::PinMismatch,
                ..
            }
        ),
        "a stale pin refuses at the seam"
    );
}

/// Row 5c — A2's ε seam: a bank document whose recorded ε disagrees
/// refuses at the seam, classified as such.
#[test]
fn row5c_epsilon_seam_refuses_typed() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-r5c-part", 0.0, 1.0), Tol::witness());
    store.eps_seam.insert(doc_ref.id, ());
    let opts = with_resolver(store);
    let (doc, ids) = assembly("asm2a-r5c-asm", &[doc_ref]);
    let ev = run(&doc, &opts);
    assert!(matches!(
        part_fault(&ev, ids[0]),
        PartFault::Unresolved {
            fault: ResolveFault::EpsilonSeam,
            ..
        }
    ));
}

/// Row 5d, FLIPPED at **ASM-2B** (acceptance row 1) — a multi-solid
/// referenced product used to refuse here, naming ASM-2b as its flip
/// condition. That door is open: the same fixture now SUCCEEDS, its two
/// solids arriving as two solids of the instance's placed body and of
/// the assembly's product. The row is kept rather than deleted because
/// it is the one that pins WHICH direction this door faces; reverting
/// the lift reds it.
#[test]
fn row5d_multi_solid_part_instantiates_since_asm_2b() {
    let mut store = StubStore::default();
    let part_doc = two_solid_part("asm2a-r5d-part");
    let doc_ref = store.insert(part_doc.clone(), Tol::witness());
    let opts = with_resolver(store);
    let (doc, ids) = assembly("asm2a-r5d-asm", &[doc_ref]);
    let ev = run(&doc, &opts);
    match ev.result(ids[0]) {
        Some(NodeResult::Ok(value)) => match &value.payload {
            editor_core::ValuePayload::Body(b) => {
                assert_eq!(b.solids().count(), 2, "both of the part's solids arrive");
            }
            other => panic!("an instance's value is a body, got {other:?}"),
        },
        other => panic!("a multi-solid part instantiates since ASM-2B, got {other:?}"),
    }
    let body = product(&doc, &ev, Tol::witness()).expect("the product gathers");
    assert_eq!(body.solids().count(), 2);

    // The part's own product, for the volume comparison: one instance
    // at identity is the part, bit for bit.
    let part_ev = run(&part_doc, &EvalOptions::default());
    let part_body = product(&part_doc, &part_ev, Tol::witness()).expect("the part's product");
    assert_eq!(
        volume(&body).to_bits(),
        volume(&part_body).to_bits(),
        "one instance at identity has the part's volume, bit for bit"
    );
}

// ---- Row 6: pin semantics ----

/// Row 6 — the assembly's own pin moves on `SetPlacement` and on a
/// pin-bump of a reference, and an untouched-content re-save leaves it
/// fixed.
#[test]
fn row6_the_assembly_pin_moves_exactly_when_its_content_does() {
    let mut store = StubStore::default();
    let a = store.insert(part("asm2a-r6-part", 0.0, 1.0), Tol::witness());
    let (doc, ids) = assembly("asm2a-r6-asm", &[a]);
    let pin0 = content_pin(&doc, Tol::witness()).expect("pins");

    // A re-save of untouched content leaves the pin fixed.
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let reloaded = load(&text, Tol::witness()).expect("loads").doc;
    assert_eq!(content_pin(&reloaded, Tol::witness()).expect("pins"), pin0);
    let again = save(&reloaded, &[], Tol::witness()).expect("saves again");
    assert_eq!(text, again, "saves are byte-stable");

    // A placement moves it.
    let (moved, _) = step(
        doc.clone(),
        DocEdit::SetPlacement {
            node: ids[0],
            frame: Frame::translation([1.0, 0.0, 0.0]),
        },
    );
    assert_ne!(content_pin(&moved, Tol::witness()).expect("pins"), pin0);

    // A pin-bump of the reference moves it too (the reference IS
    // content: Cargo.lock semantics).
    let bumped = DocRef {
        pin: content_pin(&part("asm2a-r6-part", 0.0, 2.0), Tol::witness()).expect("pins"),
        ..a
    };
    let (rebound, _) = assembly("asm2a-r6-asm", &[bumped]);
    assert_ne!(content_pin(&rebound, Tol::witness()).expect("pins"), pin0);
}

/// Row 6 (memo half) — a placement edit MOVES the node's content key,
/// so the memo does not hand back the old placement's body.
#[test]
fn row6_placement_is_part_of_the_content_key() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-r6k-part", 0.0, 1.0), Tol::witness());
    let opts = with_resolver(store);
    let (doc, ids) = assembly("asm2a-r6k-asm", &[doc_ref]);
    let first = run(&doc, &opts);

    let (moved, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[0],
            frame: Frame::translation([4.0, 0.0, 0.0]),
        },
    );
    let second = evaluate::<f64>(
        &moved,
        Some(&first),
        &CancelToken::new(),
        &opts,
        Tol::witness(),
    );
    assert_eq!(second.reused, 0, "the placement edit invalidates the memo");
    let body = product(&moved, &second, Tol::witness()).expect("gathers");
    assert!((min_x(&body) - 3.5).abs() < 1e-12);

    // And a re-evaluation with NO edit reuses — and asks the seam
    // nothing at all. That is the memo's contract and not a shortcut
    // in it (DI2): the memo is a pure function of the document, and
    // for an instantiate node the pin IS the content, so the served
    // value is exactly what this document pins. Store freshness is the
    // mounting session's question.
    let third = evaluate::<f64>(
        &moved,
        Some(&second),
        &CancelToken::new(),
        &opts,
        Tol::witness(),
    );
    assert_eq!(third.reused, 1, "an unchanged instance is a memo hit");
    assert_eq!(
        third.part_evaluations, 0,
        "a memo hit crosses no document seam"
    );
}

// ---- Row 7: persistence ----

/// Row 7 — the instantiate node and its placement round-trip, and two
/// blesses of one document are byte-identical.
#[test]
fn row7_instantiate_and_placement_round_trip() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-r7-part", 0.0, 1.0), Tol::witness());
    let (doc, ids) = assembly("asm2a-r7-asm", &[doc_ref, doc_ref]);
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[1],
            frame: Frame::rotate_then_translate(
                [0.0, 0.0, 1.0],
                0.25,
                [2.0, 3.0, 0.0],
                fixture::band(),
            )
            .expect("a literal axis has a definite direction"),
        },
    );

    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("loads").doc;
    assert!(loaded.bit_eq(&doc), "the placement round-trips bit for bit");
    assert_eq!(loaded.placements().len(), 1);
    assert!(
        loaded.placement(ids[1]).bit_eq(&doc.placement(ids[1])),
        "the frame's bits survive"
    );
    assert!(
        matches!(loaded.node(ids[0]), Some(Node::InstantiatePart { doc_ref: r, .. }) if *r == doc_ref)
    );
    assert_eq!(
        save(&loaded, &[], Tol::witness()).expect("saves"),
        text,
        "byte-stable"
    );
}

/// Row 7 (validator half) — a file whose placement names a
/// non-instance, or carries an improper frame, refuses at the save
/// door: the file can hold no placement state the edit doors could not
/// have produced.
#[test]
fn row7_the_validator_refuses_placement_states_the_edits_cannot_produce() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-r7v-part", 0.0, 1.0), Tol::witness());
    let (doc, ids) = assembly("asm2a-r7v-asm", &[doc_ref]);
    // A second, NON-instance node, so the corrupted key below names a
    // live node and the diagnosis is the placement rule rather than an
    // id-range fault.
    let (doc, other) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[0],
            frame: Frame::translation([1.0, 0.0, 0.0]),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");

    // Re-key the placement row onto the profile node — a state no edit
    // door can produce.
    let corrupt = text.replace(
        &format!("\"placements\": {{\n      \"{}\"", ids[0].0),
        &format!("\"placements\": {{\n      \"{}\"", other.0),
    );
    assert_ne!(corrupt, text, "the corruption really landed");
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::PlacementSite { node })) => {
            assert_eq!(node, other);
        }
        other => panic!("a stranded placement must refuse, got {other:?}"),
    }
}

// ---- The gather's own naming door ----

/// `product_named` is ONE implementation with two doors: the body it
/// returns is the body `product` returns, and its table covers the
/// aggregate's faces, edges and vertices.
#[test]
fn the_named_gather_agrees_with_the_plain_one() {
    let doc = part("asm2a-gather", 0.0, 1.0);
    let ev = run(&doc, &EvalOptions::default());
    let plain = product(&doc, &ev, Tol::witness()).expect("gathers");
    let (named, table) = product_named(&doc, &ev, Tol::witness()).expect("gathers with names");
    assert_eq!(volume(&plain).to_bits(), volume(&named).to_bits());
    let entities = named.faces().count() + named.edges().count() + named.vertices().count();
    assert_eq!(
        table.len(),
        entities,
        "every product entity but the body itself is named"
    );
}

// ---- Review fixes (R1): the seam's diagnosis, and its guards ----

/// The last level of a failure's carried chain — what the author has
/// to be told, however many documents down it lies — as its refusal
/// and the line its node's own tree draws for it.
fn root_cause(kind: &NodeErrorKind) -> Option<(&NodeErrorKind, String)> {
    kind.carried_chain()
        .last()
        .map(|level| (level.refusal.kind(), level.line()))
}

/// The failure `node` raised.
fn failure(ev: &Evaluation<f64>, node: RecipeNodeId) -> &NodeErrorKind {
    match ev.result(node) {
        Some(NodeResult::Failed(e)) => &e.kind,
        other => panic!("expected a failed node, got {other:?}"),
    }
}

/// A deliberately MISBEHAVING resolver: it answers each of a pair of
/// references with a document instantiating the OTHER, pins ignored.
///
/// A4 makes this unconstructible through an honest store — a document
/// would have to contain its own hash — so a resolver is the only thing
/// that can produce it, and this is that resolver. The reviewer's
/// cyclic probe, in-suite.
#[derive(Debug)]
struct CyclicStore {
    a: DocRef,
    b: DocRef,
}

impl PartResolver for CyclicStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        // Whichever is asked for, hand back a document that instantiates
        // the other one.
        let (here, there) = if *doc_ref == self.a {
            (self.a, self.b)
        } else {
            (self.b, self.a)
        };
        let doc = ProfileDoc::empty(here.id, Tol::witness());
        let (doc, _) = insert(doc, Node::instantiate_part(there));
        Ok(doc)
    }
}

/// MAJOR-1 + item 6 — a reference CYCLE refuses typed, NAMES the loop,
/// and the naming survives the whole chain of documents back to the
/// caller (who can reach no evaluation but this fault).
#[test]
fn r1_a_reference_cycle_refuses_naming_the_loop() {
    let a = DocRef {
        id: DocumentId::derive("asm2a-cycle-a"),
        pin: ContentPin([1u8; 32]),
    };
    let b = DocRef {
        id: DocumentId::derive("asm2a-cycle-b"),
        pin: ContentPin([2u8; 32]),
    };
    let opts = EvalOptions {
        resolver: Some(Arc::new(CyclicStore { a, b })),
        ..EvalOptions::default()
    };
    let (doc, ids) = assembly("asm2a-cycle-asm", &[a]);

    // Terminates at the FIRST revisit — the guard is structural, not a
    // depth counter waiting 1024 levels out.
    let ev = run(&doc, &opts);
    let (cause, rendered) =
        root_cause(failure(&ev, ids[0])).expect("the cycle is reached through a failed root");
    match cause {
        NodeErrorKind::Part {
            fault: PartFault::ReferenceCycle { cycle },
            ..
        } => {
            assert_eq!(
                cycle,
                &vec![a, b, a],
                "the loop is carried first repeated reference through last"
            );
        }
        other => panic!("expected a named cycle, got {other:?}"),
    }
    // The DIAGNOSIS reaches the caller: the chain's last line names the
    // loop, not an evaluation the caller cannot reach.
    assert!(
        rendered.contains("returns to a document it already entered"),
        "the top-level message names the cycle: {rendered}"
    );
    assert!(
        rendered.contains(&format!("{a}")) && rendered.contains(&format!("{b}")),
        "both documents of the loop are named: {rendered}"
    );
}

/// MAJOR-1 — the ORDINARY broken-part path: a part whose product root
/// failed reports WHICH root and WHY, instead of pointing the caller at
/// an `Evaluation` that died with the resolution.
#[test]
fn r1_a_broken_part_names_its_failing_root_and_cause() {
    // The part's own root instantiates a reference its store cannot
    // resolve: a typed cause, one document down.
    let missing = DocRef {
        id: DocumentId::derive("asm2a-broken-missing"),
        pin: ContentPin([7u8; 32]),
    };
    let mut store = StubStore::default();
    let broken = {
        let doc = ProfileDoc::empty(DocumentId::derive("asm2a-broken-part"), Tol::witness());
        let (doc, _) = insert(doc, Node::instantiate_part(missing));
        doc
    };
    let inner_root = broken.order()[0];
    let doc_ref = store.insert(broken, Tol::witness());
    let opts = with_resolver(store);

    let (doc, ids) = assembly("asm2a-broken-asm", &[doc_ref]);
    let ev = run(&doc, &opts);
    let fault = part_fault(&ev, ids[0]);
    match &fault {
        PartFault::PartRootFailed { node, refusal } => {
            assert_eq!(*node, inner_root, "the failing ROOT is named");
            assert!(
                matches!(
                    refusal.kind(),
                    NodeErrorKind::Part {
                        doc_ref,
                        fault: PartFault::Unresolved {
                            fault: ResolveFault::Unresolved,
                            ..
                        },
                    } if *doc_ref == missing
                ),
                "the root's refusal travels typed, with the reference it crossed: {refusal:?}"
            );
        }
        other => panic!("expected PartRootFailed, got {other:?}"),
    }
    let rendered = fault.to_string();
    assert!(
        !rendered.contains("Evaluation::node_error"),
        "the message never points at an object the caller cannot reach: {rendered}"
    );
    assert!(
        rendered.contains(&format!("node {:012x}", inner_root.0))
            && !rendered.contains("did not resolve"),
        "it names the root and points, never quoting the root's own refusal: {rendered}"
    );
    assert!(
        root_cause(failure(&ev, ids[0])).is_some_and(|(_, line)| line.contains("did not resolve")),
        "the reason is the carried line's"
    );
}

/// **A real chain three documents deep reads one level per document,
/// each in the document its node is numbered in.** The assembly
/// instantiates `p1`, whose root instantiates `p2`, whose root
/// instantiates `p3`, whose root, an extrude, refuses. The kernel's
/// chain names `p1`, `p2` and `p3` in order with each level's failed
/// root, and the last line is `p3`'s node error exactly as `p3`'s own
/// evaluation renders it.
#[test]
fn a_depth_three_chain_keeps_every_level_and_its_document() {
    let tol = Tol::witness();
    let mut store = StubStore::default();
    let p3 = ProfileDoc::empty(DocumentId::derive("asm2a-depth-p3"), tol);
    let (p3, profile) = on_frame(
        p3,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (p3, p3_root) = insert(
        p3,
        Node::Extrude {
            profile,
            distance: editor_core::Expr::div(len(1.0), scl(0.0)).unwrap(),
        },
    );
    let p3_own = match run(&p3, &EvalOptions::default()).result(p3_root) {
        Some(NodeResult::Failed(e)) => e.to_string(),
        other => panic!("p3's extrude refuses on its own: {other:?}"),
    };
    let r3 = store.insert(p3, tol);
    let wrapper = |label: &str, inner: DocRef| {
        insert(
            ProfileDoc::empty(DocumentId::derive(label), tol),
            Node::instantiate_part(inner),
        )
    };
    let (p2, p2_root) = wrapper("asm2a-depth-p2", r3);
    let r2 = store.insert(p2, tol);
    let (p1, p1_root) = wrapper("asm2a-depth-p1", r2);
    let r1 = store.insert(p1, tol);
    let (asm, instance) = wrapper("asm2a-depth-asm", r1);
    let ev = run(&asm, &with_resolver(store));
    let levels: Vec<_> = failure(&ev, instance)
        .carried_chain()
        .map(|level| (level.document, level.node, level.line()))
        .collect();
    assert_eq!(
        levels
            .iter()
            .map(|(document, node, _)| (*document, *node))
            .collect::<Vec<_>>(),
        vec![
            (CarriedIn::Part(&r1), p1_root),
            (CarriedIn::Part(&r2), p2_root),
            (CarriedIn::Part(&r3), p3_root),
        ],
        "one level per document, in order, each in the part it is numbered in"
    );
    assert_eq!(levels[2].2, p3_own, "the last line is p3's own rendering");
}

/// **A part whose root was poisoned carries the failure that poisoned
/// it.** The part's extrude refuses and its one root, a transform over
/// the extrude, never runs: the instance names both nodes, points at
/// the extrude, and carries the extrude's own refusal typed — the last
/// level of its chain is the extrude's line exactly as the part's own
/// evaluation draws it.
#[test]
fn a_poisoned_root_carries_the_failure_that_poisoned_it() {
    let tol = Tol::witness();
    let (part, extrude, moved) = poisoned_part("asm2a-poisoned-part");
    let own = own_line(&part, extrude, &EvalOptions::default());
    let mut store = StubStore::default();
    let part_ref = store.insert(part, tol);
    let (doc, ids) = assembly("asm2a-poisoned-asm", &[part_ref]);
    let ev = run(&doc, &with_resolver(store));

    let fault = part_fault(&ev, ids[0]);
    let PartFault::PartRootPoisoned {
        root,
        through,
        refusal,
    } = &fault
    else {
        panic!("expected PartRootPoisoned, got {fault:?}");
    };
    assert_eq!((*root, *through), (moved, extrude), "both nodes are named");
    assert!(
        matches!(refusal.kind(), NodeErrorKind::Expr { .. }),
        "the extrude's refusal travels typed: {refusal:?}"
    );
    let rendered = fault.to_string();
    assert!(
        rendered.contains(&format!("repair node {:012x}", extrude.0))
            && rendered.contains(&format!("node {:012x}", moved.0)),
        "the instance names the root and points at the failed node: {rendered}"
    );
    let levels: Vec<_> = failure(&ev, ids[0])
        .carried_chain()
        .map(|level| (level.document, level.node, level.line()))
        .collect();
    assert_eq!(
        levels,
        vec![(CarriedIn::Part(&part_ref), extrude, own.clone())],
        "the traceback ends at the failing node, drawn as the part draws it"
    );
    let refused = own
        .strip_prefix(&format!("node {:012x} failed: ", extrude.0))
        .expect("a node line opens with its node");
    assert!(
        !rendered.contains(refused),
        "the instance never quotes the refusal it carries: {rendered}"
    );
}

/// A transform over `input`: the root a part's move makes.
fn moved_over(doc: ProfileDoc, input: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::transform(
            input,
            editor_core::Step::Rigid {
                translation: [len(0.1), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    )
}

/// A part whose extrude refuses (its distance is 1/0) and whose one
/// root is a transform over it: the extrude, then the root.
fn poisoned_part(label: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let part = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (part, profile) = on_frame(
        part,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (part, extrude) = insert(
        part,
        Node::Extrude {
            profile,
            distance: editor_core::Expr::div(len(1.0), scl(0.0)).unwrap(),
        },
    );
    let (part, moved) = moved_over(part, extrude);
    assert_eq!(
        part.roots(),
        &[moved],
        "the transform is the part's one root"
    );
    (part, extrude, moved)
}

/// `node`'s own refusal line, as `doc`'s own evaluation draws it.
fn own_line(doc: &ProfileDoc, node: RecipeNodeId, opts: &EvalOptions) -> String {
    match run(doc, opts).result(node) {
        Some(NodeResult::Failed(e)) => e.to_string(),
        other => panic!("node {:012x} refuses on its own: {other:?}", node.0),
    }
}

/// **Two documents down, the chain still ends at the failing node.**
/// The bracket's root is a transform over an instance of the poisoned
/// part, so the bracket's root is poisoned through a failed INSTANCE,
/// whose own fault is the part's poisoned root in turn. The assembly's
/// instance carries both levels, typed, each drawn as its own document
/// draws it, and the last is the extrude's line.
#[test]
fn a_poisoned_root_two_documents_down_chains_to_the_failing_node() {
    let tol = Tol::witness();
    let mut store = StubStore::default();
    let (part, extrude, part_root) = poisoned_part("asm2a-poisoned-deep-part");
    let broken_line = own_line(&part, extrude, &EvalOptions::default());
    let part_ref = store.insert(part, tol);

    let bracket = ProfileDoc::empty(DocumentId::derive("asm2a-poisoned-deep-bracket"), tol);
    let (bracket, inner) = insert(bracket, Node::instantiate_part(part_ref));
    let (bracket, bracket_root) = moved_over(bracket, inner);
    let bracket_ref = store.insert(bracket.clone(), tol);

    let (doc, ids) = assembly("asm2a-poisoned-deep-asm", &[bracket_ref]);
    let opts = with_resolver(store);
    let ev = run(&doc, &opts);

    let fault = part_fault(&ev, ids[0]);
    let PartFault::PartRootPoisoned {
        root,
        through,
        refusal,
    } = &fault
    else {
        panic!("expected PartRootPoisoned, got {fault:?}");
    };
    assert_eq!(
        (*root, *through),
        (bracket_root, inner),
        "the bracket's root, poisoned through its failed instance"
    );
    assert!(
        matches!(refusal.kind(), NodeErrorKind::Part {
            fault: PartFault::PartRootPoisoned { root, through, .. }, ..
        } if (*root, *through) == (part_root, extrude)),
        "the carried refusal is the inner instance's own poisoned root: {refusal:?}"
    );
    let bracket_line = own_line(&bracket, inner, &opts);
    let levels: Vec<_> = failure(&ev, ids[0])
        .carried_chain()
        .map(|level| (level.document, level.node, level.line()))
        .collect();
    assert_eq!(
        levels,
        vec![
            (CarriedIn::Part(&bracket_ref), inner, bracket_line),
            (CarriedIn::Part(&part_ref), extrude, broken_line),
        ],
        "one level per document, ending at the failing node"
    );
}

/// The gather's OTHER refusals cross the seam as a CLASS beside their
/// sentence, so a consumer branches instead of substring-matching.
///
/// Two documents refuse the gather for two different reasons, and both
/// arrive as [`PartFault::PartProduct`]: the prose differs, which is
/// all a caller used to have, and the classes differ too — including
/// on `means_no_body`, the one reading every consumer of a gather
/// refusal draws. A failed or poisoned root is what does NOT come here
/// (it chains, typed, above).
#[test]
fn a_gather_refusal_crosses_as_its_class_beside_its_sentence() {
    let mut store = StubStore::default();

    // Nothing denotes a body: the one class that is an ABSENCE rather
    // than a fault.
    let empty = store.insert(
        ProfileDoc::empty(DocumentId::derive("asm2a-class-empty"), Tol::witness()),
        Tol::witness(),
    );
    // One body placed under two roots: a refusal, and NOT the absence
    // above.
    let twice = {
        let doc = part("asm2a-class-twice", 0.0, 1.0);
        let body = *doc.order().last().expect("the part has its extrude");
        let moved = |doc, dx| {
            insert(
                doc,
                Node::transform(
                    body,
                    editor_core::Step::Rigid {
                        translation: [len(dx), len(0.0), len(0.0)],
                        axis: [scl(0.0), scl(0.0), scl(1.0)],
                        angle: ang(0.0),
                    },
                ),
            )
            .0
        };
        moved(moved(doc, 2.0), 4.0)
    };
    let twice = store.insert(twice, Tol::witness());

    let opts = with_resolver(store);
    let (doc, ids) = assembly("asm2a-class-asm", &[empty, twice]);
    let ev = run(&doc, &opts);

    let kind_of = |fault: &PartFault| match fault {
        PartFault::PartProduct { kind, message } => {
            assert!(
                !message.starts_with("product"),
                "the gather's own sentence travels beside the class, without the gather's \
                 stage word: {message}"
            );
            *kind
        }
        other => panic!("expected PartProduct, got {other:?}"),
    };

    let empty_fault = part_fault(&ev, ids[0]);
    let empty_kind = kind_of(&empty_fault);
    assert_eq!(empty_kind, ProductErrorKind::NoBodyRoots);
    assert!(
        empty_kind.means_no_body(),
        "a body-less document reads as an absence"
    );
    assert!(
        empty_fault.to_string().contains("no body product"),
        "and the sentence says so: {empty_fault}"
    );

    let twice_fault = part_fault(&ev, ids[1]);
    let twice_kind = kind_of(&twice_fault);
    assert_eq!(twice_kind, ProductErrorKind::PlacedUnderTwoRoots);
    assert!(
        !twice_kind.means_no_body(),
        "a body placed twice is a fault, not an absence"
    );
    assert_ne!(
        empty_kind, twice_kind,
        "the two refusals are distinguishable without reading either sentence"
    );
}

/// MINOR-2 — `part_evaluations` counts the whole run's seam traffic:
/// A → B → P is TWO crossings, seen from the top.
#[test]
fn r1_part_evaluations_aggregates_through_nesting() {
    let mut store = StubStore::default();
    let p = store.insert(part("asm2a-nest-p", 0.0, 1.0), Tol::witness());
    let sub = {
        let doc = ProfileDoc::empty(DocumentId::derive("asm2a-nest-b"), Tol::witness());
        let (doc, _) = insert(doc, Node::instantiate_part(p));
        doc
    };
    let b = store.insert(sub, Tol::witness());
    let opts = with_resolver(store);

    let (doc, ids) = assembly("asm2a-nest-a", &[b]);
    let ev = run(&doc, &opts);
    assert_eq!(
        ev.part_evaluations, 2,
        "one crossing per document entered, nested crossings included"
    );
    // And the nesting really produced geometry: a doubly-wrapped name.
    let names = instance_names(&ev, ids[0]);
    assert!(
        names.iter().any(|n| matches!(
            &n.path[..],
            [RoleSeg::InPart { of }] if matches!(&of.path[..], [RoleSeg::InPart { .. }])
        )),
        "a nested instance's names wrap twice"
    );
}

/// MINOR-3 — the caller's candidate-generation strategy crosses the
/// seam, so a differential run exercises the PARTS' booleans too.
///
/// Honest boundary: the strategy's only observable is the verdict LOG
/// (`EvalOptions::boolean_sweep`), and a nested run's logs die with its
/// evaluation — so what is checkable from outside is the invariant the
/// inheritance exists to protect: both strategies agree, bit for bit,
/// on an assembly whose PART carries the boolean.
#[test]
fn r1_both_sweep_strategies_agree_on_a_part_carrying_a_boolean() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(boolean_part("asm2a-sweep-part"), Tol::witness());
    let resolver = with_resolver(store).resolver;

    let strategies = [
        topo::SweepStrategy::Realized,
        topo::SweepStrategy::Idealized,
    ];
    let volumes: Vec<u64> = strategies
        .into_iter()
        .map(|boolean_sweep| {
            let opts = EvalOptions {
                boolean_sweep,
                resolver: resolver.clone(),
                ..EvalOptions::default()
            };
            let (doc, _) = assembly("asm2a-sweep-asm", &[doc_ref]);
            let ev = run(&doc, &opts);
            let body = product(&doc, &ev, Tol::witness()).expect("the product gathers");
            volume(&body).to_bits()
        })
        .collect();
    assert_eq!(
        volumes[0], volumes[1],
        "the tree prunes and the predicates decide — same bits either way"
    );
}

/// MINOR-5 — `Frame::rotate_then_translate` really does agree with the
/// `Transform` node, BIT FOR BIT, including for a NON-UNIT axis (the
/// case the claim used to get wrong).
///
/// **This is an AGREEMENT row between two callers, not a value pin on
/// `Mat3::rotation_about`, and the axis list makes it read like one.**
/// The expected side below re-spells `eval::wire::wire_transform`'s
/// own expression — deliberately, because that is the right oracle for
/// agreement — so it calls `Mat3::rotation_about` itself and both
/// sides move together. **Any change INSIDE the rotation is invisible
/// here**, oblique axis and `to_bits()` notwithstanding — an oracle
/// that re-spells its caller's expression moves with the code and pins
/// nothing about it. The value pins that do object live next to the
/// subject,
/// in `geom-core`'s `linalg::mat` test module — the two rows named
/// `rotation_diagonal_takes_the_square_before_the_scale` and
/// `rotation_off_diagonals_scale_by_t_before_the_second_component`.
/// Strengthen those, not this.
#[test]
fn r1_the_placement_frame_matches_the_transform_node_bit_for_bit() {
    let angle = 0.37;
    let translation = [2.0, -3.5, 0.25];
    for axis in [
        [0.0, 0.0, 1.0],      // unit
        [0.0, 0.0, 5.0],      // non-unit, axis-aligned
        [1.0, 2.0, 3.0],      // non-unit, oblique
        [-0.5, 0.25, -0.125], // non-unit, short
    ] {
        let frame = Frame::rotate_then_translate(axis, angle, translation, fixture::band())
            .expect("a literal axis has a definite direction");
        // `eval::wire::wire_transform`'s own expression, verbatim: the
        // axis normalized (its `unit`), then `Mat3::rotation_about`,
        // then `Affine3::from_parts` with the translation.
        let unit = geom_core::Vec3::new(axis[0], axis[1], axis[2]).normalize();
        let expected = geom_core::Affine3::from_parts(
            geom_core::Mat3::rotation_about(unit, angle),
            geom_core::Vec3::new(translation[0], translation[1], translation[2]),
        );
        let got = frame.affine::<f64>();
        let cols = [
            (got.linear.c0, expected.linear.c0),
            (got.linear.c1, expected.linear.c1),
            (got.linear.c2, expected.linear.c2),
            (got.translation, expected.translation),
        ];
        for (g, e) in cols {
            for (g, e) in [(g.x, e.x), (g.y, e.y), (g.z, e.z)] {
                assert_eq!(
                    g.to_bits(),
                    e.to_bits(),
                    "axis {axis:?}: the frame and the transform node must agree by BITS"
                );
            }
        }
    }
}
