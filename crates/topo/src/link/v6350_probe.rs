//! Verifier probe for PR 6350 (not for merge).
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use geom_core::{Bounds, Interval, Tol};

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

fn ball<T: Real>(c: Point3<T>, r: T, a: Vec3<T>, side: Vec3<T>) -> geom::Surface<T> {
    geom::Surface::Sphere {
        center: c,
        radius: r,
        axis: a,
        u_ref: a.cross(side).normalize(),
    }
}

fn whole<T: Decide>(s: geom::Surface<T>, n: Vec3<T>, p: Point3<T>) -> FaceSector<T> {
    let frame = crate::census::carrier_frame(&s).unwrap();
    let fan = TangentFan::Inside {
        reach: T::from_f64(1.0),
        frame,
    };
    FaceSector::tangent(s, true, n, p, fan, band()).unwrap()
}

/// Deterministic axes over the sphere (a Fibonacci lattice), tilted every way.
fn axes(n: usize) -> Vec<Vec3<f64>> {
    let g = core::f64::consts::PI * (3.0 - 5f64.sqrt());
    (0..n)
        .map(|i| {
            let z = 1.0 - 2.0 * (i as f64 + 0.5) / n as f64;
            let r = (1.0 - z * z).sqrt();
            let t = g * i as f64;
            v(r * t.cos(), r * t.sin(), z).normalize()
        })
        .collect()
}

/// The R1 probe rebuilt: kissing balls and a ball in a socket at a
/// tilted pole, normals computed off the centres (rounding-noisy),
/// against a floor and a plane crossing it, many tilts, near and far.
#[test]
fn v_r1_rebuilt_many_tilts_far() {
    let mut bad = Vec::new();
    let mut worst: f64 = 0.0;
    let sides = [v(0.3, -0.7, 0.2), v(1.0, 0.0, 0.0), v(0.0, 0.0, 1.0)];
    let mut n_cases = 0;
    for a in axes(200) {
        for side in sides {
            if a.cross(side).norm() < 1e-3 {
                continue;
            }
            for far in [0.0, 1e3, 1e5] {
                let p = Point3::new(0.3 + far, -0.2 + 2.0 * far, 0.1 - far);
                let r = 0.7;
                let ca = p - a * r;
                let sa = ball(ca, r, a, side);
                let na = (p - ca).normalize();
                // Direct check of the fan start.
                let frame = crate::census::carrier_frame(&sa).unwrap();
                let along = in_plane(p, na, frame, band()).unwrap();
                worst = worst.max(along.dot(na).abs());
                for socket in [false, true] {
                    let (cb, rb) = if socket {
                        (p - a * (2.0 * r), 2.0 * r)
                    } else {
                        (p + a * r, r)
                    };
                    let sb = ball(cb, rb, a, side);
                    let nb = if socket {
                        -(p - cb).normalize()
                    } else {
                        (p - cb).normalize()
                    };
                    for order in [false, true] {
                        let mut s = vec![whole(sa.clone(), na, p), whole(sb.clone(), nb, p)];
                        if order {
                            s.reverse();
                        }
                        let got = rest::one_material(&Link::of(p, s).unwrap(), band());
                        n_cases += 1;
                        if got != TouchVerdict::Unread(TouchUnread::Unreadable) {
                            bad.push(format!("{a:?} far {far} socket {socket}: {got:?}"));
                        }
                    }
                }
                // Against a floor (one plane) and a plane tilted 0.3 rad (crossing).
                let floor_side = a.cross(side).normalize();
                for (what, n, want_cross) in [
                    ("floor", -a, false),
                    ("tilted", (-a * 0.3f64.cos() + floor_side * 0.3f64.sin()).normalize(), true),
                ] {
                    let fl = geom::Surface::Plane {
                        origin: p,
                        normal: n,
                        u_ref: n.cross(side).normalize(),
                    };
                    // ball above the floor: centre at p + a r? use sa (centre p - a r, outward +a);
                    // the floor's material on the other side: outward -a.
                    for order in [false, true] {
                        let mut s = vec![whole(sa.clone(), na, p), whole(fl.clone(), n, p)];
                        if order {
                            s.reverse();
                        }
                        let got = rest::one_material(&Link::of(p, s).unwrap(), band());
                        n_cases += 1;
                        let ok = if want_cross {
                            got == TouchVerdict::Crossing
                        } else {
                            !matches!(got, TouchVerdict::Rest(_) | TouchVerdict::Crossing)
                        };
                        if !ok {
                            bad.push(format!("{a:?} far {far} {what} order {order}: {got:?}"));
                        }
                    }
                }
            }
        }
    }
    eprintln!("cases {n_cases}, worst |along.n| = {worst:e}");
    assert!(worst < 1e-14, "worst {worst:e}");
    assert!(bad.is_empty(), "{} bad: {:#?}", bad.len(), &bad[..bad.len().min(20)]);
}

