//! Adversarial e2e review artifact for M1 PR 2 (2026-07-16). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value. Promoted per
//! Ev's request (PR #17 thread).
//!
//! Independent fan-semantics verification (NOT the shipped valence-4
//! fixture): a valence-5 star, hand-derived orbit, asymmetric split, the
//! moved-emanating attack, and a cross-loop fan on a closed lamina.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::{Body, MefSite, MevSite, validate};
use geom_core::Point3;
use geom_core::Tol;

fn p(x: f64) -> Point3<f64> {
    Point3::new(x, 0.0, 0.0)
}

/// mvfs + segment + 4 struts AT THE SEED VERTEX v: valence-5 star.
///
/// Hand derivation of the orbit (from the documented strut splice, not
/// from vertex_orbit): the segment makes loop [g+, g-]; each strut
/// Fan{he1: g+, he2: g+} splices ... -> prev(g+) -> t+ -> t- -> g+, so
/// the loop grows [g+, g-, t1+, t1-, ..., tk+, tk-]. Orbit step
/// next(mate(.)): from g+ -> mate=g-, g-.next = t1+; from tk+ ->
/// mate=tk-, tk-.next = t(k+1)+ (or wraps to g+). So the CW orbit at v
/// is [g+, t1+, t2+, t3+, t4+].
#[test]
fn valence_five_fan_split_moves_the_clockwise_run() {
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
    let strut = |body: &mut Body<f64>, x: f64| {
        body.mev_line(
            MevSite::Fan {
                he1: g.he_plus,
                he2: g.he_plus,
            },
            p(x),
            tol,
        )
        .unwrap()
    };
    let t1 = strut(&mut body, 2.0);
    let t2 = strut(&mut body, 3.0);
    let t3 = strut(&mut body, 4.0);
    let t4 = strut(&mut body, 5.0);
    assert_eq!(validate(&body), Ok(()));

    // Pin the hand-derived orbit BEFORE trusting any split.
    assert_eq!(
        body.vertex_orbit(g.he_plus),
        Some(vec![
            g.he_plus, t1.he_plus, t2.he_plus, t3.he_plus, t4.he_plus
        ]),
        "hand-derived CW orbit of the valence-5 star"
    );
    // Hand-derived loop cycle too.
    assert_eq!(
        body.loop_cycle(g.he_plus),
        Some(vec![
            g.he_plus,
            g.he_minus,
            t1.he_plus,
            t1.he_minus,
            t2.he_plus,
            t2.he_minus,
            t3.he_plus,
            t3.he_minus,
            t4.he_plus,
            t4.he_minus,
        ]),
        "hand-derived loop after four struts"
    );

    // Emanating attack setup: after the t4 mev, v's emanating is
    // t4.he_plus (the unconditional rule) -- and t4.he_plus is INSIDE
    // the run we are about to move to the new vertex.
    let v = seed.vertex;
    assert_eq!(body.get_vertex(v).unwrap().emanating, Some(t4.he_plus));

    // Split Fan{he1: t2+, he2: g+}: the CW run [t2+ .. g+) wraps:
    // {t2+, t3+, t4+}. The mirrored (CCW) bug would move {t2+, t1+}.
    // Run length 3 vs complement 2 -- fully asymmetric.
    let z = body
        .mev_line(
            MevSite::Fan {
                he1: t2.he_plus,
                he2: g.he_plus,
            },
            p(9.0),
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));

    let start = |body: &Body<f64>, he| body.get_half_edge(he).unwrap().start;
    // Moved to the new vertex:
    assert_eq!(start(&body, t2.he_plus), z.vertex);
    assert_eq!(start(&body, t3.he_plus), z.vertex);
    assert_eq!(start(&body, t4.he_plus), z.vertex);
    // NOT moved (a CCW walk would have moved t1+):
    assert_eq!(start(&body, g.he_plus), v);
    assert_eq!(start(&body, t1.he_plus), v);
    // New edge old -> new.
    assert_eq!(start(&body, z.he_plus), v);
    assert_eq!(body.half_edge_end(z.he_plus), Some(z.vertex));

    // Hand-derived post-split orbits.
    // At v: he_plus spliced before he1=t2+ took the run's angular slot;
    // remaining CW orbit [z+, g+, t1+].
    assert_eq!(
        body.vertex_orbit(z.he_plus),
        Some(vec![z.he_plus, g.he_plus, t1.he_plus])
    );
    // At z: [z-, t2+, t3+, t4+].
    assert_eq!(
        body.vertex_orbit(z.he_minus),
        Some(vec![z.he_minus, t2.he_plus, t3.he_plus, t4.he_plus])
    );

    // THE EMANATING ATTACK: v's old emanating (t4.he_plus) moved to z.
    // The unconditional overwrite must leave both vertices coherent:
    // emanating starts at its own vertex.
    let v_eman = body.get_vertex(v).unwrap().emanating.unwrap();
    assert_eq!(start(&body, v_eman), v, "v's emanating starts at v");
    assert_eq!(
        v_eman, z.he_plus,
        "documented rule: old vertex gets he_plus"
    );
    let z_eman = body.get_vertex(z.vertex).unwrap().emanating.unwrap();
    assert_eq!(start(&body, z_eman), z.vertex);
    assert_eq!(
        z_eman, z.he_minus,
        "documented rule: new vertex gets he_minus"
    );
}

