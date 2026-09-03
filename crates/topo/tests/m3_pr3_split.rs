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
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use topo::{
    Body, SplitError, SplitFinishError, SplitJoinError, SplitPart, SplitPlane, Surface,
    mass_properties, plane_section, split, validate_closed, validate_geometric,
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

/// Tier 3 directly at rest (D6, M3 PR 6a): split results carry honest
/// `Intersection` descriptions natively — no upgrade pass exists.
fn assert_tier3_after_upgrade(body: &Body<f64>) {
    assert_eq!(validate_geometric(body, Tol::witness()), Ok(()));
}

fn body_of<T: geom_core::Real>(part: &SplitPart<T>) -> &Body<T> {
    part.body().expect("side has material")
}

/// The four arena lengths a split side is pinned by.
///
/// Deliberately a projection, not `topo::test_support::ArenaCounts`:
/// the six expectations below are hand-derived for exactly these four,
/// so pinning the other three arenas would be three new claims per
/// site rather than the same claim spelled once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SideCensus {
    shells: usize,
    faces: usize,
    edges: usize,
    vertices: usize,
}

fn census<T: geom_core::Real>(b: &Body<T>) -> SideCensus {
    SideCensus {
        shells: b.shells().count(),
        faces: b.faces().count(),
        edges: b.edges().count(),
        vertices: b.vertices().count(),
    }
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

/// A geometric genus-1 box: outer 4×2×2, square 1×1 hole through z at
/// (0.5..1.5)² — the box_with_hole construction (§9.3) followed by a
/// plating pass that gives every face its own Newell plane from its
/// outer loop (loops are CCW-from-outside by construction, so Newell
/// yields outward normals).
fn holed_box_geometric() -> Body<f64> {
    use topo::{MefSite, MevSite};
    let pt = Point3::new;
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    let strut = |body: &mut Body<f64>, at, x, y, z| {
        body.mev_line(
            MevSite::Fan { he1: at, he2: at },
            pt(x, y, z),
            Tol::witness(),
        )
        .unwrap()
    };
    let mef = |body: &mut Body<f64>, he1, he2| {
        body.mef_chord(MefSite::Chords { he1, he2 }, Tol::witness())
            .unwrap()
    };
    // Bottom chain A→B→C→D, closed; verticals; sides (§9.4.2).
    let e_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(4.0, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    let e_bc = strut(&mut body, e_ab.he_minus, 4.0, 2.0, 0.0);
    let e_cd = strut(&mut body, e_bc.he_minus, 0.0, 2.0, 0.0);
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    let f_bottom = mef(&mut body, he_dc, e_ab.he_plus);
    let e_aa = strut(&mut body, e_ab.he_plus, 0.0, 0.0, 2.0);
    let e_bb = strut(&mut body, e_bc.he_plus, 4.0, 0.0, 2.0);
    let e_cc = strut(&mut body, e_cd.he_plus, 4.0, 2.0, 2.0);
    let e_dd = strut(&mut body, f_bottom.he_plus, 0.0, 2.0, 2.0);
    let f_front = mef(&mut body, e_aa.he_minus, e_bb.he_minus);
    let _f_right = mef(&mut body, e_bb.he_minus, e_cc.he_minus);
    let _f_back = mef(&mut body, e_cc.he_minus, e_dd.he_minus);
    let _f_left = mef(&mut body, e_dd.he_minus, f_front.he_plus);
    // Hole: strut A'→P, kemr to plant the ring, grow P→Q→R→S, close,
    // drop verticals, cut the tube walls, kfmrh the membrane.
    let hole = strut(&mut body, f_front.he_plus, 0.5, 0.5, 2.0);
    let kill = body.kemr(hole.he_plus, hole.he_minus).unwrap();
    let s_pq = body
        .mev_line(
            MevSite::Lone { r#loop: kill.ring },
            pt(1.5, 0.5, 2.0),
            Tol::witness(),
        )
        .unwrap();
    let s_qr = strut(&mut body, s_pq.he_minus, 1.5, 1.5, 2.0);
    let s_rs = strut(&mut body, s_qr.he_minus, 0.5, 1.5, 2.0);
    let mef_top = mef(&mut body, s_pq.he_plus, s_rs.he_minus);
    let e_pp = strut(&mut body, s_pq.he_plus, 0.5, 0.5, 0.0);
    let e_qq = strut(&mut body, s_qr.he_plus, 1.5, 0.5, 0.0);
    let e_rr = strut(&mut body, s_rs.he_plus, 1.5, 1.5, 0.0);
    let e_ss = strut(&mut body, mef_top.he_minus, 0.5, 1.5, 0.0);
    let w_front = mef(&mut body, e_pp.he_minus, e_qq.he_minus);
    let _w_right = mef(&mut body, e_qq.he_minus, e_rr.he_minus);
    let _w_back = mef(&mut body, e_rr.he_minus, e_ss.he_minus);
    let _w_left = mef(&mut body, e_ss.he_minus, w_front.he_plus);
    body.kfmrh(f_bottom.face, mef_top.face).unwrap();
    // Plating pass: every face gets its own outward Newell plane.
    let faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    for f in faces {
        let outer = body.get_face(f).unwrap().outer;
        let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("outer loops are cycles");
        };
        let pts: Vec<Point3<f64>> = body
            .loop_cycle(first)
            .unwrap()
            .iter()
            .map(|&he| {
                let v = body.get_half_edge(he).unwrap().start;
                *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
            })
            .collect();
        let plane = geom_brep::newell_plane(&pts, band).unwrap();
        body.set_face_surface(f, topo::FaceSurface::New(plane))
            .unwrap();
    }
    // Construction-final description step (D6): the fixture is a split
    // operand — tier-3-grade by construction.
    common::describe_as_intersections(&mut body);
    body
}

/// (i) Generic plane on an asymmetric pentagon prism: exact result
/// censuses, tier 2 both sides, volume conservation, the section-face
/// orientation mirror pin, and D9 byte-identical replay.
#[test]
fn generic_plane_asymmetric() {
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (2.0, 3.0), (0.0, 2.0)];
    let fx = prism::<f64>(&profile, 1.0);
    let plane = plane_y(1.0);
    let result = split(&fx.body, &plane, Tol::witness()).unwrap();
    let (above, below) = (body_of(&result.above), body_of(&result.below));

    // Tier 1 is validated inside every operator (debug asserts); tier 2
    // at rest, both sides — including the per-shell E–P ledger; tier 3
    // where the geometry qualifies (manifold, coincidence-free within
    // each body), after the prefer-intrinsic description upgrade (the
    // M2 posture: tier 3's TransverseNotIntrinsic check wants
    // Intersection descriptions, which the split's minted chord edges
    // do not carry — documented in the PR writeup).
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));
    assert_tier3_after_upgrade(above);
    assert_tier3_after_upgrade(below);

    // Census, derived by hand: below = quad prism (V8 E12 F6); above =
    // pentagon prism (V10 E15 F7). One shell each.
    assert_eq!(
        census(below),
        SideCensus {
            shells: 1,
            faces: 6,
            edges: 12,
            vertices: 8
        }
    );
    assert_eq!(
        census(above),
        SideCensus {
            shells: 1,
            faces: 7,
            edges: 15,
            vertices: 10
        }
    );

    // Volume conservation (evaluation-lane check).
    let (va, vb, v0) = (
        mass_properties(above, Tol::witness()).unwrap().volume,
        mass_properties(below, Tol::witness()).unwrap().volume,
        mass_properties(&fx.body, Tol::witness()).unwrap().volume,
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
    let again = split(&fx.body, &plane, Tol::witness()).unwrap();
    assert_eq!(format!("{above:?}"), format!("{:?}", body_of(&again.above)));
    assert_eq!(format!("{below:?}"), format!("{:?}", body_of(&again.below)));
}

