//! **The football revolves.** An arc of a spindle-torus sheet's circle
//! from apex to apex, closed by its chord on the axis and revolved about
//! that chord: the lemon (`R < 0`), whose apexes are vertices, and the
//! apple (`0 < R < r`), whose apexes are dimples. The horn's cusp
//! (`R = r`) is reached from one side as a needle tip.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::validated;
use core::f64::consts::PI;
use geom_core::{Point2, Tol, Vec2};
use profile::test_support::bulge_loop;
use sweep::{Revolution, RevolveAxis, revolve};

fn y_axis() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// The football of the sheet `(R, r)`: its arc from the lower apex to
/// the upper, through the circle's widest point, closed down the axis.
fn football(big: f64, r: f64) -> profile::ProfileLoop<f64> {
    let h = (r * r - big * big).sqrt();
    let v_star = (-big / r).acos();
    bulge_loop(vec![
        (Point2::new(0.0, -h), (v_star / 2.0).tan()),
        (Point2::new(0.0, h), 0.0),
    ])
}

#[test]
fn probe_the_football() {
    let tol = Tol::witness();
    let needle = bulge_loop(vec![
        (Point2::new(0.0, 0.0), -(PI / 8.0).tan()),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let pinch = bulge_loop(vec![
        (Point2::new(0.0, 0.0), -(PI / 8.0).tan()),
        (Point2::new(1.0, 1.0), -(PI / 8.0).tan()),
        (Point2::new(2.0, 0.0), -(PI / 8.0).tan()),
        (Point2::new(1.0, -1.0), -(PI / 8.0).tan()),
    ]);
    for (what, lp, how) in [
        ("the lemon", football(-0.5, 1.0), Revolution::Full),
        ("the apple", football(0.4, 1.0), Revolution::Full),
        ("the needle", needle, Revolution::Full),
        ("the pinch", pinch, Revolution::Full),
        (
            "the lemon part way",
            football(-0.5, 1.0),
            Revolution::Partial(2.0),
        ),
        (
            "the apple part way",
            football(0.4, 1.0),
            Revolution::Partial(4.0),
        ),
    ] {
        let built = match revolve(&validated(vec![lp]), y_axis(), how, tol) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("{what}: revolve: {e}");
                continue;
            }
        };
        match mesh::tessellate(&built.body, 1e-2, tol) {
            Ok(m) => eprintln!("{what}: mesh ok {:?}", mesh::validate::check_mesh(&m).err()),
            Err(e) => eprintln!("{what}: tessellate: {e:?}"),
        }
        let body = match topo::AtRestBody::validate(built.body, tol) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("{what}: tier 3: {e:?}");
                continue;
            }
        };
        match topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol)) {
            Ok(p) => eprintln!(
                "{what}: volume {} area {}",
                p.enclosure.volume, p.enclosure.surface_area
            ),
            Err(e) => eprintln!("{what}: props: {e:?}"),
        }
    }
    let _ = PI;
}
