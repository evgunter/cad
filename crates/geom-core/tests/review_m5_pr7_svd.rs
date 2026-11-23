//! **Promoted PR 7 review probe** (adversarial reviewer, adopted
//! verbatim as a committed row per the standing review policy).

//! REVIEW PROBE (PR 7): differential test of the fixed-shape SVD
//! against independent closed-form eigenvalues of A·Aᵀ, on random
//! matrices including near-rank-deficient ones.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    // Adopted verbatim from the reviewer's probe: the indexing and
    // cast shapes are theirs, and rewriting them would weaken the
    // "independent second implementation" the row exists to be.
    clippy::needless_range_loop,
    clippy::unnecessary_cast
)]

test_utils::gated_to!["crates/geom-core/src/linalg/"];

use geom_core::linalg::svd::{Svd2x3, Svd3x4};
use test_utils::fuzz;

/// Uniform in `[-1, 1)`, the shape the reviewer's probe drew.
fn signed_unit(rng: &mut fuzz::Rng) -> f64 {
    rng.range(-1.0, 1.0)
}

/// Eigenvalues of a symmetric 2x2 (closed form, independent of the SVD).
fn eig2(m: [[f64; 2]; 2]) -> [f64; 2] {
    let tr = m[0][0] + m[1][1];
    let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
    let disc = (tr * tr / 4.0 - det).max(0.0).sqrt();
    [tr / 2.0 + disc, tr / 2.0 - disc]
}

/// Eigenvalues of a symmetric 3x3 via the trigonometric closed form.
fn eig3(a: [[f64; 3]; 3]) -> [f64; 3] {
    let p1 = a[0][1] * a[0][1] + a[0][2] * a[0][2] + a[1][2] * a[1][2];
    let q = (a[0][0] + a[1][1] + a[2][2]) / 3.0;
    let p2 = (a[0][0] - q).powi(2) + (a[1][1] - q).powi(2) + (a[2][2] - q).powi(2) + 2.0 * p1;
    let p = (p2 / 6.0).sqrt();
    if p == 0.0 {
        return [q, q, q];
    }
    let mut b = [[0.0f64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            b[i][j] = (a[i][j] - if i == j { q } else { 0.0 }) / p;
        }
    }
    let detb = b[0][0] * (b[1][1] * b[2][2] - b[1][2] * b[2][1])
        - b[0][1] * (b[1][0] * b[2][2] - b[1][2] * b[2][0])
        + b[0][2] * (b[1][0] * b[2][1] - b[1][1] * b[2][0]);
    let r = (detb / 2.0).clamp(-1.0, 1.0);
    let phi = r.acos() / 3.0;
    let e1 = q + 2.0 * p * phi.cos();
    let e3 = q + 2.0 * p * (phi + 2.0 * std::f64::consts::PI / 3.0).cos();
    let e2 = 3.0 * q - e1 - e3;
    [e1, e2, e3]
}

