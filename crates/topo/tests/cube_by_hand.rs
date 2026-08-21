//! The M1 acceptance test: build a unit cube by hand through the public
//! Euler-operator API — 1 `mvfs` + 7 `mev` + 5 `mef` (the provably
//! minimal sequence for v8 e12 f6, Mäntylä §9.4.2 eq. 9.5) — validating
//! after every operator, then checking counts, provenance, and lineage
//! determinism.
//!
//! # The construction, geometrically
//!
//! Unit cube; bottom corners `A B C D` at z = 0, top corners `A' B' C' D'`
//! above them at z = 1, with `A B C D` counterclockwise viewed from
//! **above** (+z):
//!
//! ```text
//!        D'------C'
//!       /|      /|          z
//!      A'------B'|          | y
//!      | D-----|-C          |/
//!      |/      |/           +---x
//!      A-------B
//! ```
//!
//! Under the ratified interior-left/CCW-from-outside convention the
//! face loops are: bottom `A→D→C→B` (CCW seen from below), top
//! `A'→B'→C'→D'` (CCW from above), front `A→B→B'→A'`, right
//! `B→C→C'→B'`, back `C→D→D'→C'`, left `D→A→A'→D'`.
//!
//! Sequence (Mäntylä §9.3 steps (a)–(e), mirrored to our convention):
//! seed at `A`; grow the chain `A→B→C→D` (1 segment `mev` + 2 strut
//! `mev`s); close the bottom face with `mef` (the lamina step — two
//! faces sharing all four edges, a legal tier-1 intermediate); raise the
//! four verticals as struts in the top-side face; cut the four side
//! faces out of it with four `mef`s, leaving the top square.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;
use geom_core::Tol;
use topo::{
    Body, EntityId, MefCreated, MefSite, MevCreated, MevSite, MvfsCreated, Provenance, validate,
    validate_closed,
};

/// Every operator result of one cube construction, in call order.
#[derive(PartialEq, Eq, Debug)]
struct Cube {
    seed: MvfsCreated,
    mevs: [MevCreated; 7],
    mefs: [MefCreated; 5],
}

