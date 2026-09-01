//! **Fillet and chamfer authoring, replayed headlessly** (GAUTH-5):
//! the blend tool accumulating edge picks and committing exactly one
//! `Node::Fillet` or `Node::Chamfer` through the real `DocSession`,
//! with no renderer anywhere.
//!
//! # The box every row is about
//!
//! [`boxed`] authors a 10 mm cube through the creation vocabulary
//! alone (a rectangle profile, one extrude) — twelve edges, all
//! straight, meeting three at a corner. Its whole-body blend is the
//! shape a minimal instance has to take: the kernel's assembly admits
//! only a fully-requested chain set, so a fillet of ONE box edge would
//! terminate at a trivalent corner whose other two edges were never
//! requested and refuse by name. That is not a limitation this unit
//! works around — it is the freeze semantics' own consequence, and it
//! is why the all-edges door exists.
//!
//! # Where the edge names come from
//!
//! The pick index, through the door the viewport pick answers with
//! (`PickIndex::edges_in` + `edge_name_of`). Turning a cursor into one
//! of those edges is `edge_pick`'s subject and is not re-tested here;
//! what these rows drive is everything after: the set the tool
//! accumulates, the node it commits, and what evaluation and the tree
//! say about it.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use common::insert;
use pncad::document::{
    Dimension, Doc, Expr, Node, NodeErrorKind, NodeResult, ProfileProgram, RecipeNodeId, SlotId,
};
use pncad::geom_core::Tol;
use pncad::prelude::{StableName, ValuePayload};
use pncad::profile::SketchPlane;
use viewer::blend::FREEZE_NOTE;
use viewer::blend::{BlendError, BlendEvent, BlendKindChoice, BlendTarget, BlendTool};
use viewer::display::DisplayView;
use viewer::pick::{PickIndex, PickKinds};
use viewer::scene::DisplayTolerance;
use viewer::session::{
    DatumSpec, DocSession, EdgeSelection, FaceSelection, NodeKindWanted, ProfileShape, Refusal,
    Selection, SessionOp,
};
use viewer::tools::{ToolKind, ToolNotice, Tools};
use viewer::tree::{self, RowStatus};

/// The cube every row blends: 10 mm on a side.
const SIDE: f64 = 0.01;
/// The blend size every row authors, well inside half the side.
const BLEND: f64 = 0.001;
/// A box has twelve edges, and a row that accumulated eleven would
/// otherwise pass while pinning nothing.
const BOX_EDGES: usize = 12;

/// A session over a throwaway document.
fn session(tol: Tol) -> DocSession {
    DocSession::inline(Doc::empty_derived("blend-start", tol), tol)
}

/// A cube of `side`, authored through the creation doors.
fn boxed(session: &mut DocSession, side: f64) -> RecipeNodeId {
    let profile = insert(
        session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Rectangle {
                width: side,
                height: side,
            }],
        },
    );
    insert(
        session,
        SessionOp::AddExtrude {
            profile,
            distance: side,
        },
    )
}

/// The display tolerance the pick index is built at — coarse, since no
/// row here measures a facet.
fn delta() -> DisplayTolerance {
    DisplayTolerance::new(2.0e-4).expect("a positive delta")
}

/// The pick index for a session's landed evaluation — the door a
/// viewport pick answers through.
fn index_of(session: &DocSession) -> PickIndex {
    let (doc, eval) = session
        .landed_pair()
        .expect("the inline seam lands its first evaluation");
    let generation = session
        .landed_generation()
        .expect("a landed evaluation has a generation");
    PickIndex::build(doc, eval, generation, delta(), session.tol()).expect("the box indexes")
}

/// Every drawn edge of a node's body 0, as the pick selections a
/// viewport click would produce.
fn drawn_edges(session: &DocSession, node: RecipeNodeId) -> Vec<EdgeSelection> {
    let index = index_of(session);
    index
        .edges_in(node, 0)
        .iter()
        .map(|&id| EdgeSelection {
            name: index
                .edge_name_of(id)
                .expect("a drawn edge has a name")
                .clone(),
            node,
            body: 0,
        })
        .collect()
}

/// Every edge name of a node, through the shipped all-edges door.
fn all_edge_names(session: &DocSession, node: RecipeNodeId) -> Vec<StableName> {
    let eval = session.evaluation().expect("the inline seam landed");
    pncad::select::all_edges(eval, node)
}

/// Drive the all-edges affordance the way the panel does: the landed
/// evaluation for the names, the pick index for the (node, body)
/// narrowing.
fn load_all(tools: &mut Tools, session: &DocSession, target: BlendTarget) -> Option<BlendEvent> {
    let index = index_of(session);
    let eval = session.evaluation().expect("the inline seam landed");
    tools
        .blend_mut()
        .expect("the blend tool is open")
        .load_all_edges(target, eval, &index)
}

