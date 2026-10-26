//! **A boolean on a sphere octant whose pole is a vertex measures its
//! closed form.** The section certificate never asks W2 of a sphere face (no
//! arm marks a component essential on a sphere), so the octant's pairs
//! clear by a lone evented component or a witness outside a face, and
//! the pole vertex costs them nothing. A regression row on the
//! pole-vertex boolean, not on W2.
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

/// The volume of a boolean's body.
fn volume(r: Result<BooleanResult<f64>, topo::BooleanError>, what: &str) -> f64 {
    match r {
        Ok(BooleanResult::Body(b)) => {
            topo::test_support::pinned_volume("the measured body", &b.body, tol()).volume
        }
        other => panic!("{what}: {other:?}"),
    }
}

/// The octant less, and with, a slab that cuts it along the latitude
/// `y = 0.99` near its pole, and a box wholly inside it: each a body of
/// the closed-form volume. The octant is `π/6`; the slab takes a quarter
/// of the polar cap of height `h = 0.01`, `π h² (3 − h) / 3`; the box is
/// `0.2³`.
#[test]
fn a_boolean_on_a_pole_touching_octant_measures_its_closed_form() {
    let o = finished("the octant", octant(), tol());
    let octant_volume = core::f64::consts::PI / 6.0;
    let h = 0.01;
    let cap = core::f64::consts::PI * h * h * (3.0 - h) / 3.0 / 4.0;
    let slab = 4.0 * 1.01 * 4.0;
    let inner = 0.2f64.powi(3);
    for (name, b, less, with) in [
        (
            "the polar slab",
            brick::<f64>((-2.0, 2.0), (0.99, 2.0), (-2.0, 2.0), tol()),
            octant_volume - cap,
            slab + octant_volume - cap,
        ),
        (
            "the inner box",
            brick::<f64>((0.1, 0.3), (0.1, 0.3), (-0.3, -0.1), tol()),
            octant_volume - inner,
            octant_volume,
        ),
    ] {
        let b = finished(name, b, tol());
        for (op, r, want) in [
            ("∖", subtract(&o, &b, tol()), less),
            ("∪", union(&o, &b, tol()), with),
        ] {
            let what = format!("the octant {op} {name}");
            let got = volume(r, &what);
            assert!(
                (got - want).abs() <= 1e-6 * want,
                "{what}: volume {got}, closed form {want}"
            );
        }
    }
}
