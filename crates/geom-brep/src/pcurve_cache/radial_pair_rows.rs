//! Check 4's terms that read one vector at its norm rather than its
//! parts' sum: a turning class's pair against the image's (radius,
//! winding and plane at one `σ`, a pole start's included), the torus's
//! constants, the sphere's and torus tube's frame defects, the projected
//! cone's slant drift, the two fidelity channels on an orthogonal chart,
//! and the cylinder's class selector. Each row stands at its site's
//! band position — the true deviation in the band, the parts' sum over
//! it — and certifies at `f64` and at `Interval`.
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT` naming a site
//! (`super::mutants::SITES`), that site reads the sum again and its row
//! refuses; `fidelity-cone` reads the cone's channels as orthogonal,
//! and the cone's row then finds its envelope under the residual.
use core::f64::consts::{FRAC_PI_4, TAU};

use geom_core::{Bounds, Interval};

use std::sync::Arc;

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;

use super::harmonic_sup_rows::{band, certifies, circle, covers, lift_pcurve, tilted_circle, zero};
use super::*;

fn ellipse(
    center: Point3<f64>,
    axis: Vec3<f64>,
    major: f64,
    minor: f64,
    u_ref: Vec3<f64>,
) -> Curve3<f64> {
    Curve3::Ellipse {
        center,
        axis,
        semi_u: major,
        semi_v: minor,
        u_ref,
    }
}

/// The quarter of the circle about `+z` through `(0, 0, h)` of radius
/// `r` from `+x` to `+y`, as a rational quadratic: a carrier the
/// projected lane images on any chart.
fn quarter_arc(h: f64, r: f64) -> Curve3<f64> {
    let w = core::f64::consts::FRAC_1_SQRT_2;
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![
                Point3::new(r, 0.0, h),
                Point3::new(r, r, h),
                Point3::new(0.0, r, h),
            ],
            vec![1.0, w, 1.0],
        )
        .unwrap(),
    ))
}

fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

fn ring_torus() -> Surface<f64> {
    Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::unit_x(),
    }
}

fn cylinder(radius: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius,
        u_ref: Vec3::unit_x(),
    }
}

fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: FRAC_PI_4,
        u_ref: Vec3::unit_x(),
    }
}

/// The image the mint derives for `carrier` certifies against it.
fn own_image_certifies(what: &str, carrier: &Curve3<f64>, surface: &Surface<f64>) {
    let image = chart_pcurve(carrier, surface, band()).unwrap();
    certifies(what, &image, (0.0, TAU), carrier, surface);
}

/// `class-circle`: an ellipse about a cylinder's axis with semi-axes
/// `R ± δ` stands `δ` off the image the mint derives, the class circle
/// turned at `R`; the radius gap `δ` plus the orientation defect `2δ`
/// read `3δ`. `δ = 0.6·zero`.
#[test]
fn a_cylinder_ellipse_reads_its_deviation_once() {
    let (r, d) = (0.5, 0.6 * zero());
    let carrier = ellipse(
        Point3::origin(),
        Vec3::unit_z(),
        r + d,
        r - d,
        Vec3::unit_x(),
    );
    own_image_certifies("cylinder ellipse", &carrier, &cylinder(r));
}

/// `class-circle`, the radial and axial halves: a sphere parallel's
/// circle `0.55·zero` over its radius and tilted so its axial pair's
/// sup is `0.55·zero` stands `0.78·zero` off; the radial and axial
/// halves read `1.1·zero`.
#[test]
fn a_tilted_parallel_reads_its_radial_and_axial_halves_together() {
    let (h, d) = (0.3f64, 0.55 * zero());
    let radius = (1.0 - h * h).sqrt() + d;
    let carrier = tilted_circle(Point3::new(0.0, 0.0, h), radius, (d / radius).asin());
    own_image_certifies("tilted sphere parallel", &carrier, &unit_sphere());
}

/// `torus-parallel`: the outer equator's circle `0.55·zero` over its
/// radius and tilted so its axial pair's sup is `0.55·zero` stands
/// `0.78·zero` off; the tube gap and the tilt read `1.1·zero`.
#[test]
fn a_tilted_torus_parallel_reads_its_radial_and_axial_halves_together() {
    let d = 0.55 * zero();
    let radius = 2.5 + d;
    let carrier = tilted_circle(Point3::origin(), radius, (d / radius).asin());
    own_image_certifies("tilted torus parallel", &carrier, &ring_torus());
}

/// `torus-parallel-constant`: a parallel moved `δ = 0.8·zero` along
/// the tube's normal at tube angle 45° stands `δ` off its image: the
/// constant's axial part `δ·sin 45°` and the pair's radial `δ·cos 45°`
/// are orthogonal, and read as one displacement. Their triangle sum
/// reads `√2·δ = 1.13·zero`.
#[test]
fn a_torus_parallel_off_the_tube_at_45_degrees_reads_one_displacement() {
    let (big, r, d) = (2.0, 0.5, 0.8 * zero());
    let v = core::f64::consts::FRAC_PI_4;
    let carrier = circle(
        Point3::new(0.0, 0.0, (r + d) * v.sin()),
        Vec3::unit_z(),
        big + (r + d) * v.cos(),
        Vec3::unit_x(),
    );
    own_image_certifies(
        "torus parallel off the tube at 45°",
        &carrier,
        &ring_torus(),
    );
}

/// The torus parallel's reading stays a bound when the constant is not
/// orthogonal to the pair: the same 45° parallel, `0.4·zero` off the
/// tube and its plane tilted so its axial pair's sup is `0.4·zero`,
/// certifies at `f64` and at `Interval` with its envelope over its
/// sampled residual.
#[test]
fn a_tilted_torus_parallel_off_the_tube_stays_a_bound() {
    let (big, r, d) = (2.0, 0.5, 0.4 * zero());
    let v = core::f64::consts::FRAC_PI_4;
    let radius = big + (r + d) * v.cos();
    let carrier = tilted_circle(
        Point3::new(0.0, 0.0, (r + d) * v.sin()),
        radius,
        (d / radius).asin(),
    );
    own_image_certifies(
        "tilted torus parallel off the tube",
        &carrier,
        &ring_torus(),
    );
}

/// `torus-meridian-centre`: a tube meridian translated `δ = 0.6·zero`
/// radially and `δ` axially in its meridian plane stands `√2·δ` off;
/// the centre's radial and axial parts read `2·δ`.
#[test]
fn a_torus_meridian_off_the_spine_both_ways_reads_one_offset() {
    let d = 0.6 * zero();
    let carrier = circle(
        Point3::new(2.0 + d, 0.0, d),
        Vec3::unit_y(),
        0.5,
        Vec3::unit_x(),
    );
    own_image_certifies("torus meridian off the spine", &carrier, &ring_torus());
}

/// A circle of radius `r + δ` about `center` in the plane `x–z` turned
/// by the angle whose sine is `δ/(r + δ)` about `x`: its `cos t`
/// coefficient is in the meridian plane and its `sin t` coefficient
/// leans `δ` out of it.
fn leaning_meridian(center: Point3<f64>, r: f64, d: f64) -> Curve3<f64> {
    let lean = d / (r + d);
    let normal = Vec3::new(0.0, (1.0 - lean * lean).sqrt(), lean);
    circle(center, normal, r + d, Vec3::unit_x())
}

/// `sphere-meridian`: a meridian circle `0.6·zero` over the sphere's
/// radius, its plane leaning `0.6·zero` out of the axis's, stands
/// `0.85·zero` off the great circle; the radius gap and the orientation
/// defect read `1.2·zero`.
#[test]
fn a_leaning_meridian_on_a_sphere_reads_its_deviation_once() {
    let carrier = leaning_meridian(Point3::origin(), 1.0, 0.6 * zero());
    own_image_certifies("leaning sphere meridian", &carrier, &unit_sphere());
}

/// `torus-meridian`: the tube's meridian circle `0.6·zero` over the
/// minor radius, leaning `0.6·zero` out of the meridian plane, stands
/// `0.85·zero` off; the tube gap and the orientation defect read
/// `1.2·zero`.
#[test]
fn a_leaning_meridian_on_a_torus_reads_its_deviation_once() {
    let carrier = leaning_meridian(Point3::new(2.0, 0.0, 0.0), 0.5, 0.6 * zero());
    own_image_certifies("leaning torus meridian", &carrier, &ring_torus());
}

/// `pole-start`: a meridian started at the pole, on a pair whose `sin
/// t` coefficient stands `0.6·zero` axial and `0.6·zero` radial off the
/// image's, reads its whole residual as one pair, `0.85·zero`, at `f64`
/// and at `Interval`; the old split (radius, drift `3·‖a_r‖`, and the
/// orientation's two parts) reads `1.2·zero`. Read off `incidence`
/// directly: the sphere's arm admits circles only, and a circle's
/// radial part here, `‖b_r‖ − |a·e|`, is second order in the tilt that
/// makes its axial part, so no carrier reaches this band position.
#[test]
fn a_pole_start_reads_its_residual_as_one_pair() {
    let d = 0.6 * zero();
    let derivation = Derivation::SphereMeridian {
        pole: true,
        sigma: -1,
        far: false,
    };
    let incidence_of = |form: Harmonic3<f64>| -> f64 {
        let mut terms = EnvelopeTerms::new();
        incidence(form, &unit_sphere(), derivation, 1.0, &mut terms);
        terms.total()
    };
    let form = Harmonic3 {
        c: Point3::origin(),
        a: Vec3::unit_z(),
        b: Vec3::new(1.0 - d, 0.0, d),
        l: Vec3::zero(),
    };
    let f = Interval::from_f64;
    let lift = |v: Vec3<f64>| Vec3::new(f(v.x), f(v.y), f(v.z));
    let form_i = Harmonic3 {
        c: Point3::new(f(0.0), f(0.0), f(0.0)),
        a: lift(form.a),
        b: lift(form.b),
        l: lift(form.l),
    };
    let mut terms_i = EnvelopeTerms::new();
    incidence(
        form_i,
        &unit_sphere().map_scalar(f),
        derivation,
        f(1.0),
        &mut terms_i,
    );
    // `1 − d` rounds at the ulp of one, which the radial part carries.
    let (norm, slack) = (d * 2f64.sqrt(), 4.0 * f64::EPSILON);
    for (scalar, (lo, hi)) in [
        ("f64", (incidence_of(form), incidence_of(form))),
        ("Interval", {
            let o = terms_i.total();
            (o.lo(), o.hi())
        }),
    ] {
        assert!(
            lo <= norm + slack && hi >= norm - slack && hi < zero(),
            "{scalar}: the orientation [{lo:e}, {hi:e}] reads the defect's norm {norm:e}, \
             in the band"
        );
    }
}

