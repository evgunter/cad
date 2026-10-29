//! **INTENT stage 6 PR C — a sketch continues tangent to a surface's
//! trace** (`docs/INTENT-STAGE6-SPEC.md` §4, as ruled on PR #6006): a
//! profile drawn on a frame of a body's space reads the trace of a face
//! of that space, runs to a crossing of it, and continues along its
//! tangent; every sweep of it stands in that space.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::result_large_err
)]

use crate::fixture::{self, len, len2, scl};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, DrawnOn, EvalOptions, FreeVar, UnitSym,
    VarDecl, VarName, Evaluation, Formula, LoopProgram, Node,
    NodeErrorKind, Operand, ProfileDoc, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId,
    StableName, TraceCrossing, TracePassage, TraceSense, TraceStepFault, ValuePayload, all_faces,
    evaluate, face_carrier_kind,
};
use geom::SurfaceKind;
use geom_core::Tol;

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The cylinder's radius, meters, as the boss scene of stage 6 A types
/// it (`corpus/boss_tangent_by_value.rs`).
const RADIUS: f64 = 0.5;
/// The cylinder's height.
const HEIGHT: f64 = 1.0;
/// The boss's reach along `x`, and its back wall.
const REACH: f64 = 1.0;
const BACK: f64 = -0.25;
/// The boss's height.
const BOSS_H: f64 = 0.5;

/// A document holding the free variable `r` at `radius`, and a cylinder
/// of radius `r` standing on its own xy plane: the document, the
/// cylinder.
fn cylinder(radius: f64, width: Option<f64>) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("s6c_cylinder", Tol::witness());
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::new("r").expect("a name"),
            def: VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: radius,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: width.map(|w| Distribution::Uniform { lo: -w, hi: w }),
            }),
        },
    );
    let (doc, disc) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius: r(),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: disc.into(),
            distance: len(HEIGHT),
        },
    )
}

/// The cylinder's radius, read by name.
fn r() -> Formula {
    Formula::named(VarName::new("r").expect("a name"), Dimension::Length)
}

/// The name of `node`'s face on a carrier of `kind`.
fn face(ev: &Evaluation<f64>, node: RecipeNodeId, kind: SurfaceKind) -> StableName {
    all_faces(ev, node)
        .into_iter()
        .find(|n| face_carrier_kind(ev, node, n) == Ok(kind))
        .unwrap_or_else(|| panic!("a {kind:?} face"))
}

/// A frame read off `body`'s geometry, standing at its own origin.
fn frame_at_origin(doc: &ProfileDoc, body: RecipeNodeId) -> Operand {
    editor_core::test_support::frame_on(
        doc,
        &Operand::output(body, 0),
        &editor_core::Frame::IDENTITY,
        Tol::witness(),
    )
}

/// The boss's footprint, drawn on the cylinder's base and running onto
/// the trace of its wall: up from `(0, BACK)` until it leaves the disc,
/// then against the trace (its tangent there is `+x`) to `x = REACH`,
/// down to `BACK` and home.
fn boss_steps() -> Vec<ProgramStep<Formula>> {
    vec![
        ProgramStep::At(len2([0.0, BACK])),
        ProgramStep::Toward {
            dx: scl(0.0),
            dy: scl(1.0),
        },
        ProgramStep::LineTo(ProgramTarget::OnTrace {
            face: 0,
            crossing: TraceCrossing::first(TracePassage::Leave),
        }),
        ProgramStep::AlongTrace {
            sense: TraceSense::Against,
        },
        ProgramStep::LineTo(ProgramTarget::AtX(len(REACH))),
        ProgramStep::LineTo(ProgramTarget::Point(len2([REACH, BACK]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]
}

/// The cylinder, and a boss drawn on its base tracing its wall:
/// `(doc, cylinder, boss profile, boss extrude)`.
fn traced_boss(radius: f64, width: Option<f64>, steps: Vec<ProgramStep<Formula>>) -> Scene {
    let (doc, cylinder) = cylinder(radius, width);
    let ev = eval(&doc);
    let wall = face(&ev, cylinder, SurfaceKind::Cylinder);
    let frame = frame_at_origin(&doc, cylinder);
    let (doc, footprint) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(steps)],
            ids: Vec::new(),
            on: Some(DrawnOn {
                frame,
                faces: vec![Operand::select(Operand::output(cylinder, 0), vec![wall])],
            }),
        }),
    );
    let (doc, boss) = fixture::insert(
        doc,
        Node::Extrude {
            profile: footprint.into(),
            distance: len(BOSS_H),
        },
    );
    Scene {
        doc,
        cylinder,
        footprint,
        boss,
    }
}