/// The whole body of a node, as the blend tool's target.
fn whole(node: RecipeNodeId) -> BlendTarget {
    BlendTarget { node, body: 0 }
}

/// Feed one edge pick to the open tool the way the application does —
/// through the selection op stream, which is what makes single-select
/// and tool accumulation one mechanism rather than two.
fn pick(tools: &mut Tools, doc: &Doc<ProfileProgram>, edge: &EdgeSelection) -> Vec<ToolNotice> {
    tools.feed(doc, &[SessionOp::Select(Selection::Edge(edge.clone()))])
}

/// A node's single body's volume, with the seam pumped.
fn body_volume(session: &mut DocSession, node: RecipeNodeId, tol: Tol) -> f64 {
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let ValuePayload::Body(body) = &eval
        .value(node)
        .unwrap_or_else(|| {
            panic!(
                "the node evaluated: {:?}",
                eval.result(node).and_then(NodeResult::error)
            )
        })
        .payload
    else {
        panic!("a blend evaluates to a body");
    };
    pncad::topo::mass_properties(body, tol)
        .expect("mass properties")
        .volume
}

/// The blend tool with every edge of `node` held, accumulated by
/// PICKING them one at a time in a scrambled order — the affordance
/// the plan specifies, not the all-edges shortcut.
fn picked_all(session: &DocSession, node: RecipeNodeId) -> Tools {
    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    let mut edges = drawn_edges(session, node);
    assert_eq!(edges.len(), BOX_EDGES, "a box draws twelve edges");
    // Reversed, so a row that only passed for the index's own order
    // would fail: the stored set is canonical whatever order the
    // clicks arrived in.
    edges.reverse();
    for edge in &edges {
        assert!(
            pick(&mut tools, session.committed_doc(), edge).is_empty(),
            "every pick lands"
        );
    }
    tools
}

/// The tool the tools value holds open, for a row that reads it.
fn blend(tools: &Tools) -> &BlendTool {
    tools.blend().expect("the blend tool is open")
}

/// Perform the op a tool committed, closing the tool exactly as the
/// application does — when the edit LANDS, not when the button was
/// clicked.
fn commit(session: &mut DocSession, tools: &mut Tools, op: SessionOp) -> RecipeNodeId {
    assert!(
        tools.commits_open_tool(&op),
        "the op is the open tool's one committed edit"
    );
    let node = insert(session, op);
    tools.close();
    assert_eq!(tools.open_kind(), None, "a landed edit closes its tool");
    node
}

/// **The acceptance row**: a box authored from nothing, its twelve
/// edges accumulated one click at a time, one fillet committed, and
/// the inserted node's selection canonical.
#[test]
fn a_box_fillet_authors_from_picks_with_a_canonical_selection() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();
    let mut tools = picked_all(&session, target);
    assert_eq!(blend(&tools).count(), BOX_EDGES, "twelve edges held");
    assert_eq!(
        blend(&tools).target(),
        Some(BlendTarget {
            node: target,
            body: 0
        })
    );

    let op = blend(&tools)
        .fillet_op(BLEND)
        .expect("a tool holding edges commits");
    let fillet = commit(&mut session, &mut tools, op);

    let Some(Node::Fillet {
        target: stored_target,
        radius,
        selection,
    }) = session.committed_doc().node(fillet)
    else {
        panic!("the door minted a fillet");
    };
    assert_eq!(*stored_target, target);
    assert_eq!(
        *radius,
        Expr::literal(BLEND, Dimension::Length).expect("finite"),
        "the radius is a literal Length slot"
    );
    // CANONICAL: sorted, deduplicated, and equal as a set to what the
    // all-edges door answers — the same twelve names either way.
    let mut canonical = selection.clone();
    canonical.sort();
    canonical.dedup();
    assert_eq!(
        canonical, *selection,
        "the stored selection is sorted and deduplicated"
    );
    assert_eq!(*selection, all_edge_names(&session, target));

    // And it is a solid: a filleted cube has less volume than the cube.
    let filleted = body_volume(&mut session, fillet, tol);
    assert!(
        filleted < SIDE.powi(3) && filleted > 0.9 * SIDE.powi(3),
        "a 1 mm fillet takes a little off a 10 mm cube: {filleted}"
    );
}

