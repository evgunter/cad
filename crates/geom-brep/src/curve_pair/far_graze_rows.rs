//! **A far-centred conic's graze is read to the belly's own digits**
//! (the pose's [`super::pose::belly`]): a line beside an ellipse of
//! semi-axes of the order `far`, centred `far` from the vertex the line
//! grazes, read through [`super::pose()`]. The conic's signed distance
//! from the line is `c₀ + A cos θ + B sin θ`, and its extreme nearer
//! zero is the band's order while `c₀` and the swing are `far`'s.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::TAU;

use geom::Curve3;
use geom_core::exact::Expansion;
use geom_core::{Band, Point3, Vec3};

use super::{Pose, Span, pose};

/// `(wrong, of them against the exact sign past the escalation)` of the
/// poses read at `far`. Each pose is a line in the ellipse's plane, along
/// its tangent near the vertex and set off it by a gap within twice the
/// band's escalation, its origin by the arc. The belly is read exactly
/// on the stored terms along the line's normal in the plane `m`, as the
/// pose forms it: the belly's change with `m`'s direction is levered by
/// the touch's distance from the line's origin, metres, not `far`. A
/// reading is wrong where it is definite (`Apart`, or a meeting decided
/// with no window) on
/// a belly within `Kε`, or against the exact sign.
fn far_line_grazes(far: f64) -> (usize, usize) {
    let band = Band::linear_at(geom_core::Tol::witness(), 1e-12).unwrap();
    let mut rng = test_utils::fuzz::start("curve_pair::far_line_grazes");
    let unit = |rng: &mut test_utils::fuzz::Rng| {
        Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        )
        .normalize()
    };
    let (mut wrong, mut beyond) = (0, 0);
    for _ in 0..test_utils::fuzz::scaled(2000) {
        let vertex = Point3::new(
            rng.range(-500.0, 500.0),
            rng.range(-500.0, 500.0),
            rng.range(-500.0, 500.0),
        );
        let axis = unit(&mut rng);
        let u = unit(&mut rng).cross(axis).normalize();
        let v = axis.cross(u);
        let (a, b) = (far, far / 3.0);
        let centre = vertex - u * a;
        let conic = Curve3::Ellipse(geom::Ellipse::new(geom::Frame::new(centre, axis, u), a, b));
        let theta = rng.range(-0.05, 0.05);
        let crest = centre + u * (a * theta.cos()) + v * (b * theta.sin());
        let outward = (u * (b * theta.cos()) + v * (a * theta.sin())).normalize();
        let along = axis.cross(outward);
        let gap = rng.range(-2.0, 2.0) * band.escalate();
        let origin = crest + outward * gap + along * rng.range(-3.0, 3.0);
        let line = Curve3::line(origin, along);
        // The stored terms, read exactly along the pose's `m`.
        let k = conic.conic().unwrap();
        let f = k.frame();
        let Curve3::Line(stored) = line else {
            unreachable!("a line")
        };
        let m = f.axis().cross(stored.dir.normalize()).normalize();
        let (c, o) = (f.origin(), stored.origin);
        let arr = |p: Vec3<f64>| [p.x, p.y, p.z];
        let n = arr(m);
        let d0 = Expansion::difference_dot([c.x, c.y, c.z], [o.x, o.y, o.z], n);
        let (sa, sb) = k.semi_axes();
        let big_a = Expansion::dot(arr(f.u_ref()), n).scaled(sa);
        let big_b = Expansion::dot(arr(f.v_ref()), n).scaled(sb);
        let mut squares = big_a.times(&big_a);
        squares.add_expansion(&big_b.times(&big_b));
        squares.add_expansion(&d0.times(&d0).negated());
        let r = big_a.estimate().hypot(big_b.estimate());
        let exact = squares.estimate() / (r + d0.estimate().abs());
        let definite = match pose(
            &Span::new(&line, (-10.0, 10.0)),
            &Span::new(&conic, (0.0, TAU)),
            band,
        ) {
            Ok(Pose::Apart) => Some(false),
            // A candidate with a bound is a window, not a decided meeting.
            Ok(Pose::Generic(found)) if found.iter().any(|c| c.bound.is_none()) => Some(true),
            _ => None,
        };
        if let Some(meets) = definite {
            let threshold = band.escalate() * (1.0 - 64.0 * f64::EPSILON);
            let against = meets != (exact > 0.0);
            if exact.abs() < threshold || against {
                wrong += 1;
                if against && exact.abs() >= threshold {
                    beyond += 1;
                }
            }
        }
    }
    (wrong, beyond)
}

/// **A line grazing a far-centred conic is read wrong nowhere**: the
/// pose charges no rounding (`[simple.conditioned]`), so no reading may be
/// definite where the exact belly is not.
#[test]
fn a_line_grazing_a_far_centred_conic_is_read_wrong_nowhere() {
    let got: Vec<_> = [1e4, 1e5, 1e6]
        .into_iter()
        .map(|far| (far, far_line_grazes(far)))
        .collect();
    println!("(far, (wrong, beyond the escalation)): {got:?}");
    assert!(
        got.iter().all(|g| g.1.0 == 0),
        "read wrong (wrong, beyond the escalation): {got:?}; {}",
        test_utils::fuzz::replay()
    );
}
