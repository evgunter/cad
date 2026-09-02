//! R1 review probe (MESH-5, PR 1507) — an INDEPENDENT attack on the
//! premise under PR 1507's claim 1: *"the deviation is entirely the rim
//! chord's azimuthal sagitta; the rows subdivide the ruling direction,
//! where the deviation is identically zero"*.
//!
//! This file deliberately touches no kernel code. It re-parametrises a
//! cone from scratch (apex at the origin, axis `+z`, `P(u, v) = (v sinα
//! cos u, v sinα sin u, v cosα)` with `v` the slant distance from the
//! apex), builds the three candidate patch triangulations by hand, and
//! measures each one's densely sampled deviation against the exact
//! cone locus. Nothing here reads `mesh`, so a bug shared between the
//! kernel's sizing and the kernel's own probe cannot hide in it.
//!
//! The three readings, all over the SAME patch `u ∈ [0, U]`,
//! `v ∈ [v0, v1]`:
//!
//! * `strip` — the decided answer: no interior grid points at all.
//! * `rows` — the v-schedule honoured LITERALLY (issue 685's own
//!   wording: "emit the rows"), i.e. `k` interior v-rows whose
//!   endpoints sit on the two meridian sides. This is the
//!   counterfactual PR 1507 could NOT build in the kernel (the grid
//!   loop needs `nu >= 2` before any interior point exists), so it is
//!   only measurable here.
//! * `grid` — the counterfactual the PR actually measured: `m`
//!   interior columns AND `k` interior rows.
//!
//! The rim (`v = v1`) keeps ONE azimuthal chord in every reading,
//! because in the kernel the rim is boundary geometry sized by
//! `chords.rs`, not by `grid_counts` — which is exactly why the PR's
//! `max_dev` column is constant.
//!
//! Red-capable, not a reporter: the assertions below fail if v-rows
//! ever buy deviation on any swept cone shape, or if the strip's
//! deviation ever exceeds `delta_s`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

type P3 = [f64; 3];

fn sub(a: P3, b: P3) -> P3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn norm(a: P3) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}
fn lerp3(a: P3, b: P3, c: P3, la: f64, lb: f64, lc: f64) -> P3 {
    [
        a[0] * la + b[0] * lb + c[0] * lc,
        a[1] * la + b[1] * lb + c[1] * lc,
        a[2] * la + b[2] * lb + c[2] * lc,
    ]
}

/// Cone point: apex at origin, axis `+z`, `v` = slant distance.
fn cone_p(alpha: f64, u: f64, v: f64) -> P3 {
    let (s, c) = alpha.sin_cos();
    [v * s * u.cos(), v * s * u.sin(), v * c]
}

/// Exact distance from `p` to the single-nappe cone locus (`v >= 0`).
fn dist_cone(alpha: f64, p: P3) -> f64 {
    let (s, c) = alpha.sin_cos();
    let h = p[2];
    let rho = (p[0] * p[0] + p[1] * p[1]).sqrt();
    if rho * s + h * c >= 0.0 {
        (rho * c - h * s).abs()
    } else {
        norm(sub(p, [0.0, 0.0, 0.0]))
    }
}

/// Densely sampled deviation of one triangle (45 barycentric samples,
/// `n = 8` per edge — the same density PR 1507's own instrument uses).
fn tri_dev(alpha: f64, t: [P3; 3]) -> f64 {
    let n = 8u32;
    let mut worst = 0.0_f64;
    for i in 0..=n {
        for j in 0..=(n - i) {
            let k = n - i - j;
            let p = lerp3(
                t[0],
                t[1],
                t[2],
                f64::from(i) / f64::from(n),
                f64::from(j) / f64::from(n),
                f64::from(k) / f64::from(n),
            );
            worst = worst.max(dist_cone(alpha, p));
        }
    }
    worst
}

