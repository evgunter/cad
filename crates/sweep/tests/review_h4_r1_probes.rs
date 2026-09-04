//! **FILLET-H4 review lane r1 probes.** Three rows the unit's own suite
//! does not carry, each one an independent derivation rather than a
//! re-read of the shipped numbers.
//!
//! - `the_waist_arm_rests_the_ball_on_the_void_side_at_the_hand_value`
//!   pins the ARM, not the carve: the concave waist's torus spine is the
//!   hand-derived void-side rest `0.5 + r√2` to the last bit, and BOTH
//!   trim circles land inside their own cone face's span. That is the
//!   Phase-1 finding's mirror value (`0.5 − r√2`) excluded at the site
//!   the fold lives, one level below the volume rows.
//! - `the_plane_sphere_fold_is_an_xnor_in_all_four_quadrants` closes the
//!   one quadrant of `(sphere_convex, convex)` that no fixture in the
//!   tree reaches — a CONCAVE chain against a POCKET sphere — against a
//!   derivation of where the ball must be, so the fold's fourth row is
//!   pinned by a claim rather than by symmetry.
//! - `a_dropped_edge_retirement_is_invisible_to_naming_totality`
//!   records the direction the naming rows do NOT check: nothing
//!   asserts that a source entity absent from the output appears in
//!   `dead`. It is written as a POSITIVE row over the shipped record so
//!   it stays green, and states the gap in its assertions' shape.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Tol, Vec3};
use sweep::blend::arms::plane_sphere_blend;
use sweep::blend::battery::{BlendRequest, run_battery};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{rim_arcs_at, waisted};
use topo::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

const WAIST_R: f64 = 0.05;

/// The one link the battery resolves for a rim, with its arm's blend.
fn rim_link(body: &Body<f64>, arcs: &[EdgeKey], r: f64) -> sweep::blend::battery::Link<f64> {
    let req = BlendRequest {
        body,
        edges: arcs.to_vec(),
        size: r,
    };
    let verdict = run_battery(&req, band()).unwrap_or_else(|e| panic!("the battery passes: {e:?}"));
    verdict
        .chains
        .iter()
        .flat_map(|c| c.links().cloned())
        .next()
        .expect("one link at least")
}

/// **The arm rests the ball in the VOID, at the hand value.**
///
/// The waisted body's profile is `(0,0)→(1,0)→(0.5,0.5)→(1,1)→(0,1)`
/// revolved about `y`: two 45° cones meeting at the waist vertex
/// `V = (0.5, 0.5)` in the meridian half-plane, material on the axis
/// side, so the VOID wedge at `V` opens toward `+x` and is 90°. A ball
/// of radius `r` tangent to both generators from inside that wedge sits
/// on its bisector — the `+x` ray from `V` — at distance
/// `r / sin 45° = r√2`. So the spine is the circle of radius
/// `x_v + r√2` in the plane `y = y_v`, and the two feet are `r` along
/// each generator from `V`: `(x_v + r/√2, y_v ± r/√2)`.
///
/// Both consequences are asserted, because only the second one
/// distinguishes a rest from a MIRRORED rest: the mirror value
/// `x_v − r√2` puts the feet at `y_v ∓ r/√2` on the OTHER cone's
/// extension past the rim, off both faces (the Phase-1 finding). Here
/// the upper cone's face spans `y ≥ 0.5` and its foot must be above the
/// waist, the lower cone's below.
#[test]
fn the_waist_arm_rests_the_ball_on_the_void_side_at_the_hand_value() {
    let body = waisted(tol());
    let arcs = rim_arcs_at(&body, 0.5, 0.5);
    assert_eq!(arcs.len(), 2, "the waist rim is seam-split into two arcs");
    let link = rim_link(&body, &arcs, WAIST_R);

    let want_spine = 0.5 + WAIST_R * 2f64.sqrt();
    let mirror = 0.5 - WAIST_R * 2f64.sqrt();
    let Surface::Torus {
        center,
        major_radius,
        minor_radius,
        ..
    } = link.blend.surface
    else {
        panic!(
            "the coaxial arm mints a torus, got {:?}",
            link.blend.surface
        )
    };
    assert_eq!(
        major_radius, want_spine,
        "the void-side rest, bit for bit: {major_radius} vs the hand value {want_spine} \
         (the mirrored rest is {mirror})"
    );
    assert_eq!(minor_radius, WAIST_R, "the tube is the requested radius");
    assert!(
        center.y == 0.5 && center.x == 0.0 && center.z == 0.0,
        "the spine is centred on the axis in the waist's own plane, got {center:?}"
    );

    // The feet, from the trimlines, and the FACE each one must lie in.
    let foot = |c: &Curve3<f64>| match *c {
        Curve3::Circle { center, radius, .. } => (radius, center.y),
        ref other => panic!("a coaxial trimline is a circle, got {other:?}"),
    };
    let want_x = 0.5 + WAIST_R / 2f64.sqrt();
    let want_dy = WAIST_R / 2f64.sqrt();
    let (xa, ya) = foot(&link.blend.trim_a.0);
    let (xb, yb) = foot(&link.blend.trim_b.0);
    for (x, _) in [(xa, ya), (xb, yb)] {
        assert!(
            (x - want_x).abs() < 1e-15,
            "each foot is r along its generator from the vertex: {x} vs {want_x}"
        );
    }
    let mut ys = [ya, yb];
    ys.sort_by(f64::total_cmp);
    assert!(
        (ys[0] - (0.5 - want_dy)).abs() < 1e-15 && (ys[1] - (0.5 + want_dy)).abs() < 1e-15,
        "one foot per cone, each on its own side of the waist: {ys:?}"
    );
    // The discriminating half: both feet are INSIDE their face's span.
    // The lower cone's face spans y ≤ 0.5, the upper's y ≥ 0.5, and a
    // mirrored rest puts each foot on the other cone's extension.
    assert!(
        ys[0] < 0.5 && ys[1] > 0.5,
        "a trimline circle lies on its own support's face, not past the rim: {ys:?}"
    );
}

