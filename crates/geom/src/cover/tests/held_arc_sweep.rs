//! The arc-held cover against the curve its stored numbers state, over
//! sizes, eccentricities, nine windows (short, reversed, off the anchor,
//! long, past half a turn, whole and past it) and sites on the arc.
//!
//! The oracle is written here from scratch and shares nothing with
//! `conic.rs` but the stored data it reads (`axis`, `u_ref`, the anchor,
//! the semi-axes): double-double with an exact division, `sin`/`cos`
//! range-reduced by a double-double `π/2`, and the point
//! `A + u·a·(cos θ − cos θ₀) + (n × u)·b·(sin θ − sin θ₀)`.

#![allow(clippy::approx_constant, reason = "the three-double π/2 of the oracle")]

use core::f64::consts::{PI, TAU};

use geom_core::exact::{two_product, two_sum};
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

use super::super::{ParamWindow, Site};
use crate::{Curve3, Ellipse};

#[derive(Clone, Copy, Debug)]
struct Dd(f64, f64);

impl Dd {
    fn of(x: f64) -> Self {
        Dd(x, 0.0)
    }
    fn add(self, o: Dd) -> Dd {
        let (s, e) = two_sum(self.0, o.0);
        let (hi, lo) = two_sum(s, e + self.1 + o.1);
        Dd(hi, lo)
    }
    fn neg(self) -> Dd {
        Dd(-self.0, -self.1)
    }
    fn sub(self, o: Dd) -> Dd {
        self.add(o.neg())
    }
    fn mul(self, o: Dd) -> Dd {
        let (p, e) = two_product(self.0, o.0);
        let (hi, lo) = two_sum(p, e + self.0 * o.1 + self.1 * o.0);
        Dd(hi, lo)
    }
    /// `self / n`, `n` an f64, to double-double accuracy.
    fn div(self, n: f64) -> Dd {
        let q1 = self.0 / n;
        let r = self.sub(Dd::of(q1).mul(Dd::of(n)));
        let q2 = r.0 / n;
        let (hi, lo) = two_sum(q1, q2);
        Dd(hi, lo)
    }
}

/// `(sin x, cos x)` in double-double for an f64 `x`, `|x| ≤ 20`.
fn sin_cos(x: f64) -> (Dd, Dd) {
    // π/2 to three doubles.
    const P1: f64 = 1.570_796_326_794_896_6;
    const P2: f64 = 6.123_233_995_736_766e-17;
    const P3: f64 = -1.497_384_904_859_789_8e-33;
    let k = (x / P1).round();
    let r = Dd::of(x)
        .sub(Dd::of(k).mul(Dd::of(P1)))
        .sub(Dd::of(k).mul(Dd::of(P2)))
        .sub(Dd::of(k).mul(Dd::of(P3)));
    let r2 = r.mul(r);
    let (mut ts, mut s) = (r, r);
    let (mut tc, mut c) = (Dd::of(1.0), Dd::of(1.0));
    for j in 1..20 {
        let j = f64::from(j);
        ts = ts.mul(r2).div(-(2.0 * j) * (2.0 * j + 1.0));
        tc = tc.mul(r2).div(-(2.0 * j - 1.0) * (2.0 * j));
        s = s.add(ts);
        c = c.add(tc);
    }
    match (k as i64).rem_euclid(4) {
        0 => (s, c),
        1 => (c, s.neg()),
        2 => (s.neg(), c.neg()),
        _ => (c.neg(), s),
    }
}

struct Stored {
    anchor: Point3<f64>,
    u: Vec3<f64>,
    n: Vec3<f64>,
    ab: (f64, f64),
    phase: (f64, f64),
}

