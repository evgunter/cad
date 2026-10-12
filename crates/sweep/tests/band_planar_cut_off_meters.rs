//! **What a straight band's cut-off and local carve take from the faces
//! they leave**, each metered before any mutation and each pinned here
//! by a fixture the meter alone refuses:
//!
//! - the end face's SLIVER, on either side (arm (c) of the surgery's
//!   ring carry-through pass): a hole drilled into it refuses whether
//!   the band's cut takes the sliver away (convex) or its fill covers
//!   it (concave);
//! - a support's STRIP (arm (d)): a spike of the outline whose tip
//!   reaches into the strip between the sampled screen's points, on
//!   the outer cycle or on a ring that carries the requested edge (which
//!   is metered against the strip, so a clear one builds); a feature
//!   across the edge's line, which the strip never reaches, is clear;
//!   and on a ruled link's planar support, which arm (d) does not
//!   meter, a ring's feature refuses at the trimline and an outer
//!   cycle's vertex at the reach;
//! - a RIM two cut-offs split from its two ends: their feet must stand
//!   in order on it, definitely apart.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{ErrorTextReading, Point2, Point3, Vec3};
use sweep::blend::{BlendDecision, BlendError, BlendSite};
use sweep::test_support::{block, pocket_die, prism, prism_on, realized, sketch_from_axes};
use topo::{Body, ContactRecords, EntityId, validate_geometric, validate_pseudomanifold};

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

/// The pocket a tab reaches into: a `6 × 5 × 2` block less the prism
/// over `outline` from `z = 1` to `z = 2`. The tab's tip is the mouth
/// edge from `(3.5, 2.5)` to `(2.5, 2.5)` on the top face.
fn tabbed_pocket(outline: &[(f64, f64)]) -> Body<f64> {
    let outline = outline
        .iter()
        .map(|&(x, y)| (Point2::new(x, y), 0.0))
        .collect();
    let body = realized(
        topo::boolean::BooleanOp::Subtract,
        &block(6.0, 5.0, 2.0, tol()),
        &sweep::test_support::prism_at(outline, 1.0, 2.0, tol()),
        tol(),
    );
    validate_geometric(&body, tol()).expect("the tabbed pocket is tier-3 valid");
    body
}

/// The tab's tip, from its right side wall to its left.
const TAB: [(f64, f64); 2] = [(3.5, 2.5), (2.5, 2.5)];

