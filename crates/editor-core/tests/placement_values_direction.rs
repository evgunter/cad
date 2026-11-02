//! **A placement value names its direction** (D10 `[placement.values]`):
//! a slide along a direction read off a body in the target space moves
//! the copy exactly its value along it, beside slides along target axes,
//! whose sum it joins; a spin about a read axis turns about that line;
//! a read direction the other slides lie beside refuses typed at
//! evaluation; the mated direction's slide refuses with no mate; and the
//! forms persist.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    Coord, EvalOptions, Frame, MateFault, Node, Operand, Placed, ProfileDoc, RecipeNodeId,
    TargetAxis,
};
use fixture::{insert, len, on_frame, solve};
use geom_core::Tol;

/// A prism on the right triangle `(0,0), (1,0), (0,1)`, a unit tall:
/// its slanted wall's normal runs along `(1, 1, 0)`.
fn wedge() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(
        editor_core::DocumentId::derive("placement-values-direction"),
        Tol::witness(),
    );
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// The wedge's flat face whose normal runs along `normal`, read as a
/// plane: found by its carrier's axis on the evaluated wedge.
fn plane(doc: &ProfileDoc, wedge: RecipeNodeId, normal: [f64; 3]) -> Operand {
    let ev = fixture::run(doc, &EvalOptions::default());
    let n = normal.iter().map(|c| c * c).sum::<f64>().sqrt();
    let name = editor_core::all_faces(&ev, wedge)
        .into_iter()
        .find(|name| {
            editor_core::face_frame(&ev, wedge, name).is_ok_and(|pose| {
                let a = [pose.axis.x, pose.axis.y, pose.axis.z];
                let along: f64 = a.iter().zip(normal).map(|(c, u)| c * u / n).sum();
                (along.abs() - 1.0).abs() < 1e-12
            })
        })
        .unwrap_or_else(|| panic!("the wedge has a face across {normal:?}"));
    Operand::Pose(Box::new(PoseDef::Plane {
        face: Operand::select(Operand::Node(wedge), vec![name]),
    }))
}

/// The slanted wall's outward normal, a direction read through the
/// plane's role.
fn slant(doc: &ProfileDoc, wedge: RecipeNodeId) -> RoleRead<Operand> {
    RoleRead::through(plane(doc, wedge, [1.0, 1.0, 0.0]), PoseRole::Normal)
}

/// A copy of the wedge placed into its own space by `placed`, and
/// where it stands there — or the fold's fault.
fn copy(
    doc: ProfileDoc,
    wedge: RecipeNodeId,
    placed: Placed,
) -> (ProfileDoc, Result<Frame, String>) {
    let space = fixture::space(&doc, wedge);
    let (doc, placement) = insert(
        doc,
        Node::place(
            editor_core::BodyRead::plain(Operand::Node(wedge)),
            Operand::Var(space),
            placed.bundle(),
        ),
    );
    let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
    let at = match poses.fault(placement) {
        Some(fault) => Err(format!("{fault:?}")),
        None => poses
            .relative(placement)
            .ok_or_else(|| "unsolved".to_owned()),
    };
    (doc, at)
}

/// **A slide along a read direction moves the copy exactly its value
/// along it, and the slides sum**: along the slanted wall's normal by
/// 0.3, beside `x` by 0.2 and `z` by 0.5, the copy stands at
/// `0.3·(1, 1, 0)/√2 + (0.2, 0, 0.5)` — where reading each slide as the
/// copy's origin measured along its direction would put it elsewhere.
#[test]
fn a_slide_along_a_read_direction_steps_its_value_and_the_slides_sum() {
    let (doc, wedge) = wedge();
    let along = slant(&doc, wedge);
    let placed = Placed::unmated(vec![
        (Coord::SlideAlong(along), len(0.3)),
        (Coord::slide(TargetAxis::X), len(0.2)),
        (Coord::slide(TargetAxis::Z), len(0.5)),
    ]);
    let (_, at) = copy(doc, wedge, placed);
    let at = at.expect("the bundle places");
    let h = core::f64::consts::FRAC_1_SQRT_2;
    let want = [0.3 * h + 0.2, 0.3 * h, 0.5];
    assert!(
        at.translation
            .iter()
            .zip(want)
            .all(|(g, w)| (g - w).abs() < 1e-12),
        "the copy stands at {:?}, not {want:?}",
        at.translation
    );
    assert_eq!(at.columns, Frame::IDENTITY.columns, "no spin is written");
}