/// **The plane–sphere fold is an XNOR, and its fourth quadrant is a
/// claim.**
///
/// The ball must be `r` from the plane on the side the chain picks (the
/// material side on a convex chain, the void side on a concave one) and
/// `r` from the sphere on that same side of the SPHERE. Material is
/// inside the sphere exactly when `sphere_convex`, so the ball centre
/// lies inside the sphere exactly when `sphere_convex == convex`, i.e.
/// at `|c − c_s| = R − r` there and `R + r` otherwise. Three of the
/// four quadrants have fixtures in the tree (dome `(t, t)`, pip
/// `(f, t)`, boss `(t, f)`); the fourth — a concave chain against a
/// pocket, which is a spherical bubble rising out of the floor of a
/// cavity — has none, so it is pinned here.
///
/// The plane is `z = 0` with outward normal `+z` (material below), the
/// sphere is centred at `z = c` with `c > 0`.
#[test]
fn the_plane_sphere_fold_is_an_xnor_in_all_four_quadrants() {
    let origin = Point3::new(0.0, 0.0, 0.0);
    let n = Vec3::new(0.0, 0.0, 1.0);
    let u = Vec3::new(1.0, 0.0, 0.0);
    let big_r = 0.09_f64;
    let r = 0.02_f64;
    let c = 0.04_f64;
    let sphere_c = Point3::new(0.0, 0.0, c);

    for (sphere_convex, convex) in [(true, true), (false, true), (true, false), (false, false)] {
        let blend = plane_sphere_blend(origin, n, u, sphere_c, big_r, r, sphere_convex, convex);
        let Surface::Torus {
            center,
            major_radius,
            ..
        } = blend.surface
        else {
            panic!("the plane–sphere arm mints a torus")
        };
        // The ball centre, anywhere on the spine: `major_radius` out
        // along `u` from the spine's centre.
        let ball = center + u * major_radius;

        // (1) `r` from the plane, on the side the chain picks.
        let signed_depth = (ball - origin).dot(n);
        let want_depth = if convex { -r } else { r };
        assert!(
            (signed_depth - want_depth).abs() < 1e-15,
            "({sphere_convex}, {convex}): the ball is r {} the plane, got {signed_depth}",
            if convex { "into" } else { "out of" }
        );

        // (2) `r` from the sphere, inside it exactly on the XNOR.
        let dist = (ball - sphere_c).norm();
        let want = if sphere_convex == convex {
            big_r - r
        } else {
            big_r + r
        };
        assert!(
            (dist - want).abs() < 1e-15,
            "({sphere_convex}, {convex}): the offset sphere is {want}, got {dist}"
        );

        // (3) The plane's trimline is ON the plane, and its setback
        // sign agrees with whether the blend widens the plane's hole.
        let Curve3::Circle {
            center: tc, radius, ..
        } = blend.trim_a.0
        else {
            panic!("the plane trimline is a circle")
        };
        assert!(
            tc.z.abs() < 1e-15,
            "({sphere_convex}, {convex}): the plane trim sits on the plane, got {tc:?}"
        );
        let rim = (big_r * big_r - c * c).sqrt();
        let widens = sphere_convex != convex;
        assert_eq!(
            radius > rim,
            widens,
            "({sphere_convex}, {convex}): the hole widens iff the offset is R + r \
             (rim {rim}, trim {radius})"
        );
    }
}

/// **The direction naming totality does not check.**
///
/// `fillet_h4_concave_rim::a_concave_band_records_every_birth_and_every_death`
/// and its convex sibling both assert (a) every OUTPUT entity is a
/// recorded mint or a survivor and (b) every entry of `dead` names a
/// source key that did not survive. Neither asserts (c): that every
/// source entity ABSENT from the output appears in `dead`. So a
/// retirement the surgery forgets to record is invisible to both — the
/// census delta is taken over the bodies, not over the record, and
/// cannot see it either.
///
/// This row asserts (c) on the concave band, which is what closes the
/// accounting: source keys gone from the output, and the recorded
/// retirements, are the SAME SET.
#[test]
fn a_dropped_edge_retirement_is_invisible_to_naming_totality() {
    let source = waisted(tol());
    let arcs = rim_arcs_at(&source, 0.5, 0.5);
    let out = fillet_edges(&source, &arcs, WAIST_R, tol())
        .unwrap_or_else(|e| panic!("the waist carves, got {e:?}"));
    let rec = out
        .naming
        .as_ref()
        .expect("the rim phase records its births");

    let mut vanished: Vec<EdgeKey> = source
        .edges()
        .map(|(k, _)| k)
        .filter(|k| !out.body.edges().any(|(o, _)| o == *k))
        .collect();
    let mut recorded: Vec<EdgeKey> = rec.dead.edges.clone();
    vanished.sort_unstable();
    recorded.sort_unstable();
    recorded.dedup();
    assert_eq!(
        vanished, recorded,
        "every source edge the carve removed is a RECORDED retirement, and no \
         recorded retirement is a source edge that survived"
    );

    let mut gone_v: Vec<_> = source
        .vertices()
        .map(|(k, _)| k)
        .filter(|k| !out.body.vertices().any(|(o, _)| o == *k))
        .collect();
    let mut dead_v = rec.dead.vertices.clone();
    gone_v.sort_unstable();
    dead_v.sort_unstable();
    dead_v.dedup();
    assert_eq!(
        gone_v, dead_v,
        "the same, for vertices: the set the carve removed is the set recorded"
    );
}
