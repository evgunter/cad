//! Review probes for PR 4143 (lane band-dual-4143-r2): sweeps of an
//! obstacle toward a blend band, each outcome checked against an
//! analytic interference oracle and, where the body builds, against
//! tier 3' (`validate_pseudomanifold`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use sweep::blend::fillet_edges;
use sweep::test_support::ball_poled_z;
use topo::{Body, ContactRecords, mass_properties, validate_geometric};

use crate::common::cavity::{brick, cavity_edges, cut, edges_with_corners};

fn tol() -> Tol {
    Tol::witness()
}

fn fuse(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let t = tol();
    let a = sweep::test_support::finished("a", a.clone(), t);
    let b = sweep::test_support::finished("b", b.clone(), t);
    topo::union(&a, &b, t)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone()
        .into_body()
}

/// Distance from `p` to the box `[lo, hi]^3`.
fn dist_box(p: [f64; 3], lo: f64, hi: f64) -> f64 {
    p.iter()
        .map(|c| (lo - c).max(c - hi).max(0.0).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// One outcome line, plus whether it is a soundness failure (built with
/// an interference depth above `1e-9`).
fn judge(what: &str, depth: f64, out: Result<Body<f64>, String>) -> (bool, String) {
    match out {
        Ok(b) => {
            let pm = topo::validate_pseudomanifold(&b, &ContactRecords::default(), tol());
            let v = mass_properties(&b, tol()).map(|m| m.volume).ok();
            let geo = validate_geometric(&b, tol()).is_ok();
            (
                depth > 1e-9,
                format!(
                    "{what}: depth {depth:+.4} BUILT geo_ok={geo} pm_ok={} V={v:?}",
                    pm.is_ok()
                ),
            )
        }
        Err(e) => (false, format!("{what}: depth {depth:+.4} refused {e}")),
    }
}

/// CONCAVE: the sealed `[1,3]^3` cavity in `[0,4]^3`, its twelve edges
/// filleted at `r`, a ball island (a second solid) of radius `s` moved
/// toward an edge, a corner and an off-diagonal spot. The added material
/// is `{x in cavity : dist(x, [1+r, 3-r]^3) > r}`, so a ball at `c`
/// reaches it by `dist(c, B) + s - r`.
#[test]
fn concave_ball_island_sweep() {
    let r = 0.25;
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let cav = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let sealed = cut("cavity", &block, &cav);
    let mut bad = Vec::new();
    for &s in &[0.05, 0.1] {
        // direction (unit-ish offsets from the cavity's inner corner)
        let poses: Vec<(&str, Box<dyn Fn(f64) -> [f64; 3]>)> = vec![
            ("edge", Box::new(move |g| [1.0 + s + g, 1.0 + s + g, 2.0])),
            (
                "corner",
                Box::new(move |g| [1.0 + s + g, 1.0 + s + g, 1.0 + s + g]),
            ),
            (
                "near-corner",
                Box::new(move |g| [1.0 + s + g, 1.0 + s + g, 1.2]),
            ),
            (
                "floor-mid",
                Box::new(move |g| [1.0 + s + g, 2.0, 1.0 + s + g / 2.0]),
            ),
            (
                "skew",
                Box::new(move |g| [1.0 + s + g, 1.0 + s + 2.0 * g, 1.0 + s + 0.5 * g]),
            ),
        ];
        for (name, at) in &poses {
            for &g in &[0.003, 0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.09] {
                let c = at(g);
                let depth = dist_box(c, 1.0 + r, 3.0 - r) + s - r;
                let ball = ball_poled_z(s, Vec3::new(c[0], c[1], c[2]), tol());
                let body = fuse(&sealed, &ball);
                let edges = cavity_edges(&body);
                assert_eq!(edges.len(), 12);
                let out = fillet_edges(&body, &edges, r, tol())
                    .map(|f| f.body)
                    .map_err(|e| format!("{:?}", e.error));
                let (fail, line) = judge(&format!("concave s={s} {name} g={g}"), depth, out);
                println!("{line}");
                if fail {
                    bad.push(line);
                }
            }
        }
    }
    assert!(bad.is_empty(), "built through interference: {bad:#?}");
}

/// CONVEX: `[0,4]^3` with a ball VOID of radius `s` near an outer edge or
/// corner; the twelve outer edges filleted at `r`. The removed material is
/// `{x in block : dist(x, [r, 4-r]^3) > r}`.
#[test]
fn convex_ball_void_sweep() {
    let r = 0.5;
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let mut bad = Vec::new();
    for &s in &[0.1, 0.2] {
        let poses: Vec<(&str, Box<dyn Fn(f64) -> [f64; 3]>)> = vec![
            ("edge", Box::new(move |g| [s + g, s + g, 2.0])),
            ("corner", Box::new(move |g| [s + g, s + g, s + g])),
            ("near-corner", Box::new(move |g| [s + g, s + g, 0.6])),
        ];
        for (name, at) in &poses {
            for &g in &[0.02, 0.05, 0.08, 0.1, 0.12, 0.15, 0.2, 0.25] {
                let c = at(g);
                let depth = dist_box(c, r, 4.0 - r) + s - r;
                let ball = ball_poled_z(s, Vec3::new(c[0], c[1], c[2]), tol());
                let body = cut("void", &block, &ball);
                let outer = |p: Point3<f64>| {
                    p.to_array()
                        .iter()
                        .all(|c| c.abs() < 1e-12 || (c - 4.0).abs() < 1e-12)
                };
                let edges = edges_with_corners(&body, outer);
                assert_eq!(edges.len(), 12);
                let out = fillet_edges(&body, &edges, r, tol())
                    .map(|f| f.body)
                    .map_err(|e| format!("{:?}", e.error));
                let (fail, line) = judge(&format!("convex s={s} {name} g={g}"), depth, out);
                println!("{line}");
                if fail {
                    bad.push(line);
                }
            }
        }
    }
    assert!(bad.is_empty(), "built through interference: {bad:#?}");
}

#[test]
fn pm_on_the_unfilleted_and_filleted_island() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let cav = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let sealed = cut("cavity", &block, &cav);
    let ball = ball_poled_z(0.1, Vec3::new(1.2, 1.2, 2.0), tol());
    let body = fuse(&sealed, &ball);
    println!(
        "unfilleted: {:?}",
        topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol())
    );
    let ball2 = ball_poled_z(0.1, Vec3::new(2.0, 2.0, 2.0), tol());
    let body2 = fuse(&sealed, &ball2);
    let f = fillet_edges(&body2, &cavity_edges(&body2), 0.25, tol()).unwrap();
    println!(
        "centred ball, filleted: {:?}",
        topo::validate_pseudomanifold(&f.body, &ContactRecords::default(), tol())
    );
    let f0 = fillet_edges(&sealed, &cavity_edges(&sealed), 0.25, tol()).unwrap();
    println!(
        "no ball, filleted: {:?}",
        topo::validate_pseudomanifold(&f0.body, &ContactRecords::default(), tol())
    );
}

