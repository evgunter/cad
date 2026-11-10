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
    /// The images' levels shifted by a constant `by` that names no
    /// latitude, on the own name or the twin.
    Shifted {
        /// Whether the images are on the twin first.
        twin: bool,
        /// The shift (rad).
        by: f64,
    },
}

impl Name {
    fn twin(self) -> bool {
        matches!(self, Self::Twin | Self::Shifted { twin: true, .. })
    }
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

    /// The arc's length.
    fn length(self) -> f64 {
        self.rho * (self.phi.1 - self.phi.0).abs()
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
    /// An envelope the fixture states for an image standing off the true
    /// boundary, added to the certified one (`None`: the image is the
    /// boundary's own).
    env: Option<f64>,
}

fn rim(ball: Ball, level: f64, us: (f64, f64), forward: bool, swap: bool) -> SphereEdge {
    SphereEdge {
        arc: Arc::rim(ball, level, us, forward),
        harmonic: Some(harmonic(chan(0.0, 1.0), chan(level, 0.0), us, forward)),
        swap,
        env: None,
    }
}

fn meridian(ball: Ball, at: f64, vs: (f64, f64), forward: bool) -> SphereEdge {
    SphereEdge {
        arc: Arc::meridian(ball, at, vs, forward),
        harmonic: Some(harmonic(chan(at, 0.0), chan(0.0, 1.0), vs, forward)),
        swap: false,
        env: None,
    }
}

fn tilted(ball: Ball, at: (f64, f64), theta: f64, forward: bool) -> SphereEdge {
    SphereEdge {
        arc: Arc::tilted(ball, at, theta, forward),
        harmonic: None,
        swap: true,
        env: None,
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
                let mut h = e.harmonic.unwrap();
                if let Some(env) = e.env {
                    h.env = pt(env);
                    ribbon += env * e.arc.length();
                }
                out.push(EdgeTermQ::Harmonic(match name {
                    _ if name.twin() => TrimEdgeQ {
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
                    _ => h,
                }));
                continue;
            }
            let n = e.arc.net();
            let carrier = Curve3::Nurbs(std::sync::Arc::new(n.clone()));
            // A circle through a pole mints no image (the chart's mint
            // refuses there), which a row reads as a refusal.
            let image = chart_pcurve_over(&carrier, 0.0, 1.0, &surface, band).map_err(|_| {
                PropsError::QuadratureUnsupported {
                    what: "the fixture's carrier mints no image on the sphere",
                }
            })?;
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
            .envelope
                + e.env.unwrap_or(0.0);
            let image = if name.twin() {
                image.map_affine(ChartMap::sphere_involution()).unwrap()
            } else {
                image
            };
            let image = match name {
                Name::Shifted { by, .. } => image
                    .map_affine(ChartMap {
                        u: AxisMap::identity(),
                        v: AxisMap::shift(by),
                    })
                    .unwrap(),
                Name::Own | Name::Twin => image,
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
            spirics: Vec::new(),
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

/// The lane's enclosures of `images`, the edges as read, hold the closed
/// forms of the face `truth` bounds, under both names: where an image
/// stands off the true boundary within its stated envelope, only the
/// pads pay for the strip between, and the lane stops at the floor they
/// set rather than at the target.
fn standing_off_holds(
    label: &str,
    ball: Ball,
    outward: bool,
    images: &[Vec<SphereEdge>],
    truth: &[Vec<SphereEdge>],
    area: f64,
    about: Point3<f64>,
) {
    let (flux, area) = sphere_truth(ball, outward, truth, area, about);
    let band = Band::linear(Tol::witness()).unwrap();
    for (which, name) in [("own", Name::Own), ("twin", Name::Twin)] {
        let got = sphere_face(ball, outward, images, about, name)
            .and_then(|face| {
                rotational_cut_face_rounds::<f64>(
                    &face,
                    band,
                    RoundWindow::SCHEDULE,
                    &mut to_target(1e-9),
                )
            })
            .unwrap_or_else(|e| panic!("{label} ({which}): the lane refused: {e:?}"))
            .bounds;
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
    for twin in [false, true] {
        let name = if twin { Name::Twin } else { Name::Own };
        // A level offset a deck's composition could not round to, near the
        // tolerance and far past it (`latitude_name`).
        for by in [0.3, 1e-3, 1e-9] {
            let what = refused(sphere_run(
                ball,
                true,
                &north_cap(ball, 0.4, true, true),
                about,
                Name::Shifted { twin, by },
            ));
            assert!(
                what.contains("not a name of the latitude"),
                "a level shifted by {by} (twin {twin}): {what}"
            );
        }
        let slanted = SphereEdge {
            arc: Arc::rim(ball, 0.4, (0.0, TAU), true),
            harmonic: Some(harmonic(chan(0.0, 1.0), chan(0.4, 0.01), (0.0, TAU), true)),
            swap: false,
            env: None,
        };
        let what = refused(sphere_run(ball, true, &[vec![slanted]], about, name));
        assert!(
            what.contains("neither a rim nor a meridian"),
            "twin {twin}: {what}"
        );
        let got = sphere_run(
            ball,
            true,
            &[
                vec![rim(ball, -0.3, (0.0, TAU), true, true)],
                vec![rim(ball, 0.5, (0.0, TAU), true, false)],
            ],
            about,
            name,
        );
        assert!(
            matches!(got, Err(PropsError::RingWoundWithFace)),
            "a ring wound with its face (twin {twin}): {got:?}"
        );
    }
}

/// **An open junction refuses under both names**: a cap sector whose rim
/// stops `0.3` rad short of its second meridian, about `0.19` m at its
/// latitude. Under the twin the latitude's cosine is negative, so the
/// azimuth lever's magnitude is what keeps the gap from reading closed.
#[test]
fn an_open_junction_refuses_under_both_names() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    let level = 0.4_f64;
    let loops = [vec![
        rim(ball, level, (0.0, 2.0), true, false),
        meridian(ball, 2.3, (level, FRAC_PI_2), true),
        meridian(ball, 0.0, (level, FRAC_PI_2), false),
    ]];
    for (which, name) in [("own", Name::Own), ("twin", Name::Twin)] {
        let got = sphere_run(ball, true, &loops, about, name);
        assert!(
            matches!(got, Err(PropsError::ChartLoopOpen)),
            "{which}: an open loop: {got:?}"
        );
    }
}

/// **Meridians standing off the true lune are paid for by the map
/// residual, at the chart's level lever**: a lune of `1` rad on a sphere
/// of radius `2` read through meridian images `α` outside it, each within
/// its stated envelope `r·α` of the true one. The images' lune is `4αr²`
/// larger; the residual pays `2·πr·rα`, at the meridians' metric length
/// `πr`, which the level lever meters (read at `1` it pays `2π·rα`, short
/// of the excess).
#[test]
fn a_lune_whose_meridians_stand_off_holds_its_closed_form() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 2.0);
    let about = Point3::new(0.1, 0.5, -0.2);
    let (a, alpha) = (1.0, 1e-3);
    let lune = |lo: f64, hi: f64, env: Option<f64>| {
        vec![vec![
            SphereEdge {
                env,
                ..meridian(ball, hi, (-FRAC_PI_2, FRAC_PI_2), true)
            },
            SphereEdge {
                env,
                ..meridian(ball, lo, (-FRAC_PI_2, FRAC_PI_2), false)
            },
        ]]
    };
    standing_off_holds(
        "a lune read through meridians standing off it",
        ball,
        true,
        &lune(-alpha, a + alpha, Some(ball.r * alpha * (1.0 + 1e-9))),
        &lune(0.0, a, None),
        2.0 * a * ball.r * ball.r,
        about,
    );
}

/// **A carrier standing off its circle, far out, is paid for by the map
/// residual and the chart term's pad**: the sphere of radius `1.3`,
/// `1e3` out, less a disc of angular radius `θ`, read through the circle
/// at `θ + δ`, an exact rational quadratic on the sphere whose image is
/// itself, within its stated envelope `r·δ` of the true circle. Its face
/// is smaller by about `2πr²·sin θ·δ`, which the residual pays at the
/// image's metric length (the projected speed bound's), and its vector
/// area off by the ribbon, which the chart term's pad pays at the lever
/// to the world origin.
#[test]
fn a_circle_standing_off_its_carrier_far_out_holds_its_closed_form() {
    let ball = Ball::tilted(Point3::new(1e3, -1e3, 5e2), 1.3);
    let (theta, delta) = (0.5_f64, 1e-5);
    let image = SphereEdge {
        env: Some(ball.r * delta * (1.0 + 1e-9)),
        ..tilted(ball, (1.0, 0.2), theta + delta, false)
    };
    standing_off_holds(
        "the sphere less a disc, its circle standing off",
        ball,
        true,
        &[vec![image]],
        &[vec![tilted(ball, (1.0, 0.2), theta, false)]],
        ball.sphere_area() - ball.cap(theta),
        Point3::origin(),
    );
}

/// Holds its closed forms under both names, or refuses typed, never a
/// wrong enclosure; whether it held under both.
fn holds_or_refuses(
    label: &str,
    ball: Ball,
    outward: bool,
    loops: &[Vec<SphereEdge>],
    area: f64,
    about: Point3<f64>,
) -> bool {
    let (flux, area) = sphere_truth(ball, outward, loops, area, about);
    let band = Band::linear(Tol::witness()).unwrap();
    let mut held = true;
    for (which, name) in [("own", Name::Own), ("twin", Name::Twin)] {
        let run = sphere_face(ball, outward, loops, about, name).and_then(|f| {
            rotational_cut_face_rounds::<f64>(&f, band, RoundWindow::SCHEDULE, &mut to_target(1e-9))
        });
        match run {
            Ok(got) => {
                encloses(got.bounds.flux, flux, &format!("{label} ({which}) flux"));
                encloses(got.bounds.area, area, &format!("{label} ({which}) area"));
            }
            Err(e) => {
                println!("{label} ({which}): refused {e:?}");
                held = false;
            }
        }
    }
    held
}

/// **A hemisphere exactly**, the rim at the equator where `sin v = 0`, in
/// every sense and traversal, harmonic and swapped; and a tilted one.
#[test]
fn a_hemisphere_exactly_holds_its_closed_forms() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    let half = 0.5 * ball.sphere_area();
    for swap in [false, true] {
        for (forward, outward) in [(true, true), (true, false), (false, false), (false, true)] {
            assert!(holds_or_refuses(
                &format!("hemisphere forward {forward} outward {outward} swapped {swap}"),
                ball,
                outward,
                &north_cap(ball, 0.0, forward, swap),
                half,
                about,
            ));
        }
    }
    assert!(holds_or_refuses(
        "a tilted hemisphere",
        ball,
        true,
        &[vec![tilted(ball, (1.0, 0.2), FRAC_PI_2, true)]],
        half,
        about,
    ));
}

/// **A cap sector and a lune wider than a half turn at the pole**, where
/// the closure takes a whole turn off the pole junction, and exactly a
/// half turn; each with its complement under the inward sense.
#[test]
fn pole_angles_past_a_half_turn_hold_their_closed_forms() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    let level = 0.4_f64;
    for a in [2.0, PI, 3.5, 4.5, 6.0, TAU - 1e-3] {
        for swap in [true, false] {
            let loops = [vec![
                rim(ball, level, (0.0, a), true, swap),
                meridian(ball, a, (level, FRAC_PI_2), true),
                meridian(ball, 0.0, (level, FRAC_PI_2), false),
            ]];
            let area = a * ball.r * ball.r * (1.0 - level.sin());
            assert!(holds_or_refuses(
                &format!("sector {a} swapped {swap}"),
                ball,
                true,
                &loops,
                area,
                about
            ));
            assert!(holds_or_refuses(
                &format!("sector {a} swapped {swap}, its complement inward"),
                ball,
                false,
                &loops,
                ball.sphere_area() - area,
                about,
            ));
        }
        let lune = [vec![
            meridian(ball, a, (-FRAC_PI_2, FRAC_PI_2), true),
            meridian(ball, 0.0, (-FRAC_PI_2, FRAC_PI_2), false),
        ]];
        let area = 2.0 * a * ball.r * ball.r;
        assert!(holds_or_refuses(
            &format!("lune {a}"),
            ball,
            true,
            &lune,
            area,
            about
        ));
        assert!(holds_or_refuses(
            &format!("lune {a}, its complement inward"),
            ball,
            false,
            &lune,
            ball.sphere_area() - area,
            about,
        ));
    }
}

