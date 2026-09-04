//! Adversarial e2e review artifact for M1 PR 1 (2026-07-16). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value. Promoted per
//! Ev's request (PR #17 thread).
//!
//! Adversarial e2e review consumer program for M1 PR 1. Builds real
//! bodies through the raw builder + patching accessors only and attacks
//! the validator, the bounded walks, and the orientation conventions.
//! (As written, "the PUBLIC raw builder, no crate-internal access" —
//! true at PR 1.)
//!
//! **Moved from `tests/` into `src/` (cfg(test)) at M1 PR 5**, when the
//! raw builder retreated to `pub(crate)` — every probe here raw-builds,
//! which integration tests can no longer do. Adaptations at the move:
//! `use topo::…` became `use crate::…`; the probes are otherwise
//! verbatim.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::{
    Body, Edge, EdgeKey, EntityId, Face, FaceKey, HalfEdge, HalfEdgeKey, Loop, LoopBoundary,
    LoopKey, Provenance, Shell, ShellKey, Solid, SolidKey, Vertex, VertexKey, validate,
};
use geom_core::Point3;
use geom_core::Tol;

fn prov() -> Provenance {
    Provenance::Primordial { op: "e2e-review" }
}

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

// ---------------------------------------------------------------------
// Shared external cube builder. Geometry: bottom square z=0 at
// b0=(0,0,0), b1=(1,0,0), b2=(1,1,0), b3=(0,1,0) (indices CCW viewed
// from above), top square z=1 directly above. Faces oriented per the
// ratified interior-left rule:
//   top:    t0 -> t1 -> t2 -> t3            (CCW from above = outside)
//   bottom: b0 -> b3 -> b2 -> b1            (CCW from below = outside)
//   side i: b[i] -> b[i+1] -> t[i+1] -> t[i] (outward normal radial)
// Edge plus halves: et[i] (t[i]->t[i+1]) plus = top-cap half;
// eb[i] (b[i]->b[i+1]) plus = side-i half; ev[i] (b[i]->t[i]) plus =
// the ascending half (in side i-1).
// ---------------------------------------------------------------------
#[allow(dead_code)] // keys kept for hand-derivation completeness
struct Cube {
    body: Body<f64>,
    t: Vec<VertexKey>,
    b: Vec<VertexKey>,
    // half-edges by (face, position)
    top: Vec<HalfEdgeKey>,  // top[i]: t[i] -> t[i+1]
    bot: Vec<HalfEdgeKey>,  // bot[i]: b[i+1] -> b[i]  (bottom cap)
    s_b: Vec<HalfEdgeKey>,  // side i bottom: b[i] -> b[i+1]
    s_up: Vec<HalfEdgeKey>, // side i up:     b[i+1] -> t[i+1]
    s_t: Vec<HalfEdgeKey>,  // side i top:    t[i+1] -> t[i]
    s_dn: Vec<HalfEdgeKey>, // side i down:   t[i] -> b[i]
    et: Vec<EdgeKey>,
    eb: Vec<EdgeKey>,
    ev: Vec<EdgeKey>,
    loop_top: LoopKey,
    loop_bottom: LoopKey,
    loop_side: Vec<LoopKey>,
    face_top: FaceKey,
    face_bottom: FaceKey,
    face_side: Vec<FaceKey>,
    shell: ShellKey,
    solid: SolidKey,
}

