//! **Whether a line is a ruling of its chart** ([`ruling_miss`], the one
//! reading the mint's cylinder and cone arms and props' cylinder parse
//! share): metered over the edge's span, so one tilt reads apart on a
//! long edge and shown on a short one, and a line read with its `dir`
//! scaled reads the same. On the cone it is D3's lane choice: a line
//! shown on its generator takes the generator's closed form, and one in
//! band or definitely off it the general lane. Every row holds at `f64`
//! and at `Interval`, and at every ε of the run.
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT` naming `ruling-per-parameter`
//! (the span unread), the short-and-long, scaled and in-band rows refuse;
//! naming `cone-ruling-gate` (no ruling reading on the cone), the shifted
//! ruling and the cone's in-band row refuse.

use geom_core::{Bounds, Interval};

use super::harmonic_sup_rows::{band, zero};
use super::*;

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.25, -0.5, 1.0),
        axis: Vec3::unit_z(),
        radius: 1.5,
        u_ref: Vec3::unit_x(),
    }
}

/// The unit-speed line through the cylinder's wall point at azimuth
/// `phi`, tilted along the wall's tangent by `tilt` off the axis.
fn tilted(phi: f64, tilt: f64) -> Curve3<f64> {
    let Surface::Cylinder { origin, radius, .. } = cylinder() else {
        unreachable!("a cylinder")
    };
    let (s, c) = phi.sin_cos();
    Curve3::line(
        origin + Vec3::new(c, s, 0.0) * radius + Vec3::new(0.0, 0.0, 0.3),
        Vec3::new(-s * tilt.sin(), c * tilt.sin(), tilt.cos()),
    )
}

/// The cone of E4's probe.
fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.4, 0.1, -0.3),
        axis: Vec3::new(0.1, -0.2, 1.0).normalize(),
        half_angle: 0.5,
        u_ref: Vec3::new(1.0, 0.0, -0.1).normalize(),
    }
}

/// The point of the cone's nappe `sigma` at azimuth `phi` and height `h`
/// from the apex, its unit generator, and the unit tangent along its
/// rim there.
fn cone_at(phi: f64, sigma: f64, h: f64) -> (Point3<f64>, Vec3<f64>, Vec3<f64>) {
    let Surface::Cone {
        apex, half_angle, ..
    } = cone()
    else {
        unreachable!("a cone")
    };
    let Some((_, e)) = projected::chart_frame(&cone()) else {
        unreachable!("a cone has a frame")
    };
    let (sa, ca) = half_angle.sin_cos();
    let radial = e[0] * phi.cos() + e[1] * phi.sin();
    let g = (radial * sa + e[2] * ca) * sigma;
    let tangent = e[1] * phi.cos() - e[0] * phi.sin();
    (apex + g * (h / ca), g, tangent)
}

fn lift(carrier: &Curve3<f64>, surface: &Surface<f64>) -> (Curve3<Interval>, Surface<Interval>) {
    let f = Interval::from_f64;
    (carrier.map_scalar(f), surface.map_scalar(f))
}

/// The derivation the mint selects over `span`, at `f64` and at
/// `Interval`.
fn derivations(
    carrier: &Curve3<f64>,
    span: (f64, f64),
    surface: &Surface<f64>,
) -> [Result<Derivation, PcurveCertifyError>; 2] {
    let f = Interval::from_f64;
    let (carrier_i, surface_i) = lift(carrier, surface);
    let span_i = (f(span.0), f(span.1));
    [
        derive_harmonic(
            carrier,
            surface,
            Placement::over(span, carrier, surface),
            band(),
        )
        .map(|(_, d)| d),
        derive_harmonic(
            &carrier_i,
            &surface_i,
            Placement::over(span_i, &carrier_i, &surface_i),
            band(),
        )
        .map(|(_, d)| d),
    ]
}

/// The ruling reading over `span` against `ruling`, at both scalars:
/// the `f64` value and the `Interval` enclosure's bounds.
fn reading(carrier: &Curve3<f64>, span: (f64, f64), ruling: Ruling<f64>) -> (f64, (f64, f64)) {
    let f = Interval::from_f64;
    let lift3 = |v: Vec3<f64>| Vec3::new(f(v.x), f(v.y), f(v.z));
    let Curve3::Line(line) = carrier else {
        unreachable!("the rows read lines")
    };
    let point = |p: Point3<f64>| Point3::new(f(p.x), f(p.y), f(p.z));
    let line_i = geom::Line {
        origin: point(line.origin),
        dir: lift3(line.dir),
    };
    let ruling_i = match ruling {
        Ruling::Axis(axis) => Ruling::Axis(lift3(axis)),
        Ruling::Generator { apex, g } => Ruling::Generator {
            apex: point(apex),
            g: lift3(g),
        },
    };
    let at = ruling_miss(&line_i, (f(span.0), f(span.1)), ruling_i).value();
    (ruling_miss(line, span, ruling).value(), (at.lo(), at.hi()))
}

/// The image over `span` certifies at both scalars: its certificate's
/// envelope inside the band.
fn certifies(
    what: &str,
    image: (&Pcurve<f64>, &Pcurve<Interval>),
    carrier: &Curve3<f64>,
    span: (f64, f64),
    surface: &Surface<f64>,
) {
    let f = Interval::from_f64;
    let (carrier_i, surface_i) = lift(carrier, surface);
    let cache = PcurveCache::certify(image.0.clone(), span.0, span.1, carrier, surface, band())
        .unwrap_or_else(|e| panic!("{what}: certifies at f64: {e:?}"));
    let cache_i = PcurveCache::certify(
        image.1.clone(),
        f(span.0),
        f(span.1),
        &carrier_i,
        &surface_i,
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
}

/// Where `refusal` is the general lane's own capability gap ("not yet"):
/// no sector at the pose, or its sector reading in band.
fn not_yet(refusal: &PcurveCertifyError) -> bool {
    matches!(
        refusal,
        PcurveCertifyError::NoGeneralLane {
            gap: LaneGap::Pose { .. },
            ..
        } | PcurveCertifyError::Escalated {
            check: PcurveCheck::GeneralLane,
            ..
        }
    ) && !refusal
        .render(crate::recourse::Reading::Build)
        .contains(geom_core::KERNEL_DEFECT_ENDING)
}

/// The mint over `span` takes the projected (general) lane at both
/// scalars and certifies, or ends "not yet" through the lane's own
/// refusal; never the closed form, never a miss. Answers whether it
/// certified.
fn takes_the_general_lane(
    what: &str,
    carrier: &Curve3<f64>,
    span: (f64, f64),
    surface: &Surface<f64>,
) -> bool {
    let f = Interval::from_f64;
    let (carrier_i, surface_i) = lift(carrier, surface);
    let images = (
        chart_pcurve_over(carrier, span.0, span.1, surface, band()),
        chart_pcurve_over(&carrier_i, f(span.0), f(span.1), &surface_i, band()),
    );
    match images {
        (Ok(image), Ok(image_i)) => {
            assert!(
                image.projected().is_some() && image_i.projected().is_some(),
                "{what}: takes the projected image at both scalars: {image:?}"
            );
            certifies(what, (&image, &image_i), carrier, span, surface);
            true
        }
        (at_f64, at_interval) => {
            assert!(
                at_f64.as_ref().err().is_none_or(not_yet)
                    && at_interval.as_ref().err().is_none_or(not_yet),
                "{what}: certifies or ends not yet at both scalars: {at_f64:?} / \
                 {at_interval:?}"
            );
            false
        }
    }
}

/// **One tilt, a short and a long edge**: a unit-speed line tilted `θ`
/// along the wall leaves the ruling through its start by `θ·L` over a
/// span `L`, and that is what both the mint and props read. Tilted
/// `100·escalate` over `L` with `θ·L = zero/4`, it is shown a ruling: the
/// meridian form, which certifies. Tilted `zero/4` over `L` with
/// `θ·L = 4·escalate`, it is shown no ruling: the general lane, which
/// certifies, where the meridian form would miss the line by `θ·L/2`.
/// Red under `ruling-per-parameter`, which reads `θ` at both: the short
/// edge then takes the general lane, and the long one the meridian form.
#[test]
fn one_tilt_reads_by_its_reach_over_the_edge() {
    let surface = cylinder();
    let short_tilt = 100.0 * band().escalate();
    let short = 0.25 * zero() / short_tilt;
    let long_tilt = 0.25 * zero();
    let long = 4.0 * band().escalate() / long_tilt;
    for phi in [0.3, -2.0] {
        let carrier = tilted(phi, short_tilt);
        let span = (-short / 2.0, short / 2.0);
        let (at, (lo, hi)) = reading(&carrier, span, Ruling::Axis(Vec3::unit_z()));
        assert!(
            at < zero() && hi < zero() && lo <= at,
            "φ {phi}, short: the reading {at:e} ([{lo:e}, {hi:e}]) is shown Zero"
        );
        let [d, d_i] = derivations(&carrier, span, &surface);
        assert!(
            matches!(d, Ok(Derivation::CylinderMeridian))
                && matches!(d_i, Ok(Derivation::CylinderMeridian)),
            "φ {phi}, short: the meridian form at both scalars: {d:?} / {d_i:?}"
        );
        let f = Interval::from_f64;
        let (carrier_i, surface_i) = lift(&carrier, &surface);
        let image = chart_pcurve_over(&carrier, span.0, span.1, &surface, band())
            .unwrap_or_else(|e| panic!("φ {phi}, short: mints at f64: {e:?}"));
        let image_i = chart_pcurve_over(&carrier_i, f(span.0), f(span.1), &surface_i, band())
            .unwrap_or_else(|e| panic!("φ {phi}, short: mints at Interval: {e:?}"));
        certifies(
            &format!("φ {phi}, short"),
            (&image, &image_i),
            &carrier,
            span,
            &surface,
        );
        assert_eq!(
            chart_iso_family(&carrier, span, &surface, band()),
            Ok(Some(IsoFamily::U)),
            "φ {phi}, short: a ruling's family"
        );

        let carrier = tilted(phi, long_tilt);
        let span = (-long / 2.0, long / 2.0);
        let (at, (lo, _)) = reading(&carrier, span, Ruling::Axis(Vec3::unit_z()));
        assert!(
            at > band().escalate() && lo > band().escalate(),
            "φ {phi}, long: the reading {at:e} (lo {lo:e}) is definite"
        );
        let [d, d_i] = derivations(&carrier, span, &surface);
        assert!(
            d.is_err() && d_i.is_err(),
            "φ {phi}, long: no closed form at either scalar: {d:?} / {d_i:?}"
        );
        assert!(
            takes_the_general_lane(&format!("φ {phi}, long"), &carrier, span, &surface),
            "φ {phi}, long: the general lane certifies"
        );
        assert_eq!(
            chart_iso_family(&carrier, span, &surface, band()),
            Ok(None),
            "φ {phi}, long: a line off its rulings is on no family"
        );
    }
}

/// **A line's reading is the line's, not its parametrisation's**: `dir`
/// scaled by `1e-3` and `1e3`, its span scaled back, reads the same
/// margin and takes the same class, on the cylinder (the short edge of
/// [`one_tilt_reads_by_its_reach_over_the_edge`]) and on the cone (a
/// ruling turned in band off its generator). Red under
/// `ruling-per-parameter`, which reads the scale.
#[test]
fn a_scaled_direction_reads_the_same_margin() {
    let short_tilt = 100.0 * band().escalate();
    let short = 0.25 * zero() / short_tilt;
    let (anchor, g, tangent) = cone_at(1.0, 1.0, 1.0);
    let Surface::Cone { apex, .. } = cone() else {
        unreachable!("a cone")
    };
    let theta = cone_in_band_turn(CONE_SPAN.1);
    let cone_line = Curve3::line(anchor, g * theta.cos() + tangent * theta.sin());
    let rows = [
        (
            "cylinder",
            tilted(0.3, short_tilt),
            (-short / 2.0, short / 2.0),
            cylinder(),
            Ruling::Axis(Vec3::unit_z()),
        ),
        (
            "cone",
            cone_line,
            CONE_SPAN,
            cone(),
            Ruling::Generator { apex, g },
        ),
    ];
    for (what, carrier, span, surface, ruling) in rows {
        let Curve3::Line(geom::Line { origin, dir }) = carrier else {
            unreachable!("a line")
        };
        let (base, _) = reading(&carrier, span, ruling);
        let [d, _] = derivations(&carrier, span, &surface);
        for k in [1e-3, 1e3] {
            let scaled = Curve3::line(origin, dir * k);
            let span_k = (span.0 / k, span.1 / k);
            let (at, (lo, hi)) = reading(&scaled, span_k, ruling);
            // The cross product's rounding, metres over the span.
            let slack = 1e-9 * base + 64.0 * f64::EPSILON * (span.1 - span.0);
            assert!(
                (at - base).abs() <= slack && lo <= base + slack && hi >= base - slack,
                "{what}, ×{k}: the reading {at:e} ([{lo:e}, {hi:e}]) is the unscaled {base:e}"
            );
            let [d_k, d_k_i] = derivations(&scaled, span_k, &surface);
            let class = |d: &Result<Derivation, PcurveCertifyError>| {
                d.as_ref()
                    .map(core::mem::discriminant)
                    .map_err(core::mem::discriminant)
            };
            assert!(
                class(&d_k) == class(&d) && class(&d_k_i) == class(&d),
                "{what}, ×{k}: the class is the unscaled one: {d_k:?} / {d_k_i:?} against {d:?}"
            );
        }
    }
}

/// The span the cone's in-band rows read, from the anchor.
const CONE_SPAN: (f64, f64) = (0.0, 1e-2);

/// The turn off the generator in the tangent plane whose reading over
/// `[0, l]` from the anchor lies in band: `sin θ · l` the geometric mean
/// of the band's thresholds.
fn cone_in_band_turn(l: f64) -> f64 {
    ((band().zero() * band().escalate()).sqrt() / l).asin()
}

/// **E4's probe**: the ruling of azimuth 1 anchored at height 2, shifted
/// along the cone's tangent ⊥ the ruling by `s`, over `[0, 1]`. It
/// leaves the cone by `s²·cos α/(2h·tan α)` at most, `0.3·zero` here, so
/// the incidence read shows it on; it leaves its generator by about
/// `s·cos α/2`, far past the band. It takes the general lane and
/// certifies or ends "not yet", on both nappes; never the generator's
/// closed form, which misses it. Red under `cone-ruling-gate`, which
/// takes that closed form and refuses `ResidualExceeded` at check 4.
#[test]
fn a_shifted_ruling_takes_the_general_lane() {
    let surface = cone();
    let Surface::Cone { half_angle, .. } = surface else {
        unreachable!("a cone")
    };
    let h = 2.0;
    let s = (0.3 * zero() * 2.0 * h * half_angle.tan() / half_angle.cos()).sqrt();
    for sigma in [1.0, -1.0] {
        let (anchor, g, tangent) = cone_at(1.0, sigma, h);
        let carrier = Curve3::line(anchor + tangent * s, g);
        let span = (0.0, 1.0);
        let what = format!("σ {sigma}, shifted {s:e}");
        let [d, d_i] = derivations(&carrier, span, &surface);
        assert!(
            !matches!(d, Ok(Derivation::ConeRuling { .. }))
                && !matches!(d_i, Ok(Derivation::ConeRuling { .. })),
            "{what}: not the generator's closed form: {d:?} / {d_i:?}"
        );
        takes_the_general_lane(&what, &carrier, span, &surface);
        assert_eq!(
            chart_iso_family(&carrier, span, &surface, band()),
            Ok(None),
            "{what}: a line off its generators is on no family"
        );
    }
}

/// **A true ruling keeps the closed form**: a generator of either nappe,
/// anchored definitely off the apex's height, over a short and a long
/// span, takes the generator's closed form and certifies at both
/// scalars; its family is the rulings'.
#[test]
fn a_true_ruling_keeps_the_closed_form_on_both_nappes() {
    let surface = cone();
    for sigma in [1.0, -1.0] {
        let nappe = if sigma > 0.0 { Nappe::Pos } else { Nappe::Neg };
        for phi in [1.0, -2.5] {
            let (anchor, g, _) = cone_at(phi, sigma, 1.0);
            let carrier = Curve3::line(anchor, g);
            for span in [(0.5, 2.0), (0.5, 40.0)] {
                let what = format!("σ {sigma}, φ {phi}, over {span:?}");
                let [d, d_i] = derivations(&carrier, span, &surface);
                let ruling = |d: &Result<Derivation, PcurveCertifyError>| matches!(d, Ok(Derivation::ConeRuling { anchored: true, nappe: n }) if *n == nappe);
                assert!(
                    ruling(&d) && ruling(&d_i),
                    "{what}: the generator's closed form at both scalars: {d:?} / {d_i:?}"
                );
                let f = Interval::from_f64;
                let (carrier_i, surface_i) = lift(&carrier, &surface);
                let image = chart_pcurve_over(&carrier, span.0, span.1, &surface, band())
                    .unwrap_or_else(|e| panic!("{what}: mints at f64: {e:?}"));
                let image_i =
                    chart_pcurve_over(&carrier_i, f(span.0), f(span.1), &surface_i, band())
                        .unwrap_or_else(|e| panic!("{what}: mints at Interval: {e:?}"));
                certifies(&what, (&image, &image_i), &carrier, span, &surface);
                assert_eq!(
                    chart_iso_family(&carrier, span, &surface, band()),
                    Ok(Some(IsoFamily::U)),
                    "{what}: a ruling's family"
                );
            }
        }
    }
}

/// **An in-band tilt on each chart**: a line whose ruling reading over a
/// span of `1e-2` lies in the band takes the general lane and names its
/// family undecided, on the cylinder (tilted along the wall) and on the
/// cone (turned off its generator in the tangent plane, on both nappes).
/// Red under `ruling-per-parameter`, which reads the tilt 100× over, and
/// on the cone under `cone-ruling-gate`, which names it a ruling.
#[test]
fn an_in_band_tilt_takes_the_general_lane_on_each_chart() {
    let in_band = (band().zero() * band().escalate()).sqrt();
    let l = CONE_SPAN.1;
    let mut rows = vec![(
        "cylinder".to_string(),
        tilted(0.3, (in_band / l).asin()),
        (-l / 2.0, l / 2.0),
        cylinder(),
        Ruling::Axis(Vec3::unit_z()),
    )];
    let Surface::Cone { apex, .. } = cone() else {
        unreachable!("a cone")
    };
    for sigma in [1.0, -1.0] {
        let (anchor, g, tangent) = cone_at(1.0, sigma, 1.0);
        let theta = cone_in_band_turn(l);
        rows.push((
            format!("cone, σ {sigma}"),
            Curve3::line(anchor, g * theta.cos() + tangent * theta.sin()),
            CONE_SPAN,
            cone(),
            Ruling::Generator { apex, g },
        ));
    }
    for (what, carrier, span, surface, ruling) in rows {
        let (at, (lo, hi)) = reading(&carrier, span, ruling);
        assert!(
            zero() < lo && hi < band().escalate(),
            "{what}: the reading {at:e} ([{lo:e}, {hi:e}]) is in the band"
        );
        let [d, d_i] = derivations(&carrier, span, &surface);
        assert!(
            matches!(
                (&d, &d_i),
                (
                    Err(PcurveCertifyError::Escalated {
                        check: PcurveCheck::ChartClass,
                        ..
                    }),
                    Err(PcurveCertifyError::Escalated {
                        check: PcurveCheck::ChartClass,
                        ..
                    })
                )
            ),
            "{what}: the class reading is in band at both scalars: {d:?} / {d_i:?}"
        );
        takes_the_general_lane(&what, &carrier, span, &surface);
        assert!(
            matches!(
                chart_iso_family(&carrier, span, &surface, band()),
                Err(IsoFamilyRefusal::Undecided(_))
            ),
            "{what}: names its family undecided: {:?}",
            chart_iso_family(&carrier, span, &surface, band())
        );
    }
}

/// REVIEW PROBE (6504): the meridian image's azimuth is read at the
/// line's ORIGIN, the reading is metered from the SPAN START. A line
/// tilted along the wall at its origin, read over a short span far from
/// the origin.
#[test]
fn review_probe_origin_off_the_span() {
    let surface = cylinder();
    for (theta, k, l) in [
        (1e3 * zero(), 1.0, 1e-4),
        (1e2 * zero(), 1.0, 1e-3),
        (10.0 * band().escalate(), 0.5, 1e-2 * zero() / band().escalate()),
    ] {
        let carrier = tilted(0.3, theta);
        let span = (k, k + l);
        let (at, _) = reading(&carrier, span, Ruling::Axis(Vec3::unit_z()));
        let [d, d_i] = derivations(&carrier, span, &surface);
        let mint = chart_pcurve_over(&carrier, span.0, span.1, &surface, band());
        let fam = chart_iso_family(&carrier, span, &surface, band());
        let cert = mint.as_ref().ok().map(|image| {
            PcurveCache::certify(image.clone(), span.0, span.1, &carrier, &surface, band())
                .map(|c| c.certificate().envelope)
        });
        eprintln!(
            "PROBE θ {theta:e} k {k} l {l:e}: zero {:e} esc {:e} reading {at:e}\n  d {d:?}\n  d_i {d_i:?}\n  mint {mint:?}\n  certify {cert:?}\n  family {fam:?}",
            zero(),
            band().escalate()
        );
    }
}

/// REVIEW PROBE (6504): cone sweep. Reports any ConeRuling that fails to
/// certify, and any mint ending in a non-"not yet" refusal other than
/// off-chart.
#[test]
fn review_probe_cone_sweep() {
    let surface = cone();
    let Surface::Cone { apex, axis, .. } = surface else { unreachable!() };
    let mut bad = Vec::new();
    let mut n = 0;
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    for sigma in [1.0, -1.0] {
        for phi in [1.0, -2.5, 3.1415] {
            for h in [1e-6, 0.3, 2.0] {
                let (anchor, g, tangent) = cone_at(phi, sigma, h);
                for theta in [0.0, 1e-13, 1e-11, 1e-10, 3e-10, 1e-9, 3e-9, 1e-8, 1e-7, 1e-5, 1e-3] {
                    for shift in [0.0, 1e-9, 1e-7, 1e-5] {
                        for span in [(0.0, 1.0), (0.5, 0.501), (3.0, 3.001), (-0.5, 0.5), (0.0, 1e-3)] {
                            let dir = g * theta.cos() + tangent * theta.sin();
                            let carrier = Curve3::line(anchor + tangent * shift, dir);
                            n += 1;
                            let f = Interval::from_f64;
                            let (ci, si) = lift(&carrier, &surface);
                            let [d, d_i] = derivations(&carrier, span, &surface);
                            for (sc, d, mint) in [
                                ("f64", d, chart_pcurve_over(&carrier, span.0, span.1, &surface, band()).map(|_| ()).and_then(|_| {
                                    let im = chart_pcurve_over(&carrier, span.0, span.1, &surface, band()).unwrap();
                                    PcurveCache::certify(im, span.0, span.1, &carrier, &surface, band()).map(|_| ())
                                })),
                                ("iv", d_i, chart_pcurve_over(&ci, f(span.0), f(span.1), &si, band()).and_then(|im| {
                                    PcurveCache::certify(im, f(span.0), f(span.1), &ci, &si, band()).map(|_| ())
                                })),
                            ] {
                                let key = format!(
                                    "{sc} {} -> {}",
                                    match &d { Ok(d) => format!("{:?}", core::mem::discriminant(d)).len().to_string() + &format!("{d:?}").split(' ').next().unwrap().to_string(), Err(e) => format!("{e:?}").split(['{', '(', ' ']).next().unwrap().to_string() },
                                    match &mint { Ok(()) => "certified".to_string(), Err(e) => format!("{e:?}").split(['{', '(', ' ']).next().unwrap().to_string() }
                                );
                                *tally.entry(key).or_default() += 1;
                                let bad_end = match &mint {
                                    Ok(()) => false,
                                    Err(PcurveCertifyError::CarrierOffChart { .. }) => false,
                                    Err(e) => !not_yet(e) && !matches!(e, PcurveCertifyError::Escalated { .. }),
                                };
                                if bad_end || (matches!(d, Ok(Derivation::ConeRuling { .. })) && mint.is_err()) {
                                    bad.push(format!("σ{sigma} φ{phi} h{h} θ{theta:e} s{shift:e} {span:?} {sc}: {d:?} -> {mint:?}"));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // Lines through the apex: along the axis, across it, and a generator.
    for dir in [axis, Vec3::new(1.0, 0.3, 0.0), cone_at(0.7, 1.0, 1.0).1] {
        for span in [(0.0, 1e-10), (0.0, 1e-3), (-1e-3, 1e-3), (-1.0, 1.0)] {
            let carrier = Curve3::line(apex, dir);
            let m = chart_pcurve_over(&carrier, span.0, span.1, &surface, band());
            let c = m.clone().and_then(|im| PcurveCache::certify(im, span.0, span.1, &carrier, &surface, band()).map(|_| ()));
            let [d, _] = derivations(&carrier, span, &surface);
            eprintln!("APEX dir {dir:?} {span:?}: d {d:?} mint-cert {c:?}");
        }
    }
    eprintln!("cone sweep n {n}");
    for (k, v) in &tally { eprintln!("  {v:6} {k}"); }
    for b in bad.iter().take(40) { eprintln!("BAD {b}"); }
    eprintln!("bad total {}", bad.len());
}

/// REVIEW PROBE (6504): true generators anchored just past the apex's
/// height band, read over a span away from the apex.
#[test]
fn review_probe_true_ruling_anchored_near_the_apex() {
    let surface = cone();
    let Surface::Cone { half_angle, .. } = surface else { unreachable!() };
    for sigma in [1.0, -1.0] {
        for h in [2.0 * zero(), 10.0 * zero(), 1e-7] {
            let (anchor, g, _) = cone_at(1.0, sigma, h);
            let carrier = Curve3::line(anchor, g);
            let span = (0.5, 2.0);
            let [d, d_i] = derivations(&carrier, span, &surface);
            let c = chart_pcurve_over(&carrier, span.0, span.1, &surface, band())
                .and_then(|im| PcurveCache::certify(im, span.0, span.1, &carrier, &surface, band()).map(|_| ()));
            eprintln!("NEARAPEX σ{sigma} h{h:e} (α {half_angle}): {d:?} / {d_i:?} -> {c:?}");
        }
    }
}