#[test]
fn pm_error_on_a_clear_island() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let cav = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let sealed = cut("cavity", &block, &cav);
    for c in [1.19, 1.25, 1.3, 1.5] {
        let ball = ball_poled_z(0.1, Vec3::new(c, c, 2.0), tol());
        let body = fuse(&sealed, &ball);
        let f = fillet_edges(&body, &cavity_edges(&body), 0.25, tol()).unwrap();
        let e = topo::validate_pseudomanifold(&f.body, &ContactRecords::default(), tol());
        println!(
            "PM c={c}: {:?}",
            e.map_err(|v| format!("{v:?}").chars().take(700).collect::<String>())
        );
    }
}

/// Max over a ball (grid-sampled) of `depth(p)`.
fn ball_depth(c: [f64; 3], s: f64, depth: impl Fn([f64; 3]) -> f64) -> f64 {
    let n = 28;
    let mut m = f64::NEG_INFINITY;
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                let d = [i, j, k].map(|t| -s + 2.0 * s * t as f64 / n as f64);
                if d.iter().map(|x| x * x).sum::<f64>() > s * s {
                    continue;
                }
                m = m.max(depth([c[0] + d[0], c[1] + d[1], c[2] + d[2]]));
            }
        }
    }
    // the sphere's surface too, finely
    for i in 0..200 {
        for j in 0..100 {
            let (a, b) = (
                std::f64::consts::TAU * i as f64 / 200.0,
                std::f64::consts::PI * j as f64 / 99.0,
            );
            let p = [
                c[0] + s * b.sin() * a.cos(),
                c[1] + s * b.cos(),
                c[2] + s * b.sin() * a.sin(),
            ];
            m = m.max(depth(p));
        }
    }
    m
}

