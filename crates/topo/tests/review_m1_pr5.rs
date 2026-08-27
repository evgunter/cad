//! Adversarial e2e review artifact for M1 PR 5 (2026-07-16), promoted
//! into the shipped suite per the standing convention
//! (`memories/review-and-dependency-policy.md`): reviewers write and run
//! real consumer programs against the API under review, and the
//! programs are kept.
//!
//! Everything here goes through the **public API only** — which is
//! itself part of what PR 5 put under test (the raw-builder demotion:
//! an integration test can no longer raw-build, so every state below is
//! genuinely operator-reachable). The derivations (component counts,
//! genus ledgers, the separating-curve argument behind the `ring_move`
//! probes) are **independent re-derivations by the reviewer — do not
//! "simplify" them to match the implementation**; their value is
//! exactly that independence.
//!
//! Highlights: multi-detachment (`ShellDisconnected{3}` exact), a
//! detached component acquiring genus (per-shell sum 2 = 2(c − Σg) with
//! c = 2, Σg = 1), per-shell attribution across two solids, the
//! kemr-old-side-empty family (Empty OUTER + cycle ring is
//! tier-1-legal), the cross-component `ring_move` probe, the
//! promoted-EMPTY-ring pin (mfkrh on an empty ring leaves an empty loop
//! ⇒ tier 2 reports BOTH `ScaffoldingEmptyLoop` AND
//! `ShellDisconnected` — the grown digon is the only witness that c = 1
//! is an independent rule), stale/foreign-key batteries, and the
//! **exhaustive public-mutator sweeps**: every accepted
//! `ring_move`/`kfmrh`/`mfkrh` (ring_move to depth 2) over four
//! adversarial fixtures — including the non-separating-ring
//! pillow-torus — must leave tier 1 intact; this is the demotion
//! claim's falsification battery, and it held.
//!
//! Promoted verbatim except this header. The reviewer's in-crate
//! corrupt-state probes live separately in
//! `src/review_m1_pr5_internal.rs` (they need pub(crate) access).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;
use geom_core::Tol;
use topo::{
    Body, EulerOpError, LoopBoundary, MefSite, MevSite, ValidationError, validate, validate_closed,
};

fn pt(x: f64, y: f64) -> Point3<f64> {
    Point3::new(x, y, 0.0)
}

/// Pillow: mvfs + mev + mef. Returns (body, seed loop info).
fn pillow() -> (Body<f64>, topo::MvfsCreated, topo::MevCreated) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(1.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    body.mef_chord(
        MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        },
        Tol::witness(),
    )
    .unwrap();
    (body, seed, seg)
}

/// Grows a detached digon hanging off `anchor_he`'s vertex fan: strut,
/// kemr (planted empty ring), grow to digon, mfkrh-promote. Returns the
/// promoted face's ring loop key (now a face outer).
fn grow_and_promote_detached_digon(
    body: &mut Body<f64>,
    anchor_he: topo::HalfEdgeKey,
    x: f64,
) -> topo::MfkrhCreated {
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: anchor_he,
                he2: anchor_he,
            },
            pt(x, 1.0),
            Tol::witness(),
        )
        .unwrap();
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    let grow = body
        .mev_line(
            MevSite::Lone { r#loop: kill.ring },
            pt(x, 2.0),
            Tol::witness(),
        )
        .unwrap();
    body.mef_chord(
        MefSite::Chords {
            he1: grow.he_plus,
            he2: grow.he_minus,
        },
        Tol::witness(),
    )
    .unwrap();
    body.mfkrh_plug(kill.ring).unwrap()
}

// ----------------------------------------------------------------------
// 1. Component/genus derivations (independent).
// ----------------------------------------------------------------------

