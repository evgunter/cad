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
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
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
    let wall = face(&eval(&doc), cylinder, SurfaceKind::Cylinder);
    let body = Operand::output(cylinder, 0);
    let (doc, footprint, boss) = boss_on_face(
        doc,
        (BodyRead::plain(body.clone()), cap(cylinder, end)),
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
/// identity**: the disc drawn on a frame spun about `x` by the free
/// angle `a`, so its cap's normal and its wall's axis are both functions
/// of `a`, and the chart turns the face's normal by the least turn. The
/// door reads the wall's axis along the sketch's `z` in the symbols. The
/// boss is on the top cap, a height off the space's origin: a face
/// through it levers the chart's turn over nothing
/// (`work/doctail/a-face-charts-turn-is-levered-over-none-of-what-it-carries.md`).
#[test]
fn a_tilted_cylinders_boss_is_tangent_for_every_tilt() {
    let doc = ProfileDoc::empty_derived("s6e_tilted", Tol::witness());
    let a = VarName::new("a").expect("a name");
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: a.clone(),
            def: VarDecl::Free(FreeVar::continuous(Dimension::Angle, 0.2)),
        },
    );
    let (doc, base) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![disc_at(len(-2.0))],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, base) = extruded(doc, base);
    let at_origin = editor_core::test_support::frame_on(
        &doc,
        &Operand::output(base, 0),
        &editor_core::Frame::IDENTITY,
        Tol::witness(),
    );
    let tilted = Operand::Pose(Box::new(PoseDef::Spin {
        pose: RoleRead::plain(at_origin.clone()),
        about: RoleRead::through(at_origin, PoseRole::X),
        by: Formula::named(a, Dimension::Angle),
    }));
    let (doc, disc) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![disc_at(len(0.0))],
            ids: Vec::new(),
            on: Some(editor_core::DrawnOn {
                frame: tilted,
                faces: Vec::new(),
            }),
        }),
    );
    let (doc, cylinder) = extruded(doc, disc);
    let scene = boss(doc, cylinder, CapEnd::End, unslid(), BOSS_H);
    assert_proven(&scene, "tilted by a");
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
/// ([`editor_core::face_chart`], [`editor_core::nearest_to_face`]): a
/// probe profile on the bottom cap's chart, lowered into a scratch copy
/// of the document, reads the wall against the document's own
/// evaluation, its spin and slides bound as the run binds them. The
/// wall is the cylinder of the radius, its axis the sketch's `z`.
#[test]
fn a_faces_chart_reads_a_trace_before_anything_is_drawn_there() {
    let (doc, cylinder) = cylinder_at(0.0);
    let ev = eval(&doc);
    let wall = face(&ev, cylinder, SurfaceKind::Cylinder);
    let body = BodyRead::plain(Operand::output(cylinder, 0));
    let bottom = cap(cylinder, CapEnd::Start);
    let orient = editor_core::nearest_to_face(&doc, &ev, &body, &bottom);
    let zero = OnFace::nearest();
    let frame = editor_core::face_chart(body, bottom, (orient, zero.spin, zero.slides));
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
    let Ok(geom::Surface::Cylinder { axis, radius, .. }) = &traced.surface else {
        panic!("the wall is a cylinder: {:?}", traced.surface)
    };
    near(*radius, RADIUS, "the wall's radius");
    let z = traced.body_to_sketch.transform_vec(*axis);
    assert!(
        z.x.abs() < 1e-12 && z.y.abs() < 1e-12,
        "the wall's axis is the sketch's z: {z:?}"
    );
}
