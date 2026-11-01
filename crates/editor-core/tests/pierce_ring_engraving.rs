//! **A blind pocket engraved in a cylinder's end cap** — Ev's engraving
//! pose (`work/tang/pierce-ring-has-no-join-arm.md`, the "user's blind
//! pocket" recipe), authored node for node through the document doors.
//!
//! A cylinder `r = 0.04` along its own `+z` over `[0, 0.5]`, and a tool
//! drawn on its own xy — a letter-ish outline of two lines, a tangent
//! arc and a bulge arc — extruded `0.01` and placed against the
//! cylinder `0.005` down, so it straddles the `z = 0` cap and its section
//! with the cap is a closed loop strictly inside the disc: a ring in an
//! arc-bounded planar face. `Subtract` cuts it, and the pocket is the
//! tool's profile area times the `0.005` it sinks.
//!
//! The pose refused `SectionLoopMixed` while `point_in_solid`'s planar
//! arm read the arc-bounded cap as the polygon through its vertices; the
//! rows here hold it to the closed form at every tier. The same tool
//! slid until it crosses the rim cuts too: the circle × cylinder root
//! lane certifies where the rim meets the tool's arc wall, and the
//! result's notched wall measures in closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use editor_core::Formula;

use crate::corpus::{body_of, eval};
use crate::fixture::{insert, len, len2, scl, slid};
use editor_core::{
    Evaluation, LoopProgram, Node, NodeResult, ProfileDoc, ProfileProgram, ProgramArcData,
    ProgramStep, ProgramTarget, RecipeNodeId,
};
use geom_core::Tol;

const R: f64 = 0.04;
const HEIGHT: f64 = 0.5;
/// How far the tool sinks below the cap.
const DEPTH: f64 = 0.005;

/// Node 15's outline: down the `x = 0` flank, along the bottom to
/// `x = 0.008`, a tangent arc up to `(0.01, 0)`, and a `0.6` bulge back
/// to the start.
fn letter() -> LoopProgram<Formula> {
    LoopProgram::Chain(vec![
        ProgramStep::At(len2([0.0, 0.02])),
        ProgramStep::LineTo(ProgramTarget::Point(len2([0.0, -0.02]))),
        ProgramStep::LineTo(ProgramTarget::Point(len2([0.008, -0.02]))),
        ProgramStep::Tangent,
        ProgramStep::TangentArcTo(ProgramTarget::Point(len2([0.01, 0.0]))),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Start,
            b: scl(0.6),
        }),
    ])
}

/// The letter's area: the polygon through its four vertices plus the
/// circular segment each arc adds outside its chord (both bow out of
/// the counter-clockwise loop).
///
/// - The tangent arc leaves `(0.008, −0.02)` along `+x`, so its centre
///   is `(0.008, −0.02 + ρ)`; passing through `(0.01, 0)` fixes
///   `ρ = (0.002² + 0.02²) / (2 · 0.02) = 0.0101`.
/// - A bulge `b` subtends `θ = 4·atan b` over its chord `c`, on radius
///   `c / (2 sin(θ/2))`.
fn letter_area() -> f64 {
    let segment = |radius: f64, theta: f64| radius * radius / 2.0 * (theta - theta.sin());
    let polygon = {
        let p = [(0.0, 0.02), (0.0, -0.02), (0.008, -0.02), (0.01, 0.0)];
        (0..4)
            .map(|i| {
                let (a, b) = (p[i], p[(i + 1) % 4]);
                a.0 * b.1 - b.0 * a.1
            })
            .sum::<f64>()
            / 2.0
    };
    let rho = (0.002f64.powi(2) + 0.02f64.powi(2)) / (2.0 * 0.02);
    let centre = (0.008, -0.02 + rho);
    let swept = (0.0 - centre.1).atan2(0.01 - centre.0) - (-0.02 - centre.1).atan2(0.0);
    let theta = 4.0 * 0.6f64.atan();
    let chord = (0.01f64.powi(2) + 0.02f64.powi(2)).sqrt();
    polygon + segment(rho, swept) + segment(chord / (2.0 * (theta / 2.0).sin()), theta)
}