/// **Near nothing and near the whole sphere**: tiny discs, the sphere
/// less each, a tiny polar cap's complement, and a disc of radius near
/// `π`. Each holds or refuses typed, never a wrong enclosure.
#[test]
fn tiny_and_near_full_faces_hold_or_refuse() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    for theta in [1e-2, 1e-3, 1e-4] {
        let disc = ball.cap(theta);
        let full = ball.sphere_area();
        holds_or_refuses(
            &format!("disc {theta}"),
            ball,
            true,
            &[vec![tilted(ball, (1.0, 0.2), theta, true)]],
            disc,
            about,
        );
        holds_or_refuses(
            &format!("sphere less disc {theta}"),
            ball,
            true,
            &[vec![tilted(ball, (1.0, 0.2), theta, false)]],
            full - disc,
            about,
        );
        holds_or_refuses(
            &format!("sphere less disc {theta}, inward"),
            ball,
            false,
            &[vec![tilted(ball, (1.0, 0.2), theta, true)]],
            full - disc,
            about,
        );
        holds_or_refuses(
            &format!("sphere less a polar cap {theta}"),
            ball,
            true,
            &north_cap(ball, FRAC_PI_2 - theta, false, true),
            full - disc,
            about,
        );
    }
    holds_or_refuses(
        "a disc of radius π − 1e-3",
        ball,
        true,
        &[vec![tilted(ball, (1.0, 0.2), PI - 1e-3, true)]],
        ball.cap(PI - 1e-3),
        about,
    );
}

