//! The cone's vector-area lane decides every edge onto the cone at the
//! edge's TRUE distance from it, and every conic span within a period.
//!
//! Each incidence row perturbs one edge of a loop the lane answers by a
//! known distance `d` from the cone, measured on the perturbed carrier
//! itself ([`max_elevation`]), and asks for an answer at `d = zero/10`
//! and a refusal under the decide's own name at `d = 2·escalate`. A
//! decide levered past the true distance refuses the first; one levered
//! short of it lets the second through to a later decide, which refuses
//! it under the wrong name. The cones are the ends of the half-angle
//! range, about tilted axes through apexes off the origin, at `f64` and
//! at the interval scalar.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::band;
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, boundary_material_sign_loops, curved_face_loops};
use geom_core::{Decide, Interval, Real};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;
const HALF_PI: f64 = core::f64::consts::FRAC_PI_2;

/// The half-angles: both ends of the range, each at two depths.
const ALPHAS: [f64; 4] = [1e-6, 1e-3, HALF_PI - 1e-3, HALF_PI - 1e-6];

type V = [f64; 3];

fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn add(a: V, b: V, s: f64) -> V {
    [a[0] + s * b[0], a[1] + s * b[1], a[2] + s * b[2]]
}
fn unit(a: V) -> V {
    let n = dot(a, a).sqrt();
    [a[0] / n, a[1] / n, a[2] / n]
}
fn pt<T: Real>(a: V) -> geom_core::Point3<T> {
    p3(a[0], a[1], a[2])
}
fn vt<T: Real>(a: V) -> geom_core::Vec3<T> {
    v3(a[0], a[1], a[2])
}

/// A cone and its chart frame `(e1, e2, a)`.
#[derive(Clone, Copy, Debug)]
struct Cone {
    apex: V,
    a: V,
    e1: V,
    e2: V,
    alpha: f64,
}

impl Cone {
    fn new(apex: V, axis: V, alpha: f64) -> Self {
        let a = unit(axis);
        let seed = if a[0].abs() < 0.9 {
            [1.0, 0.0, 0.0]
        } else {
            [0.0, 1.0, 0.0]
        };
        let e1 = unit(add(seed, a, -dot(seed, a)));
        Cone {
            apex,
            a,
            e1,
            e2: cross(a, e1),
            alpha,
        }
    }

    /// The cones every row runs on: each half-angle, about a tilted and
    /// a skew axis, through two apexes off the origin.
    fn all() -> Vec<Cone> {
        let mut v = Vec::new();
        for alpha in ALPHAS {
            for apex in [[0.2, -0.1, -1.0], [3.1, -1.7, 0.9]] {
                for axis in [[0.3, -0.4, 1.0], [1.0, 1.0, 0.1]] {
                    v.push(Cone::new(apex, axis, alpha));
                }
            }
        }
        v
    }

    fn surface<T: Real>(&self) -> Surface<T> {
        Surface::Cone {
            apex: pt(self.apex),
            axis: vt(self.a),
            half_angle: T::from_f64(self.alpha),
            u_ref: vt(self.e1),
        }
    }

    /// The generator's unit direction at azimuth `u`.
    fn g(&self, u: f64) -> V {
        let (s, c) = self.alpha.sin_cos();
        add(
            add(add([0.0; 3], self.e1, s * u.cos()), self.e2, s * u.sin()),
            self.a,
            c,
        )
    }

    /// The exact distance from the cone of `p` (both nappes).
    fn elevation(&self, p: V) -> f64 {
        let w = add(p, self.apex, -1.0);
        let h = dot(w, self.a);
        let rho = add(w, self.a, -h);
        let (s, c) = self.alpha.sin_cos();
        dot(rho, rho).sqrt() * c - h.abs() * s
    }

    fn rim_circle(&self, v: f64) -> Curve3<f64> {
        let (s, c) = self.alpha.sin_cos();
        Curve3::Circle {
            center: pt(add(self.apex, self.a, v * c)),
            axis: vt(self.a),
            radius: v * s,
            u_ref: vt(self.e1),
        }
    }

