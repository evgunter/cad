//! **A flush partner folded onto an edge contact.** `a` and `b` are
//! flush along x; `c` touches them along one edge only, so a union
//! holding `c` and one of the two holds two vertices at each end of
//! the touch. The partner that folds in next has a vertex there,
//! which pairs with both, and every order of the three builds.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished};
use geom_core::Tol;
use topo::SideCode;
use topo::{
    AtRestBody, Body, BooleanBody, BooleanError, BooleanInputs, BooleanOp, BooleanResult,
    mass_properties, subtract, union, validate_pseudomanifold,
};

/// `op` on `a` and `b`, carrying `inputs`' records.
fn op_in(
    op: BooleanOp,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    inputs: &BooleanInputs,
    tol: Tol,
) -> Result<BooleanResult<f64>, BooleanError> {
    topo::boolean_op_with(op, a, b, inputs, topo::SweepStrategy::Realized, tol)
}

fn union_in(
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    inputs: &BooleanInputs,
    tol: Tol,
) -> Result<BooleanResult<f64>, BooleanError> {
    op_in(BooleanOp::Union, a, b, inputs, tol)
}

fn subtract_in(
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    inputs: &BooleanInputs,
    tol: Tol,
) -> Result<BooleanResult<f64>, BooleanError> {
    op_in(BooleanOp::Subtract, a, b, inputs, tol)
}

fn intersect_in(
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    inputs: &BooleanInputs,
    tol: Tol,
) -> Result<BooleanResult<f64>, BooleanError> {
    op_in(BooleanOp::Intersect, a, b, inputs, tol)
}

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

fn members(c_span: (f64, f64), tol: Tol) -> [AtRestBody<f64>; 3] {
    [
        finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol), tol),
        finished("b", brick((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol), tol),
        finished("c", brick(c_span, (-1.0, 0.0), (-1.0, 0.0), tol), tol),
    ]
}

/// **Each edge piece the boolean classed beside a vertex of its
/// operand reads one class, its side of the other operand just along
/// it**: the other operand's material at the first of 1e-4, 1e-3 and
/// 1e-2 along the edge from the vertex that `topo::point_in_solid`
/// decides, the vertex's cone there, `On` where the point lies on its
/// boundary. Pieces at a vertex the boolean
/// minted are not read. A vertex in pairs alone layers its partners
/// into one row per edge (`vtxfac::pair_classes`). Red when the
/// layering keeps each pair's rows: an edge on two partners'
/// boundaries reads `On` and `Out`, and one in a void that holds an
/// island touching it along a ray reads `In` and `Out`.
fn assert_classes(out: &BooleanBody<f64>, [a, b]: [&AtRestBody<f64>; 2], what: &str, tol: Tol) {
    let band = geom_core::Band::linear(tol).expect("the band");
    let mut classes = std::collections::BTreeMap::new();
    for row in &out.naming.edge_classes {
        classes
            .entry((row.operand, row.vertex, row.edge, row.starts))
            .or_insert_with(Vec::new)
            .push(row.class);
    }
    for ((operand, vertex, edge, starts), got) in classes {
        let (own, other) = match operand {
            topo::Operand::A => (a, b),
            topo::Operand::B => (b, a),
        };
        let Ok(p) = topo::readback::vertex_point(own, vertex) else {
            continue;
        };
        let e = own.get_edge(edge).expect("a classed edge of the operand");
        let plus = own.get_half_edge(e.he_plus).expect("its half-edge");
        let next = own.get_half_edge(plus.next).expect("the next half-edge");
        let (here, far) = if starts {
            (plus.start, next.start)
        } else {
            (next.start, plus.start)
        };
        if here != vertex {
            continue;
        }
        let d = topo::readback::vertex_point(own, far).expect("its far end") - p;
        // The nearest step the band decides, within the vertex's cone.
        let want = [1e-4, 1e-3, 1e-2]
            .iter()
            .find_map(|&step| {
                let q = p + d * (step / d.norm());
                topo::point_in_solid(other, q, band, tol).ok()
            })
            .map(|read| match read {
                topo::SolidContainment::In => SideCode::In,
                topo::SolidContainment::Out => SideCode::Out,
                topo::SolidContainment::OnBoundary => SideCode::On,
            })
            .unwrap_or_else(|| panic!("{what}: no step along {edge:?} reads decidedly"));
        assert_eq!(
            got,
            vec![want],
            "{what}: {operand:?}'s edge {edge:?} at {vertex:?} (starts {starts})"
        );
    }
}

/// The two operands of `op`, `"x ∘ y …"`, by name among `named`.
fn operands<'a>(op: &str, named: &[(&str, &'a AtRestBody<f64>)]) -> [&'a AtRestBody<f64>; 2] {
    let words: Vec<&str> = op.split_whitespace().collect();
    [words[0], words[2]].map(|w| {
        named
            .iter()
            .find(|(n, _)| *n == w)
            .unwrap_or_else(|| panic!("{op}: no operand named {w}"))
            .1
    })
}

/// Folds `order`, each step, when `carry`, carrying the accumulator's own records in ([`carried_rows`]).
/// Every step's result, or the refusal and the step it came at.
fn fold(
    bodies: &[AtRestBody<f64>; 3],
    order: [usize; 3],
    carry: bool,
    tol: Tol,
) -> Result<Vec<BooleanBody<f64>>, (usize, Box<BooleanError>)> {
    let mut steps: Vec<BooleanBody<f64>> = Vec::new();
    for (step, &next) in order[1..].iter().enumerate() {
        let acc = steps.last().map_or(&bodies[order[0]], |s| &s.body);
        let mut decls = BooleanInputs::default();
        if carry && let Some(prev) = steps.last() {
            decls.records_a = carried_rows(&prev.contacts);
        }
        match union_in(acc, &bodies[next], &decls, tol).map_err(|e| (step + 1, Box::new(e)))? {
            BooleanResult::Body(out) => {
                assert_classes(
                    &out,
                    [acc, &bodies[next]],
                    &format!("order {order:?}, step {step}"),
                    tol,
                );
                steps.push(out);
            }
            BooleanResult::Empty => panic!("a union of two bricks came back empty"),
        }
    }
    Ok(steps)
}

/// **Every order builds, at the volume of the three bricks' union**
/// (`a ∪ b` is 1.5, and `c` meets it in no volume).
/// Red when a vertex-vertex pair is classified after another pair
/// sharing its vertex has moved that vertex's orbit: the walk meets the
/// null edge the first insertion hung there and panics at step 2 of
/// `b,c,a` (every span) and `a,c,b` (the span over `b`).
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
            let volume =
                mass_properties(&last.body, tol, topo::test_support::pinned_resolution(tol))
                    .expect("the union has mass")
                    .met()
                    .expect("the pinned resolution is met")
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

/// Every row of `records` carried in: the v-v rows, and the
/// `(vertex, edge)` and edge-edge rows a join leaves.
fn carried_rows(records: &topo::ContactRecords) -> topo::OperandRecords {
    topo::OperandRecords {
        vf: Vec::new(),
        ..records.carried()
    }
}

/// **With the records carried, every order certifies and builds one
/// body.** The fold of [`fold`], each step's output joined by its own
/// output stage, as maximal edges asks of every output
/// (`docs/DESIGN.md`, the merge stage): every step passes 3′ and holds
/// no joinable vertex, and the finished bodies of all six orders have
/// one vertex, edge and face count, for each span (16, 24 and 12). Red
/// when the join drops the v-v record whose vertex it joined away: the
/// touching corner then rests on the joined rim unrecorded
/// (`UnrecordedContact` `VertexOnEdge` and the overlap it bounds). Red
/// when a step's output stage skips the join: the step holds a
/// joinable vertex, and the orders that cut the rim before the flush
/// pair merges keep a vertex the others never mint.
#[test]
fn joined_folds_certify_and_build_one_body_in_every_order() {
    let tol = Tol::witness();
    for span in C_SPANS {
        let bodies = members(span, tol);
        let mut counts = Vec::new();
        for order in ORDERS {
            let mut acc: Option<BooleanBody<f64>> = None;
            for &next in &order[1..] {
                let base = acc.as_ref().map_or(&bodies[order[0]], |s| &s.body);
                let mut decls = BooleanInputs::default();
                if let Some(prev) = &acc {
                    decls.records_a = carried_rows(&prev.contacts);
                }
                let out = match union_in(base, &bodies[next], &decls, tol) {
                    Ok(BooleanResult::Body(out)) => out,
                    other => panic!("c over {span:?}, order {order:?}: {other:?}"),
                };
                assert!(
                    topo::joinable_vertices(
                        &out.body,
                        geom_core::Band::linear(geom_core::Tol::witness()).unwrap()
                    )
                    .unwrap()
                    .is_empty(),
                    "c over {span:?}, order {order:?}, at {next}: a joinable vertex is left"
                );
                let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
                assert!(
                    verdict.is_ok(),
                    "c over {span:?}, order {order:?}, at {next}: 3′ refused {:?}",
                    verdict.err()
                );
                acc = Some(out);
            }
            let body = &acc.expect("two steps").body;
            assert!(
                topo::joinable_vertices(
                    body,
                    geom_core::Band::linear(geom_core::Tol::witness()).unwrap()
                )
                .unwrap()
                .is_empty(),
                "c over {span:?}, order {order:?}: a joinable vertex is left"
            );
            counts.push((
                order,
                body.vertices().count(),
                body.edges().count(),
                body.faces().count(),
            ));
        }
        let first = (counts[0].1, counts[0].2, counts[0].3);
        assert!(
            counts.iter().all(|c| (c.1, c.2, c.3) == first),
            "c over {span:?}: the orders build different bodies: {counts:?}"
        );
    }
}

/// Two bricks touching along the z-axis over z ∈ (0.5, 1.5), and
/// their records as carried rows: a v-v row at each end of the touch
/// and the edge-edge row along it.
fn pinch(tol: Tol) -> (BooleanBody<f64>, topo::OperandRecords) {
    quarter_pinch([(0.0, 0.0), (-1.0, -1.0)], (0.5, 1.5), tol)
}

/// Two unit bricks over `z` whose low corners in x and y are `lows`,
/// touching along the z-axis, and their records as carried rows.
fn quarter_pinch(
    lows: [(f64, f64); 2],
    z: (f64, f64),
    tol: Tol,
) -> (BooleanBody<f64>, topo::OperandRecords) {
    let [p, q] = lows.map(|(x, y)| {
        finished(
            "a quarter brick",
            brick((x, x + 1.0), (y, y + 1.0), z, tol),
            tol,
        )
    });
    let BooleanResult::Body(pinch) = union(&p, &q, tol).expect("an edge contact builds") else {
        panic!("a union of two bricks came back empty");
    };
    assert_pinch_records(&pinch.contacts);
    let carried = carried_rows(&pinch.contacts);
    (pinch, carried)
}

/// A pinch's records: a v-v row at each end of the touch, and the
/// edge-edge row along it citing exactly the two ends' decisions (C3's
/// shortest chain for a pair two vertex identities imply). Red if the
/// reduction writes no edge-edge row for the touch, or cites one end
/// alone.
fn assert_pinch_records(records: &topo::ContactRecords) {
    let rows = records.rows();
    let of = |want: fn(&topo::ContactKind) -> bool| -> Vec<&topo::Cited<topo::Contact>> {
        rows.iter().filter(|c| want(&c.record.kind())).collect()
    };
    let vv = of(|k| matches!(k, topo::ContactKind::VertexVertex(..)));
    let ee = of(|k| matches!(k, topo::ContactKind::EdgeEdge(..)));
    assert_eq!(
        (vv.len(), ee.len(), rows.len()),
        (2, 1, 3),
        "the edge contact records a v-v row at each end and the edge-edge row along it: {rows:?}"
    );
    assert_eq!(
        ee[0].cites,
        vv[0].cites.union(&vv[1].cites),
        "the edge-edge row cites both ends' decisions: {rows:?}"
    );
}

