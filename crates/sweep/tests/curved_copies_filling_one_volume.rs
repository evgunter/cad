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

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Band, Point2, Point3, RigidMotion, Tol, UnitVec3, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, ContactRecords, EntityId, ValidationError};

/// The rectangle `[r0, r1] × [y0, y1]` of the sketch, revolved about `y`
/// by `turn`: a cylinder (`r0 = 0`) or a tube, or a wedge of either.
fn rect(r0: f64, r1: f64, y0: f64, y1: f64, turn: Revolution<f64>) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(r0, y0), 0.0),
            (Point2::new(r1, y0), 0.0),
            (Point2::new(r1, y1), 0.0),
            (Point2::new(r0, y1), 0.0),
        ],
        turn,
        Tol::witness(),
    )
}

/// The unit square over `[0, h]`, revolved about `y` by `turn`: a
/// cylinder of radius 1 and height `h`, its walls two half-bands on one
/// key and its caps two discs, or a wedge of it.
fn cylinder(h: f64, turn: Revolution<f64>) -> Body<f64> {
    rect(0.0, 1.0, 0.0, h, turn)
}

/// The unit ball, its poles on `y`.
fn ball() -> Body<f64> {
    lune(Revolution::Full)
}

/// The unit ball revolved by `turn` only: a lune, or the ball.
fn lune(turn: Revolution<f64>) -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        turn,
        Tol::witness(),
    )
}

/// The unit ball's half below `y = 0` (`upper` false) or above it.
fn hemisphere(upper: bool) -> Body<f64> {
    let bulge = (PI / 8.0).tan();
    let profile = if upper {
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), bulge),
            (Point2::new(0.0, 1.0), 0.0),
        ]
    } else {
        vec![
            (Point2::new(0.0, -1.0), bulge),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 0.0), 0.0),
        ]
    };
    revolved_about_y(profile, Revolution::Full, Tol::witness())
}

