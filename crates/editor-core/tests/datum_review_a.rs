//! Review rows for the datum node (PR 7382, review lane A): each tries
//! to break one claim the PR makes of `Node::Datum`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, len};
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CapEnd, Coord, DocEdit, EditError, Formula, MateFault, Node, NodeResult, Operand, ProfileDoc,
    RecipeNodeId, RoleSeg, TargetAxis, VarId,
};
use geom_core::Tol;

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

fn moves(xyz: [f64; 3]) -> Vec<(Coord<Operand>, Formula)> {
    vec![
        (Coord::slide(TargetAxis::X), len(xyz[0])),
        (Coord::slide(TargetAxis::Y), len(xyz[1])),
        (Coord::slide(TargetAxis::Z), len(xyz[2])),
    ]
}

fn datum_at(doc: ProfileDoc, space: VarId, xyz: [f64; 3]) -> (ProfileDoc, RecipeNodeId) {
    let bundle = editor_core::Placed::of(&doc, Vec::new(), moves(xyz)).bundle();
    fixture::insert(doc, Node::datum(Operand::Var(space), bundle))
}

fn origin(doc: &ProfileDoc, datum: RecipeNodeId) -> [f64; 3] {
    let ev = fixture::run(doc, &editor_core::EvalOptions::default());
    match ev.nodes.get(&datum) {
        Some(NodeResult::Ok(v)) => v.motion.map_or([0.0; 3], |m| m.origin().to_array()),
        other => panic!("the datum did not evaluate: {other:?}"),
    }
}

fn corners(doc: &ProfileDoc, node: RecipeNodeId) -> Vec<[f64; 3]> {
    let ev = fixture::run(doc, &editor_core::EvalOptions::default());
    let Some(NodeResult::Ok(v)) = ev.nodes.get(&node) else {
        panic!("node {node:?} did not build: {:?}", ev.nodes.get(&node))
    };
    let body = v.payload.output_body(0).expect("one body").body();
    let mut out: Vec<[f64; 3]> = body.points().map(|(_, p)| p.to_array()).collect();
    out.sort_by(|a, b| a.partial_cmp(b).unwrap());
    out
}

fn cap(block: RecipeNodeId, end: CapEnd) -> Operand {
    Operand::select(
        Operand::output(block, 0),
        vec![fixture::fname(block, RoleSeg::Cap(end))],
    )
}

fn set(
    doc: &ProfileDoc,
    placement: RecipeNodeId,
    constraints: Vec<editor_core::Constraint<Formula>>,
) -> Result<editor_core::Applied<editor_core::ProfileProgram>, EditError> {
    doc.apply(
        &DocEdit::SetConstraints {
            placement,
            constraints,
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
}

fn mate_fault(
    r: Result<editor_core::Applied<editor_core::ProfileProgram>, EditError>,
) -> MateFault {
    match r {
        Err(EditError::MateRefused { fault, .. }) => *fault,
        Err(other) => panic!("refused, but not as a mate: {other:?}"),
        Ok(_) => panic!("admitted"),
    }
}

/// Claim 1: a mirror through a datum's yz plane follows an edit of the
/// datum's value.
#[test]
fn review_a_mirror_follows_the_datums_value() {
    let (doc, block, space) = block("ra-mirror-edit");
    let (doc, datum) = datum_at(doc, space, [20.0, 0.0, 0.0]);
    let own = doc.output(datum, 0).unwrap();
    let (doc, mirror) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(block, 0).into(),
            plane: RoleRead::through(Operand::Var(own), PoseRole::Yz),
        },
    );
    let at = |doc: &ProfileDoc, c: f64| {
        let mut v: Vec<[f64; 3]> = corners(doc, block)
            .into_iter()
            .map(|[x, y, z]| [c - x, y, z])
            .collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v
    };
    assert_eq!(corners(&doc, mirror), at(&doc, 40.0));
    let bundle = editor_core::Placed::of(&doc, Vec::new(), moves([30.0, 0.0, 0.0])).bundle();
    let doc = set(&doc, datum, bundle)
        .expect("a datum's values re-written")
        .doc;
    assert_eq!(corners(&doc, mirror), at(&doc, 60.0), "the mirror followed");
}

/// Claim 1: values chart the target space's axes; the world's too.
#[test]
fn review_a_world_datum_values_chart_the_worlds_axes() {
    let (doc, _, _) = block("ra-world-values");
    let (doc, datum) = datum_at(doc, VarId::WORLD, [3.0, 4.0, 5.0]);
    assert_eq!(origin(&doc, datum), [3.0, 4.0, 5.0]);
}