/// **The chamfer twin**: the same picks, the other door, the other
/// node — and a different solid, because the size means a setback
/// rather than a radius.
#[test]
fn the_chamfer_twin_authors_the_other_node_from_the_same_picks() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();
    let mut tools = picked_all(&session, target);

    let op = blend(&tools)
        .chamfer_op(BLEND)
        .expect("a tool holding edges commits");
    let chamfer = commit(&mut session, &mut tools, op);

    let Some(Node::Chamfer {
        target: stored_target,
        distance,
        selection,
    }) = session.committed_doc().node(chamfer)
    else {
        panic!("the door minted a chamfer");
    };
    assert_eq!(*stored_target, target);
    assert_eq!(
        *distance,
        Expr::literal(BLEND, Dimension::Length).expect("finite")
    );
    assert_eq!(*selection, all_edge_names(&session, target));
    // The size lands in the chamfer's OWN slot, which is what makes a
    // reader able to tell what the number means off the node kind.
    assert_eq!(
        session
            .committed_doc()
            .node(chamfer)
            .expect("the node is there")
            .slots(),
        vec![SlotId::ChamferDistance]
    );

    // A flat chamfer of setback d removes more than a fillet of radius
    // d does: the fillet keeps the quarter-disc the chamfer cuts flat
    // across. Asserted as an inequality between two authored solids
    // rather than against a recorded number.
    let chamfered = body_volume(&mut session, chamfer, tol);
    let mut twin = session_with_fillet(tol);
    let filleted = body_volume(&mut twin.0, twin.1, tol);
    assert!(
        chamfered < filleted,
        "chamfer {chamfered} vs fillet {filleted}"
    );
}

/// A second session holding the fillet the chamfer row compares
/// against — same box, same set, same size.
fn session_with_fillet(tol: Tol) -> (DocSession, RecipeNodeId) {
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();
    let mut tools = picked_all(&session, target);
    let op = blend(&tools).fillet_op(BLEND).expect("commits");
    let fillet = commit(&mut session, &mut tools, op);
    (session, fillet)
}

/// **The all-edges affordance**: the shipped door's answer, loaded
/// into tool state as an ORDINARY set — the same twelve names twelve
/// clicks would have accumulated, and nothing about the node it
/// commits says where they came from.
#[test]
fn the_all_edges_door_loads_the_set_twelve_clicks_would_have() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();

    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    let loaded = load_all(&mut tools, &session, whole(target));
    assert!(loaded.is_none(), "the box has edges to load: {loaded:?}");
    assert_eq!(blend(&tools).count(), BOX_EDGES);

    // Identical to the picked set, tool state and all — which is the
    // whole claim: "all edges" is materialized once and stored, never
    // a live query the node would have to know about.
    let picked = picked_all(&session, target);
    assert_eq!(blend(&tools), blend(&picked));

    let op = blend(&tools).fillet_op(BLEND).expect("commits");
    let fillet = commit(&mut session, &mut tools, op);
    assert_eq!(
        session.committed_doc().node(fillet),
        session_with_fillet(tol).0.committed_doc().node(fillet),
        "the node is the one the clicks would have authored"
    );
}

/// **A body with no edges loads nothing and says so**, rather than
/// emptying the held set on the way to a refusal.
#[test]
fn the_all_edges_door_refuses_a_target_with_no_edges() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    let datum = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Point {
                position: [0.0, 0.0, 0.0],
            },
        },
    );
    session.pump();

    let mut tools = picked_all(&session, target);
    let refused = load_all(&mut tools, &session, whole(datum));
    assert_eq!(
        refused,
        Some(BlendEvent::NoEdgesOnTarget {
            target: BlendTarget {
                node: datum,
                body: 0
            }
        })
    );
    assert_eq!(
        blend(&tools).count(),
        BOX_EDGES,
        "a refused load costs no held edge"
    );
}

/// **One target, and a mis-aimed click costs nothing.** A pick on a
/// second body is refused typed; the eleven good picks stay.
#[test]
fn a_pick_on_another_body_is_refused_and_keeps_the_held_edges() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let first = boxed(&mut session, SIDE);
    let second = boxed(&mut session, SIDE * 0.5);
    session.pump();

    let mut tools = picked_all(&session, first);
    let stray = drawn_edges(&session, second)
        .into_iter()
        .next()
        .expect("the second box draws edges");
    let notices = pick(&mut tools, session.committed_doc(), &stray);
    assert_eq!(notices.len(), 1, "the declined pick is reported once");
    let ToolNotice::Blend(BlendEvent::OtherTarget { held, picked }) = &notices[0] else {
        panic!("expected a cross-target refusal, got {notices:?}");
    };
    assert_eq!(held.node, first);
    assert_eq!(picked.node, second);
    assert!(
        notices[0].to_string().starts_with("blend tool: "),
        "the sentence names its tool: {}",
        notices[0]
    );
    assert_eq!(blend(&tools).count(), BOX_EDGES, "no held edge was lost");
    assert!(!blend(&tools).holds(&stray.name), "and none was gained");

    // The committed node is still the first box's, unchanged by the
    // stray click.
    let op = blend(&tools).fillet_op(BLEND).expect("commits");
    let fillet = commit(&mut session, &mut tools, op);
    assert!(matches!(
        session.committed_doc().node(fillet),
        Some(Node::Fillet { target, .. }) if *target == first
    ));
}

