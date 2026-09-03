//! Issue 1362's end-to-end gate: a rimless pole-to-pole band walked
//! through the PUBLIC doors must mesh the same body wherever that body
//! sits. `walk`'s band_u arm picks each meridian's `2πk` branch from
//! the loop's 3-D area vector, so a fold that reads cancellation noise
//! at a large placement does not refuse — it meshes the complementary
//! half of the sphere and hands back a WATERTIGHT mesh whose volume is
//! wrong by orders of magnitude. That is the failure this row exists
//! to catch, and it is silent by construction: `check_mesh` passes.
//!
//! The reachable corner, which the unit rows alone do not demonstrate:
//! the door is an ABSOLUTE test (coordinate ulp against ε) while the
//! defect is a RATIO test (placement over body size). A SMALL body at a
//! MODERATE distance clears one and trips the other — `r = 1e-6` at
//! `d = 1e3` has `ulp(1e3) ≈ 1.1e-13` yet a ratio of ~1.7e9.
//!
//! Tolerance posture: the ASSERTIONS consult no kernel tolerance — the
//! measured volume is compared to `4πr³/3` with a budget argued from
//! the chord count, and to the same body's own volume at the origin.
//! The FIXTURE rides `Tol::witness()` through profile/revolve/
//! tessellate, and CI varies that band via `CAD_TOLERANCE_EPS`; a band
//! whose doors refuse the placement skips that row LOUDLY, printing the
//! typed refusal and its stage. A panic anywhere is a finding.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use mesh::tessellate;
use mesh::validate::{check_mesh, signed_volume};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};

/// The ball of radius `r` centred at `(d, d, d)`: a half-disc revolved
/// a full turn, so BOTH faces are rimless pole-to-pole bands and both
/// take their columns from `walk`'s area-vector fold. Built through the
/// public doors only; a typed refusal is returned, never unwrapped.
fn placed_ball(r: f64, d: f64) -> Result<topo::Body<f64>, String> {
    let plane = SketchPlane::from_frame(
        Point3::new(d, d, d),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(0.0, -r), 1.0),
        ProfileVertex::new(Point2::new(0.0, r), 0.0),
    ]);
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .map_err(|e| format!("profile validate: {e:?}"))?;
    revolve(
        &vp,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .map(|x| x.body)
    .map_err(|e| format!("revolve: {e:?}"))
}

/// Mesh the ball and measure it, or report the typed upstream refusal
/// and yield `None`. Watertightness is asserted, not reported: a mesh
/// that fails `check_mesh` is a different bug and must not be quietly
/// skipped as if the door had refused it.
fn volume_or_typed(r: f64, d: f64, label: &str) -> Option<f64> {
    let body = match placed_ball(r, d) {
        Ok(b) => b,
        Err(e) => {
            println!("{label}: typed refusal upstream of the mesh: {e}");
            return None;
        }
    };
    let m = match tessellate(&body, r / 20.0, Tol::witness()) {
        Ok(m) => m,
        Err(e) => {
            println!("{label}: typed refusal at tessellate: {e:?}");
            return None;
        }
    };
    assert_eq!(check_mesh(&m), Ok(()), "{label}: mesh not watertight");
    Some(signed_volume(&m))
}

/// **Issue 1362, end to end.** A 1 µm ball at a 1 km placement — a
/// fixture the doors admit at the default band — must measure the same
/// volume it measures at the origin, and must measure the sphere.
///
/// Budgets, both argued rather than fitted. `4πr³/3` is the exact
/// volume; the mesh is INSCRIBED at `δ = r/20`, so it under-measures by
/// a chord defect of a few percent — `0.25` relative is loose enough to
/// pass any honest tessellation of a sphere at this budget and tight
/// enough that the defect cannot hide in it. Origin-vs-placed agreement
/// is held to `1e-6` relative: the placement enters the positions
/// (`ulp(1e3)/1e-6 ≈ 1.1e-7` relative per coordinate), so agreement
/// beyond that is not available from the inputs.
///
/// Under the world-origin-anchored fold this row does not merely drift:
/// the area vector at this placement is pure cancellation noise, the
/// meridians take the wrong `2πk` branch, and the still-watertight mesh
/// measures ~2.3e-34 m³ against an exact 4.19e-18 m³ — sixteen orders,
/// with no refusal anywhere.
#[test]
fn a_micron_ball_at_a_kilometre_meshes_the_same_body_it_does_at_the_origin() {
    let r = 1.0e-6;
    let exact = 4.0 * core::f64::consts::PI * r * r * r / 3.0;
    let Some(v0) = volume_or_typed(r, 0.0, "origin") else {
        return;
    };
    let Some(v) = volume_or_typed(r, 1.0e3, "placed 1e3") else {
        return;
    };
    println!("origin {v0:e}, placed {v:e}, exact {exact:e}");
    for (label, measured) in [("origin", v0), ("placed", v)] {
        let rel = ((measured - exact) / exact).abs();
        assert!(
            rel < 0.25,
            "{label}: volume {measured:e} against exact {exact:e} \
             (relative error {rel:e}) — the band took a branch that is not \
             the sphere's"
        );
    }
    let drift = ((v - v0) / v0).abs();
    assert!(
        drift < 1e-6,
        "placed volume {v:e} vs {v0:e} at the origin (relative drift \
         {drift:e}): the same body meshed differently for having been \
         moved, which is the defect issue 1362 names"
    );
}

/// The same corner, swept: the ratio is what matters, not the distance.
/// Every row whose doors admit it must agree with its own origin build.
/// Rows the band refuses print and skip, so each ε gates the fixtures
/// its doors let through.
#[test]
fn the_band_survives_every_placement_ratio_the_doors_admit() {
    for (r, d) in [
        (1.0e-2, 1.0e4),
        (1.0e-3, 1.0e4),
        (1.0e-4, 1.0e4),
        (1.0e-5, 1.0e3),
        (1.0e-6, 1.0e3),
    ] {
        let ratio = d * 3.0_f64.sqrt() / r;
        let label = format!("r {r:e} d {d:e} (ratio {ratio:e})");
        let (Some(v0), Some(v)) = (
            volume_or_typed(r, 0.0, &format!("{label} origin")),
            volume_or_typed(r, d, &label),
        ) else {
            continue;
        };
        let drift = ((v - v0) / v0).abs();
        println!("{label}: origin {v0:e}, placed {v:e}, drift {drift:e}");
        assert!(
            drift < 1e-6,
            "{label}: volume moved by {drift:e} for a translation alone"
        );
    }
}
