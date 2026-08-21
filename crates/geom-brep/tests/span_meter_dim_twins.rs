//! Dimensional pins for the NURBS carrier's metric rate — the one
//! quantity `certify`'s span check and the pcurve lanes' interval
//! checks both meter with.
//!
//! D4's ε semantics require every `classify` comparand to be a LENGTH
//! (max deviation from specified geometry at a single point). A
//! carrier's speed lower bound is metres PER PARAMETER UNIT, not
//! metres, so gating it directly compared a rate against a metre band:
//! the verdict then depended on how the net happened to be
//! parametrized, which is not a property of the geometry at all. The
//! gated quantity is now the length that rate subtends over the net's
//! own knot domain — a lower bound on its arc length.
//!
//! Three claims, one per row below:
//!
//! - the margin IS the carrier's arc length (measured, not asserted
//!   by construction);
//! - it scales LINEARLY with the model (mm twin vs metre twin, ratio
//!   exactly 1000) — the D4 point-deviation claim;
//! - it is REPARAMETRIZATION-INVARIANT: the same locus authored over
//!   a doubled knot domain (`t → 2t`) answers the identical margin,
//!   while the bare rate answers half. That pair is what separates a
//!   length from a rate; the scale twin alone cannot, because a rate
//!   over a fixed domain scales linearly too.
//!
//! The fourth row is the weakening attack the metering closes: a
//! carrier whose true arc length lies INSIDE the band while its bare
//! rate is decisively positive. The old comparand certified it; the
//! length escalates it, and the row is sized in units of the run's own
//! ε so its posture is the same at every ε in the hosted matrix.
//!
//! **NO TEST IN THIS FILE IS EXECUTED BY CI.** The probe suites CI runs are
//! rostered in `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run
//! by `scripts/k_probe_sweep.sh`; this one is on neither list, so nothing
//! here can go red on a merge and its assertions are evidence for a reader
//! rather than a gate. By hand:
//! `cargo test -p geom-brep --features probe --test all -- span_meter_dim_twins::`.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom::{Curve3, NurbsCurve3};
use geom_brep::keys::SurfaceKey;
use geom_brep::{CertifyError, EdgeCurve, EdgeCurveSpec, EdgeGeometry};
use geom_core::Tol;
use geom_core::k_stats::{self, Probe, SampleOutcome};
use geom_core::spline::KnotVector;
use geom_core::{Band, MarginDiag, Point3, Sign, Tolerance, Vec3};
use slotmap::SlotMap;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's linear band")
}

fn p(x: f64, y: f64, z: f64) -> Point3<Probe> {
    Point3::new(Probe(x), Probe(y), Probe(z))
}

fn v3(x: f64, y: f64, z: f64) -> Vec3<Probe> {
    Vec3::new(Probe(x), Probe(y), Probe(z))
}

/// The `x = 0` and `y = 0` planes: two analytic charts whose
/// intersection locus is exactly the `z` axis, which is the class a
/// NURBS carrier certifies under (`EdgeGeometry::Intersection`).
fn axis_planes() -> (Surface<Probe>, Surface<Probe>) {
    (
        Surface::Plane {
            origin: p(0.0, 0.0, 0.0),
            normal: v3(1.0, 0.0, 0.0),
            u_ref: v3(0.0, 1.0, 0.0),
        },
        Surface::Plane {
            origin: p(0.0, 0.0, 0.0),
            normal: v3(0.0, 1.0, 0.0),
            u_ref: v3(1.0, 0.0, 0.0),
        },
    )
}

/// The `z`-axis segment `0 → length`, authored as a degree-1 net over
/// the knot domain `[0, domain]`. The locus depends only on `length`;
/// `domain` is pure parametrization, so the pair `(length, domain)`
/// is exactly the scale/reparametrization knob these rows turn. The
/// net's speed lower bound is `length / domain`.
fn axis_segment(length: f64, domain: f64) -> Curve3<Probe> {
    let k = KnotVector::clamped(vec![0.0, 0.0, domain, domain], 1).expect("carrier knots");
    let n = NurbsCurve3::new(
        k,
        vec![p(0.0, 0.0, 0.0), p(0.0, 0.0, length)],
        vec![1.0, 1.0],
    )
    .expect("the carrier builds");
    Curve3::Nurbs(std::sync::Arc::new(n))
}

/// Certifies the segment against the axis planes and returns the
/// `nurbs_span_meter` sample (outcome and margin) alongside the
/// certification's own verdict. Exactly one such sample fires per
/// certification — the row asserts that too.
fn span_meter_sample(
    length: f64,
    domain: f64,
) -> ((SampleOutcome, f64), Result<EdgeCurve<Probe>, CertifyError>) {
    let (px, py) = axis_planes();
    let mut arena: SlotMap<SurfaceKey, Surface<Probe>> = SlotMap::with_key();
    let s1 = arena.insert(px);
    let s2 = arena.insert(py);
    let carrier = axis_segment(length, domain);
    let witness = carrier.eval(Probe(domain * 0.5));
    let (start, end) = (carrier.eval(Probe(0.0)), carrier.eval(Probe(domain)));
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::Intersection { s1, s2, witness },
        carrier,
        param_start: Probe(0.0),
        param_end: Probe(domain),
    };
    k_stats::start_recording();
    let got = EdgeCurve::certify(spec, start, end, |k| arena.get(k).cloned(), band());
    let samples = k_stats::take_samples();
    let mut hits: Vec<(SampleOutcome, f64)> = samples
        .iter()
        .filter(|s| s.predicate == "nurbs_span_meter")
        .map(|s| (s.outcome, s.margin))
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "the span meter is gated exactly once per certification: {hits:?}"
    );
    (hits.pop().expect("one sample"), got)
}

