//! The split gate reads a face's own patch of its carrier: a torus
//! face's chart window, partial in azimuth or in the tube, and a sphere
//! zone's latitude window up to its rims. A cut into the patch refuses
//! however shallow it is, and a cut clear of it splits.
//!
//! The oracle owes nothing to the kernel. Each fixture is a profile in
//! the `(ρ, y)` half-plane revolved about `y` through `Θ`, written here
//! by hand twice: as an inside test, and as the meridian arcs of its
//! sphere or torus face. A face's support along `n` is closed form per
//! arc over the azimuth window (and checked against a dense sample of
//! the arc here); a half's volume is a midpoint grid over `(ρ, y)` with
//! the exact azimuth measure of the half-space in each cell.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use crate::revolve_common::{axis_y, validated};
use geom::SurfaceKind;
use geom_core::{Affine3, Band, Point2, Point3, Tol, UnitVec3, Vec3};
use profile::{ArcSweep, RawLoop, bulge_from_center, test_support::bulge_loop};
use sweep::{Revolution, revolve};
use topo::splitting::{SplitError, SplitPart, SplitPlane, SplitReduceError, split};
use topo::{Body, DATUM_UNIT_NORM, transform_rigid};

/// A meridian arc `(ρ, y) = (cx + r cos t, cy + r sin t)`, `t ∈ [t0, t1]`.
#[derive(Clone, Copy)]
struct Arc {
    centre: (f64, f64),
    r: f64,
    t: (f64, f64),
}

struct Fixture {
    name: &'static str,
    kind: SurfaceKind,
    /// The profile at unit scale: each point, and the centre of the arc
    /// to the next point (counter-clockwise) or `None` for a line.
    chain: Vec<(Point2<f64>, Option<Point2<f64>>)>,
    tangent_joints: Vec<usize>,
    /// The revolve's sweep, `None` for a full turn.
    theta: Option<f64>,
    inside: fn(f64, f64) -> bool,
    /// The face's meridian at unit scale.
    arc: Arc,
    /// A box over the profile at unit scale, `((ρ0, ρ1), (y0, y1))`.
    rect: ((f64, f64), (f64, f64)),
}

fn sq(x: f64) -> f64 {
    x * x
}

fn fixtures() -> Vec<Fixture> {
    let band = |name, theta| Fixture {
        name,
        kind: SurfaceKind::Torus,
        // The circle of radius 1 about (2, 0) below y = 1/2: its
        // 240° major arc from 150° to 390°, a complement band.
        chain: vec![
            (
                Point2::new(2.0 - 0.75f64.sqrt(), 0.5),
                Some(Point2::new(2.0, 0.0)),
            ),
            (Point2::new(2.0 + 0.75f64.sqrt(), 0.5), None),
        ],
        tangent_joints: Vec::new(),
        theta,
        inside: |r, y| sq(r - 2.0) + sq(y) < 1.0 && y < 0.5,
        arc: Arc {
            centre: (2.0, 0.0),
            r: 1.0,
            t: (150f64.to_radians(), 390f64.to_radians()),
        },
        rect: ((1.0, 3.0), (-1.0, 0.5)),
    };
    let rounded = |name, theta| Fixture {
        name,
        kind: SurfaceKind::Torus,
        chain: vec![
            (Point2::new(0.0, 0.0), None),
            (Point2::new(1.0, 0.0), None),
            (Point2::new(1.0, 1.0), Some(Point2::new(0.75, 1.0))),
            (Point2::new(0.75, 1.25), None),
            (Point2::new(0.0, 1.25), None),
        ],
        tangent_joints: vec![2, 3],
        theta,
        inside: |r, y| {
            (y > 0.0 && y <= 1.0 && r < 1.0)
                || (y > 1.0 && y < 1.25 && r < 0.75 + (0.0625 - sq(y - 1.0)).max(0.0).sqrt())
        },
        arc: Arc {
            centre: (0.75, 1.0),
            r: 0.25,
            t: (0.0, PI / 2.0),
        },
        rect: ((0.0, 1.0), (0.0, 1.25)),
    };
    vec![
        band("complement torus band", None),
        band("complement torus band, partial 1.0 rad", Some(1.0)),
        rounded("rounded cylinder, partial 2.0 rad", Some(2.0)),
        Fixture {
            name: "capped cylinder",
            kind: SurfaceKind::Sphere,
            chain: vec![
                (Point2::new(0.0, 0.0), None),
                (Point2::new(1.0, 0.0), None),
                (Point2::new(1.0, 1.0), Some(Point2::new(0.0, 0.25))),
                (Point2::new(0.0, 1.5), None),
            ],
            tangent_joints: Vec::new(),
            theta: None,
            inside: |r, y| {
                (y > 0.0 && y < 1.0 && r < 1.0) || (y >= 1.0 && sq(r) + sq(y - 0.25) < 1.5625)
            },
            arc: Arc {
                centre: (0.0, 0.25),
                r: 1.25,
                t: (0.75f64.atan2(1.0), PI / 2.0),
            },
            rect: ((0.0, 1.0), (0.0, 1.5)),
        },
        Fixture {
            name: "truncated ball",
            kind: SurfaceKind::Sphere,
            chain: vec![
                (Point2::new(0.0, -1.0), Some(Point2::new(0.0, 0.25))),
                (Point2::new(1.0, 1.0), None),
                (Point2::new(0.0, 1.0), None),
            ],
            tangent_joints: Vec::new(),
            theta: None,
            inside: |r, y| y < 1.0 && sq(r) + sq(y - 0.25) < 1.5625,
            arc: Arc {
                centre: (0.0, 0.25),
                r: 1.25,
                t: (-PI / 2.0, 0.75f64.atan2(1.0)),
            },
            rect: ((0.0, 1.25), (-1.0, 1.0)),
        },
        Fixture {
            name: "ball cut at both ends (a zone with two rims)",
            kind: SurfaceKind::Sphere,
            chain: vec![
                (Point2::new(0.0, -0.5), None),
                (Point2::new(1.0, -0.5), Some(Point2::new(0.0, 0.25))),
                (Point2::new(1.0, 1.0), None),
                (Point2::new(0.0, 1.0), None),
            ],
            tangent_joints: Vec::new(),
            theta: None,
            inside: |r, y| y > -0.5 && y < 1.0 && sq(r) + sq(y - 0.25) < 1.5625,
            arc: Arc {
                centre: (0.0, 0.25),
                r: 1.25,
                t: ((-0.75f64).atan2(1.0), 0.75f64.atan2(1.0)),
            },
            rect: ((0.0, 1.25), (-0.5, 1.0)),
        },
    ]
}