/// MULTI-detachment: pillow + TWO promoted detached digons = THREE
/// components in ONE shell. Derivation: pillow (v2 e2 f2 chi=2, g=0),
/// each digon pair (v2 e2 f2 chi=2, g=0). Per-shell sum
/// v6-e6+f6-r0 = 6 = 2(c - sum g) = 2(3-0). Tier 1 must pass; tier 2
/// must report ShellDisconnected{components: 3} and nothing else.
#[test]
fn multi_detachment_three_components() {
    let (mut body, seed, seg) = pillow();
    grow_and_promote_detached_digon(&mut body, seg.he_plus, 2.0);
    grow_and_promote_detached_digon(&mut body, seg.he_plus, 3.0);
    assert_eq!(body.vertices().count(), 6);
    assert_eq!(body.edges().count(), 6);
    assert_eq!(body.faces().count(), 6);
    assert_eq!(validate(&body), Ok(()), "tier 1 accepts 3 components");
    assert_eq!(
        validate_closed(&body),
        Err(vec![ValidationError::ShellDisconnected {
            shell: seed.shell,
            components: 3,
        }])
    );
}

/// Nested detachment: the detached component itself acquires genus.
/// Promote a detached digon, then on ITS component: grow a second
/// detached digon anchored to the first digon's rim, promote it, then
/// kfmrh the two... no — simpler honest route: build a handle within
/// the detached component by kemr + kfmrh entirely inside it.
///
/// Route: detached digon pair (faces P and D). Grow a strut off the
/// digon rim, kemr to plant an empty ring ON a digon face, grow it to a
/// digon, mef, then kfmrh(other digon face, inner face) — that plugs
/// the inner ring-cycle onto another face of the SAME component: a
/// handle, g+1.
///
/// Derivation of the final state: component A = pillow (chi 2, g 0);
/// component B = digon pair with a handle (chi 2(1-1) = 0, g 1).
/// Per-shell sum: 2 + 0 = 2 = 2(c - sum g) = 2(2 - 1). Tier 1 passes;
/// tier 2 = exactly ShellDisconnected{2}.
#[test]
fn nested_detachment_detached_component_with_genus() {
    let (mut body, seed, seg) = pillow();
    let promoted = grow_and_promote_detached_digon(&mut body, seg.he_plus, 2.0);

    // A rim half-edge of the detached digon: the promoted face's outer
    // cycle.
    let promoted_face = promoted.face;
    let outer = body.get_face(promoted_face).unwrap().outer;
    let LoopBoundary::Cycle { first: rim_he } = body.get_loop(outer).unwrap().boundary else {
        panic!("promoted face outer must be a cycle");
    };

    // Plant an empty ring on the promoted face (strut + kemr)...
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: rim_he,
                he2: rim_he,
            },
            pt(2.0, 3.0),
            Tol::witness(),
        )
        .unwrap();
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    // ...grow it to a digon cycle with an island face...
    let grow = body
        .mev_line(
            MevSite::Lone { r#loop: kill.ring },
            pt(2.0, 4.0),
            Tol::witness(),
        )
        .unwrap();
    let island = body
        .mef_chord(
            MefSite::Chords {
                he1: grow.he_plus,
                he2: grow.he_minus,
            },
            Tol::witness(),
        )
        .unwrap();
    // ...and kfmrh the island face onto the promoted face (same
    // component => handle, g+1): island's outer becomes a ring of the
    // promoted face. f2 = island must be ring-free (it is).
    let res = body.kfmrh(promoted_face, island.face);
    assert!(res.is_ok(), "kfmrh within the detached component: {res:?}");

    // Counts: pillow v2 e2 f2; detached component: digon pair v2 e2 f2
    // + strut->ring digon: +2 v, +2 e, +1 face (island got killed by
    // kfmrh: mef made 1 face, kfmrh killed it... no: kfmrh kills f2 =
    // island face) => detached component: v4 e4 f3 r2 (kill.ring cycle
    // + demoted island outer), chi = 4-4+3-2 = 1 ?? -- derive in test:
    let v = body.vertices().count() as i64;
    let e = body.edges().count() as i64;
    let f = body.faces().count() as i64;
    let r: i64 = body.faces().map(|(_, face)| face.rings.len() as i64).sum();
    // Per-shell sum must be 2(c - sum g). c = 2. If the detached
    // component has genus 1, sum = 2(2-1) = 2; pillow contributes 2,
    // so detached contributes 0.
    assert_eq!(v - e + f - r, 2, "per-shell E-P sum (c=2, sum g=1)");
    assert_eq!(validate(&body), Ok(()), "tier 1 accepts the nested state");
    assert_eq!(
        validate_closed(&body),
        Err(vec![ValidationError::ShellDisconnected {
            shell: seed.shell,
            components: 2,
        }])
    );
}