impl Stored {
    fn point(&self, theta: f64) -> [Dd; 3] {
        let cross =
            |p: f64, q: f64, r: f64, s: f64| Dd::of(p).mul(Dd::of(q)).sub(Dd::of(r).mul(Dd::of(s)));
        let (n, u) = (self.n, self.u);
        let v = [
            cross(n.y, u.z, n.z, u.y),
            cross(n.z, u.x, n.x, u.z),
            cross(n.x, u.y, n.y, u.x),
        ];
        let (s, c) = sin_cos(theta);
        let ku = c.sub(Dd::of(self.phase.0));
        let kv = s.sub(Dd::of(self.phase.1));
        let at = [self.anchor.x, self.anchor.y, self.anchor.z];
        let uu = [u.x, u.y, u.z];
        let (a, b) = self.ab;
        core::array::from_fn(|i| {
            Dd::of(at[i])
                .add(Dd::of(uu[i]).mul(Dd::of(a)).mul(ku))
                .add(v[i].mul(Dd::of(b)).mul(kv))
        })
    }
}

fn from(p: [Dd; 3], q: Point3<f64>) -> Vec3<f64> {
    Vec3::new(
        (p[0].0 - q.x) + p[0].1,
        (p[1].0 - q.y) + p[1].1,
        (p[2].0 - q.z) + p[2].1,
    )
}

fn round(p: [Dd; 3]) -> Point3<f64> {
    Point3::new(p[0].0 + p[0].1, p[1].0 + p[1].1, p[2].0 + p[2].1)
}

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}
fn ipt(p: Point3<f64>) -> Point3<Interval> {
    p.map(iv)
}
fn ivec(v: Vec3<f64>) -> Vec3<Interval> {
    Vec3::new(iv(v.x), iv(v.y), iv(v.z))
}

/// The worst misses `[ball, near, far, support]` of the stored points
/// against one cover, f64 and Interval, and the Interval widths.
#[derive(Debug, Default, Clone, Copy)]
struct Misses {
    f64_: [f64; 4],
    int: [f64; 4],
    width: [f64; 4],
    scale: f64,
}

