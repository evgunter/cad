//! **The collapsed conventional meter's disposition** (PCURVE P-1a
//! fix pass): what `|C(t) − S(P(t))|` measures, how it differs from
//! the seam arm's pre-collapse `implicit_residual`, and what that
//! costs at the band edge.
//!
//! The change is a **re-baseline, not a bit move**, and these rows
//! are the argument for it plus the price of it. Both reviewers found
//! the quantity change independently (R1 on a cone, R2 on a sphere, a
//! cone and a small-radius cylinder); the rows here state the
//! disposition rather than the discovery.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{
    CertCheck, CertifyError, ChartWindow, EdgeCurve, EdgeCurveSpec, EdgeGeometry, PcurveCache,
    chart_pcurve, implicit_residual,
};
use geom_core::{Band, Point3, Vec3};

/// The rows' own ε, and the band built from it. Fixed rather than the
/// run's, so a row about the METER does not become a row about the
/// matrix point it ran at.
const ROW_EPS: f64 = 1.0e-9;

fn band() -> Band {
    Band::new(ROW_EPS, 10.0 * ROW_EPS).expect("the rows' own band")
}

fn table(
    surfs: Vec<Surface<f64>>,
) -> (
    Vec<geom_brep::SurfaceKey>,
    impl Fn(geom_brep::SurfaceKey) -> Option<Surface<f64>>,
) {
    let mut map: slotmap::SlotMap<geom_brep::SurfaceKey, Surface<f64>> =
        slotmap::SlotMap::with_key();
    let keys: Vec<geom_brep::SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
    (keys, move |k| map.get(k).cloned())
}

fn window() -> ChartWindow<f64> {
    ChartWindow {
        u_min: -100.0,
        u_max: 100.0,
        v_min: -100.0,
        v_max: 100.0,
    }
}

/// A cone seam ruling displaced `d` metres along the surface's own
/// OUTWARD NORMAL — so `d` is exactly the perpendicular distance to
/// the cone, the quantity the pre-collapse seam arm metered.
fn cone_seam(half_angle: f64, d: f64) -> (Surface<f64>, Curve3<f64>, f64, f64) {
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle,
        u_ref: Vec3::unit_x(),
    };
    let (s, c) = half_angle.sin_cos();
    let generator = Vec3::new(s, 0.0, c);
    let outward = Vec3::new(c, 0.0, -s);
    let carrier = Curve3::Line {
        origin: Point3::origin() + outward * d,
        dir: generator,
    };
    (cone, carrier, 0.5, 2.0)
}

/// **The price, stated.** An edge whose perpendicular drift is inside
/// the band — 0.98 ε, which certified before this unit — now
/// ESCALATES, because the collapsed meter reads the radial chord
/// `d·sec α` = 1.1316 ε and that is in the ambiguity band.
///
/// This row exists so the re-baseline is a decision on the record
/// rather than a surprise in a consumer. It is deliberately written
/// as an assertion of the NEW behaviour: if a later change makes this
/// certify again, that is a meter change and it has to be argued.
#[test]
fn a_cone_edge_inside_the_legacy_band_now_escalates() {
    let alpha = core::f64::consts::FRAC_PI_6; // 30°, sec α = 1.1547
    let d = 0.98 * ROW_EPS;
    let (cone, carrier, t0, t1) = cone_seam(alpha, d);
    let (keys, lookup) = table(vec![cone.clone()]);
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));

    // The legacy quantity, at the fixture's own construction: the
    // displacement IS along the outward normal, so the perpendicular
    // distance is `d` and the pre-collapse meter certified it.
    let legacy = implicit_residual(&cone, carrier.eval(t0)).abs();
    assert!(
        (legacy / d - 1.0).abs() < 1e-6,
        "the fixture's perpendicular drift is {legacy:e}, not {d:e}"
    );
    assert!(
        legacy < ROW_EPS,
        "0.98 eps is inside the band, by construction"
    );

    let err = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeGeometry::Seam { surface: keys[0] },
            carrier,
            param_start: t0,
            param_end: t1,
        },
        p0,
        p1,
        &lookup,
        band(),
    )
    .expect_err("the collapsed meter reads d·sec α, which is outside the band's zero");
    match err {
        CertifyError::Escalated {
            check: CertCheck::ChartResidual,
            cause,
            ..
        } => {
            let geom_core::MarginDiag::Value(v) = cause.margin else {
                panic!("an f64 lane classifies a value, not an enclosure: {cause:?}");
            };
            let expected = d / alpha.cos();
            assert!(
                (v / expected - 1.0).abs() < 1e-6,
                "the escalating margin is {v:e}, and d·sec α is {expected:e}"
            );
        }
        other => panic!("expected an in-band escalation at the chart meter, got {other:?}"),
    }
}

