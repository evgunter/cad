//! **INTENT stage 3 PR A — a pose is a defined variable**
//! (`docs/INTENT-STAGE3-SPEC.md` §2 and §10 rows 1–4): a face reads as
//! a plane with its outward normal, a seat holds its kind, a
//! construction refuses its degenerate case, and a revolve's axis is a
//! line in its profile's plane.
//!
//! Pose values are bound at their readers and held nowhere, so a row
//! reads one through [`editor_core::eval::bound_pose`], as the reader
//! at the named seat would bind it.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::result_large_err
)]

use std::sync::Arc;

use crate::fixture::{self, ang, len, scl};
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::eval::bound_pose;
use editor_core::pose::{PoseConstruction, PoseDef, PoseReadFault, PoseRole, RoleRead};
use editor_core::{
    CancelToken, CapEnd, Dimension, Distribution, DocEdit, EditError, EvalOptions, Evaluation,
    Formula, FreeVar, LoopProgram, Node, NodeErrorKind, NodeResult, Operand, OperandSlot,
    PoseValue, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, SlotKind, UnitSym, VarId,
    VarKind, VarName, all_faces, evaluate, face_carrier_kind,
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

/// A cylinder of radius 1 and height 2 standing on its own xy plane,
/// and its extrude node.
fn cylinder() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("s3a_cylinder", Tol::witness());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius: len(1.0),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(2.0),
        },
    )
}

/// A unit box as an extruded square on its own xy plane.
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
        },
    )
}

fn pose(def: PoseDef<Operand, Formula>) -> Operand {
    Operand::Pose(Box::new(def))
}

fn select(at: RecipeNodeId, name: editor_core::StableName) -> Operand {
    Operand::select(Operand::output(at, 0), vec![name])
}

/// `body` split by `tool`, inserted, and the read the split's tool
/// stores.
fn split_by(
    doc: ProfileDoc,
    body: RecipeNodeId,
    tool: impl Into<RoleRead<Operand>>,
) -> (ProfileDoc, RoleRead<VarId>) {
    let (doc, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(body, 0).into(),
            tool: tool.into(),
        },
    );
    let Some(Node::Split { tool, .. }) = doc.node(split) else {
        panic!("a split")
    };
    let tool = *tool;
    (doc, tool)
}

/// **The frame at `at` in `body`'s own coordinates**, read off its
/// geometry ([`editor_core::test_support::frame_on`]): a pose a definition reads
/// through a role.
fn frame_on(doc: &ProfileDoc, body: RecipeNodeId, at: &editor_core::Frame) -> Operand {
    editor_core::test_support::frame_on(doc, &Operand::from(body), at, Tol::witness())
}

