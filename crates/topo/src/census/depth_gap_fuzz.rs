//! **The depth bounds are bounds**: [`depth_gap`]'s read of a depth's
//! least and greatest value over a cover ([`Cover::range_about`], the
//! cover's distance range from an axis line, a centre or a core circle),
//! over a tube, a torus's or a sphere's surface, a torus's chart patch,
//! a cylinder's wall, a cone's frustum or a segment, against the exact
//! depth of points sampled on it.
//!
//! A shape-1 counterexample search: random tubes, surfaces, patches,
//! walls, frustums and segments of every pose against random axes,
//! centres and core circles, a carrier's on a frame skewed off unit and
//! orthogonal, as its evaluator reads one, horn tori (`|R| = |r|`
//! exactly) among them, and its depth half the time the site it is
//! measured about ([`Cover::about`]), where on a level the bounds are
//! tight; half the rings
//! coaxial with the depth's own axis, the pose the census meets most
//! and where the bounds are tight. A ring is sampled inside its tube
//! and on the tube's surface, where its depths are extreme, and a
//! quarter of the draws stand 1e3 to 1e4 from the origin. A sample past
//! a bound is a clear the census would grant across a face it does not
//! hold.
//!
//! Two lanes. In `f64` a bound may round past a sample by at most
//! [`REL`] of the draw's scale, the well-posedness the depth's forms
//! owe. In `Interval` the bound encloses the closed form's exact value,
//! so only the samples' own rounding stands between them.

#![allow(clippy::unwrap_used, clippy::expect_used)]

test_utils::gated_to![
    "crates/topo/src/census/depth_gap.rs",
    "crates/topo/src/census.rs",
    "crates/geom/src/cover.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/linalg/",
];

use geom::cover::{Cover, ParamWindow};
use geom_core::{Bounds, Interval, Point3, Real, Vec3};
use test_utils::fuzz::Rng;

use geom::cover::{Around, Core};

use super::depth_gap::split;

/// A drawn piece: a tube (a circle where `minor` is zero) about a unit
/// `normal`, a torus's surface (a sphere's where `major` is zero) or a
/// sphere's on any frame, a ring torus's chart patch, a cylinder's wall
/// or a cone's frustum over heights along its unit axis, or a segment.
#[derive(Clone, Copy, Debug)]
enum Piece<T: Real> {
    Tube {
        center: Point3<T>,
        normal: Vec3<T>,
        u_ref: Vec3<T>,
        major: T,
        minor: T,
        circle: bool,
    },
    Surface {
        frame: geom::Frame<T>,
        major: T,
        minor: T,
    },
    Sphere {
        frame: geom::Frame<T>,
        radius: T,
    },
    Patch {
        frame: geom::Frame<T>,
        major: T,
        minor: T,
        u: (T, T),
        v: (T, T),
    },
    Cone {
        frame: geom::Frame<T>,
        half_angle: T,
        heights: (T, T),
    },
    Wall {
        frame: geom::Frame<T>,
        radius: T,
        heights: (T, T),
    },
    Segment(Point3<T>, Point3<T>),
}

fn cover_of<T: Real>(piece: Piece<T>) -> Cover<T> {
    match piece {
        Piece::Tube {
            center,
            normal,
            u_ref,
            major,
            circle: true,
            ..
        } => Cover::arc(
            &geom::Frame::new(center, normal, u_ref),
            (major, major),
            ParamWindow::new(T::zero(), T::tau()),
        ),
        Piece::Tube {
            center,
            normal,
            u_ref,
            major,
            minor,
            ..
        } => Cover::tube(&geom::Frame::new(center, normal, u_ref), major, minor),
        Piece::Surface {
            frame,
            major,
            minor,
        } => Cover::torus(&frame, major, minor),
        Piece::Sphere { frame, radius } => Cover::sphere(&frame, radius),
        Piece::Patch {
            frame,
            major,
            minor,
            u,
            v,
        } => Cover::patch(
            &frame,
            (major, minor),
            Some(ParamWindow::new(u.0, u.1)),
            ParamWindow::new(v.0, v.1),
        ),
        Piece::Cone {
            frame,
            half_angle,
            heights: (lo, hi),
        } => {
            let a = frame.axis.normalize();
            let within = Cover::segment(frame.origin + a * lo, frame.origin + a * hi).unwrap();
            Cover::cone(&frame, half_angle, &within)
        }
        Piece::Wall {
            frame,
            radius,
            heights: (lo, hi),
        } => {
            let a = frame.axis.normalize();
            let within = Cover::segment(frame.origin + a * lo, frame.origin + a * hi).unwrap();
            Cover::wall(&frame, radius, &within)
        }
        Piece::Segment(a, b) => Cover::segment(a, b),
    }
    .unwrap()
}

