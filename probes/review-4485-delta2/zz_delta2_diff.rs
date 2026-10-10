//! Delta-2 review differential (probe, not a suite).
#![allow(clippy::all, clippy::pedantic, missing_docs, clippy::unwrap_used)]
use geom_brep::patch_bound::{patch_cells, patch_cells_refined};
use geom_brep::props::quad::{bspline_green_integral, nurbs_patch_face};
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Point3, Tol};
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom::surfaces::NurbsSurface;
use std::fmt::Write;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn f(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn u(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn iv(x: &Interval) -> String {
    format!("{:x}:{:x}:{}", x.lo().to_bits(), x.hi().to_bits(), x.is_certified())
}

fn kv(r: &mut Rng, p: usize, allow_interior: bool) -> KnotVector {
    let mut k = vec![0.0; p + 1];
    if allow_interior {
        let n = r.u(3);
        let mut vals: Vec<f64> = (0..n).map(|_| (r.f() * 0.8 + 0.1)).collect();
        vals.sort_by(f64::total_cmp);
        vals.dedup();
        for v in vals {
            let m = 1 + r.u(p);
            for _ in 0..m {
                k.push(v);
            }
        }
    }
    k.extend(std::iter::repeat(1.0).take(p + 1));
    KnotVector::clamped(k, p).unwrap()
}

fn green(k:&KnotVector,u:&[Interval],v:&[Interval],a:f64,b:f64,pieces:usize)->Result<Interval,geom_brep::props::PropsError>{ bspline_green_integral(k.with_coeffs(u).unwrap(),k.with_coeffs(v).unwrap(),a,b,pieces) }

/// f64 de Boor value of a B-spline at t.
fn bs(kv: &KnotVector, c: &[f64], t: f64) -> f64 {
    let k = kv.knots();
    let p = kv.degree();
    let n = c.len();
    let mut s = p;
    while s + 1 < n && !(t < k[s + 1]) {
        s += 1;
    }
    let mut d: Vec<f64> = (0..=p).map(|j| c[s - p + j]).collect();
    for r in 1..=p {
        for j in (r..=p).rev() {
            let i = s - p + j;
            let den = k[i + p + 1 - r] - k[i];
            let a = if den == 0.0 { 0.0 } else { (t - k[i]) / den };
            d[j] = (1.0 - a) * d[j - 1] + a * d[j];
        }
    }
    d[p]
}
fn dbs(kv: &KnotVector, c: &[f64], t: f64) -> f64 {
    let k = kv.knots();
    let p = kv.degree();
    let dc: Vec<f64> = (0..c.len() - 1)
        .map(|i| {
            let den = k[i + p + 1] - k[i + 1];
            if den == 0.0 { 0.0 } else { p as f64 * (c[i + 1] - c[i]) / den }
        })
        .collect();
    let dk = KnotVector::clamped(k[1..k.len() - 1].to_vec(), p - 1);
    match dk {
        Ok(dk) => bs(&dk, &dc, t),
        Err(_) => {
            // raw evaluation on the derivative's knot slice
            let kk = &k[1..k.len() - 1];
            let q = p - 1;
            let n = dc.len();
            let mut s = q;
            while s + 1 < n && !(t < kk[s + 1]) {
                s += 1;
            }
            let mut d: Vec<f64> = (0..=q).map(|j| dc[s - q + j]).collect();
            for r in 1..=q {
                for j in (r..=q).rev() {
                    let i = s - q + j;
                    let den = kk[i + q + 1 - r] - kk[i];
                    let a = if den == 0.0 { 0.0 } else { (t - kk[i]) / den };
                    d[j] = (1.0 - a) * d[j - 1] + a * d[j];
                }
            }
            d[q]
        }
    }
}
/// ∫_a^b u·v' by 8-point Gauss–Legendre per knot-free piece (exact for deg ≤ 15).
fn truth(kv: &KnotVector, u: &[f64], v: &[f64], a: f64, b: f64) -> f64 {
    let x = [-0.9602898564975363, -0.7966664774136267, -0.5255324099163290, -0.1834346424956498,
             0.1834346424956498, 0.5255324099163290, 0.7966664774136267, 0.9602898564975363];
    let w = [0.1012285362903763, 0.2223810344533745, 0.3137066458778873, 0.3626837833783620,
             0.3626837833783620, 0.3137066458778873, 0.2223810344533745, 0.1012285362903763];
    let mut cuts = vec![a];
    for &k in kv.knots() {
        if k > a && k < b && *cuts.last().unwrap() != k {
            cuts.push(k);
        }
    }
    cuts.push(b);
    let mut s = 0.0;
    for c in cuts.windows(2) {
        let (lo, hi) = (c[0], c[1]);
        let (m, h) = ((lo + hi) / 2.0, (hi - lo) / 2.0);
        for i in 0..8 {
            let t = m + h * x[i];
            s += h * w[i] * bs(kv, u, t) * dbs(kv, v, t);
        }
    }
    s
}

#[test]
fn delta2_diff() {
    let out_dir = std::env::var("PROBE_OUT").unwrap();
    let eps = Tol::witness().eps();
    let band = Band::linear(Tol::witness()).unwrap();
    // ---- patch_cells / refined / nurbs_patch_face ----
    let mut r = Rng(0x9E3779B97F4A7C15);
    let mut s = String::new();
    let n_face: usize = std::env::var("NFACE").map(|x| x.parse().unwrap()).unwrap_or(40);
    for case in 0..400 {
        let pu = 1 + r.u(3);
        let pv = 1 + r.u(3);
        let iu = pu > 1 || r.u(4) == 0; let ku = kv(&mut r, pu, iu);
        let iv_ = pv > 1 || r.u(4) == 0; let kvv = kv(&mut r, pv, iv_);
        let (nu, nv) = (ku.control_count(), kvv.control_count());
        let ctrl: Vec<Point3<f64>> = (0..nu * nv)
            .map(|_| Point3::new(r.f() * 2.0 - 1.0, r.f() * 2.0 - 1.0, r.f() * 2.0 - 1.0))
            .collect();
        let rational = r.u(2) == 0;
        let wts: Vec<f64> = (0..nu * nv).map(|_| if rational { 0.5 + 1.5 * r.f() } else { 1.0 }).collect();
        writeln!(s, "case {case} pu={pu} pv={pv} ku={:?} kv={:?} rat={rational}", ku.knots(), kvv.knots()).unwrap();
        let n = NurbsSurface::new(ku.clone(), kvv.clone(), ctrl.clone(), wts.clone()).unwrap();
        let dump = |s: &mut String, tag: &str, res: Result<Vec<geom_brep::patch_bound::PatchCell>, geom_brep::patch_bound::PatchBoundError>| {
            match res {
                Err(e) => writeln!(s, " {tag} err {e:?}").unwrap(),
                Ok(cells) => {
                    for c in cells {
                        write!(s, " {tag} {:x} {:x} {:x} {:x}", c.u.0.to_bits(), c.u.1.to_bits(), c.v.0.to_bits(), c.v.1.to_bits()).unwrap();
                        for a in [c.s_u, c.s_v, c.s_uu, c.s_uv, c.s_vv] {
                            for x in a { write!(s, " {}", iv(&x)).unwrap(); }
                        }
                        writeln!(s, " {:?}", c).unwrap();
                    }
                }
            }
        };
        dump(&mut s, "pc", patch_cells(&n));
        dump(&mut s, "pr2", patch_cells_refined(&n, 2));
        if case < n_face {
            let ctl: Vec<[Interval; 3]> = ctrl.iter().map(|p| [Interval::point(p.x), Interval::point(p.y), Interval::point(p.z)]).collect();
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| nurbs_patch_face::<f64>(&ku, &kvv, &ctl, &wts, (0.0, 1.0, 0.0, 1.0), 8.0, 0.0, 1e-3, band)));
            match res {
                Err(_) => writeln!(s, " face PANIC").unwrap(),
                Ok(res) => match res {
                Ok(fb) => writeln!(s, " face {} {}", iv(&fb.flux), iv(&fb.area)).unwrap(),
                Err(e) => writeln!(s, " face err {e:?}").unwrap(),
            }}
        }
    }
    std::fs::write(format!("{out_dir}/patch.txt"), s).unwrap();
    // ---- 1-D ladder ----
    let mut r = Rng(0xD1B54A32D192ED03);
    let mut s = String::new();
    let mut bad = 0;
    for case in 0..3000 {
        let p = 1 + r.u(5);
        let k = kv(&mut r, p, true);
        let n = k.control_count();
        let u: Vec<f64> = (0..n).map(|_| r.f() * 2.0 - 1.0).collect();
        let v: Vec<f64> = (0..n).map(|_| r.f() * 2.0 - 1.0).collect();
        let (a, b) = if r.u(2) == 0 { (0.0, 1.0) } else { let x = r.f(); let y = r.f(); (x.min(y), x.max(y)) };
        let pieces = [1, 4, 16, 64][r.u(4)];
        let ui: Vec<Interval> = u.iter().map(|x| Interval::point(*x)).collect();
        let vi: Vec<Interval> = v.iter().map(|x| Interval::point(*x)).collect();
        let res = green(&k, &ui, &vi, a, b, pieces);
        let t = truth(&k, &u, &v, a, b);
        let maxm = k.interior_knots().map(|(_, m)| m).max().unwrap_or(0);
        match res {
            Ok(x) => {
                let tol = 1e-12 * (1.0 + t.abs());
                let ok = x.lo() <= t + tol && t - tol <= x.hi();
                if !ok { bad += 1; }
                writeln!(s, "case {case} p={p} maxm={maxm} {} contains={ok} truth={t:e} w={:e}", iv(&x), x.hi()-x.lo()).unwrap();
            }
            Err(e) => writeln!(s, "case {case} p={p} maxm={maxm} err {e:?}").unwrap(),
        }
    }
    writeln!(s, "BAD {bad}").unwrap();
    std::fs::write(format!("{out_dir}/ladder.txt"), s).unwrap();
}

