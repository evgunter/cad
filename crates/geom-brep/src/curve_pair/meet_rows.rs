//! [`meet`]'s rows: the table's totality, each pose on a pose built to
//! reach it, the near-tangent and near-concentric circle pairs the
//! boolean's crossing layer misplaced
//! (`a-near-tangent-coplanar-circle-pair-places-its-touch-off-both-circles` (closed with CLEAVE: `docs/doc-ledger/cleave-leaves-the-tracker.md`)),
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
    Curve3::line(Point3::from_array(o), Vec3::from_array(d).normalize())
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
    Curve3::circle(Point3::from_array(c), axis, r, u_for(axis))
}

pub(super) fn ellipse(c: [f64; 3], axis: [f64; 3], u: [f64; 3], a: f64, b: f64) -> Curve3<f64> {
    let axis = Vec3::from_array(axis).normalize();
    let u = Vec3::from_array(u);
    Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(
            Point3::from_array(c),
            axis,
            (u - axis * u.dot(axis)).normalize(),
        ),
        a,
        b,
    ))
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
        Curve3::Line(geom::Line { origin, dir }) => {
            let w = q - origin;
            let u = dir.normalize();
            (w - u * w.dot(u)).norm()
        }
        Curve3::Circle(
            circle @ geom::Circle {
                frame:
                    geom::Frame {
                        origin: center,
                        axis,
                        ..
                    },
                ..
            },
        ) => {
            let radius = circle.radius();
            let w = q - center;
            let h = w.dot(axis);
            let rho = (w - axis * h).norm();
            ((rho - radius.abs()).powi(2) + h.powi(2)).sqrt()
        }
        Curve3::Ellipse(..) | Curve3::Spiric { .. } | Curve3::Nurbs(_) | Curve3::Periodic(_) => {
            panic!("no independent distance to a {:?}", c.kind())
        }
    }
}

/// A polished pair's places on its two spans, each read once with its
/// record held apart, as the walk's stop hands them to
/// [`super::Found::read_pair`].
fn places(
    a: &Span<'_, f64>,
    b: &Span<'_, f64>,
    c: super::Candidate<f64>,
) -> (super::PlaceRead, super::PlaceRead) {
    (
        super::PlaceRead::of(a, c.ta, band()),
        super::PlaceRead::of(b, c.tb, band()),
    )
}

