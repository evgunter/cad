//! CERT-10 R2 blinded-review probes (reviewer-local, branch
//! cert/10r2-probes — NOT for merge into the unit branch).
//!
//! 1. `probe1_*`: the signed quotient-rule reading is a genuine
//!    ENCLOSURE — per-cell, per-channel, SIGNED containment of densely
//!    sampled true partials on adversarial rational patches (extreme
//!    weight ratios, sign-alternating nets), plus a randomized sweep.
//! 2. `probe2_*`: the coarser grid the signed reading buys on the
//!    quarter cylinder still meets its chord tolerance at
//!    delta_s = 4e-3 (realized |S - Pi| measured against delta_s).
//! 3. `probe3_*`: the fold premise "cell windows COVER the net" at
//!    interior multiplicity p-1 (the case the unit's own randomized
//!    generator never reaches) — per-channel window-union hull must
//!    equal the whole-net hull, all five derivative nets.

use geom::NurbsSurface;
use geom_brep::patch_bound::{self, PatchCell};
use geom_core::Point3;
use geom_core::ring_interval::RingInterval;
use geom_core::spline::KnotVector;
use geom_core::spline::net::TensorNet;

fn contains(iv: RingInterval, x: f64) -> bool {
    iv.lo() <= x && x <= iv.hi()
}

/// Dense signed-containment check with two ledgers:
/// - the NORM claim (what the tessellation consumes): |truth| must
///   never exceed the enclosure's magnitude sup — hard assert, since a
///   single sampled point above that bound falsifies the unit;
/// - the SIGNED-enclosure claim as the field docs state it: containment
///   violations are measured and must stay at insertion-rounding dust
///   scale (the rational arm encloses the REFINED f64 net's surface,
///   not the described one — found by this probe on quarter_cylinder,
///   where s_vv's z enclosure is [-4.1e-15, -3.4e-15] but true
///   S_vv == 0).
fn assert_cells_enclose(name: &str, s: &NurbsSurface<f64>, cells: &[PatchCell], m: usize) {
    let mut checked = 0usize;
    let mut worst_gap = 0.0f64;
    let mut worst_rel = 0.0f64;
    for c in cells {
        for iu in 0..=m {
            for iv in 0..=m {
                // Strictly interior samples: ders() at a knot line hulls
                // adjacent spans, which is not this cell's claim.
                let fu = (iu as f64 + 0.5) / (m as f64 + 1.0);
                let fv = (iv as f64 + 0.5) / (m as f64 + 1.0);
                let u = c.u.0 + (c.u.1 - c.u.0) * fu;
                let v = c.v.0 + (c.v.1 - c.v.0) * fv;
                let jet = s.ders(u, v);
                let per: [(&str, [f64; 3], [RingInterval; 3]); 5] = [
                    ("s_u", [jet.du.x, jet.du.y, jet.du.z], c.s_u),
                    ("s_v", [jet.dv.x, jet.dv.y, jet.dv.z], c.s_v),
                    ("s_uu", [jet.duu.x, jet.duu.y, jet.duu.z], c.s_uu),
                    ("s_uv", [jet.duv.x, jet.duv.y, jet.duv.z], c.s_uv),
                    ("s_vv", [jet.dvv.x, jet.dvv.y, jet.dvv.z], c.s_vv),
                ];
                for (what, truth, enc) in per {
                    for k in 0..3 {
                        checked += 1;
                        let sup = enc[k].lo().abs().max(enc[k].hi().abs());
                        assert!(
                            truth[k].abs() <= sup,
                            "{name}: |{what}[{k}]| = {:.17e} EXCEEDS the magnitude sup \
                             {:.17e} at (u,v)=({u},{v}), cell u{:?} v{:?} — the bound \
                             is wrong, not merely offset",
                            truth[k].abs(),
                            sup,
                            c.u,
                            c.v
                        );
                        if !contains(enc[k], truth[k]) {
                            let gap = if truth[k] < enc[k].lo() {
                                enc[k].lo() - truth[k]
                            } else {
                                truth[k] - enc[k].hi()
                            };
                            let scale = 1.0 + truth[k].abs() + sup;
                            worst_gap = worst_gap.max(gap);
                            worst_rel = worst_rel.max(gap / scale);
                        }
                    }
                }
            }
        }
    }
    println!(
        "{name}: {} cells, {checked} samples; worst signed-containment gap \
         {worst_gap:.3e} (rel {worst_rel:.3e})",
        cells.len()
    );
    // Dust scale: the documented f64/insertion-rounding slack lives
    // well under 1e-12 relative; anything larger is a real hole.
    assert!(
        worst_rel < 1e-12,
        "{name}: signed containment violated beyond rounding dust (rel {worst_rel:.3e})"
    );
}

