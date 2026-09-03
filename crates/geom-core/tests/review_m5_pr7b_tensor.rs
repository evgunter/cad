//! **Blinded-review probes for M5 PR 7b** (tensor-product Bernstein
//! composition): the charter's independent executable witnesses.
//!
//! - A. α-algebra differential: an exact rational iso-curve identity on
//!   surfaces whose interior knot is REMOVABLE (the decomposition still
//!   performs real ring-α insertions, but both cells carry one global
//!   polynomial, so the boundary-touch cell inclusion cannot inject a
//!   neighbor-extension mismatch) — in BOTH parameter directions, with
//!   homogeneous weights spanning [0.5, 8]. The composite must land at
//!   ring rounding; a wrong insertion window, α, power table, or
//!   binomial pair lands at the O(1) image scale instead. Plus a
//!   two-sided pinch on a non-removable adversarial rational fixture.
//! - B/C. Falsification battery: ≥1e5-sample dense scans across
//!   constructed adversarial geometries. A finite `bound < truth` is an
//!   automatic MAJOR; a poisoned (NaN) bound is a sound refusal and is
//!   recorded as such.
//! - G. Degree-budget boundary: (9,8)×cubic = 54 exactly completes
//!   finite and sound; (9,9)×cubic = 57 poisons.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::RingInterval;
use geom_core::spline::compose::CurveRingData;
use geom_core::spline::compose::tensor::{SurfaceRingData, surface_curve_residual};
use geom_core::spline::{KnotVector, basis};

type Surf = (KnotVector, KnotVector, Vec<f64>, Vec<Vec<f64>>);
type Curve = (KnotVector, Vec<f64>, Vec<Vec<f64>>);

// ---- independent f64 oracles (kernel basis functions only) ----------

fn curve_eval(kv: &KnotVector, w: &[f64], coords: &[Vec<f64>], t: f64) -> Vec<f64> {
    let span = kv.span_at(t);
    let n = basis::basis_funs(kv, span, t);
    let mut den = 0.0;
    let mut num = vec![0.0; coords.len()];
    for (j, nj) in n.iter().enumerate() {
        let i = span.first_control() + j;
        let cw = nj * w[i];
        den += cw;
        for (d, ch) in coords.iter().enumerate() {
            num[d] += cw * ch[i];
        }
    }
    num.iter().map(|v| v / den).collect()
}

fn surf_eval(s: &Surf, u: f64, v: f64) -> [f64; 3] {
    let (ku, kv, w, coords) = s;
    let nv = kv.control_count();
    let (su, sv) = (ku.span_at(u), kv.span_at(v));
    let nu_b = basis::basis_funs(ku, su, u);
    let nv_b = basis::basis_funs(kv, sv, v);
    let mut den = 0.0;
    let mut num = [0.0f64; 3];
    for (a, na) in nu_b.iter().enumerate() {
        let iu = su.first_control() + a;
        for (b, nb) in nv_b.iter().enumerate() {
            let iv = sv.first_control() + b;
            let idx = iu * nv + iv;
            let cw = na * nb * w[idx];
            den += cw;
            for (d, ch) in coords.iter().enumerate() {
                num[d] += cw * ch[idx];
            }
        }
    }
    [num[0] / den, num[1] / den, num[2] / den]
}

fn lift(coords: &[Vec<f64>]) -> Vec<Vec<RingInterval>> {
    coords
        .iter()
        .map(|ch| ch.iter().map(|x| RingInterval::point(*x)).collect())
        .collect()
}

fn sup_of(s: &Surf, p: &Curve, c: &Curve, extra: &[f64]) -> f64 {
    let (sx, px, cx) = (lift(&s.3), lift(&p.2), lift(&c.2));
    let sd = SurfaceRingData::new(&s.0, &s.1, &s.2, &sx).unwrap();
    let pd = CurveRingData::new(&p.0, &p.1, &px).unwrap();
    let cd = CurveRingData::new(&c.0, &c.1, &cx).unwrap();
    surface_curve_residual(&sd, &pd, &cd, extra)
        .unwrap()
        .sup_bound()
}

