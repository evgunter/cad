//! **Review B of the datum (PR 7382)**: rows that try to break the
//! datum's readers, its persistence and its naming. Each row states
//! the claim it falsifies.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, len};
use editor_core::pose::{PoseRole, RoleRead};
use editor_core::{
    CapEnd, Coord, DocEdit, DrawnOn, EditError, Maintenance, MateFault, Node, NodeErrorKind,
    NodeResult, Operand, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, SplitSide, TargetAxis,
    ValuePayload, VarId,
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

fn bundle_at(
    doc: &ProfileDoc,
    axis: TargetAxis,
    by: f64,
) -> Vec<editor_core::Constraint<editor_core::Formula>> {
    editor_core::Placed::of(doc, Vec::new(), vec![(Coord::slide(axis), len(by))]).bundle()
}

fn datum_at(
    doc: ProfileDoc,
    space: VarId,
    axis: TargetAxis,
    by: f64,
) -> (ProfileDoc, RecipeNodeId, VarId) {
    let bundle = bundle_at(&doc, axis, by);
    let (doc, datum) = fixture::insert(doc, Node::datum(Operand::Var(space), bundle));
    let frame = doc.output(datum, 0).expect("a datum defines its frame");
    (doc, datum, frame)
}

fn corners_of(body: &editor_core::Framed<f64>) -> Vec<[f64; 3]> {
    // In the space the value stands in, not the body's own coordinates.
    let placed = fixture::in_space(body);
    let mut out: Vec<[f64; 3]> = placed.points().map(|(_, p)| p.to_array()).collect();
    out.sort_by(|a, b| a.partial_cmp(b).unwrap());
    out.dedup();
    out
}

fn corners(doc: &ProfileDoc, node: RecipeNodeId) -> Vec<[f64; 3]> {
    let ev = fixture::run(doc, &editor_core::EvalOptions::default());
    let Some(NodeResult::Ok(v)) = ev.nodes.get(&node) else {
        panic!("node {node:?} did not build: {:?}", ev.nodes.get(&node))
    };
    corners_of(v.payload.output_body(0).expect("one body"))
}

/// Claim 2, the split: a split of the block by a datum's xy plane slid
/// `0.25` up its space's `z` cuts it there, both halves present.
#[test]
fn review_b_a_split_reads_a_datums_xy_plane() {
    let (doc, block, space) = block("rb-split");
    let (doc, _, frame) = datum_at(doc, space, TargetAxis::Z, 0.25);
    let (doc, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(block, 0).into(),
            tool: RoleRead::through(Operand::Var(frame), PoseRole::Xy),
        },
    );
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    let Some(NodeResult::Ok(v)) = ev.nodes.get(&split) else {
        panic!("the split did not build: {:?}", ev.nodes.get(&split))
    };
    let ValuePayload::Split {
        above: SplitSide::Body(above),
        below: SplitSide::Body(below),
    } = &v.payload
    else {
        panic!("both halves: {:?}", v.payload.kind_name())
    };
    let top = |b| {
        corners_of(b)
            .iter()
            .map(|p| p[2])
            .fold(f64::NEG_INFINITY, f64::max)
    };
    let bot = |b| {
        corners_of(b)
            .iter()
            .map(|p| p[2])
            .fold(f64::INFINITY, f64::min)
    };
    assert_eq!((bot(above), top(above)), (0.25, 1.0), "above the datum");
    assert_eq!((bot(below), top(below)), (0.0, 0.25), "below the datum");
}

/// Claim 2, wrong-kind reads: a split's tool and a mirror's plane
/// through a datum's z axis or its origin refuse at the door, typed.
#[test]
fn review_b_a_plane_seat_refuses_a_datums_axis_or_origin() {
    let (doc, block, space) = block("rb-kinds");
    let (doc, _, frame) = datum_at(doc, space, TargetAxis::Z, 0.25);
    for role in [PoseRole::Z, PoseRole::X, PoseRole::Origin] {
        for node in [
            Node::Split {
                target: Operand::output(block, 0).into(),
                tool: RoleRead::through(Operand::Var(frame), role),
            },
            Node::Mirror {
                body: Operand::output(block, 0).into(),
                plane: RoleRead::through(Operand::Var(frame), role),
            },
        ] {
            let refused = fixture::insert_refused(&doc, node);
            assert!(
                matches!(refused, EditError::SlotVarKind { .. }),
                "{role:?} into a plane seat refuses as a kind: {refused:?}"
            );
        }
    }
}

