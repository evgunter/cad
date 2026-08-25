//! Independent review probes for VERBS-OFF-B (reviewer lane r2).
//!
//! Four families, each RED-able:
//!
//! - **P1 — the rationalization inequality itself**, brute-forced on
//!   random configurations independently of the kernel: does
//!   `‖E − d·n‖ ≤ |‖E‖ − |d|| + τ + τ²/‖E‖` hold for every `E, m, d`
//!   with `sign(E·n) = sign(d)`, and does it FAIL without that side
//!   condition (which is what `D`'s sign witness has to exclude)?
//! - **P2 — the certified bound against dense sampling** on bases the
//!   shipped suite does not use: a skinned loft, a rational torus
//!   patch, and both `d` signs.
//! - **P3 — extreme `d`**: just inside the certified collapse reach.
//! - **P4 — `certify_offset` and the fitted surface's weights.** The
//!   composite reads the fit's control net as a POLYNOMIAL
//!   (`channel(fit, c, false)`), i.e. it ignores `fit.weights()`
//!   entirely, while limb 1 evaluates the fit through `eval` and DOES
//!   respect them. `certify_offset` is a public door that takes the
//!   fit as an argument, so this row asks what the certificate says
//!   about a rational fit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use geom::NurbsSurface;
use geom::curves::fit::interpolate_columns;
use geom_brep::offset_fit::{OffsetFitError, certify_offset, fit_offset, offset_point};
use geom_brep::offset_meters::{OFFSET_METER_LADDER, patch_collapse};
use geom_brep::patch_bound::patch_cells_refined;
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn kv2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}

fn kv1() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}

/// A deterministic dense schedule, coprime counts, off the fit's grid.
fn dense(nu: usize, nv: usize) -> Vec<(f64, f64)> {
    let mut out = Vec::with_capacity(nu * nv);
    for i in 0..nu {
        for j in 0..nv {
            #[allow(clippy::cast_precision_loss)]
            out.push((i as f64 / (nu - 1) as f64, j as f64 / (nv - 1) as f64));
        }
    }
    out
}

// ---------------------------------------------------------------------
// P1 — the two-limb inequality, brute-forced
// ---------------------------------------------------------------------

/// A tiny deterministic LCG — no dependency, reproducible.
struct Lcg(u64);
impl Lcg {
    fn next_f64(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        #[allow(clippy::cast_precision_loss)]
        let x = (self.0 >> 11) as f64;
        x / ((1u64 << 53) as f64)
    }
    fn sym(&mut self) -> f64 {
        2.0 * self.next_f64() - 1.0
    }
}

fn v_norm(a: [f64; 3]) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}
fn v_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn v_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// `‖E − d·n‖ ≤ |‖E‖ − |d|| + τ + τ²/‖E‖` with `τ = ‖E × m‖/‖m‖`,
/// **under the side condition `sign(E·n) = sign(d)`** — the exact
/// inequality `offset_fit`'s limb 2 rides, re-derived here and
/// brute-forced. Goes red if the inequality is false anywhere, which
/// is the only thing the whole hull-side limb rests on.
#[test]
fn p1_the_rationalized_residual_inequality_holds_under_the_sign_condition() {
    let mut rng = Lcg(0x0FF5E7_B_u64);
    let mut checked = 0usize;
    let mut worst_slack = f64::INFINITY;
    for _ in 0..400_000 {
        let m = [rng.sym(), rng.sym(), rng.sym()];
        let mn = v_norm(m);
        if !(mn > 1e-6) {
            continue;
        }
        let n = [m[0] / mn, m[1] / mn, m[2] / mn];
        // Sample E over a wide dynamic range so the near-tangential
        // (`τ ≈ ‖E‖`) and near-collinear regimes are both hit.
        let scale = 10f64.powf(4.0 * rng.sym());
        let e = [rng.sym() * scale, rng.sym() * scale, rng.sym() * scale];
        let en = v_norm(e);
        if !(en > 0.0) || !en.is_finite() {
            continue;
        }
        let d = rng.sym() * scale;
        if d == 0.0 {
            continue;
        }
        let edotn = v_dot(e, n);
        // The side condition the composite proves through `sign(D)`.
        if (edotn > 0.0) != (d > 0.0) {
            continue;
        }
        let r = [e[0] - d * n[0], e[1] - d * n[1], e[2] - d * n[2]];
        let lhs = v_norm(r);
        let tau = v_norm(v_cross(e, m)) / mn;
        let rhs = (en - d.abs()).abs() + tau + tau * tau / en;
        assert!(
            lhs <= rhs * (1.0 + 1e-9),
            "the residual inequality FAILS: ‖R‖ = {lhs} > {rhs} \
             (‖E‖ = {en}, d = {d}, τ = {tau}, E·n = {edotn})"
        );
        worst_slack = worst_slack.min(rhs / lhs.max(f64::MIN_POSITIVE));
        checked += 1;
    }
    assert!(checked > 50_000, "only {checked} configurations exercised");
    eprintln!("P1: {checked} configurations, tightest rhs/lhs = {worst_slack:.6}");
}

