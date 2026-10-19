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
            semi_u,
            semi_v,
            u_ref,
        } => Curve3::Ellipse {
            center: p(center),
            axis: v(axis),
            semi_u: T::from_f64(semi_u),
            semi_v: T::from_f64(semi_v),
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

/// The wall `u ∈ [0, 1]`, `v ∈ [l, 8l]` at `l` the [`slant`], its
/// generator at `u = 1` turned about the apex by `δ` with `8l·sin δ = d`
/// ([`turned_on`]): its far end is `d` off the cone, eight times its
/// near end's offset.
fn generator_turned(c: &Cone, d: f64) -> Spec {
    turned_on(c, d, slant(c), 8.0 * slant(c))
}

/// As [`generator_turned`] on the wall `v ∈ [7l, 8l]`, whose generator
/// reaches eight times its span from the apex.
fn generator_turned_far(c: &Cone, d: f64) -> Spec {
    turned_on(c, d, 7.0 * slant(c), 8.0 * slant(c))
}

/// The wall `u ∈ [0, 1]`, `v ∈ [near, far]`, its generator at `u = 1`
/// turned about the apex in its meridian plane, toward the axis, until
/// its far end is `d` off the cone, and stated first, so its own decides
/// are the first to read it.
fn turned_on(c: &Cone, d: f64, near: f64, far: f64) -> Spec {
    let (s, co) = c.alpha.sin_cos();
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
/// [`slant`], tilted toward `e2` by a fraction of the most it may tilt
/// and still cut an ellipse.
fn section(c: &Cone) -> Curve3<f64> {
    section_at(c, 0.5_f64.min(0.3 * (HALF_PI - c.alpha)), slant(c))
}

/// The section by the plane through the axis at slant `l`, tilted toward
/// `e2` by `tilt`, in closed form: the major vertices are the plane's
/// points on the generators at azimuth `±π/2` (the first at `t = 0`),
/// and the minor semi-axis is the half-chord along `e1` of the cone's
/// circle at the centre's height.
fn section_at(c: &Cone, tilt: f64, l: f64) -> Curve3<f64> {
    let normal = add(add([0.0; 3], c.a, tilt.cos()), c.e2, tilt.sin());
    let h0 = l * c.alpha.cos();
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
        semi_u: major,
        semi_v: ((radius - off) * (radius + off)).sqrt(),
        u_ref: vt(unit(half)),
    }
}

/// The section's major semi-axis stretched until its farthest point is
/// `d` off the cone, as one closed edge.
fn ellipse_stretched(c: &Cone, d: f64) -> Spec {
    let Curve3::Ellipse {
        center,
        axis,
        semi_u,
        semi_v,
        u_ref,
    } = section(c)
    else {
        unreachable!()
    };
    let stretched = |k: f64| Curve3::Ellipse {
        center,
        axis,
        semi_u: semi_u * (1.0 + k),
        semi_v,
        u_ref,
    };
    // The distance is linear in a small stretch: read its slope at one.
    let k0 = 1e-6;
    let k = k0 * d / max_elevation(c, &stretched(k0), 0.0, TAU);
    vec![(stretched(k), 0.0, TAU)]
}

const PERTURBED: [Perturbed; 7] = [
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
        name: "props_meridian_on_surface",
        build: generator_turned,
    },
    Perturbed {
        name: "props_meridian_on_surface",
        build: generator_turned_far,
    },
    Perturbed {
        name: "props_meridian_on_surface",
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

/// **A generator turned off the cone at the needle refuses as incidence
/// by its own name.** At `α = 10⁻⁶` a reading of the turn by the cosine
/// difference `|dir·axis| − cos α` sees a generator `δ` off as
/// `sin α·δ`, so the generator stated `2·escalate` off at its far end
/// would read as on the cone, and the loop be refused only at the joint
/// it opens (`props_loop_closed`).
#[test]
fn the_needle_generator_refuses_by_its_own_name() {
    let b = band();
    for c in Cone::all().into_iter().filter(|c| c.alpha == 1e-6) {
        let spec = generator_turned(&c, 2.0 * b.escalate());
        for (lane, got) in verdicts(&c, &spec) {
            assert_eq!(
                got,
                Err(PropsError::OffSurface {
                    what: "props_meridian_on_surface"
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

/// **An ellipse arc's span is metered at its smaller semi-axis**, as
/// certification meters it: a section 25 times longer than wide, closed
/// on itself over `τ` plus an overrun. An overrun of `0.6·zero` at the
/// minor semi-axis — the speed at `t = 0`, where the arc closes — is
/// within the bound, though at the major semi-axis it would be fifteen
/// times the band past it; one of `2·escalate` at the minor is past it.
#[test]
fn an_ellipse_span_is_metered_at_its_smaller_semi_axis() {
    let b = band();
    let squash: f64 = 25.0;
    for c in Cone::all() {
        let tilt = ((1.0 - 1.0 / (squash * squash)).sqrt() * c.alpha.cos()).asin();
        let (s, co) = c.alpha.sin_cos();
        let e = section_at(&c, tilt, (1e3 * b.escalate() / s.min(co)).max(0.05));
        let Curve3::Ellipse { semi_u, semi_v, .. } = e else {
            unreachable!()
        };
        assert!(
            (semi_u / semi_v - squash).abs() < 0.01 * squash,
            "{c:?}: the section is {semi_u} by {semi_v}"
        );
        let cap = |over: f64| -> Spec { vec![(e.clone(), 0.0, TAU + over / semi_v)] };
        for (lane, got) in verdicts(&c, &cap(0.6 * b.zero())) {
            assert_eq!(got, Ok(()), "{c:?} {lane}: an overrun within the bound");
        }
        for (lane, got) in verdicts(&c, &cap(2.0 * b.escalate())) {
            assert_eq!(
                got,
                Err(PropsError::ConeLoop {
                    what: "props_cone_ellipse_span_winding"
                }),
                "{c:?} {lane}: an overrun past the bound"
            );
        }
    }
}

/// **An ellipse arc off the cone only between sample points refuses.**
/// The worst ellipse near a section, to first order, whose elevation is
/// within `0.9·zero` at the five midpoints of five equal pieces of a
/// closed ring's span, on sections whose far vertex is a hundred and a
/// thousand times as high as the near one: off the cone by 14 to 584
/// times the band between those points, it is refused as off the cone,
/// where five samples of the span read it as on.
mod between_samples {
    use super::{TAU, V, add, cross, dot, unit};
    use crate::shared::point::{p3, v3};
    use crate::shared::tol::band;
    use crate::shared::topo;
    use geom::{Curve3, Surface};
    use geom_brep::props::{LoopEdge, PropsError, curved_face_loops};
    use geom_core::{Decide, Interval, Real};

    const ALPHA: f64 = 0.5;
    const PI: f64 = core::f64::consts::PI;

    /// An ellipse: centre, normal, semi-axes, major direction.
    #[derive(Clone, Copy, Debug)]
    struct El {
        c: V,
        n: V,
        a: f64,
        b: f64,
        u: V,
    }

    impl El {
        fn eval(&self, t: f64) -> V {
            add(
                add(self.c, self.u, self.a * t.cos()),
                cross(self.n, self.u),
                self.b * t.sin(),
            )
        }

        /// Moved by `d`: the centre by `d[0..3]`, the semi-axes by
        /// `d[3]`, `d[4]`, the frame turned by `d[5..8]`.
        fn moved(&self, d: &[f64; 8]) -> El {
            let w = [d[5], d[6], d[7]];
            let n = unit(add(self.n, cross(w, self.n), 1.0));
            let u0 = add(self.u, cross(w, self.u), 1.0);
            El {
                c: add(self.c, [d[0], d[1], d[2]], 1.0),
                n,
                a: self.a + d[3],
                b: self.b + d[4],
                u: unit(add(u0, n, -dot(u0, n))),
            }
        }

        fn curve<T: Real>(&self) -> Curve3<T> {
            Curve3::Ellipse {
                center: p3(self.c[0], self.c[1], self.c[2]),
                axis: v3(self.n[0], self.n[1], self.n[2]),
                semi_u: T::from_f64(self.a),
                semi_v: T::from_f64(self.b),
                u_ref: v3(self.u[0], self.u[1], self.u[2]),
            }
        }
    }

    /// The distance from the cone at the origin about `z`.
    fn elevation(p: V) -> f64 {
        elevation_at(ALPHA, p)
    }

    /// The distance from the cone of half-angle `alpha` at the origin
    /// about `z`.
    fn elevation_at(alpha: f64, p: V) -> f64 {
        let (s, c) = alpha.sin_cos();
        (p[0] * p[0] + p[1] * p[1]).sqrt() * c - p[2].abs() * s
    }

    /// The section by the plane through `(0, 0, 1)` tilted toward `y` by
    /// `tilt`: the near vertex at `t = 0`, the far at `t = π`.
    fn section(tilt: f64) -> El {
        section_at(ALPHA, tilt)
    }

    fn section_at(alpha: f64, tilt: f64) -> El {
        let (s, co) = alpha.sin_cos();
        let normal = [0.0, tilt.sin(), tilt.cos()];
        let vertex = |u: f64| {
            let g = [s * u.cos(), s * u.sin(), co];
            add([0.0; 3], g, tilt.cos() / dot(normal, g))
        };
        let (p, q) = (
            vertex(-core::f64::consts::FRAC_PI_2),
            vertex(core::f64::consts::FRAC_PI_2),
        );
        let center = add(add([0.0; 3], p, 0.5), q, 0.5);
        let radius = center[2] * alpha.tan();
        let half = add(p, q, -1.0);
        El {
            c: center,
            n: normal,
            a: 0.5 * dot(half, half).sqrt(),
            b: ((radius - center[1]) * (radius + center[1])).sqrt(),
            u: unit(half),
        }
    }

    /// Solves `m x = r` by elimination with partial pivoting.
    fn solve<const N: usize>(mut m: [[f64; N]; N], mut r: [f64; N]) -> [f64; N] {
        for i in 0..N {
            let p = (i..N)
                .max_by(|&x, &y| m[x][i].abs().total_cmp(&m[y][i].abs()))
                .unwrap();
            m.swap(i, p);
            r.swap(i, p);
            let (pivot, pr) = (m[i], r[i]);
            for (row, rk) in m.iter_mut().zip(r.iter_mut()).skip(i + 1) {
                let f = row[i] / pivot[i];
                for (x, p) in row.iter_mut().zip(pivot.iter()).skip(i) {
                    *x -= f * p;
                }
                *rk -= f * pr;
            }
        }
        let mut x = [0.0; N];
        for i in (0..N).rev() {
            let tail: f64 = (i + 1..N).map(|j| m[i][j] * x[j]).sum();
            x[i] = (r[i] - tail) / m[i][i];
        }
        x
    }

    /// The elevation's gradient at `t` in the eight moves.
    fn gradient(e: &El, t: f64) -> [f64; 8] {
        core::array::from_fn(|k| {
            let mut d = [0.0; 8];
            d[k] = 1e-6;
            let up = elevation(e.moved(&d).eval(t));
            d[k] = -1e-6;
            (up - elevation(e.moved(&d).eval(t))) / 2e-6
        })
    }

    /// The move that keeps the elevation at the five piece midpoints of
    /// `[t0, t1]` within `level` while making it largest between them,
    /// to first order.
    fn worst(e: &El, t0: f64, t1: f64, level: f64) -> El {
        let nodes: [f64; 5] = core::array::from_fn(|i| t0 + (t1 - t0) * (i as f64 + 0.5) / 5.0);
        let jn: [[f64; 8]; 5] = core::array::from_fn(|i| gradient(e, nodes[i]));
        let gram: [[f64; 5]; 5] = core::array::from_fn(|i| {
            core::array::from_fn(|j| (0..8).map(|k| jn[i][k] * jn[j][k]).sum())
        });
        let mut best = (0.0, [0.0; 5]);
        for i in 0..=2000 {
            let jt = gradient(e, t0 + (t1 - t0) * f64::from(i) / 2000.0);
            let rhs: [f64; 5] = core::array::from_fn(|i| (0..8).map(|k| jn[i][k] * jt[k]).sum());
            let c = solve(gram, rhs);
            let amp: f64 = c.iter().map(|x| x.abs()).sum();
            if amp > best.0 {
                best = (amp, c);
            }
        }
        let y = solve(gram, core::array::from_fn(|i| level * best.1[i].signum()));
        e.moved(&core::array::from_fn(|k| {
            (0..5).map(|i| jn[i][k] * y[i]).sum()
        }))
    }

    fn verdict<T: Decide>(e: &El, t0: f64, t1: f64) -> Result<(), PropsError> {
        verdict_at::<T>(ALPHA, e, t0, t1)
    }

    fn verdict_at<T: Decide>(alpha: f64, e: &El, t0: f64, t1: f64) -> Result<(), PropsError> {
        let edges: Vec<LoopEdge<T>> = vec![topo::edge(e.curve::<T>(), t0, t1, 0, 0)];
        let s: Surface<T> = Surface::Cone {
            apex: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            half_angle: T::from_f64(alpha),
            u_ref: v3(1.0, 0.0, 0.0),
        };
        curved_face_loops(&s, &[&edges], true, band()).map(|_| ())
    }

    /// The tilt whose far vertex is `ratio` times as high as the near.
    fn tilt_for(alpha: f64, ratio: f64) -> f64 {
        let (mut lo, mut hi) = (0.0, core::f64::consts::FRAC_PI_2 - alpha);
        for _ in 0..200 {
            let m = 0.5 * (lo + hi);
            if (m - alpha).cos() / (m + alpha).cos() > ratio {
                hi = m;
            } else {
                lo = m;
            }
        }
        0.5 * (lo + hi)
    }

    /// Whether `r` passed the ellipse's incidence: anything but a
    /// refusal or escalation naming the nappe or the on-cone bound.
    fn incidence_passed(r: &Result<(), PropsError>) -> bool {
        let named = format!("{r:?}");
        !(named.contains("props_cone_ellipse_on_cone")
            || named.contains("props_cone_ellipse_nappe"))
    }

    #[test]
    fn an_ellipse_off_the_cone_between_samples_refuses() {
        let b = band();
        for ratio in [100.0, 1000.0] {
            let e = section(tilt_for(ALPHA, ratio));
            for t0 in [0.0, PI, PI + 0.3, PI - 0.3, 0.5] {
                let t1 = t0 + TAU;
                let p = worst(&e, t0, t1, 0.9 * b.zero());
                let off = (0..=20000)
                    .map(|i| elevation(p.eval(t0 + TAU * f64::from(i) / 20000.0)).abs())
                    .fold(0.0, f64::max);
                assert!(
                    off > b.escalate(),
                    "ratio {ratio} from {t0}: the fixture is only {off:e} off the cone"
                );
                let want = Err(PropsError::OffSurface {
                    what: "props_cone_ellipse_on_cone",
                });
                assert_eq!(
                    verdict::<f64>(&p, t0, t1),
                    want,
                    "ratio {ratio} from {t0}, f64"
                );
                assert_eq!(
                    verdict::<Interval>(&p, t0, t1),
                    want,
                    "ratio {ratio} from {t0}, interval"
                );
            }
        }
    }

    /// Every term of the bound is needed. Each fixture is a genuine
    /// section, which answers, moved along one of eight directions until
    /// it lies just past `zero` off the cone (`1.002·zero` aimed, past
    /// `zero` as sampled), where incidence must not answer at Interval.
    /// Dropping the `k_y·Y` term, or taking `ρ` or `|h|` at the piece's
    /// midpoint rather than its least, lets Interval answer up to a few
    /// percent past `zero` on these arcs.
    #[test]
    fn no_ellipse_answering_incidence_lies_past_zero() {
        let b = band();
        let fine = |alpha: f64, e: &El, t0: f64, t1: f64| {
            let n = 40_000;
            (0..=n)
                .map(|i| {
                    elevation_at(alpha, e.eval(t0 + (t1 - t0) * f64::from(i) / f64::from(n))).abs()
                })
                .fold(0.0, f64::max)
        };
        let mut over = Vec::new();
        for alpha in [1e-3, ALPHA, core::f64::consts::FRAC_PI_2 - 1e-3] {
            for ratio in [1.5, 10.0, 100.0] {
                let e = section_at(alpha, tilt_for(alpha, ratio));
                // Short arcs, about the near vertex's flank and about
                // the far vertex: where the dropped terms' share of the
                // bound is largest (a whole ring is over-read enough to
                // absorb them).
                for (t0, t1) in [(0.4, 1.9), (PI - 0.3, PI + 0.3)] {
                    for k in 0..8 {
                        let dir: [f64; 8] = core::array::from_fn(|j| {
                            if j == k {
                                1.0
                            } else if j == (k + 3) % 8 {
                                0.37
                            } else {
                                0.0
                            }
                        });
                        let moved = |s: f64| e.moved(&dir.map(|x| x * s));
                        let passes = |s: f64| {
                            incidence_passed(&verdict_at::<Interval>(alpha, &moved(s), t0, t1))
                        };
                        let case =
                            format!("alpha {alpha} ratio {ratio} [{t0}, {t1}] direction {k}");
                        assert!(passes(0.0), "{case}: the genuine section does not answer");
                        // The scale per metre of true distance, corrected
                        // at the target: at a fine ε the section's own
                        // rounding is a visible share of `zero`.
                        let want = 1.002 * b.zero();
                        let mut s = want * 1e-6 / fine(alpha, &moved(1e-6), t0, t1);
                        for _ in 0..4 {
                            s *= want / fine(alpha, &moved(s), t0, t1);
                        }
                        let off = fine(alpha, &moved(s), t0, t1) / b.zero();
                        assert!(off > 1.0, "{case}: the fixture is only {off} zero off");
                        if passes(s) {
                            over.push(format!("{case}: answers {off:.6} zero off"));
                        }
                    }
                }
            }
        }
        assert!(over.is_empty(), "{over:#?}");
    }

    /// An ellipse that reaches the apex's plane is on neither nappe, and
    /// is refused by the nappe decide's own name before the bound reads
    /// it.
    #[test]
    fn an_ellipse_through_the_apex_plane_refuses_as_off_its_nappe() {
        let want = Err(PropsError::OffSurface {
            what: "props_cone_ellipse_nappe",
        });
        for e in [
            El {
                c: [0.0, 0.0, 0.0],
                n: [0.0, 0.0, 1.0],
                a: 2.0,
                b: 1.0,
                u: [1.0, 0.0, 0.0],
            },
            El {
                c: [0.0, 0.0, 0.3],
                n: unit([0.0, 1.0, 0.2]),
                a: 2.0,
                b: 1.0,
                u: unit([0.0, -0.2, 1.0]),
            },
        ] {
            assert_eq!(verdict::<f64>(&e, 0.0, TAU), want, "{e:?}, f64");
            assert_eq!(verdict::<Interval>(&e, 0.0, TAU), want, "{e:?}, interval");
        }
    }
}
