//! **A placement value names its direction** (D10 `[placement.values]`):
//! a slide along a direction read off a body in the target space moves
//! the copy exactly its value along it, beside slides along target axes,
//! whose sum it joins; a spin about a read axis turns about that line;
//! a read direction the other slides lie beside, or one tilted out of
//! the plane a rest leaves, refuses typed at evaluation; a read of the
//! wrong kind refuses at the door; the mated direction's slide refuses
//! beside a plane mate and with no mate; a read-back beside a read line
//! reads no slide; and the forms persist.

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

/// The wedge's vertical edge through `(1, 0)`: the line its floor wall
/// `y = 0` (outward `−y`) and its slanted wall meet, along
/// `(−y) × (1, 1, 0)/√2 = +z`.
fn edge(doc: &ProfileDoc, wedge: RecipeNodeId) -> RoleRead<Operand> {
    RoleRead::plain(Operand::Pose(Box::new(PoseDef::Meet {
        a: RoleRead::plain(plane(doc, wedge, [0.0, 1.0, 0.0])),
        b: RoleRead::plain(plane(doc, wedge, [1.0, 1.0, 0.0])),
    })))
}

/// **A spin about a read axis turns the copy about that line, in its
/// sense**: a quarter turn about the vertical edge through `(1, 0)`,
/// along `+z`, carries the copy's origin from `(1, 0)`'s `−x` side to
/// its `−y` side, at its height: `(1, −1, 0)`, right-handed.
#[test]
fn a_spin_about_a_read_axis_turns_about_that_line() {
    let (doc, wedge) = wedge();
    let placed = Placed::unmated(vec![(
        Coord::SpinAbout(edge(&doc, wedge)),
        editor_core::test_support::ang(core::f64::consts::FRAC_PI_2),
    )]);
    let (_, at) = copy(doc, wedge, placed);
    let at = at.expect("the bundle places");
    let want = [1.0, -1.0, 0.0];
    assert!(
        at.translation
            .iter()
            .zip(want)
            .all(|(g, w)| (g - w).abs() < 1e-12),
        "the origin turns a right-handed quarter about the edge through (1, 0): {:?}",
        at.translation
    );
}

/// **The read-back of a chain turning about a read line off the origin
/// reads no slide**: the line moves the copy's origin, and the caller of
/// `read_back` does not hold it, so a slide read as the origin's
/// coordinate would write the turn's offset in again. Every slide reads
/// `None`, never the `(1, −1, 0)` the turn put the origin at.
#[test]
fn the_read_back_beside_a_spin_about_a_read_axis_reads_no_slide() {
    let (doc, wedge) = wedge();
    let placed = Placed::unmated(vec![(
        Coord::SpinAbout(edge(&doc, wedge)),
        editor_core::test_support::ang(core::f64::consts::FRAC_PI_2),
    )]);
    let coords: Vec<Coord<Operand>> = placed.values.iter().map(|(c, _)| c.clone()).collect();
    let (_, at) = copy(doc, wedge, placed);
    let at = at.expect("the bundle places");
    let t = at.translation;
    let affine = geom_core::linalg::Affine3::from_parts(
        geom_core::linalg::Mat3::identity(),
        geom_core::linalg::Vec3::new(t[0], t[1], t[2]),
    );
    let read = editor_core::mate::read_back(None, &affine, &coords, fixture::band());
    for (coord, read) in coords.iter().zip(&read) {
        if coord.is_slide() {
            assert_eq!(*read, None, "{} reads no number: {read:?}", coord.name());
        }
    }
}

