//! ASM-4 acceptance — split and inline (docs/ASM-4-SPEC.md).
//!
//! The kernel-layer rows: everything reachable with a stub resolver.
//! The document-layer rows — the workspace write side, duplicate-id at
//! create, and D9 across fresh processes — live in `pncad`'s suite,
//! because the store is that layer's.
//!
//! Undo note, as in `asm2a_instantiate`: this layer has no undo STACK.
//! The refactorings are pure and undo is keeping prior values, so
//! "undo restores exactly" is checked the way the layer offers it —
//! the pre-refactoring documents, held across the calls, are unchanged
//! bit for bit, and the returned edit lists replay to exactly the
//! returned documents (the recorded path and the returned value are
//! one state).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use editor_core::{
    CancelToken, DocEdit, DocParam, DocRef, DocumentId, EvalOptions, Evaluation, Expr, InlineError,
    Node, ParamName, PartResolver, ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg,
    SplitError, StableName, content_pin, evaluate, inline, load, product_named, save, split,
};
use fixture::{desc, insert, len, square, step};

// ---- The stub store (the asm2a idiom: no files, full pin gate) ----

#[derive(Debug, Default)]
struct StubStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl StubStore {
    fn insert(&mut self, doc: ProfileDoc) -> DocRef {
        let pin = content_pin(&doc).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        let found = content_pin(doc).expect("the pin computes");
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
    evaluate::<f64>(doc, None, &CancelToken::new(), opts)
}

/// A one-solid part document: a `side`-wide square extruded 1 tall,
/// centered at `cx` (the asm2a fixture).
fn part(label: &str, cx: f64, side: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label));
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

/// A two-cluster assembly: two instances of `doc_ref`, the second at
/// x = +5 — the flagship acceptance shape (row 1).
fn two_cluster_assembly(label: &str) -> (StubStore, ProfileDoc, Vec<RecipeNodeId>) {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part(&format!("{label}-part"), 0.0, 1.0));
    let mut doc = ProfileDoc::empty(DocumentId::derive(label));
    let mut ids = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[1],
            frame: editor_core::Frame::translation([5.0, 0.0, 0.0]),
        },
    );
    (store, doc, ids)
}

/// The structural census the A4 identity compares: solids, faces,
/// edges, vertices of the whole product.
fn census(body: &topo::Body<f64>) -> (usize, usize, usize, usize) {
    (
        body.solids().count(),
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
    )
}

fn volume_bits(body: &topo::Body<f64>) -> u64 {
    topo::mass_properties(body)
        .expect("mass properties")
        .volume
        .to_bits()
}

/// Wraps a part-local name under an instance — the bridge's own form.
fn wrap(instance: RecipeNodeId, of: &StableName) -> StableName {
    StableName {
        kind: of.kind,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(of.clone()),
        }],
    }
}

/// Replays an edit list over a starting document — the "the recorded
/// edits ARE the result" comparator undo leans on.
fn replay(mut doc: ProfileDoc, edits: &[DocEdit<editor_core::ProfileProgram>]) -> ProfileDoc {
    for edit in edits {
        doc = editor_core::apply(&doc, edit)
            .expect("the recorded edit replays")
            .doc;
    }
    doc
}

// ---- Row 1: split one cluster out, A4 identity ----

