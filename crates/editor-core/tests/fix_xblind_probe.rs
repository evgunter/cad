//! FIX measurement probe — is the mate solve blind to a `Transform`
//! sitting between an `InstantiatePart` and the material a mate names?
//!
//! `wire_transform` is an identity-preserving pass-through (spec D2):
//! the transform contributes no `RolePath` segment and the input's name
//! table holds verbatim, so a mate reference through a transform still
//! names the MINTING instance and `member_of` admits it on its first
//! arm. The solve then places that instance — and the transform
//! composes its own map on top of the solved pose, downstream.
//!
//! These rows measure whether the two agree.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AxisSense, CancelToken, CapEnd, ContactClass, DocEdit, DocRef, DocumentId,
    EntityKind, EvalOptions, Evaluation, Expr, MateFrame, MatePrimitive, Node, PartResolver,
    ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, StableName, content_pin,
    evaluate, product,
};
use fixture::{insert, len, on_frame, scl, step};
use geom_core::Tol;

// ---- substrate ----

#[derive(Debug, Default)]
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

impl PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
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

fn run(doc: &ProfileDoc, o: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), o, Tol::witness())
}

/// The extrude in a one-block part document (frame, profile, extrude).
const PART_BODY: RecipeNodeId = RecipeNodeId(2);

