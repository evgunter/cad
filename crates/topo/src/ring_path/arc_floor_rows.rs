//! **[`ConicArc::distance_floor`] reads a floor under every point of a
//! box.** The containment touch gate and the clearance cell door read it
//! through `splitting::containment::arc_distance_floor`, and a segment's
//! miss of a loop arc (`segment_misses`) reads it directly: each takes
//! it as a lower bound on the distance from the arc of every point the
//! reading's box holds, so a reading above one of them is a wrong
//! answer, not a refusal.
//!
//! The oracle walks the arc at a dense step. A walk's least distance is
//! at or above the true one, so a floor above it is above the true
//! distance: the oracle cannot call a sound floor unsound.

use super::*;
use core::f64::consts::PI;
use geom_core::{Bounds as _, Interval};
use test_utils::fuzz::{replay, scaled, start};

/// A point interval.
fn iv(x: f64) -> Interval {
    Interval::from_bounds(x, x)
}

/// The box of half-width `w` about `c`.
fn boxed(c: Point3<f64>, w: f64) -> Point3<Interval> {
    Point3::new(
        Interval::from_bounds(c.x - w, c.x + w),
        Interval::from_bounds(c.y - w, c.y + w),
        Interval::from_bounds(c.z - w, c.z + w),
    )
}

/// The conic of `frame` with semi-axes `(su, sv)` (a circle where they
/// are equal), at `f64` and lifted to point intervals.
fn conic(
    frame: geom::Frame<f64>,
    (su, sv): (f64, f64),
) -> (geom::Curve3<f64>, geom::Curve3<Interval>) {
    let p = |q: Point3<f64>| Point3::new(iv(q.x), iv(q.y), iv(q.z));
    let v = |q: Vec3<f64>| Vec3::new(iv(q.x), iv(q.y), iv(q.z));
    let lifted = geom::Frame::new(p(frame.origin), v(frame.axis), v(frame.u_ref));
    #[allow(clippy::float_cmp)]
    if su == sv {
        (
            geom::Curve3::Circle(geom::Circle::new(frame, su)),
            geom::Curve3::Circle(geom::Circle::new(lifted, iv(su))),
        )
    } else {
        (
            geom::Curve3::Ellipse(geom::Ellipse::new(frame, su, sv)),
            geom::Curve3::Ellipse(geom::Ellipse::new(lifted, iv(su), iv(sv))),
        )
    }
}

/// The floor [`ConicArc::distance_floor`] reads for the box `q`.
fn floor(carrier: &geom::Curve3<Interval>, span: (f64, f64), q: Point3<Interval>) -> Interval {
    let Some(LoopArc::Conic(arc)) = LoopArc::of(carrier, (iv(span.0), iv(span.1))) else {
        unreachable!("a circle or an ellipse is a conic arc")
    };
    arc.distance_floor(q)
}

/// The least distance from `p` of the arc `span` of `carrier`, walked at
/// `steps` steps: at or above the true least distance.
fn walked(carrier: &geom::Curve3<f64>, span: (f64, f64), p: Point3<f64>, steps: usize) -> f64 {
    (0..=steps)
        .map(|i| {
            let t = span.0 + (span.1 - span.0) * (i as f64 / steps as f64);
            (p - carrier.eval(t)).norm()
        })
        .fold(f64::INFINITY, f64::min)
}

/// The least walked distance over a `k³` grid of the box of half-width
/// `w` about `c`.
fn least_over_box(
    carrier: &geom::Curve3<f64>,
    span: (f64, f64),
    c: Point3<f64>,
    w: f64,
    k: usize,
    steps: usize,
) -> f64 {
    let at = |i: usize| w * (2.0 * i as f64 / (k - 1) as f64 - 1.0);
    let mut least = f64::INFINITY;
    for i in 0..k {
        for j in 0..k {
            for l in 0..k {
                let p = Point3::new(c.x + at(i), c.y + at(j), c.z + at(l));
                least = least.min(walked(carrier, span, p, steps));
            }
        }
    }
    least
}

/// **A box astride the antipode of a circle arc's middle reads a floor
/// under every point of it.** `Azimuth::near` encloses such a box's
/// azimuth on one side of the cut at the middle `± π` only, so the
/// azimuth held to the span reaches one end while part of the box is
/// nearer the other. Read at the held point alone, the floor was
/// 1.978, above a point of the box 1.894 from the arc; the nearer end's
/// distance covers it.
#[test]
fn a_box_astride_a_circle_arcs_far_side_reads_a_floor_under_every_point() {
    let r = 1.507_656_089_420_911_2;
    let span = (1.799_817_765_417_975_2, 6.028_197_567_811_072);
    let frame = geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x());
    let (point, boxed_carrier) = conic(frame, (r, r));
    let centre = Point3::new(
        1.575_227_195_471_407,
        1.624_715_212_073_861_7,
        0.344_122_334_117_553_9,
    );
    let got = floor(&boxed_carrier, span, boxed(centre, 0.05));
    let least = least_over_box(&point, span, centre, 0.05, 5, 8_000);
    assert!(
        got.lo() <= least && least < 1.94,
        "astride the far side: the floor {} must be under every point of the box, the least {least}",
        got.lo()
    );
}

