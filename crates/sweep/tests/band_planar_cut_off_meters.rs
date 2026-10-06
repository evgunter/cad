//! **What a straight band's cut-off and local carve take from the faces
//! they leave**, each metered before any mutation and each pinned here
//! by a fixture the meter alone refuses:
//!
//! - the end face's SLIVER, on either side (arm (c) of the surgery's
//!   ring carry-through pass): a hole drilled into it refuses whether
//!   the band's cut takes the sliver away (convex) or its fill covers
//!   it (concave);
//! - a support's STRIP (arm (d)): a spike of the outline whose tip
//!   reaches into the strip between the sampled screen's points;
//! - a RIM two cut-offs split from its two ends: their feet must stand
//!   in order on it, definitely apart.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Vec3};
use sweep::blend::{BlendDecision, BlendError, BlendSite};
use sweep::test_support::{block, pocket_die, prism, prism_on, realized, sketch_from_axes};
use topo::{Body, EntityId, validate_geometric};

use crate::band_planar_cut_off::{D, Verb, carve, edge, tol};

/// A round pin of radius `r` along +x from `origin`, `h` long: the
/// operand whose subtraction drills a hole.
fn pin(origin: Point3<f64>, r: f64, h: f64) -> Body<f64> {
    let plane = sketch_from_axes(
        origin,
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    prism_on(
        plane,
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        h,
        tol(),
    )
}

/// **A hole inside the end face's sliver refuses on either side.** The
/// convex control drills a blind hole into a box's end face beside the
/// cut-off's old vertex; the concave row drills the pocketed die's end
/// wall beside a floor edge's old vertex, where the band's fill would
/// cover the hole's mouth. Both inputs are tier-3 valid, and each hole
/// lies inside the sliver triangle of side `d`, so the end face would
/// lose part of the hole's ring: both refuse `RingClearance` at that
/// end face.
#[test]
fn a_hole_in_the_end_face_sliver_refuses_on_either_side() {
    let convex = realized(
        topo::boolean::BooleanOp::Subtract,
        &block(2.0, 1.5, 1.0, tol()),
        &pin(Point3::new(-0.1, 0.03, 0.97), 0.01, 0.4),
        tol(),
    );
    let concave = realized(
        topo::boolean::BooleanOp::Subtract,
        &pocket_die(0.0, 0.0, 0.0, tol()),
        &pin(Point3::new(-0.1, 0.28, 0.53), 0.01, 0.45),
        tol(),
    );
    for (what, body, a, b) in [
        ("convex", &convex, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
        ("concave", &concave, [0.25, 0.25, 0.5], [0.75, 0.25, 0.5]),
    ] {
        validate_geometric(body, tol())
            .unwrap_or_else(|e| panic!("{what}: the drilled input is tier-3 valid, got {e:?}"));
        let e = edge(body, a, b);
        for verb in [Verb::Chamfer, Verb::Fillet] {
            match verb.run(body, &[e]) {
                Err(BlendError::RingClearance { .. }) => {}
                Err(other) => panic!("{what} ({verb:?}): refuses at the sliver, got {other}"),
                Ok(out) => panic!(
                    "{what} ({verb:?}): a hole in the sliver must refuse, built one whose tier 3 \
                     reads {:?}",
                    validate_geometric(&out.body, tol())
                ),
            }
        }
    }
}

/// **A spike of the support's outline reaching into the strip refuses
/// as predicate 2 does.** The top face's outline dips in a narrow V to
/// `y = 0.05` of the requested front edge, with `d = 0.1`; the sampled
/// screen reads the spike's two sides between their samples and passes
/// them, so the strip meter is what refuses — and without it the carve
/// builds a tier-3-valid body whose strip runs through the spike.
#[test]
fn a_spike_into_the_strip_refuses_at_the_strip_meter() {
    let body = prism(
        [
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 1.0),
            (2.95, 1.0),
            (2.9, 0.05),
            (2.85, 1.0),
            (0.0, 1.0),
        ]
        .iter()
        .map(|&(x, y)| (Point2::new(x, y), 0.0))
        .collect(),
        1.0,
        tol(),
    );
    let e = edge(&body, [0.0, 0.0, 1.0], [3.0, 0.0, 1.0]);
    let top = body
        .get_half_edge(body.get_edge(e).expect("the edge").he_plus)
        .map(|h| body.get_loop(h.parent_loop).expect("its loop").face);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        match verb.run(&body, &[e]) {
            Err(BlendError::FaceClearanceUncertified {
                face, cross_chain, ..
            }) => {
                assert!(!cross_chain, "{verb:?}: one band's strip");
                let other = body
                    .get_half_edge(body.get_edge(e).expect("e").he_minus)
                    .map(|h| body.get_loop(h.parent_loop).expect("loop").face);
                assert!(
                    Some(face) == top || Some(face) == other,
                    "{verb:?}: the refusal names a support of the band"
                );
            }
            other => panic!("{verb:?}: the spike refuses at the strip, got {other:?}"),
        }
    }
}

