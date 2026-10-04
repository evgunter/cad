//! PR #4023 review probe: banded LU vs dense `solve_square`, bit for bit.
#![allow(clippy::all)]
use geom::{Collocation, NurbsCurve2, NurbsCurve3};
use geom_core::linalg::lsq::{self, LsqError};
use geom_core::spline::{KnotVector, basis};
use geom_core::{Point2, Point3};

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
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn averaged(params: &[f64], p: usize) -> Option<KnotVector> {
    let n = params.len();
    let mut k = vec![0.0f64; p + 1];
    for j in 1..(n - p) {
        let mut s = 0.0f64;
        for u in &params[j..j + p] {
            s += *u;
        }
        k.push(s / p as f64);
    }
    k.extend(std::iter::repeat_n(1.0, p + 1));
    KnotVector::clamped(k, p).ok()
}

/// Exactly fit.rs's `rational_row` with unit weights.
fn row(kv: &KnotVector, t: f64) -> (usize, Vec<f64>) {
    let span = kv.span_at(t);
    let n = basis::basis_funs(span, t);
    let first = span.first_control();
    let mut den = 0.0f64;
    for nj in &n {
        den += nj * 1.0;
    }
    (first, n.iter().map(|nj| nj * 1.0 / den).collect())
}

/// Raw (possibly rational, possibly unclamped) collocation row.
fn raw_row(knots: &[f64], p: usize, w: &[f64], t: f64) -> (usize, Vec<f64>) {
    let last = knots.len() - p - 2;
    let s = (p..=last).rev().find(|&s| knots[s] <= t).unwrap_or(p);
    let mut n = vec![1.0f64];
    for d in 1..=p {
        let mut next = vec![0.0f64; d + 1];
        for (r, v) in n.iter().enumerate() {
            let j = s - (d - 1) + r;
            let (lo, hi) = (knots[j], knots[j + d]);
            let a = (t - lo) / (hi - lo);
            next[r] += (1.0 - a) * v;
            next[r + 1] += a * v;
        }
        n = next;
    }
    let first = s - p;
    let den: f64 = n.iter().zip(&w[first..]).map(|(v, w)| v * w).sum();
    (first, n.iter().zip(&w[first..]).map(|(v, ww)| v * ww / den).collect())
}

fn dense_of(first: &[usize], rows: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = rows.len();
    first
        .iter()
        .zip(rows)
        .map(|(f, r)| {
            let mut d = vec![0.0f64; n];
            for (j, v) in r.iter().enumerate() {
                if f + j < n {
                    d[f + j] = *v;
                }
            }
            d
        })
        .collect()
}

#[derive(Default, Debug)]
struct Tally {
    cases: usize,
    strict_diff: usize,  // any bit differs (NaN payloads included)
    nanloose_diff: usize, // differs with NaN==NaN
    accept_mismatch: usize,
    err_mismatch: usize,  // both Err, different error (PartialEq)
    err_bits_mismatch: usize, // both Err, same PartialEq, pivot bits differ
}