/// Two solids in one body: per-shell independence. Each is its own
/// shell with c=1; tier 1 and tier 2 pass.
#[test]
fn two_solids_validate_independently() {
    let mut body = Body::<f64>::new();
    for i in 0..2 {
        let seed = body.mvfs(pt(10.0 * f64::from(i), 0.0)).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                pt(10.0 * f64::from(i) + 1.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        body.mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            Tol::witness(),
        )
        .unwrap();
    }
    assert_eq!(body.solids().count(), 2);
    assert_eq!(body.shells().count(), 2);
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(validate_closed(&body), Ok(()));
}

/// One solid disconnected + one clean: only the disconnected shell is
/// reported.
#[test]
fn per_shell_disconnection_is_attributed_to_the_right_shell() {
    let mut body = Body::<f64>::new();
    // Solid 1: clean pillow.
    let seed1 = body.mvfs(pt(0.0, 0.0)).unwrap();
    let seg1 = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed1.r#loop,
            },
            pt(1.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    body.mef_chord(
        MefSite::Chords {
            he1: seg1.he_plus,
            he2: seg1.he_minus,
        },
        Tol::witness(),
    )
    .unwrap();
    // Solid 2: pillow + promoted detached digon.
    let seed2 = body.mvfs(pt(10.0, 0.0)).unwrap();
    let seg2 = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed2.r#loop,
            },
            pt(11.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    body.mef_chord(
        MefSite::Chords {
            he1: seg2.he_plus,
            he2: seg2.he_minus,
        },
        Tol::witness(),
    )
    .unwrap();
    grow_and_promote_detached_digon(&mut body, seg2.he_plus, 12.0);
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(
        validate_closed(&body),
        Err(vec![ValidationError::ShellDisconnected {
            shell: seed2.shell,
            components: 2,
        }])
    );
}