/// **Per-pick add and remove**: picking a held edge again takes it
/// out, and the live count follows.
#[test]
fn picking_a_held_edge_again_removes_it() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();
    let edges = drawn_edges(&session, target);

    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    for edge in &edges[..3] {
        assert!(pick(&mut tools, session.committed_doc(), edge).is_empty());
    }
    assert_eq!(blend(&tools).count(), 3);
    assert!(pick(&mut tools, session.committed_doc(), &edges[1]).is_empty());
    assert_eq!(blend(&tools).count(), 2, "the second pick came back out");
    assert!(!blend(&tools).holds(&edges[1].name));
    assert!(pick(&mut tools, session.committed_doc(), &edges[1]).is_empty());
    assert_eq!(blend(&tools).count(), 3, "and goes back in");
    assert!(blend(&tools).holds(&edges[1].name));
}

/// **The survival step is all-or-nothing.** Losing the target body
/// voids the whole set at once and says how many went; nothing here
/// shrinks a set quietly.
#[test]
fn losing_the_target_voids_the_whole_set_and_says_so() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();
    let mut tools = picked_all(&session, target);

    assert!(
        tools
            .reconcile(session.doc(), session.landed_pair())
            .is_empty(),
        "a live target drops nothing"
    );

    assert!(
        session
            .perform(SessionOp::DeleteNode { node: target })
            .refusal
            .is_none()
    );
    session.pump();
    let notices = tools.reconcile(session.doc(), session.landed_pair());
    assert_eq!(notices.len(), 1, "one notice for the whole set");
    let ToolNotice::Blend(BlendEvent::TargetLost {
        target: lost,
        edges,
    }) = &notices[0]
    else {
        panic!("expected a lost target, got {notices:?}");
    };
    assert_eq!(lost.node, target);
    assert_eq!(*edges, BOX_EDGES);
    assert_eq!(blend(&tools).count(), 0);
    assert_eq!(blend(&tools).target(), None);
    assert_eq!(
        blend(&tools)
            .fillet_op(BLEND)
            .expect_err("a tool holding no edges refuses"),
        BlendError::NoEdges,
        "and the commit door refuses rather than authoring an empty blend"
    );
}

/// **A stranded selection refuses on the badge and does NOT shrink**
/// — the ratified #217 semantics, seen from the authoring side.
///
/// The strand is minted the way a stale pick becomes one: a name from
/// a body that is not the target. The TOOL cannot author this — its
/// cross-target rule is exactly what stops it — so the row drives the
/// session door, which is the API the tool is one caller of.
#[test]
fn a_stranded_selection_refuses_typed_rather_than_shrinking() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    let spare = boxed(&mut session, SIDE * 0.5);
    session.pump();

    let mut selection = all_edge_names(&session, target);
    let stray = all_edge_names(&session, spare)
        .into_iter()
        .next()
        .expect("the spare box has edges");
    selection.push(stray.clone());
    let wanted = selection.len();

    let fillet = insert(
        &mut session,
        SessionOp::AddFillet {
            target,
            radius: BLEND,
            selection,
        },
    );
    session.pump();

    // The set is stored whole: thirteen names, one of which cannot
    // resolve. Nothing dropped it on the way in.
    let Some(Node::Fillet { selection, .. }) = session.committed_doc().node(fillet) else {
        panic!("a fillet was authored");
    };
    assert_eq!(selection.len(), wanted, "the stranded name is still stored");
    assert!(selection.contains(&stray));

    // And evaluation refuses it, typed, on the node's own badge.
    let eval = session.evaluation().expect("the inline seam landed");
    let error = eval
        .result(fillet)
        .and_then(NodeResult::error)
        .expect("the fillet refuses");
    assert!(
        matches!(error.kind, NodeErrorKind::BlendSelectionResolve { .. }),
        "expected a selection-resolve refusal, got {:?}",
        error.kind
    );
    let rows = tree::rows(session.committed_doc(), Some(eval));
    let row = rows
        .iter()
        .find(|row| row.id == fillet)
        .expect("the fillet has a tree row");
    let RowStatus::Failed { message } = &row.status else {
        panic!("the authored blend badges FAILED, got {:?}", row.status);
    };
    assert_eq!(
        *message,
        error.to_string(),
        "the badge is the typed error's own rendering"
    );
}