/// CIRCULAR, concave: a round void `rho <= 1.5, y in [1, 3]` about `y`
/// sealed in a block; its floor rim filleted at `r = 0.25`; a ball island
/// of radius `s` off the axis moved toward the rim. The added material in
/// the sheet is the corner square `[1.25, 1.5] x [1, 1.25]` outside the
/// circle about `(1.25, 1.25)`.
#[test]
fn circular_ball_island_sweep() {
    use sweep::Revolution;
    use sweep::test_support::{corners, revolved_about_y, rim_arcs_at};
    let r = 0.25;
    let block = brick(Point3::new(-3.0, 0.0, -3.0), Point3::new(3.0, 4.0, 3.0));
    let void = revolved_about_y(
        corners(&[(0.0, 1.0), (1.5, 1.0), (1.5, 3.0), (0.0, 3.0)]),
        Revolution::Full,
        tol(),
    );
    let sealed = cut("void", &block, &void);
    let depth = |p: [f64; 3]| {
        let rho = p[0].hypot(p[2]);
        let y = p[1];
        let d = ((rho - 1.25).powi(2) + (y - 1.25).powi(2)).sqrt() - r;
        (rho - 1.25).min(1.25 - y).min(d)
    };
    let mut bad = Vec::new();
    for &s in &[0.06, 0.12] {
        for &phi in &[0.3_f64, 2.0] {
            let poses: Vec<(&str, Box<dyn Fn(f64) -> (f64, f64)>)> = vec![
                ("bisector", Box::new(move |g| (1.5 - s - g, 1.0 + s + g))),
                (
                    "floor",
                    Box::new(move |g| (1.5 - s - 2.0 * g, 1.0 + s + g * 0.3)),
                ),
                (
                    "wall",
                    Box::new(move |g| (1.5 - s - g * 0.3, 1.0 + s + 2.0 * g)),
                ),
            ];
            for (name, at) in &poses {
                for &g in &[0.005, 0.02, 0.035, 0.05, 0.065, 0.08, 0.1] {
                    let (rho, y) = at(g);
                    let c = [rho * phi.cos(), y, rho * phi.sin()];
                    let dep = ball_depth(c, s, depth);
                    let ball = ball_poled_z(s, Vec3::new(c[0], c[1], c[2]), tol());
                    let body = fuse(&sealed, &ball);
                    let edges = rim_arcs_at(&body, 1.5, 1.0);
                    let out = fillet_edges(&body, &edges, r, tol())
                        .map(|f| f.body)
                        .map_err(|e| format!("{:?}", e.error).chars().take(160).collect());
                    let (fail, line) =
                        judge(&format!("circ s={s} phi={phi} {name} g={g}"), dep, out);
                    println!("{line}");
                    if fail {
                        bad.push(line);
                    }
                }
            }
        }
    }
    assert!(bad.is_empty(), "built through interference: {bad:#?}");
}

