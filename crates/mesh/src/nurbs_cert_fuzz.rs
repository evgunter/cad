//! `nurbs_cert`'s three randomized soundness sweeps, in a file of their own.
//!
//! A `test_utils::gated_to!` marker gates a whole file's module, so a fuzz row
//! sharing a file with pinned tests drags them along: gating `nurbs_cert` as a
//! whole would skip three dozen deterministic certificate pins to buy the
//! three rows below. Splitting the file is how that granularity is bought, and
//! the helpers these rows take are `pub(crate)` in `nurbs_cert::tests` rather
//! than copied here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/mesh/src/nurbs_cert.rs",
    "crates/mesh/src/curved.rs",
    "crates/mesh/src/tessellate.rs",
    "crates/geom-brep/src/patch_bound.rs",
    "crates/geom-core/src/ring_interval.rs",
    "crates/geom-core/src/spline/",
    "crates/geom/src/surfaces/",
    "crates/geom/src/surfaces.rs",
];

use crate::nurbs_cert::tests::{fold_bound, nurbs_face_bound, sample_worst, whole_net_bound};
use crate::nurbs_cert::*;
use geom::NurbsSurface;
use geom_core::Point3;
use geom_core::spline::KnotVector;
use test_utils::fuzz;
use topo::FaceKey;

/// **The split selection stays inside the certified ellipse and
/// the aspect window** — the TESS-SPLIT counterpart of the meter's
/// optimizer test, asserted on the ANSWER over random bounds,
/// degenerate corners included (exact-zero directions, degenerate
/// 3-D speeds).
#[test]
fn split_steps_stay_on_the_ellipse_and_inside_the_cap() {
    let mut rng = fuzz::start("nurbs_cert::split_steps_constraints");
    fn mag(r: &mut fuzz::Rng) -> f64 {
        if r.unit() < 0.2 {
            0.0
        } else {
            10.0f64.powf(r.range(-6.0, 4.0))
        }
    }
    for _ in 0..fuzz::scaled(500) {
        let delta_s = 10.0f64.powf(rng.range(-6.0, -1.0));
        let b = NurbsFaceBound {
            muu: mag(&mut rng),
            muv: mag(&mut rng),
            mvv: mag(&mut rng),
            mu1: mag(&mut rng),
            mv1: mag(&mut rng),
        };
        let s = b.split_steps(delta_s);
        // Ellipse membership, checked at a finite box (an
        // unconstrained ∞ step realizes as the box extent).
        let ext = 10.0f64.powf(rng.range(-2.0, 2.0));
        let (hu, hv) = (s.hu.min(ext), s.hv.min(ext));
        let q = b.muu * hu.powi(2) + 2.0 * b.muv * hu * hv + b.mvv * hv.powi(2);
        assert!(
            q <= delta_s * (1.0 + 1e-9),
            "chosen point violates the certificate: q={q:e} > {delta_s:e} for {b:?} — {}",
            fuzz::replay()
        );
        // Aspect-cap membership whenever the window exists and the
        // chosen steps are finite (the affine ∞ arm has no cell
        // shape to cap).
        if b.mu1 > 0.0 && b.mv1 > 0.0 && s.hu.is_finite() && s.hv.is_finite() {
            let aspect = (s.hu * b.mu1) / (s.hv * b.mv1);
            assert!(
                ((1.0 - 1e-9) / ASPECT_CAP..=ASPECT_CAP * (1.0 + 1e-9)).contains(&aspect),
                "3-D aspect {aspect:e} escapes the cap for {b:?} — {}",
                fuzz::replay()
            );
        }
    }
}

