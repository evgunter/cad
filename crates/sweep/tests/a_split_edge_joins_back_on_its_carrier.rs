//! **A split edge joins back on its own carrier** (`Body::join_edges`,
//! `topo::boolean::edge_join`): a split restates its parent's carrier
//! verbatim on both children, so the join carries the killed child's own
//! end parameter as the far parameter, with no inverse and no period.
//! One row per carrier kind splits an edge of a finished body at an
//! interior parameter and joins it back: the one join takes the split
//! vertex, and every edge's carrier and interval come back bit for bit.
//! A reading off the carrier is no exact inverse of the split's
//! parameter, which the grid row sees on every analytic kind. A spline
//! carrier has no inverse at all, so its rows, open and closed, are the
//! join's only path; a spline beside a piece of another carrier still
//! refuses typed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_core::{Band, Point3, Tol};
use topo::construction::Construct;
use topo::{Body, EdgeKey};

use crate::common::torus_walls::vessel_cavity;
use crate::conic_edge_curved_face::drum_lower;

fn certified(body: &Body<f64>, e: EdgeKey) -> &geom_brep::EdgeCurve<f64> {
    body.get_curve_geom(body.get_edge(e).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
}

fn ends(body: &Body<f64>, e: EdgeKey) -> (Point3<f64>, Point3<f64>) {
    let d = body.get_edge(e).unwrap();
    let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    (
        p(body.get_half_edge(d.he_plus).unwrap().start),
        p(body.half_edge_end(d.he_plus).unwrap()),
    )
}

/// Every edge's carrier and interval, by value, with its description's
/// variant: what a split then a join must give back.
fn curves(body: &Body<f64>) -> Vec<String> {
    let mut out: Vec<String> = body
        .edges()
        .map(|(e, _)| {
            let c = certified(body, e);
            let (t0, t1) = c.params();
            let kind = format!("{:?}", c.description());
            let kind = kind.split([' ', '(', '{']).next().unwrap_or("").to_owned();
            format!(
                "{:?} [{:#x}, {:#x}] {kind}",
                c.carrier(),
                t0.to_bits(),
                t1.to_bits()
            )
        })
        .collect();
    out.sort();
    out
}

/// Splits `edge` at `t` and joins: the one join takes the split vertex,
/// and the body's edges come back as found.
fn splits_and_joins_back(what: &str, mut body: Body<f64>, edge: EdgeKey, t: f64) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let before = curves(&body);
    let (faces, vertices) = (body.faces().count(), body.vertices().count());
    let split = body.split_edge(edge, t, tol).unwrap();
    let joins = body.join_edges(band, tol).unwrap();
    assert_eq!(joins.len(), 1, "{what}: one join");
    assert_eq!(
        joins[0].vertex, split.vertex,
        "{what}: the join takes the split vertex"
    );
    assert!(
        [edge, split.new_edge].contains(&joins[0].kept)
            && [edge, split.new_edge].contains(&joins[0].gone),
        "{what}: the join makes the two children one"
    );
    assert_eq!(
        curves(&body),
        before,
        "{what}: every carrier and interval as found"
    );
    assert_eq!(
        (body.faces().count(), body.vertices().count()),
        (faces, vertices),
        "{what}: the cells as found"
    );
    assert!(
        topo::joinable_vertices(&body, band).unwrap().is_empty(),
        "{what}: nothing left joinable"
    );
}

fn mid(body: &Body<f64>, e: EdgeKey) -> f64 {
    let (t0, t1) = certified(body, e).params();
    // Off the middle, so the split lands at no value a carrier's
    // structure singles out.
    t0 + 0.37 * (t1 - t0)
}

fn first_of(body: &Body<f64>, kind: geom::CurveKind) -> EdgeKey {
    body.edges()
        .map(|(e, _)| e)
        .find(|&e| certified(body, e).carrier().kind() == kind)
        .unwrap_or_else(|| panic!("the fixture carries a {kind:?} edge"))
}

