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
use topo::{AtRestBody, Body, BooleanDeclarations, BooleanError, BooleanResult, mass_properties};

use crate::common::differential::outcome;
use crate::common::pinch_cones::{
    Op as Cones, Pieces, Plane, cone_finding, cones_at, faces_through_two_vertices_at,
    point_key_finding, shared_point_finding, shared_point_spread_finding, vertices_at,
};

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
    cube_sized(v, f, lo, SIDE)
}

/// [`cube_at`] with edge `side`.
fn cube_sized(v: [f64; 3], f: [[f64; 3]; 3], lo: [f64; 3], side: f64) -> Body<f64> {
    let [u, w, m] = f;
    fixtures::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (lo[0] + side * x, lo[1] + side * y, lo[2] + side * z);
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
    cube_planes_sized(v, f, lo, SIDE)
}

/// [`cube_sized`]'s six half-spaces.
fn cube_planes_sized(
    v: [f64; 3],
    f: [[f64; 3]; 3],
    lo: [f64; 3],
    side: f64,
) -> Vec<([f64; 3], f64)> {
    let mut out = Vec::new();
    for (axis, &dir) in f.iter().enumerate() {
        let base = dot(dir, v);
        out.push((dir.map(|c| -c), -(base + lo[axis])));
        out.push((dir, base + lo[axis] + side));
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
    boxes_within(&BOXES, planes)
}

/// The volume of the convex `pieces` of the prism `z ∈ [z0, z1]` over
/// them inside the half-spaces `planes`.
fn pieces_within(
    pieces: &[&[(f64, f64)]],
    (z0, z1): (f64, f64),
    planes: &[([f64; 3], f64)],
) -> f64 {
    pieces
        .iter()
        .map(|piece| {
            let mut all = planes.to_vec();
            all.push(([0.0, 0.0, 1.0], z1));
            all.push(([0.0, 0.0, -1.0], -z0));
            for (k, &p) in piece.iter().enumerate() {
                let q = piece[(k + 1) % piece.len()];
                let n = [q.1 - p.1, p.0 - q.0, 0.0];
                all.push((n, n[0] * p.0 + n[1] * p.1));
            }
            convex_volume(&all)
        })
        .sum()
}

/// The volume of the axis boxes `(lo, hi)` inside the half-spaces
/// `planes`.
fn boxes_within(boxes: &[([f64; 3], [f64; 3])], planes: &[([f64; 3], f64)]) -> f64 {
    boxes
        .iter()
        .map(|&(l, h)| {
            let rect = [(l[0], l[1]), (h[0], l[1]), (h[0], h[1]), (l[0], h[1])];
            pieces_within(&[&rect], (l[2], h[2]), planes)
        })
        .sum()
}

/// `body` as a boolean operand: the at-rest gate's finished body.
fn finished(what: &str, body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol())
        .unwrap_or_else(|e| panic!("{what} is not a finished body: {e:?}"))
}

/// One run of an op: its tag, its result and the oracle's volume.
type Run = (String, Result<BooleanResult<f64>, BooleanError>, f64);

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
fn pose_runs((place, lo): (&str, [f64; 3]), (i, j, psi): (u32, u32, f64)) -> Vec<Run> {
    let prism = finished(
        "the prism",
        fixtures::prism::<f64>(&PROFILE, 1.0, tol()).body,
    );
    let va = mass_properties(&prism, tol()).unwrap().volume;
    let f = frame(direction(i, j), psi);
    let b = finished("the cube", cube(f, lo));
    let common = shared(&cube_planes(f, lo));
    every_op(["pc", "cp"], (&prism, va), (&b, SIDE.powi(3)), common)
        .into_iter()
        .map(|(tag, r, want)| (format!("{place} i={i} j={j} psi={psi} {tag}"), r, want))
        .collect()
}