fn cmp(
    t: &mut Tally,
    label: &str,
    first: &[usize],
    rows: &[Vec<f64>],
    b: &[Vec<f64>],
    verbose: bool,
) {
    t.cases += 1;
    let dense = lsq::solve_square(&dense_of(first, rows), b);
    let banded = lsq::factor_banded(first, rows).and_then(|lu| lu.solve(b));
    match (&dense, &banded) {
        (Ok(d), Ok(bb)) => {
            let mut strict = 0;
            let mut loose = 0;
            let mut ex = None;
            for (i, (rd, rb)) in d.iter().zip(bb).enumerate() {
                for (c, (u, v)) in rd.iter().zip(rb).enumerate() {
                    if u.to_bits() != v.to_bits() {
                        strict += 1;
                        if !(u.is_nan() && v.is_nan()) {
                            loose += 1;
                            ex.get_or_insert((i, c, *u, *v));
                        } else {
                            ex.get_or_insert((i, c, *u, *v));
                        }
                    }
                }
            }
            if strict > 0 {
                t.strict_diff += 1;
            }
            if loose > 0 {
                t.nanloose_diff += 1;
            }
            if (strict > 0 && verbose) || loose > 0 {
                if t.strict_diff + t.nanloose_diff <= 12 {
                    println!(
                        "DIFF {label} n={} strict={strict} loose={loose} first={:?}",
                        rows.len(),
                        ex.map(|(i, c, u, v)| (i, c, u, u.to_bits(), v, v.to_bits()))
                    );
                }
            }
        }
        (Err(a), Err(c)) => {
            let same = a == c || matches!((a, c), (LsqError::LsqDegenerate { pivot_index: i1, pivot: p1 }, LsqError::LsqDegenerate { pivot_index: i2, pivot: p2 }) if i1 == i2 && p1.is_nan() && p2.is_nan());
            if !same {
                t.err_mismatch += 1;
                if t.err_mismatch <= 6 { println!("ERRDIFF {label}: dense {a:?} banded {c:?}"); }
            } else if let (
                LsqError::LsqDegenerate { pivot: p1, .. },
                LsqError::LsqDegenerate { pivot: p2, .. },
            ) = (a, c)
            {
                if p1.to_bits() != p2.to_bits() {
                    t.err_bits_mismatch += 1;
                    if t.err_bits_mismatch <= 5 {
                        println!("ERRBITS {label}: dense {a:?} banded {c:?}");
                    }
                }
            }
        }
        _ => {
            t.accept_mismatch += 1;
            if t.accept_mismatch <= 8 {
                println!(
                    "ACCEPT-MISMATCH {label} n={}: dense ok={} banded ok={} ({:?} / {:?})",
                    rows.len(),
                    dense.is_ok(),
                    banded.is_ok(),
                    dense.as_ref().err(),
                    banded.as_ref().err()
                );
            }
        }
    }
}

fn special(r: &mut Rng) -> f64 {
    let pool = [
        0.0,
        -0.0,
        f64::from_bits(1),
        -f64::from_bits(1),
        1e-310,
        -2.2e-308,
        1e300,
        -1e308,
        f64::MAX,
        -f64::MAX,
        1e-300,
        f64::NAN,
        -f64::NAN,
        f64::from_bits(0x7ff0_0000_0000_1234), // signalling-ish NaN payload
        f64::from_bits(0x7ff8_dead_beef_0001),
        f64::INFINITY,
        f64::NEG_INFINITY,
        1.0,
        -3.5,
    ];
    pool[r.below(pool.len())]
}

/// RHS generator families.
fn rhs(r: &mut Rng, n: usize, fam: usize, k: usize) -> Vec<Vec<f64>> {
    (0..n)
        .map(|i| {
            (0..k)
                .map(|c| match (fam + c) % 6 {
                    0 => (r.f() - 0.5) * 10.0,
                    1 => {
                        if r.below(2) == 0 {
                            -0.0
                        } else {
                            0.0
                        }
                    }
                    2 => {
                        // mostly signed zeros, occasional value
                        let z = if r.below(2) == 0 { -0.0 } else { 0.0 };
                        if r.below(10) == 0 { (r.f() - 0.5) * 1e-300 } else { z }
                    }
                    3 => special(r),
                    4 => {
                        // finite specials only (no NaN/inf)
                        let v = special(r);
                        if v.is_finite() { v } else { -0.0 }
                    }
                    _ => {
                        // block patterns: -0 runs then nonfinite near i
                        if i % 7 < 3 { -0.0 } else if i % 13 == 5 { f64::INFINITY } else { 0.0 }
                    }
                })
                .collect()
        })
        .collect()
}

