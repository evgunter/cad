//! **Review r1 of PR 4036's vertex-vertex probes**: corners of other
//! shapes than the L-prism's reflex corner (a 343° notch, a 203°
//! shallow reflex, a 90° and a 45° convex corner, and a 4-valent
//! pyramid apex) placed on a cube's edge and corner, over the
//! pierce-runs sweep's direction grid, near-band tilts off the
//! aligned frames, and a full turn of the four-crossing poses. Every
//! op in both orders prints one [`outcome`] line against a kernel-free
//! oracle clipping the shape's convex pieces by the cube's half-spaces.
//!
//! `cargo test -p sweep --release --test all join_vv_review_r1 --
//! --ignored --nocapture`, on two trees, and diff the lines.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use topo::test_support as fixtures;
use topo::{AtRestBody, Body, BooleanDeclarations, BooleanError, BooleanResult};

use crate::common::differential::outcome;

const SIDE: f64 = 4.0;

fn tol() -> Tol {
    Tol::witness()
}

fn unit(m: [f64; 3]) -> [f64; 3] {
    let n = m.iter().map(|c| c * c).sum::<f64>().sqrt();
    m.map(|c| c / n)
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// `join_pierce_runs_sweep`'s frame, copied.
fn frame(m: [f64; 3], psi: f64) -> [[f64; 3]; 3] {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u0 = unit(cross(seed, m));
    let w0 = cross(m, u0);
    let (c, s) = (psi.cos(), psi.sin());
    let u = [0, 1, 2].map(|i| c * u0[i] + s * w0[i]);
    let w = cross(m, u);
    [u, w, m]
}

fn cube_at(v: [f64; 3], f: [[f64; 3]; 3], lo: [f64; 3]) -> Body<f64> {
    let [u, w, m] = f;
    fixtures::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (lo[0] + SIDE * x, lo[1] + SIDE * y, lo[2] + SIDE * z);
            Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    )
}

fn cube_planes_at(v: [f64; 3], f: [[f64; 3]; 3], lo: [f64; 3]) -> Vec<([f64; 3], f64)> {
    let mut out = Vec::new();
    for (axis, &dir) in f.iter().enumerate() {
        let base = dot(dir, v);
        out.push((dir.map(|c| -c), -(base + lo[axis])));
        out.push((dir, base + lo[axis] + SIDE));
    }
    out
}

/// `join_pierce_runs_sweep`'s convex volume, copied.
fn convex_volume(planes: &[([f64; 3], f64)]) -> f64 {
    const EPS: f64 = 1e-9;
    let mut pts: Vec<[f64; 3]> = Vec::new();
    let n = planes.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (a, b, c) = (planes[i], planes[j], planes[k]);
                let det = dot(a.0, cross(b.0, c.0));
                if det.abs() < 1e-12 {
                    continue;
                }
                let bc = cross(b.0, c.0);
                let ca = cross(c.0, a.0);
                let ab = cross(a.0, b.0);
                let p = [0, 1, 2].map(|t| (a.1 * bc[t] + b.1 * ca[t] + c.1 * ab[t]) / det);
                if planes.iter().all(|&(nn, d)| dot(nn, p) <= d + EPS)
                    && !pts
                        .iter()
                        .any(|q| (0..3).all(|t| (q[t] - p[t]).abs() < EPS))
                {
                    pts.push(p);
                }
            }
        }
    }
    if pts.len() < 4 {
        return 0.0;
    }
    let inner = [0, 1, 2].map(|t| pts.iter().map(|p| p[t]).sum::<f64>() / pts.len() as f64);
    let mut vol = 0.0;
    for &(nn, d) in planes {
        let on: Vec<[f64; 3]> = pts
            .iter()
            .copied()
            .filter(|&p| (dot(nn, p) - d).abs() < EPS)
            .collect();
        if on.len() < 3 {
            continue;
        }
        let c = [0, 1, 2].map(|t| on.iter().map(|p| p[t]).sum::<f64>() / on.len() as f64);
        let e1 = unit([0, 1, 2].map(|t| on[0][t] - c[t]));
        let e2 = cross(unit(nn), e1);
        let mut ring: Vec<(f64, [f64; 3])> = on
            .iter()
            .map(|&p| {
                let r = [0, 1, 2].map(|t| p[t] - c[t]);
                (dot(r, e2).atan2(dot(r, e1)), p)
            })
            .collect();
        ring.sort_by(|a, b| a.0.total_cmp(&b.0));
        let area: f64 = (0..ring.len())
            .map(|i| {
                let (p, q) = (ring[i].1, ring[(i + 1) % ring.len()].1);
                let r1 = [0, 1, 2].map(|t| p[t] - c[t]);
                let r2 = [0, 1, 2].map(|t| q[t] - c[t]);
                dot(cross(r1, r2), unit(nn)) / 2.0
            })
            .sum();
        let h = d / dot(nn, nn).sqrt() - dot(unit(nn), inner);
        vol += area.abs() * h / 3.0;
    }
    vol
}

