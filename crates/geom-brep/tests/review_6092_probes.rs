//! Review probes for PR 6092 (round-zero schedules per knot span).
//!
//! A degree-(2, 2) patch whose interior knots are all DOUBLE is C⁰ at
//! every joint, so it is exactly the union of its one-span Bézier
//! tiles. Under the per-span schedule the whole patch and its tiles are
//! read on the same cells, so the whole patch's enclosure at a round is
//! the tiles' sum to rounding, and both hold an independent oracle: a
//! Bézier evaluator of this file's own and five-point Gauss–Legendre
//! per span, never the crate's.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_range_loop,
    clippy::cast_precision_loss
)]

use crate::shared::patch::patch_net;
use crate::shared::ring::p3 as p;
use geom_brep::props::quad::{LaneEnd, LaneRun, RoundWindow, nurbs_patch_face_rounds};
use geom_core::interval::certification::Certification;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval};

/// A C⁰ biquadratic patch: span ends per axis, and the
/// `(2·su + 1) × (2·sv + 1)` net with its weights.
struct C0Patch {
    eu: Vec<f64>,
    ev: Vec<f64>,
    cp: Vec<[f64; 3]>,
    w: Vec<f64>,
}

fn c0_knots(ends: &[f64]) -> KnotVector {
    let mut k = vec![ends[0]; 3];
    for e in &ends[1..ends.len() - 1] {
        k.extend([*e, *e]);
    }
    k.extend([*ends.last().unwrap(); 3]);
    KnotVector::clamped(k, 2).unwrap()
}

impl C0Patch {
    fn nv(&self) -> usize {
        2 * (self.ev.len() - 1) + 1
    }

    /// The crate's view: the whole net, or the 3 × 3 tile of span cell
    /// `(i, j)` on its own clamped knots.
    fn run(&self, tile: Option<(usize, usize)>, window: RoundWindow) -> LaneRun {
        let nv = self.nv();
        let (eu, ev, rows, cols): (Vec<f64>, Vec<f64>, Vec<usize>, Vec<usize>) = match tile {
            None => (
                self.eu.clone(),
                self.ev.clone(),
                (0..2 * (self.eu.len() - 1) + 1).collect(),
                (0..nv).collect(),
            ),
            Some((i, j)) => (
                vec![self.eu[i], self.eu[i + 1]],
                vec![self.ev[j], self.ev[j + 1]],
                (2 * i..=2 * i + 2).collect(),
                (2 * j..=2 * j + 2).collect(),
            ),
        };
        let (ku, kv) = (c0_knots(&eu), c0_knots(&ev));
        let mut cp = Vec::new();
        let mut w = Vec::new();
        for &a in &rows {
            for &b in &cols {
                let c = self.cp[a * nv + b];
                cp.push(p(c[0], c[1], c[2]));
                w.push(self.w[a * nv + b]);
            }
        }
        let rect = (eu[0], *eu.last().unwrap(), ev[0], *ev.last().unwrap());
        nurbs_patch_face_rounds::<f64>(
            &patch_net(&ku, &kv, &cp, &w),
            rect,
            0.0,
            0.0,
            crate::shared::tol::band(),
            window,
            &mut |_, _| Ok(false),
        )
        .expect("the patch has an enclosure")
    }

