//! **Reviewer probes for PR 6070** (`OnDeferred::sided`): one-carrier
//! pairs answered by the shells' sides. Each op's body is checked against
//! an oracle that does not go through the kernel's classification: the
//! operands' analytic membership, sampled on a grid and compared with a
//! ray-parity read of the result's tessellation, plus the mesh's own
//! divergence volume against the closed form where there is one. A
//! typed refusal passes; a wrong body fails.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{finished, revolved_about_y};
use topo::{Body, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn eps() -> f64 {
    std::env::var("CAD_TOLERANCE_EPS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1e-9)
}

type Member = fn(&Probe, [f64; 3]) -> bool;

/// An operand: its body and its analytic membership.
struct Probe {
    body: Body<f64>,
    /// Rod spans `(r, y0, y1)` unioned, minus balls `(c, r)` and minus
    /// x-drills `(y, r)`, the whole turned by `turn` about y (membership
    /// is turn-invariant for everything here but the drill, which is
    /// only ever used unturned).
    rods: Vec<(f64, f64, f64)>,
    voids: Vec<(f64, f64)>,
    /// Box pockets `(min, max)`.
    drills: Vec<([f64; 3], [f64; 3])>,
}

fn inside(p: &Probe, q: [f64; 3]) -> bool {
    let [x, y, z] = q;
    let rr = x * x + z * z;
    p.rods
        .iter()
        .any(|&(r, y0, y1)| rr < r * r && y > y0 && y < y1)
        && !p
            .voids
            .iter()
            .any(|&(c, r)| x * x + (y - c).powi(2) + z * z < r * r)
        && !p
            .drills
            .iter()
            .any(|&(lo, hi)| (0..3).all(|k| q[k] > lo[k] && q[k] < hi[k]))
}

fn pt(p: (f64, f64)) -> Point2<f64> {
    Point2::new(p.0, p.1)
}

fn ccw_bulge(c: (f64, f64), p: (f64, f64), q: (f64, f64)) -> f64 {
    let a0 = (p.1 - c.1).atan2(p.0 - c.0);
    let a1 = (q.1 - c.1).atan2(q.0 - c.0);
    let mut sweep = a1 - a0;
    while sweep <= 0.0 {
        sweep += 2.0 * core::f64::consts::PI;
    }
    (sweep / 4.0).tan()
}

fn rod_body(r: f64, y0: f64, y1: f64) -> Body<f64> {
    revolved_about_y(
        [(0.0, y0), (r, y0), (r, y1), (0.0, y1)]
            .iter()
            .map(|&p| (pt(p), 0.0))
            .collect(),
        sweep::Revolution::Full,
        tol(),
    )
}

fn ball_body(c: f64, r: f64) -> Body<f64> {
    revolved_about_y(
        vec![
            (
                pt((0.0, c - r)),
                ccw_bulge((0.0, c), (0.0, c - r), (0.0, c + r)),
            ),
            (pt((0.0, c + r)), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    )
}

fn turned(b: &Body<f64>, angle: f64) -> Body<f64> {
    if angle == 0.0 {
        return b.clone();
    }
    let map =
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), angle);
    topo::transform_rigid(b, &geom_core::test_support::rigid(&map, tol()), tol()).unwrap()
}

fn rod(r: f64, y0: f64, y1: f64, turn: f64) -> Probe {
    Probe {
        body: turned(&rod_body(r, y0, y1), turn),
        rods: vec![(r, y0, y1)],
        voids: vec![],
        drills: vec![],
    }
}

fn result_body(r: Result<BooleanResult<f64>, topo::BooleanError>, what: &str) -> Body<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b.body.into_body(),
        other => panic!("{what}: the fixture builds: {other:?}"),
    }
}

/// A rod of `(r, y0, y1)` with a ball void `(c, rv)`.
fn voided_rod(r: f64, y0: f64, y1: f64, c: f64, rv: f64) -> Probe {
    let a = finished("rod", rod_body(r, y0, y1), tol());
    let s = finished("ball", ball_body(c, rv), tol());
    Probe {
        body: result_body(topo::subtract(&a, &s, tol()), "voided rod"),
        rods: vec![(r, y0, y1)],
        voids: vec![(c, rv)],
        drills: vec![],
    }
}

/// A rod of `(r, y0, y1)` pocketed by a box across its seam at height
/// `c`, of half-height `h`: the wall face carries a hole.
fn drilled_rod(r: f64, y0: f64, y1: f64, c: f64, h: f64) -> Probe {
    let a = finished("rod", rod_body(r, y0, y1), tol());
    let (lo, hi) = ([r - 0.1, c - h, -0.1], [r + 0.2, c + h, 0.1]);
    let d = sweep::test_support::brick((lo[0], hi[0]), (lo[1], hi[1]), (lo[2], hi[2]), tol());
    let d = finished("pocket", d, tol());
    Probe {
        body: result_body(topo::subtract(&a, &d, tol()), "pocketed rod"),
        rods: vec![(r, y0, y1)],
        voids: vec![],
        drills: vec![(lo, hi)],
    }
}