/// **A pinch's edge-edge row is read off its shared ends.** Two bricks
/// touching along an edge pass 3′ with their own records. The union
/// put each end's two vertices on one point key (the v-v rows it
/// decided), so the overlap between the two shared keys is structure,
/// and the census passes it without the edge-edge row; the row stays,
/// citing both ends' decisions, for the `unproven-coincidence` lint.
/// Red if an end's two vertices sit on two keys. The shared ends
/// are ratified on PR 4793: the records are the lint's evidence, not
/// the census's.
#[test]
fn a_pinch_without_its_edge_edge_row_reads_its_shared_ends() {
    let tol = Tol::witness();
    let (pinch, _) = pinch(tol);
    let verdict = validate_pseudomanifold(&pinch.body, &pinch.contacts, tol);
    assert!(verdict.is_ok(), "with its records: {verdict:?}");
    let bounds: topo::ContactRecords = pinch
        .contacts
        .rows()
        .iter()
        .filter(|c| !matches!(c.record.kind(), topo::ContactKind::EdgeEdge(..)))
        .cloned()
        .collect();
    for (x, y) in pinch.contacts.vertex_vertex() {
        let key = |v| pinch.body.get_vertex(v).unwrap().point;
        assert_eq!(key(x), key(y), "an end's two vertices share one key");
    }
    assert_eq!(
        validate_pseudomanifold(&pinch.body, &bounds, tol),
        Ok(()),
        "the overlap between two shared keys reads as structure (the shared ends \
         ratified on PR 4793: the records are the lint's evidence, not the census's)"
    );
}

/// The unit-radius wedge from `d0` to `d1` degrees (counterclockwise,
/// under 180°) about the z-axis, over `z`.
fn wedge(d0: f64, d1: f64, z: (f64, f64), tol: Tol) -> AtRestBody<f64> {
    let at = |deg: f64| (deg.to_radians().cos(), deg.to_radians().sin());
    let body = common::prism_z(&[(0.0, 0.0), at(d0), at(d1)], z.0, z.1, tol).body;
    finished("a wedge", body, tol)
}

/// The wedge from 80° to 190° over z ∈ (0.5, 1), sheared by `shear`
/// per unit of height above 0.5: its corner stays at the pinch's lower
/// end and its vertical edge leans off the pinch line.
fn sheared_wedge(shear: (f64, f64), tol: Tol) -> AtRestBody<f64> {
    sheared(triangle(80.0, 190.0), shear, tol)
}

/// The prism over `tri` at z ∈ (0.5, 1), sheared by `shear` per unit
/// of height above 0.5.
fn sheared(tri: [(f64, f64); 3], shear: (f64, f64), tol: Tol) -> AtRestBody<f64> {
    let mut body = Body::<f64>::new();
    common::prism_ops(
        &mut body,
        &tri,
        (0.5, 1.0),
        |x, y, z| geom_core::Point3::new(x + shear.0 * (z - 0.5), y + shear.1 * (z - 0.5), z),
        common::FaceGeometry::Certified,
        tol,
    );
    common::describe_as_intersections(&mut body, tol);
    finished("a sheared prism", body, tol)
}

/// The vertices of `body` at `p`, sorted.
fn keys_at(body: &Body<f64>, p: (f64, f64, f64)) -> Vec<topo::VertexKey> {
    let mut keys: Vec<topo::VertexKey> = body
        .vertex_points()
        .filter(|(_, q)| {
            let q = *q;
            (q.x, q.y, q.z) == p
        })
        .map(|(k, _)| k)
        .collect();
    keys.sort();
    keys
}

/// Each op of `pinch` and `cutter`, in both operand orders and with
/// `carried` as the pinch's own records, builds at its volume and
/// passes 3′: `pinch ∪ cutter`, `pinch ∖ cutter`, `pinch ∩ cutter` and
/// `cutter ∖ pinch`. An op given no volume refuses
/// `BooleanError::LineWedges` instead: its result carries coincident
/// edges on the pinch line whose wedges overlap, where the material
/// pairs the faces meeting there across the operands (`cutter ∖ pinch`
/// reverting the pinch, or a union pairing each operand's own faces).
fn every_op_builds(
    pinch: &AtRestBody<f64>,
    carried: &topo::OperandRecords,
    cutter: &AtRestBody<f64>,
    [union, pinch_less, common]: [Option<f64>; 3],
    cutter_less: Option<f64>,
    tol: Tol,
) {
    let ab = BooleanInputs {
        records_a: carried.clone(),
        ..BooleanInputs::default()
    };
    let ba = BooleanInputs {
        records_b: carried.clone(),
        ..BooleanInputs::default()
    };
    for (op, got, want) in topo::test_support::ops_under_test(|| {
        [
            ("pinch ∪ cutter", union_in(pinch, cutter, &ab, tol), union),
            (
                "pinch ∖ cutter",
                subtract_in(pinch, cutter, &ab, tol),
                pinch_less,
            ),
            (
                "pinch ∩ cutter",
                intersect_in(pinch, cutter, &ab, tol),
                common,
            ),
            ("cutter ∪ pinch", union_in(cutter, pinch, &ba, tol), union),
            (
                "cutter ∖ pinch",
                subtract_in(cutter, pinch, &ba, tol),
                cutter_less,
            ),
            (
                "cutter ∩ pinch",
                intersect_in(cutter, pinch, &ba, tol),
                common,
            ),
        ]
    }) {
        let Some(want) = want else {
            assert!(
                matches!(
                    got,
                    Err(BooleanError::LineWedges {
                        fault: topo::LineWedgeFault::Overlap { .. },
                        ..
                    })
                ),
                "{op}: {got:?}, not the overlap's refusal"
            );
            continue;
        };
        let BooleanResult::Body(out) = got.unwrap_or_else(|e| panic!("{op} refused: {e:?}")) else {
            panic!("{op} came back empty");
        };
        assert_classes(
            &out,
            operands(op, &[("pinch", pinch), ("cutter", cutter)]),
            op,
            tol,
        );
        let volume = mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
            .expect("mass")
            .met()
            .expect("the pinned resolution is met")
            .volume;
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
        [Some(2.0 + outside), Some(2.0 - inside), Some(inside)],
        Some(outside),
        tol,
    );
}

/// The triangle of a unit-radius wedge from `d0` to `d1` degrees.
fn triangle(d0: f64, d1: f64) -> [(f64, f64); 3] {
    let at = |deg: f64| (deg.to_radians().cos(), deg.to_radians().sin());
    [(0.0, 0.0), at(d0), at(d1)]
}

/// The area of a counterclockwise polygon.
fn area(p: &[(f64, f64)]) -> f64 {
    let n = p.len();
    (0..n)
        .map(|i| p[i].0 * p[(i + 1) % n].1 - p[(i + 1) % n].0 * p[i].1)
        .sum::<f64>()
        / 2.0
}

/// `subject` clipped to the convex counterclockwise `window`
/// (Sutherland–Hodgman): the volumes below are read off it, outside
/// the kernel.
fn clip(subject: &[(f64, f64)], window: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut out = subject.to_vec();
    for k in 0..window.len() {
        let (a, b) = (window[k], window[(k + 1) % window.len()]);
        let side = |p: (f64, f64)| (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
        let cross = |p: (f64, f64), q: (f64, f64)| {
            let t = side(p) / (side(p) - side(q));
            (p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1))
        };
        let input = std::mem::take(&mut out);
        for i in 0..input.len() {
            let (p, q) = (input[i], input[(i + 1) % input.len()]);
            match (side(p) >= 0.0, side(q) >= 0.0) {
                (true, true) => out.push(q),
                (true, false) => out.push(cross(p, q)),
                (false, true) => out.extend([cross(p, q), q]),
                (false, false) => {}
            }
        }
    }
    out
}

/// Wedges about the z-axis over z ∈ (0.5, 1.5), folded into one body by
/// unions with each step's records carried: they touch only along the
/// axis, a pinch with a vertex per wedge at each end. Cut by the wedge
/// over `cutter` at z ∈ (0.5, 1), which crosses into the outer two and
/// holds the ones between, every op builds at the volume the wedges'
/// triangles clipped to the cutter's give, but `cutter ∖ pinch`, and
/// the unions where `union_overlaps`, which refuse their overlapping
/// wedges ([`every_op_builds`]). A nonzero `shear` leans the cutter by
/// that much per unit of height ([`sheared`]).
fn crossings_at_one_corner(spans: &[(f64, f64)], cutter: (f64, f64), shear: (f64, f64), tol: Tol) {
    let fold = |acc: &BooleanBody<f64>, next: &AtRestBody<f64>| {
        let decls = BooleanInputs {
            records_a: carried_rows(&acc.contacts),
            ..BooleanInputs::default()
        };
        match union_in(&acc.body, next, &decls, tol).expect("a touching wedge folds in") {
            BooleanResult::Body(out) => out,
            BooleanResult::Empty => panic!("a union of wedges came back empty"),
        }
    };
    let wedges: Vec<AtRestBody<f64>> = spans
        .iter()
        .map(|&(d0, d1)| wedge(d0, d1, (0.5, 1.5), tol))
        .collect();
    let BooleanResult::Body(two) =
        union(&wedges[0], &wedges[1], tol).expect("two wedges touching along an edge build")
    else {
        panic!("a union of wedges came back empty");
    };
    let pinch = wedges[2..].iter().fold(two, |acc, next| fold(&acc, next));
    let n = spans.len();
    assert_eq!(
        pinch.contacts.vertex_vertex().count(),
        n * (n - 1),
        "{n} vertices at each end of the pinch line, a row per two"
    );
    assert_eq!(
        pinch.contacts.edge_edge().count(),
        n * (n - 1) / 2,
        "{n} edges along the pinch line, an edge-edge row per two"
    );
    let cut = triangle(cutter.0, cutter.1);
    let pinch_volume: f64 = spans.iter().map(|&(d0, d1)| area(&triangle(d0, d1))).sum();
    let cutter_volume = area(&cut) * 0.5;
    let common: f64 = spans
        .iter()
        .map(|&(d0, d1)| swept_common(&cut, shear, &triangle(d0, d1)))
        .sum();
    let cutter_body = if shear == (0.0, 0.0) {
        wedge(cutter.0, cutter.1, (0.5, 1.0), tol)
    } else {
        sheared(cut, shear, tol)
    };
    every_op_builds(
        &pinch.body,
        &carried_rows(&pinch.contacts),
        &cutter_body,
        [
            Some(pinch_volume + cutter_volume - common),
            Some(pinch_volume - common),
            Some(common),
        ],
        None,
        tol,
    );
}

/// **Three crossing pairs at one corner, in every op.** Three wedges
/// (0°–60°, 90°–120°, 150°–210°) cut by one from 50° to 160°: the
/// middle wedge's corner cuts the prism's bottom corner between the
/// outer pairs' cuts as a dangling null edge. Each op, in both operand
/// orders, builds at its volume and passes 3′ with the pinch's records
/// carried. Red as the first row: `SharedVertexCrossings`. Red if a fan
/// is minted at the corner before the dangling null edge there (∪
/// panics), or if a dangling null edge refuses
/// whenever another pair cuts its corner.
#[test]
fn three_crossings_at_one_corner_build_in_every_op() {
    crossings_at_one_corner(
        &[(0.0, 60.0), (90.0, 120.0), (150.0, 210.0)],
        (50.0, 160.0),
        (0.0, 0.0),
        Tol::witness(),
    );
}

/// **Four crossing pairs at one corner, two of them dangling.** Four
/// wedges (0°–30°, 60°–80°, 100°–120°, 150°–200°) cut by one from 20°
/// to 160°: the middle two corners hang two dangling null edges in the
/// prism's bottom corner, between the outer pairs' cuts. Each op, in
/// both operand orders, builds at its volume and passes 3′. Red if a
/// dangling null edge splices at its corner's own successor, ahead of
/// one hung there before it at an earlier germ (∪ refuses
/// `JoinDesync`), and as the three's.
#[test]
fn four_crossings_at_one_corner_build_in_every_op() {
    crossings_at_one_corner(
        &[(0.0, 30.0), (60.0, 80.0), (100.0, 120.0), (150.0, 200.0)],
        (20.0, 160.0),
        (0.0, 0.0),
        Tol::witness(),
    );
}