/// (ii) Vertex-grazing plane: two profile vertices exactly ON, no
/// proper edge crossings — the section runs through existing vertices.
#[test]
fn vertex_grazing_plane() {
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 3.0), (0.0, 2.0)];
    let fx = prism::<f64>(&profile, 1.0);
    let result = split(&fx.body, &plane_y(2.0), Tol::witness()).unwrap();
    let (above, below) = (body_of(&result.above), body_of(&result.below));
    // Tier 3 qualifies: each body alone is manifold and
    // coincidence-free (the grazed corners coincide only ACROSS the
    // two bodies).
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));
    assert_tier3_after_upgrade(above);
    assert_tier3_after_upgrade(below);
    // Below = quad prism; above = triangle prism.
    assert_eq!(
        census(below),
        SideCensus {
            shells: 1,
            faces: 6,
            edges: 12,
            vertices: 8
        }
    );
    assert_eq!(
        census(above),
        SideCensus {
            shells: 1,
            faces: 5,
            edges: 9,
            vertices: 6
        }
    );
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
        mass_properties(above, Tol::witness()).unwrap().volume,
        mass_properties(below, Tol::witness()).unwrap().volume,
        mass_properties(&fx.body, Tol::witness()).unwrap().volume,
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
    let result = split(&fx.body, &plane_y(1.0), Tol::witness()).unwrap();
    let (above, below) = (body_of(&result.above), body_of(&result.below));
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));
    // Tier 3 on the verb's own products — the coplanar row's boundary
    // edges sit between flush y = 1 planes, where a citation the split
    // reassigned must have been restated, not kept (issue 1152).
    assert_tier3_after_upgrade(above);
    assert_tier3_after_upgrade(below);

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
        mass_properties(above, Tol::witness()).unwrap().volume,
        mass_properties(below, Tol::witness()).unwrap().volume,
        mass_properties(&fx.body, Tol::witness()).unwrap().volume,
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
    let err = split(&fx.body, &plane_y(1.0), Tol::witness()).unwrap_err();
    assert!(
        matches!(
            err,
            SplitError::Join(SplitJoinError::DegenerateSection { .. })
                | SplitError::Finish(SplitFinishError::DegenerateSide { .. })
        ),
        "got {err:?}"
    );
    let err = plane_section(&fx.body, &plane_y(1.0), Tol::witness()).unwrap_err();
    assert!(matches!(
        err,
        SplitError::Join(SplitJoinError::DegenerateSection { .. })
    ));
}

