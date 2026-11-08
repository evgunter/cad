//! Recheck probes for PR 6581's fix pass (lane `placing-recheck-6581`,
//! head `4103fdbc04`). Each row names the claim it tries to falsify.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::len;
use pncad::document::{
    AuthoredNode, Bodies, BodyRead, Dimension, Doc, Formula, Node, Operand, ProfileProgram,
    RecipeNodeId, VarId,
};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
use viewer::session::{DocSession, FaceSelection, PatternRuleSpec, Selection, SessionOp};
use viewer::tree::RowStatus;

const SIZE: [f64; 3] = [0.02, 0.02, 0.01];

type World = (
    Vec<(RecipeNodeId, Option<AuthoredNode>)>,
    Option<VarId>,
    Vec<RecipeNodeId>,
);

fn world(doc: &Doc<ProfileProgram>) -> World {
    let mut ids = doc.ids().to_vec();
    ids.sort_unstable();
    (
        ids.into_iter()
            .map(|id| (id, doc.node(id).map(|node| node.written(doc))))
            .collect(),
        doc.product(),
        viewer::world::drawn_nodes(doc),
    )
}

fn bracket(name: &str, tol: Tol) -> (DocSession, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived(name, tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    (session, body)
}

fn with(session: &DocSession, node: AuthoredNode, tol: Tol) -> (DocSession, RecipeNodeId) {
    let (doc, id) = viewer::test_support::inserted(session.committed_doc(), node, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    (session, id)
}

fn laterals(session: &DocSession, node: RecipeNodeId) -> Vec<StableName> {
    pncad::select::all_edges(session.evaluation().expect("landed"), node)
        .into_iter()
        .filter(|name| matches!(name.path.as_slice(), [RoleSeg::LateralEdge(_)]))
        .collect()
}

/// Whether `name` is `target` or carries it in at any depth.
fn carries(name: &StableName, target: &StableName) -> bool {
    name == target
        || name.path.iter().any(|seg| match seg {
            RoleSeg::From { of, .. } => carries(of.name(), target),
            _ => false,
        })
}

fn edge_carrying(session: &DocSession, on: RecipeNodeId, target: &StableName) -> StableName {
    pncad::select::all_edges(session.evaluation().expect("landed"), on)
        .into_iter()
        .find(|name| carries(name, target))
        .unwrap_or_else(|| panic!("{on:?} carries {target:?}"))
}

fn face_carrying(session: &DocSession, on: RecipeNodeId, target: &StableName) -> StableName {
    pncad::select::all_faces(session.evaluation().expect("landed"), on)
        .into_iter()
        .find(|name| carries(name, target))
        .unwrap_or_else(|| panic!("{on:?} carries face {target:?}"))
}

fn top_name(body: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: body,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    }
}

fn rows_ok(session: &DocSession, when: &str) -> Vec<String> {
    session
        .tree_rows()
        .into_iter()
        .filter(|row| row.status != RowStatus::Ok)
        .map(|row| format!("{when}: {row:?}"))
        .collect()
}

/// fillet(C) → chamfer → sketch on its top → fillet of what the sketch
/// made, on the copy `copy` of `body`: answers the made nodes.
fn mixed_chain(
    session: &mut DocSession,
    body: RecipeNodeId,
    copy: RecipeNodeId,
) -> Vec<RecipeNodeId> {
    let edges = laterals(session, body);
    let e0 = edge_carrying(session, copy, &edges[0]);
    let g = common::session_insert(
        session,
        SessionOp::AddFillet {
            target: copy,
            radius: len(0.001),
            selection: vec![e0],
        },
    );
    session.pump();
    let e1 = edge_carrying(session, g, &edges[1]);
    let k = common::session_insert(
        session,
        SessionOp::AddChamfer {
            target: g,
            distance: len(0.0005),
            selection: vec![e1],
        },
    );
    session.pump();
    let face = face_carrying(session, k, &top_name(body));
    let out = session.perform(SessionOp::SketchOnFace {
        face: FaceSelection {
            name: face,
            node: k,
            body: 0,
        },
        loops: vec![common::rectangle_loop([0.0, 0.0], 0.004, 0.004)],
        distance: len(0.003),
        spin: Formula::zero(Dimension::Angle),
    });
    assert!(out.refusal.is_none(), "sketch: {:?}", out.refusal);
    session.pump();
    let made: Vec<RecipeNodeId> = out.minted.clone();
    let fused = *made
        .iter()
        .find(|&&n| {
            matches!(
                session.committed_doc().node(n),
                Some(Node::Union { .. } | Node::Assemble { .. })
            )
        })
        .unwrap_or_else(|| panic!("the sketch's union among {made:?}"));
    let e2 = edge_carrying(session, fused, &edges[2]);
    let h = common::session_insert(
        session,
        SessionOp::AddFillet {
            target: fused,
            radius: len(0.0005),
            selection: vec![e2],
        },
    );
    session.pump();
    vec![g, k, fused, h]
}

fn fillet_and_repoint(session: &mut DocSession, body: RecipeNodeId) -> RecipeNodeId {
    let e3 = laterals(session, body)[3].clone();
    let f = common::session_insert(
        session,
        SessionOp::AddFillet {
            target: body,
            radius: len(0.001),
            selection: vec![e3],
        },
    );
    session.pump();
    let offer = viewer::world::repoint_offer(session.committed_doc(), f).expect("F offers");
    let out = session.perform(SessionOp::RepointPlacements {
        body: Operand::Var(offer.body),
        onto: f,
    });
    assert!(out.refusal.is_none(), "re-point: {:?}", out.refusal);
    session.pump();
    f
}

/// Claim 2: a four-deep mixed chain (fillet, chamfer, sketch, fillet) on
/// the copy BEFORE the re-point; F = fillet(B), re-point, delete F: the
/// document (nodes, product, drawn) is the one before F, rows well.
#[test]
fn rc_a_mixed_chain_before_the_repoint_survives_its_delete() {
    let tol = Tol::witness();
    let (session, body) = bracket("rc-deep-before", tol);
    let (doc, copy) = common::world_copy(session.committed_doc(), body, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let chain = mixed_chain(&mut session, body, copy);
    let mut bad = rows_ok(&session, "premise");
    let before = world(session.committed_doc());
    let f = fillet_and_repoint(&mut session, body);
    bad.extend(rows_ok(&session, "after the re-point"));
    let out = session.perform(SessionOp::DeleteNode { node: f });
    assert!(out.refusal.is_none(), "delete: {:?}", out.refusal);
    session.pump();
    bad.extend(rows_ok(&session, "after the delete"));
    for n in &chain {
        assert!(session.committed_doc().node(*n).is_some(), "{n:?} survives");
    }
    let after = world(session.committed_doc());
    assert!(bad.is_empty(), "{bad:#?}");
    assert!(after.0 == before.0, "nodes differ after the delete");
    assert_eq!(after.1, before.1, "product");
    assert_eq!(after.2, before.2, "drawn");
}

/// Claim 2: the same chain built on the copy AFTER the re-point (its
/// names now carry F's), then F deleted: the chain survives and every
/// row is well.
#[test]
fn rc_a_mixed_chain_after_the_repoint_survives_its_delete() {
    let tol = Tol::witness();
    let (session, body) = bracket("rc-deep-after", tol);
    let (doc, copy) = common::world_copy(session.committed_doc(), body, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let f = fillet_and_repoint(&mut session, body);
    let chain = mixed_chain(&mut session, body, copy);
    let mut bad = rows_ok(&session, "premise");
    let out = session.perform(SessionOp::DeleteNode { node: f });
    assert!(out.refusal.is_none(), "delete: {:?}", out.refusal);
    session.pump();
    for n in &chain {
        assert!(session.committed_doc().node(*n).is_some(), "{n:?} survives");
    }
    bad.extend(rows_ok(&session, "after the delete"));
    assert!(bad.is_empty(), "{bad:#?}");
}

fn family_and_copy(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (mut session, body) = bracket("rc-member", tol);
    let out = session.perform(SessionOp::AddPattern {
        input: body.into(),
        count: 2,
        rule: PatternRuleSpec::Linear {
            direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
            spacing: len(2.0 * SIZE[0]),
        },
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    let family = out.made.expect("the family");
    let out = session.perform(SessionOp::Place { body: body.into() });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    (session, body, family, out.made.expect("a copy"))
}

fn member(doc: &Doc<ProfileProgram>, family: RecipeNodeId, k: i64) -> BodyRead<Formula> {
    let var = doc.output(family, 0).expect("the family's output");
    BodyRead::indexed(Operand::Var(var), vec![Formula::count(k)])
}

/// Claim 3: `xs[0] − C` that NOTHING reads. DM6: a cut whose tool is
/// deleted continues its target; it has no reader to hand anything to,
/// so the delete has nothing it cannot write and should land.
#[test]
fn rc_a_cut_of_a_member_nobody_reads_deletes_its_tool() {
    let tol = Tol::witness();
    let (session, _, family, copy) = family_and_copy(tol);
    let m0 = member(session.committed_doc(), family, 0);
    let (mut session, cut) = with(
        &session,
        Node::Subtract {
            from: m0,
            tool: Operand::Node(copy).into(),
        },
        tol,
    );
    let out = session.perform(SessionOp::DeleteNode { node: copy });
    assert!(
        out.refusal.is_none(),
        "the cut {cut:?} has no reader: {}",
        out.refusal.map(|r| r.to_string()).unwrap_or_default()
    );
}

/// Claim 3: `xs[0] − C` read only as a member of `∪[cut, xs[1]]`. A list
/// keeps indices (SetMembers), the edit the fix uses for a fillet over a
/// family, so the union could be handed `xs[0]`.
#[test]
fn rc_a_cut_of_a_member_read_by_a_list_is_handed_the_member() {
    let tol = Tol::witness();
    let (session, _, family, copy) = family_and_copy(tol);
    let doc = session.committed_doc();
    let (m0, m1) = (member(doc, family, 0), member(doc, family, 1));
    let (session, cut) = with(
        &session,
        Node::Subtract {
            from: m0,
            tool: Operand::Node(copy).into(),
        },
        tol,
    );
    let (mut session, u) = with(
        &session,
        Node::Union {
            members: Bodies::Spelled(vec![Operand::Node(cut).into(), m1]),
        },
        tol,
    );
    let out = session.perform(SessionOp::DeleteNode { node: copy });
    assert!(
        out.refusal.is_none(),
        "the union {u:?} is a list: {}",
        out.refusal.map(|r| r.to_string()).unwrap_or_default()
    );
}

/// Claim 3 (no widening): `∩[xs[0], C]` read by a world copy; deleting
/// C keeps member 0, the copy reads the ∩, never the family.
#[test]
fn rc_an_intersection_left_with_a_member_keeps_it() {
    let tol = Tol::witness();
    let (session, _, family, copy) = family_and_copy(tol);
    let m0 = member(session.committed_doc(), family, 0);
    let (session, i) = with(
        &session,
        Node::Intersect {
            members: Bodies::Spelled(vec![m0, Operand::Node(copy).into()]),
        },
        tol,
    );
    let (mut session, reader) = with(&session, common::world_place(Operand::Node(i)), tol);
    let out = session.perform(SessionOp::DeleteNode { node: copy });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    assert_eq!(doc.copied_read(reader), doc.output(i, 0), "reads the ∩");
}

/// Claim 4: a copy of a copy, four features deep with a sketch, says it
/// is on the copy and goes to B; a copy of a member copy reaches B.
#[test]
fn rc_on_this_copy_at_depth_and_through_member_copies() {
    let tol = Tol::witness();
    let (session, body) = bracket("rc-depth", tol);
    let (doc, c1) = common::world_copy(session.committed_doc(), body, tol);
    let (doc, c2) =
        viewer::test_support::inserted(&doc, common::world_place(Operand::Node(c1)), tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let chain = mixed_chain(&mut session, body, c2);
    let doc = session.committed_doc();
    let mut bad = Vec::new();
    for &n in &chain {
        let (said, offers) = viewer::frame::placement_offers(doc, n);
        let ok = said.as_deref().is_some_and(|s| s.contains("placed copy"))
            && offers
                .iter()
                .any(|(_, op)| *op == SessionOp::Select(Selection::Node(body)));
        if !ok || viewer::world::built_on_copy(doc, n).map(|(c, _)| c) != Some(c2) {
            bad.push(format!(
                "{n:?}: {said:?} {offers:?} {:?}",
                viewer::world::built_on_copy(doc, n)
            ));
        }
    }
    for id in doc.ids() {
        if matches!(doc.node(id), Some(Node::Profile(_))) {
            let (said, _) = viewer::frame::placement_offers(doc, id);
            if !said.as_deref().is_some_and(|s| s.contains("placed copy")) {
                bad.push(format!("profile {id:?}: {said:?}"));
            }
        }
    }

    let (session, body, family, _) = family_and_copy(tol);
    let m1 = member(session.committed_doc(), family, 1);
    let (session, mc) = with(
        &session,
        Node::place(
            m1,
            Operand::Var(VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    let (session, mcc) = with(&session, common::world_place(Operand::Node(mc)), tol);
    let doc = session.committed_doc();
    if viewer::world::original_of(doc, mcc) != body {
        bad.push(format!(
            "copy of a member copy: original {:?}",
            viewer::world::original_of(doc, mcc)
        ));
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// Claim 1: a cycle through a mate. C1 is mated onto C2; U = B ∪ C1 reads
/// C2 through C1's pose. C3 is free. The offer must leave C1 and C2 out,
/// and its button must land.
#[test]
fn rc_the_offer_leaves_out_a_placement_read_through_a_mate() {
    let tol = Tol::witness();
    let (mut session, body) = bracket("rc-mate-cycle", tol);
    let place = |session: &mut DocSession| {
        let out = session.perform(SessionOp::Place { body: body.into() });
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
        session.pump();
        out.made.expect("a copy")
    };
    let c1 = place(&mut session);
    let c2 = place(&mut session);
    let c3 = place(&mut session);
    let bottom = StableName {
        kind: EntityKind::Face,
        node: body,
        path: vec![RoleSeg::Cap(CapEnd::Start)],
    };
    common::commit_mate(
        &mut session,
        SessionOp::AddMate {
            placement: c1,
            mate: Box::new(crate::fixture::rest(
                Operand::select(Operand::Node(body), vec![bottom]),
                Operand::select(Operand::Node(c2), vec![top_name(body)]),
            )),
            values: Vec::new(),
            into_target_space: false,
        },
    );
    let (mut session, u) = with(
        &session,
        Node::Union {
            members: Bodies::Spelled(vec![Operand::Node(body).into(), Operand::Node(c1).into()]),
        },
        tol,
    );
    let doc = session.committed_doc().clone();
    let offer = viewer::world::repoint_offer(&doc, u).expect("U offers");
    assert_eq!(offer.placements, [c3], "C1 and C2 left out");
    let (_, offers) = viewer::frame::placement_offers(&doc, u);
    let (_, op) = offers
        .into_iter()
        .find(|(_, op)| matches!(op, SessionOp::RepointPlacements { .. }))
        .expect("the button");
    let out = session.perform(op);
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    assert_eq!(doc.copied_read(c3), doc.output(u, 0));
}

/// Claim 5: a split of a body whose only readers are placements (no `⊔`,
/// no product). README "A new body lands unplaced": "where nothing reads
/// the body along such a path [a `⊔` operand, the product], the split's
/// halves are drawn in its place". Goes red where the gesture writes
/// `above ⊔ below` anyway (the fix pass's item 5).
#[test]
fn rc_a_split_of_a_body_only_placements_read() {
    let tol = Tol::witness();
    let (session, body) = bracket("rc-split", tol);
    let (doc, _copy) = common::world_copy(session.committed_doc(), body, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let plane = common::cut_across(&session, body, SIZE[2] / 2.0);
    let out = session.perform(SessionOp::AddSplit {
        target: body.into(),
        plane,
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    let assembles: Vec<_> = out
        .minted
        .iter()
        .filter(|&&n| matches!(doc.node(n), Some(Node::Assemble { .. })))
        .collect();
    assert!(
        assembles.is_empty(),
        "README: nothing reads B along a seen path, so the halves are drawn in its \
         place; the gesture wrote `above ⊔ below` {assembles:?}"
    );
}

/// Claim 2 / Q1: the forward re-point rebinds down `names::verbatim_edge`
/// chains (`take_over::chains`: a copy, a mirror, a split of the copy);
/// the splice's `rebinds` closes over `doc.placements()` alone. A mirror
/// of the copy C (`C ∪ image`), and a fillet on that union picking an
/// edge the IMAGE carries; then F = fillet(B), re-point, delete F: the
/// document must be the one before F, every row well. Red at head
/// `4103fdbc04` AFTER THE RE-POINT (the forward door leaves G stale);
/// the delete itself restores the nodes.
#[test]
fn rc_a_feature_on_a_mirror_of_the_copy_survives_the_repoint_and_its_delete() {
    let tol = Tol::witness();
    let (session, body) = bracket("rc-mirror-chain", tol);
    let (doc, copy) = common::world_copy(session.committed_doc(), body, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let index = common::plate_index(&session);
    let plane =
        common::displayed_face_at(&session, &index, &common::along_x(-1.0, 0.0, SIZE[2] / 2.0));
    assert_eq!(plane.node, copy, "the wall is picked on the copy");
    let out = session.perform(SessionOp::AddMirror {
        body: copy.into(),
        plane,
    });
    assert!(out.refusal.is_none(), "mirror: {:?}", out.refusal);
    session.pump();
    let image = out.made.expect("the mirror");
    let doc = session.committed_doc();
    let image_out = doc.output(image, 0).expect("the image");
    let union = *out
        .minted
        .iter()
        .find(|&&n| matches!(doc.node(n), Some(Node::Union { .. })))
        .expect("C ∪ image");
    // An edge F (which fillets lateral 3) leaves alone: lateral 0's image.
    let keep = laterals(&session, body)[0].clone();
    let edge = pncad::select::all_edges(session.evaluation().expect("landed"), union)
        .into_iter()
        .find(|name| {
            matches!(name.path.as_slice(),
                [RoleSeg::From { read, of }] if *read == image_out
                    && carries(of.name(), &keep))
        })
        .expect("an image lateral edge on the union");
    let g = common::session_insert(
        &mut session,
        SessionOp::AddFillet {
            target: union,
            radius: len(0.0005),
            selection: vec![edge],
        },
    );
    session.pump();
    let mut bad = rows_ok(&session, "premise");
    let before = world(session.committed_doc());
    let f = fillet_and_repoint(&mut session, body);
    bad.extend(rows_ok(&session, "after the re-point"));
    let out = session.perform(SessionOp::DeleteNode { node: f });
    assert!(out.refusal.is_none(), "delete: {:?}", out.refusal);
    session.pump();
    assert!(session.committed_doc().node(g).is_some(), "G survives");
    bad.extend(rows_ok(&session, "after the delete"));
    let after = world(session.committed_doc());
    assert!(
        after.0 == before.0,
        "nodes differ after the delete; rows: {bad:#?}"
    );
    assert!(bad.is_empty(), "{bad:#?}");
}