fn build_cube(tol: Tol) -> Cube {
    let n = 4;
    let mut body = Body::<f64>::new();
    let null_he = HalfEdgeKey::default();

    let solid = body.add_solid(Solid { shells: vec![] }, prov());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);

    let corners = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]; // CCW from above
    let bp: Vec<_> = corners
        .iter()
        .map(|&(x, y)| body.add_point(pt(x, y, 0.0)))
        .collect();
    let tp: Vec<_> = corners
        .iter()
        .map(|&(x, y)| body.add_point(pt(x, y, 1.0)))
        .collect();
    let b: Vec<_> = bp
        .iter()
        .map(|&point| {
            body.add_vertex(
                Vertex {
                    point,
                    emanating: None,
                },
                prov(),
            )
        })
        .collect();
    let t: Vec<_> = tp
        .iter()
        .map(|&point| {
            body.add_vertex(
                Vertex {
                    point,
                    emanating: None,
                },
                prov(),
            )
        })
        .collect();

    let mk_edge = |body: &mut Body<f64>| {
        let curve = body.add_curve(crate::fixtures::test_curve(Point3::origin(), tol));
        body.add_edge(
            Edge {
                he_plus: null_he,
                he_minus: null_he,
                curve,
            },
            prov(),
        )
    };
    let et: Vec<_> = (0..n).map(|_| mk_edge(&mut body)).collect();
    let eb: Vec<_> = (0..n).map(|_| mk_edge(&mut body)).collect();
    let ev: Vec<_> = (0..n).map(|_| mk_edge(&mut body)).collect();

    let mk_he = |body: &mut Body<f64>, edge: EdgeKey, start: VertexKey| {
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
    let top: Vec<_> = (0..n).map(|i| mk_he(&mut body, et[i], t[i])).collect();
    let bot: Vec<_> = (0..n)
        .map(|i| mk_he(&mut body, eb[i], b[(i + 1) % n]))
        .collect();
    let s_b: Vec<_> = (0..n).map(|i| mk_he(&mut body, eb[i], b[i])).collect();
    let s_up: Vec<_> = (0..n)
        .map(|i| mk_he(&mut body, ev[(i + 1) % n], b[(i + 1) % n]))
        .collect();
    let s_t: Vec<_> = (0..n)
        .map(|i| mk_he(&mut body, et[i], t[(i + 1) % n]))
        .collect();
    let s_dn: Vec<_> = (0..n).map(|i| mk_he(&mut body, ev[i], t[i])).collect();

    let mk_loop = |body: &mut Body<f64>, first: HalfEdgeKey| {
        body.add_loop(
            Loop {
                boundary: LoopBoundary::Cycle { first },
                face: FaceKey::default(),
            },
            prov(),
        )
    };
    let loop_top = mk_loop(&mut body, top[0]);
    let loop_bottom = mk_loop(&mut body, bot[0]);
    let loop_side: Vec<_> = (0..n).map(|i| mk_loop(&mut body, s_b[i])).collect();

    let mk_face = |body: &mut Body<f64>, outer: LoopKey| {
        let surface = body.add_surface(crate::fixtures::test_surface(Point3::origin()));
        body.add_face(
            Face {
                sense: true,
                surface,
                outer,
                rings: vec![],
                shell,
            },
            prov(),
        )
    };
    let face_top = mk_face(&mut body, loop_top);
    let face_bottom = mk_face(&mut body, loop_bottom);
    let face_side: Vec<_> = (0..n).map(|i| mk_face(&mut body, loop_side[i])).collect();
    body.get_loop_mut(loop_top).unwrap().face = face_top;
    body.get_loop_mut(loop_bottom).unwrap().face = face_bottom;
    for i in 0..n {
        body.get_loop_mut(loop_side[i]).unwrap().face = face_side[i];
    }
    {
        let faces = &mut body.get_shell_mut(shell).unwrap().faces;
        faces.push(face_top);
        faces.push(face_bottom);
        faces.extend(&face_side);
    }

    let link = |body: &mut Body<f64>, cycle: &[HalfEdgeKey], parent: LoopKey| {
        let len = cycle.len();
        for (i, &he) in cycle.iter().enumerate() {
            let h = body.get_half_edge_mut(he).unwrap();
            h.parent_loop = parent;
            h.next = cycle[(i + 1) % len];
            h.prev = cycle[(i + len - 1) % len];
        }
    };
    link(&mut body, &top, loop_top);
    // bottom cap CCW from below: b0 -> b3 -> b2 -> b1, i.e. bot[3], bot[2], bot[1], bot[0]
    let bot_walk: Vec<_> = bot.iter().rev().copied().collect();
    link(&mut body, &bot_walk, loop_bottom);
    for i in 0..n {
        link(&mut body, &[s_b[i], s_up[i], s_t[i], s_dn[i]], loop_side[i]);
    }

    for i in 0..n {
        let e = body.get_edge_mut(et[i]).unwrap();
        e.he_plus = top[i];
        e.he_minus = s_t[i];
        let e = body.get_edge_mut(eb[i]).unwrap();
        e.he_plus = s_b[i];
        e.he_minus = bot[i];
        let e = body.get_edge_mut(ev[i]).unwrap();
        e.he_plus = s_up[(i + n - 1) % n]; // ascending b[i] -> t[i]
        e.he_minus = s_dn[i];
    }
    for i in 0..n {
        body.get_vertex_mut(t[i]).unwrap().emanating = Some(top[i]);
        body.get_vertex_mut(b[i]).unwrap().emanating = Some(s_b[i]);
    }

    Cube {
        body,
        t,
        b,
        top,
        bot,
        s_b,
        s_up,
        s_t,
        s_dn,
        et,
        eb,
        ev,
        loop_top,
        loop_bottom,
        loop_side,
        face_top,
        face_bottom,
        face_side,
        shell,
        solid,
    }
}

// ---------------------------------------------------------------------
// (a) Valid shapes the fixtures don't cover.
// ---------------------------------------------------------------------

#[test]
fn cube_by_hand_validates() {
    let tol = Tol::witness();
    let c = build_cube(tol);
    assert_eq!(validate(&c.body), Ok(()));
    assert_eq!(c.body.vertices().count(), 8);
    assert_eq!(c.body.edges().count(), 12);
    assert_eq!(c.body.faces().count(), 6);
    assert_eq!(c.body.half_edges().count(), 24);
}

