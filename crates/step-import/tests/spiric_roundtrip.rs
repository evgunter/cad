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
    let mut cavity = quarter.clone().into_body();
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
/// `B_SPLINE_CURVE_WITH_KNOTS` per spiric rim, and comes back whole. The
/// native body passes tier 3: check 7 measures its spiric-bounded caps
/// by the spiric quadrature lane and its torus wall by the rotational
/// lane's spiric arm.
///
/// Each rim comes back as a spline, the export's sagitta-bounded
/// stand-in for the spiric (uncertainty 1e-4 m;
/// `step_export::spiric_export_spline`), whose `Intersection` reading is
/// a rung-3 carrier between the cutting plane and the torus. That
/// reading certifies: rebuilt here from each native rim by the writer's
/// own door, the spline passes the edge certificate against the plane
/// and the torus (limbs 2 and 3; the spline lies within about 3e-13 m of
/// the torus). Check 7 then measures the torus wall the spline bounds
/// through the rotational lane's projected arm, and the native and the
/// imported cavity each hold the volume sliced from the authored
/// stations ([`cavity_volume`]).
#[test]
fn a_spiric_rim_exports_as_a_spline_and_reimports_at_its_volume() {
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
        native
            .faces()
            .all(|(fk, _)| native.face_surface(fk).is_some_and(|s| matches!(
                s,
                geom::Surface::Plane { .. }
                    | geom::Surface::Cylinder { .. }
                    | geom::Surface::Torus { .. }
            ))),
        "the cavity's walls are planes, cylinders and tori"
    );

    let back = import_step(&text, &ImportOptions::default(), tol());
    let Ok(step_import::StepImport::Solid { body, .. }) = &back else {
        panic!("the imported cavity builds, its spline-trimmed torus wall measured, got {back:?}");
    };
    let truth = cavity_volume();
    for (what, measured) in [
        (
            "native",
            topo::mass_properties(
                &topo::test_support::finished("the cavity", native.clone(), tol()),
                tol(),
                topo::test_support::pinned_resolution(tol()),
            )
            .expect("the native cavity measures")
            .enclosure,
        ),
        (
            "imported",
            body.measure(topo::test_support::pinned_resolution(tol()))
                .expect("the imported cavity measures")
                .enclosure,
        ),
    ] {
        assert!(
            (measured.volume - truth).abs() <= measured.volume_pad + ORACLE_SLACK,
            "the {what} cavity's volume {} ± {} misses the sliced {truth}",
            measured.volume,
            measured.volume_pad
        );
    }
}

/// What the slicing oracle's own quadrature may be off by (m³), far
/// below any width the enclosures read.
const ORACLE_SLACK: f64 = 1e-12;

/// **The cavity's volume by slicing**, from the authored stations alone:
/// each face moved inward by `δ = 1/128`, the meridian's edges mitred
/// (the foot `x = r_foot − δ` above `y = δ`, the shoulder `y = y_foot + δ`
/// out to the belly's arc of radius `5/64 − δ` about `(6/64, 8/64)`, up
/// to `y = y_shoulder − δ`, the neck `x = r_neck − δ` up to
/// `y = y_mouth − δ`), and each slice the quarter disc of that radius
/// less the strips `x < δ` and `z < δ` its two moved cut planes take,
/// `∫_δ^√(ρ²−δ²) (√(ρ² − x²) − δ) dx` in closed form. The slices are
/// integrated by the 7–15 panel under the sine map on the pieces between
/// the profile's corners, halved to `1e-14`.
fn cavity_volume() -> f64 {
    use core::f64::consts::FRAC_PI_2;
    let d = 1.0 / 128.0;
    let (r_foot, r_neck) = (5.0 / 64.0, 7.0 / 64.0);
    let (y_foot, y_shoulder, y_mouth) = (4.0 / 64.0, 12.0 / 64.0, 24.0 / 64.0);
    let (cx, cy, rr) = (6.0 / 64.0, 8.0 / 64.0, 5.0 / 64.0 - d);
    let radius = |y: f64| {
        if y <= y_foot + d {
            r_foot - d
        } else if y <= y_shoulder - d {
            cx + (rr * rr - (y - cy) * (y - cy)).max(0.0).sqrt()
        } else {
            r_neck - d
        }
    };
    let slice = |y: f64| {
        let rho = radius(y);
        let prim = |x: f64| {
            0.5 * (x * (rho * rho - x * x).max(0.0).sqrt() + rho * rho * (x / rho).asin()) - d * x
        };
        prim((rho * rho - d * d).sqrt()) - prim(d)
    };
    fn adapt(f: &dyn Fn(f64) -> f64, a: f64, b: f64, depth: u32) -> f64 {
        let (k, e) = gk15(f, a, b);
        if e <= 1e-17 || depth >= 40 {
            return k;
        }
        let m = 0.5 * (a + b);
        adapt(f, a, m, depth + 1) + adapt(f, m, b, depth + 1)
    }
    let cuts = [d, y_foot + d, y_shoulder - d, y_mouth - d];
    cuts.windows(2)
        .map(|w| {
            let (m, h) = (0.5 * (w[0] + w[1]), 0.5 * (w[1] - w[0]));
            let g = |th: f64| slice(m + h * th.sin()) * h * th.cos();
            adapt(&g, -FRAC_PI_2, FRAC_PI_2, 0)
        })
        .sum()
}

/// The 7–15 Gauss–Kronrod panel on `[a, b]`: its Kronrod estimate and
/// its distance from the embedded Gauss rule.
fn gk15(f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> (f64, f64) {
    const XK: [f64; 8] = [
        0.991_455_371_120_812_6,
        0.949_107_912_342_758_5,
        0.864_864_423_359_769_1,
        0.741_531_185_599_394_4,
        0.586_087_235_467_691_1,
        0.405_845_151_377_397_2,
        0.207_784_955_007_898_5,
        0.0,
    ];
    const WK: [f64; 8] = [
        0.022_935_322_010_529_22,
        0.063_092_092_629_978_55,
        0.104_790_010_322_250_18,
        0.140_653_259_715_525_92,
        0.169_004_726_639_267_9,
        0.190_350_578_064_785_4,
        0.204_432_940_075_298_9,
        0.209_482_141_084_727_83,
    ];
    const WG: [f64; 4] = [
        0.129_484_966_168_869_7,
        0.279_705_391_489_276_7,
        0.381_830_050_505_118_9,
        0.417_959_183_673_469_4,
    ];
    let (c, h) = (0.5 * (a + b), 0.5 * (b - a));
    let fc = f(c);
    let (mut k, mut g) = (WK[7] * fc, WG[3] * fc);
    for i in 0..7 {
        let (lo, hi) = (f(c - h * XK[i]), f(c + h * XK[i]));
        k += WK[i] * (lo + hi);
        if i % 2 == 1 {
            g += WG[i / 2] * (lo + hi);
        }
    }
    (k * h, ((k - g) * h).abs())
}
