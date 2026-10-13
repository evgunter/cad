//! probe (step 0)
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Real;
use geom_core::spline::{KnotVector, Knots};
use std::sync::Arc;

use geom::{NurbsCurve2, NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, Pcurve};
use geom_core::{Point2, Point3, Tol};
use topo::{AtRestPolicy, Body, FaceSurface, MefSite, MevSite};

fn probe<T: AtRestPolicy>() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let pt = |(x, y): (f64, f64)| Point3::new(T::from_f64(x), T::from_f64(y), T::zero());
    let mut body = Body::<T>::new();
    let seed = body.mvfs(pt((0.0, 0.0)), true).unwrap();
    let unit = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let net = NurbsSurface::new(
        Knots::from(unit()).lift(Real::from_f64),
        Knots::from(unit()).lift(Real::from_f64),
        [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)].into_iter().map(pt).collect(),
        vec![Real::one(); 4],
    )
    .unwrap();
    let chart = body
        .set_face_surface(seed.face, FaceSurface::New { surface: Surface::Nurbs(Arc::new(net)), sense: true })
        .unwrap();
    let ab = body.mev_line(MevSite::Lone { r#loop: seed.r#loop }, pt((1.0, 0.0)), tol).unwrap();
    let bc = body.mev_line(MevSite::Fan { he1: ab.he_minus, he2: ab.he_minus }, pt((1.0, 1.0)), tol).unwrap();
    let he_cb = body.find_half_edge(seed.face, bc.vertex, ab.vertex).unwrap();
    let closed = body.mef_chord(MefSite::Chords { he1: he_cb, he2: ab.he_plus }, tol).unwrap();
    let chord = closed.edge;
    let edges: Vec<_> = body.edges().map(|(k, e)| (k, e.he_plus)).collect();
    for (edge, he_plus) in edges {
        if edge == chord { continue; }
        let start = body.get_half_edge(he_plus).unwrap().start;
        let end = body.get_half_edge(body.get_half_edge(he_plus).unwrap().next).unwrap().start;
        let at = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let (p, q) = (at(start), at(end));
        let image = NurbsCurve2::new(Knots::from(unit()).lift(Real::from_f64), vec![Point2::new(p.x, p.y), Point2::new(q.x, q.y)], vec![Real::one(); 2]).unwrap();
        let line = EdgeCurveSpec::line_between(p, q);
        body.set_edge_curve(edge, EdgeCurveSpec { description: EdgeDescriptionSpec::chart_image(chart, Pcurve::General(Arc::new(image))), carrier: line.carrier, param_start: line.param_start, param_end: line.param_end }, tol).unwrap();
    }
    let he = body.get_edge(chord).unwrap().he_plus;
    eprintln!("{} chord desc: {:?}", T::NAME, body.get_curve_geom(body.get_edge(chord).unwrap().curve).and_then(|c| c.certified()).map(|c| format!("{:?}", c.description())).map(|s| s[..s.len().min(80)].to_string()));
    eprintln!("{} pcurve_of: {:?}", T::NAME, topo::pcurve_of(&body, he, band));
    eprintln!("{} mint: {:?}", T::NAME, topo::mint_pcurves_of(&mut body, &[seed.face, closed.face], tol));
}

#[test]
fn probe_f64() { probe::<f64>(); }
#[test]
fn probe_interval() { probe::<geom_core::Interval>(); }
#[test]
fn probe_dual() { probe::<geom_core::Dual64>(); }
