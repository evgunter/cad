//! Reviewer rows for the cylinder's and torus's per-edge whole-turn
//! misses (`props_chart_loop_whole`), through the public door
//! [`curved_face_loops`], at `f64` and at the interval scalar, on FAR
//! walls in a generic pose (origin `(30, −110, 200)`, axis
//! `(1, 1, 1)/√3`).
//!
//! - Legal many-edged walls measure (at ε = 1e-12 the interval scalar
//!   escalated these while each miss was read back about the axis).
//! - A hand-assembled wall whose rim does not turn as its ends say is
//!   still refused, here or earlier, typed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::band;
use crate::shared::topo::edge;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, curved_face_loops};
use geom_core::{Bounds, Decide, Point3, Vec3};

/// A scalar read as one `f64` (an enclosure's midpoint).
trait Mid {
    fn mid(self) -> f64;
}
impl Mid for f64 {
    fn mid(self) -> f64 {
        self
    }
}
impl Mid for geom_core::Interval {
    fn mid(self) -> f64 {
        0.5 * (self.lo() + self.hi())
    }
}
use std::f64::consts::TAU;

fn frame<T: Decide>() -> (Point3<T>, Vec3<T>, Vec3<T>, Vec3<T>) {
    let (s3, s2, s6) = (
        3.0_f64.sqrt().recip(),
        2.0_f64.sqrt().recip(),
        6.0_f64.sqrt().recip(),
    );
    (
        p3(30.0, -110.0, 200.0),
        v3(s3, s3, s3),
        v3(s2, -s2, 0.0),
        v3(s6, s6, -2.0 * s6),
    )
}

fn at<T: Decide>(c: f64, s: f64, h: f64) -> Point3<T> {
    let (o, n, x, y) = frame::<T>();
    o + x * T::from_f64(c) + y * T::from_f64(s) + n * T::from_f64(h)
}

fn cylinder<T: Decide>(radius: f64) -> Surface<T> {
    let (o, n, x, _) = frame::<T>();
    Surface::Cylinder {
        origin: o,
        axis: n,
        radius: T::from_f64(radius),
        u_ref: x,
    }
}

fn torus<T: Decide>(major: f64, minor: f64) -> Surface<T> {
    let (o, n, x, _) = frame::<T>();
    Surface::Torus {
        center: o,
        axis: n,
        major_radius: T::from_f64(major),
        minor_radius: T::from_f64(minor),
        u_ref: x,
    }
}

/// A circle about the frame's axis at height `h`, radius `r`, walked in
/// azimuth from `u0` to `u1`.
fn rim<T: Decide>(r: f64, h: f64, u0: f64, u1: f64) -> LoopEdge<T> {
    let (_, n, x, _) = frame::<T>();
    edge(
        Curve3::circle(at(0.0, 0.0, h), n, T::from_f64(r), x),
        u0,
        u1,
        0,
        0,
    )
}

fn ruling<T: Decide>(r: f64, u: f64, z0: f64, z1: f64) -> LoopEdge<T> {
    let (_, n, _, _) = frame::<T>();
    edge(
        Curve3::line(at(r * u.cos(), r * u.sin(), 0.0), n),
        z0,
        z1,
        0,
        0,
    )
}

/// The torus's meridian at azimuth `u`, walked in tube angle `v0 → v1`.
fn meridian<T: Decide>(major: f64, minor: f64, u: f64, v0: f64, v1: f64) -> LoopEdge<T> {
    let (_, _, x, y) = frame::<T>();
    let (s, c) = u.sin_cos();
    let out = x * T::from_f64(c) + y * T::from_f64(s);
    let normal = x * T::from_f64(s) - y * T::from_f64(c);
    edge(
        Curve3::circle(
            at(major * c, major * s, 0.0),
            normal,
            T::from_f64(minor),
            out,
        ),
        v0,
        v1,
        0,
        0,
    )
}

/// `n` arcs of a rim from `u0` to `u1`.
fn arcs<T: Decide>(r: f64, h: f64, (u0, u1): (f64, f64), n: usize) -> Vec<LoopEdge<T>> {
    (0..n)
        .map(|i| {
            let t = |k: usize| u0 + (u1 - u0) * k as f64 / n as f64;
            rim(r, h, t(i), t(i + 1))
        })
        .collect()
}

fn cylinder_patch<T: Decide>(r: f64, span: f64, height: f64, n: usize) -> Vec<LoopEdge<T>> {
    let mut l = arcs::<T>(r, 0.0, (0.0, span), n);
    l.push(ruling(r, span, 0.0, height));
    l.extend(arcs::<T>(r, height, (span, 0.0), n));
    l.push(ruling(r, 0.0, height, 0.0));
    l
}

fn torus_patch<T: Decide>(
    major: f64,
    minor: f64,
    span: f64,
    (v0, v1): (f64, f64),
    n: usize,
) -> Vec<LoopEdge<T>> {
    let par = |v: f64, ends| arcs::<T>(major + minor * v.cos(), minor * v.sin(), ends, n);
    let mut l = par(v0, (0.0, span));
    l.push(meridian(major, minor, span, v0, v1));
    l.extend(par(v1, (span, 0.0)));
    l.push(meridian(major, minor, 0.0, v1, v0));
    l
}

fn measure<T: Decide + Mid>(s: &Surface<T>, loops: &[&[LoopEdge<T>]]) -> Result<f64, PropsError> {
    curved_face_loops(s, loops, true, band()).map(|c| c.area.mid())
}

