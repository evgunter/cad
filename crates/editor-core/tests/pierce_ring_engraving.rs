//! **A blind pocket engraved in a cylinder's end cap** — Ev's engraving
//! pose (`work/tang/pierce-ring-has-no-join-arm.md`, the "user's blind
//! pocket" recipe), authored node for node through the document doors.
//!
//! A cylinder `r = 0.04` along `+y` over `[0, 0.5]` (an XZ-frame circle
//! extruded `−0.5`), and a tool drawn on the same frame — a letter-ish
//! outline of two lines, a tangent arc and a bulge arc — extruded `0.01`
//! and lifted `0.005`, so it straddles the `y = 0` cap and its section
//! with the cap is a closed loop strictly inside the disc: a ring in an
//! arc-bounded planar face. `Subtract` cuts it, and the pocket is the
//! tool's profile area times the `0.005` it sinks.
//!
//! The pose refused `SectionLoopMixed` while `point_in_solid`'s planar
//! arm read the arc-bounded cap as the polygon through its vertices; the
//! rows here hold it to the closed form at every tier. The same tool
//! slid until it crosses the rim is a different door: the circle ×
//! cylinder root lane certifies where the rim meets the tool's arc
//! wall, and the result's notched wall has no volume measurement —
//! pinned by kind so it reds when that measurement lands.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use editor_core::ExtrudeSide;

use crate::corpus::{body_of, eval};
use crate::fixture::{frame, insert, len, len2, scl, xform};
use editor_core::{
    BooleanOp, Evaluation, LoopProgram, Node, NodeErrorKind, NodeResult, ProfileDoc,
    ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId,
};
use geom_core::Tol;

const R: f64 = 0.04;
const HEIGHT: f64 = 0.5;
/// How far the tool sinks below the cap.
const DEPTH: f64 = 0.005;

/// Node 15's outline: down the `x = 0` flank, along the bottom to
/// `x = 0.008`, a tangent arc up to `(0.01, 0)`, and a `0.6` bulge back
/// to the start.
fn letter() -> LoopProgram {
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

/// The pose as authored: the cylinder (node 10), the tool's extrude
/// (node 16) and the subtraction (node 19), with the tool lifted
/// `DEPTH` along `+y` and slid `dx` along `x`.
fn engrave(tool: LoopProgram, dx: f64) -> (Evaluation<f64>, [RecipeNodeId; 4]) {
    let doc = ProfileDoc::empty_derived("pierce-ring-engraving", Tol::witness());
    // The XZ frame: its normal is −y, so an extrude against it runs +y.
    let (doc, xz) = insert(doc, frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]));
    let profile = |plane, lp| {
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![lp],
            ids: Vec::new(),
        })
    };
    let (doc, disc) = insert(doc, profile(xz, LoopProgram::circle(0.0, 0.0, R).unwrap()));
    let (doc, cylinder) = insert(
        doc,
        Node::Extrude {
            profile: disc,
            distance: len(HEIGHT),
            side: ExtrudeSide::Against,
        },
    );
    let (doc, outline) = insert(doc, profile(xz, tool));
    let (doc, prism) = insert(
        doc,
        Node::Extrude {
            profile: outline,
            distance: len(2.0 * DEPTH),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, lifted) = insert(doc, xform(prism, [dx, DEPTH, 0.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: cylinder,
            b: lifted,
            declare: None,
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
        assert_eq!(topo::validate(body), Ok(()), "{label}: tier 1");
        assert_eq!(topo::validate_closed(body), Ok(()), "{label}: closed");
        assert_eq!(
            topo::validate::validate_geometric(body, tol),
            Ok(()),
            "{label}: tier 3"
        );
        let volume = |b| {
            topo::mass_properties(b, tol)
                .expect("mass properties")
                .volume
        };
        let tool_volume = volume(body_of(&ev, prism));
        assert!(
            (tool_volume - area * 2.0 * DEPTH).abs() < 1e-17,
            "{label}: the tool's prism is its outline's area times its length: \
             {tool_volume} against {}",
            area * 2.0 * DEPTH
        );
        let truth = PI * R * R * HEIGHT - area * DEPTH;
        let got = volume(body);
        assert!(
            (got - truth).abs() < 1e-15,
            "{label}: pocketed cylinder {got} against the closed form {truth}"
        );
    }
}

/// Slid to `x = 0.035`, the letter crosses the rim: the cylinder's rim
/// CIRCLE meets the tool's arc wall, a CYLINDER, and the circle ×
/// cylinder root lane certifies the crossing. The cut builds as far as
/// the volume backstop, which cannot measure the result's cylinder
/// wall: notched by the pocket along rulings and an arc, it is no
/// iso-rectangle (`work/props/a-notched-cylinder-wall-has-no-volume-measurement.md`).
/// Pinned by kind and by the props rule that refuses, so it reds when
/// that wall measures.
#[test]
fn a_pocket_across_the_rim_stops_at_the_notched_walls_volume() {
    let (ev, [_, _, _, cut]) = engrave(letter(), 0.035);
    let Some(NodeResult::Failed(e)) = ev.nodes.get(&cut) else {
        panic!("the rim-crossing pocket built; this row's door has moved");
    };
    assert!(
        matches!(
            &e.kind,
            NodeErrorKind::Boolean(topo::BooleanError::VolumeUnmeasured {
                operand: None,
                source: topo::MassPropsError::Face {
                    source: geom_brep::props::PropsError::NotIsoRectangle {
                        what: "props_rim_level"
                    },
                    ..
                },
            })
        ),
        "expected the result's notched wall to stop the volume backstop: {:?}",
        e.kind
    );
}
