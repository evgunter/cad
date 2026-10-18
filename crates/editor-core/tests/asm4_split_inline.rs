//! ASM-4 acceptance — split and inline.
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

use crate::fixture;
use editor_core::ExtrudeSide;

use std::collections::BTreeSet;

use crate::blocks::block;
use crate::p2_split::pinned_at;

use editor_core::{
    DocEdit, DocumentId, EvalOptions, Formula, FreeVar, InlineError, Node, ProfileDoc,
    RecipeNodeId, ResolveFault, RoleSeg, SplitError, StableName, VarName, content_pin, inline,
    load, product_named, save,
};
use fixture::resolver::{PartStore, with_resolver};
use fixture::{desc, insert, len, on_frame, run, split_world as split, square, step, xy_frame};
use geom_core::Tol;
use std::sync::Arc;

// ---- Evaluation through the shared part store (no files, full pin gate) ----

/// A one-solid part document: a `side`-wide square extruded 1 tall,
/// centered at `cx` (the asm2a fixture).
/// **Positions** in a `part` document's node order — its sketch frame,
/// the profile drawn on it, and the extrude that is its body. These
/// index `doc.ids()`; they are not node ids.
const PLANE_POSITION: usize = 0;
const PROFILE_POSITION: usize = 1;
const BODY_POSITION: usize = 2;

fn part(label: &str, cx: f64, side: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, side / 2.0)],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    fixture::place(doc, body).0
}

/// A two-instance assembly: two instances of `doc_ref`, the first pinned
/// to the world at the identity, the second at x = +5 — the flagship
/// acceptance shape (row 1).
fn two_instance_assembly(label: &str) -> (PartStore, ProfileDoc, Vec<RecipeNodeId>) {
    let mut store = PartStore::default();
    let doc_ref = store.insert(part(&format!("{label}-part"), 0.0, 1.0), Tol::witness());
    let o = with_resolver(store.clone());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, first) = pinned(doc, doc_ref, &o);
    let (doc, second) = pinned_at(doc, doc_ref, [5.0, 0.0, 0.0], &o);
    (store, doc, vec![first, second])
}

/// **An instance of `doc_ref` pinned to the world at the identity**
/// over its own geometry, the parts `o` serves evaluating it.
fn pinned(
    doc: ProfileDoc,
    doc_ref: editor_core::DocRef,
    o: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), o)
}

/// **The instance a split leaves stands at the identity of the
/// remainder's world**, the part served beside `store`'s.
fn left_at_identity(out: &editor_core::SplitOutcome, store: &PartStore) -> bool {
    let mut store = store.clone();
    store.insert(out.part.clone(), Tol::witness());
    fixture::offset_of(&out.remainder, out.instance).is_some()
        && crate::p2_split::at_identity(&out.remainder, out.instance, &with_resolver(store))
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
    volume(body).to_bits()
}

fn volume(body: &topo::Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness())
        .expect("mass properties")
        .volume
}

/// Two products' volumes are bit-equal: an instance a split leaves
/// pinned at the identity moves nothing.
fn same_volume(a: &topo::Body<f64>, b: &topo::Body<f64>, what: &str) {
    assert_eq!(volume_bits(a), volume_bits(b), "{what}");
}

/// Wraps a part-local name under an instance — the bridge's own form.
fn wrap(instance: RecipeNodeId, of: &StableName) -> StableName {
    StableName {
        kind: of.kind,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: of.clone().into(),
        }],
    }
}

/// **The key of the one copy a [`part`] document delivers**: its one
/// placement on its world, and the read that placement copies the body
/// through ([`editor_core::CopyKey`]).
fn plain_copy_key(part_doc: &ProfileDoc) -> editor_core::CopyKey {
    let placement = part_doc.placements()[0];
    let Some(Node::Place { shapes, .. }) = part_doc.node(placement) else {
        panic!("the part places its body")
    };
    editor_core::CopyKey::part(placement, editor_core::CopyKey::Read(shapes[0].read))
}

/// What `doc` places, in its placements' document order — the order
/// the product gathers its copies in: an instance itself, since it is
/// its own placement, and the body a one-shape `Place` copies.
fn placed_bodies(doc: &ProfileDoc) -> Vec<RecipeNodeId> {
    doc.placements()
        .into_iter()
        .map(|p| match doc.node(p) {
            Some(Node::InstantiatePart { .. }) => p,
            Some(Node::Place { shapes, .. }) => match shapes.as_slice() {
                [one] => doc
                    .operation_of(one.read)
                    .expect("a placement reads a body"),
                other => panic!("a placement of one shape, got {other:?}"),
            },
            other => panic!("a placement, got {other:?}"),
        })
        .collect()
}

/// Replays an edit list over a starting document — the "the recorded
/// edits ARE the result" comparator undo leans on.
fn replay(mut doc: ProfileDoc, edits: &[DocEdit<editor_core::ProfileProgram>]) -> ProfileDoc {
    for edit in edits {
        doc = editor_core::apply(&doc, edit, Tol::witness(), &editor_core::RefusingReach)
            .expect("the recorded edit replays")
            .doc;
    }
    doc
}

// ---- Row 1: split one instance out, A4 identity ----

