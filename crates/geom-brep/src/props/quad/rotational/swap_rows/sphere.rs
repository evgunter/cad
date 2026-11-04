//! **The sphere's carrier-swap rows**: a sphere face bounded by rims,
//! meridians and tilted circles, with its circles rewritten as exact
//! rational quadratics so that each mints a projected image. Every
//! truth is the face's closed form, its area off the cap, zone and lune
//! areas and its vector area `A⃗ = ½∮(P − c)×P′` by composite Simpson
//! over each circle's own parametrization, in `f64`, independently of
//! the lane: the flux about a point is `s_f·r·|Ω| + (c − about)·A⃗`.
//!
//! The rows cover a face holding one pole, both, a pole on its loop, a
//! loop that winds through three chords, rings, both senses, and a
//! sphere `1e3` out.

use super::*;
use crate::pcurve_cache::{AxisMap, ChartMap};
use core::f64::consts::{FRAC_PI_2, PI, TAU};

/// Which name of its chart points a fixture's images carry.
#[derive(Clone, Copy)]
enum Name {
    /// As minted.
    Own,
    /// The sphere's twin `(u + π, π − v)`, harmonic edges and images
    /// alike.
    Twin,
    /// The images' levels shifted by a constant that names no latitude.
    Shifted(f64),
}

/// A sphere `c + r·(cos v·(cos u·û + sin u·ŵ) + sin v·â)`, `ŵ = â × û`.
#[derive(Clone, Copy)]
struct Ball {
    c: Point3<f64>,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
    r: f64,
}

impl Ball {
    fn tilted(c: Point3<f64>, r: f64) -> Self {
        let axis = Vec3::new(0.2, -0.3, 1.0);
        let axis = axis / axis.norm();
        let u_ref = axis.cross(Vec3::new(1.0, 0.0, 0.0));
        Self {
            c,
            axis,
            u_ref: u_ref / u_ref.norm(),
            r,
        }
    }

    fn w(self) -> Vec3<f64> {
        self.axis.cross(self.u_ref)
    }

    fn radial(self, u: f64) -> Vec3<f64> {
        self.u_ref * u.cos() + self.w() * u.sin()
    }

    /// The unit direction of the chart point `(u, v)` from the centre.
    fn dir(self, u: f64, v: f64) -> Vec3<f64> {
        self.radial(u) * v.cos() + self.axis * v.sin()
    }

    fn surface(self) -> Surface<f64> {
        Surface::Sphere {
            center: self.c,
            radius: self.r,
            axis: self.axis,
            u_ref: self.u_ref,
        }
    }

    fn sphere_area(self) -> f64 {
        2.0 * TAU * self.r * self.r
    }

    /// The area of the cap of angular radius `theta`.
    fn cap(self, theta: f64) -> f64 {
        TAU * self.r * self.r * (1.0 - theta.cos())
    }
}

/// A circle `q + ρ·(cos φ·e₁ + sin φ·e₂)` over `[φ₀, φ₁]`, traversed
/// forward or back.
#[derive(Clone, Copy)]
struct Arc {
    q: Point3<f64>,
    e1: Vec3<f64>,
    e2: Vec3<f64>,
    rho: f64,
    phi: (f64, f64),
    forward: bool,
}

impl Arc {
    fn at(self, phi: f64) -> Point3<f64> {
        self.q + (self.e1 * phi.cos() + self.e2 * phi.sin()) * self.rho
    }

    /// The rim `v = level` from `u = a` to `u = b` (its parameter is `u`).
    fn rim(ball: Ball, level: f64, (a, b): (f64, f64), forward: bool) -> Self {
        Self {
            q: ball.c + ball.axis * (ball.r * level.sin()),
            e1: ball.u_ref,
            e2: ball.w(),
            rho: ball.r * level.cos(),
            phi: (a, b),
            forward,
        }
    }

    /// The meridian `u = at` from `v = a` to `v = b` (its parameter is
    /// `v`).
    fn meridian(ball: Ball, at: f64, (a, b): (f64, f64), forward: bool) -> Self {
        Self {
            q: ball.c,
            e1: ball.radial(at),
            e2: ball.axis,
            rho: ball.r,
            phi: (a, b),
            forward,
        }
    }

