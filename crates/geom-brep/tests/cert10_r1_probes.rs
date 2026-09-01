//! CERT-10 review probes (reviewer-authored, blinded lane).
//!
//! The highest-value falsification: the retired magnitude reading was
//! replaced by a SIGNED quotient-rule evaluation that is 11x tighter on
//! `muv`. Tighter-but-wrong is the failure mode, so these rows sample
//! the TRUE partials densely inside every cell of adversarial rational
//! patches and demand the shipped enclosure contain every sample.

use geom::surfaces::nurbs::NurbsSurface;
use geom_brep::patch_bound::{PatchCell, patch_cells};
use geom_core::spline::knots::KnotVector;
use geom_core::{Point3, Vec3};

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// Clamped knot vector of `degree` with the given interior knots on
/// `[0, 1]`.
fn kv(degree: usize, interior: &[f64]) -> KnotVector {
    let mut k = vec![0.0; degree + 1];
    k.extend_from_slice(interior);
    k.extend(core::iter::repeat_n(1.0, degree + 1));
    KnotVector::clamped(k, degree).expect("knot vector")
}

/// A component of a `Vec3`.
fn comp(v: Vec3<f64>, c: usize) -> f64 {
    match c {
        0 => v.x,
        1 => v.y,
        _ => v.z,
    }
}

/// Report of one containment sweep.
struct Sweep {
    name: &'static str,
    samples: usize,
    /// Worst RELATIVE excursion outside the enclosure, over all
    /// (cell, sample, partial, channel), as a fraction of the
    /// enclosure width (or of |value| when the width is zero).
    worst_rel: f64,
    worst_where: String,
    /// Violations beyond a slack that cannot be f64 evaluation noise.
    hard_violations: usize,
}

fn sweep(name: &'static str, n: &NurbsSurface<f64>, per_cell: usize) -> Sweep {
    let cells: Vec<PatchCell> = patch_cells(n).expect("patch_cells");
    assert!(!cells.is_empty(), "{name}: no cells");
    let mut worst_rel = 0.0f64;
    let mut worst_where = String::new();
    let mut hard = 0usize;
    let mut samples = 0usize;
    for (ci, cell) in cells.iter().enumerate() {
        // The cell's own magnitude scale, over every partial and
        // channel. An excursion has to be judged against the size of
        // the quantities being bounded: `ders` carries its own f64
        // rounding, so an exact-zero bound is legitimately missed by
        // an epsilon-scale evaluation residue, and that is the
        // sampler's noise rather than a broken enclosure.
        let cell_scale = [cell.s_u, cell.s_v, cell.s_uu, cell.s_uv, cell.s_vv]
            .iter()
            .flatten()
            .map(|iv| iv.lo().abs().max(iv.hi().abs()))
            .fold(0.0f64, f64::max);
        for a in 0..per_cell {
            for b in 0..per_cell {
                // STRICTLY INTERIOR sampling. Second partials are
                // genuinely two-valued AT a knot (a C^1 patch is only
                // C^0 in its second derivative there), and `ders`
                // resolves the tie to one side, so a closed-cell
                // sample at an endpoint compares this cell's bound
                // against the NEIGHBOUR cell's derivative. That is a
                // property of the sampler, not of the enclosure.
                let n_s = f64::from(u32::try_from(per_cell).unwrap());
                let tu = (f64::from(u32::try_from(a).unwrap()) + 0.5) / n_s;
                let tv = (f64::from(u32::try_from(b).unwrap()) + 0.5) / n_s;
                let u = cell.u.0 + tu * (cell.u.1 - cell.u.0);
                let v = cell.v.0 + tv * (cell.v.1 - cell.v.0);
                let j = n.ders(u, v);
                samples += 1;
                let truths = [j.du, j.dv, j.duu, j.duv, j.dvv];
                let bounds = [cell.s_u, cell.s_v, cell.s_uu, cell.s_uv, cell.s_vv];
                let labels = ["s_u", "s_v", "s_uu", "s_uv", "s_vv"];
                for (k, (t, bnd)) in truths.iter().zip(bounds.iter()).enumerate() {
                    for c in 0..3 {
                        let x = comp(*t, c);
                        let iv = bnd[c];
                        assert!(!iv.is_poison(), "{name}: poison bound in {}", labels[k]);
                        let (lo, hi) = (iv.lo(), iv.hi());
                        let out = (lo - x).max(x - hi);
                        if out <= 0.0 {
                            continue;
                        }
                        let scale = (hi - lo).max(x.abs()).max(f64::MIN_POSITIVE);
                        let rel = out / scale;
                        if rel > worst_rel {
                            worst_rel = rel;
                            worst_where = format!(
                                "cell {ci} uv=({u:.6},{v:.6}) {} ch{c}: x={x:e} not in [{lo:e},{hi:e}] out={out:e} cell_scale={cell_scale:e}",
                                labels[k]
                            );
                        }
                        // 1e-9 relative is ~7 decades above f64
                        // evaluation noise in `ders`; anything above
                        // that is a real containment failure.
                        if rel > 1e-9 && out > 1e-11 * cell_scale {
                            hard += 1;
                        }
                    }
                }
            }
        }
    }
    Sweep {
        name,
        samples,
        worst_rel,
        worst_where,
        hard_violations: hard,
    }
}

