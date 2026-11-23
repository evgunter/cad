//! **A datum is a placed frame** (D10 `[placement.datums]`): a
//! placement whose source is nothing places a bare frame into its
//! target space by the placement bundle; its mates' `on` sides are the
//! frame's own roles, and a reader takes a plane or an axis off it
//! through a role, only in its own space.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, len};
use editor_core::pose::{PoseRole, RoleRead};
use editor_core::{
    CapEnd, Coord, DocEdit, EditError, Formula, MateFault, Node, NodeResult, Operand, ProfileDoc,
    RecipeNodeId, RoleSeg, SlotId, SlotValue, TargetAxis, VarId,
};
use geom_core::Tol;

/// A `2 × 2` square extruded `1` on its own xy plane, and its space.
fn block(label: &str) -> (ProfileDoc, RecipeNodeId, VarId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived(label, Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(1.0, 1.0, 1.0)],
    );
    let (doc, block) = fixture::insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        },
    );
    let space = doc.output(block, 1).expect("an extrude defines its space");
    (doc, block, space)
}

/// A datum in `space` slid `x` along the space's `x`, every other value
/// zero.
fn datum_at_x(doc: ProfileDoc, space: VarId, x: f64) -> (ProfileDoc, RecipeNodeId) {
    let bundle = editor_core::Placed::of(
        &doc,
        Vec::new(),
        vec![(Coord::slide(TargetAxis::X), len(x))],
    )
    .bundle();
    fixture::insert(doc, Node::datum(Operand::Var(space), bundle))
}

/// A datum in `space` slid `xyz` along the space's axes, every spin zero.
fn datum_at(doc: ProfileDoc, space: VarId, xyz: [f64; 3]) -> (ProfileDoc, RecipeNodeId) {
    let moves = [TargetAxis::X, TargetAxis::Y, TargetAxis::Z]
        .into_iter()
        .zip(xyz)
        .map(|(axis, by)| (Coord::slide(axis), len(by)))
        .collect();
    let bundle = editor_core::Placed::of(&doc, Vec::new(), moves).bundle();
    fixture::insert(doc, Node::datum(Operand::Var(space), bundle))
}

/// The frame the datum `datum` placed: its origin and its `z`.
fn frame(doc: &ProfileDoc, datum: RecipeNodeId) -> ([f64; 3], [f64; 3]) {
    let ev = fixture::run(doc, &editor_core::EvalOptions::default());
    match ev.nodes.get(&datum).map(|r| match r {
        NodeResult::Ok(v) => Some(&v.payload),
        _ => None,
    }) {
        Some(Some(editor_core::ValuePayload::Datum(m))) => {
            (m.origin().to_array(), m.w().get().to_array())
        }
        other => panic!("the datum did not evaluate: {other:?}"),
    }
}

/// Every corner of `node`'s body, sorted, as it stands in its space
/// ([`fixture::in_space`]).
fn corners(doc: &ProfileDoc, node: RecipeNodeId) -> Vec<[f64; 3]> {
    let ev = fixture::run(doc, &editor_core::EvalOptions::default());
    let Some(NodeResult::Ok(v)) = ev.nodes.get(&node) else {
        panic!("node {node:?} did not build")
    };
    let body = fixture::in_space(v.payload.output_body(0).expect("one body"));
    let mut out: Vec<[f64; 3]> = body.points().map(|(_, p)| p.to_array()).collect();
    out.sort_by(|a, b| a.partial_cmp(b).unwrap());
    out
}

/// **A mirror across `x = 20` of a body's space**, read off a datum
/// slid `20` along that space's `x` through its yz plane: every corner
/// of the image is the block's at `40 − x`.
#[test]
fn a_mirror_across_a_datums_yz_plane_reflects_about_its_x() {
    let (doc, block, space) = block("datum-mirror");
    let (doc, datum) = datum_at_x(doc, space, 20.0);
    let frame_var = doc.output(datum, 0).expect("a datum defines its frame");
    let (doc, mirror) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(block, 0).into(),
            plane: RoleRead::through(Operand::Var(frame_var), PoseRole::Yz),
        },
    );
    let mut reflected: Vec<[f64; 3]> = corners(&doc, block)
        .into_iter()
        .map(|[x, y, z]| [40.0 - x, y, z])
        .collect();
    reflected.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(corners(&doc, mirror), reflected, "the image's corners");
}

