//! Adversarial e2e review artifact for M1 PR 2 (2026-07-16). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value. Promoted per
//! Evan's request (PR #17 thread).
//!
//! Independent cube reproduction — NOT a copy of the shipped test.
//!
//! Shipped test: seeds at bottom corner A, builds the bottom chain,
//! closes the bottom face, raises struts UP, the seed face ends as TOP.
//! This one: seeds at top corner A', builds the TOP chain A'->B'->C'->D',
//! closes the TOP face with the first mef, drops struts DOWN, and the
//! seed face ends as the BOTTOM. Different addressing throughout.
//!
//! Corners: A(0,0,0) B(1,0,0) C(1,1,0) D(0,1,0), primed at z=1.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::{euler_poincare_holds, face_polygon, signed_area, start_xyz};
use crate::{Body, LoopBoundary, MefSite, MevSite, validate};
use geom_core::Point3;
use geom_core::Tol;

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

#[test]
fn independent_cube_full_verification() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();

    // Seed at A'.
    let seed = body.mvfs(pt(0.0, 0.0, 1.0)).unwrap();
    assert_eq!(validate(&body), Ok(()));

    // Top chain A'->B'->C'->D' : one segment + two struts.
    // h0: A'->B'
    let h0 = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(1.0, 0.0, 1.0),
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    // h1: B'->C'  (strut at B' = start of h0.he_minus)
    let h1 = body
        .mev_line(
            MevSite::Fan {
                he1: h0.he_minus,
                he2: h0.he_minus,
            },
            pt(1.0, 1.0, 1.0),
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    // h2: C'->D'  (strut at C')
    let h2 = body
        .mev_line(
            MevSite::Fan {
                he1: h1.he_minus,
                he2: h1.he_minus,
            },
            pt(0.0, 1.0, 1.0),
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));

    // Close the TOP square: he1 = h0.he_plus (A'->B'), he2 = h2.he_minus
    // (D'->C'). Run [A'->B', B'->C', C'->D') becomes the new (top)
    // face's outer loop, closed by he_minus = D'->A'... no: new edge is
    // start(he1)->start(he2) = A'->D', he_minus runs D'->A' in the new
    // loop? he_minus = start(he2)->start(he1) = D'->A'. New loop cycle:
    // [he_minus? spliced before he1] => [A'->B', B'->C', C'->D', D'->A']
    // = CCW from ABOVE = outward +z = the TOP. Old (seed) loop keeps
    // he_plus (A'->D') and the minus halves: A'->D'->C'->B' -- the
    // "everything else" lamina side.
    let f_top = body
        .mef_chord(MefSite::Chords {
            he1: h0.he_plus,
            he2: h2.he_minus,
        }, tol)
        .unwrap();
    assert_eq!(validate(&body), Ok(()));

    // Cross-check find_half_edge against created keys.
    assert_eq!(
        body.find_half_edge(f_top.face, seed.vertex, h0.vertex),
        Some(h0.he_plus),
        "A'->B' lives in the top face"
    );
    assert_eq!(
        body.find_half_edge(seed.face, h0.vertex, seed.vertex),
        Some(h0.he_minus),
        "B'->A' lives in the seed (lamina) face"
    );

    // Drop the four verticals DOWN into the seed-face loop. The half in
    // the seed loop starting at each primed corner:
    //   A': f_top.he_plus (A'->D'), D': h2.he_minus (D'->C'),
    //   C': h1.he_minus (C'->B'),  B': h0.he_minus (B'->A').
    let strut = |body: &mut Body<f64>, at, x, y| {
        let c = body
            .mev_line(MevSite::Fan { he1: at, he2: at }, pt(x, y, 0.0), tol)
            .unwrap();
        assert_eq!(validate(body), Ok(()));
        c
    };
    let s_a = strut(&mut body, f_top.he_plus, 0.0, 0.0); // A'->A
    let s_d = strut(&mut body, h2.he_minus, 0.0, 1.0); // D'->D
    let s_c = strut(&mut body, h1.he_minus, 1.0, 1.0); // C'->C
    let s_b = strut(&mut body, h0.he_minus, 1.0, 0.0); // B'->B

    // Cut the four side faces out of the seed loop. Derived by hand:
    // seed loop is now [s_a+ (A'->A), s_a- (A->A'), f_top+ (A'->D'),
    //  s_d+ (D'->D), s_d- (D->D'), h2- (D'->C'), s_c+ (C'->C),
    //  s_c- (C->C'), h1- (C'->B'), s_b+ (B'->B), s_b- (B->B'),
    //  h0- (B'->A')].
    let side = |body: &mut Body<f64>, he1, he2| {
        let c = body.mef_chord(MefSite::Chords { he1, he2 }, tol).unwrap();
        assert_eq!(validate(body), Ok(()));
        c
    };
    // Front A->B->B'->A': run [s_b- (B->B'), h0- (B'->A'), s_a+ (A'->A));
    // he1 = s_b-, he2 = s_a- (A->A'). New edge B->A, he_minus A->B.
    let f_front = side(&mut body, s_b.he_minus, s_a.he_minus);
    // Right B->C->C'->B': run [s_c- (C->C'), h1- (C'->B'), s_b+ (B'->B));
    // he2 = the front face's he_plus (B->A), which stayed in the seed loop.
    let f_right = side(&mut body, s_c.he_minus, f_front.he_plus);
    // Back C->D->D'->C': run [s_d- (D->D'), h2- (D'->C'), s_c+ (C'->C));
    // he2 = right's he_plus (C->B).
    let f_back = side(&mut body, s_d.he_minus, f_right.he_plus);
    // Left D->A->A'->D': run [s_a- (A->A'), f_top+ (A'->D'), s_d+ (D'->D));
    // he1 = s_a-, he2 = back's he_plus (D->C). New edge A->D.
    let f_left = side(&mut body, s_a.he_minus, f_back.he_plus);

    // ---- Counts (Mantyla's provably minimal cube). ----
    assert_eq!(body.vertices().count(), 8);
    assert_eq!(body.edges().count(), 12);
    assert_eq!(body.faces().count(), 6);
    assert_eq!(body.loops().count(), 6);
    assert_eq!(body.half_edges().count(), 24);
    assert_eq!(body.shells().count(), 1);
    assert_eq!(body.solids().count(), 1);
    assert_eq!(body.points().count(), 8);
    assert_eq!(body.curves().count(), 12);
    assert!(euler_poincare_holds(&body, 1, 0));

    // ---- Surface-sharing rule: ONE surface, shared by all six faces. ----
    assert_eq!(body.surfaces().count(), 1);
    for (_, face) in body.faces() {
        assert_eq!(face.surface, seed.surface, "face split shares the surface");
        assert!(face.rings.is_empty());
    }

    // ---- Orientation, end-to-end, all six faces. ----
    // For each face: polygon must be planar on a cube facet; outward
    // normal = facet direction (away from cube center (.5,.5,.5));
    // explicit view basis (u, v) with u x v = outward; signed area must
    // be POSITIVE (CCW viewed from outside).
    let faces = [
        ("bottom", seed.face),
        ("top", f_top.face),
        ("front", f_front.face),
        ("right", f_right.face),
        ("back", f_back.face),
        ("left", f_left.face),
    ];
    for (name, fk) in faces {
        let poly = face_polygon(&body, fk);
        assert_eq!(poly.len(), 4, "{name}: quad");
        let cx = poly.iter().map(|p| p.0).sum::<f64>() / 4.0;
        let cy = poly.iter().map(|p| p.1).sum::<f64>() / 4.0;
        let cz = poly.iter().map(|p| p.2).sum::<f64>() / 4.0;
        let out = (cx - 0.5, cy - 0.5, cz - 0.5);
        // out is +-0.5 along exactly one axis; build u,v with u x v = out_hat.
        let (u, v, label) = if out.2 > 0.4 {
            ((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), "+z")
        } else if out.2 < -0.4 {
            ((0.0, 1.0, 0.0), (1.0, 0.0, 0.0), "-z")
        } else if out.1 > 0.4 {
            ((0.0, 0.0, 1.0), (1.0, 0.0, 0.0), "+y")
        } else if out.1 < -0.4 {
            ((1.0, 0.0, 0.0), (0.0, 0.0, 1.0), "-y")
        } else if out.0 > 0.4 {
            ((0.0, 1.0, 0.0), (0.0, 0.0, 1.0), "+x")
        } else {
            ((0.0, 0.0, 1.0), (0.0, 1.0, 0.0), "-x")
        };
        let (ux, uy, uz) = u;
        let (vx, vy, vz) = v;
        // Verify u x v points outward (basis sanity, by hand).
        let n = (uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx);
        assert!(
            n.0 * out.0 + n.1 * out.1 + n.2 * out.2 > 0.0,
            "{name}: basis error in the test itself"
        );
        let area = signed_area(&poly, u, v);
        assert!(
            area > 0.99 && area < 1.01,
            "{name} ({label}): expected CCW-from-outside unit quad, signed area = {area}"
        );
    }

    // ---- Two adjacent faces in detail: top and front share edge A'B'. ----
    // (Loop anchors are arbitrary; compare cycles up to rotation.)
    let rotate_to = |poly: Vec<(f64, f64, f64)>, first: (f64, f64, f64)| {
        let i = poly.iter().position(|&p| p == first).unwrap();
        let mut r = poly[i..].to_vec();
        r.extend_from_slice(&poly[..i]);
        r
    };
    let top_poly = rotate_to(face_polygon(&body, f_top.face), (0.0, 0.0, 1.0));
    assert_eq!(
        top_poly,
        vec![
            (0.0, 0.0, 1.0), // A'
            (1.0, 0.0, 1.0), // B'
            (1.0, 1.0, 1.0), // C'
            (0.0, 1.0, 1.0), // D'
        ],
        "top loop walks A'->B'->C'->D' (CCW from +z)"
    );
    let front_poly = rotate_to(face_polygon(&body, f_front.face), (0.0, 0.0, 0.0));
    assert_eq!(
        front_poly,
        vec![
            (0.0, 0.0, 0.0), // A
            (1.0, 0.0, 0.0), // B
            (1.0, 0.0, 1.0), // B'
            (0.0, 0.0, 1.0), // A'
        ],
        "front loop walks A->B->B'->A' (CCW from -y)"
    );
    // Mates antiparallel across the shared edge A'-B': the top face
    // traverses A'->B' (h0.he_plus), the front face must traverse B'->A'
    // (its mate), in the front loop.
    let mate = body.mate(h0.he_plus).unwrap();
    assert_eq!(mate, h0.he_minus);
    assert_eq!(start_xyz(&body, h0.he_plus), (0.0, 0.0, 1.0));
    assert_eq!(start_xyz(&body, mate), (1.0, 0.0, 1.0));
    assert_eq!(
        body.get_half_edge(h0.he_plus).unwrap().parent_loop,
        f_top.r#loop
    );
    assert_eq!(
        body.get_half_edge(mate).unwrap().parent_loop,
        body.get_face(f_front.face).unwrap().outer
    );

    // Mate antiparallelism with COORDINATES for every edge of the cube.
    for (_, edge) in body.edges() {
        let ps = start_xyz(&body, edge.he_plus);
        let pe_v = body.half_edge_end(edge.he_plus).unwrap();
        let ms = start_xyz(&body, edge.he_minus);
        let me_v = body.half_edge_end(edge.he_minus).unwrap();
        let p_of = |v: crate::VertexKey| {
            let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            (p.x, p.y, p.z)
        };
        assert_eq!(ps, p_of(me_v), "plus start == minus end");
        assert_eq!(ms, p_of(pe_v), "minus start == plus end");
    }

    // Every vertex valence 3, every loop a quad.
    for (_, vertex) in body.vertices() {
        assert_eq!(
            body.vertex_orbit(vertex.emanating.unwrap()).unwrap().len(),
            3
        );
    }
    for (_, l) in body.loops() {
        let LoopBoundary::Cycle { first } = l.boundary else {
            panic!("no empty loops in the finished cube");
        };
        assert_eq!(body.loop_cycle(first).unwrap().len(), 4);
    }
}
