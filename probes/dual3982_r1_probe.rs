//! Reviewer probe (PR #3982, lane reach-dual3982-r1). Copied into
//! `crates/sweep/tests/` to run; not part of the suite.
//!
//! The oracle owes nothing to the kernel. Each fixture is a profile in
//! the `(ρ, y)` half-plane revolved about `y` by `Θ`, written here twice
//! by hand: as an inside test `inside2d(ρ, y)`, and as the meridian arcs
//! of its unarmed (sphere/torus) faces. A face's support along `n` is
//! the meridian sampled densely, with the azimuth window's own closed
//! form. Volumes are a midpoint grid over `(ρ, y)` with the exact
//! azimuth measure of the half-space at each cell; `point_in_solid` is
//! read against `inside3`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Band, Point2, Point3, Tol, UnitVec3, Vec3};
use profile::{ArcSweep, RawLoop, bulge_from_center, test_support::bulge_loop};
use sweep::{Revolution, revolve};
use topo::splitting::{SplitError, SplitPart, SplitPlane, SplitReduceError, split};
use topo::{Body, DATUM_UNIT_NORM, SolidContainment, point_in_solid, transform_rigid};

struct Fix {
    name: &'static str,
    /// The profile chain, unit scale: `(point, ccw_center)` — the
    /// segment from this point to the next is an arc about the centre
    /// when `Some`, else a line.
    chain: Vec<(Point2<f64>, Option<Point2<f64>>)>,
    theta: Option<f64>,
    inside2d: fn(f64, f64) -> bool,
    /// Unarmed faces' meridian arcs, unit scale: `(centre, radius, t0, t1)`.
    arcs: Vec<((f64, f64), f64, f64, f64)>,
    rect: ((f64, f64), (f64, f64)),
    tangent: Vec<usize>,
}

fn sq(x: f64) -> f64 {
    x * x
}

fn fixtures() -> Vec<Fix> {
    let d = |a: f64| a.to_radians();
    let ring_band = |name, theta| Fix {
        name,
        chain: vec![
            (Point2::new(2.0 - 0.75f64.sqrt(), 0.5), Some(Point2::new(2.0, 0.0))),
            (Point2::new(2.0 + 0.75f64.sqrt(), 0.5), None),
        ],
        theta,
        inside2d: |r, y| sq(r - 2.0) + sq(y) < 1.0 && y < 0.5,
        arcs: vec![((2.0, 0.0), 1.0, d(150.0), d(390.0))],
        rect: ((1.0, 3.0), (-1.0, 0.5)),
        tangent: vec![],
    };
    let capped = |name, theta| Fix {
        name,
        chain: vec![
            (Point2::new(0.0, 0.0), None),
            (Point2::new(1.0, 0.0), None),
            (Point2::new(1.0, 1.0), Some(Point2::new(0.0, 0.25))),
            (Point2::new(0.0, 1.5), None),
        ],
        theta,
        inside2d: |r, y| {
            (y > 0.0 && y < 1.0 && r < 1.0)
                || (y >= 1.0 && sq(r) + sq(y - 0.25) < 1.5625)
        },
        arcs: vec![((0.0, 0.25), 1.25, 0.75f64.atan2(1.0), PI / 2.0)],
        rect: ((0.0, 1.0), (0.0, 1.5)),
        tangent: vec![],
    };
    let rounded = |name, theta| Fix {
        name,
        chain: vec![
            (Point2::new(0.0, 0.0), None),
            (Point2::new(1.0, 0.0), None),
            (Point2::new(1.0, 1.0), Some(Point2::new(0.75, 1.0))),
            (Point2::new(0.75, 1.25), None),
            (Point2::new(0.0, 1.25), None),
        ],
        theta,
        inside2d: |r, y| {
            (y > 0.0 && y <= 1.0 && r < 1.0)
                || (y > 1.0 && y < 1.25 && r < 0.75 + (0.0625 - sq(y - 1.0)).max(0.0).sqrt())
        },
        arcs: vec![((0.75, 1.0), 0.25, 0.0, PI / 2.0)],
        rect: ((0.0, 1.0), (0.0, 1.25)),
        tangent: vec![2, 3],
    };
    vec![
        ring_band("ring band (major arc, complement torus band)", None),
        ring_band("ring band, partial 1.0 rad", Some(1.0)),
        ring_band("ring band, partial 4.5 rad", Some(4.5)),
        Fix {
            name: "double-truncated ball (zone with both rims)",
            chain: vec![
                (Point2::new(0.0, -0.5), None),
                (Point2::new(1.0, -0.5), Some(Point2::new(0.0, 0.25))),
                (Point2::new(1.0, 1.0), None),
                (Point2::new(0.0, 1.0), None),
            ],
            theta: None,
            inside2d: |r, y| y > -0.5 && y < 1.0 && sq(r) + sq(y - 0.25) < 1.5625,
            arcs: vec![((0.0, 0.25), 1.25, (-0.75f64).atan2(1.0), 0.75f64.atan2(1.0))],
            rect: ((0.0, 1.25), (-0.5, 1.0)),
            tangent: vec![],
        },
        Fix {
            name: "lemon (spindle torus, R < r)",
            chain: vec![
                (Point2::new(0.0, -1.0), Some(Point2::new(-0.5, 0.0))),
                (Point2::new(0.0, 1.0), None),
            ],
            theta: None,
            inside2d: |r, y| sq(r + 0.5) + sq(y) < 1.25,
            arcs: vec![((-0.5, 0.0), 1.25f64.sqrt(), (-1.0f64).atan2(0.5), 1.0f64.atan2(0.5))],
            rect: ((0.0, 0.7), (-1.0, 1.0)),
            tangent: vec![],
        },
        Fix {
            name: "apple (spindle torus outer part)",
            chain: vec![
                (Point2::new(0.0, -1.0), Some(Point2::new(0.5, 0.0))),
                (Point2::new(0.0, 1.0), None),
            ],
            theta: None,
            inside2d: |r, y| sq(r - 0.5) + sq(y) < 1.25,
            arcs: vec![((0.5, 0.0), 1.25f64.sqrt(), (-1.0f64).atan2(-0.5), 1.0f64.atan2(-0.5))],
            rect: ((0.0, 1.62), (-1.12, 1.12)),
            tangent: vec![],
        },
        capped("capped cylinder, partial 2.0 rad", Some(2.0)),
        rounded("rounded cylinder, partial 2.0 rad", Some(2.0)),
        rounded("rounded cylinder, partial 5.5 rad", Some(5.5)),
    ]
}

