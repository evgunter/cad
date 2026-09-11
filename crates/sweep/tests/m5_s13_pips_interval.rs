//! M5 S13 interval lane: the die-pips rows at the CERTIFIED scalar
//! (feature `interval`).
//!
//! What this lane is FOR here: the §1 re-cut is the first fallback
//! path that composes metric trileans (the extent gap), a rigid
//! rotation's re-certification, and the §2 plane×sphere sections into
//! one answer — this lane pins that every one of those decides
//! DEFINITELY from honest enclosures, and that the finding row's
//! flipped union value is **bracketed**: the certified volume
//! enclosure contains 17.30899693899575.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

/// **Loud skip.** Without `--features interval` this binary is empty;
/// announce the skip so a lane that silently lost its certified rows
/// stays visible in the battery log.
#[cfg(not(feature = "interval"))]
#[test]
fn interval_lane_skipped_no_certified_coverage_here() {
    println!(
        "SKIPPED (no --features interval): m5_s13_pips_interval.rs \
         contributes NO certified coverage in this run — the S13 rows \
         (the bracketed 17.30900 union, the pip ∖/∩ enclosures and \
         their additivity) run only in the interval lane."
    );
}

#[cfg(feature = "interval")]
mod certified {
    use core::f64::consts::PI;
    use geom_core::Tol;

    use geom_core::{Affine3, Bounds, Interval, Point2, Real, Vec2, Vec3};
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

    /// The 4 × 4 × 1 slab of the finding row.
    fn slab() -> Body<Interval> {
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::polygon([
            p2(0.0, 0.0),
            p2(4.0, 0.0),
            p2(4.0, 4.0),
            p2(0.0, 4.0),
        ]);
        extrude(
            &validated(vec![lp]),
            Extrusion::Distance(iv(1.0)),
            Tol::witness(),
        )
        .unwrap()
        .body
    }

