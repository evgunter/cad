//! M6 surgery, interval lane: the composition surgery at the
//! CERTIFIED scalar (feature `interval`).
//!
//! One pip suffices to reach every arm — the in-place box-edge
//! blends over a ringed face (carry-through decided by
//! `fillet3_ring_clearance` from honest enclosures), the meridian
//! splits, and the rim's torus-band replacement — and the composed
//! volume is BRACKETED against the closed form (Steiner blank minus
//! the cap minus the rim-torus term). The full 21-pip die runs at
//! `f64` in `m6_surgery.rs`; the registered `die_composed` corpus
//! document carries the standard one-pip Interval rows besides this.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

/// **Loud skip.** Without `--features interval` this binary is empty;
/// announce the skip so a lane that silently lost its certified rows
/// stays visible in the battery log.
#[cfg(not(feature = "interval"))]
#[test]
fn interval_lane_skipped_no_certified_coverage_here() {
    println!(
        "SKIPPED (no --features interval): m6_surgery_interval.rs \
         contributes NO certified coverage in this run — the composed \
         die's bracketed-volume row runs only in the interval lane."
    );
}

#[cfg(feature = "interval")]
mod certified {
    use core::f64::consts::PI;
    use geom_core::Tol;

    use geom::Curve3;
    use geom::Surface;
    use geom_core::{Affine3, Band, Bounds, Interval, Point2, Real, Vec2, Vec3};
    use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane, ValidatedProfile};
    use sweep::fillet::build::fillet_edges;
    use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
    use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
    use topo::{Body, BooleanDeclarations, mass_properties};

    const DIE_L: f64 = 1.0;
    const DIE_R: f64 = 0.12;
    const PIP_R: f64 = 0.09;
    const PIP_H: f64 = 0.05;
    const RIM_R: f64 = 0.02;

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }

    fn p2(x: f64, y: f64) -> Point2<Interval> {
        Point2::new(iv(x), iv(y))
    }

    fn band() -> Band {
        let tol = Tol::witness().get();
        Band::new(tol.eps, tol.k * tol.eps).unwrap()
    }

    fn validated(loops: Vec<ProfileLoop<Interval>>) -> ValidatedProfile<Interval> {
        Profile::new(SketchPlane::xy(), loops)
            .validate(Tol::witness())
            .unwrap()
    }

    fn cube() -> Body<Interval> {
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::polygon([
            p2(0.0, 0.0),
            p2(DIE_L, 0.0),
            p2(DIE_L, DIE_L),
            p2(0.0, DIE_L),
        ]);
        extrude(
            &validated(vec![lp]),
            Extrusion::Distance(iv(DIE_L)),
            Tol::witness(),
        )
        .unwrap()
        .body
    }

    /// One +Z-poled pip ball at the top face centre — the die_pips
    /// CORPUS discipline: the sketch frame is chosen so the revolve
    /// axis IS the face normal (u = +X, v = +Z), so the chart is
    /// polar to the cutting plane EXACTLY, with no rotation at all
    /// (a π/2 rotation's sin/cos are not exact, and an enclosure a
    /// hair off the pole is an approximation where an exact placement
    /// is available).
    fn pip_ball() -> Body<Interval> {
        // The half-disc lamina: a semicircle out of the south pole and
        // the straight diameter back.
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::new(vec![
            ProfileVertex::new(p2(0.0, -PIP_R), iv(1.0)),
            ProfileVertex::new(p2(0.0, PIP_R), iv(0.0)),
        ]);
        let profile = Profile::new(
            SketchPlane::from_frame(
                geom_core::Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
                Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
            ),
            vec![lp],
        )
        .validate(Tol::witness())
        .unwrap();
        let axis = RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(iv(0.0), iv(1.0)),
        };
        let ball = revolve(&profile, axis, Revolution::Full, Tol::witness())
            .unwrap()
            .body;
        let h = DIE_L / 2.0;
        topo::transform_rigid(
            &ball,
            &Affine3::translation(Vec3::new(iv(h), iv(h), iv(DIE_L + (PIP_R - PIP_H)))),
            Tol::witness(),
        )
        .unwrap()
    }

    fn blank_volume() -> f64 {
        let core = DIE_L - 2.0 * DIE_R;
        core.powi(3)
            + 6.0 * DIE_R * core.powi(2)
            + 12.0 * (PI * DIE_R * DIE_R / 4.0) * core
            + (4.0 / 3.0) * PI * DIE_R.powi(3)
    }

    fn cap(r: f64, h: f64) -> f64 {
        PI * h * h * (3.0 * r - h) / 3.0
    }

    /// The rim-torus extra term (the f64 suite's Pappus closed form).
    fn rim_fillet_extra(big_r: f64, h: f64, r: f64) -> f64 {
        let d = big_r - h;
        let s = ((big_r + r).powi(2) - (d + r).powi(2)).sqrt();
        let rho_rim = (big_r * big_r - d * d).sqrt();
        let k = big_r / (big_r + r);
        let (tsx, tsz) = (s * k, d - (d + r) * k);
        let tri_m = |a: (f64, f64), b: (f64, f64), c: (f64, f64)| -> f64 {
            let area2 = (b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1);
            (area2.abs() / 2.0) * ((a.0 + b.0 + c.0) / 3.0)
        };
        let seg_m = |c: (f64, f64), rad: f64, p: (f64, f64), q: (f64, f64)| -> f64 {
            let a0 = (p.1 - c.1).atan2(p.0 - c.0);
            let mut a1 = (q.1 - c.1).atan2(q.0 - c.0);
            if a1 - a0 > PI {
                a1 -= 2.0 * PI;
            } else if a0 - a1 > PI {
                a1 += 2.0 * PI;
            }
            let (lo, hi) = if a0 <= a1 { (a0, a1) } else { (a1, a0) };
            let sector =
                c.0 * rad * rad * (hi - lo) / 2.0 + rad.powi(3) * (hi.sin() - lo.sin()) / 3.0;
            sector - tri_m(c, p, q)
        };
        let p_rim = (rho_rim, 0.0);
        let t_p = (s, 0.0);
        let t_s = (tsx, tsz);
        let m = tri_m(p_rim, t_p, t_s)
            - seg_m((s, -r), r, t_p, t_s)
            - seg_m((0.0, d), big_r, p_rim, t_s);
        2.0 * PI * m
    }

    /// **The one-pip composed die, BRACKETED at the certified
    /// scalar**: both surgery phases run on honest enclosures and the
    /// certified volume encloses the closed form.
    #[test]
    fn interval_one_pip_composed_die_is_bracketed() {
        let pipped = boolean_op_with(
            BooleanOp::Subtract,
            &cube(),
            &pip_ball(),
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        )
        .expect("the pip cuts at Interval")
        .body()
        .expect("a body")
        .body
        .clone();
        let box_edges: Vec<_> = pipped
            .edges()
            .filter(|(_, e)| {
                pipped
                    .get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .is_some_and(|c| matches!(c.carrier(), Curve3::Line { .. }))
            })
            .map(|(k, _)| k)
            .collect();
        assert_eq!(box_edges.len(), 12);
        let blanked = fillet_edges(&pipped, &box_edges, iv(DIE_R), band(), Tol::witness())
            .expect("the in-place box blends decide definitely at Interval")
            .body;
        let rims: Vec<_> = blanked
            .edges()
            .filter(|(_, e)| {
                let kind = |he| {
                    let h = blanked.get_half_edge(he)?;
                    let f = blanked.get_loop(h.parent_loop)?.face;
                    blanked
                        .get_surface(blanked.get_face(f)?.surface)
                        .map(|s| match s {
                            Surface::Plane { .. } => 0u8,
                            Surface::Sphere { .. } => 1,
                            _ => 2,
                        })
                };
                matches!(
                    (kind(e.he_plus), kind(e.he_minus)),
                    (Some(0), Some(1)) | (Some(1), Some(0))
                )
            })
            .map(|(k, _)| k)
            .collect();
        assert_eq!(rims.len(), 2, "one rim of two arcs");
        let out = fillet_edges(&blanked, &rims, iv(RIM_R), band(), Tol::witness())
            .expect("the rim torus band decides definitely at Interval");
        assert_eq!(out.band_faces.len(), 1);
        let die = out.body;
        assert_eq!(
            topo::validate_geometric(&die, Tol::witness()),
            Ok(()),
            "tier 3"
        );
        let vol = mass_properties(&die, Tol::witness()).unwrap().volume;
        let want = blank_volume() - cap(PIP_R, PIP_H) - rim_fillet_extra(PIP_R, PIP_H, RIM_R);
        assert!(
            vol.lo() <= want && want <= vol.hi(),
            "enclosure [{}, {}] must contain {want}",
            vol.lo(),
            vol.hi()
        );
        assert!(
            vol.hi() - vol.lo() <= 1e-6,
            "the enclosure stays usably tight, got width {}",
            vol.hi() - vol.lo()
        );
    }
}
