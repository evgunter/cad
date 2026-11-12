//! **Limb 2's first reading of a net, each span in the parts its own
//! turn needs** ([`Parts::ByTurn`]). A long span, or a short one by the
//! chart's axis or a pole, reads the parts a fixed count read there; a
//! short span far from both reads one. A first reading that clears is
//! the limb's; anything else is the rounds', at
//! [`projected::INCIDENCE_CUTS`] parts a span.
//!
//! Each fixture's band is set between its span's reading in one part and
//! its reading in the parts its turn needs, so a schedule that reads one
//! part everywhere leaves the first reading short of the band.

use super::*;

/// The clamped cubic B-spline on `controls`, uniform knots.
fn spline(controls: &[Point3<f64>]) -> Curve3<f64> {
    let spans = controls.len() - 3;
    let inner = (1..spans).map(|i| {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f64 / spans as f64;
        t
    });
    let knots: Vec<f64> = [0.0; 4].into_iter().chain(inner).chain([1.0; 4]).collect();
    let knots = KnotVector::clamped(knots, 3).unwrap();
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(knots, controls.to_vec(), vec![1.0; controls.len()]).unwrap(),
    ))
}

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **A long span**: one cubic span a quarter turn round the unit
/// cylinder, `2.7·10⁻⁴` outside it at most.
fn quarter() -> Curve3<f64> {
    let k = 0.552_284_749_830_793_4;
    spline(&[
        Point3::new(1.0, 0.0, 0.5),
        Point3::new(1.0, k, 0.5),
        Point3::new(k, 1.0, 0.5),
        Point3::new(0.0, 1.0, 0.5),
    ])
}

/// **A short span** `0.04` long, `off` from the unit sphere's axis
/// near its pole, its controls on the sphere.
fn by_pole(off: f64) -> Curve3<f64> {
    let s = 0.02;
    let at = |x: f64| Point3::new(x, off, (1.0 - x * x - off * off).sqrt());
    spline(&[at(-s), at(-s / 3.0), at(s / 3.0), at(s)])
}

/// **A chord of short spans far from the axis**: `spans` cubic spans a
/// quarter turn round the unit cylinder, its controls on it.
fn chord(spans: usize) -> Curve3<f64> {
    let controls: Vec<Point3<f64>> = (0..spans + 3)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let a = core::f64::consts::FRAC_PI_2 * i as f64 / (spans + 2) as f64;
            Point3::new(a.cos(), a.sin(), 0.5 * a)
        })
        .collect();
    spline(&controls)
}

/// The cone of half-angle 30° about `z`, its apex at the origin.
fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        half_angle: core::f64::consts::FRAC_PI_6,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **A short span on the cone**, `0.005` long round its axis at height
/// `h`, its controls on the cone.
fn on_cone(h: f64) -> Curve3<f64> {
    let rho = h * core::f64::consts::FRAC_PI_6.tan();
    let sweep = 0.005 / rho;
    let at = |k: f64| {
        let a = sweep * k / 3.0;
        Point3::new(rho * a.cos(), rho * a.sin(), h)
    };
    spline(&[at(0.0), at(1.0), at(2.0), at(3.0)])
}