/// How far an `f64` bound may round past a sample, relative to the
/// draw's scale (`1 +` its largest coordinate, plus its reach). The
/// worst of four runs at effort 400 stood 1.6e-15 past.
const REL: f64 = 1e-14;

/// How far a sample's own rounding may carry it off its piece,
/// relative to the draw's scale, in the `Interval` lane: the worst of
/// four runs at effort 400 (240 000 draws each) stood 1.1e-15 past, at
/// seed `0x7925458d5bba5dab`; a run at effort 1 typically sees ~1e-17.
const SAMPLE_REL: f64 = 5e-15;

/// The exact depth of the point `x`: the oracle.
fn exact<T: geom_core::Decide>(depth: Around<T>, x: Point3<T>) -> T {
    match depth {
        Around::Line(line) => line.distance(x),
        Around::Point(center) => (x - center).norm(),
        Around::Circle(Core {
            axis:
                geom::Line {
                    origin: center,
                    dir: axis,
                },
            radius: major,
        }) => {
            let (off, t) = split(x - center, axis);
            ((off.norm() - major.abs()).powi(2) + t.powi(2)).sqrt()
        }
    }
}

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn unit(r: &mut Rng) -> Vec3<f64> {
    loop {
        let w = Vec3::new(r.range(-1.0, 1.0), r.range(-1.0, 1.0), r.range(-1.0, 1.0));
        if w.norm() > 0.1 && w.norm() < 1.0 {
            return w / w.norm();
        }
    }
}

fn centre(r: &mut Rng, far: Vec3<f64>) -> Point3<f64> {
    p(r.range(-2.0, 2.0), r.range(-2.0, 2.0), r.range(-2.0, 2.0)) + far
}

/// One draw: a depth, a piece, points sampled on the piece, and the
/// draw's scale.
fn draw(r: &mut Rng, i: usize) -> (Around<f64>, Piece<f64>, Vec<Point3<f64>>, f64) {
    let far = if i % 4 == 3 {
        let mut c = || r.range(1.0e3, 1.0e4) * if r.unit() < 0.5 { -1.0 } else { 1.0 };
        Vec3::new(c(), c(), c())
    } else {
        Vec3::new(0.0, 0.0, 0.0)
    };
    let scale = 1.0 + far.x.abs().max(far.y.abs()).max(far.z.abs()) + 4.0;
    let n = unit(r);
    let coaxial = i.is_multiple_of(2);
    let axis = if coaxial { n } else { unit(r) } * r.range(0.5, 2.0);
    let origin = centre(r, far);
    let depth = match i % 3 {
        0 => Around::Line(geom::Line { origin, dir: axis }),
        1 => Around::Point(origin),
        _ => Around::Circle(Core {
            axis: geom::Line {
                origin,
                dir: axis,
            },
            radius: r.range(0.0, 2.0),
        }),
    };
    if i % 7 == 2 || i % 7 == 4 {
        return carrier(r, i, depth, far, scale);
    }
    if i % 7 == 6 {
        let (a, b) = (centre(r, far), centre(r, far));
        let samples = (0..=64)
            .map(|k| a + (b - a) * (f64::from(k) / 64.0))
            .collect();
        return (depth, Piece::Segment(a, b), samples, scale);
    }
    let center = if coaxial {
        origin + n * r.range(-1.0, 1.0)
    } else {
        centre(r, far)
    };
    let (major, minor) = (
        r.range(0.0, 2.0),
        if i.is_multiple_of(5) {
            0.0
        } else {
            r.range(0.0, 0.5)
        },
    );
    let e1 = split(unit(r), n).0;
    let e1 = e1 / e1.norm();
    let e2 = n.cross(e1);
    let samples = (0..128)
        .map(|k| {
            let th = f64::from(k / 2) * core::f64::consts::TAU / 64.0 + r.range(0.0, 0.1);
            let radial = e1 * th.cos() + e2 * th.sin();
            let on = center + radial * major;
            if k % 2 == 0 {
                on + unit(r) * (minor * r.unit())
            } else {
                // On the tube's surface, in the circle's meridian plane.
                let phi = r.range(0.0, core::f64::consts::TAU);
                on + (radial * phi.cos() + n * phi.sin()) * minor
            }
        })
        .collect();
    let tube = Piece::Tube {
        center,
        normal: n,
        u_ref: e1,
        major,
        minor,
        circle: minor == 0.0,
    };
    (depth, tube, samples, scale)
}