/// **The kernel's own blend refusal reaches the tree badge** from the
/// authored path: a radius the geometry cannot take fails the node,
/// and the row shows the kernel's words unaltered.
#[test]
fn a_blend_the_kernel_refuses_badges_on_the_authored_node() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();
    let mut tools = picked_all(&session, target);

    // A radius the size of the whole cube: the rolling ball does not
    // fit, and the op refuses rather than passing the sharp box
    // through.
    let op = blend(&tools).fillet_op(SIDE).expect("commits");
    let fillet = commit(&mut session, &mut tools, op);
    session.pump();

    let eval = session.evaluation().expect("the inline seam landed");
    let error = eval
        .result(fillet)
        .and_then(NodeResult::error)
        .expect("an over-large fillet refuses");
    assert!(
        matches!(error.kind, NodeErrorKind::Blend { .. }),
        "the kernel's own refusal, carried unaltered: {:?}",
        error.kind
    );
    let rows = tree::rows(session.committed_doc(), Some(eval));
    let row = rows
        .iter()
        .find(|row| row.id == fillet)
        .expect("the fillet has a tree row");
    assert!(
        matches!(&row.status, RowStatus::Failed { message } if *message == error.to_string()),
        "the badge renders the typed refusal: {:?}",
        row.status
    );
}

/// **An authored blend saves and reloads**, bit for bit — which is
/// also the canonical-form claim's other half: `persist`'s strict door
/// treats a non-canonical selection on the wire as a corrupt file, so
/// a reload that succeeds is a set that was canonical.
#[test]
fn an_authored_blend_saves_and_reloads() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();
    let mut tools = picked_all(&session, target);
    let op = blend(&tools).fillet_op(BLEND).expect("commits");
    let fillet = commit(&mut session, &mut tools, op);
    let volume = body_volume(&mut session, fillet, tol);

    let dir = common::tempdir("gauth5-blend");
    let path = dir.join("blended.pncad");
    assert!(
        session
            .perform(SessionOp::Save(path.clone()))
            .refusal
            .is_none(),
        "save"
    );
    let authored = session.committed_doc().clone();
    assert!(
        session.perform(SessionOp::Open(path)).refusal.is_none(),
        "open"
    );
    assert!(
        session.committed_doc().bit_eq(&authored),
        "the reloaded document is the authored one, bit for bit"
    );
    let reloaded = body_volume(&mut session, fillet, tol);
    assert_eq!(
        volume.to_bits(),
        reloaded.to_bits(),
        "same solid after reload"
    );
}

/// **The door's seat is a BODY**, and a target that is not one refuses
/// typed at the door rather than after the edit lands.
#[test]
fn the_blend_door_refuses_a_target_that_is_not_a_body() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();
    let selection = all_edge_names(&session, target);
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Rectangle {
                width: SIDE,
                height: SIDE,
            }],
        },
    );

    for op in [
        SessionOp::AddFillet {
            target: profile,
            radius: BLEND,
            selection: selection.clone(),
        },
        SessionOp::AddChamfer {
            target: profile,
            distance: BLEND,
            selection: selection.clone(),
        },
    ] {
        let outcome = session.perform(op);
        assert!(outcome.committed.is_empty(), "nothing was authored");
        assert!(
            matches!(
                outcome.refusal,
                Some(Refusal::WrongNodeKind {
                    node,
                    wanted: NodeKindWanted::Body
                }) if node == profile
            ),
            "expected a body-seat refusal, got {:?}",
            outcome.refusal
        );
    }
}

/// **An empty set refuses at the TOOL door**, before an op exists —
/// and the document-level rule is untouched: an empty selection that
/// reaches a node refuses at evaluation on its badge, so a
/// hand-written recipe gets the same answer.
#[test]
fn an_empty_set_refuses_at_the_tool_and_at_evaluation() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();

    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    assert_eq!(
        blend(&tools)
            .fillet_op(BLEND)
            .expect_err("a tool holding no edges refuses"),
        BlendError::NoEdges
    );
    assert_eq!(
        blend(&tools)
            .chamfer_op(BLEND)
            .expect_err("a tool holding no edges refuses"),
        BlendError::NoEdges
    );
    assert_eq!(
        blend(&tools)
            .fillet_op(BLEND)
            .expect_err("a tool holding no edges refuses")
            .to_string(),
        "no edges picked yet"
    );

    // The op door itself admits one — a recipe is allowed to be
    // unfinished — and evaluation is where it refuses.
    let fillet = insert(
        &mut session,
        SessionOp::AddFillet {
            target,
            radius: BLEND,
            selection: Vec::new(),
        },
    );
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    assert!(
        matches!(
            eval.result(fillet)
                .and_then(NodeResult::error)
                .map(|e| &e.kind),
            Some(NodeErrorKind::BlendSelectionEmpty { .. })
        ),
        "an empty selection refuses at evaluation"
    );
}

