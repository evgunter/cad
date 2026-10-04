//! Adversarial e2e review artifact for M1 PR 2 (2026-07-16).
//!
//! Atomicity under attack: every EulerOpError path leaves the body
//! DEEP-equal (all 10 arenas + provenance, not just counts), and a torn
//! body panics in the plan phase, before anything is written. The
//! companion key-sequence-purity test (failed ops consume no key slots)
//! is `euler.rs::failed_ops_leave_the_key_sequence_pure`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::assert_panics_deep_unchanged;
use crate::fixtures::assert_err_deep_unchanged;
use crate::{
    BadArgument, Body, EntityId, EulerOpError, FaceKey, HalfEdgeKey, LoopKey, MefSite, MevSite,
    PointKey, VertexKey, validate,
};

/// The premise a dangling link's panic names.
const DANGLING: &str = "which does not resolve: every public door keeps the body tier-1-valid";
/// The premise a walk that does not close names.
const WALK: &str = "every public door keeps the body tier-1-valid, where every such walk closes";

fn stale(role: &'static str, key: EntityId) -> EulerOpError {
    EulerOpError::Argument(BadArgument::Stale { role, key })
}
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
    let seed = body.mvfs(p(0.0), true).unwrap();
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

#[test]
fn stale_argument_keys_leave_the_body_deep_equal() {
    let tol = Tol::witness();
    let (mut body, seed, seg, _) = pillow(tol);
    let null_he = HalfEdgeKey::default();
    let null_loop = LoopKey::default();

    assert_err_deep_unchanged(&mut body, &stale("he1", EntityId::HalfEdge(null_he)), |b| {
        b.mev_line(
            MevSite::Fan {
                he1: null_he,
                he2: seg.he_plus,
            },
            p(9.0),
            tol,
        )
        .unwrap_err()
    });
    // he2 stale (he1 fine).
    assert_err_deep_unchanged(&mut body, &stale("he2", EntityId::HalfEdge(null_he)), |b| {
        b.mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: null_he,
            },
            p(9.0),
            tol,
        )
        .unwrap_err()
    });
    assert_err_deep_unchanged(&mut body, &stale("loop", EntityId::Loop(null_loop)), |b| {
        b.mev_line(MevSite::Lone { r#loop: null_loop }, p(9.0), tol)
            .unwrap_err()
    });
    assert_err_deep_unchanged(&mut body, &stale("he1", EntityId::HalfEdge(null_he)), |b| {
        b.mef_chord(
            MefSite::Chords {
                he1: null_he,
                he2: null_he,
            },
            tol,
        )
        .unwrap_err()
    });
    assert_err_deep_unchanged(&mut body, &stale("loop", EntityId::Loop(null_loop)), |b| {
        b.mef_chord(MefSite::Lone { r#loop: null_loop }, tol)
            .unwrap_err()
    });
    let _ = seed;
}

#[test]
fn semantic_precondition_failures_leave_the_body_deep_equal() {
    let tol = Tol::witness();
    let (mut body, seed, seg, split) = pillow(tol);

    // FanStartMismatch: seg.he_plus starts at A, seg.he_minus at B.
    assert_err_deep_unchanged(
        &mut body,
        &EulerOpError::FanStartMismatch {
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
        &EulerOpError::NotSameLoop {
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
    assert_err_deep_unchanged(
        &mut body,
        &EulerOpError::LoopNotEmpty { r#loop: cyc },
        |b| {
            b.mev_line(MevSite::Lone { r#loop: cyc }, p(9.0), tol)
                .unwrap_err()
        },
    );
    assert_err_deep_unchanged(
        &mut body,
        &EulerOpError::LoopNotEmpty { r#loop: cyc },
        |b| b.mef_chord(MefSite::Lone { r#loop: cyc }, tol).unwrap_err(),
    );
}

/// Each torn body is built through the raw builder (add_* /
/// get_*_mut), a state no public door produces; each op panics naming
/// the record or walk that fails, before writing anything.
#[test]
fn torn_bodies_panic_before_writing() {
    let tol = Tol::witness();

    // A half-edge whose start vertex key is null.
    let (mut body, _, seg, _) = pillow(tol);
    body.get_half_edge_mut(seg.he_plus).unwrap().start = VertexKey::default();
    body.get_half_edge_mut(seg.he_minus).unwrap().start = VertexKey::default();
    assert_panics_deep_unchanged(
        "null start",
        &mut body,
        &["'s start names", DANGLING],
        |b| {
            let _ = b.mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                },
                p(9.0),
                tol,
            );
        },
    );

    // Strut mev at a half-edge with a null prev.
    let (mut body, _, seg, _) = pillow(tol);
    body.get_half_edge_mut(seg.he_plus).unwrap().prev = HalfEdgeKey::default();
    assert_panics_deep_unchanged("null prev", &mut body, &["'s prev names", DANGLING], |b| {
        let _ = b.mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: seg.he_plus,
            },
            p(9.0),
            tol,
        );
    });

    // A vertex with a null point key; mef needs its coordinates (the
    // self-loop chord at he_plus, which starts at seed.vertex).
    let (mut body, seed, seg, _) = pillow(tol);
    body.get_vertex_mut(seed.vertex).unwrap().point = PointKey::default();
    assert_panics_deep_unchanged(
        "null point",
        &mut body,
        &["'s point names", DANGLING],
        |b| {
            let _ = b.mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                tol,
            );
        },
    );

    // The edge<->half-edge bijection corrupted, so the orbit walk's
    // mate step leaves the walk. seg.he_plus and split.he_plus both
    // start at the seed vertex.
    let (mut body, _, seg, split) = pillow(tol);
    body.get_edge_mut(seg.edge).unwrap().he_plus = split.he_plus;
    body.get_edge_mut(seg.edge).unwrap().he_minus = split.he_plus;
    assert_panics_deep_unchanged(
        "torn orbit",
        &mut body,
        &["the orbit walk from", WALK],
        |b| {
            let _ = b.mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: split.he_plus,
                },
                p(9.0),
                tol,
            );
        },
    );

    // next torn into the other loop: seg.he_plus's cycle is
    // [seg.he_plus, split.he_minus] (mef moved it into split's loop).
    let (mut body, _, seg, split) = pillow(tol);
    body.get_half_edge_mut(seg.he_plus).unwrap().next = split.he_plus;
    assert_panics_deep_unchanged("torn next", &mut body, &["the loop walk from", WALK], |b| {
        let _ = b.mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: split.he_minus,
            },
            tol,
        );
    });

    // Halves claiming an EMPTY loop (a disjoint skeletal body's) as
    // parent.
    let (mut body, _, seg, split) = pillow(tol);
    let seed2 = body.mvfs(p(50.0), true).unwrap();
    body.get_half_edge_mut(seg.he_plus).unwrap().parent_loop = seed2.r#loop;
    body.get_half_edge_mut(split.he_minus).unwrap().parent_loop = seed2.r#loop;
    assert_panics_deep_unchanged(
        "empty parent loop",
        &mut body,
        &["which is empty: on a tier-1-valid body an empty loop reaches no half-edge"],
        |b| {
            let _ = b.mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: split.he_minus,
                },
                tol,
            );
        },
    );

    // A loop with a null face key.
    let (mut body, _, seg, split) = pillow(tol);
    body.get_loop_mut(split.r#loop).unwrap().face = FaceKey::default();
    assert_panics_deep_unchanged("null face", &mut body, &["'s face names", DANGLING], |b| {
        let _ = b.mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: split.he_minus,
            },
            tol,
        );
    });

    // A face with a null shell key.
    let (mut body, _, seg, split) = pillow(tol);
    body.get_face_mut(split.face).unwrap().shell = crate::ShellKey::default();
    assert_panics_deep_unchanged(
        "null shell",
        &mut body,
        &["'s shell names", DANGLING],
        |b| {
            let _ = b.mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: split.he_minus,
                },
                tol,
            );
        },
    );
}
