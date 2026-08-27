//! OFF-C reviewer probes (r1): an `Approx`-faced body whose walls are
//! genuinely CURVED — a twisted loft (skinned bilinear saddles), not
//! the PR's straight prism — validated tier 3 end to end, tessellated,
//! and driven at the boolean doors.
//!
//! What these rows add over `verbs_offc_consumer.rs`:
//!
//! - the pulled-back-base construction on a NON-planar wall, where the
//!   pull-back is only approximately the inverse of the offset (the
//!   chart normal varies over a saddle), so `d` must be small enough
//!   that the description's locus stays inside the edge bands — the
//!   first body-level exercise of the re-derivation arithmetic on a
//!   curved chart;
//! - the never-trust red row on that curved chart;
//! - the boolean refusal SHAPE on a lofted `Approx` operand (the edge
//!   rule fires first — recorded, not assumed) beside the germ-pair
//!   refusal from a doubly-curved SKINNED base on a box cap.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, NurbsSurface, Surface};
use geom_core::{Point3, Tol, Vec3};
use topo::{Body, CurveGeom, FaceKey, FaceSurface};

mod common;
use common::approx::{
    ReattachRefusal, band, moved_box, pulled_back, reattach_certifies_at, top_face,
    try_approx_walls, twisted_loft, unit_box,
};

/// Small enough that the pull-back error `d·(n(u,v) − n₀)` on the
/// twisted walls stays well inside the default ε = 1e-9 band (at
/// d = 2e-9 the edge residual measured 2.3e-9 and ESCALATED in the
/// ambiguity band — the honest record of how tight the coherence
/// budget is for a curved pulled-back base); large enough to be far
/// above f64 dust.
const D: f64 = 5e-10;
const FIT_TOL: f64 = 1e-6;

/// **The measured re-attach threshold of the twisted fixture**, in
/// metres: the largest `IsoCurve` residual the surgery's re-attach
/// hands the certifier, over every edge and every schedule sample, at
/// `d = ±D`.
///
/// The pull-back is exact only on a PLANE: on a saddle the base is
/// translated along the wall's MID-POINT normal, so the described
/// offset misses the wall by `d·(n(u,v) − n₀)` and the residual is
/// construction arithmetic — a fixed number of metres, independent of
/// ε. The check is `|r| ≤ ε`, so the fixture certifies exactly when ε
/// is at or above this value, and below it the kernel refuses
/// `CertifyError::ResidualExceeded { check: IsoResidual, .. }`, which
/// is the kernel being RIGHT.
///
/// Pinned BIT-EXACTLY, both directions, by
/// [`the_twisted_reattach_threshold_is_where_it_was_measured`]: a
/// regression that widens the pull-back error is loud, and so is a
/// tightening that narrows it — including a partial one that would
/// leave this constant stale in silence behind a ceiling-only guard.
/// Either way the answer is re-measure and re-state, never loosen.
const TWISTED_REATTACH_RESIDUAL: f64 = 5.784485397203693e-10;

/// The same quantity for the `d = −D` arm. **It is a different number,
/// and that is geometry rather than noise**: the pull-back translates
/// the net along `−d·n₀`, so the two signs put the base on opposite
/// sides of the saddle and the normal drift `d·(n(u,v) − n₀)` peaks at
/// a different sample. The rows therefore pin PER SIGN — a single
/// constant would have to be the max of the two, which would let the
/// smaller arm drift by the difference in silence.
const TWISTED_REATTACH_RESIDUAL_NEG: f64 = 5.7844857741573e-10;

/// The measured threshold for the sign in play.
fn residual_for(d: f64) -> f64 {
    if d < 0.0 {
        TWISTED_REATTACH_RESIDUAL_NEG
    } else {
        TWISTED_REATTACH_RESIDUAL
    }
}

/// Is the run's ε at or above the fixture's threshold for this sign?
fn above_threshold(d: f64) -> bool {
    Tol::witness().eps() >= residual_for(d)
}

/// The twisted loft with every wall converted — the curved sibling of
/// the consumer suite's straight prism.
///
/// **The below-threshold arm lives here**, once, because all four
/// converting rows share this one gate: it asserts the honest typed
/// refusal by name and returns `None`, and each row then declines to
/// assert what the kernel has correctly refused to build.
fn twisted_approx() -> Option<(Body<f64>, Vec<FaceKey>)> {
    let mut body = twisted_loft(0.05);
    match try_approx_walls(&mut body, D, FIT_TOL) {
        Ok(faces) => {
            assert!(
                above_threshold(D),
                "the re-attach certified at eps = {:e}, below the measured threshold \
                 {:e} — the pull-back error narrowed; re-measure and re-state the constant",
                Tol::witness().eps(),
                residual_for(D)
            );
            Some((body, faces))
        }
        Err(r) => {
            assert_honest_reattach_refusal(&r, D);
            None
        }
    }
}