/// Claim 1: a flush datum's free slides are values on its target's axes.
#[test]
fn review_a_flush_datum_slides_on_the_target_axes() {
    let (doc, block, space) = block("ra-flush-slides");
    let (doc, datum) = datum_at(doc, space, [0.0; 3]);
    let own = doc.output(datum, 0).unwrap();
    let mate = editor_core::Mate {
        on: RoleRead::through(Operand::Var(own), PoseRole::Xy),
        to: fixture::plane(cap(block, CapEnd::End)).into(),
        orient: editor_core::Orientation::IDENTITY,
    };
    let (doc, _) = fixture::mated(
        doc,
        datum,
        mate,
        &[
            (Coord::slide(TargetAxis::X), len(0.25)),
            (Coord::slide(TargetAxis::Y), len(-0.5)),
        ],
    );
    let o = origin(&doc, datum);
    assert_eq!(o, [0.25, -0.5, 1.0], "slides on the space's x and y");
}

/// Claim 1: an under-pinned datum bundle refuses.
#[test]
fn review_a_an_underpinned_datum_refuses() {
    let (doc, _, space) = block("ra-underpinned");
    let (doc, datum) = datum_at(doc, space, [0.0; 3]);
    let mut bundle = editor_core::stand().bundle();
    bundle.pop();
    let fault = mate_fault(set(&doc, datum, bundle));
    assert!(matches!(fault, MateFault::Unpinned { .. }), "{fault:?}");
    // And inserted so.
    let mut bundle = editor_core::stand().bundle();
    bundle.pop();
    let refused = doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(Node::datum(Operand::Var(space), bundle)),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(refused.is_err(), "an under-pinned datum inserted: admitted");
}

/// Claim 2: every near side but the datum's own refuses.
#[test]
fn review_a_near_sides_other_than_the_datums_own_refuse() {
    // Another datum's xy plane.
    let (doc, block, space) = block("ra-other-datum");
    let (doc, a) = datum_at(doc, space, [0.0; 3]);
    let (doc, b) = datum_at(doc, space, [0.0; 3]);
    let a_own = doc.output(a, 0).unwrap();
    let b_own = doc.output(b, 0).unwrap();
    let to = || -> RoleRead<Operand> { fixture::plane(cap(block, CapEnd::End)).into() };
    let try_on = |on: RoleRead<Operand>| {
        let mate = editor_core::Mate {
            on,
            to: to(),
            orient: editor_core::Orientation::IDENTITY,
        };
        let placed = editor_core::Placed::of(&doc, vec![mate], Vec::new());
        mate_fault(set(&doc, b, placed.bundle()))
    };
    let cases: Vec<(&str, RoleRead<Operand>)> = vec![
        (
            "another datum's xy",
            RoleRead::through(Operand::Var(a_own), PoseRole::Xy),
        ),
        (
            "a face of a body",
            fixture::plane(cap(block, CapEnd::Start)).into(),
        ),
        (
            "its own yz",
            RoleRead::through(Operand::Var(b_own), PoseRole::Yz),
        ),
        (
            "its own zx",
            RoleRead::through(Operand::Var(b_own), PoseRole::Zx),
        ),
        (
            "its own x",
            RoleRead::through(Operand::Var(b_own), PoseRole::X),
        ),
        (
            "its own xy, flipped",
            RoleRead::plain(Operand::Pose(Box::new(PoseDef::Flip {
                pose: RoleRead::through(Operand::Var(b_own), PoseRole::Xy),
            }))),
        ),
    ];
    for (what, on) in cases {
        let fault = try_on(on);
        assert!(
            matches!(fault, MateFault::NotTheDatumsOwn { .. }),
            "{what}: {fault:?}"
        );
    }
}

/// Claim 2: a frame mate of the datum's own frame, plainly, onto another
/// datum: B stands on A.
#[test]
fn review_a_a_datum_frame_mated_on_a_datum() {
    let (doc, _, space) = block("ra-frame-mate");
    let (doc, a) = datum_at(doc, space, [2.0, 3.0, 4.0]);
    let (doc, b) = datum_at(doc, space, [0.0; 3]);
    let a_own = doc.output(a, 0).unwrap();
    let b_own = doc.output(b, 0).unwrap();
    let mate = editor_core::Mate {
        on: RoleRead::plain(Operand::Var(b_own)),
        to: RoleRead::plain(Operand::Var(a_own)),
        orient: editor_core::Orientation::IDENTITY,
    };
    let (doc, _) = fixture::mated(doc, b, mate, &[]);
    assert_eq!(origin(&doc, b), [2.0, 3.0, 4.0]);
}