/// Ray parity through the triangles, along an irrational direction.
fn mesh_inside(pos: &[[f64; 3]], tris: &[[u32; 3]], q: [f64; 3]) -> bool {
    let d = [0.5773, 0.6123, 0.5401];
    let mut n = 0;
    for t in tris {
        let [a, b, c] = t.map(|i| pos[i as usize]);
        let e1 = sub(b, a);
        let e2 = sub(c, a);
        let h = cross(d, e2);
        let det = dot(e1, h);
        if det.abs() < 1e-14 {
            continue;
        }
        let s = sub(q, a);
        let u = dot(s, h) / det;
        if !(0.0..=1.0).contains(&u) {
            continue;
        }
        let qq = cross(s, e1);
        let v = dot(d, qq) / det;
        if v < 0.0 || u + v > 1.0 {
            continue;
        }
        if dot(e2, qq) / det > 0.0 {
            n += 1;
        }
    }
    n % 2 == 1
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
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

/// The mesh of `body`: positions, triangles, divergence volume.
fn mesh_of(body: &Body<f64>) -> Option<(Vec<[f64; 3]>, Vec<[u32; 3]>, f64)> {
    let m = match mesh::tessellate(body, 2e-3, tol()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("  no mesh oracle: {e:?}");
            return None;
        }
    };
    let pos: Vec<[f64; 3]> = m.positions.iter().map(|p| [p.x, p.y, p.z]).collect();
    let tris: Vec<[u32; 3]> = m
        .patches
        .iter()
        .flat_map(|p| p.triangles.iter().copied())
        .collect();
    let vol = tris
        .iter()
        .map(|t| {
            let [a, b, c] = t.map(|i| pos[i as usize]);
            dot(a, cross(b, c)) / 6.0
        })
        .sum();
    Some((pos, tris, vol))
}

/// Every op of `(a, b)` against the analytic oracle, `want_vol` the
/// closed-form volumes (`∪, ∩, A∖B, B∖A`) where known. Returns the
/// failures; prints every verdict.
fn probe(label: &str, a: &Probe, b: &Probe, want_vol: Option<[f64; 4]>) -> Vec<String> {
    let (fa, fb) = (
        finished("A", a.body.clone(), tol()),
        finished("B", b.body.clone(), tol()),
    );
    let ops: [(&str, fn(bool, bool) -> bool, _); 4] = [
        ("A ∪ B", |x, y| x || y, topo::union(&fa, &fb, tol())),
        ("A ∩ B", |x, y| x && y, topo::intersect(&fa, &fb, tol())),
        ("A ∖ B", |x, y| x && !y, topo::subtract(&fa, &fb, tol())),
        ("B ∖ A", |x, y| y && !x, topo::subtract(&fb, &fa, tol())),
    ];
    let m: Member = inside;
    let mut wrong = Vec::new();
    for (k, (op, f, r)) in ops.into_iter().enumerate() {
        let want = want_vol.map(|w| w[k]);
        let body = match r {
            Err(e) => {
                eprintln!("{label} {op}: refused {e:?}");
                continue;
            }
            Ok(BooleanResult::Empty) => None,
            Ok(BooleanResult::Body(bb)) => {
                let t3 = topo::validate_geometric(&bb.body, tol());
                let t3p = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol());
                if t3.is_err() || t3p.is_err() {
                    wrong.push(format!("{label} {op}: unsound {t3:?} {t3p:?}"));
                }
                let kv = topo::test_support::pinned_volume("probe", &bb.body, tol()).volume;
                if let Some(w) = want
                    && (kv - w).abs() > 1e-6 * w.max(1.0)
                {
                    wrong.push(format!("{label} {op}: kernel volume {kv} want {w}"));
                }
                Some(bb.body)
            }
        };
        let mesh = match body.as_ref().map(|b| mesh_of(b)) {
            Some(None) => {
                eprintln!("{label} {op}: built, membership not probed");
                continue;
            }
            m => m.flatten(),
        };
        let mut bad = 0;
        let mut seen = 0;
        let n = 14;
        // Grid over [-0.6, 0.6] × [-1.1, 3.1] × [-0.6, 0.6], skipping
        // points within `h` of the analytic boundary.
        let h = 0.01;
        for i in 0..n {
            for j in 0..3 * n {
                for l in 0..n {
                    let q = [
                        -0.6 + 1.2 * (i as f64 + 0.37) / n as f64,
                        -1.1 + 4.2 * (j as f64 + 0.41) / (3 * n) as f64,
                        -0.6 + 1.2 * (l as f64 + 0.29) / n as f64,
                    ];
                    let truth = |q: [f64; 3]| f(m(a, q), m(b, q));
                    let t = truth(q);
                    let near = [
                        [h, 0., 0.],
                        [-h, 0., 0.],
                        [0., h, 0.],
                        [0., -h, 0.],
                        [0., 0., h],
                        [0., 0., -h],
                    ]
                    .iter()
                    .any(|d| truth([q[0] + d[0], q[1] + d[1], q[2] + d[2]]) != t);
                    if near {
                        continue;
                    }
                    seen += 1;
                    let got = mesh.as_ref().is_some_and(|(p, t, _)| mesh_inside(p, t, q));
                    if got != t {
                        bad += 1;
                    }
                }
            }
        }
        let mv = mesh.as_ref().map_or(0.0, |m| m.2);
        eprintln!(
            "{label} {op}: built, mesh volume {mv:.6}, want {want:?}, membership mismatches {bad}/{seen}"
        );
        if bad > 0 {
            wrong.push(format!("{label} {op}: {bad}/{seen} membership mismatches"));
        }
        if let Some(w) = want
            && (mv - w).abs() > 1e-2 * w.max(1.0)
        {
            wrong.push(format!("{label} {op}: mesh volume {mv} want {w}"));
        }
    }
    wrong
}