/// The ring torus `R = 2`, `r = 0.5` about `z`.
fn torus() -> Surface<f64> {
    Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **A span along the torus's outer equator** through `sweep` radians,
/// its controls on it.
fn on_torus(sweep: f64) -> Curve3<f64> {
    let at = |k: f64| {
        let a = sweep * k / 3.0;
        Point3::new(2.5 * a.cos(), 2.5 * a.sin(), 0.0)
    };
    spline(&[at(0.0), at(1.0), at(2.0), at(3.0)])
}

/// The twin net limb 2 reads `carrier` through on `surface`, and the
/// twin's canonical form.
fn twin_net(
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
) -> (
    NurbsCurve3<geom_core::Interval>,
    Surface<f64>,
    CanonicalSurface,
) {
    let twin = orthonormal_chart(surface);
    let (o, e) = projected::chart_frame(&twin).unwrap();
    let Curve3::Nurbs(n) = carrier else {
        panic!("the rows' carriers are nets")
    };
    let net = crate::carrier_net::framed_carrier_net(
        carrier,
        n.domain(),
        &[],
        &crate::carrier_net::Frame::of(o, e),
    )
    .unwrap();
    let at = geom_core::Interval::from_f64;
    let form = match *surface {
        Surface::Cylinder { radius, .. } => CanonicalSurface::Cylinder { radius: at(radius) },
        Surface::Sphere { radius, .. } => CanonicalSurface::Sphere { radius: at(radius) },
        Surface::Plane { .. } => CanonicalSurface::Plane,
        Surface::Cone { half_angle, .. } => CanonicalSurface::Cone {
            cos2: at(half_angle.cos()).powi(2),
            sin2: at(half_angle.sin()).powi(2),
        },
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => CanonicalSurface::Torus {
            major: at(major_radius),
            minor: at(minor_radius),
        },
        _ => panic!("the rows' charts are analytic"),
    };
    (net.curve().unwrap(), twin, form)
}

/// Per knot span of `carrier`'s twin on `surface`, the parts
/// [`Parts::ByTurn`] reads it in.
fn by_turn(carrier: &Curve3<f64>, surface: &Surface<f64>) -> Vec<usize> {
    let (net, _, form) = twin_net(carrier, surface);
    let net = net.map_scalar(geom_core::Interval::from_certified::<geom_core::Interval>);
    let Curve3::Nurbs(n) = carrier else {
        panic!("the rows' carriers are nets")
    };
    let runs: Vec<f64> = n.knot_values().runs().map(|(k, _)| k).collect();
    runs.windows(2)
        .map(|w| Parts::ByTurn.of(&net, &form, (w[0], w[1])))
        .collect()
}

/// The widest part's metres, `carrier`'s twin on `surface` read in
/// `parts`.
fn widest(carrier: &Curve3<f64>, surface: &Surface<f64>, parts: Parts) -> f64 {
    let (net, twin, _) = twin_net(carrier, surface);
    let nappe = matches!(surface, Surface::Cone { .. }).then_some(crate::offset::Nappe::Opening);
    twin_span_hulls(&net, &twin, nappe, parts, &[])
        .unwrap()
        .iter()
        .map(|s| s.meters)
        .fold(0.0, crate::carrier_net::widest)
}

/// Limb 2 on the whole of `carrier` against `surface`, read with its
/// memo emptied first: the reading, and how many rounds it composed
/// (the first reading is one).
fn limb2(
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
    band: Band,
) -> (projected::OffsetRead<f64>, u64) {
    limb2_memo::clear();
    let (_, before) = limb2_memo::counts();
    let read = limb2_offset_sup(carrier, (0.0, 1.0), surface, band).unwrap();
    (read, limb2_memo::counts().1 - before)
}

/// **A span reads the parts its own turn needs.** A quarter turn round
/// a cylinder in one span, and a short span `0.005` from a sphere's
/// axis by its pole, each read in [`projected::INCIDENCE_CUTS`] parts;
/// the same short span `0.3` from the axis, a short span far from a
/// cylinder's axis, and any span on a plane, which has none, in one.
/// Red under a schedule that reads every span alike, in one part or in
/// eight.
#[test]
fn a_span_reads_the_parts_its_own_turn_needs() {
    let most = projected::INCIDENCE_CUTS;
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.5),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    for (what, got, want) in [
        ("a long span", by_turn(&quarter(), &cylinder()), vec![most]),
        (
            "a span by the pole",
            by_turn(&by_pole(0.005), &sphere()),
            vec![most],
        ),
        (
            "a short span far from the pole",
            by_turn(&by_pole(0.3), &sphere()),
            vec![1],
        ),
        (
            "a long span on the plane",
            by_turn(&quarter(), &plane),
            vec![1],
        ),
        (
            "a chord's short spans",
            by_turn(&chord(40), &cylinder()),
            vec![1; 40],
        ),
        (
            "a short span far from the cone's apex",
            by_turn(&on_cone(1.0), &cone()),
            vec![1],
        ),
        (
            "a short span far along the torus",
            by_turn(&on_torus(0.01), &torus()),
            vec![1],
        ),
        (
            "a long span round the torus",
            by_turn(&on_torus(core::f64::consts::FRAC_PI_2), &torus()),
            vec![most],
        ),
    ] {
        assert_eq!(got, want, "{what}");
    }
    let by_apex = by_turn(&on_cone(0.01), &cone());
    assert!(
        by_apex[0] > 1,
        "the same length by the cone's apex reads more parts: {by_apex:?}"
    );
}