/// Extreme weight ratios (1e-4..1e4) on a sign-alternating net —
/// exactly where a wrong-but-tighter signed reading would hide.
fn adversarial_extreme(flip: bool) -> NurbsSurface<f64> {
    let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.35, 1.0, 1.0, 1.0], 2).unwrap();
    let kv_v = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
    let wtab = [1e-4, 3.0, 1e4, 0.02, 47.0, 1e-3];
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for i in 0..nu {
        for j in 0..nv {
            let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
            let amp = if flip { 5.0 } else { 0.7 };
            let (x, y) = (i as f64 * 0.6, j as f64 * 0.4);
            control.push(Point3::new(
                x + sign * 0.05,
                y - sign * 0.03,
                sign * amp * (1.0 + 0.3 * x * y),
            ));
            weights.push(wtab[(2 * i + 3 * j) % 6]);
        }
    }
    NurbsSurface::new(kv_u, kv_v, control, weights).unwrap()
}

#[test]
fn probe1_signed_reading_encloses_adversarial_extremes() {
    for (name, s) in [
        ("extreme_alt", adversarial_extreme(false)),
        ("extreme_alt_big", adversarial_extreme(true)),
    ] {
        let cells = patch_bound::patch_cells(&s).expect("covered");
        assert_cells_enclose(name, &s, &cells, 6);
    }
}

