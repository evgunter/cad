//! **A takeover re-seats what read the copy it takes over**: a small
//! block's copy rested on a face of a block's drawn copy, then the
//! block filleted, mirrored or sketched on — each re-points the drawn
//! copy to its result, which carries the face under a name of its own.
//! The mate is re-seated on that name in the same action, so the small
//! block stays where it stood; a face the result consumes leaves its
//! mate stale; one undo restores the document.
//!
//! The block's copy stands off the identity, and every position is
//! read off the drawn copies.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::drawn;
use common::{along_x, displayed_face_at, len, plate_index};
use pncad::document::{
    Dimension, Doc, DocEdit, Formula, Node, Operand, ProfileProgram, RecipeNodeId,
};
use pncad::geom_core::Tol;
use pncad::prelude::{RoleSeg, StableName};
use viewer::session::{DocSession, FaceSelection, Selection, SessionOp};
use viewer::tools::{ToolKind, Tools};
use viewer::tree::RowStatus;

/// The block: 40 × 20 × 10 mm, centred on its sketch origin and
/// extruded up.
const BLOCK: [f64; 3] = [0.04, 0.02, 0.01];
/// The small block rested on it: 20 × 10 × 6 mm.
const SMALL: [f64; 3] = [0.02, 0.01, 0.006];
/// Where the block's drawn copy stands: off the identity on every axis.
const AT: [f64; 3] = [0.05, 0.03, 0.02];
/// How far along x of the block the small block's copy stands.
const BESIDE: f64 = 0.1;

/// The block, its drawn copy at [`AT`], the small block's copy, and a
/// landed session in which the small block's underside is rested on
/// `face` of the block's copy by the mate tool's flow.
fn seated(face: impl FnOnce(&DocSession) -> FaceSelection) -> Seated {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("takeover", tol), tol);
    let block = common::xy_box_in(&mut session, BLOCK);
    let small = common::xy_box_in(&mut session, SMALL);
    let doc = session.committed_doc().clone();
    let (copy, small_copy) = (common::copy_of(&doc, block), common::copy_of(&doc, small));
    let doc = drawn::pinned(doc, copy, &drawn::translated(AT));
    let doc = drawn::pinned(
        doc,
        small_copy,
        &drawn::translated([AT[0] + BESIDE, AT[1], 0.0]),
    );
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let index = plate_index(&session);
    let underside = displayed_face_at(&session, &index, &common::up_at(AT[0] + BESIDE, AT[1]));
    assert_eq!(underside.node, small_copy, "the small block's underside");
    let to = face(&session);
    assert_eq!(to.node, copy, "a face of the block's drawn copy");
    drawn::rest_on(&mut session, underside, to);
    Seated {
        session,
        block,
        copy,
        small_copy,
    }
}

struct Seated {
    session: DocSession,
    block: RecipeNodeId,
    copy: RecipeNodeId,
    small_copy: RecipeNodeId,
}

/// The block copy's top, picked from above.
fn top(session: &DocSession) -> FaceSelection {
    let index = plate_index(session);
    displayed_face_at(session, &index, &common::down_at(AT[0], AT[1]))
}

/// The block copy's `−x` wall, picked by a level ray travelling `+x`.
fn minus_x_wall(session: &DocSession) -> FaceSelection {
    let index = plate_index(session);
    displayed_face_at(
        session,
        &index,
        &along_x(1.0, AT[1], AT[2] + BLOCK[2] / 2.0),
    )
}

/// One vertical edge of `block`, by its name in the landed run.
fn a_lateral_edge(session: &DocSession, block: RecipeNodeId) -> StableName {
    let eval = session.evaluation().expect("landed");
    pncad::select::all_edges(eval, block)
        .into_iter()
        .find(|name| matches!(name.path.as_slice(), [RoleSeg::LateralEdge(_)]))
        .expect("a box has vertical edges")
}

/// The mirror tool's committed op across `plane`.
fn mirror_across(session: &DocSession, plane: &FaceSelection) -> SessionOp {
    let mut tools = Tools::new();
    tools.open(ToolKind::Mirror);
    let picked = tools.feed(
        session.committed_doc(),
        Err(viewer::pickcache::NotIndexed::Absent),
        &[SessionOp::Select(Selection::Face(plane.clone()))],
    );
    assert!(picked.is_empty(), "the pick is taken");
    tools
        .mirror()
        .expect("open")
        .op()
        .expect("a body and a plane are held")
}