/// Row 1 — splitting a two-cluster assembly's second cluster out
/// leaves a remainder that instantiates the new document, and the A4
/// identity holds: censuses equal, volumes bit-equal, and every
/// probed name resolves to the corresponding entity (the split side
/// through the instance qualifier).
#[test]
fn row1_split_one_cluster_preserves_structure_and_names() {
    let (store, doc, ids) = two_cluster_assembly("asm4-r1");
    let part_ref = match doc.node(ids[0]) {
        Some(Node::InstantiatePart { doc_ref, .. }) => *doc_ref,
        _ => panic!("fixture shape"),
    };
    let opts = with_resolver(store);
    let ev1 = run(&doc, &opts);
    let (body1, names1) = product_named(&doc, &ev1).expect("the unsplit product gathers");

    let cut = BTreeSet::from([ids[1]]);
    let out = split(&doc, &cut, DocumentId::derive("asm4-r1-new")).expect("the split is legal");

    // The remainder instantiates the NEW document at its true pin.
    match out.remainder.node(out.instance) {
        Some(Node::InstantiatePart { doc_ref, .. }) => {
            assert_eq!(doc_ref.id, DocumentId::derive("asm4-r1-new"));
            assert_eq!(
                doc_ref.pin,
                content_pin(&out.part).expect("the part pins"),
                "the reference pins the part document actually produced"
            );
        }
        other => panic!("the remainder's instance is an instantiate node, got {other:?}"),
    }

    // Evaluate the remainder against a store holding BOTH documents.
    let mut store2 = StubStore::default();
    store2.insert(part("asm4-r1-part", 0.0, 1.0));
    store2.insert(out.part.clone());
    let opts2 = with_resolver(store2);
    let ev2 = run(&out.remainder, &opts2);
    let (body2, names2) = product_named(&out.remainder, &ev2).expect("the split product gathers");

    assert_eq!(census(&body1), census(&body2), "censuses are equal");
    assert_eq!(
        volume_bits(&body1),
        volume_bits(&body2),
        "volumes are bit-equal"
    );

    // NAME-RESOLUTION identity: every product name of the unsplit
    // evaluation resolves after the split — kept names verbatim, cut
    // names through the instance qualifier (probing the WHOLE table is
    // the strongest form of "probe several").
    let cut_instance = ids[1];
    let mapped = out.node_map[&cut_instance];
    let mut probed_cut = 0usize;
    for (name, _) in names1.iter() {
        if name.node == cut_instance {
            // The B-minted name, re-anchored: InPart(remap(name)) at
            // the remainder's instance.
            let RoleSeg::InPart { of } = &name.path[0] else {
                panic!("an instance-minted product name wraps a part-local one");
            };
            let expected = wrap(
                out.instance,
                &StableName {
                    kind: name.kind,
                    node: mapped,
                    path: vec![RoleSeg::InPart { of: of.clone() }],
                },
            );
            assert!(
                names2.iter().any(|(n, _)| *n == expected),
                "cut name {name:?} re-resolves as {expected:?}"
            );
            probed_cut += 1;
        } else {
            assert!(
                names2.iter().any(|(n, _)| n == name),
                "kept name {name:?} still resolves verbatim"
            );
        }
    }
    assert!(probed_cut > 0, "the probe actually crossed the seam");

    // Geometry rides the names: one part-local vertex, resolved under
    // the unsplit instance and under the split side's double wrapper,
    // lands on bit-identical coordinates.
    let sample = names1
        .iter()
        .find(|(n, _)| n.node == cut_instance && n.kind == editor_core::EntityKind::Vertex)
        .map(|(n, _)| n.clone())
        .expect("a cut vertex name");
    let RoleSeg::InPart { of } = &sample.path[0] else {
        panic!("wrapped vertex");
    };
    let before = editor_core::vertex_position(&ev1, cut_instance, &sample).expect("resolves");
    let after_name = wrap(
        out.instance,
        &StableName {
            kind: sample.kind,
            node: mapped,
            path: vec![RoleSeg::InPart { of: of.clone() }],
        },
    );
    let after =
        editor_core::vertex_position(&ev2, out.instance, &after_name).expect("resolves after");
    assert_eq!(before.x.to_bits(), after.x.to_bits());
    assert_eq!(before.y.to_bits(), after.y.to_bits());
    assert_eq!(before.z.to_bits(), after.z.to_bits());

    let _ = part_ref;
}

