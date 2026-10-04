//! Design-fork probe (fit budget): dense `solve_square` vs the same
//! no-pivot Doolittle restricted to the collocation matrix's band, on a
//! chord-parameterised loop. Reports time and whether control bits match.
use geom::curves::fit::{averaged_knot_vector, interpolate_columns};
use geom_core::spline::basis::basis_funs;
use std::time::Instant;

fn samples(n: usize) -> Vec<Vec<f64>> {
    // An ellipse-ish closed-ish loop of ~2.2 m, nonuniform spacing.
    (0..n)
        .map(|k| {
            let s = k as f64 / (n - 1) as f64;
            let a = 2.0 * std::f64::consts::PI * (s + 0.05 * (6.0 * s).sin() / 6.0);
            vec![0.45 * a.cos(), 0.25 * a.sin(), 0.02 * (3.0 * a).sin()]
        })
        .collect()
}

fn chord(rows: &[Vec<f64>]) -> Vec<f64> {
    let mut d = vec![0.0];
    for w in rows.windows(2) {
        let l: f64 = w[0].iter().zip(&w[1]).map(|(a, b)| (a - b) * (a - b)).sum::<f64>().sqrt();
        d.push(d.last().unwrap() + l);
    }
    let t = *d.last().unwrap();
    let mut p: Vec<f64> = d.iter().map(|x| x / t).collect();
    *p.last_mut().unwrap() = 1.0;
    p
}

fn banded(params: &[f64], rows: &[Vec<f64>], p: usize) -> Vec<Vec<f64>> {
    let n = params.len();
    let kv = averaged_knot_vector(params, p).unwrap();
    // Row i: first nonzero column lo[i], entries lo..lo+p.
    let w = 2 * p + 1;
    let mut lo = vec![0usize; n];
    let mut a = vec![0.0f64; n * w]; // a[i*w + (j - i + p)]
    let idx = |i: usize, j: usize| i * w + (j + p - i);
    for (i, u) in params.iter().enumerate() {
        let sp = kv.span_at(*u);
        let f = sp.first_control();
        let vals = basis_funs(sp, *u);
        // the dense path divides by the rational denominator with unit weights
        let den: f64 = vals.iter().fold(0.0, |acc, v| acc + v * 1.0);
        lo[i] = f;
        for (j, v) in vals.iter().enumerate() {
            a[idx(i, f + j)] = v * 1.0 / den;
        }
    }
    // hi[i]: last column with a possibly nonzero U entry in row i.
    let mut hi = vec![0usize; n];
    let mut m = 0;
    for i in 0..n {
        m = m.max(lo[i] + p);
        hi[i] = m.min(n - 1);
    }
    for i in 0..n {
        for j in i..=hi[i] {
            let mut s = a[idx(i, j)];
            for q in lo[i].max(j.saturating_sub(p))..i {
                s -= a[idx(i, q)] * a[idx(q, j)];
            }
            a[idx(i, j)] = s;
        }
        let piv = a[idx(i, i)];
        for r in (i + 1)..n.min(i + p + 1) {
            if lo[r] > i { continue; }
            let mut s = a[idx(r, i)];
            for q in lo[r].max(i.saturating_sub(p))..i {
                s -= a[idx(r, q)] * a[idx(q, i)];
            }
            a[idx(r, i)] = s / piv;
        }
    }
    let k = rows[0].len();
    let mut x = rows.to_vec();
    for c in 0..k {
        for i in 0..n {
            let mut s = x[i][c];
            for q in lo[i]..i { s -= a[idx(i, q)] * x[q][c]; }
            x[i][c] = s;
        }
        for i in (0..n).rev() {
            let mut s = x[i][c];
            for q in (i + 1)..=hi[i] { s -= a[idx(i, q)] * x[q][c]; }
            x[i][c] = s / a[idx(i, i)];
        }
    }
    x
}

fn main() {
    let ns: Vec<usize> = std::env::args().skip(1).map(|s| s.parse().unwrap()).collect();
    for n in ns {
        let rows = samples(n);
        let params = chord(&rows);
        let t0 = Instant::now();
        let b = banded(&params, &rows, 3);
        let tb = t0.elapsed().as_secs_f64();
        let dense = std::env::var("SKIP_DENSE").is_err();
        if dense {
            let t1 = Instant::now();
            let (_, d) = interpolate_columns(&params, 3, &rows).unwrap();
            let td = t1.elapsed().as_secs_f64();
            let mut diff = 0usize;
            let mut maxd = 0.0f64;
            for (r, s) in b.iter().zip(&d) {
                for (u, v) in r.iter().zip(s) {
                    if u.to_bits() != v.to_bits() { diff += 1; maxd = maxd.max((u - v).abs()); }
                }
            }
            println!("n={n} banded={tb:.4}s dense={td:.2}s differing_bits={diff} max_abs_diff={maxd:e}");
        } else {
            println!("n={n} banded={tb:.4}s");
        }
    }
}
