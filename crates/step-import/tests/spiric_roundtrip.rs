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
use step_import::{ImportOptions, import_step};
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
/// `B_SPLINE_CURVE_WITH_KNOTS` per spiric rim, and the import door
/// itself refuses. The native body passes tier 3: check 7 measures its
/// spiric-bounded caps and torus wall by the spiric quadrature lane (it
/// stood at `VolumeUncomputable { Face { FaceKey(1v1), Unimplemented } }`,
/// a cap's `loop_vector_area`, before that lane).
///
/// The import stops at tier 3, past adoption. Each rim comes back as a
/// spline, the export's sagitta-bounded stand-in for the spiric
/// (uncertainty 1e-4 m; `step_export::spiric_export_spline`), whose
/// `Intersection` reading is a rung-3 carrier between the cutting plane
/// and the torus. That reading certifies: rebuilt here from each native
/// rim by the writer's own door, the spline passes the edge certificate
/// against the plane and the torus (limbs 2 and 3; the spline lies
/// within about 3e-13 m of the torus). Check 7 then measures the torus
/// wall the spline bounds: the cavity's walls are planes, cylinders and
/// tori, a plane goes to the rim lane, and the conic-trim fallback
/// refuses every chart but the cylinder's, so the refused face is a
/// torus wall — no quadrature lane reads one trimmed by a spline
/// (`work/spiricarm/a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane.md`).
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
        &crate::common::at_rest(&native),
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
    assert_eq!(
        topo::validate_geometric(&native, tol()),
        Ok(()),
        "the native cavity passes tier 3, its spiric faces measured"
    );

    let band = Band::linear(tol()).expect("band");
    let mut adopted = 0;
    for (key, e) in native.edges() {
        let Some(curve) = native.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        let &geom::Curve3::Spiric {
            center,
            axis,
            u_ref,
            major_radius,
            minor_radius,
            offset,
        } = curve.carrier()
        else {
            continue;
        };
        let (t0, t1) = curve.params();
        let (spline, _) = step_export::spiric_export_spline(
            1e-4,
            curve.carrier(),
            key,
            t0,
            t1,
            major_radius,
            minor_radius,
            offset,
        )
        .expect("the writer's spline");
        let plane = geom::Surface::Plane {
            origin: center + u_ref * offset,
            normal: u_ref,
            u_ref: axis,
        };
        let torus = geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        };
        assert_eq!(
            geom_brep::analytic_rung3(&spline, spline.domain(), &plane, &torus, band),
            Ok(()),
            "the rim's spline certifies as the plane × torus intersection"
        );
        adopted += 1;
    }
    assert_eq!(adopted, rims, "every rim's spline read");
    assert!(
        native.faces().all(
            |(_, f)| native.get_surface(f.surface).is_some_and(|s| matches!(
                s,
                geom::Surface::Plane { .. }
                    | geom::Surface::Cylinder { .. }
                    | geom::Surface::Torus { .. }
            ))
        ),
        "the cavity's walls are planes, cylinders and tori"
    );

    let back = import_step(&text, &ImportOptions::default(), tol());
    println!("[roundtrip] import door: {back:?}");
    let Err(step_import::StepImportError::TierInvalid { errors, .. }) = &back else {
        panic!("the imported cavity refuses at tier 3, got {back:?}");
    };
    assert!(
        matches!(
            errors.as_slice(),
            [topo::ValidationError::VolumeUncomputable {
                source: topo::props::MassPropsError::Face {
                    source: geom_brep::props::PropsError::QuadratureUnsupported { what },
                    ..
                },
                ..
            }] if what.starts_with("conic trim on a cone/sphere/torus chart")
        ),
        "the spline's Intersection reading adopts, and check 7 meets a torus wall trimmed by a \
         spline, got {errors:?}"
    );
}
