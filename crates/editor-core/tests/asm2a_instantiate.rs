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

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    CancelToken, ContentPin, DocEdit, DocRef, DocumentId, EditError, EvalOptions, Evaluation,
    Frame, Node, NodeErrorKind, NodeResult, PartFault, PartResolver, PersistError, ProfileDoc,
    RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, SnapshotError, StableName, content_pin,
    evaluate, load, product, product_named, save,
};
use fixture::{desc, insert, len, square, step};
use geom_core::Tol;

/// The v6 bytes, kept verbatim as the clean break's refusal fixture
/// (the ASM-ROOTS precedent: a break nobody can demonstrate is a break
/// nobody can trust).
const V6: &str = include_str!("golden/v6_golden.cad");

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

fn run(doc: &ProfileDoc, opts: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), opts, Tol::witness())
}

/// A one-solid part document: a `side`-wide square extruded 1 tall,
/// centered at `cx`.
fn part(label: &str, cx: f64, side: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(cx, 0.0, side / 2.0)],
        )),
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
    let (doc, plate_p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)]],
        )),
    );
    let (doc, plate) = insert(
        doc,
        Node::Extrude {
            profile: plate_p,
            distance: len(0.8),
        },
    );
    let (doc, boss_p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(1.2, 1.7, 0.35)],
        )),
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
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(10.0, 0.0, 0.5)],
        )),
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
    ) {
        Err(e @ EditError::ImproperPlacement { node, determinant }) => {
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
    // nothing at all (the lazy cache's whole point).
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

/// Row 7 — v7 round-trips (node + placements), v6 refuses typed, and
/// two blesses of one document are byte-identical.
#[test]
fn row7_v7_round_trips_and_v6_refuses() {
    // v7 was this unit's bump; LIB-LBRET's AtToward vocabulary took
    // v8 on top of it (the two units double-claimed 7 — see the
    // SCHEMA_VERSION ledger), LIB-RESPELL's §2c re-spell took v9, and
    // ASM-UPD's `UpdateReference` arm took v10, M9-1's declaration
    // class took v11, LIB-PLACEDUNION's group boolean took v12,
    // ASM-R2a's `Node::Mate` arm took v13, ASM-R2b's interface record
    // took v14, and M10-1's parameter distributions took v15. The row's subject is the
    // round trip and the v6 refusal, both unaffected; only the number
    // moved.
    assert_eq!(editor_core::SCHEMA_VERSION, 16);
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm2a-r7-part", 0.0, 1.0), Tol::witness());
    let (doc, ids) = assembly("asm2a-r7-asm", &[doc_ref, doc_ref]);
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[1],
            frame: Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.25, [2.0, 3.0, 0.0]),
        },
    );

    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert_eq!(
        text.lines().next(),
        Some(&format!("schema: {}", editor_core::SCHEMA_VERSION)[..]),
        "a fresh save carries the CURRENT version, whatever bumped it last"
    );
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

    assert_eq!(V6.lines().next(), Some("schema: 6"));
    match load(V6, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 6);
            assert_eq!(supported, editor_core::SCHEMA_VERSION);
            assert_eq!(missing, 6, "the 6 → 7 step is the one that does not exist");
        }
        other => panic!("v6 must refuse SchemaTooOld, got {other:?}"),
    }
    assert!(editor_core::REGENERATE_RECOURSE.contains("regenerate"));
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
    let (doc, other) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(0.0, 0.0, 0.5)],
        )),
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

/// The innermost cause of a chained seam fault — what the author has
/// to be told, however many documents down it lies.
fn root_cause(fault: &PartFault) -> &PartFault {
    match fault {
        PartFault::PartRootFailed {
            cause: Some(inner), ..
        } => root_cause(inner),
        other => other,
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
    let fault = part_fault(&run(&doc, &opts), ids[0]);
    match root_cause(&fault) {
        PartFault::ReferenceCycle { cycle } => {
            assert_eq!(
                cycle,
                &vec![a, b, a],
                "the loop is carried first repeated reference through last"
            );
        }
        other => panic!("expected a named cycle, got {other:?}"),
    }
    // The DIAGNOSIS reaches the top: the rendering names the loop, not
    // an evaluation the caller cannot reach.
    let rendered = fault.to_string();
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
    let fault = part_fault(&run(&doc, &opts), ids[0]);
    match &fault {
        PartFault::PartRootFailed { node, cause, .. } => {
            assert_eq!(*node, inner_root, "the failing ROOT is named");
            assert!(
                matches!(
                    cause.as_deref(),
                    Some(PartFault::Unresolved {
                        fault: ResolveFault::Unresolved,
                        ..
                    })
                ),
                "the cause travels typed, not as prose: {cause:?}"
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
        rendered.contains("product root") && rendered.contains("did not resolve"),
        "it names the root AND the reason: {rendered}"
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
/// here**, oblique axis and `to_bits()` notwithstanding. Smell-scan
/// **S215**. The value pins that do object live next to the subject,
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
        let frame = Frame::rotate_then_translate(axis, angle, translation);
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
