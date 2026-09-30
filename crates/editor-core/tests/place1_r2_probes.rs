//! place1-r2 review probes (head only). Each prints what it measured
//! and asserts the property the PR claims; a failing assert is a finding.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::wire::doctored;

use editor_core::{
    CancelToken, Dimension, DocEdit, DocParam, EditError, EvalOptions, Expr, Frame, LoggedEdit,
    Node, ParamName, Placement, ProfileDoc, RecipeNodeId, RigidArg, SlotId, Step, evaluate, load,
    save,
};
use fixture::{ang, insert, len, on_frame, scl, step};
use geom_core::Tol;

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

fn rigid(t: [f64; 3], angle: f64) -> Step {
    Step::Rigid {
        translation: t.map(len),
        axis: [scl(0.0), scl(0.0), scl(1.0)],
        angle: ang(angle),
    }
}

fn placed(label: &str, placement: Placement) -> (ProfileDoc, RecipeNodeId) {
    let (doc, body) = cube(label);
    insert(
        doc,
        Node::Transform {
            input: body,
            placement,
        },
    )
}

fn points(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> Vec<[u64; 3]> {
    let Some(editor_core::ValuePayload::Body(b)) = ev.value(id).map(|v| &v.payload) else {
        panic!("no body at {id:?}: {:?}", ev.result(id))
    };
    let mut v: Vec<[u64; 3]> = b
        .points()
        .map(|(_, p)| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()])
        .collect();
    v.sort_unstable();
    v
}

/// Claim 4: the memo never answers one placement with another's value.
/// Two documents under ONE label (so one DocumentId and one node-id
/// sequence): evaluate B with A's evaluation as prior; the transform must
/// recompute and match a fresh evaluation of B.
#[test]
fn r2_key_never_serves_another_placement() {
    let m = Frame::translation([0.0, 3.0, 0.0]);
    let r = rigid([1.0, 0.0, 0.0], core::f64::consts::FRAC_PI_2);
    let cases: Vec<(&str, Placement, Placement)> = vec![
        (
            "[rigid, matrix] vs [matrix, rigid]",
            Placement {
                steps: vec![r.clone(), Step::Matrix(m)],
            },
            Placement {
                steps: vec![Step::Matrix(m), r.clone()],
            },
        ),
        (
            "literal -0.0 vs 0.0",
            Placement::literal(&Frame::translation([-0.0, 0.0, 0.25])),
            Placement::literal(&Frame::translation([0.0, 0.0, 0.25])),
        ),
        (
            "[matrix I] vs [matrix I, matrix I]",
            Placement::literal(&Frame::translation([0.0, 0.0, 0.0])),
            Placement {
                steps: vec![
                    Step::Matrix(Frame::translation([0.0, 0.0, 0.0])),
                    Step::Matrix(Frame::translation([0.0, 0.0, 0.0])),
                ],
            },
        ),
        (
            "[rigid a, rigid b] vs [rigid b, rigid a] (same slot multiset)",
            Placement {
                steps: vec![rigid([1.0, 0.0, 0.0], 0.0), rigid([0.0, 0.0, 0.0], 1.0)],
            },
            Placement {
                steps: vec![rigid([0.0, 0.0, 0.0], 1.0), rigid([1.0, 0.0, 0.0], 0.0)],
            },
        ),
    ];
    for (what, a, b) in cases {
        let (doc_a, ta) = placed("r2-key", a);
        let (doc_b, tb) = placed("r2-key", b);
        assert_eq!(ta, tb, "{what}: one id");
        assert_eq!(doc_a.id(), doc_b.id(), "{what}: one document id");
        let o = EvalOptions::default();
        let prior = evaluate::<f64>(&doc_a, None, &CancelToken::new(), &o, Tol::witness());
        let warm = evaluate::<f64>(&doc_b, Some(&prior), &CancelToken::new(), &o, Tol::witness());
        let cold = evaluate::<f64>(&doc_b, None, &CancelToken::new(), &o, Tol::witness());
        println!(
            "KEY {what}: warm reused {} recomputed {}; prior_refused {:?}",
            warm.reused, warm.recomputed, warm.prior_refused
        );
        assert!(warm.recomputed >= 1, "{what}: the transform must recompute");
        assert_eq!(points(&warm, tb), points(&cold, tb), "{what}: memo served another placement");
    }
}

