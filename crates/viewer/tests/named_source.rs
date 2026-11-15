//! **Placements of a body share a named source, and the re-point
//! redefines it** (Ev, PR 6581 and PR 6672; VARIABLES VR4): a placement
//! reads its body through a source, `bracket = B`. A lone placement's
//! source is anonymous and its own; a second Place of the body shares it
//! only once the person names it; the offer on a feature of the body is
//! the shortcut that redefines it, `bracket = F`, as one action; and a
//! later Place of the body reads `bracket`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use pncad::document::{
    Doc, DocEdit, EditError, Operand, ProfileProgram, RecipeNodeId, VarDecl, VarId, VarName,
};
use pncad::geom_core::Tol;
use pncad::prelude::RoleSeg;
use viewer::session::{DocSession, Refusal, SessionOp};
use viewer::tree::RowStatus;

const SIZE: [f64; 3] = [0.02, 0.02, 0.01];

/// The bracket in a session of its own, landed.
fn bracket(name: &str, tol: Tol) -> (DocSession, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived(name, tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    (session, body)
}

/// The viewer's Place of `body` where it stands, under `name`.
fn place_op(doc: &Doc<ProfileProgram>, body: Operand, name: Option<&'static str>) -> SessionOp {
    let SessionOp::Place { body, target, .. } =
        SessionOp::place_where_it_stands(doc, body).expect("a body in a space")
    else {
        unreachable!("the viewer's Place")
    };
    SessionOp::Place {
        body,
        target,
        name: name.map(VarName::from_static),
    }
}

/// `op` performed and landed: the node it made.
fn landed(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let out = session.perform(op);
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    out.made.expect("a node")
}

/// The body `copy` copies, down its source.
fn copied(doc: &Doc<ProfileProgram>, copy: RecipeNodeId) -> Option<VarId> {
    doc.copied_read(copy)
        .map(|source| doc.through_sources(source).read)
}

/// The bracket placed twice through the session: the first Place mints
/// the bracket's source, and the second names it `bracket`.
fn placed_twice(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (mut session, body) = bracket("named-source", tol);
    let first = {
        let op = place_op(session.committed_doc(), body.into(), None);
        landed(&mut session, op)
    };
    let second = {
        let op = place_op(session.committed_doc(), body.into(), Some("bracket"));
        landed(&mut session, op)
    };
    (session, body, first, second)
}

/// **Placing a body twice shares one named source**: the first Place
/// reads an anonymous source of its own (`= B`). A second Place without
/// a name is refused `SharedVarNeedsName` and writes nothing, since a
/// variable two readers share is named (VR2) and nothing proposes one;
/// with the name the person typed it shares the source and names it, in
/// one action.
///
/// *Breaks if* the second Place mints a second source, shares an
/// anonymous one, or proposes a name of its own.
#[test]
fn placing_a_body_twice_shares_one_named_source() {
    let tol = Tol::witness();
    let (mut session, body) = bracket("named-source-twice", tol);
    let first = {
        let op = place_op(session.committed_doc(), body.into(), None);
        landed(&mut session, op)
    };
    let doc = session.committed_doc().clone();
    let source = doc
        .copied_read(first)
        .expect("the placement reads a source");
    assert_eq!(
        doc.var_name(source),
        None,
        "a lone placement's source is anonymous"
    );
    assert_eq!(
        doc.source(source).map(|read| read.read),
        doc.output(body, 0),
        "and reads the bracket"
    );

    let steps = session.history().len();
    let refused = session.perform(place_op(&doc, body.into(), None));
    assert!(
        matches!(
            &refused.refusal,
            Some(Refusal::Edit(error))
                if matches!(**error, EditError::SharedVarNeedsName { ref var } if var.id() == source)
        ),
        "a second reader needs the name: {:?}",
        refused.refusal
    );
    assert_eq!(session.history().len(), steps, "nothing is written");
    assert!(
        session.committed_doc().bit_eq(&doc),
        "the document is as it was"
    );

    let second = landed(&mut session, place_op(&doc, body.into(), Some("bracket")));
    assert_eq!(session.history().len(), steps + 1, "one action");
    let doc = session.committed_doc();
    assert_eq!(
        doc.var_named("bracket"),
        Some(source),
        "the source is named"
    );
    assert_eq!(
        [first, second].map(|copy| doc.copied_read(copy)),
        [Some(source); 2],
        "both placements read it"
    );
    assert_eq!(
        doc.sources_of(doc.output(body, 0).expect("the bracket's body")),
        [source],
        "the bracket has one source"
    );
}

/// **Accepting the offer writes one definition edit**: a fillet of the
/// bracket picked as itself offers to redefine `bracket`; accepted, it is
/// one action whose only definition edit is `bracket = F`, the `Rebind`s
/// it owes beside it, and every placement reading `bracket` shows `F`. One
/// undo takes it back.
///
/// *Breaks if* the re-point writes each placement's source, a second
/// definition, or more than one undo step.
#[test]
fn accepting_the_offer_redefines_the_source_once() {
    let tol = Tol::witness();
    let (mut session, body, first, second) = placed_twice(tol);
    let bracket = session
        .committed_doc()
        .var_named("bracket")
        .expect("the source");
    let edge = pncad::select::all_edges(session.evaluation().expect("landed"), body)
        .into_iter()
        .find(|name| matches!(name.path.as_slice(), [RoleSeg::LateralEdge(_)]))
        .expect("a lateral edge");
    let fillet = landed(
        &mut session,
        SessionOp::AddFillet {
            target: body,
            radius: common::len(0.001),
            selection: vec![edge],
        },
    );
    let before = session.committed_doc().clone();
    let offers = viewer::frame::placement_offers(&before, fillet, pncad::geom_core::Tol::witness());
    let [viewer::frame::PlacementOffer { said, label, op }] = &offers[..] else {
        panic!("one offer: {offers:?}")
    };
    assert_eq!(
        *op,
        SessionOp::RepointPlacements {
            source: bracket,
            onto: fillet.into(),
        },
        "the offer redefines the named source"
    );
    assert!(
        said.starts_with("bracket (2 copies) copies ")
            && said.ends_with(", which this body is built on"),
        "the row states the fact: {said}"
    );
    assert!(label.starts_with("Re-point bracket onto "), "{label}");

    let steps = session.history().len();
    let out = session.perform(op.clone());
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert_eq!(session.history().len(), steps + 1, "one action");
    let defines: Vec<_> = out
        .committed
        .iter()
        .filter(|edit| matches!(edit, DocEdit::DefineVar { .. }))
        .collect();
    assert!(
        matches!(
            defines[..],
            [DocEdit::DefineVar {
                var: pncad::document::VarRef::Id(var),
                def: VarDecl::Source(_),
                ..
            }] if *var == bracket
        ),
        "one DefineVar, of bracket: {:?}",
        out.committed
    );
    assert!(
        out.committed
            .iter()
            .all(|edit| matches!(edit, DocEdit::DefineVar { .. } | DocEdit::Rebind { .. })),
        "and the Rebinds it owes, nothing else: {:?}",
        out.committed
    );
    session.pump();
    let doc = session.committed_doc();
    assert_eq!(
        doc.source(bracket).map(|read| read.read),
        doc.output(fillet, 0),
        "bracket = F"
    );
    assert_eq!(
        [first, second].map(|copy| (doc.copied_read(copy), copied(doc, copy))),
        [(Some(bracket), doc.output(fillet, 0)); 2],
        "both copies read bracket, and show F"
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    assert_eq!(
        viewer::world::repoint_offer(doc, fillet, pncad::geom_core::Tol::witness()),
        None,
        "nothing left to offer"
    );

    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo takes the redefinition back"
    );
}

/// **A later Place of `bracket` reads `bracket`**: after `bracket = F`,
/// Place on `F` picked as itself shares the source `F`'s placements read,
/// as Place on the source itself does, so the new copy shows `F` and
/// follows the next redefinition.
///
/// *Breaks if* the later Place reads `F`'s output, or mints a source of
/// its own beside `bracket`.
#[test]
fn a_later_place_of_bracket_reads_bracket() {
    let tol = Tol::witness();
    let (mut session, body, _, _) = placed_twice(tol);
    let bracket = session
        .committed_doc()
        .var_named("bracket")
        .expect("the source");
    let edge = pncad::select::all_edges(session.evaluation().expect("landed"), body)
        .into_iter()
        .find(|name| matches!(name.path.as_slice(), [RoleSeg::LateralEdge(_)]))
        .expect("a lateral edge");
    let fillet = landed(
        &mut session,
        SessionOp::AddFillet {
            target: body,
            radius: common::len(0.001),
            selection: vec![edge],
        },
    );
    landed(
        &mut session,
        SessionOp::RepointPlacements {
            source: bracket,
            onto: fillet.into(),
        },
    );
    for read in [Operand::Node(fillet), Operand::Var(bracket)] {
        let third = {
            let op = place_op(session.committed_doc(), read.clone(), None);
            landed(&mut session, op)
        };
        let doc = session.committed_doc();
        assert_eq!(
            doc.copied_read(third),
            Some(bracket),
            "{read:?}: reads bracket"
        );
        assert_eq!(
            copied(doc, third),
            doc.output(fillet, 0),
            "{read:?}: shows F"
        );
        assert_eq!(
            doc.sources_of(doc.output(fillet, 0).expect("F's body")),
            [bracket],
            "{read:?}: no second source"
        );
    }
}

/// **Place on a body several sources read refuses, naming each**: two
/// placements made by hand through a source each (two anonymous sources
/// of one body raise no finding); Place on the body asks which, typed,
/// and Place on either source places through it, named as it becomes
/// shared.
///
/// *Breaks if* Place picks a source unasked, or mints a third.
#[test]
fn place_on_a_body_several_sources_read_names_them() {
    let tol = Tol::witness();
    let (session, body) = bracket("named-source-several", tol);
    let space = session.committed_doc().node_space(body).expect("its space");
    let apart = |doc: &Doc<ProfileProgram>| {
        viewer::test_support::inserted(
            doc,
            pncad::document::Node::place(
                pncad::document::BodyRead::plain(Operand::Node(body)),
                Operand::Var(space),
                pncad::document::stand().bundle(),
            ),
            tol,
        )
    };
    let (doc, a) = apart(session.committed_doc());
    let (doc, b) = apart(&doc);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let doc = session.committed_doc().clone();
    let sources = [a, b].map(|copy| doc.copied_read(copy).expect("a source"));
    assert_ne!(sources[0], sources[1], "two sources");
    let out = session.perform(place_op(&doc, body.into(), None));
    assert!(
        matches!(
            &out.refusal,
            Some(Refusal::SeveralSources { sources: named, .. })
                if named.iter().map(|s| s.id()).collect::<Vec<_>>() == sources
        ),
        "{:?}",
        out.refusal
    );
    assert!(session.committed_doc().bit_eq(&doc), "nothing is written");
    let third = landed(
        &mut session,
        place_op(&doc, Operand::Var(sources[1]), Some("bracket")),
    );
    let doc = session.committed_doc();
    assert_eq!(
        doc.copied_read(third),
        Some(sources[1]),
        "shares the one asked for"
    );
    assert_eq!(
        doc.var_named("bracket"),
        Some(sources[1]),
        "named as it is shared"
    );
}

// ---- review rows (PR 6856's dual review), pinning the fixed behaviour ----

/// A revolve (body, axis and space: three ports) in a session of its own.
fn revolved(tol: Tol) -> (DocSession, RecipeNodeId) {
    let mut session = DocSession::inline(Doc::empty_derived("named-source-revolve", tol), tol);
    let profile = common::session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![common::circle_at([0.3, 0.0], 0.07)],
        },
    );
    let revolve = common::session_insert(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            axis_through: [common::len(0.0), common::len(0.0)],
            axis_heading: pncad::document::Formula::turn_over(4).expect("a quarter turn"),
            angle: common::ang(core::f64::consts::TAU),
        },
    );
    session.pump();
    (session, revolve)
}

