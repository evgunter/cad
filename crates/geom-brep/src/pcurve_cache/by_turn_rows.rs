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
    let radius = geom_core::Interval::from_f64(1.0);
    let form = match surface {
        Surface::Cylinder { .. } => CanonicalSurface::Cylinder { radius },
        Surface::Sphere { .. } => CanonicalSurface::Sphere { radius },
        Surface::Plane { .. } => CanonicalSurface::Plane,
        _ => panic!("the rows' charts are a plane, a cylinder and a sphere"),
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
    twin_span_hulls(&net, &twin, None, parts, &[])
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
    ] {
        assert_eq!(got, want, "{what}");
    }
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