/// Cross-loop fan on a closed solid: he1 and he2 start at one vertex but
/// live in different faces' loops. The moved-run rule must hold across
/// the loop boundary, and each new half must land in its addressing
/// half-edge's loop.
#[test]
fn cross_loop_fan_on_the_digon_pillow_stack() {
    let tol = Tol::witness();
    // Build a closed solid with a valence-3 vertex whose three spokes
    // are in three different loops: the triangle "pillow" (two triangle
    // faces glued -- 1 mvfs + 2 mev + 1 mef gives the two-edge digon;
    // instead build the 3-vertex chain + close = triangle lamina).
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0)).unwrap();
    let ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let bc = body
        .mev_line(
            MevSite::Fan {
                he1: ab.he_minus,
                he2: ab.he_minus,
            },
            Point3::new(0.5, 1.0, 0.0),
            tol,
        )
        .unwrap();
    // Close the triangle: he1 = bc.he_minus (C->B), he2 = ab.he_plus
    // (A->B)... run [C->B, B->A) -- new edge C->A? start(he1)=C,
    // start(he2)=A: new edge C->A. Triangle lamina: two faces.
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

    // Vertex A (seed) has valence 2: spokes A->B (ab.he_plus, in the old
    // loop) and A->C (tri.he_minus... he_minus = start(he2)->start(he1)
    // = A->C, in the NEW loop). Two spokes, two DIFFERENT loops.
    let start = |body: &Body<f64>, he| body.get_half_edge(he).unwrap().start;
    assert_eq!(start(&body, ab.he_plus), seed.vertex);
    assert_eq!(start(&body, tri.he_minus), seed.vertex);
    let loop_of = |body: &Body<f64>, he| body.get_half_edge(he).unwrap().parent_loop;
    let l_plus = loop_of(&body, ab.he_plus);
    let l_minus = loop_of(&body, tri.he_minus);
    assert_ne!(l_plus, l_minus, "the two spokes sit in different loops");

    // Orbit at A, hand-derived: from ab.he_plus, mate = ab.he_minus...
    // ab.he_minus.next: after the mef, he_plus (C->A) was spliced before
    // he2 = ab.he_plus in the OLD loop, and the run [bc.he_minus,
    // ab.he_minus] moved to the new loop with tri.he_minus (A->C)
    // spliced before bc.he_minus. So ab.he_minus.next = tri's... the new
    // loop cycle is [tri.he_minus (A->C), bc.he_minus (C->B),
    // ab.he_minus (B->A)] and the old is [tri.he_plus (C->A),
    // ab.he_plus (A->B), bc.he_plus (B->C)].
    assert_eq!(
        body.loop_cycle(tri.he_minus),
        Some(vec![tri.he_minus, bc.he_minus, ab.he_minus])
    );
    assert_eq!(
        body.loop_cycle(tri.he_plus),
        Some(vec![tri.he_plus, ab.he_plus, bc.he_plus])
    );
    // Orbit at A: next(mate(ab.he_plus)) = next(ab.he_minus) =
    // tri.he_minus; next(mate(tri.he_minus)) = next(tri.he_plus) =
    // ab.he_plus. CW orbit: [ab.he_plus, tri.he_minus].
    assert_eq!(
        body.vertex_orbit(ab.he_plus),
        Some(vec![ab.he_plus, tri.he_minus])
    );

    // The cross-loop fan: Fan{he1: tri.he_minus, he2: ab.he_plus} -- the
    // run is [tri.he_minus] only (one spoke, from the NEW loop), he_plus
    // lands in tri.he_minus's loop, he_minus in ab.he_plus's loop.
    let f = body
        .mev_line(
            MevSite::Fan {
                he1: tri.he_minus,
                he2: ab.he_plus,
            },
            Point3::new(0.0, -1.0, 0.0),
            tol,
        )
        .unwrap();
    assert_eq!(validate(&body), Ok(()));

    // Moved: exactly the run member, across the loop boundary.
    assert_eq!(start(&body, tri.he_minus), f.vertex);
    assert_eq!(start(&body, ab.he_plus), seed.vertex, "he2 not moved");
    // Landing loops: he_plus in he1's loop, he_minus in he2's loop.
    assert_eq!(loop_of(&body, f.he_plus), l_minus, "he_plus in he1's loop");
    assert_eq!(loop_of(&body, f.he_minus), l_plus, "he_minus in he2's loop");
    // Splices: he_plus before he1 in the new loop; he_minus before he2
    // in the old loop.
    assert_eq!(
        body.loop_cycle(f.he_plus),
        Some(vec![f.he_plus, tri.he_minus, bc.he_minus, ab.he_minus])
    );
    assert_eq!(
        body.loop_cycle(tri.he_plus),
        Some(vec![tri.he_plus, f.he_minus, ab.he_plus, bc.he_plus])
    );
    // Orbits: at A [f.he_plus, ab.he_plus]; at the new vertex
    // [f.he_minus, tri.he_minus].
    assert_eq!(
        body.vertex_orbit(f.he_plus),
        Some(vec![f.he_plus, ab.he_plus])
    );
    assert_eq!(
        body.vertex_orbit(f.he_minus),
        Some(vec![f.he_minus, tri.he_minus])
    );
}