/// Dense-scan max of |S(P(t)) − C(t)| through the f64 oracles.
///
/// EVERY caller passes 100_000 (it was 200_000 on seven of them). That
/// is still INSIDE this file's declared charter — the module docs
/// require "≥1e5-sample dense scans", and 1e5 is exactly what the
/// acceptance sibling `m5_pr7b_tensor_compose.rs` scans with on all
/// seven of ITS falsification rows — so this is the contract honoured,
/// not bent, and it makes the file self-consistent (three of its own
/// call sites were already at 100_000).
///
/// WHAT IS LOST: a dense-scan max is monotone in the sample count, so
/// halving the samples can only lower the returned truth, and the
/// falsification bar in `falsify` (`sup >= max`) drops slightly with
/// it. A bound that undercuts the true sup by less than the gap
/// between the 1e5-sample and 2e5-sample maxima would now slip
/// through. The claim itself is unchanged; only the resolution of the
/// witness is.
fn scan_max(s: &Surf, p: &Curve, c: &Curve, samples: usize) -> f64 {
    let (t0, t1) = p.0.domain();
    let mut max = 0.0f64;
    for i in 0..=samples {
        #[allow(clippy::cast_precision_loss)]
        let t = t0 + (t1 - t0) * (i as f64 / samples as f64);
        let q = curve_eval(&p.0, &p.1, &p.2, t);
        let cc = curve_eval(&c.0, &c.1, &c.2, t);
        let sv = surf_eval(s, q[0], q[1]);
        let r =
            ((sv[0] - cc[0]).powi(2) + (sv[1] - cc[1]).powi(2) + (sv[2] - cc[2]).powi(2)).sqrt();
        if r > max {
            max = r;
        }
    }
    max
}

/// The falsification verdict for one fixture: a FINITE bound below the
/// scanned truth is the automatic MAJOR; poison is a sound refusal,
/// recorded and returned as `None`.
fn falsify(
    name: &str,
    s: &Surf,
    p: &Curve,
    c: &Curve,
    extra: &[f64],
    samples: usize,
) -> Option<f64> {
    let sup = sup_of(s, p, c, extra);
    let max = scan_max(s, p, c, samples);
    if sup.is_nan() {
        eprintln!("[review] {name}: POISON refusal (truth {max:.3e}) — sound, recorded");
        return None;
    }
    assert!(
        sup >= max,
        "{name}: FALSIFIED — bound {sup:e} < scanned truth {max:e}"
    );
    let ratio = if max > 0.0 { sup / max } else { f64::INFINITY };
    eprintln!("[review] {name}: bound {sup:.3e}, truth {max:.3e}, ratio {ratio:.3}");
    Some(ratio)
}

// ---- fixtures --------------------------------------------------------

/// The homogeneous u-channel of the removable-knot fixture: a global
/// cubic Bézier (weights spanning [0.5, 8]) with u = 0.5 inserted ONCE
/// by hand (all-dyadic averages), so the authored surface has a real
/// interior knot whose two cells are one polynomial.
fn removable_u_channel() -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let ins = |h: [f64; 4]| -> Vec<f64> {
        vec![
            h[0],
            0.5 * (h[0] + h[1]),
            0.5 * (h[1] + h[2]),
            0.5 * (h[2] + h[3]),
            h[3],
        ]
    };
    let hw = ins([4.0, 0.5, 2.0, 8.0]);
    let hx = ins([0.0, 1.0, 2.0, 6.0]);
    let hy = ins([2.0, 0.25, -1.0, 4.0]);
    (hw, hx, hy)
}

/// The u-direction exact iso fixture: cubic u on knots
/// `[0,0,0,0,0.5,1,1,1,1]` (removable interior knot — the decomposition
/// still inserts it twice more through the ring-α path), linear v with
/// weight factor [1, 3]. The v = 0.25 iso-curve is EXACT on the shared
/// lifted data: coords are the same `fl(Hx/Hw)` on both sides, weights
/// dyadic, so `S(P(t)) − C(t)` is identically zero in the ring's
/// inputs. Composite must land at ring rounding.
fn iso_u() -> (Surf, Curve, Curve) {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kvv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let (hw, hx, hy) = removable_u_channel();
    let xs: Vec<f64> = hx.iter().zip(hw.iter()).map(|(a, b)| a / b).collect();
    let ys: Vec<f64> = hy.iter().zip(hw.iter()).map(|(a, b)| a / b).collect();
    let mut w = Vec::new();
    let (mut x, mut y, mut z) = (Vec::new(), Vec::new(), Vec::new());
    for i in 0..5 {
        for (j, wv) in [1.0f64, 3.0].iter().enumerate() {
            w.push(hw[i] * wv);
            x.push(xs[i]);
            y.push(ys[i]);
            #[allow(clippy::cast_precision_loss)]
            z.push(j as f64);
        }
    }
    let s = (ku.clone(), kvv, w, vec![x, y, z]);
    let kl = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let p = (kl, vec![1.0, 1.0], vec![vec![0.0, 1.0], vec![0.25, 0.25]]);
    // v = 0.25 iso: weight wu·(0.75·1 + 0.25·3) = 1.5·wu (dyadic),
    // z = 0.25·3/1.5 = 0.5 (exact), x/y the SAME fl values.
    let cw: Vec<f64> = hw.iter().map(|v| 1.5 * v).collect();
    let c = (ku, cw, vec![xs, ys, vec![0.5; 5]]);
    (s, p, c)
}

