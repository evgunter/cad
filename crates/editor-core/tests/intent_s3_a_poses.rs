//! **INTENT stage 3 PR A — a pose is a defined variable**
//! (`docs/INTENT-STAGE3-SPEC.md` §2 and §10 rows 1–4): a face reads as
//! a plane with its outward normal, a seat holds its kind, a
//! construction refuses its degenerate case, and a revolve's axis is a
//! line in its profile's plane.
//!
//! Pose values are bound at their readers and held nowhere, so a row
//! reads one through [`editor_core::eval::bound_pose`], as the reader
//! at the named seat would bind it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture::{self, ang, len, scl};
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::eval::bound_pose;
use editor_core::pose::{PoseConstruction, PoseDef, PoseReadFault};
use editor_core::{
    CancelToken, CapEnd, Dimension, Distribution, DocEdit, EditError, EvalOptions, Evaluation,
    ExtrudeSide, Formula, FreeVar, LoopProgram, Node, NodeErrorKind, NodeResult, Operand,
    OperandSlot, PoseValue, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, SlotKind, UnitSym,
    VarId, VarKind, VarName, all_faces, evaluate, face_carrier_kind,
};
use geom::SurfaceKind;
use geom_core::{Bounds, Decide, Decided, Interval, Sign, Sym, Tol, UnitVec3};

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A cylinder of radius 1 and height 2 standing on the world xy plane,
/// and its extrude node.
fn cylinder() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("s3a_cylinder", Tol::witness());
    let (doc, plane) = fixture::insert(
        doc,
        fixture::frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius: len(1.0),
            }],
            ids: Vec::new(),
        }),
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(2.0),
            side: ExtrudeSide::Along,
        },
    )
}

/// A unit box as an extruded square on the world xy plane.
fn unit_box() -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("s3a_box", Tol::witness()),
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

fn pose(def: PoseDef<Operand, Formula>) -> Operand {
    Operand::Pose(Box::new(def))
}

fn select(at: RecipeNodeId, name: editor_core::StableName) -> Operand {
    Operand::select(Operand::output(at, 0), vec![name])
}

/// `body` split by `tool`, inserted, and the pose variable the split's
/// tool reads.
fn split_by(doc: ProfileDoc, body: RecipeNodeId, tool: Operand) -> (ProfileDoc, VarId) {
    let (doc, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(body, 0),
            tool,
        },
    );
    let Some(Node::Split { tool, .. }) = doc.node(split) else {
        panic!("a split")
    };
    let tool = *tool;
    (doc, tool)
}

fn near(a: f64, b: f64, what: &str) {
    assert!((a - b).abs() <= 1e-12, "{what}: {a} vs {b}");
}

/// **Row 1 — a face reads as a plane, with its outward normal.** Both
/// caps of a cylinder: the top's normal is world +z, the bottom's −z,
/// whatever sense the carrier was minted with; the curved wall has no
/// plane and refuses `PoseRead` naming its cylinder.
#[test]
fn a_face_reads_as_its_plane_with_the_outward_normal() {
    for (end, z, nz) in [(CapEnd::End, 2.0, 1.0), (CapEnd::Start, 0.0, -1.0)] {
        let (doc, cyl) = cylinder();
        let cap = fixture::fname(cyl, RoleSeg::Cap(end));
        let (doc, plane) = split_by(
            doc,
            cyl,
            pose(PoseDef::Plane {
                face: select(cyl, cap),
            }),
        );
        let ev = eval(&doc);
        let value = bound_pose(&doc, &ev, OperandSlot::Tool, plane, Tol::witness());
        let Ok(PoseValue::Plane { origin, normal }) = value else {
            panic!("{end:?}: a plane, not {value:?}");
        };
        near(origin.z, z, "the plane passes through the cap");
        let n = UnitVec3::get(normal);
        near(n.x, 0.0, "no tilt");
        near(n.y, 0.0, "no tilt");
        near(n.z, nz, "the outward normal");
    }

    let (doc, cyl) = cylinder();
    let ev = eval(&doc);
    let wall = all_faces(&ev, cyl)
        .into_iter()
        .find(|n| face_carrier_kind(&ev, cyl, n) == Ok(SurfaceKind::Cylinder))
        .expect("a cylindrical wall");
    let (doc, plane) = split_by(
        doc,
        cyl,
        pose(PoseDef::Plane {
            face: select(cyl, wall),
        }),
    );
    let ev = eval(&doc);
    let refused = bound_pose(&doc, &ev, OperandSlot::Tool, plane, Tol::witness());
    assert!(
        matches!(
            refused,
            Err(NodeErrorKind::PoseRead {
                pose: VarKind::Plane,
                fault: PoseReadFault::NotPlanar {
                    carrier: SurfaceKind::Cylinder
                },
            })
        ),
        "{refused:?}"
    );
}

