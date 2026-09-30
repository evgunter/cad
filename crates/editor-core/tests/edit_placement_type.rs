//! **`Node::Transform` holds a `Placement`** — a chain of rigid steps
//! and literal frames (`crates/editor-core/ASSEMBLY.md` A11
//! (2)). Each row names the claim it keeps; the corpus's bit identity
//! is `edit_placement_corpus_bits`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::wire::doctored;

use editor_core::{
    Axis3, CancelToken, Dimension, DocEdit, DocParam, DocParamValue, EditError, EvalOptions, Expr,
    Frame, FrameSite, LoggedEdit, Node, ParamEnv, ParamName, PersistError, Placement, ProfileDoc,
    ProfileProgram, REGENERATE_RECOURSE, RecipeNodeId, RigidArg, SlotId, SnapshotError, Step,
    ValuePayload, VectorSlot, evaluate, load, save,
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

/// A rigid step about +z by `angle`, then by `t`.
fn about_z(t: [f64; 3], angle: f64) -> Step {
    Step::Rigid {
        translation: t.map(len),
        axis: [scl(0.0), scl(0.0), scl(1.0)],
        angle: ang(angle),
    }
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

fn motion(p: &Placement) -> Affine3<f64> {
    p.eval::<f64>(&ParamEnv::default(), band())
        .expect("the placement evaluates")
}

fn lands(p: &Placement, at: [f64; 3]) -> Point3<f64> {
    motion(p).transform_point(Point3::new(at[0], at[1], at[2]))
}

fn near(p: Point3<f64>, q: [f64; 3]) -> bool {
    (p.x - q[0]).abs() < 1e-12 && (p.y - q[1]).abs() < 1e-12 && (p.z - q[2]).abs() < 1e-12
}

/// **A transform over a `point_at` frame evaluates to that frame
/// exactly.** The motion is the frame's bits, the placed body is the
/// input's points under that frame's map, and the literal evaluates in
/// the interval lane as well as at `f64`.
#[test]
fn a_transform_over_a_point_at_frame_evaluates_to_that_frame_exactly() {
    let frame = aimed();
    let placement = Placement::literal(&frame);
    let got = motion(&placement);
    assert_eq!(
        affine_bits(&got),
        affine_bits(&frame.affine::<f64>()),
        "the motion is the frame's own bits"
    );
    assert!(
        Frame::from_affine(got).bit_eq(&frame),
        "and reads back as the frame"
    );

    let (doc, body) = cube("placement-point-at");
    let (doc, placed) = insert(doc, Node::transform(body, placement));
    let ev = fixture::run(&doc, &EvalOptions::default());
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

/// **`compose` is `Frame::compose`'s order: `a.compose(b)` is `a ∘ b`,
/// so `b` acts first**, in the frame `a` builds (A11 (5)'s "gauge frame
/// ∘ offset ∘ solved pose"). By value: a quarter turn composed inside a
/// shift sends `(1, 0, 0)` to `(2, 1, 0)`, and the reverse order sends
/// it to `(0, 3, 0)`; by bits, the chain's motion is the frames'
/// `Frame::compose`; and a literal shift composed with a rigid turn
/// lands where `Frame::rotate_then_translate` does, whose rotation acts
/// first.
#[test]
fn compose_is_frame_composes_order_and_its_inner_placement_acts_first() {
    let shift = Placement::from(about_z([1.0, 0.0, 0.0], 0.0));
    let turn = Placement::from(about_z([0.0; 3], core::f64::consts::FRAC_PI_2));
    let turn_then_shift = shift.compose(&turn);
    assert_eq!(turn_then_shift.steps.len(), 2);
    assert!(
        near(lands(&turn_then_shift, [1.0, 0.0, 0.0]), [1.0, 1.0, 0.0]),
        "the inner turn acts first: (1, 0, 0) → (0, 1, 0) → (1, 1, 0), not {:?}",
        lands(&turn_then_shift, [1.0, 0.0, 0.0])
    );
    let shift_then_turn = turn.compose(&shift);
    assert!(
        near(lands(&shift_then_turn, [1.0, 0.0, 0.0]), [0.0, 2.0, 0.0]),
        "the other order lands at (0, 2, 0), not {:?}",
        lands(&shift_then_turn, [1.0, 0.0, 0.0])
    );
    assert!(
        Frame::from_affine(motion(&turn_then_shift)).bit_eq(
            &Frame::from_affine(motion(&shift)).compose(&Frame::from_affine(motion(&turn)))
        ),
        "the chain's motion is Frame::compose of its steps', to the bit"
    );

    let by_frame = Frame::rotate_then_translate(
        [0.0, 0.0, 1.0],
        core::f64::consts::FRAC_PI_2,
        [1.0, 0.0, 0.0],
        band(),
    )
    .expect("a literal axis has a definite direction");
    let by_chain = Placement::literal(&Frame::translation([1.0, 0.0, 0.0])).compose(&turn);
    let want = by_frame
        .affine::<f64>()
        .transform_point(Point3::new(1.0, 0.0, 0.0));
    assert!(
        near(lands(&by_chain, [1.0, 0.0, 0.0]), [want.x, want.y, want.z]),
        "a shift composed over a turn is rotate_then_translate"
    );
}

/// **An identity step moves no bit**, wherever it sits: `[I, x]`,
/// `[x, I]` and `x` evaluate to the same bits — the one composition
/// rule `Frame::compose` answers by — for a rigid `x` whose motion
/// holds a `-0.0` (a zero angle, a `-0.0` translation) and for a
/// literal `x` holding them.
#[test]
fn an_identity_step_moves_no_bit() {
    let signed = Frame {
        columns: [[0.0, 1.0, -0.0], [-1.0, 0.0, 0.0], [0.0, -0.0, 1.0]],
        translation: [-0.0, 2.0, 0.1],
    };
    let identity = Placement::literal(&Frame::IDENTITY);
    for (what, x) in [
        ("rigid", Placement::from(about_z([-0.0, 2.0, 0.1], 0.0))),
        ("literal", Placement::literal(&signed)),
    ] {
        let alone = affine_bits(&motion(&x));
        assert_eq!(
            affine_bits(&motion(&identity.compose(&x))),
            alone,
            "[I, {what}] is {what}"
        );
        assert_eq!(
            affine_bits(&motion(&x.compose(&identity))),
            alone,
            "[{what}, I] is {what}"
        );
    }
    assert!(
        Frame::from_affine(motion(&Placement::literal(&signed))).bit_eq(&signed),
        "the fixture's -0.0s survive a lone literal"
    );
}

/// **The empty chain is the identity, and both doors admit it**: it
/// evaluates to the identity map's bits, it is `compose`'s unit, a
/// transform holding it places its input where it stands, and it saves
/// and loads to itself.
#[test]
fn the_empty_chain_is_the_identity_at_both_doors() {
    assert_eq!(
        affine_bits(&motion(&Placement::IDENTITY)),
        affine_bits(&Affine3::identity()),
    );
    let x = Placement::from(about_z([1.0, -0.0, 0.5], 0.25));
    assert!(Placement::IDENTITY.compose(&x).bit_eq(&x));
    assert!(x.compose(&Placement::IDENTITY).bit_eq(&x));

    let (doc, body) = cube("placement-empty");
    let (doc, placed) = step(
        doc,
        DocEdit::InsertNode {
            node: Node::transform(body, Placement::IDENTITY),
        },
    );
    let placed = placed.expect("the edit door admits the empty chain");
    let ev = fixture::run(&doc, &EvalOptions::default());
    assert_eq!(
        point_bits(&ev, placed, f64_bits),
        point_bits(&ev, body, f64_bits),
        "the body stays where it stands"
    );
    let text = save(&doc, &[], Tol::witness()).expect("the empty chain saves");
    let back = load(&text, Tol::witness()).expect("and loads").doc;
    assert!(back.bit_eq(&doc), "to its own bits");
}

/// **A literal step the admission rule refuses is refused, typed, at
/// the edit door and at load, naming its step.** A mirror, a
/// non-finite coordinate and a proper frame that scales an axis (which
/// the evaluation would refuse to move a body by) each refuse at the
/// edit door with the step's index and a recourse. A file cannot spell
/// a non-finite coordinate (JSON has no token for one, and the reader
/// refuses it before this build's types are asked), so the load door's
/// half is the mirror and the scale.
#[test]
fn a_bad_literal_step_is_refused_at_both_doors() {
    let mirror = Frame {
        columns: [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0, 0.0, 0.0],
    };
    let mut nan = Frame::IDENTITY;
    nan.translation[1] = f64::NAN;
    let mut stretched = Frame::IDENTITY;
    stretched.columns[1] = [0.0, 2.0, 0.0];
    let (doc, body) = cube("placement-refused");
    let at = |frame: &Frame| DocEdit::InsertNode {
        node: Node::transform(
            body,
            Placement::from(about_z([0.0; 3], 0.0)).compose(&Placement::literal(frame)),
        ),
    };
    let door = |edit| {
        editor_core::apply(&doc, &edit, Tol::witness(), &editor_core::RefusingReach).map(|_| ())
    };
    let second = FrameSite::Step { index: 1 };
    for (what, frame) in [("mirror", mirror), ("nan", nan), ("scale", stretched)] {
        let error = door(at(&frame)).expect_err(what);
        match (what, &error) {
            (
                "mirror",
                EditError::ImproperPlacement {
                    at, determinant, ..
                },
            ) => {
                assert_eq!(*at, second);
                assert!(*determinant < 0.0, "the refusal carries the determinant");
            }
            ("nan", EditError::NonFinitePlacement { at, .. })
            | ("scale", EditError::NonRigidPlacement { at, .. }) => assert_eq!(*at, second),
            _ => panic!("the {what} step refused under the wrong arm: {error:?}"),
        }
        let text = error.to_string();
        assert!(
            text.contains("step 2 of node") && text.contains("Recourse:"),
            "the {what} refusal names the step and its recourse: {text}"
        );
    }

    let (doc, placed) = step(doc, at(&Frame::translation([0.0, 0.0, 0.125])));
    let placed = placed.expect("a rigid literal step inserts");
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    let written = |x: f64, column: usize| {
        doctored(&text, |wire| {
            let entry = &mut wire["snapshot"]["nodes"][placed.0.to_string()]["Transform"]["placement"]
                ["steps"][1]["Literal"]["columns"][column][column];
            assert_eq!(
                *entry,
                serde_json::json!(1.0),
                "aimed at the literal's axis"
            );
            *entry = serde_json::json!(x);
        })
    };
    match load(&written(-1.0, 0), Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::PlacementImproper {
            node,
            at,
            determinant,
        })) => {
            assert_eq!((node, at), (placed, second));
            assert!(determinant < 0.0, "the refusal carries the determinant");
        }
        other => panic!("a mirrored step must refuse typed at load, got {other:?}"),
    }
    match load(&written(2.0, 2), Tol::witness()) {
        Err(error @ PersistError::Snapshot(SnapshotError::PlacementNonRigid { .. })) => {
            let PersistError::Snapshot(SnapshotError::PlacementNonRigid { node, at, .. }) = &error
            else {
                unreachable!("matched above")
            };
            assert_eq!((*node, *at), (placed, second));
            assert!(
                error.to_string().ends_with(REGENERATE_RECOURSE),
                "with the regenerate recourse: {error}"
            );
        }
        other => panic!("a scaled step must refuse typed at load, got {other:?}"),
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
    let placement = Placement::literal(&Frame::translation([5.0, 0.0, 0.0])).compose(
        &Step::Rigid {
            translation: [len(0.0), len(0.0), len(0.0)],
            axis: [scl(0.0), scl(0.0), scl(1.0)],
            angle: Expr::param(turn.clone(), Dimension::Angle),
        }
        .into(),
    );
    let (doc, placed) = insert(doc, Node::transform(body, placement));
    let min_x = |doc: &ProfileDoc| {
        let ev = fixture::run(doc, &EvalOptions::default());
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

/// **A later step's expressions are addressed per step and checked at
/// both doors** (the spec's slot ruling): past a literal step, a
/// wrong dimension is refused naming step 2's own slot and an unknown
/// parameter refuses, at the edit door and at load; a write at a later
/// step's slot round-trips through the saved log; the address has no
/// step 0 on the wire, and a step the transform does not hold is an
/// unknown slot.
#[test]
fn a_later_steps_slots_are_addressed_and_checked_at_both_doors() {
    let turn = ParamName::from_static("turn");
    let (doc, body) = cube("placement-later-steps");
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: turn.clone(),
            value: DocParam::continuous(Dimension::Angle, 0.0),
        },
    );
    let chain = |late: Step| {
        Node::transform(
            body,
            Placement {
                steps: vec![
                    about_z([0.0; 3], 0.0),
                    Step::Literal(Frame::translation([0.0, 0.0, 2.0])),
                    late,
                ],
            },
        )
    };
    let door = |doc: &ProfileDoc, edit: DocEdit<ProfileProgram>| {
        editor_core::apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach).map(|_| ())
    };
    let x = SlotId::rigid(2, RigidArg::Translation(Axis3::X));
    match door(
        &doc,
        DocEdit::InsertNode {
            node: chain(Step::Rigid {
                translation: [ang(1.0), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            }),
        },
    ) {
        Err(EditError::SlotDimensionMismatch { slot, .. }) => assert_eq!(slot, x),
        other => panic!("a wrong dimension at step 2 must refuse naming it, got {other:?}"),
    }
    let unknown = Step::Rigid {
        translation: [len(0.0), len(0.0), len(0.0)],
        axis: [scl(0.0), scl(0.0), scl(1.0)],
        angle: Expr::param(ParamName::from_static("nope"), Dimension::Angle),
    };
    assert!(
        door(
            &doc,
            DocEdit::InsertNode {
                node: chain(unknown)
            }
        )
        .is_err(),
        "an unknown parameter at step 2 refuses"
    );

    let snapshot = doc.clone();
    let insert_edit = DocEdit::InsertNode {
        node: chain(Step::Rigid {
            translation: [len(0.0), len(0.0), len(0.0)],
            axis: [scl(0.0), scl(0.0), scl(1.0)],
            angle: Expr::param(turn, Dimension::Angle),
        }),
    };
    let (doc, t) = step(doc, insert_edit.clone());
    let t = t.expect("the chain inserts");
    let slots = doc.node(t).expect("live").slots();
    assert_eq!(slots.len(), 14, "two rigid steps, the literal has none");
    assert!(
        slots[7..]
            .iter()
            .all(|s| s.rigid_arg().map(|(k, _)| k) == Some(2)),
        "the second rigid step is step 2, the literal's index skipped: {slots:?}"
    );
    let set = DocEdit::SetParam {
        node: t,
        slot: SlotId::rigid(2, RigidArg::Translation(Axis3::Y)),
        expr: len(-0.0),
    };
    let (doc, _) = step(doc, set.clone());
    let wire = serde_json::to_string(&set).expect("the edit serializes");
    let back: DocEdit<ProfileProgram> = serde_json::from_str(&wire).expect("and reads back");
    assert_eq!(back, set);
    assert!(
        serde_json::from_str::<DocEdit<ProfileProgram>>(&wire.replace("\"step\":2", "\"step\":0"))
            .is_err(),
        "step 0 has no later-step address"
    );
    let text = save(
        &snapshot,
        &[LoggedEdit::bare(insert_edit), LoggedEdit::bare(set)],
        Tol::witness(),
    )
    .expect("saves with the log");
    assert!(
        load(&text, Tol::witness())
            .expect("loads and replays")
            .doc
            .bit_eq(&doc),
        "the replayed log reaches the same document by bits"
    );
    assert!(
        matches!(
            door(
                &doc,
                DocEdit::SetParam {
                    node: t,
                    slot: SlotId::rigid(1, RigidArg::RotationAngle),
                    expr: ang(0.5),
                }
            ),
            Err(EditError::UnknownSlot { .. })
        ),
        "the literal step has no slot"
    );

    let full = save(&doc, &[], Tol::witness()).expect("saves");
    let key = t.0.to_string();
    let bad_ref = doctored(&full, |w| {
        let angle = &mut w["snapshot"]["nodes"][key.as_str()]["Transform"]["placement"]["steps"][2]
            ["Rigid"]["angle"];
        let text = angle.to_string().replace("\"turn\"", "\"nope\"");
        assert_ne!(text, angle.to_string(), "aimed at the parameter");
        *angle = serde_json::from_str(&text).expect("still an expression");
    });
    assert!(
        load(&bad_ref, Tol::witness()).is_err(),
        "an unknown parameter at step 2 refuses at load"
    );
    let bad_dim = doctored(&full, |w| {
        let steps = &mut w["snapshot"]["nodes"][key.as_str()]["Transform"]["placement"]["steps"];
        let angle = steps[0]["Rigid"]["angle"].clone();
        steps[2]["Rigid"]["translation"][0] = angle;
    });
    match load(&bad_dim, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotDimension { slot, .. })) => {
            assert_eq!(slot, x);
        }
        other => panic!("a wrong dimension at step 2 must refuse at load, got {other:?}"),
    }
}