fn params_family(r: &mut Rng, n: usize, fam: usize) -> Vec<f64> {
    let mut p: Vec<f64> = match fam {
        0 => (0..n).map(|i| i as f64 / (n - 1) as f64).collect(),
        1 | 2 => {
            // chord / centripetal on random points
            let pts: Vec<[f64; 3]> = (0..n).map(|_| [r.f(), r.f(), r.f()]).collect();
            let mut d: Vec<f64> = pts
                .windows(2)
                .map(|w| {
                    let s = ((w[1][0] - w[0][0]).powi(2)
                        + (w[1][1] - w[0][1]).powi(2)
                        + (w[1][2] - w[0][2]).powi(2))
                    .sqrt();
                    if fam == 2 { s.sqrt() } else { s }
                })
                .collect();
            let tot: f64 = d.iter().sum();
            for x in d.iter_mut() {
                *x /= tot;
            }
            let mut acc = 0.0;
            let mut v = vec![0.0];
            for x in d {
                acc += x;
                v.push(acc);
            }
            v
        }
        3 => {
            // nearly coincident clusters
            let mut v = vec![0.0];
            let mut t = 0.0f64;
            for i in 1..n {
                t = if i % 3 == 0 { t + 1.0 / n as f64 } else { f64::from_bits(t.to_bits() + 1 + r.below(4) as u64) };
                v.push(t);
            }
            let last = *v.last().unwrap();
            v.iter().map(|x| x / last).collect()
        }
        _ => {
            // wildly graded
            let mut v = vec![0.0];
            let mut t = 0.0f64;
            for _ in 1..n {
                t += 10f64.powf(-12.0 * r.f());
                v.push(t);
            }
            let last = *v.last().unwrap();
            v.iter().map(|x| x / last).collect()
        }
    };
    let m = p.len();
    p[0] = 0.0;
    p[m - 1] = 1.0;
    p
}