/// **The blend tool is one of the modal tools**: it narrows the cursor
/// to edges, it closes whatever was open, and whatever it opens over
/// is closed.
#[test]
fn the_blend_tool_takes_its_place_among_the_modal_tools() {
    assert_eq!(ToolKind::Blend.pick_kinds(), PickKinds::EdgesOnly);
    assert!(ToolKind::ALL.contains(&ToolKind::Blend));

    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    assert_eq!(tools.pick_kinds(), PickKinds::EdgesOnly);
    for kind in ToolKind::ALL {
        tools.open(kind);
        assert_eq!(tools.open_kind(), Some(kind), "one tool open, and it is it");
        assert!(
            ToolKind::ALL
                .into_iter()
                .filter(|&other| other != kind)
                .all(|other| tools.open_kind() != Some(other))
        );
    }
    tools.close();
    assert_eq!(tools.open_kind(), None);
    assert_eq!(tools.pick_kinds(), PickKinds::Any, "the bare cursor's rule");
}

/// **A closed tool takes no picks**, and neither does an open tool of
/// another kind — the one-open rule is what makes the selection stream
/// unambiguous.
#[test]
fn only_the_open_blend_tool_accumulates_edges() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();
    let edge = drawn_edges(&session, target)
        .into_iter()
        .next()
        .expect("the box draws edges");

    let mut tools = Tools::new();
    assert!(pick(&mut tools, session.committed_doc(), &edge).is_empty());
    assert!(tools.blend().is_none(), "nothing open takes nothing");

    tools.open(ToolKind::Blend);
    assert!(pick(&mut tools, session.committed_doc(), &edge).is_empty());
    assert_eq!(blend(&tools).count(), 1);
    // Opening another tool replaces the whole value, picks and all.
    tools.open(ToolKind::Boolean);
    assert!(pick(&mut tools, session.committed_doc(), &edge).is_empty());
    tools.open(ToolKind::Blend);
    assert_eq!(blend(&tools).count(), 0, "a re-opened tool starts over");
}

/// The two kinds' labels and their one Length field's meaning — the
/// chrome's radio row read as a value, so the sentence a user sees is
/// checked without a window.
#[test]
fn the_kind_choice_names_what_the_one_field_means() {
    assert_eq!(
        BlendKindChoice::ALL.map(|(kind, label)| (kind, label, kind.size_label())),
        [
            (BlendKindChoice::Fillet, "fillet", "radius"),
            (BlendKindChoice::Chamfer, "chamfer", "setback"),
        ]
    );
    assert_eq!(BlendKindChoice::default(), BlendKindChoice::Fillet);
}

// --- the fix pass -----------------------------------------------------

/// **The all-edges door narrows to the DRAWN body it was asked about.**
///
/// `all_edges` reads one node's name table, and a split's table covers
/// both halves: unnarrowed, the panel counted every edge of both sides
/// while the picture marked one, which is this tool's own one-target
/// rule broken from the inside. The count the panel shows and the set
/// the picture marks are asserted to be the same number here, per body
/// and on a node the commit door would refuse — because the lie was
/// visible long before any commit.
#[test]
fn the_all_edges_door_narrows_to_the_body_it_was_asked_about() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    let plane = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: [0.0, 0.0, SIDE / 2.0],
                normal: [0.0, 0.0, 1.0],
            },
        },
    );
    let split = insert(
        &mut session,
        SessionOp::AddSplit {
            target,
            tool: plane,
        },
    );
    session.pump();

    // The node-wide door sees both halves at once; each drawn half has
    // fewer edges than that.
    let node_wide = all_edge_names(&session, split).len();
    let index = index_of(&session);
    let mut tools = Tools::new();
    for body in [0u32, 1] {
        let drawn = index.edges_in(split, body).len();
        assert!(drawn > 0, "the split draws body {body}");
        assert!(
            drawn < node_wide,
            "body {body} draws {drawn} of the node's {node_wide} edge names"
        );
        tools.open(ToolKind::Blend);
        let loaded = load_all(&mut tools, &session, BlendTarget { node: split, body });
        assert!(loaded.is_none(), "{loaded:?}");
        assert_eq!(
            blend(&tools).count(),
            drawn,
            "the count the panel shows is the set the picture marks"
        );
        assert_eq!(
            blend(&tools).marks().len(),
            drawn,
            "and the marks are that same set"
        );
    }
}

