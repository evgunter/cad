//! Review 1 of PR 5529: valid bodies with coincident line edges that
//! tier 3′ must still pass once it reads their wedges, and invalid ones
//! it must refuse.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{Pose, posed_boxes, poses};
use geom_core::Tol;
use topo::{
    AtRestBody, Body, BooleanResult, Cell, CensusContact, ValidationError,
    subtract, validate_geometric, validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

fn cells(e: &ValidationError) -> Option<(Cell, Cell)> {
    let ValidationError::UnrecordedContact { contact, .. } = e else {
        return None;
    };
    Some(match *contact {
        CensusContact::VertexVertex { a, b } => (Cell::Vertex(a), Cell::Vertex(b)),
        CensusContact::VertexOnFace { vertex, face } => (Cell::Vertex(vertex), Cell::Face(face)),
        CensusContact::VertexOnEdge { vertex, edge } => (Cell::Vertex(vertex), Cell::Edge(edge)),
        CensusContact::EdgeEdgeOverlap { a, b } | CensusContact::EdgeEdgeCross { a, b, .. } => {
            (Cell::Edge(a), Cell::Edge(b))
        }
        CensusContact::EdgeFaceOverlap { edge, face } => (Cell::Edge(edge), Cell::Face(face)),
        CensusContact::ConformalPatch { ref finding }
        | CensusContact::FaceFaceTangent { ref finding } => {
            (Cell::Face(finding.pair.a), Cell::Face(finding.pair.b))
        }
        _ => return None,
    })
}

/// Records every unrecorded contact until none stands, and returns the
/// final tier 3′ verdict.
fn recorded_verdict(what: &str, body: &Body<f64>) -> Result<(), Vec<ValidationError>> {
    let mut pairs: Vec<(Cell, Cell)> = Vec::new();
    for _ in 0..4 {
        let recs = topo::test_support::records(body, pairs.iter().copied());
        match validate_pseudomanifold(body, &recs, t()) {
            Ok(()) => return Ok(()),
            Err(errs) => {
                let new: Vec<_> = errs.iter().filter_map(cells).collect();
                let fresh: Vec<_> = new.into_iter().filter(|p| !pairs.contains(p)).collect();
                if fresh.is_empty() {
                    return Err(errs);
                }
                pairs.extend(fresh);
            }
        }
    }
    eprintln!("{what}: records did not settle");
    let recs = topo::test_support::records(body, pairs.iter().copied());
    validate_pseudomanifold(body, &recs, t())
}

fn wedge_findings(errs: &[ValidationError]) -> Vec<&ValidationError> {
    errs.iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::CoincidentWedgesOverlap { .. }
                    | ValidationError::CoincidentWedgesOverlapUndecided { .. }
            )
        })
        .collect()
}

fn assert_passes(what: &str, body: &Body<f64>) {
    assert_eq!(validate_geometric(body, t()), Ok(()), "{what}: tier 3");
    match recorded_verdict(what, body) {
        Ok(()) => {}
        Err(errs) => panic!(
            "{what}: tier 3′ refuses a valid body; wedge findings {:?}; all {errs:?}",
            wedge_findings(&errs)
        ),
    }
}

/// Two bricks kissing along the z-axis (quadrants 1 and 3), equal and
/// unequal heights, and offset spans: cross-solid, 90° wedges, disjoint.
#[test]
fn p1_bricks_kissing_along_a_line_pass() {
    for pose in poses() {
        for (what, q3z) in [
            ("equal", (0.5, 1.5)),
            ("shorter", (0.5, 1.25)),
            ("offset", (1.0, 2.0)),
            ("inside", (0.75, 1.25)),
        ] {
            let what = format!("{}, kissing {what}", pose.label);
            let b = posed_boxes(
                &what,
                &[
                    [(0.0, 1.0), (0.0, 1.0), (0.5, 1.5)],
                    [(-1.0, 0.0), (-1.0, 0.0), q3z],
                ],
                &pose,
                t(),
            );
            assert_passes(&what, &b);
        }
    }
}

/// Two bricks flush along a face (quadrants 1 and 2), sharing a sheet on
/// each of their coincident edges: disjoint wedges with a coincident
/// bound.
#[test]
fn p2_bricks_flush_on_a_face_pass() {
    for pose in poses() {
        for (what, q2y) in [("equal", (0.0, 1.0)), ("shifted", (0.5, 1.5))] {
            let what = format!("{}, flush {what}", pose.label);
            let b = posed_boxes(
                &what,
                &[
                    [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)],
                    [(-1.0, 0.0), q2y, (0.0, 1.0)],
                ],
                &pose,
                t(),
            );
            assert_passes(&what, &b);
        }
    }
}

/// Three solids on one line: two flush plates and a brick on top across
/// their shared edge (partial along the line).
#[test]
fn p3_three_solids_on_one_line_pass() {
    for pose in poses() {
        let what = format!("{}, three on a line", pose.label);
        let b = posed_boxes(
            &what,
            &[
                [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)],
                [(1.0, 2.0), (0.0, 1.0), (0.0, 1.0)],
                [(1.0, 1.5), (0.25, 0.75), (1.0, 2.0)],
            ],
            &pose,
            t(),
        );
        assert_passes(&what, &b);
    }
}

