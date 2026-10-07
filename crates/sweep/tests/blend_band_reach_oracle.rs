//! **Predicate 2's reach against an analytic oracle**: a ball obstacle
//! beside a filleted (or chamfered) cavity or block, at the poses where
//! the oracle's depth crosses zero.
//!
//! Each filleted body here is the opening of a convex polytope `P` by a
//! ball of radius `r`, so the depth of a ball of radius `s` about `c` in
//! the band's material has a closed form, `dist(c, P ⊖ r) + s − r`. A
//! pose of positive depth must refuse, through the reach; a pose of
//! negative depth must build, tier-3 valid. The poses are the last that
//! refuse and the first that build along each approach, so a bound that
//! loosens past the band — a corner patch's ball widened, say — builds
//! through the obstacle and goes red here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use sweep::blend::{BlendError, chamfer_edges, fillet_edges};
use sweep::test_support::{ball_poled_z, finished};
use topo::{AtRestBody, Body, EdgeKey, validate_geometric};

use crate::common::cavity::{brick, cavity_edges, cut, edges_with_corners};

fn tol() -> Tol {
    Tol::witness()
}

/// The union of a finished `a` and the fixture `b`, finished.
fn fuse(a: &AtRestBody<f64>, b: &Body<f64>) -> AtRestBody<f64> {
    let t = tol();
    let b = sweep::test_support::finished("b", b.clone(), t);
    topo::union(a, &b, t)
        .expect("the union succeeds")
        .body()
        .expect("the union leaves material")
        .body
        .clone()
}

/// The oracle's verdict on one pose: a positive `depth` refuses through
/// the reach, a negative one builds, tier-3 valid.
fn judge(what: &str, depth: f64, out: Result<Body<f64>, BlendError>) {
    match out {
        Ok(b) => {
            assert!(
                depth < 0.0,
                "{what}: built {depth:+.4} into the band's material"
            );
            assert_eq!(validate_geometric(&b, tol()), Ok(()), "{what}: tier 3");
        }
        Err(e) => {
            assert!(depth > 0.0, "{what}: {depth:+.4} clear, refused {e:?}");
            assert!(
                matches!(e, BlendError::FaceClearance { .. }),
                "{what}: refused by the reach: {e:?}"
            );
        }
    }
}

/// The distance from `p` to the cube `[lo, hi]³`.
fn dist_box(p: [f64; 3], lo: f64, hi: f64) -> f64 {
    p.iter()
        .map(|c| (lo - c).max(c - hi).max(0.0).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// The sealed `[1, 3]³` cavity in `[0, 4]³`.
fn sealed_cavity() -> Body<f64> {
    cut(
        "cavity",
        &brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0)),
        &brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0)),
    )
}

/// **A concave cavity's twelve edges against a ball island** (a second
/// solid), at an edge, at a corner and beside a corner; `r = 0.25`. The
/// material added is the cavity outside `[1 + r, 3 − r]³ ⊕ r`.
#[test]
fn a_concave_cavity_meets_a_ball_island_where_the_oracle_says() {
    let r = 0.25;
    let sealed = finished("the sealed cavity", sealed_cavity(), tol());
    for (what, s, c) in [
        ("edge, refuses", 0.05, [1.1, 1.1, 2.0]),
        ("edge, builds", 0.05, [1.11, 1.11, 2.0]),
        ("corner, refuses", 0.05, [1.12, 1.12, 1.12]),
        ("corner, builds", 0.05, [1.14, 1.14, 1.14]),
        ("corner, refuses", 0.1, [1.16, 1.16, 1.16]),
        ("corner, builds", 0.1, [1.17, 1.17, 1.17]),
        ("near the corner, builds", 0.05, [1.12, 1.12, 1.2]),
    ] {
        let depth = dist_box(c, 1.0 + r, 3.0 - r) + s - r;
        let body = fuse(
            &sealed,
            &ball_poled_z(s, Vec3::new(c[0], c[1], c[2]), tol()),
        );
        let out = fillet_edges(&body, &cavity_edges(&body), r, tol());
        judge(
            &format!("concave s {s} {what}"),
            depth,
            out.map(|f| f.body).map_err(|e| e.error),
        );
    }
}

