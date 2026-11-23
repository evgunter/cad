//! **Placed balls mesh their own volume.** The unit ball's two faces are
//! rimless pole-to-pole bands, whose meridian columns the walk reads off the
//! loop's stored lift (`mesh::walk`'s band columns); a column read off a
//! placement-dependent quantity would mesh the complementary half at some
//! placement, so these rows mesh balls at many placements through the
//! public doors and compare the volume.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, RigidMotion, Tol, Vec2};
use mesh::validate::{check_mesh, signed_volume};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};

/// A unit ball whose sketch plane sits at `(d, d, d)`: its two bands are
/// rimless pole-to-pole loops, whose columns the walk reads off the lift.
fn placed_ball(d: f64) -> Result<topo::Body<f64>, String> {
    let plane =
        profile::SketchPlane::from_frame(RigidMotion::axes_xy(geom_core::Point3::new(d, d, d)));
    let lp = bulge_loop(vec![
        (geom_core::Point2::new(0.0, -1.0), 1.0),
        (geom_core::Point2::new(0.0, 1.0), 0.0),
    ]);
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(geom_core::Tol::witness())
        .map_err(|e| format!("profile validate: {e:?}"))?;
    sweep::revolve(
        &vp,
        sweep::RevolveAxis {
            origin: geom_core::Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        },
        sweep::Revolution::Full,
        geom_core::Tol::witness(),
    )
    .map(|r| r.body.into_body())
    .map_err(|e| format!("revolve: {e:?}"))
}

/// **The ball meshes the same volume at every placement the doors
/// admit.** Each placement either tessellates, and then must be watertight
/// and measure what the origin-placed ball measures, or refuses typed,
/// which is printed. A mis-picked meridian branch meshes the complementary
/// half and moves the volume by O(1), not by ulps.
#[test]
fn the_ball_meshes_the_same_volume_at_every_placement_the_doors_admit() {
    use mesh::validate::{check_mesh, signed_volume};
    let exact = 4.0 * core::f64::consts::PI / 3.0;
    let mut admitted: Vec<(f64, f64)> = Vec::new();
    let mut refused = Vec::new();
    for d in [0.0, 1e2, 1e4, 1e6, 1e8, 1e10] {
        let body = match placed_ball(d) {
            Ok(b) => b,
            Err(e) => {
                println!("d = {d:e}: typed refusal upstream — {e}");
                refused.push(d);
                continue;
            }
        };
        let m = match mesh::tessellate(&body, 0.02, geom_core::Tol::witness()) {
            Ok(m) => m,
            Err(e) => {
                println!("d = {d:e}: typed refusal at tessellate — {e:?}");
                refused.push(d);
                continue;
            }
        };
        let watertight = check_mesh(&m);
        let v = signed_volume(&m);
        let tris: usize = m.patches.iter().map(|p| p.triangles.len()).sum();
        println!(
            "d = {d:e}: {tris} triangles, watertight {watertight:?}, volume {v} \
             (chordal deficit vs 4π/3 = {:e})",
            ((v - exact) / exact).abs()
        );
        assert_eq!(watertight, Ok(()), "d = {d:e}: mesh not watertight");
        admitted.push((d, v));
    }
    println!("admitted placements {admitted:?}; typed-refused {refused:?}");
    let (_, v0) = *admitted.first().expect("the origin-placed ball must build");
    // The oracle: the SAME body meshed at a distance must measure the
    // same volume. A mis-picked meridian branch meshes the
    // complementary half and moves this by O(1), not by ulps.
    for &(d, v) in &admitted {
        let rel = ((v - v0) / v0).abs();
        println!("  placement {d:e}: volume drift vs origin = {rel:e}");
        assert!(
            rel < 1e-3,
            "d = {d:e}: volume {v} vs {v0} at the origin (relative {rel:e}) — \
             a branch pick, not a chord budget"
        );
    }
}

