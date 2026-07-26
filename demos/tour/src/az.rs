//! The A×Z silhouette intersect — issue #93's acceptance case, in the
//! tour at last. The #91 investigation probed this exact pair and it
//! refused typed (`JoinDesync`: "neither section loop's regions hold a
//! classifiable vertex") — Z's lower bay crossing A's crossbar-bottom
//! strip completes an isolated 6-vertex seam hexagon whose flanking
//! regions are bounded ENTIRELY by seam vertices, defeating the
//! vertex-only role anchor. #108's edge-midpoint/region-interior
//! anchor tiers closed the gap; this scene is the counter-hole variant
//! (A's triangular counter as a TRUE inner loop — the genus-1
//! showpiece) built through the same public profile/sweep/boolean API
//! as every other stop, gated on the independently re-derived exact
//! oracle 880383/327680 (pinned in
//! `crates/sweep/tests/issue93_az_intersect.rs`).
//!
//! Standalone render only (`montage: false`) — the montage sheet is
//! unchanged; this is the post-#108 victory lap, not a new panel.
//!
//! And the victory lap found the NEXT gap (#111, demo-driven
//! development doing its job again): the exact kernel is green — tiers
//! 1/2/3′ pass, the volume is the exact oracle — but the planar CDT's
//! centroid-parity filter keeps an exterior needle triangle where the
//! boolean's seam vertices land 1 ulp off an existing boundary carrier
//! line (Z's slope-3/5 diagonal arithmetic is non-dyadic), so
//! `check_mesh` refuses `BoundaryEdge` on the tessellation. That
//! refusal is PINNED here via `SceneBody::mesh_gap` (retire-on-closure,
//! the #106 pattern); the render rides our own STEP export, which is
//! untouched by the mesh lane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::Body;

use crate::booleans::{check, expect_seamed, try_intersect};
use crate::{SceneBody, Stop, View};

/// Exact volume oracle: 880383/327680 (counter-hole A × Z), derived by
/// independent exact-fraction integration for the #93 acceptance test
/// (`crates/sweep/tests/issue93_az_intersect.rs`) and re-confirmed in
/// the #108 review.
const V_AZ: f64 = 880383.0 / 327680.0;

/// Slanted-stroke A outline: feet at y = 0, crossbar band
/// y ∈ [1, 1.4375], flat apex at y = 2.5 (same fixture as the #93
/// acceptance test).
const A_OUTLINE: [(f64, f64); 8] = [
    (0.0, 0.0),
    (0.625, 0.0),
    (0.8125, 1.0),
    (1.1875, 1.0),
    (1.375, 0.0),
    (2.0, 0.0),
    (1.125, 2.5),
    (0.875, 2.5),
];

/// Triangular counter above the crossbar — a TRUE inner loop, so the A
/// prism is genus 1 before the boolean ever runs.
const A_COUNTER: [(f64, f64); 3] = [(0.90625, 1.4375), (1.09375, 1.4375), (1.0, 2.0)];

fn lp(poly: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::polygon(
        poly.iter()
            .map(|&(x, y)| Point2::new(x, y))
            .collect::<Vec<_>>(),
    )
}

fn validated(plane: SketchPlane<f64>, loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(plane, loops)
        .validate(Tolerance::get())
        .expect("A x Z profile")
}

/// The A prism: xy sketch at z = -1/16, extruded 2.125 along +z
/// (strictly covering Z's z-extent — the C2 decoupling audit from #91:
/// the only possible coincident carriers are y = const planes, and the
/// two bodies' y-plane sets are disjoint by 1/16 straddles).
fn a_prism() -> Body<f64> {
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, -0.0625),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    extrude(
        &validated(plane, vec![lp(&A_OUTLINE), lp(&A_COUNTER)]),
        Extrusion::Distance(2.125),
    )
    .expect("extrude A")
    .body
}

/// The Z prism: (y, z) sketch at x = -1/16 — bars z ∈ [0, 0.4375] and
/// [1.5625, 2], diagonal at slope 3/5 — extruded 2.125 along +x
/// (strictly covering A's x-extent).
fn z_prism() -> Body<f64> {
    let z_poly = [
        (-0.0625, 0.0),
        (2.5625, 0.0),
        (2.5625, 0.4375),
        (0.6875, 0.4375),
        (2.5625, 1.5625),
        (2.5625, 2.0),
        (-0.0625, 2.0),
        (-0.0625, 1.5625),
        (1.8125, 1.5625),
        (-0.0625, 0.4375),
    ];
    let plane = SketchPlane::from_frame(
        Point3::new(-0.0625, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    extrude(
        &validated(plane, vec![lp(&z_poly)]),
        Extrusion::Distance(2.125),
    )
    .expect("extrude Z")
    .body
}

pub fn stops() -> Vec<Stop> {
    let az = expect_seamed(
        "A x Z intersect (counter-hole A)",
        check(try_intersect(&a_prism(), &z_prism()), V_AZ),
        V_AZ,
    );
    vec![Stop {
        name: "az",
        caption: "A x Z (the #93 acceptance case)".to_string(),
        // Standalone render only — the montage sheet stays as shipped.
        montage: false,
        story: "the A x Z silhouette intersect: #91 probed it, it refused typed \
                (the vertex-only anchor gap), #93 banked it as the acceptance \
                fixture, #108 made the whole class build — counter-hole A \
                (a true inner loop) x Z, now a tour stop",
        ops: "extrude A with counter (xy sketch, +z) x extrude Z (yz sketch, +x) \
              -> 1 intersect node",
        delta: 1e-2,
        note: Some(format!(
            "volume gated on the exact oracle 880383/327680 = {V_AZ} \
             (independent exact-fraction integration, pinned in \
             crates/sweep/tests/issue93_az_intersect.rs)"
        )),
        view: View {
            elev: 22.0,
            azim: -65.0,
            up: 'y',
        },
        bodies: vec![{
            let mut sb = SceneBody::seamed("az", [0.42, 0.55, 0.74], az.body, az.contacts);
            sb.mesh_gap = Some(
                "issue #111: the planar CDT's centroid-parity filter keeps an \
                 exterior needle triangle on the 1-ulp-noisy collinear seam \
                 boundary — exact kernel green, tessellation not watertight",
            );
            sb
        }],
    }]
}