/// The other half of P1: WITHOUT the sign condition the inequality is
/// false, so `D`'s sign witness is load-bearing, not decoration. Goes
/// red if a counterexample can no longer be found — which would mean
/// the side condition is unnecessary and the `+∞` arm is dead code.
#[test]
fn p1b_without_the_sign_condition_the_inequality_is_false() {
    // E anti-parallel to n, ‖E‖ = |d|: LHS = 2|d|, RHS = 0.
    let m = [0.0, 0.0, 1.0];
    let d = 0.4_f64;
    let e = [0.0, 0.0, -d];
    let n = [0.0, 0.0, 1.0];
    let r = [e[0] - d * n[0], e[1] - d * n[1], e[2] - d * n[2]];
    let lhs = v_norm(r);
    let en = v_norm(e);
    let tau = v_norm(v_cross(e, m)) / v_norm(m);
    let rhs = (en - d.abs()).abs() + tau + tau * tau / en;
    assert!(
        lhs > rhs,
        "the sign condition looks unnecessary: ‖R‖ = {lhs} ≤ {rhs}"
    );
    eprintln!("P1b: sign condition is load-bearing — ‖R‖ = {lhs} vs bound {rhs}");
}

// ---------------------------------------------------------------------
// P2 — bases the shipped suite does not use
// ---------------------------------------------------------------------

/// A skinned loft: four cubic section curves at increasing `z`, each a
/// different planar shape, interpolated in `v` through the loft door —
/// the "genuinely non-analytic base from the existing machinery" the
/// spec's acceptance asks for, built section-wise rather than as a
/// height field.
fn skinned_loft() -> NurbsSurface<f64> {
    let nu = 7usize;
    let nv = 5usize;
    #[allow(clippy::cast_precision_loss)]
    let uparams: Vec<f64> = (0..nu).map(|i| i as f64 / (nu - 1) as f64).collect();
    #[allow(clippy::cast_precision_loss)]
    let vparams: Vec<f64> = (0..nv).map(|j| j as f64 / (nv - 1) as f64).collect();
    // Section ℓ: a smooth planar curve whose amplitude and phase both
    // vary with ℓ — no analytic kind reproduces it.
    let section = |u: f64, t: f64| -> Point3<f64> {
        let amp = 0.6 + 0.35 * t;
        let ph = 0.8 * t;
        Point3::new(
            u,
            amp * (1.7 * u + ph).sin(),
            0.9 * t + 0.15 * (2.3 * u).cos() * t,
        )
    };
    let rows_u: Vec<Vec<f64>> = uparams
        .iter()
        .map(|u| {
            let mut row = Vec::with_capacity(nv * 3);
            for t in &vparams {
                let p = section(*u, *t);
                row.extend_from_slice(&[p.x, p.y, p.z]);
            }
            row
        })
        .collect();
    let (ku, r) = interpolate_columns(&uparams, 3, &rows_u).unwrap();
    let cu = ku.control_count();
    let mut rows_v: Vec<Vec<f64>> = Vec::with_capacity(nv);
    for l in 0..nv {
        let mut row = Vec::with_capacity(cu * 3);
        for rr in &r {
            row.extend_from_slice(&rr[l * 3..l * 3 + 3]);
        }
        rows_v.push(row);
    }
    let (kv, p) = interpolate_columns(&vparams, 3, &rows_v).unwrap();
    let cv = kv.control_count();
    let mut control = Vec::with_capacity(cu * cv);
    for i in 0..cu {
        for row in p.iter().take(cv) {
            control.push(Point3::new(row[i * 3], row[i * 3 + 1], row[i * 3 + 2]));
        }
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; cu * cv]).unwrap()
}

