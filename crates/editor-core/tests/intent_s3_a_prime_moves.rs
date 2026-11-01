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
    CancelToken, CapEnd, Dimension, DocEdit, EditError, EvalOptions, Evaluation, Formula, Node,
    NodeResult, Operand, OperandSlot, PoseValue, ProfileDoc, RecipeNodeId, SlotId, SlotValue,
    VarDef, VarId, VarName, evaluate,
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

/// A unit box as an extruded square on its own xy plane.
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
        },
    )
}

/// A frame read off `cube` at `(1, 2, 3)` in its own coordinates, whose
/// x is the cube's +y and y its +z, so its z is the cube's +x: no axis
/// of it is the cube's own. A split by its xy plane reads it, and it is
/// named, so further seats share it.
fn turned_frame(doc: ProfileDoc, cube: RecipeNodeId) -> (ProfileDoc, VarId) {
    let at = fixture::axes_at([1.0, 2.0, 3.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
    let frame =
        editor_core::test_support::frame_on(&doc, &Operand::from(cube), &at, Tol::witness());
    let (doc, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: RoleRead::through(frame, PoseRole::Xy),
        },
    );
    let Some(Node::Split { tool, .. }) = doc.node(split) else {
        panic!("the split");
    };
    let var = tool.read;
    let (doc, _) = fixture::step(
        doc,
        DocEdit::RenameVar {
            var: var.into(),
            name: Some(VarName::from_static("turned")),
        },
    );
    (doc, var)
}

/// **A frame's x and y roles read** (D10: the role is part of the
/// read): through x its axis along `u`, through y along `v`, through z
/// along its normal, each through its origin; through its yz and zx
/// planes the planes normal to `u` and `v`; and the point at `(x, y, z)`
/// in it is its origin slid along its axes. A ring about the frame's y,
/// a split by its zx plane and a split by the frame through its z axis
/// and a point in it read the frame through those roles at their seats,
/// and build.
#[test]
fn a_frames_axes_and_planes_read_through_their_roles() {
    let (doc, cube) = unit_box();
    let (doc, frame) = turned_frame(doc, cube);
    let ev = eval(&doc);
    let through = |role| RoleRead::through(frame, role);
    let axis = |role| match bound_pose(&doc, &ev, OperandSlot::Axis, through(role), Tol::witness())
    {
        Ok(PoseValue::Axis(line)) => (line.foot(), UnitVec3::get(line.dir())),
        other => panic!("{role}: an axis, not {other:?}"),
    };
    // Each axis through the frame's origin (1, 2, 3), its foot that
    // origin with its component along the axis taken off.
    for (role, along, foot) in [
        (PoseRole::X, [0.0, 1.0, 0.0], [1.0, 0.0, 3.0]),
        (PoseRole::Y, [0.0, 0.0, 1.0], [1.0, 2.0, 0.0]),
        (PoseRole::Z, [1.0, 0.0, 0.0], [0.0, 2.0, 3.0]),
    ] {
        let (origin, dir) = axis(role);
        for (got, want, what) in [
            (origin.x, foot[0], "foot x"),
            (origin.y, foot[1], "foot y"),
            (origin.z, foot[2], "foot z"),
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
    // The frame's y runs along the world's z.
    let (doc, _, ring) = fixture::circular(
        doc,
        cube,
        (
            RoleRead::through(Operand::Var(frame), PoseRole::Y),
            editor_core::TargetAxis::Z,
        ),
        std::f64::consts::FRAC_PI_2,
        2,
    );
    let (doc, through) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: RoleRead::through(
                pose(PoseDef::Through {
                    axis: RoleRead::through(Operand::Var(frame), PoseRole::Z),
                    point: RoleRead::plain(pose(PoseDef::point_in(
                        &Operand::Var(frame),
                        [len(0.5), len(0.25), len(-1.0)],
                    ))),
                }),
                PoseRole::Xy,
            ),
        },
    );
    let ev = eval(&doc);
    for (node, what) in [
        (split, "the split"),
        (ring, "the ring"),
        (through, "the split by the frame through"),
    ] {
        assert!(
            matches!(ev.nodes.get(&node), Some(NodeResult::Ok(_))),
            "{what}: {:?}",
            ev.nodes.get(&node)
        );
    }
    let Some(Node::Split { tool: at, .. }) = doc.node(through) else {
        panic!("the split by the frame through");
    };
    let Some(VarDef::Pose(PoseDef::Through { point, .. })) = doc.var(at.read).map(|v| v.def())
    else {
        panic!("a frame through an axis and a point");
    };
    let Ok(PoseValue::Point { position }) =
        bound_pose(&doc, &ev, OperandSlot::Tool, point.read, Tol::witness())
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
    let (doc, frame) = turned_frame(doc, cube);
    let refused = fixture::insert_refused(
        &doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: RoleRead::through(Operand::Var(frame), PoseRole::X),
        },
    );
    assert!(
        matches!(
            &refused,
            EditError::SlotVarKind {
                found: editor_core::VarKind::Axis,
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
/// the load doors): a split's tool, an unnamed plane its seat alone
/// reads, re-pointed to that plane slid along its own normal, is read
/// by one definition twice — once plainly and once through its role —
/// and that is one reader. A second seat reading it is a second reader,
/// and refuses.
#[test]
fn a_self_read_through_a_role_is_one_reader() {
    let (doc, cube) = unit_box();
    let top = fixture::fname(cube, editor_core::RoleSeg::Cap(CapEnd::End));
    let (doc, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: pose(PoseDef::Plane {
                face: Operand::select(Operand::output(cube, 0), vec![top]),
            })
            .into(),
        },
    );
    let Some(Node::Split { tool, .. }) = doc.node(split) else {
        panic!("the split");
    };
    let plane = tool.read;
    assert!(
        doc.var_name(plane).is_none(),
        "premise: the plane is unnamed"
    );

    let slid = pose(PoseDef::Slide {
        pose: RoleRead::plain(Operand::Var(plane)),
        along: RoleRead::through(Operand::Var(plane), PoseRole::Normal),
        by: len(-0.5),
    });
    let applied = editor_core::apply(
        &doc,
        &DocEdit::SetParam {
            node: split,
            slot: SlotId::Operand(OperandSlot::Tool),
            value: SlotValue::Read(slid),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("a definition reading its pose twice is one reader");
    let doc = applied.doc;
    assert_eq!(
        doc.reader_counts().get(&plane).copied(),
        Some(1),
        "one definition, one reader"
    );
    assert!(doc.shared_unnamed_vars().is_empty());
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("saves");
    let back = editor_core::persist::load(&text, Tol::witness()).expect("the load door admits it");
    assert_eq!(back.snapshot.reader_counts().get(&plane).copied(), Some(1));
    let ev = eval(&doc);
    assert!(
        matches!(ev.nodes.get(&split), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.nodes.get(&split)
    );

    let refused = fixture::insert_refused(
        &doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: Operand::Var(plane).into(),
        },
    );
    assert!(
        matches!(&refused, EditError::SharedVarNeedsName { var } if var.id() == plane),
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
    let Ok(PoseValue::Plane { normal, offset }) =
        bound_pose(&doc, &ev, OperandSlot::Tool, *tool, Tol::witness())
    else {
        panic!("a plane");
    };
    let n = UnitVec3::get(normal);
    near(n.z, tilt.cos(), "the normal leans 15°");
    // Right-handed about the rim's own direction `d`: the top's normal
    // `z` turns towards `d × z`.
    let Some(VarDef::Pose(PoseDef::Spin { about, .. })) = doc.var(tool.read).map(|v| v.def())
    else {
        panic!("a spun plane");
    };
    let Ok(PoseValue::Axis(axis)) =
        bound_pose(&doc, &ev, OperandSlot::Axis, about.read, Tol::witness())
    else {
        panic!("the rim's axis");
    };
    let d = UnitVec3::get(axis.dir());
    let toward = d.cross(geom_core::Vec3::new(0.0, 0.0, 1.0));
    near(
        n.dot(toward),
        tilt.sin(),
        "the normal leans by the right-hand rule",
    );
    // The front rim runs from (0, 0, 1) to (1, 0, 1): the spin turns the
    // plane about it, so both its ends stay on the plane.
    for end in [[0.0, 0.0, 1.0], [1.0, 0.0, 1.0]] {
        let p = geom_core::Point3::new(end[0], end[1], end[2]);
        near(
            (p - geom_core::Point3::origin()).dot(n) - offset,
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
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(vec![fixture::square(2.0, -0.5, 0.25)])),
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
    let axis = doc.output(revolve, 1).expect("a revolve defines its axis");
    let Ok(PoseValue::Axis(axis)) =
        bound_pose(&doc, &built, OperandSlot::Axis, axis, Tol::witness())
    else {
        panic!("the revolve's axis");
    };
    let d = UnitVec3::get(axis.dir());
    for (got, want, what) in [(d.x, 1.0, "x"), (d.y, 0.0, "y"), (d.z, 0.0, "z")] {
        near(
            got,
            want,
            &format!("the axis runs along the profile's x: {what}"),
        );
    }
}

/// **A lone `0` is the constant at every door** (VR6): written at a
/// slide's length inside a pose definition, a count `0` is the constant
/// of the slot's dimension.
#[test]
fn a_lone_zero_is_the_constant_at_the_insert_door() {
    let (doc, cube) = unit_box();
    let (doc, frame) = turned_frame(doc, cube);
    let (doc, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: pose(PoseDef::Slide {
                pose: RoleRead::through(Operand::Var(frame), PoseRole::Zx),
                along: RoleRead::through(Operand::Var(frame), PoseRole::Y),
                by: Formula::count(0),
            })
            .into(),
        },
    );
    let Some(Node::Split { tool, .. }) = doc.node(split) else {
        panic!("the split");
    };
    let Some(VarDef::Pose(PoseDef::Slide { by, .. })) = doc.var(tool.read).map(|v| v.def()) else {
        panic!("a slid plane");
    };
    assert!(doc.lowers_to_constant(*by), "the slide's `0` is a constant");
}

/// **A dimensioned zero reads back as it was built**: `w + 0` keeps the
/// length's zero beside `w`, and a product or a negation of a length's
/// zero is that zero, so the text of each reads back as the tree.
#[test]
fn a_dimensioned_zero_reads_back_through_the_text_door() {
    let w = editor_core::VarName::from_static("w");
    let params = std::collections::BTreeMap::from([(w.clone(), Dimension::Length)]);
    let parsed = editor_core::parse::parse_formula("w + 0", &params).expect("parses");
    assert_eq!(parsed.dim(), Dimension::Length);
    let k = editor_core::VarName::from_static("k");
    let zero = Formula::zero(Dimension::Length);
    for built in [
        Formula::add(Formula::named(w.clone(), Dimension::Length), zero.clone()).unwrap(),
        Formula::mul(zero.clone(), Formula::named(k.clone(), Dimension::Scalar)).unwrap(),
        Formula::div(zero.clone(), Formula::named(k.clone(), Dimension::Scalar)).unwrap(),
        Formula::neg(zero.clone()).unwrap(),
    ] {
        let params = std::collections::BTreeMap::from([
            (w.clone(), Dimension::Length),
            (k.clone(), Dimension::Scalar),
        ]);
        let text = editor_core::unparse(&built, &|_| None::<&editor_core::VarName>);
        let back = editor_core::parse::parse_formula(&text, &params).expect("reads back");
        if built.is_zero() {
            // A lone zero reads back as the text door's `0`, which a
            // slot takes at its own dimension.
            assert!(back.is_zero(), "{text}");
        } else {
            assert_eq!(back, built, "{text}");
        }
    }
}

/// **A flip of a flip read through a role refuses**: it is the pose
/// read through that role, a read the door does not store a second
/// way.
#[test]
fn a_flip_of_a_flip_through_a_role_refuses() {
    let (doc, cube) = unit_box();
    let (doc, frame) = turned_frame(doc, cube);
    let refused = fixture::insert_refused(
        &doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: pose(PoseDef::Flip {
                pose: RoleRead::plain(pose(PoseDef::Flip {
                    pose: RoleRead::through(Operand::Var(frame), PoseRole::Xy),
                })),
            })
            .into(),
        },
    );
    assert!(
        matches!(
            &refused,
            EditError::PoseShape {
                fault: editor_core::pose::PoseFault::DoubleFlip,
                ..
            }
        ),
        "{refused:?}"
    );
}

/// **A role is keyed** (D9): a split by the turned frame's zx plane
/// slid along its y to the cube's mid-height, re-pointed to its yz
/// plane slid along its x to the cube's mid-depth, keys apart, and a memo of the first does not serve the
/// second.
#[test]
fn a_reads_role_is_in_the_content_key() {
    let (doc, cube) = unit_box();
    let (doc, frame) = turned_frame(doc, cube);
    let tool = |plane, along, by| {
        pose(PoseDef::Slide {
            pose: RoleRead::through(Operand::Var(frame), plane),
            along: RoleRead::through(Operand::Var(frame), along),
            by: len(by),
        })
    };
    let (first, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: tool(PoseRole::Zx, PoseRole::Y, -2.5).into(),
        },
    );
    let ev_first = eval(&first);
    let second = editor_core::apply(
        &first,
        &DocEdit::SetParam {
            node: split,
            slot: SlotId::Operand(OperandSlot::Tool),
            value: SlotValue::Read(tool(PoseRole::Yz, PoseRole::X, -1.5)),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("re-tools")
    .doc;
    let fresh = eval(&second);
    let memo = evaluate::<f64>(
        &second,
        Some(&ev_first),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let ok = |ev: &Evaluation<f64>| match ev.nodes.get(&split) {
        Some(NodeResult::Ok(v)) => (v.content_key, format!("{:?}", v.payload)),
        other => panic!("the split builds: {other:?}"),
    };
    let ((key_first, body_first), (key_second, body_second)) = (ok(&ev_first), ok(&fresh));
    assert_ne!(body_first, body_second, "premise: the two tools cut apart");
    assert_ne!(key_first, key_second, "two roles, one content key");
    assert_eq!(ok(&memo).1, body_second, "the memo served the other role");
}

/// **A re-point replaces a seat's read whole, its role with it**: a
/// split's tool read as a frame's zx plane, re-pointed to a cap's plane
/// slid along its normal, reads that plane plainly and builds;
/// re-pointed to the frame itself, it refuses, a frame being no plane.
#[test]
fn a_repoint_replaces_the_read_and_its_role() {
    let (doc, cube) = unit_box();
    let (doc, frame) = turned_frame(doc, cube);
    let (doc, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: RoleRead::through(Operand::Var(frame), PoseRole::Zx),
        },
    );
    let top = || {
        pose(PoseDef::Plane {
            face: Operand::select(
                Operand::output(cube, 0),
                vec![fixture::fname(cube, editor_core::RoleSeg::Cap(CapEnd::End))],
            ),
        })
    };
    let plane = pose(PoseDef::Slide {
        pose: RoleRead::plain(top()),
        along: RoleRead::through(top(), PoseRole::Normal),
        by: len(-0.5),
    });
    let repoint = |doc: &ProfileDoc, read: Operand| {
        editor_core::apply(
            doc,
            &DocEdit::SetParam {
                node: split,
                slot: SlotId::Operand(OperandSlot::Tool),
                value: SlotValue::Read(read),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
    };
    let moved = repoint(&doc, plane)
        .expect("a plane re-points the tool")
        .doc;
    let Some(Node::Split { tool, .. }) = moved.node(split) else {
        panic!("the split");
    };
    assert_eq!(tool.role, None, "the role went with the read it qualified");
    assert!(
        matches!(eval(&moved).nodes.get(&split), Some(NodeResult::Ok(_))),
        "the split by the plane builds"
    );
    let refused = repoint(&doc, Operand::Var(frame)).expect_err("a frame is no plane");
    assert!(
        matches!(
            refused,
            EditError::SlotVarKind {
                found: editor_core::VarKind::Frame,
                ..
            }
        ),
        "{refused:?}"
    );
}

/// **A read's two wire forms are told apart in one forward pass**
/// (`persist::refusal`'s premise; the `persist-no-backtracking` gate): a
/// plain read is written bare, and a read through its role as
/// `{"read": …, "role": …}`, told apart by the map's first key, so an
/// operand read (itself a map) still reads back bare. A role read that
/// lacks its role, names a role no pose offers, or carries a third field
/// refuses naming that field, never "no variant matched".
#[test]
fn a_reads_wire_forms_read_back_and_refuse_by_field() {
    let (doc, cube) = unit_box();
    let (_, frame) = turned_frame(doc, cube);
    let plain: RoleRead<Operand> = RoleRead::plain(Operand::Var(frame));
    let through: RoleRead<Operand> = RoleRead::through(Operand::Var(frame), PoseRole::Zx);
    for read in [&plain, &through] {
        let text = serde_json::to_string(read).expect("a read serializes");
        let back: RoleRead<Operand> = serde_json::from_str(&text).expect("and reads back");
        assert_eq!(&back, read, "{text}");
    }
    let bare = serde_json::to_value(&plain).expect("a plain read serializes");
    assert_eq!(
        bare,
        serde_json::to_value(Operand::Var(frame)).expect("an operand serializes")
    );
    let var = serde_json::to_value(Operand::Var(frame)).expect("an operand serializes");
    for (text, names) in [
        (serde_json::json!({ "read": var }), "role"),
        (serde_json::json!({ "read": var, "role": "line" }), "line"),
        (
            serde_json::json!({ "read": var, "role": "zx", "at": [] }),
            "at",
        ),
    ] {
        let err = serde_json::from_value::<RoleRead<Operand>>(text.clone())
            .expect_err("a malformed role read refuses")
            .to_string();
        assert!(err.contains(names), "{text}: {err}");
    }
}