/// CHAMFER, concave: the sealed cavity's twelve edges chamfered at `d`,
/// a ball island near an edge and a corner. The added material is the
/// cavity within `(x-1)+(y-1) < d` of each edge and `sum - 3 < 2d` of
/// each corner (the plane through the three feet).
#[test]
fn concave_chamfer_ball_island_sweep() {
    let d = 0.25;
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let cav = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let sealed = cut("cavity", &block, &cav);
    let mut bad = Vec::new();
    for &s in &[0.05, 0.1] {
        let poses: Vec<(&str, Box<dyn Fn(f64) -> [f64; 3]>)> = vec![
            ("edge", Box::new(move |g| [1.0 + s + g, 1.0 + s + g, 2.0])),
            (
                "corner",
                Box::new(move |g| [1.0 + s + g, 1.0 + s + g, 1.0 + s + g]),
            ),
            (
                "skew",
                Box::new(move |g| [1.0 + s + g, 1.0 + s + 2.0 * g, 1.0 + s + 0.5 * g]),
            ),
        ];
        for (name, at) in &poses {
            for &g in &[0.003, 0.02, 0.04, 0.06, 0.08, 0.1, 0.12, 0.15] {
                let c = at(g);
                let u = [c[0] - 1.0, c[1] - 1.0, c[2] - 1.0];
                let edge = [(0, 1), (0, 2), (1, 2)]
                    .iter()
                    .map(|&(i, j)| (d - u[i] - u[j]) / 2f64.sqrt())
                    .fold(f64::NEG_INFINITY, f64::max);
                let corner = (2.0 * d - u[0] - u[1] - u[2]) / 3f64.sqrt();
                let depth = edge.max(corner) + s;
                let ball = ball_poled_z(s, Vec3::new(c[0], c[1], c[2]), tol());
                let body = fuse(&sealed, &ball);
                let edges = cavity_edges(&body);
                let out = sweep::blend::chamfer_edges(&body, &edges, d, tol())
                    .map(|f| f.body)
                    .map_err(|e| format!("{:?}", e.error).chars().take(160).collect());
                let (fail, line) = judge(&format!("chamfer s={s} {name} g={g}"), depth, out);
                println!("{line}");
                if fail {
                    bad.push(line);
                }
            }
        }
    }
    assert!(bad.is_empty(), "built through interference: {bad:#?}");
}

/// Distance from `x` to the polytope `{n_i . p <= h_i}` (Dykstra).
fn dist_polytope(x: [f64; 3], hs: &[([f64; 3], f64)]) -> f64 {
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let mut p = x;
    let mut inc = vec![[0.0; 3]; hs.len()];
    for _ in 0..4000 {
        for (i, (n, h)) in hs.iter().enumerate() {
            let y = [p[0] + inc[i][0], p[1] + inc[i][1], p[2] + inc[i][2]];
            let v = dot(*n, y) - h;
            let q = if v > 0.0 {
                [y[0] - v * n[0], y[1] - v * n[1], y[2] - v * n[2]]
            } else {
                y
            };
            inc[i] = [y[0] - q[0], y[1] - q[1], y[2] - q[2]];
            p = q;
        }
    }
    let dd = [x[0] - p[0], x[1] - p[1], x[2] - p[2]];
    dot(dd, dd).sqrt()
}

