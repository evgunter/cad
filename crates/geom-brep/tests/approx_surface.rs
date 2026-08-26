//! `Surface::Approx` — the triple's own acceptance rows.
//!
//! The body-level consumer (surgery + tier-3 re-derivation) lives in
//! `sweep/tests/verbs_offc_consumer.rs`, where a NURBS-faced body can
//! be lofted. What is pinned HERE is what the surface alone claims:
//!
//! - the door mints only a certified surface, and the fit door's
//!   refusals (rational fits included) propagate out of it verbatim;
//! - the evaluators, the derived normal and the boxes read the FIT;
//! - the re-derivation door re-measures rather than reads, so a
//!   coarsened fit goes red there while its stored certificate still
//!   says it is fine;
//! - **the composition law**: for a rigid map `M`,
//!   `M(S + d·n) = M(S) + d·n_M` — so the fit of an offset, mapped, is
//!   a certified fit of the offset of the mapped base, at the SAME
//!   tolerance. That is why the description is the layer the map
//!   composes with, and it is pinned numerically here even though the
//!   `topo` transform pass refuses the kind (it cannot re-derive a
//!   certificate).
//! - both signs of `d`, and the kind's own dispositions at the
//!   dispatch sites that answer for it structurally.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;
use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_brep::offset_fit::{
    OffsetFitError, approx_offset_surface, certify_offset, offset_point, recertify_approx,
};
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn kv1() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}

fn kv2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}

/// A gently bowed polynomial patch over `[0,1]²` — a base whose offset
/// is genuinely not a NURBS, so the fit has real work to do.
fn bowed() -> NurbsSurface<f64> {
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let (u, v) = (f64::from(i) * 0.5, f64::from(j) * 0.5);
            control.push(Point3::new(u, v, 0.15 * u * (1.0 - u) + 0.1 * v * v));
        }
    }
    NurbsSurface::new(kv2(), kv2(), control, vec![1.0; 9]).unwrap()
}

/// The exact quarter cylinder — a RATIONAL base (unit weights are the
/// fit's business, not the base's).
fn quarter_cylinder(r: f64, h: f64) -> NurbsSurface<f64> {
    let s = (FRAC_PI_2 * 0.5).cos();
    let control = vec![
        Point3::new(r, 0.0, 0.0),
        Point3::new(r, 0.0, h),
        Point3::new(r, r, 0.0),
        Point3::new(r, r, h),
        Point3::new(0.0, r, 0.0),
        Point3::new(0.0, r, h),
    ];
    let weights = vec![1.0, 1.0, s, s, 1.0, 1.0];
    NurbsSurface::new(kv2(), kv1(), control, weights).unwrap()
}

/// `map`'s image of a spline, control point by control point — the
/// rigid map of a NURBS surface, which is a rigid map of its net and
/// nothing else (weights are invariant; knots are parameters).
fn map_net(map: &Affine3<f64>, s: &NurbsSurface<f64>) -> NurbsSurface<f64> {
    NurbsSurface::new(
        s.knots_u().clone(),
        s.knots_v().clone(),
        s.control()
            .iter()
            .map(|p| map.transform_point(*p))
            .collect(),
        s.weights().to_vec(),
    )
    .unwrap()
}

fn approx_of(s: &Surface<f64>) -> &geom::ApproxSurface<f64> {
    match s {
        Surface::Approx(a) => a,
        other => panic!("the door must mint Surface::Approx, got {other:?}"),
    }
}

// ---------------------------------------------------------------------
// The door
// ---------------------------------------------------------------------