#[test]
fn cube_orbit_direction_matches_hand_derivation() {
    let tol = Tol::witness();
    // Hand derivation at t1 = (1,0,1). Looking at the corner from
    // outside (viewer up-and-out along ~(1,-1,2)... any outside vantage
    // of the corner), the three half-edges leaving t1 are:
    //   top[1]  : t1 -> t2, direction (0,1,0)
    //   s_dn[1] : t1 -> b1, direction (0,0,-1)
    //   s_t[0]  : t1 -> t0, direction (-1,0,0)
    // Projected into the view plane of the outward corner direction
    // (~(1,-1,1)-ish is wrong for this corner; outward is average of
    // +z, +x-ish and -y-ish face normals => ~(1,-1,1)): with normal
    // toward viewer, going CLOCKWISE from "toward t2" one meets "down
    // to b1" then "toward t0" (derived by projecting the three
    // directions; see review report). next(mate(.)) must produce
    // exactly that order.
    let c = build_cube(tol);
    assert_eq!(
        c.body.vertex_orbit(c.top[1]),
        Some(vec![c.top[1], c.s_dn[1], c.s_t[0]])
    );
    // CCW inverse: mate(prev(top[1])) is the CW orbit's LAST member.
    let prev = c.body.get_half_edge(c.top[1]).unwrap().prev;
    assert_eq!(c.body.mate(prev), Some(c.s_t[0]));

    // Bottom corner b1 = (1,0,0): half-edges leaving b1 are
    //   s_b[1] : b1 -> b2, direction (0,1,0)
    //   bot[0] : b1 -> b0, direction (-1,0,0)
    //   s_up[0]: b1 -> t1, direction (0,0,1)
    // Viewed from outside/below the corner, clockwise from "toward b2"
    // is "toward b0" then "up to t1".
    assert_eq!(
        c.body.vertex_orbit(c.s_b[1]),
        Some(vec![c.s_b[1], c.bot[0], c.s_up[0]])
    );
}

#[test]
fn cube_mate_is_involution_and_antiparallel() {
    let tol = Tol::witness();
    let c = build_cube(tol);
    for (he_key, he) in c.body.half_edges() {
        let mate = c.body.mate(he_key).unwrap();
        assert_ne!(mate, he_key);
        assert_eq!(c.body.mate(mate), Some(he_key));
        // Antiparallel: end(he) == start(mate), end(mate) == start(he).
        assert_eq!(
            c.body.half_edge_end(he_key).unwrap(),
            c.body.get_half_edge(mate).unwrap().start
        );
        assert_eq!(c.body.half_edge_end(mate).unwrap(), he.start);
    }
}

/// A closed genus-0 body with a RING: two self-loop "circles" at lone
/// vertices v0, v1; face B = disk in circle 0, face C = disk in circle
/// 1, face A = annulus with outer = other half of circle 0 and one ring
/// = other half of circle 1. v2 e2 f3 r1: v-e+f = 3 = 2(s-h)+r = 2+1.
#[test]
fn ring_face_body_validates() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let null_he = HalfEdgeKey::default();
    let solid = body.add_solid(Solid { shells: vec![] }, prov());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);

    let lone = |body: &mut Body<f64>, x: f64| {
        let p = body.add_point(pt(x, 0.0, 0.0));
        body.add_vertex(
            Vertex {
                point: p,
                emanating: None,
            },
            prov(),
        )
    };
    let v0 = lone(&mut body, 0.0);
    let v1 = lone(&mut body, 1.0);

    let mk_edge = |body: &mut Body<f64>| {
        let curve = body.add_curve(crate::fixtures::test_curve(Point3::origin(), tol));
        body.add_edge(
            Edge {
                he_plus: null_he,
                he_minus: null_he,
                curve,
            },
            prov(),
        )
    };
    let e0 = mk_edge(&mut body);
    let e1 = mk_edge(&mut body);

    let mk_he = |body: &mut Body<f64>, edge: EdgeKey, start: VertexKey| {
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
    // circle 0 halves: a (disk B), a2 (annulus outer); circle 1: c
    // (disk C), c2 (annulus ring).
    let a = mk_he(&mut body, e0, v0);
    let a2 = mk_he(&mut body, e0, v0);
    let c = mk_he(&mut body, e1, v1);
    let c2 = mk_he(&mut body, e1, v1);
    body.get_edge_mut(e0).unwrap().he_plus = a;
    body.get_edge_mut(e0).unwrap().he_minus = a2;
    body.get_edge_mut(e1).unwrap().he_plus = c;
    body.get_edge_mut(e1).unwrap().he_minus = c2;

    let mk_loop = |body: &mut Body<f64>, first: HalfEdgeKey| {
        body.add_loop(
            Loop {
                boundary: LoopBoundary::Cycle { first },
                face: FaceKey::default(),
            },
            prov(),
        )
    };
    let loop_b = mk_loop(&mut body, a);
    let loop_a_outer = mk_loop(&mut body, a2);
    let loop_c = mk_loop(&mut body, c);
    let loop_a_ring = mk_loop(&mut body, c2);

    let mk_face = |body: &mut Body<f64>, outer: LoopKey, rings: Vec<LoopKey>| {
        let surface = body.add_surface(crate::fixtures::test_surface(Point3::origin()));
        body.add_face(
            Face {
                sense: true,
                surface,
                outer,
                rings,
                shell,
            },
            prov(),
        )
    };
    let face_b = mk_face(&mut body, loop_b, vec![]);
    let face_c = mk_face(&mut body, loop_c, vec![]);
    let face_a = mk_face(&mut body, loop_a_outer, vec![loop_a_ring]);
    body.get_loop_mut(loop_b).unwrap().face = face_b;
    body.get_loop_mut(loop_c).unwrap().face = face_c;
    body.get_loop_mut(loop_a_outer).unwrap().face = face_a;
    body.get_loop_mut(loop_a_ring).unwrap().face = face_a;
    {
        let faces = &mut body.get_shell_mut(shell).unwrap().faces;
        faces.extend([face_b, face_c, face_a]);
    }

    // 1-cycles.
    for (he, lp) in [
        (a, loop_b),
        (a2, loop_a_outer),
        (c, loop_c),
        (c2, loop_a_ring),
    ] {
        let h = body.get_half_edge_mut(he).unwrap();
        h.next = he;
        h.prev = he;
        h.parent_loop = lp;
    }
    body.get_vertex_mut(v0).unwrap().emanating = Some(a);
    body.get_vertex_mut(v1).unwrap().emanating = Some(c);

    assert_eq!(validate(&body), Ok(()));
    // Orbit at v0 covers both circle-0 halves.
    assert_eq!(body.vertex_orbit(a), Some(vec![a, a2]));
}