/// A rational torus patch: a quarter minor arc revolved a quarter turn
/// (A8.1's weight product) — a rational base with a genuinely
/// two-directional weight net, unlike the cylinder's one-directional
/// one.
fn torus_patch(major: f64, minor: f64) -> NurbsSurface<f64> {
    let w = (FRAC_PI_2 * 0.5).cos();
    // Minor quarter arc in the (r, z) half-plane, from (major+minor, 0)
    // to (major, minor), tangent-intersection point at
    // (major+minor, minor).
    let meridian = [
        (major + minor, 0.0, 1.0),
        (major + minor, minor, w),
        (major, minor, 1.0),
    ];
    let mut control = Vec::with_capacity(9);
    let mut weights = Vec::with_capacity(9);
    for iu in 0..3 {
        for (rr, z, wm) in meridian {
            control.push(match iu {
                0 => Point3::new(rr, 0.0, z),
                1 => Point3::new(rr, rr, z),
                _ => Point3::new(0.0, rr, z),
            });
            weights.push(if iu == 1 { wm * w } else { wm });
        }
    }
    NurbsSurface::new(kv2(), kv2(), control, weights).unwrap()
}

fn contains_dense_sample(name: &str, base: &NurbsSurface<f64>, d: f64, tol: f64) {
    let (fit, cert) = fit_offset(base, d, tol, band())
        .unwrap_or_else(|e| panic!("{name}: fit_offset refused at d = {d}: {e}"));
    assert!(
        cert.hull_sup <= tol,
        "{name}: certified sup {} exceeds {tol}",
        cert.hull_sup
    );
    let mut worst = 0.0f64;
    for (u, v) in dense(41, 37) {
        let target = offset_point(base, d, u, v)
            .unwrap_or_else(|| panic!("{name}: the exact offset is undefined at ({u}, {v})"));
        worst = worst.max((fit.eval(u, v) - target).norm());
    }
    // The red direction: a bound that UNDER-reports.
    assert!(
        worst <= cert.hull_sup,
        "{name}: d = {d}: the certified sup {} UNDER-reports the sampled max {worst}",
        cert.hull_sup
    );
    assert!(
        cert.on_locus_max <= cert.hull_sup,
        "{name}: limb 1 ({}) is above limb 2 ({})",
        cert.on_locus_max,
        cert.hull_sup
    );
    eprintln!(
        "{name} d={d}: cells={} rounds={} on_locus={:.3e} hull_sup={:.3e} sampled={worst:.3e} \
         floor={:.4} reach={} ratio={:.2}",
        cert.cells,
        cert.rounds,
        cert.on_locus_max,
        cert.hull_sup,
        cert.normal_floor,
        cert.curvature_reach,
        cert.hull_sup / worst.max(f64::MIN_POSITIVE)
    );
}

#[test]
fn p2_skinned_loft_certificate_contains_a_dense_sample_both_signs() {
    let base = skinned_loft();
    contains_dense_sample("skinned-loft", &base, 0.08, 1e-3);
    contains_dense_sample("skinned-loft", &base, -0.08, 1e-3);
}

#[test]
fn p2_rational_torus_patch_certificate_contains_a_dense_sample_both_signs() {
    let base = torus_patch(2.0, 0.6);
    contains_dense_sample("torus", &base, 0.15, 3e-3);
    contains_dense_sample("torus", &base, -0.15, 3e-3);
}

// ---------------------------------------------------------------------
// P3 — extreme d, just inside the certified collapse reach
// ---------------------------------------------------------------------

#[test]
fn p3_offset_just_inside_the_certified_reach_still_bounds_the_sample() {
    let base = torus_patch(2.0, 0.6);
    let cells = patch_cells_refined(&base, OFFSET_METER_LADDER[1]).unwrap();
    let coll = patch_collapse(&cells, -1.0);
    assert!(coll.reach.is_finite() && coll.reach > 0.0);
    // 90% of the certified reach: the door must either certify with a
    // bound that still contains a dense sample, or refuse LOUD.
    let d = -0.9 * coll.reach;
    match fit_offset(&base, d, 1e-2, band()) {
        Ok((fit, cert)) => {
            let mut worst = 0.0f64;
            for (u, v) in dense(41, 37) {
                let target = offset_point(&base, d, u, v).unwrap();
                worst = worst.max((fit.eval(u, v) - target).norm());
            }
            assert!(
                worst <= cert.hull_sup,
                "extreme d = {d}: certified sup {} UNDER-reports {worst}",
                cert.hull_sup
            );
            eprintln!(
                "P3 d={d:.4} (reach {:.4}): hull_sup={:.3e} sampled={worst:.3e}",
                coll.reach, cert.hull_sup
            );
        }
        Err(e) => eprintln!("P3 d={d:.4} (reach {:.4}) refused loud: {e}", coll.reach),
    }
}

