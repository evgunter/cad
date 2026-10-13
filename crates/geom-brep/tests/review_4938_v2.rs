//! Reviewer probes for PR 4938's second fix pass: the per-loop
//! `props_chart_loop_whole` decide against legal faces with many
//! junctions, and against rims whose carriers sit within ε of the axis
//! but not on it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::{band, eps};
use crate::shared::topo::edge;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, curved_face_loops};
use std::f64::consts::TAU;

/// A rim arc on a circle of `radius` centred `off` from the axis at
/// height `z`, from `u0` to `u1` about its own centre.
fn arc<T: geom_core::Decide>(radius: f64, off: [f64; 2], z: f64, u0: f64, u1: f64) -> LoopEdge<T> {
    edge(
        Curve3::Circle {
            center: p3(off[0], off[1], z),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(radius),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}

fn line<T: geom_core::Decide>(a: [f64; 3], b: [f64; 3]) -> LoopEdge<T> {
    let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    edge(
        Curve3::Line {
            origin: p3(a[0], a[1], a[2]),
            dir: v3(d[0] / len, d[1] / len, d[2] / len),
        },
        0.0,
        len,
        0,
        0,
    )
}

fn cyl<T: geom_core::Decide>(radius: f64) -> Surface<T> {
    Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: T::from_f64(radius),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

fn class<X: core::fmt::Debug>(r: &Result<X, PropsError>) -> String {
    match r {
        Ok(x) => format!("ok {x:?}").chars().take(40).collect(),
        Err(PropsError::Escalated { cause, .. }) => {
            let s = format!("{cause:?}");
            let pred = s
                .split("predicate: Some(\"")
                .nth(1)
                .and_then(|t| t.split('"').next())
                .unwrap_or("?")
                .to_string();
            let m = s
                .split("Value(")
                .nth(1)
                .or_else(|| s.split("lo: ").nth(1))
                .and_then(|t| t.split([',', ')']).next())
                .unwrap_or("?")
                .to_string();
            format!("esc {pred} {m}")
        }
        Err(e) => format!("{e:?}").chars().take(60).collect(),
    }
}

/// A band of the cylinder of `radius`: its lower rim split into `n`
/// arcs, each arc's centre `off·ε` from the axis toward the arc's middle
/// (`off = 0`: on the axis), consecutive arcs meeting with azimuth gaps
/// alternating `±gap·ε` (metres); its upper rim whole.
fn band_case<T: geom_core::Decide>(radius: f64, n: usize, gap: f64, off: f64) -> String {
    let e = eps();
    let step = TAU / n as f64;
    let mut lower = Vec::with_capacity(n);
    for i in 0..n {
        let t0 = i as f64 * step;
        let t1 = t0 + step;
        // The junction at the end of arc i: arc i ends `±gap/2` from
        // `t1` and arc i+1 starts `∓gap/2` from it.
        let s = if i % 2 == 0 { 1.0 } else { -1.0 };
        let half = 0.5 * gap * e / radius;
        let a = if i == 0 { t0 } else { t0 + s * half };
        let b = if i == n - 1 { t1 } else { t1 + s * half };
        let tm = 0.5 * (t0 + t1);
        let c = [off * e * tm.cos(), off * e * tm.sin()];
        lower.push(arc::<T>(radius, c, 0.0, a, b));
    }
    let upper = vec![arc::<T>(radius, [0.0, 0.0], 1.0, TAU, 0.0)];
    let s = cyl::<T>(radius);
    class(&curved_face_loops(&s, &[&lower, &upper], true, band()).map(|c| c.area))
}

/// A rectangle `u ∈ [0, span]`, `z ∈ [0, 1]` whose lower edge is `n`
/// arcs and whose sides are rulings, the arcs meeting with alternating
/// `±gap·ε` azimuth gaps, centres `off·ε` toward each arc's middle.
fn rect_case<T: geom_core::Decide>(radius: f64, n: usize, gap: f64, off: f64, span: f64) -> String {
    let e = eps();
    let step = span / n as f64;
    let mut l = Vec::new();
    for i in 0..n {
        let t0 = i as f64 * step;
        let t1 = t0 + step;
        let s = if i % 2 == 0 { 1.0 } else { -1.0 };
        let half = 0.5 * gap * e / radius;
        let a = if i == 0 { t0 } else { t0 + s * half };
        let b = if i == n - 1 { t1 } else { t1 + s * half };
        let tm = 0.5 * (t0 + t1);
        l.push(arc::<T>(
            radius,
            [off * e * tm.cos(), off * e * tm.sin()],
            0.0,
            a,
            b,
        ));
    }
    let at = |u: f64, z: f64| [radius * u.cos(), radius * u.sin(), z];
    l.push(line::<T>(at(span, 0.0), at(span, 1.0)));
    l.push(arc::<T>(radius, [0.0, 0.0], 1.0, span, 0.0));
    l.push(line::<T>(at(0.0, 1.0), at(0.0, 0.0)));
    let s = cyl::<T>(radius);
    class(&curved_face_loops(&s, &[&l], true, band()).map(|c| c.area))
}

#[test]
fn review_4938_v2_many_junctions_and_off_axis_rims() {
    let e = eps();
    for radius in [1.0, 0.01] {
        for n in [4usize, 50, 200] {
            for gap in [0.0, 1.0, 2.0, 3.0, 3.5] {
                for off in [0.0, 0.5, 0.9] {
                    eprintln!(
                        "REVIEW4938V2 eps={e:e} band R={radius} n={n} gap={gap}ε off={off}ε: f64 {} | iv {}",
                        band_case::<f64>(radius, n, gap, off),
                        band_case::<geom_core::Interval>(radius, n, gap, off)
                    );
                    eprintln!(
                        "REVIEW4938V2 eps={e:e} rect R={radius} n={n} gap={gap}ε off={off}ε: f64 {} | iv {}",
                        rect_case::<f64>(radius, n, gap, off, 5.0),
                        rect_case::<geom_core::Interval>(radius, n, gap, off, 5.0)
                    );
                }
            }
        }
    }
}

/// Item (a) variants: the zig-zag at other radii and ruling counts.
#[test]
fn review_4938_v2_zig_variants() {
    let e = eps();
    for (k, n, h) in [
        (11.0, 60usize, 0.001),
        (100.0, 400, 0.001),
        (100.0, 2000, 0.0005),
        (1000.0, 4000, 0.0001),
    ] {
        let radius = k * e;
        let delta = 2.0 * (0.45 * e / radius).asin();
        let at = |u: f64, z: f64| [radius * u.cos(), radius * u.sin(), z];
        let build = |turn_sign: f64| {
            let mut zig: Vec<LoopEdge<f64>> = Vec::new();
            let mut u = 0.0;
            for i in 0..n {
                let (z0, z1) = if i % 2 == 0 { (0.0, h) } else { (h, 0.0) };
                zig.push(line(at(u, z0), at(u + turn_sign * delta, z1)));
                u += turn_sign * delta;
            }
            zig.push(arc(radius, [0.0, 0.0], 0.0, u, 0.0));
            (zig, u)
        };
        for sign in [1.0, -1.0] {
            let (zig, u) = build(sign);
            let s = cyl::<f64>(radius);
            for up in [None, Some(true), Some(false)] {
                let whole = match up {
                    Some(true) => vec![arc(radius, [0.0, 0.0], 1.0, 0.0, TAU)],
                    Some(false) => vec![arc(radius, [0.0, 0.0], 1.0, TAU, 0.0)],
                    None => vec![],
                };
                let loops: Vec<&[LoopEdge<f64>]> = if up.is_some() {
                    vec![&zig, &whole]
                } else {
                    vec![&zig]
                };
                eprintln!(
                    "REVIEW4938V2 eps={e:e} zig R={k}ε n={n} sign={sign} turned={u:.3} rim={up:?}: {}",
                    class(&curved_face_loops(&s, &loops, true, band()).map(|c| c.area))
                );
            }
        }
    }
}

/// A whole cylinder wall cut along a seam ruling: lower rim `0 → τ` and
/// upper rim `τ → 0`, each split into `n` arcs whose centres sit
/// `off·ε` from the axis toward the arc's middle (lower) or away from it
/// (upper), so both chains' stored spans overshoot their ends' azimuths
/// the same way round the loop.
fn seam_case<T: geom_core::Decide>(n: usize, off: f64) -> String {
    let (e, radius) = (eps(), 1.0);
    let step = TAU / n as f64;
    let mut l = Vec::new();
    for i in 0..n {
        let (t0, t1) = (i as f64 * step, (i + 1) as f64 * step);
        let tm = 0.5 * (t0 + t1);
        l.push(arc::<T>(
            radius,
            [off * e * tm.cos(), off * e * tm.sin()],
            0.0,
            t0,
            t1,
        ));
    }
    let at = |u: f64, z: f64| [radius * u.cos(), radius * u.sin(), z];
    l.push(line::<T>(at(0.0, 0.0), at(0.0, 1.0)));
    for i in (0..n).rev() {
        let (t0, t1) = (i as f64 * step, (i + 1) as f64 * step);
        let tm = 0.5 * (t0 + t1);
        l.push(arc::<T>(
            radius,
            [-off * e * tm.cos(), -off * e * tm.sin()],
            1.0,
            t1,
            t0,
        ));
    }
    l.push(line::<T>(at(0.0, 1.0), at(0.0, 0.0)));
    let s = cyl::<T>(radius);
    class(&curved_face_loops(&s, &[&l], true, band()).map(|c| c.area))
}

#[test]
fn review_4938_v2_seam_cut_wall() {
    let e = eps();
    for n in [2usize, 4, 8, 50] {
        for off in [0.0, 0.5, 0.9, 0.99] {
            eprintln!(
                "REVIEW4938V2 eps={e:e} seam n={n} off={off}ε: f64 {} | iv {}",
                seam_case::<f64>(n, off),
                seam_case::<geom_core::Interval>(n, off)
            );
        }
    }
}