/// The fan start is in the plane for every analytic carrier kind, at
/// arbitrary normals (the frame is the carrier's, the normal anything).
#[test]
fn v_in_plane_every_kind_any_normal() {
    let mut worst: f64 = 0.0;
    for a in axes(50) {
        let u = a.cross(v(0.3, -0.7, 0.2)).normalize();
        let o = Point3::new(1e3, -2e3, 5e2);
        let kinds: Vec<geom::Surface<f64>> = vec![
            geom::Surface::Plane { origin: o, normal: a, u_ref: u },
            geom::Surface::Cylinder { origin: o, axis: a, radius: 0.5, u_ref: u },
            geom::Surface::Cone { apex: o, axis: a, half_angle: 0.4, u_ref: u },
            geom::Surface::Torus { center: o, axis: a, major_radius: 2.0, minor_radius: 0.5, u_ref: u },
        ];
        for s in kinds {
            let frame = crate::census::carrier_frame(&s).unwrap();
            for m in axes(100) {
                let along = in_plane(o, m, frame, band()).expect("one of three always reads");
                worst = worst.max(along.dot(m).abs()).max((along.norm() - 1.0).abs());
            }
        }
    }
    eprintln!("worst {worst:e}");
    assert!(worst < 1e-14);
}

fn iv(x: Vec3<f64>) -> Vec3<Interval> {
    Vec3::new(Interval::from_f64(x.x), Interval::from_f64(x.y), Interval::from_f64(x.z))
}
fn ip(x: Point3<f64>) -> Point3<Interval> {
    Point3::new(Interval::from_f64(x.x), Interval::from_f64(x.y), Interval::from_f64(x.z))
}

