//! place1-r1 review probes for PR 3497 (the `Placement` type, P1).
//! Each row prints what it measured; the assertions state the
//! reviewer's expectation of the contract.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::sync::Arc;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, EditError, EvalOptions,
    Frame, MateFrame, MatePrimitive, Node, NodeResult, ParamEnv, Placement, ProfileDoc,
    ProfileProgram, RecipeNodeId, RigidArg, SlotId, StableName, Step, ValuePayload, evaluate,
    load, save,
};
use fixture::resolver::{PartStore, in_part};
use fixture::{ang, insert, len, on_frame, run, scl, solve, step};
use editor_core::CancelToken;
use geom_core::{Interval, Point3, Tol};

fn band() -> geom_core::Band {
    fixture::band()
}

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

fn quarter_turn() -> Step {
    Step::Rigid {
        translation: [len(0.0), len(0.0), len(0.0)],
        axis: [scl(0.0), scl(0.0), scl(1.0)],
        angle: ang(core::f64::consts::FRAC_PI_2),
    }
}

fn sorted_points(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> Vec<[u64; 3]> {
    let Some(ValuePayload::Body(body)) = ev.value(id).map(|v| &v.payload) else {
        panic!("node {} evaluated to no body: {:?}", id.0, ev.result(id))
    };
    let mut out: Vec<[u64; 3]> = body
        .points()
        .map(|(_, p)| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()])
        .collect();
    out.sort_unstable();
    out
}

/// Claim 2: `Placement::then` against the module's own
/// `Frame::rotate_then_translate`. "rotate THEN translate" means the
/// rotation acts on the body first; `rigid(rotate).then(translate)`
/// makes the TRANSLATION act first.
#[test]
fn p2_then_reads_opposite_to_rotate_then_translate() {
    let env = ParamEnv::<f64>::default();
    let chained = Placement {
        steps: vec![quarter_turn()],
    }
    .then(Step::Matrix(Frame::translation([1.0, 0.0, 0.0])));
    let by_then = chained.eval::<f64>(&env, band()).unwrap();
    let rtt = Frame::rotate_then_translate(
        [0.0, 0.0, 1.0],
        core::f64::consts::FRAC_PI_2,
        [1.0, 0.0, 0.0],
        band(),
    )
    .unwrap();
    let origin = Point3::new(0.0, 0.0, 0.0);
    let a = by_then.transform_point(origin);
    let b = rtt.affine::<f64>().transform_point(origin);
    eprintln!("PROBE p2: rotate.then(translate) sends origin to {a:?}");
    eprintln!("PROBE p2: rotate_then_translate sends origin to {b:?}");
    assert!(
        (a.x - b.x).abs() < 1e-12 && (a.y - b.y).abs() < 1e-12,
        "`then` composes opposite to `rotate_then_translate`: {a:?} vs {b:?}"
    );
}

