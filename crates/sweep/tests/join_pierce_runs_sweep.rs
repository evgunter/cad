//! **The pierce-runs sweep**: the L-prism's reflex top corner
//! `v = (1, 1, 1)` on a cube of side 4, placed so that `v` lies on the
//! cube's near face (`face`), on one of its edges (`edge`) or at one of
//! its corners (`corner`), the cube turned over a grid of directions.
//! Depending on the direction the corner has zero, one or two Out runs
//! against the face, and the edge and corner placements reach the
//! same corner through the vertex-edge and vertex-vertex lanes. Every
//! op in both operand orders prints one [`outcome`] line, against an
//! oracle that clips each of the prism's two boxes by the cube's six
//! half-spaces (`convex_volume`), independent of the kernel.
//!
//! `cargo test -p sweep --release --test all pierce_runs_battery --
//! --ignored --nocapture`, on two trees, and diff the lines.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use topo::test_support as fixtures;
use topo::{
    AtRestBody, Body, BooleanDeclarations, BooleanError, BooleanResult, LoopBoundary,
    mass_properties,
};

use crate::common::differential::outcome;

const PROFILE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (2.0, 0.0),
    (2.0, 1.0),
    (1.0, 1.0),
    (1.0, 2.0),
    (0.0, 2.0),
];
const V: [f64; 3] = [1.0, 1.0, 1.0];
const SIDE: f64 = 4.0;
/// The prism as two boxes with disjoint interiors, `(lo, hi)`.
const BOXES: [([f64; 3], [f64; 3]); 2] = [
    ([0.0, 0.0, 0.0], [2.0, 1.0, 1.0]),
    ([0.0, 1.0, 0.0], [1.0, 2.0, 1.0]),
];

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

/// The right-handed frame `(u, w, m)` with `m` the unit of `m` and `u`
/// turned by `psi` about it.
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

/// The cube `v + a·u + b·w + c·m`, `(a, b, c)` over `lo + [0, SIDE]³`:
/// `lo = (−2, −2, 0)` puts `v` inside the near face, `(0, −2, 0)` on
/// its edge along `w`, `(0, 0, 0)` at its corner.
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

/// [`cube_at`] the L-prism's corner `V`.
fn cube(f: [[f64; 3]; 3], lo: [f64; 3]) -> Body<f64> {
    cube_at(V, f, lo)
}

/// [`cube_at`]'s six half-spaces `n·x ≤ d`.
fn cube_planes_at(v: [f64; 3], f: [[f64; 3]; 3], lo: [f64; 3]) -> Vec<([f64; 3], f64)> {
    let mut out = Vec::new();
    for (axis, &dir) in f.iter().enumerate() {
        let base = dot(dir, v);
        out.push((dir.map(|c| -c), -(base + lo[axis])));
        out.push((dir, base + lo[axis] + SIDE));
    }
    out
}

/// [`cube_planes_at`] the L-prism's corner `V`.
fn cube_planes(f: [[f64; 3]; 3], lo: [f64; 3]) -> Vec<([f64; 3], f64)> {
    cube_planes_at(V, f, lo)
}

/// The volume of `{x : n·x ≤ d for every plane}`, bounded and convex:
/// its vertices are the planes' feasible triple meets, each face is
/// the vertices on its plane fanned about their centroid, and the
/// volume sums the pyramids over the faces from an interior point.
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

/// The prism's volume inside the cube.
fn shared(planes: &[([f64; 3], f64)]) -> f64 {
    BOXES
        .iter()
        .map(|&(lo, hi)| {
            let mut all = planes.to_vec();
            for t in 0..3 {
                let mut e = [0.0; 3];
                e[t] = 1.0;
                all.push((e, hi[t]));
                all.push((e.map(|c| -c), -lo[t]));
            }
            convex_volume(&all)
        })
        .sum()
}

/// `body` as a boolean operand: the at-rest gate's finished body.
fn finished(what: &str, body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol())
        .unwrap_or_else(|e| panic!("{what} is not a finished body: {e:?}"))
}