/// The door mints the variant, and everything it stores is what it was
/// asked for: the description names the base handed in (same `Arc`),
/// the window is the base's own knot domain, and the certificate's
/// distance is `d`.
#[test]
fn the_door_stores_what_it_was_asked_for() {
    for d in [0.05_f64, -0.05] {
        let base = Arc::new(bowed());
        let s = approx_offset_surface(Arc::clone(&base), d, 1e-6, band())
            .unwrap_or_else(|e| panic!("d = {d}: {e}"));
        let a = approx_of(&s);
        let geom::SurfaceDescription::Offset {
            base: stored,
            d: sd,
        } = a.description();
        assert!(
            Arc::ptr_eq(stored, &base),
            "d = {d}: the base travels by Arc"
        );
        assert_eq!(*sd, d);
        assert_eq!(a.tolerance(), 1e-6);
        assert_eq!(a.certificate().distance, d);
        assert_eq!(a.window(), geom::ApproxWindow::of(&*base));
        assert!(
            a.certificate().hull_sup <= 1e-6,
            "d = {d}: the stored certificate is the one that certified"
        );
        // `rounds` is the FIT's provenance and travels with it — never
        // flattened to the re-derivation's zero.
        assert_eq!(
            a.certificate().rounds,
            geom_brep::offset_fit::fit_offset(&base, d, 1e-6, band())
                .unwrap()
                .1
                .rounds,
            "d = {d}: the stored round count is the fit loop's own"
        );
    }
}

/// The stored certificate is derived from the STORED pair, not carried
/// out of the refinement loop: a re-derivation of the surface reports
/// the identical two limbs.
#[test]
fn the_stored_certificate_is_a_certificate_of_the_stored_pair() {
    let s = approx_offset_surface(Arc::new(bowed()), 0.05, 1e-6, band()).unwrap();
    let a = approx_of(&s);
    let re = recertify_approx(a, 1e-6, band()).expect("the surface re-certifies at rest");
    assert_eq!(re.hull_sup, a.certificate().hull_sup);
    assert_eq!(re.on_locus_max, a.certificate().on_locus_max);
    assert_eq!(re.cells, a.certificate().cells);
}

/// A tolerance no fit can reach refuses at the door — nothing
/// uncertified is minted, and the refusal is the fit door's own.
#[test]
fn an_unreachable_tolerance_refuses_typed() {
    let e = approx_offset_surface(Arc::new(bowed()), 0.3, 1e-18, band())
        .expect_err("1e-18 m on a bowed patch is not reachable");
    assert!(
        matches!(e, OffsetFitError::BudgetExhausted { .. }),
        "expected the budget refusal, got {e}"
    );
}

/// `d = 0` and a non-finite `d` are not offsets at all — the door's
/// request check, propagated.
#[test]
fn a_degenerate_request_refuses_typed() {
    for d in [0.0_f64, f64::NAN, f64::INFINITY] {
        let e = approx_offset_surface(Arc::new(bowed()), d, 1e-6, band())
            .expect_err("a degenerate d has no offset");
        assert!(
            matches!(e, OffsetFitError::InvalidRequest { .. }),
            "d = {d}: got {e}"
        );
    }
}

/// The `RationalFitUnsupported` refusal is never bypassed: the
/// certification door refuses a rational fit, and the storage door is
/// built on that door, so no rational fit can reach an
/// `ApproxSurface`.
#[test]
fn a_rational_fit_never_reaches_the_stored_surface() {
    // `fit_offset` never mints a rational fit, so the refusal is
    // exercised through `certify_offset` — the door the storage layer
    // calls. What this row pins is that the storage door has no path
    // that skips it.
    let base = quarter_cylinder(1.0, 1.0);
    let rational_fit = quarter_cylinder(1.2, 1.0);
    let e = certify_offset(&base, &rational_fit, 0.2, 1e-3, band())
        .expect_err("a rational fit cannot be certified by the hull limb");
    assert!(
        matches!(e, OffsetFitError::RationalFitUnsupported { .. }),
        "got {e}"
    );
    // And the storage door's own fits are non-rational, so it mints.
    let s = approx_offset_surface(Arc::new(base), 0.2, 1e-4, band()).unwrap();
    assert!(
        approx_of(&s).fit().weights().iter().all(|w| *w == 1.0),
        "the door's fit is non-rational"
    );
}