#[test]
fn two_disjoint_solids_in_one_body_validate() {
    let tol = Tol::witness();
    // Two hand-built strut bodies (the mev-after-mvfs state), each its
    // own solid, in one Body. Also covers: tier-1 legality of struts.
    let mut body = Body::<f64>::new();
    for k in 0..2 {
        add_strut_solid(&mut body, f64::from(k), tol);
    }
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(body.solids().count(), 2);
    assert_eq!(body.faces().count(), 2);
    assert_eq!(body.edges().count(), 2);
}

/// One solid+shell+face whose outer loop is the 2-cycle of a single
/// edge's two halves (a strut: v0 -> v1 -> v0) — the state `mev` on the
/// mvfs body produces. Both halves of the edge in the SAME loop.
fn add_strut_solid(body: &mut Body<f64>, offset: f64, tol: Tol) -> SolidKey {
    let null_he = HalfEdgeKey::default();
    let solid = body.add_solid(Solid { shells: vec![] }, prov());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);
    let p0 = body.add_point(pt(offset, 0.0, 0.0));
    let p1 = body.add_point(pt(offset, 1.0, 0.0));
    let v0 = body.add_vertex(
        Vertex {
            point: p0,
            emanating: None,
        },
        prov(),
    );
    let v1 = body.add_vertex(
        Vertex {
            point: p1,
            emanating: None,
        },
        prov(),
    );
    let curve = body.add_curve(crate::fixtures::test_curve(Point3::origin(), tol));
    let e = body.add_edge(
        Edge {
            he_plus: null_he,
            he_minus: null_he,
            curve,
        },
        prov(),
    );
    let x = body.add_half_edge(
        HalfEdge {
            edge: e,
            start: v0,
            parent_loop: LoopKey::default(),
            next: null_he,
            prev: null_he,
        },
        prov(),
    );
    let y = body.add_half_edge(
        HalfEdge {
            edge: e,
            start: v1,
            parent_loop: LoopKey::default(),
            next: null_he,
            prev: null_he,
        },
        prov(),
    );
    body.get_edge_mut(e).unwrap().he_plus = x;
    body.get_edge_mut(e).unwrap().he_minus = y;
    let lp = body.add_loop(
        Loop {
            boundary: LoopBoundary::Cycle { first: x },
            face: FaceKey::default(),
        },
        prov(),
    );
    let surface = body.add_surface(crate::fixtures::test_surface(Point3::origin()));
    let f = body.add_face(
        Face {
            sense: true,
            surface,
            outer: lp,
            rings: vec![],
            shell,
        },
        prov(),
    );
    body.get_loop_mut(lp).unwrap().face = f;
    body.get_shell_mut(shell).unwrap().faces.push(f);
    for (he, nxt) in [(x, y), (y, x)] {
        let h = body.get_half_edge_mut(he).unwrap();
        h.next = nxt;
        h.prev = nxt;
        h.parent_loop = lp;
    }
    body.get_vertex_mut(v0).unwrap().emanating = Some(x);
    body.get_vertex_mut(v1).unwrap().emanating = Some(y);
    solid
}

#[test]
fn mvfs_skeletal_state_validates_externally() {
    let mut body = Body::<f64>::new();
    let p = body.add_point(pt(0.0, 0.0, 0.0));
    let v = body.add_vertex(
        Vertex {
            point: p,
            emanating: None,
        },
        prov(),
    );
    let solid = body.add_solid(Solid { shells: vec![] }, prov());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);
    let surface = body.add_surface(crate::fixtures::test_surface(Point3::origin()));
    let lp = body.add_loop(
        Loop {
            boundary: LoopBoundary::Empty { vertex: v },
            face: FaceKey::default(),
        },
        prov(),
    );
    let f = body.add_face(
        Face {
            sense: true,
            surface,
            outer: lp,
            rings: vec![],
            shell,
        },
        prov(),
    );
    body.get_loop_mut(lp).unwrap().face = f;
    body.get_shell_mut(shell).unwrap().faces.push(f);
    assert_eq!(validate(&body), Ok(()));
}

// ---------------------------------------------------------------------
// (b) Adversarial: structurally invalid bodies must all FAIL — at least
// one error, no panic, no hang.
// ---------------------------------------------------------------------

/// Mate-swap that keeps antiparallelism: in the cube, edges et[0]
/// (t0-t1) and et[2] (t2-t3) don't share vertices, so cross-pairing
/// breaks antiparallelism and must be caught.
#[test]
fn mate_swap_across_parallel_edges_is_caught() {
    let tol = Tol::witness();
    let mut c = build_cube(tol);
    // et[0] claims (top[0], s_t[2]); et[2] claims (top[2], s_t[0]);
    // patch backpointers so the bijection LOOKS clean.
    c.body.get_edge_mut(c.et[0]).unwrap().he_minus = c.s_t[2];
    c.body.get_edge_mut(c.et[2]).unwrap().he_minus = c.s_t[0];
    c.body.get_half_edge_mut(c.s_t[2]).unwrap().edge = c.et[0];
    c.body.get_half_edge_mut(c.s_t[0]).unwrap().edge = c.et[2];
    let errors = validate(&c.body).unwrap_err();
    assert!(!errors.is_empty());
    println!("mate-swap errors: {errors:?}");
}