/// Row 1 — splitting a two-instance assembly's second instance out
/// leaves a remainder that instantiates the new document, and the A4
/// identity holds: censuses equal, volumes bit-equal, and every probed
/// name
/// resolves to the corresponding entity (the split side through the
/// instance qualifier).
#[test]
fn row1_split_one_instance_preserves_structure_and_names() {
    let (store, doc, ids) = two_instance_assembly("asm4-r1");
    let part_ref = match doc.node(ids[0]) {
        Some(Node::InstantiatePart { doc_ref, .. }) => *doc_ref,
        _ => panic!("fixture shape"),
    };
    let opts = with_resolver(store);
    let ev1 = run(&doc, &opts);
    let (body1, names1) =
        product_named(&doc, &ev1, Tol::witness()).expect("the unsplit product gathers");

    let cut = BTreeSet::from([ids[1]]);
    let out = split(
        &doc,
        &cut,
        DocumentId::derive("asm4-r1-new"),
        Tol::witness(),
        opts.resolver.as_ref(),
    )
    .expect("the split is legal");

    // The remainder instantiates the NEW document at its true pin.
    match out.remainder.node(out.instance) {
        Some(Node::InstantiatePart { doc_ref, .. }) => {
            assert_eq!(doc_ref.id, DocumentId::derive("asm4-r1-new"));
            assert_eq!(
                doc_ref.pin,
                content_pin(&out.part, Tol::witness()).expect("the part pins"),
                "the reference pins the part document actually produced"
            );
        }
        other => panic!("the remainder's instance is an instantiate node, got {other:?}"),
    }

    // Evaluate the remainder against a store holding BOTH documents.
    let mut store2 = PartStore::default();
    store2.insert(part("asm4-r1-part", 0.0, 1.0), Tol::witness());
    store2.insert(out.part.clone(), Tol::witness());
    let opts2 = with_resolver(store2);
    let ev2 = run(&out.remainder, &opts2);
    let (body2, names2) =
        product_named(&out.remainder, &ev2, Tol::witness()).expect("the split product gathers");

    assert_eq!(census(&body1), census(&body2), "censuses are equal");
    same_volume(&body1, &body2, "volumes are equal");

    // NAME-RESOLUTION identity: every product name of the unsplit
    // evaluation resolves after the split — kept names verbatim, cut
    // names through the instance qualifier (probing the WHOLE table is
    // the strongest form of "probe several"). An instance is its own
    // placement, so the product names its entities as it does.
    let cut_instance = ids[1];
    let mapped = out.node_map[&cut_instance];
    // The cut name, re-anchored: the part's own name of the instance's
    // entity, under the remainder instance.
    let across = |name: &StableName| -> StableName {
        wrap(
            out.instance,
            &StableName {
                kind: name.kind,
                node: mapped,
                path: name.path.clone(),
            },
        )
    };
    let mut probed_cut = 0usize;
    for (name, _) in names1.iter() {
        if name.node == cut_instance {
            let expected = across(name);
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

    // Geometry rides the names: one part-local vertex, resolved through
    // the unsplit copy and through the split side's copy, lands on
    // bit-identical coordinates.
    let sample = names1
        .iter()
        .find(|(n, _)| n.node == cut_instance && n.kind == editor_core::EntityKind::Vertex)
        .map(|(n, _)| n.clone())
        .expect("a cut vertex name");
    let before = editor_core::vertex_position(&ev1, sample.node, &sample).expect("resolves");
    let after_name = across(&sample);
    let after =
        editor_core::vertex_position(&ev2, out.instance, &after_name).expect("resolves after");
    assert_eq!(before.x.to_bits(), after.x.to_bits());
    assert_eq!(before.y.to_bits(), after.y.to_bits());
    assert_eq!(before.z.to_bits(), after.z.to_bits());

    let _ = part_ref;
}

/// Row 1, a PLAIN subtree — cutting one (no instance)
/// moves the recipe verbatim; the remainder instance sits at identity
/// and the identity still holds.
#[test]
fn row1_split_plain_subtree_preserves_structure() {
    // Two disjoint plain components: a unit block at the origin and a
    // half-block at x = 10.
    let doc = part("asm4-r1p", 0.0, 1.0);
    // The half-block's own frame: the cut takes it, because a profile
    // whose plane stayed behind would sever an edge.
    let (doc, f2) = insert(doc, xy_frame());
    let (doc, p2) = insert(doc, Node::Profile(desc(f2, vec![square(10.0, 0.0, 0.5)])));
    let (doc, e2) = insert(
        doc,
        Node::Extrude {
            profile: p2.into(),
            distance: len(1.0),
        },
    );
    let doc = fixture::place(doc, e2).0;
    let opts = EvalOptions::default();
    let ev1 = run(&doc, &opts);
    let (body1, _) = product_named(&doc, &ev1, Tol::witness()).expect("gathers");

    let cut = BTreeSet::from([f2, p2, e2]);
    let out = split(
        &doc,
        &cut,
        DocumentId::derive("asm4-r1p-new"),
        Tol::witness(),
        None,
    )
    .expect("legal");
    assert!(
        left_at_identity(&out, &PartStore::default()),
        "a plain cut leaves the remainder instance pinned at the identity"
    );

    let mut store = PartStore::default();
    store.insert(out.part.clone(), Tol::witness());
    let ev2 = run(&out.remainder, &with_resolver(store));
    let (body2, _) = product_named(&out.remainder, &ev2, Tol::witness()).expect("gathers");
    assert_eq!(census(&body1), census(&body2));
    same_volume(&body1, &body2, "the plain cut keeps the volume");
}

// ---- Row 2: inline is the inverse; undo restores exactly ----

/// Row 2 — inlining the split-out instance restores the A4 identity
/// with the original document, and undo of BOTH refactorings restores
/// exactly (purity: the inputs are untouched; the recorded edits
/// replay to exactly the returned values).
#[test]
fn row2_inline_inverts_split_and_undo_restores() {
    let (store, doc, ids) = two_instance_assembly("asm4-r2");
    let snapshot = doc.clone();
    let opts = with_resolver(store);
    let ev1 = run(&doc, &opts);
    let (body1, names1) = product_named(&doc, &ev1, Tol::witness()).expect("gathers");

    let cut = BTreeSet::from([ids[1]]);
    let out = split(
        &doc,
        &cut,
        DocumentId::derive("asm4-r2-new"),
        Tol::witness(),
        opts.resolver.as_ref(),
    )
    .expect("legal");
    // Undo of the split: the input document is untouched, and the
    // recorded edits ARE the returned documents.
    assert!(doc.bit_eq(&snapshot), "split leaves its input untouched");
    assert!(
        replay(doc.clone(), &out.remainder_edits).bit_eq(&out.remainder),
        "the remainder edits replay to the remainder"
    );
    assert!(
        replay(
            ProfileDoc::empty(DocumentId::derive("asm4-r2-new"), Tol::witness()),
            &out.part_edits
        )
        .bit_eq(&out.part),
        "the part edits replay to the part"
    );

    let mut store2 = PartStore::default();
    store2.insert(part("asm4-r2-part", 0.0, 1.0), Tol::witness());
    store2.insert(out.part.clone(), Tol::witness());

    let remainder_snapshot = out.remainder.clone();
    let inlined = inline(
        &out.remainder,
        out.instance,
        &(Arc::new(store2) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    )
    .expect("the inline is legal");
    assert!(
        out.remainder.bit_eq(&remainder_snapshot),
        "inline leaves its input untouched"
    );
    assert!(
        replay(out.remainder.clone(), &inlined.edits).bit_eq(&inlined.doc),
        "the inline edits replay to the result"
    );

    // The round trip's identity with the ORIGINAL document.
    let mut store3 = PartStore::default();
    store3.insert(part("asm4-r2-part", 0.0, 1.0), Tol::witness());
    let ev3 = run(&inlined.doc, &with_resolver(store3));
    let (body3, names3) = product_named(&inlined.doc, &ev3, Tol::witness()).expect("gathers");
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
    // And the restored world pin is the original, bit for bit.
    assert!(
        fixture::same_offset(&inlined.doc, back, &doc, ids[1]),
        "the round trip restores the instance's world pin exactly"
    );
    let _ = names1;
}

/// Row 2 (document data) — an appearance record keyed by a cut
/// entity's name rides the bridge out and back: the split re-anchors
/// it to the instance-qualified name, the inline restores the local
/// form.
#[test]
fn row2_appearance_rides_the_bridge_both_ways() {
    let (store, doc, ids) = two_instance_assembly("asm4-r2a");
    let opts = with_resolver(store);
    let ev = run(&doc, &opts);
    // A face name minted at the SECOND instance, keyed with an
    // appearance attribute (the asm2a row-3 idiom).
    let face = ev
        .table_at(ids[1])
        .expect("the instance evaluated")
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| n.kind == editor_core::EntityKind::Face)
        .expect("a face name");
    let read = crate::fixture::painted_in(&doc, &face);
    let (doc, _) = step(
        doc,
        DocEdit::SetAppearance {
            body: read,
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
    let out = split(
        &doc,
        &cut,
        DocumentId::derive("asm4-r2a-new"),
        Tol::witness(),
        opts.resolver.as_ref(),
    )
    .expect("legal");
    // The instance's name, now the part's name of it, under the
    // remainder's instance.
    let expected = wrap(
        out.instance,
        &StableName {
            kind: face.kind,
            node: out.node_map[&ids[1]],
            path: face.path.clone(),
        },
    );
    assert!(
        crate::fixture::painted(&out.remainder, &face).is_none(),
        "the local key is gone from the remainder"
    );
    assert!(
        crate::fixture::painted(&out.remainder, &expected).is_some(),
        "the record re-anchored to the instance-qualified key"
    );
    assert!(
        out.part.appearance().is_empty(),
        "the record stays with the document that referenced the entity"
    );

    let mut store2 = PartStore::default();
    store2.insert(part("asm4-r2a-part", 0.0, 1.0), Tol::witness());
    store2.insert(out.part.clone(), Tol::witness());
    let inlined = inline(
        &out.remainder,
        out.instance,
        &(Arc::new(store2) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    )
    .expect("legal");
    let back = inlined.node_map[&out.node_map[&ids[1]]];
    let restored = StableName {
        kind: face.kind,
        node: back,
        path: face.path.clone(),
    };
    assert!(
        crate::fixture::painted(&inlined.doc, &restored).is_some(),
        "inline restores the local (re-minted) key"
    );
    assert!(crate::fixture::painted(&inlined.doc, &expected).is_none());
}

// ---- Row 3: refusals ----

/// Row 3a — a cut severing a consuming edge refuses typed NAMING the
/// edge, in both crossing directions.
#[test]
fn row3_severing_cut_refuses_naming_the_edge() {
    let doc = part("asm4-r3s", 0.0, 1.0);
    let plane = doc.ids()[PLANE_POSITION];
    let profile = doc.ids()[PROFILE_POSITION];
    let extrude = doc.ids()[BODY_POSITION];
    // The kept consumer's edge into the cut input… The frame rides
    // along, so the ONE severed edge is the extrude's — a cut that left
    // the profile's plane behind would sever that one first, and this
    // row is about the extrude's.
    match split(
        &doc,
        &BTreeSet::from([plane, profile]),
        DocumentId::derive("n1"),
        Tol::witness(),
        None,
    ) {
        Err(SplitError::SeveredEdge {
            consumer,
            input,
            consumer_is_cut,
            ..
        }) => {
            assert_eq!(
                (consumer, input),
                (doc.spoken(extrude), doc.spoken(profile))
            );
            assert!(!consumer_is_cut);
        }
        other => panic!("expected SeveredEdge, got {other:?}"),
    }
    // …and the cut consumer's edge into the kept input.
    match split(
        &doc,
        &BTreeSet::from([extrude]),
        DocumentId::derive("n2"),
        Tol::witness(),
        None,
    ) {
        Err(SplitError::SeveredEdge {
            consumer,
            input,
            consumer_is_cut,
            ..
        }) => {
            assert_eq!(
                (consumer, input),
                (doc.spoken(extrude), doc.spoken(profile))
            );
            assert!(consumer_is_cut);
        }
        other => panic!("expected SeveredEdge, got {other:?}"),
    }
}

/// Row 3b — a cut node referencing a parameter a kept node also
/// references refuses typed naming the parameter and both nodes.
#[test]
fn row3_uncut_param_reference_refuses() {
    let doc = ProfileDoc::empty(DocumentId::derive("asm4-r3p"), Tol::witness());
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("h"),
            def: editor_core::VarDecl::Free(FreeVar::continuous(
                editor_core::Dimension::Length,
                1.5,
            )),
        },
    );
    let h = || Formula::named(VarName::from_static("h"), editor_core::Dimension::Length);
    // Each block draws on its OWN frame. A shared one would sever an
    // edge at the cut below — the frame is a document input now — and
    // that refusal would fire before the parameter question this row
    // is about.
    let (doc, f1) = insert(doc, xy_frame());
    let (doc, p1) = insert(doc, Node::Profile(desc(f1, vec![square(0.0, 0.0, 0.5)])));
    let (doc, e1) = insert(
        doc,
        Node::Extrude {
            profile: p1.into(),
            distance: h(),
        },
    );
    let (doc, f2) = insert(doc, xy_frame());
    let (doc, p2) = insert(doc, Node::Profile(desc(f2, vec![square(10.0, 0.0, 0.5)])));
    let (doc, e2) = insert(
        doc,
        Node::Extrude {
            profile: p2.into(),
            distance: h(),
        },
    );
    let doc = fixture::place_all(doc, &[e1, e2]);
    match split(
        &doc,
        &BTreeSet::from([f1, p1, e1]),
        DocumentId::derive("n"),
        Tol::witness(),
        None,
    ) {
        Err(SplitError::UncutVarReference {
            var,
            cut_node,
            kept_node,
        }) => {
            assert_eq!(var.name(), Some(&VarName::from_static("h")));
            assert_eq!(Some(var.id()), doc.var_named("h"));
            assert_eq!(cut_node, doc.spoken(e1));
            assert_eq!(kept_node, doc.spoken(e2));
        }
        other => panic!("expected UncutVarReference, got {other:?}"),
    }
    // A parameter referenced ONLY by the cut side is copied, and the
    // split is legal.
    let out = split(
        &doc,
        &BTreeSet::from([f1, p1, e1, f2, p2, e2]),
        DocumentId::derive("n"),
        Tol::witness(),
        None,
    )
    .expect("a cut containing every referencing node carries the parameter");
    assert!(out.part.var_named("h").is_some());
}

/// Row 3c — inline of a stale pin is the resolver's PinMismatch,
/// typed; the reference is never silently retargeted.
#[test]
fn row3_inline_of_stale_pin_is_pin_mismatch() {
    let mut store = PartStore::default();
    let doc_ref = store.insert(part("asm4-r3i-part", 0.0, 1.0), Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("asm4-r3i"), Tol::witness());
    let (host, id) = insert(host, Node::instantiate_part(doc_ref));
    // The referenced document moves on under the same id: the pin the
    // host carries no longer holds.
    store.insert(part("asm4-r3i-part", 0.0, 2.0), Tol::witness());
    match inline(
        &host,
        id,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    ) {
        Err(InlineError::Unresolved { failure }) => {
            assert_eq!(failure.fault, ResolveFault::PinMismatch);
        }
        other => panic!("expected Unresolved(PinMismatch), got {other:?}"),
    }
}

/// Row 3 (this unit's own found refusals) — the empty cut, the
/// colliding part id and the unknown cut node refuse; a consumed
/// instance, and an instance of plain geometry at a pose, inline.
#[test]
fn row3_further_typed_refusals() {
    let (_, doc, ids) = two_instance_assembly("asm4-r3f");
    assert!(matches!(
        split(
            &doc,
            &BTreeSet::new(),
            DocumentId::derive("n"),
            Tol::witness(),
            None
        ),
        Err(SplitError::EmptyCut)
    ));
    assert!(matches!(
        split(
            &doc,
            &BTreeSet::from([ids[0]]),
            doc.id(),
            Tol::witness(),
            None
        ),
        Err(SplitError::PartIdCollides { .. })
    ));
    assert!(matches!(
        split(
            &doc,
            &BTreeSet::from([RecipeNodeId::new(0, 999)]),
            DocumentId::derive("n"),
            Tol::witness(),
            None
        ),
        Err(SplitError::UnknownCutNode { .. })
    ));

    // A consumed instance inlines: a reader of one of its copies reads
    // the spliced copy, beside a block of the host's own.
    let mut store = PartStore::default();
    let doc_ref = store.insert(part("asm4-r3f-part", 0.0, 1.0), Tol::witness());
    let o = with_resolver(store.clone());
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(store);
    let host = ProfileDoc::empty(DocumentId::derive("asm4-r3f-host"), Tol::witness());
    let (host, inst) = pinned(host, doc_ref, &o);
    let (host, block) = insert(host, xy_frame());
    let (host, block_profile) = insert(
        host,
        Node::Profile(desc(block, vec![square(5.0, 0.0, 0.5)])),
    );
    let (host, block) = insert(
        host,
        Node::Extrude {
            profile: block_profile.into(),
            distance: len(1.0),
        },
    );
    let (host, consumer) = insert(
        host,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                editor_core::BodyRead::keyed(
                    editor_core::Operand::Node(inst),
                    plain_copy_key(&part("asm4-r3f-part", 0.0, 1.0)),
                ),
                block.into(),
            ]),
        },
    );
    let inlined = inline(&host, inst, &resolver, Tol::witness())
        .expect("an instance its reader reads one copy of inlines");
    let Some(Node::Union {
        members: editor_core::Bodies::Spelled(members),
    }) = inlined.doc.node(consumer)
    else {
        panic!("the reader stays")
    };
    assert!(
        inlined
            .doc
            .operation_of(members[0].read)
            .is_some_and(|at| at != inst),
        "the reader reads the spliced copy"
    );

    // An instance of a PLAIN-geometry part pinned at a pose inlines:
    // the part's one placement on its world takes the motion the pin
    // solved to, appended to its world mate as world moves (A4).
    let host2 = ProfileDoc::empty(DocumentId::derive("asm4-r3f-host2"), Tol::witness());
    let (host2, inst2) = pinned_at(host2, doc_ref, [3.0, 0.0, 0.0], &o);
    let inlined2 = inline(&host2, inst2, &resolver, Tol::witness())
        .expect("a pinned instance of a plain part inlines at any pose");
    let part_doc = part("asm4-r3f-part", 0.0, 1.0);
    let spliced = inlined2.node_map[&part_doc.placements()[0]];
    assert!(
        fixture::offset_of(&inlined2.doc, spliced).is_some(),
        "the plain extrude's placement keeps one world mate"
    );
    let landed = fixture::solve(&inlined2.doc, &EvalOptions::default(), Tol::witness())
        .placement(&inlined2.doc, spliced)
        .expect("the spliced placement solves");
    // The whole placement, rotation and translation, within the pin's
    // re-composition: measured 6.1e-17 (a quarter-turn's cosine, from
    // the spins `pin_at` writes the pose as).
    let want = editor_core::Frame::translation([3.0, 0.0, 0.0]);
    let off = landed
        .columns
        .iter()
        .flatten()
        .zip(want.columns.iter().flatten())
        .chain(landed.translation.iter().zip(&want.translation))
        .map(|(x, y)| (x - y).abs())
        .fold(0.0_f64, f64::max);
    assert!(
        off <= 1e-16,
        "the plain extrude's placement takes the instance's pose, {off:e} off: {landed:?}"
    );
    // The same instance at IDENTITY inlines too.
    let host3 = ProfileDoc::empty(DocumentId::derive("asm4-r3f-host3"), Tol::witness());
    let (host3, inst3) = pinned(host3, doc_ref, &o);
    inline(&host3, inst3, &resolver, Tol::witness())
        .expect("an identity-placed plain part inlines");
}