/// A unit-scaled ball of radius `r` on a sketch plane at `(d, d, d)`.
fn placed_ball_r(d: f64, r: f64) -> Result<topo::Body<f64>, String> {
    let plane =
        profile::SketchPlane::from_frame(RigidMotion::axes_xy(geom_core::Point3::new(d, d, d)));
    let lp = bulge_loop(vec![
        (geom_core::Point2::new(0.0, -r), 1.0),
        (geom_core::Point2::new(0.0, r), 0.0),
    ]);
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(geom_core::Tol::witness())
        .map_err(|e| format!("profile validate: {e:?}"))?;
    sweep::revolve(
        &vp,
        sweep::RevolveAxis {
            origin: geom_core::Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        },
        sweep::Revolution::Full,
        geom_core::Tol::witness(),
    )
    .map(|r| r.body.into_body())
    .map_err(|e| format!("revolve: {e:?}"))
}

/// **A small ball far out meshes the volume it meshes at the origin.**
/// A small ball at a moderate distance clears the import-scale doors (an
/// absolute test, coordinate ulp against ε) while its distance-to-radius
/// ratio is large, the corner where a placement-dependent branch pick
/// would show. Each pair either meshes at both places with one volume or
/// refuses typed, which is printed.
#[test]
fn a_small_ball_far_out_meshes_the_volume_it_meshes_at_the_origin() {
    use mesh::validate::{check_mesh, signed_volume};
    for (r, d) in [
        (1e-2, 1e4),
        (1e-3, 1e4),
        (1e-4, 1e4),
        (1e-4, 1e5),
        (1e-5, 1e4),
        (1e-6, 1e3),
    ] {
        let exact = 4.0 * core::f64::consts::PI * r * r * r / 3.0;
        let ratio = (d * 3f64.sqrt()) / r;
        let build = |dd: f64| -> Result<f64, String> {
            let body = placed_ball_r(dd, r)?;
            let m = mesh::tessellate(&body, r / 20.0, geom_core::Tol::witness())
                .map_err(|e| format!("tessellate: {e:?}"))?;
            if check_mesh(&m) != Ok(()) {
                return Err(format!("not watertight: {:?}", check_mesh(&m)));
            }
            Ok(signed_volume(&m))
        };
        match (build(0.0), build(d)) {
            (Ok(v0), Ok(v)) => {
                let rel = ((v - v0) / v0).abs();
                println!(
                    "r {r:e} d {d:e} (D/r ≈ {ratio:e}): ADMITTED — origin {v0:e}, \
                     placed {v:e}, drift {rel:e}, exact {exact:e}"
                );
                assert!(
                    rel < 1e-3,
                    "r {r:e} d {d:e}: the placed mesh's volume moved by {rel:e} — \
                     a placement-dependent branch pick"
                );
            }
            (_, Err(e)) => println!("r {r:e} d {d:e} (D/r ≈ {ratio:e}): refused — {e}"),
            (Err(e), _) => println!("r {r:e} d {d:e}: origin build refused — {e}"),
        }
    }
}

/// End to end through the public API: a unit ball revolved about an
/// axis ~9e2 m from the world origin (inside the 1 km session box,
/// which refuses one past it), tessellated, volume checked
/// against 4π/3. The ball's two faces are rimless pole-to-pole bands.
#[test]
fn a_ball_far_from_the_origin_meshes_its_own_volume() {
    let (cx, cy) = (4.3e2, -7.9e2);
    let lp = bulge_loop(vec![
        (Point2::new(cx, cy - 1.0), 1.0),
        (Point2::new(cx, cy + 1.0), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("profile");
    let axis = RevolveAxis {
        origin: Point2::new(cx, cy),
        dir: Vec2::new(0.0, 1.0),
    };
    let t = revolve(&vp, axis, Revolution::Full, Tol::witness()).expect("revolve");
    let mesh = mesh::tessellate(&t.body, 0.01, Tol::witness()).expect("tessellate");
    assert_eq!(check_mesh(&mesh), Ok(()));
    let v = signed_volume(&mesh);
    let exact = 4.0 * core::f64::consts::PI / 3.0;
    println!("placed ball volume {v} exact {exact}");
    assert!(v > 0.0, "volume {v} not positive");
    assert!((v - exact).abs() < 0.1, "volume {v} vs exact {exact}");
}
