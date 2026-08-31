//! CERT-5 R2 adversarial probes: knot-aligned composite cells and the
//! `w`-uniform-in-v exact arm.
//!
//! **Adopted into the unit by merge, authorship kept.** Unchanged
//! apart from rustfmt. The sliver rows here are the ones that found
//! the hairline-cell hazard; two of the three now certify or refuse
//! honestly, and the third (`knot_one_ulp_from_a_block_edge`) still
//! carries a large width for a reason that is NOT this unit's cut
//! rule — see `refine_dir`'s exact-equality insertion guard, measured
//! and reported rather than changed here.
//!
//! Every probe drives the PUBLIC props door `nurbs_patch_face` and
//! checks the returned bracket against an INDEPENDENT plain-f64
//! Cox-de-Boor + composite Gauss-Legendre oracle. A bracket that
//! EXCLUDES the truth is the failure; a typed refusal is allowed.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_range_loop,
    clippy::too_many_arguments
)]

use geom_brep::props::PropsError;
use geom_brep::props::quad::nurbs_patch_face;
use geom_core::Tol;
use geom_core::spline::KnotVector;
use geom_core::{Band, RingInterval};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

// ---------- independent oracle (no kernel spline code) ----------

fn basis(knots: &[f64], degree: usize, ncp: usize, t: f64) -> Vec<f64> {
    let n = knots.len() - 1;
    let mut nn = vec![0.0f64; n];
    let d1 = knots[n - degree];
    for i in 0..n {
        let inside = if t >= d1 {
            knots[i] < knots[i + 1] && t >= knots[i] && t <= knots[i + 1]
        } else {
            knots[i] <= t && t < knots[i + 1]
        };
        if inside {
            nn[i] = 1.0;
        }
    }
    if t >= d1 {
        let mut last = None;
        for i in 0..n {
            if nn[i] == 1.0 {
                last = Some(i);
            }
        }
        for i in 0..n {
            nn[i] = 0.0;
        }
        if let Some(i) = last {
            nn[i] = 1.0;
        }
    }
    for pdeg in 1..=degree {
        let mut out = vec![0.0f64; n];
        for i in 0..(n - pdeg) {
            let da = knots[i + pdeg] - knots[i];
            let db = knots[i + pdeg + 1] - knots[i + 1];
            let a = if da > 0.0 {
                (t - knots[i]) / da * nn[i]
            } else {
                0.0
            };
            let b = if db > 0.0 {
                (knots[i + pdeg + 1] - t) / db * nn[i + 1]
            } else {
                0.0
            };
            out[i] = a + b;
        }
        nn = out;
    }
    nn.truncate(ncp);
    nn
}

fn dbasis(knots: &[f64], degree: usize, ncp: usize, t: f64) -> Vec<f64> {
    let n = knots.len() - 1;
    let mut low = basis(knots, degree - 1, n - (degree - 1), t);
    low.resize(n, 0.0);
    let mut out = vec![0.0f64; ncp];
    let pf = degree as f64;
    for (i, o) in out.iter_mut().enumerate() {
        let da = knots[i + degree] - knots[i];
        let db = knots[i + degree + 1] - knots[i + 1];
        let a = if da > 0.0 { pf / da * low[i] } else { 0.0 };
        let b = if db > 0.0 { pf / db * low[i + 1] } else { 0.0 };
        *o = a - b;
    }
    out
}

struct Oracle {
    ku: Vec<f64>,
    kv: Vec<f64>,
    du: usize,
    dv: usize,
    nu: usize,
    nv: usize,
    cp: Vec<[f64; 4]>,
}

impl Oracle {
    fn eval(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let bu = basis(&self.ku, self.du, self.nu, u);
        let bv = basis(&self.kv, self.dv, self.nv, v);
        let dbu = dbasis(&self.ku, self.du, self.nu, u);
        let dbv = dbasis(&self.kv, self.dv, self.nv, v);
        let (mut a, mut au, mut av) = ([0.0f64; 4], [0.0f64; 4], [0.0f64; 4]);
        for i in 0..self.nu {
            for j in 0..self.nv {
                let c = self.cp[i * self.nv + j];
                for k in 0..4 {
                    a[k] += bu[i] * bv[j] * c[k];
                    au[k] += dbu[i] * bv[j] * c[k];
                    av[k] += bu[i] * dbv[j] * c[k];
                }
            }
        }
        let w = a[3];
        let s = [a[0] / w, a[1] / w, a[2] / w];
        let su = core::array::from_fn(|k| (au[k] - s[k] * au[3]) / w);
        let sv = core::array::from_fn(|k| (av[k] - s[k] * av[3]) / w);
        (s, su, sv)
    }