/// **A long span, and a short one by the pole, are decided at limb 2's
/// first reading, as on a fixed eight parts a span.** Each one's band
/// lies between its widest part read in the parts its turn needs and
/// read in one part: the first reading clears, so it alone is composed
/// (no round follows), and its bound is the eight parts' reading. Red
/// under a schedule of one part a span, whose first reading does not
/// clear and hands the rounds the limb.
#[test]
fn a_long_span_and_one_by_the_pole_decide_at_the_first_reading() {
    for (what, carrier, surface) in [
        ("a long span", quarter(), cylinder()),
        ("a span by the pole", by_pole(0.005), sphere()),
    ] {
        let (one, eight) = (
            widest(&carrier, &surface, Parts::Each(1)),
            widest(&carrier, &surface, Parts::Each(projected::INCIDENCE_CUTS)),
        );
        assert!(
            eight < 0.9 * one,
            "{what}: one part ({one:e}) reads wider than eight ({eight:e})"
        );
        let zero = (one * eight).sqrt();
        let band = Band::new(zero, 10.0 * zero).unwrap();
        let (read, composed) = limb2(&carrier, &surface, band);
        assert_eq!(composed, 1, "{what}: decided at the first reading");
        let projected::Offset::Bound(bound) = read.offset else {
            panic!("{what}: clears")
        };
        assert!(
            bound <= zero && read.upper == bound,
            "{what}: the bound {bound:e} clears {zero:e}"
        );
        assert!(
            (bound - eight).abs() <= 1e-9 * eight,
            "{what}: the bound {bound:e} is the eight parts' reading {eight:e}"
        );
    }
}

/// **A short span far from the axis on a cone and on a torus is decided
/// at limb 2's first reading**, in one part, its metre conversion the
/// cone's lever and the torus's.
#[test]
fn a_short_span_on_a_cone_or_a_torus_decides_at_the_first_reading() {
    for (what, c, s) in [
        ("cone", on_cone(1.0), cone()),
        ("torus", on_torus(0.01), torus()),
    ] {
        let one = widest(&c, &s, Parts::ByTurn);
        assert!(one.is_finite() && one > 0.0, "{what}: reads {one:e}");
        let band = Band::new(4.0 * one, 40.0 * one).unwrap();
        let (read, composed) = limb2(&c, &s, band);
        assert_eq!(composed, 1, "{what}: decided at the first reading");
        assert!(
            matches!(read.offset, projected::Offset::Bound(b) if b <= 4.0 * one),
            "{what}: clears"
        );
    }
}

/// **A chord of short spans far from the axis reads one part a span**,
/// and is decided at limb 2's first reading: forty spans, forty parts.
/// Red under a fixed count of parts a span.
#[test]
fn a_chord_of_short_spans_reads_one_part_a_span() {
    let (c, s) = (chord(40), cylinder());
    let (net, twin, _) = twin_net(&c, &s);
    let parts = twin_span_hulls(&net, &twin, None, Parts::ByTurn, &[]).unwrap();
    assert_eq!(parts.len(), 40, "one part a span");
    let one = widest(&c, &s, Parts::ByTurn);
    let band = Band::new(4.0 * one, 40.0 * one).unwrap();
    let (read, composed) = limb2(&c, &s, band);
    assert_eq!(composed, 1, "decided at the first reading");
    assert!(
        matches!(read.offset, projected::Offset::Bound(b) if b <= 4.0 * one),
        "clears"
    );
}

/// **A first reading that does not clear hands the limb to the rounds.**
/// The long span against a band under its own eight parts' reading: the
/// first reading is composed and the rounds after it (more than one
/// read in all), and the limb does not clear. Red under a first reading
/// taken as the limb's wherever it decides.
#[test]
fn a_first_reading_that_does_not_clear_is_the_rounds() {
    let (c, s) = (quarter(), cylinder());
    let eight = widest(&c, &s, Parts::Each(projected::INCIDENCE_CUTS));
    let band = Band::new(0.25 * eight, 2.5 * eight).unwrap();
    let (read, composed) = limb2(&c, &s, band);
    assert!(composed > 1, "the rounds read after the first reading");
    let cleared = matches!(read.offset, projected::Offset::Bound(b) if b <= band.zero());
    assert!(
        !cleared,
        "a span off the chart by more than the band does not clear"
    );
}