/// `zero-winding`: a circle of radius `0.6·zero` about the sphere's
/// pole, its plane at 45°, decides a parallel with winding Zero; its
/// pair's radial half has sup `0.6·zero` and its axial half `0.42·zero`,
/// which together read `1.02·zero` where the circle stands `0.6·zero`
/// off the pole. Its envelope covers that at `f64` and at `Interval`,
/// and `covers` asserts it inside the band, which is what refuses the
/// sum (`1.02·zero`). (Not certified whole: an edge this short is under
/// check 2's span band, as no carrier of a still class is a chart's.)
#[test]
fn a_still_parallel_reads_its_pair_whole() {
    let s = 0.6 * zero();
    let sphere = unit_sphere();
    let carrier = circle(
        Point3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, -1.0, 1.0).normalize(),
        s,
        Vec3::unit_x(),
    );
    let (image, derivation) = derive_harmonic(
        &carrier,
        &sphere,
        super::Placement::own(&carrier, &sphere),
        band(),
    )
    .unwrap();
    assert!(
        matches!(derivation, Derivation::SphereParallel { beta: 0 }),
        "the circle about the pole is a still parallel: {derivation:?}"
    );
    let span = (0.0, TAU);
    let form = carrier_harmonic(&carrier).unwrap();
    let at_f64 = periodic_envelope(
        channels_of(&image),
        &carrier,
        form,
        &sphere,
        span,
        TAU,
        band(),
    )
    .unwrap()
    .total();
    let f = Interval::from_f64;
    let carrier_i = carrier.map_scalar(f);
    let at_interval = periodic_envelope(
        channels_of(&lift_pcurve(&image)),
        &carrier_i,
        carrier_harmonic(&carrier_i).unwrap(),
        &sphere.map_scalar(f),
        (f(span.0), f(span.1)),
        f(TAU),
        band(),
    )
    .unwrap()
    .total()
    .hi();
    covers(
        "still parallel",
        &image,
        span,
        &carrier,
        &sphere,
        at_f64,
        at_interval,
    );
}

