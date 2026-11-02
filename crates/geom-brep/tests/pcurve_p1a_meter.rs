//! **The collapsed conventional meter's disposition** (PCURVE P-1a
//! fix pass): what `|C(t) − S(P(t))|` measures against the seam arm's
//! pre-collapse `implicit_residual`, and that the description lane and
//! the pcurve cache read one magnitude on one edge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::surf::table;
use geom::{Curve3, Surface};
use geom_brep::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, PcurveCache, chart_pcurve,
    implicit_residual,
};
use geom_core::{Band, Point3, Vec3};

/// The rows' own ε, and the band built from it. Fixed rather than the
/// run's, so a row about the METER does not become a row about the
/// matrix point it ran at.
const ROW_EPS: f64 = 1.0e-9;

/// Built from `ROW_EPS` above, not from the run's ε, and therefore
/// deliberately not `shared::tol::band` — see that constant's note.
///
fn band() -> Band {
    Band::new(ROW_EPS, 10.0 * ROW_EPS).expect("the rows' own band")
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

/// **A cone seam reads its perpendicular drift.** The ruling's image
/// projects on the generator, so the collapsed meter reads the
/// distance `d` to the cone, and a drift of `0.98 ε` certifies at it.
#[test]
fn a_cone_edge_inside_the_band_certifies_at_its_perpendicular_drift() {
    let alpha = core::f64::consts::FRAC_PI_6;
    let d = 0.98 * ROW_EPS;
    let (cone, carrier, t0, t1) = cone_seam(alpha, d);
    let (keys, lookup) = table(vec![cone.clone()]);
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
    let legacy = implicit_residual(&cone, carrier.eval(t0)).abs();
    assert!(
        (legacy / d - 1.0).abs() < 1e-6,
        "the fixture's perpendicular drift is {legacy:e}, not {d:e}"
    );
    let read = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::wrap(keys[0]),
            carrier,
            param_start: t0,
            param_end: t1,
        },
        p0,
        p1,
        &lookup,
        1.0,
        band(),
        geom_brep::NurbsLane::certified(),
    )
    .expect("a drift inside the band certifies")
    .certificate()
    .max_residual;
    assert!(
        (read / d - 1.0).abs() < 1e-6,
        "the meter reads {read:e}, and the perpendicular drift is {d:e}"
    );
}

/// **The cache lane reads the same magnitude.** The description lane
/// and the pcurve cache meter one statement through one mint, so on
/// the same cone seam the cache certifies at the same `d`.
#[test]
fn the_cache_lane_reads_the_collapsed_meters_magnitude() {
    let alpha = core::f64::consts::FRAC_PI_6;
    let d = 0.98 * ROW_EPS;
    let (cone, carrier, t0, t1) = cone_seam(alpha, d);
    let pcurve = chart_pcurve(&carrier, &cone, band()).expect("the cone ruling mints");
    let read = PcurveCache::certify(pcurve, t0, t1, &carrier, &cone, band())
        .expect("the cache lane certifies what the description lane certifies")
        .certificate()
        .max_residual;
    assert!(
        (read / d - 1.0).abs() < 1e-6,
        "the cache lane reads {read:e}, and the description lane {d:e}"
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
                description: EdgeDescriptionSpec::wrap(keys[0]),
                carrier,
                param_start: t0,
                param_end: t1,
            },
            p0,
            p1,
            &lookup,
            1.0,
            band(),
            geom_brep::NurbsLane::certified(),
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
    // An ellipse in the pole's tangent plane is off the sphere: the
    // mint's incidence read refuses it, naming the pair.
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let (keys, lookup) = table(vec![sphere]);
    let carrier = Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::unit_z(),
        semi_u: 1.0,
        semi_v: 0.5,
        u_ref: Vec3::unit_x(),
    };
    let (p0, p1) = (carrier.eval(0.0), carrier.eval(1.0));
    let err = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::wrap(keys[0]),
            carrier,
            param_start: 0.0,
            param_end: 1.0,
        },
        p0,
        p1,
        &lookup,
        1.0,
        band(),
        geom_brep::NurbsLane::certified(),
    )
    .expect_err("an ellipse is not a locus of any sphere chart image");
    // The read quotes the miss it decided: the ellipse's far vertex is
    // `√2 − 1` off the unit sphere.
    assert!(
        matches!(
            &err,
            CertifyError::CarrierOffChart {
                chart: geom::SurfaceKind::Sphere,
                carrier: geom::CurveKind::Ellipse,
                why: "the carrier's points are not at the sphere's radius from its centre",
                margin: Some(geom_brep::OffMargin::Past(m)),
            } if (m.diagnostic_f64_for_error_text().value().unwrap_or(f64::NAN) - (2f64.sqrt() - 1.0)).abs() < 1e-12
        ),
        "the refusal must name the (chart, carrier) pair it could not state, and its miss: {err:?}"
    );
}

/// **The same disposition at the INTERVAL scalar.**
///
/// The meter is generic over `T`: an enclosure lane can widen where an
/// `f64` lane is exact, so what it reads on a seam is a claim worth a
/// row at both scalars.
mod at_intervals {
    use crate::shared::interval::iv;
    use crate::shared::surf::table;
    use geom::{Curve3, Surface};
    use geom_brep::{EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec};
    use geom_core::{Band, Bounds, Point3, Vec3};

    /// The rows' own band, fixed rather than the run's — a row about the
    /// meter must not become a row about the matrix point it drew.
    fn band() -> Band {
        Band::new(1.0e-9, 1.0e-8).expect("the row's own band")
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
                description: EdgeDescriptionSpec::wrap(keys[0]),
                carrier,
                param_start: t0,
                param_end: t1,
            },
            p0,
            p1,
            &lookup,
            geom_core::Real::from_f64(1.0),
            band(),
            geom_brep::NurbsLane::certified(),
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

    /// A cone seam's enclosure contains its perpendicular drift `d`,
    /// the ruling's distance from the cone, at the interval scalar too.
    #[test]
    fn a_cone_seam_reads_its_perpendicular_drift_at_intervals() {
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
                description: EdgeDescriptionSpec::wrap(keys[0]),
                carrier,
                param_start: t0,
                param_end: t1,
            },
            p0,
            p1,
            &lookup,
            geom_core::Real::from_f64(1.0),
            band(),
            geom_brep::NurbsLane::certified(),
        )
        .expect("a 2.5e-10 drift is inside the band");
        let m = certified.certificate().max_residual;
        assert!(
            m.hi() >= d * (1.0 - 1e-6) && m.lo() <= d * (1.0 + 1e-6),
            "the enclosure {m:?} must contain the perpendicular drift {d:e}"
        );
    }
}
