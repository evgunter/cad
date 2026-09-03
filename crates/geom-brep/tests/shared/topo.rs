//! Hand-built `LoopEdge`s: the interval-ordering wrapper every suite
//! that authors a loop by hand carries, and the two circles on a polar
//! sphere that the rim/great-circle suites cut their loops from.
//!
//! **What this module does NOT absorb.** `mesh10r2_probes.rs`'s `edge`
//! takes a carrier id and builds the struct literally rather than
//! through `LoopEdge::hand_built`, because stamping the id is the row's
//! subject; it says so at the site. The rims that are not a polar
//! sphere's stay with their suites and say why —
//! `iso_rectangle_door.rs`'s cylinder rim (its parameter is a HEIGHT,
//! not a latitude), `imported_chart_arc_rim.rs`'s bare `Curve3` and
//! `revolved_point_anchor.rs`'s `MappedCurve`, which share the letters
//! and nothing else.

use geom::Curve3;
use geom_brep::props::LoopEdge;
use geom_core::Real;

use crate::shared::point::{p3, v3};

/// One traversed boundary edge `a → b` on its carrier.
///
/// The loop's direction is the argument order; the stored interval is
/// always `t0 <= t1` with `forward` carrying the traversal sense, which
/// is the half-edge convention `props` expects.
pub(crate) fn edge<T: Real>(
    carrier: Curve3<T>,
    a: f64,
    b: f64,
    start: u32,
    end: u32,
) -> LoopEdge<T> {
    let (t0, t1, forward) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge::hand_built(
        carrier,
        T::from_f64(t0),
        T::from_f64(t1),
        forward,
        start,
        end,
    )
}

/// The rim of a polar sphere of radius `radius` at latitude `v`,
/// traversed in azimuth from `u0` to `u1`: the axis-parallel circle at
/// height `radius·sin v` with radius `radius·cos v`.
pub(crate) fn sphere_rim<T: Real>(
    radius: f64,
    v: f64,
    u0: f64,
    u1: f64,
    a: u32,
    b: u32,
) -> LoopEdge<T> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, radius * v.sin()),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(radius * v.cos()),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
    )
}

/// The great circle of a polar sphere of radius `radius` whose plane
/// contains the axis at azimuth `u`, traversed from `t0` to `t1`; its
/// parameter is the latitude.
pub(crate) fn sphere_great<T: Real>(
    radius: f64,
    u: f64,
    t0: f64,
    t1: f64,
    a: u32,
    b: u32,
) -> LoopEdge<T> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: T::from_f64(radius),
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        t0,
        t1,
        a,
        b,
    )
}

/// The rim of a torus at minor angle `v`: the coaxial circle at height
/// `minor·sin v` with radius `major + minor·cos v`.
///
/// **A curve, where the sphere pair above are edges.** The three suites
/// that build this cut it into a `LoopEdge` through THEIR OWN `edge` —
/// `mesh10r2_probes.rs`'s stamps a carrier id and the other two do not
/// — so the shared part stops at the circle.
pub(crate) fn torus_rim_circle<T: Real>(major: f64, minor: f64, v: f64) -> Curve3<T> {
    Curve3::Circle {
        center: p3(0.0, 0.0, minor * v.sin()),
        axis: v3(0.0, 0.0, 1.0),
        radius: T::from_f64(major + minor * v.cos()),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The meridian of a torus at azimuth `u`: the minor circle centred on
/// the spine at `major·(cos u, sin u, 0)`, in the plane containing the
/// axis at that azimuth. See [`torus_rim_circle`] on why this is a
/// curve and not an edge.
pub(crate) fn torus_meridian_circle<T: Real>(major: f64, minor: f64, u: f64) -> Curve3<T> {
    Curve3::Circle {
        center: p3(major * u.cos(), major * u.sin(), 0.0),
        axis: v3(u.sin(), -u.cos(), 0.0),
        radius: T::from_f64(minor),
        u_ref: v3(u.cos(), u.sin(), 0.0),
    }
}