#[test]
fn each_analytic_carrier_kind_joins_a_split_edge_back_as_found() {
    let tol = Tol::witness();
    let disc = sweep::test_support::disc_of_arcs(4, 1.0, 1.0, tol);
    let arc = first_of(&disc, geom::CurveKind::Circle);
    splits_and_joins_back("a rim arc (Circle)", disc.clone(), arc, mid(&disc, arc));
    let strut = first_of(&disc, geom::CurveKind::Line);
    splits_and_joins_back(
        "a wall strut (Line)",
        disc.clone(),
        strut,
        mid(&disc, strut),
    );

    let drum = drum_lower().into_body();
    let rim = first_of(&drum, geom::CurveKind::Ellipse);
    splits_and_joins_back("a cut rim (Ellipse)", drum.clone(), rim, mid(&drum, rim));

    let (_, cavity) = vessel_cavity(1.0 / 128.0);
    let spiric = first_of(&cavity, geom::CurveKind::Spiric);
    splits_and_joins_back(
        "a torus band's rim (Spiric)",
        cavity.clone(),
        spiric,
        mid(&cavity, spiric),
    );

    let tube = tube();
    let rim = outer_top_rim(&tube);
    splits_and_joins_back("a closed rim (Circle)", tube.clone(), rim, mid(&tube, rim));
}

/// A tube of radii 0.5 and 1 over `y ∈ [0, 1]`, revolved about `y`:
/// each rim one closed circle, whose vertex the wall's seam also leaves.
fn tube() -> Body<f64> {
    let profile = vec![
        (geom_core::Point2::new(0.5, 0.0), 0.0),
        (geom_core::Point2::new(1.0, 0.0), 0.0),
        (geom_core::Point2::new(1.0, 1.0), 0.0),
        (geom_core::Point2::new(0.5, 1.0), 0.0),
    ];
    sweep::test_support::revolved_about_y(profile, sweep::Revolution::Full, Tol::witness())
}

fn outer_top_rim(body: &Body<f64>) -> EdgeKey {
    body.edges()
        .map(|(e, _)| e)
        .find(|&e| match certified(body, e).carrier() {
            geom::Curve3::Circle(c) => c.radius == 1.0 && c.frame.center.y == 1.0,
            _ => false,
        })
        .expect("the tube's outer top rim")
}

/// The disc's top rim restated on spline carriers: `carrier((p0, p1),
/// m)` gives a top arc running `p0 → p1` through `m` its carrier and
/// interval.
fn spline_rimmed_disc(
    n: usize,
    carrier: impl Fn((Point3<f64>, Point3<f64>), Point3<f64>) -> (geom::NurbsCurve3<f64>, f64, f64),
) -> (Body<f64>, Vec<EdgeKey>) {
    let tol = Tol::witness();
    let mut body = sweep::test_support::disc_of_arcs(n, 1.0, 1.0, tol);
    let top: Vec<EdgeKey> = body
        .edges()
        .map(|(e, _)| e)
        .filter(|&e| {
            let (p, q) = ends(&body, e);
            p.z == 1.0 && q.z == 1.0
        })
        .collect();
    assert_eq!(top.len(), n, "the top rim's arcs");
    for &e in &top {
        let geom_brep::EdgeDescription::Intersection { pair, .. } =
            *certified(&body, e).description()
        else {
            panic!("a rim arc is the cylinder and cap's intersection");
        };
        let (t0, t1) = certified(&body, e).params();
        let m = certified(&body, e).carrier().mid_point(t0, t1);
        let (net, a, b) = carrier(ends(&body, e), m);
        let witness = net.eval(0.5 * (a + b));
        body.set_edge_curve(
            e,
            topo::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    pair,
                    selector: geom_brep::CurveSelector::Witness(witness),
                },
                carrier: geom::Curve3::Nurbs(Arc::new(net)),
                param_start: a,
                param_end: b,
            },
            tol,
        )
        .unwrap();
    }
    (body, top)
}

