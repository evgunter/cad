//! **A consistent table arc is never refused as inconsistent.** A table
//! whose arc is the bulge lowering's f64 output is off its vertices by
//! that lowering's rounding alone. Where the scene can read a
//! difference that small (`arc_carrier_resolution`, at each check's
//! stated rounding bound) the difference is inside the band; where it
//! cannot, the refusal is `ArcBelowSceneResolution`. `InconsistentArc`
//! is for neither.
#![allow(clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/profile/src/seg.rs",
    "crates/profile/src/lib.rs",
    "crates/profile/tests/common/",
];

use crate::common::{profile, tol};
use geom_core::Point2;
use profile::ProfileError;
use profile::test_support::bulge_loop;

/// Validates the arc `p0 → p1` of bulge `b`, closed by its chord, as a
/// table.
fn validate(p0: Point2<f64>, p1: Point2<f64>, b: f64) -> Result<(), ProfileError> {
    profile(vec![bulge_loop(vec![(p0, b), (p1, 0.0)])])
        .validate(tol())
        .map(|_| ())
}

/// Lowered D-shapes at vertex magnitudes from a fiftieth of the
/// one-ulp resolution scale K·ε/2⁻⁵² to four times it — across both
/// checks' gates, which sit at an eighth and a sixteenth of it — with
/// chords up to half the magnitude and bulges of either sign.
#[test]
fn a_lowered_table_arc_near_the_resolution_scale_is_never_inconsistent() {
    let band = geom_core::Band::linear(tol()).expect("the suite's band");
    let unit = band.escalate() / f64::EPSILON;
    let mut rng = test_utils::fuzz::start("table_arcs_inside_scene_resolution");
    for _ in 0..test_utils::fuzz::scaled(2000) {
        let m = unit * rng.range(0.02, 4.0);
        let p0 = Point2::new(m * rng.range(-1.0, 1.0), m * rng.range(-1.0, 1.0));
        let (l, d) = (
            m * rng.range(1e-4, 0.5),
            rng.range(0.0, core::f64::consts::TAU),
        );
        let p1 = Point2::new(p0.x + l * d.cos(), p0.y + l * d.sin());
        let b = rng.range(0.05, 2.0) * if rng.unit() < 0.5 { -1.0 } else { 1.0 };
        if let Err(ProfileError::InconsistentArc { check, .. }) = validate(p0, p1, b) {
            panic!(
                "p0 = {p0:?}, p1 = {p1:?}, b = {b:e} (magnitude {:.3} of K·ε/2⁻⁵²): a lowered \
                 arc refused InconsistentArc {check:?}; {}",
                m / unit,
                test_utils::fuzz::replay()
            );
        }
    }
}

/// The lowered arc that read a definite landing inside the one-ulp gate
/// at ε 1e-12 (4e4 m from the origin). At every ε it is either read in
/// the band or refused as below the scene's resolution.
#[test]
fn the_lowered_arc_forty_kilometres_out_is_not_inconsistent() {
    let got = validate(
        Point2::new(40_699.090_051_304_694, -8_736.154_085_499_025),
        Point2::new(36_968.901_970_701_314, -16_772.046_835_665_85),
        -1.308_736_876_905_907_8,
    );
    assert!(
        !matches!(got, Err(ProfileError::InconsistentArc { .. })),
        "refused as inconsistent: {got:?}"
    );
}
