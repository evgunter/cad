//! **`Node::Transform` holds a `Placement`** — a chain of rigid steps
//! and literal frames (`crates/editor-core/ASSEMBLY.md` A11
//! (2)). Each row names the claim it keeps; the corpus's bit identity
//! is `edit_placement_corpus_bits`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::wire::doctored;

use editor_core::{
    Dimension, DocEdit, DocParam, DocParamValue, EditError, Expr, Frame, Node, ParamName,
    PersistError, Placement, ProfileDoc, REGENERATE_RECOURSE, RecipeNodeId, RigidArg, SlotId,
    SnapshotError, Step, ValuePayload, load, save,
};
use fixture::{ang, insert, len, on_frame, scl, step};
use geom_core::predicate::Band;
use geom_core::{Affine3, Interval, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance forms a band")
}

/// A unit cube on the xy frame, and its extrude's id.
fn cube(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    )
}

/// The frame whose +Z aims from `(1, 2, 3)` at `(4, -1, 5)`, rolled by
/// world +X — no symmetry, so a transposed read cannot pass.
fn aimed() -> Frame {
    Frame::from_affine(
        geom_core::linalg::frame::point_at::<f64>(
            Point3::new(1.0, 2.0, 3.0),
            Point3::new(4.0, -1.0, 5.0),
            Vec3::new(1.0, 0.0, 0.0),
            Tol::witness(),
        )
        .expect("the aim is definite"),
    )
}

/// Every stored point of the body a node evaluated to, as bits.
fn point_bits<T: geom_core::Decide>(
    ev: &editor_core::Evaluation<T>,
    id: RecipeNodeId,
    bits: impl Fn(&Point3<T>) -> [u64; 3],
) -> Vec<[u64; 3]> {
    let Some(ValuePayload::Body(body)) = ev.value(id).map(|v| &v.payload) else {
        panic!("node {} evaluated to no body", id.0)
    };
    let mut out: Vec<[u64; 3]> = body.points().map(|(_, p)| bits(p)).collect();
    out.sort_unstable();
    out
}

fn f64_bits(p: &Point3<f64>) -> [u64; 3] {
    [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]
}

fn affine_bits(a: &Affine3<f64>) -> Vec<u64> {
    [a.linear.c0, a.linear.c1, a.linear.c2, a.translation]
        .iter()
        .flat_map(|c| [c.x.to_bits(), c.y.to_bits(), c.z.to_bits()])
        .collect()
}

/// **A transform over a `point_at` frame evaluates to that frame
/// exactly.** The motion is the frame's bits, the placed body is the
/// input's points under that frame's map, and the literal evaluates in
/// the interval lane as well as at `f64`.
#[test]
fn a_transform_over_a_point_at_frame_evaluates_to_that_frame_exactly() {
    let frame = aimed();
    let placement = Placement::literal(&frame);
    let motion = placement
        .eval::<f64>(&editor_core::ParamEnv::default(), band())
        .expect("a literal evaluates");
    assert_eq!(
        affine_bits(&motion),
        affine_bits(&frame.affine::<f64>()),
        "the motion is the frame's own bits"
    );
    assert!(
        Frame::from_affine(motion).bit_eq(&frame),
        "and reads back as the frame"
    );

    let (doc, body) = cube("placement-point-at");
    let (doc, placed) = insert(
        doc,
        Node::Transform {
            input: body,
            placement,
        },
    );
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    let map = frame.affine::<f64>();
    let mut want: Vec<[u64; 3]> = point_bits(&ev, body, f64_bits)
        .into_iter()
        .map(|[x, y, z]| {
            let p = Point3::new(f64::from_bits(x), f64::from_bits(y), f64::from_bits(z));
            f64_bits(&map.transform_point(p))
        })
        .collect();
    want.sort_unstable();
    assert_eq!(
        point_bits(&ev, placed, f64_bits),
        want,
        "the placed body is the input under the frame's map"
    );

    let lane = crate::corpus::eval::<Interval>(&doc);
    let bad = crate::corpus::failures(&lane);
    assert!(
        bad.is_empty(),
        "the literal evaluates in the interval lane: {bad:?}"
    );
}

