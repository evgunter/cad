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

mod common;

use common::quad;
use geom_core::Tol;
use geom_core::{Affine3, Vec3};
use profile::RawLoop;
use sweep::{Section, loft_body};

/// The shape (iii) acceptance sections: squares at z = 0 and z = 2,
/// a trapezoid at z = 1.
fn shape_iii_sections() -> (Vec<Section>, Vec<Affine3<f64>>) {
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
    let lofted =
        loft_body::<f64>(&sections, &places, 2, Tol::witness()).expect("shape (iii) loft builds");
    let body = &lofted.body;
    assert_eq!(topo::validate(body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "tier 2 (watertight)");
    let report = topo::validate_geometric(body, Tol::witness());
    assert_eq!(report, Ok(()), "tier 3 at rest");
}

#[test]
fn loft_walls_store_exact_iso_pcurves() {
    let (sections, places) = shape_iii_sections();
    let lofted = loft_body::<f64>(&sections, &places, 2, Tol::witness()).expect("loft builds");
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
    let lofted = loft_body::<f64>(&sections, &places, 2, Tol::witness()).expect("loft builds");
    let m = topo::props::mass_properties(&lofted.body, Tol::witness()).expect("mass properties");
    // R2 REVIEW PROBE (branch cert/6r2-probes only): pad record.
    eprintln!("R2M6 area_pad={:e} surface={:e}", m.area_pad, m.surface_area);
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
    // an honest enclosure — positive, and NARROW. Positivity and
    // containment are both monotone in the degrading direction, so a
    // ceiling is the only thing that reports a widening.
    //
    // This lane's pad is RESOLUTION-driven, not tolerance-driven — the
    // patch area rule is O(h) at a fixed cell count, and the enclosure
    // is built once, BEFORE the refinement rounds — so the measured
    // half-width of 0.1896 m² (a bracket 0.3792 m² wide, on a 25.32 m²
    // surface) is identical at every ε CI runs and on `Interval`. The
    // ceiling is 1.58x that half-width.
    //
    // Halving the cell count gives 0.3425 m², a factor of **1.81, not
    // 2**: the area rule intersects its padded midpoint with the cell
    // hull, and at the coarse resolution the hull is what binds on
    // some cells, so the O(h) doubling is an upper bound on how fast
    // this pad grows rather than an identity. The ceiling still reds
    // on that halving (0.3425 > 0.3), which is the property this row
    // is for; it no longer rests on an exact factor of two.
    //
    // The contrast is the point: this same body's certified VOLUME
    // half-width is 1.1e-13 m³. The flux lane refines to machine
    // precision; the area lane stays at whatever its fixed resolution
    // bought. Nothing in the kernel METERS it — by ruling, not by
    // omission (S-CERT Q1) — but `props/quad.rs`'s A2 gauge asserts a
    // generous outer ceiling on it. Widening every face's bracket by a
    // common factor reds THIS row at 1.58x and trips that assert at
    // 470x: the acceptance row is the tight meter, the assert is the
    // tripwire ~300x outside it, and neither substitutes for the other.
    //
    // Spelled as a bare absolute because this loft has no closed-form
    // area to scale to (its walls are degree-2 skins); the cut-cylinder
    // rows in `m5_pr11_quad_props.rs` scale to theirs. The tree spells
    // this kind of ceiling four ways and unifying them is not this
    // row's job.
    assert!(m.surface_area > 0.0);
    assert!(
        m.area_pad < 0.3,
        "the walls' area half-width is {} m² (surface area {} m²)",
        m.area_pad,
        m.surface_area
    );
}

/// **The cut-loft row (M6-3 §7, the PR 10 §5 contract verbatim):** a
/// boolean against the loft body refuses TYPED, naming its true
/// missing layer — the edge×NURBS-face sweep layer of M5 PR 9c item 5
/// — never a silent skip and never a wrong body.
#[test]
fn cut_loft_refuses_typed_naming_the_missing_boolean_layer() {
    let (sections, places) = shape_iii_sections();
    let loft = loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("loft builds")
        .body;
    let cutter = {
        use geom_core::Point2;
        use profile::{Profile, ProfileLoop, SketchPlane};
        let lp = ProfileLoop::polygon([
            Point2::new(0.0, -2.0),
            Point2::new(2.0, -2.0),
            Point2::new(2.0, 2.0),
            Point2::new(0.0, 2.0),
        ]);
        let vp = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(geom_core::Tol::witness())
            .unwrap();
        sweep::extrude(&vp, sweep::Extrusion::Distance(3.0), Tol::witness())
            .unwrap()
            .body
    };
    let out = topo::subtract(&loft, &cutter, Tol::witness());
    let err = match out {
        Err(e) => e,
        Ok(_) => panic!("the cut-loft boolean cannot succeed before the edge×NURBS-face layer"),
    };
    let text = err.to_string();
    println!("cut-loft refusal: {text}");
    assert!(
        text.contains("a NURBS face has no crossing layer"),
        "the refusal names the true missing layer (the M5 PR 9c item 5 lineage's \
         edge×NURBS-face crossing layer): {text}"
    );
}

/// **§9.3 interval rows:** the loft assembles at the INTERVAL scalar,
/// tier-3 green (the iso lane's hull bounds and the two flips at a
/// genuinely enclosure-valued scalar), and the certified volume
/// ENCLOSURE brackets the derived 9 m³ — asserted enclosure-style,
/// never by equality.
#[cfg(feature = "interval")]
mod certified {
    use geom_core::interval::Interval;

    use super::*;

    #[test]
    fn the_loft_body_certifies_and_encloses_nine_at_interval() {
        let (sections, places) = shape_iii_sections();
        let lofted = loft_body::<Interval>(&sections, &places, 2, Tol::witness())
            .expect("the loft builds at Interval");
        assert_eq!(
            topo::validate_geometric(&lofted.body, Tol::witness()),
            Ok(()),
            "tier 3"
        );
        let m =
            topo::props::mass_properties(&lofted.body, Tol::witness()).expect("mass properties");
        let (lo, hi) = (
            geom_core::Bounds::lo(m.volume) - geom_core::Bounds::hi(m.volume_pad),
            geom_core::Bounds::hi(m.volume) + geom_core::Bounds::hi(m.volume_pad),
        );
        assert!(lo <= 9.0 && 9.0 <= hi, "9 ∈ [{lo}, {hi}]");
        // The same two ceilings the f64 row carries, against the same
        // measured half-widths — this row folds the pads into the
        // bracket, so without them any widening of either makes it
        // easier, and it is the only row that reads this body at
        // `Interval`. Both pads come out bit-identical to the f64
        // lane's, so the constants are shared deliberately.
        let (vpad, apad) = (
            geom_core::Bounds::hi(m.volume_pad),
            geom_core::Bounds::hi(m.area_pad),
        );
        assert!(
            vpad < 1e-6,
            "the exact per-span lane is tight, pad = {vpad}"
        );
        assert!(
            apad < 0.3,
            "the walls' area half-width is {apad} m² at Interval"
        );
    }
}
