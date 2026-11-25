//! **A profile on a face tracing faces of its body** (D10 `[placement]`;
//! `REFERENCES.md` DM1; DESIGN "Constructing a coincidence"): drawn on
//! the face's chart ([`editor_core::draw_on_face`], its
//! [`OnFace::traced`]), so a profile on a face is one act whether or not
//! it traces. A boss drawn on a cylinder's cap, leaving the wall's trace
//! along its tangent and extruded by a signed depth, is tangent to the
//! wall by construction: the cap's normal and the wall's generator are
//! both the cylinder's extrude heading, which the door reads off the
//! replay as an identity in the variables. A `+` depth stands outward,
//! a `−` depth runs into the body along the wall.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, len};
use crate::intent_s6_c_trace::{BOSS_H, RADIUS, Scene, boss_steps, eval, face, max_at};
use crate::intent_s6_d_tangency::{by_its_heading, tangency};
use editor_core::coincide::{Proof, Recourse};
use editor_core::pose::{PoseDef, RoleRead};
use editor_core::{
    BodyRead, CancelToken, CapEnd, Dimension, DocEdit, EvalOptions, Formula, FreeValue, FreeVar,
    LoopProgram, Node, OnFace, Operand, ProfileDoc, ProfileProgram, RecipeNodeId, StableName,
    VarDecl, VarName,
};
use geom::SurfaceKind;
use geom_core::Tol;

/// The cylinder's height.
const HEIGHT: f64 = 1.0;

/// A document holding the free variable `cx` at `centre`, and a cylinder
/// of radius `RADIUS` and height `HEIGHT` whose disc is centred at
/// `(cx, 0)`: the document, the cylinder.
fn cylinder_at(centre: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("s6e_cylinder", Tol::witness());
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::new("cx").expect("a name"),
            def: VarDecl::Free(FreeVar::continuous(Dimension::Length, centre)),
        },
    );
    let (doc, disc) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![disc_at(cx())],
            ids: Vec::new(),
            on: None,
        }),
    );
    extruded(doc, disc)
}

/// The cylinder's centre, read by name.
fn cx() -> Formula {
    Formula::named(VarName::new("cx").expect("a name"), Dimension::Length)
}

/// A disc of radius `RADIUS` centred at `(x, 0)`.
fn disc_at(x: Formula) -> LoopProgram<Formula> {
    LoopProgram::Circle {
        centre: [x, len(0.0)],
        radius: len(RADIUS),
    }
}

/// `disc` extruded `HEIGHT`.
fn extruded(doc: ProfileDoc, disc: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    fixture::insert(
        doc,
        Node::Extrude {
            profile: disc.into(),
            distance: len(HEIGHT),
        },
    )
}

/// `body`'s entity `name`, as a selection.
fn pick(body: Operand, name: StableName) -> Operand {
    Operand::select(body, vec![name])
}

/// The cylinder's cap at its `end`.
fn cap(cylinder: RecipeNodeId, end: CapEnd) -> StableName {
    fixture::fname(cylinder, editor_core::RoleSeg::Cap(end))
}

/// `body`'s cap at its `end`, read off its evaluation: its own, or on a
/// copy its source's.
fn cap_of(ev: &editor_core::Evaluation<f64>, body: RecipeNodeId, end: CapEnd) -> StableName {
    let want = cap(body, end);
    editor_core::all_faces(ev, body)
        .into_iter()
        .find(|n| n.path == want.path)
        .unwrap_or_else(|| panic!("a {end:?} cap"))
}

