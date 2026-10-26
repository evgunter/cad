//! **The side reader's aims and crossings ask nothing the geometry does
//! not** (`work/flush/the-side-readers-aims-decide-exact-ties-at-a-documents-nominal.md`):
//! on a rod (a disc extruded, its caps bounded by two arcs) and a plate
//! drilled through by one, the faces' aimed rays read the solid's side
//! (an aim in the drill's hole crosses nothing, and reads nothing)
//! without an in-face walk of a cap, a region walk of a wall or a
//! certificate of a curved aim onto its surface — the questions whose
//! margins tie exactly at a symmetric nominal. A trimmed sphere's aim
//! (a lens of two balls) certifies nothing onto its sphere either.

#![allow(clippy::expect_used)]

use geom_core::k_stats::Bracket;
use geom_core::{Band, Point2, Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished, prism_at};
use topo::{AtRestBody, ContactRecords};

/// The predicates no aimed ray at these bodies may take: a cap's
/// in-face walk, a wall's region walk, a curved aim's on-chart
/// certificate.
const WALKS: [&str; 5] = [
    "point_in_loop",
    "point_in_arc_loop",
    "curve_foot_branch",
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
        ("lens", lens, &WALKS[4..]),
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
