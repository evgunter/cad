//! **The containment reading answers four ways on parameters a
//! crossing's ON reading admits.** A measurement row for
//! `work/scalar/a-crossing-parameters-check-bounds-across-the-face-and-its-readers-decide-along-the-edge.md`:
//! it pins today's behaviour.
//!
//! The arc is the quadratic Bézier `x(t) = 2t`, `y(t) = 4t(1 − t)·H` in
//! `z = 0`. It rises through the brick's face `y = y(¼)` at `x = ½` with
//! slope `H`, a crossing angle of about 0.57°, so every parameter whose
//! point lies within `zero / H` along `x` of the true crossing reads ON
//! the face's plane: each is a parameter `geom-brep`'s
//! `spline_composite_root_on` admits. The face's trim edge `x = x₁`
//! stands two escalation lengths past the true crossing.
//!
//! The reading is [`topo::contfp`], what the sweep's plane arm asks of a
//! spline edge's crossing (`boolean::reduce`'s `contfp_decided` on
//! `curve.carrier().eval(t)`). At the true crossing it answers `In` (the
//! sweep splits the arc and records a vertex on the face); over the trim
//! edge `OnEdge` (it splits both edges and records a vertex pair); five
//! zero bands short of the trim edge it escalates; three escalation
//! lengths past it `Out` (no event of this face). The points are
//! hand-picked inside the admissible stretch: what the sweep proposes
//! there is `geom-brep`'s `composite::crossing_parameter_rows`.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol, Vec3};
use topo::test_support::brick;
use topo::{Body, FaceContainment, contfp};

/// The arc's height parameter, and its slope where it crosses the face.
const H: f64 = 1e-2;

/// The arc's closed form at `t`.
fn arc(t: f64) -> Point3<f64> {
    Point3::new(2.0 * t, 4.0 * t * (1.0 - t) * H, 0.0)
}

#[test]
fn containment_answers_four_ways_on_parameters_the_on_reading_admits() {
    let band = Band::linear(Tol::witness()).expect("the run's band");
    let (zero, escalate) = (band.zero(), band.escalate());
    // The brick whose face `y = y(¼)`, facing `−y`, spans
    // `x ∈ [−½, x₁]`, `z ∈ [−1, 1]`.
    let (x1, y) = (0.5 + 2.0 * escalate, arc(0.25).y);
    let b: Body<f64> = brick((-0.5, x1), (y, y + 3.0), (-1.0, 1.0), Tol::witness());
    let (face, plane) = b
        .faces()
        .find_map(|(k, _)| {
            let plane = topo::face_normal::face_plane(&b, k)?;
            (plane.normal.dot(Vec3::unit_y()) < -0.5).then_some((k, plane))
        })
        .expect("the brick has the face");
    let read = |x: f64| {
        let p = arc(0.5 * x);
        let off = (p - plane.origin).dot(plane.normal).abs();
        assert!(
            off <= zero && (x - 0.5).abs() * H <= zero,
            "x = {x}: the arc's point reads ON the plane, inside the admissible stretch: {off}"
        );
        contfp(&b, face, plane.normal, p, band)
    };
    let at_truth = read(0.5);
    assert!(
        matches!(at_truth, Ok(FaceContainment::In)),
        "the true crossing: {at_truth:?}"
    );
    let over_edge = read(x1);
    assert!(
        matches!(over_edge, Ok(FaceContainment::OnEdge(_))),
        "over the trim edge: {over_edge:?}"
    );
    let in_gap = read(x1 - 5.0 * zero);
    assert!(in_gap.is_err(), "short of the trim edge: {in_gap:?}");
    let past = read(x1 + 3.0 * escalate);
    assert!(
        matches!(past, Ok(FaceContainment::Out)),
        "past the trim edge: {past:?}"
    );
}
