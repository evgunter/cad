//! **A pair the census decides interferes is decided, whatever else
//! stands on it.** A touch the touch analysis reads as a crossing is the
//! pair's interference (`ValidationError::TouchInterference`), and a
//! pair a standing finding already decides (a pierce, a same-side
//! crossing, two coincident edges' wedges overlapping) is not refused
//! as undecidable beside it. Each row runs at every pose of
//! `common::meeting::poses` and one oblique turn, since an axis-aligned
//! row alone excludes the failure modes that need a turn.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::meeting::{ESCALATING_DIPS, Pose, posed_box, posed_pyramid, poses, sunk_on_an_edge};
use geom_core::Tol;
use topo::{Body, ContactRecords, EntityId, SolidKey, ValidationError, validate_pseudomanifold};

/// Every pose of [`poses`], and one turn about an oblique axis.
fn every_pose() -> Vec<Pose> {
    let mut v = poses();
    v.push(Pose::turn(
        "oblique",
        [0.3, -0.7, 0.2],
        2.3,
        [0.11, 0.07, -0.19],
    ));
    v
}

fn findings(body: &Body<f64>) -> Vec<ValidationError> {
    validate_pseudomanifold(body, &ContactRecords::default(), Tol::witness())
        .err()
        .unwrap_or_default()
}

/// The solid-pair refusals among `errors`.
fn undecidable(errors: &[ValidationError]) -> Vec<&ValidationError> {
    errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::CensusUndecidable {
                    a: EntityId::Solid(_),
                    b: EntityId::Solid(_),
                    ..
                }
            )
        })
        .collect()
}

/// `a` with `b` grafted beside it, each its own solid.
fn pair(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut body = a.clone();
    topo::graft_disjoint(&mut body, b).unwrap();
    body
}

/// **A box sunk on an edge into a cube is a placement finding.** The
/// overlap shows only as pierces: no vertex of either lies strictly
/// inside the other and no face pair shares a plane. A row asserting a
/// pose clears reads `placement_findings(..).is_empty()`, and that
/// reads the pierces, the census's decided interference. Red, the pose
/// read as clear, where `is_decided_overlap` keeps a list of its own
/// beside `census_interference_solids`.
#[test]
fn a_box_sunk_on_an_edge_is_a_placement_finding() {
    let body = sunk_on_an_edge(&Pose::rest(), 1e-3, Tol::witness());
    let errors = findings(&body);
    assert!(
        errors
            .iter()
            .any(|e| topo::census_interference_solids(&body, e).is_some()),
        "the census decides the overlap: {errors:?}"
    );
    assert!(
        !common::placement_findings(&body, &errors).is_empty(),
        "placement_findings reads the decided overlap: {errors:?}"
    );
}

/// **A pair a standing finding decides leaves nothing undecided beside
/// it.** The box sunk on an edge over [`ESCALATING_DIPS`]: its walls
/// pierce the cube's top, and at some dips and poses the sweep also
/// escalates there, an escalation that leaves any pair it might be about
/// uncleared. Wherever the census decides this pair (from 300 ε, at
/// every ε row), the pierces and a touch on the top read as a crossing
/// decide it, so arm 2 refuses it neither for the escalation nor for
/// the probe's orderings, which find no vertex strictly inside. Red, a
/// `CensusUndecidable` beside a decided pair. (A pair only its standing
/// findings decide is
/// `coincident_line_wedges::a_pinwheel_beside_an_escalation_refuses_nothing_undecidable`.)
#[test]
fn a_box_sunk_past_the_band_refuses_nothing_undecidable_beside_its_pierces() {
    let mut escalated = 0;
    for k in ESCALATING_DIPS {
        for pose in every_pose() {
            let body = sunk_on_an_edge(&pose, k * Tol::witness().eps(), Tol::witness());
            let errors = findings(&body);
            let what = format!("{k} ε, {}", pose.label);
            let decided = errors
                .iter()
                .any(|e| topo::census_interference_solids(&body, e).is_some());
            assert!(
                decided || k < 300.0,
                "{what}: the pierces decide the pair: {errors:?}"
            );
            if !decided {
                continue;
            }
            assert!(
                undecidable(&errors).is_empty(),
                "{what}: nothing undecidable beside them: {errors:?}"
            );
            if errors
                .iter()
                .any(|e| matches!(e, ValidationError::CensusEscalated { .. }))
            {
                escalated += 1;
            }
        }
    }
    assert!(
        escalated > 0,
        "the witness: an escalation stands beside the pierces at some dip and pose"
    );
}

/// The two solids of `body`, in arena order.
fn solids(body: &Body<f64>) -> Vec<SolidKey> {
    body.solids().map(|(s, _)| s).collect()
}

/// The regular tetrahedron inscribed in the unit cube at `pose`: its
/// corners four of the cube's, its edges diagonals of the cube's faces,
/// none of its faces on a plane of the cube's.
fn inscribed(pose: &Pose) -> Body<f64> {
    let tol = Tol::witness();
    let cube = posed_box("the cube", [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)], pose, tol);
    let tet = posed_pyramid(
        &[[1.0, 1.0, 0.0], [1.0, 0.0, 1.0], [0.0, 1.0, 1.0]],
        [0.0, 0.0, 0.0],
        pose,
        tol,
    );
    pair(&cube, &tet)
}

