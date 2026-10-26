//! **Tier 3′ reads coincident line edges' wedges**: valid bodies with
//! two or more edges on one line pass once their material wedges are
//! read, every pose, and invalid ones refuse on the line, with arm 2
//! reading the overlap between two solids as one passing into the
//! other.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{Pose, posed_boxes, poses};
use geom_core::Tol;
use topo::{
    AtRestBody, Body, BooleanResult, Cell, CensusContact, ValidationError, subtract,
    validate_geometric, validate_pseudomanifold,
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
fn bricks_kissing_along_a_line_pass() {
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
fn bricks_flush_on_a_face_pass() {
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
fn three_solids_on_one_line_pass() {
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
fn a_brick_in_an_l_notch_passes() {
    for pose in poses() {
        let what = format!("{}, L notch", pose.label);
        let l = prism(
            &pose,
            "the L",
            &[
                (0.0, 0.0),
                (2.0, 0.0),
                (2.0, 1.0),
                (1.0, 1.0),
                (1.0, 2.0),
                (0.0, 2.0),
            ],
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
fn asymmetric_keels_subtract_as_a_valid_slit() {
    for pose in poses() {
        for (what, profile, span) in [
            (
                "shallow+steep, open",
                [(2.0, 0.0), (3.9, 0.3), (1.95, 2.0)],
                (1.5, 4.5),
            ),
            (
                "shallow+steep, ring",
                [(2.0, 0.0), (3.9, 0.3), (1.95, 2.0)],
                (1.5, 2.5),
            ),
            (
                "leaning, open",
                [(2.0, 0.0), (3.5, 1.0), (3.0, 2.0)],
                (1.5, 4.5),
            ),
            (
                "thin wedge, ring",
                [(2.0, 0.0), (3.9, 0.01), (1.0, 2.0)],
                (1.5, 2.5),
            ),
        ] {
            let what = format!("{}, {what}", pose.label);
            let slab = prism(
                &pose,
                "the slab",
                &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
                (0.0, 1.0),
            );
            // the trench's (y, z) profile extruded along x
            use topo::test_support::{
                FaceGeometry, describe_as_intersections, finished, prism_ops,
            };
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
                other => panic!("{what}: the Boolean builds the slit form, not {other:?}"),
            }
        }
    }
}

/// The kiss of [`bricks_kissing_along_a_line_pass`] turned invalid: two
/// bricks of one quadrant along the z-axis, the smaller inside the
/// larger, grafted as two solids. Their faces lie on each other, so the
/// body refuses on more than its lines, but every line the two share
/// refuses its wedges decided, the z-axis among them, and arm 2 reads
/// the two solids as one passing into the other.
#[test]
fn bricks_sharing_a_quadrant_refuse_on_their_lines() {
    let b = posed_boxes(
        "overlapping",
        &[
            [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)],
            [(0.0, 0.5), (0.0, 0.5), (0.0, 1.0)],
        ],
        &Pose::rest(),
        t(),
    );
    let errs = recorded_verdict("overlapping", &b).expect_err("overlap refuses");
    let wedges = wedge_findings(&errs);
    assert!(
        wedges
            .iter()
            .all(|e| matches!(e, ValidationError::CoincidentWedgesOverlap { .. })),
        "every shared line's wedges overlap, decided: {wedges:?}"
    );
    assert!(
        wedges.iter().any(|e| matches!(
            e,
            ValidationError::CoincidentWedgesOverlap { witness, .. } if witness == "(0.0, 0.0, 0.5)"
        )),
        "the z-axis refuses its wedges: {wedges:?}"
    );
    assert_eq!(
        wedges.len(),
        5,
        "the five lines the bricks share: {wedges:?}"
    );
    assert_eq!(
        arm_2(&errs),
        [MIXED_TOUCH],
        "arm 2 reads the pair as one passing into the other: {errs:?}"
    );
}

/// What arm 2 says of each solid pair it refuses.
fn arm_2(errs: &[ValidationError]) -> Vec<&str> {
    errs.iter()
        .filter_map(|e| match e {
            ValidationError::CensusUndecidable { what, .. } => Some(*what),
            _ => None,
        })
        .collect()
}

/// Arm 2's sentence for a touch where one material passes into the
/// other.
const MIXED_TOUCH: &str = "one passes into the other where they touch. Recourse: move them so \
                           they meet without overlapping, or not at all";

/// **A prism leaning into a brick on their shared line**: the brick
/// over quadrant 1 and a prism over quadrant 2 whose wall leans `δ`
/// into the brick, `δ` 40 ε and 4000 ε, each its own solid with every
/// touch recorded. No face crosses another away from the lines: the
/// three lines the two share each refuse their wedges decided, and arm
/// 2 reads the overlap for the two solids as one passing into the
/// other, the touch reading's own verdict, not as a crossing some other
/// finding reports.
#[test]
fn a_prism_leaning_into_a_brick_reads_as_passing_into_it() {
    use topo::test_support::{FaceGeometry, describe_as_intersections, finished, prism_ops};
    let tol = t();
    for k in [40.0, 4000.0] {
        let what = format!("leaning {k} ε");
        let d = k * tol.eps();
        let mut body = posed_boxes(
            "the brick",
            &[[(0.0, 1.0), (0.0, 1.0), (0.5, 1.5)]],
            &Pose::rest(),
            tol,
        )
        .into_body();
        let mut b = Body::<f64>::new();
        prism_ops(
            &mut b,
            &[(0.0, 0.0), (d, 1.0), (-1.0, 1.0), (-1.0, 0.0)],
            (0.5, 1.5),
            geom_core::Point3::new,
            FaceGeometry::Certified,
            tol,
        );
        describe_as_intersections(&mut b, tol);
        let prism = finished("the leaning prism", b, tol);
        topo::graft_disjoint(&mut body, &prism).unwrap();
        let errs = recorded_verdict(&what, &body).expect_err("the lean refuses");
        let wedges = wedge_findings(&errs);
        assert!(
            wedges.len() == 3
                && wedges
                    .iter()
                    .all(|e| matches!(e, ValidationError::CoincidentWedgesOverlap { .. })),
            "{what}: the three shared lines' wedges overlap, decided: {errs:?}"
        );
        assert_eq!(
            arm_2(&errs),
            [MIXED_TOUCH],
            "{what}: arm 2 reads the pair as one passing into the other: {errs:?}"
        );
        assert_eq!(errs.len(), 4, "{what}: nothing else refuses: {errs:?}");
    }
}

/// **A pinwheel of two prisms doubles the unit square's material, and
/// the face pairs it rests on do not answer for that.** Both profiles
/// hold the unit square `R`, with arms swept opposite ways about it, so
/// the two prisms meet flush over four wall areas, and `R × (z-overlap)`
/// lies in both: on each of the square's corner lines the two edges
/// bound material on the same side. With the four face–face records the
/// census asks, every confirmed pair holds those lines in its overlap's
/// closure, which answers for their touch's record, never for their
/// sides (D1: no record makes two wedges' overlap valid). Each corner
/// line refuses its wedges decided, and arm 2 reads the pair as one
/// passing into the other, at every pose, with B's span above A's
/// middle and below it.
#[test]
fn a_pinwheel_resting_on_confirmed_face_pairs_refuses_its_doubled_core() {
    let a = [
        (-2.0, -2.0),
        (3.0, -2.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (3.0, 3.0),
        (-2.0, 3.0),
        (0.0, 1.0),
        (0.0, 0.0),
    ];
    let b = [
        (0.0, 0.0),
        (1.0, 0.0),
        (3.0, -2.0),
        (3.0, 3.0),
        (1.0, 1.0),
        (0.0, 1.0),
        (-2.0, 3.0),
        (-2.0, -2.0),
    ];
    for pose in poses() {
        for (span, zb) in [("above", (0.25, 1.5)), ("below", (-0.5, 0.75))] {
            let what = format!("{}, B {span}", pose.label);
            let mut body = prism(&pose, "A", &a, (0.0, 1.0)).into_body();
            topo::graft_disjoint(&mut body, &prism(&pose, "B", &b, zb)).unwrap();
            assert_eq!(validate_geometric(&body, t()), Ok(()), "{what}: tier 3");
            let errs = recorded_verdict(&what, &body).expect_err("the doubled core refuses");
            let wedges = wedge_findings(&errs);
            assert!(
                wedges.len() == 4
                    && wedges
                        .iter()
                        .all(|e| matches!(e, ValidationError::CoincidentWedgesOverlap { .. })),
                "{what}: the core's four corner lines refuse their wedges, decided: {errs:?}"
            );
            assert_eq!(
                arm_2(&errs),
                [MIXED_TOUCH],
                "{what}: arm 2 reads the pair as one passing into the other: {errs:?}"
            );
            assert_eq!(errs.len(), 5, "{what}: nothing else refuses: {errs:?}");
        }
    }
}
