//! **The census's instance-containment arm must bound the CONTAINING
//! solid by a superset, not by its vertices** (SMELL-SCAN §S16 sweep).
//!
//! Arm 2 of the cross-solid backstop clears a pair when some extent
//! margin is definitely negative — when the inner solid pokes out of
//! the outer one. That reading is sound only if the OUTER box contains
//! the outer solid's whole locus. Built from vertices it does not: a
//! curved solid's vertex hull is an inscribed polytope, so a body
//! sitting between the inscribed hull and the true surface pokes out
//! of the hull while being entirely INSIDE the solid, and the backstop
//! whose whole job is "decide or refuse, never silently not-examine"
//! silently clears it.
//!
//! The fixture is the extruded three-arc cylinder (radius 0.5, six
//! vertices at 0°/120°/240° on two caps). Its vertex hull is the
//! inscribed triangular prism, whose x extent starts at −0.25 while
//! the cylinder's starts at −0.5. Every nested probe below lives in
//! that annulus.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tolerance, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, ContactRecords, EntityId, ValidationError, validate_pseudomanifold};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The three-arc cylinder: radius 0.5 about the z axis, `z ∈ [0, 1]`.
/// Six vertices; the hull of them is the inscribed triangular prism,
/// `x ∈ [−0.25, 0.5]`, `y ∈ [−0.433, 0.433]`.
fn cylinder() -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = deg.to_radians();
        p2(0.5 * th.cos(), 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0)).unwrap().body
}

/// A small axis-aligned box of half-width `h` centred at `(cx, 0, 0.5)`.
fn nested_box(cx: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        [(cx - h, -h), (cx + h, -h), (cx + h, h), (cx - h, h)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    // Lifted clear of both caps: z in [0.3, 0.7] inside the
    // cylinder's [0, 1], so the pair is nested, never touching.
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.3)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(0.4)).unwrap().body
}

/// The nested pair as one two-instance arena.
fn assembly(outer: &Body<f64>, inner: &Body<f64>) -> Body<f64> {
    let mut out = outer.clone();
    topo::graft_disjoint(&mut out, inner).unwrap();
    out
}

/// **The regression row.** A body wholly inside the cylinder, but
/// outside the hull of the cylinder's six vertices, must be REFUSED as
/// the C6 interference class — never cleared.
///
/// `cx` is swept across the whole annulus between the inscribed hull's
/// face (x = −0.25) and the true wall (x = −0.5), so the row goes red
/// for any rule that recovers only part of the outer solid's extent,
/// not merely for the one offset that happens to break the vertex
/// hull. Every probe is checked to be genuinely nested first, so a
/// refusal can never be explained by the probe poking out for real.
#[test]
fn a_body_nested_inside_a_curved_solid_is_never_silently_cleared() {
    let outer = cylinder();
    let h = 0.05;
    for &cx in &[-0.22_f64, -0.26, -0.30, -0.35, -0.40] {
        // Genuinely inside the cylinder: the far corner is within r.
        let far = ((cx.abs() + h).powi(2) + h * h).sqrt();
        assert!(
            far < 0.5,
            "probe at {cx} is not nested (corner radius {far} ≥ 0.5)"
        );
        let inner = nested_box(cx, h);
        let body = assembly(&outer, &inner);
        // Tier 3 alone cannot see it — that is the #382 finding, and
        // the reason arm 2 exists at all.
        assert_eq!(
            topo::validate_geometric(&body),
            Ok(()),
            "probe at {cx}: tier 3 does not compare solids"
        );
        let errors = validate_pseudomanifold(&body, &ContactRecords::default())
            .expect_err("a nested instance must refuse, never clear");
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::CensusUndecidable {
                    a: EntityId::Solid(_),
                    b: EntityId::Solid(_),
                    ..
                }
            )),
            "probe at {cx}: the containment arm must name the solid pair, got {errors:?}"
        );
    }
}

/// The other direction, so the row above cannot pass by refusing
/// everything: a body clearly OUTSIDE the cylinder, and outside its
/// reach box, is still cleared by the containment arm.
#[test]
fn a_body_beside_the_cylinder_is_still_cleared_by_containment() {
    let outer = cylinder();
    let beside = nested_box(3.0, 0.2);
    let body = assembly(&outer, &beside);
    let verdict = validate_pseudomanifold(&body, &ContactRecords::default());
    let containment: Vec<&ValidationError> = match &verdict {
        Ok(()) => Vec::new(),
        Err(errors) => errors
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    ValidationError::CensusUndecidable {
                        a: EntityId::Solid(_),
                        b: EntityId::Solid(_),
                        ..
                    }
                )
            })
            .collect(),
    };
    assert!(
        containment.is_empty(),
        "a separated pair must not refuse as containment: {containment:?}"
    );
}
