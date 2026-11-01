//! **`Link::offset` and `Link::lowest_on` against closed-form oracles**:
//! the signed distance from five cones whose boundary is written down by
//! hand (a block's corner, a hole's, a saddle, a ridge and a trough),
//! over random points and segments, segments at contact, turned
//! crossings, and the Interval and Dual lanes.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

// Gated to the reader it checks and the band it decides with.
test_utils::gated_to![
    "crates/topo/src/link.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
];

use geom_core::{Bounds, Dual, Interval};
use test_utils::fuzz::Rng;

use super::*;

fn band() -> Band {
    Band::linear(geom_core::Tol::witness()).expect("the witness tolerance is a band")
}

fn pt<T: Real>(x: f64, y: f64, z: f64) -> Point3<T> {
    Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}

fn vv<T: Real>(x: f64, y: f64, z: f64) -> Vec3<T> {
    Vec3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}

/// A link about the origin from bare fans, each face its outward normal
/// and its sector's real points in turn.
fn fans<T: Decide>(fans: &[(Vec3<T>, Vec<Point3<T>>)]) -> Link<T> {
    let o = pt(0.0, 0.0, 0.0);
    let sectors = fans
        .iter()
        .map(|(m, pts)| {
            FaceSector::planar(Sector {
                outward: *m,
                apex: o,
                wedges: pts.windows(2).map(|w| (w[0], w[1])).collect(),
                closed: false,
            })
        })
        .collect();
    Link::of(o, sectors).expect("the fans make a link")
}

/// A block's corner (material `x, y, z ≤ 0`) whose edge along `−x` is
/// `s` long; `hole`: an octant hole's corner (material where any
/// coordinate is negative).
fn corner<T: Decide>(hole: bool, s: f64) -> Link<T> {
    let k = if hole { 1.0 } else { -1.0 };
    let (x, y, z) = (pt(k * s, 0.0, 0.0), pt(0.0, k, 0.0), pt(0.0, 0.0, k));
    fans(&[
        (vv(1.0, 0.0, 0.0), vec![y, z]),
        (vv(0.0, 1.0, 0.0), vec![z, x]),
        (vv(0.0, 0.0, 1.0), vec![x, y]),
    ])
}

/// An L-bracket's reflex top corner: material below `z = 0` outside the
/// notch `x, y > 0`.
fn saddle<T: Decide>() -> Link<T> {
    let (x, y, z) = (pt(1.0, 0.0, 0.0), pt(0.0, 1.0, 0.0), pt(0.0, 0.0, -1.0));
    fans(&[
        (
            vv(0.0, 0.0, 1.0),
            vec![y, pt(-1.0, 0.0, 0.0), pt(0.0, -1.0, 0.0), x],
        ),
        (vv(0.0, 1.0, 0.0), vec![x, z]),
        (vv(1.0, 0.0, 0.0), vec![z, y]),
    ])
}

/// The convex edge along `y` (material `x, z ≤ 0`), turned `deg` about
/// `z`; `trough`: the concave one (material `x ≤ 0` or `z ≤ 0`).
fn edge<T: Decide>(trough: bool, deg: f64) -> Link<T> {
    let (c, s) = (deg.to_radians().cos(), deg.to_radians().sin());
    let r = |x: f64, y: f64, z: f64| pt(c * x - s * y, s * x + c * y, z);
    let (y, ny) = (r(0.0, 1.0, 0.0), r(0.0, -1.0, 0.0));
    let (top, side) = (vv(0.0, 0.0, 1.0), vv(c, s, 0.0));
    if trough {
        fans(&[
            (top, vec![ny, r(1.0, 0.0, 0.0), y]),
            (side, vec![y, r(0.0, 0.0, 1.0), ny]),
        ])
    } else {
        fans(&[
            (top, vec![y, r(-1.0, 0.0, 0.0), ny]),
            (side, vec![ny, r(0.0, 0.0, -1.0), y]),
        ])
    }
}

fn hyp(a: &[f64]) -> f64 {
    a.iter().map(|x| x * x).sum::<f64>().sqrt()
}

fn sd_block(q: Point3<f64>) -> f64 {
    if q.x <= 0.0 && q.y <= 0.0 && q.z <= 0.0 {
        -(-q.x).min(-q.y).min(-q.z)
    } else {
        hyp(&[q.x.max(0.0), q.y.max(0.0), q.z.max(0.0)])
    }
}

