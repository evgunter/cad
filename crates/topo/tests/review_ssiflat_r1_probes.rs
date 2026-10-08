//! Independent-review probes for VERBS-SSIFLAT (PR #931), kept for
//! regression value. The PR's own row pins the MARGIN PAYLOAD
//! (`Some(Enclosure)` with both endpoints bit-exact); these rows pin
//! what the PR argued but did not pin — that the margin stays legible
//! at the seams a CONSUMER actually reads:
//!
//! 1. the Display: an interval-lane `ssi_hull_sup` escalation at the SSI
//!    door renders BOTH enclosure endpoints and never the string "NaN"
//!    — red if the margin is re-flattened to one `f64`;
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
//! `plane_nurbs_ssi`; #762's guard is out of frame by construction),
//! with the arc as a RUNG-3 carrier (`fixture::arc_chain`), certified
//! at the SSI door directly: an analytic chart's pcurve row no longer
//! carries this certificate (its image is the projected one).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::PcurveCertifyError;
use geom_brep::ssi::{SsiCertificate, SsiError, SsiOperand, TubeScale, certify_rung3};
use geom_core::Tol;
use geom_core::{Band, Point3, Real, Vec3};

use crate::fixture::arc_chain;

/// The interval lane's measured `ssi_hull_sup` bound for this fixture
/// (the arc's rung-3 chain against the sphere and the tilted plane;
/// `review_ssiflat_r2_probes` reads the same number off its own
/// escalations). Probe 3 uses it as a STRICT ceiling for
/// the f64 lane's own bound; probe 1 uses it to pick the arm.
/// **Re-measured when the C9 ring became a newtype over
/// `interval-transcendentals`' `DInterval`**
/// (`1.799_393_940_644_834_8e-12` before): the backend pads only where
/// an operation is inexact, so the bound is tighter. Probe 3's
/// strict-ceiling claim and probe 1's arm selection are unmoved.
const HULL_SUP_AT_INTERVAL: f64 = 1.016_430_181_835_071_8e-12;

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

/// The SSI door, driven directly: the arc as a RUNG-3 carrier
/// (`fixture::arc_chain`) against the (tilted plane, sphere) pair at `T`.
fn drive_ssi_door<T>() -> Result<SsiCertificate<T>, SsiError>
where
    T: geom_core::Decide + geom_core::Bounds + geom_core::CertifiedEnclosure,
{
    let band = Band::linear(Tol::witness()).unwrap();
    let (f0, f1) = ARC;
    let carrier = arc_chain::chain(&general_circle::<T>(), f0, f1);
    let ctl = carrier.control();
    let mut arm = T::zero();
    for p in ctl {
        for q in ctl {
            arm = arm.max(p.distance(*q));
        }
    }
    let (plane, sphere) = (tilted_plane::<T>(), sphere::<T>());
    certify_rung3(
        &carrier,
        None,
        &SsiOperand::Analytic(&plane),
        &SsiOperand::Analytic(&sphere),
        TubeScale::uniform(arm),
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
                terminal_sliver: false,
            },
        }
        .to_string()
    };
    let value = escalated(MarginDiag::value(1.5e-12));
    let enclosure = escalated(MarginDiag::enclosure(1.5e-12, 2.5e-12));
    let poison = escalated(MarginDiag::INVALID);
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
    let hull_sup = drive_ssi_door::<f64>()
        .expect("the f64 lane certifies at every drawn ε")
        .hull_sup;
    assert!(
        hull_sup < HULL_SUP_AT_INTERVAL,
        "the f64 hull bound ({hull_sup:e}) reached the interval lane's constant \
         ({HULL_SUP_AT_INTERVAL:e}) — the scalar-width argument behind the #925 re-scope no \
         longer holds"
    );
}

mod interval_lane {
    use geom_core::interval::Interval;

    use super::*;

    /// Probe 1: the escalation is legible. Drive the SSI door at the
    /// interval scalar; when ε sits under the hull bound the door
    /// escalates, and its Display must carry the real enclosure's BOTH
    /// endpoints and never the string "NaN".
    #[test]
    fn an_interval_escalation_is_legible_in_the_doors_display() {
        let outcome = drive_ssi_door::<Interval>();
        let eps = Tol::witness().eps();
        if eps >= HULL_SUP_AT_INTERVAL {
            // DEFINITE arm: the door certifies; nothing to read.
            outcome.expect("at or above the hull bound the route certifies");
            return;
        }
        // Under one K-th of the hull bound the margin clears the escalate
        // threshold and the door refuses definitely; this probe is about
        // an escalation's display, so it applies where one happens.
        let err = outcome.expect_err("below the hull bound the door refuses");
        if eps * Tol::witness().k() <= HULL_SUP_AT_INTERVAL {
            assert!(
                !matches!(err, SsiError::CertificateEscalated { .. }),
                "below the escalate threshold the refusal must be definite: {err:?}"
            );
            return;
        }
        assert!(format!("{err:?}").contains("ssi_hull_sup"), "{err:?}");
        let text = err.to_string();
        assert!(text.contains("too close to call"), "{text}");
        assert!(
            text.contains("enclosure ["),
            "the margin must render as an enclosure, not a value or a hole: {text}"
        );
        assert!(
            text.contains("ambiguity band ("),
            "an escalation must render the band it was judged against: {text}"
        );
        assert_eq!(
            text.matches("1.0164301818350718e-12").count(),
            2,
            "both enclosure endpoints must be visible: {text}"
        );
        assert!(
            !text.contains("NaN"),
            "the manufactured NaN must stay retired: {text}"
        );
    }
}