/// **The rounds after a first reading that does not clear start at
/// eight parts a span**, whatever the first reading's count: a chord of
/// short spans far from the axis (one part each at the first reading)
/// against a band under its own eight parts' reading. The rounds' first
/// composite, kept in limb 2's memo under the rounds' key, holds eight
/// parts a span. Red under rounds that start at the first reading's
/// schedule.
#[test]
fn the_rounds_start_at_eight_parts_a_span() {
    let (c, s) = (chord(40), cylinder());
    let eight = widest(&c, &s, Parts::Each(projected::INCIDENCE_CUTS));
    let band = Band::new(0.25 * eight, 2.5 * eight).unwrap();
    let (_, composed) = limb2(&c, &s, band);
    assert!(composed > 1, "the rounds read after the first reading");
    let key =
        limb2_memo::Key::of(&c, (0.0, 1.0), &s, band).with(Parts::Each(projected::INCIDENCE_CUTS));
    let computed = core::cell::Cell::new(false);
    let round = limb2_memo::read(&key, &[], || {
        computed.set(true);
        Err(PcurveCertifyError::UnsupportedChart { chart: s.kind() })
    });
    assert!(
        !computed.get(),
        "the rounds' first composite is in the memo"
    );
    let (hull, _) = round.unwrap();
    assert_eq!(
        hull.spans.len(),
        40 * projected::INCIDENCE_CUTS,
        "eight parts a span"
    );
}

/// **A short span that bulges off the unit cylinder by about `delta`
/// near its first third**: the cubic arc on a tenth of a turn (within
/// `10⁻⁶` of the cylinder, its inner controls on the tangents at
/// `4/3·tan(θ/4)`) with its first inner control pushed `delta` further
/// out, so its distance from the chart peaks inside a part, at no
/// dyadic break of the span.
fn bulge(delta: f64) -> Curve3<f64> {
    let arc = core::f64::consts::TAU / 10.0;
    let k = 4.0 / 3.0 * (arc / 4.0).tan();
    let on = |a: f64| Point3::new(a.cos(), a.sin(), 0.0);
    let handle = |a: f64, sense: f64, push: f64| {
        let r = (1.0 + k * k).sqrt() + push;
        let turn = a + sense * k.atan();
        Point3::new(r * turn.cos(), r * turn.sin(), 0.0)
    };
    spline(&[
        on(0.0),
        handle(0.0, 1.0, delta),
        handle(arc, -1.0, 0.0),
        on(arc),
    ])
}

/// The densely sampled distance of `carrier` from the unit cylinder
/// about `z`.
fn off_cylinder(carrier: &Curve3<f64>) -> f64 {
    (0..=1000)
        .map(|i| {
            let p = carrier.eval(f64::from(i) / 1000.0);
            (p.x.hypot(p.y) - 1.0).abs()
        })
        .fold(0.0, f64::max)
}

/// **Check 4 alone** on `carrier`'s projected row on `surface`, over
/// the carrier's whole domain at `Interval` (whose check 3 takes no
/// samples), read with the span memo emptied first: the certificate's
/// envelope (its upper end), the image, and how many composites check 4
/// computed.
fn check_4(
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
    band: Band,
) -> (
    Result<f64, PcurveCertifyError>,
    ProjectedImage<geom_core::Interval>,
    u64,
) {
    use geom_core::Bounds;
    let at = geom_core::Interval::from_f64;
    let (c, s) = (carrier.map_scalar(at), surface.map_scalar(at));
    let image = projected::project(&c, None, &s, band).unwrap();
    span_memo::clear();
    let (_, before) = span_memo::counts();
    let read = projected::run_projected_checks(
        &Pcurve::Projected(Box::new(image.clone())),
        at(0.0),
        at(1.0),
        &c,
        &s,
        band,
        Some(crate::FittedLane::certified()),
    )
    .map(|cert| cert.limbs().1.hi());
    (read, image, span_memo::counts().1 - before)
}

/// **Check 4 reads a 625-span chord in one part a span.** A chord of
/// short spans far from the cylinder's axis, against a band its first
/// reading clears: check 4 composes once, and that composite is the one
/// each span in the parts its turn needs reads, 625 parts where eight a
/// span read 5,000. Red under a check 4 that reads its spans in a fixed
/// eight parts.
#[test]
fn check_4_reads_a_625_span_chord_in_one_part_a_span() {
    use geom_core::Interval;
    let (c, s) = (chord(625), cylinder());
    let band = Band::new(1e-4, 1e-3).unwrap();
    let (read, image, composed) = check_4(&c, &s, band);
    let envelope = read.unwrap_or_else(|e| panic!("the chord certifies: {e:?}"));
    assert!(envelope <= 1e-4, "the envelope {envelope:e} clears");
    assert_eq!(composed, 1, "check 4 composes once");
    let at = Interval::from_f64;
    let twin = orthonormal_chart(&s.map_scalar(at));
    let (o, e) = projected::chart_frame(&twin).unwrap();
    let Curve3::Nurbs(n) = c.map_scalar(at) else {
        panic!("the chord is a net")
    };
    let twin_net = projected::framed_net(&n, o, &e);
    let (_, misses) = span_memo::counts();
    let hull = projected_hull_lane(&image, &twin_net, &twin, Parts::ByTurn, &[]).unwrap();
    assert_eq!(
        span_memo::counts().1,
        misses,
        "check 4's one composite is the one read in the parts each span's turn needs"
    );
    assert_eq!(hull.spans.len(), 625, "one part a span");
}