struct Scene {
    doc: ProfileDoc,
    cylinder: RecipeNodeId,
    footprint: RecipeNodeId,
    boss: RecipeNodeId,
}

/// `node`'s body where it stands in its space: each vertex.
fn vertices<T: geom_core::Decide + topo::AtRestPolicy>(
    ev: &Evaluation<T>,
    node: RecipeNodeId,
) -> Vec<[T; 3]> {
    let body = match &ev.value(node).expect("the node evaluated").payload {
        ValuePayload::Body(b) => fixture::in_space(b),
        other => panic!("expected a body, got {other:?}"),
    };
    body.vertices()
        .map(|(v, _)| {
            let p = topo::readback::vertex_point(&body, v).expect("a live vertex");
            [p.x, p.y, p.z]
        })
        .collect()
}

/// The largest coordinate `axis` of `node`'s vertices.
fn max_at(ev: &Evaluation<f64>, node: RecipeNodeId, axis: usize) -> f64 {
    vertices(ev, node)
        .into_iter()
        .map(|p| p[axis])
        .fold(f64::NEG_INFINITY, f64::max)
}

fn near(a: f64, b: f64, what: &str) {
    assert!((a - b).abs() <= 1e-12, "{what}: {a} against {b}");
}

/// The trace's refusal at `node`.
fn trace_refusal(ev: &Evaluation<f64>, node: RecipeNodeId) -> editor_core::TraceRefusal {
    match ev.node_error(node).map(|e| &e.kind) {
        Some(NodeErrorKind::Trace(refusal)) => refusal.clone(),
        other => panic!("expected a trace refusal at {node:?}, got {other:?}"),
    }
}

/// **The boss runs onto the cylinder's trace and along its tangent**:
/// its wall stands at the cylinder's radius, in the cylinder's space,
/// and is the boss the value-typed scene types by hand.
#[test]
fn a_boss_drawn_on_the_cylinder_continues_tangent_to_its_wall() {
    let scene = traced_boss(RADIUS, None, boss_steps());
    let ev = eval(&scene.doc);
    assert!(
        ev.node_error(scene.footprint).is_none() && ev.node_error(scene.boss).is_none(),
        "the traced boss builds: {:?}",
        ev.node_error(scene.footprint)
            .or_else(|| ev.node_error(scene.boss))
            .map(|e| e.kind.to_string())
    );
    near(max_at(&ev, scene.boss, 1), RADIUS, "the boss's wall is at the radius");
    near(max_at(&ev, scene.boss, 0), REACH, "the boss reaches REACH");
    near(max_at(&ev, scene.boss, 2), BOSS_H, "the boss stands on the base");
    assert_eq!(
        scene.doc.node_space(scene.boss),
        scene.doc.node_space(scene.cylinder),
        "a sweep of a profile drawn on a frame is in the frame's space"
    );
}

/// **A moved radius moves the boss**: the trace is read again at every
/// evaluation, so the boss's wall follows the cylinder's.
#[test]
fn moving_the_cylinders_radius_moves_the_boss_with_it() {
    let scene = traced_boss(RADIUS, None, boss_steps());
    let (doc, _) = fixture::step(
        scene.doc,
        DocEdit::SetVarValue {
            var: VarName::new("r").expect("a name").into(),
            value: editor_core::FreeValue::Continuous(0.625),
        },
    );
    let ev = eval(&doc);
    near(max_at(&ev, scene.boss, 1), 0.625, "the wall follows the radius");
    near(max_at(&ev, scene.cylinder, 0), 0.625, "the cylinder's radius");
}

