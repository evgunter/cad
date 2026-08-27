//! Cross-scalar decision consistency: validation at `Dual<f64>` makes
//! bit-identical decisions to plain `f64` (value-part delegation — a
//! derivative never influences a branch).
//!
//! The other half of the cross-scalar story — the k-stats `Probe` scalar
//! recording the margin distribution without changing any decision —
//! lives in `scalar_channels_probe.rs`, gated on the `probe` feature.
//! The `Dual<f64>` tests here stay in the default build.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{
    annulus, arc_kisses_line, bowtie, lift, near_tangent_hole, profile, tangent_hole, tol,
};
use geom_core::{Dual, Dual64, Sign};
use profile::RawLoop;
use profile::{LoopRole, SegmentKind, ValidatedProfile};

/// The decision skeleton of a canonical form: roles, per-segment kind
/// and turn, and the value-channel bits of every canonical vertex — the
/// part that must agree bit-for-bit across scalar instantiations.
type Skeleton = Vec<(bool, Vec<(u64, u64, u64, i8)>)>;

fn skeleton_f64(vp: &ValidatedProfile<f64>) -> Skeleton {
    vp.loops()
        .iter()
        .map(|lp| {
            (
                lp.role() == LoopRole::Outer,
                lp.vertices()
                    .iter()
                    .zip(lp.segments())
                    .map(|(v, s)| {
                        (
                            v.pos().x.to_bits(),
                            v.pos().y.to_bits(),
                            v.bulge().to_bits(),
                            kind_code(&s.kind),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn skeleton_dual(vp: &ValidatedProfile<Dual64>) -> Skeleton {
    vp.loops()
        .iter()
        .map(|lp| {
            (
                lp.role() == LoopRole::Outer,
                lp.vertices()
                    .iter()
                    .zip(lp.segments())
                    .map(|(v, s)| {
                        (
                            v.pos().x.value.to_bits(),
                            v.pos().y.value.to_bits(),
                            v.bulge().value.to_bits(),
                            dual_kind_code(&s.kind),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn turn_code(turn: Sign) -> i8 {
    match turn {
        Sign::Positive => 1,
        Sign::Negative => -1,
        Sign::Zero => 0,
    }
}

fn kind_code(k: &SegmentKind<f64>) -> i8 {
    match k {
        SegmentKind::Line => 0,
        SegmentKind::Arc { turn, .. } => turn_code(*turn),
    }
}

fn dual_kind_code(k: &SegmentKind<Dual64>) -> i8 {
    match k {
        SegmentKind::Line => 0,
        SegmentKind::Arc { turn, .. } => turn_code(*turn),
    }
}

#[test]
fn dual_accepts_exactly_like_f64_with_identical_canonical_values() {
    let base = annulus();
    let f = base.validate(tol()).expect("annulus validates at f64");
    let d = lift::<Dual64>(&base)
        .validate(tol())
        .expect("annulus validates at Dual<f64>");
    assert_eq!(skeleton_f64(&f), skeleton_dual(&d));
}

#[test]
fn dual_rejects_with_the_identical_typed_error() {
    // ProfileError is scalar-free, so the errors must be *equal* —
    // including the escalation's margin diagnostic (the value channel
    // is bit-identical by construction).
    for fixture in [
        profile(vec![bowtie()]),
        tangent_hole(),
        near_tangent_hole(tol().eps()),
        arc_kisses_line(),
    ] {
        let f_err = fixture.validate(tol()).expect_err("fixture rejects at f64");
        let d_err = lift::<Dual64>(&fixture)
            .validate(tol())
            .expect_err("fixture rejects at Dual<f64>");
        assert_eq!(f_err, d_err);
    }
}

#[test]
fn dual_with_seeded_derivatives_still_decides_by_value_only() {
    // Poisoned/adversarial derivative channels must not affect any
    // decision: seed every coordinate's derivative with NaN.
    let base = annulus();
    let seeded = profile::Profile::new(
        profile::SketchPlane::xy(),
        base.loops
            .iter()
            .map(|lp| {
                profile::ProfileLoop::new(
                    lp.vertices()
                        .iter()
                        .map(|v| {
                            profile::ProfileVertex::new(
                                geom_core::Point2::new(
                                    Dual::new(v.pos().x, f64::NAN),
                                    Dual::new(v.pos().y, f64::NAN),
                                ),
                                Dual::new(v.bulge(), f64::NAN),
                            )
                        })
                        .collect(),
                )
            })
            .collect(),
    );
    let f = base.validate(tol()).expect("annulus validates at f64");
    let d = seeded
        .validate(tol())
        .expect("NaN derivatives must not poison decisions");
    assert_eq!(skeleton_f64(&f), skeleton_dual(&d));
}
