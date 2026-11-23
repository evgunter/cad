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