/// R1 randomized soundness sweep: random rational patches (degrees
/// 1-3, 1-3 spans, log-uniform weights 1e-2..1e2), dense-sampled true
/// second partials vs the certified sups. This sweep is the row that
/// KILLS the one mutation the rest of the suite missed (dropping the
/// `v0*w11` term from `suv` — the recentred-value x
/// mixed-weight-derivative cross term).
///
/// The trial count rides `CAD_FUZZ_EFFORT` and the seed varies per
/// run. The 61x61 `sample_worst` grid does NOT: it is the domination
/// check itself, not a sweep dimension.
#[test]
fn r1_random_rational_soundness_sweep() {
    let mut rng = fuzz::start("nurbs_cert::r1_random_rational_soundness");
    fn mk(r: &mut fuzz::Rng, p: usize) -> KnotVector {
        // Single-span at degree 1, for the reason `cert10`'s generator
        // states: a degree-1 direction admits no interior knot, so a
        // knotted one is refused by `nurbs_face_bound` and the trial is
        // thrown away. Measured 2026-09-11 before this line: 32% of
        // this sweep's trials, every one of them a `pu == 1` or
        // `pv == 1` draw.
        let spans = if p >= 2 { 1 + r.below(2) } else { 1 };
        let mut k = vec![0.0; p + 1];
        for i in 1..spans {
            #[allow(clippy::cast_precision_loss)]
            k.push(i as f64 / spans as f64);
        }
        k.extend(vec![1.0; p + 1]);
        KnotVector::clamped(k, p).unwrap()
    }
    let mut worst = 0.0f64;
    let mut compared = 0usize;
    // TRIALS are breadth; the 61x61 `sample_worst` grid below is the
    // per-trial falsification power and is deliberately NOT reduced —
    // it IS the domination check. With a varying seed, breadth is
    // what successive runs supply for free, so the trial count is the
    // honest lever here and the grid is not.
    let trials = fuzz::scaled(60);
    for trial in 0..trials {
        let pu = 1 + rng.below(3);
        let pv = 1 + rng.below(3);
        let kv_u = mk(&mut rng, pu);
        let kv_v = mk(&mut rng, pv);
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for _ in 0..nu * nv {
            control.push(Point3::new(
                rng.range(-2.0, 2.0),
                rng.range(-2.0, 2.0),
                rng.range(-2.0, 2.0),
            ));
            weights.push(10f64.powf(rng.range(-2.0, 2.0)));
        }
        let s = NurbsSurface::new(kv_u, kv_v, control, weights).unwrap();
        let Ok(b) = nurbs_face_bound(&s, FaceKey::default()) else {
            continue;
        };
        compared += 1;
        let (wuu, wuv, wvv) = sample_worst(&s, 60);
        let r = (wuu / b.muu).max(wuv / b.muv).max(wvv / b.mvv);
        worst = worst.max(r);
        assert!(
            wuv <= b.muv && wuu <= b.muu && wvv <= b.mvv,
            "UNSOUND at trial {trial}: ({wuu:.3e},{wuv:.3e},{wvv:.3e}) vs \
             ({:.3e},{:.3e},{:.3e}) — {}",
            b.muu,
            b.muv,
            b.mvv,
            fuzz::replay()
        );
    }
    println!("random sweep: worst truth/bound {worst:.6} over {compared} trials");
    // The breadth this row's floor is written against is the breadth it
    // ACTUALLY ran. A discarded trial lowers `worst` silently, so the
    // adversarial floor below would be satisfied by a smaller sweep than
    // the one the trial count promises — the same silence that let
    // `cert10`'s floor drift, in the shape a max takes rather than a
    // count.
    assert_eq!(
        compared,
        trials,
        "{} of {trials} trials produced no bound, so this sweep is narrower than \
         its floor assumes — {}",
        trials - compared,
        fuzz::replay()
    );
    // COVERAGE FLOOR: the sweep must keep producing cases where the
    // bound is genuinely tight, otherwise a slack bound would pass by
    // never being challenged. Verified to hold at the shipped trial
    // count; if a run ever trips it, RAISE the count rather than
    // lowering the threshold.
    assert!(
        worst > 0.5,
        "the sweep must stay adversarial (tight cases exist): worst \
         {worst:.6} — {}",
        fuzz::replay()
    );
}

