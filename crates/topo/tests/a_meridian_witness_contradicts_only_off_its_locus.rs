//! The on-surface gate's two sides on the one in-lane class whose span
//! bound is far wider than the band: a sphere centred on a torus's
//! spine is tangent to it along a whole MERIDIAN circle
//! (`geom_brep::tangent_certificate_lane`'s docs). The sag there is
//! tenths of a metre over the whole circle, past the band even on the
//! table's finest schedule, so the gate can neither verify nor, on the
//! exact locus, contradict: every reading is definite and the refusal
//! rests on the sag alone, so the certificate does not reach the span. A
//! witness lifted definitely off the locus is a measured miss whatever
//! the sag, and contradicts.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::boolean::contact_verify::tangent_locus_relation;
use topo::contact::ContactRefusal;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// Torus `R = 1`, `r = 0.2` about z; the sphere of radius `0.2` on its
/// spine at `(1, 0, 0)`; the meridian circle there, lifted radially by
/// `lift`.
fn meridian(lift: f64) -> (Surface<f64>, Surface<f64>, Curve3<f64>) {
    let torus = Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major_radius: 1.0,
        minor_radius: 0.2,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let sphere = Surface::Sphere {
        center: Point3::new(1.0, 0.0, 0.0),
        radius: 0.2,
        axis: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let circle = Curve3::Circle {
        center: Point3::new(1.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        radius: 0.2 + lift,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    (torus, sphere, circle)
}

fn read(lift: f64) -> Result<topo::contact::ContactVerdict, ContactRefusal> {
    let (torus, sphere, circle) = meridian(lift);
    let whole = core::f64::consts::TAU;
    // The table's finest schedule: sixty-four spans to each of the
    // nine-sample schedule's.
    let sag = geom_brep::tangent_span_bounds(&torus, &sphere, &circle, 0.0, whole / 64.0)
        .unwrap()
        .residual_sag;
    assert!(
        sag > 10.0 * band().escalate(),
        "the fixture's premise: a span bound wider than the band on the finest schedule \
         ({sag:e})"
    );
    tangent_locus_relation(
        &torus,
        true,
        &sphere,
        false,
        &circle,
        0.0,
        whole,
        true,
        band(),
    )
}

#[test]
fn the_exact_meridian_is_not_certifiable_on_its_sample() {
    let out = read(0.0);
    assert!(
        matches!(out, Err(ContactRefusal::NotCertifiable { .. })),
        "on the locus, an unbounded span neither verifies nor contradicts, and no \
         reading is in band: {out:?}"
    );
}

#[test]
fn a_meridian_lifted_inside_the_zero_band_is_not_certifiable() {
    let out = read(0.5 * band().zero());
    assert!(
        matches!(out, Err(ContactRefusal::NotCertifiable { .. })),
        "a sample within ε is not a miss: {out:?}"
    );
}

#[test]
fn a_meridian_lifted_beyond_the_band_contradicts_under_the_sag() {
    let out = read(3.0 * band().escalate());
    let Err(ContactRefusal::Contradicted { diag, .. }) = out else {
        panic!("a sample definitely off the surface is a miss whatever the sag: {out:?}");
    };
    assert_eq!(
        diag.predicate,
        Some("contact_tangent_off_1"),
        "the sample is the miss"
    );
}

/// The span-too-wide refusal rests on two DEFINITE readings (the span
/// bound decided past the band, the sample decided on the surface), so
/// it is no escalation and stays off the frame's escalation log: a node
/// that builds past an unglued pair must not read to the driver as
/// undecided. Both readings stay on the verdict log.
#[test]
fn the_span_too_wide_refusal_is_not_on_the_escalation_log() {
    let bracket = geom_core::k_stats::Bracket::open();
    let out = read(0.0);
    let log = bracket.finish();
    assert!(
        matches!(out, Err(ContactRefusal::NotCertifiable { .. })),
        "{out:?}"
    );
    assert!(
        log.escalations.is_empty(),
        "no escalation on the log: {:?}",
        log.escalations
    );
    for name in ["contact_tangent_on_1", "contact_tangent_off_1"] {
        assert!(
            log.verdicts.iter().any(|v| v.predicate == name),
            "{name} decided and recorded"
        );
    }
}