fn probe(
    stored: &Stored,
    e64: Ellipse<f64>,
    eiv: Ellipse<Interval>,
    (lo, hi): (f64, f64),
    sites: &[Point3<f64>],
) -> Misses {
    let dirs = [
        stored.n,
        stored.u,
        Vec3::new(0.6, -0.8, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    let cover = Curve3::Ellipse(e64)
        .cover_over(ParamWindow::new(lo, hi))
        .unwrap();
    let icover = Curve3::Ellipse(eiv)
        .cover_over(ParamWindow::new(iv(lo), iv(hi)))
        .unwrap();
    let (mid, radius) = cover.ball();
    let (imid, iradius) = icover.ball();
    let m = |x: Interval| 0.5 * (Bounds::lo(x) + Bounds::hi(x));
    let centre = Point3::new(m(imid.x), m(imid.y), m(imid.z));
    let spread = Bounds::hi((imid - ipt(centre)).norm());
    let sup: Vec<f64> = dirs.iter().map(|d| cover.support(*d)).collect();
    let isup: Vec<Interval> = dirs.iter().map(|d| icover.support(ivec(*d))).collect();
    let ranges: Vec<(f64, f64)> = sites
        .iter()
        .map(|s| cover.distance_range(&Site::Point(*s)))
        .collect();
    let iranges: Vec<(Interval, Interval)> = sites
        .iter()
        .map(|s| icover.distance_range(&Site::Point(ipt(*s))))
        .collect();
    let mut out = Misses {
        f64_: [f64::NEG_INFINITY; 4],
        int: [f64::NEG_INFINITY; 4],
        width: [0.0; 4],
        scale: 0.0,
    };
    out.width[0] = Bounds::hi(iradius) - Bounds::lo(iradius);
    for (n, f) in &iranges {
        out.width[1] = out.width[1].max(Bounds::hi(*n) - Bounds::lo(*n));
        out.width[2] = out.width[2].max(Bounds::hi(*f) - Bounds::lo(*f));
    }
    for s in &isup {
        out.width[3] = out.width[3].max(Bounds::hi(*s) - Bounds::lo(*s));
    }
    let samples = 160;
    for k in 0..=samples {
        let t = lo + (hi - lo) * f64::from(k) / f64::from(samples);
        let p = stored.point(t);
        out.scale = out.scale.max((round(p) - Point3::origin()).norm());
        out.f64_[0] = out.f64_[0].max(from(p, mid).norm() - radius);
        out.int[0] = out.int[0].max(from(p, centre).norm() - spread - Bounds::hi(iradius));
        for (i, s) in sites.iter().enumerate() {
            let d = from(p, *s).norm();
            out.f64_[1] = out.f64_[1].max(ranges[i].0 - d);
            out.f64_[2] = out.f64_[2].max(d - ranges[i].1);
            out.int[1] = out.int[1].max(Bounds::lo(iranges[i].0) - d);
            out.int[2] = out.int[2].max(d - Bounds::hi(iranges[i].1));
        }
        for (j, dir) in dirs.iter().enumerate() {
            let h = from(p, Point3::origin()).dot(*dir);
            out.f64_[3] = out.f64_[3].max(h - sup[j]);
            out.int[3] = out.int[3].max(h - Bounds::hi(isup[j]));
        }
    }
    out
}

fn anchored(a: f64, b: f64) -> (Stored, Ellipse<f64>, Ellipse<Interval>) {
    let n = Vec3::new(1.0, 2.0, 2.0) / 3.0;
    let toward = Vec3::new(2.0, 1.0, -2.0) / 3.0;
    let anchor = Point3::new(40.25, -7.5, 3.125);
    let e = Ellipse::anchored(anchor, n, toward, a, b);
    let ie = Ellipse::anchored(ipt(anchor), ivec(n), ivec(toward), iv(a), iv(b));
    let stored = Stored {
        anchor,
        u: e.u_ref(),
        n: e.axis(),
        ab: (a, b),
        phase: (1.0, 0.0),
    };
    (stored, e, ie)
}

/// The windows the sweep reads, by name: the unit's own short window
/// at the anchor and the ones it does not try.
fn windows(a: f64, b: f64) -> Vec<(&'static str, (f64, f64))> {
    let half = 3.0 / a;
    let metre = 1.0 / b.max(a);
    vec![
        ("short@anchor", (-half, half)),
        ("short,reversed", (half, -half)),
        ("short,off-anchor", (0.4, 0.4 + 2.0 * half)),
        ("1m@anchor", (-metre, metre)),
        ("1m,off-anchor", (0.0, metre)),
        ("long", (-1.0, 2.0)),
        ("past-half", (-2.0, 2.0)),
        ("whole", (0.0, TAU)),
        ("past-whole", (-PI, 3.0 * PI)),
    ]
}

/// The sweep, printed: every configuration's worst miss at f64 (the
/// cover's own lane, `[simple.conditioned]`: real bounds computed as if
/// exact) and at Interval (an enclosure: no miss is permitted), with the
/// Interval widths. `cargo test -p geom review_probes -- --nocapture`.
#[test]
fn an_arc_held_cover_holds_its_stored_arc_over_every_window() {
    let mut int_worst: f64 = f64::NEG_INFINITY;
    let mut f64_ratio: f64 = f64::NEG_INFINITY;
    let mut f64_at = String::new();
    let mut rows = Vec::new();
    for a in [2.2e1, 2.2e3, 2.2e5, 2.2e7] {
        for ratio in [0.8, 0.1, 1.0, 3.0] {
            let b = ratio * a;
            let (stored, e, ie) = anchored(a, b);
            for (name, w) in windows(a, b) {
                let (lo, hi) = w;
                let tmid = 0.5 * (lo + hi);
                let piece_mid = lo + (hi - lo) / 8.0;
                let sites = [
                    stored.anchor + stored.n * 0.5 + Vec3::new(2.0, 1.0, -2.0) / 3.0 * 0.25,
                    round(stored.point(tmid)),
                    round(stored.point(piece_mid)),
                ];
                let m = probe(&stored, e, ie, w, &sites);
                let r = m.f64_.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                    / (f64::EPSILON * 0.5 * m.scale.max(41.0));
                if r > f64_ratio {
                    f64_ratio = r;
                    f64_at = format!("a={a:e} b/a={ratio} {name}");
                }
                int_worst = int_worst.max(m.int.iter().copied().fold(f64::NEG_INFINITY, f64::max));
                rows.push(format!(
                    "a={a:.1e} b/a={ratio} {name:18} f64[ball,near,far,sup]={:+.1e},{:+.1e},{:+.1e},{:+.1e} | int={:+.1e},{:+.1e},{:+.1e},{:+.1e} | widths={:.1e},{:.1e},{:.1e},{:.1e}",
                    m.f64_[0], m.f64_[1], m.f64_[2], m.f64_[3],
                    m.int[0], m.int[1], m.int[2], m.int[3],
                    m.width[0], m.width[1], m.width[2], m.width[3],
                ));
            }
        }
    }
    assert!(
        f64_ratio <= 16.0,
        "the f64 cover misses a stored point by {f64_ratio:.2} u of the points' scale at {f64_at}"
    );
    assert!(
        int_worst <= 0.0,
        "an Interval cover misses a stored point by {int_worst:e}"
    );
}

/// **the f64 nearest distance from a point of the arc
/// overshoots by the sagitta, far past any rounding of a coordinate.**
/// An ellipse held at its vertex with `a = b = 2.2e7` m, a 4 m arc at
/// the anchor, read from the stored curve's own point at the middle of
/// one of the four pieces: the distance is zero to a rounding of that
/// point's coordinates (`~1e-14` m), the cover's `near` is not.
#[test]
fn the_nearest_distance_from_a_point_of_a_short_far_arc_is_zero_to_its_rounding() {
    let own = 8.0 * f64::EPSILON * 0.5 * 41.0;
    let mut worst: f64 = 0.0;
    for a in [2.2e5, 2.2e6, 2.2e7] {
        let (stored, e, _) = anchored(a, 0.8 * a);
        let half = 2.0 / a;
        let (lo, hi) = (-half, half);
        let cover = Curve3::Ellipse(e)
            .cover_over(ParamWindow::new(lo, hi))
            .unwrap();
        let q = (hi - lo) / 4.0;
        for piece in 0..4 {
            let t = lo + q * (f64::from(piece) + 0.5);
            let p = stored.point(t);
            let site = round(p);
            let d = from(p, site).norm();
            let (near, _) = cover.distance_range(&Site::Point(site));
            eprintln!(
                "a={a:.1e}: site on the stored arc at {t:e}: distance {d:e}, cover near {near:e}"
            );
            worst = worst.max(near - d);
        }
    }
    assert!(
        worst <= own,
        "the cover's nearest distance from a point of its own arc overshoots by {worst:e}, past {own:e}"
    );
}
/// **an arc-held ellipse in the plane `z = 3` reads an
/// Interval box `1e280`-tall along `z`.** The cover's world box at
/// `Interval`, and the certified ellipse-arc box of `boxes.rs`.
#[test]
fn the_interval_box_of_a_world_plane_arc_held_ellipse_is_of_its_size() {
    let anchor = Point3::new(40.25, -7.5, 3.0);
    let (z, x) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
    let (a, b, half) = (22.0, 17.6, 0.1);
    let ie = Ellipse::anchored(ipt(anchor), ivec(z), ivec(x), iv(a), iv(b));
    let icover = Curve3::Ellipse(ie)
        .cover_over(ParamWindow::new(iv(-half), iv(half)))
        .unwrap();
    let (lo, hi) = icover.box_in(&super::BoxAxes::World);
    let zspan = Bounds::hi(hi.z) - Bounds::lo(lo.z);
    let ibox = icover.aabb(0.0);
    let e = Ellipse::anchored(anchor, z, x, a, b);
    let c = Curve3::Ellipse(e);
    let certified =
        crate::curves::boxes::ellipse_arc_aabb(&c, -half, half, c.eval(-half), c.eval(half))
            .unwrap();
    eprintln!(
        "Interval cover box z: [{}, {}]; aabb z [{}, {}]; certified ellipse_arc_aabb z [{}, {}]",
        Bounds::lo(lo.z),
        Bounds::hi(hi.z),
        ibox.min_z,
        ibox.max_z,
        certified.min_z,
        certified.max_z
    );
    assert!(
        zspan < 1e-9 && certified.max_z - certified.min_z < 1e-9,
        "a planar arc at z = 3 is boxed {zspan:e} tall at Interval, certified box {:e}",
        certified.max_z - certified.min_z
    );
}