fn main() {
    let mut r = Rng(0x9e37_79b9_7f4a_7c15);
    let verbose = std::env::args().any(|a| a == "-v");

    // ---------- 1. Collocation matrices: fit's own rows ----------
    let mut t = Tally::default();
    let mut skipped = 0;
    let mut sizes: Vec<usize> = (2..=40).collect();
    sizes.extend([41, 63, 64, 65, 99, 101, 127, 255, 257, 511, 1001]);
    for &n in &sizes {
        for p in 1..=5usize {
            if n < p + 1 {
                continue;
            }
            for pf in 0..5 {
                let params = params_family(&mut r, n, pf);
                if !params.windows(2).all(|w| w[0] < w[1]) {
                    skipped += 1;
                    continue;
                }
                let Some(kv) = averaged(&params, p) else {
                    skipped += 1;
                    continue;
                };
                let (first, rows): (Vec<usize>, Vec<Vec<f64>>) =
                    params.iter().map(|u| row(&kv, *u)).unzip();
                for fam in 0..6 {
                    let b = rhs(&mut r, n, fam, 4);
                    cmp(&mut t, &format!("coll p={p} pf={pf} fam={fam}"), &first, &rows, &b, verbose);
                }
                // fit-level door: compare against dense on the same rows
                if n >= p + 1 && n >= 2 {
                    let pts: Vec<Point3<f64>> = rhs(&mut r, n, 3, 3)
                        .iter()
                        .map(|v| Point3::new(v[0], v[1], v[2]))
                        .collect();
                    let dense = lsq::solve_square(
                        &dense_of(&first, &rows),
                        &pts.iter().map(|q| vec![q.x, q.y, q.z]).collect::<Vec<_>>(),
                    );
                    let fit = NurbsCurve3::<f64>::interpolate_with_params(&pts, p, &params);
                    t.cases += 1;
                    match (dense, fit) {
                        (Ok(d), Ok(c)) => {
                            let cb: Vec<u64> = c.control().iter().flat_map(|q| [q.x, q.y, q.z]).map(f64::to_bits).collect();
                            let db: Vec<u64> = d.iter().flatten().map(|v| v.to_bits()).collect();
                            let loose = c.control().iter().flat_map(|q| [q.x, q.y, q.z]).zip(d.iter().flatten()).filter(|(a, b)| a.to_bits() != b.to_bits() && !(a.is_nan() && b.is_nan())).count();
                            if cb != db {
                                t.strict_diff += 1;
                                if loose > 0 { t.nanloose_diff += 1; println!("FITDIFF n={n} p={p} pf={pf} loose={loose}"); }
                            }
                        }
                        (Err(_), Err(_)) => {}
                        (d, f) => {
                            t.accept_mismatch += 1;
                            println!("FIT-ACCEPT-MISMATCH n={n} p={p} pf={pf}: dense {:?} fit {:?}", d.err(), f.err());
                        }
                    }
                    // 2-D door through a shared Collocation
                    if let Ok(col) = Collocation::new(&params, p) {
                        let uv: Vec<Point2<f64>> = pts.iter().map(|q| Point2::new(q.y, q.z)).collect();
                        let a = NurbsCurve2::<f64>::interpolate_on(&uv, &col);
                        let b2 = NurbsCurve2::<f64>::interpolate_with_params(&uv, p, &params);
                        match (a, b2) {
                            (Ok(a), Ok(b2)) => {
                                let ab: Vec<u64> = a.control().iter().flat_map(|q| [q.x, q.y]).map(f64::to_bits).collect();
                                let bb: Vec<u64> = b2.control().iter().flat_map(|q| [q.x, q.y]).map(f64::to_bits).collect();
                                if ab != bb { println!("ON-vs-WITHPARAMS differ n={n} p={p}"); t.strict_diff += 1; }
                            }
                            (a, b2) => println!("ON-vs-WITHPARAMS outcome differ n={n} p={p}: {:?} {:?}", a.err(), b2.err()),
                        }
                    }
                }
            }
        }
    }
    println!("collocation (fit rows): {t:?} skipped_params={skipped}");

    // ---------- 2. Rational and unclamped raw collocation ----------
    let mut t = Tally::default();
    for &n in &[4usize, 5, 7, 11, 23, 50, 120] {
        for p in 1..=5usize {
            if n < p + 1 {
                continue;
            }
            let params = params_family(&mut r, n, 1);
            let kv = averaged(&params, p).unwrap();
            let w: Vec<f64> = (0..n).map(|_| 0.05 + 20.0 * r.f()).collect();
            let (first, rows): (Vec<usize>, Vec<Vec<f64>>) =
                params.iter().map(|u| raw_row(kv.knots(), p, &w, *u)).unzip();
            for fam in 0..6 {
                let b = rhs(&mut r, n, fam, 3);
                cmp(&mut t, &format!("rational p={p}"), &first, &rows, &b, verbose);
            }
            if n > p {
                let knots: Vec<f64> = (0..n + p + 1).map(|i| i as f64).collect();
                let interior: Vec<f64> = (0..n)
                    .map(|i| p as f64 + (i as f64 + 0.5) * (n - p) as f64 / n as f64)
                    .collect();
                let (first, rows): (Vec<usize>, Vec<Vec<f64>>) = interior
                    .iter()
                    .map(|u| raw_row(&knots, p, &vec![1.0; n], *u))
                    .unzip();
                for fam in 0..6 {
                    let b = rhs(&mut r, n, fam, 3);
                    cmp(&mut t, &format!("unclamped p={p}"), &first, &rows, &b, verbose);
                }
            }
        }
    }
    println!("rational/unclamped: {t:?}");

    // ---------- 3. Generic banded matrices (pub API contract) ----------
    // 3a: finite, no -0.0 entries, random structure incl. off-diagonal windows
    let mut t = Tally::default();
    for _ in 0..40000 {
        let n = 1 + r.below(9);
        let w = 1 + r.below(n.min(4));
        let mut first = Vec::with_capacity(n);
        let mut f = 0usize;
        for _ in 0..n {
            if r.below(3) == 0 && f + w < n {
                f += 1;
            }
            first.push(f.min(n - w));
        }
        let rows: Vec<Vec<f64>> = (0..n)
            .map(|_| {
                (0..w)
                    .map(|_| match r.below(8) {
                        0 => 0.0,
                        1 => 1.0,
                        2 => 0.5,
                        3 => 1e-300 * r.f(),
                        _ => (r.f() - 0.3) * 4.0,
                    })
                    .collect()
            })
            .collect();
        let fam = r.below(6); let b = rhs(&mut r, n, fam, 2);
        cmp(&mut t, "generic-finite", &first, &rows, &b, verbose);
    }
    println!("generic finite (no -0 entries): {t:?}");

    // 3b: entries may be -0.0
    let mut t = Tally::default();
    for _ in 0..40000 {
        let n = 2 + r.below(8);
        let w = 1 + r.below(n.min(4));
        let mut first = Vec::with_capacity(n);
        let mut f = 0usize;
        for i in 0..n {
            if i >= f + w && f + w < n {
                f += 1;
            }
            if r.below(3) == 0 && f + w < n && f < i {
                f += 1;
            }
            first.push(f.min(n - w));
        }
        let rows: Vec<Vec<f64>> = (0..n)
            .map(|_| {
                (0..w)
                    .map(|_| match r.below(6) {
                        0 => -0.0,
                        1 => 0.0,
                        _ => (r.f() - 0.3) * 4.0,
                    })
                    .collect()
            })
            .collect();
        let b = rhs(&mut r, n, 0, 1);
        cmp(&mut t, "generic-negzero-entries", &first, &rows, &b, verbose);
    }
    println!("generic with -0.0 entries: {t:?}");

    // 3c: entries may be non-finite or huge (overflow)
    let mut t = Tally::default();
    for _ in 0..40000 {
        let n = 2 + r.below(8);
        let w = 1 + r.below(n.min(4));
        let mut first = Vec::with_capacity(n);
        let mut f = 0usize;
        for i in 0..n {
            if i >= f + w && f + w < n {
                f += 1;
            }
            if r.below(3) == 0 && f + w < n && f < i {
                f += 1;
            }
            first.push(f.min(n - w));
        }
        let rows: Vec<Vec<f64>> = (0..n)
            .map(|_| {
                (0..w)
                    .map(|_| match r.below(10) {
                        0 => special(&mut r),
                        1 => 1e200,
                        2 => -1e200,
                        _ => (r.f() - 0.3) * 4.0,
                    })
                    .collect()
            })
            .collect();
        let b = rhs(&mut r, n, 0, 1);
        cmp(&mut t, "generic-nonfinite-entries", &first, &rows, &b, verbose);
    }
    println!("generic with non-finite/huge entries: {t:?}");

    // ---------- 4. Hand-built witnesses ----------
    // 4a: finite input, U overflow outside a later row's window.
    let first = vec![0usize, 0, 2, 2, 2];
    let rows = vec![
        vec![1.0, 0.0, 1e200],
        vec![1e200, 1.0, 0.0],
        vec![1.0, 0.1, 0.1],
        vec![0.1, 1.0, 0.1],
        vec![0.1, 0.1, 1.0],
    ];
    let b = vec![vec![1.0]; 5];
    println!(
        "4a overflow witness: dense {:?}\n   banded {:?}",
        lsq::solve_square(&dense_of(&first, &rows), &b),
        lsq::factor_banded(&first, &rows).and_then(|l| l.solve(&b))
    );
    // 4b: NaN entry in row 0's window, above a row whose window starts later.
    let first = vec![0usize, 1, 1];
    let rows = vec![vec![1.0, f64::NAN], vec![1.0, 0.5], vec![0.5, 1.0]];
    let b = vec![vec![1.0]; 3];
    println!(
        "4b NaN-entry witness: dense {:?}\n   banded {:?}",
        lsq::solve_square(&dense_of(&first, &rows), &b),
        lsq::factor_banded(&first, &rows).and_then(|l| l.solve(&b))
    );
    // 4c: -0.0 entry in a window flipped by a dropped term.
    let first = vec![0usize, 0, 1];
    let rows = vec![vec![-1.0, 1.0], vec![1.0, 2.0], vec![-0.0, 1.0]];
    let b = vec![vec![-1.0], vec![2.0], vec![-0.0]];
    let d = lsq::solve_square(&dense_of(&first, &rows), &b);
    let bb = lsq::factor_banded(&first, &rows).and_then(|l| l.solve(&b));
    println!("4c -0 entry: dense {d:?}\n   banded {bb:?}");
}