/// **The tighter-or-equal claim, randomized.** The two pinned
/// fixtures show the gap exists and that a smooth net closes it;
/// this row asserts the INEQUALITY over a sweep of random integral
/// nets, which is the form the claim is actually made in. Integral
/// only, because the rational arm has always been a fold and has
/// no whole-net counterpart to be tighter than.
#[test]
fn cert10_the_fold_never_exceeds_the_whole_net_hull() {
    let mut rng = fuzz::start("nurbs_cert::cert10_fold_vs_whole_net");
    /// Interior multiplicities are drawn up to `p - 1`, the C¹
    /// gate's ceiling — NOT left at 1. That is where the fold's
    /// coverage premise is TIGHT: a cell window is
    /// `Span::window()` / `first_derived_window()` /
    /// `derived_window(2)`, and the claim that those windows COVER
    /// the derivative nets is what makes `max over cells == whole
    /// net` per channel. Raising a multiplicity shortens the
    /// derivative net and shifts every window; at `p - 1` the
    /// second-derivative net is at its shortest that
    /// `check_direction` still admits, which is exactly the case a
    /// multiplicity-1 generator never reaches.
    fn mk(r: &mut fuzz::Rng, p: usize) -> KnotVector {
        // A DEGREE-1 DIRECTION IS DRAWN SINGLE-SPAN, because it admits
        // no interior knot at all (`check_direction`'s Degree1Crease).
        // The old spelling said that in a comment and then drew one
        // anyway — `m` fell to 1 at `p == 1` instead of to 0 — so every
        // net with a knotted degree-1 direction was refused by
        // `whole_net_bound` and dropped by the `continue` below.
        // Measured 2026-09-11: that was 44% of all trials (every skip
        // this sweep took), and the floor below counted them as if they
        // had been compared.
        let spans = if p >= 2 { 1 + r.below(4) } else { 1 };
        let mut k = vec![0.0; p + 1];
        for i in 1..spans {
            #[allow(clippy::cast_precision_loss)]
            let t = i as f64 / spans as f64;
            let m = 1 + r.below(p - 1); // 1 ..= p - 1; p >= 2 here
            for _ in 0..m {
                k.push(t);
            }
        }
        k.extend(vec![1.0; p + 1]);
        KnotVector::clamped(k, p).unwrap()
    }
    let mut strict = 0usize;
    let mut compared = 0usize;
    let mut saw_high_mult = false;
    let trials = fuzz::scaled(60);
    for _ in 0..trials {
        let (pu, pv) = (1 + rng.below(3), 1 + rng.below(3));
        let kv_u = mk(&mut rng, pu);
        let kv_v = mk(&mut rng, pv);
        saw_high_mult |= kv_u.interior_knots().any(|(_, m)| m >= 2)
            || kv_v.interior_knots().any(|(_, m)| m >= 2);
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let control: Vec<Point3<f64>> = (0..nu * nv)
            .map(|_| {
                Point3::new(
                    rng.range(-4.0, 4.0),
                    rng.range(-4.0, 4.0),
                    rng.range(-4.0, 4.0),
                )
            })
            .collect();
        let w = vec![1.0; control.len()];
        let Ok(s) = NurbsSurface::new(kv_u, kv_v, control, w) else {
            continue;
        };
        let (Some(whole), Some(fold)) = (whole_net_bound(&s), fold_bound(&s)) else {
            continue;
        };
        compared += 1;
        for (what, f, w) in [
            ("muu", fold.muu, whole.muu),
            ("muv", fold.muv, whole.muv),
            ("mvv", fold.mvv, whole.mvv),
            ("mu1", fold.mu1, whole.mu1),
            ("mv1", fold.mv1, whole.mv1),
        ] {
            assert!(
                f <= w,
                "the fold EXCEEDED the whole-net hull on {what}: {f:.17e} > \
                 {w:.17e} — {}",
                fuzz::replay()
            );
            if f < w {
                strict += 1;
            }
        }
    }
    // COVERAGE FLOOR, three halves now. An inequality nothing ever
    // makes strict is a tautology; a sweep that never leaves
    // multiplicity 1 never challenges the coverage premise the
    // inequality rests on; and a sweep that DISCARDS most of its own
    // draws is not the sweep its floor is written against.
    //
    // THE DISCARD IS THE ONE THAT BIT. Every `continue` above is a
    // trial that contributes nothing to `strict` while still counting
    // toward `trials`, so the floor silently tightened as the discard
    // rate rose — and nothing measured that rate. At 44% discarded the
    // floor was asking for 60 strict comparisons out of the ~165 that
    // actually ran while reporting 300, and a bad draw crossed it
    // (`0xdcc78227f392d565`: 24 trials compared, 59 strict, reported as
    // "of 300"). Both are fixed here: the generator no longer draws
    // nets the bounds refuse, and `compared` is asserted rather than
    // assumed, so a future drift reds loudly instead of shrinking the
    // sample in silence.
    assert_eq!(
        compared,
        trials,
        "{} of {trials} trials produced no bound, so this sweep measured less than \
         it claims: the generator draws nets `whole_net_bound` or `fold_bound` \
         refuses, and the floor below is written against the full count — {}",
        trials - compared,
        fuzz::replay()
    );
    assert!(
        saw_high_mult,
        "the sweep never drew an interior multiplicity >= 2, so the fold's window \
         coverage went unchallenged where it is tight — {}",
        fuzz::replay()
    );
    // The denominator is `compared * 5`, the comparisons this run
    // PERFORMED — never `trials * 5`, which is what it would have
    // performed had nothing been discarded. They are equal only
    // because the assert above makes them so; spelling it from
    // `compared` keeps the number true if that ever changes.
    assert!(
        strict > trials,
        "the sweep must keep producing STRICT gaps: {strict} strict of {} \
         comparisons — {}",
        compared * 5,
        fuzz::replay()
    );
    println!(
        "cert10 fold-vs-whole-net: {strict} strict of {} comparisons",
        compared * 5
    );
}

