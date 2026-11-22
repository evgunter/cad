//! **A profile drawn on a picked face, at the viewer**
//! (`SessionOp::DrawOnFace`): drawn on the face's chart at its nearest
//! orientation, no spin and no slide, then extruded off the face and
//! combined with the face's body by the ordinary ops that follow.
//!
//! The face is a PICK, and what it is read through is the node whose
//! body the ray met — never the feature that minted the face: a flat
//! swept by an extrude and shrunk by a later chamfer is drawn on the
//! CHAMFER's body, and reading it through the extrude would name a face
//! of a different size.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use common::{inserted, len};

use pncad::document::{Doc, Node, Operand, ProfileProgram, RecipeNodeId};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName, SurfaceKind, attribute};
use pncad::select::{all_faces, face_carrier_kind};
use viewer::session::{BooleanSpec, DocSession, FaceSelection, NodeKindWanted, Refusal, SessionOp};

/// The box's side: 20 mm square, 10 mm tall.
const SIDE: f64 = 0.02;
/// The box's height.
const HEIGHT: f64 = 0.01;
/// The boss drawn on the box: a 5 mm square in its chart's first
/// quadrant, 4 mm tall.
const BOSS: [f64; 3] = [0.005, 0.005, 0.004];

/// A 20 mm square box, 10 mm tall, through the document door and
/// placed in the world — the body every row here picks a face on.
fn boxed(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let doc = Doc::empty_derived("drawn-on-face", tol);
    let (doc, profile) = inserted(&doc, common::square(SIDE), tol);
    let (doc, cube) = inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(HEIGHT),
        },
        tol,
    );
    let (doc, _) = common::placed(&doc, cube, tol);
    (doc, cube)
}

/// The box's top cap, picked on the box itself.
fn top_of(cube: RecipeNodeId) -> FaceSelection {
    FaceSelection {
        name: StableName {
            kind: EntityKind::Face,
            node: cube,
            path: vec![RoleSeg::Cap(CapEnd::End)],
        },
        node: cube,
        body: 0,
    }
}

/// The boss's footprint, in its chart's first quadrant.
fn footprint() -> pncad::trace::Chain {
    common::rectangle_chain([0.0, 0.0], BOSS[0], BOSS[1])
}

/// [`footprint`] drawn on `face`.
fn footprint_on(face: FaceSelection) -> SessionOp {
    SessionOp::DrawOnFace {
        face,
        traced: Vec::new(),
        chains: vec![footprint()],
    }
}

/// [`footprint`] drawn on `face`, then extruded [`BOSS`]'s height off it
/// ([`common::drawn_boss`]): the profile and the boss.
fn boss_on(session: &mut DocSession, face: FaceSelection) -> (RecipeNodeId, RecipeNodeId) {
    common::drawn_boss(session, face, footprint(), BOSS[2])
}

/// `body` and `boss` combined: the union.
fn combined(session: &mut DocSession, body: Operand, boss: RecipeNodeId) -> RecipeNodeId {
    common::session_insert(
        session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![body.into(), boss.into()]),
        },
    )
}