/// **A datum mated flush on a face moves with the face**: its xy plane
/// rests on the block's end cap, so lengthening the block carries the
/// datum's origin up with the cap, its `z` the cap's outward normal.
#[test]
fn a_datum_flush_on_a_face_follows_the_face() {
    let (doc, block, space) = block("datum-flush");
    let (doc, datum) = datum_at_x(doc, space, 0.0);
    let own = doc.output(datum, 0).expect("a datum defines its frame");
    let cap = Operand::select(
        Operand::output(block, 0),
        vec![fixture::fname(block, RoleSeg::Cap(CapEnd::End))],
    );
    let mate = editor_core::Mate {
        on: RoleRead::through(Operand::Var(own), PoseRole::Xy),
        to: fixture::plane(cap).into(),
        orient: editor_core::Orientation::IDENTITY,
    };
    let (doc, _) = fixture::mated(doc, datum, mate, &[]);
    let (origin, z) = frame(&doc, datum);
    assert_eq!(
        z,
        [0.0, 0.0, 1.0],
        "the datum's z is the cap's outward normal"
    );
    assert_eq!(origin[2], 1.0, "the datum stands on the cap");
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetParam {
            node: block,
            slot: SlotId::Distance,
            value: SlotValue::Formula(len(3.0)),
            fresh: Vec::new(),
        },
    );
    let (origin, _) = frame(&doc, datum);
    assert_eq!(origin[2], 3.0, "the datum moved with the cap");
}

/// **A mate's far side reads a datum through its xy plane**: a copy of
/// the block rests its start cap on a world datum raised `3` up the
/// world's `z`, so the copy's lowest corner stands at `3` in the world.
/// The datum at all-zero values is the review rows'.
#[test]
fn a_copy_rests_on_a_datums_xy_plane() {
    let (doc, block, _) = block("datum-far-side");
    let (doc, datum) = datum_at(doc, VarId::WORLD, [0.5, -0.25, 3.0]);
    let own = doc.output(datum, 0).expect("a datum defines its frame");
    let (doc, copy) = fixture::insert(doc, fixture::stood(Operand::output(block, 0)));
    let start = Operand::select(
        Operand::output(block, 0),
        vec![fixture::fname(block, RoleSeg::Cap(CapEnd::Start))],
    );
    let mate = editor_core::Mate {
        on: fixture::flip(fixture::plane(start)).into(),
        to: RoleRead::through(Operand::Var(own), PoseRole::Xy),
        orient: editor_core::Orientation::IDENTITY,
    };
    let (doc, _) = fixture::mated(doc, copy, mate, &[]);
    let lowest = corners(&doc, copy)
        .into_iter()
        .map(|[_, _, z]| z)
        .fold(f64::INFINITY, f64::min);
    assert_eq!(lowest, 3.0, "the copy stands on the datum's xy plane");
}

/// **A datum read from outside its space refuses**: a mirror of a body
/// in its own space cannot read a datum in the world.
#[test]
fn a_datum_read_outside_its_space_refuses() {
    let (doc, block, _) = block("datum-other-space");
    let (doc, datum) = datum_at_x(doc, VarId::WORLD, 20.0);
    let frame_var = doc.output(datum, 0).expect("a datum defines its frame");
    let refused = doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(Node::Mirror {
                body: Operand::output(block, 0).into(),
                plane: RoleRead::through(Operand::Var(frame_var), PoseRole::Yz),
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(
        matches!(refused, Err(EditError::SpaceMismatch { .. })),
        "a mirror reading a world datum refuses: {refused:?}"
    );
}

/// **A datum's mate reads the datum's own frame**: an `on` read through
/// a role other than its xy plane, z axis or origin refuses, naming the
/// mate.
#[test]
fn a_datums_mate_reads_its_own_xy_z_or_origin() {
    let (doc, block, space) = block("datum-own");
    let (doc, datum) = datum_at_x(doc, space, 0.0);
    let own = doc.output(datum, 0).expect("a datum defines its frame");
    let cap = Operand::select(
        Operand::output(block, 0),
        vec![fixture::fname(block, RoleSeg::Cap(CapEnd::End))],
    );
    let own_plane = || RoleRead::through(Operand::Var(own), PoseRole::Xy);
    let flipped_own = Operand::Pose(Box::new(editor_core::pose::PoseDef::Flip {
        pose: own_plane(),
    }));
    let sides: [(&str, RoleRead<Operand>); 4] = [
        (
            "its yz plane",
            RoleRead::through(Operand::Var(own), PoseRole::Yz),
        ),
        (
            "its zx plane",
            RoleRead::through(Operand::Var(own), PoseRole::Zx),
        ),
        (
            "its x axis",
            RoleRead::through(Operand::Var(own), PoseRole::X),
        ),
        ("its xy plane flipped", RoleRead::plain(flipped_own)),
    ];
    for (said, on) in sides {
        let mate = editor_core::Mate {
            on,
            to: fixture::plane(cap.clone()).into(),
            orient: editor_core::Orientation::IDENTITY,
        };
        let placed =
            editor_core::Placed::of(&doc, vec![mate], Vec::<(Coord<Operand>, Formula)>::new());
        let refused = doc.apply(
            &DocEdit::SetConstraints {
                placement: datum,
                constraints: placed.bundle(),
                fresh: Vec::new(),
            },
            Tol::witness(),
        );
        match refused {
            Err(EditError::MateRefused { fault, .. }) => assert!(
                matches!(*fault, MateFault::NotTheDatumsOwn { .. }),
                "{said}: the refusal is the datum's own side: {fault:?}"
            ),
            other => panic!("{said}: a datum mate off its own side refuses: {other:?}"),
        }
    }
}