/// Mate-swap that DOES keep antiparallelism (digon: both edges join the
/// same two vertices): pair each edge with its own loop-mate. Every
/// local check passes; only orbit closure can catch it.
#[test]
fn antiparallelism_preserving_mate_swap_is_caught_by_orbits() {
    let tol = Tol::witness();
    // Digon pillow by hand: v0,v1; e0, e1 both v0->v1; face A cycle
    // [a0: v0->v1, a1: v1->v0], face B cycle [b0: v1->v0, b1: v0->v1].
    // Correct pairing: e0=(a0,b0), e1=(a1,b1). Adversarial pairing:
    // e0=(a0,a1) (both in loop A!), e1=(b1,b0) — still antiparallel.
    let mut body = Body::<f64>::new();
    let null_he = HalfEdgeKey::default();
    let solid = body.add_solid(Solid { shells: vec![] }, prov());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);
    let p0 = body.add_point(pt(0.0, 0.0, 0.0));
    let p1 = body.add_point(pt(1.0, 0.0, 0.0));
    let v0 = body.add_vertex(
        Vertex {
            point: p0,
            emanating: None,
        },
        prov(),
    );
    let v1 = body.add_vertex(
        Vertex {
            point: p1,
            emanating: None,
        },
        prov(),
    );
    let mk_edge = |body: &mut Body<f64>| {
        let curve = body.add_curve(crate::fixtures::test_curve(Point3::origin(), tol));
        body.add_edge(
            Edge {
                he_plus: null_he,
                he_minus: null_he,
                curve,
            },
            prov(),
        )
    };
    let e0 = mk_edge(&mut body);
    let e1 = mk_edge(&mut body);
    let mk_he = |body: &mut Body<f64>, edge: EdgeKey, start: VertexKey| {
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
    let a0 = mk_he(&mut body, e0, v0);
    let a1 = mk_he(&mut body, e1, v1);
    let b0 = mk_he(&mut body, e0, v1);
    let b1 = mk_he(&mut body, e1, v0);
    let mk_loop = |body: &mut Body<f64>, first: HalfEdgeKey| {
        body.add_loop(
            Loop {
                boundary: LoopBoundary::Cycle { first },
                face: FaceKey::default(),
            },
            prov(),
        )
    };
    let loop_a = mk_loop(&mut body, a0);
    let loop_b = mk_loop(&mut body, b0);
    let mk_face = |body: &mut Body<f64>, outer: LoopKey| {
        let surface = body.add_surface(crate::fixtures::test_surface(Point3::origin()));
        body.add_face(
            Face {
                sense: true,
                surface,
                outer,
                rings: vec![],
                shell,
            },
            prov(),
        )
    };
    let face_a = mk_face(&mut body, loop_a);
    let face_b = mk_face(&mut body, loop_b);
    body.get_loop_mut(loop_a).unwrap().face = face_a;
    body.get_loop_mut(loop_b).unwrap().face = face_b;
    body.get_shell_mut(shell).unwrap().faces = vec![face_a, face_b];
    for (he, lp, nxt) in [
        (a0, loop_a, a1),
        (a1, loop_a, a0),
        (b0, loop_b, b1),
        (b1, loop_b, b0),
    ] {
        let h = body.get_half_edge_mut(he).unwrap();
        h.parent_loop = lp;
        h.next = nxt;
        h.prev = nxt;
    }
    body.get_vertex_mut(v0).unwrap().emanating = Some(a0);
    body.get_vertex_mut(v1).unwrap().emanating = Some(a1);
    // ADVERSARIAL pairing: same-loop mates.
    body.get_edge_mut(e0).unwrap().he_plus = a0;
    body.get_edge_mut(e0).unwrap().he_minus = a1;
    body.get_edge_mut(e1).unwrap().he_plus = b1;
    body.get_edge_mut(e1).unwrap().he_minus = b0;
    body.get_half_edge_mut(a1).unwrap().edge = e0;
    body.get_half_edge_mut(b1).unwrap().edge = e1;
    body.get_half_edge_mut(b0).unwrap().edge = e1;

    let errors = validate(&body).unwrap_err();
    assert!(!errors.is_empty());
    println!("antiparallel-preserving mate-swap errors: {errors:?}");
}

/// parent_loop pointing at a loop whose Cycle.first sits in a different
/// cycle, which itself claims a third loop — a chain of confusions.
#[test]
fn parent_loop_chain_confusion_is_caught() {
    let tol = Tol::witness();
    let mut c = build_cube(tol);
    // top[0]'s parent_loop -> loop_side[0], whose first (s_b[0]) is in
    // the side-0 cycle, whose members claim loop_side[0]... then also
    // repoint loop_side[0].first at top[1] (in the TOP cycle).
    c.body.get_half_edge_mut(c.top[0]).unwrap().parent_loop = c.loop_side[0];
    let LoopBoundary::Cycle { .. } = c.body.get_loop(c.loop_side[0]).unwrap().boundary else {
        panic!("expected cycle");
    };
    c.body.get_loop_mut(c.loop_side[0]).unwrap().boundary = LoopBoundary::Cycle { first: c.top[1] };
    let errors = validate(&c.body).unwrap_err();
    assert!(!errors.is_empty());
    println!("parent-loop chain confusion errors: {errors:?}");
}

