//! Reviewer probes for PR 3983 (Hermite-first): adversarial plane × NURBS
//! walls judged against a dense-grid component labelling of the true zero set.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, BranchEnd, SsiDomain, SsiOutcome};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

/// A polynomial in (s, t) by power coefficients, `c[i][j]` of s^i t^j, degree ≤ 4 each.
#[derive(Clone)]
struct P2 {
    c: [[f64; 5]; 5],
}

impl P2 {
    fn zero() -> Self {
        P2 { c: [[0.0; 5]; 5] }
    }
    fn k(v: f64) -> Self {
        let mut p = Self::zero();
        p.c[0][0] = v;
        p
    }
    fn s() -> Self {
        let mut p = Self::zero();
        p.c[1][0] = 1.0;
        p
    }
    fn t() -> Self {
        let mut p = Self::zero();
        p.c[0][1] = 1.0;
        p
    }
    fn add(&self, o: &Self) -> Self {
        let mut p = self.clone();
        for i in 0..5 {
            for j in 0..5 {
                p.c[i][j] += o.c[i][j];
            }
        }
        p
    }
    fn scale(&self, k: f64) -> Self {
        let mut p = self.clone();
        for row in &mut p.c {
            for v in row {
                *v *= k;
            }
        }
        p
    }
    fn mul(&self, o: &Self) -> Self {
        let mut p = Self::zero();
        for i in 0..5 {
            for j in 0..5 {
                if self.c[i][j] == 0.0 {
                    continue;
                }
                for k in 0..5 {
                    for l in 0..5 {
                        if o.c[k][l] == 0.0 {
                            continue;
                        }
                        assert!(i + k < 5 && j + l < 5, "degree overflow");
                        p.c[i + k][j + l] += self.c[i][j] * o.c[k][l];
                    }
                }
            }
        }
        p
    }
    fn degs(&self) -> (usize, usize) {
        let (mut a, mut b) = (1, 1);
        for i in 0..5 {
            for j in 0..5 {
                if self.c[i][j] != 0.0 {
                    a = a.max(i);
                    b = b.max(j);
                }
            }
        }
        (a, b)
    }
}

fn binom(n: usize, k: usize) -> f64 {
    let mut r = 1.0;
    for i in 0..k {
        r = r * (n - i) as f64 / (i + 1) as f64;
    }
    r
}

/// Bernstein net of `p` over [0,1]², degrees (n, m).
fn bernstein(p: &P2, n: usize, m: usize) -> Vec<Vec<f64>> {
    // first along s
    let mut a = vec![vec![0.0; m + 1]; n + 1];
    for j in 0..=m {
        for k in 0..=n {
            let mut v = 0.0;
            for i in 0..=k {
                v += binom(k, i) / binom(n, i) * p.c[i][j];
            }
            a[k][j] = v;
        }
    }
    let mut b = vec![vec![0.0; m + 1]; n + 1];
    for k in 0..=n {
        for l in 0..=m {
            let mut v = 0.0;
            for j in 0..=l {
                v += binom(l, j) / binom(m, j) * a[k][j];
            }
            b[k][l] = v;
        }
    }
    b
}

fn decast(c: &[f64], t: f64) -> f64 {
    let mut w = c.to_vec();
    let n = w.len();
    for r in 1..n {
        for i in 0..n - r {
            w[i] = (1.0 - t) * w[i] + t * w[i + 1];
        }
    }
    w[0]
}

struct Fx {
    wall: NurbsSurface<f64>,
    net: Vec<Vec<f64>>,
    l: f64,
    x0: f64,
    dx: f64,
}

impl Fx {
    /// Sign-carrying g at (s, t), by the same Bernstein net as the wall.
    fn g(&self, s: f64, t: f64) -> f64 {
        let col: Vec<f64> = self.net.iter().map(|row| decast(row, t)).collect();
        decast(&col, s)
    }
}