/// `frame`: a unit sphere whose stored axis is `1 + e` long (`u_ref`
/// unit and normal to it) has its axial and radial frame defects both
/// `D = 0.6·zero`; they ride `sin v` and `cos v`, so the frame term is
/// `√2·D = 0.85·zero` and a parallel on that chart certifies at `f64`
/// and at `Interval` inside the band, where their sum would read
/// `1.2·zero` from the frame alone.
#[test]
fn a_sphere_frame_off_in_both_directions_reads_its_defects_at_their_norm() {
    let d = 0.6 * zero();
    // `|axis|² − 1 = 2e + e² = D`.
    let e = (1.0 + d).sqrt() - 1.0;
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z() * (1.0 + e),
        u_ref: Vec3::unit_x(),
    };
    let h = 0.3f64;
    let carrier = circle(
        Point3::new(0.0, 0.0, h),
        Vec3::unit_z(),
        (1.0 - h * h).sqrt(),
        Vec3::unit_x(),
    );
    let image = chart_pcurve(&carrier, &sphere, band()).unwrap();
    let envelope = PcurveCache::certify(image, 0.0, TAU, &carrier, &sphere, band())
        .unwrap_or_else(|e| panic!("certifies at f64: {e:?}"))
        .certificate()
        .envelope;
    assert!(
        envelope > zero() / 2.0 && envelope < zero(),
        "the f64 envelope {envelope:e} reads the frame's defects once, in the band"
    );
    let f = Interval::from_f64;
    let (carrier_i, sphere_i) = (carrier.map_scalar(f), sphere.map_scalar(f));
    let image_i = chart_pcurve(&carrier_i, &sphere_i, band()).unwrap();
    PcurveCache::certify(image_i, f(0.0), f(TAU), &carrier_i, &sphere_i, band())
        .unwrap_or_else(|e| panic!("certifies at Interval: {e:?}"));
}

