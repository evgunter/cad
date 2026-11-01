//! **A click on a drawn circle is the whole circle**: what
//! `PickIndex::whole_edge` answers across the shapes its rim table has
//! to answer for (partial, annular, split and full revolves, the
//! teapot lid), and how the blend tool's grouping of a rim into one
//! pick holds through loading, un-picking and the survival step.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use crate::common;

use common::{ang, len, len3, plate_index, scl3, session_insert};
use pncad::document::{Dimension, Doc, Formula, NodeResult, ProfileProgram, RecipeNodeId, SlotId};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::RoleSeg;
use viewer::blend::{BlendEvent, BlendTarget, BlendTool};
use viewer::pickcache::NotIndexed;
use viewer::pickindex::PickIndex;
use viewer::props::SlotValue;
use viewer::session::{DocSession, EdgeSelection, Selection, SessionOp};
use viewer::tools::{ToolKind, ToolNotice, Tools};

const SIDE: f64 = 0.01;

fn session(tol: Tol) -> DocSession {
    DocSession::inline(Doc::empty_derived("whole-rim-pick", tol), tol)
}

/// A rectangle `[x0, x0 + w] × [0, h]` revolved by `angle` about the
/// profile's y axis.
fn revolved(session: &mut DocSession, x0: f64, w: f64, h: f64, angle: f64) -> RecipeNodeId {
    let profile = session_insert(
        session,
        SessionOp::AddProfile {
            loops: vec![common::rectangle_loop([x0, 0.0], w, h)],
        },
    );
    session_insert(
        session,
        SessionOp::AddRevolve {
            profile,
            axis_through: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: ang(angle),
        },
    )
}

fn drawn(index: &PickIndex, node: RecipeNodeId, body: u32) -> Vec<EdgeSelection> {
    index
        .edges_in(node, body)
        .iter()
        .map(|&id| EdgeSelection {
            name: index.edge_name_of(id).expect("named").clone(),
            node,
            body,
        })
        .collect()
}

fn is_rim_arc(name: &StableName, pi: bool) -> bool {
    match name.path.as_slice() {
        [.., RoleSeg::BandRim(_)] => !pi,
        [.., RoleSeg::BandRimPi(_)] => pi,
        _ => false,
    }
}

fn click(
    tools: &mut Tools,
    doc: &Doc<ProfileProgram>,
    index: Result<&PickIndex, NotIndexed>,
    edge: &EdgeSelection,
) -> Vec<ToolNotice> {
    tools.feed(
        doc,
        &viewer::parts::PartFiles::default(),
        index,
        &[SessionOp::Select(Selection::Edge(edge.clone()))],
    )
}

fn blend(tools: &Tools) -> &BlendTool {
    tools.blend().expect("open")
}

/// The count the panel shows is the number of distinct whole edges the
/// held names make up, read back through the index: the invariant the
/// tool's `rims` field is supposed to keep.
fn picks_by_index(tool: &BlendTool, index: &PickIndex) -> usize {
    let Some(target) = tool.target() else {
        return 0;
    };
    let mut wholes: Vec<Vec<StableName>> = tool
        .selection()
        .into_iter()
        .map(|name| {
            let mut whole = index
                .whole_edge(&EdgeSelection {
                    name,
                    node: target.node,
                    body: target.body,
                })
                .expect("the index answers the rim");
            whole.sort();
            whole
        })
        .collect();
    wholes.sort();
    wholes.dedup();
    wholes.len()
}

/// **A partial revolve's rim is one arc, and a click on it is itself.**
#[test]
fn a_partial_revolves_arcs_each_stand_alone() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let node = revolved(
        &mut session,
        0.0,
        SIDE * 0.5,
        SIDE,
        core::f64::consts::TAU * 0.75,
    );
    session.pump();
    let node = common::drawn_node(session.committed_doc(), node);
    let index = plate_index(&session);
    let edges = drawn(&index, node, 0);
    assert!(!edges.is_empty());
    for edge in &edges {
        assert_eq!(
            index.whole_edge(edge),
            Ok(vec![edge.name.clone()]),
            "{edge:?}"
        );
    }
}

/// **An annular full revolve's rims are one closed edge each**: no
/// seam vertex on them, so nothing to group.
#[test]
fn an_annular_full_revolves_rims_stand_alone() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let node = revolved(&mut session, SIDE, SIDE * 0.5, SIDE, core::f64::consts::TAU);
    session.pump();
    let node = common::drawn_node(session.committed_doc(), node);
    let index = plate_index(&session);
    for edge in &drawn(&index, node, 0) {
        assert_eq!(
            index.whole_edge(edge),
            Ok(vec![edge.name.clone()]),
            "{edge:?}"
        );
    }
}

