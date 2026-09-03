//! **M5 PR 7b acceptance — tensor-product Bernstein composition**
//! (spec §5's tensor-compose unit rows):
//!
//! 1. The hull bound is a **true sup bound**: dense-scan falsification
//!    probe, ≥1e5 samples against an independent `f64` rational
//!    oracle, ratio ≥ 1.0 — on an aligned pair, on a knot-mismatched
//!    pair (the merged-break path), and on a cell-straddling pcurve.
//!    Three of those rows also state a CEILING, each measured on its
//!    own geometry against the enclosure the composition exists to
//!    beat; the bicubic budget row states in place why it can carry
//!    no honest one. The remaining `falsify` callers — refinement and
//!    the far-origin shift — bound their result against ANOTHER BOUND
//!    rather than against the truth, which is a different claim.
//! 2. The cancellation row: a constructed `S`, `P`, `C` with
//!    `S(P(t)) ≡ C(t)` exactly — the composite bound lands at ring
//!    rounding (~1e-15), where any hull-then-difference enclosure is
//!    O(1) (the review M2 finding, pinned as behavior).
//! 3. Poison-on-zero-denominator: a weight extension that changes sign
//!    inside the reachable window poisons the bound (NaN), never
//!    panics, never understates.
//! 4. Degree budget: the largest SSI-realistic composition (bicubic ×
//!    bicubic against cubic curves, composite degree 21 of the 54
//!    budget) completes finite; a beyond-budget pair poisons loudly.
//! 5. Ring-lane bit-replay: the whole pipeline is deterministic to the
//!    bit (D9).
//! 6. Typed refusals at the entry points (closed `ComposeError`).
//!
//! These rows are ε-independent (pure ring arithmetic; no `Tolerance`
//! read), so the battery's ε sweep changes nothing here by design —
//! and every ratio below was confirmed BIT-IDENTICAL across the
//! battery's three ε legs (`ci-filter.py`'s `EPS_ROWS`: default,
//! 1e-6, 1e-12, where default is the compiled `DEFAULT_EPS = 1e-9`).
//! A multi-ε measurement of these ceilings is therefore degenerate:
//! the legs are the same run. What varies them is the geometry and
//! the mechanism, which is why each ceiling below is measured per row
//! against a DEGRADED reading of its own fixture.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::RingInterval;
use geom_core::spline::compose::tensor::{SurfaceRingData, surface_curve_residual};
use geom_core::spline::compose::{ComposeError, CurveRingData};
use geom_core::spline::{KnotVector, basis};
use test_utils::tightness::{Anchor, Sup, control_net_box_diagonal};

// ---------------------------------------------------------------------
// Independent f64 rational oracles (basis functions only — no ring, no
// decomposition, no compose code path)
// ---------------------------------------------------------------------

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

