//! **"Sketch on this face" at the viewer** (spec §5): a profile drawn
//! on its own plane, extruded away from a picked face, its copy placed
//! against the face's plane and fused with the face's body — one
//! committed action through `SessionOp::SketchOnFace`, which is the
//! kernel's own gesture (`Recording::sketch_on_face`).
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
use pncad::document::{Dimension, Formula};

use pncad::document::{Doc, DocEdit, Node, ProfileProgram, RecipeNodeId};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName, SurfaceKind, attribute};
use pncad::select::{all_faces, face_carrier_kind};
use viewer::session::{DocSession, FaceSelection, NodeKindWanted, Refusal, SessionOp};

/// The box's side: 20 mm square, 10 mm tall.
const SIDE: f64 = 0.02;
/// The box's height.
const HEIGHT: f64 = 0.01;
/// The boss sketched on the box's top: a 5 mm square, 4 mm tall.
const BOSS: [f64; 3] = [0.005, 0.005, 0.004];

/// A 20 mm square box, 10 mm tall, through the document door and
/// placed in the world — the body every row here picks a face on.
fn boxed(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let doc = Doc::empty_derived("sketch-on-face", tol);
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

/// The boss op on `face`.
fn boss_on(face: FaceSelection) -> SessionOp {
    SessionOp::SketchOnFace {
        face,
        loops: vec![common::rectangle_loop([0.0, 0.0], BOSS[0], BOSS[1])],
        distance: len(BOSS[2]),
        spin: Formula::zero(Dimension::Angle),
    }
}

/// **One action, one undo, and the body it makes is the box with the
/// boss on it.** The gesture writes the profile, the extrude, the
/// placement on the face and the union in one recorded action; the
/// box's world placement is re-pointed to the union, so the world is
/// that one body; its volume is the two blocks' sum (the boss stands
/// on the face, flush, meeting it in a face and no volume); and one
/// undo takes the whole action back.
#[test]
fn a_sketch_on_a_face_is_one_action_and_one_body() {
    let tol = Tol::witness();
    let (doc, cube) = boxed(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let before = session.committed_doc().clone();
    let steps = session.history().len();
    let out = session.perform(boss_on(top_of(cube)));
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert_eq!(session.history().len(), steps + 1, "one action");
    let inserted: Vec<&str> = out
        .committed
        .iter()
        .filter_map(|edit| match edit {
            DocEdit::InsertNode { node, .. } => Some(pncad::document::node_kind_noun(node)),
            _ => None,
        })
        .collect();
    assert_eq!(
        inserted,
        ["Profile", "Extrude", "Place", "Union"],
        "the profile, the construction, its placement and the combine"
    );
    let doc = session.committed_doc();
    let union = doc
        .ids()
        .into_iter()
        .find(|&id| matches!(doc.node(id), Some(Node::Union { .. })))
        .expect("the union landed");
    assert_eq!(
        common::world(doc),
        [union],
        "the box's placement now places the union"
    );
    let got = common::body_volume(&mut session, union, tol);
    let want = SIDE * SIDE * HEIGHT + BOSS[0] * BOSS[1] * BOSS[2];
    assert!(common::near(got, want), "box plus boss: {got} vs {want}");

    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo takes the whole action back"
    );
}

/// **The face is read through the body the ray met, not through the
/// feature that minted it.** A flat swept by the extrude and shrunk by
/// the chamfer is picked on the CHAMFER's body; the pick's `feature()`
/// answers the extrude, so the two are demonstrably different values
/// here — and the union the gesture writes reads the chamfer, and
/// evaluates.
#[test]
fn the_face_is_read_through_the_body_the_ray_met_and_not_the_feature() {
    let tol = Tol::witness();
    let doc = Doc::empty_derived("sketch-on-face-chamfered", tol);
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
    let out = session.perform(boss_on(picked));
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let union = {
        let doc = session.committed_doc();
        doc.ids()
            .into_iter()
            .find(|&id| matches!(doc.node(id), Some(Node::Union { .. })))
            .expect("the union landed")
    };
    assert!(
        session.committed_doc().upstream(union).contains(&chamfer),
        "the union reads the body the ray met"
    );
    session.pump();
    let ev = session.evaluation().expect("the document evaluated");
    assert!(
        ev.value(union).is_some(),
        "the sketch on the shrunk flat evaluated: {:?}",
        ev.result(union)
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
    let out = session.perform(boss_on(FaceSelection {
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

/// **A face on a split's second half is sketched on that half**: a box
/// split at mid-height, its lower half placed 0.1 m along x, and its
/// underside picked there. The boss is fused onto the split's LOWER
/// output, read by its port, and the half's copy draws the half with
/// the boss on it.
#[test]
fn a_face_on_a_splits_lower_half_is_sketched_on_that_half() {
    let tol = Tol::witness();
    let size = [SIDE, SIDE, HEIGHT];
    let (session, _, split) = common::drawn::split_box(size, tol);
    let below = pncad::select::SplitHalf::Below;
    let at = common::drawn::translated([0.1, 0.0, 0.0]);
    let (mut session, half) = common::drawn::half_placed(&session, split, below, &at, tol);
    let underside = common::asm::pick_face(&session, &common::up_at(0.1, 0.0));
    assert_eq!(underside.node, half, "the ray met the lower half's copy");
    let out = session.perform(boss_on(underside));
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    let doc = session.committed_doc();
    let Some(Node::Place { source, .. }) = doc.node(half) else {
        panic!("the half's copy is a placement");
    };
    let union = doc
        .operation_of(source.read)
        .expect("the copy reads an operation");
    let Some(Node::Union { members }) = doc.node(union) else {
        panic!("the copy draws the fused half, not {:?}", doc.node(union));
    };
    let lower = doc.output(split, below.port());
    assert!(
        members.rows().iter().any(|(_, m)| Some(m.read) == lower),
        "the union reads the split's lower half by its port: {members:?}"
    );
    let fused = common::body_volume(&mut session, half, tol);
    let want = SIDE * SIDE * HEIGHT / 2.0 + BOSS.iter().product::<f64>();
    assert!(
        common::near(fused, want),
        "the half and its boss drawn: {fused} vs {want}"
    );
}

/// **A boss sketched on a face of the box lands on it, turned by the
/// orientation nearest where it stands**, through the session's own
/// door: for each face `faces` names (a label, a ray's start and its
/// direction), the gesture's copy is placed with its `z` along the
/// face's outward normal by the orientation of greatest trace that puts
/// it there (a tie keeping the author's `+x`), its origin on the face
/// over the box's own, and the union holds the box and the boss whole.
/// Answers what went wrong, face by face.
fn bosses_land(faces: &[(&str, [f64; 3], [f64; 3])]) -> Vec<String> {
    let tol = Tol::witness();
    let mut bad = Vec::new();
    for &(label, from, dir) in faces {
        let (doc, _cube) = boxed(tol);
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let face = common::asm::pick_face(&session, &bvh::test_support::ray(from, dir));
        let out = session.perform(boss_on(face));
        if out.refusal.is_some() {
            bad.push(format!("{label}: {:?}", out.refusal));
            continue;
        }
        session.pump();
        let doc = session.committed_doc();
        let find = |want: fn(&Node<ProfileProgram>) -> bool| {
            doc.ids()
                .into_iter()
                .rev()
                .find(|&id| doc.node(id).is_some_and(want))
        };
        // The gesture's union and its copy, the last of each it inserted.
        let union = find(|n| matches!(n, Node::Union { .. })).expect("the union landed");
        let placement = find(|n| matches!(n, Node::Place { .. })).expect("the copy landed");
        let got = common::body_volume(&mut session, union, tol);
        let want = SIDE * SIDE * HEIGHT + BOSS[0] * BOSS[1] * BOSS[2];
        if !common::near(got, want) {
            bad.push(format!("{label}: volume {got} vs {want}"));
        }
        let doc = session.committed_doc();
        let frame = common::solve(&session, doc, tol)
            .relative(placement)
            .expect("the copy is placed in the box's space");
        let outward = dir.map(|c| -c);
        let cols = frame.columns;
        if cols[2] != outward {
            bad.push(format!(
                "{label}: z {:?}, the face looks along {outward:?}",
                cols[2]
            ));
        }
        // The nearest: the greatest trace of the four orientations that
        // put `z` on the normal, a tie keeping the author's `+x`.
        let axes = [
            [1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0],
        ];
        let cross = |a: [f64; 3], b: [f64; 3]| {
            [
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ]
        };
        let traces: Vec<f64> = axes
            .iter()
            .filter(|x| (0..3).all(|i| x[i] * outward[i] == 0.0))
            .map(|&x| x[0] + cross(outward, x)[1] + outward[2])
            .collect();
        let best = traces.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let trace = cols[0][0] + cols[1][1] + cols[2][2];
        if trace != best {
            bad.push(format!("{label}: trace {trace}, the nearest has {best}"));
        }
        if traces.iter().filter(|&&t| t == best).count() > 1 && cols[0] != [1.0, 0.0, 0.0] {
            bad.push(format!("{label}: a tie keeps +x, not {:?}", cols[0]));
        }
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
        if (0..3).any(|i| (frame.translation[i] - at[i]).abs() > 1e-15) {
            bad.push(format!(
                "{label}: origin {:?}, want {at:?}",
                frame.translation
            ));
        }
    }
    bad
}

/// **A boss sketched on each of the box's six faces lands on it**
/// ([`bosses_land`]): the gesture writes the orientation nearest where
/// the copy stands and the two slides that plane leaves, so every face
/// takes the boss, turned by that orientation and over the box's origin.
///
/// *Breaks if* the gesture writes the slide the face's plane fixes (a
/// side face refuses `SlidesDoNotSpan`), an orientation other than the
/// nearest, or puts the boss anywhere but over the box's origin on the
/// face.
#[test]
fn a_boss_sketched_on_each_of_the_six_faces_lands() {
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

/// **A boss sketched on a side lands on it** — the fast set's twin of
/// [`a_boss_sketched_on_each_of_the_six_faces_lands`] through the same
/// door, on a side, where the identity's slides would not span and the
/// identity would leave the boss looking along `+z`. Python's
/// `TestSketchOnEveryFace` takes all six faces through the façade on
/// every run.
///
/// *Breaks if* as the six-face row, on the side.
#[test]
fn a_boss_sketched_on_a_side_lands() {
    let bad = bosses_land(&[("+x side", [1.0, 0.005, 0.005], [-1.0, 0.0, 0.0])]);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
