//! [`Curve3::foot`] and [`Curve3::on_carrier`] on every kind: a point a
//! stated number of zero bands off the carrier, read at its foot.

use geom_core::{Band, Point3, Vec3};

use super::{CarrierRefusal, OnCarrier};
use crate::Curve3;

const ZERO: f64 = 1e-9;

fn band() -> Band {
    Band::new(ZERO, 10.0 * ZERO).unwrap()
}

fn circle() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
        radius: 2.0,
    }
}

fn ellipse(a: f64, b: f64) -> Curve3<f64> {
    Curve3::Ellipse {
        center: Point3::new(0.2, -0.1, 0.4),
        axis: Vec3::unit_z(),
        semi_u: a,
        semi_v: b,
        u_ref: Vec3::unit_x(),
    }
}

fn spiric() -> Curve3<f64> {
    Curve3::Spiric {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
        major_radius: 3.0,
        minor_radius: 1.0,
        offset: 0.5,
    }
}

fn spline() -> Curve3<f64> {
    Curve3::Nurbs(std::sync::Arc::new(
        crate::NurbsCurve3::new(
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2)
                .unwrap(),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 0.5, 0.0),
                Point3::new(2.0, -0.5, 0.0),
                Point3::new(3.0, 0.0, 0.0),
            ],
            vec![1.0, 2.0, 1.0, 1.0],
        )
        .unwrap(),
    ))
}

/// The curve's in-plane-or-any normal at `t`.
fn normal(c: &Curve3<f64>, t: f64) -> Vec3<f64> {
    let d = c.ders1(t).1;
    let n = Vec3::unit_z().cross(d);
    if n.norm() > 1e-6 {
        n.normalize()
    } else {
        Vec3::unit_x().cross(d).normalize()
    }
}

fn name(r: Result<OnCarrier, CarrierRefusal>) -> &'static str {
    match r {
        Ok(OnCarrier::On { .. }) => "on",
        Ok(OnCarrier::Off) => "off",
        Err(CarrierRefusal::Escalated(_) | CarrierRefusal::Straddle(_)) => "escalated",
        Err(CarrierRefusal::Exhausted) => "exhausted",
        Err(CarrierRefusal::Unbounded) => "unbounded",
    }
}

/// **Every kind reads a point by its distance**: half a zero band off
/// the carrier is on it, thirty bands off is off it, read at the foot
/// found from a seed a whole radian away. A thin ellipse (`a/b = 1000`)
/// is the case the unit-coordinate angle alone gets wrong.
#[test]
fn a_point_is_on_or_off_every_carrier_by_its_distance() {
    let flipped = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
        radius: -2.0,
    };
    for (c, t) in [
        (circle(), 0.7),
        (flipped, 2.5),
        (ellipse(2.0, 1.0), 0.7),
        (ellipse(1e3, 1.0), 0.9),
        (ellipse(1.0, 1e-2), 2.1),
        (spiric(), 0.8),
        (spline(), 0.3),
        (spline(), 0.8),
    ] {
        let n = normal(&c, t);
        for (bands, want) in [(0.5, "on"), (30.0, "off")] {
            let q = c.eval(t) + n * (bands * ZERO);
            let foot = c.foot(q, t + 1.0);
            let got = name(c.on_carrier(q, &foot, band()));
            assert_eq!(got, want, "{:?} at {t}, {bands} bands", c.kind());
        }
    }
    let line = Curve3::Line {
        origin: Point3::new(1.0, 2.0, 3.0),
        dir: Vec3::new(0.6, 0.0, 0.8),
    };
    let q = line.eval(4.0) + Vec3::unit_y() * (0.5 * ZERO);
    assert_eq!(name(line.on_carrier(q, &line.foot(q, 0.0), band())), "on");
}

/// **Off needs the whole carrier clear.** A point on a circle read at a
/// foot handed in from a parameter half a turn away has a gap of the
/// diameter, yet the circle's exact distance puts it on the band's zero
/// side, so the two straddle the band and escalate. Read at its own foot
/// it is on.
#[test]
fn a_far_gap_with_the_carrier_near_straddles_and_escalates() {
    let c = circle();
    let q = c.eval(0.7);
    let far = c.foot(c.eval(0.7 + core::f64::consts::PI), 0.0);
    assert_eq!(name(c.on_carrier(q, &far, band())), "escalated");
    assert_eq!(name(c.on_carrier(q, &c.foot(q, 0.0), band())), "on");
}

/// **A point between the zero band and the escalation escalates**, five
/// bands off, on every closed-form kind.
#[test]
fn a_point_in_the_bands_gap_escalates() {
    for (c, t) in [(circle(), 0.7), (ellipse(3.0, 1.0), 1.2), (spiric(), 0.8)] {
        let q = c.eval(t) + normal(&c, t) * (5.0 * ZERO);
        let foot = c.foot(q, t);
        assert_eq!(
            name(c.on_carrier(q, &foot, band())),
            "escalated",
            "{:?}",
            c.kind()
        );
    }
}

/// **The foot's gap is never worse than its seed's**, and on a thin
/// ellipse it is the point's distance where the unit-coordinate angle
/// reads it `a/b` times farther.
#[test]
fn a_thin_ellipses_foot_reads_the_distance_not_its_unit_image() {
    let c = ellipse(1e3, 1.0);
    let t = 0.9;
    let q = c.eval(t) + normal(&c, t) * ZERO;
    let foot = c.foot(q, 0.0);
    assert!(foot.gap() < 1.5 * ZERO, "{}", foot.gap());
    assert!((foot.t() - t).abs() < 1e-6, "{}", foot.t());
}
