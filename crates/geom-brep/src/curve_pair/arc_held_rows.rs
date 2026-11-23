//! **Lines, circles and conics through a point of an ellipse held on its
//! arc** (`work/oneround/an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre.md`):
//! the ellipse's centre is derived a megametre off and rounded at its
//! semi-axes' scale, and every reading of where it is goes through the
//! conic from its anchor, the seam at the anchor.
//!
//! The sections are the ladder's fat one (`b = 0.3a`) and a thin one
//! (`b = 1.04·√a`, a tip of about a metre's curvature radius), at `a` =
//! 1e6 and 1e9; the poses stand `0.3 m` and `5 m` of arc from the
//! anchor, the witness curves a metre across on the fat sections and a
//! tenth of the tip's radius on the thin ones. Each pose's meeting is
//! placed within the band's escalation of a point computed here: the
//! point it is built through, the vertex, or a Newton solve along the arc
//! of the witness's own equation, never the kernel's meeting or
//! clearance readings. Its twin moved `30ε` apart reads no meeting.
//!
//! No answer, at `f64` or at `Interval`, is ever wrong. An answer is owed
//! wherever the same locus held at its centre answers and is the same
//! locus to a tenth of `ε` (`Case::check`); every other refusal must be
//! a refusal and not a fault, and is printed with what the centre-held
//! twin read.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom::{Conic, Curve3, Ellipse};
use geom_core::{Band, Interval, Point3, Real, Tol, Vec3};

use super::{MeetFault, Span, meet};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const ANCHOR: (f64, f64, f64) = (1.3, -0.7, 0.4);
const AXIS: (f64, f64, f64) = (0.5522240050794545, 0.3110026418424273, 0.7735153553621528);
const TOWARD: (f64, f64, f64) = (0.6816514125372342, -0.702621773621506, -0.20414209516648582);

fn v((x, y, z): (f64, f64, f64)) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// The stored numbers `plane_cone_section` mints on the arc-held ladder,
/// `b` the other semi-axis.
fn section(a: f64, b: f64) -> Ellipse<f64> {
    Ellipse::anchored(
        Point3::new(ANCHOR.0, ANCHOR.1, ANCHOR.2),
        v(AXIS),
        v(TOWARD),
        a,
        b,
    )
}

/// A pose `s` metres of arc from the anchor of the section `(a, b)`.
struct Near {
    a: f64,
    b: f64,
    e: Ellipse<f64>,
    carrier: Curve3<f64>,
    theta: f64,
    p: Point3<f64>,
    tan: Vec3<f64>,
    n: Vec3<f64>,
    d: Vec3<f64>,
    span: (f64, f64),
    /// The same locus held at its centre, as the derived centre reads.
    held: Curve3<f64>,
    /// How far the centre-held twin's point at `θ` stands from `p`.
    held_off: f64,
    /// The witness curves' size: a metre on the fat sections, a tenth of
    /// the tip's radius of curvature `b²/a` on the thin ones, so a witness
    /// crosses the arc once and not the tip's far side.
    r: f64,
}

fn near_on(a: f64, b: f64, s: f64) -> Near {
    let e = section(a, b);
    let theta = s / b;
    let (p, tan, n) = (
        e.point_at(theta),
        e.derivative_at(theta).normalize(),
        e.axis(),
    );
    let r = (0.1 * b * b / a).min(1.0);
    let half = (10.0 * r + 2.0 * s.abs()) / b;
    let held = Ellipse::new(geom::Frame::new(e.center(), e.axis(), e.u_ref()), a, b);
    Near {
        a,
        b,
        e,
        carrier: Curve3::Ellipse(e),
        theta,
        p,
        tan,
        n,
        d: tan.cross(n).normalize(),
        span: (theta - half, theta + half),
        held: Curve3::Ellipse(held),
        held_off: (held.point_at(theta) - p).norm(),
        r,
    }
}