/// **Row 2 — a seat holds its kind.** A split's tool reads a plane: an
/// axis defined there refuses `SlotVarKind` at the door, expecting a
/// plane; a frame projected to its plane is admitted, and splits.
#[test]
fn a_split_tool_refuses_an_axis_and_admits_a_projected_frame() {
    let (doc, cube) = unit_box();
    let edge = fixture::rim_edge(cube, CapEnd::End, fixture::piece(&doc, cube, 0, 0));
    let refused = fixture::insert_refused(
        &doc,
        Node::Split {
            target: Operand::output(cube, 0),
            tool: pose(PoseDef::Axis {
                of: Operand::select(Operand::output(cube, 0), vec![edge]),
            }),
        },
    );
    assert!(
        matches!(
            &refused,
            EditError::SlotVarKind {
                found: VarKind::Axis,
                expected: SlotKind::Is(VarKind::Plane),
                ..
            }
        ),
        "{refused:?}"
    );

    let (doc, frame) = fixture::insert(
        doc,
        fixture::frame([0.0, 0.0, 0.5], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(cube, 0),
            tool: pose(PoseDef::Project {
                of: Operand::output(frame, 0),
                to: VarKind::Plane,
            }),
        },
    );
    let ev = eval(&doc);
    assert!(
        matches!(ev.nodes.get(&split), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.nodes.get(&split)
    );
}

/// **Row 3 — a construction refuses its degenerate case.** The frame
/// through an edge's line and a point picks its `x` towards the point;
/// a point ON the line leaves no such direction, and the construction
/// refuses `PoseDegenerate` rather than choose a roll. Off the line, the
/// frame stands at the point's foot.
#[test]
fn a_frame_through_an_axis_and_a_point_on_it_refuses() {
    // The top rim over the square's canonical edge 0 runs from
    // canonical vertex 0 to vertex 1; vertex 2 is the corner off it.
    let through = |vertex: usize| {
        let (doc, cube) = unit_box();
        let edge = fixture::rim_edge(cube, CapEnd::End, fixture::piece(&doc, cube, 0, 0));
        let point = fixture::cap_vertex(cube, CapEnd::End, fixture::vpiece(&doc, cube, 0, vertex));
        let (doc, plane) = split_by(
            doc,
            cube,
            pose(PoseDef::Project {
                of: pose(PoseDef::Through {
                    axis: pose(PoseDef::Axis {
                        of: select(cube, edge),
                    }),
                    point: pose(PoseDef::Point {
                        of: select(cube, point),
                    }),
                }),
                to: VarKind::Plane,
            }),
        );
        let ev = eval(&doc);
        bound_pose(&doc, &ev, OperandSlot::Tool, plane, Tol::witness())
    };

    for on in [0, 1] {
        let refused = through(on);
        assert!(
            matches!(
                refused,
                Err(NodeErrorKind::PoseDegenerate {
                    construction: PoseConstruction::Through
                })
            ),
            "vertex {on}: {refused:?}"
        );
    }

    // Off the line: the frame's plane passes through the point's foot
    // on the edge — an end of it, a corner of the box — normal to the
    // edge, which runs along a world axis.
    let Ok(PoseValue::Plane { origin, normal }) = through(2) else {
        panic!("a plane off the line");
    };
    for c in [origin.x, origin.y, origin.z] {
        assert!(c.abs() <= 1e-12 || (c - 1.0).abs() <= 1e-12, "{origin:?}");
    }
    near(origin.z, 1.0, "the foot is on the top rim");
    let n = UnitVec3::get(normal);
    let mut along = [n.x.abs(), n.y.abs(), n.z.abs()];
    along.sort_by(f64::total_cmp);
    near(along[2], 1.0, "normal along the edge");
    near(n.z, 0.0, "the top rim is horizontal");
}

/// The revolve row's document: a washer section on the world xz plane,
/// revolved a full turn about the line through the frame's origin along
/// `(t, 1)`, `t` a free parameter of width `width`. (On a frame tilted
/// out of the world planes the revolve itself does not certify at
/// `Interval` even at ε/128: issue 1191's class, not the axis's.)
fn tilted_revolve(width: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut doc = ProfileDoc::empty_derived("s3a_revolve", Tol::witness());
    doc = editor_core::apply(
        &doc,
        &DocEdit::DeclareVar {
            name: VarName::from_static("t"),
            def: editor_core::VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Scalar,
                value: 0.0,
                display_unit: UnitSym::canonical_for(Dimension::Scalar),
                distribution: Some(Distribution::Uniform {
                    lo: -width,
                    hi: width,
                }),
            }),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("declares")
    .doc;
    let (doc, plane, profile) = fixture::on_frame_keeping(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        vec![fixture::square(2.0, 0.0, 0.5)],
    );
    let (doc, revolve) = fixture::insert(
        doc,
        Node::Revolve {
            profile: profile.into(),
            axis_origin: [len(0.0), len(0.0)],
            axis_direction: [
                Formula::named(VarName::from_static("t"), Dimension::Scalar),
                scl(1.0),
            ],
            angle: ang(std::f64::consts::TAU),
        },
    );
    (doc, plane, revolve)
}

/// **Row 4 — a revolve's axis cannot leave its profile's plane.** Its
/// axis output is its 2-D line lifted through the plane's axes, so the
/// direction's margin against the plane's normal is identically zero —
/// decided `Zero` at the symbolic scalar for every value of the box,
/// and enclosing zero at `Interval`.
#[test]
fn a_revolves_axis_lies_in_its_profiles_plane_over_a_box() {
    let width = Tol::witness().eps() / 64.0;
    let (doc, plane, revolve) = tilted_revolve(width);
    let axis = doc.output(revolve, 1).expect("a revolve defines its axis");
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed_box(
            &doc,
            &AnalysisPolicy::default(),
        )))),
        ..EvalOptions::default()
    };
    let frame = doc.output(plane, 0).expect("a frame defines its pose");

    fn margin<T: editor_core::eval::EvalScalar>(
        doc: &ProfileDoc,
        ev: &Evaluation<T>,
        frame: VarId,
        axis: VarId,
    ) -> T {
        let Ok(PoseValue::Frame(f)) =
            bound_pose(doc, ev, OperandSlot::Frame, frame, Tol::witness())
        else {
            panic!("the profile's frame");
        };
        let lifted = bound_pose(doc, ev, OperandSlot::Axis, axis, Tol::witness());
        let Ok(PoseValue::Axis { dir, .. }) = lifted else {
            panic!("the revolve's axis: {lifted:?}");
        };
        let n = UnitVec3::get(f.u()).cross(UnitVec3::get(f.v()));
        n.dot(UnitVec3::get(dir))
    }

    let ev = evaluate::<Interval>(&doc, None, &CancelToken::new(), &opts, Tol::witness());
    let m = margin(&doc, &ev, frame, axis);
    assert!(m.lo() <= 0.0 && 0.0 <= m.hi(), "{m:?}");

    let budget = geom_core::SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let (decided, _) = geom_core::sym::with_session(budget, || {
        let ev = evaluate::<Sym<Interval>>(&doc, None, &CancelToken::new(), &opts, Tol::witness());
        margin(&doc, &ev, frame, axis).sign_within(fixture::band())
    });
    assert!(
        matches!(
            decided,
            Ok(Decided {
                sign: Sign::Zero,
                ..
            })
        ),
        "{decided:?}"
    );
}
