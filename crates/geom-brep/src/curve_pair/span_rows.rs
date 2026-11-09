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
    Curve3::line(Point3::new(1.0, 2.0, 3.0), Vec3::new(0.6, 0.0, 0.8))
}

fn circle() -> Curve3<f64> {
    Curve3::circle(
        Point3::new(0.0, 0.0, 1.0),
        Vec3::unit_z(),
        2.0,
        Vec3::unit_x(),
    )
}

fn ellipse() -> Curve3<f64> {
    Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(Point3::new(0.0, 0.0, 0.0), Vec3::unit_z(), Vec3::unit_x()),
        3.0,
        0.5,
    ))
}

/// What a contact reads as, for the table: its variant's name.
fn name(got: Result<Option<Hit<f64>>, super::SpanRefusal>) -> &'static str {
    match got {
        Ok(None) => "none",
        Ok(Some(Hit::On { .. })) => "on",
        Ok(Some(Hit::End(_))) => "end",
        Err(_) => "escalated",
    }
}

/// What a place reads as: its variant's name.
fn place(got: Result<Place, geom::CarrierRefusal>) -> &'static str {
    match got {
        Ok(Place::Inside) => "inside",
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
/// [`geom::Curve3::foot`] finds is the seed itself: the point's direction
/// in the circle's plane is exactly zero, and [`geom::Azimuth::near`]
/// reads a zero direction as its reference. So a seed past the span puts
/// the foot past it, where a place read would decide a cut from an exact
/// zero, and a move off the axis would flip it (boss_union's span
/// branch, where a disc's three-arc rim puts the centroid of three of
/// its vertices on the axis). The carrier reads the point off, and that
/// is the answer at every seed, the middle included, with the carrier's
/// reading recorded and no `curve_pair_span_*` row decided. Red where
/// the place is read first, and where the carrier's reading is dropped.
#[cfg(feature = "probe")]
#[test]
fn a_point_on_a_circles_axis_is_off_with_no_place_read() {
    use geom_core::Real;
    use geom_core::k_stats::{self, Probe};
    let p = Probe::from_f64;
    let c = Curve3::circle(
        Point3::new(p(0.0), p(0.0), p(1.0)),
        Vec3::new(p(0.0), p(0.0), p(1.0)),
        p(2.0),
        Vec3::new(p(1.0), p(0.0), p(0.0)),
    );
    let centre = Point3::new(p(0.0), p(0.0), p(1.0));
    let foot = c.foot(centre, p(4.0 * PI / 3.0)).nearest().unwrap().0;
    assert_eq!(foot, 4.0 * PI / 3.0, "an axis point's foot is its seed");
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

/// **On an arc, a refused member's window is its stretch of the arc**:
/// half of the circle and of the ellipse, each against a point standing
/// half an escalation off the carrier, out from its centre, beside a
/// parameter `at` of the span. `at`, whose point lies within the
/// escalation of the member, lies in the window (red where the piece it
/// is on is cleared), and the window is a few escalations of arc about it
/// (red where the walk keeps a piece it could clear, or falls back to the
/// whole span). Each is read again with a second point ten metres off,
/// so the member's ball is five metres across about a centre far from the
/// arc: `at` still lies in the window (red where the ball's radius is
/// left out of the reach, which clears every piece near `at`).
#[test]
fn a_refused_members_window_on_an_arc_is_its_stretch() {
    let esc = band().escalate();
    for (kind, carrier, centre) in [
        ("circle", circle(), Point3::new(0.0, 0.0, 1.0)),
        ("ellipse", ellipse(), Point3::origin()),
    ] {
        let span = Span::new(&carrier, (0.0, PI));
        let slowest = span.speeds().0.get();
        for at in [0.4, 1.3, 2.9] {
            let p = carrier.eval(at);
            let q = p + (p - centre).normalize() * (0.5 * esc);
            let point = geom::cover::Cover::points(vec![q]).unwrap();
            let (w0, w1) = span.window_near(Some(&point), band());
            let widest = 8.0 * esc / slowest;
            assert!(
                w1 - w0 <= widest,
                "{kind} at {at}: the window ({w0}, {w1}) is wider than {widest}"
            );
            assert!(
                w0 <= at && at <= w1,
                "{kind} at {at}: the point half an escalation from the member lies outside \
                 ({w0}, {w1})"
            );
            let far = q + Vec3::new(0.0, 0.0, 10.0);
            let pair = geom::cover::Cover::points(vec![q, far]).unwrap();
            let (w0, w1) = span.window_near(Some(&pair), band());
            assert!(
                w0 <= at && at <= w1,
                "{kind} at {at}: with a far second point, the point half an escalation from \
                 the member lies outside ({w0}, {w1})"
            );
        }
    }
}

/// **A refusal says which reading it rests on** ([`super::SpanReading`]):
/// on the circle's span, a point half the band off the carrier at the
/// span's middle refuses on the carrier, the place reading it inside;
/// a point on the carrier half the band past the span's start, its chord
/// to the start in band, refuses on the place. Red where the two are
/// swapped, which reads a wedge in band as a graze at the solid door's
/// curved edges, or the reverse.
#[test]
fn a_refusal_names_the_reading_it_rests_on() {
    use super::{SpanReading, SpanRefusal};
    let half = 0.5 * (band().zero() + band().escalate());
    let carrier = circle();
    let span = Span::new(&carrier, (0.5, 2.5));
    let centre = Point3::new(0.0, 0.0, 1.0);
    let mid = carrier.eval(1.5);
    let off = mid + (mid - centre).normalize() * half;
    assert!(
        matches!(
            span.read(off, 1.5, band()),
            Err(SpanRefusal {
                of: SpanReading::Carrier,
                ..
            })
        ),
        "off the carrier in band, inside the span"
    );
    let past = carrier.eval(0.5 - half / 2.0);
    assert!(
        matches!(
            span.read(past, 0.5, band()),
            Err(SpanRefusal {
                of: SpanReading::Place,
                ..
            })
        ),
        "on the carrier, in band of the start: {:?}",
        span.read(past, 0.5, band())
    );
}

/// **A point-on-span refusal stands as its carrier refusal does**, through
/// the meeting's fault: an escalation (either kind) ranked by its margin,
/// a walk past its budget unsettled, and a spline with no speed bound a
/// limit. Each groups as unread or escalated ([`super::MeetFault::kind`]),
/// and a boundary pass reads the same standing. Red where a walk refusal
/// is ranked as anything else at any of the three.
#[test]
fn a_point_on_span_refusal_stands_as_its_carrier_refusal() {
    use super::{MeetFault, MeetKind};
    use crate::curve_face::BoundaryRefusal;
    use crate::ladder::Refusal;
    use geom::{CarrierEscalation, CarrierRefusal, PieceWalkRefusal};
    let diag = geom_core::k_stats::decide(
        "curve_carrier_gap",
        geom_core::Margin::of(0.5 * (band().zero() + band().escalate())),
        band(),
    )
    .unwrap_err();
    for (refused, standing) in [
        (
            CarrierRefusal::Escalated(CarrierEscalation::InBand(diag)),
            Refusal::Margin(diag),
        ),
        (
            CarrierRefusal::Escalated(CarrierEscalation::Straddle(diag)),
            Refusal::Margin(diag),
        ),
        (
            CarrierRefusal::Unwalked(PieceWalkRefusal::Exhausted),
            Refusal::Unsettled,
        ),
        (
            CarrierRefusal::Unwalked(PieceWalkRefusal::Unbounded),
            Refusal::Limit,
        ),
    ] {
        let fault = MeetFault::from(refused);
        assert_eq!(
            fault.refusal(),
            Ok(standing),
            "{refused:?}: the meeting's standing"
        );
        assert_eq!(
            BoundaryRefusal::of(fault).refusal(),
            Ok(standing),
            "{refused:?}: the boundary pass's standing"
        );
        let grouped = matches!(
            (refused, fault.kind()),
            (CarrierRefusal::Escalated(_), MeetKind::Escalated(_))
                | (CarrierRefusal::Unwalked(_), MeetKind::Unread(_))
        );
        assert!(grouped, "{refused:?}: grouped as {:?}", fault.kind());
    }
}

/// **A refused member's window is the whole span where the walk cannot
/// place it**, never an empty one: the boundary pass reads a member only
/// inside its window, so an empty window drops its meetings.
/// - On a rational spline whose middle weight is a million times its
///   ends', against a point a metre off it: its speed bound, set by the
///   fast ends, is so far above its speed through the slow middle that
///   the pieces there clear only at a width the budget does not reach,
///   and the walk runs past its budget.
/// - On a spline whose control points coincide: its greatest speed is
///   zero, so its pieces have no reach and the walk does not run.
///
/// Red where either fallback gives anything but the whole span.
#[test]
fn a_window_the_walk_cannot_place_is_the_whole_span() {
    let spline = |pts: [Point3<f64>; 3], weights: [f64; 3]| {
        Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2)
                    .unwrap(),
                pts.to_vec(),
                weights.to_vec(),
            )
            .unwrap(),
        ))
    };
    let greatest = |c: &Curve3<f64>| geom::carrier_speed::speed_range(c).1.get();
    let loose = spline(
        [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
        ],
        [1.0, 1e6, 1.0],
    );
    assert!(
        greatest(&loose) > 1e6,
        "the loose spline's speed bound: {}",
        greatest(&loose)
    );
    let span = Span::new(&loose, (0.0, 1.0));
    let member = geom::cover::Cover::points(vec![Point3::new(1.0, 1.0, 1.0)]).unwrap();
    assert_eq!(
        span.window_near(Some(&member), band()),
        (0.0, 1.0),
        "a walk past its budget"
    );
    let at = Point3::new(1.0, 2.0, 3.0);
    let still = spline([at; 3], [1.0; 3]);
    assert!(
        greatest(&still) == 0.0,
        "the still spline's speed bound: {}",
        greatest(&still)
    );
    let span = Span::new(&still, (0.0, 1.0));
    let member = geom::cover::Cover::points(vec![at]).unwrap();
    assert_eq!(
        span.window_near(Some(&member), band()),
        (0.0, 1.0),
        "a spline with no speed bound"
    );
}