/// Small deterministic LCG so the sweep needs no test_utils dependency.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0
    }
    fn f(&mut self, lo: f64, hi: f64) -> f64 {
        let x = (self.next() >> 11) as f64 / (1u64 << 53) as f64;
        lo + (hi - lo) * x
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

#[test]
fn probe1_signed_reading_encloses_random_rational_sweep() {
    let mut rng = Lcg(0xC10_5EED);
    let mut done = 0usize;
    for _trial in 0..40 {
        let pu = 1 + rng.below(3);
        let pv = 1 + rng.below(3);
        let mk = |r: &mut Lcg, p: usize| {
            let spans = 1 + r.below(2);
            let mut k = vec![0.0; p + 1];
            for i in 1..spans {
                k.push(i as f64 / spans as f64);
            }
            k.extend(vec![1.0; p + 1]);
            KnotVector::clamped(k, p).unwrap()
        };
        let kv_u = mk(&mut rng, pu);
        let kv_v = mk(&mut rng, pv);
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for _ in 0..nu * nv {
            control.push(Point3::new(
                rng.f(-5.0, 5.0),
                rng.f(-5.0, 5.0),
                rng.f(-5.0, 5.0),
            ));
            weights.push(10f64.powf(rng.f(-3.0, 3.0)));
        }
        let s = NurbsSurface::new(kv_u, kv_v, control, weights).unwrap();
        let Ok(cells) = patch_bound::patch_cells(&s) else {
            continue;
        };
        assert_cells_enclose("rand", &s, &cells, 2);
        done += 1;
    }
    assert!(done >= 20, "sweep degenerated: only {done} covered trials");
}

/// The unit's own quarter cylinder (fixture reproduced from
/// nurbs_cert.rs) — the face the 11x muv tightening is claimed on.
fn quarter_cylinder() -> NurbsSurface<f64> {
    let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv_v = KnotVector::unit_segment(1);
    let w = core::f64::consts::FRAC_1_SQRT_2;
    let arc = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let mut control = Vec::new();
    for (x, y) in arc {
        for z in [0.0, 1.0] {
            control.push(Point3::new(x, y, z));
        }
    }
    let weights = vec![1.0, 1.0, w, w, 1.0, 1.0];
    NurbsSurface::new(kv_u, kv_v, control, weights).unwrap()
}

#[test]
fn probe1_quarter_cylinder_signed_cells_enclose_densely() {
    let s = quarter_cylinder();
    let cells = patch_bound::patch_cells(&s).expect("covered");
    assert_cells_enclose("quarter_cylinder", &s, &cells, 10);
}

/// The face-level fold read the tessellation consumes: sup ||S_kl||
/// sampled densely must stay under sqrt(max over cells of sq hi).
#[test]
fn probe1_quarter_cylinder_fold_norms_dominate() {
    let s = quarter_cylinder();
    let cells = patch_bound::patch_cells(&s).expect("covered");
    let comp = |f: fn(&PatchCell) -> [RingInterval; 3]| -> f64 {
        cells
            .iter()
            .map(|c| patch_bound::sq_norm(f(c)).hi())
            .fold(0.0f64, f64::max)
            .sqrt()
            .next_up()
    };
    let (muu, muv, mvv) = (
        comp(|c| c.s_uu),
        comp(|c| c.s_uv),
        comp(|c| c.s_vv),
    );
    let n = 700usize;
    let (mut wuu, mut wuv, mut wvv) = (0.0f64, 0.0f64, 0.0f64);
    for i in 0..=n {
        for j in 0..=n {
            let u = i as f64 / n as f64;
            let v = j as f64 / n as f64;
            let jet = s.ders(u, v);
            wuu = wuu.max(jet.duu.norm());
            wuv = wuv.max(jet.duv.norm());
            wvv = wvv.max(jet.dvv.norm());
        }
    }
    println!(
        "qc fold bounds: muu {muu:.6e} muv {muv:.6e} mvv {mvv:.6e}; \
         sampled {wuu:.6e} {wuv:.6e} {wvv:.6e}"
    );
    assert!(wuu <= muu && wuv <= muv && wvv <= mvv, "sampled escapes fold bound");
    // The PR body's claimed signed figures for this face.
    assert!((muu - 3.030539).abs() < 2e-6 || muu < 3.94, "muu scale sanity");
}

/// Claim 2: the coarser grid (48 x 2 at delta_s = 4e-3, the PR's
/// re-sizing table) still delivers the chord certificate: realized
/// max |S - Pi| over each grid triangle must stay under delta_s.
#[test]
fn probe2_coarser_grid_still_meets_chord_tolerance() {
    let s = quarter_cylinder();
    let delta_s = 4e-3f64;
    let (nu, nv) = (48usize, 2usize);
    let at = |i: usize, j: usize| -> [f64; 2] {
        [i as f64 / nu as f64, j as f64 / nv as f64]
    };
    let m = 12usize;
    let mut worst = 0.0f64;
    for i in 0..nu {
        for j in 0..nv {
            let (a, b, c, d) = (at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1));
            for uv in [[a, b, c], [a, c, d]] {
                let p: Vec<Point3<f64>> = uv.iter().map(|w| s.eval(w[0], w[1])).collect();
                for ba in 0..=m {
                    for bb in 0..=(m - ba) {
                        let (b0, b1) = (ba as f64 / m as f64, bb as f64 / m as f64);
                        let b2 = 1.0 - b0 - b1;
                        let (u, v) = (
                            b0 * uv[0][0] + b1 * uv[1][0] + b2 * uv[2][0],
                            b0 * uv[0][1] + b1 * uv[1][1] + b2 * uv[2][1],
                        );
                        let sv = s.eval(u, v);
                        let pi = Point3::new(
                            b0 * p[0].x + b1 * p[1].x + b2 * p[2].x,
                            b0 * p[0].y + b1 * p[1].y + b2 * p[2].y,
                            b0 * p[0].z + b1 * p[1].z + b2 * p[2].z,
                        );
                        let dev = ((sv.x - pi.x).powi(2)
                            + (sv.y - pi.y).powi(2)
                            + (sv.z - pi.z).powi(2))
                        .sqrt();
                        worst = worst.max(dev);
                    }
                }
            }
        }
    }
    println!("probe2: realized worst chord dev {worst:.6e} vs delta_s {delta_s:.0e}");
    assert!(
        worst <= delta_s,
        "the coarser grid broke the chord tolerance: {worst:.6e} > {delta_s:.0e}"
    );
}

