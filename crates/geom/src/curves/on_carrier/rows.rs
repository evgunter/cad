//! [`Curve3::foot`] and [`Curve3::on_carrier`] on every kind: a point a
//! stated number of zero bands off the carrier, read at its foot.

use geom_core::{Band, Point3, Vec3};

use super::{CarrierEscalation, CarrierRefusal, OnCarrier, PieceWalkRefusal};
use crate::Curve3;

const ZERO: f64 = 1e-9;

fn band() -> Band {
    Band::new(ZERO, 10.0 * ZERO).unwrap()
}

fn circle() -> Curve3<f64> {
    Curve3::circle(
        Point3::new(0.0, 0.0, 1.0),
        Vec3::unit_z(),
        2.0,
        Vec3::unit_x(),
    )
}

fn ellipse(a: f64, b: f64) -> Curve3<f64> {
    Curve3::Ellipse(crate::Ellipse::new(
        crate::Frame::new(Point3::new(0.2, -0.1, 0.4), Vec3::unit_z(), Vec3::unit_x()),
        a,
        b,
    ))
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

fn name<T>(r: Result<OnCarrier<T>, CarrierRefusal>) -> &'static str {
    match r {
        Ok(OnCarrier::On { .. }) => "on",
        Ok(OnCarrier::Off) => "off",
        Err(CarrierRefusal::Escalated(_)) => "escalated",
        Err(CarrierRefusal::Unwalked(PieceWalkRefusal::Exhausted)) => "exhausted",
        Err(CarrierRefusal::Unwalked(PieceWalkRefusal::Unbounded)) => "unbounded",
    }
}

/// **Every kind reads a point by its distance**: half a zero band off
/// the carrier is on it, thirty bands off is off it, read at the foot
/// found from a seed a whole radian away. A thin ellipse (`a/b = 1000`)
/// is the case the unit-coordinate angle alone gets wrong.
#[test]
fn a_point_is_on_or_off_every_carrier_by_its_distance() {
    for (c, t) in [
        (circle(), 0.7),
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
            let got = name(c.on_carrier(q, t + 1.0, band()));
            assert_eq!(got, want, "{:?} at {t}, {bands} bands", c.kind());
        }
    }
    let line = Curve3::line(Point3::new(1.0, 2.0, 3.0), Vec3::new(0.6, 0.0, 0.8));
    let q = line.eval(4.0) + Vec3::unit_y() * (0.5 * ZERO);
    assert_eq!(name(line.on_carrier(q, 0.0, band())), "on");
}

/// **Off needs the whole carrier clear.** An ellipse bending tighter
/// than the band (`b²/a = ε/100`) and a point `20ε` beyond its major
/// vertex: its gap at the vertex is `20ε`, past the band, while the
/// ellipse's lower bound there reads within the zero band, so the two
/// straddle the band and escalate typed, never `Off`.
#[test]
fn a_gap_past_the_band_with_the_bound_inside_it_straddles() {
    let b = (ZERO / 100.0).sqrt();
    let c = ellipse(1.0, b);
    let q = c.eval(0.0) + Vec3::unit_x() * (20.0 * ZERO);
    assert!(matches!(
        c.on_carrier(q, 0.0, band()),
        Err(CarrierRefusal::Escalated(CarrierEscalation::Straddle(_)))
    ));
}

