//! Blinded-review probes for the VERBS-SSIFLAT unit (the fitted lane's
//! SSI margin payload, and the `m6_3_chart_completion` interval row's
//! re-scope). Each probe attacks one claim from the review brief.
//!
//! **ε posture, and why it differs from the row under review**: every
//! band here is built with [`Band::new`], not from `Tol::witness()`.
//! `PcurveCache::certify_fitted` and `topo::pcurves::validate_pcurves`
//! both take the band as an ARGUMENT, so the escalation these probes
//! pin can be driven at any process ε — they run identically under
//! `CAD_TOLERANCE_EPS` of 1e-6, 1e-9 or 1e-12. That is the property the
//! reviewed row does not have: it branches on `Tol::witness().eps()`,
//! so its escalation arm is live only in a CI configuration the
//! sampler draws occasionally.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::Surface;
use geom::{Curve3, NurbsCurve2};
use geom_brep::{Pcurve, PcurveCache};
use geom_core::{Band, Point2, Point3, Real, Vec3};

/// The tight band the reviewed row only reaches at
/// `CAD_TOLERANCE_EPS=1e-12` — here it is a value, so every probe
/// reaches it at every ε.
fn tight_band() -> Band {
    Band::new(1e-12, 1e-11).unwrap()
}

/// A loose band the same route certifies under.
fn loose_band() -> Band {
    Band::new(1e-6, 1e-5).unwrap()
}

const TILT: f64 = 0.6;

fn sphere<T: Real>(radius: f64) -> Surface<T> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: T::from_f64(radius),
        axis: Vec3::new(T::zero(), T::zero(), T::one()),
        u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
    }
}

fn tilted_plane<T: Real>() -> Surface<T> {
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(T::from_f64(TILT.sin()), T::zero(), T::from_f64(TILT.cos())),
        u_ref: Vec3::new(T::from_f64(TILT.cos()), T::zero(), T::from_f64(-TILT.sin())),
    }
}

fn general_circle<T: Real>(radius: f64) -> Curve3<T> {
    Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(T::from_f64(TILT.sin()), T::zero(), T::from_f64(TILT.cos())),
        radius: T::from_f64(radius),
        u_ref: Vec3::new(T::from_f64(TILT.cos()), T::zero(), T::from_f64(-TILT.sin())),
    }
}

/// The chart image, fitted at `f64` structure on the carrier's own
/// angle parameter — the reviewed fixture's own routine, parameterized
/// by the arc and the sphere radius so the probes can vary the span.
fn fit_image(radius: f64, t0: f64, t1: f64) -> NurbsCurve2<f64> {
    let carrier = general_circle::<f64>(radius);
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
            while u - pu > core::f64::consts::PI {
                u -= core::f64::consts::TAU;
            }
            while pu - u > core::f64::consts::PI {
                u += core::f64::consts::TAU;
            }
        }
        prev_u = Some(u);
        let v = (p.z / radius).asin();
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

/// One call at the fitted door, at an explicit band — the whole route
/// the reviewed row drives, with the tolerance a parameter.
fn certify_at<T>(
    radius: f64,
    arc: (f64, f64),
    band: Band,
) -> Result<PcurveCache<T>, geom_brep::PcurveCertifyError>
where
    T: geom_brep::PcurveFittedLane,
{
    let carrier = general_circle::<T>(radius);
    let (t0, t1) = (T::from_f64(arc.0), T::from_f64(arc.1));
    let image = Arc::new(lift2::<T>(&fit_image(radius, arc.0, arc.1)));
    let window = Pcurve::Fitted(Arc::clone(&image)).chart_box(t0, t1);
    PcurveCache::<T>::certify_fitted(
        image,
        t0,
        t1,
        &carrier,
        &sphere::<T>(radius),
        Some(&tilted_plane::<T>()),
        window,
        band,
    )
}

/// The reviewed row's own arc: a quarter turn away from the seam.
const ARC: (f64, f64) = (0.3, 0.3 + core::f64::consts::FRAC_PI_2);