/// The pose as authored: the cylinder, the tool's extrude, its copy
/// placed against the cylinder — sunk `DEPTH` below the start cap and
/// slid `dx` along `x` — and the subtraction.
fn engrave(tool: LoopProgram<Formula>, dx: f64) -> (Evaluation<f64>, [RecipeNodeId; 4]) {
    let doc = ProfileDoc::empty_derived("pierce-ring-engraving", Tol::witness());
    let profile = |lp| {
        Node::Profile(ProfileProgram {
            loops: vec![lp],
            ids: Vec::new(),
            on: None,
        })
    };
    let (doc, disc) = insert(doc, profile(LoopProgram::circle(0.0, 0.0, R).unwrap()));
    let (doc, cylinder) = insert(
        doc,
        Node::Extrude {
            profile: disc.into(),
            distance: len(HEIGHT),
        },
    );
    let (doc, outline) = insert(doc, profile(tool));
    let (doc, prism) = insert(
        doc,
        Node::Extrude {
            profile: outline.into(),
            distance: len(2.0 * DEPTH),
        },
    );
    let (doc, lifted) = slid(doc, cylinder, prism, [dx, 0.0, -DEPTH]);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: cylinder.into(),
            tool: lifted.into(),
        },
    );
    (eval::<f64>(&doc), [cylinder, prism, lifted, cut])
}

/// The pocket cuts, validates at every tier and leaves exactly the
/// cylinder less the profile area times the depth — for the authored
/// letter and for a plain rectangle in its place.
#[test]
fn a_blind_pocket_in_a_cylinder_cap_cuts_to_the_closed_form() {
    let tol = Tol::witness();
    let rectangle =
        LoopProgram::polygon([(0.0, -0.02), (0.01, -0.02), (0.01, 0.02), (0.0, 0.02)]).unwrap();
    for (label, tool, area) in [
        ("letter", letter(), letter_area()),
        ("rectangle", rectangle, 0.01 * 0.04),
    ] {
        let (ev, [_, prism, _, cut]) = engrave(tool, 0.0);
        if let Some(NodeResult::Failed(e)) = ev.nodes.get(&cut) {
            panic!("{label}: the pocket refused: {e}");
        }
        let body = body_of(&ev, cut);
        assert_eq!(topo::validate(&body), Ok(()), "{label}: tier 1");
        assert_eq!(topo::validate_closed(&body), Ok(()), "{label}: closed");
        assert_eq!(
            topo::validate::validate_geometric(&body, tol),
            Ok(()),
            "{label}: tier 3"
        );
        let volume = |b: &topo::Body<f64>| {
            topo::test_support::pinned_volume(
                "mass properties",
                &topo::test_support::finished("the measured body", topo::Body::clone(b), tol),
                tol,
            )
            .volume
        };
        let tool_volume = volume(&(body_of(&ev, prism)));
        assert!(
            (tool_volume - area * 2.0 * DEPTH).abs() < 1e-17,
            "{label}: the tool's prism is its outline's area times its length: \
             {tool_volume} against {}",
            area * 2.0 * DEPTH
        );
        let truth = PI * R * R * HEIGHT - area * DEPTH;
        let got = volume(&body);
        assert!(
            (got - truth).abs() < 1e-15,
            "{label}: pocketed cylinder {got} against the closed form {truth}"
        );
    }
}

/// Slid to `x = 0.035`, the letter crosses the rim: the cylinder's rim
/// CIRCLE meets the tool's arc wall, a CYLINDER, and the circle ×
/// cylinder root lane certifies the crossing. The result's wall is
/// notched by the pocket along rulings and an arc, and measures in
/// closed form.
///
/// The letter covers the whole lens of the disc beyond `x = 0.035`: the
/// lens spans `|y| ≤ √(R² − 0.035²) ≈ 0.0194`, inside the letter's
/// `y = −0.02` bottom edge, and the bulge arc's chord `y = 0.02 − 2x`
/// (local `x`) stays above the circle across it, the arc further out
/// still. So the cut removes `DEPTH` times that circular segment.
#[test]
fn a_pocket_across_the_rim_cuts_to_the_closed_form() {
    let tol = Tol::witness();
    let (ev, [_, _, _, cut]) = engrave(letter(), 0.035);
    if let Some(NodeResult::Failed(e)) = ev.nodes.get(&cut) {
        panic!("the rim-crossing pocket refused: {e}");
    }
    let body = topo::test_support::finished("body", topo::Body::clone(&(body_of(&ev, cut))), tol);
    assert_eq!(topo::validate(&body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&body), Ok(()), "closed");
    assert_eq!(
        topo::validate::validate_geometric(&body, tol),
        Ok(()),
        "tier 3"
    );
    let d = 0.035;
    let segment = R * R * (d / R).acos() - d * (R * R - d * d).sqrt();
    let truth = PI * R * R * HEIGHT - segment * DEPTH;
    let got = topo::test_support::pinned_volume("mass properties", &body, tol).volume;
    assert!(
        (got - truth).abs() < 1e-15,
        "notched cylinder {got} against the closed form {truth}"
    );
}