// ======================================================================
// DIAGNOSTIC INSTRUMENTATION (tess/nurbs-bound-diag) — evidence, not a
// change proposed for main.
// ======================================================================
mod diag {
    use super::*;
    use geom_brep::patch_bound;

    const LABEL: &str = "nurbs_cert::r1_random_rational_soundness";

    fn label_hash(label: &str) -> u64 {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for b in label.as_bytes() {
            h ^= u64::from(*b);
            h = h.wrapping_mul(0x100_0000_01b3);
        }
        h
    }

    /// `DIAG_OLD_GEN=1` restores the generator as it stood before
    /// 47437f2b4 (2026-09-11): a span draw at EVERY degree. Instance A
    /// (2026-09-04) was drawn by that generator, so its seed replays
    /// only through it.
    fn old_gen() -> bool {
        std::env::var_os("DIAG_OLD_GEN").is_some()
    }

    fn mk(r: &mut fuzz::Rng, p: usize) -> KnotVector {
        let spans = if p >= 2 || old_gen() {
            1 + r.below(2)
        } else {
            1
        };
        let mut k = vec![0.0; p + 1];
        for i in 1..spans {
            k.push(i as f64 / spans as f64);
        }
        k.extend(vec![1.0; p + 1]);
        KnotVector::clamped(k, p).unwrap()
    }

    /// One trial's draw — byte-for-byte the sweep's generator.
    fn draw(rng: &mut fuzz::Rng) -> NurbsSurface<f64> {
        let mut pu = 1 + rng.below(3);
        let mut pv = 1 + rng.below(3);
        // `DIAG_BILINEAR=1`: a focused census of the degree-(1,1) stratum
        // both recorded reds fall in (NOT the sweep's stream any more).
        if std::env::var_os("DIAG_BILINEAR").is_some() {
            pu = 1;
            pv = 1;
        }
        let kv_u = mk(rng, pu);
        let kv_v = mk(rng, pv);
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for _ in 0..nu * nv {
            control.push(Point3::new(
                rng.range(-2.0, 2.0),
                rng.range(-2.0, 2.0),
                rng.range(-2.0, 2.0),
            ));
            weights.push(10f64.powf(rng.range(-2.0, 2.0)));
        }
        NurbsSurface::new(kv_u, kv_v, control, weights).unwrap()
    }