/// **Drawing on a face, extruding and combining writes no placement**
/// (Ev, PR 6353): the boss drawn on the box's top, picked on the box
/// itself, extruded and combined with the box, leaves the world holding
/// the box's one placement; the union offers, as its own step, to
/// re-point that placement onto it.
#[test]
fn a_boss_drawn_extruded_and_combined_writes_no_placement() {
    let tol = Tol::witness();
    let (doc, cube) = boxed(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let placed = common::world(session.committed_doc());
    let (_, boss) = boss_on(&mut session, top_of(cube));
    let union = combined(&mut session, Operand::from(cube), boss);
    let doc = session.committed_doc();
    assert_eq!(placed, [cube], "the box is placed once");
    assert_eq!(common::world(doc), placed, "no placement is written");
    assert_eq!(
        viewer::world::repoint_offer(doc, union, tol).map(|offer| offer
            .rows
            .iter()
            .map(|row| row.body)
            .collect::<Vec<_>>()),
        doc.output(cube, 0).map(|cube| vec![cube]),
        "the union offers to re-point the box's placement onto it"
    );
}

/// **The face is read through the body the ray met, not through the
/// feature that minted it.** A flat swept by the extrude and shrunk by
/// the chamfer is picked on the CHAMFER's body; the pick's `feature()`
/// answers the extrude, so the two are demonstrably different values
/// here — and the profile drawn there reads the chamfer, and its extrude
/// evaluates.
#[test]
fn the_face_is_read_through_the_body_the_ray_met_and_not_the_feature() {
    let tol = Tol::witness();
    let doc = Doc::empty_derived("drawn-on-face-chamfered", tol);
    let (doc, profile) = inserted(&doc, common::square(SIDE), tol);
    let (doc, cube) = inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(HEIGHT),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let edges = pncad::select::all_edges(session.evaluation().expect("the box evaluated"), cube);
    assert!(!edges.is_empty(), "the box has edges to bevel");
    let (doc, chamfer) = inserted(
        session.committed_doc(),
        Node::chamfer(cube, len(0.001), edges),
        tol,
    );
    let (doc, _) = common::placed(&doc, chamfer, tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let flat = {
        let ev = session.evaluation().expect("the chamfer evaluated");
        all_faces(ev, chamfer)
            .into_iter()
            .find(|name| {
                face_carrier_kind(ev, chamfer, name).is_ok_and(|kind| kind == SurfaceKind::Plane)
                    && attribute(name).minted_by() == Some(cube)
                    && name
                        .path
                        .iter()
                        .any(|seg| matches!(seg, RoleSeg::From { .. }))
            })
            .expect("a flat of the box, shrunk by the chamfer")
    };
    let picked = FaceSelection {
        name: flat,
        node: chamfer,
        body: 0,
    };
    assert_eq!(
        picked.feature(),
        cube,
        "the flat is still the extrude's face"
    );
    assert_ne!(
        cube, chamfer,
        "and the two questions have different answers"
    );
    let (drawn, boss) = boss_on(&mut session, picked);
    assert!(
        session.committed_doc().upstream(drawn).contains(&chamfer),
        "the profile reads the body the ray met"
    );
    session.pump();
    let ev = session.evaluation().expect("the document evaluated");
    assert!(
        ev.value(boss).is_some(),
        "the boss on the shrunk flat evaluated: {:?}",
        ev.result(boss)
    );
}

/// **A face whose node is no body refuses at the door, typed** — and
/// records nothing.
#[test]
fn a_face_on_no_body_is_refused_at_the_door() {
    let tol = Tol::witness();
    let (doc, cube) = boxed(tol);
    let (doc, profile) = inserted(&doc, common::square(SIDE), tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let steps = session.history().len();
    let out = session.perform(footprint_on(FaceSelection {
        node: profile,
        ..top_of(cube)
    }));
    assert!(
        matches!(
            &out.refusal,
            Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Body }) if node.id() == profile
        ),
        "{:?}",
        out.refusal
    );
    assert_eq!(session.history().len(), steps, "nothing recorded");
}

/// **A face on a split's second half is drawn on that half**: a box
/// split at mid-height, the split drawn in the box's place, and its
/// lower half's underside picked there. The profile is drawn on the
/// underside's chart, so the boss hangs below the half, flush with it,
/// and the half combined with it is the half and the boss whole. Read
/// on the upper half, the underside's name has no carrier and the
/// profile would not evaluate.
#[test]
fn a_face_on_a_splits_lower_half_is_drawn_on_that_half() {
    let tol = Tol::witness();
    let size = [SIDE, SIDE, HEIGHT];
    let (mut session, _, split) = common::drawn::split_box(size, tol);
    let below = pncad::select::SplitHalf::Below;
    let underside = common::asm::pick_face(&session, &common::up_at(0.0, 0.0));
    assert_eq!(
        underside.node, split,
        "the ray met the split drawn as itself"
    );
    let (drawn, boss) = boss_on(&mut session, underside);
    assert!(
        session.committed_doc().upstream(drawn).contains(&split),
        "the profile reads the split"
    );
    let (lo, hi) = common::drawn::extent(&mut session, boss);
    assert!(
        (lo[2] + BOSS[2]).abs() <= 1e-12 && hi[2].abs() <= 1e-12,
        "the boss hangs off the underside: {lo:?}..{hi:?}"
    );
    let lower = Operand::output(split, below.port());
    let union = combined(&mut session, lower, boss);
    let fused = common::body_volume(&mut session, union, tol);
    let want = SIDE * SIDE * HEIGHT / 2.0 + BOSS.iter().product::<f64>();
    assert!(
        common::near(fused, want),
        "the half and its boss: {fused} vs {want}"
    );
}

/// **A boss drawn on a face of the box stands on that face's chart,
/// turned by the orientation nearest where it stands**, through the
/// session's own doors: for each face `faces` names (a label, a ray's
/// start and its direction), the extruded boss spans exactly the box the
/// expected chart carries its first-quadrant footprint to — `z` along
/// the face's outward normal, `x` the orientation of greatest trace that
/// puts it there (a tie keeping the author's `+x`), the origin on the
/// face's plane over the box's own — and the box combined with it holds
/// both whole. The box's origin is a corner of every face, so where the
/// chart's first quadrant lies over the face (the top) the boss stands
/// on it, and elsewhere it stands outward beside the face, meeting the
/// box along an edge; the volume says only that they share none.
/// Answers what went wrong, face by face.
fn bosses_land(faces: &[(&str, [f64; 3], [f64; 3])]) -> Vec<String> {
    let tol = Tol::witness();
    let mut bad = Vec::new();
    for &(label, from, dir) in faces {
        let (doc, cube) = boxed(tol);
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let face = common::asm::pick_face(&session, &bvh::test_support::ray(from, dir));
        let out = session.perform(footprint_on(face));
        let Some(profile) = out.made.filter(|_| out.refusal.is_none()) else {
            bad.push(format!("{label}: {:?}", out.refusal));
            continue;
        };
        let boss = common::session_insert(
            &mut session,
            SessionOp::AddExtrude {
                profile,
                distance: len(BOSS[2]),
            },
        );
        let union = combined(&mut session, Operand::from(cube), boss);
        let got = common::body_volume(&mut session, union, tol);
        let want = SIDE * SIDE * HEIGHT + BOSS[0] * BOSS[1] * BOSS[2];
        if !common::near(got, want) {
            bad.push(format!("{label}: volume {got} vs {want}"));
        }
        let outward = dir.map(|c| -c);
        let cross = |a: [f64; 3], b: [f64; 3]| {
            [
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ]
        };
        // The nearest: the greatest trace of the four orientations that
        // put `z` on the normal, a tie keeping the author's `+x`.
        let candidates: Vec<([f64; 3], f64)> = [
            [1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0],
        ]
        .into_iter()
        .filter(|x| (0..3).all(|i| x[i] * outward[i] == 0.0))
        .map(|x| (x, x[0] + cross(outward, x)[1] + outward[2]))
        .collect();
        let best = candidates
            .iter()
            .map(|&(_, t)| t)
            .fold(f64::NEG_INFINITY, f64::max);
        let tied: Vec<[f64; 3]> = candidates
            .iter()
            .filter(|&&(_, t)| t == best)
            .map(|&(x, _)| x)
            .collect();
        let x = match tied[..] {
            [x] => x,
            _ if tied.contains(&[1.0, 0.0, 0.0]) => [1.0, 0.0, 0.0],
            _ => panic!("{label}: a tie {tied:?} the author's +x does not settle"),
        };
        let y = cross(outward, x);
        // The origin over the box's own, on the face's plane.
        let k = (0..3)
            .find(|&i| outward[i] != 0.0)
            .expect("a face looks along an axis");
        let plane = if outward[k] > 0.0 {
            [SIDE, SIDE, HEIGHT][k]
        } else {
            0.0
        };
        let at: [f64; 3] = std::array::from_fn(|i| if i == k { plane } else { 0.0 });
        // The footprint's box, carried by the chart and swept outward.
        let corners: Vec<[f64; 3]> = [0.0, BOSS[0]]
            .into_iter()
            .flat_map(|a| [0.0, BOSS[1]].map(|b| (a, b)))
            .flat_map(|(a, b)| [0.0, BOSS[2]].map(|c| (a, b, c)))
            .map(|(a, b, c)| std::array::from_fn(|i| at[i] + a * x[i] + b * y[i] + c * outward[i]))
            .collect();
        let want = common::drawn::bounds(&corners);
        let got = common::drawn::extent(&mut session, boss);
        let near = |p: [f64; 3], q: [f64; 3]| (0..3).all(|i| (p[i] - q[i]).abs() <= 1e-12);
        if !(near(got.0, want.0) && near(got.1, want.1)) {
            bad.push(format!(
                "{label}: the boss spans {got:?}, the chart with x {x:?} puts it over {want:?}"
            ));
        }
    }
    bad
}

/// **A boss drawn on each of the box's six faces stands on its chart**
/// ([`bosses_land`]): the door draws on the face's chart at the
/// orientation nearest where it stands, with the two slides that plane
/// leaves, so on every face the boss spans the box that chart carries
/// its footprint to.
///
/// *Breaks if* the chart's slide is the one the face's plane fixes, its
/// orientation is other than the nearest, or its origin is anywhere but
/// on the face's plane over the box's origin, or the boss rises inward.
#[test]
fn a_boss_drawn_on_each_of_the_six_faces_lands() {
    let bad = bosses_land(&[
        ("top", [0.005, 0.005, 1.0], [0.0, 0.0, -1.0]),
        ("bottom", [0.005, 0.005, -1.0], [0.0, 0.0, 1.0]),
        ("+x side", [1.0, 0.005, 0.005], [-1.0, 0.0, 0.0]),
        ("-x side", [-1.0, 0.005, 0.005], [1.0, 0.0, 0.0]),
        ("+y side", [0.005, 1.0, 0.005], [0.0, -1.0, 0.0]),
        ("-y side", [0.005, -1.0, 0.005], [0.0, 1.0, 0.0]),
    ]);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **A boss drawn on a side stands on its chart** — the one-face twin of
/// [`a_boss_drawn_on_each_of_the_six_faces_lands`] through the same
/// doors, on a side, where the identity's slides would not span and the
/// identity would leave the boss looking along `+z`.
///
/// *Breaks if* as the six-face row, on the side.
#[test]
fn a_boss_drawn_on_a_side_lands() {
    let bad = bosses_land(&[("+x side", [1.0, 0.005, 0.005], [-1.0, 0.0, 0.0])]);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