/// `fidelity`: a cylinder rim's stored image moved `0.6·zero` along
/// the azimuth and `0.6·zero` along the axis stands `0.85·zero` off,
/// the two legs of one displacement; the channels' sum reads
/// `1.2·zero`.
#[test]
fn a_stored_image_moved_in_both_channels_reads_one_displacement() {
    let r = 0.5;
    let cyl = cylinder(r);
    let carrier = circle(
        Point3::new(0.0, 0.0, 0.3),
        Vec3::unit_z(),
        r,
        Vec3::unit_x(),
    );
    let d = 0.6 * zero();
    let stored = chart_pcurve(&carrier, &cyl, band())
        .unwrap()
        .translate(d / r, d);
    certifies(
        "fidelity, both channels",
        &stored,
        (0.0, TAU),
        &carrier,
        &cyl,
    );
}

/// `fidelity-cone`: **the cone's channels add.** A 45° cone's rim at
/// slant `v`, its stored image a radian ahead in azimuth and `v` further
/// out along the ruling: the displacement `|2v·g(u + 1) − v·g(u)|` is
/// `1.385·v`, over `√((v·sin α)² + v²) = 1.225·v`, because the chord and
/// the ruling it is joined to meet at an acute angle. The envelope (the
/// channels' sum, `1.707·v` and up) covers it at `f64` and at
/// `Interval`; read as orthogonal it would not.
#[test]
fn the_cones_fidelity_channels_add() {
    let v = 1e3 * zero();
    let surface = cone();
    let carrier = circle(
        Point3::new(0.0, 0.0, v * FRAC_PI_4.cos()),
        Vec3::unit_z(),
        v * FRAC_PI_4.sin(),
        Vec3::unit_x(),
    );
    let stored = chart_pcurve(&carrier, &surface, band())
        .unwrap()
        .translate(1.0, v);
    let (t0, t1) = (0.0, TAU);
    let sup = (0..=4096)
        .map(|i| {
            let t = t0 + (t1 - t0) * f64::from(i) / 4096.0;
            let q = stored.eval(t);
            surface.eval(q.x, q.y).distance(carrier.eval(t))
        })
        .fold(0.0, f64::max);
    assert!(
        sup > 1.38 * v,
        "the displacement {sup:e} is the acute-angle one, past 1.38·v"
    );
    let form = carrier_harmonic(&carrier).unwrap();
    let at_f64 = periodic_envelope(
        channels_of(&stored),
        &carrier,
        form,
        &surface,
        (t0, t1),
        t1,
        band(),
    )
    .unwrap()
    .total();
    let f = Interval::from_f64;
    let carrier_i = carrier.map_scalar(f);
    let at_interval = periodic_envelope(
        channels_of(&lift_pcurve(&stored)),
        &carrier_i,
        carrier_harmonic(&carrier_i).unwrap(),
        &surface.map_scalar(f),
        (f(t0), f(t1)),
        f(t1),
        band(),
    )
    .unwrap()
    .total()
    .hi();
    assert!(at_f64 >= sup, "the f64 envelope {at_f64:e} covers {sup:e}");
    assert!(
        at_interval >= sup,
        "the Interval envelope {at_interval:e} covers {sup:e}"
    );
}

