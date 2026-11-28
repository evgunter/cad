//! **The curved lane's posture rows** ([`super::covered_posture`],
//! [`super::straddle_event`]): a covered arc's posture is read off its
//! crossing reading, one row per reading, and a straddling span the
//! answer says has no crossing fails loud.

#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

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
    covered_posture(
        cross(&Span::new(carrier, span), surface, band()),
        !carrier.is_curved(),
    )
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
/// pose, past the cull). Mutant: a meter read as the zero posture,
/// red.
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

/// **A method's sign in band is the zero posture**: a circle in the `xz`
/// plane touching the generator `x = z` of the cone of half-angle `π/4`
/// from outside at `(1, 0, 1)`. Its form has a second harmonic, so the
/// pose hands it to the ladder, whose walk reads its side in band at the
/// touch (`conic_quadric_sub_side`); under the cover, which rules out a
/// crossing, that is a touch, read by the covered rung's ends and interior.
/// Mutant: a method's sign in band read as the coincidence's refusal, red.
#[test]
fn a_method_sign_in_band_is_the_zero_posture() {
    let cone = geom::Surface::Cone {
        frame: frame(Point3::origin()),
        half_angle: core::f64::consts::FRAC_PI_4,
    };
    let out = Vec3::new(1.0, 0.0, -1.0).normalize();
    let touching = Curve3::circle(
        Point3::new(1.0, 0.0, 1.0) + out * 0.3,
        Vec3::unit_y(),
        0.3,
        Vec3::unit_x(),
    );
    let got = posture(&touching, &cone);
    assert!(matches!(got, CoveredPosture::Sign(Sign::Zero)), "{got:?}");
}

/// The covered posture of a refusal with the one window `why`.
fn refused(why: geom_brep::curve_face::CrossWhy) -> CoveredPosture {
    covered_posture::<f64>(
        Err(geom_brep::curve_face::CrossRefusal {
            found: Vec::new(),
            windows: vec![geom_brep::curve_face::Undecided {
                window: (0.0, 0.1),
                why,
            }],
        }),
        false,
    )
}

/// A method's window of `fault`.
fn method(fault: geom_brep::curve_pair::MeetFault) -> geom_brep::curve_face::CrossWhy {
    geom_brep::curve_face::CrossWhy::Method {
        door: geom_brep::curve_face::Door::ConicQuadric,
        fault,
    }
}

/// **A tangency the method could not settle is the zero posture**, under
/// the cover a touch (a rim's arc touching the partner's wall at its own
/// end, `a_closed_conic_in_the_partners_face`'s crescent tips); **a meter
/// the method refused keeps the frontier**. Mutant: the unsettled
/// tangency read as the frontier, red.
#[test]
fn a_methods_unsettled_tangency_is_the_zero_posture_and_its_meter_the_frontier() {
    use geom_brep::curve_pair::MeetFault;
    use geom_brep::roots::Unsettled;
    assert!(matches!(
        refused(method(MeetFault::Unsettled(Unsettled::Tangency))),
        CoveredPosture::Sign(Sign::Zero)
    ));
    assert!(matches!(
        refused(method(MeetFault::Unsettled(Unsettled::Meter {
            row: "a row"
        }))),
        CoveredPosture::Frontier
    ));
}