/// **A convex block's twelve outer edges against a ball void**, at an
/// edge and at a corner; `r = 0.5`. The material removed is the block
/// outside `[r, 4 − r]³ ⊕ r`.
#[test]
fn a_convex_block_meets_a_ball_void_where_the_oracle_says() {
    let r = 0.5;
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    for (what, s, c) in [
        ("edge, refuses", 0.1, [0.2, 0.2, 2.0]),
        ("edge, builds", 0.1, [0.22, 0.22, 2.0]),
        ("corner, refuses", 0.1, [0.25, 0.25, 0.25]),
        ("corner, builds", 0.1, [0.3, 0.3, 0.3]),
        ("corner, refuses", 0.2, [0.32, 0.32, 0.32]),
        ("corner, builds", 0.2, [0.35, 0.35, 0.35]),
    ] {
        let depth = dist_box(c, r, 4.0 - r) + s - r;
        let body = cut(
            "void",
            &block,
            &ball_poled_z(s, Vec3::new(c[0], c[1], c[2]), tol()),
        );
        let outer = |p: Point3<f64>| {
            p.to_array()
                .iter()
                .all(|c| c.abs() < 1e-12 || (c - 4.0).abs() < 1e-12)
        };
        let edges = edges_with_corners(&body, outer);
        assert_eq!(edges.len(), 12, "the block's outer edges");
        let out = fillet_edges(
            &sweep::test_support::at_rest(&body, tol()),
            &edges,
            r,
            tol(),
        );
        judge(
            &format!("convex s {s} {what}"),
            depth,
            out.map(|f| f.body).map_err(|e| e.error),
        );
    }
}

/// **A concave cavity's twelve chamfers against a ball island**: the
/// material added lies within `(x − 1) + (y − 1) < d` of each edge and
/// `Σ − 3 < 2d` of each corner (the patch plane through the three feet).
#[test]
fn a_chamfered_cavity_meets_a_ball_island_where_the_oracle_says() {
    let d = 0.25;
    let sealed = finished("the sealed cavity", sealed_cavity(), tol());
    for (what, s, c) in [
        ("edge, refuses", 0.1, [1.18, 1.18, 2.0]),
        ("edge, builds", 0.1, [1.2, 1.2, 2.0]),
        ("corner, refuses", 0.1, [1.22, 1.22, 1.22]),
        ("corner, builds", 0.1, [1.25, 1.25, 1.25]),
    ] {
        let u = c.map(|x| x - 1.0);
        let edge = [(0, 1), (0, 2), (1, 2)]
            .iter()
            .map(|&(i, j)| (d - u[i] - u[j]) / 2f64.sqrt())
            .fold(f64::NEG_INFINITY, f64::max);
        let corner = (2.0 * d - u[0] - u[1] - u[2]) / 3f64.sqrt();
        let depth = edge.max(corner) + s;
        let body = fuse(
            &sealed,
            &ball_poled_z(s, Vec3::new(c[0], c[1], c[2]), tol()),
        );
        let out = chamfer_edges(&body, &cavity_edges(&body), d, tol());
        judge(
            &format!("chamfer s {s} {what}"),
            depth,
            out.map(|f| f.body).map_err(|e| e.error),
        );
    }
}

/// The largest of `depth` over a ball, sampled on a grid and finely on
/// its sphere.
fn ball_depth(c: [f64; 3], s: f64, depth: impl Fn([f64; 3]) -> f64) -> f64 {
    let n = 20;
    let mut m = f64::NEG_INFINITY;
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                let d = [i, j, k].map(|t| -s + 2.0 * s * f64::from(t) / f64::from(n));
                if d.iter().map(|x| x * x).sum::<f64>() <= s * s {
                    m = m.max(depth([c[0] + d[0], c[1] + d[1], c[2] + d[2]]));
                }
            }
        }
    }
    for i in 0..200 {
        for j in 0..100 {
            let (a, b) = (
                std::f64::consts::TAU * f64::from(i) / 200.0,
                std::f64::consts::PI * f64::from(j) / 99.0,
            );
            m = m.max(depth([
                c[0] + s * b.sin() * a.cos(),
                c[1] + s * b.cos(),
                c[2] + s * b.sin() * a.sin(),
            ]));
        }
    }
    m
}

