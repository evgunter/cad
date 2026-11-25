//! **The curved lane's posture rows** ([`super::covered_posture`],
//! [`super::straddle_event`]): a covered arc's posture is read off its
//! crossing reading, one row per reading, and a straddling span the
//! answer says has no crossing fails loud.

use super::{CoveredPosture, CurvedEvent, SpanVerdict, covered_posture, straddle_event};
use crate::boolean::BooleanError;
use geom::Curve3;
use geom_brep::curve_face::cross;
use geom_brep::curve_pair::Span;
use geom_core::{Band, Point3, Sign, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap_or_else(|e| panic!("the witness band: {e:?}"))
}

fn frame(origin: Point3<f64>) -> geom::Frame<f64> {
    geom::Frame::new(origin, Vec3::unit_z(), Vec3::unit_x())
}

fn sphere(at: [f64; 3], radius: f64) -> geom::Surface<f64> {
    geom::Surface::Sphere {
        frame: frame(Point3::from_array(at)),
        radius,
    }
}

/// The unit circle about the origin in the `xy` plane.
fn ring() -> Curve3<f64> {
    Curve3::circle(Point3::origin(), Vec3::unit_z(), 1.0, Vec3::unit_x())
}

/// The covered posture of `carrier`'s span `(t0, t1)` against `surface`.
fn posture_over(
    carrier: &Curve3<f64>,
    span: (f64, f64),
    surface: &geom::Surface<f64>,
) -> CoveredPosture {
    covered_posture(cross(&Span::new(carrier, span), surface, band()))
}

/// The covered posture of `carrier`'s whole turn against `surface`.
fn posture(carrier: &Curve3<f64>, surface: &geom::Surface<f64>) -> CoveredPosture {
    posture_over(carrier, (0.0, core::f64::consts::TAU), surface)
}

/// **`Apart` is clear**: the unit circle and a sphere 4 m past it.
/// Mutant: `Apart` read as the zero posture, red.
#[test]
fn an_apart_pose_is_clear() {
    assert!(matches!(
        posture(&ring(), &sphere([5.0, 0.0, 0.0], 0.5)),
        CoveredPosture::Sign(Sign::Positive)
    ));
}

/// **A decided touch is the zero posture**: the unit circle touching a
/// unit sphere centred `(2, 0, 0)` at `(1, 0, 0)`. Mutant: `Tangent` read
/// as a crossing (`Negative`), red.
#[test]
fn a_tangent_pose_is_the_zero_posture() {
    assert!(matches!(
        posture(&ring(), &sphere([2.0, 0.0, 0.0], 1.0)),
        CoveredPosture::Sign(Sign::Zero)
    ));
}

/// **The span on the carrier is the zero posture**: the unit circle on
/// the unit wall about `z`. Mutant: `Lies` read as clear, red.
#[test]
fn a_lying_pose_is_the_zero_posture() {
    let wall = geom::Surface::Cylinder {
        frame: frame(Point3::origin()),
        radius: 1.0,
    };
    assert!(matches!(
        posture(&ring(), &wall),
        CoveredPosture::Sign(Sign::Zero)
    ));
}

/// **A crossing inside the span crosses**: the unit circle through a unit
/// sphere centred `(1, 0, 0)`. Mutant: a crossing read as the zero
/// posture, red.
#[test]
fn a_crossing_inside_the_span_crosses() {
    assert!(matches!(
        posture(&ring(), &sphere([1.0, 0.0, 0.0], 1.0)),
        CoveredPosture::Sign(Sign::Negative)
    ));
}

/// **A span that meets nothing is clear, though its carrier crosses**:
/// the unit circle's arc `[0, ½]` against a unit sphere centred
/// `(−1, 0, 0)`, which the circle crosses at `θ = ±2π/3` only. The pose is
/// `Isolated` (the ball cull holds the sphere), and no meeting lies on the
/// span. Mutant: `Isolated` read as a crossing, red.
#[test]
fn a_span_meeting_nothing_is_clear() {
    assert!(matches!(
        posture_over(&ring(), (0.0, 0.5), &sphere([-1.0, 0.0, 0.0], 1.0)),
        CoveredPosture::Sign(Sign::Positive)
    ));
}

