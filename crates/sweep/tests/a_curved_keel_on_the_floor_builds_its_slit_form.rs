//! **A curved keel resting on the floor builds its slit form.** A 270°
//! partial revolve about `y` of the slab `0.5..3 × 1..2`, less a 150°
//! partial revolve of a V whose tip lies on its floor `y = 1`, turned
//! about `y`. The keel is a circular arc of radius 2 lying in the floor,
//! and the material about it is two wedges, floor | wall a and wall b |
//! floor (`crates/topo/README.md`, tier 3), so the floor carries it as
//! two coincident arcs: turned 60°, the V lies inside the slab's 270° and
//! the arcs are a zero-area ring of the floor; turned −20°, the keel
//! leaves through the slab's end face and they are a slit of its
//! outline. The split's twin is `a_curved_keel_in_the_plane`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{BooleanResult, subtract, transform_rigid};

fn tol() -> Tol {
    Tol::witness()
}

/// **Each turn builds at its closed form and passes tier 3′ on its
/// records**: tiers 1, 3 and 3′, and one edge–edge record of the two
/// arcs, citing the parting's row of the arc lying in the floor
/// (`DecisionSite::ArcOnFace`). The volume is the slab's,
/// `0.75 π (3² − 0.5²)`, less the V's triangle in the slab (area 0.2,
/// centroid at radius 2) over the turn it holds inside the slab's: all
/// 150° turned 60°, 130° turned −20°. *Breaks if* the parting ships the
/// arcs unrecorded (3′ refuses an `EdgeEdgeOverlap`), or parts them on a
/// locus that is not the keel's (the volume moves, or tier 3 refuses).
#[test]
fn a_curved_keel_on_the_floor_builds_its_slit_form() {
    let loop_of = |pts: &[(f64, f64)]| pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
    let slab = finished(
        "the slab",
        revolved_about_y(
            loop_of(&[(0.5, 1.0), (3.0, 1.0), (3.0, 2.0), (0.5, 2.0)]),
            Revolution::Partial(1.5 * PI),
            tol(),
        ),
        tol(),
    );
    for (turn, inside) in [(60.0, 150.0f64), (-20.0, 130.0)] {
        let what = format!("the V turned {turn}°");
        let v = revolved_about_y(
            loop_of(&[(1.7, 2.5), (2.0, 1.0), (2.3, 2.5)]),
            Revolution::Partial(150f64.to_radians()),
            tol(),
        );
        let turned =
            Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), f64::to_radians(turn));
        let v = transform_rigid(&v, &geom_core::test_support::rigid(&turned, tol())).unwrap();
        let r = match subtract(&slab, &finished("the V", v, tol()), tol()) {
            Ok(BooleanResult::Body(r)) => r,
            other => panic!("{what}: {:?}, not a body", other.map(drop)),
        };
        assert_eq!(topo::validate(&r.body), Ok(()), "{what}: tier 1");
        assert_eq!(
            topo::validate_geometric(&r.body, tol()),
            Ok(()),
            "{what}: tier 3"
        );
        assert_eq!(
            topo::validate_pseudomanifold(&r.body, r.contacts(), tol()),
            Ok(()),
            "{what}: tier 3′ on {:?}",
            r.contacts()
        );
        let pairs: Vec<_> = r
            .contacts()
            .rows()
            .iter()
            .filter(|c| matches!(c.kind(), topo::ContactKind::EdgeEdge(..)))
            .collect();
        let [pair] = pairs.as_slice() else {
            panic!("{what}: the arcs' record, {:?}", r.contacts());
        };
        // The record cites the parting's reading of the arc in the floor,
        // which covers its interior, not its ends' placements.
        assert!(
            pair.cites.iter().any(|b| matches!(
                b,
                topo::Backing::Decided(k)
                    if r.coincidences[k as usize].site == topo::DecisionSite::ArcOnFace
            )),
            "{what}: {pair:?} cites no row of the arc on the floor"
        );
        let volume = 0.75 * PI * (9.0 - 0.25) - 0.2 * 2.0 * inside.to_radians();
        let got = topo::test_support::pinned_volume(
            &what,
            &finished(&what, r.body.clone(), tol()),
            tol(),
        )
        .volume;
        assert!(
            (got - volume).abs() < 1e-9 * volume,
            "{what}: volume {got}, closed form {volume}"
        );
    }
}
