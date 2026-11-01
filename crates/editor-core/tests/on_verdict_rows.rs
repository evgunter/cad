//! **One body lying on another, through the public API**: the shapes
//! whose operands carry a closed surface lying wholly on the other
//! operand's, answered by the kernel's whole-shell `On` verdict
//! (`topo::boolean::shell_witness`), and a twin.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{Recorder, len};
use topo::BooleanOp;

use editor_core::{
    BooleanValue, CancelToken, EntityKind, EvalOptions, Evaluation, Node, NodeError, NodeErrorKind,
    NodeResult, ProfileDoc, RecipeNodeId, RoleSeg, StableName, ValuePayload, evaluate,
};
use geom_core::Tol;
use topo::BooleanResultKind;

const OPS: [BooleanOp; 3] = [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The box `[x0, x1] × [y0, y1] × [0, h]` on its own xy.
fn block(r: &mut Recorder, (x0, x1): (f64, f64), (y0, y1): (f64, f64), h: f64) -> RecipeNodeId {
    let p = r.insert(Node::Profile(crate::fixture::desc(vec![vec![
        (x0, y0),
        (x1, y0),
        (x1, y1),
        (x0, y1),
    ]])));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(h),
    })
}

/// [`block`] placed against `target`, its base at `z0` in `target`'s
/// coordinates: the placement, whose output is the copy.
fn block_on(
    r: &mut Recorder,
    target: RecipeNodeId,
    xs: (f64, f64),
    ys: (f64, f64),
    z0: f64,
    h: f64,
) -> RecipeNodeId {
    let b = block(r, xs, ys, h);
    r.against(target, b, &editor_core::Frame::translation([0.0, 0.0, z0]))
}

/// The document's node for the kernel verb `op` over `a` and `b`: a
/// two-member union or intersect, or `a` cut by `b`.
fn boolean_node(op: BooleanOp, a: RecipeNodeId, b: RecipeNodeId) -> editor_core::AuthoredNode {
    match op {
        BooleanOp::Union => Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
        BooleanOp::Intersect => Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
        BooleanOp::Subtract => Node::Subtract {
            from: a.into(),
            tool: b.into(),
        },
    }
}

fn boolean(r: &mut Recorder, op: BooleanOp, a: RecipeNodeId, b: RecipeNodeId) -> RecipeNodeId {
    r.insert(boolean_node(op, a, b))
}

/// The answer a boolean node holds: `None` for the typed empty result,
/// else the body's volume, shell count and result kind.
fn answer(ev: &Evaluation<f64>, id: RecipeNodeId) -> Option<(f64, usize, BooleanResultKind)> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(NodeError { kind, .. })) => {
            panic!("node {} refused: {kind:?}", id.0)
        }
        None => panic!("node {} did not evaluate", id.0),
        Some(_) => {}
    }
    match &ev.value(id).expect("a value").payload {
        ValuePayload::Boolean(BooleanValue::Empty) => None,
        ValuePayload::Boolean(BooleanValue::Body { body, kind, .. }) => {
            let volume = topo::test_support::pinned_volume(
                "the measured body",
                &topo::test_support::finished(
                    "the measured body",
                    topo::Body::clone(body.body()),
                    Tol::witness(),
                ),
                Tol::witness(),
            )
            .volume;
            Some((volume, body.body().shells().count(), *kind))
        }
        other => panic!("node {} is a {}, not a boolean", id.0, other.kind_name()),
    }
}

fn error_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NodeErrorKind {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(NodeError { kind, .. })) => kind,
        other => panic!("node {} must fail typed, got {other:?}", id.0),
    }
}

/// A labelled boolean node and the answer it owes.
type Row = (String, RecipeNodeId, Option<(f64, usize)>);

/// Asserts each `(label, node, want)`: `want` is `None` for the typed
/// empty result, else the volume and shell count.
fn assert_answers(ev: &Evaluation<f64>, rows: &[Row]) {
    for (label, id, want) in rows {
        let got = answer(ev, *id);
        match (want, got) {
            (None, None) => {}
            (Some((v, n)), Some((volume, shells, _))) => {
                assert!(
                    (volume - v).abs() < 1e-9,
                    "{label}: volume {volume}, want {v}"
                );
                assert_eq!(shells, *n, "{label}: shell count");
            }
            _ => panic!("{label}: got {got:?}, want {want:?}"),
        }
    }
}

/// A 1 × 1 × 0.75 box in a row of two, and two copies of `row[1]`
/// standing where the member does: one body at both seats.
fn two_parts_of_one_copy(r: &mut Recorder) -> (RecipeNodeId, RecipeNodeId) {
    let cube = block(r, (0.0, 1.0), (0.0, 1.0), 0.75);
    let (_, pat) = r.linear(cube, [1.0, 0.0, 0.0], 3.0, 2);
    let p = r.member(pat, 1);
    let q = r.member(pat, 1);
    (p, q)
}