// ---------------------------------------------------------------------
// The fit IS the geometry
// ---------------------------------------------------------------------

/// Evaluation, the whole jet and the derived normal all read the fit,
/// bit for bit — the variant's stated invariant, at the enum's doors.
#[test]
fn the_evaluators_delegate_to_the_fit_bitwise() {
    let s = approx_offset_surface(Arc::new(bowed()), 0.05, 1e-6, band()).unwrap();
    let fit = approx_of(&s).fit().clone();
    for i in 0..=4 {
        for j in 0..=4 {
            let (u, v) = (f64::from(i) * 0.25, f64::from(j) * 0.25);
            let p = s.eval(u, v);
            let q = fit.eval(u, v);
            assert_eq!((p.x, p.y, p.z), (q.x, q.y, q.z), "eval at ({u}, {v})");
            let a = s.jet(u, v);
            let b = fit.ders(u, v);
            assert_eq!(a.du.x, b.du.x, "jet du at ({u}, {v})");
            assert_eq!(a.dvv.z, b.dvv.z, "jet dvv at ({u}, {v})");
            let n = s.normal(u, v);
            let m = Surface::Nurbs(Arc::new(fit.clone())).normal(u, v);
            assert_eq!((n.x, n.y, n.z), (m.x, m.y, m.z), "normal at ({u}, {v})");
        }
    }
}

/// And the fit really is within the certificate's bound of the
/// described offset locus — the certificate says something true about
/// the two things the surface carries.
#[test]
fn the_fit_is_within_the_certified_bound_of_the_description() {
    for d in [0.05_f64, -0.05] {
        let base = Arc::new(bowed());
        let s = approx_offset_surface(Arc::clone(&base), d, 1e-6, band()).unwrap();
        let bound = approx_of(&s).certificate().hull_sup;
        let mut worst = 0.0_f64;
        for i in 0..=11 {
            for j in 0..=11 {
                let (u, v) = (f64::from(i) / 11.0, f64::from(j) / 11.0);
                let exact = offset_point(&base, d, u, v).expect("the base is regular here");
                worst = worst.max(s.eval(u, v).distance(exact));
            }
        }
        assert!(
            worst <= bound,
            "d = {d}: a dense sample found {worst:e} m against the certified sup {bound:e} m"
        );
    }
}

/// `spline_chart` is the accessor every chart consumer routes through,
/// and it answers the FIT for an approximating surface — the one place
/// the delegation is stated once for all of them.
#[test]
fn the_spline_chart_accessor_answers_the_fit() {
    let s = approx_offset_surface(Arc::new(bowed()), 0.05, 1e-6, band()).unwrap();
    let chart = s
        .spline_chart()
        .expect("an approximating surface has a spline chart");
    assert!(std::ptr::eq(chart, approx_of(&s).fit()));
    // The analytic kinds have none, and the placeholder is still a
    // chart (its own refusals live downstream).
    assert!(
        Surface::<f64>::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        }
        .spline_chart()
        .is_none()
    );
}

// ---------------------------------------------------------------------
// The never-trust posture, red direction
// ---------------------------------------------------------------------