/// A 4 mm square boss sketched on `face`, in a corner clear of nothing
/// in particular.
fn boss_on(face: FaceSelection) -> SessionOp {
    SessionOp::SketchOnFace {
        face,
        loops: vec![viewer::test_support::rectangle_loop(
            [0.01, 0.0],
            0.004,
            0.004,
        )],
        distance: len(0.003),
        spin: Formula::zero(Dimension::Angle),
    }
}

/// The ops this suite takes the block over by, each named.
fn takeovers(session: &DocSession, block: RecipeNodeId) -> Vec<(&'static str, SessionOp)> {
    vec![
        (
            "fillet",
            SessionOp::AddFillet {
                target: block,
                radius: len(0.002),
                selection: vec![a_lateral_edge(session, block)],
            },
        ),
        ("mirror", mirror_across(session, &minus_x_wall(session))),
        ("sketch on face", boss_on(top(session))),
    ]
}

/// The rebinds `edits` wrote.
fn rebinds(edits: &[DocEdit<ProfileProgram>]) -> Vec<&DocEdit<ProfileProgram>> {
    edits
        .iter()
        .filter(|edit| matches!(edit, DocEdit::Rebind { .. }))
        .collect()
}

/// **The small block rested on the block's top stays seated through a
/// fillet, a mirror and a sketch on the face**: each commit re-points
/// the block's drawn copy to its result and rebinds the mate's face to
/// the result's name for the top, so the small block is drawn where it
/// stood and every row is well. One undo restores the document. Goes
/// red if the takeover stops writing the rebind: the mate's face then
/// no longer resolves and the small block's placement fails.
#[test]
fn a_mate_on_the_taken_copys_face_stays_seated() {
    for at in 0..3 {
        let Seated {
            mut session,
            block,
            copy,
            small_copy,
        } = seated(top);
        let (name, op) = takeovers(&session, block).swap_remove(at);
        let before = session.committed_doc().clone();
        let seated_at = drawn::extent(&mut session, small_copy);
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{name}: {:?}", outcome.refusal);
        session.pump();
        assert_eq!(
            rebinds(&outcome.committed).len(),
            1,
            "{name}: one rebind, the mate's face: {:?}",
            outcome.committed
        );
        let moved = session.take_landed_reseats();
        assert!(
            moved
                .iter()
                .any(|m| m.0.get(&copy).is_some_and(|names| !names.is_empty())),
            "{name}: the copy's names moved, read off the landed run"
        );
        assert_eq!(
            drawn::extent(&mut session, small_copy),
            seated_at,
            "{name}: the small block stands where it stood"
        );
        for row in session.tree_rows() {
            assert_eq!(row.status, RowStatus::Ok, "{name}: {row:?}");
        }

        let undone = session.perform(SessionOp::Undo);
        assert!(undone.refusal.is_none(), "{name}: {:?}", undone.refusal);
        session.pump();
        assert!(
            *session.committed_doc() == before,
            "{name}: one undo restores the document"
        );
        assert_eq!(
            drawn::extent(&mut session, small_copy),
            seated_at,
            "{name}: and the small block with it"
        );
    }
}

/// **A face the result consumes leaves its mate stale**: the small
/// block rested on the block's `−x` wall, then the block mirrored across
/// that wall. The symmetric part glues its halves there, so the wall is
/// no face of the result: no rebind is written, and the small block's
/// placement fails in the result's own words. Goes red if the takeover
/// re-seats a name the result holds no carrier for.
#[test]
fn a_face_the_result_consumes_stays_stale() {
    let Seated {
        mut session,
        small_copy,
        ..
    } = seated(minus_x_wall);
    let wall = minus_x_wall(&session);
    let op = mirror_across(&session, &wall);
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert!(
        rebinds(&outcome.committed).is_empty(),
        "nothing carries the wall: {:?}",
        outcome.committed
    );
    let eval = session.evaluation().expect("landed");
    let failed = format!("{:?}", eval.node_error(small_copy));
    assert!(
        failed.contains("Vanished") && failed.contains(&format!("{:?}", wall.name)),
        "the small block's mate reads the wall, which vanished: {failed}"
    );
}

