//! **The face reading's rows** ([`super::face_crossings`],
//! [`super::window_off_face`], [`super::ball_off_face`]): a window off the
//! face is no event, and every window the face may hold refuses, by the
//! window's own why.

#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

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
/// window's decision.** A line beside the wall at azimuth `½ + δ`, `δ`
/// a tenth of `√escalate` past the face's edge at `½` (inside the run the
/// line stays within the band of the wall, `√(2·escalate)` about the
/// touch), refused over a window about the touch: the cluster about the
/// touch cannot be cut finer than that run, so its ball reaches back
/// across the edge. The foot reads `Out`, but the edge is inside the
/// ball, so the reading refuses with the window, its why the pose's
/// in-band discriminant.
///
/// Mutant: the boundary check skipped (the foot's `Out` read alone), red.
#[test]
fn a_window_whose_ball_meets_a_boundary_edge_refuses_with_its_why() {
    let (y, face) = wall_face();
    let u = 0.5 + 0.1 * band().escalate().sqrt();
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
/// refuses.** A line touching the unit wall just past the face's edge at
/// `½`, by half the touch's in-band half-width `w = √(2ε)` (the line's
/// residual `t²/2` reaches the band's zero there), its touch meeting
/// carrying the arc `(−w, w)`: the touch point reads `Out`, but the arc
/// runs back across the edge within the band of the wall, so the piece
/// there holds the edge and the reading keeps the touch on the face
/// (`TouchOnFace`).
///
/// Mutant: the touch read at its point (placed alone), red: the point is
/// `Out`.
#[test]
fn a_touch_whose_arc_reaches_the_boundary_is_held() {
    let (y, face) = wall_face();
    let w = (2.0 * band().zero()).sqrt();
    let u = 0.5 + w / 2.0;
    let normal = Vec3::new(u.cos(), u.sin(), 0.0);
    let line = Curve3::Line(geom::Line {
        origin: Point3::origin() + normal + Vec3::new(0.0, 0.0, 0.5),
        dir: geom::stated_direction(Vec3::new(-u.sin(), u.cos(), 0.0)),
    });
    let span = Span::new(&line, (-0.5, 0.5));
    let touch = Meeting {
        point: line.eval(0.0),
        on_span: OnSpan::Inside { t: 0.0 },
        kind: MeetingKind::Touch { arc: (-w, w) },
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

/// **A whole-span window off the face is read by its clusters** where one
/// ball cannot hold it: a line beside the wall at azimuth `π`, half the
/// band outside it, refused over a window four metres long. Its one ball
/// (radius about 2) passes the wall's reach `ρ = 1`; bisected, the pieces
/// far from the wall hold no meeting (`curve_face_window_piece_clear`)
/// and the cluster about its touch reads off the face by its ball, so
/// the window is off the face.
///
/// Mutant: the window read whole (no bisection), red.
#[test]
fn a_long_window_off_the_face_is_read_by_its_pieces() {
    let (y, face) = wall_face();
    let line = line_along_y(-1.0 - band().zero() / 2.0, 0.5);
    let off = window_off_face(&y, face, &unit_wall(), &line, (-2.0, 2.0), band())
        .unwrap_or_else(|e| panic!("{e:?}"));
    assert!(off, "every piece of the window is off the face");
}

/// **A long window one of whose clusters reaches a boundary edge keeps
/// the window**: a line tangent to the wall at azimuth `½ + δ`, `δ` a
/// tenth of `√escalate` past the face's edge, read over a window two
/// metres long. The walk settles the pieces far from the wall, but the
/// cluster about the touch holds the edge however far it is cut, so the
/// window is held.
///
/// Mutant: a cluster whose ball does not clear read as off the face, red.
#[test]
fn a_tiled_window_whose_piece_reaches_the_boundary_is_held() {
    let (y, face) = wall_face();
    let u = 0.5 + 0.1 * band().escalate().sqrt();
    let normal = Vec3::new(u.cos(), u.sin(), 0.0);
    let line = Curve3::Line(geom::Line {
        origin: Point3::origin() + normal * (1.0 + band().zero() / 2.0) + Vec3::new(0.0, 0.0, 0.5),
        dir: geom::stated_direction(Vec3::new(-u.sin(), u.cos(), 0.0)),
    });
    assert!(
        !window_off_face(&y, face, &unit_wall(), &line, (-1.0, 1.0), band())
            .unwrap_or_else(|e| panic!("{e:?}")),
        "the piece about the touch holds the edge at azimuth ½"
    );
}

/// The least distance, found by sampling `(lo, hi)` and refining each
/// sampled minimum by bisection on the slope, from the line `line` to a
/// point of the wall face over azimuth `[−½, ½]` and height `[0, 1]`: the
/// nearest point of the unit wall to each line point, its azimuth and
/// height clamped into the face (a point OF the face, so the distance is
/// one a face point stands at).
fn least_to_face(line: &Curve3<f64>, (lo, hi): (f64, f64)) -> f64 {
    let to_face = |t: f64| {
        let q = line.eval(t);
        let az = q.y.atan2(q.x).clamp(-0.5, 0.5);
        let h = q.z.clamp(0.0, 1.0);
        (q - Point3::new(az.cos(), az.sin(), h)).norm()
    };
    let n = 4000;
    let at = |k: usize| lo + (hi - lo) * k as f64 / n as f64;
    let mut least = f64::INFINITY;
    for k in 0..=n {
        let here = to_face(at(k));
        if here < least {
            least = here;
            // Refine about this sample by golden section.
            let (mut a, mut b) = (at(k.saturating_sub(1)), at((k + 1).min(n)));
            for _ in 0..80 {
                let (x1, x2) = (b - 0.618 * (b - a), a + 0.618 * (b - a));
                if to_face(x1) < to_face(x2) {
                    b = x2;
                } else {
                    a = x1;
                }
            }
            least = least.min(to_face(geom::mid_param(a, b)));
        }
    }
    least
}

/// **The tiled reading never sets aside a window a face point stands
/// in.** Random lines tangent-ish to the unit wall about the face's
/// boundary edges and corners (azimuth drawn about `±½`, height about 0
/// and 1, the touch inside or outside by up to `3ε`), each read over a
/// random window about the touch. Wherever the reading answers off the
/// face, no point of the face, sampled and bisected (`least_to_face`),
/// stands within the band's escalation of the window. At the band rows
/// the run reads (`CAD_TOLERANCE_EPS`).
///
/// Mutant: a piece's clearance read without its reach (`|dist| −
/// escalate`), red.
#[test]
fn the_tiled_reading_never_sets_aside_a_window_a_face_point_stands_in() {
    let (y, face) = wall_face();
    let b = band();
    let mut rng = test_utils::fuzz::start("face-reading-tiling");
    let (mut off, mut held) = (0, 0);
    for case in 0..test_utils::fuzz::scaled(300) {
        let edge = [-0.5, 0.5][case % 2];
        let u = edge + rng.range(-0.15, 0.15);
        let z = if case % 3 == 0 {
            rng.range(-0.2, 1.2)
        } else {
            [0.0, 1.0][case % 2] + rng.range(-0.1, 0.1)
        };
        let normal = Vec3::new(u.cos(), u.sin(), 0.0);
        let tilt = rng.range(-1.0, 1.0);
        let along = (Vec3::new(-u.sin(), u.cos(), 0.0) + Vec3::unit_z() * tilt).normalize();
        let depth = rng.range(-3.0, 3.0) * b.zero();
        let line = Curve3::Line(geom::Line {
            origin: Point3::origin() + normal * (1.0 + depth) + Vec3::unit_z() * z,
            dir: geom::stated_direction(along),
        });
        let half = rng.log_uniform(1e-4, 1.5);
        let centre = rng.range(-half, half);
        let window = (centre - half, centre + half);
        let Ok(read) = window_off_face(&y, face, &unit_wall(), &line, window, b) else {
            held += 1;
            continue;
        };
        if !read {
            held += 1;
            continue;
        }
        off += 1;
        let least = least_to_face(&line, window);
        assert!(
            least > b.escalate(),
            "case {case}: the window {window:?} read off the face, a face point {least:e} from it ({})",
            test_utils::fuzz::replay()
        );
    }
    assert!(off > 20 && held > 20, "{off} read off, {held} held");
}

/// A line tangent to the unit wall at azimuth `u` and height ½, `off`
/// outside it, its parameter the arc length from the touch.
fn tangent_line(u: f64, off: f64) -> Curve3<f64> {
    let normal = Vec3::new(u.cos(), u.sin(), 0.0);
    Curve3::Line(geom::Line {
        origin: Point3::origin() + normal * (1.0 + off) + Vec3::new(0.0, 0.0, 0.5),
        dir: geom::stated_direction(Vec3::new(-u.sin(), u.cos(), 0.0)),
    })
}

/// **A line or a circle tangent to the wall just past the face's edge is
/// read off the face, over a long window**: tangent at azimuth `½ + δ`,
/// `δ` from 0.05 down to 0.001 past the edge (those past three of the
/// touch's in-band runs, `√(2·escalate)`), half the band outside the
/// wall: a line read over four metres about the touch, and a circle of
/// radius 3 in the plane `z = ½`, tangent there from outside, read over
/// its whole turn. The cluster about the touch is cut down to the
/// tangency floor, far inside `δ`, and its ball stands clear of the edge,
/// so the window is off the face at every `δ`, as `off_face`'s clusters
/// read it.
///
/// Mutants: the second-order bound dropped, or the halving stopped at a
/// fixed depth of eight, red (the ball about the touch stays wider than
/// `δ`).
#[test]
fn a_touch_just_past_the_faces_edge_is_off_the_face_over_a_long_window() {
    let (y, face) = wall_face();
    let off = band().zero() / 2.0;
    // A touch's in-band run reaches about `√(2·escalate)` either side of
    // it, and no localizer cuts finer than that run: the rows past three
    // runs from the edge are the ones the band can resolve (all six at
    // the default ε and at 1e-12, the two widest at 1e-6).
    let run = (2.0 * band().escalate()).sqrt();
    let deltas: Vec<f64> = [0.05, 0.02, 0.01, 0.005, 0.002, 0.001]
        .into_iter()
        .filter(|&d| d > 3.0 * run)
        .collect();
    assert!(deltas.len() >= 2, "{deltas:?}");
    for delta in deltas {
        let u = 0.5 + delta;
        let line = tangent_line(u, off);
        assert!(
            window_off_face(&y, face, &unit_wall(), &line, (-2.0, 2.0), band())
                .unwrap_or_else(|e| panic!("δ {delta}: {e:?}")),
            "δ {delta}: the line's window is off the face"
        );
        let normal = Vec3::new(u.cos(), u.sin(), 0.0);
        let circle = Curve3::circle(
            Point3::origin() + normal * (4.0 + off) + Vec3::new(0.0, 0.0, 0.5),
            Vec3::unit_z(),
            3.0,
            Vec3::new(-u.cos(), -u.sin(), 0.0),
        );
        assert!(
            window_off_face(
                &y,
                face,
                &unit_wall(),
                &circle,
                (-core::f64::consts::PI, core::f64::consts::PI),
                band()
            )
            .unwrap_or_else(|e| panic!("δ {delta}: {e:?}")),
            "δ {delta}: the circle's whole turn is off the face"
        );
    }
}

/// **An end-touch's arc inside the span is the span's** (the face
/// reading's touch rule): a line dipping half the band into the wall at
/// azimuth `½ + δ`, `δ` a tenth of `√escalate` past the face's edge, so
/// its touch's in-band arc reaches back across the edge onto the face.
/// Read on a span with the touch strictly inside, the touch is held
/// (`TouchOnFace`). Read on the span ending at the touch, the end's
/// point is the vertex's, but the arc's stretch inside the span still
/// reaches the face: held too, never passed as the end's.
///
/// Mutant: an end-touch read as the end's alone (its arc unread), red.
#[test]
fn an_end_touchs_arc_inside_the_span_is_read_against_the_face() {
    let (y, face) = wall_face();
    let u = 0.5 + 0.1 * band().escalate().sqrt();
    let line = tangent_line(u, -band().zero() / 2.0);
    for span in [(-0.5, 0.5), (-0.5, 0.0)] {
        let span = Span::new(&line, span);
        let answer = geom_brep::curve_face::cross(&span, &unit_wall(), band());
        let got = face_crossings(&y, face, &unit_wall(), &span, answer, band())
            .unwrap_or_else(|e| panic!("{:?}: {e:?}", span.params()));
        let held = match got {
            FaceCrossings::Met { meetings, .. } => meetings
                .iter()
                .any(|m| matches!(m, FaceMeeting::TouchOnFace)),
            FaceCrossings::Refused(_) => true,
            FaceCrossings::Lies | FaceCrossings::Apart => false,
        };
        assert!(
            held,
            "{:?}: the touch's arc reaches the face",
            span.params()
        );
    }
}

/// **A ball is read against a ring torus's reach, `min(r, R − r)`**: a
/// ball of radius 0.6 about a point far off the wall face (azimuth `π`),
/// against the reach of a torus of tube radius 1 about a spine of radius
/// 1.5, `0.5`: past it, the ball is not read off the face, whatever the
/// face. Against a torus of spine radius 3 (reach 1), the same ball is.
///
/// Mutant: the torus's reach read as its tube radius `r`, red.
#[test]
fn a_ball_is_read_against_a_ring_toruss_reach() {
    let (y, face) = wall_face();
    let torus = |major: f64| geom::Surface::Torus {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        major_radius: major,
        minor_radius: 1.0,
    };
    let foot = Point3::new(-1.0, 0.0, 0.5);
    let read = |major: f64| {
        let rho = super::reach(&torus(major)).unwrap_or_else(|| panic!("a torus has a reach"));
        ball_off_face(&y, face, foot, 0.6, rho, band()).unwrap_or_else(|e| panic!("{e:?}"))
    };
    assert!(!read(1.5), "a ball past the reach 0.5 is not read off");
    assert!(read(3.0), "a ball within the reach 1 is read off the face");
}

/// **A piece within the band's escalation of the carrier holds a
/// meeting**: a line beside the wall at azimuth 0, the middle of the
/// face, `1.5·escalate` outside it, read over a short window about its
/// nearest point. Its second-order bound reads about `1.5·escalate` at
/// the touch; less the escalation, it does not clear, and the cluster's
/// ball, about a foot the face holds, keeps the window.
///
/// Mutant: the piece test read without `− escalate`, red (the piece at
/// the touch clears, and the window reads off a face `1.5·escalate`
/// from it).
#[test]
fn a_piece_within_the_escalation_of_the_carrier_is_not_cleared() {
    let (y, face) = wall_face();
    let line = tangent_line(0.0, 1.5 * band().escalate());
    assert!(
        !window_off_face(&y, face, &unit_wall(), &line, (-0.01, 0.01), band())
            .unwrap_or_else(|e| panic!("{e:?}")),
        "the window stands within the escalation of the face"
    );
}

/// **A cluster's ball carries the distance of its middle from the
/// carrier**: a line leaving the plane of a slab's top at 45°, from
/// `(1.3, 0, 0)`, 0.3 past the top's edge at `x = 1`, read over the unit
/// window from that point. A line against a plane has no tangency floor
/// (its level sets and the line are both straight), so the window is one
/// cluster: its middle stands `0.354` above the plane, its reach `ℓ` is
/// 0.5, and its ball of radius `ℓ + |dist| + escalate`, about the foot
/// `0.654` from the edge, holds that edge, so the window is held. Read
/// without `|dist|`, the ball (radius 0.5) stands clear of the edge.
///
/// Mutant: the cluster's ball read without its `|dist|` term, red.
#[test]
fn a_clusters_ball_carries_its_middles_distance_from_the_carrier() {
    let tol = tol();
    let prism = crate::test_support_fixtures::prism_z::<f64>(
        &[(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)],
        -1.0,
        0.0,
        tol,
    );
    let (y, face) = (prism.body, prism.top_face);
    let plane = y.proven_face(face).expect_described().clone();
    assert!(matches!(plane, geom::Surface::Plane { .. }), "{plane:?}");
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let line = Curve3::Line(geom::Line {
        origin: Point3::new(1.3, 0.0, 0.0),
        dir: geom::stated_direction(Vec3::new(s, 0.0, s)),
    });
    assert!(
        !window_off_face(&y, face, &plane, &line, (0.0, 1.0), band())
            .unwrap_or_else(|e| panic!("{e:?}")),
        "the cluster's ball, `ℓ + |dist|` wide, holds the top's edge"
    );
}

/// The least distance from a point to the sphere cap of
/// [`crate::test_support_fixtures::sphere_cap`] at rim height `zr` (the
/// unit sphere below `z = zr`): the radial foot where it lies in the cap,
/// else the rim circle's nearest point, the nearer of the two.
fn to_cap(q: Point3<f64>, zr: f64) -> f64 {
    let r = (1.0 - zr * zr).sqrt();
    let xy = (q.x * q.x + q.y * q.y).sqrt();
    let rim = if xy > 0.0 {
        Point3::new(r * q.x / xy, r * q.y / xy, zr)
    } else {
        Point3::new(r, 0.0, zr)
    };
    let mut least = (q - rim).norm();
    let n = (q - Point3::origin()).norm();
    if n > 0.0 && q.z / n <= zr {
        least = least.min((n - 1.0).abs());
    }
    least
}

/// The least of `to_face` over the carrier's window `(lo, hi)`, sampled
/// and refined about each sampled minimum by golden section.
fn least_over(
    carrier: &Curve3<f64>,
    (lo, hi): (f64, f64),
    to_face: impl Fn(Point3<f64>) -> f64,
) -> f64 {
    let at = |t: f64| to_face(carrier.eval(t));
    let n = 6000;
    let t = |k: usize| lo + (hi - lo) * k as f64 / n as f64;
    let mut least = f64::INFINITY;
    for k in 0..=n {
        let here = at(t(k));
        let left = at(t(k.saturating_sub(1)));
        let right = at(t((k + 1).min(n)));
        if here <= left && here <= right || here < least {
            let (mut a, mut b) = (t(k.saturating_sub(1)), t((k + 1).min(n)));
            for _ in 0..90 {
                let (x1, x2) = (b - 0.618 * (b - a), a + 0.618 * (b - a));
                if at(x1) < at(x2) {
                    b = x2;
                } else {
                    a = x1;
                }
            }
            least = least.min(here).min(at(geom::mid_param(a, b)));
        }
    }
    least
}

/// **The face reading never sets aside a window a face point stands
/// within the escalation of, on a sphere cap or a wall's corners and
/// seams.** A sphere cap (the unit sphere below `z = 0.3`), drawn about
/// its rim, about its pole and over its high cap; the unit wall face over
/// azimuth `[−½, ½]` and height `[0, 1]`, drawn about its corners and its
/// two seams. At each drawn point `p` of the carrier, a line or a circle
/// grazes it at a depth up to `±3000·ε` along the normal: a line along a
/// random tangent, tilted; a circle in the plane of that tangent and the
/// normal, of radius from 0.05 to 3, curving away from the carrier or
/// towards it. Each is read over a window about the graze up to a full
/// turn of a circle. Wherever the reading answers off the face, no point
/// of the face, sampled and golden-refined (`to_cap` exact on the cap; on
/// the wall, the nearest wall point with its azimuth and height clamped
/// into the face), stands within the band's escalation of the window. At
/// the band rows the run reads (`CAD_TOLERANCE_EPS`).
///
/// Mutants, each red: the second-order bound without the level sets'
/// curvature `1/(ρ − |dist| − ℓ)`; without its `ℓ²·H/2` term.
#[test]
fn the_face_reading_never_sets_aside_a_window_a_face_point_stands_in_on_a_cap_or_a_wall() {
    let b = band();
    let zr = 0.3;
    let (cap_body, cap) = crate::test_support_fixtures::sphere_cap(zr, tol());
    let sphere = cap_body.proven_face(cap).expect_described().clone();
    let (wall_body, wall) = wall_face();
    let mut rng = test_utils::fuzz::start("face-reading-cap-and-wall");
    let (mut off, mut held) = ([0_usize; 2], [0_usize; 2]);
    for case in 0..test_utils::fuzz::scaled(600) {
        let on_cap = case % 2 == 0;
        // A point of the carrier near the face's boundary or on it, and
        // the carrier's unit normal there.
        let (p, n) = if on_cap {
            let polar = match case % 6 {
                0 => zr.acos() + rng.range(-0.15, 0.15),
                2 => core::f64::consts::PI - rng.range(0.0, 0.3),
                _ => rng.range(zr.acos() - 0.4, zr.acos()),
            };
            let az = rng.range(0.0, core::f64::consts::TAU);
            let n = Vec3::new(polar.sin() * az.cos(), polar.sin() * az.sin(), polar.cos());
            (Point3::origin() + n, n)
        } else {
            let u = [-0.5, 0.5][case % 4 / 2] + rng.range(-0.15, 0.15);
            let z = if case % 3 == 0 {
                rng.range(-0.2, 1.2)
            } else {
                [0.0, 1.0][case % 4 / 2] + rng.range(-0.1, 0.1)
            };
            let n = Vec3::new(u.cos(), u.sin(), 0.0);
            (Point3::origin() + n + Vec3::unit_z() * z, n)
        };
        let (t1, t2) = n.orthonormal_basis();
        let ang = rng.range(0.0, core::f64::consts::TAU);
        let along = t1 * ang.cos() + t2 * ang.sin();
        let depth = rng.range(-3000.0, 3000.0) * b.zero() * rng.log_uniform(1e-3, 1.0);
        let graze = p + n * depth;
        let (carrier, window) = if rng.unit() < 0.5 {
            let tilt = rng.range(-0.3, 0.3);
            let dir = (along + n * tilt).normalize();
            let half = rng.log_uniform(1e-3, 1.5);
            let centre = rng.range(-half, half);
            (
                Curve3::Line(geom::Line {
                    origin: graze,
                    dir: geom::stated_direction(dir),
                }),
                (centre - half, centre + half),
            )
        } else {
            let radius = rng.log_uniform(0.05, 3.0);
            // Curving away from the carrier (centre outward) or towards it.
            let out = if rng.unit() < 0.5 { 1.0 } else { -1.0 };
            let centre = graze + n * (out * radius);
            let axis = n.cross(along).normalize();
            let circle = Curve3::circle(centre, axis, radius, (graze - centre) * (1.0 / radius));
            let half = rng.range(0.01, core::f64::consts::PI);
            let mid = rng.range(-half, half) * 0.2;
            (circle, (mid - half, mid + half))
        };
        let (body, face, surface) = if on_cap {
            (&cap_body, cap, sphere.clone())
        } else {
            (&wall_body, wall, unit_wall())
        };
        let Ok(read) = window_off_face(body, face, &surface, &carrier, window, b) else {
            held[usize::from(on_cap)] += 1;
            continue;
        };
        if !read {
            held[usize::from(on_cap)] += 1;
            continue;
        }
        off[usize::from(on_cap)] += 1;
        let least = if on_cap {
            least_over(&carrier, window, |q| to_cap(q, zr))
        } else {
            least_over(&carrier, window, |q| {
                let az = q.y.atan2(q.x).clamp(-0.5, 0.5);
                let h = q.z.clamp(0.0, 1.0);
                (q - Point3::new(az.cos(), az.sin(), h)).norm()
            })
        };
        assert!(
            least > b.escalate(),
            "case {case} ({}): the window {window:?} of {carrier:?} read off the face, a face \
             point {least:e} from it ({})",
            if on_cap { "cap" } else { "wall" },
            test_utils::fuzz::replay()
        );
    }
    assert!(
        off.iter().all(|&o| o > 30) && held.iter().all(|&h| h > 30),
        "{off:?} read off, {held:?} held (wall, cap)"
    );
}