fn close(got: Result<f64, PropsError>, want: f64, what: &str) {
    let got = got.unwrap_or_else(|e| panic!("{what}: {e:?}"));
    assert!(
        (got - want).abs() <= 1e-9 * want,
        "{what}: area {got} against {want}"
    );
}

/// Every wall of the ladder, each outcome printed (`REVIEW-WALL`); the
/// row is red when any does not measure its closed form, after printing
/// all.
///
/// Measured on the frozen head `e010b7d7`: green at ε = 1e-9 and 1e-6
/// (72 of 72), RED at ε = 1e-12, where 33 of 72 measure (30 under the
/// merge base's reading). What still escalates `props_chart_loop_whole`
/// at the interval scalar there is every wall whose rims are several
/// PARTIAL arcs (6 arcs a rim at r = 50; one arc a rim at r = 410): a
/// carried arc is charged about twice the width of its ends' enclosures
/// whatever its radius. At `f64` the 200-arc walls refuse on rounding,
/// before and after.
#[test]
fn far_tilted_walls_of_many_edges_measure() {
    fn rows<T: Decide + Mid>(scalar: &str, bad: &mut Vec<String>) {
        let mut note = |got: Result<f64, PropsError>, want: f64, what: String| {
            let line = match got {
                Ok(a) if (a - want).abs() <= 1e-9 * want => format!("ok   {what}"),
                Ok(a) => format!("BAD  {what}: area {a} against {want}"),
                Err(e) => {
                    let e = format!("{e:?}");
                    format!("REF  {what}: {}", &e[..e.len().min(260)])
                }
            };
            eprintln!("REVIEW-WALL {line}");
            if !line.starts_with("ok") {
                bad.push(line);
            }
        };
        for r in [50.0, 213.0, 410.0, 990.0] {
            let (span, height) = (1.0, 0.7);
            for n in [1, 6, 40, 200] {
                let l = cylinder_patch::<T>(r, span, height, n);
                note(
                    measure(&cylinder::<T>(r), &[&l]),
                    r * span * height,
                    format!("cylinder patch r {r}, {n} arcs a rim, at {scalar}"),
                );
            }
            let (lo, hi) = (
                [rim::<T>(r, 0.0, 0.0, TAU)],
                [rim::<T>(r, height, TAU, 0.0)],
            );
            note(
                measure(&cylinder::<T>(r), &[&lo, &hi]),
                r * TAU * height,
                format!("cylinder band r {r} at {scalar}"),
            );
        }
        for major in [100.0, 300.0, 600.0, 990.0] {
            let minor = 2.5;
            let (v0, v1) = (0.2_f64, 0.7_f64);
            for n in [1, 6, 40, 200] {
                let l = torus_patch::<T>(major, minor, 1.0, (v0, v1), n);
                note(
                    measure(&torus::<T>(major, minor), &[&l]),
                    minor * (major * (v1 - v0) + minor * (v1.sin() - v0.sin())),
                    format!("torus patch major {major}, {n} arcs a parallel, at {scalar}"),
                );
            }
        }
    }
    let mut bad = Vec::new();
    rows::<f64>("f64", &mut bad);
    rows::<geom_core::Interval>("the interval scalar", &mut bad);
    assert!(
        bad.is_empty(),
        "{} walls did not measure:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// A hand-assembled far cylinder wall whose bottom rim does not turn as
/// the wall's other edges need is refused, at both scalars: a rim tilted
/// off the axis, a rim whose span runs a whole turn past its ends, and
/// a rim on a circle of the opposite normal walked the wrong way. The
/// same circle walked the right way measures.
#[test]
fn a_hand_built_far_wall_whose_rim_disagrees_still_refuses() {
    fn rows<T: Decide + Mid>(scalar: &str) {
        let (r, span, height) = (213.0, 1.0, 0.7);
        let (_, n, x, y) = frame::<T>();
        let with_bottom = |bottom: LoopEdge<T>| {
            let mut l = vec![bottom, ruling::<T>(r, span, 0.0, height)];
            l.extend(arcs::<T>(r, height, (span, 0.0), 1));
            l.push(ruling(r, 0.0, height, 0.0));
            measure(&cylinder::<T>(r), &[&l])
        };
        let flipped = |t0: f64, t1: f64| {
            edge(
                Curve3::circle(at(0.0, 0.0, 0.0), -n, T::from_f64(r), x),
                t0,
                t1,
                0,
                0,
            )
        };
        close(
            with_bottom(flipped(0.0, -span)),
            r * span * height,
            &format!("a rim on the opposite normal, walked the right way, at {scalar}"),
        );
        for tilt in [1e-3_f64, 1e-5, 3e-7] {
            let (s, c) = tilt.sin_cos();
            let tilted = edge(
                Curve3::circle(
                    at(0.0, 0.0, 0.0),
                    n * T::from_f64(c) + y * T::from_f64(s),
                    T::from_f64(r),
                    x,
                ),
                0.0,
                span,
                0,
                0,
            );
            let got = with_bottom(tilted);
            eprintln!("REVIEW tilted {tilt} at {scalar}: {got:?}");
            assert!(got.is_err(), "a rim tilted {tilt} at {scalar}: {got:?}");
        }
        for (what, bottom) in [
            (
                "a whole turn past its ends",
                rim::<T>(r, 0.0, 0.0, span + TAU),
            ),
            ("the opposite normal, the wrong way", flipped(0.0, span)),
            (
                "a span short of its ruling",
                rim::<T>(r, 0.0, 0.0, span - 1e-3),
            ),
        ] {
            let got = with_bottom(bottom);
            eprintln!("REVIEW {what} at {scalar}: {got:?}");
            assert!(got.is_err(), "a rim {what} at {scalar}: {got:?}");
        }
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}