/// **The held set marks exactly the edges it names** — the value claim
/// (`marks`) and the per-frame path (`mark_segments`) agree, so the
/// fast one cannot drift from the one the rest of the crate reads.
#[test]
fn a_held_set_marks_exactly_the_edges_it_names() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();
    let index = index_of(&session);
    let display = DisplayView::none();

    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    assert!(
        blend(&tools).marks().is_empty()
            && blend(&tools).mark_segments(&index, &display).is_empty(),
        "a tool holding nothing marks nothing"
    );

    let edges = drawn_edges(&session, target);
    for edge in &edges[..5] {
        assert!(pick(&mut tools, session.committed_doc(), edge).is_empty());
    }
    let tool = blend(&tools);
    let named = tool.marks();
    assert_eq!(named.len(), 5, "five held, five named");
    // The per-name door and the one-pass door produce the same
    // segments, in the index's own edge order either way.
    let mut per_name: Vec<[f32; 3]> = Vec::new();
    for id in index.edges_in(target, 0) {
        if let Ok(name) = index.edge_name_of(*id)
            && named.iter().any(|mark| mark.name == *name)
        {
            per_name.extend(viewer::pick::edge_id_segments(&index, &display, *id));
        }
    }
    assert!(!per_name.is_empty(), "five box edges draw segments");
    assert_eq!(tool.mark_segments(&index, &display), per_name);
}

/// **An upstream edit that strands a held edge drops it, loudly.**
///
/// The demonstrated degrade direction: a union of two boxes, every
/// edge of the result held, then one operand MOVED so the two no
/// longer meet the same way. The union node survives — so the
/// deleted-target arm says nothing — but six of the held names are no
/// longer edges of it. Before the fix the panel went on counting
/// thirty and the commit authored a node that refused on arrival.
#[test]
fn an_upstream_edit_that_strands_held_edges_drops_them_and_says_so() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let a = boxed(&mut session, SIDE);
    let raw_b = boxed(&mut session, SIDE);
    let b = insert(
        &mut session,
        SessionOp::AddTransform {
            input: raw_b,
            translation: [SIDE * 0.5, SIDE * 0.25, SIDE * 0.25],
            rotation_axis: [0.0, 0.0, 1.0],
            rotation_angle: 0.0,
        },
    );
    let union = insert(
        &mut session,
        SessionOp::AddBoolean {
            op: pncad::document::BooleanOp::Union,
            a,
            b,
        },
    );
    session.pump();

    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    assert!(load_all(&mut tools, &session, whole(union)).is_none());
    let held = blend(&tools).count();
    assert!(
        held > BOX_EDGES,
        "an overlapping union has more edges than a box: {held}"
    );
    assert!(
        tools
            .reconcile(session.doc(), session.landed_pair())
            .is_empty(),
        "an untouched document drops nothing"
    );

    // Move the operand clear of the other box: the union still
    // evaluates — nothing is deleted, so the target-lost arm has
    // nothing to say — but the six edges the two solids met along are
    // gone, and six of the held names with them.
    assert!(
        session
            .perform(SessionOp::SetSlot {
                node: b,
                slot: SlotId::Translation(pncad::document::Axis3::X),
                value: viewer::props::SlotValue::of(Dimension::Length, SIDE * 2.0),
            })
            .refusal
            .is_none()
    );
    session.pump();
    assert!(
        session.committed_doc().node(union).is_some(),
        "the target survived the edit"
    );
    let live = all_edge_names(&session, union);
    assert_eq!(
        live.len(),
        BOX_EDGES * 2,
        "two disjoint boxes: no intersection edges left"
    );
    assert!(
        blend(&tools).selection().iter().any(|n| !live.contains(n)),
        "the move stranded held names"
    );

    let notices = tools.reconcile(session.doc(), session.landed_pair());
    assert_eq!(notices.len(), 1, "one notice for the drop: {notices:?}");
    let ToolNotice::Blend(BlendEvent::EdgesLost {
        target,
        names,
        kept,
    }) = &notices[0]
    else {
        panic!("expected a strand drop, got {notices:?}");
    };
    assert_eq!(target.node, union);
    assert!(!names.is_empty());
    assert_eq!(*kept, blend(&tools).count());
    assert_eq!(
        *kept + names.len(),
        held,
        "every held edge is kept or named"
    );
    // And what is still held is exactly what the target still has —
    // so the commit authors a node with no strand in it.
    assert!(
        blend(&tools).selection().iter().all(|n| live.contains(n)),
        "no strand survives the drop"
    );
    assert!(
        notices[0].to_string().starts_with("blend tool: "),
        "{}",
        notices[0]
    );
}

