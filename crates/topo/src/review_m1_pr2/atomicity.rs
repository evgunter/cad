//! Adversarial e2e review artifact for M1 PR 2 (2026-07-16). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value. Promoted per
//! Ev's request (PR #17 thread).
//!
//! Atomicity under attack: every EulerOpError path leaves the body
//! DEEP-equal (all 10 arenas + provenance, not just counts). The review
//! suite's companion key-sequence-purity test (failed ops consume no key
//! slots) was already promoted into the shipped unit suite during the
//! PR 2 fix pass (`euler.rs::failed_ops_leave_the_key_sequence_pure`)
//! and is deliberately not duplicated here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::deep_snapshot;
use crate::{
    Body, EntityId, EulerOpError, FaceKey, GeomRef, HalfEdgeKey, LoopKey, MefSite, MevSite,
    PointKey, VertexKey, validate,
};
use geom_core::Point3;
use geom_core::Tol;

fn p(x: f64) -> Point3<f64> {
    Point3::new(x, 0.0, 0.0)
}

/// Builds the digon pillow through ops: 2 vertices, 2 edges, 2 faces.
/// Returns (body, seed, seg, split).
fn pillow(
    tol: Tol,
) -> (
    Body<f64>,
    crate::MvfsCreated,
    crate::MevCreated,
    crate::MefCreated,
) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
            tol,
        )
        .unwrap();
    let split = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    (body, seed, seg, split)
}

fn assert_err_deep_unchanged(
    body: &mut Body<f64>,
    expected: EulerOpError,
    op: impl FnOnce(&mut Body<f64>) -> EulerOpError,
) {
    let before = deep_snapshot(body);
    let err = op(body);
    assert_eq!(err, expected);
    let after = deep_snapshot(body);
    assert_eq!(before, after, "body mutated on Err ({err:?})");
}

#[test]
fn stale_argument_keys_leave_the_body_deep_equal() {
    let tol = Tol::witness();
    let (mut body, seed, seg, _) = pillow(tol);
    let null_he = HalfEdgeKey::default();
    let null_loop = LoopKey::default();

    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::StaleKey {
            key: EntityId::HalfEdge(null_he),
        },
        |b| {
            b.mev_line(
                MevSite::Fan {
                    he1: null_he,
                    he2: seg.he_plus,
                },
                p(9.0),
                tol,
            )
            .unwrap_err()
        },
    );
    // he2 stale (he1 fine).
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::StaleKey {
            key: EntityId::HalfEdge(null_he),
        },
        |b| {
            b.mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: null_he,
                },
                p(9.0),
                tol,
            )
            .unwrap_err()
        },
    );
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::StaleKey {
            key: EntityId::Loop(null_loop),
        },
        |b| {
            b.mev_line(MevSite::Lone { r#loop: null_loop }, p(9.0), tol)
                .unwrap_err()
        },
    );
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::StaleKey {
            key: EntityId::HalfEdge(null_he),
        },
        |b| {
            b.mef_chord(
                MefSite::Chords {
                    he1: null_he,
                    he2: null_he,
                },
                tol,
            )
            .unwrap_err()
        },
    );
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::StaleKey {
            key: EntityId::Loop(null_loop),
        },
        |b| {
            b.mef_chord(MefSite::Lone { r#loop: null_loop }, tol)
                .unwrap_err()
        },
    );
    let _ = seed;
}

