//! **M7-3 acceptance: NURBS-face import** (`docs/M7-3-SPEC.md` §2).
//!
//! The committed non-rational loft (`loft_prism`) runs the FULL row-1
//! and fixed-point obligations through `roundtrip.rs` (it joined
//! `SOLID_FIXTURES` at this unit); this suite carries the rows that
//! are NURBS-specific:
//!
//! - the **surface_sig pin** at body level (spec item 2's trap): the
//!   four distinct walls must land on four distinct surface keys;
//! - the **rational arm** (spec item 5, Arm B —
//!   import-with-typed-limitation): a natively built `arc_loft`
//!   (three walls non-rational, one RATIONAL) exports, imports, and
//!   lands in EXACTLY the native body's state — census equal, tiers
//!   1/2 green, and the tier-3 volume refusal *identical in kind and
//!   text* to the one the native body produces;
//! - the **description state**: imported seams carry `IsoCurve`,
//!   imported cap rims `MappedCurve` — the native loft's own
//!   description classes, which is what makes one adoption pass a
//!   fixed point of the writer.
//!
//! # Coverage honesty (spec §3)
//!
//! The exportable NURBS class today is bounded by the loft/sweep
//! skin's weight drift (#207): `sweep_body` with any curved path and
//! `loft_body` with any non-uniform section spacing refuse at
//! assembly (`nurbs_span_meter` poison), so the bodies this suite can
//! round-trip are uniformly-spaced lofts — polyline profiles
//! (non-rational, full tier 3) and arc-bearing profiles (rational
//! walls, the Arm-B typed tier-3 limitation). That is a statement
//! about the builder, not the reader: the import side accepts any
//! file in the written vocabulary.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::import_body;
use geom_core::{Affine3, Point2, Vec3};
use step_import::{ImportOptions, StepImport, import_step};
use sweep::{SectionSegments, SketchSegment, loft_body};

/// The arc-bearing profile loft the substrate measured (one bulged
/// side per section → 3 non-rational walls + 1 RATIONAL wall): the
/// writer exports it today with tiers 1/2 valid and a typed tier-3
/// volume refusal, which is exactly the state its import must land
/// in.
fn native_arc_loft() -> topo::Body<f64> {
    let arc_section = |s: f64| -> SectionSegments {
        let seg = |a: (f64, f64), b: (f64, f64)| SketchSegment::Line {
            a: Point2::new(a.0, a.1),
            b: Point2::new(b.0, b.1),
        };
        vec![vec![
            seg((-s, -s), (s, -s)),
            SketchSegment::Arc {
                a: Point2::new(s, -s),
                b: Point2::new(s, s),
                // tan(π/8): a quarter-circle bulge-out.
                bulge: 0.4142135623730951,
            },
            seg((s, s), (-s, s)),
            seg((-s, s), (-s, -s)),
        ]]
    };
    let sections = vec![arc_section(1.0), arc_section(1.25), arc_section(1.0)];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    loft_body::<f64>(&sections, &places, 2)
        .expect("the uniformly-spaced arc loft builds natively")
        .body
}

/// The one `QuadratureUnsupported::what` text of a body's tier-3
/// refusal — panics if tier 3 does not refuse in exactly that shape
/// (one `VolumeUncomputable` finding on one face).
fn t3_rational_refusal_text(body: &topo::Body<f64>, who: &str) -> &'static str {
    let errors = topo::validate_geometric(body)
        .expect_err(&format!("{who}: tier 3 must refuse on the rational wall"));
    let [
        topo::ValidationError::VolumeUncomputable {
            source: topo::MassPropsError::Face { source, .. },
        },
    ] = errors.as_slice()
    else {
        panic!("{who}: expected exactly one per-face volume refusal, got: {errors:?}");
    };
    let geom_brep::props::PropsError::QuadratureUnsupported { what } = source else {
        panic!("{who}: expected the quadrature refusal, got: {source:?}");
    };
    what
}

