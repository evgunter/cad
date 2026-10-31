//! **A sphere's or a cylinder's material, read by its carrier**
//! ([`Round`]): its offset and lowest offset along a segment against a
//! dense walk on carriers turned off the axes and centres off the
//! origin, the Interval and Dual lanes against the f64 value, and
//! [`rest::by_round`] over a block's corner and edge against a ball, a
//! socket, a pin and a bore, each pose turned about 24 axes.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

// Gated to the reader it checks and the band it decides with.
test_utils::gated_to![
    "crates/topo/src/link.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
];

use geom_core::{Bounds, Dual, Interval};
use test_utils::fuzz::Rng;

use super::*;

fn band() -> Band {
    Band::linear(geom_core::Tol::witness()).expect("the witness tolerance is a band")
}

fn unit(r: &mut Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(r.range(-1.0, 1.0), r.range(-1.0, 1.0), r.range(-1.0, 1.0));
        if v.norm() > 0.2 && v.norm() < 1.0 {
            return v.normalize();
        }
    }
}

fn point(r: &mut Rng, half: f64) -> Point3<f64> {
    Point3::new(
        r.range(-half, half),
        r.range(-half, half),
        r.range(-half, half),
    )
}

/// A sphere or a cylinder face's surface at scalar `T`: centre or axis
/// origin `c`, axis `a` (unused for a sphere), radius `radius`.
fn surface<T: Real>(cyl: bool, c: Point3<f64>, a: Vec3<f64>, radius: f64) -> geom::Surface<T> {
    let pt = |q: Point3<f64>| Point3::new(T::from_f64(q.x), T::from_f64(q.y), T::from_f64(q.z));
    let vc = |q: Vec3<f64>| Vec3::new(T::from_f64(q.x), T::from_f64(q.y), T::from_f64(q.z));
    let u_ref = a.cross(Vec3::new(0.3, -0.5, 0.8)).normalize();
    if cyl {
        geom::Surface::Cylinder {
            origin: pt(c),
            axis: vc(a),
            radius: T::from_f64(radius),
            u_ref: vc(u_ref),
        }
    } else {
        geom::Surface::Sphere {
            center: pt(c),
            radius: T::from_f64(radius),
            axis: vc(a),
            u_ref: vc(u_ref),
        }
    }
}

/// The written-down offset: the distance from the centre or the axis
/// line, less the radius, positive away from the material.
fn written(
    cyl: bool,
    convex: bool,
    c: Point3<f64>,
    a: Vec3<f64>,
    radius: f64,
    q: Point3<f64>,
) -> f64 {
    let rel = q - c;
    let across = if cyl { rel - a * rel.dot(a) } else { rel };
    let d = across.norm() - radius;
    if convex { d } else { -d }
}