/// The projected envelope never reads its channels as orthogonal: its
/// `FidelityV` carries the deck sign, the cone's slant drift and the
/// torus's major radius beside the channel, none of them a leg of the
/// channel's displacement. A cylinder image moved in both channels
/// (where the closed-form lane would set it) sums them.
#[test]
fn the_projected_envelope_sums_its_channels() {
    let surface = cylinder(0.5);
    let carrier = quarter_arc(0.3, 0.5);
    let (t0, t1) = (0.0, 1.0);
    let mut image = projected::project(&carrier, Some((t0, t1)), &surface, band()).unwrap();
    image.u_off += 0.6 * zero();
    image.v_off += 0.6 * zero();
    let boxed = Pcurve::Projected(Box::new(image.clone())).chart_box(t0, t1);
    let lane = Some(crate::FittedLane::<f64>::certified());
    let terms =
        projected::projected_envelope(&image, t0, t1, &boxed, &carrier, &surface, band(), lane)
            .map(|(terms, _)| terms)
            .unwrap();
    let (u, v) = (
        terms.0[EnvelopeTerm::FidelityU.slot()],
        terms.0[EnvelopeTerm::FidelityV.slot()],
    );
    assert!(
        u > 0.0 && v > 0.0,
        "both channels moved: FidelityU {u:e}, FidelityV {v:e}"
    );
    let sum = terms.0.iter().sum::<f64>();
    assert_eq!(
        terms.1,
        FidelityLegs::Summed,
        "the projected envelope reads its channels added"
    );
    assert_eq!(terms.total(), sum, "the projected envelope sums its terms");
}

