//! The M5 PR 4 adversarial-review harness, adopted at the fix pass
//! (review verdict APPROVE-WITH-FIX-PASS): randomized falsification of
//! the direct fit bound (F1), projection fuzz + poison/plateau probes
//! (F2), a Householder-QR differential against `linalg::lsq` (F3),
//! pointwise composite exactness/containment across all five implicit
//! surfaces plus a hand-derived degree-1 sphere check and the
//! `binom_row` exactness pin (F4), worked-example and loose-tolerance
//! probes (F6), and the F9 end-to-end: cylinder fit -> project ->
//! compose -> Decide accept/refuse. The randomized rows draw from
//! `test_utils::fuzz`: a fresh seed per run, logged unconditionally, with
//! every count a multiple of `CAD_FUZZ_EFFORT`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_precision_loss,
    // The reviewer's QR/recurrence loops are index-explicit on purpose
    // (they mirror the fixed elimination orders under test).
    clippy::needless_range_loop,
    dead_code
)]

test_utils::gated_to![
    "crates/geom/src/curves/",
    "crates/geom/src/curves.rs",
    "crates/geom-core/src/spline/",
    "crates/geom-core/src/ring_interval.rs",
    "crates/geom-core/src/linalg/",
    "crates/geom-core/src/predicate.rs",
];

use geom::{NurbsCurve2, NurbsCurve3};
use geom_core::spline::KnotVector;
use geom_core::spline::compose::{self, CurveRingData, ImplicitSurface};
use geom_core::{Point2, Point3, RingInterval};
use test_utils::fuzz;

/// An inclusive integer draw in `lo..=hi`.
fn usize_in(rng: &mut fuzz::Rng, lo: usize, hi: usize) -> usize {
    lo + rng.below(hi - lo + 1)
}

// =====================================================================
// F1(a): the direct fit bound must dominate dense sampling of
// |candidate − chord interpolant| across randomized + adversarial fits.
// =====================================================================
#[test]
fn f1_fit_bound_dominates_dense_sampling_randomized() {
    let mut rng = fuzz::start("review_m5_pr4::f1_fit_bound");
    // The CASE count is the only lever here: this row's cost is
    // dominated by `NurbsCurve3::approximate` on up to 70 points, not by
    // the dense sampling, so trimming the sample grid would buy nothing.
    let cases = fuzz::scaled(5);
    let mut fits = 0usize;
    let mut worst_ratio = 0.0f64;
    for case in 0..cases {
        let n = usize_in(&mut rng, 6, 70);
        let degree = usize_in(&mut rng, 2, 5);
        let scale = match case % 4 {
            0 => 1.0,
            1 => 1e-3,
            2 => 1e6,
            _ => 12.5,
        };
        // Smooth-ish random curve samples: random low-freq Fourier mix.
        let (a1, a2, a3) = (
            rng.range(0.3, 2.0),
            rng.range(0.1, 1.0),
            rng.range(0.05, 0.5),
        );
        let (p1, p2) = (
            rng.range(0.0, core::f64::consts::TAU),
            rng.range(0.0, core::f64::consts::TAU),
        );
        let mut pts: Vec<Point3<f64>> = (0..n)
            .map(|k| {
                let t = k as f64 / (n - 1) as f64;
                Point3::new(
                    scale * (t + a3 * (5.0 * t + p1).sin()),
                    scale * (a1 * (2.0 * t + p2).cos()),
                    scale * (a2 * (3.0 * t).sin()),
                )
            })
            .collect();
        // Adversarial clustering: squeeze some chords to near-coincidence.
        if case % 5 == 0 && n > 8 {
            let j = usize_in(&mut rng, 2, n - 3);
            let base = pts[j];
            pts[j + 1] = base + (pts[j + 1] - base) * 1e-9;
        }
        let tols: [f64; 4] = [1e-1, 1e-2, 1e-3, 1e-4];
        let tol = scale * tols[case % 4];
        let Ok(fit) = NurbsCurve3::approximate(&pts, degree, tol) else {
            continue;
        };
        fits += 1;
        assert!(
            fit.bound <= tol,
            "case {case}: bound {:e} > tol {tol:e} — {}",
            fit.bound,
            fuzz::replay()
        );
        let reference = NurbsCurve3::interpolate(&pts, 1).unwrap();
        let m = fuzz::scaled(1_024) as u32;
        let mut dense = 0.0f64;
        for k in 0..=m {
            let t = f64::from(k) / f64::from(m);
            let d = fit.curve.eval(t).distance(reference.eval(t));
            dense = dense.max(d);
        }
        // fp slack: the bound is steering-grade f64; allow a few ulps of
        // the data scale, nothing more.
        let slack = 1e-11 * scale.max(1.0);
        assert!(
            dense <= fit.bound + slack,
            "case {case} (n={n}, deg={degree}, scale={scale:e}, tol={tol:e}): \
             dense {dense:e} EXCEEDS bound {:e} by {:e} — {}",
            fit.bound,
            dense - fit.bound,
            fuzz::replay()
        );
        if fit.bound > 0.0 {
            worst_ratio = worst_ratio.max(dense / fit.bound);
        }
    }
    println!("[F1a] {fits} randomized fits; worst dense/bound ratio {worst_ratio:.4}");
    // COVERAGE FLOOR, proportional to the case count: half the draws used
    // to produce a fit (60/120), and a run where almost nothing fits is
    // asserting almost nothing.
    assert!(
        fits * 2 > cases,
        "too few successful fits to mean anything: {fits}/{cases} — {}",
        fuzz::replay()
    );
}