/// Row-major `iu·nv + iv` control layout, like the kernel's surfaces.
fn surf_eval(
    ku: &KnotVector,
    kv: &KnotVector,
    w: &[f64],
    coords: &[Vec<f64>],
    u: f64,
    v: f64,
) -> [f64; 3] {
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

// ---------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------

/// A curved bicubic × linear wall with non-unit weights (the rational
/// path must be exercised, not just the polynomial one). Row-major
/// `iu·nv + iv`, `nv = 2`.
fn wall() -> (KnotVector, KnotVector, Vec<f64>, Vec<Vec<f64>>) {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let cols = [(0.0, 0.0), (0.35, 0.18), (0.70, -0.12), (1.05, 0.04)];
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();
    for (cx, cy) in cols {
        x.extend([cx, cx]);
        y.extend([cy, cy]);
        z.extend([0.0, 0.8]);
    }
    let weights = vec![1.0, 1.2, 0.9, 1.0, 1.1, 1.0, 1.0, 0.8];
    (ku, kv, weights, vec![x, y, z])
}

/// A cubic parameter curve crossing the wall's chart diagonally, and a
/// deliberately imperfect cubic carrier near (not on) the composite —
/// the residual is genuinely nonzero, so the falsification ratio is a
/// statement about a real function, not about zero.
fn pcurve_data() -> (KnotVector, Vec<f64>, Vec<Vec<f64>>) {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let u = vec![0.05, 0.3, 0.5, 0.75, 0.95];
    let v = vec![0.1, 0.35, 0.6, 0.7, 0.9];
    let w = vec![1.0, 1.1, 1.0, 0.9, 1.0];
    (kv, w, vec![u, v])
}

fn carrier_data() -> (KnotVector, Vec<f64>, Vec<Vec<f64>>) {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    // Roughly along the wall's image of the pcurve, off by ~1e-2.
    let x = vec![0.05, 0.3, 0.52, 0.76, 0.99];
    let y = vec![0.02, 0.1, 0.05, -0.02, 0.03];
    let z = vec![0.08, 0.3, 0.47, 0.56, 0.72];
    let w = vec![1.0, 1.0, 1.05, 1.0, 1.0];
    (kv, w, vec![x, y, z])
}

/// `|S(P(t)) − C(t)|` at `t`, entirely through the f64 oracles.
fn oracle_residual(
    wall: &(KnotVector, KnotVector, Vec<f64>, Vec<Vec<f64>>),
    pc: &(KnotVector, Vec<f64>, Vec<Vec<f64>>),
    ca: &(KnotVector, Vec<f64>, Vec<Vec<f64>>),
    t: f64,
) -> f64 {
    let p = curve_eval(&pc.0, &pc.1, &pc.2, t);
    let c = curve_eval(&ca.0, &ca.1, &ca.2, t);
    let s = surf_eval(&wall.0, &wall.1, &wall.2, &wall.3, p[0], p[1]);
    ((s[0] - c[0]).powi(2) + (s[1] - c[1]).powi(2) + (s[2] - c[2]).powi(2)).sqrt()
}

/// The composite sup bound for a (wall, pcurve, carrier) triple.
fn sup_of(
    wall: &(KnotVector, KnotVector, Vec<f64>, Vec<Vec<f64>>),
    pc: &(KnotVector, Vec<f64>, Vec<Vec<f64>>),
    ca: &(KnotVector, Vec<f64>, Vec<Vec<f64>>),
    extra: &[f64],
) -> f64 {
    let (sx, px, cx) = (lift(&wall.3), lift(&pc.2), lift(&ca.2));
    let s = SurfaceRingData::new(&wall.0, &wall.1, &wall.2, &sx).unwrap();
    let p = CurveRingData::new(&pc.0, &pc.1, &px).unwrap();
    let c = CurveRingData::new(&ca.0, &ca.1, &cx).unwrap();
    surface_curve_residual(&s, &p, &c, extra)
        .unwrap()
        .sup_bound()
}

/// The dense-scan falsification probe: `samples` oracle evaluations,
/// the observed max, and the bound it must dominate.
fn falsify(
    wall: &(KnotVector, KnotVector, Vec<f64>, Vec<Vec<f64>>),
    pc: &(KnotVector, Vec<f64>, Vec<Vec<f64>>),
    ca: &(KnotVector, Vec<f64>, Vec<Vec<f64>>),
    extra: &[f64],
    samples: usize,
) -> (f64, f64) {
    let sup = sup_of(wall, pc, ca, extra);
    let (t0, t1) = pc.0.domain();
    let mut max = 0.0f64;
    for i in 0..=samples {
        #[allow(clippy::cast_precision_loss)]
        let t = t0 + (t1 - t0) * (i as f64 / samples as f64);
        let r = oracle_residual(wall, pc, ca, t);
        if r > max {
            max = r;
        }
    }
    (sup, max)
}

// ---------------------------------------------------------------------
// Row 1: the hull bound is a true sup bound (dense-scan falsification)
// ---------------------------------------------------------------------

#[test]
fn the_bound_dominates_a_dense_scan_on_the_aligned_pair() {
    let (w, p, c) = (wall(), pcurve_data(), carrier_data());
    let (sup, max) = falsify(&w, &p, &c, &[], 100_000);
    assert!(sup.is_finite(), "bound must be finite here, got {sup}");
    // And the bound is TIGHT — the whole point of the composition. The
    // ceiling is set against the enclosure the composition exists to
    // beat: replacing `cell_residual`'s coefficient subtraction with a
    // hull-then-difference of the two hulls — still sound, a pure
    // tightness loss — takes this row from 1.491x the sampled truth to
    // 20.843x. 3.0 is twice the healthy ratio and seven times under the
    // degraded one. The whole-object box would admit 45.707x, which is
    // a necessary check on the ceiling and NOT what makes it a guard —
    // the degraded reading is what does, and it sits well under the
    // box (`test_utils::tightness`).
    Sup::new("aligned pair", sup, max)
        .truth_at_least(
            1e-3,
            "`carrier_data` is authored ~1e-2 m off the wall's image of the pcurve, \
             so a residual under this means the two curves have converged and the \
             falsification probe is scanning zero",
        )
        .dominates()
        .within(
            3.0,
            0.0,
            Anchor::ObjectBox(control_net_box_diagonal(&[&w.3, &c.2])),
            "the composite bound lost the cancellation — the degraded reading here \
             is 20.843x",
        );
}

#[test]
fn the_bound_dominates_when_the_two_curves_disagree_on_knots() {
    // The merged-break path: carrier on a DIFFERENT interior knot set
    // than the pcurve (0.25/0.75 vs 0.5). Exact insertion, still sound.
    let (w, p, _) = (wall(), pcurve_data(), ());
    let kv =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.75, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let x = vec![0.05, 0.25, 0.5, 0.7, 0.9, 0.99];
    let y = vec![0.02, 0.12, 0.06, -0.01, 0.0, 0.03];
    let z = vec![0.08, 0.25, 0.45, 0.5, 0.6, 0.72];
    let ca = (kv, vec![1.0; 6], vec![x, y, z]);
    let (sup, max) = falsify(&w, &p, &ca, &[], 100_000);
    assert!(sup.is_finite(), "bound must be finite here, got {sup}");
    // Exact insertion costs no tightness: the merged-break bound tracks
    // the residual's own scale (1.520x) as closely as the aligned
    // pair's does (1.491x). Under the hull-then-difference enclosure
    // this row reads 6.730x, so 3.0 sits 2.2x under the degraded state
    // and 2.0x over the healthy one. The whole-object box would admit
    // 16.000x — a ceiling anywhere under that passes the anchor while
    // saying nothing, which is why the degraded reading is the
    // evidence and the box is only a floor under the argument.
    Sup::new("merged-break carrier", sup, max)
        .truth_at_least(
            1e-2,
            "this row's own six-point carrier sits ~1e-2 m off the composite on a \
             DIFFERENT interior knot set; a residual under this means the exact \
             insertion collapsed it onto the pcurve's breaks and the merged-break \
             path is no longer being exercised",
        )
        .dominates()
        .within(
            3.0,
            0.0,
            Anchor::ObjectBox(control_net_box_diagonal(&[&w.3, &ca.2])),
            "the merged-break path lost the cancellation — the degraded reading \
             here is 6.730x",
        );
}

#[test]
fn the_bound_dominates_when_the_pcurve_straddles_surface_cells() {
    // A wall with an interior knot line in u: spans of the pcurve
    // whose window straddles it are hulled across BOTH cells.
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kvv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let xs = [0.0, 0.3, 0.7, 1.0];
    let ys = [0.0, 0.2, -0.1, 0.05];
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();
    for i in 0..4 {
        x.extend([xs[i], xs[i]]);
        y.extend([ys[i], ys[i]]);
        z.extend([0.0, 1.0]);
    }
    let w = (ku, kvv, vec![1.0; 8], vec![x, y, z]);
    let (p, c) = (pcurve_data(), carrier_data());
    let (sup, max) = falsify(&w, &p, &c, &[], 100_000);
    assert!(sup.is_finite(), "bound must be finite here, got {sup}");
    // The fixture must actually straddle. A pcurve confined to one cell
    // measures the same residual, so the floor below cannot detect this
    // and nothing else in the row would: assert the crossing itself.
    let (u_lo, u_hi) = p.2[0]
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), v| {
            (l.min(*v), h.max(*v))
        });
    assert!(
        u_lo < 0.5 && u_hi > 0.5,
        "this row is about the CROSS-CELL hull, but the pcurve's u range \
         [{u_lo}, {u_hi}] does not cross the wall's interior knot at 0.5"
    );
    // Straddling costs tightness and this states how much: the window is
    // hulled across BOTH cells, so the enclosure is a union of two
    // cells' boxes — 3.264x the true residual against the aligned pair's
    // 1.491x. Under the hull-then-difference enclosure it reads 5.108x,
    // so 4.0 is the value that separates the two, with 22% over the
    // healthy ratio and 28% under the degraded one. That narrowness is
    // the finding: on this fixture the cross-cell union and a lost
    // cancellation are nearly the same size, and the file's actual
    // cancellation witness is
    // `an_exact_composite_bounds_at_ring_rounding_not_at_the_variation`,
    // which dies outright under that enclosure.
    Sup::new("cell-straddling pcurve", sup, max)
        .truth_at_least(
            1e-2,
            "the residual across the two-cell wall is an order larger than the \
             single-cell one; a value under this means the cross-cell image and \
             the carrier have converged and the union hull below has nothing to \
             enclose",
        )
        .dominates()
        .within(
            4.0,
            0.0,
            Anchor::ObjectBox(control_net_box_diagonal(&[&w.3, &c.2])),
            "the cross-cell hull is no longer residual-scaled — the degraded \
             reading here is 5.108x, and the whole-object box would admit 7.744x, \
             which the OLD 6.0 ceiling passed while missing the degradation",
        );
}