/// PROBE 1 (claim C3, ε-independence of the row): the f64 sibling
/// really does certify at a 1e-12 band on its own merits — and, unlike
/// the reviewed row, this is asserted at EVERY process ε, because the
/// band is an argument rather than the run's.
#[test]
fn the_f64_route_certifies_at_a_1e_12_band_at_any_process_eps() {
    certify_at::<f64>(1.0, ARC, tight_band()).expect("the f64 lane bounds the limb under 1e-12");
}

/// PROBE 2 (claim C1/C3, the payload): at the interval scalar the same
/// route escalates at the same band, and the refusal carries a REAL
/// enclosure — not a poison and not a hole. This is the reviewed row's
/// content, made unconditional on the run's ε.
#[cfg(feature = "interval")]
#[test]
fn the_interval_route_escalates_with_a_legible_enclosure_at_any_process_eps() {
    use geom_core::interval::Interval;
    let err = certify_at::<Interval>(1.0, ARC, tight_band())
        .err()
        .expect("the interval lane escalates at a 1e-12 band");
    // AMENDED (fix pass): escalations now leave by their own door,
    // `FittedEscalated`, carrying the classifier's `Indeterminate`
    // whole — margin, band and predicate together. The probe's claim is
    // unchanged (a legible enclosure at any process ε); only the door
    // it reads it from moved.
    let geom_brep::PcurveCertifyError::FittedEscalated { cause } = err else {
        panic!("the fitted door must refuse through its escalation arm: {err:?}");
    };
    let (limb, what, margin) = (
        Option::<geom_brep::SsiLimb>::None,
        cause.predicate.unwrap_or("<unnamed>"),
        Some(cause.margin),
    );
    assert_eq!(
        limb, None,
        "an escalation names no limb, only its predicate"
    );
    assert_eq!(what, "ssi_hull_sup");
    let Some(geom_core::MarginDiag::Enclosure { lo, hi }) = margin else {
        panic!("the escalation must carry its enclosure: {margin:?}");
    };
    assert!(
        lo.is_finite() && hi.is_finite() && lo <= hi,
        "the enclosure must be two real numbers, not poison: [{lo:e}, {hi:e}]"
    );
    // END-TO-END LEGIBILITY: the numbers must survive into the text a
    // consumer actually reads, not merely into the payload.
    let shown = err.to_string();
    // AMENDED (fix pass): the escalation renders through the
    // classifier's own `IndeterminatePayload`, which words it
    // "enclosure [lo, hi] cannot be classified against the band" and
    // adds the band itself. The claim is unchanged.
    assert!(
        shown.contains("enclosure"),
        "the consumer-visible text must name the enclosure: {shown}"
    );
    assert!(
        shown.contains("zero = ") && shown.contains("escalate = "),
        "and the band it was judged against: {shown}"
    );
    assert!(
        !shown.contains("NaN"),
        "no manufactured NaN may reach the consumer: {shown}"
    );
}

/// The `ssi_hull_sup` bound this route certifies at the interval
/// scalar, for an arc of `1/div` of a quarter turn — `None` when the
/// route certifies instead.
#[cfg(feature = "interval")]
fn hull_sup_at_interval(div: f64) -> Option<f64> {
    use geom_core::interval::Interval;
    let arc = (0.3, 0.3 + core::f64::consts::FRAC_PI_2 / div);
    match certify_at::<Interval>(1.0, arc, tight_band()) {
        Ok(_) => None,
        // AMENDED (fix pass): the escalation's own door.
        Err(geom_brep::PcurveCertifyError::FittedEscalated { cause })
            if cause.predicate == Some("ssi_hull_sup") =>
        {
            match cause.margin {
                geom_core::MarginDiag::Enclosure { hi, .. } => Some(hi),
                other => panic!("unexpected margin shape at div={div}: {other:?}"),
            }
        }
        Err(e) => panic!("unexpected refusal at div={div}: {e:?}"),
    }
}

