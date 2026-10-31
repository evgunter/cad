//! **A `General` chart image the construction STATES, minted at every
//! scalar** — the mint's own-chart arm, reached through the public
//! kernel API alone.
//!
//! An edge described as an image in its face's own chart
//! (`EdgeDescriptionSpec::chart_image`) carries that image to the
//! mint verbatim: `nurbs_iso_derive`'s own-chart arm hands it to
//! `mint_face` without deriving anything, so a stated
//! `Pcurve::General` image reaches the fitted-grade certification
//! (`PcurveCache::certify_general`) with no derivation in front of it
//! — at a dual too. The fitted-grade row reads its carrier and chart
//! alone, so the carrier's kind decides nothing: a LINE carrier and a
//! spline one each certify against the identity chart and mint their
//! rows at every scalar that holds the fitted door. At a dual the door
//! is absent, so check 4 refuses `FittedLaneUnsupported` naming the
//! scalar, and the mint leaves the faces uncached, since that scalar
//! is owed no fitted row.
//!
//! The fixture is a unit-square lamina (`mvfs`, three `mev_line`s, one
//! `mef_chord`) on a bilinear NURBS chart that is the identity map
//! `(u, v) ↦ (u, v, 0)`, every edge then re-described through
//! `Body::set_edge_curve` as a `General` image in that chart. Every
//! coordinate is dyadic, so the lift to each scalar is exact and every
//! ε row states the same thing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Real;
use geom_core::spline::Knots;
use std::sync::Arc;
use topo::construction::Construct;

use geom::{Curve3, NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, PcurveCertifyError};
use geom_core::spline::KnotVector;
use geom_core::{Point2, Point3, Tol};
use topo::construction::{FaceSurface, MefSite, MevSite};
use topo::{AtRestPolicy, Body, FaceKey};

/// Which carrier every edge of the lamina is re-described with.
#[derive(Clone, Copy, Debug)]
enum Carrier {
    /// The edge's own chord line, as `mev_line` minted it.
    Line,
    /// The same chord as a degree-1 spline on `[0, 1]`.
    Spline,
}

/// The unit square, counter-clockwise from the origin, in chart
/// coordinates (the chart is the identity onto `z = 0`).
const SQUARE: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

/// Builds the lamina at `T`, re-describes every edge as a stated
/// `General` image with `carrier`, and returns the body with its two
/// faces.
fn lamina<T: AtRestPolicy>(carrier: Carrier) -> (Body<T>, [FaceKey; 2]) {
    let tol = Tol::witness();
    let pt = |(x, y): (f64, f64)| Point3::new(T::from_f64(x), T::from_f64(y), T::zero());
    let mut body = Body::<T>::new();
    let seed = body.mvfs(pt(SQUARE[0]), true).unwrap();
    // The identity chart: row-major over `u` then `v`.
    let unit = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let net = NurbsSurface::new(
        Knots::from(unit()).lift(geom_core::Real::from_f64),
        Knots::from(unit()).lift(geom_core::Real::from_f64),
        [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
            .into_iter()
            .map(pt)
            .collect(),
        vec![Real::one(); 4],
    )
    .unwrap();
    let chart = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Nurbs(Arc::new(net)),
                sense: true,
            },
        )
        .unwrap();
    let ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(SQUARE[1]),
            tol,
        )
        .unwrap();
    let bc = body
        .mev_line(
            MevSite::Fan {
                he1: ab.he_minus,
                he2: ab.he_minus,
            },
            pt(SQUARE[2]),
            tol,
        )
        .unwrap();
    let cd = body
        .mev_line(
            MevSite::Fan {
                he1: bc.he_minus,
                he2: bc.he_minus,
            },
            pt(SQUARE[3]),
            tol,
        )
        .unwrap();
    let he_dc = body
        .find_half_edge(seed.face, cd.vertex, bc.vertex)
        .unwrap();
    let closed = body
        .mef_chord(
            MefSite::Chords {
                he1: he_dc,
                he2: ab.he_plus,
            },
            tol,
        )
        .unwrap();

    let edges: Vec<_> = body
        .edges()
        .map(|(key, edge)| (key, edge.he_plus))
        .collect();
    for (edge, he_plus) in edges {
        let start = body.get_half_edge(he_plus).unwrap().start;
        let end = body
            .get_half_edge(body.get_half_edge(he_plus).unwrap().next)
            .unwrap()
            .start;
        let at = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let (p, q) = (at(start), at(end));
        let image = NurbsCurve2::new(
            Knots::from(unit()).lift(geom_core::Real::from_f64),
            vec![Point2::new(p.x, p.y), Point2::new(q.x, q.y)],
            vec![Real::one(); 2],
        )
        .unwrap();
        let line = EdgeCurveSpec::line_between(p, q);
        let (carrier, param_start, param_end) = match carrier {
            Carrier::Line => (line.carrier, line.param_start, line.param_end),
            Carrier::Spline => (
                Curve3::Nurbs(Arc::new(
                    NurbsCurve3::new(
                        Knots::from(unit()).lift(geom_core::Real::from_f64),
                        vec![p, q],
                        vec![Real::one(); 2],
                    )
                    .unwrap(),
                )),
                T::zero(),
                T::one(),
            ),
        };
        body.set_edge_curve(
            edge,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::chart_image(
                    chart,
                    Pcurve::General(Arc::new(image)),
                ),
                carrier,
                param_start,
                param_end,
            },
            tol,
        )
        .unwrap();
    }
    (body, [seed.face, closed.face])
}

/// Every half-edge of the lamina: both faces' four.
const HALF_EDGES: usize = 8;

