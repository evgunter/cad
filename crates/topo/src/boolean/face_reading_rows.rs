//! **The face reading's rows** ([`super::face_crossings`],
//! [`super::window_off_face`], [`super::ball_off_face`]): a window off the
//! face is no event, and every window the face may hold refuses, by the
//! window's own why.

use super::{
    FaceCrossings, FaceMeeting, MedialDistance, ball_off_face, face_crossings, window_off_face,
};
use crate::body::Body;
use crate::boolean::BooleanError;
use crate::contain::{CurvedPlacement, FaceContainment};
use crate::entity::FaceKey;
use crate::test_support_fixtures::{CylFrame, cone_cap, cyl_wall_sheet, set_sheet_sense};
use geom::Curve3;
use geom_brep::curve_face::{
    CrossRefusal, CrossWhy, Meeting, MeetingKind, OnSpan, PoseRefusal, PoseRung, Undecided,
};
use geom_brep::curve_pair::Span;
use geom_core::{Band, Point3, Tol, Vec3};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap_or_else(|e| panic!("the witness band: {e:?}"))
}

/// The unit wall about `z`, as a surface.
fn unit_wall() -> geom::Surface<f64> {
    geom::Surface::Cylinder {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        radius: 1.0,
    }
}

/// **The wall face**: the unit wall over azimuth `[−½, ½]` and height
/// `[0, 1]`, its sense set so it holds that patch: a point of the wall at
/// azimuth 0 reads `In`, one at azimuth `π` reads `Out` (checked).
fn wall_face() -> (Body<f64>, FaceKey) {
    let mut y = Body::new();
    let face = cyl_wall_sheet(
        &mut y,
        CylFrame::canonical(1.0),
        (-0.5, 0.5),
        (0.0, 1.0),
        tol(),
    );
    set_sheet_sense(&mut y, face, true, tol());
    let at = |u: f64| {
        let p = Point3::new(u.cos(), u.sin(), 0.5);
        let held = crate::held::Held::constructed(p, "a point of the wall");
        crate::contain::placement_on_carrier(&y, face, held, band()).map(|(at, _, _)| at)
    };
    assert!(
        matches!(
            at(0.0),
            Ok(CurvedPlacement::Trim(Some(FaceContainment::In)))
        ),
        "the fixture holds azimuth 0: {:?}",
        at(0.0)
    );
    assert!(
        matches!(
            at(core::f64::consts::PI),
            Ok(CurvedPlacement::Trim(Some(FaceContainment::Out)))
        ),
        "the fixture leaves azimuth π out: {:?}",
        at(core::f64::consts::PI)
    );
    (y, face)
}

/// The line along `y` through `(x, 0, z)`, its parameter `y`.
fn line_along_y(x: f64, z: f64) -> Curve3<f64> {
    Curve3::Line(geom::Line {
        origin: Point3::new(x, 0.0, z),
        dir: geom::stated_direction(Vec3::unit_y()),
    })
}

/// A window's why: a rung read in band (the pose's discriminant).
fn in_band_why() -> CrossWhy {
    CrossWhy::Pose(PoseRefusal::InBand {
        rung: PoseRung::Discriminant,
        diag: geom_core::Indeterminate {
            margin: geom_core::MarginDiag::value((band().zero() + band().escalate()) / 2.0),
            band: band(),
            predicate: Some("curve_face_pose_disc"),
            terminal_sliver: false,
        },
    })
}

/// `cross`'s refusal with the one window `window` and nothing found.
fn one_window(
    window: (f64, f64),
) -> Result<geom_brep::curve_face::Crossings<f64>, CrossRefusal<f64>> {
    Err(CrossRefusal {
        found: Vec::new(),
        windows: vec![Undecided {
            window,
            why: in_band_why(),
        }],
    })
}

