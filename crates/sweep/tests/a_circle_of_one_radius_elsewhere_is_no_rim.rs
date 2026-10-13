//! **A circle of the same radius elsewhere is no rim.** The glue
//! door's pre-gate asks whether two curved faces share a boundary
//! circle (`rim_wedge::shared_rim`), reading each pair of circles by
//! the identity ladder's rule: a datum decided apart answers no, so a
//! radius in band of another's never hides centres decided far apart.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::revolved_about_y;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn at_rest(b: topo::Body<f64>) -> topo::AtRestBody<f64> {
    topo::AtRestBody::validate(b, tol()).unwrap()
}

/// A full turn about the `y` axis from radius `inner` (0 for a rod)
/// to `outer` over `y ∈ [lo, hi]`, its axis moved to `(x, ·, 0)`.
fn about_y((inner, outer): (f64, f64), (lo, hi): (f64, f64), x: f64) -> topo::Body<f64> {
    let profile = vec![
        (Point2::new(inner, lo), 0.0),
        (Point2::new(outer, lo), 0.0),
        (Point2::new(outer, hi), 0.0),
        (Point2::new(inner, hi), 0.0),
    ];
    let body = revolved_about_y(profile, sweep::Revolution::Full, tol());
    topo::transform_rigid(
        &body,
        &geom_core::test_support::rigid(&Affine3::translation(Vec3::new(x, 0.0, 0.0)), tol()),
        tol(),
    )
    .unwrap()
}

/// Every op in both operand orders, each answer's offer key.
fn answers(x: topo::Body<f64>, y: topo::Body<f64>) -> Vec<String> {
    let (x, y) = (at_rest(x), at_rest(y));
    let mut out = Vec::new();
    for (p, q) in [(&x, &y), (&y, &x)] {
        for r in [
            topo::union(p, q, tol()).map(|_| ()),
            topo::subtract(p, q, tol()).map(|_| ()),
            topo::intersect(p, q, tol()).map(|_| ()),
        ] {
            out.push(match r {
                Ok(()) => "Ok".to_owned(),
                Err(e) => topo::test_support::offer_key(&e).0.to_string(),
            });
        }
    }
    out
}

/// A frustum (radius 0.5 at `y = 0` to 0.3 at `y = 1`) through a plate
/// whose hole has radius `0.3 + 5.5·ε`, centred 1.4 m off the frustum's
/// axis: the cone face's top circle and the hole's circle have radii in
/// band and centres far apart, and the faces cross. Every op builds,
/// as it does where the radii differ definitely, with no rim reading
/// left on the escalation log.
#[test]
fn a_frustum_through_a_plate_with_a_hole_of_its_top_radius_builds() {
    let profile = vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(0.5, 0.0), 0.0),
        (Point2::new(0.3, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ];
    for (why, delta) in [("in band", 5.5 * band().zero()), ("apart", 0.05)] {
        let frustum = revolved_about_y(profile.clone(), sweep::Revolution::Full, tol());
        let slab = at_rest(topo::test_support::brick(
            (-1.0, 2.0),
            (0.5, 0.6),
            (-1.0, 1.0),
            tol(),
        ));
        let rod = at_rest(about_y((0.0, 0.3 + delta), (0.4, 0.7), 1.4));
        let plate = topo::subtract(&slab, &rod, tol()).expect("the hole drills");
        let plate = plate.body().expect("a plate").body.clone().into_body();
        let bracket = geom_core::k_stats::Bracket::open();
        let got = answers(frustum, plate);
        let log = bracket.finish();
        assert!(got.iter().all(|r| r == "Ok"), "{why}: {got:?}");
        // The centres decide the circles apart, so the in-band radius
        // is no reading the answer rests on.
        let rim: Vec<_> = log
            .escalations
            .iter()
            .filter(|e| {
                e.source
                    .predicate
                    .is_some_and(|p| p.starts_with("rim_circle"))
            })
            .collect();
        assert!(
            rim.is_empty(),
            "{why}: every op built, yet the log holds {rim:?}"
        );
    }
}

/// Two washers crossing, their inner radii `0.3` and `0.3 + 5.5·ε`,
/// their centres apart: no op refuses on the rim identity.
#[test]
fn crossing_washers_with_radii_in_band_are_no_rim() {
    let a = about_y((0.3, 1.0), (0.0, 0.1), 0.0);
    let w = about_y((0.3 + 5.5 * band().zero(), 0.8), (-0.05, 0.05), 0.0);
    let turn = Affine3::translation(Vec3::new(0.4, 0.05, 0.1))
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), 0.5);
    let w =
        topo::transform_rigid(&w, &geom_core::test_support::rigid(&turn, tol()), tol()).unwrap();
    let got = answers(a, w);
    assert!(
        got.iter().all(|r| r != "Coincidence(Rim)"),
        "circles apart refuse on the rim: {got:?}"
    );
}
