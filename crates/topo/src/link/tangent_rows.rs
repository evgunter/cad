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

/// The sector [`FaceSector::tangent`] reads a face as: a cusp corner it
/// keeps as a horn is no sector, and refuses here.
fn sector<T: Decide>(
    surface: geom::Surface<T>,
    sense: bool,
    outward: Vec3<T>,
    apex: Point3<T>,
    fan_of: TangentFan<T>,
    band: Band,
) -> Result<FaceSector<T>, TouchUnread> {
    match FaceSector::tangent(surface, sense, outward, apex, fan_of, band)? {
        TangentPiece::Sector(sector) => Ok(sector),
        TangentPiece::Horn(_) => Err(TouchUnread::Unreadable),
    }
}

impl<T: Real> CornerTurn<T> {
    /// The fan's wedges; `None` for a horn.
    fn fan(self) -> Option<Vec<Segment<T>>> {
        match self {
            Self::Fan(wedges) => Some(wedges),
            Self::Horn => None,
        }
    }
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
        u_ref: (if n.z.abs() < 0.9 {
            n.cross(v(0.0, 0.0, 1.0))
        } else {
            n.cross(v(1.0, 0.0, 0.0))
        })
        .normalize(),
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

/// The bound along `edge`, a line or a curve with no bend.
fn tag(edge: EdgeKey, line: bool) -> Bound<f64> {
    Bound {
        edge,
        line,
        bend: Vec3::zero(),
    }
}

/// A whole tangent plane `z = 0` of `surface` at the origin, outward
/// `±z`.
fn whole(surface: geom::Surface<f64>, up: bool) -> FaceSector<f64> {
    let n = v(0.0, 0.0, if up { 1.0 } else { -1.0 });
    let fan = TangentFan::Inside {
        reach: 2.0,
        frame: [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)],
    };
    sector(surface, true, n, Point3::origin(), fan, band()).expect("a whole plane")
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
    corner_at(Point3::origin(), ring, inside, keys, curves)
}