/// **Every gesture that places a body read by its output variable lands**
/// through a source (VARIABLES VR4, VR9): a revolve's node defines its body
/// beside its axis, so the viewer reads it as `Var(body)`, and Place and
/// the pattern place it alike.
///
/// *Breaks if* a construction path writes the body read as given.
#[test]
fn a_revolve_is_placed_and_patterned_through_a_source() {
    let tol = Tol::witness();
    let (mut session, revolve) = revolved(tol);
    let op = place_op(session.committed_doc(), revolve.into(), None);
    let copy = landed(&mut session, op);
    let doc = session.committed_doc();
    assert_eq!(
        copied(doc, copy),
        doc.output(revolve, 0),
        "Place reads the body"
    );
    let (mut session, revolve) = revolved(tol);
    let family = landed(
        &mut session,
        SessionOp::AddPattern {
            input: revolve.into(),
            count: 2,
            rule: viewer::session::PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: common::len(1.0),
            },
        },
    );
    let doc = session.committed_doc();
    let source = doc.copied_read(family).expect("the pattern reads a source");
    assert_eq!(
        doc.source(source).map(|read| read.read),
        doc.output(revolve, 0)
    );
}

/// **The combine places a revolve off its space through a source**
/// (`session::in_one_space`): a union of a box and a revolve in a space of
/// its own places the revolve into the box's space, its body read as the
/// revolve's body variable (`as_body`), which the placement reads through
/// a source it mints. The union lands and evaluates.
#[test]
fn the_combine_places_a_revolve_off_its_space_through_a_source() {
    let tol = Tol::witness();
    let (mut session, revolve) = revolved(tol);
    let boxed = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let union = landed(
        &mut session,
        SessionOp::AddBoolean {
            spec: viewer::session::BooleanSpec::Union(vec![boxed.into(), revolve.into()]),
        },
    );
    let doc = session.committed_doc();
    let body = doc.output(revolve, 0).expect("the revolve's body");
    let placed: Vec<_> = doc
        .placements()
        .into_iter()
        .filter(|&p| copied(doc, p) == Some(body))
        .collect();
    assert_eq!(placed.len(), 1, "one placement of the revolve");
    assert!(
        doc.copied_read(placed[0])
            .is_some_and(|source| doc.source(source).is_some()),
        "it reads the revolve through a source"
    );
    assert!(
        session
            .evaluation()
            .is_some_and(|ev| ev.node_error(union).is_none()),
        "the union evaluates"
    );
}

