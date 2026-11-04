//! **A circle's general lane on a cylinder, cone and torus** (D3's lane
//! choice on a circle's class readings): a circle the incidence read
//! does not show off the chart, that is none of the chart's own circles
//! or whose class reading is in band, takes the projected image, whose
//! check-4 incidence term is `implicit::conic_form_sup` converted through
//! `projected::Meters`. A circle definitely off ends as its incidence
//! read. Every row holds at `f64` and at `Interval`, and at every ε of
//! the run.
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT` naming `circle-incidence`
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
pub(super) fn in_band() -> f64 {
    (band().zero() * band().escalate()).sqrt()
}

/// A chart-frame point or direction of `surface`, in the world.
pub(super) fn world(surface: &Surface<f64>, p: Vec3<f64>) -> Point3<f64> {
    let Some((o, e)) = projected::chart_frame(surface) else {
        unreachable!("the rows' charts are analytic")
    };
    o + e[0] * p.x + e[1] * p.y + e[2] * p.z
}

pub(super) fn world_dir(surface: &Surface<f64>, v: Vec3<f64>) -> Vec3<f64> {
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

pub(super) fn cone(half_angle: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.3, -0.2, 0.5),
        axis: Vec3::new(0.2, 0.1, 1.0).normalize(),
        half_angle,
        u_ref: Vec3::new(1.0, 0.0, -0.2).normalize(),
    }
}

pub(super) fn torus(major: f64, minor: f64) -> Surface<f64> {
    Surface::Torus {
        center: Point3::new(-0.4, 0.7, 0.2),
        axis: Vec3::new(0.0, 0.3, 1.0).normalize(),
        major_radius: major,
        minor_radius: minor,
        u_ref: Vec3::unit_x(),
    }
}

pub(super) fn cylinder(radius: f64) -> Surface<f64> {
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
pub(super) fn cone_distance(surface: &Surface<f64>, alpha: f64, p: Point3<f64>) -> f64 {
    let Some((o, e)) = projected::chart_frame(surface) else {
        unreachable!("a cone has a frame")
    };
    let w = p - o;
    let (x, y, z) = (w.dot(e[0]), w.dot(e[1]), w.dot(e[2]));
    (x.hypot(y) * alpha.cos() - z.abs() * alpha.sin()).abs()
}

pub(super) fn sampled_distance(carrier: &Curve3<f64>, distance: impl Fn(Point3<f64>) -> f64) -> f64 {
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
pub(super) fn derived(
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
pub(super) fn certifies_projected(
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
            .is_some_and(|p| matches!(p.carrier(), projected::FramedCarrier::Conic { .. }))
            && image_i
                .projected()
                .is_some_and(|p| matches!(p.carrier(), projected::FramedCarrier::Conic { .. })),
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
pub(super) fn class_in_band(what: &str, carrier: &Curve3<f64>, span: (f64, f64), surface: &Surface<f64>) {
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
/// alone (red under `circle-incidence`).
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
/// red under `circle-incidence`.
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
                chart_iso_family(&carrier, super::own_interval(&carrier), &surface, band()),
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
pub(super) fn check_4_terms(
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

/// The exact excess of a cone section's `f64` form coefficients
/// (`implicit::conic_cone_harmonics`, times their `per`, both as stored)
/// over the coefficients of the stored inputs: `Σ_k |f64_k − exact_k|`,
/// each `exact_k` read at both ends of `cos²α`'s and `sin²α`'s
/// enclosures, and the charge `rounding_charge(terms)` it must stay
/// under. The `wall` and `height` coefficients are exact rationals of
/// the stored apex, axis, centre, normal, reference and radius
/// (`v = n × u` exactly); only `α`'s sine and cosine are enclosed. The
/// sum bounds what the sup of the harmonics can lose to the
/// coefficients' rounding.
pub(super) fn cone_coefficient_excess(surface: &Surface<f64>, carrier: &Curve3<f64>) -> (bool, f64) {
    use crate::exact_q::Q;
    let Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = *surface
    else {
        unreachable!("a cone row")
    };
    let Some(conic) = carrier.conic() else {
        unreachable!("a circle row")
    };
    let h = crate::implicit::conic_cone_harmonics(conic, apex, axis, half_angle);
    let frame = conic.frame();
    let (a, b) = conic.semi_axes();
    let q = |v: Vec3<f64>| [Q::of(v.x), Q::of(v.y), Q::of(v.z)];
    let dot = |x: &[Q; 3], y: &[Q; 3]| x[0].mul(&y[0]).add(&x[1].mul(&y[1])).add(&x[2].mul(&y[2]));
    let cross = |x: &[Q; 3], y: &[Q; 3]| {
        [
            x[1].mul(&y[2]).sub(&x[2].mul(&y[1])),
            x[2].mul(&y[0]).sub(&x[0].mul(&y[2])),
            x[0].mul(&y[1]).sub(&x[1].mul(&y[0])),
        ]
    };
    let ax = q(axis);
    let perp = |x: &[Q; 3]| {
        let along = dot(&ax, x);
        [
            x[0].sub(&ax[0].mul(&along)),
            x[1].sub(&ax[1].mul(&along)),
            x[2].sub(&ax[2].mul(&along)),
        ]
    };
    let (o, c) = (
        q(apex - Point3::origin()),
        q(frame.origin - Point3::origin()),
    );
    let d = [c[0].sub(&o[0]), c[1].sub(&o[1]), c[2].sub(&o[2])];
    let u = q(frame.u_ref);
    let v = cross(&q(frame.axis), &u);
    let (e, up, vp) = (perp(&d), perp(&u), perp(&v));
    let (a, b, two, half) = (Q::of(a), Q::of(b), Q::of(2.0), Q::of(0.5));
    let (aa, bb) = (a.mul(&a).mul(&dot(&up, &up)), b.mul(&b).mul(&dot(&vp, &vp)));
    let wall = [
        dot(&e, &e).add(&aa.add(&bb).mul(&half)),
        two.mul(&a).mul(&dot(&e, &up)),
        two.mul(&b).mul(&dot(&e, &vp)),
        aa.sub(&bb).mul(&half),
        a.mul(&b).mul(&dot(&up, &vp)),
    ];
    let (h0, hu, hv) = (dot(&d, &ax), a.mul(&dot(&u, &ax)), b.mul(&dot(&v, &ax)));
    let height = [
        h0.mul(&h0).add(&hu.mul(&hu).add(&hv.mul(&hv)).mul(&half)),
        two.mul(&h0).mul(&hu),
        two.mul(&h0).mul(&hv),
        hu.mul(&hu).sub(&hv.mul(&hv)).mul(&half),
        hu.mul(&hv),
    ];
    let (sin_a, cos_a) = Interval::from_f64(half_angle).sin_cos();
    let (cc, ss) = (cos_a * cos_a, sin_a * sin_a);
    let per = Q::of(h.per);
    let stored = [h.c0, h.c1, h.s1, h.c2, h.s2];
    let excess = (0..5).fold(Q::of(0.0), |sum, k| {
        let at = Q::of(stored[k]).mul(&per);
        let worst = [cc.lo(), cc.hi()]
            .into_iter()
            .flat_map(|x| [ss.lo(), ss.hi()].map(|y| (x, y)))
            .map(|(x, y)| {
                at.sub(&Q::of(x).mul(&wall[k]).sub(&Q::of(y).mul(&height[k])))
                    .abs()
            })
            .fold(Q::of(0.0), Q::max);
        sum.add(&worst)
    });
    let charge = geom_core::rounding_charge(h.terms);
    (excess.le(&Q::of(charge)), charge)
}

/// **A cone section's form coefficients stay within their charge**,
/// exactly: [`cone_coefficient_excess`] over the delta verifier's needle
/// (`α 0.003`, a section at height 1 tilted `1e-7`, the apex `1e5` off
/// the origin, whose 50-digit distance from the cone is `2.72e-12` m)
/// and 600 draws of its coefficient search on PR 6485 (needle, wide and
/// middling cones, the apex at the origin or up to `1e5` off it,
/// sections at heights `1e-4` to `1e3` of either nappe, level or tilted
/// up to `1e-2`; a draw tilted past a wide cone's opening is skipped). topo's `reduce::conic_clearance` and `conic_quadric`
/// read these harmonics and charge, so the row is theirs too. Red under
/// `cone-terms-unprojected`.
#[test]
fn a_cone_sections_coefficients_stay_within_their_charge() {
    let mut rows: Vec<(String, Surface<f64>, Curve3<f64>)> = Vec::new();
    let needle = |far: f64, axis: Vec3<f64>, u_ref: Vec3<f64>, alpha: f64| Surface::Cone {
        apex: Point3::new(0.3 + far, -0.7 - 0.6 * far, 0.2 + 0.5 * far),
        axis,
        half_angle: alpha,
        u_ref,
    };
    let axis = Vec3::new(0.2, -0.4, 1.0).normalize();
    let surface = needle(1e5, axis, axis.cross(Vec3::unit_x()).normalize(), 0.003);
    rows.push((
        "the verifier's needle".into(),
        surface.clone(),
        cone_section_circle(&surface, 0.003, 1.0, 1e-7),
    ));
    let mut s = fuzz::start("pcurve_cache::circle_lane_rows::cone_coefficients");
    for i in 0..600 {
        let alpha = match s.below(3) {
            0 => s.log_uniform(1e-4, 0.05),
            1 => s.range(1.52, core::f64::consts::FRAC_PI_2 - 1e-4),
            _ => s.range(0.05, 1.5),
        };
        let far = if s.below(2) == 0 {
            0.0
        } else {
            s.log_uniform(1.0, 1e5)
        };
        let axis =
            Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0)).normalize();
        let u_ref = axis.cross(Vec3::new(0.3, 1.0, -0.2)).normalize();
        let surface = needle(far, axis, u_ref, alpha);
        let h = s.log_uniform(1e-4, 1e3) * if s.below(2) == 0 { 1.0 } else { -1.0 };
        let tau = if s.below(2) == 0 {
            0.0
        } else {
            s.log_uniform(1e-9, 1e-2)
        };
        let carrier = cone_section_circle(&surface, alpha, h, tau);
        // A wide cone tilted past its opening cuts a hyperbola: no
        // section circle.
        if carrier.conic().is_some_and(|c| c.semi_axes().0.is_finite()) {
            rows.push((
                format!("draw {i}: α {alpha:e}, far {far:e}, h {h:e}, τ {tau:e}"),
                surface,
                carrier,
            ));
        }
    }
    for (what, surface, carrier) in rows {
        let (within, charge) = cone_coefficient_excess(&surface, &carrier);
        assert!(
            within,
            "{what}: the coefficients' exact excess is over their charge {charge:e}"
        );
    }
}

/// The verifier's needle (the row above): its `f64` projected envelope
/// covers its 50-digit distance from the cone, `2.7234227197864185e-12`
/// m, and so does its `Interval` envelope. Red under
/// `cone-terms-unprojected`.
#[test]
fn a_needle_cones_far_section_is_covered_at_its_50_digit_distance() {
    let far = 1e5;
    let axis = Vec3::new(0.2, -0.4, 1.0).normalize();
    let surface = Surface::Cone {
        apex: Point3::new(0.3 + far, -0.7 - 0.6 * far, 0.2 + 0.5 * far),
        axis,
        half_angle: 0.003,
        u_ref: axis.cross(Vec3::unit_x()).normalize(),
    };
    let carrier = cone_section_circle(&surface, 0.003, 1.0, 1e-7);
    let exact = 2.723_422_719_786_418_5e-12;
    let b = band();
    let p = projected::project(&carrier, Some((0.0, TAU)), &surface, b)
        .unwrap_or_else(|e| panic!("the section projects: {e:?}"));
    let (t, _) = projected::projected_envelope(
        &p,
        0.0,
        TAU,
        &p.chart_box(0.0, TAU),
        &carrier,
        &surface,
        b,
        None,
    )
    .unwrap_or_else(|e| panic!("the envelope reads: {e:?}"));
    assert!(
        t.total() >= exact,
        "f64 envelope {:e} under the 50-digit distance {exact:e}",
        t.total()
    );
    let f = Interval::from_f64;
    let (ci, si) = (carrier.map_scalar(f), surface.map_scalar(f));
    let pi = projected::project(&ci, Some((f(0.0), f(TAU))), &si, b)
        .unwrap_or_else(|e| panic!("the section projects at Interval: {e:?}"));
    let (ti, _) = projected::projected_envelope(
        &pi,
        f(0.0),
        f(TAU),
        &pi.chart_box(f(0.0), f(TAU)),
        &ci,
        &si,
        b,
        None,
    )
    .unwrap_or_else(|e| panic!("the envelope reads at Interval: {e:?}"));
    assert!(
        ti.total().lo() >= exact,
        "Interval envelope {:?} under the 50-digit distance {exact:e}",
        ti.total()
    );
}

/// **Every sector channel of a lane choice's own image ends "not yet"**,
/// not only the lever (the delta verifier's note on PR 6485: the one
/// home relabels every `SectorRefused` of the mint's fresh image, since
/// a sector is the pose's). A wall's rim circle projected over `2.5`
/// rad, read as one piece (`envelope_lemma_fuzz`'s grazing row, on a
/// circle): the piece's box reaches the axis, so its azimuth sector
/// refuses, `SectorRefused { Azimuth }`, at `f64` and at `Interval`,
/// and [`minted_refusal`] reads it as `LaneGap::Pose { Azimuth }`, ending
/// "not yet" at a build and at rest, never a defect or a tolerance offer.
#[test]
fn a_lane_choices_azimuth_refusal_ends_not_yet() {
    use crate::recourse::Reading;
    const SPAN: f64 = 2.5;
    let b = band();
    let f = Interval::from_f64;
    let surface = cylinder(0.7);
    let carrier = framed_circle(
        &surface,
        Vec3::new(0.0, 0.0, 0.4),
        Vec3::unit_x(),
        Vec3::unit_y(),
        0.7,
    );
    let one_piece = || {
        geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, SPAN, SPAN], 1)
            .unwrap_or_else(|e| panic!("one piece: {e:?}"))
    };
    let azimuth = |refused: &PcurveCertifyError| {
        matches!(
            refused,
            PcurveCertifyError::SectorRefused {
                channel: projected::SectorChannel::Azimuth,
                ..
            }
        )
    };
    let pose = |minted: &PcurveCertifyError| {
        matches!(
            minted,
            PcurveCertifyError::NoGeneralLane {
                gap: LaneGap::Pose {
                    channel: projected::SectorChannel::Azimuth,
                    ..
                },
                ..
            }
        )
    };
    let mut image = projected::project(&carrier, Some((0.0, SPAN)), &surface, b)
        .unwrap_or_else(|e| panic!("the rim projects: {e:?}"));
    image.breaks = one_piece();
    image.azimuth.truncate(1);
    let Err(refused) =
        PcurveCache::certify_projected(image.clone(), 0.0, SPAN, &carrier, &surface, b, None)
    else {
        panic!("a piece reaching the axis certified")
    };
    assert!(azimuth(&refused), "f64: {refused:?}");
    let minted = minted_refusal(
        refused,
        &Pcurve::Projected(Box::new(image)),
        &carrier,
        &surface,
    );
    assert!(pose(&minted), "f64: {minted:?}");
    for reading in [Reading::Build, Reading::AtRest] {
        let ending = minted.ending(reading);
        assert!(
            ending.starts_with(geom_core::NOT_YET_ENDING),
            "{reading:?}: {ending}"
        );
        for not in ["defect", "tolerance"] {
            assert!(!ending.contains(not), "{reading:?}: {ending}");
        }
    }
    let (ci, si) = (carrier.map_scalar(f), surface.map_scalar(f));
    let mut image_i = projected::project(&ci, Some((f(0.0), f(SPAN))), &si, b)
        .unwrap_or_else(|e| panic!("the rim projects at Interval: {e:?}"));
    image_i.breaks = one_piece();
    image_i.azimuth.truncate(1);
    let Err(refused_i) =
        PcurveCache::certify_projected(image_i.clone(), f(0.0), f(SPAN), &ci, &si, b, None)
    else {
        panic!("a piece reaching the axis certified at Interval")
    };
    assert!(azimuth(&refused_i), "Interval: {refused_i:?}");
    let minted_i = minted_refusal(refused_i, &Pcurve::Projected(Box::new(image_i)), &ci, &si);
    assert!(pose(&minted_i), "Interval: {minted_i:?}");
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
/// `form-uncharged` and `circle-incidence`.
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
                        origin: center,
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

/// The mint's image of `carrier` over `span`, certified as the mint
/// certifies its own image ([`minted_refusal`]), at `f64` and at
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
            PcurveCache::certify(image.clone(), t0, t1, carrier, surface, band())
                .map(|cache| cache.certificate().envelope)
                .map_err(|e| minted_refusal(e, &image, carrier, surface))
        }),
        chart_pcurve_over(&c, f(t0), f(t1), &s, band()).and_then(|image| {
            PcurveCache::certify(image.clone(), f(t0), f(t1), &c, &s, band())
                .map(|cache| cache.certificate().envelope.hi())
                .map_err(|e| minted_refusal(e, &image, &c, &s))
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
/// - a ruling of the reviewers' needle cone (half-angle `0.016`), standing
///   `0.13·zero` off, read from `9e-4` of the apex: its incidence is
///   converted on parts halved where the cone's lever spreads, so it
///   certifies at `f64`, its envelope over the residual, and at
///   `Interval` too but where its enclosures' width, read through the
///   fidelity term's lever (the part's box's slant over its least
///   radius, the box's spread rather than the pose's, filed as
///   `work/pcert/the-cones-fidelity-lever-reads-the-boxs-spread-not-the-poses.md`),
///   reaches the band, where it ends "not yet";
/// - a crest circle on a near-horn torus (`r = R − 1e-5`), whose fidelity
///   reads the tube's core `1e-5` away: at `Interval` its envelope lands
///   in band, the lane's escalation (`GeneralLane`);
/// - a chord of a cylinder dipping `0.6·zero` inside the wall (a line
///   shown off its rulings, so the general lane), whose Bernstein bound
///   reads the dip twice over: its envelope refuses or lands in band at
///   both scalars, ending as the lane's gap.
#[test]
fn a_lane_choices_envelope_refusal_ends_not_yet() {
    // The reviewers' needle cone and span, with a ruling of it anchored
    // at the apex's height within the band and standing `0.13·zero` off
    // (their pose stood a fixed `1.298e-10` off, at 60 digits: past the
    // band at a smaller ε, where its incidence read ends it first).
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
    let (sa, ca) = 0.016142392389299435_f64.sin_cos();
    let phi = 1.9_f64;
    let g = Vec3::new(sa * phi.cos(), sa * phi.sin(), ca);
    let normal = Vec3::new(ca * phi.cos(), ca * phi.sin(), -sa);
    let line = Curve3::line(
        world(&apex, g * (in_band() / ca) + normal * (0.13 * zero())),
        world_dir(&apex, g),
    );
    let span = (0.0009067415940370447, 2.4187216369829954);
    let image = chart_pcurve_over(&line, span.0, span.1, &apex, band())
        .unwrap_or_else(|e| panic!("the apex ruling mints: {e:?}"));
    let sup = sampled_residual(&image, span, &line, &apex, 4096);
    // At `Interval` the fidelity's Lipschitz bound near the apex
    // (`1 + v·sin α/ρ`, some thousands here) multiplies the enclosures'
    // own width, which at a small ε reaches the band: the lane's "not
    // yet", the apex's pose its lever.
    for (scalar, got) in ["f64", "Interval"]
        .into_iter()
        .zip(minted_and_certified(&line, span, &apex))
    {
        match got {
            Ok(envelope) => assert!(
                envelope >= sup && envelope < zero(),
                "apex ruling, {scalar}: the envelope {envelope:e} covers the residual {sup:e}"
            ),
            Err(e) => assert!(
                scalar == "Interval" && ends_not_yet(&e),
                "apex ruling, {scalar}: certifies, or at Interval ends not yet: {e:?}"
            ),
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

/// **A stored row's envelope refusal ends as its own term; the mint's own
/// image's ends as the lane's gap** (the provenance [`minted_refusal`]
/// reads). The cylinder chord of
/// [`a_lane_choices_envelope_refusal_ends_not_yet`]: its fresh image,
/// certified as the mint certifies it, ends "not yet". The same image,
/// stored and then moved a whole period off its deck, re-certified as a
/// stored row is, ends as the deck's term (`FidelityU`), the lifted copy
/// the row is, never relabelled. At `f64` and at `Interval`.
#[test]
fn a_stored_rows_envelope_refusal_keeps_its_term() {
    let wall = cylinder(1.0);
    let a = (1.0 - 0.6 * zero()).acos();
    let (sa, ca) = a.sin_cos();
    let (q0, q1) = (Vec3::new(ca, -sa, 0.1), Vec3::new(ca, sa, 0.4));
    let chord = Curve3::line(world(&wall, q0), world_dir(&wall, q1 - q0));
    let span = (0.0, 1.0);
    for (scalar, got) in ["f64", "Interval"]
        .into_iter()
        .zip(minted_and_certified(&chord, span, &wall))
    {
        assert!(
            got.as_ref().err().is_some_and(ends_not_yet),
            "the fresh image, {scalar}: ends not yet: {got:?}"
        );
    }
    let image = chart_pcurve_over(&chord, span.0, span.1, &wall, band())
        .unwrap_or_else(|e| panic!("the chord mints: {e:?}"));
    let moved = image.translate(TAU, 0.0);
    let f = Interval::from_f64;
    for (scalar, got) in [
        (
            "f64",
            PcurveCache::certify(moved.clone(), span.0, span.1, &chord, &wall, band()).err(),
        ),
        (
            "Interval",
            PcurveCache::certify(
                chart_pcurve_over(
                    &chord.map_scalar(f),
                    f(span.0),
                    f(span.1),
                    &wall.map_scalar(f),
                    band(),
                )
                .unwrap_or_else(|e| panic!("the chord mints at Interval: {e:?}"))
                .translate(f(TAU), f(0.0)),
                f(span.0),
                f(span.1),
                &chord.map_scalar(f),
                &wall.map_scalar(f),
                band(),
            )
            .err(),
        ),
    ] {
        assert!(
            matches!(
                got,
                Some(PcurveCertifyError::ResidualExceeded {
                    check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::FidelityU),
                    ..
                })
            ),
            "the stored copy moved a period, {scalar}: ends as the deck's term: {got:?}"
        );
    }
}

/// A carrier turned about `surface`'s axis by `angle`: on a surface of
/// revolution, as much on the chart as before, and no longer the carrier
/// a row minted for the original images.
fn turned(carrier: &Curve3<f64>, surface: &Surface<f64>, angle: f64) -> Curve3<f64> {
    let Some((o, e)) = projected::chart_frame(surface) else {
        unreachable!("the rows' charts are analytic")
    };
    let k = e[2];
    let (s, c) = angle.sin_cos();
    let turn = |v: Vec3<f64>| v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c));
    match *carrier {
        Curve3::Circle(geom::Circle {
            frame:
                geom::Frame {
                    origin: centre,
                    axis: n,
                    u_ref: x,
                },
            radius,
        }) => Curve3::circle(o + turn(centre - o), turn(n), radius, turn(x)),
        Curve3::Line(geom::Line { origin, dir }) => Curve3::line(o + turn(origin - o), turn(dir)),
        _ => unreachable!("the rows' carriers are circles and lines"),
    }
}

/// **A stored row that is not its edge's carrier never ends "not yet"**
/// (the delta verifier's probe on PR 6485): the mint's own image of a
/// carrier `A` (a torus crest circle, a cone ruling read clear of its
/// apex, a cone's tilted section), certified for `A`, re-checked at rest
/// (`recertify`) against `A` turned about the chart's axis by `1e-6` to
/// `1e-2` rad, which is on the chart as much as `A` is. The stored row is
/// wrong, so at `f64` and at `Interval` it ends by its own term's
/// decision, never as the lane's gap, which only the mint's own image
/// takes ([`minted_refusal`]).
#[test]
fn a_foreign_stored_row_never_ends_not_yet() {
    use crate::recourse::Reading;
    let f = Interval::from_f64;
    let torus = torus(2.0, 0.5);
    let cone = cone(1.1);
    let (sa, ca) = 1.1_f64.sin_cos();
    let g = Vec3::new(sa * 0.7_f64.cos(), sa * 0.7_f64.sin(), ca);
    let ruling = Curve3::line(world(&cone, g * (in_band() / ca)), world_dir(&cone, g));
    let tau = cone_tilt_for(&cone, 1.1, 2.0, 0.4 * zero());
    let rows = [
        (
            "torus crest",
            &torus,
            crest_circle(&torus, (0.4 * zero()).sqrt(), 0.3),
            (1.0, 3.9),
        ),
        ("cone ruling", &cone, ruling, (0.5, 2.0)),
        (
            "cone section",
            &cone,
            cone_section_circle(&cone, 1.1, 2.0, tau),
            (2.0, 4.5),
        ),
    ];
    for (what, surface, a, span) in rows {
        let (a_i, s_i) = (a.map_scalar(f), surface.map_scalar(f));
        let row = chart_pcurve_over(&a, span.0, span.1, surface, band())
            .and_then(|image| PcurveCache::certify(image, span.0, span.1, &a, surface, band()))
            .unwrap_or_else(|e| panic!("{what}: A's row certifies at f64: {e:?}"));
        let row_i = chart_pcurve_over(&a_i, f(span.0), f(span.1), &s_i, band())
            .and_then(|image| PcurveCache::certify(image, f(span.0), f(span.1), &a_i, &s_i, band()))
            .unwrap_or_else(|e| panic!("{what}: A's row certifies at Interval: {e:?}"));
        for angle in [1e-6, 1e-4, 1e-2] {
            let b = turned(&a, surface, angle);
            for (scalar, got) in [
                ("f64", row.recertify(&b, span, surface, band(), None).err()),
                (
                    "Interval",
                    row_i
                        .recertify(&b.map_scalar(f), (f(span.0), f(span.1)), &s_i, band(), None)
                        .err(),
                ),
            ] {
                let Some(e) = got else {
                    panic!("{what}, turned {angle:e}, {scalar}: a foreign row certified")
                };
                assert!(
                    !e.ending(Reading::AtRest)
                        .starts_with(geom_core::NOT_YET_ENDING),
                    "{what}, turned {angle:e}, {scalar}: a foreign stored row ends not yet: {e:?}"
                );
            }
        }
    }
}

/// Exact-rational vectors for the coefficient rows.
type Q3 = [crate::exact_q::Q; 3];

fn q3(v: Vec3<f64>) -> Q3 {
    use crate::exact_q::Q;
    [Q::of(v.x), Q::of(v.y), Q::of(v.z)]
}

fn q_dot(x: &Q3, y: &Q3) -> crate::exact_q::Q {
    x[0].mul(&y[0]).add(&x[1].mul(&y[1])).add(&x[2].mul(&y[2]))
}

fn q_sub(x: &Q3, y: &Q3) -> Q3 {
    [x[0].sub(&y[0]), x[1].sub(&y[1]), x[2].sub(&y[2])]
}

fn q_cross(x: &Q3, y: &Q3) -> Q3 {
    [
        x[1].mul(&y[2]).sub(&x[2].mul(&y[1])),
        x[2].mul(&y[0]).sub(&x[0].mul(&y[2])),
        x[0].mul(&y[1]).sub(&x[1].mul(&y[0])),
    ]
}

/// `Σ_k |stored_k − exact_k| ≤ charge`, exactly.
fn q_within(stored: &[f64], exact: &[crate::exact_q::Q], scale: f64, charge: f64) -> bool {
    use crate::exact_q::Q;
    let sum = stored.iter().zip(exact).fold(Q::of(0.0), |sum, (&x, e)| {
        sum.add(&Q::of(x).mul(&Q::of(scale)).sub(e).abs())
    });
    sum.le(&Q::of(charge))
}

/// The exact `trig_product` of two degree-two series, truncated at
/// degree four as `implicit`'s is.
fn q_trig_product(
    a: &([crate::exact_q::Q; 5], [crate::exact_q::Q; 5]),
    b: &([crate::exact_q::Q; 5], [crate::exact_q::Q; 5]),
) -> ([crate::exact_q::Q; 5], [crate::exact_q::Q; 5]) {
    use crate::exact_q::Q;
    let half = Q::of(0.5);
    let mut cos: [Q; 5] = core::array::from_fn(|_| Q::of(0.0));
    let mut sin: [Q; 5] = core::array::from_fn(|_| Q::of(0.0));
    for k in 0..5 {
        for m in 0..5 - k {
            let cc = a.0[k].mul(&b.0[m]).mul(&half);
            let ss = a.1[k].mul(&b.1[m]).mul(&half);
            let cs = a.0[k].mul(&b.1[m]).mul(&half);
            let sc = a.1[k].mul(&b.0[m]).mul(&half);
            let (sum, diff) = (k + m, k.abs_diff(m));
            cos[sum] = cos[sum].add(&cc).sub(&ss);
            cos[diff] = cos[diff].add(&cc).add(&ss);
            sin[sum] = sin[sum].add(&cs).add(&sc);
            match m.cmp(&k) {
                core::cmp::Ordering::Greater => sin[diff] = sin[diff].add(&cs).sub(&sc),
                core::cmp::Ordering::Less => sin[diff] = sin[diff].add(&sc).sub(&cs),
                core::cmp::Ordering::Equal => {}
            }
        }
    }
    (cos, sin)
}

/// **A sphere's, a wall's and a torus's circle coefficients stay within
/// their charge**, exactly (the cone's row above, for the other three
/// charts the circle lane reads; the delta verifier asked on PR 6485
/// whether their `terms` miss the cone's `|d|`-scale rounding). Each
/// draw places the chart at the origin or up to `1e5` off it, at a
/// scale `1e-3` to `1e3`, and a circle of any normal (a third of them
/// the chart's axis) up to `3e3` scales along the axis and up to two
/// off it; the torus's tube ratio runs from `1e-4` to `1 − 1e-5`. The
/// stored coefficients (times their `per` on the quadrics) differ from
/// the exact ones of the stored inputs by at most
/// `rounding_charge(terms)` in sum. Unlike the cone's, these `terms`
/// carry `|C₀ − o|²` and the reach squared whole, which dominate every
/// cross term the projection's rounding enters at.
#[test]
fn the_other_charts_circle_coefficients_stay_within_their_charge() {
    let mut s = fuzz::start("pcurve_cache::circle_lane_rows::other_coefficients");
    for i in 0..400 {
        let far = if s.below(2) == 0 {
            0.0
        } else {
            s.log_uniform(1.0, 1e5)
        };
        let at = Point3::new(0.3 + far, -0.7 - 0.6 * far, 0.2 + 0.5 * far);
        let unit = |s: &mut fuzz::Rng| {
            Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0)).normalize()
        };
        let axis = unit(&mut s);
        let scale = s.log_uniform(1e-3, 1e3);
        let n = if s.below(3) == 0 { axis } else { unit(&mut s) };
        let u = n.cross(Vec3::new(0.3, 1.0, -0.2)).normalize();
        let along = s.range(-3.0, 3.0) * scale * if s.below(2) == 0 { 1.0 } else { 1e3 };
        let off = unit(&mut s) * scale * s.log_uniform(1e-6, 2.0);
        let radius = scale * s.log_uniform(1e-4, 3.0);
        let carrier = Curve3::circle(at + axis * along + off, n, radius, u);
        let Some(conic) = carrier.conic() else {
            unreachable!("a circle")
        };
        let r = scale * s.log_uniform(0.1, 3.0);
        let small = r * s.log_uniform(1e-4, 0.99999);
        let [sphere, wall, torus] = other_charts_within(conic, at, axis, r, small);
        for (what, within) in [("sphere", sphere), ("cylinder", wall), ("torus", torus)] {
            assert!(
                within,
                "draw {i}: the {what}'s coefficients' exact excess is over their charge"
            );
        }
    }
}

/// Whether a conic's exact coefficient excess is within its charge on
/// the sphere `(at, r)`, the wall `(at, axis, r)` and the torus
/// `(at, axis, r, small)`, in that order: the stored coefficients
/// (times their `per` on the quadrics) differ from the exact ones of the
/// stored inputs by at most `rounding_charge(terms)` in sum.
pub(super) fn other_charts_within(
    conic: &dyn geom::Conic<f64>,
    at: Point3<f64>,
    axis: Vec3<f64>,
    r: f64,
    small: f64,
) -> [bool; 3] {
    use crate::exact_q::Q;
    let (two, half) = (Q::of(2.0), Q::of(0.5));
    let mut within = [false; 3];
    let frame = conic.frame();
    let (a, b) = conic.semi_axes();
    let (o, c) = (
        q3(at - Point3::origin()),
        q3(frame.origin - Point3::origin()),
    );
    let d = q_sub(&c, &o);
    let (uq, ax) = (q3(frame.u_ref), q3(axis));
    let vq = q_cross(&q3(frame.axis), &uq);
    let (aq, bq, rq) = (Q::of(a), Q::of(b), Q::of(r));
    // The quadrics: `|⊥(C − o)|² − r²`, `⊥` the identity on the sphere.
    for (slot, wall) in [(0, false), (1, true)] {
        let perp = |x: &Q3| {
            if wall {
                let along = q_dot(&ax, x);
                [
                    x[0].sub(&ax[0].mul(&along)),
                    x[1].sub(&ax[1].mul(&along)),
                    x[2].sub(&ax[2].mul(&along)),
                ]
            } else {
                x.clone()
            }
        };
        let (e, up, vp) = (perp(&d), perp(&uq), perp(&vq));
        let aa = aq.mul(&aq).mul(&q_dot(&up, &up));
        let bb = bq.mul(&bq).mul(&q_dot(&vp, &vp));
        let exact = [
            q_dot(&e, &e).add(&aa.add(&bb).mul(&half)).sub(&rq.mul(&rq)),
            two.mul(&aq).mul(&q_dot(&e, &up)),
            two.mul(&bq).mul(&q_dot(&e, &vp)),
            aa.sub(&bb).mul(&half),
            aq.mul(&bq).mul(&q_dot(&up, &vp)),
        ];
        let h = if wall {
            crate::implicit::conic_cylinder_harmonics(conic, at, axis, r)
        } else {
            crate::implicit::conic_sphere_harmonics(conic, at, r)
        };
        within[slot] = q_within(
            &[h.c0, h.c1, h.s1, h.c2, h.s2],
            &exact,
            h.per,
            geom_core::rounding_charge(h.terms),
        );
    }
    // The torus: `(|w|² + R² − r²)² − 4R²(|w|² − (w·â)²)`, `w = C − o`.
    let z = || Q::of(0.0);
    let aa = aq.mul(&aq).mul(&q_dot(&uq, &uq));
    let bb = bq.mul(&bq).mul(&q_dot(&vq, &vq));
    let sq = (
        [
            q_dot(&d, &d).add(&aa.add(&bb).mul(&half)),
            two.mul(&aq).mul(&q_dot(&d, &uq)),
            aa.sub(&bb).mul(&half),
            z(),
            z(),
        ],
        [
            z(),
            two.mul(&bq).mul(&q_dot(&d, &vq)),
            aq.mul(&bq).mul(&q_dot(&uq, &vq)),
            z(),
            z(),
        ],
    );
    let hq = (
        [q_dot(&d, &ax), aq.mul(&q_dot(&uq, &ax)), z(), z(), z()],
        [z(), bq.mul(&q_dot(&vq, &ax)), z(), z(), z()],
    );
    let (big, tube) = (Q::of(r), Q::of(small));
    let mut pq = sq.clone();
    pq.0[0] = pq.0[0].add(&big.mul(&big).sub(&tube.mul(&tube)));
    let (pp, hh) = (q_trig_product(&pq, &pq), q_trig_product(&hq, &hq));
    let four_rr = Q::of(4.0).mul(&big).mul(&big);
    let exact: Vec<Q> = (0..5)
        .map(|j| pp.0[j].sub(&four_rr.mul(&sq.0[j].sub(&hh.0[j]))))
        .chain((0..5).map(|j| pp.1[j].sub(&four_rr.mul(&sq.1[j].sub(&hh.1[j])))))
        .collect();
    let h = crate::implicit::conic_torus_harmonics(conic, at, axis, r, small);
    let stored: Vec<f64> = h.cos.iter().chain(h.sin.iter()).copied().collect();
    within[2] = q_within(&stored, &exact, 1.0, geom_core::rounding_charge(h.terms));
within
}
