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
fn pierce_point_finding(body: &Body<f64>) -> Option<String> {
    let at_v: Vec<_> = body
        .vertex_points()
        .filter(|(_, p)| p.as_ref().is_ok_and(|p| [p.x, p.y, p.z] == V))
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
/// meets it. Refusals pass: the residue is the filed rows'.
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
                            res.body().and_then(|bb| pierce_point_finding(&bb.body))
                        }
                        _ => None,
                    };
                    let line = outcome(r, want, tol());
                    if line.starts_with("OK") && !line.starts_with("OK SOUND")
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
            let p = topo::readback::vertex_point_ref(&bb.body, k).unwrap();
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
