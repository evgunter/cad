//! **What a straight band's cut-off and local carve take from the faces
//! they leave**, each metered before any mutation and each pinned here
//! by a fixture the meter alone refuses:
//!
//! - the end face's SLIVER, on either side (arm (c) of the surgery's
//!   ring carry-through pass): a hole drilled into it whose ring the
//!   band crosses refuses whether the band's cut takes the sliver away
//!   (convex) or its fill covers it (concave);
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

use geom_core::RigidMotion;
use geom_core::{ErrorTextReading, Point2, Point3};
use profile::SketchPlane;
use sweep::blend::{BlendDecision, BlendError, BlendSite, Convexity};
use sweep::test_support::{block, pocket_die, prism, prism_on, realized};
use topo::{Body, ContactRecords, EntityId, validate_geometric, validate_pseudomanifold};

use crate::band_one_cycle_members::{assert_taken, cup_faces};
use crate::band_planar_cut_off::{D, Verb, carve, edge, tol};

/// A round pin of radius `r` along +x from `origin`, `h` long: the
/// operand whose subtraction drills a hole.
fn pin(origin: Point3<f64>, r: f64, h: f64) -> Body<f64> {
    let plane = SketchPlane::from_frame(RigidMotion::axes_yz(origin));
    prism_on(
        plane,
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        h,
        tol(),
    )
}