/// **A chain of literal and rigid steps evaluates in the interval
/// lane**, as a lone literal does.
#[test]
fn a_mixed_chain_evaluates_in_the_interval_lane() {
    let (doc, body) = cube("placement-mixed-interval");
    let placement = Placement::literal(
        &Frame::rotate_then_translate([1.0, 2.0, 3.0], 0.7, [1.0, -2.0, 0.5], band())
            .expect("a definite axis"),
    )
    .compose(&about_z([0.0; 3], core::f64::consts::FRAC_PI_2).into())
    .compose(&Placement::literal(&Frame::translation([0.1, 0.2, 0.3])));
    let (doc, _) = insert(doc, Node::transform(body, placement));
    let bad = crate::corpus::failures(&crate::corpus::eval::<Interval>(&doc));
    assert!(bad.is_empty(), "{bad:?}");
    let bad = crate::corpus::failures(&crate::corpus::eval::<f64>(&doc));
    assert!(bad.is_empty(), "{bad:?}");
}

/// **The content key tells every chain apart.** Two documents under
/// one identity, whose transform differs only in its chain: the two
/// transforms key apart, and B evaluated with A's evaluation as its
/// prior recomputes the transform and places the body as a cold
/// evaluation of B does — over a chain's order, its length, a literal's
/// signed zero, and two rigid steps swapped. (The two transforms are
/// two inserts, so the mint gives them two ids; the key is compared
/// directly, since the memo is by id first.)
#[test]
fn the_content_key_tells_every_chain_apart() {
    let literal = Step::Literal(Frame::translation([0.0, 3.0, 0.0]));
    let turn = about_z([1.0, 0.0, 0.0], core::f64::consts::FRAC_PI_2);
    let cases: [(&str, Vec<Step>, Vec<Step>); 4] = [
        (
            "[rigid, literal] against [literal, rigid]",
            vec![turn.clone(), literal.clone()],
            vec![literal, turn.clone()],
        ),
        (
            "one rigid step against two",
            vec![turn.clone()],
            vec![turn.clone(), turn],
        ),
        (
            "a literal's -0.0 against its 0.0",
            vec![Step::Literal(Frame::translation([-0.0, 0.0, 0.25]))],
            vec![Step::Literal(Frame::translation([0.0, 0.0, 0.25]))],
        ),
        (
            "[rigid a, rigid b] against [rigid b, rigid a]",
            vec![about_z([1.0, 0.0, 0.0], 0.0), about_z([0.0; 3], 1.0)],
            vec![about_z([0.0; 3], 1.0), about_z([1.0, 0.0, 0.0], 0.0)],
        ),
    ];
    let o = EvalOptions::default();
    let run = |doc: &ProfileDoc, prior: Option<&editor_core::Evaluation<f64>>| {
        evaluate::<f64>(doc, prior, &CancelToken::new(), &o, Tol::witness())
    };
    for (what, a, b) in cases {
        let (doc_a, ta) = {
            let (doc, body) = cube("placement-key");
            insert(doc, Node::transform(body, Placement { steps: a }))
        };
        let (doc_b, tb) = {
            let (doc, body) = cube("placement-key");
            insert(doc, Node::transform(body, Placement { steps: b }))
        };
        assert_eq!(doc_a.id(), doc_b.id(), "{what}: one identity");
        let prior = run(&doc_a, None);
        let warm = run(&doc_b, Some(&prior));
        let cold = run(&doc_b, None);
        let key = |ev: &editor_core::Evaluation<f64>, t| {
            ev.value(t).expect("the transform evaluates").content_key
        };
        assert_ne!(
            key(&prior, ta),
            key(&cold, tb),
            "{what}: the two chains key apart"
        );
        assert!(
            warm.recomputed >= 1,
            "{what}: the transform was served from the other chain's evaluation"
        );
        assert_eq!(
            point_bits(&warm, tb, f64_bits),
            point_bits(&cold, tb, f64_bits),
            "{what}: the prior served another placement's body"
        );
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
            Step::Rigid {
                translation: [len(0.0), len(0.0), len(1.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.25),
            },
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

/// **A literal frame compares by bits** (D7): a transform's literal
/// step and an explicit rule's listed frame differing only in a signed
/// zero are two nodes to `Node::bit_eq`, and a saved literal reads back
/// to its own bits. (The content key's half is
/// `the_content_key_tells_every_chain_apart`.)
#[test]
fn a_literal_frame_compares_by_bits() {
    let signed = Frame::translation([-0.0, 0.0, 0.25]);
    let plain = Frame::translation([0.0, 0.0, 0.25]);
    let body = RecipeNodeId(3);
    let transform =
        |f: &Frame| -> Node<ProfileProgram> { Node::transform(body, Placement::literal(f)) };
    assert!(transform(&signed).bit_eq(&transform(&signed)));
    assert!(
        !transform(&signed).bit_eq(&transform(&plain)),
        "a transform's -0.0 is not its 0.0"
    );
    let group = |f: &Frame| -> Node<ProfileProgram> { Node::placed_union_at(body, vec![*f]) };
    assert!(
        !group(&signed).bit_eq(&group(&plain)),
        "an explicit rule's -0.0 is not its 0.0"
    );

    let (doc, input) = cube("placement-bits");
    let (doc, _) = insert(doc, Node::transform(input, Placement::literal(&signed)));
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    let back = load(&text, Tol::witness()).expect("its own bytes load").doc;
    assert!(back.bit_eq(&doc), "the literal round-trips to its own bits");
}

/// **A step reads by its position, counted from one, and has one
/// address both ways.** A later step's slot says which step it edits
/// ("step 3" is the chain's third), the first step keeps the
/// transform's unnumbered names, `SlotId::rigid` and `rigid_arg` are
/// inverses at every index a chain can hold, and a vector family's
/// slots are `SlotId::rigid`'s.
#[test]
fn a_step_reads_by_its_position_counted_from_one() {
    let x = RigidArg::Translation(Axis3::X);
    assert_eq!(SlotId::rigid(0, x).label(), "translation x");
    assert_eq!(SlotId::rigid(2, x).label(), "step 3 translation x");
    assert_eq!(
        SlotId::rigid(1, RigidArg::RotationAngle).label(),
        "step 2 rotation angle"
    );
    assert_eq!(
        VectorSlot::RotationAxis { step: 2 }.label(),
        "step 3 rotation axis"
    );
    for step in [0, 1, 2, usize::MAX] {
        for arg in [x, RigidArg::RotationAxis(Axis3::Z), RigidArg::RotationAngle] {
            let slot = SlotId::rigid(step, arg);
            assert_eq!(slot.rigid_arg(), Some((step, arg)), "{slot:?}");
        }
        assert_eq!(
            VectorSlot::Translation { step }.slot(Axis3::Y),
            SlotId::rigid(step, RigidArg::Translation(Axis3::Y))
        );
    }
    assert_eq!(SlotId::Distance.rigid_arg(), None);
}
