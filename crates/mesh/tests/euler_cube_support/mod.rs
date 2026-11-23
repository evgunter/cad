//! The unit cube built by the Euler operators alone, every face on the
//! `mvfs` seed's pending slot, tier-1 valid after every operator.
#![allow(clippy::unwrap_used, clippy::expect_used, dead_code, unreachable_pub)]
use geom_core::{Point3, Tol};
use topo::construction::Construct;
use topo::construction::{MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};
use topo::{Body, validate};

/// Every operator result of one cube construction, in call order.
#[derive(PartialEq, Eq, Debug)]
pub struct Cube {
    pub seed: MvfsCreated,
    pub mevs: [MevCreated; 7],
    pub mefs: [MefCreated; 5],
}

/// Builds the unit cube, asserting tier-1 validity after EVERY operator
/// (explicitly — not relying on the operators' debug postconditions).
pub fn build_cube_pub(body: &mut Body<f64>) -> Cube {
    build_cube(body)
}
fn build_cube(body: &mut Body<f64>) -> Cube {
    // (a) Seed: lone vertex A in the face that will become the top.
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
    assert_eq!(validate(body), Ok(()));

    // (b) The bottom chain A→B→C→D: one segment, then two struts, each
    // spliced before the previous edge's minus half (the half starting
    // at the chain's current tip).
    let e_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, 0.0),
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
            Point3::new(1.0, 1.0, 0.0),
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
            Point3::new(0.0, 1.0, 0.0),
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
                Point3::new(x, y, 1.0),
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