/// `operand` read plainly, where a definition reads a pose.
fn plain(operand: Operand) -> RoleRead<Operand> {
    RoleRead::plain(operand)
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
        let Ok(PoseValue::Plane { normal, offset }) = value else {
            panic!("{end:?}: a plane, not {value:?}");
        };
        near(offset, z * nz, "the plane passes through the cap");
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
/// plane, and so does a frame read through its x axis; a frame read
/// through its xy plane is admitted, and splits.
#[test]
fn a_split_tool_refuses_an_axis_and_admits_a_frames_plane() {
    let (doc, cube) = unit_box();
    let edge = fixture::rim_edge(cube, CapEnd::End, fixture::piece(&doc, cube, 0, 0));
    let refused = fixture::insert_refused(
        &doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: pose(PoseDef::Axis {
                of: Operand::select(Operand::output(cube, 0), vec![edge]),
            })
            .into(),
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

    let frame = frame_on(
        &doc,
        cube,
        &editor_core::Frame::translation([0.0, 0.0, 0.5]),
    );
    let refused = fixture::insert_refused(
        &doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: RoleRead::through(frame.clone(), PoseRole::X),
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
    let (doc, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: RoleRead::through(frame, PoseRole::Xy),
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
            RoleRead::through(
                pose(PoseDef::Through {
                    axis: plain(pose(PoseDef::Axis {
                        of: select(cube, edge),
                    })),
                    point: plain(pose(PoseDef::Point {
                        of: select(cube, point),
                    })),
                }),
                PoseRole::Xy,
            ),
        );
        let ev = eval(&doc);
        let frame = bound_pose(
            &doc,
            &ev,
            OperandSlot::Frame,
            RoleRead::plain(plane.read),
            Tol::witness(),
        );
        (
            bound_pose(&doc, &ev, OperandSlot::Tool, plane, Tol::witness()),
            frame,
        )
    };

    for on in [0, 1] {
        let (refused, _) = through(on);
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
    let (Ok(plane @ PoseValue::Plane { normal, .. }), Ok(PoseValue::Frame(frame))) = through(2)
    else {
        panic!("a plane off the line, in its frame");
    };
    let corner = frame.origin();
    for c in [corner.x, corner.y, corner.z] {
        assert!(
            c.abs() <= 1e-12 || (c - 1.0).abs() <= 1e-12,
            "the foot is a corner: {corner:?}"
        );
    }
    near(corner.z, 1.0, "the foot is on the top rim");
    let on = editor_core::pose::pose_distance(&plane, corner).expect("a plane has a position");
    near(on, 0.0, "the plane passes through the foot");
    let n = UnitVec3::get(normal);
    let mut along = [n.x.abs(), n.y.abs(), n.z.abs()];
    along.sort_by(f64::total_cmp);
    near(along[2], 1.0, "normal along the edge");
    near(n.z, 0.0, "the top rim is horizontal");
}

/// The revolve row's document: a washer section on its own xy plane,
/// revolved a full turn about the line through its origin at the
/// heading `h`, a free angle of a quarter turn and width `width`.
fn headed_revolve(width: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut doc = ProfileDoc::empty_derived("s3a_revolve", Tol::witness());
    doc = editor_core::apply(
        &doc,
        &DocEdit::DeclareVar {
            name: VarName::from_static("h"),
            def: editor_core::VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Angle,
                value: std::f64::consts::FRAC_PI_2,
                display_unit: UnitSym::canonical_for(Dimension::Angle),
                distribution: Some(Distribution::Uniform {
                    lo: -width,
                    hi: width,
                }),
            }),
        },
        Tol::witness(),
    )
    .expect("declares")
    .doc;
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(vec![fixture::square(2.0, 0.0, 0.5)])),
    );
    let (doc, revolve) = fixture::insert(
        doc,
        Node::Revolve {
            profile: profile.into(),
            axis_through: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            axis_heading: Formula::named(VarName::from_static("h"), Dimension::Angle),
            angle: ang(std::f64::consts::TAU),
        },
    );
    (doc, revolve)
}

/// **Row 4 — a revolve's axis cannot leave its profile's plane.** Its
/// axis output is its 2-D line in the profile's own xy, so the
/// direction's margin against the profile's normal (its own z) is
/// identically zero — decided `Zero` at the symbolic scalar for every
/// value of the box, and enclosing zero at `Interval`.
#[test]
fn a_revolves_axis_lies_in_its_profiles_plane_over_a_box() {
    let width = Tol::witness().eps() / 64.0;
    let (doc, revolve) = headed_revolve(width);
    let axis = doc.output(revolve, 1).expect("a revolve defines its axis");
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed_box(
            &doc,
            &AnalysisPolicy::default(),
        )))),
        ..EvalOptions::default()
    };

    fn margin<T: editor_core::eval::EvalScalar>(
        doc: &ProfileDoc,
        ev: &Evaluation<T>,
        axis: VarId,
    ) -> T {
        let lifted = bound_pose(doc, ev, OperandSlot::Axis, axis, Tol::witness());
        let Ok(PoseValue::Axis(axis)) = lifted else {
            panic!("the revolve's axis: {lifted:?}");
        };
        UnitVec3::get(axis.dir()).z
    }

    let ev = evaluate::<Interval>(&doc, None, &CancelToken::new(), &opts, Tol::witness());
    let m = margin(&doc, &ev, axis);
    assert!(m.lo() <= 0.0 && 0.0 <= m.hi(), "{m:?}");

    let budget = geom_core::SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let (decided, _) = geom_core::sym::with_session(budget, || {
        let ev = evaluate::<Sym<Interval>>(&doc, None, &CancelToken::new(), &opts, Tol::witness());
        margin(&doc, &ev, axis).sign_within(fixture::band())
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

/// **A standoff**: `plane` slid along its own normal by `by` (FORK-MOVE:
/// the one number a plane's own equation fixes, as a slide).
fn standoff(plane: Operand, by: Formula) -> PoseDef<Operand, Formula> {
    PoseDef::Slide {
        pose: plain(plane.clone()),
        along: RoleRead::through(plane, PoseRole::Normal),
        by,
    }
}

/// **A construction reads what was written, through a pose too** (D10):
/// a split's tool defined as a cap's plane moved by a measured length
/// reads the measure, and the door refuses it as it refuses a slot
/// reading one — naming the split's tool.
#[test]
fn a_pose_moved_by_a_measured_length_is_observed() {
    let (doc, cube) = unit_box();
    let (doc, measure) = fixture::measure_node(
        &doc,
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            fixture::cap_ref(cube, CapEnd::Start),
            fixture::cap_ref(cube, CapEnd::End),
        ],
    );
    let out = fixture::output(&doc, measure);
    let cap = fixture::fname(cube, RoleSeg::Cap(CapEnd::End));
    let refused = fixture::insert_refused(
        &doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: pose(standoff(
                pose(PoseDef::Plane {
                    face: select(cube, cap),
                }),
                Formula::mul(Formula::var(out, Dimension::Length), scl(-0.5)).unwrap(),
            ))
            .into(),
        },
    );
    assert!(
        matches!(
            &refused,
            EditError::ConstructionReadsObserved {
                slot: editor_core::SlotId::Operand(OperandSlot::Tool),
                ..
            }
        ),
        "{refused:?}"
    );
}