/// **A window off the face, its ball clear of the boundary, answers no
/// event.** A line beside the wall at azimuth `π`, half the band outside
/// it, refused over a short window about its nearest point: the window's
/// ball, about its foot at azimuth `π`, holds no vertex or edge of the
/// face and its foot reads `Out`, so the reading sets it aside and the
/// span meets the face nowhere (`Met` with no meeting, windowed).
///
/// Mutant: the boundary check skipped, green here and red on the next
/// row.
#[test]
fn a_window_off_the_face_with_its_ball_clear_of_the_boundary_is_no_event() {
    let (y, face) = wall_face();
    let line = line_along_y(-1.0 - band().zero() / 2.0, 0.5);
    let span = Span::new(&line, (-0.5, 0.5));
    let window = (-0.01, 0.01);
    assert!(
        window_off_face(&y, face, &unit_wall(), &line, window, band())
            .unwrap_or_else(|e| panic!("{e:?}")),
        "the window's ball is off the face"
    );
    let got = face_crossings(&y, face, &unit_wall(), &span, one_window(window), band())
        .unwrap_or_else(|e| panic!("{e:?}"));
    assert!(
        matches!(got, FaceCrossings::Met { ref meetings, windowed: true } if meetings.is_empty()),
        "no event, windowed"
    );
}

/// **A window whose ball meets a boundary edge refuses, naming the
/// window's decision.** A line beside the wall at azimuth `0.52`, just
/// past the face's edge at `½`, refused over a window wide enough that
/// its ball reaches back across that edge: the foot reads `Out`, but the
/// edge is inside the ball, so the reading refuses with the window, its
/// why the pose's in-band discriminant.
///
/// Mutant: the boundary check skipped (the foot's `Out` read alone), red.
#[test]
fn a_window_whose_ball_meets_a_boundary_edge_refuses_with_its_why() {
    let (y, face) = wall_face();
    let u = 0.52_f64;
    // The line tangent to the wall at azimuth `u`, half the band off it,
    // its parameter the arc length from the touch.
    let normal = Vec3::new(u.cos(), u.sin(), 0.0);
    let line = Curve3::Line(geom::Line {
        origin: Point3::origin() + normal * (1.0 + band().zero() / 2.0) + Vec3::new(0.0, 0.0, 0.5),
        dir: geom::stated_direction(Vec3::new(-u.sin(), u.cos(), 0.0)),
    });
    let span = Span::new(&line, (-0.5, 0.5));
    let window = (-0.05, 0.05);
    assert!(
        !window_off_face(&y, face, &unit_wall(), &line, window, band())
            .unwrap_or_else(|e| panic!("{e:?}")),
        "the ball holds the edge at azimuth ½"
    );
    let got = face_crossings(&y, face, &unit_wall(), &span, one_window(window), band())
        .unwrap_or_else(|e| panic!("{e:?}"));
    let FaceCrossings::Refused(w) = got else {
        panic!("the window refuses");
    };
    assert_eq!(w.window, window);
    assert!(matches!(
        w.why,
        CrossWhy::Pose(PoseRefusal::InBand {
            rung: PoseRung::Discriminant,
            ..
        })
    ));
}

/// **A window on a cone face refuses.** The cone carries no distance and
/// foot here (its medial axis reaches its apex), so every window refuses,
/// however far from the face: a window about a point half the band off
/// the far nappe, below a cap that holds only the near one.
///
/// Mutant: a cone given a distance (its elevation and the foot along the
/// meridian, its reach unbounded), red: the far nappe's ball reads off
/// the cap.
#[test]
fn a_window_on_a_cone_face_refuses() {
    let alpha = core::f64::consts::FRAC_PI_4;
    let (y, face) = cone_cap(alpha, 1.0, tol());
    let cone = y.proven_face(face).expect_described().clone();
    // A line square to the far nappe's meridian at height −1, half the
    // band outside it.
    let r = alpha.tan();
    let line = Curve3::Line(geom::Line {
        origin: Point3::new(r + band().zero(), 0.0, -1.0),
        dir: geom::stated_direction(Vec3::unit_y()),
    });
    let span = Span::new(&line, (-0.5, 0.5));
    let window = (-0.01, 0.01);
    assert!(
        !window_off_face(&y, face, &cone, &line, window, band())
            .unwrap_or_else(|e| panic!("{e:?}")),
        "a cone window is held"
    );
    assert!(matches!(
        face_crossings(&y, face, &cone, &span, one_window(window), band()),
        Ok(FaceCrossings::Refused(_))
    ));
}