/// **A circle grazing a pole ever closer, and through it**: holds or
/// refuses typed, never guesses.
#[test]
fn a_circle_grazing_the_pole_holds_or_refuses() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    let theta = 0.4;
    for miss in [1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-9, 0.0, -1e-6, -1e-5, -1e-3] {
        let at = (0.7, FRAC_PI_2 - theta - miss);
        holds_or_refuses(
            &format!("disc missing the pole by {miss}"),
            ball,
            true,
            &[vec![tilted(ball, at, theta, true)]],
            ball.cap(theta),
            about,
        );
        holds_or_refuses(
            &format!("sphere less that disc ({miss})"),
            ball,
            true,
            &[vec![tilted(ball, at, theta, false)]],
            ball.sphere_area() - ball.cap(theta),
            about,
        );
    }
}

/// **Two holes**: the sphere less two discs, and a zone less a disc.
#[test]
fn two_holes_hold_their_closed_forms() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let about = Point3::new(0.1, 0.5, -0.2);
    let (t1, t2) = (0.3, 0.25);
    assert!(holds_or_refuses(
        "sphere less two discs",
        ball,
        true,
        &[
            vec![tilted(ball, (1.0, 0.2), t1, false)],
            vec![tilted(ball, (4.0, -0.5), t2, false)]
        ],
        ball.sphere_area() - ball.cap(t1) - ball.cap(t2),
        about,
    ));
    let (low, high) = (-0.6_f64, 0.7_f64);
    let zone = TAU * ball.r * ball.r * (high.sin() - low.sin());
    assert!(holds_or_refuses(
        "zone less a disc",
        ball,
        true,
        &[
            vec![rim(ball, low, (0.0, TAU), true, true)],
            vec![rim(ball, high, (0.0, TAU), false, true)],
            vec![tilted(ball, (2.0, 0.0), 0.2, false)],
        ],
        zone - ball.cap(0.2),
        about,
    ));
}

