//! **FILLET-H4 review lane r1 probes.** Rows the unit's own suite does
//! not carry, each one an independent derivation rather than a re-read
//! of the shipped numbers.
//!
//! - `the_waist_arm_rests_the_ball_on_the_void_side_at_the_hand_value`
//!   pins the ARM, not the carve: the concave waist's torus spine is the
//!   hand-derived void-side rest `0.5 + r√2` to the last bit, and BOTH
//!   trim circles land inside their own cone face's span. That is the
//!   Phase-1 finding's mirror value (`0.5 − r√2`) excluded at the site
//!   the fold lives, one level below the volume rows. (Lane r2's rest
//!   row pinned the same numbers at 1e-12; this bit-exact one is the
//!   survivor, and r2's was retired at adoption.)
//! - `the_plane_sphere_fold_is_an_xnor_in_all_four_quadrants` pins the
//!   arm in every `(sphere sense, chain convexity)` quadrant against a
//!   derivation of where the ball must be — INCLUDING the plane's
//!   setback, a positive length in every quadrant, which is the value
//!   predicate 2's consumption screen subtracts: with the setback's case
//!   split reverted to the sphere's sense alone, two quadrants return a
//!   NEGATIVE setback and the screen widens, and no other row sees it.
//! - `the_fourth_quadrant_is_built_and_its_floor_rim_carves` builds the
//!   quadrant no other fixture reaches — a CONCAVE chain against a
//!   POCKET sphere, `test_support::domed_cavity` — through the public
//!   doors and carves it, with hand values at the arm and a Pappus
//!   closed form for the fill.
//!
//! The direction-(c) naming row this lane first carried (a vanished
//! source entity is a RECORDED retirement) was retired at adoption in
//! favour of lane r2's both-sides row, and (c) now lives in the shared
//! walk every totality row calls (`test_support::assert_naming_totality`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Tol, Vec3};
use sweep::blend::Convexity;
use sweep::blend::arms::plane_sphere_blend;
use sweep::blend::battery::{BlendRequest, run_battery};
use sweep::blend::build::fillet_edges;
use sweep::test_support::pappus::{pappus_volume, sector, segment, triangle};
use sweep::test_support::{domed_cavity, rim_arcs_at, waisted};
use topo::{Body, EdgeKey, mass_properties, validate_geometric};

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

/// **The plane–sphere fold is an XNOR, in all four quadrants, setback
/// included.**
///
/// The ball must be `r` from the plane on the side the chain picks (the
/// material side on a convex chain, the void side on a concave one) and
/// `r` from the sphere on that same side of the SPHERE. Material is
/// inside the sphere exactly when `sphere_convex`, so the ball centre
/// lies inside the sphere exactly when the sphere's sense agrees with
/// the chain's convexity, i.e. at `|c − c_s| = R − r` there and `R + r`
/// otherwise. All four quadrants have fixtures now (dome `(t, convex)`,
/// pip `(f, convex)`, boss `(t, concave)`, and `domed_cavity`
/// `(f, concave)` in the row below); this row pins the ARM's numbers in
/// each against the derivation.
///
/// The fourth assertion is the one the mutant table needs. The plane's
/// setback is the trimline's displacement from the rim, `|rim − s|`, a
/// POSITIVE length in every quadrant: `s² = rim² − 2r(R ± depth)` on the
/// inner offset and `rim² + 2r(R ± depth)` on the outer, so `s < rim`
/// exactly where the offset is `R − r`. The arm spells it by the same
/// agreement bit (`rim − s` inner, `s − rim` outer); spelt by the
/// sphere's sense ALONE it comes out NEGATIVE on both concave quadrants,
/// and predicate 2's `gap − setback − setback` then screens a concave
/// plane–sphere pair MORE permissively — which no carve row can see,
/// because a permissive screen only ever lets a carve through.
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
    let rim = (big_r * big_r - c * c).sqrt();

    for (sphere_convex, convexity) in [
        (true, Convexity::Convex),
        (false, Convexity::Convex),
        (true, Convexity::Concave),
        (false, Convexity::Concave),
    ] {
        let convex = convexity.blend_sense();
        let blend = plane_sphere_blend(origin, n, u, sphere_c, big_r, r, sphere_convex, convexity);
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
            "({sphere_convex}, {convexity}): the ball is r {} the plane, got {signed_depth}",
            if convex { "into" } else { "out of" }
        );

        // (2) `r` from the sphere, inside it exactly on the XNOR.
        let dist = (ball - sphere_c).norm();
        let inner = sphere_convex == convex;
        let want = if inner { big_r - r } else { big_r + r };
        assert!(
            (dist - want).abs() < 1e-15,
            "({sphere_convex}, {convexity}): the offset sphere is {want}, got {dist}"
        );

        // (3) The plane's trimline is ON the plane, and the hole widens
        // exactly on the outer offset.
        let Curve3::Circle {
            center: tc, radius, ..
        } = blend.trim_a.0
        else {
            panic!("the plane trimline is a circle")
        };
        assert!(
            tc.z.abs() < 1e-15,
            "({sphere_convex}, {convexity}): the plane trim sits on the plane, got {tc:?}"
        );
        assert_eq!(
            radius > rim,
            !inner,
            "({sphere_convex}, {convexity}): the hole widens iff the offset is R + r \
             (rim {rim}, trim {radius})"
        );

        // (4) The plane's SETBACK is the positive displacement |rim − s|
        // in every quadrant — the value predicate 2 subtracts.
        let setback = blend.trim_a.1;
        assert!(
            setback > 0.0 && (setback - (radius - rim).abs()).abs() < 1e-15,
            "({sphere_convex}, {convexity}): the plane setback is the positive length \
             |rim − s| = {}, got {setback} — a negative value here is the setback sign \
             folded from the sphere's sense alone, and it WIDENS predicate 2's margin",
            (radius - rim).abs()
        );
    }
}