/// **P6b — the same lever, permissive direction.** A planar patch
/// whose two parameter directions differ by `1e-8` radians: the chart
/// is as ill-conditioned as a chart gets while still having a nonzero
/// normal, and `sine_floor ≈ 1e-8`. The margin is `sine_floor · |d|`,
/// so the SAME patch passes the regularity door at a large `|d|` and
/// fails it at a small one — the opposite of the physics the meter's
/// own docs cite (`|d|·θ` is the displacement a normal ambiguity
/// inflicts, so a large `|d|` is the dangerous case). Goes RED if the
/// door's verdict on this fixed geometry is not `d`-dependent.
#[test]
fn p6b_a_near_degenerate_chart_passes_the_floor_at_large_d_and_fails_at_small_d() {
    // Bilinear plane with S_u = (1,0,0), S_v = (1,1e-8,0).
    let skew = 1e-8;
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, skew, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(2.0, skew, 0.0),
    ];
    let base = NurbsSurface::new(kv1(), kv1(), control, vec![1.0; 4]).unwrap();
    let cells = patch_cells_refined(&base, OFFSET_METER_LADDER[0]).unwrap();
    let reg = geom_brep::offset_meters::patch_regularity(&cells);
    eprintln!(
        "P6b: sine_floor = {:.3e}, floor = {:.3e}",
        reg.sine_floor, reg.floor
    );
    let verdict = |d: f64| {
        !matches!(
            fit_offset(&base, d, 1e-3, band()),
            Err(OffsetFitError::Meter(
                geom_brep::offset_meters::MeterError::NormalFloor { .. }
                    | geom_brep::offset_meters::MeterError::Escalated { .. }
            ))
        )
    };
    let (big, small) = (verdict(1.0), verdict(1e-3));
    eprintln!("P6b: |d| = 1 m passes the floor: {big}; |d| = 1e-3 m passes: {small}");
    assert_eq!(
        big, small,
        "P6b: the regularity door's verdict on ONE fixed geometry \
         (sine_floor {:.3e}) depends on |d| — it passes at |d| = 1 m ({big}) and \
         not at |d| = 1e-3 m ({small}); the more permissive answer is the LARGER \
         offset, which is the more dangerous one",
        reg.sine_floor
    );
}

// ---------------------------------------------------------------------
// P4 — certify_offset and the fitted surface's weights
// ---------------------------------------------------------------------

/// `certify_offset` is a public door taking the fit as an argument.
/// Limb 2's composite reads the fit's control net as a POLYNOMIAL and
/// never looks at `fit.weights()`; limb 1 evaluates the fit and does.
///
/// This row hands the door a fit whose weights are non-unit and asks
/// what comes back. It goes RED if a certificate is issued whose
/// `hull_sup` is below the residual an independent dense sample
/// measures on the surface that was actually handed in — i.e. if the
/// certifying limb certified a different surface.
#[test]
fn p4_certify_offset_on_a_rational_fit() {
    let base = quarter_cylinder(1.0, 1.0);
    let d = 0.25;
    let (fit, _) = fit_offset(&base, d, 1e-3, band()).unwrap();
    let (cu, cv) = fit.control_counts();
    // Same control points and knots; weights perturbed. In ℝ this is a
    // DIFFERENT surface — but the composite's `Ẽ = F·w − A` reads only
    // the control points, so limb 2's net is the unperturbed fit's.
    let mut weights = vec![1.0; cu * cv];
    for (i, w) in weights.iter_mut().enumerate() {
        *w = if i % 2 == 0 { 1.0 } else { 1.6 };
    }
    let rational_fit = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        fit.control().to_vec(),
        weights,
    )
    .unwrap();
    // What the handed-in surface's residual actually is.
    let mut worst = 0.0f64;
    for (u, v) in dense(41, 37) {
        let target = offset_point(&base, d, u, v).unwrap();
        worst = worst.max((rational_fit.eval(u, v) - target).norm());
    }
    // A tolerance ABOVE the true residual: limb 1 cannot refuse, so
    // whatever limb 2 reports is what certifies.
    let tol = worst * 4.0;
    match certify_offset(&base, &rational_fit, d, tol, band()) {
        Ok(cert) => {
            eprintln!(
                "P4: certificate ISSUED for a rational fit — hull_sup={:.3e}, \
                 on_locus={:.3e}, true sampled residual={worst:.3e}",
                cert.hull_sup, cert.on_locus_max
            );
            assert!(
                worst <= cert.hull_sup,
                "P4: the certificate's hull_sup {} UNDER-reports the sampled residual \
                 {worst} of the surface that was handed in — limb 2 certified a \
                 DIFFERENT surface (it ignores fit.weights())",
                cert.hull_sup
            );
        }
        Err(e) => eprintln!("P4: refused (no certificate escapes): {e}"),
    }
}

