//! GERM VERBS-CONE U1 and U2: the certified root lanes for line and
//! circle edges against a cone face, read at the reduction sweep.
//!
//! The cone is off the operand roster until the flip, so every row runs
//! the sweep through `topo::test_support::sweep_past_the_cone_roster`,
//! which lets the cone past the roster and keeps every other gate. A
//! row reads the vertex-on-face contacts the sweep recorded on the cone
//! face, or its refusal.
//!
//! The cone is the spec's preview cone: the triangle `(0,0) (1,0) (0,1)`
//! revolved fully about `y`, merged to one cone face (apex `(0, 1, 0)`,
//! base radius `1` at `y = 0`, half-angle `π/4`, so the lateral face is
//! `ρ = 1 − y`). Its mirror, the same triangle through `y ↦ −y`, has its
//! apex at `(0, −1, 0)` and puts every row's roots in the other solver
//! order along the same edges.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Point2, Point3, Tol};
use profile::{ProfileLoop, RawLoop};
use revolve_common::*;
use sweep::{Revolution, revolve};
use topo::test_support::{brick, prism_z, sweep_past_the_cone_roster};
use topo::{Body, BooleanError, FaceKey};

/// The preview cone, apex up (`flip = false`) or apex down.
fn cone(flip: bool) -> Body<f64> {
    let apex = if flip { -1.0 } else { 1.0 };
    let mut body = revolve(
        &validated(vec![ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(0.0, apex),
        ])]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    body.merge_coplanar_faces(Tol::witness()).unwrap();
    assert_all_tiers(&body);
    body
}