    /// The whole circle at angular radius `theta` about the direction
    /// `(u, v)`, wound about it as `forward` says: forward, its inside is
    /// on its left under the outward normal.
    fn tilted(ball: Ball, (u, v): (f64, f64), theta: f64, forward: bool) -> Self {
        let d = ball.dir(u, v);
        let e1 = d.cross(ball.axis);
        let e1 = e1 / e1.norm();
        Self {
            q: ball.c + d * (ball.r * theta.cos()),
            e1,
            e2: d.cross(e1),
            rho: ball.r * theta.sin(),
            phi: (0.0, TAU),
            forward,
        }
    }

    /// The circle as an exact rational quadratic on `[0, 1]`, in its own
    /// parameter's direction, one segment per third of a turn or less.
    fn net(self) -> NurbsCurve3<f64> {
        let (a, b) = self.phi;
        let segments = ((b - a).abs() / (TAU / 3.0)).ceil().max(1.0);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let n = segments as usize;
        let half = (b - a) / (2.0 * segments);
        let mut control = vec![self.at(a)];
        let mut weights = vec![1.0];
        let mut knots = vec![0.0; 3];
        for k in 0..n {
            #[allow(clippy::cast_precision_loss)]
            let s = a + (b - a) * (k as f64) / segments;
            let m = s + half;
            control
                .push(self.q + (self.e1 * m.cos() + self.e2 * m.sin()) * (self.rho / half.cos()));
            weights.push(half.cos());
            control.push(self.at(s + 2.0 * half));
            weights.push(1.0);
            if k + 1 < n {
                #[allow(clippy::cast_precision_loss)]
                knots.extend([(k + 1) as f64 / segments; 2]);
            }
        }
        knots.extend([1.0; 3]);
        net(knots, 2, control, weights)
    }

    /// `½∫(P − c)×P′` over the arc, traversal folded in, by composite
    /// Simpson on its own parametrization.
    fn vector_area(self, c: Point3<f64>) -> Vec3<f64> {
        let integrand = |phi: f64| {
            let p = self.at(phi) - c;
            let dp = (self.e2 * phi.cos() - self.e1 * phi.sin()) * self.rho;
            p.cross(dp) * 0.5
        };
        let (a, b) = self.phi;
        let n = 4096_u32;
        let h = (b - a) / f64::from(n);
        let mut sum = integrand(a) + integrand(b);
        for i in 1..n {
            let w = if i % 2 == 1 { 4.0 } else { 2.0 };
            sum = sum + integrand(a + h * f64::from(i)) * w;
        }
        let va = sum * (h / 3.0);
        if self.forward { va } else { -va }
    }
}

/// One edge of a sphere fixture: its arc, its harmonic image where it
/// has one (a rim's or a meridian's), and whether it is swapped for its
/// exact net.
struct SphereEdge {
    arc: Arc,
    harmonic: Option<TrimEdgeQ>,
    swap: bool,
}

fn rim(ball: Ball, level: f64, us: (f64, f64), forward: bool, swap: bool) -> SphereEdge {
    SphereEdge {
        arc: Arc::rim(ball, level, us, forward),
        harmonic: Some(harmonic(chan(0.0, 1.0), chan(level, 0.0), us, forward)),
        swap,
    }
}

fn meridian(ball: Ball, at: f64, vs: (f64, f64), forward: bool) -> SphereEdge {
    SphereEdge {
        arc: Arc::meridian(ball, at, vs, forward),
        harmonic: Some(harmonic(chan(at, 0.0), chan(0.0, 1.0), vs, forward)),
        swap: false,
    }
}

fn tilted(ball: Ball, at: (f64, f64), theta: f64, forward: bool) -> SphereEdge {
    SphereEdge {
        arc: Arc::tilted(ball, at, theta, forward),
        harmonic: None,
        swap: true,
    }
}