/// Triangulate the patch `u ∈ [0, U] × v ∈ [v0, v1]` with `m` azimuth
/// columns and `k` v-rows, and report `(triangles, max deviation)`.
///
/// `m = k = 1` is the strip; `m = 1, k > 1` is "rows honoured
/// literally"; `m > 1, k > 1` is the PR's measured counterfactual. A
/// degenerate `v0 == 0` row (apex) collapses to a fan, matching the
/// kernel's pole-degenerate drop.
fn patch(alpha: f64, uspan: f64, v0: f64, v1: f64, m: usize, k: usize) -> (usize, f64) {
    let mut tris = 0usize;
    let mut worst = 0.0_f64;
    for jj in 0..k {
        #[allow(clippy::cast_precision_loss)]
        let (va, vb) = (
            v0 + (v1 - v0) * (jj as f64 / k as f64),
            v0 + (v1 - v0) * ((jj + 1) as f64 / k as f64),
        );
        for ii in 0..m {
            #[allow(clippy::cast_precision_loss)]
            let (ua, ub) = (
                uspan * (ii as f64 / m as f64),
                uspan * ((ii + 1) as f64 / m as f64),
            );
            let (a, b, c, d) = (
                cone_p(alpha, ua, va),
                cone_p(alpha, ub, va),
                cone_p(alpha, ub, vb),
                cone_p(alpha, ua, vb),
            );
            // Apex row: the two v = va vertices coincide, so the quad
            // is one triangle.
            if va == 0.0 {
                tris += 1;
                worst = worst.max(tri_dev(alpha, [a, c, d]));
            } else {
                tris += 2;
                worst = worst.max(tri_dev(alpha, [a, b, c]));
                worst = worst.max(tri_dev(alpha, [a, c, d]));
            }
        }
    }
    (tris, worst)
}

