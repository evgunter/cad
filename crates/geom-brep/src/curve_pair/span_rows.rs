//! [`Span::contact`], and [`Span::place`] under it, on each of their
//! answers and refusals, on a line, a circle and an ellipse: an enumeration of poses, each placed a
//! stated number of zero bands from the place it tests.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom_core::{Band, Point3, Vec3};

use super::{Hit, Place, Span, SpanEnd};

const ZERO: f64 = 1e-9;

fn band() -> Band {
    Band::new(ZERO, 10.0 * ZERO).unwrap()
}

fn line() -> Curve3<f64> {
    Curve3::Line {
        origin: Point3::new(1.0, 2.0, 3.0),
        dir: Vec3::new(0.6, 0.0, 0.8),
    }
}

fn circle() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::unit_z(),
        radius: 2.0,
        u_ref: Vec3::unit_x(),
    }
}

fn ellipse() -> Curve3<f64> {
    Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        semi_u: 3.0,
        semi_v: 0.5,
        u_ref: Vec3::unit_x(),
    }
}

/// What a contact reads as, for the table: its variant's name.
fn name(got: Result<Option<Hit<f64>>, geom::CarrierRefusal>) -> &'static str {
    match got {
        Ok(None) => "none",
        Ok(Some(Hit::On { .. })) => "on",
        Ok(Some(Hit::End(_))) => "end",
        Err(_) => "escalated",
    }
}

/// What a place reads as: its variant's name.
fn place(got: Result<Place<f64>, geom::CarrierRefusal>) -> &'static str {
    match got {
        Ok(Place::Inside { .. }) => "inside",
        Ok(Place::End(_)) => "end",
        Ok(Place::Past) => "past",
        Err(_) => "escalated",
    }
}