fn rods_vol(r: f64, l: f64) -> f64 {
    core::f64::consts::PI * r * r * l
}

/// Short, inner, partial-overlap and equal rods, with A turned about y
/// so its seams and rim vertices sit off B's (π puts each rim's
/// midpoint on the other's seam).
#[test]
fn turned_rods_sharing_a_wall() {
    let v = |l| rods_vol(0.5, l);
    let mut wrong = Vec::new();
    for turn in [
        0.0,
        core::f64::consts::PI,
        core::f64::consts::FRAC_PI_2,
        0.3,
    ] {
        for (name, (y0, y1), (z0, z1), w) in [
            (
                "short vs long",
                (0.0, 1.0),
                (0.0, 2.0),
                [v(2.0), v(1.0), 0.0, v(1.0)],
            ),
            (
                "inner vs long",
                (0.5, 1.5),
                (0.0, 2.0),
                [v(2.0), v(1.0), 0.0, v(1.0)],
            ),
            (
                "partial",
                (0.5, 2.5),
                (0.0, 2.0),
                [v(2.5), v(1.5), v(0.5), v(0.5)],
            ),
            ("equal", (0.0, 2.0), (0.0, 2.0), [v(2.0), v(2.0), 0.0, 0.0]),
            (
                "long vs short",
                (0.0, 2.0),
                (0.0, 1.0),
                [v(2.0), v(1.0), v(1.0), 0.0],
            ),
        ] {
            let a = rod(0.5, y0, y1, turn);
            let b = rod(0.5, z0, z1, 0.0);
            wrong.extend(probe(&format!("{name} turn {turn}"), &a, &b, Some(w)));
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// Rims a hair apart, in and around the band: B's top rim `δ` below A's,
/// and a partial overlap of `δ` at both ends, for `δ` a multiple of ε.
#[test]
fn rims_in_band_of_each_other() {
    let e = eps();
    let v = |l| rods_vol(0.5, l);
    let mut wrong = Vec::new();
    for k in [0.3, 1.0, 3.0, 30.0, 1e3] {
        let d = k * e;
        wrong.extend(probe(
            &format!("near-equal short δ={d:e}"),
            &rod(0.5, 0.0, 2.0 - d, 0.0),
            &rod(0.5, 0.0, 2.0, 0.0),
            Some([v(2.0), v(2.0 - d), 0.0, v(d)]),
        ));
        wrong.extend(probe(
            &format!("near-equal shifted δ={d:e}"),
            &rod(0.5, d, 2.0 + d, 0.0),
            &rod(0.5, 0.0, 2.0, 0.0),
            Some([v(2.0 + d), v(2.0 - d), v(d), v(d)]),
        ));
        wrong.extend(probe(
            &format!("near-equal shifted turned δ={d:e}"),
            &rod(0.5, d, 2.0 + d, core::f64::consts::PI),
            &rod(0.5, 0.0, 2.0, 0.0),
            Some([v(2.0 + d), v(2.0 - d), v(d), v(d)]),
        ));
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// A deferred face with a hole: a long rod pocketed by a box across its
/// seam at `y = 1`, against short rods whose rim runs below the hole,
/// through it, and above it, both orders.
#[test]
fn a_drilled_wall_against_a_short_rod() {
    let (c, h) = (1.0, 0.1);
    let a = drilled_rod(0.5, 0.0, 2.0, c, h);
    // The pocket's section of the disc, by the midpoint rule.
    let n = 4000;
    let (dx, dz) = (0.3 / n as f64, 0.2 / n as f64);
    let mut s = 0.0;
    for i in 0..n {
        for k in 0..n {
            let x = 0.4 + (i as f64 + 0.5) * dx;
            let z = -0.1 + (k as f64 + 0.5) * dz;
            if x * x + z * z < 0.25 {
                s += dx * dz;
            }
        }
    }
    let disc = core::f64::consts::PI * 0.25;
    let va = 2.0 * disc - s * 2.0 * h;
    let mut wrong = Vec::new();
    for top in [0.7, 1.0, 1.5] {
        let vb = disc * top;
        let lap = (top.min(c + h) - (c - h).max(0.0)).max(0.0);
        let vi = vb - s * lap;
        let w = [va + vb - vi, vi, va - vi, vb - vi];
        let b = rod(0.5, 0.0, top, 0.0);
        wrong.extend(probe(&format!("drilled vs rod to {top}"), &a, &b, Some(w)));
        wrong.extend(probe(
            &format!("rod to {top} vs drilled"),
            &b,
            &a,
            Some([w[0], w[1], w[3], w[2]]),
        ));
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// Operands of two shells: a rod with a ball void, inside and around a
/// rod sharing its wall.
#[test]
fn voided_rods_sharing_a_wall() {
    let mut wrong = Vec::new();
    for (name, a, b) in [
        (
            "long voided vs short",
            voided_rod(0.5, 0.0, 2.0, 1.5, 0.2),
            rod(0.5, 0.0, 1.0, 0.0),
        ),
        (
            "short voided vs long",
            voided_rod(0.5, 0.0, 1.0, 0.5, 0.2),
            rod(0.5, 0.0, 2.0, 0.0),
        ),
        (
            "both voided",
            voided_rod(0.5, 0.0, 1.0, 0.5, 0.2),
            voided_rod(0.5, 0.0, 2.0, 1.5, 0.2),
        ),
        (
            "long voided in the short's span",
            voided_rod(0.5, 0.0, 2.0, 0.5, 0.2),
            rod(0.5, 0.0, 1.0, 0.0),
        ),
    ] {
        wrong.extend(probe(name, &a, &b, None));
        wrong.extend(probe(&format!("{name}, swapped"), &b, &a, None));
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// A body of rod bands `(r, y0, y1)`, stacked bottom to top, revolved.
fn stack(bands: &[(f64, f64, f64)]) -> Probe {
    let mut pts = vec![(0.0, bands[0].1)];
    for &(r, y0, y1) in bands {
        pts.push((r, y0));
        pts.push((r, y1));
    }
    pts.push((0.0, bands.last().unwrap().2));
    Probe {
        body: revolved_about_y(
            pts.iter().map(|&p| (pt(p), 0.0)).collect(),
            sweep::Revolution::Full,
            tol(),
        ),
        rods: bands.to_vec(),
        voids: vec![],
        drills: vec![],
    }
}

/// A short rod with a mid-length flange or groove against the long rod
/// on its wall: the flange turns both shells across the shared band, the
/// groove leaves each one-sided.
#[test]
fn flanged_and_grooved_short_rods() {
    let v = |r: f64, l: f64| core::f64::consts::PI * r * r * l;
    let long = rod(0.5, 0.0, 2.0, 0.0);
    let flange = stack(&[(0.5, 0.0, 0.4), (1.0, 0.4, 0.6), (0.5, 0.6, 1.0)]);
    let groove = stack(&[(0.5, 0.0, 0.4), (0.3, 0.4, 0.6), (0.5, 0.6, 1.0)]);
    let vf = v(0.5, 0.8) + v(1.0, 0.2);
    let vg = v(0.5, 0.8) + v(0.3, 0.2);
    let vl = v(0.5, 2.0);
    let fl_in = v(0.5, 1.0);
    let mut wrong = Vec::new();
    for (name, s, vs, i) in [("flange", &flange, vf, fl_in), ("groove", &groove, vg, vg)] {
        let w = [vs + vl - i, i, vs - i, vl - i];
        wrong.extend(probe(name, s, &long, Some(w)));
        wrong.extend(probe(
            &format!("{name}, swapped"),
            &long,
            s,
            Some([w[0], w[1], w[3], w[2]]),
        ));
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