/// **Two cut-offs splitting one rim from its two ends.** On a
/// `2 × 1.5 × h` block the top front edge (end face `x = 2`) and the
/// bottom right edge (end face `y = 0`) both cut the vertical edge
/// `x = 2, y = 0`: one foot at `z = h − d`, the other at `z = d`. Apart
/// by `h − 2d` — `h = 0.25` — the two build at the prism closed form;
/// crossing or coinciding they refuse typed before any mutation, naming
/// the shared rim, and apart only within the band they escalate there.
/// (Unmetered, the carve's second split refused as an Euler operator's
/// error or a chain shape, depending on where the feet fell.)
#[test]
fn two_cut_offs_on_one_rim_build_apart_and_refuse_crossing() {
    let eps = tol().eps();
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let h = 0.25;
        let body = block(2.0, 1.5, h, tol());
        let pair = [
            edge(&body, [0.0, 0.0, h], [2.0, 0.0, h]),
            edge(&body, [2.0, 0.0, 0.0], [2.0, 1.5, 0.0]),
        ];
        carve(
            &body,
            &pair,
            verb,
            verb.section() * 3.5,
            "feet apart on a shared rim",
        );
        // Definitely crossing, coincident within ε, and apart or crossing
        // by a few ε, inside the band.
        for (dh, in_band) in [
            (-0.05, false),
            (-eps / 2.0, false),
            (0.0, false),
            (eps / 2.0, false),
            (3.0 * eps, true),
            (-3.0 * eps, true),
        ] {
            let h = 2.0 * D + dh;
            let body = block(2.0, 1.5, h, tol());
            let pair = [
                edge(&body, [0.0, 0.0, h], [2.0, 0.0, h]),
                edge(&body, [2.0, 0.0, 0.0], [2.0, 1.5, 0.0]),
            ];
            let rim = edge(&body, [2.0, 0.0, 0.0], [2.0, 0.0, h]);
            match verb.run(&body, &pair) {
                Err(BlendError::Escalated {
                    site: BlendSite::Link { edge },
                    decision: BlendDecision::CutOffFeet,
                    ..
                }) if in_band => {
                    assert_eq!(edge, rim, "{verb:?} h − 2d = {dh:e}: the rim");
                }
                Err(BlendError::UnsupportedRunOut { at, detail }) if !in_band => {
                    assert_eq!(at, EntityId::Edge(rim), "{verb:?} h − 2d = {dh:e}: the rim");
                    assert!(
                        detail.contains("feet cross or coincide on the rim they share"),
                        "{verb:?} h − 2d = {dh:e}: {detail}"
                    );
                }
                other => panic!(
                    "{verb:?} h − 2d = {dh:e}: feet not definitely apart stop at plan (in band: \
                     {in_band}), got {other:?}"
                ),
            }
        }
    }
}
