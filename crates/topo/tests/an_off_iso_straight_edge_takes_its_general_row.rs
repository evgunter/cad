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
use topo::construction::Construct;

use geom::{NurbsSurface, Surface};
use geom_brep::{Pcurve, PcurveCertifyError};
use geom_core::{Band, Point3, Tol};
use topo::construction::{FaceSurface, MefSite, MevSite};
use topo::{AtRestPolicy, Body, FaceKey, HalfEdgeKey, PcurveMintError};

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
    // Every value read here is exact at each scalar, so its lower
    // endpoint is the value.
    let lo = |x: T| x.lo();
    match image {
        Pcurve::IsoLine { p0, pl } if lo(pl.y) == 0.0 => Kind::Row(lo(p0.y)),
        Pcurve::IsoLine { p0, pl } if lo(pl.x) == 0.0 => Kind::Column(lo(p0.x)),
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

/// **An Intersection seam on an interior column takes its exact
/// column.** The bored block with two square holes, its top cap
/// re-charted onto the bilinear patch `(u, v) ↦ (3u, 2v, 2)`, every
/// edge described as the intersection of its two faces' surfaces. Each
/// hole's four rim edges are straight (`Line` carriers) and lie on the
/// chart's INTERIOR rows and columns, which no closed-form candidate
/// reaches: the carrier's end feet measure them. A rim along `y` is an
/// interior column, and derives the exact `IsoLine` at its `u` (where
/// it took `General` before the measured columns were offered to a
/// line); a rim along `x` is an interior row. Each image certifies.
#[test]
fn an_intersection_seam_on_an_interior_column_takes_its_exact_column() {
    let tol = Tol::witness();
    let centres = [0.75, 2.25];
    let mut body = topo::test_support::holed_block::<f64>(3.0, &centres, tol);
    topo::test_support::describe_as_intersections(&mut body, tol);
    let top = body
        .faces()
        .find(|(_, f)| {
            !f.rings.is_empty()
                && matches!(
                    body.get_surface(f.surface),
                    Some(Surface::Plane { normal, .. }) if normal.z > 0.5
                )
        })
        .map(|(k, _)| k)
        .expect("the top cap holds the holes");
    let unit = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let patch = NurbsSurface::new(
        unit(),
        unit(),
        vec![
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(0.0, 2.0, 2.0),
            Point3::new(3.0, 0.0, 2.0),
            Point3::new(3.0, 2.0, 2.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    // Lifts RechartStrandsDescriptions: the cap on a chart wider than
    // its holes is the row's premise.
    body.set_face_surface_unvouched_for_tests(
        top,
        FaceSurface::New {
            surface: Surface::Nurbs(Arc::new(patch)),
            sense: true,
        },
    )
    .unwrap();
    let wall = body
        .get_surface(body.get_face(top).unwrap().surface)
        .unwrap()
        .clone();
    let rings = body.get_face(top).unwrap().rings.clone();
    let rims: Vec<HalfEdgeKey> = body
        .half_edges()
        .filter(|(_, he)| rings.contains(&he.parent_loop))
        .map(|(k, _)| k)
        .collect();
    // A hole of side 1 about `x = c` spans `u ∈ [(c − ½)/3, (c + ½)/3]`.
    let columns: Vec<f64> = centres
        .iter()
        .flat_map(|c| [(c - 0.5) / 3.0, (c + 0.5) / 3.0])
        .collect();
    let (mut on_columns, mut on_rows) = (Vec::new(), 0);
    for h in rims {
        // The derivation, not a stored row: `pcurve_of` answers a
        // cached image first.
        let mut b = body.clone();
        b.detach_pcurve(h);
        let image = topo::pcurve_of(&b, h, band()).unwrap();
        match kind(&image) {
            Kind::Column(u) => on_columns.push(u),
            Kind::Row(v) => {
                assert!([0.25, 0.75].contains(&v), "{h:?}: a rim row at v = {v}");
                on_rows += 1;
            }
            Kind::General => panic!("{h:?}: a straight rim on an interior iso takes General"),
        }
        let curve = b
            .get_curve_geom(b.get_edge(b.get_half_edge(h).unwrap().edge).unwrap().curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap();
        let (t0, t1) = curve.params();
        geom_brep::PcurveCache::certify(image, t0, t1, curve.carrier(), &wall, band())
            .unwrap_or_else(|e| panic!("{h:?}: the exact image certifies: {e:?}"));
    }
    on_columns.sort_by(f64::total_cmp);
    assert_eq!(on_rows, 4, "each hole's two rims along x are rows");
    assert_eq!(
        on_columns.len(),
        4,
        "each hole's two rims along y are columns"
    );
    for (got, want) in on_columns.iter().zip(&columns) {
        assert!(
            (got - want).abs() <= 1e-12,
            "the column the feet measure: {got} against {want}"
        );
    }
}
