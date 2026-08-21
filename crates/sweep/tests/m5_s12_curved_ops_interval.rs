//! M5 S12 interval lane: curved `revert` and the newly-live curved
//! ∖/∩ at the CERTIFIED scalar (feature `interval`).
//!
//! Everything S12 adds is exact structure — a `bool` negation, a
//! surface-KEY equality, an arena scan of surface kinds — so none of it
//! can widen and none of it can escalate. That is precisely what makes
//! this lane worth running: it pins that the flip and the inheritance
//! are STILL bitwise at a scalar that widens everything metric around
//! them, and that the ops they unblock decide DEFINITELY from honest
//! enclosures rather than landing on an escalation.
//!
//! Rows: the involution and determinism rows bitwise on a curved
//! `Body<Interval>`; the blind hole and its ∩ twin with certified
//! volume enclosures containing the closed forms; the mixed-sense split
//! with the inherited bit read back; and the S13-flipped sphere row
//! (the half-buried ball's ∖ deciding definitely through the extent
//! scan, re-cut and plane×sphere germ arm at the certified scalar).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

/// **Loud skip.** Without `--features interval` this binary is empty,
/// and an empty binary reports "0 passed" — which reads like coverage
/// in a battery summary. Announce the skip instead, so a lane that
/// silently lost its certified rows is visible in the log.
#[cfg(not(feature = "interval"))]
#[test]
fn interval_lane_skipped_no_certified_coverage_here() {
    println!(
        "SKIPPED (no --features interval): m5_s12_curved_ops_interval.rs \
         contributes NO certified coverage in this run — the S12 rows \
         (bitwise curved revert, definite curved subtract/intersect, the \
         mixed-sense split's inherited bit, the S13 sphere re-cut row) \
         run only in the interval lane."
    );
}

#[cfg(feature = "interval")]
mod certified {
    use core::f64::consts::PI;

    use geom::Surface;
    use geom_core::{Affine3, Bounds, Interval, Point2, Real, Tolerance, Vec2, Vec3};
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

    const R: f64 = 0.35;

    fn plate() -> Body<Interval> {
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::polygon([
            p2(0.0, 0.0),
            p2(3.0, 0.0),
            p2(3.0, 3.0),
            p2(0.0, 3.0),
        ]);
        extrude(&validated(vec![lp]), Extrusion::Distance(iv(0.8)))
            .unwrap()
            .body
    }

    /// The three-arc cylindrical boss at (1.2, 1.7), sketched at `z0`.
    fn boss(z0: f64, len: f64) -> Body<Interval> {
        let theta = 2.0 * PI / 3.0;
        let bulge = iv((theta / 4.0).tan());
        let at = |i: usize| {
            let th = theta * i as f64;
            p2(1.2 + R * th.cos(), 1.7 + R * th.sin())
        };
        // Three equal 120° arcs: every vertex leaves with the same
        // bulge, the third one closing the circle.
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::new(vec![
            ProfileVertex::new(at(0), bulge),
            ProfileVertex::new(at(1), bulge),
            ProfileVertex::new(at(2), bulge),
        ]);
        let plane = SketchPlane::from_frame(
            geom_core::Point3::new(iv(0.0), iv(0.0), iv(z0)),
            Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
            Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
        );
        let vp = Profile::new(plane, vec![lp])
            .validate(Tol::witness())
            .unwrap();
        extrude(&vp, Extrusion::Distance(iv(len))).unwrap().body
    }