/// Row 1, the non-hoisted shape — cutting a PLAIN subtree (no cluster)
/// moves the recipe verbatim; the remainder instance sits at identity
/// and the identity still holds.
#[test]
fn row1_split_plain_subtree_preserves_structure() {
    // Two disjoint plain components: a unit block at the origin and a
    // half-block at x = 10.
    let doc = part("asm4-r1p", 0.0, 1.0);
    let (doc, p2) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(10.0, 0.0, 0.5)],
        )),
    );
    let (doc, e2) = insert(
        doc,
        Node::Extrude {
            profile: p2,
            distance: len(1.0),
        },
    );
    let opts = EvalOptions::default();
    let ev1 = run(&doc, &opts);
    let (body1, _) = product_named(&doc, &ev1).expect("gathers");

    let cut = BTreeSet::from([p2, e2]);
    let out = split(&doc, &cut, DocumentId::derive("asm4-r1p-new")).expect("legal");
    assert!(
        out.remainder.placements().is_empty(),
        "a plain cut leaves the remainder instance at identity"
    );

    let mut store = StubStore::default();
    store.insert(out.part.clone());
    let ev2 = run(&out.remainder, &with_resolver(store));
    let (body2, _) = product_named(&out.remainder, &ev2).expect("gathers");
    assert_eq!(census(&body1), census(&body2));
    assert_eq!(volume_bits(&body1), volume_bits(&body2));
}

// ---- Row 2: inline is the inverse; undo restores exactly ----

/// Row 2 — inlining the split-out instance restores the A4 identity
/// with the original document, and undo of BOTH refactorings restores
/// exactly (purity: the inputs are untouched; the recorded edits
/// replay to exactly the returned values).
#[test]
fn row2_inline_inverts_split_and_undo_restores() {
    let (store, doc, ids) = two_cluster_assembly("asm4-r2");
    let snapshot = doc.clone();
    let opts = with_resolver(store);
    let ev1 = run(&doc, &opts);
    let (body1, names1) = product_named(&doc, &ev1).expect("gathers");

    let cut = BTreeSet::from([ids[1]]);
    let out = split(&doc, &cut, DocumentId::derive("asm4-r2-new")).expect("legal");
    // Undo of the split: the input document is untouched, and the
    // recorded edits ARE the returned documents.
    assert!(doc.bit_eq(&snapshot), "split leaves its input untouched");
    assert!(
        replay(doc.clone(), &out.remainder_edits).bit_eq(&out.remainder),
        "the remainder edits replay to the remainder"
    );
    assert!(
        replay(
            ProfileDoc::empty(DocumentId::derive("asm4-r2-new")),
            &out.part_edits
        )
        .bit_eq(&out.part),
        "the part edits replay to the part"
    );

    let mut store2 = StubStore::default();
    store2.insert(part("asm4-r2-part", 0.0, 1.0));
    store2.insert(out.part.clone());

    let remainder_snapshot = out.remainder.clone();
    let inlined = inline(&out.remainder, out.instance, &store2).expect("the inline is legal");
    assert!(
        out.remainder.bit_eq(&remainder_snapshot),
        "inline leaves its input untouched"
    );
    assert!(
        replay(out.remainder.clone(), &inlined.edits).bit_eq(&inlined.doc),
        "the inline edits replay to the result"
    );

    // The round trip's identity with the ORIGINAL document.
    let mut store3 = StubStore::default();
    store3.insert(part("asm4-r2-part", 0.0, 1.0));
    let ev3 = run(&inlined.doc, &with_resolver(store3));
    let (body3, names3) = product_named(&inlined.doc, &ev3).expect("gathers");
    assert_eq!(census(&body1), census(&body3));
    assert_eq!(volume_bits(&body1), volume_bits(&body3));
    // The cut instance came back as a fresh node; its names correspond
    // through the two maps (split's, then inline's).
    let back = inlined.node_map[&out.node_map[&ids[1]]];
    for (name, _) in names1.iter() {
        let expected = if name.node == ids[1] {
            StableName {
                kind: name.kind,
                node: back,
                path: name.path.clone(),
            }
        } else {
            name.clone()
        };
        assert!(
            names3.iter().any(|(n, _)| *n == expected),
            "{name:?} re-resolves as {expected:?} after the round trip"
        );
    }
    // And the restored placement is the original frame, bit for bit.
    assert!(
        inlined.doc.placement(back).bit_eq(&doc.placement(ids[1])),
        "the round trip restores the cluster frame exactly"
    );
    let _ = names1;
}