/// Extreme weight ratios: 1e-6 to 1e+6 across one bi-quadratic patch,
/// with a sign-alternating (saddle-checkerboard) control net. This is
/// where "all + and divide by the smallest weight" and "the true minus
/// signs over the whole weight hull" disagree most.
fn extreme_weights() -> NurbsSurface<f64> {
    let ku = kv(2, &[0.35]);
    let kv_ = kv(2, &[0.6]);
    let (nu, nv) = (ku.control_count(), kv_.control_count());
    let mut ctrl = Vec::new();
    let mut ws = Vec::new();
    for i in 0..nu {
        for j in 0..nv {
            let x = f64::from(u32::try_from(i).unwrap());
            let y = f64::from(u32::try_from(j).unwrap());
            // Sign-alternating z: adjacent coefficients differ in sign,
            // so every difference net alternates too.
            let s = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
            ctrl.push(p(x, y, s * (1.0 + 0.5 * x * y)));
            // Weights spanning twelve decades in a checkerboard.
            let w = if (i + 2 * j) % 3 == 0 {
                1e-6
            } else if (i + 2 * j) % 3 == 1 {
                1.0
            } else {
                1e6
            };
            ws.push(w);
        }
    }
    NurbsSurface::new(ku, kv_, ctrl, ws).expect("surface")
}

/// A quarter cylinder — the PR body's own headline fixture for the
/// 11x `muv` tightening, rebuilt here independently.
fn quarter_cylinder() -> NurbsSurface<f64> {
    let ku = kv(2, &[]);
    let kv_ = kv(1, &[]);
    let w = core::f64::consts::FRAC_1_SQRT_2;
    let ctrl = vec![
        p(1.0, 0.0, 0.0),
        p(1.0, 0.0, 2.0),
        p(1.0, 1.0, 0.0),
        p(1.0, 1.0, 2.0),
        p(0.0, 1.0, 0.0),
        p(0.0, 1.0, 2.0),
    ];
    let ws = vec![1.0, 1.0, w, w, 1.0, 1.0];
    NurbsSurface::new(ku, kv_, ctrl, ws).expect("surface")
}

/// Sign-alternating nets at higher degree with several interior knots,
/// weights alternating high/low along BOTH directions so the weight
/// derivative nets alternate in sign as well.
fn alternating_cubic() -> NurbsSurface<f64> {
    let ku = kv(3, &[0.25, 0.5, 0.75]);
    let kv_ = kv(2, &[0.4, 0.8]);
    let (nu, nv) = (ku.control_count(), kv_.control_count());
    let mut ctrl = Vec::new();
    let mut ws = Vec::new();
    for i in 0..nu {
        for j in 0..nv {
            let x = f64::from(u32::try_from(i).unwrap()) * 0.7;
            let y = f64::from(u32::try_from(j).unwrap()) * 1.3;
            let s = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
            ctrl.push(p(x, y, s * 3.0));
            ws.push(if (i + j) % 2 == 0 { 1e-3 } else { 1e3 });
        }
    }
    NurbsSurface::new(ku, kv_, ctrl, ws).expect("surface")
}