// ---- Row 4: placements and world pins, both sides ----

/// Row 4 — the A10 maintenance and A4's placement rules, through the
/// recorded edits, produce the expected placements and world pins on
/// both sides, each its own assertion.
#[test]
fn row4_placements_and_pins_land_as_the_rules_say() {
    // A one-instance cut moves as selected.
    let (store, doc, ids) = two_instance_assembly("asm4-r4");
    let out = split(
        &doc,
        &BTreeSet::from([ids[1]]),
        DocumentId::derive("asm4-r4-new"),
        Tol::witness(),
        with_resolver(store.clone()).resolver.as_ref(),
    )
    .expect("legal");
    let mapped = out.node_map[&ids[1]];
    assert_eq!(
        placed_bodies(&out.remainder),
        [ids[0], out.instance],
        "the remainder places the kept instance, then the new one"
    );
    assert_eq!(
        placed_bodies(&out.part),
        [mapped],
        "the part places what the cut placed"
    );
    assert!(
        left_at_identity(&out, &store),
        "the instance is pinned at the identity"
    );
    assert!(
        fixture::same_offset(&out.part, mapped, &doc, ids[1]),
        "the cut instance keeps its world pin in the part"
    );

    // The several-instance cut: both world pins MOVE, the remainder
    // instance sits at the identity, and the part keeps the cut
    // placements' order.
    let mut store = PartStore::default();
    let doc_ref = store.insert(part("asm4-r4b-part", 0.0, 1.0), Tol::witness());
    let o = with_resolver(store.clone());
    let doc2 = ProfileDoc::empty(DocumentId::derive("asm4-r4b"), Tol::witness());
    let (doc2, i0) = pinned(doc2, doc_ref, &o);
    let (doc2, i1) = pinned_at(doc2, doc_ref, [7.0, 0.0, 0.0], &o);
    let (doc2, i2) = pinned_at(doc2, doc_ref, [14.0, 0.0, 0.0], &o);
    let out2 = split(
        &doc2,
        &BTreeSet::from([i0, i2]),
        DocumentId::derive("asm4-r4b-new"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("legal");
    assert_eq!(
        placed_bodies(&out2.remainder),
        [i1, out2.instance],
        "the remainder's new placement comes after the kept one"
    );
    assert_eq!(
        placed_bodies(&out2.part),
        [out2.node_map[&i0], out2.node_map[&i2]],
        "the part keeps the cut placements' order"
    );
    assert!(
        left_at_identity(&out2, &store),
        "a several-instance cut leaves the remainder instance pinned at the identity"
    );
    assert!(
        fixture::same_offset(&out2.part, out2.node_map[&i2], &doc2, i2),
        "the moved world pin is verbatim"
    );
    assert!(
        fixture::same_offset(&out2.part, out2.node_map[&i0], &doc2, i0)
            && crate::p2_split::at_identity(&out2.part, out2.node_map[&i0], &o),
        "a cut instance at the identity stays there"
    );
}

// ---- Row 5: the interface-record hook ----

/// Row 5 — the empty crossing-declaration record exists on the
/// remainder's instance and round-trips persistence, costing no wire
/// bytes while empty (so no pin moves until R2 populates it).
#[test]
fn row5_interface_record_exists_and_round_trips() {
    let (store, doc, ids) = two_instance_assembly("asm4-r5");
    let out = split(
        &doc,
        &BTreeSet::from([ids[1]]),
        DocumentId::derive("asm4-r5-new"),
        Tol::witness(),
        with_resolver(store).resolver.as_ref(),
    )
    .expect("legal");
    match out.remainder.node(out.instance) {
        Some(Node::InstantiatePart { interface, .. }) => {
            assert!(interface.is_empty(), "no mate vocabulary exists to cross");
        }
        other => panic!("expected the instance, got {other:?}"),
    }
    let text = save(&out.remainder, &[], Tol::witness()).expect("saves");
    assert!(
        !text.contains("\"interface\""),
        "the empty record is absent from the wire (pin stability)"
    );
    let loaded = load(&text, Tol::witness()).expect("loads");
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
    let (store, doc, ids) = two_instance_assembly("asm4-per");
    let opts = with_resolver(store);
    let ev1 = run(&doc, &opts);
    let (body1, _) = product_named(&doc, &ev1, Tol::witness()).expect("gathers");

    let out = split(
        &doc,
        &BTreeSet::from([ids[1]]),
        DocumentId::derive("asm4-per-new"),
        Tol::witness(),
        opts.resolver.as_ref(),
    )
    .expect("legal");
    let part_loaded = load(
        &save(&out.part, &[], Tol::witness()).expect("part saves"),
        Tol::witness(),
    )
    .expect("part loads");
    let rem_loaded = load(
        &save(&out.remainder, &[], Tol::witness()).expect("remainder saves"),
        Tol::witness(),
    )
    .expect("remainder loads");
    assert!(part_loaded.doc.bit_eq(&out.part));
    assert!(rem_loaded.doc.bit_eq(&out.remainder));
    assert_eq!(
        content_pin(&part_loaded.doc, Tol::witness()).expect("pins"),
        content_pin(&out.part, Tol::witness()).expect("pins"),
        "the pin is stable across the wire"
    );

    let mut store2 = PartStore::default();
    store2.insert(part("asm4-per-part", 0.0, 1.0), Tol::witness());
    store2.insert(part_loaded.doc, Tol::witness());
    let ev2 = run(&rem_loaded.doc, &with_resolver(store2));
    let (body2, _) = product_named(&rem_loaded.doc, &ev2, Tol::witness()).expect("gathers");
    assert_eq!(census(&body1), census(&body2));
    same_volume(&body1, &body2, "the loaded pair keeps the volume");
}

// ---- A cut a kept placement separates ----

/// A non-adjacent several-instance cut's placements leave together and come
/// back together, after the kept ones, so the round trip returns the
/// document up to node ids and that one regrouping: the placed set and
/// the cut's own order are kept, the interleaving with the kept
/// placement is not, and the census, bit-equal volumes and whole-table
/// name re-resolution all hold.
#[test]
fn a_separated_cut_comes_back_after_the_kept_placements_and_the_round_trip_keeps_the_product() {
    let mut store = PartStore::default();
    let doc_ref = store.insert(part("asm4-min1-part", 0.0, 1.0), Tol::witness());
    // A plain component (dyadic-exact volume, disjoint from the
    // instances) plus three pinned instances.
    let doc = ProfileDoc::empty(DocumentId::derive("asm4-min1"), Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(5.0, 0.0, 0.5)],
    );
    let (doc, e) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        },
    );
    let doc = fixture::place(doc, e).0;
    let opts = with_resolver(store);
    let (doc, i0) = pinned_at(doc, doc_ref, [10.0, 0.0, 0.0], &opts);
    let (doc, i1) = pinned_at(doc, doc_ref, [20.0, 0.0, 0.0], &opts);
    let (doc, i2) = pinned_at(doc, doc_ref, [30.0, 0.0, 0.0], &opts);
    let ev1 = run(&doc, &opts);
    let (body1, names1) = product_named(&doc, &ev1, Tol::witness()).expect("gathers");

    let cut = BTreeSet::from([i0, i2]);
    let out = split(
        &doc,
        &cut,
        DocumentId::derive("asm4-min1-new"),
        Tol::witness(),
        opts.resolver.as_ref(),
    )
    .expect("legal");
    assert_eq!(
        placed_bodies(&out.remainder),
        [e, i1, out.instance],
        "the kept placements keep their order; the new one comes last"
    );
    assert_eq!(
        placed_bodies(&out.part),
        [out.node_map[&i0], out.node_map[&i2]],
        "the part keeps the cut placements' host order"
    );

    let mut store2 = PartStore::default();
    store2.insert(part("asm4-min1-part", 0.0, 1.0), Tol::witness());
    store2.insert(out.part.clone(), Tol::witness());
    let inlined = inline(
        &out.remainder,
        out.instance,
        &(Arc::new(store2) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    )
    .expect("inlines");
    let back = |i: RecipeNodeId| inlined.node_map[&out.node_map[&i]];
    // [e, i0, i1, i2] round-trips to [e, i1, i0, i2] (in
    // correspondence): the part's placements come in after the kept
    // ones, in their own order.
    assert_eq!(
        placed_bodies(&inlined.doc),
        [e, i1, back(i0), back(i2)],
        "the carried placements keep their own order, after the kept ones"
    );

    // The product and its names are the original's.
    let mut store3 = PartStore::default();
    store3.insert(part("asm4-min1-part", 0.0, 1.0), Tol::witness());
    let ev3 = run(&inlined.doc, &with_resolver(store3));
    let (body3, names3) = product_named(&inlined.doc, &ev3, Tol::witness()).expect("gathers");
    assert_eq!(census(&body1), census(&body3));
    assert_eq!(volume_bits(&body1), volume_bits(&body3));
    for (name, _) in names1.iter() {
        let expected = if name.node == i0 || name.node == i2 {
            StableName {
                kind: name.kind,
                node: back(name.node),
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
    // And the moved world pins ride verbatim, both hops.
    assert!(
        fixture::same_offset(&out.part, out.node_map[&i0], &doc, i0),
        "the moved world pin is verbatim in the part"
    );
    assert!(
        fixture::same_offset(&inlined.doc, back(i2), &doc, i2),
        "the round trip restores the world pin bit for bit"
    );
}

// ---- MIN-2 (review round 1): the untested refusal arms ----

/// Split's three name-classification refusals: each constructs its
/// case, asserts the typed arm, and asserts the message names its
/// subject.
#[test]
fn split_name_refusals_fire_typed_and_name_their_subjects() {
    use editor_core::EntityKind;
    // BodyNameCrossesCut: appearance keyed by a cut instance's BODY
    // name (product tables carry no body rows, so the wrapped
    // rewrite could never resolve).
    let (store, doc, ids) = two_instance_assembly("asm4-min2-body");
    let body_name = StableName {
        kind: EntityKind::Body,
        node: ids[1],
        path: vec![RoleSeg::OutputBody],
    };
    let read = crate::fixture::painted_in(&doc, &body_name);
    let (doc, _) = step(
        doc,
        DocEdit::SetAppearance {
            body: read,
            name: body_name.clone(),
            attr: editor_core::Attr::Visibility(false),
        },
    );
    match split(
        &doc,
        &BTreeSet::from([ids[1]]),
        DocumentId::derive("n"),
        Tol::witness(),
        with_resolver(store).resolver.as_ref(),
    ) {
        Err(SplitError::BodyNameCrossesCut { name }) => {
            assert_eq!(name.name(), &body_name);
            let msg = format!("{}", SplitError::BodyNameCrossesCut { name });
            assert!(
                msg.contains(&format!("the output body of {}", doc.spoken(ids[1]))),
                "the message names the name: {msg}"
            );
            assert!(
                msg.contains("the output body of"),
                "the message states the class: {msg}"
            );
        }
        other => panic!("expected BodyNameCrossesCut, got {other:?}"),
    }

    // NameStraddlesCut: a name a KEPT fillet selects derives from a
    // kept node AND (through an embedded operand name) from a cut node.
    let doc = part("asm4-min2-straddle-kept", 0.0, 1.0);
    let kept_e = doc.ids()[BODY_POSITION];
    let (doc, cut_f) = insert(doc, xy_frame());
    let (doc, cut_p) = insert(
        doc,
        Node::Profile(desc(cut_f, vec![square(10.0, 0.0, 0.5)])),
    );
    let (doc, cut_e) = insert(
        doc,
        Node::Extrude {
            profile: cut_p.into(),
            distance: len(1.0),
        },
    );
    let straddler = StableName {
        kind: EntityKind::Edge,
        node: kept_e,
        path: vec![RoleSeg::From {
            read: crate::fixture::out(&doc, cut_e),
            of: StableName {
                kind: EntityKind::Edge,
                node: cut_e,
                path: vec![RoleSeg::OutputBody],
            }
            .into(),
        }],
    };
    let (doc, _) = insert(
        doc,
        Node::Fillet {
            radius: len(0.1),
            selection: editor_core::Operand::select(kept_e, vec![straddler.clone()]),
        },
    );
    match split(
        &doc,
        &BTreeSet::from([cut_f, cut_p, cut_e]),
        DocumentId::derive("n"),
        Tol::witness(),
        None,
    ) {
        Err(SplitError::NameStraddlesCut { name, missing }) => {
            assert_eq!(name.name(), &straddler);
            assert_eq!(
                missing, None,
                "the straddle classification weighs the whole derivation set, so it singles \
                 out no one node"
            );
            let msg = format!("{}", SplitError::NameStraddlesCut { name, missing });
            assert!(
                msg.contains("both sides"),
                "the message states the fault: {msg}"
            );
        }
        other => panic!("expected NameStraddlesCut, got {other:?}"),
    }

    // PartNameReachesRemainder: a name a CUT fillet selects is a KEPT
    // node's entity — the part document could not express the
    // reference.
    let doc = part("asm4-min2-reach-kept", 0.0, 1.0);
    let kept_e = doc.ids()[BODY_POSITION];
    let reaching = StableName {
        kind: EntityKind::Edge,
        node: kept_e,
        path: vec![RoleSeg::OutputBody],
    };
    let (doc, cut) = selecting_fillet(doc, reaching.clone());
    let fillet = fillet_of(&doc, &cut);
    match split(&doc, &cut, DocumentId::derive("n"), Tol::witness(), None) {
        Err(SplitError::PartNameReachesRemainder {
            node,
            name,
            missing,
        }) => {
            assert_eq!(node, doc.spoken(fillet));
            assert_eq!(name.name(), &reaching);
            assert_eq!(
                missing,
                doc.spoken(kept_e),
                "and the node outside the cut that it reaches, which the carrier id does not \
                 say"
            );
            let msg = format!(
                "{}",
                SplitError::PartNameReachesRemainder {
                    node,
                    name,
                    missing
                }
            );
            assert!(
                msg.contains(&doc.spoken(fillet).to_string()) && msg.contains("outside the cut"),
                "the message names the site and the fault: {msg}"
            );
        }
        other => panic!("expected PartNameReachesRemainder, got {other:?}"),
    }
}

/// **A cut name reaching outside the cut names the earliest node it
/// reaches**: the least id, which is the node placed first. The name
/// is a union's face kept from operand A, so it reaches the union and
/// A's block, and A's block is named.
#[test]
fn a_reaching_name_names_the_earliest_node_outside_the_cut_in_document_order() {
    use editor_core::{EntityKind, derivation_nodes};
    let doc = ProfileDoc::empty_derived("asm4-reach-earliest", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.25, 0.75), 0.25, 0.5);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    let table = ev.table_at(u).expect("the union evaluates");
    let a_read = crate::fixture::out(&doc, a);
    let from_a = table
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| {
            n.kind == EntityKind::Edge
                && n.path
                    .first()
                    .is_some_and(|s| matches!(s, RoleSeg::From { read, .. } if *read == a_read))
        })
        .expect("the union keeps an edge of operand A");
    let reached = derivation_nodes(&from_a);
    assert!(reached.contains(&u) && reached.contains(&a), "{reached:?}");
    let (doc, decl) = selecting_fillet(doc, from_a.clone());
    match split(
        &doc,
        &decl,
        DocumentId::derive("asm4-reach-earliest-part"),
        Tol::witness(),
        None,
    ) {
        Err(SplitError::PartNameReachesRemainder {
            node,
            name,
            missing,
        }) => {
            assert_eq!(node, doc.spoken(fillet_of(&doc, &decl)));
            assert_eq!(name.name(), &from_a);
            assert_eq!(
                missing,
                doc.spoken(a),
                "the node named is the one the document holds first"
            );
        }
        other => panic!("expected PartNameReachesRemainder, got {other:?}"),
    }
}

/// A cut-local block and a fillet on it whose selection is `name`,
/// a frozen name with no edge to the node that minted it: the cut a
/// split takes to carry `name` into a part without carrying its
/// minter.
fn selecting_fillet(doc: ProfileDoc, name: StableName) -> (ProfileDoc, BTreeSet<RecipeNodeId>) {
    let before: BTreeSet<RecipeNodeId> = doc.ids().iter().copied().collect();
    let (doc, body) = block(doc, (5.0, 6.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, _fillet) = insert(
        doc,
        Node::Fillet {
            radius: len(0.1),
            selection: editor_core::Operand::select(body, vec![name]),
        },
    );
    let cut = doc
        .ids()
        .iter()
        .copied()
        .filter(|id| !before.contains(id))
        .collect();
    (doc, cut)
}

/// The fillet in a cut [`selecting_fillet`] took.
fn fillet_of(doc: &ProfileDoc, cut: &BTreeSet<RecipeNodeId>) -> RecipeNodeId {
    *cut.iter()
        .find(|id| matches!(doc.node(**id), Some(Node::Fillet { .. })))
        .expect("the cut holds its fillet")
}

/// **A deleted node the name reaches is named after every live one**:
/// a deleted node has no place in the document, so the live node is
/// the one the author can still act on. The union whose face the cut
/// names is deleted (the name stays, a DM7 strand), so a pick that let
/// the deleted minter answer would name the union. The union is
/// inserted after the block it names, so its id sorts after the
/// block's too: this shape holds the live answer, and cannot build a
/// deleted node that sorts first.
#[test]
fn a_reaching_name_names_a_live_node_before_a_deleted_one() {
    use editor_core::{EntityKind, derivation_nodes};
    let doc = ProfileDoc::empty_derived("asm4-reach-deleted", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.25, 0.75), 0.25, 0.5);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    let table = ev.table_at(u).expect("the union evaluates");
    let a_read = crate::fixture::out(&doc, a);
    let from_a = table
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| {
            n.kind == EntityKind::Edge
                && n.path
                    .first()
                    .is_some_and(|s| matches!(s, RoleSeg::From { read, .. } if *read == a_read))
        })
        .expect("the union keeps an edge of operand A");
    let reached = derivation_nodes(&from_a);
    assert!(reached.contains(&u) && reached.contains(&a), "{reached:?}");
    let (doc, decl) = selecting_fillet(doc, from_a.clone());
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: u });
    assert!(doc.node(u).is_none(), "the union is gone, its name stays");
    match split(
        &doc,
        &decl,
        DocumentId::derive("asm4-reach-deleted-part"),
        Tol::witness(),
        None,
    ) {
        Err(SplitError::PartNameReachesRemainder { missing, .. }) => assert_eq!(
            missing,
            doc.spoken(a),
            "the live node, not the deleted union"
        ),
        other => panic!("expected PartNameReachesRemainder, got {other:?}"),
    }
}