/// **A two-step chain composes in order**: `[a, b]` is `a ∘ b`, so `b`
/// acts first. The fixture is a translation and a quarter turn, whose
/// two orders put `(1, 0, 0)` at different points.
#[test]
fn a_two_step_chain_composes_in_order() {
    let env = editor_core::ParamEnv::default();
    let shift = Placement::rigid(
        [len(1.0), len(0.0), len(0.0)],
        [scl(0.0), scl(0.0), scl(1.0)],
        ang(0.0),
    );
    let turn = Placement::rigid(
        [len(0.0), len(0.0), len(0.0)],
        [scl(0.0), scl(0.0), scl(1.0)],
        ang(core::f64::consts::FRAC_PI_2),
    );
    let one = |p: &Placement| p.eval::<f64>(&env, band()).expect("a rigid step evaluates");
    let chain = |a: &Placement, b: &Placement| {
        let mut steps = a.steps.clone();
        steps.extend(b.steps.iter().cloned());
        Placement { steps }
    };
    let shift_then_turn = one(&chain(&shift, &turn));
    assert_eq!(
        affine_bits(&shift_then_turn),
        affine_bits(&(one(&shift) * one(&turn))),
        "the chain is the left-to-right product, to the bit"
    );
    let p = shift_then_turn.transform_point(Point3::new(1.0, 0.0, 0.0));
    assert!(
        (p.x - 1.0).abs() < 1e-12 && (p.y - 1.0).abs() < 1e-12,
        "the turn acts first, then the shift: (1, 0, 0) lands at (1, 1, 0), not {p:?}"
    );
    let q = one(&chain(&turn, &shift)).transform_point(Point3::new(1.0, 0.0, 0.0));
    assert!(
        q.x.abs() < 1e-12 && (q.y - 2.0).abs() < 1e-12,
        "the other order lands at (0, 2, 0), not {q:?}"
    );
}

/// **A matrix step that is improper or non-finite is refused, typed,
/// at the edit door and at load.** A file cannot spell a non-finite
/// coordinate (JSON has no token for one, and the reader refuses it
/// before this build's types are asked), so the load door's half is
/// the mirror.
#[test]
fn an_improper_or_non_finite_matrix_step_is_refused_at_both_doors() {
    let mirror = Frame {
        columns: [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0, 0.0, 0.0],
    };
    let mut nan = Frame::IDENTITY;
    nan.translation[1] = f64::NAN;
    let (doc, body) = cube("placement-refused");
    let at = |frame: &Frame| DocEdit::InsertNode {
        node: Node::Transform {
            input: body,
            placement: Placement::rigid(
                [len(0.0), len(0.0), len(0.0)],
                [scl(0.0), scl(0.0), scl(1.0)],
                ang(0.0),
            )
            .then(Step::Matrix(*frame)),
        },
    };
    let door = |edit| {
        editor_core::apply(&doc, &edit, Tol::witness(), &editor_core::RefusingReach).map(|_| ())
    };
    match door(at(&mirror)) {
        Err(EditError::ImproperPlacement { determinant, .. }) => {
            assert!(determinant < 0.0, "the refusal carries the determinant");
        }
        other => panic!("a mirrored step must refuse typed, got {other:?}"),
    }
    match door(at(&nan)) {
        Err(EditError::NonFinitePlacement { .. }) => {}
        other => panic!("a non-finite step must refuse typed, got {other:?}"),
    }

    let (doc, placed) = step(doc, at(&Frame::translation([0.0, 0.0, 0.125])));
    let placed = placed.expect("a proper step inserts");
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    let corrupt = doctored(&text, |wire| {
        let entry = &mut wire["snapshot"]["nodes"][placed.0.to_string()]["Transform"]["placement"]
            ["steps"][1]["Matrix"]["columns"][0][0];
        assert_eq!(
            *entry,
            serde_json::json!(1.0),
            "aimed at the literal's first axis"
        );
        *entry = serde_json::json!(-1.0);
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::PlacementImproper { node, determinant })) => {
            assert_eq!(node, placed);
            assert!(determinant < 0.0, "the refusal carries the determinant");
        }
        other => panic!("a mirrored step must refuse typed at load, got {other:?}"),
    }
}