/// A near-degenerate weight cliff: one interior weight many decades
/// below its neighbours, so the whole-weight-hull divisor is dominated
/// by a value the true `w(u,v)` almost never attains.
fn weight_cliff() -> NurbsSurface<f64> {
    let ku = kv(2, &[0.5]);
    let kv_ = kv(2, &[0.5]);
    let (nu, nv) = (ku.control_count(), kv_.control_count());
    let mut ctrl = Vec::new();
    let mut ws = Vec::new();
    for i in 0..nu {
        for j in 0..nv {
            let x = f64::from(u32::try_from(i).unwrap());
            let y = f64::from(u32::try_from(j).unwrap());
            ctrl.push(p(x, y, (x - y) * (x + y) * 0.37));
            ws.push(1.0);
        }
    }
    let mid = (nu / 2) * nv + nv / 2;
    ws[mid] = 1e-9;
    NurbsSurface::new(ku, kv_, ctrl, ws).expect("surface")
}

#[test]
fn cert10r1_the_signed_reading_encloses_the_true_partials() {
    let sweeps = vec![
        sweep("extreme_weights", &extreme_weights(), 7),
        sweep("quarter_cylinder", &quarter_cylinder(), 9),
        sweep("alternating_cubic", &alternating_cubic(), 5),
        sweep("weight_cliff", &weight_cliff(), 7),
    ];
    let mut bad = Vec::new();
    for s in &sweeps {
        println!(
            "[cert10r1] {:>18}: {:>7} samples, worst_rel={:e} {}",
            s.name, s.samples, s.worst_rel, s.worst_where
        );
        if s.hard_violations > 0 {
            bad.push(format!("{}: {} hard violations", s.name, s.hard_violations));
        }
    }
    assert!(bad.is_empty(), "containment falsified: {bad:?}");
}

// ---------------------------------------------------------------
// CONTROLS. Before any non-containment above is read as a finding,
// the probe's own two assumptions have to be falsifiable too:
// (a) `ders` really is d/du, d/dv in the same (u, v) sense the cell
//     rectangle names; (b) the sweep passes on an arm that is
//     unquestionably sound (the integral arm is a plain coefficient
//     hull, no quotient rule).
// ---------------------------------------------------------------

/// `ders` against central differences of `eval`, on the rational
/// fixtures themselves. If this row is green, the truth source and the
/// u/v convention the sweep uses are both right.
#[test]
fn cert10r1_control_ders_agrees_with_finite_differences() {
    let fixtures: Vec<(&str, NurbsSurface<f64>)> = vec![
        ("extreme_weights", extreme_weights()),
        ("quarter_cylinder", quarter_cylinder()),
        ("alternating_cubic", alternating_cubic()),
        ("weight_cliff", weight_cliff()),
    ];
    let h = 1e-5;
    let mut worst = 0.0f64;
    let mut worst_where = String::new();
    for (name, n) in &fixtures {
        for a in 1..8u32 {
            for b in 1..8u32 {
                let (u, v) = (f64::from(a) / 8.17 + 0.013, f64::from(b) / 8.17 + 0.017);
                let j = n.ders(u, v);
                let e = |uu: f64, vv: f64| n.eval(uu, vv);
                let fd_u = (e(u + h, v).x - e(u - h, v).x) / (2.0 * h);
                let fd_v = (e(u, v + h).x - e(u, v - h).x) / (2.0 * h);
                let fd_uu = (e(u + h, v).x - 2.0 * e(u, v).x + e(u - h, v).x) / (h * h);
                let fd_vv = (e(u, v + h).x - 2.0 * e(u, v).x + e(u, v - h).x) / (h * h);
                let fd_uv = (e(u + h, v + h).x - e(u + h, v - h).x - e(u - h, v + h).x
                    + e(u - h, v - h).x)
                    / (4.0 * h * h);
                for (lbl, an, fd) in [
                    ("du", j.du.x, fd_u),
                    ("dv", j.dv.x, fd_v),
                    ("duu", j.duu.x, fd_uu),
                    ("dvv", j.dvv.x, fd_vv),
                    ("duv", j.duv.x, fd_uv),
                ] {
                    // Absolute floor: a central difference of a quantity that
                    // is exactly zero returns its own O(eps/h^2) noise.
                    let d = (an - fd).abs();
                    let rel = if d < 1e-4 { 0.0 } else { d / an.abs().max(fd.abs()) };
                    if rel > worst {
                        worst = rel;
                        worst_where =
                            format!("{name} uv=({u},{v}) {lbl}: ders={an:e} fd={fd:e} rel={rel:e}");
                    }
                }
            }
        }
    }
    println!("[cert10r1] control ders-vs-fd worst rel = {worst:e}  {worst_where}");
    // Central differences at h = 1e-5 carry O(h^2) truncation and
    // O(eps/h^2) noise on the second differences; 1e-3 is loose enough
    // for that and three decades tighter than the excursions above.
    assert!(worst < 1e-3, "ders disagrees with finite differences: {worst_where}");
}

