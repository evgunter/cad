//! **The semicircle prism and its arc wall**, shared by the rows that
//! re-chart that wall and ask what its rows derive: the `f64` loft of a
//! profile with one semicircular edge (bulge 1), its carriers held as
//! points at any lane scalar. Its arc wall
//! is a rational-quadratic spline chart whose two seams are described as
//! `IsoLine` images on the neighbouring walls and whose two rims are its
//! `IsoArc` rows; its knots mirror bitwise, so `reversed_u` and
//! `reversed_v` both accept it.

use geom::{NurbsSurface, Surface};
use geom_core::{Band, Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::stacked_at;
use topo::construction::Construct;
use topo::construction::FaceSurface;
use topo::{AtRestPolicy, Body, FaceKey, HalfEdgeKey, LeafBody as _, Pcurve, PcurveMintError};

pub fn tol() -> Tol {
    Tol::witness()
}

pub fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// The band's interior: a distance the probe can neither confirm nor
/// rule out (between ε and K·ε).
pub fn in_band() -> f64 {
    tol().eps() * tol().k().sqrt()
}

/// A prism with a semicircle wall at `T`, its wall rows minted: the
/// `f64` prism's carriers held at `T` as points
/// ([`sweep::test_support::f64_body_held_as_points`]), so the rows here
/// have a rational spline wall to read at every scalar. At `f64` that is
/// the loft's own body.
pub fn semicircle_prism<T: AtRestPolicy>() -> Body<T> {
    let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
    let profile = || {
        vec![bulge_loop(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 1.0),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ])]
    };
    sweep::test_support::f64_body_held_as_points::<T>(
        &[profile(), profile()],
        &stacked_at(&[0.0, 1.0]),
        1,
        tol(),
    )
    .expect("the prism builds")
    .body
    .into_body()
}

/// The arc wall: the face whose rows include the `IsoArc` rims.
pub fn arc_wall<T: AtRestPolicy>(body: &Body<T>) -> FaceKey {
    let rim = body
        .pcurves()
        .find(|(_, c)| matches!(c.pcurve(), Pcurve::IsoArc { .. }))
        .map(|(h, _)| h)
        .expect("the prism has an arc rim");
    body.face_of_half_edge(rim).unwrap()
}

/// The wall's half-edges.
pub fn wall_half_edges<T: AtRestPolicy>(body: &Body<T>, wall: FaceKey) -> Vec<HalfEdgeKey> {
    body.half_edges()
        .map(|(k, _)| k)
        .filter(|&k| body.face_of_half_edge(k) == Some(wall))
        .collect()
}

/// `body` with `wall` re-charted in place onto `remap(its surface)`, its
/// sense flipped where `flip_sense`.
pub fn recharted<T: AtRestPolicy>(
    body: &Body<T>,
    wall: FaceKey,
    remap: impl Fn(&NurbsSurface<T>) -> NurbsSurface<T>,
    flip_sense: bool,
) -> Body<T> {
    let mut out = body.clone();
    let face = out.get_face(wall).unwrap();
    let (key, sense) = (face.surface, face.sense);
    let Some(Surface::Nurbs(net)) = out.get_surface(key).cloned() else {
        panic!("the wall is a spline chart")
    };
    // Lifts RechartUnvouched and RechartStrandsDescriptions: the wall
    // re-charted in place onto the same surface reparameterized or
    // moved, its edges' descriptions left as they were, is the row's
    // premise.
    out.set_face_surface_unvouched_for_tests(
        wall,
        FaceSurface::New {
            surface: Surface::Nurbs(std::sync::Arc::new(remap(&net))),
            sense: sense != flip_sense,
        },
    )
    .unwrap();
    out
}

/// What `nurbs_iso_derive` answers for `h`, the stored row detached:
/// `pcurve_of` answers a cached image first.
pub fn derived<T: AtRestPolicy>(
    body: &Body<T>,
    h: HalfEdgeKey,
) -> Result<Pcurve<T>, PcurveMintError> {
    let mut fresh = body.clone();
    fresh.detach_pcurve(h);
    topo::pcurve_of(&fresh, h, band())
}
