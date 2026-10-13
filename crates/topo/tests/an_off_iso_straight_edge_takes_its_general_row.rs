//! **A straight edge on a spline chart takes the image it has**: the
//! exact `IsoLine` where it lies on a boundary row or column, and its
//! `General` image where it runs off-iso.
//!
//! The fixture is the triangle lamina `(0, 0) → (1, 0) → (1, 1)` on the
//! bilinear identity chart `(u, v) ↦ (u, v, 0)`, built by `mvfs`, two
//! `mev_line`s and one `mef_chord`, every edge left as the Euler ops
//! described it (a `Line` carrier). The first edge is the chart's
//! boundary row `v = 0`, the second its boundary column `u = 1`, and
//! the chord the diagonal `u = v`. Every candidate the mint's iso
//! schedule builds for the diagonal passes through its start foot, a
//! chart corner, so only a pick that reads the carrier in its interior
//! tells them apart. Every coordinate is dyadic, so each scalar lifts
//! the fixture exactly.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Real;
use geom_core::spline::{KnotVector, Knots};
use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_brep::{Pcurve, PcurveCertifyError};
use geom_core::{Band, Point3, Tol};
use topo::{
    AtRestPolicy, Body, FaceKey, FaceSurface, HalfEdgeKey, MefSite, MevSite, PcurveMintError,
};

/// The lamina at `T`, lifted `lift` off the chart along its normal, with
/// its two faces and the `he_plus` of its row, column and diagonal.
fn lamina<T: AtRestPolicy>(lift: f64) -> (Body<T>, [FaceKey; 2], [HalfEdgeKey; 3]) {
    let tol = Tol::witness();
    let on =
        |(x, y, z): (f64, f64, f64)| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z));
    let pt = |(x, y): (f64, f64)| on((x, y, lift));
    let mut body = Body::<T>::new();
    let seed = body.mvfs(pt((0.0, 0.0)), true).unwrap();
    let unit = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let net = NurbsSurface::new(
        Knots::from(unit()).lift(Real::from_f64),
        Knots::from(unit()).lift(Real::from_f64),
        [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
            .into_iter()
            .map(|(x, y)| on((x, y, 0.0)))
            .collect(),
        vec![Real::one(); 4],
    )
    .unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Nurbs(Arc::new(net)),
            sense: true,
        },
    )
    .unwrap();
    let row = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt((1.0, 0.0)),
            tol,
        )
        .unwrap();
    let column = body
        .mev_line(
            MevSite::Fan {
                he1: row.he_minus,
                he2: row.he_minus,
            },
            pt((1.0, 1.0)),
            tol,
        )
        .unwrap();
    let back = body
        .find_half_edge(seed.face, column.vertex, row.vertex)
        .unwrap();
    let closed = body
        .mef_chord(
            MefSite::Chords {
                he1: back,
                he2: row.he_plus,
            },
            tol,
        )
        .unwrap();
    let plus = |e| body.get_edge(e).unwrap().he_plus;
    let halves = [plus(row.edge), plus(column.edge), plus(closed.edge)];
    (body, [seed.face, closed.face], halves)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// Every half-edge of the lamina: both faces' three.
const HALF_EDGES: usize = 6;

/// Which image the iso schedule derives for one half-edge.
#[derive(Debug, PartialEq)]
enum Kind {
    /// An `IsoLine` with `v` fixed at the given value.
    Row(f64),
    /// An `IsoLine` with `u` fixed at the given value.
    Column(f64),
    General,
}

fn kind<T: AtRestPolicy + geom_core::Bounds>(image: &Pcurve<T>) -> Kind {
    let mid = |x: T| x.lo();
    match image {
        Pcurve::IsoLine { p0, pl } if mid(pl.y) == 0.0 => Kind::Row(mid(p0.y)),
        Pcurve::IsoLine { p0, pl } if mid(pl.x) == 0.0 => Kind::Column(mid(p0.x)),
        Pcurve::General(_) => Kind::General,
        other => panic!("not an image the schedule derives here: {other:?}"),
    }
}