fn cone_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cone { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// A box, mirrored through `y ↦ −y` when `flip`.
fn bar(x: (f64, f64), y: (f64, f64), z: (f64, f64), flip: bool) -> Body<f64> {
    let y = if flip { (-y.1, -y.0) } else { y };
    brick(x, y, z, Tol::witness())
}

/// The sweep's A-on-B contacts on `b`'s cone face, sorted by `y`.
fn pierces(a: &Body<f64>, b: &Body<f64>) -> Result<Vec<Point3<f64>>, BooleanError> {
    let (_, [a_on_b, _]) = sweep_past_the_cone_roster(a, b, Tol::witness())?;
    let walls = cone_faces(b);
    let mut ps: Vec<Point3<f64>> = a_on_b
        .into_iter()
        .filter(|(_, f)| walls.contains(f))
        .map(|(p, _)| p)
        .collect();
    ps.sort_by(|p, q| p.y.total_cmp(&q.y).then(p.x.total_cmp(&q.x)));
    Ok(ps)
}

/// Every point on the lateral face `ρ = 1 − |y|` of the (flipped) cone
/// within `1e-9`, on the solid's own nappe.
fn assert_on_face(label: &str, ps: &[Point3<f64>], flip: bool) {
    for p in ps {
        let rho = p.x.hypot(p.z);
        let h = if flip { -p.y } else { p.y };
        assert!(
            (0.0..1.0).contains(&h) && (rho - (1.0 - h)).abs() < 1e-9,
            "{label}: {p:?} is on the solid's lateral face"
        );
    }
}

/// **P3, the apex pin: four real crossings, recorded as none on main.**
/// Each long edge has one end inside each nappe, so the convexity arm
/// read `(Negative, Negative)` as no event. The roots put one crossing
/// on the face at `y = 1 − ρ` and one on the mirror nappe at `1 + ρ`,
/// which the trim places `Out`; the flipped fixture takes the same
/// edges in the other solver order, so on one of the two every edge
/// meets its `Out` root first.
#[test]
fn the_apex_pin_crosses_the_face_four_times() {
    for flip in [false, true] {
        let b = cone(flip);
        let a = bar((-0.02, 0.02), (0.5, 1.5), (0.02, 0.06), flip);
        let ps = pierces(&a, &b).unwrap_or_else(|e| panic!("flip {flip}: refused {e:?}"));
        assert_eq!(ps.len(), 4, "flip {flip}: four pierces {ps:?}");
        assert_on_face("apex pin", &ps, flip);
        let mut rhos: Vec<f64> = ps.iter().map(|p| p.x.hypot(p.z)).collect();
        rhos.sort_by(f64::total_cmp);
        let near = 0.02_f64.hypot(0.02);
        let far = 0.02_f64.hypot(0.06);
        for (got, want) in rhos.iter().zip([near, near, far, far]) {
            assert!((got - want).abs() < 1e-9, "flip {flip}: ρ {got} vs {want}");
        }
    }
}

/// A box wholly inside the solid, near the axis: every edge has both
/// ends inside one nappe, the segment stays inside, and no event is
/// recorded — through the roots, which put both crossings off the span.
#[test]
fn a_box_inside_one_nappe_records_nothing() {
    for flip in [false, true] {
        let b = cone(flip);
        let a = bar((-0.01, 0.01), (0.2, 0.4), (-0.01, 0.01), flip);
        let ps = pierces(&a, &b).unwrap_or_else(|e| panic!("flip {flip}: refused {e:?}"));
        assert!(ps.is_empty(), "flip {flip}: no pierce {ps:?}");
    }
}

/// **The belly chord**: a bar through the cone at `y = 0.5`, `z = 0.2`,
/// each long edge outside at both ends and through the solid between.
/// Red on main: `CurvedPierceUnsupported`, the `f2` door.
#[test]
fn a_belly_chord_pierces_the_face_twice_per_edge() {
    for flip in [false, true] {
        let b = cone(flip);
        let a = bar((-2.0, 2.0), (0.49, 0.51), (0.19, 0.21), flip);
        let ps = pierces(&a, &b).unwrap_or_else(|e| panic!("flip {flip}: refused {e:?}"));
        assert_eq!(ps.len(), 8, "flip {flip}: two pierces per long edge {ps:?}");
        assert_on_face("belly", &ps, flip);
    }
}

/// **P4's brick**, clear of the cone with the boxes overlapping: no
/// event. Red on main: the `f2` door.
#[test]
fn a_brick_clear_of_the_cone_records_nothing() {
    for flip in [false, true] {
        let b = cone(flip);
        let a = bar((-0.3, 0.3), (0.2, 0.3), (0.9, 1.2), flip);
        let ps = pierces(&a, &b).unwrap_or_else(|e| panic!("flip {flip}: refused {e:?}"));
        assert!(ps.is_empty(), "flip {flip}: no pierce {ps:?}");
    }
}

/// The door a row expects, naming a cone face of `b`.
fn assert_pierce_door(label: &str, got: Result<Vec<Point3<f64>>, BooleanError>, b: &Body<f64>) {
    match got {
        Err(BooleanError::CurvedPierceUnsupported { face, .. }) => {
            assert!(cone_faces(b).contains(&face), "{label}: the cone face");
        }
        other => panic!("{label}: expected the pierce door, got {other:?}"),
    }
}

/// **An edge through the apex** has a double root there, and the apex
/// has no tangent plane: the door, never no event. The bar's corner edge
/// runs up the axis, one end inside each nappe.
#[test]
fn an_edge_through_the_apex_keeps_the_door() {
    for flip in [false, true] {
        let b = cone(flip);
        let a = bar((0.0, 0.05), (0.5, 1.5), (0.0, 0.05), flip);
        assert_pierce_door(&format!("flip {flip}"), pierces(&a, &b), &b);
    }
}

/// **An edge parallel to a generator that crosses the face** keeps the
/// door (Q3): the quadratic degenerates to a line, and its one root is
/// not taken. The prism's two slanted edges run along `(1, −1, 0)`,
/// parallel to the generator at azimuth `0`, from outside the cone at
/// the apex height to inside it.
#[test]
fn an_edge_parallel_to_a_generator_keeps_the_door() {
    let b = cone(false);
    let a = prism_z(
        &[(-0.2, 1.0), (0.3, 0.5), (0.3, 1.0)],
        -0.05,
        0.05,
        Tol::witness(),
    )
    .body;
    assert_pierce_door("generator-parallel", pierces(&a, &b), &b);
}

/// A cylinder revolved from the rectangle `x ∈ [x0, x1]`, `y ∈ [y0, y1]`
/// of the sketch plane about the sketch axis through `origin` along
/// `dir`, mirrored through `y ↦ −y` when `flip`.
fn pin(x: (f64, f64), y: (f64, f64), origin: (f64, f64), dir: (f64, f64), flip: bool) -> Body<f64> {
    let m = |v: f64| if flip { -v } else { v };
    let lp = ProfileLoop::polygon([
        Point2::new(x.0, m(y.0)),
        Point2::new(x.1, m(y.0)),
        Point2::new(x.1, m(y.1)),
        Point2::new(x.0, m(y.1)),
    ]);
    let mut body = revolve(
        &validated(vec![lp]),
        sweep::RevolveAxis {
            origin: Point2::new(origin.0, m(origin.1)),
            // The reflection reverses the sketch's handedness: the axis
            // is reversed with it, so the profile stays on its side.
            dir: geom_core::Vec2::new(if flip { -dir.0 } else { dir.0 }, dir.1),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    body.merge_coplanar_faces(Tol::witness()).unwrap();
    assert_all_tiers(&body);
    body
}

/// **Coaxial rims are clear.** The spec's P10 pin, `r = 0.1`,
/// `y ∈ [−0.5, 0.5]`, on the cone's axis: its top rim is inside the solid
/// and its bottom rim inside the carrier's continuation below the base,
/// each at a constant elevation. Red on main: `CurvedPierceUnsupported`
/// at the circle rung, which has no enclosure form for a cone.
#[test]
fn coaxial_rims_are_clear() {
    for flip in [false, true] {
        let b = cone(flip);
        let a = pin((0.0, 0.1), (-0.5, 0.5), (0.0, 0.0), (0.0, 1.0), flip);
        let ps = pierces(&a, &b).unwrap_or_else(|e| panic!("flip {flip}: refused {e:?}"));
        assert!(
            ps.is_empty(),
            "flip {flip}: no pierce on the cone face {ps:?}"
        );
    }
}

/// **A parallel lying ON the cone keeps the door**: a coaxial pin whose
/// bottom rim, `r = 0.5` at `y = 0.5`, is a parallel of the cone.
#[test]
fn a_rim_on_the_cone_keeps_the_door() {
    let b = cone(false);
    let a = pin((0.0, 0.5), (0.5, 0.8), (0.0, 0.0), (0.0, 1.0), false);
    assert_pierce_door("a parallel on the cone", pierces(&a, &b), &b);
}

/// **A parallel-axis rim crossing the lateral face**: a pin about the
/// line `x = 0.3`, `r = 0.1`, `y ∈ [0.2, 0.65]`. Its top rim sweeps
/// `ρ ∈ [0.2, 0.4]` across the parallel `ρ = 0.35` and crosses it twice,
/// at the closed-form points; its bottom rim is inside; its seam, at
/// `x = 0.4`, crosses at `y = 0.6`. Red on main: the circle rung's door.
#[test]
fn a_parallel_axis_rim_crosses_the_face_at_the_closed_form_points() {
    let b = cone(false);
    let a = pin((0.3, 0.4), (0.2, 0.65), (0.3, 0.0), (0.0, 1.0), false);
    let ps = pierces(&a, &b).unwrap_or_else(|e| panic!("refused {e:?}"));
    assert_on_face("parallel-axis pin", &ps, false);
    let rim: Vec<&Point3<f64>> = ps.iter().filter(|p| (p.y - 0.65).abs() < 1e-9).collect();
    assert_eq!(rim.len(), 2, "two rim pierces at y = 0.65: {ps:?}");
    // `(x − 0.3)² + z² = 0.01` and `x² + z² = 0.35²`.
    let x = (0.35_f64.powi(2) - 0.01 + 0.09) / 0.6;
    let z = (0.35_f64.powi(2) - x * x).sqrt();
    for p in rim {
        assert!(
            (p.x - x).abs() < 1e-9 && (p.z.abs() - z).abs() < 1e-9,
            "the closed-form crossing ({x}, ±{z}): {p:?}"
        );
    }
}

/// **Rims crossing both nappes, in both solver orders.** A short pin
/// about the line `y = 1`, `z = 0` (through the apex height, along `x`),
/// `r = 0.3`, `x ∈ [0.1, 0.15]`: each rim meets the double cone four
/// times, twice on the face and twice on the mirror nappe, which the
/// trim places `Out`. The flipped fixture takes the rims the other way
/// round. Four pierces, all on the face, at `ρ² = x₀² + r² sin²θ` where
/// `r|cos θ| = ρ`.
#[test]
fn rims_through_both_nappes_pierce_the_face_twice_each() {
    for flip in [false, true] {
        let b = cone(flip);
        let a = pin((0.1, 0.15), (0.7, 1.0), (0.0, 1.0), (1.0, 0.0), flip);
        let ps = pierces(&a, &b).unwrap_or_else(|e| panic!("flip {flip}: refused {e:?}"));
        assert_eq!(ps.len(), 4, "flip {flip}: two per rim {ps:?}");
        assert_on_face("both nappes", &ps, flip);
        let mut rhos: Vec<f64> = ps.iter().map(|p| p.x.hypot(p.z)).collect();
        rhos.sort_by(f64::total_cmp);
        // `ρ² = x₀² + r²(1 − cos²θ) = r² cos²θ` ⇒ `ρ² = (x₀² + r²)/2`.
        let want = |x0: f64| ((x0 * x0 + 0.09) / 2.0).sqrt();
        for (got, w) in rhos
            .iter()
            .zip([want(0.1), want(0.1), want(0.15), want(0.15)])
        {
            assert!((got - w).abs() < 1e-9, "flip {flip}: ρ {got} vs {w}");
        }
    }
}

/// **A chord through the mirror nappe only**, its box overlapping the
/// face's: a thin strip rising along `(1, 0.5, 0)` from `(−0.8, 0.9)`
/// to `(1, 1.8)`, outside the cone at both ends, whose long edges cross
/// the mirror nappe twice (a line outside the aperture meets one nappe
/// only). Every root is on the carrier and `Out` of the face: no event.
#[test]
fn a_chord_through_the_mirror_nappe_only_records_nothing() {
    let b = cone(false);
    let a = prism_z(
        &[(-0.8, 0.9), (1.0, 1.8), (1.0, 1.82), (-0.8, 0.92)],
        0.1,
        0.12,
        Tol::witness(),
    )
    .body;
    let ps = pierces(&a, &b).unwrap_or_else(|e| panic!("refused {e:?}"));
    assert!(ps.is_empty(), "no pierce on the face {ps:?}");
}
