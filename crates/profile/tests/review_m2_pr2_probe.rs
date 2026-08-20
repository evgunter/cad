//! The Probe-lane half of `review_m2_pr2.rs` (adversarial e2e review
//! artifact for M2 PR 2): the K-hook section — canonical output and
//! error outcomes at the recording scalar are bit-identical to f64,
//! which is what makes `Probe` a delegating wrapper rather than a
//! second arithmetic.
//!
//! Split out because `Probe` — the K-telemetry recording scalar — is
//! gated behind the `probe` cargo feature: it is a `Real` instantiation,
//! so every generic-over-`Real` body monomorphizes at it, and the
//! default build has no reason to pay for a diagnostics scalar. The
//! other 19 tests of the review artifact stay ungated in
//! `review_m2_pr2.rs`; only this file carries the whole-file gate.
//!
//! **CI COMPILES THIS SUITE AND DOES NOT RUN IT.** The probe suites CI
//! executes are rostered in `scripts/gates/probe-suite-census.sh`
//! (`RUN_FLOOR`) and run by `scripts/k_probe_sweep.sh`; this one is on
//! neither list, so nothing here can go red on a merge and its assertions
//! are evidence for a reader rather than a gate. By hand:
//! `cargo test -p profile --features probe --test all -- review_m2_pr2_probe::`.

#![cfg(feature = "probe")]
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]

mod common;

use common::{lift, profile, tol};
use geom_core::k_stats::{self, Probe};
use profile::SegmentKind;

// ------------------------------------------------------------ K-hook --

/// Probe canonical output is bit-identical to f64 (delegation), on an
/// arc-heavy fixture.
#[test]
fn probe_canonical_form_is_bit_identical_to_f64() {
    let fixtures = [
        profile(vec![common::rounded_rect(4.0, 3.0, 0.5)]),
        common::annulus(),
        profile(vec![common::lens()]),
    ];
    for p in fixtures {
        let f = p.validate(tol()).expect("f64 validates");
        let q = lift::<Probe>(&p).validate(tol()).expect("probe validates");
        let fb: Vec<(u64, u64, u64)> = f
            .loops()
            .iter()
            .flat_map(|l| l.vertices().iter())
            .map(|v| {
                (
                    v.pos().x.to_bits(),
                    v.pos().y.to_bits(),
                    v.bulge().to_bits(),
                )
            })
            .collect();
        let qb: Vec<(u64, u64, u64)> = q
            .loops()
            .iter()
            .flat_map(|l| l.vertices().iter())
            .map(|v| {
                (
                    v.pos().x.0.to_bits(),
                    v.pos().y.0.to_bits(),
                    v.bulge().0.to_bits(),
                )
            })
            .collect();
        assert_eq!(fb, qb);
        // And identical arc geometry down to carrier bits.
        for (lf, lq) in f.loops().iter().zip(q.loops()) {
            for (sf, sq) in lf.segments().iter().zip(lq.segments()) {
                match (&sf.kind, &sq.kind) {
                    (
                        SegmentKind::Arc {
                            center: cf,
                            radius: rf,
                            turn: tf,
                        },
                        SegmentKind::Arc {
                            center: cq,
                            radius: rq,
                            turn: tq,
                        },
                    ) => {
                        assert_eq!(cf.x.to_bits(), cq.x.0.to_bits());
                        assert_eq!(cf.y.to_bits(), cq.y.0.to_bits());
                        assert_eq!(rf.to_bits(), rq.0.to_bits());
                        assert_eq!(tf, tq);
                    }
                    (SegmentKind::Line, SegmentKind::Line) => {}
                    other => panic!("kind divergence: {other:?}"),
                }
            }
        }
    }
}

/// Probe error outcomes equal f64's exactly on rejecting fixtures.
#[test]
fn probe_errors_equal_f64_errors() {
    k_stats::start_recording();
    for p in [
        profile(vec![common::bowtie()]),
        common::tangent_hole(),
        common::near_tangent_hole(tol().eps),
    ] {
        let f = p.validate(tol()).expect_err("rejects at f64");
        let q = lift::<Probe>(&p)
            .validate(tol())
            .expect_err("rejects at Probe");
        assert_eq!(f, q);
    }
    let samples = k_stats::take_samples();
    assert!(!samples.is_empty());
}