#[test]
fn refinement_breaks_tighten_without_ever_undercutting() {
    let (w, p, c) = (wall(), pcurve_data(), carrier_data());
    let coarse = sup_of(&w, &p, &c, &[]);
    #[allow(clippy::cast_precision_loss)]
    let fine_breaks: Vec<f64> = (1..64).map(|i| i as f64 / 64.0).collect();
    let (fine, max) = falsify(&w, &p, &c, &fine_breaks, 100_000);
    assert!(fine >= max, "refined bound undercuts: {fine:e} < {max:e}");
    assert!(
        fine <= coarse,
        "refinement must not loosen: {fine:e} > {coarse:e}"
    );
}

#[test]
fn a_far_from_origin_wall_bounds_at_the_representation_floor() {
    // The center-shift row (fix pass, review MINOR 2): the same
    // realistic wall/pcurve/carrier triple translated 1e6 m from the
    // origin. The +1e6 rounds each control coordinate (half-ulp(1e6)
    // ≈ 5.8e-11), so the translated TRUTH is the near-origin truth
    // plus ~1e-10 of representation noise — and the composite bound
    // must stay at that floor, not at the unshifted pipeline's
    // magnitude-scaled ~1e-6 failure mode. Still a bound (dense-scan
    // dominated) and still tight (tracks the translated truth).
    //
    // The full plane×NURBS operation does NOT run at 1e6 m today —
    // foot-point projection and the exhaustiveness sweep hit their own
    // representation floors first (typed refusals, pre-existing
    // machinery) — so the certification-grade far-origin pin lives
    // here, on the limb-2 composite itself.
    let (mut w, p, mut c) = (wall(), pcurve_data(), carrier_data());
    let near = sup_of(&w, &p, &c, &[]);
    for ch in 0..3 {
        for v in w.3[ch].iter_mut() {
            *v += 1.0e6;
        }
        for v in c.2[ch].iter_mut() {
            *v += 1.0e6;
        }
    }
    let (far, max) = falsify(&w, &p, &c, &[], 100_000);
    assert!(far.is_finite() && far >= max, "far {far:e}, max {max:e}");
    // Band-relative sameness: the residual here is O(1e-2), so the
    // translation's ~1e-10 noise must be invisible at the bound's own
    // scale — far and near agree to well under a percent.
    assert!(
        (far - near).abs() <= 1e-2 * near,
        "far-origin bound left the near-origin bound: {far:e} vs {near:e}"
    );
}