/// **A face of `C ∪ D` read through the copy `C` is re-seated too**: the
/// block's drawn copy and a second copy of it, apart, unioned, and the
/// small block rested on the union's face that is the first copy's top.
/// The fillet takes both copies over; the union's face for the first
/// copy's top is rebound to its name under the result, so the small
/// block stays seated. Goes red if the takeover re-seats only the
/// selections of the copy itself.
#[test]
fn a_face_of_a_union_of_the_copy_is_re_seated() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("takeover", tol), tol);
    let block = common::xy_box_in(&mut session, BLOCK);
    let small = common::xy_box_in(&mut session, SMALL);
    let doc = session.committed_doc().clone();
    let (copy, small_copy) = (common::copy_of(&doc, block), common::copy_of(&doc, small));
    let doc = drawn::pinned(doc, copy, &drawn::translated(AT));
    let (doc, other) = viewer::test_support::placed(&doc, block, tol);
    let doc = drawn::pinned(
        doc,
        other,
        &drawn::translated([AT[0], AT[1] + 2.0 * BLOCK[1], AT[2]]),
    );
    let doc = drawn::pinned(
        doc,
        small_copy,
        &drawn::translated([AT[0] + BESIDE, AT[1], 0.0]),
    );
    let read = |node: RecipeNodeId| doc.output(node, 0).expect("a copy defines its body");
    let (c, d) = (read(copy), read(other));
    let (doc, both) = viewer::test_support::inserted(
        &doc,
        Node::Union {
            members: pncad::document::Bodies::Spelled(vec![
                Operand::Var(c).into(),
                Operand::Var(d).into(),
            ]),
        },
        tol,
    );
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let doc = session.committed_doc().clone();
    let u = doc.output(both, 0).expect("the union defines its body");
    let block_top = top(&session).name;
    let face = {
        let eval = session.evaluation().expect("landed");
        eval.table(u)
            .expect("the union is named")
            .iter()
            .map(|(name, _)| name.clone())
            .find(|name| {
                matches!(name.path.as_slice(),
                    [RoleSeg::From { read, of }] if *read == c && *of.name() == block_top)
            })
            .expect("the union carries the first copy's top")
    };
    let underside = Operand::select(
        Operand::Node(small),
        vec![StableName {
            path: vec![RoleSeg::Cap(pncad::prelude::CapEnd::Start)],
            node: small,
            kind: block_top.kind,
        }],
    );
    common::commit_mate(
        &mut session,
        SessionOp::AddMate {
            placement: small_copy,
            mate: Box::new(crate::fixture::rest(
                underside,
                Operand::select(Operand::Var(u), vec![face]),
            )),
            values: Vec::new(),
            into_target_space: false,
        },
    );
    let seated_at = drawn::extent(&mut session, small_copy);
    let lateral = a_lateral_edge(&session, block);
    let outcome = session.perform(SessionOp::AddFillet {
        target: block,
        radius: len(0.002),
        selection: vec![lateral],
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let rebound: Vec<_> = rebinds(&outcome.committed)
        .into_iter()
        .filter(|edit| matches!(edit, DocEdit::Rebind { body: Some(body), .. } if *body == u))
        .collect();
    assert_eq!(
        rebound.len(),
        1,
        "the union's face: {:?}",
        outcome.committed
    );
    assert_eq!(
        drawn::extent(&mut session, small_copy),
        seated_at,
        "the small block stands where it stood"
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
}

/// **The mirror's merged top keeps the old top's frame**, which is why
/// the small block mated on it stands where it stood: the symmetric
/// part is `Union[body, image]` (`DocSession::add_mirror`), and a union
/// keeps its FIRST member's carrier for a face it merges, so the merged
/// top's frame is the body's top's. The order is load-bearing: a part
/// spelled `Union[image, body]` would frame the top by the image's,
/// mirrored, and the mate would turn the block over. Goes red if the
/// mirror's union is written image first, or the union stops keeping
/// its first member's carrier.
#[test]
fn the_mirrors_merged_top_keeps_the_old_tops_frame() {
    let Seated {
        mut session, copy, ..
    } = seated(top);
    let held = top(&session);
    let var = session.committed_doc().output(copy, 0).expect("copy body");
    let before = {
        let eval = session.evaluation().expect("landed");
        pncad::select::face_frame(eval, var, &held.name).expect("frame before")
    };
    let op = mirror_across(&session, &minus_x_wall(&session));
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let moved = session.take_landed_reseats();
    let now = moved
        .iter()
        .find_map(|m| m.reseat(copy, &held.name))
        .expect("the top moved to the merged top")
        .clone();
    assert!(
        matches!(now.path.as_slice(), [RoleSeg::Merged(set)] if set.len() == 2),
        "the top merged across the glue plane: {now:?}"
    );
    let after = {
        let eval = session.evaluation().expect("landed");
        pncad::select::face_frame(eval, var, &now).expect("frame after")
    };
    // Compared exactly, through `Debug`'s shortest round-trip spelling
    // of each number (the frame types carry no `PartialEq`): the same
    // carrier, not one close to it.
    assert_eq!(
        format!("{after:?}"),
        format!("{before:?}"),
        "the merged top is framed as the body's top was"
    );
}