/// The BOB mirror (PR 2 carry-forward 1b), CLOSED by M3 PR 6a's D7:
/// full split of the touching-wedge fixture now SUCCEEDS in BOTH
/// orientations — the below-side pinch (whose below fans share the
/// one original vertex under ch. 14's above-only copy minting) gets
/// its below-vertex copies through the pinch lane (`split`'s mirror
/// identity; see the splitting module docs), and the former
/// orientation-dependent `DegenerateSection` refusal is gone for the
/// pinch class. This test pins the two presentations that used to
/// refuse — (MIRRORED, +n) and (NOTCHED, −n) — with the equal-volume
/// oracle and the piece-assignment check (pinched pieces land on the
/// correct SIDE for this call's normal, wherever the copies were
/// minted).
#[test]
fn bob_mirror_pinch_refuses_typed() {
    // MIRRORED under +n: pinched floor pieces are BELOW.
    let fx = prism::<f64>(MIRRORED, 1.0);
    let r = split(&fx.body, &plane_y(1.0), Tol::witness()).unwrap();
    let (slab, pieces) = (body_of(&r.above), body_of(&r.below));
    assert_eq!(validate_closed(slab), Ok(()));
    assert_eq!(validate_closed(pieces), Ok(()));
    assert_eq!(pieces.shells().count(), 3, "three pinched floor pieces");
    assert_eq!(slab.shells().count(), 1);
    // Distinct coincident tip copies on the pieces side; one vertex on
    // the slab side (same census as the flipped-plane presentation).
    for z in [0.0, 1.0] {
        assert_eq!(vertices_at(pieces, 4.0, 1.0, z).len(), 2);
        assert_eq!(vertices_at(slab, 4.0, 1.0, z).len(), 1);
    }
    let v0 = mass_properties(&fx.body, Tol::witness()).unwrap().volume;
    let (vs, vp) = (
        mass_properties(slab, Tol::witness()).unwrap().volume,
        mass_properties(pieces, Tol::witness()).unwrap().volume,
    );
    assert!((vs + vp - v0).abs() <= 1e-12 * v0, "{vs} + {vp} vs {v0}");

    // NOTCHED under −n: pinched prisms are BELOW the flipped normal.
    let fx = prism::<f64>(NOTCHED, 1.0);
    let flipped = SplitPlane {
        origin: Point3::new(0.0, 1.0, 0.0),
        normal: Vec3::new(0.0, -1.0, 0.0),
    };
    let r = split(&fx.body, &flipped, Tol::witness()).unwrap();
    // Below the flipped normal = the y > 1 pinched prisms.
    let (pieces, slab) = (body_of(&r.below), body_of(&r.above));
    assert_eq!(validate_closed(pieces), Ok(()));
    assert_eq!(validate_closed(slab), Ok(()));
    assert_eq!(pieces.shells().count(), 3);
    assert_eq!(slab.shells().count(), 1);
    let v0 = mass_properties(&fx.body, Tol::witness()).unwrap().volume;
    let (vs, vp) = (
        mass_properties(slab, Tol::witness()).unwrap().volume,
        mass_properties(pieces, Tol::witness()).unwrap().volume,
    );
    assert!((vs + vp - v0).abs() <= 1e-12 * v0, "{vs} + {vp} vs {v0}");
}

