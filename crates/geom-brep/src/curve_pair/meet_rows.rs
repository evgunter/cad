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
/// to meet: they cross at an angle of about `θ`. Above the basin floor
/// (the basin meter's crossing angle, about `√(8κe)`, `2.4·10⁻⁴` here) every meeting
/// lands on both circles;
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

/// **A crossing of the other's plane at a found meeting is not
/// polished.** A rod of radius `0.1` cut at 45°, its ellipse's quarter
/// from `(−0.8586, 0, 0.9)`, and a fillet's circle in the plane `y = 0`
/// ending there, tangent to it: the ellipse touches the circle's plane at
/// the joint. The ends find the joint; the height harmonic's other root
/// is the joint again, within the band's escalation of it, so it is not
/// polished. Polished, the closest-pair meter's basin reads the two
/// tangents parallel there and refuses (`Unsettled::Meter`). Red where a
/// deflated candidate in a found meeting's ring is polished.
#[test]
fn a_crossing_of_the_others_plane_at_a_found_meeting_is_not_polished() {
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
