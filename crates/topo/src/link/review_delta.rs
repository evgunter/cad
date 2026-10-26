//! Review probes for PR 5773's fix pass (review/atrest-link-delta), not
//! for merging. Turned, thin and random cones against an independent
//! oracle: the distance is the least over wedges of the distance to the
//! closed planar wedge, membership is a ray cast from `q` to a far point
//! of known membership.

#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    clippy::print_stderr,
    clippy::too_many_lines
)]

use geom_core::{Bounds, Dual, Interval};

use super::*;

fn band() -> Band {
    Band::linear(geom_core::Tol::witness()).expect("a band")
}

struct Rng(u64);
impl Rng {
    fn u(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn r(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.u()
    }
    fn log(&mut self, lo: f64, hi: f64) -> f64 {
        10f64.powf(self.r(lo, hi))
    }
    fn sgn(&mut self) -> f64 {
        if self.u() < 0.5 { -1.0 } else { 1.0 }
    }
    fn v(&mut self) -> Vec3<f64> {
        Vec3::new(self.r(-1.0, 1.0), self.r(-1.0, 1.0), self.r(-1.0, 1.0))
    }
}

type V = Vec3<f64>;
type P = Point3<f64>;

/// A cone as fans of (outward normal, real points), with a reference
/// direction whose membership is known.
#[derive(Clone)]
struct Cone {
    name: String,
    fans: Vec<(V, Vec<V>)>,
    refd: V,
    ref_in: bool,
}

fn o() -> P {
    P::new(0.0, 0.0, 0.0)
}

/// Orient a fan counterclockwise about its normal.
fn orient(n: V, mut pts: Vec<V>) -> Vec<V> {
    if n.dot(pts[0].cross(pts[1])) < 0.0 {
        pts.reverse();
    }
    pts
}

impl Cone {
    fn complement(&self) -> Self {
        Self {
            name: format!("{}~", self.name),
            fans: self
                .fans
                .iter()
                .map(|(n, p)| (-*n, orient(-*n, p.clone())))
                .collect(),
            refd: self.refd,
            ref_in: !self.ref_in,
        }
    }
    fn turned(&self, r: [V; 3]) -> Self {
        let m = |v: V| r[0] * v.x + r[1] * v.y + r[2] * v.z;
        Self {
            name: self.name.clone(),
            fans: self
                .fans
                .iter()
                .map(|(n, p)| (m(*n), p.iter().map(|&v| m(v)).collect()))
                .collect(),
            refd: m(self.refd),
            ref_in: self.ref_in,
        }
    }
    fn link<T: Decide>(&self) -> Link<T> {
        let c = |v: V| Vec3::new(T::from_f64(v.x), T::from_f64(v.y), T::from_f64(v.z));
        let p0 = Point3::new(T::zero(), T::zero(), T::zero());
        let sectors = self
            .fans
            .iter()
            .map(|(m, pts)| Sector {
                surface: geom::Surface::Plane {
                    origin: p0,
                    normal: c(*m),
                    u_ref: c(pts[0]).normalize(),
                },
                sense: true,
                outward: c(*m),
                apex: p0,
                wedges: pts.windows(2).map(|w| (p0 + c(w[0]), p0 + c(w[1]))).collect(),
                closed: false,
            })
            .collect();
        Link::of(p0, sectors).unwrap_or_else(|_| panic!("a link"))
    }
    fn wedges(&self) -> Vec<(V, V, V)> {
        self.fans
            .iter()
            .flat_map(|(n, p)| p.windows(2).map(|w| (*n, w[0], w[1])).collect::<Vec<_>>())
            .collect()
    }
    /// Unsigned distance from the cone's boundary.
    fn dist(&self, q: V) -> f64 {
        let ray = |u: V| {
            let u = u.normalize();
            (q - u * u.dot(q).max(0.0)).norm()
        };
        self.wedges()
            .iter()
            .map(|&(n, a, b)| {
                let qp = q - n * n.dot(q);
                let over = n.dot(a.cross(qp)) >= 0.0 && n.dot(qp.cross(b)) >= 0.0;
                if over {
                    n.dot(q).abs().min(ray(a)).min(ray(b))
                } else {
                    ray(a).min(ray(b))
                }
            })
            .fold(f64::INFINITY, f64::min)
    }
    /// Membership by a ray cast to a far point of known membership; `None`
    /// where two casts disagree.
    fn inside(&self, q: V) -> Option<bool> {
        let cast = |d: V| {
            let far = d.normalize() * (1e4 * q.norm().max(1.0));
            let mut odd = false;
            for (n, a, b) in self.wedges() {
                let (oq, of) = (n.dot(q), n.dot(far));
                if (oq > 0.0) == (of > 0.0) || oq == 0.0 {
                    continue;
                }
                let x = q + (far - q) * (oq / (oq - of));
                if n.dot(a.cross(x)) > 0.0 && n.dot(x.cross(b)) > 0.0 {
                    odd = !odd;
                }
            }
            odd != self.ref_in
        };
        let a = cast(self.refd);
        let b = cast(self.refd + Vec3::new(1e-3, -2e-3, 1.5e-3));
        (a == b).then_some(a)
    }
    fn sd(&self, q: V) -> Option<f64> {
        let d = self.dist(q);
        Some(if self.inside(q)? { -d } else { d })
    }
}

fn rot(r: &mut Rng) -> [V; 3] {
    let a = r.v().normalize();
    let b = r.v();
    let b = (b - a * a.dot(b)).normalize();
    [a, b, a.cross(b)]
}

fn x() -> V {
    Vec3::new(1.0, 0.0, 0.0)
}
fn y() -> V {
    Vec3::new(0.0, 1.0, 0.0)
}
fn z() -> V {
    Vec3::new(0.0, 0.0, 1.0)
}

/// A dihedral edge along y: material within `th` of `−x` in the xz-plane.
fn knife(th: f64) -> Cone {
    let (c, s) = ((th / 2.0).cos(), (th / 2.0).sin());
    let (u1, u2) = (Vec3::new(-c, 0.0, s), Vec3::new(-c, 0.0, -s));
    let (n1, n2) = (Vec3::new(s, 0.0, c), Vec3::new(s, 0.0, -c));
    Cone {
        name: format!("knife{:.0}", th.to_degrees()),
        fans: vec![(n1, orient(n1, vec![y(), u1, -y()])), (n2, orient(n2, vec![y(), u2, -y()]))],
        refd: -x(),
        ref_in: true,
    }
}

fn corner(s: f64) -> Cone {
    Cone {
        name: format!("block{s:e}"),
        fans: vec![
            (x(), orient(x(), vec![-y(), -z()])),
            (y(), orient(y(), vec![-z(), -x() * s])),
            (z(), orient(z(), vec![-x() * s, -y()])),
        ],
        refd: -(x() + y() + z()),
        ref_in: true,
    }
}

fn saddle() -> Cone {
    Cone {
        name: "saddle".into(),
        fans: vec![
            (z(), orient(z(), vec![y(), -x(), -y(), x()])),
            (y(), orient(y(), vec![x(), -z()])),
            (x(), orient(x(), vec![-z(), y()])),
        ],
        refd: -(x() + y() + z()),
        ref_in: true,
    }
}

/// A random convex cone about −z, `thin` adding a near-duplicate ray.
fn convex(r: &mut Rng, thin: bool) -> Cone {
    let mut pts: Vec<(f64, f64)> = (0..r.r(3.0, 6.99) as usize)
        .map(|_| (r.r(-1.5, 1.5), r.r(-1.5, 1.5)))
        .collect();
    if thin {
        let (a, b) = pts[0];
        let ang = r.r(0.0, 6.28);
        pts.push((a + 1e-3 * ang.cos(), b + 1e-3 * ang.sin()));
    }
    // Hull (monotone chain), counterclockwise in xy.
    pts.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let cr = |o: (f64, f64), a: (f64, f64), b: (f64, f64)| {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    };
    let mut h: Vec<(f64, f64)> = Vec::new();
    for pass in 0..2 {
        let start = h.len();
        let it: Vec<_> = if pass == 0 { pts.clone() } else { pts.iter().rev().copied().collect() };
        for p in it {
            while h.len() >= start + 2 && cr(h[h.len() - 2], h[h.len() - 1], p) <= 0.0 {
                h.pop();
            }
            h.push(p);
        }
        h.pop();
    }
    if h.len() < 3 {
        return convex(r, thin);
    }
    let (cx, cy) = (
        h.iter().map(|p| p.0).sum::<f64>() / h.len() as f64,
        h.iter().map(|p| p.1).sum::<f64>() / h.len() as f64,
    );
    let refd = Vec3::new(cx, cy, -1.0);
    let rays: Vec<V> = h.iter().map(|&(a, b)| Vec3::new(a, b, -1.0)).collect();
    let fans = (0..rays.len())
        .map(|i| {
            let (a, b) = (rays[i], rays[(i + 1) % rays.len()]);
            let mut n = a.cross(b).normalize();
            if n.dot(refd) > 0.0 {
                n = -n;
            }
            (n, orient(n, vec![a, b]))
        })
        .collect();
    Cone {
        name: format!("convex{}{}", rays.len(), if thin { "t" } else { "" }),
        fans,
        refd,
        ref_in: true,
    }
}

fn cones(r: &mut Rng) -> Vec<Cone> {
    let mut base = vec![
        corner(1.0),
        corner(1e-3),
        saddle(),
        knife(90f64.to_radians()),
        knife(4f64.to_radians()),
        knife(0.3f64.to_radians()),
        knife(150f64.to_radians()),
    ];
    for _ in 0..4 {
        base.push(convex(r, false));
        base.push(convex(r, true));
    }
    let mut out = Vec::new();
    for c in base {
        for k in 0..3 {
            let t = if k == 0 { c.clone() } else { c.turned(rot(r)) };
            out.push(t.complement());
            out.push(t);
        }
    }
    // Self-check the oracle against the plain one on an axis cone.
    out
}

/// A point within a decade range of a plane, anywhere in it.
fn near_plane(r: &mut Rng, c: &Cone) -> V {
    let (n, pts) = &c.fans[(r.u() * c.fans.len() as f64) as usize % c.fans.len()];
    let u = pts[0].normalize();
    let w = n.cross(u);
    let ang = r.r(0.0, std::f64::consts::TAU);
    let rad = r.log(-3.0, 0.0);
    let inplane = (u * ang.cos() + w * ang.sin()) * rad;
    inplane + *n * (r.sgn() * r.log(-13.0, -3.0))
}

fn near_ray(r: &mut Rng, c: &Cone) -> V {
    let ws = c.wedges();
    let (_, a, _) = ws[(r.u() * ws.len() as f64) as usize % ws.len()];
    let u = a.normalize();
    let off = r.v();
    let off = (off - u * u.dot(off)).normalize();
    u * r.log(-3.0, 0.3) + off * r.log(-13.0, -3.0)
}

fn pt(v: V) -> P {
    o() + v
}

/// Claim 1: offset's sign near planes, rays and in general, against the
/// oracle, on turned, thin, convex and reflex cones.
#[test]
fn probe_offset_sign_against_the_oracle() {
    let b = band();
    eprintln!("band zero {:e} escalate {:e}", b.zero(), b.escalate());
    let mut r = Rng(0x5eed_1234_9876_abcd);
    let cs = cones(&mut r);
    let (mut n, mut refused, mut wrong, mut worst) = (0usize, 0usize, Vec::new(), 0.0f64);
    let mut worst_at = String::new();
    let mut zero_far = 0usize;
    let mut per = std::collections::BTreeMap::new();
    for c in &cs {
        let link = c.link::<f64>();
        for i in 0..3000 {
            let q = match i % 3 {
                0 => near_plane(&mut r, c),
                1 => near_ray(&mut r, c),
                _ => r.v() * r.log(-6.0, 0.0),
            };
            let Some(want) = c.sd(q) else { continue };
            n += 1;
            match link.offset(pt(q), b) {
                Err(_) => refused += 1,
                Ok(got) => {
                    let e = (got - want).abs();
                    if e > worst {
                        worst = e;
                        worst_at = format!("{} q {q:?} got {got:e} want {want:e}", c.name);
                    }
                    if want.abs() > 10.0 * b.escalate() && got.signum() != want.signum() {
                        *per.entry(c.name.clone()).or_insert(0usize) += 1;
                        wrong.push(format!("{} q {q:?}: got {got:e} want {want:e}", c.name));
                    }
                    if got.abs() <= b.zero() && want.abs() > b.zero() {
                        zero_far += 1;
                    }
                }
            }
        }
    }
    eprintln!(
        "offset: {n} points on {} cones, refused {refused}, wrong-signed {}, worst |err| {worst:e} ({worst_at}), read within zero while out of it {zero_far}",
        cs.len(),
        wrong.len()
    );
    eprintln!("per cone: {per:?}");
    for w in wrong.iter().take(10) {
        eprintln!("WRONG {w}");
    }
    assert!(wrong.is_empty());
}

fn least(c: &Cone, a: V, e: V) -> Option<f64> {
    let at = |t: f64| c.sd(a + e * t);
    let n = 3000;
    let (mut best, mut bt) = (f64::INFINITY, 0.0);
    for i in 0..=n {
        let t = f64::from(i) / f64::from(n);
        let v = at(t)?;
        if v < best {
            (best, bt) = (v, t);
        }
    }
    let step = 1.0 / f64::from(n);
    let (mut lo, mut hi) = ((bt - step).max(0.0), (bt + step).min(1.0));
    for _ in 0..100 {
        let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
        if at(m1)? < at(m2)? {
            hi = m2;
        } else {
            lo = m1;
        }
    }
    Some(best.min(at(0.5 * (lo + hi))?))
}

/// Claims 2 and 3: lowest_on against a dense sample of the oracle.
#[test]
fn probe_lowest_on_against_the_oracle() {
    let b = band();
    let mut r = Rng(0x0dd_ba11_c0ff_ee00);
    let cs = cones(&mut r);
    let (mut n, mut refused, mut worst_over, mut worst_under) = (0usize, 0usize, 0.0f64, 0.0f64);
    let (mut over_at, mut under_at) = (String::new(), String::new());
    let mut bad = Vec::new();
    let mut by_kind = [0usize; 4];
    for c in &cs {
        let link = c.link::<f64>();
        for i in 0..120 {
            let s = r.log(-4.0, 0.0);
            let (a, e) = match i % 4 {
                // general
                0 => {
                    let a = r.v() * s;
                    (a, r.v() * s * 2.0 - a * 0.0)
                }
                // through a point near a ray
                1 => {
                    let m = near_ray(&mut r, c);
                    let d = r.v() * s;
                    (m - d * r.r(0.1, 1.0), d * 2.0)
                }
                // through a point near a plane
                2 => {
                    let m = near_plane(&mut r, c);
                    let d = r.v() * s;
                    (m - d * r.r(0.1, 1.0), d * 2.0)
                }
                // nearly in a plane
                _ => {
                    let m = near_plane(&mut r, c);
                    let (nrm, _) = c.fans[(r.u() * c.fans.len() as f64) as usize % c.fans.len()];
                    let d = r.v();
                    let d = (d - nrm * nrm.dot(d)) * s + nrm * r.sgn() * r.log(-13.0, -6.0);
                    (m - d * 0.5, d)
                }
            };
            let Some(want) = least(c, a, e) else { continue };
            n += 1;
            match link.lowest_on((pt(a), pt(a + e)), b) {
                Err(why) => {
                    refused += 1;
                    by_kind[i % 4] += 1;
                    if refused <= 4 {
                        eprintln!("REFUSED {} kind {} {a:?} + {e:?}: {why:?}", c.name, i % 4);
                    }
                }
                Ok(got) => {
                    let d = got - want;
                    if d > worst_over {
                        worst_over = d;
                        over_at = format!("{} kind {} {a:?}+{e:?}: got {got:e} want {want:e}", c.name, i % 4);
                    }
                    if -d > worst_under {
                        worst_under = -d;
                        under_at = format!("{} kind {} {a:?}+{e:?}: got {got:e} want {want:e}", c.name, i % 4);
                    }
                    if d.abs() > 1e-7 * e.norm().max(1e-9) + b.escalate() {
                        bad.push(format!("{} kind {} {a:?}+{e:?}: got {got:e} want {want:e}", c.name, i % 4));
                    }
                }
            }
        }
    }
    eprintln!(
        "lowest_on: {n} segments, refused {refused} {by_kind:?}, worst over {worst_over:e} ({over_at}), worst under {worst_under:e} ({under_at}), bad {}",
        bad.len()
    );
    for w in bad.iter().take(10) {
        eprintln!("BAD {w}");
    }
    assert!(bad.is_empty());
}

/// Claim 4: the Interval offset and lowest_on over boxes enclose the f64
/// reading at sampled points; a one-signed enclosure is never of the
/// wrong sign.
#[test]
fn probe_interval_encloses_over_boxes() {
    let b = band();
    let mut r = Rng(0xfeed_face_0bad_f00d);
    let cs = cones(&mut r);
    let (mut boxes, mut refused, mut miss, mut wrong_sign) = (0usize, 0usize, Vec::new(), Vec::new());
    let (mut segs, mut seg_refused, mut seg_miss) = (0usize, 0usize, Vec::new());
    let ib = |c: V, w: V| {
        Point3::new(
            Interval::from_bounds(c.x - w.x, c.x + w.x),
            Interval::from_bounds(c.y - w.y, c.y + w.y),
            Interval::from_bounds(c.z - w.z, c.z + w.z),
        )
    };
    let sample = |r: &mut Rng, c: V, w: V| {
        Vec3::new(
            c.x + w.x * r.r(-1.0, 1.0),
            c.y + w.y * r.r(-1.0, 1.0),
            c.z + w.z * r.r(-1.0, 1.0),
        )
    };
    for c in &cs {
        let (li, lf) = (c.link::<Interval>(), c.link::<f64>());
        for i in 0..150 {
            let ctr = match i % 3 {
                0 => near_plane(&mut r, c),
                1 => near_ray(&mut r, c),
                _ => r.v() * r.log(-4.0, 0.0),
            };
            let s = r.log(-12.0, -2.0);
            // Thin in one coordinate at times.
            let mut w = Vec3::new(s, s, s);
            if i % 2 == 0 {
                w.y *= 1e-4;
            }
            boxes += 1;
            match li.offset(ib(ctr, w), b) {
                Err(_) => refused += 1,
                Ok(got) => {
                    for k in 0..25 {
                        let q = if k == 0 { ctr } else { sample(&mut r, ctr, w) };
                        if let Ok(v) = lf.offset(pt(q), b)
                            && !(got.lo() <= v && v <= got.hi())
                        {
                            miss.push(format!("{} box {ctr:?}±{w:?}: [{:e}, {:e}] misses {v:e}", c.name, got.lo(), got.hi()));
                        }
                        if let Some(sd) = c.sd(q)
                            && sd.abs() > 10.0 * b.escalate()
                            && ((got.lo() > 0.0 && sd < 0.0) || (got.hi() < 0.0 && sd > 0.0))
                        {
                            wrong_sign.push(format!("{} box {ctr:?}±{w:?}: [{:e}, {:e}] vs sd {sd:e}", c.name, got.lo(), got.hi()));
                        }
                    }
                }
            }
            // A segment of boxes.
            let d = r.v() * r.log(-4.0, 0.0);
            let (c0, c1) = (ctr - d * 0.5, ctr + d * 0.5);
            segs += 1;
            match li.lowest_on((ib(c0, w), ib(c1, w)), b) {
                Err(_) => seg_refused += 1,
                Ok(got) => {
                    for _ in 0..8 {
                        let (a, e) = (sample(&mut r, c0, w), sample(&mut r, c1, w));
                        if let Ok(v) = lf.lowest_on((pt(a), pt(e)), b)
                            && !(got.lo() <= v && v <= got.hi())
                        {
                            seg_miss.push(format!("{} seg {c0:?}->{c1:?}±{w:?}: [{:e}, {:e}] misses {v:e}", c.name, got.lo(), got.hi()));
                        }
                    }
                }
            }
        }
    }
    eprintln!(
        "interval: {boxes} boxes, refused {refused}, misses {}, wrong-signed {}; {segs} segments, refused {seg_refused}, misses {}",
        miss.len(),
        wrong_sign.len(),
        seg_miss.len()
    );
    for w in miss.iter().chain(&wrong_sign).chain(&seg_miss).take(12) {
        eprintln!("MISS {w}");
    }
    assert!(miss.is_empty() && wrong_sign.is_empty() && seg_miss.is_empty());
}

/// Claim 5: the f64 error beside short rays, and points out of contact
/// by more than the zero that read within it.
#[test]
fn probe_short_ray_accuracy() {
    let b = band();
    let mut r = Rng(0x1357_9bdf_2468_ace0);
    for s in [1.0, 1e-3, 1e-6] {
        let c = corner(s);
        let link = c.link::<f64>();
        let (mut worst, mut worst_at, mut zero_far, mut max_true_at_zero) = (0.0f64, String::new(), 0usize, 0.0f64);
        for _ in 0..20000 {
            // Beside the short −x ray, past its real point, near its two
            // faces' planes.
            let q = Vec3::new(
                -r.log(-7.0, 0.3),
                r.sgn() * r.log(-11.0, -4.0),
                r.sgn() * r.log(-11.0, -4.0) * if r.u() < 0.3 { 0.0 } else { 1.0 },
            );
            let Some(want) = c.sd(q) else { continue };
            if let Ok(got) = link.offset(pt(q), b) {
                let e = (got - want).abs();
                if e > worst {
                    worst = e;
                    worst_at = format!("q {q:?} got {got:e} want {want:e}");
                }
                if got.abs() <= b.zero() && want.abs() > b.zero() {
                    zero_far += 1;
                    max_true_at_zero = max_true_at_zero.max(want.abs());
                }
            }
        }
        eprintln!(
            "short ray s {s:e}: worst |err| {worst:e} ({worst_at}); read within zero while out of it: {zero_far}, worst true |sd| there {max_true_at_zero:e} (zero {:e}, escalate {:e})",
            b.zero(),
            b.escalate()
        );
    }
}

/// Claim 6: the Dual tangent at contact, on turned ridges and knives.
#[test]
fn probe_dual_tangent_at_contact() {
    type D = Dual<f64>;
    let b = band();
    let mut r = Rng(0xabcd_ef01_2345_6789);
    let mut bad = Vec::new();
    for th in [90.0f64, 4.0, 150.0] {
        for k in 0..6 {
            let base = knife(th.to_radians());
            let rr = if k == 0 { [x(), y(), z()] } else { rot(&mut r) };
            for c in [base.turned(rr), base.complement().turned(rr)] {
                let link = c.link::<D>();
                let m = |v: V| rr[0] * v.x + rr[1] * v.y + rr[2] * v.z;
                let dv = |v: V| Vec3::new(D::from_f64(v.x), D::from_f64(v.y), D::from_f64(v.z));
                let h = D::variable(0.0);
                // A vertex on the ridge moved along a random direction.
                let dir = r.v();
                let q = Point3::new(D::zero(), D::zero(), D::zero()) + dv(m(y() * 0.3)) + dv(dir) * h;
                let normals: Vec<f64> = c.fans.iter().map(|(n, _)| n.dot(dir)).collect();
                match link.offset(q, b) {
                    Ok(v) if v.deriv.is_finite() && v.deriv.abs() < 10.0 => {
                        if v.value == 0.0 && !normals.iter().any(|n| (n - v.deriv).abs() < 1e-12) {
                            bad.push(format!("{} k {k} vertex: exact contact, deriv {} not a face's {normals:?}", c.name, v.deriv));
                        }
                    }
                    got => bad.push(format!("{} k {k} vertex: {got:?} rot {rr:?} dir {dir:?}", c.name)),
                }
                // A segment across the ridge, outside the material.
                let out = m(x() * 1.0);
                let seg_d = m(y() * 0.1 + z() * 0.7);
                let a = dv(m(y() * 0.3) - seg_d) + dv(out * 0.0);
                let a = Point3::new(D::zero(), D::zero(), D::zero()) + a + dv(dir) * h;
                let e = a + dv(seg_d * 2.0);
                match link.lowest_on((a, e), b) {
                    Ok(v) if v.deriv.is_finite() && v.deriv.abs() < 10.0 => {}
                    got => bad.push(format!("{} k {k} segment: {got:?}", c.name)),
                }
            }
        }
    }
    for w in &bad {
        eprintln!("DUAL {w}");
    }
    assert!(bad.is_empty(), "{} dual misses", bad.len());
}

/// Claim 8: cost per call, on cones of 2 to 7 sectors.
#[test]
fn probe_cost() {
    let b = band();
    let mut r = Rng(0x2222_3333_4444_5555);
    let cs = cones(&mut r);
    let mut rows = std::collections::BTreeMap::<usize, (f64, f64, usize)>::new();
    for c in cs.iter().step_by(3) {
        let link = c.link::<f64>();
        let segs: Vec<(V, V)> = (0..500).map(|_| (r.v(), r.v())).collect();
        let t = std::time::Instant::now();
        for &(a, e) in &segs {
            let _ = std::hint::black_box(link.lowest_on((pt(a), pt(e)), b));
        }
        let lo = t.elapsed().as_secs_f64() / 500.0 * 1e6;
        let t = std::time::Instant::now();
        for &(a, _) in &segs {
            let _ = std::hint::black_box(link.offset(pt(a), b));
        }
        let of = t.elapsed().as_secs_f64() / 500.0 * 1e6;
        let e = rows.entry(c.fans.len()).or_insert((0.0, 0.0, 0));
        e.0 = e.0.max(lo);
        e.1 = e.1.max(of);
        e.2 += 1;
    }
    for (k, (lo, of, n)) in rows {
        eprintln!("{k} sectors ({n} cones): lowest_on worst {lo:.1} µs, offset worst {of:.2} µs");
    }
}

/// A hole's corner whose +x-side edge is short: a point inside the
/// material, in band of a plane and beside the short ray, reads its
/// distance with the wrong sign through `touches`.
#[test]
fn probe_wrong_sign_beside_a_short_ray_of_a_hole() {
    let b = band();
    for (s, q) in [
        (1e-3, Vec3::new(-0.39, 9.4e-9, 5.0e-7)),
        (1e-3, Vec3::new(-1.0, 5e-9, 5e-6)),
        (1e-6, Vec3::new(-1.0, 5e-9, 5e-3)),
    ] {
        let c = corner(s).complement();
        let got = c.link::<f64>().offset(pt(q), b);
        let low = c.link::<f64>().lowest_on((pt(q), pt(q + Vec3::new(0.0, 0.0, 1e-9))), b);
        eprintln!("hole, edge {s:e}, q {q:?}: offset {got:?}, lowest_on {low:?}, oracle {:?}", c.sd(q));
    }
}
