//! **A caller's interval that runs no clear length forward, or past a
//! full turn, is refused typed at the door that takes it**
//! (`[failure]`, `[tolerance.recourse]` (iv)): every public door that
//! mints an edge from the caller's input refuses it as
//! `CertifyError::StatedInterval`, in the span check's own lever, and
//! never reaches the span check's unminted side at a build, which
//! panics. One row per door and per way a caller's input reaches the
//! span: a reversed or empty `param_start … param_end`, a chord between
//! coincident points, a carrier of no positive radius or minor
//! semi-axis, and an arc past a full turn. Red the day a door lets one
//! through to the certify check again.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom_brep::recourse::SpanDecision;
use geom_brep::{CertifyError, EdgeCurve, EdgeCurveSpec, StatedSpan};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::construction::Construct;
use topo::construction::{MefSite, MevSite};
use topo::{Body, EulerOpError};

const TAU: f64 = core::f64::consts::TAU;

fn p(x: f64, y: f64) -> Point3<f64> {
    Point3::new(x, y, 0.0)
}

/// The unit circle about the origin in the xy plane, of radius `r`.
fn circle(r: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    }
}

/// Which span decision refused, once the door's whole render at a build
/// is shown to end in the decision's lever (rendering it does not
/// panic).
fn refused(result: Result<impl core::fmt::Debug, EulerOpError>, row: &str) -> SpanDecision {
    let error = result.expect_err(row);
    let EulerOpError::Certification {
        error: CertifyError::StatedInterval { refused, .. },
    } = &error
    else {
        panic!("{row}: not the caller's interval: {error:?}");
    };
    let text = error.to_string();
    assert!(
        text.ends_with(&format!("Recourse: {}", refused.decision().lever())),
        "{row}: {text}"
    );
    refused.decision()
}

/// `Body::mev` from a lone vertex at `(1, 0)` with the caller's spec.
fn mev(spec: EdgeCurveSpec<f64>, to: Point3<f64>, row: &str) -> SpanDecision {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(1.0, 0.0), true).unwrap();
    let r#loop = seed.r#loop;
    refused(
        body.mev(MevSite::Lone { r#loop }, to, spec, Tol::witness()),
        row,
    )
}

#[test]
fn every_door_refuses_a_callers_interval_typed() {
    let (a, b) = (p(1.0, 0.0), p(2.0, 0.0));
    let line = EdgeCurveSpec::line_between(a, b);
    let reversed = EdgeCurveSpec {
        param_start: line.param_end,
        param_end: line.param_start,
        ..line.clone()
    };
    let empty = EdgeCurveSpec {
        param_end: line.param_start,
        ..line.clone()
    };
    let arc = |r: f64, t1: f64| EdgeCurveSpec::arc_of_circle(circle(r), 0.0, t1).unwrap();
    let ellipse = EdgeCurveSpec {
        carrier: Curve3::Ellipse {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            semi_u: 1.0,
            semi_v: -0.5,
            u_ref: Vec3::unit_x(),
        },
        param_start: 0.0,
        param_end: 1.0,
        ..line.clone()
    };
    let not_forward = SpanDecision::Forward;
    let rows = [
        (
            "mev, reversed",
            mev(reversed, b, "mev, reversed"),
            not_forward,
        ),
        ("mev, empty", mev(empty, b, "mev, empty"), not_forward),
        (
            "mev, a circle of radius zero",
            mev(arc(0.0, 1.0), a, "radius zero"),
            not_forward,
        ),
        (
            "mev, a circle of negative radius",
            mev(arc(-1.0, 1.0), p(-1.0, 0.0), "negative radius"),
            not_forward,
        ),
        (
            "mev, an ellipse of negative minor semi-axis",
            mev(ellipse, b, "negative minor"),
            not_forward,
        ),
        (
            "mev, an arc past a full turn",
            mev(arc(1.0, 1.5 * TAU), p(-1.0, 0.0), "past a turn"),
            SpanDecision::Winding,
        ),
    ];
    for (row, got, want) in rows {
        assert_eq!(got, want, "{row}");
    }

    // `mev_line` to the point it starts from.
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a, true).unwrap();
    let r#loop = seed.r#loop;
    assert_eq!(
        refused(
            body.mev_line(MevSite::Lone { r#loop }, a, Tol::witness()),
            "mev_line, coincident"
        ),
        not_forward
    );

    // `mef_chord` between two distinct vertices at one point: a strut
    // out to `b` and back to a new vertex at `a`.
    let seg = body
        .mev_line(MevSite::Lone { r#loop }, b, Tol::witness())
        .unwrap();
    let back = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_minus,
                he2: seg.he_minus,
            },
            a,
            Tol::witness(),
        )
        .unwrap();
    let chords = MefSite::Chords {
        he1: seg.he_plus,
        he2: back.he_minus,
    };
    assert_eq!(
        refused(
            body.mef_chord(chords, Tol::witness()),
            "mef_chord, coincident"
        ),
        not_forward
    );

    // The attach door, on the strut's edge.
    let at_rest_reversed = EdgeCurveSpec {
        param_start: 1.0,
        param_end: 0.0,
        ..EdgeCurveSpec::line_between(a, b)
    };
    assert_eq!(
        refused(
            body.set_edge_curve(seg.edge, at_rest_reversed, Tol::witness()),
            "set_edge_curve, reversed"
        ),
        not_forward
    );

    // The certifier's own minting door.
    let band = Band::linear(Tol::witness()).unwrap();
    let direct = EdgeCurve::certify(
        EdgeCurveSpec::line_between(a, a),
        a,
        a,
        |_| None,
        1.0,
        band,
        geom_brep::NurbsLane::certified(),
    )
    .unwrap_err();
    assert!(
        matches!(
            direct,
            CertifyError::StatedInterval {
                refused: StatedSpan::Forward(_),
            }
        ),
        "EdgeCurve::certify, coincident: {direct:?}"
    );
}
