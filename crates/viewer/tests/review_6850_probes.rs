//! Review lane `placing-review-6850`'s probes of PR 6850 (the target
//! row): each row asserts what the ruling on `[ev]` PR 6600 asks, so a
//! red row is a finding.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use pncad::document::{AuthoredNode, Doc, Operand, ProfileProgram, RecipeNodeId, VarId};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
use viewer::matetool::MateTool;
use viewer::session::{DocSession, PlaceTarget, Refusal, SessionOp};

const BOLT: [f64; 3] = [0.005, 0.005, 0.02];
const PLATE: [f64; 3] = [0.04, 0.04, 0.005];

fn cap(body: RecipeNodeId, end: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: body,
        path: vec![RoleSeg::Cap(end)],
    }
}

fn placed(session: &mut DocSession, body: RecipeNodeId) -> RecipeNodeId {
    let op = SessionOp::place_where_it_stands(session.committed_doc(), body.into())
        .expect("a body in a space");
    let out = session.perform(op);
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    out.made.expect("a copy")
}

fn rest_on(bolt: RecipeNodeId, to: Operand) -> pncad::document::Mate<pncad::document::Formula> {
    crate::fixture::rest(
        Operand::select(Operand::Node(bolt), vec![cap(bolt, CapEnd::Start)]),
        to,
    )
}

/// The bolt, copies 1 and 2 in its space, copy 2 resting on copy 1; the
/// plate in a space of its own.
fn bench(
    tag: &str,
    tol: Tol,
) -> (
    DocSession,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
) {
    let mut session = common::inline_without_door(Doc::empty_derived(tag, tol), tol);
    let bolt = common::xy_box_in(&mut session, BOLT);
    let plate = common::xy_box_in(&mut session, PLATE);
    session.pump();
    let c1 = placed(&mut session, bolt);
    let c2 = placed(&mut session, bolt);
    common::commit_mate(
        &mut session,
        SessionOp::AddMate {
            placement: c2,
            mate: Box::new(rest_on(
                bolt,
                Operand::select(Operand::Node(c1), vec![cap(bolt, CapEnd::End)]),
            )),
            values: Vec::new(),
        },
    );
    session.pump();
    (session, bolt, plate, c1, c2)
}

#[allow(clippy::result_large_err)]
fn offered(session: &DocSession, tool: &mut MateTool) -> Option<SessionOp> {
    let (doc, ev) = session.landed_pair().expect("landed");
    tool.offer(doc, ev, session.tol(), |op| session.bundle_door(op))
}

fn held(doc: &Doc<ProfileProgram>, p: RecipeNodeId) -> Option<AuthoredNode> {
    doc.node(p).map(|n| n.authored(doc))
}

fn x_value() -> (pncad::document::Coord<Operand>, pncad::document::Formula) {
    common::slides([0.003, 0.0, 0.0]).remove(0)
}

