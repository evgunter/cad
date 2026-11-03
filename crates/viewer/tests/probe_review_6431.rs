//! Review probes for PR 6431 (not for merge).
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr,
    clippy::unwrap_used
)]

use crate::common;
use common::{body_volume, len, near, session_insert};
use pncad::document::{
    Coord, Doc, DocEdit, Operand, ProfileProgram, RecipeNodeId, SplitSide, TargetAxis, VarRef,
};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use viewer::combine::SplitTool;
use viewer::session::{DocSession, PatternDirection, PatternRuleSpec, SessionOp, SplitPlane};

const A: [f64; 3] = [0.04, 0.02, 0.01];

fn session(tol: Tol) -> DocSession {
    common::inline_without_door(Doc::empty_derived("probe-6431", tol), tol)
}

fn sides(session: &mut DocSession, split: RecipeNodeId, tol: Tol) -> Result<(f64, f64), String> {
    session.pump();
    let eval = session.evaluation().expect("landed");
    let Some(value) = eval.value(split) else {
        return Err(format!("{:?}", eval.result(split)));
    };
    let ValuePayload::Split { above, below } = &value.payload else {
        return Err("not a split".into());
    };
    let v = |s: &SplitSide<f64>| match s {
        SplitSide::Body(b) => {
            pncad::topo::test_support::pinned_volume(
                "mass",
                &pncad::topo::AtRestBody::validate(pncad::topo::Body::clone(b.body()), tol)
                    .unwrap(),
                tol,
            )
            .volume
        }
        SplitSide::Empty => 0.0,
    };
    Ok((v(above), v(below)))
}

/// Claim 1a: a non-planar face as the split plane.
#[test]
fn p1_a_cylinder_wall_as_split_plane() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![common::circle_at([0.0, 0.0], 0.01)],
        },
    );
    let rod = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: len(0.02),
        },
    );
    session.pump();
    let index = common::plate_index(&session);
    let wall = common::displayed_face_at(&session, &index, &common::along_x(1.0, 0.004, 0.01));
    let mut tool = SplitTool::new();
    tool.pick_plane(session.committed_doc(), wall);
    let op = tool.op();
    eprintln!("P1 op: {op:?}");
    let outcome = session.perform(op.expect("op"));
    eprintln!(
        "P1 refusal: {:?}",
        outcome.refusal.as_ref().map(ToString::to_string)
    );
    if outcome.refusal.is_none() {
        let doc = session.committed_doc().clone();
        let split = *doc
            .ids()
            .iter()
            .find(|&&id| matches!(doc.node(id), Some(pncad::document::Node::Split { .. })))
            .unwrap();
        eprintln!(
            "P1 rod {rod:?} split {split:?} -> {:?}",
            sides(&mut session, split, tol)
        );
    }
}

/// Claim 1b: target and plane on bodies in different spaces, by a
/// tree pick then a face pick.
#[test]
fn p2_target_and_plane_in_two_spaces() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let a = common::xy_box_in(&mut session, A);
    let b = common::xy_box_in(&mut session, A);
    session.pump();
    let index = common::plate_index(&session);
    let doc = session.committed_doc().clone();
    eprintln!(
        "P2 spaces a={:?} b={:?}",
        doc.operand_space(&a.into()),
        doc.operand_space(&b.into())
    );
    let _ = index;
    let face = viewer::session::FaceSelection {
        name: pncad::prelude::StableName {
            kind: pncad::prelude::EntityKind::Face,
            node: b,
            path: vec![pncad::select::RoleSeg::Cap(pncad::select::CapEnd::End)],
        },
        node: b,
        body: 0,
    };
    eprintln!("P2 face picked on node {:?} (a={a:?}, b={b:?})", face.node);
    let mut tool = SplitTool::new();
    tool.pick(&doc, a);
    tool.pick_plane(&doc, face.clone());
    eprintln!(
        "P2 target {:?} plane {:?}",
        tool.target(),
        tool.face().map(|f| f.node)
    );
    let mut tool2 = SplitTool::new();
    tool2.pick_plane(&doc, face);
    tool2.pick(&doc, if tool.target() == Some(a) { b } else { a });
    for t in [tool, tool2] {
        let outcome = session.perform(t.op().expect("op"));
        eprintln!(
            "P2 target {:?}: refusal {:?}",
            t.target(),
            outcome.refusal.as_ref().map(ToString::to_string)
        );
    }
}