    /// (flux, area) by composite 5-pt Gauss-Legendre, `cells` per span.
    fn dense(&self, cells: usize) -> (f64, f64) {
        const GX: [f64; 5] = [
            -0.906_179_845_938_664,
            -0.538_469_310_105_683,
            0.0,
            0.538_469_310_105_683,
            0.906_179_845_938_664,
        ];
        const GW: [f64; 5] = [
            0.236_926_885_056_189,
            0.478_628_670_499_366,
            0.568_888_888_888_889,
            0.478_628_670_499_366,
            0.236_926_885_056_189,
        ];
        let spans = |k: &[f64]| -> Vec<(f64, f64)> {
            k.windows(2)
                .filter(|w| w[1] > w[0])
                .map(|w| (w[0], w[1]))
                .collect()
        };
        let (mut flux, mut area) = (0.0, 0.0);
        for (ua, ub) in spans(&self.ku) {
            for (va, vb) in spans(&self.kv) {
                let hu = (ub - ua) / cells as f64;
                let hv = (vb - va) / cells as f64;
                for cu in 0..cells {
                    for cv in 0..cells {
                        let u0 = ua + cu as f64 * hu;
                        let v0 = va + cv as f64 * hv;
                        for a in 0..5 {
                            for b in 0..5 {
                                let u = u0 + hu * 0.5 * (1.0 + GX[a]);
                                let v = v0 + hv * 0.5 * (1.0 + GX[b]);
                                let (s, sud, svd) = self.eval(u, v);
                                let cx = [
                                    sud[1] * svd[2] - sud[2] * svd[1],
                                    sud[2] * svd[0] - sud[0] * svd[2],
                                    sud[0] * svd[1] - sud[1] * svd[0],
                                ];
                                let wq = GW[a] * GW[b] * hu * hv * 0.25;
                                flux += wq * (s[0] * cx[0] + s[1] * cx[1] + s[2] * cx[2]);
                                area += wq * (cx[0] * cx[0] + cx[1] * cx[1] + cx[2] * cx[2]).sqrt();
                            }
                        }
                    }
                }
            }
        }
        (flux, area)
    }
}

fn p(x: f64, y: f64, z: f64) -> [RingInterval; 3] {
    [
        RingInterval::point(x),
        RingInterval::point(y),
        RingInterval::point(z),
    ]
}

