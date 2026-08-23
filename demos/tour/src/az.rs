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
//! development doing its job again): the exact kernel was green — tiers
//! 1/2/3′ passing, the volume the exact oracle — while the planar CDT's
//! centroid-parity filter kept an exterior needle triangle where the
//! boolean's seam vertices land 1 ulp off an existing boundary carrier
//! line (Z's slope-3/5 diagonal arithmetic is non-dyadic), so
//! `check_mesh` refused `BoundaryEdge` on the tessellation. This scene
//! PINNED that refusal for as long as it stood.
//!
//! #111 is CLOSED: the CDT now classifies regions by a constraint-
//! crossing flood fill over the triangulation's face-adjacency graph
//! instead of by ray-casting a constructed centroid, so every boundary
//! segment is an edge of exactly one kept triangle by construction —
//! no float decision is left to be 1 ulp wrong. The pin has been
//! retired per its own instructions and this stop rides the standard
//! mesh + STL lane like every other; the geometry never changed, so
//! the shipped render still stands.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::geom_core::{Point3, Vec3};
use pncad::profile::{ProfileLoop, SketchPlane};
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::Body;

use crate::booleans::{check, expect_seamed, try_intersect};
use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::authoring::validated;
use pncad::geom_core::Tol;

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

/// Letterform polygons, authored through the PATHS algebra (LIB-U2
/// PR-2): same vertices, same loop — said as a chain of `line_to`s
/// closing at `Start`.
fn lp<S: Scalar>(poly: &[(f64, f64)], tol: Tol) -> ProfileLoop<S> {
    crate::paths::path_polygon(poly, tol)
}

/// The A prism: xy sketch at z = -1/16, extruded 2.125 along +z
/// (strictly covering Z's z-extent — the C2 decoupling audit from #91:
/// the only possible coincident carriers are y = const planes, and the
/// two bodies' y-plane sets are disjoint by 1/16 straddles).
fn a_prism<S: Scalar>(tol: Tol) -> Body<S> {
    let plane = SketchPlane::from_frame(
        Point3::new(S::from_f64(0.0), S::from_f64(0.0), S::from_f64(-0.0625)),
        Vec3::new(S::from_f64(1.0), S::from_f64(0.0), S::from_f64(0.0)),
        Vec3::new(S::from_f64(0.0), S::from_f64(1.0), S::from_f64(0.0)),
    );
    extrude(
        &validated(plane, vec![lp(&A_OUTLINE, tol), lp(&A_COUNTER, tol)], tol)
            .expect("A x Z profile"),
        Extrusion::Distance(S::from_f64(2.125)),
        tol,
    )
    .expect("extrude A")
    .body
}

/// The Z prism: (y, z) sketch at x = -1/16 — bars z ∈ [0, 0.4375] and
/// [1.5625, 2], diagonal at slope 3/5 — extruded 2.125 along +x
/// (strictly covering A's x-extent).
fn z_prism<S: Scalar>(tol: Tol) -> Body<S> {
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
        Point3::new(S::from_f64(-0.0625), S::from_f64(0.0), S::from_f64(0.0)),
        Vec3::new(S::from_f64(0.0), S::from_f64(1.0), S::from_f64(0.0)),
        Vec3::new(S::from_f64(0.0), S::from_f64(0.0), S::from_f64(1.0)),
    );
    extrude(
        &validated(plane, vec![lp(&z_poly, tol)], tol).expect("A x Z profile"),
        Extrusion::Distance(S::from_f64(2.125)),
        tol,
    )
    .expect("extrude Z")
    .body
}

/// Builds the A × Z intersect result (generic — the Probe sweep runs
/// the same construction).
pub(crate) fn build<S: Scalar>(tol: Tol) -> pncad::topo::BooleanBody<S> {
    expect_seamed(
        "A x Z intersect (counter-hole A)",
        check(
            try_intersect(&a_prism::<S>(tol), &z_prism::<S>(tol), tol),
            V_AZ,
            tol,
        ),
        V_AZ,
    )
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let az = build::<f64>(tol);
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
        bodies: vec![SceneBody::seamed(
            "az",
            [0.42, 0.55, 0.74],
            az.body,
            az.contacts,
        )],
    }]
}
