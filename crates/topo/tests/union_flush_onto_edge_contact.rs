//! **A flush partner folded onto an edge contact.** `a` and `b` are
//! flush along x; `c` touches them along one edge only, so a union
//! holding `c` and one of the two holds two vertices at each end of
//! the touch. The partner that folds in next has a vertex there,
//! which pairs with both, and every order of the three builds.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished, flush_declarations};
use geom_core::Tol;
use topo::{
    AtRestBody, Body, BooleanBody, BooleanError, BooleanResult, CarriedVv, ContactClass,
    intersect_with, mass_properties, subtract_with, union_with, validate_pseudomanifold,
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

fn members(c_span: (f64, f64), tol: Tol) -> [AtRestBody<f64>; 3] {
    [
        finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol), tol),
        finished("b", brick((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol), tol),
        finished("c", brick(c_span, (-1.0, 0.0), (-1.0, 0.0), tol), tol),
    ]
}

/// Folds `order`, each step declaring its flush pairs and, when
/// `carry`, carrying the accumulator's own v-v records in as `Rest`.
/// Every step's result, or the refusal and the step it came at.
fn fold(
    bodies: &[AtRestBody<f64>; 3],
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
    quarter_pinch([(0.0, 0.0), (-1.0, -1.0)], (0.5, 1.5), tol)
}

/// Two unit bricks over `z` whose low corners in x and y are `lows`,
/// touching along the z-axis, and their record carried as `Rest` rows.
fn quarter_pinch(
    lows: [(f64, f64); 2],
    z: (f64, f64),
    tol: Tol,
) -> (BooleanBody<f64>, Vec<CarriedVv>) {
    let [p, q] = lows.map(|(x, y)| {
        finished(
            "a quarter brick",
            brick((x, x + 1.0), (y, y + 1.0), z, tol),
            tol,
        )
    });
    let BooleanResult::Body(pinch) =
        union_with(&p, &q, &flush_declarations(&p, &q, tol), tol).expect("an edge contact builds")
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
        .vertices()
        .filter(|(_, v)| {
            body.get_point(v.point)
                .is_some_and(|q| (q.x, q.y, q.z) == p)
        })
        .map(|(k, _)| k)
        .collect();
    keys.sort();
    keys
}

