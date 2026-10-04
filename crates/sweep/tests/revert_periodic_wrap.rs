//! **`Body::revert` inverts every joint element and moves no anchor.**
//! A pcurve row is an image — the edge's chart curve, a function of the
//! edge and the chart — and the element of the joint into its half-edge
//! (C4, `topo::joint`). Reversing a cycle runs each joint `p → he` as
//! `he → p`, so its element moves onto `p` as its inverse, and every
//! image stays where it is: no stored byte depends on which half-edge is
//! a loop's `first` (`topo::revert` module docs, the joint bullet).
//!
//! The loops that wind here are the ones with an AZIMUTH-FREE joint. At
//! a sphere's pole or a cone's apex the lever is zero, so that joint is
//! a reset and the loop's chain restarts its azimuth there; a lune whose
//! meridians meet at the pole carries one. A torus has no azimuth-free
//! joint (its lever `|R + r·cos v|` never vanishes for `R > r`, which the
//! revolve and tube doors require), and a drum's wall absorbs its
//! azimuth in a full rim, so those loops wind nothing and are the
//! controls: elements inverted all the same, images untouched, tier 3
//! reporting nothing but the complement.
//!
//! The fixtures are `common::latitude_seam`'s, `shell7_common`'s and
//! `revolve_common`'s, shared with the SHELL-9 suites and the
//! mass-properties rows so every row here measures a body those rows
//! measure.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use core::mem::{Discriminant, discriminant};

use geom_core::Band;
use sweep::Revolution;
use topo::joint::Winding;
use topo::{Body, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, PcurveMintError, ValidationError};

use super::common::approx::twisted_loft;
use super::common::latitude_seam::{collinear_cap_drum, door_cavity, two_arc_sphere};
use super::common::shell_operands::vessel;
use super::revolve_common::donut_profile;
use super::shell7_common::*;

/// A cone of radius 1 and height 2 through its apex: the wall's loop
/// runs the base rim and the two seam generators, which meet at the
/// apex — the cone's azimuth-free joint.
fn apex_cone() -> Body<f64> {
    polyline(&[(0.0, 0.0), (1.0, 0.0), (0.0, 2.0)], Revolution::Full)
}

/// A torus of radii `2, 0.5` authored as two cocircular arcs, the same
/// way `two_arc_sphere` is authored — `mass_props`'s donut: one torus
/// in four faces, no azimuth-free joint anywhere on it.
fn two_arc_torus() -> Body<f64> {
    revolved(donut_profile(), Revolution::Full)
}

/// Half a drum: a cylinder wall whose loop never wraps the chart.
fn half_drum() -> Body<f64> {
    wedge(1.0, 2.0, PI)
}

/// Every stored row of `body`, keyed by its half-edge, as `Debug`
/// text: image, interval and certificate, bit for bit.
/// NOT `common::pcurve_rows::rows`, which omits the certificate and
/// the key this suite compares by.
fn rows(body: &Body<f64>) -> Vec<(HalfEdgeKey, String)> {
    body.pcurves()
        .map(|(he, cache)| (he, format!("{cache:?}")))
        .collect()
}

/// Every cycle's anchor.
fn anchors(body: &Body<f64>) -> Vec<(LoopKey, HalfEdgeKey)> {
    body.loops()
        .filter_map(|(lk, lp)| match lp.boundary {
            LoopBoundary::Cycle { first } => Some((lk, first)),
            LoopBoundary::Empty { .. } => None,
        })
        .collect()
}

/// A loop's winding, from its stored elements in cycle order (`None`
/// where a joint stores none).
fn winding(body: &Body<f64>, lk: LoopKey) -> Option<Winding> {
    let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
        return None;
    };
    let elements: Option<Vec<_>> = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| body.joint(he))
        .collect();
    Some(Winding::of(elements?))
}

/// The loops whose elements wind them: a whole period, or a reset.
fn wrapping_loops(body: &Body<f64>) -> Vec<LoopKey> {
    anchors(body)
        .into_iter()
        .filter(|(lk, _)| winding(body, *lk).is_some_and(|w| !w.is_zero()))
        .map(|(lk, _)| lk)
        .collect()
}

/// What `chart_boundary` answers for every face of `body`, by result
/// KIND (`Ok`, or which refusal variant), in face order: a reversal
/// reverses every loop, and this reader pins that it does not change
/// whether a refusal fires.
fn boundary_kinds(body: &Body<f64>) -> Vec<(FaceKey, Result<(), Discriminant<PcurveMintError>>)> {
    let band = Band::linear(tol()).unwrap();
    body.faces()
        .map(|(fk, face)| {
            let chart = body.get_surface(face.surface).unwrap();
            let kind = topo::chart_boundary(body, fk, chart, band)
                .map(|_| ())
                .map_err(|e| discriminant(&e));
            (fk, kind)
        })
        .collect()
}