fn build(f: &Fix, s: f64) -> Result<Body<f64>, String> {
    let n = f.chain.len();
    let chain = (0..n)
        .map(|i| {
            let (p, c) = f.chain[i];
            let q = f.chain[(i + 1) % n].0;
            let ps = Point2::new(p.x * s, p.y * s);
            let b = c.map_or(0.0, |c| {
                bulge_from_center(ps, Point2::new(q.x * s, q.y * s), Point2::new(c.x * s, c.y * s), ArcSweep::Ccw)
            });
            (ps, b)
        })
        .collect();
    let rev = f.theta.map_or(Revolution::Full, Revolution::Partial);
    revolve(&validated(vec![bulge_loop(chain).with_tangent_joints(f.tangent.clone())]), axis_y(), rev, Tol::witness())
        .map(|r| r.body)
        .map_err(|e| format!("{e:?}"))
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The greatest `cos(φ − c)` over `φ ∈ [0, Θ]`.
fn max_cos(c: f64, theta: f64) -> f64 {
    for k in -3..=3 {
        let at = c + TAU * k as f64;
        if (0.0..=theta).contains(&at) {
            return 1.0;
        }
    }
    (0.0 - c).cos().max((theta - c).cos())
}

struct Oracle<'a> {
    f: &'a Fix,
    s: f64,
    theta: f64,
    /// `+1` or `−1`: azimuth `φ` places `(ρ, y)` at `(ρ cos φ, y, σ ρ sin φ)`.
    sigma: f64,
}

