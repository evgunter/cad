//! **A section chord's end in band of its vertex is the input's only
//! where the vertex's own offset off the plane is the miss**
//! ([`SectionEnds::reading`]), in closed form: the quarter of the unit
//! circle in `z = 0` from `(1, 0, 0)` to `(0, 1, 0)`, its end pin
//! escalated, with the vertex or the carrier moved `1.2ε` off.

use super::*;
use geom_brep::{CertCheck, CertifyError};
use geom_core::MarginDiag;

const EPS: f64 = 1e-9;
const OFF: f64 = 1.2 * EPS;

fn band() -> Band {
    Band::new(EPS, 10.0 * EPS).unwrap()
}

/// The quarter arc of the unit circle about `z` in the plane `z = h`.
fn arc(h: f64) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec::arc_of_circle(
        geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, h),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
        0.0,
        core::f64::consts::FRAC_PI_2,
    )
    .unwrap()
}

/// The section plane `z = 0`, or the stood-in circle's plane.
fn ends(square: bool) -> SectionEnds<f64> {
    SectionEnds {
        origin: Point3::new(0.5, 0.5, 0.0),
        normal: Vec3::unit_z(),
        square,
    }
}

fn pin(check: CertCheck) -> (EulerOpError, Indeterminate) {
    let cause = Indeterminate {
        margin: MarginDiag::value(OFF),
        band: band(),
        predicate: Some("carrier_endpoint_end"),
        terminal_sliver: false,
    };
    let error = CertifyError::Escalated {
        check,
        sample: 8,
        cause,
    };
    (EulerOpError::Certification { error }, cause)
}

/// The vertex `OFF` along the plane's normal is the input's: on the
/// section plane an edge on its faces, on a stood-in circle's plane the
/// plane read square, each with the pin's diagnostics. At the start and
/// at the end alike.
#[test]
fn a_vertex_off_the_plane_is_the_inputs() {
    let start = Point3::new(1.0, 0.0, 0.0);
    let end = Point3::new(0.0, 1.0, 0.0);
    let up = Vec3::unit_z() * OFF;
    for (check, at) in [
        (CertCheck::EndpointStart, (start + up, end)),
        (CertCheck::EndpointEnd, (start, end - up)),
    ] {
        let (refusal, cause) = pin(check);
        for (square, want) in [
            (false, SectionEndReading::OnPlane),
            (true, SectionEndReading::SquareToAxis),
        ] {
            assert_eq!(
                ends(square).reading(&arc(0.0), at, &refusal, band()),
                Some((want, cause)),
                "{check:?}, square {square}"
            );
        }
    }
}

/// A miss the vertex's offset does not account for is the carrier's
/// own, whichever way it points: the vertex on the plane `OFF` from the
/// end within the plane, or the carrier itself `OFF` off the plane (a
/// reading that took the normal part of the miss from the carrier would
/// call that the vertex's). A definite miss and another check stay the
/// gate's too.
#[test]
fn the_carriers_own_miss_stays_the_gates() {
    let start = Point3::new(1.0, 0.0, 0.0);
    let (refusal, _) = pin(CertCheck::EndpointEnd);
    for (what, carrier, end) in [
        ("in the plane", arc(0.0), Point3::new(0.0, 1.0 + OFF, 0.0)),
        ("off the plane", arc(OFF), Point3::new(0.0, 1.0, 0.0)),
    ] {
        for square in [false, true] {
            assert_eq!(
                ends(square).reading(&carrier, (start, end), &refusal, band()),
                None,
                "{what}, square {square}"
            );
        }
    }
    let at = (start, Point3::new(0.0, 1.0, OFF));
    let definite = EulerOpError::Certification {
        error: CertifyError::ResidualExceeded {
            check: CertCheck::EndpointEnd,
            sample: 8,
            margin: MarginDiag::value(20.0 * EPS),
        },
    };
    let (other, _) = pin(CertCheck::WitnessMidpoint);
    for refusal in [definite, other] {
        assert_eq!(
            ends(false).reading(&arc(0.0), at, &refusal, band()),
            None,
            "{refusal:?}"
        );
    }
}
