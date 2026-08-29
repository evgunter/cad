//! **F7 pole-exemption R1 review probes (ordinal 104, PR #1131)** —
//! claims-to-falsify, attack fixtures for `pole_split_cap`.
//!
//! **ADOPTED (VERBS/F7), and RE-POLARISED.** These were written to
//! falsify a gate exemption that has since been WITHDRAWN — the
//! attacks succeeded, which is why it was. R1's fixture geometry and
//! reasoning are preserved verbatim; what changed is the assertions,
//! which now pin the behaviour the fixtures actually produce: every
//! one of these bent/ordinary shapes REFUSES `NonMaximalFaces`, and
//! that is what makes them the negative differential rows for the
//! repair op's collinearity trigger (`merge_faces::
//! redundant_subdivision_vertex`). The positive pole of that
//! differential is `verbs_f7_collinear_seam` and, for a real revolve
//! cap, `sweep`'s `f7_pole_split_cap_repairs_to_one_face`.
//! Every fixture here is HAND-BUILT via public euler ops — no revolve
//! anywhere — so what these rows measure is the structural predicate
//! itself, divorced from the producer whose shape motivated it.
//!
//! The claim under attack (reduce.rs doc, "argued rather than
//! asserted"): *"a pair that should have been merged shares a boundary
//! somewhere away from any pole, and that edge's endpoints carry the
//! pair's other neighbours, so it has no valence-2 same-pair endpoint
//! and refuses on its own account."* That argument silently assumes
//! the shared boundary chain has no interior valence-2 vertex. The
//! probes below construct exactly that chain.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism_z;
use geom_core::Tol;
use topo::{
    Body, BooleanError, BooleanOp, FaceSurface, MefSite, MekrSite, MevSite, boolean_reduce,
    validate_closed,
};

/// A brick far away from every fixture, so `gate_operand_pairs`' boxes
/// never meet and the reduction of a PASSING pair is the trivial
/// disjoint one — the refusal (or its absence) is the gates' own
/// signal, uncontaminated by contact machinery.
fn distant_brick() -> Body<f64> {
    prism_z::<f64>(
        &[(50.0, 50.0), (51.0, 50.0), (51.0, 51.0), (50.0, 51.0)],
        50.0,
        51.0,
    )
    .body
}

/// The half-edge of `face`'s outer loop starting at the vertex whose
/// point is (x, y, z).
fn he_at(body: &Body<f64>, face: topo::FaceKey, x: f64, y: f64, z: f64) -> topo::HalfEdgeKey {
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("outer loop is not a cycle")
    };
    let cycle = body.loop_cycle(first).unwrap();
    *cycle
        .iter()
        .find(|&&he| {
            let v = body.get_half_edge(he).unwrap().start;
            let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            (p.x - x).abs() < 1e-12 && (p.y - y).abs() < 1e-12 && (p.z - z).abs() < 1e-12
        })
        .unwrap_or_else(|| panic!("no half-edge starts at ({x}, {y}, {z})"))
}

/// **P1 (control, the teapot-cup shape one dimension down): a single
/// full-valence chord between two same-plane-key faces still refuses.**
/// This is `m3_pr4_boolean::non_maximal_operand_refuses` restated in
/// this file so the differential against P2 is one screen tall: the
/// chord's endpoints have valence 3, no valence-2 same-pair endpoint
/// exists, and the gate refuses exactly as before the exemption.
#[test]
fn p1_single_chord_pair_still_refuses() {
    let p = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 0.0, 1.0);
    let mut b = p.body;
    let tol = Tol::witness();
    let he1 = he_at(&b, p.top_face, 0.0, 0.0, 1.0);
    let he2 = he_at(&b, p.top_face, 2.0, 2.0, 1.0);
    b.mef_chord(MefSite::Chords { he1, he2 }, tol).unwrap();
    assert_eq!(validate_closed(&b), Ok(()), "fixture is tier-2 legal");
    let err = boolean_reduce(BooleanOp::Union, &distant_brick(), &b, tol).unwrap_err();
    assert!(
        matches!(err, BooleanError::NonMaximalFaces { .. }),
        "control must refuse — got {err:?}"
    );
}

