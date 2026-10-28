//! **The link against an independent oracle, on ninety cones**: three
//! blocks' corners (one with a millimetre edge), a saddle, four knife
//! edges (90°, 4°, 0.3°, 150°) and eight random convex cones (four with
//! a near-duplicate ray), each turned twice at random, and every one of
//! them complemented, so a reflex cone is read as often as a convex one.
//! The oracle reads the distance as the least over wedges of the
//! distance from the closed planar wedge, and membership by a ray cast
//! to a far point of known membership, in plain `f64`: nothing of the
//! link's own reading. (From the dual review of the link's fix pass,
//! `review/atrest-link-delta`.)

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

// Gated to the reader it checks and the band it decides with.
test_utils::gated_to![
    "crates/topo/src/link.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
];

use geom_core::{Bounds, Dual, Interval};

use super::*;

fn band() -> Band {
    Band::linear(geom_core::Tol::witness()).expect("a band")
}

/// The fuzz stream, in the shapes the draws take.
struct Rng(test_utils::fuzz::Rng);

impl Rng {
    fn u(&mut self) -> f64 {
        self.0.unit()
    }
    fn r(&mut self, lo: f64, hi: f64) -> f64 {
        self.0.range(lo, hi)
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
                wedges: pts
                    .windows(2)
                    .map(|w| (p0 + c(w[0]), p0 + c(w[1])))
                    .collect(),
                closed: false,
                clear: None,
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
        fans: vec![
            (n1, orient(n1, vec![y(), u1, -y()])),
            (n2, orient(n2, vec![y(), u2, -y()])),
        ],
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
        let ang = r.r(0.0, std::f64::consts::TAU);
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
        let it: Vec<_> = if pass == 0 {
            pts.clone()
        } else {
            pts.iter().rev().copied().collect()
        };
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

/// The least of the oracle's signed distance along `a`–`a + e`: a dense
/// sample, refined about its least by a ternary search; `None` where a
/// cast disagrees.
fn least(c: &Cone, a: V, e: V) -> Option<f64> {
    let at = |t: f64| c.sd(a + e * t);
    let n = 1500;
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

fn fail(label: &str, bad: &[String]) {
    assert!(
        bad.is_empty(),
        "{label}: {} wrong; {}:\n{}",
        bad.len(),
        test_utils::fuzz::replay(),
        bad[..bad.len().min(6)].join("\n")
    );
}

/// **`offset`'s sign agrees with the oracle wherever the point stands
/// out of the band**, on all ninety cones, at points near a face's plane,
/// near a ray, and anywhere, at scales from a micrometre to a metre. A
/// refusal is never wrong here. Red if a point more than ten escalations
/// from the boundary reads the wrong sign (a reflex cone's short ray read
/// drawn in did), or its magnitude reads more than two escalations off
/// the distance. Within ten escalations of the boundary the sign may
/// flip: the parity reading decides there through the band.
#[test]
fn offset_agrees_with_the_oracle_on_ninety_cones() {
    let label = "link_cone_oracle_offset";
    let mut r = Rng(test_utils::fuzz::start(label));
    let b = band();
    let mut bad = Vec::new();
    for c in &cones(&mut r) {
        let link = c.link::<f64>();
        for i in 0..test_utils::fuzz::scaled(60) {
            let q = match i % 3 {
                0 => near_plane(&mut r, c),
                1 => near_ray(&mut r, c),
                _ => r.v() * r.log(-6.0, 0.0),
            };
            let (Some(want), Ok(got)) = (c.sd(q), link.offset(pt(q), b)) else {
                continue;
            };
            let wrong_sign = want.abs() > 10.0 * b.escalate() && got.signum() != want.signum();
            if wrong_sign || (got.abs() - want.abs()).abs() > 2.0 * b.escalate() {
                bad.push(format!("{} q {q:?}: {got:e}, not {want:e}", c.name));
            }
        }
    }
    fail(label, &bad);
}

/// **`lowest_on` agrees with the oracle's least along a segment**, on
/// all ninety cones: segments anywhere, through a point near a ray or a
/// face's plane, and nearly in a face's plane. Red if a segment's least
/// reads the wrong sign ten escalations out of the band, or its magnitude
/// off the oracle's by more than two escalations (a candidate read within
/// the band of another, and the in-band branch; with the sample's own
/// resolution).
#[test]
fn lowest_on_agrees_with_the_oracle_on_ninety_cones() {
    let label = "link_cone_oracle_lowest";
    let mut r = Rng(test_utils::fuzz::start(label));
    let b = band();
    let mut bad = Vec::new();
    for c in &cones(&mut r) {
        let link = c.link::<f64>();
        for i in 0..test_utils::fuzz::scaled(4) {
            let s = r.log(-4.0, 0.0);
            let (a, e) = match i % 4 {
                0 => (r.v() * s, r.v() * s * 2.0),
                1 => {
                    let (m, d) = (near_ray(&mut r, c), r.v() * s);
                    (m - d * r.r(0.1, 1.0), d * 2.0)
                }
                2 => {
                    let (m, d) = (near_plane(&mut r, c), r.v() * s);
                    (m - d * r.r(0.1, 1.0), d * 2.0)
                }
                _ => {
                    let m = near_plane(&mut r, c);
                    let (nrm, _) = c.fans[(r.u() * c.fans.len() as f64) as usize % c.fans.len()];
                    let d = r.v();
                    let d = (d - nrm * nrm.dot(d)) * s + nrm * r.sgn() * r.log(-13.0, -6.0);
                    (m - d * 0.5, d)
                }
            };
            let (Some(want), Ok(got)) = (least(c, a, e), link.lowest_on((pt(a), pt(a + e)), b))
            else {
                continue;
            };
            let wrong_sign = want.abs() > 10.0 * b.escalate() && got.signum() != want.signum();
            if wrong_sign || (got.abs() - want.abs()).abs() > 1e-6 * e.norm() + 2.0 * b.escalate() {
                bad.push(format!("{} {a:?} + {e:?}: {got:e}, not {want:e}", c.name));
            }
        }
    }
    fail(label, &bad);
}

/// **An Interval reading over a box encloses the f64 reading at every
/// sampled point of it, and is never of the wrong sign**, for `offset`
/// and for `lowest_on` over a segment of boxes, on all ninety cones,
/// with boxes from a picometre to a centimetre, some thin in one
/// coordinate. Red if an enclosure misses a point's value, or reads one
/// sign where a point stands ten escalations to the other side.
#[test]
fn interval_readings_enclose_their_boxes_on_ninety_cones() {
    let label = "link_cone_oracle_interval";
    let mut r = Rng(test_utils::fuzz::start(label));
    let b = band();
    let mut bad = Vec::new();
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
    for c in &cones(&mut r) {
        let (li, lf) = (c.link::<Interval>(), c.link::<f64>());
        for i in 0..test_utils::fuzz::scaled(4) {
            let ctr = match i % 3 {
                0 => near_plane(&mut r, c),
                1 => near_ray(&mut r, c),
                _ => r.v() * r.log(-4.0, 0.0),
            };
            let s = r.log(-12.0, -2.0);
            let mut w = Vec3::new(s, s, s);
            if i % 2 == 0 {
                w.y *= 1e-4;
            }
            if let Ok(got) = li.offset(ib(ctr, w), b) {
                for k in 0..12 {
                    let q = if k == 0 { ctr } else { sample(&mut r, ctr, w) };
                    if let Ok(v) = lf.offset(pt(q), b)
                        && !(got.lo() <= v && v <= got.hi())
                    {
                        bad.push(format!(
                            "{} box {ctr:?}±{w:?}: [{:e}, {:e}] misses {v:e}",
                            c.name,
                            got.lo(),
                            got.hi()
                        ));
                    }
                    if let Some(sd) = c.sd(q)
                        && sd.abs() > 10.0 * b.escalate()
                        && ((got.lo() > 0.0 && sd < 0.0) || (got.hi() < 0.0 && sd > 0.0))
                    {
                        bad.push(format!(
                            "{} box {ctr:?}±{w:?}: [{:e}, {:e}] vs {sd:e}",
                            c.name,
                            got.lo(),
                            got.hi()
                        ));
                    }
                }
            }
            let d = r.v() * r.log(-4.0, 0.0);
            let (c0, c1) = (ctr - d * 0.5, ctr + d * 0.5);
            if let Ok(got) = li.lowest_on((ib(c0, w), ib(c1, w)), b) {
                for _ in 0..4 {
                    let (a, e) = (sample(&mut r, c0, w), sample(&mut r, c1, w));
                    if let Ok(v) = lf.lowest_on((pt(a), pt(e)), b)
                        && !(got.lo() <= v && v <= got.hi())
                    {
                        bad.push(format!(
                            "{} segment {c0:?}->{c1:?}±{w:?}: [{:e}, {:e}] misses {v:e}",
                            c.name,
                            got.lo(),
                            got.hi()
                        ));
                    }
                }
            }
        }
    }
    fail(label, &bad);
}

/// **Beside a hole's short ray the offset reads its sign**: an octant
/// hole's corner whose `−x` edge is 1 mm or 1 µm long, read at points
/// past the edge's real point, in band of its face's plane, a decided
/// distance inside the material. Read against the edge's point, the
/// side of the edge shrank into the band, the place in the face went
/// undecided, and the touch read `+5e-3` where the truth is `−5e-3`. Red
/// if one reads the wrong sign or off its distance by an escalation.
#[test]
fn beside_a_holes_short_ray_the_offset_reads_its_sign() {
    let b = band();
    for (s, q) in [
        (1e-3, Vec3::new(-0.39, 9.4e-9, 5.0e-7)),
        (1e-3, Vec3::new(-1.0, 5e-9, 5e-6)),
        (1e-6, Vec3::new(-1.0, 5e-9, 5e-3)),
    ] {
        let c = corner(s).complement();
        let want = c.sd(q).expect("the oracle reads");
        let link = c.link::<f64>();
        let got = link.offset(pt(q), b).expect("the offset reads");
        assert!(
            (got - want).abs() <= b.escalate(),
            "edge {s:e}, {q:?}: {got:e}, not {want:e}"
        );
        let e = Vec3::new(0.0, 0.0, 1e-9);
        let low = link
            .lowest_on((pt(q), pt(q + e)), b)
            .expect("the segment reads");
        let least = least(&c, q, e).expect("the oracle reads");
        assert!(
            (low - least).abs() <= b.escalate(),
            "edge {s:e}, segment at {q:?}: {low:e}, not {least:e}"
        );
    }
}

/// **At contact a derivative is a face's, on knives turned about any
/// axis**: a vertex on a 90°, 4° or 150° knife edge moved in a direction,
/// and a segment across the edge moved with it, at `Dual`, for each knife
/// and its complement, as laid and turned five ways. The turns are a
/// fixed table (this seed's draws): a witness set, not a search. A tie
/// at contact takes a face's plane, whose derivative is the face's
/// normal along the move, and the least along a segment is read at its
/// candidate as a point (the envelope's derivative). Red if a derivative
/// is not finite or not of a unit's size, or a vertex at exact contact
/// reads a derivative no face's normal gives (a ray's norm at zero, or a
/// root's square root at a double root).
#[test]
fn at_contact_a_derivative_is_a_faces_on_turned_knives() {
    type D = Dual<f64>;
    let b = band();
    // A fixture identifier: the turns and moves these draws give.
    let mut r = Rng(test_utils::fuzz::pinned(
        "link_cone_oracle_dual",
        0xabcd_ef01_2345_6789,
    ));
    let mut bad = Vec::new();
    for th in [90.0f64, 4.0, 150.0] {
        for k in 0..6 {
            let base = knife(th.to_radians());
            let rr = if k == 0 { [x(), y(), z()] } else { rot(&mut r) };
            for c in [base.turned(rr), base.complement().turned(rr)] {
                let link = c.link::<D>();
                let m = |v: V| rr[0] * v.x + rr[1] * v.y + rr[2] * v.z;
                let dv = |v: V| Vec3::new(D::from_f64(v.x), D::from_f64(v.y), D::from_f64(v.z));
                let origin = Point3::new(D::zero(), D::zero(), D::zero());
                let h = D::variable(0.0);
                let dir = r.v();
                let q = origin + dv(m(y() * 0.3)) + dv(dir) * h;
                let normals: Vec<f64> = c.fans.iter().map(|(n, _)| n.dot(dir)).collect();
                match link.offset(q, b) {
                    Ok(v) if v.deriv.is_finite() && v.deriv.abs() < 10.0 => {
                        if v.value == 0.0 && !normals.iter().any(|n| (n - v.deriv).abs() < 1e-12) {
                            bad.push(format!(
                                "{} turn {k} vertex: derivative {} is no face's {normals:?}",
                                c.name, v.deriv
                            ));
                        }
                    }
                    got => bad.push(format!("{} turn {k} vertex: {got:?}", c.name)),
                }
                let seg_d = m(y() * 0.1 + z() * 0.7);
                let a = origin + dv(m(y() * 0.3) - seg_d) + dv(dir) * h;
                match link.lowest_on((a, a + dv(seg_d * 2.0)), b) {
                    Ok(v) if v.deriv.is_finite() && v.deriv.abs() < 10.0 => {}
                    got => bad.push(format!("{} turn {k} segment: {got:?}", c.name)),
                }
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} derivatives:\n{}",
        bad.len(),
        bad.join("\n")
    );
}
