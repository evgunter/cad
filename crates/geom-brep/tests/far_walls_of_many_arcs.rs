//! **Far, generic-pose walls of many edges** through the public door
//! [`curved_face_loops`], at `f64` and at the interval scalar: hand-built
//! legal cylinder and torus walls in the frame at `(30, −110, 200)` about
//! the axis `(1, 1, 1)/√3`, each rim or parallel in 1 to 200 arcs.
//!
//! - Every wall measures its closed form above ε = 1e-12. At ε = 1e-12
//!   each that does not is pinned to the refusal it measures, typed by
//!   its predicate and citing the row that holds it ([`PINS`]).
//! - A hand-built wall whose rim does not turn as its ends say refuses
//!   before the whole-turn decision is asked, typed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::{band, eps};
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

/// The row that holds a carried arc's miss at the interval scalar.
const CARRIED: &str = "work/nearpoint/a-carried-arcs-miss-is-charged-its-ends-enclosure-width-at-the-interval-scalar.md";
/// The row that holds the junction and incidence reads beside it.
const BESIDE: &str =
    "work/nearpoint/a-far-walls-junctions-and-ruling-incidence-escalate-at-the-interval-scalar.md";
/// The row that holds the `f64` rounding of many arcs, summed.
const ROUNDING: &str = "work/nearpoint/a-legal-far-wall-of-many-arcs-refuses-its-whole-turn-at-f64-on-summed-rounding.md";

