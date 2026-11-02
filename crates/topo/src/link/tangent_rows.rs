//! **A link of tangent sectors** ([`FaceSector::tangent`]): a corner's
//! fan as its departures turn, and [`rest::one_material`] over tangent
//! cones, held to the coincidences the faces keep — a ray off every
//! other plane read, a line edge lying in a planar face read as the
//! planar link reads it, a line along a cylinder's ruling kept, two
//! line edges on one ray kept, and a ray along a sphere's tangent plane,
//! a curve's tangent on a line, or two whole tangent planes on one
//! plane refused; and no other reading of a link takes a tangent one.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use geom_core::Tol;

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance is a band")
}

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// `n` distinct edge keys.
fn edges(n: usize) -> Vec<EdgeKey> {
    let mut keys = slotmap::SlotMap::<EdgeKey, ()>::with_key();
    (0..n).map(|_| keys.insert(())).collect()
}

fn plane(n: Vec3<f64>) -> geom::Surface<f64> {
    geom::Surface::Plane {
        origin: Point3::origin(),
        normal: n,
        u_ref: n.orthonormal_basis().0,
    }
}

/// The sphere through the origin with its centre at `(0, 0, c)`, its
/// tangent plane there `z = 0`.
fn sphere(c: f64) -> geom::Surface<f64> {
    geom::Surface::Sphere {
        center: Point3::new(0.0, 0.0, c),
        radius: c.abs(),
        axis: v(0.0, 0.0, 1.0),
        u_ref: v(1.0, 0.0, 0.0),
    }
}

/// The cylinder of radius 1 about the line `x` through `(0, 0, −1)`, its
/// ruling through the origin the `x` axis and its tangent plane there
/// `z = 0`.
fn cylinder() -> geom::Surface<f64> {
    geom::Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, -1.0),
        axis: v(1.0, 0.0, 0.0),
        radius: 1.0,
        u_ref: v(0.0, 0.0, 1.0),
    }
}

/// A whole tangent plane `z = 0` of `surface` at the origin, outward
/// `±z`.
fn whole(surface: geom::Surface<f64>, up: bool) -> FaceSector<f64> {
    let n = v(0.0, 0.0, if up { 1.0 } else { -1.0 });
    let fan = TangentFan::Inside { reach: 2.0 };
    FaceSector::tangent(surface, true, n, Point3::origin(), fan, band()).expect("a whole plane")
}

/// A corner at the origin over `ring` (its rays, around its axis), its
/// material inside it or outside, each face planar unless `curved`
/// names it (by its first ray), each ray along the edge `keys[i]`, a
/// line unless `curves` names it.
fn corner(
    ring: &[Vec3<f64>],
    inside: bool,
    keys: &[EdgeKey],
    curves: &[usize],
) -> Vec<FaceSector<f64>> {
    let axis = ring.iter().fold(Vec3::zero(), |s, &r| s + r);
    let n = ring.len();
    (0..n)
        .map(|k| {
            let (a, b) = (k, (k + 1) % n);
            let m = ring[a].cross(ring[b]).normalize();
            let tag = |i: usize| (keys[i], !curves.contains(&i));
            let (m, d0, d1, bounds) = if (m.dot(axis) < 0.0) == inside {
                (m, ring[a], ring[b], [tag(a), tag(b)])
            } else {
                (-m, ring[b], ring[a], [tag(b), tag(a)])
            };
            let fan = TangentFan::Corner { d0, d1, bounds };
            FaceSector::tangent(plane(m), true, m, Point3::origin(), fan, band())
                .expect("a corner's face")
        })
        .collect()
}

fn read(sectors: Vec<FaceSector<f64>>) -> TouchVerdict {
    rest::one_material(
        &Link::of(Point3::origin(), sectors).expect("a link"),
        band(),
    )
}

/// **A corner's fan turns as its departures do**: one wedge under half
/// a turn; past it, the quarter turns from the first ray, three quarters
/// only where the last ray lies past it; half a turn about the quarter;
/// one direction twice a cusp, refused. Each wedge turns
/// counterclockwise about the normal, under half a turn. Red where a
/// turn reads the other way round, or a cusp builds.
#[test]
fn a_corners_fan_turns_as_its_departures_do() {
    let z = v(0.0, 0.0, 1.0);
    let at = |deg: f64| {
        let t = deg.to_radians();
        v(t.cos(), t.sin(), 0.0) * 2.0
    };
    for (deg, wedges) in [(90.0, 1), (191.0, 3), (300.0, 4), (180.0, 2)] {
        let fan = corner_fan(Point3::origin(), z, at(0.0), at(deg), band()).expect("a fan");
        assert_eq!(fan.len(), wedges, "{deg}°: {fan:?}");
        for &(a, b) in &fan {
            let (a, b) = (a - Point3::origin(), b - Point3::origin());
            assert!(
                a.cross(b).dot(z) > 0.0,
                "{deg}°: ({a:?}, {b:?}) under half a turn"
            );
        }
        let (first, last) = (fan[0].0, fan[wedges - 1].1);
        assert!(
            ((first - Point3::origin()) - at(0.0)).norm() < 1e-12,
            "{deg}°: starts at d0"
        );
        assert!(
            ((last - Point3::origin()) - at(deg)).norm() < 1e-12,
            "{deg}°: ends at d1"
        );
    }
    assert!(
        matches!(
            corner_fan(Point3::origin(), z, at(0.0), at(0.0) * 0.5, band()),
            Err(TouchUnread::Unreadable)
        ),
        "a cusp"
    );
}