/// `cone-drift`: a projected image on a 45° cone whose stored
/// half-angle is `Δα = 0.8·zero` per metre of slant off the chart's.
/// The slant direction `(sin α, cos α)` moves `Δα` whole; its radial
/// and axial components read `√2·Δα`. Certifies at `f64` and at
/// `Interval` inside the band.
#[test]
fn a_projected_cone_reads_its_half_angle_drift_at_its_norm() {
    let surface = cone();
    let (s, c) = FRAC_PI_4.sin_cos();
    let carrier = quarter_arc(c, s);
    let (t0, t1) = (0.0, 1.0);
    let drifted = |sin: f64, cos: f64| {
        let alpha = sin.atan2(cos) + 0.8 * zero();
        alpha.sin_cos()
    };
    let lane = Some(crate::FittedLane::<f64>::certified());
    let mut image = projected::project(&carrier, Some((t0, t1)), &surface, band()).unwrap();
    let projected::ProjectedChart::Cone { sin, cos, nappe } = image.chart else {
        panic!("a cone image writes the cone's chart")
    };
    let (sin, cos) = drifted(sin, cos);
    image.chart = projected::ProjectedChart::Cone { sin, cos, nappe };
    let envelope = PcurveCache::certify_projected(image, t0, t1, &carrier, &surface, band(), lane)
        .unwrap_or_else(|e| panic!("certifies at f64: {e:?}"))
        .certificate()
        .envelope;
    assert!(
        envelope > zero() / 2.0 && envelope < zero(),
        "the f64 envelope {envelope:e} reads the drift once, in the band"
    );
    let f = Interval::from_f64;
    let (carrier_i, surface_i) = (carrier.map_scalar(f), surface.map_scalar(f));
    let mut image_i =
        projected::project(&carrier_i, Some((f(t0), f(t1))), &surface_i, band()).unwrap();
    let projected::ProjectedChart::Cone { nappe, .. } = image_i.chart else {
        panic!("a cone image writes the cone's chart")
    };
    image_i.chart = projected::ProjectedChart::Cone {
        sin: f(sin),
        cos: f(cos),
        nappe,
    };
    let lane = Some(crate::FittedLane::<Interval>::certified());
    PcurveCache::certify_projected(image_i, f(t0), f(t1), &carrier_i, &surface_i, band(), lane)
        .unwrap_or_else(|e| panic!("certifies at Interval: {e:?}"));
}

/// `radial-amplitude`: a rim of a cylinder of radius `0.7·escalate`
/// reads its radial circle at `0.7·escalate`, in the band, and takes
/// the meridian (where check 2 then escalates, as the moving class's
/// winding gate does: no carrier of a cylinder this thin certifies).
/// The coefficient sum reads `1.4·escalate` and decides it moving. The
/// rim's centre is on the axis, so its radial part is exactly zero and
/// names no direction: the meridian, taken in band as a tie-break, reads
/// its azimuth off the chart's `u_ref` (exactly `0`), at `f64` and at
/// `Interval` alike, where it read `atan2(0, 0)` and refused only at
/// `Interval` (PR 5225's delta review, G1). Its placement's lever is the
/// rim's reach from the axis, `0.7·escalate`: not zero, so the copy is
/// named, by the marks' tie-break in band, and both scalars name the
/// same one, a whole number of turns from `0`.
#[test]
fn a_sub_band_rim_reads_its_radial_circle_at_its_sup() {
    let r = 0.7 * band().escalate();
    let surface = cylinder(r);
    let carrier = circle(
        Point3::new(0.0, 0.0, 0.25),
        Vec3::unit_z(),
        r,
        Vec3::unit_x(),
    );
    let derived = derive_harmonic(
        &carrier,
        &surface,
        super::Placement::own(&carrier, &surface),
        band(),
    );
    let Ok((image, Derivation::CylinderMeridian)) = &derived else {
        panic!("f64: the sub-band rim takes the meridian: {derived:?}")
    };
    let Some((at_f64, ..)) = image.harmonic_form() else {
        panic!("f64: the meridian's image is harmonic: {image:?}")
    };
    assert_eq!(
        at_f64.x - TAU * (at_f64.x / TAU).round(),
        0.0,
        "f64: the meridian's azimuth is 0 modulo a turn: {at_f64:?}"
    );
    let f = Interval::from_f64;
    let (carrier, surface) = (carrier.map_scalar(f), surface.map_scalar(f));
    let derived = derive_harmonic(
        &carrier,
        &surface,
        super::Placement::own(&carrier, &surface),
        band(),
    );
    assert!(
        matches!(
            &derived,
            Ok((image, Derivation::CylinderMeridian))
                if image.harmonic_form().is_some_and(|(p0, ..)| {
                    p0.x.lo() <= at_f64.x && at_f64.x <= p0.x.hi()
                })
        ),
        "Interval: the sub-band rim takes the meridian on f64's copy {at_f64:?}: {derived:?}"
    );
}
