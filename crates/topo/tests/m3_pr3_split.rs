//! M3 PR 3 acceptance: the public `split` op end to end — generic /
//! vertex-grazing / face-coplanar planes on asymmetric solids, the
//! Fig. 14.2 notched block (Above disconnected, coplanar artifacts),
//! the PR 2 carry-forwards (tangent tip + BOB mirror through full
//! split; one-sided tangency refused typed), ring re-homing through a
//! genus-1 fixture, slicing (`plane_section`), mirror-check pins
//! (section-face normals; heads-join-heads), D9 byte-identical
//! replay, and the interval lane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism;
use geom_core::{Point3, Vec3};
use topo::{
    Body, PlaneSide, SplitError, SplitFinishError, SplitJoinError, SplitPart, SplitPlane,
    Surface, mass_properties, plane_section, split, validate_closed,
};

/// The split plane y = c, Above = +y.
fn plane_y<T: geom_core::Decide>(c: f64) -> SplitPlane<T> {
    SplitPlane {
        origin: Point3::new(T::from_f64(0.0), T::from_f64(c), T::from_f64(0.0)),
        normal: Vec3::new(T::from_f64(0.0), T::from_f64(1.0), T::from_f64(0.0)),
    }
}

/// Fig. 14.2 analogue (PR 2's fixture, restated): flat notch floor ON
/// the plane + V-notch tip ON the plane; Above = three disconnected
/// prisms.
const NOTCHED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (7.0, 1.0),
    (6.0, 1.0),
    (5.0, 2.0),
    (4.0, 1.0),
    (3.0, 2.0),
    (0.0, 2.0),
];

/// The BOB mirror (touching-wedge) fixture.
const MIRRORED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (3.0, 0.0),
    (4.0, 1.0),
    (5.0, 0.0),
    (6.0, 1.0),
    (7.0, 1.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (0.0, 2.0),
];

fn body_of<T: geom_core::Real>(part: &SplitPart<T>) -> &Body<T> {
    part.body().expect("side has material")
}

fn census<T: geom_core::Real>(b: &Body<T>) -> (usize, usize, usize, usize) {
    (
        b.shells().count(),
        b.faces().count(),
        b.edges().count(),
        b.vertices().count(),
    )
}

/// Vertices of `body` at exactly (x, y, z), bitwise.
fn vertices_at(body: &Body<f64>, x: f64, y: f64, z: f64) -> Vec<topo::VertexKey> {
    body.vertices()
        .filter(|(_, v)| {
            let p = *body.get_point(v.point).unwrap();
            p.x == x && p.y == y && p.z == z
        })
        .map(|(k, _)| k)
        .collect()
}