/// The arc `p0 → p1` about the top cap's centre as its exact rational
/// quadratic over `[0, 1]`: its ends, the corner their tangents meet at,
/// and the corner's weight `cos(θ/2)`.
fn rational_quarter((p0, p1): (Point3<f64>, Point3<f64>)) -> (geom::NurbsCurve3<f64>, f64, f64) {
    let centre = Point3::new(0.0, 0.0, 1.0);
    let corner = centre + (p0 - centre) + (p1 - centre);
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let net = geom::NurbsCurve3::new(
        kv,
        vec![p0, corner, p1],
        vec![1.0, core::f64::consts::FRAC_1_SQRT_2, 1.0],
    )
    .unwrap();
    (net, 0.0, 1.0)
}

#[test]
fn a_split_spline_edge_joins_back_as_found() {
    let (body, top) = spline_rimmed_disc(4, |ends, _| rational_quarter(ends));
    splits_and_joins_back("a rim arc (Nurbs)", body, top[0], 0.37);
}

/// `rim`'s circle as one rational span of degree 6 over `[0, 1]`, from
/// the rim's own start, the way the rim runs: C¹ by its knot structure
/// (no interior knot), as every carrier at rest is (DESIGN, seams). It
/// is the stereographic circle `((1 − τ²), 2τ)/(1 + τ²)` reparametrized
/// by `τ = (2s³ − 3s² − s + 1)/(4s(1 − s))`, which sweeps `τ` over the
/// line once, so the span closes at `s = 0` and `s = 1`, every weight
/// positive. Below, the plane's coordinates are `(−x, −y)` of that
/// circle, so `s = 0` is the rim's start and the run leaves along it.
fn as_one_span_spline(body: &mut Body<f64>, rim: EdgeKey) {
    let c = certified(body, rim).clone();
    let geom::Curve3::Circle(circle) = c.carrier() else {
        panic!("a circle rim");
    };
    let geom_brep::EdgeDescription::Intersection { pair, .. } = *c.description() else {
        panic!("a rim is its faces' intersection");
    };
    let (t0, _) = c.params();
    let centre = circle.frame.center;
    let u = c.carrier().ders1(t0).0 - centre;
    let v = c.carrier().ders1(t0).1;
    let v = v / v.norm() * circle.radius;
    let at = |x: f64, y: f64| centre + u * (-x) + v * (-y);
    let kv = geom_core::spline::KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ],
        6,
    )
    .unwrap();
    let net = geom::NurbsCurve3::new(
        kv,
        vec![
            at(-1.0, 0.0),
            at(-1.0, -2.0),
            at(1.0, -1.5),
            at(21.0 / 11.0, 0.0),
            at(1.0, 1.5),
            at(-1.0, 2.0),
            at(-1.0, 0.0),
        ],
        vec![
            1.0,
            2.0 / 3.0,
            16.0 / 15.0,
            1.1,
            16.0 / 15.0,
            2.0 / 3.0,
            1.0,
        ],
    )
    .unwrap();
    let witness = net.eval(0.5);
    body.set_edge_curve(
        rim,
        topo::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                pair,
                selector: geom_brep::CurveSelector::Witness(witness),
            },
            carrier: geom::Curve3::Nurbs(Arc::new(net)),
            param_start: 0.0,
            param_end: 1.0,
        },
        Tol::witness(),
    )
    .unwrap();
}

#[test]
fn a_closed_spline_edge_split_once_joins_back_into_one_closed_edge() {
    let mut body = tube();
    let rim = outer_top_rim(&body);
    as_one_span_spline(&mut body, rim);
    splits_and_joins_back("a closed rim (Nurbs)", body, rim, 0.37);
}