/// **The table is total, and its closed arms are the line, circle and
/// ellipse pairs.** Every pair of kinds has a method; a pair is the
/// walk's exactly when a side is a spiric or a spline, clamped or
/// periodic; and the mirrored
/// pair has the mirrored arm.
#[test]
fn the_table_reads_every_kind_pair_once() {
    let walked = |k: CurveKind| {
        matches!(
            k,
            CurveKind::Spiric | CurveKind::Nurbs | CurveKind::Periodic
        )
    };
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

/// **Two circles cut from one sphere**, of radius `1.3` about
/// `(3.7, −2.1, 0.4)`, by two planes through `centre + (0, 0.5, 0)`, the
/// second turned by `theta` about the line there along `x`: the circles
/// meet on that line.
fn circles_on_one_sphere(theta: f64) -> (Curve3<f64>, Curve3<f64>) {
    let (centre, radius, h) = (Point3::new(3.7, -2.1, 0.4), 1.3, 0.5);
    let (s, c) = theta.sin_cos();
    let foot = |n: Vec3<f64>| {
        let d = Vec3::new(0.0, h, 0.0).dot(n);
        (centre + n * d, (radius * radius - d * d).sqrt())
    };
    let (n1, n2) = (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, c, s));
    let ((c1, r1), (c2, r2)) = (foot(n1), foot(n2));
    (
        circle(c1.to_array(), n1.to_array(), r1),
        circle(c2.to_array(), n2.to_array(), r2),
    )
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
    for (theta, decided) in [
        (1e-2_f64, true),
        (1e-3, true),
        (3e-4, true),
        (1e-4, false),
        (1e-5, false),
        (1e-7, false),
        (1e-11, true),
    ] {
        let (a, b) = circles_on_one_sphere(theta);
        let label = format!("tilt {theta:e}");
        let m = meets((&a, (0.05, TAU - 0.05)), (&b, (0.05, TAU - 0.05)));
        if !decided {
            assert!(
                matches!(m, Err(ref r) if matches!(r.fault, MeetFault::Unsettled(Unsettled::Meter { .. }))),
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
/// on both circles and meets inside both spans.
#[test]
fn a_decided_coplanar_touch_lies_on_both_circles() {
    let a = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    for (label, a, b, at, ta) in [
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
pub(super) fn flat_ellipse(at: [f64; 2], u: [f64; 2], a: f64, b: f64) -> Curve3<f64> {
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

/// **Ellipses crossing near a touch read one meeting where the arc
/// between their crossings lies in the zero band, and two where it does
/// not** (C5, `crates/geom-brep/README.md`: a meeting is a longest
/// interval of one span everywhere definitely on the other). Three
/// families, at ε = 1e-6, 1e-9 and 1e-12, at three turns of the plane, in
/// both orders:
/// - two equal `(2, 1)` ellipses, one pushed down so they cross twice
///   `s` apart, `s` from `0.1ε` to `10⁵ε`;
/// - `(2, 1)` and `(3, 1.5)` touching outside at `(0, 1)`, pushed
///   together likewise;
/// - an osculating pair (`(2, 1)` and `(√8, 2)` about `(0, −1)`, equal
///   curvature at the touch), pushed together, pulled apart, or touching.
///
/// Each crossing pair is judged against an oracle that shares no code
/// with the reader ([`super::order_rows::point_gap`], dense sampling and
/// golden-section refinement): the most distance from the second ellipse
/// over the arc of the first between the true crossings. No crossing
/// pair reads no meeting. One meeting reads only where that arc lies in
/// the zero band, and then on the arc and within the zero band of the
/// second; two read only where it does not, each within the zero band of
/// the second. A pair pulled apart past `10ε` reads none.
/// Red where a touch's in-band arc reads as two crossings, or two
/// crossings parted past the band as one.
#[test]
fn ellipses_crossing_near_a_touch_read_one_meeting_where_their_arc_is_in_band() {
    use super::order_rows::point_gap;
    let mut bad = Vec::new();
    let mut apart_met = Vec::new();
    let (mut touches, mut pairs) = (0, 0);
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for phi in [0.0f64, 0.7, 2.3] {
            let shift = if phi == 0.0 { [0.0, 0.0] } else { [3.1, -1.7] };
            let u = [phi.cos(), phi.sin()];
            let at = |x: f64, y: f64| {
                let t = turned_by([x, y], phi, shift);
                Point3::new(t[0], t[1], 0.0)
            };
            // The first ellipse `(2, 1)` about the origin, at its upper
            // points `±x`: the crossings, and the arc between them.
            let on_a = |x: f64| at(x, (1.0 - x * x / 4.0).sqrt());
            let mut judge = |label: String,
                             got: &Result<super::Meetings<f64>, super::MeetRefusal<f64>>,
                             b: &Curve3<f64>,
                             half: f64| {
                let Ok(m) = got else {
                    return;
                };
                if m.points.is_empty() {
                    bad.push(format!("{label}: crossing, no meeting"));
                    return;
                }
                let arc: Vec<Point3<f64>> = (0..=64)
                    .map(|i| on_a(-half + half * f64::from(i) / 32.0))
                    .collect();
                let most = arc
                    .iter()
                    .map(|&p| point_gap(p, b, (-PI, PI)))
                    .fold(0.0, f64::max);
                match m.points.len() {
                    1 => {
                        touches += 1;
                        let p = m.points[0].point;
                        let on_arc = arc
                            .iter()
                            .map(|&q| (p - q).norm())
                            .fold(f64::INFINITY, f64::min);
                        if most > band.zero() {
                            bad.push(format!(
                                "{label}: one meeting, its arc {most:e} from the other"
                            ));
                        }
                        if point_gap(p, b, (-PI, PI)) > band.zero() || on_arc > 10.0 * eps {
                            bad.push(format!("{label}: the touch {on_arc:e} off its arc"));
                        }
                    }
                    _ => {
                        pairs += 1;
                        if most <= band.zero() {
                            bad.push(format!("{label}: two meetings, their arc {most:e} in band"));
                        }
                        for p in &m.points {
                            let gap = point_gap(p.point, b, (-PI, PI));
                            if gap > band.zero() {
                                bad.push(format!("{label}: a meeting {gap:e} off the other"));
                            }
                        }
                    }
                }
            };
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
                let a = flat_ellipse(turned_by([0.0, 0.0], phi, shift), u, 2.0, 1.0);
                let b = flat_ellipse(turned_by([0.0, cy], phi, shift), u, 2.0, 1.0);
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
                        "eq eps {eps:e} phi {phi} s {:.3e} ({k}eps) {order}",
                        2.0 * half
                    );
                    judge(label, &got, &b, half);
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
                    judge(label, &got, &b, half);
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
                    let label = format!("osc eps {eps:e} phi {phi} dy {dy:e} {order}");
                    if delta > 0.0 {
                        // The crossing's `x`, where the heights of the
                        // two upper arcs agree, bisected.
                        let gap = |x: f64| {
                            (1.0 - x * x / 4.0).sqrt() - cy - 2.0 * (1.0 - x * x / 8.0).sqrt()
                        };
                        let (mut lo, mut hi) = (0.0, 1.9);
                        for _ in 0..200 {
                            let mid = 0.5 * (lo + hi);
                            if gap(mid) > 0.0 { lo = mid } else { hi = mid }
                        }
                        judge(label, &got, &b, 0.5 * (lo + hi));
                    } else if delta < -10.0 * eps && matches!(&got, Ok(m) if !m.points.is_empty()) {
                        apart_met.push(format!("{label}: apart, met"));
                    }
                }
            }
        }
    }
    bad.extend(apart_met);
    // Both claims are read: in-band arcs read as one touch, and crossings
    // parted past the band as two.
    assert!(touches > 0 && pairs > 0, "touches {touches}, pairs {pairs}");
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
pub(super) fn shallow_circles(tilt: f64, h: f64) -> [(Curve3<f64>, (f64, f64)); 2] {
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
        matches!(got, Err(ref r) if matches!(r.fault, MeetFault::Unsettled(Unsettled::Meter { .. }))),
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
pub(super) fn touching_circles(
    tilt: f64,
    h: f64,
    along: f64,
    angle: f64,
) -> [(Curve3<f64>, (f64, f64)); 2] {
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
    let mk = |c: [f64; 3], radius: f64| {
        Curve3::circle(Point3::from_array(c), Vec3::from_array(axis), radius, u)
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
    let arc = Curve3::circle(
        Point3::new(2.0, 0.25, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
        r,
        Vec3::new(0.0, -1.0, 0.0),
    );
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
    let edge = Curve3::line(
        Point3::new(-1.999_356_330_248_317_7, 0.535_723_727_818_681_7, 1.0),
        Vec3::new(0.965_926_090_343_448_7, -0.258_818_059_636_146_96, 0.0),
    );
    let boundary = Curve3::line(
        Point3::new(1.999_999_999_999_999_8, -0.535_898_384_862_245_2, 1.0),
        Vec3::new(-0.965_925_826_289_068_3, 0.258_819_045_102_520_7, 0.0),
    );
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
        bound: None,
    };
    let ring = super::Candidate {
        point: Point3::new(5e-9, 0.0, 0.0),
        ta: 5e-9,
        tb: 0.0,
        bound: None,
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
    let rim = Curve3::circle(
        p(
            i(0.199_999_999_999_999_76, 0.200_000_000_000_000_2),
            i(-0.346_410_161_513_776_7, -0.346_410_161_513_774_13),
            i(-1.615_374_500_829_602_8e-14, 1.604_272_270_583_351_2e-14),
        ),
        v(
            i(0.0, 0.0),
            i(0.0, 0.0),
            i(-1.000_000_000_000_032, -0.999_999_999_999_968_1),
        ),
        i(0.1, 0.1),
        v(
            i(0.499_999_999_999_959_2, 0.500_000_000_000_041),
            i(-0.866_025_403_784_513_9, -0.866_025_403_784_363_2),
            i(-3.236_300_116_782_472_7e-13, 3.236_300_116_782_472_7e-13),
        ),
    );
    let rim_span = (
        i(-1.047_197_551_196_718_9, -1.047_197_551_196_476),
        i(-6.525_890_938_746_67e-13, 6.539_213_615_042_172e-13),
    );
    let edge = Curve3::line(
        p(i(0.3, 0.3), i(-0.4, -0.4), i(0.0, 0.0)),
        v(
            i(0.0, 0.0),
            i(0.999_999_999_999_999_7, 1.000_000_000_000_000_4),
            i(0.0, 0.0),
        ),
    );
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
    let rim = Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(
            Point3::new(x, -0.1, 0.9),
            Vec3::new(h, h, 0.0),
            Vec3::new(-h, h, 0.0),
        ),
        0.141_421_356_237_309_53,
        0.1,
    ));
    let fillet = Curve3::circle(
        Point3::new(x, 0.0, 0.9),
        Vec3::new(0.0, 1.0, 0.0),
        0.1,
        Vec3::new(0.0, 0.0, 1.0),
    );
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
        found.read(
            &whole,
            &seg,
            super::Candidate {
                point,
                ta,
                tb,
                bound: None,
            },
            band(),
        );
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
        found.read(
            &sc,
            &se,
            super::Candidate {
                point,
                ta,
                tb,
                bound: None,
            },
            band(),
        );
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
    let a = Curve3::line(Point3::origin() - d * far, d);
    let b = Curve3::line(Point3::origin() + d * far, d);
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
    let rim = Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(
            Point3::new(0.561_803_398_874_989_5, -0.1, 0.9),
            Vec3::new(h, h, 0.0),
            Vec3::new(-h, h, 0.0),
        ),
        0.141_421_356_237_309_53,
        0.1,
    ));
    let fillet = Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(
            Point3::new(0.45, -0.1, 0.9),
            Vec3::new(c, 0.0, -s),
            Vec3::new(s, 0.0, c),
        ),
        0.111_803_398_874_989_48,
        0.1,
    ));
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
    let rim = Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(
            Point3::new(0.1, 0.0, 0.0) - u * big,
            u.cross(Vec3::new(0.0, 1.0, 0.0)),
            u,
        ),
        big,
        0.1,
    ));
    for radius in [0.1, 0.2] {
        let circle = Curve3::circle(
            Point3::new(0.1 - radius, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            radius,
            Vec3::new(1.0, 0.0, 0.0),
        );
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
    let rim = Curve3::circle(
        p(i(0.5, 0.5), i(0.5, 0.5), i(1.0, 1.0)),
        v(i(-0.0, -0.0), i(-0.0, -0.0), i(-1.0, -1.0)),
        i(0.080_622_577_482_985_46, 0.080_622_577_482_985_49),
        v(
            i(-4e-323, 4e-323),
            i(0.999_999_999_999_997_2, 1.000_000_000_000_002_7),
            i(0.0, 0.0),
        ),
    );
    let rim_span = (
        i(-1.570_796_326_794_898_3, -1.570_796_326_794_894_3),
        i(1.570_796_326_794_890_3, 1.570_796_326_794_902_3),
    );
    let arc = Curve3::circle(
        p(i(0.5, 0.5), i(0.5, 0.5), i(1.04, 1.04)),
        v(
            i(-1.224_646_799_147_357_4e-16, 3.216_245_299_353_283e-16),
            i(-1.000_000_000_000_000_2, -0.999_999_999_999_998_9),
            i(-1.969_382_255_562_861_5e-32, 5.172_116_912_806_034e-32),
        ),
        i(0.09, 0.09),
        v(
            i(0.0, 0.0),
            i(0.0, 0.0),
            i(0.999_999_999_999_999, 1.000_000_000_000_001),
        ),
    );
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
    let pip = Curve3::circle(
        p(
            i(0.812_195_444_572_928_1, 0.812_195_444_572_929_5),
            i(0.019_999_999_999_999_98, 0.020_000_000_000_000_028),
            i(0.279_999_999_999_999_25, 0.280_000_000_000_000_75),
        ),
        v(
            i(-9.663_752_709_139_367e-14, 9.332_596_074_246_75e-14),
            i(-7.216_449_660_066_244e-14, 7.216_449_660_066_244e-14),
            i(-1.000_000_000_000_377_3, -0.999_999_999_999_622_7),
        ),
        i(0.02, 0.02),
        v(
            i(-5.551_115_123_125_793e-14, 5.551_115_123_125_793e-14),
            i(-1.000_000_000_000_003_6, -0.999_999_999_999_996_4),
            i(-7.216_449_660_063_531e-14, 7.216_449_660_063_531e-14),
        ),
    );
    let pip_span = (
        i(0.0, 0.0),
        i(0.993_864_981_558_250_5, 0.993_864_981_558_627_5),
    );
    let arc = Curve3::circle(
        p(
            i(0.72, 0.72),
            i(-3.469_446_951_953_614e-18, 1.040_834_085_586_084_3e-17),
            i(0.28, 0.28),
        ),
        v(i(-0.0, -0.0), i(1.0, 1.0), i(-0.0, -0.0)),
        i(0.092_195_444_572_928_8, 0.092_195_444_572_928_9),
        v(
            i(-9.846_911_277_814_305e-33, 3.749_399_456_654_659e-33),
            i(0.0, 0.0),
            i(0.999_999_999_999_997, 1.000_000_000_000_002_9),
        ),
    );
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
    let a = Curve3::line(
        Point3::new(i(0.0), i(0.0), i(0.0)),
        Vec3::new(i(1.0), i(0.0), i(0.0)),
    );
    let b = Curve3::line(
        Point3::new(i(0.0), i(2.0 * ZERO), i(0.0)),
        Vec3::new(i(1.0), i(ZERO), i(0.0)),
    );
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
/// turns a right angle at the knot: the crease bounds no `|C″|` over the
/// whole carrier. The walk's window reads the curvature over its own
/// arcs (`walk::Read::curvature_near`), which lie inside the second span
/// far from the crease, so every window the walk asks proves its basin
/// (`curve_pair_walk_basin`). Read over the whole carrier, the basin
/// proves nowhere on it.
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
    let (got, rec) = geom_core::k_stats::detached(|| {
        super::walk::meet(
            &Span::new(&a, (0.0, 1.0)),
            &Span::new(&b, (0.0, 0.6)),
            band(),
        )
    });
    let Ok(m) = got else {
        panic!("the walk reads the crossing: {got:?}")
    };
    assert!(
        m.points.len() == 1 && interiors_meet(&m),
        "one meeting, on both interiors: {m:?}"
    );
    let basin = |p: &str| p == "curve_pair_walk_basin";
    let signs: Vec<geom_core::Sign> = rec
        .recorded()
        .verdicts
        .iter()
        .filter(|v| basin(v.predicate))
        .map(|v| v.sign)
        .collect();
    assert!(
        !signs.is_empty() && signs.iter().all(|&s| s == geom_core::Sign::Positive),
        "every window the walk asks proves its basin: {signs:?}"
    );
    assert!(
        !rec.recorded()
            .escalations
            .iter()
            .any(|e| basin(e.predicate())),
        "no basin is left undecided: {:?}",
        rec.recorded().escalations
    );
}

/// The verdicts `rec` holds on `predicate`, in order.
fn verdicts_on(rec: &geom_core::k_stats::Detached, predicate: &str) -> Vec<geom_core::Sign> {
    rec.recorded()
        .verdicts
        .iter()
        .filter(|v| v.predicate == predicate)
        .map(|v| v.sign)
        .collect()
}

/// **Two splines joined end to end read their joint's window one-sided.**
/// Two straight degree-2 splines run along `y = 0`, the first from
/// `(−0.3, 0, 0)` to the joint at the origin and the second on from there
/// to `(0.3, 0, 0)`, each with its middle control point off the chord's
/// middle so that its parameter is not arc length. At the joint the
/// tangents are parallel, so the two-sided sine is zero and the basin
/// (`curve_pair_walk_basin`) cannot prove. The polished pair is held at
/// the first span's last end and the second's first, and the
/// half-tangents pointing into the spans part by `180°`: the one-sided
/// window's sine is `1`, and its basin (`curve_pair_walk_basin_held`)
/// proves at every pair held so.
#[test]
fn two_splines_joined_end_to_end_read_their_joint_one_sided() {
    use geom::NurbsCurve3;
    use geom_core::spline::{KnotVector, Knots};
    let straight = |from: f64, to: f64| {
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        Curve3::Nurbs(std::sync::Arc::new(
            NurbsCurve3::new(
                Knots::from(knots),
                vec![
                    Point3::new(from, 0.0, 0.0),
                    Point3::new(from + (to - from) * 0.3, 0.0, 0.0),
                    Point3::new(to, 0.0, 0.0),
                ],
                vec![1.0; 3],
            )
            .unwrap(),
        ))
    };
    let (a, b) = (straight(-0.3, 0.0), straight(0.0, 0.3));
    let (got, rec) = geom_core::k_stats::detached(|| {
        super::walk::meet(
            &Span::new(&a, (0.0, 1.0)),
            &Span::new(&b, (0.0, 1.0)),
            band(),
        )
    });
    let Ok(m) = got else {
        panic!("the walk reads the joint: {got:?}")
    };
    assert!(m.points.len() == 1, "one meeting, the joint: {m:?}");
    let held = verdicts_on(&rec, "curve_pair_walk_basin_held");
    assert!(
        !held.is_empty() && held.iter().all(|&s| s == geom_core::Sign::Positive),
        "every pair held at the joint proves its one-sided basin: {held:?}"
    );
    assert!(
        !verdicts_on(&rec, "curve_pair_walk_basin").contains(&geom_core::Sign::Positive),
        "the two-sided basin proves nowhere at the joint"
    );
}

/// **Circles on one sphere meet at the certified scalar where they meet
/// at `f64`**: the pair of [`circles_on_one_sphere_meet_on_both_at_any_tilt`]
/// at every tilt `f64` decides above the basin floor (`1e-2`, `1e-3`,
/// `3e-4`), lifted to point intervals, at the band `(1e-12, 1e-11)`. A
/// polish stepped from a whole enclosure grows by the box's spread each
/// step; and a meter that compares where along the carriers the pair
/// sits with the zero band reads the data's spread over the sine between
/// the tangents. Main refused tilt `1e-3` on its convergence meter
/// (`curve_pair_pose_polish`, step `[0, 2e-12]`); the window's bound
/// reads metres of gap, and decides every tilt here.
#[test]
fn circles_on_one_sphere_meet_at_the_certified_scalar() {
    use geom_core::{Interval, Real};
    let band = Band::new(1e-12, 1e-11).unwrap();
    let i = Interval::from_f64;
    let lift = |c: &Curve3<f64>| {
        let Curve3::Circle(conic_k @ geom::Circle { frame, .. }) = *c else {
            unreachable!("a circle")
        };
        let radius = conic_k.radius();
        let v = |v: Vec3<f64>| Vec3::new(i(v.x), i(v.y), i(v.z));
        let o = frame.origin;
        Curve3::circle(
            Point3::new(i(o.x), i(o.y), i(o.z)),
            v(frame.axis),
            i(radius),
            v(frame.u_ref),
        )
    };
    let mid = |x: Interval| geom::bracket_mid((geom_core::Bounds::lo(x), geom_core::Bounds::hi(x)));
    for tilt in [1e-2, 1e-3, 3e-4] {
        let (a, b) = circles_on_one_sphere(tilt);
        let span = (0.05, TAU - 0.05);
        let at_f64 = meet(&Span::new(&a, span), &Span::new(&b, span), band).unwrap();
        let (a, b) = (lift(&a), lift(&b));
        let span = (i(span.0), i(span.1));
        let m = meet(&Span::new(&a, span), &Span::new(&b, span), band)
            .unwrap_or_else(|e| panic!("tilt {tilt:e} at Interval: {:?}", e.fault));
        assert!(
            !m.points.is_empty() && m.points.len() == at_f64.points.len(),
            "tilt {tilt:e}: the meetings f64 reads, {at_f64:?}, at Interval: {m:?}"
        );
        // Each Interval meeting is one f64 reads: its middle within the
        // band's escalation of an f64 meeting.
        for p in &m.points {
            let q = Point3::new(mid(p.point.x), mid(p.point.y), mid(p.point.z));
            let near = at_f64
                .points
                .iter()
                .map(|f| (f.point - q).norm())
                .fold(f64::INFINITY, f64::min);
            assert!(
                near <= band.escalate(),
                "tilt {tilt:e}: an Interval meeting at {q:?} lies {near:e} from every f64 \
                 meeting: {at_f64:?}"
            );
        }
    }
}

/// **The walk offers an endless span no end reads.** A whole circle read
/// with no ends ([`Span::endless`]) has none to offer ([`Span::ends`]),
/// and its seam is a point of its interior: a line crossing the circle
/// square at the seam meets it once, on the circle's interior, by the
/// walk as by the closed forms, and a line clear of the circle not at
/// all. Read as ends, the seam's point would be offered as an end of
/// the circle, which it is not.
#[test]
fn the_walk_offers_an_endless_span_no_end_reads() {
    let c = circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
    let whole = Span::new(&c, (0.0, TAU)).endless();
    assert_eq!(whole.ends().count(), 0, "an endless span has no ends");
    let across = line([1.0, 0.0, -0.5], [0.0, 0.0, 1.0]);
    let clear = line([2.0, 0.0, -0.5], [0.0, 0.0, 1.0]);
    let (seg, far) = (
        Span::new(&across, (0.0, 1.0)),
        Span::new(&clear, (0.0, 1.0)),
    );
    for (order, walked, closed) in [
        (
            "circle first",
            super::walk::meet(&whole, &seg, band()),
            meet(&whole, &seg, band()),
        ),
        (
            "line first",
            super::walk::meet(&seg, &whole, band()),
            meet(&seg, &whole, band()),
        ),
    ] {
        let (Ok(walked), Ok(closed)) = (walked, closed) else {
            panic!("{order}: both decide")
        };
        assert!(
            walked.points.len() == 1 && closed.points.len() == 1 && interiors_meet(&walked),
            "{order}: one meeting, on both interiors: walked {walked:?}, closed {closed:?}"
        );
    }
    let walked = super::walk::meet(&whole, &far, band());
    assert!(
        matches!(&walked, Ok(m) if m.points.is_empty() && m.shared.is_none()),
        "clear of the circle: {walked:?}"
    );
}

/// **A window read off short of its reach escalates; its point read with
/// no window, or past a span by more than its reach, does not.** The
/// axes of `x` and `y` cross at the origin. A polished pair left `10⁻⁶`
/// apart in their plane, `p = (10⁻⁶, 0, 0)` on the first and the origin
/// on the second, states a window ([`super::polish::bound`]) whose floor
/// is zero (the gap lies along the carriers) and whose reach is about
/// `10⁻⁶`. Read as its pair ([`super::Found::read_pair`]), its gap is
/// past the band, so with no window it is no meeting; with its window it
/// straddles the floor against the gap at the pair
/// (`curve_pair_meet_window`) and escalates, never prunes. With the
/// second span moved past the origin by far more than the reach, the
/// window's whole run lies past that span and it prunes.
#[test]
fn a_window_read_off_short_of_its_reach_escalates() {
    let (x, y) = (
        line([0.0; 3], [1.0, 0.0, 0.0]),
        line([0.0; 3], [0.0, 1.0, 0.0]),
    );
    let (p, q) = (Point3::new(1e-6, 0.0, 0.0), Point3::origin());
    let Some(bound) = super::polish::bound(
        ("test_basin", None),
        super::polish::Along::two_sided(
            (p, Vec3::new(1.0, 0.0, 0.0)),
            (q, Vec3::new(0.0, 1.0, 0.0)),
        ),
        0.0,
        (band().escalate(), 0.1),
    ) else {
        panic!("square lines hold the basin")
    };
    assert!(
        bound.floor == 0.0 && (bound.reach - (1e-6 + band().escalate())).abs() < 1e-15,
        "{bound:?}"
    );
    let candidate = |bound| super::Candidate {
        point: p,
        ta: 1e-6,
        tb: 0.0,
        bound,
    };
    let read = |b: &Span<'_, f64>, bound| {
        let a = Span::new(&x, (-1.0, 1.0));
        let mut found = super::Found::default();
        let c = candidate(bound);
        found.read_pair(&a, b, c, places(&a, b, c), band());
        found.finish(&a, b, None, band())
    };
    let across = Span::new(&y, (-1.0, 1.0));
    let got = read(&across, None);
    assert!(
        matches!(&got, Ok(m) if m.points.is_empty()),
        "its pair alone: {got:?}"
    );
    let got = read(&across, Some(bound));
    assert!(
        matches!(&got, Err(r) if matches!(r.fault,
            MeetFault::Escalated(diag) if diag.predicate == Some("curve_pair_meet_window"))),
        "its window: {got:?}"
    );
    let past = Span::new(&y, (0.5, 1.0));
    let got = read(&past, Some(bound));
    assert!(
        matches!(&got, Ok(m) if m.points.is_empty()),
        "its window past the span: {got:?}"
    );
}