/// The face of `loops` on `ball` under the sense `outward`, its chart
/// term about `about`, its chart points under `name`: each swapped edge
/// projected, its carrier's vector area read by the lane's rule, the
/// rest of `A⃗` the fixture's own.
fn sphere_face(
    ball: Ball,
    outward: bool,
    loops: &[Vec<SphereEdge>],
    about: Point3<f64>,
    name: Name,
) -> Result<RotationalFaceQ, PropsError> {
    let band = Band::linear(Tol::witness()).unwrap();
    let surface = ball.surface();
    let chart = RotationalChartQ::Sphere {
        radius: pt(ball.r),
        outward,
    };
    let lever = ball.c - about;
    let lever = [pt(lever.x), pt(lever.y), pt(lever.z)];
    let mut closed = Vec3::zero();
    let mut rationals = Vec::new();
    let mut ribbon = 0.0_f64;
    let mut terms = Vec::new();
    for edges in loops {
        let mut out = Vec::new();
        for e in edges {
            if !e.swap {
                closed = closed + e.arc.vector_area(ball.c);
                let h = e.harmonic.unwrap();
                out.push(EdgeTermQ::Harmonic(match name {
                    Name::Twin => TrimEdgeQ {
                        u: HarmChan {
                            c0: h.u.c0 + pt(PI),
                            ..h.u
                        },
                        v: HarmChan {
                            c0: pt(PI) - h.v.c0,
                            ca: -h.v.ca,
                            cb: -h.v.cb,
                            cl: -h.v.cl,
                        },
                        ..h
                    },
                    Name::Own | Name::Shifted(_) => h,
                }));
                continue;
            }
            let n = e.arc.net();
            let carrier = Curve3::Nurbs(std::sync::Arc::new(n.clone()));
            let image = chart_pcurve_over(&carrier, 0.0, 1.0, &surface, band).unwrap();
            let env = PcurveCache::certify_projected(
                image.projected().unwrap().clone(),
                0.0,
                1.0,
                &carrier,
                &surface,
                band,
                Some(crate::FittedLane::<f64>::certified()),
            )
            .unwrap()
            .certificate()
            .envelope;
            let image = match name {
                Name::Own => image,
                Name::Twin => image.map_affine(ChartMap::sphere_involution()).unwrap(),
                Name::Shifted(s) => image
                    .map_affine(ChartMap {
                        u: AxisMap::identity(),
                        v: AxisMap::shift(s),
                    })
                    .unwrap(),
            };
            let at = |t: f64| {
                let p = image.eval(t);
                (pt(p.x), pt(p.y))
            };
            out.push(EdgeTermQ::Projected(ProjectedEdgeQ::of_image(
                image.projected().unwrap(),
                chart,
                (pt(0.0), pt(1.0)),
                e.arc.forward,
                [at(0.0), at(1.0)],
                pt(env),
                pt,
            )?));
            let iv = Interval::from_certified;
            rationals.push(
                rational_edge_q(
                    &n.map_scalar(iv),
                    pt(0.0),
                    pt(1.0),
                    e.arc.forward,
                    Point3::new(iv(ball.c.x), iv(ball.c.y), iv(ball.c.z)),
                )
                .unwrap(),
            );
            ribbon += ribbon_of(&n, env);
        }
        terms.push(out);
    }
    let dot = |v: Vec3<f64>| lever[0] * pt(v.x) + lever[1] * pt(v.y) + lever[2] * pt(v.z);
    Ok(RotationalFaceQ {
        chart,
        chart_term: ChartTermQ {
            closed: dot(closed),
            lever,
            rationals,
            ribbon,
        },
        loops: terms,
    })
}

/// The lane's run over `loops`, its chart points under `name`.
fn sphere_run(
    ball: Ball,
    outward: bool,
    loops: &[Vec<SphereEdge>],
    about: Point3<f64>,
    name: Name,
) -> Result<FaceCutBounds, PropsError> {
    let band = Band::linear(Tol::witness()).unwrap();
    met(rotational_cut_face_rounds::<f64>(
        &sphere_face(ball, outward, loops, about, name)?,
        band,
        RoundWindow::SCHEDULE,
        &mut to_target(1e-9),
    ))
}

/// The face's closed forms about `about`: `s_f·r·|Ω| + (c − about)·A⃗`
/// and `|Ω|`, `A⃗` summed over every edge's arc.
fn sphere_truth(
    ball: Ball,
    outward: bool,
    loops: &[Vec<SphereEdge>],
    area: f64,
    about: Point3<f64>,
) -> (f64, f64) {
    let va = loops
        .iter()
        .flatten()
        .fold(Vec3::zero(), |va, e| va + e.arc.vector_area(ball.c));
    let s = if outward { 1.0 } else { -1.0 };
    (s * ball.r * area + (ball.c - about).dot(va), area)
}

/// The lane's enclosures of `loops` hold the face's closed forms, under
/// both names of its chart points.
fn sphere_holds(
    label: &str,
    ball: Ball,
    outward: bool,
    loops: &[Vec<SphereEdge>],
    area: f64,
    about: Point3<f64>,
) {
    let (flux, area) = sphere_truth(ball, outward, loops, area, about);
    for (which, name) in [("own", Name::Own), ("twin", Name::Twin)] {
        let got = sphere_run(ball, outward, loops, about, name)
            .unwrap_or_else(|e| panic!("{label} ({which}): the lane refused: {e:?}"));
        encloses(got.flux, flux, &format!("{label} ({which}) flux"));
        encloses(got.area, area, &format!("{label} ({which}) area"));
    }
}

