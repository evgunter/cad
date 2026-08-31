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
//! **What these rows DO and do not reach, counted rather than
//! claimed.** An earlier header listed the door's decide names and said
//! all of them execute here; both review arms measured otherwise, and
//! the honest statement is per fixture:
//!
//! | fixture | the arms it is here for |
//! |---|---|
//! | sphere-zone vase | the CARRIED azimuth; `seam_concentric` (a sphere's great-circle seam); `pole`; line∩circle `corner` and `branch` |
//! | partial-revolve wedge | the SOLVED azimuth (`azimuth`, `azimuth_amp`, `azimuth_arm`, `azimuth_residual`); `meridian_through`; line∩line `corner` |
//! | cone frustum | `nappe` and `side` — the generator's own branch choice, which neither of the others has a cone to reach |
//! | drum | `seam_radial`, a cylinder generator seam, which a partial revolve has no seam to reach and a sphere zone has no cylinder for |
//!
//! Still unreached at this scalar, and said so rather than implied:
//! `chart_motion`'s Zero arm (no interval row calls the door with a
//! mixed set — that is `shell_open`'s lift, which these rows do not
//! take), `latitude_tilt`'s and `seam_concentric`'s REFUSING arms, and
//! `pole_station`. Those are f64 rows in `sf2b_axial.rs` and the two
//! review-probe files.
//!
//! Both shapes the door solves are here: the CARRIED azimuth (a full
//! revolve, where two surfaces meet at a corner) and the SOLVED one (a
//! partial revolve, whose meridian caps contain the axis).

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Bounds, Interval, Point2, Real, Tol, Vec2};
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
    let hollow = topo::shell(&body, iv(t), FIT_TOL, tol)
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
    contains("sphere-zone wall", got, want);
}

/// **Containment AND width.** Containment alone gets easier as an
/// enclosure degrades — `[0, 1]` contains every answer here — so a row
/// that only asserts it cannot tell a certified result from a collapsed
/// one. The width is bounded relative to the answer's own size, which
/// is the quantity a widening would be relative to.
fn contains(what: &str, got: Interval, want: f64) {
    let (lo, hi) = (got.lo(), got.hi());
    println!(
        "[sf2b-interval] {what} enclosure [{lo}, {hi}] vs {want} (width {})",
        hi - lo
    );
    assert!(
        lo <= want && want <= hi,
        "{what}: the enclosure must contain the closed form {want}, got [{lo}, {hi}]"
    );
    assert!(
        hi - lo <= 1e-8 * want.abs(),
        "{what}: the enclosure must stay TIGHT — width {} against {want}",
        hi - lo
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
    let hollow = topo::shell(&body, iv(t), FIT_TOL, tol)
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
    contains("wedge wall", got, want);
}

/// **The cone frustum at `T = Interval`** — the only fixture here whose
/// corner reaches `offset_axial_nappe` and `offset_axial_side`. The
/// generator's branch is chosen from the corner's own side of the apex
/// and the caller's distance is turned over for the mirror nappe, both
/// DECIDED; at this scalar an ambiguous side escalates rather than
/// picking, which is the property the row is here for.
#[test]
fn interval_offset_charts_together_cone_frustum() {
    let tol = Tol::witness();
    let (r0, r1, h, t) = (4.0 / 64.0, 2.0 / 64.0, 8.0 / 64.0, 1.0 / 128.0);
    let body = revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), iv(0.0)),
            ProfileVertex::new(p2(r0, 0.0), iv(0.0)),
            ProfileVertex::new(p2(r1, h), iv(0.0)),
            ProfileVertex::new(p2(0.0, h), iv(0.0)),
        ]),
        Revolution::Full,
    );
    let hollow = topo::shell(&body, iv(t), FIT_TOL, tol)
        .expect("the cone frustum hollows at the certified scalar");
    assert_eq!(hollow.shells().count(), 2, "outer + cavity");
    let frustum_v =
        |a: f64, b: f64, k: f64| core::f64::consts::PI * k * (a * a + a * b + b * b) / 3.0;
    let tan_a = (r0 - r1) / h;
    let apex_in = r0 / tan_a - t / tan_a.atan().sin();
    let want = frustum_v(r0, r1, h)
        - frustum_v(
            (apex_in - t) * tan_a,
            (apex_in - (h - t)) * tan_a,
            h - 2.0 * t,
        );
    contains(
        "cone frustum wall",
        topo::mass_properties(&hollow, tol).expect("props").volume,
        want,
    );
}

/// **The drum at `T = Interval`** — the only fixture here with a
/// CYLINDER generator seam, so the only one that reaches
/// `offset_axial_seam_radial`. A partial revolve has no seam at all and
/// a sphere zone's seam is the concentric arc, not this.
#[test]
fn interval_offset_charts_together_drum() {
    let tol = Tol::witness();
    let (r, h, t) = (3.0 / 64.0, 8.0 / 64.0, 1.0 / 128.0);
    let body = revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), iv(0.0)),
            ProfileVertex::new(p2(r, 0.0), iv(0.0)),
            ProfileVertex::new(p2(r, h), iv(0.0)),
            ProfileVertex::new(p2(0.0, h), iv(0.0)),
        ]),
        Revolution::Full,
    );
    let hollow =
        topo::shell(&body, iv(t), FIT_TOL, tol).expect("the drum hollows at the certified scalar");
    assert_eq!(hollow.shells().count(), 2, "outer + cavity");
    let pi = core::f64::consts::PI;
    let want = pi * r * r * h - pi * (r - t) * (r - t) * (h - 2.0 * t);
    contains(
        "drum wall",
        topo::mass_properties(&hollow, tol).expect("props").volume,
        want,
    );
}
