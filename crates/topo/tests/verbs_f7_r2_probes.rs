//! R2 review probes, ADOPTED (PR #1131, #1031's pole half). They were
//! written to falsify a GATE EXEMPTION, and they succeeded — the
//! exemption was withdrawn. Left as the split makes it, each body here
//! is refused by the at-rest gate, one `ScaffoldAtRest` per seam edge
//! between its same-plane-key pair; with each seam at rest in the
//! shared plane's chart it finishes, and still refuses
//! `NonMaximalFaces` at the boolean's gate, through the public union
//! and the public reduction. The repair is the caller's
//! explicit `merge_coplanar_faces`, and it takes every one of these
//! seams: once the faces are joined, a seam edge left dangling goes
//! with its free end, at any angle and along a chain.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished, line, prism_z};
use geom_core::{Point3, Tol};
use topo::{
    AtRestBody, Body, BooleanError, BooleanOp, FaceSurface, LoopBoundary, MefSite, MevSite,
    ValidationError, boolean_reduce, validate, validate_closed,
};

fn point_of(b: &Body<f64>, he: topo::HalfEdgeKey) -> Point3<f64> {
    *b.get_point(
        b.get_vertex(b.get_half_edge(he).unwrap().start)
            .unwrap()
            .point,
    )
    .unwrap()
}

/// `b`'s at-rest refusal is exactly one `ScaffoldAtRest` for each edge
/// separating two faces on one surface key (the seams a split with an
/// inherited surface leaves), `seams` of them in edge-arena order, and
/// nothing else; with every seam at rest in the shared plane's chart
/// it finishes, and the public union and the public reduction with a
/// unit brick overlapping it refuse it `NonMaximalFaces` on a seam.
fn assert_refused_on_its_seams(b: &Body<f64>, seams: usize, what: &str) {
    let tol = Tol::witness();
    let face_of = |he| {
        b.get_loop(b.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    let want: Vec<ValidationError> = b
        .edges()
        .filter(|(_, e)| {
            let (fp, fm) = (face_of(e.he_plus), face_of(e.he_minus));
            fp != fm && b.get_face(fp).unwrap().surface == b.get_face(fm).unwrap().surface
        })
        .map(|(edge, _)| ValidationError::ScaffoldAtRest { edge })
        .collect();
    assert_eq!(want.len(), seams, "{what}: seam-edge census");
    let errors = AtRestBody::validate(b.clone(), Tol::witness())
        .expect_err(&format!("{what}: a split body does not finish"));
    println!("{what} => {errors:?}");
    assert_eq!(
        errors, want,
        "{what}: refused on every seam edge, and on nothing else"
    );
    let mut rested = b.clone();
    let seam_edges: Vec<topo::EdgeKey> = want
        .iter()
        .map(|e| match e {
            ValidationError::ScaffoldAtRest { edge } => *edge,
            _ => unreachable!(),
        })
        .collect();
    for &edge in &seam_edges {
        let e = rested.get_edge(edge).unwrap();
        let (p0, p1) = (point_of(&rested, e.he_plus), point_of(&rested, e.he_minus));
        let chart = rested.get_face(face_of(e.he_plus)).unwrap().surface;
        rested
            .set_edge_curve(
                edge,
                geom_brep::EdgeCurveSpec::line_between(p0, p1).at_rest_in_chart(chart, false),
                tol,
            )
            .unwrap_or_else(|e| panic!("{what}: the seam rests in its plane's chart: {e:?}"));
    }
    let operand = finished(what, rested, tol);
    let a = finished(
        "the unit brick",
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    );
    for (door, err) in [
        (
            "union",
            topo::union(&a, &operand, tol).map(|_| ()).unwrap_err(),
        ),
        (
            "boolean_reduce",
            boolean_reduce(BooleanOp::Union, &a, &operand, tol)
                .map(|_| ())
                .unwrap_err(),
        ),
    ] {
        println!("{what} {door} => {err:?}");
        assert!(
            matches!(
                err,
                BooleanError::NonMaximalFaces { operand: topo::Operand::B, edge }
                    if seam_edges.contains(&edge)
            ),
            "{what}, {door}: the gate refuses on a seam: {err:?}"
        );
    }
}

/// CONTROL (the pinned pre-PR behaviour): the top face split by ONE
/// chord between two rim vertices — both endpoints valence 3 — still
/// refuses `NonMaximalFaces`.
#[test]
fn r2_control_single_chord_split_still_refuses() {
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
    assert_refused_on_its_seams(&b, 1, "R2-CONTROL single-chord split");
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
    assert_refused_on_its_seams(&b, 2, "R2-ATTACK mid-vertex chord split");
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
    assert_refused_on_its_seams(&b, 3, "R2-ATTACK two-mid chain");
    assert_repairs(&mut b, 2);
}