/// Builds the unit cube, asserting tier-1 validity after EVERY operator
/// (explicitly — not relying on the operators' debug postconditions).
fn build_cube(body: &mut Body<f64>) -> Cube {
    let pt = |x: f64, y: f64, z: f64| Point3::new(x, y, z);

    // (a) Seed: lone vertex A in the face that will become the top.
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    assert_eq!(validate(body), Ok(()));

    // (b) The bottom chain A→B→C→D: one segment, then two struts, each
    // spliced before the previous edge's minus half (the half starting
    // at the chain's current tip).
    let e_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(1.0, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(validate(body), Ok(()));
    let e_bc = body
        .mev_line(
            MevSite::Fan {
                he1: e_ab.he_minus,
                he2: e_ab.he_minus,
            },
            pt(1.0, 1.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(validate(body), Ok(()));
    let e_cd = body
        .mev_line(
            MevSite::Fan {
                he1: e_bc.he_minus,
                he2: e_bc.he_minus,
            },
            pt(0.0, 1.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(validate(body), Ok(()));

    // (c) Close the square: the lamina step. Address the chords the
    // high-level GWB way, through find_half_edge. he1 = the half D→C
    // (start of the run that becomes the bottom face), he2 = the half
    // A→B (exclusive end); the new edge runs D→A, and the new (bottom)
    // face's outer loop walks A→D→C→B — CCW seen from below, per the
    // convention table above.
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    let he_ab = body
        .find_half_edge(seed.face, seed.vertex, e_ab.vertex)
        .unwrap();
    assert_eq!(
        he_ab, e_ab.he_plus,
        "find_half_edge agrees with created keys"
    );
    let f_bottom = body
        .mef_chord(
            MefSite::Chords {
                he1: he_dc,
                he2: he_ab,
            },
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(validate(body), Ok(()));

    // (d) Raise the verticals: four struts in the remaining (top-side)
    // loop, each at a bottom corner, spliced before that corner's
    // outgoing bottom-square half.
    let strut = |body: &mut Body<f64>, at: topo::HalfEdgeKey, x: f64, y: f64| {
        let created = body
            .mev_line(
                MevSite::Fan { he1: at, he2: at },
                pt(x, y, 1.0),
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(validate(body), Ok(()));
        created
    };
    let e_aa = strut(body, e_ab.he_plus, 0.0, 0.0); // A→A' before A→B
    let e_bb = strut(body, e_bc.he_plus, 1.0, 0.0); // B→B' before B→C
    let e_cc = strut(body, e_cd.he_plus, 1.0, 1.0); // C→C' before C→D
    let e_dd = strut(body, f_bottom.he_plus, 0.0, 1.0); // D→D' before D→A

    // (e) Cut the four side faces out of the top-side loop. Each mef's
    // he1 is a strut's minus half (T'→T, the downward traversal), and
    // the run [he1 .. he2) — down the strut, along one bottom edge, up
    // the next strut — becomes the side face's outer loop.
    let side = |body: &mut Body<f64>, he1, he2| {
        let created = body
            .mef_chord(MefSite::Chords { he1, he2 }, Tol::witness())
            .unwrap();
        assert_eq!(validate(body), Ok(()));
        created
    };
    // Front A→B→B'→A': run [A'→A, A→B, B→B'), new edge A'→B'.
    let f_front = side(body, e_aa.he_minus, e_bb.he_minus);
    // Right B→C→C'→B': run [B'→B, B→C, C→C'), new edge B'→C'.
    let f_right = side(body, e_bb.he_minus, e_cc.he_minus);
    // Back C→D→D'→C': run [C'→C, C→D, D→D'), new edge C'→D'.
    let f_back = side(body, e_cc.he_minus, e_dd.he_minus);
    // Left D→A→A'→D': run [D'→D, D→A, A→A'), new edge D'→A'. Its he2 is
    // the front face's plus half A'→B' — found through the face, the
    // way a hand construction would address it.
    let he_a1b1 = body
        .find_half_edge(seed.face, e_aa.vertex, e_bb.vertex)
        .unwrap();
    assert_eq!(
        he_a1b1, f_front.he_plus,
        "find_half_edge agrees with created keys"
    );
    let f_left = side(body, e_dd.he_minus, he_a1b1);

    Cube {
        seed,
        mevs: [e_ab, e_bc, e_cd, e_aa, e_bb, e_cc, e_dd],
        mefs: [f_bottom, f_front, f_right, f_back, f_left],
    }
}

#[test]
fn cube_by_hand_validates_with_minimal_counts() {
    let mut body = Body::<f64>::new();
    let cube = build_cube(&mut body);

    // The provably minimal counts (Mäntylä eq. 9.5: v−1 = 7 mev,
    // f+h−1 = 5 mef, 1 mvfs).
    assert_eq!(body.vertices().count(), 8);
    assert_eq!(body.edges().count(), 12);
    assert_eq!(body.faces().count(), 6);
    assert_eq!(body.loops().count(), 6);
    assert_eq!(body.half_edges().count(), 24);
    assert_eq!(body.shells().count(), 1);
    assert_eq!(body.solids().count(), 1);
    // Geometry policy: one point per vertex, one curve per edge, ONE
    // surface — every mef shares the split face's surface, and all six
    // faces descend from mvfs's face.
    assert_eq!(body.points().count(), 8);
    assert_eq!(body.curves().count(), 12);
    assert_eq!(body.surfaces().count(), 1);
    // Euler–Poincaré: v − e + f = 8 − 12 + 6 = 2 = 2(s − h) + r with
    // s = 1, h = r = 0. (The validator's per-shell component pass now
    // checks this internally; the count check here is the acceptance
    // criterion's explicit form.)
    assert_eq!(8 - 12 + 6, 2);
    assert_eq!(validate(&body), Ok(()));
    // The finished cube is a tier-2 closed solid: no scaffolding, one
    // connected shell component.
    assert_eq!(validate_closed(&body), Ok(()));

    // Every face's outer loop is a quad, and no face has rings.
    for (_, face) in body.faces() {
        assert!(face.rings.is_empty());
        let topo::LoopBoundary::Cycle { first } = body.get_loop(face.outer).unwrap().boundary
        else {
            panic!("finished cube has no empty loops");
        };
        assert_eq!(body.loop_cycle(first).unwrap().len(), 4);
    }
    // Every vertex has valence 3.
    for (_, vertex) in body.vertices() {
        let orbit = body.vertex_orbit(vertex.emanating.unwrap()).unwrap();
        assert_eq!(orbit.len(), 3);
    }

    // The final top face: the seed face's loop walks A'→B'→C'→D'
    // (counterclockwise viewed from above = from outside, per the
    // ratified convention).
    let [e_ab, _, _, e_aa, e_bb, e_cc, e_dd] = cube.mevs;
    let top_cycle = body.loop_cycle(cube.mefs[1].he_plus).unwrap();
    let top_starts: Vec<_> = top_cycle
        .iter()
        .map(|&he| body.get_half_edge(he).unwrap().start)
        .collect();
    assert_eq!(
        top_starts,
        vec![e_aa.vertex, e_bb.vertex, e_cc.vertex, e_dd.vertex],
        "top loop walks A' → B' → C' → D'"
    );
    // The bottom face's loop walks D→C→B→A→(D) from its he_plus... its
    // cycle from the D→C half: starts D, C, B, A.
    let bottom_cycle = body.loop_cycle(cube.mefs[0].he_minus).unwrap();
    let bottom_starts: Vec<_> = bottom_cycle
        .iter()
        .map(|&he| body.get_half_edge(he).unwrap().start)
        .collect();
    assert_eq!(
        bottom_starts,
        vec![
            cube.seed.vertex,    // A
            cube.mevs[2].vertex, // D
            cube.mevs[1].vertex, // C
            e_ab.vertex,         // B
        ],
        "bottom loop walks A → D → C → B (CCW seen from below)"
    );
}

#[test]
fn cube_provenance_records_every_birth() {
    let mut body = Body::<f64>::new();
    let cube = build_cube(&mut body);

    // mvfs's five entities.
    for id in [
        EntityId::Solid(cube.seed.solid),
        EntityId::Shell(cube.seed.shell),
        EntityId::Face(cube.seed.face),
        EntityId::Loop(cube.seed.r#loop),
        EntityId::Vertex(cube.seed.vertex),
    ] {
        assert_eq!(body.provenance(id), Some(&Provenance::Mvfs), "for {id}");
    }
    // Each mev's four entities carry its exact site. Reconstruct the
    // sites in call order.
    let sites: [MevSite; 7] = [
        MevSite::Lone {
            r#loop: cube.seed.r#loop,
        },
        MevSite::Fan {
            he1: cube.mevs[0].he_minus,
            he2: cube.mevs[0].he_minus,
        },
        MevSite::Fan {
            he1: cube.mevs[1].he_minus,
            he2: cube.mevs[1].he_minus,
        },
        MevSite::Fan {
            he1: cube.mevs[0].he_plus,
            he2: cube.mevs[0].he_plus,
        },
        MevSite::Fan {
            he1: cube.mevs[1].he_plus,
            he2: cube.mevs[1].he_plus,
        },
        MevSite::Fan {
            he1: cube.mevs[2].he_plus,
            he2: cube.mevs[2].he_plus,
        },
        MevSite::Fan {
            he1: cube.mefs[0].he_plus,
            he2: cube.mefs[0].he_plus,
        },
    ];
    for (created, site) in cube.mevs.iter().zip(sites) {
        let expected = Provenance::Mev { site };
        for id in [
            EntityId::Vertex(created.vertex),
            EntityId::Edge(created.edge),
            EntityId::HalfEdge(created.he_plus),
            EntityId::HalfEdge(created.he_minus),
        ] {
            assert_eq!(body.provenance(id), Some(&expected), "for {id}");
        }
    }
    // Each mef's five entities carry a Mef record (site payloads checked
    // exactly in the unit tests; here the sweep asserts kind + presence
    // for every remaining entity).
    for created in &cube.mefs {
        for id in [
            EntityId::Face(created.face),
            EntityId::Loop(created.r#loop),
            EntityId::Edge(created.edge),
            EntityId::HalfEdge(created.he_plus),
            EntityId::HalfEdge(created.he_minus),
        ] {
            assert!(
                matches!(body.provenance(id), Some(Provenance::Mef { .. })),
                "for {id}"
            );
        }
    }
    // Nothing primordial anywhere: the cube was built purely by
    // operators.
    for (key, _) in body.half_edges() {
        assert!(matches!(
            body.provenance(EntityId::HalfEdge(key)),
            Some(Provenance::Mev { .. } | Provenance::Mef { .. })
        ));
    }
}

#[test]
fn cube_lineage_is_deterministic() {
    // D9 + lineage replay: the same construction in two fresh bodies
    // mints identical key sequences — every created-keys struct compares
    // equal key for key.
    let mut body_a = Body::<f64>::new();
    let mut body_b = Body::<f64>::new();
    let cube_a = build_cube(&mut body_a);
    let cube_b = build_cube(&mut body_b);
    assert_eq!(cube_a, cube_b);
}