/// The cap north of `level`, wound about the pole, its rim harmonic or
/// swapped.
fn north_cap(ball: Ball, level: f64, forward: bool, swap: bool) -> Vec<Vec<SphereEdge>> {
    vec![vec![rim(ball, level, (0.0, TAU), forward, swap)]]
}

/// **A face holding one pole, and its complement holding the other, in
/// both senses**: the cap north of a rim, harmonic and swapped. The loop
/// wound about the north pole bounds the cap on its left under the
/// outward normal and the southern rest under the inward one; reversed,
/// the inward normal's left is the cap again. Each holds its closed
/// form, the pole's term read off the winding with the sense.
#[test]
fn a_cap_holds_its_pole_in_either_sense() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    let level = 0.4_f64;
    let cap = ball.cap(FRAC_PI_2 - level);
    for swap in [false, true] {
        for (label, forward, outward, area) in [
            ("the north cap", true, true, cap),
            (
                "the south rest, inward",
                true,
                false,
                ball.sphere_area() - cap,
            ),
            ("the north cap, inward", false, false, cap),
            ("the south rest", false, true, ball.sphere_area() - cap),
        ] {
            sphere_holds(
                &format!("{label} (swapped {swap})"),
                ball,
                outward,
                &north_cap(ball, level, forward, swap),
                area,
                about,
            );
        }
    }
}

/// **A face holding both poles**: the sphere less a tilted disc, the
/// disc's circle swapped (it has no harmonic image), in both senses, and
/// the disc itself, holding neither.
#[test]
fn the_sphere_less_a_tilted_disc_holds_both_poles() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    let theta = 0.5;
    let disc = ball.cap(theta);
    for (label, forward, outward, area) in [
        (
            "the sphere less the disc",
            false,
            true,
            ball.sphere_area() - disc,
        ),
        ("the disc", true, true, disc),
        ("the disc, inward", false, false, disc),
        (
            "the sphere less the disc, inward",
            true,
            false,
            ball.sphere_area() - disc,
        ),
    ] {
        sphere_holds(
            label,
            ball,
            outward,
            &[vec![tilted(ball, (1.0, 0.2), theta, forward)]],
            area,
            about,
        );
    }
}

/// **Rings**: a zone between two rims, either swapped (its loops wind
/// the axis once each way, and it holds neither pole), and the sphere
/// less the north cap and a tilted disc (a pole held, the face's two
/// holes read against it).
#[test]
fn rings_on_a_sphere_hold_their_closed_forms() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    let (low, high) = (-0.3_f64, 0.5_f64);
    let zone = TAU * ball.r * ball.r * (high.sin() - low.sin());
    for (label, swap) in [("bottom", [true, false]), ("top", [false, true])] {
        sphere_holds(
            &format!("a zone, its {label} rim swapped"),
            ball,
            true,
            &[
                vec![rim(ball, low, (0.0, TAU), true, swap[0])],
                vec![rim(ball, high, (0.0, TAU), false, swap[1])],
            ],
            zone,
            about,
        );
    }
    let theta = 0.3;
    let area = ball.sphere_area() - ball.cap(FRAC_PI_2 - high) - ball.cap(theta);
    sphere_holds(
        "the sphere less the north cap and a disc",
        ball,
        true,
        &[
            vec![rim(ball, high, (0.0, TAU), false, true)],
            vec![tilted(ball, (2.0, -0.4), theta, false)],
        ],
        area,
        about,
    );
}

/// **A chord loop that winds**: the north cap's rim as three swapped
/// arcs of a third of a turn, beside the cap with one harmonic third.
#[test]
fn a_cap_bounded_by_three_chords_winds_once() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    let level = 0.4_f64;
    let cap = ball.cap(FRAC_PI_2 - level);
    let third = |k: f64| (k * TAU / 3.0, (k + 1.0) * TAU / 3.0);
    for (label, harmonic_third) in [("three chords", None), ("two chords", Some(1))] {
        let loop_: Vec<SphereEdge> = (0..3)
            .map(|k| {
                rim(
                    ball,
                    level,
                    third(f64::from(k)),
                    true,
                    harmonic_third != Some(k),
                )
            })
            .collect();
        sphere_holds(label, ball, true, &[loop_], cap, about);
    }
}