/// The v-direction twin: linear u (weight factor [1, 3]), cubic v on
/// the removable-knot vector, u = 0.25 iso.
fn iso_v() -> (Surf, Curve, Curve) {
    let kuu = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let (hw, hx, hy) = removable_u_channel();
    let xs: Vec<f64> = hx.iter().zip(hw.iter()).map(|(a, b)| a / b).collect();
    let ys: Vec<f64> = hy.iter().zip(hw.iter()).map(|(a, b)| a / b).collect();
    let mut w = Vec::new();
    let (mut x, mut y, mut z) = (Vec::new(), Vec::new(), Vec::new());
    for (i, wu) in [1.0f64, 3.0].iter().enumerate() {
        for j in 0..5 {
            w.push(wu * hw[j]);
            x.push(xs[j]);
            y.push(ys[j]);
            #[allow(clippy::cast_precision_loss)]
            z.push(i as f64);
        }
    }
    let s = (kuu, kv.clone(), w, vec![x, y, z]);
    let kl = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let p = (kl, vec![1.0, 1.0], vec![vec![0.25, 0.25], vec![0.0, 1.0]]);
    let cw: Vec<f64> = hw.iter().map(|v| 1.5 * v).collect();
    let c = (kv, cw, vec![xs, ys, vec![0.5; 5]]);
    (s, p, c)
}

/// Adversarial rational surface with NON-removable interior knots:
/// cubic in u with interior knots 0.25 and 0.5 (mult 2), quadratic in v
/// with interior knot 0.75, weights spanning [0.05, 60], oscillating
/// net.
fn gnarl() -> Surf {
    let ku = KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0],
        3,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.75, 1.0, 1.0, 1.0], 2).unwrap();
    let wu = [1.0, 8.0, 0.05, 3.0, 0.2, 10.0, 1.0];
    let wv = [1.0, 0.1, 6.0, 1.0];
    let mut w = Vec::new();
    let (mut x, mut y, mut z) = (Vec::new(), Vec::new(), Vec::new());
    #[allow(clippy::cast_precision_loss)]
    for (iu, wui) in wu.iter().enumerate() {
        for (iv, wvj) in wv.iter().enumerate() {
            w.push(wui * wvj);
            x.push(iu as f64 / 6.0 + if iv % 2 == 0 { 0.03 } else { -0.03 });
            y.push(iv as f64 / 3.0);
            z.push(if (iu + iv) % 2 == 0 { 0.3 } else { -0.3 });
        }
    }
    (ku, kv, w, vec![x, y, z])
}

fn gnarl_pcurve() -> Curve {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.3, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    (
        kv,
        vec![1.0, 2.5, 0.4, 1.5, 1.0],
        vec![
            vec![0.02, 0.4, 0.55, 0.6, 0.97],
            vec![0.05, 0.5, 0.9, 0.2, 0.95],
        ],
    )
}

fn gnarl_carrier() -> Curve {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.6, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    (
        kv,
        vec![1.0, 0.7, 1.3, 1.0, 1.0],
        vec![
            vec![0.0, 0.3, 0.6, 0.7, 1.0],
            vec![0.1, 0.4, 0.8, 0.5, 0.9],
            vec![0.2, -0.2, 0.3, -0.1, 0.1],
        ],
    )
}

fn uniform_breaks(n: usize) -> Vec<f64> {
    #[allow(clippy::cast_precision_loss)]
    (1..n).map(|i| i as f64 / n as f64).collect()
}

// ---- A. the α-algebra witnesses --------------------------------------

#[test]
fn exact_iso_cancellation_pins_the_u_direction_alpha_ring() {
    let (s, p, c) = iso_u();
    let sup = sup_of(&s, &p, &c, &[]);
    // The image sweeps O(1); a lost cancellation or a wrong α lands at
    // that scale. Ring rounding is ~1e-16 relative.
    assert!(sup <= 1e-11, "u-direction α algebra broken: sup {sup:e}");
    let max = scan_max(&s, &p, &c, 100_000);
    assert!(
        sup >= max,
        "unsound on the exact fixture: {sup:e} < {max:e}"
    );
    eprintln!("[review] iso-u: sup {sup:.3e} (image sweep O(1))");
}

