//! **The blend arms place their centres at the inputs' own scale**:
//! `sheet_center`'s round × round arm on offset circles orders of
//! magnitude apart, and `corner_ball` at a near-coplanar corner far
//! from the origin, each at `f64` and at `Interval`.
//!
//! Each row is an enumeration over a fixed ladder, and each bound is in
//! units of `f64::EPSILON` times the scale the arm's inputs carry: the
//! largest of the offset radii and the centre distance for the sheet,
//! and `|p| + r/det` for the corner (`r/det²` at `Interval`) — the
//! rounding of the vertex's coordinates, and the conditioning of the
//! three-plane solve on the ball's own offset from it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::SurfaceSide;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};
use sweep::blend::Convexity;
use sweep::blend::arms::{SupportTrace, corner_ball, sheet_center};

const U: f64 = f64::EPSILON;

/// One secant pair of offset circles in the plane `z = 0`: centres
/// `d` apart along the direction at angle `phi`, offset radii
/// `off_a`, `off_b` (each the trace radius less the ball radius on its
/// `Inner` side), and a rim on the `+across` side of the centre line.
struct Pair {
    c_a: [f64; 3],
    c_b: [f64; 3],
    r_a: f64,
    r_b: f64,
    radius: f64,
    rim: [f64; 3],
}

impl Pair {
    fn new(off_a: f64, off_b: f64, d: f64, phi: f64) -> Self {
        let c_a = [0.3, -0.7, 0.0];
        let (s, c) = phi.sin_cos();
        let radius = 0.125;
        Self {
            c_a,
            c_b: [c_a[0] + d * c, c_a[1] + d * s, 0.0],
            r_a: off_a + radius,
            r_b: off_b + radius,
            radius,
            rim: [c_a[0] - s, c_a[1] + c, 0.0],
        }
    }

    fn offsets(&self) -> (f64, f64) {
        (self.r_a - self.radius, self.r_b - self.radius)
    }

    fn centre<T: Real>(&self) -> Point3<T> {
        let p = |v: [f64; 3]| Point3::new(T::from_f64(v[0]), T::from_f64(v[1]), T::from_f64(v[2]));
        let round = |c, r| SupportTrace::Round {
            center: p(c),
            radius: T::from_f64(r),
            side: SurfaceSide::Inner,
        };
        sheet_center(
            p(self.rim),
            Vec3::new(T::zero(), T::zero(), T::one()),
            round(self.c_a, self.r_a),
            round(self.c_b, self.r_b),
            T::from_f64(self.radius),
        )
    }
}

/// `off_a = 100` against `off_b = 100/ratio`, the centre distance at
/// nine places across the secant range `(off_a − off_b, off_a + off_b)`
/// and the centre line at three angles.
fn ladder() -> Vec<(f64, Pair)> {
    let mut rows = Vec::new();
    for ratio in [1.0, 1e2, 1e4, 1e6, 1e8, 1e10] {
        let off_a = 100.0;
        let off_b = off_a / ratio;
        for k in 1..10 {
            let t = -0.9 + 0.2 * f64::from(k - 1) + 0.013;
            let d = if ratio == 1.0 {
                off_a * (1.0 + t)
            } else {
                off_a + off_b * t
            };
            for phi in [0.37, 1.9, -2.6] {
                rows.push((ratio, Pair::new(off_a, off_b, d, phi)));
            }
        }
    }
    rows
}

fn dist(p: Point3<f64>, c: [f64; 3]) -> f64 {
    p.distance(Point3::new(c[0], c[1], c[2]))
}