/// **Spec §2 row 1 (rational half) + row for item 5's pin.** The
/// built arc_loft imports; census and validity match the SOURCE body
/// literally — tiers 1/2 green, and the tier-3 refusal is the SAME
/// variant with the SAME recourse text the native body produces (the
/// writer/reader asymmetry retired one arm down: what the kernel
/// writes, it reads back into the identical honest partial state).
#[test]
fn arc_loft_imports_to_exactly_the_native_state() {
    let native = native_arc_loft();
    assert_eq!(topo::validate(&native), Ok(()), "native tier 1");
    assert_eq!(topo::validate_closed(&native), Ok(()), "native tier 2");
    let native_refusal = t3_rational_refusal_text(&native, "native");
    assert!(
        native_refusal.contains("RATIONAL patch flux"),
        "the native refusal names the banked rational lane: {native_refusal}"
    );

    let text = step_export::step_string(&native, &step_export::StepOptions::default())
        .expect("the writer exports the rational-walled body today (measured)");
    let StepImport::Solid { body, .. } = import_step(&text, &ImportOptions::default())
        .expect("Arm B: the rational arm imports with the typed limitation")
    else {
        panic!("arc_loft must import as a solid");
    };

    assert_eq!(
        common::census(&body),
        common::census(&native),
        "census (solids, shells, faces, edges, vertices) matches the source body"
    );
    assert_eq!(topo::validate(&body), Ok(()), "imported tier 1");
    assert_eq!(topo::validate_closed(&body), Ok(()), "imported tier 2");
    let imported_refusal = t3_rational_refusal_text(&body, "imported");
    assert_eq!(
        imported_refusal, native_refusal,
        "the imported body's tier-3 refusal is the native body's, verbatim"
    );

    // The wall census behind the story: 4 NURBS walls arrived, exactly
    // one of them rational — the RATIONAL_B_SPLINE_SURFACE complex
    // instance genuinely took its own parse arm.
    let mut rational = 0;
    let mut non_rational = 0;
    for (_, face) in body.faces() {
        if let Some(geom_surfaces::Surface::Nurbs(p)) = body.get_surface(face.surface) {
            if p.weights().iter().all(|w| *w == 1.0) {
                non_rational += 1;
            } else {
                rational += 1;
            }
        }
    }
    assert_eq!(
        (non_rational, rational),
        (3, 1),
        "3 non-rational walls + 1 rational wall (arc side)"
    );
}

/// **Spec §2 row 5 — the surface_sig pin at body level.** The
/// committed loft_prism's four walls are four DISTINCT
/// `B_SPLINE_SURFACE_WITH_KNOTS` records; with the caps that is 6
/// distinct surface keys. The pre-M7-3 `surface_sig` Nurbs arm
/// (`vec![5u64]`) would have collapsed all four walls onto ONE shared
/// key (3 keys total) — a body that assembles, but with every wall
/// claiming the same surface: the silent-wrong-body class this pin
/// exists to keep dead. (The unit pin on `surface_sig` itself lives
/// in `adopt.rs`'s test module.)
#[test]
fn loft_prism_walls_get_distinct_surface_keys() {
    let (body, _) = import_body("loft_prism");
    let keys: std::collections::BTreeSet<_> = body.faces().map(|(_, f)| f.surface).collect();
    assert_eq!(
        keys.len(),
        6,
        "4 distinct NURBS walls + 2 caps = 6 distinct surface keys"
    );
}

/// **The description state** (spec items 3–4): every wall–wall seam
/// adopted as `IsoCurve` and every cap rim as a conventional
/// `MappedCurve` — the native loft builder's own description classes.
/// This is the load-bearing half of the fixed-point story: the
/// re-mint pass (`topo::mint_pcurves`, run unconditionally by the
/// import) derives every wall boundary's exact line-in-UV image from
/// exactly these descriptions.
#[test]
fn loft_prism_descriptions_land_in_the_native_classes() {
    let (body, _) = import_body("loft_prism");
    let mut iso = 0;
    let mut mapped = 0;
    for (_, edge) in body.edges() {
        match body.get_curve_geom(edge.curve) {
            Some(topo::CurveGeom::Certified(curve)) => match curve.description() {
                geom_brep::EdgeGeometry::IsoCurve { .. } => iso += 1,
                geom_brep::EdgeGeometry::MappedCurve(_) => mapped += 1,
                other => panic!("unexpected description class on a loft edge: {other:?}"),
            },
            other => panic!("every imported edge is certified, got: {other:?}"),
        }
    }
    assert_eq!(
        (iso, mapped),
        (4, 8),
        "4 wall–wall seams under IsoCurve, 8 cap rims under MappedCurve"
    );
}