/// A `1x1xh` block, as a whole part document.
fn block(label: &str, h: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
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

/// A face of `instance`'s part product — the plain member spelling.
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

fn mate_frame(origin: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

/// A determining `Rest` mate seating `b`'s bottom onto `a`'s top.
fn seat(a: StableName, b: StableName) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: mate_frame([0.0, 0.0, 1.0]),
            b: mate_frame([0.0, 0.0, 0.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// The z-extent of a body, over its vertices. Exact: a rigid
/// translation along z moves it by exactly the translation.
fn z_span(body: &topo::Body<f64>) -> (f64, f64) {
    body.vertices()
        .filter_map(|(_, v)| body.get_point(v.point))
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| {
            (lo.min(p.z), hi.max(p.z))
        })
}

fn body_at(ev: &Evaluation<f64>, node: RecipeNodeId) -> Arc<topo::Body<f64>> {
    match ev.value(node).map(|v| &v.payload) {
        Some(editor_core::ValuePayload::Body(b)) => Arc::clone(b),
        other => panic!("expected a body at {node:?}, got {other:?}"),
    }
}

/// The lift height the transform applies.
const LIFT: f64 = 10.0;

/// `base` and `top`, mated bottom-onto-top; when `lift` is set, `top`
/// is wrapped in a `Transform` translating +z by [`LIFT`].
/// Returns `(doc, opts, base, top, wrapped_or_top)`.
fn seated(
    label: &str,
    lift: bool,
) -> (
    ProfileDoc,
    EvalOptions,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
) {
    let mut store = StubStore::default();
    // DIFFERENT HEIGHTS on purpose: with equal unit cubes the
    // `Opposed` flip maps the top block back onto the base's own
    // z-range, and an identity solve is indistinguishable from a
    // correct one. 1 and 3 separate them.
    let base_ref = store.insert(block(&format!("{label}-base"), 1.0), Tol::witness());
    let top_ref = store.insert(block(&format!("{label}-top"), 3.0), Tol::witness());
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };

    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, tail) = if lift {
        insert(
            doc,
            Node::Transform {
                input: top,
                translation: [len(0.0), len(0.0), len(LIFT)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: Expr::literal(0.0, editor_core::Dimension::Angle).unwrap(),
            },
        )
    } else {
        (doc, top)
    };
    let (doc, _mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(in_part(base, CapEnd::Top), in_part(top, CapEnd::Bottom)),
        },
    );
    (doc, opts, base, top, tail)
}

/// STEP 1 — reachability: does a `Transform` between an
/// `InstantiatePart` and the material a mate names survive the edit
/// doors and evaluate?
#[test]
fn step1_transform_over_a_mated_instance_is_reachable() {
    let (doc, opts, base, top, xf) = seated("xblind-reach", true);
    assert_ne!(xf, top, "the transform node was inserted");
    let ev = run(&doc, &opts);
    for (what, id) in [("base", base), ("top", top), ("transform", xf)] {
        assert!(
            matches!(ev.result(id), Some(editor_core::NodeResult::Ok(_))),
            "{what} ({id:?}) evaluated: {:?}",
            ev.result(id)
        );
    }
    let body = product(&doc, &ev, Tol::witness()).expect("the product gathers");
    eprintln!(
        "REACH: solids={} base_z={:?} top_z={:?} xf_z={:?} product_z={:?}",
        body.solids().count(),
        z_span(&body_at(&ev, base)),
        z_span(&body_at(&ev, top)),
        z_span(&body_at(&ev, xf)),
        z_span(&body),
    );
}

/// STEP 2 — the comparison: is the solved pose blind to the
/// transform's map?
#[test]
fn step2_solved_pose_versus_evaluated_geometry() {
    let (cdoc, copts, cbase, ctop, _) = seated("xblind-control", false);
    let cev = run(&cdoc, &copts);
    let control_base = z_span(&body_at(&cev, cbase));
    let control_top = z_span(&body_at(&cev, ctop));

    let (doc, opts, base, top, xf) = seated("xblind-test", true);
    let ev = run(&doc, &opts);
    let test_base = z_span(&body_at(&ev, base));
    let test_top = z_span(&body_at(&ev, top));
    let test_xf = z_span(&body_at(&ev, xf));

    let cposes = editor_core::solve_document(&cdoc, Tol::witness());
    eprintln!("CONTROL base z {control_base:?}  top z {control_top:?}");
    eprintln!("CONTROL solved relative(top) = {:?}", cposes.relative(ctop));
    eprintln!("TEST    base z {test_base:?}  instance z {test_top:?}  transform out z {test_xf:?}");
    let poses = editor_core::solve_document(&doc, Tol::witness());
    eprintln!("TEST    solved relative(top) = {:?}", poses.relative(top));
    assert_eq!(
        format!("{:?}", poses.relative(top)),
        format!("{:?}", cposes.relative(ctop)),
        "the solve produces the SAME relative pose with and without the transform"
    );
    eprintln!(
        "solve moved the instance by {} between control and test; \
         transform output sits {} from the control's mated pose",
        test_top.0 - control_top.0,
        test_xf.0 - control_top.0
    );
}

/// The z of a named PLANAR face's plane, read out of a value's own
/// name table — the mate's own spelling, resolved the way any
/// reference to it resolves.
fn named_face_z(ev: &Evaluation<f64>, at: RecipeNodeId, name: &StableName) -> f64 {
    let value = ev.value(at).expect("a value");
    let entry = value
        .name_table
        .lookup(name)
        .unwrap_or_else(|| panic!("{name:?} is absent from {at:?}'s table"));
    let editor_core::Entry::Unique(ent) = entry else {
        panic!("expected a unique entry, got {entry:?}")
    };
    let editor_core::EntityKey::Face(f) = ent.key else {
        panic!("expected a face, got {:?}", ent.key)
    };
    let body = body_at(ev, at);
    let face = body.get_face(f).expect("the face");
    match body.get_surface(face.surface).expect("the surface") {
        topo::Surface::Plane { origin, .. } => origin.z,
        other => panic!("expected a plane, got {other:?}"),
    }
}

/// STEP 3 — what a USER sees: the mate's own named faces, resolved in
/// the values the product actually gathers.
#[test]
fn step3_the_mated_faces_in_the_product() {
    let (doc, opts, base, top, xf) = seated("xblind-faces", true);
    let ev = run(&doc, &opts);
    let a = in_part(base, CapEnd::Top);
    let b = in_part(top, CapEnd::Bottom);

    let a_z = named_face_z(&ev, base, &a);
    let b_at_instance = named_face_z(&ev, top, &b);
    let b_at_transform = named_face_z(&ev, xf, &b);

    eprintln!(
        "mate face A (base top) z = {a_z}; mate face B (top bottom) z = \
         {b_at_instance} at the INSTANCE, {b_at_transform} at the TRANSFORM \
         (the node the product gathers)"
    );
    eprintln!(
        "contact at the instance: {} | contact in the product: {}",
        (a_z - b_at_instance).abs() < 1e-12,
        (a_z - b_at_transform).abs() < 1e-12
    );

    // No refusal anywhere: the document is fully green.
    assert!(
        editor_core::solve_document(&doc, Tol::witness())
            .fault(top)
            .is_none(),
        "the solve records no fault"
    );
    assert!(
        product(&doc, &ev, Tol::witness()).is_ok(),
        "the product gathers"
    );
}

/// STEP 4 — the OTHER half of the same wrong answer: a pattern OVER a
/// transform. The guard the ruling would remove.
#[test]
fn step4_pattern_over_transform_refuses_today() {
    let mut store = StubStore::default();
    let base_ref = store.insert(block("xblind-p-base", 1.0), Tol::witness());
    let top_ref = store.insert(block("xblind-p-top", 3.0), Tol::witness());
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };

    let doc = ProfileDoc::empty(DocumentId::derive("xblind-pat"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, xf) = insert(
        doc,
        Node::Transform {
            input: top,
            translation: [len(0.0), len(0.0), len(LIFT)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: Expr::literal(0.0, editor_core::Dimension::Angle).unwrap(),
        },
    );
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: xf,
            count: Expr::count(3),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let copy_face = StableName {
        kind: EntityKind::Face,
        node: pattern,
        path: vec![RoleSeg::Instance {
            i: 1,
            of: Box::new(in_part(top, CapEnd::Bottom)),
        }],
    };
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(in_part(base, CapEnd::Top), copy_face),
        },
    );
    let mate = mate.unwrap();

    let poses = editor_core::solve_document(&doc, Tol::witness());
    eprintln!(
        "PATTERN-OF-TRANSFORM: fault at the mate = {:?}",
        poses.fault(mate)
    );
    let ev = run(&doc, &opts);
    eprintln!("  base result   = {:?}", ev.result(base).map(discriminant));
    eprintln!(
        "  pattern result= {:?}",
        ev.result(pattern).map(discriminant)
    );
    eprintln!(
        "  product = {:?}",
        product(&doc, &ev, Tol::witness()).map(|b| b.solids().count())
    );
}

fn discriminant(r: &editor_core::NodeResult<f64>) -> String {
    match r {
        editor_core::NodeResult::Ok(_) => "Ok".to_string(),
        editor_core::NodeResult::Failed(e) => format!("Failed({:?})", e.kind),
        other => format!("{other:?}"),
    }
}