/// **Two pinches crossing on one line refuse their union as
/// non-manifold, and build the rest.** Four bricks in the four quadrants about the
/// z-axis, paired into two pinches that overlap over z ∈ (1, 1.5). The
/// union is a pinch below the overlap and another above it. Below, the
/// two coincident pinch edges each pair one brick's two faces, which
/// separates their ends into two vertices at (0,0,0.5), the bricks'
/// corners; at (0,0,1) each empty corner is bounded by one face of each
/// brick, so the two edges' ends there lie in one vertex orbit, which
/// passes the pinch line twice. That shared-entity wedge fan is
/// unrepresentable under tier 3′, and the reduction refuses it
/// `NonManifoldResult` where two crossing pairs share both their
/// vertices, naming A's vertex at (0,0,1) and two of B's. Red if that case
/// reaches the finish: the zip fuses a vertex onto an edge's other end
/// and refuses `Euler(SelfLoopEdge)`. The pinches' interiors are
/// disjoint, so each difference is its minuend at volume 2 and passes
/// 3′ with both records carried, and the intersection is empty.
#[test]
fn two_pinches_crossing_on_one_line_refuse_their_union_typed() {
    let tol = Tol::witness();
    let (pinch_a, carried_a) = pinch(tol);
    let (pinch_b, carried_b) = quarter_pinch([(-1.0, 0.0), (0.0, -1.0)], (1.0, 2.0), tol);
    let minuend = 2.0 * 1.0 * 1.0;
    for (name, a, b, ca, cb) in [
        ("a, b", &pinch_a, &pinch_b, &carried_a, &carried_b),
        ("b, a", &pinch_b, &pinch_a, &carried_b, &carried_a),
    ] {
        let decls = BooleanInputs {
            records_a: ca.clone(),
            records_b: cb.clone(),
        };
        // At (0,0,1) the lower pinch's two vertices are its split edges'
        // and the upper pinch's are its corners; every pair crosses.
        // The refusal names A's vertex there and the two B vertices it
        // crosses into.
        let (a_at, b_at) = (
            keys_at(&a.body, (0.0, 0.0, 1.0)),
            keys_at(&b.body, (0.0, 0.0, 1.0)),
        );
        match union_in(&a.body, &b.body, &decls, tol) {
            Err(BooleanError::NonManifoldResult {
                a_vertex: vertex,
                b_vertices: [p0, p1],
            }) => {
                let mut partners = vec![p0, p1];
                partners.sort();
                assert_ne!(p0, p1, "{name} ∪: two distinct partners");
                if b_at.is_empty() {
                    assert!(a_at.contains(&vertex), "{name} ∪: A's corner at (0,0,1)");
                } else {
                    assert_eq!(partners, b_at, "{name} ∪: B's two corners at (0,0,1)");
                }
            }
            other => panic!(
                "{name} ∪: want NonManifoldResult, got {:?}",
                other.map(|_| ())
            ),
        }
        let less = subtract_in(&a.body, &b.body, &decls, tol);
        let BooleanResult::Body(less) = less.unwrap_or_else(|e| panic!("{name} ∖ refused: {e:?}"))
        else {
            panic!("{name} ∖ came back empty");
        };
        let volume = mass_properties(&less.body, tol, topo::test_support::pinned_resolution(tol))
            .expect("the difference has mass")
            .met()
            .expect("the pinned resolution is met")
            .volume;
        assert!(
            (volume - minuend).abs() < 1e-9,
            "{name} ∖: volume {volume}, want {minuend}"
        );
        let verdict = validate_pseudomanifold(&less.body, &less.contacts, tol);
        assert!(verdict.is_ok(), "{name} ∖: 3′ refused {:?}", verdict.err());
        let common = intersect_in(&a.body, &b.body, &decls, tol);
        assert!(
            matches!(common, Ok(BooleanResult::Empty)),
            "{name} ∩: want empty, got {:?}",
            common.map(|_| ())
        );
    }
}

/// **Two pinches meeting end to end build in every op.** The pinch in
/// the first and third quadrants over z ∈ (0.5, 1) and the one in the
/// second and fourth over z ∈ (1, 2) meet at (0,0,1) alone. Every
/// brick's corner there stays its own, touching each neighbour along
/// an edge on the plane z = 1 and its diagonal opposite at the point,
/// so the union holds four vertices at (0,0,1) and passes 3′ with both
/// pinches' records carried. Interiors are disjoint: the union is the
/// sum of 1 and 2, each difference its minuend, the intersection empty.
#[test]
fn two_pinches_meeting_end_to_end_build_in_every_op() {
    let tol = Tol::witness();
    let (low, low_rows) = quarter_pinch([(0.0, 0.0), (-1.0, -1.0)], (0.5, 1.0), tol);
    let (high, high_rows) = quarter_pinch([(-1.0, 0.0), (0.0, -1.0)], (1.0, 2.0), tol);
    let (low_volume, high_volume) = (2.0 * 0.5, 2.0 * 1.0);
    for (name, a, b, ca, cb, minuend) in [
        ("low, high", &low, &high, &low_rows, &high_rows, low_volume),
        ("high, low", &high, &low, &high_rows, &low_rows, high_volume),
    ] {
        let decls = BooleanInputs {
            records_a: ca.clone(),
            records_b: cb.clone(),
        };
        let built = |op: &str, got: Result<BooleanResult<f64>, BooleanError>, want: f64| {
            let BooleanResult::Body(out) =
                got.unwrap_or_else(|e| panic!("{name} {op} refused: {e:?}"))
            else {
                panic!("{name} {op} came back empty");
            };
            let volume =
                mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
                    .expect("the result has mass")
                    .met()
                    .expect("the pinned resolution is met")
                    .volume;
            assert!(
                (volume - want).abs() < 1e-9,
                "{name} {op}: volume {volume}, want {want}"
            );
            let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
            assert!(
                verdict.is_ok(),
                "{name} {op}: 3′ refused {:?}",
                verdict.err()
            );
            out
        };
        let union = built(
            "∪",
            union_in(&a.body, &b.body, &decls, tol),
            low_volume + high_volume,
        );
        assert_eq!(
            keys_at(&union.body, (0.0, 0.0, 1.0)).len(),
            4,
            "{name} ∪: each brick's corner at (0,0,1) is its own vertex"
        );
        built("∖", subtract_in(&a.body, &b.body, &decls, tol), minuend);
        let common = intersect_in(&a.body, &b.body, &decls, tol);
        assert!(
            matches!(common, Ok(BooleanResult::Empty)),
            "{name} ∩: want empty, got {:?}",
            common.map(|_| ())
        );
    }
}

/// `∫ area(clip(triangle + shear·(z − 0.5), window)) dz` over
/// z ∈ (0.5, 1), outside the kernel. The clipped area is quadratic in
/// z between the heights where a vertex of one polygon crosses an
/// edge line of the other, so Simpson's rule on each piece is exact.
fn swept_common(tri: &[(f64, f64)], shear: (f64, f64), window: &[(f64, f64)]) -> f64 {
    let (z0, z1) = (0.5, 1.0);
    let mut breaks = vec![z0, z1];
    let mut push = |num: f64, den: f64| {
        if den != 0.0 {
            let z = z0 + num / den;
            if z > z0 && z < z1 {
                breaks.push(z);
            }
        }
    };
    let cross = |u: (f64, f64), v: (f64, f64)| u.0 * v.1 - u.1 * v.0;
    let sub = |u: (f64, f64), v: (f64, f64)| (u.0 - v.0, u.1 - v.1);
    // A vertex of the moving triangle on an edge line of the window,
    // and a window vertex on an edge line of the moving triangle.
    for (fixed, moving, sign) in [(window, tri, -1.0), (tri, window, 1.0)] {
        for k in 0..fixed.len() {
            let (p, q) = (fixed[k], fixed[(k + 1) % fixed.len()]);
            for &v in moving {
                push(sign * cross(sub(q, p), sub(v, p)), cross(sub(q, p), shear));
            }
        }
    }
    breaks.sort_by(f64::total_cmp);
    let at = |z: f64| {
        let t = z - z0;
        let moved: Vec<(f64, f64)> = tri
            .iter()
            .map(|&(x, y)| (x + shear.0 * t, y + shear.1 * t))
            .collect();
        let got = clip(&moved, window);
        if got.len() < 3 { 0.0 } else { area(&got) }
    };
    breaks
        .windows(2)
        .map(|w| (w[1] - w[0]) / 6.0 * (at(w[0]) + 4.0 * at((w[0] + w[1]) / 2.0) + at(w[1])))
        .sum()
}

