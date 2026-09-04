//! FILLET-E3 review probes (PR 1763): what the classified-margin payload
//! says at the CERTIFIED scalar beside the fields that ride with it.
//!
//! Three rows, all at `T = Interval`. Each asserts the property the
//! unit's invariant asks for, so each is RED at the head under review
//! and goes green when the shape it names is fixed:
//!
//! 1. **One bracket, one shape.** A thin interval margin is diagnosed
//!    as `MarginDiag::Enclosure { lo, hi }` with `lo == hi` when it
//!    escalates (`Interval::sign_within`, pinned in `geom-core`'s dual
//!    suite) and as `MarginDiag::Value` when it decides
//!    (`blend::battery::classified`). Two spellings of "what a
//!    bracket's reading is", one per twin.
//! 2. **The companion `gap` is a projected endpoint.**
//!    `FaceClearanceUncertified.gap` is `gap.lo()` of a measured
//!    enclosure and the message states it as the distance.
//! 3. **The companion `arm` is a projected endpoint.** `ChainNotG1.arm`
//!    is `arm.lo()` of a computed lever and the message states it as
//!    the lever arm.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

/// **Loud skip.** Without `--features interval` this file is empty.
#[cfg(not(feature = "interval"))]
#[test]
fn interval_lane_skipped_no_certified_coverage_here() {
    println!(
        "SKIPPED (no --features interval): review_fillet_e3_probes.rs \
         contributes NO certified coverage in this run."
    );
}

#[cfg(feature = "interval")]
mod certified {
    use geom_core::{Band, Decide, Interval, MarginDiag, Real, Sign, Tol, Vec3};
    use sweep::blend::BlendError;
    use sweep::blend::battery::{chain_g1, face_clearance};
    use sweep::blend::surgery::ring_clearance;
    use topo::{FaceKey, VertexKey};

    fn band() -> Band {
        let tol = Tol::witness();
        Band::new(tol.eps(), tol.k() * tol.eps()).unwrap()
    }

    /// The SAME kind of reading — a point bracket — becomes a
    /// different `MarginDiag` variant depending on which twin carries
    /// it. Whichever spelling is right, a reader of `MarginDiag` should
    /// not need to know whether the predicate decided or escalated to
    /// know what a thin bracket looks like.
    #[test]
    fn a_thin_bracket_has_one_diagnostic_shape_whether_decided_or_escalated() {
        let band = band();
        // Escalated: a thin bracket strictly inside the ambiguity band.
        let in_band = (band.zero() + band.escalate()) / 2.0;
        let escalated = Interval::from_f64(in_band)
            .sign_within(band)
            .expect_err("a value between zero and escalate is indeterminate");
        // Decided: a thin bracket definitely below zero, through a
        // producer that builds a `ClassifiedMargin`.
        let decided = match ring_clearance(FaceKey::default(), Interval::from_f64(-0.05), band) {
            Err(BlendError::RingClearance { margin, .. }) => margin,
            other => panic!("expected RingClearance, got {other:?}"),
        };
        assert_eq!(decided.sign, Sign::Negative);
        let esc_is_value = matches!(escalated.margin, MarginDiag::Value(_));
        let dec_is_value = matches!(decided.reading, MarginDiag::Value(_));
        assert_eq!(
            esc_is_value, dec_is_value,
            "a thin bracket is diagnosed as {:?} when it escalates and as {:?} when it \
             decides — two spellings of one reading",
            escalated.margin, decided.reading
        );
    }

    /// `gap` is a measured enclosure at this scalar (the closest
    /// approach of two sampled boundaries), and the refusal states one
    /// endpoint of it as the distance.
    #[test]
    fn the_clearance_refusal_does_not_state_an_endpoint_of_the_gap_as_the_distance() {
        let gap = Interval::from_bounds(0.10, 0.20);
        let setback = Interval::from_f64(0.15);
        let err = face_clearance(FaceKey::default(), gap, setback, setback, false, band())
            .expect_err("a gap wholly inside the two setbacks refuses definitely");
        let BlendError::FaceClearanceUncertified { margin, .. } = &err else {
            panic!("expected FaceClearanceUncertified, got {err:?}");
        };
        assert!(
            matches!(margin.reading, MarginDiag::Enclosure { .. }),
            "the margin itself is carried as the enclosure it is: {:?}",
            margin.reading
        );
        let text = err.to_string();
        assert!(
            !text.contains("0.1 m apart"),
            "the gap was [0.1, 0.2] m; the refusal states its lower end as the distance: {text}"
        );
    }

    /// `arm` is the folded lever arm — computed from the carrier, an
    /// enclosure at this scalar — and the refusal states one endpoint
    /// of it as the lever.
    #[test]
    fn the_g1_refusal_does_not_state_an_endpoint_of_the_arm_as_the_lever() {
        let z = Interval::from_f64(0.0);
        let one = Interval::from_f64(1.0);
        let tau_in = Vec3::new(one, z, z);
        let tau_out = Vec3::new(z, one, z);
        let arm = Interval::from_bounds(0.5, 1.0);
        let err = chain_g1(tau_in, tau_out, arm, VertexKey::default(), band())
            .expect_err("a 90° kink refuses G1 definitely");
        let BlendError::ChainNotG1 { margin, .. } = &err else {
            panic!("expected ChainNotG1, got {err:?}");
        };
        assert_eq!(margin.sign, Sign::Positive);
        assert!(
            matches!(margin.reading, MarginDiag::Enclosure { .. }),
            "sin θ · arm over a wide arm is an enclosure: {:?}",
            margin.reading
        );
        let text = err.to_string();
        assert!(
            !text.contains("lever arm 0.5 m"),
            "the arm was [0.5, 1] m; the refusal states its lower end as the lever: {text}"
        );
    }
}