/// **A tangent link reads only the coincidences its faces keep**: a
/// corner on a ball's tangent plane rests off the ball and crosses into
/// it; a corner edge along that plane leaves the ball only to second
/// order and refuses, but along a cylinder's ruling it lies in the
/// cylinder and reads; a planar corner's line edge lying in a planar
/// face reads as the planar link does; two line edges on one ray of a
/// curved corner read, a curve's tangent on a line refuses; two balls'
/// tangent planes on one plane refuse. Red where a kept coincidence
/// refuses or one only first order reads.
#[test]
fn a_tangent_link_reads_only_the_coincidences_its_faces_keep() {
    use TouchVerdict::{Crossing, Unread};
    let k = edges(8);
    let up = [v(0.3, 0.1, 1.0), v(-0.2, 0.3, 1.0), v(-0.1, -0.4, 1.0)];
    let down = up.map(|r| v(r.x, r.y, -r.z));
    // Off the ball (its material below `z = 0`) and into it.
    let mut off = corner(&up, true, &k, &[]);
    off.push(whole(sphere(-1.0), true));
    assert!(matches!(read(off), TouchVerdict::Rest(_)), "off the ball");
    let mut into = corner(&down, true, &k, &[]);
    into.push(whole(sphere(-1.0), true));
    assert_eq!(read(into), Crossing, "into the ball");
    // One edge along `x`, in the tangent plane.
    let flat = [v(1.0, 0.0, 0.0), v(-0.2, 0.3, 1.0), v(-0.1, -0.4, 1.0)];
    let mut on_ball = corner(&flat, true, &k, &[]);
    on_ball.push(whole(sphere(-1.0), true));
    assert_eq!(
        read(on_ball),
        Unread(TouchUnread::Unreadable),
        "along a ball"
    );
    let mut on_ruling = corner(&flat, true, &k, &[]);
    on_ruling.push(whole(cylinder(), true));
    assert!(
        matches!(read(on_ruling), TouchVerdict::Rest(_)),
        "along a pin's ruling"
    );
    let mut on_floor = corner(&flat, true, &k, &[]);
    on_floor.push(whole(plane(v(0.0, 0.0, 1.0)), true));
    assert!(
        matches!(read(on_floor), TouchVerdict::Rest(_)),
        "along a floor"
    );
    let mut arc_on_floor = corner(&flat, true, &k, &[0]);
    arc_on_floor.push(whole(plane(v(0.0, 0.0, 1.0)), true));
    assert_eq!(
        read(arc_on_floor),
        Unread(TouchUnread::Unreadable),
        "a curve's tangent along a floor"
    );
    // Two corners on one point, meeting along the ray `x`: edge 0 of
    // the one above, edge 4 of the one below, whose faces are curved.
    // Both lines: kept. Either a curve's tangent: refused.
    let pinch = |below_line: bool, above_line: bool| {
        let above = [v(1.0, 0.0, 0.0), v(-0.2, 0.3, 1.0), v(-0.1, -0.4, 1.0)];
        let below = above.map(|r| v(r.x, r.y, -r.z));
        let curve = |line: bool| if line { vec![] } else { vec![0] };
        let mut s = corner(&above, true, &k, &curve(above_line));
        let mut low = corner(&below, true, &k[4..], &curve(below_line));
        for f in &mut low {
            f.face.surface = cylinder();
            if let Some(t) = f.face.tangent.as_mut() {
                t.plane_exact = false;
            }
        }
        s.extend(low);
        read(s)
    };
    assert!(
        matches!(pinch(true, true), TouchVerdict::Rest(_)),
        "two lines on one ray: {:?}",
        pinch(true, true)
    );
    assert_eq!(
        pinch(false, true),
        Unread(TouchUnread::Unreadable),
        "a line on a curve's tangent"
    );
    assert_eq!(
        pinch(true, false),
        Unread(TouchUnread::Unreadable),
        "a curve's tangent on a line"
    );
    // Two balls kissing at the origin, and a ball in a socket.
    for (c, up) in [(1.0, false), (-2.0, true)] {
        let s = vec![whole(sphere(-1.0), true), whole(sphere(c), up)];
        assert_eq!(read(s), Unread(TouchUnread::Unreadable), "two balls, {c}");
    }
    // No reading but the one-material one takes a tangent link.
    let tangent = Link::of(Point3::origin(), corner(&up, true, &k, &[])).unwrap();
    let ball = Round::of(&sphere(-1.0), true).unwrap();
    assert_eq!(
        rest::judge(&tangent, &tangent, band()),
        Unread(TouchUnread::Unreadable),
        "judged"
    );
    assert_eq!(
        rest::by_round(&tangent, &ball, 1.0, band()),
        Unread(TouchUnread::Unreadable),
        "against a ball"
    );
}