/// **A hole inside the end face's sliver: the fillet's arc crosses it
/// on either side, and the chamfer takes the convex one and refuses the
/// concave one.** The convex row drills a blind hole `0.3` deep into a
/// box's end face beside the cut-off's old vertex; the concave row drills
/// the pocketed die's end wall beside a floor edge's old vertex, where
/// the band's fill would cover the hole's mouth. Both inputs are tier-3
/// valid, and each hole lies inside the sliver triangle of side `d`. The
/// fillet's arc crosses each hole's ring, which refuses `RingClearance`
/// at the end face as a measurement. The chamfer's chord does not: each
/// ring lies wholly inside the triangle it cuts off. The convex hole is a
/// cup inside the material the band removes, which goes with it at
/// `ΔV = −(2·d²/2 − π·0.01²·0.3)`; the concave one runs into the die
/// behind the wall the fill covers, which `S ∪ F` seals and the blend
/// does not build, so the reach reads its wall, which meets the end
/// face's plane, as a bound: `FaceClearance { bounded: true }`.
#[test]
fn a_hole_in_the_end_face_sliver_crosses_the_arc_and_the_chamfer_takes_the_convex_one() {
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
    for (what, body, a, b, convexity) in [
        (
            "convex",
            &convex,
            [0.0, 0.0, 1.0],
            [2.0, 0.0, 1.0],
            Convexity::Convex,
        ),
        (
            "concave",
            &concave,
            [0.25, 0.25, 0.5],
            [0.75, 0.25, 0.5],
            Convexity::Concave,
        ),
    ] {
        validate_geometric(body, tol())
            .unwrap_or_else(|e| panic!("{what}: the drilled input is tier-3 valid, got {e:?}"));
        let e = edge(body, a, b);
        for verb in [Verb::Chamfer, Verb::Fillet] {
            match (verb, verb.run(body, &[e])) {
                (Verb::Chamfer, Ok(_)) if convexity == Convexity::Convex => {
                    let cups = cup_faces(body, (0.03, 0.97), 0.01, (0.0, 0.3));
                    assert_eq!(cups.len(), 3, "{what}: the hole's two half-walls and floor");
                    let dv = -(D * D - core::f64::consts::PI * 0.01 * 0.01 * 0.3);
                    assert_taken(verb, body, &[e], D, (&cups, 1, 0), (dv, 1.0), what);
                }
                (
                    Verb::Chamfer,
                    Err(BlendError::FaceClearance {
                        bounded: true,
                        chain,
                        ..
                    }),
                ) if chain == convexity => {}
                (Verb::Chamfer, Err(other)) => {
                    panic!("{what} (Chamfer): a bound at the hole's wall, got {other}")
                }
                (Verb::Fillet, Err(BlendError::RingClearance { bounded: false, .. })) => {}
                (_, Err(other)) => {
                    panic!("{what} ({verb:?}): refuses at the sliver, got {other}")
                }
                (_, Ok(out)) => panic!(
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
    arced_pocket(&outline.iter().map(|&p| (p, 0.0)).collect::<Vec<_>>())
}

/// [`tabbed_pocket`] over an outline whose vertices each carry the
/// bulge of the edge leaving them.
fn arced_pocket(outline: &[((f64, f64), f64)]) -> Body<f64> {
    let outline = outline
        .iter()
        .map(|&((x, y), bulge)| (Point2::new(x, y), bulge))
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
/// tab's side wall, its tip a semicircle of radius `0.03` whose apex
/// stands `depth` from the tip edge at `x = 3.0625`: midway between two
/// of the screen's samples on that edge, which then reads a gap of
/// `√(0.0625² + (depth + 0.03)²) − 0.03`, more than `D` for a depth
/// above `0.084`. The semicircle's two vertices stand `depth + 0.03`
/// from the tip's line, outside the reach meter's tube of radius `D`
/// for a depth above `0.07`, so only a meter reading the ring's EDGES
/// sees the tip.
fn spiked_pocket(depth: f64) -> Body<f64> {
    let y = 2.5 + depth + 0.03;
    arced_pocket(&[
        ((1.0, 1.0), 0.0),
        ((5.0, 1.0), 0.0),
        ((5.0, 4.0), 0.0),
        ((3.5, 4.0), 0.0),
        ((3.5, 3.95), 0.0),
        ((3.0325, y), 1.0),
        ((3.0925, y), 0.0),
        ((3.5, 3.9), 0.0),
        (TAB[0], 0.0),
        (TAB[1], 0.0),
        ((2.5, 4.0), 0.0),
        ((1.0, 4.0), 0.0),
    ])
}

/// **A ring edge reaching into a requested ring edge's strip refuses
/// there, at arm (d), as predicate 2 does.** The spike's arc tip
/// dips to `0.09` from the tab's tip, inside the strip of depth `D`,
/// with both its vertices outside the reach meter's tube and past the
/// sampled screen, so arm (d), the one meter reading the ring's edges,
/// is what refuses, at the top face, at the arc's own gap; `3ε` either
/// side of `D` it escalates there; a spike clear of the strip builds
/// at the prism closed form. With arm (d)'s ring walk off, the cell
/// meter refuses the arc instead (`FaceClearance`, off its cells), and
/// the row goes red at its first request.
#[test]
fn a_ring_arc_with_no_vertex_in_a_requested_edges_strip_refuses_at_the_strip_meter() {
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
                    "{verb:?}: arm (d) reports the arc's own gap, 0.09, where the screen \
                     reads 0.1053 off its samples and the reach meter sees no vertex: {gap:?}"
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
    arced_boss(&outline.iter().map(|&p| (p, 0.0)).collect::<Vec<_>>())
}

/// [`tabbed_boss`] over an outline whose vertices each carry the bulge
/// of the edge leaving them.
fn arced_boss(outline: &[((f64, f64), f64)]) -> Body<f64> {
    let outline = outline
        .iter()
        .map(|&((x, y), bulge)| (Point2::new(x, y), bulge))
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

/// The tab's outline with `far` spliced into the pocket's far wall
/// (each vertex with the bulge of the edge it starts), and, when
/// `tab2`, a second tab `[1.3, 1.8] × [3, 4]` whose tip, from
/// `(1.8, 3)` to `(1.3, 3)`, is a chain apart from the first tab's.
fn far_walled(far: &[((f64, f64), f64)], tab2: bool) -> Vec<((f64, f64), f64)> {
    let mut o = vec![((1.0, 1.0), 0.0)];
    o.extend_from_slice(far);
    o.extend([
        ((5.0, 1.0), 0.0),
        ((5.0, 4.0), 0.0),
        ((3.5, 4.0), 0.0),
        (TAB[0], 0.0),
        (TAB[1], 0.0),
        ((2.5, 4.0), 0.0),
    ]);
    if tab2 {
        o.extend([
            ((1.8, 4.0), 0.0),
            ((1.8, 3.0), 0.0),
            ((1.3, 3.0), 0.0),
            ((1.3, 4.0), 0.0),
        ]);
    }
    o.push(((1.0, 4.0), 0.0));
    o
}

/// The tab's tip, and the second tab's when `tab2`, on the pocket
/// (`convex`) or the boss over `outline`; with the requested length.
fn far_walled_request(
    outline: &[((f64, f64), f64)],
    convex: bool,
    tab2: bool,
) -> (Body<f64>, Vec<topo::EdgeKey>, f64) {
    let (body, z) = if convex {
        (arced_pocket(outline), 2.0)
    } else {
        (arced_boss(outline), 1.0)
    };
    let mut edges = vec![edge(&body, [3.5, 2.5, z], [2.5, 2.5, z])];
    if tab2 {
        edges.push(edge(&body, [1.8, 3.0, z], [1.3, 3.0, z]));
    }
    (body, edges, if tab2 { 1.5 } else { 1.0 })
}

/// **A far-side arc with no vertex in the tube is clear of the strip.**
/// A plateau of the support rises from the pocket's far wall to
/// `y = 2.5 − base`, with a semicircular bump of radius `0.03` under
/// `x = 3.0625`, midway between two of the tip's screen samples: its
/// apex stands `apex = base − 0.03` from the tip's line, inside the
/// reach meter's tube of radius `D`, its two vertices `base` from it,
/// outside the tube. So `reach::screened`, which reads vertices, finds
/// the far side vacant although an edge of the face lies across the
/// line within the tube: the witness of
/// `work/bandreach/the-strip-screen-reads-a-supports-vertices-not-its-edges.md`'s
/// premise. Alone and with a second tab's tip a chain apart, pocket and
/// boss, both verbs, the band builds at the prism closed form at apex
/// `0.09`; at `0.08` the sampled screen reads the bump off its two
/// samples, less than `D`, and refuses.
#[test]
fn a_far_side_arc_with_no_vertex_in_the_tube_is_clear_of_the_strip() {
    for convex in [true, false] {
        for tab2 in [false, true] {
            for apex in [0.09, 0.08] {
                let y = 2.5 - apex - 0.03;
                let far = [
                    ((2.2, 1.0), 0.0),
                    ((2.2, y), 0.0),
                    ((3.0325, y), -1.0),
                    ((3.0925, y), 0.0),
                    ((3.8, y), 0.0),
                    ((3.8, 1.0), 0.0),
                ];
                let (body, edges, len) = far_walled_request(&far_walled(&far, tab2), convex, tab2);
                for verb in [Verb::Chamfer, Verb::Fillet] {
                    let what = format!("convex {convex}, tab2 {tab2}, apex {apex} ({verb:?})");
                    if apex > 0.085 {
                        let removed = len * verb.section();
                        let removed = if convex { removed } else { -removed };
                        let out = carve(&body, &edges, verb, removed, &what);
                        validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
                            .unwrap_or_else(|err| panic!("{what}: tier 3′, got {err:?}"));
                        continue;
                    }
                    let want = 0.0625_f64.hypot(apex + 0.03) - 0.03;
                    match verb.run(&body, &edges) {
                        Err(BlendError::FaceClearanceUncertified { gap, .. }) => assert!(
                            matches!(gap.diagnostic_f64_for_error_text(), ErrorTextReading::Value(g) if (g - want).abs() < 1e-3),
                            "{what}: the screen refuses off its two samples, near {want}: {gap:?}"
                        ),
                        other => panic!("{what}: the bump refuses at the screen, got {other:?}"),
                    }
                }
            }
        }
    }
}

/// **A far-side peninsula on a cross-chain request.** The peninsula of
/// [`far_side`], its tip a vertex `gap` across the tip's line, so
/// `reach::screened` finds the far side occupied and the cross-chain
/// excuse reads the strip on the trimline's side alone, with a second
/// tab's tip requested a chain apart. Pocket and boss, both verbs:
/// `0.2` and `0.09` build at the prism closed form; at `0.05` the
/// sampled screen refuses. This pins the build; it cannot go red with
/// the excuse read two-sided, which is the excuse's looser reading of
/// a region nothing else of the face reaches here.
#[test]
fn a_far_side_peninsula_on_a_cross_chain_request_builds() {
    for convex in [true, false] {
        for gap in [0.2, 0.09, 0.05] {
            let far = [
                ((3.025, 1.0), 0.0),
                ((3.0625, 2.5 - gap), 0.0),
                ((3.1, 1.0), 0.0),
            ];
            let (body, edges, len) = far_walled_request(&far_walled(&far, true), convex, true);
            for verb in [Verb::Chamfer, Verb::Fillet] {
                let what = format!("convex {convex}, gap {gap} ({verb:?})");
                if gap > 0.085 {
                    let removed = len * verb.section();
                    let removed = if convex { removed } else { -removed };
                    let out = carve(&body, &edges, verb, removed, &what);
                    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
                        .unwrap_or_else(|err| panic!("{what}: tier 3′, got {err:?}"));
                    continue;
                }
                match verb.run(&body, &edges) {
                    Err(BlendError::FaceClearanceUncertified { gap: g, .. }) => assert!(
                        matches!(g.diagnostic_f64_for_error_text(), ErrorTextReading::Value(g) if (g - 0.0625_f64.hypot(gap)).abs() < 1e-9),
                        "{what}: the screen refuses off its two samples: {g:?}"
                    ),
                    other => panic!("{what}: the peninsula refuses at the screen, got {other:?}"),
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
        SketchPlane::from_frame(RigidMotion::axes_yz(Point3::new(-0.1, 4.6, 2.0))),
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
