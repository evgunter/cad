//! **A groove cut round a shaft's root continues the shaft's wall.** A
//! stepped shaft less a tube whose bore is the shaft's own cylinder: the
//! tube's bore rests on the shaft's wall below the step and runs on
//! inside the flange above it, where ∖ keeps it, reverted, as the
//! groove's inner wall. The two walls are one carrier on two records,
//! glued where they rest, so the result's faces on that cylinder end on
//! one record, each facing its material.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Surface;
use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::Body;

fn v(x: f64, y: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), 0.0)
}

fn revolved(verts: Vec<(Point2<f64>, f64)>) -> topo::AtRestBody<f64> {
    finished(
        "the revolved operand",
        revolved_about_y(verts, Revolution::Full, Tol::witness()),
        Tol::witness(),
    )
}

/// The records worn by `body`'s faces on a cylinder of radius 1.
fn shaft_wall_records(body: &Body<f64>) -> Vec<topo::SurfaceKey> {
    let mut records: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Cylinder { radius, .. }) if (*radius - 1.0).abs() < 1e-9
            )
        })
        .map(|(_, f)| f.surface)
        .collect();
    records.sort_unstable();
    records.dedup();
    records
}

#[test]
fn a_groove_at_a_shafts_root_leaves_the_shafts_wall_on_one_record() {
    let tol = Tol::witness();
    // A shaft of radius 1 over y ∈ [0, 1] under a flange of radius 2
    // over [1, 2].
    let shaft = revolved(vec![
        v(0.0, 0.0),
        v(1.0, 0.0),
        v(1.0, 1.0),
        v(2.0, 1.0),
        v(2.0, 2.0),
        v(0.0, 2.0),
    ]);
    // A tube of bore 1 and outer radius 1.5 over y ∈ [0.5, 1.5].
    let tube = revolved(vec![v(1.0, 0.5), v(1.5, 0.5), v(1.5, 1.5), v(1.0, 1.5)]);
    let out = topo::boolean::subtract(&shaft, &tube, tol).unwrap();
    let grooved = out.body().expect("the shaft is not consumed");
    topo::validate_closed(&grooved.body).expect("tier 2");
    topo::validate_pseudomanifold(&grooved.body, &grooved.contacts, tol).expect("tier 3′");
    topo::validate_geometric_certificate(&grooved.body, tol).expect("certificate");
    let volume = topo::test_support::pinned_volume("the grooved shaft", &grooved.body, tol).volume;
    let expected = PI * (1.0 + 4.0 - (1.5 * 1.5 - 1.0) * 0.5);
    assert!(
        (volume - expected).abs() <= 1e-9 * expected,
        "volume {volume} against the closed form {expected}"
    );
    assert_eq!(
        shaft_wall_records(&grooved.body).len(),
        1,
        "the shaft's wall and the groove's inner wall wear one record"
    );
}
