//! Blinded-review probes for PR #922 at the whole-body level.
//!
//! One row, at the interval scalar:
//!
//! - **E1, an independent consumer of the arc lane**: a revolved ball
//!   is the *left* operand of a subtract whose cutter block chops a cap
//!   off it — the mirror of the m5_s12 fixture (there the sphere is the
//!   cutter). The crossing insertion splits the ball's seam meridians,
//!   which are `MappedCurve` over `Arc`, so the split rides
//!   `MappedCurve::restrict`/`eval` end-to-end and re-certifies
//!   against `carrier_matches_mapped_source`. The row asserts the
//!   subtraction decides definitely at the certified scalar and that
//!   the volume enclosure contains the closed form (ball minus a
//!   spherical cap).
//!
//! The m5_s12 sphere-recut fixture's escalation constant is not pinned
//! here: `m5_s12_curved_ops_interval.rs`'s
//! `interval_sphere_subtract_decides_definitely_after_the_recut` owns
//! it, pinning the escalation's `hi` from both sides against
//! `common::sphere_recut`'s `RECUT_MAPPED_ENCLOSURE_HI`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod certified {
    use core::f64::consts::PI;

    use geom_core::{Bounds, Interval, Real, Tol, Vec3};

    use sweep::test_support::{ball_poled_y, finished};
    use topo::{AtRestBody, mass_properties};

    /// A block covering the ball laterally, spanning `z ∈ [z0, z0 + len]`
    /// — the cap cutter.
    fn block(z0: f64, len: f64) -> AtRestBody<Interval> {
        finished(
            "the block",
            sweep::test_support::brick::<geom_core::Interval>(
                (-1.0, 1.0),
                (-1.0, 1.0),
                (z0, z0 + len),
                Tol::witness(),
            ),
            Tol::witness(),
        )
    }

    /// E1: the ball as LEFT operand, its cap chopped by a block — the
    /// meridian-splitting configuration the m5 fixture exercises only
    /// with the sphere on the cutter side. Decides definitely at the
    /// certified scalar; the volume enclosure contains the closed form.
    #[test]
    fn e1_ball_minus_cap_block_decides_definitely_at_interval() {
        const R: f64 = 0.6;
        const Z_CUT: f64 = 0.3;
        let cut = topo::subtract(
            &finished(
                "the ball",
                ball_poled_y(
                    Interval::from_f64(R),
                    Vec3::new(Interval::zero(), Interval::zero(), Interval::zero()),
                    Tol::witness(),
                ),
                Tol::witness(),
            ),
            &block(Z_CUT, 1.0),
            Tol::witness(),
        )
        .expect("the cap cut decides at Interval");
        let cut = &cut.body().expect("a body").body;
        assert_eq!(topo::validate_geometric(cut, Tol::witness()), Ok(()));
        let h = R - Z_CUT;
        let cap = PI * h * h * (3.0 * R - h) / 3.0;
        let closed = 4.0 * PI * R * R * R / 3.0 - cap;
        let vol = mass_properties(
            cut,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume;
        assert!(
            vol.lo() <= closed && closed <= vol.hi(),
            "volume enclosure [{}, {}] must contain {closed}",
            vol.lo(),
            vol.hi()
        );
        println!(
            "e1: eps={:e}  volume enclosure [{}, {}]  width {:e}",
            Tol::witness().eps(),
            vol.lo(),
            vol.hi(),
            vol.hi() - vol.lo()
        );
    }
}
