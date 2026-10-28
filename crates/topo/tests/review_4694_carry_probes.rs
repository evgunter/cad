//! Review rows for PR 4694 (the surgery carry): a merge's killed seam
//! vertex, and a split through a corner kiss.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use common::{brick, describe_as_intersections, finished, geometric_cube, line};
use geom_core::{Point3, Tol, Vec3};
use topo::construction::Construct;
use topo::construction::{FaceSurface, MefSite};
use topo::{
    AtRestBody, BooleanBody, BooleanResult, Cell, Cited, Cites, Contact, ContactRecords, union,
    validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

fn box_of(x: (f64, f64), y: (f64, f64), z: (f64, f64), what: &str) -> AtRestBody<f64> {
    finished(what, brick(x, y, z, tol()), tol())
}

fn union_of(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> BooleanBody<f64> {
    match union(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    }
}

/// **A merge's killed seam vertex is substituted where its seam edge
/// is.** The vertex lies inside the merged face exactly as its killed
/// seam edges do, and the merge's one lineage (`MergedGroup::substitutions`,
/// which the boolean's descendant map reads too) takes both onto the
/// kept face. A record of the vertex and the bottom face then lands on
/// (kept face, bottom) by lineage, unread. Red if the lineage keeps the
/// vertex, or the carry re-reads a substituted pair.
#[test]
fn a_merges_killed_seam_vertex_is_substituted_with_its_edge() {
    let mut cube = geometric_cube::<f64>(tol());
    describe_as_intersections(&mut cube.body, tol());
    let bottom = cube.mefs[0].face;
    let (a1, c1) = (cube.mevs[3].vertex, cube.mevs[5].vertex);
    let top = cube.seed.face;
    let f = cube.body.get_face(top).unwrap();
    let topo::LoopBoundary::Cycle { first } = cube.body.get_loop(f.outer).unwrap().boundary else {
        panic!("the top loop is a cycle");
    };
    let starting = |v| {
        cube.body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&he| cube.body.get_half_edge(he).unwrap().start == v)
            .unwrap()
    };
    let (he1, he2) = (starting(a1), starting(c1));
    let at = |v| {
        *cube
            .body
            .get_point(cube.body.get_vertex(v).unwrap().point)
            .unwrap()
    };
    let diagonal = cube
        .body
        .mef(
            MefSite::Chords { he1, he2 },
            line(at(a1), at(c1)),
            FaceSurface::Inherit,
            tol(),
        )
        .expect("the diagonal cuts the top");
    let mid = cube
        .body
        .split_edge(diagonal.edge, 0.5, tol())
        .expect("the diagonal splits");
    let vertex_rec: ContactRecords = [Cited::new(
        Contact::new(&cube.body, Cell::Vertex(mid.vertex), Cell::Face(bottom)).unwrap(),
        Cites::decided(0),
    )]
    .into_iter()
    .collect();
    let outcome = cube.body.merge_coplanar_faces(tol()).expect("the merge");
    assert!(outcome.groups[0].killed_vertices.contains(&mid.vertex));
    let kept = Cell::Face(outcome.groups[0].kept);
    let lineage = outcome.lineage();
    for (what, cell) in [
        ("the seam edge", Cell::Edge(mid.new_edge)),
        ("the seam vertex", Cell::Vertex(mid.vertex)),
    ] {
        assert_eq!(
            lineage.fate(cell),
            topo::Fate::Substituted(kept),
            "{what} is the kept face now"
        );
    }
    assert_eq!(
        outcome.carry_contacts(
            &vertex_rec,
            &cube.body,
            geom_core::Band::linear(tol()).unwrap(),
            Some(topo::RegionLane::certified()),
        ),
        Ok([Cited::new(
            Contact::new(&cube.body, kept, Cell::Face(bottom)).unwrap(),
            Cites::one(topo::Backing::Carried {
                input: 0,
                record: 0
            }),
        )]
        .into_iter()
        .collect()),
        "the kept top against the bottom"
    );
}

/// **A split through a corner kiss carries the kiss onto both sides.**
/// The plane x = y passes through the kiss and halves both cubes, so
/// both sides hold the two cubes meeting at (1, 1, 1): one side under
/// the operand's keys, the other under the copies the cut minted
/// (`SplitNaming::vertex_pairs`), which share the kiss's point. The
/// record lands on both, each citing the operand's, and both pass at
/// rest. Red if the carry ignores the cut's copies: the second side
/// refuses the kiss undeclared.
#[test]
fn a_split_through_a_corner_kiss_carries_it_onto_both_sides() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), "b");
    let base = union_of(&a, &b);
    let plane = common::split_plane(Point3::new(1.0, 1.0, 1.0), Vec3::new(1.0, -1.0, 0.0), tol());
    let split = topo::split(&base.body, &plane, tol()).expect("the kiss splits");
    let sides = split
        .carry_contacts(
            &base.contacts,
            geom_core::Band::linear(tol()).unwrap(),
            Some(topo::RegionLane::certified()),
        )
        .expect("the record lands");
    for (what, body, recs) in [
        ("above", split.above.body(), &sides.above),
        ("below", split.below.body(), &sides.below),
    ] {
        assert_eq!(recs.vertex_vertex().count(), 1, "{what}: {recs:?}");
        assert_eq!(
            recs.rows()[0].cites.iter().collect::<Vec<_>>(),
            [topo::Backing::Carried {
                input: 0,
                record: 0
            }],
            "{what}"
        );
        assert_eq!(
            validate_pseudomanifold(body.expect("material"), recs, tol()),
            Ok(()),
            "{what}"
        );
    }
}