/// Inline's parameter, tolerance, and metadata refusals.
#[test]
fn inline_param_epsilon_and_metadata_refusals_fire_typed() {
    // VarNameConflict: both documents declare "L", bit-different values.
    let mut store = PartStore::default();
    let part_doc = part("asm4-min2-param-part", 0.0, 1.0);
    let (part_doc, _) = step(
        part_doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("L"),
            def: editor_core::VarDecl::Free(FreeVar::continuous(
                editor_core::Dimension::Length,
                2.0,
            )),
        },
    );
    let doc_ref = store.insert(part_doc, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("asm4-min2-param-host"), Tol::witness());
    let (host, _) = step(
        host,
        DocEdit::DeclareVar {
            name: VarName::from_static("L"),
            def: editor_core::VarDecl::Free(FreeVar::continuous(
                editor_core::Dimension::Length,
                1.0,
            )),
        },
    );
    let (host, inst) = pinned(host, doc_ref, &with_resolver(store.clone()));
    match inline(
        &host,
        inst,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    ) {
        Err(InlineError::VarNameConflict { name }) => {
            assert_eq!(name, VarName::from_static("L"));
            let msg = format!("{}", InlineError::VarNameConflict { name });
            assert!(
                msg.contains("both documents hold a variable named L"),
                "the message names the variable: {msg}"
            );
        }
        other => panic!("expected VarNameConflict, got {other:?}"),
    }

    // EpsilonSeam: the referenced document records a different ε (the
    // stub store, unlike the workspace, has no load door to refuse it
    // earlier — the inline door is the backstop under test).
    let mut store = PartStore::default();
    let part_doc = part("asm4-min2-eps-part", 0.0, 1.0);
    let host = ProfileDoc::empty(DocumentId::derive("asm4-min2-eps-host"), Tol::witness());
    let moved_eps = host.epsilon() * 0.5;
    let (part_doc, _) = step(part_doc, DocEdit::SetTolerance { eps: moved_eps });
    let doc_ref = store.insert(part_doc, Tol::witness());
    let (host, inst) = insert(host, Node::instantiate_part(doc_ref));
    match inline(
        &host,
        inst,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    ) {
        Err(InlineError::EpsilonSeam { host_eps, part_eps }) => {
            assert_eq!(host_eps.to_bits(), host.epsilon().to_bits());
            assert_eq!(part_eps.to_bits(), moved_eps.to_bits());
            let msg = format!("{}", InlineError::EpsilonSeam { host_eps, part_eps });
            assert!(
                msg.contains("tolerance"),
                "the message states the seam: {msg}"
            );
        }
        other => panic!("expected EpsilonSeam, got {other:?}"),
    }

    // PartCarriesMetadata: no edit arm writes document metadata, so
    // the carrying document is authored through the wire (inject the
    // key into a saved snapshot and load it back).
    let mut store = PartStore::default();
    let part_doc = part("asm4-min2-meta-part", 0.0, 1.0);
    let text = save(&part_doc, &[], Tol::witness()).expect("saves");
    let injected = text.replace(
        "\"metadata\": {}",
        "\"metadata\": {\n    \"note\": \"x\"\n  }",
    );
    assert_ne!(text, injected, "the fixture's metadata key was found");
    let carrying = load(&injected, Tol::witness())
        .expect("the carrying document loads")
        .doc;
    assert_eq!(carrying.metadata().len(), 1);
    let doc_ref = store.insert(carrying, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("asm4-min2-meta-host"), Tol::witness());
    let (host, inst) = insert(host, Node::instantiate_part(doc_ref));
    match inline(
        &host,
        inst,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    ) {
        Err(InlineError::PartCarriesMetadata { key }) => {
            assert_eq!(key, "note");
            let msg = format!("{}", InlineError::PartCarriesMetadata { key });
            assert!(msg.contains("\"note\""), "the message names the key: {msg}");
        }
        other => panic!("expected PartCarriesMetadata, got {other:?}"),
    }
}

