//! R2 review probes, ADOPTED (PR #1131, #1031's pole half). They were
//! written to falsify a GATE EXEMPTION, and they succeeded — the
//! exemption was withdrawn, so each split operand here still refuses
//! `NonMaximalFaces` at the boolean's gate. The repair is the caller's
//! explicit `merge_coplanar_faces`, and it takes every one of these
//! seams: once the faces are joined, a seam edge left dangling goes
//! with its free end, at any angle and along a chain.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, line, prism_z};
use geom_core::{Point3, Tol};
use topo::{
    Body, BooleanError, BooleanOp, FaceSurface, LoopBoundary, MefSite, MevSite, boolean_reduce,
    validate, validate_closed,
};

fn point_of(b: &Body<f64>, he: topo::HalfEdgeKey) -> Point3<f64> {
    *b.get_point(
        b.get_vertex(b.get_half_edge(he).unwrap().start)
            .unwrap()
            .point,
    )
    .unwrap()
}

/// CONTROL (the pinned pre-PR behaviour): the top face split by ONE
/// chord between two rim vertices — both endpoints valence 3 — still
/// refuses `NonMaximalFaces`.
#[test]
fn r2_control_single_chord_split_still_refuses() {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let p = prism_z::<f64>(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
        0.0,
        1.0,
        Tol::witness(),
    );
    let mut b = p.body;
    let outer = b.get_face(p.top_face).unwrap().outer;
    let LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
        panic!()
    };
    let cycle = b.loop_cycle(first).unwrap();
    let (he1, he2) = (cycle[0], cycle[2]);
    let (p0, p1) = (point_of(&b, he1), point_of(&b, he2));
    b.mef(
        MefSite::Chords { he1, he2 },
        line(p0, p1),
        FaceSurface::Inherit,
        Tol::witness(),
    )
    .unwrap();
    validate(&b).unwrap();
    let err = boolean_reduce(BooleanOp::Union, &a, &b, Tol::witness()).unwrap_err();
    println!("R2-CONTROL single-chord split => {err:?}");
    assert!(
        matches!(err, BooleanError::NonMaximalFaces { .. }),
        "{err:?}"
    );
}

/// ATTACK on the structural predicate: the SAME defect, but the
/// splitting chord carries ONE interior vertex (valence 2, both of
/// whose edges separate the same face pair). Nothing here is a
/// revolve, no pole, no axis — yet the gate exemption this probe was
/// written against fired on BOTH shared edges, admitting the whole
/// pair. **That exemption was WITHDRAWN because of this row**, so the
/// pair must still refuse `NonMaximalFaces` at the gate, and the
/// caller's explicit merge repairs it: `kef` takes one seam edge and
/// `kev` the other with the mid vertex, bent seam or not.
#[test]
fn r2_attack_midvertex_chord_split() {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let p = prism_z::<f64>(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
        0.0,
        1.0,
        Tol::witness(),
    );
    let mut b = p.body;
    let outer = b.get_face(p.top_face).unwrap().outer;
    let LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
        panic!()
    };
    let cycle = b.loop_cycle(first).unwrap();
    let (he1, he2) = (cycle[0], cycle[2]);
    let (p0, p1) = (point_of(&b, he1), point_of(&b, he2));
    // Interior point of the top quad, off the p0-p1 diagonal.
    let mid = Point3::new(1.0, 0.7, p0.z);
    let strut = b
        .mev(
            MevSite::Fan { he1, he2: he1 },
            mid,
            line(p0, mid),
            Tol::witness(),
        )
        .unwrap();
    b.mef(
        MefSite::Chords {
            he1: strut.he_minus,
            he2,
        },
        line(mid, p1),
        FaceSurface::Inherit,
        Tol::witness(),
    )
    .unwrap();
    validate(&b).unwrap();
    // The mid vertex is valence 2 and both its edges separate the same
    // (planar, same-key) face pair — the predicate's exact shape.
    let orbit = b.vertex_orbit(strut.he_minus).unwrap();
    println!("R2-ATTACK mid-vertex valence = {}", orbit.len());
    let res = boolean_reduce(BooleanOp::Union, &a, &b, Tol::witness());
    match &res {
        Ok(_) => println!("R2-ATTACK mid-vertex chord split => Ok(reduction) — GATE ADMITTED"),
        Err(e) => println!("R2-ATTACK mid-vertex chord split => {e:?}"),
    }
    assert!(
        matches!(res, Err(BooleanError::NonMaximalFaces { .. })),
        "{res:?}"
    );
    assert_repairs(&mut b, 1);
}

/// The explicit merge takes the split top back to one face: one
/// group, `kef` on one seam edge and `kev` on each of the chain's
/// `interior` junctions with the edge dangling from it, tier 2 green.
fn assert_repairs(b: &mut Body<f64>, interior: usize) {
    let (f0, v0, e0) = (b.faces().count(), b.vertices().count(), b.edges().count());
    let out = b
        .merge_coplanar_faces(Tol::witness())
        .expect("the split top repairs");
    let [group] = &out.groups[..] else {
        panic!("one group: {:?}", out.groups)
    };
    assert_eq!(group.killed_vertices.len(), interior);
    assert!(group.rings_made.is_empty());
    assert_eq!(
        (b.faces().count(), b.vertices().count(), b.edges().count()),
        (f0 - 1, v0 - interior, e0 - 1 - interior)
    );
    assert_eq!(validate_closed(b), Ok(()));
}

/// ATTACK, longer chain: TWO interior valence-2 vertices. The middle
/// edge has valence-2 same-pair endpoints at BOTH ends. A seam chain
/// of three edges: the gate refuses it, and the merge loses both
/// interior junctions — whichever edge `kef` takes, the pruning then
/// peels the chain one free end at a time.
#[test]
fn r2_attack_two_midvertex_chain_split() {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let p = prism_z::<f64>(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
        0.0,
        1.0,
        Tol::witness(),
    );
    let mut b = p.body;
    let outer = b.get_face(p.top_face).unwrap().outer;
    let LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
        panic!()
    };
    let cycle = b.loop_cycle(first).unwrap();
    let (he1, he2) = (cycle[0], cycle[2]);
    let (p0, p1) = (point_of(&b, he1), point_of(&b, he2));
    let m1 = Point3::new(0.7, 0.6, p0.z);
    let m2 = Point3::new(1.4, 0.6, p0.z);
    let s1 = b
        .mev(
            MevSite::Fan { he1, he2: he1 },
            m1,
            line(p0, m1),
            Tol::witness(),
        )
        .unwrap();
    let s2 = b
        .mev(
            MevSite::Fan {
                he1: s1.he_minus,
                he2: s1.he_minus,
            },
            m2,
            line(m1, m2),
            Tol::witness(),
        )
        .unwrap();
    b.mef(
        MefSite::Chords {
            he1: s2.he_minus,
            he2,
        },
        line(m2, p1),
        FaceSurface::Inherit,
        Tol::witness(),
    )
    .unwrap();
    validate(&b).unwrap();
    let res = boolean_reduce(BooleanOp::Union, &a, &b, Tol::witness());
    match &res {
        Ok(_) => println!("R2-ATTACK two-mid chain => Ok(reduction) — GATE ADMITTED"),
        Err(e) => println!("R2-ATTACK two-mid chain => {e:?}"),
    }
    assert!(
        matches!(res, Err(BooleanError::NonMaximalFaces { .. })),
        "{res:?}"
    );
    assert_repairs(&mut b, 2);
}