/// **The boss the traced chain draws on `face` of `body`, tracing
/// `traced`**, slid by `slides` and extruded `depth`: drawn through the
/// door, at the chart's nearest orientation and no spin.
fn boss_on_face(
    doc: ProfileDoc,
    (body, face): (BodyRead<Formula>, StableName),
    traced: Operand,
    slides: [Formula; 2],
    depth: f64,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let drawn = editor_core::draw_on_face(
        &doc,
        body,
        face,
        vec![LoopProgram::Chain(boss_steps())],
        OnFace {
            slides,
            traced: vec![traced],
            ..OnFace::nearest()
        },
        &EvalOptions::default(),
        &CancelToken::new(),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the door draws on the face: {e}"));
    let (doc, boss) = fixture::insert(
        drawn.doc,
        Node::Extrude {
            profile: drawn.profile.into(),
            distance: len(depth),
        },
    );
    (doc, drawn.profile, boss)
}

/// The boss drawn on `cylinder`'s cap at `end`, tracing its wall, slid
/// by `slides` and extruded `depth`.
fn boss(
    doc: ProfileDoc,
    cylinder: RecipeNodeId,
    end: CapEnd,
    slides: [Formula; 2],
    depth: f64,
) -> Scene {
    let ev = eval(&doc);
    let wall = face(&ev, cylinder, SurfaceKind::Cylinder);
    let body = Operand::output(cylinder, 0);
    let (doc, footprint, boss) = boss_on_face(
        doc,
        (BodyRead::plain(body.clone()), cap_of(&ev, cylinder, end)),
        pick(body, wall),
        slides,
        depth,
    );
    Scene {
        doc,
        cylinder,
        footprint,
        boss,
    }
}

/// No slide.
fn unslid() -> [Formula; 2] {
    [len(0.0), len(0.0)]
}

/// `node`'s least coordinate `axis`, where it stands in its space.
fn min_at(doc: &ProfileDoc, node: RecipeNodeId, axis: usize) -> f64 {
    let ev = eval(doc);
    let body = match &ev.value(node).expect("the node evaluated").payload {
        editor_core::ValuePayload::Body(b) => fixture::in_space(b),
        other => panic!("expected a body, got {other:?}"),
    };
    body.vertices()
        .map(|(v, _)| {
            let p = topo::readback::vertex_point(&body, v).expect("a live vertex");
            [p.x, p.y, p.z][axis]
        })
        .fold(f64::INFINITY, f64::min)
}

fn near(a: f64, b: f64, what: &str) {
    assert!((a - b).abs() <= 1e-12, "{what}: {a} against {b}");
}

/// The union's tangency, proven by its heading.
fn assert_proven(scene: &Scene, what: &str) {
    let proof = tangency(
        scene.doc.clone(),
        vec![
            Operand::output(scene.cylinder, 0),
            Operand::output(scene.boss, 0),
        ],
    );
    assert!(by_its_heading(&proof), "{what}: {proof:?}");
}

/// **On the bottom cap a `+` depth stands outward, below the base, and a
/// `−` depth runs up along the wall**: the chart's `z` is the cap's
/// outward normal, `−z`, and its `y` the space's `−y`, so the boss's
/// wall stands at `y = −r`. Each is tangent to the wall by construction.
#[test]
fn a_boss_on_the_bottom_cap_stands_outward_and_its_pocket_runs_up_the_wall() {
    let (doc, cylinder) = cylinder_at(0.0);
    let outward = boss(doc.clone(), cylinder, CapEnd::Start, unslid(), BOSS_H);
    near(
        min_at(&outward.doc, outward.boss, 1),
        -RADIUS,
        "the wall at -r",
    );
    near(
        max_at(&eval(&outward.doc), outward.boss, 2),
        0.0,
        "on the base",
    );
    near(min_at(&outward.doc, outward.boss, 2), -BOSS_H, "outward");
    assert_proven(&outward, "outward of the bottom cap");
    let inward = boss(doc, cylinder, CapEnd::Start, unslid(), -BOSS_H);
    near(min_at(&inward.doc, inward.boss, 2), 0.0, "from the base");
    near(
        max_at(&eval(&inward.doc), inward.boss, 2),
        BOSS_H,
        "up the wall",
    );
    assert_proven(&inward, "into the bottom cap");
}

/// **On the top cap the boss stands on the top**, and is tangent by
/// construction there too.
#[test]
fn a_boss_on_the_top_cap_stands_on_the_top() {
    let (doc, cylinder) = cylinder_at(0.0);
    let scene = boss(doc, cylinder, CapEnd::End, unslid(), BOSS_H);
    near(min_at(&scene.doc, scene.boss, 2), HEIGHT, "on the top cap");
    near(
        max_at(&eval(&scene.doc), scene.boss, 1),
        RADIUS,
        "the wall at the radius",
    );
    assert_proven(&scene, "the top cap");
}

/// **A cylinder off its body's origin keeps its boss through an edit of
/// its centre**: the chart slides along `x` by the centre's own variable,
/// so the boss moves with the disc and stays tangent.
#[test]
fn an_offset_cylinder_keeps_its_boss_through_a_centre_edit() {
    let (doc, cylinder) = cylinder_at(0.3);
    let scene = boss(doc, cylinder, CapEnd::Start, [cx(), len(0.0)], BOSS_H);
    near(
        min_at(&scene.doc, scene.boss, 0),
        0.3,
        "the boss starts on the axis",
    );
    assert_proven(&scene, "off the origin");
    let (doc, _) = fixture::step(
        scene.doc,
        DocEdit::SetVarValue {
            var: VarName::new("cx").expect("a name").into(),
            value: FreeValue::Continuous(0.45),
        },
    );
    let scene = Scene { doc, ..scene };
    near(
        min_at(&scene.doc, scene.boss, 0),
        0.45,
        "the boss moved with the disc",
    );
    near(
        min_at(&scene.doc, scene.boss, 1),
        -RADIUS,
        "its wall at the radius",
    );
    assert_proven(&scene, "after the edit");
}

/// **A cylinder tilted by a variable keeps its boss's tangency as an
/// identity**: the cylinder placed turned about `x` by the free angle
/// `a`, so its cap's normal and its wall's axis are both functions of
/// `a`, and the chart turns the face's normal by the least turn. The
/// door reads the wall's axis along the sketch's `z` in the symbols.
#[test]
fn a_tilted_cylinders_boss_is_tangent_for_every_tilt() {
    let (doc, a) = with_angle("a", 0.2);
    let (doc, base) = slab(doc, [-3.0, -2.0], [-0.5, 0.5], 0.5);
    let (doc, cylinder) = turned_cylinder(doc, base, a, ([0.0, 0.0], 0.0));
    // Pinned red: the replay does not see the heading through the
    // placement (`work/doctail/the-tangency-prover-does-not-see-through-a-placement.md`).
    test_utils::pinned_red::row(
        Tol::witness().eps(),
        &test_utils::pinned_red::EVERY_ROW,
        "the placed tilted cylinder's boss",
        &[
            "tilted by a: Unproven",
            "maps: [World(",
            "recourse: ReadTheAxis",
        ],
        || {
            let scene = boss(doc, cylinder, CapEnd::End, unslid(), BOSS_H);
            assert_proven(&scene, "tilted by a");
        },
    );
}

/// A document holding the free angle `name` at `value`, and its read.
fn with_angle(name: &str, value: f64) -> (ProfileDoc, Formula) {
    let doc = ProfileDoc::empty_derived("s6e_turned", Tol::witness());
    let name = VarName::new(name).expect("a name");
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: name.clone(),
            def: VarDecl::Free(FreeVar::continuous(Dimension::Angle, value)),
        },
    );
    (doc, Formula::named(name, Dimension::Angle))
}