/// The fat section at `0.3 m`, the shape the base answered wrongly on.
fn near(a: f64) -> Near {
    near_on(a, 0.3 * a, 0.3)
}

/// Every section and place the meeting rows read.
fn poses() -> Vec<Near> {
    let mut out = Vec::new();
    for a in [1e6, 1e9] {
        for b in [0.3 * a, 1.04 * a.sqrt()] {
            for s in [0.3, 5.0] {
                out.push(near_on(a, b, s));
            }
        }
    }
    out
}

impl Near {
    fn what(&self) -> String {
        format!("a = {:e}, b = {:e}, θ = {:e}", self.a, self.b, self.theta)
    }

    /// The ellipse's point nearest a curve's zero `g` along the arc
    /// from `θ`, by Newton on `g(P(θ))` with a central difference: the
    /// independent truth for a crossing the pose does not pass through.
    fn crossing(&self, g: impl Fn(Point3<f64>) -> f64, from: f64) -> Point3<f64> {
        let mut t = from;
        let h = 1e-4 / self.b;
        for _ in 0..60 {
            let (gm, gp) = (g(self.e.point_at(t - h)), g(self.e.point_at(t + h)));
            let step = g(self.e.point_at(t)) / ((gp - gm) / (2.0 * h));
            t -= step;
            if (step * self.b).abs() < 1e-18 {
                break;
            }
        }
        self.e.point_at(t)
    }
}

/// A reading's meeting points, or its refusal.
type Read = Result<Vec<Point3<f64>>, MeetFault>;

/// `x` over `sx` against `y` over `sy`, at `f64` and at `Interval` (the
/// middles of the meetings' enclosures).
fn read(x: &Curve3<f64>, sx: (f64, f64), y: &Curve3<f64>, sy: (f64, f64)) -> (Read, Read) {
    let band = band();
    let at = meet(&Span::new(x, sx), &Span::new(y, sy), band)
        .map(|m| m.points.iter().map(|p| p.point).collect())
        .map_err(|r| r.fault);
    let i = <Interval as Real>::from_f64;
    let mid = |x: Interval| 0.5 * (geom_core::Bounds::lo(x) + geom_core::Bounds::hi(x));
    let (xi, yi) = (x.map_scalar(i), y.map_scalar(i));
    let at_i = meet(
        &Span::new(&xi, (i(sx.0), i(sx.1))),
        &Span::new(&yi, (i(sy.0), i(sy.1))),
        band,
    )
    .map(|m| {
        m.points
            .iter()
            .map(|p| Point3::new(mid(p.point.x), mid(p.point.y), mid(p.point.z)))
            .collect()
    })
    .map_err(|r| r.fault);
    (at, at_i)
}

/// The row a refusal names: a meter's row, or a gap decision's
/// predicate.
fn named(fault: &MeetFault) -> Option<&'static str> {
    match fault {
        MeetFault::Unsettled(crate::roots::Unsettled::Meter { row }) => Some(row),
        MeetFault::Escalated(i) | MeetFault::Touch(i) => i.predicate,
        _ => None,
    }
}

/// A refusal's kind and row, for a red row's message.
fn brief(fault: &MeetFault) -> String {
    let kind = match fault {
        MeetFault::Escalated(_) => "Escalated",
        MeetFault::Touch(_) => "Touch",
        MeetFault::Unsettled(_) => "Unsettled",
        _ => "other",
    };
    match named(fault) {
        Some(row) => format!("{kind}({row})"),
        None => format!("{fault:?}"),
    }
}

/// One family's pose: a witness curve lifted by `lift` along the
/// direction its twin is moved, over `wspan`, read against the ellipse
/// over `espan`, the ellipse first or second.
struct Case<'a, W> {
    f: &'a Near,
    what: String,
    witness: W,
    wspan: (f64, f64),
    espan: (f64, f64),
    ellipse_first: bool,
    at: Point3<f64>,
}

