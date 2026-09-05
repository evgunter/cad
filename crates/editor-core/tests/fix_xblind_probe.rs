//! **These rows pin a DEFECT, not a contract.** Every expectation
//! below records what the kernel does TODAY and is believed WRONG; the
//! finding is written up in
//! `work/issues/mate-solve-is-transform-blind.md`, which is what
//! explains them.
//!
//! The defect: `wire_transform` is an identity-preserving pass-through
//! (spec D2) — a `Node::Transform` contributes no `RolePath` segment
//! and its input's name table holds verbatim — so a mate reference
//! through a transform still names the MINTING instance and
//! `member_of` admits it on its first arm. The solve then places that
//! instance in the instance's OWN coordinates (`fold_pair` reads
//! authored alignment and the pattern-derived offsets, never the
//! evaluated body), and the transform composes its map on top,
//! downstream. The two disagree by exactly the transform's map, and
//! nothing refuses.
//!
//! **The correct answer, when this is fixed:** `contact in the
//! product` must become `true` — the mate's named faces must meet in
//! the value the product actually gathers — and the two solved frames
//! in `step2` must STOP being identical, because the solve must
//! compose the transform's map into the pose it hands the instance.
//!
//! **The fix DELETES these rows; it does not update their
//! expectations.** A lane that finds them red because it composed the
//! map has not broken a test — it has removed the reason these rows
//! exist, and the rows go with the defect. Re-pointing an assertion
//! here at a new number would turn a defect pin back into a baseline,
//! which is the one thing it must never become. Owner: the S-MATE unit
//! that adds the `derived_offset` sibling walking the input chain.
//!
//! `step4` is the same story from the other side: the pattern-headed
//! form of this shape refuses `DanglingHead` today, and that refusal
//! is the ONLY thing keeping the patterned half honest. It is expected
//! to go red when the member vocabulary is extended through
//! identity-transparent nodes — which is the ruled change, and must
//! land only WITH the map composition, or it converts this refusal
//! into a second silent wrong answer.
//!
//! The `step2` mechanism assertion is the stronger of the two shapes:
//! it isolates the cause (the solve does not read the transform) and
//! so goes red on any fix that composes the map, whatever the geometry
//! then measures.

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
            node: seat(in_part(base, CapEnd::End), in_part(top, CapEnd::Start)),
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
    // Not vacuous: both sides must actually HAVE a solved pose, or the
    // Debug comparison below would pass on two `None`s.
    assert!(
        poses.relative(top).is_some() && cposes.relative(ctop).is_some(),
        "both documents solved a relative pose for the mated instance"
    );
    // THE MECHANISM. This is the assertion that isolates the cause:
    // the solve hands the instance the identical pose whether or not a
    // transform sits downstream, i.e. it never reads the map. It goes
    // red on ANY fix that composes the map, whatever the geometry then
    // measures — at which point DELETE this row (see the module
    // header); do not re-point it at the new frame.
    assert_eq!(
        format!("{:?}", poses.relative(top)),
        format!("{:?}", cposes.relative(ctop)),
        "the solve is no longer transform-blind — it produced a \
         different relative pose with the transform present. That is \
         the intended repair: delete this suite rather than updating \
         this expectation (work/issues/mate-solve-is-transform-blind.md)"
    );
    // The consequence, in exact arithmetic: the transform's output
    // sits exactly the transform's own translation away from the pose
    // the mate asked for.
    assert_eq!(
        (test_xf.0 - control_top.0).to_bits(),
        LIFT.to_bits(),
        "the transform's output is displaced from the mated pose by \
         exactly the transform's translation"
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
    let a = in_part(base, CapEnd::End);
    let b = in_part(top, CapEnd::Start);

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

    // The mate IS satisfied where the solve worked...
    assert_eq!(
        a_z.to_bits(),
        b_at_instance.to_bits(),
        "the mate seats the faces together at the instance"
    );
    // ...and is NOT satisfied in the body the product gathers. THIS IS
    // THE DEFECT. When it is fixed these two faces meet and this
    // assertion goes red: delete this row, do not re-point it.
    assert_ne!(
        a_z.to_bits(),
        b_at_transform.to_bits(),
        "the mated faces now MEET in the product — the transform-blind \
         solve was fixed. Delete this suite rather than updating this \
         expectation (work/issues/mate-solve-is-transform-blind.md)"
    );
    assert_eq!(
        (b_at_transform - b_at_instance).to_bits(),
        LIFT.to_bits(),
        "the named face is displaced by exactly the transform's translation"
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
            of: Box::new(in_part(top, CapEnd::Start)),
        }],
    };
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(in_part(base, CapEnd::End), copy_face),
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