/// **A member placement's family source is not re-pointed onto one body**:
/// the "copy of" row's and the variables pane's op, redefining it onto a
/// body that is no family, refuses typed (`SlotVarKind` at the member's
/// source seat), and the offer has no row for it.
///
/// *Breaks if* the session commits a document the load refuses.
#[test]
fn a_member_placements_source_is_not_re_pointed_onto_one_body() {
    let tol = Tol::witness();
    let (mut session, body) = bracket("named-source-member", tol);
    let other = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let family = landed(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: viewer::session::PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: common::len(2.0 * SIZE[0]),
            },
        },
    );
    let (doc, member) = common::member_read(session.committed_doc(), family, 0, tol);
    let source = doc.copied_read(member).expect("a source");
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let before = session.committed_doc().clone();
    let out = session.perform(SessionOp::RepointPlacements {
        source,
        onto: other.into(),
    });
    assert!(
        matches!(&out.refusal, Some(Refusal::Edit(error))
            if matches!(**error, EditError::SlotVarKind { .. })),
        "{:?}",
        out.refusal
    );
    assert!(session.committed_doc().bit_eq(&before), "nothing written");
    assert!(
        viewer::world::repoint_targets(
            session.committed_doc(),
            doc_body(&before, family),
            other,
            pncad::geom_core::Tol::witness()
        )
        .iter()
        .all(|row| row.source != source),
        "no offer row re-points it onto one body"
    );
}

