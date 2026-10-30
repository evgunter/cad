//! **INTENT stage 6 PR D — the door proves a constructed tangency
//! along a swept curve** (`docs/INTENT-STAGE6-SPEC.md` §5, as ruled on
//! PR #6006; DESIGN `[coincidence.structural]`): a wall swept from a
//! piece that leaves a face's trace along its tangent, by a motion in
//! that face's symmetry, is tangent to the face by construction.
//!
//! Each of the statement's three reads has a scene that breaks it alone
//! and stays unproven: the heading typed equal to the trace's tangent
//! (the step's equality), the frame read off a rim instead of the wall
//! (the pose row), and a cylinder revolved rather than extruded (the
//! symmetry).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, len, len2, scl};
use crate::intent_s6_c_trace::{
    BOSS_H, RADIUS, Scene, boss_on, boss_steps, cylinder as standing, eval, face, max_at,
};
use editor_core::coincide::{self, Proof, Recourse, Rung};
use editor_core::{
    CancelToken, DrawnOn, LoopProgram, Node, Operand, ProfileDoc, ProfileProgram, ProgramStep,
    ProgramTarget, RecipeNodeId, StableName, TraceCrossing, TracePassage, TraceSense, all_edges,
    all_vertices, edge_carrier_kind, vertex_position,
};
use geom::{CurveKind, SurfaceKind};
use geom_core::Tol;

/// A pose read off `body`'s entity `name`.
fn read_off(body: RecipeNodeId, name: StableName) -> Operand {
    Operand::select(Operand::output(body, 0), vec![name])
}

/// **The frame read off a cylinder's wall**: `z` the wall's own axis,
/// `x` towards its seam vertex at `(r, 0, 0)`, so it stands at the
/// cylinder's origin with its axes. `axis` names what `z` is read off:
/// the wall, or (for the pose row's control) its bottom rim.
fn frame_off(doc: &ProfileDoc, body: RecipeNodeId, axis: StableName) -> Operand {
    use editor_core::pose::{PoseDef, RoleRead};
    let ev = eval(doc);
    let seam = all_vertices(&ev, body)
        .into_iter()
        .find(|v| {
            let p = vertex_position(&ev, body, v).expect("a vertex");
            (p.x - RADIUS).abs() < 1e-12 && p.y.abs() < 1e-12 && p.z.abs() < 1e-12
        })
        .expect("a seam vertex at (r, 0, 0)");
    let read = |def| RoleRead::plain(Operand::Pose(Box::new(def)));
    Operand::Pose(Box::new(PoseDef::Through {
        axis: read(PoseDef::Axis {
            of: read_off(body, axis),
        }),
        point: read(PoseDef::Point {
            of: read_off(body, seam),
        }),
    }))
}

/// The cylinder's bottom rim: a circle at `z = 0`.
fn bottom_rim(doc: &ProfileDoc, body: RecipeNodeId) -> StableName {
    let ev = eval(doc);
    all_edges(&ev, body)
        .into_iter()
        .find(|e| {
            edge_carrier_kind(&ev, body, e) == Ok(CurveKind::Circle)
                && editor_core::edge_frame(&ev, body, e).is_ok_and(|f| f.origin.z.abs() < 1e-12)
        })
        .expect("a bottom rim")
}

/// A boss traced on `cylinder`'s wall from a frame read off `axis`.
fn boss(doc: ProfileDoc, cylinder: RecipeNodeId, axis: Option<StableName>) -> Scene {
    let ev = eval(&doc);
    let wall = face(&ev, cylinder, SurfaceKind::Cylinder);
    let frame = frame_off(&doc, cylinder, axis.unwrap_or(wall));
    boss_on(doc, cylinder, frame, boss_steps())
}