/// **A read direction in the plane of the other two slides refuses
/// typed at evaluation**: the slanted normal beside `x` and `y` spans
/// no translation along `z`, so the three are not independent — a
/// `NoChart` naming the read, decided where it evaluates.
#[test]
fn a_read_direction_beside_slides_it_lies_among_refuses_typed() {
    let (doc, wedge) = wedge();
    let placed = Placed::unmated(vec![
        (Coord::SlideAlong(slant(&doc, wedge)), len(0.3)),
        (Coord::slide(TargetAxis::X), len(0.0)),
        (Coord::slide(TargetAxis::Y), len(0.0)),
    ]);
    let (_, at) = copy(doc, wedge, placed);
    let fault = at.expect_err("the slides do not span");
    assert!(
        fault.contains("ReadSlidesDoNotSpan"),
        "the fault names the read slides: {fault}"
    );
}

/// **A spin about a read axis turns the copy about that line**: a
/// quarter turn about the line where the wedge's floor wall `y = 0` and
/// its slanted wall meet, the vertical edge through `(1, 0)`, carries
/// the copy's origin a unit across it, at its height.
#[test]
fn a_spin_about_a_read_axis_turns_about_that_line() {
    let (doc, wedge) = wedge();
    let edge = Operand::Pose(Box::new(PoseDef::Meet {
        a: RoleRead::plain(plane(&doc, wedge, [0.0, 1.0, 0.0])),
        b: RoleRead::plain(plane(&doc, wedge, [1.0, 1.0, 0.0])),
    }));
    let placed = Placed::unmated(vec![(
        Coord::SpinAbout(RoleRead::plain(edge)),
        editor_core::test_support::ang(core::f64::consts::FRAC_PI_2),
    )]);
    let (_, at) = copy(doc, wedge, placed);
    let at = at.expect("the bundle places");
    let [x, y, z] = at.translation;
    assert!(
        (x - 1.0).abs() < 1e-12 && ((y.abs() - 1.0).abs() < 1e-12) && z.abs() < 1e-12,
        "the origin turns a quarter about the edge through (1, 0): {:?}",
        at.translation
    );
}

/// **The mated direction's slide refuses where no mate fixes one**, as
/// the axisless spin does: the door names it.
#[test]
fn the_mated_slide_refuses_with_no_mate() {
    let (doc, wedge) = wedge();
    let space = fixture::space(&doc, wedge);
    let (doc, placement) = insert(
        doc,
        Node::place(
            editor_core::BodyRead::plain(Operand::Node(wedge)),
            Operand::Var(space),
            Placed::identity().bundle(),
        ),
    );
    let mut bundle = Placed::identity().bundle();
    bundle.push(editor_core::Constraint::of_value(
        editor_core::ConstraintId::placeholder(99),
        editor_core::Value {
            coord: Coord::SLIDE,
            by: len(0.1),
        },
    ));
    let fault = fixture::bundle_refusal(&doc, placement, bundle);
    assert!(
        matches!(fault, MateFault::Overconstrained { .. }),
        "the mated slide is named: {fault:?}"
    );
}

/// **The new forms persist**: a bundle sliding along a read direction
/// through a role and spinning about a read axis saves, loads and saves
/// again to the same text, the read and its role kept.
#[test]
fn the_new_forms_persist() {
    let (doc, wedge) = wedge();
    let along = slant(&doc, wedge);
    let (doc, at) = copy(
        doc,
        wedge,
        Placed::unmated(vec![
            (Coord::SlideAlong(along), len(0.3)),
            (Coord::slide(TargetAxis::X), len(0.2)),
            (Coord::slide(TargetAxis::Z), len(0.5)),
        ]),
    );
    at.expect("the bundle places");
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = editor_core::persist::load(&text, Tol::witness()).expect("the document loads");
    let again = editor_core::persist::save(&loaded.snapshot, &loaded.edits, Tol::witness())
        .expect("it saves again");
    assert_eq!(text, again, "the saved text round-trips");
    let read = loaded
        .snapshot
        .placements()
        .into_iter()
        .filter_map(|p| {
            loaded.snapshot.node(p).and_then(|n| match n {
                Node::Place { constraints, .. } => Some(constraints.clone()),
                _ => None,
            })
        })
        .flatten()
        .find_map(|c| c.value().and_then(|v| v.coord.read().copied()))
        .expect("a value reads a direction");
    assert_eq!(read.role, Some(PoseRole::Normal), "its role is kept");
}