/// A ring of three boxes about the line two planes meet in: `a` the
/// block's own yz plane spun `a_tilt` about its y axis and slid to
/// `a_origin`, `b` its own plane `y = 0`, both read off the block's
/// frame at its own origin. The ring's mate is the `Meet`'s reader.
fn ring_about_a_meet(a_origin: [f64; 3], a_tilt: f64) -> String {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("s3a_meet", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(2.0, 0.5), (3.0, 0.5), (3.0, 1.5), (2.0, 1.5)]],
    );
    let (doc, block) = fixture::insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        },
    );
    let own = frame_on(&doc, block, &editor_core::Frame::IDENTITY);
    let axis = |role| RoleRead::through(own.clone(), role);
    let spun = pose(PoseDef::Spin {
        pose: axis(PoseRole::Yz),
        about: axis(PoseRole::Y),
        by: ang(a_tilt),
    });
    let a = [PoseRole::X, PoseRole::Y, PoseRole::Z]
        .into_iter()
        .zip(a_origin)
        .fold(spun, |at, (along, by)| {
            pose(PoseDef::Slide {
                pose: plain(at),
                along: axis(along),
                by: len(by),
            })
        });
    let meet = RoleRead::plain(pose(PoseDef::Meet {
        a: plain(a),
        b: axis(PoseRole::Zx),
    }));
    let (doc, _, ring) = fixture::circular(
        doc,
        block,
        (meet, editor_core::TargetAxis::Z),
        std::f64::consts::FRAC_PI_2,
        3,
    );
    let ev = eval(&doc);
    format!("{:?}", ev.value(ring).expect("the ring builds").payload)
}

/// The spin that turns the world's x to `(0.6, 0, 0.8)` about its y:
/// `atan2(-0.8, 0.6)`, computed once.
const TILT: f64 = -0.927_295_218_001_612_3;

/// The numbers in a value's printed form, and the text between them:
/// two values have one outcome — one topology, one set of decisions —
/// when the text agrees exactly and only the numbers may move.
fn numbers_and_shape(printed: &str) -> (Vec<f64>, String) {
    let mut numbers = Vec::new();
    let mut shape = String::new();
    let mut rest = printed;
    while let Some(at) = rest.find(|c: char| c.is_ascii_digit() || c == '-') {
        let tail = &rest[at..];
        let len = tail
            .find(|c: char| !(c.is_ascii_digit() || matches!(c, '.' | '-' | 'e' | '+')))
            .unwrap_or(tail.len());
        match tail[..len].parse::<f64>() {
            Ok(x) if tail[..len].contains('.') => {
                shape.push_str(&rest[..at]);
                shape.push('#');
                numbers.push(x);
            }
            _ => shape.push_str(&rest[..at + len.max(1)]),
        }
        rest = &tail[len.max(1)..];
    }
    shape.push_str(rest);
    (numbers, shape)
}