/// The SAME sweep on the integral arm (all weights 1.0): unit weights
/// make `patch_cells` take `integral_cells`, whose enclosure is a plain
/// B-spline coefficient hull. If this row is green while the rational
/// rows are red, the sweep is right and the rational arm is what moved.
#[test]
fn cert10r1_control_the_integral_arm_encloses() {
    let unweight = |s: &NurbsSurface<f64>| {
        NurbsSurface::new(
            s.knots_u().clone(),
            s.knots_v().clone(),
            s.control().to_vec(),
            vec![1.0; s.weights().len()],
        )
        .expect("surface")
    };
    let a = unweight(&extreme_weights());
    let b = unweight(&alternating_cubic());
    let c = unweight(&weight_cliff());
    let sweeps = vec![
        sweep("integral(extreme)", &a, 7),
        sweep("integral(cubic)", &b, 5),
        sweep("integral(cliff)", &c, 7),
    ];
    let mut bad = Vec::new();
    for s in &sweeps {
        println!(
            "[cert10r1] CONTROL {:>18}: {:>7} samples, worst_rel={:e} {}",
            s.name, s.samples, s.worst_rel, s.worst_where
        );
        if s.hard_violations > 0 {
            bad.push(format!("{}: {} hard violations", s.name, s.hard_violations));
        }
    }
    assert!(bad.is_empty(), "integral control ALSO fails: {bad:?}");
}

/// ROOT CAUSE of the one residual excursion above. The quarter
/// cylinder's weights do not vary along `v`, so `w_v == 0` in R and
/// `S_vv == 0` exactly. The rational arm bounds the REFINED surface,
/// and f64 knot insertion does not reproduce equal weights bitwise, so
/// `w_v`'s coefficient hull is a tiny nonzero and the signed `s_vv`
/// comes out as a tiny interval that EXCLUDES the true zero. This row
/// records the mechanism; it is ulp-scale and pre-dates this PR's
/// arithmetic, but the retired MAGNITUDE reading could not exhibit it
/// (a magnitude bound is [-m, m], which always contains 0).
#[test]
fn cert10r1_the_residual_is_f64_knot_refinement_not_the_recurrence() {
    let n = quarter_cylinder();
    let nv = n.knots_v().control_count();
    // Before refinement: bitwise equal along v.
    for i in 0..n.knots_u().control_count() {
        for j in 1..nv {
            assert_eq!(n.weights()[i * nv + j], n.weights()[i * nv]);
        }
    }
    let splits = 16;
    let r = n
        .refine_knots_u(&geom_brep::patch_bound::split_points(n.knots_u(), splits))
        .and_then(|r| {
            let sp = geom_brep::patch_bound::split_points(r.knots_v(), splits);
            r.refine_knots_v(&sp)
        })
        .expect("refine");
    let rnv = r.knots_v().control_count();
    let mut unequal = 0usize;
    let mut worst = 0.0f64;
    for i in 0..r.knots_u().control_count() {
        for j in 1..rnv {
            let (a, b) = (r.weights()[i * rnv + j], r.weights()[i * rnv]);
            if a != b {
                unequal += 1;
                worst = worst.max((a - b).abs() / b.abs());
            }
        }
    }
    println!(
        "[cert10r1] refined weights unequal along v: {unequal} slots, worst rel gap {worst:e}"
    );
    assert!(
        unequal > 0,
        "if refinement preserved equality bitwise, the residual would have another cause"
    );
}