/// A quarter cylinder, exact (the shipped suite's fixture, re-spelled
/// here so the probes stand alone).
fn quarter_cylinder(r: f64, h: f64) -> NurbsSurface<f64> {
    let s = (FRAC_PI_2 * 0.5).cos();
    let control = vec![
        Point3::new(r, 0.0, 0.0),
        Point3::new(r, 0.0, h),
        Point3::new(r, r, 0.0),
        Point3::new(r, r, h),
        Point3::new(0.0, r, 0.0),
        Point3::new(0.0, r, h),
    ];
    NurbsSurface::new(kv2(), kv1(), control, vec![1.0, 1.0, s, s, 1.0, 1.0]).unwrap()
}

/// **P6 — the regularity predicate's lever.** `offset_normal_floor`
/// classifies `Margin::levered(sine_floor, |d|)`. The geometry it is
/// supposed to be about (is the chart normal degenerate?) does not
/// depend on `d` at all, but the margin is proportional to `|d|`, so
/// the SAME perfectly regular patch is classified differently at
/// different offset distances. This row fits an exact quarter cylinder
/// — whose sine floor is ~1 — at a small `|d|`, with a tolerance that
/// is loose in absolute terms. It goes RED if a regular patch is
/// refused at the regularity meter purely because `|d|` is small.
#[test]
fn p6_a_regular_patch_is_refused_when_d_is_small() {
    let base = quarter_cylinder(1.0, 1.0);
    let cells = patch_cells_refined(&base, OFFSET_METER_LADDER[0]).unwrap();
    let reg = geom_brep::offset_meters::patch_regularity(&cells);
    eprintln!(
        "P6: cylinder sine_floor = {:.6}, floor = {:.6}",
        reg.sine_floor, reg.floor
    );
    for d in [1e-3_f64, 1e-6, 1e-9] {
        let r = fit_offset(&base, d, 1e-3, band());
        let refused_at_the_floor = matches!(
            r,
            Err(OffsetFitError::Meter(
                geom_brep::offset_meters::MeterError::NormalFloor { .. }
                    | geom_brep::offset_meters::MeterError::Escalated { .. }
            ))
        );
        eprintln!(
            "P6: d = {d:.0e} -> {}",
            match &r {
                Ok((_, c)) => format!("certified, hull_sup = {:.3e}", c.hull_sup),
                Err(e) => format!("REFUSED: {e}"),
            }
        );
        assert!(
            !refused_at_the_floor,
            "P6: the quarter cylinder (sine_floor {:.4}) was refused at the REGULARITY \
             meter at d = {d} — the patch's normal does not depend on d, so the meter's \
             |d| lever is deciding geometry it is not about",
            reg.sine_floor
        );
    }
}

/// The refinement loop's termination and the honesty of its payload,
/// pinned from outside: an unreachable tolerance must stop, and the
/// `achieved` it reports must be a bound the same door re-derives.
#[test]
fn p5_budget_refusal_payload_is_the_bound_the_door_re_derives() {
    let base = skinned_loft();
    let d = 0.08;
    match fit_offset(&base, d, 1e-14, band()) {
        Err(OffsetFitError::BudgetExhausted {
            achieved,
            tolerance,
            grid,
            ..
        }) => {
            assert!(
                achieved.is_finite(),
                "the payload's achieved bound is {achieved}"
            );
            assert!(achieved > tolerance);
            // The same base at a tolerance the achieved bound clears
            // must now certify — i.e. `achieved` is a real bound the
            // loop reached, not a number it printed.
            let (_, cert) = fit_offset(&base, d, achieved * 1.5, band()).unwrap_or_else(|e| {
                panic!("the achieved bound {achieved} is not reachable by the same door: {e}")
            });
            assert!(cert.hull_sup <= achieved * 1.5);
            eprintln!(
                "P5: budget refusal grid={grid:?} achieved={achieved:.3e}; \
                 re-fit at 1.5x certifies at {:.3e}",
                cert.hull_sup
            );
        }
        other => panic!("an unreachable tolerance did not refuse typed: {other:?}"),
    }
}