/// **Two copies of `row[1]`**: ∪ and ∩ are the member, A's; − is the
/// typed empty result. The n-ary union of the two is the member too.
#[test]
fn two_parts_of_one_copy_answer_under_every_op() {
    let mut r = Recorder::new();
    let (p, q) = two_parts_of_one_copy(&mut r);
    let nodes: Vec<RecipeNodeId> = OPS.iter().map(|&op| boolean(&mut r, op, p, q)).collect();
    let union = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![p.into(), q.into()]),
    });
    let ev = eval(&r.doc);
    for (op, &id) in OPS.iter().zip(&nodes) {
        let got = answer(&ev, id);
        match op {
            BooleanOp::Subtract => {
                assert_eq!(got, None, "a member less itself is the typed empty result")
            }
            _ => assert_eq!(
                got.map(|(v, n, k)| ((v - 0.75).abs() < 1e-9, n, k)),
                Some((true, 1, BooleanResultKind::OperandA)),
                "{op:?}: the copy, one shell, A's"
            ),
        }
    }
    let copy = topo::mass_properties(
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(&(crate::corpus::body_of(&ev, union))),
            Tol::witness(),
        ),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap();
    assert!(
        (copy.enclosure.volume - 0.75).abs() < 1e-9,
        "the n-ary union is the copy: {}",
        copy.enclosure.volume
    );
}

/// **The kept `On` shell is named from both seats**: the two copies
/// hold one body, so every face of the result is held by both members,
/// and the union names each as the `Merged` of the two seats' names,
/// each carried in through its own read — exactly the six faces of the
/// half, and no face under one seat's name alone.
#[test]
fn the_kept_copy_is_named_from_both_seats() {
    let mut r = Recorder::new();
    let (p, q) = two_parts_of_one_copy(&mut r);
    let joined = boolean(&mut r, BooleanOp::Union, p, q);
    let ev = eval(&r.doc);
    let faces = |id: RecipeNodeId| -> Vec<StableName> {
        ev.table_at(id)
            .expect("a value")
            .iter()
            .filter(|(n, _)| n.kind == EntityKind::Face)
            .map(|(n, _)| n.clone())
            .collect()
    };
    let seat = faces(p);
    assert_eq!(seat.len(), 6, "the half has six faces: {seat:?}");
    assert_eq!(faces(q), seat, "the two seats name the half alike");
    let (p_read, q_read) = (
        crate::fixture::out(&r.doc, p),
        crate::fixture::out(&r.doc, q),
    );
    let carried = |read, n: &StableName| StableName {
        kind: EntityKind::Face,
        node: joined,
        path: vec![RoleSeg::From {
            read,
            of: n.clone().into(),
        }],
    };
    let mut want: Vec<StableName> = seat
        .iter()
        .map(|n| {
            let mut both = vec![carried(p_read, n), carried(q_read, n)];
            both.sort();
            StableName {
                kind: EntityKind::Face,
                node: joined,
                path: vec![RoleSeg::Merged(both)],
            }
        })
        .collect();
    want.sort();
    let mut got = faces(joined);
    got.sort();
    assert_eq!(
        got, want,
        "the union's face names are the merges of the two seats' names"
    );
}