/// The faces of `body` whose surface is a Plane with bitwise normal
/// `n` and origin `o` — the section-face identification (mirror pin).
fn section_faces_with(body: &Body<f64>, o: Point3<f64>, n: Vec3<f64>) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| match body.get_surface(f.surface) {
            Some(Surface::Plane { origin, normal, .. }) => {
                origin.x == o.x
                    && origin.y == o.y
                    && origin.z == o.z
                    && normal.x == n.x
                    && normal.y == n.y
                    && normal.z == n.z
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect()
}

/// (i) Generic plane on an asymmetric pentagon prism: exact result
/// censuses, tier 2 both sides, volume conservation, the section-face
/// orientation mirror pin, and D9 byte-identical replay.
#[test]
fn generic_plane_asymmetric() {
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (2.0, 3.0), (0.0, 2.0)];
    let fx = prism::<f64>(&profile, 1.0);
    let plane = plane_y(1.0);
    let result = split(&fx.body, &plane).unwrap();
    let (above, below) = (body_of(&result.above), body_of(&result.below));

    // Tier 1 is validated inside every operator (debug asserts); tier 2
    // at rest, both sides — including the per-shell E–P ledger.
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));

    // Census, derived by hand: below = quad prism (V8 E12 F6); above =
    // pentagon prism (V10 E15 F7). One shell each.
    assert_eq!(census(below), (1, 6, 12, 8));
    assert_eq!(census(above), (1, 7, 15, 10));

    // Volume conservation (evaluation-lane check).
    let (va, vb, v0) = (
        mass_properties(above).unwrap().volume,
        mass_properties(below).unwrap().volume,
        mass_properties(&fx.body).unwrap().volume,
    );
    assert!((va + vb - v0).abs() <= 1e-12 * v0);

    // Mirror pin — section-face orientation, derived from
    // enters_material: above section face normal is bitwise −n_SP,
    // below is +n_SP, both at the split-plane origin.
    let o = Point3::new(0.0, 1.0, 0.0);
    assert_eq!(
        section_faces_with(above, o, Vec3::new(0.0, -1.0, 0.0)).len(),
        1
    );
    assert_eq!(
        section_faces_with(below, o, Vec3::new(0.0, 1.0, 0.0)).len(),
        1
    );
    // ... and the wrong signs identify nothing.
    assert!(section_faces_with(above, o, Vec3::new(0.0, 1.0, 0.0)).is_empty());
    assert!(section_faces_with(below, o, Vec3::new(0.0, -1.0, 0.0)).is_empty());

    // Mirror pin — heads join heads / tails join tails (the lmef/lmekr
    // arg-pair outcome): every vertex of the above body lies at
    // y ≥ 1, every vertex of the below body at y ≤ 1 (a mixed-side
    // connecting edge would violate one of these).
    for (b, above_side) in [(above, true), (below, false)] {
        for (_, v) in b.vertices() {
            let p = *b.get_point(v.point).unwrap();
            assert!(if above_side { p.y >= 1.0 } else { p.y <= 1.0 });
        }
    }

    // D9: byte-identical replay (Debug dump of the full arenas).
    let again = split(&fx.body, &plane).unwrap();
    assert_eq!(
        format!("{above:?}"),
        format!("{:?}", body_of(&again.above))
    );
    assert_eq!(
        format!("{below:?}"),
        format!("{:?}", body_of(&again.below))
    );
}

/// (ii) Vertex-grazing plane: two profile vertices exactly ON, no
/// proper edge crossings — the section runs through existing vertices.
#[test]
fn vertex_grazing_plane() {
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 3.0), (0.0, 2.0)];
    let fx = prism::<f64>(&profile, 1.0);
    let result = split(&fx.body, &plane_y(2.0)).unwrap();
    let (above, below) = (body_of(&result.above), body_of(&result.below));
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));
    // Below = quad prism; above = triangle prism.
    assert_eq!(census(below), (1, 6, 12, 8));
    assert_eq!(census(above), (1, 5, 9, 6));
    // The grazed corners: one vertex per side at each ON position
    // (coincident across bodies — distinct entities in distinct
    // bodies).
    for z in [0.0, 1.0] {
        for x in [0.0, 4.0] {
            assert_eq!(vertices_at(above, x, 2.0, z).len(), 1);
            assert_eq!(vertices_at(below, x, 2.0, z).len(), 1);
        }
    }
    let (va, vb, v0) = (
        mass_properties(above).unwrap().volume,
        mass_properties(below).unwrap().volume,
        mass_properties(&fx.body).unwrap().volume,
    );
    assert!((va + vb - v0).abs() <= 1e-12 * v0);
}

