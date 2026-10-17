//! Reviewer r2 probes for PR 5232 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::resolver::{PartStore, in_part, with_resolver};
use crate::fixture::{self, at_the_door_of, insert, len, mated_instance, on_frame, run, square, step};
use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, ExtrudeSide, Formula,
    MateFault, MateFrame, MatePrimitive, Node, NodeErrorKind, NodeResult, Placement, ProfileDoc,
    RecipeNodeId, RoleSeg,
};
use geom_core::Tol;

fn block(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

fn host(label: &str) -> (ProfileDoc, PartStore, editor_core::DocRef, RecipeNodeId) {
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(block(&format!("{label}-part")), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    (doc, store, doc_ref, body)
}

fn z_up() -> MateFrame<Formula> {
    MateFrame::authored([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], Tol::witness()).unwrap()
}

fn cap_mate(
    body: RecipeNodeId,
    a: RecipeNodeId,
    b: RecipeNodeId,
    primitive: MatePrimitive,
    clocking: Option<f64>,
) -> editor_core::Mate<Formula> {
    fixture::mate(
        fixture::head(in_part(a, body, CapEnd::Start)),
        fixture::head(in_part(b, body, CapEnd::Start)),
        ContactClass::Rest,
        Alignment {
            a: z_up(),
            b: z_up(),
            primitive,
            sense: AxisSense::Aligned,
            clocking,
        },
    )
}

/// B4's named interim is author-reachable: a face side on a `Place`
/// copy. What does the refusal say?
#[test]
fn r2_b4_face_side_on_a_place_copy() {
    let (doc, body) = block("r2-b4");
    let (doc, place) = step(doc, DocEdit::place(vec![body.into()], vec![]));
    let place = place.unwrap();
    let side = editor_core::Mate {
        a: editor_core::SideRead::World,
        b: editor_core::SideRead::Face(
            fixture::head(fixture::fname(body, RoleSeg::Cap(CapEnd::End))).into(),
        ),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame {
                base: editor_core::FrameBase::World,
                offset: Placement::IDENTITY,
            },
            b: MateFrame::from_face(),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    };
    let o = editor_core::EvalOptions::default();
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    match at_the_door_of(&doc, &reach, place, side.clone()) {
        Ok(_) => eprintln!("R2-B4 door admitted"),
        Err((_, fault)) => eprintln!("R2-B4 door refused: {fault}"),
    }
    let (doc, _) = fixture::constrain_at(doc, place, side);
    let ev = run(&doc, &o);
    match ev.result(place) {
        Some(NodeResult::Failed(e)) => {
            let text = format!("{}", e.kind);
            eprintln!("R2-B4 eval: {text}");
            if let NodeErrorKind::Mate(f) = &e.kind {
                eprintln!("R2-B4 spoken: {}", f.spoken(&doc));
            }
            assert!(
                !text.contains("kernel defect"),
                "an author-reachable named interim says it is a kernel defect: {text}"
            );
        }
        other => panic!("R2-B4 not failed: {other:?}"),
    }
}

/// The door's verdict depends on the order the constraints arrive in.
#[test]
fn r2_door_is_order_dependent() {
    let (doc, store, doc_ref, body) = host("r2-order");
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let o = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let coax = || cap_mate(body, a, b, MatePrimitive::Coaxial, Some(0.0));
    let pin = || cap_mate(body, a, b, MatePrimitive::FrameCoincidence, None);
    // coaxial, then a frame coincidence that pins alone
    let first = at_the_door_of(&doc, &reach, b, coax()).map(|(d, _)| d);
    let first = first.expect("coaxial admits");
    let second = at_the_door_of(&first, &reach, b, pin());
    let coax_then_pin_admitted = second.is_ok();
    if let Ok((d, _)) = &second {
        let ev = run(d, &o);
        eprintln!("R2-ORDER coax->pin eval: {:?}", ev.result(b).map(|r| matches!(r, NodeResult::Ok(_))));
        if let Some(NodeResult::Failed(e)) = ev.result(b) {
            eprintln!("R2-ORDER coax->pin refusal: {}", e.kind);
        }
    }
    // the other order
    let first = at_the_door_of(&doc, &reach, b, pin()).map(|(d, _)| d).expect("pin admits");
    let second = at_the_door_of(&first, &reach, b, coax());
    let pin_then_coax_refused = matches!(second, Err((_, MateFault::Overconstrained { .. })));
    eprintln!("R2-ORDER coax->pin admitted={coax_then_pin_admitted} pin->coax overconstrained={pin_then_coax_refused}");
    assert!(
        !(coax_then_pin_admitted && pin_then_coax_refused),
        "one set of constraints, two door verdicts by arrival order"
    );
}

/// How the door's bundle refusal reads as an edit error.
#[test]
fn r2_overconstrained_edit_error_text() {
    let (doc, store, doc_ref, body) = host("r2-text");
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(doc_ref));
    let o = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let refused = doc
        .apply(
            &DocEdit::AddConstraint {
                placement: b,
                mate: Box::new(cap_mate(body, a, b, MatePrimitive::Coaxial, Some(0.0))),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &reach,
        )
        .err()
        .expect("refused");
    let text = refused.to_string();
    eprintln!("R2-TEXT: {text}");
    eprintln!("R2-TEXT words: {}", text.split_whitespace().count());
    assert!(
        !text.contains("on its own datum"),
        "a bundle verdict reported as the constraint's own datum: {text}"
    );
}

/// The content key skips an own-copy side read through a placer of
/// the copy (`Doc::upstream_of`'s exemption). Editing that placer moves
/// the copy; a memo run over the prior must move it too.
#[test]
fn r2_own_copy_side_through_a_pattern_memo() {
    use crate::p2_gauges::{body_of, min_corner, parts};
    use editor_core::{CancelToken, PatternKind, SlotId, SlotValue};
    let p = parts("r2-own-copy-memo");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-own-copy-memo"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(p.top));
    let (doc, mover) = insert(doc, mated_instance(p.base));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: mover.into(),
            count: Formula::count(3),
            kind: PatternKind::Linear {
                direction: [fixture::scl(1.0), fixture::scl(0.0), fixture::scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let mate = fixture::mate(
        fixture::head(fixture::in_copy(pattern, 1, in_part(mover, p.base_body, CapEnd::Start))),
        fixture::head(p.top_upper_cap(leg)),
        ContactClass::Rest,
        Alignment {
            a: MateFrame::authored([0.0; 3], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0], Tol::witness())
                .unwrap(),
            b: MateFrame::from_face(),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    );
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (doc, _) = match at_the_door_of(&doc, &reach, mover, mate) {
        Ok(x) => x,
        Err((_, f)) => panic!("admitted: {f}"),
    };
    let ev = |d: &ProfileDoc, prior| {
        editor_core::evaluate::<f64>(d, prior, &CancelToken::new(), &o, Tol::witness())
    };
    let first = ev(&doc, None);
    let before = min_corner(&body_of(&first, mover));
    let (doc2, _) = step(
        doc,
        DocEdit::SetParam {
            node: pattern,
            slot: SlotId::Spacing,
            value: SlotValue::Formula(len(3.0)),
            fresh: Vec::new(),
        },
    );
    let fresh = min_corner(&body_of(&ev(&doc2, None), mover));
    let memo = min_corner(&body_of(&ev(&doc2, Some(&first)), mover));
    eprintln!("R2-MEMO before={before:?} fresh={fresh:?} memo={memo:?}");
    assert_ne!(before, fresh, "the spacing edit moves the mover");
    assert_eq!(memo, fresh, "a memo run serves the copy where a fresh run puts it");
}