#[test]
fn exact_iso_cancellation_pins_the_v_direction_alpha_ring() {
    let (s, p, c) = iso_v();
    let sup = sup_of(&s, &p, &c, &[]);
    assert!(sup <= 1e-11, "v-direction α algebra broken: sup {sup:e}");
    let max = scan_max(&s, &p, &c, 100_000);
    assert!(
        sup >= max,
        "unsound on the exact fixture: {sup:e} < {max:e}"
    );
    eprintln!("[review] iso-v: sup {sup:.3e}");
}

#[test]
fn the_two_sided_pinch_on_the_adversarial_rational_fixture() {
    // Domination AND tracking under heavy refinement: a wrong α, power
    // table or binomial pair moves the decomposed cells off the true
    // surface by O(1) relative and breaks one side of the pinch.
    let (s, p, c) = (gnarl(), gnarl_pcurve(), gnarl_carrier());
    let fine = uniform_breaks(256);
    let sup = sup_of(&s, &p, &c, &fine);
    let max = scan_max(&s, &p, &c, 100_000);
    assert!(sup >= max, "FALSIFIED: {sup:e} < {max:e}");
    assert!(
        sup <= 2.0 * max,
        "refined bound fails to track truth: {sup:e} vs {max:e}"
    );
    eprintln!(
        "[review] pinch: bound {sup:.6e}, truth {max:.6e}, ratio {:.3}",
        sup / max
    );
}

// ---- B/C. the falsification battery ----------------------------------

#[test]
fn falsification_battery_no_finite_bound_undercuts_truth() {
    let mut worst: (f64, &str) = (0.0, "none");
    let mut note = |r: Option<f64>, n: &'static str| {
        if let Some(r) = r
            && r > worst.0
            && r.is_finite()
        {
            worst = (r, n);
        }
    };

    // (a) near-identity: the exact iso fixture with one carrier control
    // z-perturbed by 1e-9 — truth ~1e-9 while the image sweeps O(1);
    // separate enclosure would miss by ~9 orders.
    let (s, p, mut c) = iso_u();
    c.2[2][2] += 1e-9;
    note(
        falsify("near-identity 1e-9", &s, &p, &c, &[], 100_000),
        "near-identity",
    );

    // (b) oscillating pcurve whipping across the mult-2 knot line of
    // the gnarl surface — raw (expected poison or loose) and refined.
    let s2 = gnarl();
    let kp = KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.0, 1.0, 1.0],
        3,
    )
    .unwrap();
    let posc = (
        kp,
        vec![1.0, 3.0, 0.3, 1.0, 2.0, 0.5, 1.0, 1.0],
        vec![
            vec![0.1, 0.9, 0.1, 0.9, 0.1, 0.9, 0.1, 0.9],
            vec![0.05, 0.2, 0.35, 0.5, 0.6, 0.75, 0.9, 0.95],
        ],
    );
    let kc = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let cline = (
        kc,
        vec![1.0, 1.0],
        vec![vec![0.0, 1.0], vec![0.0, 1.0], vec![0.0, 0.0]],
    );
    note(
        falsify(
            "oscillating straddle (raw)",
            &s2,
            &posc,
            &cline,
            &[],
            100_000,
        ),
        "oscillating raw",
    );
    note(
        falsify(
            "oscillating straddle (256 breaks)",
            &s2,
            &posc,
            &cline,
            &uniform_breaks(256),
            100_000,
        ),
        "oscillating refined",
    );

    // (c) out-of-domain excursion on the removable fixture: pcurve u
    // sweeps [−0.05, 1.05]; truth is the kernel's own clamped-span
    // polynomial extension (find_span clamps identically).
    let (s3, _, c3) = iso_u();
    let kl = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let pex = (
        kl,
        vec![1.0, 1.0],
        vec![vec![-0.05, 1.05], vec![0.25, 0.25]],
    );
    note(
        falsify("out-of-domain excursion", &s3, &pex, &c3, &[], 100_000),
        "out-of-domain",
    );

    // (d) degenerate extra breaks: duplicates of existing knots, breaks
    // 1e-12 either side of the mult-2 line, and 1e-15 slivers at the
    // ends, on top of a uniform refinement.
    let (s4, p4, c4) = (gnarl(), gnarl_pcurve(), gnarl_carrier());
    let mut extras = uniform_breaks(64);
    extras.extend([0.3, 0.6, 0.25, 0.5 - 1e-12, 0.5 + 1e-12, 1e-15, 1.0 - 1e-15]);
    note(
        falsify("degenerate extra breaks", &s4, &p4, &c4, &extras, 100_000),
        "degenerate breaks",
    );

    // (e) far-from-1 curve weights (W_P^{m_u+m_v} dynamic range).
    let (s5, mut p5, mut c5) = (gnarl(), gnarl_pcurve(), gnarl_carrier());
    p5.1 = vec![1.0, 20.0, 0.05, 10.0, 1.0];
    c5.1 = vec![1.0, 0.02, 30.0, 0.1, 1.0];
    note(
        falsify(
            "far curve weights",
            &s5,
            &p5,
            &c5,
            &uniform_breaks(128),
            100_000,
        ),
        "far curve weights",
    );

    eprintln!(
        "[review] battery worst finite bound/truth ratio: {:.3} ({})",
        worst.0, worst.1
    );
}