    /// A 3 × 3 × 1 plate with a concave semicircular notch on its `x = 3`
    /// wall — S11's `sense: false` arc wall at the certified scalar.
    fn notched() -> Body<Interval> {
        // Only (3, 1) leaves on an arc: the semicircular notch bowing
        // into the plate.
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), iv(0.0)),
            ProfileVertex::new(p2(3.0, 0.0), iv(0.0)),
            ProfileVertex::new(p2(3.0, 1.0), iv(-1.0)),
            ProfileVertex::new(p2(3.0, 2.0), iv(0.0)),
            ProfileVertex::new(p2(3.0, 3.0), iv(0.0)),
            ProfileVertex::new(p2(0.0, 3.0), iv(0.0)),
        ]);
        extrude(&validated(vec![lp]), Extrusion::Distance(iv(1.0)))
            .unwrap()
            .body
    }

    fn encloses(vol: Interval, analytic: f64, what: &str) {
        assert!(
            vol.lo() <= analytic && analytic <= vol.hi(),
            "{what}: enclosure [{}, {}] must contain {analytic}",
            vol.lo(),
            vol.hi()
        );
        assert!(
            vol.hi() - vol.lo() <= 1e-9,
            "{what}: enclosure stays tight, got width {}",
            vol.hi() - vol.lo()
        );
    }

    /// The flip is bitwise at the certified scalar: nothing widens, the
    /// involution is exact, and the chart is not perturbed (a widened
    /// sphere or cylinder chart would show up immediately here).
    #[test]
    fn interval_curved_revert_is_bitwise() {
        for body in [boss(0.0, 1.0), notched()] {
            let original = format!("{body:?}");
            let rev = body.revert().unwrap();
            assert_eq!(
                format!("{:?}", rev.revert().unwrap()),
                original,
                "involution"
            );
            assert_eq!(
                format!("{:?}", body.revert().unwrap()),
                format!("{rev:?}"),
                "determinism"
            );
            for (k, f) in body.faces() {
                let curved = !matches!(body.get_surface(f.surface), Some(Surface::Plane { .. }));
                let now = rev.get_face(k).unwrap().sense;
                assert_eq!(now, if curved { !f.sense } else { f.sense });
                if curved {
                    assert_eq!(
                        format!("{:?}", rev.get_surface(f.surface).unwrap()),
                        format!("{:?}", body.get_surface(f.surface).unwrap()),
                        "a chart must not widen under revert"
                    );
                }
            }
        }
    }

    /// The blind hole and its ∩ twin decide DEFINITELY at the certified
    /// scalar, with volume enclosures containing the closed forms.
    #[test]
    fn interval_curved_subtract_and_intersect_decide_definitely() {
        let a = plate();
        let b = boss(0.3, 1.0);
        let cut = topo::subtract(&a, &b).expect("curved subtract decides at Interval");
        let cut = &cut.body().expect("a body").body;
        assert_eq!(topo::validate_geometric(cut), Ok(()));
        encloses(
            mass_properties(cut).unwrap().volume,
            3.0 * 3.0 * 0.8 - PI * R * R * 0.5,
            "blind hole",
        );

        let met = topo::intersect(&a, &b).expect("curved intersect decides at Interval");
        let met = &met.body().expect("a body").body;
        assert_eq!(topo::validate_geometric(met), Ok(()));
        encloses(
            mass_properties(met).unwrap().volume,
            PI * R * R * 0.5,
            "the plug",
        );
    }

    /// The mixed-sense split at the certified scalar: the fragments of the
    /// reversed wall still come back `sense: false`, and the volume
    /// enclosure still contains the exact closed form.
    #[test]
    fn interval_split_of_a_reversed_wall_inherits_the_bit() {
        let a = notched();
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::polygon([
            p2(2.0, 0.5),
            p2(4.0, 0.5),
            p2(4.0, 2.5),
            p2(2.0, 2.5),
        ]);
        let plane = SketchPlane::from_frame(
            geom_core::Point3::new(iv(0.0), iv(0.0), iv(0.3)),
            Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
            Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
        );
        let vp = Profile::new(plane, vec![lp])
            .validate(Tol::witness())
            .unwrap();
        let b = extrude(&vp, Extrusion::Distance(iv(0.4))).unwrap().body;

        let notch = PI * 0.25 / 2.0;
        let out = topo::intersect(&a, &b).expect("the split decides at Interval");
        let out = &out.body().expect("a body").body;
        assert_eq!(topo::validate_geometric(out), Ok(()));
        encloses(
            mass_properties(out).unwrap().volume,
            (2.0 - notch) * 0.4,
            "the meet across the reversed wall",
        );
        let reversed = out
            .faces()
            .filter(|(_, f)| !matches!(out.get_surface(f.surface), Some(Surface::Plane { .. })))
            .filter(|(_, f)| !f.sense)
            .count();
        assert!(
            reversed > 0,
            "the mef re-mint did not inherit the parent bit at Interval"
        );
    }

    /// **CONSTRUCTION row, flipped from the S12 door pin** (M5 S13):
    /// the sphere class now goes ALL the way through at the certified
    /// scalar. The half-buried ball is the finding's own
    /// poking-but-not-crossing shape, so this row certifies the whole
    /// §1 chain under Interval — the extent scan's trileans decide
    /// definitely from honest enclosures, the re-cut's rigid rotation
    /// re-certifies, and the re-entered pipeline's plane×sphere germs
    /// mint arcs whose volume enclosure contains the closed form.
    #[test]
    fn interval_sphere_subtract_decides_definitely_after_the_recut() {
        // The half-disc lamina: a semicircle out of (0, -1) and the
        // straight diameter back.
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::new(vec![
            ProfileVertex::new(p2(0.0, -1.0), iv(1.0)),
            ProfileVertex::new(p2(0.0, 1.0), iv(0.0)),
        ]);
        let axis = RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(iv(0.0), iv(1.0)),
        };
        let ball = revolve(&validated(vec![lp]), axis, Revolution::Full)
            .unwrap()
            .body;
        let ball = topo::transform_rigid(
            &ball,
            &Affine3::translation(Vec3::new(iv(1.5), iv(1.5), iv(0.5))),
        )
        .unwrap();

        let cut = topo::subtract(&plate(), &ball).expect("S13: the sphere class decides");
        let cut = &cut.body().expect("a body").body;
        assert_eq!(topo::validate_geometric(cut), Ok(()));
        // plate − (ball zone between z = 0 and z = 0.8):
        // zone = 4π/3 − cap(0.7) − cap(0.5), cap(h) = πh²(3−h)/3.
        let cap = |h: f64| PI * h * h * (3.0 - h) / 3.0;
        let zone = 4.0 * PI / 3.0 - cap(0.7) - cap(0.5);
        let vol = mass_properties(cut).unwrap().volume;
        assert!(
            vol.lo() <= 7.2 - zone && 7.2 - zone <= vol.hi(),
            "enclosure [{}, {}] must contain {}",
            vol.lo(),
            vol.hi(),
            7.2 - zone
        );
        assert!(
            vol.hi() - vol.lo() <= 1e-6,
            "enclosure stays usably tight, got width {}",
            vol.hi() - vol.lo()
        );
        // And the cylinder class still decides at this scalar (S13
        // opens a class, it does not trade one away).
        assert!(topo::subtract(&plate(), &boss(0.3, 1.0)).is_ok());
    }
}
use geom_core::Tol;