type Op = fn(
    &AtRestBody<f64>,
    &AtRestBody<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

/// The placements: `v` inside the near face, on its edge along `w`
/// (four turns about the face normal), at its corner (four turns).
const PLACEMENTS: [(&str, [f64; 3], &[f64]); 3] = [
    ("face", [-2.0, -2.0, 0.0], &[0.0]),
    ("edge", [0.0, -2.0, 0.0], &[0.0, 1.0, 2.2, 4.0]),
    ("corner", [0.0, 0.0, 0.0], &[0.0, 1.0, 2.2, 4.0]),
];

/// The grid's direction `(i, j)`: 12 turns about z by 7 elevations.
fn direction(i: u32, j: u32) -> [f64; 3] {
    let theta = std::f64::consts::TAU * (f64::from(i) + 0.37) / 12.0;
    let phi = (f64::from(j) - 3.0) * 0.4 + 0.05;
    [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()]
}

/// Every op in both orders at one pose: `(tag, result, want)`.
fn pose_runs(
    (place, lo): (&str, [f64; 3]),
    (i, j, psi): (u32, u32, f64),
) -> Vec<(String, Result<BooleanResult<f64>, BooleanError>, f64)> {
    let prism = finished(
        "the prism",
        fixtures::prism::<f64>(&PROFILE, 1.0, tol()).body,
    );
    let va = mass_properties(&prism, tol()).unwrap().volume;
    let vb = SIDE * SIDE * SIDE;
    let decls = BooleanDeclarations::default();
    let f = frame(direction(i, j), psi);
    let b = finished("the cube", cube(f, lo));
    let common = shared(&cube_planes(f, lo));
    let mut out = Vec::new();
    for (order, x, y, vx) in [("pc", &prism, &b, va), ("cp", &b, &prism, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vx - common),
        ];
        for (op, run, want) in ops {
            let tag = format!("{place} i={i} j={j} psi={psi} {order} {op}");
            out.push((tag, run(x, y, &decls, tol()), want));
        }
    }
    out
}

#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn pierce_runs_battery() {
    for (place, lo, psis) in PLACEMENTS {
        for i in 0..12 {
            for j in 0..7 {
                for &psi in psis {
                    for (tag, r, want) in pose_runs((place, lo), (i, j, psi)) {
                        println!("{tag}: {}", outcome(r, want, tol()));
                    }
                }
            }
        }
    }
}

/// Where the pierce point `v` holds several vertices in a built body:
/// none, if they share one point and no face runs through two of them
/// (a face meeting two is where the pierce weld joins them), else the
/// finding.
fn pierce_point_finding(body: &Body<f64>, at: [f64; 3]) -> Option<String> {
    let at_v: Vec<_> = body
        .vertex_points()
        .filter(|(_, p)| [p.x, p.y, p.z] == at)
        .map(|(k, _)| k)
        .collect();
    let point = |k| body.get_vertex(k).unwrap().point;
    if at_v.iter().any(|&k| point(k) != point(at_v[0])) {
        return Some(format!(
            "the vertices at v do not share one point: {at_v:?}"
        ));
    }
    for (face, f) in body.faces() {
        let mut met = Vec::new();
        for &l in std::iter::once(&f.outer).chain(&f.rings) {
            if let LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary {
                for he in body.loop_cycle(first).unwrap() {
                    let v = body.get_half_edge(he).unwrap().start;
                    if at_v.contains(&v) && !met.contains(&v) {
                        met.push(v);
                    }
                }
            }
        }
        if met.len() > 1 {
            return Some(format!("face {face:?} runs through two vertices at v"));
        }
    }
    None
}

