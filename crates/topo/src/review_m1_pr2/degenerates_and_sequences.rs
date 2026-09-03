//! Adversarial e2e review artifact for M1 PR 2 (2026-07-16). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value. Promoted per
//! Ev's request (PR #17 thread).
//!
//! Degenerate sites (with hand E-P ledgers), ring-split mef, and fresh
//! construction sequences: tetrahedron, triangular prism, two disjoint
//! solids in one Body.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::{euler_poincare_holds, face_polygon};
use crate::{
    Body, Edge, EntityId, Face, HalfEdge, HalfEdgeKey, Loop, LoopBoundary, LoopKey, MefSite,
    MevSite, Provenance, Vertex, validate,
};
use geom_core::Point3;
use geom_core::Tol;

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn p(x: f64) -> Point3<f64> {
    pt(x, 0.0, 0.0)
}

#[test]
fn degenerate_ladder_with_euler_poincare_ledger() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();

    // mvfs: v1 e0 f1 r0 s1 h0 -> 1-0+1-0 = 2(1-0). Ledger OK.
    let seed = body.mvfs(p(0.0)).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert!(euler_poincare_holds(&body, 1, 0));

    // Lone mev from the mvfs state: v2 e1 f1 -> 2-1+1 = 2.
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert!(euler_poincare_holds(&body, 1, 0));

    // Strut (Fan he1==he2): v3 e2 f1 -> 3-2+1 = 2. Valence-1 vertex.
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_minus,
                he2: seg.he_minus,
            },
            p(2.0),
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert!(euler_poincare_holds(&body, 1, 0));
    assert_eq!(
        body.vertex_orbit(strut.he_minus),
        Some(vec![strut.he_minus]),
        "strut tip has valence 1"
    );

    // Self-loop mef (Chords he1==he2) at the strut tip: v3 e3 f2 ->
    // 3-3+2 = 2. One-edge circular face.
    let circ = body
        .mef_chord(
            MefSite::Chords {
                he1: strut.he_minus,
                he2: strut.he_minus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert!(euler_poincare_holds(&body, 1, 0));
    assert_eq!(body.loop_cycle(circ.he_minus), Some(vec![circ.he_minus]));
    // The circular edge starts and ends at the strut tip.
    assert_eq!(
        body.get_half_edge(circ.he_plus).unwrap().start,
        strut.vertex
    );
    assert_eq!(body.half_edge_end(circ.he_plus), Some(strut.vertex));

    // Emanating probe: the strut tip's emanating must still start there.
    let em = body.get_vertex(strut.vertex).unwrap().emanating.unwrap();
    assert_eq!(body.get_half_edge(em).unwrap().start, strut.vertex);
}

#[test]
fn lone_mef_circular_edge_ledger() {
    let tol = Tol::witness();
    // Lone mef straight from the mvfs state: v1 e1 f2 -> 1-1+2 = 2.
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let circ = body
        .mef_chord(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert!(euler_poincare_holds(&body, 1, 0));
    assert_eq!(body.vertices().count(), 1);
    assert_eq!(body.edges().count(), 1);
    assert_eq!(body.faces().count(), 2);
    // Both loops are one-half-edge cycles at the (formerly lone) vertex.
    assert_eq!(body.loop_cycle(circ.he_plus), Some(vec![circ.he_plus]));
    assert_eq!(body.loop_cycle(circ.he_minus), Some(vec![circ.he_minus]));
    // The vertex regained an emanating half-edge that starts at it.
    let em = body.get_vertex(seed.vertex).unwrap().emanating.unwrap();
    assert_eq!(body.get_half_edge(em).unwrap().start, seed.vertex);
    // Its orbit covers both halves (single cycle - watertight).
    assert_eq!(body.vertex_orbit(em).unwrap().len(), 2);
}

/// mev Fan across a circular (self-loop) edge's two halves: the orbit at
/// the lone-ish vertex is [circ+, circ-]; Fan{he1: circ+, he2: circ-}
/// moves circ+ to the new vertex, turning the circular edge into a digon
/// between old and new. Hand-derived; v2 e2 f2 afterwards.
#[test]
fn fan_mev_across_a_circular_edge() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let circ = body
        .mef_chord(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            tol,
        )
        .unwrap();
    assert_eq!(
        body.vertex_orbit(circ.he_plus),
        Some(vec![circ.he_plus, circ.he_minus])
    );
    let f = body
        .mev_line(
            MevSite::Fan {
                he1: circ.he_plus,
                he2: circ.he_minus,
            },
            p(1.0),
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert!(euler_poincare_holds(&body, 1, 0));
    assert_eq!(body.vertices().count(), 2);
    assert_eq!(body.edges().count(), 2);
    assert_eq!(body.faces().count(), 2);
    // The circular edge became a digon edge: circ+ now w->v, circ- v->w.
    assert_eq!(body.get_half_edge(circ.he_plus).unwrap().start, f.vertex);
    assert_eq!(body.half_edge_end(circ.he_plus), Some(seed.vertex));
    assert_eq!(
        body.get_half_edge(circ.he_minus).unwrap().start,
        seed.vertex
    );
    assert_eq!(body.half_edge_end(circ.he_minus), Some(f.vertex));
}

/// mef Chords with he1 != he2 but start(he1) == start(he2): a legal
/// self-loop chord (Mantyla's teardrop). The new edge starts and ends at
/// the same vertex; the loop splits; the vertex orbit stays one cycle.
#[test]
fn self_loop_chord_between_distinct_half_edges() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let g = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
            tol,
        )
        .unwrap();
    let t = body
        .mev_line(
            MevSite::Fan {
                he1: g.he_plus,
                he2: g.he_plus,
            },
            p(2.0),
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    // Loop: [g+ (v->w), g- (w->v), t+ (v->u), t- (u->v)]; g+ and t+
    // both start at the seed vertex v.
    assert_eq!(
        body.loop_cycle(g.he_plus),
        Some(vec![g.he_plus, g.he_minus, t.he_plus, t.he_minus])
    );
    let cut = body
        .mef_chord(
            MefSite::Chords {
                he1: g.he_plus,
                he2: t.he_plus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert!(euler_poincare_holds(&body, 1, 0));
    // New edge is a self-loop at v.
    assert_eq!(body.get_half_edge(cut.he_plus).unwrap().start, seed.vertex);
    assert_eq!(body.half_edge_end(cut.he_plus), Some(seed.vertex));
    // New loop = he1's side [g+, g-] plus he_minus.
    assert_eq!(
        body.loop_cycle(cut.he_minus),
        Some(vec![cut.he_minus, g.he_plus, g.he_minus])
    );
    // Old loop = [plus, t+, t-].
    assert_eq!(
        body.loop_cycle(cut.he_plus),
        Some(vec![cut.he_plus, t.he_plus, t.he_minus])
    );
    // v's orbit is still ONE cycle covering all four halves at v.
    let em = body.get_vertex(seed.vertex).unwrap().emanating.unwrap();
    assert_eq!(body.vertex_orbit(em).unwrap().len(), 4);
}

/// Raw-builds a digon island + ring inside face A of an ops-built digon
/// pillow (rings are unreachable through ops until PR 3's kemr), then
/// mef-splits the RING and checks the ring stays on the old face.
#[test]
fn ring_split_mef_keeps_the_ring_on_the_old_face() {
    let tol = Tol::witness();
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
    // Face A := the OLD face (seed.face, outer loop seed.r#loop).

    // Raw-graft the island (public builder, Primordial provenance).
    let prov = || Provenance::Primordial {
        op: "review:island",
    };
    let null_he = HalfEdgeKey::default();
    let p2 = body.add_point(p(10.0));
    let p3 = body.add_point(p(11.0));
    let v2 = body.add_vertex(
        Vertex {
            point: p2,
            emanating: None,
        },
        prov(),
    );
    let v3 = body.add_vertex(
        Vertex {
            point: p3,
            emanating: None,
        },
        prov(),
    );
    let cu2 = body.add_curve(crate::fixtures::test_curve(p(10.0), tol));
    let cu3 = body.add_curve(crate::fixtures::test_curve(p(11.0), tol));
    let e2 = body.add_edge(
        Edge {
            he_plus: null_he,
            he_minus: null_he,
            curve: cu2,
        },
        prov(),
    );
    let e3 = body.add_edge(
        Edge {
            he_plus: null_he,
            he_minus: null_he,
            curve: cu3,
        },
        prov(),
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
            prov(),
        )
    };
    let c0 = half(&mut body, e2, v2);
    let c1 = half(&mut body, e3, v3);
    let r0 = half(&mut body, e2, v3);
    let r1 = half(&mut body, e3, v2);
    let ring = body.add_loop(
        Loop {
            boundary: LoopBoundary::Cycle { first: r0 },
            face: seed.face,
        },
        prov(),
    );
    let loop_c = body.add_loop(
        Loop {
            boundary: LoopBoundary::Cycle { first: c0 },
            face: crate::FaceKey::default(),
        },
        prov(),
    );
    let surface_c = body.add_surface(crate::fixtures::test_surface(p(10.0)));
    let face_c = body.add_face(
        Face {
            sense: true,
            surface: surface_c,
            outer: loop_c,
            rings: vec![],
            shell: seed.shell,
        },
        prov(),
    );
    body.get_loop_mut(loop_c).unwrap().face = face_c;
    body.get_shell_mut(seed.shell).unwrap().faces.push(face_c);
    body.get_face_mut(seed.face).unwrap().rings.push(ring);
    for (a, b, parent) in [(c0, c1, loop_c), (r0, r1, ring)] {
        for (x, y) in [(a, b), (b, a)] {
            let he = body.get_half_edge_mut(x).unwrap();
            he.next = y;
            he.prev = y;
            he.parent_loop = parent;
        }
    }
    body.get_edge_mut(e2).unwrap().he_plus = c0;
    body.get_edge_mut(e2).unwrap().he_minus = r0;
    body.get_edge_mut(e3).unwrap().he_plus = c1;
    body.get_edge_mut(e3).unwrap().he_minus = r1;
    body.get_vertex_mut(v2).unwrap().emanating = Some(c0);
    body.get_vertex_mut(v3).unwrap().emanating = Some(c1);
    assert_eq!(validate(&body), Ok(()), "island fixture is tier-1 valid");

    // Ledger before: v4 e4 f3 r1 s1 h0 -> 4-4+3-1 = 2. OK.
    assert!(euler_poincare_holds(&body, 1, 0));

    // Split the RING loop: he1 = r0, he2 = r1.
    let cut = body
        .mef_chord(MefSite::Chords { he1: r0, he2: r1 }, tol)
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    // Ledger after: v4 e5 f4 r1 -> 4-5+4-1 = 2. OK.
    assert!(euler_poincare_holds(&body, 1, 0));

    // The old loop (the ring) is STILL a ring of face A...
    assert_eq!(body.get_face(seed.face).unwrap().rings, vec![ring]);
    // ...and he1's side became the NEW face's OUTER loop.
    assert_eq!(body.get_half_edge(r0).unwrap().parent_loop, cut.r#loop);
    assert_eq!(body.get_face(cut.face).unwrap().outer, cut.r#loop);
    // New face shares the SPLIT face's surface (face A's), not face C's.
    assert_eq!(
        body.get_face(cut.face).unwrap().surface,
        body.get_face(seed.face).unwrap().surface
    );

    let _ = split;
}

/// Fresh tetrahedron derivation: 1 mvfs + 3 mev + 3 mef, v4 e6 f4, all
/// faces wind CCW viewed from outside (Newell normal vs centroid ray).
#[test]
fn tetrahedron_by_ops_with_orientation() {
    let tol = Tol::witness();
    let a = pt(0.0, 0.0, 0.0);
    let b = pt(1.0, 0.0, 0.0);
    let c = pt(0.5, 1.0, 0.0);
    let d = pt(0.5, 0.3, 1.0);

    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    let ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            tol,
        )
        .unwrap();
    let bc = body
        .mev_line(
            MevSite::Fan {
                he1: ab.he_minus,
                he2: ab.he_minus,
            },
            c,
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    // Close triangle: he1 = bc.he_minus (C->B), he2 = ab.he_plus (A->B).
    // New face (he1's side + minus): A->C? he_minus = start(he2)->
    // start(he1) = A->C; new loop [A->C, C->B, B->A] -- the BOTTOM
    // (CCW from below).
    let tri = body
        .mef_chord(
            MefSite::Chords {
                he1: bc.he_minus,
                he2: ab.he_plus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));

    // Apex strut from A in the top-side (old) loop: half starting at A
    // there is ab.he_plus.
    let s = body
        .mev_line(
            MevSite::Fan {
                he1: ab.he_plus,
                he2: ab.he_plus,
            },
            d,
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    // Old loop now [tri+ (C->A), s+ (A->D), s- (D->A), ab+ (A->B),
    // bc+ (B->C)].
    // Face (A,B,D): run [s- (D->A), ab+ (A->B)) -> he1 = s.he_minus,
    // he2 = bc.he_plus (B->C). New edge D->B.
    let f_abd = body
        .mef_chord(
            MefSite::Chords {
                he1: s.he_minus,
                he2: bc.he_plus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    // Old loop now [tri+ (C->A), s+ (A->D), f_abd+ (D->B), bc+ (B->C)].
    // Face (C,A,D): run [tri+ (C->A), s+ (A->D)) -> he1 = tri.he_plus,
    // he2 = f_abd.he_plus (D->B). New edge C->D.
    let f_cad = body
        .mef_chord(
            MefSite::Chords {
                he1: tri.he_plus,
                he2: f_abd.he_plus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));

    assert_eq!(body.vertices().count(), 4);
    assert_eq!(body.edges().count(), 6);
    assert_eq!(body.faces().count(), 4);
    assert_eq!(body.half_edges().count(), 12);
    assert!(euler_poincare_holds(&body, 1, 0));
    assert_eq!(body.surfaces().count(), 1, "all faces share mvfs's surface");

    // Orientation: every face's Newell normal points AWAY from the
    // body centroid (convex solid => CCW from outside).
    let centroid = (0.5, 0.325, 0.25);
    for (fk, _) in body.faces() {
        let poly = face_polygon(&body, fk);
        assert_eq!(poly.len(), 3);
        // Newell normal.
        let mut n = (0.0, 0.0, 0.0);
        for i in 0..poly.len() {
            let p1 = poly[i];
            let p2 = poly[(i + 1) % poly.len()];
            n.0 += (p1.1 - p2.1) * (p1.2 + p2.2);
            n.1 += (p1.2 - p2.2) * (p1.0 + p2.0);
            n.2 += (p1.0 - p2.0) * (p1.1 + p2.1);
        }
        let fc = (
            poly.iter().map(|p| p.0).sum::<f64>() / 3.0 - centroid.0,
            poly.iter().map(|p| p.1).sum::<f64>() / 3.0 - centroid.1,
            poly.iter().map(|p| p.2).sum::<f64>() / 3.0 - centroid.2,
        );
        let dot = n.0 * fc.0 + n.1 * fc.1 + n.2 * fc.2;
        assert!(dot > 0.0, "face {fk:?} winds CW from outside (dot={dot})");
    }
    let _ = f_cad;
}

/// Fresh triangular prism: 1 mvfs + 5 mev + 4 mef, v6 e9 f5.
#[test]
fn triangular_prism_by_ops_with_orientation() {
    let tol = Tol::witness();
    // Bottom A B C at z=0, top A' B' C' at z=1.
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    let ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(1.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let bc = body
        .mev_line(
            MevSite::Fan {
                he1: ab.he_minus,
                he2: ab.he_minus,
            },
            pt(0.5, 1.0, 0.0),
            tol,
        )
        .unwrap();
    // Close bottom: he1 = bc.he_minus (C->B), he2 = ab.he_plus (A->B):
    // bottom loop [A->C, C->B, B->A] = CCW from below (outward -z). OK:
    // shoelace of A(0,0) C(.5,1) B(1,0) in (x,y) is negative => CW from
    // above = CCW from below.
    let bot = body
        .mef_chord(
            MefSite::Chords {
                he1: bc.he_minus,
                he2: ab.he_plus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));

    // Struts up at A, B, C in the top-side loop
    // [tri+ (C->A), ab+ (A->B), bc+ (B->C)].
    let sa = body
        .mev_line(
            MevSite::Fan {
                he1: ab.he_plus,
                he2: ab.he_plus,
            },
            pt(0.0, 0.0, 1.0),
            tol,
        )
        .unwrap();
    let sb = body
        .mev_line(
            MevSite::Fan {
                he1: bc.he_plus,
                he2: bc.he_plus,
            },
            pt(1.0, 0.0, 1.0),
            tol,
        )
        .unwrap();
    let sc = body
        .mev_line(
            MevSite::Fan {
                he1: bot.he_plus,
                he2: bot.he_plus,
            },
            pt(0.5, 1.0, 1.0),
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    // Top-side loop now:
    // [tri+ (C->A), sa+ (A->A'), sa- (A'->A), ab+ (A->B),
    //  sb+ (B->B'), sb- (B'->B), bc+ (B->C), sc+ (C->C'), sc- (C'->C)].
    // Side face A,B,B',A'... front face loop A->B->B'->A' needs run
    // [sa- (A'->A), ab+ (A->B), sb+ (B->B')) -- he1 = sa.he_minus,
    // he2 = sb.he_minus (B'->B). New edge A'->B'.
    let f_ab = body
        .mef_chord(
            MefSite::Chords {
                he1: sa.he_minus,
                he2: sb.he_minus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    // Side face B,C,C',B': run [sb- (B'->B), bc+ (B->C), sc+ (C->C')) --
    // he1 = sb.he_minus, he2 = sc.he_minus. New edge B'->C'.
    let f_bc = body
        .mef_chord(
            MefSite::Chords {
                he1: sb.he_minus,
                he2: sc.he_minus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    // Side face C,A,A',C': run [sc- (C'->C), tri+ (C->A), sa+ (A->A'))
    // -- he1 = sc.he_minus, he2 = f_ab.he_plus (A'->B'). New edge C'->A'.
    let f_ca = body
        .mef_chord(
            MefSite::Chords {
                he1: sc.he_minus,
                he2: f_ab.he_plus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));

    assert_eq!(body.vertices().count(), 6);
    assert_eq!(body.edges().count(), 9);
    assert_eq!(body.faces().count(), 5);
    assert!(euler_poincare_holds(&body, 1, 0));
    assert_eq!(body.surfaces().count(), 1);

    // Orientation sweep (convex prism, centroid ray).
    let centroid = (0.5, 1.0 / 3.0, 0.5);
    for (fk, _) in body.faces() {
        let poly = face_polygon(&body, fk);
        let n_pts = poly.len() as f64;
        let mut n = (0.0, 0.0, 0.0);
        for i in 0..poly.len() {
            let p1 = poly[i];
            let p2 = poly[(i + 1) % poly.len()];
            n.0 += (p1.1 - p2.1) * (p1.2 + p2.2);
            n.1 += (p1.2 - p2.2) * (p1.0 + p2.0);
            n.2 += (p1.0 - p2.0) * (p1.1 + p2.1);
        }
        let fc = (
            poly.iter().map(|p| p.0).sum::<f64>() / n_pts - centroid.0,
            poly.iter().map(|p| p.1).sum::<f64>() / n_pts - centroid.1,
            poly.iter().map(|p| p.2).sum::<f64>() / n_pts - centroid.2,
        );
        let dot = n.0 * fc.0 + n.1 * fc.1 + n.2 * fc.2;
        assert!(dot > 0.0, "face {fk:?} winds CW from outside (dot={dot})");
    }
    // The two triangles have 3 half-edges, the three quads 4.
    let mut lens: Vec<usize> = body
        .faces()
        .map(|(fk, _)| face_polygon(&body, fk).len())
        .collect();
    lens.sort_unstable();
    assert_eq!(lens, vec![3, 3, 4, 4, 4]);
    let _ = (f_bc, f_ca);
}

/// Two disjoint solids built in ONE Body via two mvfs chains.
#[test]
fn two_disjoint_solids_in_one_body() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();

    // Solid 1: digon pillow at the origin.
    let s1 = body.mvfs(p(0.0)).unwrap();
    let seg1 = body
        .mev_line(MevSite::Lone { r#loop: s1.r#loop }, p(1.0), tol)
        .unwrap();
    let _ = body
        .mef_chord(
            MefSite::Chords {
                he1: seg1.he_plus,
                he2: seg1.he_minus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));

    // Solid 2: a second skeletal body far away, grown to a pillow too.
    let s2 = body.mvfs(p(100.0)).unwrap();
    assert_eq!(validate(&body), Ok(()));
    let seg2 = body
        .mev_line(MevSite::Lone { r#loop: s2.r#loop }, p(101.0), tol)
        .unwrap();
    let _ = body
        .mef_chord(
            MefSite::Chords {
                he1: seg2.he_plus,
                he2: seg2.he_minus,
            },
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));

    assert_eq!(body.solids().count(), 2);
    assert_eq!(body.shells().count(), 2);
    assert_eq!(body.vertices().count(), 4);
    assert_eq!(body.edges().count(), 4);
    assert_eq!(body.faces().count(), 4);
    // Global E-P with s=2, h=0: 4-4+4-0 = 4 = 2(2-0). OK.
    assert!(euler_poincare_holds(&body, 2, 0));
    assert_ne!(s1.solid, s2.solid);
    assert_ne!(s1.surface, s2.surface, "no surface sharing across solids");

    // Provenance sweep: every topology entity has SOME provenance and it
    // is an operator variant (nothing primordial, nothing missing).
    for (k, _) in body.vertices() {
        assert!(matches!(
            body.provenance(EntityId::Vertex(k)),
            Some(Provenance::Mvfs | Provenance::Mev { .. })
        ));
    }
    for (k, _) in body.faces() {
        assert!(matches!(
            body.provenance(EntityId::Face(k)),
            Some(Provenance::Mvfs | Provenance::Mef { .. })
        ));
    }
    for (k, _) in body.edges() {
        assert!(matches!(
            body.provenance(EntityId::Edge(k)),
            Some(Provenance::Mev { .. } | Provenance::Mef { .. })
        ));
    }
    for (k, _) in body.half_edges() {
        assert!(body.provenance(EntityId::HalfEdge(k)).is_some());
    }
    for (k, _) in body.loops() {
        assert!(body.provenance(EntityId::Loop(k)).is_some());
    }
    for (k, _) in body.shells() {
        assert_eq!(body.provenance(EntityId::Shell(k)), Some(&Provenance::Mvfs));
    }
    for (k, _) in body.solids() {
        assert_eq!(body.provenance(EntityId::Solid(k)), Some(&Provenance::Mvfs));
    }

    // Exact-site provenance spot check: seg2's vertex records the Lone
    // site with solid 2's loop key.
    assert_eq!(
        body.provenance(EntityId::Vertex(seg2.vertex)),
        Some(&Provenance::Mev {
            site: MevSite::Lone { r#loop: s2.r#loop }
        })
    );
}