impl Oracle<'_> {
    fn support(&self, n: Vec3<f64>) -> (f64, f64) {
        let a = (sq(n.x) + sq(n.z)).sqrt();
        let c = (self.sigma * n.z).atan2(n.x);
        // PROBE_FULLAZ: classify against the whole turn (what a zone
        // that ignores the azimuth window reads).
        let th = if std::env::var("PROBE_FULLAZ").is_ok() { TAU } else { self.theta };
        let (up, dn) = (a * max_cos(c, th), a * max_cos(c + PI, th));
        let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
        for &((cx, cy), r, t0, t1) in &self.f.arcs {
            let k = 40_000;
            for i in 0..=k {
                let t = t0 + (t1 - t0) * i as f64 / k as f64;
                let (rho, y) = ((cx + r * t.cos()) * self.s, (cy + r * t.sin()) * self.s);
                if rho < 0.0 {
                    continue;
                }
                hi = hi.max(n.y * y + rho * up);
                lo = lo.min(n.y * y - rho * dn);
            }
        }
        (lo, hi)
    }

    fn inside3(&self, p: Point3<f64>) -> bool {
        let rho = (sq(p.x) + sq(p.z)).sqrt();
        let phi = (self.sigma * p.z).atan2(p.x).rem_euclid(TAU);
        (self.theta >= TAU || phi <= self.theta) && (self.f.inside2d)(rho / self.s, p.y / self.s)
    }

    /// Volume of the part with `n·p > d` (`n` unit), and the whole.
    fn volume(&self, n: Vec3<f64>, d: f64) -> (f64, f64) {
        let a = (sq(n.x) + sq(n.z)).sqrt();
        let c = (self.sigma * n.z).atan2(n.x);
        let ((r0, r1), (y0, y1)) = self.f.rect;
        let m = 700;
        let (hr, hy) = ((r1 - r0) / m as f64, (y1 - y0) / m as f64);
        let (mut part, mut whole) = (0.0, 0.0);
        for i in 0..m {
            let r = r0 + hr * (i as f64 + 0.5);
            for j in 0..m {
                let y = y0 + hy * (j as f64 + 0.5);
                if !(self.f.inside2d)(r, y) {
                    continue;
                }
                let (rho, yy) = (r * self.s, y * self.s);
                let w = rho * self.s * self.s * hr * hy;
                whole += w * self.theta;
                let k = d - n.y * yy;
                let meas = if a < 1e-12 || rho <= 0.0 {
                    // A square cut: the fraction of the cell's height on
                    // the kept side, so the step is integrated exactly.
                    let (c0, c1) = ((y - hy / 2.0) * self.s, (y + hy / 2.0) * self.s);
                    let cut = d / n.y;
                    let frac = if n.y > 0.0 {
                        ((c1 - cut.max(c0)) / (c1 - c0)).clamp(0.0, 1.0)
                    } else {
                        ((cut.min(c1) - c0) / (c1 - c0)).clamp(0.0, 1.0)
                    };
                    let _ = k;
                    self.theta * frac
                } else {
                    let q = k / (a * rho);
                    if q >= 1.0 {
                        0.0
                    } else if q <= -1.0 {
                        self.theta
                    } else {
                        let hw = q.acos();
                        (-3..=3)
                            .map(|m| {
                                let (lo, hi) = (c - hw + TAU * m as f64, c + hw + TAU * m as f64);
                                (hi.min(self.theta) - lo.max(0.0)).max(0.0)
                            })
                            .sum()
                    }
                };
                part += w * meas;
            }
        }
        (part, whole)
    }
}

fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    UnitVec3::new(v, DATUM_UNIT_NORM, band()).unwrap()
}

fn poses(s: f64) -> Vec<(&'static str, Affine3<f64>)> {
    vec![
        ("upright", Affine3::identity()),
        (
            "turned 0.3 z",
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), 0.3),
        ),
        (
            "skew A",
            Affine3::translation(Vec3::new(0.3 * s, -0.7 * s, 0.2 * s))
                * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 2.0, 3.0), 1.1),
        ),
        (
            "skew B",
            Affine3::translation(Vec3::new(-2.0 * s, 0.5 * s, 1.5 * s))
                * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(-0.4, 0.3, 0.86), 2.2),
        ),
    ]
}

