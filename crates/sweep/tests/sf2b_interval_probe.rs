//! VERBS-SHELLFIX PR-2b, interval lane: the AXIAL door instantiated at
//! the certified scalar.
//!
//! The door lives in `crates/topo/src/offset_axial.rs`, an ordinarily
//! named file, so `scripts/ci-filter.py`'s path rule does not match it
//! and the interval lane is left to the run's own draw. A drawn
//! interval point re-runs the f64-typed suites under the interval
//! BUILD, which never instantiates `offset_charts_together` at
//! `T = Interval` — so this row does, and every new decide site the
//! unit added is executed at the scalar that can escalate rather than
//! at the one that cannot:
//!
//! `offset_axial_alignment`, `offset_axial_centre`,
//! `offset_axial_meridian`, `offset_axial_meridian_through`,
//! `offset_axial_nappe`, `offset_axial_request`, `offset_axial_pole`,
//! `offset_axial_pole_station`, `offset_axial_side`,
//! `offset_axial_corner`, `offset_axial_branch`,
//! `offset_axial_radius`, `offset_axial_concurrence`,
//! `offset_axial_azimuth_arm`, `offset_axial_azimuth_amp`,
//! `offset_axial_azimuth`, `offset_axial_azimuth_residual`,
//! `offset_axial_chart_motion`, `offset_axial_seam_radial`,
//! `offset_axial_seam_concentric`, `offset_axial_latitude`,
//! `offset_axial_latitude_tilt`, `offset_axial_edge_agreement`,
//! `offset_axial_edge_on_surface`.
//!
//! Both shapes the door solves are here: the CARRIED azimuth (a full
//! revolve, where two surfaces meet at a corner) and the SOLVED one (a
//! partial revolve, whose meridian caps contain the axis).

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Bounds, Interval, Point2, Real, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(iv(x), iv(y))
}

const FIT_TOL: f64 = 1e-6;

fn revolved(lp: ProfileLoop<Interval>, turn: Revolution<Interval>) -> Body<Interval> {
    let profile = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(iv(0.0), iv(1.0)),
        },
        turn,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

/// **The sphere-zone vase at `T = Interval`.** Every corner solve is a
/// line meeting a circle in the meridian half-plane, every
/// transversality and concurrence margin is decided from interval
/// enclosures, and the seam's azimuth is carried through them. The
/// result's volume enclosure must CONTAIN the closed form.
#[test]
fn interval_offset_charts_together_sphere_zone() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let (r, h, t) = (3.0 / 64.0, 8.0 / 64.0, 1.0 / 128.0);
    // The belly's arc, as a bulge about a centre ON the axis.
    let c = p2(0.0, h / 2.0);
    let (u, v) = (p2(r, 0.0) - c, p2(r, h) - c);
    let sweep = u.perp_dot(v).atan2(u.dot(v));
    let body = revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), iv(0.0)),
            ProfileVertex::new(p2(r, 0.0), (sweep / iv(4.0)).tan()),
            ProfileVertex::new(p2(r, h), iv(0.0)),
            ProfileVertex::new(p2(0.0, h), iv(0.0)),
        ]),
        Revolution::Full,
    );
    let hollow = topo::shell(&body, iv(t), FIT_TOL, band, tol)
        .expect("the sphere-zone vase hollows at the certified scalar");
    assert_eq!(hollow.shells().count(), 2, "outer + cavity");

    // The wall, as the difference of two spherical zones — the same
    // closed form `sf2b_axial.rs` pins at `f64`, here as an ENCLOSURE
    // claim: the interval result must contain the real answer.
    let big = (r * r + h * h / 4.0).sqrt();
    let zone = |rr: f64, half: f64| {
        core::f64::consts::PI * 2.0 * (rr * rr * half - half * half * half / 3.0)
    };
    let want = zone(big, h / 2.0) - zone(big - t, h / 2.0 - t);
    let got = topo::mass_properties(&hollow, tol).expect("props").volume;
    println!(
        "[sf2b-interval] sphere-zone wall enclosure [{}, {}] vs {want}",
        got.lo(),
        got.hi()
    );
    assert!(
        got.lo() <= want && want <= got.hi(),
        "the enclosure must contain the closed form {want}, got [{}, {}]",
        got.lo(),
        got.hi()
    );
}

/// **The partial-revolve wedge at `T = Interval`** — the door's OTHER
/// azimuth shape. Its meridian caps CONTAIN the axis, so each rim
/// corner's azimuth is solved as a circle meeting a plane, with the
/// two roots' separation metered as the length it is. That whole arm
/// is unreached by the vase above.
#[test]
fn interval_offset_charts_together_partial_wedge() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let (r, h, t) = (3.0 / 64.0, 8.0 / 64.0, 1.0 / 128.0);
    let body = revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), iv(0.0)),
            ProfileVertex::new(p2(r, 0.0), iv(0.0)),
            ProfileVertex::new(p2(r, h), iv(0.0)),
            ProfileVertex::new(p2(0.0, h), iv(0.0)),
        ]),
        Revolution::Partial(iv(core::f64::consts::FRAC_PI_2)),
    );
    let hollow = topo::shell(&body, iv(t), FIT_TOL, band, tol)
        .expect("the quarter-revolve wedge hollows at the certified scalar");
    assert_eq!(hollow.shells().count(), 2, "outer + cavity");

    // The cavity's cross-section is the disc of radius `r − t` cut by
    // two PERPENDICULAR chords at distance `t` — the azimuth solve's
    // own answer written as an area.
    let two_chord = |rr: f64, d: f64| {
        let top = (rr * rr - d * d).sqrt();
        let f = |x: f64| x * (rr * rr - x * x).sqrt() / 2.0 + rr * rr * (x / rr).asin() / 2.0;
        f(top) - f(d) - d * (top - d)
    };
    let want = core::f64::consts::PI * r * r * h / 4.0 - two_chord(r - t, t) * (h - 2.0 * t);
    let got = topo::mass_properties(&hollow, tol).expect("props").volume;
    println!(
        "[sf2b-interval] wedge wall enclosure [{}, {}] vs {want}",
        got.lo(),
        got.hi()
    );
    assert!(
        got.lo() <= want && want <= got.hi(),
        "the enclosure must contain the closed form {want}, got [{}, {}]",
        got.lo(),
        got.hi()
    );
}
