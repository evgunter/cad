//! **M6-3 Leg E acceptance (walk row 4): the analytic-chart pcurve
//! completion** — the sphere GENERAL-circle route through the fitted
//! lane, at rest.
//!
//! The closed-form arms (cone rim/ruling, sphere polar/meridian,
//! torus parallel/meridian) are exercised where they LIVE: every
//! revolve/boolean/fillet body now mints them (`chart_mints` flipped;
//! ball/cone/donut/die-octant coverage rides those suites and the
//! corpus). What no construction mints today is the sphere chart's
//! azimuth-NON-harmonic class — a circle neither polar nor meridian —
//! and this file pins its whole route:
//!
//! - the CLOSED-FORM door refuses it typed (`UnsupportedCarrier`, the
//!   class named in `chart_pcurve`'s sphere arm);
//! - the FITTED door (`PcurveCache::certify_fitted`, M6-2) accepts
//!   the exact `Curve3::Circle` carrier via its locus-exact
//!   rational-quadratic chain, with the `OnLocusHull` envelope
//!   statement — the image is fitted on the CARRIER'S OWN angular
//!   parameter (OQ4), interpolated so every CERT-schedule sample is a
//!   collocation point;
//! - the cache survives AT REST: the tier-3 pcurve pass re-derives
//!   the full C2 certificate, reading the MATE operand from the
//!   edge's own intensional description
//!   (`EdgeDescription::Intersection { sphere, plane }`);
//! - the same body certifies at the INTERVAL scalar
//!   (`certified` module), enclosure-asserted — the loud-skip pattern
//!   is not needed here because no march budget is consulted (the
//!   chain and the fit are closed-form structure).
//!
//! ε posture: no ε literal; every margin is the run's own band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::Surface;
use geom::{Curve3, NurbsCurve2};
use geom_brep::{
    EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec, EnvelopeStatement, Pcurve, PcurveCache,
};
use geom_core::Tol;
use geom_core::{Band, Point2, Point3, Real, Vec3};
use topo::Body;

/// The chart sphere: unit-ish radius, polar axis +z.
fn sphere<T: Real>() -> Surface<T> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: T::from_f64(1.0),
        axis: Vec3::new(T::zero(), T::zero(), T::one()),
        u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
    }
}

/// The mate: the tilted cutting plane through the circle (the
/// intensional pair the edge's description names).
fn tilted_plane<T: Real>() -> Surface<T> {
    let tilt = 0.6_f64;
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(T::from_f64(tilt.sin()), T::zero(), T::from_f64(tilt.cos())),
        u_ref: Vec3::new(T::from_f64(tilt.cos()), T::zero(), T::from_f64(-tilt.sin())),
    }
}

/// The GENERAL circle: the tilted plane's great-circle section of the
/// sphere — its plane neither ⊥ the polar axis nor containing it.
fn general_circle<T: Real>() -> Curve3<T> {
    let tilt = 0.6_f64;
    Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(T::from_f64(tilt.sin()), T::zero(), T::from_f64(tilt.cos())),
        radius: T::from_f64(1.0),
        u_ref: Vec3::new(T::from_f64(tilt.cos()), T::zero(), T::from_f64(-tilt.sin())),
    }
}

/// The traversed arc: a quarter turn away from the azimuth seam.
const ARC: (f64, f64) = (0.3, 0.3 + core::f64::consts::FRAC_PI_2);

