//! The profile-door and split-door fixtures, declared once for the
//! whole binary.
//!
//! Every suite that runs a sweep verb needs the same three things: a
//! disc on the sketch xy plane, a disc clear of the axis it will be
//! spun about, and that axis; every suite that runs the split needs
//! a plane through the unit cube and the one-sided pinch prism the
//! kernel's D7 lane exists for. Two suites spelling them separately is
//! two chances for one of them to drift onto a different radius or a
//! different half-plane and for a row to be pinning something other
//! than it says.

#![allow(dead_code)] // one instance per binary; no single suite uses all of it
#![allow(clippy::expect_used)] // a fixture that will not build is a failure, not a value
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::RevolveAxis;
use topo::{Body, SplitPlane};

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

/// A horizontal plane at height `z`, normal `+z` — through the unit
/// cube at `z = 0.5`, clear of it above `z = 1`.
pub fn z_plane(z: f64) -> SplitPlane<f64> {
    SplitPlane {
        origin: Point3::new(0.0, 0.0, z),
        normal: Vec3::new(0.0, 0.0, 1.0),
    }
}

/// **The touching-wedge prism** — the kernel split's own D7 fixture
/// (`topo`'s acceptance suite calls it MIRRORED): a slab over three
/// floor pieces that touch the slab's underside at `y = 1` along tip
/// lines only, extruded 1 along `+z`. Split at [`pinch_plane`] with
/// `+y` up, the pinched pieces are BELOW, and the direct run refuses
/// the one-sided pinch; the kernel door reruns it mirrored and swaps
/// the sides back. A dispatch row that agrees with the door on THIS
/// operand is agreeing through that lane, not around it.
pub fn pinch_prism() -> Body<f64> {
    let verts = [
        (0.0, 0.0),
        (3.0, 0.0),
        (4.0, 1.0),
        (5.0, 0.0),
        (6.0, 1.0),
        (7.0, 1.0),
        (8.0, 0.0),
        (8.0, 2.0),
        (0.0, 2.0),
    ]
    .into_iter()
    .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
    .collect();
    sweep::test_support::prism(verts, 1.0, tol())
}

/// The plane through [`pinch_prism`]'s tip lines: `y = 1`, normal
/// `+y`.
pub fn pinch_plane() -> SplitPlane<f64> {
    SplitPlane {
        origin: Point3::new(0.0, 1.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    }
}