// ---------------------------------------------------------------
// The FOLD's soundness premise: "the cell windows COVER the net".
// The PR argues the fold is tighter-or-equal from this premise, but
// the premise carries more weight than that: if some coefficient
// index were covered by NO cell window, the fold would be WRONG
// (below the truth), not merely tighter. The shipped property row
// generates only multiplicity-1 interior knots, so the premise is
// never exercised where it is tight.
// ---------------------------------------------------------------

/// Union-of-windows coverage at every degree and every interior
/// multiplicity the C1 gate admits, for the VALUE net and for the
/// first- and second-derivative nets.
#[test]
fn cert10r1_cell_windows_cover_the_net_at_every_admissible_multiplicity() {
    let mut rows = Vec::new();
    for p in 1..=5usize {
        // The C1 gate: degree 1 must be single-span; degree >= 2
        // admits interior multiplicity up to p - 1.
        let mults: Vec<usize> = if p == 1 { vec![] } else { (1..=p - 1).collect() };
        for m in mults {
            let mut k = vec![0.0; p + 1];
            for (idx, t) in [0.3, 0.7].iter().enumerate() {
                let _ = idx;
                k.extend(core::iter::repeat_n(*t, m));
            }
            k.extend(core::iter::repeat_n(1.0, p + 1));
            let Ok(kvv) = KnotVector::clamped(k, p) else {
                continue;
            };
            let n = kvv.control_count();
            let mut cover_val = vec![false; n];
            let mut cover_d1 = vec![false; n.saturating_sub(1)];
            let mut cover_d2 = vec![false; n.saturating_sub(2)];
            let mut nonempty = 0usize;
            for s in kvv.first_span()..=kvv.last_span() {
                let ks = kvv.knots();
                if ks[s] >= ks[s + 1] {
                    continue; // empty span: no cell is emitted for it
                }
                nonempty += 1;
                let span = kvv.span_at(0.5 * (ks[s] + ks[s + 1]));
                assert_eq!(span.index(), s, "span lookup disagreed at p={p} m={m}");
                for i in span.window() {
                    if let Some(c) = cover_val.get_mut(i) {
                        *c = true;
                    }
                }
                for i in span.first_derived_window() {
                    if let Some(c) = cover_d1.get_mut(i) {
                        *c = true;
                    }
                }
                if let Some(w2) = span.derived_window(2) {
                    for i in w2 {
                        if let Some(c) = cover_d2.get_mut(i) {
                            *c = true;
                        }
                    }
                }
            }
            let miss_v = cover_val.iter().filter(|c| !**c).count();
            let miss_1 = cover_d1.iter().filter(|c| !**c).count();
            let miss_2 = if p >= 2 {
                cover_d2.iter().filter(|c| !**c).count()
            } else {
                0
            };
            rows.push(format!(
                "p={p} mult={m}: n={n} nonempty_spans={nonempty} uncovered val={miss_v} d1={miss_1} d2={miss_2}"
            ));
            assert_eq!(miss_v, 0, "value net not covered at p={p} mult={m}");
            assert_eq!(miss_1, 0, "d1 net not covered at p={p} mult={m}");
            assert_eq!(miss_2, 0, "d2 net not covered at p={p} mult={m}");
        }
    }
    for r in &rows {
        println!("[cert10r1] cover {r}");
    }
    // The reason this is TIGHT rather than comfortable: between two
    // consecutive nonempty spans separated by a knot of multiplicity
    // m, the order-k derived windows [s-p, s-k] are contiguous only
    // when m <= p - k + 1. At k = 2 that is m <= p - 1, which is
    // EXACTLY the C1 gate's ceiling. The gate is therefore load-bearing
    // for the fold's soundness, not only for C1.
    assert!(!rows.is_empty(), "no admissible multiplicities were exercised");
}

