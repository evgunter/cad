//! **One body lying on another, through the public API**: the shapes
//! whose operands carry a closed surface lying wholly on the other
//! operand's, answered by the kernel's whole-shell `On` verdict
//! (`topo::boolean::shell_witness`), and the twin no recipe or
//! declaration makes one, which still refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{Recorder, ang, len, scl};

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, Datum, EntityKind, EvalOptions, Evaluation, Expr, Node,
    NodeError, NodeErrorKind, NodeResult, PartSelect, PatternKind, ProfileDoc, RecipeNodeId,
    RoleSeg, SplitHalf, StableName, ValuePayload, declared_pairs, evaluate, find_flush_candidates,
};
use geom_core::Tol;
use topo::{BooleanResultKind, mass_properties};

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

/// The box `[x0, x1] × [y0, y1] × [z0, z0 + h]`.
fn block(
    r: &mut Recorder,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    h: f64,
) -> RecipeNodeId {
    let p = r.profile(
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    r.insert(Node::Extrude {
        profile: p,
        distance: len(h),
    })
}

fn boolean(r: &mut Recorder, op: BooleanOp, a: RecipeNodeId, b: RecipeNodeId) -> RecipeNodeId {
    r.insert(Node::Boolean {
        op,
        a,
        b,
        declare: Vec::new(),
    })
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
            let volume = mass_properties(body, Tol::witness()).unwrap().volume;
            Some((volume, body.shells().count(), *kind))
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

/// The unit box split at z = 0.25, and two `Part(Above)` of it: one
/// body at both seats.
fn two_parts_of_one_half(r: &mut Recorder) -> (RecipeNodeId, RecipeNodeId) {
    let cube = block(r, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let tool = r.insert(Node::Datum(Datum::Plane {
        origin: [len(0.0), len(0.0), len(0.25)],
        normal: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    let split = r.insert(Node::Split { target: cube, tool });
    let above = PartSelect::SplitHalf(SplitHalf::Above);
    let p = r.insert(Node::Part {
        of: split,
        select: above.clone(),
    });
    let q = r.insert(Node::Part {
        of: split,
        select: above,
    });
    (p, q)
}

/// **Two `Part(Above)` of one split**: ∪ and ∩ are the half, A's copy;
/// − is the typed empty result. The n-ary union of the two is the half
/// too.
#[test]
fn two_parts_of_one_half_answer_under_every_op() {
    let mut r = Recorder::new();
    let (p, q) = two_parts_of_one_half(&mut r);
    let nodes: Vec<RecipeNodeId> = OPS.iter().map(|&op| boolean(&mut r, op, p, q)).collect();
    let union = r.insert(Node::Union {
        members: vec![p, q],
        declare: Vec::new(),
    });
    let ev = eval(&r.doc);
    for (op, &id) in OPS.iter().zip(&nodes) {
        let got = answer(&ev, id);
        match op {
            BooleanOp::Subtract => assert_eq!(got, None, "Part − Part is the typed empty result"),
            _ => assert_eq!(
                got.map(|(v, n, k)| ((v - 0.75).abs() < 1e-9, n, k)),
                Some((true, 1, BooleanResultKind::OperandA)),
                "{op:?}: the half, one shell, A's copy"
            ),
        }
    }
    let half = mass_properties(crate::corpus::body_of(&ev, union), Tol::witness()).unwrap();
    assert!(
        (half.volume - 0.75).abs() < 1e-9,
        "the n-ary union is the half: {}",
        half.volume
    );
}

/// **The kept `On` shell is named as the boolean names A's surviving
/// faces**: the result is A's copy (`OperandA`), so the emitter mints
/// each of its faces as `FromA(<the A seat's name>)` under the boolean
/// node, exactly the six faces of the half, and no `FromB` name: B's
/// copy is dropped whole.
#[test]
fn the_kept_copy_is_named_from_the_a_seat() {
    let mut r = Recorder::new();
    let (p, q) = two_parts_of_one_half(&mut r);
    let joined = boolean(&mut r, BooleanOp::Union, p, q);
    let ev = eval(&r.doc);
    let faces = |id: RecipeNodeId| -> Vec<StableName> {
        ev.value(id)
            .expect("a value")
            .name_table
            .iter()
            .filter(|(n, _)| n.kind == EntityKind::Face)
            .map(|(n, _)| n.clone())
            .collect()
    };
    let seat = faces(p);
    assert_eq!(seat.len(), 6, "the half has six faces: {seat:?}");
    let mut want: Vec<StableName> = seat
        .into_iter()
        .map(|n| StableName {
            kind: EntityKind::Face,
            node: joined,
            path: vec![RoleSeg::FromA(n.into())],
        })
        .collect();
    want.sort();
    let mut got = faces(joined);
    got.sort();
    assert_eq!(
        got, want,
        "the union's face names are FromA of the A seat's"
    );
}

/// **`master` with `Part(Instance(0))`**: instance 0 is the master's own
/// body, so the two seats hold one body. Both orders under − are empty.
#[test]
fn a_master_and_its_instance_zero_answer() {
    let mut r = Recorder::new();
    let cube = block(&mut r, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let pat = r.insert(Node::Pattern {
        input: cube,
        count: Expr::count(3),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(3.0),
        },
    });
    let p0 = r.insert(Node::Part {
        of: pat,
        select: PartSelect::Instance(Expr::count(0)),
    });
    let mut rows = Vec::new();
    for op in OPS {
        let want = (op != BooleanOp::Subtract).then_some((1.0, 1));
        rows.push((
            format!("master {op:?} Part(0)"),
            boolean(&mut r, op, cube, p0),
            want,
        ));
        rows.push((
            format!("Part(0) {op:?} master"),
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
    let x = block(&mut r, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let z = block(&mut r, (3.0, 4.0), (0.0, 1.0), 0.0, 2.0);
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
    let x = block(&mut r, (0.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let y = block(&mut r, (0.5, 1.5), (0.25, 0.75), 0.25, 0.5);
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

/// **Two placements of one block, all six face pairs declared**: the
/// declarations settle each face with its twin, so the shells are one.
/// Undeclared, the same pair refuses `UndeclaredCoincidence`, as do two
/// independent identical extrudes: value equality never makes one face.
#[test]
fn a_declared_twin_answers_and_an_undeclared_one_refuses() {
    let mut r = Recorder::new();
    let x = block(&mut r, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let place = |r: &mut Recorder| {
        r.insert(Node::transform(
            x,
            editor_core::Step::Rigid {
                translation: [len(0.0), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ))
    };
    let (s, t) = (place(&mut r), place(&mut r));
    let twin = block(&mut r, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let undeclared: Vec<(String, RecipeNodeId)> = OPS
        .iter()
        .flat_map(|&op| {
            [
                (format!("S {op:?} T"), boolean(&mut r, op, s, t)),
                (format!("X {op:?} T"), boolean(&mut r, op, x, t)),
                (format!("X {op:?} twin"), boolean(&mut r, op, x, twin)),
            ]
        })
        .collect();
    let ev = eval(&r.doc);
    for (label, id) in &undeclared {
        assert!(
            matches!(
                error_of(&ev, *id),
                NodeErrorKind::UndeclaredCoincidence { .. }
            ),
            "{label}: {:?}",
            error_of(&ev, *id)
        );
    }

    let findings = find_flush_candidates(&ev, s, t, Tol::witness()).unwrap();
    assert_eq!(findings.len(), 6, "one finding per face pair: {findings:?}");
    let pairs = declared_pairs(&findings).unwrap();
    let rows: Vec<Row> = OPS
        .iter()
        .map(|&op| {
            let id = r.insert(Node::Boolean {
                op,
                a: s,
                b: t,
                declare: pairs.clone(),
            });
            (
                format!("S {op:?} T, declared"),
                id,
                (op != BooleanOp::Subtract).then_some((1.0, 1)),
            )
        })
        .collect();
    assert_answers(&eval(&r.doc), &rows);
}

/// **A surface the coincidences cover one way only refuses**: X against
/// X with a pocket sunk from its top face. Every point the union tries
/// on X lies on the pocketed body, and every face of X is the same face
/// as one of the pocketed body's, by recipe; but the pocket's faces are
/// the same face as none of X's, so X's surface is not certified to lie
/// on one surface of the other that lies back on it. The refusal names
/// no face pair, so the refusal menu has no declaration to offer and
/// the kernel's refusal crosses as it is.
#[test]
fn a_surface_covered_one_way_refuses_coincident_shell() {
    let mut r = Recorder::new();
    let x = block(&mut r, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let pocket = block(&mut r, (0.05, 0.25), (0.75, 0.95), 0.5, 1.5);
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
