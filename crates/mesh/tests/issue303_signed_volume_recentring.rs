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
//! `CAD_TOLERANCE_EPS` — at ε = 1e-12 the invariance row's in-box
//! offset placement may refuse TYPED at extrude (`ResidualExceeded`:
//! placement coordinate ulps, ~1.1e-13 at 900 m, sit at band scale),
//! which is the loud path — a door refusal, never a silently different
//! volume — and its comparison is skipped and printed. Every placement
//! past the session box (1e3, 1e6 and 1e8 m on every axis) is pinned to
//! the box's refusal at its first edge, at every ε.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, RigidMotion, Tol};
use mesh::validate::{check_mesh, signed_volume};
use mesh::{Mesh, tessellate};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::ExtrudeSide;
use sweep::{Extrusion, extrude};

fn plane_at(offset: f64) -> SketchPlane<f64> {
    SketchPlane::from_frame(RigidMotion::axes_xy(Point3::new(offset, offset, offset)))
}

/// **The prism on `plane` stands past the session box, and its extrude
/// says so**: refused at its first edge's certification, by the box, and
/// never by a residual.
fn refused_by_the_session_box(plane: SketchPlane<f64>, poly: &[(f64, f64)], h: f64, label: &str) {
    let lp = ProfileLoop::polygon(poly.iter().map(|&(x, y)| Point2::new(x, y)));
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("a valid profile");
    let got = extrude(
        &vp,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    );
    let said = format!("{got:?}");
    assert!(
        (said.starts_with("Err(Op { source: DescriptionUncertified {")
            && said.contains("error: OutsideSessionBox {"))
            && !said.contains("ResidualExceeded"),
        "{label}: past the session box, refused by it at its first edge: {said}"
    );
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
    let body = match extrude(
        &vp,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    ) {
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
/// of magnitude. At 1e8 m the cube is past the session box, so the
/// extrude refuses it by the box at every ε and no mesh is made: the row
/// pins that refusal, and the oracle's own recentring is pinned in the box
/// by `translation_invariance_1e3_1e6`.
#[test]
fn huge_offset_volume_through_public_oracle() {
    let poly = [(0.0, 0.0), (1.0e-3, 0.0), (1.0e-3, 1.0e-3), (0.0, 1.0e-3)];
    refused_by_the_session_box(plane_at(1.0e8), &poly, 1.0e-3, "huge-offset");
}

/// Translation invariance, pinned: the same 2.7 x 1.3 x 0.9 prism
/// (non-dyadic edges, so the offset positions round — a dyadic
/// axis-aligned box cancels its fold terms pairwise-exactly and hides
/// the defect) measured at the origin and at a 900 m offset must
/// agree relatively to ~1e2 x ulp(offset)/edge — the offset enters the
/// positions themselves (each coordinate rounded near the offset's
/// magnitude, ulp(900) ~ 1.1e-13), so agreement
/// beyond that budget is not available from the inputs; within it, the
/// recentred fold's own rounding is subordinate. At 1e3 and 1e6 m the
/// prism reaches past the session box, and the extrude refuses it there,
/// by the box. A placement whose
/// build refuses typed at the current band skips its comparison
/// loudly (module docs: the ε premise); the assertions themselves
/// consult no tolerance.
#[test]
fn translation_invariance_1e3_1e6() {
    let poly = [(0.0, 0.0), (2.7, 0.0), (2.7, 1.3), (0.0, 1.3)];
    // A metre prism every band resolves: at the origin and 900 m out it
    // is meshed, never stood down.
    let m0 = mesh_or_typed(SketchPlane::xy(), &poly, 0.9, 1e-2, "origin")
        .expect("the prism at the origin meshes");
    assert_eq!(check_mesh(&m0), Ok(()));
    let v0 = signed_volume(&m0);
    assert!((v0 - 3.159).abs() < 1e-12, "origin volume {v0}");
    for offset in [1.0e3, 1.0e6] {
        refused_by_the_session_box(plane_at(offset), &poly, 0.9, &format!("offset {offset:e}"));
    }
    {
        let (offset, tol) = (9.0e2, 1e-11);
        let label = format!("offset {offset:e}");
        let m = mesh_or_typed(plane_at(offset), &poly, 0.9, 1e-2, &label)
            .unwrap_or_else(|| panic!("{label}: the prism inside the box meshes"));
        assert_eq!(check_mesh(&m), Ok(()), "{label}: not watertight");
        let v = signed_volume(&m);
        let rel = ((v - v0) / v0).abs();
        assert!(
            rel < tol,
            "{label}: volume {v} vs {v0} (relative drift {rel:e}, budget {tol:e})"
        );
    }
}
