//! Blinded-review probes for PR #922 at the whole-body level.
//!
//! Two rows, both `--features interval` only:
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
//! - **E2, a tight staleness pin for the re-scoped m5 row's constant**:
//!   the re-scoped row bounds its escalation's `hi` only from above
//!   (`hi ≤ 2·RECUT_MAPPED_ENCLOSURE_HI`), so a *partial* tightening of
//!   the arc chain — one that lands between the band and the constant —
//!   leaves the constant stale silently. This row re-runs the same
//!   fixture and pins `hi` to the measured value from both sides, so
//!   any movement of the enclosure, in either direction, is loud.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod certified {
    use core::f64::consts::PI;

    use geom_core::{Bounds, Interval, Point2, Real, Tol, Vec2, Vec3};
    use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane, ValidatedProfile};
    use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
    use topo::{Body, mass_properties};

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }

    fn p2(x: f64, y: f64) -> Point2<Interval> {
        Point2::new(iv(x), iv(y))
    }

    fn validated(loops: Vec<ProfileLoop<Interval>>) -> ValidatedProfile<Interval> {
        Profile::new(SketchPlane::xy(), loops)
            .validate(Tol::witness())
            .unwrap()
    }

    /// A ball of radius `r` at the origin: semicircular profile (bulge
    /// exactly 1) revolved fully about the sketch y-axis.
    fn ball(r: f64) -> Body<Interval> {
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::new(vec![
            ProfileVertex::new(p2(0.0, -r), iv(1.0)),
            ProfileVertex::new(p2(0.0, r), iv(0.0)),
        ]);
        let axis = RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(iv(0.0), iv(1.0)),
        };
        revolve(&validated(vec![lp]), axis, Revolution::Full, Tol::witness())
            .unwrap()
            .body
    }

    /// A block covering the ball laterally, sketched at `z0`, extruded
    /// `len` upward — the cap cutter.
    fn block(z0: f64, len: f64) -> Body<Interval> {
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::polygon([
            p2(-1.0, -1.0),
            p2(1.0, -1.0),
            p2(1.0, 1.0),
            p2(-1.0, 1.0),
        ]);
        let plane = SketchPlane::from_frame(
            geom_core::Point3::new(iv(0.0), iv(0.0), iv(z0)),
            Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
            Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
        );
        let vp = Profile::new(plane, vec![lp])
            .validate(Tol::witness())
            .unwrap();
        extrude(&vp, Extrusion::Distance(iv(len)), Tol::witness())
            .unwrap()
            .body
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

    /// The re-scoped m5 row's constant, restated (see
    /// `m5_s12_curved_ops_interval.rs`); this probe pins it from BOTH
    /// sides where the shipped row bounds it only from above.
    // **Re-measured 2026-08-31.** Was `1.1414768974413613e-12`. The arc
    // chain tightened under enclosure work that merged with gates
    // drawing default-ε only, so no run compared this constant until a
    // later branch drew (interval, 1e-12). Re-stated, not loosened, as
    // the constant's own doc requires.
    const RECUT_MAPPED_ENCLOSURE_HI: f64 = 1.136_277_333_393_965_9e-12;

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
        // The m5_s12 fixture, restated: 3x3x0.8 plate minus the unit
        // ball at (1.5, 1.5, 0.5).
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::polygon([
            p2(0.0, 0.0),
            p2(3.0, 0.0),
            p2(3.0, 3.0),
            p2(0.0, 3.0),
        ]);
        let plate = extrude(
            &validated(vec![lp]),
            Extrusion::Distance(iv(0.8)),
            Tol::witness(),
        )
        .unwrap()
        .body;
        let ball = topo::transform_rigid(
            &ball(1.0),
            &geom_core::Affine3::translation(Vec3::new(iv(1.5), iv(1.5), iv(0.5))),
            Tol::witness(),
        )
        .unwrap();
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
