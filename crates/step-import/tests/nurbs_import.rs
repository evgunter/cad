//! **M7-3 acceptance: NURBS-face import** (`docs/M7-3-SPEC.md` §2).
//!
//! The committed non-rational loft (`loft_prism`) runs the FULL row-1
//! and fixed-point obligations through `roundtrip.rs` (it joined
//! `SOLID_FIXTURES` at this unit); this suite carries the rows that
//! are NURBS-specific:
//!
//! - the **surface_sig pin** at body level (spec item 2's trap): the
//!   four distinct walls must land on four distinct surface keys;
//! - the **rational arm** (spec item 5, Arm B): a natively built
//!   `arc_loft` (three walls non-rational, one RATIONAL) exports and
//!   then REFUSES at import with exactly the tier-3 verdict its native
//!   twin carries at rest. Arm B was "import with a typed limitation"
//!   until M7-7 wired the shared at-rest gate (#260 ruling (a)):
//!   `StepImport::Solid` may only carry a body the gate passes, and a
//!   body whose volume the kernel cannot compute is not one — so the
//!   limitation now lands as a typed refusal instead of riding out on
//!   a shipped body. Same verdict, same recourse text, earlier;
//!   the class returns to importing when the banked rational-wall
//!   quadrature lands;
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
//! walls, which since M7-7 refuse at the import gate on the banked
//! volume lane rather than shipping). That is a statement about the
//! builder and the quadrature, not the reader: the import side reads
//! any file in the written vocabulary, and only the at-rest gate has
//! anything left to say about the rational one.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::import_body;
use geom_core::{Affine3, Point2, Vec3};
use profile::{ProfileLoop, ProfileVertex};
use step_import::{ImportOptions, import_step};
use sweep::{Section, loft_body};

/// The arc-bearing profile loft the substrate measured (one bulged
/// side per section → 3 non-rational walls + 1 RATIONAL wall): the
/// writer exports it today with tiers 1/2 valid and a typed tier-3
/// volume refusal — the at-rest state its import must reproduce, and
/// therefore (the gate being shared) the refusal its import must
/// carry.
fn native_arc_loft() -> topo::Body<f64> {
    let arc_section = |s: f64| -> Section {
        let v = |x: f64, y: f64, bulge: f64| ProfileVertex {
            pos: Point2::new(x, y),
            bulge,
        };
        vec![ProfileLoop::new(vec![
            v(-s, -s, 0.0),
            // tan(π/8) on the +x side: a quarter-circle bulge-out.
            v(s, -s, 0.4142135623730951),
            v(s, s, 0.0),
            v(-s, s, 0.0),
        ])]
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
    rational_refusal_text(&errors, who)
}

/// The same read on a verdict list already in hand (the import gate
/// hands its verdicts back inside the typed refusal, never a body).
fn rational_refusal_text(errors: &[topo::ValidationError], who: &str) -> &'static str {
    let [
        topo::ValidationError::VolumeUncomputable {
            source: topo::MassPropsError::Face { source, .. },
        },
    ] = errors
    else {
        panic!("{who}: expected exactly one per-face volume refusal, got: {errors:?}");
    };
    let geom_brep::props::PropsError::QuadratureUnsupported { what } = source else {
        panic!("{who}: expected the quadrature refusal, got: {source:?}");
    };
    what
}

