//! [`meet`] read in either order: `meet(a, b)` and `meet(b, a)` give one
//! answer with the roles swapped, and the two-conic pairs whose answer
//! the conic read across the other's plane decided
//! ([`super::pose`]'s docs, "Two conics").

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom_core::{Band, Point3, Vec3};

use super::meet_rows::{band, circle, ellipse, line, shallow_circles, touching_circles, u_for};
use super::{Hit, MeetRefusal, Meetings, Pose, Span, meet, pose};

type Read = Result<Meetings<f64>, MeetRefusal<f64>>;

fn read(a: (&Curve3<f64>, (f64, f64)), b: (&Curve3<f64>, (f64, f64)), band: Band) -> Read {
    meet(&Span::new(a.0, a.1), &Span::new(b.0, b.1), band)
}

/// Whether a hit is an end, which is the same on either order.
fn at_end(h: &Hit<f64>) -> bool {
    matches!(h, Hit::End(_))
}

/// One answer read in two orders: both refuse, or both decide the same
/// shared run and the same meetings, each `back` meeting the swap of one
/// of `got`'s within the band's escalation.
fn agree(label: &str, got: &Read, back: &Read, band: Band) -> Option<String> {
    let (got, back) = match (got, back) {
        (Err(_), Err(_)) => return None,
        (Ok(g), Ok(b)) => (g, b),
        _ => return Some(format!("{label}: one order decides\n  {got:?}\n  {back:?}")),
    };
    if got.shared != back.shared || got.points.len() != back.points.len() {
        return Some(format!("{label}: answers differ\n  {got:?}\n  {back:?}"));
    }
    let e = band.escalate();
    for p in &got.points {
        let partner = back.points.iter().any(|q| {
            (p.point - q.point).norm() <= e
                && at_end(&p.on_a) == at_end(&q.on_b)
                && at_end(&p.on_b) == at_end(&q.on_a)
        });
        if !partner {
            return Some(format!("{label}: {p:?} has no swapped partner in {back:?}"));
        }
    }
    None
}

/// Each pair read in both orders, the failures collected.
fn every(pairs: &[(String, [(Curve3<f64>, (f64, f64)); 2])], band: Band) -> Vec<String> {
    pairs
        .iter()
        .filter_map(|(label, [(a, sa), (b, sb)])| {
            agree(
                label,
                &read((a, *sa), (b, *sb), band),
                &read((b, *sb), (a, *sa), band),
                band,
            )
        })
        .collect()
}