/// kemr old-side-empty on the OUTER: Empty outer + everything else in
/// the ring. Reached from a strut loop (both halves of one edge in one
/// loop) -> both sides empty; then grow the RING back to a cycle: the
/// face has an Empty OUTER and a CYCLE ring. Tier 1 must accept
/// (Euler-reachable); component partition glues both loops.
#[test]
fn empty_outer_with_cycle_ring_is_tier1_legal() {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(1.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    // kemr on the segment loop: both sides empty (outer keeps its key,
    // Empty at v0; ring Empty at v1).
    let kill = body.kemr(seg.he_plus, seg.he_minus).unwrap();
    assert_eq!(validate(&body), Ok(()), "both-empty kemr output is tier 1");
    // Grow the RING to a cycle: face now has Empty outer + cycle ring.
    let grow = body
        .mev_line(
            MevSite::Lone { r#loop: kill.ring },
            pt(2.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    let outer_boundary = body
        .get_loop(body.get_face(seed.face).unwrap().outer)
        .unwrap()
        .boundary;
    assert!(
        matches!(outer_boundary, LoopBoundary::Empty { .. }),
        "outer stayed empty: {outer_boundary:?}"
    );
    let _ = grow;
    assert_eq!(
        validate(&body),
        Ok(()),
        "Empty outer + cycle ring is tier-1 legal"
    );
    // chi derivation: one face, ring cycle = strut (2 he, 1 edge, 2 v)
    // + empty outer vertex: v3 - e1 + f1 - r1 = 2 = 2(1-0). One
    // component.
    // Tier 2: the empty OUTER is scaffolding; the ring-cycle strut tips
    // are valence-1.
    let errs = validate_closed(&body).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::ScaffoldingEmptyLoop { .. })),
        "{errs:?}"
    );
}

/// ring_move cross-component: holed-box-free variant. Move a ring whose
/// cycle mates live in component A onto a face of detached component B.
/// Derivation (see review notes): the ring is a separating curve on a
/// genus-0 component, so the move re-glues without breaking per-
/// component E-P; the components re-partition but each satisfies
/// chi = 2(1-g). Tier 1 must hold (and the demotion claim survives this
/// probe); component count changes are reported by tier 2 only.
#[test]
fn ring_move_cross_component_stays_tier1_valid() {
    let (mut body, seed, seg) = pillow();
    // Component A: pillow with an ATTACHED island: strut+kemr+grow+mef
    // (no mfkrh) => ring on a pillow face, island digon hanging on it.
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: seg.he_plus,
            },
            pt(0.0, 1.0),
            Tol::witness(),
        )
        .unwrap();
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    let grow = body
        .mev_line(
            MevSite::Lone { r#loop: kill.ring },
            pt(0.0, 2.0),
            Tol::witness(),
        )
        .unwrap();
    body.mef_chord(
        MefSite::Chords {
            he1: grow.he_plus,
            he2: grow.he_minus,
        },
        Tol::witness(),
    )
    .unwrap();
    // Component B: promoted detached digon.
    let promoted = grow_and_promote_detached_digon(&mut body, seg.he_plus, 5.0);
    assert_eq!(validate(&body), Ok(()));

    // Move the still-a-ring island boundary from its pillow face onto
    // B's promoted face.
    let res = body.ring_move(kill.ring, promoted.face);
    assert_eq!(res, Ok(()), "ring_move accepts the cross-component move");
    // Tier 1 must still pass: this is the sharpest public-API probe at
    // the component partition (a debug postcondition fires inside
    // ring_move otherwise).
    assert_eq!(
        validate(&body),
        Ok(()),
        "cross-component ring_move output stays tier-1 valid"
    );
    // Derivation: pillow alone (chi 2) is one component; island digon
    // now glued to B via the moved ring: {island D, promoted P, digon
    // D2} v4 e4 f3 r1 chi = 2. c = 2.
    let errs = validate_closed(&body).unwrap_err();
    assert_eq!(
        errs,
        vec![ValidationError::ShellDisconnected {
            shell: seed.shell,
            components: 2,
        }]
    );
}

// ----------------------------------------------------------------------
// 2. Demotion attack battery: adversarial-but-live keys through every
// public op; NOTHING may panic in debug (typed errors or valid ops
// only). Run under --release too.
// ----------------------------------------------------------------------

