//! **M6-3 Leg A acceptance: the loft/sweep BODY assembly** — shape
//! (iii)'s loft builds, is watertight, and validates tier 3 at rest
//! (both M6-3 flips exercised: described-NURBS faces pass check 1,
//! Nurbs-adjacent edges are exempt from check 4 by kind), with every
//! wall boundary carrying a stored exact line-in-UV pcurve.
//!
//! The acceptance loft is a POLYLINE-profile loft (spec §3): three
//! quad sections, the middle one a trapezoid — NOT an affine image of
//! the squares (affine maps preserve parallelism; the trapezoid has a
//! non-parallel pair), so the walls are genuinely curved (v-degree 2)
//! and the R5 (iii) text's "≥3 sections, ≥1 non-affine pair" is met
//! without arcs (arc-bearing profiles skin to RATIONAL walls, whose
//! flux lane is banked — the honest partial, spec §3).

// Panicking is a test's failure mechanism (workspace lint policy).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Vec3};
use sweep::{SectionSegments, SketchSegment, loft_body};

/// The shape (iii) acceptance sections: squares at z = 0 and z = 2,
/// a trapezoid at z = 1.
fn shape_iii_sections() -> (Vec<SectionSegments>, Vec<Affine3<f64>>) {
    let quad = |pts: [(f64, f64); 4]| -> SectionSegments {
        let seg = |a: (f64, f64), b: (f64, f64)| SketchSegment::Line {
            a: geom_core::Point2::new(a.0, a.1),
            b: geom_core::Point2::new(b.0, b.1),
        };
        vec![vec![
            seg(pts[0], pts[1]),
            seg(pts[1], pts[2]),
            seg(pts[2], pts[3]),
            seg(pts[3], pts[0]),
        ]]
    };
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let trapezoid = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let sections = vec![quad(square), quad(trapezoid), quad(square)];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    (sections, places)
}

#[test]
fn shape_iii_loft_body_is_tier3_valid_at_rest() {
    let (sections, places) = shape_iii_sections();
    let lofted = loft_body::<f64>(&sections, &places, 2).expect("shape (iii) loft builds");
    let body = &lofted.body;
    assert_eq!(topo::validate(body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "tier 2 (watertight)");
    let report = topo::validate_geometric(body);
    assert_eq!(report, Ok(()), "tier 3 at rest");
}

#[test]
fn loft_walls_store_exact_iso_pcurves() {
    let (sections, places) = shape_iii_sections();
    let lofted = loft_body::<f64>(&sections, &places, 2).expect("loft builds");
    let body = &lofted.body;
    // Every wall-face half-edge carries a stored cache; caps (planar)
    // store nothing.
    let mut wall_hes = 0usize;
    let mut cached = 0usize;
    for (he, _) in body.half_edges() {
        if body.pcurve(he).is_some() {
            cached += 1;
        }
    }
    for faces in &lofted.side_faces {
        for &f in faces {
            let face = body.get_face(f).expect("face resolves");
            let cycle = match face_outer_cycle(body, face) {
                Some(c) => c,
                None => panic!("wall outer loop resolves"),
            };
            wall_hes += cycle.len();
            for he in cycle {
                let cache = body.pcurve(he).expect("wall half-edge stores a pcurve");
                assert!(
                    matches!(cache.pcurve(), geom_brep::Pcurve::IsoLine { .. }),
                    "wall pcurves are the exact iso-line lane"
                );
                let cert = cache.certificate();
                assert!(
                    matches!(
                        cert.statement,
                        geom_brep::EnvelopeStatement::MapResidualIsoHull
                    ),
                    "iso lane statement"
                );
                assert!(cert.max_residual < 1e-9, "sampled residual in metres");
                assert!(cert.envelope < 1e-9, "sup envelope in metres");
            }
        }
    }
    assert_eq!(cached, wall_hes, "exactly the wall half-edges store caches");
}

fn face_outer_cycle(body: &topo::Body<f64>, face: &topo::Face) -> Option<Vec<topo::HalfEdgeKey>> {
    let lp = body.get_loop(face.outer)?;
    let topo::LoopBoundary::Cycle { first } = lp.boundary else {
        return None;
    };
    body.loop_cycle(first)
}

/// **The derived closed-form volume** (the corpus `loft_prism`
/// module docs carry the same derivation): the degree-2 skin through
/// equal end sections has corner paths `P(v) = S + λ(v)·D` with
/// `λ = 4v(1−v)` and `z = 2v` exactly; each slice is a trapezoid of
/// area `4 + 2dλ` with `d = 0.375`, so
/// `V = 2·∫₀¹ (4 + 0.75·λ) dv = 8 + 0.75·(4/3)·? = 9 m³` exactly
/// (`∫₀¹ λ = 2/3`; `2·(4 + 0.75·2/3·…)` — spelled out: 2·4 = 8 plus
/// 2·(2d)·(2/3) = 8d/3·… with d = 0.375 gives exactly 1). The
/// certified enclosure must BRACKET it, and tightly.
#[test]
fn shape_iii_volume_matches_the_derived_closed_form() {
    let (sections, places) = shape_iii_sections();
    let lofted = loft_body::<f64>(&sections, &places, 2).expect("loft builds");
    let m = topo::props::mass_properties(&lofted.body).expect("mass properties");
    let exact = 9.0;
    assert!(
        (m.volume - exact).abs() <= m.volume_pad + 1e-9,
        "volume {} ± {} must bracket the derived {exact}",
        m.volume,
        m.volume_pad
    );
    assert!(
        m.volume_pad < 1e-6,
        "the exact per-span lane is tight, pad = {}",
        m.volume_pad
    );
    // The caps' closed forms contribute pad-free; the walls' area is
    // an honest enclosure (positive, finite).
    assert!(m.surface_area > 0.0 && m.area_pad.is_finite());
}