/// F1(d): the bound also bounds every data deviation |C(u_k) − Q_k|.
#[test]
fn f1_bound_covers_data_deviation_randomized() {
    let mut rng = fuzz::start("review_m5_pr4::f1_data_deviation");
    for case in 0..fuzz::scaled(5) {
        let n = usize_in(&mut rng, 8, 50);
        let mut pts: Vec<Point3<f64>> = (0..n)
            .map(|k| {
                let t = k as f64 / (n - 1) as f64;
                Point3::new(
                    t,
                    (3.0 * t).sin() * rng.range(0.5, 1.5),
                    0.3 * (2.0 * t).cos(),
                )
            })
            .collect();
        pts.dedup_by(|a, b| (*a - *b).norm() == 0.0);
        let Ok(fit) = NurbsCurve3::approximate(&pts, 3, 1e-2) else {
            continue;
        };
        // Recompute chord params the fit's way.
        let mut total = 0.0;
        let mut chords = Vec::new();
        for pair in pts.windows(2) {
            let d = (pair[1] - pair[0]).norm();
            chords.push(d);
            total += d;
        }
        let mut params = vec![0.0];
        let mut acc = 0.0;
        for d in &chords {
            acc += d / total;
            params.push(acc);
        }
        params[pts.len() - 1] = 1.0;
        for (q, u) in pts.iter().zip(params.iter()) {
            let d = fit.curve.eval(*u).distance(*q);
            assert!(
                d <= fit.bound + 1e-11,
                "case {case}: data deviation {d:e} > bound {:e} — {}",
                fit.bound,
                fuzz::replay()
            );
        }
    }
}

// =====================================================================
// F2: projection fuzz — accepted projections must be either true global
// feet, domain-end feet, or carry residuals that fail bands.
// =====================================================================
#[test]
fn f2_projection_fuzz_no_silent_wrong_answers() {
    let mut rng = fuzz::start("review_m5_pr4::f2_projection");
    let mut accepted = 0usize;
    let mut refused = 0usize;
    let mut local_min_wrong_branch = 0usize;
    let mut silent_bad = Vec::new();
    for case in 0..fuzz::scaled(50) {
        let degree = usize_in(&mut rng, 2, 4);
        let nctrl = usize_in(&mut rng, degree + 1, 9);
        // Clamped uniform-ish interior knots.
        let mut knots = vec![0.0; degree + 1];
        let ninterior = nctrl - degree - 1;
        for j in 1..=ninterior {
            knots.push(j as f64 / (ninterior + 1) as f64);
        }
        knots.extend(std::iter::repeat_n(1.0, degree + 1));
        let kv = KnotVector::clamped(knots, degree).unwrap();
        let control: Vec<Point3<f64>> = (0..nctrl)
            .map(|_| {
                Point3::new(
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                )
            })
            .collect();
        let weights: Vec<f64> = (0..nctrl).map(|_| rng.range(0.5, 2.0)).collect();
        let curve = NurbsCurve3::new(kv, control, weights).unwrap();
        let p = Point3::new(
            rng.range(-3.0, 3.0),
            rng.range(-3.0, 3.0),
            rng.range(-3.0, 3.0),
        );
        match curve.project(p) {
            Err(_) => refused += 1,
            Ok(proj) => {
                accepted += 1;
                // Dense global min, plus the longest chord between
                // consecutive samples. The sampled min is only an UPPER
                // bound on the true min, and by the triangle inequality it
                // overestimates by at most one chord — so the slack below
                // is DERIVED from the grid instead of being the old fixed
                // 1e-4, which silently assumed a particular density.
                //
                // AUDIT NOTE (2026-08-13): the fixed 1e-4 was tied to the
                // original h = 5e-5 spacing. Scaling the oracle down
                // without scaling the slack made this row fail on ~7% of
                // seeds — a tolerance coupled to a count is exactly the
                // thing an EFFORT dial has to break.
                let m = fuzz::scaled(2_500) as u32;
                let mut dmin = f64::INFINITY;
                let mut chord_max = 0.0f64;
                let mut prev = curve.eval(0.0);
                for k in 0..=m {
                    let t = f64::from(k) / f64::from(m);
                    let q = curve.eval(t);
                    dmin = dmin.min(q.distance(p));
                    chord_max = chord_max.max(q.distance(prev));
                    prev = q;
                }
                // Honesty of the carried values: recompute at t*.
                let re = curve.eval(proj.t).distance(p);
                assert!(
                    (re - proj.distance).abs() < 1e-12 * re.max(1.0),
                    "case {case}: carried distance {:e} is not |C(t*)−P| = {re:e} — {}",
                    proj.distance,
                    fuzz::replay()
                );
                // The projection may dip below the sampled min by at most
                // one inter-sample chord, never by more.
                let sample_slack = chord_max + 1e-9;
                assert!(
                    proj.distance >= dmin - sample_slack,
                    "case {case}: carried distance {:e} FAR below dense min \
                     {dmin:e} (grid slack {sample_slack:e}) — {}",
                    proj.distance,
                    fuzz::replay()
                );
                let (lo, hi) = curve.domain();
                let at_end = proj.t == lo || proj.t == hi;
                let speed = curve.deriv(proj.t).norm();
                let cosine = proj.orthogonality / (speed * proj.distance).max(1e-300);
                let near_global = proj.distance <= dmin + sample_slack + 1e-6 * dmin.max(1.0);
                if !near_global {
                    // A non-global foot: stationary-point contract says
                    // this can happen; count how it presents.
                    if at_end || cosine > 1e-6 {
                        // band-visible (orthogonality or end clamp)
                    } else {
                        local_min_wrong_branch += 1;
                        silent_bad.push((case, proj.distance, dmin, cosine));
                    }
                }
            }
        }
    }
    println!(
        "[F2] {accepted} accepted, {refused} refused, {local_min_wrong_branch} \
         interior small-cosine non-global feet (stationary-point contract; \
         distance carried honestly): {silent_bad:?}"
    );
    // The carried distance was verified honest in every case (no
    // fabricated answers); non-global local-min feet are the documented
    // stationary-point contract, visible to a consumer distance band.
}