/// **All edges of an axis cylinder count each cap circle once**, and
/// hold every arc.
#[test]
fn loading_every_edge_counts_each_drawn_circle_once() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let node = revolved(&mut session, 0.0, SIDE * 0.5, SIDE, core::f64::consts::TAU);
    session.pump();
    let node = common::drawn_node(session.committed_doc(), node);
    let index = plate_index(&session);
    let edges = drawn(&index, node, 0);
    let rims = edges.iter().filter(|e| is_rim_arc(&e.name, false)).count();
    assert_eq!(rims, 2, "two cap circles: {edges:?}");
    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    let (doc, eval) = session.landed_pair().expect("landed");
    let loaded = tools.blend_mut().expect("open").load_all_edges(
        BlendTarget { node, body: 0 },
        doc,
        eval,
        &index,
    );
    assert_eq!(loaded, None);
    let tool = blend(&tools);
    assert_eq!(tool.selection().len(), edges.len(), "every arc is held");
    assert_eq!(tool.count(), edges.len() - rims, "each circle counts once");
    assert_eq!(tool.count(), picks_by_index(tool, &index));
}

/// **Arc A with no index is refused, then arc B through the index
/// holds the circle**, and un-picking A drops both.
#[test]
fn an_unindexed_arc_pick_is_refused_and_an_indexed_one_holds_the_circle() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let node = revolved(&mut session, 0.0, SIDE * 0.5, SIDE, core::f64::consts::TAU);
    session.pump();
    let node = common::drawn_node(session.committed_doc(), node);
    let index = plate_index(&session);
    let edges = drawn(&index, node, 0);
    let a = edges
        .iter()
        .find(|e| is_rim_arc(&e.name, false))
        .expect("an arc");
    let whole = index.whole_edge(a).expect("the index answers the rim");
    let b = edges
        .iter()
        .find(|e| e.name != a.name && whole.contains(&e.name))
        .expect("its twin");
    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    let doc = session.committed_doc().clone();
    // The two absences say different things: one being built is worth
    // waiting for, and one the cache refused is not retried until the
    // document or the display tolerance moves.
    for (why, says) in [
        (NotIndexed::Building, "still being indexed"),
        (
            NotIndexed::Absent,
            "pick-index badge says why the last build refused",
        ),
    ] {
        match click(&mut tools, &doc, Err(why), a).as_slice() {
            [notice @ ToolNotice::Blend(BlendEvent::Unindexed { picked, why: said })] => {
                assert_eq!(picked, a, "the refused pick is the click");
                assert_eq!(*said, why);
                let line = notice.to_string();
                assert!(line.contains(says), "{why:?}: {line}");
            }
            other => panic!("a pick with no index is refused, not held as a half: {other:?}"),
        }
    }
    assert_eq!(blend(&tools).count(), 0, "the refused pick holds nothing");
    assert_eq!(blend(&tools).target(), None, "nor fixes a target");
    assert!(click(&mut tools, &doc, Ok(&index), b).is_empty());
    assert_eq!(
        (blend(&tools).count(), blend(&tools).selection().len()),
        (1, 2)
    );
    assert_eq!(blend(&tools).count(), picks_by_index(blend(&tools), &index));
    // A straight edge alongside.
    let straight = edges
        .iter()
        .find(|e| !whole.contains(&e.name))
        .expect("a straight edge");
    assert!(click(&mut tools, &doc, Ok(&index), straight).is_empty());
    assert_eq!(
        (blend(&tools).count(), blend(&tools).selection().len()),
        (2, 3)
    );
    assert!(click(&mut tools, &doc, Ok(&index), a).is_empty());
    assert_eq!(
        (blend(&tools).count(), blend(&tools).selection().len()),
        (1, 1),
        "un-picking either arc drops the circle and keeps the straight edge"
    );
}