/// At a scalar holding the fitted door, the stated `General` images
/// certify over either carrier and the mint stores every row, through
/// both entry points; tier 3 then finds nothing.
fn certifies_and_mints_its_rows<T: AtRestPolicy>(carrier: Carrier) {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let (mut body, faces) = lamina::<T>(carrier);
    assert_eq!(
        topo::mint_pcurves_of(&mut body, &faces, tol),
        Ok(HALF_EDGES),
        "{carrier:?}: the stated images certify and mint, at {}",
        T::NAME
    );
    let (mut whole, _) = lamina::<T>(carrier);
    assert_eq!(
        topo::mint_pcurves(&mut whole, tol),
        Ok(()),
        "{carrier:?}: the whole-body mint answers as the per-face one, at {}",
        T::NAME
    );
    for b in [&body, &whole] {
        let rows: Vec<_> = b.half_edges().filter_map(|(he, _)| b.pcurve(he)).collect();
        assert_eq!(
            rows.len(),
            HALF_EDGES,
            "{carrier:?}: every half-edge, at {}",
            T::NAME
        );
        assert!(
            rows.iter()
                .all(|r| matches!(r.pcurve(), Pcurve::General(_))),
            "{carrier:?}: each row is the stated General image, at {}",
            T::NAME
        );
        assert_eq!(
            topo::pcurves::validate_pcurves(b, band),
            Vec::new(),
            "{carrier:?}: tier 3 re-certifies every row, at {}",
            T::NAME
        );
    }
}

/// At a dual the door is absent: check 4 refuses naming the scalar,
/// and the mint leaves the faces uncached.
fn refuses_at_check_4_without_the_door<T: AtRestPolicy>(carrier: Carrier) {
    assert!(
        T::fitted_lane().is_none(),
        "{} holds no fitted door",
        T::NAME
    );
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let (mut body, faces) = lamina::<T>(carrier);
    let edge = body.edges().next().map(|(_, e)| e.curve).unwrap();
    let curve = body
        .get_curve_geom(edge)
        .and_then(|c| c.certified())
        .unwrap();
    let geom_brep::EdgeDescription::Chart(chart_curve) = curve.description() else {
        panic!("the lamina's edges are chart images");
    };
    let Pcurve::General(image) = &chart_curve.pcurve else {
        panic!("the lamina's images are General");
    };
    let surface = body.get_surface(chart_curve.surface).unwrap().clone();
    let (t0, t1) = curve.params();
    let got = geom_brep::PcurveCache::certify_general(
        Arc::clone(image),
        t0,
        t1,
        curve.carrier(),
        &surface,
        band,
        T::fitted_lane(),
    );
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::FittedLaneUnsupported { scalar }) if scalar == T::NAME
        ),
        "{carrier:?}: check 4 refuses naming the scalar, at {}: {got:?}",
        T::NAME
    );
    assert_eq!(
        topo::mint_pcurves_of(&mut body, &faces, tol),
        Ok(0),
        "{carrier:?}: the mint leaves the faces uncached, at {}",
        T::NAME
    );
}

/// One row per carrier at one scalar holding the door.
macro_rules! rows_at {
    ($(#[$attr:meta])* $module:ident, $scalar:ty) => {
        $(#[$attr])*
        mod $module {
            #[test]
            fn a_line_carrier_certifies_and_mints_its_rows() {
                super::certifies_and_mints_its_rows::<$scalar>(super::Carrier::Line);
            }

            #[test]
            fn a_spline_carrier_certifies_and_mints_its_rows() {
                super::certifies_and_mints_its_rows::<$scalar>(super::Carrier::Spline);
            }
        }
    };
}

rows_at!(at_f64, f64);
rows_at!(at_interval, geom_core::Interval);
rows_at!(at_sym_f64, geom_core::Sym<f64>);
rows_at!(
    #[cfg(feature = "probe")]
    at_probe,
    geom_core::Probe
);

/// The scalar with no fitted door.
mod at_dual64 {
    #[test]
    fn a_line_carrier_refuses_at_check_4() {
        super::refuses_at_check_4_without_the_door::<geom_core::Dual64>(super::Carrier::Line);
    }

    #[test]
    fn a_spline_carrier_refuses_at_check_4() {
        super::refuses_at_check_4_without_the_door::<geom_core::Dual64>(super::Carrier::Spline);
    }
}

/// **The fitted refusal's replay list is the policy's answer.**
/// `geom_brep::FITTED_DOOR_HOLDERS` is what the refusal renders, and
/// geom-brep cannot read the policy, so its membership is pinned here:
/// exactly the scalars whose `fitted_lane()` answers `Some`, the probe
/// among them in a build that has it.
#[test]
fn the_fitted_replay_list_is_the_policys_holders() {
    fn holds<T: AtRestPolicy>(held: &mut Vec<&'static str>) {
        if T::fitted_lane().is_some() {
            held.push(T::NAME);
        }
    }
    let mut held = Vec::new();
    holds::<f64>(&mut held);
    holds::<geom_core::Interval>(&mut held);
    holds::<geom_core::Sym<f64>>(&mut held);
    holds::<geom_core::Sym<geom_core::Interval>>(&mut held);
    holds::<geom_core::Dual64>(&mut held);
    #[cfg(feature = "probe")]
    holds::<geom_core::Probe>(&mut held);
    held.sort_unstable();
    held.dedup();
    let mut listed = geom_brep::FITTED_DOOR_HOLDERS.to_vec();
    listed.sort_unstable();
    assert_eq!(
        listed, held,
        "the fitted refusal's replay list must name exactly the scalars whose policy holds \
         the door"
    );
}