/// **The reversal on `body`**: every image is the source's bit for bit,
/// every loop keeps its anchor, the element on each half-edge is the
/// inverse of the one on its source successor, every loop winds as far
/// as it did, tier 3 of the reverted body reports exactly the
/// complement, `chart_boundary` answers the same kind for every face,
/// and the involution restores the bits.
fn assert_reverted(label: &str, body: &Body<f64>) {
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "{label}: the source is tier-3 valid"
    );
    let reverted = body.revert().expect("revert");
    assert_eq!(
        topo::validate_geometric(&reverted, tol()),
        Err(vec![ValidationError::NegativeVolume {
            solid: reverted.solids().next().expect("one solid").0
        }]),
        "{label}: a reverted body bounds the complement and nothing else fails"
    );
    assert_eq!(rows(&reverted), rows(body), "{label}: no image moves");
    assert_eq!(anchors(&reverted), anchors(body), "{label}: no anchor moves");
    for (he, data) in body.half_edges() {
        assert_eq!(
            reverted.joint(he),
            body.joint(data.next).map(topo::JointElement::inverse),
            "{label}: {he:?} carries its source successor's element, inverted"
        );
    }
    for (lk, _) in anchors(body) {
        let read = |w: Option<Winding>| w.map(|w| (w.closes(), w.is_zero()));
        assert_eq!(
            read(winding(&reverted, lk)),
            read(winding(body, lk)),
            "{label}: {lk:?} winds as far reversed"
        );
    }
    assert_eq!(
        boundary_kinds(&reverted),
        boundary_kinds(body),
        "{label}: chart_boundary answers the same kind for every face of the reversed body"
    );
    assert_eq!(
        format!("{:?}", reverted.revert().unwrap()),
        format!("{body:?}"),
        "{label}: bitwise involution"
    );
}

/// **The two-arc sphere and its door-built cavity.** Each sphere lune's
/// loop resets at the pole, where its meridians meet; reversed, it reads
/// the `NegativeVolume` alone, and its images are the cavity's bit for
/// bit.
#[test]
fn reverted_sphere_and_its_cavity_report_only_the_complement() {
    let sphere = two_arc_sphere();
    assert!(
        !wrapping_loops(&sphere).is_empty(),
        "the sphere's lunes reset at the pole"
    );
    assert_reverted("sphere", &sphere);
    assert_reverted("cavity", &door_cavity(&sphere, 0.05));
}

/// **A cone through its apex, the same way.** The apex is the cone's
/// azimuth-free joint, and the wall's loop resets there as a sphere
/// lune's does at the pole.
#[test]
fn reverted_apex_cone_reports_only_the_complement() {
    let cone = apex_cone();
    assert!(
        !wrapping_loops(&cone).is_empty(),
        "the cone wall's loop resets at the apex"
    );
    assert_reverted("cone", &cone);
}

/// **Controls: periodic charts whose loops close exactly.** Two tori
/// (the tube door's and a revolved two-arc profile's), the collinear-cap
/// drum and its cavity — whose rows are the plane mirror's, unchanged
/// by this — and a half drum, whose cylinder loop never wraps. No loop
/// winds, the reversal inverts their elements all the same, no image
/// moves, and tier 3 reports exactly the complement before and after.
#[test]
fn periodic_charts_whose_loops_wind_nothing_report_only_the_complement() {
    for (label, body) in [
        ("tube torus", tube_torus(2.0, 0.5)),
        ("two-arc torus", two_arc_torus()),
        ("drum", vessel(1.0, 2.0)),
        ("collinear-cap drum", collinear_cap_drum()),
        ("drum cavity", door_cavity(&collinear_cap_drum(), 0.05)),
        ("half drum", half_drum()),
    ] {
        assert!(
            wrapping_loops(&body).is_empty(),
            "{label}: no loop winds"
        );
        assert!(!rows(&body).is_empty(), "{label}: the fixture carries rows");
        assert_reverted(label, &body);
    }
}

/// **Control: a non-periodic curved chart.** The twisted loft's walls
/// are bilinear NURBS saddles, closed in nothing; their iso-line rows
/// are untouched, their elements inverted, and tier 3 of the reversed
/// body reports exactly the complement.
#[test]
fn a_non_periodic_curved_charts_rows_are_untouched() {
    let loft = twisted_loft(0.05);
    assert!(!rows(&loft).is_empty(), "the loft carries rows");
    assert!(wrapping_loops(&loft).is_empty());
    assert_reverted("twisted loft", &loft);
}