/// PROBE 3 (claim C3, the terminal-sliver argument): the PR and the
/// re-scoped row both say of this escalation that "there is nothing to
/// tighten and nothing to subdivide". `ssi_hull_sup` bounds the
/// CARRIER's incidence with the sphere — a quantity whose true value is
/// exactly zero, since the circle lies on the sphere — and the bound is
/// a control-hull bound over `SSI_CERT_SPANS` spans of the arc, whose
/// own doc (`ssi/certify.rs:164-167`) reads "More spans ⇒ tighter hulls
/// and a tighter tube".
///
/// This probe MEASURES that. It reports the bound at four span lengths
/// and asserts it is strictly span-dependent — i.e. that something IS
/// tightenable. The measurement also shows how weakly: the bound falls
/// far slower than the span, because at `T = Interval` it is dominated
/// by the width the ring data carries rather than by the span, which is
/// the honest version of the PR's claim.
#[cfg(feature = "interval")]
#[test]
fn the_interval_hull_bound_is_span_dependent() {
    let bounds: Vec<(f64, Option<f64>)> = [1.0, 2.0, 8.0, 64.0]
        .into_iter()
        .map(|d| (d, hull_sup_at_interval(d)))
        .collect();
    println!("ssi_hull_sup vs span divisor: {bounds:?}");
    let full = bounds[0].1.expect("the quarter turn escalates");
    let eighth = bounds[2].1.expect("an eighth of it still escalates");
    assert!(
        eighth < full,
        "the bound must move with the span — 'nothing to tighten' claims it cannot: \
         {bounds:?}"
    );
}

/// PROBE 4 (claim C2, the sweep's blind spot): `ssi_refusal` is not the
/// only site that manufactures a NaN margin. `ssi/certify.rs:844`
/// raises `SsiError::CertificateLimb { limb: Tube, value: f64::NAN }`
/// when the tube ladder is EMPTY — a structural refusal with no margin
/// at all, reachable on a legal body whose feature extent is under
/// `64·ε`. The PR's rewritten `ssi_refusal` turns that into
/// `Some(MarginDiag::Value(NaN))`, which is exactly the manufactured
/// poison #925 was filed as, wearing the label the classifier reserves
/// for a real f64 margin — and the text still says a limb "exceeded ε".
///
/// This row goes RED when that site is swept (it should be `None`, or a
/// distinct structural variant), which is the point: it pins the hole.
#[test]
fn a_structural_tube_refusal_reports_an_honest_typed_shape() {
    // extent ≈ the arc's control-net diameter; the ladder is empty once
    // `extent/8 < 8·ε`, i.e. extent < 64·ε = 6.4e-5 m here. The radius
    // also has to keep the arc's METRE span above ε, or the earlier
    // `pcurve_interval_meter` check answers first — 1e-5 m clears both.
    //
    // AMENDED (fix pass): this probe was written RED-by-design, pinning
    // that the structural empty-ladder case still minted
    // `CertificateLimb { limb: Tube, value: NaN }` — a structural
    // refusal wearing a limb-exceeded costume. That is fixed at the
    // SOURCE (`SsiError::TubeLadderEmpty`), so the probe now pins the
    // honest shape instead: a refusal that names the ladder, carries NO
    // magnitude because it measured nothing, and shows no NaN to a
    // consumer.
    let err = certify_at::<f64>(1.0e-5, ARC, loose_band())
        .err()
        .expect("a 10-micron arc has no certifiable uniqueness tube at a 1e-6 band");
    let geom_brep::PcurveCertifyError::FittedCertificate {
        what, magnitude, ..
    } = err
    else {
        panic!("expected the certificate arm: {err:?}");
    };
    assert!(
        magnitude.is_none(),
        "a structural refusal measured nothing and must carry no magnitude: \
         what={what:?} magnitude={magnitude:?}"
    );
    let rendered = err.to_string();
    assert!(
        !rendered.contains("NaN"),
        "no manufactured NaN may reach a consumer: {rendered}"
    );
    assert!(
        rendered.contains("ladder"),
        "the refusal must name the empty ladder as its cause: {rendered}"
    );
}