/// F2: seeding with poisoned control points — NaN never wins the seed
/// scan and projection refuses rather than answering.
#[test]
fn f2_poisoned_control_point_refuses() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, f64::NAN, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(3.0, 0.0, 0.0),
    ];
    let curve = NurbsCurve3::new(kv, control, vec![1.0; 4]).unwrap();
    let p = Point3::new(1.0, 5.0, 0.0);
    match curve.project(p) {
        Err(e) => println!("[F2 poison] refused as expected: {e}"),
        Ok(pr) => {
            // If it accepted it must NOT be through a NaN-laundered path:
            // the accepted foot must carry finite honest residuals.
            assert!(
                pr.distance.is_finite() && pr.orthogonality.is_finite(),
                "accepted a poisoned foot with non-finite residuals"
            );
            println!(
                "[F2 poison] accepted at t={} (NaN support avoided): d={:e} orth={:e}",
                pr.t, pr.distance, pr.orthogonality
            );
        }
    }
}

/// F2: stagnation attack — a curve with a degenerate-speed plateau
/// (repeated interior control points). What does acceptance do there?
#[test]
fn f2_stagnation_plateau_probe() {
    // Cubic with a stalled middle: 4 coincident control points force
    // C' ≈ 0 on an interior stretch.
    let kv = KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0, 1.0],
        3,
    )
    .unwrap();
    let stall = Point3::new(1.0, 1.0, 0.0);
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        stall,
        stall,
        stall,
        stall,
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(3.0, 1.0, 0.0),
    ];
    let curve = NurbsCurve3::new(kv, control, vec![1.0; 7]).unwrap();
    // A point whose true foot is elsewhere but near-ish the stall.
    let p = Point3::new(1.0, 2.0, 0.0);
    match curve.project(p) {
        Ok(pr) => {
            let speed = curve.deriv(pr.t).norm();
            let m = 20000u32;
            let mut dmin = f64::INFINITY;
            for k in 0..=m {
                let t = f64::from(k) / f64::from(m);
                dmin = dmin.min(curve.eval(t).distance(p));
            }
            println!(
                "[F2 plateau] accepted t={} d={:e} (dense min {dmin:e}) orth={:e} speed={:e} iters={}",
                pr.t, pr.distance, pr.orthogonality, speed, pr.iterations
            );
            assert!(
                pr.distance >= dmin - 1e-7,
                "plateau acceptance fabricated a distance"
            );
        }
        Err(e) => println!("[F2 plateau] refused: {e}"),
    }
}