/// Inline's name-shape refusals: the foreign derivation, the
/// instance's own body name, and the stranded part-side reference.
#[test]
fn inline_name_refusals_fire_typed_and_name_their_subjects() {
    use editor_core::EntityKind;
    let mut store = PartStore::default();
    let doc_ref = store.insert(part("asm4-min2-name-part", 0.0, 1.0), Tol::witness());
    let o = with_resolver(store.clone());
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(store);

    // ForeignInstanceName: a host reference DERIVES from the instance
    // but is not the bridge's own wrapped form.
    let host = part("asm4-min2-name-host", 5.0, 1.0);
    let kept_e = host.ids()[BODY_POSITION];
    let (host, inst) = pinned(host, doc_ref, &o);
    let foreign = StableName {
        kind: EntityKind::Face,
        node: kept_e,
        path: vec![RoleSeg::From {
            read: crate::fixture::out(&host, inst),
            of: wrap(
                inst,
                &StableName {
                    kind: EntityKind::Face,
                    node: RecipeNodeId::new(0, 0),
                    path: vec![RoleSeg::OutputBody],
                },
            )
            .into(),
        }],
    };
    let read = crate::fixture::painted_in(&host, &foreign);
    let (host, _) = step(
        host,
        DocEdit::SetAppearance {
            body: read,
            name: foreign.clone(),
            attr: editor_core::Attr::Visibility(false),
        },
    );
    let host = labelled(host, kept_e, "host plate");
    match inline(&host, inst, &resolver, Tol::witness()) {
        Err(InlineError::ForeignInstanceName { name }) => {
            assert_eq!(name.name(), &foreign);
            assert_eq!(
                minter_label(&name),
                Some("host plate"),
                "a host name is spoken from the host: {name}"
            );
            let msg = format!("{}", InlineError::ForeignInstanceName { name });
            assert!(
                msg.contains("InPart"),
                "the message states the expected form: {msg}"
            );
        }
        other => panic!("expected ForeignInstanceName, got {other:?}"),
    }

    // InstanceBodyNameReferenced: the instance's own output-body name
    // has no spliced correspondent.
    let host = ProfileDoc::empty(DocumentId::derive("asm4-min2-body-host"), Tol::witness());
    let (host, inst) = pinned(host, doc_ref, &o);
    let body_name = StableName {
        kind: EntityKind::Body,
        node: inst,
        path: vec![RoleSeg::OutputBody],
    };
    let read = crate::fixture::painted_in(&host, &body_name);
    let (host, _) = step(
        host,
        DocEdit::SetAppearance {
            body: read,
            name: body_name.clone(),
            attr: editor_core::Attr::Visibility(false),
        },
    );
    let host = labelled(host, inst, "left leg");
    match inline(&host, inst, &resolver, Tol::witness()) {
        Err(InlineError::InstanceBodyNameReferenced { name }) => {
            assert_eq!(name.name(), &body_name);
            assert_eq!(
                minter_label(&name),
                Some("left leg"),
                "the instance's body name is the host's, spoken from it: {name}"
            );
            let msg = format!("{}", InlineError::InstanceBodyNameReferenced { name });
            assert!(
                msg.contains("output body"),
                "the message states the fault: {msg}"
            );
        }
        other => panic!("expected InstanceBodyNameReferenced, got {other:?}"),
    }

    // StrandedPartName / StrandedPartRead: the referenced document
    // carries an N5-stranded selected name (its node deleted after
    // authoring) — there is nothing to remap it onto. BOTH shapes
    // run. The FLAT name is minted AT the deleted node, so the node the
    // refusal carries is the name's own mint; the NESTED name is minted
    // at the SURVIVING body and carries the deleted node's name in
    // through that node's read, so what stranded is a segment DOWN from
    // the name, and the refusal names it separately: the read, which no
    // operation of the part defines any more.
    for nested in [false, true] {
        let mut store = PartStore::default();
        let part_doc = part("asm4-min2-stranded-part", 0.0, 1.0);
        let (part_doc, extra_profile) = on_frame(
            part_doc,
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(10.0, 0.0, 0.5)],
        );
        // A body whose name a fillet on the part body selects; the
        // delete strands it.
        let (part_doc, extra) = insert(
            part_doc,
            Node::Extrude {
                profile: extra_profile.into(),
                distance: len(1.0),
            },
        );
        let body = part_doc.ids()[BODY_POSITION];
        let at_extra = StableName {
            kind: EntityKind::Edge,
            node: extra,
            path: vec![RoleSeg::OutputBody],
        };
        let extra_read = crate::fixture::out(&part_doc, extra);
        let stranded = if nested {
            StableName {
                kind: EntityKind::Edge,
                node: body,
                path: vec![RoleSeg::From {
                    read: extra_read,
                    of: at_extra.into(),
                }],
            }
        } else {
            at_extra
        };
        // The selection is READ at the surviving body; the stranded
        // NAME derives from the extra node, which is what the delete
        // below strands.
        let (part_doc, _) = insert(
            part_doc,
            Node::Fillet {
                radius: len(0.1),
                selection: editor_core::Operand::select(body, vec![stranded.clone()]),
            },
        );
        let (part_doc, _) = step(part_doc, DocEdit::DeleteNode { id: extra });
        let part_doc = labelled(part_doc, body, "part body");
        let doc_ref = store.insert(part_doc, Tol::witness());
        let o = with_resolver(store.clone());
        let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(store);
        let host = ProfileDoc::empty(
            DocumentId::derive("asm4-min2-stranded-host"),
            Tol::witness(),
        );
        let (host, inst) = pinned(host, doc_ref, &o);
        match inline(&host, inst, &resolver, Tol::witness()) {
            // NESTED: the segment carries the name in through the deleted
            // node's read, and the read is the first local id of the
            // segment the walk meets; with its node gone nothing defines
            // it, so the refusal names the read itself.
            Err(InlineError::StrandedPartRead { name, read }) if nested => {
                assert_eq!(name.name(), &stranded);
                assert_eq!(
                    read.id(),
                    extra_read,
                    "the refusal carries the read the deleted node defined"
                );
                assert_eq!(
                    minter_label(&name),
                    Some("part body"),
                    "a carried name is the part's, spoken from it: {name}"
                );
                let msg = format!("{}", InlineError::StrandedPartRead { name, read });
                assert!(
                    msg.contains("no operation of the referenced document defines"),
                    "the message states the fault: {msg}"
                );
            }
            Err(InlineError::StrandedPartName { name, missing }) if !nested => {
                assert_eq!(name.name(), &stranded);
                assert_eq!(
                    missing,
                    editor_core::SpokenNode::absent(extra),
                    "the refusal carries the deleted node, which the part no longer holds, \
                     nested={nested}"
                );
                assert_eq!(
                    missing.id(),
                    name.name().node,
                    "flat: the name IS minted at the stranded node, so the two coincide — \
                     the case that cannot tell the id from the name"
                );
                let msg = format!("{}", InlineError::StrandedPartName { name, missing });
                assert!(
                    msg.contains("no longer has"),
                    "the message states the fault: {msg}"
                );
            }
            other => panic!(
                "expected StrandedPartName (flat) or StrandedPartRead (nested), \
                 nested={nested}, got {other:?}"
            ),
        }
    }
}