/// A prism over `profile` extruded along z over `span`.
fn prism(pose: &Pose, what: &str, profile: &[(f64, f64)], span: (f64, f64)) -> AtRestBody<f64> {
    use topo::test_support::{FaceGeometry, describe_as_intersections, finished, prism_ops};
    let mut b = Body::<f64>::new();
    prism_ops(
        &mut b,
        profile,
        span,
        |x, y, z| pose.at([x, y, z]),
        FaceGeometry::Certified,
        t(),
    );
    describe_as_intersections(&mut b, t());
    finished(what, b, t())
}

/// A brick in an L's notch: the L's 270° reflex edge and the brick's 90°
/// corner on one line, complementary wedges with both sheets shared.
#[test]
fn p4_a_brick_in_an_l_notch_passes() {
    for pose in poses() {
        let what = format!("{}, L notch", pose.label);
        let l = prism(
            &pose,
            "the L",
            &[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
            (0.0, 1.0),
        );
        let mut body = l.into_body();
        let brick = posed_boxes(&what, &[[(1.0, 2.0), (1.0, 2.0), (0.25, 0.75)]], &pose, t());
        topo::graft_disjoint(&mut body, &brick).unwrap();
        assert_passes(&what, &body);
    }
}

/// A slab less an asymmetric trench whose keel rests on the floor: the
/// Boolean's slit form, two coincident keel edges of unequal wedges.
#[test]
fn p5_asymmetric_keels_subtract_as_a_valid_slit() {
    for pose in poses() {
        for (what, profile, span) in [
            ("shallow+steep, open", [(2.0, 0.0), (3.9, 0.3), (1.95, 2.0)], (1.5, 4.5)),
            ("shallow+steep, ring", [(2.0, 0.0), (3.9, 0.3), (1.95, 2.0)], (1.5, 2.5)),
            ("leaning, open", [(2.0, 0.0), (3.5, 1.0), (3.0, 2.0)], (1.5, 4.5)),
            ("thin wedge, ring", [(2.0, 0.0), (3.9, 0.01), (1.0, 2.0)], (1.5, 2.5)),
        ] {
            let what = format!("{}, {what}", pose.label);
            let slab = prism(
                &pose,
                "the slab",
                &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
                (0.0, 1.0),
            );
            // the trench's (y, z) profile extruded along x
            use topo::test_support::{FaceGeometry, describe_as_intersections, finished, prism_ops};
            let mut b = Body::<f64>::new();
            prism_ops(
                &mut b,
                &profile,
                span,
                |y, z, x| pose.at([x, y, z]),
                FaceGeometry::Certified,
                t(),
            );
            describe_as_intersections(&mut b, t());
            let trench = finished("the trench", b, t());
            match subtract(&slab, &trench, t()) {
                Ok(BooleanResult::Body(r)) => {
                    let recs = r.contacts.clone();
                    assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
                    if let Err(errs) = validate_pseudomanifold(&r.body, &recs, t()) {
                        panic!(
                            "{what}: tier 3′ refuses the Boolean's slit; wedge findings {:?}; all {errs:?}",
                            wedge_findings(&errs)
                        );
                    }
                }
                other => eprintln!("{what}: the Boolean answered {other:?}"),
            }
        }
    }
}

/// The disjoint-touch control turned invalid: two bricks overlapping
/// in a quadrant along a shared line, grafted as two solids. Not a
/// coincident-wedge case alone (their faces cross), so this only checks
/// the census refuses.
#[test]
fn p6_bricks_sharing_a_quadrant_refuse() {
    let pose = Pose::rest();
    let b = posed_boxes(
        "overlapping",
        &[
            [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)],
            [(0.0, 0.5), (0.0, 0.5), (0.0, 1.0)],
        ],
        &pose,
        t(),
    );
    let errs = recorded_verdict("overlapping", &b).expect_err("overlap refuses");
    eprintln!("p6: {:?}", wedge_findings(&errs));
}

/// Witness 3 replayed, printing every finding (arm 2's share included).
#[test]
fn p7_hair_apart_findings() {
    use topo::test_support::{FaceGeometry, describe_as_intersections, finished, prism_ops};
    let tol = t();
    for k in [-40.0, -4000.0] {
        let d = k * tol.eps();
        let mut body =
            posed_boxes("the brick", &[[(0.0, 1.0), (0.0, 1.0), (0.5, 1.5)]], &Pose::rest(), tol)
                .into_body();
        let mut b = Body::<f64>::new();
        prism_ops(
            &mut b,
            &[(0.0, 0.0), (-d, 1.0), (-1.0, 1.0), (-1.0, 0.0)],
            (0.5, 1.5),
            geom_core::Point3::new,
            FaceGeometry::Certified,
            tol,
        );
        describe_as_intersections(&mut b, tol);
        let prism = finished("the leaning prism", b, tol);
        topo::graft_disjoint(&mut body, &prism).unwrap();
        eprintln!("p7 k={k}: {:?}", recorded_verdict("p7", &body));
    }
}