/// **The union's one tangency and the door's proof of it**, the boss
/// standing on the cylinder's base as the scene draws it.
fn tangency(scene: Scene) -> Proof {
    let ev = eval(&scene.doc);
    assert!(
        (max_at(&ev, scene.boss, 2) - BOSS_H).abs() < 1e-12,
        "the boss stands on the base, extruded up the axis"
    );
    let (doc, union) = fixture::insert(
        scene.doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![scene.cylinder.into(), scene.boss.into()]),
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
    proof.clone()
}

fn unproven(proof: &Proof) -> bool {
    matches!(proof, Proof::Unproven { .. })
}

/// **A boss traced along the cylinder's wall and extruded up its axis
/// is tangent to it by construction**: the piece leaves the wall's
/// trace along its tangent, the extrude's heading is the axis read off
/// the wall, and the wall's symmetry slides along it.
#[test]
fn a_boss_traced_off_the_walls_axis_is_tangent_by_construction() {
    let (doc, cylinder) = standing(RADIUS, None);
    let proof = tangency(boss(doc, cylinder, None));
    assert_eq!(proof, Proof::Structural(Rung::SameConstruction));
}

/// **The step's equality**: the same boss with its heading typed `+x`
/// (the trace's tangent at today's radius) rather than read off the
/// trace is not drawn along it.
#[test]
fn a_heading_typed_along_the_tangent_is_unproven() {
    let (doc, cylinder) = standing(RADIUS, None);
    let steps = boss_steps()
        .into_iter()
        .map(|s| match s {
            ProgramStep::AlongTrace { .. } => ProgramStep::Toward {
                dx: scl(1.0),
                dy: scl(0.0),
            },
            s => s,
        })
        .collect();
    let ev = eval(&doc);
    let wall = face(&ev, cylinder, SurfaceKind::Cylinder);
    let frame = frame_off(&doc, cylinder, wall);
    let proof = tangency(boss_on(doc, cylinder, frame, steps));
    assert!(unproven(&proof), "the heading is typed: {proof:?}");
}

/// **The pose row**: the frame's `z` read off the bottom rim is the
/// wall's axis only at today's values, so the extrude is not one `Axis`
/// with the wall's; the finding asks for the axis read off the face
/// (C's scene, `intent_s6_c_trace`, is the same case).
#[test]
fn a_frame_read_off_a_rim_is_unproven() {
    let (doc, cylinder) = standing(RADIUS, None);
    let rim = bottom_rim(&doc, cylinder);
    let proof = tangency(boss(doc, cylinder, Some(rim)));
    assert!(
        matches!(
            proof,
            Proof::Unproven {
                recourse: Recourse::ReadTheAxis,
                ..
            }
        ),
        "the frame is read off the rim: {proof:?}"
    );
}