// ---- Profile step ids across the refactorings ----

/// A `part` with a second component (a half-block at x = 10) whose
/// profile has been reshaped once, its step at `position` re-minted,
/// and a face frame on that step's wall added FIRST when `stranded` —
/// so the reshaping strands the frame's name. Returns the document and
/// the second component's `(frame, profile, extrude, face frame)`.
fn reshaped_component(
    label: &str,
    position: usize,
    stranded: bool,
) -> (ProfileDoc, [RecipeNodeId; 3], Option<RecipeNodeId>) {
    let doc = part(label, 0.0, 1.0);
    let (doc, f2) = insert(doc, xy_frame());
    let (doc, p2) = insert(doc, Node::Profile(desc(f2, vec![square(10.0, 0.0, 0.5)])));
    let (doc, e2) = insert(
        doc,
        Node::Extrude {
            profile: p2.into(),
            distance: len(1.0),
        },
    );
    let program = match doc.node(p2) {
        Some(Node::Profile(p)) => p.clone(),
        other => panic!("a profile, got {other:?}"),
    };
    let (doc, face_frame) = if stranded {
        let dropped = editor_core::ProfileEdgeRef::Piece {
            step: program.ids[0][position],
            role: editor_core::PieceRole::Leg,
        };
        let (doc, id) = insert(
            doc,
            Node::Datum(editor_core::Datum::FaceFrame {
                face: editor_core::Operand::select(
                    e2,
                    vec![fixture::fname(e2, RoleSeg::Lateral(dropped.into()))],
                ),
                spin: fixture::ang(0.0),
            }),
        );
        (doc, Some(id))
    } else {
        (doc, None)
    };
    let mut ids = program.kept_in_place();
    ids[0][position] = None;
    let (doc, _) = step(
        doc,
        DocEdit::SetProgram {
            node: p2,
            loops: program
                .loops
                .iter()
                .map(editor_core::LoopProgram::authored)
                .collect(),
            ids,
            fresh: Vec::new(),
        },
    );
    // Placed once reshaped: a pin is read off the body's own edges, so
    // a pin written before the reshape would read a step it drops.
    let doc = fixture::place(doc, e2).0;
    (doc, [f2, p2, e2], face_frame)
}

/// The ids a profile node holds, flattened in loop then step order.
/// `doc` with `node` labelled `text`, so a sentence spoken from the
/// wrong document would read a different label or none.
fn labelled(doc: ProfileDoc, node: RecipeNodeId, text: &str) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetLabel {
            node,
            label: Some(editor_core::Label::new(text).expect("a valid label")),
        },
    )
    .0
}

