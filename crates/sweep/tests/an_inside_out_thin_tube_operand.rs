//! **An inside-out thin tube never finishes at rest.** A tube `t`
//! thick, the profile `[r, r + t] × [0, 1]` turned a full turn about the
//! sketch `y` axis, reverted. At `t = 20ε` its `V/A` is in band, but
//! check 7 reads its orientation off a ray aimed at 60° at its outer
//! wall, which crosses the wall's two faces `2t` apart and enters its
//! material first: the at-rest gate refuses it inside out, so no Boolean
//! is ever handed it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{AtRestBody, Body, ValidationError};

fn tol() -> Tol {
    Tol::witness()
}

fn tube(r: f64, t: f64) -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    revolved_about_y(
        vec![p(r, 0.0), p(r + t, 0.0), p(r + t, 1.0), p(r, 1.0)],
        Revolution::Full,
        tol(),
    )
}

#[test]
fn an_inside_out_thin_tube_refuses_at_rest() {
    let t = 20.0 * tol().eps();
    for r in [0.5, 1.0, 2.0] {
        let rev = tube(r, t).revert().unwrap();
        let solid = rev.solids().next().expect("one solid").0;
        assert_eq!(
            AtRestBody::validate(rev, tol()).map(|_| ()),
            Err(vec![ValidationError::NegativeVolume { solid }]),
            "r {r}: the reverted tube refuses at rest"
        );
    }
}