/// The cubic hairpin of `geom`'s on-carrier rows, its branches `h`
/// apart, and an `Interval` box of `±r` about its point at `t = 0.08`.
fn cubic_hairpin(h: f64) -> Curve3<f64> {
    Curve3::Nurbs(std::sync::Arc::new(
        geom::NurbsCurve3::new(
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3)
                .unwrap(),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.4, 0.0, 0.0),
                Point3::new(1.4, h, 0.0),
                Point3::new(0.0, h, 0.0),
            ],
            vec![1.0; 4],
        )
        .unwrap(),
    ))
}

fn boxed(p: Point3<f64>, r: f64) -> Point3<geom_core::Interval> {
    let w = |x: f64| geom_core::Interval::from_bounds(x - r, x + r);
    Point3::new(w(p.x), w(p.y), w(p.z))
}

/// **A span over a hairpin closer than the band reads `On` at the
/// seed's branch at `Interval`**. The whole span of a hairpin whose
/// branches lie 3e-10 apart, inside the 1e-9 zero, read the way an edge's
/// span is read: at a ±1e-10 box about a point of the near branch
/// (`t = 0.08`) seeded on the far branch, it reads `On` at `t ≈ 0.92`,
/// where the tangent is reversed, with nothing logged; seeded at the
/// span's middle (the census's seed) it escalates; at `f64` it reads the
/// near branch from either seed.
///
/// The carrier is not one an edge can hold: its tip speed is `1.5·h`, so
/// `EdgeCurve::certify` refuses it `SpanMeterCollapsed`. The row pins the
/// reader on a bare carrier. A fix that gates edges at the certificate
/// leaves it green, and one that certifies the branch at the reader turns
/// it red (`a-carriers-branches-within-the-band-of-each-other`).
#[test]
fn a_span_over_a_hairpin_closer_than_the_band_reads_on_at_the_seeds_branch() {
    use super::Reading;
    use geom_core::{Bounds, Interval, Real};
    let c = cubic_hairpin(3e-10);
    let p = c.eval(0.08);
    for seed in [0.5, 0.92] {
        let read = Span::new(&c, (0.0, 1.0)).read(p, seed, band());
        assert!(
            matches!(read, Ok(Reading::On { t, .. }) if (t - 0.08).abs() < 1e-6),
            "f64, seed {seed}: on at the near branch: {read:?}"
        );
    }
    let ci: Curve3<Interval> = c.map_scalar(Interval::from_f64);
    let span = Span::new(&ci, (Interval::from_f64(0.0), Interval::from_f64(1.0)));
    let far = span.read(boxed(p, 1e-10), Interval::from_f64(0.92), band());
    assert!(
        matches!(&far, Ok(Reading::On { t, .. }) if t.lo() > 0.5 && ci.deriv(*t).x.hi() < 0.0),
        "Interval, far seed: on at the far branch, its tangent reversed: {far:?}"
    );
    let mid = span.read(boxed(p, 1e-10), Interval::from_f64(0.5), band());
    assert!(
        matches!(
            mid,
            Err(super::SpanRefusal {
                refused: geom::CarrierRefusal::Escalated(_),
                ..
            })
        ),
        "Interval, the span's middle: escalates: {mid:?}"
    );
}
