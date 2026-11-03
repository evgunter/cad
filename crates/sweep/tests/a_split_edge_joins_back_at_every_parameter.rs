//! **A split edge joins back on its carrier at every split parameter**
//! (`topo::boolean::edge_join::carried_far`): the carry rung's rows,
//! read through the public `split_edge` + `join_edges` doors, over a
//! grid of split parameters rather than one, so a reading off the
//! carrier (the inverse, `param_near`) that lands an ulp off the
//! parent's interval anywhere on the grid turns the row red. Each
//! analytic kind's row therefore pins the rung's wiring in
//! `joined_spec`, not only `carried_far` in isolation. A last row splits
//! a spline with an interior knot of full multiplicity AT that knot: the
//! vertex the knot puts there is never joined.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_core::{Band, Tol};
use topo::construction::Construct;
use topo::{Body, EdgeKey};

use crate::conic_edge_curved_face::drum_lower;

fn certified(body: &Body<f64>, e: EdgeKey) -> &geom_brep::EdgeCurve<f64> {
    body.get_curve_geom(body.get_edge(e).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
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

fn first_of(body: &Body<f64>, kind: geom::CurveKind) -> EdgeKey {
    body.edges()
        .map(|(e, _)| e)
        .find(|&e| certified(body, e).carrier().kind() == kind)
        .unwrap_or_else(|| panic!("the fixture carries a {kind:?} edge"))
}

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

#[test]
fn a_spline_split_at_its_full_multiplicity_knot_keeps_the_vertex() {
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
