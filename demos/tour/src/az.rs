//! The A×Z silhouette intersect: the letter pair of issue #93, whose
//! acceptance fixture (`crates/sweep/tests/issue93_az_intersect.rs`)
//! offsets every plane by 1/16; here the letters are drawn at the
//! proportions a person would draw them. A's triangular counter is a
//! TRUE inner loop, so the A prism is genus 1 before the boolean runs.
//!
//! Both letters are drawn in one block, x ∈ [0, 2], y ∈ [0, 2.5],
//! z ∈ [0, 2]: the Z's bars end on the A's feet and apex planes, and
//! the A's caps lie on the Z's outer bar faces. Those contacts are
//! DECLARED ([`crate::booleans::try_intersect_declared`]).
//!
//! The Z's diagonal has slope 9/14, so its seam vertices are
//! non-dyadic.
//!
//! Built `A ∩ Z`: `Z ∩ A` refuses `JoinDesync` on the same
//! declarations (`work/join/declared-flush-intersect-refuses-in-one-operand-order.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::profile::{ConstructedLoop, SketchPlane};
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::Body;

use crate::booleans::{check, expect_seamed, try_intersect_declared};
use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::authoring::{p3, polygon, validated};
use pncad::geom_core::{OrthoFrame, Tol};

/// Exact volume, 38627/14336: V = ∫ len_x(A at y) · len_z(Z at y) dy
/// over y ∈ [0, 5/2] (each prism spans the other's extrusion). Both
/// lengths are linear between the breakpoints {0, 3/4, 1, 23/16, 7/4,
/// 2, 5/2}, so each piece's integrand is quadratic and Milne's rule
/// is exact on it; summed in exact fractions.
const V_AZ: f64 = 38627.0 / 14336.0;

/// Slanted-stroke A, (x, y): feet at y = 0, crossbar band
/// y ∈ [1, 1.4375], flat apex at y = 2.5.
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

/// Triangular counter above the crossbar.
const A_COUNTER: [(f64, f64); 3] = [(0.90625, 1.4375), (1.09375, 1.4375), (1.0, 2.0)];

/// Z, (y, z): bars z ∈ [0, 0.4375] and [1.5625, 2] across the A's
/// height, diagonal stroke 3/4 wide.
const Z_OUTLINE: [(f64, f64); 10] = [
    (0.0, 0.0),
    (2.5, 0.0),
    (2.5, 0.4375),
    (0.75, 0.4375),
    (2.5, 1.5625),
    (2.5, 2.0),
    (0.0, 2.0),
    (0.0, 1.5625),
    (1.75, 1.5625),
    (0.0, 0.4375),
];

/// Every corner of these polygons is definitely sharp, so the door's
/// authoring-time classification passes.
fn lp<S: Scalar>(poly: &[(f64, f64)], tol: Tol) -> ConstructedLoop<S> {
    polygon(poly, tol).expect("letterform outline")
}

/// The A prism: xy sketch at z = 0, extruded 2 along +z.
fn a_prism<S: Scalar>(tol: Tol) -> Body<S> {
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(p3(0.0, 0.0, 0.0)));
    extrude(
        &validated(plane, vec![lp(&A_OUTLINE, tol), lp(&A_COUNTER, tol)], tol).expect("A profile"),
        Extrusion::Distance(S::from_f64(2.0)),
        tol,
    )
    .expect("extrude A")
    .body
}

/// The Z prism: yz sketch at x = 0, extruded 2 along +x.
fn z_prism<S: Scalar>(tol: Tol) -> Body<S> {
    let plane = SketchPlane::from_frame(OrthoFrame::axes_yz(p3(0.0, 0.0, 0.0)));
    extrude(
        &validated(plane, vec![lp(&Z_OUTLINE, tol)], tol).expect("Z profile"),
        Extrusion::Distance(S::from_f64(2.0)),
        tol,
    )
    .expect("extrude Z")
    .body
}

/// Builds the A × Z intersect result (generic — the Probe sweep runs
/// the same construction).
pub(crate) fn build<S: Scalar>(tol: Tol) -> pncad::topo::BooleanBody<S> {
    expect_seamed(
        "declared A x Z intersect (counter-hole A)",
        check(
            try_intersect_declared(&a_prism::<S>(tol), &z_prism::<S>(tol), tol),
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
        // Standalone render only: the montage carries the letterforms
        // through silhouette3.
        montage: false,
        story: "the A x Z silhouette intersect: counter-hole A (a true inner loop) x Z, \
                both letters filling one block",
        ops: "extrude A with counter (xy sketch, +z) x extrude Z (yz sketch, +x), flush \
              contacts declared -> 1 intersect node",
        delta: 1e-2,
        note: Some(format!(
            "volume gated on the exact oracle 38627/14336 = {V_AZ} (exact-fraction \
             integration); both letters fill one block, so their contacts are declared"
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