/// **The sweep's guard**, over a committed subset of
/// [`pierce_runs_battery`]: every direction of the face placement (the
/// pierce lane, where the two-run families live) and every third
/// direction of the edge and corner placements at their first turn. No
/// pose ships a body that is not `SOUND` by [`outcome`], and in the
/// face placement every body holds `v` as one vertex wherever a face
/// meets it. In the face placement refusals pass: the residue is the
/// filed rows'. The edge and corner placements reach `v` through the
/// vertex-vertex lane, where every run builds `SOUND`.
#[test]
fn the_sweep_subset_ships_no_bad_body() {
    let mut bad = Vec::new();
    for (place, lo, psis) in PLACEMENTS {
        let every = if place == "face" { 1 } else { 3 };
        for i in (0..12).step_by(every) {
            for j in 0..7 {
                for (tag, r, want) in pose_runs((place, lo), (i, j, psis[0])) {
                    let finding = match &r {
                        Ok(res) if place == "face" => {
                            res.body().and_then(|bb| pierce_point_finding(&bb.body, V))
                        }
                        _ => None,
                    };
                    let line = outcome(r, want, tol());
                    let sound = line.starts_with("OK SOUND") || line.starts_with("EMPTY ok");
                    if place != "face" && !sound
                        || line.starts_with("OK") && !line.starts_with("OK SOUND")
                        || line.starts_with("EMPTY WRONG")
                    {
                        bad.push(format!("{tag}: {line}"));
                    }
                    if let Some(finding) = finding {
                        bad.push(format!("{tag}: {finding}"));
                    }
                }
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} bad lines:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// **A reflex corner crossing a cube's edge or corner four times builds
/// every op**, in both operand orders, `SOUND` by [`outcome`] against
/// the clipping oracle. One pose per way the vertex-vertex lane used to
/// stop there (`boolean/insert.rs`):
/// - `corner i=0 j=0 psi=2.2`: two germs in one sector of each solid.
///   Red as `PairingMismatch` when they are ordered by the other solid's
///   sector rather than round their own. Union and difference are red
///   as `JoinDesync` or `Euler(SelfLoopEdge)` when the pairing starts
///   at A's first germ whatever the op keeps of A: a kept vertex of A
///   then holds both null edges.
/// - `edge i=2 j=0 psi=1`: a fan and a strut in the two entries of one
///   physical sector of the cube's edge vertex. Red as `JoinDesync`
///   "B senses agree" when the fan mints first and moves the half the
///   strut anchors on, and as a `ClassificationInvariant` when B runs a
///   null edge forward in A's order, the long way round.
/// - `edge i=6 j=0 psi=0`: `PairingMismatch` in both orders on the
///   other solid's sector order.
#[test]
fn four_germ_vertex_pairs_build_every_op() {
    let mut bad = Vec::new();
    for (place, pose) in [
        ("corner", (0, 0, 2.2)),
        ("edge", (2, 0, 1.0)),
        ("edge", (6, 0, 0.0)),
    ] {
        let lo = PLACEMENTS.iter().find(|p| p.0 == place).unwrap().1;
        for (tag, r, want) in pose_runs((place, lo), pose) {
            let line = outcome(r, want, tol());
            if !line.starts_with("OK SOUND") {
                bad.push(format!("{tag}: {line}"));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} runs not SOUND:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// **The F12 adjacency guard fires on six distinct germs.** A 343°
/// notch's reflex corner at `v` on the cube's edge (the grid's `i=3
/// j=0 psi=1`) crosses the cube six times. A's walk order pairs
/// `(0, 3) (2, 4) (5, 1)`, and B reads them at `(2, 5) (0, 1) (4, 3)` of
/// six: a nested, non-crossing matching, so `(0, 3)` is not adjacent in
/// B and every op in both orders refuses `PairingMismatch`
/// (`work/join/a-six-crossing-vertex-pair-nests-its-pairing-and-refuses-pairing-mismatch.md`).
/// Red when a pair not adjacent in B falls to the run-swallowing test
/// instead (the run's In and Out ends then meet at one vertex), and red,
/// as it should be, when the nested pairing builds.
#[test]
fn a_six_crossing_notch_corner_refuses_pairing_mismatch_on_distinct_germs() {
    let notch = [
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 0.85),
        (1.0, 1.0),
        (2.0, 1.15),
        (2.0, 2.0),
        (0.0, 2.0),
    ];
    let notch = finished("the notch", fixtures::prism::<f64>(&notch, 1.0, tol()).body);
    let cube = finished(
        "the cube",
        cube(frame(direction(3, 0), 1.0), PLACEMENTS[1].1),
    );
    let decls = BooleanDeclarations::default();
    for (order, x, y) in [("nc", &notch, &cube), ("cn", &cube, &notch)] {
        let ops: [(&str, Op); 3] = [
            ("U", topo::union_with),
            ("I", topo::intersect_with),
            ("S", topo::subtract_with),
        ];
        for (op, run) in ops {
            let r = run(x, y, &decls, tol());
            assert!(
                matches!(r, Err(BooleanError::PairingMismatch { .. })),
                "{order} {op}: {:?}",
                r.map(|_| ())
            );
        }
    }
}

/// The staircase prism: reflex corners at `(2, 1, 1)` and `(1, 2, 1)`,
/// translates of one L corner.
const STAIR: [(f64, f64); 8] = [
    (0.0, 0.0),
    (3.0, 0.0),
    (3.0, 1.0),
    (2.0, 1.0),
    (2.0, 2.0),
    (1.0, 2.0),
    (1.0, 3.0),
    (0.0, 3.0),
];
/// [`STAIR`] as three boxes with disjoint interiors, `(lo, hi)`.
const STAIR_BOXES: [([f64; 3], [f64; 3]); 3] = [
    ([0.0, 0.0, 0.0], [3.0, 1.0, 1.0]),
    ([0.0, 1.0, 0.0], [2.0, 2.0, 1.0]),
    ([0.0, 2.0, 0.0], [1.0, 3.0, 1.0]),
];

/// **Two pinches in one op are each crossed.** The cube's near face lies
/// in a plane through both of [`STAIR`]'s reflex top corners, turned so
/// that each corner has two Out runs (PR 4038's review r1, `u2 S_tt`,
/// direction 204 of 720). The intersection pinches at both corners, so
/// `cross_pinches` crosses two vertices in one op. Every op in both
/// orders builds `SOUND` at the boxes' clipped volume and holds each
/// corner as one vertex wherever a face meets it. Red if the pre-pass
/// stops after its first crossing: the second pinch's zip fuses it to
/// itself.
#[test]
fn two_pinches_in_one_op_are_each_crossed() {
    let (a, b) = ([2.0, 1.0, 1.0], [1.0, 2.0, 1.0]);
    let [e1, e2, _] = frame(unit([b[0] - a[0], b[1] - a[1], b[2] - a[2]]), 0.0);
    let t = std::f64::consts::TAU * (204.0 + 0.37) / 720.0;
    let m = [0, 1, 2].map(|k| t.cos() * e1[k] + t.sin() * e2[k]);
    let f = frame(m, 0.0);
    let lo = [-2.0, -2.0, 0.0];
    let mid = [1.5, 1.5, 1.0];
    let prism = finished(
        "the staircase",
        fixtures::prism::<f64>(&STAIR, 1.0, tol()).body,
    );
    let cube = finished("the cube", cube_at(mid, f, lo));
    let planes = cube_planes_at(mid, f, lo);
    let common: f64 = STAIR_BOXES
        .iter()
        .map(|&(blo, bhi)| {
            let mut all = planes.clone();
            for t in 0..3 {
                let mut e = [0.0; 3];
                e[t] = 1.0;
                all.push((e, bhi[t]));
                all.push((e.map(|c| -c), -blo[t]));
            }
            convex_volume(&all)
        })
        .sum();
    assert!(
        common > 1e-3,
        "the cube holds some of the staircase: {common}"
    );
    let (va, vb) = (6.0, SIDE * SIDE * SIDE);
    let decls = BooleanDeclarations::default();
    for (order, x, y, vx) in [("pc", &prism, &cube, va), ("cp", &cube, &prism, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vx - common),
        ];
        for (op, run, want) in ops {
            let r = run(x, y, &decls, tol());
            let findings: Vec<String> = match &r {
                Ok(res) => res
                    .body()
                    .map(|bb| {
                        [a, b]
                            .into_iter()
                            .filter_map(|p| pierce_point_finding(&bb.body, p))
                            .collect()
                    })
                    .unwrap_or_default(),
                Err(_) => Vec::new(),
            };
            let line = outcome(r, want, tol());
            assert!(line.starts_with("OK SOUND"), "{order} {op}: {line}");
            assert!(findings.is_empty(), "{order} {op}: {findings:?}");
        }
    }
}

/// **A pinch crossed on the second operand's side.** The L-prism's
/// bottom reflex corner `(1, 1, 0)` at a corner of the cube, the cube
/// along Fibonacci direction 19 of 120 and turned 1.9 about it (PR
/// 4038's review r2, `Lbot fib19 corner psi=1.9`). In cube ∪ prism the
/// collision's first operand has no face to cross, and the prism does:
/// `cross_pinches` splits the second operand's vertex, so every first
/// operand vertex that corresponded to it gains the new vertex. The union
/// builds `SOUND` with one vertex at the corner wherever a face meets
/// it. Red if the new vertex gains no correspondents: the zip finds no
/// ring half-edge for it.
#[test]
fn a_pinch_split_on_the_second_operands_side_builds() {
    let v = [1.0, 1.0, 0.0];
    let ga = std::f64::consts::PI * (3.0 - 5f64.sqrt());
    let z: f64 = 1.0 - 2.0 * (19.0 + 0.5) / 120.0;
    let r = (1.0 - z * z).sqrt();
    let m = [r * (ga * 19.0).cos(), r * (ga * 19.0).sin(), z];
    let f = frame(m, 1.9);
    let lo = [0.0, 0.0, 0.0];
    let prism = finished(
        "the prism",
        fixtures::prism::<f64>(&PROFILE, 1.0, tol()).body,
    );
    let cube = finished("the cube", cube_at(v, f, lo));
    let common = shared(&cube_planes_at(v, f, lo));
    let want = 3.0 + SIDE * SIDE * SIDE - common;
    let r = topo::union_with(&cube, &prism, &BooleanDeclarations::default(), tol());
    let finding = r
        .as_ref()
        .ok()
        .and_then(|res| res.body())
        .and_then(|bb| pierce_point_finding(&bb.body, v));
    let line = outcome(r, want, tol());
    assert!(line.starts_with("OK SOUND"), "cube ∪ prism: {line}");
    assert_eq!(finding, None, "cube ∪ prism");
}

/// **A vertex-vertex pinch the pairing start avoids.** With `v` on the
/// cube's edge (direction `i = 6, j = 1`), cube ∖ prism used to pinch at
/// `v` over two seams: each cube face through `v` passes it twice on its
/// outer loop, round a notch the prism cuts, so no kept face could cross
/// the pinch and the op refused `PinchUncrossed`
/// (`a-pinch-no-kept-face-can-cross-refuses`). The vertex pair crosses
/// four times, and its pairing starts where A's runs lie on the side the
/// op keeps of A (`insert::pairing_start_turns`): each run A keeps is a
/// copy of its own, and the result needs no crossing at `v`. Every op in
/// both orders builds `SOUND` at the clipping oracle. Red when the
/// pairing starts at A's first germ (`JoinDesync` "conflicting seam
/// vertex correspondence" in prism ∩ cube, the first run checked).
#[test]
fn a_four_germ_pinch_the_pairing_start_avoids_builds_every_op() {
    let (place, lo, psis) = PLACEMENTS[1];
    for (tag, r, want) in pose_runs((place, lo), (6, 1, psis[0])) {
        let line = outcome(r, want, tol());
        assert!(line.starts_with("OK SOUND"), "{tag}: {line}");
    }
}

/// Every op in both orders between `x` (volume `vx`) and `y` (`vy`),
/// which share `common`: `(tag, result, want)`.
fn every_op(
    (x, vx): (&AtRestBody<f64>, f64),
    (y, vy): (&AtRestBody<f64>, f64),
    common: f64,
) -> Vec<(String, Result<BooleanResult<f64>, BooleanError>, f64)> {
    let decls = BooleanDeclarations::default();
    let mut out = Vec::new();
    for (order, p, q, vp) in [("xy", x, y, vx), ("yx", y, x, vy)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, vx + vy - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vp - common),
        ];
        for (op, run, want) in ops {
            out.push((format!("{order} {op}"), run(p, q, &decls, tol()), want));
        }
    }
    out
}

/// The volume of the convex `pieces` of a unit-height prism inside the
/// half-spaces `planes`.
fn pieces_within(pieces: &[&[(f64, f64)]], planes: &[([f64; 3], f64)]) -> f64 {
    pieces
        .iter()
        .map(|piece| {
            let mut all = planes.to_vec();
            all.push(([0.0, 0.0, 1.0], 1.0));
            all.push(([0.0, 0.0, -1.0], 0.0));
            for (k, &p) in piece.iter().enumerate() {
                let q = piece[(k + 1) % piece.len()];
                let n = [q.1 - p.1, p.0 - q.0, 0.0];
                all.push((n, n[0] * p.0 + n[1] * p.1));
            }
            convex_volume(&all)
        })
        .sum()
}

/// Asserts that `x ∖ y`'s result (the `"yx S"` run of [`every_op`])
/// refuses `PinchUncrossed` and every other run builds `SOUND`.
fn assert_only_the_difference_refuses(
    runs: Vec<(String, Result<BooleanResult<f64>, BooleanError>, f64)>,
) {
    for (tag, r, want) in runs {
        if tag == "yx S" {
            assert!(
                matches!(r, Err(BooleanError::PinchUncrossed { .. })),
                "{tag}: {}",
                outcome(r, want, tol())
            );
        } else {
            let line = outcome(r, want, tol());
            assert!(line.starts_with("OK SOUND"), "{tag}: {line}");
        }
    }
}

/// **A pinch round a notch on a face's outer loop refuses typed.** A
/// ≈300° vee's reflex top corner `(2, 0.5, 1)` inside the cube's near
/// face, the cube along Fibonacci direction 62 of 120 (PR 4038's review
/// r2, `vee300 fib62 face`). The plane cuts the vee in two lobes
/// meeting at the corner; one runs off the face's edge, so in cube ∖
/// vee the near face's outer loop passes the corner twice, round the
/// other lobe's notch. No kept face crosses there but that outer loop,
/// and crossing it leaves a ring meeting the outer loop: the op refuses
/// `PinchUncrossed` (`a-pinch-no-kept-face-can-cross-refuses`, nested).
/// Every other run builds `SOUND` at the clipped volume. Red if an outer
/// loop may cross (`ResultInvalid { RingMeetsOuter }`) or the pre-pass
/// is skipped (`Euler(SelfLoopEdge)`).
#[test]
fn a_pinch_round_a_notch_on_a_faces_outer_loop_refuses_typed() {
    const VEE: [(f64, f64); 5] = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5), (0.0, 4.0)];
    let pieces: [&[(f64, f64)]; 2] = [
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 0.5), (0.0, 4.0)],
        &[(2.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5)],
    ];
    let v = [2.0, 0.5, 1.0];
    let ga = std::f64::consts::PI * (3.0 - 5f64.sqrt());
    let z: f64 = 1.0 - 2.0 * (62.0 + 0.5) / 120.0;
    let r = (1.0 - z * z).sqrt();
    let f = frame([r * (ga * 62.0).cos(), r * (ga * 62.0).sin(), z], 0.0);
    let lo = [-2.0, -2.0, 0.0];
    let vee = finished("the vee", fixtures::prism::<f64>(&VEE, 1.0, tol()).body);
    let cube = finished("the cube", cube_at(v, f, lo));
    let va = mass_properties(&vee, tol()).unwrap().volume;
    let common = pieces_within(&pieces, &cube_planes_at(v, f, lo));
    assert!(common > 1e-3, "the cube holds some of the vee: {common}");
    assert_only_the_difference_refuses(every_op((&vee, va), (&cube, SIDE.powi(3)), common));
}