/// **The wall moves with the radius, to first order and over a box**:
/// its derivative by the radius is one at `Dual`, and at `Interval` over
/// a box on the radius its height encloses the box. The value-typed
/// boss's wall (stage 6 A) has derivative zero.
#[test]
fn the_traced_wall_moves_with_the_radius_at_every_lane() {
    use geom_core::{Bounds, Dual, Interval};
    // A box narrow enough that every decision the boss's validation
    // makes across it lands inside the band: the trace's root and its
    // tangent widen it some hundredfold at `Interval`.
    let width = Tol::witness().eps() / 4096.0;
    let scene = traced_boss(RADIUS, Some(width), boss_steps());
    let r = scene.doc.var_named("r").expect("the radius");

    let seeded = EvalOptions {
        seed: Some(r),
        ..EvalOptions::default()
    };
    let ev = evaluate::<Dual<f64>>(
        &scene.doc,
        None,
        &CancelToken::new(),
        &seeded,
        Tol::witness(),
    );
    assert!(
        ev.node_error(scene.boss).is_none(),
        "the boss builds at Dual: {:?}",
        ev.node_error(scene.boss).map(|e| e.kind.to_string())
    );
    let wall = vertices(&ev, scene.boss)
        .into_iter()
        .map(|p| p[1])
        .max_by(|a, b| a.value.total_cmp(&b.value))
        .expect("vertices");
    near(wall.value, RADIUS, "the wall at the radius");
    near(wall.deriv, 1.0, "the wall moves one for one with the radius");

    let boxed = EvalOptions {
        param_box: Some(std::sync::Arc::new(editor_core::analysis::ParamBox::of(
            &editor_core::analysis::analyzed_box(
                &scene.doc,
                &editor_core::analysis::AnalysisPolicy::default(),
            ),
        ))),
        ..EvalOptions::default()
    };
    let ev = evaluate::<Interval>(
        &scene.doc,
        None,
        &CancelToken::new(),
        &boxed,
        Tol::witness(),
    );
    assert!(
        ev.node_error(scene.boss).is_none(),
        "the boss builds over the box: {:?}",
        ev.node_error(scene.boss).map(|e| e.kind.to_string())
    );
    let wall = vertices(&ev, scene.boss)
        .into_iter()
        .map(|p| p[1])
        .max_by(|a, b| a.hi().total_cmp(&b.hi()))
        .expect("vertices");
    assert!(
        wall.lo() <= RADIUS - width * 0.99 && wall.hi() >= RADIUS + width * 0.99,
        "the wall's height encloses the radius's box: {wall:?}"
    );
    // The enclosure is wider than the box (the root and the tangent
    // widen it), and still inside the zero band, where the margin of
    // the tangency it constructs is decided.
    assert!(
        wall.hi() - wall.lo() < Tol::witness().eps(),
        "and inside the band: {wall:?}"
    );
}

/// **The union glues the constructed tangency, unproven until D**: the
/// boss's wall and the cylinder's meet in one `SurfaceTangent` row at
/// `TangentWitness`, reported unproven exactly as the value-typed
/// boss's is today (stage 6 A); stage 6 D proves it.
#[test]
fn the_union_records_the_constructed_tangency_unproven() {
    use editor_core::coincide::{self, Proof};
    let scene = traced_boss(RADIUS, None, boss_steps());
    let (doc, union) = fixture::insert(
        scene.doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                scene.cylinder.into(),
                scene.boss.into(),
            ]),
        },
    );
    let ev = eval(&doc);
    assert!(
        ev.node_error(union).is_none(),
        "the union builds: {:?}",
        ev.node_error(union).map(|e| e.kind.to_string())
    );
    let rows = &ev.value(union).expect("the union").coincidences;
    let proofs = coincide::prove(
        &doc,
        &[(union, &rows[..])],
        None,
        Tol::witness(),
        &CancelToken::new(),
    )
    .expect("an uncanceled door");
    let tangent: Vec<_> = rows
        .iter()
        .zip(&proofs[0])
        .filter(|(row, _)| row.relation == topo::Relation::SurfaceTangent)
        .collect();
    assert_eq!(tangent.len(), 1, "one tangency: {rows:?}");
    let (row, proof) = tangent[0];
    assert_eq!(row.site, topo::DecisionSite::TangentWitness);
    assert!(
        matches!(proof, Proof::Unproven { .. }),
        "unproven until stage 6 D: {proof:?}"
    );
}