/// **THE M8-3 FLIP** of `arc_loft_refuses_at_import_with_the_native_bodys_verdict`,
/// executed as far as it goes and pinned exactly there.
///
/// The row's retirement condition was the banked rational patch flux,
/// and M8-3 retires it: the native arc loft is now tier-3 VALID with a
/// certified volume. What the row measured beyond that — the import
/// round trip — is blocked by a DIFFERENT and strictly narrower gap
/// (the adoption side derives no pcurve for the imported arc rim's
/// description kind), so the second half of this row pins that
/// residue by name instead of letting it hide behind the retired one.
///
/// The writer/reader symmetry the row was written for is intact and
/// now literal: the reader reaches exactly the native body's validity
/// state, and because `StepImport::Solid` may only carry a body the
/// shared at-rest gate passes, reaching a state the gate refuses means
/// refusing. A body whose volume the kernel cannot compute is not
/// tier-valid at rest, and shipping it would be import quietly holding
/// a laxer standard than the kernel (escalate-never-guess — an
/// indeterminate verdict is not a pass).
///
/// **The rational parse arm is pinned BY this refusal**: the banked
/// lane only fires on a face whose surface carries weights ≠ 1, so a
/// reader that dropped the `RATIONAL_B_SPLINE_SURFACE` weights would
/// produce a fully non-rational body whose volume computes — and this
/// row would go green through the import door instead. (The
/// non-rational side's positive control is `loft_prism`, which imports
/// and validates.)
#[test]
fn arc_loft_natively_computes_its_rational_volume() {
    let native = native_arc_loft();
    assert_eq!(topo::validate(&native), Ok(()), "native tier 1");
    assert_eq!(topo::validate_closed(&native), Ok(()), "native tier 2");
    assert_eq!(
        topo::validate_geometric(&native),
        Ok(()),
        "M8-3: the rational wall's volume certifies, so tier 3 now PASSES natively"
    );
    let want = topo::mass_properties(&native).expect("native rational mass properties");
    assert!(
        want.volume > 12.0 && want.volume < 13.0,
        "native arc-loft volume: {}",
        want.volume,
    );
    // The pad ceiling, pinned separately (the SKINFIT two-assertion
    // shape) so a loosening enclosure cannot absorb the band above.
    assert!(
        want.volume_pad < 5.0e-6,
        "native volume pad ceiling: {} (M8-3 measured 1.01e-6)",
        want.volume_pad,
    );

    // The writer still exports it; the READER still refuses — but for
    // a NEW and strictly narrower reason, pinned here so the residue
    // cannot go quiet. The rational quadrature is retired; what
    // remains is the ADOPTION-side pcurve derivation for an arc rim
    // whose imported description is not the loft builder's
    // `PlacedSegment` (M8-4's `nurbs_iso_derive` Intersection/adopted
    // arm). This row flips the rest of the way when that lands.
    let text = step_export::step_string(&native, &step_export::StepOptions::default())
        .expect("the writer exports the rational-walled body");
    let refusal = import_step(&text, &ImportOptions::default())
        .expect_err("the adoption-side pcurve gap still refuses the round trip");
    let msg = refusal.to_string();
    assert!(
        !msg.contains("RATIONAL patch flux"),
        "the RATIONAL patch flux bank is RETIRED — this refusal must not name it: {msg}"
    );
    assert!(
        msg.contains("no iso derivation for this description kind"),
        "the surviving refusal is the adoption-side pcurve derivation gap: {msg}"
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

/// **The description state, re-pinned at M7-6** (stage-1 promotion).
/// The four wall–wall seams still adopt as `IsoCurve` — each seam has
/// at least one stays-NURBS wall beside it, and its carrier
/// bitwise-matches that wall's boundary column (the native at-rest
/// preference, undisturbed by the neighbour's promotion). The cap
/// rims split by wall class: the four rims on the stays-NURBS walls
/// keep the conventional `MappedCurve` (the Nurbs-adjacency
/// exemption), while the four rims on the PROMOTED walls become
/// honest cap-plane × wall-plane `Intersection`s — a strictly
/// stronger description (certified against both surfaces) that
/// promotion unlocked; the ruling accepts the divergence and this
/// row enumerates it.
#[test]
fn loft_prism_descriptions_land_in_the_native_classes() {
    let (body, _) = import_body("loft_prism");
    let mut iso = 0;
    let mut mapped = 0;
    let mut intersection = 0;
    for (_, edge) in body.edges() {
        match body.get_curve_geom(edge.curve) {
            Some(topo::CurveGeom::Certified(curve)) => match curve.description() {
                geom_brep::EdgeGeometry::IsoCurve { .. } => iso += 1,
                geom_brep::EdgeGeometry::MappedCurve(_) => mapped += 1,
                geom_brep::EdgeGeometry::Intersection { .. } => intersection += 1,
                other => panic!("unexpected description class on a loft edge: {other:?}"),
            },
            other => panic!("every imported edge is certified, got: {other:?}"),
        }
    }
    assert_eq!(
        (iso, mapped, intersection),
        (4, 4, 4),
        "4 wall–wall seams under IsoCurve, 4 stays-NURBS-wall cap rims under \
         MappedCurve, 4 promoted-wall cap rims under Intersection"
    );
}