    fn hx(x: f64) -> String {
        format!("{:016x}", x.to_bits())
    }
    fn hxs(xs: impl IntoIterator<Item = f64>) -> String {
        xs.into_iter().map(hx).collect::<Vec<_>>().join(",")
    }

    fn dump_surface(tag: &str, s: &NurbsSurface<f64>) {
        println!(
            "DUMP {tag} pu={} pv={} nu={} nv={}",
            s.knots_u().degree(),
            s.knots_v().degree(),
            s.control_counts().0,
            s.control_counts().1
        );
        println!("DUMP {tag} knots_u {:?}", s.knots_u().knots());
        println!("DUMP {tag} knots_v {:?}", s.knots_v().knots());
        println!(
            "DUMP {tag} knots_u_hex {}",
            hxs(s.knots_u().knots().iter().copied())
        );
        println!(
            "DUMP {tag} knots_v_hex {}",
            hxs(s.knots_v().knots().iter().copied())
        );
        println!(
            "DUMP {tag} weights_hex {}",
            hxs(s.weights().iter().copied())
        );
        println!(
            "DUMP {tag} control_hex {}",
            hxs(s.control().iter().flat_map(|p| [p.x, p.y, p.z]))
        );
        if s.control().len() <= 40 {
            for (k, (p, w)) in s.control().iter().zip(s.weights()).enumerate() {
                println!(
                    "DUMP {tag} cp[{k}] = ({:?},{:?},{:?}) w={:?}",
                    p.x, p.y, p.z, w
                );
            }
        }
    }

    /// Per component (uu, uv, vv): (norm, i, j, u, v, vec).
    fn argmax(s: &NurbsSurface<f64>, n: u32) -> [(f64, u32, u32, f64, f64, [f64; 3]); 3] {
        let (u0, u1) = s.knots_u().domain();
        let (v0, v1) = s.knots_v().domain();
        let mut best = [(0.0f64, 0, 0, 0.0, 0.0, [0.0; 3]); 3];
        for i in 0..=n {
            for j in 0..=n {
                let u = u0 + (u1 - u0) * f64::from(i) / f64::from(n);
                let v = v0 + (v1 - v0) * f64::from(j) / f64::from(n);
                let jet = s.ders(u, v);
                for (k, d) in [jet.duu, jet.duv, jet.dvv].iter().enumerate() {
                    let nn = d.norm();
                    if nn > best[k].0 {
                        best[k] = (nn, i, j, u, v, [d.x, d.y, d.z]);
                    }
                }
            }
        }
        best
    }

