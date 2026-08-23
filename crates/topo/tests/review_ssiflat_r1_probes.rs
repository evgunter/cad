//! Independent-review probes for VERBS-SSIFLAT (PR #931), kept for
//! regression value. The PR's own row pins the MARGIN PAYLOAD
//! (`Some(Enclosure)` with both endpoints bit-exact); these rows pin
//! what the PR argued but did not pin — that the margin stays legible
//! at the seams a CONSUMER actually reads:
//!
//! 1. the Display chain end-to-end: an interval-lane `ssi_hull_sup`
//!    escalation, wrapped exactly as `topo::pcurves::validate_pcurves`
//!    wraps it (`PcurveMintError::Certify`), renders BOTH enclosure
//!    endpoints and never the string "NaN" — red if any layer
//!    re-flattens the margin to one `f64`;
//! 2. the four margin shapes are pairwise distinguishable in Display —
//!    a poisoned margin says so in words, an enclosure carries two
//!    endpoints, a value one, a hole none — red if two shapes ever
//!    collapse into one rendering again (the #925 conflation, one
//!    layer up);
//! 3. the f64 sibling's own merits: the same route at `f64` bounds
//!    `ssi_hull_sup` STRICTLY under the interval lane's measured
//!    constant — the "ring data widens with the scalar" claim as an
//!    inequality between the two lanes' own numbers, red if the f64
//!    bound drifts up to the interval one.
//!
//! The fixture is the M6-3 general-circle pair (a sphere and a tilted
//! plane — `SsiOperand::Analytic` both, so nothing here enters
//! `plane_nurbs_ssi`; #762's guard is out of frame by construction).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::Surface;
use geom::{Curve3, NurbsCurve2};
use geom_brep::{PcurveCache, PcurveCertifyError};
use geom_core::Tol;
use geom_core::{Band, Point2, Point3, Real, Vec3};

/// The interval lane's measured `ssi_hull_sup` bound for this fixture
/// (the PR row's constant). Probe 3 uses it as a STRICT ceiling for
/// the f64 lane's own bound; probe 1 uses it to pick the arm.
const HULL_SUP_AT_INTERVAL: f64 = 1.799_393_940_644_834_8e-12;

fn sphere<T: Real>() -> Surface<T> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: T::from_f64(1.0),
        axis: Vec3::new(T::zero(), T::zero(), T::one()),
        u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
    }
}

fn tilted_plane<T: Real>() -> Surface<T> {
    let tilt = 0.6_f64;
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(T::from_f64(tilt.sin()), T::zero(), T::from_f64(tilt.cos())),
        u_ref: Vec3::new(T::from_f64(tilt.cos()), T::zero(), T::from_f64(-tilt.sin())),
    }
}

/// The general circle: the tilted plane's great-circle section of the
/// sphere (neither polar nor meridian).
fn general_circle<T: Real>() -> Curve3<T> {
    let tilt = 0.6_f64;
    Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(T::from_f64(tilt.sin()), T::zero(), T::from_f64(tilt.cos())),
        radius: T::from_f64(1.0),
        u_ref: Vec3::new(T::from_f64(tilt.cos()), T::zero(), T::from_f64(-tilt.sin())),
    }
}

const ARC: (f64, f64) = (0.3, 0.3 + core::f64::consts::FRAC_PI_2);

/// The chart image, fitted at `f64` structure on the carrier's own
/// angle parameter (the M6-3 fixture's construction).
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
            while u - pu > core::f64::consts::PI {
                u -= core::f64::consts::TAU;
            }
            while pu - u > core::f64::consts::PI {
                u += core::f64::consts::TAU;
            }
        }
        prev_u = Some(u);
        let v = p.z.asin();
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

