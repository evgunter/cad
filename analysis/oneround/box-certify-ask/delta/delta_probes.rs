//! Reviewer probes for the delta read of PR 7272 (head 1ff43e30).
//! Module `certify::delta_probes` (see `delta_probes.patch`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stderr)]

use crate::{CertifyError, EdgeCurve, EdgeCurveSpec, NurbsLane};
use geom_core::{Band, Point3, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn certify(spec: EdgeCurveSpec<f64>, a: Point3<f64>, b: Point3<f64>) -> CertifyError {
    match EdgeCurve::certify(
        spec,
        a,
        b,
        |_| None,
        geom_core::test_support::fixture_lever(1.0),
        band(),
        NurbsLane::certified(),
    ) {
        Ok(_) => panic!("certified"),
        Err(e) => e,
    }
}

fn show(name: &str, e: &CertifyError) {
    let s = format!("{e:?}");
    eprintln!(
        "{name}: {} || {}",
        s.chars().take(110).collect::<String>(),
        e.render(crate::recourse::Reading::Build)
    );
}

/// Claim 2: which refusal, and which ending, for each mix of span and
/// cover. Printed; the asserts are the PR body's own sentences.
#[test]
fn claim2_span_and_cover() {
    let o = Point3::origin();
    let x = Point3::new(1.0, 0.0, 0.0);
    let far = Point3::new(2e3, 0.0, 0.0);
    let nan = Point3::new(f64::NAN, 0.0, 0.0);

    // (a) coincident points 2 km out: the box shows the edge past it, so
    // nothing else is decided: OutsideSessionBox, not the span.
    let e = certify(EdgeCurveSpec::line_between(far, far), far, far);
    show("coincident points 2 km out", &e);
    assert!(matches!(e, CertifyError::OutsideSessionBox { .. }), "{e:?}");

    // (b) a covered line in the box over a zero span: the span's refusal,
    // from the checks (no box decision on it but its reach).
    let zero = EdgeCurveSpec { param_end: 0.0, ..EdgeCurveSpec::line_between(o, x) };
    show("covered, zero span", &certify(zero, o, o));

    // (c) a covered far line over a zero span: past the box first.
    let zero_far = EdgeCurveSpec {
        param_end: 0.0,
        ..EdgeCurveSpec::line_between(far, Point3::new(2e3 + 1.0, 0.0, 0.0))
    };
    let e = certify(zero_far, far, far);
    show("covered far line, zero span", &e);
    assert!(matches!(e, CertifyError::OutsideSessionBox { .. }), "{e:?}");

    // (d) a NaN end beside an end 5 km out: past the box by the far end.
    let e = certify(EdgeCurveSpec::line_between(o, x), Point3::new(5e3, 0.0, 0.0), nan);
    show("NaN end beside a far end", &e);
    assert!(matches!(e, CertifyError::OutsideSessionBox { .. }), "{e:?}");

    // (e) a NaN end on a good line, forward span: the caller handed a
    // point that is no point. Whose defect does the ending name?
    show("NaN end, forward span", &certify(EdgeCurveSpec::line_between(o, x), o, nan));
    // (f) the same over a zero span and over a reversed one.
    let z = EdgeCurveSpec { param_end: 0.0, ..EdgeCurveSpec::line_between(o, x) };
    show("NaN end, zero span", &certify(z, o, nan));
    let r = EdgeCurveSpec { param_end: -1.0, ..EdgeCurveSpec::line_between(o, x) };
    show("NaN end, reversed span", &certify(r, o, nan));
    // (g) an in-band span that is not zero, on the NaN-direction line.
    let tiny = EdgeCurveSpec { param_end: 5e-10, ..EdgeCurveSpec::line_between(o, o) };
    show("NaN line, span 5e-10", &certify(tiny, o, o));
    // (h) a line the caller spelled between a point and a NaN point.
    show("line_between(o, nan)", &certify(EdgeCurveSpec::line_between(o, nan), o, nan));
    // (i) a NaN-direction line whose stated span is NaN.
    let s = EdgeCurveSpec { param_end: f64::NAN, ..EdgeCurveSpec::line_between(o, o) };
    show("NaN line, NaN span", &certify(s, o, o));
}