fn iv(x: f64) -> Interval {
    Interval::from_bounds(x, x)
}

fn ip(x: Point3<f64>) -> Point3<Interval> {
    Point3::new(iv(x.x), iv(x.y), iv(x.z))
}

fn ivec(x: Vec3<f64>) -> Vec3<Interval> {
    x.map(iv)
}

fn lift_depth(d: Around<f64>) -> Around<Interval> {
    match d {
        Around::Line(geom::Line { origin, dir: axis }) => Around::Line(geom::Line {
            origin: ip(origin),
            dir: ivec(axis),
        }),
        Around::Point(center) => Around::Point(ip(center)),
        Around::Circle(Core {
            axis:
                geom::Line {
                    origin: center,
                    dir: axis,
                },
            radius,
        }) => Around::Circle(Core {
            axis: geom::Line {
                origin: ip(center),
                dir: ivec(axis),
            },
            radius: iv(radius),
        }),
    }
}

fn lift_frame(f: geom::Frame<f64>) -> geom::Frame<Interval> {
    geom::Frame::new(ip(f.origin), ivec(f.axis), ivec(f.u_ref))
}

fn lift_piece(piece: Piece<f64>) -> Piece<Interval> {
    match piece {
        Piece::Tube {
            center,
            normal,
            u_ref,
            major,
            minor,
            circle,
        } => Piece::Tube {
            center: ip(center),
            normal: ivec(normal),
            u_ref: ivec(u_ref),
            major: iv(major),
            minor: iv(minor),
            circle,
        },
        Piece::Surface {
            frame,
            major,
            minor,
        } => Piece::Surface {
            frame: lift_frame(frame),
            major: iv(major),
            minor: iv(minor),
        },
        Piece::Sphere { frame, radius } => Piece::Sphere {
            frame: lift_frame(frame),
            radius: iv(radius),
        },
        Piece::Patch {
            frame,
            major,
            minor,
            u,
            v,
        } => Piece::Patch {
            frame: lift_frame(frame),
            major: iv(major),
            minor: iv(minor),
            u: (iv(u.0), iv(u.1)),
            v: (iv(v.0), iv(v.1)),
        },
        Piece::Cone {
            frame,
            half_angle,
            heights: (lo, hi),
        } => Piece::Cone {
            frame: lift_frame(frame),
            half_angle: iv(half_angle),
            heights: (iv(lo), iv(hi)),
        },
        Piece::Wall {
            frame,
            radius,
            heights: (lo, hi),
        } => Piece::Wall {
            frame: lift_frame(frame),
            radius: iv(radius),
            heights: (iv(lo), iv(hi)),
        },
        Piece::Segment(a, b) => Piece::Segment(ip(a), ip(b)),
    }
}