    fn generator(&self, dir: V) -> Curve3<f64> {
        Curve3::Line {
            origin: pt(self.apex),
            dir: vt(dir),
        }
    }

    /// The azimuth of `p` in the chart frame.
    fn azimuth(&self, p: V) -> f64 {
        let w = add(p, self.apex, -1.0);
        dot(w, self.e2).atan2(dot(w, self.e1))
    }
}

/// The slant every fixture is built at: 2, or as far out as puts its
/// rims a thousand escalation bands from the axis and from the apex's
/// plane. Inside the first a needle's face is a sliver at the run's ε,
/// and inside the second a near-flat cone's two nappes are within the
/// perturbations of each other.
fn slant(c: &Cone) -> f64 {
    let (s, co) = c.alpha.sin_cos();
    (1e3 * band().escalate() / s.min(co)).max(2.0)
}

/// The farthest a carrier strays from the cone over `[t0, t1]`, sampled
/// densely: the fixture's own measure of the distance it was built at.
fn max_elevation(c: &Cone, e: &Curve3<f64>, t0: f64, t1: f64) -> f64 {
    (0..=4096)
        .map(|i| {
            let p = e.eval(t0 + (t1 - t0) * f64::from(i) / 4096.0);
            c.elevation([p.x, p.y, p.z]).abs()
        })
        .fold(0.0, f64::max)
}

/// A curve's coordinates lifted exactly into `T`.
fn lift<T: Real>(e: &Curve3<f64>) -> Curve3<T> {
    let p = |q: geom_core::Point3<f64>| pt::<T>([q.x, q.y, q.z]);
    let v = |q: geom_core::Vec3<f64>| vt::<T>([q.x, q.y, q.z]);
    match *e {
        Curve3::Line { origin, dir } => Curve3::Line {
            origin: p(origin),
            dir: v(dir),
        },
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => Curve3::Circle {
            center: p(center),
            axis: v(axis),
            radius: T::from_f64(radius),
            u_ref: v(u_ref),
        },
        Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => Curve3::Ellipse {
            center: p(center),
            axis: v(axis),
            major: T::from_f64(major),
            minor: T::from_f64(minor),
            u_ref: v(u_ref),
        },
        _ => unreachable!(),
    }
}

/// A loop as `(carrier, from, to)` edges, built at `T`.
type Spec = Vec<(Curve3<f64>, f64, f64)>;

fn build<T: Real>(spec: &Spec) -> Vec<LoopEdge<T>> {
    let n = u32::try_from(spec.len()).unwrap();
    spec.iter()
        .zip(0..)
        .map(|((e, a, b), i)| topo::edge(lift::<T>(e), *a, *b, i, (i + 1) % n))
        .collect()
}

fn measure<T: Decide>(c: &Cone, spec: &Spec) -> Result<(), PropsError> {
    let edges = build::<T>(spec);
    curved_face_loops(&c.surface::<T>(), &[&edges], true, band()).map(|_| ())
}