/// [`corner`] at `p`.
fn corner_at(
    p: Point3<f64>,
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
            let tag = |i: usize| tag(keys[i], !curves.contains(&i));
            let (m, d0, d1, bounds) = if (m.dot(axis) < 0.0) == inside {
                (m, ring[a], ring[b], [tag(a), tag(b)])
            } else {
                (-m, ring[b], ring[a], [tag(b), tag(a)])
            };
            let fan = TangentFan::Corner {
                d0,
                d1,
                bounds,
                spur: false,
            };
            sector(plane(m), true, m, p, fan, band()).expect("a corner's face")
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
        let fan = corner_fan(
            Point3::origin(),
            z,
            (at(0.0), at(deg)),
            [Vec3::zero(); 2],
            false,
            band(),
        )
        .expect("a fan")
        .fan()
        .expect("a fan");
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
            corner_fan(
                Point3::origin(),
                z,
                (at(0.0), at(0.0) * 0.5),
                [Vec3::zero(); 2],
                false,
                band()
            ),
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

/// A ball of radius `r` about `c`, its axis `a`.
fn ball(c: Point3<f64>, r: f64, a: Vec3<f64>) -> geom::Surface<f64> {
    geom::Surface::Sphere {
        center: c,
        radius: r,
        axis: a,
        u_ref: a.cross(v(0.3, -0.7, 0.2)).normalize(),
    }
}

/// A sphere face's whole tangent plane at `p`, outward `n`, its fan
/// turned off the carrier's own frame as the census turns it
/// ([`crate::census::carrier_frame`]), out to `reach`.
fn whole_at(s: geom::Surface<f64>, n: Vec3<f64>, p: Point3<f64>, reach: f64) -> FaceSector<f64> {
    let frame = crate::census::carrier_frame(&s).expect("an analytic carrier");
    let fan = TangentFan::Inside { reach, frame };
    sector(s, true, n, p, fan, band()).expect("a whole plane")
}

fn read_at(p: Point3<f64>, sectors: Vec<FaceSector<f64>>) -> TouchVerdict {
    rest::one_material(&Link::of(p, sectors).expect("a link"), band())
}

/// **A sphere's whole tangent plane at its pole, its axis tilted, reads
/// by its first order**: its fan lies in the plane however the pole
/// rounds. Two balls kissing there, and a ball in a socket, are one
/// plane to first order and refuse, and so do two kissing off the pole,
/// where the frame's first direction stands half off square to the
/// normal; a corner resting shallow on the
/// ball off its material rests. Near the origin and a kilometre off it,
/// at five tilts of the axis. Red where the fan leaves the plane on
/// rounding (a kiss read as a crossing or degenerate, the rest as a
/// crossing).
#[test]
fn a_sphere_poles_whole_plane_reads_by_its_first_order_at_any_tilt() {
    use TouchVerdict::Unread;
    let k = edges(3);
    let mut bad = Vec::new();
    for raw in [
        [1.0, 2.0, 3.0],
        [0.3, -0.9, 0.1],
        [-2.0, 0.5, 7.0],
        [1.0, 1.0, 1.0],
        [0.1, 0.2, -0.97],
    ] {
        let a = v(raw[0], raw[1], raw[2]).normalize();
        let s0 = a.cross(v(0.3, -0.7, 0.2)).normalize();
        let s1 = a.cross(s0);
        for far in [0.0, 1e3] {
            let p = Point3::new(0.3 + far, -0.2 + 2.0 * far, 0.1);
            let r = 0.7;
            // The ball below `p` along `a`, outward `a` at its pole.
            let below = || whole_at(ball(p - a * r, r, a), a, p, 1.0);
            let kiss = whole_at(ball(p + a * r, r, a), -a, p, 1.0);
            let socket = whole_at(ball(p - a * (2.0 * r), 2.0 * r, a), -a, p, 1.0);
            for (what, other) in [("kissing", kiss), ("in a socket", socket)] {
                let got = read_at(p, vec![other, below()]);
                if got != Unread(TouchUnread::Unreadable) {
                    bad.push(format!("{raw:?} {far}: {what}: {got:?}"));
                }
            }
            // Off the pole, where the frame's first direction is half off
            // square to the normal: the fan is the part of it in the plane.
            // `s0` is the ball's reference direction.
            let n = (s0 * 0.5 + a * 0.8).normalize();
            let (one, two) = (
                whole_at(ball(p - n * r, r, a), n, p, 1.0),
                whole_at(ball(p + n * r, r, a), -n, p, 1.0),
            );
            let got = read_at(p, vec![one, two]);
            if got != Unread(TouchUnread::Unreadable) {
                bad.push(format!("{raw:?} {far}: kissing off the pole: {got:?}"));
            }
            let ring = [
                s0 * 0.3 + a * 0.05,
                s1 * 0.3 + a * 0.05,
                -(s0 + s1) * 0.2 + a * 0.05,
            ];
            let mut rest = corner_at(p, &ring, true, &k, &[]);
            rest.push(below());
            let got = read_at(p, rest);
            if !matches!(got, TouchVerdict::Rest(_)) {
                bad.push(format!("{raw:?} {far}: a shallow corner: {got:?}"));
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **Two whole tangent planes are read one plane each against the
/// other**: a ball's plane out to a hundred metres and another's out to
/// a centimetre, turned off each other by an angle the long fan reads
/// past the band and the short one inside it, refuse in either order.
/// Red where only the first plane's rays are read against the second's.
#[test]
fn two_whole_planes_are_read_one_plane_either_way_round() {
    let (z, x) = (v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0));
    let theta = 3.0 * band().escalate() / 100.0;
    let tilted = (z * theta.cos() + x * theta.sin()).normalize();
    let o = Point3::origin();
    let long = || whole_at(ball(o - z, 1.0, z), z, o, 100.0);
    let short = || whole_at(ball(o + tilted, 1.0, tilted), -tilted, o, 0.01);
    for (what, s) in [
        ("long first", vec![long(), short()]),
        ("short first", vec![short(), long()]),
    ] {
        assert_eq!(
            read(s),
            TouchVerdict::Unread(TouchUnread::Unreadable),
            "{what}"
        );
    }
}

/// **A spur's tip is the whole turn**: the loop leaving back along the
/// half-edge it came in by is four quarter turns, where two departures
/// along one direction are a cusp, refused. Red where a spur's tip reads as a cusp, or a
/// cusp as a whole turn.
#[test]
fn a_spurs_tip_is_the_whole_turn_and_a_cusp_refuses() {
    let z = v(0.0, 0.0, 1.0);
    let d = v(2.0, 0.0, 0.0);
    let tip = corner_fan(Point3::origin(), z, (d, d), [Vec3::zero(); 2], true, band())
        .expect("a spur's tip")
        .fan()
        .expect("a spur's tip");
    assert_eq!(tip.len(), 4, "{tip:?}");
    assert!(matches!(
        corner_fan(
            Point3::origin(),
            z,
            (d, d),
            [Vec3::zero(); 2],
            false,
            band()
        ),
        Err(TouchUnread::Unreadable)
    ));
}

/// **A bound in band of another sector's plane refuses in band**: a
/// corner on a ball's tangent plane, one of its edges off the plane by
/// three of the band's zero. Red where an in-band reading is passed as
/// off the plane.
#[test]
fn a_bound_in_band_of_another_plane_refuses_in_band() {
    let k = edges(3);
    let lift = 3.0 * band().zero();
    let ring = [v(1.0, 0.0, lift), v(-0.2, 0.3, 1.0), v(-0.1, -0.4, 1.0)];
    let mut s = corner(&ring, true, &k, &[]);
    s.push(whole(sphere(-1.0), true));
    assert_eq!(read(s), TouchVerdict::Unread(TouchUnread::InBand));
}

/// **A bound kept on another's bound is kept only ahead of it**: two
/// half-planes on curved carriers about one line, one bounded by the
/// line edge `A` along `+x` and the closed curved edge `D` along `−x`,
/// the other by `D` along `+x` and the line edge `C` along `−x` — `D`
/// one vertex's edge, leaving along one direction and coming back along
/// the other. `A` lies on the other's bound along `D`, another edge and
/// a curve, so it refuses; `C`'s line behind that bound is no bound `A`
/// lies on. Red where a bound is kept on a bound it lies behind.
#[test]
fn a_bound_is_kept_only_on_a_bound_it_lies_ahead_on() {
    let k = edges(3);
    let (a, c, d) = (k[0], k[1], k[2]);
    let x = v(1.0, 0.0, 0.0);
    let half = |n: Vec3<f64>, bounds| {
        let fan = TangentFan::Corner {
            d0: x,
            d1: -x,
            bounds,
            spur: false,
        };
        sector(cylinder(), true, n, Point3::origin(), fan, band()).unwrap()
    };
    let s = vec![
        half(v(0.0, 0.0, 1.0), [tag(a, true), tag(d, false)]),
        half(v(0.0, -1.0, 0.0), [tag(d, false), tag(c, true)]),
    ];
    assert_eq!(read(s), TouchVerdict::Unread(TouchUnread::Unreadable));
}

/// **Only a spur's tip is the whole turn**: two departures a tenth of the
/// band's zero apart along one edge's two ends (a closed edge's one
/// vertex, not a spur) are a cusp, refused either way round. Red where a
/// near-full turn reads as the whole one.
#[test]
fn a_near_full_turn_off_one_edge_is_a_cusp() {
    let z = v(0.0, 0.0, 1.0);
    let d0 = v(2.0, 0.0, 0.0);
    let tiny = 0.1 * band().zero();
    for d1 in [v(2.0, tiny, 0.0), v(2.0, -tiny, 0.0)] {
        assert!(
            matches!(
                corner_fan(
                    Point3::origin(),
                    z,
                    (d0, d1),
                    [Vec3::zero(); 2],
                    false,
                    band()
                ),
                Err(TouchUnread::Unreadable)
            ),
            "{d1:?}"
        );
    }
}

/// A corner standing on the plane `z = 0` at the origin, one line edge
/// along `+x` in it, and `other`'s whole tangent plane there, outward
/// `+z`, its material below.
fn corner_on(other: geom::Surface<f64>, sense: bool) -> TouchVerdict {
    let k = edges(3);
    let n = v(0.0, 0.0, 1.0);
    let frame = crate::census::carrier_frame(&other).unwrap();
    let fan = TangentFan::Inside { reach: 2.0, frame };
    let whole = sector(other, sense, n, Point3::origin(), fan, band()).unwrap();
    let ring = [v(1.0, 0.0, 0.0), v(-0.2, 0.3, 1.0), v(-0.1, -0.4, 1.0)];
    let mut s = corner(&ring, true, &k, &[]);
    s.push(whole);
    read(s)
}

/// **A line edge in a huge sphere's or cylinder's tangent plane lies in
/// the face only along a cylinder's ruling**: radius 1e9, so the edge's
/// far end stands within the band of the carrier whichever way it runs.
/// On a sphere, which has no ruling, and across a cylinder's ruling, it
/// refuses; along the ruling it reads. Red where the band alone keeps a
/// line on a carrier.
#[test]
fn a_line_in_a_huge_carriers_plane_is_kept_only_along_a_ruling() {
    let big = 1e9;
    let sphere = geom::Surface::Sphere {
        center: Point3::new(0.0, 0.0, -big),
        radius: big,
        axis: v(0.0, 0.0, 1.0),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let tube = |axis: Vec3<f64>| geom::Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, -big),
        axis,
        radius: big,
        u_ref: v(0.0, 0.0, 1.0),
    };
    let refused = TouchVerdict::Unread(TouchUnread::Unreadable);
    assert_eq!(corner_on(sphere, true), refused, "a sphere");
    assert_eq!(
        corner_on(tube(v(0.0, 1.0, 0.0)), true),
        refused,
        "across a ruling"
    );
    assert!(
        matches!(
            corner_on(tube(v(1.0, 0.0, 0.0)), true),
            TouchVerdict::Rest(_)
        ),
        "along a ruling"
    );
}

/// **A frame direction opposite the normal turns no fan**: a sphere's
/// whole plane on its equator opposite its reference direction, outward
/// `−u_ref`, so the first frame direction is the normal reversed. The
/// fan turns from another of the frame, in the plane; two balls kissing
/// there refuse. Red where a direction is taken as square to the normal
/// for lying below `+0.6` only.
#[test]
fn a_frame_direction_opposite_the_normal_turns_no_fan() {
    let a = v(0.2, -0.5, 0.8).normalize();
    let u = a.cross(v(0.3, -0.7, 0.2)).normalize();
    let p = Point3::new(0.3, -0.2, 0.1);
    let r = 0.7;
    let frame = crate::census::carrier_frame(&ball(p + u * r, r, a)).unwrap();
    let along = in_plane(p, -u, frame, band()).unwrap();
    assert!(
        along.dot(u).abs() < 1e-14 && (along.norm() - 1.0).abs() < 1e-14,
        "{along:?}"
    );
    let s = vec![
        whole_at(ball(p + u * r, r, a), -u, p, 1.0),
        whole_at(ball(p - u * r, r, a), u, p, 1.0),
    ];
    assert_eq!(read_at(p, s), TouchVerdict::Unread(TouchUnread::Unreadable));
}