/// Row 2 (document data) — an appearance record keyed by a cut
/// entity's name rides the bridge out and back: the split re-anchors
/// it to the instance-qualified name, the inline restores the local
/// form.
#[test]
fn row2_appearance_rides_the_bridge_both_ways() {
    let (store, doc, ids) = two_cluster_assembly("asm4-r2a");
    let opts = with_resolver(store);
    let ev = run(&doc, &opts);
    // A face name minted at the SECOND instance, keyed with an
    // appearance attribute (the asm2a row-3 idiom).
    let face = ev
        .value(ids[1])
        .expect("the instance evaluated")
        .name_table
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| n.kind == editor_core::EntityKind::Face)
        .expect("a face name");
    let (doc, _) = step(
        doc,
        DocEdit::SetAppearance {
            name: face.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8 {
                r: 1,
                g: 2,
                b: 3,
                a: 255,
            }),
        },
    );

    let cut = BTreeSet::from([ids[1]]);
    let out = split(&doc, &cut, DocumentId::derive("asm4-r2a-new")).expect("legal");
    let RoleSeg::InPart { of } = &face.path[0] else {
        panic!("wrapped face");
    };
    let expected = wrap(
        out.instance,
        &StableName {
            kind: face.kind,
            node: out.node_map[&ids[1]],
            path: vec![RoleSeg::InPart { of: of.clone() }],
        },
    );
    assert!(
        out.remainder.appearance_of(&face).is_none(),
        "the local key is gone from the remainder"
    );
    assert!(
        out.remainder.appearance_of(&expected).is_some(),
        "the record re-anchored to the instance-qualified key"
    );
    assert!(
        out.part.appearance().is_empty(),
        "the record stays with the document that referenced the entity"
    );

    let mut store2 = StubStore::default();
    store2.insert(part("asm4-r2a-part", 0.0, 1.0));
    store2.insert(out.part.clone());
    let inlined = inline(&out.remainder, out.instance, &store2).expect("legal");
    let back = inlined.node_map[&out.node_map[&ids[1]]];
    let restored = StableName {
        kind: face.kind,
        node: back,
        path: face.path.clone(),
    };
    assert!(
        inlined.doc.appearance_of(&restored).is_some(),
        "inline restores the local (re-minted) key"
    );
    assert!(inlined.doc.appearance_of(&expected).is_none());
}

// ---- Row 3: refusals ----

/// Row 3a — a cut severing a consuming edge refuses typed NAMING the
/// edge, in both crossing directions.
#[test]
fn row3_severing_cut_refuses_naming_the_edge() {
    let doc = part("asm4-r3s", 0.0, 1.0);
    let profile = doc.order()[0];
    let extrude = doc.order()[1];
    // The kept consumer's edge into the cut input…
    match split(&doc, &BTreeSet::from([profile]), DocumentId::derive("n1")) {
        Err(SplitError::SeveredEdge {
            consumer,
            input,
            consumer_is_cut,
        }) => {
            assert_eq!((consumer, input), (extrude, profile));
            assert!(!consumer_is_cut);
        }
        other => panic!("expected SeveredEdge, got {other:?}"),
    }
    // …and the cut consumer's edge into the kept input.
    match split(&doc, &BTreeSet::from([extrude]), DocumentId::derive("n2")) {
        Err(SplitError::SeveredEdge {
            consumer,
            input,
            consumer_is_cut,
        }) => {
            assert_eq!((consumer, input), (extrude, profile));
            assert!(consumer_is_cut);
        }
        other => panic!("expected SeveredEdge, got {other:?}"),
    }
}