/// OBLIQUE corners: the skewed (60 deg) vented cavity's twelve edges at
/// `r`, a ball island (second solid) near its acute and obtuse corners.
/// The filleted cavity is the opening of the prism by a ball of radius
/// `r`, so the island reaches the material by `dist(c, P - r) + s - r`.
#[test]
fn oblique_ball_island_sweep() {
    let r = 0.2;
    let theta = 60f64.to_radians();
    let (body0, _) = crate::common::cavity::skewed_cavity_edges(theta, 1.0);
    // the cavity prism: base quad from (1,1), sides 1.2, skew theta, z in [1,3]
    let (dx, dy) = (1.2 * theta.cos(), 1.2 * theta.sin());
    let n_side = [-theta.sin(), theta.cos(), 0.0]; // outward of the left side (through (1,1))
    let hs_raw = vec![
        ([0.0, 0.0, -1.0], -1.0),
        ([0.0, 0.0, 1.0], 3.0),
        ([0.0, -1.0, 0.0], -1.0),
        ([0.0, 1.0, 0.0], 1.0 + dy),
        (n_side, n_side[0] * 1.0 + n_side[1] * 1.0),
        (
            [-n_side[0], -n_side[1], 0.0],
            -(n_side[0] * 2.2 + n_side[1] * 1.0),
        ),
    ];
    let shrunk: Vec<([f64; 3], f64)> = hs_raw.iter().map(|(n, h)| (*n, h - r)).collect();
    let wall: Vec<([f64; 3], f64)> = hs_raw.clone();
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let mut bad = Vec::new();
    // acute corner at (1,1); obtuse at (2.2,1); bisector directions
    let acute = [(0.5 * theta).cos(), (0.5 * theta).sin()];
    let obt = [-(0.5 * theta).sin(), (0.5 * theta).cos()];
    let obt = [obt[0].mul_add(1.0, 0.0), obt[1]];
    let _ = dx;
    for &s in &[0.05, 0.1] {
        for (name, base, dir, zf) in [
            ("acute-edge", [1.0, 1.0], acute, 2.0),
            ("acute-corner", [1.0, 1.0], acute, -1.0),
            (
                "obtuse-edge",
                [2.2, 1.0],
                [
                    -(0.5 * (std::f64::consts::PI - theta)).cos(),
                    (0.5 * (std::f64::consts::PI - theta)).sin(),
                ],
                2.0,
            ),
            (
                "obtuse-corner",
                [2.2, 1.0],
                [
                    -(0.5 * (std::f64::consts::PI - theta)).cos(),
                    (0.5 * (std::f64::consts::PI - theta)).sin(),
                ],
                -1.0,
            ),
        ] {
            let _ = obt;
            for &t in &[0.12, 0.18, 0.24, 0.3, 0.36, 0.42, 0.5, 0.6, 0.8] {
                let z = if zf > 0.0 { zf } else { 1.0 + s + 0.4 * t };
                let c = [base[0] + dir[0] * t, base[1] + dir[1] * t, z];
                // inside the cavity with room
                if wall.iter().any(|(n, h)| dot(*n, c) - h > -(s + 0.003)) {
                    continue;
                }
                let depth = dist_polytope(c, &shrunk) + s - r;
                let ball = ball_poled_z(s, Vec3::new(c[0], c[1], c[2]), tol());
                let body = fuse(&body0, &ball);
                let edges = edges_with_corners(&body, |q| {
                    ((q.z - 1.0).abs() < 1e-9 || (q.z - 3.0).abs() < 1e-9)
                        && [
                            (1.0, 1.0),
                            (2.2, 1.0),
                            (2.2 + dx, 1.0 + dy),
                            (1.0 + dx, 1.0 + dy),
                        ]
                        .iter()
                        .any(|(x, y)| (q.x - x).abs() < 1e-9 && (q.y - y).abs() < 1e-9)
                });
                assert_eq!(edges.len(), 12);
                let out = fillet_edges(&body, &edges, r, tol())
                    .map(|f| f.body)
                    .map_err(|e| format!("{:?}", e.error).chars().take(160).collect());
                let (fail, line) = judge(&format!("oblique s={s} {name} t={t}"), depth, out);
                println!("{line}");
                if fail {
                    bad.push(line);
                }
            }
        }
    }
    assert!(bad.is_empty(), "built through interference: {bad:#?}");
}

/// COST: a stepped revolve (a pagoda) with `k` steps; every step rim is
/// requested (2k-1 closed rims). Time `band_reach` at f64 and Interval.
#[test]
fn cost_scaling_pagoda() {
    use crate::common::interval::iv;
    use geom_core::{Band, Interval, Point2};
    use sweep::Revolution;
    use sweep::blend::BlendRequest;
    use sweep::test_support::{band_reach, revolved_about_y_at, rim_arcs_at};
    let band = Band::linear(tol()).unwrap();
    for k in [1usize, 2, 4, 8] {
        let rho = |i: usize| 1.0 + (k - i) as f64 * 0.5;
        let mut pts = vec![(0.0, 0.0)];
        for i in 0..k {
            pts.push((rho(i), i as f64));
            pts.push((rho(i), (i + 1) as f64));
        }
        pts.push((0.0, k as f64));
        let mut rims = Vec::new();
        for i in 0..k {
            rims.push((rho(i), (i + 1) as f64));
            if i + 1 < k {
                rims.push((rho(i + 1), (i + 1) as f64));
            }
        }
        let f_pts: Vec<(Point2<f64>, f64)> =
            pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
        let body = revolved_about_y_at::<f64>(f_pts, Revolution::Full, tol());
        let edges: Vec<_> = rims
            .iter()
            .flat_map(|&(r, y)| rim_arcs_at(&body, r, y))
            .collect();
        let faces = body.faces().count();
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: 0.1,
        };
        let t = std::time::Instant::now();
        let v = band_reach(&req, band);
        let f64_ms = t.elapsed().as_secs_f64() * 1e3;
        let i_pts: Vec<(Point2<Interval>, Interval)> = pts
            .iter()
            .map(|&(x, y)| (Point2::new(iv(x), iv(y)), iv(0.0)))
            .collect();
        let ibody = revolved_about_y_at::<Interval>(i_pts, Revolution::Full, tol());
        let iedges: Vec<_> = rims
            .iter()
            .flat_map(|&(r, y)| rim_arcs_at(&ibody, r, y))
            .collect();
        let ireq = BlendRequest {
            body: &ibody,
            edges: iedges,
            size: iv(0.1),
        };
        let t = std::time::Instant::now();
        let iv_out = band_reach(&ireq, band);
        let iv_ms = t.elapsed().as_secs_f64() * 1e3;
        println!(
            "COST k={k} links={} faces={faces}: f64 {f64_ms:.1} ms ({}) | Interval {iv_ms:.1} ms ({})",
            edges.len(),
            v.is_ok(),
            iv_out.is_ok()
        );
    }
}