/// **P2 (attack): the same mergeable pair with its chord SUBDIVIDED
/// once slips the gate.** Chain V0 → P → V2 where P is an interior
/// valence-2 vertex: every shared edge now has a valence-2 endpoint
/// both of whose edges separate the same pair, so `pole_split_cap`
/// admits each of them, and the pair the F7 rule exists for — two
/// genuinely coplanar faces the producing construction should have
/// merged — is no longer refused. No revolve, no pole, no axis: the
/// body is the P1 control with one extra vertex on the cut.
///
/// This is the doc comment's own "honest residue" made concrete and
/// present-tense: the site frames the slip-through as reachable by
/// "some future producer"; plain euler ops reach it today.
#[test]
fn p2_subdivided_chord_pair_still_refuses() {
    let p = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 0.0, 1.0);
    let mut b = p.body;
    let tol = Tol::witness();
    let he1 = he_at(&b, p.top_face, 0.0, 0.0, 1.0);
    // Strut V0 → P, P interior to the top face (NOT on segment V0–V2,
    // so the chain is a genuine bent cut, not a degenerate straight
    // edge split).
    let strut = b
        .mev_line(
            MevSite::Fan { he1, he2: he1 },
            geom_core::Point3::new(0.9, 0.6, 1.0),
            tol,
        )
        .unwrap();
    // Chord P → V2 closes the cut and splits the top face.
    let he2 = he_at(&b, p.top_face, 2.0, 2.0, 1.0);
    b.mef_chord(
        MefSite::Chords {
            he1: strut.he_minus,
            he2,
        },
        tol,
    )
    .unwrap();
    assert_eq!(validate_closed(&b), Ok(()), "fixture is tier-2 legal");
    let err = boolean_reduce(BooleanOp::Union, &distant_brick(), &b, tol)
        .expect_err("a BENT subdivided chord is an ordinary non-maximal pair");
    println!("[p2] subdivided (bent) chord operand => {err:?}");
    assert!(
        matches!(err, BooleanError::NonMaximalFaces { .. }),
        "the gate must catch the pair the F7 rule exists for — got {err:?}"
    );
}