/// **A parameter drives a rigid step's angle** — a step after a
/// literal, so its slots are the later-step addresses: the parameter
/// route and a literal written at `SlotId::rigid(1, …)` place the body
/// at the same bits, and step 0 (the literal) has no slot to write.
#[test]
fn a_parameter_drives_a_rigid_steps_angle() {
    let turn = ParamName::from_static("turn");
    let (doc, body) = cube("placement-param");
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: turn.clone(),
            value: DocParam::continuous(Dimension::Angle, 0.0),
        },
    );
    let placement = Placement::literal(&Frame::translation([5.0, 0.0, 0.0])).then(Step::Rigid {
        translation: [len(0.0), len(0.0), len(0.0)],
        axis: [scl(0.0), scl(0.0), scl(1.0)],
        angle: Expr::param(turn.clone(), Dimension::Angle),
    });
    let (doc, placed) = insert(
        doc,
        Node::Transform {
            input: body,
            placement,
        },
    );
    let min_x = |doc: &ProfileDoc| {
        let ev = fixture::run(doc, &editor_core::EvalOptions::default());
        let points = point_bits(&ev, placed, f64_bits);
        (
            points
                .iter()
                .map(|p| f64::from_bits(p[0]))
                .fold(f64::INFINITY, f64::min),
            points,
        )
    };
    let (at_rest, _) = min_x(&doc);
    assert!(
        (at_rest - 5.0).abs() < 1e-12,
        "no turn: the cube starts at x = 5"
    );

    let quarter = core::f64::consts::FRAC_PI_2;
    let (turned_doc, _) = step(
        doc.clone(),
        DocEdit::SetDocParamValue {
            name: turn,
            value: DocParamValue::Continuous(quarter),
        },
    );
    let (turned, by_param) = min_x(&turned_doc);
    assert!(
        (turned - 4.0).abs() < 1e-12,
        "a quarter turn about +z swings the cube to x = 4, not {turned}"
    );

    let angle = SlotId::rigid(1, RigidArg::RotationAngle);
    assert!(
        matches!(angle, SlotId::PlacementStep { .. }),
        "step 1 is a later-step address"
    );
    let (by_slot_doc, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: placed,
            slot: angle,
            expr: ang(quarter),
        },
    );
    assert_eq!(
        min_x(&by_slot_doc).1,
        by_param,
        "the parameter and a literal at the step's slot place the body at the same bits"
    );
    match editor_core::apply(
        &doc,
        &DocEdit::SetParam {
            node: placed,
            slot: SlotId::RotationAngle,
            expr: ang(quarter),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::UnknownSlot { slot, .. }) => assert_eq!(slot, SlotId::RotationAngle),
        other => panic!("step 0 is a literal and has no angle slot, got {other:?}"),
    }
}

/// **An old file is refused, typed**: a transform written as its three
/// bare components is vocabulary this build does not read, refused as
/// `Unreadable` with the regenerate recourse — never loaded with a
/// different meaning.
#[test]
fn an_old_file_is_refused_typed() {
    let (doc, body) = cube("placement-old-file");
    let (doc, placed) = insert(
        doc,
        Node::transform(
            body,
            [len(0.0), len(0.0), len(1.0)],
            [scl(0.0), scl(0.0), scl(1.0)],
            ang(0.25),
        ),
    );
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    let old = doctored(&text, |wire| {
        let node = &mut wire["snapshot"]["nodes"][placed.0.to_string()]["Transform"];
        let rigid = node["placement"]["steps"][0]["Rigid"].take();
        assert!(rigid.is_object(), "aimed at the one rigid step");
        let object = node
            .as_object_mut()
            .expect("a transform is an object on the wire");
        object.remove("placement");
        object.insert("translation".to_owned(), rigid["translation"].clone());
        object.insert("rotation_axis".to_owned(), rigid["axis"].clone());
        object.insert("rotation_angle".to_owned(), rigid["angle"].clone());
    });
    match load(&old, Tol::witness()) {
        Err(error @ PersistError::Unreadable { .. }) => {
            let PersistError::Unreadable { detail, .. } = &error else {
                unreachable!("matched above")
            };
            assert!(
                detail.contains("translation") || detail.contains("placement"),
                "the refusal names the vocabulary it could not place: {detail}"
            );
            assert!(
                error.to_string().ends_with(REGENERATE_RECOURSE),
                "with the regenerate recourse: {error}"
            );
        }
        other => panic!("an old transform must refuse typed, got {other:?}"),
    }
}

/// **A literal frame compares by bits** (D7): a transform's matrix
/// step and an explicit rule's listed frame differing only in a signed
/// zero are two nodes to `Node::bit_eq`, as they are to the content
/// key, and a saved literal reads back to its own bits.
#[test]
fn a_literal_frame_compares_by_bits() {
    let signed = Frame::translation([-0.0, 0.0, 0.25]);
    let plain = Frame::translation([0.0, 0.0, 0.25]);
    let body = RecipeNodeId(3);
    let transform = |f: &Frame| -> Node<editor_core::ProfileProgram> {
        Node::Transform {
            input: body,
            placement: Placement::literal(f),
        }
    };
    assert!(transform(&signed).bit_eq(&transform(&signed)));
    assert!(
        !transform(&signed).bit_eq(&transform(&plain)),
        "a transform's -0.0 is not its 0.0"
    );
    let group =
        |f: &Frame| -> Node<editor_core::ProfileProgram> { Node::placed_union_at(body, vec![*f]) };
    assert!(
        !group(&signed).bit_eq(&group(&plain)),
        "an explicit rule's -0.0 is not its 0.0"
    );

    let (doc, input) = cube("placement-bits");
    let (doc, _) = insert(
        doc,
        Node::Transform {
            input,
            placement: Placement::literal(&signed),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    let back = load(&text, Tol::witness()).expect("its own bytes load").doc;
    assert!(back.bit_eq(&doc), "the literal round-trips to its own bits");
}