// ---------------------------------------------------------------
// CLAIM 2: the coarser grids the tighter bound buys (8.4x fewer cells
// on the quarter cylinder at delta_s = 4e-3). The re-baseline is only
// owned if the COARSER grid still delivers the certificate, so this
// measures the REALIZED chord error against delta_s.
//
// Everything below is re-derived independently of `mesh` (whose
// `NurbsFaceBound` is crate-private): the fold, the sup collapse and
// the split selection are re-implemented from the shipped formulas,
// which also cross-checks the PR body's pinned digits.
// ---------------------------------------------------------------

const ASPECT_CAP: f64 = 16.0;

#[derive(Clone, Copy, Debug, Default)]
struct FaceBound {
    muu: f64,
    muv: f64,
    mvv: f64,
    mu1: f64,
    mv1: f64,
}

/// `sqrt(sum_c sup_c^2)`, rounded up — `mesh::nurbs_cert::cell_component`
/// applied to `patch_bound::sq_norm`.
fn component(v: [geom_core::RingInterval; 3]) -> f64 {
    let sq = geom_brep::patch_bound::sq_norm(v);
    let hi = sq.hi();
    if hi == 0.0 { 0.0 } else { hi.sqrt().next_up() }
}

/// The fold: per-cell sups, maxed over the cells.
fn fold(n: &NurbsSurface<f64>) -> FaceBound {
    let cells = patch_cells(n).expect("patch_cells");
    let mut m = FaceBound::default();
    for c in &cells {
        m.muu = m.muu.max(component(c.s_uu));
        m.muv = m.muv.max(component(c.s_uv));
        m.mvv = m.mvv.max(component(c.s_vv));
        m.mu1 = m.mu1.max(component(c.s_u));
        m.mv1 = m.mv1.max(component(c.s_v));
    }
    m
}

/// `NurbsFaceBound::split_steps`, re-implemented.
fn split_steps(b: FaceBound, delta_s: f64) -> (f64, f64) {
    let (muu, muv, mvv) = (b.muu, b.muv, b.mvv);
    if muu == 0.0 && mvv == 0.0 && muv == 0.0 {
        return (f64::INFINITY, f64::INFINITY);
    }
    let rho = (b.mu1 > 0.0 && b.mv1 > 0.0 && b.mu1.is_finite() && b.mv1.is_finite())
        .then(|| b.mv1 / b.mu1)
        .filter(|r| r.is_finite() && *r > 0.0);
    let window = rho.map(|r| (r / ASPECT_CAP, r * ASPECT_CAP));
    let t = if muu > 0.0 && mvv > 0.0 {
        let t_star = (mvv / muu).sqrt();
        match window {
            Some((lo, hi)) => t_star.clamp(lo, hi),
            None => t_star,
        }
    } else if muu == 0.0 && mvv > 0.0 {
        window.map_or(1.0, |(_, hi)| hi)
    } else if mvv == 0.0 && muu > 0.0 {
        window.map_or(1.0, |(lo, _)| lo)
    } else {
        rho.unwrap_or(1.0)
    };
    let q = muv.mul_add(2.0 * t, muu.mul_add(t.powi(2), mvv));
    let hv = (delta_s / q).sqrt();
    (t * hv, hv)
}

fn divisions(h: f64) -> usize {
    if h.is_finite() && h > 0.0 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let d = (1.0f64 / h).ceil().max(1.0) as usize;
        d
    } else {
        1
    }
}