/// The sphere's radius, the cone's base height and its apex, as stage
/// 6 A's `cone_on_sphere` types them by hand.
const SPHERE_R: f64 = 1.25;
const BASE: f64 = -1.0;
const APEX: f64 = -1.5625;

/// A sphere of radius `SPHERE_R`, a half disc revolved about its own
/// `y`, and its revolve.
fn sphere() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("s6c_sphere", Tol::witness());
    let (doc, half) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At(len2([0.0, -SPHERE_R])),
                ProgramStep::ArcTo(editor_core::ProgramArcData::Bulge {
                    target: ProgramTarget::Point(len2([0.0, SPHERE_R])),
                    b: scl(1.0),
                }),
                ProgramStep::LineTo(ProgramTarget::Start),
            ])],
            ids: Vec::new(),
            on: None,
        }),
    );
    fixture::insert(
        doc,
        fixture::revolve_about(
            half,
            (0.0, 0.0),
            fixture::along_y(),
            fixture::ang(std::f64::consts::TAU),
        ),
    )
}

/// **Ev's cone on a sphere, constructed**: drawn on the plane through
/// the sphere's axis, the cone's base runs out along `y = BASE` until it
/// leaves the sphere, then back along the sphere's trace to the axis,
/// and revolves about it.
fn traced_cone() -> Vec<ProgramStep<Formula>> {
    vec![
        ProgramStep::At(len2([0.0, BASE])),
        ProgramStep::Toward {
            dx: scl(1.0),
            dy: scl(0.0),
        },
        ProgramStep::LineTo(ProgramTarget::OnTrace {
            face: 0,
            crossing: TraceCrossing::first(TracePassage::Leave),
        }),
        ProgramStep::AlongTrace {
            sense: TraceSense::Against,
        },
        ProgramStep::LineTo(ProgramTarget::AtX(len(0.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]
}

/// A profile drawn on `body`'s own axes, tracing `face` of it, and
/// inserted: the document and the profile.
fn drawn_on(
    doc: ProfileDoc,
    body: RecipeNodeId,
    at: &editor_core::Frame,
    face: StableName,
    steps: Vec<ProgramStep<Formula>>,
) -> (ProfileDoc, RecipeNodeId) {
    let frame =
        editor_core::test_support::frame_on(&doc, &Operand::output(body, 0), at, Tol::witness());
    fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(steps)],
            ids: Vec::new(),
            on: Some(DrawnOn {
                frame,
                faces: vec![Operand::select(Operand::output(body, 0), vec![face])],
            }),
        }),
    )
}

/// **The cone builds tangent to the sphere along its base rim**, and
/// its union with the sphere refuses where the value-typed cone's does
/// (stage 6 A): the boolean has no closed-form arm for the pair
/// (`work/germ/`, cone × sphere).
#[test]
fn a_cone_drawn_on_a_sphere_runs_tangent_to_it_along_its_rim() {
    let (doc, sphere) = sphere();
    let ev = eval(&doc);
    let wall = face(&ev, sphere, SurfaceKind::Sphere);
    let (doc, section) = drawn_on(doc, sphere, &editor_core::Frame::IDENTITY, wall, traced_cone());
    let (doc, cone) = fixture::insert(
        doc,
        fixture::revolve_about(
            section,
            (0.0, 0.0),
            fixture::along_y(),
            fixture::ang(std::f64::consts::TAU),
        ),
    );
    let ev = eval(&doc);
    assert!(
        ev.node_error(cone).is_none(),
        "the cone builds: {:?}",
        ev.node_error(cone).map(|e| e.kind.to_string())
    );
    // The apex on the axis where the hand-typed scene puts it, and the
    // rim on the sphere: the generator from one to the other passes
    // the centre at the sphere's radius.
    let points = vertices(&ev, cone);
    let apex = points
        .iter()
        .min_by(|a, b| a[1].total_cmp(&b[1]))
        .expect("an apex");
    near(apex[1], APEX, "the apex where the tangent meets the axis");
    let rim = points
        .iter()
        .max_by(|a, b| a[0].total_cmp(&b[0]))
        .expect("a rim vertex");
    let along = [rim[0] - apex[0], rim[1] - apex[1], rim[2] - apex[2]];
    let to_centre = [-apex[0], -apex[1], -apex[2]];
    let cross = [
        along[1] * to_centre[2] - along[2] * to_centre[1],
        along[2] * to_centre[0] - along[0] * to_centre[2],
        along[0] * to_centre[1] - along[1] * to_centre[0],
    ];
    let norm = |v: [f64; 3]| (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    near(
        norm(cross) / norm(along),
        SPHERE_R,
        "the generator's distance from the centre is the radius",
    );
    let (doc, union) = fixture::insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                Operand::output(sphere, 0).into(),
                Operand::output(cone, 0).into(),
            ]),
        },
    );
    let ev = eval(&doc);
    assert!(
        matches!(
            ev.node_error(union).map(|e| &e.kind),
            Some(NodeErrorKind::Boolean(e))
                if e.kind() == topo::BooleanErrorKind::CurvedPierceUnsupported
        ),
        "the cone × sphere union refuses as the hand-typed one does: {:?}",
        ev.node_error(union).map(|e| e.kind.to_string())
    );
}