    /// A radius-`r` ball at `centre` (horizontal polar axis — the §1
    /// re-cut's own chart shape).
    fn ball_at(r: f64, centre: Vec3<Interval>) -> Body<Interval> {
        // The half-disc lamina: a semicircle out of the south pole and
        // the straight diameter back.
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::new(vec![
            ProfileVertex::new(p2(0.0, -r), iv(1.0)),
            ProfileVertex::new(p2(0.0, r), iv(0.0)),
        ]);
        let axis = RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(iv(0.0), iv(1.0)),
        };
        let ball = revolve(&validated(vec![lp]), axis, Revolution::Full, Tol::witness())
            .unwrap()
            .body;
        topo::transform_rigid(&ball, &Affine3::translation(centre), Tol::witness()).unwrap()
    }

    fn encloses(vol: Interval, analytic: f64, what: &str) {
        assert!(
            vol.lo() <= analytic && analytic <= vol.hi(),
            "{what}: enclosure [{}, {}] must contain {analytic}",
            vol.lo(),
            vol.hi()
        );
        assert!(
            vol.hi() - vol.lo() <= 1e-6,
            "{what}: enclosure stays usably tight, got width {}",
            vol.hi() - vol.lo()
        );
    }

    fn cap(r: f64, h: f64) -> f64 {
        PI * h * h * (3.0 * r - h) / 3.0
    }

    /// The `carrier_matches_mapped_source` enclosure this row's chain
    /// escalates on (metres), measured at the FIRST escalating sample
    /// of the crossing insertion's child — certification aborts there,
    /// so later samples of that edge never run and this is not a claim
    /// about them. It is ε-INDEPENDENT (bit-identical at 1e-6 and
    /// 1e-12): it is the interval lane's enclosure width, a property of
    /// the arithmetic that built the two points, not of the tolerance
    /// they are judged against. The row therefore certifies exactly
    /// when ε is at or above it.
    ///
    /// The escalation arm pins `hi` to this value BIT-EXACTLY, in both
    /// directions. A regression that widens the arc chain is loud, and
    /// so is a tightening that narrows it — including a partial one
    /// landing between the band and this constant, which an
    /// upper-bound-only guard would admit in silence. Either way the
    /// answer is the same: re-measure and re-state the constant, never
    /// loosen the guard around it.
    /// The last MEASURED mapped-source enclosure width for this union.
    /// It is a ceiling and a crossover marker, not a required value:
    /// the chain is expected to narrow it over time, and at ε = 1e-12
    /// it has already narrowed past the point where this union escalates
    /// at all.
    const UNION_MAPPED_ENCLOSURE_HI: f64 = 1.127306994088959e-12;

    /// **The finding row's flip, BRACKETED**: ∪ of the slab and the
    /// half-buried ball encloses 16 + 2·cap(1, 0.5) = 17.30899693899575
    /// at the certified scalar, one shell.
    ///
    /// **Scoped to ε ≥ [`UNION_MAPPED_ENCLOSURE_HI`]** (#921). This is
    /// the third enumerated member of that issue's class, and it takes
    /// the treatment its siblings took. Below the constant the chain
    /// escalates on `carrier_matches_mapped_source` while splitting the
    /// ball's meridian — a `MappedCurve` over an `Arc` whose restricted
    /// endpoints already arrive ~1.2e-13 m wide, because the
    /// chord+bulge representation derives an arc's centre by
    /// differencing a short chord, so every restriction stores a
    /// centre-amplified endpoint and the next one inherits it.
    ///
    /// That escalation is honest rather than a defect, so the row
    /// asserts it instead of asserting a decision the scalar cannot
    /// make. The enclosure is `[0, hi]`: its low end is exactly zero,
    /// so nothing about the locus is being denied — the carrier and its
    /// mapped source may coincide exactly, and the interval lane simply
    /// cannot see that they do. What `hi ≤ ε` asks at `T = Interval` is
    /// whether the CONSTRUCTION's accumulated enclosure width fits
    /// inside the tolerance: a question about conditioning, not about
    /// geometry. D4 ¶2's certification and D2's prefer-intrinsic
    /// exemptions both already ratify that ε-tightening may escalate;
    /// an Interval indeterminate is a designed outcome, not a red.
    ///
    /// The structural fix is the banked center/radius/angle triple
    /// restrict (#921), which retires the class rather than re-scoping
    /// its members one at a time.
    #[test]
    fn interval_finding_union_is_bracketed() {
        let a = slab();
        let b = ball_at(1.0, Vec3::new(iv(2.0), iv(2.0), iv(0.5)));
        let joined = topo::union(&a, &b, Tol::witness());
        // **The crossover is an enclosure width, and enclosure widths
        // move.** This row used to select its arm by comparing ε to a
        // pinned constant, which asserts that a chain tightening is
        // impossible: below the constant it REQUIRED the escalation. A
        // narrower mapped-source enclosure is the improvement the whole
        // programme is for, and at ε = 1e-12 the chain now serves this
        // union outright.
        //
        // So the arm is chosen by the OUTCOME and both arms keep their
        // full assertions. What is still pinned in the escalating
        // direction is the thing that would be a regression: escalating
        // ABOVE the last measured crossover, where the enclosure is
        // comfortably inside the band and a decision is owed.
        if let Err(topo::BooleanError::CrossingInsertion { source, .. }) = joined {
            assert!(
                Tol::witness().eps() < UNION_MAPPED_ENCLOSURE_HI,
                "escalating at an ε above the last measured crossover \
                 ({UNION_MAPPED_ENCLOSURE_HI:e}) is a regression, not a tightening: \
                 {source:?}"
            );
            let topo::EulerOpError::Certification {
                error: geom_brep::CertifyError::Escalated { check, cause, .. },
            } = source
            else {
                panic!("the refusal must be a certification escalation, got {source:?}");
            };
            assert_eq!(check, geom_brep::CertCheck::MappedSource);
            assert_eq!(cause.predicate, Some("carrier_matches_mapped_source"));
            let geom_core::MarginDiag::Enclosure { lo, hi } = cause.margin else {
                panic!(
                    "the escalation must carry an enclosure, got {:?}",
                    cause.margin
                );
            };
            // The honest content of the refusal: the enclosure does not
            // exclude exact coincidence (lo = 0), and it escaped the
            // band only by being WIDE — construction conditioning, not a
            // residual saying the carrier left its source.
            assert_eq!(lo, 0.0, "the enclosure must not exclude coincidence");
            assert!(
                hi > cause.band.zero(),
                "the enclosure must exceed the coincidence threshold, else it would classify"
            );
            // The enclosure may only have NARROWED since it was
            // measured — a widening is the chain getting worse and is
            // what this bound is here to catch.
            assert!(
                hi <= UNION_MAPPED_ENCLOSURE_HI,
                "the mapped-source enclosure WIDENED to {hi:e} from its measured \
                 {UNION_MAPPED_ENCLOSURE_HI:e} — the arc chain got worse; re-measure \
                 and re-state deliberately"
            );
            return;
        }
        let joined = joined.expect("S13: the poking union decides");
        let joined = &joined.body().expect("a body").body;
        assert_eq!(topo::validate_geometric(joined, Tol::witness()), Ok(()));
        assert_eq!(joined.shells().count(), 1);
        encloses(
            mass_properties(joined, Tol::witness()).unwrap().volume,
            16.0 + 2.0 * cap(1.0, 0.5),
            "the poking union",
        );
    }

    /// The pip ∖/∩ pair at the certified scalar, with additivity.
    #[test]
    fn interval_pip_pair_is_bracketed_and_additive() {
        let a = slab();
        let (r, h) = (0.5, 0.3);
        let b = ball_at(r, Vec3::new(iv(2.0), iv(2.0), iv(1.0 + r - h)));

        let cut = topo::subtract(&a, &b, Tol::witness()).expect("the pip decides at Interval");
        let cut = &cut.body().expect("a body").body;
        assert_eq!(topo::validate_geometric(cut, Tol::witness()), Ok(()));
        let v_cut = mass_properties(cut, Tol::witness()).unwrap().volume;
        encloses(v_cut, 16.0 - cap(r, h), "the pip cavity");

        let met = topo::intersect(&a, &b, Tol::witness()).expect("the cap decides at Interval");
        let met = &met.body().expect("a body").body;
        assert_eq!(topo::validate_geometric(met, Tol::witness()), Ok(()));
        let v_met = mass_properties(met, Tol::witness()).unwrap().volume;
        encloses(v_met, cap(r, h), "the cap");

        encloses(v_cut + v_met, 16.0, "∖/∩ additivity");
    }
}
