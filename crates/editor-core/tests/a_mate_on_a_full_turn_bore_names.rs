//! **A shaft mated in a full-turn bore names cleanly, the boolean
//! classing the shaft's edges where it meets a rim**
//! (`topo::EdgePieceClass`).
//!
//! The collar is the rectangle ρ ∈ [0.3, 0.6], y ∈ [0, 1] revolved a full
//! turn about `y`; the shaft a circle of the bore's radius on a `+y`
//! frame, its vertex turned `a` from the collar's seam, extruded over
//! four spans. Where a shaft vertex meets the collar at a rim's vertex,
//! the boolean reads the shaft's edges through one cone of the bore and
//! the end face there, and classes each: a shaft ruling split at the rim
//! runs along the bore into the collar (`On`) and out past the end face
//! (`Out`); every other edge there runs along the bore or the rim
//! (`On`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    CancelToken, EvalOptions, Evaluation, LoopProgram, Node, NodeResult, ProfileDoc,
    ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, ValuePayload, evaluate,
};
use geom_core::Tol;
use std::f64::consts::{FRAC_PI_2, TAU};

use editor_core::{Coord, Formula, TargetAxis};

use crate::fixture::{ang, insert, len, len2, revolve_about};

/// A copy of `body`'s revolve in `space`, placed by `placed`.
fn place(
    doc: ProfileDoc,
    body: RecipeNodeId,
    space: editor_core::VarId,
    placed: editor_core::Placed,
) -> (ProfileDoc, RecipeNodeId) {
    let applied = doc
        .apply(
            &editor_core::DocEdit::place(
                editor_core::BodyRead::plain(editor_core::Operand::output(body, 0)),
                editor_core::Operand::Var(space),
                placed.bundle(),
            ),
            Tol::witness(),
        )
        .expect("the collar is placed");
    let placement = applied.record.minted.expect("a placement mints its node");
    (applied.doc, placement)
}

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The evaluated body of node `n` where it stands in its space,
/// finished as an operand.
fn operand(ev: &Evaluation<f64>, n: RecipeNodeId) -> topo::AtRestBody<f64> {
    match &ev.value(n).expect("the sweep evaluates").payload {
        ValuePayload::Body(b) => sweep::test_support::finished(
            "the operand",
            crate::fixture::in_space(b),
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
            on: None,
        }),
    );
    let (doc, collar) = insert(
        doc,
        revolve_about(section, (0.0, 0.0), crate::fixture::along_y(), ang(TAU)),
    );
    let (doc, disc) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius: len(0.3),
            }],
            ids: Vec::new(),
            on: None,
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
    // the collar's coordinates, `x` along `(cos a, 0, −sin a)` and `y` along
    // `(−sin a, 0, −cos a)`, so the collar is placed at that frame's inverse: a
    // quarter turn about `x`, then `−a` about `z`, slid `−y0` along `z`.
    let placed = editor_core::Placed::unmated(vec![
        (
            Coord::Spin(Some(TargetAxis::X)),
            Formula::turn_over(4).expect("a quarter turn"),
        ),
        (Coord::Spin(Some(TargetAxis::Z)), ang(-a)),
        (Coord::Slide(TargetAxis::Z), len(-y0)),
    ]);
    let space = doc
        .operand_space(&shaft.into())
        .expect("the shaft is in a space");
    let (doc, collar) = place(doc, collar, space, placed);
    (doc, collar, shaft)
}

const SPANS: [(&str, f64, f64); 4] = [
    ("through", -0.5, 2.0),
    ("flush", 0.0, 1.0),
    ("proud above", 0.0, 1.5),
    ("proud below", -0.5, 1.5),
];

/// **Every pose names, and the shaft's vertices at the rims are
/// classed**: at every azimuth and span the shaft meets the collar at
/// vertices paired at a rim (its seam ruling crossing a rim, a cap on
/// one). No partner is listed unread, and each of the shaft's vertices
/// paired at a rim reads as the module docs derive by hand: a ruling (a
/// line edge) in two pieces, split at the rim, one piece `On` and the
/// other `Out`; every other row (the cap's circle along the rim, split or
/// not, and a ruling ending at the rim) `On`.
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
                let naming = &bb.naming;
                assert_eq!(naming.unread_partners, [], "{tag}: every partner reads");
                let shaft_side = if order.starts_with("shaft") {
                    topo::Operand::A
                } else {
                    topo::Operand::B
                };
                let rims: std::collections::BTreeSet<_> = naming
                    .reduction_contacts
                    .vv
                    .iter()
                    .map(|c| match shaft_side {
                        topo::Operand::A => c.a,
                        topo::Operand::B => c.b,
                    })
                    .collect();
                for &v in &rims {
                    let mut edges = std::collections::BTreeMap::new();
                    for r in naming
                        .edge_classes
                        .iter()
                        .filter(|r| (r.operand, r.vertex) == (shaft_side, v))
                    {
                        edges.entry(r.edge).or_insert_with(Vec::new).push(r.class);
                    }
                    for pieces in edges.values_mut() {
                        pieces.sort();
                    }
                    let shaft: &topo::Body<f64> = if order.starts_with("shaft") { x } else { y };
                    // Per edge: a ruling (a line edge) split at the rim reads one
                    // piece `On` and the other `Out`; every other edge `On`.
                    let want: std::collections::BTreeMap<_, Vec<_>> = edges
                        .iter()
                        .map(|(&e, pieces)| {
                            let ruling = topo::query::edge_carrier_kind(shaft, e)
                                == Some(geom::CurveKind::Line);
                            let mut want = match pieces.len() {
                                2 if ruling => vec![topo::SideCode::Out, topo::SideCode::On],
                                n => vec![topo::SideCode::On; n],
                            };
                            want.sort();
                            (e, want)
                        })
                        .collect();
                    assert!(
                        !edges.is_empty() && edges == want,
                        "{tag}: the shaft's vertex {v:?} at a rim reads {edges:?}"
                    );
                }
                assert!(!rims.is_empty(), "{tag}: the shaft meets a rim");
                listed += 1;
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
    assert_eq!(listed, 32, "every pose, in both orders");
}
