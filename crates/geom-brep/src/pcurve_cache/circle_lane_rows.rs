//! **A circle's general lane on a cylinder, cone and torus** (D3's lane
//! choice on a circle's class readings): a circle the incidence read
//! does not show off the chart, that is none of the chart's own circles
//! or whose class reading is in band, takes the projected image, whose
//! check-4 incidence term is `circle_chart::form_sup` converted through
//! `projected::Meters`. A circle definitely off ends as its incidence
//! read. Every row holds at `f64` and at `Interval`, and at every ε of
//! the run.
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT` naming `circle-chart-incidence`
//! (the incidence term dropped on these charts), the rows standing a
//! measured distance off the chart refuse, and so does the search.

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
/// ring torus, its tube channel's sector held; red under
/// `circle-chart-incidence`.
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

/// **A circle's projected envelope covers its residual** (a
/// counterexample search; the seed varies). Each draw places a cone,
/// torus or cylinder at a random scale, axis and distance from the
/// origin, and a circle near it that its class readings miss or read in
/// band: on the cone a tilted section's nearest circle (its distance
/// from the cone drawn up to `0.9·zero`, or its tilt drawn in band); on
/// the torus a crest circle (top or bottom, its centre off the axis by
/// up to the band's worth of distance) or a parallel at a random tube
/// angle tilted in band; on the cylinder a circle of in-band radius on
/// the wall. Each is then moved by an offset and a radius change inside
/// the zero band, and read over a random arc that may cross the azimuth
/// window's cut or make a whole turn. Whatever takes the projected lane
/// has check 4's envelopes, at `f64` and at `Interval`, over its densely
/// sampled residual.
#[test]
fn a_circles_projected_envelope_covers_its_residual() {
    let mut s = fuzz::start("pcurve_cache::circle_lane_rows::search");
    let b = band();
    let mut tally = [0usize; 4];
    for trial in 0..fuzz::scaled(300) {
        let scale = s.log_uniform(0.05, 20.0);
        let far = if s.below(2) == 0 {
            0.0
        } else {
            s.log_uniform(1.0, 100.0)
        };
        let place = Point3::new(far, -0.5 * far, 0.3 * far);
        let axis = Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), 1.0).normalize();
        let u_ref = axis.cross(Vec3::new(0.3, 1.0, -0.2)).normalize();
        let sign = |s: &mut fuzz::Rng| if s.below(2) == 0 { 1.0 } else { -1.0 };
        let chart = s.below(3);
        let (surface, carrier) = match chart {
            0 => {
                let alpha = s.range(0.1, 1.3);
                let surface = Surface::Cone {
                    apex: place,
                    axis,
                    half_angle: alpha,
                    u_ref,
                };
                let h = s.range(0.3, 2.0) * scale * sign(&mut s);
                let tau = if s.below(2) == 0 {
                    cone_tilt_for(&surface, alpha, h, b.zero() * s.range(0.01, 0.9))
                } else {
                    s.range(b.zero(), b.escalate()) / (h.abs() * alpha.tan())
                };
                let carrier = cone_section_circle(&surface, alpha, h, tau);
                (surface, carrier)
            }
            1 => {
                let (major, minor) = (scale, scale * s.range(0.1, 0.9));
                let surface = Surface::Torus {
                    center: place,
                    axis,
                    major_radius: major,
                    minor_radius: minor,
                    u_ref,
                };
                let carrier = if s.below(2) == 0 {
                    let delta = (2.0 * minor * b.zero() * s.range(0.01, 0.9)).sqrt();
                    let phi = s.range(-3.1, 3.1);
                    let centre = Vec3::new(phi.cos(), phi.sin(), 0.0) * delta
                        + Vec3::new(0.0, 0.0, minor * sign(&mut s));
                    framed_circle(&surface, centre, Vec3::unit_x(), Vec3::unit_y(), major)
                } else {
                    let theta = s.range(-3.1, 3.1);
                    let (z, rho) = (minor * theta.sin(), major + minor * theta.cos());
                    let tau = (s.range(b.zero(), b.escalate()) / rho).asin();
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
            _ => {
                let radius = scale;
                let surface = Surface::Cylinder {
                    origin: place,
                    axis,
                    radius,
                    u_ref,
                };
                let phi = s.range(-3.1, 3.1);
                let (sp, cp) = phi.sin_cos();
                let psi = s.range(-1.5, 1.5);
                let tangent = Vec3::new(-sp, cp, 0.0);
                let carrier = framed_circle(
                    &surface,
                    Vec3::new(cp * radius, sp * radius, s.range(-1.0, 1.0) * scale),
                    tangent * psi.cos() + Vec3::unit_z() * psi.sin(),
                    Vec3::unit_z() * psi.cos() - tangent * psi.sin(),
                    s.range(b.zero(), b.escalate()),
                );
                (surface, carrier)
            }
        };
        let Curve3::Circle(geom::Circle {
            frame:
                geom::Frame {
                    center,
                    axis: n,
                    u_ref: x,
                },
            radius,
        }) = carrier
        else {
            unreachable!("the draws are circles")
        };
        let offset = Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0))
            * (0.3 * b.zero() * f64::from(u8::from(s.below(2) == 0)));
        let dr = 0.3 * b.zero() * s.range(-1.0, 1.0) * f64::from(u8::from(s.below(2) == 0));
        let carrier = Curve3::circle(center + offset, n, (radius + dr).abs(), x);
        let t0 = s.range(-4.0, 4.0);
        let t1 = t0
            + if s.below(4) == 0 {
                TAU
            } else {
                s.range(0.2, 6.0)
            };
        let context = || {
            format!(
                "trial {trial}, chart {chart}, scale {scale:e}, far {far:e}, offset {offset:?}, \
                 dr {dr:e}, span ({t0}, {t1}), carrier {carrier:?} — {}",
                fuzz::replay()
            )
        };
        let (Ok(image), Ok(image_i)) = derived(&carrier, (t0, t1), &surface) else {
            tally[0] += 1;
            continue;
        };
        if image.projected().is_none() || image_i.projected().is_none() {
            tally[0] += 1;
            continue;
        }
        let Ok((envelope, hi)) = check_4_terms(&image, &image_i, &carrier, (t0, t1), &surface)
        else {
            tally[1] += 1;
            continue;
        };
        tally[2] += 1;
        let sup = sampled_residual(&image, (t0, t1), &carrier, &surface, 2048);
        let noise = 64.0 * f64::EPSILON * (scale + far);
        assert!(
            envelope >= sup - noise && hi >= sup - noise,
            "{}: envelopes {envelope:e} / {hi:e} under the residual {sup:e}",
            context()
        );
        if PcurveCache::certify(image, t0, t1, &carrier, &surface, b).is_ok() {
            tally[3] += 1;
        }
    }
    println!(
        "[fuzz] circle_lane_rows::search: {} took no projected image, {} refused at check 4, \
         {} read check 4's envelope, {} of them certified at f64",
        tally[0], tally[1], tally[2], tally[3]
    );
}