/// Row 3b — a cut node referencing a parameter a kept node also
/// references refuses typed naming the parameter and both nodes.
#[test]
fn row3_uncut_param_reference_refuses() {
    let doc = ProfileDoc::empty(DocumentId::derive("asm4-r3p"));
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("h"),
            value: DocParam::Continuous {
                dim: editor_core::Dimension::Length,
                value: 1.5,
            },
        },
    );
    let h = || Expr::param(ParamName::new("h"), editor_core::Dimension::Length);
    let sq = |cx: f64| {
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(cx, 0.0, 0.5)],
        ))
    };
    let (doc, p1) = insert(doc, sq(0.0));
    let (doc, e1) = insert(
        doc,
        Node::Extrude {
            profile: p1,
            distance: h(),
        },
    );
    let (doc, p2) = insert(doc, sq(10.0));
    let (doc, e2) = insert(
        doc,
        Node::Extrude {
            profile: p2,
            distance: h(),
        },
    );
    match split(&doc, &BTreeSet::from([p1, e1]), DocumentId::derive("n")) {
        Err(SplitError::UncutParamReference {
            param,
            cut_node,
            kept_node,
        }) => {
            assert_eq!(param, ParamName::new("h"));
            assert_eq!(cut_node, e1);
            assert_eq!(kept_node, e2);
        }
        other => panic!("expected UncutParamReference, got {other:?}"),
    }
    // A parameter referenced ONLY by the cut side is copied, and the
    // split is legal.
    let out = split(
        &doc,
        &BTreeSet::from([p1, e1, p2, e2]),
        DocumentId::derive("n"),
    )
    .expect("a cut containing every referencing node carries the parameter");
    assert!(out.part.params().contains_key(&ParamName::new("h")));
}

/// Row 3c — inline of a stale pin is the resolver's PinMismatch,
/// typed; the reference is never silently retargeted.
#[test]
fn row3_inline_of_stale_pin_is_pin_mismatch() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm4-r3i-part", 0.0, 1.0));
    let host = ProfileDoc::empty(DocumentId::derive("asm4-r3i"));
    let (host, id) = insert(host, Node::instantiate_part(doc_ref));
    // The referenced document moves on under the same id: the pin the
    // host carries no longer holds.
    store.insert(part("asm4-r3i-part", 0.0, 2.0));
    match inline(&host, id, &store) {
        Err(InlineError::Unresolved { failure }) => {
            assert_eq!(failure.fault, ResolveFault::PinMismatch);
        }
        other => panic!("expected Unresolved(PinMismatch), got {other:?}"),
    }
}

