//! **INTENT stage 6 PR E — the frame a sketch tracing a face stands on**
//! (`docs/INTENT-STAGE6-SPEC.md` §6; `editor_core::traced_frame`): `Through`
//! the traced face's own axis and a vertex of a picked plane face square
//! to it. A boss drawn on it, leaving the wall's trace along its tangent
//! and swept up the axis, is tangent to the wall by construction, in
//! whichever cap's plane it is drawn, and stays so through an edit of
//! the cylinder's centre: the frame holds no number read off the
//! evaluation. A traced face with no axis, or a picked face that is no
//! plane, refuses typed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, len};
use crate::intent_s6_c_trace::{BOSS_H, RADIUS, Scene, boss_on, boss_steps, eval, face, max_at};
use crate::intent_s6_d_tangency::{proven, tangency};
use editor_core::{
    Dimension, DocEdit, Formula, FreeValue, FreeVar, LoopProgram, Node, Operand, ProfileDoc,
    ProfileProgram, RecipeNodeId, StableName, TracedFrameFault, UnitSym, VarDecl, VarName,
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
            def: VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: centre,
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
                radius: len(RADIUS),
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

/// `cylinder`'s face named `name`, as a selection.
fn pick(cylinder: RecipeNodeId, name: StableName) -> Operand {
    Operand::select(Operand::output(cylinder, 0), vec![name])
}

/// The cylinder's cap at its `end`.
fn cap(cylinder: RecipeNodeId, end: editor_core::CapEnd) -> StableName {
    StableName {
        kind: editor_core::EntityKind::Face,
        node: cylinder,
        path: vec![editor_core::RoleSeg::Cap(end)],
    }
}

/// The frame tracing the wall in the plane of `plane`.
fn frame(
    doc: &ProfileDoc,
    cylinder: RecipeNodeId,
    plane: StableName,
) -> Result<Operand, TracedFrameFault> {
    let wall = face(&eval(doc), cylinder, SurfaceKind::Cylinder);
    editor_core::traced_frame(
        doc,
        &eval(doc),
        &pick(cylinder, plane),
        &pick(cylinder, wall),
        Tol::witness(),
    )
}

/// The boss drawn on the frame in the plane of `plane`, extruded
/// `BOSS_H`.
fn boss(doc: ProfileDoc, cylinder: RecipeNodeId, plane: StableName) -> Scene {
    let frame = frame(&doc, cylinder, plane).expect("a frame reads off the cap and the wall");
    boss_on(doc, cylinder, frame, boss_steps())
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

/// The union's tangency, proven by construction.
fn assert_proven(scene: &Scene, what: &str) {
    let proof = tangency(
        scene.doc.clone(),
        vec![
            Operand::output(scene.cylinder, 0),
            Operand::output(scene.boss, 0),
        ],
    );
    assert!(proven(&proof), "{what}: {proof:?}");
}

/// **On the bottom cap's plane the boss stands on the base, its wall at
/// the radius, tangent by construction.**
#[test]
fn a_boss_in_the_bottom_caps_plane_is_tangent_by_construction() {
    let (doc, cylinder) = cylinder_at(0.0);
    let scene = boss(doc, cylinder, cap(cylinder, editor_core::CapEnd::Start));
    let ev = eval(&scene.doc);
    near(max_at(&ev, scene.boss, 1), RADIUS, "the wall at the radius");
    near(min_at(&scene.doc, scene.boss, 2), 0.0, "on the base");
    near(max_at(&ev, scene.boss, 2), BOSS_H, "its height");
    assert_proven(&scene, "the bottom cap");
}

/// **On the top cap's plane the boss stands on the top**, and is
/// tangent by construction there too.
#[test]
fn a_boss_in_the_top_caps_plane_stands_on_the_top() {
    let (doc, cylinder) = cylinder_at(0.0);
    let scene = boss(doc, cylinder, cap(cylinder, editor_core::CapEnd::End));
    near(min_at(&scene.doc, scene.boss, 2), HEIGHT, "on the top cap");
    near(
        max_at(&eval(&scene.doc), scene.boss, 1),
        RADIUS,
        "the wall at the radius",
    );
    assert_proven(&scene, "the top cap");
}

/// **A cylinder off its body's origin keeps its boss through an edit of
/// its centre**: the frame is read off the wall's axis and the cap's
/// vertex at every evaluation, so the boss moves with the disc and stays
/// tangent.
#[test]
fn an_offset_cylinder_keeps_its_boss_through_a_centre_edit() {
    let (doc, cylinder) = cylinder_at(0.3);
    let scene = boss(doc, cylinder, cap(cylinder, editor_core::CapEnd::Start));
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
        max_at(&eval(&scene.doc), scene.boss, 1),
        RADIUS,
        "its wall at the radius",
    );
    assert_proven(&scene, "after the edit");
}

/// **A traced face with no axis, and a picked face that is no plane,
/// refuse typed.**
#[test]
fn a_frame_with_no_axis_or_no_plane_refuses_typed() {
    let (doc, cylinder) = cylinder_at(0.0);
    let ev = eval(&doc);
    let bottom = cap(cylinder, editor_core::CapEnd::Start);
    let wall = face(&ev, cylinder, SurfaceKind::Cylinder);
    let read = |plane: &StableName, traced: &StableName| {
        editor_core::traced_frame(
            &doc,
            &ev,
            &pick(cylinder, plane.clone()),
            &pick(cylinder, traced.clone()),
            Tol::witness(),
        )
    };
    assert!(
        matches!(read(&bottom, &bottom), Err(TracedFrameFault::Read(_))),
        "a cap has no axis"
    );
    assert!(
        matches!(
            read(&wall, &wall),
            Err(TracedFrameFault::NotAPlane {
                carrier: SurfaceKind::Cylinder
            })
        ),
        "the wall is no plane"
    );
}