/// **The offset is the carrier's signed distance, and the lowest along
/// a segment is the least a dense walk finds**, on spheres off the
/// origin and cylinders whose axes are turned off the coordinate axes,
/// both senses, short and long segments, and segments run near the axis
/// direction. The Interval lane encloses the f64 value over a box about
/// each input, and the Dual lane reads its value. Red if the
/// candidate held to the segment is missing (a ball or a pin reads its
/// ends' least), if a sense is flipped, or if a lane misses.
#[test]
fn a_round_offset_and_its_least_along_a_segment_meet_a_dense_walk() {
    let label = "link_round_rows";
    let mut r = test_utils::fuzz::start(label);
    let replay = test_utils::fuzz::replay();
    let mut bad = Vec::new();
    for k in 0..test_utils::fuzz::scaled(200) {
        let cyl = k % 2 == 0;
        let convex = (k / 2) % 2 == 0;
        let (c, a, radius) = (point(&mut r, 2.0), unit(&mut r), r.log_uniform(0.05, 3.0));
        let round = Round::of(&surface::<f64>(cyl, c, a, radius), convex).expect("round");
        let q = point(&mut r, 3.0);
        let want = written(cyl, convex, c, a, radius, q);
        let at = round.offset(q).expect("off the axis");
        if (at - want).abs() > 1e-12 {
            bad.push(format!("offset at {q:?}: {at} not {want}"));
        }
        let x0 = point(&mut r, 3.0);
        let x1 = if k % 5 == 0 {
            // Nearly along the axis: the quadratic's least runs far off.
            x0 + (a + unit(&mut r) * 1e-7) * r.range(-2.0, 2.0)
        } else {
            point(&mut r, 3.0)
        };
        let walk = (0..=4000)
            .map(|i| {
                written(
                    cyl,
                    convex,
                    c,
                    a,
                    radius,
                    x0 + (x1 - x0) * (f64::from(i) / 4000.0),
                )
            })
            .fold(f64::INFINITY, f64::min);
        let got = round.lowest_on((x0, x1)).expect("off the axis");
        // The offset moves no faster than the point (a distance less a
        // constant), and the true least lies within half a step of a
        // walked point, so it is at least the walk's least less half a
        // step. The reading is a value the segment takes, so it is never
        // above the walk, and never below the true least.
        let step = (x1 - x0).norm() / 4000.0;
        if got > walk + 1e-12 || got < walk - step / 2.0 - 1e-12 {
            bad.push(format!(
                "{} {}: least of {x0:?}–{x1:?} reads {got}, the walk {walk}",
                if cyl { "cylinder" } else { "sphere" },
                if convex { "convex" } else { "concave" }
            ));
        }
        // The Interval lane over a box of 1e-9 about each input.
        let wide = |v: f64| Interval::from_bounds(v - 1e-9, v + 1e-9);
        let wp = |q: Point3<f64>| Point3::new(wide(q.x), wide(q.y), wide(q.z));
        let iround = Round::of(&surface::<Interval>(cyl, c, a, radius), convex).expect("round");
        let ig = iround
            .lowest_on((wp(x0), wp(x1)))
            .expect("an enclosure off the axis");
        if !(ig.lo() <= got && got <= ig.hi()) {
            bad.push(format!("interval [{}, {}] misses {got}", ig.lo(), ig.hi()));
        }
        // The Dual lane's value.
        let dp = |q: Point3<f64>| {
            Point3::new(
                Dual::new(q.x, 1.0),
                Dual::constant(q.y),
                Dual::constant(q.z),
            )
        };
        let dround = Round::of(&surface::<Dual<f64>>(cyl, c, a, radius), convex).expect("round");
        let dg = dround.lowest_on((dp(x0), dp(x1))).expect("off the axis");
        if (dg.value - got).abs() > 1e-12 || !dg.deriv.is_finite() {
            bad.push(format!("dual reads {dg:?}, f64 {got}"));
        }
    }
    assert!(bad.is_empty(), "{replay}\n{}", bad.join("\n"));
}

/// A rotation about the unit `axis` by `angle`.
fn turn(axis: Vec3<f64>, angle: f64) -> impl Fn(Vec3<f64>) -> Vec3<f64> {
    move |w: Vec3<f64>| {
        let (s, c) = angle.sin_cos();
        w * c + axis.cross(w) * s + axis * axis.dot(w) * (1.0 - c)
    }
}

/// Fans about the origin: each face's outward normal and its sector's
/// directions in turn.
type Fans = Vec<(Vec3<f64>, Vec<Vec3<f64>>)>;