/// The label a spoken name's minting node carries, as text.
fn minter_label(name: &editor_core::SpokenName) -> Option<&str> {
    name.minter().label().map(editor_core::Label::as_str)
}

fn flat_ids(doc: &ProfileDoc, profile: RecipeNodeId) -> Vec<editor_core::StepId> {
    match doc.node(profile) {
        Some(Node::Profile(p)) => p.ids.iter().flatten().copied().collect(),
        other => panic!("a profile, got {other:?}"),
    }
}

/// **A split re-mints the cut profiles' steps from the part's own mint,
/// and its step map says so, however the source minted them.** The cut
/// profile's step 2 was re-minted by a reshaping, so its ids come from
/// two different edits. The part document mints its one profile's five
/// ids from its own empty mint and no more, and the step map pairs each
/// old id with the id the part's program holds in the same position;
/// the same split minted twice mints the same ids (D9).
#[test]
fn a_split_step_map_follows_a_non_contiguous_re_mint() {
    let (doc, [f2, p2, e2], _) = reshaped_component("asm4-steps", 2, false);
    let old = flat_ids(&doc, p2);
    let n = 5;
    assert_eq!(old.len(), n, "one id per authored step");
    let cut = || {
        split(
            &doc,
            &BTreeSet::from([f2, p2, e2]),
            DocumentId::derive("asm4-steps-new"),
            Tol::witness(),
            None,
        )
        .expect("legal")
    };
    let out = cut();
    let part_profile = out.node_map[&p2];
    let minted = flat_ids(&out.part, part_profile);
    assert_eq!(
        out.part.mint().steps().collect::<Vec<_>>(),
        minted
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>(),
        "the part's mint log holds exactly its one profile's {n} ids"
    );
    assert_eq!(minted.len(), n);
    let expected: editor_core::StepMap = old.iter().copied().zip(minted.clone()).collect();
    assert_eq!(out.step_map, expected);
    assert_eq!(
        flat_ids(&cut().part, part_profile),
        minted,
        "one split mints one set of ids"
    );
}

/// **A name on a step a `SetProgram` dropped does not cross a split or
/// an inline**: no carried profile holds the step, so the step map has
/// no id for it, and each refactoring refuses typed, naming the
/// stranded name and the dropped step.
#[test]
fn a_name_on_a_dropped_step_refuses_a_split_and_an_inline() {
    let (doc, [f2, p2, e2], face_frame) = reshaped_component("asm4-dropped", 1, true);
    let doc = labelled(doc, e2, "walled block");
    let face_frame = face_frame.expect("the stranded frame");
    let dropped = match doc.node(face_frame) {
        Some(Node::Datum(editor_core::Datum::FaceFrame { face, .. })) => {
            doc.selection(*face).expect("a face selection").names[0].clone()
        }
        other => panic!("a face frame, got {other:?}"),
    };
    let dropped_step = match dropped.path.as_slice() {
        [RoleSeg::Lateral(run)] => match run.single() {
            Some(editor_core::ProfileEdgeRef::Piece { step, .. }) => step,
            other => panic!("a wall spelled by one piece, got {other:?}"),
        },
        other => panic!("a wall spelled by a piece, got {other:?}"),
    };
    assert!(
        !flat_ids(&doc, p2).contains(&dropped_step),
        "the frame spells a step the profile no longer holds"
    );
    match split(
        &doc,
        &BTreeSet::from([f2, p2, e2, face_frame]),
        DocumentId::derive("asm4-dropped-new"),
        Tol::witness(),
        None,
    ) {
        Err(SplitError::NameOnDroppedStep { name, step }) => {
            assert_eq!((name.name(), step), (&dropped, dropped_step));
            assert_eq!(minter_label(&name), Some("walled block"), "{name}");
        }
        other => panic!("expected NameOnDroppedStep, got {other:?}"),
    }

    let mut store = PartStore::default();
    let doc_ref = store.insert(doc, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("asm4-dropped-host"), Tol::witness());
    let (host, instance) = pinned(host, doc_ref, &with_resolver(store.clone()));
    match inline(
        &host,
        instance,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    ) {
        Err(InlineError::NameOnDroppedStep { name, step }) => {
            assert_eq!((name.name(), step), (&dropped, dropped_step));
            assert_eq!(
                minter_label(&name),
                Some("walled block"),
                "a carried name is the part's, spoken from it: {name}"
            );
        }
        other => panic!("expected NameOnDroppedStep, got {other:?}"),
    }
}

/// **A split-out part takes an appearance key's read with it, and the
/// key follows** (PR #4674 review): a key is never moved off its read by
/// a Rebind, so the split re-attaches a key read in a cut body in the
/// instance's body, and the paint still lands on the remainder.
#[test]
fn a_split_reattaches_an_appearance_key_onto_the_instances_body() {
    let (store, doc, ids) = two_instance_assembly("asm4-split-reattach");
    let opts = with_resolver(store);
    let ev = run(&doc, &opts);
    let face = ev
        .table_at(ids[1])
        .expect("the instance evaluated")
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| n.kind == editor_core::EntityKind::Face)
        .expect("a face name");
    let read = crate::fixture::painted_in(&doc, &face);
    let (doc, _) = step(
        doc,
        DocEdit::SetAppearance {
            body: read,
            name: face,
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
    );
    assert!(run(&doc, &opts).appearance.is_lossless(), "premise");
    let cut = BTreeSet::from([ids[1]]);
    let out = split(
        &doc,
        &cut,
        DocumentId::derive("asm4-split-reattach-new"),
        Tol::witness(),
        opts.resolver.as_ref(),
    )
    .expect("legal");
    let keys: Vec<_> = out.remainder.appearance().keys().cloned().collect();
    assert_eq!(keys.len(), 1, "one key, re-attached: {keys:?}");
    assert_eq!(
        out.remainder.operation_of(keys[0].at),
        Some(out.instance),
        "the key reads the instance's body"
    );
    let mut store2 = PartStore::default();
    store2.insert(part("asm4-split-reattach-part", 0.0, 1.0), Tol::witness());
    store2.insert(out.part.clone(), Tol::witness());
    let ev2 = run(&out.remainder, &with_resolver(store2));
    assert!(
        ev2.appearance.is_lossless(),
        "the paint still lands: {:?}",
        ev2.appearance.losses
    );
}

/// **The same for inline**: a host key painted in the instance's body is
/// re-attached in the inlined body, so the paint lands after the
/// instance is gone.
#[test]
fn an_inline_reattaches_an_appearance_key_onto_the_inlined_body() {
    let (store, doc, ids) = two_instance_assembly("asm4-inline-reattach");
    let cut = BTreeSet::from([ids[1]]);
    let out = split(
        &doc,
        &cut,
        DocumentId::derive("asm4-inline-reattach-new"),
        Tol::witness(),
        with_resolver(store).resolver.as_ref(),
    )
    .expect("legal");
    let mut store2 = PartStore::default();
    store2.insert(part("asm4-inline-reattach-part", 0.0, 1.0), Tol::witness());
    store2.insert(out.part.clone(), Tol::witness());
    let opts = with_resolver(store2.clone());
    let ev = run(&out.remainder, &opts);
    let face = ev
        .table_at(out.instance)
        .expect("the instance evaluated")
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| n.kind == editor_core::EntityKind::Face)
        .expect("a face name");
    let read = out
        .remainder
        .read_of_node(out.instance)
        .expect("the instance's read");
    let (host, _) = step(
        out.remainder.clone(),
        DocEdit::SetAppearance {
            body: read,
            name: face,
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
    );
    assert!(run(&host, &opts).appearance.is_lossless(), "premise");
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(store2);
    let inlined = inline(&host, out.instance, &resolver, Tol::witness()).expect("legal");
    let keys: Vec<_> = inlined.doc.appearance().keys().cloned().collect();
    assert_eq!(keys.len(), 1, "one key, re-attached: {keys:?}");
    assert!(
        inlined.doc.operation_of(keys[0].at).is_some(),
        "the key reads a live body"
    );
    let ev2 = run(&inlined.doc, &opts);
    assert!(
        ev2.appearance.is_lossless(),
        "the paint still lands: {:?}",
        ev2.appearance.losses
    );
}

/// Whether `map` keeps the order of every pair of the ids it maps
/// (keys ascend, so their images must).
fn keeps_id_order(map: &editor_core::StepMap) -> bool {
    let images: Vec<_> = map.values().collect();
    images.windows(2).all(|w| w[0] < w[1])
}

/// The lateral edge of extrude `body` that starts at `step`'s leg.
fn lateral_edge(body: RecipeNodeId, step: editor_core::StepId) -> StableName {
    StableName {
        kind: editor_core::EntityKind::Edge,
        node: body,
        path: vec![RoleSeg::LateralEdge(editor_core::ProfileVertexRef::Piece {
            step,
            role: editor_core::PieceRole::Leg,
        })],
    }
}

/// A pair boolean's ranked `EdgeCrossing` run over two lateral edges of
/// `body`, told apart by the steps `a` and `b`, in name order.
fn ranked_run(
    body: RecipeNodeId,
    minter: RecipeNodeId,
    a: editor_core::StepId,
    b: editor_core::StepId,
) -> StableName {
    let mut run: Vec<editor_core::RunLine> = [a, b]
        .into_iter()
        .map(|step| editor_core::RunLine {
            line: editor_core::NameRef::new(lateral_edge(body, step)),
            sense: Some(editor_core::Sense::Outside),
        })
        .collect();
    run.sort();
    StableName {
        kind: editor_core::EntityKind::Vertex,
        node: minter,
        path: vec![
            RoleSeg::EdgeCrossing(run),
            RoleSeg::Fragment(editor_core::Qualifier::OrderAlong { rank: 0, of: 2 }),
        ],
    }
}