/// **Into the space the loaded target names, after a held re-target
/// with a typed value**: copy 2's target set to the world (held: its
/// mate reads the bolt's space) and x typed. Its row offers "Into <bolt's
/// space>"; clicking it must not lose the typed value.
#[test]
fn probe_into_the_loaded_space_keeps_the_typed_value() {
    let tol = Tol::witness();
    let (session, _, _, _, c2) = bench("rv6850-into-back", tol);
    let session = session;
    let old = session.committed_doc().node_space(c2).expect("placed");
    let mut tool = MateTool::edit(session.committed_doc(), c2).expect("a placement");
    tool.set_target(PlaceTarget::space(VarId::WORLD));
    assert_eq!(offered(&session, &mut tool), None);
    let (x, by) = x_value();
    tool.type_value(x, by);
    assert_eq!(offered(&session, &mut tool), None, "still held");
    let (doc, ev) = session.landed_pair().expect("landed");
    assert_eq!(tool.off_target(doc, ev), [(0, old)], "Into <old> offered");
    tool.retarget_into(old);
    let first = offered(&session, &mut tool);
    let second = offered(&session, &mut tool);
    let typed = tool.typed().len();
    if first.is_none() && second.is_none() && typed == 0 {
        // Where it went: a later passing change brings it back as a
        // step of its own after that change lands.
        let mut session = session;
        let y = common::slides([0.0, 0.004, 0.0]).remove(1);
        tool.type_value(y.0, y.1);
        let later = offered(&session, &mut tool).expect("y passes");
        assert!(session.perform(later).refusal.is_none());
        session.pump();
        tool.landed(session.committed_doc());
        let typed_now = tool.typed().to_vec();
        let next = offered(&session, &mut tool);
        eprintln!(
            "PROBE into-back: after a later change landed, typed={} next_offer={:?}",
            typed_now.len(),
            next.map(|op| match op {
                SessionOp::SetBundle { constraints, .. } => constraints.len(),
                _ => 0,
            })
        );
        let session = session;
        let _ = &session;
    }
    assert!(
        first.is_some() || second.is_some() || typed == 1,
        "the typed x is neither offered nor kept in the draft: typed={typed}, \
         first={first:?}, second={second:?}, pending={:?}",
        tool.pending()
    );
}

/// **A refused Into puts back a mix of picks and values; the whole lands
/// in one step once it passes; undo and redo are one step each**: copy 2
/// with its old rest struck, a new rest written onto the plate (another
/// space) and x typed; Into the plate's space: the target alone fails
/// (the loaded rest reads the bolt's space), so the draft is put back and
/// offered whole over the plate's space.
#[test]
fn probe_a_refused_into_puts_back_the_mix_and_lands_whole() {
    let tol = Tol::witness();
    let (mut session, bolt, plate, _, c2) = bench("rv6850-mix", tol);
    let plate_space = session
        .committed_doc()
        .node_space(plate)
        .expect("in a space");
    let before = session.committed_doc().clone();
    let states = session.history().len();
    let mut tool = MateTool::edit(session.committed_doc(), c2).expect("a placement");
    tool.strike(0, true);
    tool.add_written(
        session.committed_doc(),
        &rest_on(
            bolt,
            Operand::select(Operand::Node(plate), vec![cap(plate, CapEnd::End)]),
        ),
    );
    let (x, by) = x_value();
    tool.type_value(x, by);
    assert_eq!(
        offered(&session, &mut tool),
        None,
        "the cross-space to is held"
    );
    let rows = tool.rows().to_vec();
    tool.retarget_into(plate_space);
    let first = offered(&session, &mut tool);
    let op = first
        .or_else(|| offered(&session, &mut tool))
        .unwrap_or_else(|| {
            panic!(
                "the whole draft passes over the plate's space: {:?}",
                tool.pending()
            )
        });
    assert_eq!(tool.rows(), &rows[..], "the rows are put back");
    let SessionOp::SetBundle {
        target,
        constraints,
        ..
    } = &op
    else {
        panic!("{op:?}")
    };
    assert_eq!(*target, PlaceTarget::space(plate_space));
    assert_eq!(constraints.iter().filter(|c| c.mate().is_some()).count(), 1);
    let out = session.perform(op);
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    assert_eq!(session.history().len(), states + 1, "one step");
    let after = session.committed_doc().clone();
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(session.committed_doc().bit_eq(&before), "one undo");
    assert!(session.perform(SessionOp::Redo).refusal.is_none());
    assert!(session.committed_doc().bit_eq(&after), "one redo");
}