/// **A candidate with no window reads as its point alone, bit for bit**:
/// each end of two pairs (two segments, a circle's arc and a line), read
/// through [`super::Found::read`], gives the meetings, the refusal and
/// the logged decisions that reading its point on both spans gives, the
/// read before windows were stated. Ends, witnesses and exact roots state
/// no window, so this is their read.
#[test]
fn a_candidate_with_no_window_reads_as_its_point_alone() {
    use geom_core::k_stats::detached;
    let pairs = [
        (
            line([0.0; 3], [1.0, 0.0, 0.0]),
            (-1.0, 1.0),
            line([0.0; 3], [0.0, 1.0, 0.0]),
            (0.0, 1.0),
        ),
        (
            circle([0.0; 3], [0.0, 0.0, 1.0], 1.0),
            (0.2, 2.0),
            line([0.5, -2.0, 0.0], [0.0, 1.0, 0.0]),
            (0.0, 4.0),
        ),
    ];
    for (ca, ra, cb, rb) in &pairs {
        let (a, b) = (Span::new(ca, *ra), Span::new(cb, *rb));
        for c in super::pose::end_candidates(&a, &b) {
            assert!(c.bound.is_none(), "an end states no window");
            let (via, rec_via) = detached(|| {
                let mut found = super::Found::default();
                found.read(&a, &b, c, band());
                format!("{:?}", found.finish(&a, &b, None, band()))
            });
            let (alone, rec_alone) = detached(|| {
                let mut found = super::Found::default();
                found.offer(
                    c.point,
                    || {
                        super::both(
                            a.contact(c.point, c.ta, band()).map_err(|r| r.refused()),
                            b.contact(c.point, c.tb, band()).map_err(|r| r.refused()),
                        )
                    },
                    band(),
                );
                format!("{:?}", found.finish(&a, &b, None, band()))
            });
            assert_eq!(via, alone, "{c:?}");
            assert_eq!(
                format!("{:?}", rec_via.recorded()),
                format!("{:?}", rec_alone.recorded()),
                "{c:?}"
            );
        }
    }
}