/// The wall z = ε·λ·g(x/L, y/L), L = λ/S, over [0, L]².
fn wall(g: &P2, slope: f64, lambda: f64) -> Fx {
    let (n, m) = g.degs();
    let net = bernstein(g, n, m);
    let l = lambda / slope;
    let h = eps() * lambda;
    let ku = KnotVector::clamped(
        [vec![0.0; n + 1], vec![1.0; n + 1]].concat(),
        n,
    )
    .unwrap();
    let kv = KnotVector::clamped(
        [vec![0.0; m + 1], vec![1.0; m + 1]].concat(),
        m,
    )
    .unwrap();
    let mut control = Vec::new();
    for i in 0..=n {
        for j in 0..=m {
            control.push(Point3::new(
                l * i as f64 / n as f64,
                l * j as f64 / m as f64,
                h * net[i][j],
            ));
        }
    }
    let w = vec![1.0; (n + 1) * (m + 1)];
    Fx {
        wall: NurbsSurface::new(ku, kv, control, w).unwrap(),
        net,
        l,
        x0: 0.0,
        dx: l,
    }
}

/// The wall z = g(s, t) (absolute heights) over x = x0 + dx·s, y = l·t.
fn wall_abs(g: &P2, x0: f64, dx: f64, l: f64) -> Fx {
    let (n, m) = g.degs();
    let net = bernstein(g, n, m);
    let ku = KnotVector::clamped([vec![0.0; n + 1], vec![1.0; n + 1]].concat(), n).unwrap();
    let kv = KnotVector::clamped([vec![0.0; m + 1], vec![1.0; m + 1]].concat(), m).unwrap();
    let mut control = Vec::new();
    for i in 0..=n {
        for j in 0..=m {
            control.push(Point3::new(
                x0 + dx * i as f64 / n as f64,
                l * j as f64 / m as f64,
                net[i][j],
            ));
        }
    }
    let w = vec![1.0; (n + 1) * (m + 1)];
    Fx {
        wall: NurbsSurface::new(ku, kv, control, w).unwrap(),
        net,
        l,
        x0,
        dx,
    }
}

fn ground() -> (Surface<f64>, SsiDomain) {
    (
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        SsiDomain {
            center: Point3::new(0.0, 0.0, 0.0),
            half_extent: 1.0,
            extent: 1.0,
            floor_scale: 1.0,
        },
    )
}