    fn report(tag: &str, s: &NurbsSurface<f64>) {
        let names = ["uu", "uv", "vv"];
        let b = match nurbs_face_bound(s, FaceKey::default()) {
            Ok(b) => b,
            Err(e) => {
                println!("REPORT {tag}: bound refused: {e:?}");
                return;
            }
        };
        let (wuu, wuv, wvv) = sample_worst(s, 60);
        println!("REPORT {tag}: rational={}", patch_bound::is_rational(s));
        println!("REPORT {tag}: sampled ({wuu:.17e}, {wuv:.17e}, {wvv:.17e})");
        println!(
            "REPORT {tag}: bound   ({:.17e}, {:.17e}, {:.17e})",
            b.muu, b.muv, b.mvv
        );
        let ms = [b.muu, b.muv, b.mvv];
        let ws = [wuu, wuv, wvv];
        for k in 0..3 {
            println!(
                "REPORT {tag}: {} sampled-bound={:e} rel={:e} {}",
                names[k],
                ws[k] - ms[k],
                (ws[k] - ms[k]) / ms[k],
                if ws[k] > ms[k] { "RED" } else { "ok" }
            );
        }
        println!("HEX {tag} bound {}", hxs(ms));
        let am = argmax(s, 60);
        let cells = patch_bound::patch_cells(s).unwrap();
        let cb = nurbs_cell_bounds(s, FaceKey::default()).unwrap();
        println!("REPORT {tag}: ncells={}", cells.len());
        for k in 0..3 {
            let (nn, i, j, u, v, d) = am[k];
            println!(
                "REPORT {tag}: argmax {} norm={nn:.17e} at grid(i={i},j={j}) u={u:?} v={v:?} vec=({:?},{:?},{:?})",
                names[k], d[0], d[1], d[2]
            );
            println!(
                "HEX {tag} argmax {} u={} v={} vec={}",
                names[k],
                hx(u),
                hx(v),
                hxs(d)
            );
            // every cell whose closed rectangle contains (u,v)
            for (ci, (c, cbb)) in cells.iter().zip(&cb).enumerate() {
                if c.u.0 <= u && u <= c.u.1 && c.v.0 <= v && v <= c.v.1 {
                    let enc = [c.s_uu, c.s_uv, c.s_vv][k];
                    let m = [cbb.bound.muu, cbb.bound.muv, cbb.bound.mvv][k];
                    println!(
                        "REPORT {tag}:   containing cell#{ci} u=[{:?},{:?}] v=[{:?},{:?}] cell-bound={m:.17e} (global-cellbound={:e})",
                        c.u.0,
                        c.u.1,
                        c.v.0,
                        c.v.1,
                        ms[k] - m
                    );
                    for ch in 0..3 {
                        let inside = enc[ch].lo() <= d[ch] && d[ch] <= enc[ch].hi();
                        println!(
                            "REPORT {tag}:     ch{ch} enc=[{:.17e},{:.17e}] sampled={:.17e} inside={inside} (lo-gap={:e}, hi-gap={:e})",
                            enc[ch].lo(),
                            enc[ch].hi(),
                            d[ch],
                            d[ch] - enc[ch].lo(),
                            enc[ch].hi() - d[ch]
                        );
                        println!(
                            "HEX {tag} cell{ci} {} ch{ch} {} {}",
                            names[k],
                            hx(enc[ch].lo()),
                            hx(enc[ch].hi())
                        );
                    }
                }
            }
            // which cell attains the global bound
            let (bi, bm) = cb
                .iter()
                .enumerate()
                .map(|(ci, c)| (ci, [c.bound.muu, c.bound.muv, c.bound.mvv][k]))
                .fold((0, 0.0f64), |a, x| if x.1 > a.1 { x } else { a });
            println!(
                "REPORT {tag}:   global {} bound attained by cell#{bi} u=[{:?},{:?}] v=[{:?},{:?}] = {bm:.17e}",
                names[k], cb[bi].u.0, cb[bi].u.1, cb[bi].v.0, cb[bi].v.1
            );
        }
    }

    fn unit_weights(s: &NurbsSurface<f64>) -> NurbsSurface<f64> {
        NurbsSurface::new(
            s.knots_u().clone(),
            s.knots_v().clone(),
            s.control().to_vec(),
            vec![1.0; s.weights().len()],
        )
        .unwrap()
    }

    fn refined(s: &NurbsSurface<f64>) -> NurbsSurface<f64> {
        let r = s
            .refine_knots_u(&patch_bound::split_points(
                s.knots_u(),
                patch_bound::RATIONAL_CERT_SPLITS,
            ))
            .unwrap();
        r.refine_knots_v(&patch_bound::split_points(
            r.knots_v(),
            patch_bound::RATIONAL_CERT_SPLITS,
        ))
        .unwrap()
    }

