//! **A tangent seam on a spline wall, which no iso lane derives, ends as
//! the wall's incidence read.** A `TangentIntersection` edge on a spline
//! chart reaches `topo::pcurves::nurbs_iso_derive`'s last arm, which has
//! no lane for it and refuses `IsoUnsupported`, "not yet": behind the
//! chart's incidence read (`geom_brep::read_edge_incidence`), a carrier
//! shown off the wall ends `CarrierOffChart` and one read in band ends in
//! the read's lever instead.
//!
//! The fixture is a block with one top edge filleted, its quarter
//! cylinder then swapped (`Body::with_surface_moved_for_tests`) for a
//! rational quadratic net of the same cylinder over the fillet's turn and
//! length, of the fillet's radius, a millimetre more, or a band-scale
//! distance more. The fillet's two tangent seams are lines on it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_brep::{PcurveCertifyError, PcurveCheck};
use geom_core::spline::KnotVector;
use geom_core::{Band, Tol};
use sweep::blend::fillet_edges;
use sweep::test_support::{at_rest, block};
use topo::construction::Construct;
use topo::{Body, EdgeDescription, HalfEdgeKey, PcurveMintError};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// The fillet's radius.
const RADIUS: f64 = 0.2;

/// The block `[0, 2]² × [0, 1]` with its top edge along `x` at `y = 0`
/// filleted, the fillet's cylinder key, the cylinder, and the fillet's
/// tangent seams' half-edges on it.
fn filleted() -> (Body<f64>, topo::SurfaceKey, Surface<f64>, Vec<HalfEdgeKey>) {
    let body: Body<f64> = block(2.0, 2.0, 1.0, tol());
    let edge = body
        .edges()
        .map(|(k, _)| k)
        .find(|&k| {
            <[_; 2]>::from(body.edge_vertices(k).unwrap())
                .map(|v| topo::readback::vertex_point(&body, v).unwrap())
                .iter()
                .all(|p| p.y == 0.0 && p.z == 1.0)
        })
        .expect("the block has its top front edge");
    let body = fillet_edges(&at_rest(&body, tol()), &[edge], RADIUS, tol())
        .unwrap_or_else(|r| panic!("{}", r.render(geom_brep::recourse::Reading::Build)))
        .body;
    let (face, key, cylinder) = body
        .faces()
        .find_map(|(k, f)| match body.get_surface(f.surface) {
            Some(s @ Surface::Cylinder { .. }) => Some((k, f.surface, s.clone())),
            _ => None,
        })
        .expect("the fillet is a cylinder");
    let seams = body
        .half_edges()
        .map(|(h, _)| h)
        .filter(|&h| body.face_of_half_edge(h) == Some(face))
        .filter(|&h| {
            let edge = body.get_half_edge(h).unwrap().edge;
            body.get_curve_geom(body.get_edge(edge).unwrap().curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(|c| {
                    matches!(c.description(), EdgeDescription::TangentIntersection { .. })
                })
        })
        .collect::<Vec<_>>();
    assert_eq!(seams.len(), 2, "the fillet has two tangent seams");
    (body, key, cylinder, seams)
}

/// A rational quadratic net of `cylinder` at radius `radius`, over the
/// azimuths and axial heights its seams reach, with a margin each way.
fn net_of(
    body: &Body<f64>,
    cylinder: &Surface<f64>,
    seams: &[HalfEdgeKey],
    radius: f64,
) -> Surface<f64> {
    let Surface::Cylinder {
        origin,
        axis,
        u_ref,
        ..
    } = *cylinder
    else {
        unreachable!()
    };
    let (e1, e3) = (u_ref.normalize(), axis.normalize());
    let e2 = e3.cross(e1);
    let (mut azimuths, mut heights) = (Vec::new(), Vec::new());
    for &h in seams {
        let edge = body.get_half_edge(h).unwrap().edge;
        let ends = <[_; 2]>::from(body.edge_vertices(edge).unwrap())
            .map(|v| topo::readback::vertex_point(body, v).unwrap());
        for at in ends {
            let d = at - origin;
            azimuths.push(d.dot(e2).atan2(d.dot(e1)));
            heights.push(d.dot(e3));
        }
    }
    let lo = |v: &[f64]| v.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = |v: &[f64]| v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let (a0, a1) = (lo(&azimuths) - 0.1, hi(&azimuths) + 0.1);
    assert!(
        a1 - a0 < 2.0,
        "the fillet's turn is under a third of a turn"
    );
    let (h0, h1) = (lo(&heights) - 0.1, hi(&heights) + 0.1);
    let half = 0.5 * (a1 - a0);
    let at = |theta: f64, reach: f64, h: f64| {
        origin + (e1 * theta.cos() + e2 * theta.sin()) * reach + e3 * h
    };
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for (theta, reach, w) in [
        (a0, radius, 1.0),
        (a0 + half, radius / half.cos(), half.cos()),
        (a1, radius, 1.0),
    ] {
        for h in [h0, h1] {
            control.push(at(theta, reach, h));
            weights.push(w);
        }
    }
    let net = NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap();
    Surface::Nurbs(Arc::new(net))
}

/// What the mint derives for each tangent seam on the net of `radius`.
fn derived(radius: f64) -> Vec<Result<geom_brep::Pcurve<f64>, PcurveMintError>> {
    let (body, key, cylinder, seams) = filleted();
    let net = net_of(&body, &cylinder, &seams, radius);
    let mut moved = body.with_surface_moved_for_tests(key, net).unwrap();
    for &h in &seams {
        moved.detach_pcurve(h);
    }
    seams
        .iter()
        .map(|&h| topo::pcurve_of(&moved, h, band()))
        .collect()
}

#[test]
fn a_tangent_seam_on_a_spline_wall_ends_as_its_incidence_read() {
    let error = |got: &Result<geom_brep::Pcurve<f64>, PcurveMintError>| match got {
        Err(PcurveMintError::Certify { error, .. }) => Some(error.clone()),
        _ => None,
    };
    for got in derived(RADIUS) {
        assert!(
            matches!(error(&got), Some(PcurveCertifyError::IsoUnsupported { .. })),
            "a seam on the wall has no iso lane: {got:?}"
        );
    }
    for got in derived(RADIUS + 1e-3) {
        assert!(
            matches!(
                error(&got),
                Some(PcurveCertifyError::CarrierOffChart {
                    chart: geom::SurfaceKind::Nurbs,
                    carrier: geom::CurveKind::Line,
                    ..
                })
            ),
            "a seam a millimetre inside the wall ends off it: {got:?}"
        );
    }
    let b = band();
    for got in derived(RADIUS + 0.5 * (b.zero() + b.escalate())) {
        assert!(
            matches!(
                error(&got),
                Some(PcurveCertifyError::Escalated {
                    check: PcurveCheck::ChartIncidence,
                    ..
                })
            ),
            "a seam in band of the wall ends in the read's lever: {got:?}"
        );
    }
}
