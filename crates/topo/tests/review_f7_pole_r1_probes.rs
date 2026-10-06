//! **F7 pole-exemption R1 review probes (ordinal 104, PR #1131)** —
//! claims-to-falsify, attack fixtures for the WITHDRAWN gate
//! exemption `pole_split_cap` (see the adoption note below).
//!
//! **ADOPTED (VERBS/F7), and RE-POLARISED.** These were written to
//! falsify a gate exemption that has since been WITHDRAWN — the
//! attacks succeeded, which is why it was. R1's fixture geometry and
//! reasoning are preserved; what changed is the assertions,
//! which pin the behaviour the fixtures actually produce: left as the
//! split makes them, every one of these bent/ordinary shapes is
//! refused by the at-rest gate, with one `ScaffoldAtRest` for each
//! seam edge between the same-plane-key pair and nothing else; with
//! each seam at rest in the shared plane's chart, which holds it
//! exactly, the body finishes, and the boolean's maximal-faces gate
//! refuses it `NonMaximalFaces` on a seam, through the public union
//! and the public reduction. Repairing such a body is the caller's explicit
//! `merge_coplanar_faces` (for a real revolve cap, `sweep`'s
//! `f7_pole_split_cap_repairs_to_one_face`).
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

use crate::common;

use common::{brick, finished, plant_ring_face, prism_z};
use geom_core::Tol;
use topo::{
    AtRestBody, Body, BooleanError, BooleanOp, FaceSurface, MefSite, MekrSite, MevSite,
    ValidationError, boolean_reduce, validate_closed,
};

/// A brick far away from every fixture, so `gate_operand_pairs`' boxes
/// never meet and the reduction of a PASSING pair is the trivial
/// disjoint one — the refusal (or its absence) is the gates' own
/// signal, uncontaminated by contact machinery.
fn distant_brick() -> AtRestBody<f64> {
    let tol = Tol::witness();
    finished(
        "the distant brick",
        brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol),
        tol,
    )
}