    /// Replays the sweep for `CAD_FUZZ_SEED`, printing every trial at
    /// 17 digits and dumping every red trial. Never panics on a red.
    #[test]
    #[ignore = "diagnostic"]
    fn diag_replay() {
        let seed = fuzz::seed();
        let mut rng = fuzz::Rng::from_seed(seed ^ label_hash(LABEL));
        let trials = fuzz::scaled(60);
        for trial in 0..trials {
            let s = draw(&mut rng);
            let Ok(b) = nurbs_face_bound(&s, FaceKey::default()) else {
                println!("TRIAL {trial}: refused");
                continue;
            };
            let (wuu, wuv, wvv) = sample_worst(&s, 60);
            let red = !(wuv <= b.muv && wuu <= b.muu && wvv <= b.mvv);
            println!(
                "TRIAL {trial}: pu={} pv={} red={red} ratios=({:.6},{:.6},{:.6})",
                s.knots_u().degree(),
                s.knots_v().degree(),
                wuu / b.muu,
                wuv / b.muv,
                wvv / b.mvv
            );
            if red {
                println!("==== RED trial {trial} seed=0x{seed:016x}");
                dump_surface("orig", &s);
                report("orig", &s);
                report("transposed", &s.transposed());
                report("unitw", &unit_weights(&s));
                dump_surface("refined", &refined(&s));
            }
        }
    }

    /// Frequency census over `DIAG_SEEDS` seeds x 60 trials.
    #[test]
    #[ignore = "diagnostic"]
    fn diag_frequency() {
        let nseeds: u64 = std::env::var("DIAG_SEEDS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(100);
        let ntrials: usize = std::env::var("DIAG_TRIALS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(60);
        let mut seeder = fuzz::Rng::from_seed(0xD1A6_0000_0000_0001);
        let names = ["uu", "uv", "vv"];
        let (mut compared, mut reds, mut a_type, mut b_type) = (0usize, 0usize, 0usize, 0usize);
        let mut worst_rel = f64::NEG_INFINITY;
        let (mut zero_dir, mut worst_b) = (0usize, f64::NEG_INFINITY);
        let mut near = 0usize;
        let mut tightest = f64::NEG_INFINITY; // max over all comparisons of (w-m)/m
        let mut red_seeds = 0usize;
        for _ in 0..nseeds {
            let seed = seeder.next_u64();
            let mut rng = fuzz::Rng::from_seed(seed ^ label_hash(LABEL));
            let mut seed_red = false;
            for trial in 0..ntrials {
                let s = draw(&mut rng);
                // `DIAG_UNITW=1`: the same nets through the INTEGRAL arm.
                let s = if std::env::var_os("DIAG_UNITW").is_some() {
                    unit_weights(&s)
                } else {
                    s
                };
                let Ok(b) = nurbs_face_bound(&s, FaceKey::default()) else {
                    continue;
                };
                compared += 1;
                let (wuu, wuv, wvv) = sample_worst(&s, 60);
                let ws = [wuu, wuv, wvv];
                let ms = [b.muu, b.muv, b.mvv];
                for k in 0..3 {
                    let rel = (ws[k] - ms[k]) / ms[k];
                    tightest = tightest.max(rel);
                    if rel > -1e-12 {
                        near += 1;
                    }
                    if ws[k] > ms[k] {
                        reds += 1;
                        seed_red = true;
                        worst_rel = worst_rel.max(rel);
                        if ms[k] < 1e-100 {
                            zero_dir += 1;
                        } else if rel > 1e-9 {
                            a_type += 1;
                        } else {
                            b_type += 1;
                            worst_b = worst_b.max(rel);
                        }
                        println!(
                            "RED seed=0x{seed:016x} trial={trial} comp={} pu={} pv={} sampled={:.17e} bound={:.17e} rel={rel:e} ulps={}",
                            names[k],
                            s.knots_u().degree(),
                            s.knots_v().degree(),
                            ws[k],
                            ms[k],
                            ws[k].to_bits() as i64 - ms[k].to_bits() as i64
                        );
                    }
                }
            }
            if seed_red {
                red_seeds += 1;
            }
        }
        println!(
            "CENSUS seeds={nseeds} trials/seed={ntrials} compared={compared} red-components={reds} red-seeds={red_seeds} A-type(rel>1e-9)={a_type} B-type={b_type} (worst B rel={worst_b:e}) zero-direction(bound<1e-100)={zero_dir} worst-rel={worst_rel:e} comparisons-within-1e-12-of-bound={near} max-rel-overall={tightest:e}"
        );
    }
}