/// **A staircase's second pinch, round a notch, refuses typed.** The
/// cube's near face through both of [`STAIR`]'s reflex top corners,
/// turned to direction 594 of 720 (PR 4038's review r1, `u2 S_tt
/// b594`). Cube ∖ staircase crosses its first pinch; at the second, the
/// one face through the corner twice passes it on its outer loop. An
/// outer-loop crossing there reads `LoopRoleInverted` on both loops: it
/// keeps the outer role on the hole-shaped half, and the two halves
/// meet at the corner (`RingMeetsOuter`), the nested shape again. The
/// op refuses `PinchUncrossed`, and every other run builds `SOUND`.
/// Red as [`a_pinch_round_a_notch_on_a_faces_outer_loop_refuses_typed`].
#[test]
fn a_staircases_second_pinch_round_a_notch_refuses_typed() {
    let (a, b) = ([2.0, 1.0, 1.0], [1.0, 2.0, 1.0]);
    let [e1, e2, _] = frame(unit([b[0] - a[0], b[1] - a[1], b[2] - a[2]]), 0.0);
    let t = std::f64::consts::TAU * (594.0 + 0.37) / 720.0;
    let m = [0, 1, 2].map(|k| t.cos() * e1[k] + t.sin() * e2[k]);
    let f = frame(m, 0.0);
    let (mid, lo) = ([1.5, 1.5, 1.0], [-2.0, -2.0, 0.0]);
    let stair = finished(
        "the staircase",
        fixtures::prism::<f64>(&STAIR, 1.0, tol()).body,
    );
    let cube = finished("the cube", cube_at(mid, f, lo));
    let pieces: Vec<Vec<(f64, f64)>> = STAIR_BOXES
        .iter()
        .map(|&(l, h)| vec![(l[0], l[1]), (h[0], l[1]), (h[0], h[1]), (l[0], h[1])])
        .collect();
    let pieces: Vec<&[(f64, f64)]> = pieces.iter().map(Vec::as_slice).collect();
    let common = pieces_within(&pieces, &cube_planes_at(mid, f, lo));
    assert!(
        common > 1e-3,
        "the cube holds some of the staircase: {common}"
    );
    assert_only_the_difference_refuses(every_op((&stair, 6.0), (&cube, SIDE.powi(3)), common));
}

