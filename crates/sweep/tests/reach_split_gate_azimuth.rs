//! The split gate reads a sphere face's azimuth window as well as its
//! latitude window: a cap revolved through part of a turn is bounded by
//! the patch it is, not by the whole ring of its latitudes, so a cut
//! clear of the patch splits however the body is turned, at every
//! scale, and a cut into it refuses.
//!
//! The oracle owes nothing to the kernel. Each fixture is a profile in
//! the `(ρ, y)` half-plane revolved about `y` through `Θ`. A face's
//! support along `n` is closed form over its meridian arc and the
//! azimuth window (and checked against a dense sample of the patch);
//! a half's volume is a slice integral across `y`, each slice the
//! sector `ρ ≤ R(y)`, `φ ∈ [0, Θ]` cut by a line, whose area is closed
//! form piece by piece in `φ`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Band, Point2, Point3, Tol, UnitVec3, Vec3};
use profile::{ArcSweep, bulge_from_center, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::splitting::{SplitError, SplitPart, SplitPlane, SplitReduceError, split};
use topo::{AtRestBody, Body, DATUM_UNIT_NORM, transform_rigid};

fn sq(x: f64) -> f64 {
    x * x
}

/// One fixture at unit scale: the profile, the revolve's sweep `Θ`, its
/// radius at height `y` (the slice integral's integrand), the heights
/// where that radius is not smooth, and the sphere face's meridian arc
/// `(ρ, y) = (cx + r cos t, cy + r sin t)`, `t ∈ [t0, t1]`.
struct Fixture {
    name: &'static str,
    /// Each point, and the centre of the arc to the next point
    /// (counter-clockwise) or `None` for a line.
    chain: Vec<(Point2<f64>, Option<Point2<f64>>)>,
    theta: f64,
    radius_at: fn(f64) -> f64,
    breaks: Vec<f64>,
    arc: ((f64, f64), f64, (f64, f64)),
}

/// The sphere of radius 5/4 about `(0, 1/4)`.
const R: f64 = 1.25;
const CY: f64 = 0.25;

fn ball(y: f64) -> f64 {
    (sq(R) - sq(y - CY)).max(0.0).sqrt()
}

fn fixtures() -> Vec<Fixture> {
    let rim = 0.75f64.atan2(1.0);
    vec![
        // The unit cylinder `y ∈ [0, 1]` under the cap `y ≥ 1`.
        Fixture {
            name: "capped cylinder, partial 2.0 rad",
            chain: vec![
                (Point2::new(0.0, 0.0), None),
                (Point2::new(1.0, 0.0), None),
                (Point2::new(1.0, 1.0), Some(Point2::new(0.0, CY))),
                (Point2::new(0.0, 1.5), None),
            ],
            theta: 2.0,
            radius_at: |y| if y <= 1.0 { 1.0 } else { ball(y) },
            breaks: vec![0.0, 1.0, 1.5],
            arc: ((0.0, CY), R, (rim, FRAC_PI_2)),
        },
        // The ball below `y = 1`: the complement zone, reaching the
        // south pole, through more than half a turn.
        Fixture {
            name: "truncated ball, partial 4.0 rad",
            chain: vec![
                (Point2::new(0.0, -1.0), Some(Point2::new(0.0, CY))),
                (Point2::new(1.0, 1.0), None),
                (Point2::new(0.0, 1.0), None),
            ],
            theta: 4.0,
            radius_at: ball,
            breaks: vec![-1.0, 1.0],
            arc: ((0.0, CY), R, (-FRAC_PI_2, rim)),
        },
        // The ball between `y = −1/2` and `y = 1`: a zone with two rims.
        Fixture {
            name: "ball cut at both ends, partial 1.0 rad",
            chain: vec![
                (Point2::new(0.0, -0.5), None),
                (Point2::new(1.0, -0.5), Some(Point2::new(0.0, CY))),
                (Point2::new(1.0, 1.0), None),
                (Point2::new(0.0, 1.0), None),
            ],
            theta: 1.0,
            radius_at: ball,
            breaks: vec![-0.5, 1.0],
            arc: ((0.0, CY), R, ((-0.75f64).atan2(1.0), rim)),
        },
    ]
}

fn build(f: &Fixture, s: f64) -> Body<f64> {
    let n = f.chain.len();
    let at = |p: Point2<f64>| Point2::new(p.x * s, p.y * s);
    let chain = (0..n)
        .map(|i| {
            let (p, c) = f.chain[i];
            let q = f.chain[(i + 1) % n].0;
            let bulge = c.map_or(0.0, |c| {
                bulge_from_center(at(p), at(q), at(c), ArcSweep::Ccw)
            });
            (at(p), bulge)
        })
        .collect();
    revolve(
        &validated(vec![bulge_loop(chain)]),
        axis_y(),
        Revolution::Partial(f.theta),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The sense of the revolve's azimuth: a profile point `(ρ, y)` at
/// azimuth `φ` sits at `(ρ cos φ, y, σ·ρ sin φ)`, read off the body's
/// own vertices on the end half-plane.
fn sense(body: &Body<f64>, theta: f64) -> f64 {
    let (s, c) = theta.sin_cos();
    for (_, p) in body.vertex_points() {
        let rho = p.x.hypot(p.z);
        if rho > 1e-9 && (p.x - rho * c).abs() < 1e-9 * rho.max(1.0) && p.z.abs() > 1e-6 * rho {
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

/// The face's least and greatest `n·p` (body frame, `n` unit): over the
/// azimuth `ρ·(nₓ cos φ + σ n_z sin φ)` peaks at `ρ·a·M` with `M` the
/// window's best cosine, whatever `ρ ≥ 0` is, and what is left is one
/// sinusoid in `t` over the arc.
fn support(f: &Fixture, s: f64, sigma: f64, n: Vec3<f64>) -> (f64, f64) {
    let a = n.x.hypot(n.z);
    let c = (sigma * n.z).atan2(n.x);
    let ((cx, cy), r, t) = f.arc;
    let most = |m: f64, ny: f64| ny * cy + m * cx + r * m.hypot(ny) * most_cos(ny.atan2(m), t);
    let hi = most(a * most_cos(c, (0.0, f.theta)), n.y);
    let lo = -most(a * most_cos(c + PI, (0.0, f.theta)), -n.y);
    (lo * s, hi * s)
}

/// The same support by a dense sample of the patch.
fn sampled_support(f: &Fixture, s: f64, sigma: f64, n: Vec3<f64>) -> (f64, f64) {
    let ((cx, cy), r, (t0, t1)) = f.arc;
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    let k = 600;
    for i in 0..=k {
        let t = t0 + (t1 - t0) * f64::from(i) / f64::from(k);
        let (rho, y) = (cx + r * t.cos(), cy + r * t.sin());
        for j in 0..=k {
            let phi = f.theta * f64::from(j) / f64::from(k);
            let v = n.x * rho * phi.cos() + n.y * y + n.z * sigma * rho * phi.sin();
            lo = lo.min(v);
            hi = hi.max(v);
        }
    }
    (lo * s, hi * s)
}

/// The area of the sector `ρ ≤ rho`, `φ ∈ [0, Θ]` where
/// `ρ·a·cos(φ − c) > t`, closed form: `[0, Θ]` is cut where the cosine
/// vanishes and where the line meets the arc, and on each piece the
/// radial measure is `0`, `ρ²/2`, `ρ²/2 − t²/(2g²)` or `t²/(2g²)` with
/// `g = a·cos(φ − c)`, whose integral is `t²·tan(φ − c)/(2a²)`.
fn sector_area(rho: f64, theta: f64, a: f64, c: f64, t: f64) -> f64 {
    let full = 0.5 * sq(rho);
    if a < 1e-15 {
        return if t < 0.0 { theta * full } else { 0.0 };
    }
    let mut cuts = vec![0.0, theta];
    let mut offsets = vec![FRAC_PI_2, -FRAC_PI_2];
    if t.abs() < a * rho {
        let w = (t / (a * rho)).acos();
        offsets.extend([w, -w]);
    }
    for o in offsets {
        for k in -3..=3 {
            let x = c + o + TAU * f64::from(k);
            if x > 0.0 && x < theta {
                cuts.push(x);
            }
        }
    }
    cuts.sort_by(f64::total_cmp);
    let tan_part = |p: f64, q: f64| sq(t) / (2.0 * sq(a)) * ((q - c).tan() - (p - c).tan());
    cuts.windows(2)
        .map(|w| {
            let (p, q) = (w[0], w[1]);
            if q - p < 1e-15 {
                return 0.0;
            }
            let g = a * (0.5 * (p + q) - c).cos();
            if g > 0.0 {
                if t <= 0.0 {
                    (q - p) * full
                } else if t >= rho * g {
                    0.0
                } else {
                    (q - p) * full - tan_part(p, q)
                }
            } else if t >= 0.0 {
                0.0
            } else if -t >= rho * -g {
                (q - p) * full
            } else {
                tan_part(p, q)
            }
        })
        .sum()
}

/// The volume of the body's part on the `n·p > d` side: composite
/// Simpson across `y` per smooth piece of the profile, each slice's
/// area closed form ([`sector_area`]).
fn half_volume(f: &Fixture, s: f64, sigma: f64, n: Vec3<f64>, d: f64) -> f64 {
    let a = n.x.hypot(n.z);
    let c = (sigma * n.z).atan2(n.x);
    let area = |y: f64| sector_area((f.radius_at)(y), f.theta, a, c, d / s - n.y * y);
    let mut v = 0.0;
    for w in f.breaks.windows(2) {
        let (lo, hi) = (w[0], w[1]);
        // A cut square to the axis is a step in the slice area at its
        // height, which Simpson cannot integrate across: split there.
        let mut pieces = vec![lo, hi];
        if a < 1e-15 {
            let cut = d / s / n.y;
            if cut > lo && cut < hi {
                pieces.insert(1, cut);
            }
        }
        for p in pieces.windows(2) {
            let (y0, y1) = (p[0], p[1]);
            // Each side of the step reads its slices whole or empty.
            let mid = 0.5 * (y0 + y1);
            let area = |y: f64| {
                if a < 1e-15 {
                    sector_area((f.radius_at)(y), f.theta, a, c, d / s - n.y * mid)
                } else {
                    area(y)
                }
            };
            let k = 20_000;
            let h = (y1 - y0) / f64::from(k);
            let mut acc = area(y0) + area(y1);
            for i in 1..k {
                acc += area(y0 + h * f64::from(i)) * if i % 2 == 1 { 4.0 } else { 2.0 };
            }
            v += acc * h / 3.0;
        }
    }
    v * s * s * s
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn poses(s: f64) -> [(&'static str, Affine3<f64>); 3] {
    [
        ("upright", Affine3::identity()),
        (
            "turned 0.3 about z",
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), 0.3),
        ),
        (
            "skew",
            Affine3::translation(Vec3::new(0.3 * s, -0.7 * s, 0.2 * s))
                * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 2.0, 3.0), 1.1),
        ),
    ]
}

/// The planes, in the body frame at unit scale, as `(n, d)` with `n`
/// unit: cuts square to the axis through and beyond the face, oblique
/// cuts at three tilts in two azimuths, and cuts grazing the face from
/// both sides at its least and greatest support along fourteen tilted
/// directions, `δ` outside and `δ` inside at three `δ`.
fn planes(f: &Fixture, sigma: f64) -> Vec<(Vec3<f64>, f64)> {
    let unit = |phi: f64, psi: f64| Vec3::new(phi.sin() * psi.cos(), phi.cos(), phi.sin() * psi.sin());
    let mut out = Vec::new();
    for c in [
        -0.9, -0.5, 0.05, 0.2, 0.3, 0.5, 0.7, 0.8, 0.9, 0.95, 0.99, 1.05, 1.2, 1.4, 1.6,
    ] {
        out.push((Vec3::new(0.0, 1.0, 0.0), c));
    }
    for phi in [0.3f64, 0.7, 1.1] {
        for psi in [0.0f64, 1.3] {
            let n = unit(phi, psi);
            for c in [-0.5, 0.2, 0.5, 0.8, 0.95, 1.2, 1.4] {
                out.push((n, c * n.y));
            }
        }
    }
    for phi in [0.0f64, 0.3, 0.7, 1.1, 1.4, 2.0, 2.8] {
        for psi in [0.0f64, 1.3] {
            let n = unit(phi, psi);
            let (lo, hi) = support(f, 1.0, sigma, n);
            for delta in [1e-2, 1e-4, 1e-5] {
                out.extend([lo - delta, lo + delta, hi + delta, hi - delta].map(|d| (n, d)));
            }
        }
    }
    out
}

/// What a cut does, against the face's support.
#[derive(Default, Debug)]
struct Tally {
    clear_split: usize,
    clear_gate: usize,
    clear_other: usize,
    meets_split: usize,
    meets_gate: usize,
    meets_other: usize,
    near: usize,
    props_refused: usize,
}

/// Split the posed body with the body-frame plane `n·p = d`, and hold
/// each half's volume to the slice integral.
fn split_and_check(
    f: &Fixture,
    (s, sigma): (f64, f64),
    posed: &AtRestBody<f64>,
    map: Affine3<f64>,
    (n, d): (Vec3<f64>, f64),
    what: &str,
    props_refused: &mut usize,
) -> Result<(), SplitError> {
    let cut = SplitPlane {
        origin: map.transform_point(Point3::new(n.x * d, n.y * d, n.z * d)),
        normal: UnitVec3::new(map.transform_vec(n), DATUM_UNIT_NORM, band()).unwrap(),
    };
    let result = split(posed, &cut, Tol::witness())?;
    for (part, sign, side) in [
        (&result.above, 1.0, "above"),
        (&result.below, -1.0, "below"),
    ] {
        let oracle = half_volume(f, s, sigma, n * sign, d * sign);
        let SplitPart::Body(b) = part else {
            assert!(
                oracle.abs() <= 1e-9 * s * s * s,
                "{what}, {side}: empty against the slice integral's {oracle}"
            );
            continue;
        };
        let p = match topo::props::mass_properties(b, Tol::witness()) {
            Ok(p) => p,
            Err(e) => {
                println!("PROPS {what}, {side}: {e}");
                *props_refused += 1;
                continue;
            }
        };
        assert!(
            (p.volume - oracle).abs() <= p.volume_pad + 1e-7 * s * s * s,
            "{what}, {side}: {} ± {} against the slice integral's {oracle}",
            p.volume,
            p.volume_pad
        );
    }
    Ok(())
}

/// Every fixture's body, posed, at `s`; `None` (stood down loudly) where
/// posing it refuses below the default ε.
fn posed_bodies(f: &Fixture, s: f64) -> Vec<(&'static str, Affine3<f64>, AtRestBody<f64>)> {
    let body = build(f, s);
    let tol = Tol::witness();
    poses(s)
        .into_iter()
        .filter_map(|(pose, map)| match transform_rigid(&body, &map, tol) {
            Ok(posed) => Some((pose, map, finished("the posed body", posed, tol))),
            Err(e) if tol.eps() < geom_core::tolerance::DEFAULT_EPS => {
                test_utils::vacuity::stood_down(
                    &format!("{} at scale {s}, {pose}, eps = {:e}", f.name, tol.eps()),
                    &format!("posing the body refused ({e}) before any cut"),
                );
                None
            }
            Err(e) => panic!("{} at scale {s}, {pose}: posing refused: {e}", f.name),
        })
        .collect()
}

/// The probe: every fixture at three scales and three poses against
/// every plane, tallied against the oracle. Run with `--no-capture` to
/// read the table.
#[test]
#[ignore = "measurement probe: cargo nextest run -p sweep --run-ignored all reach_split_gate_azimuth::probe --no-capture"]
fn probe() {
    for f in fixtures() {
        let sigma = sense(&build(&f, 1.0), f.theta);
        for s in [1e-3, 1.0, 1e3] {
            for (pose, map, posed) in posed_bodies(&f, s) {
                let mut t = Tally::default();
                for (n, d) in planes(&f, sigma) {
                    let (lo, hi) = support(&f, 1.0, sigma, n);
                    let gap = (lo - d).max(d - hi);
                    // Clear of the gate's `12 ε` pad too, at every scale.
                    let margin = 1e-6 + 20.0 * Tol::witness().eps() / s;
                    let (clear, meets) = (gap > margin, gap < -margin);
                    let what = format!("{} s={s} {pose} n={n:?} d={d}", f.name);
                    match split_and_check(&f, (s, sigma), &posed, map, (n, d * s), &what, &mut t.props_refused) {
                        Ok(()) if clear => t.clear_split += 1,
                        Ok(()) if meets => t.meets_split += 1,
                        Err(SplitError::Reduce(
                            SplitReduceError::CurvedBooleanUnsupported { .. },
                        )) if clear => t.clear_gate += 1,
                        Err(SplitError::Reduce(
                            SplitReduceError::CurvedBooleanUnsupported { .. },
                        )) if meets => t.meets_gate += 1,
                        Err(e) if clear => {
                            t.clear_other += 1;
                            println!("OTHER clear {what}: {e}");
                        }
                        Err(_) if meets => t.meets_other += 1,
                        _ => t.near += 1,
                    }
                }
                println!("ROW | {} | {s:e} | {pose} | {t:?}", f.name);
            }
        }
    }
}