/// **A flipped sense bit certifies the complement**: the north cap's
/// loop under the inward sense is the southern rest, a valid face of its
/// own, which the lane cannot tell from a cap whose bit was flipped (the
/// body's tier 3 refuses the flip,
/// `sweep/tests/verbs_cylsph_opening.rs`,
/// `a_flipped_chorded_sphere_face_is_refused_as_a_lamina`).
#[test]
fn a_flipped_bit_reads_the_complement() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7);
    let level = 0.4_f64;
    let cap = ball.cap(FRAC_PI_2 - level);
    let got = sphere_run(
        ball,
        false,
        &north_cap(ball, level, true, true),
        Point3::new(0.1, 0.5, -0.2),
        Name::Own,
    )
    .unwrap_or_else(|e| panic!("the flipped cap measures: {e:?}"));
    encloses(got.area, ball.sphere_area() - cap, "the complement's area");
}

/// **The projected image's metric length is what the map residual pays
/// at**: the cap north of a rim `0.02` rad above the equator, read
/// through the swapped rim at that latitude standing for the true rim
/// `δ` above it, within its stated envelope `r·δ`. The rim's cross
/// product `p × p′` lies near the axis there, so the speed bound reads
/// close to the true speed, and the strip between the two rims, about
/// `L·r·δ`, is paid only at the image's length; read at half the speed
/// bound the residual falls short.
#[test]
fn a_rim_standing_off_its_circle_is_paid_at_its_length() {
    let ball = Ball::tilted(Point3::new(0.4, -0.2, 0.3), 1.3);
    let about = Point3::new(0.1, 0.5, -0.2);
    let (level, delta) = (0.02_f64, 1e-4);
    let image = SphereEdge {
        env: Some(ball.r * delta * (1.0 + 1e-9)),
        ..rim(ball, level, (0.0, TAU), true, true)
    };
    standing_off_holds(
        "the cap read through a rim below its own",
        ball,
        true,
        &[vec![image]],
        &north_cap(ball, level + delta, true, true),
        ball.cap(FRAC_PI_2 - level - delta),
        about,
    );
}