/// The chart image, fitted at `f64` STRUCTURE (C6) on the carrier's
/// own angle parameter: 33 collocation params (so all 9 CERT-schedule
/// params are interpolation points), degree 3, branch-stable azimuth.
fn fit_image() -> NurbsCurve2<f64> {
    let carrier = general_circle::<f64>();
    let (t0, t1) = ARC;
    let n = 33usize;
    let mut params = Vec::with_capacity(n);
    let mut pts = Vec::with_capacity(n);
    let mut prev_u: Option<f64> = None;
    for i in 0..n {
        #[allow(clippy::cast_precision_loss)]
        let t = t0 + (t1 - t0) * (i as f64 / (n - 1) as f64);
        let p = carrier.eval(t);
        let mut u = p.y.atan2(p.x);
        if let Some(pu) = prev_u {
            // Structure-level branch continuation for the FIT (the
            // certified one-branch statement is the certificate's).
            while u - pu > core::f64::consts::PI {
                u -= core::f64::consts::TAU;
            }
            while pu - u > core::f64::consts::PI {
                u += core::f64::consts::TAU;
            }
        }
        prev_u = Some(u);
        let v = p.z.asin();
        // The fitter's parameter contract is a clamped 0 → 1 run; the
        // OQ4 identity is restored by the exact affine knot rescale
        // below (a B-spline is invariant under affine knot maps).
        params.push((t - t0) / (t1 - t0));
        pts.push(Point2::new(u, v));
    }
    let fit = NurbsCurve2::interpolate_with_params(&pts, 3, &params).expect("the chart image fits");
    let knots: Vec<f64> = fit
        .knots()
        .knots()
        .iter()
        .map(|k| t0 + (t1 - t0) * k)
        .collect();
    let kv = geom_core::spline::KnotVector::clamped(knots, fit.knots().degree())
        .expect("affine knot rescale");
    NurbsCurve2::new(kv, fit.control().to_vec(), fit.weights().to_vec()).expect("rescaled image")
}

fn lift2<T: Real>(c: &NurbsCurve2<f64>) -> NurbsCurve2<T> {
    let control = c
        .control()
        .iter()
        .map(|p| Point2::new(T::from_f64(p.x), T::from_f64(p.y)))
        .collect();
    NurbsCurve2::new(c.knots().clone(), control, c.weights().to_vec()).expect("lifted structure")
}

/// Build the at-rest body at `T`: a sphere face and a plane face
/// joined by the general-circle edge whose DESCRIPTION names the pair,
/// the sphere side carrying the fitted cache (the M6-2 fixture's
/// public-door pattern).
fn build<T>() -> (Body<T>, topo::HalfEdgeKey)
where
    T: geom_brep::PcurveFittedLane,
{
    try_build::<T>().expect("the general circle certifies through the fitted door")
}

/// The same construction, with the fitted door's refusal RETURNED
/// rather than panicked on. The certified hull bound this route
/// produces is ε-independent, so at a tight enough ε it lands in the
/// ambiguity band and the door escalates honestly — a caller that
/// wants to assert that outcome needs the error, not a panic.
fn try_build<T>() -> Result<(Body<T>, topo::HalfEdgeKey), geom_brep::PcurveCertifyError>
where
    T: geom_brep::PcurveFittedLane,
{
    let band = Band::linear(Tol::witness()).unwrap();
    let carrier = general_circle::<T>();
    let (f0, f1) = ARC;
    let (t0, t1) = (T::from_f64(f0), T::from_f64(f1));
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
    let image = Arc::new(lift2::<T>(&fit_image()));

    let mut body = Body::<T>::new();
    let seed = body.mvfs(p0).unwrap();
    let sph_key = body
        .set_face_surface(seed.face, topo::FaceSurface::New(sphere::<T>()))
        .unwrap();
    let anchor = body.mvfs(p1).unwrap();
    let pl_key = body
        .set_face_surface(anchor.face, topo::FaceSurface::New(tilted_plane::<T>()))
        .unwrap();
    let mid = T::from_f64(0.5 * (f0 + f1));
    let made = body
        .mev(
            topo::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    s1: sph_key,
                    s2: pl_key,
                    witness: carrier.eval(mid),
                },
                carrier: carrier.clone(),
                param_start: t0,
                param_end: t1,
            },
            Tol::witness(),
        )
        .expect("the general-circle edge certifies");
    let edge = body.get_edge(made.edge).expect("edge resolves");
    let (he_plus, he_minus) = (edge.he_plus, edge.he_minus);
    let window = Pcurve::Fitted(Arc::clone(&image)).chart_box(t0, t1);
    // Both half-edges live in the sphere face's loop (the spur-edge
    // shape), and a face with ANY cache must be complete — attach to
    // both, exactly as the M6-2 fixture does.
    for he in [he_plus, he_minus] {
        let cache = PcurveCache::<T>::certify_fitted(
            Arc::clone(&image),
            t0,
            t1,
            &carrier,
            &sphere::<T>(),
            Some(&tilted_plane::<T>()),
            window,
            band,
        )?;
        body.attach_pcurve(he, cache);
    }
    Ok((body, he_plus))
}