fn build(f: &Fixture, s: f64) -> Body<f64> {
    let n = f.chain.len();
    let chain = (0..n)
        .map(|i| {
            let (p, c) = f.chain[i];
            let q = f.chain[(i + 1) % n].0;
            let at = |p: Point2<f64>| Point2::new(p.x * s, p.y * s);
            let bulge = c.map_or(0.0, |c| {
                bulge_from_center(at(p), at(q), at(c), ArcSweep::Ccw)
            });
            (at(p), bulge)
        })
        .collect();
    revolve(
        &validated(vec![
            bulge_loop(chain).with_tangent_joints(f.tangent_joints.clone()),
        ]),
        axis_y(),
        f.theta.map_or(Revolution::Full, Revolution::Partial),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The sense of the revolve's azimuth: a profile point `(ρ, y)` at
/// azimuth `φ` sits at `(ρ cos φ, y, σ·ρ sin φ)`, read off the body's
/// own vertices on the end half-plane of a partial revolve.
fn sense(body: &Body<f64>, theta: Option<f64>) -> f64 {
    let Some(theta) = theta else { return 1.0 };
    let (s, c) = theta.sin_cos();
    for (_, v) in body.vertices() {
        let p = *body.get_point(v.point).unwrap();
        let rho = p.x.hypot(p.z);
        if rho > 1e-6 && (p.x - rho * c).abs() < 1e-9 * rho.max(1.0) && p.z.abs() > 1e-6 * rho {
            return (p.z / (rho * s)).signum();
        }
    }
    panic!("a partial revolve has a vertex on its end half-plane")
}

/// The greatest `cos(φ − c)` over `φ ∈ [lo, hi]`.
fn most_cos(c: f64, (lo, hi): (f64, f64)) -> f64 {
    if (-4..=4).any(|k| (lo..=hi).contains(&(c + TAU * f64::from(k)))) {
        1.0
    } else {
        (lo - c).cos().max((hi - c).cos())
    }
}

/// The face's least and greatest `n·p` (body frame, `n` unit): over
/// the azimuth `ρ·(nₓ cos φ + σ n_z sin φ)` peaks at `ρ·a·M` with
/// `M` the window's best cosine, and `ρ ≥ 0` on every arc here, so
/// what is left is one sinusoid in `t` over the arc.
fn support(f: &Fixture, s: f64, sigma: f64, n: Vec3<f64>) -> (f64, f64) {
    let theta = f.theta.unwrap_or(TAU);
    let a = n.x.hypot(n.z);
    let c = (sigma * n.z).atan2(n.x);
    let Arc {
        centre: (cx, cy),
        r,
        t,
    } = f.arc;
    let most = |m: f64, ny: f64| ny * cy + m * cx + r * m.hypot(ny) * most_cos(ny.atan2(m), t);
    let hi = most(a * most_cos(c, (0.0, theta)), n.y);
    let lo = -most(a * most_cos(c + PI, (0.0, theta)), -n.y);
    (lo * s, hi * s)
}

/// The same support by a dense sample of the face, which the closed
/// form must agree with to the sample's own chord error.
fn sampled_support(f: &Fixture, s: f64, sigma: f64, n: Vec3<f64>) -> (f64, f64) {
    let theta = f.theta.unwrap_or(TAU);
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    let k = 600;
    for i in 0..=k {
        let t = f.arc.t.0 + (f.arc.t.1 - f.arc.t.0) * f64::from(i) / f64::from(k);
        let (rho, y) = (
            f.arc.centre.0 + f.arc.r * t.cos(),
            f.arc.centre.1 + f.arc.r * t.sin(),
        );
        for j in 0..=k {
            let phi = theta * f64::from(j) / f64::from(k);
            let v = n.x * rho * phi.cos() + n.y * y + n.z * sigma * rho * phi.sin();
            lo = lo.min(v);
            hi = hi.max(v);
        }
    }
    (lo * s, hi * s)
}

/// The volume of the body's part on the `n·p > d` side: a midpoint grid
/// over `(ρ, y)`, each cell inside the profile weighted by `ρ` and by
/// the exact measure of the azimuths in `[0, Θ]` it puts on that side.
fn half_volume(f: &Fixture, s: f64, sigma: f64, n: Vec3<f64>, d: f64) -> f64 {
    let theta = f.theta.unwrap_or(TAU);
    let a = n.x.hypot(n.z);
    let c = (sigma * n.z).atan2(n.x);
    let ((r0, r1), (y0, y1)) = f.rect;
    let k = 400;
    let (hr, hy) = ((r1 - r0) / f64::from(k), (y1 - y0) / f64::from(k));
    let mut v = 0.0;
    for i in 0..k {
        let rho = r0 + hr * (f64::from(i) + 0.5);
        for j in 0..k {
            let y = y0 + hy * (f64::from(j) + 0.5);
            if !(f.inside)(rho, y) {
                continue;
            }
            // `a·cos(φ − c) > (d/s − n_y·y)/ρ`.
            let need = (d / s - n.y * y) / rho;
            let measure = if a < 1e-15 {
                if need < 0.0 { theta } else { 0.0 }
            } else if need >= a {
                0.0
            } else if need <= -a {
                theta
            } else {
                let w = (need / a).acos();
                (-3..=3)
                    .map(|m| {
                        let (lo, hi) = (c - w + TAU * f64::from(m), c + w + TAU * f64::from(m));
                        (hi.min(theta) - lo.max(0.0)).max(0.0)
                    })
                    .sum()
            };
            v += rho * measure;
        }
    }
    v * hr * hy * s * s * s
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// Thirteen directions spread over the sphere, none on a world axis.
fn directions() -> Vec<Vec3<f64>> {
    (0..13)
        .map(|i| {
            let z = 1.0 - (2.0 * f64::from(i) + 1.0) / 13.0;
            let phi = f64::from(i) * PI * (3.0 - 5f64.sqrt()) + 0.3;
            let r = (1.0 - z * z).sqrt();
            Vec3::new(r * phi.cos(), z, r * phi.sin())
        })
        .collect()
}

fn poses(s: f64) -> [(&'static str, Affine3<f64>); 2] {
    [
        ("upright", Affine3::identity()),
        (
            "skew",
            Affine3::translation(Vec3::new(0.3 * s, -0.7 * s, 0.2 * s))
                * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 2.0, 3.0), 1.1),
        ),
    ]
}

/// The plane `n·p = d` in the body frame, posed by `map`.
fn posed_plane(map: Affine3<f64>, n: Vec3<f64>, d: f64) -> SplitPlane<f64> {
    SplitPlane {
        origin: map.transform_point(Point3::new(n.x * d, n.y * d, n.z * d)),
        normal: UnitVec3::new(map.transform_vec(n), DATUM_UNIT_NORM, band()).unwrap(),
    }
}

/// **A cut into the face refuses at the gate, however shallow it is**:
/// `10⁻⁴` and `10⁻⁷` of the size past the face's least and greatest
/// support, along thirteen directions, in two poses. The tori's windows
/// are partial in the tube (the complement band's 240° meridian) and in
/// azimuth (the 1.0 and 2.0 rad revolves), where a crest read off the
/// wrong end of a window, or on the other window, admits cuts through
/// the face. The zones' cuts at `10⁻⁷` graze their rims and poles,
/// where a zone narrowed by a millionth of its radius admits them.
#[test]
fn a_cut_into_the_faces_window_refuses_at_the_gate() {
    let s = 1.0;
    for f in fixtures() {
        let body = build(&f, s);
        let sigma = sense(&body, f.theta);
        for (pose, map) in poses(s) {
            let posed = transform_rigid(&body, &map, Tol::witness()).unwrap();
            for n in directions() {
                let (lo, hi) = support(&f, s, sigma, n);
                for depth in [1e-4 * s, 1e-7 * s] {
                    for d in [lo + depth, hi - depth] {
                        let what = format!("{}, {pose}, n = {n:?}, {depth:e} in", f.name);
                        match split(&posed, &posed_plane(map, n, d), Tol::witness()) {
                            Err(SplitError::Reduce(
                                SplitReduceError::CurvedBooleanUnsupported { face, kind },
                            )) => {
                                assert_eq!(kind, f.kind, "{what}");
                                let surface = posed.get_face(face).unwrap().surface;
                                assert_eq!(
                                    posed.get_surface(surface).unwrap().kind(),
                                    f.kind,
                                    "{what}"
                                );
                            }
                            other => panic!(
                                "{what}: a cut through the face must refuse at the gate, got {other:?}"
                            ),
                        }
                    }
                }
            }
        }
    }
}

/// **A cut clear of the face's window splits, and its halves are
/// right**: `10⁻⁴` of the size plus `20 ε` beyond the face's least and
/// greatest support (clear of the gate's `12 ε` pad), along thirteen
/// directions, in two poses. The zones' fixtures are full turns: a
/// sphere face's azimuth window is not read
/// (`reach/split-gate-zone-ignores-the-azimuth-window`). Each half's
/// volume is held to the grid within `2·10⁻³` of the body's, whose own
/// volume is first held to the same grid.
#[test]
fn a_cut_clear_of_the_faces_window_splits() {
    let s = 1.0;
    let eps = Tol::witness().eps();
    for f in fixtures() {
        let body = build(&f, s);
        let sigma = sense(&body, f.theta);
        let whole = topo::props::mass_properties(&body, Tol::witness())
            .unwrap()
            .volume;
        let grid = half_volume(&f, s, sigma, Vec3::new(0.0, 1.0, 0.0), -10.0 * s);
        let slack = 2e-3 * whole;
        assert!(
            (whole - grid).abs() <= slack,
            "{}: the grid reads {grid} for a body of {whole}",
            f.name
        );
        for (pose, map) in poses(s) {
            let posed = transform_rigid(&body, &map, Tol::witness()).unwrap();
            for n in directions() {
                let (lo, hi) = support(&f, s, sigma, n);
                let (slo, shi) = sampled_support(&f, s, sigma, n);
                assert!(
                    lo <= slo + 1e-12
                        && hi >= shi - 1e-12
                        && slo - lo <= 1e-4 * s
                        && hi - shi <= 1e-4 * s,
                    "{}: the closed-form support [{lo}, {hi}] along {n:?} disagrees with the sample [{slo}, {shi}]",
                    f.name
                );
                let clear = 1e-4 * s + 20.0 * eps;
                for d in [lo - clear, hi + clear] {
                    let what = format!("{}, {pose}, n = {n:?}, d = {d}", f.name);
                    let result = split(&posed, &posed_plane(map, n, d), Tol::witness())
                        .unwrap_or_else(|e| panic!("{what}: a cut clear of the face splits: {e}"));
                    for (part, sign, side) in [
                        (&result.above, 1.0, "above"),
                        (&result.below, -1.0, "below"),
                    ] {
                        let oracle = half_volume(&f, s, sigma, n * sign, d * sign);
                        let got = match part {
                            SplitPart::Body(b) => {
                                topo::props::mass_properties(b, Tol::witness())
                                    .unwrap()
                                    .volume
                            }
                            SplitPart::Empty => 0.0,
                        };
                        assert!(
                            (got - oracle).abs() <= slack,
                            "{what}, {side}: {got} against the grid's {oracle}"
                        );
                    }
                }
            }
        }
    }
}