// ---------------------------------------------------------------------
// Row 2: the cancellation survives composition-then-hull
// ---------------------------------------------------------------------

#[test]
fn an_exact_composite_bounds_at_ring_rounding_not_at_the_variation() {
    // S(u,v) = (u, v, u+v) as a bilinear patch; P(t) = (t, t²) as a
    // quadratic Bézier; C(t) = S(P(t)) = (t, t², t+t²) authored EXACTLY
    // as a quadratic Bézier. The residual is identically zero, but both
    // S∘P and C individually sweep O(1) across the single span — any
    // hull-then-difference enclosure (midpoint + two variation radii)
    // reports that O(1) motion. The composite must report ~1e-15:
    // the cancellation happens between coefficients that are the same
    // ring numbers, before any hull is taken.
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    // Row-major iu·nv+iv with nv = 2: corners (u,v) = 00, 01, 10, 11.
    let w = (
        ku,
        kv,
        vec![1.0; 4],
        vec![
            vec![0.0, 0.0, 1.0, 1.0],
            vec![0.0, 1.0, 0.0, 1.0],
            vec![0.0, 1.0, 1.0, 2.0],
        ],
    );
    let kq = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let p = (
        kq.clone(),
        vec![1.0; 3],
        vec![vec![0.0, 0.5, 1.0], vec![0.0, 0.0, 1.0]],
    );
    let c = (
        kq,
        vec![1.0; 3],
        vec![
            vec![0.0, 0.5, 1.0],
            vec![0.0, 0.0, 1.0],
            vec![0.0, 0.5, 2.0],
        ],
    );
    let sup = sup_of(&w, &p, &c, &[]);
    assert!(
        sup <= 1e-12,
        "the cancellation was lost in the coefficients: sup = {sup:e}"
    );
    // The same identity holds through the falsification harness.
    let (sup2, max) = falsify(&w, &p, &c, &[], 100_000);
    assert!(sup2 >= max && max <= 1e-12, "sup {sup2:e}, max {max:e}");
}