/// Components of g = 0 over [0,1]² by marching squares + union-find.
/// Returns (boundary crossings (s,t,comp), all zero points (s,t,comp)).
fn components(fx: &Fx, n: usize) -> (Vec<(f64, f64, usize)>, Vec<(f64, f64, usize)>) {
    let xs = |i: usize| i as f64 / n as f64;
    let mut vals = vec![vec![0.0; n + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=n {
            vals[i][j] = fx.g(xs(i), xs(j));
        }
    }
    let neg = |i: usize, j: usize| vals[i][j] < 0.0;
    let h_id = |i: usize, j: usize| j * n + i;
    let v_id = |i: usize, j: usize| n * (n + 1) + j * (n + 1) + i;
    let mut parent: Vec<usize> = (0..2 * n * (n + 1)).collect();
    fn find(p: &mut [usize], mut a: usize) -> usize {
        while p[a] != a {
            p[a] = p[p[a]];
            a = p[a];
        }
        a
    }
    for j in 0..n {
        for i in 0..n {
            let edges = [
                (h_id(i, j), neg(i, j) != neg(i + 1, j)),
                (v_id(i + 1, j), neg(i + 1, j) != neg(i + 1, j + 1)),
                (h_id(i, j + 1), neg(i, j + 1) != neg(i + 1, j + 1)),
                (v_id(i, j), neg(i, j) != neg(i, j + 1)),
            ];
            let cut: Vec<usize> = edges.iter().filter(|e| e.1).map(|e| e.0).collect();
            let pairs: Vec<(usize, usize)> = match cut.len() {
                2 => vec![(cut[0], cut[1])],
                4 => {
                    let centre = fx.g(0.5 * (xs(i) + xs(i + 1)), 0.5 * (xs(j) + xs(j + 1))) < 0.0;
                    if centre == neg(i, j) {
                        vec![(cut[0], cut[1]), (cut[2], cut[3])]
                    } else {
                        vec![(cut[0], cut[3]), (cut[1], cut[2])]
                    }
                }
                _ => vec![],
            };
            for (a, b) in pairs {
                let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
                parent[ra] = rb;
            }
        }
    }
    let mut bd = Vec::new();
    let mut all = Vec::new();
    for j in 0..=n {
        for i in 0..n {
            if neg(i, j) != neg(i + 1, j) {
                let p = (0.5 * (xs(i) + xs(i + 1)), xs(j), find(&mut parent, h_id(i, j)));
                all.push(p);
                if j == 0 || j == n {
                    bd.push(p);
                }
            }
        }
    }
    for j in 0..n {
        for i in 0..=n {
            if neg(i, j) != neg(i, j + 1) {
                let p = (xs(i), 0.5 * (xs(j) + xs(j + 1)), find(&mut parent, v_id(i, j)));
                all.push(p);
                if i == 0 || i == n {
                    bd.push(p);
                }
            }
        }
    }
    (bd, all)
}

#[derive(Debug, PartialEq)]
enum Verdict {
    Good,
    Refused(String),
    Wrong(String),
}

/// Judge `out` against the locus: pairing by component, every component carried.
fn judge(fx: &Fx, out: &SsiOutcome) -> Verdict {
    let (bd, all) = components(fx, 800);
    let comp = |p: Point3<f64>| {
        let (s, t) = ((p.x - fx.x0) / fx.dx, p.y / fx.l);
        bd.iter()
            .min_by(|a, b| {
                let da = (a.0 - s).powi(2) + (a.1 - t).powi(2);
                let db = (b.0 - s).powi(2) + (b.1 - t).powi(2);
                da.total_cmp(&db)
            })
            .map(|c| (c.2, ((c.0 - s).powi(2) + (c.1 - t).powi(2)).sqrt()))
    };
    let mut seen: Vec<usize> = Vec::new();
    let mut samples: Vec<(f64, f64)> = Vec::new();
    for b in &out.branches {
        let (t0, t1) = b.params;
        for k in 0..=2000 {
            let q = b.carrier.eval(t0 + (t1 - t0) * f64::from(k) / 2000.0);
            samples.push(((q.x - fx.x0) / fx.dx, q.y / fx.l));
        }
        if let BranchEnd::Crossings { .. } = b.end {
            let (p, q) = (b.carrier.eval(t0), b.carrier.eval(t1));
            let (cp, cq) = (comp(p).unwrap(), comp(q).unwrap());
            if cp.0 != cq.0 {
                return Verdict::Wrong(format!(
                    "branch ({:.4},{:.4})->({:.4},{:.4}) joins comps {} and {} (snap {:.1e},{:.1e})",
                    (p.x - fx.x0) / fx.dx,
                    p.y / fx.l,
                    (q.x - fx.x0) / fx.dx,
                    q.y / fx.l,
                    cp.0,
                    cq.0,
                    cp.1,
                    cq.1
                ));
            }
            if seen.contains(&cp.0) {
                return Verdict::Wrong("two open branches on one component".into());
            }
            seen.push(cp.0);
        }
    }
    let mut comps: Vec<usize> = all.iter().map(|c| c.2).collect();
    comps.sort_unstable();
    comps.dedup();
    let mut lost = Vec::new();
    for c in &comps {
        let d = all
            .iter()
            .filter(|z| z.2 == *c)
            .map(|z| {
                samples
                    .iter()
                    .map(|q| (q.0 - z.0).powi(2) + (q.1 - z.1).powi(2))
                    .fold(f64::INFINITY, f64::min)
            })
            .fold(f64::INFINITY, f64::min)
            .sqrt();
        // a component none of whose zeros is near any carrier
        if d > 0.05 {
            let pts = all.iter().filter(|z| z.2 == *c).count();
            lost.push(format!("comp {c} ({pts} zeros) min dist {d:.3}"));
        }
    }
    if !lost.is_empty() {
        return Verdict::Wrong(format!("lost: {lost:?}"));
    }
    let mut bcomps: Vec<usize> = bd.iter().map(|c| c.2).collect();
    bcomps.sort_unstable();
    bcomps.dedup();
    if bcomps.len() != seen.len() {
        return Verdict::Wrong(format!(
            "{} boundary components, {} open branches",
            bcomps.len(),
            seen.len()
        ));
    }
    Verdict::Good
}

fn run(name: &str, g: &P2, slope: f64, lambda: f64) -> Verdict {
    run_fx(name, &wall(g, slope, lambda), &format!("slope={slope} lambda={lambda}"))
}

fn run_fx(name: &str, fx: &Fx, tag: &str) -> Verdict {
    let fx = fx;
    let (plane, domain) = ground();
    let v = match ssi::plane_nurbs_ssi(&plane, &fx.wall, domain, band()) {
        Ok(out) => {
            let v = judge(&fx, &out);
            let closed = out.branches.iter().filter(|b| matches!(b.end, BranchEnd::Closed)).count();
            eprintln!("  ENDS {name}: {} branches, {closed} closed", out.branches.len());
            if let Verdict::Wrong(_) = v {
                eprintln!(
                    "  {} branches: {:?}",
                    out.branches.len(),
                    out.branches
                        .iter()
                        .map(|b| (format!("{:?}", b.params), format!("{:?}", b.end)))
                        .collect::<Vec<_>>()
                );
            }
            v
        }
        Err(e) => {
            let s = format!("{e}");
            let key: String = s.chars().take(60).collect();
            Verdict::Refused(key)
        }
    };
    eprintln!("CASE {name} {tag}: {v:?}");
    v
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn u(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
}

fn lin(a: f64, b: f64, c: f64) -> P2 {
    // a + b s + c t
    P2::k(a).add(&P2::s().scale(b)).add(&P2::t().scale(c))
}

fn tally(name: &str, vs: &[Verdict]) {
    let good = vs.iter().filter(|v| **v == Verdict::Good).count();
    let wrong = vs.iter().filter(|v| matches!(v, Verdict::Wrong(_))).count();
    let refused = vs.len() - good - wrong;
    eprintln!("TALLY {name} eps={:e}: good {good} wrong {wrong} refused {refused}", eps());
}

/// Families: three crossings on one side; near-saddle pairs; loop beside a
/// line; two S-curves; random bicubics.
#[test]
fn rev3983_adversarial_sweep() {
    let only = std::env::var("REV_FAMILY").ok();
    let want = |f: &str| only.as_deref().is_none_or(|o| o == f);
    let mut all = Vec::new();
    // B. three crossings on the bottom side
    if want("three") {
        let mut vs = Vec::new();
        for (r, mu) in [
            ([0.2, 0.45, 0.7], 0.3),
            ([0.15, 0.3, 0.8], -0.2),
            ([0.3, 0.4, 0.5], 0.05),
            ([0.25, 0.5, 0.75], 0.6),
            ([0.1, 0.55, 0.6], 0.1),
        ] {
            for k in [4.0, 12.0] {
                let p = lin(-r[0], 1.0, 0.0)
                    .mul(&lin(-r[1], 1.0, 0.0))
                    .mul(&lin(-r[2], 1.0, 0.0))
                    .scale(k)
                    .add(&lin(0.0, 0.0, mu))
                    .add(&P2::t().mul(&P2::t()).scale(-0.5 * mu));
                for lambda in [30.0, 300.0] {
                    vs.push(run(&format!("three r={r:?} mu={mu} k={k}"), &p, 800.0, lambda));
                }
            }
        }
        tally("three", &vs);
        all.extend(vs);
    }
    // E. near-saddle: (s-.5)^2*a - (t-.5)^2 + d
    if want("saddle") {
        let mut vs = Vec::new();
        for a in [1.0, 3.0] {
            for d in [-0.05, -0.01, -0.002, 0.002, 0.01, 0.05] {
                let sm = lin(-0.5, 1.0, 0.0);
                let tm = lin(-0.5, 0.0, 1.0);
                let p = sm.mul(&sm).scale(a).add(&tm.mul(&tm).scale(-1.0)).add(&P2::k(d));
                for lambda in [30.0, 300.0] {
                    vs.push(run(&format!("saddle a={a} d={d}"), &p, 800.0, lambda));
                }
            }
        }
        tally("saddle", &vs);
        all.extend(vs);
    }
    // C. loop beside a line
    if want("loop") {
        let mut vs = Vec::new();
        for (cx, r, lx, d) in [
            (0.35, 0.15, 0.75, 0.0),
            (0.35, 0.2, 0.62, 0.0),
            (0.4, 0.2, 0.65, 0.0),
            (0.35, 0.2, 0.62, 0.0005),
            (0.5, 0.2, 0.75, 0.0),
        ] {
            let sm = lin(-cx, 1.0, 0.0);
            let tm = lin(-0.5, 0.0, 1.0);
            let circ = sm.mul(&sm).add(&tm.mul(&tm)).add(&P2::k(-r * r));
            let line = lin(-lx, 1.0, 0.1);
            let p = line.mul(&circ).scale(8.0).add(&P2::k(d));
            for lambda in [30.0, 300.0] {
                vs.push(run(&format!("loop cx={cx} r={r} lx={lx} d={d}"), &p, 800.0, lambda));
            }
        }
        tally("loop", &vs);
        all.extend(vs);
    }
    // D. two S curves: (s - S1(t))(s - S2(t)) + d
    if want("s") {
        let mut vs = Vec::new();
        for (c1, c2, al, d) in [
            (0.4, 0.55, 1.2, 0.0),
            (0.4, 0.5, 1.2, 0.0),
            (0.45, 0.5, 0.8, 0.0),
            (0.4, 0.5, 1.2, 0.001),
            (0.4, 0.5, 1.2, -0.001),
            (0.3, 0.6, 2.0, 0.0),
        ] {
            let tm = lin(-0.5, 0.0, 1.0);
            let cube = tm.mul(&tm).mul(&tm);
            let s1 = lin(-c1, 1.0, 0.0).add(&cube.scale(-al)).add(&tm.scale(0.3 * al));
            let s2 = lin(-c2, 1.0, 0.0).add(&cube.scale(-al)).add(&tm.scale(0.3 * al));
            // degree in t: 6 — too high; use the shared S on one only
            let _ = s2;
            let s2b = lin(-c2, 1.0, 0.0).add(&tm.scale(-0.2));
            let p = s1.mul(&s2b).scale(6.0).add(&P2::k(d));
            for lambda in [30.0, 300.0] {
                vs.push(run(&format!("s c1={c1} c2={c2} al={al} d={d}"), &p, 800.0, lambda));
            }
        }
        tally("s", &vs);
        all.extend(vs);
    }
    // A. random bicubics
    if want("random") {
        let mut vs = Vec::new();
        let n: u64 = std::env::var("REV_N").ok().and_then(|s| s.parse().ok()).unwrap_or(40);
        for seed in 0..n {
            let mut r = Rng(seed * 7919 + 13);
            let mut p = P2::zero();
            for i in 0..4 {
                for j in 0..4 {
                    p.c[i][j] = r.u(-1.0, 1.0) * if i + j > 3 { 0.5 } else { 1.0 };
                }
            }
            let lambda = [20.0, 100.0, 500.0][(seed % 3) as usize];
            vs.push(run(&format!("random seed={seed}"), &p, 800.0, lambda));
        }
        tally("random", &vs);
        all.extend(vs);
    }
    let wrong: Vec<_> = all.iter().filter(|v| matches!(v, Verdict::Wrong(_))).collect();
    assert!(wrong.is_empty(), "{} wrong: {wrong:?}", wrong.len());
}

/// m2: a straight branch whose transversality dips into the band at its
/// middle, with the arm clamp (extent) small so the per-state decision
/// is in band where the tube may clear.
#[test]
fn rev3983_pinch_mid_transversality() {
    let only_ext: Option<f64> = std::env::var("REV_EXT").ok().and_then(|s| s.parse().ok());
    for extent in only_ext.map_or(vec![1.0, 0.1, 0.05, 0.025], |e| vec![e]) {
    for mmin in [0.5, 0.6, 0.7, 0.8, 0.9, 1.0] {
        let sm = lin(-0.5, 1.0, 0.0);
        let bump_t = P2::t().mul(&lin(1.0, 0.0, -1.0)).scale(4.0);
        let bump_s = P2::k(1.0).add(&sm.mul(&sm).scale(-4.0));
        let m = P2::k(1.0).add(&bump_t.mul(&bump_s).scale(-(1.0 - mmin)));
        let g = sm.mul(&m);
        for lambda in [200.0] {
            let fx = wall(&g, 800.0, lambda);
            let (plane, mut domain) = ground();
            domain.extent = extent;
            let tv = 800.0 * extent;
            match ssi::plane_nurbs_ssi(&plane, &fx.wall, domain, band()) {
                Ok(out) => {
                    let v = judge(&fx, &out);
                    eprintln!(
                        "PINCH ext={extent} mmin={mmin} (ends ~{tv}eps mid ~{}eps): Ok {} branches {:?} verdict {v:?}",
                        tv * mmin,
                        out.branches.len(),
                        out.branches.iter().map(|b| b.params).collect::<Vec<_>>(),
                    );
                }
                Err(e) => eprintln!("PINCH ext={extent} mmin={mmin} (ends ~{tv}eps mid ~{}eps): Err {e}", tv * mmin),
            }
        }
    }
    }
}

fn cc_wall(gap: f64, height: f64) -> NurbsSurface<f64> {
    let a = 0.5 - gap / 2.0;
    let ab = a * (1.0 - a);
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::with_capacity(6);
    for (x, y) in [(0.0, ab), (0.5, ab - 0.5), (1.0, ab)] {
        control.push(Point3::new(x, y, 0.0));
        control.push(Point3::new(x, y, height));
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap()
}

/// Claim 6: the hair-inside regime (the march's last state on the far edge
/// within rounding) on the close-crossings wall, at heights the 0.04 m step divides.
#[test]
fn rev3983_hair_at_rounding() {
    let plane = Surface::Plane {
        origin: Point3::new(0.5, 0.0, 0.5),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let dom = SsiDomain {
        center: Point3::new(0.5, 0.0, 0.5),
        half_extent: 2.0,
        extent: 1.0,
        floor_scale: 1.0,
    };
    let mut bad = Vec::new();
    for k in [20.0, 21.0, 22.0, 23.0, 24.0, 25.0] {
        for wiggle in [0.0, 1e-16, -1e-16, 3e-16, -3e-16, 1e-15, -1e-15] {
            let h = 0.04 * k + wiggle;
            for gap in [0.2, 0.24, 0.16] {
                let r = ssi::plane_nurbs_ssi(&plane, &cc_wall(gap, h), dom, band());
                let ok = match &r {
                    Ok(out) => out.branches.len() == 2,
                    Err(_) => false,
                };
                if !ok {
                    bad.push(format!("h={h:e} gap={gap}: {:?}", r.as_ref().map(|o| o.branches.len()).map_err(|e| e.to_string())));
                }
            }
        }
    }
    eprintln!("HAIR bad {}: {bad:#?}", bad.len());
}

/// Fold family: z = c·x + a·x² + β·H(y/L), with H a single hump, a double
/// hump (a closed loop between two gaps), a double hump offset, or an
/// asymmetric hump; every height scaled to ε as the PR's fold.
#[test]
fn rev3983_fold_family() {
    let beta = eps();
    let mut vs = Vec::new();
    let tt = P2::t();
    let one_minus_t = lin(1.0, 0.0, -1.0);
    let tm = lin(-0.5, 0.0, 1.0);
    let u = tm.mul(&tm);
    // H(t) shapes, each a P2 in t only
    let single = tt.mul(&one_minus_t).scale(4.0);
    let double = u.mul(&P2::k(0.25).add(&u.scale(-1.0))).scale(64.0);
    let asym = tt.mul(&one_minus_t).mul(&one_minus_t).scale(6.75);
    let shapes: Vec<(&str, P2)> = vec![
        ("single", single.clone()),
        ("double", double.clone()),
        ("double+0.3", double.add(&P2::k(0.3))),
        ("double-0.2", double.add(&P2::k(-0.2))),
        ("asym", asym.clone()),
        ("single+double", single.scale(0.5).add(&double.scale(0.8))),
    ];
    let only = std::env::var("REV_SHAPE").ok();
    for (name, h) in &shapes {
        if only.as_deref().is_some_and(|o| o != *name) {
            continue;
        }
        for ck in [40.0, 80.0] {
            for ak in [0.22, 0.28, 0.4] {
                for (x0k, x1k) in [(-1.5, 1.8), (-2.5, 1.0), (-1.2, 0.6)] {
                    for lk in [1.0, 1.5, 2.5] {
                        let c = ck * beta;
                        let a = ak * c * c / beta;
                        let w = beta / c;
                        let (x0, x1, l) = (x0k * w, x1k * w, lk * w);
                        let dx = x1 - x0;
                        // c·x + a·x² with x = x0 + dx·s
                        let xs = P2::k(x0).add(&P2::s().scale(dx));
                        let g = xs.scale(c).add(&xs.mul(&xs).scale(a)).add(&h.scale(beta));
                        let fx = wall_abs(&g, x0, dx, l);
                        vs.push(run_fx(
                            &format!("fold {name} ck={ck} ak={ak} x=[{x0k},{x1k}] lk={lk}"),
                            &fx,
                            "",
                        ));
                    }
                }
            }
        }
    }
    tally("fold", &vs);
    let wrong: Vec<_> = vs.iter().filter(|v| matches!(v, Verdict::Wrong(_))).collect();
    assert!(wrong.is_empty(), "{} wrong: {wrong:?}", wrong.len());
}

/// m2: a geometrically flat wall (the plane z = α(x − x0)) whose u lines
/// are parameterised unevenly in the middle of the locus, so the R4 lever
/// arm (chart speed² / |second derivative|) shrinks there while the ends'
/// arm is the extent. The march decides `ssi_transversality` at every
/// state; the Hermite decides only at its two ends, and limb 3 levers its
/// clearance by the extent.
#[test]
fn rev3983_warp_mid_arm() {
    for (alpha, xw, kappa) in [
        (1e-6, 0.04, 4.0),
        (1e-6, 0.04, 2.0),
        (1e-6, 0.06, 4.0),
        (2e-6, 0.03, 4.0),
        (1e-6, 0.1, 4.0),
    ] {
        let x0 = 0.0;
        let y_len = 0.5;
        let d = lin(-0.5, 1.0, 0.0);
        let r = P2::t().mul(&lin(1.0, 0.0, -1.0)).scale(4.0);
        // x = x0 + X·d + X·κ·R·d²·(1 + 2d)
        let xp = P2::k(x0)
            .add(&d.scale(xw))
            .add(&r.mul(&d).mul(&d).mul(&P2::k(1.0).add(&d.scale(2.0))).scale(xw * kappa));
        let zp = xp.add(&P2::k(-x0)).scale(alpha);
        let (n, m) = (3, 2);
        let xn = bernstein(&xp, n, m);
        let zn = bernstein(&zp, n, m);
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let mut control = Vec::new();
        for i in 0..=n {
            for j in 0..=m {
                control.push(Point3::new(xn[i][j], y_len * j as f64 / m as f64, zn[i][j]));
            }
        }
        let wall = NurbsSurface::new(ku, kv, control, vec![1.0; 12]).unwrap();
        let (plane, mut domain) = ground();
        domain.extent = 1.0;
        let t_mid = alpha * xw / (2.0 * kappa);
        match ssi::plane_nurbs_ssi(&plane, &wall, domain, band()) {
            Ok(out) => {
                let desc: Vec<String> = out
                    .branches
                    .iter()
                    .map(|b| {
                        let (p, q) = (b.carrier.eval(b.params.0), b.carrier.eval(b.params.1));
                        let mut worst: f64 = 0.0;
                        for k in 0..=200 {
                            let x = b.carrier.eval(b.params.0 + (b.params.1 - b.params.0) * f64::from(k) / 200.0);
                            worst = worst.max((x.x - x0).abs()).max(x.z.abs());
                        }
                        format!(
                            "({:.3e},{:.3e})->({:.3e},{:.3e}) off-line {worst:.1e} tube {:?}",
                            p.x, p.y, q.x, q.y, b.certificate.tube
                        )
                    })
                    .collect();
                eprintln!(
                    "WARP alpha={alpha} X={xw} kappa={kappa} per-state mid ~{t_mid:.2e} m: Ok {} {desc:?}",
                    out.branches.len()
                );
            }
            Err(e) => eprintln!("WARP alpha={alpha} X={xw} kappa={kappa} per-state mid ~{t_mid:.2e} m: Err {e}"),
        }
    }
}

/// Fold with a closed loop between two gaps: z = c·x + a·x² + β·H(y/L), H
/// the double hump, x-range wide enough that both roots lie inside in the
/// middle band. Topology: two arcs (each joining its v side's two
/// crossings) and a closed loop between them.
#[test]
fn rev3983_loop_fold() {
    let beta = eps();
    let mut vs = Vec::new();
    let tm = lin(-0.5, 0.0, 1.0);
    let u = tm.mul(&tm);
    let double = u.mul(&P2::k(0.25).add(&u.scale(-1.0))).scale(64.0);
    for (hk, ak) in [(1.0, 0.4), (1.0, 0.5), (1.2, 0.3), (1.5, 0.3), (1.0, 0.33)] {
        for (x0k, x1k) in [(-4.0, 1.0), (-3.6, 0.8), (-5.0, 1.5)] {
            for lk in [0.8, 1.5, 3.0] {
                let ck = 80.0;
                let c = ck * beta;
                let a = ak * c * c / beta;
                let w = beta / c;
                let (x0, x1, l) = (x0k * w, x1k * w, lk * w);
                let dx = x1 - x0;
                let xs = P2::k(x0).add(&P2::s().scale(dx));
                let g = xs
                    .scale(c)
                    .add(&xs.mul(&xs).scale(a))
                    .add(&double.scale(beta * hk));
                let fx = wall_abs(&g, x0, dx, l);
                vs.push(run_fx(
                    &format!("loopfold hk={hk} ak={ak} x=[{x0k},{x1k}] lk={lk}"),
                    &fx,
                    "",
                ));
            }
        }
    }
    tally("loopfold", &vs);
    let wrong: Vec<_> = vs.iter().filter(|v| matches!(v, Verdict::Wrong(_))).collect();
    assert!(wrong.is_empty(), "{} wrong: {wrong:?}", wrong.len());
}

/// Lever B: the witness's flat wall `z = α·x`, its chart bunched in `s`
/// about the locus (`κ`) AND its chart path bent (`μ`): the locus is
/// the straight line `x = z = 0` in space, but in the chart it is the
/// parabola `s − ½ − κh(s − ½)² = μh`, `h = 4t(1 − t)`, so the march
/// steps along it, reading its lever at every state. `μ = 0` is the
/// witness's straight chart path.
#[test]
fn leverb_warp_chart_path() {
    // κ < 1 keeps `x` monotone in `s` (an injective chart); the chart
    // arm at `(½, ½)` is `X/(2κ)` = 6.25e-3 m, so `α · arm` = 6.25e-9.
    let alpha = 1e-6;
    let xw = 0.01;
    let kappa = 0.8;
    for mu in [0.0, 0.1, 0.2, 0.3] {
        let d = lin(-0.5, 1.0, 0.0);
        let h = P2::t().mul(&lin(1.0, 0.0, -1.0)).scale(4.0);
        // x = X·[(s − ½) − κ h (s − ½)² − μ h]
        let xp = d
            .add(&h.mul(&d).mul(&d).scale(-kappa))
            .add(&h.scale(-mu))
            .scale(xw);
        let zp = xp.scale(alpha);
        let (n, m) = (2, 2);
        let xn = bernstein(&xp, n, m);
        let zn = bernstein(&zp, n, m);
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let mut control = Vec::new();
        for i in 0..=n {
            for j in 0..=m {
                control.push(Point3::new(xn[i][j], 0.5 * j as f64 / m as f64, zn[i][j]));
            }
        }
        let wall = NurbsSurface::new(ku, kv, control, vec![1.0; 9]).unwrap();
        let (plane, mut domain) = ground();
        domain.extent = 1.0;
        match ssi::plane_nurbs_ssi(&plane, &wall, domain, band()) {
            Ok(out) => {
                let desc: Vec<String> = out
                    .branches
                    .iter()
                    .map(|b| {
                        let (p, q) = (b.carrier.eval(b.params.0), b.carrier.eval(b.params.1));
                        let mut worst: f64 = 0.0;
                        for k in 0..=200 {
                            let x = b.carrier.eval(b.params.0 + (b.params.1 - b.params.0) * f64::from(k) / 200.0);
                            worst = worst.max(x.x.abs()).max(x.z.abs());
                        }
                        format!(
                            "({:.2e},{:.2e})->({:.2e},{:.2e}) off-line {worst:.1e} samples {}",
                            p.x, p.y, q.x, q.y, b.certificate.samples
                        )
                    })
                    .collect();
                eprintln!("LEVERB mu={mu} eps={:e}: Ok {} {desc:?}", eps(), out.branches.len());
            }
            Err(e) => eprintln!("LEVERB mu={mu} eps={:e}: Err {e}", eps()),
        }
    }
}
