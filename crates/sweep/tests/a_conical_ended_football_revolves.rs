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

impl Band {
    /// The exact volume and area of the revolved profile: `∮ π ρ² dz`
    /// and `∮ 2π ρ ds` round the meridian loop, each segment in closed
    /// form (a line's frustum, the arc's band integrated in its tube
    /// angle; the axis segment adds nothing).
    fn exact(self) -> (f64, f64) {
        use core::f64::consts::PI;
        let (big, r, v1) = (self.major, self.minor, self.half);
        let (rho, z) = self.rim();
        let frustum = |(r0, z0): (f64, f64), (r1, z1): (f64, f64)| {
            let slant = ((r1 - r0).powi(2) + (z1 - z0).powi(2)).sqrt();
            (
                PI * (z1 - z0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0,
                PI * (r0 + r1) * slant,
            )
        };
        let band = |v: f64| {
            let (s, c) = v.sin_cos();
            PI * r * (big * big * s + big * r * (v + s * c) + r * r * (s - s.powi(3) / 3.0))
        };
        let (low, low_area) = frustum((0.0, -self.tip), (rho, -z));
        let (high, high_area) = frustum((rho, z), (0.0, self.tip));
        let volume = low + (band(v1) - band(-v1)) + high;
        let area = low_area + high_area + 2.0 * PI * r * (2.0 * big * v1 + 2.0 * r * v1.sin());
        (volume.abs(), area)
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

/// The sheet's apex height `√(r² − R²)` over its tube angle `v*`.
fn apex_angle(band: Band) -> f64 {
    (-band.major / band.minor).acos()
}

#[test]
fn a_conical_ended_football_revolves() {
    let tol = Tol::witness();
    let near_apex = Band {
        half: 0.97 * apex_angle(LEMON),
        tip: 1.2,
        ..LEMON
    };
    for (what, band) in [
        ("the lemon", LEMON),
        ("the apple", APPLE),
        (
            "the wide apple",
            Band {
                major: 0.8,
                minor: 1.0,
                half: 2.4,
                tip: 1.2,
                at: (0.0, 0.0),
            },
        ),
        ("the lemon near its apexes", near_apex),
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
        // The arc sweeps the sheet on its own side of the axis: `R`
        // signed as the profile's centre lies, never its mirror.
        let sheets: Vec<f64> = built
            .body
            .faces()
            .filter_map(|(_, f)| match built.body.get_surface(f.surface) {
                Some(geom::Surface::Spindle { major_radius, .. }) => Some(*major_radius),
                _ => None,
            })
            .collect();
        assert!(
            !sheets.is_empty() && sheets.iter().all(|&m| (m - band.major).abs() < 1e-12),
            "{what}: the sheet faces' major radii {sheets:?}, not {}",
            band.major
        );
        let body = topo::AtRestBody::validate(built.body, tol)
            .unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
        let props = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let (volume, area) = band.exact();
        let got = props.enclosure;
        assert!(
            (got.volume - volume).abs() <= 1e-10 * volume + got.volume_pad,
            "{what}: volume {} against {volume} (pad {})",
            got.volume,
            got.volume_pad
        );
        assert!(
            (got.surface_area - area).abs() <= 1e-10 * area + got.area_pad,
            "{what}: area {} against {area} (pad {})",
            got.surface_area,
            got.area_pad
        );
    }
}