/// Claim 2, a mirror plainly reading a datum's frame (no role) refuses
/// at the door as a frame where a plane is asked: a frame read into a
/// plane seat names its role.
#[test]
fn review_b_a_mirror_reading_a_datum_plainly() {
    let (doc, block, space) = block("rb-plain");
    let (doc, _, frame) = datum_at(doc, space, TargetAxis::Z, 3.0);
    let node = Node::Mirror {
        body: Operand::output(block, 0).into(),
        plane: RoleRead::plain(Operand::Var(frame)),
    };
    match doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Err(EditError::SlotVarKind { found, .. }) => {
            assert_eq!(found, editor_core::VarKind::Frame, "found a frame");
        }
        other => panic!(
            "a plain frame read into a plane seat refuses: {:?}",
            other.map(|_| ())
        ),
    }
}

/// Claim 2, a mate's far side through each plane role: a copy's start
/// cap rests on a world datum's zx plane slid `5` along `y`; the copy's
/// cap lands at `y = 5`. And a plane `on` against the datum's `z` axis
/// refuses typed as kinds that differ.
#[test]
fn review_b_a_mates_far_side_reads_a_datums_zx_and_refuses_its_axis() {
    let (doc, block, _) = block("rb-far");
    let (doc, _, frame) = datum_at(doc, VarId::WORLD, TargetAxis::Y, 5.0);
    let (doc, copy) = fixture::insert(doc, fixture::stood(Operand::output(block, 0)));
    let start = Operand::select(
        Operand::output(block, 0),
        vec![fixture::fname(block, RoleSeg::Cap(CapEnd::Start))],
    );
    let wrong = editor_core::Mate {
        on: fixture::plane(start.clone()).into(),
        to: RoleRead::through(Operand::Var(frame), PoseRole::Z),
        orient: editor_core::Orientation::IDENTITY,
    };
    let placed = editor_core::Placed::of(
        &doc,
        vec![wrong],
        Vec::<(Coord<Operand>, editor_core::Formula)>::new(),
    );
    match doc.apply(
        &DocEdit::SetConstraints {
            placement: copy,
            constraints: placed.bundle(),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Err(EditError::MateRefused { fault, .. }) => assert!(
            matches!(*fault, MateFault::KindsDiffer { .. }),
            "a plane against an axis: {fault:?}"
        ),
        other => panic!("a plane mated to a datum's axis refuses: {other:?}"),
    }
    let mate = editor_core::Mate {
        on: fixture::flip(fixture::plane(start)).into(),
        to: RoleRead::through(Operand::Var(frame), PoseRole::Zx),
        orient: editor_core::Orientation::IDENTITY,
    };
    let (doc, _) = fixture::mated(doc, copy, mate, &[]);
    let ys: Vec<f64> = corners(&doc, copy).into_iter().map(|p| p[1]).collect();
    let lo = ys.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    assert!(
        (lo - 5.0).abs() < 1e-12 || (hi - 5.0).abs() < 1e-12,
        "the copy's start cap on y = 5: {lo}..{hi}"
    );
}

/// Claim 2, a profile drawn on a datum's frame: the datum slid `2` up
/// its space's `z`, a square drawn on it extruded `1` stands on
/// `z ∈ [2, 3]`.
#[test]
fn review_b_a_profile_drawn_on_a_datums_frame_stands_there() {
    let (doc, _, space) = block("rb-drawn");
    let (doc, _, frame) = datum_at(doc, space, TargetAxis::Z, 2.0);
    let program = ProfileProgram {
        on: Some(DrawnOn {
            frame: Operand::Var(frame),
            faces: Vec::new(),
        }),
        ..fixture::desc(vec![fixture::square(1.0, 1.0, 1.0)])
    };
    let (doc, p) = fixture::insert(doc, Node::Profile(program));
    let (doc, boss) = fixture::insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        },
    );
    let zs: Vec<f64> = corners(&doc, boss).into_iter().map(|p| p[2]).collect();
    let lo = zs.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = zs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    assert_eq!((lo, hi), (2.0, 3.0), "the boss stands on the datum");
}

