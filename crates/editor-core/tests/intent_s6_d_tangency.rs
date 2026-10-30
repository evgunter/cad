//! **INTENT stage 6 PR D — the door proves a constructed tangency
//! along a swept curve** (`docs/INTENT-STAGE6-SPEC.md` §5, as ruled on
//! PR #6006; DESIGN `[coincidence.structural]`): a wall swept from a
//! piece that leaves a face's trace along its tangent, by a slide its
//! construction states, is tangent to the face by construction.
//!
//! The statement reads three things, and each has scenes tangent only
//! by value that break it alone and stay unproven: the piece's heading
//! (typed along the tangent, or a leg not on it), the extrude's heading
//! (a frame read off a rim, or spun about an axis off its `z`), and the
//! face's construction (a cylinder revolved, or a twin of the one
//! traced). Each frame move the heading is walked through has a scene
//! that proves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, ang, len, len2, scl};
use crate::intent_s6_c_trace::{
    BOSS_H, RADIUS, boss_on, boss_steps, cylinder as standing, eval, face, max_at,
};
use editor_core::coincide::{self, Proof, Recourse, Rung};
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CancelToken, Dimension, DocEdit, DrawnOn, Formula, FreeVar, LoopProgram, Node, Operand,
    ProfileDoc, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, StableName,
    TraceCrossing, TracePassage, TraceSense, UnitSym, VarDecl, VarName, all_edges, all_faces,
    all_vertices, edge_carrier_kind, face_carrier_kind, vertex_position,
};
use geom::{CurveKind, SurfaceKind};
use geom_core::Tol;

/// A pose read off `body`'s entity `name`.
fn read_off(body: RecipeNodeId, name: StableName) -> Operand {
    Operand::select(Operand::output(body, 0), vec![name])
}

/// `def` as an operand.
fn pose(def: PoseDef<Operand, Formula>) -> Operand {
    Operand::Pose(Box::new(def))
}

/// A plain read of `of`.
fn plain(of: Operand) -> RoleRead<Operand> {
    RoleRead::plain(of)
}

/// The axis read off `body`'s entity `name`.
fn axis_off(body: RecipeNodeId, name: StableName) -> RoleRead<Operand> {
    plain(pose(PoseDef::Axis {
        of: read_off(body, name),
    }))
}

/// **The frame through the axis read off `body`'s `axis`, towards its
/// vertex at `(r, 0, 0)`** in its own coordinates: on a cylinder
/// standing on its xy plane, the cylinder's own axes.
fn frame_off(doc: &ProfileDoc, body: RecipeNodeId, axis: StableName) -> Operand {
    let ev = eval(doc);
    let seam = all_vertices(&ev, body)
        .into_iter()
        .find(|v| {
            let p = vertex_position(&ev, body, v).expect("a vertex");
            (p.x - RADIUS).abs() < 1e-12 && p.y.abs() < 1e-12 && p.z.abs() < 1e-12
        })
        .expect("a vertex at (r, 0, 0)");
    pose(PoseDef::Through {
        axis: axis_off(body, axis),
        point: plain(pose(PoseDef::Point {
            of: read_off(body, seam),
        })),
    })
}

/// The cylinder's wall: its first face on a cylinder.
fn wall(doc: &ProfileDoc, body: RecipeNodeId) -> StableName {
    face(&eval(doc), body, SurfaceKind::Cylinder)
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

/// A profile drawn on `frame` tracing `traced`, extruded `BOSS_H`: the
/// document and the boss.
fn boss_tracing(
    doc: ProfileDoc,
    frame: Operand,
    traced: Operand,
    steps: Vec<ProgramStep<Formula>>,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, footprint) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(steps)],
            ids: Vec::new(),
            on: Some(DrawnOn {
                frame,
                faces: vec![traced],
            }),
        }),
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: footprint.into(),
            distance: len(BOSS_H),
        },
    )
}