/// Drive the public door; assert SOUNDNESS against the oracle.
/// Returns (posture string, width if refused, seconds).
fn drive(
    name: &str,
    ku: &KnotVector,
    kv: &KnotVector,
    control: &[[RingInterval; 3]],
    weights: &[f64],
) -> (String, Option<f64>, f64) {
    let nu = ku.control_count();
    let nv = kv.control_count();
    assert_eq!(control.len(), nu * nv, "{name}: net shape");
    let oracle = Oracle {
        ku: ku.knots().to_vec(),
        kv: kv.knots().to_vec(),
        du: ku.degree(),
        dv: kv.degree(),
        nu,
        nv,
        cp: control
            .iter()
            .zip(weights)
            .map(|(c, w)| [c[0].lo() * w, c[1].lo() * w, c[2].lo() * w, *w])
            .collect(),
    };
    let (f1, a1) = oracle.dense(16);
    let (f2, a2) = oracle.dense(32);
    let converged =
        (f1 - f2).abs() < 1e-9 * (1.0 + f2.abs()) && (a1 - a2).abs() < 1e-9 * (1.0 + a2.abs());
    let (ua, ub) = ku.domain();
    let (va, vb) = kv.domain();
    let t0 = std::time::Instant::now();
    let out = nurbs_patch_face::<f64>(
        ku,
        kv,
        control,
        weights,
        (ua, ub, va, vb),
        0.0,
        0.0,
        Tol::witness().get().eps,
        band(),
    );
    let secs = t0.elapsed().as_secs_f64();
    match out {
        Ok(fb) => {
            println!(
                "R2 {name}: CERTIFIED flux [{:.12e},{:.12e}] oracle {:.12e} | area \
                 [{:.12e},{:.12e}] oracle {:.12e} | {:.2}s | oracle-converged {converged}",
                fb.flux.lo(),
                fb.flux.hi(),
                f2,
                fb.area.lo(),
                fb.area.hi(),
                a2,
                secs
            );
            if converged {
                assert!(
                    f2 >= fb.flux.lo() && f2 <= fb.flux.hi(),
                    "UNSOUND {name}: flux oracle {f2:.17e} outside [{:.17e},{:.17e}]",
                    fb.flux.lo(),
                    fb.flux.hi()
                );
                assert!(
                    a2 >= fb.area.lo() && a2 <= fb.area.hi(),
                    "UNSOUND {name}: area oracle {a2:.17e} outside [{:.17e},{:.17e}]",
                    fb.area.lo(),
                    fb.area.hi()
                );
            }
            ("CERTIFIED".to_string(), None, secs)
        }
        Err(PropsError::QuadratureBudget {
            width_len,
            target_len,
        }) => {
            println!(
                "R2 {name}: BUDGET width {width_len:.6e} vs target {target_len:.6e} \
                 ({:.1}x) | {secs:.2}s",
                width_len / target_len
            );
            assert!(width_len.is_finite() && width_len > target_len);
            ("BUDGET".to_string(), Some(width_len), secs)
        }
        Err(e) => {
            println!("R2 {name}: OTHER {e} | {secs:.2}s");
            (format!("{e:?}"), None, secs)
        }
    }
}

// ---------- the carriers ----------

/// A rational bi-quadratic "tile": a lifted, bulged patch over the
/// unit square with a genuine z-relief, so flux and area are both
/// nontrivial. `wf` supplies the weight at grid node (i, j).
fn tile(
    ku: &KnotVector,
    kv: &KnotVector,
    wf: &dyn Fn(usize, usize) -> f64,
) -> (Vec<[RingInterval; 3]>, Vec<f64>) {
    let (nu, nv) = (ku.control_count(), kv.control_count());
    let mut cp = Vec::with_capacity(nu * nv);
    let mut ws = Vec::with_capacity(nu * nv);
    for i in 0..nu {
        for j in 0..nv {
            let x = i as f64 / (nu - 1) as f64;
            let y = j as f64 / (nv - 1) as f64;
            // A relief that is NOT a ruled surface in either direction.
            let z = 0.35 * (x - 0.5) * (x - 0.5) + 0.25 * y * y + 0.30 * x * y;
            cp.push(p(2.0 * x, 1.5 * y, 1.0 + z));
            ws.push(wf(i, j));
        }
    }
    (cp, ws)
}

/// Interior knots at generic (non-dyadic) parameters, degree 2.
fn kv_offgrid_deg2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.31, 0.63, 1.0, 1.0, 1.0], 2).unwrap()
}

fn kv_offgrid_deg2_v() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.27, 0.71, 1.0, 1.0, 1.0], 2).unwrap()
}

/// E2E-1 — the brief's required carrier: MY OWN rational patch with
/// off-grid interior knots in BOTH directions, weights varying in both
/// directions (so the composite arm, not the exact arm).
#[test]
fn e2e_offgrid_knots_in_both_directions_composite_arm() {
    let (ku, kv) = (kv_offgrid_deg2(), kv_offgrid_deg2_v());
    let (cp, ws) = tile(&ku, &kv, &|i, j| 1.0 + 0.20 * i as f64 + 0.13 * j as f64);
    drive("e2e-1 biknot composite", &ku, &kv, &cp, &ws);
}