/// Claim 3, persistence: a document holding a datum (and a mirror and
/// a mate reading it) saves and loads equal, and saves again to the
/// same bytes.
#[test]
fn review_b_a_datum_saves_and_loads_equal() {
    let (doc, block, space) = block("rb-persist");
    let (doc, datum, frame) = datum_at(doc, space, TargetAxis::X, 20.0);
    let cap = Operand::select(
        Operand::output(block, 0),
        vec![fixture::fname(block, RoleSeg::Cap(CapEnd::End))],
    );
    let mate = editor_core::Mate {
        on: RoleRead::through(Operand::Var(frame), PoseRole::Xy),
        to: fixture::plane(cap).into(),
        orient: editor_core::Orientation::IDENTITY,
    };
    let (doc, _) = fixture::mated(doc, datum, mate, &[]);
    let (doc, _) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(block, 0).into(),
            plane: RoleRead::through(Operand::Var(frame), PoseRole::Yz),
        },
    );
    let text = editor_core::save(&doc, Tol::witness()).unwrap();
    let back = editor_core::load(&text, Tol::witness()).unwrap();
    assert!(back == doc, "the loaded document is the saved one");
    assert_eq!(
        editor_core::save(&back, Tol::witness()).unwrap(),
        text,
        "same bytes"
    );
}

/// Claim 3, naming: the datum's output variable is the same id after
/// its value is re-written, and the mirror reading it follows the move.
#[test]
fn review_b_a_datums_output_name_is_stable_under_value_edits() {
    let (doc, block, space) = block("rb-stable");
    let (doc, datum, frame) = datum_at(doc, space, TargetAxis::X, 20.0);
    let (doc, mirror) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(block, 0).into(),
            plane: RoleRead::through(Operand::Var(frame), PoseRole::Yz),
        },
    );
    let before = corners(&doc, mirror);
    let bundle = bundle_at(&doc, TargetAxis::X, 10.0);
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetConstraints {
            placement: datum,
            constraints: bundle,
            fresh: Vec::new(),
        },
    );
    assert_eq!(doc.output(datum, 0), Some(frame), "the frame keeps its id");
    let after = corners(&doc, mirror);
    assert_ne!(before, after, "the mirror moved");
    let mut want: Vec<[f64; 3]> = corners(&doc, block)
        .into_iter()
        .map(|[x, y, z]| [20.0 - x, y, z])
        .collect();
    want.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(after, want, "across x = 10");
}

/// Claim 3, delete: deleting a datum a mirror reads is not refused; it
/// reports the mirror's read stranded, and the mirror then refuses
/// typed as an unresolved read.
#[test]
fn review_b_deleting_a_datum_strands_its_mirror() {
    let (doc, block, space) = block("rb-delete");
    let (doc, datum, frame) = datum_at(doc, space, TargetAxis::X, 20.0);
    let (doc, mirror) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(block, 0).into(),
            plane: RoleRead::through(Operand::Var(frame), PoseRole::Yz),
        },
    );
    let applied = doc
        .apply(&DocEdit::DeleteNode { id: datum }, Tol::witness())
        .expect("a delete is never refused");
    assert!(
        applied
            .maintenance
            .iter()
            .any(|m| matches!(m, Maintenance::StrandedRead { .. })),
        "the mirror's read is reported stranded: {:?}",
        applied.maintenance
    );
    let ev = fixture::run(&applied.doc, &editor_core::EvalOptions::default());
    match ev.nodes.get(&mirror) {
        Some(NodeResult::Failed(e)) => assert!(
            matches!(e.kind, NodeErrorKind::UnresolvedRead { .. }),
            "unresolved: {e:?}"
        ),
        other => panic!("the mirror refuses once its datum is gone: {other:?}"),
    }
}

