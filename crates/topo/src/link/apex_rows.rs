//! **A cone's apex read as its nappe, and two arcs of one circle kept**
//! ([`rest::one_material`]): a cone face's corner at its apex
//! ([`ApexLobe`]) against whole planes and half-planes through it, and a
//! boss resting on its own plate's hole, the two rims one circle through
//! the point. Each row near the origin and a kilometre off it, in `f64`
//! and in `Interval`, its directions in the frame [`v`] turns.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use geom_core::{Interval, Tol};

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance is a band")
}

/// `(x, y, z)` in the rows' frame, turned a third of a turn about
/// `(1, 1, 1)` to `(z, x, y)`, as `lens_rows` turns its own.
fn v<T: Real>(x: f64, y: f64, z: f64) -> Vec3<T> {
    Vec3::new(T::from_f64(z), T::from_f64(x), T::from_f64(y))
}

/// Where the rows stand: the origin, and a kilometre off it.
fn apexes<T: Real>() -> [Point3<T>; 2] {
    [
        Point3::origin(),
        Point3::new(
            T::from_f64(1000.0),
            T::from_f64(-2000.0),
            T::from_f64(500.0),
        ),
    ]
}

/// `n` distinct edge keys.
fn edges(n: usize) -> Vec<EdgeKey> {
    let mut keys = slotmap::SlotMap::<EdgeKey, ()>::with_key();
    (0..n).map(|_| keys.insert(())).collect()
}

/// `n` distinct surface keys.
fn carriers(n: usize) -> Vec<crate::SurfaceKey> {
    let mut keys = slotmap::SlotMap::<crate::SurfaceKey, ()>::with_key();
    (0..n).map(|_| keys.insert(())).collect()
}

/// The half-angle of the rows' cones: 30°.
const ALPHA: f64 = core::f64::consts::FRAC_PI_6;

/// The cone of half-angle [`ALPHA`] at `p`, its nappe along `−y`, of
/// `Face::sense` `sense` (its material inside where set).
fn cone<T: Real>(p: Point3<T>, sense: bool) -> crate::sector_face::ConeApex<T> {
    crate::sector_face::ConeApex {
        apex: p,
        axis: v(0.0, 1.0, 0.0),
        half_angle: T::from_f64(ALPHA),
        u_ref: v(1.0, 0.0, 0.0),
        sense,
    }
}

/// The nappe's unit ruling at azimuth `phi` about `−y`, from `+x`.
fn ruling<T: Real>(phi: f64) -> Vec3<T> {
    let (s, c) = ALPHA.sin_cos();
    v(s * phi.cos(), -c, s * phi.sin())
}

/// The nappe whole, as one face's corner leaving its apex and coming
/// back along one edge (a spur).
fn spur<T: Real>(p: Point3<T>, sense: bool) -> TangentPiece<T> {
    let (e, k) = (edges(1)[0], carriers(1)[0]);
    TangentPiece::Apex(ApexLobe {
        cone: cone(p, sense),
        carrier: k,
        vertex: p,
        ends: [(ruling(0.0), e), (ruling(0.0), e)],
        spur: true,
    })
}

/// The nappe as two faces' corners, from the ruling at azimuth `0` round
/// to `turn` and back, each bounded by the other's two edges; on the
/// carriers `on`, of senses `senses`.
fn halves<T: Real>(
    p: Point3<T>,
    turn: f64,
    on: [crate::SurfaceKey; 2],
    senses: [bool; 2],
) -> [TangentPiece<T>; 2] {
    let e = edges(2);
    let (a, b) = (ruling::<T>(0.0), ruling::<T>(turn));
    [
        TangentPiece::Apex(ApexLobe {
            cone: cone(p, senses[0]),
            carrier: on[0],
            vertex: p,
            ends: [(a, e[0]), (b, e[1])],
            spur: false,
        }),
        TangentPiece::Apex(ApexLobe {
            cone: cone(p, senses[1]),
            carrier: on[1],
            vertex: p,
            ends: [(b, e[1]), (a, e[0])],
            spur: false,
        }),
    ]
}