/// Claim 1c: a frame variable (a space) picked as the plane.
#[test]
fn p3_a_space_frame_as_the_plane() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let a = common::xy_box_in(&mut session, A);
    session.pump();
    let doc = session.committed_doc().clone();
    let space = doc.operand_space(&a.into()).expect("a space");
    for var in [space, pncad::document::VarId::WORLD] {
        let kind = doc.var(var).map(pncad::document::Var::kind);
        let mut tool = SplitTool::new();
        tool.pick(&doc, a);
        tool.pick_defined(&doc, var);
        eprintln!("P3 var {var:?} kind {kind:?} held {:?}", tool.plane());
        let Ok(op) = tool.op() else {
            eprintln!("P3 no op: {:?}", tool.op());
            continue;
        };
        let outcome = session.perform(op);
        eprintln!(
            "P3 refusal {:?}",
            outcome.refusal.as_ref().map(ToString::to_string)
        );
        if outcome.refusal.is_none() {
            let doc = session.committed_doc().clone();
            let split = *doc
                .ids()
                .iter()
                .rev()
                .find(|&&id| matches!(doc.node(id), Some(pncad::document::Node::Split { .. })))
                .unwrap();
            eprintln!("P3 sides {:?}", sides(&mut session, split, tol));
            session.perform(SessionOp::Undo);
        }
    }
}

/// `work/doors/a-split-half-cannot-be-split.md`: does a half split now?
#[test]
fn p4_a_split_half_splits() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    session.pump();
    let plane = common::cut_across(&session, body, TargetAxis::Z, A[2] / 2.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    session.pump();
    let half = Operand::output(split, pncad::select::SplitHalf::Below.port());
    let cap = Operand::select(
        Operand::from(body),
        vec![pncad::prelude::StableName {
            kind: pncad::prelude::EntityKind::Face,
            node: body,
            path: vec![pncad::select::RoleSeg::Cap(pncad::select::CapEnd::Start)],
        }],
    );
    let outcome = session.perform(SessionOp::AddSplit {
        target: half,
        plane: common::plane_inside(cap, A[2] / 4.0),
    });
    eprintln!(
        "P4 refusal {:?}",
        outcome.refusal.as_ref().map(ToString::to_string)
    );
    if outcome.refusal.is_none() {
        let doc = session.committed_doc().clone();
        let second = *doc
            .ids()
            .iter()
            .rev()
            .find(|&&id| {
                id != split && matches!(doc.node(id), Some(pncad::document::Node::Split { .. }))
            })
            .unwrap();
        eprintln!(
            "P4 sides {:?} (want {} each)",
            sides(&mut session, second, tol),
            A[0] * A[1] * A[2] / 4.0
        );
    }
}

/// Claim 6: cut_across is the old plane: box at h, above = A0 A1 (A2-h).
#[test]
fn p5_cut_across_is_the_old_plane() {
    let tol = Tol::witness();
    let base = A[0] * A[1];
    for h in [0.004, A[2] / 4.0, A[2] / 2.0, 0.0071] {
        let mut session = session(tol);
        let body = common::xy_box_in(&mut session, A);
        session.pump();
        let plane = common::cut_across(&session, body, TargetAxis::Z, h);
        let split = session_insert(
            &mut session,
            SessionOp::AddSplit {
                target: body.into(),
                plane,
            },
        );
        let (up, down) = sides(&mut session, split, tol).unwrap();
        eprintln!(
            "P5 h={h}: up {up} want {} down {down} want {}",
            base * (A[2] - h),
            base * h
        );
        assert!(near(up, base * (A[2] - h)) && near(down, base * h));
    }
    // A union target, as takeovers' `a_split_of_the_body_the_product_reads_re_points_the_slot`.
    let mut session = session(tol);
    let (_, a) = common::box_in(&mut session, A);
    let doc = session.committed_doc().clone();
    let space = doc.operand_space(&a.into()).unwrap();
    eprintln!("P5 union space {space:?}");
    // A moved copy target: old numbers were in the space; the new plane rides the copy.
    let moved = common::moved_to(&mut session, a, [0.0, 0.0, 0.003]);
    session.pump();
    let plane = common::cut_across(&session, moved, TargetAxis::Z, A[2] / 2.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: moved.into(),
            plane,
        },
    );
    eprintln!(
        "P5 moved copy halves {:?} (old space-z=5mm would give up {} down {})",
        sides(&mut session, split, tol),
        base * 0.005,
        base * 0.005
    );
}

/// Claim 3: defaults clear for a non-symmetric off-origin target; ring closes.
#[test]
fn p6_defaults_clear_and_ring() {
    let tol = Tol::witness();
    let l: Vec<(f64, f64)> = [
        (0.0, 0.0),
        (0.04, 0.0),
        (0.04, 0.01),
        (0.01, 0.01),
        (0.01, 0.02),
        (0.0, 0.02),
    ]
    .iter()
    .map(|&(x, y)| (x + 0.07, y + 0.03))
    .collect();
    let mut session = session(tol);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![pncad::document::LoopProgram::polygon(l).unwrap()],
        },
    );
    let block = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: len(0.01),
        },
    );
    session.pump();
    let one = body_volume(&mut session, block, tol);
    let spacing = session.clearing_step(&block.into()).expect("measured");
    eprintln!("P6 L volume {one} spacing {spacing}");
    let row = session_insert(
        &mut session,
        SessionOp::AddFusedPattern {
            input: block.into(),
            count: viewer::combine::PATTERN_COUNT,
            rule: PatternRuleSpec::Linear {
                direction: PatternDirection::Axis(viewer::combine::ROW_ALONG),
                spacing: len(spacing),
            },
        },
    );
    let fused = body_volume(&mut session, row, tol);
    eprintln!("P6 fused row {fused} vs 4x {}", 4.0 * one);
    let ring = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: block.into(),
            count: viewer::combine::PATTERN_COUNT,
            rule: PatternRuleSpec::Circular {
                axis: PatternDirection::Axis(viewer::combine::RING_ABOUT),
                step: common::ang(viewer::combine::RING_STEP),
            },
        },
    );
    session.pump();
    eprintln!(
        "P6 ring node {ring:?} result {:?}",
        session
            .evaluation()
            .unwrap()
            .result(ring)
            .map(|r| matches!(r, pncad::document::NodeResult::Ok(_)))
    );
}