/// What a wall answers at ε = 1e-12 where it does not measure.
#[derive(Clone, Copy, Debug)]
enum Pin {
    /// Escalates on the predicate, held by the row.
    Escalates(&'static str, &'static str),
    /// Refuses `LoopsWindChart`, held by the row.
    Winds(&'static str),
}

/// Every wall that does not measure at ε = 1e-12, by its label and the
/// scalar, `"f64"` or `"interval"`.
const PINS: &[(&str, &str, Pin)] = &[
    (
        "cylinder patch r 213, 200 arcs",
        "f64",
        Pin::Escalates("props_chart_loop_whole", ROUNDING),
    ),
    (
        "cylinder patch r 410, 200 arcs",
        "f64",
        Pin::Escalates("props_chart_loop_whole", ROUNDING),
    ),
    (
        "cylinder patch r 990, 40 arcs",
        "f64",
        Pin::Escalates("props_chart_loop_whole", ROUNDING),
    ),
    (
        "cylinder patch r 990, 200 arcs",
        "f64",
        Pin::Winds(ROUNDING),
    ),
    (
        "torus patch major 100, 200 arcs",
        "f64",
        Pin::Escalates("props_chart_loop_whole", ROUNDING),
    ),
    (
        "torus patch major 300, 200 arcs",
        "f64",
        Pin::Escalates("props_chart_loop_whole", ROUNDING),
    ),
    (
        "torus patch major 600, 200 arcs",
        "f64",
        Pin::Winds(ROUNDING),
    ),
    (
        "torus patch major 990, 40 arcs",
        "f64",
        Pin::Escalates("props_chart_loop_whole", ROUNDING),
    ),
    (
        "torus patch major 990, 200 arcs",
        "f64",
        Pin::Winds(ROUNDING),
    ),
    (
        "cylinder patch r 50, 6 arcs",
        "interval",
        Pin::Escalates("props_chart_loop_whole", CARRIED),
    ),
    (
        "cylinder patch r 50, 40 arcs",
        "interval",
        Pin::Escalates("props_chart_loop_whole", CARRIED),
    ),
    (
        "cylinder patch r 50, 200 arcs",
        "interval",
        Pin::Escalates("props_chart_loop_whole", CARRIED),
    ),
    (
        "cylinder patch r 213, 6 arcs",
        "interval",
        Pin::Escalates("props_chart_loop_whole", CARRIED),
    ),
    (
        "cylinder patch r 213, 40 arcs",
        "interval",
        Pin::Escalates("props_chart_loop_whole", CARRIED),
    ),
    (
        "cylinder patch r 213, 200 arcs",
        "interval",
        Pin::Escalates("props_chart_loop_whole", CARRIED),
    ),
    (
        "cylinder patch r 410, 1 arcs",
        "interval",
        Pin::Escalates("props_chart_loop_whole", CARRIED),
    ),
    (
        "cylinder patch r 410, 6 arcs",
        "interval",
        Pin::Escalates("props_chart_loop_whole", CARRIED),
    ),
    (
        "cylinder patch r 410, 40 arcs",
        "interval",
        Pin::Escalates("props_chart_junction_closed", BESIDE),
    ),
    (
        "cylinder patch r 410, 200 arcs",
        "interval",
        Pin::Escalates("props_chart_junction_closed", BESIDE),
    ),
    (
        "cylinder patch r 990, 1 arcs",
        "interval",
        Pin::Escalates("props_meridian_on_surface", BESIDE),
    ),
    (
        "cylinder patch r 990, 6 arcs",
        "interval",
        Pin::Escalates("props_meridian_on_surface", BESIDE),
    ),
    (
        "cylinder patch r 990, 40 arcs",
        "interval",
        Pin::Escalates("props_meridian_on_surface", BESIDE),
    ),
    (
        "cylinder patch r 990, 200 arcs",
        "interval",
        Pin::Escalates("props_meridian_on_surface", BESIDE),
    ),
    (
        "cylinder band r 990",
        "interval",
        Pin::Escalates("props_chart_junction_closed", BESIDE),
    ),
    (
        "torus patch major 100, 6 arcs",
        "interval",
        Pin::Escalates("props_chart_loop_whole", CARRIED),
    ),
    (
        "torus patch major 100, 40 arcs",
        "interval",
        Pin::Escalates("props_chart_loop_whole", CARRIED),
    ),
    (
        "torus patch major 100, 200 arcs",
        "interval",
        Pin::Escalates("props_chart_loop_whole", CARRIED),
    ),
    (
        "torus patch major 300, 1 arcs",
        "interval",
        Pin::Escalates("props_chart_loop_whole", CARRIED),
    ),
    (
        "torus patch major 300, 6 arcs",
        "interval",
        Pin::Escalates("props_chart_junction_closed", BESIDE),
    ),
    (
        "torus patch major 300, 40 arcs",
        "interval",
        Pin::Escalates("props_chart_junction_closed", BESIDE),
    ),
    (
        "torus patch major 300, 200 arcs",
        "interval",
        Pin::Escalates("props_chart_junction_closed", BESIDE),
    ),
    (
        "torus patch major 600, 1 arcs",
        "interval",
        Pin::Escalates("props_chart_junction_closed", BESIDE),
    ),
    (
        "torus patch major 600, 6 arcs",
        "interval",
        Pin::Escalates("props_chart_junction_closed", BESIDE),
    ),
    (
        "torus patch major 600, 40 arcs",
        "interval",
        Pin::Escalates("props_chart_junction_closed", BESIDE),
    ),
    (
        "torus patch major 600, 200 arcs",
        "interval",
        Pin::Escalates("props_chart_junction_closed", BESIDE),
    ),
    (
        "torus patch major 990, 1 arcs",
        "interval",
        Pin::Escalates("props_meridian_fit", BESIDE),
    ),
    (
        "torus patch major 990, 6 arcs",
        "interval",
        Pin::Escalates("props_meridian_fit", BESIDE),
    ),
    (
        "torus patch major 990, 40 arcs",
        "interval",
        Pin::Escalates("props_meridian_fit", BESIDE),
    ),
    (
        "torus patch major 990, 200 arcs",
        "interval",
        Pin::Escalates("props_meridian_fit", BESIDE),
    ),
];

/// **Each wall measures its closed form, or at ε = 1e-12 refuses as
/// [`PINS`] records**, with every label checked, so a wall that starts to
/// measure turns its pin red as surely as one that starts to refuse.
#[test]
fn far_tilted_walls_of_many_edges_measure_or_refuse_as_pinned() {
    fn rows<T: Decide + Mid>(scalar: &str, seen: &mut Vec<String>) {
        let mut check = |got: Result<f64, PropsError>, want: f64, what: String| {
            let pin = (eps() <= 1e-12)
                .then(|| PINS.iter().find(|(w, s, _)| *w == what && *s == scalar))
                .flatten();
            seen.push(format!("{what} at {scalar}"));
            match (pin, got) {
                (None, Ok(a)) => assert!(
                    (a - want).abs() <= 1e-9 * want,
                    "{what} at {scalar}: area {a} against {want}"
                ),
                (None, Err(e)) => panic!("{what} at {scalar}: {e:?}"),
                (Some((_, _, Pin::Winds(row))), got) => assert_eq!(
                    got,
                    Err(PropsError::LoopsWindChart),
                    "{what} at {scalar}, pinned by {row}"
                ),
                (Some((_, _, Pin::Escalates(predicate, row))), got) => assert!(
                    matches!(
                        &got,
                        Err(PropsError::Escalated { cause, .. })
                            if cause.predicate == Some(*predicate)
                    ),
                    "{what} at {scalar}, pinned to escalate {predicate} by {row}: {got:?}"
                ),
            }
        };
        for r in [50.0, 213.0, 410.0, 990.0] {
            let (span, height) = (1.0, 0.7);
            for n in [1, 6, 40, 200] {
                let l = cylinder_patch::<T>(r, span, height, n);
                check(
                    measure(&cylinder::<T>(r), &[&l]),
                    r * span * height,
                    format!("cylinder patch r {r}, {n} arcs"),
                );
            }
            let (lo, hi) = (
                [rim::<T>(r, 0.0, 0.0, TAU)],
                [rim::<T>(r, height, TAU, 0.0)],
            );
            check(
                measure(&cylinder::<T>(r), &[&lo, &hi]),
                r * TAU * height,
                format!("cylinder band r {r}"),
            );
        }
        for major in [100.0, 300.0, 600.0, 990.0] {
            let minor = 2.5;
            let (v0, v1) = (0.2_f64, 0.7_f64);
            for n in [1, 6, 40, 200] {
                let l = torus_patch::<T>(major, minor, 1.0, (v0, v1), n);
                check(
                    measure(&torus::<T>(major, minor), &[&l]),
                    minor * (major * (v1 - v0) + minor * (v1.sin() - v0.sin())),
                    format!("torus patch major {major}, {n} arcs"),
                );
            }
        }
    }
    let mut seen = Vec::new();
    rows::<f64>("f64", &mut seen);
    rows::<geom_core::Interval>("interval", &mut seen);
    for (what, scalar, _) in PINS {
        assert!(
            seen.contains(&format!("{what} at {scalar}")),
            "the pin {what} at {scalar} names no wall"
        );
    }
}

/// **A hand-built far wall whose rim does not turn as its ends say
/// refuses before its whole turns are asked**, at both scalars and every
/// ε: a rim tilted off the axis, a rim whose span runs a whole turn past
/// its ends, a rim on the opposite normal walked the wrong way, a rim
/// short of its ruling. A rim's ends are its carrier at its stored span,
/// so a stored turn cannot disagree with them through any door, and what
/// disagrees refuses on its own reading (the tilt, the span, a junction
/// left open). The same opposite-normal circle walked the right way
/// measures.
#[test]
fn a_hand_built_far_wall_whose_rim_disagrees_refuses_before_its_turns() {
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
        let early = |what: &str, got: Result<f64, PropsError>, names: &[&str]| {
            let named = match &got {
                Err(PropsError::OffSurface { what } | PropsError::NotIsoRectangle { what }) => {
                    names.contains(what)
                }
                Err(PropsError::Escalated { cause, .. }) => {
                    cause.predicate.is_some_and(|p| names.contains(&p))
                }
                Err(PropsError::ChartLoopOpen) => names.contains(&"ChartLoopOpen"),
                _ => false,
            };
            assert!(
                named,
                "a rim {what} at {scalar}, refused as one of {names:?}: {got:?}"
            );
        };
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
            early(
                &format!("tilted {tilt}"),
                with_bottom(tilted),
                &["props_cylinder_circle_tilt", "props_rim_axis_parallel"],
            );
        }
        early(
            "a whole turn past its ends",
            with_bottom(rim::<T>(r, 0.0, 0.0, span + TAU)),
            &["props_rim_span_winding"],
        );
        early(
            "on the opposite normal, walked the wrong way",
            with_bottom(flipped(0.0, span)),
            &["ChartLoopOpen"],
        );
        early(
            "short of its ruling",
            with_bottom(rim::<T>(r, 0.0, 0.0, span - 1e-3)),
            &["ChartLoopOpen"],
        );
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}
