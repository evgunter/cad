//! **A flush partner folded onto an edge contact.** `a` and `b` are
//! flush along x; `c` touches them along one edge only, so a union
//! holding `c` and one of the two holds two vertices at each end of
//! the touch. The partner that folds in next has a vertex there,
//! which pairs with both, and every order of the three builds.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, flush_declarations};
use geom_core::Tol;
use topo::{
    Body, BooleanBody, BooleanError, BooleanResult, CarriedVv, ContactClass, intersect_with,
    mass_properties, subtract_with, union_with, validate_pseudomanifold,
};

/// `c`'s x-range: over `b` alone, inside `a ∩ b`, and across `a`'s
/// far end.
const C_SPANS: [(f64, f64); 3] = [(0.5, 1.5), (0.25, 0.75), (-0.5, 0.5)];

const ORDERS: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

fn members(c_span: (f64, f64), tol: Tol) -> [Body<f64>; 3] {
    [
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        brick((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol),
        brick(c_span, (-1.0, 0.0), (-1.0, 0.0), tol),
    ]
}

/// Folds `order`, each step declaring its flush pairs and, when
/// `carry`, carrying the accumulator's own v-v records in as `Rest`.
/// Every step's result, or the refusal and the step it came at.
fn fold(
    bodies: &[Body<f64>; 3],
    order: [usize; 3],
    carry: bool,
    tol: Tol,
) -> Result<Vec<BooleanBody<f64>>, (usize, BooleanError)> {
    let mut steps: Vec<BooleanBody<f64>> = Vec::new();
    for (step, &next) in order[1..].iter().enumerate() {
        let acc = steps.last().map_or(&bodies[order[0]], |s| &s.body);
        let mut decls = flush_declarations(acc, &bodies[next], tol);
        if carry && let Some(prev) = steps.last() {
            decls.carried_a.vv = rest_rows(&prev.contacts);
        }
        match union_with(acc, &bodies[next], &decls, tol).map_err(|e| (step + 1, e))? {
            BooleanResult::Body(out) => steps.push(out),
            BooleanResult::Empty => panic!("a union of two bricks came back empty"),
        }
    }
    Ok(steps)
}

/// **Every order builds, at the volume of the three bricks' union**
/// (`a ∪ b` is 1.5, and `c` meets it in no volume).
/// Red when a vertex-vertex pair is classified after another pair
/// sharing its vertex has moved that vertex's orbit: the walk meets the
/// null edge the first insertion hung there and refuses
/// `CorruptOperand { operand: B }` at step 2 of `b,c,a` (every span)
/// and `a,c,b` (the span over `b`).
#[test]
fn every_order_folds_a_flush_partner_onto_the_edge_contact() {
    let tol = Tol::witness();
    for span in C_SPANS {
        let bodies = members(span, tol);
        for order in ORDERS {
            let steps = fold(&bodies, order, false, tol).unwrap_or_else(|(step, e)| {
                panic!("c over {span:?}, order {order:?}: step {step} refused: {e:?}")
            });
            let last = steps.last().expect("two steps");
            let volume = mass_properties(&last.body, tol)
                .expect("the union has mass")
                .volume;
            let want = 1.5 + (span.1 - span.0);
            assert!(
                (volume - want).abs() < 1e-9,
                "c over {span:?}, order {order:?}: volume {volume}, want {want}"
            );
        }
    }
}

/// **With the accumulator's records carried, every step passes the
/// at-rest census.** Red when a step's v-v row naming the vertex that
/// fused into its partner is dropped as consumed: the vertex it still
/// touches (the edge contact's other vertex there) then sits beside
/// the fused one with no record, and 3′ refuses the two vertices and
/// the edge overlaps either side of them as undeclared (`b,c,a` and
/// `c,b,a` over `b`; `a,c,b` and `c,a,b` inside `a ∩ b`).
#[test]
fn carried_records_certify_every_order_of_the_fold() {
    let tol = Tol::witness();
    for span in C_SPANS {
        let bodies = members(span, tol);
        for order in ORDERS {
            let steps = fold(&bodies, order, true, tol).unwrap_or_else(|(step, e)| {
                panic!("c over {span:?}, order {order:?}: step {step} refused: {e:?}")
            });
            for (i, s) in steps.iter().enumerate() {
                let verdict = validate_pseudomanifold(&s.body, &s.contacts, tol);
                assert!(
                    verdict.is_ok(),
                    "c over {span:?}, order {order:?}, step {}: 3′ refused {:?}",
                    i + 1,
                    verdict.err()
                );
            }
        }
    }
}

/// Two bricks touching along the z-axis over z ∈ (0.5, 1.5), and
/// their record carried as `Rest` rows: two vertices at each end of
/// the touch.
fn pinch(tol: Tol) -> (BooleanBody<f64>, Vec<CarriedVv>) {
    let q1: Body<f64> = brick((0.0, 1.0), (0.0, 1.0), (0.5, 1.5), tol);
    let q3: Body<f64> = brick((-1.0, 0.0), (-1.0, 0.0), (0.5, 1.5), tol);
    let BooleanResult::Body(pinch) = union_with(&q1, &q3, &flush_declarations(&q1, &q3, tol), tol)
        .expect("an edge contact builds")
    else {
        panic!("a union of two bricks came back empty");
    };
    assert_eq!(
        pinch.contacts.vv.len(),
        2,
        "the edge contact records a v-v row at each end"
    );
    let carried = rest_rows(&pinch.contacts);
    (pinch, carried)
}

fn rest_rows(records: &topo::ContactRecords) -> Vec<CarriedVv> {
    records
        .vv
        .iter()
        .map(|&pair| CarriedVv {
            pair,
            class: ContactClass::Rest,
        })
        .collect()
}

/// **A vertex crossing into both of a pinch's neighbourhoods refuses
/// typed.** Two bricks touching along the z-axis hold two vertices at
/// each end of it; a prism whose corner wedge runs from 80° to 190°
/// sits on the axis with a corner at the pinch, so its corner crosses
/// both bricks' corners. Each crossing pair would split the shared
/// corner's orbit the other read, and the insertion handles one: the
/// refusal names both pairs. Red if the reading-before-insertion pass
/// lets both through (the second walks a moved orbit and refuses
/// `CorruptOperand` at the prism's corner).
#[test]
fn a_vertex_crossing_both_sides_of_a_pinch_refuses_typed() {
    let tol = Tol::witness();
    let (pinch, _) = pinch(tol);
    let at = |deg: f64| (deg.to_radians().cos(), deg.to_radians().sin());
    let wedge: Body<f64> = common::prism_z(&[(0.0, 0.0), at(190.0), at(80.0)], 0.5, 1.0, tol).body;
    let decls = flush_declarations(&pinch.body, &wedge, tol);
    match union_with(&pinch.body, &wedge, &decls, tol) {
        Err(BooleanError::SharedVertexCrossings {
            operand,
            partners: [p0, p1],
            ..
        }) => assert!(
            operand == topo::Operand::B && p0 != p1,
            "the prism's corner is the shared vertex, and its partners are the \
             pinch's two vertices: {operand:?}, {p0:?}, {p1:?}"
        ),
        other => panic!("want SharedVertexCrossings, got {:?}", other.map(|_| ())),
    }
}

/// **A face through the pinch line, in every op and both operand
/// orders.** Two bricks touching along the z-axis (the pinch above)
/// against a prism whose diagonal wall holds that axis: each of the
/// wall's vertices on the axis pairs with both of the pinch's
/// vertices there, one pair crossing and one touching. Every op
/// builds at its volume, and with the pinch's records carried the
/// result passes 3′. Red as the first row: `CorruptOperand` at the
/// prism's vertex on the axis, in all four.
#[test]
fn a_face_through_a_pinch_line_builds_in_every_op() {
    let tol = Tol::witness();
    let (pinch, carried) = pinch(tol);
    let wall: Body<f64> =
        common::prism_z(&[(-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)], 0.0, 1.0, tol).body;
    let mut ab = flush_declarations(&pinch.body, &wall, tol);
    ab.carried_a.vv.clone_from(&carried);
    let mut ba = flush_declarations(&wall, &pinch.body, tol);
    ba.carried_b.vv = carried;
    // The prism holds the half x + y > 0 of the square, so it meets
    // the pinch in the brick on that side, z ∈ (0.5, 1): 0.5.
    for (op, got, want) in [
        (
            "pinch ∪ wall",
            union_with(&pinch.body, &wall, &ab, tol),
            3.5,
        ),
        (
            "pinch ∖ wall",
            subtract_with(&pinch.body, &wall, &ab, tol),
            1.5,
        ),
        (
            "pinch ∩ wall",
            intersect_with(&pinch.body, &wall, &ab, tol),
            0.5,
        ),
        (
            "wall ∪ pinch",
            union_with(&wall, &pinch.body, &ba, tol),
            3.5,
        ),
    ] {
        let BooleanResult::Body(out) = got.unwrap_or_else(|e| panic!("{op} refused: {e:?}")) else {
            panic!("{op} came back empty");
        };
        let volume = mass_properties(&out.body, tol).expect("mass").volume;
        assert!(
            (volume - want).abs() < 1e-9,
            "{op}: volume {volume}, want {want}"
        );
        let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
        assert!(verdict.is_ok(), "{op}: 3′ refused {:?}", verdict.err());
    }
}