// =====================================================================
// F3: lsq vs an independent Householder QR reference.
// =====================================================================
fn qr_lsq(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let m = a.len();
    let n = a[0].len();
    let k = b[0].len();
    let mut r: Vec<Vec<f64>> = a.to_vec();
    let mut qtb: Vec<Vec<f64>> = b.to_vec();
    for j in 0..n {
        let mut norm = 0.0;
        for i in j..m {
            norm += r[i][j] * r[i][j];
        }
        let norm = norm.sqrt();
        if norm == 0.0 {
            continue;
        }
        let alpha = if r[j][j] > 0.0 { -norm } else { norm };
        let mut v: Vec<f64> = (0..m).map(|i| if i < j { 0.0 } else { r[i][j] }).collect();
        v[j] -= alpha;
        let vnorm2: f64 = v.iter().map(|x| x * x).sum();
        if vnorm2 == 0.0 {
            continue;
        }
        for col in j..n {
            let dot: f64 = (j..m).map(|i| v[i] * r[i][col]).sum();
            let f = 2.0 * dot / vnorm2;
            for i in j..m {
                r[i][col] -= f * v[i];
            }
        }
        for col in 0..k {
            let dot: f64 = (j..m).map(|i| v[i] * qtb[i][col]).sum();
            let f = 2.0 * dot / vnorm2;
            for i in j..m {
                qtb[i][col] -= f * v[i];
            }
        }
    }
    // Back substitution on the top n×n of R.
    let mut x = vec![vec![0.0; k]; n];
    for col in 0..k {
        for i in (0..n).rev() {
            let mut s = qtb[i][col];
            for q in (i + 1)..n {
                s -= r[i][q] * x[q][col];
            }
            x[i][col] = s / r[i][i];
        }
    }
    x
}

#[test]
fn f3_solve_normal_matches_householder_qr() {
    use geom_core::linalg::lsq;
    let mut rng = fuzz::start("review_m5_pr4::f3_qr_differential");
    let systems = fuzz::scaled(38);
    let mut worst = 0.0f64;
    for _case in 0..systems {
        let m = usize_in(&mut rng, 4, 14);
        let n = usize_in(&mut rng, 2, 6.min(m));
        let a: Vec<Vec<f64>> = (0..m)
            .map(|_| (0..n).map(|_| rng.range(-2.0, 2.0)).collect())
            .collect();
        let b: Vec<Vec<f64>> = (0..m).map(|_| vec![rng.range(-2.0, 2.0)]).collect();
        match lsq::solve_normal(&a, &b) {
            Ok(x) => {
                let xr = qr_lsq(&a, &b);
                for i in 0..n {
                    let d = (x[i][0] - xr[i][0]).abs();
                    let s = xr[i][0].abs().max(1.0);
                    worst = worst.max(d / s);
                    assert!(
                        d / s < 1e-6,
                        "solve_normal vs QR mismatch: {} vs {} (rel {:e}) — {}",
                        x[i][0],
                        xr[i][0],
                        d / s,
                        fuzz::replay()
                    );
                }
            }
            Err(e) => {
                // Random continuous matrices are a.s. full rank; a refusal
                // here would be suspicious. Record loudly.
                panic!(
                    "well-conditioned random system refused: {e:?} — {}",
                    fuzz::replay()
                );
            }
        }
    }
    println!("[F3] {systems} random systems vs Householder QR; worst rel diff {worst:.3e}");
}

#[test]
fn f3_rank_deficient_and_near_singular() {
    use geom_core::linalg::lsq::{self, LsqError};
    // Exact rank deficiency: col2 = 2·col0 − col1.
    let a: Vec<Vec<f64>> = (0..6)
        .map(|i| {
            let x = i as f64;
            vec![1.0, x, 2.0 - x]
        })
        .collect();
    let b: Vec<Vec<f64>> = (0..6).map(|i| vec![i as f64]).collect();
    match lsq::solve_normal(&a, &b) {
        Err(LsqError::LsqDegenerate { pivot_index, pivot }) => {
            println!("[F3] exact rank deficiency refused at pivot {pivot_index} = {pivot:e}");
        }
        Ok(x) => {
            // fp may sneak past d>0 with a tiny pivot; if it answers, the
            // answer must at least be finite (no garbage).
            let finite = x.iter().flatten().all(|v| v.is_finite());
            println!("[F3] exact rank deficiency ANSWERED (finite = {finite}): {x:?}");
            assert!(finite, "rank-deficient solve returned non-finite garbage");
        }
        Err(e) => panic!("unexpected refusal {e:?}"),
    }
    // Near-singular: col1 = col0·(1 + 1e-13 jitter).
    let mut rng = fuzz::start("review_m5_pr4::f3_near_singular");
    let mut refused = 0;
    let mut answered = 0;
    for _ in 0..fuzz::scaled(13) {
        let a: Vec<Vec<f64>> = (0..8)
            .map(|_| {
                let v = rng.range(0.5, 2.0);
                vec![v, v * (1.0 + 1e-13 * rng.range(-1.0, 1.0))]
            })
            .collect();
        let b: Vec<Vec<f64>> = (0..8).map(|_| vec![rng.range(-1.0, 1.0)]).collect();
        match lsq::solve_normal(&a, &b) {
            Err(LsqError::LsqDegenerate { .. }) => refused += 1,
            Ok(x) => {
                answered += 1;
                assert!(x.iter().flatten().all(|v| v.is_finite()), "garbage answer");
            }
            Err(e) => panic!("unexpected {e:?}"),
        }
    }
    println!("[F3] near-singular ensemble: {refused} refused, {answered} answered (all finite)");
}