#[test]
fn a_spline_beside_a_piece_of_another_carrier_refuses_typed() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let (mut body, top) = spline_rimmed_disc(4, |ends, _| rational_quarter(ends));
    let split = body.split_edge(top[0], 0.37, tol).unwrap();
    // The second child restated on its own clamped piece of the
    // carrier: one locus, a carrier of other values, creased at the
    // split vertex, so the first child is kept and runs on.
    let child = certified(&body, split.new_edge).clone();
    let geom::Curve3::Nurbs(net) = child.carrier() else {
        panic!("a spline child");
    };
    let (_, piece) = net.split_at(0.37).unwrap();
    body.set_edge_curve(
        split.new_edge,
        topo::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                pair: match child.description() {
                    geom_brep::EdgeDescription::Intersection { pair, .. } => *pair,
                    other => panic!("{other:?}"),
                },
                selector: geom_brep::CurveSelector::Witness(piece.eval(0.5 * (0.37 + 1.0))),
            },
            carrier: geom::Curve3::Nurbs(Arc::new(piece)),
            param_start: 0.37,
            param_end: 1.0,
        },
        tol,
    )
    .unwrap();
    assert_eq!(
        topo::joinable_vertices(&body, band).unwrap(),
        vec![split.vertex],
        "the split vertex is joinable"
    );
    let refusal = body.join_edges(band, tol).unwrap_err();
    assert!(
        matches!(
            refusal,
            topo::BooleanError::JoinCarrierUnsupported {
                edge,
                carrier: geom::CurveKind::Nurbs,
                closed: false,
            } if edge == top[0]
        ),
        "{refusal:?}"
    );
}

/// Splits `edge` at each of `fractions` of its interval, joins, and
/// returns the fractions at which the joined edge's interval is not
/// the parent's bit for bit.
fn off_grid(body: &Body<f64>, edge: EdgeKey, fractions: &[f64]) -> Vec<(f64, u64, u64)> {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let (t0, t1) = certified(body, edge).params();
    let mut off = Vec::new();
    for &f in fractions {
        let mut b = body.clone();
        let t = t0 + f * (t1 - t0);
        let split = b.split_edge(edge, t, tol).unwrap();
        let joins = b.join_edges(band, tol).unwrap();
        assert_eq!(joins.len(), 1, "f = {f}: one join");
        assert_eq!(joins[0].vertex, split.vertex, "f = {f}: the split vertex");
        let (a, z) = certified(&b, joins[0].kept).params();
        if (a.to_bits(), z.to_bits()) != (t0.to_bits(), t1.to_bits()) {
            off.push((f, a.to_bits(), z.to_bits()));
        }
    }
    off
}

fn grid() -> Vec<f64> {
    (1..64).map(|i| f64::from(i) / 64.0 + 1e-3).collect()
}

/// Each analytic kind split over a grid of parameters joins back on the
/// parent's interval bit for bit. Without the carry rung, the reading
/// off the carrier lands some ulps off at some fractions (a circle's at 15
/// of the 63, an ellipse's at 3), so this row pins the rung's wiring on
/// those kinds; a line's reading lands exactly at every fraction, so the
/// line's rung is the `carry` unit row's (`topo::boolean::edge_join`).
#[test]
fn each_analytic_kind_joins_back_exactly_across_a_grid_of_splits() {
    let tol = Tol::witness();
    let disc = sweep::test_support::disc_of_arcs(4, 1.0, 1.0, tol);
    let drum = drum_lower().into_body();
    let moved = [
        ("rim arc (Circle)", &disc, geom::CurveKind::Circle),
        ("wall strut (Line)", &disc, geom::CurveKind::Line),
        ("cut rim (Ellipse)", &drum, geom::CurveKind::Ellipse),
    ]
    .map(|(what, body, kind)| (what, off_grid(body, first_of(body, kind), &grid())));
    for (what, off) in &moved {
        assert!(
            off.is_empty(),
            "{what}: joined interval moved at {off:x?} ({moved:x?})"
        );
    }
}

