//! Reviewer R2's probes of INTENT stage 6 C (PR #6043).

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::result_large_err
)]

use crate::fixture::{self, len, len2, scl};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, DrawnOn, EvalOptions, Evaluation, Formula,
    FreeVar, LoopProgram, Node, NodeErrorKind, Operand, ProfileDoc, ProfileProgram, ProgramStep,
    ProgramTarget, RecipeNodeId, StableName, TraceCrossing, TracePassage, TraceSense,
    TraceStepFault, UnitSym, ValuePayload, VarDecl, VarName, all_faces, evaluate,
    face_carrier_kind,
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

fn r() -> Formula {
    Formula::named(VarName::new("r").expect("a name"), Dimension::Length)
}

fn cylinder(doc: ProfileDoc, radius: f64, width: Option<f64>) -> (ProfileDoc, RecipeNodeId) {
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
            distance: len(1.0),
        },
    )
}

/// A cylinder of radius `r` as a rectangle revolved about its own y:
/// no circle loop, so no arc/arc identity over the box.
fn rcylinder(doc: ProfileDoc, radius: f64, width: Option<f64>) -> (ProfileDoc, RecipeNodeId) {
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
    let (doc, rect) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At(len2([0.0, 0.0])),
                ProgramStep::LineTo(ProgramTarget::Point([r(), len(0.0)])),
                ProgramStep::LineTo(ProgramTarget::Point([r(), len(1.0)])),
                ProgramStep::LineTo(ProgramTarget::Point(len2([0.0, 1.0]))),
                ProgramStep::LineTo(ProgramTarget::Start),
            ])],
            ids: Vec::new(),
            on: None,
        }),
    );
    fixture::insert(
        doc,
        fixture::revolve_about(
            rect,
            (0.0, 0.0),
            fixture::along_y(),
            fixture::ang(std::f64::consts::TAU),
        ),
    )
}

/// The boss on the revolved cylinder's base (the plane y = 0, sketch
/// x = world x, sketch y = world z): (doc, cyl, footprint, boss).
fn rboss(width: Option<f64>) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (doc, cyl) = rcylinder(
        ProfileDoc::empty_derived("r2_rboss", Tol::witness()),
        0.5,
        width,
    );
    let ev = eval(&doc);
    assert!(ev.node_error(cyl).is_none(), "{:?}", ev.node_error(cyl));
    let wall = face(&ev, cyl, SurfaceKind::Cylinder);
    let base = editor_core::test_support::axes_at([0.0; 3], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let node = drawn(&doc, cyl, &base, cyl, vec![wall], boss_steps());
    let (doc, fp) = fixture::insert(doc, node);
    let (doc, b) = fixture::insert(
        doc,
        Node::Extrude {
            profile: fp.into(),
            distance: len(0.5),
        },
    );
    (doc, cyl, fp, b)
}

fn face(ev: &Evaluation<f64>, node: RecipeNodeId, kind: SurfaceKind) -> StableName {
    all_faces(ev, node)
        .into_iter()
        .find(|n| face_carrier_kind(ev, node, n) == Ok(kind))
        .unwrap_or_else(|| panic!("a {kind:?} face"))
}

fn drawn(
    doc: &ProfileDoc,
    frame_body: RecipeNodeId,
    at: &editor_core::Frame,
    face_body: RecipeNodeId,
    faces: Vec<StableName>,
    steps: Vec<ProgramStep<Formula>>,
) -> editor_core::AuthoredNode {
    let frame = editor_core::test_support::frame_on(
        doc,
        &Operand::output(frame_body, 0),
        at,
        Tol::witness(),
    );
    Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::Chain(steps)],
        ids: Vec::new(),
        on: Some(DrawnOn {
            frame,
            faces: faces
                .into_iter()
                .map(|f| Operand::select(Operand::output(face_body, 0), vec![f]))
                .collect(),
        }),
    })
}

