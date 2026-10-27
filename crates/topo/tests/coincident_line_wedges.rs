//! **Tier 3′ reads coincident line edges' wedges**: valid bodies with
//! two or more edges on one line pass once their material wedges are
//! read, every pose, and invalid ones refuse on the line, with arm 2
//! reading the overlap between two solids as one passing into the
//! other. A line in the closure of two faces' overlap, on one carrier
//! and facing the same way, is that pair's: arm 2's one interference.
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
/// larger, grafted as two solids. Every line the two share, the z-axis
/// among them, lies in the closure of an overlap of two of their faces
/// on one carrier facing the same way (the caps, and the walls on the
/// axes' planes), so each line's wedges are that pair's, and the pair is
/// one interference: arm 2's, on one aligned face pair.
#[test]
fn bricks_sharing_a_quadrant_are_one_interference() {
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
    assert_same_side("overlapping", &errs);
    assert_eq!(errs.len(), 1, "the lines are the pair's: {errs:?}");
}

/// Arm 2 decides the pair as one interference on a face pair, and says
/// nothing undecided of it.
fn assert_same_side(what: &str, errs: &[ValidationError]) {
    assert_eq!(
        errs.iter()
            .filter(|e| matches!(e, ValidationError::SameSideInterference { .. }))
            .count(),
        1,
        "{what}: arm 2 decides the overlap on one aligned face pair: {errs:?}"
    );
    assert_eq!(
        arm_2(errs),
        [] as [&str; 0],
        "{what}: and leaves nothing undecided: {errs:?}"
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

/// **A prism leaning into a brick on their shared line**: the brick
/// over quadrant 1 and a prism over quadrant 2 whose wall leans `δ`
/// into the brick, `δ` 40 ε and 4000 ε, each its own solid with every
/// touch recorded. The two caps lie on each other facing the same way
/// over the sliver between the walls, so arm 2 decides the pair on
/// them, and the two cap lines that bound the sliver are theirs. The
/// z-axis meets the sliver's caps only at its ends, so its wedges are a
/// pair of their own, and refuse decided.
#[test]
fn a_prism_leaning_into_a_brick_refuses_the_axis_beside_its_caps() {
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
        assert!(
            matches!(
                wedge_findings(&errs)[..],
                [ValidationError::CoincidentWedgesOverlap { witness, .. }]
                    if witness == "(0.0, 0.0, 1.0)"
            ),
            "{what}: the z-axis alone refuses its wedges, decided: {errs:?}"
        );
        assert_same_side(&what, &errs);
        assert_eq!(errs.len(), 2, "{what}: nothing else refuses: {errs:?}");
    }
}

/// The pinwheel at `pose`: prism A over `z ∈ [0, 1]`, B over `zb`,
/// each its own solid.
fn pinwheel(pose: &Pose, zb: (f64, f64)) -> Body<f64> {
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
    let mut body = prism(pose, "A", &a, (0.0, 1.0)).into_body();
    topo::graft_disjoint(&mut body, &prism(pose, "B", &b, zb)).unwrap();
    body
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
/// line refuses its wedges decided, the pair's interference, beside
/// which arm 2 adds nothing, at every pose, with B's span above A's
/// middle and below it.
#[test]
fn a_pinwheel_resting_on_confirmed_face_pairs_refuses_its_doubled_core() {
    for pose in poses() {
        for (span, zb) in [("above", (0.25, 1.5)), ("below", (-0.5, 0.75))] {
            let what = format!("{}, B {span}", pose.label);
            let body = pinwheel(&pose, zb);
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
                [] as [&str; 0],
                "{what}: the overlaps decide the pair, which arm 2 leaves nothing undecided \
                 beside: {errs:?}"
            );
            assert_eq!(errs.len(), 4, "{what}: nothing else refuses: {errs:?}");
        }
    }
}

/// **A pair its wedges decide leaves nothing undecided beside them,
/// whatever else stands.** The pinwheel, and far off a box sunk on an
/// edge into a cube's top over the dips that escalate there
/// (`ESCALATING_DIPS`), at the dips and poses where the sweep does. An
/// escalation names no
/// entity, so it leaves every pair uncleared, the pinwheel's included;
/// none of the pinwheel's touches reads as a crossing, so only its four
/// wedge overlaps decide it. Red, a `CensusUndecidable` on the
/// pinwheel's pair, where arm 2 reads a decided pair's other findings.
#[test]
fn a_pinwheel_beside_an_escalation_refuses_nothing_undecidable() {
    use common::meeting::{ESCALATING_DIPS, sunk_on_an_edge};
    let mut read = 0;
    for (k, pose) in ESCALATING_DIPS
        .into_iter()
        .flat_map(|k| poses().into_iter().map(move |p| (k, p)))
    {
        let what = pose.label;
        let mut body = pinwheel(&pose, (0.25, 1.5));
        let far = Pose::turn(what, [0.0, 0.0, 1.0], 0.0, [20.0, 0.0, 0.0]).after(&pose);
        topo::graft_disjoint_all(&mut body, &sunk_on_an_edge(&far, k * t().eps(), t())).unwrap();
        let errs = recorded_verdict(what, &body).expect_err("the doubled core refuses");
        if !errs
            .iter()
            .any(|e| matches!(e, ValidationError::CensusEscalated { .. }))
        {
            continue;
        }
        read += 1;
        let pinwheel: Vec<_> = body.solids().map(|(s, _)| s).take(2).collect();
        assert!(
            !errs.iter().any(|e| {
                matches!(e, ValidationError::CensusUndecidable { .. })
                    && topo::census_uncleared_solids(&body, e)
                        .is_some_and(|(x, y)| pinwheel.contains(&x) && pinwheel.contains(&y))
            }),
            "{what}: nothing undecidable on the pinwheel's pair: {errs:?}"
        );
        assert_eq!(
            wedge_findings(&errs).len(),
            4,
            "{what}: the four overlaps decide it: {errs:?}"
        );
    }
    assert!(
        read > 0,
        "the witness: an escalation stands at some dip and pose"
    );
}