impl<W: Fn(f64) -> Curve3<f64>> Case<'_, W> {
    fn read(&self, ellipse: &Curve3<f64>, lift: f64) -> (Read, Read) {
        let w = (self.witness)(lift);
        if self.ellipse_first {
            read(ellipse, self.espan, &w, self.wspan)
        } else {
            read(&w, self.wspan, ellipse, self.espan)
        }
    }

    /// The pose meets within the band's escalation of `at`, and its twin
    /// `30ε` apart does not, at `f64` and at `Interval`; an answer is
    /// never wrong. **An answer is owed wherever the same locus held at
    /// its centre answers, where that is the same locus**: where the
    /// centre-held twin's point stands within a tenth of `ε` of this
    /// one's, a refusal passes only if the twin refuses the pose too (the
    /// pose's shape refused, not where the ellipse is held). Where the
    /// twin is a different locus (its derived centre rounded past that,
    /// at `a = 1e9` from `1e-9` and at `1e6` at `1e-12`) nothing gauges
    /// what is owed, and a refusal passes only as a refusal, not a
    /// fault; each is printed. Every failure is returned.
    fn check(&self) -> Vec<String> {
        let e = band().escalate();
        let thirty = 30.0 * band().zero();
        let gauged = self.f.held_off <= 0.1 * band().zero();
        let mut out = Vec::new();
        for (lift, meets) in [(0.0, true), (thirty, false)] {
            let (at_f, at_i) = self.read(&self.f.carrier, lift);
            let (held_f, held_i) = self.read(&self.f.held, lift);
            for (lane, r, held) in [("f64", at_f, held_f), ("Interval", at_i, held_i)] {
                let what = format!(
                    "{}{} at {lane}",
                    self.what,
                    if meets { "" } else { ", 30ε apart," }
                );
                match (r, held) {
                    (Ok(pts), _) if meets && pts.iter().any(|q| (*q - self.at).norm() <= e) => {}
                    (Ok(pts), _) if !meets && pts.is_empty() => {}
                    (Ok(pts), _) => out.push(format!(
                        "{what}: met at {pts:?}, owed {}",
                        if meets {
                            format!("a meeting within {e:e} of {:?}", self.at)
                        } else {
                            "none".to_owned()
                        }
                    )),
                    (Err(fault), _) if !refusal(&fault) => {
                        out.push(format!("{what}: a fault, {fault:?}"))
                    }
                    (Err(fault), Ok(_)) if gauged => out.push(format!(
                        "{what}: refused, {}, where the centre-held twin, {:e} off, answers",
                        brief(&fault),
                        self.f.held_off
                    )),
                    (Err(fault), held) => eprintln!(
                        "excused: {what}: refused, {}; centre-held twin {:e} off read {:?}",
                        brief(&fault),
                        self.f.held_off,
                        held.map(|p| p.len()).map_err(|h| brief(&h))
                    ),
                }
            }
        }
        out
    }
}

/// A refusal a pose may earn: a decision in the band's gap, a meter or a
/// root finder's limit. Not a fault, a contradiction or a desync.
fn refusal(fault: &MeetFault) -> bool {
    matches!(
        fault,
        MeetFault::Escalated(_) | MeetFault::Touch(_) | MeetFault::Unsettled(_)
    )
}

