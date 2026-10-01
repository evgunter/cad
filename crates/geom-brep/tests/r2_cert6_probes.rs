//! **R2 review probes for CERT-6 (PR 1366, frozen head ce567bad).**
//!
//! Own fixtures through the props public door `nurbs_patch_face`: a
//! flat-square perimeter anchor, and the fallback-arm reachability
//! question — can a face whose whole rectangle boundary collapses to a
//! point reach a certified return (PR body: "per-face, not per-lane"
//! fallback with zero corpus coverage).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::ring::p3;
use crate::shared::tol::band;
use geom_brep::props::quad::nurbs_patch_face;
use geom_core::Bounds;
use geom_core::Tol;
use geom_core::interval::certification::Certification;
use geom_core::spline::KnotVector;

/// Perimeter-bound soundness anchor: a FLAT unit square patch has true
/// boundary perimeter exactly 4. The certified lower bound must sit at
/// or below 4 and, with straight edges (chords lie ON the curve),
/// within outward rounding of it. Read via the R2_GAUGE_TRACE line; this
/// row asserts through the door only that the face certifies.
#[test]
fn r2_flat_square_perimeter_anchor() {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut net = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            #[allow(clippy::cast_precision_loss)]
            let (x, y) = (i as f64 / 2.0, j as f64 / 2.0);
            net.push(p3(x, y, 0.0));
        }
    }
    let out = nurbs_patch_face::<f64>(
        &k,
        &k,
        &net,
        &[1.0; 9],
        (0.0, 1.0, 0.0, 1.0),
        4.0,
        0.0,
        Tol::witness().get().eps,
        band(),
    )
    .expect("the flat square certifies");
    println!(
        "R2 flat square: area [{:e},{:e}] (true 1)",
        out.area.lo(),
        out.area.hi()
    );
}

/// Claim 5's reachability question: a "balloon" patch whose ENTIRE
/// rectangle boundary maps to one point (all boundary control points
/// coincide), with real interior area. If this reaches a certified
/// return, the relative-gauge fallback arm is live; if it refuses,
/// the fallback is dead code at today's call sites. The probe records
/// the posture rather than asserting one.
#[test]
fn r2_collapsed_boundary_balloon_probe() {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let n = k.control_count(); // 4
    let mut net = Vec::new();
    for i in 0..n {
        for j in 0..n {
            let boundary = i == 0 || j == 0 || i == n - 1 || j == n - 1;
            if boundary {
                net.push(p3(0.0, 0.0, 0.0));
            } else {
                #[allow(clippy::cast_precision_loss)]
                let (x, y) = (i as f64 - 1.5, j as f64 - 1.5);
                net.push(p3(3.0 * x, 3.0 * y, 2.0));
            }
        }
    }
    let (a, b) = k.domain();
    let out = nurbs_patch_face::<f64>(
        &k,
        &k,
        &net,
        &vec![1.0; n * n],
        (a, b, a, b),
        0.0,
        0.0,
        Tol::witness().get().eps,
        band(),
    );
    match out {
        Ok(fb) => println!(
            "R2 balloon: CERTIFIED, area [{:e},{:e}] width {:e} — fallback arm was LIVE if the \
             chord perimeter was 0",
            fb.area.lo(),
            fb.area.hi(),
            fb.area.width()
        ),
        Err(e) => println!("R2 balloon: refused {e} — fallback not reached through this shape"),
    }
}
