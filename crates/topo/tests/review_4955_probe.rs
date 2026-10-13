//! Review probes for PR 4955 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_brep::Pcurve;
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Band, Point3, Real, Tol};
use std::sync::Arc;
use topo::{Body, FaceSurface, MefSite, MevSite};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A planar chart `z = 0` whose boundary row `v = 0` is the cubic
/// `y = k·u(u − ¼)²` (k = −½), and a straight edge along `y = 0` from
/// x = 0 to x = 1. The row meets the carrier at t0 and touches it at
/// span/4, so the probe accepts it, but it is off the carrier by
/// 0.28 m at t1.
#[test]
fn a_row_matching_the_carrier_at_the_probe_but_not_elsewhere_is_never_stored() {
    let tol = Tol::witness();
    let k = -0.5;
    let g = [0.0, 1.0 / 48.0, -0.125, 0.5625];
    let mut cps = Vec::new();
    for i in 0..4 {
        for j in 0..2 {
            let u = i as f64 / 3.0;
            cps.push(Point3::new(u, j as f64 + k * g[i], 0.0));
        }
    }
    let net = NurbsSurface::new(
        Knots::from(KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap())
            .lift(Real::from_f64),
        Knots::from(KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap())
            .lift(Real::from_f64),
        cps,
        vec![1.0; 8],
    )
    .unwrap();
    // sanity: the chart's row v=0 at u=0.25 is on y=0; at u=1 it is not.
    let s = Surface::Nurbs(Arc::new(net));
    let p = s.eval(0.25, 0.0);
    eprintln!("S(0.25,0) = {p:?}; S(1,0) = {:?}", s.eval(1.0, 0.0));
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New { surface: s, sense: true })
        .unwrap();
    let row = body
        .mev_line(MevSite::Lone { r#loop: seed.r#loop }, Point3::new(1.0, 0.0, 0.0), tol)
        .unwrap();
    let column = body
        .mev_line(
            MevSite::Fan { he1: row.he_minus, he2: row.he_minus },
            Point3::new(1.0, 0.5, 0.0),
            tol,
        )
        .unwrap();
    let back = body.find_half_edge(seed.face, column.vertex, row.vertex).unwrap();
    let closed = body
        .mef_chord(MefSite::Chords { he1: back, he2: row.he_plus }, tol)
        .unwrap();
    let faces = [seed.face, closed.face];
    let he = body.get_edge(row.edge).unwrap().he_plus;
    let derived = topo::pcurve_of(&body, he, band());
    eprintln!("derived row-edge image: {derived:?}");
    if let Ok(image) = derived.clone() {
        let curve = body
            .get_curve_geom(body.get_edge(row.edge).unwrap().curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap();
        let (t0, t1) = curve.params();
        let wall = body.get_surface(body.get_face(seed.face).unwrap().surface).unwrap().clone();
        let got = geom_brep::PcurveCache::certify(image, t0, t1, curve.carrier(), &wall, band());
        eprintln!("certify(wrong row) = {:?}", got.as_ref().map(|_| "CERTIFIED").map_err(|e| format!("{e:?}")));
        assert!(got.is_err(), "the wrong row certifies");
    }
    let picked_wrong_row = matches!(derived, Ok(Pcurve::IsoLine { pl, .. }) if pl.y == 0.0);
    let minted = topo::mint_pcurves_of(&mut body, &faces, tol);
    eprintln!("mint: {minted:?}");
    for h in [he, body.get_edge(row.edge).unwrap().he_minus] {
        if let Some(c) = body.pcurve(h) {
            eprintln!("stored {h:?}: {:?}", c.pcurve());
            assert!(
                !matches!(c.pcurve(), Pcurve::IsoLine { pl, .. } if pl.y == 0.0),
                "a wrong row is stored at rest"
            );
        }
    }
    eprintln!("picked_wrong_row = {picked_wrong_row}");
    eprintln!("validate: {:?}", topo::pcurves::validate_pcurves(&body, band()));
}

/// Degree-4 variant: g = u(u-1/4)^2(u-1), so the wrong row agrees at t0, span/4 AND t1.
/// A planar chart `z = 0` whose boundary row `v = 0` is the cubic
/// `y = k·u(u − ¼)²` (k = −½), and a straight edge along `y = 0` from
/// x = 0 to x = 1. The row meets the carrier at t0 and touches it at
/// span/4, so the probe accepts it, but it is off the carrier by
/// 0.28 m at t1.
#[test]
fn a_row_matching_the_carrier_at_t0_probe_and_t1_is_never_stored() {
    let tol = Tol::witness();
    let k = 1.0;
    let g = [0.0, -0.015625, 0.0625, -0.140625, 0.0];
    let mut cps = Vec::new();
    for i in 0..5 {
        for j in 0..2 {
            let u = i as f64 / 4.0;
            cps.push(Point3::new(u, j as f64 + k * g[i], 0.0));
        }
    }
    let net = NurbsSurface::new(
        Knots::from(KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0], 4).unwrap())
            .lift(Real::from_f64),
        Knots::from(KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap())
            .lift(Real::from_f64),
        cps,
        vec![1.0; 10],
    )
    .unwrap();
    // sanity: the chart's row v=0 at u=0.25 is on y=0; at u=1 it is not.
    let s = Surface::Nurbs(Arc::new(net));
    let p = s.eval(0.25, 0.0);
    eprintln!("S(0.25,0) = {p:?}; S(1,0) = {:?}", s.eval(1.0, 0.0));
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New { surface: s, sense: true })
        .unwrap();
    let row = body
        .mev_line(MevSite::Lone { r#loop: seed.r#loop }, Point3::new(1.0, 0.0, 0.0), tol)
        .unwrap();
    let column = body
        .mev_line(
            MevSite::Fan { he1: row.he_minus, he2: row.he_minus },
            Point3::new(1.0, 0.5, 0.0),
            tol,
        )
        .unwrap();
    let back = body.find_half_edge(seed.face, column.vertex, row.vertex).unwrap();
    let closed = body
        .mef_chord(MefSite::Chords { he1: back, he2: row.he_plus }, tol)
        .unwrap();
    let faces = [seed.face, closed.face];
    let he = body.get_edge(row.edge).unwrap().he_plus;
    let derived = topo::pcurve_of(&body, he, band());
    eprintln!("derived row-edge image: {derived:?}");
    if let Ok(image) = derived.clone() {
        let curve = body
            .get_curve_geom(body.get_edge(row.edge).unwrap().curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap();
        let (t0, t1) = curve.params();
        let wall = body.get_surface(body.get_face(seed.face).unwrap().surface).unwrap().clone();
        let got = geom_brep::PcurveCache::certify(image, t0, t1, curve.carrier(), &wall, band());
        eprintln!("certify(wrong row) = {:?}", got.as_ref().map(|_| "CERTIFIED").map_err(|e| format!("{e:?}")));
        assert!(got.is_err(), "the wrong row certifies");
    }
    let picked_wrong_row = matches!(derived, Ok(Pcurve::IsoLine { pl, .. }) if pl.y == 0.0);
    let minted = topo::mint_pcurves_of(&mut body, &faces, tol);
    eprintln!("mint: {minted:?}");
    for h in [he, body.get_edge(row.edge).unwrap().he_minus] {
        if let Some(c) = body.pcurve(h) {
            eprintln!("stored {h:?}: {:?}", c.pcurve());
            assert!(
                !matches!(c.pcurve(), Pcurve::IsoLine { pl, .. } if pl.y == 0.0),
                "a wrong row is stored at rest"
            );
        }
    }
    eprintln!("picked_wrong_row = {picked_wrong_row}");
    eprintln!("validate: {:?}", topo::pcurves::validate_pcurves(&body, band()));
}