/// A link at `at` from fans written about the origin, turned by `rot`.
fn turned_link(at: Point3<f64>, rot: &dyn Fn(Vec3<f64>) -> Vec3<f64>, fans: &Fans) -> Link<f64> {
    let sectors = fans
        .iter()
        .map(|(m, pts)| {
            let pts: Vec<Point3<f64>> = pts.iter().map(|&d| at + rot(d)).collect();
            Sector {
                surface: geom::Surface::Plane {
                    origin: at,
                    normal: rot(*m),
                    u_ref: (pts[0] - at).normalize(),
                },
                sense: true,
                outward: rot(*m),
                apex: at,
                wedges: pts.windows(2).map(|w| (w[0], w[1])).collect(),
                closed: false,
                clear: None,
            }
        })
        .collect();
    Link::of(at, sectors).expect("the fans make a link")
}

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// A block's corner, its material where `x, y, z ≤ 0`.
fn corner() -> Fans {
    let (x, y, z) = (v(-1.0, 0.0, 0.0), v(0.0, -1.0, 0.0), v(0.0, 0.0, -1.0));
    vec![
        (v(1.0, 0.0, 0.0), vec![y, z]),
        (v(0.0, 1.0, 0.0), vec![z, x]),
        (v(0.0, 0.0, 1.0), vec![x, y]),
    ]
}

/// A block's edge along `y`, its material where `x, z ≤ 0`.
fn ridge() -> Fans {
    let (y, ny) = (v(0.0, 1.0, 0.0), v(0.0, -1.0, 0.0));
    vec![
        (v(0.0, 0.0, 1.0), vec![y, v(-1.0, 0.0, 0.0), ny]),
        (v(1.0, 0.0, 0.0), vec![ny, v(0.0, 0.0, -1.0), y]),
    ]
}

/// One pose: the cell, the carrier (cylinder?, its centre or axis point
/// off the touch, its axis, convex?), and what the touch reads.
struct Pose {
    name: &'static str,
    cell: fn() -> Fans,
    cyl: bool,
    centre: Vec3<f64>,
    axis: Vec3<f64>,
    convex: bool,
    rest: bool,
}

