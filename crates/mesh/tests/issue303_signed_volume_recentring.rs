//! Issue 303 gate: `mesh::validate::signed_volume` must measure a
//! body's volume at the relative precision the coordinates allow, no
//! matter where the body sits — the divergence sum of a closed mesh is
//! translation-invariant over ℝ, so placement must not buy
//! cancellation. Two rows: the issue's own huge-offset probe read
//! through the PUBLIC oracle, and a translation-invariance row pinning
//! origin-vs-offset agreement.
//!
//! Tolerance posture, split precisely: the ASSERTIONS consult no
//! kernel tolerance — pure f64 comparisons against ulp-argued budgets.
//! The FIXTURES ride `Tol::witness()` through
//! validate/extrude/tessellate, and CI varies that band via
//! `CAD_TOLERANCE_EPS` — at ε = 1e-12 the invariance row's offset
//! placements refuse TYPED at extrude (`ResidualExceeded`: placement
//! coordinate ulps, ~1.1e-13 at 1e3 m, sit at band scale), which is
//! the loud path — a door refusal, never a silently different volume.
//! The rows accept-and-print typed refusals, so each band gates the
//! fixtures its doors admit. Local three-ε sweep (default 1e-9 /
//! 1e-6 / 1e-12): huge-offset row builds and passes at all three;
//! invariance row builds and passes at default and 1e-6, refuses
//! typed at 1e-12.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use mesh::validate::{check_mesh, signed_volume};
use mesh::{Mesh, tessellate};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};

fn plane_at(offset: f64) -> SketchPlane<f64> {
    SketchPlane::from_frame(
        Point3::new(offset, offset, offset),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    )
}

/// Build the prism's mesh at chord tolerance `delta`, or report the
/// typed upstream refusal and yield `None` — the tight-band outcome
/// for offset placements (module docs). Every refusal is printed with
/// its stage; a panic anywhere is a finding.
fn mesh_or_typed(
    plane: SketchPlane<f64>,
    poly: &[(f64, f64)],
    h: f64,
    delta: f64,
    label: &str,
) -> Option<Mesh> {
    let lp = ProfileLoop::polygon(poly.iter().map(|&(x, y)| Point2::new(x, y)));
    let vp = match Profile::new(plane, vec![lp]).validate(Tol::witness()) {
        Ok(v) => v,
        Err(e) => {
            println!("{label}: typed refusal at profile validation: {e:?}");
            return None;
        }
    };
    let body = match extrude(&vp, Extrusion::Distance(h), Tol::witness()) {
        Ok(x) => x.body,
        Err(e) => {
            println!("{label}: typed refusal at extrude: {e:?}");
            return None;
        }
    };
    match tessellate(&body, delta, Tol::witness()) {
        Ok(m) => Some(m),
        Err(e) => {
            println!("{label}: typed refusal at tessellate: {e:?}");
            None
        }
    }
}

/// Issue 303's own case (probe (d) of the #301 review, now demanded of
/// the public oracle itself — probe_d in `newell_probes.rs` keeps the
/// same fixture through its own harness; THIS row is the defect's
/// dedicated gate): a 1e-3 cube at a 1e8 m offset on all axes. True
/// volume 1e-9 m³. Tolerance: the positions are ~1e8 with ulp ~1.5e-8
/// against 1e-3 edges — ~1.5e-5 relative slop per coordinate is
/// already in the INPUTS, so 1e-4 relative is the honest bar; a
/// world-origin-anchored fold (terms ~1e24) misses it by ten orders
/// of magnitude. Builds at all three swept ε bands today; a band that
/// refuses it would do so typed, and the row would say so.
#[test]
fn huge_offset_volume_through_public_oracle() {
    let poly = [(0.0, 0.0), (1.0e-3, 0.0), (1.0e-3, 1.0e-3), (0.0, 1.0e-3)];
    let Some(m) = mesh_or_typed(plane_at(1.0e8), &poly, 1.0e-3, 1e-6, "huge-offset") else {
        return;
    };
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
/// recentred fold's own rounding is subordinate. A placement whose
/// build refuses typed at the current band skips its comparison
/// loudly (module docs: the ε premise); the assertions themselves
/// consult no tolerance.
#[test]
fn translation_invariance_1e3_1e6() {
    let poly = [(0.0, 0.0), (2.7, 0.0), (2.7, 1.3), (0.0, 1.3)];
    let Some(m0) = mesh_or_typed(SketchPlane::xy(), &poly, 0.9, 1e-2, "origin") else {
        return;
    };
    assert_eq!(check_mesh(&m0), Ok(()));
    let v0 = signed_volume(&m0);
    assert!((v0 - 3.159).abs() < 1e-12, "origin volume {v0}");
    for (offset, tol) in [(1.0e3, 1e-11), (1.0e6, 1e-8)] {
        let label = format!("offset {offset:e}");
        let Some(m) = mesh_or_typed(plane_at(offset), &poly, 0.9, 1e-2, &label) else {
            continue;
        };
        assert_eq!(check_mesh(&m), Ok(()), "{label}: not watertight");
        let v = signed_volume(&m);
        let rel = ((v - v0) / v0).abs();
        assert!(
            rel < tol,
            "{label}: volume {v} vs {v0} (relative drift {rel:e}, budget {tol:e})"
        );
    }
}