/// **A rung in band is the coincidence's refusal**, its diagnostics
/// kept: the unit circle a gap in the band's escalation gap short of
/// touching a unit sphere centred past `(2, 0, 0)`. Mutant: an in-band
/// rung read as the frontier (`None`), red.
#[test]
fn an_in_band_pose_refuses_with_its_diagnostics() {
    let b = band();
    let gap = (b.zero() + b.escalate()) / 2.0;
    assert!(matches!(
        posture(&ring(), &sphere([2.0 + gap, 0.0, 0.0], 1.0)),
        CoveredPosture::InBand(_)
    ));
}

/// **A meter's refusal keeps the frontier**: a circle of radius 2·10⁵ m
/// passing 2 m outside a unit sphere (the conic × quadric noise meter's
/// pose, past the cull). Mutant: a meter read as the coincidence's
/// in-band refusal, red.
#[test]
fn a_meter_refusal_is_the_frontier() {
    let far = 2e5;
    let big = Curve3::circle(Point3::origin(), Vec3::unit_z(), far, Vec3::unit_x());
    assert!(matches!(
        posture_over(&big, (-1.0, 1.0), &sphere([far - 3.0, 0.0, 0.0], 1.0)),
        CoveredPosture::Frontier
    ));
}

/// **The covered cone reads its crossing reading exactly** (where the
/// cone's clearance enclosure, a lower bound only, sent every reading short
/// of clear to the frontier): a circle in the plane `z = 1` centred
/// `(0.3, 0, 1)`, against the cone of half-angle `π/4` about `z` (radius 1
/// at that height). Its form against the cone is a first harmonic, so the
/// pose reads its extremes: of radius `0.7` it touches the cone from
/// inside at `(1, 0, 1)`, the zero posture; of radius `0.6` it is clear; of
/// radius `0.8` it crosses.
#[test]
fn a_covered_cone_reads_its_pose() {
    let cone = geom::Surface::Cone {
        frame: frame(Point3::origin()),
        half_angle: core::f64::consts::FRAC_PI_4,
    };
    let circle = |radius: f64| {
        Curve3::circle(
            Point3::new(0.3, 0.0, 1.0),
            Vec3::unit_z(),
            radius,
            Vec3::unit_x(),
        )
    };
    let got = posture(&circle(0.7), &cone);
    assert!(matches!(got, CoveredPosture::Sign(Sign::Zero)), "{got:?}");
    assert!(matches!(
        posture(&circle(0.6), &cone),
        CoveredPosture::Sign(Sign::Positive)
    ));
    assert!(matches!(
        posture(&circle(0.8), &cone),
        CoveredPosture::Sign(Sign::Negative)
    ));
}

fn frontier() -> BooleanError {
    BooleanError::ClassificationInvariant {
        what: "the row's frontier",
    }
}

fn at_apex() -> BooleanError {
    BooleanError::ClassificationInvariant {
        what: "the row's apex",
    }
}

/// **A straddle with no interior crossing fails loud**: the endpoints
/// decided on opposite sides, the answer with no window and no crossing
/// strictly inside (`NoInterior`), none at all (`Miss`), or the span on
/// the carrier (`LiesOn`), is a contradiction between certified
/// readings. A windowed answer read off the face (`OffFace`) and a
/// crossing placed off it (`Elsewhere`) are no event; an unsettled one
/// keeps the frontier.
///
/// Mutant: the contradiction read as the frontier, red.
#[test]
fn a_straddle_with_no_interior_crossing_fails_loud() {
    let fails_loud = |verdict: SpanVerdict<f64>| {
        matches!(
            straddle_event(verdict, frontier, at_apex),
            Err(BooleanError::ClassificationInvariant { what }) if what.starts_with("a span whose endpoints straddle")
        )
    };
    assert!(fails_loud(SpanVerdict::NoInterior));
    assert!(fails_loud(SpanVerdict::Miss));
    assert!(fails_loud(SpanVerdict::LiesOn));
    for quiet in [SpanVerdict::<f64>::OffFace, SpanVerdict::Elsewhere] {
        assert!(matches!(
            straddle_event(quiet, frontier, at_apex),
            Ok(CurvedEvent::None)
        ));
    }
    assert!(matches!(
        straddle_event(SpanVerdict::<f64>::Unsettled, frontier, at_apex),
        Err(BooleanError::ClassificationInvariant {
            what: "the row's frontier"
        })
    ));
}