/// **A requested edge of a support's ring builds at its strip.** A
/// pocket whose outline has a tab reaching into it: the tab's tip is a
/// mouth edge between two corners whose three edges are all convex, so
/// the edge alone is cut off at both ends, and the strip it takes lies
/// inside the tab. The ring it lies in is metered against that strip,
/// not against its unbounded trimline (on which its own edge sits), so
/// the band builds at the prism closed form, both verbs.
#[test]
fn a_requested_ring_edge_builds_at_its_strip() {
    let body = tabbed_pocket(&[
        (1.0, 1.0),
        (5.0, 1.0),
        (5.0, 4.0),
        (3.5, 4.0),
        TAB[0],
        TAB[1],
        (2.5, 4.0),
        (1.0, 4.0),
    ]);
    let e = edge(&body, [3.5, 2.5, 2.0], [2.5, 2.5, 2.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        carve(&body, &[e], verb, verb.section(), "the tab's tip");
    }
}

/// The tabbed pocket with a narrow spike of the pocket cut into the
/// tab's side wall, its tip `depth` from the tip edge at `x = 3.0625`:
/// midway between two of the screen's samples on that edge, which then
/// reads a gap of at least `√(0.0625² + depth²)`, more than `D` for a
/// depth above `0.078`. The spike's own samples stand a length's eighth
/// up its sides, far from the strip.
fn spiked_pocket(depth: f64) -> Body<f64> {
    tabbed_pocket(&[
        (1.0, 1.0),
        (5.0, 1.0),
        (5.0, 4.0),
        (3.5, 4.0),
        (3.5, 3.95),
        (3.0625, 2.5 + depth),
        (3.5, 3.9),
        TAB[0],
        TAB[1],
        (2.5, 4.0),
        (1.0, 4.0),
    ])
}

/// **A ring edge reaching into a requested ring edge's strip refuses
/// there, as predicate 2 does.** The spike's tip stands `0.09` from the
/// tab's tip, inside the strip of depth `D` and past the sampled
/// screen, so the strip meter reading the ring is what refuses, at the
/// top face; `3ε` either side of `D` it escalates there; a spike
/// clear of the strip builds at the prism closed form.
#[test]
fn a_ring_edge_in_a_requested_ring_edges_strip_refuses_at_the_strip_meter() {
    let eps = tol().eps();
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let body = spiked_pocket(0.09);
        let e = edge(&body, [3.5, 2.5, 2.0], [2.5, 2.5, 2.0]);
        let top = body
            .get_half_edge(body.get_edge(e).expect("the edge").he_plus)
            .map(|h| body.get_loop(h.parent_loop).expect("its loop").face);
        let other = body
            .get_half_edge(body.get_edge(e).expect("the edge").he_minus)
            .map(|h| body.get_loop(h.parent_loop).expect("its loop").face);
        match verb.run(&body, &[e]) {
            Err(BlendError::FaceClearanceUncertified {
                face,
                cross_chain,
                gap,
                ..
            }) => {
                assert!(!cross_chain, "{verb:?}: one band's strip");
                assert!(
                    Some(face) == top || Some(face) == other,
                    "{verb:?}: the refusal names a support of the band"
                );
                assert!(
                    matches!(gap.diagnostic_f64_for_error_text(), ErrorTextReading::Value(g) if (g - 0.09).abs() < 1e-9),
                    "{verb:?}: the strip meter reports the tip's own gap, 0.09, where the \
                     screen reads at least 0.1096 off its samples: {gap:?}"
                );
            }
            other => panic!("{verb:?}: the spike refuses at the strip, got {other:?}"),
        }
        for dh in [3.0 * eps, -3.0 * eps] {
            let body = spiked_pocket(D + dh);
            let e = edge(&body, [3.5, 2.5, 2.0], [2.5, 2.5, 2.0]);
            match verb.run(&body, &[e]) {
                Err(BlendError::Escalated {
                    site: BlendSite::Chain,
                    decision: BlendDecision::FaceClearance,
                    ..
                }) => {}
                other => panic!(
                    "{verb:?} depth − D = {dh:e}: the spike's tip in band escalates, got {other:?}"
                ),
            }
        }
        let body = spiked_pocket(D + 0.05);
        let e = edge(&body, [3.5, 2.5, 2.0], [2.5, 2.5, 2.0]);
        carve(
            &body,
            &[e],
            verb,
            verb.section(),
            "a spike clear of the strip",
        );
    }
}

/// The tabbed pocket's concave mirror: a `6 × 5 × 1` block united with
/// the prism over `outline` from `z = 0.5` to `z = 2`, so the block's
/// top face carries the outline, and the tab's tip is the concave base
/// edge from `(3.5, 2.5)` to `(2.5, 2.5)` at `z = 1`.
fn tabbed_boss(outline: &[(f64, f64)]) -> Body<f64> {
    let outline = outline
        .iter()
        .map(|&(x, y)| (Point2::new(x, y), 0.0))
        .collect();
    let body = realized(
        topo::boolean::BooleanOp::Union,
        &block(6.0, 5.0, 1.0, tol()),
        &sweep::test_support::prism_at(outline, 0.5, 1.5, tol()),
        tol(),
    );
    validate_geometric(&body, tol()).expect("the tabbed boss is tier-3 valid");
    body
}

