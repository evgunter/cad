//! Blinded-review probes for the VERBS-SSIFLAT unit (the SSI
//! certificate's margin payload). Each probe attacks one claim from the
//! review brief. The fixture is a sphere's general circle as a rung-3
//! carrier against the (sphere, tilted plane) pair, certified at the SSI
//! door (`geom_brep::ssi::certify_rung3`); an analytic chart's pcurve
//! row no longer carries this certificate (its image is the projected
//! one), so the door is driven directly.
//!
//! **ε posture**: every band here is built with [`Band::new`], not from
//! `Tol::witness()`, so the escalation these probes pin can be driven at
//! any process ε — they run identically under `CAD_TOLERANCE_EPS` of
//! 1e-6, 1e-9 or 1e-12.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, NurbsCurve3, Surface};
use geom_brep::ssi::{SsiCertificate, SsiError, SsiOperand, certify_rung3};
use geom_core::{Band, Point3, Real, Vec3};

use crate::fixture::arc_chain;

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

/// The arc as a RUNG-3 carrier at `T` (`fixture::arc_chain`): these
/// probes are about the SSI certificate's payloads, which only a fitted
/// carrier runs.
fn rung3<T: Real>(radius: f64, (f0, f1): (f64, f64)) -> NurbsCurve3<T> {
    arc_chain::chain(&general_circle::<T>(radius), f0, f1)
}

/// The control net's diameter: the tube ladder's arm, as the fitted
/// lane read it.
fn diameter<T: Real>(c: &NurbsCurve3<T>) -> T {
    let ctl = c.control();
    let mut far = T::zero();
    for p in ctl {
        for q in ctl {
            far = far.max(p.distance(*q));
        }
    }
    far
}

