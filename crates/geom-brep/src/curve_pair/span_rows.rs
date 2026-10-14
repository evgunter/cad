//! [`Span::contact`] on each of its four answers and its refusal, on a
//! line, a circle and an ellipse: an enumeration of poses, each placed a
//! stated number of zero bands from the place it tests.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom_core::{Band, Point3, Vec3};

use super::{Contact, Span};

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
        major: 3.0,
        minor: 0.5,
        u_ref: Vec3::unit_x(),
    }
}

/// What a contact reads as, for the table: its variant's name.
fn name(got: Result<Contact<f64>, geom_core::Indeterminate>) -> &'static str {
    match got {
        Ok(Contact::Off) => "off",
        Ok(Contact::Carrier) => "carrier",
        Ok(Contact::On { .. }) => "on",
        Ok(Contact::End) => "end",
        Err(_) => "escalated",
    }
}

/// **Each answer, on each kind.** A point is read at its own parameter,
/// moved along the carrier (`along`, metres, past the span's end at `t1`
/// when positive) and off it (`off`, metres, along a direction square to
/// the carrier there).
#[test]
fn a_point_reads_off_on_at_an_end_or_past_the_span() {
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
        let speed = crate::carrier_speed::speed_at(carrier, t1).unwrap();
        let at = |along: f64, off: f64| {
            let t = t1 + along / speed;
            let (p, d) = carrier.ders1(t);
            let normal = d.cross(Vec3::new(0.3, -0.5, 0.8)).normalize();
            (p + normal * off, t)
        };
        for (label, along, off, want) in [
            ("inside, on the carrier", -0.3, 0.0, "on"),
            ("inside, half a zero off", -0.3, 0.5 * ZERO, "on"),
            ("inside, off the carrier", -0.3, 30.0 * ZERO, "off"),
            ("inside, in the gap off it", -0.3, 3.0 * ZERO, "escalated"),
            ("at the end", 0.0, 0.0, "end"),
            ("half a zero inside the end", -0.5 * ZERO, 0.0, "end"),
            ("half a zero past the end", 0.5 * ZERO, 0.0, "end"),
            ("in the gap past the end", 3.0 * ZERO, 0.0, "escalated"),
            ("in the gap inside the end", -3.0 * ZERO, 0.0, "escalated"),
            ("thirty zeros inside the end", -30.0 * ZERO, 0.0, "on"),
            ("thirty zeros past the end", 30.0 * ZERO, 0.0, "carrier"),
            ("well past the end", 0.2, 0.0, "carrier"),
            ("well past the end, off it", 0.2, 30.0 * ZERO, "off"),
        ] {
            let (q, t) = at(along, off);
            let got = name(span.contact(q, t, b));
            assert_eq!(got, want, "{kind}, {label}");
        }
    }
}

/// **A closed carrier is read on the branch about the span's middle**: a
/// point given a whole turn away reads where it lies, and the parameter
/// an `On` reports is the span's.
#[test]
fn a_closed_carrier_reads_the_parameter_on_the_spans_branch() {
    let b = band();
    for carrier in [circle(), ellipse()] {
        let span = Span::new(&carrier, (-0.4, 0.6));
        let q = carrier.eval(0.1);
        for turns in [-2.0, -1.0, 1.0, 3.0] {
            let got = span.contact(q, 0.1 + turns * TAU, b).unwrap();
            let Contact::On { t, .. } = got else {
                panic!("{turns} turns away: {got:?}");
            };
            assert!((t - 0.1).abs() < 1e-12, "{turns} turns away: read at {t}");
        }
        // The antipode of the middle is past the span, whichever branch.
        let far = carrier.eval(0.1 + PI);
        assert_eq!(name(span.contact(far, 0.1 + PI, b)), "carrier");
        assert_eq!(name(span.contact(far, 0.1 - PI, b)), "carrier");
    }
}

/// **An endless span has no end to be near**: a closed edge's point at
/// its start reads on it, where the same span with ends reads `End`.
#[test]
fn an_endless_span_reads_its_end_point_on_it() {
    let b = band();
    let carrier = circle();
    let span = Span::new(&carrier, (0.0, TAU));
    let q = carrier.eval(0.0);
    assert_eq!(name(span.contact(q, 0.0, b)), "end");
    assert_eq!(name(span.endless().contact(q, 0.0, b)), "on");
    let off = q + Vec3::unit_z() * (30.0 * ZERO);
    assert_eq!(name(span.endless().contact(off, 0.0, b)), "off");
}

/// **An end is the end's point**, on a slow stretch too: where the
/// ellipse runs at its minor semi-axis, half a zero of arc past the end
/// reads at it and thirty zeros past it reads clear of it.
#[test]
fn an_end_is_read_by_its_chord_on_a_slow_stretch() {
    let b = band();
    let carrier = ellipse();
    // At `t = 0` the ellipse runs at its minor semi-axis, 0.5 m/rad.
    let span = Span::new(&carrier, (-PI / 2.0, 0.0));
    let t = 0.5 * ZERO / 0.5;
    assert_eq!(name(span.contact(carrier.eval(t), t, b)), "end");
    let t = 30.0 * ZERO / 0.5;
    assert_eq!(name(span.contact(carrier.eval(t), t, b)), "carrier");
}