/// **A pinch line lying flat in the shared corner's face, in every
/// op.** The 80°–190° wedge sheared along its 80° face by half a unit
/// per unit of height, backwards, so the z-axis runs inside that face
/// from the corner: both pairs' cuts lie along the axis in one corner
/// of the prism's corner, the brick in the first quadrant's run on the
/// face's side toward the 80° bottom edge and the third quadrant's
/// toward the vertical edge. Each op, in both operand orders, builds
/// at the volume a polygon clip of the sheared triangle against each
/// brick's square gives, and passes 3′ with the pinch's records
/// carried. Red as the first row: the tied cuts each read as held, and
/// all six refuse `SharedVertexCrossings`. Red in both unions and
/// `cutter ∖ pinch` if the output stage does not merge the sliver of no
/// area the join leaves on the pinch line below `z = 1`: the result
/// carries the line as two edges, overlapping, and refuses
/// `LineWedges`.
#[test]
fn a_pinch_line_flat_in_the_shared_corners_face_builds_in_every_op() {
    let tol = Tol::witness();
    let (pinch, carried) = pinch(tol);
    let (c, s) = (80f64.to_radians().cos(), 80f64.to_radians().sin());
    let shear = (-0.5 * c, -0.5 * s);
    let tri = triangle(80.0, 190.0);
    let squares = [
        [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        [(-1.0, -1.0), (0.0, -1.0), (0.0, 0.0), (-1.0, 0.0)],
    ];
    let common: f64 = squares.iter().map(|q| swept_common(&tri, shear, q)).sum();
    let cutter = area(&tri) * 0.5;
    every_op_builds(
        &pinch.body,
        &carried,
        &sheared_wedge(shear, tol),
        [
            Some(2.0 + cutter - common),
            Some(2.0 - common),
            Some(common),
        ],
        Some(cutter - common),
        tol,
    );
}

/// **A pinch line in the shared corner's face, that face flush with
/// both bricks.** The wedge from 90° to 200° or 250°, sheared along its
/// 90° face by half a unit per unit of height, toward −y, so that face
/// lies in x = 0 with the z-axis inside its corner; and its mirror, the
/// wedge from 270° to 20° sheared toward +y. Both bricks' faces x = 0
/// lie flush with the corner's face, one on each side of the axis, so
/// both pairs' cuts lie along the axis in one entry, each along one of
/// the pinch's two edges, and the corner's orbit orders them by the
/// side of the line each edge's flush face lies on wherever it reads a
/// position: the runs' ends and the cuts each holds
/// (`insert::walks_before`). Each op, in both operand orders, builds at
/// the volume a polygon clip of the sheared triangle against each
/// brick's square gives, and passes 3′ with the pinch's records
/// carried.
#[test]
fn a_pinch_line_in_a_face_flush_with_both_bricks_builds_in_every_op() {
    let tol = Tol::witness();
    let (pinch, carried) = pinch(tol);
    let squares = [
        [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        [(-1.0, -1.0), (0.0, -1.0), (0.0, 0.0), (-1.0, 0.0)],
    ];
    for (span, shear) in [
        ((90.0, 200.0), (0.0, -0.5)),
        ((90.0, 250.0), (0.0, -0.5)),
        ((270.0, 380.0), (0.0, 0.5)),
    ] {
        let tri = triangle(span.0, span.1);
        let common: f64 = squares.iter().map(|q| swept_common(&tri, shear, q)).sum();
        let cutter = area(&tri) * 0.5;
        every_op_builds(
            &pinch.body,
            &carried,
            &sheared(tri, shear, tol),
            [
                Some(2.0 + cutter - common),
                Some(2.0 - common),
                Some(common),
            ],
            Some(cutter - common),
            tol,
        );
    }
}

/// **Three pieces, two of whose cuts tie.** Wedges 0°–60°, 90°–120°
/// and 150°–240° about the z-axis, cut by the wedge from 50° to 160°
/// sheared backwards along its 50° face, so the pinch line runs flat
/// in that face: the outer two pieces' cuts lie along it, on its two
/// sides, and the middle piece hangs a dangling null edge in the
/// prism's bottom corner. Each op, in both operand orders, builds at
/// the volume a polygon clip of the sheared triangle against each
/// wedge's gives, and passes 3′ with the pinch's records carried. Red
/// as the two-piece tie, and in both unions if the output stage does not
/// merge the sliver of no area the join leaves on the pinch line.
/// `cutter ∖ pinch` refuses `LineWedges` instead: edges of 170°, 100°
/// and 330°, overlapping, which every tier passed.
#[test]
fn three_pieces_two_of_whose_cuts_tie_build_in_every_op() {
    let (c, s) = (50f64.to_radians().cos(), 50f64.to_radians().sin());
    crossings_at_one_corner(
        &[(0.0, 60.0), (90.0, 120.0), (150.0, 240.0)],
        (50.0, 160.0),
        (-0.5 * c, -0.5 * s),
        Tol::witness(),
    );
}

/// **A pinch line crossing a face's interior keeps the pinch's records
/// at the new pinch end.** The 80°–190° wedge sheared off the axis, so
/// the pinch line runs inside its corner and leaves through its top
/// face, and a brick holding the axis, which its top face crosses. Each
/// op but `cutter ∖ pinch` builds, the sheared one's volumes agreeing
/// with each other, and passes 3′: where a result keeps the pinch's two
/// pieces at the top face's crossing, the carried edge-edge row's split
/// lineage records the two split vertices there. Red if the pinch
/// records no edge-edge row, or the row is not carried: 3′ refuses
/// `VertexVertex` at (0,0,1) in `pinch ∖ cutter`, both intersections and
/// `block ∩ pinch`.
///
/// `cutter ∖ pinch` keeps the pinch line inside the cutter, where the
/// complement of the pinch's two quadrants is the other two: the
/// pinch's two edges re-pair across the bricks, which ∖ does not build:
/// the result's two edges there bound 270° each, overlapping, and it
/// refuses `BooleanError::LineWedges`.
#[test]
fn a_pinch_line_through_a_face_keeps_its_records_at_the_new_end() {
    let tol = Tol::witness();
    let (pinch, carried) = pinch(tol);
    let cutter = sheared_wedge((0.3, -0.3), tol);
    let block = finished(
        "block",
        brick::<f64>((-0.5, 0.5), (-0.5, 0.5), (0.0, 1.0), tol),
        tol,
    );
    let ab = BooleanInputs {
        records_a: carried.clone(),
        ..BooleanInputs::default()
    };
    let ba = BooleanInputs {
        records_b: carried.clone(),
        ..BooleanInputs::default()
    };
    let block_first = BooleanInputs {
        records_b: carried,
        ..BooleanInputs::default()
    };
    let volume_of = |op: &str, got: Result<BooleanResult<f64>, BooleanError>| {
        let BooleanResult::Body(out) = got.unwrap_or_else(|e| panic!("{op} refused: {e:?}")) else {
            panic!("{op} came back empty");
        };
        assert_classes(
            &out,
            operands(
                op,
                &[
                    ("pinch", &pinch.body),
                    ("cutter", &cutter),
                    ("block", &block),
                ],
            ),
            op,
            tol,
        );
        let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
        assert!(verdict.is_ok(), "{op}: 3′ refused {:?}", verdict.err());
        mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
            .expect("mass")
            .met()
            .expect("the pinned resolution is met")
            .volume
    };
    let union = volume_of("pinch ∪ cutter", union_in(&pinch.body, &cutter, &ab, tol));
    let pinch_less = volume_of(
        "pinch ∖ cutter",
        subtract_in(&pinch.body, &cutter, &ab, tol),
    );
    let common = volume_of(
        "pinch ∩ cutter",
        intersect_in(&pinch.body, &cutter, &ab, tol),
    );
    let swapped = volume_of("cutter ∪ pinch", union_in(&cutter, &pinch.body, &ba, tol));
    let refusal = subtract_in(&cutter, &pinch.body, &ba, tol);
    assert!(
        matches!(
            refusal,
            Err(BooleanError::LineWedges {
                fault: topo::LineWedgeFault::Overlap { .. },
                ..
            })
        ),
        "cutter ∖ pinch: {refusal:?}, not the overlap's refusal"
    );
    let common_swapped = volume_of(
        "cutter ∩ pinch",
        intersect_in(&cutter, &pinch.body, &ba, tol),
    );
    // Shearing keeps the cutter's volume: its triangle × the height.
    let cutter_volume = area(&triangle(80.0, 190.0)) * 0.5;
    for (what, got, want) in [
        ("pinch ∖ cutter + ∩", pinch_less + common, 2.0),
        ("∪", union, 2.0 + cutter_volume - common),
        ("∪ swapped", swapped, union),
        ("∩ swapped", common_swapped, common),
    ] {
        assert!((got - want).abs() < 1e-9, "{what}: {got}, want {want}");
    }
    let block_common = volume_of(
        "block ∩ pinch",
        intersect_in(&block, &pinch.body, &block_first, tol),
    );
    assert!(
        (block_common - 0.25).abs() < 1e-9,
        "block ∩ pinch: {block_common}, want two quarter-unit squares over z ∈ (0.5, 1)"
    );
}

/// **A face through the pinch line, in every op and both operand
/// orders.** Two bricks touching along the z-axis (the pinch above)
/// against a prism whose diagonal wall holds that axis: each of the
/// wall's vertices on the axis pairs with both of the pinch's
/// vertices there, one pair crossing and one touching. Every op
/// builds at its volume, and with the pinch's records carried the
/// result passes 3′. Red as the first row: a panic at the prism's
/// vertex on the axis, in all four. The subtract of the wall
/// over z ∈ (0.75, 1.25) leaves the pinch a new end at its cut: red if
/// the result's v-v rows leave out the null-edge copies there (3′
/// refuses `VertexVertex` at (0,0,0.75), the corner closing the cut
/// brick's lower piece being a copy no row names).
#[test]
fn a_face_through_a_pinch_line_builds_in_every_op() {
    let tol = Tol::witness();
    let (pinch, carried) = pinch(tol);
    let wall = finished(
        "wall",
        common::prism_z(&[(-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)], 0.0, 1.0, tol).body,
        tol,
    );
    let ab = BooleanInputs {
        records_a: carried.clone(),
        ..BooleanInputs::default()
    };
    let ba = BooleanInputs {
        records_b: carried.clone(),
        ..BooleanInputs::default()
    };
    // The same wall over z ∈ (0.75, 1.25) cuts the brick on its side
    // in two and leaves the other whole: the pinch survives below 0.75
    // with a new pair of vertices at its cut.
    let upper = finished(
        "upper",
        common::prism_z(&[(-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)], 0.75, 1.25, tol).body,
        tol,
    );
    let cut = BooleanInputs {
        records_a: carried,
        ..BooleanInputs::default()
    };
    // The prism holds the half x + y > 0 of the square, so it meets
    // the pinch in the brick on that side, z ∈ (0.5, 1): 0.5, and the
    // upper one over z ∈ (0.75, 1.25), 0.5 again.
    for (op, got, want) in [
        (
            "pinch ∖ upper wall",
            subtract_in(&pinch.body, &upper, &cut, tol),
            1.5,
        ),
        ("pinch ∪ wall", union_in(&pinch.body, &wall, &ab, tol), 3.5),
        (
            "pinch ∖ wall",
            subtract_in(&pinch.body, &wall, &ab, tol),
            1.5,
        ),
        (
            "pinch ∩ wall",
            intersect_in(&pinch.body, &wall, &ab, tol),
            0.5,
        ),
        ("wall ∪ pinch", union_in(&wall, &pinch.body, &ba, tol), 3.5),
    ] {
        let BooleanResult::Body(out) = got.unwrap_or_else(|e| panic!("{op} refused: {e:?}")) else {
            panic!("{op} came back empty");
        };
        assert_classes(
            &out,
            operands(
                op,
                &[("pinch", &pinch.body), ("upper", &upper), ("wall", &wall)],
            ),
            op,
            tol,
        );
        let volume = mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
            .expect("mass")
            .met()
            .expect("the pinned resolution is met")
            .volume;
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
    let q2 = finished(
        "q2",
        brick::<f64>((-1.0, 0.0), (0.0, 1.0), (1.0, 2.0), tol),
        tol,
    );
    let q4 = finished(
        "q4",
        brick::<f64>((0.0, 1.0), (-1.0, 0.0), (1.0, 2.0), tol),
        tol,
    );
    let BooleanResult::Body(pinch_b) = union(&q2, &q4, tol).expect("a pinch builds") else {
        panic!("a union of two bricks came back empty");
    };
    let decls = BooleanInputs {
        records_a: carried_a,
        records_b: carried_rows(&pinch_b.contacts),
    };
    let BooleanResult::Body(out) =
        subtract_in(&pinch_a.body, &pinch_b.body, &decls, tol).expect("the subtract builds")
    else {
        panic!("the subtract of disjoint quadrants came back empty");
    };
    let volume = mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
        .expect("mass")
        .met()
        .expect("the pinned resolution is met")
        .volume;
    assert!((volume - 2.0).abs() < 1e-9, "volume {volume}, want 2.0");
    let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
    assert!(verdict.is_ok(), "3′ refused {:?}", verdict.err());
}

/// The triangular prism whose corner at the origin spans `rays`
/// (a right-handed triple), scaled by `scale`: the linear image of the
/// unit right prism, so its faces stay planar and its corner cone is
/// the rays' own.
pub(crate) fn corner_prism(rays: [[f64; 3]; 3], scale: f64, tol: Tol) -> AtRestBody<f64> {
    let mut body = Body::<f64>::new();
    common::prism_ops(
        &mut body,
        &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)],
        (0.0, 1.0),
        |x, y, z| {
            let p = |k: usize| scale * (x * rays[0][k] + y * rays[1][k] + z * rays[2][k]);
            geom_core::Point3::new(p(0), p(1), p(2))
        },
        common::FaceGeometry::Certified,
        tol,
    );
    common::describe_as_intersections(&mut body, tol);
    finished("a corner prism", body, tol)
}

/// The unit cube's corner at the origin against `y`'s, every op in
/// both operand orders, with `carried` as `y`'s records.
fn against_the_cube(
    y: &AtRestBody<f64>,
    carried: &topo::OperandRecords,
    tol: Tol,
) -> [(&'static str, BooleanOutcome); 6] {
    let cube = the_cube(tol);
    let ab = BooleanInputs {
        records_a: carried.clone(),
        ..BooleanInputs::default()
    };
    let ba = BooleanInputs {
        records_b: carried.clone(),
        ..BooleanInputs::default()
    };
    topo::test_support::ops_under_test(|| {
        [
            ("y ∪ cube", union_in(y, &cube, &ab, tol)),
            ("y ∖ cube", subtract_in(y, &cube, &ab, tol)),
            ("y ∩ cube", intersect_in(y, &cube, &ab, tol)),
            ("cube ∪ y", union_in(&cube, y, &ba, tol)),
            ("cube ∖ y", subtract_in(&cube, y, &ba, tol)),
            ("cube ∩ y", intersect_in(&cube, y, &ba, tol)),
        ]
    })
}

type BooleanOutcome = Result<BooleanResult<f64>, BooleanError>;

/// The unit cube [`against_the_cube`] meets.
fn the_cube(tol: Tol) -> AtRestBody<f64> {
    finished(
        "cube",
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    )
}

/// **Two dangling null edges meeting on the pinch line.** The pinch
/// against a prism along the diagonal x = y whose face in that plane
/// has a reflex corner at the pinch's lower end, its notch opening
/// downward (−40° to 200° from the face's horizontal): the pinch line
/// lies flat in the face, and each brick meets the face in a quarter
/// plane between the line and its bottom edge, wholly inside the
/// face's corner. So each pair's cut is a dangling null edge, the two
/// meeting along the line from opposite sides. Every op, in both
/// operand orders, builds at the volume a polygon clip of the prism's
/// rectangle against each brick's square gives (the prism is that
/// rectangle at every height of the pinch), and the result passes 3′
/// with the pinch's records carried. Red in `pinch ∖ prism` when a
/// carried row's ends do not reach their null-edge copies: the pinch's
/// top end lies inside the prism's face, and the result keeps both
/// pieces there with no v-v row. Red as the first row: each dangling null edge
/// read the other's cut along the line as held, and all six refused
/// `SharedVertexCrossings`. Red with the notch from −20° to 220° if a
/// dangling null edge splices by the germ its run leaves from rather
/// than its lower germ: both leave from the line, and all six refuse
/// the anchor's invariant. Red in both unions and `prism ∖ pinch` if
/// the output stage does not merge the sliver of no area the join
/// leaves on the pinch line: two edges of 225° each (135° each in the
/// difference), overlapping, and `LineWedges`.
#[test]
fn two_dangling_null_edges_meeting_on_the_pinch_line_build_in_every_op() {
    for notch in [(-40.0, 200.0), (-20.0, 220.0)] {
        dangling_null_edges_meeting_on_the_pinch_line(notch, Footing::Diagonal, Tol::witness());
    }
}

/// **Two dangling null edges meeting on the pinch line, in a face flush
/// with both bricks.** The notched prism of
/// [`two_dangling_null_edges_meeting_on_the_pinch_line_build_in_every_op`]
/// stood in the plane x = 0, extruded along +x or −x: its face there
/// lies flush with each brick's face x = 0, on either side of the pinch
/// line. Each pair's strut runs from its brick's edge along the line to
/// its bottom edge, so the two struts' ends on the line lie along one
/// direction in one entry, each along one of the pinch's two edges, and
/// the face's orbit orders them by the side of the line each edge's
/// flush face lies on wherever it reads a position: the struts' ends,
/// the cuts each holds and whether one holds the other whole
/// (`insert::walks_before`). Every op but `prism ∖ pinch`, in both
/// operand orders and at four notches, builds at the volume a polygon
/// clip of the prism's rectangle against each brick's square gives, and
/// passes 3′ with the pinch's records carried. `prism ∖ pinch` refuses
/// `LineWedges` on the bottom edge `y = 0, z = 0.5` of the brick the
/// prism holds, which runs along the notch's reflex edge: four sheets
/// meet there, and the prism's 240° less the brick's 90° is two wedges
/// of 40° and 110°, which the result's edges do not pair as
/// (`work/cleave/lumps-meeting-four-sheets-on-a-line-refuse-line-wedges.md`).
#[test]
fn two_dangling_null_edges_meeting_beside_the_pinch_line_build_in_every_op() {
    for footing in [Footing::Flush, Footing::FlushBack] {
        for notch in [
            (-40.0, 200.0),
            (-20.0, 220.0),
            (-60.0, 240.0),
            (-10.0, 190.0),
        ] {
            dangling_null_edges_meeting_on_the_pinch_line(notch, footing, Tol::witness());
        }
    }
}

/// Where [`dangling_null_edges_meeting_on_the_pinch_line`] stands its
/// prism.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Footing {
    /// In the plane x = y, extruded along (1, −1, 0)/√2.
    Diagonal,
    /// In the plane x = 0, extruded along +x.
    Flush,
    /// In the plane x = 0, extruded along −x.
    FlushBack,
}

impl Footing {
    /// `(x, y)` at the profile's width `w` and the depth `u` it is
    /// extruded through, a map of determinant −1.
    fn at(self, w: f64, u: f64) -> (f64, f64) {
        match self {
            Self::Diagonal => {
                let r = std::f64::consts::FRAC_1_SQRT_2;
                ((w + u) * r, (w - u) * r)
            }
            Self::Flush => (u, w),
            Self::FlushBack => (-u, -w),
        }
    }
}

/// The pinch against the prism stood at `footing` whose face's notch
/// runs from `notch.0` to `notch.1` degrees (docs above). Only the
/// prism in x = 0 holds a brick's bottom edge along its notch's reflex
/// edge, and its `prism ∖ pinch` refuses as its test says.
fn dangling_null_edges_meeting_on_the_pinch_line(notch: (f64, f64), footing: Footing, tol: Tol) {
    let (pinch, carried) = pinch(tol);
    let at = |deg: f64| {
        let (c, s) = (f64::to_radians(deg).cos(), f64::to_radians(deg).sin());
        (0.5 * c, 0.5 + 0.5 * s)
    };
    let (right, left) = (at(notch.0), at(notch.1));
    let profile = [
        (0.0, 0.5),
        right,
        (0.8, right.1),
        (0.8, 1.6),
        (-0.8, 1.6),
        (-0.8, left.1),
        left,
    ];
    // Profile (w, z), extruded through u ∈ (0, 1).
    let mut prism = Body::<f64>::new();
    common::prism_ops(
        &mut prism,
        &profile,
        (0.0, 1.0),
        |w, z, u| {
            let (x, y) = footing.at(w, u);
            geom_core::Point3::new(x, y, z)
        },
        common::FaceGeometry::Certified,
        tol,
    );
    common::describe_as_intersections(&mut prism, tol);
    let prism = finished("the notched prism", prism, tol);
    let rect: Vec<(f64, f64)> = [(-0.8, 1.0), (0.8, 1.0), (0.8, 0.0), (-0.8, 0.0)]
        .iter()
        .map(|&(w, u)| footing.at(w, u))
        .collect();
    let squares = [
        [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        [(-1.0, -1.0), (0.0, -1.0), (0.0, 0.0), (-1.0, 0.0)],
    ];
    let common: f64 = squares.iter().map(|q| area(&clip(&rect, q))).sum();
    let prism_volume = area(&profile);
    let ab = BooleanInputs {
        records_a: carried.clone(),
        ..BooleanInputs::default()
    };
    let ba = BooleanInputs {
        records_b: carried,
        ..BooleanInputs::default()
    };
    for (op, got, want) in topo::test_support::ops_under_test(|| {
        [
            (
                "pinch ∪ prism",
                union_in(&pinch.body, &prism, &ab, tol),
                2.0 + prism_volume - common,
            ),
            (
                "pinch ∖ prism",
                subtract_in(&pinch.body, &prism, &ab, tol),
                2.0 - common,
            ),
            (
                "pinch ∩ prism",
                intersect_in(&pinch.body, &prism, &ab, tol),
                common,
            ),
            (
                "prism ∪ pinch",
                union_in(&prism, &pinch.body, &ba, tol),
                2.0 + prism_volume - common,
            ),
            (
                "prism ∖ pinch",
                subtract_in(&prism, &pinch.body, &ba, tol),
                prism_volume - common,
            ),
            (
                "prism ∩ pinch",
                intersect_in(&prism, &pinch.body, &ba, tol),
                common,
            ),
        ]
    }) {
        let refuses = footing != Footing::Diagonal && op == "prism ∖ pinch";
        let op = format!("{op} (notch {notch:?}, {footing:?})");
        if refuses {
            assert!(
                matches!(
                    got,
                    Err(BooleanError::LineWedges {
                        fault: topo::LineWedgeFault::Overlap { .. },
                        ..
                    })
                ),
                "{op}: {:?}, not the overlap's refusal",
                got.map(|_| ())
            );
            continue;
        }
        let BooleanResult::Body(out) = got.unwrap_or_else(|e| panic!("{op} refused: {e:?}")) else {
            panic!("{op} came back empty");
        };
        assert_classes(
            &out,
            operands(&op, &[("pinch", &pinch.body), ("prism", &prism)]),
            &op,
            tol,
        );
        let volume = mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
            .expect("mass")
            .met()
            .expect("the pinned resolution is met")
            .volume;
        assert!(
            (volume - want).abs() < 1e-9,
            "{op}: volume {volume}, want {want}"
        );
        let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
        assert!(verdict.is_ok(), "{op}: 3′ refused {:?}", verdict.err());
    }
}

/// The faces of [`corner_prism`]`(rays, scale)` as vertex loops.
fn prism_faces(rays: [[f64; 3]; 3], scale: f64) -> Vec<Vec<[f64; 3]>> {
    let at = |u: f64, v: f64, w: f64| {
        core::array::from_fn(|k| scale * (u * rays[0][k] + v * rays[1][k] + w * rays[2][k]))
    };
    let (b, t) = (
        [at(0.0, 0.0, 0.0), at(1.0, 0.0, 0.0), at(0.0, 1.0, 0.0)],
        [at(0.0, 0.0, 1.0), at(1.0, 0.0, 1.0), at(0.0, 1.0, 1.0)],
    );
    let mut faces = vec![b.to_vec(), t.to_vec()];
    for k in 0..3 {
        let l = (k + 1) % 3;
        faces.push(vec![b[k], b[l], t[l], t[k]]);
    }
    faces
}

/// The half-spaces `n·p ≤ d` of the box `lo ≤ p ≤ hi`.
pub(crate) fn box_planes(lo: f64, hi: f64) -> Vec<([f64; 3], f64)> {
    (0..3)
        .flat_map(|k| {
            let e: [f64; 3] = core::array::from_fn(|j| if j == k { 1.0 } else { 0.0 });
            [(e, hi), (e.map(|x| -x), -lo)]
        })
        .collect()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// The volume of the convex polyhedron `faces` cut to `planes`, outside
/// the kernel: each face loop clipped to each half-space in turn
/// (Sutherland–Hodgman), the points on the plane closing the cut's cap
/// in angular order unless a face lies on it, then the pyramids from
/// the mean vertex summed. "On the plane" is exact equality, so a face
/// counts as lying on a plane only where its coordinates meet it
/// exactly, as the boxes' and the lenses' zero coordinates do.
pub(crate) fn clipped_volume(mut faces: Vec<Vec<[f64; 3]>>, planes: &[([f64; 3], f64)]) -> f64 {
    let area = |f: &[[f64; 3]]| {
        (1..f.len() - 1)
            .map(|i| cross(sub(f[i], f[0]), sub(f[i + 1], f[0])))
            .fold([0.0; 3], |s, a| core::array::from_fn(|k| s[k] + a[k]))
    };
    let mean = |ps: &[[f64; 3]]| -> [f64; 3] {
        core::array::from_fn(|k| ps.iter().map(|p| p[k]).sum::<f64>() / ps.len() as f64)
    };
    for &(n, d) in planes {
        let mut kept = Vec::new();
        let mut cap = Vec::new();
        for f in &faces {
            let mut g = Vec::new();
            for i in 0..f.len() {
                let (p, q) = (f[i], f[(i + 1) % f.len()]);
                let (sp, sq) = (dot(n, p) - d, dot(n, q) - d);
                if sp <= 0.0 {
                    g.push(p);
                }
                if sp == 0.0 {
                    cap.push(p);
                }
                if sp * sq < 0.0 {
                    let t = sp / (sp - sq);
                    let x = core::array::from_fn(|k| p[k] + t * (q[k] - p[k]));
                    g.push(x);
                    cap.push(x);
                }
            }
            if g.len() >= 3 {
                kept.push(g);
            }
        }
        // A face lying on the plane already closes the cut; a sliver
        // the clip left on it does not.
        let flat = kept
            .iter()
            .any(|g| g.iter().all(|&p| dot(n, p) == d) && dot(area(g), n) != 0.0);
        if cap.len() >= 3 && !flat {
            let c = mean(&cap);
            let u = sub(cap[0], c);
            let w = cross(n, u);
            cap.sort_by(|&a, &b| {
                let ang = |p| f64::atan2(dot(sub(p, c), w), dot(sub(p, c), u));
                ang(a).total_cmp(&ang(b))
            });
            kept.push(cap);
        }
        faces = kept;
    }
    let c = mean(&faces.concat());
    faces
        .iter()
        .map(|f| dot(area(f), sub(f[0], c)).abs() / 6.0)
        .sum()
}

/// The pit's corner: a cone at the origin crossing the cube's bottom
/// face, scaled out past the block.
const PIT: [[f64; 3]; 3] = [[1.0, 0.6, -0.3], [0.6, 1.0, -0.3], [1.0, 1.0, 0.5]];

/// **A dangling null edge whose segment holds other pairs' cuts, in
/// every op.** `y` is the block `[-1, 1]³` with a pyramidal pit whose
/// apex is the origin ([`PIT`]), and `spikes` in the pit, each
/// touching the block and each other only there, against the unit
/// cube's corner: the pit opens across the cube's bottom face, so the
/// pit's pair crosses that face's corner twice, and its run is a
/// dangling null edge on the Out side (its other way round is the
/// whole orbit) whose segment holds each spike's pair's In segment
/// whole. Each spike's dangling null edge hangs at the pit's tip.
/// Every op, in both operand orders, builds at the volume clipping
/// each prism to each box gives ([`clipped_volume`]), and the result
/// passes 3′ with `y`'s records carried, holding `records[k]` records in
/// [`against_the_cube`]'s `k`-th op: a contact and its record both absent
/// would pass 3′ where the census cannot see it.
fn pit_holding_spikes(spikes: &[([[f64; 3]; 3], f64)], records: [usize; 6], tol: Tol) {
    for spike_first in [false, true] {
        pit_holding_spikes_built(spikes, spike_first, records, tol);
    }
}

/// [`pit_holding_spikes`] with each spike's union taken spike first or
/// the holed block first.
fn pit_holding_spikes_built(
    spikes: &[([[f64; 3]; 3], f64)],
    spike_first: bool,
    records: [usize; 6],
    tol: Tol,
) {
    let pit = corner_prism(PIT, 3.0, tol);
    let block = finished(
        "block",
        brick::<f64>((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0), tol),
        tol,
    );
    let BooleanResult::Body(mut y) = subtract(&block, &pit, tol).expect("the pit is cut") else {
        panic!("the block ∖ pit came back empty");
    };
    for &(rays, scale) in spikes {
        let spike = corner_prism(rays, scale, tol);
        let got = if spike_first {
            let decls = BooleanInputs {
                records_b: carried_rows(&y.contacts),
                ..BooleanInputs::default()
            };
            union_in(&spike, &y.body, &decls, tol)
        } else {
            let decls = BooleanInputs {
                records_a: carried_rows(&y.contacts),
                ..BooleanInputs::default()
            };
            union_in(&y.body, &spike, &decls, tol)
        };
        let BooleanResult::Body(next) = got.expect("the spike touches the pit's apex") else {
            panic!("the union came back empty");
        };
        y = next;
    }
    let (cube, block) = (box_planes(0.0, 1.0), box_planes(-1.0, 1.0));
    let in_cube: f64 = spikes
        .iter()
        .map(|&(r, s)| clipped_volume(prism_faces(r, s), &cube))
        .sum();
    let whole: f64 = spikes
        .iter()
        .map(|&(r, s)| clipped_volume(prism_faces(r, s), &block))
        .sum();
    let common = 1.0 - clipped_volume(prism_faces(PIT, 3.0), &cube) + in_cube;
    let y_volume = 8.0 - clipped_volume(prism_faces(PIT, 3.0), &block) + whole;
    let got = mass_properties(&y.body, tol, topo::test_support::pinned_resolution(tol))
        .expect("mass")
        .met()
        .expect("the pinned resolution is met")
        .volume;
    assert!(
        (got - y_volume).abs() < 1e-9,
        "y: volume {got}, want {y_volume}"
    );
    assert_eq!(
        keys_at(&y.body, (0.0, 0.0, 0.0)).len(),
        1 + spikes.len(),
        "the pit's apex and each spike's corner"
    );
    let ops = against_the_cube(&y.body, &carried_rows(&y.contacts), tol);
    for ((op, got), records) in ops.into_iter().zip(records) {
        let want = match op {
            "y ∪ cube" | "cube ∪ y" => y_volume + 1.0 - common,
            "y ∖ cube" => y_volume - common,
            "cube ∖ y" => 1.0 - common,
            _ => common,
        };
        let op = format!("{op} (spike first: {spike_first})");
        let BooleanResult::Body(out) = got.unwrap_or_else(|e| panic!("{op} refused: {e:?}")) else {
            panic!("{op} came back empty");
        };
        assert_classes(
            &out,
            operands(&op, &[("y", &y.body), ("cube", &the_cube(tol))]),
            &op,
            tol,
        );
        let volume = mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
            .expect("mass")
            .met()
            .expect("the pinned resolution is met")
            .volume;
        assert!(
            (volume - want).abs() < 1e-9,
            "{op}: volume {volume}, want {want}"
        );
        let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
        assert!(verdict.is_ok(), "{op}: 3′ {:?}", verdict.err());
        assert_eq!(
            out.contacts.len(),
            records,
            "{op}: records {:?}",
            out.contacts
        );
    }
}

/// **One spike in the pit** ([`pit_holding_spikes`]). Its far corners
/// `0.5·(r₀ + r₂)` and `0.5·(r₁ + r₂)` rest on the cube's faces x = 1
/// and y = 1, inside the pit, on a part of each face the section
/// divides off: every op carries the rest on the piece that holds it,
/// so `cube ∖ y` keeps both (its two v-f records) and `cube ∩ y` drops
/// them with the piece. Red as the first row: the pit's dangling null
/// edge held the spike's cuts, and every op refused
/// `SharedVertexCrossings`. Red if the spike's strut hangs at the cube's
/// corner beside the pit's: the corner would be the In end of one and
/// the Out end of the other, an invariant.
#[test]
fn a_dangling_null_edge_holding_another_pairs_cut_builds_in_every_op() {
    let spike = [[1.0, 0.8, -0.1], [0.8, 1.0, -0.1], [1.0, 1.0, 0.2]];
    pit_holding_spikes(&[(spike, 0.5)], [0, 1, 1, 0, 2, 1], Tol::witness());
}

/// **Two spikes in the pit**, side by side ([`pit_holding_spikes`]), in
/// both orders of union: both struts hang at the pit's tip, the later
/// minted past the earlier when its lower germ is later. Every op
/// builds, and every result passes 3′.
#[test]
fn a_dangling_null_edge_holding_two_pairs_cuts_builds_in_every_op() {
    let one = (
        [[1.0, 0.78, -0.1], [1.0, 0.9, -0.1], [1.0, 0.86, 0.15]],
        0.4,
    );
    let two = (
        [[0.9, 1.0, -0.1], [0.78, 1.0, -0.1], [0.86, 1.0, 0.15]],
        0.4,
    );
    for spikes in [[one, two], [two, one]] {
        pit_holding_spikes(&spikes, [0, 3, 3, 0, 0, 3], Tol::witness());
    }
}

/// The two prisms of a lens at the origin: the corner prisms over
/// `(d1, d2, top)` and `(d2, d1, bottom)`, flush along their face
/// `(d1, d2)`, so the lens's cone holds the plane sector between `d1`
/// and `d2`.
fn lens_prisms(
    (d1, d2): ([f64; 3], [f64; 3]),
    (top, bottom): ([f64; 3], [f64; 3]),
) -> [[[f64; 3]; 3]; 2] {
    [[d1, d2, top], [d2, d1, bottom]]
}

/// **A strut whose segment holds another whole, sharing its ends'
/// directions.** `y` is the block `[-1, 1]³` less the lens `outer`
/// (scaled out past the block), with the lens `inner` (inside `outer`,
/// its rays along `outer`'s at the ends `inner` names) put back at its
/// scale: the two pieces touch along pinch lines in the cube's bottom
/// face. Against the unit cube's corner, both pieces' pairs cross that
/// face's corner twice, so both are dangling null edges: the block's an
/// Out run whose other way round is the whole orbit, the lens's an In
/// run inside it that hangs at its tip. `y` is built in both orders of
/// its union, and every op, in both operand orders, builds at the
/// volume clipping each prism to each box gives, and passes 3′ with
/// `y`'s records carried, but, where `overlapping`, those that refuse
/// ([`lens_against_the_cube`]).
fn lens_in_a_lens(
    outer: [[[f64; 3]; 3]; 2],
    inner: ([[[f64; 3]; 3]; 2], f64),
    overlapping: bool,
    tol: Tol,
) {
    let volumes = lens_volumes(outer, inner, None, None);
    for (order, y) in lens_ys(outer, inner, tol) {
        assert_eq!(
            keys_at(&y.body, (0.0, 0.0, 0.0)).len(),
            2,
            "y = {order}: each piece's corner at the origin"
        );
        lens_against_the_cube(&y, volumes, overlapping, order, tol);
    }
}

/// The outer lens's scale in [`lens_ys`].
const OUTER_SCALE: f64 = 3.0;

/// [`lens_in_a_lens`]'s `y` in both orders of its union.
fn lens_ys(
    outer: [[[f64; 3]; 3]; 2],
    (inner, scale): ([[[f64; 3]; 3]; 2], f64),
    tol: Tol,
) -> [(&'static str, BooleanBody<f64>); 2] {
    let k = lens(outer, OUTER_SCALE, tol);
    let block = finished(
        "block",
        brick::<f64>((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0), tol),
        tol,
    );
    let BooleanResult::Body(cut) = subtract(&block, &k.body, tol).expect("the lens is cut") else {
        panic!("the block ∖ lens came back empty");
    };
    let piece = lens(inner, scale, tol);
    let cut_first = BooleanInputs {
        records_a: carried_rows(&cut.contacts),
        ..BooleanInputs::default()
    };
    let piece_first = BooleanInputs {
        records_b: carried_rows(&cut.contacts),
        ..BooleanInputs::default()
    };
    [
        (
            "cut ∪ lens",
            union_in(&cut.body, &piece.body, &cut_first, tol),
        ),
        (
            "lens ∪ cut",
            union_in(&piece.body, &cut.body, &piece_first, tol),
        ),
    ]
    .map(|(order, got)| {
        let BooleanResult::Body(y) = got.unwrap_or_else(|e| panic!("{order} refused: {e:?}"))
        else {
            panic!("{order} came back empty");
        };
        (order, y)
    })
}

/// The lens over `rays` ([`lens_prisms`]) at `scale`.
fn lens(rays: [[[f64; 3]; 3]; 2], scale: f64, tol: Tol) -> BooleanBody<f64> {
    let [up, down] = rays.map(|r| corner_prism(r, scale, tol));
    let BooleanResult::Body(lens) = union(&up, &down, tol).expect("a lens") else {
        panic!("the lens came back empty");
    };
    lens
}

/// The half-spaces `n·p ≤ d` of the convex prism `faces`, each face's
/// plane turned away from the prism's mean vertex.
fn prism_planes(faces: &[Vec<[f64; 3]>]) -> Vec<([f64; 3], f64)> {
    let all = faces.concat();
    let c: [f64; 3] =
        core::array::from_fn(|k| all.iter().map(|p| p[k]).sum::<f64>() / all.len() as f64);
    faces
        .iter()
        .map(|f| {
            let n = cross(sub(f[1], f[0]), sub(f[2], f[0]));
            let n = if dot(n, c) > dot(n, f[0]) {
                n.map(|x| -x)
            } else {
                n
            };
            (n, dot(n, f[0]))
        })
        .collect()
}

/// A lens's two prisms, by rays and scale ([`lens_prisms`]).
type Lens = ([[[f64; 3]; 3]; 2], f64);

/// `[common, y]`: the volumes of `y ∩ cube` and of `y`
/// ([`lens_against_the_cube`]), for `y` the block less the lens
/// `outer` at [`OUTER_SCALE`], with `inner` put back less `notch`, and
/// `fill` put back in the notch. Each lens's two halves lie on the two
/// sides of the plane they share, so halves are clipped pairwise. The
/// block's part is clipped to the block; the lenses put back count
/// whole in `y`, as `inner` may reach past the block. `inner` and the
/// notch lie in `outer` within the block, and the fill in the notch,
/// which the generator checks.
fn lens_volumes(
    outer: [[[f64; 3]; 3]; 2],
    inner: Lens,
    notch: Option<Lens>,
    fill: Option<Lens>,
) -> [f64; 2] {
    let halves = |(rays, s): Lens| rays.map(|r| prism_faces(r, s));
    let (o, i) = (halves((outer, OUTER_SCALE)), halves(inner));
    let clip = |f: &[Vec<[f64; 3]>], within: &[&[Vec<[f64; 3]>]], box_: &[([f64; 3], f64)]| {
        let mut planes = box_.to_vec();
        for w in within {
            planes.extend(prism_planes(w));
        }
        clipped_volume(f.to_vec(), &planes)
    };
    let (cube, block) = (box_planes(0.0, 1.0), box_planes(-1.0, 1.0));
    let anywhere: Vec<([f64; 3], f64)> = Vec::new();
    [(1.0, &cube, &cube), (8.0, &block, &anywhere)].map(|(whole, box_, put)| {
        (0..2)
            .map(|h| {
                let (in_outer, i_block) = (clip(&i[h], &[&o[h]], &block), clip(&i[h], &[], &block));
                assert!(
                    (in_outer - i_block).abs() < 1e-12,
                    "the inner lens lies in the outer within the block: {in_outer} vs {i_block}"
                );
                let mut v = clip(&i[h], &[], put) - clip(&o[h], &[], box_);
                if let Some(n) = notch.map(halves) {
                    let in_outer = clip(&n[h], &[&o[h]], &block);
                    let n_block = clip(&n[h], &[], &block);
                    assert!(
                        (in_outer - n_block).abs() < 1e-12,
                        "the notch lies in the outer lens: {in_outer} vs {n_block}"
                    );
                    v -= clip(&n[h], &[&i[h]], put);
                    if let Some(f) = fill.map(halves) {
                        let (in_notch, whole) = (clip(&f[h], &[&n[h]], &[]), clip(&f[h], &[], &[]));
                        assert!(
                            (in_notch - whole).abs() < 1e-12,
                            "the fill lies in the notch: {in_notch} vs {whole}"
                        );
                        v += clip(&f[h], &[], put);
                    }
                }
                v
            })
            .sum::<f64>()
            + whole
    })
}

/// A lens `y`, built in the `order` named, against the cube's corner:
/// each op builds at its share of `[common, y]` ([`lens_volumes`]) and
/// passes 3′ with `y`'s records carried, but, where `overlapping`,
/// `cube ∖ y` and both unions: `y`'s coincident struts lie inside the
/// cube, and those results carry them as edges whose wedges overlap,
/// which every tier passed, so the result gate refuses
/// `BooleanError::LineWedges`.
fn lens_against_the_cube(
    y: &BooleanBody<f64>,
    [common, y_volume]: [f64; 2],
    overlapping: bool,
    order: &str,
    tol: Tol,
) {
    let got = mass_properties(&y.body, tol, topo::test_support::pinned_resolution(tol))
        .expect("mass")
        .met()
        .expect("the pinned resolution is met")
        .volume;
    assert!(
        (got - y_volume).abs() < 1e-9,
        "y = {order}: volume {got}, want {y_volume}"
    );
    for (op, got) in against_the_cube(&y.body, &carried_rows(&y.contacts), tol) {
        if overlapping && matches!(op, "cube ∖ y" | "y ∪ cube" | "cube ∪ y") {
            assert!(
                matches!(
                    got,
                    Err(BooleanError::LineWedges {
                        fault: topo::LineWedgeFault::Overlap { .. },
                        ..
                    })
                ),
                "{op} (y = {order}): {got:?}, not the overlap's refusal"
            );
            continue;
        }
        let volume = match op {
            "y ∪ cube" | "cube ∪ y" => y_volume + 1.0 - common,
            "y ∖ cube" => y_volume - common,
            "cube ∖ y" => 1.0 - common,
            _ => common,
        };
        let op = format!("{op} (y = {order})");
        let out = match got {
            Err(e) => panic!("{op} refused: {e:?}"),
            Ok(BooleanResult::Body(out)) => out,
            Ok(_) => panic!("{op} came back empty"),
        };
        assert_classes(
            &out,
            operands(&op, &[("y", &y.body), ("cube", &the_cube(tol))]),
            &op,
            tol,
        );
        let got = mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
            .expect("mass")
            .met()
            .expect("the pinned resolution is met")
            .volume;
        assert!(
            (got - volume).abs() < 1e-9,
            "{op}: volume {got}, want {volume}"
        );
        let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
        assert!(verdict.is_ok(), "{op}: 3′ refused {:?}", verdict.err());
    }
}

/// The rays the cube's bottom face holds the lenses' pinch lines along.
const D1: [f64; 3] = [1.0, 0.3, 0.0];
const D2: [f64; 3] = [0.3, 1.0, 0.0];

/// The outer lens of the one-arc witnesses: its pinch lines along both
/// [`D1`] and [`D2`].
fn one_arc_outer() -> [[[f64; 3]; 3]; 2] {
    lens_prisms((D1, D2), ([0.8, 0.8, 0.3], [0.8, 0.8, -0.3]))
}

/// The inner lens of the one-arc witnesses, along the outer's rays.
fn one_arc_inner() -> [[[f64; 3]; 3]; 2] {
    lens_prisms((D1, D2), ([0.8, 0.8, 0.15], [0.8, 0.8, -0.15]))
}

/// **Two dangling null edges whose runs are one arc, in every op**: the
/// inner lens's rays in the cube's face are the outer's ([`lens_ys`]),
/// so the two pieces touch along both, and their pairs' dangling null
/// edges have one segment. The inner lens at scale 2 reaches past the
/// cube, so the pinch lines leave the cube's face through its bottom
/// edges. The block's strut (Out on the segment) holds the lens's (In)
/// (`insert::holds_whole`), so the vertex between them keeps the
/// corner's region outside both pieces, whichever order `y` was built
/// in. Every op, in both operand orders and with `y` built both ways,
/// builds at the volume clipping each prism to each box gives, and
/// passes 3′ with `y`'s records carried. Red as the first row: each
/// strut held the other's cuts, and every op refused
/// `SharedVertexCrossings`. Red with the In strut holding, the vertex
/// between them inside both pieces: every op refuses `JoinDesync`. Red
/// if the strut minted first holds: `y = lens ∪ cut` refuses the same
/// way, and so does `cut ∪ lens` with its vertex pairs reversed
/// ([`every_tied_strut_witness_holds_with_its_vertex_pairs_reversed`]).
/// Red in both unions and `cube ∖ y` if the output stage does not merge
/// the sliver of no area the join leaves on each pinch line: two edges
/// of 330.8° and 195.6° there (164.4° and 29.2° in the difference),
/// overlapping, and `LineWedges`.
#[test]
fn two_dangling_null_edges_with_one_segment_build_in_every_op() {
    lens_in_a_lens(
        one_arc_outer(),
        (one_arc_inner(), 2.0),
        false,
        Tol::witness(),
    );
}

/// **The one arc with the pinch lines ending inside the cube's face**:
/// [`two_dangling_null_edges_with_one_segment_build_in_every_op`] with
/// the inner lens at scale 0.5, whose far corners (0.5, 0.15, 0) and
/// (0.15, 0.5, 0) lie inside the cube's bottom face; then with a thinner
/// lens over `([1, 0.6, 0], [0.6, 1, 0])` cut out of the inner lens
/// (whose pair crosses the face's corner four times, a third strut,
/// Out, nested in the lens's), with and without a lens in that notch.
/// The struts at the origin nest in every one, the Out run holding.
/// Every op of every one builds at its volume, `y` built both ways, and
/// passes 3′ with `y`'s records carried, but `cube ∖ y` and both unions,
/// which carry the struts as edges of 330.8° and 195.6° (164.4° and
/// 29.2° in the difference), overlapping, which every tier passed, and
/// refuse `LineWedges`. Red in `y ∖ cube` when a
/// carried row's ends do not reach their null-edge copies: the pinch
/// lines' far ends lie inside the cube's face, and the copies the
/// result keeps there are unrecorded.
#[test]
fn two_dangling_null_edges_with_one_segment_ending_in_the_cubes_face_build_in_every_op() {
    let tol = Tol::witness();
    let inner = (one_arc_inner(), 0.5);
    let notch = (
        lens_prisms(
            ([1.0, 0.6, 0.0], [0.6, 1.0, 0.0]),
            ([0.8, 0.8, 0.1], [0.8, 0.8, -0.1]),
        ),
        3.0,
    );
    let fill = (
        lens_prisms(
            ([1.0, 0.75, 0.0], [0.75, 1.0, 0.0]),
            ([0.8, 0.8, 0.05], [0.8, 0.8, -0.05]),
        ),
        0.3,
    );
    let [(_, cut_first), (_, lens_first)] = lens_ys(one_arc_outer(), inner, tol);
    let plain = lens_volumes(one_arc_outer(), inner, None, None);
    for (order, y) in [("cut ∪ lens", &cut_first), ("lens ∪ cut", &lens_first)] {
        lens_against_the_cube(y, plain, true, order, tol);
    }
    let notched_y = notched(&cut_first, notch, None, tol);
    lens_against_the_cube(
        &notched_y,
        lens_volumes(one_arc_outer(), inner, Some(notch), None),
        true,
        "cut ∪ notched lens",
        tol,
    );
    lens_against_the_cube(
        &notched(&cut_first, notch, Some(fill), tol),
        lens_volumes(one_arc_outer(), inner, Some(notch), Some(fill)),
        true,
        "cut ∪ notched lens ∪ lens in the notch",
        tol,
    );
}

/// `y` with the lens `notch` taken out, then the lens `fill` put back
/// if given.
fn notched(y: &BooleanBody<f64>, notch: Lens, fill: Option<Lens>, tol: Tol) -> BooleanBody<f64> {
    let notch = lens(notch.0, notch.1, tol);
    let decls = BooleanInputs {
        records_a: carried_rows(&y.contacts),
        ..BooleanInputs::default()
    };
    let BooleanResult::Body(mut y) =
        subtract_in(&y.body, &notch.body, &decls, tol).expect("the notch is cut")
    else {
        panic!("the notched y came back empty");
    };
    if let Some((rays, scale)) = fill {
        let fill = lens(rays, scale, tol);
        let decls = BooleanInputs {
            records_a: carried_rows(&y.contacts),
            ..BooleanInputs::default()
        };
        let BooleanResult::Body(next) =
            union_in(&y.body, &fill.body, &decls, tol).expect("the fill touches the notch's apex")
        else {
            panic!("the filled y came back empty");
        };
        y = next;
    }
    y
}

/// **A strut inside another's segment, sharing one end's direction**
/// ([`lens_in_a_lens`]): the inner lens's rays in the cube's face are
/// the outer's `D1` and a ray strictly between, so the pieces touch
/// along one pinch line. Red as the first row: the cut strictly
/// inside the Out run's segment held it, and every op refused
/// `SharedVertexCrossings`. Red in `y ∖ cube` when a carried row's ends
/// do not reach their null-edge copies: the pinch line's far end lies
/// inside the cube's face, and the copies the result keeps there are
/// unrecorded. Red in `y ∪ cube` (3′ `VertexOnEdge`, `EdgeEdgeOverlap`)
/// when the join's chord along `y`'s pinch-line edge writes no
/// substitution row: the op drops the edge and keeps the chord, and a
/// carried row on the edge has nowhere to land.
/// Both unions and `cube ∖ y` refuse `LineWedges` instead: two edges of
/// 330.8° and 195.6° (29.2° and 164.4° in the difference),
/// overlapping, which every tier passed.
#[test]
fn a_dangling_null_edge_inside_another_along_one_end_builds_in_every_op() {
    lens_in_a_lens(
        one_arc_outer(),
        (
            lens_prisms((D1, [0.6, 1.0, 0.0]), ([0.8, 0.8, 0.15], [0.8, 0.8, -0.15])),
            0.5,
        ),
        true,
        Tol::witness(),
    );
}

/// **The tied-strut witnesses do not read their vertex pairs' order.**
/// Each runs again with every reduction taking its vertex pairs, and
/// each pair's crossing records, in reverse
/// (`test_support::with_vertex_pairs_reversed`). That turns round the
/// plans' order, and with it the pairs' and the mints' and the plan
/// index that breaks ties among equal keys, and a two-crossing pair's
/// `from` and `to`. Each holds its own assertions unchanged: the same
/// builds at the same volumes and 3′ verdicts, the same refusals. Each
/// must have reversed, among the ops it tests
/// (`test_support::ops_under_test`), a reduction with two crossing
/// pairs at one vertex. The booleans that build the operands run
/// reversed too, and are not counted. Red if two struts tied at both
/// ends nest by mint order: the one-arc witness's `cut ∪ lens` then
/// refuses `JoinDesync`.
#[test]
fn every_tied_strut_witness_holds_with_its_vertex_pairs_reversed() {
    let witnesses: [(&str, fn()); 10] = [
        (
            "a pinch line flat in the shared corner's face",
            a_pinch_line_flat_in_the_shared_corners_face_builds_in_every_op,
        ),
        (
            "a pinch line in a face flush with both bricks",
            a_pinch_line_in_a_face_flush_with_both_bricks_builds_in_every_op,
        ),
        (
            "two dangling null edges meeting beside the pinch line",
            two_dangling_null_edges_meeting_beside_the_pinch_line_build_in_every_op,
        ),
        (
            "three pieces, two of whose cuts tie",
            three_pieces_two_of_whose_cuts_tie_build_in_every_op,
        ),
        (
            "two dangling null edges meeting on the pinch line",
            two_dangling_null_edges_meeting_on_the_pinch_line_build_in_every_op,
        ),
        (
            "a strut holding a spike",
            a_dangling_null_edge_holding_another_pairs_cut_builds_in_every_op,
        ),
        (
            "a strut holding two spikes",
            a_dangling_null_edge_holding_two_pairs_cuts_builds_in_every_op,
        ),
        (
            "a strut inside another along one end",
            a_dangling_null_edge_inside_another_along_one_end_builds_in_every_op,
        ),
        (
            "two struts with one segment",
            two_dangling_null_edges_with_one_segment_build_in_every_op,
        ),
        (
            "two struts with one segment ending in the cube's face",
            two_dangling_null_edges_with_one_segment_ending_in_the_cubes_face_build_in_every_op,
        ),
    ];
    for (name, witness) in witnesses {
        let ((), reversed) = topo::test_support::with_vertex_pairs_reversed(witness);
        assert!(
            reversed > 0,
            "{name}: no reduction held two crossing pairs at one vertex"
        );
    }
}

/// **A corner crossing the cube's corner four times builds every op**
/// (`work/cleave/a-corner-crossing-another-four-times-refuses-pairing-mismatch.md`):
/// a thin trihedral corner at the origin whose cone holds the cube's
/// z-edge and crosses its bottom face, so the two corners' boundaries
/// cross four times, in two section-polygon edges. Each solid orders
/// the germs round its own corner, two in one sector by angle, so the
/// pairing reads adjacent in both; every op, in both operand orders,
/// builds at the volume clipping the corner to the cube gives
/// ([`clipped_volume`]) and passes tier 2, 3′ and the certificate.
/// Red as `PairingMismatch` when two germs in one sector are ordered by
/// the other solid's sector, and as `JoinDesync` when the pairing starts
/// at A's first germ whatever the op keeps of A.
#[test]
fn a_corner_crossing_the_cubes_four_times_builds_every_op() {
    let tol = Tol::witness();
    let rays = [
        [0.5948, 0.757, -0.2704],
        [-0.5774, -0.5774, 0.5774],
        [0.757, 0.5948, -0.2704],
    ];
    let band = corner_prism(rays, 0.4, tol);
    let y_volume = clipped_volume(prism_faces(rays, 0.4), &[]);
    let common = clipped_volume(prism_faces(rays, 0.4), &box_planes(0.0, 1.0));
    assert!(
        common > 0.0 && common < y_volume,
        "the corner crosses the cube"
    );
    for (op, got) in against_the_cube(&band, &topo::OperandRecords::default(), tol) {
        let want = match op {
            "y ∪ cube" | "cube ∪ y" => y_volume + 1.0 - common,
            "y ∖ cube" => y_volume - common,
            "cube ∖ y" => 1.0 - common,
            _ => common,
        };
        let BooleanResult::Body(out) = got.unwrap_or_else(|e| panic!("{op} refused: {e:?}")) else {
            panic!("{op} came back empty");
        };
        assert_classes(
            &out,
            operands(op, &[("y", &band), ("cube", &the_cube(tol))]),
            op,
            tol,
        );
        let volume = mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
            .expect("mass")
            .met()
            .expect("the pinned resolution is met")
            .volume;
        assert!(
            (volume - want).abs() < 1e-9,
            "{op}: volume {volume}, want {want}"
        );
        assert_eq!(topo::validate_closed(&out.body), Ok(()), "{op}: tier 2");
        assert_eq!(
            validate_pseudomanifold(&out.body, &out.contacts, tol),
            Ok(()),
            "{op}: 3′"
        );
        assert!(
            topo::validate_geometric_certificate(&out.body, tol).is_ok(),
            "{op}: the certificate"
        );
    }
}

/// **Three corners alternating round the cube's corner refuse three
/// ops** (`work/cleave/three-corners-alternating-round-a-corner-refuse-at-the-join.md`):
/// three trihedral corners touching only at the origin, one a band
/// that meets the cube's corner's boundary in two dangling segments
/// and the other two in the gaps between, so the pieces' cuts
/// alternate round the corner. The shared corner's runs reconcile, and
/// ∪ and ∖ with `y` first and `cube ∩ y` build at volumes that agree
/// with each other. In `y ∩ cube`, `cube ∪ y` and `cube ∖ y` the band's
/// pair, which crosses the cube's corner four times, has no run in the
/// cube's corner clear of the other pieces' cuts, both ways round, and
/// refuses `SharedVertexCrossings`
/// (`work/gauge/shared-vertex-crossings-that-tie-or-interleave-are-unprobed.md`):
/// pinned as they stand.
#[test]
fn three_corners_alternating_round_the_cube_refuse_three_ops() {
    let tol = Tol::witness();
    let rays = [
        [
            [0.5325, -0.0249, 0.8461],
            [-0.0701, 0.6232, -0.7789],
            [0.2838, 0.2685, 0.9205],
        ],
        [
            [0.6008, -0.1593, 0.7834],
            [0.9615, -0.182, -0.2061],
            [0.578, 0.8019, -0.1512],
        ],
        [
            [-0.9791, 0.1382, 0.1493],
            [0.1722, 0.2407, 0.9552],
            [0.1469, 0.9677, 0.2049],
        ],
    ];
    let [i, j, k] = rays.map(|r| corner_prism(r, 0.4, tol));
    let BooleanResult::Body(two) = union(&i, &j, tol).expect("i ∪ j") else {
        panic!("i ∪ j came back empty");
    };
    let decls = BooleanInputs {
        records_a: carried_rows(&two.contacts),
        ..BooleanInputs::default()
    };
    let BooleanResult::Body(y) = union_in(&two.body, &k, &decls, tol).expect("∪ k") else {
        panic!("∪ k came back empty");
    };
    let y_volume = mass_properties(&y.body, tol, topo::test_support::pinned_resolution(tol))
        .expect("mass")
        .met()
        .expect("the pinned resolution is met")
        .volume;
    let mut volumes = std::collections::BTreeMap::new();
    for (op, got) in against_the_cube(&y.body, &carried_rows(&y.contacts), tol) {
        match (op, got) {
            (
                "y ∩ cube" | "cube ∪ y" | "cube ∖ y",
                Err(BooleanError::SharedVertexCrossings { .. }),
            ) => {}
            ("y ∪ cube" | "y ∖ cube" | "cube ∩ y", Ok(BooleanResult::Body(out))) => {
                let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
                assert!(verdict.is_ok(), "{op}: 3′ refused {:?}", verdict.err());
                volumes.insert(
                    op,
                    mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
                        .expect("mass")
                        .met()
                        .expect("the pinned resolution is met")
                        .volume,
                );
            }
            (op, got) => panic!("{op}: the pin moved: {:?}", got.map(|_| ())),
        }
    }
    let (union, less, common) = (
        volumes["y ∪ cube"],
        volumes["y ∖ cube"],
        volumes["cube ∩ y"],
    );
    for (what, got, want) in [
        ("y ∖ cube + ∩", less + common, y_volume),
        ("∪", union, 1.0 + less),
    ] {
        assert!((got - want).abs() < 1e-9, "{what}: {got}, want {want}");
    }
}

/// **A face holding the pinch line ties its pierces' germs** (PR 4728's
/// review r1): a cutter whose near face holds the whole pinch line, so at
/// each end of the line the two bricks' pierces of that face share a
/// germ along the line, and the ring orders the two by their chords
/// (`vtxfac::tie_breaks_after`). With the face's normal at slopes 0.5
/// (either side) and 2 the join cannot wind a run round the corner of no
/// angle the tie leaves, and every op refuses typed `PinchRaysInAFace`,
/// not as the kernel's defect. At slope −0.5 every op builds, passing 3′:
/// the unions 33, `pinch ∖` and both `∩`s 1, `cutter ∖` 31.
#[test]
fn a_face_holding_the_pinch_line_ties_the_pierces_germs() {
    let tol = Tol::witness();
    let (pinch, carried) = pinch(tol);
    for (slope, side) in [(0.5, -1.0), (0.5, 1.0), (2.0, -1.0), (-0.5, -1.0)] {
        // The near face is the plane through the z-axis with normal
        // n ∝ (1, −slope, 0); the cutter lies on `side` of it.
        let k = (1.0f64 + slope * slope).sqrt();
        let n = [side / k, -side * slope / k, 0.0];
        let w = [slope / k, 1.0 / k, 0.0];
        let u = [0.0, 0.0, 1.0];
        // (w, u, n) right-handed when side = −1? Pick the handed order.
        let wxu = [
            w[1] * u[2] - w[2] * u[1],
            w[2] * u[0] - w[0] * u[2],
            w[0] * u[1] - w[1] * u[0],
        ];
        let rh = wxu[0] * n[0] + wxu[1] * n[1] + wxu[2] * n[2] > 0.0;
        let (e0, e1) = if rh { (u, w) } else { (w, u) };
        let (r0, r1) = if rh {
            ((-1.0, 3.0), (-2.0, 2.0))
        } else {
            ((-2.0, 2.0), (-1.0, 3.0))
        };
        let cutter = topo::test_support::mapped_cube::<f64>(
            move |x, y, z| {
                let (a, b, c) = (r0.0 + (r0.1 - r0.0) * x, r1.0 + (r1.1 - r1.0) * y, -2.0 * z);
                geom_core::Point3::new(
                    a * e0[0] + b * e1[0] + c * n[0],
                    a * e0[1] + b * e1[1] + c * n[1],
                    a * e0[2] + b * e1[2] + c * n[2],
                )
            },
            tol,
        );
        let cutter = finished("the cutter", cutter, tol);
        let ab = BooleanInputs {
            records_a: carried.clone(),
            ..BooleanInputs::default()
        };
        let ba = BooleanInputs {
            records_b: carried.clone(),
            ..BooleanInputs::default()
        };
        for (what, r) in [
            ("pinch ∪ cutter", union_in(&pinch.body, &cutter, &ab, tol)),
            (
                "pinch ∖ cutter",
                subtract_in(&pinch.body, &cutter, &ab, tol),
            ),
            (
                "pinch ∩ cutter",
                intersect_in(&pinch.body, &cutter, &ab, tol),
            ),
            ("cutter ∪ pinch", union_in(&cutter, &pinch.body, &ba, tol)),
            (
                "cutter ∖ pinch",
                subtract_in(&cutter, &pinch.body, &ba, tol),
            ),
            (
                "cutter ∩ pinch",
                intersect_in(&cutter, &pinch.body, &ba, tol),
            ),
        ] {
            let pose = format!("slope={slope} side={side} {what}");
            if slope < 0.0 {
                let Ok(BooleanResult::Body(out)) = r else {
                    panic!("{pose}: {r:?}")
                };
                let v = mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
                    .expect("mass")
                    .met()
                    .expect("the pinned resolution is met")
                    .volume;
                let want = match what {
                    "pinch ∪ cutter" | "cutter ∪ pinch" => 33.0,
                    "cutter ∖ pinch" => 31.0,
                    _ => 1.0,
                };
                assert!((v - want).abs() < 1e-9, "{pose}: volume {v}, want {want}");
                let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
                assert!(verdict.is_ok(), "{pose}: 3′ refused {:?}", verdict.err());
            } else {
                assert!(
                    matches!(r, Err(BooleanError::PinchRaysInAFace { .. })),
                    "{pose}: {r:?}"
                );
            }
        }
    }
}

/// **A pinch line's end joins where the next union straightens it, its
/// records carried.** `p` and `q` touch along the z-axis over
/// `z ∈ (0, 1)`; `r` stacks on `p`, so `p`'s vertex at `(0, 0, 1)`
/// lies inside the stacked block's vertical edge. Carrying `p ∪ q`'s
/// records, the union with `r` joins that vertex away (its key shared
/// only with `q`'s corner, another lump's: `edge_join::free_vertices`),
/// leaves no joinable vertex, passes 3′ on its records, and builds the
/// body `r` first builds. Red where the join keeps every vertex on a
/// shared key (a vertex more than the other order, joinable at rest).
#[test]
fn a_pinch_lines_end_joins_where_the_next_union_straightens_it() {
    let tol = Tol::witness();
    let p = finished("p", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol), tol);
    let q = finished("q", brick((-1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), tol), tol);
    let r = finished("r", brick((0.0, 1.0), (0.0, 1.0), (1.0, 2.0), tol), tol);
    let step = |a: &AtRestBody<f64>, b: &AtRestBody<f64>, prev: Option<&BooleanBody<f64>>| {
        let mut decls = BooleanInputs::default();
        if let Some(prev) = prev {
            decls.records_a = carried_rows(&prev.contacts);
        }
        match union_in(a, b, &decls, tol) {
            Ok(BooleanResult::Body(out)) => out,
            other => panic!("{other:?}"),
        }
    };
    let pq = step(&p, &q, None);
    let fold = step(&pq.body, &r, Some(&pq));
    let pr = step(&p, &r, None);
    let other = step(&pr.body, &q, Some(&pr));
    let counts = |b: &Body<f64>| (b.vertices().count(), b.edges().count(), b.faces().count());
    let band = geom_core::Band::linear(tol).unwrap();
    assert_eq!(
        topo::joinable_vertices(&fold.body, band).unwrap(),
        vec![],
        "no joinable vertex is left"
    );
    assert_eq!(
        validate_pseudomanifold(&fold.body, &fold.contacts, tol),
        Ok(()),
        "3′ on its records"
    );
    assert_eq!(
        counts(&fold.body),
        counts(&other.body),
        "the two orders' bodies"
    );
}