/// **Each refusal names its step and its face**, at the scalar that
/// decides it.
#[test]
fn a_step_reading_a_trace_refuses_typed() {
    use TraceStepFault as F;
    let toward = |dx, dy| ProgramStep::Toward {
        dx: scl(dx),
        dy: scl(dy),
    };
    let on = |passage, nth| {
        ProgramStep::LineTo(ProgramTarget::OnTrace {
            face: 0,
            crossing: TraceCrossing { passage, nth },
        })
    };
    let closing = |mut steps: Vec<ProgramStep<Formula>>| {
        steps.extend([
            ProgramStep::LineTo(ProgramTarget::Point(len2([REACH, BACK]))),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]);
        steps
    };
    let at = || ProgramStep::At(len2([0.0, BACK]));
    let rows: Vec<(&str, Vec<ProgramStep<Formula>>, u32, Option<u32>, F)> = vec![
        (
            "a second leave of a disc the line leaves once",
            closing(vec![at(), toward(0.0, 1.0), on(TracePassage::Leave, 2)]),
            2,
            Some(0),
            F::NoSuchCrossing {
                crossing: TraceCrossing {
                    passage: TracePassage::Leave,
                    nth: 2,
                },
                found: 1,
            },
        ),
        (
            "an entry the line, starting inside, never makes",
            closing(vec![at(), toward(0.0, 1.0), on(TracePassage::Enter, 1)]),
            2,
            Some(0),
            F::NoSuchCrossing {
                crossing: TraceCrossing::first(TracePassage::Enter),
                found: 0,
            },
        ),
        (
            "a line to a crossing with no heading bound",
            closing(vec![at(), on(TracePassage::Leave, 1)]),
            1,
            None,
            F::NoHeading,
        ),
        (
            "a heading along a trace no piece ran onto",
            closing(vec![
                at(),
                toward(0.0, 1.0),
                ProgramStep::Line(len(0.5)),
                ProgramStep::AlongTrace {
                    sense: TraceSense::With,
                },
            ]),
            3,
            None,
            F::OffTrace,
        ),
        (
            "a coordinate the heading never reaches",
            closing(vec![
                at(),
                toward(0.0, 1.0),
                ProgramStep::LineTo(ProgramTarget::AtX(len(REACH))),
            ]),
            2,
            None,
            F::Unreached,
        ),
        (
            "a trace target on a tangent arc",
            closing(vec![
                at(),
                toward(0.0, 1.0),
                ProgramStep::TangentArcTo(ProgramTarget::OnTrace {
                    face: 0,
                    crossing: TraceCrossing::first(TracePassage::Leave),
                }),
            ]),
            2,
            None,
            F::NotARun,
        ),
        (
            "a face the profile does not trace",
            closing(vec![
                at(),
                toward(0.0, 1.0),
                ProgramStep::LineTo(ProgramTarget::OnTrace {
                    face: 1,
                    crossing: TraceCrossing::first(TracePassage::Leave),
                }),
            ]),
            2,
            Some(1),
            F::NoSuchFace { traced: 1 },
        ),
        (
            "a line grazing the cylinder",
            closing(vec![
                ProgramStep::At(len2([RADIUS, BACK])),
                toward(0.0, 1.0),
                on(TracePassage::Leave, 1),
            ]),
            2,
            Some(0),
            F::Carrier(topo::boolean::trace::TraceFault::Grazing),
        ),
    ];
    for (what, steps, step, face_at, fault) in rows {
        let scene = traced_boss(RADIUS, None, steps);
        let ev = eval(&scene.doc);
        let refusal = trace_refusal(&ev, scene.footprint);
        assert_eq!(
            (refusal.loop_, refusal.step, refusal.face, &refusal.fault),
            (0, step, face_at, &fault),
            "{what}: {refusal}"
        );
    }
}

