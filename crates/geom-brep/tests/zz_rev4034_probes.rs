//! Review probes for PR 4034 (ssi/neighbour-cap). Not for merge.
//!
//! Every fixture is a graph wall `S(u, v) = (x(u,v), y(u,v), z(u,v))`
//! cut by the plane `z = 0`, whose locus in the wall's chart is the
//! zero set of `Σ wᵢⱼ zᵢⱼ Nᵢⱼ(u, v)`. The truth is an independent
//! contour tracer in the chart (tiny adaptive steps, re-traced at a
//! quarter of the step to confirm), from every boundary crossing found
//! by a dense scan of the four sides. Every certified branch must pair
//! its two ends as the truth does and lie along that truth branch.
#![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used, missing_docs)]

use std::collections::{BTreeSet, HashMap};
use std::time::Instant;

use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, BranchEnd, SsiDomain, SsiError};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Vec3};

// ---------- Bernstein interpolation ----------

fn binom(n: usize, k: usize) -> f64 {
    let mut r = 1.0;
    for i in 0..k {
        r *= (n - i) as f64 / (i + 1) as f64;
    }
    r
}

fn bern(n: usize, i: usize, t: f64) -> f64 {
    binom(n, i) * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32)
}

fn solve(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Vec<f64> {
    let n = b.len();
    for c in 0..n {
        let p = (c..n)
            .max_by(|&i, &j| a[i][c].abs().total_cmp(&a[j][c].abs()))
            .unwrap();
        a.swap(c, p);
        b.swap(c, p);
        for r in 0..n {
            if r != c {
                let f = a[r][c] / a[c][c];
                for k in c..n {
                    a[r][k] -= f * a[c][k];
                }
                b[r] -= f * b[c];
            }
        }
    }
    (0..n).map(|i| b[i] / a[i][i]).collect()
}

/// Bernstein coefficients `c[i][j]` of `f` at degrees `(p, q)` (exact
/// when `f` is a polynomial of at most those degrees).
fn bernstein(f: &dyn Fn(f64, f64) -> f64, p: usize, q: usize) -> Vec<Vec<f64>> {
    let nodes = |n: usize| -> Vec<f64> {
        (0..=n)
            .map(|k| if n == 0 { 0.5 } else { k as f64 / n as f64 })
            .collect()
    };
    let (us, vs) = (nodes(p), nodes(q));
    let mu: Vec<Vec<f64>> = us
        .iter()
        .map(|&u| (0..=p).map(|i| bern(p, i, u)).collect())
        .collect();
    let mv: Vec<Vec<f64>> = vs
        .iter()
        .map(|&v| (0..=q).map(|j| bern(q, j, v)).collect())
        .collect();
    // Solve in v for each u node, then in u.
    let mut g = vec![vec![0.0; q + 1]; p + 1];
    for (a, &u) in us.iter().enumerate() {
        let rhs: Vec<f64> = vs.iter().map(|&v| f(u, v)).collect();
        g[a] = solve(mv.clone(), rhs);
    }
    let mut c = vec![vec![0.0; q + 1]; p + 1];
    for j in 0..=q {
        let rhs: Vec<f64> = (0..=p).map(|a| g[a][j]).collect();
        let col = solve(mu.clone(), rhs);
        for i in 0..=p {
            c[i][j] = col[i];
        }
    }
    c
}

struct Variant {
    name: &'static str,
    /// Weight at net index (i, j) of an (p+1)×(q+1) net.
    weight: fn(usize, usize, usize, usize) -> f64,
    /// x, y warp at (i, j).
    warp: fn(f64, f64) -> (f64, f64),
    /// Knots inserted in u (very short spans).
    insert_u: &'static [f64],
}

fn w_one(_: usize, _: usize, _: usize, _: usize) -> f64 {
    1.0
}
fn w_mixed(i: usize, j: usize, _: usize, _: usize) -> f64 {
    [2.0, 0.5, 1.0, 4.0, 0.7, 3.0, 0.4][(i * 3 + j * 5) % 7]
}
fn w_extreme(i: usize, j: usize, _: usize, _: usize) -> f64 {
    [20.0, 0.05, 1.0, 8.0, 0.1, 30.0, 0.03][(i * 3 + j * 5) % 7]
}
fn no_warp(x: f64, y: f64) -> (f64, f64) {
    (x, y)
}
fn bend_warp(x: f64, y: f64) -> (f64, f64) {
    (x + 0.3 * (y - 0.5) * (y - 0.5), y + 0.25 * x * (1.0 - x))
}
/// Nearly-degenerate axes: the v direction tilted nearly onto u.
fn shear_warp(x: f64, y: f64) -> (f64, f64) {
    (x + 0.98 * y, 0.05 * y)
}

const VARIANTS: [Variant; 6] = [
    Variant {
        name: "plain",
        weight: w_one,
        warp: no_warp,
        insert_u: &[],
    },
    Variant {
        name: "bent",
        weight: w_one,
        warp: bend_warp,
        insert_u: &[],
    },
    Variant {
        name: "rational",
        weight: w_mixed,
        warp: no_warp,
        insert_u: &[],
    },
    Variant {
        name: "rational-extreme+bent",
        weight: w_extreme,
        warp: bend_warp,
        insert_u: &[],
    },
    Variant {
        name: "near-degenerate-axes",
        weight: w_one,
        warp: shear_warp,
        insert_u: &[],
    },
    Variant {
        name: "short-spans",
        weight: w_mixed,
        warp: bend_warp,
        insert_u: &[0.3, 0.3 + 1e-7, 0.5, 0.5 + 1e-9, 0.7],
    },
];

fn wall(f: &dyn Fn(f64, f64) -> f64, p: usize, q: usize, var: &Variant) -> NurbsSurface<f64> {
    let c = bernstein(f, p, q);
    let mut ku = vec![0.0; p + 1];
    ku.extend(vec![1.0; p + 1]);
    let mut kv = vec![0.0; q + 1];
    kv.extend(vec![1.0; q + 1]);
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for i in 0..=p {
        for j in 0..=q {
            let w = (var.weight)(i, j, p, q);
            let (x, y) = (var.warp)(i as f64 / p as f64, j as f64 / q.max(1) as f64);
            control.push(Point3::new(x, y, c[i][j] / w));
            weights.push(w);
        }
    }
    let s = NurbsSurface::new(
        KnotVector::clamped(ku, p).unwrap(),
        KnotVector::clamped(kv, q).unwrap(),
        control,
        weights,
    )
    .unwrap();
    if var.insert_u.is_empty() {
        s
    } else {
        s.refine_knots_u(var.insert_u).unwrap()
    }
}

/// B-spline basis N_{i,p}(t) by Cox–de Boor (right end closed).
fn bspline_basis(knots: &[f64], p: usize, i: usize, t: f64) -> f64 {
    if p == 0 {
        let last = knots[knots.len() - 1];
        let (a, b) = (knots[i], knots[i + 1]);
        return if (a <= t && t < b) || (t == last && b == last && a < b) {
            1.0
        } else {
            0.0
        };
    }
    let mut r = 0.0;
    let d1 = knots[i + p] - knots[i];
    if d1 > 0.0 {
        r += (t - knots[i]) / d1 * bspline_basis(knots, p - 1, i, t);
    }
    let d2 = knots[i + p + 1] - knots[i + 1];
    if d2 > 0.0 {
        r += (knots[i + p + 1] - t) / d2 * bspline_basis(knots, p - 1, i + 1, t);
    }
    r
}

/// A graph wall `(u, v, f(u, v))` whose `u` direction is a spline of
/// degree `pu` on `ku` (interpolated at Greville points, exact when `f`
/// lies in the space) and `v` a Bézier of degree `q`.
fn spline_wall(f: &dyn Fn(f64, f64) -> f64, ku: &[f64], pu: usize, q: usize) -> NurbsSurface<f64> {
    let n = ku.len() - pu - 1;
    let grev: Vec<f64> = (0..n)
        .map(|i| ku[i + 1..=i + pu].iter().sum::<f64>() / pu as f64)
        .collect();
    let mu: Vec<Vec<f64>> = grev
        .iter()
        .map(|&u| (0..n).map(|i| bspline_basis(ku, pu, i, u)).collect())
        .collect();
    let vs: Vec<f64> = (0..=q).map(|k| k as f64 / q as f64).collect();
    let mv: Vec<Vec<f64>> = vs
        .iter()
        .map(|&v| (0..=q).map(|j| bern(q, j, v)).collect())
        .collect();
    let mut g = vec![vec![0.0; q + 1]; n];
    for (a, &u) in grev.iter().enumerate() {
        g[a] = solve(mv.clone(), vs.iter().map(|&v| f(u, v)).collect());
    }
    let mut control = Vec::new();
    let mut c = vec![vec![0.0; q + 1]; n];
    for j in 0..=q {
        let col = solve(mu.clone(), (0..n).map(|a| g[a][j]).collect());
        for i in 0..n {
            c[i][j] = col[i];
        }
    }
    for i in 0..n {
        for j in 0..=q {
            control.push(Point3::new(grev[i], j as f64 / q as f64, c[i][j]));
        }
    }
    let mut kv = vec![0.0; q + 1];
    kv.extend(vec![1.0; q + 1]);
    NurbsSurface::new(
        KnotVector::clamped(ku.to_vec(), pu).unwrap(),
        KnotVector::clamped(kv, q).unwrap(),
        control,
        vec![1.0; n * (q + 1)],
    )
    .unwrap()
}

/// Straight for `u ≤ ½`, bending by `β(u − ½)²` after.
fn psi(u: f64, beta: f64) -> f64 {
    if u <= 0.5 { 0.0 } else { beta * (u - 0.5) * (u - 0.5) }
}

#[test]
fn probe_straight_then_bend() {
    let epss: Vec<f64> = std::env::var("PROBE_EPS")
        .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or(vec![1e-6, 1e-9]);
    let ku = [0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0];
    let mut wrong = 0;
    for g in [1e-2, 1e-3, 1e-4] {
        for beta in [-2.0, -0.5, 0.5, 2.0] {
            let f = move |u: f64, v: f64| {
                let a = v - 0.4 - psi(u, beta);
                a * (a - g)
            };
            let s = spline_wall(&f, &ku, 4, 2);
            // the interpolation must be exact
            let mut worst = 0.0f64;
            for k in 0..=50 {
                for l in 0..=10 {
                    let (u, v) = (k as f64 / 50.0, l as f64 / 10.0);
                    worst = worst.max((s.eval(u, v).z - f(u, v)).abs());
                }
            }
            assert!(worst < 1e-12, "spline wall not exact: {worst:e}");
            let tr = truth(&s);
            let label = format!("bend pair g={g:e} β={beta} n={} sep={:.1e}", tr.crossings.len(), tr.min_sep);
            for &eps in &epss {
                let (row, bad) = check(&label, &s, &tr, eps);
                println!("{row}");
                wrong += usize::from(bad);
            }
        }
    }
    // A single branch, straight then bending out through the bottom.
    for beta in [-0.5, -2.0, -8.0] {
        let f = move |u: f64, v: f64| v - 0.3 - psi(u, beta);
        let s = spline_wall(&f, &ku, 4, 1);
        let tr = truth(&s);
        let label = format!("bend single β={beta} n={}", tr.crossings.len());
        for &eps in &epss {
            let (row, bad) = check(&label, &s, &tr, eps);
            println!("{row}");
            wrong += usize::from(bad);
        }
    }
    // A parabola dipping out of the bottom side and back.
    for q in [0.05, 0.2, 1.0] {
        for c in [-5e-6, -1e-4, -1e-3] {
            let f = move |u: f64, v: f64| v - c - q * (u - 0.5) * (u - 0.5);
            let s = wall(&f, 2, 1, &VARIANTS[0]);
            let tr = truth(&s);
            let label = format!("dip q={q} c={c:e} n={} sep={:.1e}", tr.crossings.len(), tr.min_sep);
            for &eps in &epss {
                let (row, bad) = check(&label, &s, &tr, eps);
                println!("{row}");
                wrong += usize::from(bad);
            }
        }
    }
    println!("WRONG CERTIFIED: {wrong}");
    assert_eq!(wrong, 0);
}

// ---------- truth ----------

struct Truth {
    crossings: Vec<[f64; 2]>,
    pairs: BTreeSet<(usize, usize)>,
    paths: HashMap<(usize, usize), Vec<[f64; 2]>>,
    min_sep: f64,
}

fn side_point(side: usize, t: f64) -> [f64; 2] {
    match side {
        0 => [t, 0.0],
        1 => [1.0, t],
        2 => [t, 1.0],
        _ => [0.0, t],
    }
}

fn truth(s: &NurbsSurface<f64>) -> Truth {
    let f = |p: [f64; 2]| s.eval(p[0], p[1]).z;
    let n = 400_000;
    let mut crossings = Vec::new();
    for side in 0..4 {
        let mut prev = f(side_point(side, 0.0));
        for k in 1..=n {
            let t = k as f64 / n as f64;
            let cur = f(side_point(side, t));
            if prev == 0.0 || prev * cur < 0.0 {
                let (mut lo, mut hi) = ((k - 1) as f64 / n as f64, t);
                let flo = f(side_point(side, lo));
                for _ in 0..80 {
                    let mid = 0.5 * (lo + hi);
                    if f(side_point(side, mid)) * flo > 0.0 {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                let c = side_point(side, 0.5 * (lo + hi));
                if !crossings
                    .iter()
                    .any(|q: &[f64; 2]| (q[0] - c[0]).abs() + (q[1] - c[1]).abs() < 1e-12)
                {
                    crossings.push(c);
                }
            }
            prev = cur;
        }
    }
    let mut pairs = BTreeSet::new();
    let mut paths = HashMap::new();
    for i in 0..crossings.len() {
        let (j, path) = trace(&f, &crossings, i, 1e-4);
        let (j2, _) = trace(&f, &crossings, i, 2.5e-5);
        assert_eq!(j, j2, "truth tracer unstable from {:?}", crossings[i]);
        let key = (i.min(j), i.max(j));
        pairs.insert(key);
        paths.entry(key).or_insert(path);
    }
    // Each crossing must be in exactly one pair.
    let mut seen = vec![0; crossings.len()];
    for &(a, b) in &pairs {
        seen[a] += 1;
        seen[b] += 1;
    }
    assert!(seen.iter().all(|&k| k == 1), "truth pairing inconsistent: {pairs:?}");
    let mut min_sep = f64::INFINITY;
    for a in 0..crossings.len() {
        for b in a + 1..crossings.len() {
            min_sep = min_sep.min(dist(crossings[a], crossings[b]));
        }
    }
    Truth {
        crossings,
        pairs,
        paths,
        min_sep,
    }
}

fn dist(a: [f64; 2], b: [f64; 2]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
}

fn grad(f: &dyn Fn([f64; 2]) -> f64, p: [f64; 2]) -> [f64; 2] {
    let h = 1e-7;
    [
        (f([p[0] + h, p[1]]) - f([p[0] - h, p[1]])) / (2.0 * h),
        (f([p[0], p[1] + h]) - f([p[0], p[1] - h])) / (2.0 * h),
    ]
}

fn inside(p: [f64; 2]) -> bool {
    (0.0..=1.0).contains(&p[0]) && (0.0..=1.0).contains(&p[1])
}

fn trace(
    f: &dyn Fn([f64; 2]) -> f64,
    cs: &[[f64; 2]],
    i: usize,
    s_max: f64,
) -> (usize, Vec<[f64; 2]>) {
    let start = cs[i];
    let mut p = start;
    let g = grad(f, p);
    let mut t = [-g[1], g[0]];
    let nt = (t[0] * t[0] + t[1] * t[1]).sqrt();
    t = [t[0] / nt, t[1] / nt];
    // Inward.
    let probe = [p[0] + 1e-6 * t[0], p[1] + 1e-6 * t[1]];
    if !inside(probe) {
        t = [-t[0], -t[1]];
    }
    let mut path = vec![p];
    let mut s = s_max;
    let mut steps = 0usize;
    loop {
        steps += 1;
        assert!(steps < 5_000_000, "truth trace runs away");
        let pred = [p[0] + s * t[0], p[1] + s * t[1]];
        let mut q = pred;
        for _ in 0..8 {
            let g = grad(f, q);
            let gg = g[0] * g[0] + g[1] * g[1];
            let v = f(q);
            q = [q[0] - v * g[0] / gg, q[1] - v * g[1] / gg];
        }
        let corr = dist(q, pred);
        let g = grad(f, q);
        let mut tn = [-g[1], g[0]];
        let n = (tn[0] * tn[0] + tn[1] * tn[1]).sqrt();
        tn = [tn[0] / n, tn[1] / n];
        if tn[0] * t[0] + tn[1] * t[1] < 0.0 {
            tn = [-tn[0], -tn[1]];
        }
        let turn = 1.0 - (tn[0] * t[0] + tn[1] * t[1]);
        if (corr > 1e-3 * s.max(1e-9) || turn > 5e-5 || f(q).abs() > 1e-12) && s > 1e-10 {
            s *= 0.5;
            continue;
        }
        if !inside(q) {
            // Exit: nearest crossing to the step's segment end.
            let (j, _) = cs
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(j, c)| (j, dist(*c, q).min(dist(*c, p))))
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .unwrap();
            path.push(cs[j]);
            return (j, path);
        }
        p = q;
        t = tn;
        path.push(p);
        s = (s * 1.5).min(s_max);
    }
}

fn seg_dist(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let d = [b[0] - a[0], b[1] - a[1]];
    let l = d[0] * d[0] + d[1] * d[1];
    let t = if l > 0.0 {
        (((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / l).clamp(0.0, 1.0)
    } else {
        0.0
    };
    dist(p, [a[0] + t * d[0], a[1] + t * d[1]])
}

fn path_dist(p: [f64; 2], path: &[[f64; 2]]) -> f64 {
    path.windows(2)
        .map(|w| seg_dist(p, w[0], w[1]))
        .fold(f64::INFINITY, f64::min)
}

// ---------- run ----------

fn plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn dom() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.5, 0.5, 0.0),
        half_extent: 3.0,
        extent: 1.0,
        floor_scale: 1.0,
    }
}

fn err_name(e: &SsiError) -> String {
    let s = format!("{e:?}");
    let head: String = s.chars().take_while(|c| c.is_alphanumeric()).collect();
    match e {
        SsiError::Escalated { decision, .. } => format!("Escalated({decision:?})"),
        SsiError::StepBudget { bound, .. } => format!("StepBudget({bound:?})"),
        SsiError::Fit(f) => format!("Fit({f:?})"),
        SsiError::RefinementExhausted { stop, .. } => format!("RefinementExhausted({stop:?})"),
        _ => head,
    }
}

/// Runs one fixture at one ε; returns a row and whether a WRONG answer
/// was certified.
fn check(name: &str, s: &NurbsSurface<f64>, tr: &Truth, eps: f64) -> (String, bool) {
    let band = Band::new(eps, 10.0 * eps).unwrap();
    let t0 = Instant::now();
    geom_brep::ssi::march::PROBE_HALVINGS.with(|c| c.set(0));
    let r = ssi::plane_nurbs_ssi(&plane(), s, dom(), band);
    let dt = t0.elapsed().as_secs_f64();
    let halv = geom_brep::ssi::march::PROBE_HALVINGS.with(|c| c.get());
    let name = &format!("{name} h={halv}");
    let out = match r {
        Err(e) => {
            return (
                format!("{name:<48} ε {eps:.0e}  ERR {:<40} {dt:7.2}s", err_name(&e)),
                false,
            );
        }
        Ok(o) => o,
    };
    let tol = 1e-6f64;
    let mut problems = Vec::new();
    let mut got = BTreeSet::new();
    let mut samples = Vec::new();
    for (k, b) in out.branches.iter().enumerate() {
        let pc = b.pcurve_b.as_ref().expect("pcurve_b");
        samples.push(pc.control().len());
        if matches!(b.end, BranchEnd::Closed) {
            problems.push(format!("branch {k} is a loop; truth has none traced"));
            continue;
        }
        let e0 = pc.eval(b.params.0);
        let e1 = pc.eval(b.params.1);
        let near = |e: geom_core::Point2<f64>| {
            tr.crossings
                .iter()
                .enumerate()
                .map(|(j, c)| (j, dist(*c, [e.x, e.y])))
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .unwrap()
        };
        let (i, di) = near(e0);
        let (j, dj) = near(e1);
        if di > 1e-6 || dj > 1e-6 {
            problems.push(format!("branch {k} ends off any crossing ({di:e}, {dj:e})"));
            continue;
        }
        let key = (i.min(j), i.max(j));
        if !tr.pairs.contains(&key) {
            problems.push(format!("branch {k} WRONG PAIR {key:?}; truth {:?}", tr.pairs));
            continue;
        }
        if !got.insert(key) {
            problems.push(format!("branch {k} duplicates pair {key:?}"));
        }
        let path = &tr.paths[&key];
        let mut worst = 0.0f64;
        for m in 0..=400 {
            let t = b.params.0 + (b.params.1 - b.params.0) * m as f64 / 400.0;
            let p = pc.eval(t);
            let mut q = [p.x, p.y];
            let f = |p: [f64; 2]| s.eval(p[0], p[1]).z;
            for _ in 0..8 {
                let g = grad(&f, q);
                let gg = g[0] * g[0] + g[1] * g[1];
                let v = f(q);
                q = [q[0] - v * g[0] / gg, q[1] - v * g[1] / gg];
            }
            worst = worst.max(path_dist(q, path));
        }
        if worst > tol {
            problems.push(format!("branch {k} strays {worst:e} from its truth path (tol {tol:e})"));
        }
    }
    for key in &tr.pairs {
        if !got.contains(key) {
            problems.push(format!("LOST truth pair {key:?}"));
        }
    }
    let wrong = !problems.is_empty();
    (
        format!(
            "{name:<48} ε {eps:.0e}  OK  {} branches samples {samples:?} {dt:7.2}s{}",
            out.branches.len(),
            if wrong {
                format!("  !!! {}", problems.join("; "))
            } else {
                String::new()
            }
        ),
        wrong,
    )
}

type Fx = (String, Box<dyn Fn(f64, f64) -> f64>, usize, usize);

fn fixtures() -> Vec<Fx> {
    let mut v: Vec<Fx> = Vec::new();
    for c in [1e-2, 1e-4, 1e-6, 1e-8, -1e-4, -1e-6, -1e-8] {
        v.push((
            format!("hyperbola c={c:e}"),
            Box::new(move |u, w| {
                let w = w + 0.1 * u;
                (w - 0.55).powi(2) - 0.25 * (u - 0.5).powi(2) - c
            }),
            2,
            2,
        ));
    }
    for g in [1e-2, 1e-3, 1e-4] {
        v.push((
            format!("parabolas g={g:e}"),
            Box::new(move |u, w| {
                let a = w - 0.2 - (u - 0.5).powi(2);
                a * (a - g)
            }),
            4,
            2,
        ));
        for sc in [1.0, 3.0] {
            v.push((
                format!("S-curves s={sc} g={g:e}"),
                Box::new(move |u, w| {
                    let a = w - 0.5 - sc * (u - 0.5).powi(3);
                    a * (a - g)
                }),
                6,
                2,
            ));
        }
        v.push((
            format!("straight-parallel tilted g={g:e}"),
            Box::new(move |u, w| {
                let a = w - 0.3 - 0.4 * u;
                a * (a - g)
            }),
            2,
            2,
        ));
    }
    for g in [5e-2, 1e-2, 2e-3] {
        for amp in [-4.0, -400.0] {
            v.push((
                format!("3 crossings on a side amp={amp} g={g:e}"),
                Box::new(move |u, w| {
                    w - amp * (u - 0.5 + g) * (u - 0.5) * (u - 0.5 - g) - 1e-3 * u
                }),
                3,
                1,
            ));
        }
    }
    v
}

#[test]
fn probe_pairing_against_truth() {
    let only = std::env::var("PROBE_ONLY").ok();
    let epss: Vec<f64> = std::env::var("PROBE_EPS")
        .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or(vec![1e-6, 1e-9, 1e-12]);
    let mut wrong = 0;
    for (name, f, p, q) in fixtures() {
        if only.as_deref().is_some_and(|o| !name.contains(o)) {
            continue;
        }
        for var in &VARIANTS {
            if let Ok(vf) = std::env::var("PROBE_VAR")
                && !vf.split(',').any(|x| x == var.name)
            {
                continue;
            }
            let s = wall(&*f, p, q, var);
            let tr = truth(&s);
            let label = format!("{name} [{}] n={} sep={:.1e}", var.name, tr.crossings.len(), tr.min_sep);
            for &eps in &epss {
                let (row, bad) = check(&label, &s, &tr, eps);
                println!("{row}");
                wrong += usize::from(bad);
            }
        }
    }
    println!("WRONG CERTIFIED: {wrong}");
    assert_eq!(wrong, 0);
}

/// Random nets: degree 3×3, random z, weights, warps.
#[test]
fn probe_random_walls() {
    let n: u64 = std::env::var("PROBE_N").map(|s| s.parse().unwrap()).unwrap_or(40);
    let epss: Vec<f64> = std::env::var("PROBE_EPS")
        .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or(vec![1e-6, 1e-9, 1e-12]);
    let mut state = 0x9e37_79b9_7f4a_7c15u64;
    let mut rnd = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut wrong = 0;
    for seed in 0..n {
        let (p, q) = (3, 3);
        let mut control = Vec::new();
        let mut weights = Vec::new();
        let rational = seed % 2 == 1;
        for i in 0..=p {
            for j in 0..=q {
                let w = if rational { 0.2 + 4.0 * rnd() } else { 1.0 };
                let x = i as f64 / 3.0 + 0.08 * (rnd() - 0.5);
                let y = j as f64 / 3.0 + 0.08 * (rnd() - 0.5);
                let z = 2.0 * rnd() - 1.0;
                control.push(Point3::new(x, y, z));
                weights.push(w);
            }
        }
        let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let s = NurbsSurface::new(k(), k(), control, weights).unwrap();
        let tr = truth(&s);
        let label = format!("random #{seed} rat={rational} n={} sep={:.1e}", tr.crossings.len(), tr.min_sep);
        for &eps in &epss {
            let (row, bad) = check(&label, &s, &tr, eps);
            println!("{row}");
            wrong += usize::from(bad);
        }
    }
    println!("WRONG CERTIFIED: {wrong}");
    assert_eq!(wrong, 0);
}

/// A single branch: straight for u ≤ ½, then bending down by 8(u − ½)²
/// out through the bottom side. Prints the answer and its rendering.
#[test]
fn probe_bend_single_detail() {
    use geom_brep::recourse::Reading;
    let ku = [0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0];
    for beta in [-2.0, -4.0, -6.0, -8.0, -16.0] {
        let f = move |u: f64, v: f64| v - 0.3 - psi(u, beta);
        let s = spline_wall(&f, &ku, 4, 1);
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            match ssi::plane_nurbs_ssi(&plane(), &s, dom(), band) {
                Ok(o) => println!(
                    "β={beta} ε {eps:e}: Ok {} branch(es), samples {:?}",
                    o.branches.len(),
                    o.branches.iter().map(|b| b.pcurve_b.as_ref().map_or(0, |c| c.control().len())).collect::<Vec<_>>()
                ),
                Err(e) => println!("β={beta} ε {eps:e}: {e:?}\n    {}", e.render(Reading::Build)),
            }
        }
    }
}