#[test]
fn a_point_in_the_bands_gap_escalates() {
    for (c, t) in [(circle(), 0.7), (ellipse(3.0, 1.0), 1.2), (spiric(), 0.8)] {
        let q = c.eval(t) + normal(&c, t) * (5.0 * ZERO);
        assert_eq!(
            name(c.on_carrier(q, t, band())),
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
    assert!((foot.seed() - t).abs() < 1e-6, "{}", foot.seed());
    assert!((foot.point() - q).norm() == 0.0);
    assert!(
        foot.nearest().is_none(),
        "an ellipse's foot is a stationary point"
    );
}

/// **A closed spline's foot crosses its seam**: a point on the loop just
/// short of the domain's end, seeded at its start (the same point), and
/// one just past the start seeded at the end. The seed lies nearer than
/// any span sample, and its descent is held at the end it stands on, so
/// the foot is found from the other end, on the carrier.
#[test]
fn a_closed_splines_foot_crosses_its_seam() {
    let c = Curve3::Nurbs(std::sync::Arc::new(
        crate::NurbsCurve3::new(
            geom_core::spline::KnotVector::clamped(
                vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0],
                2,
            )
            .unwrap(),
            vec![
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(-1.0, 1.0, 0.0),
                Point3::new(-1.0, -1.0, 0.0),
                Point3::new(1.0, -1.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
            ],
            vec![1.0; 6],
        )
        .unwrap(),
    ));
    for (t, seed) in [(0.99, 0.0), (0.01, 1.0)] {
        let q = c.eval(t);
        let foot = c.foot(q, seed);
        assert!(
            foot.gap() < ZERO,
            "t {t} from {seed}: foot at {} gap {:e}",
            foot.seed(),
            foot.gap()
        );
        assert_eq!(
            name(c.on_carrier(q, seed, band())),
            "on",
            "t {t} from {seed}"
        );
    }
}

/// **A closed spline's foot is found from any seed**: two hundred closed
/// quadratic and cubic loops on uneven knots, twenty points each on the
/// loop within 0.03 of its seam, seeded at either end or anywhere: every
/// foot lies on the carrier.
#[test]
fn a_closed_splines_foot_is_found_from_any_seed() {
    let mut s: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut rnd = || {
        s = s
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((s >> 11) as f64) / ((1u64 << 53) as f64)
    };
    let mut miss = 0;
    for case in 0..200 {
        let deg = 2 + case % 2;
        let n = 5 + (rnd() * 6.0) as usize;
        let mut pts: Vec<Point3<f64>> = (0..n)
            .map(|i| {
                let a = std::f64::consts::TAU * i as f64 / n as f64;
                let r = 0.5 + rnd();
                Point3::new(r * a.cos(), r * a.sin(), 0.3 * (rnd() - 0.5))
            })
            .collect();
        pts.push(pts[0]);
        let m = pts.len();
        let w: Vec<f64> = (0..m - deg - 1).map(|_| 0.2 + rnd()).collect();
        let tot = w.iter().sum::<f64>() + 0.2 + rnd();
        let mut ks = vec![0.0; deg + 1];
        let mut c = 0.0;
        for a in &w {
            c += a / tot;
            ks.push(c);
        }
        ks.extend(vec![1.0; deg + 1]);
        let kv = geom_core::spline::KnotVector::clamped(ks, deg).unwrap();
        let curve = Curve3::Nurbs(std::sync::Arc::new(
            crate::NurbsCurve3::new(kv, pts, vec![1.0; m]).unwrap(),
        ));
        for _ in 0..20 {
            let t = if rnd() < 0.5 {
                rnd() * 0.03
            } else {
                1.0 - rnd() * 0.03
            };
            let seed = [0.0, 1.0, rnd()][(rnd() * 3.0) as usize];
            // A NaN gap is a miss too.
            if curve.foot(curve.eval(t), seed).gap().partial_cmp(&ZERO)
                != Some(core::cmp::Ordering::Less)
            {
                miss += 1;
            }
        }
    }
    assert_eq!(
        miss, 0,
        "{miss} points on a closed spline missed by their foot"
    );
}

/// A cubic hairpin's two branches tie across a wide `Interval` box: the
/// polish keeps the seed's branch, whose tangent no point of the box
/// has, and a narrower box picks the right one. So the tie reaches the
/// log as the driver's cue to refine.
#[test]
fn a_tie_across_a_hairpins_branches_reaches_the_log() {
    use geom_core::{Bounds, Interval, Real, k_stats::detached};
    let h = 0.2;
    let c = Curve3::Nurbs(std::sync::Arc::new(
        crate::NurbsCurve3::new(
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
    ));
    let p = c.eval(0.08);
    let ci: Curve3<Interval> = c.map_scalar(Interval::from_f64);
    let w = |x: f64| Interval::from_bounds(x - 0.05, x + 0.05);
    let (foot, rec) = detached(|| {
        ci.foot(
            Point3::new(w(p.x), w(p.y), w(p.z)),
            Interval::from_f64(0.92),
        )
    });
    assert!(
        foot.seed().lo() > 0.5,
        "the tie keeps the seed's branch: {:?}",
        foot.seed()
    );
    assert!(
        rec.recorded()
            .escalations
            .iter()
            .any(|e| e.predicate() == "curve_foot_polish"),
        "a tie across branches, which refining resolves, must reach the driver"
    );
}