/// The kernel's cone sizing, restated from `mesh::curved::grid_counts`
/// plus `mesh::sizing` (both private) so this file can say which shapes
/// are in the `nu == 1` regime at all. If the kernel's spelling moves,
/// this restatement is what goes stale — it is a REVIEW instrument and
/// the numbers it gates are re-derived, not imported.
fn nu_raw(delta: f64, rho_max: f64, uspan: f64) -> (usize, f64) {
    let ds = delta * 0.5;
    let cap = core::f64::consts::FRAC_PI_4; // mesh::sizing::MAX_ANGULAR_STEP
    let hu = if ds < rho_max {
        let h = 2.0 * (1.0 - ds / rho_max).acos();
        if h < cap { h } else { cap }
    } else {
        cap
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = (uspan / hu).ceil().max(1.0) as usize;
    (n, hu)
}

/// **Claim 1's premise, attacked over a shape sweep.** For every cone
/// shape that lands in the `nu == 1` regime — near-degenerate to
/// near-flat half-angles, tall and squat, apex-anchored and frustum —
/// honouring the v-schedule literally (rows, no extra column) must not
/// reduce the densely sampled deviation, and the strip's deviation
/// must stay inside `delta_s`.
#[test]
fn v_rows_never_buy_deviation_on_a_cone() {
    println!(
        "{:>10} {:>8} {:>7} {:>7} {:>5} {:>6} {:>11} {:>11} {:>11} {:>9}",
        "alpha", "U", "v0", "v1", "nu", "delta", "strip_dev", "rows_dev", "grid_dev", "delta_s"
    );
    let mut checked = 0usize;
    for &alpha in &[
        1e-7,
        1e-3,
        0.05,
        core::f64::consts::FRAC_PI_8,
        core::f64::consts::FRAC_PI_6,
        core::f64::consts::FRAC_PI_4,
        core::f64::consts::FRAC_PI_3,
        1.45,
        core::f64::consts::FRAC_PI_2 - 1e-6,
    ] {
        // tall (v1/v0 large / apex), squat, frustum.
        for &(v0, v1) in &[
            (0.0, 1.0),
            (0.0, 1000.0),
            (0.999, 1.0),
            (0.5, 1.0),
            (1.0, 1e6),
            (1e-6, 1.0),
        ] {
            for &uspan in &[1e-3, 0.1, core::f64::consts::FRAC_PI_6, 0.7, 1.5] {
                for &delta in &[0.25, 0.1, 0.05, 0.01, 1e-4] {
                    let rho_max = v1 * alpha.sin();
                    if rho_max == 0.0 || !rho_max.is_finite() {
                        continue;
                    }
                    let (nu, _hu) = nu_raw(delta, rho_max, uspan);
                    if nu != 1 {
                        continue; // only the decided regime
                    }
                    let ds = delta * 0.5;
                    let (_, strip) = patch(alpha, uspan, v0, v1, 1, 1);
                    let (_, rows) = patch(alpha, uspan, v0, v1, 1, 8);
                    let (_, grid) = patch(alpha, uspan, v0, v1, 3, 8);
                    checked += 1;
                    if checked <= 24 {
                        println!(
                            "{alpha:>10.3e} {uspan:>8.3} {v0:>7.1e} {v1:>7.1e} {nu:>5} \
                             {delta:>6} {strip:>11.4e} {rows:>11.4e} {grid:>11.4e} {ds:>9.3e}"
                        );
                    }
                    // (a) rows never buy deviation (allow 1e-12 slack
                    // for the sampling grid's own asymmetry).
                    assert!(
                        rows <= strip * (1.0 + 1e-9) + 1e-15,
                        "v-rows REDUCED deviation: alpha={alpha:e} U={uspan} v=[{v0:e},{v1:e}] \
                         delta={delta}: strip={strip:e} rows={rows:e}"
                    );
                    // (b) nor do interior columns, once the rim chord
                    // is boundary geometry the grid cannot touch.
                    assert!(
                        grid <= strip * (1.0 + 1e-9) + 1e-15,
                        "interior grid REDUCED deviation: alpha={alpha:e} U={uspan} \
                         v=[{v0:e},{v1:e}] delta={delta}: strip={strip:e} grid={grid:e}"
                    );
                    // (c) the decided answer certifies at delta_s.
                    assert!(
                        strip <= ds * (1.0 + 1e-9),
                        "single strip exceeds delta_s: alpha={alpha:e} U={uspan} \
                         v=[{v0:e},{v1:e}] delta={delta}: strip={strip:e} > {ds:e}"
                    );
                }
            }
        }
    }
    println!("checked {checked} nu == 1 shapes");
    assert!(checked > 100, "sweep did not reach the nu == 1 regime");
}

/// **The measured constant, re-derived.** PR 1507's `max_dev` column is
/// `2.409e-2` at every `nu == 1` δ on the π/6 wedge. Closed form: the
/// rim chord's in-plane sagitta `ρ_max·(1 − cos(U/2))` projected onto
/// the cone normal, i.e. times `cos α`. If that identity holds, the
/// column is a property of the RIM (boundary geometry, sized by
/// `chords.rs`) and is structurally blind to `grid_counts` — which is
/// the honest reading of "identical at every δ".
#[test]
fn the_measured_constant_is_the_rim_chord_sagitta() {
    let alpha = core::f64::consts::FRAC_PI_4;
    let uspan = core::f64::consts::FRAC_PI_6;
    let (_, strip) = patch(alpha, uspan, 0.0, 2.0_f64.sqrt(), 1, 1);
    let closed = 1.0 * (1.0 - (uspan * 0.5).cos()) * alpha.cos();
    println!("strip dev = {strip:.6e}; closed form = {closed:.6e}; PR column = 2.409e-2");
    assert!(
        (strip - closed).abs() <= 1e-12,
        "sampled {strip:e} vs closed form {closed:e}"
    );
    assert!(
        (strip - 2.409e-2).abs() < 5e-6,
        "does not reproduce the PR's 2.409e-2: {strip:e}"
    );
}
