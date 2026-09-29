//! Split-seam rows for `merge_coplanar_faces`' dangling-seam pruning:
//! a prism's top face cut by a seam through an interior vertex — on
//! the chord, off it, near it, at a four-way junction, and around a
//! zero-width bigon — merged back into one face.
//!
//! The pruning is topological: once the group's faces are joined, a
//! shared edge whose far end has no other edge dangles inside the
//! merged face, encloses no area, and is deleted together with that
//! free end (`kev`), at any angle and repeatedly. So every row here
//! repairs to the prism's one top face, whatever the seam's shape, and
//! the rows pin the Euler arithmetic of each repair and tier 2 and
//! tier 3 on the result.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::prism_z;
use geom_core::{Point3, Tol};
use topo::{Body, MefSite, MevSite, validate_closed, validate_geometric};

/// A prism whose top face is split by a chord from (0,0) to (2,2)
/// through an interior vertex at `mid` (the PR's own split_top shape,
/// with the mid point free).
fn split_top_at(mid: Point3<f64>) -> Body<f64> {
    let p = prism_z::<f64>(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        0.0,
        1.0,
        Tol::witness(),
    );
    let mut b = p.body;
    let tol = Tol::witness();
    let he1 = he_at(&b, p.top_face, 0.0, 0.0);
    let strut = b
        .mev_line(MevSite::Fan { he1, he2: he1 }, mid, tol)
        .unwrap();
    let he2 = he_at(&b, p.top_face, 2.0, 2.0);
    b.mef_chord(
        MefSite::Chords {
            he1: strut.he_minus,
            he2,
        },
        tol,
    )
    .unwrap();
    assert_eq!(validate_closed(&b), Ok(()), "fixture is tier-2 legal");
    b
}

fn he_at(b: &Body<f64>, face: topo::FaceKey, x: f64, y: f64) -> topo::HalfEdgeKey {
    let outer = b.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
        panic!("outer loop is a cycle")
    };
    b.loop_cycle(first)
        .unwrap()
        .into_iter()
        .find(|&he| {
            let v = b.get_half_edge(he).unwrap().start;
            let pt = b.get_point(b.get_vertex(v).unwrap().point).unwrap();
            (pt.x - x).abs() < 1e-12 && (pt.y - y).abs() < 1e-12
        })
        .expect("a half-edge starting there")
}

fn census(b: &Body<f64>) -> (usize, usize, usize) {
    (b.faces().count(), b.vertices().count(), b.edges().count())
}

/// Merges `b`, requiring the one-face repair: one group, no ring, the
/// seam's junction vertices in `killed_vertices`, the census moved by
/// `delta` = (faces, vertices, edges) removed, tier 2 and tier 3 green.
fn assert_repairs(b: &mut Body<f64>, delta: (usize, usize, usize), what: &str) {
    let tol = Tol::witness();
    let (f0, v0, e0) = census(b);
    let out = b
        .merge_coplanar_faces(tol)
        .unwrap_or_else(|e| panic!("{what}: the split top repairs, got {e:?}"));
    assert_eq!(out.groups.len(), 1, "{what}");
    let group = &out.groups[0];
    assert!(group.rings_made.is_empty(), "{what}: kev path, not kemr");
    assert_eq!(group.killed_vertices.len(), delta.1, "{what}");
    for v in &group.killed_vertices {
        assert!(b.get_vertex(*v).is_none(), "{what}: {v:?} deleted");
    }
    assert_eq!(
        census(b),
        (f0 - delta.0, v0 - delta.1, e0 - delta.2),
        "{what}: Euler arithmetic"
    );
    assert_eq!(validate_closed(b), Ok(()), "{what}: tier 2 after repair");
    assert_eq!(
        validate_geometric(b, tol),
        Ok(()),
        "{what}: tier 3 after repair"
    );
}

/// D1 — collinear and bent alike: the same subdivided chord with its
/// interior vertex on the diagonal and off it (R1's P2 point) repairs
/// the same way — `kef` takes one seam edge, `kev` the other with the
/// vertex it dangles from. The angle at the vertex decides nothing.
#[test]
fn d1_collinear_and_bent_seams_both_repair() {
    for mid in [Point3::new(1.0, 1.0, 1.0), Point3::new(0.9, 0.6, 1.0)] {
        let mut b = split_top_at(mid);
        assert_repairs(&mut b, (1, 1, 2), &format!("mid {mid:?}"));
    }
}

/// D2 — no numeric band decides the repair: the interior vertex
/// offset from the diagonal well inside the band's `zero`, inside the
/// old ambiguity band, and far outside it (0.1×, 5× and 10⁶× `zero`,
/// so the three track the run's ε) all repair the same way. There is
/// no escalation arm to reach: the decision reads a vertex's edge
/// count, not a coordinate.
#[test]
fn d2_no_band_decides_the_repair() {
    let band = geom_core::Band::linear(Tol::witness()).expect("linear band");
    for scale in [0.1, 5.0, 1.0e6] {
        let d = band.zero() * scale;
        let mut b = split_top_at(Point3::new(1.0, 1.0 + d, 1.0));
        assert_repairs(&mut b, (1, 1, 2), &format!("offset {d:e}"));
    }
}