/// At `f64` the centre lies on both offset circles to a few ulps of
/// the pair's scale, on the rim's side of the centre line, at every
/// ratio; the differences of squares took it off the smaller circle by
/// about `ratio` ulps and to NaN past a ratio of `1e8`.
#[test]
fn sheet_centre_lies_on_both_offset_circles_at_any_ratio() {
    let mut worst = 0.0_f64;
    for (ratio, pair) in ladder() {
        let (off_a, off_b) = pair.offsets();
        let scale = off_a
            .max(off_b)
            .max(dist(Point3::new(pair.c_b[0], pair.c_b[1], 0.0), pair.c_a));
        let c = pair.centre::<f64>();
        let miss = (dist(c, pair.c_a) - off_a)
            .abs()
            .max((dist(c, pair.c_b) - off_b).abs());
        let ulps = miss / (U * scale);
        assert!(
            ulps <= 8.0,
            "ratio {ratio:e}: the centre {c:?} is {ulps} ulps of scale off an offset circle"
        );
        let span = [pair.c_b[0] - pair.c_a[0], pair.c_b[1] - pair.c_a[1]];
        let side = span[0] * (c.y - pair.c_a[1]) - span[1] * (c.x - pair.c_a[0]);
        assert!(
            side > 0.0,
            "ratio {ratio:e}: the centre is off the rim's branch"
        );
        worst = worst.max(ulps);
    }
    eprintln!("sheet_center round x round, worst off-carrier: {worst:.2} ulps of scale");
}

/// At `Interval` the enclosure of the centre holds the `f64` centre and
/// is a few ulps of scale wide at every ratio: the differences of
/// squares widened it by about `ratio` ulps, and past `1e8` its
/// radicand straddled zero.
#[test]
fn sheet_centre_enclosure_is_narrow_at_any_ratio() {
    let mut worst = 0.0_f64;
    for (ratio, pair) in ladder() {
        let (off_a, off_b) = pair.offsets();
        let scale = off_a
            .max(off_b)
            .max(dist(Point3::new(pair.c_b[0], pair.c_b[1], 0.0), pair.c_a));
        let c = pair.centre::<Interval>();
        let point = pair.centre::<f64>();
        for (axis, (iv, x)) in [(c.x, point.x), (c.y, point.y), (c.z, point.z)]
            .into_iter()
            .enumerate()
        {
            let width = (iv.hi() - iv.lo()) / (U * scale);
            assert!(
                width <= 96.0,
                "ratio {ratio:e}, axis {axis}: the enclosure {iv:?} is {width} ulps of scale wide"
            );
            assert!(
                iv.lo() <= x && x <= iv.hi(),
                "ratio {ratio:e}, axis {axis}: the enclosure {iv:?} misses the f64 centre {x}"
            );
            worst = worst.max(width);
        }
    }
    eprintln!("sheet_center round x round at Interval, widest: {worst:.2} ulps of scale");
}

/// A corner whose third normal leans out of the plane of the first two
/// by `s`, so `det(n₁, n₂, n₃)` is about `s`, turned by a generic
/// rotation so no normal has a zero coordinate.
fn normals(s: f64) -> [[f64; 3]; 3] {
    let n3 = [-0.6, -0.8, s];
    let len = (n3[0] * n3[0] + n3[1] * n3[1] + n3[2] * n3[2]).sqrt();
    let k = Vec3::new(0.3, -0.5, 0.8).normalize();
    let (sin, cos) = 0.7_f64.sin_cos();
    let turn = |n: [f64; 3]| {
        let v = Vec3::new(n[0], n[1], n[2]);
        let w = v * cos + k.cross(v) * sin + k * (k.dot(v) * (1.0 - cos));
        let w = w.normalize();
        [w.x, w.y, w.z]
    };
    [
        turn([0.8, -0.6, 0.0]),
        turn([0.28, 0.96, 0.0]),
        turn([n3[0] / len, n3[1] / len, n3[2] / len]),
    ]
}

/// The vertex about 1 km from the origin, and the three points the arm
/// is handed: the vertex thrice, as the doors hand it, or a point of
/// each plane a unit off it.
fn corner(distinct: bool, n: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let p = [613.7, -580.3, 541.9];
    if !distinct {
        return [p; 3];
    }
    let along = |n: [f64; 3], k: f64| {
        let t = Vec3::new(n[0], n[1], n[2])
            .cross(Vec3::new(0.31, 0.47, -0.83))
            .normalize();
        [p[0] + k * t.x, p[1] + k * t.y, p[2] + k * t.z]
    };
    [p, along(n[1], 0.7), along(n[2], -1.3)]
}