/// **A held rim through an edit that makes the revolve partial**: the
/// survival step drops what stopped being an edge, and the count and
/// the held names still agree with the index of the new picture.
#[test]
fn a_held_rim_through_an_edit_that_makes_the_revolve_partial() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let feature = revolved(&mut session, 0.0, SIDE * 0.5, SIDE, core::f64::consts::TAU);
    session.pump();
    let node = common::drawn_node(session.committed_doc(), feature);
    let index = plate_index(&session);
    let edges = drawn(&index, node, 0);
    let a = edges
        .iter()
        .find(|e| is_rim_arc(&e.name, false))
        .expect("an arc");
    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    let doc = session.committed_doc().clone();
    assert!(click(&mut tools, &doc, Ok(&index), a).is_empty());
    assert_eq!(
        (blend(&tools).count(), blend(&tools).selection().len()),
        (1, 2)
    );

    let outcome = session.perform(SessionOp::SetSlot {
        node: feature,
        slot: SlotId::RevolveAngle,
        value: SlotValue::Continuous(core::f64::consts::TAU * 0.75),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let notices = tools.reconcile(session.doc(), session.landed_pair());
    let index = plate_index(&session);
    let tool = blend(&tools);
    match notices.as_slice() {
        [ToolNotice::Blend(BlendEvent::EdgesLost { names, kept, .. })] => {
            assert_eq!(names.len(), 1, "the BandRimPi arc went: {names:?}");
            assert_eq!(*kept, 1);
        }
        other => panic!("one strand event: {other:?}"),
    }
    assert_eq!(
        tool.count(),
        picks_by_index(tool, &index),
        "count agrees with the new picture"
    );
    for name in tool.selection() {
        assert!(
            drawn(&index, node, 0).iter().any(|e| e.name == name),
            "a held name is drawn: {name:?}"
        );
    }
}

/// **A held rim, then undo past the revolve**: the whole set goes with
/// one event whose count is the picks, not the names.
#[test]
fn a_held_rim_through_an_undo_that_removes_the_body() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let feature = revolved(&mut session, 0.0, SIDE * 0.5, SIDE, core::f64::consts::TAU);
    session.pump();
    let node = common::drawn_node(session.committed_doc(), feature);
    let index = plate_index(&session);
    let edges = drawn(&index, node, 0);
    let a = edges
        .iter()
        .find(|e| is_rim_arc(&e.name, false))
        .expect("an arc");
    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    let doc = session.committed_doc().clone();
    assert!(click(&mut tools, &doc, Ok(&index), a).is_empty());
    for _ in 0..3 {
        let _ = session.perform(SessionOp::Undo);
        if session.doc().node(node).is_none() {
            break;
        }
    }
    session.pump();
    assert!(session.doc().node(node).is_none(), "undone past the body");
    let notices = tools.reconcile(session.doc(), session.landed_pair());
    let events: Vec<&BlendEvent> = notices
        .iter()
        .filter_map(|n| match n {
            ToolNotice::Blend(event) => Some(event),
            _ => None,
        })
        .collect();
    match events.as_slice() {
        [BlendEvent::TargetLost { edges, .. }] => assert_eq!(*edges, 1, "one pick went"),
        other => panic!("{other:?}"),
    }
    assert_eq!(blend(&tools).count(), 0);
    assert!(blend(&tools).selection().is_empty());
}

/// **A full revolve cut across its axis by a split**: the two halves
/// are bodies 0 and 1 of one node (one table, two windows, through
/// `body_read_of`'s split arm), and the cut adds a circle the seam
/// meridian splits for a reason of its own. Every drawn circle's whole
/// edge is in its own half's window, and every drawn edge on a closed
/// circle answers two arcs or more.
#[test]
fn a_split_revolves_circles_stay_in_their_own_half() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let feature = revolved(&mut session, 0.0, SIDE * 0.5, SIDE, core::f64::consts::TAU);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: pncad::document::Operand::output(feature, 0),
            origin: len3([0.0, SIDE / 2.0, 0.0]),
            normal: scl3([0.0, 1.0, 0.0]),
        },
    );
    let mut doc = session.committed_doc().clone();
    let whole = common::copy_of(&doc, feature);
    common::edit_into(
        &mut doc,
        pncad::document::DocEdit::DeleteNode { id: whole },
        tol,
    );
    let (doc, _) = common::placed_read(&doc, pncad::document::Operand::output(split, 0), tol);
    let (doc, _) = common::placed_read(&doc, pncad::document::Operand::output(split, 1), tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let index = plate_index(&session);
    assert_the_old_walk_agrees(&session, &index);
    let parts: Vec<(RecipeNodeId, u32)> = index
        .parts()
        .iter()
        .map(|part| (part.node(), part.body()))
        .collect();
    assert_eq!(parts.len(), 2, "two halves drawn: {parts:?}");
    let mut circles = 0;
    for &(node, body) in &parts {
        let edges = drawn(&index, node, body);
        for edge in &edges {
            let whole = index.whole_edge(edge).expect("the index answers the rim");
            if whole.len() > 1 {
                circles += 1;
                for arc in &whole {
                    assert!(
                        edges.iter().any(|e| &e.name == arc),
                        "a half's circle is drawn in that half's window: {arc:?}"
                    );
                }
            }
        }
    }
    assert_eq!(
        circles, 8,
        "each half: its cap circle and the cut circle, two arcs each"
    );
}