/// E2E-2 — the SAME geometry with weights exactly constant along v:
/// the `w`-uniform-in-v EXACT arm. Its enclosure must still contain
/// the brute-force reference.
#[test]
fn e2e_offgrid_knots_exact_arm_contains_reference() {
    let (ku, kv) = (kv_offgrid_deg2(), kv_offgrid_deg2_v());
    let (cp, ws) = tile(&ku, &kv, &|i, _| 1.0 + 0.20 * i as f64);
    drive("e2e-2 biknot EXACT arm", &ku, &kv, &cp, &ws);
}

/// E2E-3 — the same net with ONE weight moved by a single ulp: the
/// eligibility test is exact f64 equality, so this must fall to the
/// composite arm and must STILL be sound.
#[test]
fn nearly_uniform_weights_take_the_composite_arm_and_stay_sound() {
    let (ku, kv) = (kv_offgrid_deg2(), kv_offgrid_deg2_v());
    let (cp, mut ws) = tile(&ku, &kv, &|i, _| 1.0 + 0.20 * i as f64);
    let nv = kv.control_count();
    // perturb the LAST v entry of the middle u row by one ulp
    let idx = 2 * nv + (nv - 1);
    ws[idx] = f64::from_bits(ws[idx].to_bits() + 1);
    drive("e2e-3 1-ulp non-uniform", &ku, &kv, &cp, &ws);
}

/// The brief's C0 attack: an interior v knot of multiplicity EQUAL to
/// the degree at an off-grid parameter — the surface is only C0 there
/// and `S_v` genuinely jumps. Is a knot-ON-boundary cell still a
/// smoothness island on BOTH sides?
#[test]
fn c0_multiplicity_to_degree_knot_offgrid() {
    let ku = kv_offgrid_deg2();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.37, 0.37, 0.79, 1.0, 1.0, 1.0], 2).unwrap();
    let (cp, ws) = tile(&ku, &kv, &|i, j| 1.0 + 0.20 * i as f64 + 0.13 * j as f64);
    drive("c0-kink deg-mult v knot", &ku, &kv, &cp, &ws);
}

/// C0 kink AND the exact arm together: multiplicity-to-degree v knot
/// with v-uniform weights, so the exact per-span Newton-Cotes runs
/// across a genuinely discontinuous `S_v`.
#[test]
fn c0_multiplicity_knot_under_the_exact_arm() {
    let ku = kv_offgrid_deg2();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.37, 0.37, 0.79, 1.0, 1.0, 1.0], 2).unwrap();
    let (cp, ws) = tile(&ku, &kv, &|i, _| 1.0 + 0.20 * i as f64);
    drive("c0-kink EXACT arm", &ku, &kv, &cp, &ws);
}

/// The sliver attack: an interior v knot one ulp above a coarse BLOCK
/// edge (`QUAD2_HULL_BLOCKS = 8`), so two cut points sit one ulp apart
/// and the cell between them is one ulp wide.
#[test]
fn knot_one_ulp_from_a_block_edge() {
    let ku = kv_offgrid_deg2();
    // block edge 3/8 of [0,1]
    let edge: f64 = 3.0 / 8.0;
    let k = f64::from_bits(edge.to_bits() + 1);
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, k, 0.71, 1.0, 1.0, 1.0], 2).unwrap();
    let (cp, ws) = tile(&ku, &kv, &|i, j| 1.0 + 0.20 * i as f64 + 0.13 * j as f64);
    drive("sliver: knot 1 ulp above block edge", &ku, &kv, &cp, &ws);
}

/// The same sliver, one ulp BELOW the block edge, and under the exact
/// arm, where the whole v integral of that cell is taken through the
/// span located by the cell's own midpoint.
#[test]
fn knot_one_ulp_below_a_block_edge_exact_arm() {
    let ku = kv_offgrid_deg2();
    let edge: f64 = 3.0 / 8.0;
    let k = f64::from_bits(edge.to_bits() - 1);
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, k, 0.71, 1.0, 1.0, 1.0], 2).unwrap();
    let (cp, ws) = tile(&ku, &kv, &|i, _| 1.0 + 0.20 * i as f64);
    drive("sliver-below EXACT arm", &ku, &kv, &cp, &ws);
}