/// **A box near a small circle's axis reads a floor under every point
/// of it.** Its directions span much of a turn about the axis, reaching
/// the antipode of the arc's middle, so the azimuth names them all a turn
/// from their copies about the middle (`[3.49, 5.49]` for corners at
/// `[−2.58, −1.21]`), and held to the span it reaches the far end. Part
/// of the box lies inside the arc, nearer its own foot than either end:
/// the floor read 0.17403 at the held point and the ends alike, above a
/// corner 0.17339 from the arc. The azimuth a turn either side covers
/// it, here and in `geom::Curve3::distance_within`.
#[test]
fn a_box_near_a_small_circles_axis_reads_a_floor_under_every_point() {
    let frame = geom::Frame::new(
        Point3::new(
            -2.952_743_838_252_743,
            2.489_685_279_672_152_7,
            0.463_444_270_601_713_45,
        ),
        Vec3::new(
            -0.115_678_488_803_909_13,
            -0.880_023_969_068_104_8,
            -0.460_625_988_295_996_16,
        ),
        Vec3::new(
            0.261_266_359_263_853_44,
            -0.474_367_192_871_185_06,
            0.840_663_818_565_140_2,
        ),
    );
    let r = 0.201_860_043_509_975_75;
    let span = (-1.979_977_359_412_634_4, 2.881_954_529_652_717_6);
    let centre = Point3::new(
        -2.920_840_704_218_852_2,
        2.566_788_324_474_794_4,
        0.486_358_444_809_275_5,
    );
    let w = 0.012_209_002_778_406_763;
    let (point, boxed_carrier) = conic(frame, (r, r));
    let q = boxed(centre, w);
    let least = least_over_box(&point, span, centre, w, 3, 20_000);
    let got = floor(&boxed_carrier, span, q);
    let closed = boxed_carrier
        .distance_within(q, (iv(span.0), iv(span.1)))
        .expect("a circle's arc distance is closed-form");
    assert!(
        got.lo() <= least && closed.lo() <= least && least < 0.1735,
        "near the axis: the floor {} and distance_within {} must be under every point of the box, \
         the least {least}",
        got.lo(),
        closed.lo()
    );
}

/// **Every box reads a floor under each of its points**, on circles and
/// ellipses over any span, half the boxes placed about the antipode of
/// the arc's middle, where the azimuth's enclosure is cut. A circle's
/// box is read by `geom::Curve3::distance_within` too, the closed form
/// geom keeps for the same question.
#[test]
fn every_box_reads_a_floor_under_each_of_its_points() {
    let mut rng = start("a conic arc's distance floor at Interval");
    let unit = |rng: &mut test_utils::fuzz::Rng| loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let n = v.norm();
        if n > 0.1 && n < 1.0 {
            break v * (1.0 / n);
        }
    };
    let (mut astride, mut circles) = (0, 0);
    for case in 0..scaled(48) {
        let axis = unit(&mut rng);
        let u_ref = loop {
            let u = unit(&mut rng);
            let along = u - axis * u.dot(axis);
            if along.norm() > 0.2 {
                break along * (1.0 / along.norm());
            }
        };
        let origin = Point3::new(
            rng.range(-3.0, 3.0),
            rng.range(-3.0, 3.0),
            rng.range(-3.0, 3.0),
        );
        let frame = geom::Frame::new(origin, axis, u_ref);
        let su = rng.range(0.2, 3.0);
        let circle = rng.below(3) != 0;
        let sv = if circle { su } else { su * rng.range(0.5, 1.5) };
        let (point, boxed_carrier) = conic(frame, (su, sv));
        let t0 = rng.range(-PI, PI);
        let span = (t0, t0 + rng.range(0.05, 2.0 * PI - 0.05));
        let w = rng.log_uniform(1e-6, 0.3);
        let centre = if rng.below(2) == 0 {
            astride += 1;
            let far = geom::mid_param(span.0, span.1) + PI + rng.range(-0.05, 0.05);
            let rho = su * rng.range(0.2, 2.5);
            origin
                + (frame.u_ref * far.cos() + frame.v_ref() * far.sin()) * rho
                + axis * rng.range(-0.5, 0.5)
        } else {
            origin + unit(&mut rng) * (su * rng.range(0.0, 3.0))
        };
        let q = boxed(centre, w);
        let least = least_over_box(&point, span, centre, w, 3, 1024);
        let got = floor(&boxed_carrier, span, q);
        assert!(
            got.lo() <= least,
            "case {case}: the floor {} is above a box point's walked distance {least}: \
             semi-axes ({su}, {sv}), span {span:?}, box {w} about {centre:?}, frame {frame:?}; {}",
            got.lo(),
            replay()
        );
        if circle {
            circles += 1;
            let closed = boxed_carrier
                .distance_within(q, (iv(span.0), iv(span.1)))
                .expect("a circle's arc distance is closed-form");
            assert!(
                closed.lo() <= least,
                "case {case}: distance_within reads {} above a box point's walked distance \
                 {least}: radius {su}, span {span:?}, box {w} about {centre:?}, frame {frame:?}; {}",
                closed.lo(),
                replay()
            );
        }
    }
    println!("{astride} boxes astride the far side, {circles} circles");
}
