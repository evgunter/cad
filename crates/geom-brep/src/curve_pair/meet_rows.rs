//! [`meet`]'s rows: the table's totality, each pose on a pose built to
//! reach it, the near-tangent and near-concentric circle pairs the
//! boolean's crossing layer misplaced
//! (`work/cleave/a-near-tangent-coplanar-circle-pair-places-its-touch-off-both-circles.md`),
//! and, in [`super::meet_oracle`], the walk as the closed forms' oracle.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use geom::{Curve3, CurveKind};
use geom_core::{Band, Point3, Vec3};

use super::{Arm, Hit, MeetFault, Meetings, Method, Pose, Sense, Span, meet, method, pose};
use crate::roots::Unsettled;

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
            ((rho - radius.abs()).powi(2) + h.powi(2)).sqrt()
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
/// to meet: they cross at an angle of about `θ`. Above the basin floor
/// (the basin meter's crossing angle, about `√(8κe)`, `2.4·10⁻⁴` here) every meeting
/// lands on both circles;
/// below it the circles stay within the band's escalation `e` of each
/// other over a stretch of about `e/θ`, the polish's basin meter refuses
/// (`Unsettled::Meter`) rather than place one point on it; and in one
/// plane the coplanar arm decides.
#[test]
fn circles_on_one_sphere_meet_on_both_at_any_tilt() {
    let centre = Point3::new(3.7, -2.1, 0.4);
    let radius = 1.3;
    for (theta, decided) in [
        (1e-2_f64, true),
        (1e-3, true),
        (3e-4, true),
        (1e-4, false),
        (1e-5, false),
        (1e-7, false),
        (1e-11, true),
    ] {
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
        if !decided {
            assert!(
                matches!(m, Err(ref r) if matches!(r.fault, MeetFault::Unsettled(Unsettled::Meter))),
                "{label}: below the basin floor, {m:?}"
            );
            continue;
        }
        let m = m.unwrap_or_else(|e| panic!("{label}: {e:?}"));
        assert!(!m.points.is_empty(), "{label}: they meet");
        on_both(&label, &m, &a, &b);
    }
}

/// **Two coplanar circles near tangency, centres `d` apart (cleave's
/// pin).** The crossing layer placed the touch of `d = 1e-8`,
/// `Δr = 0.9d` at `c₁ + e·(d² + r₁² − r₂²)/2d`, 0.1 m off both circles,
/// and read the edges clear. Here, on that pose and a little either side
/// of it, the reader never misplaces: a decided answer meets, with every
/// meeting on both circles, a refusal is typed, and the walk agrees
/// wherever both decide. The touch lies past the first span, so where
/// its stretch reaches that span's ends the meetings are read there.
#[test]
fn a_near_tangent_coplanar_pair_is_placed_on_both_circles() {
    let a = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    let (sa, sb) = ((-PI + 0.1, PI - 0.1), (0.2, TAU - 0.2));
    for (label, d, r2) in [
        ("the reproduction, d = 1e-8, Δr = 0.9d", 1e-8, 1.0 + 9e-9),
        ("d = 1e-8, Δr = 0.95d", 1e-8, 1.0 + 9.5e-9),
        ("d = 1e-8, Δr = −0.95d", 1e-8, 1.0 - 9.5e-9),
        ("d = 3e-8, Δr = 1e-8", 3e-8, 1.0 + 1e-8),
    ] {
        let b = circle([d, 0.0, 0.0], [0.0, 0.0, 1.0], r2);
        let got = meets((&a, sa), (&b, sb));
        let found = match &got {
            Ok(m) => {
                assert!(!m.points.is_empty(), "{label}: decided no meeting: {m:?}");
                &m.points
            }
            Err(r) => &r.found,
        };
        on_both(
            label,
            &Meetings {
                shared: None,
                points: found.clone(),
            },
            &a,
            &b,
        );
        let walk = super::walk::meet(&Span::new(&a, sa), &Span::new(&b, sb), band());
        if let (Ok(_), Ok(w)) = (&got, walk) {
            assert!(!w.points.is_empty(), "{label}: the walk finds no meeting");
        }
    }
}