/// Claim 3: `[matrix, rigid]` — the slot table, a SetParam at step 1,
/// save/load and the logged edit's wire.
#[test]
fn p3_matrix_rigid_addressing_and_wire() {
    let (doc, body) = cube("r1-p3");
    let placement =
        Placement::literal(&Frame::translation([5.0, 0.0, 0.0])).then(quarter_turn());
    let (doc, placed) = insert(
        doc,
        Node::Transform {
            input: body,
            placement,
        },
    );
    let slots = doc.node(placed).unwrap().slots();
    eprintln!("PROBE p3 slots: {slots:?}");
    assert_eq!(slots.len(), 7);
    assert!(
        slots
            .iter()
            .all(|s| matches!(s, SlotId::PlacementStep { step, .. } if step.get() == 1))
    );
    let labels: Vec<String> = slots.iter().map(|s| s.label()).collect();
    eprintln!("PROBE p3 labels: {labels:?}");
    let edit = DocEdit::SetParam {
        node: placed,
        slot: SlotId::rigid(1, RigidArg::RotationAngle),
        expr: ang(0.0),
    };
    let wire = serde_json::to_string(&edit).unwrap();
    eprintln!("PROBE p3 SetParam wire: {wire}");
    let back: DocEdit<ProfileProgram> = serde_json::from_str(&wire).unwrap();
    assert_eq!(back, edit);
    let zero = wire.replace("\"step\":1", "\"step\":0");
    let refused = serde_json::from_str::<DocEdit<ProfileProgram>>(&zero);
    eprintln!("PROBE p3 step 0 on the wire: {refused:?}");
    assert!(refused.is_err(), "step 0 has no PlacementStep address");
    let (doc2, _) = step(doc.clone(), edit);
    let text = save(&doc2, &[], Tol::witness()).unwrap();
    let back = load(&text, Tol::witness()).unwrap().doc;
    assert!(back.bit_eq(&doc2));
    // A later step's slot on a one-step placement: typed refusal.
    let (doc3, one) = insert(doc.clone(), fixture::xform(body, [0.0; 3], [0.0, 0.0, 1.0], 0.0));
    let r = editor_core::apply(
        &doc3,
        &DocEdit::SetParam {
            node: one,
            slot: SlotId::rigid(1, RigidArg::RotationAngle),
            expr: ang(0.0),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("PROBE p3 step-1 slot on a one-step transform: {:?}", r.as_ref().err());
    assert!(matches!(r, Err(EditError::UnknownSlot { .. })));
}

/// Claim 3: the dimension and parameter-reference checks at a later
/// step, at the edit door.
#[test]
fn p3_later_step_dimension_and_param_ref_checks() {
    let (doc, body) = cube("r1-p3b");
    let bad_dim = Placement::literal(&Frame::IDENTITY).then(Step::Rigid {
        translation: [len(0.0), len(0.0), len(0.0)],
        axis: [scl(0.0), scl(0.0), scl(1.0)],
        angle: len(1.0),
    });
    let r = editor_core::apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Transform {
                input: body,
                placement: bad_dim,
            },
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("PROBE p3b dim: {:?}", r.as_ref().err());
    assert!(r.is_err());
    let bad_ref = Placement::literal(&Frame::IDENTITY).then(Step::Rigid {
        translation: [len(0.0), len(0.0), len(0.0)],
        axis: [scl(0.0), scl(0.0), scl(1.0)],
        angle: editor_core::Expr::param(
            editor_core::ParamName::from_static("nope"),
            editor_core::Dimension::Angle,
        ),
    });
    let r = editor_core::apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Transform {
                input: body,
                placement: bad_ref,
            },
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("PROBE p3b ref: {:?}", r.as_ref().err());
    assert!(r.is_err());
}

/// Claim 4: the evaluation memo. Two documents of ONE identity whose
/// transform (same id) differs only in the chain; B is evaluated with
/// A's evaluation as its prior. B must place its body as B does alone.
fn memo_pair(label: &str, pa: Placement, pb: Placement) {
    let build = |p: Placement| {
        let (doc, body) = cube(label);
        insert(
            doc,
            Node::Transform {
                input: body,
                placement: p,
            },
        )
    };
    let (a, ida) = build(pa);
    let (b, idb) = build(pb);
    assert_eq!(ida, idb);
    assert_eq!(a.id(), b.id());
    let opts = EvalOptions::default();
    let ev_a = evaluate::<f64>(&a, None, &CancelToken::new(), &opts, Tol::witness());
    let ev_b_alone = evaluate::<f64>(&b, None, &CancelToken::new(), &opts, Tol::witness());
    let ev_b_memo = evaluate::<f64>(&b, Some(&ev_a), &CancelToken::new(), &opts, Tol::witness());
    let alone = sorted_points(&ev_b_alone, idb);
    let memo = sorted_points(&ev_b_memo, idb);
    let other = sorted_points(&ev_a, ida);
    let ev_b_self = evaluate::<f64>(&b, Some(&ev_b_alone), &CancelToken::new(), &opts, Tol::witness());
    eprintln!(
        "PROBE p4 {label}: a!=b alone {} ; memo==alone {} ; reused with A's prior {} of {} (self prior {})",
        other != alone,
        memo == alone,
        ev_b_memo.reused,
        b.order().len(),
        ev_b_self.reused
    );
    assert_eq!(memo, alone, "{label}: the memo served another placement's body");
    assert!(
        ev_b_memo.reused < ev_b_self.reused,
        "{label}: the transform was reused across two placements"
    );
}

#[test]
fn p4_the_memo_never_serves_another_placement() {
    let m = Frame::translation([0.0, 0.25, 0.0]);
    let r = || Step::Rigid {
        translation: [len(0.5), len(0.0), len(0.0)],
        axis: [scl(0.0), scl(0.0), scl(1.0)],
        angle: ang(0.3),
    };
    memo_pair(
        "r1-p4-order",
        Placement {
            steps: vec![r(), Step::Matrix(m)],
        },
        Placement {
            steps: vec![Step::Matrix(m), r()],
        },
    );
    // Same numbers, different chain length: two identical rigid steps
    // against one (the slot stream alone repeats).
    memo_pair(
        "r1-p4-length",
        Placement { steps: vec![r()] },
        Placement {
            steps: vec![r(), r()],
        },
    );
    // A turn about -0.0-carrying frames: a signed zero in a literal.
    let signed = Frame {
        columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [-0.0, 0.0, 0.0],
    };
    let plain = Frame {
        columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0, 0.0, 0.0],
    };
    memo_pair(
        "r1-p4-signed",
        Placement::literal(&signed),
        Placement::literal(&plain),
    );
}

/// Claim 9: a chain holding a matrix AND a rigid step evaluates in the
/// interval lane, and its enclosure holds the f64 body.
#[test]
fn p9_a_mixed_chain_evaluates_in_the_interval_lane() {
    let (doc, body) = cube("r1-p9");
    let placement = Placement::literal(&Frame::rotate_then_translate(
        [1.0, 2.0, 3.0],
        0.7,
        [1.0, -2.0, 0.5],
        band(),
    )
    .unwrap())
    .then(quarter_turn())
    .then(Step::Matrix(Frame::translation([0.1, 0.2, 0.3])));
    let (doc, placed) = insert(
        doc,
        Node::Transform {
            input: body,
            placement,
        },
    );
    let lane = crate::corpus::eval::<Interval>(&doc);
    let bad = crate::corpus::failures(&lane);
    eprintln!("PROBE p9 interval failures: {bad:?}");
    assert!(bad.is_empty());
    let ev = run(&doc, &EvalOptions::default());
    assert!(matches!(ev.result(placed), Some(NodeResult::Ok(_))));
}

// ---- claim 5/9: a proper but NON-RIGID literal ----

fn block(label: &str) -> ProfileDoc {
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
            distance: len(1.0),
        },
    );
    doc
}

