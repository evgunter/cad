//! Adversarial e2e review artifact for M1 PR 3 (2026-07-16), promoted
//! into the shipped suite per the standing convention
//! (`memories/review-and-dependency-policy.md`): reviewers write and run
//! real consumer programs against the API under review, and the
//! programs are kept.
//!
//! Everything here goes through the public API. The derivations
//! (ledgers, anchor rules, orbit orders, slot/generation semantics) are
//! **independent re-derivations by the reviewer — do not "simplify"
//! them to match the implementation's comments**; their value is
//! exactly that they were computed from Mäntylä ch. 9/11 and the
//! ratified conventions without reading the surgery code. Only exact
//! duplicates of shipped unit tests were dropped at promotion;
//! spirit-overlaps are deliberate redundancy.
//!
//! Highlights: an independently-routed triangular side-face through-hole
//! (genus 1), the project's first genus-2 body (double hole, ledger
//! −2), per-arena pair-convergence and unbalanced-kill divergence pins,
//! shared-vs-private geometry reaping, and atomicity sweeps over every
//! ring-op error path.
//!
//! **Moved from `tests/` into `src/` (cfg(test)) at M1 PR 5**, when the
//! raw builder retreated to `pub(crate)`: two probes here
//! (`raw_self_loop`, `raw_shared_curve_chain`) deliberately raw-build
//! states no operator can reach, which integration tests can no longer
//! do. Adaptations at the move: `use topo::…` became `use crate::…`;
//! the probes are otherwise verbatim (the "everything here goes through
//! the public API" claim above is historical — true when written,
//! qualified by exactly this note now).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::{
    Body, Edge, EntityId, EulerOpError, Face, FaceKey, HalfEdge, HalfEdgeKey, Loop, LoopBoundary,
    LoopKey, MefCreated, MefSite, MekrSite, MevCreated, MevSite, MvfsCreated, Provenance, Shell,
    Solid, Vertex, VertexKey, validate,
};
use geom_core::Point3;
use geom_core::Tol;

// ---------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------

/// Deep public-API snapshot (keys + payloads + provenance, all arenas).
fn snapshot(body: &Body<f64>) -> Vec<String> {
    let mut lines = Vec::new();
    for (k, e) in body.solids() {
        lines.push(format!(
            "{k:?}:{e:?}:{:?}",
            body.provenance(EntityId::Solid(k))
        ));
    }
    for (k, e) in body.shells() {
        lines.push(format!(
            "{k:?}:{e:?}:{:?}",
            body.provenance(EntityId::Shell(k))
        ));
    }
    for (k, e) in body.faces() {
        lines.push(format!(
            "{k:?}:{e:?}:{:?}",
            body.provenance(EntityId::Face(k))
        ));
    }
    for (k, e) in body.loops() {
        lines.push(format!(
            "{k:?}:{e:?}:{:?}",
            body.provenance(EntityId::Loop(k))
        ));
    }
    for (k, e) in body.half_edges() {
        lines.push(format!(
            "{k:?}:{e:?}:{:?}",
            body.provenance(EntityId::HalfEdge(k))
        ));
    }
    for (k, e) in body.edges() {
        lines.push(format!(
            "{k:?}:{e:?}:{:?}",
            body.provenance(EntityId::Edge(k))
        ));
    }
    for (k, e) in body.vertices() {
        lines.push(format!(
            "{k:?}:{e:?}:{:?}",
            body.provenance(EntityId::Vertex(k))
        ));
    }
    for (k, e) in body.points() {
        lines.push(format!("{k:?}:{e:?}"));
    }
    for (k, e) in body.curves() {
        lines.push(format!("{k:?}:{e:?}"));
    }
    for (k, e) in body.surfaces() {
        lines.push(format!("{k:?}:{e:?}"));
    }
    lines
}

/// Assert `op` fails with exactly `expected` and the body is deeply
/// untouched.
fn assert_err_unchanged(
    body: &mut Body<f64>,
    expected: &EulerOpError,
    op: impl FnOnce(&mut Body<f64>) -> EulerOpError,
) {
    let before = snapshot(body);
    let err = op(body);
    assert_eq!(&err, expected);
    assert_eq!(snapshot(body), before, "body changed on Err");
}

/// The five Euler–Poincaré components genus is derived FROM, counted
/// off the body. Genus itself is deliberately NOT a field: it is the
/// quantity under test here, and carrying it would make [`genus`]
/// tautological. (The crate's running six-component ledger, which does
/// track `h`, is [`crate::seqgen::Ledger`].)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct GenusInputs {
    v: i64,
    e: i64,
    f: i64,
    r: i64,
    s: i64,
}

fn genus_inputs(body: &Body<f64>) -> GenusInputs {
    GenusInputs {
        v: body.vertices().count() as i64,
        e: body.edges().count() as i64,
        f: body.faces().count() as i64,
        r: body.faces().map(|(_, face)| face.rings.len() as i64).sum(),
        s: body.shells().count() as i64,
    }
}

/// Derived genus: v − e + f − r = 2(s − h)  ⇒  h = s − (v−e+f−r)/2.
fn genus(l: GenusInputs) -> i64 {
    let x = l.v - l.e + l.f - l.r;
    assert_eq!(x.rem_euclid(2), 0, "E–P parity violated: {l:?}");
    l.s - x / 2
}

fn starts(body: &Body<f64>, he: HalfEdgeKey) -> Vec<VertexKey> {
    body.loop_cycle(he)
        .unwrap()
        .into_iter()
        .map(|m| body.get_half_edge(m).unwrap().start)
        .collect()
}

fn check(body: &Body<f64>, expect: GenusInputs, h: i64) {
    assert_eq!(validate(body), Ok(()));
    let l = genus_inputs(body);
    assert_eq!(l, expect);
    assert_eq!(genus(l), h);
}

// ---------------------------------------------------------------------
// The independent genus-1 / genus-2 build: 2×2×2 box, TRIANGULAR hole
// through the FRONT/BACK faces (different placement, orientation, and
// rim count from the shipped top/bottom square-hole test), then a
// second square hole through TOP/BOTTOM on the same body → genus 2.
// ---------------------------------------------------------------------

#[allow(dead_code)]
struct BoxBuilt {
    seed: MvfsCreated,
    // A B C D bottom, A' B' C' D' top
    e_ab: MevCreated,
    e_bc: MevCreated,
    e_cd: MevCreated,
    e_aa: MevCreated,
    e_bb: MevCreated,
    e_cc: MevCreated,
    e_dd: MevCreated,
    f_bottom: MefCreated,
    f_front: MefCreated,
    f_right: MefCreated,
    f_back: MefCreated,
    f_left: MefCreated,
}

/// The 2×2×2 box (cube-test sequence; PR 2 material, re-verified here
/// via loop-walk assertions rather than trusted).
fn build_box(body: &mut Body<f64>, tol: Tol) -> BoxBuilt {
    let pt = Point3::new;
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap(); // A
    let e_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(2.0, 0.0, 0.0),
            tol,
        )
        .unwrap(); // B
    let strut = |body: &mut Body<f64>, at, x, y, z| {
        body.mev_line(MevSite::Fan { he1: at, he2: at }, pt(x, y, z), tol)
            .unwrap()
    };
    let e_bc = strut(body, e_ab.he_minus, 2.0, 2.0, 0.0); // C
    let e_cd = strut(body, e_bc.he_minus, 0.0, 2.0, 0.0); // D
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    let f_bottom = body
        .mef_chord(
            MefSite::Chords {
                he1: he_dc,
                he2: e_ab.he_plus,
            },
            tol,
        )
        .unwrap();
    let e_aa = strut(body, e_ab.he_plus, 0.0, 0.0, 2.0);
    let e_bb = strut(body, e_bc.he_plus, 2.0, 0.0, 2.0);
    let e_cc = strut(body, e_cd.he_plus, 2.0, 2.0, 2.0);
    let e_dd = strut(body, f_bottom.he_plus, 0.0, 2.0, 2.0);
    let mef =
        |body: &mut Body<f64>, he1, he2| body.mef_chord(MefSite::Chords { he1, he2 }, tol).unwrap();
    let f_front = mef(body, e_aa.he_minus, e_bb.he_minus);
    let f_right = mef(body, e_bb.he_minus, e_cc.he_minus);
    let f_back = mef(body, e_cc.he_minus, e_dd.he_minus);
    let f_left = mef(body, e_dd.he_minus, f_front.he_plus);
    check(
        body,
        GenusInputs {
            v: 8,
            e: 12,
            f: 6,
            r: 0,
            s: 1,
        },
        0,
    );

    // Re-verify my reading of the box: top outer A'→B'→C'→D', bottom
    // A→D→C→B, and the A→B half sits in the FRONT face's loop.
    assert_eq!(
        starts(body, f_front.he_plus),
        vec![e_aa.vertex, e_bb.vertex, e_cc.vertex, e_dd.vertex]
    );
    let he_front_ab = body
        .find_half_edge(f_front.face, seed.vertex, e_ab.vertex)
        .expect("front face holds the A→B half");
    assert_eq!(
        body.get_half_edge(he_front_ab).unwrap().parent_loop,
        body.get_face(f_front.face).unwrap().outer
    );

    BoxBuilt {
        seed,
        e_ab,
        e_bc,
        e_cd,
        e_aa,
        e_bb,
        e_cc,
        e_dd,
        f_bottom,
        f_front,
        f_right,
        f_back,
        f_left,
    }
}

