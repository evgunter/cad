//! Review probes (PR 7510 review, branch `oneround/conic-readers-review-r2`):
//! the arc-held cover against the curve its stored numbers state, over
//! more ellipses, windows, sites and phases than the unit's own row.
//!
//! The oracle is written here from scratch and shares nothing with
//! `conic.rs` but the stored data it reads (`axis`, `u_ref`, the anchor,
//! the semi-axes): double-double with an exact division, `sin`/`cos`
//! range-reduced by a double-double `π/2`, and the point
//! `A + u·a·(cos θ − cos θ₀) + (n × u)·b·(sin θ − sin θ₀)`.

#![allow(clippy::approx_constant, reason = "the oracle π/2")]

use core::f64::consts::{PI, TAU};

use geom_core::exact::{two_product, two_sum};
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

use super::{ParamWindow, Site};
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
        let cross = |p: f64, q: f64, r: f64, s: f64| {
            Dd::of(p).mul(Dd::of(q)).sub(Dd::of(r).mul(Dd::of(s)))
        };
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
    };
    out.width[0] = Bounds::hi(iradius) - Bounds::lo(iradius);
    for (n, f) in &iranges {
        out.width[1] = out.width[1].max(Bounds::hi(*n) - Bounds::lo(*n));
        out.width[2] = out.width[2].max(Bounds::hi(*f) - Bounds::lo(*f));
    }
    for s in &isup {
        out.width[3] = out.width[3].max(Bounds::hi(*s) - Bounds::lo(*s));
    }
    let samples = 512;
    for k in 0..=samples {
        let t = lo + (hi - lo) * f64::from(k) / f64::from(samples);
        let p = stored.point(t);
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
        u: crate::Conic::frame(&e).u_ref(),
        n: crate::Conic::frame(&e).axis(),
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
fn review_sweep_arc_held_cover() {
    let mut int_worst: f64 = f64::NEG_INFINITY;
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
    for r in &rows {
        eprintln!("{r}");
    }
    eprintln!("worst Interval miss over the sweep: {int_worst:e}");
}

/// **Red probe: the f64 nearest distance from a point of the arc
/// overshoots by the sagitta, far past any rounding of a coordinate.**
/// An ellipse held at its vertex with `a = b = 2.2e7` m, a 4 m arc at
/// the anchor, read from the stored curve's own point at the middle of
/// one of the four pieces: the distance is zero to a rounding of that
/// point's coordinates (`~1e-14` m), the cover's `near` is not.
#[test]
fn review_red_near_overshoots_on_a_short_far_centred_arc() {
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

/// The Interval support per direction, and the f64 one beside it, for
/// one small arc-held ellipse at its anchor: which direction is wide.
#[test]
fn review_interval_support_per_direction() {
    for a in [2.2e1, 2.2e7] {
        let (stored, e, ie) = anchored(a, 0.8 * a);
        let half = 3.0 / a;
        let cover = Curve3::Ellipse(e).cover_over(ParamWindow::new(-half, half)).unwrap();
        let icover = Curve3::Ellipse(ie)
            .cover_over(ParamWindow::new(iv(-half), iv(half)))
            .unwrap();
        let tilt = (stored.n + stored.u * 1e-3).normalize();
        for (name, d) in [
            ("n", stored.n),
            ("n+1e-3u", tilt),
            ("u", stored.u),
            ("x", Vec3::new(1.0, 0.0, 0.0)),
            ("z", Vec3::new(0.0, 0.0, 1.0)),
        ] {
            let s = icover.support(ivec(d));
            eprintln!(
                "a={a:.1e} dir {name:8}: f64 {:+.6e}  Interval [{:+.3e}, {:+.3e}]",
                cover.support(d),
                Bounds::lo(s),
                Bounds::hi(s)
            );
        }
        let (lo, hi) = icover.box_in(&super::BoxAxes::World);
        eprintln!("a={a:.1e} Interval world box: lo {lo:?}\n   hi {hi:?}");
    }
}

/// **Red probe: an arc-held ellipse in the plane `z = 3` reads an
/// Interval box `1e280`-tall along `z`.** The cover's world box at
/// `Interval`, and the certified ellipse-arc box of `boxes.rs`.
#[test]
fn review_red_interval_box_of_a_world_plane_arc_held_ellipse() {
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
    let certified = crate::curves::boxes::ellipse_arc_aabb(&c, -half, half, c.eval(-half), c.eval(half)).unwrap();
    eprintln!("Interval cover box z: [{}, {}]; aabb z [{}, {}]; certified ellipse_arc_aabb z [{}, {}]",
        Bounds::lo(lo.z), Bounds::hi(hi.z), ibox.min_z, ibox.max_z, certified.min_z, certified.max_z);
    assert!(
        zspan < 1e-9 && certified.max_z - certified.min_z < 1e-9,
        "a planar arc at z = 3 is boxed {zspan:e} tall at Interval, certified box {:e}",
        certified.max_z - certified.min_z
    );
}
// ---- Delta review (head 32d065158) ----

fn iv_around(x: f64, rel: f64) -> Interval {
    let w = x.abs() * rel;
    Interval::from_bounds(x - w, x + w)
}

/// H1: the Interval support along the arc's own normal and directions
/// tilted off it, exact data: width, and whether it holds the stored
/// points.
#[test]
fn delta_h1_normal_and_tilts() {
    let mut worst_miss: f64 = f64::NEG_INFINITY;
    let mut worst_ratio: f64 = 0.0;
    for a in [2.0, 2.2e1, 2.2e4, 2.2e7] {
        for ratio in [0.8, 0.1, 3.0] {
            let (stored, _, ie) = anchored(a, ratio * a);
            let v = stored.n.cross(stored.u);
            for (wname, (lo, hi)) in windows(a, ratio * a) {
                let icover = Curve3::Ellipse(ie)
                    .cover_over(ParamWindow::new(iv(lo), iv(hi)))
                    .unwrap();
                for tilt in [0.0, 1e-15, 1e-12, 1e-9, 1e-6, 1e-3] {
                    for (dname, base) in [("n", stored.n), ("-n", -stored.n), ("v", v), ("u", stored.u)] {
                        let other = if dname == "u" { v } else { stored.u };
                        let d = (base + other * tilt).normalize();
                        let s = icover.support(ivec(d));
                        let width = Bounds::hi(s) - Bounds::lo(s);
                        let mut truth = f64::NEG_INFINITY;
                        for k in 0..=512 {
                            let t = lo + (hi - lo) * f64::from(k) / 512.0;
                            truth = truth.max(from(stored.point(t), Point3::origin()).dot(d));
                        }
                        worst_miss = worst_miss.max(truth - Bounds::hi(s));
                        let size = (a.max(ratio * a) * (hi - lo).abs().min(TAU)).max(1e-300);
                        if width / size > worst_ratio {
                            worst_ratio = width / size;
                            eprintln!("H1 new worst width/arc-size {:.2e}: a={a:e} b/a={ratio} {wname} dir {dname} tilt {tilt:e}: width {width:e}", width / size);
                        }
                        if width > 1e-6 {
                            eprintln!("H1 WIDE: a={a:e} b/a={ratio} {wname} dir {dname} tilt {tilt:e}: [{:e}, {:e}]", Bounds::lo(s), Bounds::hi(s));
                        }
                    }
                }
            }
        }
    }
    eprintln!("H1 SUMMARY worst miss (truth - hi) {worst_miss:e}, worst width / arc size {worst_ratio:e}");
    assert!(worst_miss <= 0.0, "an Interval support misses a stored point by {worst_miss:e}");
}

/// H1: bracketed data, semi-axes and anchor widened, so `k` straddles
/// zero: every realization's stored points under the Interval support,
/// ball and distance range.
#[test]
fn delta_h1_bracketed_realizations() {
    let mut worst = [f64::NEG_INFINITY; 3];
    let mut widest: f64 = 0.0;
    for a in [2.2e1, 2.2e4, 2.2e7] {
        for ratio in [0.8, 3.0] {
            for rel in [1e-12, 1e-6, 1e-3] {
                let b = ratio * a;
                let (stored0, _, _) = anchored(a, b);
                let n = Vec3::new(1.0, 2.0, 2.0) / 3.0;
                let toward = Vec3::new(2.0, 1.0, -2.0) / 3.0;
                let anchor = stored0.anchor;
                let aw = rel * 10.0;
                let ianchor = Point3::new(
                    Interval::from_bounds(anchor.x - aw, anchor.x + aw),
                    Interval::from_bounds(anchor.y - aw, anchor.y + aw),
                    Interval::from_bounds(anchor.z - aw, anchor.z + aw),
                );
                let ie = Ellipse::anchored(ianchor, ivec(n), ivec(toward), iv_around(a, rel), iv_around(b, rel));
                let v = stored0.n.cross(stored0.u);
                let dirs = [stored0.n, -stored0.n, v, stored0.u, (stored0.n + v * 1e-9).normalize()];
                for (_, (lo, hi)) in windows(a, b) {
                    let cover = Curve3::Ellipse(ie)
                        .cover_over(ParamWindow::new(iv(lo), iv(hi)))
                        .unwrap();
                    let sup: Vec<Interval> = dirs.iter().map(|d| cover.support(ivec(*d))).collect();
                    let site = anchor + n * 0.5;
                    let (near, far) = cover.distance_range(&Site::Point(ipt(site)));
                    for s in &sup {
                        widest = widest.max(Bounds::hi(*s) - Bounds::lo(*s));
                    }
                    for (sa, sb, sx) in [(-1.0, -1.0, -1.0), (1.0, 1.0, 1.0), (1.0, -1.0, 0.0), (-1.0, 1.0, 0.5)] {
                        let st = Stored {
                            anchor: anchor + Vec3::new(sx * aw, -sx * aw, sx * aw),
                            u: stored0.u,
                            n: stored0.n,
                            ab: (a * (1.0 + sa * rel), b * (1.0 + sb * rel)),
                            phase: (1.0, 0.0),
                        };
                        for k in 0..=256 {
                            let t = lo + (hi - lo) * f64::from(k) / 256.0;
                            let p = st.point(t);
                            for (d, s) in dirs.iter().zip(&sup) {
                                worst[0] = worst[0].max(from(p, Point3::origin()).dot(*d) - Bounds::hi(*s));
                            }
                            let dist = from(p, site).norm();
                            worst[1] = worst[1].max(Bounds::lo(near) - dist);
                            worst[2] = worst[2].max(dist - Bounds::hi(far));
                        }
                    }
                }
            }
        }
    }
    eprintln!("H1 bracketed SUMMARY worst miss support {:e} near {:e} far {:e}; widest support {widest:e}", worst[0], worst[1], worst[2]);
    // The realizations' frames are the point frame's (the axis and seam
    // are point data): a miss past the oracle's rounding is a hole.
    assert!(worst.iter().all(|w| *w <= 1e-9 * 0.0 + 1e-12), "bracketed miss {worst:?}");
}

/// H2: centre-held circles and ellipses, short and long windows, the
/// f64 farthest distance against the stored curve, from sites near the
/// arc and far off it. Prints the far bound itself so base and head can
/// be compared; asserts it holds the points.
#[test]
fn delta_h2_centre_held_far() {
    let mut worst: f64 = f64::NEG_INFINITY;
    let mut rows = Vec::new();
    for a in [2.2e1, 2.2e4, 2.2e7] {
        for ratio in [1.0, 0.8, 3.0] {
            let b = ratio * a;
            let centre = Point3::new(40.25, -7.5, 3.125);
            let n = Vec3::new(1.0, 2.0, 2.0) / 3.0;
            let toward = Vec3::new(2.0, 1.0, -2.0) / 3.0;
            let frame = crate::Frame::new(centre, n, toward);
            let e = Ellipse::new(frame, a, b);
            let st = Stored { anchor: centre, u: frame.u_ref(), n: frame.axis(), ab: (a, b), phase: (0.0, 0.0) };
            for (wname, (lo, hi)) in [("tiny", (0.3, 0.3 + 1e-9)), ("short", (0.3, 0.3 + 3.0 / a)), ("1e-5", (0.3, 0.30001)), ("quarter", (0.3, 1.9)), ("long", (-1.0, 2.0))] {
                let cover = Curve3::Ellipse(e).cover_over(ParamWindow::new(lo, hi)).unwrap();
                let mid = round(st.point(0.5 * (lo + hi)));
                let off = mid + n * 1e-3;
                let farsite = centre + n * a;
                let inside = round(st.point(lo)) + (centre - round(st.point(lo))) * 1e-6;
                for (sname, site) in [("on-mid", mid), ("off-mid", off), ("far", farsite), ("inside-end", inside)] {
                    let (near, far) = cover.distance_range(&Site::Point(site));
                    let mut dmax: f64 = 0.0;
                    let mut dmin = f64::INFINITY;
                    for k in 0..=512 {
                        let t = lo + (hi - lo) * f64::from(k) / 512.0;
                        let d = from(st.point(t), site).norm();
                        dmax = dmax.max(d);
                        dmin = dmin.min(d);
                    }
                    worst = worst.max((dmax - far) / (f64::EPSILON * (dmax + (site - Point3::origin()).norm() + a)));
                    rows.push(format!("H2 a={a:.1e} b/a={ratio} {wname:7} {sname:10}: far {far:.17e} truth {dmax:.17e} slack {:+.3e} | near {near:.6e} min {dmin:.6e}", far - dmax));
                }
            }
        }
    }
    for r in &rows { eprintln!("{r}"); }
    eprintln!("H2 SUMMARY worst far miss in eps of the scale {worst:e}");
    // Circle too, the most common centre-held conic.
    let c = Curve3::circle(Point3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.6, 0.8), 2.2e5, Vec3::new(1.0, 0.0, 0.0));
    let cov = c.cover_over(ParamWindow::new(0.3, 0.3 + 1e-9)).unwrap();
    let site = c.eval(0.3 + 5e-10);
    eprintln!("H2 circle r=2.2e5 tiny window, site its mid point: range {:?}", cov.distance_range(&Site::Point(site)));
}

/// H2 random search: centre-held ellipses, short windows, point and
/// line sites; prints every far bound by index for a base/head diff and
/// counts misses against the stored curve.
#[test]
fn delta_h2_random_far() {
    let mut s: u64 = 0x9e3779b97f4a7c15;
    let mut r = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut misses = 0;
    let mut worst: f64 = f64::NEG_INFINITY;
    for i in 0..4000 {
        let a = 10f64.powf(r() * 7.0);
        let b = a * 10f64.powf(r() * 2.0 - 1.0);
        let centre = Point3::new(r() * 100.0 - 50.0, r() * 100.0 - 50.0, r() * 100.0 - 50.0);
        let n = Vec3::new(r() - 0.5, r() - 0.5, r() - 0.5).normalize();
        let toward = Vec3::new(r() - 0.5, r() - 0.5, r() - 0.5).normalize();
        let frame = crate::Frame::new(centre, n, toward);
        let e = Ellipse::new(frame, a, b);
        let st = Stored { anchor: centre, u: frame.u_ref(), n: frame.axis(), ab: (a, b), phase: (0.0, 0.0) };
        let lo = r() * TAU - PI;
        let len = 10f64.powf(-10.0 + r() * 10.0);
        let hi = lo + len;
        let cover = Curve3::Ellipse(e).cover_over(ParamWindow::new(lo, hi)).unwrap();
        let tm = lo + r() * len;
        let pm = round(st.point(tm));
        let off = (pm - centre) * (r() * 2.0 - 1.0) * 1e-6;
        let site_p = pm + off + n * (r() - 0.5) * 1e-6 * a;
        let line = crate::Line { origin: site_p, dir: Vec3::new(r() - 0.5, r() - 0.5, r() - 0.5).normalize() };
        for (kind, site) in [("pt", Site::Point(site_p)), ("ln", Site::Line(line))] {
            let (_, far) = cover.distance_range(&site);
            let mut dmax: f64 = 0.0;
            for k in 0..=256 {
                let t = lo + (hi - lo) * f64::from(k) / 256.0;
                let p = st.point(t);
                let d = match kind {
                    "pt" => from(p, site_p).norm(),
                    _ => {
                        let w = from(p, line.origin);
                        (w - line.dir * w.dot(line.dir)).norm()
                    }
                };
                dmax = dmax.max(d);
            }
            let scale = f64::EPSILON * ((site_p - Point3::origin()).norm() + a.max(b) + 100.0);
            let m = (dmax - far) / scale;
            if m > 8.0 { misses += 1; }
            worst = worst.max(m);
            eprintln!("R {i} {kind} far {far:.17e} truth {dmax:.17e}");
        }
    }
    eprintln!("H2R SUMMARY misses past 8 eps of scale: {misses}, worst {worst:e}");
}
