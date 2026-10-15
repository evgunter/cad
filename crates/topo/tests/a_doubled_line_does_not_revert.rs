//! **A body with two distinct edges on one line does not revert, and no
//! door that reverts hands one back.**
//!
//! The split's one-end slit (a box less a trench whose keel lies in the
//! plane, the piece above the plane) carries the keel on its section
//! face as two coincident edges, floor | wall a and wall b | floor, each
//! bounding the material wedge between the floor and one wall. Its
//! complement pairs the same four sheets differently: the floor runs
//! flat across the line and the two walls meet in one crease. Reversing
//! each face's side alone leaves each edge bounding `2π` less its own
//! wedge, the two overlapping, and a ∖ result built that way passed
//! tiers 1, 2, 3 and 3′.
//!
//! Every row runs at every pose; the slit runs from `MEET` to [`OPEN`].
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{MEET, Pose, at, posed_box, posed_boxes, poses};
use common::split_plane;
use geom_core::{Point3, Tol, Vec3};
use topo::{
    AtRestBody, Body, BooleanError, BooleanResult, ContactRecords, Operand, RevertError, SplitPart,
    VoidContainment, VoidEvidence, VoidInsertError, insert_void, split, subtract,
    validate_geometric, validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

/// The slit's open end, where the trench leaves the box.
const OPEN: [f64; 3] = [4.0, 1.0, 1.0];

/// The piece of the box less the trench above the plane through the
/// keel, at `pose`: the split's one-end slit.
fn slit_body(pose: &Pose) -> Body<f64> {
    use topo::test_support::{FaceGeometry, describe_as_intersections, finished, prism_ops};
    let tol = t();
    let prism = |what: &str, profile: &[(f64, f64)], span: (f64, f64), axis_x: bool| {
        let mut b = Body::<f64>::new();
        if axis_x {
            prism_ops(
                &mut b,
                profile,
                span,
                |a, c, h| pose.at([h, a, c]),
                FaceGeometry::Certified,
                tol,
            );
        } else {
            prism_ops(
                &mut b,
                profile,
                span,
                |x, y, z| pose.at([x, y, z]),
                FaceGeometry::Certified,
                tol,
            );
        }
        describe_as_intersections(&mut b, tol);
        finished(what, b, tol)
    };
    let block = prism(
        "the box",
        &[(0.0, -1.0), (4.0, -1.0), (4.0, 3.0), (0.0, 3.0)],
        (0.0, 2.0),
        false,
    );
    let trench = prism(
        "the trench",
        &[(1.0, 1.0), (1.6, 3.0), (0.4, 3.0)],
        (1.5, 4.5),
        true,
    );
    let BooleanResult::Body(cut) = subtract(&block, &trench, tol)
        .unwrap_or_else(|e| panic!("{}: the box less the trench: {e:?}", pose.label))
    else {
        panic!("{}: the box less the trench is empty", pose.label);
    };
    let o = pose.at(MEET);
    let n = pose.at([MEET[0], MEET[1], MEET[2] + 1.0]) - o;
    let r = split(
        &cut.body,
        &split_plane(o, Vec3::new(n.x, n.y, n.z), tol),
        tol,
    )
    .unwrap_or_else(|e| panic!("{}: the split: {e:?}", pose.label));
    let SplitPart::Body(above) = r.above else {
        panic!("{}: nothing above the plane", pose.label);
    };
    above
}

/// Asserts `refusal` names the slit: both its edges of `body` run
/// between the points of `MEET` and [`OPEN`] at `pose`.
fn names_the_slit(what: &str, body: &Body<f64>, pose: &Pose, refusal: &RevertError) {
    let RevertError::CoincidentEdges { edges, .. } = refusal else {
        panic!("{what}: {refusal:?}, not the coincident edges");
    };
    let near = |p: Point3<f64>, q: [f64; 3]| p.distance(pose.at(q)) < 1e-9;
    for &edge in edges {
        let he = body.get_edge(edge).unwrap().he_plus;
        let ends = [
            body.get_half_edge(he).unwrap().start,
            body.half_edge_end(he).unwrap(),
        ]
        .map(|v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap());
        assert!(
            ends.iter().any(|&e| near(e, MEET)) && ends.iter().any(|&e| near(e, OPEN)),
            "{what}: {edge:?} runs {:?}, not the keel",
            ends.map(at)
        );
    }
}

/// **The slit body is legal and does not revert.** Tiers 3 and 3′ pass
/// it, and `revert` refuses it naming the keel's two edges.
#[test]
fn the_split_slit_body_is_legal_and_refuses_to_revert() {
    for pose in poses() {
        let what = pose.label;
        let body = slit_body(&pose);
        assert_eq!(validate_geometric(&body, t()), Ok(()), "{what}: tier 3");
        assert_eq!(
            validate_pseudomanifold(&body, &ContactRecords::default(), t()),
            Ok(()),
            "{what}: tier 3′"
        );
        let refusal = body
            .revert()
            .err()
            .unwrap_or_else(|| panic!("{what}: the slit body reverted"));
        names_the_slit(what, &body, &pose, &refusal);
    }
}

/// **∖ refuses a tool whose kept part holds the slit, at every door that
/// reverts it, and answers where the slit is discarded.**
///
/// - A slab `z ∈ [0, 1.5]` round the slit body keeps its floor whole:
///   the seamed finish reverts it.
/// - The same slab ending at `x = 3` cuts the slit across, keeping its
///   first 1.5 units.
/// - A box round the whole slit body keeps it as a cavity: the void door
///   reverts it.
/// - A slab `z ∈ [1.5, 3]` keeps only the slit body's top, whose revert
///   is sound: the slab's 54 less the top's `8 − 2.5 · ½ · 0.6 · (1 − ¼)`.
#[test]
fn subtracting_a_slit_tool_refuses_where_it_keeps_the_slit() {
    for pose in poses() {
        let tool = AtRestBody::validate(slit_body(&pose), t()).unwrap();
        // The seamed finish reads the result's wedges on the line; the
        // containment fallback reads the cavity's line, which nothing
        // joins.
        for (what, slab, seamed) in [
            (
                "the slab under the top",
                [(-1.0, 5.0), (-2.0, 4.0), (0.0, 1.5)],
                true,
            ),
            (
                "the slab across the slit",
                [(-1.0, 3.0), (-2.0, 4.0), (0.0, 1.5)],
                true,
            ),
            (
                "the box round it",
                [(-1.0, 5.0), (-2.0, 4.0), (0.0, 3.0)],
                false,
            ),
        ] {
            let what = format!("{}, {what}", pose.label);
            let a = posed_box(&what, slab, &pose, t());
            match (subtract(&a, &tool, t()), seamed) {
                (
                    Err(BooleanError::Revert {
                        operand: Operand::B,
                        source: RevertError::WedgesOverlap { .. },
                    }),
                    true,
                )
                | (
                    Err(BooleanError::Revert {
                        operand: Operand::B,
                        source: RevertError::CoincidentEdges { .. },
                    }),
                    false,
                ) => {}
                (other, _) => panic!("{what}: {other:?}, not the revert's refusal"),
            }
        }
        let what = format!("{}, the slab over the slit", pose.label);
        let a = posed_box(&what, [(-1.0, 5.0), (-2.0, 4.0), (1.5, 3.0)], &pose, t());
        let BooleanResult::Body(r) =
            subtract(&a, &tool, t()).unwrap_or_else(|e| panic!("{what}: {e:?}"))
        else {
            panic!("{what}: empty");
        };
        let v = topo::mass_properties(&r.body, t()).unwrap().volume;
        let closed = 54.0 - (8.0 - 2.5 * 0.5 * 0.6 * 0.75);
        assert!(
            (v - closed).abs() < 1e-9,
            "{what}: volume {v}, closed form {closed}"
        );
        assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
    }
}

/// **The void door refuses a slit cavity before writing.** The
/// destination is untouched, bit for bit.
#[test]
fn a_slit_cavity_refuses_before_the_destination_is_written() {
    for pose in poses() {
        let what = pose.label;
        let cavity = slit_body(&pose);
        let mut dst =
            posed_box(what, [(-1.0, 5.0), (-2.0, 4.0), (0.0, 3.0)], &pose, t()).into_body();
        let before = format!("{dst:?}");
        let (solid, _) = dst.solids().next().unwrap();
        let evidence = VoidEvidence {
            shells: cavity
                .shells()
                .map(|(s, _)| (s, VoidContainment::Probed(topo::SolidContainment::In)))
                .collect(),
        };
        let refusal = match insert_void(&mut dst, solid, cavity.clone(), &evidence) {
            Err(VoidInsertError::Revert(refusal)) => refusal,
            other => panic!("{what}: {other:?}, not the revert's refusal"),
        };
        names_the_slit(what, &cavity, &pose, &refusal);
        assert_eq!(format!("{dst:?}"), before, "{what}: the destination moved");
    }
}

/// **A pinch of two bricks of unequal heights does not revert through
/// ∖ either.** Bricks over quadrants 1 and 3 touch along the z-axis,
/// the first to `z = 1.5`, the second to `z = 1.25`: the touch is a
/// partial collinear overlap whose upper bound is the second brick's
/// vertex on the first's edge, not one point key. `B − pinch`, with `B`
/// the box `[−0.5, 0.5]² × [0, 1]`, keeps the touch inside `B`, where
/// the complement pairs the two bricks' faces across them; the reversed
/// pair would bound 270° each, and ∖ refuses.
#[test]
fn a_pinch_of_unequal_bricks_refuses_its_revert_through_subtract() {
    for pose in poses() {
        let what = pose.label;
        let q1 = posed_box(what, [(0.0, 1.0), (0.0, 1.0), (0.5, 1.5)], &pose, t());
        let q3 = posed_box(what, [(-1.0, 0.0), (-1.0, 0.0), (0.5, 1.25)], &pose, t());
        let BooleanResult::Body(pinch) = topo::union(&q1, &q3, t()).unwrap() else {
            panic!("{what}: the pinch is empty");
        };
        let b = posed_box(what, [(-0.5, 0.5), (-0.5, 0.5), (0.0, 1.0)], &pose, t());
        match subtract(&b, &pinch.body, t()) {
            Err(BooleanError::Revert {
                operand: Operand::B,
                source: RevertError::WedgesOverlap { .. },
            }) => {}
            other => panic!("{what}: B − pinch: {other:?}, not the revert's refusal"),
        }
    }
}

/// **Two edges of the tool that are nearly parallel and stand apart
/// are not a near-coincidence.** A prism over the profile `(0, 0)`,
/// `(2, 2)`, `(2, 3)`, `(0, 1 + η)` has two diagonals about 0.7 apart
/// whose turn, `η` over the profile, reads in band at `η = 10 ε`; their
/// boxes overlap. Subtracted from the box `[−1, 3] × [−1, 4] × [0.5, 2]`
/// at rest it builds at its closed form, `30 − (2 + η) · h` over the height `h`
/// the box keeps, wherever the diagonals go: kept on the prism's top as
/// it rises through the box's floor (the seamed finish reads the pair,
/// and the gap between their lines decides it apart), discarded as it
/// runs through the box (the kept filter skips them), or kept whole as
/// a cavity (the containment fallback reads every edge).
#[test]
fn a_tool_with_near_parallel_edges_standing_apart_builds() {
    use topo::test_support::{FaceGeometry, describe_as_intersections, finished, prism_ops};
    let tol = t();
    let eta = 10.0 * tol.eps();
    let profile = [(0.0, 0.0), (2.0, 2.0), (2.0, 3.0), (0.0, 1.0 + eta)];
    let a = posed_box(
        "the box",
        [(-1.0, 3.0), (-1.0, 4.0), (0.5, 2.0)],
        &Pose::rest(),
        tol,
    );
    for (what, span, h) in [
        ("rising through the floor", (0.0, 1.0), 0.5),
        ("through the box", (0.0, 3.0), 1.5),
        ("inside the box", (1.0, 1.5), 0.5),
    ] {
        let mut b = Body::<f64>::new();
        prism_ops(
            &mut b,
            &profile,
            span,
            Point3::new,
            FaceGeometry::Certified,
            tol,
        );
        describe_as_intersections(&mut b, tol);
        let tool = finished(what, b, tol);
        let BooleanResult::Body(r) =
            subtract(&a, &tool, tol).unwrap_or_else(|e| panic!("{what}: {e}"))
        else {
            panic!("{what}: empty");
        };
        let v = topo::mass_properties(&r.body, tol).unwrap().volume;
        let closed = 30.0 - (2.0 + eta) * h;
        assert!(
            (v - closed).abs() < 1e-9,
            "{what}: volume {v}, closed form {closed}"
        );
    }
}

/// **Two bricks of the tool a few ε apart on one line escalate, naming
/// only the move that builds.** The tool holds the bricks
/// `[0, 1] × [0, 1] × [0.5, 1.5]` and `[−1, −g] × [−1, 0] × [0.5, 1.5]`,
/// whose edges at the gap lie within `g = 4 ε` of one line inside the
/// box `[−0.5, 0.5]² × [0, 2]`. Whether they share it is in band:
/// ∖ refuses `LineEdges`, whose lever is the bricks clearly apart, not
/// clearly on one line (which ∖ refuses as reversed wedges). Moved
/// clearly apart, `g = 0.25`, it builds: the box's 2 less the two
/// bricks inside it, `0.25 + 0.5 · (0.5 − g)`.
#[test]
fn bricks_a_hair_apart_on_one_line_escalate_with_the_move_that_builds() {
    let tol = t();
    let tool = |g: f64| {
        posed_boxes(
            "the bricks",
            &[
                [(0.0, 1.0), (0.0, 1.0), (0.5, 1.5)],
                [(-1.0, -g), (-1.0, 0.0), (0.5, 1.5)],
            ],
            &Pose::rest(),
            tol,
        )
    };
    let a = posed_box(
        "the box",
        [(-0.5, 0.5), (-0.5, 0.5), (0.0, 2.0)],
        &Pose::rest(),
        tol,
    );
    let refusal = subtract(&a, &tool(4.0 * tol.eps()), tol).err();
    let Some(
        refusal @ BooleanError::Escalated {
            decision: topo::BooleanDecision::LineEdges,
            diag,
        },
    ) = refusal
    else {
        panic!("{refusal:?}, not the line's escalation");
    };
    assert_eq!(
        refusal.to_string(),
        format!(
            "whether two straight edges of the subtracted solid lie on one line along a length \
             is undecided: margin {:e} lies inside the ambiguity band ({:e}, {:e}). Recourse: move the \
             geometry so the subtracted solid's straight edges clearly stand apart",
            diag.margin,
            diag.band.zero(),
            diag.band.escalate()
        )
    );
    let g = 0.25;
    let BooleanResult::Body(r) =
        subtract(&a, &tool(g), tol).unwrap_or_else(|e| panic!("moved apart: {e}"))
    else {
        panic!("moved apart: empty");
    };
    let v = topo::mass_properties(&r.body, tol).unwrap().volume;
    let closed = 2.0 - 0.25 - 0.5 * (0.5 - g);
    assert!(
        (v - closed).abs() < 1e-9,
        "moved apart: volume {v}, closed form {closed}"
    );
}
