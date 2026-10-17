//! **INTENT stage 3 PR A′ — a pose is moved by lengths and angles**
//! (`docs/INTENT-STAGE3-SPEC.md`, "A′ (FORK-MOVE)"): a read carries the
//! role it reads a finer pose through, a pose is moved by a `Slide`
//! along or a `Spin` about what it reads, and a lone `0` is the constant
//! zero of its slot's dimension (VR6).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, ang, len};
use editor_core::eval::bound_pose;
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CancelToken, CapEnd, Dimension, DocEdit, EditError, EvalOptions, Evaluation, ExtrudeSide,
    Formula, Node, NodeResult, Operand, OperandSlot, PoseValue, ProfileDoc, RecipeNodeId, SlotId,
    SlotValue, TubeWindow, VarDef, VarId, evaluate,
};
use geom_core::{Tol, UnitVec3};

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn pose(def: PoseDef<Operand, Formula>) -> Operand {
    Operand::Pose(Box::new(def))
}

fn near(a: f64, b: f64, what: &str) {
    assert!((a - b).abs() <= 1e-12, "{what}: {a} vs {b}");
}

/// A unit box as an extruded square on the world xy plane.
fn unit_box() -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("s3ap_box", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

/// A frame at `(1, 2, 3)` whose x is world +y and y world +z, so its z
/// is world +x: no axis of it is the world's own.
fn turned_frame(doc: ProfileDoc) -> (ProfileDoc, VarId) {
    let (doc, frame) = fixture::insert(
        doc,
        fixture::frame([1.0, 2.0, 3.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
    );
    let var = doc.output(frame, 0).expect("a frame defines its pose");
    (doc, var)
}

/// **A frame's x and y roles read** (D10: the role is part of the
/// read): through x its axis along `u`, through y along `v`, through z
/// along its normal, each through its origin; through its yz and zx
/// planes the planes normal to `u` and `v`; and the point at `(x, y, z)`
/// in it is its origin slid along its axes. A ring about the frame's y
/// and a split by its zx plane read the frame through those roles at
/// their seats, and build.
#[test]
fn a_frames_axes_and_planes_read_through_their_roles() {
    let (doc, cube) = unit_box();
    let (doc, frame) = turned_frame(doc);
    let ev = eval(&doc);
    let through = |role| RoleRead::through(frame, role);
    let axis = |role| match bound_pose(&doc, &ev, OperandSlot::Axis, through(role), Tol::witness())
    {
        Ok(PoseValue::Axis { origin, dir }) => (origin, UnitVec3::get(dir)),
        other => panic!("{role}: an axis, not {other:?}"),
    };
    for (role, along) in [
        (PoseRole::X, [0.0, 1.0, 0.0]),
        (PoseRole::Y, [0.0, 0.0, 1.0]),
        (PoseRole::Z, [1.0, 0.0, 0.0]),
    ] {
        let (origin, dir) = axis(role);
        for (got, want, what) in [
            (origin.x, 1.0, "origin x"),
            (origin.y, 2.0, "origin y"),
            (origin.z, 3.0, "origin z"),
            (dir.x, along[0], "direction x"),
            (dir.y, along[1], "direction y"),
            (dir.z, along[2], "direction z"),
        ] {
            near(got, want, &format!("{role} {what}"));
        }
    }
    for (role, normal) in [
        (PoseRole::Yz, [0.0, 1.0, 0.0]),
        (PoseRole::Zx, [0.0, 0.0, 1.0]),
    ] {
        let Ok(PoseValue::Plane { normal: n, .. }) =
            bound_pose(&doc, &ev, OperandSlot::Tool, through(role), Tol::witness())
        else {
            panic!("{role}: a plane");
        };
        let n = UnitVec3::get(n);
        near(n.x, normal[0], &format!("{role} normal x"));
        near(n.y, normal[1], &format!("{role} normal y"));
        near(n.z, normal[2], &format!("{role} normal z"));
    }

    // The point at (0.5, 0.25, -1) in the frame.
    let (doc, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: RoleRead::through(Operand::Var(frame), PoseRole::Zx),
        },
    );
    let (doc, ring) = fixture::insert(
        doc,
        Node::Pattern {
            input: Operand::output(cube, 0),
            count: Formula::count(2),
            kind: editor_core::PatternKind::Circular {
                axis: RoleRead::through(Operand::Var(frame), PoseRole::Y),
                step: ang(std::f64::consts::FRAC_PI_2),
            },
        },
    );
    let (doc, tube) = fixture::insert(
        doc,
        Node::Tube {
            frame: pose(PoseDef::Through {
                axis: RoleRead::through(Operand::Var(frame), PoseRole::Z),
                point: RoleRead::plain(pose(PoseDef::point_in(
                    &Operand::Var(frame),
                    [len(0.5), len(0.25), len(-1.0)],
                ))),
            }),
            major_radius: len(2.0),
            window: TubeWindow::Full,
            minor_radius: len(0.25),
        },
    );
    let ev = eval(&doc);
    for (node, what) in [(split, "the split"), (ring, "the ring"), (tube, "the tube")] {
        assert!(
            matches!(ev.nodes.get(&node), Some(NodeResult::Ok(_))),
            "{what}: {:?}",
            ev.nodes.get(&node)
        );
    }
    let Some(Node::Tube { frame: at, .. }) = doc.node(tube) else {
        panic!("the tube");
    };
    let Some(VarDef::Pose(PoseDef::Through { point, .. })) = doc.var(*at).map(|v| v.def()) else {
        panic!("a frame through an axis and a point");
    };
    let Ok(PoseValue::Point { position }) =
        bound_pose(&doc, &ev, OperandSlot::Frame, point.read, Tol::witness())
    else {
        panic!("a point");
    };
    // origin + 0.5 u + 0.25 v - 1 w = (1, 2, 3) + (0, 0.5, 0) + (0, 0, 0.25) - (1, 0, 0).
    near(position.x, 0.0, "the point's x");
    near(position.y, 2.5, "the point's y");
    near(position.z, 3.25, "the point's z");
}

/// **A role whose projection the seat does not admit refuses**: a
/// frame's x axis at a split's tool reads an axis where a plane is
/// read, and a plane read through a frame's role is no frame at all.
#[test]
fn a_role_the_seat_does_not_admit_refuses() {
    let (doc, cube) = unit_box();
    let (doc, frame) = turned_frame(doc);
    let refused = fixture::insert_refused(
        &doc,
        Node::Pattern {
            input: Operand::output(cube, 0),
            count: Formula::count(2),
            kind: editor_core::PatternKind::Circular {
                axis: RoleRead::through(Operand::Var(frame), PoseRole::Xy),
                step: ang(std::f64::consts::FRAC_PI_2),
            },
        },
    );
    assert!(
        matches!(
            &refused,
            EditError::SlotVarKind {
                found: editor_core::VarKind::Plane,
                ..
            }
        ),
        "{refused:?}"
    );
    // Inside a definition: a slide is along a direction or an axis, and
    // a frame's xy plane is neither.
    let refused = fixture::insert_refused(
        &doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: pose(PoseDef::Slide {
                pose: RoleRead::through(Operand::Var(frame), PoseRole::Zx),
                along: RoleRead::through(Operand::Var(frame), PoseRole::Xy),
                by: len(0.5),
            })
            .into(),
        },
    );
    assert!(
        matches!(
            &refused,
            EditError::PoseShape {
                fault: editor_core::pose::PoseFault::Role {
                    role: PoseRole::Xy,
                    ..
                },
                ..
            }
        ),
        "{refused:?}"
    );
}

/// **A self-read through a role is one reader** (VR7, at the edit and
/// the load doors): a tube's frame, an unnamed pose its seat alone
/// reads, re-pointed to that frame spun about its own z axis, is read
/// by one definition twice — once plainly and once through its role —
/// and that is one reader. A second seat reading it is a second reader,
/// and refuses.
#[test]
fn a_self_read_through_a_role_is_one_reader() {
    let (doc, cube) = unit_box();
    let edge = fixture::rim_edge(cube, CapEnd::End, fixture::piece(&doc, cube, 0, 0));
    let corner = fixture::cap_vertex(cube, CapEnd::End, fixture::vpiece(&doc, cube, 0, 2));
    let (doc, tube) = fixture::insert(
        doc,
        Node::Tube {
            frame: pose(PoseDef::Through {
                axis: RoleRead::plain(pose(PoseDef::Axis {
                    of: Operand::select(Operand::output(cube, 0), vec![edge]),
                })),
                point: RoleRead::plain(pose(PoseDef::Point {
                    of: Operand::select(Operand::output(cube, 0), vec![corner]),
                })),
            }),
            major_radius: len(2.0),
            window: TubeWindow::Full,
            minor_radius: len(0.25),
        },
    );
    let Some(Node::Tube { frame, .. }) = doc.node(tube) else {
        panic!("the tube");
    };
    let frame = *frame;
    assert!(
        doc.var_name(frame).is_none(),
        "premise: the frame is unnamed"
    );

    let spun = pose(PoseDef::Spin {
        pose: RoleRead::plain(Operand::Var(frame)),
        about: RoleRead::through(Operand::Var(frame), PoseRole::Z),
        by: ang(0.5),
    });
    let applied = editor_core::apply(
        &doc,
        &DocEdit::SetParam {
            node: tube,
            slot: SlotId::Operand(OperandSlot::Frame),
            value: SlotValue::Read(spun),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a definition reading its pose twice is one reader");
    let doc = applied.doc;
    assert_eq!(
        doc.reader_counts().get(&frame).copied(),
        Some(1),
        "one definition, one reader"
    );
    assert!(doc.shared_unnamed_vars().is_empty());
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("saves");
    let back = editor_core::persist::load(&text, Tol::witness()).expect("the load door admits it");
    assert_eq!(back.snapshot.reader_counts().get(&frame).copied(), Some(1));
    let ev = eval(&doc);
    assert!(
        matches!(ev.nodes.get(&tube), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.nodes.get(&tube)
    );

    let refused = fixture::insert_refused(
        &doc,
        Node::Tube {
            frame: Operand::Var(frame),
            major_radius: len(2.0),
            window: TubeWindow::Full,
            minor_radius: len(0.25),
        },
    );
    assert!(
        matches!(&refused, EditError::SharedVarNeedsName { var } if var.id() == frame),
        "{refused:?}"
    );
}

/// **The tilted seat plane**: the box's top plane spun 15° about its
/// front rim evaluates, and splits the box along a plane through that
/// rim whose normal leans 15° from the top's.
#[test]
fn the_tilted_seat_plane_evaluates() {
    let (doc, cube) = unit_box();
    let top = fixture::fname(cube, editor_core::RoleSeg::Cap(CapEnd::End));
    let front = fixture::rim_edge(cube, CapEnd::End, fixture::piece(&doc, cube, 0, 0));
    let tilt = 15.0_f64.to_radians();
    let (doc, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: pose(PoseDef::Spin {
                pose: RoleRead::plain(pose(PoseDef::Plane {
                    face: Operand::select(Operand::output(cube, 0), vec![top]),
                })),
                about: RoleRead::plain(pose(PoseDef::Axis {
                    of: Operand::select(Operand::output(cube, 0), vec![front]),
                })),
                by: ang(tilt),
            })
            .into(),
        },
    );
    let ev = eval(&doc);
    assert!(
        matches!(ev.nodes.get(&split), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.nodes.get(&split)
    );
    let Some(Node::Split { tool, .. }) = doc.node(split) else {
        panic!("the split");
    };
    let Ok(PoseValue::Plane { origin, normal }) =
        bound_pose(&doc, &ev, OperandSlot::Tool, *tool, Tol::witness())
    else {
        panic!("a plane");
    };
    let n = UnitVec3::get(normal);
    near(n.z, tilt.cos(), "the normal leans 15°");
    // The front rim runs from (0, 0, 1) to (1, 0, 1): the spin turns the
    // plane about it, so both its ends stay on the plane.
    for end in [[0.0, 0.0, 1.0], [1.0, 0.0, 1.0]] {
        let p = geom_core::Point3::new(end[0], end[1], end[2]);
        near(
            (p - origin).dot(n),
            0.0,
            "the plane passes through the front rim",
        );
    }
}

/// **A lone `0` is the constant zero of its slot's dimension** (VR6):
/// a revolve's heading written `0`, an extrude's distance typed `0`
/// through the slot door, and a count written `0` each store a variable
/// defined by the constant, which lowers to no free variable; a written
/// `0 mm` mints a free length like any written quantity.
#[test]
fn a_lone_zero_is_the_constant_of_its_slots_dimension() {
    let (doc, cube) = unit_box();
    let Some(Node::Extrude { distance, .. }) = doc.node(cube) else {
        panic!("the box");
    };
    let written = *distance;
    assert!(
        !doc.lowers_to_constant(written),
        "a written length is a value"
    );
    let typed = |doc: &ProfileDoc, formula: Formula| {
        let doc = editor_core::apply(
            doc,
            &DocEdit::SetParam {
                node: cube,
                slot: SlotId::Distance,
                value: SlotValue::Formula(formula),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("a lone zero is written")
        .doc;
        let var = doc.slot(cube, SlotId::Distance).expect("the distance");
        (doc, var)
    };
    // `0` as the text door reads it, a count: the slot takes it at its
    // own dimension.
    for zero in [Formula::count(0), Formula::zero(Dimension::Length)] {
        let (doc, var) = typed(&doc, zero.clone());
        assert!(doc.lowers_to_constant(var), "{zero:?} is a constant");
        let Some(VarDef::Defined(expr)) = doc.var(var).map(|v| v.def()) else {
            panic!("{zero:?}: a defined variable");
        };
        assert!(expr.is_zero(), "{zero:?}: defined by the constant");
        assert_eq!(
            expr.dim(),
            Dimension::Length,
            "{zero:?}: at the slot's dimension"
        );
    }
    let (doc, var) = typed(&doc, Formula::length_in(0.0, quantity::MM).unwrap());
    assert!(doc.free(var).is_some(), "`0 mm` mints a free length");
    assert!(!doc.lowers_to_constant(var));

    // A revolve's heading written `0` is along its profile's x.
    let (doc, _plane, profile) = fixture::on_frame_keeping(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        vec![fixture::square(2.0, -0.5, 0.25)],
    );
    let (doc, revolve) = fixture::insert(
        doc,
        fixture::revolve_about(profile, (0.0, 0.0), fixture::along_x(), ang(1.0)),
    );
    let heading = doc.slot(revolve, SlotId::AxisHeading).expect("the heading");
    assert!(
        doc.lowers_to_constant(heading),
        "a heading of `0` is a constant"
    );
    let built = eval(&doc);
    assert!(
        matches!(built.nodes.get(&revolve), Some(NodeResult::Ok(_))),
        "the revolve about x builds: {:?}",
        built.nodes.get(&revolve)
    );
}
