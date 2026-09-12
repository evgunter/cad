//! Blinded-review probes for the M5 PR 9 jet certificate (charter B/C):
//! near-osculating escalation sweep, tube-threading attack, drift-0
//! discrimination, and the second-order lever-arm scaling law.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::surf::arena2;
use crate::shared::tol::band;
use geom::Curve3;
use geom::Surface;
use geom_brep::{CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Point3, Vec3};

fn zcyl(cx: f64, r: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(cx, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// Certify a z-line at (x, y) over z in [0, 1] as TangentIntersection
/// of the two surfaces.
fn certify_line(
    s1: Surface<f64>,
    s2: Surface<f64>,
    x: f64,
    y: f64,
) -> Result<EdgeCurve<f64>, CertifyError> {
    let (k1, k2, map) = arena2(s1, s2);
    let carrier = Curve3::Line {
        origin: Point3::new(x, y, 0.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    };
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::TangentIntersection {
            s1: k1,
            s2: k2,
            witness: carrier.eval(0.5),
        },
        carrier,
        param_start: 0.0,
        param_end: 1.0,
    };
    let (p0, p1) = (spec.carrier.eval(0.0), spec.carrier.eval(1.0));
    EdgeCurve::certify(spec, p0, p1, |k| map.get(k).cloned(), band())
}

#[test]
fn near_osculating_sweep_escalates_before_the_certificate_lies() {
    // Internal tangency: cyl1 (r = 1, axis through origin) inside
    // cyl2 (r = 1 + d, axis at (-d, 0)); contact ruling x = 1.
    // kappa_rel = d/(1+d); arm = min(1, 1+d, extent=1) = 1;
    // second-order margin = kappa_rel/2. Outcome classes must flip
    // exactly at the RESOLVED band's thresholds (no eps literals).
    let b = band();
    let mut rows = Vec::new();
    for exp in 4..=11 {
        let d = 10f64.powi(-exp);
        let kappa_rel = 1.0 - 1.0 / (1.0 + d);
        let margin = kappa_rel / 2.0;
        let out = certify_line(zcyl(0.0, 1.0), zcyl(-d, 1.0 + d), 1.0, 0.0);
        let got = match &out {
            Ok(_) => "definite",
            Err(CertifyError::NotSecondOrderSeparated { .. }) => "zero",
            Err(CertifyError::Escalated { .. }) => "escalated",
            Err(e) => panic!("unexpected arm at d={d:e}: {e:?}"),
        };
        let want = if margin <= b.zero() {
            "zero"
        } else if margin < b.escalate() {
            "escalated"
        } else {
            "definite"
        };
        rows.push(format!("d={d:e} margin={margin:e} got={got} want={want}"));
        assert_eq!(got, want, "rows so far:\n{}", rows.join("\n"));
    }
    eprintln!("SWEEP:\n{}", rows.join("\n"));
}

#[test]
fn an_offset_ruling_cannot_thread_the_tube_beyond_eps() {
    // d = 1e-3: definitively second-order separated (margin ~ 5e-4).
    // A parallel line at azimuth theta on cyl1 is on cyl1 exactly and
    // within ~d*theta^2/2 of cyl2 — residuals pass for large theta.
    // The claim under attack: only the parallelism check (lever arm
    // 1/kappa_rel) stops a distinct locus from certifying. Spatial
    // separation at the acceptance boundary must be ~eps, not
    // sqrt(2 eps / kappa).
    let d = 1e-3;
    let b = band();
    // theta = separation in metres on the r=1 wall.
    let mut max_accepted: f64 = 0.0;
    let mut min_rejected = f64::MAX;
    for exp in [-12.0, -10.0, -9.5, -9.0, -8.5, -8.0, -7.0, -6.0, -5.0, -4.0] {
        let theta = 10f64.powf(exp);
        let (x, y) = (theta.cos(), theta.sin());
        match certify_line(zcyl(0.0, 1.0), zcyl(-d, 1.0 + d), x, y) {
            Ok(_) => max_accepted = max_accepted.max(theta),
            Err(_) => min_rejected = min_rejected.min(theta),
        }
    }
    eprintln!("max accepted offset {max_accepted:e}, min rejected {min_rejected:e}");
    // The tube holds iff no distinct locus beyond the escalate width
    // certifies definitely: K * eps = band.escalate() (metres at the
    // r = 1 lever arm).
    assert!(
        max_accepted <= b.escalate(),
        "a ruling {max_accepted:e} m away from the true locus certified definitely \
         (band escalate {:e}) — the tube is threaded",
        b.escalate()
    );
}

#[test]
fn ruling_drift_is_exactly_zero_and_the_bound_discriminates() {
    // Long ruling t in [0, 100] on a small cylinder r = 0.1 tangent
    // to the plane x = 0.1. If the certified kappa-drift bound were
    // step * 8|dir|/r^2 (full |dir|, not the axis-transverse
    // component), the tube would be (10 - 10000)*0.005 << 0 and the
    // certificate would refuse; the ruled-surface drift being exactly
    // 0 is what lets this accept with tube = 0.05.
    let cyl = zcyl(0.0, 0.1);
    let plane = Surface::Plane {
        origin: Point3::new(0.1, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        u_ref: Vec3::new(0.0, 0.0, 1.0),
    };
    let (k1, k2, map) = arena2(cyl, plane);
    let carrier = Curve3::Line {
        origin: Point3::new(0.1, 0.0, 0.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    };
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::TangentIntersection {
            s1: k1,
            s2: k2,
            witness: carrier.eval(50.0),
        },
        carrier,
        param_start: 0.0,
        param_end: 100.0,
    };
    let (p0, p1) = (spec.carrier.eval(0.0), spec.carrier.eval(100.0));
    let out = EdgeCurve::certify(spec, p0, p1, |k| map.get(k).cloned(), band());
    assert!(
        out.is_ok(),
        "a 100 m ruling on an r=0.1 cylinder must certify (drift = 0 on a ruling): {:?}",
        out.err()
    );
}

#[test]
fn second_order_margin_scales_with_the_lever_arm_squared() {
    // enters_material_order2: margin = (d2.n / speed^2) * arm^2 / 2.
    // Fixed curvature, arm doubled => margin quadrupled (the D4
    // lever-arm law, executed): probe via outcome flips against the
    // resolved band.
    use geom_brep::enters_material_order2;
    let b = band();
    // Departure curving toward +n with curvature kappa: d2.n/speed^2
    // = kappa. Pick kappa so kappa*arm^2/2 crosses the band inside
    // the arm sweep: kappa = 4*zero. arm=1: margin=2*zero (in band,
    // escalates); arm=sqrt(K/2)*~: margin above escalate (definite).
    let kappa = 4.0 * b.zero();
    let d2 = Vec3::new(0.0, kappa, 0.0);
    // The reference side is a split plane's normal — the convention,
    // not a face's sense.
    let n = geom_brep::ReferenceNormal::of_split_plane(Vec3::new(0.0, 1.0, 0.0));
    let small = enters_material_order2(d2, 1.0, n, 1.0, b);
    assert!(
        small.is_err(),
        "margin 2*zero at arm 1 must escalate F6: {small:?}"
    );
    let big_arm = (2.0 * b.escalate() / kappa).sqrt() * 1.5;
    let big = enters_material_order2(d2, 1.0, n, big_arm, b).unwrap();
    assert_eq!(
        big,
        geom_brep::EntersMaterial::Exits,
        "the same curvature at lever arm {big_arm} resolves definitely"
    );
    // Exact tie stays Tangent (typed, never guessed).
    let tie = enters_material_order2(Vec3::new(0.0, 0.0, 0.0), 1.0, n, 1.0, b).unwrap();
    assert_eq!(tie, geom_brep::EntersMaterial::Tangent);
    // Sign convention: curving along -n enters material.
    let enters = enters_material_order2(Vec3::new(0.0, -1.0, 0.0), 1.0, n, 1.0, b).unwrap();
    assert_eq!(enters, geom_brep::EntersMaterial::Enters);
}