/// An Empty loop whose vertex also starts half-edges elsewhere.
#[test]
fn empty_loop_vertex_with_incidence_is_caught() {
    let tol = Tol::witness();
    let mut c = build_cube(tol);
    let surface = c
        .body
        .add_surface(crate::fixtures::test_surface(Point3::origin()));
    let lp = c.body.add_loop(
        Loop {
            boundary: LoopBoundary::Empty { vertex: c.t[0] },
            face: FaceKey::default(),
        },
        prov(),
    );
    let f = c.body.add_face(
        Face {
            sense: true,
            surface,
            outer: lp,
            rings: vec![],
            shell: c.shell,
        },
        prov(),
    );
    c.body.get_loop_mut(lp).unwrap().face = f;
    c.body.get_shell_mut(c.shell).unwrap().faces.push(f);
    let errors = validate(&c.body).unwrap_err();
    assert!(!errors.is_empty());
    println!("empty-loop-vertex-with-incidence errors: {errors:?}");
}

/// A half-edge referenced by two edges.
#[test]
fn half_edge_claimed_by_two_edges_is_caught() {
    let tol = Tol::witness();
    let mut c = build_cube(tol);
    c.body.get_edge_mut(c.eb[0]).unwrap().he_minus = c.top[0];
    let errors = validate(&c.body).unwrap_err();
    assert!(!errors.is_empty());
    println!("double-claim errors: {errors:?}");
}

/// Ownership double-count: face A's outer listed among face B's rings.
#[test]
fn outer_of_one_face_as_ring_of_another_is_caught() {
    let tol = Tol::witness();
    let mut c = build_cube(tol);
    let lt = c.loop_top;
    c.body.get_face_mut(c.face_side[0]).unwrap().rings.push(lt);
    let errors = validate(&c.body).unwrap_err();
    assert!(!errors.is_empty());
    println!("cross-face outer-as-ring errors: {errors:?}");
}

/// Self-referential next inside a longer cycle.
#[test]
fn self_referential_next_is_caught_without_hanging() {
    let tol = Tol::witness();
    let mut c = build_cube(tol);
    let he = c.top[2];
    c.body.get_half_edge_mut(he).unwrap().next = he;
    let errors = validate(&c.body).unwrap_err();
    assert!(!errors.is_empty());
    println!("self-next errors: {errors:?}");
}

/// The two halves of one edge moved into loops of the same face
/// (outer + a bogus ring arrangement) — via next-splice corruption.
#[test]
fn next_splice_across_loops_is_caught() {
    let tol = Tol::witness();
    let mut c = build_cube(tol);
    // Splice the top loop and side-0 loop into one big next-cycle while
    // leaving both Loop records claiming their old firsts.
    let top0_next = c.body.get_half_edge(c.top[0]).unwrap().next;
    let sb0_next = c.body.get_half_edge(c.s_b[0]).unwrap().next;
    c.body.get_half_edge_mut(c.top[0]).unwrap().next = sb0_next;
    c.body.get_half_edge_mut(c.s_b[0]).unwrap().next = top0_next;
    let errors = validate(&c.body).unwrap_err();
    assert!(!errors.is_empty());
    println!("next-splice errors: {errors:?}");
}

