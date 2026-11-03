//! **The football with conical ends revolves.** A band `|v| ≤ v₁` of
//! one spindle-torus sheet between two cones, built from its profile
//! through the revolve door: the arc on the circle of radius `r` centred
//! `R` from the axis, its ends joined to the axis by lines and closed
//! along it. `R < 0` is the lemon, `0 < R < r` the apple.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::validated;
use geom_core::{Point2, Tol, Vec2};
use profile::test_support::bulge_loop;
use sweep::{Revolution, RevolveAxis, revolve};

/// A conical-ended band: the sheet's circle `(R, r)`, its band's half
/// angle `v₁`, the cones' apex height `z_t`, all about the axis through
/// `at` along the sketch `y`.
#[derive(Clone, Copy, Debug)]
struct Band {
    major: f64,
    minor: f64,
    half: f64,
    tip: f64,
    at: (f64, f64),
}

impl Band {
    fn rim(self) -> (f64, f64) {
        (
            self.major + self.minor * self.half.cos(),
            self.minor * self.half.sin(),
        )
    }

    fn profile(self) -> profile::ProfileLoop<f64> {
        let (rho, z) = self.rim();
        let (x0, y0) = self.at;
        let p = |x: f64, y: f64| Point2::new(x0 + x, y0 + y);
        bulge_loop(vec![
            (p(0.0, -self.tip), 0.0),
            (p(rho, -z), (self.half / 2.0).tan()),
            (p(rho, z), 0.0),
            (p(0.0, self.tip), 0.0),
        ])
    }

    fn axis(self) -> RevolveAxis<f64> {
        RevolveAxis {
            origin: Point2::new(self.at.0, self.at.1),
            dir: Vec2::new(0.0, 1.0),
        }
    }
}

const LEMON: Band = Band {
    major: -0.5,
    minor: 1.0,
    half: 0.8,
    tip: 1.2,
    at: (0.0, 0.0),
};

const APPLE: Band = Band {
    major: 0.4,
    minor: 1.0,
    half: 1.6,
    tip: 1.6,
    at: (0.0, 0.0),
};

#[test]
fn a_conical_ended_football_revolves() {
    let tol = Tol::witness();
    for (what, band) in [
        ("the lemon", LEMON),
        ("the apple", APPLE),
        (
            "the lemon a thousand out",
            Band {
                at: (1e3, 1e3),
                ..LEMON
            },
        ),
    ] {
        let built = revolve(
            &validated(vec![band.profile()]),
            band.axis(),
            Revolution::Full,
            tol,
        )
        .unwrap_or_else(|e| panic!("{what}: {e}"));
        topo::AtRestBody::validate(built.body, tol).unwrap_or_else(|e| panic!("{what}: {e:?}"));
    }
}