/// **`meet` reads one answer in either order** over the meeting rows'
/// fixtures: two circles on one sphere at every tilt, circles crossing
/// across nearly parallel planes and touching the other's plane, a
/// thin ellipse in a circle's plane, near-concentric circles and their
/// ellipse twins, and a line against each. Red where the two-conic arm
/// parametrizes whichever conic comes first.
#[test]
fn meet_reads_one_answer_in_either_order() {
    let mut pairs = Vec::new();
    let centre = Point3::new(3.7, -2.1, 0.4);
    let radius = 1.3;
    for theta in [1e-2_f64, 1e-3, 3e-4, 1e-4, 1e-5, 1e-7, 1e-11] {
        let h = 0.5;
        let (s, c) = theta.sin_cos();
        let foot = |n: Vec3<f64>| {
            let d = Vec3::new(0.0, h, 0.0).dot(n);
            (centre + n * d, (radius * radius - d * d).sqrt())
        };
        let (n1, n2) = (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, c, s));
        let ((c1, r1), (c2, r2)) = (foot(n1), foot(n2));
        let span = (0.05, TAU - 0.05);
        pairs.push((
            format!("one sphere, tilt {theta:e}"),
            [
                (circle(c1.to_array(), n1.to_array(), r1), span),
                (circle(c2.to_array(), n2.to_array(), r2), span),
            ],
        ));
    }
    for tilt in [2e-8, 1e-7, 1e-5, 1e-3] {
        for h in [0.5e-9, 3e-8, 1e-3] {
            pairs.push((
                format!("shallow, tilt {tilt:e}, h {h:e}"),
                shallow_circles(tilt, h),
            ));
        }
        let [_, b] = shallow_circles(tilt, 0.5e-9);
        pairs.push((
            format!("line × shallow, tilt {tilt:e}"),
            [(line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]), (-1.0, 1.0)), b],
        ));
    }
    for tilt in [1.2e-8_f64, 1e-6, 1e-4] {
        for angle in [2.0_f64, 20.0, 90.0] {
            let along: f64 = 1e-3;
            let lift = tilt.sin() * (1.0 - along.cos());
            pairs.push((
                format!("touching, tilt {tilt:e}, crossing {angle}°"),
                touching_circles(tilt, 6.5e-10 - lift, along, angle.to_radians()),
            ));
        }
    }
    for (b_over_a, tilt) in [(1e-3, 5e-7), (1e-2, 5e-8), (0.5, 0.3)] {
        let (s, c) = f64::sin_cos(tilt);
        pairs.push((
            format!("thin ellipse, b/a {b_over_a:e}, tilt {tilt:e}"),
            [
                (
                    ellipse(
                        [0.0, 0.0, 0.0],
                        [0.0, -s, c],
                        [1.0, 0.0, 0.0],
                        1.0,
                        b_over_a,
                    ),
                    (-PI, PI),
                ),
                (
                    circle([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5),
                    (0.1, TAU - 0.1),
                ),
            ],
        ));
    }
    let unit = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    for (gap, tilt) in [(2e-10, 1e-10), (7e-10, 5e-10), (3e-10, 6e-10)] {
        let (s, c) = f64::sin_cos(tilt);
        let r = 1.0 + gap;
        pairs.push((
            format!("near-concentric, gap {gap:e}, tilt {tilt:e}"),
            [
                (unit.clone(), (0.3, 2.9)),
                (circle([0.0, 0.0, 0.0], [0.0, -s, c], r), (0.5, 3.1)),
            ],
        ));
        pairs.push((
            format!("near-concentric ellipse, gap {gap:e}, tilt {tilt:e}"),
            [
                (unit.clone(), (0.3, 2.9)),
                (
                    ellipse([0.0, 0.0, 0.0], [0.0, -s, c], [1.0, 0.0, 0.0], r, r),
                    (0.5, 3.1),
                ),
            ],
        ));
    }
    let bad = every(&pairs, band());
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

fn sph(polar: f64, az: f64) -> Vec3<f64> {
    Vec3::new(polar.sin() * az.cos(), polar.sin() * az.sin(), polar.cos())
}

/// **A parallel tilted off the equator, clear of the equator's plane,
/// reads apart whatever its tilt reads** (the slim lune's second leg
/// against its equatorial arc). On a sphere of radius `8.7·10⁻⁴`, the
/// parallel about `p`, `off` from the pole, through a corner at polar
/// `0.75π`, lies about `6·10⁻⁴` below the equator's plane, a plain miss,
/// while its swing above that plane, `r·sin(off)`, is in the band's gap
/// at `off = 3·10⁻³` and `10⁻²`. Red where the swing routes before the
/// offset is read.
#[test]
fn a_parallel_clear_of_the_equators_plane_reads_apart() {
    let band = Band::new(1e-6, 1e-5).unwrap();
    let c = Point3::new(3.0, -2.0, 5.0);
    let r = 8.726e-4;
    let z = Vec3::new(0.0, 0.0, 1.0);
    let edge = Curve3::circle(c, z, r, Vec3::new(1.0, 0.0, 0.0));
    let edge_span = (1.0, 1.0 + 1f64.to_radians());
    let target = sph(0.75 * PI, 1.0);
    for off in [0.0, 1e-3, 3e-3, 1e-2] {
        let p = sph(off, 3.0);
        let (cos, sin) = (p.dot(target), p.cross(target).norm());
        let leg = Curve3::circle(c + p * (r * cos), p, r * sin, u_for(p));
        let label = format!("off {off:e}");
        for (got, order) in [
            (
                pose(
                    &Span::new(&leg, (0.0, TAU)),
                    &Span::new(&edge, edge_span),
                    band,
                ),
                "leg first",
            ),
            (
                pose(
                    &Span::new(&edge, edge_span),
                    &Span::new(&leg, (0.0, TAU)),
                    band,
                ),
                "arc first",
            ),
        ] {
            assert!(matches!(got, Ok(Pose::Apart)), "{label}, {order}: {got:?}");
        }
        let m = read((&leg, (0.0, TAU)), (&edge, edge_span), band).unwrap();
        assert!(m.points.is_empty(), "{label}: {m:?}");
    }
}

/// **A great circle through a tiny cap crosses its rim twice, read in
/// either order** (the tiny cap's ray against its rim). The unit sphere's
/// rim at polar `10⁻⁴`, and a great circle through `p`, `off` from the
/// pole, along a tangent at `az`: its height above the rim's plane peaks
/// at about `4·10⁻⁹`, in the band's gap, while the rim crosses the great
/// circle's plane square to it. Red where the great circle, the conic
/// that comes first, is the one read across the other's plane.
#[test]
fn a_great_circle_crosses_a_tiny_rim_in_either_order() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let o = Point3::origin();
    let z = Vec3::new(0.0, 0.0, 1.0);
    let polar = 1e-4f64;
    let rim = Curve3::circle(
        o + z * polar.cos(),
        z,
        polar.sin(),
        Vec3::new(1.0, 0.0, 0.0),
    );
    for off in [5e-5, 2e-5] {
        let p = sph(off, 0.3);
        for az in [0.0f64, 1.0, 2.0] {
            let e1 = u_for(p);
            let t = e1 * az.cos() + p.cross(e1) * az.sin();
            let ray = Curve3::circle(o, p.cross(t).normalize(), 1.0, p);
            let label = format!("off {off:e}, az {az}");
            let got = read((&ray, (0.0, TAU)), (&rim, (0.0, TAU)), band);
            let back = read((&rim, (0.0, TAU)), (&ray, (0.0, TAU)), band);
            for (m, order) in [(&got, "ray first"), (&back, "rim first")] {
                let m = m
                    .as_ref()
                    .unwrap_or_else(|e| panic!("{label}, {order}: {e:?}"));
                assert_eq!(m.points.len(), 2, "{label}, {order}: {m:?}");
                for q in &m.points {
                    let height = (q.point - o).norm() - 1.0;
                    let off_rim = (q.point.z - polar.cos()).abs();
                    assert!(
                        height.abs() <= 1e-9 && off_rim <= 1e-9,
                        "{label}, {order}: {q:?} lies off the sphere or the rim's plane"
                    );
                }
            }
            assert!(
                agree(&label, &got, &back, band).is_none(),
                "{label}: orders differ"
            );
        }
    }
}

