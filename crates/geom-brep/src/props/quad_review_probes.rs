//! Reviewer probes for PR 4839 (cylinder lane `−∮ (v − v_a) du`, the
//! winding decides, and `quad::converged`). Independent oracles only.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::*;
use geom_core::Tol;

const TAU: f64 = core::f64::consts::TAU;
const PI: f64 = core::f64::consts::PI;

fn chan(c0: f64, ca: f64, cb: f64, cl: f64) -> HarmChan {
    HarmChan { c0: pt(c0), ca: pt(ca), cb: pt(cb), cl: pt(cl) }
}

fn edge(u: HarmChan, v: HarmChan, t0: f64, t1: f64, forward: bool) -> TrimEdgeQ {
    TrimEdgeQ {
        u,
        v,
        t0: pt(t0),
        t1: pt(t1),
        forward,
        trig0: (pt(t0.cos()), pt(t0.sin())),
        env: pt(0.0),
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn eps() -> f64 {
    Tol::witness().get().eps
}

fn run<T: Decide>(r: f64, loops: &[&[TrimEdgeQ]]) -> Result<FaceCutBounds, PropsError> {
    cylinder_cut_face::<T>(pt(r), Interval::zero(), loops, eps(), band())
}

/// Independent oracle: the area of the region {(u,v): u∈[0,2π), lo(u) ≤ v ≤ hi(u)}
/// by dense midpoint quadrature of (hi − lo) du — no Green form at all.
fn strip_area(lo: impl Fn(f64) -> f64, hi: impl Fn(f64) -> f64, r: f64) -> f64 {
    let n = 2_000_000u32;
    let h = TAU / f64::from(n);
    let (mut s, mut c) = (0.0f64, 0.0f64);
    for i in 0..n {
        let u = (f64::from(i) + 0.5) * h;
        let y = (hi(u) - lo(u)) * h - c;
        let t = s + y;
        c = (t - s) - y;
        s = t;
    }
    r * s
}

fn brackets(x: Interval, v: f64) -> bool {
    x.lo() <= v && v <= x.hi()
}

/// Claim 1: seamless bands with an ellipse rim, k from near 0 to near
/// π/2 tilt, far along the axis, both senses, both scalars, radius ≠ 1.
#[test]
fn probe_band_ellipse_sweep_both_scalars() {
    for &r in &[1.0, 0.003, 250.0] {
        for &k in &[0.0f64, 1e-12, 1e-6, 0.4, -0.7, 3.0, 50.0, 1e3] {
            for &lift in &[0.0, -7.5, 1e3, 1e6] {
                for &sense in &[true, false] {
                    let h = 2.0 + 2.0 * k.abs();
                    let bot = edge(chan(0.0, 0.0, 0.0, 1.0), chan(lift, 0.0, 0.0, 0.0), 0.0, TAU, sense);
                    let top = edge(chan(0.0, 0.0, 0.0, 1.0), chan(lift + h, k, 0.0, 0.0), 0.0, TAU, !sense);
                    let truth = r * r * TAU * ((lift + h) - lift); // closed form of the STORED rims: ∫(h + k cos u) du = 2πh
                    let area_truth = truth / r;
                    let sgn = if sense { 1.0 } else { -1.0 };
                    for scalar in ["f64", "Interval"] {
                        let got = if scalar == "f64" {
                            run::<f64>(r, &[&[bot], &[top]])
                        } else {
                            run::<Interval>(r, &[&[bot], &[top]])
                        };
                        let at = format!("r {r} k {k} lift {lift} sense {sense} {scalar}");
                        match got {
                            Ok(out) => {
                                assert!(brackets(out.area, area_truth), "{at}: area {:?} vs {area_truth}", out.area);
                                assert!(brackets(out.flux, sgn * truth), "{at}: flux {:?} vs {}", out.flux, sgn * truth);
                            }
                            Err(e) => println!("REFUSED {at}: {e:?}"),
                        }
                    }
                }
            }
        }
    }
}

/// Claim 1: a seamed face (one loop: bottom, seam up at 2π, the ellipse
/// top backwards, seam down at 0), and the same with the up-seam stored
/// a period off (at u = 0), and a non-winding loop straddling the seam
/// stored in two branches.
#[test]
fn probe_seamed_and_period_off_faces() {
    let k = 0.6;
    let lift = 3.0;
    let h = 2.0;
    let truth = strip_area(|_| lift, |u| lift + h + k * u.cos(), 1.0);
    for &up_u in &[TAU, 0.0, 2.0 * TAU, -TAU] {
        let bot = edge(chan(0.0, 0.0, 0.0, 1.0), chan(lift, 0.0, 0.0, 0.0), 0.0, TAU, true);
        // seam up at u = up_u from v = lift to lift + h + k
        let up = edge(chan(up_u, 0.0, 0.0, 0.0), chan(lift, 0.0, 0.0, 1.0), 0.0, h + k, true);
        let top = edge(chan(0.0, 0.0, 0.0, 1.0), chan(lift + h, k, 0.0, 0.0), 0.0, TAU, false);
        let down = edge(chan(0.0, 0.0, 0.0, 0.0), chan(lift, 0.0, 0.0, 1.0), 0.0, h + k, false);
        for scalar in ["f64", "Interval"] {
            let loops: [&[TrimEdgeQ]; 1] = [&[bot, up, top, down]];
            let got = if scalar == "f64" { run::<f64>(1.0, &loops) } else { run::<Interval>(1.0, &loops) };
            let out = got.unwrap_or_else(|e| panic!("seam at {up_u} {scalar}: {e:?}"));
            assert!(brackets(out.area, truth), "seam at {up_u} {scalar}: {:?} vs {truth}", out.area);
            assert!(brackets(out.flux, truth), "seam at {up_u} {scalar}: {:?} vs {truth}", out.flux);
        }
    }
    // A small square [2π−0.1, 2π+0.1] × [1, 1.3] around the seam,
    // its right half stored in the branch at u ≈ 0 (each edge in its own
    // branch), with a circular hole straddling too.
    let a = TAU - 0.1;
    let pieces = [
        edge(chan(a, 0.0, 0.0, 1.0), chan(1.0, 0.0, 0.0, 0.0), 0.0, 0.1, true), // bottom left half
        edge(chan(0.0, 0.0, 0.0, 1.0), chan(1.0, 0.0, 0.0, 0.0), 0.0, 0.1, true), // bottom right half (branch 0)
        edge(chan(0.1, 0.0, 0.0, 0.0), chan(1.0, 0.0, 0.0, 1.0), 0.0, 0.3, true), // right side (branch 0)
        edge(chan(0.0, 0.0, 0.0, 1.0), chan(1.3, 0.0, 0.0, 0.0), 0.0, 0.1, false), // top right half (branch 0)
        edge(chan(a, 0.0, 0.0, 1.0), chan(1.3, 0.0, 0.0, 0.0), 0.0, 0.1, false), // top left half
        edge(chan(a, 0.0, 0.0, 0.0), chan(1.0, 0.0, 0.0, 1.0), 0.0, 0.3, false), // left side
    ];
    let hole = edge(chan(TAU, 0.05, 0.0, 0.0), chan(1.15, 0.0, 0.05, 0.0), 0.0, TAU, false);
    let truth = 0.2 * 0.3 - PI * 0.05 * 0.05;
    for scalar in ["f64", "Interval"] {
        let loops: [&[TrimEdgeQ]; 2] = [&pieces, &[hole]];
        let got = if scalar == "f64" { run::<f64>(2.0, &loops) } else { run::<Interval>(2.0, &loops) };
        let out = got.unwrap_or_else(|e| panic!("seam-straddling square {scalar}: {e:?}"));
        assert!(brackets(out.area, 2.0 * truth), "square {scalar}: {:?} vs {}", out.area, 2.0 * truth);
        assert!(brackets(out.flux, 4.0 * truth), "square {scalar}: {:?} vs {}", out.flux, 4.0 * truth);
    }
}

/// Claim 1: a band with a hole, and a band with a ring wound WITH it
/// (must refuse RingWoundWithFace).
#[test]
fn probe_band_with_hole() {
    let bot = edge(chan(0.0, 0.0, 0.0, 1.0), chan(0.0, 0.0, 0.0, 0.0), 0.0, TAU, true);
    let top = edge(chan(0.0, 0.0, 0.0, 1.0), chan(3.0, 0.5, 0.0, 0.0), 0.0, TAU, false);
    let hole = |fwd| edge(chan(2.0, 0.4, 0.0, 0.0), chan(1.2, 0.0, 0.4, 0.0), 0.0, TAU, fwd);
    let (hf, ht) = ([hole(false)], [hole(true)]);
    let (bot, top) = ([bot], [top]);
    let truth = 3.0 * TAU - PI * 0.16;
    for order in 0..3 {
        let loops: Vec<&[TrimEdgeQ]> = match order {
            0 => vec![&bot[..], &top[..], &hf[..]],
            1 => vec![&hf[..], &bot[..], &top[..]],
            _ => vec![&top[..], &hf[..], &bot[..]],
        };
        for scalar in ["f64", "Interval"] {
            let got = if scalar == "f64" { run::<f64>(1.0, &loops) } else { run::<Interval>(1.0, &loops) };
            match got {
                Ok(out) => assert!(brackets(out.area, truth), "order {order} {scalar}: {:?} vs {truth}", out.area),
                Err(e) => println!("REFUSED band+hole order {order} {scalar}: {e:?}"),
            }
        }
        let loops_bad: Vec<&[TrimEdgeQ]> = match order {
            0 => vec![&bot[..], &top[..], &ht[..]],
            1 => vec![&ht[..], &bot[..], &top[..]],
            _ => vec![&top[..], &ht[..], &bot[..]],
        };
        let got = run::<f64>(1.0, &loops_bad);
        println!("band + ring wound WITH face, order {order}: {:?}", got.map(|o| (o.area, o.flux)));
    }
}

/// Claim 2: a pair of loops that each wind TWICE, opposite ways. Each
/// is a self-crossing curve; no face's boundary is one. Total winding 0.
#[test]
fn probe_two_double_winders() {
    let a = edge(chan(0.0, 0.0, 0.0, 2.0), chan(0.0, 0.0, 0.0, 0.0), 0.0, TAU, true);
    let b = edge(chan(0.0, 0.0, 0.0, 2.0), chan(2.0, 0.0, 0.0, 0.0), 0.0, TAU, false);
    for scalar in ["f64", "Interval"] {
        let loops: [&[TrimEdgeQ]; 2] = [&[a], &[b]];
        let got = if scalar == "f64" { run::<f64>(1.0, &loops) } else { run::<Interval>(1.0, &loops) };
        println!("double winders {scalar}: {:?} (a band of these rims at one turn has area {})",
            got.map(|o| (o.area, o.flux)), 2.0 * TAU);
    }
    // 1.5 turns each: no closed curve at all.
    let a = edge(chan(0.0, 0.0, 0.0, 1.5), chan(0.0, 0.0, 0.0, 0.0), 0.0, TAU, true);
    let b = edge(chan(0.0, 0.0, 0.0, 1.5), chan(2.0, 0.0, 0.0, 0.0), 0.0, TAU, false);
    let got = run::<Interval>(1.0, &[&[a], &[b]]);
    println!("1.5-turn pair Interval: {:?}", got.map(|o| (o.area, o.flux)));
    // 0.4 of a turn, alone: an open arc with nothing to bound.
    let a = edge(chan(0.0, 0.0, 0.0, 0.4), chan(0.0, 0.0, 0.0, 0.0), 0.0, TAU, true);
    let got = run::<Interval>(1.0, &[&[a]]);
    println!("0.4-turn open arc Interval: {:?}", got.map(|o| (o.area, o.flux)));
}

/// Claim 2: the refusals at the Interval scalar.
#[test]
fn probe_refusals_at_interval() {
    let bot = edge(chan(0.0, 0.0, 0.0, 1.0), chan(0.0, 0.0, 0.0, 0.0), 0.0, TAU, true);
    let top = edge(chan(0.0, 0.0, 0.0, 1.0), chan(2.0, 0.4, 0.0, 0.0), 0.0, TAU, false);
    let mut same = top;
    same.forward = true;
    for (what, loops) in [
        ("top alone", vec![&[top][..]]),
        ("same way", vec![&[bot][..], &[same][..]]),
        ("bottom alone", vec![&[bot][..]]),
    ] {
        let got = run::<Interval>(1.0, &loops);
        assert!(matches!(got, Err(PropsError::LoopsWindChart)), "{what}: {got:?}");
    }
}

/// Claim 3: a reading in the Zero band (|target − width| ≤ ε) at the
/// last round. `converged` maps it to "not converged" and `round_exit`
/// to `QuadratureBudget` — with a width that may be UNDER the target.
#[test]
fn probe_zero_band_at_the_last_round_is_a_budget_refusal_under_target() {
    let target = QUAD_TARGET_LEN_FACTOR * eps();
    // The hopeless-endpoint fixture, its start bracket half-width w.
    let at = |w: f64| {
        let e = TrimEdgeQ {
            u: chan(0.0, 1.0, 0.0, 0.0),
            v: chan(0.0, 0.0, 1.0, 0.0),
            t0: Interval::from_bounds(-w, w),
            t1: pt(TAU),
            forward: true,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        };
        cylinder_cut_face::<f64>(pt(1.0), Interval::zero(), &[&[e]], eps(), band())
    };
    let wl = |r: &Result<FaceCutBounds, PropsError>| match r {
        Ok(o) => o.flux.width() / (3.0 * o.area.lo()),
        Err(PropsError::QuadratureBudget { width_len, .. }) => *width_len,
        Err(_) => f64::NAN,
    };
    // Measured: margin 9.998e-9 at w = 2.3891717e-6 (width 1.0140e-6), linear in w.
    let lo = 2.389_171_737_005_034e-6 * (1.0 + 9.998_457_735e-9 / 1.014_001_542e-6);
    let mut under = 0;
    let mut seen = Vec::new();
    for i in -40..=40 {
        let w = lo * (1.0 + f64::from(i) * 5e-5);
        let r = at(w);
        let x = wl(&r);
        let kind = match &r {
            Ok(_) => "converged",
            Err(PropsError::QuadratureBudget { .. }) => "budget",
            Err(PropsError::Escalated { .. }) => "escalated",
            Err(_) => "other",
        };
        if i % 4 == 0 { println!("w {w:e} kind {kind} width {x:e} target {target:e} margin {:e}", target - x); }
        if kind == "budget" && x < target {
            under += 1;
            if seen.len() < 3 { seen.push((w, x, target)); }
        }
    }
    println!("budget refusals with width < target: {under}; e.g. {seen:?}");
    assert_eq!(under, 0, "a QuadratureBudget refusal carries width_len < target_len: {seen:?}");
}

/// Claim 2 / regression vs main: a loop whose edges do not meet in v
/// (a rectangle whose top sits 5 m above its sides' ends).
#[test]
fn probe_open_in_v_rectangle() {
    let line = |u: HarmChan, v: HarmChan, span: f64, forward: bool| edge(u, v, 0.0, span, forward);
    let outer = [
        line(chan(0.0, 0.0, 0.0, 1.0), chan(0.0, 0.0, 0.0, 0.0), 2.0, true),
        line(chan(2.0, 0.0, 0.0, 0.0), chan(0.0, 0.0, 0.0, 1.0), 1.0, true),
        line(chan(0.0, 0.0, 0.0, 1.0), chan(6.0, 0.0, 0.0, 0.0), 2.0, false),
        line(chan(0.0, 0.0, 0.0, 0.0), chan(0.0, 0.0, 0.0, 1.0), 1.0, false),
    ];
    println!("open-in-v rectangle: {:?}", run::<Interval>(1.0, &[&outer]).map(|o| (o.area, o.flux)));
}
