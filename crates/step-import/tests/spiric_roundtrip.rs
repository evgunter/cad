//! **The spiric rim's interchange row** (CURVED-SPIRIC §5): where a
//! body carrying `Curve3::Spiric` edges lands when it is written out
//! as an approximating spline and read back.
//!
//! A spiric is a bicircular quartic of genus 1, so no rational
//! parameterization of it exists and AP214 has no entity for it. The
//! writer's one approximation is therefore this: an export-only cubic
//! `B_SPLINE_CURVE_WITH_KNOTS` whose bound the file states in its
//! `FILE_DESCRIPTION`. This row measures the other end of that trip —
//! what the kernel makes of the spline when it reads it back — and
//! pins it, because the two halves live in two crates and only this
//! one dev-depends on both.
//!
//! The export runs at `uncertainty_m = 1e-4`: §5's sagitta
//! certificate is second order and cannot state the kernel's own ε
//! under its 1024-node cap
//! (`work/curved/spiric-step-spline-bound-is-second-order.md`), which
//! `sweep`'s `spiric_rim` row pins from the writer's side.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Tol};
use profile::path::{Open, Start};
use profile::{ArcSweep, Center, Profile, ProfileLoop, SketchPlane};
use step_import::{ImportOptions, StepImport, import_step};
use sweep::{Revolution, RevolveAxis, revolve};

fn tol() -> Tol {
    Tol::witness()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The sectioned vessel's cavity — the tour's `torusvessel` meridian
/// (`sweep/tests/spiric_rim.rs` spells the same stations), hollowed
/// through the axial door so its torus-wall rims mint as spirics.
/// Taken BEFORE tier 3, which is where the native body stops.
fn vessel_cavity() -> topo::Body<f64> {
    let (r_foot, r_band, r_neck) = (5.0 / 64.0, 9.0 / 64.0, 7.0 / 64.0);
    let (y_foot, y_shoulder, y_mouth) = (4.0 / 64.0, 12.0 / 64.0, 24.0 / 64.0);
    let (h_tube, r_bellied) = (8.0 / 64.0, 6.0 / 64.0);
    let lp: ProfileLoop<f64> = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(r_foot, 0.0), tol())
        .expect("the base disc")
        .line_to(p2(r_foot, y_foot), tol())
        .expect("the foot")
        .line_to(p2(r_band, y_foot), tol())
        .expect("the lower shoulder")
        .arc_to(
            Center {
                c: p2(r_bellied, h_tube),
                winding: ArcSweep::Ccw,
                p: p2(r_band, y_shoulder),
            },
            tol(),
        )
        .expect("the band")
        .line_to(p2(r_neck, y_shoulder), tol())
        .expect("the upper shoulder")
        .line_to(p2(r_neck, y_mouth), tol())
        .expect("the neck")
        .line_to(p2(0.0, y_mouth), tol())
        .expect("the mouth disc")
        .line_to(Start, tol())
        .expect("the axis closes the meridian")
        .into();
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the meridian validates");
    let quarter = revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol(),
    )
    .expect("the meridian revolves")
    .body;
    let mut charts: Vec<(topo::SurfaceKey, Vec<topo::FaceKey>)> = Vec::new();
    for (k, f) in quarter.faces() {
        match charts.iter_mut().find(|(s, _)| *s == f.surface) {
            Some((_, v)) => v.push(k),
            None => charts.push((f.surface, vec![k])),
        }
    }
    let moves: Vec<topo::ChartMove<f64>> = charts
        .into_iter()
        .map(|(_, faces)| {
            let sense = quarter.get_face(faces[0]).expect("face").sense;
            topo::ChartMove {
                faces,
                distance: if sense { -1.0 / 128.0 } else { 1.0 / 128.0 },
            }
        })
        .collect();
    let mut cavity = quarter.clone();
    topo::offset_charts_together(
        &mut cavity,
        &moves,
        Band::linear(tol()).expect("band"),
        tol(),
    )
    .expect("the vessel's corners solve and its rims mint");
    cavity
}

/// **Where the trip lands.** The cavity writes out with one cubic
/// `B_SPLINE_CURVE_WITH_KNOTS` per spiric rim and reads back as a
/// solid; the imported body's tier-3 answer is the payload this row
/// pins, beside the NATIVE body's, so the two doors are compared
/// rather than asserted equal by hope.
#[test]
fn a_spiric_rim_exports_as_a_spline_and_its_reimport_door_is_pinned() {
    let native = vessel_cavity();
    let rims = native
        .edges()
        .filter(|(_, e)| {
            native
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Spiric { .. }))
        })
        .count();
    assert_eq!(rims, 2, "the cavity carries two spiric rims");

    let text = step_export::step_string(
        &native,
        &step_export::StepOptions {
            uncertainty_m: Some(1e-4),
            ..Default::default()
        },
        tol(),
    )
    .expect("the cavity exports at a tolerance the sagitta bound can state");
    assert_eq!(
        text.matches("B_SPLINE_CURVE_WITH_KNOTS").count(),
        rims,
        "one export-only spline per spiric rim"
    );
    println!(
        "[roundtrip] native tier 3: {:?}",
        topo::validate_geometric(&native, tol())
    );

    let back = match import_step(&text, &ImportOptions::default(), tol()) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("the cavity must re-import as a solid, got {other:?}"),
    };
    assert_eq!(topo::validate(&back), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&back), Ok(()), "tier 2");
    let tier3 = topo::validate_geometric(&back, tol());
    println!("[roundtrip] imported tier 3: {tier3:?}");
    assert!(
        tier3.is_err(),
        "the imported body's volume is the props lane's frontier, not a computed number"
    );
}