/// Long corrupted chain: a large body with one torn link must terminate
/// promptly (bounded walks) and report errors.
#[test]
fn long_corrupted_chain_terminates() {
    let tol = Tol::witness();
    // Build a large ngon-pillow-like body externally: two n-gon faces.
    let n = 3000;
    let mut body = Body::<f64>::new();
    let null_he = HalfEdgeKey::default();
    let solid = body.add_solid(Solid { shells: vec![] }, prov());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);
    let vs: Vec<_> = (0..n)
        .map(|i| {
            let p = body.add_point(pt(i as f64, 0.0, 0.0));
            body.add_vertex(
                Vertex {
                    point: p,
                    emanating: None,
                },
                prov(),
            )
        })
        .collect();
    let es: Vec<_> = (0..n)
        .map(|_| {
            let curve = body.add_curve(crate::fixtures::test_curve(Point3::origin(), tol));
            body.add_edge(
                Edge {
                    he_plus: null_he,
                    he_minus: null_he,
                    curve,
                },
                prov(),
            )
        })
        .collect();
    let ha: Vec<_> = (0..n)
        .map(|i| {
            body.add_half_edge(
                HalfEdge {
                    edge: es[i],
                    start: vs[i],
                    parent_loop: LoopKey::default(),
                    next: null_he,
                    prev: null_he,
                },
                prov(),
            )
        })
        .collect();
    let hb: Vec<_> = (0..n)
        .map(|i| {
            body.add_half_edge(
                HalfEdge {
                    edge: es[i],
                    start: vs[(i + 1) % n],
                    parent_loop: LoopKey::default(),
                    next: null_he,
                    prev: null_he,
                },
                prov(),
            )
        })
        .collect();
    let loop_a = body.add_loop(
        Loop {
            boundary: LoopBoundary::Cycle { first: ha[0] },
            face: FaceKey::default(),
        },
        prov(),
    );
    let loop_b = body.add_loop(
        Loop {
            boundary: LoopBoundary::Cycle { first: hb[0] },
            face: FaceKey::default(),
        },
        prov(),
    );
    let mk_face = |body: &mut Body<f64>, outer: LoopKey| {
        let surface = body.add_surface(crate::fixtures::test_surface(Point3::origin()));
        body.add_face(
            Face {
                sense: true,
                surface,
                outer,
                rings: vec![],
                shell,
            },
            prov(),
        )
    };
    let face_a = mk_face(&mut body, loop_a);
    let face_b = mk_face(&mut body, loop_b);
    body.get_loop_mut(loop_a).unwrap().face = face_a;
    body.get_loop_mut(loop_b).unwrap().face = face_b;
    body.get_shell_mut(shell).unwrap().faces = vec![face_a, face_b];
    for i in 0..n {
        let a = body.get_half_edge_mut(ha[i]).unwrap();
        a.parent_loop = loop_a;
        a.next = ha[(i + 1) % n];
        a.prev = ha[(i + n - 1) % n];
        let b = body.get_half_edge_mut(hb[i]).unwrap();
        b.parent_loop = loop_b;
        b.next = hb[(i + n - 1) % n];
        b.prev = hb[(i + 1) % n];
    }
    for i in 0..n {
        let e = body.get_edge_mut(es[i]).unwrap();
        e.he_plus = ha[i];
        e.he_minus = hb[i];
        body.get_vertex_mut(vs[i]).unwrap().emanating = Some(ha[i]);
    }
    assert_eq!(validate(&body), Ok(()));

    // Tear ONE link so loop A's walk diverges into loop B's cycle: the
    // walk must overrun at the arena bound, not hang.
    body.get_half_edge_mut(ha[0]).unwrap().next = hb[0];
    let start = std::time::Instant::now();
    let errors = validate(&body).unwrap_err();
    let elapsed = start.elapsed();
    assert!(!errors.is_empty());
    println!(
        "large-corrupt validate: {} errors in {elapsed:?}",
        errors.len()
    );
    assert!(elapsed.as_secs() < 30, "validator too slow: {elapsed:?}");
    // The public walks are also bounded.
    assert_eq!(body.loop_cycle(ha[0]), None);
    assert_eq!(body.vertex_orbit(ha[1]), None);
}

// ---------------------------------------------------------------------
// Totality: stale/null keys into every public accessor.
// ---------------------------------------------------------------------

#[test]
fn null_keys_are_none_everywhere_no_panics() {
    let tol = Tol::witness();
    let mut c = build_cube(tol);
    let b = &c.body;
    assert!(b.get_solid(SolidKey::default()).is_none());
    assert!(b.get_shell(ShellKey::default()).is_none());
    assert!(b.get_face(FaceKey::default()).is_none());
    assert!(b.get_loop(LoopKey::default()).is_none());
    assert!(b.get_half_edge(HalfEdgeKey::default()).is_none());
    assert!(b.get_edge(EdgeKey::default()).is_none());
    assert!(b.get_vertex(VertexKey::default()).is_none());
    assert!(b.get_point(crate::PointKey::default()).is_none());
    assert!(b.get_curve_geom(crate::CurveKey::default()).is_none());
    assert!(b.get_surface(crate::SurfaceKey::default()).is_none());
    assert!(b.mate(HalfEdgeKey::default()).is_none());
    assert!(b.half_edge_end(HalfEdgeKey::default()).is_none());
    assert!(b.loop_cycle(HalfEdgeKey::default()).is_none());
    assert!(b.vertex_orbit(HalfEdgeKey::default()).is_none());
    assert!(
        b.provenance(EntityId::Vertex(VertexKey::default()))
            .is_none()
    );
    assert!(c.body.get_vertex_mut(VertexKey::default()).is_none());
    assert!(c.body.get_half_edge_mut(HalfEdgeKey::default()).is_none());
    assert!(c.body.get_edge_mut(EdgeKey::default()).is_none());
    assert!(c.body.get_loop_mut(LoopKey::default()).is_none());
    assert!(c.body.get_face_mut(FaceKey::default()).is_none());
    assert!(c.body.get_shell_mut(ShellKey::default()).is_none());
    assert!(c.body.get_solid_mut(SolidKey::default()).is_none());
}

#[test]
fn foreign_keys_resolve_arbitrarily_as_documented() {
    let tol = Tol::witness();
    // Same construction history => same keys (lineage determinism);
    // a foreign key from an identically-built body resolves.
    let c1 = build_cube(tol);
    let c2 = build_cube(tol);
    assert!(c2.body.get_vertex(c1.t[0]).is_some());
    // And a key from a DIFFERENT history may or may not resolve — it
    // must never panic.
    let pillow_key = {
        let mut other = Body::<f64>::new();
        other.add_solid(Solid { shells: vec![] }, prov())
    };
    let _ = c1.body.get_solid(pillow_key); // no panic is the assertion
}

// ---------------------------------------------------------------------
// Known-gap probes — the gap this probe documented is now CLOSED.
// ---------------------------------------------------------------------

