//! **M9-2 PR-1 — the loft-wall (IsoLine/IsoArc) chart-region rows.**
//!
//! A described NURBS wall's boundary caches are the exact iso images
//! (`IsoLine` seams and line rims; `IsoArc` arc rims), so the
//! chart-region UV extraction reads them structurally — and then the
//! POSITIVE-AREA claim refuses typed at the arm gate: a NURBS chart
//! has no exact constant lever arms (`sup` stretch bounds over-state
//! in the unsafe direction; the `inf`-bounds extension is the named
//! follow-up). These rows pin exactly that seam: the refusal is
//! `ArmUnbounded` — extraction PASSED the inventory gate — never
//! `NonPlanarTrim`, never `MissingCache`, never a silent answer.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::Tol;
use geom_core::{Affine3, Band, Point2, Vec3};
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use topo::{Body, ChartRegionError, FaceKey, Pcurve, chart_region_overlap};

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

fn at_z(zs: &[f64]) -> Vec<Affine3<f64>> {
    zs.iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect()
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
            }
        })
        .collect()
}

#[test]
fn an_iso_line_wall_extracts_and_refuses_at_the_arm_gate() {
    let v = |x: f64, y: f64| ProfileVertex::new(Point2::new(x, y), 0.0);
    let square = || {
        vec![ProfileLoop::new(vec![
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
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.5, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
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
    // POSITIVE claim then refuses at the ARM gate, typed.
    match chart_region_overlap(&body, wall, &body, wall, band()) {
        Err(ChartRegionError::ArmUnbounded { chart }) => assert_eq!(chart, "NURBS"),
        other => panic!("a NURBS wall must refuse the area claim at the arm gate, got {other:?}"),
    }
}

#[test]
fn an_iso_arc_wall_extracts_and_refuses_at_the_arm_gate() {
    // A square with one bulged (arc) edge: the swept wall over the
    // arc is RATIONAL and its cap rims store `IsoArc` caches (M8-3).
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    let bulged = || {
        vec![ProfileLoop::new(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 0.4),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ])]
    };
    let sections = vec![bulged(), bulged()];
    let body = sweep::loft_body::<f64>(&sections, &at_z(&[0.0, 1.0]), 1, Tol::witness())
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
    match chart_region_overlap(&body, arc_wall, &body, arc_wall, band()) {
        Err(ChartRegionError::ArmUnbounded { chart }) => assert_eq!(chart, "NURBS"),
        other => panic!(
            "an IsoArc-rimmed wall must refuse the area claim at the arm gate, got {other:?}"
        ),
    }
}