fn ball<T: Real>(verts: [[f64; 3]; 3], n: [[f64; 3]; 3], r: f64) -> Point3<T> {
    let p = |v: [f64; 3]| Point3::new(T::from_f64(v[0]), T::from_f64(v[1]), T::from_f64(v[2]));
    let v = |v: [f64; 3]| Vec3::new(T::from_f64(v[0]), T::from_f64(v[1]), T::from_f64(v[2]));
    corner_ball(
        [p(verts[0]), p(verts[1]), p(verts[2])],
        [v(n[0]), v(n[1]), v(n[2])],
        T::from_f64(r),
        Convexity::Convex,
    )
    .center
}

/// `(s, distinct, the three plane points, the three normals)`.
type Corner = (f64, bool, [[f64; 3]; 3], [[f64; 3]; 3]);

fn corners() -> Vec<Corner> {
    let mut rows = Vec::new();
    for s in [0.5, 1e-2, 1e-4, 1e-6, 1e-7] {
        let n = normals(s);
        for distinct in [false, true] {
            rows.push((s, distinct, corner(distinct, n), n));
        }
    }
    rows
}

/// At `f64` the ball's centre lies at depth `r` inside each of its three
/// planes to a few ulps of `|p| + r/det`; anchored at the world origin
/// it sat about `|p|/r` times further off, `u·|p|/det`.
#[test]
fn corner_ball_far_from_the_origin_sits_on_its_planes() {
    let r = 0.25;
    let mut worst = 0.0_f64;
    for (s, distinct, verts, n) in corners() {
        let c = ball::<f64>(verts, n, r);
        let det = Vec3::new(n[0][0], n[0][1], n[0][2])
            .dot(Vec3::new(n[1][0], n[1][1], n[1][2]).cross(Vec3::new(n[2][0], n[2][1], n[2][2])));
        let scale = 613.7_f64.hypot(580.3).hypot(541.9) + r / det.abs();
        for (i, (p, n)) in verts.iter().zip(n).enumerate() {
            let depth = (c.x - p[0]) * n[0] + (c.y - p[1]) * n[1] + (c.z - p[2]) * n[2];
            let ulps = (depth + r).abs() / (U * scale);
            assert!(
                ulps <= 8.0,
                "s {s:e}, distinct {distinct}: the centre {c:?} is {ulps} ulps of |p| + r/det \
                 off plane {i}"
            );
            worst = worst.max(ulps);
        }
    }
    eprintln!("corner_ball far from the origin, worst off-plane: {worst:.2} ulps of |p| + r/det");
}

/// At `Interval` the enclosure of the centre holds the `f64` centre and
/// is a few ulps of `|p| + r/det²` wide: `det`'s own enclosure is
/// relatively `u/det` wide, which the ball's offset `r/det` carries.
/// Anchored at the world origin it was about `u·|p|/det` wide.
#[test]
fn corner_ball_far_from_the_origin_enclosure_is_narrow() {
    let r = 0.25;
    let mut worst = 0.0_f64;
    for (s, distinct, verts, n) in corners() {
        let c = ball::<Interval>(verts, n, r);
        let point = ball::<f64>(verts, n, r);
        let det = Vec3::new(n[0][0], n[0][1], n[0][2])
            .dot(Vec3::new(n[1][0], n[1][1], n[1][2]).cross(Vec3::new(n[2][0], n[2][1], n[2][2])));
        let scale = 613.7_f64.hypot(580.3).hypot(541.9) + r / det.powi(2);
        for (axis, (iv, x)) in [(c.x, point.x), (c.y, point.y), (c.z, point.z)]
            .into_iter()
            .enumerate()
        {
            let width = (iv.hi() - iv.lo()) / (U * scale);
            assert!(
                width <= 16.0,
                "s {s:e}, distinct {distinct}, axis {axis}: the enclosure {iv:?} is {width} ulps \
                 wide"
            );
            assert!(
                iv.lo() <= x && x <= iv.hi(),
                "s {s:e}, distinct {distinct}, axis {axis}: the enclosure {iv:?} misses {x}"
            );
            worst = worst.max(width);
        }
    }
    eprintln!(
        "corner_ball far from the origin at Interval, widest: {worst:.2} ulps of |p| + r/det²"
    );
}