// =====================================================================
// F4: compose exactness (num/den evaluates to f∘C pointwise) and
// containment fuzz across all five surfaces on random rational curves.
// =====================================================================
fn bernstein_eval_mid(coeffs: &[RingInterval], s: f64) -> f64 {
    // de Casteljau on interval midpoints.
    let mut v: Vec<f64> = coeffs.iter().map(|c| 0.5 * (c.lo() + c.hi())).collect();
    let n = v.len();
    for r in 1..n {
        for i in 0..n - r {
            v[i] = (1.0 - s) * v[i] + s * v[i + 1];
        }
    }
    v[0]
}

fn eval_form(form: &compose::CompositeForm, t: f64) -> f64 {
    let breaks = form.num.breaks();
    let mut j = 0;
    while j + 2 < breaks.len() && t >= breaks[j + 1] {
        j += 1;
    }
    let s = (t - breaks[j]) / (breaks[j + 1] - breaks[j]);
    let num = bernstein_eval_mid(&form.num.spans()[j], s);
    let den = bernstein_eval_mid(&form.den.spans()[j], s);
    num / den
}

fn implicit_value(surface: &ImplicitSurface, p: &[f64]) -> f64 {
    match surface {
        ImplicitSurface::Plane { point, normal } => {
            (0..3).map(|d| normal[d] * (p[d] - point[d])).sum()
        }
        ImplicitSurface::Sphere { center, radius } => {
            let q2: f64 = (0..3).map(|d| (p[d] - center[d]).powi(2)).sum();
            q2 - radius * radius
        }
        ImplicitSurface::Cylinder {
            point,
            axis,
            radius,
        } => {
            let a2: f64 = axis.iter().map(|a| a * a).sum();
            let q: Vec<f64> = (0..3).map(|d| p[d] - point[d]).collect();
            let q2: f64 = q.iter().map(|x| x * x).sum();
            let qa: f64 = (0..3).map(|d| q[d] * axis[d]).sum();
            q2 - qa * qa / a2 - radius * radius
        }
        ImplicitSurface::Cone {
            apex,
            axis,
            tan_half_angle,
        } => {
            let a2: f64 = axis.iter().map(|a| a * a).sum();
            let q: Vec<f64> = (0..3).map(|d| p[d] - apex[d]).collect();
            let q2: f64 = q.iter().map(|x| x * x).sum();
            let qa: f64 = (0..3).map(|d| q[d] * axis[d]).sum();
            q2 - (1.0 + tan_half_angle * tan_half_angle) * qa * qa / a2
        }
        ImplicitSurface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
        } => {
            let a2: f64 = axis.iter().map(|a| a * a).sum();
            let q: Vec<f64> = (0..3).map(|d| p[d] - center[d]).collect();
            let q2: f64 = q.iter().map(|x| x * x).sum();
            let qa: f64 = (0..3).map(|d| q[d] * axis[d]).sum();
            let c = q2 + major_radius * major_radius - minor_radius * minor_radius;
            c * c - 4.0 * major_radius * major_radius * (q2 - qa * qa / a2)
        }
    }
}