/// A shape: its body, its corner, its convex pieces by half-spaces.
pub struct Shape {
    pub name: &'static str,
    pub body: AtRestBody<f64>,
    pub v: [f64; 3],
    pub pieces: Vec<Vec<([f64; 3], f64)>>,
}

/// A convex CCW polygon's half-spaces, extruded over z ∈ [0, 1].
fn prism_piece(poly: &[(f64, f64)]) -> Vec<([f64; 3], f64)> {
    let n = poly.len();
    let mut planes: Vec<([f64; 3], f64)> = (0..n)
        .map(|i| {
            let (p, q) = (poly[i], poly[(i + 1) % n]);
            let out = [q.1 - p.1, p.0 - q.0, 0.0];
            (out, out[0] * p.0 + out[1] * p.1)
        })
        .collect();
    planes.push(([0.0, 0.0, 1.0], 1.0));
    planes.push(([0.0, 0.0, -1.0], 0.0));
    planes
}

fn finished(what: &str, body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol())
        .unwrap_or_else(|e| panic!("{what} is not a finished body: {e:?}"))
}

fn prism_shape(
    name: &'static str,
    profile: &[(f64, f64)],
    pieces: &[&[(f64, f64)]],
) -> Shape {
    Shape {
        name,
        body: finished(name, fixtures::prism::<f64>(profile, 1.0, tol()).body),
        v: [1.0, 1.0, 1.0],
        pieces: pieces.iter().map(|p| prism_piece(p)).collect(),
    }
}

/// The square pyramid apex `(0, 0, 1)` over `|x|, |y| ≤ 1` at z = 0,
/// built as the intersection of two wedges whose ridges cross there;
/// `None` when the kernel refuses to build it.
fn pyramid() -> Option<Shape> {
    let mk = |profile: &[(f64, f64)], ext: (f64, f64), map: fn(f64, f64, f64) -> Point3<f64>| {
        let mut b = Body::<f64>::new();
        fixtures::prism_ops(
            &mut b,
            profile,
            ext,
            map,
            fixtures::FaceGeometry::Certified,
            tol(),
        );
        fixtures::describe_as_intersections(&mut b, tol());
        finished("a wedge", b)
    };
    // Wedge 1: triangle in (x, z), extruded along y ∈ [−1.2, 1.2].
    let w1 = mk(
        &[(-1.0, 0.0), (1.0, 0.0), (0.0, 1.0)],
        (-1.2, 1.2),
        |x, y, z| Point3::new(x, -z, y),
    );
    // Wedge 2: triangle in (y, z) down to z = −0.5, along x ∈ [−1.3, 1.3].
    let w2 = mk(
        &[(-1.5, -0.5), (1.5, -0.5), (0.0, 1.0)],
        (-1.3, 1.3),
        |x, y, z| Point3::new(z, x, y),
    );
    let r = topo::intersect(&w1, &w2, tol());
    let body = match r {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => {
            println!("pyramid: did not build: {:?}", other.err());
            return None;
        }
    };
    let s = 2f64.sqrt();
    let pieces = vec![vec![
        ([1.0 / s, 0.0, 1.0 / s], 1.0 / s),
        ([-1.0 / s, 0.0, 1.0 / s], 1.0 / s),
        ([0.0, 1.0 / s, 1.0 / s], 1.0 / s),
        ([0.0, -1.0 / s, 1.0 / s], 1.0 / s),
        ([0.0, 0.0, -1.0], 0.0),
    ]];
    let v = convex_volume(&pieces[0]);
    let got = topo::mass_properties(&body, tol()).unwrap().volume;
    println!("pyramid: volume {got} oracle {v}");
    Some(Shape {
        name: "pyr4",
        body,
        v: [0.0, 0.0, 1.0],
        pieces,
    })
}