/// D3 — a four-way junction: the pole of a FOUR-sector disc, the
/// top face cut by both diagonals through its centre. The four
/// sectors are one group sharing four spokes; the absorption kills
/// three of them with `kef`, which leaves the fourth dangling from
/// the centre, and `kev` takes it with the centre. One top face,
/// the centre gone.
#[test]
fn d3_a_four_way_junction_repairs() {
    let p = prism_z::<f64>(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        0.0,
        1.0,
        Tol::witness(),
    );
    let mut b = p.body;
    let tol = Tol::witness();
    let mid = Point3::new(1.0, 1.0, 1.0);
    // First diagonal through the centre: (0,0) -> M -> (2,2).
    let he1 = he_at(&b, p.top_face, 0.0, 0.0);
    let strut = b
        .mev_line(MevSite::Fan { he1, he2: he1 }, mid, tol)
        .unwrap();
    let he2 = he_at(&b, p.top_face, 2.0, 2.0);
    b.mef_chord(
        MefSite::Chords {
            he1: strut.he_minus,
            he2,
        },
        tol,
    )
    .unwrap();
    // Second diagonal, both halves into M: (2,0) -> M and M -> (0,2).
    // Each cut lives in whichever fragment holds both endpoints.
    let fragment_with = |b: &Body<f64>, x: f64, y: f64| {
        b.faces()
            .map(|(fk, _)| fk)
            .find(|&fk| cycle_has_corner(b, fk, 1.0, 1.0) && cycle_has_corner(b, fk, x, y))
    };
    let fk = fragment_with(&b, 2.0, 0.0).expect("a fragment holds both M and (2,0)");
    let he_corner = he_at(&b, fk, 2.0, 0.0);
    let m_in_face = he_at(&b, fk, 1.0, 1.0);
    b.mef_chord(
        MefSite::Chords {
            he1: he_corner,
            he2: m_in_face,
        },
        tol,
    )
    .unwrap();
    // And M -> (0,2) in the fragment holding both.
    let fk2 = fragment_with(&b, 0.0, 2.0).expect("a fragment holds both M and (0,2)");
    let he_m2 = he_at(&b, fk2, 1.0, 1.0);
    let he_c2 = he_at(&b, fk2, 0.0, 2.0);
    b.mef_chord(
        MefSite::Chords {
            he1: he_c2,
            he2: he_m2,
        },
        tol,
    )
    .unwrap();
    assert_eq!(
        validate_closed(&b),
        Ok(()),
        "four-sector fixture is tier-2 legal"
    );
    assert_repairs(&mut b, (3, 1, 4), "four-way junction");
}

fn cycle_has_corner(b: &Body<f64>, fk: topo::FaceKey, x: f64, y: f64) -> bool {
    let outer = b.get_face(fk).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
        return false;
    };
    let Some(cycle) = b.loop_cycle(first) else {
        return false;
    };
    cycle.iter().any(|&he| {
        let v = b.get_half_edge(he).unwrap().start;
        let pt = b.get_point(b.get_vertex(v).unwrap().point).unwrap();
        (pt.x - x).abs() < 1e-12 && (pt.y - y).abs() < 1e-12
    })
}

/// D4 — a zero-width bigon: a strut from the corner (0,0) to M
/// closed by a second chord M → (0,0) along the same line, so the top
/// face carries a zero-area face whose two edges both border it. The
/// bigon is absorbed by `kef` across one edge, which leaves the other
/// dangling from M; M was the bigon's tip, and with the bigon gone it
/// is on no face's boundary, so `kev` takes it with that edge. The
/// top face is whole again.
#[test]
fn d4_a_zero_width_bigon_is_absorbed_with_its_tip() {
    let p = prism_z::<f64>(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        0.0,
        1.0,
        Tol::witness(),
    );
    let mut b = p.body;
    let tol = Tol::witness();
    let mid = Point3::new(1.0, 1.0, 1.0);
    let he1 = he_at(&b, p.top_face, 0.0, 0.0);
    let strut = b
        .mev_line(MevSite::Fan { he1, he2: he1 }, mid, tol)
        .unwrap();
    // Close a bigon: a second edge M -> (0,0) beside the strut.
    let he_back = he_at(&b, p.top_face, 0.0, 0.0);
    b.mef_chord(
        MefSite::Chords {
            he1: strut.he_minus,
            he2: he_back,
        },
        tol,
    )
    .expect("the bigon closes through mef_chord");
    assert_eq!(validate_closed(&b), Ok(()), "the bigon is tier-2 legal");
    assert_repairs(&mut b, (1, 1, 2), "zero-width bigon");
}
