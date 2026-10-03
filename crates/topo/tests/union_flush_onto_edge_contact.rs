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

/// The unit-radius wedge from `d0` to `d1` degrees (counterclockwise,
/// under 180°) about the z-axis, over `z`.
fn wedge(d0: f64, d1: f64, z: (f64, f64), tol: Tol) -> Body<f64> {
    let at = |deg: f64| (deg.to_radians().cos(), deg.to_radians().sin());
    common::prism_z(&[(0.0, 0.0), at(d0), at(d1)], z.0, z.1, tol).body
}

/// Each op of `pinch` and `cutter`, in both operand orders and with
/// `carried` as the pinch's own records, builds at its volume and
/// passes 3′: `pinch ∪ cutter`, `pinch ∖ cutter`, `pinch ∩ cutter` and
/// `cutter ∖ pinch`.
fn every_op_builds(
    pinch: &Body<f64>,
    carried: &[CarriedVv],
    cutter: &Body<f64>,
    [union, pinch_less, common, cutter_less]: [f64; 4],
    tol: Tol,
) {
    let mut ab = flush_declarations(pinch, cutter, tol);
    ab.carried_a.vv = carried.to_vec();
    let mut ba = flush_declarations(cutter, pinch, tol);
    ba.carried_b.vv = carried.to_vec();
    for (op, got, want) in [
        ("pinch ∪ cutter", union_with(pinch, cutter, &ab, tol), union),
        (
            "pinch ∖ cutter",
            subtract_with(pinch, cutter, &ab, tol),
            pinch_less,
        ),
        (
            "pinch ∩ cutter",
            intersect_with(pinch, cutter, &ab, tol),
            common,
        ),
        ("cutter ∪ pinch", union_with(cutter, pinch, &ba, tol), union),
        (
            "cutter ∖ pinch",
            subtract_with(cutter, pinch, &ba, tol),
            cutter_less,
        ),
        (
            "cutter ∩ pinch",
            intersect_with(cutter, pinch, &ba, tol),
            common,
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

/// **A vertex crossing into both of a pinch's neighbourhoods, in every
/// op.** Two bricks touching along the z-axis hold two vertices at
/// each end of it; a prism whose corner wedge runs from 80° to 190° sits
/// on the axis with a corner at the pinch's lower end, so that corner
/// crosses into both bricks' corners, and its vertical edge runs along
/// the pinch line. Each op, in both operand orders, builds at its
/// volume and, with the pinch's records carried, passes 3′. Red as the
/// first row: the reduction refused `SharedVertexCrossings` in all six.
/// Red if the two pairs' germs along the prism's edge keep their own
/// folds: their cuts interleave round the corner, which refuses the
/// same way. Red under the pinch's ∖ and ∩ if a discarded face's
/// stretch takes any kept copy of the corner (`JoinDesync`, two kept
/// ends), and if the result's v-v rows leave out the null-edge copies
/// (3′ refuses the two kept corners and the edge overlap between
/// them). Red with the prism first if an A vertex may correspond to
/// one B vertex only (∪ and prism ∖ pinch refuse `JoinDesync`).
#[test]
fn a_vertex_crossing_both_sides_of_a_pinch_builds_in_every_op() {
    let tol = Tol::witness();
    let (pinch, carried) = pinch(tol);
    // The wedge's triangle is cut by the quadrants' walls; the part
    // between them, (0,0), (0, y), (x, 0) with the chord's two axis
    // crossings, lies outside the pinch, and the rest inside it.
    let (p, q) = (
        (80f64.to_radians().cos(), 80f64.to_radians().sin()),
        (190f64.to_radians().cos(), 190f64.to_radians().sin()),
    );
    let y = p.1 + p.0 / (p.0 - q.0) * (q.1 - p.1);
    let x = p.0 + p.1 / (p.1 - q.1) * (q.0 - p.0);
    let triangle = 0.5 * 110f64.to_radians().sin();
    let between = 0.5 * y * -x;
    let (outside, inside) = (between * 0.5, (triangle - between) * 0.5);
    every_op_builds(
        &pinch.body,
        &carried,
        &wedge(80.0, 190.0, (0.5, 1.0), tol),
        [2.0 + outside, 2.0 - inside, inside, outside],
        tol,
    );
}

/// Wedges about the z-axis over z ∈ (0.5, 1.5), folded into one body by
/// unions with each step's records carried: they touch only along the
/// axis, a pinch with a vertex per wedge at each end. Cut by the wedge
/// over `cutter` at z ∈ (0.5, 1), which crosses into the outer two and
/// holds the ones between, every op builds at `want` (∪, pinch ∖
/// cutter, ∩) and `cutter ∖ pinch` at the rest of the cutter.
fn crossings_at_one_corner(spans: &[(f64, f64)], cutter: (f64, f64), want: [f64; 3], tol: Tol) {
    let fold = |acc: &BooleanBody<f64>, next: &Body<f64>| {
        let mut decls = flush_declarations(&acc.body, next, tol);
        decls.carried_a.vv = rest_rows(&acc.contacts);
        match union_with(&acc.body, next, &decls, tol).expect("a touching wedge folds in") {
            BooleanResult::Body(out) => out,
            BooleanResult::Empty => panic!("a union of wedges came back empty"),
        }
    };
    let wedges: Vec<Body<f64>> = spans
        .iter()
        .map(|&(d0, d1)| wedge(d0, d1, (0.5, 1.5), tol))
        .collect();
    let BooleanResult::Body(two) = union_with(
        &wedges[0],
        &wedges[1],
        &flush_declarations(&wedges[0], &wedges[1], tol),
        tol,
    )
    .expect("two wedges touching along an edge build") else {
        panic!("a union of wedges came back empty");
    };
    let pinch = wedges[2..].iter().fold(two, |acc, next| fold(&acc, next));
    let n = spans.len();
    assert_eq!(
        pinch.contacts.vv.len(),
        n * (n - 1),
        "{n} vertices at each end of the pinch line, a row per two"
    );
    let [union, pinch_less, common] = want;
    let cutter_volume = 0.5 * (cutter.1 - cutter.0).to_radians().sin() * 0.5;
    every_op_builds(
        &pinch.body,
        &rest_rows(&pinch.contacts),
        &wedge(cutter.0, cutter.1, (0.5, 1.0), tol),
        [union, pinch_less, common, cutter_volume - common],
        tol,
    );
}

/// **Three crossing pairs at one corner, in every op.** Three wedges
/// (0°–60°, 90°–120°, 150°–210°) cut by one from 50° to 160°: the
/// middle wedge's corner cuts the prism's bottom corner between the
/// outer pairs' cuts as a dangling null edge. Each op, in both operand
/// orders, builds at the volume a polygon clipping outside the kernel
/// gives and passes 3′ with the pinch's records carried. Red as the
/// first row: `SharedVertexCrossings`. Red if a fan is minted at the
/// corner before the dangling null edge there (∪ refuses
/// `CorruptOperand`), or if a dangling null edge refuses whenever
/// another pair cuts its corner.
#[test]
fn three_crossings_at_one_corner_build_in_every_op() {
    crossings_at_one_corner(
        &[(0.0, 60.0), (90.0, 120.0), (150.0, 210.0)],
        (50.0, 160.0),
        [
            1.238_158_743_915_086_7,
            1.003_235_588_718_609_7,
            0.112_789_815_065_828_86,
        ],
        Tol::witness(),
    );
}

/// **Four crossing pairs at one corner, two of them dangling.** Four
/// wedges (0°–30°, 60°–80°, 100°–120°, 150°–200°) cut by one from 20°
/// to 160°: the middle two corners hang two dangling null edges in the
/// prism's bottom corner, between the outer pairs' cuts. Each op, in
/// both operand orders, builds at the volume a polygon clipping outside
/// the kernel gives and passes 3′. Red if a dangling null edge splices
/// at its corner's own successor, ahead of one hung there before it at
/// an earlier germ (∪ refuses `JoinDesync`), and as the three's.
#[test]
fn four_crossings_at_one_corner_build_in_every_op() {
    crossings_at_one_corner(
        &[(0.0, 30.0), (60.0, 80.0), (100.0, 120.0), (150.0, 200.0)],
        (20.0, 160.0),
        [
            1.053_289_933_583_019_3,
            0.892_593_031_161_384_3,
            0.082_449_333_723_773_3,
        ],
        Tol::witness(),
    );
}

/// **Two pinches crossing on one line refuse their union typed.** Four
/// bricks in the four quadrants about the z-axis, paired into two
/// pinches that overlap over z ∈ (1, 1.5): at each end of the overlap
/// each pinch holds two vertices and every pair of them crosses, so two
/// crossing pairs share both their vertices, and the union would join
/// all four into one. Red if that case reaches the finish: the zip
/// fuses a vertex onto an edge's other end and refuses
/// `Euler(SelfLoopEdge)`.
#[test]
fn two_pinches_crossing_on_one_line_refuse_their_union_typed() {
    let tol = Tol::witness();
    let (pinch_a, carried_a) = pinch(tol);
    let q2: Body<f64> = brick((-1.0, 0.0), (0.0, 1.0), (1.0, 2.0), tol);
    let q4: Body<f64> = brick((0.0, 1.0), (-1.0, 0.0), (1.0, 2.0), tol);
    let BooleanResult::Body(pinch_b) =
        union_with(&q2, &q4, &flush_declarations(&q2, &q4, tol), tol).expect("a pinch builds")
    else {
        panic!("a union of two bricks came back empty");
    };
    let carried_b = rest_rows(&pinch_b.contacts);
    for (name, a, b, ca, cb) in [
        ("a ∪ b", &pinch_a, &pinch_b, &carried_a, &carried_b),
        ("b ∪ a", &pinch_b, &pinch_a, &carried_b, &carried_a),
    ] {
        let mut decls = flush_declarations(&a.body, &b.body, tol);
        decls.carried_a.vv.clone_from(ca);
        decls.carried_b.vv.clone_from(cb);
        let got = union_with(&a.body, &b.body, &decls, tol);
        assert!(
            matches!(got, Err(BooleanError::SharedVertexCrossings { .. })),
            "{name}: want SharedVertexCrossings, got {:?}",
            got.map(|_| ())
        );
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

/// **A remap group of four rows.** Two pinches on one axis in
/// complementary quadrants, overlapping over z ∈ (1, 1.5): at each end
/// of the overlap, two vertices of one pinch meet two of the other's
/// (one end's, and the other's edges split there), so the reduction
/// holds four v-v rows chained through shared keys. The subtract
/// builds at the first pinch's volume and, with both pinches' records
/// carried, passes 3′. Red if the remap records only the pairs a row
/// names: 3′ refuses the two vertices left at (0,0,1) and the edge
/// overlaps beside them.
#[test]
fn a_four_row_remap_group_certifies_a_subtract_of_two_pinches() {
    let tol = Tol::witness();
    let (pinch_a, carried_a) = pinch(tol);
    let q2: Body<f64> = brick((-1.0, 0.0), (0.0, 1.0), (1.0, 2.0), tol);
    let q4: Body<f64> = brick((0.0, 1.0), (-1.0, 0.0), (1.0, 2.0), tol);
    let BooleanResult::Body(pinch_b) =
        union_with(&q2, &q4, &flush_declarations(&q2, &q4, tol), tol).expect("a pinch builds")
    else {
        panic!("a union of two bricks came back empty");
    };
    let mut decls = flush_declarations(&pinch_a.body, &pinch_b.body, tol);
    decls.carried_a.vv = carried_a;
    decls.carried_b.vv = rest_rows(&pinch_b.contacts);
    let BooleanResult::Body(out) =
        subtract_with(&pinch_a.body, &pinch_b.body, &decls, tol).expect("the subtract builds")
    else {
        panic!("the subtract of disjoint quadrants came back empty");
    };
    let volume = mass_properties(&out.body, tol).expect("mass").volume;
    assert!((volume - 2.0).abs() < 1e-9, "volume {volume}, want 2.0");
    let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
    assert!(verdict.is_ok(), "3′ refused {:?}", verdict.err());
}
