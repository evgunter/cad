//! Shared fixtures and helpers for the profile integration suites.
//!
//! Fixtures are built at `f64` and lifted to other scalars via
//! [`lift`]; geometry that must sit at an ε-relative margin takes the
//! run's ε explicitly so the multi-ε CI rows genuinely re-exercise the
//! bands. The run tolerance comes from `Tolerance::get()` — one read
//! per test process (each integration binary is its own process, so the
//! geom-core global-state discipline is satisfied).
#![allow(dead_code)]

use geom_core::{Point2, Real, Tolerance};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};

/// The run's tolerance (env-driven; the multi-ε matrix parameterizes
/// it).
pub fn tol() -> Tolerance {
    Tolerance::get()
}

/// tan(π/8) = √2 − 1: the bulge of a counterclockwise quarter-circle
/// arc.
pub fn quarter_bulge() -> f64 {
    std::f64::consts::SQRT_2 - 1.0
}

/// Lifts an `f64` profile to any scalar (exact embedding per
/// `Real::from_f64`).
pub fn lift<T: Real>(p: &Profile<f64>) -> Profile<T> {
    Profile::new(
        SketchPlane::xy(),
        p.loops
            .iter()
            .map(|lp| {
                ProfileLoop::new(
                    lp.vertices
                        .iter()
                        .map(|v| ProfileVertex {
                            pos: Point2::new(T::from_f64(v.pos.x), T::from_f64(v.pos.y)),
                            bulge: T::from_f64(v.bulge),
                        })
                        .collect(),
                )
            })
            .collect(),
    )
}

/// A loop from `(x, y, bulge)` triples.
pub fn chain(vs: &[(f64, f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::new(
        vs.iter()
            .map(|&(x, y, bulge)| ProfileVertex {
                pos: Point2::new(x, y),
                bulge,
            })
            .collect(),
    )
}

/// A single-loop profile on the world xy-plane.
pub fn profile(loops: Vec<ProfileLoop<f64>>) -> Profile<f64> {
    Profile::new(SketchPlane::xy(), loops)
}

/// An axis-aligned rectangle, counterclockwise from `(x0, y0)`.
pub fn rect(x0: f64, y0: f64, w: f64, h: f64) -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(x0, y0),
        Point2::new(x0 + w, y0),
        Point2::new(x0 + w, y0 + h),
        Point2::new(x0, y0 + h),
    ])
}

/// The L-profile (counterclockwise hexagon).
pub fn l_profile() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(2.0, 1.0),
        Point2::new(1.0, 1.0),
        Point2::new(1.0, 2.0),
        Point2::new(0.0, 2.0),
    ])
}

/// A circle as two counterclockwise semicircular arcs, split
/// horizontally (vertices at (cx ± r, cy)).
pub fn circle_h(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    chain(&[(cx - r, cy, 1.0), (cx + r, cy, 1.0)])
}

/// A circle as two counterclockwise semicircular arcs, split
/// vertically (vertices at (cx, cy ± r)) — used when a fixture needs
/// the points (cx ± r, cy) free of vertices.
pub fn circle_v(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    chain(&[(cx, cy - r, 1.0), (cx, cy + r, 1.0)])
}

/// A rounded rectangle: straight sides, counterclockwise quarter-arc
/// corners of radius `r`.
pub fn rounded_rect(w: f64, h: f64, r: f64) -> ProfileLoop<f64> {
    let b = quarter_bulge();
    chain(&[
        (r, 0.0, 0.0),
        (w - r, 0.0, b),
        (w, r, 0.0),
        (w, h - r, b),
        (w - r, h, 0.0),
        (r, h, b),
        (0.0, h - r, 0.0),
        (0.0, r, b),
    ])
}

/// A lens (lune): a semicircular arc out and a shallower arc back —
/// the legal two-vertex loop with distinct carriers.
pub fn lens() -> ProfileLoop<f64> {
    // Out: counterclockwise semicircle (apex below the chord);
    // back: clockwise quarter arc (apex also below, shallower).
    chain(&[(0.0, 0.0, 1.0), (2.0, 0.0, -quarter_bulge())])
}

/// The annulus: circle outer + concentric circle hole.
pub fn annulus() -> Profile<f64> {
    profile(vec![circle_h(0.0, 0.0, 2.0), circle_h(0.0, 0.0, 1.0)])
}

/// Bowtie: a self-crossing quadrilateral (segments 0 and 2 cross).
pub fn bowtie() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
    ])
}

/// Internally tangent circle-in-circle hole: exact tangency at (1, 0),
/// interior to arcs of both loops (both circles split vertically).
pub fn tangent_hole() -> Profile<f64> {
    profile(vec![circle_v(0.0, 0.0, 1.0), circle_v(0.5, 0.0, 0.5)])
}

/// An arc kissing a line: the top arc of this profile dips down and
/// touches the bottom segment tangentially at (2, 0), interior to
/// both.
pub fn arc_kisses_line() -> Profile<f64> {
    profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (4.0, 0.0, 0.0),
        (4.0, 3.0, 0.0),
        (3.0, 3.0, -3.0),
        (1.0, 3.0, 0.0),
        (0.0, 3.0, 0.0),
    ])])
}

/// A near-tangent circle-in-circle hole: the internal clearance
/// d − |r₁ − r₂| is exactly −5ε — inside the ambiguity band (ε, Kε), so
/// validation must escalate rather than guess.
pub fn near_tangent_hole(eps: f64) -> Profile<f64> {
    profile(vec![
        circle_v(0.0, 0.0, 1.0),
        circle_v(0.5 - 5.0 * eps, 0.0, 0.5),
    ])
}