/// The realized chord error of the grid: for every grid cell, both
/// triangles of the diagonal split, the max distance from the true
/// surface point to the triangle's linear interpolant, sampled
/// barycentrically.
fn realized_chord_error(n: &NurbsSurface<f64>, du: usize, dv: usize, per_tri: usize) -> f64 {
    let (hu, hv) = (1.0 / du as f64, 1.0 / dv as f64);
    let mut worst = 0.0f64;
    for a in 0..du {
        for b in 0..dv {
            let (u0, v0) = (a as f64 * hu, b as f64 * hv);
            let (u1, v1) = (u0 + hu, v0 + hv);
            let corners = [[u0, v0], [u1, v0], [u1, v1], [u0, v1]];
            for tri in [[0usize, 1, 2], [0, 2, 3]] {
                let c: Vec<[f64; 2]> = tri.iter().map(|k| corners[*k]).collect();
                let pts: Vec<Point3<f64>> = c.iter().map(|q| n.eval(q[0], q[1])).collect();
                for i in 0..=per_tri {
                    for j in 0..=(per_tri - i) {
                        let l0 = i as f64 / per_tri as f64;
                        let l1 = j as f64 / per_tri as f64;
                        let l2 = 1.0 - l0 - l1;
                        let u = l0 * c[0][0] + l1 * c[1][0] + l2 * c[2][0];
                        let v = l0 * c[0][1] + l1 * c[1][1] + l2 * c[2][1];
                        let s = n.eval(u, v);
                        let lin = [
                            l0 * pts[0].x + l1 * pts[1].x + l2 * pts[2].x,
                            l0 * pts[0].y + l1 * pts[1].y + l2 * pts[2].y,
                            l0 * pts[0].z + l1 * pts[1].z + l2 * pts[2].z,
                        ];
                        let d = ((s.x - lin[0]).powi(2)
                            + (s.y - lin[1]).powi(2)
                            + (s.z - lin[2]).powi(2))
                        .sqrt();
                        worst = worst.max(d);
                    }
                }
            }
        }
    }
    worst
}

#[test]
fn cert10r1_the_coarsened_grid_still_meets_its_chord_tolerance() {
    // The PR's own retired MAGNITUDE numbers for the quarter cylinder,
    // transcribed from the shipped `cert10_rational_grid_resizing`
    // harness, so the two readings are sized side by side.
    let mag = FaceBound {
        muu: 3.942_263_838_556_179_7,
        muv: 1.266_375_820_315_083_4,
        mvv: 4.250_461_439_678_581e-15,
        mu1: 1.758_098_729_671_621_3,
        mv1: 1.064_513_033_689_903,
    };
    let s = quarter_cylinder();
    let signed = fold(&s);
    println!(
        "[cert10r1] quarter_cylinder re-derived signed fold: muu={:.7e} muv={:.7e} mvv={:.7e} mu1={:.7e} mv1={:.7e}",
        signed.muu, signed.muv, signed.mvv, signed.mu1, signed.mv1
    );
    println!(
        "[cert10r1] ratios vs the PR's retired magnitude: muu={:.4} muv={:.4} mu1={:.4} mv1={:.4}",
        signed.muu / mag.muu,
        signed.muv / mag.muv,
        signed.mu1 / mag.mu1,
        signed.mv1 / mag.mv1
    );
    let mut failures = Vec::new();
    for delta_s in [1e-2f64, 4e-3, 1e-3] {
        for (tag, b) in [("mag", mag), ("signed", signed)] {
            let (hu, hv) = split_steps(b, delta_s);
            let (du, dv) = (divisions(hu), divisions(hv));
            // delta_s is a squared-deviation budget: the certificate is
            // Q/4 with Q in units of length, so the chord error the
            // grid promises is delta_s itself (same units the tour's
            // tess-lint compares in).
            let err = realized_chord_error(&s, du, dv, 6);
            println!(
                "[cert10r1] delta_s={delta_s:.0e} {tag:>6}: {du} x {dv} = {} cells, realized chord error = {err:.6e}, err/delta_s = {:.4}",
                du * dv,
                err / delta_s
            );
            if err > delta_s {
                failures.push(format!(
                    "delta_s={delta_s:e} {tag}: realized {err:e} EXCEEDS delta_s"
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "the coarsened grid does not deliver its chord tolerance: {failures:?}"
    );
}