fn dirs() -> Vec<Vec3<f64>> {
    let mut out = Vec::new();
    for phi in [0.0f64, 0.3, 0.7, 1.1, PI / 2.0, 2.0, 2.8, PI] {
        for psi in [0.0f64, 1.3, 2.5, 4.0] {
            if (phi == 0.0 || phi == PI) && psi != 0.0 {
                continue;
            }
            out.push(Vec3::new(phi.sin() * psi.cos(), phi.cos(), phi.sin() * psi.sin()));
        }
    }
    out
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[derive(Default, Debug)]
struct Tally {
    clear_split: usize,
    clear_gate: usize,
    clear_other: usize,
    meets_split: usize,
    meets_gate: usize,
    meets_other: usize,
    wrong_volume: usize,
    pis_wrong: usize,
    props_refused: usize,
    reuse_split: usize,
    reuse_pis_wrong: usize,
    /// Largest |kernel − oracle| / whole over every admitted half, in ppm.
    max_rel_ppm: u64,
}

#[test]
#[ignore = "reviewer probe"]
fn dual_probe() {
    let only = std::env::var("PROBE_ONLY").ok();
    let scales: Vec<f64> = std::env::var("PROBE_SCALES")
        .map(|v| v.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or(vec![1.0]);
    let tol = Tol::witness();
    for s in scales {
        for f in fixtures() {
            if only.as_ref().is_some_and(|o| !f.name.contains(o.as_str())) {
                continue;
            }
            let body = match build(&f, s) {
                Ok(b) => b,
                Err(e) => {
                    println!("BUILD-REFUSED {} s={s}: {e}", f.name);
                    continue;
                }
            };
            let theta = f.theta.unwrap_or(TAU);
            // The revolve's sense, from the body itself at mid-window.
            let mid = (theta / 2.0).min(0.3);
            let probe_pt = |sg: f64| {
                let ((r0, r1), (y0, y1)) = f.rect;
                let (mut r, mut y) = ((r0 + r1) / 2.0, (y0 + y1) / 2.0);
                let mut g = Lcg(7);
                while !(f.inside2d)(r, y) {
                    r = r0 + (r1 - r0) * g.next();
                    y = y0 + (y1 - y0) * g.next();
                }
                Point3::new(r * s * mid.cos(), y * s, sg * r * s * mid.sin())
            };
            let sigma = if theta >= TAU {
                1.0
            } else {
                match point_in_solid(&body, probe_pt(1.0), band(), tol) {
                    Ok(SolidContainment::In) => 1.0,
                    _ => -1.0,
                }
            };
            let o = Oracle { f: &f, s, theta, sigma };
            let (_, whole) = o.volume(Vec3::new(0.0, 1.0, 0.0), -1e9);
            if let Ok(p) = topo::props::mass_properties(&body, tol) {
                println!(
                    "WHOLE {} s={s}: kernel {} ± {}, oracle {whole} (sigma {sigma})",
                    f.name, p.volume, p.volume_pad
                );
            }
            for (pose, map) in poses(s) {
                let posed = match transform_rigid(&body, &map, tol) {
                    Ok(b) => b,
                    Err(e) => {
                        println!("POSE-REFUSED {} s={s} {pose}: {e}", f.name);
                        continue;
                    }
                };
                let mut t = Tally::default();
                let mut rng = Lcg(42);
                for n in dirs() {
                    let n = n * (1.0 / n.norm());
                    let (lo, hi) = o.support(n);
                    let mut ds = Vec::new();
                    for delta in [3e-2, 1e-3, 1e-4] {
                        for e in [-delta, delta] {
                            ds.push(lo + e * s);
                            ds.push(hi + e * s);
                        }
                    }
                    for d in ds {
                        let gap = (lo - d).max(d - hi);
                        let clear = gap > 1e-6 * s;
                        let meets = gap < -1e-6 * s;
                        let q = Point3::new(n.x * d, n.y * d, n.z * d);
                        let cut = SplitPlane {
                            origin: map.transform_point(q),
                            normal: unit(map.transform_vec(n)),
                        };
                        match split(&posed, &cut, tol) {
                            Ok(res) => {
                                if clear {
                                    t.clear_split += 1;
                                } else if meets {
                                    t.meets_split += 1;
                                    println!(
                                        "MEETS-SPLIT {} s={s} {pose} n={n:?} d={d} gap={gap:e}",
                                        f.name
                                    );
                                }
                                for (part, sg) in [(&res.above, 1.0), (&res.below, -1.0)] {
                                    let (want, _) = o.volume(n * sg, d * sg);
                                    let SplitPart::Body(b) = part else {
                                        if want > 1e-3 * whole {
                                            t.wrong_volume += 1;
                                            println!("WRONG-EMPTY {} s={s} {pose} n={n:?} d={d} want {want}", f.name);
                                        }
                                        continue;
                                    };
                                    match topo::props::mass_properties(b, tol) {
                                        Ok(p) => {
                                            t.max_rel_ppm = t.max_rel_ppm.max(((p.volume - want).abs() / whole * 1e6) as u64);
                                            if (p.volume - want).abs() > p.volume_pad + 2e-4 * whole {
                                                t.wrong_volume += 1;
                                                println!(
                                                    "WRONG {} s={s} {pose} n={n:?} d={d} side {sg}: {} ± {} vs {want}",
                                                    f.name, p.volume, p.volume_pad
                                                );
                                            }
                                        }
                                        Err(_) => t.props_refused += 1,
                                    }
                                    // point_in_solid against the oracle.
                                    let ((r0, r1), (y0, y1)) = f.rect;
                                    for _ in 0..24 {
                                        let (r, y, ph) = (
                                            (r0 + (r1 - r0) * rng.next()) * s,
                                            (y0 + (y1 - y0) * rng.next()) * s,
                                            theta * rng.next(),
                                        );
                                        let p = Point3::new(r * ph.cos(), y, sigma * r * ph.sin());
                                        let side = sg * (n.dot(Vec3::new(p.x, p.y, p.z)) - d);
                                        if side.abs() < 1e-4 * s {
                                            continue;
                                        }
                                        let want_in = o.inside3(p) && side > 0.0;
                                        match point_in_solid(b, map.transform_point(p), band(), tol) {
                                            Ok(SolidContainment::In) if !want_in => {
                                                t.pis_wrong += 1;
                                                println!("PIS-WRONG {} {pose} n={n:?} d={d} p={p:?} kernel In", f.name);
                                            }
                                            Ok(SolidContainment::Out) if want_in => {
                                                // Near-boundary oracle points are not filtered,
                                                // so read these against the volume verdict.
                                                t.pis_wrong += 1;
                                                println!("PIS-WRONG {} {pose} n={n:?} d={d} p={p:?} kernel Out", f.name);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                // Reuse: split the above half again by a second plane.
                                if let (SplitPart::Body(b), true) = (&res.above, clear) {
                                    let n2 = Vec3::new(0.31, -0.5, 0.81);
                                    let n2 = n2 * (1.0 / n2.norm());
                                    let d2 = n2.dot(Vec3::new(1.6 * s, 0.1 * s, 0.4 * s));
                                    let cut2 = SplitPlane {
                                        origin: map.transform_point(Point3::new(n2.x * d2, n2.y * d2, n2.z * d2)),
                                        normal: unit(map.transform_vec(n2)),
                                    };
                                    if let Ok(r2) = split(b, &cut2, tol) {
                                        t.reuse_split += 1;
                                        for (part, sg2) in [(&r2.above, 1.0), (&r2.below, -1.0)] {
                                            let SplitPart::Body(b2) = part else { continue };
                                            let ((r0, r1), (y0, y1)) = f.rect;
                                            for _ in 0..12 {
                                                let (r, y, ph) = (
                                                    (r0 + (r1 - r0) * rng.next()) * s,
                                                    (y0 + (y1 - y0) * rng.next()) * s,
                                                    theta * rng.next(),
                                                );
                                                let p = Point3::new(r * ph.cos(), y, sigma * r * ph.sin());
                                                let pv = Vec3::new(p.x, p.y, p.z);
                                                let (s1, s2) = (n.dot(pv) - d, sg2 * (n2.dot(pv) - d2));
                                                if s1.abs() < 1e-4 * s || s2.abs() < 1e-4 * s {
                                                    continue;
                                                }
                                                let want_in = o.inside3(p) && s1 > 0.0 && s2 > 0.0;
                                                match point_in_solid(b2, map.transform_point(p), band(), tol) {
                                                    Ok(SolidContainment::In) if !want_in => t.reuse_pis_wrong += 1,
                                                    Ok(SolidContainment::Out) if want_in => t.reuse_pis_wrong += 1,
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(SplitError::Reduce(SplitReduceError::CurvedBooleanUnsupported { .. })) => {
                                if clear {
                                    t.clear_gate += 1;
                                } else if meets {
                                    t.meets_gate += 1;
                                }
                            }
                            Err(e) => {
                                if clear {
                                    t.clear_other += 1;
                                    println!("OTHER-CLEAR {} s={s} {pose} n={n:?} d={d}: {e}", f.name);
                                } else if meets {
                                    t.meets_other += 1;
                                }
                            }
                        }
                    }
                }
                println!("ROW | {} | s={s:e} | {pose} | {t:?}", f.name);
            }
        }
    }
}