/// Row 3 (this unit's own found refusals) — the empty cut, the
/// colliding part id, a consumed instance, and the frame no local
/// recipe can express.
#[test]
fn row3_further_typed_refusals() {
    let (_, doc, ids) = two_cluster_assembly("asm4-r3f");
    assert!(matches!(
        split(&doc, &BTreeSet::new(), DocumentId::derive("n")),
        Err(SplitError::EmptyCut)
    ));
    assert!(matches!(
        split(&doc, &BTreeSet::from([ids[0]]), doc.id()),
        Err(SplitError::PartIdCollides { .. })
    ));
    assert!(matches!(
        split(
            &doc,
            &BTreeSet::from([RecipeNodeId(999)]),
            DocumentId::derive("n")
        ),
        Err(SplitError::UnknownCutNode { .. })
    ));

    // A consumed instance cannot inline (the recipe cannot rewire the
    // consumer onto a spliced product).
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm4-r3f-part", 0.0, 1.0));
    let host = ProfileDoc::empty(DocumentId::derive("asm4-r3f-host"));
    let (host, inst) = insert(host, Node::instantiate_part(doc_ref));
    let (host, consumer) = insert(
        host,
        Node::Transform {
            input: inst,
            translation: [len(1.0), len(0.0), len(0.0)],
            rotation_axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
            rotation_angle: fixture::ang(0.0),
        },
    );
    match inline(&host, inst, &store) {
        Err(InlineError::InstanceConsumed { node, by }) => {
            assert_eq!((node, by), (inst, consumer));
        }
        other => panic!("expected InstanceConsumed, got {other:?}"),
    }

    // A placed instance of a PLAIN-geometry part cannot inline: the
    // frame is not something the spliced recipe can express (D-3's
    // refusal, the case v1 found).
    let host2 = ProfileDoc::empty(DocumentId::derive("asm4-r3f-host2"));
    let (host2, inst2) = insert(host2, Node::instantiate_part(doc_ref));
    let (host2, _) = step(
        host2,
        DocEdit::SetPlacement {
            node: inst2,
            frame: editor_core::Frame::translation([3.0, 0.0, 0.0]),
        },
    );
    match inline(&host2, inst2, &store) {
        Err(InlineError::UnplaceableFrame { root }) => {
            let part_doc = part("asm4-r3f-part", 0.0, 1.0);
            assert_eq!(root, part_doc.order()[1], "the plain extrude root is named");
        }
        other => panic!("expected UnplaceableFrame, got {other:?}"),
    }
    // The same instance at IDENTITY inlines fine.
    let host3 = ProfileDoc::empty(DocumentId::derive("asm4-r3f-host3"));
    let (host3, inst3) = insert(host3, Node::instantiate_part(doc_ref));
    inline(&host3, inst3, &store).expect("an identity-placed plain part inlines");
}

// ---- Row 4: roots and placements, both sides ----

/// Row 4 — the A10/A11 maintenance through the recorded edits
/// produces the expected root and placement lists on both sides, each
/// its own assertion.
#[test]
fn row4_roots_and_placements_land_as_the_rules_say() {
    // The hoisted single-cluster cut.
    let (_, doc, ids) = two_cluster_assembly("asm4-r4");
    let out = split(
        &doc,
        &BTreeSet::from([ids[1]]),
        DocumentId::derive("asm4-r4-new"),
    )
    .expect("legal");
    let mapped = out.node_map[&ids[1]];
    assert_eq!(
        out.remainder.roots(),
        &[ids[0], out.instance],
        "the instance takes the cut root's list position"
    );
    assert_eq!(
        out.part.roots(),
        &[mapped],
        "the part's root is the cut root"
    );
    assert!(
        out.remainder
            .placement(out.instance)
            .bit_eq(&doc.placement(ids[1])),
        "the hoisted frame is the cluster's old frame"
    );
    assert!(
        out.part.placements().is_empty(),
        "the hoisted cluster sits at identity in the part"
    );

    // The multi-cluster cut: both frames MOVE, the remainder instance
    // sits at identity, and the part keeps the cut roots' LIST order
    // even where insertion order disagrees.
    let mut store = StubStore::default();
    let doc_ref = store.insert(part("asm4-r4b-part", 0.0, 1.0));
    let mut doc2 = ProfileDoc::empty(DocumentId::derive("asm4-r4b"));
    let mut inst = Vec::new();
    for dx in [0.0_f64, 7.0, 14.0] {
        let (next, id) = insert(doc2, Node::instantiate_part(doc_ref));
        doc2 = next;
        if dx != 0.0 {
            let (next, _) = step(
                doc2,
                DocEdit::SetPlacement {
                    node: id,
                    frame: editor_core::Frame::translation([dx, 0.0, 0.0]),
                },
            );
            doc2 = next;
        }
        inst.push(id);
    }
    // Reorder the roots so the cut pair's list order disagrees with
    // insertion order.
    let (doc2, _) = step(
        doc2,
        DocEdit::SetRoots {
            roots: vec![inst[2], inst[0], inst[1]],
        },
    );
    let out2 = split(
        &doc2,
        &BTreeSet::from([inst[0], inst[2]]),
        DocumentId::derive("asm4-r4b-new"),
    )
    .expect("legal");
    assert_eq!(
        out2.remainder.roots(),
        &[out2.instance, inst[1]],
        "the instance takes the FIRST cut root's position"
    );
    assert_eq!(
        out2.part.roots(),
        &[out2.node_map[&inst[2]], out2.node_map[&inst[0]]],
        "the part keeps the cut roots' A10 list order"
    );
    assert!(
        out2.remainder.placement(out2.instance).is_identity_bits(),
        "a multi-cluster cut leaves the remainder instance at identity"
    );
    assert!(
        out2.part
            .placement(out2.node_map[&inst[2]])
            .bit_eq(&doc2.placement(inst[2])),
        "the moved frame is verbatim"
    );
    assert!(
        out2.part
            .placement(out2.node_map[&inst[0]])
            .is_identity_bits(),
        "an unplaced cut instance stays unplaced"
    );
}

