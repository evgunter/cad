//! **Pins of today's refusals, which BANDRULE's rows will flip.**
//!
//! Each row below pins a refusal the blend gives TODAY on an ordinary
//! part once its smooth edges are left as they are (REFS). They are not
//! what the blend should do: each names the BANDRULE row whose work
//! flips it, and that row's PR turns its pin into a build.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::blend::{BlendError, CornerConfig, fillet_edges};
use sweep::test_support::{at_rest, corners, cylinder_of_arcs_at, extruded, prism_at, realized};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey};

use crate::a_blend_leaves_a_smooth_edge_as_it_is::{edge_between, slot};

fn tol() -> Tol {
    Tol::witness()
}

fn every_edge(body: &Body<f64>) -> Vec<EdgeKey> {
    let mut all = topo::query::all_edges(body);
    all.sort_unstable();
    all
}

/// **The bracket**: a 6 × 4 × 0.5 plate, an upright 0.5 × 3 boss flush
/// with its `x = 0` side standing 2 tall, and a two-arc bore of radius
/// 0.3 through the plate at (3, 2). The boss meets the plate at
/// vertices where two of three edges are convex (on the flush side) or
/// one of three (inside).
fn bracket() -> Body<f64> {
    let plate = prism_at(
        corners(&[(0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)]),
        0.0,
        0.5,
        tol(),
    );
    let boss = prism_at(
        corners(&[(0.0, 0.5), (0.5, 0.5), (0.5, 3.5), (0.0, 3.5)]),
        0.25,
        1.75,
        tol(),
    );
    let bore = cylinder_of_arcs_at(2, 0.3, Point2::new(3.0, 2.0), -0.5, 1.5, tol());
    realized(
        BooleanOp::Subtract,
        &realized(BooleanOp::Union, &plate, &boss, tol()),
        &bore,
        tol(),
    )
}

/// **PIN (flipped by `work/band/a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds.md`)**:
/// the bracket with every edge requested, as "select all edges" asks,
/// refuses at a mixed-convexity corner where the boss meets the plate.
#[test]
fn pin_today_the_bracket_filleted_all_round_refuses_a_mixed_convexity_corner() {
    let body = bracket();
    let got =
        fillet_edges(&at_rest(&body, tol()), &every_edge(&body), 0.05, tol()).map_err(|r| r.error);
    assert!(
        matches!(
            got,
            Err(BlendError::UnsupportedCorner {
                corner: CornerConfig::MixedConvexity { .. },
                ..
            })
        ),
        "today the bracket refuses at a mixed corner, got {got:?}"
    );
}

/// **The rounded rectangle**: [−1, 1] × [−0.6, 0.6], corner radius 0.3,
/// extruded 0.5 up. Each cap's outline is one closed tangent-continuous
/// chain of four lines and four quarter arcs.
fn rounded_rectangle() -> Body<f64> {
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    extruded(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (Point2::new(-0.7, -0.6), 0.0),
            (Point2::new(0.7, -0.6), q),
            (Point2::new(1.0, -0.3), 0.0),
            (Point2::new(1.0, 0.3), q),
            (Point2::new(0.7, 0.6), 0.0),
            (Point2::new(-0.7, 0.6), q),
            (Point2::new(-1.0, 0.3), 0.0),
            (Point2::new(-1.0, -0.3), q),
        ])],
        0.5,
        tol(),
    )
}

/// **PIN (flipped by `work/band/a-rounded-outline-is-a-closed-chain-of-mixed-links-the-blend-refuses.md`)**:
/// the rounded rectangle with every edge requested leaves its eight
/// smooth struts and refuses each cap's outline, a closed chain of
/// mixed links, as an unbuilt chain shape.
#[test]
fn pin_today_a_rounded_rectangle_filleted_all_round_refuses_its_closed_mixed_chain() {
    let body = rounded_rectangle();
    let got =
        fillet_edges(&at_rest(&body, tol()), &every_edge(&body), 0.05, tol()).map_err(|r| r.error);
    assert!(
        matches!(got, Err(BlendError::UnsupportedChain { .. })),
        "today a rounded outline refuses as a closed chain shape, got {got:?}"
    );
}

/// **PIN (flipped by `work/band/a-chain-end-beside-a-smooth-seam-folds-to-an-indeterminate-corner.md`)**:
/// one of the slot's semicircular top arcs, requested alone, ends at
/// vertices where a smooth strut arrives; the corner read folds that
/// strut into an unreadable configuration.
#[test]
fn pin_today_a_slot_end_arc_alone_folds_its_smooth_neighbour_to_indeterminate() {
    let body = slot();
    let arc = topo::query::all_edges(&body)
        .into_iter()
        .find(|&e| {
            let he = body
                .get_half_edge(body.get_edge(e).unwrap().he_plus)
                .unwrap();
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let (a, b) = (
                p(he.start),
                p(body
                    .half_edge_end(body.get_edge(e).unwrap().he_plus)
                    .unwrap()),
            );
            a.z > 0.5 && b.z > 0.5 && a.x > 0.9 && b.x > 0.9 && (a.y - b.y).abs() > 0.9
        })
        .expect("the slot's right top arc");
    let line = edge_between(&body, [-1.0, -0.5, 1.0], [1.0, -0.5, 1.0]);
    assert_ne!(arc, line);
    let got = fillet_edges(&at_rest(&body, tol()), &[arc], 0.05, tol()).map_err(|r| r.error);
    assert!(
        matches!(
            got,
            Err(BlendError::UnsupportedCorner {
                corner: CornerConfig::Indeterminate,
                ..
            })
        ),
        "today a slot's end arc alone folds its strut into an indeterminate corner, got {got:?}"
    );
}
