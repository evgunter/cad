//! M9-3 unit review R1 — adversarial probes (blinded lane).
//!
//! Attacks: the C8 boundary under novel configurations, the two-peg
//! oracle re-derived and perturbed, the
//! zip's ring bookkeeping under nested/mismatched rings, partial
//! engagement, and the tube-chain additivity bound measured.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::{plate6, plate6_cyl};
use geom_core::Point3;
use geom_core::{Point2, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::test_support::{brick, finished};
use sweep::{Extrusion, extrude};
use topo::{AtRestBody, BooleanError, BooleanResult, mass_properties};

fn body_of(r: BooleanResult<f64>) -> AtRestBody<f64> {
    match r {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("operand cannot be empty"),
    }
}

/// [`plate6`], finished.
fn plate(z0: f64) -> AtRestBody<f64> {
    finished("the plate", plate6(z0), Tol::witness())
}

/// [`plate6_cyl`], finished.
fn cyl(cx: f64, z0: f64, h: f64, r: f64) -> AtRestBody<f64> {
    finished("the cylinder", plate6_cyl(cx, z0, h, r), Tol::witness())
}

fn plate_with_pegs(cx1: f64, cx2: f64, r: f64) -> AtRestBody<f64> {
    let p0 = plate(0.0);
    let p1 = body_of(topo::union(&p0, &cyl(cx1, 0.4, 1.6, r), Tol::witness()).unwrap());
    body_of(topo::union(&p1, &cyl(cx2, 0.4, 1.6, r), Tol::witness()).unwrap())
}

fn plate_with_bores() -> AtRestBody<f64> {
    let q0 = plate(1.0);
    let q1 = body_of(topo::subtract(&q0, &cyl(2.0, 0.8, 1.4, 0.5), Tol::witness()).unwrap());
    body_of(topo::subtract(&q1, &cyl(4.0, 0.8, 1.4, 0.5), Tol::witness()).unwrap())
}

/// Oracle re-derivation: the plate pair is 6·4·1 each = 24+24 = 48 and
/// the peg/bore π terms cancel IN REAL ARITHMETIC. The PR claims the
/// closed form is exactly 48 — check both the closed form and the
/// bitwise chain vp + vq == v == 48.
#[test]
fn probe_two_peg_oracle_rederivation() {
    let p = plate_with_pegs(2.0, 4.0, 0.5);
    let q = plate_with_bores();
    let vp = mass_properties(
        &p,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    let vq = mass_properties(
        &q,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    let body = body_of(topo::union(&p, &q, Tol::witness()).unwrap());
    let v = mass_properties(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    eprintln!(
        "vp = {vp:.17e}, vq = {vq:.17e}, vp+vq = {:.17e}, v = {v:.17e}",
        vp + vq
    );
    assert_eq!(v, vp + vq, "bitwise additivity");
    assert_eq!(v, 48.0, "the closed form is exactly 48 (PR claim)");
}

/// Perturbation: peg 2 offset by ~2 ulp of 4.0 (in the zero band).
/// C4 bridges in-band residues under a verified pair — the
/// probe records which arm actually fires and that nothing SILENTLY
/// wrong emerges (either a typed refusal, or a union whose volume is
/// still additive to dust).
#[test]
fn probe_peg_offset_one_ulp_characterized() {
    let ulp = 4.0_f64 * f64::EPSILON; // ~8.9e-16
    let p = plate_with_pegs(2.0, 4.0 + ulp, 0.5);
    let q = plate_with_bores();
    let vp = mass_properties(
        &p,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    let vq = mass_properties(
        &q,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    match topo::union(&p, &q, Tol::witness()) {
        Ok(out) => {
            let body = body_of(out);
            let v = mass_properties(
                &body,
                Tol::witness(),
                topo::test_support::pinned_resolution(Tol::witness()),
            )
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume;
            eprintln!(
                "1-ulp offset unioned: v = {v:.17e}, vp+vq = {:.17e}",
                vp + vq
            );
            assert!(
                (v - (vp + vq)).abs() < 1e-12,
                "bridged union must stay additive to dust: {v} vs {}",
                vp + vq
            );
            if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
                panic!("bridged union must stay tier-3 valid: {errs:?}");
            }
        }
        Err(err) => {
            eprintln!("1-ulp offset refused: {err:?}");
            assert!(
                matches!(
                    err,
                    BooleanError::Escalated { .. }
                        | BooleanError::ContactContradicted { .. }
                        | BooleanError::CurvedPierceUnsupported { .. }
                ),
                "typed only: {err:?}"
            );
        }
    }
}

/// Partial engagement: a peg reaching only halfway up its through-bore
/// (the bore is NOT fully engaged), resting on the walls and the mate:
/// whatever happens must be a typed refusal or an exactly-
/// additive union — never a silently wrong body.
#[test]
fn probe_partial_engagement_never_silent() {
    let p = {
        let p0 = plate(0.0);
        body_of(topo::union(&p0, &cyl(2.0, 0.4, 1.1, 0.5), Tol::witness()).unwrap())
    };
    let q = {
        let q0 = plate(1.0);
        body_of(topo::subtract(&q0, &cyl(2.0, 0.8, 1.4, 0.5), Tol::witness()).unwrap())
    };
    let vp = mass_properties(
        &p,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    let vq = mass_properties(
        &q,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    match topo::union(&p, &q, Tol::witness()) {
        Ok(out) => {
            let body = body_of(out);
            let v = mass_properties(
                &body,
                Tol::witness(),
                topo::test_support::pinned_resolution(Tol::witness()),
            )
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume;
            eprintln!(
                "partial engagement unioned: v = {v:.17e} vs {:.17e}",
                vp + vq
            );
            // Exact additivity on the 4-ULP relative oracle the MATE-2
            // suites use (`mate2_common::assert_additive`): the π terms
            // of the peg and the bore do not cancel, so a bitwise pin
            // is not available here. This branch first RAN when the
            // merge door stopped refusing the cylindrical glued pair; first measured 0.672 ULP (the sibling scenes
            // measure ~1.5 ULP).
            crate::mate2_common::assert_additive(v, vp, vq);
            if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
                panic!("must be tier-3 valid: {errs:?}");
            }
        }
        Err(err) => {
            eprintln!("partial engagement refused: {err:?}");
            assert!(
                matches!(
                    err,
                    BooleanError::Join(_)
                        | BooleanError::CurvedPierceUnsupported { .. }
                        | BooleanError::CurvedBooleanUnsupported { .. }
                ),
                "typed only: {err:?}"
            );
        }
    }
}

/// A plate with `holes` square through-holes (two-ring patches when
/// stacked): the ring-capable glue must handle TWO rings per patch
/// face, and the volume must be exactly additive.
fn holed_plate(z0: f64, z1: f64, holes: &[(f64, f64)]) -> AtRestBody<f64> {
    let mut b = finished(
        "the plate",
        brick((0.0, 6.0), (0.0, 3.0), (z0, z1), Tol::witness()),
        Tol::witness(),
    );
    for &(hx, hy) in holes {
        b = body_of(
            topo::subtract(
                &b,
                &finished(
                    "the hole's brick",
                    brick(
                        (hx, hx + 1.0),
                        (hy, hy + 1.0),
                        (z0 - 0.5, z1 + 0.5),
                        Tol::witness(),
                    ),
                    Tol::witness(),
                ),
                Tol::witness(),
            )
            .unwrap(),
        );
    }
    b
}

/// Two stacked plates each with TWO square through-holes: the contact
/// patch face carries two rings on each side. Exact additivity or bust.
#[test]
fn probe_two_ring_patch_unions_exactly() {
    let holes = [(1.0, 1.0), (4.0, 1.0)];
    let bot = holed_plate(0.0, 1.0, &holes);
    let top = holed_plate(1.0, 2.0, &holes);
    let vb = mass_properties(
        &bot,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    let vt = mass_properties(
        &top,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    let out = topo::union(&bot, &top, Tol::witness()).expect("the two-ring patch union runs");
    let body = body_of(out);
    let v = mass_properties(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    assert_eq!(v, vb + vt, "exactly additive with two rings per patch");
    if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
        panic!("tier-3 valid: {errs:?}");
    }
}

/// Ring-count mismatch: the bottom plate has two holes, the top ONE.
/// The pair's patch faces are not congruent — must refuse typed or
/// decline to the original join refusal, never a silent wrong body.
#[test]
fn probe_ring_count_mismatch_never_silent() {
    let bot = holed_plate(0.0, 1.0, &[(1.0, 1.0), (4.0, 1.0)]);
    let top = holed_plate(1.0, 2.0, &[(1.0, 1.0)]);
    let vb = mass_properties(
        &bot,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    let vt = mass_properties(
        &top,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    match topo::union(&bot, &top, Tol::witness()) {
        Ok(out) => {
            // A legitimate union of this mate exists (the top caps one
            // hole); if the kernel produces it, it must be exact.
            let body = body_of(out);
            let v = mass_properties(
                &body,
                Tol::witness(),
                topo::test_support::pinned_resolution(Tol::witness()),
            )
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume;
            eprintln!("ring mismatch unioned: v = {v:.17e}");
            assert_eq!(v, vb + vt, "if it unions it must be exactly additive");
            if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
                panic!("tier-3 valid: {errs:?}");
            }
        }
        Err(err) => {
            eprintln!("ring mismatch refused: {err:?}");
            assert!(
                matches!(
                    err,
                    BooleanError::Join(_) | BooleanError::ZipCorrespondence { .. }
                ),
                "typed only: {err:?}"
            );
        }
    }
}

// ---- tube-chain rim (acceptance ii) attacks ----

fn lying_plane() -> SketchPlane<f64> {
    SketchPlane::from_frame(geom_core::RigidMotion::axes_zx(Point3::new(0.0, 0.0, 0.0)))
}

fn lying_extrude(vertices: Vec<(Point2<f64>, f64)>) -> AtRestBody<f64> {
    let profile = Profile::new(lying_plane(), vec![bulge_loop(vertices)])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: 4.0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body;
    finished("the lying extrusion", body, Tol::witness())
}

fn quarter_round_below() -> AtRestBody<f64> {
    let b90 = (core::f64::consts::PI / 8.0).tan();
    lying_extrude(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 2.0), b90),
        (Point2::new(0.0, 3.0), 0.0),
    ])
}

fn quarter_round_above() -> AtRestBody<f64> {
    let b90 = (core::f64::consts::PI / 8.0).tan();
    lying_extrude(vec![
        (Point2::new(1.0, 0.5), 0.0),
        (Point2::new(1.0, 2.0), -b90),
        (Point2::new(2.0, 3.0), 0.0),
        (Point2::new(3.0, 3.0), 0.0),
        (Point2::new(3.0, 0.5), 0.0),
    ])
}

/// Measure the actual additivity error on the rim fixture — is the
/// 1e-12 bound honest headroom or a fitted constant?
#[test]
fn probe_tube_chain_additivity_error_measured() {
    let a = quarter_round_below();
    let b = quarter_round_above();
    let va = mass_properties(
        &a,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    let vb = mass_properties(
        &b,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    let body = body_of(topo::union(&a, &b, Tol::witness()).unwrap());
    let v = mass_properties(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    eprintln!(
        "tube-chain: va = {va:.17e}, vb = {vb:.17e}, v = {v:.17e}, err = {:.3e}",
        (v - (va + vb)).abs()
    );
    assert!((v - (va + vb)).abs() < 1e-12);
}
