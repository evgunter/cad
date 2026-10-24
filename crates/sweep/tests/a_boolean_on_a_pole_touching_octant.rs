//! **A boolean on a sphere octant whose pole is a vertex does not
//! refuse.** The section certificate never asks W2 of a sphere face (no
//! arm marks a component essential on a sphere), so the octant's pairs
//! clear by a lone evented component or a witness outside a face, and
//! the pole vertex costs them nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;
use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};
use topo::{BooleanResult, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

/// A quarter disc of radius 1 in the `xy` plane revolved a quarter turn
/// about `y`: its sphere face meets the pole `(0, 1, 0)` at a vertex.
fn octant() -> topo::Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), (FRAC_PI_2 / 4.0).tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Partial(FRAC_PI_2),
        tol(),
    )
}

/// The octant less, and with, a slab that cuts it along the latitude
/// `y = 0.99` near its pole and a box at its corner: each a body.
#[test]
fn a_boolean_on_a_pole_touching_octant_does_not_refuse() {
    let o = finished("the octant", octant(), tol());
    for (name, b) in [
        (
            "the polar slab",
            brick::<f64>((-2.0, 2.0), (0.99, 2.0), (-2.0, 2.0), tol()),
        ),
        (
            "the corner box",
            brick::<f64>((0.2, 1.5), (0.2, 1.5), (-1.5, -0.2), tol()),
        ),
    ] {
        let b = finished(name, b, tol());
        for (op, r) in [("∖", subtract(&o, &b, tol())), ("∪", union(&o, &b, tol()))] {
            assert!(
                matches!(r, Ok(BooleanResult::Body(_))),
                "the octant {op} {name}: {r:?}"
            );
        }
    }
}
