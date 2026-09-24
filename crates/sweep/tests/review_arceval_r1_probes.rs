//! Blinded-review probes for PR #922 at the whole-body level.
//!
//! Two rows, both at the interval scalar:
//!
//! - **E1, an independent consumer of the arc lane**: a revolved ball
//!   is the *left* operand of a subtract whose cutter block chops a cap
//!   off it — the mirror of the m5_s12 fixture (there the sphere is the
//!   cutter). The crossing insertion splits the ball's seam meridians,
//!   which are `MappedCurve` over `Arc`, so the split rides
//!   `SketchSegment::restrict`/`eval` end-to-end and re-certifies
//!   against `carrier_matches_mapped_source`. The row asserts the
//!   subtraction decides definitely at the certified scalar and that
//!   the volume enclosure contains the closed form (ball minus a
//!   spherical cap).
//!
//! - **E2, a second witness on the re-scoped m5 row's constant**: it
//!   re-runs that row's fixture from the reviewer's side and pins `hi`
//!   to the measured value from both sides, so any movement of the
//!   enclosure, in either direction, is loud here too.
//!
//!   **What holds "the same fixture" is the compiler, not this
//!   sentence.** E2 builds its operands from
//!   `crate::m5_s12_curved_ops_interval::certified`'s `plate` and
//!   `recut_ball`, and reads that module's
//!   `RECUT_MAPPED_ENCLOSURE_HI`. One plate, one ball, one constant.
//!   Each was two until 2026-09-19, held together by a sentence here —
//!   which is precisely what a staleness pin must not rest on.
//!
//!   **And that leaves E2 with nothing of its own, which is worth
//!   saying rather than letting "second witness" carry it.** It now
//!   runs the same subtract on the same two bodies and asserts the same
//!   predicate against the same constant, read from the place the other
//!   row reads it. So it cannot go red while the shipped row's arm is
//!   green, and it cannot catch that constant going stale, because it
//!   is not a second statement of the constant — it is the same one.
//!   What it still is: a second execution, in a second binary module,
//!   of a row that only runs at (interval, 1e-12).
//!
//!   That is a smaller claim than the one this row was opened with. The
//!   shipped row's guard was tightened to the both-sides form E2 was
//!   written to supply, so the independence E2 had has been absorbed
//!   rather than removed. Whether a probe with no residual independence
//!   earns its place is this file's owner's call, not a duplication
//!   unit's; it is filed and left standing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod certified {
    use core::f64::consts::PI;

    use geom_core::{Bounds, Interval, Tol};

    use crate::m5_s12_curved_ops_interval::certified::{
        RECUT_MAPPED_ENCLOSURE_HI, ball, plate, recut_ball,
    };
    use topo::{Body, mass_properties};

    /// A block covering the ball laterally, spanning `z ∈ [z0, z0 + len]`
    /// — the cap cutter.
    fn block(z0: f64, len: f64) -> Body<Interval> {
        sweep::test_support::brick((-1.0, 1.0), (-1.0, 1.0), (z0, z0 + len), Tol::witness())
    }

    /// E1: the ball as LEFT operand, its cap chopped by a block — the
    /// meridian-splitting configuration the m5 fixture exercises only
    /// with the sphere on the cutter side. Decides definitely at the
    /// certified scalar; the volume enclosure contains the closed form.
    #[test]
    fn e1_ball_minus_cap_block_decides_definitely_at_interval() {
        const R: f64 = 0.6;
        const Z_CUT: f64 = 0.3;
        let cut = topo::subtract(&ball(R), &block(Z_CUT, 1.0), Tol::witness())
            .expect("the cap cut decides at Interval");
        let cut = &cut.body().expect("a body").body;
        assert_eq!(topo::validate_geometric(cut, Tol::witness()), Ok(()));
        let h = R - Z_CUT;
        let cap = PI * h * h * (3.0 * R - h) / 3.0;
        let closed = 4.0 * PI * R * R * R / 3.0 - cap;
        let vol = mass_properties(cut, Tol::witness()).unwrap().volume;
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

    /// E2: the m5_s12 sphere-recut fixture, re-run; below the constant
    /// the escalation's `hi` must be *at* the measured value — a
    /// tightening of the arc chain that moves it is loud here even when
    /// it does not cross the band (the shipped row's silent window).
    #[test]
    fn e2_recut_escalation_hi_is_pinned_to_the_measured_constant() {
        if Tol::witness().eps() >= RECUT_MAPPED_ENCLOSURE_HI {
            // Above the constant the row's DEFINITE arm owns the claim.
            return;
        }
        // The m5_s12 fixture itself: its 3x3x0.8 plate, minus the unit
        // ball at (1.5, 1.5, 0.5).
        let plate = plate();
        let ball = recut_ball();
        let cut = topo::subtract(&plate, &ball, Tol::witness());
        let Err(topo::BooleanError::CrossingInsertion { source, .. }) = cut else {
            panic!("below the constant the chain must escalate, got {cut:?}");
        };
        let topo::EulerOpError::Certification {
            error: geom_brep::CertifyError::Escalated { check, cause, .. },
        } = source
        else {
            panic!("expected a certification escalation, got {source:?}");
        };
        assert_eq!(check, geom_brep::CertCheck::MappedSource);
        let geom_core::MarginDiag::Enclosure { lo, hi } = cause.margin else {
            panic!("expected an enclosure margin, got {:?}", cause.margin);
        };
        assert_eq!(lo, 0.0);
        println!(
            "e2: eps={:e}  escalation hi={hi:e}  constant={RECUT_MAPPED_ENCLOSURE_HI:e}",
            Tol::witness().eps()
        );
        // Pinned EXACTLY: measured bit-reproducible at eps = 1e-12
        // (hi == the constant, digit for digit — the escalating sample
        // is the schedule max). Any chain change that moves the
        // enclosure, in either direction, is loud here — including the
        // partial tightenings that land between the band and the
        // constant, which the shipped row's `hi ≤ 2·constant` ceiling
        // admits silently.
        assert!(
            hi == RECUT_MAPPED_ENCLOSURE_HI,
            "the escalation hi {hi:e} is not the measured constant \
             {RECUT_MAPPED_ENCLOSURE_HI:e} — the arc chain moved and the m5 row's constant \
             is stale (re-measure and re-state)"
        );
    }
}