/// Check 4's envelope of `carrier`'s projected row on `surface`, over
/// its whole domain at `Interval`, its spans read in `parts`: the terms'
/// total with the placement pair, its upper end.
fn envelope_in(carrier: &Curve3<f64>, surface: &Surface<f64>, band: Band, parts: Parts) -> f64 {
    use geom_core::Bounds;
    let at = geom_core::Interval::from_f64;
    let (c, s) = (carrier.map_scalar(at), surface.map_scalar(at));
    let image = projected::project(&c, None, &s, band).unwrap();
    let (t0, t1) = (at(0.0), at(1.0));
    let (mut terms, name) = projected::projected_envelope(
        &image,
        t0,
        t1,
        &image.chart_box(t0, t1),
        &c,
        &s,
        band,
        Some(crate::FittedLane::certified()),
        parts,
    )
    .unwrap();
    let pcurve = Pcurve::Projected(Box::new(image));
    let (fu, fv) = placement_terms(&pcurve, name, (t0, t1), &c, &s, band).unwrap();
    terms.add(EnvelopeTerm::FidelityU, fu);
    terms.add(EnvelopeTerm::FidelityV, fv);
    terms.total().hi()
}

/// **Check 4's first reading bounds the distance it certifies.** A
/// short span bulging off the cylinder inside its part, against a band
/// above its sampled distance: check 4 certifies at its first reading,
/// with an envelope over that distance. Red under a first reading that
/// under-reads a part (mutant `by-turn-sampled`, which reads a part off
/// its ends alone).
#[test]
fn check_4s_first_reading_bounds_what_it_certifies() {
    let (c, s) = (bulge(1e-4), cylinder());
    let off = off_cylinder(&c);
    assert!(
        (3e-5..1e-4).contains(&off),
        "the span bulges off the chart by about 0.44·10⁻⁴: {off:e}"
    );
    let (read, _, composed) = check_4(&c, &s, Band::new(1e-3, 1e-2).unwrap());
    let envelope = read.unwrap_or_else(|e| panic!("certifies: {e:?}"));
    assert_eq!(composed, 1, "decided at the first reading");
    assert!(
        envelope >= off,
        "the envelope {envelope:e} bounds the sampled distance {off:e}"
    );
}

/// **A first reading that does not pass is read again at eight parts a
/// span, and that reading decides.** The bulging span against a band
/// whose zero lies between its envelope read in the parts its turn
/// needs and read in eight parts a span: the first reading does not
/// pass, and check 4 certifies on a second composite, with the eight
/// parts' envelope. Red under a check 4 that takes its first reading's
/// refusal or escalation as its own.
#[test]
fn a_first_reading_that_does_not_pass_is_read_again_at_eight_parts() {
    let (c, s) = (bulge(1e-4), cylinder());
    let wide = Band::new(1e-3, 1e-2).unwrap();
    let (first, eight) = (
        envelope_in(&c, &s, wide, Parts::ByTurn),
        envelope_in(&c, &s, wide, Parts::Each(projected::INCIDENCE_CUTS)),
    );
    assert!(
        first > 1.05 * eight,
        "the first reading ({first:e}) reads wider than eight parts ({eight:e})"
    );
    let zero = (first * eight).sqrt();
    let (read, _, composed) = check_4(&c, &s, Band::new(zero, 10.0 * zero).unwrap());
    let envelope = read.unwrap_or_else(|e| panic!("certifies at eight parts a span: {e:?}"));
    assert_eq!(composed, 2, "read again after the first reading");
    assert!(
        envelope <= zero && (envelope - eight).abs() <= 1e-9 * eight,
        "the envelope {envelope:e} is the eight parts' {eight:e}, under {zero:e}"
    );
}