/// A box over `x` by `y` in its own `xy`, extruded `h` up its `z`.
fn slab(doc: ProfileDoc, x: [f64; 2], y: [f64; 2], h: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, outline) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::polygon_expr([
                [len(x[0]), len(y[0])],
                [len(x[1]), len(y[0])],
                [len(x[1]), len(y[1])],
                [len(x[0]), len(y[1])],
            ])],
            ids: Vec::new(),
            on: None,
        }),
    );
    extruded_by(doc, outline, h)
}

/// `outline` extruded `h`.
fn extruded_by(doc: ProfileDoc, outline: RecipeNodeId, h: f64) -> (ProfileDoc, RecipeNodeId) {
    fixture::insert(
        doc,
        Node::Extrude {
            profile: outline.into(),
            distance: len(h),
        },
    )
}

/// **A cylinder of radius `RADIUS` and height `HEIGHT`, its disc
/// centred at `centre` on its own `xy`, placed into the space of `base`
/// turned about that space's `x` by `by` and slid `lift` up its `z`**:
/// the document and the placed copy, which a placement by values turns
/// by nothing but `by`.
fn turned_cylinder(
    doc: ProfileDoc,
    base: RecipeNodeId,
    by: Formula,
    (centre, lift): ([f64; 2], f64),
) -> (ProfileDoc, RecipeNodeId) {
    use editor_core::{Coord, TargetAxis};
    let (doc, disc) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(centre[0]), len(centre[1])],
                radius: len(RADIUS),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, upright) = extruded(doc, disc);
    let space = fixture::space(&doc, base);
    let placed = editor_core::Placed {
        mates: Vec::new(),
        values: vec![
            (Coord::Spin(Some(TargetAxis::X)), by),
            (Coord::Spin(Some(TargetAxis::Y)), fixture::ang(0.0)),
            (Coord::Spin(Some(TargetAxis::Z)), fixture::ang(0.0)),
            (Coord::slide(TargetAxis::X), len(0.0)),
            (Coord::slide(TargetAxis::Y), len(0.0)),
            (Coord::slide(TargetAxis::Z), len(lift)),
        ],
    };
    fixture::insert(
        doc,
        Node::place(
            BodyRead::plain(Operand::from(upright)),
            Operand::Var(space),
            placed.bundle(),
        ),
    )
}