/// A carrier draw: a torus's or a sphere's surface (a horn torus's
/// exactly `|R| = |r|` one ring draw in five), a ring torus's chart
/// patch, a cylinder's wall or a cone's frustum, on a frame skewed up to
/// 5% off unit and orthogonal, with signed radii, sampled through its
/// evaluator's formula; its depth is the first site it is measured about
/// on odd draws, and a patch's its core on even draws in four.
fn carrier(
    r: &mut Rng,
    i: usize,
    depth: Around<f64>,
    far: Vec3<f64>,
    scale: f64,
) -> (Around<f64>, Piece<f64>, Vec<Point3<f64>>, f64) {
    let n = unit(r);
    let e1 = split(unit(r), n).0;
    let skew = |r: &mut Rng| {
        Vec3::new(
            r.range(-0.05, 0.05),
            r.range(-0.05, 0.05),
            r.range(-0.05, 0.05),
        )
    };
    let a = (n + skew(r)) * r.range(0.95, 1.05);
    let u = e1 / e1.norm() + skew(r);
    let frame = geom::Frame::new(centre(r, far), a, u);
    let sign = |r: &mut Rng| if r.unit() < 0.5 { -1.0 } else { 1.0 };
    let rho = |t: f64| u * t.cos() + a.cross(u) * t.sin();
    let tau = core::f64::consts::TAU;
    let on = |t: f64, v: f64, (major, minor): (f64, f64)| {
        frame.origin + rho(t) * (major + minor * v.cos()) + a * (minor * v.sin())
    };
    let mut depth = depth;
    let (piece, samples): (Piece<f64>, Vec<Point3<f64>>) = if i % 7 == 2
        && i % 3 == 2
        && i.is_multiple_of(2)
    {
        // A ring patch: `R ≥ |r|`, `R` positive, as its premise reads.
        let minor = r.range(0.05, 1.0) * sign(r);
        let major = minor.abs() * r.range(1.0, 3.0);
        let (u0, v0) = (r.range(-tau, tau), r.range(-tau, tau));
        let (u, v) = ((u0, u0 + r.range(0.1, 3.0)), (v0, v0 + r.range(0.1, 3.0)));
        let samples = (0..128)
            .map(|k| {
                let (t, w) = if k < 4 {
                    ([u.0, u.1][k % 2], [v.0, v.1][k / 2])
                } else {
                    (r.range(u.0, u.1), r.range(v.0, v.1))
                };
                on(t, w, (major, minor))
            })
            .collect();
        if i.is_multiple_of(4) {
            depth = Around::Circle(Core {
                axis: geom::Line {
                    origin: frame.origin,
                    dir: a,
                },
                radius: major,
            });
        }
        (
            Piece::Patch {
                frame,
                major,
                minor,
                u,
                v,
            },
            samples,
        )
    } else if i % 7 == 2 {
        let minor = r.range(0.05, 1.0) * sign(r);
        let major = match i % 3 {
            0 => 0.0,
            1 => minor.abs() * r.range(0.5, 1.0),
            _ if i.is_multiple_of(5) => minor.abs(),
            _ => minor.abs() * r.range(1.0, 3.0),
        } * sign(r);
        let samples = (0..128)
            .map(|_| on(r.range(0.0, tau), r.range(0.0, tau), (major, minor)))
            .collect();
        let piece = if major == 0.0 && i % 4 < 2 {
            Piece::Sphere {
                frame,
                radius: minor,
            }
        } else {
            Piece::Surface {
                frame,
                major,
                minor,
            }
        };
        (piece, samples)
    } else if i.is_multiple_of(3) {
        let half_angle = r.range(0.1, 1.2) * sign(r);
        let (s_a, c_a) = half_angle.sin_cos();
        let (v0, v1) = (r.range(-2.0, 0.0), r.range(0.0, 2.0));
        let samples = (0..128)
            .map(|k| {
                let v = if k < 4 {
                    [v0, v1][k % 2]
                } else {
                    r.range(v0, v1)
                };
                frame.origin + (a * c_a + rho(r.range(0.0, tau)) * s_a) * v
            })
            .collect::<Vec<_>>();
        let unit_axis = a / a.norm();
        let heights = samples
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &x| {
                let h = (x - frame.origin).dot(unit_axis);
                (lo.min(h), hi.max(h))
            });
        (
            Piece::Cone {
                frame,
                half_angle,
                heights,
            },
            samples,
        )
    } else {
        let radius = r.range(0.05, 2.0) * sign(r);
        let (v0, v1) = (r.range(-2.0, 0.0), r.range(0.0, 2.0));
        let samples = (0..128)
            .map(|k| {
                let v = if k < 4 {
                    [v0, v1][k % 2]
                } else {
                    r.range(v0, v1)
                };
                frame.origin + rho(r.range(0.0, tau)) * radius + a * v
            })
            .collect::<Vec<_>>();
        let unit_axis = a / a.norm();
        let heights = samples
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &x| {
                let h = (x - frame.origin).dot(unit_axis);
                (lo.min(h), hi.max(h))
            });
        (
            Piece::Wall {
                frame,
                radius,
                heights,
            },
            samples,
        )
    };
    if i % 2 == 1 {
        depth = cover_of(piece).about().first().copied().unwrap_or(depth);
    }
    (depth, piece, samples, scale)
}