/// Interval: the fan start encloses an in-plane direction, and kissing
/// balls at a tilted pole refuse.
#[test]
fn v_interval_kissing_balls() {
    let mut bad = Vec::new();
    for a in axes(40) {
        let side = v(0.3, -0.7, 0.2);
        for far in [0.0, 1e3] {
            let p = Point3::new(0.3 + far, -0.2 + 2.0 * far, 0.1);
            let r = 0.7;
            let (ai, pi, ri, si) = (iv(a), ip(p), Interval::from_f64(r), iv(side));
            let ca = pi - ai * ri;
            let cb = pi + ai * ri;
            let sa = ball(ca, ri, ai, si);
            let sb = ball(cb, ri, ai, si);
            let na = (pi - ca).normalize();
            let nb = (pi - cb).normalize();
            let frame = crate::census::carrier_frame(&sa).unwrap();
            match in_plane(pi, na, frame, band()) {
                Ok(along) => {
                    let d = along.dot(na);
                    if !(d.lo() <= 0.0 && 0.0 <= d.hi()) || d.hi() - d.lo() > 1e-9 {
                        bad.push(format!("{a:?} {far}: along.n = [{}, {}]", d.lo(), d.hi()));
                    }
                }
                Err(e) => bad.push(format!("{a:?} {far}: in_plane {e:?}")),
            }
            let got = rest::one_material(
                &Link::of(pi, vec![whole(sa, na, pi), whole(sb, nb, pi)]).unwrap(),
                band(),
            );
            if got != TouchVerdict::Unread(TouchUnread::Unreadable) {
                bad.push(format!("{a:?} {far}: {got:?}"));
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// A one-edge loop (a closed curved edge) whose vertex is a cusp: it
/// leaves and comes back along one direction, but its two departures are
/// two computations, not one, so they differ by rounding. `slit` is the
/// edge-key test, so does it pass a near-full turn as the whole turn?
#[test]
fn v_slit_flag_admits_a_near_full_turn() {
    let z = v(0.0, 0.0, 1.0);
    let d0 = v(2.0, 0.0, 0.0);
    let tiny = 0.1 * band().zero();
    let d1 = v(2.0, tiny, 0.0);
    let got = corner_fan(Point3::origin(), z, (d0, d1), true, band());
    eprintln!("slit near-full: {:?}", got.as_ref().map(Vec::len));
    let d1 = v(2.0, -tiny, 0.0);
    let got2 = corner_fan(Point3::origin(), z, (d0, d1), true, band());
    eprintln!("slit near-full other way: {:?}", got2.as_ref().map(Vec::len));
    assert!(got.is_err() && got2.is_err(), "a near-full turn read as a whole turn");
}

fn edges(n: usize) -> Vec<EdgeKey> {
    let mut keys = slotmap::SlotMap::<EdgeKey, ()>::with_key();
    (0..n).map(|_| keys.insert(())).collect()
}

fn cyl() -> geom::Surface<f64> {
    geom::Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, -1.0),
        axis: v(1.0, 0.0, 0.0),
        radius: 1.0,
        u_ref: v(0.0, 0.0, 1.0),
    }
}

/// `kept_bound` without `ahead`: two half-planes about one line, each
/// bounded on one side by a line edge and on the other by a closed curved
/// edge `D` (one vertex, which leaves along `+x` and comes back along
/// `-x`, so it tags a bound both ways). k: `+x` line A, `-x` D; j: `+x`
/// D, `-x` line C. The real code refuses (A lies on D's bound, other
/// edge, a curve); the mutant keeps every bound through a bound behind.
#[test]
fn v_kept_bound_ahead_counterexample() {
    let k = edges(3);
    let (a, c, d) = (k[0], k[1], k[2]);
    let o = Point3::origin();
    let x = v(1.0, 0.0, 0.0);
    let sk = FaceSector::tangent(
        cyl(),
        true,
        v(0.0, 0.0, 1.0),
        o,
        TangentFan::Corner { d0: x, d1: -x, bounds: [(a, true), (d, false)] },
        band(),
    )
    .unwrap();
    let sj = FaceSector::tangent(
        cyl(),
        true,
        v(0.0, -1.0, 0.0),
        o,
        TangentFan::Corner { d0: x, d1: -x, bounds: [(d, false), (c, true)] },
        band(),
    )
    .unwrap();
    let got = rest::one_material(&Link::of(o, vec![sk, sj]).unwrap(), band());
    eprintln!("kept_bound probe: {got:?}");
    assert_eq!(got, TouchVerdict::Unread(TouchUnread::Unreadable));
}

/// A sphere's whole plane on its equator opposite its reference
/// direction: outward `-u_ref`, so the first frame direction is
/// anti-parallel to the normal. Kissing balls there refuse; a fan of
/// NaN does not.
#[test]
fn v_frame_direction_antiparallel_to_the_normal() {
    let mut bad = Vec::new();
    for a in axes(20) {
        let side = v(0.3, -0.7, 0.2);
        let u = a.cross(side).normalize();
        let p = Point3::new(0.3, -0.2, 0.1);
        let r = 0.7;
        // ball A: centre p + u r, so p = centre - u r, outward -u.
        let sa = ball(p + u * r, r, a, side);
        let na = (p - (p + u * r)).normalize();
        let frame = crate::census::carrier_frame(&sa).unwrap();
        let along = in_plane(p, na, frame, band()).unwrap();
        if !(along.dot(na).abs() < 1e-14 && (along.norm() - 1.0).abs() < 1e-14) {
            bad.push(format!("{a:?}: along {along:?}"));
        }
        let sb = ball(p - u * r, r, a, side);
        let nb = (p - (p - u * r)).normalize();
        let got = rest::one_material(
            &Link::of(p, vec![whole(sa, na, p), whole(sb, nb, p)]).unwrap(),
            band(),
        );
        if got != TouchVerdict::Unread(TouchUnread::Unreadable) {
            bad.push(format!("{a:?}: {got:?}"));
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// A line edge's ray lying in a huge sphere's tangent plane inside its
/// sector, within the band of the sphere's carrier at the ray's point:
/// a sphere has no ruling, so this is a tangency and must refuse.
#[test]
fn v_a_line_on_a_huge_spheres_tangent_plane_is_no_ruling() {
    let k = edges(3);
    let big = 1e9;
    let sphere = geom::Surface::Sphere {
        center: Point3::new(0.0, 0.0, -big),
        radius: big,
        axis: v(0.0, 0.0, 1.0),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let n = v(0.0, 0.0, 1.0);
    let fan = TangentFan::Inside {
        reach: 2.0,
        frame: crate::census::carrier_frame(&sphere).unwrap(),
    };
    let ball = FaceSector::tangent(sphere, true, n, Point3::origin(), fan, band()).unwrap();
    // A corner standing on the plane, one line edge along +x in it.
    let ring = [v(1.0, 0.0, 0.0), v(-0.2, 0.3, 1.0), v(-0.1, -0.4, 1.0)];
    let axis = ring.iter().fold(Vec3::zero(), |s, &r| s + r);
    let mut s: Vec<FaceSector<f64>> = (0..3)
        .map(|i| {
            let (a, b) = (i, (i + 1) % 3);
            let m = ring[a].cross(ring[b]).normalize();
            let (m, d0, d1, bounds) = if m.dot(axis) < 0.0 {
                (m, ring[a], ring[b], [(k[a], true), (k[b], true)])
            } else {
                (-m, ring[b], ring[a], [(k[b], true), (k[a], true)])
            };
            let plane = geom::Surface::Plane {
                origin: Point3::origin(),
                normal: m,
                u_ref: m.cross(v(0.3, 0.5, 0.7)).normalize(),
            };
            FaceSector::tangent(plane, true, m, Point3::origin(), TangentFan::Corner { d0, d1, bounds }, band())
                .unwrap()
        })
        .collect();
    s.push(ball);
    let got = rest::one_material(&Link::of(Point3::origin(), s).unwrap(), band());
    eprintln!("huge sphere: {got:?}");
    assert_eq!(got, TouchVerdict::Unread(TouchUnread::Unreadable));
}

/// As above with a huge cylinder whose axis is `y`: the line edge along
/// `+x` is tangent to its circle, not a ruling, yet lies within the band
/// of the carrier at its ray's point.
#[test]
fn v_a_line_across_a_huge_cylinders_ruling_is_no_ruling() {
    eprintln!("band zero {:e} escalate {:e}", band().zero(), band().escalate());
    let k = edges(3);
    let big = 1e9;
    for (what, axis, z0, sense) in [
        ("boss, ruling along y", v(0.0, 1.0, 0.0), -big, true),
        ("boss, ruling along x", v(1.0, 0.0, 0.0), -big, true),
        ("bore, ruling along y", v(0.0, 1.0, 0.0), big, false),
        ("bore, ruling along x", v(1.0, 0.0, 0.0), big, false),
    ] {
        let cyl = geom::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, z0),
            axis,
            radius: big,
            u_ref: v(0.0, 0.0, if z0 < 0.0 { 1.0 } else { -1.0 }),
        };
        let n = v(0.0, 0.0, 1.0);
        let fan = TangentFan::Inside {
            reach: 2.0,
            frame: crate::census::carrier_frame(&cyl).unwrap(),
        };
        let c = FaceSector::tangent(cyl, sense, n, Point3::origin(), fan, band()).unwrap();
        let ring = [v(1.0, 0.0, 0.0), v(-0.2, 0.3, 1.0), v(-0.1, -0.4, 1.0)];
        let ax = ring.iter().fold(Vec3::zero(), |s, &r| s + r);
        let mut s: Vec<FaceSector<f64>> = (0..3)
            .map(|i| {
                let (a, b) = (i, (i + 1) % 3);
                let m = ring[a].cross(ring[b]).normalize();
                let (m, d0, d1, bounds) = if m.dot(ax) < 0.0 {
                    (m, ring[a], ring[b], [(k[a], true), (k[b], true)])
                } else {
                    (-m, ring[b], ring[a], [(k[b], true), (k[a], true)])
                };
                let plane = geom::Surface::Plane {
                    origin: Point3::origin(),
                    normal: m,
                    u_ref: m.cross(v(0.3, 0.5, 0.7)).normalize(),
                };
                FaceSector::tangent(plane, true, m, Point3::origin(), TangentFan::Corner { d0, d1, bounds }, band())
                    .unwrap()
            })
            .collect();
        s.push(c);
        let got = rest::one_material(&Link::of(Point3::origin(), s).unwrap(), band());
        eprintln!("huge cylinder, {what}: {got:?}");
    }
}