fn sd_ridge(q: Point3<f64>) -> f64 {
    if q.x <= 0.0 && q.z <= 0.0 {
        -(-q.x).min(-q.z)
    } else {
        hyp(&[q.x.max(0.0), q.z.max(0.0)])
    }
}

fn sd_saddle(q: Point3<f64>) -> f64 {
    let (x, y, z) = (q.x, q.y, q.z);
    let t_in = if x <= 0.0 || y <= 0.0 { 0.0 } else { x.min(y) };
    let d = hyp(&[z, t_in])
        .min(hyp(&[y, (-x).max(0.0), z.max(0.0)]))
        .min(hyp(&[x, (-y).max(0.0), z.max(0.0)]));
    if z <= 0.0 && !(x > 0.0 && y > 0.0) {
        -d
    } else {
        d
    }
}

type Oracle = fn(Point3<f64>) -> f64;

/// The five cones, each with its signed distance written down.
fn cases() -> Vec<(&'static str, Link<f64>, Oracle)> {
    vec![
        ("block", corner(false, 1.0), sd_block as Oracle),
        ("hole", corner(true, 1.0), |q| {
            -sd_block(Point3::new(-q.x, -q.y, -q.z))
        }),
        ("saddle", saddle(), sd_saddle),
        ("ridge", edge(false, 0.0), sd_ridge),
        ("trough", edge(true, 0.0), |q| {
            -sd_ridge(Point3::new(-q.x, -q.y, -q.z))
        }),
    ]
}

/// The least of `sd` along `a`–`c`: a dense sample, refined about its
/// least by a ternary search.
fn least(sd: Oracle, a: Point3<f64>, c: Point3<f64>) -> f64 {
    let at = |t: f64| sd(a + (c - a) * t);
    let n = 2000;
    let (mut best, mut best_t) = (f64::INFINITY, 0.0);
    for i in 0..=n {
        let t = f64::from(i) / f64::from(n);
        if at(t) < best {
            (best, best_t) = (at(t), t);
        }
    }
    let step = 1.0 / f64::from(n);
    let (mut lo, mut hi) = ((best_t - step).max(0.0), (best_t + step).min(1.0));
    for _ in 0..100 {
        let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
        if at(m1) < at(m2) {
            hi = m2;
        } else {
            lo = m1;
        }
    }
    best.min(at(0.5 * (lo + hi)))
}