    /// The oracle: `(∫∫ S·(S_u×S_v), ∫∫ |S_u×S_v|)` by Gauss–Legendre on
    /// `n × n` sub-cells of every span cell.
    fn oracle(&self, n: usize) -> (f64, f64) {
        let gx = [
            -0.906_179_845_938_664,
            -0.538_469_310_105_683,
            0.0,
            0.538_469_310_105_683,
            0.906_179_845_938_664,
        ];
        let gw = [
            0.236_926_885_056_189,
            0.478_628_670_499_366,
            0.568_888_888_888_889,
            0.478_628_670_499_366,
            0.236_926_885_056_189,
        ];
        let nv = self.nv();
        let b = |t: f64| [(1.0 - t) * (1.0 - t), 2.0 * t * (1.0 - t), t * t];
        let db = |t: f64| [-2.0 * (1.0 - t), 2.0 - 4.0 * t, 2.0 * t];
        let (mut flux, mut area) = (0.0, 0.0);
        for i in 0..self.eu.len() - 1 {
            for j in 0..self.ev.len() - 1 {
                let (ua, ub, va, vb) = (self.eu[i], self.eu[i + 1], self.ev[j], self.ev[j + 1]);
                let eval = |t: f64, s: f64| {
                    // homogeneous A, A_t, A_s (4-vectors), per unit t, s
                    let (bt, dbt, bs, dbs) = (b(t), db(t), b(s), db(s));
                    let mut h = [[0.0f64; 4]; 3];
                    for a in 0..3 {
                        for c in 0..3 {
                            let k = (2 * i + a) * nv + 2 * j + c;
                            let wt = self.w[k];
                            let q = [
                                self.cp[k][0] * wt,
                                self.cp[k][1] * wt,
                                self.cp[k][2] * wt,
                                wt,
                            ];
                            for d in 0..4 {
                                h[0][d] += bt[a] * bs[c] * q[d];
                                h[1][d] += dbt[a] * bs[c] * q[d] / (ub - ua);
                                h[2][d] += bt[a] * dbs[c] * q[d] / (vb - va);
                            }
                        }
                    }
                    let wv = h[0][3];
                    let s0: [f64; 3] = core::array::from_fn(|d| h[0][d] / wv);
                    let su: [f64; 3] = core::array::from_fn(|d| (h[1][d] - h[1][3] * s0[d]) / wv);
                    let sv: [f64; 3] = core::array::from_fn(|d| (h[2][d] - h[2][3] * s0[d]) / wv);
                    (s0, su, sv)
                };
                for su_ in 0..n {
                    for sv_ in 0..n {
                        let (t0, t1) = (su_ as f64 / n as f64, (su_ + 1) as f64 / n as f64);
                        let (s0_, s1_) = (sv_ as f64 / n as f64, (sv_ + 1) as f64 / n as f64);
                        for (x, wx) in gx.iter().zip(&gw) {
                            for (y, wy) in gx.iter().zip(&gw) {
                                let t = 0.5 * (t0 + t1) + 0.5 * (t1 - t0) * x;
                                let s = 0.5 * (s0_ + s1_) + 0.5 * (s1_ - s0_) * y;
                                let (s0, su, sv) = eval(t, s);
                                let c = [
                                    su[1] * sv[2] - su[2] * sv[1],
                                    su[2] * sv[0] - su[0] * sv[2],
                                    su[0] * sv[1] - su[1] * sv[0],
                                ];
                                let jac = 0.25
                                    * (t1 - t0)
                                    * (s1_ - s0_)
                                    * (ub - ua)
                                    * (vb - va)
                                    * wx
                                    * wy;
                                flux += jac * (s0[0] * c[0] + s0[1] * c[1] + s0[2] * c[2]);
                                area += jac * (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
                            }
                        }
                    }
                }
            }
        }
        (flux, area)
    }
}

/// A wavy C⁰ patch on unequal, non-dyadic spans, with weights
/// `w(i, j)` and the whole net shifted by `shift`.
fn wavy(eu: &[f64], ev: &[f64], weight: impl Fn(usize, usize) -> f64, shift: f64) -> C0Patch {
    let (nu, nv) = (2 * (eu.len() - 1) + 1, 2 * (ev.len() - 1) + 1);
    let mut cp = Vec::new();
    let mut w = Vec::new();
    for a in 0..nu {
        for b in 0..nv {
            let (x, y) = (a as f64 / (nu - 1) as f64, b as f64 / (nv - 1) as f64);
            cp.push([
                shift + 2.0 * x + 0.1 * y,
                shift + 1.5 * y,
                shift + 0.3 * (3.0 * x).sin() * (2.0 * y).cos() + 0.2,
            ]);
            w.push(weight(a, b));
        }
    }
    C0Patch {
        eu: eu.to_vec(),
        ev: ev.to_vec(),
        cp,
        w,
    }
}

/// The whole patch at `round` against its tiles' sum at the same round:
/// both hold the oracle, and the whole's widths are the tiles' summed
/// widths to `rel`.
fn whole_against_tiles(
    name: &str,
    patch: &C0Patch,
    round: usize,
    rel: f64,
    oracle_n: usize,
    area_same: bool,
) {
    let window = RoundWindow::at(round);
    let whole = patch.run(None, window);
    assert!(
        !matches!(whole.end, LaneEnd::Wall),
        "{name}: the whole patch walls at round {round}: {whole:?}"
    );
    let (mut flux, mut area) = (Interval::zero(), Interval::zero());
    for i in 0..patch.eu.len() - 1 {
        for j in 0..patch.ev.len() - 1 {
            let t = patch.run(Some((i, j)), window);
            assert_eq!(t.round, whole.round, "{name}: tile ({i},{j}) {t:?}");
            flux = flux + t.bounds.flux;
            area = area + t.bounds.area;
        }
    }
    let (f, a) = patch.oracle(oracle_n);
    let (f2, a2) = patch.oracle(2 * oracle_n);
    // The oracle's own error, bounded by its halving step; the
    // brackets are asked to hold the truth with that much room.
    let (ef, ea) = ((f - f2).abs(), (a - a2).abs());
    assert!(
        ef <= 1e-6 * f2.abs().max(1.0),
        "{name}: oracle flux {f} {f2}"
    );
    assert!(
        ea <= 1e-6 * a2.abs().max(1.0),
        "{name}: oracle area {a} {a2}"
    );
    let wd = |x: Interval| x.hi() - x.lo();
    println!(
        "6092 {name} r{round}: whole flux {:?} w {:.6e} | tiles w {:.6e} | oracle {f2:.15e}\n\
         \x20   whole area {:?} w {:.6e} | tiles w {:.6e} | oracle {a2:.15e}",
        whole.bounds.flux,
        wd(whole.bounds.flux),
        wd(flux),
        whole.bounds.area,
        wd(whole.bounds.area),
        wd(area),
    );
    for (what, x, truth, e) in [
        ("whole flux", whole.bounds.flux, f2, ef),
        ("tiles flux", flux, f2, ef),
        ("whole area", whole.bounds.area, a2, ea),
        ("tiles area", area, a2, ea),
    ] {
        assert!(
            x.lo() <= truth - e && truth + e <= x.hi(),
            "{name}: {what} {x:?} misses the oracle {truth} ± {e:e}"
        );
    }
    for (what, a, b, mag) in [
        ("flux", wd(whole.bounds.flux), wd(flux), f2.abs()),
        ("area", wd(whole.bounds.area), wd(area), a2),
    ] {
        if what == "area" && !area_same {
            continue;
        }
        assert!(
            (a - b).abs() <= rel * b + 1e-12 * mag,
            "{name}: the whole {what} width {a:e} is not the tiles' {b:e}"
        );
    }
}

const EU: [f64; 4] = [0.0, 0.13, 0.61, 1.0];
const EV: [f64; 4] = [0.2, 0.2001, 0.55, 3.7];

/// The polynomial lane's flux is the exact per-span rule; its area pass
/// is the per-span one. Both hold the oracle; the flux widths agree.
/// The area widths do NOT: see the ignored row below.
#[test]
fn a_polynomial_c0_patch_reads_as_its_tiles() {
    whole_against_tiles(
        "polynomial",
        &wavy(&EU, &EV, |_, _| 1.0, 0.0),
        0,
        1e-9,
        8,
        false,
    );
}

/// **Red at `ae1ba4fa`.** The polynomial lane's area pass hulls each
/// cell with `Collapse::Over` on the raw derivative grids, and a closed
/// cell `[x, k]` ending ON a C⁰ knot reads `span_of(k)`, the NEXT
/// span's coefficients (and the empty double-knot span's zero filler).
/// The whole patch's area width is about 1.1% wider than its tiles'
/// summed, so the two are not read alike although their cells are.
#[test]
#[ignore = "PR 6092 review finding: whole and tiled polynomial area widths differ"]
fn a_polynomial_c0_patchs_area_reads_as_its_tiles() {
    whole_against_tiles(
        "polynomial area",
        &wavy(&EU, &EV, |_, _| 1.0, 0.0),
        0,
        1e-9,
        8,
        true,
    );
}

#[test]
fn a_rational_c0_patch_reads_as_its_tiles() {
    let w = |a: usize, b: usize| 1.0 + 0.4 * ((a * 7 + b * 3) % 5) as f64;
    for round in [0, 2] {
        whole_against_tiles("rational", &wavy(&EU, &EV, w, 0.0), round, 1e-8, 8, true);
    }
}

#[test]
fn an_extreme_weight_c0_patch_reads_as_its_tiles() {
    let w = |a: usize, b: usize| if (a + b) % 2 == 0 { 1e-1 } else { 1e1 };
    whole_against_tiles(
        "extreme weights",
        &wavy(&EU, &EV, w, 0.0),
        1,
        1e-8,
        512,
        true,
    );
}

#[test]
fn a_translated_rational_c0_patch_reads_as_its_tiles() {
    let w = |a: usize, b: usize| 1.0 + 0.4 * ((a * 7 + b * 3) % 5) as f64;
    whole_against_tiles("translated 1e4", &wavy(&EU, &EV, w, 1e4), 1, 1e-8, 8, true);
}

/// The v-uniform weights take the exact-v arm.
#[test]
fn an_exact_v_c0_patch_reads_as_its_tiles() {
    let w = |a: usize, _: usize| 1.0 + 0.3 * (a % 3) as f64;
    whole_against_tiles("exact-v", &wavy(&EU, &EV, w, 0.0), 2, 1e-8, 8, true);
}