/// **A pose rung read in its zero band is the zero posture; one read in
/// its escalation gap is the coincidence refusal.** Mutant: the zero-band
/// reading read as the coincidence's refusal, red.
#[test]
fn a_pose_rung_in_its_zero_band_is_the_zero_posture() {
    use geom_brep::curve_face::{CrossWhy, PoseRefusal, PoseRung};
    let b = band();
    let zero = geom_core::k_stats::decide_nonzero(
        "test_curve_posture_zero",
        geom_core::Margin::of(0.0_f64),
        b,
    )
    .err()
    .unwrap_or_else(|| panic!("a zero reading is rejected"));
    let gap = geom_core::k_stats::decide_nonzero(
        "test_curve_posture_gap",
        geom_core::Margin::of((b.zero() + b.escalate()) / 2.0),
        b,
    )
    .err()
    .unwrap_or_else(|| panic!("a reading in the gap is undecided"));
    let rung = |diag| {
        CrossWhy::Pose(PoseRefusal::InBand {
            rung: PoseRung::Extreme,
            diag,
        })
    };
    assert!(matches!(
        refused(rung(zero)),
        CoveredPosture::Sign(Sign::Zero)
    ));
    assert!(matches!(refused(rung(gap)), CoveredPosture::InBand(_)));
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

/// **A covered span through a cone's apex crosses** (the frontier): a
/// circle in the `xz` plane through the apex of the cone of half-angle
/// `π/4`, whose pose is `Through`. The cover rules out a crossing, so a
/// span through the apex is the frontier, never the zero posture.
///
/// Mutant: `Through` read as the zero posture, red.
#[test]
fn a_covered_span_through_a_cones_apex_is_the_frontier() {
    let cone = geom::Surface::Cone {
        frame: frame(Point3::origin()),
        half_angle: core::f64::consts::FRAC_PI_4,
    };
    let through = Curve3::circle(
        Point3::new(0.5, 0.0, 0.0),
        Vec3::unit_y(),
        0.5,
        Vec3::new(-1.0, 0.0, 0.0),
    );
    let got = posture(&through, &cone);
    assert!(
        matches!(got, CoveredPosture::Sign(Sign::Negative)),
        "{got:?}"
    );
}

/// **A refusal is read whole**: two windows, a graze and a method's
/// tangency (each the zero posture alone), and a crossing found strictly
/// inside the span between them. The crossing decides: the frontier.
/// With no crossing found, the two windows read the zero posture; with a
/// walk's exhausted budget beside them, the frontier.
///
/// Mutant: the leading window read alone, red.
#[test]
fn a_refusal_is_read_by_every_window_and_its_found_crossing() {
    use geom_brep::curve_face::{CrossRefusal, Meeting, MeetingKind, OnSpan, Undecided};
    use geom_brep::curve_pair::MeetFault;
    use geom_brep::roots::Unsettled;
    let graze = method(MeetFault::Unsettled(Unsettled::Tangency));
    let limit = method(MeetFault::Unsettled(Unsettled::Exhausted));
    let read = |whys: &[geom_brep::curve_face::CrossWhy], found: Vec<Meeting<f64>>| {
        covered_posture::<f64>(
            Err(CrossRefusal {
                found,
                windows: whys
                    .iter()
                    .enumerate()
                    .map(|(k, &why)| Undecided {
                        window: (k as f64, k as f64 + 0.1),
                        why,
                    })
                    .collect(),
            }),
            false,
        )
    };
    let crossing = Meeting {
        point: Point3::origin(),
        on_span: OnSpan::Inside { t: 0.5 },
        kind: MeetingKind::Cross,
        chart: None,
    };
    let got = read(&[graze, graze], vec![crossing]);
    assert!(
        matches!(got, CoveredPosture::Sign(Sign::Negative)),
        "{got:?}"
    );
    let got = read(&[graze, graze], Vec::new());
    assert!(matches!(got, CoveredPosture::Sign(Sign::Zero)), "{got:?}");
    let got = read(&[graze, limit], Vec::new());
    assert!(matches!(got, CoveredPosture::Frontier), "{got:?}");
}

/// A prism slab `[−3, 3] × [−2.5, 2.5] × [c − 1, c]` with a disc of radius
/// 0.3 planted in its top at `(0, 1 + 0.3 + gap, c)`, and a sheet of the
/// unit wall about the `x` axis through `(0, 0, c)` over azimuth `[−½, ½]`
/// about `+y` and `x ∈ [−½, ½]`, its sense set so it holds that patch: the
/// disc's circle passes the wall at `(0, 1 + gap, c)`, `gap` outside it,
/// in a plane holding the wall's axis direction. With the disc face
/// covered `Off(Positive)` against both faces of the sheet, the circle's edge is swept
/// against the wall (operand A against B) and what the sweep deferred is
/// settled.
fn covered_near_miss(gap: f64) -> (usize, Result<(), BooleanError>) {
    use crate::boolean::{GluedPairs, Operand};
    let tol = Tol::witness();
    let c = 0.5;
    let prism = crate::test_support_fixtures::prism_z::<f64>(
        &[(-3.0, -2.5), (3.0, -2.5), (3.0, 2.5), (-3.0, 2.5)],
        c - 1.0,
        c,
        tol,
    );
    let mut a = prism.body;
    let top = a.get_face(prism.top_face).unwrap().outer;
    let crate::entity::LoopBoundary::Cycle { first } = a.get_loop(top).unwrap().boundary else {
        panic!("the top's loop is a cycle");
    };
    let disc = crate::test_support_fixtures::plant_disc_face(
        &mut a,
        first,
        Point3::new(0.0, 1.3 + gap, c),
        0.3,
        tol,
    );
    let mut b = crate::Body::<f64>::new();
    let wall = crate::test_support_fixtures::cyl_wall_sheet(
        &mut b,
        crate::test_support_fixtures::CylFrame {
            origin: Point3::new(-0.5, 0.0, c),
            axis: Vec3::unit_x(),
            radius: 1.0,
            u_ref: Vec3::unit_y(),
        },
        (-0.5, 0.5),
        (0.0, 1.0),
        tol,
    );
    crate::test_support_fixtures::set_sheet_sense(&mut b, wall, true, tol);
    // The disc covered against both faces of the sheet (the patch and its
    // lamina twin across the same edges).
    let mut declared = GluedPairs::<f64>::default();
    for (target, _) in b.faces() {
        declared.one_sided.insert(
            (
                crate::boolean::tagged(Operand::A, disc.face),
                crate::boolean::tagged(Operand::B, target),
            ),
            crate::boolean::CoverSide::Off(Sign::Positive),
        );
    }
    let mut acc = super::ContactAcc::default();
    let mut deferred = Vec::new();
    let tips = crate::sector_shape::SlitTips::of(&a);
    let swept = super::sweep_direction(
        (&mut a, &tips),
        &mut b,
        Operand::A,
        &declared,
        &mut acc,
        band(),
        super::SweepStrategy::Realized,
        &super::SweepKnobs::default(),
        None,
        &mut deferred,
        &crate::separating::OperandAxes::new(crate::boxes::axis_key),
        tol,
    );
    if let Err(e) = swept {
        return (0, Err(e));
    }
    let count = deferred.len();
    let settled = super::settle_deferred(
        &mut a,
        &mut b,
        deferred,
        &declared,
        &mut acc,
        band(),
        [None, None],
        tol,
    );
    (count, settled)
}

/// **A covered near miss read in the gap never records what a miss would
/// not** (`covered_posture`'s argument): the disc's circle passes a wall
/// sheet `3ε` outside it, inside `Kε`, its parent covered `Off(Positive)`
/// against the wall, so the ladder method reads its side in the band's
/// gap and the posture is the zero one. The sweep records nothing and
/// refuses nothing at the arm: it defers the circle's edge against both
/// faces of the sheet, and the settle,
/// with no vertex splitting the edge, refuses it typed (the frontier).
/// Moved `100·Kε` clear, the same circle defers nothing and settles
/// clean. Neither ever records an incidence, so no different body can
/// come of the gap reading, only a typed refusal where a miss builds
/// (`work/germ/a-covered-near-miss-no-vertex-splits-refuses-where-a-miss-builds.md`).
///
/// Mutant: the method's sign in band sent to the frontier, red (the arm
/// refuses at the sweep instead of deferring).
#[test]
fn a_covered_near_miss_in_the_gap_records_nothing_a_miss_would_not() {
    let eps = band().zero();
    let (near, near_settled) = covered_near_miss(3.0 * eps);
    assert_eq!(
        near, 2,
        "the near miss's circle is deferred against both faces of the sheet: {near_settled:?}"
    );
    assert!(
        matches!(
            near_settled,
            Err(BooleanError::CurvedPierceUnsupported { .. })
        ),
        "with no vertex splitting it, the settle refuses typed: {near_settled:?}"
    );
    let (clear, clear_settled) = covered_near_miss(100.0 * band().escalate());
    assert_eq!(clear, 0, "the clear circle defers nothing");
    assert!(clear_settled.is_ok(), "{clear_settled:?}");
}

/// **The lane's one reading of a refused window, read alike by both
/// maps** (`read_why`): an apex reading in its zero band is the apex's
/// before it is a graze, a conic's on its edge (`AtApex`) and a covered
/// arc's on its posture (the frontier); a method that met a pose with no
/// closed form (`MeetFault::Unsupported`, a limit by
/// `MeetFault::kind`) keeps the door on an edge (`Unsettled`) and is the
/// frontier under a cover, never a fault.
///
/// Mutants: the zero-band arm read before the apex arms, red (an apex in
/// its zero band reads as a graze); an unread method fault read as a
/// fault, red.
#[test]
fn a_refused_window_is_read_alike_by_both_maps() {
    use crate::boolean::{Operand, reduce::SpanVerdict as Verdict};
    use geom_brep::curve_face::{CrossWhy, PoseRefusal, PoseRung};
    use geom_brep::curve_pair::MeetFault;
    let b = band();
    let zero = geom_core::k_stats::decide_nonzero(
        "test_curve_posture_zero",
        geom_core::Margin::of(0.0_f64),
        b,
    )
    .err()
    .unwrap_or_else(|| panic!("a zero reading is rejected"));
    let cone = geom::Surface::Cone {
        frame: frame(Point3::origin()),
        half_angle: core::f64::consts::FRAC_PI_4,
    };
    let apex_in_zero_band = CrossWhy::Pose(PoseRefusal::InBand {
        rung: PoseRung::Apex,
        diag: zero,
    });
    let unsupported = method(MeetFault::Unsupported {
        pose: "a row's pose",
    });
    let edge = |why| {
        super::crossing_refused(
            why,
            &ring(),
            &cone,
            (Operand::B, crate::entity::FaceKey::default()),
            &|_, _| Err(frontier()),
        )
    };
    assert!(
        matches!(edge(apex_in_zero_band), Ok(Verdict::AtApex)),
        "{:?}",
        edge(apex_in_zero_band)
    );
    assert!(
        matches!(edge(unsupported), Ok(Verdict::Unsettled)),
        "{:?}",
        edge(unsupported)
    );
    assert!(matches!(
        refused(apex_in_zero_band),
        CoveredPosture::Frontier
    ));
    assert!(matches!(refused(unsupported), CoveredPosture::Frontier));
}
