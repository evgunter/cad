//! **A shaft mated in a full-turn bore names cleanly while the boolean
//! lists the partners it read nothing of** (`topo::UnreadPartner`).
//!
//! The collar is the rectangle ρ ∈ [0.3, 0.6], y ∈ [0, 1] revolved a full
//! turn about `y`; the shaft a circle of the bore's radius on a `+y`
//! frame, its vertex turned `a` from the collar's seam, extruded over
//! four spans. Where a shaft vertex meets the collar at a rim's vertex,
//! the bore and the end face hold no subdivision bisector to read their
//! wedge by, so the boolean classes none of the shaft's edges there and
//! lists the vertex. Naming never reads a crossing there: a listed
//! vertex is a member's own, and a finished vertex at a member vertex is
//! named by that identity, so the union names cleanly in every case.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    CancelToken, EvalOptions, Evaluation, LoopProgram, Node, NodeResult, ProfileDoc,
    ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, ValuePayload, evaluate,
};
use geom_core::Tol;
use std::f64::consts::{FRAC_PI_2, TAU};

use crate::fixture::{against, ang, frame_at, insert, len, len2, revolve_about};

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The evaluated body of node `n`, finished as an operand.
fn operand(ev: &Evaluation<f64>, n: RecipeNodeId) -> topo::AtRestBody<f64> {
    match &ev.value(n).expect("the sweep evaluates").payload {
        ValuePayload::Body(b) => {
            sweep::test_support::finished("the operand", (**b).clone(), Tol::witness())
        }
        ValuePayload::Copies(copies) if copies.bodies.len() == 1 => sweep::test_support::finished(
            "the operand",
            (*copies.bodies[0]).clone(),
            Tol::witness(),
        ),
        other => panic!("a sweep's payload is a body, got {}", other.kind_name()),
    }
}

/// The collar (its placement against the shaft) and the shaft: the
/// shaft's vertex at azimuth `a` from
/// the collar's seam (`+x`), spanning `y ∈ [y0, y0 + h]`.
fn mate(a: f64, y0: f64, h: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("a_mate_on_a_full_turn_bore", Tol::witness());
    let to = |x: f64, y: f64| ProgramTarget::Point(len2([x, y]));
    let (doc, section) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At(len2([0.3, 0.0])),
                ProgramStep::LineTo(to(0.6, 0.0)),
                ProgramStep::LineTo(to(0.6, 1.0)),
                ProgramStep::LineTo(to(0.3, 1.0)),
                ProgramStep::LineTo(ProgramTarget::Start),
            ])],
            ids: Vec::new(),
        }),
    );
    let (doc, collar) = insert(
        doc,
        revolve_about(section, (0.0, 0.0), crate::fixture::along_y(), ang(TAU)),
    );
    // x along azimuth `a` about `y`, and y chosen so the normal is `+y`.
    let (c, s) = (a.cos(), a.sin());
    let (doc, disc) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius: len(0.3),
            }],
            ids: Vec::new(),
        }),
    );
    let (doc, shaft) = insert(
        doc,
        Node::Extrude {
            profile: disc.into(),
            distance: len(h),
        },
    );
    // The shaft is the root: its own frame stands at `(0, y0, 0)` in
    // the collar's coordinates, `x` along `(c, 0, −s)` and `y` along
    // `(−s, 0, −c)`, so the collar is placed at that frame's inverse.
    let (doc, collar) = against(
        doc,
        shaft,
        editor_core::Operand::output(collar, 0),
        &frame_at([0.0, 0.0, -y0], [c, -s, 0.0], [0.0, 0.0, 1.0]),
    );
    (doc, collar, shaft)
}

const SPANS: [(&str, f64, f64); 4] = [
    ("through", -0.5, 2.0),
    ("flush", 0.0, 1.0),
    ("proud above", 0.0, 1.5),
    ("proud below", -0.5, 1.5),
];

/// **Every pose names, and the record lists the shaft's vertices at the
/// rims**: on the seam at every span, off it wherever a shaft cap meets
/// a rim (an off-seam shaft through the collar meets no rim vertex and
/// lists none). Each listed vertex is the shaft's, its partner read
/// nothing for want of a bisector, and holds no class.
#[test]
fn a_mate_on_a_full_turn_bore_names() {
    let tol = Tol::witness();
    let mut listed = 0;
    for a in [0.0, 0.5, FRAC_PI_2, 2.0] {
        for (span, y0, h) in SPANS {
            let (doc, collar, shaft) = mate(a, y0, h);
            let ev = eval(&doc);
            let (c, s) = (operand(&ev, collar), operand(&ev, shaft));
            for (order, x, y, members) in [
                ("collar ∪ shaft", &c, &s, [collar, shaft]),
                ("shaft ∪ collar", &s, &c, [shaft, collar]),
            ] {
                let tag = format!("azimuth {a}, {span}, {order}");
                let bb = match topo::union(x, y, tol) {
                    Ok(topo::BooleanResult::Body(bb)) => bb,
                    other => panic!("{tag}: the mate does not union: {:?}", other.err()),
                };
                let unread = &bb.naming.unread_partners;
                let shaft_side = if order.starts_with("shaft") {
                    topo::Operand::A
                } else {
                    topo::Operand::B
                };
                for u in unread {
                    assert_eq!(
                        (u.operand, u.neighbourhood),
                        (shaft_side, topo::UnreadNeighbourhood::NoBisector),
                        "{tag}: {u:?}"
                    );
                    assert!(
                        !bb.naming
                            .edge_classes
                            .iter()
                            .any(|r| (r.operand, r.vertex) == (u.operand, u.vertex)),
                        "{tag}: a listed vertex holds a class: {u:?}"
                    );
                }
                let rim = a == 0.0 || span != "through";
                assert_eq!(!unread.is_empty(), rim, "{tag}: {unread:?}");
                listed += usize::from(rim);
                let reads = members.map(|n| doc.output(n, 0).expect("a body port").into());
                let (doc, union) = insert(
                    doc.clone(),
                    Node::Union {
                        members: editor_core::Bodies::Spelled(reads.to_vec()),
                    },
                );
                assert!(
                    matches!(eval(&doc).nodes.get(&union), Some(NodeResult::Ok(_))),
                    "{tag}: the union names"
                );
            }
        }
    }
    assert_eq!(
        listed, 26,
        "every on-seam pose and every off-seam cap at a rim"
    );
}