/// **An island face pinched to its hole's ring crosses there.** The
/// holed block `[0, 2]³` less `[0.5, 1.5]² × [0, 2]`, its hole corner
/// `(0.5, 0.5, 2)` inside the cube's near face, the cube along the
/// grid's direction `i = 6, j = 0` (PR 4026's review r2, `holed c00
/// side=4 g6.0`). In cube ∖ block the plane's section closes round the
/// hole, so the cube's plane keeps an island inside the hole of its
/// near face, the two touching at the corner, and one seam meets the
/// corner twice. No kept face passes the corner twice; the island's
/// outer corner and the near face's ring corner, one surface and sense,
/// cross by `kef` (`zip::split_across`), the island ringless: the
/// difference builds `SOUND` at the clipped volume, as do ∪ and ∩ in
/// both orders. Red if the two faces' crossing asks both corners to be
/// outer (`PinchUncrossed`). Block ∖ cube refuses before the pre-pass,
/// where two fragments of the pierced face meet the pinch
/// (`a-pierce-weld-refuses-where-its-copies-divide-a-kept-face`).
#[test]
fn an_island_face_pinched_to_its_holes_ring_crosses_and_builds() {
    let v = [0.5, 0.5, 2.0];
    let f = frame(direction(6, 0), 0.0);
    let lo = [-2.0, -2.0, 0.0];
    let mut block = fixtures::holed_block::<f64>(2.0, &[1.0], tol());
    fixtures::describe_as_intersections(&mut block, tol());
    let block = finished("the holed block", block);
    let cube = finished("the cube", cube_at(v, f, lo));
    let box_planes = |l: [f64; 3], h: [f64; 3]| {
        let mut out = cube_planes_at(v, f, lo);
        for t in 0..3 {
            let mut e = [0.0; 3];
            e[t] = 1.0;
            out.push((e, h[t]));
            out.push((e.map(|c| -c), -l[t]));
        }
        out
    };
    let common = convex_volume(&box_planes([0.0; 3], [2.0; 3]))
        - convex_volume(&box_planes([0.5, 0.5, 0.0], [1.5, 1.5, 2.0]));
    assert!(common > 1e-3, "the cube holds some of the block: {common}");
    for (tag, r, want) in every_op((&block, 6.0), (&cube, SIDE.powi(3)), common) {
        let line = outcome(r, want, tol());
        if tag == "xy S" {
            assert!(
                line.contains("two fragments of a pierced face meet one pinch"),
                "{tag}: {line}"
            );
        } else {
            assert!(line.starts_with("OK SOUND"), "{tag}: {line}");
        }
    }
}

