//! **A line or a circle through a point of an ellipse held on its arc**
//! (`work/oneround/an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre.md`):
//! the ellipse's centre is derived a megametre off and rounded at its
//! semi-axes' scale, and every reading of where it is goes through the
//! conic from its anchor. Each pose is met at the point it was built
//! through, and the same pose lifted `1000ε` off is read apart; both at
//! `f64` and at `Interval`, which may refuse but never answers wrongly.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom::{Conic, Curve3, Ellipse};
use geom_core::{Band, Interval, Point3, Real, Tol, Vec3};

use super::{Span, meet};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The stored numbers `plane_cone_section` mints on the arc-held ladder.
fn section(a: f64) -> Ellipse<f64> {
    Ellipse::anchored(
        Point3::new(1.3, -0.7, 0.4),
        Vec3::new(0.5522240050794545, 0.3110026418424273, 0.7735153553621528),
        Vec3::new(0.6816514125372342, -0.702621773621506, -0.20414209516648582),
        a,
        0.3 * a,
    )
}

/// What a meeting reads: `None` refused, else the meetings' parameters
/// on the ellipse.
fn met<T: geom_core::Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    on_ellipse_first: bool,
    band: Band,
) -> Option<Vec<T>> {
    let m = meet(a, b, band).ok()?;
    Some(
        m.points
            .iter()
            .map(|p| {
                if on_ellipse_first {
                    a.at(p.on_a)
                } else {
                    b.at(p.on_b)
                }
            })
            .collect(),
    )
}

fn mid(x: Interval) -> f64 {
    0.5 * (geom_core::Bounds::lo(x) + geom_core::Bounds::hi(x))
}

/// Checks one pose at both scalars: `through` the parameter the pose
/// was built through, `apart` whether it was lifted off, `speed` the
/// ellipse's there.
fn reads(
    (a, sa): (&Curve3<f64>, (f64, f64)),
    (b, sb): (&Curve3<f64>, (f64, f64)),
    ellipse_first: bool,
    (through, apart, speed): (f64, bool, f64),
    what: &str,
) {
    let band = band();
    let e = band.escalate();
    let read = met(&Span::new(a, sa), &Span::new(b, sb), ellipse_first, band);
    let check = |got: &[f64], lane: &str| {
        if apart {
            assert!(
                got.is_empty(),
                "{what} at {lane}: lifted off, read meeting at {got:?}"
            );
        } else {
            assert!(
                got.iter().any(|t| (t - through).abs() * speed <= e * 10.0),
                "{what} at {lane}: built through {through:e}, met at {got:?}"
            );
        }
    };
    match &read {
        Some(got) => check(got, "f64"),
        None => panic!("{what}: refused at f64"),
    }
    let lift = |c: &Curve3<f64>| c.map_scalar(<Interval as Real>::from_f64);
    let i = <Interval as Real>::from_f64;
    let (ai, bi) = (lift(a), lift(b));
    if let Some(got) = met(
        &Span::new(&ai, (i(sa.0), i(sa.1))),
        &Span::new(&bi, (i(sb.0), i(sb.1))),
        ellipse_first,
        band,
    ) {
        check(&got.into_iter().map(mid).collect::<Vec<_>>(), "Interval");
    }
}

/// **The reproduction**: a line through a point of the ellipse, in its
/// plane, square to the arc `0.3 m` from the anchor, meets it there; and
/// lifted `1000ε` off the plane it does not. The base answered `Ok` with
/// no meeting from `a = 1e6` at `1e-12` and at `1e9` at `1e-9`: the
/// router read the line's height above the plane from the derived
/// centre.
#[test]
fn a_line_through_a_point_of_an_arc_held_ellipse_meets_it() {
    let lift = 1000.0 * band().zero();
    for a in [1e6, 1e9] {
        let e = section(a);
        let theta = 0.3 / (0.3 * a);
        let p = e.point_at(theta);
        let tan = e.derivative_at(theta).normalize();
        let d = e.axis().cross(tan).normalize();
        let carrier = Curve3::Ellipse(e);
        let half = 10.0 / (0.3 * a);
        for (off, apart) in [(0.0, false), (lift, true)] {
            let line = Curve3::line(p - d * 2.0 + e.axis() * off, d);
            reads(
                (&line, (0.0, 4.0)),
                (&carrier, (theta - half, theta + half)),
                false,
                (theta, apart, 0.3 * a),
                &format!("a = {a:e}, the line lifted {off:e}"),
            );
        }
    }
}