/// The union of `a` and `b`'s one tangency, as the door proves it.
fn tangency_of(doc: ProfileDoc, a: RecipeNodeId, b: RecipeNodeId) -> Proof {
    tangency(doc, vec![Operand::output(a, 0), Operand::output(b, 0)])
}

/// Unproven, naming where to draw.
fn asks_for_the_axis(proof: &Proof) -> bool {
    matches!(
        proof,
        Proof::Unproven {
            recourse: Recourse::ReadTheAxis,
            ..
        }
    )
}

/// The boss the traced chain draws on `face` of `body`, tracing the
/// wall of `cylinder`, slid by `slides` and extruded `depth`.
fn boss_tracing(
    doc: ProfileDoc,
    (body, face): (RecipeNodeId, StableName),
    cylinder: RecipeNodeId,
    slides: [Formula; 2],
    depth: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let wall = face_of_kind(&doc, cylinder, SurfaceKind::Cylinder);
    let (doc, _, boss) = boss_on_face(
        doc,
        (BodyRead::plain(Operand::output(body, 0)), face),
        pick(Operand::output(cylinder, 0), wall),
        slides,
        depth,
    );
    (doc, boss)
}

/// `node`'s face on a carrier of `kind`, in `doc`'s evaluation.
fn face_of_kind(doc: &ProfileDoc, node: RecipeNodeId, kind: SurfaceKind) -> StableName {
    face(&eval(doc), node, kind)
}

/// **A wall parallel to the face it is traced from by value alone asks
/// for the axis**: a cylinder spun about `x` by a free angle `a` at `0`,
/// traced from the chart of a box's top beside it. At `a = 0` the wall
/// stands square to the box's top, so the boss is tangent, but the wall's
/// axis leaves the sketch's `z` by `sin a`: no identity, and the finding
/// names where to draw.
#[test]
fn a_hole_parallel_to_the_face_by_value_only_asks_for_the_axis() {
    let (doc, a) = with_angle("a", 0.0);
    let (doc, block) = slab(doc, [-3.0, -2.0], [-0.5, 0.5], 0.5);
    let (doc, cylinder) = turned_cylinder(doc, block, a, ([0.0, 0.0], 0.0));
    let (doc, boss) = boss_tracing(
        doc,
        (block, cap(block, CapEnd::End)),
        cylinder,
        unslid(),
        BOSS_H,
    );
    let proof = tangency_of(doc, cylinder, boss);
    assert!(asks_for_the_axis(&proof), "{proof:?}");
}