/// **A keel's wall past a bore's rim is cleared by its held end's
/// floor, with main's answer.** A line down the keel's wall crosses the
/// plane of the bore's rim at the rim's own end, `2.2143` rad, and just
/// past it (`a_keel_on_a_bores_ruling`'s near-tangent pose, as `meet` met
/// it). The polish is held to the spans, so the pair stops at the rim's
/// end, `1.7·10⁻⁸` from the line, past the band's escalation. The gap
/// there runs along the rim's tangent, so the two-sided floor across both
/// tangents is nothing; but every pair of the window has the rim's side
/// on the arc into its span, which runs away from the line, so the floor
/// read across the line's tangent with the rim's arc a half-line is the
/// whole gap less the bend (`pose`'s module docs, "A held end's floor"):
/// no meeting, as main read it on its convergence meter and then by the
/// run arm. With no floor, the same pair escalates.
#[test]
fn a_keel_wall_past_a_bores_rim_is_cleared_by_its_held_ends_floor() {
    let wall = Curve3::line(
        Point3::new(1.0, -1.710_783_078_032_602_8e-8, 2.75),
        Vec3::new(0.0, 0.0, -1.0),
    );
    let rim = Curve3::circle(
        Point3::new(1.037_500_008_874_687_3, -1.774_937_458_641_49e-8, 2.0),
        Vec3::new(0.0, 0.0, 1.0),
        0.037_500_008_874_691_515,
        Vec3::new(0.599_999_621_346_698_8, 0.800_000_283_989_836, 0.0),
    );
    let (a, b) = (
        Span::new(&wall, (-1.25, 0.75)),
        Span::new(&rim, (0.0, 2.214_296_488_955_094_7)),
    );
    let got = meet(&a, &b, band());
    assert!(
        matches!(&got, Ok(m) if m.points.is_empty() && m.shared.is_none()),
        "no meeting: {got:?}"
    );
    let Ok(Pose::Generic(cs)) = pose(&a, &b, band()) else {
        panic!("a crossing of the rim's plane")
    };
    let Some(c) = cs.into_iter().find(|c| c.bound.is_some()) else {
        panic!("a polished candidate")
    };
    assert_eq!(
        c.tb, 2.214_296_488_955_094_7,
        "held at the rim's end: {c:?}"
    );
    let no_floor = super::Candidate {
        bound: c.bound.map(|w| super::WindowBound {
            floor: f64::NEG_INFINITY,
            ..w
        }),
        ..c
    };
    let mut found = super::Found::default();
    super::posed(&mut found, &a, &b, no_floor, band());
    let got = found.finish(&a, &b, None, band());
    assert!(
        matches!(&got, Err(r) if matches!(r.fault, MeetFault::Escalated(_))),
        "with no floor: {got:?}"
    );
}