/// **A ball whose radius reads in band against the carrier's reach
/// refuses.** About a foot at azimuth `π`, far from the face, a ball of
/// radius `ρ` less a gap in the band's escalation gap, and one past it:
/// the first may meet the carrier in more than one disc and is not read
/// off; the second, decided below `ρ`, is.
///
/// Mutant: the gap read as fitting (the reach's in-band reading taken as
/// clear), red.
#[test]
fn a_ball_whose_radius_reads_in_band_against_the_reach_refuses() {
    let (y, face) = wall_face();
    let b = band();
    let foot = Point3::new(-1.0, 0.0, 0.5);
    let gap = (b.zero() + b.escalate()) / 2.0;
    let rho = MedialDistance::Bounded(1.0);
    let off = |radius: f64| {
        ball_off_face(&y, face, foot, radius, rho, b).unwrap_or_else(|e| panic!("{e:?}"))
    };
    assert!(
        !off(1.0 - gap),
        "a radius in band against ρ is not read off"
    );
    assert!(off(0.5), "a radius decided below ρ is");
}

/// **A touch whose point is `Out` but whose arc reaches the boundary
/// refuses.** A touch meeting at a point of the wall at azimuth `0.55`,
/// past the face's edge, whose in-band arc runs back across the edge at
/// `½`: its ball holds the edge, so the reading keeps it on the face
/// (`TouchOnFace`), whatever its point reads.
///
/// Mutant: the touch read at its point (placed alone), red: the point is
/// `Out`.
#[test]
fn a_touch_whose_arc_reaches_the_boundary_is_held() {
    let (y, face) = wall_face();
    let u = 0.55_f64;
    let normal = Vec3::new(u.cos(), u.sin(), 0.0);
    let line = Curve3::Line(geom::Line {
        origin: Point3::origin() + normal + Vec3::new(0.0, 0.0, 0.5),
        dir: geom::stated_direction(Vec3::new(-u.sin(), u.cos(), 0.0)),
    });
    let span = Span::new(&line, (-0.5, 0.5));
    let touch = Meeting {
        point: line.eval(0.0),
        on_span: OnSpan::Inside { t: 0.0 },
        kind: MeetingKind::Touch { arc: (-0.08, 0.08) },
    };
    let answer = Err(CrossRefusal {
        found: vec![touch],
        windows: Vec::new(),
    });
    let got = face_crossings(&y, face, &unit_wall(), &span, answer, band())
        .unwrap_or_else(|e| panic!("{e:?}"));
    assert!(
        matches!(got, FaceCrossings::Met { ref meetings, .. } if matches!(meetings.as_slice(), [FaceMeeting::TouchOnFace])),
        "the touch is held on the face"
    );
}

/// **A fault window fails loud**, never set aside, even where its ball is
/// off the face.
#[test]
fn a_fault_window_fails_loud() {
    let (y, face) = wall_face();
    let line = line_along_y(-1.5, 0.5);
    let span = Span::new(&line, (-0.5, 0.5));
    let answer = Err(CrossRefusal {
        found: Vec::new(),
        windows: vec![Undecided {
            window: (-0.01, 0.01),
            why: CrossWhy::Pose(PoseRefusal::Fault("a row's fault")),
        }],
    });
    assert!(matches!(
        face_crossings(&y, face, &unit_wall(), &span, answer, band()),
        Err(BooleanError::ClassificationInvariant {
            what: "a row's fault"
        })
    ));
}