/// **A torus meridian meets a parallel near the tube's top circle once,
/// read in either order.** On the torus `R = 3`, `r = 1` about `ẑ`, the
/// meridian in `y = 0` and the parallel at `v = π/2 − δ`: the parallel's
/// plane is tangent to the meridian near its top, so the meridian's
/// height above it has a near-double root, while the parallel crosses
/// the meridian's plane square to it. Both orders read the parallel
/// across the meridian's plane and meet at `(R + r·cos v, 0, r·sin v)`.
#[test]
fn a_meridian_meets_a_parallel_by_the_tubes_top_once() {
    let (major, minor) = (3.0, 1.0);
    let meridian = circle([major, 0.0, 0.0], [0.0, 1.0, 0.0], minor);
    for delta in [1e-2, 1e-4, 1e-6, 0.0] {
        let v = PI / 2.0 - delta;
        let parallel = circle(
            [0.0, 0.0, minor * v.sin()],
            [0.0, 0.0, 1.0],
            major + minor * v.cos(),
        );
        let at = Point3::new(major + minor * v.cos(), 0.0, minor * v.sin());
        let label = format!("δ {delta:e}");
        let got = read((&meridian, (0.0, TAU)), (&parallel, (0.0, TAU)), band());
        let back = read((&parallel, (0.0, TAU)), (&meridian, (0.0, TAU)), band());
        for (m, order) in [(&got, "meridian first"), (&back, "parallel first")] {
            let m = m
                .as_ref()
                .unwrap_or_else(|e| panic!("{label}, {order}: {e:?}"));
            assert!(
                m.points.len() == 1 && (m.points[0].point - at).norm() <= 1e-9,
                "{label}, {order}: one meeting at {at:?}: {m:?}"
            );
        }
        assert!(
            agree(&label, &got, &back, band()).is_none(),
            "{label}: orders differ"
        );
    }
}