/// The derivation for the row, column and diagonal, and what the mint
/// then stores: the two exact iso lines and the diagonal's `General`
/// image, every row certified, and tier 3 re-certifies them all.
fn each_edge_takes_its_own_image<T: AtRestPolicy + geom_core::Bounds>() {
    let tol = Tol::witness();
    let (mut body, faces, [row, column, diagonal]) = lamina::<T>(0.0);
    let derived = |he| kind(&topo::pcurve_of(&body, he, band()).unwrap());
    assert_eq!(
        derived(row),
        Kind::Row(0.0),
        "the row is its boundary row, at {}",
        T::NAME
    );
    assert_eq!(
        derived(column),
        Kind::Column(1.0),
        "the column is its boundary column, at {}",
        T::NAME
    );
    assert_eq!(
        derived(diagonal),
        Kind::General,
        "the diagonal is no row: its image is General, at {}",
        T::NAME
    );
    assert_eq!(
        topo::mint_pcurves_of(&mut body, &faces, tol),
        Ok(HALF_EDGES),
        "every row certifies and is stored, at {}",
        T::NAME
    );
    for (he, want) in [
        (row, Kind::Row(0.0)),
        (column, Kind::Column(1.0)),
        (diagonal, Kind::General),
    ] {
        for half in [he, body.mate(he).unwrap()] {
            assert_eq!(
                kind(body.pcurve(half).unwrap().pcurve()),
                want,
                "{half:?}: the stored row, at {}",
                T::NAME
            );
        }
    }
    assert_eq!(
        topo::pcurves::validate_pcurves(&body, band()),
        Vec::new(),
        "tier 3 re-certifies every row, at {}",
        T::NAME
    );
}

#[test]
fn each_edge_takes_its_own_image_at_f64() {
    each_edge_takes_its_own_image::<f64>();
}

#[test]
fn each_edge_takes_its_own_image_at_interval() {
    each_edge_takes_its_own_image::<geom_core::Interval>();
}

/// **At a dual the `General` route has no door.** The two iso lines
/// still derive, since their pick is the probe alone; the diagonal's
/// derivation refuses `FittedLaneUnsupported` naming the scalar, which
/// the scalar is not owed, so the mint leaves both faces uncached.
#[test]
fn at_a_dual_the_diagonal_has_no_fitted_door_and_the_faces_stay_uncached() {
    type T = geom_core::Dual64;
    let (mut body, faces, [row, column, diagonal]) = lamina::<T>(0.0);
    let derived = |he| topo::pcurve_of(&body, he, band());
    assert_eq!(kind(&derived(row).unwrap()), Kind::Row(0.0));
    assert_eq!(kind(&derived(column).unwrap()), Kind::Column(1.0));
    let got = derived(diagonal);
    assert!(
        matches!(
            got,
            Err(PcurveMintError::Certify {
                error: PcurveCertifyError::FittedLaneUnsupported { scalar: "dual" },
                ..
            })
        ),
        "the diagonal's General route refuses naming the scalar: {got:?}"
    );
    assert_eq!(
        topo::mint_pcurves_of(&mut body, &faces, Tol::witness()),
        Ok(0),
        "the mint leaves the faces uncached"
    );
}

/// **An escalated candidate outranks the `General` image.** Lifted off
/// the chart by a distance inside the band (ε, K·ε), the row edge's
/// boundary-row candidate escalates at the probe, every other candidate
/// is definitely off, and the derivation answers the escalation. It does
/// not go on to derive the edge's `General` image, which would chart a
/// carrier the probe could not place.
#[test]
fn an_escalated_candidate_outranks_the_general_image() {
    let tol = Tol::witness();
    let lift = tol.eps() * tol.k().sqrt();
    let (body, _, [row, _, _]) = lamina::<f64>(lift);
    let got = topo::pcurve_of(&body, row, band());
    assert!(
        matches!(got, Err(PcurveMintError::Escalated { .. })),
        "a row in the band escalates rather than deriving its General image: {got:?}"
    );
}