/// **`master` with a copy of `row[0]`**: member 0 stands where the
/// master does and shares its body, so the two seats hold one body.
/// Both orders under − are empty.
#[test]
fn a_master_and_its_member_zero_answer() {
    let mut r = Recorder::new();
    let cube = block(&mut r, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (_, pat) = r.linear(cube, [1.0, 0.0, 0.0], 3.0, 3);
    let p0 = r.member(pat, 0);
    let mut rows = Vec::new();
    for op in OPS {
        let want = (op != BooleanOp::Subtract).then_some((1.0, 1));
        rows.push((
            format!("master {op:?} row[0]"),
            boolean(&mut r, op, cube, p0),
            want,
        ));
        rows.push((
            format!("row[0] {op:?} master"),
            boolean(&mut r, op, p0, cube),
            want,
        ));
    }
    assert_answers(&eval(&r.doc), &rows);
}

/// **`(X ∪ Z) op X` and `X op (X ∪ Z)`, Z disjoint from X**: X's shell
/// in the union is X's own, so it lies on X; Z's shell reads Out.
#[test]
fn a_union_with_a_disjoint_member_against_that_member() {
    let mut r = Recorder::new();
    let x = block(&mut r, (0.0, 1.0), (0.0, 1.0), 1.0);
    let z = block_on(&mut r, x, (3.0, 4.0), (0.0, 1.0), 0.0, 2.0);
    let xz = boolean(&mut r, BooleanOp::Union, x, z);
    let rows = vec![
        (
            "(X ∪ Z) ∪ X".to_owned(),
            boolean(&mut r, BooleanOp::Union, xz, x),
            Some((3.0, 2)),
        ),
        (
            "(X ∪ Z) ∩ X".to_owned(),
            boolean(&mut r, BooleanOp::Intersect, xz, x),
            Some((1.0, 1)),
        ),
        (
            "(X ∪ Z) − X".to_owned(),
            boolean(&mut r, BooleanOp::Subtract, xz, x),
            Some((2.0, 1)),
        ),
        (
            "X ∪ (X ∪ Z)".to_owned(),
            boolean(&mut r, BooleanOp::Union, x, xz),
            Some((3.0, 2)),
        ),
        (
            "X ∩ (X ∪ Z)".to_owned(),
            boolean(&mut r, BooleanOp::Intersect, x, xz),
            Some((1.0, 1)),
        ),
        (
            "X − (X ∪ Z)".to_owned(),
            boolean(&mut r, BooleanOp::Subtract, x, xz),
            None,
        ),
    ];
    assert_answers(&eval(&r.doc), &rows);
}

/// **`(X ∪ Y) op X`, Y strictly inside X**: the union carries exactly
/// X's sources, so its one shell lies on X's.
#[test]
fn a_union_with_an_inner_member_against_the_outer() {
    let mut r = Recorder::new();
    let x = block(&mut r, (0.0, 2.0), (0.0, 1.0), 1.0);
    let y = block_on(&mut r, x, (0.5, 1.5), (0.25, 0.75), 0.25, 0.5);
    let xy = boolean(&mut r, BooleanOp::Union, x, y);
    let rows = vec![
        (
            "(X ∪ Y) ∪ X".to_owned(),
            boolean(&mut r, BooleanOp::Union, xy, x),
            Some((2.0, 1)),
        ),
        (
            "(X ∪ Y) ∩ X".to_owned(),
            boolean(&mut r, BooleanOp::Intersect, xy, x),
            Some((2.0, 1)),
        ),
        (
            "(X ∪ Y) − X".to_owned(),
            boolean(&mut r, BooleanOp::Subtract, xy, x),
            None,
        ),
        (
            "X − (X ∪ Y)".to_owned(),
            boolean(&mut r, BooleanOp::Subtract, x, xy),
            None,
        ),
    ];
    assert_answers(&eval(&r.doc), &rows);
}

/// **A twin answers**: two placements of one block, and two
/// independent identical extrudes, lie face on face; the margins decide
/// each face pair one carrier, so every op answers.
#[test]
fn a_twin_answers() {
    let mut r = Recorder::new();
    let x = block(&mut r, (0.0, 1.0), (0.0, 1.0), 1.0);
    let place = |r: &mut Recorder| r.against(x, x, &editor_core::Frame::IDENTITY);
    let (s, t) = (place(&mut r), place(&mut r));
    let twin = block_on(&mut r, x, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let cases: Vec<(String, RecipeNodeId)> = OPS
        .iter()
        .flat_map(|&op| {
            [
                (format!("S {op:?} T"), boolean(&mut r, op, s, t)),
                (format!("X {op:?} T"), boolean(&mut r, op, x, t)),
                (format!("X {op:?} twin"), boolean(&mut r, op, x, twin)),
            ]
        })
        .collect();
    let want = |op: BooleanOp| (op != BooleanOp::Subtract).then_some((1.0, 1));
    let ev = eval(&r.doc);
    let rows: Vec<Row> = OPS
        .iter()
        .zip(cases.chunks(3))
        .flat_map(|(&op, ids)| {
            ids.iter()
                .map(move |(label, id)| (label.clone(), *id, want(op)))
                .collect::<Vec<_>>()
        })
        .collect();
    assert_answers(&ev, &rows);
}

/// **A surface the coincidences cover one way only refuses**: X against
/// X with a pocket sunk from its top face. Every point the union tries
/// on X lies on the pocketed body, and every face of X is the same face
/// as one of the pocketed body's, by recipe; but the pocket's faces are
/// the same face as none of X's, so X's surface is not certified to lie
/// on one surface of the other that lies back on it. The refusal names
/// no face pair, and the kernel's refusal crosses as it is.
#[test]
fn a_surface_covered_one_way_refuses_coincident_shell() {
    let mut r = Recorder::new();
    let x = block(&mut r, (0.0, 1.0), (0.0, 1.0), 1.0);
    let pocket = block_on(&mut r, x, (0.05, 0.25), (0.75, 0.95), 0.5, 1.5);
    let pocketed = boolean(&mut r, BooleanOp::Subtract, x, pocket);
    let joined = boolean(&mut r, BooleanOp::Union, x, pocketed);
    let ev = eval(&r.doc);
    assert!(
        matches!(
            error_of(&ev, joined),
            NodeErrorKind::Boolean(topo::BooleanError::CoincidentShell {
                operand: topo::Operand::A,
                orientation: topo::ShellOrientation::Same,
                ..
            })
        ),
        "{:?}",
        error_of(&ev, joined)
    );
}