/// (iii) + the Fig. 14.2 story: the notched block at the face-coplanar
/// plane — Above lands as THREE disconnected prisms; the coplanar
/// notch floor survives as a Below wall (artifact face, F7: not
/// auto-merged); the tangent tip disconnects via distinct vertex
/// copies (PR 2 carry-forward 1).
#[test]
fn notched_block_end_to_end() {
    let fx = prism::<f64>(NOTCHED, 1.0);
    let result = split(&fx.body, &plane_y(1.0)).unwrap();
    let (above, below) = (body_of(&result.above), body_of(&result.below));
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));

    // Above: three disconnected prisms, one shell each, one solid.
    assert_eq!(above.shells().count(), 3);
    assert_eq!(above.solids().count(), 1);
    assert_eq!(below.shells().count(), 1);

    // Tangent-tip carry-forward: the Above side holds TWO distinct
    // coincident vertices at each tip position (one per flanking
    // prism), in different shells; Below keeps ONE, with the tip edge
    // surviving as an artifact edge inside its coplanar top.
    for z in [0.0, 1.0] {
        let above_tips = vertices_at(above, 4.0, 1.0, z);
        assert_eq!(above_tips.len(), 2, "two coincident distinct copies");
        let shell_of = |b: &Body<f64>, v: topo::VertexKey| {
            let he = b.get_vertex(v).unwrap().emanating.unwrap();
            let l = b.get_half_edge(he).unwrap().parent_loop;
            let f = b.get_loop(l).unwrap().face;
            b.get_face(f).unwrap().shell
        };
        assert_ne!(
            shell_of(above, above_tips[0]),
            shell_of(above, above_tips[1]),
            "the copies live in different components"
        );
        assert_eq!(vertices_at(below, 4.0, 1.0, z).len(), 1);
    }
    let (tb, tt) = (
        vertices_at(below, 4.0, 1.0, 0.0)[0],
        vertices_at(below, 4.0, 1.0, 1.0)[0],
    );
    let tip_edges = below
        .edges()
        .filter(|(_, e)| {
            let s1 = below.get_half_edge(e.he_plus).unwrap().start;
            let s2 = below.get_half_edge(e.he_minus).unwrap().start;
            (s1 == tb && s2 == tt) || (s1 == tt && s2 == tb)
        })
        .count();
    assert_eq!(tip_edges, 1, "tip edge survives in Below's coplanar top");

    // Coplanar artifacts: Below's top at y = 1 carries the operand's
    // notch-floor face (outward +y) alongside the three +y section
    // faces — four y = 1 faces in total, none merged (F7).
    let up_faces = below
        .faces()
        .filter(|(_, f)| match below.get_surface(f.surface) {
            Some(Surface::Plane { origin, normal, .. }) => {
                normal.x == 0.0 && normal.y == 1.0 && normal.z == 0.0 && origin.y == 1.0
            }
            _ => false,
        })
        .count();
    assert_eq!(up_faces, 4, "3 section faces + the artifact floor");

    // Volume conservation.
    let (va, vb, v0) = (
        mass_properties(above).unwrap().volume,
        mass_properties(below).unwrap().volume,
        mass_properties(&fx.body).unwrap().volume,
    );
    assert!((va + vb - v0).abs() <= 1e-12 * v0);
}

/// One-sided pure tangency (PR 2 carry-forward 2): the apex prism
/// touching the plane from above along its apex edge only — the
/// degenerate side is REFUSED typed (zero-area section polygon), no
/// degenerate body is ever emitted; `plane_section` refuses the same
/// way.
#[test]
fn one_sided_tangency_refused_typed() {
    let profile = [(3.0, 4.0), (6.0, 1.0), (9.0, 4.0)]; // apex down, ON y=1
    let fx = prism::<f64>(&profile, 1.0);
    let err = split(&fx.body, &plane_y(1.0)).unwrap_err();
    assert!(
        matches!(
            err,
            SplitError::Join(SplitJoinError::DegenerateSection { .. })
                | SplitError::Finish(SplitFinishError::DegenerateSide { .. })
        ),
        "got {err:?}"
    );
    let err = plane_section(&fx.body, &plane_y(1.0)).unwrap_err();
    assert!(matches!(
        err,
        SplitError::Join(SplitJoinError::DegenerateSection { .. })
    ));
}

/// ∅ sides are typed variants: a plane missing the body entirely, and
/// a plane touching a face without cutting.
#[test]
fn empty_sides_are_typed() {
    let fx = prism::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)], 1.0);
    // Plane far above: everything Below.
    let r = split(&fx.body, &plane_y(5.0)).unwrap();
    assert!(matches!(r.above, SplitPart::Empty));
    let below = body_of(&r.below);
    assert_eq!(validate_closed(below), Ok(()));
    assert_eq!(census(below), census(&fx.body));
    // Plane coplanar with the top face: ON contact, no cut — still a
    // typed Empty above.
    let r = split(&fx.body, &plane_y(1.0)).unwrap();
    assert!(matches!(r.above, SplitPart::Empty));
    assert_eq!(validate_closed(body_of(&r.below)), Ok(()));
}