/// A whole plane through `p` of outward unit `m`, its material behind it.
fn whole<T: Decide>(p: Point3<T>, m: Vec3<T>) -> TangentPiece<T> {
    let surface = geom::Surface::Plane {
        origin: p,
        normal: m,
        u_ref: m.cross(v::<T>(1.0, 0.3, 0.7)).normalize(),
    };
    let fan = TangentFan::Inside {
        reach: T::from_f64(2.0),
        frame: [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)],
    };
    FaceSector::tangent(surface, true, m, p, fan, band()).expect("a whole plane")
}

/// The unit normal tilted `beta` off `+y` toward `−x`: its plane holds
/// the nappe below it, its nearest ruling `cos(α + β)` under it a unit
/// out.
fn tilted<T: Real>(beta: f64) -> Vec3<T> {
    v(-beta.sin(), beta.cos(), 0.0)
}

fn read<T: Decide>(p: Point3<T>, pieces: Vec<TangentPiece<T>>) -> TouchVerdict {
    rest::one_material(&Link::of_tangent(p, pieces).expect("a link"), band())
}

/// **A nappe off every plane is the thorn or the needle it bounds.** A
/// cone's apex on a whole plane, the nappe under it: a void cone in the
/// plane's material (a needle, the tipped cone's `h ∖ c`), and a solid
/// cone under the plane's material above (a thorn, `c ∪ h`), rest, as
/// they do with the plane tilted while it holds the nappe off, and with
/// the nappe as two faces' halves on one carrier. A solid cone inside
/// the material, or a void one outside it, is no one material and is not
/// read; nor is a nappe not held whole (one half alone, two halves on two
/// carriers or of two senses, or two whole turns). Red where the region
/// round the nappe is not read, its material is read the other way
/// round, or a nappe not held whole is read.
fn a_nappe_off_every_plane_is_the_thorn_or_needle_it_bounds<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Curved(CurvedUnread::NoTangentPlane));
    let up = v::<T>(0.0, 1.0, 0.0);
    for p in apexes::<T>() {
        let rests = |what: &str, pieces: Vec<TangentPiece<T>>| {
            let got = read(p, pieces);
            assert!(
                matches!(got, TouchVerdict::Rest(_)),
                "{what} at {p:?}: {got:?}"
            );
        };
        rests(
            "a needle under its plane",
            vec![spur(p, false), whole(p, up)],
        );
        rests(
            "a thorn under the plane's material",
            vec![spur(p, true), whole(p, -up)],
        );
        rests(
            "a needle under a tilted plane",
            vec![
                spur(p, false),
                whole(p, (v::<T>(0.2, 1.0, 0.1)).normalize()),
            ],
        );
        let k = carriers(2);
        let mut two = halves(p, core::f64::consts::PI, [k[0], k[0]], [false; 2])
            .into_iter()
            .collect::<Vec<_>>();
        two.push(whole(p, up));
        rests("a needle in two halves", two);
        assert_eq!(
            read(p, vec![spur(p, true), whole(p, up)]),
            unread,
            "a thorn inside the material at {p:?}"
        );
        assert_eq!(
            read(p, vec![spur(p, false), whole(p, -up)]),
            unread,
            "a needle outside the material at {p:?}"
        );
        let [a, _] = halves(p, core::f64::consts::PI, [k[0], k[0]], [false; 2]);
        assert_eq!(
            read(p, vec![a, whole(p, up)]),
            unread,
            "a half alone at {p:?}"
        );
        for (what, on, senses) in [
            ("two carriers", [k[0], k[1]], [false; 2]),
            ("two senses", [k[0], k[0]], [false, true]),
        ] {
            let mut two = halves(p, core::f64::consts::PI, on, senses)
                .into_iter()
                .collect::<Vec<_>>();
            two.push(whole(p, up));
            assert_eq!(read(p, two), unread, "{what} at {p:?}");
        }
        let mut twice = halves(p, 0.0, [k[0], k[0]], [false; 2])
            .into_iter()
            .collect::<Vec<_>>();
        twice.push(whole(p, up));
        assert_eq!(read(p, twice), unread, "two whole turns at {p:?}");
    }
}

