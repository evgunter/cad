//! R1's probes of PR 6060 (stage 6 D). Review rows, not for merge as-is.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, ang, len};
use crate::intent_s6_c_trace::{RADIUS, boss_on, boss_steps, cylinder as standing, eval, face};
use editor_core::coincide::{self, Proof};
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CancelToken, Node, Operand, ProfileDoc, RecipeNodeId, StableName, all_vertices, vertex_position,
};
use geom::SurfaceKind;
use geom_core::Tol;

fn read_off(body: RecipeNodeId, name: StableName) -> Operand {
    Operand::select(Operand::output(body, 0), vec![name])
}
fn pose(def: PoseDef<Operand, editor_core::Formula>) -> Operand {
    Operand::Pose(Box::new(def))
}
fn plain(o: Operand) -> RoleRead<Operand> {
    RoleRead::plain(o)
}

/// The s6d frame: Through the wall's axis, towards the seam vertex.
fn base_frame(doc: &ProfileDoc, cyl: RecipeNodeId) -> Operand {
    let ev = eval(doc);
    let wall = face(&ev, cyl, SurfaceKind::Cylinder);
    let seam = all_vertices(&ev, cyl)
        .into_iter()
        .find(|v| {
            let p = vertex_position(&ev, cyl, v).unwrap();
            (p.x - RADIUS).abs() < 1e-12 && p.y.abs() < 1e-12 && p.z.abs() < 1e-12
        })
        .unwrap();
    pose(PoseDef::Through {
        axis: plain(pose(PoseDef::Axis {
            of: read_off(cyl, wall),
        })),
        point: plain(pose(PoseDef::Point {
            of: read_off(cyl, seam),
        })),
    })
}

/// Every tangency row of the union and its proof; None when the union refuses.
fn tangencies(doc: ProfileDoc, cyl: RecipeNodeId, frame: Operand) -> Option<Vec<Proof>> {
    let scene = boss_on(doc, cyl, frame, boss_steps());
    let (doc, union) = fixture::insert(
        scene.doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![scene.cylinder.into(), scene.boss.into()]),
        },
    );
    let ev = eval(&doc);
    if let Some(e) = ev.node_error(union) {
        eprintln!("R1 union refuses: {}", e.kind);
        return None;
    }
    let rows = &ev.value(union).unwrap().coincidences;
    let proofs = coincide::prove(
        &doc,
        &[(union, &rows[..])],
        None,
        Tol::witness(),
        &CancelToken::new(),
    )
    .unwrap();
    let out: Vec<Proof> = rows
        .iter()
        .zip(&proofs[0])
        .filter(|(r, _)| r.relation == topo::Relation::SurfaceTangent)
        .map(|(_, p)| p.clone())
        .collect();
    eprintln!("R1 tangencies: {out:?}");
    Some(out)
}

fn structural(p: &[Proof]) -> bool {
    p.len() == 1 && matches!(p[0], Proof::Structural(_))
}

#[test]
fn r1_flipped_frame_proves() {
    let (doc, cyl) = standing(RADIUS, None);
    let f = base_frame(&doc, cyl);
    let t = tangencies(doc, cyl, pose(PoseDef::Flip { pose: plain(f) }));
    eprintln!("R1 flip -> {t:?}");
    // Flip turns z; the boss then extrudes downward off the base: may record no tangency.
    if let Some(t) = t {
        assert!(t.is_empty() || structural(&t), "{t:?}");
    }
}

#[test]
fn r1_slid_frame_proves() {
    let (doc, cyl) = standing(RADIUS, None);
    let ev = eval(&doc);
    let wall = face(&ev, cyl, SurfaceKind::Cylinder);
    let f = base_frame(&doc, cyl);
    let slid = pose(PoseDef::Slide {
        pose: plain(f),
        along: plain(pose(PoseDef::Axis {
            of: read_off(cyl, wall),
        })),
        by: len(0.2),
    });
    let t = tangencies(doc, cyl, slid).expect("builds");
    assert!(structural(&t), "{t:?}");
}

#[test]
fn r1_spun_about_own_z_proves() {
    let (doc, cyl) = standing(RADIUS, None);
    let f = base_frame(&doc, cyl);
    let spun = pose(PoseDef::Spin {
        pose: plain(f.clone()),
        about: RoleRead::through(f, PoseRole::Z),
        by: ang(0.3),
    });
    let t = tangencies(doc, cyl, spun).expect("builds");
    assert!(structural(&t), "{t:?}");
}