/// The below-threshold arm's whole content: the refusal is the named,
/// typed one, at the check that owns it, and the residual that caused
/// it is the measured constant — bit for bit.
fn assert_honest_reattach_refusal(r: &ReattachRefusal, d: f64) {
    let expected = residual_for(d);
    assert!(
        !above_threshold(d),
        "the re-attach refused at eps = {:e}, at or above the measured threshold \
         {expected:e} — the pull-back error widened; re-measure and re-state the \
         constant (refusal was: {})",
        Tol::witness().eps(),
        r.error
    );
    let topo::EulerOpError::Certification { error } = &r.error else {
        panic!(
            "the re-attach must refuse through certification, got {:?}",
            r.error
        );
    };
    let geom_brep::CertifyError::ResidualExceeded { check, .. } = error else {
        panic!(
            "below the threshold the residual DEFINITELY exceeds the band, so the refusal \
             is ResidualExceeded rather than an escalation — got {error:?}"
        );
    };
    assert_eq!(
        *check,
        geom_brep::CertCheck::IsoResidual,
        "the refusing check is the iso lane's metric residual"
    );
    // The number the refusal does not carry. `ResidualExceeded` reports
    // a verdict, not a margin (unlike the #921 family's escalations,
    // whose `Indeterminate` carries an enclosure), so the fixture
    // measures the classified quantity itself and pins it here.
    assert!(
        r.max_iso_residual == expected,
        "the re-attach residual at d = {d:e} is {:e}, not its measured value \
         {expected:e} — the fixture moved; re-measure and re-state",
        r.max_iso_residual
    );
    assert!(
        r.max_iso_residual > Tol::witness().eps(),
        "a definite excess must exceed the band's zero threshold"
    );
}

/// A genuinely doubly-curved SKINNED base: bicubic interpolation-shaped
/// net over `[0,1]²` with a bump — nothing about it is planar or
/// ruled.
fn skinned_base() -> NurbsSurface<f64> {
    let kv =
        geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3)
            .unwrap();
    let mut control = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            let (u, v) = (f64::from(i) / 3.0, f64::from(j) / 3.0);
            control.push(Point3::new(
                2.0 * u,
                2.0 * v,
                0.95 + 0.4 * (u * (1.0 - u)) * (v * (1.0 - v)) * 4.0 + 0.02 * u,
            ));
        }
    }
    NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 16]).unwrap()
}

/// The twisted loft's walls really are curved — the premise the whole
/// file rests on, asserted rather than assumed.
#[test]
fn the_twisted_walls_are_not_planar() {
    let body = twisted_loft(0.05);
    let mut spline_walls = 0;
    for (_, face) in body.faces() {
        if let Some(Surface::Nurbs(p)) = body.get_surface(face.surface)
            && !p.is_placeholder()
        {
            spline_walls += 1;
            // A bilinear saddle: the mixed partial is nonzero, so the
            // normal varies. Compare corner normals.
            let n00 = {
                let j = p.ders(0.0, 0.0);
                j.du.cross(j.dv).normalize()
            };
            let n11 = {
                let j = p.ders(1.0, 1.0);
                j.du.cross(j.dv).normalize()
            };
            assert!(
                (n00 - n11).norm() > 1e-3,
                "a twisted wall's normal must vary; this wall is planar"
            );
        }
    }
    assert_eq!(spline_walls, 4);
}