/// **A frame whose plane misses the face has no trace**, and a profile
/// with a trace step drawn on no frame has none to read.
#[test]
fn no_plane_no_trace() {
    let (doc, cylinder) = cylinder(RADIUS, None);
    let ev = eval(&doc);
    let wall = face(&ev, cylinder, SurfaceKind::Cylinder);
    // A plane along the axis, beyond the wall.
    let beyond = editor_core::test_support::axes_at(
        [2.0 * RADIUS, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    );
    let (doc, profile) = drawn_on(doc, cylinder, &beyond, wall, boss_steps());
    let ev = eval(&doc);
    let refusal = trace_refusal(&ev, profile);
    assert_eq!(
        (refusal.step, refusal.face, refusal.fault),
        (2, Some(0), TraceStepFault::NoTrace)
    );

    let (doc, free) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(boss_steps())],
            ids: Vec::new(),
            on: None,
        }),
    );
    let ev = eval(&doc);
    assert_eq!(
        trace_refusal(&ev, free).fault,
        TraceStepFault::Undrawn,
        "a free profile reads no trace"
    );
}

/// **A piece running onto a cone's apex finds no tangent there.**
#[test]
fn a_crossing_at_a_cones_apex_has_no_tangent() {
    let (doc, sphere) = sphere();
    let ev = eval(&doc);
    let wall = face(&ev, sphere, SurfaceKind::Sphere);
    let (doc, section) = drawn_on(doc, sphere, &editor_core::Frame::IDENTITY, wall, traced_cone());
    let (doc, cone) = fixture::insert(
        doc,
        fixture::revolve_about(
            section,
            (0.0, 0.0),
            fixture::along_y(),
            fixture::ang(std::f64::consts::TAU),
        ),
    );
    let ev = eval(&doc);
    let side = face(&ev, cone, SurfaceKind::Cone);
    // Up the cone's axis, through its apex.
    let up = vec![
        ProgramStep::At(len2([0.0, -2.0])),
        ProgramStep::Toward {
            dx: scl(0.0),
            dy: scl(1.0),
        },
        ProgramStep::LineTo(ProgramTarget::OnTrace {
            face: 0,
            crossing: TraceCrossing::first(TracePassage::Enter),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(len2([1.0, 0.0]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ];
    let (doc, profile) = drawn_on(doc, cone, &editor_core::Frame::IDENTITY, side, up);
    let ev = eval(&doc);
    let refusal = trace_refusal(&ev, profile);
    assert_eq!(
        (refusal.step, refusal.face, refusal.fault),
        (2, Some(0), TraceStepFault::NoTangent)
    );
}

/// **A traced profile survives the wire**: saved and loaded, it is the
/// same document and builds the same boss.
#[test]
fn a_traced_profile_round_trips() {
    let scene = traced_boss(RADIUS, None, boss_steps());
    let text = editor_core::persist::save(&scene.doc, &[], Tol::witness()).expect("saves");
    let loaded = editor_core::persist::load(&text, Tol::witness()).expect("loads");
    assert_eq!(loaded.doc, scene.doc, "the same document");
    let ev = eval(&loaded.doc);
    near(max_at(&ev, scene.boss, 1), RADIUS, "the same boss");
}