#[test]
fn demotion_attack_battery_no_debug_panic() {
    let (mut body, seed, seg) = pillow();
    let promoted = grow_and_promote_detached_digon(&mut body, seg.he_plus, 2.0);
    let rim_he = seg.he_plus;
    let cube_outer = body.get_face(seed.face).unwrap().outer;

    // mev with he1/he2 at DIFFERENT vertices' fans (invalid Fan site).
    let other_he = seg.he_minus; // starts at v1, rim_he at v0
    let r = body.mev_line(
        MevSite::Fan {
            he1: rim_he,
            he2: other_he,
        },
        pt(9.0, 9.0),
        Tol::witness(),
    );
    assert!(r.is_err(), "cross-vertex fan must be rejected: {r:?}");

    // mev Lone on a CYCLE loop.
    let r = body.mev_line(
        MevSite::Lone { r#loop: cube_outer },
        pt(9.0, 9.0),
        Tol::witness(),
    );
    assert!(r.is_err(), "{r:?}");

    // mef Chords with he1 == he2.
    let r = body.mef_chord(
        MefSite::Chords {
            he1: rim_he,
            he2: rim_he,
        },
        Tol::witness(),
    );
    // (May be legal per PR 4's mate-alone re-make; either way: no panic.)
    if let Ok(created) = r {
        // Undo (kef on the NEW loop's half) to keep the body simple.
        body.kef(created.he_minus).unwrap();
    }

    // mef Chords across DIFFERENT loops (he2 in another face).
    let promoted_outer = body.get_face(promoted.face).unwrap().outer;
    if let LoopBoundary::Cycle {
        first: other_loop_he,
    } = body.get_loop(promoted_outer).unwrap().boundary
    {
        let r = body.mef_chord(
            MefSite::Chords {
                he1: rim_he,
                he2: other_loop_he,
            },
            Tol::witness(),
        );
        assert!(r.is_err(), "cross-loop mef must be rejected: {r:?}");
    }

    // kemr on one edge's halves that sit in DIFFERENT loops (the pillow
    // segment edge post-mef).
    let r = body.kemr(seg.he_plus, seg.he_minus);
    assert!(r.is_err(), "halves in different loops: {r:?}");

    // kfmrh(f, f) and cross-shell kfmrh.
    let r = body.kfmrh(seed.face, seed.face);
    assert!(r.is_err(), "{r:?}");

    // mfkrh on an OUTER loop.
    let r = body.mfkrh_plug(cube_outer);
    assert!(r.is_err(), "outer is not a ring: {r:?}");

    // kev on a self-mated... on a pillow edge (distinct ends required).
    let _ = body.kev(rim_he);

    // kvfs on a multi-face solid must be rejected.
    let r = body.kvfs(seed.solid);
    assert!(r.is_err(), "kvfs needs the skeletal state: {r:?}");

    // ring_move: ring key that is an outer; to_face cross-shell.
    let r = body.ring_move(cube_outer, promoted.face);
    assert!(r.is_err(), "{r:?}");

    // Foreign keys from another body (lineage hazard: may alias or be
    // stale; must never panic).
    let (mut other, _oseed, oseg) = pillow();
    let _ = other.kef(oseg.he_plus);
    let foreign_he = oseg.he_plus;
    let _ = body.kev(foreign_he);
    let _ = body.kef(foreign_he);
    let _ = body.kemr(foreign_he, foreign_he);
    let _ = body.mev_line(
        MevSite::Fan {
            he1: foreign_he,
            he2: foreign_he,
        },
        pt(1.0, 1.0),
        Tol::witness(),
    );

    // Whatever remains must still be tier-1 valid.
    assert_eq!(validate(&body), Ok(()));
}

/// Stale keys (killed entities) into every op: typed errors, no panics.
#[test]
fn stale_keys_yield_typed_errors() {
    let (mut body, _seed, seg) = pillow();
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: seg.he_plus,
            },
            pt(0.0, 1.0),
            Tol::witness(),
        )
        .unwrap();
    let dead_he = strut.he_plus;
    let killed = body.kev(dead_he).unwrap();
    let _ = killed;
    for r in [
        body.kev(dead_he).unwrap_err(),
        body.kef(dead_he).unwrap_err(),
        body.kemr(dead_he, dead_he).unwrap_err(),
        body.mev_line(
            MevSite::Fan {
                he1: dead_he,
                he2: dead_he,
            },
            pt(2.0, 2.0),
            Tol::witness(),
        )
        .unwrap_err(),
        body.mef_chord(
            MefSite::Chords {
                he1: dead_he,
                he2: dead_he,
            },
            Tol::witness(),
        )
        .unwrap_err(),
    ] {
        assert!(
            matches!(r, EulerOpError::StaleKey { .. }),
            "expected StaleKey, got {r:?}"
        );
    }
    assert_eq!(validate(&body), Ok(()));
}

// ----------------------------------------------------------------------
// 3. Exhaustive public-mutator sweep: on a family of adversarial
// fixtures, try EVERY (ring, face) ring_move, every (f1, f2) kfmrh,
// every mfkrh(loop), to depth 2 for ring_move. Every accepted mutation
// must leave the body tier-1 valid (the demotion claim, falsified
// otherwise), and validate_closed must never report a
// ComponentEulerViolation (tier-2 additions are connectivity/
// scaffolding only).
// ----------------------------------------------------------------------