/// **A `Meet`'s representative is a function of its definition, and no
/// reader's outcome depends on it** (the orchestrator's ruling (a)):
/// its point is "nearest `a`'s origin", and `a`'s origin is itself a
/// plane's representative, so moving `a`'s origin within its plane
/// moves the line's point along the line. A ring about the line keeps
/// its outcome — every vertex, edge, face and decision — whichever
/// point it is built about; its coordinates may differ at the rounding
/// level, and nowhere more.
///
/// The planes are read off a block by a frame (`axes_at`) whose spins
/// are written as angles that round (`cos(π/2)` is about 6e-17), so even
/// the untilted line is a world axis only to rounding, and the ring
/// moves at the rounding level there too. Each coordinate of a copy is `R·(p − c) + c` for the pivot `c`:
/// the two shifts each round at half an ulp of the larger of `|p|` and
/// `|c|`, and the rotation's three-term dot products add about three
/// ulps of `|p − c|`, so a coordinate moves by at most about 8 ulps of
/// the scale `S = max(|p|, |c|)`. The row allows 16 ulps of `S`, with
/// `S` the largest coordinate or pivot in play (6 tilted, 8 untilted).
#[test]
fn a_meets_representative_moves_no_readers_outcome() {
    // Planes x = 0 and y = 0 meet in the z axis; `a`'s origin slides
    // within x = 0, so the line's point slides along z.
    let bound = 16.0 * f64::EPSILON * 8.0;
    let (base, shape) = numbers_and_shape(&ring_about_a_meet([0.0; 3], 0.0));
    for moved in [[0.0, 4.0, 8.0], [0.0, -0.75, 3.0], [0.0, 0.1, 0.3]] {
        let (numbers, other) = numbers_and_shape(&ring_about_a_meet(moved, 0.0));
        assert_eq!(other, shape, "a's origin at {moved:?}: the outcome moved");
        assert_eq!(numbers.len(), base.len());
        for (x, y) in numbers.iter().zip(&base) {
            assert!(
                (x - y).abs() <= bound,
                "a's origin at {moved:?}: {x} against {y}"
            );
        }
    }
    // Tilted: `a` is spun to the normal (0.6, 0, 0.8), up to the
    // rounding of its angle, and meets `y = 0` along (−0.8, 0, 0.6); its
    // origin slides within `a`, up to the same rounding.
    let tilt = TILT;
    let (base, shape) = numbers_and_shape(&ring_about_a_meet([0.0; 3], tilt));
    let bound = 16.0 * f64::EPSILON * 6.0;
    let mut moved_any = false;
    for moved in [[0.8, 0.0, -0.6], [4.0, 1.0, -3.0], [1.2, 0.0, -0.9]] {
        let (numbers, other) = numbers_and_shape(&ring_about_a_meet(moved, tilt));
        assert_eq!(other, shape, "a's origin at {moved:?}: the outcome moved");
        assert_eq!(numbers.len(), base.len());
        for (x, y) in numbers.iter().zip(&base) {
            assert!(
                (x - y).abs() <= bound,
                "a's origin at {moved:?}: {x} against {y}"
            );
        }
        moved_any |= numbers != base;
    }
    // The tilted case is where the rounding shows; a row that never saw
    // it would not be measuring the bound.
    assert!(
        moved_any,
        "off the axes the representative reaches the bits"
    );
}

/// **A representative is deterministic** (D9: bits are a function of
/// the reads): the same definition, built and evaluated twice, gives
/// the ring bit for bit.
#[test]
fn a_meets_representative_is_a_function_of_its_definition() {
    for origin in [[0.0; 3], [4.0, 1.0, -3.0]] {
        assert_eq!(
            ring_about_a_meet(origin, TILT),
            ring_about_a_meet(origin, TILT),
            "a's origin at {origin:?}"
        );
    }
}

/// The tool of the content-key row: the plane normal to the line where
/// `f1`'s yz plane meets `f2`'s xy plane, through the foot of the point
/// at `(0.5, 0.5, 0.5)` in `point_frame` — the frame through them read
/// through its xy plane, slid by the constant `0` along its z so that a
/// re-point, which reads plainly, defines a plane.
fn meet_through_tool(f1: &Operand, f2: &Operand, point_frame: &Operand) -> Operand {
    let through = || {
        pose(PoseDef::Through {
            axis: plain(pose(PoseDef::Meet {
                a: RoleRead::through(f1.clone(), PoseRole::Yz),
                b: RoleRead::through(f2.clone(), PoseRole::Xy),
            })),
            point: plain(pose(PoseDef::point_in(point_frame, [0.5; 3].map(len)))),
        })
    };
    pose(PoseDef::Slide {
        pose: RoleRead::through(through(), PoseRole::Xy),
        along: RoleRead::through(through(), PoseRole::Z),
        by: Formula::zero(Dimension::Length),
    })
}