/// **The threshold row.** Measures the fixture's re-attach residual
/// and pins it bit-exactly, both directions — and then CROSS-CHECKS
/// the measurement against the kernel's own certifier, which is what
/// keeps `iso_residual`'s replica of the iso arm honest: an
/// `EdgeCurve::certify` just ABOVE the measured value must succeed and
/// one just BELOW must not.
///
/// **Measured ε-INDEPENDENTLY, and that is checked rather than
/// assumed**: this row runs at every ε and asserts the same constant,
/// and the two arms reach it by different routes — below the threshold
/// from the replica's direct measurement, above it from a bisection
/// driven by the kernel's own `EdgeCurve::certify`. Both answered
/// `5.784485397203693e-10` (bits `0x3e03e016506042e7`) for `d = +D` at
/// ε = 1e-12 and ε = 1e-9, three orders apart, bit for bit. That agreement is
/// what says the quantity is construction arithmetic — a fixed number
/// of metres compared against ε — rather than something the band
/// moves, which is the property the #921 pattern needs and would not
/// have if the threshold were ε-dependent.
#[test]
fn the_twisted_reattach_threshold_is_where_it_was_measured() {
    for d in [D, -D] {
        let expected = residual_for(d);
        let mut body = twisted_loft(0.05);
        let outcome = try_approx_walls(&mut body, d, FIT_TOL);
        let measured = match &outcome {
            // Above the threshold the surgery completed, so the number
            // is recovered from the KERNEL's certifier by bisection —
            // the independent route.
            Ok(_) => {
                let mut lo = 0.0_f64;
                let mut hi = 1e-3_f64;
                for _ in 0..200 {
                    let mid = 0.5 * (lo + hi);
                    if mid <= lo || mid >= hi {
                        break;
                    }
                    if body
                        .edges()
                        .all(|(e, _)| reattach_certifies_at(&body, e, mid))
                    {
                        hi = mid;
                    } else {
                        lo = mid;
                    }
                }
                hi
            }
            Err(r) => r.max_iso_residual,
        };
        assert!(
            measured == expected,
            "the twisted fixture's re-attach residual at d = {d:e} is {measured:e}, not its \
             measured value {expected:e} — re-measure and re-state the constant"
        );
        // The cross-check that keeps the replica honest, on the body
        // the surgery actually produced. Only meaningful when the
        // surgery COMPLETED — below the threshold the edges were never
        // attached, so there is nothing to certify either side of.
        if outcome.is_ok() {
            for (e, _) in body.edges() {
                assert!(
                    reattach_certifies_at(&body, e, expected * 1.001),
                    "edge {e:?} must certify just above the measured threshold — the \
                     replica of the iso arm has drifted from the kernel's"
                );
            }
            assert!(
                body.edges()
                    .any(|(e, _)| !reattach_certifies_at(&body, e, expected * 0.999)),
                "some edge must refuse just below the measured threshold — the replica of \
                 the iso arm has drifted from the kernel's"
            );
        }
    }
}

/// **Tier 3, end to end, both signs**, on curved `Approx` walls: the
/// re-derivation runs against a genuinely curved chart and agrees.
#[test]
fn a_curved_approx_walled_body_validates_at_tier_three() {
    for d in [D, -D] {
        let mut body = twisted_loft(0.05);
        let faces = match try_approx_walls(&mut body, d, FIT_TOL) {
            Ok(faces) => faces,
            Err(r) => {
                // Below the threshold the surgery cannot build the
                // body at all; the refusal is the claim.
                assert_honest_reattach_refusal(&r, d);
                continue;
            }
        };
        assert!(
            matches!(
                body.get_surface(body.get_face(faces[0]).unwrap().surface),
                Some(Surface::Approx(_))
            ),
            "d = {d}: the wall carries the approximating surface"
        );
        assert_eq!(
            topo::validate_geometric(&body, Tol::witness()),
            Ok(()),
            "d = {d}: tier 3 on the curved Approx-walled body"
        );
    }
}

/// The never-trust red row on the CURVED chart: coarsen one wall's fit
/// behind an honest stored certificate; tier 3 names the face.
#[test]
fn a_degraded_curved_fit_goes_red_at_tier_three() {
    let Some((mut body, faces)) = twisted_approx() else {
        return;
    };
    let face = faces[0];
    let Some(Surface::Approx(live)) = body.get_surface(body.get_face(face).unwrap().surface) else {
        panic!("the wall carries an approximating surface")
    };
    let geom::SurfaceDescription::Offset { base, .. } = live.description();
    let base = Arc::clone(base);
    let honest = geom_brep::approx_offset_surface(Arc::clone(&base), D, FIT_TOL, band()).unwrap();
    let Surface::Approx(good) = &honest else {
        panic!("the door mints the variant")
    };
    let fit = good.fit();
    let mut control = fit.control().to_vec();
    let mid = control.len() / 2;
    control[mid] = control[mid] + Vec3::new(0.0, 0.0, 1e-4);
    let coarsened = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        control,
        fit.weights().to_vec(),
    )
    .unwrap();
    let planted = geom::ApproxSurface::certify(
        geom::SurfaceSpec {
            description: geom::SurfaceDescription::Offset { base, d: D },
            fit: coarsened,
            window: good.window(),
            tolerance: good.tolerance(),
        },
        |_, _, _, _| Ok::<_, geom_brep::OffsetFitError>(*good.certificate()),
    )
    .unwrap();
    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(planted))))
        .unwrap();
    let _ = topo::mint_pcurves(&mut body, Tol::witness());
    let errors = topo::validate_geometric(&body, Tol::witness())
        .expect_err("a degraded curved fit must not validate");
    assert!(
        errors.iter().any(|e| matches!(
            e,
            topo::ValidationError::ApproxCertification { face: f, .. } if *f == face
        )),
        "tier 3 must report the re-derivation failure on face {face:?}, got {errors:?}"
    );
}