/// **`offset` is the signed distance from the cone, and `lowest_on` its
/// least along a segment, at random**: points and segments in the cube
/// `[−1, 1]³` about five cones, against their written-down signed
/// distance and a dense sample of it refined by a ternary search. A
/// candidate that is a value the segment takes never reads below the
/// least; one the least is at reads it within `1e-9`. Within the band of
/// the boundary the sign is the band's: a point's offset may read either
/// sign of its distance there, and a segment's least within two
/// escalations. Within the band of a sector's bound a point may read the
/// plane's distance for the ray's, or the ray's for the plane's: they
/// part by `√(d² + s²) − |d| ≤ s²/2|d|`, `s` its offset across the bound
/// (at most an escalation), so its offset reads its distance within
/// that, on the same side, and a segment's least no lower than that
/// below its own. Red if a reading
/// refuses, if a wedge's distance takes a plane it does not stand over,
/// or if a candidate the least is at is missing (with the ties between
/// two rays, or a ray and a plane, removed, the saddle and the ridge
/// read high).
#[test]
fn offset_and_lowest_on_meet_the_written_down_distance() {
    let label = "link_oracle";
    let mut r = test_utils::fuzz::start(label);
    let b = band();
    let draw =
        |r: &mut Rng| Point3::new(r.range(-1.0, 1.0), r.range(-1.0, 1.0), r.range(-1.0, 1.0));
    let mut bad = Vec::new();
    for (name, link, sd) in cases() {
        for _ in 0..test_utils::fuzz::scaled(400) {
            let q = draw(&mut r);
            // Within the band of the boundary the sign is the band's.
            let band_sign = |got: f64, want: f64| {
                (got.abs() - want.abs()).abs() <= 1e-12 && want.abs() <= b.escalate()
            };
            // Within the band of a sector's bound, the plane's distance
            // or the ray's.
            let bound = |got: f64, want: f64| {
                got.signum() == want.signum()
                    && (got - want).abs()
                        <= 1e-12 + b.escalate().powi(2) / (2.0 * want.abs().max(b.escalate()))
            };
            match link.offset(q, b) {
                Ok(got)
                    if (got - sd(q)).abs() <= 1e-12
                        || band_sign(got, sd(q))
                        || bound(got, sd(q)) => {}
                got => bad.push(format!(
                    "{name}: offset at {q:?} reads {got:?}, not {}",
                    sd(q)
                )),
            }
        }
        for _ in 0..test_utils::fuzz::scaled(60) {
            let (a, c) = (draw(&mut r), draw(&mut r));
            let want = least(sd, a, c);
            match link.lowest_on((a, c), b) {
                Ok(got)
                    if got
                        >= want
                            - 1e-12
                            - b.escalate().powi(2) / (2.0 * want.abs().max(b.escalate()))
                        && got <= want + 1e-9 => {}
                Ok(got)
                    if want.abs() <= b.escalate() && (got - want).abs() <= 2.0 * b.escalate() => {}
                got => bad.push(format!(
                    "{name}: lowest on {a:?}–{c:?} reads {got:?}, not {want}"
                )),
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{label}: {} wrong; {}:\n{}",
        bad.len(),
        test_utils::fuzz::replay(),
        bad[..bad.len().min(6)].join("\n")
    );
}

/// **Segments at contact read zero**: through a ridge's line in a
/// random direction outside its material, and resting on a block's top
/// face in a random direction. Every candidate there is a contact or
/// stands in a plane's band, which is where a reading refused. Red if
/// one refuses or reads off zero.
#[test]
fn segments_at_contact_read_zero() {
    let label = "link_contact";
    let mut r = test_utils::fuzz::start(label);
    let b = band();
    let (ridge, block) = (edge::<f64>(false, 0.0), corner::<f64>(false, 1.0));
    let mut bad = Vec::new();
    for _ in 0..test_utils::fuzz::scaled(300) {
        let at = Point3::new(0.0, r.range(-0.5, 0.5), 0.0);
        let d = Vec3::new(r.range(0.05, 1.0), r.range(-1.0, 1.0), -r.range(0.05, 1.0));
        let seg = (at - d * r.range(0.2, 1.0), at + d * r.range(0.2, 1.0));
        match ridge.lowest_on(seg, b) {
            Ok(g) if g.abs() <= 1e-9 => {}
            got => bad.push(format!("ridge {seg:?}: {got:?}")),
        }
        let on = |r: &mut Rng| Point3::new(-r.range(0.1, 0.9), -r.range(0.1, 0.9), 0.0);
        let seg = (on(&mut r), on(&mut r));
        match block.lowest_on(seg, b) {
            Ok(g) if g.abs() <= 1e-9 => {}
            got => bad.push(format!("block top {seg:?}: {got:?}")),
        }
    }
    assert!(
        bad.is_empty(),
        "{label}: {} wrong; {}:\n{}",
        bad.len(),
        test_utils::fuzz::replay(),
        bad[..bad.len().min(6)].join("\n")
    );
}

/// **A turned crossing reads**: the ridge turned about `z` by angles
/// off the axes, read by a strut along `z` crossing its line at `h`.
/// Every fixture before this was axis-aligned, which hid the double
/// roots rounding put in a plane's band. Red if one refuses or reads
/// the wrong sign: `0` at contact, `+h` apart, `−h·sin(45°)` inside.
#[test]
fn a_turned_crossing_reads_its_sign() {
    let b = band();
    for deg in [0.0, 1.0, 7.0, 13.0, 30.0, 45.0, 61.0] {
        let ridge = edge::<f64>(false, deg);
        let (c, s) = (f64::to_radians(deg).cos(), f64::to_radians(deg).sin());
        let r = |x: f64, y: f64, z: f64| Point3::new(c * x - s * y, s * x + c * y, z);
        for (h, want) in [(0.0, 0.0), (0.01, 0.01), (-0.01, -0.01)] {
            let got = ridge
                .lowest_on((r(h, 0.3, -0.5), r(h, 0.3, 0.5)), b)
                .unwrap_or_else(|why| panic!("{deg}° h {h}: refused {why:?}"));
            assert!(
                (got - want).abs() <= 1e-12,
                "{deg}° h {h}: reads {got}, not {want}"
            );
        }
    }
}

/// **Beside a short ray the offset is the distance from the ray, within
/// the band**: a corner whose `−x` edge is 1 mm long, read at `(−1, d,
/// 0)`, `d` off the `y = 0` face's plane on the top face's plane. Read
/// against the ray's own point, `d` shrank a thousandfold into the band
/// and the plane was taken: `0` at `d = 5e-7`. Red if a point beside a
/// ray reads off its distance by more than the band's escalation, the
/// farthest a side read in band can stand off the ray's line.
#[test]
fn beside_a_short_ray_the_offset_is_the_distance_within_the_band() {
    let b = band();
    for s in [1.0, 1e-3] {
        let link = corner::<f64>(false, s);
        for k in [0.5, 2.0, 50.0, 500.0] {
            let d = k * b.zero();
            let got = link
                .offset(pt(-1.0, d, 0.0), b)
                .unwrap_or_else(|why| panic!("edge {s}, d {d:e}: refused {why:?}"));
            assert!(
                (got - d).abs() <= b.escalate(),
                "edge {s}, d {d:e}: reads {got:e}"
            );
        }
    }
}

/// **An Interval offset over a box encloses the offset at every point
/// of it**: a block's corner read over `x` in a box at `(x, −0.5, 0)`,
/// on the top face for `x ≤ 0` and beside the block for `x > 0`, and a
/// segment over the same box. Taking a wedge's plane wherever a side
/// read in band read `[0, 0]` over `x ∈ [0, 0.1]`. Red if an enclosure
/// misses a point's value, or a box reads no value at all.
#[test]
fn an_interval_offset_encloses_its_box() {
    let b = band();
    let (li, lf) = (corner::<Interval>(false, 1.0), corner::<f64>(false, 1.0));
    let samples = |lo: f64, hi: f64| (0..=20).map(move |i| lo + (hi - lo) * f64::from(i) / 20.0);
    for (lo, hi) in [
        (-0.1, 0.1),
        (0.0, 0.1),
        (0.05, 0.1),
        (-1e-3, 1e-3),
        (-0.1, 0.0),
    ] {
        let box_x = Interval::from_bounds(lo, hi);
        let q = Point3::new(box_x, Interval::from_f64(-0.5), Interval::zero());
        let got = li
            .offset(q, b)
            .unwrap_or_else(|why| panic!("[{lo}, {hi}]: {why:?}"));
        for x in samples(lo, hi) {
            let want = lf
                .offset(Point3::new(x, -0.5, 0.0), b)
                .expect("a point reads");
            assert!(
                got.lo() <= want && want <= got.hi(),
                "[{lo}, {hi}]: [{}, {}] misses {want} at x = {x}",
                got.lo(),
                got.hi()
            );
        }
        let seg = (
            q,
            Point3::new(box_x, Interval::from_f64(-0.4), Interval::zero()),
        );
        let got = li
            .lowest_on(seg, b)
            .unwrap_or_else(|why| panic!("[{lo}, {hi}] segment: {why:?}"));
        for x in samples(lo, hi) {
            let want = lf
                .lowest_on((Point3::new(x, -0.5, 0.0), Point3::new(x, -0.4, 0.0)), b)
                .expect("a segment reads");
            assert!(
                got.lo() <= want && want <= got.hi(),
                "[{lo}, {hi}] segment: [{}, {}] misses {want} at x = {x}",
                got.lo(),
                got.hi()
            );
        }
    }
}

/// **At contact the derivative is a face's one-sided one, finite**: a
/// vertex on a ridge's line moved along `x`, a segment crossing the line
/// moved along `x + z`, and a vertex on the top face moved along `z`, at
/// `Dual`. The distance from a ray is a norm, whose derivative at zero
/// is not a number; the plane is taken first in a tie. Red if a
/// derivative at contact is not finite, or is not the face's.
#[test]
fn at_contact_the_derivative_is_a_faces() {
    type D = Dual<f64>;
    let b = band();
    let link = edge::<D>(false, 0.0);
    let h = D::variable(0.0);
    let vertex = link
        .offset(Point3::new(h, D::from_f64(0.3), D::zero()), b)
        .expect("a vertex on the ridge reads");
    let lift = Vec3::new(h, D::zero(), h);
    let crossing = link
        .lowest_on(
            (
                pt::<D>(-1.0, 0.3, 1.0) + lift,
                pt::<D>(1.0, 0.3, -1.0) + lift,
            ),
            b,
        )
        .expect("a crossing reads");
    let on_face = link
        .offset(Point3::new(D::from_f64(-0.4), D::from_f64(0.3), h), b)
        .expect("a vertex on the top face reads");
    for (what, got) in [
        ("vertex", vertex),
        ("crossing", crossing),
        ("on the face", on_face),
    ] {
        assert_eq!(got.value, 0.0, "{what}: contact");
        assert!(got.deriv.is_finite(), "{what}: derivative {}", got.deriv);
    }
    assert_eq!(on_face.deriv, 1.0, "the top face's normal is `+z`");
}