/// A **planted degraded fit**: an `ApproxSurface` minted through the
/// injection door with a certifier that hands back a clean certificate
/// for a coarsened net. Its stored certificate says the surface is
/// fine; the re-derivation door measures and refuses. That is exactly
/// the direction O5's never-trust posture exists for.
#[test]
fn a_planted_degraded_fit_goes_red_at_re_derivation() {
    let base = Arc::new(bowed());
    let honest = approx_offset_surface(Arc::clone(&base), 0.05, 1e-6, band()).unwrap();
    let good = approx_of(&honest);

    // Coarsen: push one interior control point of the fit a millimetre
    // off. The surface is still a valid spline; it is no longer within
    // 1e-6 m of the offset locus.
    let fit = good.fit();
    let mut control = fit.control().to_vec();
    let mid = control.len() / 2;
    control[mid] = control[mid] + Vec3::new(0.0, 0.0, 1e-3);
    let coarsened = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        control,
        fit.weights().to_vec(),
    )
    .unwrap();

    let planted = geom::ApproxSurface::certify(
        geom::SurfaceSpec {
            description: geom::SurfaceDescription::Offset {
                base: Arc::clone(&base),
                d: 0.05,
            },
            fit: coarsened,
            window: good.window(),
            tolerance: good.tolerance(),
        },
        // A certifier that does not measure — the planted claim.
        |_, _, _, _| Ok::<_, OffsetFitError>(*good.certificate()),
    )
    .expect("the injection door stores what the certifier returned");

    assert_eq!(
        planted.certificate().hull_sup,
        good.certificate().hull_sup,
        "the stored certificate still claims the honest bound"
    );
    let e = recertify_approx(&planted, good.tolerance(), band())
        .expect_err("the re-derivation must refuse the coarsened fit");
    assert!(
        matches!(e, OffsetFitError::Limb { .. }),
        "expected a limb refusal, got {e}"
    );
}

/// **The classification tolerance is the CALLER's, not the surface's.**
/// A surface minted at a loose tolerance re-derives GREEN against that
/// loose bound and RED against a tighter one — the edge machinery's
/// exact posture, and the reason tier 3 passes the run's ε rather than
/// reading the stored field. D4 blesses the consequence: ε-tightening
/// may escalate, and a mint that no longer meets the ratified
/// `≤ ε_precision` claim refuses honestly.
#[test]
fn the_re_derivation_classifies_against_the_callers_tolerance() {
    let base = Arc::new(bowed());
    // Minted loose: the fit stops as soon as it is inside 1e-3.
    let s = approx_offset_surface(Arc::clone(&base), 0.05, 1e-3, band()).unwrap();
    let a = approx_of(&s);
    assert_eq!(a.tolerance(), 1e-3, "the stored tolerance is the MINT's");
    let loose = recertify_approx(a, 1e-3, band()).expect("green at the bound it was minted at");
    assert!(loose.hull_sup <= 1e-3);
    // The same surface, unchanged, at a tighter run epsilon.
    let e = recertify_approx(a, 1e-12, band())
        .expect_err("a loose mint must refuse at a tighter epsilon");
    assert!(
        matches!(e, OffsetFitError::Limb { .. }),
        "expected a limb refusal naming the bound it measured, got {e}"
    );
}

/// The storage door CHECKS the window it is asked for rather than
/// taking it on trust: the certificate covers the base's whole chart
/// rectangle and nothing narrower.
#[test]
fn a_window_the_certifier_cannot_honour_refuses_typed() {
    let base = Arc::new(bowed());
    let (fit, _) = geom_brep::offset_fit::fit_offset(&base, 0.05, 1e-6, band()).unwrap();
    let narrow = geom::ApproxWindow {
        u: (0.25, 0.75),
        v: (0.25, 0.75),
    };
    let e = geom::ApproxSurface::certify(
        geom::SurfaceSpec {
            description: geom::SurfaceDescription::Offset {
                base: Arc::clone(&base),
                d: 0.05,
            },
            fit,
            window: narrow,
            tolerance: 1e-6,
        },
        |description, fit, window, tolerance| {
            let geom::SurfaceDescription::Offset { base, d } = description;
            if window != geom::ApproxWindow::of(base) {
                return Err(OffsetFitError::WindowUnsupported { window });
            }
            certify_offset(base, fit, *d, tolerance, band())
        },
    )
    .expect_err("a sub-window is not a bound this certificate proved");
    assert!(
        matches!(e, OffsetFitError::WindowUnsupported { .. }),
        "got {e}"
    );
}

// ---------------------------------------------------------------------
// The composition law
// ---------------------------------------------------------------------