/// The tab's outline with a peninsula of the support rising from the
/// pocket's far wall, its tip `gap` across the tip edge's line, at
/// `x = 3.0625`, midway between two of the screen's samples on the
/// tip. `open` runs the outline out through the block's left side, so
/// the tip lies on the support's outer cycle rather than a ring.
fn far_side(gap: f64, open: bool) -> Vec<(f64, f64)> {
    let x0 = if open { -1.0 } else { 1.0 };
    vec![
        (x0, 1.0),
        (3.025, 1.0),
        (3.0625, 2.5 - gap),
        (3.1, 1.0),
        (5.0, 1.0),
        (5.0, 4.0),
        (3.5, 4.0),
        TAB[0],
        TAB[1],
        (2.5, 4.0),
        (x0, 4.0),
    ]
}

/// **A feature across a requested edge's line is clear of its strip.**
/// A peninsula of the support rises from the pocket's far wall to `0.09`
/// short of the tab's tip, on the side of the tip's line the band never
/// reaches: on the ring and on the outer cycle, pocket and boss, both
/// verbs, the band builds at the prism closed form, tier 3 and 3′. At
/// `0.05` the sampled screen reads `√(0.0625² + 0.05²)` between its two
/// samples, less than `D` on either side of the edge, and refuses.
#[test]
fn a_feature_across_a_requested_edges_line_is_clear_of_its_strip() {
    for open in [false, true] {
        for (convex, z) in [(true, 2.0), (false, 1.0)] {
            let at = |gap: f64| {
                let outline = far_side(gap, open);
                let body = if convex {
                    tabbed_pocket(&outline)
                } else {
                    tabbed_boss(&outline)
                };
                let e = edge(&body, [3.5, 2.5, z], [2.5, 2.5, z]);
                (body, e)
            };
            let what = format!("open {open}, convex {convex}");
            for verb in [Verb::Chamfer, Verb::Fillet] {
                let (body, e) = at(0.09);
                let removed = if convex {
                    verb.section()
                } else {
                    -verb.section()
                };
                let out = carve(&body, &[e], verb, removed, &what);
                validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
                    .unwrap_or_else(|err| panic!("{what} ({verb:?}): tier 3′, got {err:?}"));
                let (body, e) = at(0.05);
                match verb.run(&body, &[e]) {
                    Err(BlendError::FaceClearanceUncertified { gap, .. }) => assert!(
                        matches!(gap.diagnostic_f64_for_error_text(), ErrorTextReading::Value(g) if (g - 0.0625_f64.hypot(0.05)).abs() < 1e-9),
                        "{what} ({verb:?}): the screen refuses off its two samples: {gap:?}"
                    ),
                    other => panic!(
                        "{what} ({verb:?}): 0.05 across refuses at the screen, got {other:?}"
                    ),
                }
            }
        }
    }
}