/// **The union of `members`' one tangency and the door's proof of it.**
fn tangency(doc: ProfileDoc, members: Vec<Operand>) -> Proof {
    let (doc, union) = fixture::insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(members.into_iter().map(Into::into).collect()),
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

/// **The boss C draws, on `frame` of a standing cylinder, unioned with
/// it**: the tangency's proof. `frame` is given the document and the
/// cylinder.
fn on_the_cylinder(
    frame: impl FnOnce(&ProfileDoc, RecipeNodeId) -> Operand,
    steps: Vec<ProgramStep<Formula>>,
) -> Proof {
    let (doc, cylinder) = standing(RADIUS, None);
    let frame = frame(&doc, cylinder);
    let scene = boss_on(doc, cylinder, frame, steps);
    tangency(
        scene.doc,
        vec![
            Operand::output(scene.cylinder, 0),
            Operand::output(scene.boss, 0),
        ],
    )
}

/// The frame read off the cylinder's wall's axis.
fn off_the_wall(doc: &ProfileDoc, cylinder: RecipeNodeId) -> Operand {
    frame_off(doc, cylinder, wall(doc, cylinder))
}

fn proven(proof: &Proof) -> bool {
    *proof == Proof::Structural(Rung::SameConstruction)
}

fn unproven(proof: &Proof) -> bool {
    matches!(proof, Proof::Unproven { .. })
}

fn asks_for_the_axis(proof: &Proof) -> bool {
    matches!(
        proof,
        Proof::Unproven {
            recourse: Recourse::ReadTheAxis,
            ..
        }
    )
}

/// **A boss traced along the cylinder's wall and extruded up its axis
/// is tangent to it by construction**: the piece leaves the wall's
/// trace along its tangent, the extrude's heading is the axis read off
/// the wall, and the wall is an extrude's, which slides along it.
#[test]
fn a_boss_traced_off_the_walls_axis_is_tangent_by_construction() {
    let (doc, cylinder) = standing(RADIUS, None);
    let frame = off_the_wall(&doc, cylinder);
    let scene = boss_on(doc, cylinder, frame, boss_steps());
    let ev = eval(&scene.doc);
    assert!(
        (max_at(&ev, scene.boss, 2) - BOSS_H).abs() < 1e-12,
        "the boss stands on the base, extruded up the axis"
    );
    let proof = tangency(
        scene.doc,
        vec![
            Operand::output(scene.cylinder, 0),
            Operand::output(scene.boss, 0),
        ],
    );
    assert!(proven(&proof), "{proof:?}");
}

/// **The extrude's heading through each frame move that keeps it**: a
/// slide along the wall's axis, a spin about the frame's own `z`, and a
/// flip (the frame's `z` turned) slid up to the cylinder's top, from
/// where the boss extrudes down: each proves.
#[test]
fn a_frame_slid_spun_or_flipped_along_the_axis_still_proves() {
    let slid = on_the_cylinder(
        |doc, cylinder| {
            pose(PoseDef::Slide {
                pose: plain(off_the_wall(doc, cylinder)),
                along: axis_off(cylinder, wall(doc, cylinder)),
                by: len(0.2),
            })
        },
        boss_steps(),
    );
    assert!(proven(&slid), "slid along the axis: {slid:?}");
    let spun = on_the_cylinder(
        |doc, cylinder| {
            let frame = off_the_wall(doc, cylinder);
            pose(PoseDef::Spin {
                pose: plain(frame.clone()),
                about: RoleRead::through(frame, PoseRole::Z),
                by: ang(0.3),
            })
        },
        boss_steps(),
    );
    assert!(proven(&spun), "spun about its own z: {spun:?}");
    let flipped = on_the_cylinder(
        |doc, cylinder| {
            let flip = pose(PoseDef::Flip {
                pose: plain(off_the_wall(doc, cylinder)),
            });
            pose(PoseDef::Slide {
                pose: plain(flip.clone()),
                along: RoleRead::through(flip, PoseRole::Z),
                by: len(-1.0),
            })
        },
        boss_steps(),
    );
    assert!(proven(&flipped), "flipped and slid to the top: {flipped:?}");
}

/// **A trace of one half of a circle's wall reads its carrier**: the
/// cylinder's wall is two faces, one carrier; a boss tracing the half
/// its crossing does not lie on glues to the other, and is proven
/// against it.
#[test]
fn a_boss_traced_on_the_walls_other_half_proves() {
    let (doc, cylinder) = standing(RADIUS, None);
    let ev = eval(&doc);
    let halves: Vec<_> = all_faces(&ev, cylinder)
        .into_iter()
        .filter(|n| face_carrier_kind(&ev, cylinder, n) == Ok(SurfaceKind::Cylinder))
        .collect();
    assert_eq!(halves.len(), 2, "the wall is two faces: {halves:?}");
    let first = off_the_wall(&doc, cylinder);
    for traced in halves {
        let (doc, boss) = boss_tracing(
            doc.clone(),
            first.clone(),
            read_off(cylinder, traced.clone()),
            boss_steps(),
        );
        let proof = tangency(
            doc,
            vec![Operand::output(cylinder, 0), Operand::output(boss, 0)],
        );
        assert!(proven(&proof), "tracing {traced:?}: {proof:?}");
    }
}

/// **The step's equality**: the same boss with its heading typed `+x`
/// (the trace's tangent at today's radius) rather than read off the
/// trace is not drawn along it.
#[test]
fn a_heading_typed_along_the_tangent_is_unproven() {
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
    let proof = on_the_cylinder(off_the_wall, steps);
    assert!(unproven(&proof), "the heading is typed: {proof:?}");
}

/// **A leg that departs along the trace's tangent as an arc**: the
/// tangent arc after the trace's tangent is a circle tangent to the
/// trace where it leaves it, so its wall is tangent to the cylinder's by
/// construction too.
#[test]
fn a_tangent_arc_leaving_the_trace_proves() {
    let steps = boss_steps()
        .into_iter()
        .map(|s| match s {
            ProgramStep::LineTo(ProgramTarget::AtX(_)) => {
                ProgramStep::TangentArcTo(ProgramTarget::Point(len2([1.0, 1.0])))
            }
            s => s,
        })
        .collect();
    let proof = on_the_cylinder(off_the_wall, steps);
    assert!(proven(&proof), "{proof:?}");
}

/// **The pose row**: the frame's `z` read off the bottom rim. The rim's
/// axis is the wall's for every radius, but no statement the door reads
/// says so; the finding asks for the axis read off the face (C's scene,
/// `intent_s6_c_trace`, is the same case).
#[test]
fn a_frame_read_off_a_rim_asks_for_the_walls_axis() {
    let proof = on_the_cylinder(
        |doc, cylinder| frame_off(doc, cylinder, bottom_rim(doc, cylinder)),
        boss_steps(),
    );
    assert!(asks_for_the_axis(&proof), "{proof:?}");
}

/// **A spin about an axis off the frame's `z`** turns its heading, at
/// any angle but today's `0`: spun about its own `x`, or about the rim's
/// axis (the wall's only by what the door does not read), the boss is
/// tangent by value alone.
#[test]
fn a_frame_spun_about_another_axis_is_unproven() {
    for (what, about) in [
        (
            "its own x",
            Box::new(|doc: &ProfileDoc, cylinder| {
                RoleRead::through(off_the_wall(doc, cylinder), PoseRole::X)
            }) as Box<dyn Fn(&ProfileDoc, RecipeNodeId) -> RoleRead<Operand>>,
        ),
        (
            "the rim's axis",
            Box::new(|doc: &ProfileDoc, cylinder| axis_off(cylinder, bottom_rim(doc, cylinder))),
        ),
    ] {
        let proof = on_the_cylinder(
            |doc, cylinder| {
                pose(PoseDef::Spin {
                    pose: plain(off_the_wall(doc, cylinder)),
                    about: about(doc, cylinder),
                    by: ang(0.0),
                })
            },
            boss_steps(),
        );
        assert!(asks_for_the_axis(&proof), "spun about {what}: {proof:?}");
    }
}

/// **The traced face is the one glued**: a boss traced on a cylinder
/// and run on to its mirror image (across `x = 3/2`, in the cylinder's
/// space) is tangent to the image's wall by value alone.
#[test]
fn a_boss_traced_on_one_cylinder_is_unproven_against_its_image() {
    const FAR: f64 = 3.0;
    let (doc, cylinder) = standing(RADIUS, None);
    let plane = editor_core::test_support::plane_on_frame(
        &doc,
        Operand::output(cylinder, 0),
        [1.5, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        Tol::witness(),
    );
    let (doc, image) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(cylinder, 0).into(),
            plane,
        },
    );
    let steps = boss_steps()
        .into_iter()
        .map(|s| match s {
            ProgramStep::LineTo(ProgramTarget::AtX(_)) => {
                ProgramStep::LineTo(ProgramTarget::AtX(len(FAR)))
            }
            ProgramStep::LineTo(ProgramTarget::Point(_)) => {
                ProgramStep::LineTo(ProgramTarget::Point(len2([FAR, -0.25])))
            }
            s => s,
        })
        .collect();
    let frame = off_the_wall(&doc, cylinder);
    let traced = read_off(cylinder, wall(&doc, cylinder));
    let (doc, boss) = boss_tracing(doc, frame, traced, steps);
    let proof = tangency(
        doc,
        vec![Operand::output(image, 0), Operand::output(boss, 0)],
    );
    assert!(
        unproven(&proof) && !asks_for_the_axis(&proof),
        "the image is not traced: {proof:?}"
    );
}