fn boss_steps() -> Vec<ProgramStep<Formula>> {
    vec![
        ProgramStep::At(len2([0.0, -0.25])),
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
        ProgramStep::LineTo(ProgramTarget::AtX(len(1.0))),
        ProgramStep::LineTo(ProgramTarget::Point(len2([1.0, -0.25]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]
}

/// (doc, cylinder, footprint, boss)
fn boss(width: Option<f64>) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (doc, cyl) = cylinder(
        ProfileDoc::empty_derived("r2_boss", Tol::witness()),
        0.5,
        width,
    );
    let ev = eval(&doc);
    let wall = face(&ev, cyl, SurfaceKind::Cylinder);
    let node = drawn(
        &doc,
        cyl,
        &editor_core::Frame::IDENTITY,
        cyl,
        vec![wall],
        boss_steps(),
    );
    let (doc, fp) = fixture::insert(doc, node);
    let (doc, b) = fixture::insert(
        doc,
        Node::Extrude {
            profile: fp.into(),
            distance: len(0.5),
        },
    );
    (doc, cyl, fp, b)
}

fn verts<T: geom_core::Decide + topo::AtRestPolicy>(
    ev: &Evaluation<T>,
    node: RecipeNodeId,
) -> Vec<[T; 3]> {
    let body = match &ev.value(node).expect("evaluated").payload {
        ValuePayload::Body(b) => fixture::in_space(b),
        other => panic!("{other:?}"),
    };
    body.vertices()
        .map(|(v, _)| {
            let p = topo::readback::vertex_point(&body, v).expect("vertex");
            [p.x, p.y, p.z]
        })
        .collect()
}

/// Claim 1: the `Sym` lane — does the boss's wall keep the radius
/// symbol, or is it frozen at the nominal?
#[test]
fn r2_sym_lane_wall_form() {
    use geom_core::{Bounds, Interval, Sym};
    let width = Tol::witness().eps() / 4096.0;
    let (doc, cyl, fp, b) = boss(Some(width));
    let opts = EvalOptions {
        param_box: Some(std::sync::Arc::new(editor_core::analysis::ParamBox::of(
            &editor_core::analysis::analyzed_box(
                &doc,
                &editor_core::analysis::AnalysisPolicy::default(),
            ),
        ))),
        ..EvalOptions::default()
    };
    let budget = geom_core::SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let (out, _) = geom_core::sym::with_session(budget, || {
        let ev = evaluate::<Sym<Interval>>(&doc, None, &CancelToken::new(), &opts, Tol::witness());
        if let Some(e) = ev.node_error(fp).or_else(|| ev.node_error(b)) {
            return Err(format!("{:?}", e.kind));
        }
        let wall = verts(&ev, b)
            .into_iter()
            .map(|p| p[1])
            .max_by(|a, b| a.value.hi().total_cmp(&b.value.hi()))
            .unwrap();
        let cx = verts(&ev, cyl)
            .into_iter()
            .map(|p| p[0])
            .max_by(|a, b| a.value.hi().total_cmp(&b.value.hi()))
            .unwrap();
        Ok((
            format!("{:?}", wall.value),
            geom_core::sym::report::render_of(wall.node()),
            format!("{:?}", cx.value),
            geom_core::sym::report::render_of(cx.node()),
        ))
    });
    eprintln!("SYM {out:#?}");
    let (_, wall_form, _, cyl_form) = out.expect("the boss builds at Sym");
    let wall_form = wall_form.expect("a form");
    let cyl_form = cyl_form.expect("a form");
    assert!(
        wall_form.chars().any(|c| c.is_alphabetic()),
        "the wall's Sym form is a constant: {wall_form} (cylinder: {cyl_form})"
    );
}

/// Claim 1: a wider Interval box — enclosure or a typed refusal.
#[test]
fn r2_interval_wider_boxes() {
    use geom_core::{Bounds, Interval};
    for width in [1e-13, 1e-11, 1e-9, 1e-7, 1e-5, 1e-3] {
        let (doc, _, fp, b) = rboss(Some(width));
        let opts = EvalOptions {
            param_box: Some(std::sync::Arc::new(editor_core::analysis::ParamBox::of(
                &editor_core::analysis::analyzed_box(
                    &doc,
                    &editor_core::analysis::AnalysisPolicy::default(),
                ),
            ))),
            ..EvalOptions::default()
        };
        let ev = evaluate::<Interval>(&doc, None, &CancelToken::new(), &opts, Tol::witness());
        if let Some(e) = ev.node_error(fp).or_else(|| ev.node_error(b)) {
            eprintln!("WIDTH {width}: refused {}", e.kind);
            continue;
        }
        let wall = verts(&ev, b)
            .into_iter()
            .map(|p| p[2])
            .max_by(|a, b| a.hi().total_cmp(&b.hi()))
            .unwrap();
        eprintln!("WIDTH {width}: wall {wall:?}");
        assert!(
            wall.lo() <= 0.5 - width && wall.hi() >= 0.5 + width,
            "width {width}: the wall {wall:?} does not enclose r's box"
        );
    }
}

/// Claim 2: Ev's torus, in a plane through its axis: entering on the
/// inner branch, leaving on the outer.
#[test]
fn r2_torus_through_its_axis() {
    let doc = ProfileDoc::empty_derived("r2_torus", Tol::witness());
    let (doc, ring) = fixture::insert(
        doc,
        Node::Tube {
            major_radius: len(2.0),
            window: editor_core::TubeWindow::Full,
            minor_radius: len(0.5),
        },
    );
    let ev = eval(&doc);
    let tube = face(&ev, ring, SurfaceKind::Torus);
    // Which axis is the torus about? Try the upright frame (sketch y =
    // world z) — the plane through the axis if the axis is z.
    let upright =
        editor_core::test_support::axes_at([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let cases = [
        (TracePassage::Enter, 1, Some(-2.5)),
        (TracePassage::Leave, 1, Some(-1.5)),
        (TracePassage::Enter, 2, Some(1.5)),
        (TracePassage::Leave, 2, Some(2.5)),
        (TracePassage::Enter, 3, None),
    ];
    for (passage, nth, want) in cases {
        let steps = vec![
            ProgramStep::At(len2([-3.0, 0.0])),
            ProgramStep::Toward {
                dx: scl(1.0),
                dy: scl(0.0),
            },
            ProgramStep::LineTo(ProgramTarget::OnTrace {
                face: 0,
                crossing: TraceCrossing { passage, nth },
            }),
            ProgramStep::LineTo(ProgramTarget::Point(len2([-3.0, 1.0]))),
            ProgramStep::LineTo(ProgramTarget::Start),
        ];
        let node = drawn(&doc, ring, &upright, ring, vec![tube.clone()], steps);
        let (d, p) = fixture::insert(doc.clone(), node);
        let got = crossing_x_in(&d, p, 0);
        eprintln!("TORUS {passage:?} {nth}: {got:?}");
        match want {
            Some(x) => {
                let got = got.unwrap_or_else(|e| panic!("{passage:?} {nth}: {e}"));
                assert!((got - x).abs() < 1e-9, "{passage:?} {nth}: {got} vs {x}");
            }
            None => assert!(got.is_err(), "{passage:?} {nth}: {got:?}"),
        }
    }
    // Leave on the outer branch then continue along the trace: tangent
    // there is vertical (sketch y).
    for sense in [TraceSense::With, TraceSense::Against] {
        let steps = vec![
            ProgramStep::At(len2([1.0, 0.0])),
            ProgramStep::Toward {
                dx: scl(1.0),
                dy: scl(0.0),
            },
            ProgramStep::LineTo(ProgramTarget::OnTrace {
                face: 0,
                crossing: TraceCrossing::first(TracePassage::Leave),
            }),
            ProgramStep::AlongTrace { sense },
            ProgramStep::Line(len(0.25)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ];
        let node = drawn(&doc, ring, &upright, ring, vec![tube.clone()], steps);
        let (d, p) = fixture::insert(doc.clone(), node);
        let ev = eval(&d);
        eprintln!(
            "TORUS along {sense:?}: {:?}",
            ev.node_error(p)
                .map(|e| e.kind.to_string())
                .unwrap_or_else(|| profile_points(&ev, p))
        );
    }
}

fn profile_points(ev: &Evaluation<f64>, p: RecipeNodeId) -> String {
    format!("{:?}", ev.node_error(p).map(|e| e.kind.to_string()))
}

/// The sketch-x of the crossing (frame = body axes): the profile is pen
/// → crossing → (pen.x, 1) → pen, so its extrude's max x is the
/// crossing's x in the frame.
fn crossing_x_in(doc: &ProfileDoc, p: RecipeNodeId, axis: usize) -> Result<f64, String> {
    let (d, e) = fixture::insert(
        doc.clone(),
        Node::Extrude {
            profile: p.into(),
            distance: len(0.1),
        },
    );
    let ev = eval(&d);
    if let Some(err) = ev.node_error(p).or_else(|| ev.node_error(e)) {
        return Err(err.kind.to_string());
    }
    Ok(verts(&ev, e)
        .into_iter()
        .map(|v| v[axis])
        .fold(f64::NEG_INFINITY, f64::max))
}

/// Claim 2: a concave face — a bore. Running out from the bore's axis
/// ENTERS the plate's material at the bore wall.
#[test]
fn r2_concave_bore_passage() {
    let doc = ProfileDoc::empty_derived("r2_bore", Tol::witness());
    let (doc, plate) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![
                LoopProgram::polygon([(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)])
                    .expect("a square"),
                LoopProgram::Circle {
                    centre: [len(0.0), len(0.0)],
                    radius: len(0.5),
                },
            ],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, slab) = fixture::insert(
        doc,
        Node::Extrude {
            profile: plate.into(),
            distance: len(1.0),
        },
    );
    let ev = eval(&doc);
    assert!(ev.node_error(slab).is_none(), "{:?}", ev.node_error(slab));
    let bore = face(&ev, slab, SurfaceKind::Cylinder);
    for (passage, ok) in [(TracePassage::Enter, true), (TracePassage::Leave, false)] {
        let steps = vec![
            ProgramStep::At(len2([0.0, 0.0])),
            ProgramStep::Toward {
                dx: scl(1.0),
                dy: scl(0.0),
            },
            ProgramStep::LineTo(ProgramTarget::OnTrace {
                face: 0,
                crossing: TraceCrossing::first(passage),
            }),
            ProgramStep::LineTo(ProgramTarget::Point(len2([0.0, 1.0]))),
            ProgramStep::LineTo(ProgramTarget::Start),
        ];
        let node = drawn(
            &doc,
            slab,
            &editor_core::Frame::IDENTITY,
            slab,
            vec![bore.clone()],
            steps,
        );
        let (d, p) = fixture::insert(doc.clone(), node);
        let got = crossing_x_in(&d, p, 0);
        eprintln!("BORE {passage:?}: {got:?}");
        assert_eq!(got.is_ok(), ok, "{passage:?}: {got:?}");
        if ok {
            assert!((got.unwrap() - 0.5).abs() < 1e-9);
        }
    }
}

/// Claim 2/5: a line tangent to the cylinder's trace circle grazes.
#[test]
fn r2_tangent_line_grazes() {
    let (doc, cyl) = cylinder(
        ProfileDoc::empty_derived("r2_graze", Tol::witness()),
        0.5,
        None,
    );
    let ev = eval(&doc);
    let wall = face(&ev, cyl, SurfaceKind::Cylinder);
    for (y, crossing) in [
        (0.5, TraceCrossing::first(TracePassage::Enter)),
        (0.5 - 1e-13, TraceCrossing::first(TracePassage::Enter)),
        (0.5 + 1e-13, TraceCrossing::first(TracePassage::Enter)),
    ] {
        let steps = vec![
            ProgramStep::At(len2([-2.0, y])),
            ProgramStep::Toward {
                dx: scl(1.0),
                dy: scl(0.0),
            },
            ProgramStep::LineTo(ProgramTarget::OnTrace { face: 0, crossing }),
            ProgramStep::LineTo(ProgramTarget::Point(len2([-2.0, 2.0]))),
            ProgramStep::LineTo(ProgramTarget::Start),
        ];
        let node = drawn(
            &doc,
            cyl,
            &editor_core::Frame::IDENTITY,
            cyl,
            vec![wall.clone()],
            steps,
        );
        let (d, p) = fixture::insert(doc.clone(), node);
        let ev = eval(&d);
        let got = ev.node_error(p).map(|e| &e.kind);
        eprintln!("GRAZE y={y}: {got:?}");
        assert!(
            matches!(
                got,
                Some(NodeErrorKind::Trace(t)) if matches!(
                    t.fault,
                    TraceStepFault::Carrier(topo::boolean::trace::TraceFault::Grazing)
                        | TraceStepFault::Carrier(topo::boolean::trace::TraceFault::Escalated(_))
                )
            ),
            "y={y}: {got:?}"
        );
    }
}

/// Claim 3: a frame of one space with a face of another refuses at the
/// door.
#[test]
fn r2_frame_and_face_in_two_spaces_refuse_at_the_door() {
    let (doc, a) = cylinder(
        ProfileDoc::empty_derived("r2_spaces", Tol::witness()),
        0.5,
        None,
    );
    let (doc, disc) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius: len(0.75),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, b) = fixture::insert(
        doc,
        Node::Extrude {
            profile: disc.into(),
            distance: len(1.0),
        },
    );
    assert_ne!(doc.node_space(a), doc.node_space(b), "two spaces");
    let ev = eval(&doc);
    let wall_b = face(&ev, b, SurfaceKind::Cylinder);
    let node = drawn(
        &doc,
        a,
        &editor_core::Frame::IDENTITY,
        b,
        vec![wall_b],
        boss_steps(),
    );
    let got = editor_core::apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    eprintln!(
        "SPACES: {:?}",
        got.as_ref().map(|_| ()).map_err(|e| e.to_string())
    );
    assert!(
        matches!(got, Err(editor_core::EditError::SpaceMismatch { .. })),
        "{:?}",
        got.map(|_| ())
    );
}

/// Claim 3: a drawn profile revolved — the revolve's axis read in the
/// frame, and the body in the frame's space, with the frame off the
/// body's origin.
#[test]
fn r2_revolve_and_extrude_in_an_offset_frame() {
    let (doc, cyl) = cylinder(
        ProfileDoc::empty_derived("r2_offset", Tol::witness()),
        0.5,
        None,
    );
    let ev = eval(&doc);
    let wall = face(&ev, cyl, SurfaceKind::Cylinder);
    // A frame on the cylinder's top cap: z = 1, axes rotated a quarter
    // turn (sketch x = world y).
    let top =
        editor_core::test_support::axes_at([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
    let node = drawn(&doc, cyl, &top, cyl, vec![wall], boss_steps());
    let (doc, fp) = fixture::insert(doc, node);
    let (doc, ext) = fixture::insert(
        doc,
        Node::Extrude {
            profile: fp.into(),
            distance: len(0.5),
        },
    );
    let ev = eval(&doc);
    assert!(ev.node_error(ext).is_none(), "{:?}", ev.node_error(ext));
    let v = verts(&ev, ext);
    let max = |k: usize| v.iter().map(|p| p[k]).fold(f64::NEG_INFINITY, f64::max);
    let min = |k: usize| v.iter().map(|p| p[k]).fold(f64::INFINITY, f64::min);
    eprintln!(
        "OFFSET extrude x [{}, {}] y [{}, {}] z [{}, {}]",
        min(0),
        max(0),
        min(1),
        max(1),
        min(2),
        max(2)
    );
    // Sketch y (world -x) reaches the trace at sketch y = 0.5 → world
    // x = -0.5; sketch x to 1 → world y = 1; z from 1 to 1.5.
    assert!((min(0) + 0.5).abs() < 1e-9, "world x min");
    assert!((max(1) - 1.0).abs() < 1e-9, "world y max");
    assert!(
        (min(2) - 1.0).abs() < 1e-9 && (max(2) - 1.5).abs() < 1e-9,
        "z"
    );
    // A revolve of it about its sketch x axis (world y): every vertex
    // stays at distance ≤ its sketch y from the world-y line through
    // (0,0,1).
    let (doc, rev) = fixture::insert(
        doc,
        fixture::revolve_about(
            fp,
            (0.0, -0.25),
            fixture::ang(std::f64::consts::PI),
            fixture::ang(std::f64::consts::PI),
        ),
    );
    let ev = eval(&doc);
    eprintln!(
        "OFFSET revolve: {:?}; same space {}",
        ev.node_error(rev).map(|e| e.kind.to_string()),
        doc.node_space(rev) == doc.node_space(cyl)
    );
    if ev.node_error(rev).is_none() {
        let v = verts(&ev, rev);
        // the axis in world: through (0.25, 0, 1)? sketch (0,-0.25) →
        // world x = 0.25, y = 0, z = 1, direction world y.
        let far = v
            .iter()
            .map(|p| ((p[0] - 0.25).powi(2) + (p[2] - 1.0).powi(2)).sqrt())
            .fold(0.0, f64::max);
        eprintln!("OFFSET revolve max distance from the posed axis: {far}");
        assert!(
            (far - 0.75).abs() < 1e-9,
            "the revolve's axis in the frame: {far}"
        );
    }
    let axis = doc.output(rev, 1).expect("a revolve's axis");
    let pose = editor_core::eval::bound_pose(
        &doc,
        &ev,
        editor_core::OperandSlot::Axis,
        axis,
        Tol::witness(),
    );
    let Ok(editor_core::PoseValue::Axis(line)) = pose else {
        panic!("the axis: {pose:?}")
    };
    let (foot, dir) = (line.foot(), line.dir().get());
    eprintln!("OFFSET revolve axis foot {foot:?} dir {dir:?}");
    assert!(
        (foot.x - 0.25).abs() < 1e-9 && foot.y.abs() < 1e-9 && (foot.z - 1.0).abs() < 1e-9,
        "the axis's foot in the space: {foot:?}"
    );
    assert!((dir.y.abs() - 1.0).abs() < 1e-9, "along world y: {dir:?}");
}

/// Control for the revolve: the same footprint free, and drawn on the
/// identity frame, revolved about the same line.
#[test]
fn r2_revolve_control() {
    let (doc, cyl) = cylinder(
        ProfileDoc::empty_derived("r2_rev_control", Tol::witness()),
        0.5,
        None,
    );
    let ev = eval(&doc);
    let wall = face(&ev, cyl, SurfaceKind::Cylinder);
    let free = Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(0.0, -0.25), (0.0, 0.5), (1.0, 0.5), (1.0, -0.25)])
                .expect("a quad"),
        ],
        ids: Vec::new(),
        on: None,
    });
    let top =
        editor_core::test_support::axes_at([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
    for (what, node) in [
        ("free", free),
        (
            "identity",
            drawn(
                &doc,
                cyl,
                &editor_core::Frame::IDENTITY,
                cyl,
                vec![wall.clone()],
                boss_steps(),
            ),
        ),
        (
            "top",
            drawn(&doc, cyl, &top, cyl, vec![wall.clone()], boss_steps()),
        ),
    ] {
        for (origin, heading) in [
            ((0.0, -0.25), 0.0),
            ((0.0, -0.25), std::f64::consts::PI),
            ((0.0, -1.0), 0.0),
        ] {
            let (d, p) = fixture::insert(doc.clone(), node.clone());
            let (d, rev) = fixture::insert(
                d,
                fixture::revolve_about(
                    p,
                    origin,
                    fixture::ang(heading),
                    fixture::ang(std::f64::consts::PI),
                ),
            );
            let ev = eval(&d);
            let res = match ev.node_error(rev) {
                Some(e) => e.kind.to_string(),
                None => {
                    let v = verts(&ev, rev);
                    format!("ok, {} vertices, first {:?}", v.len(), v.first())
                }
            };
            eprintln!("REVCTL {what} origin {origin:?} heading {heading}: {res}");
        }
    }
}

/// Claim 3 / departure 5: a loft whose first section is drawn stands
/// in the frame; one whose later section is drawn refuses typed.
#[test]
fn r2_loft_of_a_drawn_section() {
    let (doc, cyl) = cylinder(
        ProfileDoc::empty_derived("r2_loft", Tol::witness()),
        0.5,
        None,
    );
    let ev = eval(&doc);
    let wall = face(&ev, cyl, SurfaceKind::Cylinder);
    let top =
        editor_core::test_support::axes_at([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
    let node = drawn(&doc, cyl, &top, cyl, vec![wall], boss_steps());
    let (doc, fp) = fixture::insert(doc, node);
    let (doc, quad) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![
                LoopProgram::polygon([(0.0, -0.25), (0.0, 0.25), (0.5, 0.25), (0.5, -0.25)])
                    .expect("a quad"),
            ],
            ids: Vec::new(),
            on: None,
        }),
    );
    let loft = |a: RecipeNodeId, b: RecipeNodeId| Node::Loft {
        first: a.into(),
        then: vec![editor_core::LoftSection {
            profile: b.into(),
            station: vec![editor_core::StationMove::Slide {
                along: editor_core::Axis3::Z,
                by: len(1.0),
            }],
        }],
        v_degree: Formula::count(1),
    };
    let (d1, l1) = fixture::insert(doc.clone(), loft(fp, quad));
    let ev = eval(&d1);
    match ev.node_error(l1) {
        Some(e) => eprintln!("LOFT first drawn: refused {}", e.kind),
        None => {
            let v = verts(&ev, l1);
            let max = |k: usize| v.iter().map(|p| p[k]).fold(f64::NEG_INFINITY, f64::max);
            let min = |k: usize| v.iter().map(|p| p[k]).fold(f64::INFINITY, f64::min);
            eprintln!(
                "LOFT first drawn: x [{}, {}] y [{}, {}] z [{}, {}], space {}",
                min(0),
                max(0),
                min(1),
                max(1),
                min(2),
                max(2),
                d1.node_space(l1) == d1.node_space(cyl)
            );
            assert!(
                (min(2) - 1.0).abs() < 1e-9 && (max(2) - 2.0).abs() < 1e-9,
                "z"
            );
            assert!(
                (min(0) + 0.5).abs() < 1e-9,
                "world x min = -0.5 (the trace)"
            );
        }
    }
    let r = editor_core::apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(loft(quad, fp)),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    match r {
        Err(e) => eprintln!("LOFT later drawn: door refused {e}"),
        Ok(out) => {
            let d2 = out.doc;
            let l2 = *d2.ids().last().unwrap();
            let ev = eval(&d2);
            eprintln!(
                "LOFT later drawn: {:?}",
                ev.node_error(l2).map(|e| e.kind.to_string())
            );
            assert!(ev.node_error(l2).is_some(), "a later drawn section refuses");
        }
    }
}

/// Claim 4: moving the traced body's wall (its disc's centre, a
/// variable) moves the crossing: from (0, -1/4) up, the line leaves the
/// frame read off the body rides with it.
#[test]
fn r2_moving_the_traced_body() {
    let doc = ProfileDoc::empty_derived("r2_move", Tol::witness());
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::new("cx").expect("a name"),
            def: VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: 0.0,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: None,
            }),
        },
    );
    let (doc, disc) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [
                    Formula::named(VarName::new("cx").expect("a name"), Dimension::Length),
                    len(0.0),
                ],
                radius: len(0.5),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, cyl) = fixture::insert(
        doc,
        Node::Extrude {
            profile: disc.into(),
            distance: len(1.0),
        },
    );
    let ev = eval(&doc);
    let wall = face(&ev, cyl, SurfaceKind::Cylinder);
    let node = drawn(
        &doc,
        cyl,
        &editor_core::Frame::IDENTITY,
        cyl,
        vec![wall],
        boss_steps(),
    );
    let (doc, fp) = fixture::insert(doc, node);
    let (doc, b) = fixture::insert(
        doc,
        Node::Extrude {
            profile: fp.into(),
            distance: len(0.5),
        },
    );
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetVarValue {
            var: VarName::new("cx").expect("a name").into(),
            value: editor_core::FreeValue::Continuous(0.3),
        },
    );
    let ev = eval(&doc);
    let msg = ev
        .node_error(fp)
        .or_else(|| ev.node_error(b))
        .map(|e| e.kind.to_string());
    eprintln!("MOVE: {msg:?}");
    assert!(msg.is_none());
    let ys: Vec<f64> = verts(&ev, b).iter().map(|p| p[1]).collect();
    eprintln!(
        "MOVE boss xs {:?}",
        verts(&ev, b).iter().map(|p| p[0]).collect::<Vec<_>>()
    );
    let xs: Vec<f64> = verts(&ev, cyl).iter().map(|p| p[0]).collect();
    eprintln!("MOVE boss ys {ys:?}; cylinder xs {xs:?}");
    // The frame is read off the body, so it rides with it: the boss
    // moves with the cylinder and stays tangent at y = r.
    let bx = verts(&ev, b)
        .iter()
        .map(|p| p[0])
        .fold(f64::INFINITY, f64::min);
    assert!(
        (bx - 0.3).abs() < 1e-9,
        "the boss moved with the body: {bx}"
    );
    assert!(
        ys.iter().any(|y| (y - 0.5).abs() < 1e-12),
        "still tangent: {ys:?}"
    );
}