/// **A round void's concave floor rim against a ball island off its
/// axis**: the void `ρ ≤ 1.5`, `y ∈ [1, 3]` about `y`, its floor rim at
/// `r = 0.25`; the material added is the sheet corner `[1.25, 1.5] ×
/// [1, 1.25]` outside the circle about `(1.25, 1.25)`. The island moves
/// along the bisector at azimuth `2.0`.
#[test]
fn a_concave_floor_rim_meets_a_ball_island_where_the_oracle_says() {
    use sweep::Revolution;
    use sweep::test_support::{corners, revolved_about_y, rim_arcs_at};
    let r = 0.25;
    let block = brick(Point3::new(-3.0, 0.0, -3.0), Point3::new(3.0, 4.0, 3.0));
    let void = revolved_about_y(
        corners(&[(0.0, 1.0), (1.5, 1.0), (1.5, 3.0), (0.0, 3.0)]),
        Revolution::Full,
        tol(),
    );
    let sealed = finished("the sealed void", cut("void", &block, &void), tol());
    let depth = |p: [f64; 3]| {
        let (rho, y) = (p[0].hypot(p[2]), p[1]);
        let d = ((rho - 1.25).powi(2) + (y - 1.25).powi(2)).sqrt() - r;
        (rho - 1.25).min(1.25 - y).min(d)
    };
    let (s, phi) = (0.12, 2.0_f64);
    for (what, g) in [("refuses", 0.035), ("builds", 0.05)] {
        let (rho, y) = (1.5 - s - g, 1.0 + s + g);
        let c = [rho * phi.cos(), y, rho * phi.sin()];
        let body = fuse(
            &sealed,
            &ball_poled_z(s, Vec3::new(c[0], c[1], c[2]), tol()),
        );
        let out = fillet_edges(&body, &rim_arcs_at(&body, 1.5, 1.0), r, tol());
        judge(
            &format!("floor rim s {s} {what}"),
            ball_depth(c, s, depth),
            out.map(|f| f.body).map_err(|e| e.error),
        );
    }
}

/// The distance from `x` to the polytope `{n_i · p ≤ h_i}` (Dykstra's
/// alternating projections).
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

/// **An oblique cavity's corners**: the vented cavity skewed `60°`, its
/// twelve edges at `r = 0.2`, a ball island moving into its acute and
/// its obtuse corner. There the ball's centre projects behind the
/// vertex along one edge, so the patch's band-end planes are taken only
/// where it lies ahead; the acute corner's patch reaches farther.
#[test]
fn an_oblique_cavity_s_corners_meet_a_ball_island_where_the_oracle_says() {
    let r = 0.2;
    let theta = 60f64.to_radians();
    let (body0, _) = crate::common::cavity::skewed_cavity_edges(theta, 1.0);
    let body0 = finished("the skewed cavity", body0, tol());
    let (dx, dy) = (1.2 * theta.cos(), 1.2 * theta.sin());
    let n_side = [-theta.sin(), theta.cos(), 0.0];
    let walls = [
        ([0.0, 0.0, -1.0], -1.0),
        ([0.0, 0.0, 1.0], 3.0),
        ([0.0, -1.0, 0.0], -1.0),
        ([0.0, 1.0, 0.0], 1.0 + dy),
        (n_side, n_side[0] + n_side[1]),
        (
            [-n_side[0], -n_side[1], 0.0],
            -(n_side[0] * 2.2 + n_side[1]),
        ),
    ];
    let shrunk: Vec<([f64; 3], f64)> = walls.iter().map(|(n, h)| (*n, h - r)).collect();
    let acute = [(0.5 * theta).cos(), (0.5 * theta).sin()];
    let half = 0.5 * (std::f64::consts::PI - theta);
    let obtuse = [-half.cos(), half.sin()];
    for (what, s, base, dir, t) in [
        ("acute corner, refuses", 0.05, [1.0, 1.0], acute, 0.24),
        ("acute corner, builds", 0.05, [1.0, 1.0], acute, 0.3),
        ("obtuse corner, refuses", 0.05, [2.2, 1.0], obtuse, 0.12),
        ("obtuse corner, builds", 0.05, [2.2, 1.0], obtuse, 0.18),
        ("obtuse corner, refuses", 0.1, [2.2, 1.0], obtuse, 0.12),
    ] {
        let c = [
            base[0] + dir[0] * t,
            base[1] + dir[1] * t,
            1.0 + s + 0.4 * t,
        ];
        let depth = dist_polytope(c, &shrunk) + s - r;
        let body = fuse(&body0, &ball_poled_z(s, Vec3::new(c[0], c[1], c[2]), tol()));
        let cavity: Vec<EdgeKey> = edges_with_corners(&body, |q| {
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
        assert_eq!(cavity.len(), 12, "the oblique cavity's edges");
        let out = fillet_edges(&body, &cavity, r, tol());
        judge(
            &format!("oblique s {s} {what}"),
            depth,
            out.map(|f| f.body).map_err(|e| e.error),
        );
    }
}