/// The run's lines, in the order the name holds them.
fn run_lines(name: &StableName) -> Vec<StableName> {
    match name.path.first() {
        Some(RoleSeg::EdgeCrossing(run)) => run.iter().map(|l| (*l.line).clone()).collect(),
        other => panic!("an edge-crossing run, got {other:?}"),
    }
}

/// **A split and an inline keep a profile's step ids in name order,
/// however a `SetProgram` left them in the program.** The reshaping
/// re-mints step 2 after every other step, so the source program holds
/// its ids out of order. A ranked run over two lateral edges of one
/// extrude, told apart by those steps, is ranked along its least line;
/// the part and the host spell it with the same line least and the same
/// rank, so it is the name their emitters mint (ASSEMBLY.md A4,
/// *Acceptance*: name-resolution identity, up to minted ids).
#[test]
fn a_split_and_an_inline_keep_a_reordered_profiles_step_order() {
    let (doc, [f2, p2, e2], _) = reshaped_component("asm4-order", 2, false);
    let old = flat_ids(&doc, p2);
    assert!(
        old[2] > old[3],
        "the fixture holds its ids out of program order: {old:?}"
    );
    let was = ranked_run(e2, e2, old[2], old[3]);
    assert_eq!(
        run_lines(&was)[0],
        lateral_edge(e2, old[3]),
        "the step re-minted last lies on the greater line"
    );

    let out = split(
        &doc,
        &BTreeSet::from([f2, p2, e2]),
        DocumentId::derive("asm4-order-new"),
        Tol::witness(),
        None,
    )
    .expect("legal");
    assert!(
        keeps_id_order(&out.step_map),
        "the split's step map keeps id order: {:?}",
        out.step_map
    );
    let part_profile = out.node_map[&p2];
    let minted = flat_ids(&out.part, part_profile);
    let expected: Vec<_> = old.iter().map(|s| out.step_map[s]).collect();
    assert_eq!(
        minted, expected,
        "the part's program holds each step's image"
    );
    let reds = std::collections::BTreeMap::new();
    let crossed = editor_core::remap_name(&was, &out.node_map, &out.step_map, &reds)
        .expect("the split's maps cover the run");
    let e2_part = out.node_map[&e2];
    assert_eq!(
        crossed,
        ranked_run(
            e2_part,
            e2_part,
            out.step_map[&old[2]],
            out.step_map[&old[3]]
        ),
        "the part spells the run with its least line and its rank as the source did"
    );
    assert_eq!(
        run_lines(&crossed)[0],
        editor_core::remap_name(&run_lines(&was)[0], &out.node_map, &out.step_map, &reds)
            .expect("covered"),
        "the least line is the image of the least line"
    );

    let mut store = PartStore::default();
    let doc_ref = store.insert(doc, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("asm4-order-host"), Tol::witness());
    let (host, instance) = pinned(host, doc_ref, &with_resolver(store.clone()));
    let inlined = inline(
        &host,
        instance,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    )
    .expect("legal");
    assert!(
        keeps_id_order(&inlined.step_map),
        "the inline's step map keeps id order: {:?}",
        inlined.step_map
    );
    let e2_host = inlined.node_map[&e2];
    assert_eq!(
        editor_core::remap_name(&was, &inlined.node_map, &inlined.step_map, &reds)
            .expect("the inline's maps cover the run"),
        ranked_run(
            e2_host,
            e2_host,
            inlined.step_map[&old[2]],
            inlined.step_map[&old[3]]
        ),
        "the host spells the run with its least line and its rank as the part did"
    );
}

/// **Across two profiles the order is not kept, and a name it would
/// break refuses the inline.** The part's first profile has a step
/// re-minted after the second profile was inserted, so the two
/// profiles' ids interleave. The host mints the first profile's ids
/// before the second's, so each profile keeps its own order and the
/// re-minted step falls below the second profile's steps
/// (`refactor::in_id_order` says why no dealing can do better). A
/// name embedding a ranked run whose lines are told apart by those two
/// steps would change the run's least line, so the inline refuses it,
/// typed, rather than carry a reference that no longer resolves.
#[test]
fn an_inline_keeps_each_profiles_order_and_refuses_a_run_across_two() {
    let (doc, [_, p2, _], _) = reshaped_component("asm4-two", 2, false);
    let order = doc.ids();
    let (p1, e1) = (order[PROFILE_POSITION], order[BODY_POSITION]);
    let program = match doc.node(p1) {
        Some(Node::Profile(p)) => p.clone(),
        other => panic!("a profile, got {other:?}"),
    };
    let mut ids = program.kept_in_place();
    ids[0][1] = None;
    let (doc, _) = step(
        doc,
        DocEdit::SetProgram {
            node: p1,
            loops: program
                .loops
                .iter()
                .map(editor_core::LoopProgram::authored)
                .collect(),
            ids,
            fresh: Vec::new(),
        },
    );
    // The first block's pin is read off its edges, which the reshape
    // re-minted: written afresh over them.
    let first_place = doc.placements()[0];
    let doc = crate::p2_split::repin(doc, first_place, [0.0; 3], &EvalOptions::default());
    let (first, second) = (flat_ids(&doc, p1), flat_ids(&doc, p2));
    let late = first[1];
    assert!(
        second.iter().all(|&s| s < late) && second.iter().any(|&s| s > first[0]),
        "the two profiles' ids interleave: {first:?}, {second:?}"
    );
    let inline_into = |part: ProfileDoc, label: &str| {
        let mut store = PartStore::default();
        let doc_ref = store.insert(part, Tol::witness());
        let host = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (host, instance) = pinned(host, doc_ref, &with_resolver(store.clone()));
        let resolver = Arc::new(store) as Arc<dyn editor_core::PartResolver>;
        inline(&host, instance, &resolver, Tol::witness())
    };

    let inlined = inline_into(doc.clone(), "asm4-two-host").expect("legal");
    let within = |ids: &[editor_core::StepId]| -> editor_core::StepMap {
        ids.iter().map(|s| (*s, inlined.step_map[s])).collect()
    };
    assert!(
        keeps_id_order(&within(&first)) && keeps_id_order(&within(&second)),
        "each profile keeps its own order: {:?}",
        inlined.step_map
    );
    assert!(
        inlined.step_map[&late] < inlined.step_map[&second[0]],
        "across the two profiles the re-minted step falls below the second's: {:?}",
        inlined.step_map
    );

    // A face name embedding a ranked run told apart by those two steps
    // (an appearance is keyed by a face).
    let crossing = StableName {
        kind: editor_core::EntityKind::Face,
        node: e1,
        path: vec![RoleSeg::CornerFace(editor_core::NameRef::new(ranked_run(
            e1, e1, late, second[0],
        )))],
    };
    let body = doc.read_of_node(e1).expect("the extrude's read");
    let (doc, _) = step(
        doc,
        DocEdit::SetAppearance {
            body,
            name: crossing.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8 {
                r: 1,
                g: 2,
                b: 3,
                a: 255,
            }),
        },
    );
    match inline_into(doc, "asm4-two-host") {
        Err(InlineError::RankedRunReordered { name }) => {
            assert_eq!(name.name(), &crossing, "the refusal names the run");
        }
        other => panic!("expected RankedRunReordered, got {other:?}"),
    }
}

/// **Inline refuses where the part's world would move under a copy
/// pinned against a construction**: at a pose of the instance's own the
/// part's copies move with it and its constructions do not, so a copy
/// seated on a raw extrude of the part would land where the extrude
/// stays, not where the instance put it. Refused typed, naming the
/// placement and the construction, rather than spliced displaced.
///
/// *Breaks if* the mate's target is moved by the instance's motion as
/// a world mate's is, or the copy splices displaced.
#[test]
fn inline_at_a_pose_refuses_a_copy_pinned_against_a_construction() {
    use editor_core::{CapEnd, EntityKind};
    let part = ProfileDoc::empty(
        DocumentId::derive("asm4-pinned-to-construction"),
        Tol::witness(),
    );
    let extrude = |doc: ProfileDoc, half: f64| {
        let (doc, profile) = on_frame(
            doc,
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(0.0, 0.0, half)],
        );
        insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(1.0),
            },
        )
    };
    let (part, anchor) = extrude(part, 0.5);
    let (part, body) = extrude(part, 0.25);
    let (part, place) = insert(
        part,
        Node::place(vec![editor_core::BodyRead::from(
            editor_core::Operand::Node(body),
        )]),
    );
    let cap = |node, end| StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Cap(end)],
    };
    // The copy's bottom seated on the anchor's top, two along x.
    let (part, _) = crate::p2_split::seat_at(
        part,
        place,
        fixture::head_at(body, cap(body, CapEnd::Start)),
        fixture::head_at(anchor, cap(anchor, CapEnd::End)),
        [2.0, 0.0],
    );
    let mut store = PartStore::default();
    let doc_ref = store.insert(part, Tol::witness());
    let o = with_resolver(store.clone());
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(store);
    let host = ProfileDoc::empty(
        DocumentId::derive("asm4-pinned-to-construction-host"),
        Tol::witness(),
    );
    let (host, instance) = pinned_at(host, doc_ref, [0.0, 0.0, 5.0], &o);
    match inline(&host, instance, &resolver, Tol::witness()) {
        Err(InlineError::CopyPinnedToConstruction {
            placement,
            construction,
        }) => assert_eq!((placement.id(), construction.id()), (place, anchor)),
        Err(other) => panic!("refused naming the placement and its construction, not {other}"),
        Ok(_) => panic!("spliced, with the copy displaced from where the instance put it"),
    }
}