/// **A plane the nappe does not keep off crosses it or refuses.** A whole
/// plane holding the cone's axis crosses the nappe; one tangent to it
/// along a ruling, or tilted so the nearest ruling stands in band of it
/// at the arm, refuses in band, and the same tilt past the escalation
/// rests. A half-plane whose plane cuts the nappe but whose sector stands
/// above it is not read. Red where a plane's side is read uncharged by
/// the rulings' spread, a decided straddle is taken for either side, or a
/// half-plane is taken for its whole plane.
fn a_plane_the_nappe_does_not_keep_off_crosses_it_or_refuses<T: Decide>() {
    let e = band().escalate();
    for p in apexes::<T>() {
        assert_eq!(
            read(p, vec![spur(p, false), whole(p, v(1.0, 0.0, 0.0))]),
            TouchVerdict::Crossing,
            "a plane through the axis at {p:?}"
        );
        let beta = core::f64::consts::FRAC_PI_2 - ALPHA;
        assert_eq!(
            read(p, vec![spur(p, false), whole(p, tilted(beta))]),
            TouchVerdict::Unread(TouchUnread::InBand),
            "a plane tangent along a ruling at {p:?}"
        );
        // The nearest ruling `sin δ` under the plane a unit out.
        for (frac, rests) in [(0.5, false), (2.0, true)] {
            let delta = (frac * e).asin();
            let got = read(p, vec![spur(p, false), whole(p, tilted(beta - delta))]);
            if rests {
                assert!(
                    matches!(got, TouchVerdict::Rest(_)),
                    "the nearest ruling {frac} of the escalation under at {p:?}: {got:?}"
                );
            } else {
                assert_eq!(
                    got,
                    TouchVerdict::Unread(TouchUnread::InBand),
                    "the nearest ruling {frac} of the escalation under at {p:?}"
                );
            }
        }
        // An edge along `x`, its two faces the half of `z = 0` above the
        // nappe and the half of `y = 0` toward `+z`: the first one's plane
        // holds the axis, but its sector stands clear of the nappe.
        let k = edges(1)[0];
        let half = |m: Vec3<T>, along: Vec3<T>| {
            let surface = geom::Surface::Plane {
                origin: p,
                normal: m,
                u_ref: along,
            };
            let fan = TangentFan::Edge {
                along,
                reach: T::from_f64(2.0),
                edge: Bound {
                    edge: k,
                    line: true,
                    circle: false,
                    bend: Vec3::zero(),
                },
            };
            FaceSector::tangent(surface, true, m, p, fan, band()).expect("a half-plane")
        };
        let book = vec![
            spur(p, false),
            half(v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0)),
            half(v(0.0, 1.0, 0.0), v(-1.0, 0.0, 0.0)),
        ];
        assert_eq!(
            read(p, book),
            TouchVerdict::Unread(TouchUnread::Curved(CurvedUnread::NoTangentPlane)),
            "a half-plane over the nappe at {p:?}"
        );
    }
}