/// **The face's construction**: a band revolved from a quadrilateral
/// whose outer side leans by `t`, a cylinder at today's `t = 0` and a
/// cone at any other, holds no slide along its axis by construction; the
/// boss extruded along that axis is tangent by value alone.
#[test]
fn a_boss_on_a_revolved_band_is_unproven() {
    let doc = ProfileDoc::empty_derived("s6d_revolved", Tol::witness());
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::new("t").expect("a name"),
            def: VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: 0.0,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: None,
            }),
        },
    );
    let t = Formula::named(VarName::new("t").expect("a name"), Dimension::Length);
    let point = |x: Formula, y: f64| ProgramTarget::Point([x, len(y)]);
    let (doc, half) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At(len2([0.0, 0.0])),
                ProgramStep::LineTo(point(len(RADIUS), 0.0)),
                ProgramStep::LineTo(point(Formula::add(len(RADIUS), t).expect("a length"), 1.0)),
                ProgramStep::LineTo(point(len(0.0), 1.0)),
                ProgramStep::LineTo(ProgramTarget::Start),
            ])],
            ids: Vec::new(),
            on: None,
        }),
    );
    // About the profile's `y`: the band stands along its own `y`.
    let (doc, band) = fixture::insert(
        doc,
        fixture::revolve_about(
            half,
            (0.0, 0.0),
            fixture::along_y(),
            fixture::ang(std::f64::consts::TAU),
        ),
    );
    let ev = eval(&doc);
    let side = face(&ev, band, SurfaceKind::Cylinder);
    let base = all_vertices(&ev, band)
        .into_iter()
        .find(|v| {
            let p = vertex_position(&ev, band, v).expect("a vertex");
            p.y.abs() < 1e-12 && (p.x.hypot(p.z) - RADIUS).abs() < 1e-12
        })
        .expect("a rim vertex on the base");
    let frame = pose(PoseDef::Through {
        axis: axis_off(band, side.clone()),
        point: plain(pose(PoseDef::Point {
            of: read_off(band, base),
        })),
    });
    let (doc, boss) = boss_tracing(doc, frame, read_off(band, side), boss_steps());
    let proof = tangency(
        doc,
        vec![Operand::output(band, 0), Operand::output(boss, 0)],
    );
    assert!(
        unproven(&proof) && !asks_for_the_axis(&proof),
        "a revolved band's construction holds no slide: {proof:?}"
    );
}

/// **The slice caveat**: a boss drawn on a plane across a sphere off
/// its centre traces a small circle, and extruded normal to the plane
/// it cuts the sphere. No tangency is proven there: a sphere is a
/// revolve's band, which holds no slide. Red if a union that builds
/// proves one, or if this one refuses otherwise.
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
    let shell = face(&ev, sphere, SurfaceKind::Sphere);
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
    let (doc, boss) = boss_tracing(doc, frame, read_off(sphere, shell), steps);
    let (doc, union) = fixture::insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                Operand::output(sphere, 0).into(),
                Operand::output(boss, 0).into(),
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