// ---------------------------------------------------------------------
// Row 3: zero-touching denominator poisons loudly
// ---------------------------------------------------------------------

#[test]
fn a_sign_changing_weight_extension_poisons_the_bound() {
    // Bilinear patch on [0,1]² whose weight function along v is
    // w(u,·) = 1 + 99u; the pcurve runs u from −0.5 to 1, so the
    // boundary cell's polynomial extension (the documented domain
    // posture) sees the weight change sign inside the reachable
    // window. The ring refuses the zero-touching divisor: the bound is
    // NaN — poisoned, not panicked, and it fails any ≤ ε comparison.
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let w = (
        ku,
        kv,
        vec![1.0, 1.0, 100.0, 100.0],
        vec![
            vec![0.0, 0.0, 1.0, 1.0],
            vec![0.0, 1.0, 0.0, 1.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ],
    );
    let kl = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let p = (
        kl.clone(),
        vec![1.0; 2],
        vec![vec![-0.5, 1.0], vec![0.5, 0.5]],
    );
    let c = (
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![1.0; 2],
        vec![vec![0.0, 1.0], vec![0.0, 0.0], vec![0.0, 0.0]],
    );
    let sup = sup_of(&w, &p, &c, &[]);
    assert!(sup.is_nan(), "expected poison, got {sup:e}");
}

// ---------------------------------------------------------------------
// Row 4: the degree budget — in it, and beyond it
// ---------------------------------------------------------------------

/// A degree (du, dv) single-patch surface with `S(u,v) ≈ paraboloid`.
fn elevated_patch(du: usize, dv: usize) -> (KnotVector, KnotVector, Vec<f64>, Vec<Vec<f64>>) {
    let clamp = |d: usize| {
        let mut k = vec![0.0; d + 1];
        k.extend(vec![1.0; d + 1]);
        KnotVector::clamped(k, d).unwrap()
    };
    let (ku, kv) = (clamp(du), clamp(dv));
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
    (ku, kv, vec![1.0; nu * nv], vec![x, y, z])
}

#[test]
fn a_bicubic_bicubic_composition_completes_within_the_budget() {
    // The largest SSI-realistic shape: bicubic × bicubic wall, cubic
    // pcurve and carrier — composite degree 3·(3+3)+3 = 21 of the 54
    // budget. It must complete with a finite, sound bound.
    //
    //
    // **The tightest ceiling in this file, and it is tight because the
    // fixture leaves little room.** The healthy ratio is 1.443x the
    // sampled truth; a TOTAL loss of the cancellation — `cell_residual`
    // hulling each product and then subtracting — reads 1.809x. That is
    // a quarter of a factor of separation, against the aligned row's
    // fourteen-fold. The cause is the fixture: the paraboloid patch and
    // the carrier are nearly as far apart as the object is big (a
    // residual of 0.910 m inside a box of 1.600 m), so the enclosure
    // has almost nothing to be tight about and the whole-object box
    // admits only 1.757x. 1.6 sits 11% over the healthy reading and 12%
    // under the degraded one — narrow, but the whole computation is
    // pure ring arithmetic and bit-identical across the battery, so the
    // margin is real rather than noise budget. If a legitimate change
    // moves it, this row reds with both numbers in the message and the
    // literal is re-measured rather than widened.
    let w = elevated_patch(3, 3);
    let (p, c) = (pcurve_data(), carrier_data());
    let (sup, max) = falsify(&w, &p, &c, &[], 100_000);
    assert!(sup.is_finite(), "in-budget composition poisoned: {sup:e}");
    Sup::new("bicubic x bicubic at degree 21", sup, max)
        .truth_at_least(
            1e-1,
            "the paraboloid patch and the carrier are genuinely apart, so the \
             domination is a statement about a real function",
        )
        .dominates()
        .within(
            1.6,
            0.0,
            Anchor::ObjectBox(control_net_box_diagonal(&[&w.3, &c.2])),
            "the in-budget composition lost the cancellation — the degraded \
             reading here is 1.809x, the narrowest separation in this file",
        );
}

#[test]
fn a_beyond_budget_composition_poisons_rather_than_rounds() {
    // Degrees (9, 9) against cubic curves: composite numerator degree
    // 3·(9+9)+3 = 57 > 54 — the binomial row poisons and the bound is
    // NaN. Loud, never a silently rounded weight.
    let w = elevated_patch(9, 9);
    let (p, c) = (pcurve_data(), carrier_data());
    let sup = sup_of(&w, &p, &c, &[]);
    assert!(sup.is_nan(), "expected the budget poison, got {sup:e}");
}

// ---------------------------------------------------------------------
// Row 5: ring-lane bit-replay (D9)
// ---------------------------------------------------------------------

#[test]
fn the_pipeline_is_deterministic_to_the_bit() {
    let (w, p, c) = (wall(), pcurve_data(), carrier_data());
    let (sx, px, cx) = (lift(&w.3), lift(&p.2), lift(&c.2));
    let run = || {
        let s = SurfaceRingData::new(&w.0, &w.1, &w.2, &sx).unwrap();
        let pd = CurveRingData::new(&p.0, &p.1, &px).unwrap();
        let cd = CurveRingData::new(&c.0, &c.1, &cx).unwrap();
        surface_curve_residual(&s, &pd, &cd, &[0.1, 0.9]).unwrap()
    };
    let (a, b) = (run(), run());
    assert_eq!(a.breaks(), b.breaks());
    assert_eq!(a.span_bounds().len(), b.span_bounds().len());
    for (ra, rb) in a.span_bounds().iter().zip(b.span_bounds().iter()) {
        for d in 0..3 {
            assert_eq!(ra[d].lo().to_bits(), rb[d].lo().to_bits());
            assert_eq!(ra[d].hi().to_bits(), rb[d].hi().to_bits());
        }
    }
    assert_eq!(a.sup_bound().to_bits(), b.sup_bound().to_bits());
}

// ---------------------------------------------------------------------
// Row 6: typed refusals at the entry points
// ---------------------------------------------------------------------

#[test]
fn the_domain_mismatch_message_carries_the_recourse_exactly_once() {
    // S6/S9 acceptance style for the arm PR 7b added: one situation,
    // one recourse sentence appearing EXACTLY once, both domains
    // riding the payload as data.
    let e = ComposeError::DomainMismatch {
        a: (0.0, 1.0),
        b: (0.0, 2.5),
    };
    let msg = format!("{e}");
    assert_eq!(
        msg.matches("refit the pair on one parameterization")
            .count(),
        1,
        "the recourse fragment must appear exactly once: {msg}"
    );
    assert_eq!(
        msg.matches("the shared-parameter identity").count(),
        1,
        "the identity is named exactly once: {msg}"
    );
    assert!(
        msg.contains("[0, 1]") && msg.contains("[0, 2.5]"),
        "both domains as payload data: {msg}"
    );
}

#[test]
fn the_entry_points_refuse_typed() {
    let (w, p, c) = (wall(), pcurve_data(), carrier_data());
    let (sx, px, cx) = (lift(&w.3), lift(&p.2), lift(&c.2));

    // Surface: weight count, weight sign, channel count.
    match SurfaceRingData::new(&w.0, &w.1, &w.2[..4], &sx) {
        Err(ComposeError::Structure(_)) => {}
        other => panic!("weight count: {other:?}"),
    }
    let bad_w: Vec<f64> =
        w.2.iter()
            .enumerate()
            .map(|(i, x)| if i == 3 { -1.0 } else { *x })
            .collect();
    match SurfaceRingData::new(&w.0, &w.1, &bad_w, &sx) {
        Err(ComposeError::Structure(_)) => {}
        other => panic!("weight sign: {other:?}"),
    }
    match SurfaceRingData::new(&w.0, &w.1, &w.2, &sx[..2]) {
        Err(ComposeError::DimensionMismatch {
            dims: 2,
            expected: 3,
        }) => {}
        other => panic!("channel count: {other:?}"),
    }

    let s = SurfaceRingData::new(&w.0, &w.1, &w.2, &sx).unwrap();
    let pd = CurveRingData::new(&p.0, &p.1, &px).unwrap();
    let cd = CurveRingData::new(&c.0, &c.1, &cx).unwrap();

    // A 3-channel "pcurve" and a 2-channel "carrier" both refuse.
    match surface_curve_residual(&s, &cd, &cd, &[]) {
        Err(ComposeError::DimensionMismatch {
            dims: 3,
            expected: 2,
        }) => {}
        other => panic!("pcurve dims: {other:?}"),
    }
    match surface_curve_residual(&s, &pd, &pd, &[]) {
        Err(ComposeError::DimensionMismatch {
            dims: 2,
            expected: 3,
        }) => {}
        other => panic!("carrier dims: {other:?}"),
    }

    // A carrier on a different knot domain refuses (the OQ4 identity).
    let kv2 = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 2.0], 3).unwrap();
    let cx4: Vec<Vec<RingInterval>> = cx.iter().map(|ch| ch[..4].to_vec()).collect();
    let cd2 = CurveRingData::new(&kv2, &c.1[..4], &cx4).unwrap();
    match surface_curve_residual(&s, &pd, &cd2, &[]) {
        Err(ComposeError::DomainMismatch { .. }) => {}
        other => panic!("domain mismatch: {other:?}"),
    }
}