/// **The run arm needs the span's end decidedly off the other carrier.**
/// A segment of `x` from the origin, and a line crossing `x` at
/// `X = (−2·10⁻⁸, 0, 0)`, past the segment's start by twice the band's
/// escalation, at an angle `θ`. The polished pair is `X`, on both
/// carriers; the window's reach, `e/sin θ`, is longer than `X`'s chord to
/// the start, so the reach clears nothing. At `sin θ = 0.25` the start
/// lies `5·10⁻⁹` from the line, in the band's gap: the run may hold it,
/// and the window escalates. At `θ = 60°` it lies `1.7·10⁻⁸` off, past
/// the escalation and twice the bend (zero on lines), the run crosses no
/// end, and the window is no meeting; with no bend bounded it escalates.
#[test]
fn the_run_arm_needs_the_span_end_decidedly_off_the_other_carrier() {
    let x = line([0.0; 3], [1.0, 0.0, 0.0]);
    let at = Point3::new(-2e-8, 0.0, 0.0);
    let read = |sine: f64, bent: bool| {
        let cosine = (1.0 - sine * sine).sqrt();
        let dir = Vec3::new(cosine, sine, 0.0);
        let other = Curve3::line(at, dir);
        let (a, b) = (Span::new(&x, (0.0, 1.0)), Span::new(&other, (-1.0, 1.0)));
        let Some(bound) = super::polish::bound(
            ("test_basin", None),
            super::polish::Along::two_sided((at, Vec3::new(1.0, 0.0, 0.0)), (at, dir)),
            0.0,
            (band().escalate(), 0.1),
        ) else {
            panic!("lines hold the basin")
        };
        let bound = if bent {
            bound
        } else {
            super::WindowBound {
                bend: f64::INFINITY,
                ..bound
            }
        };
        let mut found = super::Found::default();
        let c = super::Candidate {
            point: at,
            ta: -2e-8,
            tb: 0.0,
            bound: Some(bound),
        };
        found.read_pair(&a, &b, c, places(&a, &b, c), band());
        found.finish(&a, &b, None, band())
    };
    let escalated = |got: &Result<Meetings<f64>, super::MeetRefusal<f64>>| matches!(got, Err(r) if matches!(r.fault, MeetFault::Escalated(_)));
    let got = read(0.25, true);
    assert!(escalated(&got), "the start in the band's gap: {got:?}");
    let got = read(0.75_f64.sqrt(), true);
    assert!(
        matches!(&got, Ok(m) if m.points.is_empty()),
        "the start decidedly off: {got:?}"
    );
    let got = read(0.75_f64.sqrt(), false);
    assert!(escalated(&got), "with no bend bounded: {got:?}");
}

/// **A line leaving an arc's end nearly along its tangent meets it there
/// by the one-sided window.** A unit circle's arc ending at `X = (1, 0,
/// 0)`, and a segment from `X` at `10⁻⁴` rad above the arc's tangent,
/// running on the other way from the arc. The line crosses the circle's
/// plane at `X`, where the pose polishes it: the pair is `X` on both,
/// held at both spans' ends. The tangents part by `10⁻⁴`, so the
/// two-sided basin, `sin φ` less twice the turn over the window, is not
/// proved; but every pair of the window lies along the half-tangents
/// into the spans, which part by nearly `180°`, and the one-sided window
/// bounds its reach (`pose`'s module docs, "The one-sided window"). The
/// ends read first found the meeting at `X`, which rings the pair: one
/// meeting, at both ends. Read two-sided alone, the pose refuses on its
/// basin.
#[test]
fn a_line_leaving_an_arcs_end_along_its_tangent_meets_there_by_the_one_sided_window() {
    let tilt: f64 = 1e-4;
    let arc = circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
    let end = arc.eval(0.0);
    let leaving = line(end.to_array(), [0.0, tilt.cos(), tilt.sin()]);
    let (a, b) = (
        Span::new(&leaving, (0.0, 1.0)),
        Span::new(&arc, (-1.0, 0.0)),
    );
    let m = meet(&a, &b, band()).unwrap();
    assert!(
        matches!(m.points[..], [p] if matches!((p.on_a, p.on_b), (Hit::End(_), Hit::End(_)))),
        "one meeting at both ends: {m:?}"
    );
}

/// **A crossing off an arc's span reads no window.** A unit circle's arc
/// over `(0.2, 0.8)` rad, and a segment square to the circle's plane
/// through the circle at `1.4` rad, `0.6` rad past the arc's end, about
/// `0.59` m from it. The ball of `ℓ + e = 2·10⁻⁸` m about the crossing is
/// clear of the arc by its walk (`Span::clear_by`), so the
/// window, the region on the spans, holds no pair, and the pose reads
/// none (`curve_carrier_piece_clear_by`): no meeting. Read
/// with the region on the arc, the polish holds the pair at the arc's
/// end `0.6` m from the crossing, where the basin, `sin φ` less twice the
/// turn over that run, is not proved, and the pose refuses.
#[test]
fn a_crossing_off_an_arcs_span_reads_no_window() {
    let arc = circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
    let through = arc.eval(1.4);
    let square = line(through.to_array(), [0.0, 0.0, 1.0]);
    let (a, b) = (Span::new(&arc, (0.2, 0.8)), Span::new(&square, (-1.0, 1.0)));
    for (a, b) in [(&a, &b), (&b, &a)] {
        let m = meet(a, b, band()).unwrap();
        assert!(m.points.is_empty(), "no meeting: {m:?}");
    }
}

/// The full review's case for the off-span guard: a circle of radius
/// `r = 10⁻⁴` m about the origin in the `z = 0` plane, its arc `(π − 0.3,
/// π + 0.3)` about `(−r, 0, 0)`, and a segment crossing the plane at
/// `(−r + d, 0, 0)` and rising `e/ℓ` per metre towards `−x`, so it passes
/// `e·d/ℓ` over `(−r, 0, 0)`, `e` the band's escalation. The crossing's
/// foot on the circle is `(r, 0, 0)`, `2.84` rad of arc (`2.84·10⁻⁴` m)
/// from the arc. In metres the arc's end is `1.14r` from the crossing,
/// inside the ball of `ℓ + e` about it.
fn over_an_arcs_far_side(
    (d, ell): (f64, f64),
    band: Band,
) -> [Result<Meetings<f64>, super::MeetRefusal<f64>>; 2] {
    let r = 1e-4;
    let rise = band.escalate() / ell;
    let arc = circle([0.0; 3], [0.0, 0.0, 1.0], r);
    let over = line([-r + d, 0.0, 0.0], [-1.0, 0.0, rise]);
    let (a, b) = (
        Span::new(&over, (-3.0 * r, 3.0 * r)),
        Span::new(&arc, (PI - 0.3, PI + 0.3)),
    );
    [meet(&a, &b, band), meet(&b, &a, band)]
}