/// Every pose's failures, red together.
fn all_green(failures: Vec<String>) {
    assert!(
        failures.is_empty(),
        "{} failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// **The reproduction**: a line through a point of the ellipse, in its
/// plane, square to the arc, meets it there; and lifted `30ε` off the
/// plane it does not. The base answered `Ok` with no meeting from `a =
/// 1e6` at `1e-12` and at `1e9` at `1e-9`: the router read the line's
/// height above the plane from the derived centre.
#[test]
fn a_line_through_a_point_of_an_arc_held_ellipse_meets_it() {
    let mut failed = Vec::new();
    for f in poses() {
        let line = |lift: f64| {
            Curve3::line(
                f.p - f.d * (2.0 * f.r) + f.n * lift,
                geom_core::UnitVec3::checked(f.d),
            )
        };
        failed.extend(
            Case {
                f: &f,
                what: f.what(),
                witness: line,
                wspan: (0.0, 4.0 * f.r),
                espan: f.span,
                ellipse_first: false,
                at: f.p,
            }
            .check(),
        );
    }
    all_green(failed);
}

/// **A unit circle through a point of the ellipse, across its plane**,
/// meets it there, and moved `30ε` along the pair's common normal it
/// does not. The base answered `Ok` with no meeting from `a = 1e8` at
/// `1e-9`: each conic's height above the other's plane was read from
/// the centres, and the window from the derived one.
#[test]
fn a_circle_through_a_point_of_an_arc_held_ellipse_meets_it() {
    let mut failed = Vec::new();
    for f in poses() {
        let w = (f.d * 0.7_f64.cos() + f.n * 0.7_f64.sin()).normalize();
        let normal = (f.tan * 0.5_f64.sin() + w * 0.5_f64.cos()).normalize();
        let in_plane = normal.cross(w.cross(f.tan)).normalize();
        let common = f.tan.cross(normal.cross(in_plane)).normalize();
        let circle =
            |off: f64| Curve3::circle(f.p - in_plane * f.r + common * off, normal, f.r, in_plane);
        failed.extend(
            Case {
                f: &f,
                what: f.what(),
                witness: circle,
                wspan: (-1.0, 1.0),
                espan: f.span,
                ellipse_first: true,
                at: f.p,
            }
            .check(),
        );
    }
    all_green(failed);
}

/// **A line piercing the plane through a point of the arc, 30° off the
/// plane, meets it**, and moved `30ε` off the arc does not. The base
/// answered `Ok` with no meeting (`curve_pair_pose_clear`: the polish's
/// clearance read about the derived centre).
#[test]
fn a_line_piercing_the_plane_through_a_point_of_the_arc_meets_it() {
    let g = 30.0_f64.to_radians();
    let mut failed = Vec::new();
    for f in poses() {
        let dir = (f.d * g.cos() + f.n * g.sin()).normalize();
        let line = |at: Point3<f64>| {
            Curve3::line(at - dir * (2.0 * f.r), geom_core::UnitVec3::checked(dir))
        };
        failed.extend(
            Case {
                f: &f,
                what: f.what(),
                witness: |by: f64| line(f.p + f.d * (by / g.sin())),
                wspan: (0.0, 4.0 * f.r),
                espan: f.span,
                ellipse_first: false,
                at: f.p,
            }
            .check(),
        );
    }
    all_green(failed);
}

/// **A line along the major axis out through the anchor vertex meets it
/// there**, and one starting `30ε` outside the vertex does not. The base
/// read the conic's extent on the line from its derived centre
/// (`curve_pair_pose_gap`, `curve_pair_pose_parallel`).
#[test]
fn a_line_out_through_the_anchor_vertex_meets_it() {
    let thirty = 30.0 * band().zero();
    let mut failed = Vec::new();
    for f in poses() {
        let (vertex, u) = (f.e.point_at(0.0), f.e.u_ref());
        let half = 10.0 * f.r / f.b;
        let line = |start: Point3<f64>| Curve3::line(start, geom_core::UnitVec3::checked(u));
        failed.extend(
            Case {
                f: &f,
                what: f.what(),
                witness: |by: f64| line(vertex + u * (2.0 * by - thirty)),
                wspan: (0.0, 2.0 * f.r),
                espan: (-half, half),
                ellipse_first: false,
                at: vertex,
            }
            .check(),
        );
    }
    all_green(failed);
}

/// **A unit circle in the ellipse's plane crossing the arc meets it**,
/// where a Newton solve of the circle's own equation along the arc puts
/// the crossing, and lifted `30ε` off the plane does not. The base read
/// each conic's height above the other's plane from the centres
/// (`curve_pair_pose_parallel`, `Pose::Apart`).
#[test]
fn a_circle_in_the_plane_crossing_the_arc_meets_it() {
    let mut failed = Vec::new();
    for f in poses() {
        let centre = f.p + f.tan * (0.6 * f.r);
        let circle = |lift: f64| Curve3::circle(centre + f.n * lift, f.n, f.r, -f.tan);
        let at = f.crossing(|q| (q - centre).norm() - f.r, f.theta - 0.4 * f.r / f.b);
        failed.extend(
            Case {
                f: &f,
                what: f.what(),
                witness: circle,
                wspan: (-1.0, 1.0),
                espan: f.span,
                ellipse_first: false,
                at,
            }
            .check(),
        );
    }
    all_green(failed);
}

/// **A small ellipse held at its centre, in the plane and crossing the
/// arc, meets it** where a Newton solve of its own level along the arc
/// puts the crossing, and lifted `30ε` does not. The base read the
/// constant from the derived centre, and the coplanar subdivision's
/// residual about it (`curve_pair_coplanar_sub_clear`).
#[test]
fn a_small_ellipse_in_the_plane_crossing_the_arc_meets_it() {
    let mut failed = Vec::new();
    for f in poses() {
        let centre = f.p + f.tan * (1.2 * f.r);
        let (u, w) = (-f.tan, f.n.cross(-f.tan));
        let small = |lift: f64| {
            Curve3::Ellipse(Ellipse::new(
                geom::Frame::new(centre + f.n * lift, f.n, -f.tan),
                2.0 * f.r,
                f.r,
            ))
        };
        let level = |q: Point3<f64>| {
            let r = q - centre;
            ((r.dot(u) / 2.0).powi(2) + r.dot(w).powi(2)).sqrt() - f.r
        };
        let at = f.crossing(level, f.theta - 0.8 * f.r / f.b);
        failed.extend(
            Case {
                f: &f,
                what: f.what(),
                witness: small,
                wspan: (-1.0, 1.0),
                espan: f.span,
                ellipse_first: false,
                at,
            }
            .check(),
        );
    }
    all_green(failed);
}

/// **Two arc-held ellipses of one size through one point meet there**,
/// in one plane and across planes, and moved `30ε` apart do not. In one
/// plane the base read the constant with a derived centre on both sides;
/// across planes the window and the root gap.
#[test]
fn two_arc_held_ellipses_through_one_point_meet() {
    two_arc_held(false);
}

/// **The pose the row above leaves out**: across planes on the thin
/// section at `a = 1e9`, whose twin `30ε` apart the `Interval` lane reads
/// for about 70 s at `1e-6` before refusing on `curve_pair_pose_basin`
/// (the centre-held twin refuses on the same row in 0.2 s):
/// `work/oneround/an-interval-meeting-of-two-thin-arc-held-ellipses-spends-a-minute-to-refuse.md`.
#[test]
#[ignore = "slow: about 140 s at 1e-6, the Interval lane's refusal"]
fn two_thin_far_arc_held_ellipses_across_planes_meet() {
    two_arc_held(true);
}

fn two_arc_held(slow: bool) {
    use geom_core::Affine3;
    let mut failed = Vec::new();
    for f in poses() {
        for (name, about) in [("in one plane", f.n), ("across planes", f.d)] {
            if slow != (f.a > 1e8 && f.b < 1e5 && name == "across planes") {
                continue;
            }
            let m = Affine3::rotation_about_axis(f.p, about, 0.7);
            let turned_tan = m.transform_vec(f.tan);
            let apart = if name == "in one plane" {
                f.n
            } else {
                f.tan.cross(turned_tan).normalize()
            };
            let turned = |by: f64| {
                Curve3::Ellipse(Ellipse::anchored(
                    m.transform_point(Point3::new(ANCHOR.0, ANCHOR.1, ANCHOR.2)) + apart * by,
                    m.transform_vec(v(AXIS)),
                    m.transform_vec(v(TOWARD)),
                    f.a,
                    f.b,
                ))
            };
            failed.extend(
                Case {
                    f: &f,
                    what: format!("{}, {name}", f.what()),
                    witness: turned,
                    wspan: f.span,
                    espan: f.span,
                    ellipse_first: false,
                    at: f.p,
                }
                .check(),
            );
        }
    }
    all_green(failed);
}

/// **At `Interval` a window part not decided empty is read**: a unit
/// circle across a mid-sized arc-held ellipse a half or two radians from
/// its anchor, where the root's enclosure is wider than its arc. The
/// window passed over a part whose width straddled zero and answered `Ok`
/// with no meeting; now it meets or refuses.
#[test]
fn an_interval_window_reads_a_part_it_has_not_decided_empty() {
    let band = band();
    let i = <Interval as Real>::from_f64;
    for (a, theta) in [(1e4, 0.5), (1e6, 2.0)] {
        let e = section(a, 0.3 * a);
        let carrier = Curve3::Ellipse(e).map_scalar(i);
        {
            let p = e.point_at(theta);
            let tan = e.derivative_at(theta).normalize();
            let across = e.axis().cross(tan).normalize();
            let w = (across * 0.7_f64.cos() + e.axis() * 0.7_f64.sin()).normalize();
            let in_plane = tan.cross(w.cross(tan)).normalize();
            let circle = Curve3::circle(p - in_plane, tan, 1.0, in_plane).map_scalar(i);
            let got = meet(
                &Span::new(&carrier, (i(theta - 0.1), i(theta + 0.1))),
                &Span::new(&circle, (i(-1.0), i(1.0))),
                band,
            );
            assert!(
                !matches!(&got, Ok(m) if m.points.is_empty() && m.shared.is_none()),
                "a = {a:e}, θ = {theta}: a circle through the ellipse's point read apart at Interval"
            );
        }
    }
}

/// **`OnSurface::under` reads a line of the edge's own plane as in it**,
/// a line `30ε` above it as not, and a line piercing it at 30° as one
/// root at the arc's point. The base stated the plane through the derived
/// centre: `Roots([])` in it, `InPlane` `30ε` above at `a = 1e7`, and the
/// piercing root `1.2e-7` m off at `1e9`.
#[test]
fn an_arc_held_edges_plane_reads_a_line_against_it() {
    use super::on_surface::{Charted, CoSurface, OnSurface, SurfaceCurve};
    let thirty = 30.0 * band().zero();
    for a in [1e6, 1e7, 1e9] {
        let f = near(a);
        let key = crate::SurfaceKey::default();
        let edge = OnSurface::edge(key, &f.carrier, f.span);
        let under = |origin: Point3<f64>, normal: Vec3<f64>, along: Vec3<f64>| {
            let chart = Charted::held(
                key,
                geom::Surface::Plane {
                    frame: geom::Frame::new(origin, normal, along),
                },
            );
            let line = SurfaceCurve::u_line(&chart, geom_core::Point2::new(0.0, 0.0)).unwrap();
            edge.under(&line, (0.0, 4.0), false, band())
        };
        let lies = under(f.p, f.n, f.d);
        assert!(
            matches!(lies, Ok(CoSurface::InPlane)),
            "a = {a:e}, in the plane: {lies:?}"
        );
        let above = under(f.p + f.n * thirty, f.n, f.d);
        assert!(
            matches!(&above, Ok(CoSurface::Roots(r)) if r.is_empty()),
            "a = {a:e}, 30ε above: {above:?}"
        );
        let g = 30.0_f64.to_radians();
        let through = (f.d * g.cos() + f.n * g.sin()).normalize();
        let pierce = under(f.p - through * 2.0, f.tan, through);
        assert!(
            matches!(&pierce, Ok(CoSurface::Roots(r)) if r.len() == 1
                && (r[0].point - f.p).norm() <= band().escalate()),
            "a = {a:e}, piercing at 30°: {pierce:?}"
        );
    }
}

/// **The plane door reads an arc-held ellipse's own plane as its plane**,
/// and the plane `30ε` above it as missed. The base read the conic's
/// height against the plane from its derived centre and answered
/// `Roots([])` for its own plane from `a = 1e9` at `1e-9`, `1e7` at
/// `1e-12`.
#[test]
fn the_plane_door_reads_an_arc_held_ellipses_own_plane() {
    use crate::curve_face::plane_crossing::{
        ConicPlaneMeet, PlaneCrossingLane, plane_crossing_lane,
    };
    let thirty = 30.0 * band().zero();
    for a in [1e7, 1e9] {
        let f = near(a);
        let lane = |lift: f64| {
            plane_crossing_lane(
                &f.carrier,
                f.span.0,
                f.span.1,
                f.e.point_at(0.0) + f.n * lift,
                f.n,
                band(),
            )
        };
        let own = lane(0.0);
        assert!(
            matches!(own, PlaneCrossingLane::Conic(ConicPlaneMeet::InPlane)),
            "a = {a:e}, its own plane: {own:?}"
        );
        let above = lane(thirty);
        assert!(
            matches!(above, PlaneCrossingLane::Conic(ConicPlaneMeet::Miss)),
            "a = {a:e}, 30ε above: {above:?}"
        );
    }
}

/// **The plane door reads a plane grazing an arc-held ellipse at its
/// anchor as one root on the graze's run there**, the ellipse on either
/// side of it, at
/// `f64` and at `Interval`; and the same plane moved `30ε` off the arc,
/// away from it, as missed. The graze's side is read off the height's
/// extremes ([`geom::Height::extremes`]).
#[test]
fn the_plane_door_reads_a_plane_grazing_an_arc_held_ellipse() {
    use crate::curve_face::plane_crossing::{
        ConicPlaneMeet, PlaneCrossingLane, plane_crossing_lane,
    };
    let band = band();
    let thirty = 30.0 * band.zero();
    let i = <Interval as Real>::from_f64;
    for a in [1e3, 1e6, 1e9] {
        let f = near(a);
        let vertex = f.e.point_at(0.0);
        let half = 10.0 / (0.3 * a);
        let lifted = f.carrier.map_scalar(i);
        // A graze's run: the arc within the escalation of the plane
        // about the vertex, `√(2e·ρ)`, `ρ = b²/a` the vertex's radius.
        let run = (2.0 * band.escalate() * (0.3 * a).powi(2) / a).sqrt() / (0.3 * a);
        for sign in [-1.0, 1.0] {
            // The ellipse lies on the side of `−u_ref` from its vertex.
            let n = f.e.u_ref() * sign;
            let at = |off: f64| vertex + f.e.u_ref() * off;
            let got = plane_crossing_lane(&f.carrier, -half, half, at(0.0), n, band);
            assert!(
                matches!(&got, PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Ok(r)))
                    if r.len() == 1 && r[0].abs() <= run),
                "a = {a:e}, normal {sign}·u: {got:?}"
            );
            let away = plane_crossing_lane(&f.carrier, -half, half, at(thirty), n, band);
            assert!(
                matches!(&away, PlaneCrossingLane::Conic(ConicPlaneMeet::Miss)),
                "a = {a:e}, normal {sign}·u, 30ε off: {away:?}"
            );
            let p = at(0.0);
            let got_i = plane_crossing_lane(
                &lifted,
                i(-half),
                i(half),
                Point3::new(i(p.x), i(p.y), i(p.z)),
                Vec3::new(i(n.x), i(n.y), i(n.z)),
                band,
            );
            assert!(
                matches!(&got_i, PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Ok(r)))
                    if r.len() == 1
                        && geom_core::Bounds::lo(r[0]) <= run
                        && -run <= geom_core::Bounds::hi(r[0])),
                "a = {a:e}, normal {sign}·u at Interval: {got_i:?}"
            );
        }
    }
}
