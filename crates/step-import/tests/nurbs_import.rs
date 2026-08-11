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
//! The exportable NURBS class was bounded by the loft/sweep skin's
//! weight drift (#207): `sweep_body` with any curved path and
//! `loft_body` with any non-uniform section spacing refused at
//! assembly (`nurbs_span_meter` poison — the meter had no rational
//! arm at all, so a drifted weight channel was fatal on its own).
//! #207 fixed the drift at its source and M7's rational span meter
//! retired the poison itself, so the bodies this suite round-trips are
//! uniformly-spaced lofts by the fixture's own choice, not by refusal
//! — polyline profiles (non-rational, full tier 3) and arc-bearing
//! profiles (rational walls, which since M7-7 refuse at the import
//! gate on the banked QUADRATURE lane — a different bank from the span
//! meter's, and the one still open). That is a statement about the
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

/// The honest posture of a rational-wall body's mass properties at the
/// run's ε (M8-3). The schedule is FIXED (D9) and the target is
/// `1024·ε`, so `Ok` at one ε and a typed budget refusal at a tighter
/// one are both correct; nothing else is. `Ok` carries the certified
/// enclosure — the caller reuses it rather than paying a second
/// rational quadrature, which is the expensive thing in these rows.
fn rational_props_posture(body: &topo::Body<f64>, who: &str) -> Option<topo::MassProperties<f64>> {
    let target = 1024.0 * geom_core::Tolerance::get().eps;
    match topo::mass_properties(body) {
        Ok(props) => Some(props),
        Err(err) => {
            let topo::MassPropsError::Face { source, .. } = &err else {
                panic!("{who}: expected a per-face volume refusal, got: {err:?}");
            };
            match source {
                geom_brep::props::PropsError::QuadratureBudget {
                    width_len,
                    target_len,
                } => {
                    assert!(
                        width_len.is_finite() && width_len > target_len,
                        "{who}: a budget refusal must carry a width that really missed: \
                         {width_len:e} vs {target_len:e}"
                    );
                    assert!(
                        (target_len - target).abs() <= target * 1e-12,
                        "{who}: the refused target must BE 1024·ε: {target_len:e} vs {target:e}"
                    );
                    None
                }
                geom_brep::props::PropsError::Escalated { cause } => {
                    assert_eq!(
                        cause.predicate,
                        Some("props_quad_converged"),
                        "{who}: only the convergence predicate may escalate here: {cause:?}"
                    );
                    None
                }
                other => panic!("{who}: not an honest quadrature posture: {other:?}"),
            }
        }
    }
}

