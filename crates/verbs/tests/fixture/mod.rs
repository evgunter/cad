//! The profile-door fixtures, declared once for the whole binary.
//!
//! Every suite that runs a sweep verb needs the same three things: a
//! disc on the sketch xy plane, a disc clear of the axis it will be
//! spun about, and that axis. Two suites spelling them separately is
//! two chances for one of them to drift onto a different radius or a
//! different half-plane and for a row to be pinning something other
//! than it says.

#![allow(dead_code)] // one instance per binary; no single suite uses all of it
#![allow(clippy::expect_used)] // a fixture that will not build is a failure, not a value
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, SketchPlane, ValidatedProfile};
use sweep::RevolveAxis;

/// The committed global tolerance, witnessed.
pub fn tol() -> Tol {
    Tol::witness()
}

/// A disc of radius `r` on the sketch xy plane, centred at the
/// origin — the extrude fixture, and the shape whose walls carry the
/// declared radius.
pub fn disc(r: f64) -> ValidatedProfile<f64> {
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol()).expect("a circle of positive radius");
    Profile::new(SketchPlane::xy(), vec![lp.into()])
        .validate(tol())
        .expect("a circle is a valid profile")
}

/// A disc of radius `r` centred at `(0, −d)` — the revolve fixture,
/// clear of the x axis it is spun about, so its walls are tori. The
/// sign is the door's own half-plane convention: the signed radial
/// coordinate about `(origin, dir)` is `(p − origin).perp_dot(dir)`,
/// which for the +x axis is `−y`, so the profile lives at negative y.
pub fn offset_disc(r: f64, d: f64) -> ValidatedProfile<f64> {
    let lp = profile::circle(Point2::new(0.0, -d), r, tol()).expect("a circle of positive radius");
    Profile::new(SketchPlane::xy(), vec![lp.into()])
        .validate(tol())
        .expect("a circle is a valid profile")
}

/// The x axis, in sketch coordinates.
pub fn x_axis() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(1.0, 0.0),
    }
}