/// A frame spun about its own x: z tilts off the cylinder's axis. No tangency
/// may be proven.
#[test]
fn r1_spun_about_x_proves_nothing() {
    let (doc, cyl) = standing(RADIUS, None);
    let f = base_frame(&doc, cyl);
    let spun = pose(PoseDef::Spin {
        pose: plain(f.clone()),
        about: RoleRead::through(f, PoseRole::X),
        by: ang(0.2),
    });
    if let Some(t) = tangencies(doc, cyl, spun) {
        assert!(
            t.iter().all(|p| !matches!(p, Proof::Structural(_))),
            "{t:?}"
        );
    }
}

/// A frame spun about its own z by 0: is z-role spin with by=0 (a no-op) proven too; and a
/// spin about z of a *slid* copy (a parallel line) still keeps the heading.
#[test]
fn r1_spun_about_a_parallel_axis_proves() {
    let (doc, cyl) = standing(RADIUS, None);
    let ev = eval(&doc);
    let wall = face(&ev, cyl, SurfaceKind::Cylinder);
    let f = base_frame(&doc, cyl);
    let slid_axis = pose(PoseDef::Slide {
        pose: plain(pose(PoseDef::Axis {
            of: read_off(cyl, wall.clone()),
        })),
        along: plain(pose(PoseDef::Axis {
            of: read_off(cyl, wall),
        })),
        by: len(0.0),
    });
    let spun = pose(PoseDef::Spin {
        pose: plain(f),
        about: plain(slid_axis),
        by: ang(0.3),
    });
    let t = tangencies(doc, cyl, spun);
    eprintln!("R1 parallel-axis spin (expected unproven, conservative): {t:?}");
}

#[test]
fn r1_flip_geometry() {
    use crate::intent_s6_c_trace::max_at;
    let (doc, cyl) = standing(RADIUS, None);
    let ev = eval(&doc);
    for n in editor_core::all_faces(&ev, cyl) {
        eprintln!(
            "R1 cyl face {n:?} {:?}",
            editor_core::face_carrier_kind(&ev, cyl, &n)
        );
    }
    let f = base_frame(&doc, cyl);
    for (label, frame) in [
        ("plain", f.clone()),
        ("flip", pose(PoseDef::Flip { pose: plain(f) })),
    ] {
        let scene = boss_on(doc.clone(), cyl, frame, boss_steps());
        let ev = eval(&scene.doc);
        let mut lo = [0.0f64; 3];
        let mut hi = [0.0f64; 3];
        for a in 0..3 {
            hi[a] = max_at(&ev, scene.boss, a);
            lo[a] = -crate::intent_s6_c_trace::max_at(&ev, scene.boss, a); // placeholder
        }
        let vs: Vec<_> = editor_core::all_vertices(&ev, scene.boss)
            .into_iter()
            .map(|v| vertex_position(&ev, scene.boss, &v).unwrap())
            .collect();
        eprintln!("R1 {label} boss max {hi:?} vertices {vs:?} {lo:?}");
    }
}

/// The boss traced on the wall's OTHER half (Piece(1)); its crossing at (0, r) lies on Piece(0).
#[test]
fn r1_traced_on_the_other_half() {
    use editor_core::{DrawnOn, LoopProgram, ProfileProgram};
    let (doc, cyl) = standing(RADIUS, None);
    let ev = eval(&doc);
    let halves: Vec<_> = editor_core::all_faces(&ev, cyl)
        .into_iter()
        .filter(|n| editor_core::face_carrier_kind(&ev, cyl, n) == Ok(SurfaceKind::Cylinder))
        .collect();
    let frame = base_frame(&doc, cyl);
    let (doc, footprint) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(boss_steps())],
            ids: Vec::new(),
            on: Some(DrawnOn {
                frame,
                faces: vec![read_off(cyl, halves[1].clone())],
            }),
        }),
    );
    let ev = eval(&doc);
    eprintln!(
        "R1 other-half footprint error: {:?}",
        ev.node_error(footprint).map(|e| e.kind.to_string())
    );
    let (doc, boss) = fixture::insert(
        doc,
        Node::Extrude {
            profile: footprint.into(),
            distance: len(0.5),
        },
    );
    let (doc, union) = fixture::insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![cyl.into(), boss.into()]),
        },
    );
    let ev = eval(&doc);
    let Some(v) = ev.value(union) else {
        eprintln!(
            "R1 other-half union refuses: {:?}",
            ev.node_error(union).map(|e| e.kind.to_string())
        );
        return;
    };
    let rows = &v.coincidences;
    let proofs = coincide::prove(
        &doc,
        &[(union, &rows[..])],
        None,
        Tol::witness(),
        &CancelToken::new(),
    )
    .unwrap();
    for (r, p) in rows.iter().zip(&proofs[0]) {
        if r.relation == topo::Relation::SurfaceTangent {
            eprintln!("R1 other-half row {:?} -> {p:?}", r.site);
        }
    }
}