/// **Rigid-map-then-offset ≡ offset-then-rigid-map.** A rigid map
/// carries unit normals to unit normals, so `M(S + d·n)` is
/// `M(S) + d·n_M`. The consequence the description layer rests on: the
/// mapped fit certifies against the mapped base, at the same `d` and
/// the same tolerance — a certified statement, not a sampled one.
#[test]
fn a_rigid_map_of_an_offset_is_the_offset_of_the_rigid_map() {
    // A rotation about ẑ by 0.7 rad composed with a translation:
    // det = +1, the kernel's rigid contract.
    let mut map = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), 0.7);
    map.translation = map.translation + Vec3::new(0.3, -0.2, 1.1);
    for d in [0.05_f64, -0.05] {
        let base = Arc::new(bowed());
        let s = approx_offset_surface(Arc::clone(&base), d, 1e-6, band()).unwrap();
        let fit = approx_of(&s).fit();

        let mapped_base = map_net(&map, &base);
        let mapped_fit = map_net(&map, fit);
        // The map of the fit is a certified fit of the offset of the
        // map of the base — same d, same tolerance.
        let cert = certify_offset(&mapped_base, &mapped_fit, d, 1e-6, band()).unwrap_or_else(|e| {
            panic!("d = {d}: the composition law must hold under certification: {e}")
        });
        // And the two runs agree to well inside the tolerance: the
        // residual is a DISTANCE, which a rigid map preserves.
        let here = approx_of(&s).certificate().hull_sup;
        assert!(
            (cert.hull_sup - here).abs() <= 1e-9,
            "d = {d}: sup bounds diverge under a rigid map: {} vs {here}",
            cert.hull_sup
        );
    }
}

// ---------------------------------------------------------------------
// Dispositions that answer for the kind structurally
// ---------------------------------------------------------------------

/// `Approx` is its own [`geom_brep::SurfaceKind`] — not the kind its
/// fit is — and every pair the routing table names for it is refused.
#[test]
fn approx_is_its_own_kind_and_every_pair_refuses() {
    use geom_brep::intersect::{SurfaceKind, route};
    let s = approx_offset_surface(Arc::new(bowed()), 0.05, 1e-6, band()).unwrap();
    assert_eq!(SurfaceKind::of(&s), SurfaceKind::Approx);
    assert_ne!(SurfaceKind::of(&s), SurfaceKind::Nurbs);
    for other in [
        SurfaceKind::Plane,
        SurfaceKind::Cylinder,
        SurfaceKind::Cone,
        SurfaceKind::Sphere,
        SurfaceKind::Torus,
        SurfaceKind::Nurbs,
        SurfaceKind::Approx,
    ] {
        for (a, b) in [(SurfaceKind::Approx, other), (other, SurfaceKind::Approx)] {
            assert!(
                !route(a, b).implemented,
                "{a:?} x {b:?} must refuse: an SSI claim about a fit is not one about the \
                 described surface"
            );
        }
    }
}

/// Offsetting an approximating surface would nest one description
/// inside another — refused typed, not silently fitted again.
#[test]
fn offsetting_an_approximating_surface_refuses_typed() {
    let s = approx_offset_surface(Arc::new(bowed()), 0.05, 1e-6, band()).unwrap();
    let e = geom_brep::offset_surface(&s, 0.05, band()).expect_err("nesting refuses");
    assert!(
        matches!(e, geom_brep::OffsetError::ApproxNesting),
        "got {e}"
    );
}

/// The implicit-form layer answers poison, as it does for a spline:
/// there is no implicit form for a fit, and an offset description has
/// none to lend.
#[test]
fn the_implicit_layer_is_poison_for_an_approximating_surface() {
    let s = approx_offset_surface(Arc::new(bowed()), 0.05, 1e-6, band()).unwrap();
    let p = Point3::new(0.5, 0.5, 0.2);
    assert!(geom_brep::implicit_residual(&s, p).is_nan());
    assert!(geom_brep::implicit_gradient(&s, p).x.is_nan());
}