/// Slicing (§14.9): `plane_section` returns the section polygons
/// without building bodies — the notched block yields THREE polygons
/// with in-plane (u, v) coordinates; a missing plane yields the empty
/// typed success; the operand is untouched.
#[test]
fn plane_section_slicing() {
    let fx = prism::<f64>(NOTCHED, 1.0);
    let before = format!("{:?}", fx.body);
    let section = plane_section(&fx.body, &plane_y(1.0), Tol::witness()).unwrap();
    assert_eq!(format!("{:?}", fx.body), before, "operand untouched");
    assert_eq!(section.polygons.len(), 3);
    let (u, v) = (section.u_ref.unwrap(), section.v_ref.unwrap());
    // The frame is in-plane and orthonormal (exact for these axes).
    assert_eq!(u.dot(section.plane.normal), 0.0);
    assert_eq!(v.dot(section.plane.normal), 0.0);
    for poly in &section.polygons {
        assert_eq!(poly.points.len(), poly.uv.len());
        assert!(poly.points.len() >= 4);
        for (p, q) in poly.points.iter().zip(&poly.uv) {
            // Every corner lies ON the plane, and uv reproduces it.
            assert_eq!(p.y, 1.0);
            let back = section.plane.origin + u * q.x + v * q.y;
            assert_eq!((back.x, back.y, back.z), (p.x, p.y, p.z));
        }
    }
    // Total section area = the y = 1 material cross-section: the
    // notched block's slice is x ∈ [0,4] ∪ [4,6] ∪ [7,8], z ∈ [0,1].
    let mut total = 0.0;
    for poly in &section.polygons {
        let mut twice = 0.0;
        for i in 0..poly.uv.len() {
            let a = poly.uv[i];
            let b = poly.uv[(i + 1) % poly.uv.len()];
            twice += a.x * b.y - b.x * a.y;
        }
        total += (twice / 2.0).abs();
    }
    assert!((total - 7.0).abs() < 1e-12);

    // A plane that misses the body: zero polygons, typed success.
    let empty = plane_section(&fx.body, &plane_y(9.0), Tol::witness()).unwrap();
    assert!(empty.polygons.is_empty());
    assert!(empty.u_ref.is_none());
}

/// Ring re-homing through the join (`laringmv`, the lkemr
/// ring-placement mirror site): a genus-1 box (through-hole along z)
/// split beside the hole — the top and bottom faces carry rings and
/// are divided by the join, and each ring must land in the piece that
/// geometrically contains it (decided by the trilean point-in-loop).
#[test]
fn ring_rehoming_genus_one() {
    let body = holed_box_geometric();
    assert_eq!(validate_closed(&body), Ok(()));
    // Split at x = 3: the hole (x ∈ [0.5, 1.5]) is entirely below.
    let plane = SplitPlane {
        origin: Point3::new(3.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    };
    let result = split(&body, &plane, Tol::witness()).unwrap();
    let (above, below) = (body_of(&result.above), body_of(&result.below));
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));
    assert_tier3_after_upgrade(above);
    assert_tier3_after_upgrade(below);
    // The hole went below: genus bookkeeping via census. Below: the
    // holed slab [0,3] — V16 E24 F10 (8 outer + hole rim ×2 … as the
    // holed box, x-cut): outer box 8 + 8 hole verts = 16; above: plain
    // slab V8 E12 F6.
    assert_eq!(
        census(above),
        SideCensus {
            shells: 1,
            faces: 6,
            edges: 12,
            vertices: 8
        }
    );
    assert_eq!(census(below).shells, 1);
    assert_eq!(census(below).vertices, 16);
    // Rings: below's top/bottom faces keep exactly one ring each;
    // above has none.
    let rings = |b: &Body<f64>| b.faces().map(|(_, f)| f.rings.len()).sum::<usize>();
    assert_eq!(rings(below), 2);
    assert_eq!(rings(above), 0);
    // Volume: 4×2×2 box minus 1×1×2 hole = 14; above slab 1×2×2 = 4.
    let (va, vb) = (
        mass_properties(above, Tol::witness()).unwrap().volume,
        mass_properties(below, Tol::witness()).unwrap().volume,
    );
    assert!((va - 4.0).abs() < 1e-12, "above {va}");
    assert!((vb - 10.0).abs() < 1e-12, "below {vb}");
}