/// Builds the prism whose top face carries an inset coplanar PATCH:
/// ring planted (strut + kemr), grown P→Q→R→S, closed by a mef whose
/// membrane inherits the TOP face's own plane key. The membrane and
/// the top face are two same-plane-key faces adjacent across all four
/// ring edges, and all four ring vertices have valence 2.
fn inset_patch_prism() -> (
    Body<f64>,
    topo::FaceKey,        // top face
    [topo::VertexKey; 4], // P, Q, R, S
    topo::LoopKey,        // the ring (dead after a later mekr)
) {
    let p = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 0.0, 1.0);
    let mut b = p.body;
    let tol = Tol::witness();
    let pt = geom_core::Point3::new;
    // (f) strut from corner (0,0) to P(0.5, 0.5).
    let he_a = he_at(&b, p.top_face, 0.0, 0.0, 1.0);
    let strut = b
        .mev_line(
            MevSite::Fan {
                he1: he_a,
                he2: he_a,
            },
            pt(0.5, 0.5, 1.0),
            tol,
        )
        .unwrap();
    // (g) kill it: P is an empty ring of the top face.
    let kill = b.kemr(strut.he_plus, strut.he_minus).unwrap();
    // (h) grow P→Q→R→S.
    let s_pq = b
        .mev_line(MevSite::Lone { r#loop: kill.ring }, pt(1.5, 0.5, 1.0), tol)
        .unwrap(); // Q
    let s_qr = b
        .mev_line(
            MevSite::Fan {
                he1: s_pq.he_minus,
                he2: s_pq.he_minus,
            },
            pt(1.5, 1.5, 1.0),
            tol,
        )
        .unwrap(); // R
    let s_rs = b
        .mev_line(
            MevSite::Fan {
                he1: s_qr.he_minus,
                he2: s_qr.he_minus,
            },
            pt(0.5, 1.5, 1.0),
            tol,
        )
        .unwrap(); // S
    // (i) close: the ring becomes the patch rim; the membrane face
    // inherits the top face's surface key (mef_chord ⇒ Inherit).
    b.mef_chord(
        MefSite::Chords {
            he1: s_pq.he_plus,
            he2: s_rs.he_minus,
        },
        tol,
    )
    .unwrap();
    (
        b,
        p.top_face,
        [strut.vertex, s_pq.vertex, s_qr.vertex, s_rs.vertex],
        kill.ring,
    )
}

/// **P3 (attack #2, a different predicate path): a coplanar INSET
/// PATCH — a face and a same-plane-key face covering a hole in it,
/// adjacent across a closed ring all of whose vertices are valence
/// 2 — slips the gate.** Here the valence-2 vertices are not chain
/// interiors but the ring's own corners, and each ring edge is
/// admitted through a DIFFERENT vertex's orbit. The pair is exactly
/// the "should have been merged" defect (the patch is two regions of
/// one plane, sharing every boundary edge), and nothing about it is a
/// pole, a revolve, or an axis.
#[test]
fn p3_inset_coplanar_patch_still_refuses() {
    let (b, _top, _psrq, _ring) = inset_patch_prism();
    let tol = Tol::witness();
    assert_eq!(validate_closed(&b), Ok(()), "fixture is tier-2 legal");
    let err = boolean_reduce(BooleanOp::Union, &distant_brick(), &b, tol)
        .expect_err("an inset coplanar patch is an ordinary non-maximal pair");
    println!("[p3] inset-patch operand => {err:?}");
    assert!(
        matches!(err, BooleanError::NonMaximalFaces { .. }),
        "the gate must catch the inset patch — got {err:?}"
    );
}

/// **P4 (the brief's differential): a pair sharing BOTH a pole-like
/// valence-2 chain AND an ordinary full-valence edge still refuses,
/// on the ordinary edge.** The inset-patch top face is bridged to its
/// ring (mekr: an ordinary edge, both endpoints valence ≥ 3) and then
/// cut a second time by a chain with an interior valence-2 vertex
/// (strut + mef). The two resulting faces share the bridge AND the
/// chain; per-edge admission means the exempt chain does not save the
/// pair, and the refusal names the bridge edge specifically.
#[test]
fn p4_mixed_pair_refuses() {
    let (mut b, top, [pv, _qv, rv, _sv], ring) = inset_patch_prism();
    let tol = Tol::witness();
    let pt = geom_core::Point3::new;
    // Bridge corner (0,0) → P: joins the ring into the outer loop.
    let target = he_at(&b, top, 0.0, 0.0, 1.0);
    let ring_he = {
        let topo::LoopBoundary::Cycle { first } = b.get_loop(ring).unwrap().boundary else {
            panic!("ring did not grow into a cycle")
        };
        let cycle = b.loop_cycle(first).unwrap();
        *cycle
            .iter()
            .find(|&&he| b.get_half_edge(he).unwrap().start == pv)
            .expect("a ring half-edge starts at P")
    };
    let bridge = b
        .mekr_chord(
            MekrSite::Cycles {
                target,
                ring: ring_he,
            },
            tol,
        )
        .unwrap();
    // Second cut, subdivided: corner (2,2) → M → R.
    let he_c = he_at(&b, top, 2.0, 2.0, 1.0);
    let strut2 = b
        .mev_line(
            MevSite::Fan {
                he1: he_c,
                he2: he_c,
            },
            pt(1.8, 1.7, 1.0),
            tol,
        )
        .unwrap(); // M
    let he_r = {
        let outer = b.get_face(top).unwrap().outer;
        let topo::LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
            panic!("top outer loop is not a cycle")
        };
        let cycle = b.loop_cycle(first).unwrap();
        *cycle
            .iter()
            .find(|&&he| b.get_half_edge(he).unwrap().start == rv)
            .expect("a top-loop half-edge starts at R after the bridge")
    };
    b.mef(
        MefSite::Chords {
            he1: strut2.he_minus,
            he2: he_r,
        },
        common::line(pt(1.8, 1.7, 1.0), pt(1.5, 1.5, 1.0)),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    assert_eq!(validate_closed(&b), Ok(()), "fixture is tier-2 legal");
    let err = boolean_reduce(BooleanOp::Union, &distant_brick(), &b, tol).unwrap_err();
    // R1's original row demanded the BRIDGE edge by name. That was a
    // consequence of the exemption: with the chain edges exempt, only
    // the bridge could fire. With the exemption withdrawn no edge is
    // exempt, so the gate names the FIRST offender in arena order —
    // measured as a chain edge here. What the row pins is unchanged in
    // substance: a pair sharing an ordinary edge refuses, whatever
    // else it shares.
    match err {
        BooleanError::NonMaximalFaces { edge, .. } => {
            println!(
                "[p4] mixed pair refused at {edge:?} (bridge is {:?})",
                bridge.edge
            );
        }
        other => panic!(
            "a pair sharing an ordinary edge must refuse NonMaximalFaces \
             whatever else it shares — got {other:?}"
        ),
    }
}
