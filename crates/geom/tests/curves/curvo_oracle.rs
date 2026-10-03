//! Interpolation differential vs curvo (docs/CURVO-AUDIT.md: study
//! and test oracle ONLY, pinned commit, tolerant comparison — NOT an
//! oracle for approximation/bounds). Curvo's non-periodic
//! `Interpolation::interpolate` is the same A9.1 shape as ours:
//! chord-length parameters normalized to [0, 1], averaged knots, dense
//! collocation solve — so the two interpolants describe the same curve
//! and may be compared pointwise at matched parameters, at oracle
//! (solver-difference) tolerance, never bitwise (curvo routes through
//! std math and nalgebra LU).
//!
//! # Hermeticity (ruled at the M5 PR 4 review; accepted with the risk
//! stated)
//!
//! curvo is a **git dev-dependency** pinned at the audited rev
//! (`47d19d5`): resolving it needs the git database, so hermetic or
//! offline builds of `geom`'s tests require that db to be present
//! (CI's rust-cache covers the hosted lanes). One 51-line oracle test
//! does not warrant an oracle crate today. If this oracle surface ever
//! grows, the recorded escape hatch is the excluded-oracle-crate
//! pattern (the `interval-transcendentals` precedent: a
//! workspace-excluded crate owns the heavy/networked oracle
//! dependency, and kernel `cargo test --workspace` lanes never resolve
//! it).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use curvo::prelude::{Interpolation, NurbsCurve3D};
use geom::NurbsCurve3;
use geom_core::Point3;

#[test]
fn interpolation_matches_curvo_pointwise_at_oracle_tolerance() {
    // A non-planar, non-symmetric sample set (helix-ish arc).
    let ours_pts: Vec<Point3<f64>> = (0..9)
        .map(|k| {
            let t = f64::from(k) / 8.0;
            let th = 1.7 * t;
            let (s, c) = th.sin_cos();
            Point3::new(2.0 * c - 0.5, 1.5 * s + 1.0, 0.8 * t - 0.25)
        })
        .collect();
    let curvo_pts: Vec<nalgebra::Point3<f64>> = ours_pts
        .iter()
        .map(|p| nalgebra::Point3::new(p.x, p.y, p.z))
        .collect();

    for degree in [2usize, 3] {
        let ours = NurbsCurve3::interpolate(&ours_pts, degree).unwrap();
        let oracle = NurbsCurve3D::<f64>::interpolate(&curvo_pts, degree).unwrap();
        let (lo, hi) = oracle.knots_domain();
        assert!((lo - 0.0).abs() < 1e-15 && (hi - 1.0).abs() < 1e-15);
        let mut worst = 0.0f64;
        for k in 0..=256 {
            let t = f64::from(k) / 256.0;
            let a = ours.eval(t);
            let b = oracle.point_at(t);
            let d = ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt();
            worst = worst.max(d);
        }
        assert!(
            worst < 1e-9,
            "degree {degree}: max pointwise gap {worst:e} vs curvo"
        );
    }
}