/// **The teapot lid, one click per rim** (`demos/tour/src/teapot.rs`
/// `lid_meridian`, `LID_RIMS`): the three rims the tour rolls, each
/// clicked once on its `BandRim` arc, commit one fillet that builds.
#[test]
fn the_teapot_lids_three_rims_roll_from_one_click_each() {
    use pncad::document::{LoopProgram, ProgramArcData, ProgramStep, ProgramTarget};
    use pncad::profile::ArcSweep;
    const Y_MOUTH: f64 = 8.0 / 64.0;
    const R_NECK: f64 = 3.0 / 64.0;
    const LID_BASE: f64 = Y_MOUTH + 1.0 / 32.0;
    const R_FLANGE: f64 = 14.0 / 256.0;
    const Y_FLANGE: f64 = LID_BASE + 6.0 / 256.0;
    const DOME_C: f64 = LID_BASE + 1.0 / 256.0;
    const R_KNOB: f64 = 5.0 / 256.0;
    const Y_KNOB: f64 = LID_BASE + 13.0 / 256.0;
    const Y_TOP: f64 = LID_BASE + 18.0 / 256.0;
    const ROLL: f64 = 2.0 / 256.0;
    let p = |x: f64, y: f64| [len(x), len(y)];
    let line = |x: f64, y: f64| ProgramStep::LineTo(ProgramTarget::Point(p(x, y)));
    let meridian = LoopProgram::Chain(vec![
        ProgramStep::At(p(0.0, LID_BASE)),
        line(R_FLANGE, LID_BASE),
        line(R_NECK, Y_FLANGE),
        ProgramStep::ArcTo(ProgramArcData::Center {
            c: p(0.0, DOME_C),
            winding: ArcSweep::Ccw,
            target: ProgramTarget::Point(p(R_KNOB, Y_KNOB)),
        }),
        line(R_KNOB, Y_TOP),
        line(0.0, Y_TOP),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![meridian],
        },
    );
    let lid = session_insert(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            axis_through: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: ang(core::f64::consts::TAU),
        },
    );
    session.pump();
    let node = common::drawn_node(session.committed_doc(), lid);
    let index = plate_index(&session);
    assert_the_old_walk_agrees(&session, &index);
    let mut arcs: Vec<EdgeSelection> = drawn(&index, node, 0)
        .into_iter()
        .filter(|e| is_rim_arc(&e.name, false))
        .collect();
    // Authored order: the rims at meridian vertices 1, 2, 3, 4.
    arcs.sort_by(|a, b| a.name.cmp(&b.name));
    assert_eq!(arcs.len(), 4, "four latitude circles: {arcs:?}");
    let mut tools = Tools::new();
    tools.open(ToolKind::Blend);
    let doc = session.committed_doc().clone();
    for at in [0usize, 1, 3] {
        assert!(click(&mut tools, &doc, Ok(&index), &arcs[at]).is_empty());
    }
    assert_eq!(blend(&tools).count(), 3, "three clicks, three rims");
    assert_eq!(blend(&tools).selection().len(), 6, "six arcs held");
    let op = blend(&tools).fillet_op(len(ROLL)).expect("commits");
    let fillet = session_insert(&mut session, op);
    tools.close();
    session.pump();
    let eval = session.evaluation().expect("landed");
    if let Some(error) = eval.result(fillet).and_then(NodeResult::error) {
        panic!(
            "the lid's three rims roll from three clicks: {:?}",
            error.kind
        );
    }
}