/// Claim 3's coverage premise, at the multiplicities the unit's own
/// generator never reaches: per channel and per derivative net, the
/// hull over the UNION of cell windows must equal the whole-net hull.
#[test]
fn probe3_cell_windows_cover_the_net_at_multiplicity_p_minus_one() {
    for (pu, mu) in [(2usize, 1usize), (3, 2), (3, 1)] {
        let mut ku = vec![0.0; pu + 1];
        ku.extend(std::iter::repeat(0.5).take(mu));
        ku.extend(vec![1.0; pu + 1]);
        let kv_u = KnotVector::clamped(ku, pu).unwrap();
        let kv_v = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        // A staggered-peak net: extremes pushed to different corners.
        let base = TensorNet::from_fn(nu, nv, |i, j| {
            let x = (i as f64 * 1.3).sin() * 7.0 - (j as f64 * 0.9).cos() * 5.0
                + if (i, j) == (0, 0) { 40.0 } else { 0.0 }
                + if (i, j) == (nu - 1, nv - 1) { -33.0 } else { 0.0 };
            RingInterval::point(x)
        });
        let kv_u1 = patch_bound::derived_knots(&kv_u).unwrap();
        let kv_v1 = patch_bound::derived_knots(&kv_v).unwrap();
        let d10 = base.diff_u_knots(&kv_u);
        let d01 = base.diff_v_knots(&kv_v);
        let d11 = d10.diff_v_knots(&kv_v);
        let d20 = d10.diff_u_knots(&kv_u1);
        let d02 = d01.diff_v_knots(&kv_v1);
        // Union-of-window hull per net, over nonempty span pairs.
        let mut hulls: [Option<RingInterval>; 5] = [None; 5];
        for su in kv_u.first_span()..=kv_u.last_span() {
            if !kv_u.span_is_nonempty(su) {
                continue;
            }
            let span_u = kv_u.span(su).unwrap();
            for sv in kv_v.first_span()..=kv_v.last_span() {
                if !kv_v.span_is_nonempty(sv) {
                    continue;
                }
                let span_v = kv_v.span(sv).unwrap();
                let w_uval = span_u.window();
                let w_vval = span_v.window();
                let w_ud1 = span_u.first_derived_window();
                let w_vd1 = span_v.first_derived_window();
                let reads = [
                    d10.window_hull(&w_ud1, &w_vval),
                    d01.window_hull(&w_uval, &w_vd1),
                    d11.window_hull(&w_ud1, &w_vd1),
                    span_u
                        .derived_window(2)
                        .map_or(RingInterval::zero(), |w2| d20.window_hull(&w2, &w_vval)),
                    span_v
                        .derived_window(2)
                        .map_or(RingInterval::zero(), |w2| d02.window_hull(&w_uval, &w2)),
                ];
                for (slot, h) in hulls.iter_mut().zip(reads) {
                    *slot = Some(match *slot {
                        None => h,
                        Some(a) => RingInterval::hull(a, h),
                    });
                }
            }
        }
        for (idx, (net, name)) in [
            (&d10, "d10"),
            (&d01, "d01"),
            (&d11, "d11"),
            (&d20, "d20"),
            (&d02, "d02"),
        ]
        .into_iter()
        .enumerate()
        {
            let whole = net.hull();
            let unioned = hulls[idx].unwrap();
            assert!(
                unioned.lo() <= whole.lo() && whole.hi() <= unioned.hi(),
                "p={pu} mult={mu} {name}: window union [{:.17e},{:.17e}] does NOT cover \
                 the whole net [{:.17e},{:.17e}] — a coefficient is in no cell window",
                unioned.lo(),
                unioned.hi(),
                whole.lo(),
                whole.hi()
            );
        }
        println!("p={pu} mult={mu}: all five nets covered by cell windows");
    }
}
