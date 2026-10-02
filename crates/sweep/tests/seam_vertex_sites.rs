//! **Where `SeamVertex` fires with ONE co-surface seam, and where it
//! does not.** A full revolve sweeps a planar wall whole, so a latitude
//! rim between that disc and a curved wall split at its seam has one
//! seam meridian at each crossing, not two. The one-seam reading of
//! `battery::is_seam_vertex` admits that site — and only where the rim
//! CLOSES, because its recourse promises `rim_of` lists the rim whole.
//! One arc of an OPEN run of cocircular arcs swept beside a whole face
//! (a D's two quarter arcs, extruded) has the same orbit at its station
//! and must not be told to request a rim that does not exist.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, CornerConfig};
use sweep::test_support::{lantern, prism, rim_arcs_at};
use topo::EdgeKey;

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
    match fillet_edges(&body, &arcs[..1], 0.05, tol()).map_err(|r| r.error) {
        Err(BlendError::UnsupportedCorner {
            corner: CornerConfig::SeamVertex,
            ..
        }) => {}
        other => panic!("one arc of a closed rim refuses SeamVertex, got {other:?}"),
    }
}

/// A D: the straight side on `x = 0`, the round side two CCW quarter
/// arcs of the unit circle meeting at `(1, 0)`. The two arcs sweep onto
/// one cylinder key with a strut at the station, so the bottom rim's
/// station vertex carries one co-surface seam and two rim arcs on
/// (cap, cylinder) — the closed rim's orbit — but the rim is OPEN (it
/// ends at the straight side), so the tag must not fire there.
#[test]
fn one_quarter_arc_of_a_d_is_not_a_seam_vertex() {
    let b = (core::f64::consts::PI / 8.0).tan();
    let body = prism(
        vec![
            (Point2::new(0.0, 1.0), 0.0),
            (Point2::new(0.0, -1.0), b),
            (Point2::new(1.0, 0.0), b),
        ],
        1.0,
        tol(),
    );
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
    if let Err(r) = fillet_edges(&body, &quarter[..1], 0.05, tol()) {
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