/// A spline with an interior knot of full multiplicity, split off the
/// knot, joins back across it (the parent spanned it already), and split
/// AT it keeps the vertex (both children crease there). Such a carrier
/// breaks the ruling that every carrier at rest is C¹ by its knot
/// structure (DESIGN, seams), so this row pins today's behaviour only,
/// until `measuredoor/tier-3-refuses-a-carrier-with-an-interior-c0-knot`
/// refuses the carrier at rest; it then goes red, and goes.
#[test]
fn a_spline_with_a_c0_knot_joins_across_it_and_keeps_the_vertex_split_at_it() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut body = sweep::test_support::disc_of_arcs(4, 1.0, 1.0, tol);
    let arc = body
        .edges()
        .map(|(e, _)| e)
        .find(|&e| {
            let c = certified(&body, e);
            matches!(c.carrier(), geom::Curve3::Circle(_))
                && matches!(
                    c.description(),
                    geom_brep::EdgeDescription::Intersection { .. }
                )
                && {
                    let d = body.get_edge(e).unwrap();
                    let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                    p(body.get_half_edge(d.he_plus).unwrap().start).z == 1.0
                        && p(body.half_edge_end(d.he_plus).unwrap()).z == 1.0
                }
        })
        .unwrap();
    let c = certified(&body, arc).clone();
    let geom_brep::EdgeDescription::Intersection { pair, .. } = *c.description() else {
        unreachable!()
    };
    let (t0, t1) = c.params();
    let at = |t: f64| c.carrier().ders1(t).0;
    let (p0, p1, pm) = (at(t0), at(t1), at(0.5 * (t0 + t1)));
    let geom::Curve3::Circle(circle) = c.carrier() else {
        unreachable!()
    };
    let centre = circle.frame.center;
    // The arc as two rational quadratic halves joined at a knot of
    // multiplicity two (C0 in its parameter), over [0, 2].
    let corner = |a: geom_core::Point3<f64>, b: geom_core::Point3<f64>| {
        let (u, v) = (a - centre, b - centre);
        let half = (u.dot(v) / (u.norm() * v.norm())).acos() / 2.0;
        centre + (u + v) * (0.5 / half.cos().powi(2))
    };
    let half_w = {
        let (u, v) = (p0 - centre, pm - centre);
        ((u.dot(v) / (u.norm() * v.norm())).acos() / 2.0).cos()
    };
    let kv =
        geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0], 2)
            .unwrap();
    let net = geom::NurbsCurve3::new(
        kv,
        vec![p0, corner(p0, pm), pm, corner(pm, p1), p1],
        vec![1.0, half_w, 1.0, half_w, 1.0],
    )
    .unwrap();
    body.set_edge_curve(
        arc,
        topo::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                pair,
                selector: geom_brep::CurveSelector::Witness(net.eval(1.0)),
            },
            carrier: geom::Curve3::Nurbs(Arc::new(net)),
            param_start: 0.0,
            param_end: 2.0,
        },
        tol,
    )
    .expect("a spline with an interior C0 knot is accepted at rest");
    // Split off the knot then join: the carry spans the C0 knot that
    // the parent already spanned.
    let mut b = body.clone();
    b.split_edge(arc, 0.6, tol).unwrap();
    let joins = b.join_edges(band, tol).unwrap();
    assert_eq!(joins.len(), 1, "off the knot: joins");
    assert_eq!(
        certified(&b, joins[0].kept).params(),
        (0.0, 2.0),
        "the parent's interval"
    );
    // Split AT the knot: both children crease there, the vertex stays.
    let split = body.split_edge(arc, 1.0, tol).unwrap();
    assert!(
        !topo::joinable_vertices(&body, band)
            .unwrap()
            .contains(&split.vertex),
        "the knot's vertex is not joinable"
    );
    assert!(body.join_edges(band, tol).unwrap().is_empty(), "no join");
}