/// A datum's mate whose `on` is ANOTHER datum's frame refuses as not
/// the datum's own.
#[test]
fn review_b_a_datums_mate_on_another_datum_refuses() {
    let (doc, _, space) = block("rb-other");
    let (doc, a, _) = datum_at(doc, space, TargetAxis::X, 0.0);
    let (doc, _, b) = datum_at(doc, space, TargetAxis::X, 4.0);
    let (doc, _, c) = datum_at(doc, space, TargetAxis::Y, 4.0);
    let mate = editor_core::Mate {
        on: RoleRead::through(Operand::Var(b), PoseRole::Xy),
        to: RoleRead::through(Operand::Var(c), PoseRole::Xy),
        orient: editor_core::Orientation::IDENTITY,
    };
    let placed = editor_core::Placed::of(
        &doc,
        vec![mate],
        Vec::<(Coord<Operand>, editor_core::Formula)>::new(),
    );
    match doc.apply(
        &DocEdit::SetConstraints {
            placement: a,
            constraints: placed.bundle(),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Err(EditError::MateRefused { fault, .. }) => assert!(
            matches!(*fault, MateFault::NotTheDatumsOwn { .. }),
            "{fault:?}"
        ),
        other => panic!("another datum's frame is not this one's own: {other:?}"),
    }
}

/// A datum's mate whose `to` reads its own frame: refused (a cycle, or
/// not its own far side), never accepted.
#[test]
fn review_b_a_datums_mate_to_its_own_frame_refuses() {
    let (doc, _, space) = block("rb-self");
    let (doc, a, own) = datum_at(doc, space, TargetAxis::X, 0.0);
    let mate = editor_core::Mate {
        on: RoleRead::through(Operand::Var(own), PoseRole::Xy),
        to: RoleRead::through(Operand::Var(own), PoseRole::Xy),
        orient: editor_core::Orientation::IDENTITY,
    };
    let placed = editor_core::Placed::of(
        &doc,
        vec![mate],
        Vec::<(Coord<Operand>, editor_core::Formula)>::new(),
    );
    let refused = doc.apply(
        &DocEdit::SetConstraints {
            placement: a,
            constraints: placed.bundle(),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(
        refused.is_err(),
        "a datum mated to itself refuses: {:?}",
        refused.map(|_| ())
    );
}

/// The proof of the face–face row of `union` where two caps rest
/// opposed, at the door's first rung.
fn rest_row_proof(doc: &ProfileDoc, union: RecipeNodeId) -> editor_core::Proof {
    let ev = fixture::run(doc, &editor_core::EvalOptions::default());
    let rows = ev
        .value(union)
        .unwrap_or_else(|| panic!("the union: {:?}", ev.nodes.get(&union)))
        .coincidences
        .to_vec();
    let row = rows
        .iter()
        .find(|r| {
            r.relation == topo::Relation::SameOpposite
                && r.cells.iter().all(|c| {
                    matches!(c, editor_core::NamedCell::Entity { name, .. }
                        if name.kind == editor_core::EntityKind::Face)
                })
        })
        .unwrap_or_else(|| panic!("the rested caps' row: {rows:?}"))
        .clone();
    editor_core::coincide::prove(
        doc,
        &[(union, &[row][..])],
        ev.resolver(),
        Tol::witness(),
        &editor_core::Stop::default(),
    )
    .expect("not canceled")[0][0]
        .clone()
}

/// Two copies of the block in the world, the second's start cap resting
/// on the first's end cap: directly, or through a datum flush on the
/// first's end cap. `[coincidence.structural]`: "being one construction
/// is the equivalence the document's constructions and mates
/// generate", and a datum's mates are mates, so the rested caps are one
/// construction by the mates either way.
fn stacked(through_a_datum: bool) -> (ProfileDoc, RecipeNodeId) {
    let (doc, block, _) = block(if through_a_datum {
        "rb-st-datum"
    } else {
        "rb-st-plain"
    });
    let (doc, p0) = fixture::insert(doc, fixture::stood(Operand::output(block, 0)));
    let (doc, p1) = fixture::insert(doc, fixture::stood(Operand::output(block, 0)));
    let cap = |of: RecipeNodeId, end| {
        Operand::select(
            Operand::output(of, 0),
            vec![fixture::fname(block, RoleSeg::Cap(end))],
        )
    };
    let start = cap(block, CapEnd::Start);
    let top = fixture::plane(cap(p0, CapEnd::End));
    let doc = if through_a_datum {
        let (doc, datum, own) = datum_at(doc, VarId::WORLD, TargetAxis::X, 0.0);
        let flush = editor_core::Mate {
            on: RoleRead::through(Operand::Var(own), PoseRole::Xy),
            to: top.into(),
            orient: editor_core::Orientation::IDENTITY,
        };
        let (doc, _) = fixture::mated(doc, datum, flush, &[]);
        let rest = editor_core::Mate {
            on: fixture::flip(fixture::plane(start)).into(),
            to: RoleRead::through(Operand::Var(own), PoseRole::Xy),
            orient: editor_core::Orientation::IDENTITY,
        };
        fixture::mated(doc, p1, rest, &[]).0
    } else {
        let rest = editor_core::Mate {
            on: fixture::plane(start).into(),
            to: fixture::flip(top).into(),
            orient: editor_core::Orientation::IDENTITY,
        };
        fixture::mated(doc, p1, rest, &[]).0
    };
    fixture::union_over(doc, &[p0, p1])
}

/// The control: resting directly, the mate proves the caps one
/// construction at the first rung.
#[test]
fn review_b_a_direct_rest_is_stated() {
    let (doc, union) = stacked(false);
    assert_eq!(
        rest_row_proof(&doc, union),
        editor_core::Proof::Structural(editor_core::Rung::SameConstruction)
    );
}

/// Through a datum: the two mates state the same equivalence, so the
/// first rung should prove the row too. RED at 1c7255b5 (proven only at
/// `CarrierVerdict`, by a 2 s replay), while `Stated::of` walks
/// `Doc::placements()`, which leaves datums out.
#[test]
fn review_b_a_rest_through_a_datum_is_stated() {
    let (doc, union) = stacked(true);
    assert_eq!(
        rest_row_proof(&doc, union),
        editor_core::Proof::Structural(editor_core::Rung::SameConstruction)
    );
}

/// `flatten` (ASSEMBLY A4) re-targets "every placement into that space"
/// to the flattened placement's target, then deletes the space. A datum
/// in that space is placed into it as a copy is, so it should be
/// re-targeted too (or the flatten refuse, typed) — never left reading
/// the deleted space. RED at 1c7255b5: the walk is `Doc::placements()`
/// (`refactor.rs:5386`), which leaves datums out, and the datum is
/// reported `StrandedRead` on its `Space` slot.
#[test]
fn review_b_flatten_carries_a_datum_in_the_flattened_space() {
    let (doc, block, _) = block("rb-flatten");
    let (doc, space_node) = fixture::insert(doc, Node::Space);
    let space = doc.output(space_node, 0).expect("a space");
    let (doc, inner) = fixture::insert(
        doc,
        Node::place(
            Operand::output(block, 0),
            Operand::Var(space),
            editor_core::stand().bundle(),
        ),
    );
    let (doc, datum, _) = datum_at(doc, space, TargetAxis::X, 20.0);
    let (doc, outer) = fixture::insert(
        doc,
        Node::place(
            Operand::output(inner, 0),
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    match editor_core::flatten(&doc, outer, Tol::witness()) {
        Err(e) => eprintln!("flatten refused: {e:?}"),
        Ok(flat) => {
            assert!(flat.doc.node(space_node).is_none(), "the space is deleted");
            let target = match flat.doc.node(datum) {
                Some(Node::Datum { target, .. }) => *target,
                other => panic!("the datum survives: {other:?}"),
            };
            assert_eq!(
                target,
                VarId::WORLD,
                "the datum is re-targeted with the copy; maintenance {:?}",
                flat.maintenance
            );
        }
    }
}