/// Claim 3: every chain expression reaches the dimension and
/// parameter-reference checks at the EDIT door, past a matrix step.
#[test]
fn r2_edit_door_walks_a_later_step() {
    let (doc, body) = cube("r2-edit-door");
    let chain = |late: Step| Node::Transform {
        input: body,
        placement: Placement {
            steps: vec![rigid([0.0; 3], 0.0), Step::Matrix(Frame::IDENTITY), late],
        },
    };
    // wrong dimension in step 2's translation x
    let bad_dim = Step::Rigid {
        translation: [ang(1.0), len(0.0), len(0.0)],
        axis: [scl(0.0), scl(0.0), scl(1.0)],
        angle: ang(0.0),
    };
    let r = editor_core::apply(
        &doc,
        &DocEdit::InsertNode { node: chain(bad_dim) },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    println!("EDITDOOR dim: {:?}", r.as_ref().err());
    match r {
        Err(EditError::SlotDimensionMismatch { slot, .. }) => assert_eq!(
            slot,
            SlotId::rigid(2, RigidArg::Translation(editor_core::Axis3::X))
        ),
        other => panic!("dimension at step 2 not refused typed: {:?}", other.err()),
    }
    // unknown param in step 2's angle
    let bad_ref = Step::Rigid {
        translation: [len(0.0), len(0.0), len(0.0)],
        axis: [scl(0.0), scl(0.0), scl(1.0)],
        angle: Expr::param(ParamName::from_static("nope"), Dimension::Angle),
    };
    let r = editor_core::apply(
        &doc,
        &DocEdit::InsertNode { node: chain(bad_ref) },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    println!("EDITDOOR ref: {}", r.as_ref().err().map(|e| e.to_string()).unwrap_or_default());
    assert!(r.is_err(), "an unknown parameter at step 2 must refuse");
    // SetParam at a matrix step's would-be slot
    let (doc2, t) = insert(doc.clone(), chain(rigid([0.0; 3], 0.0)));
    let r = editor_core::apply(
        &doc2,
        &DocEdit::SetParam {
            node: t,
            slot: SlotId::rigid(1, RigidArg::RotationAngle),
            expr: ang(0.5),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    println!("EDITDOOR setparam at matrix step 1: {:?}", r.as_ref().err());
    assert!(matches!(r, Err(EditError::UnknownSlot { .. })));
}

/// Claim 3: the load door walks a later step (param ref + dimension), and
/// a SetParam at a PlacementStep slot round-trips through the saved log.
#[test]
fn r2_load_door_and_log_walk_a_later_step() {
    let turn = ParamName::from_static("turn");
    let (doc, body) = cube("r2-load");
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: turn.clone(),
            value: DocParam::continuous(Dimension::Angle, 0.0),
        },
    );
    let snapshot = doc.clone();
    let node = Node::Transform {
        input: body,
        placement: Placement {
            steps: vec![
                rigid([0.0; 3], 0.0),
                Step::Matrix(Frame::translation([0.0, 0.0, 2.0])),
                Step::Rigid {
                    translation: [len(0.0), len(0.0), len(0.0)],
                    axis: [scl(0.0), scl(0.0), scl(1.0)],
                    angle: Expr::param(turn.clone(), Dimension::Angle),
                },
            ],
        },
    };
    let e1 = DocEdit::InsertNode { node };
    let (doc, t) = step(doc, e1.clone());
    let t = t.unwrap();
    let e2 = DocEdit::SetParam {
        node: t,
        slot: SlotId::rigid(2, RigidArg::Translation(editor_core::Axis3::Y)),
        expr: len(-0.0),
    };
    let (doc, _) = step(doc, e2.clone());
    let text = save(
        &snapshot,
        &[LoggedEdit::bare(e1), LoggedEdit::bare(e2.clone())],
        Tol::witness(),
    )
    .expect("saves with the log");
    let setparam_wire = serde_json::to_string(&e2).unwrap();
    println!("LOG SetParam wire: {setparam_wire}");
    let back = load(&text, Tol::witness()).expect("loads and replays");
    assert!(back.doc.bit_eq(&doc), "the replayed log reaches the same document by bits");
    // step-0 slot wire unchanged
    let s0 = serde_json::to_string(&SlotId::Translation(editor_core::Axis3::X)).unwrap();
    println!("LOG step-0 slot wire: {s0}");
    assert_eq!(s0, r#"{"Translation":"X"}"#);

    // load door: a later step's param ref
    let full = save(&doc, &[], Tol::witness()).unwrap();
    let key = t.0.to_string();
    let bad_ref = doctored(&full, |w| {
        let angle = &mut w["snapshot"]["nodes"][key.as_str()]["Transform"]["placement"]["steps"][2]
            ["Rigid"]["angle"];
        let s = angle.to_string().replace("\"turn\"", "\"nope\"");
        *angle = serde_json::from_str(&s).unwrap();
    });
    let r = load(&bad_ref, Tol::witness());
    println!("LOADDOOR ref: {:?}", r.as_ref().err());
    assert!(r.is_err(), "an unknown param at step 2 must refuse at load");
    let bad_dim = doctored(&full, |w| {
        let steps = &mut w["snapshot"]["nodes"][key.as_str()]["Transform"]["placement"]["steps"];
        let angle = steps[0]["Rigid"]["angle"].clone();
        steps[2]["Rigid"]["translation"][0] = angle;
    });
    let r = load(&bad_dim, Tol::witness());
    println!("LOADDOOR dim: {:?}", r.as_ref().err());
    assert!(r.is_err(), "a wrong dimension at step 2 must refuse at load");
    // non-finite through JSON: an overflowing literal
    let wire_step1 = {
        let (_, w) = full.split_once('\n').unwrap_or(("", &full));
        w.to_owned()
    };
    let _ = wire_step1;
    let over = full.replacen("\"translation\":[0.0,0.0,2.0]", "\"translation\":[0.0,0.0,1e309]", 1);
    println!("NONFINITE text replaced: {}", over != full);
    if over != full {
        println!("NONFINITE load: {:?}", load(&over, Tol::witness()).err());
    }
}

/// Claim 6: the refusal texts, and a non-rigid literal at the doors.
#[test]
fn r2_refusal_texts_and_a_non_rigid_literal() {
    let (doc, body) = cube("r2-texts");
    let mut nan = Frame::IDENTITY;
    nan.translation[0] = f64::NAN;
    let mirror = Frame {
        columns: [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0; 3],
    };
    for f in [nan, mirror] {
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
        let s = r.err().unwrap().to_string();
        println!("TEXT edit ({} words): {s}", s.split_whitespace().count());
    }
    for e in [
        editor_core::SnapshotError::PlacementNonFinite { node: body },
        editor_core::SnapshotError::PlacementImproper {
            node: body,
            determinant: -1.0,
        },
    ] {
        let s = editor_core::PersistError::Snapshot(e).to_string();
        println!("TEXT load ({} words): {s}", s.split_whitespace().count());
    }
    // A finite, proper, NON-RIGID literal: what do the doors and the
    // evaluation do with it?
    let scale = Frame {
        columns: [[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 2.0]],
        translation: [0.0; 3],
    };
    let r = editor_core::apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Transform {
                input: body,
                placement: Placement::literal(&scale),
            },
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    println!("NONRIGID edit door: {:?}", r.as_ref().err());
    if let Ok(applied) = r {
        let d = applied.doc;
        let ev = fixture::run(&d, &EvalOptions::default());
        let id = *d.order().last().unwrap();
        println!("NONRIGID eval: {:?}", ev.result(id).map(|r| format!("{r:?}").chars().take(300).collect::<String>()));
    }
}

fn part_block(label: &str, w: f64, h: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(editor_core::DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    insert(doc, Node::Extrude { profile, distance: len(h) }).0
}

/// Claim 9 / 5: a mate read through a transform whose chain holds a
/// finite, proper, NON-RIGID literal. The edit door admits the literal
/// (A6 is finite + proper); the node evaluation refuses NotRigid; what
/// does the mate solve (which calls `placement.eval`, no rigidity check)
/// answer?
#[test]
fn r2_mate_through_a_non_rigid_literal() {
    use editor_core::{Alignment, AxisSense, CapEnd, ContactClass, MateFrame, MatePrimitive};
    use fixture::resolver::{PartStore, in_part, with_resolver};
    for (what, frame) in [
        ("identity literal", Frame::IDENTITY),
        (
            "scale-2 literal",
            Frame {
                columns: [[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 2.0]],
                translation: [0.0; 3],
            },
        ),
        (
            "shear literal",
            Frame {
                columns: [[1.0, 0.0, 0.0], [0.5, 1.0, 0.0], [0.0, 0.0, 1.0]],
                translation: [0.0; 3],
            },
        ),
    ] {
        let mut store = PartStore::default();
        let base_ref = store.insert(part_block(&format!("r2m-{what}-base"), 3.0, 1.0), Tol::witness());
        let top_ref = store.insert(part_block(&format!("r2m-{what}-top"), 1.0, 3.0), Tol::witness());
        let opts = with_resolver(store);
        let doc = ProfileDoc::empty(editor_core::DocumentId::derive(&format!("r2m-{what}")), Tol::witness());
        let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
        let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
        let (doc, b_at) = insert(
            doc,
            Node::Transform {
                input: top,
                placement: Placement::literal(&frame),
            },
        );
        let mate = Node::Mate {
            a: fixture::head_at(base, in_part(base, CapEnd::End)),
            b: fixture::head_at(b_at, in_part(top, CapEnd::Start)),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: MateFrame { origin: [1.0, 1.0, 1.0], axis: [0.0, 0.0, 1.0], reference: [1.0, 0.0, 0.0] },
                b: MateFrame { origin: [0.0, 0.0, 0.0], axis: [0.0, 0.0, -1.0], reference: [1.0, 0.0, 0.0] },
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Opposed,
                clocking: None,
            },
        };
        let r = editor_core::apply(&doc, &DocEdit::InsertNode { node: mate }, Tol::witness(), &editor_core::RefusingReach);
        let doc = match r {
            Ok(a) => a.doc,
            Err(e) => {
                println!("MATE {what}: insert refused: {e}");
                continue;
            }
        };
        let mate_id = *doc.order().last().unwrap();
        let poses = fixture::solve(&doc, &opts, Tol::witness());
        let ev = fixture::run(&doc, &opts);
        println!(
            "MATE {what}: mate fault {:?}; top fault {:?}; relative(top) {:?}",
            poses.fault(mate_id).map(|f| f.to_string()),
            poses.fault(top).map(|f| f.to_string()),
            poses.relative(top)
        );
        println!(
            "MATE {what}: eval top {:?}; eval b_at {:?}",
            ev.result(top).map(|r| format!("{r:?}").chars().take(160).collect::<String>()),
            ev.result(b_at).map(|r| format!("{r:?}").chars().take(160).collect::<String>())
        );
    }
}

/// Style probe: does an identity literal step compose bit-exactly, as
/// `Frame::compose`'s identity fast path does?
#[test]
fn r2_identity_literal_step_is_not_bit_neutral() {
    let band = geom_core::predicate::Band::linear(Tol::witness()).unwrap();
    let env = editor_core::ParamEnv::default();
    let bits = |p: &Placement| {
        let a = p.eval::<f64>(&env, band).unwrap();
        [a.linear.c0, a.linear.c1, a.linear.c2, a.translation]
            .iter()
            .flat_map(|c| [c.x.to_bits(), c.y.to_bits(), c.z.to_bits()])
            .collect::<Vec<u64>>()
    };
    let r = rigid([-0.0, 2.0, 0.1], 0.0);
    let one = Placement { steps: vec![r.clone()] };
    let before = Placement { steps: vec![Step::Matrix(Frame::IDENTITY), r.clone()] };
    let after = Placement { steps: vec![r.clone(), Step::Matrix(Frame::IDENTITY)] };
    println!("IDENT [r] == [I, r]: {}", bits(&one) == bits(&before));
    println!("IDENT [r] == [r, I]: {}", bits(&one) == bits(&after));
    let f = Frame::from_affine(one.eval::<f64>(&env, band).unwrap());
    println!("IDENT Frame::compose(I, f) bit_eq f: {}", Frame::IDENTITY.compose(&f).bit_eq(&f));
    println!("IDENT f = {f:?}");
    println!("IDENT [I, r] = {:?}", Frame::from_affine(before.eval::<f64>(&env, band).unwrap()));
}