#[test]
fn delta2_collide() {
    let (a, b) = (0.5_f64, f64::from_bits(0.5_f64.to_bits() + 3));
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, a, b, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let (nu, nv) = (ku.control_count(), kv.control_count());
    let mut control = Vec::new();
    for i in 0..nu { for j in 0..nv { let (x, y) = (i as f64, j as f64); control.push(Point3::new(x * x * x, y, 0.0)); } }
    for (arm, w) in [("integral", 1.0), ("rational", 1.5)] {
        let mut weights = vec![1.0; nu * nv];
        weights[nv] = w;
        let n = NurbsSurface::new(ku.clone(), kv.clone(), control.clone(), weights).unwrap();
        for splits in [2, 4, 8, 12, 16] {
            match patch_cells_refined(&n, splits) {
                Err(e) => println!("COLLIDE {arm} @{splits}: Err {e:?}"),
                Ok(cells) => println!("COLLIDE {arm} @{splits}: Ok any S_uu.x hi>0 = {} max|S_uu.x| = {:e}", cells.iter().any(|c| c.s_uu[0].hi() > 0.0), cells.iter().map(|c| c.s_uu[0].hi().abs().max(c.s_uu[0].lo().abs())).fold(0.0, f64::max)),
            }
        }
        match patch_cells(&n) { Err(e) => println!("COLLIDE {arm} patch_cells: Err {e:?}"), Ok(c) => println!("COLLIDE {arm} patch_cells: Ok {}", c.len()) }
    }
}