/// A unit block rested by its bottom on the wedge's bottom (both facing
/// `−z`, equated plainly) in the wedge's space, by the values `values`
/// writes over the wedge; where it stands, or the fold's fault.
fn rested(
    values: impl FnOnce(&ProfileDoc, RecipeNodeId) -> Vec<(Coord<Operand>, editor_core::Formula)>,
) -> Result<Frame, String> {
    let (doc, wedge) = wedge();
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (0.2, 0.0), (0.2, 0.2), (0.0, 0.2)]],
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(0.2),
        },
    );
    let placed = Placed {
        mates: vec![fixture::mate(bottom(block), bottom(wedge))],
        values: values(&doc, wedge),
    };
    let space = fixture::space(&doc, wedge);
    let (doc, placement) = insert(
        doc,
        Node::place(
            editor_core::BodyRead::plain(Operand::Node(block)),
            Operand::Var(space),
            placed.bundle(),
        ),
    );
    let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
    match poses.fault(placement) {
        Some(fault) => Err(format!("{fault:?}")),
        None => poses
            .relative(placement)
            .ok_or_else(|| "unsolved".to_owned()),
    }
}

/// An extrude's bottom cap, read as its plane.
fn bottom(body: RecipeNodeId) -> Operand {
    fixture::plane(fixture::head(fixture::fname(
        body,
        editor_core::RoleSeg::Cap(editor_core::CapEnd::Start),
    )))
}

/// The slanted wall's normal turned by `tilt` about the wedge's bottom
/// edge along `x`, out of the floor's plane for any tilt but zero.
fn slant_tilted(doc: &ProfileDoc, wedge: RecipeNodeId, tilt: f64) -> RoleRead<Operand> {
    let edge = Operand::Pose(Box::new(PoseDef::Meet {
        a: RoleRead::plain(bottom(wedge)),
        b: RoleRead::plain(plane(doc, wedge, [0.0, 1.0, 0.0])),
    }));
    RoleRead::plain(Operand::Pose(Box::new(PoseDef::Spin {
        pose: slant(doc, wedge),
        about: RoleRead::plain(edge),
        by: editor_core::test_support::ang(tilt),
    })))
}

/// The rest's values: a slide along `along` by `by`, `x` zero, the spin
/// zero.
fn along_the_rest(
    along: RoleRead<Operand>,
    by: f64,
) -> Vec<(Coord<Operand>, editor_core::Formula)> {
    vec![
        (Coord::SlideAlong(along), len(by)),
        (Coord::slide(TargetAxis::X), len(0.0)),
        (Coord::SPIN, editor_core::test_support::ang(0.0)),
    ]
}

/// **A read direction in the plane a rest leaves steps its value in
/// it**: the block on the wedge's bottom, its zero at the plane's foot of
/// the origin, slid 0.3 and then 0.6 along the slanted wall's normal,
/// stands `0.3·(1, 1, 0)/√2` further each time, on the plane.
#[test]
fn a_read_slide_beside_a_rest_steps_its_value_in_the_plane() {
    let a = rested(|doc, w| along_the_rest(slant(doc, w), 0.3)).expect("the rest places");
    let b = rested(|doc, w| along_the_rest(slant(doc, w), 0.6)).expect("the rest places");
    let h = core::f64::consts::FRAC_1_SQRT_2;
    let step = [0.3 * h, 0.3 * h, 0.0];
    for (k, step) in step.iter().enumerate() {
        assert!(
            (a.translation[k] - step).abs() < 1e-12
                && (b.translation[k] - a.translation[k] - step).abs() < 1e-12,
            "coordinate {k}: the block stands at {:?}, then {:?}",
            a.translation,
            b.translation
        );
    }
}

/// **A read direction tilted out of the plane a rest leaves refuses
/// typed**, decided at evaluation, never re-read along its shadow: the
/// slanted normal turned 0.3 and 0.001 rad out of the floor.
#[test]
fn a_read_slide_tilted_out_of_the_plane_refuses_typed() {
    for tilt in [0.3, 1e-3] {
        let fault = rested(|doc, w| along_the_rest(slant_tilted(doc, w, tilt), 0.3))
            .expect_err("a read across the plane refuses");
        assert!(
            fault.contains("ReadSlidesDoNotSpan"),
            "tilted {tilt}: the fault names the read slides: {fault}"
        );
    }
}