/// PROBE 5 (the E2E exercise): drive the escalation through a PUBLIC
/// `topo` door on a body of the reviewer's own authoring, and read what
/// a consumer sees. The cache is minted at a loose band and attached;
/// `validate_pcurves` then RE-DERIVES the full C2 certificate at a
/// tighter band, which is the at-rest pass a consumer runs. The
/// question is whether the margin survives that layer or is
/// re-flattened.
#[cfg(feature = "interval")]
#[test]
fn the_margin_is_legible_through_the_public_topo_door() {
    use geom_core::Tol;
    use geom_core::interval::Interval;
    use topo::Body;

    let radius = 1.0;
    let carrier = general_circle::<Interval>(radius);
    let (f0, f1) = ARC;
    let (t0, t1) = (
        <Interval as Real>::from_f64(f0),
        <Interval as Real>::from_f64(f1),
    );
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
    let image = Arc::new(lift2::<Interval>(&fit_image(radius, f0, f1)));

    let mut body = Body::<Interval>::new();
    let seed = body.mvfs(p0).unwrap();
    let sph_key = body
        .set_face_surface(
            seed.face,
            topo::FaceSurface::New(sphere::<Interval>(radius)),
        )
        .unwrap();
    let anchor = body.mvfs(p1).unwrap();
    let pl_key = body
        .set_face_surface(
            anchor.face,
            topo::FaceSurface::New(tilted_plane::<Interval>()),
        )
        .unwrap();
    let mid = <Interval as Real>::from_f64(0.5 * (f0 + f1));
    let made = body
        .mev(
            topo::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeGeometry::Intersection {
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
    let window = Pcurve::Fitted(Arc::clone(&image)).chart_box(t0, t1);
    for he in [edge.he_plus, edge.he_minus] {
        let cache = PcurveCache::<Interval>::certify_fitted(
            Arc::clone(&image),
            t0,
            t1,
            &carrier,
            &sphere::<Interval>(radius),
            Some(&tilted_plane::<Interval>()),
            window,
            loose_band(),
        )
        .expect("the cache mints at a loose band");
        body.attach_pcurve(he, cache);
    }

    // The public at-rest door, at a band tighter than the cache's.
    let findings = topo::pcurves::validate_pcurves(&body, tight_band());
    assert!(
        !findings.is_empty(),
        "re-deriving at a tighter band must refuse"
    );
    let shown: Vec<String> = findings.iter().map(ToString::to_string).collect();
    // AMENDED (fix pass): same rewording, same claim — and the band now
    // survives the public door too.
    assert!(
        shown.iter().any(|s| s.contains("enclosure")),
        "a consumer reading the public door must see the enclosure, not a flattened \
         value: {shown:?}"
    );
    assert!(
        shown.iter().any(|s| s.contains("zero = ")),
        "the band must survive the public door with the margin: {shown:?}"
    );
    assert!(
        !shown.iter().any(|s| s.contains("NaN")),
        "no layer may re-manufacture the NaN: {shown:?}"
    );
}

/// PROBE 6 (claim C3, the re-scope's SCOPE): the row under review gates
/// its escalation arm on `Tol::witness().eps() < HULL_SUP_AT_INTERVAL`
/// — one-sided. The escalation is only the outcome while the bound is
/// INSIDE the band, i.e. `band.zero() < HULL_SUP < band.escalate()`.
/// Below `HULL_SUP / K` the bound is ABOVE the band and the door
/// refuses definitely instead, which the row's arm does not admit. This
/// probe names what the door actually does there, at an explicit band
/// so it does not depend on the run's ε.
#[cfg(feature = "interval")]
#[test]
fn below_the_band_the_route_refuses_definitely_not_by_escalation() {
    use geom_core::interval::Interval;
    let below = Band::new(1e-13, 1e-12).unwrap();
    let err = certify_at::<Interval>(1.0, ARC, below)
        .err()
        .expect("the route cannot certify at a band under the hull bound");
    println!("below-band refusal: {err:?} / {err}");
    assert!(
        !matches!(
            err,
            geom_brep::PcurveCertifyError::FittedCertificate {
                limb: None,
                what: "ssi_hull_sup",
                ..
            }
        ),
        "the reviewed row's escalation arm assumes this shape at every eps below the \
         constant; it is not what the door produces here: {err:?}"
    );
}