// ---- Row 5: the interface-record hook ----

/// Row 5 — the empty crossing-declaration record exists on the
/// remainder's instance and round-trips persistence, costing no wire
/// bytes while empty (so no pin moves until R2 populates it).
#[test]
fn row5_interface_record_exists_and_round_trips() {
    let (_, doc, ids) = two_cluster_assembly("asm4-r5");
    let out = split(
        &doc,
        &BTreeSet::from([ids[1]]),
        DocumentId::derive("asm4-r5-new"),
    )
    .expect("legal");
    match out.remainder.node(out.instance) {
        Some(Node::InstantiatePart { interface, .. }) => {
            assert!(interface.is_empty(), "no mate vocabulary exists to cross");
        }
        other => panic!("expected the instance, got {other:?}"),
    }
    let text = save(&out.remainder, &[]).expect("saves");
    assert!(
        !text.contains("\"interface\""),
        "the empty record is absent from the wire (pin stability)"
    );
    let loaded = load(&text).expect("loads");
    assert!(loaded.doc.bit_eq(&out.remainder), "the round trip is exact");
    match loaded.doc.node(out.instance) {
        Some(Node::InstantiatePart { interface, .. }) => {
            assert!(interface.is_empty(), "absent-on-wire IS the empty record");
        }
        other => panic!("expected the instance after load, got {other:?}"),
    }
}

// ---- D-4: round trip through persistence, both sides ----

/// D-4's persistence leg — both produced documents survive
/// save/load, and the loaded pair still evaluates to the A4 identity.
#[test]
fn split_pair_round_trips_persistence_and_still_evaluates_identically() {
    let (store, doc, ids) = two_cluster_assembly("asm4-per");
    let opts = with_resolver(store);
    let ev1 = run(&doc, &opts);
    let (body1, _) = product_named(&doc, &ev1).expect("gathers");

    let out = split(
        &doc,
        &BTreeSet::from([ids[1]]),
        DocumentId::derive("asm4-per-new"),
    )
    .expect("legal");
    let part_loaded = load(&save(&out.part, &[]).expect("part saves")).expect("part loads");
    let rem_loaded =
        load(&save(&out.remainder, &[]).expect("remainder saves")).expect("remainder loads");
    assert!(part_loaded.doc.bit_eq(&out.part));
    assert!(rem_loaded.doc.bit_eq(&out.remainder));
    assert_eq!(
        content_pin(&part_loaded.doc).expect("pins"),
        content_pin(&out.part).expect("pins"),
        "the pin is stable across the wire"
    );

    let mut store2 = StubStore::default();
    store2.insert(part("asm4-per-part", 0.0, 1.0));
    store2.insert(part_loaded.doc);
    let ev2 = run(&rem_loaded.doc, &with_resolver(store2));
    let (body2, _) = product_named(&rem_loaded.doc, &ev2).expect("gathers");
    assert_eq!(census(&body1), census(&body2));
    assert_eq!(volume_bits(&body1), volume_bits(&body2));
}