/// One call at the SSI door, at an explicit band — the whole
/// certificate, with the tolerance a parameter.
fn certify_at<T>(radius: f64, arc: (f64, f64), band: Band) -> Result<SsiCertificate<T>, SsiError>
where
    T: geom_core::Decide + geom_core::Bounds + geom_core::CertifiedEnclosure,
{
    let carrier = rung3::<T>(radius, arc);
    let (plane, sphere) = (tilted_plane::<T>(), sphere::<T>(radius));
    certify_rung3(
        &carrier,
        None,
        &SsiOperand::Analytic(&plane),
        &SsiOperand::Analytic(&sphere),
        diameter(&carrier),
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
/// enclosure — not a poison and not a hole — at any process ε.
#[test]
fn the_interval_route_escalates_with_a_legible_enclosure_at_any_process_eps() {
    use geom_core::interval::Interval;
    let err = certify_at::<Interval>(1.0, ARC, tight_band())
        .expect_err("the interval lane escalates at a 1e-12 band");
    let SsiError::CertificateEscalated { ref cause, .. } = err else {
        panic!("the door must refuse through its escalation arm: {err:?}");
    };
    let (what, margin) = (cause.predicate.unwrap_or("<unnamed>"), Some(cause.margin));
    assert_eq!(what, "ssi_hull_sup");
    let Some(geom_core::ErrorTextReading::Enclosure { lo, hi }) =
        margin.map(geom_core::MarginDiag::diagnostic_f64_for_error_text)
    else {
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
        shown.contains("ambiguity band ("),
        "and the band it was judged against: {shown}"
    );
    assert!(
        !shown.contains("NaN"),
        "no manufactured NaN may reach the consumer: {shown}"
    );
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
/// This probe MEASURES that, over eight span lengths from a full turn
/// (`div = 0.25`) down to 1/64 of a quarter (`div = 64`), and asserts
/// the escalation is a property of THIS SPAN rather than a floor:
///
/// - the quarter turn escalates, at about 1.02e-12;
/// - a SHORTER arc certifies — so there is something to subdivide;
/// - a LONGER arc certifies too — so the escalation is not a monotone
///   floor the bound is pressed against, it is where this one span
///   happens to sit.
///
/// Either of the last two alone falsifies "there is nothing to tighten
/// and nothing to subdivide"; together they say the quarter turn is
/// singular among its neighbours in both directions, which is a
/// stronger and more useful statement than a monotone trend would be.
///
/// **This row was re-pointed when `insert_once_ring` took the convex
/// insertion form, because its previous claim went vacuous.** Under the
/// lerp form the bound was dominated by the width the ring data carried
/// rather than by the span, so four spans sat within 1.6x of each other
/// (1.80e-12, 1.30e-12, 1.14e-12, 1.14e-12) and NONE certified; the row
/// asserted a strict decrease across them. The convex form took that
/// floor away, every arc but the quarter turn now certifies, and a
/// "strictly smaller bound" loop over them compares nothing — every
/// iteration passes on `None`. A row named for span dependence that
/// measures no span dependence is the defect, not the re-baseline.
///
/// The full turn is not in the list: over a rung-3 chain carrier its
/// map residual at the interval scalar reaches the 1e-12 band (sample
/// 6, `pcurve_map_residual`) before the hull limb is read, so it
/// measures that check rather than this one.
#[test]
fn the_interval_hull_bound_is_span_dependent() {
    /// The `ssi_hull_sup` bound this route certifies at the interval
    /// scalar, for an arc of `1/div` of a quarter turn — `None` when the
    /// route certifies instead.
    fn hull_sup_at_interval(div: f64) -> Option<f64> {
        use geom_core::interval::Interval;
        let arc = (0.3, 0.3 + core::f64::consts::FRAC_PI_2 / div);
        match certify_at::<Interval>(1.0, arc, tight_band()) {
            Ok(_) => None,
            Err(SsiError::CertificateEscalated { cause, .. })
                if cause.predicate == Some("ssi_hull_sup") =>
            {
                match cause.margin.diagnostic_f64_for_error_text() {
                    geom_core::ErrorTextReading::Enclosure { hi, .. } => Some(hi),
                    other => panic!("unexpected margin shape at div={div}: {other:?}"),
                }
            }
            Err(e) => panic!("unexpected refusal at div={div}: {e:?}"),
        }
    }
    let bounds: Vec<(f64, Option<f64>)> = [0.5, 0.75, 1.0, 1.5, 2.0, 8.0, 64.0]
        .into_iter()
        .map(|d| (d, hull_sup_at_interval(d)))
        .collect();
    println!("ssi_hull_sup vs span divisor: {bounds:?}");
    let quarter = bounds
        .iter()
        .find(|(d, _)| *d == 1.0)
        .and_then(|(_, b)| *b)
        .unwrap_or_else(|| panic!("the quarter turn no longer escalates: {bounds:?}"));
    assert!(
        bounds.iter().any(|(d, b)| *d > 1.0 && b.is_none()),
        "no arc SHORTER than the quarter turn certifies, so there is nothing to subdivide \
         after all — the quarter turn escalates at {quarter:e}: {bounds:?}"
    );
    assert!(
        bounds.iter().any(|(d, b)| *d < 1.0 && b.is_none()),
        "no arc LONGER than the quarter turn certifies, so the escalation reads as a floor \
         the bound is pressed against rather than as this span's own — the quarter turn \
         escalates at {quarter:e}: {bounds:?}"
    );
}

/// PROBE 4 (claim C2, the sweep's blind spot): `ssi_refusal` is not the
/// only site that manufactures a NaN margin. `ssi/certify.rs:844`
/// raises `SsiError::CertificateLimb { limb: Tube, value: f64::NAN }`
/// when the tube ladder is EMPTY — a structural refusal with no margin
/// at all, reachable on a legal body whose feature extent is under
/// `64·ε`. The PR's rewritten `ssi_refusal` turns that into
/// `Some(MarginKind::Value(NaN))`, which is exactly the manufactured
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
        .expect_err("a 10-micron arc has no certifiable uniqueness tube at a 1e-6 band");
    assert!(
        matches!(err, SsiError::TubeLadderEmpty { .. }),
        "a structural refusal is the empty ladder's, measuring nothing: {err:?}"
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