/// **A turn written in radians tilts the wall by its rounding; a frame
/// read off the wall's axis does not.** A horizontal cylinder, its
/// disc's frame turned by the float `π/2` about `x`, traced from the chart
/// of a box's side square to it: its axis leaves the box's normal by the
/// rounding of `cos(π/2)`, so the boss is tangent within the band but not
/// as an identity, and the finding names where to draw. Drawn on a frame
/// whose `z` is read off the wall's axis, the same boss proves by the
/// first rung, whatever the turn's rounding.
#[test]
fn a_frame_turned_in_radians_asks_for_the_axis_read_off_the_face() {
    let doc = ProfileDoc::empty_derived("s6e_radians", Tol::witness());
    let (doc, block) = slab(doc, [-3.0, -2.0], [-0.5, 0.0], 2.0);
    let (doc, cylinder) = turned_cylinder(
        doc,
        block,
        fixture::ang(core::f64::consts::FRAC_PI_2),
        ([0.0, 0.0], 1.0),
    );
    let ev = eval(&doc);
    let side = editor_core::all_faces(&ev, block)
        .into_iter()
        .find(|name| {
            editor_core::face_frame(&ev, block, name)
                .is_ok_and(|pose| (pose.origin.y + 0.5).abs() < 1e-12 && pose.axis.y.abs() > 0.5)
        })
        .expect("the box's side at y = -1/2");
    // The chart's origin, the space's projected onto the side, slid up its
    // `z` onto the cylinder's axis.
    let (on_the_side, boss) = boss_tracing(
        doc.clone(),
        (block, side),
        cylinder,
        [len(0.0), len(1.0)],
        BOSS_H,
    );
    let radians = tangency_of(on_the_side, cylinder, boss);
    assert!(
        asks_for_the_axis(&radians),
        "on the box's side: {radians:?}"
    );
    let wall = face_of_kind(&doc, cylinder, SurfaceKind::Cylinder);
    let vertex = editor_core::all_vertices(&ev, cylinder)
        .into_iter()
        .next()
        .expect("a vertex");
    let read = |def| RoleRead::plain(Operand::Pose(Box::new(def)));
    let off_the_axis = Operand::Pose(Box::new(PoseDef::Through {
        axis: read(PoseDef::Axis {
            of: pick(Operand::output(cylinder, 0), wall),
        }),
        point: read(PoseDef::Point {
            of: pick(Operand::output(cylinder, 0), vertex),
        }),
    }));
    let scene = crate::intent_s6_c_trace::boss_on(doc, cylinder, off_the_axis, boss_steps());
    let read_off = tangency_of(scene.doc, cylinder, scene.boss);
    assert!(
        crate::intent_s6_d_tangency::proven(&read_off),
        "on a frame read off the wall's axis: {read_off:?}"
    );
}

