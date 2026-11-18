//! **A meet a construction states is a claim** (fork 288): where one
//! plane is a spin of the other about a line that is a meet of the
//! other, the two meet in that line whatever the angle, and the mint
//! returns it rather than solving for it.

#![allow(clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::pose::{PoseDef, RoleRead};
use editor_core::{Node, Operand};
use fixture::{Recorder, len};
use geom_core::Tol;

/// A unit cube, and a copy of it whose top edge along `x` is mated to
/// the meet of `held` and `spun`, `spun` the cube's top turned by
/// `angle` about that edge. Where the copy stands: its placement's
/// origin.
fn placed_on_a_meet(held_z: f64, angle: f64) -> [f64; 3] {
    let mut r = Recorder::new();
    let p = r.insert(Node::Profile(editor_core::ProfileProgram {
        loops: vec![
            editor_core::LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                .expect("finite square corners"),
        ],
        ids: Vec::new(),
        on: None,
    }));
    let block = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    let top = fixture::plane_on(&r.doc, block, [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]);
    let held = fixture::plane_on(&r.doc, block, [0.0, 0.0, held_z], [0.0, 0.0, 1.0]);
    let edge = fixture::axis_on(&r.doc, block, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    let spun = RoleRead::plain(Operand::Pose(Box::new(PoseDef::Spin {
        pose: top,
        about: edge.clone(),
        by: fixture::ang(angle),
    })));
    let meet = Operand::Pose(Box::new(PoseDef::Meet { a: held, b: spun }));
    let target = fixture::space(&r.doc, block);
    let placed = editor_core::Placed::of(
        &r.doc,
        vec![fixture::mate(edge.read.clone(), meet)],
        Vec::new(),
    );
    let copy = r.insert(Node::place(
        editor_core::BodyRead::plain(Operand::Node(block)),
        Operand::Var(target),
        placed.bundle(),
    ));
    let ev = editor_core::evaluate::<f64>(
        &r.doc,
        None,
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    );
    let framed = match ev.value(copy).map(|v| &v.payload) {
        Some(editor_core::ValuePayload::Body(b)) => b.clone(),
        other => panic!("the copy places: {other:?}"),
    };
    let o = editor_core::in_space(&framed).map_or(geom_core::Point3::origin(), |m| m.origin());
    [o.x, o.y, o.z]
}

/// **The claim fires on its shape**: the cube's top and the top turned
/// about its own edge by an angle 11ε meet in that edge exactly, so the
/// copy stands at the origin to the bit. The solve divides by the
/// angle and rounds the edge off by about 1.4e-8 here.
#[test]
fn a_plane_and_its_spin_about_its_own_meet_meet_in_that_line() {
    let at = placed_on_a_meet(1.0, 11.0 * Tol::witness().eps());
    assert_eq!(at, [0.0, 0.0, 0.0], "the edge is the meet by construction");
}

/// **The claim does not fire on another plane**: the floor and the top
/// turned a quarter about the top's edge meet on the floor, a unit
/// below the edge, so the copy's edge stands there. A claim read off
/// the spin alone would put it back on the edge.
#[test]
fn a_spin_of_another_plane_meets_by_the_solve() {
    let at = placed_on_a_meet(0.0, std::f64::consts::FRAC_PI_2);
    assert!(
        at[0].abs() < 1e-12 && at[1].abs() < 1e-12 && (at[2] + 1.0).abs() < 1e-12,
        "the meet is on the floor: {at:?}"
    );
}