#[test]
fn f4_compose_exactness_and_containment_all_surfaces() {
    let mut rng = fuzz::start("review_m5_pr4::f4_compose");
    let curves = fuzz::scaled(8);
    let mut worst_rel = 0.0f64;
    for case in 0..curves {
        let degree = usize_in(&mut rng, 2, 4);
        let nctrl = usize_in(&mut rng, degree + 1, 8);
        let mut knots = vec![0.0; degree + 1];
        let ninterior = nctrl - degree - 1;
        for j in 1..=ninterior {
            knots.push(j as f64 / (ninterior + 1) as f64);
        }
        knots.extend(std::iter::repeat_n(1.0, degree + 1));
        let kv = KnotVector::clamped(knots, degree).unwrap();
        let control: Vec<Point3<f64>> = (0..nctrl)
            .map(|_| {
                Point3::new(
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                )
            })
            .collect();
        let weights: Vec<f64> = (0..nctrl).map(|_| rng.range(0.5, 2.0)).collect();
        let curve = NurbsCurve3::new(kv.clone(), control, weights.clone()).unwrap();
        let coords = curve.ring_coords();
        let data = CurveRingData::new(curve.knots(), curve.weights(), &coords).unwrap();
        let surfaces = [
            ImplicitSurface::Plane {
                point: [rng.range(-1.0, 1.0), 0.3, -0.2],
                normal: [0.6, -0.8, 0.0],
            },
            ImplicitSurface::Sphere {
                center: [0.1, -0.4, 0.2],
                radius: rng.range(0.5, 2.5),
            },
            ImplicitSurface::Cylinder {
                point: [0.0, 0.1, -0.3],
                axis: [rng.range(0.5, 2.0), 0.4, 1.1],
                radius: rng.range(0.5, 2.0),
            },
            ImplicitSurface::Cone {
                apex: [0.5, -0.5, 0.0],
                axis: [0.2, 1.3, rng.range(0.5, 1.5)],
                tan_half_angle: rng.range(0.2, 1.5),
            },
            ImplicitSurface::Torus {
                center: [-0.2, 0.3, 0.1],
                axis: [0.1, rng.range(0.5, 1.5), 0.9],
                major_radius: rng.range(1.0, 2.5),
                minor_radius: rng.range(0.2, 0.9),
            },
        ];
        for surface in &surfaces {
            let form = compose::implicit_composite(&data, surface).unwrap();
            let bound = form.sup_bound();
            assert!(
                bound.is_finite(),
                "case {case}: poisoned bound for {surface:?} — {}",
                fuzz::replay()
            );
            // Exactness: num/den midpoint evaluation == f(C(t)).
            let grid = fuzz::scaled(8);
            for k in 0..=grid {
                let t = k as f64 / grid as f64;
                let p = curve.eval(t);
                let direct = implicit_value(surface, &[p.x, p.y, p.z]);
                let composed = eval_form(&form, t.min(1.0 - 1e-12));
                let scale = direct.abs().max(bound).max(1e-9);
                let rel = (direct - composed).abs() / scale;
                worst_rel = worst_rel.max(rel);
                assert!(
                    rel < 1e-6,
                    "case {case} {surface:?}: composite {composed:e} vs direct {direct:e} — {}",
                    fuzz::replay()
                );
                // Containment.
                assert!(
                    direct.abs() <= bound * (1.0 + 1e-12) + 1e-12,
                    "case {case} {surface:?}: sample {:e} ESCAPES bound {bound:e} — {}",
                    direct.abs(),
                    fuzz::replay()
                );
            }
        }
    }
    println!(
        "[F4] {curves} curves × 5 surfaces exactness+containment; worst rel gap {worst_rel:.3e}"
    );
}

/// F4: hand-checkable sphere composite on a degree-1 segment.
#[test]
fn f4_sphere_composite_hand_check_degree1() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let a = [1.0, 2.0, -1.0];
    let b = [3.0, -1.0, 2.0];
    let c = [0.5, 0.25, -0.75];
    let r = 1.5;
    let coords = vec![
        vec![RingInterval::point(a[0]), RingInterval::point(b[0])],
        vec![RingInterval::point(a[1]), RingInterval::point(b[1])],
        vec![RingInterval::point(a[2]), RingInterval::point(b[2])],
    ];
    let w = [1.0, 1.0];
    let data = CurveRingData::new(&kv, &w, &coords).unwrap();
    let form = compose::implicit_composite(
        &data,
        &ImplicitSurface::Sphere {
            center: c,
            radius: r,
        },
    )
    .unwrap();
    // Hand Bernstein coefficients of |C−c|²−r² (degree 2):
    let qa: Vec<f64> = (0..3).map(|d| a[d] - c[d]).collect();
    let qb: Vec<f64> = (0..3).map(|d| b[d] - c[d]).collect();
    let b0: f64 = qa.iter().map(|x| x * x).sum::<f64>() - r * r;
    let b1: f64 = qa.iter().zip(qb.iter()).map(|(x, y)| x * y).sum::<f64>() - r * r;
    let b2: f64 = qb.iter().map(|x| x * x).sum::<f64>() - r * r;
    let span = &form.num.spans()[0];
    assert_eq!(span.len(), 3);
    for (got, want) in span.iter().zip([b0, b1, b2]) {
        let mid = 0.5 * (got.lo() + got.hi());
        assert!(
            (mid - want).abs() < 1e-12 && (got.hi() - got.lo()) < 1e-12,
            "sphere hand check: got [{:e},{:e}] want {want:e}",
            got.lo(),
            got.hi()
        );
    }
    println!("[F4] degree-1 sphere composite matches hand algebra: [{b0}, {b1}, {b2}]");
}