/// **THE M8-3 FLIP** of `arc_loft_refuses_at_import_with_the_native_bodys_verdict`,
/// executed in full.
///
/// The row's retirement condition was the banked rational patch flux,
/// and M8-3 retires it on both sides: the native arc loft is tier-3
/// VALID with a certified volume, and the round trip imports
/// FIRST-CLASS — the arc rim mints (`Pcurve::IsoArc`) from either
/// mapped description form, so the reader reaches the same state the
/// builder does.
///
/// The writer/reader symmetry the row was written for is now literal
/// AND green: `StepImport::Solid` may only carry a body the shared
/// at-rest gate passes, so an `Ok` here IS the imported body's tier-3
/// verdict. What used to make that impossible — a body whose volume
/// the kernel could not compute is not tier-valid at rest, and
/// shipping it would be import quietly holding a laxer standard than
/// the kernel — has stopped being true rather than been relaxed.
///
/// **The rational parse arm is still pinned BY this row**: a reader
/// that dropped the `RATIONAL_B_SPLINE_SURFACE` weights would produce
/// a non-rational body of a DIFFERENT volume, and the round-trip
/// volume agreement below is what catches that. (The non-rational
/// side's positive control is `loft_prism`.)
///
/// **ε posture.** The schedule is fixed (D9) against a `1024·ε`
/// target, so at a tight enough ε the honest outcome is a typed
/// budget refusal on both sides. All three outcomes are pinned; none
/// is widened.
#[test]
fn arc_loft_natively_computes_its_rational_volume() {
    let native = native_arc_loft();
    assert_eq!(topo::validate(&native), Ok(()), "native tier 1");
    assert_eq!(topo::validate_closed(&native), Ok(()), "native tier 2");
    let eps = geom_core::Tolerance::get().eps;
    // Computed ONCE: a rational quadrature is the expensive thing in
    // this row, and the round trip below reuses this enclosure. Tier 3
    // consumes exactly this number through its +V invariant, and the
    // sweep suite's `tier3_admits_the_rational_wall_body` pins the
    // verdict itself — paying for it twice here would only buy the
    // same quadrature at the same ε.
    let native_props = rational_props_posture(&native, "native");
    let certified = native_props.is_some();
    if let Some(want) = &native_props {
        assert!(
            want.volume > 12.0 && want.volume < 13.0,
            "native arc-loft volume: {}",
            want.volume,
        );
        // The pad ceiling, pinned separately (the SKINFIT two-assertion
        // shape) so a loosening enclosure cannot absorb the band above.
        // Keyed to ε, never widened: the schedule is fixed, so the pad
        // is what that schedule achieves against the run's own target.
        let ceiling = 2.0 * 1024.0 * eps;
        assert!(
            want.volume_pad < ceiling,
            "native volume pad ceiling: {} vs {ceiling} (M8-3 measured 1.0098754e-6 at ε=1e-9)",
            want.volume_pad,
        );
        println!(
            "M8-3 arc loft @ eps={eps:e}: Certified, {} ± {}",
            want.volume, want.volume_pad
        );
    } else {
        println!("M8-3 arc loft @ eps={eps:e}: Budget (the fixed schedule's honest frontier)");
    }

    // ---- The ROUND TRIP, which the bank used to block entirely. ----
    let text = step_export::step_string(&native, &step_export::StepOptions::default())
        .expect("the writer exports the rational-walled body");
    match import_step(&text, &ImportOptions::default()) {
        Ok(step_import::StepImport::Solid { body, .. }) => {
            assert!(
                certified,
                "an imported Solid means the at-rest gate passed, which means the \
                 quadrature certified — it cannot happen at an ε where the native \
                 body's own flux ran out of schedule"
            );
            let got = topo::mass_properties(&body).expect("imported rational mass properties");
            let want = native_props.as_ref().expect("the native side certified");
            // **BIT identity, not overlap** (R1 MINOR-2). Overlap is
            // what soundness needs — two certified enclosures of one
            // solid must intersect — but it is not what actually
            // happens, and asserting the weaker fact would let a real
            // divergence hide inside a pad. The reader reconstructs the
            // same chart and the same trim, so the same quadrature runs
            // on the same bits: pin THAT, and let a single ulp of drift
            // be a finding.
            assert_eq!(
                got.volume.to_bits(),
                want.volume.to_bits(),
                "the round trip must reproduce the volume BIT-identically: \
                 imported {} ± {}, native {} ± {}",
                got.volume,
                got.volume_pad,
                want.volume,
                want.volume_pad,
            );
            assert_eq!(
                got.volume_pad.to_bits(),
                want.volume_pad.to_bits(),
                "and the same certified pad: imported {}, native {}",
                got.volume_pad,
                want.volume_pad,
            );
            println!(
                "M8-3 arc loft round trip @ eps={eps:e}: FIRST-CLASS, {} ± {}",
                got.volume, got.volume_pad
            );
        }
        // The fixed schedule's honest frontier, reached through the
        // import gate instead of the native one.
        Err(refusal @ step_import::StepImportError::TierInvalid { .. }) => {
            assert!(
                !certified,
                "the import gate may only refuse where the native body's flux also \
                 ran out of schedule: {refusal:?}"
            );
            let msg = refusal.to_string();
            assert!(
                !msg.contains("RATIONAL patch flux"),
                "the RATIONAL patch flux bank is RETIRED — no refusal may name it: {msg}"
            );
            assert!(
                msg.contains("stalled at a mean boundary displacement"),
                "the only surviving refusal is the quadrature budget, with its number: {msg}"
            );
        }
        other => panic!("no other round-trip posture is pinned: {other:?}"),
    }
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