/// **A segment over an arc's far side, inside the zero band, is never
/// read as no meeting.** The full review's case
/// ([`over_an_arcs_far_side`], `d = 1.15r`, `ℓ = 1.3r`) at the bands `(10⁻⁹,
/// 1.05·10⁻⁹)` and `(10⁻⁹, 1.2·10⁻⁹)`: the segment passes `9.29·10⁻¹⁰` m
/// (and `1.06·10⁻⁹` m) over the arc's middle, so the pair meets or the
/// pose refuses; it never reads no meeting. Red where the off-span guard
/// reads the start's parameter distance at the least speed, `2.84·10⁻⁴`
/// m, against the region, `2.6·10⁻⁴` m: in metres the arc is `1.14r` from
/// the crossing, inside the ball of `ℓ + e` (`Span::clear_by`).
#[test]
fn a_segment_over_an_arcs_far_side_in_the_zero_band_is_never_no_meeting() {
    let r = 1e-4;
    for escalate in [1.05e-9, 1.2e-9] {
        let band = Band::new(1e-9, escalate).unwrap();
        let [first, second] = over_an_arcs_far_side((1.15 * r, 1.3 * r), band);
        for (order, got) in [("segment first", first), ("arc first", second)] {
            assert!(
                !matches!(&got, Ok(m) if m.points.is_empty()),
                "escalate {escalate}, {order}: no meeting read: {got:?}"
            );
        }
    }
}

/// **A segment over an arc's far side inside the escalation band refuses
/// or meets.** [`over_an_arcs_far_side`] at the shipping band `(10⁻⁹,
/// 10⁻⁸)`, `d = 1.1r` and `ℓ = 1.3r`: the segment passes `8.46·10⁻⁹` m
/// over the arc's middle, between the zero and the escalation, so the
/// pair never reads no meeting. Red under the same guard as
/// [`a_segment_over_an_arcs_far_side_in_the_zero_band_is_never_no_meeting`].
#[test]
fn a_segment_over_an_arcs_far_side_in_the_escalation_band_is_never_no_meeting() {
    let r = 1e-4;
    let [first, second] = over_an_arcs_far_side((1.1 * r, 1.3 * r), band());
    for (order, got) in [("segment first", first), ("arc first", second)] {
        assert!(
            !matches!(&got, Ok(m) if m.points.is_empty()),
            "{order}: no meeting read: {got:?}"
        );
    }
}

/// **A crossing whose foot on a thin ellipse lies far from it is read by
/// the ball about the crossing.** The meet oracle's draw 1721 (seed
/// `0x601115d67dc12df3`, effort 20, the shallow family): a segment
/// passing within the zero band of an ellipse of semi-axes `0.369` and
/// `0.0017`. The crossing's foot on the ellipse, where the polish starts,
/// is at `0.416` rad, `1.36` rad past the ellipse's span; the ball of the
/// window about that foot is clear of the span, but the meeting lies in
/// the ball of `ℓ + e` about the crossing, which is not. The pair meets
/// or the pose refuses, in either order; it never reads no meeting. Red
/// where the guard reads the ball about the second carrier's start.
#[test]
fn a_crossing_whose_foot_on_a_thin_ellipse_is_far_is_read_by_the_ball_about_it() {
    let segment = Curve3::line(
        Point3::new(
            -2.039_633_426_038_924,
            -0.995_972_943_460_762_7,
            -1.860_142_371_212_321_5,
        ),
        Vec3::new(
            -0.887_113_993_102_562,
            -0.019_566_488_067_895_276,
            -0.461_135_463_596_454_13,
        ),
    );
    let thin = Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(
            Point3::new(
                -0.828_141_776_510_814_9,
                -1.080_527_770_428_637_1,
                -1.220_265_462_034_935_4,
            ),
            Vec3::new(
                0.461_126_207_144_500_64,
                -0.080_406_063_711_237_68,
                -0.883_684_041_953_339_4,
            ),
            Vec3::new(
                -0.406_828_854_691_622_94,
                0.865_888_628_586_523_3,
                -0.291_079_312_000_788,
            ),
        ),
        0.368_635_757_026_020_3,
        0.001_683_987_557_222_959_5,
    ));
    let (a, b) = (
        Span::new(
            &segment,
            (-1.857_733_710_570_704_8, -0.093_244_569_165_410_02),
        ),
        Span::new(&thin, (-1.775_551_102_998_633_3, -0.942_491_638_652_538_3)),
    );
    for got in [meet(&a, &b, band()), meet(&b, &a, band())] {
        assert!(
            !matches!(&got, Ok(m) if m.points.is_empty()),
            "no meeting read: {got:?}"
        );
    }
}