/// **The argument that makes the price payable.** The collapsed
/// description lane now states exactly what the pcurve CACHE lane has
/// always stated on the same geometry, through the same mint.
///
/// Before this unit the two disagreed: a cone seam with a drift in
/// `(ε·cos α, ε]` certified as a DESCRIPTION (perpendicular residual
/// within ε) while its own cache escalated (`pcurve_map_residual`
/// reads `d·sec α`). The collapse does not invent a rule — it removes
/// a place where the kernel said two different things about one edge.
///
/// Nothing in this row is code this unit changed: `PcurveCache` and
/// `chart_pcurve` are untouched.
#[test]
fn the_cache_lane_already_imposed_the_collapsed_meter() {
    let alpha = core::f64::consts::FRAC_PI_6;
    let d = 0.98 * ROW_EPS;
    let (cone, carrier, t0, t1) = cone_seam(alpha, d);
    let pcurve = chart_pcurve(&carrier, &cone, band()).expect("the cone ruling mints");
    let cache = PcurveCache::certify(pcurve, t0, t1, &carrier, &cone, window(), band());
    assert!(
        cache.is_err(),
        "the cache lane must already refuse the geometry the description lane now \
         refuses — if it certifies, the collapse is imposing a NEW rule and the \
         re-baseline argument is wrong"
    );
}

/// **What the collapsed meter is, positively**: an upper bound on the
/// distance from the carrier to the surface, exact when the chart
/// image names the foot point.
///
/// It is NOT "the same quantity, computed better", and it is not
/// uniformly larger than the legacy reading either — R2's
/// small-radius cylinder row is the counter-example, where the legacy
/// normalized implicit form over-reads by its own `d²/(2r)` term. The
/// invariant that does hold, on every chart, is this one: the
/// collapsed meter never understates the distance to the surface,
/// because `S(P(t))` is a point ON it.
#[test]
fn the_collapsed_meter_never_understates_the_distance_to_the_surface() {
    let d = 2.5e-10;
    /// One fixture of the conservatism row: name, chart, carrier, and
    /// the certified parameter interval.
    type Case = (&'static str, Surface<f64>, Curve3<f64>, f64, f64);
    let cases: Vec<Case> = vec![
        {
            let (cone, carrier, t0, t1) = cone_seam(core::f64::consts::FRAC_PI_6, d);
            ("cone", cone, carrier, t0, t1)
        },
        (
            "cylinder",
            Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 2.0,
                u_ref: Vec3::unit_x(),
            },
            Curve3::Line {
                origin: Point3::new(2.0 + d, 0.0, 0.0),
                dir: Vec3::unit_z(),
            },
            0.0,
            3.0,
        ),
    ];
    for (name, surface, carrier, t0, t1) in cases {
        let (keys, lookup) = table(vec![surface.clone()]);
        let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
        let cert = EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeGeometry::Seam { surface: keys[0] },
                carrier,
                param_start: t0,
                param_end: t1,
            },
            p0,
            p1,
            &lookup,
            band(),
        )
        .unwrap_or_else(|e| panic!("{name} seam certifies at a {d:e} drift: {e:?}"))
        .certificate()
        .max_residual;
        assert!(
            cert >= d * (1.0 - 1e-9),
            "{name}: the collapsed meter read {cert:e}, below the true distance {d:e} \
             — `S(P)` lies on the surface, so this cannot happen"
        );
    }
}

/// **R2's MINOR-2, first half**: a carrier kind with no chart image on
/// the described chart must refuse as a statement about the
/// DESCRIPTION, not as `Unimplemented` (which means a kind this build
/// refuses wholesale, and sends the reader looking for a missing
/// feature instead of a wrong locus).
#[test]
fn a_carrier_with_no_chart_image_names_the_pair_it_could_not_state() {
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: 0.5,
        u_ref: Vec3::unit_x(),
    };
    let (keys, lookup) = table(vec![cone]);
    let carrier = Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::unit_z(),
        major: 1.0,
        minor: 0.5,
        u_ref: Vec3::unit_x(),
    };
    let (p0, p1) = (carrier.eval(0.0), carrier.eval(1.0));
    let err = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeGeometry::Seam { surface: keys[0] },
            carrier,
            param_start: 0.0,
            param_end: 1.0,
        },
        p0,
        p1,
        &lookup,
        band(),
    )
    .expect_err("an ellipse is not a locus of any cone chart image");
    assert_eq!(
        err,
        CertifyError::ChartImageUnavailable {
            chart: "cone",
            carrier: "ellipse",
        },
        "the refusal must name the (chart, carrier) pair it could not state"
    );
}