/// The oracle against the kernel-free closed form the strut-facing row
/// reads: a face placement at the bare tilt holds the prism's cut by
/// the plane through `v`, and the empty and whole cases read 0 and 3.
#[test]
fn the_sweep_oracle_reads_the_prism() {
    let f = frame([1.0, 1.3, -0.7], 0.0);
    let face = shared(&cube_planes(f, [-2.0, -2.0, 0.0]));
    let opposite = shared(&cube_planes(
        frame([-1.0, -1.3, 0.7], 0.0),
        [-2.0, -2.0, 0.0],
    ));
    assert!(
        (face + opposite - 3.0).abs() < 1e-9,
        "{face} + {opposite} is not the prism's 3"
    );
    assert!(face > 0.5 && opposite > 0.5, "both cuts substantial");
    let away = shared(&cube_planes(frame([0.0, 0.0, 1.0], 0.0), [-2.0, -2.0, 0.0]));
    assert!(
        away.abs() < 1e-12,
        "a cube above the top face holds nothing: {away}"
    );
    let all = shared(&cube_planes(
        frame([0.0, 0.0, -1.0], 0.0),
        [-2.0, -2.0, -0.5],
    ));
    assert!(
        (all - 3.0).abs() < 1e-9,
        "a cube around the prism holds it whole: {all}"
    );
}