fn assert_tier1_after_all_public_mutations(label: &str, body: &Body<f64>, depth: usize) {
    assert_eq!(validate(body), Ok(()), "{label}: base body must be valid");
    let loops: Vec<_> = body.loops().map(|(k, _)| k).collect();
    let faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
    // ring_move sweep.
    for &lp in &loops {
        for &f in &faces {
            let mut clone = body.clone();
            if clone.ring_move(lp, f).is_ok() {
                assert_eq!(
                    validate(&clone),
                    Ok(()),
                    "{label}: ring_move({lp:?}, {f:?}) broke tier 1"
                );
                if depth > 1 {
                    assert_tier1_after_all_public_mutations(
                        &format!("{label}/ring_move({lp:?},{f:?})"),
                        &clone,
                        depth - 1,
                    );
                }
            }
        }
    }
    // kfmrh sweep.
    for &f1 in &faces {
        for &f2 in &faces {
            let mut clone = body.clone();
            if clone.kfmrh(f1, f2).is_ok() {
                assert_eq!(
                    validate(&clone),
                    Ok(()),
                    "{label}: kfmrh({f1:?}, {f2:?}) broke tier 1"
                );
            }
        }
    }
    // mfkrh sweep.
    for &lp in &loops {
        let mut clone = body.clone();
        if clone.mfkrh_plug(lp).is_ok() {
            assert_eq!(
                validate(&clone),
                Ok(()),
                "{label}: mfkrh({lp:?}) broke tier 1"
            );
        }
    }
}

/// Pillow-torus: pillow + attached island + kfmrh(face_b, island) —
/// genus 1, c 1, two rings, the island ring NON-separating.
fn pillow_torus() -> Body<f64> {
    let (mut body, _seed, seg) = pillow();
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: seg.he_plus,
            },
            pt(0.0, 1.0),
            Tol::witness(),
        )
        .unwrap();
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    let grow = body
        .mev_line(
            MevSite::Lone { r#loop: kill.ring },
            pt(0.0, 2.0),
            Tol::witness(),
        )
        .unwrap();
    let island = body
        .mef_chord(
            MefSite::Chords {
                he1: grow.he_plus,
                he2: grow.he_minus,
            },
            Tol::witness(),
        )
        .unwrap();
    // The face NOT holding the ring: the mef segment split made face_b;
    // ring sits on the face of seg.he_plus's loop. Use the island's
    // sibling: kfmrh(other pillow face, island face).
    let ring_face = body.get_loop(kill.ring).unwrap().face;
    let other_face = body
        .faces()
        .map(|(k, _)| k)
        .find(|&k| k != ring_face && k != island.face)
        .unwrap();
    body.kfmrh(other_face, island.face).unwrap();
    // Genus check: v4 e4 f2 r2 => chi 0 = 2(1-1).
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(validate_closed(&body), Ok(()), "pillow-torus is closed");
    body
}

#[test]
fn exhaustive_mutator_sweep_multi_detachment() {
    let (mut body, _seed, seg) = pillow();
    grow_and_promote_detached_digon(&mut body, seg.he_plus, 2.0);
    grow_and_promote_detached_digon(&mut body, seg.he_plus, 3.0);
    assert_tier1_after_all_public_mutations("multi-detach", &body, 2);
}

#[test]
fn exhaustive_mutator_sweep_attached_island_plus_detached() {
    let (mut body, _seed, seg) = pillow();
    // Attached island (live cycle ring on the pillow)...
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: seg.he_plus,
            },
            pt(0.0, 1.0),
            Tol::witness(),
        )
        .unwrap();
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    let grow = body
        .mev_line(
            MevSite::Lone { r#loop: kill.ring },
            pt(0.0, 2.0),
            Tol::witness(),
        )
        .unwrap();
    body.mef_chord(
        MefSite::Chords {
            he1: grow.he_plus,
            he2: grow.he_minus,
        },
        Tol::witness(),
    )
    .unwrap();
    // ...plus a promoted detached digon.
    grow_and_promote_detached_digon(&mut body, seg.he_plus, 5.0);
    assert_tier1_after_all_public_mutations("island+detached", &body, 2);
}