/// **A boss on a box mated flat onto the cylinder's top cap is tangent
/// for every tilt** (DESIGN `[placement]`: "or a face mated flat onto
/// that cap"): the cylinder tilted by a free `a` at `0.2`, a thin box
/// placed on its top cap by the plane mate a sketch on a face writes, and
/// a boss drawn on the box's top tracing the wall, run down past the box
/// along the wall.
#[test]
fn a_boss_on_a_box_mated_flat_onto_the_cap_is_tangent_for_every_tilt() {
    let (doc, a) = with_angle("a", 0.2);
    let (doc, block) = slab(doc, [-3.0, -2.0], [-0.5, 0.5], 0.5);
    let (doc, cylinder) = turned_cylinder(doc, block, a, ([0.0, 0.0], 0.0));
    let placed = editor_core::sketch_on_face(
        &doc,
        BodyRead::plain(Operand::output(cylinder, 0)),
        cap_of(&eval(&doc), cylinder, CapEnd::End),
        ProfileProgram {
            loops: vec![LoopProgram::polygon_expr([
                [len(-1.5), len(-1.5)],
                [len(1.5), len(-1.5)],
                [len(1.5), len(1.5)],
                [len(-1.5), len(1.5)],
            ])],
            ids: Vec::new(),
            on: None,
        },
        len(0.2),
        fixture::ang(0.0),
        None,
        &EvalOptions::default(),
        &CancelToken::new(),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the box is placed on the cap: {e}"));
    let box_top = cap(placed.sketched.extrude, CapEnd::End);
    let (doc, boss) = boss_tracing(
        placed.doc,
        (placed.sketched.placement, box_top),
        cylinder,
        unslid(),
        -0.8,
    );
    // Pinned red: the replay does not see the heading through the
    // placement (`work/doctail/the-tangency-prover-does-not-see-through-a-placement.md`).
    test_utils::pinned_red::row(
        Tol::witness().eps(),
        &test_utils::pinned_red::EVERY_ROW,
        "the boss on the box mated onto the placed cylinder's cap",
        &["Unproven", "maps: [World(", "recourse: ReadTheAxis"],
        || {
            let proof = tangency_of(doc, cylinder, boss);
            assert!(by_its_heading(&proof), "{proof:?}");
        },
    );
}

/// **A boss traced on a hole drilled through the same chart proves**:
/// the hole a disc drawn on a box's top cap's chart, slid along `x` by
/// the free `cx`, and pocketed through; the boss drawn on the same chart
/// with the same slide, tracing the hole's wall (whose material is
/// outside its circle, so the chain enters it and runs with it) and run
/// down into the box.
#[test]
fn a_boss_traced_on_a_hole_drilled_through_the_same_chart_proves() {
    let doc = ProfileDoc::empty_derived("s6e_hole", Tol::witness());
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::new("cx").expect("a name"),
            def: VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.1)),
        },
    );
    let (doc, block) = slab(doc, [-2.0, 2.0], [-2.0, 2.0], 1.0);
    let top = || {
        (
            BodyRead::plain(Operand::output(block, 0)),
            cap(block, CapEnd::End),
        )
    };
    let on = |traced: Vec<Operand>| OnFace {
        slides: [cx(), len(0.0)],
        traced,
        ..OnFace::nearest()
    };
    let drill = editor_core::draw_on_face(
        &doc,
        top().0,
        top().1,
        vec![disc_at(len(0.0))],
        on(Vec::new()),
        &EvalOptions::default(),
        &CancelToken::new(),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the hole's disc is drawn: {e}"));
    let (doc, tool) = extruded_by(drill.doc, drill.profile, -1.0);
    let (doc, drilled) = fixture::insert(
        doc,
        Node::Subtract {
            from: Operand::from(block).into(),
            tool: Operand::from(tool).into(),
        },
    );
    let hole = face_of_kind(&doc, drilled, SurfaceKind::Cylinder);
    let steps = boss_steps()
        .into_iter()
        .map(|step| match step {
            editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::OnTrace {
                face, ..
            }) => editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::OnTrace {
                face,
                crossing: editor_core::TraceCrossing::first(editor_core::TracePassage::Enter),
            }),
            editor_core::ProgramStep::AlongTrace { .. } => editor_core::ProgramStep::AlongTrace {
                sense: editor_core::TraceSense::With,
            },
            step => step,
        })
        .collect();
    let drawn = editor_core::draw_on_face(
        &doc,
        top().0,
        top().1,
        vec![LoopProgram::Chain(steps)],
        on(vec![pick(Operand::output(drilled, 0), hole)]),
        &EvalOptions::default(),
        &CancelToken::new(),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the boss is drawn: {e}"));
    let (doc, boss) = extruded_by(drawn.doc, drawn.profile, BOSS_H);
    let proof = tangency_of(doc, drilled, boss);
    assert!(by_its_heading(&proof), "{proof:?}");
}