/// The body variable the family `family` repeats.
fn doc_body(doc: &Doc<ProfileProgram>, family: RecipeNodeId) -> VarId {
    copied(doc, family).expect("the family copies a body")
}

/// **Accepting the offer and deleting F is the identity** on the written
/// nodes and on the whole variable table — every definition and name, none
/// added or lost.
#[test]
fn accept_then_delete_f_is_the_identity_on_the_variables() {
    let tol = Tol::witness();
    let (mut session, body, _, _) = placed_twice(tol);
    let bracket = session.committed_doc().var_named("bracket").expect("named");
    let table = |doc: &Doc<ProfileProgram>| {
        doc.var_ids()
            .iter()
            .map(|&id| {
                (
                    id,
                    doc.var(id).expect("live").def().clone(),
                    doc.var_name(id).cloned(),
                )
            })
            .collect::<Vec<_>>()
    };
    let nodes = |doc: &Doc<ProfileProgram>| {
        let mut ids = doc.ids().to_vec();
        ids.sort_unstable();
        ids.into_iter()
            .map(|id| format!("{id:?}: {:?}", doc.node(id).map(|n| n.written(doc))))
            .collect::<Vec<_>>()
    };
    let before = (
        nodes(session.committed_doc()),
        table(session.committed_doc()),
    );
    let edge = pncad::select::all_edges(session.evaluation().expect("landed"), body)
        .into_iter()
        .find(|name| matches!(name.path.as_slice(), [RoleSeg::LateralEdge(_)]))
        .expect("a lateral edge");
    let f = landed(
        &mut session,
        SessionOp::AddFillet {
            target: body,
            radius: common::len(0.001),
            selection: vec![edge],
        },
    );
    landed(
        &mut session,
        SessionOp::RepointPlacements {
            source: bracket,
            onto: f.into(),
        },
    );
    let out = session.perform(SessionOp::DeleteNode { node: f });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    let after = (
        nodes(session.committed_doc()),
        table(session.committed_doc()),
    );
    assert_eq!(after.0, before.0, "the nodes");
    assert_eq!(after.1, before.1, "the variables");
}

/// **Place on a body a pattern places shares the pattern's source**
/// (rule 9 as written): without a name it is refused `SharedVarNeedsName`;
/// with one it shares the pattern's source and names it.
#[test]
fn place_on_a_pattern_placed_body_shares_the_patterns_source() {
    let tol = Tol::witness();
    let (mut session, body) = bracket("named-source-pattern", tol);
    let family = landed(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: viewer::session::PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: common::len(2.0 * SIZE[0]),
            },
        },
    );
    let doc = session.committed_doc().clone();
    let pattern_source = doc.copied_read(family).expect("the pattern reads a source");
    let out = session.perform(place_op(&doc, body.into(), None));
    assert!(
        matches!(&out.refusal, Some(Refusal::Edit(error))
            if matches!(**error, EditError::SharedVarNeedsName { .. })),
        "{:?}",
        out.refusal
    );
    let copy = landed(&mut session, place_op(&doc, body.into(), Some("bracket")));
    let doc = session.committed_doc();
    assert_eq!(doc.copied_read(copy), Some(pattern_source));
    assert_eq!(doc.var_named("bracket"), Some(pattern_source));
}

/// **A Place given a name for a named source refuses in its own words**.
#[test]
fn a_place_naming_a_named_source_refuses() {
    let tol = Tol::witness();
    let (mut session, body, _, _) = placed_twice(tol);
    let op = place_op(session.committed_doc(), body.into(), Some("again"));
    let out = session.perform(op);
    assert!(
        matches!(&out.refusal, Some(Refusal::SourceIsNamed { source })
            if source.name().map(VarName::as_str) == Some("bracket")),
        "{:?}",
        out.refusal
    );
}

/// The lateral edges of `node`'s body, as the landed evaluation names
/// them.
fn lateral_edges(session: &DocSession, node: RecipeNodeId) -> Vec<pncad::prelude::StableName> {
    pncad::select::all_edges(session.evaluation().expect("landed"), node)
        .into_iter()
        .filter(|name| matches!(name.path.as_slice(), [RoleSeg::LateralEdge(_)]))
        .collect()
}