/// `q` read on `span` at its foot, seeded at `t`.
fn read(span: &Span<'_, f64>, q: Point3<f64>, t: f64) -> &'static str {
    name(span.contact(q, t, band()))
}

/// **Each answer, on each kind**, with the place under it. A point is
/// read at its foot seeded at its own parameter, moved along the carrier
/// (`along`, metres, past the span's end at `t1` when positive) and off
/// it (`off`, metres, along a direction square to the carrier there).
/// Past the span is no contact whether or not the carrier reads the point
/// on it: thirty zeros past and in the band's gap off it is none, the
/// carrier in band and the place past (the 5154 case).
#[test]
fn a_point_reads_on_at_an_end_or_none() {
    let b = band();
    let curves = [
        ("line", line()),
        ("circle", circle()),
        ("ellipse", ellipse()),
    ];
    for (kind, carrier) in &curves {
        let (t0, t1) = (0.2, 1.4);
        let span = Span::new(carrier, (t0, t1));
        // `t1 + s` for an arc length `s` past the end, read to first
        // order at the end's own speed.
        let speed = geom::carrier_speed::speed_at(carrier, t1).unwrap();
        let at = |along: f64, off: f64| {
            let t = t1 + along / speed;
            let (p, d) = carrier.ders1(t);
            let normal = d.cross(Vec3::new(0.3, -0.5, 0.8)).normalize();
            (p + normal * off, t)
        };
        for (label, along, off, want, placed) in [
            ("inside, on the carrier", -0.3, 0.0, "on", "inside"),
            ("inside, half a zero off", -0.3, 0.5 * ZERO, "on", "inside"),
            (
                "inside, off the carrier",
                -0.3,
                30.0 * ZERO,
                "none",
                "inside",
            ),
            (
                "inside, in the gap off it",
                -0.3,
                3.0 * ZERO,
                "escalated",
                "inside",
            ),
            ("at the end", 0.0, 0.0, "end", "end"),
            ("half a zero inside the end", -0.5 * ZERO, 0.0, "end", "end"),
            ("half a zero past the end", 0.5 * ZERO, 0.0, "end", "end"),
            (
                "in the gap past the end",
                3.0 * ZERO,
                0.0,
                "escalated",
                "escalated",
            ),
            (
                "in the gap inside the end",
                -3.0 * ZERO,
                0.0,
                "escalated",
                "escalated",
            ),
            (
                "thirty zeros inside the end",
                -30.0 * ZERO,
                0.0,
                "on",
                "inside",
            ),
            (
                "thirty zeros past the end",
                30.0 * ZERO,
                0.0,
                "none",
                "past",
            ),
            ("well past the end", 0.2, 0.0, "none", "past"),
            (
                "well past the end, off it",
                0.2,
                30.0 * ZERO,
                "none",
                "past",
            ),
            (
                "well past the end, in the gap off it",
                0.2,
                3.0 * ZERO,
                "none",
                "past",
            ),
            (
                "thirty zeros past, in the gap off it",
                30.0 * ZERO,
                3.0 * ZERO,
                "none",
                "past",
            ),
            (
                "in the gap past the end and off it",
                3.0 * ZERO,
                3.0 * ZERO,
                "escalated",
                "escalated",
            ),
        ] {
            let (q, t) = at(along, off);
            assert_eq!(name(span.contact(q, t, b)), want, "{kind}, {label}");
            assert_eq!(place(span.place(q, t, b)), placed, "{kind}, {label}: place");
        }
    }
}

/// **A closed carrier is read on the branch about the span's middle**: a
/// foot seeded a whole turn away reads where the point lies, and the
/// parameter an `On` reports is the span's.
#[test]
fn a_closed_carrier_reads_the_parameter_on_the_spans_branch() {
    let b = band();
    for carrier in [circle(), ellipse()] {
        let span = Span::new(&carrier, (-0.4, 0.6));
        let q = carrier.eval(0.1);
        for turns in [-2.0, -1.0, 1.0, 3.0] {
            let got = span.contact(q, 0.1 + turns * TAU, b).unwrap();
            let Some(Hit::On { t, .. }) = got else {
                panic!("{turns} turns away: {got:?}");
            };
            assert!((t - 0.1).abs() < 1e-12, "{turns} turns away: read at {t}");
        }
        // The antipode of the middle is past the span, whichever branch.
        let far = carrier.eval(0.1 + PI);
        assert_eq!(read(&span, far, 0.1 + PI), "none");
        assert_eq!(read(&span, far, 0.1 - PI), "none");
    }
}

/// **An endless span has no end to be near**: a closed edge's point at
/// its start reads on it, where the same span with ends reads `End`.
#[test]
fn an_endless_span_reads_its_end_point_on_it() {
    let carrier = circle();
    let span = Span::new(&carrier, (0.0, TAU));
    let q = carrier.eval(0.0);
    assert_eq!(read(&span, q, 0.0), "end");
    assert_eq!(read(&span.endless(), q, 0.0), "on");
    let off = q + Vec3::unit_z() * (30.0 * ZERO);
    assert_eq!(read(&span.endless(), off, 0.0), "none");
}

/// **An end is the end's point, on its raw chord, whatever the seed**:
/// each end's point reads that end by name, its foot seeded at the
/// span's other end or half a turn away.
#[test]
fn an_end_reads_by_name_at_any_foot() {
    let b = band();
    for carrier in [line(), circle(), ellipse()] {
        let span = Span::new(&carrier, (0.2, 1.4));
        for (end, other, which) in [(0.2, 1.4, SpanEnd::Start), (1.4, 0.2, SpanEnd::Stop)] {
            let q = carrier.eval(end);
            for seed in [other, end + PI] {
                assert_eq!(
                    span.contact(q, seed, b),
                    Ok(Some(Hit::End(which))),
                    "{:?}",
                    carrier.kind()
                );
            }
        }
    }
}

/// **An end is the end's point**, on a slow arc too: where the
/// ellipse runs at its minor semi-axis, half a zero of arc past the end
/// reads at it and thirty zeros past it reads clear of it; one in the
/// band's gap off the carrier and five zeros past is not read past.
#[test]
fn an_end_is_read_by_its_chord_on_a_slow_arc() {
    let carrier = ellipse();
    // At `t = 0` the ellipse runs at its minor semi-axis, 0.5 m/rad.
    let span = Span::new(&carrier, (-PI / 2.0, 0.0));
    let t = 0.5 * ZERO / 0.5;
    assert_eq!(read(&span, carrier.eval(t), t), "end");
    let t = 30.0 * ZERO / 0.5;
    assert_eq!(read(&span, carrier.eval(t), t), "none");
    // Five zeros of arc past the end and three off the carrier: the
    // greatest speed meters the parameter thirty zeros past, but the
    // chord is in the band's gap, so whether it is past is not read. Red
    // where the place reads the side alone.
    let t = 5.0 * ZERO / 0.5;
    let q = carrier.eval(t) + Vec3::unit_z() * (3.0 * ZERO);
    assert_eq!(read(&span, q, t), "escalated");
}

/// A hairpin spline: out along `y = 0` over its first knot span, back
/// along `y = 1e-6` over forty more, the two branches a thousand zero
/// bands apart.
fn hairpin() -> Curve3<f64> {
    let mut points = vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)];
    for i in 0..=40 {
        points.push(Point3::new(1.0 - f64::from(i) / 40.0, 1e-6, 0.0));
    }
    let n = points.len();
    let last = f64::from(u32::try_from(n - 1).unwrap());
    let mut knots = vec![0.0, 0.0];
    knots.extend((1..n - 1).map(|i| f64::from(u32::try_from(i).unwrap()) / last));
    knots.extend([1.0, 1.0]);
    Curve3::Nurbs(std::sync::Arc::new(
        geom::NurbsCurve3::new(
            geom_core::spline::KnotVector::clamped(knots, 1).unwrap(),
            points,
            vec![1.0; n],
        )
        .unwrap(),
    ))
}