#[allow(dead_code)]
struct HoleBuilt {
    strut: MevCreated,
    kill: crate::KemrResult,
    rim: Vec<MevCreated>,
    membrane: MefCreated,
    drops: Vec<MevCreated>,
    walls: Vec<MefCreated>,
    plug: crate::KfmrhResult,
}

/// Carve an n-gon hole from face `f_from` (strut planted at the start
/// vertex of `at`) through to face `f_to`. `rim_pts` are the n rim
/// coordinates on the from-plane; `drop_pts` the n far-plane points.
/// [`GenusInputs`] are asserted after EVERY operator against the
/// caller's running expectation (`l`, mutated in place).
// Promotion adaptation (lint only, reviewer's signature kept verbatim):
// the shipped crate denies clippy::too_many_arguments at 8/7.
#[allow(clippy::too_many_arguments)]
fn carve_hole(
    body: &mut Body<f64>,
    at: HalfEdgeKey,
    f_from: FaceKey,
    f_to: FaceKey,
    rim_pts: &[Point3<f64>],
    drop_pts: &[Point3<f64>],
    l: &mut GenusInputs,
    h: i64,
    tol: Tol,
) -> HoleBuilt {
    let n = rim_pts.len() as i64;
    assert!(n >= 3);

    // Strut, then kemr: the planted empty ring.
    let strut = body
        .mev_line(MevSite::Fan { he1: at, he2: at }, rim_pts[0], tol)
        .unwrap();
    l.v += 1;
    l.e += 1;
    check(body, *l, h);
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    l.e -= 1;
    l.r += 1;
    check(body, *l, h);
    assert_eq!(
        body.get_loop(kill.ring).unwrap().boundary,
        LoopBoundary::Empty {
            vertex: strut.vertex
        }
    );
    assert!(body.get_face(f_from).unwrap().rings.contains(&kill.ring));
    assert_eq!(body.get_vertex(strut.vertex).unwrap().emanating, None);
    // Kill hygiene: dead keys, dead provenance, reaped private curve.
    assert!(body.get_edge(kill.killed_edge).is_none());
    assert!(body.get_half_edge(kill.killed_he_plus).is_none());
    assert!(body.get_half_edge(kill.killed_he_minus).is_none());
    assert_eq!(body.provenance(EntityId::Edge(kill.killed_edge)), None);
    assert_eq!(
        body.provenance(EntityId::HalfEdge(kill.killed_he_plus)),
        None
    );
    assert_eq!(kill.killed_curve, Some(strut.curve));
    assert!(body.get_curve_geom(strut.curve).is_none());

    // Grow the rim chain inside the ring.
    let mut rim: Vec<MevCreated> = Vec::new();
    let first_rim = body
        .mev_line(MevSite::Lone { r#loop: kill.ring }, rim_pts[1], tol)
        .unwrap();
    l.v += 1;
    l.e += 1;
    check(body, *l, h);
    rim.push(first_rim);
    for p in &rim_pts[2..] {
        let prev = rim.last().unwrap().he_minus;
        let next = body
            .mev_line(
                MevSite::Fan {
                    he1: prev,
                    he2: prev,
                },
                *p,
                tol,
            )
            .unwrap();
        l.v += 1;
        l.e += 1;
        check(body, *l, h);
        rim.push(next);
    }

    // Close the rim: membrane face (he1's side) + rim ring (old loop).
    let membrane = body
        .mef_chord(
            MefSite::Chords {
                he1: rim[0].he_plus,
                he2: rim.last().unwrap().he_minus,
            },
            tol,
        )
        .unwrap();
    l.e += 1;
    l.f += 1;
    check(body, *l, h);
    // The ring kept its ring designation on f_from.
    assert!(body.get_face(f_from).unwrap().rings.contains(&kill.ring));
    assert_eq!(body.get_loop(membrane.r#loop).unwrap().face, membrane.face);

    // Drop the far verticals inside the membrane. Anchor for rim
    // vertex i: rim[i].he_plus starts there (it moved to the membrane
    // as he1's side of the mef); the LAST rim vertex is the start of
    // the membrane's own he_minus.
    let mut drops: Vec<MevCreated> = Vec::new();
    for (i, p) in drop_pts.iter().enumerate() {
        let anchor = if i < rim.len() {
            rim[i].he_plus
        } else {
            membrane.he_minus
        };
        let d = body
            .mev_line(
                MevSite::Fan {
                    he1: anchor,
                    he2: anchor,
                },
                *p,
                tol,
            )
            .unwrap();
        l.v += 1;
        l.e += 1;
        check(body, *l, h);
        drops.push(d);
    }

    // Cut the tube walls out of the membrane.
    let mut walls: Vec<MefCreated> = Vec::new();
    for i in 0..drops.len() - 1 {
        let w = body
            .mef_chord(
                MefSite::Chords {
                    he1: drops[i].he_minus,
                    he2: drops[i + 1].he_minus,
                },
                tol,
            )
            .unwrap();
        l.e += 1;
        l.f += 1;
        check(body, *l, h);
        walls.push(w);
    }
    let he_first_far = body
        .find_half_edge(membrane.face, drops[0].vertex, drops[1].vertex)
        .expect("far edge of first wall still in the membrane loop");
    assert_eq!(he_first_far, walls[0].he_plus);
    let w_last = body
        .mef_chord(
            MefSite::Chords {
                he1: drops.last().unwrap().he_minus,
                he2: he_first_far,
            },
            tol,
        )
        .unwrap();
    l.e += 1;
    l.f += 1;
    check(body, *l, h);
    walls.push(w_last);

    // The connected sum. Assert the half-edge/edge/vertex arenas are
    // untouched by kfmrh (byte-identical Debug lines).
    let hes_before: Vec<String> = body
        .half_edges()
        .map(|(k, e)| format!("{k:?}:{e:?}"))
        .collect();
    let edges_before: Vec<String> = body.edges().map(|(k, e)| format!("{k:?}:{e:?}")).collect();
    let vertices_before: Vec<String> = body
        .vertices()
        .map(|(k, e)| format!("{k:?}:{e:?}"))
        .collect();
    let plug = body.kfmrh(f_to, membrane.face).unwrap();
    l.f -= 1;
    l.r += 1;
    check(body, *l, h + 1);
    assert_eq!(
        body.half_edges()
            .map(|(k, e)| format!("{k:?}:{e:?}"))
            .collect::<Vec<_>>(),
        hes_before
    );
    assert_eq!(
        body.edges()
            .map(|(k, e)| format!("{k:?}:{e:?}"))
            .collect::<Vec<_>>(),
        edges_before
    );
    assert_eq!(
        body.vertices()
            .map(|(k, e)| format!("{k:?}:{e:?}"))
            .collect::<Vec<_>>(),
        vertices_before
    );
    assert_eq!(plug.ring, membrane.r#loop);
    assert_eq!(plug.killed_surface, None, "membrane shared mvfs's surface");
    assert!(body.get_face(membrane.face).is_none());
    assert_eq!(body.provenance(EntityId::Face(membrane.face)), None);
    assert!(body.get_face(f_to).unwrap().rings.contains(&plug.ring));
    // The demoted loop keeps its mef birth record (D5 survivors).
    assert!(matches!(
        body.provenance(EntityId::Loop(plug.ring)),
        Some(Provenance::Mef { .. })
    ));

    HoleBuilt {
        strut,
        kill,
        rim,
        membrane,
        drops,
        walls,
        plug,
    }
}

#[test]
fn independent_genus_one_and_two_builds_with_hand_ledger() {
    let tol = Tol::witness();
    let pt = Point3::new;
    let mut body = Body::<f64>::new();
    let b = build_box(&mut body, tol);
    let mut l = GenusInputs {
        v: 8,
        e: 12,
        f: 6,
        r: 0,
        s: 1,
    };

    // ---- Hole 1: TRIANGULAR, through the FRONT (y=0) to the BACK
    // (y=2). H1(0.5,0,0.5) H2(1.5,0,0.5) H3(1.0,0,1.5); G_i behind. ----
    let he_front_ab = body
        .find_half_edge(b.f_front.face, b.seed.vertex, b.e_ab.vertex)
        .unwrap();
    let hole1 = carve_hole(
        &mut body,
        he_front_ab,
        b.f_front.face,
        b.f_back.face,
        &[pt(0.5, 0.0, 0.5), pt(1.5, 0.0, 0.5), pt(1.0, 0.0, 1.5)],
        &[pt(0.5, 2.0, 0.5), pt(1.5, 2.0, 0.5), pt(1.0, 2.0, 1.5)],
        &mut l,
        0,
        tol,
    );
    assert_eq!(
        l,
        GenusInputs {
            v: 14,
            e: 21,
            f: 9,
            r: 2,
            s: 1
        }
    );
    assert_eq!(genus(l), 1);

    // Orientation, derived by hand (see review notes): the rim ring on
    // the front face walks H1→H3→H2 — CW viewed from OUTSIDE (−y),
    // per interior-left; the final membrane walks G3→G1→G2 — CW viewed
    // from outside (+y) once it is the back face's ring.
    let hole_ring_first = match body.get_loop(hole1.kill.ring).unwrap().boundary {
        LoopBoundary::Cycle { first } => first,
        LoopBoundary::Empty { .. } => panic!("rim ring must be a cycle"),
    };
    assert_eq!(
        starts(&body, hole_ring_first),
        vec![
            hole1.strut.vertex,  // H1
            hole1.rim[1].vertex, // H3
            hole1.rim[0].vertex  // H2
        ],
        "front rim ring walks H1 → H3 → H2 (CW from outside)"
    );
    assert_eq!(
        starts(&body, hole1.walls[2].he_plus),
        vec![
            hole1.drops[2].vertex,
            hole1.drops[0].vertex,
            hole1.drops[1].vertex
        ],
        "back ring walks G3 → G1 → G2 (CW from outside)"
    );

    // ---- Hole 2: SQUARE, through the TOP to the BOTTOM (a second,
    // independent handle → genus 2). ----
    let he_top = body
        .find_half_edge(b.seed.face, b.e_aa.vertex, b.e_bb.vertex)
        .expect("top face holds A'→B'");
    let hole2 = carve_hole(
        &mut body,
        he_top,
        b.seed.face,
        b.f_bottom.face,
        &[
            pt(0.5, 0.5, 2.0),
            pt(1.5, 0.5, 2.0),
            pt(1.5, 1.5, 2.0),
            pt(0.5, 1.5, 2.0),
        ],
        &[
            pt(0.5, 0.5, 0.0),
            pt(1.5, 0.5, 0.0),
            pt(1.5, 1.5, 0.0),
            pt(0.5, 1.5, 0.0),
        ],
        &mut l,
        1,
        tol,
    );
    // Hand ledger, genus 2: v−e+f−r = 22−33+13−4 = −2 = 2(1−2).
    assert_eq!(
        l,
        GenusInputs {
            v: 22,
            e: 33,
            f: 13,
            r: 4,
            s: 1
        }
    );
    assert_eq!(l.v - l.e + l.f - l.r, -2);
    assert_eq!(genus(l), 2);
    assert_eq!(body.loops().count(), 17); // 13 outer + 4 rings
    assert_eq!(body.half_edges().count(), 66);
    assert_eq!(body.surfaces().count(), 1);
    assert_eq!(body.curves().count(), 33);
    // No vertex is ever killed (kemr strands the strut tip as the ring
    // anchor), so points == vertices.
    assert_eq!(body.points().count(), 22);

    // Ring memberships: exactly four faces carry exactly one ring each.
    assert_eq!(
        body.get_face(b.f_front.face).unwrap().rings,
        vec![hole1.kill.ring]
    );
    assert_eq!(
        body.get_face(b.f_back.face).unwrap().rings,
        vec![hole1.plug.ring]
    );
    assert_eq!(
        body.get_face(b.seed.face).unwrap().rings,
        vec![hole2.kill.ring]
    );
    assert_eq!(
        body.get_face(b.f_bottom.face).unwrap().rings,
        vec![hole2.plug.ring]
    );
    for (key, face) in body.faces() {
        if [b.f_front.face, b.f_back.face, b.seed.face, b.f_bottom.face].contains(&key) {
            assert_eq!(face.rings.len(), 1);
        } else {
            assert!(face.rings.is_empty(), "unexpected ring on {key:?}");
        }
    }

    // ring_move exercise on the finished genus-2 body: reparent the
    // bottom ring to a side face and back; provenance untouched, Euler
    // vector untouched, still valid. (Geometrically silly, structurally
    // legal — ring_move is pure reparenting.)
    let birth = body.provenance(EntityId::Loop(hole2.plug.ring)).cloned();
    body.ring_move(hole2.plug.ring, b.f_right.face).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(genus_inputs(&body), l);
    assert_eq!(
        body.get_face(b.f_right.face).unwrap().rings,
        vec![hole2.plug.ring]
    );
    body.ring_move(hole2.plug.ring, b.f_bottom.face).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(
        body.provenance(EntityId::Loop(hole2.plug.ring)).cloned(),
        birth
    );
    assert_eq!(
        body.get_face(b.f_bottom.face).unwrap().rings,
        vec![hole2.plug.ring]
    );

    // Provenance sweep: every live topology entity has a birth record;
    // every killed key answers None.
    for (k, _) in body.vertices() {
        assert!(body.provenance(EntityId::Vertex(k)).is_some());
    }
    for (k, _) in body.edges() {
        assert!(body.provenance(EntityId::Edge(k)).is_some());
    }
    for (k, _) in body.half_edges() {
        assert!(body.provenance(EntityId::HalfEdge(k)).is_some());
    }
    for (k, _) in body.loops() {
        assert!(body.provenance(EntityId::Loop(k)).is_some());
    }
    for (k, _) in body.faces() {
        assert!(body.provenance(EntityId::Face(k)).is_some());
    }
    for (k, _) in body.shells() {
        assert!(body.provenance(EntityId::Shell(k)).is_some());
    }
    for (k, _) in body.solids() {
        assert!(body.provenance(EntityId::Solid(k)).is_some());
    }
    for hole in [&hole1, &hole2] {
        assert_eq!(body.provenance(EntityId::Edge(hole.kill.killed_edge)), None);
        assert_eq!(
            body.provenance(EntityId::HalfEdge(hole.kill.killed_he_plus)),
            None
        );
        assert_eq!(
            body.provenance(EntityId::HalfEdge(hole.kill.killed_he_minus)),
            None
        );
        assert_eq!(body.provenance(EntityId::Face(hole.plug.killed_face)), None);
    }

    // Replay determinism of the whole genus-2 history, kills included.
    let mut body2 = Body::<f64>::new();
    let b2 = build_box(&mut body2, tol);
    let mut l2 = GenusInputs {
        v: 8,
        e: 12,
        f: 6,
        r: 0,
        s: 1,
    };
    let he_front_ab2 = body2
        .find_half_edge(b2.f_front.face, b2.seed.vertex, b2.e_ab.vertex)
        .unwrap();
    let h1b = carve_hole(
        &mut body2,
        he_front_ab2,
        b2.f_front.face,
        b2.f_back.face,
        &[pt(0.5, 0.0, 0.5), pt(1.5, 0.0, 0.5), pt(1.0, 0.0, 1.5)],
        &[pt(0.5, 2.0, 0.5), pt(1.5, 2.0, 0.5), pt(1.0, 2.0, 1.5)],
        &mut l2,
        0,
        tol,
    );
    let he_top2 = body2
        .find_half_edge(b2.seed.face, b2.e_aa.vertex, b2.e_bb.vertex)
        .unwrap();
    let h2b = carve_hole(
        &mut body2,
        he_top2,
        b2.seed.face,
        b2.f_bottom.face,
        &[
            pt(0.5, 0.5, 2.0),
            pt(1.5, 0.5, 2.0),
            pt(1.5, 1.5, 2.0),
            pt(0.5, 1.5, 2.0),
        ],
        &[
            pt(0.5, 0.5, 0.0),
            pt(1.5, 0.5, 0.0),
            pt(1.5, 1.5, 0.0),
            pt(0.5, 1.5, 0.0),
        ],
        &mut l2,
        1,
        tol,
    );
    body2.ring_move(h2b.plug.ring, b2.f_right.face).unwrap();
    body2.ring_move(h2b.plug.ring, b2.f_bottom.face).unwrap();
    assert_eq!((hole1.kill, hole1.plug), (h1b.kill, h1b.plug));
    assert_eq!((hole2.kill, hole2.plug), (h2b.kill, h2b.plug));
    assert_eq!(hole1.rim, h1b.rim);
    assert_eq!(hole2.walls, h2b.walls);
    assert_eq!(snapshot(&body), snapshot(&body2));
}

// ---------------------------------------------------------------------
// kemr semantics under independent derivation
// ---------------------------------------------------------------------

/// mvfs + mev(Lone) + (n−1) strut mevs: the n-edge open chain
/// v0–v1–…–vn in one loop, cycle [e0+ … e(n−1)+ e(n−1)− … e0−].
fn chain(n: usize, tol: Tol) -> (Body<f64>, MvfsCreated, Vec<MevCreated>) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0)).unwrap();
    let mut es = Vec::new();
    let first = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    es.push(first);
    for i in 1..n {
        let prev = es.last().unwrap().he_minus;
        let e = body
            .mev_line(
                MevSite::Fan {
                    he1: prev,
                    he2: prev,
                },
                Point3::new(1.0 + i as f64, 0.0, 0.0),
                tol,
            )
            .unwrap();
        es.push(e);
    }
    assert_eq!(validate(&body), Ok(()));
    (body, seed, es)
}

#[test]
fn kemr_asymmetric_split_exact_membership_and_anchors() {
    let tol = Tol::witness();
    // 5-edge chain; kill the MIDDLE edge e2. Hand derivation:
    // cycle from e2+ is [e2+, e3+, e4+, e4−, e3−, e2−, e1−, e0−, e0+, e1+];
    // ring side (strictly between e2+ and e2−) = [e3+, e4+, e4−, e3−];
    // old side = [e1−, e0−, e0+, e1+].
    let (mut body, seed, es) = chain(5, tol);
    let result = body.kemr(es[2].he_plus, es[2].he_minus).unwrap();
    assert_eq!(validate(&body), Ok(()));

    // Ring anchored at next(he1) = e3+, exact cycle order.
    assert_eq!(
        body.get_loop(result.ring).unwrap().boundary,
        LoopBoundary::Cycle {
            first: es[3].he_plus
        }
    );
    assert_eq!(
        body.loop_cycle(es[3].he_plus).unwrap(),
        vec![es[3].he_plus, es[4].he_plus, es[4].he_minus, es[3].he_minus]
    );
    // Old loop anchored at next(he2) = e1−, exact cycle order.
    assert_eq!(
        body.get_loop(seed.r#loop).unwrap().boundary,
        LoopBoundary::Cycle {
            first: es[1].he_minus
        }
    );
    assert_eq!(
        body.loop_cycle(es[1].he_minus).unwrap(),
        vec![es[1].he_minus, es[0].he_minus, es[0].he_plus, es[1].he_plus]
    );
    // Memberships moved exactly for the ring side.
    for he in [es[3].he_plus, es[4].he_plus, es[4].he_minus, es[3].he_minus] {
        assert_eq!(body.get_half_edge(he).unwrap().parent_loop, result.ring);
    }
    for he in [es[1].he_minus, es[0].he_minus, es[0].he_plus, es[1].he_plus] {
        assert_eq!(body.get_half_edge(he).unwrap().parent_loop, seed.r#loop);
    }
    // Emanating: u = v2 (= es[1].vertex) → next(he2) = e1−;
    // w = v3 (= es[2].vertex) → next(he1) = e3+.
    assert_eq!(
        body.get_vertex(es[1].vertex).unwrap().emanating,
        Some(es[1].he_minus)
    );
    assert_eq!(
        body.get_vertex(es[2].vertex).unwrap().emanating,
        Some(es[3].he_plus)
    );

    // Roundtrip via the documented inverse anchors restores the exact
    // original cycle order (structurally; fresh keys expected).
    let restore = body
        .mekr_chord(
            MekrSite::Cycles {
                target: es[1].he_minus,
                ring: es[3].he_plus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    let vs: Vec<VertexKey> = starts(&body, restore.he_plus);
    // he_plus plays e2+ (v2→v3): [v2 v3 v4 v5 v4 v3 v2 v1 v0 v1]
    assert_eq!(
        vs,
        vec![
            es[1].vertex,
            es[2].vertex,
            es[3].vertex,
            es[4].vertex,
            es[3].vertex,
            es[2].vertex,
            es[1].vertex,
            es[0].vertex,
            seed.vertex,
            es[0].vertex,
        ]
    );
    assert_eq!(body.loops().count(), 1);
    assert!(body.get_face(seed.face).unwrap().rings.is_empty());
}

#[test]
fn kemr_adjacent_halves_empty_side_anchor_rules() {
    let tol = Tol::witness();
    // Strut kill, unswapped: ring side empty (next(he1) == he2) → ring
    // Empty at w = start(he2) = the tip.
    let (mut body, seed, es) = chain(2, tol);
    let strut = &es[1];
    let result = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(
        body.get_loop(result.ring).unwrap().boundary,
        LoopBoundary::Empty {
            vertex: strut.vertex
        }
    );
    assert_eq!(body.get_vertex(strut.vertex).unwrap().emanating, None);
    // Old loop keeps the e0 segment, first := next(he2) = e0−.
    assert_eq!(
        body.get_loop(seed.r#loop).unwrap().boundary,
        LoopBoundary::Cycle {
            first: es[0].he_minus
        }
    );

    // Swapped: old side empty (next(he2) == he1) → OLD loop Empty at
    // u = start(he1) = the tip; ring gets the whole surviving cycle,
    // anchored at next(he1) = e0−.
    let (mut body2, seed2, es2) = chain(2, tol);
    let strut2 = &es2[1];
    let result2 = body2.kemr(strut2.he_minus, strut2.he_plus).unwrap();
    assert_eq!(validate(&body2), Ok(()));
    assert_eq!(
        body2.get_loop(seed2.r#loop).unwrap().boundary,
        LoopBoundary::Empty {
            vertex: strut2.vertex
        }
    );
    assert_eq!(
        body2.get_loop(result2.ring).unwrap().boundary,
        LoopBoundary::Cycle {
            first: es2[0].he_minus
        }
    );
    assert_eq!(
        body2.loop_cycle(es2[0].he_minus).unwrap(),
        vec![es2[0].he_minus, es2[0].he_plus]
    );
}

/// Raw-built (public builder) self-loop segment: one vertex, one edge,
/// both halves a 2-cycle loop — the u == w collision state.
fn raw_self_loop(tol: Tol) -> (Body<f64>, HalfEdgeKey, HalfEdgeKey, VertexKey) {
    let mut body = Body::<f64>::new();
    let null_he = HalfEdgeKey::default();
    let prov = Provenance::Primordial { op: "review" };
    let pt = body.add_point(Point3::new(0.0, 0.0, 0.0));
    let v = body.add_vertex(
        Vertex {
            point: pt,
            emanating: None,
        },
        prov.clone(),
    );
    let curve = body.add_curve(crate::fixtures::test_curve(Point3::new(0.0, 0.0, 0.0), tol));
    let edge = body.add_edge(
        Edge {
            he_plus: null_he,
            he_minus: null_he,
            curve,
        },
        prov.clone(),
    );
    let h1 = body.add_half_edge(
        HalfEdge {
            edge,
            start: v,
            parent_loop: LoopKey::default(),
            next: null_he,
            prev: null_he,
        },
        prov.clone(),
    );
    let h2 = body.add_half_edge(
        HalfEdge {
            edge,
            start: v,
            parent_loop: LoopKey::default(),
            next: null_he,
            prev: null_he,
        },
        prov.clone(),
    );
    let solid = body.add_solid(Solid { shells: vec![] }, prov.clone());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov.clone(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);
    let surface = body.add_surface(crate::fixtures::test_surface(Point3::new(0.0, 0.0, 0.0)));
    let lp = body.add_loop(
        Loop {
            boundary: LoopBoundary::Cycle { first: h1 },
            face: FaceKey::default(),
        },
        prov.clone(),
    );
    let face = body.add_face(
        Face {
            sense: true,
            surface,
            outer: lp,
            rings: vec![],
            shell,
        },
        prov.clone(),
    );
    body.get_loop_mut(lp).unwrap().face = face;
    body.get_shell_mut(shell).unwrap().faces.push(face);
    body.get_edge_mut(edge).unwrap().he_plus = h1;
    body.get_edge_mut(edge).unwrap().he_minus = h2;
    for (a, b) in [(h1, h2), (h2, h1)] {
        let he = body.get_half_edge_mut(a).unwrap();
        he.next = b;
        he.prev = b;
        he.parent_loop = lp;
    }
    body.get_vertex_mut(v).unwrap().emanating = Some(h1);
    (body, h1, h2, v)
}

#[test]
fn kemr_anchor_collision_input_is_tier1_invalid_and_typed() {
    let tol = Tol::witness();
    let (mut body, h1, h2, v) = raw_self_loop(tol);
    // The PR's claim: this state is ALREADY tier-1-invalid (the vertex
    // orbit splits over the two halves). Verify independently.
    assert!(
        validate(&body).is_err(),
        "self-loop segment must be tier-1-invalid"
    );
    let expected = EulerOpError::EmptyAnchorsCollide { vertex: v };
    assert_err_unchanged(&mut body, &expected, |b| b.kemr(h1, h2).unwrap_err());
    // Swapped arguments: same collision.
    assert_err_unchanged(&mut body, &expected, |b| b.kemr(h2, h1).unwrap_err());
}

// ---------------------------------------------------------------------
// Kill hygiene: stale keys never resurrect across kill/re-mint cycles
// ---------------------------------------------------------------------

#[test]
fn killed_keys_stay_dead_across_slot_recycling() {
    let tol = Tol::witness();
    let (mut body, seed, es) = chain(1, tol);
    let seg = es[0];
    let kill = body.kemr(seg.he_plus, seg.he_minus).unwrap();
    let restore = body
        .mekr_chord(
            MekrSite::BothEmpty {
                target: seed.r#loop,
                ring: kill.ring,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));

    // The re-mint reuses the freed slots (deterministic replay), so the
    // NEW keys must differ from the old in generation — and the OLD
    // keys must resolve to nothing anywhere: arenas and provenance.
    assert_ne!(restore.edge, seg.edge);
    assert_ne!(restore.he_plus, seg.he_plus);
    assert_ne!(restore.he_minus, seg.he_minus);
    assert_ne!(restore.curve, seg.curve);
    assert!(body.get_edge(seg.edge).is_none());
    assert!(body.get_half_edge(seg.he_plus).is_none());
    assert!(body.get_half_edge(seg.he_minus).is_none());
    assert!(body.get_curve_geom(seg.curve).is_none());
    assert!(body.get_loop(kill.ring).is_none());
    assert_eq!(body.provenance(EntityId::Edge(seg.edge)), None);
    assert_eq!(body.provenance(EntityId::HalfEdge(seg.he_plus)), None);
    assert_eq!(body.provenance(EntityId::HalfEdge(seg.he_minus)), None);
    assert_eq!(body.provenance(EntityId::Loop(kill.ring)), None);
    // And the fresh keys carry fresh provenance.
    assert!(matches!(
        body.provenance(EntityId::Edge(restore.edge)),
        Some(Provenance::Mekr { .. })
    ));

    // Second kill/re-mint cycle on the SAME slots: still coherent.
    let kill2 = body.kemr(restore.he_plus, restore.he_minus).unwrap();
    let restore2 = body
        .mekr_chord(
            MekrSite::BothEmpty {
                target: seed.r#loop,
                ring: kill2.ring,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    for dead_he in [seg.he_plus, seg.he_minus, restore.he_plus, restore.he_minus] {
        assert!(body.get_half_edge(dead_he).is_none());
        assert_eq!(body.provenance(EntityId::HalfEdge(dead_he)), None);
    }
    assert_ne!(restore2.he_plus, restore.he_plus);
    assert_ne!(kill2.ring, kill.ring);
}

// ---------------------------------------------------------------------
// Replay/lineage semantics (D9) — convergence boundary probes
// ---------------------------------------------------------------------

#[test]
fn balanced_pair_convergence_holds_for_mev_but_not_for_loop_minting() {
    let tol = Tol::witness();
    // Build A: segment, kemr∘mekr roundtrip, then a follow-up mev and a
    // follow-up mef. Build C: segment, then the same follow-ups.
    let (mut body_a, seed_a, es_a) = chain(1, tol);
    let seg_a = es_a[0];
    let kill_a = body_a.kemr(seg_a.he_plus, seg_a.he_minus).unwrap();
    let restore_a = body_a
        .mekr_chord(
            MekrSite::BothEmpty {
                target: seed_a.r#loop,
                ring: kill_a.ring,
            },
            tol,
        )
        .unwrap();
    let mev_a = body_a
        .mev_line(
            MevSite::Fan {
                he1: restore_a.he_minus,
                he2: restore_a.he_minus,
            },
            Point3::new(9.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let mef_a = body_a
        .mef_chord(
            MefSite::Chords {
                he1: restore_a.he_plus,
                he2: restore_a.he_minus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body_a), Ok(()));

    let (mut body_c, _seed_c, es_c) = chain(1, tol);
    let seg_c = es_c[0];
    let mev_c = body_c
        .mev_line(
            MevSite::Fan {
                he1: seg_c.he_minus,
                he2: seg_c.he_minus,
            },
            Point3::new(9.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let mef_c = body_c
        .mef_chord(
            MefSite::Chords {
                he1: seg_c.he_plus,
                he2: seg_c.he_minus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body_c), Ok(()));

    // The balanced pair consumed exactly the half-edge/edge/curve slots
    // it freed: the follow-up mev's keys converge.
    assert_eq!(mev_a, mev_c, "post-pair mev keys must converge");

    // BUT the pair is NOT loop-slot-balanced: kemr minted the ring in a
    // fresh loop slot and mekr freed it, so history A carries a free
    // loop slot history C never had. Probe where the next loop mint
    // lands in each.
    println!("mef_a: {mef_a:?}");
    println!("mef_c: {mef_c:?}");
    assert_eq!(mef_a.edge, mef_c.edge);
    assert_eq!(mef_a.he_plus, mef_c.he_plus);
    assert_eq!(mef_a.he_minus, mef_c.he_minus);
    assert_eq!(mef_a.curve, mef_c.curve);
    assert_eq!(mef_a.face, mef_c.face);
    // Observed divergence (reviewer probe): the loop key differs — the
    // recycled slot carries a bumped generation.
    assert_ne!(
        mef_a.r#loop, mef_c.r#loop,
        "loop minting after a kemr∘mekr pair reuses the freed ring slot \
         (bumped generation) — NOT the key the pair-free history mints"
    );

    // AFTER the freed loop slot is consumed, loop minting converges
    // again: a second mef gets fully identical keys in both histories.
    let mef2_a = body_a
        .mef_chord(
            MefSite::Chords {
                he1: mev_a.he_plus,
                he2: mev_a.he_minus,
            },
            tol,
        )
        .unwrap();
    let mef2_c = body_c
        .mef_chord(
            MefSite::Chords {
                he1: mev_c.he_plus,
                he2: mev_c.he_minus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body_a), Ok(()));
    assert_eq!(validate(&body_c), Ok(()));
    assert_eq!(mef2_a, mef2_c, "second loop mint after the pair converges");
}

#[test]
fn unbalanced_kill_divergence_persists() {
    let tol = Tol::witness();
    // A: 3-chain, kemr only (no mekr), then two mevs.
    let (mut body_a, _seed_a, es_a) = chain(3, tol);
    let _kill = body_a.kemr(es_a[1].he_plus, es_a[1].he_minus).unwrap();
    let site_a = MevSite::Fan {
        he1: es_a[0].he_minus,
        he2: es_a[0].he_minus,
    };
    let mev1_a = body_a
        .mev_line(site_a, Point3::new(9.0, 0.0, 0.0), tol)
        .unwrap();
    let mev2_a = body_a
        .mev_line(site_a, Point3::new(10.0, 0.0, 0.0), tol)
        .unwrap();

    // C: 3-chain, the same two mevs, no kill.
    let (mut body_c, _seed_c, es_c) = chain(3, tol);
    let site_c = MevSite::Fan {
        he1: es_c[0].he_minus,
        he2: es_c[0].he_minus,
    };
    let mev1_c = body_c
        .mev_line(site_c, Point3::new(9.0, 0.0, 0.0), tol)
        .unwrap();
    let mev2_c = body_c
        .mev_line(site_c, Point3::new(10.0, 0.0, 0.0), tol)
        .unwrap();

    // Killed-arena keys diverge at the first mint (recycled slots)…
    assert_ne!(mev1_a.edge, mev1_c.edge);
    assert_ne!(mev1_a.he_plus, mev1_c.he_plus);
    assert_ne!(mev1_a.he_minus, mev1_c.he_minus);
    assert_ne!(mev1_a.curve, mev1_c.curve);
    // …while untouched arenas stay aligned…
    assert_eq!(mev1_a.vertex, mev1_c.vertex);
    assert_eq!(mev1_a.point, mev1_c.point);
    // …and the divergence PERSISTS at the second mint (the freed slots
    // are drained but the allocation cursor is permanently offset).
    println!("mev2_a: {mev2_a:?}");
    println!("mev2_c: {mev2_c:?}");
    assert_ne!(mev2_a.edge, mev2_c.edge);
    assert_ne!(mev2_a.he_plus, mev2_c.he_plus);
    assert_eq!(mev2_a.vertex, mev2_c.vertex);
}

// ---------------------------------------------------------------------
// mekr beyond kemr's image: joining independently-built rings
// ---------------------------------------------------------------------

#[test]
fn mekr_joins_two_independent_rings_and_then_the_outer() {
    let tol = Tol::witness();
    // One face, TWO empty rings from two independent strut/kemr
    // episodes — a state no single kemr produces.
    let (mut body, seed, es) = chain(1, tol);
    let seg = es[0];
    let strut_a = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_minus,
                he2: seg.he_minus,
            },
            Point3::new(2.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let ring_a = body.kemr(strut_a.he_plus, strut_a.he_minus).unwrap();
    let strut_b = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: seg.he_plus,
            },
            Point3::new(3.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let ring_b = body.kemr(strut_b.he_plus, strut_b.he_minus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(
        body.get_face(seed.face).unwrap().rings,
        vec![ring_a.ring, ring_b.ring]
    );
    let l = genus_inputs(&body);
    assert_eq!(
        l,
        GenusInputs {
            v: 4,
            e: 1,
            f: 1,
            r: 2,
            s: 1
        }
    );
    assert_eq!(genus(l), 0);

    // Join ring B into ring A: BothEmpty with a RING as the target —
    // the merged segment loop stays a ring of the face.
    let join = body
        .mekr_chord(
            MekrSite::BothEmpty {
                target: ring_a.ring,
                ring: ring_b.ring,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(body.get_face(seed.face).unwrap().rings, vec![ring_a.ring]);
    assert!(body.get_loop(ring_b.ring).is_none());
    assert_eq!(
        body.get_loop(ring_a.ring).unwrap().boundary,
        LoopBoundary::Cycle {
            first: join.he_plus
        }
    );
    // he_plus runs target anchor (strut_a tip) → ring anchor (strut_b
    // tip).
    assert_eq!(
        body.get_half_edge(join.he_plus).unwrap().start,
        strut_a.vertex
    );
    assert_eq!(body.half_edge_end(join.he_plus), Some(strut_b.vertex));
    assert_eq!(
        genus_inputs(&body),
        GenusInputs {
            v: 4,
            e: 2,
            f: 1,
            r: 1,
            s: 1
        }
    );

    // Now join the (cycle) ring into the OUTER loop with Cycles: the
    // documented merged-cycle order he_plus → ring-from-anchor →
    // he_minus → target-from-anchor.
    let merge = body
        .mekr_chord(
            MekrSite::Cycles {
                target: seg.he_plus,
                ring: join.he_plus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert!(body.get_face(seed.face).unwrap().rings.is_empty());
    assert_eq!(
        starts(&body, merge.he_plus),
        vec![
            seed.vertex,    // he_plus: v0 → tipA
            strut_a.vertex, // join.he_plus: tipA → tipB
            strut_b.vertex, // join.he_minus: tipB → tipA
            strut_a.vertex, // merge.he_minus: tipA → v0
            seed.vertex,    // seg.he_plus: v0 → v1
            seg.vertex,     // seg.he_minus: v1 → v0
        ]
    );
    let l = genus_inputs(&body);
    assert_eq!(
        l,
        GenusInputs {
            v: 4,
            e: 3,
            f: 1,
            r: 0,
            s: 1
        }
    );
    assert_eq!(genus(l), 0);
}

// ---------------------------------------------------------------------
// Geometry hygiene: shared vs private curve/surface reaping
// ---------------------------------------------------------------------

/// Raw-built 2-edge chain (v0–v1–v2, one loop) whose two edges SHARE
/// one curve — unreachable through M1 operators (each op mints its own
/// curve) but tier-1-valid, exercising the reap scan's keep branch.
fn raw_shared_curve_chain(
    tol: Tol,
) -> (
    Body<f64>,
    [HalfEdgeKey; 4], // a0 (v0→v1), a1 (v1→v2), b1 (v2→v1), b0 (v1→v0)
    crate::CurveKey,
) {
    let mut body = Body::<f64>::new();
    let null_he = HalfEdgeKey::default();
    let prov = Provenance::Primordial { op: "review" };
    let p = |i: f64| Point3::new(i, 0.0, 0.0);
    let pts = [
        body.add_point(p(0.0)),
        body.add_point(p(1.0)),
        body.add_point(p(2.0)),
    ];
    let vs: Vec<VertexKey> = pts
        .iter()
        .map(|&point| {
            body.add_vertex(
                Vertex {
                    point,
                    emanating: None,
                },
                prov.clone(),
            )
        })
        .collect();
    let curve = body.add_curve(crate::fixtures::test_curve(p(0.0), tol));
    let e0 = body.add_edge(
        Edge {
            he_plus: null_he,
            he_minus: null_he,
            curve,
        },
        prov.clone(),
    );
    let e1 = body.add_edge(
        Edge {
            he_plus: null_he,
            he_minus: null_he,
            curve,
        },
        prov.clone(),
    );
    let half = |body: &mut Body<f64>, edge, start| {
        body.add_half_edge(
            HalfEdge {
                edge,
                start,
                parent_loop: LoopKey::default(),
                next: null_he,
                prev: null_he,
            },
            prov.clone(),
        )
    };
    let a0 = half(&mut body, e0, vs[0]);
    let a1 = half(&mut body, e1, vs[1]);
    let b1 = half(&mut body, e1, vs[2]);
    let b0 = half(&mut body, e0, vs[1]);
    let solid = body.add_solid(Solid { shells: vec![] }, prov.clone());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov.clone(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);
    let surface = body.add_surface(crate::fixtures::test_surface(p(0.0)));
    let lp = body.add_loop(
        Loop {
            boundary: LoopBoundary::Cycle { first: a0 },
            face: FaceKey::default(),
        },
        prov.clone(),
    );
    let face = body.add_face(
        Face {
            sense: true,
            surface,
            outer: lp,
            rings: vec![],
            shell,
        },
        prov.clone(),
    );
    body.get_loop_mut(lp).unwrap().face = face;
    body.get_shell_mut(shell).unwrap().faces.push(face);
    body.get_edge_mut(e0).unwrap().he_plus = a0;
    body.get_edge_mut(e0).unwrap().he_minus = b0;
    body.get_edge_mut(e1).unwrap().he_plus = a1;
    body.get_edge_mut(e1).unwrap().he_minus = b1;
    let cycle = [a0, a1, b1, b0];
    for i in 0..4 {
        let he = body.get_half_edge_mut(cycle[i]).unwrap();
        he.next = cycle[(i + 1) % 4];
        he.prev = cycle[(i + 3) % 4];
        he.parent_loop = lp;
    }
    body.get_vertex_mut(vs[0]).unwrap().emanating = Some(a0);
    body.get_vertex_mut(vs[1]).unwrap().emanating = Some(a1);
    body.get_vertex_mut(vs[2]).unwrap().emanating = Some(b1);
    (body, [a0, a1, b1, b0], curve)
}

#[test]
fn kemr_keeps_shared_curves_and_reaps_private_ones() {
    let tol = Tol::witness();
    let (mut body, [a0, a1, b1, b0], curve) = raw_shared_curve_chain(tol);
    assert_eq!(
        validate(&body),
        Ok(()),
        "shared-curve chain must be tier-1-valid"
    );

    // Kill e1 (halves a1/b1): the curve is still referenced by e0 —
    // KEPT, and the result reports keeping it.
    let kill1 = body.kemr(a1, b1).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(kill1.killed_curve, None);
    assert!(body.get_curve_geom(curve).is_some());
    assert_eq!(body.curves().count(), 1);

    // Kill e0 too: now the curve is orphaned — REAPED.
    let kill2 = body.kemr(a0, b0).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(kill2.killed_curve, Some(curve));
    assert!(body.get_curve_geom(curve).is_none());
    assert_eq!(body.curves().count(), 0);
}

#[test]
fn kfmrh_reaps_private_surfaces_and_keeps_shared_ones() {
    let tol = Tol::witness();
    // Ops-built pillow; shared case first.
    let (mut body, seed, es) = chain(1, tol);
    let seg = es[0];
    let split = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            tol,
        )
        .unwrap();
    let shared = body.kfmrh(seed.face, split.face).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(shared.killed_surface, None);
    assert_eq!(body.surfaces().count(), 1);

    // Private case: same construction, but give the mef face its OWN
    // surface first (public mutation; body stays tier-1-valid since
    // the shared surface is still referenced by the seed face).
    let (mut body2, seed2, es2) = chain(1, tol);
    let seg2 = es2[0];
    let split2 = body2
        .mef_chord(
            MefSite::Chords {
                he1: seg2.he_plus,
                he2: seg2.he_minus,
            },
            tol,
        )
        .unwrap();
    let private = body2.add_surface(crate::fixtures::test_surface(Point3::new(5.0, 0.0, 0.0)));
    body2.get_face_mut(split2.face).unwrap().surface = private;
    assert_eq!(validate(&body2), Ok(()));
    let reaped = body2.kfmrh(seed2.face, split2.face).unwrap();
    assert_eq!(validate(&body2), Ok(()));
    assert_eq!(reaped.killed_surface, Some(private));
    assert!(body2.get_surface(private).is_none());
    assert_eq!(body2.surfaces().count(), 1);
}

// ---------------------------------------------------------------------
// Atomicity: every new error variant through the public API, with a
// deep body-unchanged assertion (including corrupt-body paths, which
// must behave identically in release builds).
// ---------------------------------------------------------------------

#[test]
fn kemr_error_paths_are_atomic() {
    let tol = Tol::witness();
    // NotSameEdge: same key twice.
    let (mut body, _seed, es) = chain(2, tol);
    assert_err_unchanged(
        &mut body,
        &EulerOpError::NotSameEdge {
            he1: es[0].he_plus,
            he2: es[0].he_plus,
        },
        |b| b.kemr(es[0].he_plus, es[0].he_plus).unwrap_err(),
    );
    // NotSameEdge: halves of different edges.
    assert_err_unchanged(
        &mut body,
        &EulerOpError::NotSameEdge {
            he1: es[0].he_plus,
            he2: es[1].he_minus,
        },
        |b| b.kemr(es[0].he_plus, es[1].he_minus).unwrap_err(),
    );
    // NotSameEdge: corrupt bijection (edge does not claim its half).
    let (mut body, _seed, es) = chain(2, tol);
    body.get_edge_mut(es[1].edge).unwrap().he_plus = es[0].he_plus;
    assert_err_unchanged(
        &mut body,
        &EulerOpError::NotSameEdge {
            he1: es[1].he_plus,
            he2: es[1].he_minus,
        },
        |b| b.kemr(es[1].he_plus, es[1].he_minus).unwrap_err(),
    );
    // NotSameLoop: halves split across a mef.
    let (mut body, _seed, es) = chain(1, tol);
    let seg = es[0];
    body.mef_chord(
        MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        },
        tol,
    )
    .unwrap();
    assert_err_unchanged(
        &mut body,
        &EulerOpError::NotSameLoop {
            he1: seg.he_plus,
            he2: seg.he_minus,
        },
        |b| b.kemr(seg.he_plus, seg.he_minus).unwrap_err(),
    );
    // StaleKey (half-edge argument).
    let (mut body, _seed, es) = chain(2, tol);
    let dead = HalfEdgeKey::default();
    assert_err_unchanged(
        &mut body,
        &EulerOpError::StaleKey {
            key: EntityId::HalfEdge(dead),
        },
        |b| b.kemr(dead, es[0].he_minus).unwrap_err(),
    );
    // LoopNotCycle: both halves' parent repointed at an empty ring loop
    // (corrupt body; typed error, atomic, release-safe).
    let (mut body, _seed, es) = chain(2, tol);
    let strut = es[1];
    let ring = body.kemr(strut.he_plus, strut.he_minus).unwrap().ring;
    body.get_half_edge_mut(es[0].he_plus).unwrap().parent_loop = ring;
    body.get_half_edge_mut(es[0].he_minus).unwrap().parent_loop = ring;
    assert_err_unchanged(
        &mut body,
        &EulerOpError::LoopNotCycle { r#loop: ring },
        |b| b.kemr(es[0].he_plus, es[0].he_minus).unwrap_err(),
    );
    // LoopCycleBroken: self-linked next skips he2 (corrupt).
    let (mut body, seed, es) = chain(2, tol);
    body.get_half_edge_mut(es[1].he_plus).unwrap().next = es[1].he_plus;
    assert_err_unchanged(
        &mut body,
        &EulerOpError::LoopCycleBroken {
            r#loop: seed.r#loop,
        },
        |b| b.kemr(es[1].he_plus, es[1].he_minus).unwrap_err(),
    );
    // LoopCycleBroken via a WANDERING walk: next points into another
    // loop's cycle so the walk never returns — the bounded walk must
    // terminate (no hang, also in release).
    let (mut body, seed, es) = chain(1, tol);
    let seg = es[0];
    let split = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            tol,
        )
        .unwrap();
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_minus,
                he2: seg.he_minus,
            },
            Point3::new(2.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    // strut halves live in the OLD face's loop; point the strut's next
    // into the mef face's loop (which cycles without ever reaching the
    // strut again).
    body.get_half_edge_mut(strut.he_plus).unwrap().next = split.he_minus;
    let _ = seed;
    let t0 = std::time::Instant::now();
    let before = snapshot(&body);
    let err = body.kemr(strut.he_plus, strut.he_minus).unwrap_err();
    assert!(matches!(err, EulerOpError::LoopCycleBroken { .. }));
    assert_eq!(snapshot(&body), before);
    assert!(
        t0.elapsed() < std::time::Duration::from_secs(5),
        "bounded walk must terminate promptly"
    );
    // StaleKey on a start vertex (corrupt half-edge).
    let (mut body, _seed, es) = chain(2, tol);
    body.get_half_edge_mut(es[1].he_plus).unwrap().start = VertexKey::default();
    assert_err_unchanged(
        &mut body,
        &EulerOpError::StaleKey {
            key: EntityId::Vertex(VertexKey::default()),
        },
        |b| b.kemr(es[1].he_plus, es[1].he_minus).unwrap_err(),
    );
}

#[test]
fn mekr_error_paths_are_atomic() {
    let tol = Tol::witness();
    // SameLoop.
    let (mut body, seed, es) = chain(1, tol);
    let seg = es[0];
    assert_err_unchanged(
        &mut body,
        &EulerOpError::SameLoop {
            r#loop: seed.r#loop,
        },
        |b| {
            b.mekr_chord(
                MekrSite::Cycles {
                    target: seg.he_plus,
                    ring: seg.he_minus,
                },
                tol,
            )
            .unwrap_err()
        },
    );
    // NotSameFace: ring on the old face, target half in the mef face.
    let (mut body, _seed, es) = chain(1, tol);
    let seg = es[0];
    let split = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            tol,
        )
        .unwrap();
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_minus,
                he2: seg.he_minus,
            },
            Point3::new(2.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let ring = body.kemr(strut.he_plus, strut.he_minus).unwrap().ring;
    assert_eq!(validate(&body), Ok(()));
    let target_loop = body.get_half_edge(split.he_minus).unwrap().parent_loop;
    assert_err_unchanged(
        &mut body,
        &EulerOpError::NotSameFace {
            target: target_loop,
            ring,
        },
        |b| {
            b.mekr_chord(
                MekrSite::EmptyRing {
                    target: split.he_minus,
                    ring,
                },
                tol,
            )
            .unwrap_err()
        },
    );
    // RingIsOuter: name the outer loop as the ring to kill.
    let (mut body, seed, es) = chain(2, tol);
    let strut = es[1];
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    assert_err_unchanged(
        &mut body,
        &EulerOpError::RingIsOuter {
            r#loop: seed.r#loop,
        },
        |b| {
            b.mekr_chord(
                MekrSite::EmptyTarget {
                    target: b_ring(b, seed.face),
                    ring: es[0].he_plus,
                },
                tol,
            )
            .unwrap_err()
        },
    );
    // LoopNotEmpty: EmptyRing with a cycle ring.
    let (mut body, _seed, es) = chain(3, tol);
    let kill = body.kemr(es[1].he_plus, es[1].he_minus).unwrap();
    assert_err_unchanged(
        &mut body,
        &EulerOpError::LoopNotEmpty { r#loop: kill.ring },
        |b| {
            b.mekr_chord(
                MekrSite::EmptyRing {
                    target: es[0].he_minus,
                    ring: kill.ring,
                },
                tol,
            )
            .unwrap_err()
        },
    );
    // LoopCycleBroken: corrupt the ring's cycle with a null next.
    let (mut body, _seed, es) = chain(3, tol);
    let kill = body.kemr(es[1].he_plus, es[1].he_minus).unwrap();
    body.get_half_edge_mut(es[2].he_plus).unwrap().next = HalfEdgeKey::default();
    let ring_loop = kill.ring;
    assert_err_unchanged(
        &mut body,
        &EulerOpError::LoopCycleBroken { r#loop: ring_loop },
        |b| {
            b.mekr_chord(
                MekrSite::Cycles {
                    target: es[0].he_minus,
                    ring: es[2].he_plus,
                },
                tol,
            )
            .unwrap_err()
        },
    );
    // EmptyAnchorsCollide: two empty loops sharing one vertex (corrupt,
    // raw-grafted).
    let (mut body, seed, es) = chain(2, tol);
    let strut = es[1];
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    let extra = body.add_loop(
        Loop {
            boundary: LoopBoundary::Empty {
                vertex: strut.vertex,
            },
            face: seed.face,
        },
        Provenance::Primordial { op: "review" },
    );
    body.get_face_mut(seed.face).unwrap().rings.push(extra);
    assert!(
        validate(&body).is_err(),
        "shared lone vertex must be tier-1-invalid"
    );
    assert_err_unchanged(
        &mut body,
        &EulerOpError::EmptyAnchorsCollide {
            vertex: strut.vertex,
        },
        |b| {
            b.mekr_chord(
                MekrSite::BothEmpty {
                    target: kill.ring,
                    ring: extra,
                },
                tol,
            )
            .unwrap_err()
        },
    );
    // StaleKey: dead loop key as the ring.
    let (mut body, _seed, es) = chain(2, tol);
    let dead_loop = LoopKey::default();
    assert_err_unchanged(
        &mut body,
        &EulerOpError::StaleKey {
            key: EntityId::Loop(dead_loop),
        },
        |b| {
            b.mekr_chord(
                MekrSite::EmptyRing {
                    target: es[0].he_plus,
                    ring: dead_loop,
                },
                tol,
            )
            .unwrap_err()
        },
    );
    // StaleKey on the splice point: target's prev dangles (corrupt).
    let (mut body, _seed, es) = chain(2, tol);
    let strut = es[1];
    let ring = body.kemr(strut.he_plus, strut.he_minus).unwrap().ring;
    body.get_half_edge_mut(es[0].he_plus).unwrap().prev = HalfEdgeKey::default();
    assert_err_unchanged(
        &mut body,
        &EulerOpError::StaleKey {
            key: EntityId::HalfEdge(HalfEdgeKey::default()),
        },
        |b| {
            b.mekr_chord(
                MekrSite::EmptyRing {
                    target: es[0].he_plus,
                    ring,
                },
                tol,
            )
            .unwrap_err()
        },
    );
}

/// The single ring of `face` (helper for the RingIsOuter case above).
fn b_ring(body: &Body<f64>, face: FaceKey) -> LoopKey {
    body.get_face(face).unwrap().rings[0]
}

#[test]
fn kfmrh_and_ring_move_error_paths_are_atomic() {
    let tol = Tol::witness();
    // Pillow through ops.
    let (mut body, seed, es) = chain(1, tol);
    let seg = es[0];
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

    // SameFace.
    assert_err_unchanged(
        &mut body,
        &EulerOpError::SameFace { face: seed.face },
        |b| b.kfmrh(seed.face, seed.face).unwrap_err(),
    );
    // CrossSolid (two solids in one body): since M3 PR 1 kfmrh accepts
    // cross-shell faces of ONE solid (shell fusion); across solids it
    // stays a typed error under the new name.
    let other = body.mvfs(Point3::new(9.0, 0.0, 0.0)).unwrap();
    assert_err_unchanged(
        &mut body,
        &EulerOpError::CrossSolid {
            f1: seed.face,
            f2: other.face,
        },
        |b| b.kfmrh(seed.face, other.face).unwrap_err(),
    );
    // StaleKey f1 / f2.
    let dead_face = FaceKey::default();
    assert_err_unchanged(
        &mut body,
        &EulerOpError::StaleKey {
            key: EntityId::Face(dead_face),
        },
        |b| b.kfmrh(dead_face, split.face).unwrap_err(),
    );
    assert_err_unchanged(
        &mut body,
        &EulerOpError::StaleKey {
            key: EntityId::Face(dead_face),
        },
        |b| b.kfmrh(seed.face, dead_face).unwrap_err(),
    );
    // FaceHasRings: put a ring on f2 first (strut + kemr in the mef
    // face's loop), then try to kill it.
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: split.he_minus,
                he2: split.he_minus,
            },
            Point3::new(2.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let ring = body.kemr(strut.he_plus, strut.he_minus).unwrap().ring;
    assert_eq!(validate(&body), Ok(()));
    assert_err_unchanged(
        &mut body,
        &EulerOpError::FaceHasRings { face: split.face },
        |b| b.kfmrh(seed.face, split.face).unwrap_err(),
    );

    // ring_move: RingIsOuter.
    assert_err_unchanged(
        &mut body,
        &EulerOpError::RingIsOuter {
            r#loop: seed.r#loop,
        },
        |b| b.ring_move(seed.r#loop, split.face).unwrap_err(),
    );
    // ring_move: CrossShell (to the second solid's face).
    assert_err_unchanged(
        &mut body,
        &EulerOpError::CrossShell {
            f1: split.face,
            f2: other.face,
        },
        |b| b.ring_move(ring, other.face).unwrap_err(),
    );
    // ring_move: stale ring / stale destination.
    assert_err_unchanged(
        &mut body,
        &EulerOpError::StaleKey {
            key: EntityId::Loop(LoopKey::default()),
        },
        |b| b.ring_move(LoopKey::default(), seed.face).unwrap_err(),
    );
    assert_err_unchanged(
        &mut body,
        &EulerOpError::StaleKey {
            key: EntityId::Face(dead_face),
        },
        |b| b.ring_move(ring, dead_face).unwrap_err(),
    );
    // ring_move to its own face: documented Ok(()) no-op, deeply
    // unchanged.
    let before = snapshot(&body);
    assert_eq!(body.ring_move(ring, split.face), Ok(()));
    assert_eq!(snapshot(&body), before);
}

// ---------------------------------------------------------------------
// Key purity: failing ring-op calls interleaved into a kill-heavy build
// must not perturb any minted key or the final body.
// ---------------------------------------------------------------------

#[test]
fn failing_ring_ops_leave_lineage_pure() {
    let tol = Tol::witness();
    let pt = Point3::new;
    let build = |inject: bool| -> (Body<f64>, Vec<String>) {
        let mut body = Body::<f64>::new();
        let b = build_box(&mut body, tol);
        let mut l = GenusInputs {
            v: 8,
            e: 12,
            f: 6,
            r: 0,
            s: 1,
        };
        if inject {
            // Failing kemr (NotSameEdge), failing kfmrh (SameFace),
            // failing ring_move (RingIsOuter).
            assert!(body.kemr(b.e_ab.he_plus, b.e_ab.he_plus).is_err());
            assert!(body.kfmrh(b.seed.face, b.seed.face).is_err());
            assert!(body.ring_move(b.seed.r#loop, b.f_right.face).is_err());
        }
        let he_front_ab = body
            .find_half_edge(b.f_front.face, b.seed.vertex, b.e_ab.vertex)
            .unwrap();
        let hole = carve_hole(
            &mut body,
            he_front_ab,
            b.f_front.face,
            b.f_back.face,
            &[pt(0.5, 0.0, 0.5), pt(1.5, 0.0, 0.5), pt(1.0, 0.0, 1.5)],
            &[pt(0.5, 2.0, 0.5), pt(1.5, 2.0, 0.5), pt(1.0, 2.0, 1.5)],
            &mut l,
            0,
            tol,
        );
        if inject {
            // Failing mekr (NotSameFace: ring on front face, target in
            // the top face) and failing kfmrh (FaceHasRings).
            let he_top = body
                .find_half_edge(b.seed.face, b.e_aa.vertex, b.e_bb.vertex)
                .unwrap();
            assert!(matches!(
                body.mekr_chord(
                    MekrSite::Cycles {
                        target: he_top,
                        ring: {
                            let LoopBoundary::Cycle { first } =
                                body.get_loop(hole.kill.ring).unwrap().boundary
                            else {
                                panic!()
                            };
                            first
                        }
                    },
                    tol
                ),
                Err(EulerOpError::NotSameFace { .. })
            ));
            assert!(matches!(
                body.kfmrh(b.f_back.face, b.f_front.face),
                Err(EulerOpError::FaceHasRings { .. })
            ));
        }
        // One more op after the injections, to catch cursor drift.
        let post = body
            .mev_line(
                MevSite::Fan {
                    he1: b.e_ab.he_minus,
                    he2: b.e_ab.he_minus,
                },
                pt(3.0, 0.0, 0.0),
                tol,
            )
            .unwrap();
        let mut lines = snapshot(&body);
        lines.push(format!("{post:?}"));
        lines.push(format!("{:?} {:?}", hole.kill, hole.plug));
        (body, lines)
    };
    let (_clean_body, clean) = build(false);
    let (_dirty_body, dirty) = build(true);
    assert_eq!(clean, dirty, "failed ring ops perturbed the lineage");
}