/// The tabbed pocket beside a through half-cylinder groove along `x`,
/// its axis at `y = 4.6` on the top face, so the face carrying the
/// tab's ring also carries the groove's mouth line `y = 4.4`, a
/// cylinder–plane (ruled) link's edge. `wall` is spliced into the
/// pocket's `y = 4` wall, each vertex with the bulge of the edge it
/// starts: a feature reaching toward the mouth about `x = 1.875`, midway
/// between two of the mouth's screen samples. `open` runs the pocket out
/// through the block's left side, so the feature lies on the top face's
/// outer cycle rather than its ring.
fn grooved(wall: &[((f64, f64), f64)], open: bool) -> Body<f64> {
    let x0 = if open { -1.0 } else { 1.0 };
    let outline: Vec<(Point2<f64>, f64)> = [
        (x0, 1.0),
        (5.0, 1.0),
        (5.0, 4.0),
        (3.5, 4.0),
        TAB[0],
        TAB[1],
        (2.5, 4.0),
    ]
    .into_iter()
    .map(|p| (p, 0.0))
    .chain(wall.iter().copied())
    .chain([((x0, 4.0), 0.0)])
    .map(|((x, y), bulge)| (Point2::new(x, y), bulge))
    .collect();
    let pocket = realized(
        topo::boolean::BooleanOp::Subtract,
        &block(6.0, 5.0, 2.0, tol()),
        &sweep::test_support::prism_at(outline, 1.0, 2.0, tol()),
        tol(),
    );
    let rod = prism_on(
        sketch_from_axes(
            Point3::new(-0.1, 4.6, 2.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        ),
        vec![(Point2::new(-0.2, 0.0), 1.0), (Point2::new(0.2, 0.0), 1.0)],
        6.2,
        tol(),
    );
    let body = realized(topo::boolean::BooleanOp::Subtract, &pocket, &rod, tol());
    validate_geometric(&body, tol()).expect("the grooved pocket is tier-3 valid");
    body
}

/// **A ruled link's trimline still meters a ring carrying a planar
/// requested edge, and a ruled link's planar support refuses a vertex in
/// its strip typed.** The groove's mouth filleted (a chamfer of it is
/// no plane–plane arm) takes a strip of setback `√0.08 − 0.2` off the
/// top face. A narrow spike, or an arc bulging from two vertices on the
/// wall, reaches `0.05` short of the mouth: inside the strip, between
/// the screen's samples. On the ring that also carries the tab's
/// requested tip, the mouth's trimline refuses either exactly
/// (`RingClearance`), since no strip meter covers a ruled link; on the
/// outer cycle, the reach refuses the spike's vertex in the strip
/// (`FaceClearanceUncertified`, gap `0.05`). Without them the mouth and
/// tip build together, and a spike `0.1` short of the mouth, clear of
/// the strip, builds.
#[test]
fn a_ruled_links_strip_on_a_planar_support_refuses_a_feature_inside_it() {
    let setback = 0.08_f64.sqrt() - 0.2;
    let near = |r: geom_core::MarginDiag, want: f64| matches!(r.diagnostic_f64_for_error_text(), ErrorTextReading::Value(v) if (v - want).abs() < 1e-9);
    let both = |body: &Body<f64>| {
        let mouth = edge(body, [0.0, 4.4, 2.0], [6.0, 4.4, 2.0]);
        let tip = edge(body, [3.5, 2.5, 2.0], [2.5, 2.5, 2.0]);
        Verb::Fillet.run(body, &[mouth, tip])
    };
    let built = |what: &str, out: Result<sweep::blend::build::Blended<f64>, BlendError>| {
        let out = out.unwrap_or_else(|e| panic!("{what}: builds, got {e}"));
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
        validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
            .unwrap_or_else(|e| panic!("{what}: tier 3′, got {e:?}"));
    };
    let spike = |tip: f64| [((1.9, 4.0), 0.0), ((1.875, tip), 0.0), ((1.85, 4.0), 0.0)];
    // Sagitta 0.35 over a half-chord of 0.125: the arc's apex at 4.35.
    let arc = [((2.0, 4.0), 2.8), ((1.75, 4.0), 0.0)];
    built("the mouth and the tip", both(&grooved(&[], false)));
    for (what, wall) in [("spike", &spike(4.35)[..]), ("arc", &arc[..])] {
        match both(&grooved(wall, false)) {
            Err(BlendError::RingClearance { margin, .. }) => assert!(
                near(margin.reading, 0.05 - setback),
                "{what}: the mouth's trimline reads its reach toward the mouth: {margin:?}"
            ),
            other => {
                panic!("{what}: in the mouth's strip it refuses at the trimline, got {other:?}")
            }
        }
    }
    let mouth = |body: &Body<f64>| {
        let e = edge(body, [0.0, 4.4, 2.0], [6.0, 4.4, 2.0]);
        Verb::Fillet.run(body, &[e])
    };
    match mouth(&grooved(&spike(4.35), true)) {
        Err(BlendError::FaceClearanceUncertified {
            margin,
            gap,
            cross_chain,
            ..
        }) => {
            assert!(!cross_chain, "one band's strip");
            assert!(
                near(gap, 0.05),
                "the reach reports the tip's gap to the mouth: {gap:?}"
            );
            assert!(
                near(margin.reading, 0.05 - setback),
                "its depth inside the strip: {margin:?}"
            );
        }
        other => panic!("an outer spike in the mouth's strip refuses at the reach, got {other:?}"),
    }
    built(
        "a spike clear of the mouth's strip",
        mouth(&grooved(&spike(4.3), true)),
    );
}