/// **The pre-fix viewer-side rim walk, kept as an oracle** (delta
/// review of PR 6129): the name → key → `rim_of` → names round trip
/// `pickindex.rs` `Rims::push` made before the walk moved into
/// `NodePick::boundary_rims`. Every drawn edge of every part of
/// `index` must answer the same whole edge from both.
fn assert_the_old_walk_agrees(session: &DocSession, index: &PickIndex) {
    use editor_core::{EntityKey, EntityRef, Entry};
    let (_, eval) = session.landed_pair().expect("landed");
    let mut compared = 0;
    for part in index.parts() {
        let (node, body) = (part.node(), part.body());
        let (var, at) = eval.body_read_of(node, body).expect("a read");
        let table = eval.table(var).expect("a table");
        let solid = eval.read_body(var, at).expect("usable").expect("a body");
        for edge in drawn(index, node, body) {
            let Some(Entry::Unique(EntityRef {
                key: EntityKey::Edge(key),
                ..
            })) = table.lookup(&edge.name)
            else {
                panic!("a drawn edge's name is one edge: {edge:?}");
            };
            let old = pncad::topo::query::rim_of(solid.body(), *key)
                .ok()
                .and_then(|rim| {
                    rim.iter()
                        .map(|&arc| {
                            table
                                .name_of(&EntityRef {
                                    body: at,
                                    key: EntityKey::Edge(arc),
                                })
                                .cloned()
                        })
                        .collect::<Option<Vec<_>>>()
                })
                .filter(|arcs| arcs.len() > 1);
            let new = index.whole_edge(&edge).expect("the index answers the rim");
            match old {
                // The old table kept the first polyline's walk order
                // for every arc of a rim; compare as sets.
                Some(mut old) => {
                    let mut new = new;
                    old.sort();
                    new.sort();
                    assert_eq!(new, old, "{edge:?}");
                }
                None => assert_eq!(new, vec![edge.name.clone()], "{edge:?}"),
            }
            compared += 1;
        }
    }
    assert!(compared > 0, "the oracle compared something");
}

/// **The moved walk answers what the viewer-side walk answered**, on
/// partial, annular and axis-touching full revolves and on both halves
/// of a split one (the teapot lid is held the same way in its own row).
#[test]
fn boundary_rims_answers_what_the_viewer_side_walk_answered() {
    let tol = Tol::witness();
    for (x0, angle) in [
        (0.0, core::f64::consts::TAU * 0.75),
        (SIDE, core::f64::consts::TAU),
        (0.0, core::f64::consts::TAU),
    ] {
        let mut session = session(tol);
        revolved(&mut session, x0, SIDE * 0.5, SIDE, angle);
        session.pump();
        assert_the_old_walk_agrees(&session, &plate_index(&session));
    }
}

/// **A `NodePick` held across pictures by hand reaches
/// `RimNameFault::Stale`** (delta review of PR 6129): the admitted
/// later-evaluation case `NodePick`'s pairing doc describes. Built on a
/// partial revolve and asked of the evaluation after the revolve went
/// full, some polyline's key no longer resolves; `boundary_names`
/// answers the same slot as `Unnamed`.
#[test]
fn a_node_pick_held_across_an_edit_reaches_the_stale_arm() {
    use editor_core::resolve::{NodePick, RimNameFault};
    let tol = Tol::witness();
    let mut session = session(tol);
    let feature = revolved(
        &mut session,
        0.0,
        SIDE * 0.5,
        SIDE,
        core::f64::consts::TAU * 0.75,
    );
    session.pump();
    let node = common::drawn_node(session.committed_doc(), feature);
    let pick = {
        let (_, eval) = session.landed_pair().expect("landed");
        NodePick::build(eval, node, 0, common::plate_delta().get(), tol).expect("builds")
    };
    let outcome = session.perform(SessionOp::SetSlot {
        node: feature,
        slot: SlotId::RevolveAngle,
        value: SlotValue::Continuous(core::f64::consts::TAU),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let (_, eval) = session.landed_pair().expect("landed");
    let rims = pick.boundary_rims(eval).expect("the pairing admits it");
    let names = pick.boundary_names(eval).expect("the pairing admits it");
    let stale = rims
        .iter()
        .position(|rim| matches!(rim, Err(RimNameFault::Stale { .. })))
        .unwrap_or_else(|| panic!("a stale key: {rims:?}"));
    assert!(names[stale].is_err(), "the name door reads it unnamed");
    let said = rims[stale].clone().expect_err("stale").to_string();
    assert!(
        said.starts_with("rim names: ")
            && said.contains("built against an earlier evaluation")
            && !said.contains("kernel bug"),
        "the stale arm says what it is, a pick held across an edit: {said}"
    );
}