/// The fitted door, driven directly (the same public door the M6-3
/// fixture drives through `Body`): certify the general circle's chart
/// image against the (sphere, tilted plane) pair at `T`.
fn drive_fitted_door<T>() -> Result<PcurveCache<T>, PcurveCertifyError>
where
    T: geom_brep::PcurveFittedLane,
{
    let band = Band::linear(Tol::witness()).unwrap();
    let (f0, f1) = ARC;
    let (t0, t1) = (T::from_f64(f0), T::from_f64(f1));
    let image = Arc::new(lift2::<T>(&fit_image()));
    let window = geom_brep::Pcurve::Fitted(Arc::clone(&image)).chart_box(t0, t1);
    PcurveCache::<T>::certify_fitted(
        image,
        t0,
        t1,
        &general_circle::<T>(),
        &sphere::<T>(),
        Some(&tilted_plane::<T>()),
        window,
        band,
    )
}

/// Probe 2: the four margin shapes render pairwise distinguishably.
/// The #925 conflation was exactly two shapes (an honest enclosure and
/// true poison) collapsing into one rendering; this row goes red if
/// any two collapse again — at THIS layer or in a future "helpful"
/// Display rewrite.
#[test]
fn the_four_margin_shapes_render_pairwise_distinguishably() {
    use geom_core::{Band, Indeterminate, MarginDiag};
    // AMENDED (fix pass): the shapes now live behind two doors rather
    // than one `Option<MarginDiag>` field — escalations carry the
    // classifier's whole `Indeterminate` (margin AND band AND
    // predicate), definite/structural refusals carry a named
    // `FittedMagnitude` or nothing. The probe's claim is unchanged and
    // is if anything sharper: no two of these may render alike.
    let band = Band::new(1e-12, 1e-11).unwrap();
    let escalated = |margin: MarginDiag| {
        PcurveCertifyError::FittedEscalated {
            cause: Indeterminate {
                margin,
                band,
                predicate: Some("probe"),
            },
        }
        .to_string()
    };
    let value = escalated(MarginDiag::Value(1.5e-12));
    let enclosure = escalated(MarginDiag::Enclosure {
        lo: 1.5e-12,
        hi: 2.5e-12,
    });
    let poison = escalated(MarginDiag::Invalid);
    let hole = PcurveCertifyError::FittedCertificate {
        limb: None,
        what: "probe",
        magnitude: None,
    }
    .to_string();

    // The enclosure renders BOTH endpoints (a reader needs the width).
    assert!(
        enclosure.contains("1.5e-12") && enclosure.contains("2.5e-12"),
        "the enclosure must surface both endpoints: {enclosure}"
    );
    // Poison says so in words, and no honest shape claims poison.
    assert!(
        poison.contains("poison") || poison.contains("invalid"),
        "{poison}"
    );
    for honest in [&value, &enclosure, &hole] {
        assert!(
            !honest.contains("poison") && !honest.contains("NaN"),
            "an honest margin must not read as poison: {honest}"
        );
    }
    // Every escalation renders its BAND too — a margin without the band
    // it was judged against cannot be read.
    for e in [&value, &enclosure, &poison] {
        assert!(
            e.contains("1e-12") && e.contains("1e-11"),
            "an escalation must render the band it was judged against: {e}"
        );
    }
    // All four renderings are pairwise distinct.
    let all = [&value, &enclosure, &poison, &hole];
    for (i, a) in all.iter().enumerate() {
        for b in all.iter().skip(i + 1) {
            assert_ne!(a, b, "two margin shapes collapsed into one rendering");
        }
    }
}

/// Probe 3: the f64 sibling on its own merits. The route certifies at
/// `f64` (any CI ε), and its own `ssi_hull_sup` bound sits STRICTLY
/// under the interval lane's measured constant — the "ring data widens
/// with the scalar" claim as an inequality between the two lanes'
/// numbers rather than prose. Red if the f64 bound ever drifts up to
/// the interval one (at which point the f64 row is no longer
/// unaffected "on its own merits" and the #925 re-scope needs
/// re-arguing).
#[test]
fn the_f64_siblings_hull_bound_sits_strictly_under_the_interval_constant() {
    let cache = drive_fitted_door::<f64>().expect("the f64 lane certifies at every drawn ε");
    let ssi = cache.certificate().ssi.expect("the full C2 certificate");
    assert!(
        ssi.hull_sup < HULL_SUP_AT_INTERVAL,
        "the f64 hull bound ({:e}) reached the interval lane's constant ({HULL_SUP_AT_INTERVAL:e}) \
         — the scalar-width argument behind the #925 re-scope no longer holds",
        ssi.hull_sup
    );
}

