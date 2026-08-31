//! Issue 303 gate: `mesh::validate::signed_volume` must measure a
//! body's volume at the relative precision the coordinates allow, no
//! matter where the body sits — the divergence sum of a closed mesh is
//! translation-invariant over ℝ, so placement must not buy
//! cancellation. Two rows: the issue's own huge-offset probe read
//! through the PUBLIC oracle, and a translation-invariance row pinning
//! origin-vs-offset agreement. Both rows are pure f64 — they consult
//! no kernel tolerance and are not ε-band-sensitive, so they carry no
//! per-band premises.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use mesh::tessellate;
use mesh::validate::{check_mesh, signed_volume};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;

fn prism_on(plane: SketchPlane<f64>, poly: &[(f64, f64)], h: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon(poly.iter().map(|&(x, y)| Point2::new(x, y)));
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("profile validation");
    extrude(&vp, Extrusion::Distance(h), Tol::witness())
        .expect("extrude")
        .body
}

fn plane_at(offset: f64) -> SketchPlane<f64> {
    SketchPlane::from_frame(
        Point3::new(offset, offset, offset),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    )
}

/// Issue 303's own case (probe (d) of the #301 review, now demanded of
/// the public oracle itself): a 1e-3 cube at a 1e8 m offset on all
/// axes. True volume 1e-9 m³. Tolerance: the positions are ~1e8 with
/// ulp ~1.5e-8 against 1e-3 edges — ~1.5e-5 relative slop per
/// coordinate is already in the INPUTS, so 1e-4 relative is the
/// honest bar; a world-origin-anchored fold (terms ~1e24) misses it
/// by ten orders of magnitude.
#[test]
fn huge_offset_volume_through_public_oracle() {
    let body = prism_on(
        plane_at(1.0e8),
        &[(0.0, 0.0), (1.0e-3, 0.0), (1.0e-3, 1.0e-3), (0.0, 1.0e-3)],
        1.0e-3,
    );
    let m = tessellate(&body, 1e-6, Tol::witness()).expect("tessellate");
    assert_eq!(check_mesh(&m), Ok(()), "huge-offset: mesh not watertight");
    let v = signed_volume(&m);
    let rel = ((v - 1.0e-9) / 1.0e-9).abs();
    assert!(
        rel < 1e-4,
        "huge-offset: volume {v} (relative error {rel:e})"
    );
}

/// Translation invariance, pinned: the same 2.7 x 1.3 x 0.9 prism
/// (non-dyadic edges, so the offset positions round — a dyadic
/// axis-aligned box cancels its fold terms pairwise-exactly and hides
/// the defect) measured at the origin and at 1e3 / 1e6 m offsets must
/// agree relatively to ~1e2 x ulp(offset)/edge — the offset enters the
/// positions themselves (each coordinate rounded near the offset's
/// magnitude, ulp(1e3) ~ 1.1e-13, ulp(1e6) ~ 1.2e-10), so agreement
/// beyond that budget is not available from the inputs; within it, the
/// recentred fold's own rounding is subordinate. Pure f64 row: no
/// tolerance consulted.
#[test]
fn translation_invariance_1e3_1e6() {
    let poly = [(0.0, 0.0), (2.7, 0.0), (2.7, 1.3), (0.0, 1.3)];
    let v0 = {
        let m = tessellate(&prism_on(SketchPlane::xy(), &poly, 0.9), 1e-2, Tol::witness())
            .expect("tessellate at origin");
        assert_eq!(check_mesh(&m), Ok(()));
        signed_volume(&m)
    };
    assert!((v0 - 3.159).abs() < 1e-12, "origin volume {v0}");
    for (offset, tol) in [(1.0e3, 1e-11), (1.0e6, 1e-8)] {
        let m = tessellate(&prism_on(plane_at(offset), &poly, 0.9), 1e-2, Tol::witness())
            .expect("tessellate at offset");
        assert_eq!(check_mesh(&m), Ok(()), "offset {offset:e}: not watertight");
        let v = signed_volume(&m);
        let rel = ((v - v0) / v0).abs();
        assert!(
            rel < tol,
            "offset {offset:e}: volume {v} vs {v0} (relative drift {rel:e}, budget {tol:e})"
        );
    }
}