/// **A tetrahedron inscribed in a cube interferes at its touches.** No
/// vertex of either lies strictly inside the other, no edge pierces a
/// face and no face pair shares a plane, so the overlap shows only where
/// they touch. The census reads a touch there as a crossing and reports
/// the one `TouchInterference`, at a corner the two share and read as
/// the two solids, with nothing undecidable beside it. Red, the pair
/// refused undecidable or cleared, where a decided crossing is read as
/// anything else.
#[test]
fn a_tetrahedron_inscribed_in_a_cube_interferes_at_every_pose() {
    for pose in every_pose() {
        let what = pose.label;
        let body = inscribed(&pose);
        let errors = findings(&body);
        let decided: Vec<_> = errors
            .iter()
            .filter(|e| topo::census_interference_solids(&body, e).is_some())
            .collect();
        assert!(
            matches!(decided[..], [ValidationError::TouchInterference { .. }]),
            "{what}: one decided interference, at a touch: {errors:?}"
        );
        let ValidationError::TouchInterference { witness, .. } = decided[0] else {
            unreachable!("matched above");
        };
        let at: Vec<f64> = witness
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        assert!(
            [
                [0.0, 0.0, 0.0],
                [1.0, 1.0, 0.0],
                [1.0, 0.0, 1.0],
                [0.0, 1.0, 1.0]
            ]
            .iter()
            .any(|&c| {
                let q = pose.at(c);
                (q.x - at[0]).abs() < 1e-9
                    && (q.y - at[1]).abs() < 1e-9
                    && (q.z - at[2]).abs() < 1e-9
            }),
            "{what}: the touch is read at a corner the two share: {witness}"
        );
        let (x, y) = topo::census_interference_solids(&body, decided[0]).unwrap();
        let mut got = [x, y];
        got.sort();
        assert_eq!(got.to_vec(), solids(&body), "{what}: the two solids");
        assert!(
            undecidable(&errors).is_empty(),
            "{what}: nothing undecidable beside it: {errors:?}"
        );
    }
}

/// The touches the census finds unrecorded on `body`, as records.
fn recorded(body: &Body<f64>) -> ContactRecords {
    ContactRecords::from_iter(findings(body).iter().filter_map(|e| {
        let [x, y] = topo::census_contact_row(e)?.cells.map(|c| match c {
            topo::RowCell::Result { cell } => cell,
            other => panic!("a cell of the body: {other:?}"),
        });
        Some(topo::Cited::new(
            topo::Contact::new(body, x, y).unwrap(),
            topo::Cites::decided(0),
        ))
    }))
}

/// **A crossing touch decides its pair over an undecided reading
/// standing before it.** The inscribed tetrahedron, its touches with
/// the cube recorded, and far off the box sunk on an edge into a second
/// cube, at the dips and poses where the sweep escalates there
/// ([`ESCALATING_DIPS`]). An
/// escalation names no entity, so it leaves every pair uncleared, the
/// tetrahedron's included, and arm 2 reads it among the standing
/// findings, before the tetrahedron's records; a recorded touch read as
/// a crossing still decides that pair. Red, the tetrahedron's pair
/// refused undecidable, where the first undecided reading ends the
/// pair's read.
#[test]
fn a_crossing_touch_outranks_an_escalation_read_before_it() {
    let mut read = 0;
    for (k, pose) in ESCALATING_DIPS
        .into_iter()
        .flat_map(|k| every_pose().into_iter().map(move |p| (k, p)))
    {
        let dip = k * Tol::witness().eps();
        let what = pose.label;
        let far = Pose::turn(what, [0.0, 0.0, 1.0], 0.0, [10.0, 0.0, 0.0]).after(&pose);
        let mut body = inscribed(&pose);
        let records = recorded(&body);
        let [cube, tet] = solids(&body)[..] else {
            panic!("{what}: two solids");
        };
        topo::graft_disjoint_all(&mut body, &sunk_on_an_edge(&far, dip, Tol::witness())).unwrap();
        let errors = validate_pseudomanifold(&body, &records, Tol::witness())
            .expect_err("the overlaps refuse");
        if !errors
            .iter()
            .any(|e| matches!(e, ValidationError::CensusEscalated { .. }))
        {
            continue;
        }
        read += 1;
        let ours = |x: SolidKey, y: SolidKey| [x, y].contains(&cube) && [x, y].contains(&tet);
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::TouchInterference { .. })
                    && topo::census_interference_solids(&body, e).is_some_and(|(x, y)| ours(x, y))),
            "{what}: the tetrahedron's touch decides its pair: {errors:?}"
        );
        assert!(
            !undecidable(&errors).iter().any(|e| {
                topo::census_uncleared_solids(&body, e).is_some_and(|(x, y)| ours(x, y))
            }),
            "{what}: nothing undecidable on the tetrahedron's pair: {errors:?}"
        );
    }
    assert!(
        read > 0,
        "the witness: an escalation stands at some dip and pose"
    );
}