/// **Past stands only on a nearest foot or a certificate.** On a hairpin
/// spline the span over the outgoing branch, seeded at its middle, finds
/// some interior points' feet on the return branch, past the span. That
/// foot is no nearest point, so `Past` is not taken from it: the span's
/// own walk keeps a piece at the point, and every interior point reads
/// on the span or escalates, never off it. Red where `Past` is read from
/// any foot.
#[test]
fn a_foot_on_a_hairpin_spline_far_branch_is_no_past() {
    let c = hairpin();
    let out = 1.0 / 42.0;
    let span = Span::new(&c, (0.0, out));
    let mid = 0.5 * out;
    for i in 1..100 {
        let q = c.eval(out * f64::from(i) / 100.0);
        assert!(
            !matches!(span.contact(q, mid, band()), Ok(None)),
            "{q:?}: an interior point read off the span"
        );
    }
}

/// **A circle's foot is its nearest point, so its `Past` stands**: a
/// point on the unit circle read on the span `(0, 1)` with its seed a
/// half turn away is still read where it lies, never at another point's
/// foot. Red where a foot found for another point can be read.
#[test]
fn a_point_is_read_at_its_own_foot_whatever_the_seed() {
    let c = circle();
    let span = Span::new(&c, (0.0, 1.0));
    let q = c.eval(0.5);
    assert!(matches!(
        span.contact(q, 3.0, band()),
        Ok(Some(Hit::On { .. }))
    ));
    let far = c.eval(3.0);
    assert_eq!(read(&span, far, 0.5), "none");
}

/// **A place the band cannot read is off where the carrier reads the
/// point off.** A point thirty zeros off a circle beside its span's end,
/// its foot three zeros of arc inside the end: the chord to the end is
/// past the band, the side of the end is in the band's gap, and the
/// carrier reads the point off, so it is off the span. Red where an
/// undecided place escalates whatever the carrier says.
#[test]
fn an_undecided_place_beside_an_off_carrier_is_off() {
    let c = circle();
    let span = Span::new(&c, (0.2, 1.4));
    let speed = geom::carrier_speed::speed_at(&c, 1.4).unwrap();
    let t = 1.4 - 3.0 * ZERO / speed;
    let q = c.eval(t) + Vec3::unit_z() * (30.0 * ZERO);
    assert_eq!(place(span.place(q, t, band())), "escalated");
    assert_eq!(read(&span, q, t), "none");
}

/// **A point on a circle's axis is off its span with no place read.**
/// Every parameter of the carrier is its foot, and the one
/// [`geom::Curve3::foot`] finds is the seed turned by `atan2` of two
/// signed zeros: with the seed at a span's middle of 4π/3, where the
/// middle's cosine and sine are both negative, `atan2(+0, −0) = π` puts
/// the foot exactly antipodal to the middle. Read there, a half turn's
/// cut decides from an exact zero, and a move off the axis flips it
/// (boss_union's `curve_pair_span_branch`, where a disc's three-arc rim
/// puts the centroid of three of its vertices on the axis, and the arc's
/// own middle is the seed). The carrier reads the point off, and that is
/// the answer at every seed, the middle included, with the carrier's
/// reading recorded and no `curve_pair_span_*` row decided. Red where
/// the place is read first, and where the carrier's reading is dropped.
#[cfg(feature = "probe")]
#[test]
fn a_point_on_a_circles_axis_is_off_with_no_place_read() {
    use geom_core::Real;
    use geom_core::k_stats::{self, Probe};
    let p = Probe::from_f64;
    let c = Curve3::Circle {
        center: Point3::new(p(0.0), p(0.0), p(1.0)),
        axis: Vec3::new(p(0.0), p(0.0), p(1.0)),
        radius: p(2.0),
        u_ref: Vec3::new(p(1.0), p(0.0), p(0.0)),
    };
    let centre = Point3::new(p(0.0), p(0.0), p(1.0));
    let antipode = c.foot(centre, p(4.0 * PI / 3.0)).t().0;
    assert_eq!(
        antipode,
        7.0 * PI / 3.0,
        "the middle's foot is its antipode"
    );
    for (t0, t1) in [(2.0 * PI / 3.0, 4.0 * PI / 3.0), (PI, 5.0 * PI / 3.0)] {
        let span = Span::new(&c, (p(t0), p(t1)));
        let mid = 0.5 * (t0 + t1);
        for seed in [0.0, PI / 3.0, PI, 5.0 * PI / 3.0, TAU, mid] {
            k_stats::start_recording();
            let got = span.contact(centre, p(seed), band());
            let names: Vec<&str> = k_stats::take_samples()
                .iter()
                .map(|s| s.predicate)
                .collect();
            let placed: Vec<&&str> = names
                .iter()
                .filter(|n| n.starts_with("curve_pair_span_"))
                .collect();
            let label = format!("span ({t0}, {t1}), seed {seed}");
            assert!(matches!(got, Ok(None)), "{label}: {got:?}");
            assert!(placed.is_empty(), "{label}: the place was read: {placed:?}");
            assert!(
                names.contains(&"curve_carrier_gap"),
                "{label}: the carrier's reading is not recorded: {names:?}"
            );
        }
    }
}