/// **Two pose trees reaching the same operations in another
/// arrangement key apart** (the dual review's MAJOR): a tool whose
/// point is written in `f1` and one whose point is written in `f2`
/// read the same two frames off one body, so their upstream lists are
/// equal; their content keys must still differ, or the memo serves one
/// tool's halves for the other's.
#[test]
fn a_pose_trees_leaves_are_keyed_by_the_port_they_read() {
    let (doc, cube) = unit_box();
    let f1 = frame_on(&doc, cube, &editor_core::Frame::IDENTITY);
    let f2 = frame_on(
        &doc,
        cube,
        &editor_core::Frame::translation([0.3, 0.2, 0.0]),
    );
    let (first, split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: meet_through_tool(&f1, &f2, &f1).into(),
        },
    );
    let ev_first = eval(&first);
    let second = editor_core::apply(
        &first,
        &DocEdit::SetParam {
            node: split,
            slot: editor_core::SlotId::Operand(OperandSlot::Tool),
            value: editor_core::SlotValue::Read(meet_through_tool(&f1, &f2, &f2)),
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
    assert_ne!(key_first, key_second, "two tools, one content key");
    assert_eq!(ok(&memo).1, body_second, "the memo served the other tool");
}

/// **Row 1 on a reversed face**: a revolved washer's two annuli are
/// minted with opposite senses, and each still reads its outward
/// normal — the lower one −y, the upper one +y along the washer's own
/// axis — so a reader that took the carrier's chart axis instead would
/// turn one of them inward.
#[test]
fn both_annuli_of_a_revolved_washer_read_outward() {
    let (doc, profile) = fixture::insert(
        ProfileDoc::empty_derived("s3a_annuli", Tol::witness()),
        Node::Profile(fixture::desc(vec![fixture::square(2.0, 0.0, 0.5)])),
    );
    let (doc, washer) = fixture::insert(
        doc,
        fixture::revolve_about(
            profile,
            (0.0, 0.0),
            crate::fixture::along_y(),
            ang(std::f64::consts::TAU),
        ),
    );
    let ev = eval(&doc);
    let annuli: Vec<_> = all_faces(&ev, washer)
        .into_iter()
        .filter(|n| face_carrier_kind(&ev, washer, n) == Ok(SurfaceKind::Plane))
        .collect();
    assert_eq!(annuli.len(), 2);
    let mut doc = doc;
    let mut planes = Vec::new();
    for face in annuli {
        let (next, plane) = split_by(
            doc,
            washer,
            pose(PoseDef::Plane {
                face: select(washer, face),
            }),
        );
        doc = next;
        planes.push(plane);
    }
    let ev = eval(&doc);
    let mut heights = Vec::new();
    for plane in planes {
        let value = bound_pose(&doc, &ev, OperandSlot::Tool, plane, Tol::witness());
        let Ok(PoseValue::Plane { normal, offset }) = value else {
            panic!("a plane, not {value:?}");
        };
        let n = UnitVec3::get(normal);
        near(n.y.abs(), 1.0, "an annulus is normal to the axis");
        assert!(offset > 0.0, "inward: offset {offset}, normal {n:?}");
        heights.push(offset * n.y);
    }
    heights.sort_by(f64::total_cmp);
    near(heights[0], -0.5, "the lower annulus");
    near(heights[1], 0.5, "the upper annulus");
}

/// **The same, when the scalar becomes observed later** (the dual
/// review's second MAJOR): the standoff reads a free `k` the split's
/// insert minted before any measure, and a later redefinition makes
/// `k` a measured length. Definition order now sees the pose
/// definition's reads, so the walk that finds observed variables meets
/// the pose after `k` and the redefinition refuses.
#[test]
fn a_pose_whose_scalar_becomes_observed_refuses_the_redefinition() {
    let (doc, cube) = unit_box();
    let k = VarName::from_static("k");
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: k.clone(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, -0.25)),
        },
    );
    let cap = fixture::fname(cube, RoleSeg::Cap(CapEnd::End));
    let (doc, _split) = split_by(
        doc,
        cube,
        pose(standoff(
            pose(PoseDef::Plane {
                face: select(cube, cap),
            }),
            Formula::named(k.clone(), Dimension::Length),
        )),
    );
    let (doc, measure) = fixture::measure_node(
        &doc,
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            fixture::cap_ref(cube, CapEnd::Start),
            fixture::cap_ref(cube, CapEnd::End),
        ],
    );
    let out = fixture::output(&doc, measure);
    let refused = editor_core::apply(
        &doc,
        &DocEdit::DefineVar {
            var: k.into(),
            def: editor_core::VarDecl::defined(
                Formula::mul(Formula::var(out, Dimension::Length), scl(-0.25)).unwrap(),
            ),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(
        matches!(
            refused,
            Err(EditError::ConstructionReadsObserved {
                slot: editor_core::SlotId::Operand(OperandSlot::Tool),
                ..
            })
        ),
        "{refused:?}"
    );
}

/// The flip-standoff document the door rows read: a split whose tool is
/// the top cap's plane, flipped, slid along its normal by a free length;
/// and that split's tool variable.
fn flipped_standoff() -> (ProfileDoc, VarId) {
    let (doc, cube) = unit_box();
    let cap = fixture::fname(cube, RoleSeg::Cap(CapEnd::End));
    let (doc, tool) = split_by(
        doc,
        cube,
        pose(standoff(
            pose(PoseDef::Flip {
                pose: plain(pose(PoseDef::Plane {
                    face: select(cube, cap),
                })),
            }),
            len(0.25),
        )),
    );
    (doc, tool.read)
}

/// **The edit door normalises a flip of a flip** (FORK-S3M: an
/// involution normalised to one side): a tool written `Flip(Flip(p))`
/// is stored as `p`'s definition, never a flip; a point flipped twice
/// still refuses, because a flip admits only what has a sense.
#[test]
fn the_door_normalises_a_flip_of_a_flip() {
    let (doc, cube) = unit_box();
    let cap = fixture::fname(cube, RoleSeg::Cap(CapEnd::End));
    let plane = || {
        pose(PoseDef::Plane {
            face: select(cube, cap.clone()),
        })
    };
    let (doc, tool) = split_by(
        doc,
        cube,
        pose(PoseDef::Flip {
            pose: plain(pose(PoseDef::Flip {
                pose: plain(plane()),
            })),
        }),
    );
    let tool = tool.read;
    assert!(
        matches!(
            doc.var(tool).map(editor_core::Var::def),
            Some(editor_core::VarDef::Pose(PoseDef::Plane { .. }))
        ),
        "stored {:?}",
        doc.var(tool).map(editor_core::Var::def)
    );
    let corner = fixture::cap_vertex(
        cube,
        CapEnd::End,
        fixture::vpiece(&unit_box().0, cube, 0, 0),
    );
    let refused = fixture::insert_refused(
        &unit_box().0,
        Node::Split {
            target: Operand::output(cube, 0).into(),
            tool: pose(PoseDef::Flip {
                pose: plain(pose(PoseDef::Flip {
                    pose: plain(pose(PoseDef::Point {
                        of: select(cube, corner),
                    })),
                })),
            })
            .into(),
        },
    );
    assert!(
        matches!(
            &refused,
            EditError::SlotVarKind {
                found: VarKind::Point,
                ..
            }
        ),
        "{refused:?}"
    );
}

/// **The load door refuses a pose definition no edit door writes**
/// (`Walk::Pose`): a saved document, hand-edited four ways — a flip
/// reading a length, a slide by a plane, a flip of a flip, a plane read
/// through a frame's role — refuses `PoseShape` naming each fault, and
/// the file as saved round-trips.
#[test]
fn the_load_door_refuses_a_malformed_pose_definition() {
    let (doc, tool) = flipped_standoff();
    let Some(editor_core::VarDef::Pose(PoseDef::Slide {
        pose: flip,
        along,
        by,
    })) = doc.var(tool).map(editor_core::Var::def)
    else {
        panic!("a sliding tool");
    };
    let (flip, along, by) = (flip.read, along.read, *by);
    let Some(editor_core::VarDef::Pose(PoseDef::Flip { pose: face_plane })) =
        doc.var(flip).map(editor_core::Var::def)
    else {
        panic!("a flipped plane");
    };
    let face_plane = face_plane.read;
    let id = |var: VarId| serde_json::to_string(&var).unwrap();
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("saves");
    let back = editor_core::persist::load(&text, Tol::witness()).expect("loads as saved");
    assert_eq!(
        editor_core::persist::save(&back.snapshot, &back.edits, Tol::witness()).unwrap(),
        text,
        "round-trips bit-identical"
    );
    let flip_reads = format!("\"pose\": {}", id(face_plane));
    let slides_by = format!("\"by\": {}", id(by));
    let through_normal = "\"role\": \"Normal\"";
    assert!(
        text.contains(&flip_reads) && text.contains(&slides_by) && text.contains(through_normal),
        "the wire spelling moved"
    );
    for (mutated, fault) in [
        (
            text.replacen(&flip_reads, &format!("\"pose\": {}", id(by)), 1),
            "ReadKind",
        ),
        (
            text.replace(&slides_by, &format!("\"by\": {}", id(face_plane))),
            "ScalarKind",
        ),
        (
            text.replacen(&flip_reads, &format!("\"pose\": {}", id(along)), 1),
            "DoubleFlip",
        ),
        (text.replace(through_normal, "\"role\": \"Xy\""), "Role"),
    ] {
        assert_ne!(mutated, text, "{fault}: the mutation applied");
        let refused = editor_core::persist::load(&mutated, Tol::witness());
        let said = format!("{refused:?}");
        assert!(
            refused.is_err() && said.contains("PoseShape") && said.contains(fault),
            "{fault}: {said}"
        );
    }
}

/// **A revolve's axis output is the line its body turns about**
/// (claim 6's second half): about an axis line off the profile's
/// origin and off its axes, the port-1 axis and the axis read off the
/// body's conical wall are one line — parallel, and each origin on the
/// other — to the last few ulps.
#[test]
fn a_revolves_axis_output_is_the_axis_its_body_turns_about() {
    let (doc, profile) = fixture::insert(
        ProfileDoc::empty_derived("s3a_port1", Tol::witness()),
        Node::Profile(fixture::desc(vec![fixture::square(2.0, 0.0, 0.5)])),
    );
    let (doc, revolve) = fixture::insert(
        doc,
        // The heading of the line along `(0.3, 1)`, `atan2(1, 0.3)`.
        fixture::revolve_about(
            profile,
            (0.25, 0.5),
            ang(1.279_339_532_317_029_6),
            ang(std::f64::consts::TAU),
        ),
    );
    let ev = eval(&doc);
    // The square's sides are oblique to the axis, so its walls are
    // cones; each carries the body's axis.
    let wall = all_faces(&ev, revolve)
        .into_iter()
        .find(|n| face_carrier_kind(&ev, revolve, n) == Ok(SurfaceKind::Cone))
        .expect("a conical wall");
    let wall_axis = RoleRead::plain(pose(PoseDef::Axis {
        of: select(revolve, wall),
    }));
    let (doc, _, ring) = fixture::circular(
        doc,
        revolve,
        (wall_axis, editor_core::TargetAxis::Y),
        std::f64::consts::FRAC_PI_2,
        2,
    );
    let ev = eval(&doc);
    let axis_of = |var| match bound_pose(&doc, &ev, OperandSlot::Axis, var, Tol::witness()) {
        Ok(PoseValue::Axis(line)) => (line.foot(), UnitVec3::get(line.dir())),
        other => panic!("an axis, not {other:?}"),
    };
    let port = doc.output(revolve, 1).expect("a revolve defines its axis");
    // The ring's mate equates the wall's axis on the copy with the
    // wall's axis read off the revolve: its `to` side.
    let read = fixture::bundle(&doc, ring)
        .into_iter()
        .find_map(|c| c.mate().map(|m| m.to))
        .expect("the ring");
    let ((o1, d1), (o2, d2)) = (axis_of(port), axis_of(read));
    let slack = 8.0 * f64::EPSILON * 4.0;
    assert!(d1.cross(d2).norm() <= slack, "{d1:?} against {d2:?}");
    let off = |o: geom_core::Point3<f64>, at: geom_core::Point3<f64>, d: geom_core::Vec3<f64>| {
        (o - at).cross(d).norm()
    };
    assert!(
        off(o1, o2, d2) <= slack,
        "port 1's origin off the wall's axis: {o1:?}"
    );
    assert!(
        off(o2, o1, d1) <= slack,
        "the wall's origin off port 1: {o2:?}"
    );
}

/// A subgroup's Lie algebra as twists `(ω, v)` (`v` the velocity of the
/// point at the origin), as a spanning list.
fn algebra(g: &editor_core::Subgroup<f64>) -> Vec<[f64; 6]> {
    use editor_core::Subgroup as S;
    use geom_core::Vec3;
    let twist = |w: Vec3<f64>, v: Vec3<f64>| [w.x, w.y, w.z, v.x, v.y, v.z];
    let zero = Vec3::new(0.0, 0.0, 0.0);
    let axes = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    // A rotation about the line through `p` along `d`.
    let turn = |p: geom_core::Point3<f64>, d: Vec3<f64>| {
        let p = p - geom_core::Point3::origin();
        twist(d, p.cross(d))
    };
    let across = |n: Vec3<f64>| axes.iter().map(move |&a| twist(zero, a - n * a.dot(n)));
    match *g {
        S::Se3 => axes
            .iter()
            .flat_map(|&a| [twist(a, zero), twist(zero, a)])
            .collect(),
        S::Planar { normal } => {
            let n = UnitVec3::get(normal);
            std::iter::once(twist(n, zero)).chain(across(n)).collect()
        }
        S::Cylindrical { point, direction } => {
            let d = UnitVec3::get(direction);
            vec![turn(point, d), twist(zero, d)]
        }
        S::Prismatic { direction } => vec![twist(zero, UnitVec3::get(direction))],
        S::Revolute { point, direction } => vec![turn(point, UnitVec3::get(direction))],
        S::Spherical { point } => axes.iter().map(|&a| turn(point, a)).collect(),
        S::Parallel { direction } => std::iter::once(twist(UnitVec3::get(direction), zero))
            .chain(axes.iter().map(|&a| twist(zero, a)))
            .collect(),
        S::Translation => axes.iter().map(|&a| twist(zero, a)).collect(),
        S::PlaneTranslation { normal } => across(UnitVec3::get(normal)).collect(),
        S::Trivial => Vec::new(),
    }
}

/// The rank of a list of six-vectors, by elimination at `1e-9`.
fn rank(rows: &[[f64; 6]]) -> usize {
    let mut m: Vec<[f64; 6]> = rows.to_vec();
    let mut r = 0;
    for col in 0..6 {
        let Some(pivot) = (r..m.len()).max_by(|&a, &b| m[a][col].abs().total_cmp(&m[b][col].abs()))
        else {
            break;
        };
        if m[pivot][col].abs() < 1e-9 {
            continue;
        }
        m.swap(r, pivot);
        for i in 0..m.len() {
            if i != r {
                let f = m[i][col] / m[r][col];
                let pivot_row = m[r];
                for (x, p) in m[i].iter_mut().zip(pivot_row) {
                    *x -= f * p;
                }
            }
        }
        r += 1;
    }
    r
}

/// **The family meet is the intersection in general position** (the
/// coset module's proof obligation, executed): for every pair of the
/// ten families, each placed in general position (directions and points
/// no two of which are parallel, perpendicular or incident), the
/// dimension [`editor_core::SubgroupFamily::meet`] answers is the
/// dimension of the intersection of the two Lie algebras, computed here
/// independently as subspaces of the twists, in both orders.
///
/// *Breaks if* a row of the meet answers a family a general pair does
/// not meet in, which would admit or refuse a bundle wrongly.
#[test]
fn the_family_meet_spans_the_intersection_in_general_position() {
    use editor_core::Subgroup as S;
    use editor_core::SubgroupFamily as F;
    use geom_core::{Point3, Vec3};
    let band = geom_core::predicate::Band::linear(Tol::witness()).expect("a band");
    let unit = |x: f64, y: f64, z: f64| {
        UnitVec3::new(Vec3::new(x, y, z), topo::query::POSE_UNIT_NORM, band).unwrap()
    };
    // One general placing per side: no two of these directions are
    // parallel or perpendicular, and neither point lies on the other
    // side's line.
    let place = |family: F, side: usize| {
        let (d, p) = if side == 0 {
            (unit(0.3, 0.5, 0.81), Point3::new(0.17, -0.43, 0.29))
        } else {
            (unit(-0.62, 0.21, 0.4), Point3::new(-0.38, 0.71, -0.13))
        };
        match family {
            F::Se3 => S::Se3,
            F::Planar => S::Planar { normal: d },
            F::Cylindrical => S::Cylindrical {
                point: p,
                direction: d,
            },
            F::Prismatic => S::Prismatic { direction: d },
            F::Revolute => S::Revolute {
                point: p,
                direction: d,
            },
            F::Spherical => S::Spherical { point: p },
            F::Parallel => S::Parallel { direction: d },
            F::Translation => S::Translation,
            F::PlaneTranslation => S::PlaneTranslation { normal: d },
            F::Trivial => S::Trivial,
        }
    };
    let all = [
        F::Se3,
        F::Planar,
        F::Cylindrical,
        F::Prismatic,
        F::Revolute,
        F::Spherical,
        F::Parallel,
        F::Translation,
        F::PlaneTranslation,
        F::Trivial,
    ];
    for a in all {
        for b in all {
            let (ga, gb) = (algebra(&place(a, 0)), algebra(&place(b, 1)));
            let both = [ga.clone(), gb.clone()].concat();
            // dim(A ∩ B) = dim A + dim B − dim(A + B).
            let wanted = rank(&ga) + rank(&gb) - rank(&both);
            assert_eq!(
                usize::from(a.meet(b).dimension()),
                wanted,
                "{a:?} ∩ {b:?} = {:?}: the dimension",
                a.meet(b)
            );
        }
    }
}