/// **The fourth quadrant, built: a concave chain against a pocket
/// sphere carves and adds its fill.**
///
/// `test_support::domed_cavity`: floor plane `y = 0.3` (material
/// below), ceiling the hemisphere of radius `R = 0.5` centred on the
/// axis at the floor's level (material OUTSIDE it — the sphere face's
/// sense is `false`), a bore of radius `0.2` to the top. The rim is the
/// floor's edge at radius `R`; at it the material is the union of the
/// two half-spaces, so the chain is CONCAVE.
///
/// **By hand, in the meridian half-plane.** The floor's outward normal
/// is `+y` and the sphere centre `O = (0, 0.3)` sits ON the floor, so
/// `depth = 0`; on a concave chain the ball is `r` OUT of the floor's
/// material, i.e. `r` above it, and inside the sphere (sense `false`,
/// chain concave: agreement), at `R − r` from `O`. Its centre is
/// `C = (s, 0.3 + r)` with `s = √((R − r)² − r²)`, the spine radius;
/// the plane foot is `F_p = (s, 0.3)`, directly below; the sphere foot
/// is `F_s = O + (C − O)·R/(R − r)`; the plane's setback is `R − s > 0`
/// (the trim circle sits INSIDE the rim — the band rests on the floor
/// beyond the trim and adds material out to the rim).
///
/// **The fill** is the region between the floor, the sphere and the
/// fillet arc: the kite `V F_p C F_s` (`V = (R, 0.3)`) minus the
/// sector of the ball between the feet, PLUS the circular segment of
/// the sphere between the chord `V F_s` and its arc — the arc bulges
/// away from `O`, i.e. out of the kite, and the void (the fill) extends
/// to it. Pappus turns the pieces' first moments into `ΔV`.
#[test]
fn the_fourth_quadrant_is_built_and_its_floor_rim_carves() {
    let (big_r, floor, r) = (0.5_f64, 0.3_f64, 0.05_f64);
    let body = domed_cavity(tol());
    validate_geometric(&body, tol()).unwrap_or_else(|e| panic!("valid at rest, got {e:?}"));
    let mut spheres = 0;
    for (_, f) in body.faces() {
        if let Some(Surface::Sphere { center, radius, .. }) = body.get_surface(f.surface) {
            spheres += 1;
            assert!(
                center.x.abs() < 1e-12
                    && (center.y - floor).abs() < 1e-12
                    && (radius - big_r).abs() < 1e-12,
                "the ceiling is the sphere centred on the axis at the floor, got {center:?} {radius}"
            );
            assert!(
                !f.sense,
                "a POCKET: the sphere face's material is outside it"
            );
        }
    }
    assert_eq!(
        spheres, 2,
        "the ceiling is a seam-split sphere, two half-faces"
    );
    let arcs = rim_arcs_at(&body, big_r, floor);
    assert_eq!(arcs.len(), 2, "the floor rim is seam-split into two arcs");

    // The arm, against the hand values.
    let link = rim_link(&body, &arcs, r);
    let s = ((big_r - r).powi(2) - r * r).sqrt();
    let Surface::Torus {
        center,
        major_radius,
        minor_radius,
        ..
    } = link.blend.surface
    else {
        panic!(
            "the plane–sphere arm mints a torus, got {:?}",
            link.blend.surface
        )
    };
    assert!(
        (center.y - (floor + r)).abs() < 1e-15 && center.x.abs() < 1e-15,
        "the ball rests r ABOVE the floor, in the void: {center:?}"
    );
    assert!(
        (major_radius - s).abs() < 1e-15 && (minor_radius - r).abs() < 1e-15,
        "the spine rides the INNER offset sphere, s = √((R−r)² − r²) = {s}, got {major_radius}"
    );
    let (plane_trim, sphere_trim) = {
        let is_plane = |t: &(Curve3<f64>, f64)| matches!(t.0, Curve3::Circle { center, .. } if (center.y - floor).abs() < 1e-15);
        if is_plane(&link.blend.trim_a) {
            (&link.blend.trim_a, &link.blend.trim_b)
        } else {
            (&link.blend.trim_b, &link.blend.trim_a)
        }
    };
    let Curve3::Circle { radius: tp, .. } = plane_trim.0 else {
        panic!("the plane trim is a circle")
    };
    assert!(
        (tp - s).abs() < 1e-15,
        "the plane trim circle has the spine's radius"
    );
    assert!(
        plane_trim.1 > 0.0 && (plane_trim.1 - (big_r - s)).abs() < 1e-15,
        "the plane setback is the positive length R − s = {}, got {} (negative here is the \
         sign folded from the sphere's sense alone)",
        big_r - s,
        plane_trim.1
    );
    let scale = big_r / (big_r - r);
    let Curve3::Circle {
        center: sc,
        radius: sr,
        ..
    } = sphere_trim.0
    else {
        panic!("the sphere trim is a circle")
    };
    assert!(
        (sc.y - (floor + r * scale)).abs() < 1e-15 && (sr - s * scale).abs() < 1e-15,
        "the sphere trim is the ball's contact circle scaled to R: {sc:?} {sr}"
    );

    // The carve, and the fill by Pappus.
    let v0 = mass_properties(&body, tol()).expect("props").volume;
    let out = fillet_edges(&body, &arcs, r, tol())
        .unwrap_or_else(|e| panic!("the floor rim carves, got {e:?}"));
    assert_eq!(out.band_faces.len(), 1, "one band");
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier-3 valid, got {e:?}"));
    let p1 = mass_properties(&out.body, tol()).expect("props");
    assert_eq!(
        p1.volume_pad, 0.0,
        "closed-form inventory: the pad is exactly zero"
    );
    assert!(
        p1.volume > v0,
        "a concave band ADDS material: {} vs {v0}",
        p1.volume
    );

    let o = (0.0, floor);
    let v = (big_r, floor);
    let c = (s, floor + r);
    let f_p = (s, floor);
    let f_s = (o.0 + (c.0 - o.0) * scale, o.1 + (c.1 - o.1) * scale);
    let fill = pappus_volume(&[
        (1.0, triangle(v, f_p, c)),
        (1.0, triangle(v, c, f_s)),
        (-1.0, sector(c, r, f_p, f_s)),
        (1.0, segment(o, big_r, v, f_s)),
    ]);
    assert!(
        (p1.volume - v0 - fill).abs() < 1e-13,
        "the fill is the Pappus closed form: measured {} vs derived {fill}",
        p1.volume - v0
    );
}