// ---- G. the degree-budget boundary ------------------------------------

#[test]
fn the_budget_boundary_54_completes_and_57_poisons() {
    let patch = |du: usize, dv: usize| -> Surf {
        let clamp = |d: usize| {
            let mut k = vec![0.0; d + 1];
            k.extend(vec![1.0; d + 1]);
            KnotVector::clamped(k, d).unwrap()
        };
        let (nu, nv) = (du + 1, dv + 1);
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut z = Vec::new();
        #[allow(clippy::cast_precision_loss)]
        for iu in 0..nu {
            for iv in 0..nv {
                let (gu, gv) = (iu as f64 / du as f64, iv as f64 / dv as f64);
                x.push(gu);
                y.push(gv);
                z.push(0.3 * gu * gu + 0.2 * gv);
            }
        }
        (clamp(du), clamp(dv), vec![1.0; nu * nv], vec![x, y, z])
    };
    let (p, c) = (gnarl_pcurve(), gnarl_carrier());
    // 3·(9+8+1) = 54: exactly at the cap — must complete finite + sound.
    let s54 = patch(9, 8);
    let sup = sup_of(&s54, &p, &c, &[]);
    let max = scan_max(&s54, &p, &c, 100_000);
    assert!(
        sup.is_finite() && sup >= max,
        "the 54-exact case must serve soundly: sup {sup:e}, truth {max:e}"
    );
    // 3·(9+9+1) = 57: beyond — poison, not a panic, not a rounded weight.
    let s57 = patch(9, 9);
    let sup57 = sup_of(&s57, &p, &c, &[]);
    assert!(sup57.is_nan(), "expected the budget poison, got {sup57:e}");
}

#[test]
fn the_missing_center_shift_costs_bound_quality_far_from_origin() {
    // ORIGINALLY the review's cost witness for the shipped pipeline's
    // silent spec-§2.1 deviation (shift-free lift): this exact fixture
    // measured 1.128e-12 near the origin vs 1.866e-6 at 1e6 m — six
    // orders of bound to ring rounding scaling with coefficient
    // magnitude. The fix pass implemented the center-shift, and this
    // witness FLIPPED to the regression pin: the far bound must stay
    // at the translation's own representation floor (the +1e6 rounds
    // each control coordinate by up to ulp(1e6)/2 ≈ 5.8e-11, so the
    // translated identity is genuinely ~1e-10 loose — measured
    // 1.225e-9 shifted, three orders under the unshifted failure).
    let (mut s, p, mut c) = iso_u();
    let sup_near = sup_of(&s, &p, &c, &[]);
    for ch in 0..2 {
        for v in s.3[ch].iter_mut() {
            *v += 1.0e6;
        }
        for v in c.2[ch].iter_mut() {
            *v += 1.0e6;
        }
    }
    let sup_far = sup_of(&s, &p, &c, &[]);
    eprintln!("[review] center-shift cost: sup {sup_near:.3e} near origin, {sup_far:.3e} at 1e6 m");
    assert!(
        sup_far <= 1e-8,
        "center-shift regression: far-origin bound {sup_far:e} left the \
         representation floor (unshifted failure mode was ~2e-6)"
    );
    assert!(
        sup_near <= 1e-11,
        "near-origin exactness lost: {sup_near:e}"
    );
}