/// **Nothing landed is not "it is gone"**, and a target that failed
/// outright costs no picks: the strand test needs an evaluation with
/// an answer, and a run that has none says nothing rather than
/// emptying the set.
#[test]
fn the_strand_check_is_not_asked_without_an_answer() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();
    let mut tools = picked_all(&session, target);

    // No landed pair at all.
    assert!(
        tools.reconcile(session.doc(), None).is_empty(),
        "we cannot tell is not it is gone"
    );
    assert_eq!(blend(&tools).count(), BOX_EDGES);

    // A target that FAILS: the extrude's distance goes to zero, the
    // node has no value, and `all_edges` answers empty for a reason
    // that is not "the body lost every edge".
    assert!(
        session
            .perform(SessionOp::SetSlot {
                node: target,
                slot: SlotId::Distance,
                value: viewer::props::SlotValue::of(Dimension::Length, 0.0),
            })
            .refusal
            .is_none()
    );
    session.pump();
    assert!(
        all_edge_names(&session, target).is_empty(),
        "the failed target names no edges"
    );
    assert!(
        tools
            .reconcile(session.doc(), session.landed_pair())
            .is_empty(),
        "a failed run costs no picks"
    );
    assert_eq!(blend(&tools).count(), BOX_EDGES, "the set is intact");
}

/// **Un-picking the last edge releases the target**, so the next click
/// may start on any body and the commit door's one sentence is true of
/// the one state.
#[test]
fn an_emptied_set_releases_its_target() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let first = boxed(&mut session, SIDE);
    let second = boxed(&mut session, SIDE * 0.5);
    session.pump();
    let one = drawn_edges(&session, first)
        .into_iter()
        .next()
        .expect("edges");
    let other = drawn_edges(&session, second)
        .into_iter()
        .next()
        .expect("edges");

    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    assert!(pick(&mut tools, session.committed_doc(), &one).is_empty());
    assert_eq!(blend(&tools).target(), Some(whole(first)));
    // Un-pick it: the tool is holding nothing, so it is latched to
    // nothing.
    assert!(pick(&mut tools, session.committed_doc(), &one).is_empty());
    assert_eq!(blend(&tools).count(), 0);
    assert_eq!(blend(&tools).target(), None, "an empty set holds no target");
    assert_eq!(
        blend(&tools)
            .fillet_op(BLEND)
            .expect_err("a tool holding no edges refuses"),
        BlendError::NoEdges
    );
    // And the next click starts wherever the user aims it.
    assert!(pick(&mut tools, session.committed_doc(), &other).is_empty());
    assert_eq!(blend(&tools).target(), Some(whole(second)));
    assert_eq!(blend(&tools).count(), 1);

    // `clear` is the same state by the panel's own door.
    tools.blend_mut().expect("open").clear();
    assert_eq!(blend(&tools).target(), None);
    assert_eq!(blend(&tools).count(), 0);
}

/// **A drawn pick names a body; a tree click does not.** The all-edges
/// affordance's fallback target reads the selection, and answering
/// `body: 0` for a feature would be a guess that reads wrong on
/// exactly the multi-body nodes the narrowing is for.
#[test]
fn only_a_drawn_selection_names_a_body_for_the_all_edges_door() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let target = boxed(&mut session, SIDE);
    session.pump();
    let index = index_of(&session);
    let edge = drawn_edges(&session, target)
        .into_iter()
        .next()
        .expect("edges");
    let face = index
        .ids_in(target, 0)
        .first()
        .and_then(|&id| index.name_of(id))
        .and_then(|named| named.as_ref().ok())
        .map(|name| FaceSelection {
            name: name.clone(),
            node: target,
            body: 0,
        })
        .expect("the box draws named patches");

    assert_eq!(
        BlendTarget::of_selection(&Selection::Edge(edge.clone())),
        Some(whole(target))
    );
    assert_eq!(
        BlendTarget::of_selection(&Selection::Face(face)),
        Some(whole(target))
    );
    assert_eq!(
        BlendTarget::of_selection(&Selection::Node(target)),
        None,
        "a feature picked in the tree does not say which body"
    );
    assert_eq!(BlendTarget::of_selection(&Selection::None), None);
}

/// **The freeze sentence the panel shows is true of the node it is
/// about.** It says the set does not grow on an upstream edit, that a
/// removed edge refuses rather than shrinking the stored set, and that
/// a rebind is the one thing that rewrites one — `Node::Fillet`'s
/// three clauses, no wider.
#[test]
fn the_freeze_note_states_the_ratified_semantics_and_no_more() {
    // Not a spelling check: each clause is a claim `Node::Fillet`'s
    // docs make, and the fourth clause the note used to imply — that
    // nothing shrinks a stored selection — is one they do NOT, because
    // a rebind onto an already-selected name shrinks it by one.
    assert!(FREEZE_NOTE.contains("upstream edit that adds an edge does not extend"));
    assert!(FREEZE_NOTE.contains("refuses on the node rather than shrinking it"));
    assert!(
        FREEZE_NOTE.contains("rebind"),
        "the one door that rewrites a stored selection is named: {FREEZE_NOTE}"
    );
    assert!(
        !FREEZE_NOTE.contains("never"),
        "no unqualified never-shrinks claim: {FREEZE_NOTE}"
    );
}
