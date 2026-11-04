//! **A circle's general lane on a cylinder, cone and torus** (D3's lane
//! choice on a circle's class readings): a circle the incidence read
//! does not show off the chart, that is none of the chart's own circles
//! or whose class reading is in band, takes the projected image, whose
//! check-4 incidence term is `implicit::conic_form_sup` converted through
//! `projected::Meters`. A circle definitely off ends as its incidence
//! read. Every row holds at `f64` and at `Interval`, and at every ε of
//! the run.
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT` naming `circle-chart-incidence`
//! (the incidence term dropped on these charts), the rows standing a
//! measured distance off the chart refuse, and so does the search; naming
//! `form-uncharged` (the form's coefficients' rounding uncharged), the
//! thin-torus rows and the search refuse at `f64`.

use core::f64::consts::TAU;

use geom_core::{Bounds, Interval};
use test_utils::fuzz;

use super::harmonic_sup_rows::{band, covers, sampled_residual, zero};
use super::*;

/// A class reading in band: the geometric mean of the band's two
/// thresholds, in metres.
fn in_band() -> f64 {
    (band().zero() * band().escalate()).sqrt()
}

/// A chart-frame point or direction of `surface`, in the world.
fn world(surface: &Surface<f64>, p: Vec3<f64>) -> Point3<f64> {
    let Some((o, e)) = projected::chart_frame(surface) else {
        unreachable!("the rows' charts are analytic")
    };
    o + e[0] * p.x + e[1] * p.y + e[2] * p.z
}

fn world_dir(surface: &Surface<f64>, v: Vec3<f64>) -> Vec3<f64> {
    world(surface, v) - world(surface, Vec3::zero())
}

/// The circle of centre `c`, in the plane of the orthonormal `u`, `v`,
/// of radius `r`, all in `surface`'s chart frame.
fn framed_circle(
    surface: &Surface<f64>,
    c: Vec3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    r: f64,
) -> Curve3<f64> {
    let (u, v) = (world_dir(surface, u), world_dir(surface, v));
    Curve3::circle(world(surface, c), u.cross(v).normalize(), r, u.normalize())
}

fn cone(half_angle: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.3, -0.2, 0.5),
        axis: Vec3::new(0.2, 0.1, 1.0).normalize(),
        half_angle,
        u_ref: Vec3::new(1.0, 0.0, -0.2).normalize(),
    }
}

fn torus(major: f64, minor: f64) -> Surface<f64> {
    Surface::Torus {
        center: Point3::new(-0.4, 0.7, 0.2),
        axis: Vec3::new(0.0, 0.3, 1.0).normalize(),
        major_radius: major,
        minor_radius: minor,
        u_ref: Vec3::unit_x(),
    }
}

fn cylinder(radius: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.1, 0.2, -0.3),
        axis: Vec3::new(-0.3, 0.0, 1.0).normalize(),
        radius,
        u_ref: Vec3::unit_y(),
    }
}

/// The circle nearest the section of the cone (half-angle `alpha`, on
/// the nappe `h`'s sign names) by the plane through the axis point at
/// height `h` tilted by `tau` about the frame's `x`: centred at the
/// section ellipse's centre, in its plane, its radius the mean of the
/// semi-axes. At `tau = 0` the rim at `h`; tilted, it leaves the cone to
/// second order in `tau`.
fn cone_section_circle(surface: &Surface<f64>, alpha: f64, h: f64, tau: f64) -> Curve3<f64> {
    let (sa, ca) = alpha.sin_cos();
    let (st, ct) = tau.sin_cos();
    let n = Vec3::new(0.0, -st, ct);
    let sigma = h.signum();
    // The generators at azimuth ±π/2 of the nappe meet the plane at the
    // major axis's ends.
    let vertex = |s_phi: f64| {
        let g = Vec3::new(0.0, sa * s_phi, ca) * sigma;
        g * (h * ct / n.dot(g))
    };
    let (p, q) = (vertex(1.0), vertex(-1.0));
    let m = (p + q) * 0.5;
    let semi_major = (p - q).norm() * 0.5;
    let semi_minor = ((m.z * sa / ca).powi(2) - m.y.powi(2)).sqrt();
    let u = Vec3::unit_x();
    let v = (p - q).normalize();
    framed_circle(surface, m, u, v, 0.5 * (semi_major + semi_minor))
}

/// The cone's frame distance `|ρ cos α − σ z sin α|` of a world point.
fn cone_distance(surface: &Surface<f64>, alpha: f64, p: Point3<f64>) -> f64 {
    let Some((o, e)) = projected::chart_frame(surface) else {
        unreachable!("a cone has a frame")
    };
    let w = p - o;
    let (x, y, z) = (w.dot(e[0]), w.dot(e[1]), w.dot(e[2]));
    (x.hypot(y) * alpha.cos() - z.abs() * alpha.sin()).abs()
}

fn sampled_distance(carrier: &Curve3<f64>, distance: impl Fn(Point3<f64>) -> f64) -> f64 {
    (0..4096)
        .map(|i| distance(carrier.eval(f64::from(i) * TAU / 4096.0)))
        .fold(0.0, f64::max)
}

/// The tilt that stands [`cone_section_circle`] `target` off the cone:
/// its distance is quadratic in the tilt, so one probe scales it.
fn cone_tilt_for(surface: &Surface<f64>, alpha: f64, h: f64, target: f64) -> f64 {
    let probe = 1e-3;
    let at = sampled_distance(&cone_section_circle(surface, alpha, h, probe), |p| {
        cone_distance(surface, alpha, p)
    });
    probe * (target / at).sqrt()
}

/// The circle in the torus's crest plane `z = r` through the crest
/// circle's point at azimuth `phi`, its centre `delta` off the axis: the
/// crest's own parallel at `delta = 0`, otherwise none of the torus's
/// circles, `δ²/2r` off it at most.
fn crest_circle(surface: &Surface<f64>, delta: f64, phi: f64) -> Curve3<f64> {
    let Surface::Torus {
        major_radius,
        minor_radius,
        ..
    } = *surface
    else {
        unreachable!("a crest circle is a torus's")
    };
    let (s, c) = phi.sin_cos();
    let centre = Vec3::new(c, s, 0.0) * delta + Vec3::new(0.0, 0.0, minor_radius);
    framed_circle(
        surface,
        centre,
        Vec3::unit_x(),
        Vec3::unit_y(),
        major_radius,
    )
}

/// The image `chart_pcurve_over` derives over `span`, at `f64` and at
/// `Interval`.
fn derived(
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
    surface: &Surface<f64>,
) -> (
    Result<Pcurve<f64>, PcurveCertifyError>,
    Result<Pcurve<Interval>, PcurveCertifyError>,
) {
    let f = Interval::from_f64;
    (
        chart_pcurve_over(carrier, t0, t1, surface, band()),
        chart_pcurve_over(
            &carrier.map_scalar(f),
            f(t0),
            f(t1),
            &surface.map_scalar(f),
            band(),
        ),
    )
}

/// The projected circle image certifies at both scalars, with envelopes
/// inside the band that cover the sampled residual; answers the `f64`
/// and `Interval` envelopes and the residual.
fn certifies_projected(
    what: &str,
    carrier: &Curve3<f64>,
    span: (f64, f64),
    surface: &Surface<f64>,
) -> (f64, f64, f64) {
    let (at_f64, at_interval) = derived(carrier, span, surface);
    let image = at_f64.unwrap_or_else(|e| panic!("{what}: derives at f64: {e:?}"));
    let image_i = at_interval.unwrap_or_else(|e| panic!("{what}: derives at Interval: {e:?}"));
    assert!(
        image
            .projected()
            .is_some_and(|p| matches!(p.carrier(), projected::FramedCarrier::Circle { .. }))
            && image_i
                .projected()
                .is_some_and(|p| matches!(p.carrier(), projected::FramedCarrier::Circle { .. })),
        "{what}: takes the projected circle image at both scalars"
    );
    let f = Interval::from_f64;
    let cache = PcurveCache::certify(image.clone(), span.0, span.1, carrier, surface, band())
        .unwrap_or_else(|e| panic!("{what}: certifies at f64: {e:?}"));
    let cache_i = PcurveCache::certify(
        image_i,
        f(span.0),
        f(span.1),
        &carrier.map_scalar(f),
        &surface.map_scalar(f),
        band(),
    )
    .unwrap_or_else(|e| panic!("{what}: certifies at Interval: {e:?}"));
    let (envelope, hi) = (
        cache.certificate().envelope,
        cache_i.certificate().envelope.hi(),
    );
    assert!(
        envelope < zero() && hi < zero(),
        "{what}: the envelopes {envelope:e} (f64) and {hi:e} (Interval) are inside the band"
    );
    let sup = sampled_residual(&image, span, carrier, surface, 4096);
    let slack = 64.0 * f64::EPSILON;
    assert!(
        envelope >= sup - slack && hi >= sup - slack,
        "{what}: the envelopes {envelope:e} (f64) and {hi:e} (Interval) cover the sampled \
         residual {sup:e}"
    );
    if sup > zero() / 2.0 {
        covers(what, &image, span, carrier, surface, envelope, hi);
    }
    (envelope, hi, sup)
}

/// The class reading lands in band: the closed-form derivation escalates
/// as a class reading.
fn class_in_band(what: &str, carrier: &Curve3<f64>, span: (f64, f64), surface: &Surface<f64>) {
    let derivation = derive_harmonic(
        carrier,
        surface,
        Placement::over(span, carrier, surface),
        band(),
    );
    assert!(
        matches!(
            derivation,
            Err(PcurveCertifyError::Escalated {
                check: PcurveCheck::ChartClass,
                ..
            })
        ),
        "{what}: the class reading is in band: {derivation:?}"
    );
}

/// A circle of the cone that is no rim (a tilted section's nearest
/// circle, its tilt reading definite) stands `0.4·zero` off the cone and
/// certifies through the projected lane on both nappes, on a narrow and
/// a wide cone, over an arc across the azimuth window's cut and over a
/// whole turn; its envelope covers the residual, which is incidence
/// alone (red under `circle-chart-incidence`).
#[test]
fn a_cone_circle_that_is_no_rim_certifies_through_the_projected_lane() {
    for alpha in [0.15, 1.1] {
        let surface = cone(alpha);
        for h in [2.0, -1.5] {
            let tau = cone_tilt_for(&surface, alpha, h, 0.4 * zero());
            let carrier = cone_section_circle(&surface, alpha, h, tau);
            let what = format!("α {alpha}, h {h}, τ {tau:e}");
            let Some(form) = carrier_harmonic(&carrier) else {
                unreachable!("a circle has a harmonic form")
            };
            let Surface::Cone { axis, .. } = surface else {
                unreachable!("a cone")
            };
            assert!(
                harmonic_sup_scalar(form.a.dot(axis), form.b.dot(axis)) > band().escalate(),
                "{what}: the tilt reads definite"
            );
            for span in [(2.0, 4.5), (0.0, TAU)] {
                let (_, _, sup) = certifies_projected(
                    &format!("{what}, over {span:?}"),
                    &carrier,
                    span,
                    &surface,
                );
                assert!(sup > 0.2 * zero(), "{what}: stands off by {sup:e}");
            }
        }
    }
}

/// A torus crest circle (the crest plane's circle of radius `R` centred
/// off the axis, so none of the torus's circles) standing up to
/// `0.4·zero` off the tube certifies through the projected lane on a
/// ring torus, its tube channel's sector held, and names no iso family;
/// red under `circle-chart-incidence`.
#[test]
fn a_torus_crest_circle_certifies_through_the_projected_lane() {
    for (major, minor) in [(2.0, 0.5), (1.0, 0.9)] {
        let surface = torus(major, minor);
        let delta = (0.8 * minor * zero()).sqrt();
        assert!(
            delta > band().escalate(),
            "the centre's offset reads definite"
        );
        for phi in [0.4, 3.0] {
            let carrier = crest_circle(&surface, delta, phi);
            let what = format!("R {major}, r {minor}, φ {phi}");
            assert_eq!(
                chart_iso_family(&carrier, &surface, band()),
                Ok(None),
                "{what}: a crest circle's image is the general one, on no family"
            );
            for span in [(1.0, 3.9), (0.0, TAU)] {
                let (_, _, sup) = certifies_projected(
                    &format!("{what}, over {span:?}"),
                    &carrier,
                    span,
                    &surface,
                );
                assert!(sup > 0.2 * zero(), "{what}: stands off by {sup:e}");
            }
        }
    }
}

/// A circle whose class reading is in band takes the projected lane and
/// certifies: a rim whose tilt reads in band on the cone (both nappes)
/// and the torus (its outer equator, where an axial move is tangent to
/// the tube), and on the cylinder a circle whose radial amplitude reads
/// in band.
#[test]
fn an_in_band_rim_certifies_through_the_projected_lane() {
    let mut rows: Vec<(String, Surface<f64>, Curve3<f64>)> = Vec::new();
    for h in [1.2, -2.0] {
        let alpha = 0.6;
        let surface = cone(alpha);
        let rho = h.abs() * alpha.tan();
        let carrier = cone_section_circle(&surface, alpha, h, (in_band() / rho).asin());
        rows.push((format!("cone, h {h}"), surface, carrier));
    }
    {
        let surface = torus(2.0, 0.5);
        let rho = 2.5_f64;
        let tau = (in_band() / rho).asin();
        let (st, ct) = tau.sin_cos();
        let carrier = framed_circle(
            &surface,
            Vec3::zero(),
            Vec3::unit_x(),
            Vec3::new(0.0, ct, st),
            rho,
        );
        rows.push(("torus equator".into(), surface, carrier));
    }
    {
        // A tilted rim on a cylinder is a moving circle at any tilt,
        // which its closed form images; the reading a circle near the
        // wall can take in band is its radial amplitude, a circle of
        // in-band radius in the wall's tangent plane.
        let surface = cylinder(0.8);
        let carrier = framed_circle(
            &surface,
            Vec3::new(0.0, 0.8, 1.3),
            Vec3::unit_x(),
            Vec3::unit_z(),
            in_band(),
        );
        rows.push(("cylinder".into(), surface, carrier));
    }
    for (what, surface, carrier) in rows {
        for span in [(-0.7, 2.2), (0.5, 0.5 + TAU)] {
            class_in_band(&what, &carrier, span, &surface);
            if what == "cylinder" {
                band_scale_circle_mints(&carrier, span, &surface);
            } else {
                certifies_projected(&format!("{what}, over {span:?}"), &carrier, span, &surface);
            }
        }
    }
}

/// A circle of in-band radius: its projected image mints at both
/// scalars and check 4's envelope covers its sampled residual, but its
/// certificate escalates at check 2, on the carrier's own speed, as
/// every band-scale circle's does whatever its image.
fn band_scale_circle_mints(carrier: &Curve3<f64>, span: (f64, f64), surface: &Surface<f64>) {
    let (Ok(image), Ok(image_i)) = derived(carrier, span, surface) else {
        panic!("{span:?}: the band-scale circle mints at both scalars")
    };
    let f = Interval::from_f64;
    let lift = |c: &Curve3<f64>| c.map_scalar(f);
    for (scalar, certified) in [
        (
            "f64",
            PcurveCache::certify(image.clone(), span.0, span.1, carrier, surface, band()).err(),
        ),
        (
            "Interval",
            PcurveCache::certify(
                image_i.clone(),
                f(span.0),
                f(span.1),
                &lift(carrier),
                &surface.map_scalar(f),
                band(),
            )
            .err(),
        ),
    ] {
        assert!(
            matches!(
                certified,
                Some(PcurveCertifyError::Escalated {
                    check: PcurveCheck::ParamSpan,
                    ..
                })
            ),
            "{scalar}, {span:?}: escalates on the carrier's speed: {certified:?}"
        );
    }
    let (envelope, hi) = check_4_terms(&image, &image_i, carrier, span, surface)
        .unwrap_or_else(|e| panic!("{span:?}: check 4's terms: {e:?}"));
    let sup = sampled_residual(&image, span, carrier, surface, 4096);
    assert!(
        envelope >= sup - 64.0 * f64::EPSILON && hi >= sup && envelope < zero(),
        "{span:?}: the envelopes {envelope:e} (f64) and {hi:e} (Interval) cover the residual \
         {sup:e} inside the band"
    );
}

/// Check 4's envelope of a projected image over `span`, read straight
/// off its terms at `f64` and at `Interval` (the latter's upper end), so
/// a carrier whose certificate stops at an earlier check still has its
/// lemma read.
fn check_4_terms(
    image: &Pcurve<f64>,
    image_i: &Pcurve<Interval>,
    carrier: &Curve3<f64>,
    span: (f64, f64),
    surface: &Surface<f64>,
) -> Result<(f64, f64), PcurveCertifyError> {
    let f = Interval::from_f64;
    let (Some(p), Some(p_i)) = (image.projected(), image_i.projected()) else {
        panic!("{span:?}: not a projected image")
    };
    let (terms, _) = projected::projected_envelope(
        p,
        span.0,
        span.1,
        &p.chart_box(span.0, span.1),
        carrier,
        surface,
        band(),
        None,
    )?;
    let (terms_i, _) = projected::projected_envelope(
        p_i,
        f(span.0),
        f(span.1),
        &p_i.chart_box(f(span.0), f(span.1)),
        &carrier.map_scalar(f),
        &surface.map_scalar(f),
        band(),
        None,
    )?;
    Ok((terms.total(), terms_i.total().hi()))
}

/// A circle definitely off its chart ends as its incidence read before
/// any lane (E1c): a tilted cone section standing `4·escalate` off, a
/// crest circle centred far off the axis, and a cylinder rim of the
/// wrong radius.
#[test]
fn a_circle_definitely_off_ends_as_its_incidence_read() {
    let cone = cone(0.5);
    let tau = cone_tilt_for(&cone, 0.5, 1.5, 4.0 * band().escalate());
    let torus = torus(2.0, 0.5);
    let cylinder = cylinder(1.0);
    for (what, surface, carrier) in [
        ("cone", &cone, cone_section_circle(&cone, 0.5, 1.5, tau)),
        ("torus", &torus, crest_circle(&torus, 0.05, 1.0)),
        (
            "cylinder",
            &cylinder,
            framed_circle(
                &cylinder,
                Vec3::zero(),
                Vec3::unit_x(),
                Vec3::unit_y(),
                1.0 + 4.0 * band().escalate(),
            ),
        ),
    ] {
        let (at_f64, at_interval) = derived(&carrier, (0.0, 2.0), surface);
        assert!(
            matches!(at_f64, Err(PcurveCertifyError::CarrierOffChart { .. }))
                && matches!(at_interval, Err(PcurveCertifyError::CarrierOffChart { .. })),
            "{what}: ends as its incidence read: {at_f64:?} / {at_interval:?}"
        );
    }
}

/// A chart's form at a world point, its factors read apart so the
/// near-zero one carries no cancellation of the other's size: at
/// `Interval` it encloses the exact form at every point of the point's
/// enclosure. The cylinder `(ρ − R)(ρ + R)`, the cone
/// `(ρ cos α − h sin α)(ρ cos α + h sin α)`, the sphere
/// `(|q| − R)(|q| + R)` and the torus
/// `((ρ − R)² + h² − r²)((ρ + R)² + h² − r²)`.
fn form_at<T: Real>(surface: &Surface<T>, p: Point3<T>) -> T {
    let split = |o: Point3<T>, axis: Vec3<T>| {
        let q = p - o;
        let h = q.dot(axis);
        let rho = (q - axis * h).norm();
        (q, rho, h)
    };
    match *surface {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (_, rho, _) = split(origin, axis);
            (rho - radius) * (rho + radius)
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (_, rho, h) = split(apex, axis);
            let (sa, ca) = half_angle.sin_cos();
            (rho * ca - h * sa) * (rho * ca + h * sa)
        }
        Surface::Sphere { center, radius, .. } => {
            let q = (p - center).norm();
            (q - radius) * (q + radius)
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (_, rho, h) = split(center, axis);
            let tube = |x: T| x.powi(2) + h.powi(2) - minor_radius.powi(2);
            tube(rho - major_radius) * tube(rho + major_radius)
        }
        _ => unreachable!("the rows' charts are the four a circle's lane reads"),
    }
}

/// The least magnitude an `Interval` encloses: `0` where it holds `0`.
fn least_magnitude(x: Interval) -> f64 {
    if x.lo() > 0.0 {
        x.lo()
    } else if x.hi() < 0.0 {
        -x.hi()
    } else {
        0.0
    }
}

/// `implicit::conic_form_sup`'s `f64` bound holds against the form
/// enclosed at `Interval` at dense points of the carrier (the exact form
/// of the carrier as stored is at least [`least_magnitude`] of each
/// enclosure there): thin tori's crest circles (`R` from 1 to 50, `r/R`
/// from `1e-2` to `1e-4`, standing `1e-12` to `1e-9` off, the reviewers'
/// probes on PR 6485), near-horn tori, needle and flat cones near their
/// apex, a sphere's tilted circle and a cylinder's tilted rim, each
/// placed far from the origin too. Red under `form-uncharged`.
#[test]
fn the_form_bound_covers_the_form_enclosed_at_interval() {
    let f = Interval::from_f64;
    let mut rows: Vec<(String, Surface<f64>, Curve3<f64>)> = Vec::new();
    for far in [0.0, 300.0] {
        let at = Point3::new(0.3 + far, -0.7, 0.2 + 0.5 * far);
        let axis = Vec3::new(0.2, -0.4, 1.0).normalize();
        let u_ref = axis.cross(Vec3::unit_x()).normalize();
        for (major, ratio) in [
            (1.0, 1e-2),
            (5.0, 1e-3),
            (20.0, 1e-2),
            (50.0, 1e-4),
            (50.0, 1e-2),
            (2.0, 1.0 - 1e-5),
            (3.0, 0.5),
        ] {
            let surface = Surface::Torus {
                center: at,
                axis,
                major_radius: major,
                minor_radius: major * ratio,
                u_ref,
            };
            for target in [1e-12, 2e-10, 1e-9] {
                let delta = (2.0 * major * ratio * target).sqrt();
                let c = Vec3::new(0.7 * delta, 0.714 * delta, major * ratio);
                rows.push((
                    format!("torus R {major}, r/R {ratio:e}, off {target:e}, far {far}"),
                    surface.clone(),
                    framed_circle(&surface, c, Vec3::unit_x(), Vec3::unit_y(), major),
                ));
            }
        }
        for (alpha, h) in [(0.003, 1e-4), (1.567, 1e-4), (0.6, 2.0)] {
            let surface = Surface::Cone {
                apex: at,
                axis,
                half_angle: alpha,
                u_ref,
            };
            rows.push((
                format!("cone α {alpha}, h {h:e}, far {far}"),
                surface.clone(),
                cone_section_circle(&surface, alpha, h, 1e-5),
            ));
        }
        let sphere = Surface::Sphere {
            center: at,
            radius: 2.0,
            axis,
            u_ref,
        };
        let (st, ct) = 0.3_f64.sin_cos();
        rows.push((
            format!("sphere, far {far}"),
            sphere.clone(),
            framed_circle(
                &sphere,
                Vec3::new(0.0, -0.6 * st, 0.6 * ct),
                Vec3::unit_x(),
                Vec3::new(0.0, ct, st),
                (4.0_f64 - 0.36).sqrt(),
            ),
        ));
        let wall = Surface::Cylinder {
            origin: at,
            axis,
            radius: 0.7,
            u_ref,
        };
        rows.push((
            format!("cylinder, far {far}"),
            wall.clone(),
            framed_circle(
                &wall,
                Vec3::new(0.0, 0.0, 0.4),
                Vec3::unit_x(),
                Vec3::new(0.0, ct, st),
                0.7,
            ),
        ));
    }
    for (what, surface, carrier) in rows {
        let Some(conic) = carrier.conic() else {
            unreachable!("the rows are circles")
        };
        let Some(bound) = crate::implicit::conic_form_sup(&surface, conic) else {
            unreachable!("the rows' charts have a form along a conic")
        };
        let bound =
            super::mutants::form_uncharged(bound.harmonics).unwrap_or_else(|| bound.bound());
        let (carrier_i, surface_i) = (carrier.map_scalar(f), surface.map_scalar(f));
        let exact = (0..2048)
            .map(|i| {
                let t = f(f64::from(i) * TAU / 2048.0);
                least_magnitude(form_at(&surface_i, carrier_i.eval(t)))
            })
            .fold(0.0, f64::max);
        assert!(
            bound >= exact,
            "{what}: the f64 bound {bound:e} is under the form's enclosed magnitude {exact:e}"
        );
    }
}

/// **A circle's or a cone line's projected envelope covers its
/// residual** (a counterexample search; the seed varies). Each draw
/// places a chart at a random scale and axis, at the origin or up to
/// `1e3` from it, and a carrier near it, and mints its projected image
/// directly (`projected::project`, past the class readings, so a carrier
/// at any distance is read):
///
/// - a cone of half-angle `0.003` to `1.567` and a tilted section's
///   nearest circle at a height down to `1e-4` of the scale, standing up
///   to `0.9·zero` off, or tilted in band, or far off;
/// - a torus with `r/R` from `1e-3` to `1 − 1e-5`, and a crest circle (top
///   or bottom) standing up to `0.9·zero` or far off, or a parallel at a
///   random tube angle tilted in band or past it;
/// - a cylinder and a circle of in-band radius on its wall, or a rim
///   tilted and resized;
/// - a ruling of the cone anchored at the apex's height within the band
///   or past it, read over a segment starting as near the apex as `1e-6`
///   of the scale.
///
/// Each carrier is moved by an offset inside the zero band, and a circle
/// read over a random arc that may cross the azimuth window's cut or
/// make a whole turn. Wherever check 4 reads the envelope, its `f64` and
/// `Interval` envelopes cover the densely sampled residual. Red under
/// `form-uncharged` and `circle-chart-incidence`.
#[test]
fn a_circles_projected_envelope_covers_its_residual() {
    let mut s = fuzz::start("pcurve_cache::circle_lane_rows::search");
    let b = band();
    let f = Interval::from_f64;
    let mut tally = [[0usize; 3]; 4];
    let sign = |s: &mut fuzz::Rng| if s.below(2) == 0 { 1.0 } else { -1.0 };
    // A distance off the chart: inside the zero band, or past it.
    let off = |s: &mut fuzz::Rng| {
        if s.below(3) == 0 {
            s.log_uniform(b.escalate(), 1e-4)
        } else {
            b.zero() * s.range(0.01, 0.9)
        }
    };
    for trial in 0..fuzz::scaled(300) {
        let scale = s.log_uniform(0.05, 20.0);
        let far = if s.below(2) == 0 {
            0.0
        } else {
            s.log_uniform(1.0, 1e3)
        };
        let place = Point3::new(far, -0.5 * far, 0.3 * far);
        let axis = Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), 1.0).normalize();
        let u_ref = axis.cross(Vec3::new(0.3, 1.0, -0.2)).normalize();
        let chart = s.below(4);
        let (surface, carrier) = match chart {
            0 => {
                let alpha = s.log_uniform(0.003, 1.567);
                let surface = Surface::Cone {
                    apex: place,
                    axis,
                    half_angle: alpha,
                    u_ref,
                };
                let h = s.log_uniform(1e-4, 2.0) * scale * sign(&mut s);
                let tau = if s.below(2) == 0 {
                    cone_tilt_for(&surface, alpha, h, off(&mut s))
                } else {
                    s.range(b.zero(), b.escalate()) / (h.abs() * alpha.tan())
                };
                (
                    surface.clone(),
                    cone_section_circle(&surface, alpha, h, tau),
                )
            }
            1 => {
                let ratio = if s.below(2) == 0 {
                    s.log_uniform(1e-3, 0.5)
                } else {
                    1.0 - s.log_uniform(1e-5, 0.5)
                };
                let (major, minor) = (scale, scale * ratio);
                let surface = Surface::Torus {
                    center: place,
                    axis,
                    major_radius: major,
                    minor_radius: minor,
                    u_ref,
                };
                let carrier = if s.below(2) == 0 {
                    let delta = (2.0 * minor * off(&mut s)).sqrt();
                    let phi = s.range(-3.1, 3.1);
                    let centre = Vec3::new(phi.cos(), phi.sin(), 0.0) * delta
                        + Vec3::new(0.0, 0.0, minor * sign(&mut s));
                    framed_circle(&surface, centre, Vec3::unit_x(), Vec3::unit_y(), major)
                } else {
                    let theta = s.range(-3.1, 3.1);
                    let (z, rho) = (minor * theta.sin(), major + minor * theta.cos());
                    let tau = (off(&mut s) / rho).min(0.5).asin();
                    let (st, ct) = tau.sin_cos();
                    framed_circle(
                        &surface,
                        Vec3::new(0.0, 0.0, z),
                        Vec3::unit_x(),
                        Vec3::new(0.0, ct, st),
                        rho,
                    )
                };
                (surface, carrier)
            }
            2 => {
                let radius = scale;
                let surface = Surface::Cylinder {
                    origin: place,
                    axis,
                    radius,
                    u_ref,
                };
                let carrier = if s.below(2) == 0 {
                    let phi = s.range(-3.1, 3.1);
                    let (sp, cp) = phi.sin_cos();
                    let psi = s.range(-1.5, 1.5);
                    let tangent = Vec3::new(-sp, cp, 0.0);
                    framed_circle(
                        &surface,
                        Vec3::new(cp * radius, sp * radius, s.range(-1.0, 1.0) * scale),
                        tangent * psi.cos() + Vec3::unit_z() * psi.sin(),
                        Vec3::unit_z() * psi.cos() - tangent * psi.sin(),
                        s.range(b.zero(), b.escalate()),
                    )
                } else {
                    let tau = (off(&mut s) / radius).min(0.5).asin();
                    let (st, ct) = tau.sin_cos();
                    framed_circle(
                        &surface,
                        Vec3::new(0.0, 0.0, s.range(-1.0, 1.0) * scale),
                        Vec3::unit_x(),
                        Vec3::new(0.0, ct, st),
                        radius + sign(&mut s) * off(&mut s),
                    )
                };
                (surface, carrier)
            }
            _ => {
                let alpha = s.log_uniform(0.003, 1.567);
                let surface = Surface::Cone {
                    apex: place,
                    axis,
                    half_angle: alpha,
                    u_ref,
                };
                let (sa, ca) = alpha.sin_cos();
                let phi = s.range(-3.1, 3.1);
                let g = Vec3::new(sa * phi.cos(), sa * phi.sin(), ca) * sign(&mut s);
                let h = if s.below(2) == 0 {
                    s.range(b.zero(), b.escalate())
                } else {
                    off(&mut s)
                } * sign(&mut s);
                let carrier = Curve3::line(world(&surface, g * (h / ca)), world_dir(&surface, g));
                (surface, carrier)
            }
        };
        let offset = Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0))
            * (0.3 * b.zero() * f64::from(u8::from(s.below(2) == 0)));
        let (carrier, t0, t1) = match carrier {
            Curve3::Circle(geom::Circle {
                frame:
                    geom::Frame {
                        center,
                        axis: n,
                        u_ref: x,
                    },
                radius,
            }) => {
                let t0 = s.range(-4.0, 4.0);
                let t1 = t0
                    + if s.below(4) == 0 {
                        TAU
                    } else {
                        s.range(0.2, 6.0)
                    };
                (Curve3::circle(center + offset, n, radius, x), t0, t1)
            }
            Curve3::Line(geom::Line { origin, dir }) => {
                let t0 = s.log_uniform(1e-6, 1.0) * scale;
                let t1 = t0 + s.log_uniform(1e-2, 3.0) * scale;
                (Curve3::line(origin + offset, dir), t0, t1)
            }
            _ => unreachable!("the draws are circles and lines"),
        };
        let context = || {
            format!(
                "trial {trial}, chart {chart}, scale {scale:e}, far {far:e}, span ({t0}, {t1}), \
                 carrier {carrier:?} on {surface:?} — {}",
                fuzz::replay()
            )
        };
        let (Ok(image), Ok(image_i)) = (
            projected::project(&carrier, Some((t0, t1)), &surface, b),
            projected::project(
                &carrier.map_scalar(f),
                Some((f(t0), f(t1))),
                &surface.map_scalar(f),
                b,
            ),
        ) else {
            tally[chart][0] += 1;
            continue;
        };
        let (image, image_i) = (
            Pcurve::Projected(Box::new(image)),
            Pcurve::Projected(Box::new(image_i)),
        );
        let Ok((envelope, hi)) = check_4_terms(&image, &image_i, &carrier, (t0, t1), &surface)
        else {
            tally[chart][1] += 1;
            continue;
        };
        tally[chart][2] += 1;
        let sup = sampled_residual(&image, (t0, t1), &carrier, &surface, 2048);
        let noise = 16.0 * f64::EPSILON * (scale + far);
        assert!(
            envelope >= sup - noise && hi >= sup - noise,
            "{}: envelopes {envelope:e} / {hi:e} under the residual {sup:e}",
            context()
        );
    }
    for (chart, [none, refused, read]) in tally.iter().enumerate() {
        println!(
            "[fuzz] circle_lane_rows::search, chart {chart}: {none} took no projected image, \
             {refused} refused at check 4, {read} read check 4's envelope"
        );
    }
}

/// The mint's image of `carrier` over `span`, certified, at `f64` and at
/// `Interval`: the envelope's upper end, or the refusal.
fn minted_and_certified(
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
    surface: &Surface<f64>,
) -> [Result<f64, PcurveCertifyError>; 2] {
    let f = Interval::from_f64;
    let (c, s) = (carrier.map_scalar(f), surface.map_scalar(f));
    [
        chart_pcurve_over(carrier, t0, t1, surface, band()).and_then(|image| {
            PcurveCache::certify(image, t0, t1, carrier, surface, band())
                .map(|cache| cache.certificate().envelope)
        }),
        chart_pcurve_over(&c, f(t0), f(t1), &s, band()).and_then(|image| {
            PcurveCache::certify(image, f(t0), f(t1), &c, &s, band())
                .map(|cache| cache.certificate().envelope.hi())
        }),
    ]
}

/// Whether a refusal ends lever-only "not yet", never a tolerance offer
/// or a defect, at a build and at rest.
fn ends_not_yet(e: &PcurveCertifyError) -> bool {
    use crate::recourse::Reading;
    [Reading::Build, Reading::AtRest].into_iter().all(|at| {
        let ending = e.ending(at);
        ending.starts_with(geom_core::NOT_YET_ENDING)
            && !ending.contains(geom_core::KERNEL_LIMIT_RECOURSE)
            && !ending.contains("tolerance")
    })
}

/// **A lane choice's envelope refusal ends "not yet"** (D3: a lane choice
/// refuses with its pose as its lever, never a tolerance offer), at
/// `f64` and at `Interval`:
///
/// - the reviewers' cone ruling read from `9e-4` of the apex (half-angle
///   `0.016`): its incidence is converted on parts halved where the
///   cone's lever spreads, so it certifies at both scalars, its envelope
///   over the residual;
/// - a crest circle on a near-horn torus (`r = R − 1e-5`), whose fidelity
///   reads the tube's core `1e-5` away: at `Interval` its envelope lands
///   in band, the lane's escalation (`GeneralLane`);
/// - a chord of a cylinder dipping `0.6·zero` inside the wall (a line
///   shown off its rulings, so the general lane), whose Bernstein bound
///   reads the dip twice over: its envelope refuses or lands in band at
///   both scalars, ending as the lane's gap.
#[test]
fn a_lane_choices_envelope_refusal_ends_not_yet() {
    let line = Curve3::line(
        Point3::new(
            1.239792126505943e-10,
            -9.746673200094881e-10,
            -1.939344479653521e-9,
        ),
        Vec3::new(
            -0.031367901511575216,
            0.38332687822435824,
            0.9230799310923882,
        ),
    );
    let apex = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::new(
            0.036582851423955946,
            -0.3691082383934535,
            -0.9286661420186344,
        ),
        half_angle: 0.016142392389299435,
        u_ref: Vec3::new(-0.7808453755737287, -0.590498825246728, 0.203940277599489),
    };
    let span = (0.0009067415940370447, 2.4187216369829954);
    // The pose stands `1.3e-10` off the cone: inside the band at the
    // default ε, past it at a smaller one, where its incidence read ends it
    // first.
    let image = projected::project(&line, Some(span), &apex, band())
        .unwrap_or_else(|e| panic!("the apex ruling has a projected image: {e:?}"));
    let sup = sampled_residual(
        &Pcurve::Projected(Box::new(image)),
        span,
        &line,
        &apex,
        4096,
    );
    for (scalar, got) in ["f64", "Interval"]
        .into_iter()
        .zip(minted_and_certified(&line, span, &apex))
    {
        if sup < 0.9 * zero() {
            let envelope =
                got.unwrap_or_else(|e| panic!("apex ruling, {scalar}: certifies: {e:?}"));
            assert!(
                envelope >= sup && envelope < zero(),
                "apex ruling, {scalar}: the envelope {envelope:e} covers the residual {sup:e}"
            );
        } else if sup > 1.1 * zero() {
            assert!(
                matches!(got, Err(PcurveCertifyError::CarrierOffChart { .. })),
                "apex ruling, {scalar}: {sup:e} off ends as its incidence read: {got:?}"
            );
        }
    }

    let horn = torus(1.0, 1.0 - 1e-5);
    let crest = crest_circle(&horn, (0.8 * (1.0 - 1e-5) * zero()).sqrt(), 0.3);
    let [_, at_interval] = minted_and_certified(&crest, (0.0, TAU), &horn);
    assert!(
        at_interval.as_ref().err().is_some_and(|e| {
            matches!(
                e,
                PcurveCertifyError::Escalated {
                    check: PcurveCheck::GeneralLane,
                    ..
                }
            ) && ends_not_yet(e)
        }),
        "near-horn crest circle, Interval: ends not yet: {at_interval:?}"
    );

    let wall = cylinder(1.0);
    let a = (1.0 - 0.6 * zero()).acos();
    let (sa, ca) = a.sin_cos();
    let (q0, q1) = (Vec3::new(ca, -sa, 0.1), Vec3::new(ca, sa, 0.4));
    let chord = Curve3::line(world(&wall, q0), world_dir(&wall, q1 - q0));
    for (scalar, got) in
        ["f64", "Interval"]
            .into_iter()
            .zip(minted_and_certified(&chord, (0.0, 1.0), &wall))
    {
        assert!(
            got.as_ref().err().is_some_and(|e| {
                matches!(
                    e,
                    PcurveCertifyError::Escalated {
                        check: PcurveCheck::GeneralLane,
                        ..
                    } | PcurveCertifyError::NoGeneralLane {
                        gap: LaneGap::Envelope,
                        ..
                    }
                ) && ends_not_yet(e)
            }),
            "cylinder chord, {scalar}: ends not yet: {got:?}"
        );
    }
}