/// The flux lane at `f64` and at the interval scalar, and the material
/// sign at `f64`, which reads the loops by the same decides.
fn verdicts(c: &Cone, spec: &Spec) -> [(&'static str, Result<(), PropsError>); 3] {
    let edges = build::<f64>(spec);
    [
        ("f64", measure::<f64>(c, spec)),
        ("interval", measure::<Interval>(c, spec)),
        (
            "material sign",
            boundary_material_sign_loops(&c.surface::<f64>(), &[&edges], band()).map(|_| ()),
        ),
    ]
}

/// One incidence row's fixture: the loop with its first edge stated `d`
/// off the cone, and the decide that must refuse it.
struct Perturbed {
    name: &'static str,
    build: fn(&Cone, f64) -> Spec,
}

/// The apex cap: one full rim.
fn cap(c: &Cone, rim: Curve3<f64>) -> Spec {
    let _ = c;
    vec![(rim, 0.0, TAU)]
}

/// The rim `d` too wide for its height.
fn rim_too_wide(c: &Cone, d: f64) -> Spec {
    let (s, co) = c.alpha.sin_cos();
    let l = slant(c);
    cap(
        c,
        Curve3::Circle {
            center: pt(add(c.apex, c.a, l * co)),
            axis: vt(c.a),
            radius: l * s + d / co,
            u_ref: vt(c.e1),
        },
    )
}

/// The rim tilted by `θ` about `e2`, with
/// `r·(cos α − cos(α + θ)) = d` — its farthest point, opposite the tilt,
/// moved off the cone by `r·(sin θ·sin α + (1 − cos θ)·cos α)`.
fn rim_tilted(c: &Cone, d: f64) -> Spec {
    let (s, co) = c.alpha.sin_cos();
    let l = slant(c);
    let r = l * s;
    // `2·sin(α + θ/2)·sin(θ/2) = d/r`, solved for `θ` by bisection.
    let f = |th: f64| 2.0 * (c.alpha + th / 2.0).sin() * (th / 2.0).sin() - d / r;
    let (mut lo, mut hi) = (0.0, HALF_PI);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if f(mid) > 0.0 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let th = 0.5 * (lo + hi);
    let n = add(add([0.0; 3], c.a, th.cos()), c.e1, th.sin());
    let u = add(add([0.0; 3], c.e1, th.cos()), c.a, -th.sin());
    cap(
        c,
        Curve3::Circle {
            center: pt(add(c.apex, c.a, l * co)),
            axis: vt(n),
            radius: r,
            u_ref: vt(u),
        },
    )
}

/// The rim with its centre `d / cos α` off the axis.
fn rim_off_axis(c: &Cone, d: f64) -> Spec {
    let (s, co) = c.alpha.sin_cos();
    let l = slant(c);
    cap(
        c,
        Curve3::Circle {
            center: pt(add(add(c.apex, c.a, l * co), c.e1, d / co)),
            axis: vt(c.a),
            radius: l * s,
            u_ref: vt(c.e1),
        },
    )
}

/// The wall `u ∈ [0, 1]`, `v ∈ [3l/4, l]` at `l` the [`slant`], its
/// generator at `u = 1` turned about the apex in its meridian plane,
/// toward the axis, by `δ` with `l·sin δ = d`: its far end is `d` off
/// the cone, and it is stated first, so its own decides are the first
/// to read it. Its reach from the apex is four times its span.
fn generator_turned(c: &Cone, d: f64) -> Spec {
    let (s, co) = c.alpha.sin_cos();
    let (near, far) = (0.75 * slant(c), slant(c));
    let g = c.g(1.0);
    let radial = unit(add(g, c.a, -co));
    let inward = add(add([0.0; 3], c.a, s), radial, -co);
    let delta = (d / far).asin();
    let dir = unit(add(add([0.0; 3], g, delta.cos()), inward, delta.sin()));
    vec![
        (c.generator(dir), near, far),
        (c.rim_circle(far), 1.0, 0.0),
        (c.generator(c.g(0.0)), far, near),
        (c.rim_circle(near), 0.0, 1.0),
    ]
}

/// As [`generator_turned`], the generator moved `d` along the cone's
/// normal, so it misses the apex by `d`.
fn generator_off_apex(c: &Cone, d: f64) -> Spec {
    let (s, co) = c.alpha.sin_cos();
    let g = c.g(1.0);
    let radial = unit(add(g, c.a, -co));
    let normal = add(add([0.0; 3], radial, co), c.a, -s);
    let mut spec = generator_turned(c, 0.0);
    spec[0].0 = Curve3::Line {
        origin: pt(add(c.apex, normal, d)),
        dir: vt(g),
    };
    spec
}

/// The section of the cone by the plane through its axis at the
/// [`slant`],
/// tilted toward `e2` by a fraction of the most it may tilt and still
/// cut an ellipse, in closed form: the major vertices are the plane's
/// points on the generators at azimuth `±π/2`, and the minor semi-axis
/// is the half-chord along `e1` of the cone's circle at the centre's
/// height.
fn section(c: &Cone) -> Curve3<f64> {
    let tilt = 0.5_f64.min(0.3 * (HALF_PI - c.alpha));
    let normal = add(add([0.0; 3], c.a, tilt.cos()), c.e2, tilt.sin());
    let h0 = slant(c) * c.alpha.cos();
    let vertex = |u: f64| {
        let g = c.g(u);
        add(c.apex, g, h0 * tilt.cos() / dot(normal, g))
    };
    let (p, q) = (vertex(HALF_PI), vertex(-HALF_PI));
    let center = add(add([0.0; 3], p, 0.5), q, 0.5);
    let w = add(center, c.apex, -1.0);
    let radius = dot(w, c.a) * c.alpha.tan();
    let off = dot(w, c.e2);
    let half = add(p, q, -1.0);
    let major = 0.5 * dot(half, half).sqrt();
    Curve3::Ellipse {
        center: pt(center),
        axis: vt(normal),
        major,
        minor: ((radius - off) * (radius + off)).sqrt(),
        u_ref: vt(unit(half)),
    }
}

/// The section's major semi-axis stretched until its farthest point is
/// `d` off the cone, as one closed edge.
fn ellipse_stretched(c: &Cone, d: f64) -> Spec {
    let Curve3::Ellipse {
        center,
        axis,
        major,
        minor,
        u_ref,
    } = section(c)
    else {
        unreachable!()
    };
    let stretched = |k: f64| Curve3::Ellipse {
        center,
        axis,
        major: major * (1.0 + k),
        minor,
        u_ref,
    };
    // The distance is linear in a small stretch: read its slope at one.
    let k0 = 1e-6;
    let k = k0 * d / max_elevation(c, &stretched(k0), 0.0, TAU);
    vec![(stretched(k), 0.0, TAU)]
}

const PERTURBED: [Perturbed; 6] = [
    Perturbed {
        name: "props_rim_fit",
        build: rim_too_wide,
    },
    Perturbed {
        name: "props_cone_circle_tilt",
        build: rim_tilted,
    },
    Perturbed {
        name: "props_cone_circle_off_axis",
        build: rim_off_axis,
    },
    Perturbed {
        name: "props_meridian_generator",
        build: generator_turned,
    },
    Perturbed {
        name: "props_meridian_apex",
        build: generator_off_apex,
    },
    Perturbed {
        name: "props_cone_ellipse_on_cone",
        build: ellipse_stretched,
    },
];

/// **Every incidence decide of the cone's vector-area lane meters the
/// edge's true distance from the cone.** Near a needle a rim tilted
/// against the axis moves its points along the cone far more than off
/// it, and near the flat a rim off the axis does; a generator turned
/// about the apex strays farthest at its far end, which its span does
/// not measure; an ellipse stretched off its section is decided at all.
/// Each perturbation is answered at a tenth of the zero band and
/// refused under its decide's name at twice the escalation band, by the
/// flux lane at both scalars and by the material sign.
#[test]
fn every_cone_incidence_decide_meters_the_true_distance() {
    let b = band();
    let mut bad = Vec::new();
    for c in Cone::all() {
        for m in &PERTURBED {
            for (d, refuses) in [(0.1 * b.zero(), false), (2.0 * b.escalate(), true)] {
                let spec = (m.build)(&c, d);
                let (e, t0, t1) = &spec[0];
                let (t0, t1) = (t0.min(*t1), t0.max(*t1));
                let reach = dot(c.apex, c.apex).sqrt() + 4.0 * slant(&c);
                let measured = max_elevation(&c, e, t0, t1);
                assert!(
                    (measured - d).abs() <= 0.05 * d + 64.0 * f64::EPSILON * reach,
                    "{} {c:?}: the fixture is {measured:e} off the cone, not {d:e}",
                    m.name
                );
                for (lane, got) in verdicts(&c, &spec) {
                    let named = got == Err(PropsError::OffSurface { what: m.name });
                    if refuses != named || (!refuses && got.is_err()) {
                        bad.push(format!(
                            "{} alpha {} apex {:?} axis {:?} d {d:e} {lane}: {got:?}",
                            m.name, c.alpha, c.apex, c.a
                        ));
                    }
                }
            }
        }
    }
    assert!(bad.is_empty(), "{} wrong:\n{}", bad.len(), bad.join("\n"));
}

/// **The generator decide meters the angle off the generator, not its
/// cosine.** At `α = 10⁻⁶` the cosine difference `|dir·axis| − cos α`
/// reads a generator `δ` off as `sin α·δ`, so the generator stated
/// `2·escalate` off at its far end reads as on the cone, and the loop is
/// refused only at the joint it opens (`props_loop_closed`).
#[test]
fn the_needle_generator_refuses_by_its_own_name() {
    let b = band();
    for c in Cone::all().into_iter().filter(|c| c.alpha == 1e-6) {
        let spec = generator_turned(&c, 2.0 * b.escalate());
        for (lane, got) in verdicts(&c, &spec) {
            assert_eq!(
                got,
                Err(PropsError::OffSurface {
                    what: "props_meridian_generator"
                }),
                "{c:?} {lane}"
            );
        }
    }
}

/// **An ellipse arc's span is decided within a period.** The band
/// between the section ellipse and a rim above it, cut by two
/// generators, with the ellipse arc stored over `[0, θ]` and over
/// `[0, τ + θ]`: the two arcs end at the same points, so the loop
/// closes either way, turns once round the axis either way, and meets
/// itself at no vertex, while the longer one's vector area counts the
/// whole section a second time. The span decide refuses it, by name,
/// as a loop premise, before the winding or the vector area reads it.
#[test]
fn an_ellipse_arc_past_a_period_refuses_by_name() {
    for c in Cone::all() {
        let e = section(&c);
        let at = |t: f64| {
            let p = e.eval(t);
            [p.x, p.y, p.z]
        };
        let top = 1.5
            * (0..64)
                .map(|i| dot(add(at(TAU * f64::from(i) / 64.0), c.apex, -1.0), c.a))
                .fold(0.0, f64::max)
            / c.alpha.cos();
        let theta = 1.0;
        // The azimuth the arc turns through over `[0, θ]`, unwrapped.
        let mut turn = 0.0;
        for i in 0..256 {
            let (u0, u1) = (
                c.azimuth(at(theta * f64::from(i) / 256.0)),
                c.azimuth(at(theta * f64::from(i + 1) / 256.0)),
            );
            turn += (u1 - u0 + 3.0 * PI).rem_euclid(TAU) - PI;
        }
        let (pa, pb) = (at(0.0), at(theta));
        let ua = c.azimuth(pa);
        let reach = |p: V| dot(add(p, c.apex, -1.0), add(p, c.apex, -1.0)).sqrt();
        let face = |span: f64| -> Spec {
            vec![
                (e.clone(), 0.0, span),
                (c.generator(unit(add(pb, c.apex, -1.0))), reach(pb), top),
                (c.rim_circle(top), ua + turn, ua),
                (c.generator(unit(add(pa, c.apex, -1.0))), top, reach(pa)),
            ]
        };
        for (lane, got) in verdicts(&c, &face(theta)) {
            assert_eq!(got, Ok(()), "{c:?} {lane}: the band within a period");
        }
        for (lane, got) in verdicts(&c, &face(TAU + theta)) {
            assert_eq!(
                got,
                Err(PropsError::ConeLoop {
                    what: "props_cone_ellipse_span_winding"
                }),
                "{c:?} {lane}"
            );
        }
    }
}