/// The shallow prism (a reflex corner of about 200° at
/// `(2, 0.6, 1)`) as two convex pieces, each by its half-spaces.
fn shallow_pieces() -> Vec<Vec<([f64; 3], f64)>> {
    let pieces: [&[(f64, f64)]; 2] = [
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 0.6), (0.0, 1.0)],
        &[(2.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.6)],
    ];
    pieces
        .iter()
        .map(|poly| {
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
        })
        .collect()
}

/// The least distance from `p` to segment `[s0, s1]`.
fn point_segment_distance(p: [f64; 3], (s0, s1): ([f64; 3], [f64; 3])) -> f64 {
    let d = [s1[0] - s0[0], s1[1] - s0[1], s1[2] - s0[2]];
    let r = [p[0] - s0[0], p[1] - s0[1], p[2] - s0[2]];
    let t = (dot(r, d) / dot(d, d)).clamp(0.0, 1.0);
    let gap = [0, 1, 2].map(|k| r[k] - d[k] * t);
    dot(gap, gap).sqrt()
}

/// The least distance between segments `[a0, a1]` and `[b0, b1]`.
fn segment_distance(a: ([f64; 3], [f64; 3]), b: ([f64; 3], [f64; 3])) -> f64 {
    let sub = |p: [f64; 3], q: [f64; 3]| [p[0] - q[0], p[1] - q[1], p[2] - q[2]];
    let (d1, d2, r) = (sub(a.1, a.0), sub(b.1, b.0), sub(a.0, b.0));
    let (aa, ee, f) = (dot(d1, d1), dot(d2, d2), dot(d2, r));
    let (c, bb) = (dot(d1, r), dot(d1, d2));
    let denom = aa * ee - bb * bb;
    let mut sc = if denom > 0.0 {
        ((bb * f - c * ee) / denom).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let mut tc = (bb * sc + f) / ee;
    if tc < 0.0 {
        tc = 0.0;
        sc = (-c / aa).clamp(0.0, 1.0);
    } else if tc > 1.0 {
        tc = 1.0;
        sc = ((bb - c) / aa).clamp(0.0, 1.0);
    }
    let pa = [0, 1, 2].map(|t| a.0[t] + d1[t] * sc);
    let pb = [0, 1, 2].map(|t| b.0[t] + d2[t] * tc);
    let gap = sub(pa, pb);
    dot(gap, gap).sqrt()
}

/// **A near-tangent two-run pierce builds right where the census reads
/// an overlap that is not there.** The shallow prism's reflex corner on
/// a cube whose face plane lies ten bands off the corner's edge toward
/// `(4, 1)` (1e-7 rad at the default ε: review r1's
/// `shallow200 nt e0 a3 d1e-7`; scaled with the band so that the pose
/// is the same at every ε). Prism ∪ cube and prism ∖ cube build at the
/// clipping oracle's volume, with tier 2 and the certificate. Read by
/// segment distance, not by the census, the body's edge pairs come
/// within the band only at the pierce's copies:
/// - edges that share no end point lie farther apart than the band;
/// - edges that share an end vertex part by more than it at the
///   shorter one's far end;
/// - edges that end on one point at two vertices meet at `v`, where the
///   pierce's copies stay apart (no face meets both, PR 3813). Those
///   edges leave `v` 3.7e-7 rad apart (at the default ε) and run
///   within the band for a
///   stretch, which the census passes: that class is filed
///   (`work/contact/two-copies-of-a-pierce-carry-edges-that-run-within-the-band.md`),
///   and this row does not claim them apart.
///
/// Tier 3′ is not asserted: its edge-edge lane reads the section edge
/// from `v` and the prism's edge piece near `(4, 1)` as an overlap,
/// because it reads their line offset at the long edge's start, which
/// lies on the short edge's line
/// (`work/contact/the-census-edge-edge-collinear-lane-reads-the-offset-at-the-long-edges-start.md`).
#[test]
fn a_near_tangent_two_run_pierce_builds_with_edges_in_band_only_at_its_copies() {
    let v = [2.0, 0.6, 1.0];
    let e = unit([2.0, 0.4, 0.0]);
    let [p1, p2, _] = frame(e, 0.0);
    let al = std::f64::consts::TAU * 3.25 / 16.0;
    let band = tol().eps() * tol().get().k;
    let tilt = 10.0 * band;
    let m = unit([0, 1, 2].map(|t| p1[t] * al.cos() + p2[t] * al.sin() + e[t] * tilt));
    let f = frame(m, 0.0);
    let lo = [-2.0, -2.0, 0.0];
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.6), (0.0, 1.0)];
    let prism = finished(
        "the prism",
        fixtures::prism::<f64>(&profile, 1.0, tol()).body,
    );
    let cube = finished("the cube", cube_at(v, f, lo));
    let vol = |b: &Body<f64>| mass_properties(b, tol()).unwrap().volume;
    let common: f64 = shallow_pieces()
        .into_iter()
        .map(|mut planes| {
            planes.extend(cube_planes_at(v, f, lo));
            convex_volume(&planes)
        })
        .sum();
    let (va, vb) = (vol(&prism), SIDE * SIDE * SIDE);
    // Whether the op leaves the pierce's copies apart at `v`.
    let ops: [(&str, Op, f64, bool); 2] = [
        ("union", topo::union_with, va + vb - common, true),
        ("subtract", topo::subtract_with, va - common, false),
    ];
    for (op, run, want, apart) in ops {
        let Ok(BooleanResult::Body(bb)) =
            run(&prism, &cube, &BooleanDeclarations::default(), tol())
        else {
            panic!("{op}: the near-tangent pose did not build");
        };
        assert_eq!(topo::validate_closed(&bb.body), Ok(()), "{op}: tier 2");
        assert!(
            topo::validate_geometric_certificate(&bb.body, tol()).is_ok(),
            "{op}: the certificate"
        );
        let got = vol(&bb.body);
        assert!((got - want).abs() < 1e-9, "{op}: volume {got}, want {want}");
        let end = |he| {
            let k = bb.body.get_half_edge(he).unwrap().start;
            let p = topo::readback::vertex_point(&bb.body, k).unwrap();
            (k, [p.x, p.y, p.z])
        };
        let ends: Vec<_> = bb
            .body
            .edges()
            .map(|(_, ed)| (end(ed.he_plus), end(ed.he_minus)))
            .collect();
        let len = |(a, b): ([f64; 3], [f64; 3])| {
            let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            dot(d, d).sqrt()
        };
        let mut at_copies = 0;
        for (i, &(a0, a1)) in ends.iter().enumerate() {
            for &(b0, b1) in &ends[i + 1..] {
                let (g1, g2) = ((a0.1, a1.1), (b0.1, b1.1));
                let vertex = [a0.0, a1.0].into_iter().find(|k| [b0.0, b1.0].contains(k));
                let point = [g1.0, g1.1].into_iter().find(|p| [g2.0, g2.1].contains(p));
                let gap = match (vertex, point) {
                    (Some(k), _) => {
                        let (short, long, near) = if len(g1) <= len(g2) {
                            (g1, g2, a0.0 == k)
                        } else {
                            (g2, g1, b0.0 == k)
                        };
                        let far = if near { short.1 } else { short.0 };
                        point_segment_distance(far, long)
                    }
                    (None, Some(p)) => {
                        assert!(
                            len((p, v)) <= band,
                            "{op}: edges {g1:?} and {g2:?} end on one point at two \
                             vertices away from the pierce"
                        );
                        at_copies += 1;
                        continue;
                    }
                    (None, None) => segment_distance(g1, g2),
                };
                assert!(
                    gap > band,
                    "{op}: edges {g1:?} and {g2:?} come within {gap:e} of each other"
                );
            }
        }
        assert_eq!(at_copies > 0, apart, "{op}: edges meeting at the copies");
    }
}