/// **The offer reaches through a chain of features** (Ev, PR 6915: it
/// stands on every drawn body that reads, at any depth, as a body, one
/// a source places): `F = fillet(B)` offers `bracket`; left unpressed, `G =
/// chamfer(F)` makes `F` an intermediate body the picture no longer
/// draws, so `F` offers nothing and `G` offers `bracket`, saying what it
/// re-points. Pressed, `bracket` reads `G`.
///
/// *Breaks if* the offer looks only at the bodies the node reads
/// directly (`G` reads `F`, which no source reads), or an undrawn `F`
/// keeps offering.
#[test]
fn a_feature_on_an_unpressed_fillet_offers_bracket() {
    let tol = Tol::witness();
    let (mut session, body, _, _) = placed_twice(tol);
    let bracket = session
        .committed_doc()
        .var_named("bracket")
        .expect("the source");
    let edges = lateral_edges(&session, body);
    let f = landed(
        &mut session,
        SessionOp::AddFillet {
            target: body,
            radius: common::len(0.001),
            selection: vec![edges[0].clone()],
        },
    );
    let sources = |doc: &Doc<ProfileProgram>, node| {
        viewer::world::repoint_offer(doc, node, tol)
            .map(|offer| offer.rows.iter().map(|row| row.source).collect::<Vec<_>>())
    };
    assert_eq!(
        sources(session.committed_doc(), f),
        Some(vec![bracket]),
        "F, drawn, offers bracket"
    );

    let carried = pncad::select::all_edges(session.evaluation().expect("landed"), f)
        .into_iter()
        .find(|name| {
            matches!(name.path.as_slice(),
                [RoleSeg::From { of, .. }] if *of.name() == edges[1])
        })
        .expect("F carries a second lateral edge of B");
    let g = landed(
        &mut session,
        SessionOp::AddChamfer {
            target: f,
            distance: common::len(0.0005),
            selection: vec![carried],
        },
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    let doc = session.committed_doc().clone();
    let picture = viewer::world::Picture::of(&doc);
    let drawn = |node| picture.drawn.iter().any(|d| d.of_node(node));
    assert_eq!([drawn(f), drawn(g)], [false, true], "G is drawn, F is not");
    assert_eq!(sources(&doc, f), None, "F, no longer drawn, offers nothing");
    assert_eq!(
        sources(&doc, g),
        Some(vec![bracket]),
        "G, built on B through F, offers bracket"
    );
    let by = pncad::document::Speaker::of(&doc);
    let b = doc.output(body, 0).expect("B's body");
    let offers = viewer::frame::placement_offers(&doc, g, tol);
    assert_eq!(
        offers,
        [viewer::frame::PlacementOffer {
            said: format!(
                "bracket (2 copies) copies {}, which this body is built on",
                by.read(b)
            ),
            label: format!("Re-point bracket onto {}", by.node(g)),
            op: SessionOp::RepointPlacements {
                source: bracket,
                onto: g.into(),
            },
        }],
        "the row states the fact beside its button"
    );

    let out = session.perform(offers[0].op.clone());
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    assert_eq!(
        doc.source(bracket).map(|read| read.read),
        doc.output(g, 0),
        "bracket reads G"
    );
}

/// **A cut offers the source of its tool too** (Ev, PR 6915: a node that
/// reaches several placed bodies offers a row for each source): with
/// `bracket = B` placed twice and a tool `T` (the Transform's copy of a
/// post, in B's space) placed in the world through an anonymous source,
/// `D = B − T` offers both, in id order, each row saying which body its
/// source copies; the tool's row lands.
///
/// *Breaks if* the offer stops at the first body with a placed source,
/// or leaves out a copy the node reads as a body (the walk stops at `T`,
/// whose own placement is still a row).
#[test]
fn a_cut_offers_the_source_of_its_tool_as_well() {
    let tol = Tol::witness();
    let (mut session, body, _, _) = placed_twice(tol);
    let bracket = session
        .committed_doc()
        .var_named("bracket")
        .expect("the source");
    // A post through B's edge, moved into B's space (a combine moves no
    // copy), and that copy placed in the world by hand.
    let raw = common::xy_box_in(&mut session, [0.008, 0.008, 0.02]);
    let tool = common::moved_to(&mut session, raw, [0.01, 0.0, -0.005]);
    let home = session
        .committed_doc()
        .node_space(body)
        .expect("B is in a space");
    common::into_space(&mut session, tool, home);
    let (mut session, tool_copy) = common::world_copy_by_hand_in(&session, tool, tol);
    let tool_source = session
        .committed_doc()
        .copied_read(tool_copy)
        .expect("T's source");
    let cut = landed(
        &mut session,
        SessionOp::AddBoolean {
            spec: viewer::session::BooleanSpec::Subtract {
                from: body.into(),
                tool: tool.into(),
            },
        },
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    let doc = session.committed_doc().clone();
    let b = doc.output(body, 0).expect("B's body");
    let t = doc.output(tool, 0).expect("T's body");
    let offer = viewer::world::repoint_offer(&doc, cut, tol).expect("the cut offers");
    let mut want = vec![(bracket, b), (tool_source, t)];
    want.sort();
    assert_eq!(
        offer
            .rows
            .iter()
            .map(|row| (row.source, row.body))
            .collect::<Vec<_>>(),
        want,
        "a row for bracket and one for T's source, in id order"
    );
    let offers = viewer::frame::placement_offers(&doc, cut, tol);
    let by = pncad::document::Speaker::of(&doc);
    let tool_row = offers
        .iter()
        .find(|offer| {
            offer.op
                == SessionOp::RepointPlacements {
                    source: tool_source,
                    onto: cut.into(),
                }
        })
        .unwrap_or_else(|| panic!("T's row: {offers:?}"));
    assert_eq!(
        tool_row.said,
        format!(
            "the copy {} copies {}, which this body is built on",
            by.node(tool_copy),
            by.read(t)
        ),
        "the tool's row names its copy and the body it copies"
    );

    let out = session.perform(tool_row.op.clone());
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    assert_eq!(
        doc.source(tool_source).map(|read| read.read),
        doc.output(cut, 0),
        "T's copy reads the cut"
    );
    assert_eq!(
        doc.source(bracket).map(|read| read.read),
        Some(b),
        "bracket left on B"
    );
}

/// B's top cap, picked on B itself.
fn top_of(body: RecipeNodeId) -> viewer::session::FaceSelection {
    viewer::session::FaceSelection {
        name: pncad::prelude::StableName {
            kind: pncad::prelude::EntityKind::Face,
            node: body,
            path: vec![RoleSeg::Cap(pncad::prelude::CapEnd::End)],
        },
        node: body,
        body: 0,
    }
}

/// **A body that reads a placed body only through a plane offers
/// nothing for it** (orchestrator, within Ev's PR 6915 rule: built on a
/// body means reads it as a body): `bracket = B` placed twice, a mirror
/// of a post `X` in B's space whose plane is B's top cap, picked on B,
/// offers nothing (nor does `X ∪ mirror(X)`, the body the gesture
/// draws) — it reads B as a plane, not as a body. The post is itself a
/// placed copy (`moved_to` places it), which the union reads as a body,
/// so the pane says the union is on the post's copy, not on B (Ev, PR
/// 6917: "built on a copy" reads bodies as bodies too).
///
/// *Breaks if* the offer's walk follows a mirror's plane seat, or "built
/// on a copy" does (it would reach B, built from nothing, and say
/// nothing).
#[test]
fn a_mirror_whose_plane_is_on_b_offers_nothing_for_bracket() {
    let tol = Tol::witness();
    let (mut session, body, _, _) = placed_twice(tol);
    let raw = common::xy_box_in(&mut session, [0.008, 0.008, 0.004]);
    let post = common::moved_to(&mut session, raw, [0.05, 0.0, 0.0]);
    let home = session
        .committed_doc()
        .node_space(body)
        .expect("B is in a space");
    common::into_space(&mut session, post, home);
    session.pump();
    let out = session.perform(SessionOp::AddMirror {
        body: post.into(),
        plane: top_of(body),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    let doc = session.committed_doc();
    // The gesture writes `X ∪ mirror(X)`; the union is what is drawn.
    let mirror = minted_union(doc, &out.minted);
    assert_eq!(
        viewer::world::repoint_offer(doc, mirror, tol),
        None,
        "a mirror reading B only as its plane offers nothing for bracket"
    );
    let original = viewer::world::original_of(doc, post);
    assert_ne!(original, body, "the post's original is not B");
    assert_eq!(
        viewer::world::built_on_copy(doc, mirror),
        Some((post, original)),
        "the union is on the post's copy"
    );
    assert_eq!(
        viewer::frame::placement_offers(doc, mirror, tol)
            .into_iter()
            .map(|offer| offer.op)
            .collect::<Vec<_>>(),
        [SessionOp::Select(viewer::session::Selection::Node(
            original
        ))],
        "one row: on the post's copy, its button the post's original"
    );
}

/// **A separate part sketched on B's face offers nothing for B's
/// placements** (orchestrator, within Ev's PR 6915 rule): `bracket = B`
/// (a cylinder, whose wall a drawn frame traces) placed twice, a
/// profile drawn on B's top cap and extruded on its own
/// (no union with B) is another part: the extrude reads its profile,
/// which reads B's face as a frame, not B as a body.
///
/// *Breaks if* the offer's walk follows an extrude's profile seat or a
/// profile's frame.
#[test]
fn a_standalone_extrude_sketched_on_bs_face_offers_nothing_for_bracket() {
    let tol = Tol::witness();
    // B a disc's extrude (a frame traces a wall with an axis), placed
    // twice through `bracket`.
    let (mut session, body) = disc_extrude("sketched-part", tol);
    for name in [None, Some("bracket")] {
        let op = place_op(session.committed_doc(), body.into(), name);
        landed(&mut session, op);
    }
    let bracket = session
        .committed_doc()
        .var_named("bracket")
        .expect("the source");
    let profile = square_drawn_on(&mut session, body, body);
    let extrude = landed(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: common::len(0.003),
        },
    );
    let doc = session.committed_doc();
    assert!(
        matches!(doc.node(profile), Some(pncad::document::Node::Profile(p)) if p.on.is_some()),
        "the profile is drawn on B's face"
    );
    assert!(
        viewer::world::Picture::of(doc)
            .drawn
            .iter()
            .any(|drawn| drawn.of_node(extrude)),
        "the extrude is drawn"
    );
    let sources = viewer::world::repoint_offer(doc, extrude, tol)
        .map(|offer| offer.rows.iter().map(|row| row.source).collect::<Vec<_>>());
    assert!(
        !sources.iter().flatten().any(|&source| source == bracket),
        "{sources:?}"
    );
    assert_eq!(
        sources, None,
        "an extrude reading B only through its profile's frame offers nothing"
    );
    assert_eq!(
        viewer::frame::placement_offers(doc, extrude, tol),
        [],
        "and says nothing"
    );
}

/// A disc's extrude in a session of its own, landed: a body with a
/// planar cap and a wall with an axis, which a drawn frame traces.
fn disc_extrude(name: &str, tol: Tol) -> (DocSession, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived(name, tol), tol);
    let disc = landed(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![common::circle_at([0.0, 0.0], 0.01)],
        },
    );
    let body = landed(
        &mut session,
        SessionOp::AddExtrude {
            profile: disc,
            distance: common::len(0.01),
        },
    );
    (session, body)
}

/// A small square drawn on `body`'s top cap as seen on `on` (`body`
/// itself, or a copy of it), its frame tracing the wall: the profile.
fn square_drawn_on(session: &mut DocSession, body: RecipeNodeId, on: RecipeNodeId) -> RecipeNodeId {
    let wall = {
        let ev = session.evaluation().expect("landed");
        pncad::select::all_faces(ev, body)
            .into_iter()
            .find(|name| {
                pncad::select::face_carrier_kind(ev, body, name)
                    == Ok(pncad::prelude::SurfaceKind::Cylinder)
            })
            .expect("a wall")
    };
    let point =
        |x: f64, y: f64| pncad::document::ProgramTarget::Point([common::len(x), common::len(y)]);
    landed(
        session,
        SessionOp::AddDrawnProfile {
            plane: viewer::session::FaceSelection {
                node: on,
                ..top_of(body)
            },
            traced: vec![viewer::session::FaceSelection {
                name: wall,
                node: on,
                body: 0,
            }],
            chains: vec![
                pncad::trace::Chain::at(common::len(-0.002), common::len(-0.002))
                    .line_to(point(0.002, -0.002))
                    .line_to(point(0.002, 0.002))
                    .line_to(point(-0.002, 0.002))
                    .line_to(pncad::document::ProgramTarget::Start),
            ],
        },
    )
}

/// The one union among `minted`, drawn: the body a mirror gesture
/// (`X ∪ mirror(X)`) or a combine makes.
fn minted_union(doc: &Doc<ProfileProgram>, minted: &[RecipeNodeId]) -> RecipeNodeId {
    let unions: Vec<RecipeNodeId> = minted
        .iter()
        .copied()
        .filter(|&n| matches!(doc.node(n), Some(pncad::document::Node::Union { .. })))
        .collect();
    let [union] = unions[..] else {
        panic!("one union minted: {unions:?}");
    };
    assert!(
        viewer::world::Picture::of(doc)
            .drawn
            .iter()
            .any(|drawn| drawn.of_node(union)),
        "the union is drawn"
    );
    union
}

/// **A body read both as a plane and as a body still offers**: `X =
/// fillet(B)`, `A = chamfer(X)`, and a mirror of `A` whose plane is
/// picked on `X`. `A ∪ mirror(A)` reads `X` as the mirror's plane and,
/// through `A`, as the body `A` chamfers; through the second it reads
/// `B` as a body, so the union offers `bracket`.
///
/// *Breaks if* a node met through a plane is marked seen and never
/// walked as a body (the walk follows body seats alone).
#[test]
fn a_body_reached_first_as_a_plane_then_as_a_body_offers() {
    let tol = Tol::witness();
    let (mut session, body, _, _) = placed_twice(tol);
    let bracket = session
        .committed_doc()
        .var_named("bracket")
        .expect("the source");
    let edges = lateral_edges(&session, body);
    let x = landed(
        &mut session,
        SessionOp::AddFillet {
            target: body,
            radius: common::len(0.001),
            selection: vec![edges[0].clone()],
        },
    );
    let carried = pncad::select::all_edges(session.evaluation().expect("landed"), x)
        .into_iter()
        .find(|name| {
            matches!(name.path.as_slice(),
                [RoleSeg::From { of, .. }] if *of.name() == edges[1])
        })
        .expect("X carries a second lateral edge of B");
    let a = landed(
        &mut session,
        SessionOp::AddChamfer {
            target: x,
            distance: common::len(0.0005),
            selection: vec![carried],
        },
    );
    let out = session.perform(SessionOp::AddMirror {
        body: a.into(),
        plane: viewer::session::FaceSelection {
            node: x,
            ..top_of(body)
        },
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    let doc = session.committed_doc();
    let union = minted_union(doc, &out.minted);
    assert_eq!(
        viewer::world::repoint_offer(doc, union, tol).map(|offer| offer
            .rows
            .iter()
            .map(|row| row.source)
            .collect::<Vec<_>>()),
        Some(vec![bracket]),
        "A ∪ mirror(A) reads B as a body, through A and X"
    );
}

/// **A split half does not offer its sibling's placements**: `B` split,
/// its body 1 placed alone, then each half fused with another body. The
/// union of body 0 reads no placed body as a body and offers nothing;
/// the union of body 1 offers body 1's source.
///
/// *Breaks if* the walk takes every output of a node it reads rather
/// than the output read (a split's two halves are one node's two).
#[test]
fn a_split_half_does_not_offer_its_siblings_placements() {
    let tol = Tol::witness();
    let (mut session, body) = bracket("split-halves", tol);
    let raw = common::xy_box_in(&mut session, [0.004, 0.004, 0.004]);
    let other = common::moved_to(&mut session, raw, [0.0, 0.0, 0.0]);
    let plane = common::cut_across(&session, body, SIZE[2] / 2.0);
    let out = session.perform(SessionOp::AddSplit {
        target: body.into(),
        plane,
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    let split = *out
        .minted
        .iter()
        .find(|&&n| {
            matches!(
                session.committed_doc().node(n),
                Some(pncad::document::Node::Split { .. })
            )
        })
        .expect("the split");
    let half = |port| Operand::output(split, port);
    let placed = {
        let op = place_op(session.committed_doc(), half(1), None);
        landed(&mut session, op)
    };
    let source = session
        .committed_doc()
        .copied_read(placed)
        .expect("body 1's source");
    let home = session
        .committed_doc()
        .node_space(split)
        .expect("the split's space");
    common::into_space(&mut session, other, home);
    session.pump();
    let fused = |session: &mut DocSession, port| {
        let doc = session.committed_doc();
        let spec = viewer::session::BooleanSpec::Union(vec![
            pncad::document::BodyRead::plain(half(port)),
            pncad::document::BodyRead::plain(Operand::from(doc.output(other, 0).expect("other"))),
        ]);
        landed(session, SessionOp::AddBoolean { spec })
    };
    let zero = fused(&mut session, 0);
    let one = fused(&mut session, 1);
    let doc = session.committed_doc();
    let sources = |node| {
        viewer::world::repoint_offer(doc, node, tol)
            .map(|offer| offer.rows.iter().map(|row| row.source).collect::<Vec<_>>())
    };
    assert_eq!(sources(zero), None, "body 0's union offers nothing");
    assert_eq!(
        sources(one),
        Some(vec![source]),
        "body 1's union offers body 1's source"
    );
}

/// **A body that reaches a copy only through its frame is not on that
/// copy** (Ev, PR 6917: "built on a copy" reads bodies as bodies, as the
/// offer does): a cylinder's world copy `C`, a profile drawn on `C`'s
/// top cap and extruded on its own — another part. The extrude reads
/// `C` only through its profile's frame: it is on no placed copy, offers
/// nothing for `C`'s source, and the pane says nothing.
///
/// *Breaks if* "built on a copy" follows every seat.
#[test]
fn a_body_reaching_a_copy_only_through_its_frame_is_not_on_that_copy() {
    let tol = Tol::witness();
    let (session, body) = disc_extrude("framed-on-a-copy", tol);
    let (mut session, copy) = common::world_copy_by_hand_in(&session, body, tol);
    let source = session
        .committed_doc()
        .copied_read(copy)
        .expect("C's source");
    let profile = square_drawn_on(&mut session, body, copy);
    let extrude = landed(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: common::len(0.003),
        },
    );
    let doc = session.committed_doc();
    assert!(
        viewer::world::Picture::of(doc)
            .drawn
            .iter()
            .any(|drawn| drawn.of_node(extrude)),
        "the extrude is drawn"
    );
    assert_eq!(
        viewer::world::built_on_copy(doc, extrude),
        None,
        "the extrude is on no placed copy"
    );
    let sources = viewer::world::repoint_offer(doc, extrude, tol)
        .map(|offer| offer.rows.iter().map(|row| row.source).collect::<Vec<_>>());
    assert!(
        !sources.iter().flatten().any(|&s| s == source),
        "nothing for C's source: {sources:?}"
    );
    assert_eq!(
        viewer::frame::placement_offers(doc, extrude, tol),
        [],
        "and the pane says nothing"
    );
}

/// **A mirror whose plane is picked on a copy is not on that copy**
/// (Ev, PR 6917): world copies `C` of B and then `D` of X, and a mirror
/// of `D` whose plane is `C`'s top cap. `D ∪ mirror(D)` reads `D` as a
/// body and `C` only as its plane, so it is on `D`, not on `C` — though
/// `C` is minted first, which a walk over every seat would answer.
///
/// *Breaks if* "built on a copy" follows a mirror's plane seat.
#[test]
fn a_mirror_whose_plane_is_on_a_copy_is_not_on_that_copy() {
    let tol = Tol::witness();
    let (mut session, body) = bracket("mirror-on-a-copy", tol);
    let other = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let (session, c) = common::world_copy_by_hand_in(&session, body, tol);
    let (mut session, d) = common::world_copy_by_hand_in(&session, other, tol);
    assert!(c < d, "C is minted first");
    let out = session.perform(SessionOp::AddMirror {
        body: d.into(),
        plane: viewer::session::FaceSelection {
            node: c,
            ..top_of(body)
        },
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    let doc = session.committed_doc();
    let union = minted_union(doc, &out.minted);
    assert_eq!(
        viewer::world::built_on_copy(doc, union),
        Some((d, other)),
        "on D, the copy it reads as a body, not on C"
    );
    assert_eq!(
        viewer::frame::placement_offers(doc, union, tol)
            .into_iter()
            .map(|offer| offer.op)
            .collect::<Vec<_>>(),
        [SessionOp::Select(viewer::session::Selection::Node(other))],
        "one row, its button X"
    );
}