/// **The same disposition at the INTERVAL scalar.**
///
/// The meter is generic over `T`, and this fix pass changed what it
/// MEASURES on the conventional arm, not merely how the bits fall. A
/// quantity change in a generic meter deserves a row at more than one
/// scalar: an enclosure lane can widen where an `f64` lane is exact,
/// and the `sec α` re-baseline is a claim about the GEOMETRY that
/// should survive the widening — which it does, measured below.
///
/// **Why these rows live here rather than in a file of their own.**
/// A separate `*_interval.rs` file pins the interval compile-mode
/// lane for the whole change (`ci-filter.py`'s `_forces_interval`
/// matches on basenames), and pinning it would mean the DEFAULT lane
/// never draws — which is where every bit-level row in this unit
/// lives, including the mint tripwire that is the D2 guard. The proof
/// below is already obtained; sampling re-verifies it over time,
/// which is the normal posture. Guaranteeing one lane by excluding
/// the other was the bad trade.
#[cfg(feature = "interval")]
mod at_intervals {
    use geom::{Curve3, Surface};
    use geom_brep::{EdgeCurve, EdgeCurveSpec, EdgeGeometry};
    use geom_core::{Band, Bounds, Interval, Point3, Real, Vec3};

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }

    /// The rows' own band, fixed rather than the run's — a row about the
    /// meter must not become a row about the matrix point it drew.
    fn band() -> Band {
        Band::new(1.0e-9, 1.0e-8).expect("the row's own band")
    }

    fn table(
        surfs: Vec<Surface<Interval>>,
    ) -> (
        Vec<geom_brep::SurfaceKey>,
        impl Fn(geom_brep::SurfaceKey) -> Option<Surface<Interval>>,
    ) {
        let mut map: slotmap::SlotMap<geom_brep::SurfaceKey, Surface<Interval>> =
            slotmap::SlotMap::with_key();
        let keys: Vec<geom_brep::SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
        (keys, move |k| map.get(k).cloned())
    }

    /// A cylinder seam certifies at the interval scalar through the
    /// collapsed meter — the arm the fix pass rewrote, on the enclosure
    /// lane, at a drift well inside the band.
    #[test]
    fn a_cylinder_seam_certifies_through_the_collapsed_meter_at_intervals() {
        let r = 2.0;
        let d = 2.5e-10;
        let (keys, lookup) = table(vec![Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
            radius: iv(r),
            u_ref: Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
        }]);
        let carrier = Curve3::Line {
            origin: Point3::new(iv(r + d), iv(0.0), iv(0.0)),
            dir: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        };
        let (t0, t1) = (iv(0.0), iv(3.0));
        let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
        let certified = EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeGeometry::Seam { surface: keys[0] },
                carrier,
                param_start: t0,
                param_end: t1,
            },
            p0,
            p1,
            &lookup,
            band(),
        )
        .expect("the seam certifies at the interval scalar");
        // The enclosure must CONTAIN the drift it was built from: an
        // interval meter that certified while excluding the true residual
        // would be unsound, and that is the property worth a row here.
        //
        // The comparand is the REPRESENTABLE drift `fl(r + d) − r`, not
        // the nominal `d`. The fixture's carrier is placed at the `f64`
        // `r + d`, so the geometry's true offset is that rounding's
        // result — about 2.1e-17 m above `d` here. Asserting against the
        // nominal value fails, and it fails for the right reason: the
        // enclosure is sound and the expectation was not. (It did fail,
        // the first time this row ran.)
        let representable = (r + d) - r;
        let m = certified.certificate().max_residual;
        assert!(
            m.lo() <= representable && representable <= m.hi(),
            "the certified residual enclosure {m:?} must contain the constructed drift \
             {representable:e}"
        );
    }

    /// The **cone re-baseline** survives the widening: the collapsed
    /// meter's enclosure on a cone seam must contain `d·sec α`, the
    /// radial chord, and must NOT sit at the perpendicular distance `d`
    /// the pre-collapse arm metered. If an interval lane could still
    /// certify at `d`, the `sec α` finding would be an `f64` artifact
    /// rather than the geometry it is.
    #[test]
    fn the_cone_rebaseline_survives_the_interval_widening() {
        let alpha = core::f64::consts::FRAC_PI_6;
        let d = 2.5e-10;
        let (s, c) = alpha.sin_cos();
        let (keys, lookup) = table(vec![Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
            half_angle: iv(alpha),
            u_ref: Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
        }]);
        let carrier = Curve3::Line {
            origin: Point3::new(iv(c * d), iv(0.0), iv(-s * d)),
            dir: Vec3::new(iv(s), iv(0.0), iv(c)),
        };
        let (t0, t1) = (iv(0.5), iv(2.0));
        let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
        let certified = EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeGeometry::Seam { surface: keys[0] },
                carrier,
                param_start: t0,
                param_end: t1,
            },
            p0,
            p1,
            &lookup,
            band(),
        )
        .expect("a 2.5e-10 drift is inside the band even after the sec α re-baseline");
        let m = certified.certificate().max_residual;
        let chord = d / c;
        assert!(
            m.hi() >= chord * (1.0 - 1e-6),
            "the collapsed meter's enclosure {m:?} must reach the radial chord {chord:e} \
             — if it stops at the perpendicular distance {d:e}, the sec α re-baseline is \
             an f64 artifact and the disposition is wrong"
        );
    }
}