/// **A pole on the loop**: the cap north of a rim between two meridians
/// meeting at the pole, its rim swapped, where the junction at the pole
/// is the chart's pole edge; and a lune from pole to pole holding a
/// tilted disc as a ring.
#[test]
fn a_pole_on_the_loop_reads_its_corner() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    let (level, a) = (0.4_f64, 1.3_f64);
    sphere_holds(
        "a cap's sector",
        ball,
        true,
        &[vec![
            rim(ball, level, (0.0, a), true, true),
            meridian(ball, a, (level, FRAC_PI_2), true),
            meridian(ball, 0.0, (level, FRAC_PI_2), false),
        ]],
        a * ball.r * ball.r * (1.0 - level.sin()),
        about,
    );
    let theta = 0.2;
    sphere_holds(
        "a lune holding a disc",
        ball,
        true,
        &[
            vec![
                meridian(ball, PI / 2.0, (-FRAC_PI_2, FRAC_PI_2), true),
                meridian(ball, -PI / 2.0, (-FRAC_PI_2, FRAC_PI_2), false),
            ],
            vec![tilted(ball, (0.0, 0.1), theta, false)],
        ],
        2.0 * PI * ball.r * ball.r - ball.cap(theta),
        about,
    );
}

/// **A sphere `1e3` out**, its flux about the world origin: a cap and
/// the sphere less a disc.
#[test]
fn a_sphere_far_out_holds_its_closed_forms() {
    let ball = Ball::tilted(Point3::new(1e3, -1e3, 5e2), 0.7);
    let level = 0.4_f64;
    sphere_holds(
        "the north cap, far out",
        ball,
        true,
        &north_cap(ball, level, true, true),
        ball.cap(FRAC_PI_2 - level),
        Point3::origin(),
    );
    sphere_holds(
        "the sphere less a disc, far out",
        ball,
        true,
        &[vec![tilted(ball, (1.0, 0.2), 0.5, false)]],
        ball.sphere_area() - ball.cap(0.5),
        Point3::origin(),
    );
}

/// **A circle passing close by a pole**: the sphere less a disc whose
/// circle passes `0.02` rad from the north pole, where its azimuth turns
/// fast, and the disc itself.
#[test]
fn a_disc_beside_the_pole_holds_its_closed_forms() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    let theta = 0.4;
    let at = (0.7, FRAC_PI_2 - theta - 0.02);
    sphere_holds(
        "the sphere less a disc beside the pole",
        ball,
        true,
        &[vec![tilted(ball, at, theta, false)]],
        ball.sphere_area() - ball.cap(theta),
        about,
    );
    sphere_holds(
        "a disc beside the pole",
        ball,
        true,
        &[vec![tilted(ball, at, theta, true)]],
        ball.cap(theta),
        about,
    );
}

/// **What the sphere's arm refuses**: an image whose deck level names no
/// latitude, a harmonic image that is neither a rim nor a meridian, and
/// a ring wound with its face (a zone whose top rim winds as its bottom
/// does).
#[test]
fn the_spheres_arm_refuses_what_it_cannot_read() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    let refused = |got: Result<FaceCutBounds, PropsError>| match got {
        Err(PropsError::QuadratureUnsupported { what }) => what,
        other => panic!("expected a typed refusal, got {other:?}"),
    };
    let what = refused(sphere_run(
        ball,
        true,
        &north_cap(ball, 0.4, true, true),
        about,
        Name::Shifted(0.3),
    ));
    assert!(what.contains("not a name of the latitude"), "{what}");
    let slanted = SphereEdge {
        arc: Arc::rim(ball, 0.4, (0.0, TAU), true),
        harmonic: Some(harmonic(chan(0.0, 1.0), chan(0.4, 0.01), (0.0, TAU), true)),
        swap: false,
    };
    let what = refused(sphere_run(ball, true, &[vec![slanted]], about, Name::Own));
    assert!(what.contains("neither a rim nor a meridian"), "{what}");
    let got = sphere_run(
        ball,
        true,
        &[
            vec![rim(ball, -0.3, (0.0, TAU), true, true)],
            vec![rim(ball, 0.5, (0.0, TAU), true, false)],
        ],
        about,
        Name::Own,
    );
    assert!(
        matches!(got, Err(PropsError::RingWoundWithFace)),
        "a ring wound with its face: {got:?}"
    );
}