fn seat(a: StableName, b: StableName) -> Node<ProfileProgram> {
    let frame = |origin: [f64; 3], axis: [f64; 3]| MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    };
    Node::Mate {
        a: fixture::head(a),
        b: fixture::head(b),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// A scale-2 literal (determinant 8 — "proper" by A6's sign test) on a
/// transform both doors admit; what do evaluation and the mate solve
/// do with it?
#[test]
fn p5_a_non_rigid_literal() {
    let scale = Frame {
        columns: [[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 2.0]],
        translation: [0.0, 0.0, 0.0],
    };
    let mut store = PartStore::new();
    let leg = store.insert(block("r1-p5-leg"), Tol::witness());
    let top = store.insert(block("r1-p5-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r1-p5"), Tol::witness());
    let (doc, legs) = insert(doc, Node::instantiate_part(leg));
    let (doc, placer) = insert(
        doc,
        Node::Transform {
            input: legs,
            placement: Placement::literal(&scale),
        },
    );
    let (doc, cap) = insert(doc, Node::instantiate_part(top));
    let mut node = seat(in_part(legs, CapEnd::End), in_part(cap, CapEnd::Start));
    if let Node::Mate { a, .. } = &mut node {
        *a = fixture::head_at(placer, (*a.name).clone());
    }
    let applied = doc.apply(
        &DocEdit::InsertNode { node },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("PROBE p5 mate insert: {:?}", applied.as_ref().err());
    let doc = applied.unwrap().doc;
    let text = save(&doc, &[], Tol::witness()).unwrap();
    eprintln!(
        "PROBE p5 load of the scaled literal: {:?}",
        load(&text, Tol::witness()).map(|_| ())
    );
    let store = Arc::new(store);
    let opts = EvalOptions {
        resolver: Some(Arc::clone(&store) as Arc<dyn editor_core::PartResolver>),
        ..EvalOptions::default()
    };
    let poses = solve(&doc, &opts, Tol::witness());
    let mate = doc
        .order()
        .iter()
        .copied()
        .find(|id| matches!(doc.node(*id), Some(Node::Mate { .. })))
        .unwrap();
    eprintln!("PROBE p5 mate fault: {:?}", poses.fault(mate));
    eprintln!("PROBE p5 cap relative pose: {:?}", poses.relative(cap));
    let ev = run(&doc, &opts);
    eprintln!("PROBE p5 placer row: {:?}", ev.result(placer).map(|r| format!("{r:?}")));
    eprintln!("PROBE p5 cap row: {:?}", ev.result(cap).map(|r| format!("{r:?}")));
    eprintln!("PROBE p5 legs row ok: {:?}", matches!(ev.result(legs), Some(NodeResult::Ok(_))));
    if let Some(ValuePayload::Body(b)) = ev.value(cap).map(|v| &v.payload) {
        let zs: Vec<f64> = b.points().map(|(_, p)| p.z).collect();
        eprintln!(
            "PROBE p5 cap z range: {:?}..{:?}",
            zs.iter().copied().fold(f64::INFINITY, f64::min),
            zs.iter().copied().fold(f64::NEG_INFINITY, f64::max)
        );
    }
}

/// Claim 6: the reused arms, as a user reads them, from the new door.
#[test]
fn p6_the_reused_arms_read_from_a_transform() {
    let (doc, body) = cube("r1-p6");
    let mirror = Frame {
        columns: [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0, 0.0, 0.0],
    };
    let mut nan = Frame::IDENTITY;
    nan.translation[0] = f64::NAN;
    for (what, f) in [("mirror", mirror), ("nan", nan)] {
        let r = editor_core::apply(
            &doc,
            &DocEdit::InsertNode {
                node: Node::Transform {
                    input: body,
                    placement: Placement::literal(&f),
                },
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        );
        let text = r.err().map(|e| e.to_string()).unwrap_or_default();
        eprintln!("PROBE p6 edit {what} ({} words): {text}", text.split_whitespace().count());
    }
    let (doc, placed) = insert(
        doc,
        Node::Transform {
            input: body,
            placement: Placement::literal(&Frame::translation([0.0, 0.0, 1.0])),
        },
    );
    let text = save(&doc, &[], Tol::witness()).unwrap();
    let corrupt = crate::wire::doctored(&text, |wire| {
        wire["snapshot"]["nodes"][placed.0.to_string()]["Transform"]["placement"]["steps"][0]
            ["Matrix"]["columns"][0][0] = serde_json::json!(-1.0);
    });
    let e = load(&corrupt, Tol::witness()).err().unwrap().to_string();
    eprintln!("PROBE p6 load mirror ({} words): {e}", e.split_whitespace().count());
}