/// A face moved to a second shell (in a second solid): the edge's two
/// faces now sit in DIFFERENT shells/solids. As reviewed, this
/// documented a GAP — the body still validated, with shell-partition
/// coherence deferred to PR 5. **Adapted at the PR 5 move**: the
/// deferred passes exist now, so the probe's assertion is inverted —
/// the same corruption is reported by BOTH new passes (four
/// `EdgeAcrossShells`, one odd-characteristic `ComponentEulerViolation`
/// per shell) and by nothing else. The construction is verbatim.
#[test]
fn cross_shell_face_move_gap_probe_now_caught() {
    let tol = Tol::witness();
    let mut c = build_cube(tol);
    let solid2 = c.body.add_solid(Solid { shells: vec![] }, prov());
    let shell2 = c.body.add_shell(
        Shell {
            faces: vec![],
            solid: solid2,
        },
        prov(),
    );
    c.body.get_solid_mut(solid2).unwrap().shells.push(shell2);
    // Move side face 0 into shell2.
    let f = c.face_side[0];
    c.body.get_face_mut(f).unwrap().shell = shell2;
    let faces = &mut c.body.get_shell_mut(c.shell).unwrap().faces;
    faces.retain(|&k| k != f);
    c.body.get_shell_mut(shell2).unwrap().faces.push(f);
    let errors = validate(&c.body).unwrap_err();
    let crossings = errors
        .iter()
        .filter(|e| matches!(e, crate::ValidationError::EdgeAcrossShells { .. }))
        .count();
    let component_failures: Vec<_> = errors
        .iter()
        .filter_map(|e| match e {
            crate::ValidationError::ComponentEulerViolation {
                shell,
                vertices,
                edges,
                faces,
                rings,
                ..
            } => Some((
                *shell,
                *vertices as i64 - *edges as i64 + *faces as i64 - *rings as i64,
            )),
            _ => None,
        })
        .collect();
    assert_eq!(crossings, 4, "the moved face's four edges cross shells");
    // χ = 8 − 12 + 5 = 1 on the mutilated shell; 4 − 4 + 1 = 1 on the
    // lone-face shell — both odd, both reported.
    assert_eq!(component_failures, vec![(c.shell, 1), (shell2, 1)]);
    assert_eq!(errors.len(), 6, "and nothing else fires");
}

// ---------------------------------------------------------------------
// PR-2 readiness: an lmev-shaped surgery through the public API.
// ---------------------------------------------------------------------

/// Sketch of `lmev`'s simplest case (mev creating a strut into an
/// existing loop): insert a new vertex nv and edge v->nv, splicing two
/// new half-edges into the top loop right before `he`. Exercises: find
/// half-edge in loop L starting at V; splice next/prev; patch
/// emanating; the result must validate.
#[test]
fn lmev_strut_surgery_sketch_validates() {
    let tol = Tol::witness();
    let mut c = build_cube(tol);
    let v = c.t[1];
    // Find the half-edge in loop_top starting at v (consumer idiom:
    // read the loop, walk its cycle, filter by start).
    let LoopBoundary::Cycle { first } = c.body.get_loop(c.loop_top).unwrap().boundary else {
        panic!("top loop must be a cycle");
    };
    let cycle = c.body.loop_cycle(first).unwrap();
    let he = *cycle
        .iter()
        .find(|&&h| c.body.get_half_edge(h).unwrap().start == v)
        .unwrap();
    assert_eq!(he, c.top[1]);

    // New vertex + edge, two half-edges spliced in before `he`:
    // ... x (ends at v) | a: v->nv | b: nv->v | he (starts at v) ...
    let np = c.body.add_point(pt(1.0, 0.0, 2.0));
    let nv = c.body.add_vertex(
        Vertex {
            point: np,
            emanating: None,
        },
        prov(),
    );
    let curve = c
        .body
        .add_curve(crate::fixtures::test_curve(Point3::origin(), tol));
    let ne = c.body.add_edge(
        Edge {
            he_plus: HalfEdgeKey::default(),
            he_minus: HalfEdgeKey::default(),
            curve,
        },
        prov(),
    );
    let x = c.body.get_half_edge(he).unwrap().prev;
    let a = c.body.add_half_edge(
        HalfEdge {
            edge: ne,
            start: v,
            parent_loop: c.loop_top,
            next: HalfEdgeKey::default(),
            prev: x,
        },
        prov(),
    );
    let b = c.body.add_half_edge(
        HalfEdge {
            edge: ne,
            start: nv,
            parent_loop: c.loop_top,
            next: he,
            prev: a,
        },
        prov(),
    );
    c.body.get_half_edge_mut(a).unwrap().next = b;
    c.body.get_half_edge_mut(x).unwrap().next = a;
    c.body.get_half_edge_mut(he).unwrap().prev = b;
    let e = c.body.get_edge_mut(ne).unwrap();
    e.he_plus = a;
    e.he_minus = b;
    c.body.get_vertex_mut(nv).unwrap().emanating = Some(b);

    assert_eq!(validate(&c.body), Ok(()));
    // The new vertex's orbit is the single returning half-edge.
    assert_eq!(c.body.vertex_orbit(b), Some(vec![b]));
    // v's orbit grew from 3 to 4 and includes `a`.
    let orbit = c.body.vertex_orbit(c.top[1]).unwrap();
    assert_eq!(orbit.len(), 4);
    assert!(orbit.contains(&a));
}