/// **A decided coplanar touch is the extreme's own point.** Two circles
/// in one plane `0.3` of a zero short of tangency, outside each other
/// (the near extreme in the zero band) and one inside the other (the far
/// extreme): each is `Pose::Tangent` at the point of tangency, which lies
/// on both circles and meets inside both spans; so does the first stored
/// with a negative radius, its points half a turn on.
#[test]
fn a_decided_coplanar_touch_lies_on_both_circles() {
    let a = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    let mut flipped = a.clone();
    if let Curve3::Circle { radius, .. } = &mut flipped {
        *radius = -1.0;
    }
    for (label, a, b, at, ta) in [
        (
            "outside, the first stored with a negative radius",
            flipped,
            circle([1.5 + 0.3 * ZERO, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5),
            Point3::new(1.0, 0.0, 0.0),
            PI,
        ),
        (
            "outside",
            a.clone(),
            circle([1.5 + 0.3 * ZERO, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5),
            Point3::new(1.0, 0.0, 0.0),
            0.0,
        ),
        (
            "inside",
            a.clone(),
            circle([1.0 - 0.3 * ZERO, 0.0, 0.0], [0.0, 0.0, 1.0], 2.0),
            Point3::new(-1.0, 0.0, 0.0),
            PI,
        ),
    ] {
        let (sa, sb) = (
            Span::new(&a, (ta - 1.0, ta + 1.0)),
            Span::new(&b, (0.1, TAU - 0.1)),
        );
        let got = pose(&sa, &sb, band()).unwrap_or_else(|e| panic!("{label}: {e:?}"));
        let Pose::Tangent(touch) = got else {
            panic!("{label}: a touch, got {got:?}");
        };
        assert!(
            (touch.point - at).norm() <= ZERO,
            "{label}: the touch is the point of tangency, {:?}",
            touch.point
        );
        let m = meet(&sa, &sb, band()).unwrap_or_else(|e| panic!("{label}: {e:?}"));
        assert!(interiors_meet(&m), "{label}: {m:?}");
        on_both(label, &m, &a, &b);
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
/// itself.
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
}

/// The ellipse `(2, 0.5)` about the origin in `z = 0`, its major axis
/// turned to `u`, moved by `at`.
fn flat_ellipse(at: [f64; 2], u: [f64; 2], a: f64, b: f64) -> Curve3<f64> {
    ellipse(
        [at[0], at[1], 0.0],
        [0.0, 0.0, 1.0],
        [u[0], u[1], 0.0],
        a,
        b,
    )
}

/// **Two ellipses in one plane cross where one's level on the other
/// changes sign.** The ellipses `(2, 0.5)` along `x̂` and along `ŷ` about
/// the origin cross at `(±s, ±s)`, `s = 1/√4.25`, four times over whole
/// turns, in either order, each meeting on both interiors at one of those
/// points; the first's quarter turn `(0, π/2)` holds one of them, at
/// `(s, s)`. Control: the second shrunk to `(1.9, 0.45)` still crosses
/// four times, each meeting moved off `(±s, ±s)`. Red where a crossing is
/// cleared, placed off either ellipse, or read twice.
#[test]
fn two_coplanar_ellipses_cross_where_the_level_changes_sign() {
    let s = 1.0 / 4.25_f64.sqrt();
    let x = flat_ellipse([0.0, 0.0], [1.0, 0.0], 2.0, 0.5);
    let whole = (-PI, PI);
    let near = |p: Point3<f64>| {
        [(s, s), (s, -s), (-s, s), (-s, -s)]
            .iter()
            .any(|&(u, v)| (p - Point3::new(u, v, 0.0)).norm() <= 1e-8)
    };
    for (what, y, at_s) in [
        (
            "square",
            flat_ellipse([0.0, 0.0], [0.0, 1.0], 2.0, 0.5),
            true,
        ),
        (
            "shrunk",
            flat_ellipse([0.0, 0.0], [0.0, 1.0], 1.9, 0.45),
            false,
        ),
    ] {
        for (order, got) in [
            ("first first", meets((&x, whole), (&y, whole))),
            ("second first", meets((&y, whole), (&x, whole))),
        ] {
            let m = got.unwrap_or_else(|e| panic!("{what}, {order}: {e:?}"));
            assert_eq!(m.points.len(), 4, "{what}, {order}: four crossings: {m:?}");
            assert!(
                m.points
                    .iter()
                    .all(|p| matches!((p.on_a, p.on_b), (Hit::On { .. }, Hit::On { .. }))),
                "{what}, {order}: each on both interiors: {m:?}"
            );
            for p in &m.points {
                assert_eq!(near(p.point), at_s, "{what}, {order}: {:?}", p.point);
                for (c, name) in [(&x, "first"), (&y, "second")] {
                    let on = c.on_carrier(p.point, c.param_near(p.point, 0.0).unwrap(), band());
                    assert!(
                        matches!(on, Ok(geom::OnCarrier::On { .. })),
                        "{what}, {order}: on the {name}: {:?} {on:?}",
                        p.point
                    );
                }
            }
        }
    }
    let y = flat_ellipse([0.0, 0.0], [0.0, 1.0], 2.0, 0.5);
    let m = meets((&x, (0.0, PI / 2.0)), (&y, whole)).unwrap();
    assert!(
        matches!(m.points[..], [p] if (p.point - Point3::new(s, s, 0.0)).norm() <= 1e-8),
        "the quarter turn's one crossing: {m:?}"
    );
}

/// **Two ellipses in one plane that keep apart meet nowhere**: the
/// ellipse `(1, 0.4)` inside `(2, 0.5)` about one centre, and the same
/// moved `5` along `x̂`, outside it, each decided apart with no meeting,
/// in either order. Control: the inner one widened to `(2.2, 0.4)` pokes
/// out through the other's ends and crosses it four times. Red where a
/// clear pair is refused or an undecided one cleared.
#[test]
fn two_coplanar_ellipses_apart_meet_nowhere() {
    let x = flat_ellipse([0.0, 0.0], [1.0, 0.0], 2.0, 0.5);
    let whole = (-PI, PI);
    for (what, y, crossings) in [
        ("inside", flat_ellipse([0.0, 0.0], [1.0, 0.0], 1.0, 0.4), 0),
        ("outside", flat_ellipse([5.0, 0.0], [1.0, 0.0], 1.0, 0.4), 0),
        (
            "poking out",
            flat_ellipse([0.0, 0.0], [1.0, 0.0], 2.2, 0.4),
            4,
        ),
    ] {
        for (order, got) in [
            ("first first", meets((&x, whole), (&y, whole))),
            ("second first", meets((&y, whole), (&x, whole))),
        ] {
            let m = got.unwrap_or_else(|e| panic!("{what}, {order}: {e:?}"));
            assert_eq!(m.points.len(), crossings, "{what}, {order}: {m:?}");
        }
        let got = pose(&Span::new(&x, whole), &Span::new(&y, whole), band());
        assert_eq!(
            matches!(got, Ok(Pose::Apart)),
            crossings == 0,
            "{what}: apart exactly where clear: {got:?}"
        );
    }
}

/// **Two ellipses in one plane that touch meet once, at the touch.** The
/// ellipses `(2, 1)` about the origin and about `(0, 2)` touch from
/// outside at `(0, 1)`, a double root of the level. The two stay within
/// the band of each other over about `10⁻⁴` of arc about it, wider than
/// the band, so no cut there reads a definite sign: the subdivision
/// places the touch at the level's extremum, in either order. Pulled
/// `10⁻³` apart they are decided apart; pushed `10⁻³` together they
/// cross twice, near `(±0.09, 1)`. Red where the touch refuses or is
/// cleared.
#[test]
fn two_coplanar_ellipses_that_touch_meet_once_at_the_touch() {
    let x = flat_ellipse([0.0, 0.0], [1.0, 0.0], 2.0, 1.0);
    let y = flat_ellipse([0.0, 2.0], [1.0, 0.0], 2.0, 1.0);
    let whole = (-PI, PI);
    for (order, got) in [
        ("first first", meets((&x, whole), (&y, whole))),
        ("second first", meets((&y, whole), (&x, whole))),
    ] {
        let m = got.unwrap_or_else(|e| panic!("{order}: the touch refuses: {e:?}"));
        assert!(
            m.points.len() == 1 && (m.points[0].point - Point3::new(0.0, 1.0, 0.0)).norm() < ZERO,
            "{order}: one meeting at the touch: {m:?}"
        );
    }
    let apart = meets(
        (&x, whole),
        (&flat_ellipse([0.0, 2.001], [1.0, 0.0], 2.0, 1.0), whole),
    )
    .unwrap();
    assert!(apart.points.is_empty(), "pulled apart: {apart:?}");
    let pushed = meets(
        (&x, whole),
        (&flat_ellipse([0.0, 1.999], [1.0, 0.0], 2.0, 1.0), whole),
    )
    .unwrap();
    assert_eq!(pushed.points.len(), 2, "pushed together: {pushed:?}");
    for p in &pushed.points {
        assert!(
            (p.point.y - 0.9995).abs() < 1e-3 && p.point.x.abs() < 0.2,
            "near the touch: {:?}",
            p.point
        );
    }
}

/// **Coplanar conic rims tangent at a vertex meet once, there, at every
/// band.** The rims of `ellipse_rims_tangent_outside_build_every_op`
/// (sweep): a boss `r = 0.3` about the origin and a hole `r = 0.1` about
/// `(0.4, 0)`, both tilted `0.3` rad and cut flat, so ellipses tangent
/// outside at `(0.3, 0)`; the hole as its circle; and an ellipse
/// `(0.2, 0.2/cos 0.3)` about `(0.1, 0)` tangent inside the boss's. Whole
/// turns starting at the touch, at its antipode, and short arcs about
/// it, in both orders, at ε = 1e-6, 1e-9 and 1e-12: one meeting, at the
/// touch. Red where the touch refuses (`curve_pair_coplanar_sub_side`).
#[test]
fn coplanar_conic_rims_tangent_at_a_vertex_meet_once_there() {
    let c = 0.3f64.cos();
    let boss = flat_ellipse([0.0, 0.0], [1.0, 0.0], 0.3, 0.3 / c);
    let hole = flat_ellipse([0.4, 0.0], [1.0, 0.0], 0.1, 0.1 / c);
    let round = circle([0.4, 0.0, 0.0], [0.0, 0.0, 1.0], 0.1);
    let inner = flat_ellipse([0.1, 0.0], [1.0, 0.0], 0.2, 0.2 / c);
    let touch = Point3::new(0.3, 0.0, 0.0);
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for (what, x, tx, y, ty) in [
            (
                "outside, both from the touch",
                &boss,
                (0.0, TAU),
                &hole,
                (PI, PI + TAU),
            ),
            (
                "outside, the hole's from it",
                &boss,
                (-PI, PI),
                &hole,
                (PI, PI + TAU),
            ),
            ("outside, neither", &boss, (-PI, PI), &hole, (0.0, TAU)),
            ("outside, two arcs", &boss, (-1.0, 1.0), &hole, (2.0, 4.0)),
            ("a circle outside", &boss, (-1.0, 1.0), &round, (2.0, 4.0)),
            (
                "inside, from the touch",
                &boss,
                (0.0, TAU),
                &inner,
                (0.0, TAU),
            ),
            ("inside, neither", &boss, (-PI, PI), &inner, (-PI, PI)),
        ] {
            for (order, got) in [
                (
                    "first first",
                    meet(&Span::new(x, tx), &Span::new(y, ty), band),
                ),
                (
                    "second first",
                    meet(&Span::new(y, ty), &Span::new(x, tx), band),
                ),
            ] {
                let m = got.unwrap_or_else(|e| panic!("ε {eps:e}, {what}, {order}: {e:?}"));
                assert!(
                    m.points.len() == 1 && (m.points[0].point - touch).norm() < eps,
                    "ε {eps:e}, {what}, {order}: one meeting at the touch: {m:?}"
                );
            }
        }
    }
}

/// `at` turned by `phi` about the origin, then moved by `shift`.
fn turned_by(at: [f64; 2], phi: f64, shift: [f64; 2]) -> [f64; 2] {
    let (s, c) = phi.sin_cos();
    [
        c * at[0] - s * at[1] + shift[0],
        s * at[0] + c * at[1] + shift[1],
    ]
}

/// **Ellipses crossing near a touch read one meeting only where the
/// noise parts them.** Three families, at ε = 1e-6, 1e-9 and 1e-12,
/// at three turns of the plane, in both orders:
/// - two equal `(2, 1)` ellipses, one pushed down so they cross twice
///   `s` apart, `s` from `0.1ε` to `10⁵ε`;
/// - `(2, 1)` and `(3, 1.5)` touching outside at `(0, 1)`, pushed
///   together likewise;
/// - an osculating pair (`(2, 1)` and `(√8, 2)` about `(0, −1)`, equal
///   curvature at the touch), pushed together, pulled apart, or touching.
///
/// No crossing pair reads no meeting. Crossings more than `10ε` apart
/// whose depth is past the noise (`2·10⁻¹³`) never read as one meeting,
/// and every meeting lies within `10ε` of a true crossing. A pair pulled
/// apart past `10ε` reads none. Red where the touch's reach is not read.
#[test]
fn ellipses_crossing_near_a_touch_read_one_meeting_only_where_the_noise_parts_them() {
    let mut bad = Vec::new();
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for phi in [0.0f64, 0.7, 2.3] {
            let shift = if phi == 0.0 { [0.0, 0.0] } else { [3.1, -1.7] };
            let u = [phi.cos(), phi.sin()];
            // Case 1: equal ellipses (2, 1), the second pushed down by δ:
            // crossings at (±2√(δ − δ²/4), 1 − δ/2).
            for k in [0.1, 0.5, 2.0, 5.0, 20.0, 100.0, 1e3, 1e4, 1e5] {
                let s = k * eps;
                let cy = 2.0 - s * s / 16.0;
                let delta = 2.0 - cy;
                if delta == 0.0 {
                    continue;
                }
                let half = 2.0 * (delta - delta * delta / 4.0).sqrt();
                let s_true = 2.0 * half;
                let a = flat_ellipse(turned_by([0.0, 0.0], phi, shift), u, 2.0, 1.0);
                let b = flat_ellipse(turned_by([0.0, cy], phi, shift), u, 2.0, 1.0);
                let truth = [
                    turned_by([half, 1.0 - delta / 2.0], phi, shift),
                    turned_by([-half, 1.0 - delta / 2.0], phi, shift),
                ];
                for (order, got) in [
                    (
                        "ab",
                        meet(&Span::new(&a, (-PI, PI)), &Span::new(&b, (-PI, PI)), band),
                    ),
                    (
                        "ba",
                        meet(&Span::new(&b, (-PI, PI)), &Span::new(&a, (-PI, PI)), band),
                    ),
                ] {
                    let label = format!("eq eps {eps:e} phi {phi} s {s_true:.3e} ({k}eps) {order}");
                    if let Ok(m) = &got {
                        if m.points.is_empty() {
                            bad.push(format!("{label}: no meeting"));
                        }
                        if m.points.len() == 1 && s_true > 10.0 * eps && delta > 2e-13 {
                            bad.push(format!("{label}: one meeting, crossings {s_true:e} apart"));
                        }
                        for p in &m.points {
                            let d = truth
                                .iter()
                                .map(|t| (p.point - Point3::new(t[0], t[1], 0.0)).norm())
                                .fold(f64::INFINITY, f64::min);
                            if d > 10.0 * eps && delta > 2e-13 {
                                bad.push(format!("{label}: point {d:e} off a crossing"));
                            }
                        }
                    }
                }
            }
            // Case 2: (2, 1) and (3, 1.5) touching outside at (0, 1),
            // pushed together δ: crossings at x ≈ ±√(24δ/5).
            for k in [0.5, 2.0, 5.0, 20.0, 1e3, 1e5] {
                let s = k * eps;
                let x = s / 2.0;
                let cy = 2.5 - x * x * 5.0 / 24.0;
                let delta = 2.5 - cy;
                if delta == 0.0 {
                    continue;
                }
                let half = (24.0 * delta / 5.0).sqrt();
                let a = flat_ellipse(turned_by([0.0, 0.0], phi, shift), u, 2.0, 1.0);
                let b = flat_ellipse(turned_by([0.0, cy], phi, shift), u, 3.0, 1.5);
                for (order, got) in [
                    (
                        "ab",
                        meet(&Span::new(&a, (-PI, PI)), &Span::new(&b, (-PI, PI)), band),
                    ),
                    (
                        "ba",
                        meet(&Span::new(&b, (-PI, PI)), &Span::new(&a, (-PI, PI)), band),
                    ),
                ] {
                    let label = format!(
                        "uneq eps {eps:e} phi {phi} s {:.3e} ({k}eps) {order}",
                        2.0 * half
                    );
                    if let Ok(m) = &got {
                        if m.points.is_empty() {
                            bad.push(format!("{label}: no meeting"));
                        }
                        if m.points.len() == 1 && 2.0 * half > 10.0 * eps && delta > 2e-13 {
                            bad.push(format!("{label}: one meeting, crossings apart"));
                        }
                    }
                }
            }
            // Case 3: osculating. (2, 1) about the origin and (√8, 2)
            // about (0, −1): equal curvature 1/4 at (0, 1), the second
            // above by x⁴/256. Pushed down δ they cross at
            // x = ±(256δ)^¼; pulled up δ they are δ apart.
            for dy in [
                0.0,
                -1e-14,
                -1e-12,
                -1e-10,
                -1e-8,
                1e-14,
                1e-10,
                -0.1 * eps,
                -eps,
                -100.0 * eps,
                0.5 * eps,
                5.0 * eps,
                100.0 * eps,
            ] {
                let cy = -1.0 + dy;
                let delta = -(cy + 1.0);
                let a = flat_ellipse(turned_by([0.0, 0.0], phi, shift), u, 2.0, 1.0);
                let b = flat_ellipse(turned_by([0.0, cy], phi, shift), u, 8.0f64.sqrt(), 2.0);
                let s_true = if delta > 0.0 {
                    2.0 * (256.0 * delta).powf(0.25)
                } else {
                    0.0
                };
                for (order, got) in [
                    (
                        "ab",
                        meet(&Span::new(&a, (-PI, PI)), &Span::new(&b, (-PI, PI)), band),
                    ),
                    (
                        "ba",
                        meet(&Span::new(&b, (-PI, PI)), &Span::new(&a, (-PI, PI)), band),
                    ),
                ] {
                    let label =
                        format!("osc eps {eps:e} phi {phi} dy {dy:e} s {s_true:.3e} {order}");
                    if let Ok(m) = &got {
                        if delta > 0.0 && m.points.is_empty() {
                            bad.push(format!("{label}: crossing, no meeting"));
                        }
                        if delta < -10.0 * eps && !m.points.is_empty() {
                            bad.push(format!("{label}: apart, met"));
                        }
                        if m.points.len() == 1 && s_true > 10.0 * eps && delta > 1e-13 {
                            bad.push(format!("{label}: one meeting, crossings {s_true:e} apart"));
                        }
                    }
                }
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// What a pair whose least distance is in the zero band may answer: a
/// meeting on both interiors, or a refusal. A decided answer with no
/// meeting prunes an in-band decision (D4).
fn meets_or_refuses(label: &str, got: &Result<Meetings<f64>, super::MeetRefusal<f64>>) {
    if let Ok(m) = got {
        assert!(interiors_meet(m), "{label}: decided no meeting: {m:?}");
    }
}

/// A unit circle in `z = 0` through the origin along `x̂`, and a unit
/// circle through `h·ẑ` along `ŷ` in a plane turned `tilt` about `ŷ`:
/// they cross at a right angle, their planes `tilt` apart, and their
/// least distance is `h`, at the origin. Each span runs a radian either
/// side of it.
fn shallow_circles(tilt: f64, h: f64) -> [(Curve3<f64>, (f64, f64)); 2] {
    let (s, c) = tilt.sin_cos();
    let e = Vec3::new(c, 0.0, -s);
    let a = circle([0.0, -1.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    let at_b = Point3::new(0.0, 0.0, h);
    let b = circle((at_b - e).to_array(), [s, 0.0, c], 1.0);
    let tb = b.param_near(at_b, 0.0).unwrap();
    let ta = PI / 2.0;
    [(a, (ta - 1.0, ta + 1.0)), (b, (tb - 1.0, tb + 1.0))]
}

/// **Two curves in nearly parallel planes, crossing at a wide angle half
/// a zero apart, meet**: the meeting is not where either crosses the
/// other's plane, which lies `h/sin(tilt)` along it. Circle × circle, line
/// × circle and circle × line. At a tilt of `2·10⁻⁸` the stretch about
/// the crossing is half a metre, and the polish's basin meter
/// refuses (`Unsettled::Meter`).
#[test]
fn an_in_band_miss_across_nearly_parallel_planes_meets() {
    let [(a, sa), (b, sb)] = shallow_circles(2e-8, 0.5 * ZERO);
    let got = meets((&a, sa), (&b, sb));
    assert!(
        matches!(got, Err(ref r) if matches!(r.fault, MeetFault::Unsettled(Unsettled::Meter))),
        "tilt 2e-8: {got:?}"
    );
    for tilt in [1e-7, 1e-5, 1e-3] {
        let [(a, sa), (b, sb)] = shallow_circles(tilt, 0.5 * ZERO);
        let label = format!("circles, tilt {tilt:e}");
        let got = meets((&a, sa), (&b, sb));
        meets_or_refuses(&label, &got);
        let m = got.unwrap_or_else(|e| panic!("{label}: {e:?}"));
        on_both(&label, &m, &a, &b);
        let x = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        for (label, got) in [
            (
                format!("line × circle, tilt {tilt:e}"),
                meets((&x, (-1.0, 1.0)), (&b, sb)),
            ),
            (
                format!("circle × line, tilt {tilt:e}"),
                meets((&b, sb), (&x, (-1.0, 1.0))),
            ),
        ] {
            meets_or_refuses(&label, &got);
            let m = got.unwrap_or_else(|e| panic!("{label}: {e:?}"));
            on_both(&label, &m, &x, &b);
        }
    }
}

/// A unit circle `a` turned `tilt` about `x̂`, its lowest point at
/// `(0, 0, h)` and its angle there `−π/2`, and a unit circle `b` in
/// `z = 0` crossing `a`'s shadow at a right angle... `angle` off its
/// tangent, `along` radians round `a` from that lowest point.
fn touching_circles(tilt: f64, h: f64, along: f64, angle: f64) -> [(Curve3<f64>, (f64, f64)); 2] {
    let (s, c) = tilt.sin_cos();
    let a = circle([0.0, c, h + s], [0.0, -s, c], 1.0);
    let x = Point3::new(along.sin(), c * (1.0 - along.cos()), 0.0);
    let dir = along + angle;
    let b_center = x + Vec3::new(-dir.sin(), dir.cos(), 0.0);
    let b = circle(b_center.to_array(), [0.0, 0.0, 1.0], 1.0);
    let tb = b.param_near(x, 0.0).unwrap();
    let ta = -PI / 2.0 + along;
    [(a, (ta - 1.0, ta + 1.0)), (b, (tb - 1.0, tb + 1.0))]
}

/// **A circle whose lowest point touches the other's plane within the
/// band meets the other where it crosses its shadow**, not at that
/// lowest point, which lies a millimetre from the crossing: their least
/// distance there is `6.5·10⁻¹⁰`, in the zero band, so the answer is a
/// meeting or a refusal; where the touch's stretch is short against
/// the angle the shadow is crossed at, a meeting on both.
#[test]
fn a_touch_of_the_other_plane_meets_where_the_curves_do() {
    let along: f64 = 1e-3;
    let pose_of = |tilt: f64, angle: f64| {
        let lift = tilt.sin() * (1.0 - along.cos());
        touching_circles(tilt, 6.5e-10 - lift, along, angle.to_radians())
    };
    for tilt in [1.2e-8_f64, 1e-7, 1e-6] {
        for angle in [2.0, 5.0, 20.0, -20.0] {
            let [(a, sa), (b, sb)] = pose_of(tilt, angle);
            meets_or_refuses(
                &format!("tilt {tilt:e}, crossing {angle}°"),
                &meets((&a, sa), (&b, sb)),
            );
        }
    }
    let [(a, sa), (b, sb)] = pose_of(1e-4, 90.0);
    let m = meets((&a, sa), (&b, sb)).unwrap();
    assert!(interiors_meet(&m), "tilt 1e-4, square: {m:?}");
    on_both("tilt 1e-4, square", &m, &a, &b);
}

/// **A thin ellipse lying in a circle's plane, its own plane turned
/// about its long axis, crosses the circle in that plane.** The planes'
/// tilt read at the long semi-axis is definite, but the ellipse's swing
/// above the circle's plane, at most its short semi-axis times the tilt,
/// reads zero: it lies in that plane, and the in-plane arm decides.
#[test]
fn a_thin_ellipse_in_the_circles_plane_crosses_it_there() {
    for (b_over_a, tilt) in [(1e-3, 5e-7), (1e-2, 5e-8)] {
        let (s, c) = f64::sin_cos(tilt);
        let e = ellipse(
            [0.0, 0.0, 0.0],
            [0.0, -s, c],
            [1.0, 0.0, 0.0],
            1.0,
            b_over_a,
        );
        let k = circle([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5);
        let label = format!("b/a {b_over_a:e}, tilt {tilt:e}");
        for (got, order) in [
            (
                meets((&e, (-PI, PI)), (&k, (0.1, TAU - 0.1))),
                "ellipse first",
            ),
            (
                meets((&k, (0.1, TAU - 0.1)), (&e, (-PI, PI))),
                "circle first",
            ),
        ] {
            let m = got.unwrap_or_else(|r| panic!("{label}, {order}: {r:?}"));
            assert_eq!(m.points.len(), 2, "{label}, {order}: {m:?}");
            for p in &m.points {
                let off = distance(&k, p.point);
                assert!(off <= ZERO, "{label}, {order}: {off:e} off the circle");
            }
        }
    }
}

/// **A touch past one span whose stretch reaches into both meets at
/// the span's end** (the oracle's counterexample, seed 3 at effort 40,
/// draw 6159). Two circles in one plane `5·10⁻⁸` apart, their radii's gap
/// as near: the touch at `−π/2` lies `0.0026` rad past the second span's
/// end, but round it they stay within `2·10⁻¹⁰` of each other for a fifth
/// of a radian, so the second span's end meets the first.
#[test]
fn a_touch_past_a_span_meets_at_its_end() {
    let axis = [0.6740110957474658, -0.7358278013826933, 0.06531837047578558];
    let u = Vec3::new(0.7387212213069966, 0.6713711321490372, -0.05959664510372472);
    let mk = |c: [f64; 3], radius: f64| Curve3::Circle {
        center: Point3::from_array(c),
        axis: Vec3::from_array(axis),
        radius,
        u_ref: u,
    };
    let a = mk(
        [0.8401501109250811, 0.9273190696785121, 1.472118831061076],
        2.2967052369001464,
    );
    let b = mk(
        [0.8401501109250811, 0.9273190741477213, 1.4721188814078372],
        2.296705287626492,
    );
    let (sa, sb) = (
        (1.4046786871774843, 5.893025302260511),
        (0.40467868717748434, 4.709841330271556),
    );
    let got = meets((&a, sa), (&b, sb));
    let m = got.unwrap_or_else(|e| panic!("{e:?}"));
    assert!(
        m.points
            .iter()
            .any(|p| matches!((p.on_a, p.on_b), (Hit::On { .. }, Hit::End(_)))),
        "the second span's end meets the first: {m:?}"
    );
    on_both("past a span", &m, &a, &b);
}

/// **Two near-concentric circles in slightly slanted planes read as one
/// reading whichever arm holds them**: as two circles, and with the
/// second stored as an ellipse of equal semi-axes, the radii's gap and
/// the slant out of one plane are charged alike, so both read one curve
/// or both refuse.
#[test]
fn a_circle_reads_as_an_equal_ellipse_does() {
    let a = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    for (gap, tilt) in [(2e-10, 1e-10), (7e-10, 5e-10), (3e-10, 6e-10)] {
        let (s, c) = f64::sin_cos(tilt);
        let r = 1.0 + gap;
        let b = circle([0.0, 0.0, 0.0], [0.0, -s, c], r);
        let e = ellipse([0.0, 0.0, 0.0], [0.0, -s, c], [1.0, 0.0, 0.0], r, r);
        let (sa, sb) = ((0.3, 2.9), (0.5, 3.1));
        let shape = |got: Result<Meetings<f64>, super::MeetRefusal<f64>>| match got {
            Ok(m) => format!("shared {:?}", m.shared),
            Err(r) => format!("{:?}", r.fault)
                .split('(')
                .next()
                .unwrap()
                .to_owned(),
        };
        let as_circle = shape(meets((&a, sa), (&b, sb)));
        let as_ellipse = shape(meets((&e, sb), (&a, sa)));
        assert_eq!(as_circle, as_ellipse, "gap {gap:e}, tilt {tilt:e}");
    }
}

/// **A crossing past a segment's end at a shallow angle meets at that
/// end**: two lines `5·10⁻⁸` rad apart cross a millimetre past the first
/// segment's end, where the other line passes `5·10⁻¹¹` from it.
#[test]
fn a_shallow_crossing_past_a_segment_meets_at_its_end() {
    let phi: f64 = 5e-8;
    let x = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let y = line([1.0, 0.0, 0.0], [phi.cos(), phi.sin(), 0.0]);
    let m = meets((&x, (-1.0, 1.0 - 1e-3)), (&y, (-1.0, 1.0))).unwrap();
    assert!(
        m.points
            .iter()
            .any(|p| matches!((p.on_a, p.on_b), (Hit::End(_), Hit::On { .. }))),
        "the segment's end meets the other line: {m:?}"
    );
    on_both("past a segment", &m, &x, &y);
}

/// **A touch at a span's end is the one meeting of its stretch.** The
/// unit circle's arc from `0.3` to `π/2`, and a unit circle centred at
/// `(0, 2)` touching it from outside at `(0, 1)`, the arc's end. The two
/// stay within the zero band of each other over about `√z` of arc either
/// side of the touch, so points of the arc's interior clear of its end
/// lie within the band of the other circle; but that stretch holds one
/// meeting, at the end, and it takes the whole stretch. Two circles
/// kissing a tenth of a metre inside the arc's end meet there on both
/// interiors, and pushed `0.1 m` apart meet nowhere. Red where a point of
/// the end's stretch is read as a meeting of its own.
#[test]
fn a_touch_at_a_spans_end_is_the_one_meeting_of_its_stretch() {
    let arc = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    let span = (0.3, PI / 2.0);
    let touching = |c: [f64; 3]| circle(c, [0.0, 0.0, 1.0], 1.0);
    let at = |theta: f64| [2.0 * theta.cos(), 2.0 * theta.sin(), 0.0];
    let (end, inside, apart) = (at(PI / 2.0), at(PI / 2.0 - 0.1), at(PI / 2.0 - 0.1));
    let apart = [apart[0] * 1.05, apart[1] * 1.05, 0.0];
    for (what, c, want) in [
        ("at the end", end, false),
        ("inside", inside, true),
        ("apart", apart, false),
    ] {
        let other = touching(c);
        let m = meets((&arc, span), (&other, (-PI, PI))).unwrap();
        assert_eq!(interiors_meet(&m), want, "{what}: {m:?}");
        on_both(what, &m, &arc, &other);
        if what == "at the end" {
            assert!(
                matches!(m.points[..], [p] if matches!(p.on_a, Hit::End(_))),
                "one meeting, at the arc's end: {m:?}"
            );
        }
    }
}

/// **A line tangent to an arc at its end, its radius a rounding long,
/// meets at the end.** The fillet of radius `0.25 + 6·10⁻¹⁷` about
/// `(2, 0.25)`, from `−π/2` to `0`, and the line `x = 2.25` from its end
/// upward: the residual's near extreme is `−5.5·10⁻¹⁷`, a crossing only
/// the rounding makes, whose two roots would lie `10⁻⁸` apart, each in
/// the band's gap of the shared end. It is a touch, read at the end of
/// both. Red when the root gap is read at the extreme's own value.
#[test]
fn a_tangent_whose_crossing_is_rounding_meets_at_the_end() {
    let r = 0.25_f64 + 6e-17;
    let arc = Curve3::Circle {
        center: Point3::new(2.0, 0.25, 1.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(0.0, -1.0, 0.0),
    };
    let edge = line([2.25, 0.25, 1.0], [0.0, 1.0, 0.0]);
    let m = meets((&edge, (0.0, 1.75)), (&arc, (0.0, PI / 2.0))).unwrap();
    assert!(!interiors_meet(&m), "{m:?}");
    assert!(
        m.points
            .iter()
            .any(|p| matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
        "at both ends: {m:?}"
    );
}

/// **A shallow crossing at both spans' ends is the one meeting of its
/// stretch**: the unit arc from `0.3` to `π/2`, and a segment leaving its
/// end `(0, 1)` at `φ` above the tangent. They part by about `φ·s` over
/// an arc `s`, so the arc's interior lies within the zero band of the
/// segment for `z/φ`: `8·10⁻⁸` at `φ = 0.012`, past the escalation clear
/// of the end. That stretch's one meeting is the joint, read at both
/// ends, and the line's other crossing of the circle, divided out of the
/// line × circle form, lies past both spans. Red where the stretch's
/// points are read again, or the joint's root re-derived.
#[test]
fn a_shallow_crossing_at_both_ends_is_the_one_meeting_of_its_stretch() {
    let arc = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    for phi in [0.004_f64, 0.012, 0.016, 0.02] {
        let edge = line([0.0, 1.0, 0.0], [phi.cos(), phi.sin(), 0.0]);
        for (m, order) in [
            (
                meets((&arc, (0.3, PI / 2.0)), (&edge, (0.0, 1.0))),
                "arc first",
            ),
            (
                meets((&edge, (0.0, 1.0)), (&arc, (0.3, PI / 2.0))),
                "line first",
            ),
        ] {
            let m = m.unwrap();
            assert!(
                matches!(m.points[..], [p] if matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
                "φ = {phi}, {order}: the joint alone: {m:?}"
            );
            on_both(&format!("φ = {phi}"), &m, &arc, &edge);
        }
    }
}

/// **Two crossings only the rounding separates merge into one touch.** A
/// line `10⁻¹²` inside a circle of radius `10³` crosses it twice, about
/// `90 µm` apart, but the residual's near extreme, `−10⁻¹²`, is within
/// its own rounding there: the pose reads one touch, at the extreme's
/// point, on both carriers within the zero band. A line `10⁻⁶` inside
/// crosses definitely, twice.
#[test]
fn crossings_only_the_rounding_separates_are_one_touch() {
    let r = 1e3;
    let c = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], r);
    for (depth, touch) in [(1e-12, true), (1e-6, false)] {
        let l = line([-1.0, r - depth, 0.0], [1.0, 0.0, 0.0]);
        let (sl, sc) = (Span::new(&l, (0.0, 2.0)), Span::new(&c, (0.0, PI)));
        let got = pose(&sl, &sc, band()).unwrap();
        if touch {
            assert!(matches!(got, Pose::Tangent(_)), "{depth:e} inside: {got:?}");
        } else {
            assert!(
                matches!(got, Pose::Generic(ref cs) if cs.len() == 2),
                "{depth:e} inside: {got:?}"
            );
        }
        let m = meet(&sl, &sc, band()).unwrap();
        assert!(interiors_meet(&m), "{depth:e} inside: {m:?}");
        on_both(&format!("{depth:e} inside"), &m, &l, &c);
    }
}

/// **Two lines read whole or in pieces get one verdict.** A line along
/// `x̂` over `(0, 8)` and a second a millimetre beside it over `(1, 5)`,
/// tilted `1.5·10⁻¹⁰` toward it. Where they can meet is their overlap,
/// four metres, over which the tilt moves them `6·10⁻¹⁰`, within the zero
/// band: parallel, read whole and read in the pieces a sweep would split
/// the first into. Levered by the longer span, the whole pair read the
/// tilt in the band's gap (`1.2·10⁻⁹`) and escalated where its pieces
/// decided. Red where the lever is a span's own length.
#[test]
fn two_lines_read_whole_or_in_pieces_meet_alike() {
    let s = 1.5e-10;
    let x = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let beside = line([0.0, 1e-3, 0.0], [1.0, -s, 0.0]);
    for span in [(0.0, 8.0), (0.0, 2.0), (2.0, 6.0), (6.0, 8.0)] {
        let got = pose(
            &Span::new(&x, span),
            &Span::new(&beside, (1.0, 5.0)),
            band(),
        );
        assert!(
            matches!(got, Ok(Pose::Parallel)),
            "{span:?}: one verdict, parallel: {got:?}"
        );
        let m = meets((&x, span), (&beside, (1.0, 5.0))).unwrap();
        assert!(m.points.is_empty(), "{span:?}: {m:?}");
    }
    // A pair whose tilt over the overlap is in the band's gap: `x̂` over
    // `(0, 8)` against a line through `(100, 0, 0)` tilted `s` toward it
    // over `(-99, -93)`, which projects on `x ∈ [1, 7]`, about `96·s` off
    // it there, and meets it a hundred metres on. At `s = 10⁻⁹` the tilt
    // over the overlap, `s·(6 + 2e)`, is in the gap, so a router that
    // only goes parallel on a zero tilt escalated whole where the pieces
    // read parallel. The parallel margin, `96·s − s·(L/2 + e)`, decides
    // positive whole and in every piece. Red where the tilt routes first.
    let pieces = |(t0, t1): (f64, f64), k: usize| {
        (0..k)
            .map(|i| {
                let at = |j: usize| t0 + (t1 - t0) * j as f64 / k as f64;
                (at(i), at(i + 1))
            })
            .collect::<Vec<_>>()
    };
    for s in [1e-9, 3e-9, 1e-8, 1e-7, 1e-6] {
        let far = line([100.0, 0.0, 0.0], [1.0, s, 0.0]);
        for k in [1, 2, 3, 10] {
            for span in pieces((0.0, 8.0), k) {
                let got = pose(
                    &Span::new(&x, span),
                    &Span::new(&far, (-99.0, -93.0)),
                    band(),
                );
                assert!(
                    matches!(got, Ok(Pose::Parallel)),
                    "tilt {s:e}, {k} pieces, {span:?}: {got:?}"
                );
            }
        }
    }
}

/// **A line and a circle read whole or in pieces get one verdict.** The
/// circle of radius 3 about `(4, 0, 0)` in `z = 0`, and a line along `x̂`
/// at `y = 5` over `(0, 8)`, `10⁻⁷` above the plane at `x = 4` and tilted
/// `d̂·n̂ = s` out of it. Where they can meet is `x ∈ [1, 7]`, over which
/// the height moves `3·s`: parallel, whole and in pieces. At `s = 10⁻⁹`
/// the tilt over the overlap is in the band's gap, which escalated whole
/// where a piece read parallel. Red where the tilt routes first.
#[test]
fn a_line_and_a_circle_read_whole_or_in_pieces_meet_alike() {
    let c = circle([4.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0);
    for s in [1e-9, 3e-9, 1e-8] {
        let up = line([4.0, 5.0, 1e-7], [1.0, 0.0, s]);
        for k in [1, 2, 3, 10] {
            for i in 0..k {
                let at = |j: usize| -4.0 + 8.0 * j as f64 / k as f64;
                let span = (at(i), at(i + 1));
                let got = pose(&Span::new(&up, span), &Span::new(&c, (0.0, TAU)), band());
                assert!(
                    matches!(got, Ok(Pose::Parallel)),
                    "tilt {s:e}, {k} pieces, {span:?}: {got:?}"
                );
            }
        }
    }
}

/// **Two lines end to end at a shallow corner are no one curve.** Two
/// segments a metre long meeting at the origin at `0.5°`–`2.8°`: their
/// projections overlap at one point, where their distance is zero, but
/// each one's far end lies centimetres off the other. `Coincident` claims
/// one curve over both spans, so the pair is the crossing at the corner,
/// met at both ends.
#[test]
fn a_shallow_corner_is_no_one_curve() {
    let x = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    for deg in [0.5_f64, 1.0, 2.0, 2.8] {
        let (sn, cs) = deg.to_radians().sin_cos();
        let y = line([0.0, 0.0, 0.0], [cs, sn, 0.0]);
        let got = pose(
            &Span::new(&x, (-1.0, 0.0)),
            &Span::new(&y, (0.0, 1.0)),
            band(),
        );
        assert!(matches!(got, Ok(Pose::Generic(_))), "{deg}°: {got:?}");
        let m = meets((&x, (-1.0, 0.0)), (&y, (0.0, 1.0))).unwrap();
        assert!(
            m.shared.is_none()
                && !m.points.is_empty()
                && m.points
                    .iter()
                    .all(|p| matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
            "{deg}°: {m:?}"
        );
    }
}

/// **A near-parallel pair is read where it can meet.** A line along `x̂`
/// over `(0, 1)`, and a second `2·10⁻⁸` beside it at the first's span,
/// converging on it at `10⁻¹⁰` per metre over `(0, 100)`, so it would
/// reach it two hundred metres on, far past both spans. Over the overlap
/// the gap moves `10⁻¹⁰`: parallel, no meeting. Levered by the second
/// span, the tilt read `10⁻⁸` and escalated. Set `5ε` beside instead, the
/// gap at the overlap is in the band's gap and the pose escalates; set
/// `ε/2` beside, it is one line.
#[test]
fn a_near_parallel_pair_is_read_over_its_overlap() {
    let s = 1e-10;
    let x = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let read = |off: f64| {
        let other = line([0.0, off, 0.0], [1.0, -s, 0.0]);
        pose(
            &Span::new(&x, (0.0, 1.0)),
            &Span::new(&other, (0.0, 100.0)),
            band(),
        )
    };
    assert!(
        matches!(read(20.0 * ZERO), Ok(Pose::Parallel)),
        "{:?}",
        read(20.0 * ZERO)
    );
    assert!(read(5.0 * ZERO).is_err(), "{:?}", read(5.0 * ZERO));
    // `ε/2` beside at the first span, the second's far end lies
    // `9.5·10⁻⁹` off the first line, in the band's gap: one curve over
    // the overlap is no one curve over both spans, and the pose
    // escalates. Over `(0, 5)` its far end stays within `ε` of the first
    // line, and the pair is one curve.
    assert!(read(0.5 * ZERO).is_err(), "{:?}", read(0.5 * ZERO));
    let short = pose(
        &Span::new(&x, (0.0, 1.0)),
        &Span::new(&line([0.0, 0.5 * ZERO, 0.0], [1.0, -s, 0.0]), (0.0, 5.0)),
        band(),
    );
    assert!(matches!(short, Ok(Pose::Coincident { .. })), "{short:?}");
}

/// **Two segments on one line, two metres apart, meet nowhere**, read
/// on the census's own pair: a straight edge of the near-tangent probe's
/// `w345 nt e0 a4 d1e-7` union and the face boundary edge it was cut
/// against, at `ε = 10⁻⁶`. Each span's far end lies `1.06·10⁻⁶` off the
/// other's line, in the band's gap, so reading the ends against the
/// carriers escalated, though the spans' projections on the first line
/// part by two metres.
#[test]
fn two_segments_on_one_line_two_metres_apart_meet_nowhere() {
    let band = Band::new(1e-6, 1e-5).unwrap();
    let edge = Curve3::Line {
        origin: Point3::new(-1.999_356_330_248_317_7, 0.535_723_727_818_681_7, 1.0),
        dir: Vec3::new(0.965_926_090_343_448_7, -0.258_818_059_636_146_96, 0.0),
    };
    let boundary = Curve3::Line {
        origin: Point3::new(1.999_999_999_999_999_8, -0.535_898_384_862_245_2, 1.0),
        dir: Vec3::new(-0.965_925_826_289_068_3, 0.258_819_045_102_520_7, 0.0),
    };
    let a = Span::new(&edge, (0.0, 2.069_885_419_015_256_5));
    let b = Span::new(&boundary, (0.0, 0.000_666_381_398_214_661_7));
    for (label, x, y) in [("edge first", &a, &b), ("boundary first", &b, &a)] {
        let got = pose(x, y, band);
        assert!(matches!(got, Ok(Pose::Apart)), "{label}: pose {got:?}");
        let m = meet(x, y, band);
        assert!(
            m.as_ref()
                .is_ok_and(|m| m.points.is_empty() && m.shared.is_none()),
            "{label}: meetings {m:?}"
        );
    }
}

/// **The gap between two lines' projections decides only where it is
/// past the band.** The first segment runs along `x̂` over `(0, 1)`; the
/// second lies on a line `2ε` off it at the origin and leaning `ε` per
/// metre, so every end of either lies in the band's gap of the other's
/// line, or on one inside the zero band throughout. Read over `(3, 4)`
/// along `x̂`, two metres apart, they meet nowhere, whole or in pieces;
/// reading the ends against the lines escalated there. Overlapping, or
/// with a gap in or below the band, the gap says nothing: a pair inside
/// the zero band is one curve, or meets at the joint, and no such pair
/// is read apart.
#[test]
fn a_gap_along_one_line_past_the_band_meets_nowhere() {
    let x = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let leaning = line([0.0, 2.0 * ZERO, 0.0], [1.0, ZERO, 0.0]);
    let within = line([0.0, 0.2 * ZERO, 0.0], [1.0, 0.1 * ZERO, 0.0]);
    let read = |c: &Curve3<f64>, ta: (f64, f64), tb: (f64, f64)| {
        pose(&Span::new(&x, ta), &Span::new(c, tb), band())
    };
    for (label, c) in [("leaning", &leaning), ("within", &within)] {
        for (ta, tb) in [
            ((0.0, 1.0), (3.0, 4.0)),
            ((0.0, 0.5), (3.0, 3.5)),
            ((0.5, 1.0), (3.5, 4.0)),
            ((0.0, 1.0), (4.0, 3.0)),
        ] {
            let got = read(c, ta, tb);
            assert!(
                matches!(got, Ok(Pose::Apart)),
                "{label} {ta:?} {tb:?}: {got:?}"
            );
            let back = pose(&Span::new(c, tb), &Span::new(&x, ta), band());
            assert!(
                matches!(back, Ok(Pose::Apart)),
                "{label} {ta:?} {tb:?} reversed: {back:?}"
            );
        }
    }
    for (label, tb) in [
        ("overlapping", (0.5, 1.5)),
        ("joined in the zero band", (1.0 + 0.5 * ZERO, 2.0)),
        ("a gap in the band's gap", (1.0 + 5.0 * ZERO, 2.0)),
    ] {
        for (which, c) in [("leaning", &leaning), ("within", &within)] {
            let got = read(c, (0.0, 1.0), tb);
            assert!(
                !matches!(got, Ok(Pose::Apart | Pose::Parallel)),
                "{which} {label}: read apart: {got:?}"
            );
        }
    }
    let one = read(&within, (0.0, 1.0), (0.5, 1.5));
    assert!(
        matches!(one, Ok(Pose::Coincident { .. })),
        "overlapping within: {one:?}"
    );
    let m = meets((&x, (0.0, 1.0)), (&within, (0.5, 1.5))).unwrap();
    assert!(
        m.shared == Some(Sense::Same) && interiors_meet(&m),
        "overlapping within: {m:?}"
    );
    let m = meets((&x, (0.0, 1.0)), (&within, (1.0 + 0.5 * ZERO, 2.0))).unwrap();
    assert!(!m.points.is_empty(), "joined within: {m:?}");
}

/// **A line in a circle's plane, far along from it, meets nowhere.** The
/// line runs `3ε` outside the unit circle's top, so its in-plane
/// distance's extreme is in the band's gap; over `(5, 6)` along `x̂` its
/// span projects four metres past the circle's extent, and it meets
/// nowhere, in either order. Over `(−½, ½)`, above the circle's top, the
/// touch in the band's gap still escalates.
#[test]
fn a_line_far_along_from_a_circle_in_its_plane_meets_nowhere() {
    let c = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    let l = line([0.0, 1.0 + 3.0 * ZERO, 0.0], [1.0, 0.0, 0.0]);
    let whole = (0.0, TAU);
    let got = pose(&Span::new(&l, (5.0, 6.0)), &Span::new(&c, whole), band());
    assert!(matches!(got, Ok(Pose::Apart)), "line first: {got:?}");
    let got = pose(&Span::new(&c, whole), &Span::new(&l, (5.0, 6.0)), band());
    assert!(matches!(got, Ok(Pose::Apart)), "circle first: {got:?}");
    let m = meets((&c, whole), (&l, (5.0, 6.0))).unwrap();
    assert!(m.points.is_empty(), "circle first: {m:?}");
    let m = meets((&l, (-0.5, 0.5)), (&c, whole));
    assert!(
        matches!(&m, Err(r) if matches!(r.fault, MeetFault::Escalated(_))),
        "over the top: {m:?}"
    );
}

/// **The ring rule: a candidate nearer a found meeting than the band's
/// escalation width is not read again.** Two segments joined square at
/// the origin meet there, at the end of both. A candidate `5·10⁻⁹` along
/// the first lies in the band's gap of the first span's end, so read on
/// its own it escalates (the control); within the escalation of the
/// joint it can only be the joint again, and it is not read, whether it
/// comes before the joint or after. Red where the ring is the zero band.
#[test]
fn a_candidate_in_a_found_meetings_ring_is_not_read_again() {
    let (x, y) = (
        line([0.0; 3], [1.0, 0.0, 0.0]),
        line([0.0; 3], [0.0, 1.0, 0.0]),
    );
    let (a, b) = (Span::new(&x, (0.0, 1.0)), Span::new(&y, (0.0, 1.0)));
    let joint = super::Candidate {
        point: Point3::origin(),
        ta: 0.0,
        tb: 0.0,
    };
    let ring = super::Candidate {
        point: Point3::new(5e-9, 0.0, 0.0),
        ta: 5e-9,
        tb: 0.0,
    };
    let mut alone = super::Found::default();
    alone.read(&a, &b, ring, band());
    assert!(
        matches!(
            alone.finish(&a, &b, None, band()),
            Err(super::MeetRefusal {
                fault: MeetFault::Escalated(_),
                ..
            })
        ),
        "the ring's candidate read alone escalates"
    );
    for (order, cs) in [
        ("joint first", [joint, ring]),
        ("ring first", [ring, joint]),
    ] {
        let mut found = super::Found::default();
        for c in cs {
            found.read(&a, &b, c, band());
        }
        let m = found
            .finish(&a, &b, None, band())
            .unwrap_or_else(|e| panic!("{order}: {e:?}"));
        assert!(
            matches!(m.points[..], [p] if matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
            "{order}: the joint alone: {m:?}"
        );
    }
}

/// A carrier and the parameters of a span on it.
type Carried = (Curve3<f64>, (f64, f64));

/// A line leaving the end `(0, R)` of an arc of radius `R = 10⁻²` from
/// `0.3` to `π/2`, through the arc's point `δ` before its end: the arc
/// span, the line's over a millimetre, and that second point.
fn secant_from_the_end(delta: f64) -> ([Carried; 2], Point3<f64>) {
    let r = 1e-2;
    let arc = circle([0.0; 3], [0.0, 0.0, 1.0], r);
    let end = arc.eval(PI / 2.0);
    let again = arc.eval(PI / 2.0 - delta);
    let d = again - end;
    let edge = line(end.to_array(), d.to_array());
    ([(arc, (0.3, PI / 2.0)), (edge, (0.0, 1e-3))], again)
}

/// **Two meetings in one stretch escalate.** A line leaves an arc's end
/// and meets the arc again `δ·R` further on, `20 µm` at `δ = 2·10⁻³`;
/// between the two it stays `R·δ²/8 = 5·10⁻⁹` off the arc, in the band's
/// gap, so the spans are nowhere definitely apart there, and whether the
/// two are two crossings or one run is undecided: it escalates, carrying
/// both meetings. At `δ = 2·10⁻²` the line is `5·10⁻⁷` off between them,
/// two stretches, and both meetings are reported: the joint at the ends,
/// and the second, divided out of the line × circle form, on both
/// carriers at the arc's point. Red where a stretch's second meeting is
/// reported beside its first.
#[test]
fn two_meetings_in_one_stretch_escalate() {
    for (delta, one_stretch) in [(2e-3, true), (2e-2, false)] {
        let ([arc, edge], again) = secant_from_the_end(delta);
        for (order, got) in [
            ("arc first", meets((&arc.0, arc.1), (&edge.0, edge.1))),
            ("line first", meets((&edge.0, edge.1), (&arc.0, arc.1))),
        ] {
            let label = format!("δ = {delta:e}, {order}");
            let points = match (&got, one_stretch) {
                (Err(r), true) => {
                    assert!(
                        matches!(r.fault, MeetFault::Escalated(_)),
                        "{label}: {got:?}"
                    );
                    &r.found
                }
                (Ok(m), false) => &m.points,
                _ => panic!("{label}: {got:?}"),
            };
            assert_eq!(points.len(), 2, "{label}: both meetings: {got:?}");
            assert!(
                points
                    .iter()
                    .any(|p| matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
                "{label}: the joint: {got:?}"
            );
            let second = points
                .iter()
                .find(|p| matches!((p.on_a, p.on_b), (Hit::On { .. }, Hit::On { .. })))
                .unwrap_or_else(|| panic!("{label}: the second meeting: {got:?}"));
            assert!(
                (second.point - again).norm() <= ZERO,
                "{label}: the second meeting is the arc's point: {second:?}"
            );
            on_both(
                &label,
                &Meetings {
                    shared: None,
                    points: points.clone(),
                },
                &arc.0,
                &edge.0,
            );
        }
    }
}

/// **A closed form divides a found meeting's root out: a tangent joint
/// decides.** A rod's rim of radius `0.1` about `(0.2, −0.3464)`, an arc
/// of `π/3` ending tangent to the line `x = 0.3` at the line's start, in
/// the enclosures a rotation leaves its frame (a swept rod's rim, read
/// at interval). The ends find the joint, at
/// the end of both; the line × circle form's other root, `2θ_hi − θ_J`,
/// is the joint again, so the pair meets there alone, whichever span is
/// read first. Re-derived instead, the double root's gap comes from a
/// square root whose rounding is `√ulp` of the radius, and it escalates
/// (`curve_pair_pose_root_gap`). Red where the pose re-derives the
/// joint's root.
#[test]
fn a_found_meetings_root_is_divided_out_of_the_closed_form() {
    use geom_core::Interval;
    let i = Interval::from_bounds;
    let p = |x, y, z| Point3::new(x, y, z);
    let v = |x, y, z| Vec3::new(x, y, z);
    let rim = Curve3::Circle {
        center: p(
            i(0.199_999_999_999_999_76, 0.200_000_000_000_000_2),
            i(-0.346_410_161_513_776_7, -0.346_410_161_513_774_13),
            i(-1.615_374_500_829_602_8e-14, 1.604_272_270_583_351_2e-14),
        ),
        axis: v(
            i(0.0, 0.0),
            i(0.0, 0.0),
            i(-1.000_000_000_000_032, -0.999_999_999_999_968_1),
        ),
        radius: i(0.1, 0.1),
        u_ref: v(
            i(0.499_999_999_999_959_2, 0.500_000_000_000_041),
            i(-0.866_025_403_784_513_9, -0.866_025_403_784_363_2),
            i(-3.236_300_116_782_472_7e-13, 3.236_300_116_782_472_7e-13),
        ),
    };
    let rim_span = (
        i(-1.047_197_551_196_718_9, -1.047_197_551_196_476),
        i(-6.525_890_938_746_67e-13, 6.539_213_615_042_172e-13),
    );
    let edge = Curve3::Line {
        origin: p(i(0.3, 0.3), i(-0.4, -0.4), i(0.0, 0.0)),
        dir: v(
            i(0.0, 0.0),
            i(0.999_999_999_999_999_7, 1.000_000_000_000_000_4),
            i(0.0, 0.0),
        ),
    };
    let edge_span = (
        i(0.053_589_838_486_223_26, 0.053_589_838_486_225_98),
        i(0.746_410_161_513_773_7, 0.746_410_161_513_777_4),
    );
    let (r, e) = (Span::new(&rim, rim_span), Span::new(&edge, edge_span));
    for (order, got) in [
        ("rim first", meet(&r, &e, band())),
        ("line first", meet(&e, &r, band())),
    ] {
        let got = got.unwrap_or_else(|e| panic!("{order}: {e:?}"));
        assert!(
            matches!(got.points[..], [p] if matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
            "{order}: the joint alone: {got:?}"
        );
    }
}

/// **A crossing of the other's plane at a found meeting is read by its
/// window.** A rod of radius `0.1` cut at 45°, its ellipse's quarter
/// from `(−0.8586, 0, 0.9)`, and a fillet's circle in the plane `y = 0`
/// ending there, tangent to it: the ellipse touches the circle's plane at
/// the joint. The ends find the joint. The height harmonic's root there is
/// a crossing of the plane, not the joint's own equation, and its window
/// stays within the band of the circle over `4·10⁻⁴` rad, far past the
/// joint's ring. Polished, the closest-pair meter's basin reads the two
/// tangents parallel and refuses (`Unsettled::Meter`). Read instead
/// (`pose::window`), the window is on the circle next to the ring, then
/// off it past the zero band, so it holds the joint alone, whichever
/// span is read first. Red where the window is polished.
#[test]
fn a_crossing_of_the_others_plane_at_a_found_meeting_is_read_by_its_window() {
    let h = core::f64::consts::FRAC_1_SQRT_2;
    let x = -0.758_578_643_762_690_5;
    let rim = Curve3::Ellipse {
        center: Point3::new(x, -0.1, 0.9),
        axis: Vec3::new(h, h, 0.0),
        semi_u: 0.141_421_356_237_309_53,
        semi_v: 0.1,
        u_ref: Vec3::new(-h, h, 0.0),
    };
    let fillet = Curve3::Circle {
        center: Point3::new(x, 0.0, 0.9),
        axis: Vec3::new(0.0, 1.0, 0.0),
        radius: 0.1,
        u_ref: Vec3::new(0.0, 0.0, 1.0),
    };
    let (sr, sf) = ((0.0, PI / 2.0), (-3.0 * PI / 4.0, -PI / 2.0));
    for (order, got) in [
        ("rim first", meets((&rim, sr), (&fillet, sf))),
        ("fillet first", meets((&fillet, sf), (&rim, sr))),
    ] {
        let got = got.unwrap_or_else(|e| panic!("{order}: {e:?}"));
        assert!(
            matches!(got.points[..], [p] if matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
            "{order}: the joint alone: {got:?}"
        );
    }
}

/// **On a span with no ends, two meetings across the seam are next to
/// each other.** A whole circle of radius `10⁻²` read with no ends, its
/// seam at `(R, 0)`, and the chord between its points `±10⁻³` rad about
/// the seam, `5·10⁻⁹` off the circle at its middle: the chord's ends are
/// two meetings in one stretch, across the seam, so it escalates. Held
/// at `10⁻³` and `2π − 10⁻³` along the circle, the halfway point inside
/// the span's parameters lies half a turn away and definitely off the
/// chord. Red where the arc over the seam goes unread.
#[test]
fn two_meetings_across_an_endless_spans_seam_escalate() {
    let r = 1e-2;
    let c = circle([0.0; 3], [0.0, 0.0, 1.0], r);
    let (lo, hi) = (c.eval(-1e-3), c.eval(1e-3));
    let chord = line(lo.to_array(), (hi - lo).to_array());
    let whole = Span::new(&c, (0.0, TAU)).endless();
    let seg = Span::new(&chord, (0.0, (hi - lo).norm()));
    let escalated = |refused: Option<&super::MeetRefusal<f64>>| {
        refused.is_some_and(|r| matches!(r.fault, MeetFault::Escalated(_)) && r.found.len() == 2)
    };
    for (order, got) in [
        ("circle first", meet(&whole, &seg, band())),
        ("chord first", meet(&seg, &whole, band())),
    ] {
        assert!(escalated(got.as_ref().err()), "{order}: {got:?}");
    }
    let mut found = super::Found::default();
    for (point, ta, tb) in [(hi, 1e-3, (hi - lo).norm()), (lo, TAU - 1e-3, 0.0)] {
        found.read(&whole, &seg, super::Candidate { point, ta, tb }, band());
    }
    assert_eq!(found.points.len(), 2, "both chord ends meet the circle");
    let got = found.finish(&whole, &seg, None, band());
    assert!(
        escalated(got.as_ref().err()),
        "held across the seam: {got:?}"
    );
}

/// **Two lines joined at a vertex meet there alone.** Two segments
/// leaving `J = (1, 2, 0)` at `10⁻⁷` rad to each other, a metre long,
/// each line stored from an origin a kilometre behind `J`. Within the
/// band of each other over their first centimetre, they meet in one
/// stretch, whose meeting is the joint, found at both ends. The line ×
/// line form's crossing, solved from the origins with no square root,
/// lands within `10⁻⁹` of `J`, inside the joint's ring, so it is not read
/// again. Red where the ring is narrower than the form's rounding, or the
/// form's crossing is placed off the joint.
#[test]
fn two_lines_joined_at_a_vertex_meet_there_alone() {
    let j = Point3::new(1.0, 2.0, 0.0);
    let phi = 1e-7_f64;
    let u1 = Vec3::new(1.0, 0.0, 0.0);
    let u2 = Vec3::new(phi.cos(), phi.sin(), 0.0);
    let back = 1000.0;
    let x = line((j - u1 * back).to_array(), u1.to_array());
    let y = line((j - u2 * back).to_array(), u2.to_array());
    let span = (back, back + 1.0);
    for (order, got) in [
        ("first first", meets((&x, span), (&y, span))),
        ("second first", meets((&y, span), (&x, span))),
    ] {
        let m = got.unwrap_or_else(|e| panic!("{order}: {e:?}"));
        assert!(
            matches!(m.points[..], [p] if matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
            "{order}: the joint alone: {m:?}"
        );
    }
}

/// **The coplanar ellipse pair's ceiling holds** (`pose::level_ceiling`):
/// wherever a point lies in an ellipse's plane, the signed lower bound
/// on its distance from the ellipse ([`geom::ellipse_level_clearance`]),
/// which the subdivision reads as the residual, is at least
/// `min(reach, |F|/ceiling)`, `F` the ellipse's level there. Read over a
/// grid of points out to six semi-major axes, on ellipses of axis ratio
/// `1` to `10⁻³`. Anti-vacuity, each a witness the grid holds: the worst
/// `|F|·s/bound` passes `3` (so a ceiling of `1/s` breaks the row), and a
/// point lies past `s/4` where `|F|·s/4` exceeds its bound (so a reach of
/// `4s` breaks it). Red where the ceiling or its reach is weakened.
#[test]
fn the_coplanar_ellipse_ceiling_holds_at_its_extremes() {
    let (mut worst, mut past_reach) = (0.0_f64, 0usize);
    for (a, b) in [
        (1.0, 1.0),
        (2.0, 1.0),
        (1.0, 0.3),
        (3.0, 0.1),
        (1.0, 1e-3),
        (0.2, 1.0),
    ] {
        let small: f64 = f64::min(a, b);
        let (ceiling, reach) = super::pose::level_ceiling(small);
        let extent = 6.0 * f64::max(a, b);
        let n = 400;
        for i in 0..=n {
            for k in 0..=n {
                let x = extent * (2.0 * f64::from(i) / f64::from(n) - 1.0);
                let y = extent * (2.0 * f64::from(k) / f64::from(n) - 1.0);
                let f = (x / a).powi(2) + (y / b).powi(2) - 1.0;
                let bound = geom::ellipse_level_clearance((x, y), (a, b)).abs();
                assert!(
                    bound >= f64::min(reach, f.abs() / ceiling) * (1.0 - 1e-12),
                    "({a}, {b}) at ({x}, {y}): bound {bound} under min({reach}, {})",
                    f.abs() / ceiling
                );
                if bound > 0.0 {
                    worst = worst.max(f.abs() * small / bound);
                    if bound < f64::min(4.0 * small, f.abs() * small / 4.0) {
                        past_reach += 1;
                    }
                }
            }
        }
    }
    assert!(worst > 3.0, "a point the ceiling 1/s misses: worst {worst}");
    assert!(past_reach > 0, "a point the reach 4s misses");
}

/// **Meetings are next to each other along the span, whatever order
/// they are found in.** The unit circle from `−π/2` to `3π/2`, and the
/// ellipse `(1 + η, 0.5)` about its centre, `η = 1.5·10⁻¹⁰`: they cross
/// at `±θ₀` and `π ± θ₀`, `θ₀ ≈ 10⁻⁵`, and between `−θ₀` and `θ₀` the
/// ellipse lies at most `η` off the circle, inside the zero band, so
/// those two are two meetings in one stretch (and so are the two about
/// `π`). Found in the order `θ₀, π − θ₀, −θ₀, π + θ₀`, the pair escalates
/// on the stretch (`curve_pair_meet_stretch`), carrying all four. Read in
/// that order instead of along the circle, each pair's halfway point lies
/// near `π/2`, half a metre off the ellipse, and the pair would be
/// decided. Red where the meetings are not ordered along the span.
#[test]
fn meetings_are_neighbours_along_the_span_whatever_order_they_are_found_in() {
    let eta = 1.5e-10;
    let (big, minor) = (1.0 + eta, 0.5_f64);
    let c = circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
    let e = flat_ellipse([0.0, 0.0], [1.0, 0.0], big, minor);
    let s2 = (1.0 - 1.0 / big.powi(2)) / (1.0 / minor.powi(2) - 1.0 / big.powi(2));
    let theta = s2.sqrt().asin();
    let (sc, se) = (
        Span::new(&c, (-PI / 2.0, 1.5 * PI)),
        Span::new(&e, (0.0, TAU)).endless(),
    );
    let mut found = super::Found::default();
    for ta in [theta, PI - theta, -theta, PI + theta] {
        let point = c.eval(ta);
        let tb = e.param_near(point, 0.0).unwrap();
        found.read(&sc, &se, super::Candidate { point, ta, tb }, band());
    }
    assert_eq!(found.points.len(), 4, "four meetings: {:?}", found.points);
    let got = found.finish(&sc, &se, None, band());
    assert!(
        matches!(
            &got,
            Err(r) if r.found.len() == 4
                && matches!(r.fault, MeetFault::Escalated(d) if d.predicate == Some("curve_pair_meet_stretch"))
        ),
        "two meetings in one stretch escalate: {got:?}"
    );
}

/// **Two segments that touch, on carriers far from them, are never read
/// apart** (the #5481 reviewer's witness). Both carriers run along the
/// unit `d`, with origins at `∓d·2²⁰`; `a`'s span ends at `t₁` and `b`'s
/// starts at `t₁ − 2²¹`, every input exact in `f64`, so in Q
/// `a(t₁) = b(t₁ − 2²¹) = d·(t₁ − 2²⁰)`: the spans touch at one point.
/// Evaluated in `f64`, `origin + dir·t` rounds at `|t| ≈ 2²⁰` m, and the
/// two ends land `5.8·10⁻¹¹` apart in `y`, past the band. Charged only at
/// the ends' own distances from the origin, the projections read that
/// rounding as a gap of `2.2·10⁻¹¹` past their charge, and `meet`
/// answered "no meeting" for a touching pair (the PR's first head).
/// Charged for the evaluation (`geom_core::one_line`, "The charge"), the
/// pair meets or refuses, in both orders.
#[test]
fn two_segments_touching_on_far_carriers_are_never_read_apart() {
    let fine = Band::new(1e-12, 1e-11).unwrap();
    let far = 2f64.powi(20);
    let d = Vec3::new(
        0.803_992_292_151_306_2,
        0.499_755_838_673_424_7,
        -0.322_242_914_387_750_1,
    );
    let a = Curve3::Line {
        origin: Point3::origin() - d * far,
        dir: d,
    };
    let b = Curve3::Line {
        origin: Point3::origin() + d * far,
        dir: d,
    };
    let t1 = 1_049_576.874_015_081_7;
    let s0 = t1 - 2.0 * far;
    assert_eq!(s0, -1_047_575.125_984_918_3, "the touch is exact in f64");
    let (sa, sb) = (Span::new(&a, (t1 - 1.0, t1)), Span::new(&b, (s0, s0 + 1.0)));
    for (order, got) in [
        ("a first", meet(&sa, &sb, fine)),
        ("b first", meet(&sb, &sa, fine)),
    ] {
        if let Ok(got) = got {
            assert!(
                got.shared.is_some() || !got.points.is_empty(),
                "{order}: a touching pair read apart: {got:?}"
            );
        }
    }
}

/// **A fillet joint between two ellipses across their planes is read by
/// its window.** A rod's rim cut at 45°, the quarter of its ellipse from
/// `θ = 0.4636`, and a fillet's ellipse in a plane tilted `26.6°` ending
/// tangent to it at `(0.4724, −0.0106, 0.9447)`, a swept fillet's joint.
/// The window about the joint stays within the band of the other ellipse
/// over a long arc; there both the height above the other's plane and the
/// other's level grow away from the joint, so the window holds the joint
/// alone, whichever span is read first. Red where a window whose height
/// and level both recede is polished, which refuses
/// (`Unsettled::Meter`).
#[test]
fn a_fillet_joint_between_two_ellipses_is_read_by_its_window() {
    let h = core::f64::consts::FRAC_1_SQRT_2;
    let (c, s) = (0.894_427_190_999_916, 0.447_213_595_499_958);
    let rim = Curve3::Ellipse {
        center: Point3::new(0.561_803_398_874_989_5, -0.1, 0.9),
        axis: Vec3::new(h, h, 0.0),
        semi_u: 0.141_421_356_237_309_53,
        semi_v: 0.1,
        u_ref: Vec3::new(-h, h, 0.0),
    };
    let fillet = Curve3::Ellipse {
        center: Point3::new(0.45, -0.1, 0.9),
        axis: Vec3::new(c, 0.0, -s),
        semi_u: 0.111_803_398_874_989_48,
        semi_v: 0.1,
        u_ref: Vec3::new(s, 0.0, c),
    };
    let (sr, sf) = (
        (0.463_647_609_000_806_1, FRAC_PI_2),
        (-FRAC_PI_2, -1.107_148_717_794_090_2),
    );
    for (order, got) in [
        ("rim first", meets((&rim, sr), (&fillet, sf))),
        ("fillet first", meets((&fillet, sf), (&rim, sr))),
    ] {
        let got = got.unwrap_or_else(|e| panic!("{order}: {e:?}"));
        assert!(
            matches!(got.points[..], [p] if matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
            "{order}: the joint alone: {got:?}"
        );
    }
}

/// **A rim touching its fillet circle's plane meets it at the joint alone
/// at `ε = 10⁻¹²`.** The needle's rim, a rod of radius `0.1` cut so its
/// ellipse's semi-axes are `6.406` and `0.1`, the quarter of it from its
/// major vertex `J = (0.1, 0, 0)`, and a fillet's circle of radius `0.1`
/// in `z = 0` ending at `J` tangent to it, and one of radius `0.2` tangent
/// inside it there. The rim touches the circle's plane at `J`, so the
/// plane root's window stays within the zero band of the circle over
/// about `10⁶` rings; read on along the circle it holds the joint alone,
/// whichever span is read first. Red at that band where the run is read
/// by the first-order on readings alone: the window runs out of pieces
/// and the root's polish meets parallel tangents (`Unsettled::Meter`).
#[test]
fn a_rim_touching_its_fillet_circles_plane_meets_it_at_the_joint_alone_at_a_fine_band() {
    let fine = Band::new(1e-12, 1e-11).unwrap();
    let big = 6.406_f64;
    let (c, s) = (0.1 / big, (1.0 - (0.1 / big) * (0.1 / big)).sqrt());
    let u = Vec3::new(c, 0.0, s);
    let rim = Curve3::Ellipse {
        center: Point3::new(0.1, 0.0, 0.0) - u * big,
        axis: u.cross(Vec3::new(0.0, 1.0, 0.0)),
        semi_u: big,
        semi_v: 0.1,
        u_ref: u,
    };
    for radius in [0.1, 0.2] {
        let circle = Curve3::Circle {
            center: Point3::new(0.1 - radius, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let (sr, sc) = ((0.0, FRAC_PI_2), (-FRAC_PI_2, 0.0));
        for (order, got) in [
            (
                "rim first",
                meet(&Span::new(&rim, sr), &Span::new(&circle, sc), fine),
            ),
            (
                "circle first",
                meet(&Span::new(&circle, sc), &Span::new(&rim, sr), fine),
            ),
        ] {
            let got = got.unwrap_or_else(|e| panic!("radius {radius}, {order}: {e:?}"));
            assert!(
                matches!(got.points[..], [p] if matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
                "radius {radius}, {order}: the joint alone: {got:?}"
            );
        }
    }
}

/// **A height harmonic whose highest point lies on the negative `û`
/// axis is read clear of `atan2`'s cut.** A die's pip, as the corpus
/// builds it and in the enclosures it carries at interval: the pip's rim
/// of radius `0.0806` on the face `z = 1`, the half of it at `y ≥ 0.5`,
/// and the section arc of radius `0.09` about `(0.5, 0.5, 1.04)` in the
/// plane `y = 0.5`, from the face down. The rim's height above the
/// arc's plane has its cosine part `−r` and its sine part an enclosure
/// of zero, so the principal `atan2` reads the turn `[−π, π]`; the root
/// gap then reads `[0, 0.228]` and escalates
/// (`curve_pair_pose_root_gap`), where `f64` reads the two ends' joint.
/// Read a turn on, the angle is `π` to its rounding, and the pair meets
/// at the joint alone, whichever span is read first. Red where the
/// angle is read on the principal branch.
#[test]
fn a_height_harmonic_on_the_negative_axis_is_read_clear_of_the_cut() {
    use geom_core::Interval;
    let i = Interval::from_bounds;
    let p = |x, y, z| Point3::new(x, y, z);
    let v = |x, y, z| Vec3::new(x, y, z);
    let rim = Curve3::Circle {
        center: p(i(0.5, 0.5), i(0.5, 0.5), i(1.0, 1.0)),
        axis: v(i(-0.0, -0.0), i(-0.0, -0.0), i(-1.0, -1.0)),
        radius: i(0.080_622_577_482_985_46, 0.080_622_577_482_985_49),
        u_ref: v(
            i(-4e-323, 4e-323),
            i(0.999_999_999_999_997_2, 1.000_000_000_000_002_7),
            i(0.0, 0.0),
        ),
    };
    let rim_span = (
        i(-1.570_796_326_794_898_3, -1.570_796_326_794_894_3),
        i(1.570_796_326_794_890_3, 1.570_796_326_794_902_3),
    );
    let arc = Curve3::Circle {
        center: p(i(0.5, 0.5), i(0.5, 0.5), i(1.04, 1.04)),
        axis: v(
            i(-1.224_646_799_147_357_4e-16, 3.216_245_299_353_283e-16),
            i(-1.000_000_000_000_000_2, -0.999_999_999_999_998_9),
            i(-1.969_382_255_562_861_5e-32, 5.172_116_912_806_034e-32),
        ),
        radius: i(0.09, 0.09),
        u_ref: v(
            i(0.0, 0.0),
            i(0.0, 0.0),
            i(0.999_999_999_999_999, 1.000_000_000_000_001),
        ),
    };
    let arc_span = (
        i(2.031_350_318_476_216_3, 2.031_350_318_476_222_5),
        i(core::f64::consts::PI, core::f64::consts::PI),
    );
    let (r, a) = (Span::new(&rim, rim_span), Span::new(&arc, arc_span));
    for (order, got) in [
        ("rim first", meet(&r, &a, band())),
        ("arc first", meet(&a, &r, band())),
    ] {
        let got = got.unwrap_or_else(|e| panic!("{order}: {e:?}"));
        assert!(
            matches!(got.points[..], [p] if matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
            "{order}: the joint alone: {got:?}"
        );
    }
}

/// **A touch at a harmonic's lowest point is read clear of the
/// reduction's cut.** A die's pip as the composed tour builds it, in its
/// enclosures at interval: a circle of radius `0.02` about
/// `(0.8122, 0.02, 0.28)` in the plane `z = 0.28`, touching the plane
/// `y = 0` at its start, and the section arc of radius `0.0922` about
/// `(0.72, 0, 0.28)` in that plane, ending there. The circle's height
/// above the arc's plane touches zero at its lowest point, half a turn
/// from its highest, so the polish's reach reads the angle from the
/// highest point through a reduction whose cut lies exactly there: read
/// so, the reach is the whole half turn and the basin meter refuses
/// (`Unsettled::Meter`). Read from the lowest point, the reach is the
/// band's arc, and the pair meets at the joint alone, whichever span is
/// read first. Red where the reach is read across the cut.
#[test]
fn a_touch_at_a_harmonics_lowest_point_is_read_clear_of_the_cut() {
    use geom_core::Interval;
    let i = Interval::from_bounds;
    let p = |x, y, z| Point3::new(x, y, z);
    let v = |x, y, z| Vec3::new(x, y, z);
    let pip = Curve3::Circle {
        center: p(
            i(0.812_195_444_572_928_1, 0.812_195_444_572_929_5),
            i(0.019_999_999_999_999_98, 0.020_000_000_000_000_028),
            i(0.279_999_999_999_999_25, 0.280_000_000_000_000_75),
        ),
        axis: v(
            i(-9.663_752_709_139_367e-14, 9.332_596_074_246_75e-14),
            i(-7.216_449_660_066_244e-14, 7.216_449_660_066_244e-14),
            i(-1.000_000_000_000_377_3, -0.999_999_999_999_622_7),
        ),
        radius: i(0.02, 0.02),
        u_ref: v(
            i(-5.551_115_123_125_793e-14, 5.551_115_123_125_793e-14),
            i(-1.000_000_000_000_003_6, -0.999_999_999_999_996_4),
            i(-7.216_449_660_063_531e-14, 7.216_449_660_063_531e-14),
        ),
    };
    let pip_span = (
        i(0.0, 0.0),
        i(0.993_864_981_558_250_5, 0.993_864_981_558_627_5),
    );
    let arc = Curve3::Circle {
        center: p(
            i(0.72, 0.72),
            i(-3.469_446_951_953_614e-18, 1.040_834_085_586_084_3e-17),
            i(0.28, 0.28),
        ),
        axis: v(i(-0.0, -0.0), i(1.0, 1.0), i(-0.0, -0.0)),
        radius: i(0.092_195_444_572_928_8, 0.092_195_444_572_928_9),
        u_ref: v(
            i(-9.846_911_277_814_305e-33, 3.749_399_456_654_659e-33),
            i(0.0, 0.0),
            i(0.999_999_999_999_997, 1.000_000_000_000_002_9),
        ),
    };
    let arc_span = (
        i(-4.712_388_980_384_697, -4.712_388_980_384_683),
        i(1.570_796_326_794_889_5, 1.570_796_326_794_903_7),
    );
    let (c, a) = (Span::new(&pip, pip_span), Span::new(&arc, arc_span));
    for (order, got) in [
        ("pip first", meet(&c, &a, band())),
        ("arc first", meet(&a, &c, band())),
    ] {
        let got = got.unwrap_or_else(|e| panic!("{order}: {e:?}"));
        assert!(
            matches!(got.points[..], [p] if matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
            "{order}: the joint alone: {got:?}"
        );
    }
}

/// **A conic's extent on a line is its larger semi-axis.** A line along
/// an ellipse's major axis over `(1.5, 3)`, past its minor semi-axis,
/// crosses it at `x = 2` (the ellipse read over `(−π, π)`, so the
/// crossing is interior to both), in either order; so does a line across its
/// plane through that point. An extent read from the smaller semi-axis
/// would put the span a metre past it and read the pair apart.
#[test]
fn a_line_crossing_an_ellipse_past_its_minor_semi_axis_meets_it() {
    let e = ellipse([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 2.0, 0.5);
    let l = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let m = meets((&l, (1.5, 3.0)), (&e, (-PI, PI))).unwrap();
    assert!(interiors_meet(&m), "in plane: {m:?}");
    let m = meets((&e, (-PI, PI)), (&l, (1.5, 3.0))).unwrap();
    assert!(interiors_meet(&m), "in plane, ellipse first: {m:?}");
    let p = line([1.0, 0.0, -1.0], [1.0, 0.0, 1.0]);
    let m = meets((&p, (0.5, 2.5)), (&e, (-PI, PI))).unwrap();
    assert!(interiors_meet(&m), "across: {m:?}");
}

/// **The gap reads Apart in interval arithmetic too**: the leaning pair of
/// [`a_gap_along_one_line_past_the_band_meets_nowhere`], two metres
/// apart, each coordinate an interval a few ulps wide.
#[test]
fn two_segments_on_one_line_far_apart_read_apart_at_interval() {
    use geom_core::Interval;
    let i = |x: f64| Interval::from_bounds(x - 1e-15, x + 1e-15);
    let a = Curve3::Line {
        origin: Point3::new(i(0.0), i(0.0), i(0.0)),
        dir: Vec3::new(i(1.0), i(0.0), i(0.0)),
    };
    let b = Curve3::Line {
        origin: Point3::new(i(0.0), i(2.0 * ZERO), i(0.0)),
        dir: Vec3::new(i(1.0), i(ZERO), i(0.0)),
    };
    let got = pose(
        &Span::new(&a, (i(0.0), i(1.0))),
        &Span::new(&b, (i(3.0), i(4.0))),
        band(),
    );
    assert!(matches!(got, Ok(Pose::Apart)), "{got:?}");
}

/// **The gap is charged with the rounding its ends carry**: the
/// evaluation that produced them from the carrier, and the projections
/// at the spans' own lengths (`geom_core::one_line`, "The charge"). At
/// `ε = 10⁻¹²` a span along `x̂` from `(10⁴, 0, 0)` over `(0, 1)` and one
/// on the same line over `(1 + g, 2)`: a gap `g = 4·10⁻¹¹`, past the
/// escalation but within the rounding of ends evaluated ten kilometres
/// out (about `7.1·10⁻¹¹` charged, nearly all of it the evaluation), is
/// not read apart, and `g = 2·10⁻¹⁰` is.
/// Two spans on one carrier
/// through the origin, large by their parameter near 1586 m and
/// `9.8·10⁻¹²` apart, below the escalation, are not read apart either.
#[test]
fn the_gap_is_charged_with_the_rounding_of_far_coordinates() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let x = line([1e4, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let read = |g: f64| {
        pose(
            &Span::new(&x, (0.0, 1.0)),
            &Span::new(&x, (1.0 + g, 2.0)),
            band,
        )
    };
    assert!(
        !matches!(read(4e-11), Ok(Pose::Apart | Pose::Parallel)),
        "4e-11: {:?}",
        read(4e-11)
    );
    assert!(
        matches!(read(2e-10), Ok(Pose::Apart)),
        "2e-10: {:?}",
        read(2e-10)
    );
    let far = line([0.0, 0.0, 0.0], [-0.42383, -0.24814, 0.21293]);
    let t1 = 1_586.431_778_335_844_2;
    let s = 1_586.431_778_335_854;
    let got = pose(
        &Span::new(&far, (t1 - 1.0, t1)),
        &Span::new(&far, (s, s + 1.0)),
        band,
    );
    assert!(
        !matches!(got, Ok(Pose::Apart | Pose::Parallel)),
        "far by the parameter: {got:?}"
    );
}

/// **A spline with a corner meets a line across its second span.** The
/// degree-1 spline runs along `y = 0.28` to a corner just past its
/// middle, then straight down through `y = 0` to `y = −0.008`; the line
/// `y = 0` crosses that second span at `(0.012, 0, 0)`. Each span is
/// straight, so a bound read span by span is zero, though the tangent
/// turns a right angle at the knot: the crease bounds no `|C″|`.
#[test]
fn a_spline_with_a_corner_meets_a_line_past_it() {
    use geom::NurbsCurve3;
    use geom_core::spline::{KnotVector, Knots};
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.52, 1.0, 1.0], 1).unwrap();
    let a = Curve3::Nurbs(std::sync::Arc::new(
        NurbsCurve3::new(
            Knots::from(knots),
            vec![
                Point3::new(-0.3, 0.28, 0.0),
                Point3::new(0.012, 0.28, 0.0),
                Point3::new(0.012, -0.008, 0.0),
            ],
            vec![1.0; 3],
        )
        .unwrap(),
    ));
    let b = line([-0.3, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let got = super::walk::meet(
        &Span::new(&a, (0.0, 1.0)),
        &Span::new(&b, (0.0, 0.6)),
        band(),
    );
    let Ok(m) = got else {
        panic!("the walk reads the crossing: {got:?}")
    };
    assert!(
        m.points.len() == 1 && interiors_meet(&m),
        "one meeting, on both interiors: {m:?}"
    );
}