/// **The symmetry**: a cylinder revolved from a rectangle is a band
/// about the revolve's axis, whose construction holds the spin about it
/// and no slide along it; the boss extruded along that axis is not in
/// its symmetry by construction.
#[test]
fn a_boss_on_a_revolved_cylinder_is_unproven() {
    let doc = ProfileDoc::empty_derived("s6d_revolved", Tol::witness());
    let (doc, half) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(vec![vec![
            (0.0, 0.0),
            (RADIUS, 0.0),
            (RADIUS, 1.0),
            (0.0, 1.0),
        ]])),
    );
    // About the profile's `y`, so the cylinder stands along its own `y`;
    // the frame reads its axis off the wall, so the boss is drawn across
    // it as on the extruded one.
    let (doc, cylinder) = fixture::insert(
        doc,
        fixture::revolve_about(
            half,
            (0.0, 0.0),
            fixture::along_y(),
            fixture::ang(std::f64::consts::TAU),
        ),
    );
    let ev = eval(&doc);
    let wall = face(&ev, cylinder, SurfaceKind::Cylinder);
    let seam = all_vertices(&ev, cylinder)
        .into_iter()
        .find(|v| {
            let p = vertex_position(&ev, cylinder, v).expect("a vertex");
            p.y.abs() < 1e-12 && (p.x.hypot(p.z) - RADIUS).abs() < 1e-12
        })
        .expect("a rim vertex on the base");
    let frame = {
        use editor_core::pose::{PoseDef, RoleRead};
        let read = |def| RoleRead::plain(Operand::Pose(Box::new(def)));
        Operand::Pose(Box::new(PoseDef::Through {
            axis: read(PoseDef::Axis {
                of: read_off(cylinder, wall.clone()),
            }),
            point: read(PoseDef::Point {
                of: read_off(cylinder, seam),
            }),
        }))
    };
    let (doc, footprint) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(boss_steps())],
            ids: Vec::new(),
            on: Some(DrawnOn {
                frame,
                faces: vec![read_off(cylinder, wall)],
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
    let (doc, union) = fixture::insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                Operand::output(cylinder, 0).into(),
                boss.into(),
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
    assert!(
        unproven(tangent[0].1),
        "a revolved band's construction holds no slide: {:?}",
        tangent[0].1
    );
}

/// **The slice caveat**: a boss drawn on a plane across a sphere off
/// its centre traces a small circle, and extruded normal to the plane
/// it cuts the sphere. No tangency is proven there: a revolve's band
/// holds no slide. Red if a union that builds proves one, or if this one
/// refuses otherwise.
#[test]
fn a_boss_on_a_spheres_small_circle_is_not_proven_tangent() {
    const R: f64 = 1.25;
    let doc = ProfileDoc::empty_derived("s6d_slice", Tol::witness());
    let (doc, half) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At(len2([0.0, -R])),
                ProgramStep::ArcTo(editor_core::ProgramArcData::Bulge {
                    target: ProgramTarget::Point(len2([0.0, R])),
                    b: scl(1.0),
                }),
                ProgramStep::LineTo(ProgramTarget::Start),
            ])],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, sphere) = fixture::insert(
        doc,
        fixture::revolve_about(
            half,
            (0.0, 0.0),
            fixture::along_y(),
            fixture::ang(std::f64::consts::TAU),
        ),
    );
    let ev = eval(&doc);
    let wall = face(&ev, sphere, SurfaceKind::Sphere);
    let lifted = editor_core::Frame {
        translation: [0.0, 0.0, 0.5],
        ..editor_core::Frame::IDENTITY
    };
    let frame = editor_core::test_support::frame_on(
        &doc,
        &Operand::output(sphere, 0),
        &lifted,
        Tol::witness(),
    );
    let steps = vec![
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
        ProgramStep::LineTo(ProgramTarget::AtX(len(2.0))),
        ProgramStep::LineTo(ProgramTarget::Point(len2([2.0, -0.25]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ];
    let (doc, footprint) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(steps)],
            ids: Vec::new(),
            on: Some(DrawnOn {
                frame,
                faces: vec![read_off(sphere, wall)],
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
    let (doc, union) = fixture::insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                Operand::output(sphere, 0).into(),
                boss.into(),
            ]),
        },
    );
    // The union refuses today: the boss's edges pierce the sphere, and
    // the boolean has no arm to settle it. A refused union records no
    // tangency; once it builds, none it records may be proven.
    let ev = eval(&doc);
    match ev.value(union) {
        None => assert!(
            matches!(
                ev.node_error(union).map(|e| &e.kind),
                Some(editor_core::NodeErrorKind::Boolean(e))
                    if e.kind() == topo::BooleanErrorKind::CurvedPierceUnsupported
            ),
            "the slice union refuses on the pierce: {:?}",
            ev.node_error(union).map(|e| e.kind.to_string())
        ),
        Some(value) => {
            let rows = &value.coincidences;
            let proofs = coincide::prove(
                &doc,
                &[(union, &rows[..])],
                None,
                Tol::witness(),
                &CancelToken::new(),
            )
            .expect("an uncanceled door");
            assert!(
                rows.iter().zip(&proofs[0]).all(|(row, proof)| {
                    row.relation != topo::Relation::SurfaceTangent || unproven(proof)
                }),
                "no tangency is proven where the extrude cuts the sphere: {rows:?}"
            );
        }
    }
}