/// The boss resting on its own plate's hole about `p`, both rims of
/// radius `0.3` about the boss's axis `0.3` behind `p` along `x`, its
/// hole's rim bending by `hole` (its curvature vector along `x`) and
/// held a circle's where `circle`: the boss's wall over `z = 0` (two
/// corners at its seam along `+z`), its floor, the plate's top, and the
/// hole's wall under it.
fn boss_on_hole<T: Decide>(p: Point3<T>, hole: f64, circle: bool) -> Vec<TangentPiece<T>> {
    let k = edges(4);
    let (seam, rim_out, rim_in, rim) = (k[0], k[1], k[2], k[3]);
    let r = 0.3;
    let wall = geom::Surface::Cylinder {
        origin: p - v::<T>(r, 0.0, 0.0),
        axis: v(0.0, 0.0, 1.0),
        radius: T::from_f64(r),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let bore = geom::Surface::Cylinder {
        origin: p - v::<T>(r, 0.0, 0.5),
        axis: v(0.0, 0.0, -1.0),
        radius: T::from_f64(r),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let arc = |edge, bend: f64, circle| Bound {
        edge,
        line: false,
        circle,
        bend: v(bend, 0.0, 0.0),
    };
    let boss = arc(rim_out, -1.0 / r, true);
    let boss_in = arc(rim_in, -1.0 / r, true);
    let lip = arc(rim, hole, circle);
    let line = Bound {
        edge: seam,
        line: true,
        circle: false,
        bend: Vec3::zero(),
    };
    let s = T::from_f64(0.2);
    let corner = |surface: geom::Surface<T>, m: Vec3<T>, (d0, d1): (Vec3<T>, Vec3<T>), bounds| {
        let fan = TangentFan::Corner {
            d0: d0 * s,
            d1: d1 * s,
            bounds,
            spur: false,
        };
        FaceSector::tangent(surface, true, m, p, fan, band()).expect("a corner")
    };
    let along = |surface: geom::Surface<T>, sense, m: Vec3<T>, a: Vec3<T>| {
        let fan = TangentFan::Edge {
            along: a,
            reach: T::from_f64(0.2),
            edge: lip,
        };
        FaceSector::tangent(surface, sense, m, p, fan, band()).expect("a half-plane")
    };
    let plane = |m: Vec3<T>| geom::Surface::Plane {
        origin: p,
        normal: m,
        u_ref: v(1.0, 0.0, 0.0),
    };
    let x = v::<T>(1.0, 0.0, 0.0);
    let (y, z) = (v::<T>(0.0, 1.0, 0.0), v::<T>(0.0, 0.0, 1.0));
    vec![
        corner(wall.clone(), x, (z, -y), [line, boss]),
        corner(wall, x, (y, z), [boss_in, line]),
        corner(plane(-z), -z, (-y, y), [boss, boss_in]),
        along(plane(z), true, z, -y),
        along(bore, false, -x, y),
    ]
}

/// **Two arcs of one circle are kept as one.** A boss resting on its own
/// plate's hole, the two rims one circle through the point (the boss's
/// vertex inside the shared arc), touches along the rim with its
/// material and the plate's in opposite quarters: a rest. The same rims
/// held as arcs of no circle, or the hole's rim bending in band of the
/// boss's, are not read, nor is the hole's rim bending tighter, which
/// puts the boss's floor on the plate's top over a crescent. Red where
/// two arcs are kept by their bends alone, or two bends that differ are
/// kept.
fn two_arcs_of_one_circle_are_kept_as_one<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Curved(CurvedUnread::Unreadable));
    let e = band().escalate();
    let boss = -1.0 / 0.3;
    for p in apexes::<T>() {
        let got = read(p, boss_on_hole(p, boss, true));
        assert!(
            matches!(got, TouchVerdict::Rest(_)),
            "the rims one circle at {p:?}: {got:?}"
        );
        assert_eq!(
            read(p, boss_on_hole(p, boss, false)),
            unread,
            "the hole's rim no circle at {p:?}"
        );
        // A bend `2·frac·e/l²` off the boss's, at the arm `l = 0.2`.
        let off = 2.0 * 0.5 * e / 0.04;
        assert_eq!(
            read(p, boss_on_hole(p, boss - off, true)),
            unread,
            "the hole's rim bending in band of the boss's at {p:?}"
        );
        assert_eq!(
            read(p, boss_on_hole(p, -1.0 / 0.2, true)),
            unread,
            "the hole's rim tighter at {p:?}"
        );
    }
}

#[test]
fn a_nappe_off_every_plane_is_the_thorn_or_needle_it_bounds_f64() {
    a_nappe_off_every_plane_is_the_thorn_or_needle_it_bounds::<f64>();
}

#[test]
fn a_nappe_off_every_plane_is_the_thorn_or_needle_it_bounds_interval() {
    a_nappe_off_every_plane_is_the_thorn_or_needle_it_bounds::<Interval>();
}

#[test]
fn a_plane_the_nappe_does_not_keep_off_crosses_it_or_refuses_f64() {
    a_plane_the_nappe_does_not_keep_off_crosses_it_or_refuses::<f64>();
}

#[test]
fn a_plane_the_nappe_does_not_keep_off_crosses_it_or_refuses_interval() {
    a_plane_the_nappe_does_not_keep_off_crosses_it_or_refuses::<Interval>();
}

#[test]
fn two_arcs_of_one_circle_are_kept_as_one_f64() {
    two_arcs_of_one_circle_are_kept_as_one::<f64>();
}

#[test]
fn two_arcs_of_one_circle_are_kept_as_one_interval() {
    two_arcs_of_one_circle_are_kept_as_one::<Interval>();
}
