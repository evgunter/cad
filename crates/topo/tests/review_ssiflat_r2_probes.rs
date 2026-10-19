//! Blinded-review probes for the VERBS-SSIFLAT unit (the SSI
//! certificate's margin payload). Each probe attacks one claim from the
//! review brief. The fixture is a sphere's general circle as a rung-3
//! carrier against the (sphere, tilted plane) pair, certified at the SSI
//! door (`geom_brep::ssi::certify_rung3`); an analytic chart's pcurve
//! row no longer carries this certificate (its image is the projected
//! one), so the door is driven directly.
//!
//! **ε posture**: every band here is built with [`Band::new`], not from
//! `Tol::witness()`, so what these probes pin can be driven at any
//! process ε — they run identically under `CAD_TOLERANCE_EPS` of 1e-6,
//! 1e-9 or 1e-12.

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
fn rung3<T: geom_core::spline::SpanLocate>(radius: f64, (f0, f1): (f64, f64)) -> NurbsCurve3<T> {
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
/// route escalates at a band below its own residuals, and the refusal
/// carries a REAL enclosure — not a poison and not a hole — at any
/// process ε. The described carrier's residuals sit near `4e-15`, so a
/// `1e-15` band is inside their enclosures and limb 1 escalates first.
#[test]
fn the_interval_route_escalates_with_a_legible_enclosure_at_any_process_eps() {
    use geom_core::interval::Interval;
    let err = certify_at::<Interval>(1.0, ARC, Band::new(1e-15, 1e-14).unwrap())
        .expect_err("the interval lane escalates at a 1e-15 band");
    let SsiError::CertificateEscalated { ref cause, .. } = err else {
        panic!("the door must refuse through its escalation arm: {err:?}");
    };
    let (what, margin) = (cause.predicate.unwrap_or("<unnamed>"), Some(cause.margin));
    assert_eq!(what, "ssi_on_locus");
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

/// PROBE 3 (claim C3): `ssi_hull_sup` bounds the carrier's incidence
/// with the sphere, whose true value is exactly zero since the circle
/// lies on the sphere. Read off the described carrier, every arc from a
/// half turn (`div = 0.5`) down to 1/64 of a quarter certifies at the
/// 1e-12 band at the interval scalar, the quarter turn included.
///
/// A certificate that refined the carrier in the ring in the projective
/// form `x + (y − x)·λ` put the quarter turn's first hull at about
/// 1.02e-12, past this band. That number was the form's interval width,
/// not the refined carrier's geometry: the `f64` lane reads the same
/// rounded refinement at about 1.5e-15, and the described carrier, cut
/// in the ring, reads 3.3e-15 to 5.0e-15 on every arc here. Limb 2 now
/// subdivides its composite before it refuses, which brings even that
/// form's bound under this band (it levels off near 3.9e-13), so this
/// row does not single out the form's width any more; it holds that
/// every arc certifies. What the ring refinement encloses is held by the
/// exact-net rows in `geom_brep`'s `ssi::certify` and `edge_nurbs`
/// tests, which also see a refinement made at `f64` and lifted thin.
///
/// The full turn is not in the list: over a rung-3 chain carrier its
/// map residual at the interval scalar reaches the 1e-12 band (sample
/// 6, `pcurve_map_residual`) before the hull limb is read, so it
/// measures that check rather than this one.
#[test]
fn the_interval_route_certifies_every_arc_at_a_1e_12_band() {
    use geom_core::interval::Interval;
    for div in [0.5, 0.75, 1.0, 1.5, 2.0, 8.0, 64.0] {
        let arc = (0.3, 0.3 + core::f64::consts::FRAC_PI_2 / div);
        if let Err(e) = certify_at::<Interval>(1.0, arc, tight_band()) {
            panic!("the arc of 1/{div} of a quarter turn refused at a 1e-12 band: {e:?}");
        }
    }
}

/// PROBE 4 (claim C2): a structural tube refusal carries no margin.
/// Where the tube ladder is empty, reachable on a legal body whose
/// feature extent is under `64·ε`, the certificate measured nothing, so
/// it refuses as `SsiError::TubeLadderEmpty`: it names the ladder,
/// carries no magnitude, and shows no NaN to a consumer, where a limb
/// refusal would say a limb exceeded ε on a number nothing measured.
#[test]
fn a_structural_tube_refusal_reports_an_honest_typed_shape() {
    // extent ≈ the arc's control-net diameter; the ladder is empty once
    // `extent/8 < 8·ε`, i.e. extent < 64·ε = 6.4e-5 m here. The radius
    // also has to keep the arc's METRE span above ε, or the earlier
    // `pcurve_interval_meter` check answers first — 1e-5 m clears both.
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

/// PROBE 5 (the escalation payload at the interval scalar): at
/// `Interval` the door's escalation carries its margin as an
/// ENCLOSURE, and the consumer-visible text renders it.
///
/// **Which limb escalates is measured, not chosen.** On the quarter
/// turn limb 2's hull bound reads about 3.13e-15, under limb 1's worst
/// sample (about 4.77e-15, its upper end), because every Bézier row of
/// the composite is restricted from its own span window through the
/// monotone step (`Certification::between`). Under the convex step it
/// read 5.0e-15, just over limb 1, and this row pinned limb 2's
/// escalation there. Limb 2 stays under limb 1 over the arcs, tilts,
/// radii and translations of this pair (hull/locus 0.06 to 0.88), so
/// no band makes limb 2 the first limb in it, and the band here sits
/// between the two: limb 1 escalates, limb 2 is under the band. Limb
/// 2's own `Interval` payload is guarded where it can still be driven
/// into the band, at `hull_limb` itself
/// (`geom_brep`'s `the_hull_bound_escalates_with_an_enclosure_at_interval`).
///
/// Red if the margin is flattened to one `f64` (a value has one end,
/// and renders as one), if the escalation moves off limb 1, or if limb
/// 2's bound climbs back over limb 1's reading.
#[test]
fn the_interval_escalation_carries_an_enclosure() {
    use geom_core::interval::Interval;
    use geom_core::{Bounds, ErrorTextReading, MarginDiag};
    let cert = certify_at::<Interval>(1.0, ARC, tight_band())
        .expect("the quarter turn certifies at a 1e-12 band");
    let (locus, hull) = (cert.on_locus_max.hi(), cert.hull_sup);
    let band = Band::new(4.0e-15, 5.5e-15).unwrap();
    assert!(
        hull.hi() < band.zero() && band.zero() < locus && locus < band.escalate(),
        "the band must sit between limb 2 ([{:e}, {:e}]) and limb 1 ({locus:e})",
        hull.lo(),
        hull.hi()
    );
    let err = certify_at::<Interval>(1.0, ARC, band).expect_err("limb 1 escalates in-band");
    let SsiError::CertificateEscalated { ref cause, .. } = err else {
        panic!("the door must refuse through its escalation arm: {err:?}");
    };
    assert_eq!(cause.predicate, Some("ssi_on_locus"));
    let ErrorTextReading::Enclosure { lo, hi } =
        MarginDiag::diagnostic_f64_for_error_text(cause.margin)
    else {
        panic!(
            "limb 1's margin at Interval must read as an enclosure: {:?}",
            cause.margin
        );
    };
    assert!(
        lo <= hi && hi <= locus && hi > band.zero(),
        "the escalation must carry an in-band sample's enclosure, no higher than limb 1's \
         worst {locus:e}: [{lo:e}, {hi:e}]"
    );
    let shown = err.to_string();
    assert!(
        shown.contains("enclosure [") && !shown.contains("NaN"),
        "the consumer-visible text must render the enclosure: {shown}"
    );
}