/// **Tessellation** of the curved `Approx` walls through the delegate
/// path: every wall gets triangles.
#[test]
fn the_curved_approx_walls_tessellate() {
    let Some((body, faces)) = twisted_approx() else {
        return;
    };
    let mesh = mesh::tessellate(&body, 0.05, Tol::witness()).expect("the twisted body meshes");
    for face in faces {
        let patch = mesh
            .patches
            .iter()
            .find(|p| p.face == face)
            .expect("every Approx wall has a patch");
        assert!(!patch.triangles.is_empty(), "no triangles for {face:?}");
    }
}

/// **The boolean against the twisted body refuses TYPED at the edge
/// rule** — a lofted operand's wall carriers are rung-3 splines, so the
/// gate's body-scoped edge rule fires before the face rule ever sees
/// the `Approx` kind. Recorded as the refusal's true shape for THIS
/// operand (the germ-pair shape needs `Line` carriers; next row).
#[test]
fn a_boolean_against_the_twisted_approx_body_refuses_typed() {
    let Some((a, _)) = twisted_approx() else {
        return;
    };
    let e = topo::union(&a, &moved_box(), Tol::witness())
        .expect_err("a lofted Approx operand is outside the boolean envelope");
    assert!(
        matches!(e, topo::BooleanError::CurvedEdgeUnsupported { .. }),
        "expected the edge rule's typed refusal on a spline carrier, got {e}"
    );
}

/// **The germ-pair refusal from a genuinely skinned base**: a
/// doubly-curved bicubic base's certified offset surface sits on a box
/// cap (`Line` carriers, so the FACE rule decides), and the pair-scoped
/// gate refuses naming `SurfaceKind::Approx` against `Plane`.
///
/// The base here is nothing like the PR's planar pull-backs: the fit
/// has real curvature in both directions and a `d` five orders above ε.
#[test]
fn a_skinned_base_approx_face_earns_the_germ_pair_refusal() {
    let base = Arc::new(skinned_base());
    let approx = geom_brep::approx_offset_surface(Arc::clone(&base), 0.05, 1e-5, band())
        .expect("the skinned base's offset fits to 1e-5");
    let Surface::Approx(a_surf) = &approx else {
        panic!("the door mints the variant")
    };
    // The certificate is real: re-derivation agrees at rest.
    let re = geom_brep::offset_fit::recertify_approx(a_surf, 1e-5, band())
        .expect("the skinned offset re-certifies");
    assert!(re.hull_sup <= 1e-5);

    let mut a = unit_box();
    let face = top_face(&a);
    a.set_face_surface(face, FaceSurface::New(approx))
        .expect("the attach-layer door accepts a live face");
    let e = topo::union(&a, &moved_box(), Tol::witness())
        .expect_err("an Approx operand is unsupported-kind for the boolean gate");
    assert!(
        matches!(
            e,
            topo::BooleanError::CurvedPairUnsupported {
                kind: geom_brep::SurfaceKind::Approx,
                other_kind: geom_brep::SurfaceKind::Plane,
                face: f,
                ..
            } if f == face
        ),
        "expected the germ-pair refusal naming SurfaceKind::Approx on {face:?}, got {e}"
    );
}

/// TEMP diagnostic: what spline spaces are in play on the twisted wall?
#[test]
fn diag_spline_spaces() {
    let body = twisted_loft(0.05);
    for (fk, face) in body.faces() {
        if let Some(Surface::Nurbs(p)) = body.get_surface(face.surface)
            && !p.is_placeholder()
        {
            let base = Arc::new(pulled_back(p, D));
            let approx = geom_brep::approx_offset_surface(Arc::clone(&base), D, FIT_TOL, band())
                .expect("fit");
            let Surface::Approx(a) = &approx else {
                panic!()
            };
            println!(
                "face {fk:?}: wall ku={:?} kv={:?} | fit ku={:?} kv={:?}",
                p.knots_u().knots(),
                p.knots_v().knots(),
                a.fit().knots_u().knots(),
                a.fit().knots_v().knots()
            );
            break;
        }
    }
    for (ek, edge) in body.edges() {
        if let Some(c) = body
            .get_curve_geom(edge.curve)
            .and_then(CurveGeom::certified)
            && let Curve3::Nurbs(n) = c.carrier()
        {
            println!(
                "edge {ek:?}: carrier degree {} knots {:?}",
                n.knots().degree(),
                n.knots().knots()
            );
        }
    }
}