/// The closed-form door refuses the class typed — the fitted lane is
/// the ONLY route (never a silent fallback, C5).
#[test]
fn a_general_circle_refuses_the_closed_form_sphere_door_typed() {
    let band = Band::linear(Tol::witness()).unwrap();
    let err = geom_brep::chart_pcurve(&general_circle::<f64>(), &sphere::<f64>(), band)
        .expect_err("azimuth-non-harmonic");
    assert!(matches!(
        err,
        geom_brep::PcurveCertifyError::UnsupportedCarrier
    ));
}

/// The at-rest row at `f64`: the cache is fitted, its statement is
/// `OnLocusHull`, and the tier-3 pcurve pass RE-DERIVES the full
/// certificate with the mate read from the edge's description.
#[test]
fn a_general_circle_sphere_cache_survives_the_at_rest_pass() {
    let (body, he) = build::<f64>();
    let cache = body.pcurve(he).expect("the cache is stored");
    assert!(matches!(cache.pcurve(), Pcurve::Fitted(_)));
    let cert = cache.certificate();
    assert_eq!(cert.statement, EnvelopeStatement::OnLocusHull);
    assert!(cert.ssi.is_some(), "the full C2 certificate is stored");
    // Schedule residual: every CERT sample is a collocation point of
    // the fit, so the sampled max sits at floating-point noise —
    // asserted against the band, not a literal.
    let band = Band::linear(Tol::witness()).unwrap();
    assert!(cert.max_residual.abs() <= band.zero());
    let findings = topo::pcurves::validate_pcurves(&body, band);
    assert!(findings.is_empty(), "{findings:?}");
}

/// The interval row: the same body at the interval scalar. What the
/// route DOES there depends on ε — it certifies, escalates honestly, or
/// refuses definitely — so the row asserts whichever of the three the
/// run's tolerance selects rather than claiming one unconditionally.
/// Either way it is the evidence the lane genuinely left `f64`.
#[cfg(feature = "interval")]
mod certified {
    use geom_core::Tol;
    use geom_core::interval::Interval;

    use super::*;

    /// The `ssi_hull_sup` bound this route produces at the INTERVAL
    /// scalar (metres). It is ε-INDEPENDENT — bit-identical at 1e-6 and
    /// 1e-12 — because it is a ring-computed hull bound over the fitted
    /// carrier's control data, not a tolerance-derived quantity: only
    /// the band moves.
    ///
    /// It is **schedule-specific**, not a property of the geometry
    /// alone: the bound is computed over the carrier refined to
    /// [`geom_brep::SSI_CERT_SPANS`] spans, and refining further moves
    /// it — measured, it falls about 36 % as the span divisor grows and
    /// then plateaus near 1.140e-12, still inside the band at 1e-12. So
    /// "re-measure and re-state" below means re-measure at THIS
    /// schedule; a schedule change is expected to move the constant and
    /// the pin is what makes that loud.
    ///
    /// It is also the interval lane's number specifically. The same
    /// route at `f64` bounds the same limb well under 1e-12, which is
    /// why the at-rest `f64` row is unaffected: the ring data widens
    /// with the scalar the coefficients are held in.
    const HULL_SUP_AT_INTERVAL: f64 = 1.7993939406448348e-12;