#[test]
fn semantic_precondition_failures_leave_the_body_deep_equal() {
    let tol = Tol::witness();
    let (mut body, seed, seg, split) = pillow(tol);

    // FanStartMismatch: seg.he_plus starts at A, seg.he_minus at B.
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::FanStartMismatch {
            he1: seg.he_plus,
            he2: seg.he_minus,
        },
        |b| {
            b.mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                },
                p(9.0),
                tol,
            )
            .unwrap_err()
        },
    );
    // NotSameLoop: the two pillow loops.
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::NotSameLoop {
            he1: seg.he_plus,
            he2: split.he_plus,
        },
        |b| {
            b.mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: split.he_plus,
                },
                tol,
            )
            .unwrap_err()
        },
    );
    // LoopNotEmpty on both Lone sites (the loops are cycles now).
    let cyc = seed.r#loop;
    assert_err_deep_unchanged(&mut body, EulerOpError::LoopNotEmpty { r#loop: cyc }, |b| {
        b.mev_line(MevSite::Lone { r#loop: cyc }, p(9.0), tol)
            .unwrap_err()
    });
    assert_err_deep_unchanged(&mut body, EulerOpError::LoopNotEmpty { r#loop: cyc }, |b| {
        b.mef_chord(MefSite::Lone { r#loop: cyc }, tol).unwrap_err()
    });
}

#[test]
fn raw_corruption_paths_leave_the_body_deep_equal() {
    let tol = Tol::witness();
    // Each corruption is built through the PUBLIC raw builder (add_* /
    // get_*_mut), so this is a consumer-reachable state today.

    // StaleKey(vertex): a half-edge whose start vertex key is null.
    let (mut body, _, seg, _) = pillow(tol);
    body.get_half_edge_mut(seg.he_plus).unwrap().start = VertexKey::default();
    body.get_half_edge_mut(seg.he_minus).unwrap().start = VertexKey::default();
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::StaleKey {
            key: EntityId::Vertex(VertexKey::default()),
        },
        |b| {
            b.mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                },
                p(9.0),
                tol,
            )
            .unwrap_err()
        },
    );

    // StaleKey(prev): strut mev at a half-edge with a null prev.
    let (mut body, _, seg, _) = pillow(tol);
    body.get_half_edge_mut(seg.he_plus).unwrap().prev = HalfEdgeKey::default();
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::StaleKey {
            key: EntityId::HalfEdge(HalfEdgeKey::default()),
        },
        |b| {
            b.mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(9.0),
                tol,
            )
            .unwrap_err()
        },
    );

    // StaleGeometry: vertex with a null point key, mef needs its coords.
    let (mut body, seed, seg, _) = pillow(tol);
    body.get_vertex_mut(seed.vertex).unwrap().point = PointKey::default();
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::StaleGeometry {
            key: GeomRef::Point(PointKey::default()),
        },
        |b| {
            // seg.he_plus starts at seed.vertex; same loop as the mef
            // half spliced there... use self-loop chords at he_plus.
            b.mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                tol,
            )
            .unwrap_err()
        },
    );

    // FanOrbitBroken: corrupt the edge<->half-edge bijection so mate()
    // fails mid-orbit.
    let (mut body, _, seg, split) = pillow(tol);
    body.get_edge_mut(seg.edge).unwrap().he_plus = split.he_plus;
    body.get_edge_mut(seg.edge).unwrap().he_minus = split.he_plus;
    // seg.he_plus and split.he_plus both start at the seed vertex.
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::FanOrbitBroken {
            he1: seg.he_plus,
            he2: split.he_plus,
        },
        |b| {
            b.mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: split.he_plus,
                },
                p(9.0),
                tol,
            )
            .unwrap_err()
        },
    );

    // LoopCycleBroken: tear next so the walk from he1 never reaches he2.
    let (mut body, _, seg, split) = pillow(tol);
    // seg.he_plus is in split's loop (mef moved it); its cycle is
    // [seg.he_plus, split.he_minus]. Tear seg.he_plus.next into the
    // OTHER loop.
    body.get_half_edge_mut(seg.he_plus).unwrap().next = split.he_plus;
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::LoopCycleBroken {
            r#loop: split.r#loop,
        },
        |b| {
            b.mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: split.he_minus,
                },
                tol,
            )
            .unwrap_err()
        },
    );

    // LoopNotCycle: halves claiming an EMPTY loop as parent.
    let (mut body, _, seg, split) = pillow(tol);
    let seed2 = body.mvfs(p(50.0)).unwrap(); // a second, disjoint skeletal body
    body.get_half_edge_mut(seg.he_plus).unwrap().parent_loop = seed2.r#loop;
    body.get_half_edge_mut(split.he_minus).unwrap().parent_loop = seed2.r#loop;
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::LoopNotCycle {
            r#loop: seed2.r#loop,
        },
        |b| {
            b.mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: split.he_minus,
                },
                tol,
            )
            .unwrap_err()
        },
    );

    // StaleKey(face): loop with a null face key.
    let (mut body, _, seg, split) = pillow(tol);
    body.get_loop_mut(split.r#loop).unwrap().face = FaceKey::default();
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::StaleKey {
            key: EntityId::Face(FaceKey::default()),
        },
        |b| {
            b.mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: split.he_minus,
                },
                tol,
            )
            .unwrap_err()
        },
    );

    // StaleKey(shell): face with a null shell key.
    let (mut body, _, seg, split) = pillow(tol);
    body.get_face_mut(split.face).unwrap().shell = crate::ShellKey::default();
    assert_err_deep_unchanged(
        &mut body,
        EulerOpError::StaleKey {
            key: EntityId::Shell(crate::ShellKey::default()),
        },
        |b| {
            b.mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: split.he_minus,
                },
                tol,
            )
            .unwrap_err()
        },
    );
}
