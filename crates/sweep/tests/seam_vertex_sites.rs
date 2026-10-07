//! **Where `SeamVertex` fires with ONE co-surface seam, and where it
//! does not.** A full revolve sweeps a planar wall whole, so a latitude
//! rim between that disc and a curved wall split at its seam has one
//! seam meridian at each crossing, not two. The one-seam reading of
//! `battery::is_seam_vertex` admits that site — and only where the rim
//! CLOSES, because its recourse promises `rim_of` lists the rim whole.
//! One arc of an OPEN run of cocircular arcs beside a whole face (a
//! D's round side, extruded, cut at its station with a ruling across
//! its one cylinder wall) has the same orbit at its station and must
//! not be told to request a rim that does not exist.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, CornerConfig};
use sweep::test_support::{lantern, prism, rim_arcs_at};
use topo::EdgeKey;

use crate::common::stations::station_vertices;

fn tol() -> Tol {
    Tol::witness()
}

/// The lantern's base rim (plane disc × unit sphere at radius 1,
/// station 0): the disc is whole, the sphere zone two half-bands, so
/// each crossing has ONE seam. One arc of it refuses `SeamVertex`, and
/// the rim the recourse names is there — two arcs, closing.
#[test]
fn one_arc_of_a_whole_disc_rim_refuses_seam_vertex() {
    let body = lantern(tol());
    let arcs = rim_arcs_at(&body, 1.0, 0.0);
    assert_eq!(
        arcs.len(),
        2,
        "the disc's rim is two arcs, split by the zone's seam"
    );
    match fillet_edges(
        &sweep::test_support::at_rest(&body),
        &arcs[..1],
        0.05,
        tol(),
    )
    .map_err(|r| r.error)
    {
        Err(BlendError::UnsupportedCorner {
            corner: CornerConfig::SeamVertex,
            ..
        }) => {}
        other => panic!("one arc of a closed rim refuses SeamVertex, got {other:?}"),
    }
}

/// A D: the straight side on `x = 0`, the round side two CCW quarter
/// arcs of the unit circle meeting at `(1, 0)`. The sweep builds the
/// round side as one cylinder wall with one rim arc on each cap; the
/// station at `(1, 0)` is cut into both rim arcs and joined by a
/// ruling `mef` across the wall, so the bottom rim's station vertex
/// carries one co-surface seam and two rim arcs on (cap, cylinder) —
/// the closed rim's orbit — but the rim is OPEN (it ends at the
/// straight side), so the tag must not fire there.
#[test]
fn one_quarter_arc_of_a_d_is_not_a_seam_vertex() {
    let b = (core::f64::consts::PI / 8.0).tan();
    let mut body = prism(
        vec![
            (Point2::new(0.0, 1.0), 0.0),
            (Point2::new(0.0, -1.0), b),
            (Point2::new(1.0, 0.0), b),
        ],
        1.0,
        tol(),
    );
    let rims: Vec<EdgeKey> = body
        .edges()
        .filter(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Circle { .. }))
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(rims.len(), 2, "one rim arc on each cap");
    let e = body.get_edge(rims[0]).unwrap();
    let wall = [e.he_plus, e.he_minus]
        .into_iter()
        .filter_map(|h| body.face_of_half_edge(h))
        .find(|&f| {
            matches!(
                body.get_surface(body.get_face(f).unwrap().surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .expect("the round side's cylinder wall");
    let stations: Vec<topo::VertexKey> = rims
        .iter()
        .map(|&rim| {
            let curve = body.get_edge(rim).unwrap().curve;
            let (t0, t1) = body
                .get_curve_geom(curve)
                .unwrap()
                .certified()
                .unwrap()
                .params();
            body.split_edge(rim, (t0 + t1) * 0.5, tol()).unwrap().vertex
        })
        .collect();
    assert_eq!(
        station_vertices(&body),
        stations,
        "a station cut into an arc rim is one the reader finds"
    );
    let leaving = |v: topo::VertexKey| {
        body.half_edges()
            .find(|(h, he)| he.start == v && body.face_of_half_edge(*h) == Some(wall))
            .unwrap()
            .0
    };
    let (he1, he2) = (leaving(stations[0]), leaving(stations[1]));
    // The ruling rests between two faces of the wall, so it is described
    // in the wall's chart rather than as a scaffold.
    let at = |v: topo::VertexKey| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let ruling = geom_brep::EdgeCurveSpec::line_between(at(stations[0]), at(stations[1]))
        .at_rest_in_chart(body.get_face(wall).unwrap().surface, false);
    body.mef(
        topo::MefSite::Chords { he1, he2 },
        ruling,
        topo::FaceSurface::Inherit,
        tol(),
    )
    .unwrap();
    let quarter: Vec<EdgeKey> = body
        .edges()
        .filter(|(_, e)| {
            let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
                return false;
            };
            matches!(c.carrier(), geom::Curve3::Circle { center, .. } if center.z.abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(quarter.len(), 2, "the bottom rim's two quarter arcs");
    assert!(
        topo::query::rim_of(&body, quarter[0]).is_err(),
        "the D's round side is not a closed rim"
    );
    if let Err(r) = fillet_edges(
        &sweep::test_support::at_rest(&body),
        &quarter[..1],
        0.05,
        tol(),
    ) {
        assert!(
            !matches!(
                r.error,
                BlendError::UnsupportedCorner {
                    corner: CornerConfig::SeamVertex,
                    ..
                }
            ),
            "an open run's station is not a chart seam: {:?}",
            r.error
        );
    }
}