/// **A chart on a member the family does not have refuses typed**: the
/// cylinder patterned three times, a profile drawn on `xs[3]`'s bottom
/// cap.
#[test]
fn a_chart_on_a_missing_member_refuses_typed() {
    let (doc, cylinder) = cylinder_at(0.0);
    let (doc, _k, xs) = fixture::linear(doc, cylinder, [1.0, 0.0, 0.0], 2.0, 3);
    let missing = BodyRead::indexed(Operand::output(xs, 0), vec![Formula::count(3)]);
    let drawn = editor_core::draw_on_face(
        &doc,
        missing,
        cap(cylinder, CapEnd::Start),
        vec![disc_at(len(0.0))],
        OnFace::nearest(),
        &EvalOptions::default(),
        &CancelToken::new(),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the door draws: {e}"));
    let ev = eval(&drawn.doc);
    let refusal = ev.node_error(drawn.profile).map(|e| &e.kind);
    assert!(
        matches!(refusal, Some(editor_core::NodeErrorKind::MemberOutOfRange { input, at }) if *input == xs && at.as_slice() == [3]),
        "no member 3: {refusal:?}"
    );
}

/// **A face that is no plane has no chart**: a profile drawn on the
/// cylinder's wall refuses typed where it is read.
#[test]
fn a_profile_on_a_face_that_is_no_plane_refuses_typed() {
    let (doc, cylinder) = cylinder_at(0.0);
    let wall = face(&eval(&doc), cylinder, SurfaceKind::Cylinder);
    let body = Operand::output(cylinder, 0);
    let (doc, footprint, _) = boss_on_face(
        doc,
        (BodyRead::plain(body.clone()), wall.clone()),
        pick(body, wall),
        unslid(),
        BOSS_H,
    );
    let ev = eval(&doc);
    let refusal = ev.node_error(footprint).map(|e| &e.kind);
    assert!(
        matches!(refusal, Some(editor_core::NodeErrorKind::PoseRead { .. })),
        "the wall has no chart: {refusal:?}"
    );
}

/// **A profile on one member of a family, `xs[i]`, traces that member**
/// (REFERENCES DM3). The cylinder patterned three times along `x` at
/// spacing 2: drawn on member 2's bottom cap, slid along `x` to its axis
/// and tracing member 2's wall, the boss starts at `x = 4` and its wall
/// stands at member 2's radius. Read as the family (its first member),
/// the trace would miss: the line up from `x = 4` crosses no wall of
/// member 0.
#[test]
fn a_profile_on_a_member_traces_that_member() {
    let (doc, cylinder) = cylinder_at(0.0);
    let (doc, _k, xs) = fixture::linear(doc, cylinder, [1.0, 0.0, 0.0], 2.0, 3);
    let ev = eval(&doc);
    let wall = face(&ev, cylinder, SurfaceKind::Cylinder);
    let member = || BodyRead::indexed(Operand::output(xs, 0), vec![Formula::count(2)]);
    let (doc, footprint, boss) = boss_on_face(
        doc.clone(),
        (member(), cap(cylinder, CapEnd::Start)),
        Operand::select(member(), vec![wall]),
        [len(4.0), len(0.0)],
        BOSS_H,
    );
    let ev = eval(&doc);
    assert!(
        ev.node_error(footprint).is_none() && ev.node_error(boss).is_none(),
        "the traced boss builds: {:?}",
        ev.node_error(footprint)
            .or_else(|| ev.node_error(boss))
            .map(|e| e.kind.to_string())
    );
    near(
        min_at(&doc, boss, 0),
        4.0,
        "the boss starts on member 2's axis",
    );
    near(
        min_at(&doc, boss, 1),
        -RADIUS,
        "its wall at member 2's radius",
    );
}

/// **A face's chart reads a trace before anything is drawn on it**
/// ([`editor_core::chart_on_face`]): a probe profile on the bottom cap's
/// chart, lowered into a scratch copy of the document, reads the wall
/// against the document's own evaluation, its spin and slides bound as
/// the run binds them. The wall is the cylinder of the radius, its axis
/// the sketch's `z`, and the chart is the one the door writes.
#[test]
fn a_faces_chart_reads_a_trace_before_anything_is_drawn_there() {
    let (doc, cylinder) = cylinder_at(0.0);
    let ev = eval(&doc);
    let wall = face(&ev, cylinder, SurfaceKind::Cylinder);
    let body = BodyRead::plain(Operand::output(cylinder, 0));
    let bottom = cap(cylinder, CapEnd::Start);
    assert_eq!(
        editor_core::nearest_to_face(&doc, &ev, &body, &bottom).ok(),
        editor_core::Orientation::new(
            editor_core::SignedAxis::PlusX,
            editor_core::SignedAxis::MinusZ
        ),
        "the nearest orientation puts z down the cap's normal"
    );
    let frame = editor_core::chart_on_face(&doc, Some(&ev), body, bottom, &OnFace::nearest());
    let mut scratch = doc.clone();
    let probe = Node::Profile(ProfileProgram {
        loops: Vec::new(),
        ids: Vec::new(),
        on: Some(editor_core::DrawnOn {
            frame,
            faces: vec![pick(Operand::output(cylinder, 0), wall)],
        }),
    });
    let Node::Profile(program) =
        editor_core::edit::lower_node_into(&mut scratch, &probe).expect("the probe lowers")
    else {
        unreachable!("a profile lowers to a profile")
    };
    let faces = editor_core::traced_faces(&scratch, &ev, &program, Tol::witness())
        .unwrap_or_else(|e| panic!("the chart and the wall read: {e}"));
    let [traced] = faces.as_slice() else {
        panic!("one traced face: {}", faces.len())
    };
    let Ok(geom::Surface::Cylinder { frame, radius, .. }) = &traced.surface else {
        panic!("the wall is a cylinder: {:?}", traced.surface)
    };
    let axis = frame.axis();
    near(*radius, RADIUS, "the wall's radius");
    let z = traced.body_to_sketch.transform_vec(axis);
    assert!(
        z.x.abs() < 1e-12 && z.y.abs() < 1e-12,
        "the wall's axis is the sketch's z: {z:?}"
    );
}

/// **A boss on a cylinder tilted by a variable at zero, tracing an upright
/// cylinder it is coaxial with only there, is not proved tangent**: the
/// tilted cylinder placed by values (`a` at `0`), an upright one sketched
/// on the box's top cap on the same axis, and a boss drawn on the tilted
/// cylinder's top cap tracing the upright wall, unioned with it. At `a = 0`
/// the boss is tangent; at any other `a` the tilted cap's chart leans with
/// the cylinder and the boss runs off the upright wall. So no identity
/// holds, and the door must not read the chart's least turn as the
/// identity off the nominal, where the cap's normal is the space's `z`
/// in value alone.
#[test]
fn a_boss_traced_across_two_cylinders_coaxial_only_at_zero_tilt_is_not_proved() {
    let (doc, a) = with_angle("a", 0.0);
    let (doc, block) = slab(doc, [-3.0, -2.0], [-0.5, 0.5], 0.5);
    let (doc, tilted) = turned_cylinder(doc, block, a, ([0.0, 0.0], 0.0));
    let placed = editor_core::sketch_on_face(
        &doc,
        BodyRead::plain(Operand::output(block, 0)),
        cap(block, CapEnd::End),
        ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius: len(RADIUS),
            }],
            ids: Vec::new(),
            on: None,
        },
        len(2.0),
        fixture::ang(0.0),
        None,
        &EvalOptions::default(),
        &CancelToken::new(),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the upright cylinder is placed on the box: {e}"));
    let upright = placed.sketched.placement;
    let tilted_cap = cap_of(&eval(&placed.doc), tilted, CapEnd::End);
    let (doc, traced) = boss_tracing(placed.doc, (tilted, tilted_cap), upright, unslid(), BOSS_H);
    let proof = tangency_of(doc, upright, traced);
    assert!(
        matches!(proof, Proof::Unproven { .. }),
        "tangent at a = 0 alone: {proof:?}"
    );
}