#[test]
fn delta2_ladder_dyadic() {
    let out_dir = std::env::var("PROBE_OUT").unwrap();
    let mut r = Rng(0xA0761D6478BD642F);
    let mut s = String::new();
    let mut bad = 0;
    for case in 0..6000 {
        let p = 1 + r.u(5);
        let mut k = vec![0.0; p + 1];
        let n_int = 1 + r.u(3);
        let mut vals: Vec<f64> = (0..n_int).map(|_| (1 + r.u(7)) as f64 / 8.0).collect();
        vals.sort_by(f64::total_cmp); vals.dedup();
        for v in vals { let m = 1 + r.u(p); for _ in 0..m { k.push(v); } }
        k.extend(std::iter::repeat(1.0).take(p + 1));
        let k = KnotVector::clamped(k, p).unwrap();
        let n = k.control_count();
        let u: Vec<f64> = (0..n).map(|_| r.f() * 2.0 - 1.0).collect();
        let v: Vec<f64> = (0..n).map(|_| r.f() * 2.0 - 1.0).collect();
        let (a, b) = [(0.0, 1.0), (0.25, 0.75), (0.125, 1.0)][r.u(3)];
        let pieces = [8, 16, 64][r.u(3)];
        let ui: Vec<Interval> = u.iter().map(|x| Interval::point(*x)).collect();
        let vi: Vec<Interval> = v.iter().map(|x| Interval::point(*x)).collect();
        let res = green(&k, &ui, &vi, a, b, pieces);
        let t = truth(&k, &u, &v, a, b);
        let maxm = k.interior_knots().map(|(_, m)| m).max().unwrap_or(0);
        match res {
            Ok(x) => {
                let tol = 1e-13 * (1.0 + t.abs());
                let ok = x.lo() <= t + tol && t - tol <= x.hi();
                if !ok { bad += 1; }
                writeln!(s, "case {case} p={p} maxm={maxm} {} contains={ok} truth={t:e} w={:e} lo={:e} hi={:e}", iv(&x), x.hi()-x.lo(), x.lo(), x.hi()).unwrap();
            }
            Err(e) => writeln!(s, "case {case} p={p} maxm={maxm} err {e:?}").unwrap(),
        }
    }
    writeln!(s, "BAD {bad}").unwrap();
    std::fs::write(format!("{out_dir}/dy.txt"), s).unwrap();
}