/// **A unit circle through a point of the ellipse, across its plane**,
/// meets it there, and moved `1000ε` along the pair's common normal it
/// does not. The base answered `Ok` with no meeting from `a = 1e8` at
/// `1e-9`: each conic's height above the other's plane was read from
/// the centres, and the window from the derived one.
#[test]
fn a_circle_through_a_point_of_an_arc_held_ellipse_meets_it() {
    let lift = 1000.0 * band().zero();
    for a in [1e8, 1e9] {
        let e = section(a);
        let theta = 0.3 / (0.3 * a);
        let p = e.point_at(theta);
        let tan = e.derivative_at(theta).normalize();
        let across = e.axis().cross(tan).normalize();
        let w = (across * 0.7_f64.cos() + e.axis() * 0.7_f64.sin()).normalize();
        let normal = (tan * 0.5_f64.sin() + w * 0.5_f64.cos()).normalize();
        let in_plane = normal.cross(w.cross(tan)).normalize();
        let common = tan.cross(normal.cross(in_plane)).normalize();
        let carrier = Curve3::Ellipse(e);
        let half = 10.0 / (0.3 * a);
        for (off, apart) in [(0.0, false), (lift, true)] {
            let circle = Curve3::circle(p - in_plane + common * off, normal, 1.0, in_plane);
            reads(
                (&carrier, (theta - half, theta + half)),
                (&circle, (-1.0, 1.0)),
                true,
                (theta, apart, 0.3 * a),
                &format!("a = {a:e}, the circle moved {off:e}"),
            );
        }
    }
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
        let e = section(a);
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

/// The seven families of the arc-held suspects, each through `p` at
/// `0.3 m` of arc from the anchor and each with a twin `30ε` apart:
/// `(pose, twin)` for each, `a` the ellipse's larger semi-axis.
struct Near {
    e: Ellipse<f64>,
    carrier: Curve3<f64>,
    p: Point3<f64>,
    tan: Vec3<f64>,
    n: Vec3<f64>,
    d: Vec3<f64>,
    span: (f64, f64),
}

fn near(a: f64) -> Near {
    let e = section(a);
    let theta = 0.3 / (0.3 * a);
    let (p, tan, n) = (
        e.point_at(theta),
        e.derivative_at(theta).normalize(),
        e.axis(),
    );
    let half = 10.0 / (0.3 * a);
    Near {
        e,
        carrier: Curve3::Ellipse(e),
        p,
        tan,
        n,
        d: tan.cross(n).normalize(),
        span: (theta - half, theta + half),
    }
}

/// `other` over `span` against the ellipse over its span: the meetings'
/// points, or the refusal.
fn points(
    f: &Near,
    other: &Curve3<f64>,
    span: (f64, f64),
) -> Result<Vec<Point3<f64>>, super::MeetFault> {
    meet(
        &Span::new(other, span),
        &Span::new(&f.carrier, f.span),
        band(),
    )
    .map(|m| m.points.iter().map(|p| p.point).collect())
    .map_err(|r| r.fault)
}

type Read = Result<Vec<Point3<f64>>, super::MeetFault>;

/// A pose that meets at `at` and its twin that does not, both answered.
fn pair(what: &str, met: Read, at: Point3<f64>, twin: Read) {
    pair_refusing(what, met, at, twin, &[]);
}

/// [`pair`] where the meeting may also refuse on one of the meter rows
/// `may`, and never answer no meeting.
fn pair_refusing(what: &str, met: Read, at: Point3<f64>, twin: Read, may: &[&str]) {
    let reach = 10.0 * band().escalate();
    match met {
        Ok(pts) => assert!(
            pts.iter().any(|q| (*q - at).norm() <= reach),
            "{what}: met at {pts:?}, not at {at:?}"
        ),
        Err(super::MeetFault::Unsettled(crate::roots::Unsettled::Meter { row }))
            if may.contains(&row) => {}
        Err(fault) => panic!("{what}: refused, {fault:?}"),
    }
    match twin {
        Ok(pts) => assert!(pts.is_empty(), "{what}, 30ε apart: met at {pts:?}"),
        Err(fault) => panic!("{what}, 30ε apart: refused, {fault:?}"),
    }
}

/// **A line piercing the plane through a point of the arc, 30° off the
/// plane, meets it**, and moved `30ε` off the arc does not. The base
/// answered `Ok` with no meeting (`curve_pair_pose_clear`: the polish's
/// clearance read about the derived centre).
#[test]
fn a_line_piercing_the_plane_through_a_point_of_the_arc_meets_it() {
    let thirty = 30.0 * band().zero();
    for a in [1e6, 1e9] {
        let f = near(a);
        let g = 30.0_f64.to_radians();
        let dir = (f.d * g.cos() + f.n * g.sin()).normalize();
        let line = |at: Point3<f64>| Curve3::line(at - dir * 2.0, dir);
        pair(
            &format!("a = {a:e}"),
            points(&f, &line(f.p), (0.0, 4.0)),
            f.p,
            points(&f, &line(f.p + f.d * (thirty / g.sin())), (0.0, 4.0)),
        );
    }
}

/// **A line along the major axis out through the anchor vertex meets it
/// there**, and one starting `30ε` outside the vertex does not. The base
/// read the conic's extent on the line from its derived centre
/// (`curve_pair_pose_gap`, `curve_pair_pose_parallel`).
#[test]
fn a_line_out_through_the_anchor_vertex_meets_it() {
    let thirty = 30.0 * band().zero();
    for a in [1e6, 1e9] {
        let f = near(a);
        let (vertex, u) = (f.e.point_at(0.0), f.e.u_ref());
        let half = 10.0 / (0.3 * a);
        let read = |start: Point3<f64>| {
            let line = Curve3::line(start, u);
            meet(
                &Span::new(&line, (0.0, 2.0)),
                &Span::new(&f.carrier, (-half, half)),
                band(),
            )
            .map(|m| m.points.iter().map(|p| p.point).collect())
            .map_err(|r| r.fault)
        };
        pair(
            &format!("a = {a:e}"),
            read(vertex - u * thirty),
            vertex,
            read(vertex + u * thirty),
        );
    }
}

/// **A unit circle in the ellipse's plane crossing the arc meets it**,
/// and lifted `30ε` off the plane does not. The base read each conic's
/// height above the other's plane from the centres
/// (`curve_pair_pose_parallel`, `Pose::Apart`).
#[test]
fn a_circle_in_the_plane_crossing_the_arc_meets_it() {
    let thirty = 30.0 * band().zero();
    for a in [1e6, 1e9] {
        let f = near(a);
        let circle = |lift: f64| Curve3::circle(f.p + f.tan * 0.6 + f.n * lift, f.n, 1.0, -f.tan);
        let met = points(&f, &circle(0.0), (-1.0, 1.0));
        let at = met
            .as_ref()
            .ok()
            .and_then(|pts| pts.first().copied())
            .unwrap_or(f.p);
        assert!(
            f.carrier.clearance(at).unwrap() <= band().escalate(),
            "a = {a:e}: {met:?}"
        );
        pair(
            &format!("a = {a:e}"),
            met,
            at,
            points(&f, &circle(thirty), (-1.0, 1.0)),
        );
    }
}

/// **A small ellipse held at its centre, in the plane and crossing the
/// arc, meets it**, and lifted `30ε` does not. The base read the
/// constant from the derived centre, and the coplanar subdivision's
/// residual about it (`curve_pair_coplanar_sub_clear`).
#[test]
fn a_small_ellipse_in_the_plane_crossing_the_arc_meets_it() {
    let thirty = 30.0 * band().zero();
    for a in [1e6, 1e9] {
        let f = near(a);
        let small = |lift: f64| {
            Curve3::Ellipse(Ellipse::new(
                geom::Frame::new(f.p + f.tan * 1.2 + f.n * lift, f.n, -f.tan),
                2.0,
                1.0,
            ))
        };
        let met = points(&f, &small(0.0), (-1.0, 1.0));
        let at = met
            .as_ref()
            .ok()
            .and_then(|pts| pts.first().copied())
            .unwrap_or(f.p);
        assert!(
            f.carrier.clearance(at).unwrap() <= band().escalate(),
            "a = {a:e}: {met:?}"
        );
        pair(
            &format!("a = {a:e}"),
            met,
            at,
            points(&f, &small(thirty), (-1.0, 1.0)),
        );
    }
}

/// **Two arc-held ellipses of one size through one point meet there**,
/// in one plane and across planes, and moved `30ε` apart do not. In one
/// plane the base read the constant with a derived centre on both sides;
/// across planes the window and the root gap. In one plane the meeting
/// may refuse on the polish's basin (`curve_pair_pose_basin`).
#[test]
fn two_arc_held_ellipses_through_one_point_meet() {
    use geom_core::Affine3;
    let thirty = 30.0 * band().zero();
    for a in [1e6, 1e9] {
        let f = near(a);
        let (anchor, axis, toward) = (
            Point3::new(1.3, -0.7, 0.4),
            Vec3::new(0.5522240050794545, 0.3110026418424273, 0.7735153553621528),
            Vec3::new(0.6816514125372342, -0.702621773621506, -0.20414209516648582),
        );
        for (name, about) in [("in one plane", f.n), ("across planes", f.d)] {
            let m = Affine3::rotation_about_axis(f.p, about, 0.7);
            let turned_tan = m.transform_vec(f.tan);
            let apart = if name == "in one plane" {
                f.n
            } else {
                f.tan.cross(turned_tan).normalize()
            };
            let turned = |by: f64| {
                Curve3::Ellipse(Ellipse::anchored(
                    m.transform_point(anchor) + apart * by,
                    m.transform_vec(axis),
                    m.transform_vec(toward),
                    a,
                    0.3 * a,
                ))
            };
            // In one plane the polish's basin may not hold at the far
            // pair's own scale: a refusal, never an answer of none.
            pair_refusing(
                &format!("a = {a:e}, {name}"),
                points(&f, &turned(0.0), f.span),
                f.p,
                points(&f, &turned(thirty), f.span),
                &["curve_pair_pose_basin"],
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
