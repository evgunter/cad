//! [`meet`]'s rows: the table's totality, each pose on a pose built to
//! reach it, the near-tangent and near-concentric circle pairs the
//! boolean's crossing layer misplaced
//! (`work/cleave/a-near-tangent-coplanar-circle-pair-places-its-touch-off-both-circles.md`),
//! and, in [`super::meet_oracle`], the walk as the closed forms' oracle.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::{Curve3, CurveKind};
use geom_core::{Band, Point3, Vec3};

use super::{Arm, Hit, MeetFault, Meetings, Method, Pose, Sense, Span, meet, method, pose};

pub(super) const ZERO: f64 = 1e-9;

pub(super) fn band() -> Band {
    Band::new(ZERO, 10.0 * ZERO).unwrap()
}

pub(super) fn line(o: [f64; 3], d: [f64; 3]) -> Curve3<f64> {
    Curve3::Line {
        origin: Point3::from_array(o),
        dir: Vec3::from_array(d).normalize(),
    }
}

/// A frame `u_ref` square to `axis`.
pub(super) fn u_for(axis: Vec3<f64>) -> Vec3<f64> {
    let k = if axis.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    (k - axis * k.dot(axis)).normalize()
}

pub(super) fn circle(c: [f64; 3], axis: [f64; 3], r: f64) -> Curve3<f64> {
    let axis = Vec3::from_array(axis).normalize();
    Curve3::Circle {
        center: Point3::from_array(c),
        axis,
        radius: r,
        u_ref: u_for(axis),
    }
}

pub(super) fn ellipse(c: [f64; 3], axis: [f64; 3], u: [f64; 3], a: f64, b: f64) -> Curve3<f64> {
    let axis = Vec3::from_array(axis).normalize();
    let u = Vec3::from_array(u);
    Curve3::Ellipse {
        center: Point3::from_array(c),
        axis,
        semi_u: a,
        semi_v: b,
        u_ref: (u - axis * u.dot(axis)).normalize(),
    }
}

/// Whether an answer has a meeting on both spans' interiors.
pub(super) fn interiors_meet(m: &Meetings<f64>) -> bool {
    m.points
        .iter()
        .any(|p| matches!((p.on_a, p.on_b), (Hit::On { .. }, Hit::On { .. })))
}

/// The distance from `q` to a line or circle carrier, independently of
/// the reader.
pub(super) fn distance(c: &Curve3<f64>, q: Point3<f64>) -> f64 {
    match *c {
        Curve3::Line { origin, dir } => {
            let w = q - origin;
            let u = dir.normalize();
            (w - u * w.dot(u)).norm()
        }
        Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => {
            let w = q - center;
            let h = w.dot(axis);
            let rho = (w - axis * h).norm();
            ((rho - radius).powi(2) + h.powi(2)).sqrt()
        }
        Curve3::Ellipse { .. } | Curve3::Spiric { .. } | Curve3::Nurbs(_) => {
            panic!("no independent distance to a {:?}", c.kind())
        }
    }
}

/// **The table is total, and its closed arms are the line, circle and
/// ellipse pairs.** Every pair of kinds has a method; a pair is the
/// walk's exactly when a side is a spiric or a spline; and the mirrored
/// pair has the mirrored arm.
#[test]
fn the_table_reads_every_kind_pair_once() {
    let walked = |k: CurveKind| matches!(k, CurveKind::Spiric | CurveKind::Nurbs);
    let mirror = |arm: Arm| match arm {
        Arm::LineConic => Arm::ConicLine,
        Arm::ConicLine => Arm::LineConic,
        Arm::EllipseCircle => Arm::CircleEllipse,
        Arm::CircleEllipse => Arm::EllipseCircle,
        same @ (Arm::LineLine | Arm::CircleCircle | Arm::EllipseEllipse) => same,
    };
    for a in CurveKind::ALL {
        for b in CurveKind::ALL {
            match (method(a, b), method(b, a)) {
                (Method::Walk, Method::Walk) => {
                    assert!(walked(a) || walked(b), "({a:?}, {b:?}) walked");
                }
                (Method::Closed(arm), Method::Closed(back)) => {
                    assert!(!walked(a) && !walked(b), "({a:?}, {b:?}) closed");
                    assert_eq!(mirror(arm), back, "({a:?}, {b:?}) mirrors");
                }
                other => panic!("({a:?}, {b:?}) and its mirror differ: {other:?}"),
            }
        }
    }
}