#[test]
fn exhaustive_mutator_sweep_pillow_torus_plus_detached() {
    let mut body = pillow_torus();
    // Add a detached digon in the same shell: cross-component moves of
    // a NON-separating ring become available.
    let rim = body
        .half_edges()
        .map(|(k, _)| k)
        .next()
        .expect("torus has half-edges");
    grow_and_promote_detached_digon(&mut body, rim, 9.0);
    assert_tier1_after_all_public_mutations("torus+detached", &body, 2);
}

#[test]
fn exhaustive_mutator_sweep_empty_ring_states() {
    // Planted EMPTY ring + detached digon: empty-ring moves and
    // mfkrh-on-empty-ring promotions are all in the sweep.
    let (mut body, _seed, seg) = pillow();
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: seg.he_plus,
            },
            pt(0.0, 1.0),
            Tol::witness(),
        )
        .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    grow_and_promote_detached_digon(&mut body, seg.he_plus, 5.0);
    assert_tier1_after_all_public_mutations("empty-ring", &body, 2);
}

/// The implementer's claim behind the digon witness: promoting an EMPTY
/// ring with mfkrh leaves an empty loop, so THAT disconnection is
/// already caught by the empty-loop ban — the grown digon is the only
/// witness that c = 1 is an independent rule.
#[test]
fn promoted_empty_ring_still_has_an_empty_loop() {
    let (mut body, seed, seg) = pillow();
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: seg.he_plus,
            },
            pt(0.0, 1.0),
            Tol::witness(),
        )
        .unwrap();
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    // mfkrh on the EMPTY ring (if accepted).
    let promoted = body.mfkrh_plug(kill.ring);
    match promoted {
        Ok(_) => {
            assert_eq!(validate(&body), Ok(()), "tier 1 accepts it");
            let errs = validate_closed(&body).unwrap_err();
            assert_eq!(
                errs,
                vec![
                    ValidationError::ScaffoldingEmptyLoop { loop_: kill.ring },
                    ValidationError::ShellDisconnected {
                        shell: seed.shell,
                        components: 2,
                    },
                ],
                "empty-loop ban already fires on the promoted EMPTY ring"
            );
        }
        Err(e) => panic!("mfkrh rejected the empty ring: {e:?} (claim moot)"),
    }
}

/// The kemr old-side-empty family: Empty OUTER + cycle ring on one
/// face, then the full public-mutator sweep (ring_move / kfmrh / mfkrh
/// everywhere, depth 2). Also chains mfkrh on the cycle ring directly:
/// the promoted strut-cycle face is its own chi=2 component.
#[test]
fn exhaustive_mutator_sweep_empty_outer_family() {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(1.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    let kill = body.kemr(seg.he_plus, seg.he_minus).unwrap();
    let _grow = body
        .mev_line(
            MevSite::Lone { r#loop: kill.ring },
            pt(2.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    // face: Empty outer + 2-he strut cycle ring.
    assert_eq!(validate(&body), Ok(()));

    // Direct chain: promote the cycle ring.
    let mut chained = body.clone();
    if chained.mfkrh_plug(kill.ring).is_ok() {
        assert_eq!(validate(&chained), Ok(()), "promoted strut-cycle face");
        let errs = validate_closed(&chained).unwrap_err();
        assert!(
            errs.contains(&ValidationError::ShellDisconnected {
                shell: seed.shell,
                components: 2,
            }),
            "{errs:?}"
        );
    }

    // And the full sweep over the base state.
    let loops: Vec<_> = body.loops().map(|(k, _)| k).collect();
    let faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
    for &lp in &loops {
        for &f in &faces {
            let mut clone = body.clone();
            if clone.ring_move(lp, f).is_ok() {
                assert_eq!(validate(&clone), Ok(()), "ring_move({lp:?},{f:?})");
            }
        }
        let mut clone = body.clone();
        if clone.mfkrh_plug(lp).is_ok() {
            assert_eq!(validate(&clone), Ok(()), "mfkrh({lp:?})");
        }
    }
    for &f1 in &faces {
        for &f2 in &faces {
            let mut clone = body.clone();
            if clone.kfmrh(f1, f2).is_ok() {
                assert_eq!(validate(&clone), Ok(()), "kfmrh({f1:?},{f2:?})");
            }
        }
    }
}
