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
//! — at a dual too. What the mint then answers is the fitted lane's
//! four checks in their fixed order, and the order decides the
//! verdict, not only its text:
//!
//! - a LINE carrier fails check 1 (`UnsupportedCarrier`: a line on a
//!   spline chart has no fitted-grade class), which the mint reads as
//!   "no lane covers this pair yet" and answers by leaving the face
//!   uncached, `Ok`;
//! - a spline carrier passes check 1 and fails on its missing mate
//!   (`FittedMateMissing`), which propagates.
//!
//! Both hold at every scalar, a dual included: the fitted door's
//! absence at a dual is check 4's refusal, and a row that fails an
//! earlier check never reaches check 4. A mint that refused the
//! absent door BEFORE check 1 would turn the dual's `Ok` into an
//! error and its `FittedMateMissing` into `FittedLaneUnsupported`;
//! the `Dual64` rows below are the ones that go red on that.
//!
//! The fixture is a unit-square lamina (`mvfs`, three `mev_line`s, one
//! `mef_chord`) on a bilinear NURBS chart that is the identity map
//! `(u, v) ↦ (u, v, 0)`, every edge then re-described through
//! `Body::set_edge_curve` as a `General` image in that chart. Every
//! coordinate is dyadic, so the lift to each scalar is exact and every
//! ε row states the same thing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, PcurveCertifyError};
use geom_core::spline::KnotVector;
use geom_core::{Point2, Point3, Tol};
use topo::{AtRestPolicy, Body, FaceKey, FaceSurface, MefSite, MevSite, PcurveMintError};

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
        unit(),
        unit(),
        [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
            .into_iter()
            .map(pt)
            .collect(),
        vec![1.0; 4],
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
            unit(),
            vec![Point2::new(p.x, p.y), Point2::new(q.x, q.y)],
            vec![1.0; 2],
        )
        .unwrap();
        let line = EdgeCurveSpec::line_between(p, q);
        let (carrier, param_start, param_end) = match carrier {
            Carrier::Line => (line.carrier, line.param_start, line.param_end),
            Carrier::Spline => (
                Curve3::Nurbs(Arc::new(
                    NurbsCurve3::new(unit(), vec![p, q], vec![1.0; 2]).unwrap(),
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

/// A LINE carrier fails the fitted lane's check 1, and the mint leaves
/// the face uncached — through both entry points, and with no row
/// stored on any half-edge.
fn a_line_carrier_leaves_the_face_uncached<T: AtRestPolicy>() {
    let tol = Tol::witness();
    let (mut body, faces) = lamina::<T>(Carrier::Line);
    assert_eq!(
        topo::mint_pcurves_of(&mut body, &faces, tol),
        Ok(0),
        "a stated General image over a line carrier is outside every route: the \
         faces are left uncached, at {}",
        T::NAME
    );
    assert_eq!(
        topo::mint_pcurves(&mut body, tol),
        Ok(()),
        "the whole-body mint answers as the per-face one, at {}",
        T::NAME
    );
    let rows = body
        .half_edges()
        .filter(|(he, _)| body.pcurve(*he).is_some())
        .count();
    assert_eq!(rows, 0, "no half-edge carries a row, at {}", T::NAME);
    // The face the mint legally left uncached has no certificate for
    // its General images, so its description is not yet available —
    // which is what `chart_boundary` says, rather than a defect.
    let band = geom_core::Band::linear(tol).unwrap();
    let chart = body
        .get_surface(body.get_face(faces[0]).unwrap().surface)
        .unwrap()
        .clone();
    let got = topo::pcurves::chart_boundary(&body, faces[0], &chart, band);
    assert!(
        matches!(got, Err(PcurveMintError::UncertifiedImage { .. })),
        "an uncached face's General images have no certificate, at {}: {:?}",
        T::NAME,
        got.err()
    );
}

/// A spline carrier passes check 1 and refuses on its missing mate —
/// the stated image names one chart, so there is no operand pair to
/// state a tube about — through both entry points.
fn a_spline_carrier_without_a_mate_refuses_on_the_pair<T: AtRestPolicy>() {
    let tol = Tol::witness();
    let (mut body, faces) = lamina::<T>(Carrier::Spline);
    let missing = |got: Result<(), PcurveMintError>, door: &str| {
        assert!(
            matches!(
                got,
                Err(PcurveMintError::Certify {
                    error: PcurveCertifyError::FittedMateMissing,
                    ..
                })
            ),
            "{door}: a stated General image over a spline carrier with no mate \
             refuses on the pair, at {}: {got:?}",
            T::NAME
        );
    };
    missing(
        topo::mint_pcurves_of(&mut body, &faces, tol).map(|_| ()),
        "mint_pcurves_of",
    );
    missing(topo::mint_pcurves(&mut body, tol), "mint_pcurves");
}

/// One row per carrier at one scalar.
macro_rules! rows_at {
    ($(#[$attr:meta])* $module:ident, $scalar:ty) => {
        $(#[$attr])*
        mod $module {
            #[test]
            fn a_line_carrier_leaves_the_face_uncached() {
                super::a_line_carrier_leaves_the_face_uncached::<$scalar>();
            }

            #[test]
            fn a_spline_carrier_without_a_mate_refuses_on_the_pair() {
                super::a_spline_carrier_without_a_mate_refuses_on_the_pair::<$scalar>();
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
// The scalar with no fitted door: its verdicts are the certifying
// scalars' verdicts, because both rows fail a check that runs before
// the door is asked for.
rows_at!(at_dual64, geom_core::Dual64);

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