/// Claim 3/4: the row's text with the index named and anonymous;
/// direction change by zeroing slide-x and setting slide-y.
#[test]
fn p7_index_text_and_direction_change() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    session.pump();
    let spacing = session.clearing_step(&body.into()).unwrap();
    session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: viewer::combine::PATTERN_COUNT,
            rule: PatternRuleSpec::Linear {
                direction: PatternDirection::Axis(TargetAxis::X),
                spacing: len(spacing),
            },
        },
    );
    session.pump();
    let doc = session.committed_doc().clone();
    let row = *doc
        .ids()
        .iter()
        .find(|&&id| pncad::pattern::recognise(&doc, id).is_some())
        .unwrap();
    let pncad::pattern::Recognised::Linear { index, .. } =
        pncad::pattern::recognise(&doc, row).unwrap()
    else {
        panic!()
    };
    let texts = |session: &DocSession| {
        let (doc, ev) = session.landed_pair().unwrap();
        let editor = viewer::matetool::MateTool::edit(doc, row).unwrap();
        let p = editor.proposal(doc, ev, session.tol()).unwrap();
        p.values
            .iter()
            .map(|r| (r.coord.clone(), viewer::matetool::value_text(doc, &r.by)))
            .collect::<Vec<_>>()
    };
    let named = texts(&session);
    eprintln!("P7 named rows {named:?}");
    for (coord, text) in &named {
        eprintln!(
            "P7 named retype {text:?}: {:?}",
            viewer::matetool::typed_value(&doc, coord.clone(), text).map(|_| ())
        );
    }
    // Direction change: slide x -> 0, slide y -> scalar(k) * 7 mm.
    {
        let (doc, ev) = session.landed_pair().unwrap();
        let name = doc.var_name(index).unwrap().as_str().to_owned();
        let mut editor = viewer::matetool::MateTool::edit(doc, row).unwrap();
        editor.type_value(
            Coord::slide(TargetAxis::X),
            viewer::matetool::typed_value(doc, Coord::slide(TargetAxis::X), "0").unwrap(),
        );
        editor.type_value(
            Coord::slide(TargetAxis::Y),
            viewer::matetool::typed_value(
                doc,
                Coord::slide(TargetAxis::Y),
                &format!("scalar({name}) * 7 mm"),
            )
            .unwrap(),
        );
        let p = editor.proposal(doc, ev, session.tol());
        eprintln!(
            "P7 direction proposal ok? {:?}",
            p.as_ref().map(|_| ()).map_err(ToString::to_string)
        );
        if let Ok(p) = p {
            let out = session.perform(p.op());
            eprintln!(
                "P7 direction refusal {:?}",
                out.refusal.as_ref().map(ToString::to_string)
            );
            session.pump();
            let doc = session.committed_doc();
            match pncad::pattern::recognise(doc, row) {
                Some(pncad::pattern::Recognised::Linear { steps, .. }) => eprintln!(
                    "P7 after: {:?}",
                    steps
                        .iter()
                        .map(|(d, s)| (d.clone(), doc.unparse(s)))
                        .collect::<Vec<_>>()
                ),
                other => eprintln!("P7 after: {other:?}"),
            }
            eprintln!("P7 rows after {:?}", texts(&session));
        }
    }
    // Anonymous index.
    let doc = session.committed_doc().clone();
    let unnamed = common::try_edited(
        &doc,
        DocEdit::RenameVar {
            var: VarRef::from(index),
            name: None,
        },
        tol,
    );
    match unnamed {
        Err(e) => eprintln!("P7 unname refused: {e}"),
        Ok((doc, _)) => {
            let mut s2 = DocSession::inline(doc.clone(), tol);
            s2.pump();
            let rows = texts(&s2);
            eprintln!("P7 anonymous rows {rows:?}");
            for (coord, text) in &rows {
                eprintln!(
                    "P7 anon retype {text:?}: {:?}",
                    viewer::matetool::typed_value(&doc, coord.clone(), text)
                        .map(|_| ())
                        .map_err(|e| e.to_string())
                );
            }
        }
    }
}

#[allow(dead_code)]
fn _unused(_: &Doc<ProfileProgram>, _: SplitPlane) {}