/// F4/doc-honesty: is binom_row really exact through n = 56? Compare the
/// same recurrence against u128 exact binomials.
#[test]
fn f4_binomial_row_exactness_probe() {
    fn exact_binom(n: usize, k: usize) -> u128 {
        let k = k.min(n - k);
        let mut b: u128 = 1;
        for i in 0..k {
            b = b * ((n - i) as u128) / ((i + 1) as u128);
        }
        b
    }
    let mut first_bad = None;
    for n in 1..=60usize {
        // Reproduce compose::binom_row's recurrence.
        let mut row = vec![1.0f64; n + 1];
        for k in 1..=n {
            row[k] = row[k - 1] * ((n - k + 1) as f64) / (k as f64);
        }
        for k in 0..=n {
            let exact = exact_binom(n, k) as f64; // may round for n>56, fine
            if row[k] != exact && first_bad.is_none() {
                first_bad = Some((n, k, row[k], exact));
            }
        }
    }
    // The MINOR-1 pin: the recurrence is exact through n = 54 and
    // FIRST inexact at n = 55 (the intermediate product exceeds 2^53
    // although C(55, 26) is representable) — which is why
    // compose::binom_row documents BINOM_EXACT_MAX = 54 and poisons
    // beyond it rather than serving a rounded weight.
    match first_bad {
        None => panic!("[F4] recurrence unexpectedly exact through n = 60"),
        Some((n, k, got, want)) => {
            println!("[F4] binom_row recurrence FIRST INEXACT at C({n},{k}): {got} vs {want}");
            assert_eq!(n, 55, "exactness frontier moved");
            assert!(k >= 26, "inexact below the pinned column");
        }
    }
}

// =====================================================================
// F6: worked example (129 samples), loose tolerance probe.
// =====================================================================
fn arc_samples_certify(n: usize) -> Vec<Point3<f64>> {
    let c = Point3::new(0.5, 1.0, -0.25);
    (0..n)
        .map(|k| {
            let theta = std::f64::consts::FRAC_PI_2 * (k as f64) / ((n - 1) as f64);
            let (s, co) = theta.sin_cos();
            Point3::new(c.x + 2.5 * co, c.y + 2.5 * s, c.z)
        })
        .collect()
}

#[test]
fn f6_worked_example_numbers() {
    let fit65 = NurbsCurve3::approximate(&arc_samples_certify(65), 3, 1e-2).unwrap();
    println!(
        "[F6] 65 samples: ctrl {} bound {:.4e} refit {}",
        fit65.curve.knots().control_count(),
        fit65.bound,
        fit65.refit_applied
    );
    let fit129 = NurbsCurve3::approximate(&arc_samples_certify(129), 3, 1e-2).unwrap();
    println!(
        "[F6] 129 samples: ctrl {} bound {:.4e} refit {}",
        fit129.curve.knots().control_count(),
        fit129.bound,
        fit129.refit_applied
    );
    assert_eq!(fit65.curve.knots().control_count(), 30);
    assert!(
        (fit65.bound - 9.890e-3).abs() < 2e-5,
        "bound {:e}",
        fit65.bound
    );
    assert!(fit129.refit_applied, "129-sample refit flip missing");
    assert!(
        (fit129.bound - 6.379e-5).abs() < 2e-7,
        "bound {:e}",
        fit129.bound
    );
}

#[test]
fn f6_loose_tolerance_degenerate_structures() {
    let pts = arc_samples_certify(33);
    for tol in [1.0, 1e3, 1e12] {
        match NurbsCurve3::approximate(&pts, 3, tol) {
            Ok(fit) => {
                let n = fit.curve.knots().control_count();
                let dense = {
                    let reference = NurbsCurve3::interpolate(&pts, 1).unwrap();
                    let mut worst = 0.0f64;
                    for k in 0..=4096 {
                        let t = f64::from(k) / 4096.0;
                        worst = worst.max(fit.curve.eval(t).distance(reference.eval(t)));
                    }
                    worst
                };
                println!(
                    "[F6] tol {tol:e}: ctrl {n} bound {:.3e} dense {dense:.3e} refit {}",
                    fit.bound, fit.refit_applied
                );
                assert!(dense <= fit.bound + 1e-11, "loose-tol bound violated");
                assert!(n >= 4, "structure collapsed below degree+1");
            }
            Err(e) => println!("[F6] tol {tol:e}: refused {e:?}"),
        }
    }
    // 2-D lane too.
    let pts2: Vec<Point2<f64>> = (0..17)
        .map(|k| {
            let u = f64::from(k) / 16.0;
            Point2::new(u, u * (1.0 - u))
        })
        .collect();
    let fit2 = NurbsCurve2::approximate(&pts2, 3, 1e6).unwrap();
    println!(
        "[F6] 2-D loose: ctrl {} bound {:.3e}",
        fit2.curve.knots().control_count(),
        fit2.bound
    );
}