/// Claim 4: an incremental re-evaluation (with the prior evaluation)
/// after the radius moves recomputes the traced profile.
#[test]
fn r2_incremental_reevaluation_recomputes_the_trace() {
    let (doc, _cyl, fp, b) = boss(None);
    let ev1 = eval(&doc);
    let (doc2, _) = fixture::step(
        doc.clone(),
        DocEdit::SetVarValue {
            var: VarName::new("r").expect("a name").into(),
            value: editor_core::FreeValue::Continuous(0.625),
        },
    );
    let ev2 = evaluate::<f64>(
        &doc2,
        Some(&ev1),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(ev2.node_error(fp).is_none() && ev2.node_error(b).is_none());
    let wall = verts(&ev2, b)
        .iter()
        .map(|p| p[1])
        .fold(f64::NEG_INFINITY, f64::max);
    eprintln!("INCR wall {wall}");
    assert!((wall - 0.625).abs() < 1e-12, "stale trace: {wall}");
    // And back, reusing the moved evaluation.
    let ev3 = evaluate::<f64>(
        &doc,
        Some(&ev2),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let wall = verts(&ev3, b)
        .iter()
        .map(|p| p[1])
        .fold(f64::NEG_INFINITY, f64::max);
    eprintln!("INCR back wall {wall}");
    assert!((wall - 0.5).abs() < 1e-12, "stale trace: {wall}");
}

/// Does a FREE profile using `AtX` (no trace read) build?
#[test]
fn r2_free_profile_at_x() {
    let doc = ProfileDoc::empty_derived("r2_atx", Tol::witness());
    let (doc, p) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At(len2([0.0, 0.0])),
                ProgramStep::Toward {
                    dx: scl(1.0),
                    dy: scl(1.0),
                },
                ProgramStep::LineTo(ProgramTarget::AtX(len(1.0))),
                ProgramStep::LineTo(ProgramTarget::Point(len2([1.0, 0.0]))),
                ProgramStep::LineTo(ProgramTarget::Start),
            ])],
            ids: Vec::new(),
            on: None,
        }),
    );
    let ev = eval(&doc);
    eprintln!(
        "ATX free: {:?}",
        ev.node_error(p).map(|e| e.kind.to_string())
    );
}