#[test]
fn no_sampled_depth_lies_past_its_bounds() {
    let mut r = test_utils::fuzz::start("census_depth_gap_bounds");
    let (mut over, mut under) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
    let mut sampled = f64::NEG_INFINITY;
    for i in 0..test_utils::fuzz::scaled(600) {
        let (depth, piece, samples, scale) = draw(&mut r, i);
        let (lo, hi) = cover_of(piece).range_about(&depth);
        let (idepth, ipiece) = (lift_depth(depth), lift_piece(piece));
        let (ilo, ihi) = cover_of(ipiece).range_about(&idepth);
        for x in samples {
            let d = exact(depth, x);
            let id = exact(idepth, ip(x));
            {
                over = over.max((d - hi) / scale);
                assert!(
                    d - hi <= REL * scale,
                    "draw {i}: {x:?} on {piece:?} is {:e} deeper than the bound {hi} on \
                     {depth:?}; {}",
                    d - hi,
                    test_utils::fuzz::replay()
                );
                sampled = sampled.max((id.lo() - ihi.hi()) / scale);
                assert!(
                    id.lo() - ihi.hi() <= SAMPLE_REL * scale,
                    "draw {i}, Interval: {x:?} on {piece:?} is {:e} deeper than the bound \
                     {ihi:?} on {depth:?}; {}",
                    id.lo() - ihi.hi(),
                    test_utils::fuzz::replay()
                );
            }
            {
                under = under.max((lo - d) / scale);
                assert!(
                    lo - d <= REL * scale,
                    "draw {i}: {x:?} on {piece:?} is {:e} shallower than the bound {lo} on \
                     {depth:?}; {}",
                    lo - d,
                    test_utils::fuzz::replay()
                );
                sampled = sampled.max((ilo.lo() - id.hi()) / scale);
                assert!(
                    ilo.lo() - id.hi() <= SAMPLE_REL * scale,
                    "draw {i}, Interval: {x:?} on {piece:?} is {:e} shallower than the bound \
                     {ilo:?} on {depth:?}; {}",
                    ilo.lo() - id.hi(),
                    test_utils::fuzz::replay()
                );
            }
        }
    }
    println!(
        "[fuzz] the worst samples stood {over:e} past the upper and {under:e} past the lower, \
         and {sampled:e} past an Interval bound, relative to their draws' scale"
    );
}