/// **A shallow segment stopping short of a circle under its carrier is
/// cleared by its held end's floor.** A segment rising `0.05` per metre
/// along `x` through the origin, the `z = 0` plane's crossing, its span
/// stopping at `x = 5·10⁻⁸`; and a unit circle in that plane passing
/// `(1.5·10⁻⁷, 0, 0)` square to `x`. The circle passes `7.5·10⁻⁹` m under
/// the segment's carrier, past the segment's end, so the polished pair
/// holds the segment at its end: the gap there, `10⁻⁷` m, runs along the
/// segment, and `|w·n̂|`, `7.5·10⁻⁹` m, is in the escalation band. The
/// held end's floor reads it across the circle's tangent with the
/// segment a half-line back into its span: `10⁻⁷` m less the bend, no
/// meeting, in either order. Red with the first side's held floor off.
#[test]
fn a_shallow_segment_stopping_short_of_a_circle_is_cleared_by_its_held_ends_floor() {
    let (rise, under_at, end): (f64, f64, f64) = (0.05, 1.5e-7, 5e-8);
    let shallow = line([0.0; 3], [1.0, 0.0, rise]);
    let under = circle([1.0 + under_at, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    let (a, b) = (
        Span::new(&shallow, (-1.0, end * rise.hypot(1.0))),
        Span::new(&under, (PI - 0.5, PI + 0.5)),
    );
    assert!((under.eval(PI) - Point3::new(under_at, 0.0, 0.0)).norm() < 1e-15);
    for got in [meet(&a, &b, band()), meet(&b, &a, band())] {
        let m = got.unwrap();
        assert!(m.points.is_empty(), "no meeting: {m:?}");
    }
}

/// **A held end whose span comes round within the window reads it
/// two-sided.** The keel's wall and the bore's rim of
/// [`a_keel_wall_past_a_bores_rim_is_cleared_by_its_held_ends_floor`],
/// the rim's arc now running round from its end at `2.2143` rad to
/// within `10⁻⁶` rad of it, `3.75·10⁻⁸` m: the crossing lies in the gap,
/// `1.7·10⁻⁸` m past the stop and `2·10⁻⁸` m short of the start, and the
/// window about it holds arcs of the rim on both sides of the gap. The
/// held end's floor reads one side of its end alone, so it is not read
/// (`curve_pair_pose_held_one_piece`), and the two-sided floor, across a
/// gap along the rim's tangent, reads nothing: the pair escalates, the
/// safe direction, where the held floor would have cleared the window on
/// one of its two pieces.
#[test]
fn a_held_end_whose_span_comes_round_within_the_window_reads_it_two_sided() {
    let wall = Curve3::line(
        Point3::new(1.0, -1.710_783_078_032_602_8e-8, 2.75),
        Vec3::new(0.0, 0.0, -1.0),
    );
    let rim = Curve3::circle(
        Point3::new(1.037_500_008_874_687_3, -1.774_937_458_641_49e-8, 2.0),
        Vec3::new(0.0, 0.0, 1.0),
        0.037_500_008_874_691_515,
        Vec3::new(0.599_999_621_346_698_8, 0.800_000_283_989_836, 0.0),
    );
    let stop = 2.214_296_488_955_094_7;
    let (a, b) = (
        Span::new(&wall, (-1.25, 0.75)),
        Span::new(&rim, (stop + 1e-6 - TAU, stop)),
    );
    let got = meet(&a, &b, band());
    assert!(
        matches!(&got, Err(r) if matches!(r.fault, MeetFault::Escalated(_))),
        "the window in two pieces escalates: {got:?}"
    );
}

/// **A candidate with a window read at its point is a desync.** A
/// polished pair states a window and is read as that pair
/// ([`super::Found::read_pair`], [`super::Found::read_posed`]); the point
/// read takes the ends, witnesses and exact roots, whose point speaks for
/// itself. Handed one with a window, it holds a desync at its point, and
/// the answer refuses on it.
#[test]
fn a_candidate_with_a_window_read_at_its_point_is_a_desync() {
    let (x, y) = (
        line([0.0; 3], [1.0, 0.0, 0.0]),
        line([0.0; 3], [0.0, 1.0, 0.0]),
    );
    let (a, b) = (Span::new(&x, (-1.0, 1.0)), Span::new(&y, (-1.0, 1.0)));
    let mut found = super::Found::default();
    found.read(
        &a,
        &b,
        super::Candidate {
            point: Point3::new(0.5, 0.0, 0.0),
            ta: 0.5,
            tb: 0.0,
            bound: Some(super::polish::undecided()),
        },
        band(),
    );
    let got = found.finish(&a, &b, None, band());
    assert!(
        matches!(&got, Err(r) if matches!(r.fault, MeetFault::Desync)),
        "{got:?}"
    );
}

/// **A pose read with the spans swapped reads the pair at the first
/// carrier's point.** A circle's arc and a line segment, the circle
/// first (the meet oracle's draw 49 under seed `0x1738ccf5186d46b7`): the
/// pose reads the line against the circle's plane, so its polished pair
/// comes back flipped, its point the line's. Read as a pair, the gap is
/// from the first carrier's point at its parameter to the second's, so
/// the pair's point is the circle's. Their least distance is `3.42·10⁻⁹`
/// m (a dense search), inside the escalation band, so the pair escalates
/// in either order (`curve_pair_walk_pair_gap`), and any meeting found
/// lies on both carriers within the band's zero. Read at the line's
/// point, the pair's gap reads zero from that point to itself, and a
/// meeting `3.4·10⁻⁹` off the circle is answered.
#[test]
fn a_pose_read_with_the_spans_swapped_reads_the_pair_at_the_first_carriers_point() {
    let circle = Curve3::Circle(geom::Circle::new(
        geom::Frame::new(
            Point3::new(
                0.133_145_488_046_633_8,
                -2.074_020_603_639_783_5,
                -3.302_042_264_002_020_4,
            ),
            Vec3::new(
                -0.657_131_251_160_850_5,
                0.592_632_918_800_665,
                0.465_794_742_672_756_9,
            ),
            Vec3::new(
                -0.715_945_656_916_274_5,
                -0.297_411_903_979_507_73,
                -0.631_639_118_258_209_2,
            ),
        ),
        0.776_092_476_588_328_4,
    ));
    let line = Curve3::line(
        Point3::new(
            2.151_062_836_563_022,
            -1.868_833_724_593_999_4,
            -3.663_965_499_381_238_3,
        ),
        Vec3::new(
            -0.876_296_707_642_925,
            0.058_777_680_981_946_19,
            0.478_172_839_455_101_75,
        ),
    );
    let (a, b) = (
        Span::new(&circle, (-3.411_413_515_132_779, -2.298_345_592_838_532)),
        Span::new(&line, (0.460_055_357_913_058_6, 3.084_883_767_727_332_4)),
    );
    for (order, got) in [
        ("circle first", meet(&a, &b, band())),
        ("line first", meet(&b, &a, band())),
    ] {
        assert!(
            matches!(&got, Err(r) if matches!(r.fault, MeetFault::Escalated(_))),
            "{order}: not escalated: {got:?}"
        );
        let found = match &got {
            Ok(m) => &m.points,
            Err(r) => &r.found,
        };
        for m in found {
            let off = distance(&circle, m.point).max(distance(&line, m.point));
            assert!(
                off <= band().zero(),
                "{order}: a meeting {off:e} off a carrier: {got:?}"
            );
        }
    }
}

/// **A segment across a thin ellipse's vertex meets it at a root, not at
/// the extreme between its two.** Two ellipses of semi-axes `~10⁻⁴` and
/// `~10⁻⁸` m, and a segment nearly in the ellipse's plane, `h` over it,
/// crossing it near its major vertex (the delta review's cases 1410 and
/// 1021, true distances `6.57·10⁻¹¹` and `2.95·10⁻¹⁰` m). In the plane the
/// segment crosses both branches, whose points the band cannot split
/// (`curve_pair_pose_root_gap` reads zero). Between the two roots the
/// vertex, its radius of curvature `b²/a ≈ 10⁻¹²` m, takes the ellipse
/// `~10⁻⁸` m off the line, past the band, so the pair is read at its roots,
/// on both carriers. At the band `(10⁻⁹, 10⁻⁸)` it meets once, in either
/// order, within the band's zero of the segment (the point is the
/// ellipse's own, at a root). At `(10⁻⁹, 1.05·10⁻⁹)` it meets so, or,
/// where the segment's slant over the plane is not proved below the zero
/// (case 1410), refuses (`curve_pair_pose_slant_tight`, module docs, "One
/// meeting to a part"). It never reads no meeting: red where the two roots
/// are one touch at the extreme and that touch is read at its own point,
/// main's reading, its point off the segment.
#[test]
fn a_segment_across_a_thin_ellipses_vertex_meets_it_at_a_root() {
    let (tight, wide) = (Band::new(1e-9, 1.05e-9).unwrap(), band());
    for (a, b, span_b, tp, h, psi, len, span_a) in [
        (
            1.237_105_813_627_213_6e-4_f64,
            1.271_740_623_280_322_8e-8,
            (-0.364_737_545_254_126_1, 0.113_832_612_465_155_19),
            0.025_188_254_926_288_012_f64,
            6.568_256_938_995_103e-11,
            4.330_477_594_017_474_f64,
            1.718_855_939_666_199_8e-6,
            (-1.006_498_565_508_213_4e-6, 6.843_763_647_487_737e-7),
        ),
        (
            3.253_973_551_187_656e-4,
            3.445_361_057_969_92e-8,
            (-0.735_646_227_281_412_5, 0.528_524_751_652_512_8),
            0.009_417_137_441_319_778,
            2.945_053_589_198_488_5e-10,
            2.232_695_454_957_603,
            2.768_962_141_790_441_3e-4,
            (-0.000_115_225_579_162_314_02, 0.000_421_974_107_942_566_43),
        ),
    ] {
        let thin = ellipse([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], a, b);
        let through = [a * tp.cos(), b * tp.sin(), h];
        let dir = [psi.cos(), psi.sin(), h / len];
        let seg = line(through, dir);
        let (sa, sb) = (Span::new(&seg, span_a), Span::new(&thin, span_b));
        for (order, got) in [
            ("segment first", meet(&sa, &sb, tight)),
            ("ellipse first", meet(&sb, &sa, tight)),
        ] {
            let slant = MeetFault::Unsettled(Unsettled::Meter {
                row: "curve_pair_pose_slant_tight",
            });
            assert!(
                match &got {
                    Ok(m) =>
                        m.points.len() == 1 && distance(&seg, m.points[0].point) <= tight.zero(),
                    Err(r) => r.fault == slant,
                },
                "a = {a:e}, {order}: one meeting, or the slant refused: {got:?}"
            );
        }
        for (order, got) in [
            ("segment first", meet(&sa, &sb, wide)),
            ("ellipse first", meet(&sb, &sa, wide)),
        ] {
            let Ok(m) = &got else {
                panic!("a = {a:e}, {order}: refused: {got:?}");
            };
            assert_eq!(m.points.len(), 1, "a = {a:e}, {order}: {got:?}");
            let p = m.points[0].point;
            let off = distance(&seg, p);
            assert!(
                off <= wide.zero(),
                "a = {a:e}, {order}: the meeting {off:e} off the segment"
            );
        }
    }
}

/// **A segment slanted over a thin ellipse's vertex is never read as one
/// meeting.** The probe's draw 7969 (seed 1): an ellipse of semi-axes
/// `7.7·10⁻⁵` and `7·10⁻⁹` m, and a segment `5.4·10⁻¹⁰` m over a point of
/// it near its vertex, nearly in its plane (the tilt reads level), at the
/// band `(10⁻⁹, 1.038·10⁻⁹)`. In the plane the residual's extreme is
/// `9.9·10⁻¹⁰`, in the zero band: a touch. The line lies over the plane by
/// a slant, so at the extreme the distance is `√(R² + H²) ≈ 1.13·10⁻⁹`,
/// past the escalation, and at each of the residual's two roots, `2.3·10⁻⁹`
/// m apart, it is the slant alone, `5.4·10⁻¹⁰`: two meetings. The touch's
/// extreme is not within the zero, so both roots are read (module docs,
/// "A slanted pair in one plane"). The second reads its gap in an
/// enclosure the thin ellipse's carrier cannot narrow past the band
/// (`curve_carrier_straddle`), and the pair escalates. It is never one
/// meeting, nor none: red where a touch reads one root, or at the
/// extreme with no floor.
#[test]
fn a_segment_slanted_over_a_thin_ellipses_vertex_is_never_one_meeting() {
    let band = Band::new(1e-9, 1.037_893_055_681_901_7e-9).unwrap();
    let (a, b, tp, h, psi, len) = (
        7.746_858_770_958_64e-5_f64,
        7.046_761_348_332_281e-9,
        -0.016_917_715_640_598_61_f64,
        5.416_570_518_273_72e-10,
        3.241_691_953_463_229_f64,
        2.045_945_765_717_882e-6,
    );
    let thin = ellipse([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], a, b);
    let seg = line(
        [a * tp.cos(), b * tp.sin(), h],
        [psi.cos(), psi.sin(), h / len],
    );
    let (sa, sb) = (
        Span::new(
            &seg,
            (-5.830_182_073_260_333e-7, 1.388_844_419_437_708_8e-6),
        ),
        Span::new(&thin, (tp - 0.5, tp + 0.5)),
    );
    for (order, got) in [
        ("segment first", meet(&sa, &sb, band)),
        ("ellipse first", meet(&sb, &sa, band)),
    ] {
        assert!(
            !matches!(&got, Ok(m) if m.points.len() < 2),
            "{order}: fewer than two meetings: {got:?}"
        );
    }
}

/// **A line touching a circle in its plane meets it once, at the
/// extreme.** The unit circle and the line `y = 1 − 5·10⁻¹⁰` in its
/// plane: the residual's extreme, `5·10⁻¹⁰`, is in the zero band, the
/// line has no slant, and the touch is the extreme's own point, `(0, 1)`
/// to within the zero, read once in either order, as before the slant was
/// read. Unmoved by the stretch's reading.
#[test]
fn a_line_touching_a_circle_in_its_plane_meets_it_once_at_the_extreme() {
    let unit = circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
    let touching = line([-1.0, 1.0 - 5e-10, 0.0], [1.0, 0.0, 0.0]);
    let (c, l) = (
        Span::new(&unit, (0.5, 2.5)),
        Span::new(&touching, (0.0, 2.0)),
    );
    for (order, got) in [
        ("circle first", meet(&c, &l, band())),
        ("line first", meet(&l, &c, band())),
    ] {
        let m = got.unwrap_or_else(|e| panic!("{order}: {e:?}"));
        assert_eq!(m.points.len(), 1, "{order}: {m:?}");
        let p = m.points[0].point;
        assert!(
            (p - Point3::new(0.0, 1.0, 0.0)).norm() < 1e-4,
            "{order}: at {p:?}"
        );
    }
}

/// **Two crossings of a thin ellipse's branches the band can tell apart,
/// with the line between them within the gap of it, escalate.** An
/// ellipse of semi-axes `10⁻⁴` and `10⁻⁶` m, and a segment in its plane
/// square to its major axis where its branches lie `1.5·10⁻⁸` m apart: it
/// crosses each, two meetings past the escalation of each other, and
/// halfway between them the segment is `7.5·10⁻⁹` m from either branch,
/// in the band's gap: one stretch of the segment or two is not decided,
/// and the pair escalates (`curve_pair_meet_stretch`), in either order.
#[test]
fn two_crossings_of_a_thin_ellipses_branches_the_band_tells_apart_escalate() {
    let (a, b) = (1e-4_f64, 1e-6_f64);
    let thin = ellipse([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], a, b);
    // `2b sin t = 1.5·10⁻⁸` apart, at `x = a cos t`.
    let t = (7.5e-9 / b).asin();
    let square = line([a * t.cos(), 0.0, 0.0], [0.0, 1.0, 0.0]);
    let (e, s) = (
        Span::new(&thin, (-0.5, 0.5)),
        Span::new(&square, (-1e-6, 1e-6)),
    );
    for (order, got) in [
        ("ellipse first", meet(&e, &s, band())),
        ("segment first", meet(&s, &e, band())),
    ] {
        assert!(
            matches!(&got, Err(r) if matches!(r.fault, MeetFault::Escalated(_))),
            "{order}: not escalated: {got:?}"
        );
    }
}

/// The full review's touch whose zero set parts about its two roots: a
/// circle of radius `10⁻³` m about the origin in `z = 0`, its span
/// `π/2 ± 1`, and a segment along `x` through `(0, r − 0.9·10⁻⁹,
/// 0.9·10⁻⁹)`, its span `±2·10⁻³`. The residual's near extreme,
/// `−0.9·10⁻⁹`, is in the zero band, so the pose reads a touch; with the
/// slant the extreme is `1.27·10⁻⁹` m off, and the two roots, at
/// `x ≈ ±1.34·10⁻⁶`, each `0.9·10⁻⁹` m off.
fn a_touch_parted_about_its_roots(
    band: Band,
) -> [Result<Meetings<f64>, super::MeetRefusal<f64>>; 2] {
    let r = 1e-3;
    let c = circle([0.0; 3], [0.0, 0.0, 1.0], r);
    let l = line([0.0, r - 0.9e-9, 0.9e-9], [1.0, 0.0, 0.0]);
    let (sl, sc) = (
        Span::new(&l, (-2e-3, 2e-3)),
        Span::new(&c, (FRAC_PI_2 - 1.0, FRAC_PI_2 + 1.0)),
    );
    [meet(&sl, &sc, band), meet(&sc, &sl, band)]
}

/// **A touch whose zero set parts about its roots meets at both.**
/// [`a_touch_parted_about_its_roots`] at the band `(10⁻⁹, 1.05·10⁻⁹)`:
/// the extreme between the roots is past the escalation, so the two are
/// two meetings, in either order. Red where a touch reads one root.
#[test]
fn a_touch_parted_about_its_roots_meets_at_both() {
    let band = Band::new(1e-9, 1.05e-9).unwrap();
    for (order, got) in ["segment first", "circle first"]
        .into_iter()
        .zip(a_touch_parted_about_its_roots(band))
    {
        let m = got.unwrap_or_else(|e| panic!("{order}: {e:?}"));
        assert_eq!(m.points.len(), 2, "{order}: two meetings: {m:?}");
        for p in &m.points {
            assert!(
                (p.point.x.abs() - 1.34e-6).abs() < 2e-8,
                "{order}: at a root: {p:?}"
            );
        }
    }
}

/// **A touch whose zero set parts about its roots, the extreme between
/// them in the escalation band, escalates.** [`a_touch_parted_about_its_roots`]
/// at the band `(10⁻⁹, 10⁻⁸)`: the two roots' meetings lie on one stretch
/// whose point between them is in the band (C5,
/// `curve_pair_meet_stretch`), in either order. Red where a touch reads
/// one root, which answers one meeting.
#[test]
fn a_touch_parted_about_its_roots_in_the_band_escalates() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    for (order, got) in ["segment first", "circle first"]
        .into_iter()
        .zip(a_touch_parted_about_its_roots(band))
    {
        assert!(
            matches!(&got, Err(r) if matches!(r.fault, MeetFault::Escalated(_))),
            "{order}: escalates: {got:?}"
        );
    }
}