/// COST in space (no sheet): a plate with `k` x `k` square pockets, the
/// plate's twelve outer edges filleted: every pocket face is metered on
/// cells against straight reaches.
#[test]
fn cost_scaling_pockets() {
    use geom_core::Band;
    use sweep::blend::BlendRequest;
    use sweep::test_support::band_reach;
    let band = Band::linear(tol()).unwrap();
    for k in [1usize, 2, 4, 6] {
        let size = 4.0;
        let mut body = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(size, size, 1.0));
        let pitch = (size - 0.4) / k as f64;
        for i in 0..k {
            for j in 0..k {
                let x0 = 0.2 + i as f64 * pitch + 0.05;
                let y0 = 0.2 + j as f64 * pitch + 0.05;
                let tool = brick(
                    Point3::new(x0, y0, 0.3),
                    Point3::new(x0 + pitch - 0.1, y0 + pitch - 0.1, 2.0),
                );
                body = cut("pocket", &body, &tool);
            }
        }
        let outer = |p: Point3<f64>| p.x.abs() < 1e-12 || (p.x - size).abs() < 1e-12;
        let edges = edges_with_corners(&body, |p| {
            outer(p) || ((p.y.abs() < 1e-12 || (p.y - size).abs() < 1e-12) && (p.z.abs() < 1e-12))
        });
        let faces = body.faces().count();
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: 0.1,
        };
        let t = std::time::Instant::now();
        let v = band_reach(&req, band);
        println!(
            "COSTP k={k} links={} faces={faces}: f64 {:.1} ms ({:?})",
            edges.len(),
            t.elapsed().as_secs_f64() * 1e3,
            v.err()
                .map(|e| format!("{e:?}").chars().take(80).collect::<String>())
        );
    }
}

/// CO-REQUEST: the sealed cavity's twelve (concave) and a box island's
/// twelve (convex) edges in ONE request at `r`. The rounded island
/// `[1+g,3-g]^3` opened by `r` never meets the rounded cavity for `g > 0`
/// (its eroded core lies inside the cavity's), so every gap must build.
#[test]
fn co_requested_rounded_island_in_rounded_cavity() {
    use geom_core::Band;
    use sweep::blend::BlendRequest;
    use sweep::test_support::band_reach;
    let r = 0.25;
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let cav = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let sealed = cut("cavity", &block, &cav);
    for g in [0.02, 0.05, 0.1, 0.2] {
        let (lo, hi) = (1.0 + g, 3.0 - g);
        let island = brick(Point3::new(lo, lo, lo), Point3::new(hi, hi, hi));
        let body = fuse(&sealed, &island);
        let isl = |p: Point3<f64>| {
            p.to_array()
                .iter()
                .all(|c| (c - lo).abs() < 1e-12 || (c - hi).abs() < 1e-12)
        };
        let mut edges = cavity_edges(&body);
        edges.extend(edges_with_corners(&body, isl));
        edges.sort_unstable();
        assert_eq!(edges.len(), 24);
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: r,
        };
        let meter = band_reach(&req, Band::linear(tol()).unwrap());
        let out = fillet_edges(&body, &edges, r, tol());
        println!(
            "COREQ g={g}: meter {:?} | fillet {}",
            meter
                .err()
                .map(|e| format!("{e:?}").chars().take(120).collect::<String>()),
            match out {
                Ok(f) => format!(
                    "BUILT geo_ok={} V={:?}",
                    validate_geometric(&f.body, tol()).is_ok(),
                    mass_properties(&f.body, tol()).map(|m| m.volume).ok()
                ),
                Err(e) => format!("refused {:?}", e.error).chars().take(160).collect(),
            }
        );
    }
}