/// **The mated direction's slide refuses beside a plane mate**, which
/// fixes it: the door names it.
#[test]
fn the_mated_slide_refuses_beside_a_rest() {
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
    let top = fixture::plane(fixture::head(fixture::fname(
        wedge,
        editor_core::RoleSeg::Cap(editor_core::CapEnd::End),
    )));
    let bundle = Placed {
        mates: vec![fixture::mate(bottom(wedge), fixture::flip(top))],
        values: vec![
            (Coord::SLIDE, len(0.1)),
            (Coord::slide(TargetAxis::X), len(0.0)),
            (Coord::SPIN, editor_core::test_support::ang(0.0)),
        ],
    }
    .bundle();
    let fault = fixture::bundle_refusal(&doc, placement, bundle);
    assert!(
        matches!(fault, MateFault::Overconstrained { .. }),
        "the mated slide is named: {fault:?}"
    );
}

/// **A read of the wrong kind at a value's seat is refused at the door**,
/// never reaching the fold: a plane read plainly at a slide's
/// direction, a plane's origin through a role it does not offer, and a
/// direction at a spin's axis.
#[test]
fn a_read_of_the_wrong_kind_at_a_values_seat_is_refused_at_the_door() {
    type Case = Box<dyn Fn(&ProfileDoc, RecipeNodeId) -> Coord<Operand>>;
    let cases: Vec<(&str, Case)> = vec![
        (
            "a plane at a slide's direction",
            Box::new(|doc, w| Coord::SlideAlong(RoleRead::plain(plane(doc, w, [1.0, 1.0, 0.0])))),
        ),
        (
            "a plane's origin at a slide's direction",
            Box::new(|doc, w| {
                Coord::SlideAlong(RoleRead::through(
                    plane(doc, w, [1.0, 1.0, 0.0]),
                    PoseRole::Origin,
                ))
            }),
        ),
        (
            "a direction at a spin's axis",
            Box::new(|doc, w| Coord::SpinAbout(slant(doc, w))),
        ),
    ];
    for (label, coord) in cases {
        let (doc, wedge) = wedge();
        let space = fixture::space(&doc, wedge);
        let coord = coord(&doc, wedge);
        let by = if coord.is_slide() {
            len(0.1)
        } else {
            editor_core::test_support::ang(0.1)
        };
        let insert = editor_core::DocEdit::InsertNode {
            node: Box::new(Node::place(
                editor_core::BodyRead::plain(Operand::Node(wedge)),
                Operand::Var(space),
                Placed::unmated(vec![(coord, by)]).bundle(),
            )),
            fresh: Vec::new(),
        };
        let refused = editor_core::apply(&doc, &insert, Tol::witness());
        assert!(
            matches!(refused, Err(editor_core::EditError::SlotVarKind { .. })),
            "{label}: the door refuses the read's kind, not {:?}",
            refused.err()
        );
    }
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
    let about = edge(&doc, wedge);
    let (doc, at) = copy(
        doc,
        wedge,
        Placed::unmated(vec![
            (Coord::SlideAlong(along), len(0.3)),
            (Coord::slide(TargetAxis::X), len(0.2)),
            (Coord::slide(TargetAxis::Z), len(0.5)),
            (
                Coord::SpinAbout(about),
                editor_core::test_support::ang(0.25),
            ),
        ]),
    );
    at.expect("the bundle places");
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = editor_core::persist::load(&text, Tol::witness()).expect("the document loads");
    let again = editor_core::persist::save(&loaded.snapshot, &loaded.edits, Tol::witness())
        .expect("it saves again");
    assert_eq!(text, again, "the saved text round-trips");
    let coords: Vec<Coord> = loaded
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
        .filter_map(|c| c.value().map(|v| v.coord))
        .collect();
    let along = coords
        .iter()
        .find_map(|c| match c {
            Coord::SlideAlong(read) => Some(*read),
            _ => None,
        })
        .expect("a value slides along a read direction");
    assert_eq!(along.role, Some(PoseRole::Normal), "its role is kept");
    assert_eq!(
        coords
            .iter()
            .filter(|c| matches!(c, Coord::SpinAbout(_)))
            .count(),
        1,
        "the spin about a read axis is kept: {coords:?}"
    );
}