#[test]
fn delta2_fixture() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let u: Vec<Interval> = [1.0; 6].iter().map(|x| Interval::point(*x)).collect();
    let v: Vec<Interval> = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0].iter().map(|x| Interval::point(*x)).collect();
    let x = green(&kv, &u, &v, 0.0, 1.0, 64).unwrap();
    println!("FIX2 m=p-1 p=3: [{:e}, {:e}]", x.lo(), x.hi());
    let mut r = Rng(0x1234567);
    let mut bad = 0; let mut n_ok = 0;
    for _ in 0..6000 {
        let p = 2 + r.u(4);
        let mut k = vec![0.0; p + 1];
        let mut vals: Vec<f64> = (0..1 + r.u(3)).map(|_| (1 + r.u(7)) as f64 / 8.0).collect();
        vals.sort_by(f64::total_cmp); vals.dedup();
        for vv in vals { let m = 1 + r.u(p); for _ in 0..m { k.push(vv); } }
        k.extend(std::iter::repeat(1.0).take(p + 1));
        let k = KnotVector::clamped(k, p).unwrap();
        let n = k.control_count();
        let c = r.f();
        let uf: Vec<f64> = vec![c; n];
        let vf: Vec<f64> = (0..n).map(|_| r.f() * 2.0 - 1.0).collect();
        let ui: Vec<Interval> = uf.iter().map(|x| Interval::point(*x)).collect();
        let vi: Vec<Interval> = vf.iter().map(|x| Interval::point(*x)).collect();
        let (a, b) = (0.0, 1.0);
        let t = truth(&k, &uf, &vf, a, b);
        if let Ok(x) = green(&k, &ui, &vi, a, b, 64) {
            n_ok += 1;
            let tol = 1e-13 * (1.0 + t.abs());
            if !(x.lo() <= t + tol && t - tol <= x.hi()) {
                bad += 1;
                if bad <= 5 { println!("CONSTU-BAD p={p} knots={:?} [{:e},{:e}] truth {t:e}", k.knots(), x.lo(), x.hi()); }
            }
        }
    }
    println!("CONSTU ok={n_ok} bad={bad}");
}