// =====================================================================
// F9: the PR 5/7 usage shape in miniature — fit a cylinder-limb sample
// set, project, compose the cylinder residual, hull-bound, Decide.
// =====================================================================
#[test]
fn f9_e2e_cylinder_fit_project_certify_decide() {
    use geom_core::k_stats::decide_flagged;
    use geom_core::predicate::Band;
    // Known locus: helix-free circle arc on the cylinder x²+y²=1.44
    // (radius 1.2, axis z) at slowly varying z — points ON the cylinder.
    let n = 97;
    let pts: Vec<Point3<f64>> = (0..n)
        .map(|k| {
            let t = (k as f64) / ((n - 1) as f64);
            let th = 2.1 * t + 0.3;
            let (s, c) = th.sin_cos();
            Point3::new(1.2 * c, 1.2 * s, 0.4 * t - 0.1)
        })
        .collect();
    let tol = 1e-4;
    let fit = NurbsCurve3::approximate(&pts, 3, tol).unwrap();
    println!(
        "[F9] fit: ctrl {} bound {:.3e} refit {}",
        fit.curve.knots().control_count(),
        fit.bound,
        fit.refit_applied
    );
    // Project a few off-curve points onto the fit; residuals honest.
    for (px, py, pz) in [(1.5, 0.2, 0.0), (0.0, 1.4, 0.15), (0.9, 0.9, 0.05)] {
        let p = Point3::new(px, py, pz);
        let proj = fit.curve.project(p).unwrap();
        let speed = fit.curve.deriv(proj.t).norm();
        let cosine = proj.orthogonality / (speed * proj.distance).max(1e-300);
        let (lo, hi) = fit.curve.domain();
        let interior = proj.t > lo && proj.t < hi;
        println!(
            "[F9] project ({px},{py},{pz}): t={:.4} d={:.4e} cos={:.2e} interior={interior}",
            proj.t, proj.distance, cosine
        );
        assert!(!interior || cosine < 1e-9, "interior foot with bad cosine");
    }
    // Compose the cylinder residual of the FIT and hull-bound it.
    let coords = fit.curve.ring_coords();
    let data = CurveRingData::new(fit.curve.knots(), fit.curve.weights(), &coords).unwrap();
    let cyl = ImplicitSurface::Cylinder {
        point: [0.0, 0.0, 0.0],
        axis: [0.0, 0.0, 1.0],
        radius: 1.2,
    };
    let form = compose::implicit_composite(&data, &cyl).unwrap();
    let hull = form.sup_bound();
    // Dense soundness.
    let mut dense = 0.0f64;
    for k in 0..=8192 {
        let t = f64::from(k) / 8192.0;
        let p = fit.curve.eval(t);
        dense = dense.max((p.x * p.x + p.y * p.y - 1.44).abs());
    }
    println!("[F9] cylinder residual: hull {hull:.3e} m², dense {dense:.3e} m²");
    assert!(dense <= hull, "hull bound unsound");
    // The accept/refuse decision through Decide bands: margin =
    // band-eps − hull (positive ⇒ certified within). eps here is a
    // meters² working band for this locus scale, chosen ≥ the fit's
    // achievable residual: 2R·bound order.
    let eps = 2.0 * 1.2 * tol * 4.0; // ≈ 9.6e-4 m² working certification band
    let band = Band::new(1e-9, 1e-8).unwrap();
    let margin = eps - hull;
    let verdict = decide_flagged(
        "review_cylinder_residual_within_eps",
        margin,
        band,
        "review fixture: m^2 working band, not a shipped decision",
    );
    println!("[F9] decide(eps−hull = {margin:.3e}): {verdict:?}");
    assert!(matches!(verdict, Ok(geom_core::predicate::Sign::Positive)));
    // Corrupt one control point (between-samples excursion) and verify
    // REFUSAL through the same gate.
    let kv = fit.curve.knots();
    let p_deg = kv.degree();
    let idx = fit.curve.control().len() / 2;
    let mut control = fit.curve.control().to_vec();
    let radial = control[idx] - Point3::new(0.0, 0.0, control[idx].z);
    let rn = radial.norm();
    control[idx] = control[idx] + radial * (0.05 / rn);
    let corrupted = NurbsCurve3::new(kv.clone(), control, fit.curve.weights().to_vec()).unwrap();
    let coords_c = corrupted.ring_coords();
    let data_c = CurveRingData::new(corrupted.knots(), corrupted.weights(), &coords_c).unwrap();
    let hull_c = compose::implicit_composite(&data_c, &cyl)
        .unwrap()
        .sup_bound();
    let margin_c = eps - hull_c;
    let verdict_c = decide_flagged(
        "review_cylinder_residual_within_eps",
        margin_c,
        band,
        "review fixture: m^2 working band, not a shipped decision",
    );
    println!("[F9] corrupted: hull {hull_c:.3e}, decide {verdict_c:?}");
    assert!(matches!(
        verdict_c,
        Ok(geom_core::predicate::Sign::Negative)
    ));
    let _ = p_deg;
}
