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
/// to meet: they cross at an angle of about `θ`. Above the tangency floor
/// (`√(κ·e)`, about `10⁻⁴` here) every meeting lands on both circles;
/// below it the circles stay within the band's escalation `e` of each
/// other over a run of about `e/θ`, the polish's basin meter refuses
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
                "{label}: below the tangency floor, {m:?}"
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
/// of it, the reader never misplaces: a decided answer meets inside both
/// spans with every meeting on both circles, a refusal is typed, and the
/// walk agrees wherever both decide.
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
                assert!(interiors_meet(m), "{label}: decided no meeting: {m:?}");
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
            assert!(interiors_meet(&w), "{label}: the walk finds no meeting");
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
/// × circle and circle × line. At a tilt of `2·10⁻⁸` the run within the
/// band about the crossing is half a metre, and the polish's basin meter
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
/// meeting or a refusal; where the touch's in-band run is short against
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

/// **A touch past one span whose in-band run reaches into both meets at
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
            .any(|p| matches!((p.on_a, p.on_b), (Hit::On { .. }, Hit::End))),
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
            .any(|p| matches!((p.on_a, p.on_b), (Hit::End, Hit::On { .. }))),
        "the segment's end meets the other line: {m:?}"
    );
    on_both("past a segment", &m, &x, &y);
}