/// The unit ball's cap above `y = 1/2`: its sphere face is the
/// complement on the sphere of [`cut_ball`]'s, meeting it along the rim
/// circle only.
fn upper_cap() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.5), 0.0),
            (Point2::new(0.75_f64.sqrt(), 0.5), (PI / 12.0).tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

/// The ball of radius 2 with the unit ball hollowed out of it.
fn shell() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, -2.0), 1.0),
            (Point2::new(0.0, 2.0), 0.0),
            (Point2::new(0.0, 1.0), -1.0),
            (Point2::new(0.0, -1.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

/// `body` reflected across the plane through the origin normal to
/// `normal`.
fn mirrored(body: &Body<f64>, normal: Vec3<f64>) -> Body<f64> {
    let r = topo::Reflection::across(Point3::new(0.0, 0.0, 0.0), normal, Tol::witness()).unwrap();
    topo::mirror_body(body, &r, Tol::witness()).unwrap()
}

/// A move by `v`.
fn moved(v: Vec3<f64>) -> RigidMotion<f64> {
    RigidMotion::translation(v).unwrap()
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

/// The faces of each flush row.
fn flush_faces(flush: &topo::FlushAtRest) -> Vec<[topo::FaceKey; 2]> {
    flush
        .rows
        .iter()
        .map(|row| {
            row.cells.map(|cell| match cell {
                topo::RowCell::Result {
                    cell: topo::Cell::Face(f),
                } => f,
                other => panic!("a flush row names two faces: {other:?}"),
            })
        })
        .collect()
}

/// That `body`'s two solids read as one `SameSideInterference` and no
/// other interference, with no unrecorded touch left beside it (a
/// coincident vertex, edge or rim is a touch in the closure of the
/// pair's shared area, which the pair answers for), and that `flush`
/// face pairs lie flush, each sharing only boundary, none of them the
/// interfering pair.
fn assert_one_same_side(label: &str, body: &Body<f64>, flush_rows: usize) -> topo::RecordedAtRest {
    let (recorded, flush) = at_rest(body);
    let [ValidationError::SameSideInterference { a, b }] =
        interference(body, &recorded.findings)[..]
    else {
        panic!(
            "{label}: one same-side interference: {:?}",
            recorded.findings
        );
    };
    let flushed = flush_faces(&flush);
    assert_eq!(
        flushed.len(),
        flush_rows,
        "{label}: the flush pairs: {flushed:?}"
    );
    assert!(
        !flushed.iter().any(|p| p.contains(a) && p.contains(b)),
        "{label}: the overlapping pair is not flush: {flushed:?}"
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
        recorded.rows.is_empty() && flush.findings.is_empty(),
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
/// No pair of discs or of walls stays undecidable: each pair whose
/// interiors meet is decided, and in the coincident copies the
/// half-bands that share a seam ruling and no interior (one of each
/// copy, a turn apart) are certified apart, two flush pairs. The spun
/// copy's half-bands each share interior with both of the other's, so no
/// wall pair is flush. A disc against the other copy's wall, two
/// carriers meeting along the rim, is the proximity arm's, and stays
/// undecidable.
///
/// Red if the arc-bounded disc pair goes back to the region door's
/// refusal, the curved arm stops reading two keys, a same-side pair is
/// still refused by the proximity arm beside its decided overlap, or the
/// half-bands sharing a seam are not read flush.
#[test]
fn copies_of_one_cylinder_are_one_interference() {
    let base = cylinder(1.0, Revolution::Full);
    let half = cylinder(0.5, Revolution::Full);
    for (label, b, flush) in [
        ("coincident", placed(&base, spun(0.0)), 2),
        ("spun", placed(&base, spun(1.1)), 0),
        ("half height", placed(&half, spun(0.4)), 0),
    ] {
        let body = two(&placed(&base, spun(0.0)), &b);
        let recorded = assert_one_same_side(label, &body, flush);
        let discs = undecided_pairs(&body, &recorded.findings, geom::SurfaceKind::Plane);
        assert!(
            discs.is_empty(),
            "{label}: every disc pair is decided: {discs:?}"
        );
        let walls = undecided_pairs(&body, &recorded.findings, geom::SurfaceKind::Cylinder);
        assert!(
            walls.is_empty(),
            "{label}: every wall pair is decided: {walls:?}"
        );
    }
}

/// **Two copies of one ball fill one volume**, spun or not, and so does a
/// ball with a cap cut off inside it: its sphere face is a proper part of
/// the ball's. Each is one `SameSideInterference`, read off the sphere
/// faces on two keys; the coincident copies' halves that share only a
/// seam meridian are flush. Red if the curved arm stops reading spheres.
#[test]
fn copies_of_one_ball_are_one_interference() {
    let ball = ball();
    for (label, b, flush) in [
        ("coincident", placed(&ball, spun(0.0)), 2),
        ("spun", placed(&ball, spun(2.3)), 0),
        ("cut", placed(&cut_ball(), spun(0.9)), 0),
    ] {
        let body = two(&placed(&ball, spun(0.0)), &b);
        assert_one_same_side(label, &body, flush);
    }
}

/// **Copies that only abut are no interference, and lie flush.** A
/// cylinder stacked on the base along its axis: the walls lie on one
/// carrier facing the same way and share only the rim, so their cones
/// there lie on either side of it. Two half-cylinders of one carrier side
/// by side, each the other's half-turn: their walls share only the two
/// seam rulings and their half-discs only a diameter. Neither is a
/// `SameSideInterference` or any other interference, and each pair
/// sharing boundary and no interior is a flush row: the stacked walls'
/// half-bands over the rim, and the halves' walls and two half-disc
/// pairs. A disc against the other copy's wall, two carriers meeting
/// along the rim, is the proximity arm's, and stays undecidable.
///
/// Red if the same-side reading takes a shared rim or seam for shared
/// interior, or cannot certify that interiors sharing one do not meet.
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
    use geom::SurfaceKind::{Cylinder, Plane};
    for (label, body, flush_kinds) in [
        ("stacked", two(&base, &stacked), vec![Cylinder, Cylinder]),
        ("halves", halves, vec![Plane, Plane, Cylinder]),
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
        assert!(
            flush.findings.is_empty(),
            "{label}: every flush pair is read: {:?}",
            flush.findings
        );
        let mut kinds: Vec<_> = flush_faces(&flush)
            .iter()
            .map(|&[a, b]| {
                let (ka, kb) = (
                    kind(&body, EntityId::Face(a)),
                    kind(&body, EntityId::Face(b)),
                );
                assert_eq!(ka, kb, "{label}: a flush pair lies on one carrier");
                ka.unwrap()
            })
            .collect();
        kinds.sort_by_key(|k| format!("{k:?}"));
        let mut want = flush_kinds.clone();
        want.sort_by_key(|k| format!("{k:?}"));
        assert_eq!(kinds, want, "{label}: the flush pairs");
        let walls = undecided_pairs(&body, &recorded.findings, Cylinder);
        assert!(
            walls.is_empty(),
            "{label}: the walls are decided: {walls:?}"
        );
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

/// The same-side findings over `body`, at rest and flush, and its flush
/// rows.
fn same_sides(body: &Body<f64>) -> (usize, usize, String) {
    let (recorded, flush) = at_rest(body);
    let all: Vec<_> = recorded.findings.iter().chain(&flush.findings).collect();
    let n = all
        .iter()
        .filter(|f| matches!(f, ValidationError::SameSideInterference { .. }))
        .count();
    let interfering = interference(body, &recorded.findings).len();
    (n.max(interfering), flush.rows.len(), format!("{all:?}"))
}

/// **Faces of one carrier that only abut are no interference**, on
/// spheres and cylinders alike (the review probes of the curved arm,
/// ported). Each pair below shares boundary on one carrier facing the
/// same way, or lies on one carrier facing opposite ways, and the two
/// solids share no material:
///
/// - two hemispheres on the equator, spun apart, and a ball's cut-off
///   cap against the rest of it: they share a circle;
/// - two lunes side by side, a reflex lune against its complement, and
///   quarter balls opposite each other: they share meridians, or only
///   the poles;
/// - a reflex wedge of the cylinder against its complement, quarter
///   cylinders opposite each other, and a half cylinder against its
///   mirror image: they share rulings, or only the axis;
/// - two tubes end to end: they share a rim;
/// - a rod in a tube, a shaft in its bore and a ball in a shell's cavity:
///   walls on one carrier, the senses opposed; the rod's and the shaft's
///   end discs lie flush with the tube's end annuli, sharing a circle.
///
/// None is a `SameSideInterference` or any other interference, and each
/// aligned pair sharing a run of boundary is a flush row. Red if the
/// witness search takes a shared circle, meridian, ruling or pole for
/// shared interior, the curved arm reads an opposed pair, or an aligned
/// pair sharing only boundary is not certified apart.
#[test]
fn faces_of_one_carrier_that_only_abut_are_no_interference() {
    let half = Revolution::Partial(PI);
    let quarter = Revolution::Partial(FRAC_PI_2);
    let reflex = Revolution::Partial(1.5 * PI);
    let rest = |b: &Body<f64>, at| placed(b, spun(at));
    let wedge = cylinder(1.0, half);
    for (label, body) in [
        (
            "hemispheres on the equator",
            two(
                &rest(&hemisphere(false), 0.0),
                &rest(&hemisphere(true), 0.3),
            ),
        ),
        (
            "cap and cut ball",
            two(&rest(&cut_ball(), 0.0), &rest(&upper_cap(), 0.9)),
        ),
        (
            "lunes side by side",
            two(&rest(&lune(half), 0.0), &rest(&lune(half), PI)),
        ),
        (
            "reflex lune and its complement",
            two(&rest(&lune(reflex), 0.0), &rest(&lune(quarter), 1.5 * PI)),
        ),
        (
            "quarter balls at the poles",
            two(&rest(&lune(quarter), 0.0), &rest(&lune(quarter), PI)),
        ),
        (
            "reflex wedge and its complement",
            two(
                &rest(&cylinder(1.0, reflex), 0.0),
                &rest(&cylinder(1.0, quarter), 1.5 * PI),
            ),
        ),
        (
            "quarter cylinders on the axis",
            two(
                &rest(&cylinder(1.0, quarter), 0.0),
                &rest(&cylinder(1.0, quarter), PI),
            ),
        ),
        (
            "half cylinder and its mirror",
            two(
                &rest(&wedge, 0.0),
                &rest(&mirrored(&wedge, Vec3::new(0.0, 0.0, 1.0)), 0.0),
            ),
        ),
        (
            "tubes end to end",
            two(
                &rest(&rect(0.5, 1.0, 0.0, 1.0, Revolution::Full), 0.0),
                &rest(&rect(0.5, 1.0, 1.0, 2.0, Revolution::Full), 0.0),
            ),
        ),
        (
            "rod in a tube",
            two(
                &rest(&cylinder(1.0, Revolution::Full), 0.0),
                &rest(&rect(1.0, 2.0, 0.0, 1.0, Revolution::Full), 0.7),
            ),
        ),
        (
            "shaft in its bore",
            two(
                &rest(&rect(0.0, 0.5, 0.0, 1.0, Revolution::Full), 0.0),
                &rest(&rect(0.5, 1.0, 0.0, 1.0, Revolution::Full), 0.4),
            ),
        ),
        (
            "ball in a shell",
            two(&rest(&ball(), 0.0), &rest(&shell(), 0.0)),
        ),
    ] {
        let (n, rows, all) = same_sides(&body);
        assert_eq!(n, 0, "{label}: no interference: {all}");
        // Each face pair sharing a run of boundary on one carrier facing
        // the same way: a hemisphere's two halves against the other's
        // two, along the equator; the lunes' and wedges' walls along a
        // meridian or ruling, the wedges' discs along a radius; the
        // tubes' walls and bores along a rim; the rod's and the shaft's
        // end discs against the tube's end annuli, along the circle they
        // share. Pairs meeting at a pole or on the axis share no run, and
        // opposed pairs (the walls about a rod or a shaft) are no flush
        // pair.
        let flush = match label {
            "hemispheres on the equator" | "cap and cut ball" => 4,
            "lunes side by side" | "reflex lune and its complement" => 1,
            "reflex wedge and its complement" | "half cylinder and its mirror" => 3,
            "tubes end to end" | "rod in a tube" | "shaft in its bore" => 2,
            _ => 0,
        };
        assert_eq!(rows, flush, "{label}: the flush pairs: {all}");
    }
}

/// **Two halves of one cylinder side by side, the second's radius or
/// axis off by `k·ε`**, across the band. Off in radius, or moved away
/// from the first, they share no material at any `k`: no interference.
/// Moved into the first by `100·ε` or more, they share a sliver past the
/// band, one `SameSideInterference`; by `3·ε` or less, the sliver is
/// within the band, which no reading decides. Red if the curved arm or
/// the disc reading decides an overlap off a reading in the band, or
/// misses a sliver past it.
#[test]
fn halves_off_by_a_few_eps_are_decided_past_the_band_only() {
    let eps = Tol::witness().eps();
    let half = |r: f64| rect(0.0, r, 0.0, 1.0, Revolution::Partial(PI));
    let a = placed(&half(1.0), spun(0.0));
    let shifted = |dz: f64| {
        let b = topo::transform_rigid(&half(1.0), &moved(Vec3::new(0.0, 0.0, dz)), Tol::witness());
        placed(&b.unwrap(), spun(PI))
    };
    for k in [0.5, 3.0, 100.0, 1000.0] {
        let d = k * eps;
        for (label, b, overlaps) in [
            ("radius", placed(&half(1.0 + d), spun(PI)), false),
            ("into", shifted(d), k >= 100.0),
            ("away", shifted(-d), false),
        ] {
            let (n, _, all) = same_sides(&two(&a, &b));
            assert_eq!(n, usize::from(overlaps), "{label} by {k} eps: {all}");
        }
    }
}

/// **Copies that overlap on one carrier are one `SameSideInterference`**
/// however the copy is placed (the review's positive probes, ported): a
/// mirror image of the cylinder, a copy flipped end over end, half
/// cylinders and lunes a quarter turn apart, a copy overlapping the base
/// by half its height, two coincident tubes (whose bores face inward),
/// and a hemisphere inside the ball. Red if the curved arm misses an
/// overlap the copy's pose hides from it.
#[test]
fn copies_overlapping_on_one_carrier_are_one_interference() {
    let full = cylinder(1.0, Revolution::Full);
    let base = placed(&full, spun(0.0));
    let half = Revolution::Partial(PI);
    let posed = |b: &Body<f64>, inner: RigidMotion<f64>| {
        topo::transform_rigid(b, &tilted().compose(inner).unwrap(), Tol::witness()).unwrap()
    };
    let flipped = moved(Vec3::new(0.0, 1.0, 0.0))
        .compose(RigidMotion::rotation(UnitVec3::exact_x(), PI).unwrap())
        .unwrap();
    let raised = moved(Vec3::new(0.0, 0.5, 0.0)).compose(spun(0.3)).unwrap();
    let tube = rect(0.5, 1.0, 0.0, 1.0, Revolution::Full);
    for (label, a, b) in [
        (
            "mirrored",
            base.clone(),
            placed(&mirrored(&full, Vec3::new(0.0, 0.0, 1.0)), spun(0.0)),
        ),
        ("flipped", base.clone(), posed(&full, flipped)),
        ("raised", base.clone(), posed(&full, raised)),
        (
            "half cylinders",
            placed(&cylinder(1.0, half), spun(0.0)),
            placed(&cylinder(1.0, half), spun(FRAC_PI_2)),
        ),
        (
            "lunes",
            placed(&lune(half), spun(0.0)),
            placed(&lune(half), spun(FRAC_PI_2)),
        ),
        ("tubes", placed(&tube, spun(0.0)), placed(&tube, spun(0.7))),
        (
            "hemisphere in the ball",
            placed(&ball(), spun(0.0)),
            placed(&hemisphere(true), spun(0.5)),
        ),
    ] {
        let body = two(&a, &b);
        let (recorded, _) = at_rest(&body);
        assert!(
            matches!(
                interference(&body, &recorded.findings)[..],
                [ValidationError::SameSideInterference { .. }]
            ),
            "{label}: one same-side interference: {:?}",
            recorded.findings
        );
    }
}

/// **What the curved aligned arm costs on pins stacked end to end**: a
/// three-by-three grid of stacks of four pins, every stacked pair one
/// carrier facing the same way and sharing a rim. Each pair's walls
/// span the axis apart, so none reaches the witness search
/// (`CensusTrace::aligned`), while two coincident copies' walls do and
/// are found. Red if the axial reject stops dropping abutting stacks
/// before Door 1 and the search, the cost the review measured.
#[test]
fn pins_stacked_end_to_end_reach_no_witness_search() {
    let traced = |body: &Body<f64>| {
        topo::census_traces(
            body,
            &ContactRecords::default(),
            Band::linear(Tol::witness()).unwrap(),
            Tol::witness(),
            Some(topo::RegionLane::certified()),
            topo::CensusStrategy::Realized,
        )
        .1
        .aligned
    };
    let pin = rect(0.0, 0.4, 0.0, 1.0, Revolution::Full);
    let mut stacks = placed(&pin, moved(Vec3::new(0.0, 0.0, 0.0)));
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..4 {
                if (i, j, k) != (0, 0, 0) {
                    let at = Vec3::new(1.5 * f64::from(i), f64::from(k), 1.5 * f64::from(j));
                    topo::graft_disjoint_all_keyed(&mut stacks, &placed(&pin, moved(at))).unwrap();
                }
            }
        }
    }
    let stacked = traced(&stacks);
    assert!(
        stacked.examined.is_empty(),
        "no stacked pair reaches the witness search: {stacked:?}"
    );
    let full = cylinder(1.0, Revolution::Full);
    let copies = traced(&two(&placed(&full, spun(0.0)), &placed(&full, spun(1.1))));
    assert!(
        !copies.examined.is_empty() && copies.accepted.len() == copies.examined.len(),
        "coincident copies' walls reach the search and are found: {copies:?}"
    );
}

/// **Copies that overlap on one carrier are never read clear, and copies
/// that only abut are never read as interference** (the review's probes,
/// ported). Each overlapping pair below shares material, so the census
/// reports an interference or leaves the pair uncleared: wedges and
/// lunes straddling a seam or a joint, reflex pieces against each other,
/// a half-height wedge sharing part of a ruling, a hemisphere against a
/// lune across the equator, a cut ball against a hemisphere, a rod
/// inside a tube's wall, a tube over a tube by half, and pins or half
/// pins overlapping by `1000·ε`. Each abutting pair shares only
/// boundary: a rod in a tube or inside its hole, pins stacked or
/// overlapping by less than the band, half pins stacked and spun, half
/// cylinders side by side, a reflex wedge and its complement. Red if a
/// flush or apart reading hides an overlap, or reads an abutment as one.
#[test]
fn overlaps_are_never_read_clear_and_abutments_never_interfere() {
    let eps = Tol::witness().eps();
    let full = Revolution::Full;
    let half = Revolution::Partial(PI);
    let quarter = Revolution::Partial(FRAC_PI_2);
    let reflex = Revolution::Partial(1.5 * PI);
    let rest = |b: &Body<f64>, at| placed(b, spun(at));
    let lift = |b: &Body<f64>, dy: f64, at: f64| {
        placed(b, moved(Vec3::new(0.0, dy, 0.0)).compose(spun(at)).unwrap())
    };
    let (pin, tube) = (cylinder(1.0, full), rect(0.5, 1.0, 0.0, 1.0, full));
    let overlapping = [
        (
            "quarter wedge over the seam",
            two(
                &rest(&pin, 0.0),
                &rest(&cylinder(1.0, quarter), -FRAC_PI_2 / 2.0),
            ),
        ),
        (
            "quarter wedge over the joint",
            two(
                &rest(&pin, 0.0),
                &rest(&cylinder(1.0, quarter), PI - FRAC_PI_2 / 2.0),
            ),
        ),
        (
            "reflex wedges a half turn apart",
            two(
                &rest(&cylinder(1.0, reflex), 0.0),
                &rest(&cylinder(1.0, reflex), PI),
            ),
        ),
        (
            "a quarter inside a reflex wedge",
            two(
                &rest(&cylinder(1.0, reflex), 0.0),
                &rest(&cylinder(1.0, quarter), 0.5),
            ),
        ),
        (
            "half-height wedge on a ruling",
            two(
                &rest(&cylinder(1.0, half), 0.0),
                &rest(&cylinder(0.5, quarter), 0.0),
            ),
        ),
        (
            "reflex lune and a half lune",
            two(&rest(&lune(reflex), 0.0), &rest(&lune(half), PI)),
        ),
        (
            "reflex lunes a half turn apart",
            two(&rest(&lune(reflex), 0.0), &rest(&lune(reflex), PI)),
        ),
        (
            "hemisphere and a lune",
            two(&rest(&hemisphere(true), 0.0), &rest(&lune(quarter), 0.4)),
        ),
        (
            "cut ball and a hemisphere",
            two(&rest(&cut_ball(), 0.0), &rest(&hemisphere(false), 0.7)),
        ),
        (
            "a rod in a tube's wall",
            two(
                &rest(&pin, 0.0),
                &rest(&rect(1.0, 2.0, 0.0, 1.0, full), 0.7),
            ),
        ),
        (
            "a tube over a tube by half",
            two(&rest(&tube, 0.0), &lift(&tube, 0.5, 0.3)),
        ),
        (
            "pins over by 1000 eps",
            two(&rest(&pin, 0.0), &lift(&pin, 1.0 - 1000.0 * eps, 0.3)),
        ),
        (
            "half pins over by 1000 eps",
            two(
                &rest(&cylinder(1.0, half), 0.0),
                &lift(&cylinder(1.0, half), 1.0 - 1000.0 * eps, 0.0),
            ),
        ),
    ];
    for (label, body) in &overlapping {
        let (recorded, _) = at_rest(body);
        let held = recorded
            .findings
            .iter()
            .any(|f| topo::census_uncleared_solids(body, f).is_some());
        assert!(
            held,
            "{label}: an overlap is never clear: {:?}",
            recorded.findings
        );
    }
    let abutting = [
        (
            "rod in a tube",
            two(
                &rest(&pin, 0.0),
                &rest(&rect(1.0, 2.0, 0.0, 1.0, full), 0.7),
            ),
        ),
        (
            "rod in a tube's hole",
            two(
                &rest(&rect(0.0, 0.5, 0.0, 1.0, full), 0.0),
                &rest(&rect(1.0, 2.0, 0.0, 1.0, full), 0.7),
            ),
        ),
        ("pins stacked", two(&rest(&pin, 0.0), &lift(&pin, 1.0, 0.3))),
        (
            "pins over by 0.5 eps",
            two(&rest(&pin, 0.0), &lift(&pin, 1.0 - 0.5 * eps, 0.3)),
        ),
        (
            "pins over by 3 eps",
            two(&rest(&pin, 0.0), &lift(&pin, 1.0 - 3.0 * eps, 0.3)),
        ),
        (
            "half pins stacked, spun",
            two(
                &rest(&cylinder(1.0, half), 0.0),
                &lift(&cylinder(1.0, half), 1.0, 1.0),
            ),
        ),
        (
            "half cylinders side by side",
            two(
                &rest(&cylinder(1.0, half), 0.0),
                &rest(&cylinder(1.0, half), PI),
            ),
        ),
        (
            "reflex wedge and complement",
            two(
                &rest(&cylinder(1.0, reflex), 0.0),
                &rest(&cylinder(1.0, quarter), 1.5 * PI),
            ),
        ),
    ];
    for (label, body) in &abutting {
        let (recorded, _) = at_rest(body);
        assert!(
            interference(body, &recorded.findings).is_empty(),
            "{label}: an abutment is no interference: {:?}",
            recorded.findings
        );
    }
}
