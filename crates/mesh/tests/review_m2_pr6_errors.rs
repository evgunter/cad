//! M2 PR 6 adversarial review — error paths (assignment 8): δ edge
//! values (incl. −0.0 and denormals) on curved bodies, honest
//! CertificateExceeded unreachability through the public API, and the
//! spade crossing pre-check (unreachable via validated profiles —
//! reachable only through the θ ∈ (3π/2, 2π) walk finding, pinned in
//! review_m2_pr6_walk_shapes).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{axis_y, ball, cone, donut, p2, validated, washer};
use mesh::{TessellateError, tessellate};
use profile::ProfileLoop;
use profile::RawLoop;
use sweep::{Revolution, revolve};

#[test]
fn survives_negative_zero_delta_refused() {
    // −0.0 is not > 0.0: must be refused, not treated as "very fine".
    match tessellate(&donut(), -0.0) {
        Err(TessellateError::InvalidChordalTolerance { value }) => {
            assert_eq!(value.to_bits(), (-0.0f64).to_bits());
        }
        other => panic!("expected refusal of -0.0, got {:?}", other.map(|_| ())),
    }
}

#[test]
fn survives_denormal_delta_typed_overflow_on_curved_bodies() {
    for body in [donut(), washer(), cone()] {
        match tessellate(&body, 5e-324) {
            Err(TessellateError::ResolutionOverflow { count }) => {
                assert!(count > 16_777_216.0);
            }
            other => panic!("expected ResolutionOverflow, got {:?}", other.map(|_| ())),
        }
    }
}

#[test]
fn survives_delta_fine_but_sane_still_tessellates() {
    // δ small but with counts far below 2^24 must succeed (the cap is
    // a sanity bound, not a usability cliff). NOTE (perf, measured in
    // review): tessellation time scales ≈ quadratically in point count
    // (spade insertion path) — washer at 1e-4/3e-6/1e-6 takes
    // 19ms/386ms/1.2s release; δ ≈ 1e-9 (n ≈ 2e5 per rim, well under
    // the cap) ran > 11 CPU-minutes without completing. The 2^24 cap
    // bounds allocation, not wall-clock. Those three timings are a
    // one-time release-build reading with nothing re-taking them; they
    // can have no guard because the assertion below is about REFUSAL vs
    // SUCCESS, not about duration, and a timing asserted in the suite
    // would be box-dependent. The δ this row passes is chosen for the
    // point-count regime, which the numbers only illustrate.
    let body = washer();
    let mesh = tessellate(&body, 1e-6);
    assert!(mesh.is_ok(), "fine-but-sane delta refused");
}

#[test]
fn survives_certificate_exceeded_unreachable_over_body_sweep() {
    // The implementer claims CertificateExceeded is honest fail-loud
    // for kernel defects, unreachable for valid bodies (sizing targets
    // δ/2; certificates check ≤ δ). Sweep bodies × δ hunting one.
    let extreme_torus = {
        // R ≫ r: the conservative (~24×) torus bound at its most
        // stressed relative to the grid heuristic.
        let lp = ProfileLoop::new(vec![
            profile::ProfileVertex::new(p2(10.0, -0.05), 1.0),
            profile::ProfileVertex::new(p2(10.0, 0.05), 1.0),
        ]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
            .unwrap()
            .body
    };
    let flat_cone = {
        // Nearly flat cone (half-angle → π/2) — cosα·sinα maximal
        // sensitivity region for the cone bound.
        let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(4.0, 0.2), p2(0.0, 0.2)]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
            .unwrap()
            .body
    };
    for body in [ball(), cone(), donut(), extreme_torus, flat_cone] {
        for delta in [3.0, 0.7, 0.09, 0.013] {
            match tessellate(&body, delta) {
                Ok(_) => {}
                Err(TessellateError::CertificateExceeded {
                    bound, requested, ..
                }) => panic!("CertificateExceeded reached: bound {bound} > {requested}"),
                Err(e) => panic!("unexpected error {e:?} at delta {delta}"),
            }
        }
    }
}

#[test]
fn survives_torus_wedge_outside_pole_window() {
    // Torus faces carry no poles: the θ ∈ (3π/2, 2π) pole-junction
    // window must NOT affect a partial donut.
    let lp = ProfileLoop::new(vec![
        profile::ProfileVertex::new(p2(2.0, -0.5), 1.0),
        profile::ProfileVertex::new(p2(2.0, 0.5), 1.0),
    ]);
    let body = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(2.0 * core::f64::consts::PI - 0.1),
    )
    .unwrap()
    .body;
    common::check_mesh_acceptance(&body, 0.08, None);
}

#[test]
fn survives_self_intersecting_profile_never_reaches_spade() {
    // The crossing-constraint panic is pre-checked in-crate; the only
    // way to feed spade crossing segments would be a self-intersecting
    // boundary, which profile validation refuses upstream — typed.
    let bowtie = ProfileLoop::polygon([p2(0.5, 0.0), p2(2.0, 1.0), p2(2.0, 0.0), p2(0.5, 1.0)]);
    let res = profile::Profile::new(profile::SketchPlane::xy(), vec![bowtie])
        .validate(geom_core::Tolerance::get());
    assert!(res.is_err(), "self-intersecting profile must be refused");
}