/// The trilean point-in-loop predicate itself (the F8 containment
/// seed): In / Out / OnBoundary on a real face loop, plus the
/// escalation posture on the interval lane is covered by the interval
/// acceptance below.
#[test]
fn point_in_loop_trilean() {
    use topo::{LoopContainment, point_in_loop};
    let fx = prism::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 1.0);
    let body = &fx.body;
    let top = body.get_face(fx.top_face).unwrap();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let n = Vec3::new(0.0, 0.0, 1.0);
    let q = |x: f64, y: f64| Point3::new(x, y, 1.0);
    assert_eq!(
        point_in_loop(body, top.outer, n, q(1.0, 1.0), band).unwrap(),
        LoopContainment::In
    );
    assert_eq!(
        point_in_loop(body, top.outer, n, q(5.0, 5.0), band).unwrap(),
        LoopContainment::Out
    );
    assert_eq!(
        point_in_loop(body, top.outer, n, q(2.0, 1.0), band).unwrap(),
        LoopContainment::OnBoundary
    );
    assert_eq!(
        point_in_loop(body, top.outer, n, q(0.0, 0.0), band).unwrap(),
        LoopContainment::OnBoundary
    );
}

/// The interval lane: the acceptance fixtures replayed at `Interval` —
/// declared/structural coincidences decide exactly (dyadic fixture
/// coordinates ⇒ singleton enclosures), the splits land with the same
/// structure as the f64 lane, and the degenerate refusals hold.
#[cfg(feature = "interval")]
#[test]
fn interval_lane_acceptance() {
    use geom_core::Interval;
    // Generic asymmetric split.
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (2.0, 3.0), (0.0, 2.0)];
    let fx = prism::<Interval>(&profile, 1.0);
    let r = split(&fx.body, &plane_y::<Interval>(1.0), Tol::witness()).unwrap();
    let (above, below) = (body_of(&r.above), body_of(&r.below));
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));
    assert_eq!(
        census(below),
        SideCensus {
            shells: 1,
            faces: 6,
            edges: 12,
            vertices: 8
        }
    );
    assert_eq!(
        census(above),
        SideCensus {
            shells: 1,
            faces: 7,
            edges: 15,
            vertices: 10
        }
    );

    // The notched block: three disconnected Above prisms, as at f64.
    let fx = prism::<Interval>(NOTCHED, 1.0);
    let r = split(&fx.body, &plane_y::<Interval>(1.0), Tol::witness()).unwrap();
    assert_eq!(body_of(&r.above).shells().count(), 3);
    assert_eq!(body_of(&r.below).shells().count(), 1);

    // Slicing: three polygons, corners on the plane (containment).
    let s = plane_section(&fx.body, &plane_y::<Interval>(1.0), Tol::witness()).unwrap();
    assert_eq!(s.polygons.len(), 3);

    // One-sided tangency refuses typed on this lane too.
    let fx = prism::<Interval>(&[(3.0, 4.0), (6.0, 1.0), (9.0, 4.0)], 1.0);
    let err = split(&fx.body, &plane_y::<Interval>(1.0), Tol::witness()).unwrap_err();
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
    let r = split(&fx.body, &plane_y(5.0), Tol::witness()).unwrap();
    assert!(matches!(r.above, SplitPart::Empty));
    let below = body_of(&r.below);
    assert_eq!(validate_closed(below), Ok(()));
    assert_eq!(census(below), census(&fx.body));
    // Plane coplanar with the top face: ON contact, no cut — still a
    // typed Empty above.
    let r = split(&fx.body, &plane_y(1.0), Tol::witness()).unwrap();
    assert!(matches!(r.above, SplitPart::Empty));
    assert_eq!(validate_closed(body_of(&r.below)), Ok(()));
}

/// **Every arm of [`SplitError`] names the split in its message.**
///
/// Three stages carry the door's name inside their own (`split_reduce`,
/// `split join`, `split finish`), so `SplitError` does not re-state it
/// and a forwarded refusal says "split" once instead of twice.
/// `Pcurves` is the one stage whose error is shared with callers that
/// are not splits, so that arm supplies the name itself. A stage
/// renamed without this in mind would silently drop the door from every
/// message a consumer sees, which is what this pins.
#[test]
fn every_split_refusal_names_its_door_exactly_once() {
    let band = geom_core::BandError::Empty {
        zero: 1.0,
        escalate: 0.5,
    };
    let cases = [
        SplitError::Reduce(topo::splitting::SplitReduceError::Band(band)),
        SplitError::Join(SplitJoinError::Band(band)),
        SplitError::Finish(SplitFinishError::Band(band)),
        SplitError::Pcurves(topo::pcurves::PcurveMintError::Band(band)),
    ];
    for e in cases {
        let msg = e.to_string();
        assert!(
            msg.contains("split"),
            "the refusal must name its door: {msg}"
        );
        assert_eq!(
            msg.matches("split").count(),
            1,
            "the door is named twice: {msg}"
        );
        assert!(!msg.contains('{'), "Debug guts leaked: {msg}");
    }
}