/// A knot one ulp inside the trim rectangle's own edge.
#[test]
fn knot_one_ulp_inside_the_rectangle_edge() {
    let hi = f64::from_bits(1.0f64.to_bits() - 1);
    let lo = f64::from_bits(0.0f64.to_bits() + 1);
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, lo, hi, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = kv_offgrid_deg2_v();
    let (cp, ws) = tile(&ku, &kv, &|i, j| 1.0 + 0.20 * i as f64 + 0.13 * j as f64);
    drive("knot 1 ulp inside rect edge", &ku, &kv, &cp, &ws);
}

/// Cost: the cut list is `pieces + blocks + knots` per axis and the
/// coarse hull grid is now `(blocks + knots)²` blocks, so a
/// heavily-knotted patch pays quadratically in KNOT COUNT for
/// something the PR body describes as "built once, before the rounds".
#[test]
fn many_offgrid_knots_cost() {
    let mk = |seed: f64| -> KnotVector {
        let mut k = vec![0.0, 0.0, 0.0];
        for i in 1..=12 {
            // generic, strictly increasing, off any dyadic grid
            k.push((i as f64 + seed) / 13.37);
        }
        k.extend([1.0, 1.0, 1.0]);
        KnotVector::clamped(k, 2).unwrap()
    };
    let (ku, kv) = (mk(0.11), mk(0.29));
    let (cp, ws) = tile(&ku, &kv, &|i, j| 1.0 + 0.05 * i as f64 + 0.03 * j as f64);
    let (_, _, secs) = drive("12+12 off-grid knots", &ku, &kv, &cp, &ws);
    println!("R2 COST: 12+12 interior knots each axis took {secs:.2}s");
}

/// The same shape with the knots on the DYADIC grid, for the cost
/// contrast (same cell count arithmetic, no extra cuts).
#[test]
fn many_dyadic_knots_cost() {
    let mk = || -> KnotVector {
        let mut k = vec![0.0, 0.0, 0.0];
        for i in 1..=12 {
            k.push(i as f64 / 16.0);
        }
        k.extend([1.0, 1.0, 1.0]);
        KnotVector::clamped(k, 2).unwrap()
    };
    let (ku, kv) = (mk(), mk());
    let (cp, ws) = tile(&ku, &kv, &|i, j| 1.0 + 0.05 * i as f64 + 0.03 * j as f64);
    let (_, _, secs) = drive("12+12 dyadic knots", &ku, &kv, &cp, &ws);
    println!("R2 COST: 12+12 dyadic knots took {secs:.2}s");
}

/// **The isolation control for the sliver finding.** One interior v
/// knot, placed at a graded distance from the coarse block edge 3/8.
/// The geometry is the SAME surface class at every gap; only the
/// distance from a cut point the engine adds by itself changes.
#[test]
fn width_versus_gap_from_a_block_edge() {
    let edge: f64 = 3.0 / 8.0;
    let gaps: [(&str, f64); 6] = [
        ("1 ulp", f64::from_bits(edge.to_bits() + 1) - edge),
        ("1e-15", 1e-15),
        ("1e-12", 1e-12),
        ("1e-9", 1e-9),
        ("1e-6", 1e-6),
        ("1e-3", 1e-3),
    ];
    let ku = kv_offgrid_deg2();
    for (label, g) in gaps {
        let k = edge + g;
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, k, 0.71, 1.0, 1.0, 1.0], 2).unwrap();
        let (cp, ws) = tile(&ku, &kv, &|i, j| 1.0 + 0.20 * i as f64 + 0.13 * j as f64);
        let (post, w, secs) = drive(&format!("gap {label}"), &ku, &kv, &cp, &ws);
        println!("R2 GAPSCAN gap={label} -> {post} width={w:?} {secs:.1}s");
    }
    // And the far control: nowhere near a block edge.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.313, 0.71, 1.0, 1.0, 1.0], 2).unwrap();
    let (cp, ws) = tile(&ku, &kv, &|i, j| 1.0 + 0.20 * i as f64 + 0.13 * j as f64);
    let (post, w, _) = drive("far control 0.313", &ku, &kv, &cp, &ws);
    println!("R2 GAPSCAN far-control -> {post} width={w:?}");
}