/// The margin IS the carrier's arc length — a metre, measured.
#[test]
fn the_span_meter_margin_is_the_carrier_arc_length() {
    let ((outcome, margin), got) = span_meter_sample(1.0, 1.0);
    got.expect("a unit segment on the axis certifies");
    assert!(
        matches!(outcome, SampleOutcome::Definite(Sign::Positive)),
        "a 1 m carrier is decisively positive against every supported band: {outcome:?}"
    );
    assert!(
        (margin - 1.0).abs() < 1e-15,
        "margin {margin:e} is not the 1 m arc length"
    );
}

/// The D4 point-deviation claim: the margin scales LINEARLY with the
/// model. A scale-quadratic comparand answers 1e6 here.
#[test]
fn the_span_meter_margin_scales_linearly_with_the_model() {
    let ((_, mm), a) = span_meter_sample(1e-3, 1.0);
    let ((_, m), b) = span_meter_sample(1.0, 1.0);
    a.expect("mm twin certifies");
    b.expect("metre twin certifies");
    let ratio = m / mm;
    assert!(
        (ratio / 1e3 - 1.0).abs() < 1e-12,
        "margin must scale linearly with the model: ratio {ratio:e}"
    );
}

/// **The fold-in's own proof.** Same locus, doubled knot domain: the
/// metered length is invariant, while the bare rate the gate used to
/// classify halves. A rename could not produce this row.
#[test]
fn the_span_meter_margin_is_reparametrization_invariant() {
    let ((o1, once), a) = span_meter_sample(1.0, 1.0);
    let ((o2, twice), b) = span_meter_sample(1.0, 2.0);
    a.expect("the unit-domain twin certifies");
    b.expect("the doubled-domain twin certifies");
    assert_eq!(o1, o2, "the twins must classify identically");
    assert!(
        (once - twice).abs() < 1e-15,
        "the same locus must answer the same length: {once:e} vs {twice:e}"
    );
    // The comparand that is NOT invariant: the bare rate, recovered
    // from the recorded length by the domain it was metered over.
    let (rate_once, rate_twice) = (once / 1.0, twice / 2.0);
    assert!(
        (rate_once / rate_twice - 2.0).abs() < 1e-12,
        "the bare rate must SPLIT these twins ({rate_once:e} vs {rate_twice:e}) — \
         otherwise this row proves nothing about the fold-in"
    );
}

/// **The weakening attack.** A carrier whose arc length is `3ε` — in
/// band at every ε — authored over a domain small enough that its
/// bare rate is decisively positive (`3ε / 1e-6`, ≥ 3e-6 at the
/// tightest supported ε). The rate-gated predicate certified this
/// span; the length-gated one escalates it, and the escalation names
/// the meter, not the span direction.
///
/// Posture, asserted not assumed: Escalated at every ε row, because
/// the fixture is sized in units of the run's own ε.
#[test]
fn a_sub_band_carrier_escalates_on_its_meter_not_its_span() {
    let eps = Tol::witness().get().eps;
    let (length, domain) = (3.0 * eps, 1e-6);
    let ((outcome, margin), got) = span_meter_sample(length, domain);
    let b = band();
    // The bare rate — what the old comparand classified — is decisive.
    let rate = length / domain;
    assert!(
        rate > b.escalate(),
        "the attack only bites if the RATE clears the band: {rate:e} vs {:e}",
        b.escalate()
    );
    assert!(
        matches!(outcome, SampleOutcome::Indeterminate),
        "the true arc length {margin:e} lies inside Band {{ {:e}, {:e} }} and must \
         escalate — anything else is the weakening this fold-in closes: {outcome:?}",
        b.zero(),
        b.escalate()
    );
    match got {
        Err(CertifyError::Escalated { cause, .. }) => {
            assert_eq!(
                cause.predicate,
                Some("nurbs_span_meter"),
                "the escalation must name the METER: {cause:?}"
            );
            assert!(
                !matches!(cause.margin, MarginDiag::Invalid),
                "an in-band length is indeterminate, not invalid: {cause:?}"
            );
        }
        other => panic!(
            "a carrier shorter than the band must escalate on its meter, not \
             certify: {other:?}"
        ),
    }
}

/// The other half of "decides it correctly": the SAME tiny domain
/// with an honest length certifies. The gate refuses short carriers,
/// not short parametrizations.
#[test]
fn a_short_domain_with_an_honest_length_still_certifies() {
    let ((outcome, margin), got) = span_meter_sample(1e-3, 1e-6);
    got.expect("a millimetre carrier over a 1e-6 knot domain certifies");
    assert!(
        matches!(outcome, SampleOutcome::Definite(Sign::Positive)),
        "a 1 mm carrier clears every supported band: {outcome:?}"
    );
    assert!(
        (margin - 1e-3).abs() < 1e-18,
        "margin {margin:e} is not the 1 mm arc length"
    );
}