/// The edges separating two faces on one surface key — the seams a
/// split with an inherited surface leaves — in edge-arena order.
fn common_plane_seams(b: &Body<f64>) -> Vec<topo::EdgeKey> {
    let face_of = |he| {
        b.get_loop(b.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    b.edges()
        .filter(|(_, e)| {
            let (fp, fm) = (face_of(e.he_plus), face_of(e.he_minus));
            fp != fm && b.get_face(fp).unwrap().surface == b.get_face(fm).unwrap().surface
        })
        .map(|(edge, _)| edge)
        .collect()
}

/// `b`'s at-rest refusal, as the split leaves it, is exactly one
/// `ScaffoldAtRest` per seam edge of its same-plane-key pairs, `seams`
/// of them, and nothing else; with every seam at rest in the shared
/// plane's chart it finishes, and the public union and the public
/// reduction with a distant brick refuse it `NonMaximalFaces` on one of
/// those seams.
fn assert_refused_on_its_seams(b: Body<f64>, seams: usize, what: &str) -> Vec<topo::EdgeKey> {
    let tol = Tol::witness();
    let edges = common_plane_seams(&b);
    assert_eq!(edges.len(), seams, "{what}: seam-edge census");
    let mut rested = b.clone();
    for &edge in &edges {
        let e = rested.get_edge(edge).unwrap();
        let at = |he| {
            let v = rested.get_half_edge(he).unwrap().start;
            *rested
                .get_point(rested.get_vertex(v).unwrap().point)
                .unwrap()
        };
        let (p0, p1) = (at(e.he_plus), at(e.he_minus));
        let face = rested
            .get_loop(rested.get_half_edge(e.he_plus).unwrap().parent_loop)
            .unwrap()
            .face;
        let chart = rested.get_face(face).unwrap().surface;
        rested
            .set_edge_curve(
                edge,
                geom_brep::EdgeCurveSpec::line_between(p0, p1).at_rest_in_chart(chart, false),
                tol,
            )
            .unwrap_or_else(|e| panic!("{what}: the seam rests in its plane's chart: {e:?}"));
    }
    let operand = finished(what, rested, tol);
    for (door, err) in [
        (
            "union",
            topo::union(&distant_brick(), &operand, tol)
                .map(|_| ())
                .unwrap_err(),
        ),
        (
            "boolean_reduce",
            boolean_reduce(BooleanOp::Union, &distant_brick(), &operand, tol)
                .map(|_| ())
                .unwrap_err(),
        ),
    ] {
        println!("{what} {door} => {err:?}");
        assert!(
            matches!(
                err,
                BooleanError::NonMaximalFaces { operand: topo::Operand::B, edge }
                    if edges.contains(&edge)
            ),
            "{what}, {door}: the gate refuses on a seam: {err:?}"
        );
    }
    let errors = AtRestBody::validate(b, Tol::witness()).expect_err(&format!(
        "{what} is an ordinary non-maximal pair and does not finish as the split leaves it"
    ));
    println!("{what} => {errors:?}");
    assert_eq!(
        errors,
        edges
            .iter()
            .map(|&edge| ValidationError::ScaffoldAtRest { edge })
            .collect::<Vec<_>>(),
        "{what}: the at-rest gate refuses on every seam edge, and on nothing else"
    );
    edges
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
/// exists, and the gate refuses. (The "exemption" these rows were
/// written against was WITHDRAWN; what ships is a repair in
/// `merge_coplanar_faces`, and the gate is unchanged — so this control
/// and its siblings pin the gate.)
#[test]
fn p1_single_chord_pair_still_refuses() {
    let p = prism_z::<f64>(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        0.0,
        1.0,
        Tol::witness(),
    );
    let mut b = p.body;
    let tol = Tol::witness();
    let he1 = he_at(&b, p.top_face, 0.0, 0.0, 1.0);
    let he2 = he_at(&b, p.top_face, 2.0, 2.0, 1.0);
    b.mef_chord(MefSite::Chords { he1, he2 }, tol).unwrap();
    assert_eq!(validate_closed(&b), Ok(()), "fixture is tier-2 legal");
    assert_refused_on_its_seams(b, 1, "[p1] single-chord operand");
}

/// **P2 (attack): the same mergeable pair with its chord SUBDIVIDED
/// once slips the gate.** Chain V0 → P → V2 where P is an interior
/// valence-2 vertex: every shared edge now has a valence-2 endpoint
/// both of whose edges separate the same pair, so the withdrawn
/// `pole_split_cap`
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
    let p = prism_z::<f64>(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        0.0,
        1.0,
        Tol::witness(),
    );
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
    // The gate must catch the pair the F7 rule exists for, on both
    // halves of the bent chord.
    assert_refused_on_its_seams(b, 2, "[p2] subdivided (bent) chord operand");
}

/// Builds the prism whose top face carries an inset coplanar PATCH:
/// ring planted (strut + kemr), grown P→Q→R→S, closed by a mef whose
/// membrane inherits the TOP face's own plane key
/// ([`common::plant_ring_face`], anchored at the top face's corner
/// (0, 0)). The membrane and the top face are two same-plane-key faces
/// adjacent across all four ring edges, and all four ring vertices have
/// valence 2.
fn inset_patch_prism() -> (
    Body<f64>,
    topo::FaceKey,        // top face
    [topo::VertexKey; 4], // P, Q, R, S
    topo::LoopKey,        // the ring (dead after a later mekr)
) {
    let p = prism_z::<f64>(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        0.0,
        1.0,
        Tol::witness(),
    );
    let mut b = p.body;
    let he_a = he_at(&b, p.top_face, 0.0, 0.0, 1.0);
    let ring = plant_ring_face(
        &mut b,
        he_a,
        &[
            geom_core::Point3::new(0.5, 0.5, 1.0),
            geom_core::Point3::new(1.5, 0.5, 1.0),
            geom_core::Point3::new(1.5, 1.5, 1.0),
            geom_core::Point3::new(0.5, 1.5, 1.0),
        ],
        Tol::witness(),
    );
    (
        b,
        p.top_face,
        [
            ring.strut.vertex,
            ring.rim[0].vertex,
            ring.rim[1].vertex,
            ring.rim[2].vertex,
        ],
        ring.kill.ring,
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
    assert_eq!(validate_closed(&b), Ok(()), "fixture is tier-2 legal");
    // The gate must catch the inset patch, on all four ring edges.
    assert_refused_on_its_seams(b, 4, "[p3] inset-patch operand");
}

/// **P4 (the brief's differential): a pair sharing BOTH a pole-like
/// valence-2 chain AND an ordinary full-valence edge still refuses,
/// on the ordinary edge.** The inset-patch top face is bridged to its
/// ring (mekr: an ordinary edge, both endpoints valence ≥ 3) and then
/// cut a second time by a chain with an interior valence-2 vertex
/// (strut + mef). The two resulting faces share the bridge AND the
/// chain; per-edge admission means the exempt chain does not save the
/// pair, and the refusal names the bridge edge among the seam edges.
#[test]
fn p4_mixed_pair_refuses() {
    let (mut b, top, [pv, _qv, rv, _sv], ring) = inset_patch_prism();
    let tol = Tol::witness();
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
            geom_core::Point3::new(1.8, 1.7, 1.0),
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
        common::line(
            geom_core::Point3::new(1.8, 1.7, 1.0),
            geom_core::Point3::new(1.5, 1.5, 1.0),
        ),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    assert_eq!(validate_closed(&b), Ok(()), "fixture is tier-2 legal");
    // Every edge the pair shares refuses — the four ring edges, the
    // bridge and the two chain edges — so a pair sharing an ordinary
    // edge refuses on it, whatever else it shares.
    let seams = assert_refused_on_its_seams(b, 7, "[p4] mixed pair");
    assert!(
        seams.contains(&bridge.edge),
        "the ordinary bridge edge {:?} is among the refusals {seams:?}",
        bridge.edge
    );
}