/// **A block's corner or edge against a ball, a socket, a pin or a bore
/// reads Rest where the materials only touch and Crossing where they
/// overlap, about any axis.** Each pose is written about the origin,
/// radius 0.7, then turned about 24 axes off the coordinate ones and
/// moved off the origin. The tangent poses are the ones a first-order
/// reading gets wrong: a face lying tangent to a bore enters its wall
/// beside the touch, and one lying tangent to a pin, or an edge on a
/// pin's or a bore's ruling, does not. Red if a pose reads the other
/// verdict, or refuses.
#[test]
fn a_cell_against_a_round_face_reads_by_its_carrier_about_any_axis() {
    let r = 0.7;
    let d = |x: f64, y: f64, z: f64| v(x, y, z).normalize() * r;
    let poses = [
        Pose {
            name: "corner on a ball",
            cell: corner,
            cyl: false,
            centre: d(1.0, 1.0, 1.0),
            axis: v(0.0, 0.0, 1.0),
            convex: true,
            rest: true,
        },
        Pose {
            name: "corner on a ball beside its face",
            cell: corner,
            cyl: false,
            centre: d(1.0, 0.0, 0.0),
            axis: v(0.0, 0.0, 1.0),
            convex: true,
            rest: true,
        },
        Pose {
            name: "corner into a ball",
            cell: corner,
            cyl: false,
            centre: d(1.0, 1.0, -0.5),
            axis: v(0.0, 0.0, 1.0),
            convex: true,
            rest: false,
        },
        Pose {
            name: "corner in a ball",
            cell: corner,
            cyl: false,
            centre: d(-1.0, -1.0, -1.0),
            axis: v(0.0, 0.0, 1.0),
            convex: true,
            rest: false,
        },
        Pose {
            name: "corner in a socket",
            cell: corner,
            cyl: false,
            centre: d(-1.0, -1.0, -1.0),
            axis: v(0.0, 0.0, 1.0),
            convex: false,
            rest: true,
        },
        Pose {
            name: "corner through a socket's wall",
            cell: corner,
            cyl: false,
            centre: d(-1.0, -1.0, 0.3),
            axis: v(0.0, 0.0, 1.0),
            convex: false,
            rest: false,
        },
        Pose {
            name: "edge on a pin's ruling",
            cell: ridge,
            cyl: true,
            centre: d(1.0, 0.0, 1.0),
            axis: v(0.0, 1.0, 0.0),
            convex: true,
            rest: true,
        },
        Pose {
            name: "face tangent to a pin",
            cell: ridge,
            cyl: true,
            centre: d(0.0, 0.0, 1.0),
            axis: v(0.0, 1.0, 0.0),
            convex: true,
            rest: true,
        },
        Pose {
            name: "edge across a pin",
            cell: ridge,
            cyl: true,
            centre: d(1.0, 0.0, 1.0),
            axis: v(1.0, 0.0, -1.0).normalize(),
            convex: true,
            rest: true,
        },
        Pose {
            name: "edge into a pin",
            cell: ridge,
            cyl: true,
            centre: d(1.0, 0.0, -0.4),
            axis: v(0.0, 1.0, 0.0),
            convex: true,
            rest: false,
        },
        Pose {
            name: "edge on a bore's ruling",
            cell: ridge,
            cyl: true,
            centre: d(-1.0, 0.0, -1.0),
            axis: v(0.0, 1.0, 0.0),
            convex: false,
            rest: true,
        },
        Pose {
            name: "face tangent to a bore",
            cell: ridge,
            cyl: true,
            centre: d(0.0, 0.0, -1.0),
            axis: v(0.0, 1.0, 0.0),
            convex: false,
            rest: false,
        },
        Pose {
            name: "edge across a bore",
            cell: ridge,
            cyl: true,
            centre: d(-1.0, 0.0, -1.0),
            axis: v(1.0, 0.0, -1.0).normalize(),
            convex: false,
            rest: false,
        },
        Pose {
            name: "corner on a bore's wall",
            cell: corner,
            cyl: true,
            centre: d(-1.0, -1.0, -1.0),
            axis: v(1.0, -1.0, 0.0).normalize(),
            convex: false,
            rest: true,
        },
    ];
    let b = band();
    let mut bad = Vec::new();
    for k in 0..24 {
        let kf = f64::from(k);
        let axis = v(
            1.0 + 0.37 * kf.sin(),
            0.6 * (1.3 * kf).cos(),
            0.25 + 0.5 * (0.7 * kf).sin(),
        )
        .normalize();
        let rot = turn(axis, 0.3 + 0.21 * kf);
        let at = Point3::new(0.3 * kf.cos(), -1.1 + 0.05 * kf, 0.7 * (0.4 * kf).sin());
        for pose in &poses {
            let link = turned_link(at, &rot, &(pose.cell)());
            let round = Round::of(
                &surface::<f64>(pose.cyl, at + rot(pose.centre), rot(pose.axis), r),
                pose.convex,
            )
            .expect("round");
            let got = rest::by_round(&link, &round, 0.5, b);
            let ok = match got {
                TouchVerdict::Rest(_) => pose.rest,
                TouchVerdict::Crossing => !pose.rest,
                TouchVerdict::Unread(_) => false,
            };
            if !ok {
                bad.push(format!("axis {k}, {}: reads {got:?}", pose.name));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {}:\n{}",
        bad.len(),
        24 * poses.len(),
        bad.join("\n")
    );
}

/// **On the axis or at the centre, the Dual lane reads the radius's
/// derivative and refuses a point moved across.** A pin and a ball,
/// the radius seeded: a vertex on the axis or at the centre, and an
/// edge along the axis, across it and through the centre, each read
/// `∂g/∂r = −1` (a pin's or a ball's offset is the distance less the
/// radius), finite. A vertex on the axis seeded across it has no
/// derivative there, and reads [`AtTheAxis`] rather than a NaN. The
/// cylinder is turned off the axes and the centre off the origin; the
/// vertex on the axis is the axis's own origin, so it lies on it exactly
/// (a point a rounding off the axis has the derivative of its own,
/// rounded, direction from it).
/// Red if a NaN is minted or a defined derivative refused.
#[test]
fn the_dual_lane_at_the_axis_reads_the_radius_and_refuses_a_point_moved_across() {
    let c = |x: f64| Dual::constant(x);
    let cp = |q: Point3<f64>| Point3::new(c(q.x), c(q.y), c(q.z));
    let cv = |q: Vec3<f64>| Vec3::new(c(q.x), c(q.y), c(q.z));
    let o = Point3::new(0.4, -0.3, 1.1);
    let a = Vec3::new(0.3, -0.5, 0.8).normalize();
    let across = Vec3::new(0.8, 0.0, -0.3).normalize();
    let pin = Round::cylinder(cp(o), cv(a), Dual::variable(1.0), true);
    let ball = Round::sphere(cp(o), Dual::variable(1.0), true);
    let mut bad = Vec::new();
    let mut check = |what: &str, got: Result<Dual<f64>, AtTheAxis>, value: f64| match got {
        Ok(g) if (g.value - value).abs() < 1e-12 && (g.deriv + 1.0).abs() < 1e-12 => {}
        got => bad.push(format!("{what}: {got:?}")),
    };
    let on = o;
    check("vertex on the axis", pin.offset(cp(on)), -1.0);
    check("vertex at the centre", ball.offset(cp(o)), -1.0);
    check(
        "edge along the axis",
        pin.lowest_on((cp(o + a * 0.2), cp(o + a * 0.9))),
        -1.0,
    );
    check(
        "edge across the axis",
        pin.lowest_on((cp(on - across * 2.0), cp(on + across * 2.0))),
        -1.0,
    );
    check(
        "edge through the centre",
        ball.lowest_on((cp(o - across * 2.0), cp(o + across * 2.0))),
        -1.0,
    );
    let moved = Point3::new(Dual::variable(on.x), c(on.y), c(on.z));
    let fixed = Round::cylinder(cp(o), cv(a), c(1.0), true);
    if fixed.offset(moved).err() != Some(AtTheAxis) {
        bad.push(format!(
            "a vertex moved across the axis: {:?}",
            fixed.offset(moved)
        ));
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **A corner whose edges enter a bore's hollow at a shallow angle and
/// meet its wall again within reach crosses** (the review's probe of
/// #5827). The bore has radius 1; three rays dip `0.05` rad into the
/// hollow, so each meets the wall again `2·sin 0.05 ≈ 0.1` from the
/// touch, well inside the link's ball. A first-order reading (every
/// ray above the tangent plane) reads Rest. Red if it does.
#[test]
fn a_corner_meeting_a_bores_wall_again_within_reach_crosses() {
    let p = Point3::new(1.0, 0.0, 0.0);
    let theta: f64 = 0.05;
    let len = 0.4;
    let dir = |phi: f64| {
        v(
            -theta.sin(),
            theta.cos() * phi.cos(),
            theta.cos() * phi.sin(),
        )
    };
    let (d1, d2) = (dir(-0.15), dir(0.15));
    let d3 = (dir(0.0) + v(-0.03, 0.0, 0.0)).normalize();
    let centroid = (d1 + d2 + d3) * (1.0 / 3.0);
    let sectors = [(d1, d2), (d2, d3), (d3, d1)]
        .into_iter()
        .map(|(x, y)| {
            let (mut m, mut pts) = (x.cross(y).normalize(), vec![p + x * len, p + y * len]);
            if m.dot(centroid) > 0.0 {
                m = -m;
                pts.reverse();
            }
            Sector {
                surface: geom::Surface::Plane {
                    origin: p,
                    normal: m,
                    u_ref: (pts[0] - p).normalize(),
                },
                sense: true,
                outward: m,
                apex: p,
                wedges: vec![(pts[0], pts[1])],
                closed: false,
                clear: None,
            }
        })
        .collect();
    let link = Link::of(p, sectors).expect("the fans make a link");
    let bore = Round::cylinder(Point3::origin(), v(0.0, 0.0, 1.0), 1.0, false);
    assert_eq!(
        rest::by_round(&link, &bore, 0.5, band()),
        TouchVerdict::Crossing,
        "the rays meet the wall again within reach"
    );
}
