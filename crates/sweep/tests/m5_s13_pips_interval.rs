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

    /// **The finding row's flip, BRACKETED**: ∪ of the slab and the
    /// half-buried ball encloses 16 + 2·cap(1, 0.5) = 17.30899693899575
    /// at the certified scalar, one shell.
    #[test]
    fn interval_finding_union_is_bracketed() {
        let a = slab();
        let b = ball_at(1.0, Vec3::new(iv(2.0), iv(2.0), iv(0.5)));
        let joined = topo::union(&a, &b, Tol::witness()).expect("S13: the poking union decides");
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