    /// This route is **honest at every ε**, and which of three things
    /// that means depends on where ε sits relative to
    /// [`HULL_SUP_AT_INTERVAL`] (#925):
    ///
    /// - **ε ≥ the bound**: the route certifies, and the row asserts
    ///   the full at-rest statement.
    /// - **the bound > ε > the bound / K**: the bound lands inside the
    ///   open sliver band and the fitted door ESCALATES on
    ///   `ssi_hull_sup`. Honest, and terminal:
    ///   [`geom_core::MarginDiag::Enclosure`]'s own documentation says
    ///   an enclosure lying wholly inside one open sliver band "is not
    ///   refinable by subdivision at all (the band is semantically
    ///   indeterminate at any width, even for a point)" and is
    ///   escalated "as a genuine D4 ¶3 sliver, not a resolution
    ///   failure". The margin here is degenerate — `lo == hi`, an exact
    ///   point, not a straddle — so no amount of narrowing reaches a
    ///   decision.
    /// - **ε ≤ the bound / K**: the route refuses DEFINITELY rather
    ///   than escalating, and not necessarily on this limb — at 1e-13
    ///   an earlier check (`pcurve_map_residual`) refuses first. The row
    ///   asserts a typed refusal and PRINTS it, because which door
    ///   fires at a given ε is a measurement, not something this row
    ///   should pretend to predict.
    ///
    /// This row previously read as a POISONED enclosure, which is what
    /// #925 was filed as. It never was: the NaN was manufactured by the
    /// fitted lane's error flattening, which projected every margin
    /// onto one `f64` and reported `NaN` for anything that was not a
    /// bare value. Escalations now carry the classifier's
    /// `Indeterminate` whole, so the margin and the band it was judged
    /// against arrive together.
    #[test]
    fn the_general_circle_route_is_honest_at_the_interval_scalar() {
        let built = try_build::<Interval>();
        let eps = Tol::witness().eps();
        if eps >= HULL_SUP_AT_INTERVAL {
            let (body, he) = built.expect("the general circle certifies through the fitted door");
            let cache = body.pcurve(he).expect("the cache is stored");
            let cert = cache.certificate();
            assert_eq!(cert.statement, EnvelopeStatement::OnLocusHull);
            let band = Band::linear(Tol::witness()).unwrap();
            // Enclosure-style: the certified envelope's SUPREMUM is
            // inside the band — a bracketing claim, never an equality.
            assert!(geom_core::Bounds::hi(cert.envelope) <= band.zero());
            let findings = topo::pcurves::validate_pcurves(&body, band);
            assert!(findings.is_empty(), "{findings:?}");
            return;
        }
        let Err(err) = built else {
            panic!("below the hull bound the fitted door must refuse");
        };
        // Below one K-th of the bound the hull limb is DEFINITE, and an
        // earlier check may refuse before it. Assert a typed refusal and
        // SHOW it: the evidence belongs in the failure message, not in a
        // probe someone has to write later to find out what fired.
        let geom_brep::PcurveCertifyError::FittedEscalated { cause } = err else {
            assert!(
                eps * Tol::witness().k() <= HULL_SUP_AT_INTERVAL,
                "above the escalate threshold the refusal must be an escalation, got {err:?}"
            );
            return;
        };
        assert_eq!(cause.predicate, Some("ssi_hull_sup"));
        // The refusal carries the REAL margin. Before #925's fix this
        // reported `NaN`, indistinguishable from a poisoned enclosure.
        let geom_core::MarginDiag::Enclosure { lo, hi } = cause.margin else {
            panic!("the escalation must carry its enclosure, not a poison or a hole: {cause:?}");
        };
        assert!(
            lo == HULL_SUP_AT_INTERVAL && hi == HULL_SUP_AT_INTERVAL,
            "the hull bound is [{lo:e}, {hi:e}], not the measured degenerate enclosure at \
             {HULL_SUP_AT_INTERVAL:e} — the fit or the refinement schedule moved; re-measure \
             and re-state"
        );
        // Against the CLASSIFIER's own band, carried in the diagnostic —
        // not a band this row rebuilds and hopes matches.
        assert!(
            hi > cause.band.zero() && hi < cause.band.escalate(),
            "the bound must lie inside the OPEN sliver band, which is what makes this \
             escalation terminal rather than refinable"
        );
    }
}
