//! **M9-2 PR-1 — the loft-wall (IsoLine/IsoArc) chart-region rows.**
//!
//! A described NURBS wall's boundary caches are the exact iso images
//! (`IsoLine` seams and line rims; `IsoArc` arc rims), so the
//! chart-region UV extraction reads them structurally — and the
//! POSITIVE-AREA claim now CERTIFIES on them, metred by the chart's
//! certified LOWER stretch arms (`chart_stretch_inf`'s
//! derivative-net reading, skew-discounted). The `sup` bounds are
//! still the wrong side here and are still not what these rows read.
//!
//! These are the extension's acceptance rows and they carry their
//! digits. What they pin is that a real lofted wall — polynomial and
//! rational — earns a metre-honest positive answer rather than the
//! `ArmUnbounded` refusal it used to earn, and that the answer comes
//! out of the extraction path unchanged: never `NonPlanarTrim`,
//! never `MissingCache`, never a silent answer.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::Point3;
use geom_core::Tol;
use geom_core::{Band, Point2};
use profile::SketchPlane;
use profile::test_support::bulge_loop;
use sweep::test_support::stacked_at;
use topo::{Body, ChartOverlap, FaceKey, Pcurve, chart_region_overlap};

/// **A FIXED band, deliberately NOT `common::approx::band`.** These
/// rows decide at 1e-9/1e-8 whatever the run's ε is: the digits
/// above are the extension's acceptance figures, so the shared
/// home — which resolves the RUN's band — would change what they
/// decide against rather than just where the helper lives.
fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// The first described-NURBS wall face of a lofted body.
fn nurbs_wall(body: &Body<f64>) -> FaceKey {
    body.faces()
        .find(|(_, face)| {
            matches!(
                body.get_surface(face.surface),
                Some(Surface::Nurbs(payload)) if !payload.is_placeholder()
            )
        })
        .map(|(key, _)| key)
        .expect("a loft has described NURBS walls")
}

/// Every stored pcurve variant on the face's outer loop.
fn wall_pcurve_kinds(body: &Body<f64>, face: FaceKey) -> Vec<&'static str> {
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("wall outer loop is a cycle");
    };
    body.loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| {
            match body
                .pcurve(he)
                .expect("loft walls mint their iso caches")
                .pcurve()
            {
                Pcurve::IsoLine { .. } => "IsoLine",
                Pcurve::IsoArc { .. } => "IsoArc",
                Pcurve::Harmonic { .. } => "Harmonic",
                Pcurve::Fitted(_) => "Fitted",
                Pcurve::General(_) => "General",
                Pcurve::Spiric { .. } => "Spiric",
                Pcurve::FocalSection(_) => "FocalSection",
                Pcurve::Projected(_) => "Projected",
            }
        })
        .collect()
}

#[test]
fn an_iso_line_wall_extracts_and_certifies_positive_area() {
    let v = |x: f64, y: f64| (Point2::new(x, y), 0.0);
    let square = || {
        vec![bulge_loop(vec![
            v(-1.0, -1.0),
            v(1.0, -1.0),
            v(1.0, 1.0),
            v(-1.0, 1.0),
        ])]
    };
    // The offset middle place keeps the x = ±1 walls genuinely bowed
    // (a real described NURBS chart, not a promotable plane).
    let sections = vec![square(), square(), square()];
    let places = vec![
        SketchPlane::xy(),
        SketchPlane::xy_at(Point3::new(0.5, 0.0, 1.0)),
        SketchPlane::xy_at(Point3::new(0.0, 0.0, 2.0)),
    ];
    let body = sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the offset square prism builds")
        .body;
    let wall = nurbs_wall(&body);
    let kinds = wall_pcurve_kinds(&body, wall);
    assert!(
        kinds.iter().all(|k| *k == "IsoLine"),
        "a lofted line-profile wall's boundary is all IsoLine caches, got {kinds:?}"
    );
    // The self-pair is same-chart by construction (one SurfaceKey);
    // extraction reads the IsoLine endpoints structurally; the
    // POSITIVE claim is then metred by the chart's inf arms.
    //
    // The digits, on a `[0, 1]²` chart: `inf |S_u| = 2`,
    // `inf |S_v| = 2`, `sup |S_u| = 2`, `sup |S_v| = 2√2` (the wall is
    // BOWED, so the loft direction's stretch varies), and
    // `inf |S_u × S_v| = 4`. The normalized trace is `1 + 2 = 3` and
    // the normalized determinant `1`, so `ρ = √(2/(3 + √5)) ≈ 0.618`
    // and the arms are `≈ (1.236, 1.236)` m per chart unit. The unit
    // square scales to a 1.236 m square whose mean width `2A/P` is
    // ≈ 0.618 m — eight orders above this band's 1e-8, so the verdict
    // is definite rather than an escalation.
    match chart_region_overlap(&body, wall, &body, wall, band()) {
        Ok(ChartOverlap::PositiveArea) => {}
        other => panic!("a bowed NURBS wall must certify positive area, got {other:?}"),
    }
}

#[test]
fn a_rational_iso_arc_wall_extracts_and_certifies_positive_area() {
    // A square with one bulged (arc) edge: the swept wall over the
    // arc is RATIONAL and its cap rims store `IsoArc` caches (M8-3).
    let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
    let bulged = || {
        vec![bulge_loop(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 0.4),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ])]
    };
    let sections = vec![bulged(), bulged()];
    let body = sweep::loft_body::<f64>(&sections, &stacked_at(&[0.0, 1.0]), 1, Tol::witness())
        .expect("the bulged prism builds")
        .body;
    // Find the arc wall: the NURBS face whose rims are IsoArc.
    let arc_wall = body
        .faces()
        .filter(|(_, face)| {
            matches!(
                body.get_surface(face.surface),
                Some(Surface::Nurbs(payload)) if !payload.is_placeholder()
            )
        })
        .map(|(key, _)| key)
        .find(|&key| wall_pcurve_kinds(&body, key).contains(&"IsoArc"))
        .expect("the bulged edge sweeps to a wall with IsoArc rims");
    // A RATIONAL chart: the infs and sups are the quotient bound on
    // each cell's dyadic pieces (`NurbsSurface::stretch_floor`,
    // `area_floor`, `stretch_sup`). What this wall reads is
    // `inf |S_u| ≈ 1.8709`, `inf |S_v| ≈ 1.0000`, `sup ≈ (2.3659, 1.0000)`
    // and `inf |S_u × S_v| ≈ 1.8778`, giving `ρ ≈ 0.6885` and arms
    // `≈ (1.2881, 0.6885)`. The unit square scales to a 1.2881 × 0.6885 m
    // rectangle of mean width ≈ 0.4487 m: definite.
    match chart_region_overlap(&body, arc_wall, &body, arc_wall, band()) {
        Ok(ChartOverlap::PositiveArea) => {}
        other => panic!("a rational NURBS wall must certify positive area, got {other:?}"),
    }
}