#[test]
fn svd2x3_matches_independent_eigenvalues_and_solves() {
    let mut rng = fuzz::start("review_m5_pr7_svd::svd2x3");
    for case in 0..fuzz::scaled(500) {
        let mut a = [[0.0f64; 3]; 2];
        for row in a.iter_mut() {
            for v in row.iter_mut() {
                *v = signed_unit(&mut rng) * 3.0;
            }
        }
        // Every 4th case: make the rows nearly parallel (near rank 1).
        if case % 4 == 0 {
            let t = 10f64.powi(-(((case / 4) % 14) as i32));
            for j in 0..3 {
                a[1][j] = 1.7 * a[0][j] + t * signed_unit(&mut rng);
            }
        }
        let s = Svd2x3::new(a);
        // Independent singular values from A·Aᵀ.
        let mut g = [[0.0f64; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                g[i][j] = (0..3).map(|k| a[i][k] * a[j][k]).sum();
            }
        }
        let e = eig2(g);
        let scale = e[0].sqrt().max(1e-30);
        let (smax, smin) = (s.sigma_max(), s.sigma_min());
        // Compare in the squared domain at the Gram matrix's own
        // absolute scale: the eig reference cannot resolve σ below
        // ~sqrt(ulp)·σmax, while one-sided Jacobi can.
        assert!(
            (smax * smax - e[0]).abs() <= 1e-12 * e[0].max(1.0),
            "case {case}: smax {smax} vs eig {} — {}",
            e[0].sqrt(),
            fuzz::replay()
        );
        assert!(
            (smin * smin - e[1]).abs() <= 1e-12 * e[0].max(1.0),
            "case {case}: smin {smin} vs eig {} (a={a:?}) — {}",
            e[1].max(0.0).sqrt(),
            fuzz::replay()
        );
        // Null direction annihilated + unit.
        let n = s.null_direction();
        let nn: f64 = n.iter().map(|x| x * x).sum::<f64>().sqrt();
        assert!(
            (nn - 1.0).abs() < 1e-12,
            "case {case}: |n| = {nn} — {}",
            fuzz::replay()
        );
        for i in 0..2 {
            let r: f64 = (0..3).map(|j| a[i][j] * n[j]).sum();
            assert!(
                r.abs() <= 1e-10 * scale,
                "case {case}: A n = {r} — {}",
                fuzz::replay()
            );
        }
        // Min-norm solve, only when well-conditioned.
        if smin > 1e-6 * scale {
            let b = [signed_unit(&mut rng), signed_unit(&mut rng)];
            let x = s.solve_min_norm(&b);
            for i in 0..2 {
                let r: f64 = (0..3).map(|j| a[i][j] * x[j]).sum();
                assert!(
                    (r - b[i]).abs()
                        <= 1e-8
                            * (1.0 + x.iter().map(|v| v.abs()).fold(0.0, f64::max))
                            * scale.max(1.0),
                    "case {case}: residual {r} vs {} — {}",
                    b[i],
                    fuzz::replay()
                );
            }
            let d: f64 = x.iter().zip(n.iter()).map(|(p, q)| p * q).sum();
            assert!(
                d.abs() <= 1e-8 * (1.0 + nn),
                "case {case}: not min-norm, x.n = {d} — {}",
                fuzz::replay()
            );
        }
    }
}

#[test]
fn svd3x4_matches_independent_eigenvalues() {
    let mut rng = fuzz::start("review_m5_pr7_svd::svd3x4");
    for case in 0..fuzz::scaled(500) {
        let mut a = [[0.0f64; 4]; 3];
        for row in a.iter_mut() {
            for v in row.iter_mut() {
                *v = signed_unit(&mut rng) * 2.0;
            }
        }
        if case % 5 == 0 {
            // Near rank 2: row 2 a combination of rows 0 and 1.
            let t = 10f64.powi(-(((case / 5) % 14) as i32));
            for j in 0..4 {
                a[2][j] = 0.6 * a[0][j] - 1.1 * a[1][j] + t * signed_unit(&mut rng);
            }
        }
        let s = Svd3x4::new(a);
        let mut g = [[0.0f64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                g[i][j] = (0..4).map(|k| a[i][k] * a[j][k]).sum();
            }
        }
        let e = eig3(g);
        let scale = e[0].max(0.0).sqrt().max(1e-30);
        assert!(
            (s.sigma_max() * s.sigma_max() - e[0]).abs() <= 1e-11 * e[0].max(1.0),
            "case {case}: smax {} vs {} — {}",
            s.sigma_max(),
            e[0].sqrt(),
            fuzz::replay()
        );
        assert!(
            (s.sigma_min() * s.sigma_min() - e[2]).abs() <= 1e-11 * e[0].max(1.0),
            "case {case}: smin {} vs {} — {}",
            s.sigma_min(),
            e[2].max(0.0).sqrt(),
            fuzz::replay()
        );
        let n = s.null_direction();
        for i in 0..3 {
            let r: f64 = (0..4).map(|j| a[i][j] * n[j]).sum();
            assert!(
                r.abs() <= 1e-9 * scale,
                "case {case}: A n = {r} — {}",
                fuzz::replay()
            );
        }
        // Reconstruction.
        let rec = s.reconstruct();
        for i in 0..3 {
            for j in 0..4 {
                assert!(
                    (rec[i][j] - a[i][j]).abs() <= 1e-8 * scale,
                    "case {case}: rec {} vs {} — {}",
                    rec[i][j],
                    a[i][j],
                    fuzz::replay()
                );
            }
        }
    }
}