/// **Undo between the two steps of Into then the held mate**: Into lands
/// (step 1), then Undo before the held flush is offered. What does the
/// editor hold? Records it.
#[test]
fn probe_undo_between_into_and_the_held_mate() {
    let tol = Tol::witness();
    let (mut session, bolt, plate, _, c1) = {
        let (s, b, p, c1, _) = bench("rv6850-undo-mid", tol);
        (s, b, p, (), c1)
    };
    let plate_space = session
        .committed_doc()
        .node_space(plate)
        .expect("in a space");
    let before = session.committed_doc().clone();
    let mut tools = viewer::tools::Tools::new();
    tools.open(viewer::tools::ToolKind::Mate);
    let mut tool = MateTool::edit(session.committed_doc(), c1).expect("a placement");
    tool.add_written(
        session.committed_doc(),
        &rest_on(
            bolt,
            Operand::select(Operand::Node(plate), vec![cap(plate, CapEnd::End)]),
        ),
    );
    assert_eq!(offered(&session, &mut tool), None, "held");
    *tools.mate_mut().expect("open") = tool;
    tools.mate_mut().expect("open").retarget_into(plate_space);
    let into = offered(&session, tools.mate_mut().expect("open")).expect("the target alone");
    let out = session.perform(into.clone());
    session.pump();
    tools.answered(
        &into,
        out.made,
        out.refusal.is_none(),
        session.committed_doc(),
    );
    assert_eq!(session.committed_doc().node_space(c1), Some(plate_space));
    let rows_after_into = tools.mate().expect("open").rows().len();
    assert_eq!(rows_after_into, 1, "the held mate is made again over it");
    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none());
    session.pump();
    assert!(session.committed_doc().bit_eq(&before));
    let committed = session.committed_doc().clone();
    let events = tools
        .mate_mut()
        .expect("open")
        .reconcile(&committed, session.landed_pair().expect("landed"));
    eprintln!("PROBE undo-mid events: {events:?}");
    let editor = tools.mate().expect("open");
    eprintln!(
        "PROBE undo-mid: rows={} target={:?} pending={:?}",
        editor.rows().len(),
        editor.target(),
        editor.pending()
    );
}

/// **Add to the product on a mated copy in its body's space** (decision
/// 1): records which branch it takes.
#[test]
fn probe_add_to_product_on_a_mated_copy() {
    let tol = Tol::witness();
    let (mut session, _, _, _, c2) = bench("rv6850-product", tol);
    let before = session.committed_doc().clone();
    let placements = before.placements().len();
    let out = session.perform(SessionOp::AddToProduct {
        node: c2,
        body: None,
    });
    let doc = session.committed_doc();
    eprintln!(
        "PROBE product-mated: refusal={:?} c2_space_world={} placements {}->{} c2_same={}",
        out.refusal,
        doc.node_space(c2) == Some(VarId::WORLD),
        placements,
        doc.placements().len(),
        held(doc, c2) == held(&before, c2)
    );
}

/// **A product slot reading a mated copy outside the world**: the
/// `ProductOutsideWorld` recourse and Add to the product on it.
#[test]
fn probe_product_outside_world_on_a_mated_copy() {
    let tol = Tol::witness();
    let (mut session, _, _, _, c2) = bench("rv6850-outside", tol);
    let copy = session.committed_doc().output(c2, 0).expect("a copy");
    let set = session.perform(SessionOp::SetProduct {
        bodies: vec![Operand::Var(copy)],
    });
    assert!(set.refusal.is_none(), "{:?}", set.refusal);
    session.pump();
    let out = session.perform(SessionOp::AddToProduct {
        node: c2,
        body: None,
    });
    eprintln!("PROBE outside-world-mated: refusal={:?}", out.refusal);
}

/// **A combine with an instance-like copy and a whole family**: is a
/// `Place` in another space with no mate refused (it is a copy)?
#[test]
fn probe_combine_refuses_an_unmated_copy_in_another_space() {
    let tol = Tol::witness();
    let (mut session, bolt, plate, _, _) = bench("rv6850-combine", tol);
    let c3 = placed(&mut session, bolt);
    let before = session.committed_doc().clone();
    let out = session.perform(SessionOp::AddBoolean {
        spec: viewer::session::BooleanSpec::Union(vec![plate.into(), c3.into()]),
    });
    assert!(
        matches!(out.refusal, Some(Refusal::CopyInAnotherSpace { .. })),
        "{:?}",
        out.refusal
    );
    assert!(session.committed_doc().bit_eq(&before));
}