/// The hyperbola c = 1e-4 (plain chart): the `Fit(TooFewPoints)` path,
/// rendered.
#[test]
fn probe_hyperbola_fit_short_detail() {
    use geom_brep::recourse::Reading;
    for c in [1e-4, 1e-6] {
        let f = move |u: f64, w: f64| {
            let w = w + 0.1 * u;
            (w - 0.55).powi(2) - 0.25 * (u - 0.5).powi(2) - c
        };
        let s = wall(&f, 2, 2, &VARIANTS[0]);
        for eps in [1e-6, 1e-9] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            match ssi::plane_nurbs_ssi(&plane(), &s, dom(), band) {
                Ok(o) => println!("c={c:e} ε {eps:e}: Ok {}", o.branches.len()),
                Err(e) => println!("c={c:e} ε {eps:e}: {e:?}\n    {}", e.render(Reading::Build)),
            }
        }
    }
}

/// A FLAT wall `z = y − 0.3` whose chart is straight for `u ≤ ½` and
/// bends after: `y(u, v) = v + β·ψ(u)`. The carrier is a straight line
/// (every carrier rung unbounded); the state path is straight at the
/// start crossing and bends only past the knot.
#[test]
fn probe_flat_wall_chart_bends_after_a_knot() {
    let ku = [0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0];
    let pu = 4;
    let n = ku.len() - pu - 1;
    let grev: Vec<f64> = (0..n)
        .map(|i| ku[i + 1..=i + pu].iter().sum::<f64>() / pu as f64)
        .collect();
    let mu: Vec<Vec<f64>> = grev
        .iter()
        .map(|&u| (0..n).map(|i| bspline_basis(&ku, pu, i, u)).collect())
        .collect();
    for beta in [0.5, 1.0, 2.0, 4.0, -0.5, -1.0] {
        let c = solve(mu.clone(), grev.iter().map(|&u| psi(u, beta)).collect());
        let mut control = Vec::new();
        for i in 0..n {
            for j in 0..2 {
                let y = j as f64 + c[i];
                control.push(Point3::new(grev[i], y, y - 0.3));
            }
        }
        let s = NurbsSurface::new(
            KnotVector::clamped(ku.to_vec(), pu).unwrap(),
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            control,
            vec![1.0; n * 2],
        )
        .unwrap();
        let tr = truth(&s);
        let label = format!("flat wall, chart bends past ½, β={beta} n={}", tr.crossings.len());
        for eps in [1e-6, 1e-9, 1e-12] {
            let (row, _) = check(&label, &s, &tr, eps);
            println!("{row}");
        }
    }
}