/// The meetings of `a` over `ta` and `b` over `tb`.
fn meets(
    (a, ta): (&Curve3<f64>, (f64, f64)),
    (b, tb): (&Curve3<f64>, (f64, f64)),
) -> Result<Meetings<f64>, super::MeetRefusal<f64>> {
    meet(&Span::new(a, ta), &Span::new(b, tb), band())
}

/// Every meeting point lies on both line or circle carriers to within
/// the zero band.
fn on_both(label: &str, m: &Meetings<f64>, a: &Curve3<f64>, b: &Curve3<f64>) {
    for p in &m.points {
        let (da, db) = (distance(a, p.point), distance(b, p.point));
        assert!(
            da <= ZERO && db <= ZERO,
            "{label}: {:?} lies {da:e} and {db:e} off",
            p.point
        );
    }
}

/// **Two lines**: crossing, skew, parallel, and one line.
#[test]
fn two_lines_cross_miss_or_are_one() {
    let x = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let y = line([0.5, -1.0, 0.0], [0.0, 1.0, 0.0]);
    let m = meets((&x, (-1.0, 2.0)), (&y, (0.0, 3.0))).unwrap();
    assert!(interiors_meet(&m), "crossing: {m:?}");
    on_both("crossing", &m, &x, &y);
    let skew = line([0.5, -1.0, 30.0 * ZERO], [0.0, 1.0, 0.0]);
    let m = meets((&x, (-1.0, 2.0)), (&skew, (0.0, 3.0))).unwrap();
    assert!(m.points.is_empty(), "skew by thirty zeros: {m:?}");
    let beside = line([0.0, 1e-3, 0.0], [1.0, 0.0, 0.0]);
    let got = pose(
        &Span::new(&x, (0.0, 1.0)),
        &Span::new(&beside, (0.0, 1.0)),
        band(),
    );
    assert!(matches!(got, Ok(Pose::Parallel)), "{got:?}");
    let back = line([3.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    let m = meets((&x, (0.0, 2.0)), (&back, (0.0, 2.0))).unwrap();
    assert_eq!(m.shared, Some(Sense::Opposite));
    assert!(interiors_meet(&m), "overlapping runs of one line: {m:?}");
    let m = meets((&x, (0.0, 1.0)), (&back, (0.0, 1.5))).unwrap();
    assert!(!interiors_meet(&m), "disjoint runs of one line: {m:?}");
}

/// **A line and a circle**: through its plane on it and beside it,
/// parallel to it, and in it crossing, touching and apart.
#[test]
fn a_line_meets_a_circle_through_its_plane_or_in_it() {
    let c = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    let whole = (0.0, TAU - 0.1);
    let through = line([0.6, 0.8, -1.0], [0.0, 0.0, 1.0]);
    let m = meets((&through, (0.0, 2.0)), (&c, whole)).unwrap();
    assert!(interiors_meet(&m), "through the rim: {m:?}");
    on_both("through the rim", &m, &through, &c);
    let inside = line([0.3, 0.4, -1.0], [0.0, 0.0, 1.0]);
    let m = meets((&inside, (0.0, 2.0)), (&c, whole)).unwrap();
    assert!(m.points.is_empty(), "through the disc: {m:?}");
    let above = line([-2.0, 0.0, 1e-3], [1.0, 0.0, 0.0]);
    let got = pose(
        &Span::new(&above, (0.0, 4.0)),
        &Span::new(&c, whole),
        band(),
    );
    assert!(matches!(got, Ok(Pose::Parallel)), "{got:?}");
    let chord = line([-2.0, 0.5, 0.0], [1.0, 0.0, 0.0]);
    let m = meets((&chord, (0.0, 4.0)), (&c, whole)).unwrap();
    assert_eq!(m.points.len(), 2, "a chord crosses twice: {m:?}");
    on_both("a chord", &m, &chord, &c);
    let graze = line([-2.0, 1.0 + 0.5 * ZERO, 0.0], [1.0, 0.0, 0.0]);
    let got = pose(
        &Span::new(&graze, (0.0, 4.0)),
        &Span::new(&c, whole),
        band(),
    )
    .unwrap();
    let Pose::Tangent(touch) = got else {
        panic!("a graze half a zero off is a touch: {got:?}");
    };
    assert!(
        (touch.point - Point3::new(0.0, 1.0, 0.0)).norm() < 1e-12,
        "the touch is the extreme's own point: {:?}",
        touch.point
    );
    let clear = line([-2.0, 1.0 + 30.0 * ZERO, 0.0], [1.0, 0.0, 0.0]);
    let got = pose(
        &Span::new(&clear, (0.0, 4.0)),
        &Span::new(&c, whole),
        band(),
    );
    assert!(matches!(got, Ok(Pose::Apart)), "{got:?}");
}

/// **Two circles cut from one sphere**, their planes tilted by `θ`, built
/// to meet: the candidates are each circle's point on the other's plane,
/// read in its own angle, and land on both circles at every tilt the
/// band reads as definite, and in one plane below it.
#[test]
fn circles_on_one_sphere_meet_on_both_at_any_tilt() {
    let centre = Point3::new(3.7, -2.1, 0.4);
    let radius = 1.3;
    for theta in [1e-2_f64, 1e-4, 1e-5, 1e-6, 1e-7, 1e-11] {
        // Two planes through `centre + (0, h, 0)`, the second turned by
        // `θ` about the line there along `x`: the circles meet on it.
        let h = 0.5;
        let (s, c) = theta.sin_cos();
        let foot = |n: Vec3<f64>| {
            let d = Vec3::new(0.0, h, 0.0).dot(n);
            (centre + n * d, (radius * radius - d * d).sqrt())
        };
        let (n1, n2) = (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, c, s));
        let ((c1, r1), (c2, r2)) = (foot(n1), foot(n2));
        let a = circle(c1.to_array(), n1.to_array(), r1);
        let b = circle(c2.to_array(), n2.to_array(), r2);
        let label = format!("tilt {theta:e}");
        let m = meets((&a, (0.05, TAU - 0.05)), (&b, (0.05, TAU - 0.05)));
        let m = m.unwrap_or_else(|e| panic!("{label}: {e:?}"));
        assert!(!m.points.is_empty(), "{label}: they meet");
        on_both(&label, &m, &a, &b);
    }
}

/// **Two coplanar circles near tangency, centres `d` apart (cleave's
/// pin).** The crossing layer placed the touch of `d = 1e-8`,
/// `Δr = 0.9d` at `c₁ + e·(d² + r₁² − r₂²)/2d`, 0.1 m off both circles,
/// and read the edges clear. That pose's near extreme sits at the band's
/// zero threshold, a rounding past it, so the reader escalates there; a
/// little either side, every meeting lies on both circles, and the walk
/// agrees that they meet.
#[test]
fn a_near_tangent_coplanar_pair_is_placed_on_both_circles() {
    let a = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    let (sa, sb) = ((-PI + 0.1, PI - 0.1), (0.2, TAU - 0.2));
    let reproduction = circle([1e-8, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0 + 9e-9);
    let got = meets((&a, sa), (&reproduction, sb));
    assert!(
        matches!(got, Err(ref r) if matches!(r.fault, MeetFault::Escalated(_)) && r.found.is_empty()),
        "the reproduction: {got:?}"
    );
    for (label, d, r2) in [
        ("d = 1e-8, Δr = 0.95d", 1e-8, 1.0 + 9.5e-9),
        ("d = 1e-8, Δr = −0.95d", 1e-8, 1.0 - 9.5e-9),
        ("d = 3e-8, Δr = 1e-8", 3e-8, 1.0 + 1e-8),
    ] {
        let b = circle([d, 0.0, 0.0], [0.0, 0.0, 1.0], r2);
        let m = meets((&a, sa), (&b, sb)).unwrap_or_else(|e| panic!("{label}: {e:?}"));
        assert!(interiors_meet(&m), "{label}: {m:?}");
        on_both(label, &m, &a, &b);
        let walk = super::walk::meet(&Span::new(&a, sa), &Span::new(&b, sb), band());
        if let Ok(w) = walk {
            assert!(interiors_meet(&w), "{label}: the walk finds no meeting");
        }
    }
}

/// **Near-concentric arcs are decided** by the circle pair's arm: a radius
/// gap in the zero band is one curve, whose overlapping arcs meet; a
/// definite one is two circles on one centre, which meet nowhere.
#[test]
fn near_concentric_arcs_are_decided() {
    let a = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    for (gap, one_curve) in [(5e-10, true), (1e-7, false), (1e-5, false), (1e-3, false)] {
        let b = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0 + gap);
        let (sa, sb) = (Span::new(&a, (0.3, 2.9)), Span::new(&b, (0.5, 3.1)));
        let got = pose(&sa, &sb, band()).unwrap();
        let m = meet(&sa, &sb, band()).unwrap();
        if one_curve {
            assert!(
                matches!(
                    got,
                    Pose::Coincident {
                        sense: Sense::Same,
                        ..
                    }
                ),
                "{gap:e}: {got:?}"
            );
            assert!(interiors_meet(&m), "{gap:e}: the overlap meets");
        } else {
            assert!(matches!(got, Pose::Concentric), "{gap:e}: {got:?}");
            assert!(m.points.is_empty(), "{gap:e}: {m:?}");
        }
    }
}

/// **A circle and an ellipse in one plane** cross where the ellipse's
/// distance from the circle changes sign, four times here, in either
/// order; **an ellipse stored the other way round** is one curve with
/// itself; **two ellipses in one plane that are not one curve** refuse
/// typed.
#[test]
fn coplanar_conics_with_an_ellipse() {
    let c = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    let e = ellipse([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 2.0, 0.5);
    let (se, sc) = ((-PI, PI), (-PI + 0.05, PI - 0.05));
    let m = meets((&e, se), (&c, sc)).unwrap();
    assert_eq!(m.points.len(), 4, "four crossings: {m:?}");
    for p in &m.points {
        assert!(
            distance(&c, p.point) <= ZERO,
            "on the circle: {:?}",
            p.point
        );
    }
    let m = meets((&c, sc), (&e, se)).unwrap();
    assert_eq!(m.points.len(), 4, "in either order: {m:?}");
    let flipped = ellipse(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, -1.0],
        [-1.0, 0.0, 0.0],
        2.0,
        0.5,
    );
    let got = pose(
        &Span::new(&e, (0.0, 1.0)),
        &Span::new(&flipped, (2.0, 3.0)),
        band(),
    );
    assert!(matches!(got, Ok(Pose::Coincident { .. })), "{got:?}");
    let other = ellipse([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0], 2.0, 0.5);
    let got = meets((&e, (0.0, 1.0)), (&other, (0.0, 1.0)));
    assert!(
        matches!(got, Err(ref r) if matches!(r.fault, MeetFault::Unsupported { .. })),
        "{got:?}"
    );
}