#[cfg(feature = "interval")]
mod interval_lane {
    use geom_core::interval::Interval;

    use super::*;

    /// Probe 1: the escalation is legible END-TO-END. Drive the public
    /// fitted door at the interval scalar; when ε sits under the hull
    /// bound the door escalates, and the diagnostic a consumer reads —
    /// the Display of the refusal, wrapped exactly as
    /// `validate_pcurves` wraps it — must carry the real enclosure's
    /// BOTH endpoints and never the string "NaN". Red if `ssi_refusal`
    /// re-flattens, if the Display drops an endpoint, or if the topo
    /// wrapper substitutes its own summary.
    #[test]
    fn an_interval_escalation_is_legible_through_the_consumer_display_chain() {
        let outcome = drive_fitted_door::<Interval>();
        let eps = Tol::witness().eps();
        if eps >= HULL_SUP_AT_INTERVAL {
            // DEFINITE arm: the door certifies; nothing to read.
            outcome.expect("at or above the hull bound the route certifies");
            return;
        }
        // AMENDED (fix pass): the escalation regime is BOUNDED BELOW as
        // well as above. Under one K-th of the hull bound the margin
        // clears the escalate threshold and the door refuses
        // DEFINITELY — and an earlier check may refuse before it (at
        // ε = 1e-13, `pcurve_map_residual`). This probe is about the
        // legibility of an ESCALATION's display, so it applies where an
        // escalation is what happens; the definite regime is the
        // re-scoped row's third arm.
        let err = outcome.expect_err("below the hull bound the fitted door refuses");
        if eps * Tol::witness().k() <= HULL_SUP_AT_INTERVAL {
            assert!(
                !matches!(err, PcurveCertifyError::FittedEscalated { .. }),
                "below the escalate threshold the refusal must be definite, not an \
                 escalation: {err:?}"
            );
            return;
        }
        // The refusal itself, then the refusal as the tier-3 pass
        // reports it (the consumer's actual seam).
        let direct = err.to_string();
        let wrapped = topo::pcurves::PcurveMintError::Certify {
            half_edge: topo::HalfEdgeKey::default(),
            error: err,
        }
        .to_string();
        for text in [&direct, &wrapped] {
            assert!(
                text.contains("ssi_hull_sup"),
                "the escalating predicate's name is the actionable part: {text}"
            );
            // AMENDED (fix pass): escalations now render through
            // `IndeterminatePayload`, the classifier's own renderer, so
            // the wording is "enclosure [lo, hi] cannot be classified
            // against the band" rather than this lane's former
            // "offending enclosure [". The probe's claim — the margin
            // renders as an ENCLOSURE with both endpoints, never as a
            // value or a hole — is unchanged, and the payload
            // additionally carries the band.
            assert!(
                text.contains("enclosure ["),
                "the margin must render as an enclosure, not a value or a hole: {text}"
            );
            assert!(
                text.contains("zero = ") && text.contains("escalate = "),
                "an escalation must render the band it was judged against: {text}"
            );
            // Both endpoints of the degenerate enclosure — rendered
            // twice, since lo == hi.
            assert_eq!(
                text.matches("1.7993939406448348e-12").count(),
                2,
                "both enclosure endpoints must be visible: {text}"
            );
            assert!(
                !text.contains("NaN"),
                "the manufactured NaN must stay retired: {text}"
            );
        }
    }
}