pub fn shapes() -> Vec<Shape> {
    let mut out = vec![
        prism_shape(
            "L270",
            &[
                (0.0, 0.0),
                (2.0, 0.0),
                (2.0, 1.0),
                (1.0, 1.0),
                (1.0, 2.0),
                (0.0, 2.0),
            ],
            &[
                &[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
                &[(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
            ],
        ),
        prism_shape(
            "notch343",
            &[
                (0.0, 0.0),
                (2.0, 0.0),
                (2.0, 0.85),
                (1.0, 1.0),
                (2.0, 1.15),
                (2.0, 2.0),
                (0.0, 2.0),
            ],
            &[
                &[(0.0, 0.0), (2.0, 0.0), (2.0, 0.85), (1.0, 1.0), (0.0, 1.0)],
                &[(0.0, 1.0), (1.0, 1.0), (2.0, 1.15), (2.0, 2.0), (0.0, 2.0)],
            ],
        ),
        prism_shape(
            "shallow203",
            &[(0.0, 0.0), (2.0, 0.0), (2.0, 1.2), (1.0, 1.0), (0.0, 1.2)],
            &[
                &[(0.0, 0.0), (2.0, 0.0), (2.0, 1.2), (1.0, 1.0)],
                &[(0.0, 0.0), (1.0, 1.0), (0.0, 1.2)],
            ],
        ),
        prism_shape(
            "convex90",
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            &[&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        ),
        prism_shape(
            "acute45",
            &[(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            &[&[(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        ),
    ];
    out.extend(pyramid());
    out
}

type Op = fn(
    &AtRestBody<f64>,
    &AtRestBody<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

/// Every op in both orders of `shape` against the cube in frame `f`
/// at placement `lo`.
pub fn runs(
    shape: &Shape,
    f: [[f64; 3]; 3],
    lo: [f64; 3],
) -> Vec<(String, Option<Result<BooleanResult<f64>, BooleanError>>, f64)> {
    let va: f64 = shape.pieces.iter().map(|p| convex_volume(p)).sum();
    let vb = SIDE * SIDE * SIDE;
    let planes = cube_planes_at(shape.v, f, lo);
    let common: f64 = shape
        .pieces
        .iter()
        .map(|p| {
            let mut all = p.clone();
            all.extend(planes.iter().copied());
            convex_volume(&all)
        })
        .sum();
    let b = finished("the cube", cube_at(shape.v, f, lo));
    let decls = BooleanDeclarations::default();
    let mut out = Vec::new();
    for (order, x, y, vx) in [("pc", &shape.body, &b, va), ("cp", &b, &shape.body, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vx - common),
        ];
        for (op, run, want) in ops {
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                run(x, y, &decls, tol())
            }));
            out.push((format!("{order} {op}"), r.ok(), want));
        }
    }
    out
}

const PLACEMENTS: [(&str, [f64; 3]); 2] = [("edge", [0.0, -2.0, 0.0]), ("corner", [0.0, 0.0, 0.0])];

fn direction(i: u32, j: u32) -> [f64; 3] {
    let theta = std::f64::consts::TAU * (f64::from(i) + 0.37) / 12.0;
    let phi = (f64::from(j) - 3.0) * 0.4 + 0.05;
    [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()]
}

fn emit(shape: &Shape, tag: &str, f: [[f64; 3]; 3], lo: [f64; 3]) {
    for (op, r, want) in runs(shape, f, lo) {
        let line = r.map_or_else(|| "PANIC".to_owned(), |r| outcome(r, want, tol()));
        println!("{} {tag} {op}: {line}", shape.name);
    }
}

/// Shapes × placements × the 12 × 7 direction grid × four turns.
/// Shard with `VV_SHAPE=<name>`.
#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn join_vv_review_r1_grid_battery() {
    let only = std::env::var("VV_SHAPE").ok();
    for shape in shapes() {
        if only.as_deref().is_some_and(|o| o != shape.name) {
            continue;
        }
        for (place, lo) in PLACEMENTS {
            for i in 0..12 {
                for j in 0..7 {
                    for psi in [0.0, 1.0, 2.2, 4.0] {
                        let f = frame(direction(i, j), psi);
                        emit(&shape, &format!("{place} i={i} j={j} psi={psi}"), f, lo);
                    }
                }
            }
        }
    }
}

/// Near-band tilts: the cube's frame aligned with the shape's axes
/// (its face normal along ±x, ±y, ±z or a diagonal, turned 0 or 45°),
/// tilted by `t ∈ 1e-2 … 1e-8` about two axes, both signs.
#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn join_vv_review_r1_tilt_battery() {
    let only = std::env::var("VV_SHAPE").ok();
    let bases: [[f64; 3]; 8] = [
        [0.0, 0.0, 1.0],
        [0.0, 0.0, -1.0],
        [1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, -1.0, 0.0],
        [1.0, 1.0, 0.0],
        [-1.0, -1.0, -1.0],
    ];
    let perturb: [[f64; 3]; 2] = [[0.3, 0.7, 0.2], [-0.6, 0.1, 0.8]];
    for shape in shapes() {
        if only.as_deref().is_some_and(|o| o != shape.name) {
            continue;
        }
        for (place, lo) in PLACEMENTS {
            for (bi, base) in bases.iter().enumerate() {
                for psi in [0.0, std::f64::consts::FRAC_PI_4] {
                    for (pi, p) in perturb.iter().enumerate() {
                        for e in 2..=8 {
                            for sign in [1.0, -1.0] {
                                let t = sign * 10f64.powi(-e);
                                let m = [0, 1, 2].map(|k| unit(*base)[k] + t * p[k]);
                                let f = frame(m, psi + t);
                                emit(
                                    &shape,
                                    &format!("{place} base={bi} psi={psi:.3} p={pi} t={t:e}"),
                                    f,
                                    lo,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A full turn of the four-crossing poses: the PR's three
/// `four_germ_vertex_pairs_build_every_op` directions (and the theta
/// grid at j = 0), the cube turned about its own normal in 5° steps.
#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn join_vv_review_r1_turn_battery() {
    let only = std::env::var("VV_SHAPE").ok();
    for shape in shapes() {
        if only.as_deref().is_some_and(|o| o != shape.name) {
            continue;
        }
        for (place, lo) in PLACEMENTS {
            for i in [0, 2, 6] {
                for k in 0..72 {
                    let psi = f64::from(k) * 5f64.to_radians();
                    let f = frame(direction(i, 0), psi);
                    emit(&shape, &format!("{place} i={i} j=0 turn={k}"), f, lo);
                }
            }
        }
    }
}

/// The frame whose edge `which` (0: `u`, 1: `w`) runs horizontally at
/// azimuth `theta`, the cube turned `alpha` about that edge.
fn edge_frame(which: usize, theta: f64, alpha: f64) -> [[f64; 3]; 3] {
    let e = [theta.cos(), theta.sin(), 0.0];
    let side = [-theta.sin(), theta.cos(), 0.0];
    let up = [0.0, 0.0, 1.0];
    let o = [0, 1, 2].map(|k| alpha.cos() * side[k] + alpha.sin() * up[k]);
    if which == 0 {
        // u = e, w = o, m = u × w.
        [e, o, cross(e, o)]
    } else {
        // w = e, m = o... u = w × m.
        [cross(e, o), e, o]
    }
}

/// **Germs that tie along one direction**: a cube edge lying exactly
/// in the shape's top face plane through its corner, at every 15° of
/// azimuth and six turns about that edge; the corner placement lays
/// `u` there, the edge placement `w`.
#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn join_vv_review_r1_tie_battery() {
    let only = std::env::var("VV_SHAPE").ok();
    for shape in shapes() {
        if only.as_deref().is_some_and(|o| o != shape.name) {
            continue;
        }
        for (pi, (place, lo)) in PLACEMENTS.into_iter().enumerate() {
            for k in 0..24 {
                let theta = f64::from(k) * 15f64.to_radians() + 0.013;
                for alpha in [-2.5, -1.9, -0.8, 0.6, 1.3, 2.4] {
                    let f = if pi == 0 { edge_frame(1, theta, alpha) } else { edge_frame(0, theta, alpha) };
                    emit(&shape, &format!("{place} theta={k} alpha={alpha}"), f, lo);
                }
            }
        }
    }
}

/// Rotation of `p` about the unit `axis` by `ang`.
fn rotate(p: [f64; 3], axis: [f64; 3], ang: f64) -> [f64; 3] {
    let (c, s) = (ang.cos(), ang.sin());
    let kxp = cross(axis, p);
    let kdp = dot(axis, p);
    [0, 1, 2].map(|t| p[t] * c + kxp[t] * s + axis[t] * kdp * (1.0 - c))
}

/// **Six crossings**: the cube's corner octant set against the shape's
/// own corner octant `(−x, −y, −z)` reversed, turned about a diagonal
/// in 5° steps; at 60° two convex corners' links form a hexagram.
#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn join_vv_review_r1_hex_battery() {
    let only = std::env::var("VV_SHAPE").ok();
    let base = [[0.0, -1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, -1.0]];
    let axes = [
        unit([1.0, 1.0, 1.0]),
        unit([1.0, 1.0, 0.6]),
        unit([0.8, 1.1, 1.0]),
    ];
    for shape in shapes() {
        if only.as_deref().is_some_and(|o| o != shape.name) {
            continue;
        }
        for (ai, axis) in axes.iter().enumerate() {
            for k in 0..72 {
                let ang = f64::from(k) * 5f64.to_radians() + 0.0007;
                let f = base.map(|d| rotate(d, *axis, ang));
                emit(&shape, &format!("corner hex axis={ai} k={k}"), f, [0.0, 0.0, 0.0]);
            }
        }
    }
}
