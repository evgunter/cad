//! **The side reader's aims and crossings ask nothing the geometry does
//! not** (`work/flush/the-side-readers-aims-decide-exact-ties-at-a-documents-nominal.md`):
//! on a rod (a disc extruded, its caps bounded by two arcs) and a plate
//! drilled through by one, the faces' aimed rays read the solid's side
//! (an aim in the drill's hole crosses nothing, and reads nothing)
//! without an in-face walk of a cap, a region walk of a wall or a
//! certificate of a curved aim onto its surface — the questions whose
//! margins tie exactly at a symmetric nominal. A trimmed sphere's aim
//! (a lens of two balls) certifies nothing onto its sphere either.
//!
//! A curved aim's on-carrier foot is decided outside the log
//! (`Azimuth::near`, audit row F31), so the verdict log cannot see it;
//! its samples reach the `probe` sink, and the rod's aims at `Probe` take
//! none of them.

#![allow(clippy::expect_used)]

use geom_core::k_stats::Bracket;
use geom_core::{Band, Point2, Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished, prism_at};
use topo::{AtRestBody, ContactRecords};

/// The predicates no aimed ray at these bodies may take: a cap's
/// in-face walk, a wall's region walk, a curved aim's on-chart
/// certificate.
const WALKS: [&str; 4] = [
    "point_in_loop",
    "point_in_arc_loop",
    "bool_ruled_region",
    "bool_pierce_normal_on",
];

/// Every face's aimed ray's reading of `body`, and the predicates they
/// decided.
fn aims(body: &AtRestBody<f64>, band: Band) -> (Vec<String>, Vec<&'static str>) {
    let bracket = Bracket::open();
    let read = topo::test_support::each_aim(body, band);
    let log = bracket.finish();
    let predicates = log
        .verdicts
        .iter()
        .map(|v| v.predicate)
        .chain(log.escalations.iter().filter_map(|e| e.source.predicate))
        .collect();
    (read, predicates)
}

#[test]
fn a_rod_and_a_drilled_plate_read_their_side_by_turns_alone() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let rod = |z0: f64, h: f64| {
        let disc = vec![
            (Point2::new(0.25, 0.0), 1.0),
            (Point2::new(-0.25, 0.0), 1.0),
        ];
        finished("rod", prism_at(disc, z0, h, tol), tol)
    };
    let plate = finished(
        "plate",
        brick::<f64>((-1.0, 1.0), (-1.0, 1.0), (0.0, 0.5), tol),
        tol,
    );
    let drilled = topo::subtract(&plate, &rod(-0.5, 1.5), tol)
        .expect("the drill")
        .body()
        .expect("a drilled plate")
        .body
        .clone();
    let ball = |c: f64| {
        let b = finished(
            "ball",
            ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), tol),
            tol,
        );
        let m = topo::transform_rigid(
            &b,
            &geom_core::RigidMotion::translation(Vec3::new(0.0, 0.0, c)).expect("a translation"),
            tol,
        )
        .expect("the move");
        finished("moved ball", m, tol)
    };
    let lens = topo::intersect(&ball(-0.5), &ball(0.5), tol)
        .expect("the lens")
        .body()
        .expect("a lens")
        .body
        .clone();
    for (what, body, walks) in [
        ("rod", rod(0.0, 1.0), &WALKS[..]),
        ("drilled plate", drilled, &WALKS[..]),
        ("lens", lens, &WALKS[3..]),
    ] {
        assert!(
            topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol).is_ok(),
            "{what}: the body is at rest with no contacts"
        );
        let (read, predicates) = aims(&body, band);
        assert!(
            read.iter().any(|r| r.ends_with(": Ok(Side(Out))"))
                && read.iter().all(|r| !r.contains("Side(In")),
            "{what}: the aims read the outside, and none the inside: {read:#?}"
        );
        assert!(
            predicates.iter().any(|p| p.starts_with("side_at_infinity")),
            "{what}: the aims cast rays"
        );
        let asked: Vec<_> = predicates
            .iter()
            .filter(|p| walks.iter().any(|w| p.starts_with(w)))
            .collect();
        assert!(asked.is_empty(), "{what}: the aims ask {asked:?}");
    }
}

/// The names a circle foot's samples reach the `probe` sink under
/// (`Azimuth::near`, audit row F31): its window's cut side and the
/// period's half-turn marks.
#[cfg(feature = "probe")]
const FOOT: [&str; 2] = ["azimuth_near_cut_side", "period_near_mark"];

/// **No aim at the rod's walls takes a circle foot**: every face's aimed
/// ray, at `Probe`, records no sample under [`FOOT`]. A point inside the
/// rod, classified against it, does: its rays' wall crossings read their
/// azimuth, so the sink sees the names on this body.
#[cfg(feature = "probe")]
#[test]
fn the_rods_aims_take_no_circle_foot() {
    use geom_core::k_stats;
    use geom_core::{Point3, Probe};
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let p = Probe;
    let disc = vec![
        (Point2::new(p(0.25), p(0.0)), p(1.0)),
        (Point2::new(p(-0.25), p(0.0)), p(1.0)),
    ];
    let rod = finished("rod", prism_at(disc, p(0.0), p(1.0), tol), tol);
    let foot = |samples: Vec<k_stats::MarginSample>| {
        samples
            .into_iter()
            .filter(|s| FOOT.contains(&s.predicate))
            .count()
    };
    k_stats::start_recording();
    let read = topo::test_support::each_aim(&rod, band);
    let taken = foot(k_stats::take_samples());
    assert!(
        read.iter().any(|r| r.ends_with(": Ok(Side(Out))")),
        "the aims read the outside: {read:#?}"
    );
    assert_eq!(taken, 0, "the aims take a circle foot: {read:#?}");
    k_stats::start_recording();
    let inside = topo::point_in_solid(&rod, Point3::new(p(0.0), p(0.0), p(0.5)), band, tol);
    let witnessed = foot(k_stats::take_samples());
    assert!(
        matches!(inside, Ok(topo::SolidContainment::In)) && witnessed > 0,
        "the rod's axis point classifies In and takes a circle foot, so the sink sees the \
         names: {inside:?}, {witnessed} samples"
    );
}
