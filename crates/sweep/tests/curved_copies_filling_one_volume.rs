//! **Copies whose coincident faces are curved, or planar with an arc
//! rim, interfere** (`topo::census`'s aligned face-pair arm on curved
//! carriers, and its same-side reading of an arc-bounded disc): two
//! revolved cylinders or balls of one volume are one
//! [`ValidationError::SameSideInterference`], however every boundary
//! cell of the two coincides, and copies that only abut or stand off in
//! the band are none.
//!
//! Every body is turned off the world axes and moved off the origin
//! ([`tilted`]), so no reading rests on an axis-aligned carrier.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use geom_core::{Band, Point2, RigidMotion, Tol, UnitVec3, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, ContactRecords, EntityId, ValidationError};

/// The unit square over `[0, h]`, revolved about `y` by `turn`: a
/// cylinder of radius 1 and height `h`, its walls two half-bands on one
/// key and its caps two discs, or a wedge of it.
fn cylinder(h: f64, turn: Revolution<f64>) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, h), 0.0),
            (Point2::new(0.0, h), 0.0),
        ],
        turn,
        Tol::witness(),
    )
}

/// The unit ball, its poles on `y`.
fn ball() -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        Tol::witness(),
    )
}

/// The unit ball with its cap above `y = 1/2` cut off: a sphere face
/// that is a proper subset of the ball's, and a disc on the cut.
fn cut_ball() -> Body<f64> {
    let rim = Point2::new(0.75_f64.sqrt(), 0.5);
    revolved_about_y(
        vec![
            (Point2::new(0.0, -1.0), (core::f64::consts::PI / 6.0).tan()),
            (rim, 0.0),
            (Point2::new(0.0, 0.5), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    UnitVec3::new(v, "a row's axis", Band::linear(Tol::witness()).unwrap()).unwrap()
}

/// The pose every row reads its bodies in: a turn about an axis off
/// every world axis, then a move off the origin.
fn tilted() -> RigidMotion<f64> {
    RigidMotion::rotation_then_translation(
        unit(Vec3::new(0.3, 0.2, 1.0)),
        0.7,
        Vec3::new(0.31, -0.17, 0.42),
    )
    .unwrap()
}

/// A turn by `angle` about the bodies' own axis, `y`.
fn spun(angle: f64) -> RigidMotion<f64> {
    RigidMotion::rotation(UnitVec3::exact_y(), angle).unwrap()
}

/// `body` moved by `inner`, then by [`tilted`].
fn placed(body: &Body<f64>, inner: RigidMotion<f64>) -> Body<f64> {
    let motion = tilted().compose(inner).unwrap();
    topo::transform_rigid(body, &motion, Tol::witness()).unwrap()
}

/// `a` and `b` as two solids of one body.
fn two(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut body = a.clone();
    topo::graft_disjoint_all_keyed(&mut body, b).unwrap();
    body
}

/// What the census finds over `body` at rest, each solid its own group,
/// and the flush pairs between them.
fn at_rest(body: &Body<f64>) -> (topo::RecordedAtRest, topo::FlushAtRest) {
    let groups: BTreeMap<_, u32> = body.solids().map(|(s, _)| s).zip(0..).collect();
    let recorded = topo::record_at_rest(
        body,
        &ContactRecords::default(),
        |records| {
            topo::validate_pseudomanifold(body, records, Tol::witness())
                .err()
                .unwrap_or_default()
        },
        &|s| groups.get(&s).copied(),
        0,
    );
    let band = Band::linear(Tol::witness()).unwrap();
    let flush =
        <f64 as topo::AtRestPolicy>::flush_at_rest(body, &|s| groups.get(&s).copied(), band);
    (recorded, flush)
}

/// The findings the census reads as interference between two solids.
fn interference<'a>(body: &Body<f64>, findings: &'a [ValidationError]) -> Vec<&'a ValidationError> {
    findings
        .iter()
        .filter(|f| topo::census_interference_solids(body, f).is_some())
        .collect()
}

/// The surface kind of face `f`'s carrier.
fn kind(body: &Body<f64>, f: EntityId) -> Option<geom::SurfaceKind> {
    let EntityId::Face(f) = f else { return None };
    Some(body.get_surface(body.get_face(f)?.surface)?.kind())
}

/// The undecidable face pairs on two carriers of `kind` each.
fn undecided_pairs(
    body: &Body<f64>,
    findings: &[ValidationError],
    of: geom::SurfaceKind,
) -> Vec<String> {
    findings
        .iter()
        .filter(|f| {
            matches!(f, ValidationError::CensusUndecidable { a, b, .. }
                if kind(body, *a) == Some(of) && kind(body, *b) == Some(of))
        })
        .map(|f| format!("{f:?}"))
        .collect()
}

/// That `body`'s two solids read as one `SameSideInterference` and no
/// other interference, with no unrecorded touch left beside it (a
/// coincident vertex, edge or rim is a touch in the closure of the
/// pair's shared area, which the pair answers for), and that neither
/// solid lies flush in the other.
fn assert_one_same_side(label: &str, body: &Body<f64>) -> topo::RecordedAtRest {
    let (recorded, flush) = at_rest(body);
    assert!(
        matches!(
            interference(body, &recorded.findings)[..],
            [ValidationError::SameSideInterference { .. }]
        ),
        "{label}: one same-side interference: {:?}",
        recorded.findings
    );
    assert!(
        !recorded
            .findings
            .iter()
            .any(|f| matches!(f, ValidationError::UnrecordedContact { .. })),
        "{label}: every touch is in the closure of the shared area: {:?}",
        recorded.findings
    );
    assert!(
        recorded.rows.is_empty() && flush.rows.is_empty() && flush.findings.is_empty(),
        "{label}: an overlapping pair records nothing: {:?} {flush:?}",
        recorded.rows
    );
    recorded
}

/// **Two copies of one revolved cylinder fill one volume** (the item's
/// probe, turned off the axes), and so do a copy spun about its own axis,
/// whose seams lie apart, and a half-height copy standing in the base,
/// whose wall and bottom disc are proper parts of the base's. Each is one
/// `SameSideInterference`: the two bottom discs, arc-bounded, are read by
/// the faces' own reading where the region door refuses the arc, and the
/// walls, on two keys, by the curved aligned arm.
///
/// What stays undecidable is pinned: no pair of discs (each pair whose
/// interiors meet is decided), and in the coincident copies only the
/// half-bands that share a seam ruling and no interior (one of each
/// copy, a turn apart), which the curved arm finds no witness for and
/// does not read as apart. The spun copy's half-bands each share
/// interior with both of the other's, so no wall pair is left. A disc
/// against the other copy's wall, two carriers meeting along the rim,
/// is the proximity arm's, and stays undecidable.
///
/// Red if the arc-bounded disc pair goes back to the region door's
/// refusal, the curved arm stops reading two keys, or a same-side pair
/// is still refused by the proximity arm beside its decided overlap.
#[test]
fn copies_of_one_cylinder_are_one_interference() {
    let base = cylinder(1.0, Revolution::Full);
    let half = cylinder(0.5, Revolution::Full);
    for (label, b, walls_left) in [
        ("coincident", placed(&base, spun(0.0)), 2),
        ("spun", placed(&base, spun(1.1)), 0),
        ("half height", placed(&half, spun(0.4)), 0),
    ] {
        let body = two(&placed(&base, spun(0.0)), &b);
        let recorded = assert_one_same_side(label, &body);
        let discs = undecided_pairs(&body, &recorded.findings, geom::SurfaceKind::Plane);
        assert!(
            discs.is_empty(),
            "{label}: every disc pair is decided: {discs:?}"
        );
        let walls = undecided_pairs(&body, &recorded.findings, geom::SurfaceKind::Cylinder);
        assert_eq!(
            walls.len(),
            walls_left,
            "{label}: the wall pairs left: {walls:?}"
        );
    }
}

/// **Two copies of one ball fill one volume**, spun or not, and so does a
/// ball with a cap cut off inside it: its sphere face is a proper part of
/// the ball's. Each is one `SameSideInterference`, read off the sphere
/// faces on two keys. Red if the curved arm stops reading spheres.
#[test]
fn copies_of_one_ball_are_one_interference() {
    let ball = ball();
    for (label, b) in [
        ("coincident", placed(&ball, spun(0.0))),
        ("spun", placed(&ball, spun(2.3))),
        ("cut", placed(&cut_ball(), spun(0.9))),
    ] {
        let body = two(&placed(&ball, spun(0.0)), &b);
        assert_one_same_side(label, &body);
    }
}

/// **Copies that only abut are no interference.** A cylinder stacked on
/// the base along its axis: the walls lie on one carrier facing the same
/// way and share only the rim, so their cones there lie on either side
/// of it. Two half-cylinders of one carrier side by side, each the
/// other's half-turn: their walls share only the two seam rulings and
/// their half-discs only a diameter. Neither is a `SameSideInterference`
/// or any other interference; the half-cylinders' walls and discs are
/// read no further, so the census stays loud about the pair (the
/// same-side reading on an arc or a curved carrier only ever finds a
/// meeting, and cannot say there is none).
///
/// Red if the same-side reading takes a shared rim or seam for shared
/// interior.
#[test]
fn copies_that_abut_along_a_rim_or_a_seam_are_no_interference() {
    let base = placed(&cylinder(1.0, Revolution::Full), spun(0.0));
    let stacked = topo::transform_rigid(
        &cylinder(1.0, Revolution::Full),
        &tilted()
            .compose(RigidMotion::translation(Vec3::new(0.0, 1.0, 0.0)).unwrap())
            .unwrap(),
        Tol::witness(),
    )
    .unwrap();
    let wedge = cylinder(1.0, Revolution::Partial(core::f64::consts::PI));
    let halves = two(
        &placed(&wedge, spun(0.0)),
        &placed(&wedge, RigidMotion::half_turn_y()),
    );
    for (label, body, loud) in [
        ("stacked", two(&base, &stacked), false),
        ("halves", halves, true),
    ] {
        let (recorded, flush) = at_rest(&body);
        assert!(
            interference(&body, &recorded.findings).is_empty(),
            "{label}: no interference: {:?}",
            recorded.findings
        );
        assert!(
            !flush
                .findings
                .iter()
                .chain(&recorded.findings)
                .any(|f| matches!(f, ValidationError::SameSideInterference { .. })),
            "{label}: no same-side pair: {:?} {:?}",
            recorded.findings,
            flush.findings
        );
        if loud {
            assert!(
                recorded.findings.iter().any(|f| matches!(
                    f,
                    ValidationError::CensusUndecidable {
                        reason: topo::Undecidable::MayOverlap,
                        ..
                    }
                )),
                "{label}: the walls' shared seams stay undecidable: {:?}",
                recorded.findings
            );
        }
    }
}

/// **Copies apart within the band are no interference.** A copy of the
/// cylinder and of the ball moved off the other across the axis, by a
/// distance mid-way between the band's zero edge and its escalation
/// edge: Door 1 reads the carriers in band, so the
/// curved arm does not read them as one, and no `SameSideInterference`
/// is decided off a reading in the band. Red if the curved arm or the
/// disc reading runs on an in-band carrier.
#[test]
fn copies_apart_within_the_band_are_no_interference() {
    let (eps, k) = (Tol::witness().eps(), Tol::witness().k());
    let d = 0.5 * (1.0 + k) * eps;
    let off = RigidMotion::translation(Vec3::new(0.8 * d, 0.0, 0.6 * d)).unwrap();
    for (label, body) in [
        ("cylinder", cylinder(1.0, Revolution::Full)),
        ("ball", ball()),
    ] {
        let both = two(&placed(&body, spun(0.0)), &placed(&body, off));
        let (recorded, flush) = at_rest(&both);
        assert!(
            !flush
                .findings
                .iter()
                .chain(&recorded.findings)
                .any(|f| matches!(f, ValidationError::SameSideInterference { .. })),
            "{label}: no same-side pair off an in-band reading: {:?}",
            recorded.findings
        );
    }
}