/// Claim 2: a datum mated onto another space's datum refuses as a
/// space mismatch.
#[test]
fn review_a_a_datum_mated_onto_another_spaces_datum_refuses() {
    let (doc, _, space) = block("ra-cross-space-mate");
    let (doc, w) = datum_at(doc, VarId::WORLD, [0.0; 3]);
    let (doc, b) = datum_at(doc, space, [0.0; 3]);
    let w_own = doc.output(w, 0).unwrap();
    let b_own = doc.output(b, 0).unwrap();
    let mate = editor_core::Mate {
        on: RoleRead::through(Operand::Var(b_own), PoseRole::Xy),
        to: RoleRead::through(Operand::Var(w_own), PoseRole::Xy),
        orient: editor_core::Orientation::IDENTITY,
    };
    let placed = editor_core::Placed::of(&doc, vec![mate], Vec::new());
    let r = set(&doc, b, placed.bundle());
    assert!(
        matches!(r, Err(EditError::SpaceMismatch { .. })),
        "{:?}",
        r.err()
    );
}

/// Claim 4: a far side read through a role its variable does not offer
/// (a face's plane read through `xy`) passes `side_kind` as a plane.
#[test]
fn review_a_a_far_side_through_a_role_its_variable_lacks() {
    let (doc, block, space) = block("ra-wrong-role");
    let (doc, datum) = datum_at(doc, space, [0.0; 3]);
    let own = doc.output(datum, 0).unwrap();
    for role in [PoseRole::Xy, PoseRole::Yz] {
        let mate = editor_core::Mate {
            on: RoleRead::through(Operand::Var(own), PoseRole::Xy),
            to: RoleRead::through(fixture::plane(cap(block, CapEnd::End)), role),
            orient: editor_core::Orientation::IDENTITY,
        };
        let placed = editor_core::Placed::of(&doc, vec![mate], Vec::new());
        let r = set(&doc, datum, placed.bundle());
        eprintln!("far side through {role:?}: {:?}", r.as_ref().err());
        assert!(r.is_err(), "a plane read through {role:?}: admitted");
    }
    // The near side: the datum's own frame through Origin, onto a face
    // plane read through Origin.
    let mate = editor_core::Mate {
        on: RoleRead::through(Operand::Var(own), PoseRole::Origin),
        to: RoleRead::through(fixture::plane(cap(block, CapEnd::End)), PoseRole::Origin),
        orient: editor_core::Orientation::IDENTITY,
    };
    let placed = editor_core::Placed::of(&doc, vec![mate], Vec::new());
    let r = set(&doc, datum, placed.bundle());
    eprintln!("origin of a plane: {:?}", r.as_ref().err());
    assert!(r.is_err(), "a plane read through its origin: admitted");
}

/// Claim 2 (to side): a datum's `to` reading its own frame is a cycle,
/// refused.
#[test]
fn review_a_a_datums_far_side_reading_itself_refuses() {
    let (doc, _, space) = block("ra-self-far");
    let (doc, datum) = datum_at(doc, space, [0.0; 3]);
    let own = doc.output(datum, 0).unwrap();
    let mate = editor_core::Mate {
        on: RoleRead::through(Operand::Var(own), PoseRole::Xy),
        to: RoleRead::through(Operand::Var(own), PoseRole::Yz),
        orient: editor_core::Orientation::IDENTITY,
    };
    let placed = editor_core::Placed::of(&doc, vec![mate], Vec::new());
    let r = set(&doc, datum, placed.bundle());
    eprintln!("self far side: {:?}", r.as_ref().err());
    assert!(r.is_err(), "a datum mated onto itself: admitted");
}

/// Q3 probe of the PR's own `a_copy_rests_on_a_datum_at_its_spaces_origin`:
/// its measure (`corners`, the body's own points) reads `0` whether or
/// not the mate moved anything. Here the datum stands at `z = 5`: the
/// copy's motion moves, the points the PR row reads do not.
#[test]
fn review_a_the_rest_on_a_datum_row_reads_points_the_mate_does_not_move() {
    let (doc, block, _) = block("ra-q3");
    let (doc, datum) = datum_at(doc, VarId::WORLD, [0.0, 0.0, 5.0]);
    let own = doc.output(datum, 0).unwrap();
    let (doc, copy) = fixture::insert(doc, fixture::stood(Operand::output(block, 0)));
    let start = cap(block, CapEnd::Start);
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
    assert_eq!(origin(&doc, copy)[2], 5.0, "the copy is moved to the datum");
    assert_eq!(lowest, 0.0, "the corners the PR row reads did not move");
}