/// Every op in both orders between `x` (volume `vx`) and `y` (`vy`),
/// which share `common`: `(tag, result, want)`, the tag `"{order} {op}"`
/// with `orders` naming x-then-y and y-then-x.
fn every_op(
    orders: [&str; 2],
    (x, vx): (&AtRestBody<f64>, f64),
    (y, vy): (&AtRestBody<f64>, f64),
    common: f64,
) -> Vec<Run> {
    let decls = BooleanDeclarations::default();
    let mut out = Vec::new();
    for (order, p, q, vp) in [(orders[0], x, y, vx), (orders[1], y, x, vy)] {
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

/// Boxes `(lo, hi)` as convex pieces.
fn box_pieces(boxes: &[([f64; 3], [f64; 3])]) -> Pieces {
    boxes
        .iter()
        .map(|&(l, h)| {
            (0..3)
                .flat_map(|k| {
                    let e = [0, 1, 2].map(|j| if j == k { 1.0 } else { 0.0 });
                    [(e.map(|c| -c), -l[k]), (e, h[k])]
                })
                .collect()
        })
        .collect()
}

/// The operands and op a run's tag names, its last two words `"{order}
/// {op}"`: `order` is `first` where `x` came first.
fn tag_cones<'p>(
    tag: &str,
    first: &str,
    (x, y): (&'p [Vec<Plane>], &'p [Vec<Plane>]),
) -> (&'p [Vec<Plane>], &'p [Vec<Plane>], Cones) {
    let mut words = tag.rsplit(' ');
    let (op, order) = (words.next().unwrap(), words.next().unwrap());
    let (p, q) = if order == first { (x, y) } else { (y, x) };
    let op = match op {
        "U" => Cones::Union,
        "I" => Cones::Intersect,
        "S" => Cones::Subtract,
        other => panic!("{tag}: no op {other}"),
    };
    (p, q, op)
}

/// Asserts that every run's body holds one vertex per cone at each of
/// `points` ([`pierce_point_finding`]), the runs from [`every_op`] with
/// `first` naming `x` first.
fn assert_one_vertex_per_cone(
    runs: &[Run],
    points: &[[f64; 3]],
    first: &str,
    pieces: (&[Vec<Plane>], &[Vec<Plane>]),
) {
    for (tag, r, _) in runs {
        let Some(bb) = r.as_ref().ok().and_then(BooleanResult::body) else {
            continue;
        };
        for &p in points {
            let finding = pierce_point_finding(&bb.body, p, tag_cones(tag, first, pieces));
            assert_eq!(finding, None, "{tag} at {p:?}");
        }
    }
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

/// Where a built body breaks one vertex per cone at the pierce point
/// `at`: none, if its vertices there share one point key, are as many as
/// the op's cones ([`cone_finding`]), and the body tessellates and
/// passes `check_mesh` (the mesher refuses corners that cross at a
/// pinch), else the finding.
fn pierce_point_finding(
    body: &Body<f64>,
    at: [f64; 3],
    cones: (&[Vec<Plane>], &[Vec<Plane>], Cones),
) -> Option<String> {
    if let Some(finding) = shared_point_spread_finding()
        .1
        .or_else(|| shared_point_finding(body, at))
        .or_else(|| cone_finding(body, at, cones))
    {
        return Some(finding);
    }
    match mesh::tessellate(body, 0.05, tol()).map(|m| mesh::validate::check_mesh(&m)) {
        Ok(Ok(())) => None,
        other => Some(format!("the body does not mesh: {other:?}")),
    }
}

/// **The sweep's guard**, over a committed subset of
/// [`pierce_runs_battery`]: every direction of the face placement (the
/// pierce lane, where the two-run families live) and every third
/// direction of the edge and corner placements at their first turn. No
/// pose ships a body that is not `SOUND` by [`outcome`], and in the
/// face placement every body holds one vertex per cone at `v`, on one
/// point key, and meshes. In the face placement refusals pass: the residue is the
/// filed rows'. The edge and corner placements reach `v` through the
/// vertex-vertex lane, where every run builds `SOUND`.
#[test]
fn the_sweep_subset_ships_no_bad_body() {
    let mut bad = Vec::new();
    for (place, lo, psis) in PLACEMENTS {
        let every = if place == "face" { 1 } else { 3 };
        for i in (0..12).step_by(every) {
            for j in 0..7 {
                let prism = box_pieces(&BOXES);
                let cube = vec![cube_planes(frame(direction(i, j), psis[0]), lo)];
                for (tag, r, want) in pose_runs((place, lo), (i, j, psis[0])) {
                    let finding = match &r {
                        Ok(res) if place == "face" => res.body().and_then(|bb| {
                            pierce_point_finding(
                                &bb.body,
                                V,
                                tag_cones(&tag, "pc", (&prism, &cube)),
                            )
                        }),
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

/// A convex CCW polygon's prism over z ∈ [0, 1] as half-spaces `n·x ≤ d`.
fn polygon_prism(poly: &[(f64, f64)]) -> Vec<([f64; 3], f64)> {
    let mut planes: Vec<([f64; 3], f64)> = (0..poly.len())
        .map(|i| {
            let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
            let out = [q.1 - p.1, p.0 - q.0, 0.0];
            (out, out[0] * p.0 + out[1] * p.1)
        })
        .collect();
    planes.push(([0.0, 0.0, 1.0], 1.0));
    planes.push(([0.0, 0.0, -1.0], 0.0));
    planes
}

/// A prism with a reflex top corner at `v`: its profile and the convex
/// polygons whose prisms tile it, with disjoint interiors.
struct Corner {
    profile: Vec<(f64, f64)>,
    pieces: Vec<Vec<(f64, f64)>>,
    v: [f64; 3],
}

/// A 343° notch: corner `(1, 1, 1)`.
fn notch343() -> Corner {
    Corner {
        profile: vec![
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 0.85),
            (1.0, 1.0),
            (2.0, 1.15),
            (2.0, 2.0),
            (0.0, 2.0),
        ],
        pieces: vec![
            vec![(0.0, 0.0), (2.0, 0.0), (2.0, 0.85), (1.0, 1.0), (0.0, 1.0)],
            vec![(0.0, 1.0), (1.0, 1.0), (2.0, 1.15), (2.0, 2.0), (0.0, 2.0)],
        ],
        v: [1.0, 1.0, 1.0],
    }
}

/// The wedge of the square `[−2, 2]²` between the rays at 0 and `alpha`
/// degrees, counterclockwise: an `alpha`° corner at `(0, 0, 1)`, fanned
/// into triangles from it.
fn wedge(alpha: f64) -> Corner {
    let at = |t: f64| {
        let t = t.to_radians();
        let r = 2.0 / t.cos().abs().max(t.sin().abs());
        (r * t.cos(), r * t.sin())
    };
    let mut profile = vec![(0.0, 0.0), at(0.0)];
    let mut c = 45.0;
    while c < alpha {
        profile.push(at(c));
        c += 90.0;
    }
    profile.push(at(alpha));
    let pieces = (1..profile.len() - 1)
        .map(|k| vec![profile[0], profile[k], profile[k + 1]])
        .collect();
    Corner {
        profile,
        pieces,
        v: [0.0, 0.0, 1.0],
    }
}

fn wedge345() -> Corner {
    wedge(345.0)
}

/// A pose: its name, the corner, the cube's direction and turn, and its
/// placement.
type Pose = (&'static str, fn() -> Corner, [f64; 3], f64, [f64; 3]);

/// **A corner whose link crosses the cube's six times builds every op.**
/// Two simple links round one point cross an even number of times, and
/// A's runs on one side of B are disjoint arcs of one disk B's link
/// bounds, so A's consecutive pairing never crosses in B's walk order.
/// At six crossings it may nest there: a pair holds another's two germs
/// between its own either way round B's vertex, so B's run for it holds
/// that pair's run, which mints at its copy. Each pose here nests, and
/// on a tree that demanded adjacency in B all refused `PairingMismatch`
/// (`work/join/a-six-crossing-vertex-pair-nests-its-pairing-and-refuses-pairing-mismatch.md`).
/// The held runs here are struts inside the holder's fan, in its first
/// corner, its last or between, and fans ending in its first corner or
/// its last. Every op in both orders is `SOUND` at the corner's pieces
/// clipped by the cube's half-spaces. Red if a held run mints at the
/// corner's own vertex, mints before its holder, or anchors a strut in
/// the holder's first corner on the half the holder left behind.
#[test]
fn six_crossing_corners_build_every_op() {
    // The sweep's grid direction for the notch, review r2's for the
    // wedge.
    let r2 = |i: u32, j: u32| {
        let theta = std::f64::consts::TAU * (f64::from(i) + 0.11) / 12.0;
        let phi = (f64::from(j) - 3.0) * 0.43 + 0.02;
        [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()]
    };
    let (edge, corner) = (PLACEMENTS[1].1, PLACEMENTS[2].1);
    let poses: [Pose; 3] = [
        (
            "notch edge i=3 j=0 psi=1",
            notch343,
            direction(3, 0),
            1.0,
            edge,
        ),
        (
            "notch corner i=0 j=0 psi=2.2",
            notch343,
            direction(0, 0),
            2.2,
            corner,
        ),
        ("wedge edge i=3 j=0 psi=0.3", wedge345, r2(3, 0), 0.3, edge),
    ];
    let decls = BooleanDeclarations::default();
    let mut bad = Vec::new();
    for (pose, shape, m, psi, lo) in poses {
        let c = shape();
        let a = finished(pose, fixtures::prism::<f64>(&c.profile, 1.0, tol()).body);
        let f = frame(m, psi);
        let b = finished("the cube", cube_at(c.v, f, lo));
        let clip = |extra: &[([f64; 3], f64)]| -> f64 {
            c.pieces
                .iter()
                .map(|p| {
                    let mut all = polygon_prism(p);
                    all.extend_from_slice(extra);
                    convex_volume(&all)
                })
                .sum()
        };
        let (va, vb) = (clip(&[]), SIDE * SIDE * SIDE);
        let common = clip(&cube_planes_at(c.v, f, lo));
        for (order, x, y, vx) in [("ac", &a, &b, va), ("ca", &b, &a, vb)] {
            let ops: [(&str, Op, f64); 3] = [
                ("U", topo::union_with, va + vb - common),
                ("I", topo::intersect_with, common),
                ("S", topo::subtract_with, vx - common),
            ];
            for (op, run, want) in ops {
                let line = outcome(run(x, y, &decls, tol()), want, tol());
                if !line.starts_with("OK SOUND") {
                    bad.push(format!("{pose} {order} {op}: {line}"));
                }
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

/// **Two pinches in one op are each split per cone.** The cube's near
/// face lies in a plane through both of [`STAIR`]'s reflex top corners,
/// turned so that each corner has two Out runs (PR 4038's review r1, `u2
/// S_tt`, direction 204 of 720). The intersection pinches at both
/// corners, so `zip::split_cones` splits two vertex pairs in one op, and
/// the lumps that met there become shells of their own. Every op in
/// both orders builds `SOUND` at the boxes' clipped volume, holds one
/// vertex per cone at each corner, on one point key, and meshes. Red if the split
/// stops after its first pinch (the second pinch's zip fuses it to
/// itself), or if the lumps stay one shell (the merge refuses a
/// disconnected shell).
#[test]
fn two_pinches_in_one_op_are_each_split_per_cone() {
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
    let common = boxes_within(&STAIR_BOXES, &cube_planes_at(mid, f, lo));
    assert!(
        common > 1e-3,
        "the cube holds some of the staircase: {common}"
    );
    let pieces = (box_pieces(&STAIR_BOXES), vec![cube_planes_at(mid, f, lo)]);
    for (tag, r, want) in every_op(["pc", "cp"], (&prism, 6.0), (&cube, SIDE.powi(3)), common) {
        let findings: Vec<String> = match &r {
            Ok(res) => res
                .body()
                .map(|bb| {
                    [a, b]
                        .into_iter()
                        .filter_map(|p| {
                            let cones = tag_cones(&tag, "pc", (&pieces.0, &pieces.1));
                            pierce_point_finding(&bb.body, p, cones)
                        })
                        .collect()
                })
                .unwrap_or_default(),
            Err(_) => Vec::new(),
        };
        let line = outcome(r, want, tol());
        assert!(line.starts_with("OK SOUND"), "{tag}: {line}");
        assert!(findings.is_empty(), "{tag}: {findings:?}");
    }
}

/// **A corner pinch on the second operand's side builds.** The
/// L-prism's bottom reflex corner `(1, 1, 0)` at a corner of the cube,
/// the cube along Fibonacci direction 19 of 120 and turned 1.9 about it
/// (PR 4038's review r2, `Lbot fib19 corner psi=1.9`). Cube ∪ prism
/// holds two vertices at the corner, one per cone, on one point key: the
/// seams meet each cone once, so the zips leave them apart and no vertex
/// is split. The union builds `SOUND` and meshes. Red if the vertices at
/// the corner are not one per cone on one point key, or the body does
/// not mesh.
#[test]
fn a_corner_pinch_on_the_second_operands_side_builds() {
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
    let pieces = (vec![cube_planes_at(v, f, lo)], box_pieces(&BOXES));
    let finding = r
        .as_ref()
        .ok()
        .and_then(|res| res.body())
        .and_then(|bb| pierce_point_finding(&bb.body, v, (&pieces.0, &pieces.1, Cones::Union)));
    let line = outcome(r, want, tol());
    assert!(line.starts_with("OK SOUND"), "cube ∪ prism: {line}");
    assert_eq!(finding, None, "cube ∪ prism");
}

/// **A vertex-vertex pinch the pairing start avoids.** With `v` on the
/// cube's edge (direction `i = 6, j = 1`), the vertex pair crosses four
/// times, and its pairing starts where A's runs lie on the side the
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

/// Asserts that the `tag` run's body has a face through two vertices at
/// `at`: one vertex per cone, the face passing both.
fn assert_a_face_runs_through_two_vertices(runs: &[Run], tag: &str, at: [f64; 3]) {
    let body = runs
        .iter()
        .find(|(t, ..)| t == tag)
        .and_then(|(_, r, _)| r.as_ref().ok())
        .and_then(BooleanResult::body)
        .unwrap_or_else(|| panic!("{tag}: no body"));
    assert!(
        faces_through_two_vertices_at(&body.body, at) > 0,
        "{tag}: no face runs through two vertices at the pinch"
    );
}

/// Asserts that every run builds `SOUND`, and that each body tessellates
/// and passes `check_mesh`.
fn assert_every_run_builds_and_meshes(runs: Vec<Run>) {
    for (tag, r, want) in runs {
        let meshed = r.as_ref().ok().and_then(BooleanResult::body).map(|bb| {
            mesh::tessellate(&bb.body, 0.05, tol()).map(|m| mesh::validate::check_mesh(&m))
        });
        let line = outcome(r, want, tol());
        assert!(line.starts_with("OK SOUND"), "{tag}: {line}");
        assert!(
            matches!(meshed, Some(Ok(Ok(())))),
            "{tag}: the body does not mesh: {meshed:?}"
        );
    }
}

/// **A pinch round a notch on a face's outer loop is one vertex per
/// cone.** A ≈300° vee's reflex top corner `(2, 0.5, 1)` inside the
/// cube's near face, the cube along Fibonacci direction 62 of 120 (PR
/// 4038's review r2, `vee300 fib62 face`). The plane cuts the vee in two
/// lobes meeting at the corner; one runs off the face's edge, so in cube
/// ∖ vee the near face's outer loop passes the corner twice, round the
/// other lobe's notch. The result holds one vertex per cone there, two
/// on one point key, and the near face's one outer loop runs through
/// both (`a-pinch-no-kept-face-can-cross-refuses`). Every op in both
/// orders builds `SOUND` at the clipped volume and meshes. Red if the
/// zips fuse the pinch to itself (`Euler(SelfLoopEdge)`), or if the
/// outer loop is split into a ring meeting it (`RingMeetsOuter`).
#[test]
fn a_pinch_round_a_notch_on_a_faces_outer_loop_is_one_vertex_per_cone() {
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
    let common = pieces_within(&pieces, (0.0, 1.0), &cube_planes_at(v, f, lo));
    assert!(common > 1e-3, "the cube holds some of the vee: {common}");
    let runs = every_op(["xy", "yx"], (&vee, va), (&cube, SIDE.powi(3)), common);
    assert_a_face_runs_through_two_vertices(&runs, "yx S", v);
    let vee_pieces: Pieces = pieces.iter().map(|p| polygon_prism(p)).collect();
    let cube_pieces = vec![cube_planes_at(v, f, lo)];
    assert_one_vertex_per_cone(&runs, &[v], "xy", (&vee_pieces, &cube_pieces));
    assert_every_run_builds_and_meshes(runs);
}

/// **A staircase's second pinch, round a notch, is one vertex per
/// cone.** The cube's near face through both of [`STAIR`]'s reflex top
/// corners, turned to direction 594 of 720 (PR 4038's review r1, `u2
/// S_tt b594`). Cube ∖ staircase pinches at both corners, and each
/// holds two vertices, one per cone, on one point key. Every op in both
/// orders builds `SOUND` and meshes. Red as
/// [`a_pinch_round_a_notch_on_a_faces_outer_loop_is_one_vertex_per_cone`].
#[test]
fn a_staircases_second_pinch_round_a_notch_is_one_vertex_per_cone() {
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
    let common = boxes_within(&STAIR_BOXES, &cube_planes_at(mid, f, lo));
    assert!(
        common > 1e-3,
        "the cube holds some of the staircase: {common}"
    );
    let runs = every_op(["xy", "yx"], (&stair, 6.0), (&cube, SIDE.powi(3)), common);
    let body = runs
        .iter()
        .find(|(t, ..)| t == "yx S")
        .and_then(|(_, r, _)| r.as_ref().ok())
        .and_then(BooleanResult::body)
        .expect("yx S builds");
    for p in [a, b] {
        assert_eq!(vertices_at(&body.body, p).len(), 2, "yx S: {p:?}");
    }
    let pieces = (box_pieces(&STAIR_BOXES), vec![cube_planes_at(mid, f, lo)]);
    assert_one_vertex_per_cone(&runs, &[a, b], "xy", (&pieces.0, &pieces.1));
    assert_every_run_builds_and_meshes(runs);
}

/// `fixtures::holed_block(2.0, &[1.0])`, `[0, 2]³` less `[0.5, 1.5]² ×
/// [0, 2]`, as four boxes.
fn holed_block_pieces() -> Pieces {
    box_pieces(&[
        ([0.0, 0.0, 0.0], [2.0, 0.5, 2.0]),
        ([0.0, 0.5, 0.0], [0.5, 1.5, 2.0]),
        ([1.5, 0.5, 0.0], [2.0, 1.5, 2.0]),
        ([0.0, 1.5, 0.0], [2.0, 2.0, 2.0]),
    ])
}

/// **An island face pinched to its hole's ring stays its own face.**
/// The holed block `[0, 2]³` less `[0.5, 1.5]² × [0, 2]`, its hole corner
/// `(0.5, 0.5, 2)` inside the cube's near face, the cube along the
/// grid's direction `i = 6, j = 0` (PR 4026's review r2, `holed c00
/// side=4 g6.0`). In cube ∖ block the plane's section closes round the
/// hole, so the cube's plane keeps an island inside the hole of its
/// near face, the two touching at the corner, and one seam meets the
/// corner twice: the island's outer loop and the near face's ring pass
/// it at two vertices on one point key. Every op in both orders builds
/// `SOUND` at the clipped volume and meshes. Red if the split leaves the
/// seam meeting one vertex twice (`ZipCorrespondence`), or kills the
/// island into the holed face.
#[test]
fn an_island_face_pinched_to_its_holes_ring_stays_its_own_face() {
    let v = [0.5, 0.5, 2.0];
    let f = frame(direction(6, 0), 0.0);
    let lo = [-2.0, -2.0, 0.0];
    let mut block = fixtures::holed_block::<f64>(2.0, &[1.0], tol());
    fixtures::describe_as_intersections(&mut block, tol());
    let block = finished("the holed block", block);
    let cube = finished("the cube", cube_at(v, f, lo));
    let planes = cube_planes_at(v, f, lo);
    let common = boxes_within(&[([0.0; 3], [2.0; 3])], &planes)
        - boxes_within(&[([0.5, 0.5, 0.0], [1.5, 1.5, 2.0])], &planes);
    assert!(common > 1e-3, "the cube holds some of the block: {common}");
    let runs = every_op(["xy", "yx"], (&block, 6.0), (&cube, SIDE.powi(3)), common);
    let pieces = (holed_block_pieces(), vec![planes]);
    assert_one_vertex_per_cone(&runs, &[v], "xy", (&pieces.0, &pieces.1));
    assert_every_run_builds_and_meshes(runs);
}

/// **The holed block's intersection pinched at its hole corner builds.**
/// The holed block of
/// [`an_island_face_pinched_to_its_holes_ring_stays_its_own_face`], its
/// hole corner `(0.5, 0.5, 2)` inside the cube's near face, the cube
/// along the grid's direction `i = 0, j = 5` (PR 4026's review r2,
/// `holed c00 side=4 g0.5`). Block ∩ cube pinches at the corner, where
/// the cube's plane meets the hole's two walls; the result holds one
/// vertex per cone there, on one point key. Every op in both orders
/// builds `SOUND` at the clipped volume and meshes. Red if the zips fuse
/// the pinch to itself.
#[test]
fn the_holed_blocks_intersection_pinched_at_its_hole_corner_builds() {
    let v = [0.5, 0.5, 2.0];
    let f = frame(direction(0, 5), 0.0);
    let lo = [-2.0, -2.0, 0.0];
    let mut block = fixtures::holed_block::<f64>(2.0, &[1.0], tol());
    fixtures::describe_as_intersections(&mut block, tol());
    let block = finished("the holed block", block);
    let cube = finished("the cube", cube_at(v, f, lo));
    let planes = cube_planes_at(v, f, lo);
    let common = boxes_within(&[([0.0; 3], [2.0; 3])], &planes)
        - boxes_within(&[([0.5, 0.5, 0.0], [1.5, 1.5, 2.0])], &planes);
    assert!(common > 1e-3, "the cube holds some of the block: {common}");
    let runs = every_op(["xy", "yx"], (&block, 6.0), (&cube, SIDE.powi(3)), common);
    let pieces = (holed_block_pieces(), vec![planes]);
    assert_one_vertex_per_cone(&runs, &[v], "xy", (&pieces.0, &pieces.1));
    assert_every_run_builds_and_meshes(runs);
}

/// **An island pinched twice to its hole's ring stays its own face at
/// both points.** The block `[0, 4]² × [0, 2]` less a U-shaped hole
/// whose arms end in tips at `(1.25, 3, 2)` and `(2.75, 3, 2)`; the cube
/// (side 12) has its near face in a plane through both tips, turned to
/// direction 54 of 72 about their line (PR 4051's review, `u2tip mid
/// side=12 psi=0 th54`). In cube ∖ block the plane keeps an island inside
/// the hole of its near face, touching the hole's ring at both tips, and
/// each tip is a pinch split per cone. Every op in both orders builds
/// `SOUND` at the clipped volume and meshes.
#[test]
fn an_island_pinched_twice_to_its_holes_ring_stays_its_own_face() {
    const RIM: [(f64, f64); 10] = [
        (1.0, 1.0),
        (3.0, 1.0),
        (3.0, 2.8),
        (2.75, 3.0),
        (2.5, 2.8),
        (2.5, 1.8),
        (1.5, 1.8),
        (1.5, 2.8),
        (1.25, 3.0),
        (1.0, 2.8),
    ];
    let hole: [&[(f64, f64)]; 3] = [
        &[(1.0, 1.0), (3.0, 1.0), (3.0, 1.8), (1.0, 1.8)],
        &[(1.0, 1.8), (1.5, 1.8), (1.5, 2.8), (1.25, 3.0), (1.0, 2.8)],
        &[(2.5, 1.8), (3.0, 1.8), (3.0, 2.8), (2.75, 3.0), (2.5, 2.8)],
    ];
    let h = 2.0;
    let mut block = Body::<f64>::new();
    let ops = fixtures::prism_ops(
        &mut block,
        &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
        (0.0, h),
        fixtures::identity_map::<f64>,
        fixtures::FaceGeometry::Declined,
        tol(),
    );
    let at =
        |z: f64| -> Vec<Point3<f64>> { RIM.iter().map(|&(x, y)| Point3::new(x, y, z)).collect() };
    fixtures::drill_hole(
        &mut block,
        ops.sides[0].he_plus,
        ops.bottom.face,
        &at(h),
        &at(0.0),
        tol(),
    );
    fixtures::plane_every_face(&mut block, tol());
    fixtures::describe_as_intersections(&mut block, tol());
    let block = finished("the U-holed block", block);
    let (p, q) = ([1.25, 3.0, h], [2.75, 3.0, h]);
    let [e1, e2, _] = frame(unit([q[0] - p[0], q[1] - p[1], q[2] - p[2]]), 0.0);
    let t = std::f64::consts::TAU * (54.0 + 0.37) / 72.0;
    let f = frame([0, 1, 2].map(|k| t.cos() * e1[k] + t.sin() * e2[k]), 0.0);
    let (v, side) = ([2.0, 3.0, h], 12.0);
    let lo = [-side / 2.0, -side / 2.0, 0.0];
    let cube = finished("the cube", cube_sized(v, f, lo, side));
    let planes = cube_planes_sized(v, f, lo, side);
    let common = boxes_within(&[([0.0, 0.0, 0.0], [4.0, 4.0, h])], &planes)
        - pieces_within(&hole, (0.0, h), &planes);
    let vb = 16.0 * h - pieces_within(&hole, (0.0, h), &[]);
    assert!(common > 1e-3, "the cube holds some of the block: {common}");
    let runs = every_op(["xy", "yx"], (&block, vb), (&cube, side.powi(3)), common);
    // The block round the hole, each tip's cap cut along the tip's line.
    let solid: [&[(f64, f64)]; 8] = [
        &[(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (0.0, 1.0)],
        &[(0.0, 1.0), (1.0, 1.0), (1.0, 4.0), (0.0, 4.0)],
        &[(3.0, 1.0), (4.0, 1.0), (4.0, 4.0), (3.0, 4.0)],
        &[(1.5, 1.8), (2.5, 1.8), (2.5, 4.0), (1.5, 4.0)],
        &[(1.0, 2.8), (1.25, 3.0), (1.25, 4.0), (1.0, 4.0)],
        &[(1.25, 3.0), (1.5, 2.8), (1.5, 4.0), (1.25, 4.0)],
        &[(2.5, 2.8), (2.75, 3.0), (2.75, 4.0), (2.5, 4.0)],
        &[(2.75, 3.0), (3.0, 2.8), (3.0, 4.0), (2.75, 4.0)],
    ];
    let block_pieces: Pieces = solid
        .iter()
        .map(|poly| {
            let mut planes = polygon_prism(poly);
            planes.retain(|&(n, _)| n[2] == 0.0);
            planes.extend([([0.0, 0.0, 1.0], h), ([0.0, 0.0, -1.0], 0.0)]);
            planes
        })
        .collect();
    let block_volume: f64 = solid
        .iter()
        .map(|poly| h * crate::common::differential::area(poly))
        .sum();
    assert!(
        (block_volume - vb).abs() < 1e-12,
        "the pieces tile the block: {block_volume} vs {vb}"
    );
    let cube_pieces = vec![planes];
    assert_one_vertex_per_cone(&runs, &[p, q], "xy", (&block_pieces, &cube_pieces));
    assert_every_run_builds_and_meshes(runs);
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
/// - edges that meet at `v`, where the pierce's copies stay apart (no
///   face meets both, PR 3813): edges that end there at two vertices,
///   or an edge ending there on the interior of a seam edge the output
///   stage joined through the other copy's point. Those
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
            .map(|(k, ed)| (end(ed.he_plus), end(ed.he_minus), k))
            .collect();
        let joined: Vec<_> = bb.naming.edge_joins.iter().map(|j| j.kept).collect();
        let len = |(a, b): ([f64; 3], [f64; 3])| {
            let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            dot(d, d).sqrt()
        };
        let mut at_copies = 0;
        for (i, &(a0, a1, ka)) in ends.iter().enumerate() {
            for &(b0, b1, kb) in &ends[i + 1..] {
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
                    (None, None) => {
                        // The output stage joins the copy lying on a seam
                        // line away (maximal edges), so the other copy's
                        // edge ends at `v` on the interior of that joined
                        // seam: one edge ends at `v`, and the other is a
                        // joined edge with `v` inside it, both its ends
                        // clear of `v`.
                        let at_v = |g: ([f64; 3], [f64; 3])| {
                            len((g.0, v)) <= band || len((g.1, v)) <= band
                        };
                        let through_v = |g: ([f64; 3], [f64; 3]), k| {
                            joined.contains(&k) && !at_v(g) && point_segment_distance(v, g) <= band
                        };
                        if (at_v(g1) && through_v(g2, kb)) || (at_v(g2) && through_v(g1, ka)) {
                            at_copies += 1;
                            continue;
                        }
                        segment_distance(g1, g2)
                    }
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

/// A half-space `n·x ≤ d`.
type HalfSpace = ([f64; 3], f64);

/// `c` turned by the frame `f` about its corner and moved to `at`
/// (local `x ↦ f·(x − c.v) + at`): its body and its pieces' half-spaces.
fn posed(c: &Corner, f: [[f64; 3]; 3], at: [f64; 3]) -> (AtRestBody<f64>, Vec<Vec<HalfSpace>>) {
    let map = |x: [f64; 3]| {
        let r = [x[0] - c.v[0], x[1] - c.v[1], x[2] - c.v[2]];
        [0, 1, 2].map(|t| at[t] + f[0][t] * r[0] + f[1][t] * r[1] + f[2][t] * r[2])
    };
    let mut b = Body::<f64>::new();
    fixtures::prism_ops(
        &mut b,
        &c.profile,
        (0.0, 1.0),
        |x, y, z| {
            let p = map([x, y, z]);
            Point3::new(p[0], p[1], p[2])
        },
        fixtures::FaceGeometry::Certified,
        tol(),
    );
    fixtures::describe_as_intersections(&mut b, tol());
    let turn = |n: [f64; 3]| [0, 1, 2].map(|t| f[0][t] * n[0] + f[1][t] * n[1] + f[2][t] * n[2]);
    let pieces = c
        .pieces
        .iter()
        .map(|p| {
            polygon_prism(p)
                .into_iter()
                .map(|(n, d)| (turn(n), d - dot(n, c.v) + dot(turn(n), at)))
                .collect()
        })
        .collect();
    (finished("a posed corner", b), pieces)
}

/// `a` at rest and `b` posed by `f` with its corner on `a`'s: every op
/// in both orders, against the two corners' pieces clipped pairwise.
fn corner_pair_runs(a: &Corner, b: &Corner, f: [[f64; 3]; 3]) -> Vec<Run> {
    let x = finished(
        "a corner",
        fixtures::prism::<f64>(&a.profile, 1.0, tol()).body,
    );
    let (y, bp) = posed(b, f, a.v);
    let ap: Vec<_> = a.pieces.iter().map(|p| polygon_prism(p)).collect();
    let vol = |ps: &[Vec<([f64; 3], f64)>]| ps.iter().map(|p| convex_volume(p)).sum::<f64>();
    let (va, vb) = (vol(&ap), vol(&bp));
    let common: f64 = ap
        .iter()
        .flat_map(|p| {
            bp.iter().map(move |q| {
                let mut all = p.clone();
                all.extend_from_slice(q);
                convex_volume(&all)
            })
        })
        .sum();
    let decls = BooleanDeclarations::default();
    let mut out = Vec::new();
    for (order, l, r, vl) in [("ab", &x, &y, va), ("ba", &y, &x, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vl - common),
        ];
        for (op, run, want) in ops {
            out.push((format!("{order} {op}"), run(l, r, &decls, tol()), want));
        }
    }
    out
}

/// **Two reflex corners crossing eight times nest two deep, and build
/// every op.** Each pose is a review's (PR 4050's r2 grid
/// `notch343 vs notch343 i=13 j=8 k=2`, r1's seeded rotation
/// `n343 n330 k=105`). In B one pair's fan holds a strut that holds
/// another strut, so the innermost mints at the fan's copy and hangs at
/// the outer strut's tip there. Every op in both orders is `SOUND` at
/// the two corners' pieces clipped pairwise. Red as a
/// `ClassificationInvariant` when a run held only by a strut mints at
/// the plan's own vertex although a fan further out carried its corner
/// away, and red when a run's holder is the outermost run that holds it
/// rather than the innermost.
#[test]
fn eight_crossing_corners_nest_two_deep_and_build_every_op() {
    let r2 = {
        let theta = std::f64::consts::TAU * 13.11 / 24.0;
        let phi: f64 = 2.0 * 0.23 + 0.02;
        let m = [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()];
        frame(m, 2.3 * std::f64::consts::TAU / 24.0)
    };
    let r1 = [
        [0.6060175730696241, -0.785797686202516, -0.12355038441694527],
        [
            -0.29342602576332566,
            -0.36520218952479033,
            0.8834752561170229,
        ],
        [
            -0.7393536829796299,
            -0.4991486322981075,
            -0.45189243669194745,
        ],
    ];
    let mut bad = Vec::new();
    for (pose, a, b, f) in [
        ("notch343 vs notch343", notch343(), notch343(), r2),
        ("wedge343 vs wedge330", wedge(343.0), wedge(330.0), r1),
    ] {
        for (tag, r, want) in corner_pair_runs(&a, &b, f) {
            let line = outcome(r, want, tol());
            if !line.starts_with("OK SOUND") {
                bad.push(format!("{pose} {tag}: {line}"));
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

/// **A nested pairing at a vertex another crossing pair shares refuses
/// typed.** [`notch343`] against a pinch of two cubes at its corner,
/// the sweep's grid direction `i=0 j=0` turned `psi=2.2`, the pinch
/// first: the notch crosses one cube's corner six times, nested in that
/// cube's walk order, and the other's as well, at the notch's corner,
/// B's vertex here and shared. Every op refuses `SharedVertexCrossings`,
/// since turning a nested run to clear the other pair's cuts would make
/// it hold the rest
/// (`work/join/a-nested-pairing-at-a-shared-vertex-refuses-shared-vertex-crossings.md`).
/// Red if the shared vertex's nested plan reaches the reconcile.
#[test]
fn a_nested_pairing_at_a_shared_vertex_refuses_typed() {
    for (tag, r, want) in pinch_runs(direction(0, 0), 2.2) {
        if tag.starts_with("ba") {
            assert!(
                matches!(r, Err(BooleanError::SharedVertexCrossings { .. })),
                "{tag}: {}",
                outcome(r, want, tol())
            );
        }
    }
}

/// **A six-crossing pair at the notch's shared corner builds.** The pose
/// of [`a_nested_pairing_at_a_shared_vertex_refuses_typed`] with the
/// notch first: the shared vertex is A's, where no pairing nests, and
/// one run turns. Every op builds `SOUND`, one vertex per cone at the
/// corner, on one point key, and meshing ([`pierce_point_finding`]).
/// Red as `ClassificationInvariant` ("a vertex at a shared point is the
/// In end of one null edge and the Out end of another") if a turned
/// run's own pair's runs mint at the shared vertex
/// (`insert::hang_in_turned`).
#[test]
fn a_six_crossing_pair_at_the_notchs_shared_corner_builds() {
    let (m, psi) = (direction(0, 0), 2.2);
    let (notch, pinch) = pinch_pieces(m, psi);
    for (tag, r, want) in pinch_runs(m, psi) {
        if !tag.starts_with("ab") {
            continue;
        }
        let finding = r.as_ref().ok().and_then(BooleanResult::body).map(|bb| {
            pierce_point_finding(
                &bb.body,
                notch343().v,
                tag_cones(&tag, "ab", (&notch, &pinch)),
            )
        });
        let line = outcome(r, want, tol());
        assert!(line.starts_with("OK SOUND"), "{tag}: {line}");
        assert_eq!(finding, Some(None), "{tag}");
    }
}

/// **A run turned round a shared vertex holds the rest of its pair.**
/// [`notch343`] against the corner pinch at the pinch battery's poses
/// `i=0 j=2 k=2` and `i=0 j=6 k=3`. Two vertex pairs cross at the
/// notch's corner, which both share: one four times, the other twice.
/// One run of the four-crossing pair holds the other pair's cuts and
/// turns onto its complement, which holds its own pair's other run. Every
/// op in both orders builds `SOUND`, one vertex per cone at the corner,
/// on one point key, and meshing ([`pierce_point_finding`]). Red as
/// `ClassificationInvariant` ("a vertex at a shared point is the In end
/// of one null edge and the Out end of another") when the held run
/// mints at the shared vertex rather than at the turned run's copy
/// (`insert::hang_in_turned`).
#[test]
fn a_run_turned_at_a_shared_vertex_holds_the_rest_of_its_pair() {
    let v = notch343().v;
    let mut bad = Vec::new();
    for (i, j, k) in [(0, 2, 2), (0, 6, 3)] {
        let psi = f64::from(k) * 1.05 + 0.1;
        let m = direction(i, j);
        let (notch, pinch) = pinch_pieces(m, psi);
        for (tag, r, want) in pinch_runs(m, psi) {
            let finding = r
                .as_ref()
                .ok()
                .and_then(BooleanResult::body)
                .and_then(|bb| {
                    pierce_point_finding(&bb.body, v, tag_cones(&tag, "ab", (&notch, &pinch)))
                });
            let line = outcome(r, want, tol());
            if !line.starts_with("OK SOUND") || finding.is_some() {
                bad.push(format!("i={i} j={j} k={k} {tag}: {line} {finding:?}"));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} runs not SOUND at one vertex per cone:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// **A fan at a shared vertex reads the other pair's cuts from its own
/// first entry.** [`notch343`] against the corner pinch at the pinch
/// battery's poses `i=0 j=3 k=2` and `i=1 j=5 k=2`. With the pinch first
/// the notch's vertex is B's and shared; a fan's run there is weighed
/// against the other pair's cuts, which lie in its first and last
/// entries. Every op in both orders builds `SOUND`. Red as
/// `ClassificationInvariant` ("a vertex at a shared point is the In end
/// of one null edge and the Out end of another") when the run's cuts are
/// read from the orbit's first entry instead (`insert::held_cut`).
#[test]
fn a_shared_vertex_fan_reads_its_cuts_from_its_own_first_entry() {
    let mut bad = Vec::new();
    for (i, j, k) in [(0, 3, 2), (1, 5, 2)] {
        let psi = f64::from(k) * 1.05 + 0.1;
        for (tag, r, want) in pinch_runs(direction(i, j), psi) {
            let line = outcome(r, want, tol());
            if !line.starts_with("OK SOUND") {
                bad.push(format!("i={i} j={j} k={k} {tag}: {line}"));
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

/// **The pinch battery**: [`notch343`] against [`pinch_runs`]' two-cube
/// corner pinch over the sweep's 84 directions, each turned
/// `psi = 1.05 k + 0.1` for six `k`, every op in both orders. Prints one
/// [`outcome`] line per run, for a diff between two trees.
#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn pinch_runs_battery() {
    for i in 0..12 {
        for j in 0..7 {
            for k in 0..6 {
                let psi = f64::from(k) * 1.05 + 0.1;
                for (tag, r, want) in pinch_runs(direction(i, j), psi) {
                    println!("i={i} j={j} k={k} {tag}: {}", outcome(r, want, tol()));
                }
            }
        }
    }
}

/// [`notch343`] at its corner against a pinch: two cubes touching only
/// at that corner, in opposite octants of the frame, united undeclared.
/// Every op in both orders, against the pinch's two cubes clipped by
/// the notch's pieces.
fn pinch_runs(m: [f64; 3], psi: f64) -> Vec<Run> {
    let c = notch343();
    let a = finished(
        "the notch",
        fixtures::prism::<f64>(&c.profile, 1.0, tol()).body,
    );
    let f = frame(m, psi);
    let decls = BooleanDeclarations::default();
    let (c1, c2) = (
        finished("a cube", cube_at(c.v, f, [0.0; 3])),
        finished("a cube", cube_at(c.v, f, [-SIDE; 3])),
    );
    let BooleanResult::Body(pinch) = topo::union_with(&c1, &c2, &decls, tol()).unwrap() else {
        panic!("the pinch is empty");
    };
    let b = pinch.body;
    let clip = |extra: &[([f64; 3], f64)]| -> f64 {
        c.pieces
            .iter()
            .map(|p| {
                let mut all = polygon_prism(p);
                all.extend_from_slice(extra);
                convex_volume(&all)
            })
            .sum()
    };
    let va = clip(&[]);
    let vb = 2.0 * SIDE * SIDE * SIDE;
    let common =
        clip(&cube_planes_at(c.v, f, [0.0; 3])) + clip(&cube_planes_at(c.v, f, [-SIDE; 3]));
    let mut out = Vec::new();
    for (order, x, y, vx) in [("ab", &a, &b, va), ("ba", &b, &a, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vx - common),
        ];
        for (op, run, want) in ops {
            out.push((format!("{order} {op}"), run(x, y, &decls, tol()), want));
        }
    }
    out
}

/// [`pinch_runs`]' operands as convex pieces: the notch's, and the
/// pinch's two cubes.
fn pinch_pieces(m: [f64; 3], psi: f64) -> (Pieces, Pieces) {
    let c = notch343();
    let f = frame(m, psi);
    (
        c.pieces.iter().map(|p| polygon_prism(p)).collect(),
        vec![
            cube_planes_at(c.v, f, [0.0; 3]),
            cube_planes_at(c.v, f, [-SIDE; 3]),
        ],
    )
}

/// A right-handed frame, as rows.
type Frame = [[f64; 3]; 3];

/// The volume `x`'s and `y`'s convex pieces share.
fn common_volume(x: &[Vec<Plane>], y: &[Vec<Plane>]) -> f64 {
    x.iter()
        .flat_map(|p| {
            y.iter().map(move |q| {
                let mut all = p.clone();
                all.extend_from_slice(q);
                convex_volume(&all)
            })
        })
        .sum()
}

/// [`notch343`] at its corner against `pinch` (whose pieces are
/// `pinch_pieces`): every op in both orders, against the oracle, as
/// `(tag, run, want)`, with the two operands' pieces.
fn notch_against(pinch: &AtRestBody<f64>, pinch_pieces: &[Vec<Plane>]) -> (Vec<Run>, Pieces) {
    let c = notch343();
    let notch = finished(
        "the notch",
        fixtures::prism::<f64>(&c.profile, 1.0, tol()).body,
    );
    let notch_pieces: Pieces = c.pieces.iter().map(|p| polygon_prism(p)).collect();
    let volume = |ps: &[Vec<Plane>]| ps.iter().map(|p| convex_volume(p)).sum::<f64>();
    let runs = every_op(
        ["ab", "ba"],
        (&notch, volume(&notch_pieces)),
        (pinch, volume(pinch_pieces)),
        common_volume(&notch_pieces, pinch_pieces),
    );
    (runs, notch_pieces)
}

/// Unites `bodies` one by one; `None` where a union refuses.
fn unite_all(bodies: Vec<Body<f64>>) -> Option<AtRestBody<f64>> {
    let decls = BooleanDeclarations::default();
    let mut parts = bodies.into_iter().map(|b| finished("a part", b));
    let mut acc = parts.next()?;
    for b in parts {
        match topo::union_with(&acc, &b, &decls, tol()) {
            Ok(BooleanResult::Body(bb)) => acc = bb.body,
            _ => return None,
        }
    }
    Some(acc)
}

/// **Three pairs whose hang leaves the point on two keys refuse typed.**
/// [`notch343`]'s corner against three cubes whose corners touch only
/// there, two poses (PR 4249's review probes): `three i=4 j=3 k=1`
/// (side-2 cubes, diagonals 120° apart in the plane normal to the grid
/// direction) and `tripod i=0 j=4 t=1 k=0` (side-4 cubes tilted 0.25
/// toward it). In each union a run turns at the shared corner and its
/// siblings hang at its copy (`insert::hang_in_turned`), and the pinched
/// operand's own cones sit on keys no seam links
/// (`work/join/a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys.md`):
/// the union refuses `SharedVertexCrossings`, and the intersection and
/// difference build `SOUND`, one vertex per cone on one key, meshing.
/// Red as `OK BAD` (tier 3′ `CensusUndecidable`, the cones on two keys)
/// without `zip::refuse_split_hung_points`.
#[test]
fn three_pairs_whose_hang_leaves_the_point_on_two_keys_refuse_typed() {
    let v = notch343().v;
    let mut poses: Vec<(&str, Vec<Frame>, f64)> = Vec::new();
    // `three`: each cube's corner diagonal along `d`, twisted by `psi`.
    let [p, q, _] = frame(direction(4, 3), 0.3 + 0.9);
    let three = (0..3)
        .map(|t| {
            let th = f64::from(t) * std::f64::consts::TAU / 3.0;
            let d = [0, 1, 2].map(|c| th.cos() * p[c] + th.sin() * q[c]);
            let [p, q, a] = frame(d, 0.0);
            let s = (2.0f64 / 3.0).sqrt();
            let e = |k: f64| {
                let t = 0.2 + f64::from(t) + k * std::f64::consts::TAU / 3.0;
                [0, 1, 2].map(|i| a[i] / 3f64.sqrt() + s * (t.cos() * p[i] + t.sin() * q[i]))
            };
            let (e0, e1, e2) = (e(0.0), e(1.0), e(2.0));
            if dot(cross(e0, e1), e2) > 0.0 {
                [e0, e1, e2]
            } else {
                [e1, e0, e2]
            }
        })
        .collect();
    poses.push(("three i=4 j=3 k=1", three, 2.0));
    // `tripod`: each cube's corner diagonal along `d`, its frame turned.
    let n = unit(direction(0, 4));
    let [p, q, _] = frame(n, 0.0);
    let tripod = (0..3)
        .map(|c| {
            let a = std::f64::consts::TAU * f64::from(c) / 3.0;
            let d = [0, 1, 2].map(|x| a.cos() * p[x] + a.sin() * q[x] + 0.25 * n[x]);
            let [u, w, m] = frame(d, 0.3 + f64::from(c));
            let (s6, s2, s3) = (6f64.sqrt(), 2f64.sqrt(), 3f64.sqrt());
            [
                [(2.0f64 / 3.0).sqrt(), 0.0, 1.0 / s3],
                [-1.0 / s6, 1.0 / s2, 1.0 / s3],
                [-1.0 / s6, -1.0 / s2, 1.0 / s3],
            ]
            .map(|l| [0, 1, 2].map(|x| l[0] * u[x] + l[1] * w[x] + l[2] * m[x]))
        })
        .collect();
    poses.push(("tripod i=0 j=4 t=1 k=0", tripod, SIDE));
    for (pose, frames, side) in poses {
        let pinch = unite_all(
            frames
                .iter()
                .map(|&f| cube_sized(v, f, [0.0; 3], side))
                .collect(),
        )
        .unwrap_or_else(|| panic!("{pose}: the cubes do not unite"));
        let pinch_pieces: Pieces = frames
            .iter()
            .map(|&f| cube_planes_sized(v, f, [0.0; 3], side))
            .collect();
        let (runs, notch_pieces) = notch_against(&pinch, &pinch_pieces);
        for (tag, r, want) in runs.into_iter().filter(|(tag, ..)| tag.starts_with("ab")) {
            if tag == "ab U" {
                assert!(
                    matches!(r, Err(BooleanError::SharedVertexCrossings { .. })),
                    "{pose} {tag}: {}",
                    outcome(r, want, tol())
                );
                continue;
            }
            let finding = r.as_ref().ok().and_then(BooleanResult::body).map(|bb| {
                pierce_point_finding(
                    &bb.body,
                    v,
                    tag_cones(&tag, "ab", (&notch_pieces, &pinch_pieces)),
                )
            });
            let line = outcome(r, want, tol());
            assert!(line.starts_with("OK SOUND"), "{pose} {tag}: {line}");
            assert_eq!(finding, Some(None), "{pose} {tag}");
        }
    }
}

/// **A run turned round a shared vertex holds two siblings.**
/// [`notch343`]'s corner against a pinch of two 50° wedges touching
/// there, the second the first turned half a turn about its own `y`, at
/// the grid direction `i=1 j=3` turned `0.7 + 0.3 j` (PR 4249's review
/// probe `wpinch n343 a=50 i=1 j=3`). With the notch first, in the union
/// and the difference one run at the notch's corner turns and holds two
/// runs of its own pair, both hung at its copy. Every op with the notch
/// first builds `SOUND`, one vertex per cone on one key, meshing. Red as
/// `ClassificationInvariant` ("a vertex at a shared point is the In end
/// of one null edge and the Out end of another") without
/// `insert::hang_in_turned`.
#[test]
fn a_run_turned_at_a_shared_vertex_holds_two_siblings() {
    let v = notch343().v;
    let f = frame(direction(1, 3), 0.7 + 0.3 * 3.0);
    let g = [f[0].map(|t| -t), f[1], f[2].map(|t| -t)];
    let w = wedge(50.0);
    let (b1, p1) = posed(&w, f, v);
    let (b2, p2) = posed(&w, g, v);
    let BooleanResult::Body(pinch) =
        topo::union_with(&b1, &b2, &BooleanDeclarations::default(), tol()).unwrap()
    else {
        panic!("the wedges' union is empty");
    };
    let pinch_pieces: Pieces = p1.into_iter().chain(p2).collect();
    let (runs, notch_pieces) = notch_against(&pinch.body, &pinch_pieces);
    for (tag, r, want) in runs.into_iter().filter(|(tag, ..)| tag.starts_with("ab")) {
        let finding = r.as_ref().ok().and_then(BooleanResult::body).map(|bb| {
            pierce_point_finding(
                &bb.body,
                v,
                tag_cones(&tag, "ab", (&notch_pieces, &pinch_pieces)),
            )
        });
        let line = outcome(r, want, tol());
        assert!(line.starts_with("OK SOUND"), "{tag}: {line}");
        assert_eq!(finding, Some(None), "{tag}");
    }
}

/// A named pair of corners.
type CornerPair = (&'static str, fn() -> Corner, fn() -> Corner);

/// **The corner-pairs battery**: reflex corner pairs at one vertex over
/// the sweep's grid plus axis-aligned turns and exact-tie turns
/// (ψ ∈ {0, π/2, π}), every op in both orders. One [`outcome`] line
/// per run, for a diff between two trees.
#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn corner_pairs_battery() {
    let pairs: [CornerPair; 5] = [
        ("n343-n343", notch343, notch343),
        ("w343-w330", || wedge(343.0), || wedge(330.0)),
        ("w345-n343", wedge345, notch343),
        ("w300-w270", || wedge(300.0), || wedge(270.0)),
        ("w350-w200", || wedge(350.0), || wedge(200.0)),
    ];
    let mut dirs: Vec<(String, [f64; 3])> = Vec::new();
    for i in 0..12 {
        for j in 0..7 {
            dirs.push((format!("i={i} j={j}"), direction(i, j)));
        }
    }
    for (name, m) in [
        ("z", [0.0, 0.0, 1.0]),
        ("-z", [0.0, 0.0, -1.0]),
        ("x", [1.0, 0.0, 0.0]),
        ("y", [0.0, 1.0, 0.0]),
        ("xy", [1.0, 1.0, 0.0]),
        ("xyz", [1.0, 1.0, 1.0]),
        ("-xyz", [-1.0, -1.0, 1.0]),
    ] {
        dirs.push((name.to_owned(), m));
    }
    let psis = [
        0.0,
        0.7,
        2.3,
        std::f64::consts::FRAC_PI_2,
        std::f64::consts::PI,
        4.4,
    ];
    for (pname, a, b) in pairs {
        let (a, b) = (a(), b());
        for (dname, m) in &dirs {
            for psi in psis {
                let f = frame(*m, psi);
                let runs = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    corner_pair_runs(&a, &b, f)
                        .into_iter()
                        .map(|(tag, r, want)| format!("{tag}: {}", outcome(r, want, tol())))
                        .collect::<Vec<_>>()
                }));
                match runs {
                    Ok(lines) => {
                        for l in lines {
                            println!("{pname} {dname} psi={psi:.4} {l}");
                        }
                    }
                    Err(_) => println!("{pname} {dname} psi={psi:.4} PANIC"),
                }
            }
        }
    }
}

/// The L-prism ([`PROFILE`]) as a corner at `V`, tiled by [`BOXES`]'
/// footprints.
fn ltop() -> Corner {
    Corner {
        profile: PROFILE.to_vec(),
        pieces: BOXES
            .iter()
            .map(|&(l, h)| vec![(l[0], l[1]), (h[0], l[1]), (h[0], h[1]), (l[0], h[1])])
            .collect(),
        v: V,
    }
}

/// The asymmetric reflex corner: `(1, 1, 1)`, a 0° edge and a 34° one
/// round a 326° notch (PR 4139's review r1, `asym`).
fn asym() -> Corner {
    Corner {
        profile: vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (1.0, 1.0), (0.0, 1.5)],
        pieces: vec![
            vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.5)],
            vec![(1.0, 0.0), (4.0, 0.0), (4.0, 3.0), (1.0, 1.0)],
        ],
        v: [1.0, 1.0, 1.0],
    }
}

/// A seeded rotation (xorshift) as three orthonormal rows: PR 4139's
/// review r1's poses, kept bit for bit.
fn seeded_rotation(seed: u64) -> [[f64; 3]; 3] {
    let mut s = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut r = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
    };
    let a = unit([r(), r(), r()]);
    let b0 = [r(), r(), r()];
    let along = dot(a, b0);
    let b = unit([0, 1, 2].map(|t| b0[t] - along * a[t]));
    [a, b, cross(a, b)]
}

/// Fibonacci direction `i` of `n`.
fn fibonacci(i: u32, n: u32) -> [f64; 3] {
    let ga = std::f64::consts::PI * (3.0 - 5f64.sqrt());
    let z = 1.0 - 2.0 * (f64::from(i) + 0.5) / f64::from(n);
    let r = (1.0 - z * z).sqrt();
    let t = ga * f64::from(i);
    [r * t.cos(), r * t.sin(), z]
}

/// PR 4139's review r1's `dbl` pose `seed`, `fib`: corner `a` at rest
/// and corner `b` turned by a seeded rotation about it, touching only at
/// their corner `v` (their union, two vertices at `v`), and the cube whose near face
/// holds `v`, along Fibonacci direction `fib` of 600 and turned 0.4
/// about it; with the operand's pieces and volume, the cube's planes,
/// and the volume they share.
struct Dbl {
    pinched: AtRestBody<f64>,
    pieces: Pieces,
    cube: AtRestBody<f64>,
    planes: Vec<Plane>,
    volume: f64,
    common: f64,
}

fn dbl((a, b): (&Corner, &Corner), seed: u64, fib: u32) -> Dbl {
    let pose = format!("seed={seed} fib{fib}");
    let v = a.v;
    let x1 = finished(
        "a corner",
        fixtures::prism::<f64>(&a.profile, 1.0, tol()).body,
    );
    let (x2, posed_pieces) = posed(b, seeded_rotation(seed * 31 + 5), v);
    let pinched = topo::union_with(&x1, &x2, &BooleanDeclarations::default(), tol())
        .unwrap_or_else(|e| panic!("{pose}: the corners' union: {e:?}"));
    let pinched = pinched.body().expect("the union is not empty").body.clone();
    assert_eq!(
        vertices_at(&pinched, v).len(),
        2,
        "{pose}: the operand's pinch"
    );
    let mut pieces: Pieces = a.pieces.iter().map(|p| polygon_prism(p)).collect();
    pieces.extend(posed_pieces);
    let f = frame(fibonacci(fib, 600), 0.4);
    let lo = [-2.0, -2.0, 0.0];
    let cube = finished("the cube", cube_at(v, f, lo));
    let planes = cube_planes_at(v, f, lo);
    let volume: f64 = pieces.iter().map(|p| convex_volume(p)).sum();
    let common: f64 = pieces
        .iter()
        .map(|p| convex_volume(&[p.clone(), planes.clone()].concat()))
        .sum();
    assert!(
        common > 1e-3,
        "{pose}: the cube holds some of the operand: {common}"
    );
    Dbl {
        pinched,
        pieces,
        cube,
        planes,
        volume,
        common,
    }
}

/// **The output stage's join leaves a pinch's cones their vertices**
/// (`boolean::edge_join`). The union of [`dbl`]'s seed 268, direction
/// 11 holds two cones at `v`, one vertex each; one of them has two
/// edges only, collinear between one pair of planes, the shape the join
/// kills. It shares its point key with the other cone's vertex, so the
/// join leaves it: the union builds `SOUND` at the clipped volume, with
/// two vertices at `v` and no joinable vertex. Red if the join kills it:
/// the joined edge runs through the other vertex, and tier 3′ refuses
/// `UndeclaredContact { VertexOnEdge }` at `v`.
#[test]
fn the_join_stage_leaves_a_pinchs_cones_their_vertices() {
    let v = asym().v;
    let d = dbl((&asym(), &asym()), 268, 11);
    let want = d.volume + SIDE.powi(3) - d.common;
    let r = topo::union_with(&d.pinched, &d.cube, &BooleanDeclarations::default(), tol());
    let body = r
        .as_ref()
        .ok()
        .and_then(|res| res.body())
        .expect("the union builds")
        .body
        .clone();
    let line = outcome(r, want, tol());
    assert!(line.starts_with("OK SOUND"), "{line}");
    let at_v = vertices_at(&body, v);
    assert_eq!(
        cone_finding(
            &body,
            v,
            (&d.pieces, std::slice::from_ref(&d.planes), Cones::Union)
        ),
        None
    );
    assert_eq!(at_v.len(), 2, "one vertex per cone at v");
    let valence = |w| body.half_edges().filter(|(_, h)| h.start == w).count();
    assert!(
        at_v.iter().any(|&w| valence(w) == 2),
        "a cone at v is a straight edge through it: {:?}",
        at_v.iter().map(|&w| valence(w)).collect::<Vec<_>>()
    );
    assert_eq!(
        topo::joinable_vertices(
            &body,
            geom_core::Band::linear(geom_core::Tol::witness()).unwrap()
        )
        .unwrap(),
        vec![],
        "maximal edges"
    );
}

/// **A pinched operand's pierces weld only where their corners nest.**
/// The operand is two [`asym`] corners touching only at `v = (1, 1, 1)`,
/// one turned by a seeded rotation (their union, two vertices at `v`);
/// the cube's near face holds `v`, the cube along Fibonacci direction
/// `fib` of 600 and turned 0.4 about it (PR 4139's review r1, set
/// `dbl`). One corner crosses the face in two sectors and the other in
/// one between them. The face then holds a copy of the first corner's
/// pierce vertex per sector pair, and the second's pierce, on one point,
/// each with a corner of the face; the second's edges leave `v` inside
/// one copy's corner only (`finish::corners_nest`). Every op in both
/// orders builds at the pieces' clipped volume, holds one vertex per
/// cone at `v`, and meshes: the unions and cube ∖
/// operand `SOUND`; the rest fail tier 3′ only, on the vertices the cones
/// leave at `v`, a contact no declaration names (D10). Red if the weld
/// joins the second pierce to the other copy: its fan then reads two
/// cones where cube ∖ operand has one, and the split's loops wind
/// inside out (`LoopRoleInverted`).
#[test]
fn a_pinched_operands_pierces_weld_where_their_corners_nest() {
    for (seed, fib) in [(268, 11), (15, 11), (426, 6)] {
        let pose = format!("seed={seed} fib{fib}");
        let v = asym().v;
        let Dbl {
            pinched,
            pieces,
            cube,
            planes,
            volume,
            common,
        } = dbl((&asym(), &asym()), seed, fib);
        let runs = every_op(
            ["xy", "yx"],
            (&pinched, volume),
            (&cube, SIDE.powi(3)),
            common,
        );
        let cube_pieces = vec![planes];
        for (tag, r, want) in runs {
            let finding = r.as_ref().ok().and_then(BooleanResult::body).map(|bb| {
                let cones = tag_cones(&tag, "xy", (&pieces, &cube_pieces));
                cone_finding(&bb.body, v, cones).or_else(|| {
                    match mesh::tessellate(&bb.body, 0.05, tol())
                        .map(|m| mesh::validate::check_mesh(&m))
                    {
                        Ok(Ok(())) => None,
                        other => Some(format!("the body does not mesh: {other:?}")),
                    }
                })
            });
            let line = outcome(r, want, tol());
            if ["xy U", "yx U", "yx S"].contains(&tag.as_str()) {
                assert!(line.starts_with("OK SOUND"), "{pose} {tag}: {line}");
            } else {
                assert!(
                    line.starts_with("OK BAD t2=true t3p=false cert=true operand=true"),
                    "{pose} {tag}: {line}"
                );
            }
            assert_eq!(finding, Some(None), "{pose} {tag}");
        }
    }
}

/// **A pinch's cones sit on one point key** (`zip::share_points`). Six
/// of [`dbl`]'s poses whose union with the cube pinches at `v` in two
/// cones. Both cones' vertices are the pinched operand's own, which the
/// corners' union left on two keys; the cube's copies of `v` sit on a
/// third, which the zips fuse away. The seam correspondence pairs each
/// operand vertex with a cube copy, so it ties all three keys, and after
/// the zips the two cones' vertices move onto one key.
/// Both unions build `SOUND` at the clipped volume, with one vertex per
/// cone at `v`, on one key, and mesh. Red without the move: tier 3′
/// refuses `UndeclaredContact { VertexVertex }` at `v`, and the output
/// stage's join, which reads the pinch from its keys, kills a cone's
/// vertex (one vertex for two cones; the mesher refuses or panics).
/// The unions rebind at least one class (a union whose zips already
/// leave its cones on one key rebinds none), and every class rebound
/// held one point, bit for bit (`topo::take_shared_points`): the
/// rebind reads no position, so this is its premise's pin.
#[test]
fn a_pinchs_cones_share_one_point_key() {
    let mut rebound = 0;
    for (names, (a, b), seed, fib) in [
        ("Ltop asym", (ltop(), asym()), 2296, 21),
        ("Ltop asym", (ltop(), asym()), 2296, 3),
        ("Ltop asym", (ltop(), asym()), 2296, 8),
        ("Ltop asym", (ltop(), asym()), 2959, 3),
        ("asym asym", (asym(), asym()), 15, 6),
        ("asym asym", (asym(), asym()), 225, 6),
    ] {
        let pose = format!("{names} seed={seed} fib{fib}");
        let v = a.v;
        let d = dbl((&a, &b), seed, fib);
        let want = d.volume + SIDE.powi(3) - d.common;
        let cube_pieces = vec![d.planes.clone()];
        for (order, x, y) in [("xy", &d.pinched, &d.cube), ("yx", &d.cube, &d.pinched)] {
            shared_point_spread_finding();
            let r = topo::union_with(x, y, &BooleanDeclarations::default(), tol());
            let (classes, spread) = shared_point_spread_finding();
            rebound += classes;
            assert_eq!(spread, None, "{pose} {order} U");
            let finding = r.as_ref().ok().and_then(BooleanResult::body).map(|bb| {
                point_key_finding(&bb.body, v)
                    .or_else(|| cone_finding(&bb.body, v, (&d.pieces, &cube_pieces, Cones::Union)))
                    .or_else(|| {
                        match mesh::tessellate(&bb.body, 0.05, tol())
                            .map(|m| mesh::validate::check_mesh(&m))
                        {
                            Ok(Ok(())) => None,
                            other => Some(format!("the body does not mesh: {other:?}")),
                        }
                    })
            });
            let line = outcome(r, want, tol());
            assert!(line.starts_with("OK SOUND"), "{pose} {order} U: {line}");
            assert_eq!(finding, Some(None), "{pose} {order} U");
        }
    }
    assert!(rebound > 0, "no union rebound a class");
}

/// The near-tangent corners: the L-prism's, review r1's `vee300` and
/// `asym` notches, and the 345° and 60° wedges.
fn near_tangent_corners() -> [(&'static str, Corner); 5] {
    let l = |pieces: Vec<Vec<(f64, f64)>>| Corner {
        profile: PROFILE.to_vec(),
        pieces,
        v: V,
    };
    [
        (
            "Ltop",
            l(vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
                vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
            ]),
        ),
        (
            "vee300",
            Corner {
                profile: vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5), (0.0, 4.0)],
                pieces: vec![
                    vec![(0.0, 0.0), (2.0, 0.0), (2.0, 0.5), (0.0, 4.0)],
                    vec![(2.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5)],
                ],
                v: [2.0, 0.5, 1.0],
            },
        ),
        (
            "asym",
            Corner {
                profile: vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (1.0, 1.0), (0.0, 1.5)],
                pieces: vec![
                    vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.5)],
                    vec![(1.0, 0.0), (4.0, 0.0), (4.0, 3.0), (1.0, 1.0)],
                ],
                v: [1.0, 1.0, 1.0],
            },
        ),
        ("w345", wedge345()),
        ("w60", wedge(60.0)),
    ]
}

/// The near-tangent cube for corner `c`: its near face's plane holds `v`
/// and is tilted `d` off the corner's edge `e` (0 and 1 its top edges,
/// before and after `v` in the profile, 2 the vertical edge), the
/// normal at `al` eighths of a turn about the edge.
fn near_tangent_frame(c: &Corner, e: usize, al: u32, d: f64) -> [[f64; 3]; 3] {
    let n = c.profile.len();
    let i = c
        .profile
        .iter()
        .position(|&(a, b)| a == c.v[0] && b == c.v[1])
        .unwrap();
    let (p, a, b) = (
        c.profile[i],
        c.profile[(i + n - 1) % n],
        c.profile[(i + 1) % n],
    );
    let edge = unit(
        [
            [a.0 - p.0, a.1 - p.1, 0.0],
            [b.0 - p.0, b.1 - p.1, 0.0],
            [0.0, 0.0, -1.0],
        ][e],
    );
    let [p1, p2, _] = frame(edge, 0.0);
    let th = std::f64::consts::TAU * (f64::from(al) + 0.25) / 8.0;
    let m = [0, 1, 2].map(|k| th.cos() * p1[k] + th.sin() * p2[k] + d * edge[k]);
    frame(m, 0.7)
}

/// Every op in both orders between corner `c`'s prism and the cube of
/// side [`SIDE`] at `c.v` in frame `f`, placed by `lo`.
fn corner_runs(c: &Corner, f: [[f64; 3]; 3], lo: [f64; 3]) -> Vec<Run> {
    let a = finished(
        "the corner",
        fixtures::prism::<f64>(&c.profile, 1.0, tol()).body,
    );
    let b = finished("the cube", cube_at(c.v, f, lo));
    let clip = |extra: &[([f64; 3], f64)]| -> f64 {
        c.pieces
            .iter()
            .map(|p| {
                let mut all = polygon_prism(p);
                all.extend_from_slice(extra);
                convex_volume(&all)
            })
            .sum()
    };
    let common = clip(&cube_planes_at(c.v, f, lo));
    every_op(["ac", "ca"], (&a, clip(&[])), (&b, SIDE.powi(3)), common)
}

/// [`corner_runs`]'s operands as convex pieces.
fn corner_pieces(c: &Corner, f: [[f64; 3]; 3], lo: [f64; 3]) -> (Pieces, Pieces) {
    (
        c.pieces.iter().map(|p| polygon_prism(p)).collect(),
        vec![cube_planes_at(c.v, f, lo)],
    )
}

/// **Near-tangent pierces** (PR 4139's review r1, its `nt` set): each
/// corner against the cube whose near face is tilted ±1e-3, ±1e-5 or
/// 1e-7 off one of the corner's three edges, `v` inside the face or on
/// its edge. Every op in both orders prints its [`outcome`], and in the
/// face placement its [`pierce_point_finding`] at `v`. On the cube's
/// edge, the cube-first ops hold `v` as the edge's split, ulps off `v`,
/// which the exact point match does not read.
///
/// `cargo test -p sweep --release --test all near_tangent_battery --
/// --ignored --nocapture`, on two trees, and diff the lines.
#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn near_tangent_battery() {
    for (name, c) in near_tangent_corners() {
        for e in 0..3 {
            for al in 0..8 {
                for d in [1e-3, -1e-3, 1e-5, -1e-5, 1e-7] {
                    for (place, lo, _) in &PLACEMENTS[..2] {
                        let f = near_tangent_frame(&c, e, al, d);
                        let (x, y) = corner_pieces(&c, f, *lo);
                        for (tag, r, want) in corner_runs(&c, f, *lo) {
                            let body = r.as_ref().ok().and_then(BooleanResult::body);
                            let at = body.filter(|_| *place == "face").map(|bb| {
                                pierce_point_finding(&bb.body, c.v, tag_cones(&tag, "ac", (&x, &y)))
                                    .unwrap_or_else(|| "one vertex per cone".into())
                            });
                            println!(
                                "{name} nt e{e} a{al} d{d:e} {place} {tag}: {} | {}",
                                outcome(r, want, tol()),
                                at.unwrap_or_default()
                            );
                        }
                    }
                }
            }
        }
    }
}

/// **A near-tangent sliver is a cone of its own.** The 345° wedge's
/// corner on the face of a cube tilted 1e-7 off the wedge's edge at 345°
/// ([`near_tangent_battery`]'s `w345 nt e0 a0 d1e-7 face`): the face's
/// plane crosses the wedge's top face 1e-7 rad inside that edge, so the
/// intersection holds, beside its main lump, a sliver about 2e-7 wide and
/// 1e-6 deep along the edge, which meets the lump only at `v`. Two cones
/// at `v`, so the intersection builds in both orders at the clipped
/// volume, two solids, two vertices at `v` on one point key, and meshes.
/// Red if the counter steps over the sliver's cell round `v` (reading one
/// cone), or if the boolean drops the sliver or fuses its corner with the
/// lump's.
#[test]
fn a_near_tangent_sliver_is_a_cone_of_its_own() {
    let (_, c) = near_tangent_corners()
        .into_iter()
        .find(|(name, _)| *name == "w345")
        .unwrap();
    let lo = PLACEMENTS[0].1;
    // The battery's tilt is 1e-7, a sliver about 2e-7 wide: two hundred
    // bands at the default ε, but under one band at ε = 1e-6, where the
    // sliver is honestly one solid with the lump. So the tilt is never
    // less than 100 ε, which keeps it two hundred bands wide there.
    let f = near_tangent_frame(&c, 0, 0, (100.0 * tol().eps()).max(1e-7));
    let (x, y) = corner_pieces(&c, f, lo);
    assert_eq!(
        cones_at(c.v, &x, &y, Cones::Intersect),
        Ok((2, 0)),
        "the intersection's cones at v"
    );
    for (tag, r, want) in corner_runs(&c, f, lo) {
        if !tag.ends_with('I') {
            continue;
        }
        let Some(bb) = r.as_ref().ok().and_then(BooleanResult::body) else {
            panic!("{tag}: the intersection did not build: {r:?}");
        };
        assert_eq!(
            bb.body.solids().count(),
            2,
            "{tag}: the lump and the sliver"
        );
        assert_eq!(
            pierce_point_finding(&bb.body, c.v, tag_cones(&tag, "ac", (&x, &y))),
            None,
            "{tag} at v"
        );
        let line = outcome(r, want, tol());
        assert!(line.starts_with("OK "), "{tag}: {line}");
    }
}