/// Each op of `pinch` and `cutter`, in both operand orders and with
/// `carried` as the pinch's own records, builds at its volume and
/// passes 3′: `pinch ∪ cutter`, `pinch ∖ cutter`, `pinch ∩ cutter` and
/// `cutter ∖ pinch`.
fn every_op_builds(
    pinch: &AtRestBody<f64>,
    carried: &[CarriedVv],
    cutter: &AtRestBody<f64>,
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
/// triangles clipped to the cutter's give. A nonzero `shear` leans
/// the cutter by that much per unit of height ([`sheared`]).
fn crossings_at_one_corner(spans: &[(f64, f64)], cutter: (f64, f64), shear: (f64, f64), tol: Tol) {
    let fold = |acc: &BooleanBody<f64>, next: &AtRestBody<f64>| {
        let mut decls = flush_declarations(&acc.body, next, tol);
        decls.carried_a.vv = rest_rows(&acc.contacts);
        match union_with(&acc.body, next, &decls, tol).expect("a touching wedge folds in") {
            BooleanResult::Body(out) => out,
            BooleanResult::Empty => panic!("a union of wedges came back empty"),
        }
    };
    let wedges: Vec<AtRestBody<f64>> = spans
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
        &rest_rows(&pinch.contacts),
        &cutter_body,
        [
            pinch_volume + cutter_volume - common,
            pinch_volume - common,
            common,
            cutter_volume - common,
        ],
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
/// refuses `CorruptOperand`), or if a dangling null edge refuses
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
        let mut decls = flush_declarations(&a.body, &b.body, tol);
        decls.carried_a.vv.clone_from(ca);
        decls.carried_b.vv.clone_from(cb);
        // At (0,0,1) the lower pinch's two vertices are its split edges'
        // and the upper pinch's are its corners; every pair crosses.
        // The refusal names A's vertex there and the two B vertices it
        // crosses into.
        let (a_at, b_at) = (
            keys_at(&a.body, (0.0, 0.0, 1.0)),
            keys_at(&b.body, (0.0, 0.0, 1.0)),
        );
        match union_with(&a.body, &b.body, &decls, tol) {
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
        let less = subtract_with(&a.body, &b.body, &decls, tol);
        let BooleanResult::Body(less) = less.unwrap_or_else(|e| panic!("{name} ∖ refused: {e:?}"))
        else {
            panic!("{name} ∖ came back empty");
        };
        let volume = mass_properties(&less.body, tol)
            .expect("the difference has mass")
            .volume;
        assert!(
            (volume - minuend).abs() < 1e-9,
            "{name} ∖: volume {volume}, want {minuend}"
        );
        let verdict = validate_pseudomanifold(&less.body, &less.contacts, tol);
        assert!(verdict.is_ok(), "{name} ∖: 3′ refused {:?}", verdict.err());
        let common = intersect_with(&a.body, &b.body, &decls, tol);
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
        let mut decls = flush_declarations(&a.body, &b.body, tol);
        decls.carried_a.vv.clone_from(ca);
        decls.carried_b.vv.clone_from(cb);
        let built = |op: &str, got: Result<BooleanResult<f64>, BooleanError>, want: f64| {
            let BooleanResult::Body(out) =
                got.unwrap_or_else(|e| panic!("{name} {op} refused: {e:?}"))
            else {
                panic!("{name} {op} came back empty");
            };
            let volume = mass_properties(&out.body, tol)
                .expect("the result has mass")
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
            union_with(&a.body, &b.body, &decls, tol),
            low_volume + high_volume,
        );
        assert_eq!(
            keys_at(&union.body, (0.0, 0.0, 1.0)).len(),
            4,
            "{name} ∪: each brick's corner at (0,0,1) is its own vertex"
        );
        built("∖", subtract_with(&a.body, &b.body, &decls, tol), minuend);
        let common = intersect_with(&a.body, &b.body, &decls, tol);
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
/// all six refuse `SharedVertexCrossings`.
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
    let cutter_volume = area(&tri) * 0.5;
    every_op_builds(
        &pinch.body,
        &carried,
        &sheared_wedge(shear, tol),
        [
            2.0 + cutter_volume - common,
            2.0 - common,
            common,
            cutter_volume - common,
        ],
        tol,
    );
}

/// **Three pieces, two of whose cuts tie.** Wedges 0°–60°, 90°–120°
/// and 150°–240° about the z-axis, cut by the wedge from 50° to 160°
/// sheared backwards along its 50° face, so the pinch line runs flat
/// in that face: the outer two pieces' cuts lie along it, on its two
/// sides, and the middle piece hangs a dangling null edge in the
/// prism's bottom corner. Each op, in both operand orders, builds at
/// the volume a polygon clip of the sheared triangle against each
/// wedge's gives, and passes 3′ with the pinch's records carried. Red
/// as the two-piece tie.
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

/// **A pinch line crossing a face's interior drops the pinch's records
/// at the new pinch end** (`work/fuse/a-pinch-line-crossing-a-face-interior-drops-the-pinchs-records.md`):
/// pinned as it stands. The 80°–190° wedge sheared off the axis, so the
/// pinch line runs inside its corner and leaves through its top face,
/// and a brick holding the axis, which its top face crosses. Each op
/// builds, the sheared one's volumes agreeing with each other; the
/// results that keep the pinch's two pieces at the top face's crossing
/// fail 3′ there with an undeclared `VertexVertex` at (0,0,1), and the
/// rest pass. Red when that end gets its record.
#[test]
fn a_pinch_line_through_a_face_drops_its_records_at_the_new_end() {
    let tol = Tol::witness();
    let (pinch, carried) = pinch(tol);
    let cutter = sheared_wedge((0.3, -0.3), tol);
    let block = finished(
        "block",
        brick::<f64>((-0.5, 0.5), (-0.5, 0.5), (0.0, 1.0), tol),
        tol,
    );
    let mut ab = flush_declarations(&pinch.body, &cutter, tol);
    ab.carried_a.vv.clone_from(&carried);
    let mut ba = flush_declarations(&cutter, &pinch.body, tol);
    ba.carried_b.vv.clone_from(&carried);
    let mut block_first = flush_declarations(&block, &pinch.body, tol);
    block_first.carried_b.vv = carried;
    let volume_of = |op: &str, got: Result<BooleanResult<f64>, BooleanError>, drops: bool| {
        let BooleanResult::Body(out) = got.unwrap_or_else(|e| panic!("{op} refused: {e:?}")) else {
            panic!("{op} came back empty");
        };
        match validate_pseudomanifold(&out.body, &out.contacts, tol) {
            Ok(_) => assert!(!drops, "{op}: passes 3′; the row's pin flips"),
            Err(errors) => {
                assert!(drops, "{op}: 3′ refused {errors:?}");
                assert!(
                    format!("{errors:?}").contains("VertexVertex")
                        && format!("{errors:?}").contains("(0.0, 0.0, 1.0)"),
                    "{op}: the undeclared pair at the pinch's new end: {errors:?}"
                );
            }
        }
        mass_properties(&out.body, tol).expect("mass").volume
    };
    let union = volume_of(
        "pinch ∪ cutter",
        union_with(&pinch.body, &cutter, &ab, tol),
        false,
    );
    let pinch_less = volume_of(
        "pinch ∖ cutter",
        subtract_with(&pinch.body, &cutter, &ab, tol),
        true,
    );
    let common = volume_of(
        "pinch ∩ cutter",
        intersect_with(&pinch.body, &cutter, &ab, tol),
        true,
    );
    let swapped = volume_of(
        "cutter ∪ pinch",
        union_with(&cutter, &pinch.body, &ba, tol),
        false,
    );
    let cutter_less = volume_of(
        "cutter ∖ pinch",
        subtract_with(&cutter, &pinch.body, &ba, tol),
        false,
    );
    let common_swapped = volume_of(
        "cutter ∩ pinch",
        intersect_with(&cutter, &pinch.body, &ba, tol),
        true,
    );
    // Shearing keeps the cutter's volume: its triangle × the height.
    let cutter_volume = area(&triangle(80.0, 190.0)) * 0.5;
    for (what, got, want) in [
        ("pinch ∖ cutter + ∩", pinch_less + common, 2.0),
        ("cutter ∖ pinch + ∩", cutter_less + common, cutter_volume),
        ("∪", union, 2.0 + cutter_volume - common),
        ("∪ swapped", swapped, union),
        ("∩ swapped", common_swapped, common),
    ] {
        assert!((got - want).abs() < 1e-9, "{what}: {got}, want {want}");
    }
    let block_common = volume_of(
        "block ∩ pinch",
        intersect_with(&block, &pinch.body, &block_first, tol),
        true,
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
/// result passes 3′. Red as the first row: `CorruptOperand` at the
/// prism's vertex on the axis, in all four. The subtract of the wall
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
    let mut ab = flush_declarations(&pinch.body, &wall, tol);
    ab.carried_a.vv.clone_from(&carried);
    let mut ba = flush_declarations(&wall, &pinch.body, tol);
    ba.carried_b.vv.clone_from(&carried);
    // The same wall over z ∈ (0.75, 1.25) cuts the brick on its side
    // in two and leaves the other whole: the pinch survives below 0.75
    // with a new pair of vertices at its cut.
    let upper = finished(
        "upper",
        common::prism_z(&[(-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)], 0.75, 1.25, tol).body,
        tol,
    );
    let mut cut = flush_declarations(&pinch.body, &upper, tol);
    cut.carried_a.vv = carried;
    // The prism holds the half x + y > 0 of the square, so it meets
    // the pinch in the brick on that side, z ∈ (0.5, 1): 0.5, and the
    // upper one over z ∈ (0.75, 1.25), 0.5 again.
    for (op, got, want) in [
        (
            "pinch ∖ upper wall",
            subtract_with(&pinch.body, &upper, &cut, tol),
            1.5,
        ),
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

/// The triangular prism whose corner at the origin spans `rays`
/// (a right-handed triple), scaled by `scale`: the linear image of the
/// unit right prism, so its faces stay planar and its corner cone is
/// the rays' own.
fn corner_prism(rays: [[f64; 3]; 3], scale: f64, tol: Tol) -> AtRestBody<f64> {
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
    carried: &[CarriedVv],
    tol: Tol,
) -> [(&'static str, BooleanOutcome); 6] {
    let cube = finished(
        "cube",
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    );
    let mut ab = flush_declarations(y, &cube, tol);
    ab.carried_a.vv = carried.to_vec();
    let mut ba = flush_declarations(&cube, y, tol);
    ba.carried_b.vv = carried.to_vec();
    [
        ("y ∪ cube", union_with(y, &cube, &ab, tol)),
        ("y ∖ cube", subtract_with(y, &cube, &ab, tol)),
        ("y ∩ cube", intersect_with(y, &cube, &ab, tol)),
        ("cube ∪ y", union_with(&cube, y, &ba, tol)),
        ("cube ∖ y", subtract_with(&cube, y, &ba, tol)),
        ("cube ∩ y", intersect_with(&cube, y, &ba, tol)),
    ]
}

type BooleanOutcome = Result<BooleanResult<f64>, BooleanError>;

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
/// with the pinch's records carried, except `pinch ∖ prism`: the
/// pinch's top end lies inside the prism's face, and that result keeps
/// both pieces there with no v-v row
/// (`work/fuse/a-carried-row-whose-ends-split-into-null-edge-copies-is-dropped.md`),
/// pinned as it stands. Red as the first row: each dangling null edge
/// read the other's cut along the line as held, and all six refused
/// `SharedVertexCrossings`. Red with the notch from −20° to 220° if a
/// dangling null edge splices by the germ its run leaves from rather
/// than its lower germ: both leave from the line, and all six refuse
/// the anchor's invariant.
#[test]
fn two_dangling_null_edges_meeting_on_the_pinch_line_build_in_every_op() {
    for notch in [(-40.0, 200.0), (-20.0, 220.0)] {
        dangling_null_edges_meeting_on_the_pinch_line(notch, Tol::witness());
    }
}

/// The pinch against the diagonal prism whose face's notch runs from
/// `notch.0` to `notch.1` degrees (doc above).
fn dangling_null_edges_meeting_on_the_pinch_line(notch: (f64, f64), tol: Tol) {
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
    // Profile (w, z) in the plane x = y, extruded along (1, −1, 0)/√2.
    let r = std::f64::consts::FRAC_1_SQRT_2;
    let mut prism = Body::<f64>::new();
    common::prism_ops(
        &mut prism,
        &profile,
        (0.0, 1.0),
        |w, z, u| geom_core::Point3::new((w + u) * r, (w - u) * r, z),
        common::FaceGeometry::Certified,
        tol,
    );
    common::describe_as_intersections(&mut prism, tol);
    let prism = finished("the diagonal prism", prism, tol);
    let rect: Vec<(f64, f64)> = [(-0.8, 1.0), (0.8, 1.0), (0.8, 0.0), (-0.8, 0.0)]
        .iter()
        .map(|&(w, u)| ((w + u) * r, (w - u) * r))
        .collect();
    let squares = [
        [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        [(-1.0, -1.0), (0.0, -1.0), (0.0, 0.0), (-1.0, 0.0)],
    ];
    let common: f64 = squares.iter().map(|q| area(&clip(&rect, q))).sum();
    let prism_volume = area(&profile);
    let mut ab = flush_declarations(&pinch.body, &prism, tol);
    ab.carried_a.vv.clone_from(&carried);
    let mut ba = flush_declarations(&prism, &pinch.body, tol);
    ba.carried_b.vv = carried;
    for (op, got, want, drops) in [
        (
            "pinch ∪ prism",
            union_with(&pinch.body, &prism, &ab, tol),
            2.0 + prism_volume - common,
            false,
        ),
        (
            "pinch ∖ prism",
            subtract_with(&pinch.body, &prism, &ab, tol),
            2.0 - common,
            true,
        ),
        (
            "pinch ∩ prism",
            intersect_with(&pinch.body, &prism, &ab, tol),
            common,
            false,
        ),
        (
            "prism ∪ pinch",
            union_with(&prism, &pinch.body, &ba, tol),
            2.0 + prism_volume - common,
            false,
        ),
        (
            "prism ∖ pinch",
            subtract_with(&prism, &pinch.body, &ba, tol),
            prism_volume - common,
            false,
        ),
        (
            "prism ∩ pinch",
            intersect_with(&prism, &pinch.body, &ba, tol),
            common,
            false,
        ),
    ] {
        let op = format!("{op} (notch {notch:?})");
        let BooleanResult::Body(out) = got.unwrap_or_else(|e| panic!("{op} refused: {e:?}")) else {
            panic!("{op} came back empty");
        };
        let volume = mass_properties(&out.body, tol).expect("mass").volume;
        assert!(
            (volume - want).abs() < 1e-9,
            "{op}: volume {volume}, want {want}"
        );
        match validate_pseudomanifold(&out.body, &out.contacts, tol) {
            Ok(_) => assert!(!drops, "{op}: passes 3′; the row's pin flips"),
            Err(errors) => {
                assert!(drops, "{op}: 3′ refused {errors:?}");
                let text = format!("{errors:?}");
                assert!(
                    text.contains("VertexVertex") && text.contains("(0.0, 0.0, 1.5)"),
                    "{op}: the undeclared pair at the pinch's top end: {errors:?}"
                );
            }
        }
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
fn box_planes(lo: f64, hi: f64) -> Vec<([f64; 3], f64)> {
    (0..3)
        .flat_map(|k| {
            let e: [f64; 3] = core::array::from_fn(|j| if j == k { 1.0 } else { 0.0 });
            [(e, hi), (e.map(|x| -x), -lo)]
        })
        .collect()
}

/// The volume of the convex polyhedron `faces` cut to `planes`, outside
/// the kernel: each face loop clipped to each half-space in turn
/// (Sutherland–Hodgman), the points on the plane closing the cut's cap
/// in angular order unless a face lies on it, then the pyramids from
/// the mean vertex summed. "On the plane" is exact equality, so a face
/// counts as lying on a plane only where its coordinates meet it
/// exactly, as the boxes' and the lenses' zero coordinates do.
fn clipped_volume(mut faces: Vec<Vec<[f64; 3]>>, planes: &[([f64; 3], f64)]) -> f64 {
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let sub = |a: [f64; 3], b: [f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let cross = |a: [f64; 3], b: [f64; 3]| {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
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
/// passes 3′ with `y`'s records carried except in the ops `drops` names
/// ([`a_dangling_null_edge_holding_another_pairs_cut_builds_in_every_op`]).
fn pit_holding_spikes(spikes: &[([[f64; 3]; 3], f64)], drops: &[&str], tol: Tol) {
    for spike_first in [false, true] {
        pit_holding_spikes_built(spikes, spike_first, drops, tol);
    }
}

/// [`pit_holding_spikes`] with each spike's union taken spike first or
/// the holed block first.
fn pit_holding_spikes_built(
    spikes: &[([[f64; 3]; 3], f64)],
    spike_first: bool,
    drops: &[&str],
    tol: Tol,
) {
    let pit = corner_prism(PIT, 3.0, tol);
    let block = finished(
        "block",
        brick::<f64>((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0), tol),
        tol,
    );
    let BooleanResult::Body(mut y) =
        subtract_with(&block, &pit, &flush_declarations(&block, &pit, tol), tol)
            .expect("the pit is cut")
    else {
        panic!("the block ∖ pit came back empty");
    };
    for &(rays, scale) in spikes {
        let spike = corner_prism(rays, scale, tol);
        let got = if spike_first {
            let mut decls = flush_declarations(&spike, &y.body, tol);
            decls.carried_b.vv = rest_rows(&y.contacts);
            union_with(&spike, &y.body, &decls, tol)
        } else {
            let mut decls = flush_declarations(&y.body, &spike, tol);
            decls.carried_a.vv = rest_rows(&y.contacts);
            union_with(&y.body, &spike, &decls, tol)
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
    let got = mass_properties(&y.body, tol).expect("mass").volume;
    assert!(
        (got - y_volume).abs() < 1e-9,
        "y: volume {got}, want {y_volume}"
    );
    assert_eq!(
        keys_at(&y.body, (0.0, 0.0, 0.0)).len(),
        1 + spikes.len(),
        "the pit's apex and each spike's corner"
    );
    for (op, got) in against_the_cube(&y.body, &rest_rows(&y.contacts), tol) {
        let want = match op {
            "y ∪ cube" | "cube ∪ y" => y_volume + 1.0 - common,
            "y ∖ cube" => y_volume - common,
            "cube ∖ y" => 1.0 - common,
            _ => common,
        };
        let dropped = drops.contains(&op);
        let op = format!("{op} (spike first: {spike_first})");
        let BooleanResult::Body(out) = got.unwrap_or_else(|e| panic!("{op} refused: {e:?}")) else {
            panic!("{op} came back empty");
        };
        let volume = mass_properties(&out.body, tol).expect("mass").volume;
        assert!(
            (volume - want).abs() < 1e-9,
            "{op}: volume {volume}, want {want}"
        );
        let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
        assert_eq!(verdict.is_err(), dropped, "{op}: 3′ {:?}", verdict.err());
    }
}

/// **One spike in the pit** ([`pit_holding_spikes`]). Its far corners
/// `0.5·(r₀ + r₂)` and `0.5·(r₁ + r₂)` rest on the cube's faces x = 1
/// and y = 1, inside the pit, and with the cube first ∖ drops the rest
/// at (0.9, 1, 0.05) and ∩ declares it on a face that survives only
/// away from it; a brick that never reaches the origin does the same,
/// so neither touches the origin's null edges
/// (`work/fuse/a-vertex-on-face-row-follows-its-face-not-the-part-it-rests-on.md`):
/// pinned as they stand. Red as the first row: the pit's dangling null
/// edge held the spike's cuts, and every op refused
/// `SharedVertexCrossings`. Red if the spike's strut hangs at the cube's
/// corner beside the pit's: the corner would be the In end of one and
/// the Out end of the other, an invariant.
#[test]
fn a_dangling_null_edge_holding_another_pairs_cut_builds_in_every_op() {
    let spike = [[1.0, 0.8, -0.1], [0.8, 1.0, -0.1], [1.0, 1.0, 0.2]];
    pit_holding_spikes(&[(spike, 0.5)], &["cube ∖ y", "cube ∩ y"], Tol::witness());
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
        pit_holding_spikes(&spikes, &[], Tol::witness());
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
/// its rays along `outer`'s at the ends `inner` names) put back: the
/// two pieces touch along pinch lines in the cube's bottom face. Against
/// the unit cube's corner, both pieces' pairs cross that face's corner
/// twice, so both are dangling null edges: the block's an Out run whose
/// other way round is the whole orbit, the lens's an In run inside it
/// that hangs at its tip. `y` is built in both orders of its union, and
/// every op, in both operand orders, builds at
/// the volume clipping each prism to each box gives, and passes 3′
/// with `y`'s records carried except in the ops `drops` names.
fn lens_in_a_lens(outer: [[[f64; 3]; 3]; 2], inner: [[[f64; 3]; 3]; 2], drops: &[&str], tol: Tol) {
    for (order, y) in lens_ys(outer, inner, tol) {
        lens_against_the_cube(
            &y,
            (outer, LENS_SCALES.0),
            (inner, LENS_SCALES.1),
            order,
            drops,
            tol,
        );
    }
}

/// The outer and inner lenses' scales in [`lens_ys`].
const LENS_SCALES: (f64, f64) = (3.0, 0.5);

/// [`lens_in_a_lens`]'s `y` in both orders of its union.
fn lens_ys(
    outer: [[[f64; 3]; 3]; 2],
    inner: [[[f64; 3]; 3]; 2],
    tol: Tol,
) -> [(&'static str, BooleanBody<f64>); 2] {
    let lens = |rays: [[[f64; 3]; 3]; 2], scale: f64| {
        let [up, down] = rays.map(|r| corner_prism(r, scale, tol));
        let BooleanResult::Body(lens) =
            union_with(&up, &down, &flush_declarations(&up, &down, tol), tol).expect("a lens")
        else {
            panic!("the lens came back empty");
        };
        lens
    };
    let k = lens(outer, LENS_SCALES.0);
    let block = finished(
        "block",
        brick::<f64>((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0), tol),
        tol,
    );
    let BooleanResult::Body(cut) = subtract_with(
        &block,
        &k.body,
        &flush_declarations(&block, &k.body, tol),
        tol,
    )
    .expect("the lens is cut") else {
        panic!("the block ∖ lens came back empty");
    };
    let piece = lens(inner, LENS_SCALES.1);
    let mut cut_first = flush_declarations(&cut.body, &piece.body, tol);
    cut_first.carried_a.vv = rest_rows(&cut.contacts);
    let mut piece_first = flush_declarations(&piece.body, &cut.body, tol);
    piece_first.carried_b.vv = rest_rows(&cut.contacts);
    [
        (
            "cut ∪ lens",
            union_with(&cut.body, &piece.body, &cut_first, tol),
        ),
        (
            "lens ∪ cut",
            union_with(&piece.body, &cut.body, &piece_first, tol),
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

/// [`lens_in_a_lens`]'s `y`, built in the `order` named, against the
/// cube's corner.
fn lens_against_the_cube(
    y: &BooleanBody<f64>,
    (outer, outer_scale): ([[[f64; 3]; 3]; 2], f64),
    (inner, inner_scale): ([[[f64; 3]; 3]; 2], f64),
    order: &str,
    drops: &[&str],
    tol: Tol,
) {
    let (cube, block) = (box_planes(0.0, 1.0), box_planes(-1.0, 1.0));
    let volume = |rays: [[[f64; 3]; 3]; 2], scale: f64, planes: &[([f64; 3], f64)]| {
        rays.iter()
            .map(|&r| clipped_volume(prism_faces(r, scale), planes))
            .sum::<f64>()
    };
    let common = 1.0 - volume(outer, outer_scale, &cube) + volume(inner, inner_scale, &cube);
    let y_volume = 8.0 - volume(outer, outer_scale, &block) + volume(inner, inner_scale, &block);
    let got = mass_properties(&y.body, tol).expect("mass").volume;
    assert!(
        (got - y_volume).abs() < 1e-9,
        "y: volume {got}, want {y_volume}"
    );
    assert_eq!(
        keys_at(&y.body, (0.0, 0.0, 0.0)).len(),
        2,
        "each piece's corner at the origin"
    );
    for (op, got) in against_the_cube(&y.body, &rest_rows(&y.contacts), tol) {
        let dropped = drops.contains(&op);
        let want = match op {
            "y ∪ cube" | "cube ∪ y" => y_volume + 1.0 - common,
            "y ∖ cube" => y_volume - common,
            "cube ∖ y" => 1.0 - common,
            _ => common,
        };
        let op = format!("{op} (y = {order})");
        let BooleanResult::Body(out) = got.unwrap_or_else(|e| panic!("{op} refused: {e:?}")) else {
            panic!("{op} came back empty");
        };
        let volume = mass_properties(&out.body, tol).expect("mass").volume;
        assert!(
            (volume - want).abs() < 1e-9,
            "{op}: volume {volume}, want {want}"
        );
        match validate_pseudomanifold(&out.body, &out.contacts, tol) {
            Ok(_) => assert!(!dropped, "{op}: passes 3′; the row's pin flips"),
            Err(errors) => {
                assert!(dropped, "{op}: 3′ refused {errors:?}");
                assert!(
                    format!("{errors:?}").contains("VertexVertex"),
                    "{op}: the undeclared pair at a pinch line's far end: {errors:?}"
                );
            }
        }
    }
}

/// The rays the cube's bottom face holds the lenses' pinch lines along.
const D1: [f64; 3] = [1.0, 0.3, 0.0];
const D2: [f64; 3] = [0.3, 1.0, 0.0];

/// **Two dangling null edges whose runs are one arc refuse typed**: the
/// inner lens's rays in the cube's face are the outer's
/// ([`lens_ys`]), so the two pieces touch along both, and their
/// pairs' dangling null edges have one segment, whose other way round
/// is the whole orbit. Neither nests the other (`insert::holds_whole`):
/// hanging either at the other's tip builds in one order of `y`'s union
/// and fails the join in the other. So every op, in both operand orders
/// and with `y` built both ways, refuses `SharedVertexCrossings` naming
/// the cube's corner and two of `y`'s vertices there. So does the inner
/// lens with a thinner lens cut out of it between the two rays, whose
/// pair's four crossings pair the outer two, with or without a third
/// lens in the notch: pinned as they stand
/// (`work/fuse/two-dangling-null-edges-with-one-segment-refuse-shared-vertex-crossings.md`).
#[test]
fn two_dangling_null_edges_with_one_segment_refuse_typed() {
    let tol = Tol::witness();
    let outer = lens_prisms((D1, D2), ([0.8, 0.8, 0.3], [0.8, 0.8, -0.3]));
    let inner = lens_prisms((D1, D2), ([0.8, 0.8, 0.15], [0.8, 0.8, -0.15]));
    let cube_corner = keys_at(
        &brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        (0.0, 0.0, 0.0),
    );
    let [(_, cut_first), lens_first] = lens_ys(outer, inner, tol);
    let notched = notched(&cut_first, None, tol);
    let filled = notched_filled(&cut_first, tol);
    for (order, y) in [
        ("cut ∪ lens", cut_first),
        lens_first,
        ("cut ∪ notched lens", notched),
        ("cut ∪ notched lens ∪ lens in the notch", filled),
    ] {
        let ends = keys_at(&y.body, (0.0, 0.0, 0.0));
        for (op, got) in against_the_cube(&y.body, &rest_rows(&y.contacts), tol) {
            let side = if op.starts_with("cube") {
                topo::Operand::A
            } else {
                topo::Operand::B
            };
            match got {
                Err(BooleanError::SharedVertexCrossings {
                    operand,
                    vertex,
                    partners: [p0, p1],
                }) => {
                    assert_eq!(
                        (operand, vertex),
                        (side, cube_corner[0]),
                        "{op} (y = {order}): the cube's corner"
                    );
                    assert!(
                        p0 != p1 && ends.contains(&p0) && ends.contains(&p1),
                        "{op} (y = {order}): two of y's vertices there, {ends:?}"
                    );
                }
                other => panic!(
                    "{op} (y = {order}): want SharedVertexCrossings, got {:?}",
                    other.map(|_| ())
                ),
            }
        }
    }
}

/// `cut ∪ lens` ([`lens_ys`]) with the inner lens notched: the lens over
/// `([1, 0.6, 0], [0.6, 1, 0])`, thinner and scaled past it, taken out
/// of the inner lens, then `fill` put back if given.
fn notched(y: &BooleanBody<f64>, fill: Option<&AtRestBody<f64>>, tol: Tol) -> BooleanBody<f64> {
    let lens = |rays: [[[f64; 3]; 3]; 2], scale: f64| {
        let [up, down] = rays.map(|r| corner_prism(r, scale, tol));
        let BooleanResult::Body(lens) =
            union_with(&up, &down, &flush_declarations(&up, &down, tol), tol).expect("a lens")
        else {
            panic!("the lens came back empty");
        };
        lens
    };
    let notch = lens(
        lens_prisms(
            ([1.0, 0.6, 0.0], [0.6, 1.0, 0.0]),
            ([0.8, 0.8, 0.1], [0.8, 0.8, -0.1]),
        ),
        3.0,
    );
    let mut decls = flush_declarations(&y.body, &notch.body, tol);
    decls.carried_a.vv = rest_rows(&y.contacts);
    let BooleanResult::Body(mut y) =
        subtract_with(&y.body, &notch.body, &decls, tol).expect("the notch is cut")
    else {
        panic!("the notched y came back empty");
    };
    if let Some(fill) = fill {
        let mut decls = flush_declarations(&y.body, fill, tol);
        decls.carried_a.vv = rest_rows(&y.contacts);
        let BooleanResult::Body(next) =
            union_with(&y.body, fill, &decls, tol).expect("the fill touches the notch's apex")
        else {
            panic!("the filled y came back empty");
        };
        y = next;
    }
    y
}

/// [`notched`] with a lens over `([1, 0.75, 0], [0.75, 1, 0])` in the
/// notch.
fn notched_filled(y: &BooleanBody<f64>, tol: Tol) -> BooleanBody<f64> {
    let [up, down] = lens_prisms(
        ([1.0, 0.75, 0.0], [0.75, 1.0, 0.0]),
        ([0.8, 0.8, 0.05], [0.8, 0.8, -0.05]),
    )
    .map(|r| corner_prism(r, 0.3, tol));
    let BooleanResult::Body(fill) =
        union_with(&up, &down, &flush_declarations(&up, &down, tol), tol).expect("a lens")
    else {
        panic!("the lens came back empty");
    };
    notched(y, Some(&fill.body), tol)
}

/// **A strut inside another's segment, sharing one end's direction**
/// ([`lens_in_a_lens`]): the inner lens's rays in the cube's face are
/// the outer's `D1` and a ray strictly between, so the pieces touch
/// along one pinch line. `y ∖ cube` drops the carried row at its far
/// end (`work/fuse/a-carried-row-whose-ends-split-into-null-edge-copies-is-dropped.md`):
/// pinned as it stands. Red as the first row: the cut strictly
/// inside the Out run's segment held it, and every op refused
/// `SharedVertexCrossings`.
#[test]
fn a_dangling_null_edge_inside_another_along_one_end_builds_in_every_op() {
    lens_in_a_lens(
        lens_prisms((D1, D2), ([0.8, 0.8, 0.3], [0.8, 0.8, -0.3])),
        lens_prisms((D1, [0.6, 1.0, 0.0]), ([0.8, 0.8, 0.15], [0.8, 0.8, -0.15])),
        &["y ∖ cube"],
        Tol::witness(),
    );
}

/// **A corner crossing the cube's corner four times refuses
/// `PairingMismatch`**
/// (`work/cleave/a-corner-crossing-another-four-times-refuses-pairing-mismatch.md`):
/// a thin trihedral corner at the origin whose cone holds the cube's
/// z-edge and crosses its bottom face, so the two corners' boundaries
/// cross four times, in two section-polygon edges. The record pairing's
/// B-adjacency guard orders two germs in one B sector by their A
/// sector rather than round the sector, and refuses every op, in both
/// operand orders: pinned as it stands.
#[test]
fn a_corner_crossing_the_cubes_four_times_refuses_pairing_mismatch() {
    let tol = Tol::witness();
    let band = corner_prism(
        [
            [0.5948, 0.757, -0.2704],
            [-0.5774, -0.5774, 0.5774],
            [0.757, 0.5948, -0.2704],
        ],
        0.4,
        tol,
    );
    for (op, got) in against_the_cube(&band, &[], tol) {
        assert!(
            matches!(got, Err(BooleanError::PairingMismatch { .. })),
            "{op}: want PairingMismatch, got {:?}",
            got.map(|_| ())
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
/// with each other; `y ∩ cube` and `cube ∪ y` refuse `JoinDesync` and
/// `cube ∖ y` refuses `Euler(SelfLoopEdge)`: pinned as they stand.
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
    let BooleanResult::Body(two) =
        union_with(&i, &j, &flush_declarations(&i, &j, tol), tol).expect("i ∪ j")
    else {
        panic!("i ∪ j came back empty");
    };
    let mut decls = flush_declarations(&two.body, &k, tol);
    decls.carried_a.vv = rest_rows(&two.contacts);
    let BooleanResult::Body(y) = union_with(&two.body, &k, &decls, tol).expect("∪ k") else {
        panic!("∪ k came back empty");
    };
    let y_volume = mass_properties(&y.body, tol).expect("mass").volume;
    let mut volumes = std::collections::BTreeMap::new();
    for (op, got) in against_the_cube(&y.body, &rest_rows(&y.contacts), tol) {
        match (op, got) {
            ("y ∩ cube" | "cube ∪ y", Err(BooleanError::JoinDesync { .. }))
            | ("cube ∖ y", Err(BooleanError::Euler(_))) => {}
            ("y ∪ cube" | "y ∖ cube" | "cube ∩ y", Ok(BooleanResult::Body(out))) => {
                let verdict = validate_pseudomanifold(&out.body, &out.contacts, tol);
                assert!(verdict.is_ok(), "{op}: 3′ refused {:?}", verdict.err());
                volumes.insert(op, mass_properties(&out.body, tol).expect("mass").volume);
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
