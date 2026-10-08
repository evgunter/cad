//! **The polygon-cone reader against an exact oracle**: random cones
//! nobody chose, read by [`cone_side`] and [`wedge_classes`], each
//! decided class checked against an exact rational reading of the same
//! cone.
//!
//! **What it certifies.** Every `cone_side` reading, on every family.
//! Every `wedge_classes` reading but the long-probe family's: there its
//! convex branch reads a corner whose reflex edge is flat within the
//! band at its short bounds as convex, and classes long edges wrong far
//! out of band (CLEAVE's
//! `wedge-classes-reads-a-corner-flat-at-its-short-bounds-as-convex`,
//! P1, pre-existing). That column is counted and printed as known
//! wrong, not asserted.
//!
//! The cones are stars about an axis, their corners risen and fallen
//! at random, some with two corners a hair apart (a thin fin or
//! sliver), some turned off the axis; the graze family of PR 4289's
//! second review: a flat corner with one face dented a hair, read by
//! directions a hair off the flat faces; its third review's bound
//! family: a short bound, read by directions a hair past it; and its
//! fourth review's long-probe family: a dart over 1 mm or 1 cm bounds,
//! its reflex edge dented a hair, read by edges up to 1 km long. Every
//! bound is a line edge, its far point an `f64` vector, and every
//! direction a line edge too.
//!
//! The oracle reads the same `f64` numbers exactly, as rationals: a
//! direction is on the cone where it lies on a face's plane within its
//! sector, and otherwise inside it where the segment from its far point
//! to a reference point crosses the faces' sectors an even number of
//! times. The reference lies just inside one face, off every other, by
//! an exact step; a segment through a sector's edge or along a plane is
//! passed over for a nudged reference. None of it shares the reader's
//! rounding, so a decided class the oracle contradicts is a defect, not
//! a rounding.
//!
//! **In band, by D4.** A direction is in band where some deviation of
//! the points the cone and the probe are read from, each moved by no
//! more than the zero band, puts the probe's far point `D` on the
//! cone's boundary, so that the exact class can flip. A face is its
//! plane, as the reader holds it (a surface, not three points), and
//! its sector is bounded in that plane by its bounds, each a far point
//! at its own reach `L`. Moving `D` by `δ` moves it off the plane by
//! `δ`; moving a bound's far point by `δ` turns the bound in the plane
//! by `δ/L`, which carries the bound's ray, a distance `t` out, by
//! `t·δ/L`. So where `D` projects into a sector, the least such `δ` is
//! `D`'s height `h` over the plane; elsewhere it is the larger of `h`
//! and `w/(1 + t/L)`, `w` the projection's distance from the nearer
//! bound's ray and `t` its distance along it. The probe is in band
//! where that is within the zero band for some face, or it lies exactly
//! on the cone.
//!
//! **Two face models, one cone.** The oracle (`Oracle::new`) reads the
//! cone as the faces' planes and the bounds' far points exactly;
//! `least_flip` holds each face's plane and turns its bounds in it.
//! On a valid cone, unmoved (every bound on its faces' planes within
//! the band, which is checked), the two read the same cone; they differ
//! only on moved cones, which only `least_flip` reads.
//!
//! A reading is **wrong** where it is a decided class (`In` or `Out`)
//! the oracle contradicts, or a decided class in band (a band-sized
//! move could flip it, so deciding it is no reading of the input), or
//! `On` out of band. An `On`, a refusal or `None` in band is the
//! reader's answer there. `None` and refusals are counted and printed.
//!
//! It reads at ε = 1e-9, 1e-6 and 1e-12, and at the session's ε where
//! that is another, each cone that is one at that ε: a face whose
//! bounds stand off its own plane beyond the zero band is skipped, and
//! only readings checked are counted.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the reader it checks, and the band it decides in.
test_utils::gated_to![
    "crates/topo/src/boolean/sectors.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
];

use super::*;
use geom_core::Tol;
use num_bigint::BigInt;
use test_utils::fuzz::Rng;

/// An exact rational, `n / d` with `d > 0`.
#[derive(Clone, Debug)]
struct Q {
    n: BigInt,
    d: BigInt,
}

impl Q {
    fn int(n: i64) -> Self {
        Self {
            n: BigInt::from(n),
            d: BigInt::from(1),
        }
    }

    /// The exact value of a finite `f64`.
    fn of(x: f64) -> Self {
        assert!(x.is_finite(), "the fuzz draws finite numbers");
        if x == 0.0 {
            return Self::int(0);
        }
        let bits = x.to_bits();
        let sign = if bits >> 63 == 1 { -1 } else { 1 };
        let exp = ((bits >> 52) & 0x7ff) as i64;
        let frac = bits & ((1u64 << 52) - 1);
        let (mant, e) = if exp == 0 {
            (frac, -1074)
        } else {
            (frac | (1u64 << 52), exp - 1075)
        };
        let m = BigInt::from(mant) * sign;
        if e >= 0 {
            Self {
                n: m << (e as usize),
                d: BigInt::from(1),
            }
        } else {
            Self {
                n: m,
                d: BigInt::from(1) << ((-e) as usize),
            }
        }
    }

    fn add(&self, o: &Self) -> Self {
        Self {
            n: &self.n * &o.d + &o.n * &self.d,
            d: &self.d * &o.d,
        }
    }

    fn sub(&self, o: &Self) -> Self {
        Self {
            n: &self.n * &o.d - &o.n * &self.d,
            d: &self.d * &o.d,
        }
    }

    fn mul(&self, o: &Self) -> Self {
        Self {
            n: &self.n * &o.n,
            d: &self.d * &o.d,
        }
    }

    /// `self / o`, `o` nonzero.
    fn div(&self, o: &Self) -> Self {
        let (n, d) = (&self.n * &o.d, &self.d * &o.n);
        if d.sign() == num_bigint::Sign::Minus {
            Self { n: -n, d: -d }
        } else {
            Self { n, d }
        }
    }

    fn sign(&self) -> i32 {
        match self.n.sign() {
            num_bigint::Sign::Minus => -1,
            num_bigint::Sign::NoSign => 0,
            num_bigint::Sign::Plus => 1,
        }
    }
}

type V = [Q; 3];

fn qv(v: [f64; 3]) -> V {
    v.map(Q::of)
}

fn cross(a: &V, b: &V) -> V {
    [
        a[1].mul(&b[2]).sub(&a[2].mul(&b[1])),
        a[2].mul(&b[0]).sub(&a[0].mul(&b[2])),
        a[0].mul(&b[1]).sub(&a[1].mul(&b[0])),
    ]
}

fn dot(a: &V, b: &V) -> Q {
    a[0].mul(&b[0]).add(&a[1].mul(&b[1])).add(&a[2].mul(&b[2]))
}

fn det(a: &V, b: &V, c: &V) -> Q {
    dot(a, &cross(b, c))
}

fn vadd(a: &V, b: &V) -> V {
    [a[0].add(&b[0]), a[1].add(&b[1]), a[2].add(&b[2])]
}

fn vscale(a: &V, s: &Q) -> V {
    [a[0].mul(s), a[1].mul(s), a[2].mul(s)]
}

/// A face's corner as the oracle reads it: its two bounds, from
/// `start` round to `end`, each under 180°.
struct Wedge {
    start: V,
    end: V,
    /// `+1` where the outward normal is along `start × end`.
    orient: i32,
}

/// The exact reading of one cone.
struct Oracle {
    wedges: Vec<Wedge>,
    reference: V,
}

impl Oracle {
    fn new(cone: &[Sector]) -> Option<Self> {
        let wedges: Vec<Wedge> = cone
            .iter()
            .map(|s| {
                let (start, end) = (qv(s.start_far), qv(s.end_far));
                let orient = dot(&cross(&start, &end), &qv(s.normal)).sign();
                Wedge { start, end, orient }
            })
            .collect();
        // Just inside one face, by an exact step that halves until the
        // segment from the face's middle crosses no other face.
        for (k, w) in wedges.iter().enumerate() {
            let middle = vadd(&w.start, &w.end);
            let inner = vscale(&cross(&w.start, &w.end), &Q::int(-i64::from(w.orient)));
            let mut eta = Q {
                n: BigInt::from(1),
                d: BigInt::from(64),
            };
            for _ in 0..40 {
                let r = vadd(&middle, &vscale(&inner, &eta));
                if Self::crossings_of(&wedges, &middle, &r, Some(k)) == Some(0) {
                    return Some(Self {
                        wedges,
                        reference: r,
                    });
                }
                eta = eta.mul(&Q {
                    n: BigInt::from(1),
                    d: BigInt::from(4),
                });
            }
        }
        None
    }

    /// The parity of the segment `q → r`'s crossings of the wedges, or
    /// `None` where it meets one at an edge or lies along a plane.
    fn crossings_of(wedges: &[Wedge], q: &V, r: &V, skip: Option<usize>) -> Option<u32> {
        let d = [r[0].sub(&q[0]), r[1].sub(&q[1]), r[2].sub(&q[2])];
        let mut parity = 0;
        for (k, w) in wedges.iter().enumerate() {
            if Some(k) == skip {
                continue;
            }
            let (ms, me) = (vscale(&w.start, &Q::int(-1)), vscale(&w.end, &Q::int(-1)));
            let big = det(&d, &ms, &me);
            if big.sign() == 0 {
                if det(q, &w.start, &w.end).sign() == 0 {
                    return None;
                }
                continue;
            }
            let nq = vscale(q, &Q::int(-1));
            let t = det(&nq, &ms, &me).div(&big);
            let a = det(&d, &nq, &me).div(&big);
            let b = det(&d, &ms, &nq).div(&big);
            if t.sign() < 0 || t.sub(&Q::int(1)).sign() > 0 || a.sign() < 0 || b.sign() < 0 {
                continue;
            }
            if t.sign() == 0 || t.sub(&Q::int(1)).sign() == 0 || a.sign() == 0 || b.sign() == 0 {
                return None;
            }
            parity ^= 1;
        }
        Some(parity)
    }

    /// Whether `q` lies on a face's plane within its sector.
    fn on(&self, q: &V) -> bool {
        self.wedges.iter().any(|w| {
            det(q, &w.start, &w.end).sign() == 0 && {
                let se = cross(&w.start, &w.end);
                dot(&cross(q, &w.end), &se).sign() >= 0 && dot(&cross(&w.start, q), &se).sign() >= 0
            }
        })
    }

    fn class(&self, far: [f64; 3], rng: &mut Rng) -> SideCode {
        let q = qv(far);
        if self.on(&q) {
            return SideCode::On;
        }
        let mut reference = self.reference.clone();
        for _ in 0..50 {
            if let Some(parity) = Self::crossings_of(&self.wedges, &q, &reference, None) {
                return if parity == 0 {
                    SideCode::In
                } else {
                    SideCode::Out
                };
            }
            // A nudged reference, itself still inside: no face between.
            let nudge = [0, 1, 2].map(|_| Q {
                n: BigInt::from(rng.below(2001) as i64 - 1000),
                d: BigInt::from(1u64 << 40),
            });
            let nudged = vadd(&self.reference, &nudge);
            if Self::crossings_of(&self.wedges, &self.reference, &nudged, None) == Some(0) {
                reference = nudged;
            }
        }
        panic!("no reference reads {far:?}")
    }
}

/// One face's corner as drawn: its bounds' far points (each a line
/// edge), its outward normal, its arm, its face.
#[derive(Clone)]
struct Sector {
    start_far: [f64; 3],
    end_far: [f64; 3],
    normal: [f64; 3],
    arm: f64,
    face: u64,
}

fn norm3(v: [f64; 3]) -> [f64; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    v.map(|x| x / l)
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// The corners of the cone over `ring`, each corner `(direction,
/// chord length)`, the material on the left of the ring (inside a cone
/// over a ring counterclockwise from its axis) or its complement where
/// `hollow`.
fn sectors_from_ring(ring: &[([f64; 3], f64)], hollow: bool) -> Vec<Sector> {
    let n = ring.len();
    let unit: Vec<[f64; 3]> = ring.iter().map(|&(d, _)| norm3(d)).collect();
    let far = |k: usize| unit[k].map(|x| x * ring[k].1);
    (0..n)
        .map(|k| {
            let j = (k + 1) % n;
            let (start, end) = if hollow { (k, j) } else { (j, k) };
            Sector {
                start_far: far(start),
                end_far: far(end),
                normal: norm3(cross3(unit[start], unit[end])),
                arm: ring[k].1.min(ring[j].1),
                face: k as u64,
            }
        })
        .collect()
}

fn bool_sectors(cone: &[Sector]) -> Vec<BoolSector<f64>> {
    let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
    let o = Point3::new(0.0, 0.0, 0.0);
    cone.iter()
        .enumerate()
        .map(|(k, s)| {
            let v = |a: [f64; 3]| Vec3::new(a[0], a[1], a[2]);
            BoolSector {
                he: HalfEdgeKey::from(key(k as u64 + 1)),
                start: v(s.start_far).normalize(),
                end: v(s.end_far).normalize(),
                start_reach: Reach::Chord {
                    base: o,
                    far: o + v(s.start_far),
                },
                end_reach: Reach::Chord {
                    base: o,
                    far: o + v(s.end_far),
                },
                face: FaceKey::from(key(1000 + s.face)),
                normal: OutwardNormal::from_chart(v(s.normal), true),
                arm: s.arm,
            }
        })
        .collect()
}

/// The least deviation of the cone's far points and the probe's that
/// puts the probe's far point `far` on the cone's boundary (module
/// docs, "In band, by D4").
fn least_flip(cone: &[Sector], far: [f64; 3]) -> f64 {
    let dot = |x: [f64; 3], y: [f64; 3]| x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
    let len = |x: [f64; 3]| dot(x, x).sqrt();
    cone.iter()
        .map(|s| {
            let n = s.normal;
            let h = dot(far, n);
            let pr = [0, 1, 2].map(|i| far[i] - h * n[i]);
            let (u, v) = (norm3(s.start_far), norm3(s.end_far));
            if dot(cross3(u, pr), n) >= 0.0 && dot(cross3(pr, v), n) >= 0.0 {
                return h.abs();
            }
            [s.start_far, s.end_far]
                .iter()
                .map(|&bound| {
                    let (b, l_b) = (norm3(bound), len(bound));
                    let t = dot(pr, b).max(0.0);
                    let w = len([0, 1, 2].map(|i| pr[i] - t * b[i]));
                    h.abs().max(w / (1.0 + t / l_b))
                })
                .fold(f64::MAX, f64::min)
        })
        .fold(f64::MAX, f64::min)
}

/// A cone's corners, in order round it: each a unit bound and the
/// length of its edge.
type Ring = Vec<([f64; 3], f64)>;

/// A star of `n` corners about the axis `z`, risen between `lo` and
/// `hi`, its largest gap under 180°, two corners perhaps a hair apart,
/// perhaps turned off the axis. `None` where the gaps do not allow it.
fn star(rng: &mut Rng, n: usize, lo: f64, hi: f64, thin: bool, turn: bool) -> Option<Ring> {
    let tau = std::f64::consts::TAU;
    let mut th: Vec<f64> = (0..n).map(|_| rng.range(0.0, tau)).collect();
    th.sort_by(f64::total_cmp);
    if thin {
        let k = rng.below(n - 1);
        th[k + 1] = th[k] + [1e-3, 1e-6, 1e-8, 3e-9][rng.below(4)];
        th.sort_by(f64::total_cmp);
    }
    let widest = (0..n)
        .map(|i| (th[(i + 1) % n] - th[i]).rem_euclid(tau))
        .fold(0.0, f64::max);
    if widest >= 0.98 * std::f64::consts::PI {
        return None;
    }
    let (axis, angle) = if turn {
        (
            norm3([
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            ]),
            rng.range(0.0, 3.0),
        )
    } else {
        ([0.0, 0.0, 1.0], 0.0)
    };
    let rot = |p: [f64; 3]| {
        let (s, c) = angle.sin_cos();
        let kxp = cross3(axis, p);
        let kd = axis[0] * p[0] + axis[1] * p[1] + axis[2] * p[2];
        [0, 1, 2].map(|i| p[i] * c + kxp[i] * s + axis[i] * kd * (1.0 - c))
    };
    Some(
        th.iter()
            .map(|&t| {
                let z = if rng.unit() < 0.15 {
                    [lo, hi][rng.below(2)]
                } else {
                    rng.range(lo, hi)
                };
                (rot([t.cos(), t.sin(), z]), rng.range(0.5, 2.0))
            })
            .collect(),
    )
}

/// Review 2's graze family: a flat corner, one face's far corner dented
/// `dz` beside a short edge, read by directions a hair off the flat.
fn graze(rng: &mut Rng) -> (Ring, Vec<[f64; 3]>) {
    let pt = |a: f64, z: f64| [a.cos(), a.sin(), z];
    let pick = |rng: &mut Rng, xs: &[f64]| xs[rng.below(xs.len())];
    let sign = |rng: &mut Rng| if rng.below(2) == 0 { 1.0 } else { -1.0 };
    let a1 = pick(rng, &[0.03, 0.04, 0.045, 0.0, -0.1, -0.3]);
    let dz = pick(rng, &[1e-9, 3e-9, 1e-8, 5e-8, 1e-7, 1.8e-7, 3e-7]) * sign(rng);
    let la = pick(rng, &[0.01, 0.001, 0.1, 1.0]);
    let lr = pick(rng, &[1.0, 0.5, 2.0]);
    let zc = pick(rng, &[0.3, -0.3, 0.0, 0.5]);
    let ring = vec![
        (pt(a1, dz), la),
        (pt(0.05, 0.0), la),
        (pt(0.5, 0.0), lr),
        (pt(2.0, zc), lr),
        (pt(3.5, -zc), lr),
        (pt(5.0, zc * 0.5), lr),
    ];
    let probes = (0..12)
        .map(|_| {
            let th = rng.range(-0.5, 1.6);
            let h = pick(rng, &[1e-8, 2e-8, 3e-8, 5e-8, 1e-7, 1e-6]) * sign(rng);
            let l = pick(rng, &[0.5, 1.0, 2.0]);
            norm3(pt(th, h)).map(|x| x * l)
        })
        .collect();
    (ring, probes)
}

/// Review 3's bound family: a crown whose face on `z = 0` has a short
/// edge at bearing 0.05, read by directions a hair past or before that
/// bound, on or a hair off the plane.
fn beside_bound(rng: &mut Rng) -> (Ring, Vec<[f64; 3]>) {
    let pt = |a: f64, z: f64| [a.cos(), a.sin(), z];
    let pick = |rng: &mut Rng, xs: &[f64]| xs[rng.below(xs.len())];
    let za = pick(rng, &[0.2, -0.2, 0.05, -0.05]);
    let short = pick(rng, &[1e-3, 1e-2, 1e-4]);
    let at = 0.05 + pick(rng, &[0.0, 3e-7, -3e-7]);
    let ring = vec![
        (pt(-0.3, za), 1.0),
        (pt(at, 0.0), short),
        (pt(0.5, 0.0), 1.0),
        (pt(2.0, 0.3), 1.0),
        (pt(3.5, -0.3), 1.0),
        (pt(5.0, 0.15), 1.0),
    ];
    let probes = (0..12)
        .map(|_| {
            let past = pick(rng, &[1e-7, 2e-7, 4e-7, 1e-6, -1e-7, -1e-6]);
            let h = pick(rng, &[0.0, 1e-10, -1e-10, 4e-10, -4e-10, 2e-9, -2e-9]);
            let l = pick(rng, &[0.5, 1.0, 2.0]);
            norm3(pt(0.05 - past, h)).map(|x| x * l)
        })
        .collect();
    (ring, probes)
}

/// Review 4's long-probe family: a dart of four faces over 1 mm or
/// 1 cm bounds, one edge reflex by a dent `g` within the band or a few
/// bands at those bounds, read by edges 1 m, 50 m and 1 km long a hair
/// above or below the flat (`L_d/L_b` up to a million).
fn dart_long(rng: &mut Rng) -> (Ring, Vec<[f64; 3]>) {
    let pick = |rng: &mut Rng, xs: &[f64]| xs[rng.below(xs.len())];
    let g = pick(rng, &[1e-5, -1e-5, 1e-6, -1e-6, 3e-7, -3e-7]);
    let lb = pick(rng, &[1e-3, 1e-2]);
    let ring = vec![
        ([1.0, 0.0, 0.0], lb),
        ([0.0, 1.0, g], lb),
        ([-1.0, 0.0, 0.0], lb),
        ([0.0, -1.0, 0.0], lb),
    ];
    let probes = (0..12)
        .map(|_| {
            let ld = pick(rng, &[1.0, 50.0, 1000.0]);
            let az = rng.range(0.05, std::f64::consts::PI - 0.05);
            let sign = if rng.below(2) == 0 { 1.0 } else { -1.0 };
            let el = sign * pick(rng, &[3e-9, 1e-8, 3e-8, 1e-7, 1e-6, 1e-5]) * rng.range(0.5, 2.0)
                / ld
                * 1e3;
            norm3([az.cos() * el.cos(), az.sin() * el.cos(), el.sin()]).map(|x| x * ld)
        })
        .collect();
    (ring, probes)
}

/// Directions about a cone: random ones, and ones on, beside and a
/// hair off its faces and bounds, each a far point at a random length.
fn probes_about(rng: &mut Rng, cone: &[Sector]) -> Vec<[f64; 3]> {
    let mut out = Vec::new();
    let mut add = |rng: &mut Rng, d: [f64; 3]| {
        let l = rng.range(0.5, 2.0);
        out.push(norm3(d).map(|x| x * l));
    };
    for _ in 0..6 {
        let d = [
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        ];
        add(rng, d);
    }
    for _ in 0..2 {
        let s = &cone[rng.below(cone.len())];
        let (a, b) = (norm3(s.start_far), norm3(s.end_far));
        let (wa, wb) = (rng.unit(), rng.unit());
        let inside = norm3([0, 1, 2].map(|i| a[i] * wa + b[i] * wb));
        let outside = norm3([0, 1, 2].map(|i| -a[i] * wa + b[i] * wb * 0.3));
        for base in [inside, outside] {
            let off = [0.0, 3e-9, -3e-9, 2e-8, -2e-8, 1e-6, -1e-6][rng.below(7)];
            add(rng, [0, 1, 2].map(|i| base[i] + s.normal[i] * off));
        }
        add(rng, a.map(|x| -x));
        let jitter = [0; 3].map(|_| rng.range(-1e-7, 1e-7));
        add(rng, [0, 1, 2].map(|i| b[i] + jitter[i]));
    }
    out
}

/// The exact reading of `cone`, where it is one at `band`: a face
/// whose bounds stand off its own plane beyond the zero band is no face
/// at this ε, since a sector a few nanoradians wide has its normal
/// drawn in floating point to about 1e-16 over its width.
fn cone_oracle(cone: &[Sector], band: Band) -> Option<Oracle> {
    let planar = cone.iter().all(|s| {
        let n = qv(s.normal);
        [s.start_far, s.end_far].iter().all(|&far| {
            let off = dot(&qv(far), &n);
            let zero = Q::of(band.zero());
            off.sub(&zero).sign() <= 0 && off.add(&zero).sign() >= 0
        })
    });
    planar.then(|| Oracle::new(cone)).flatten()
}

/// A probe edge to the far point `far`, as the one sector of the vertex
/// read whose edge it ends.
fn probe_sector(far: [f64; 3]) -> BoolSector<f64> {
    let o = Point3::new(0.0, 0.0, 0.0);
    let d = Vec3::new(far[0], far[1], far[2]);
    BoolSector {
        he: HalfEdgeKey::from(slotmap::KeyData::from_ffi((1 << 32) | 5000)),
        start: Vec3::new(0.0, 0.0, 1.0),
        end: d.normalize(),
        start_reach: Reach::Bisector(1.0),
        end_reach: Reach::Chord {
            base: o,
            far: o + d,
        },
        face: FaceKey::from(slotmap::KeyData::from_ffi((1 << 32) | 99_999)),
        normal: OutwardNormal::from_chart(Vec3::new(1.0, 0.0, 0.0), true),
        arm: 1.0,
    }
}

/// One cone's reading against the oracle; returns the wrong classes'
/// descriptions, and counts what was not decided.
fn check_cone(
    what: &str,
    cone: &[Sector],
    probes: &[[f64; 3]],
    band: Band,
    rng: &mut Rng,
    counts: &mut [Counts; 2],
    wedge_known_wrong: bool,
) -> Vec<String> {
    let Some(oracle) = cone_oracle(cone, band) else {
        return Vec::new();
    };
    let sectors = bool_sectors(cone);
    let mut wrong = Vec::new();
    for (k, &far) in probes.iter().enumerate() {
        let probe = probe_sector(far);
        let (d, reach) = (Vec3::new(far[0], far[1], far[2]), probe.end_reach);
        let readings = [
            ("cone_side", cone_side(d.normalize(), reach, &sectors, band)),
            (
                "wedge_classes",
                wedge_classes(std::slice::from_ref(&probe), &sectors, band)
                    .map(|r| r.map(|r| r.rows[0].1)),
            ),
        ];
        let truth = oracle.class(far, rng);
        let flip = least_flip(cone, far);
        let in_band = truth == SideCode::On || flip <= band.zero();
        for ((reader, got), counts) in readings.into_iter().zip(counts.iter_mut()) {
            counts.read += 1;
            let got = match got {
                Ok(Some(got)) => got,
                Ok(None) => {
                    counts.unread += 1;
                    continue;
                }
                Err(_) => {
                    counts.refused += 1;
                    continue;
                }
            };
            let defect = match got {
                SideCode::On => (!in_band).then_some("On out of band"),
                _ if got != truth => Some("the oracle contradicts it"),
                _ => in_band.then_some("decided in band"),
            };
            if let Some(defect) = defect {
                if wedge_known_wrong && reader == "wedge_classes" {
                    counts.known_wrong += 1;
                    continue;
                }
                wrong.push(format!(
                    "{what}, probe {k} {far:?}: {reader} reads {got:?}, exactly {truth:?}, \
                     flipped by a {flip:.3e} m deviation: {defect}"
                ));
            }
        }
    }
    wrong
}

/// Whether two wedges share a ray off the vertex: the line their
/// planes meet in lies, one way or the other, within both, closed; two
/// wedges on one plane are read as sharing one.
fn shares_ray(f: &Wedge, g: &Wedge) -> bool {
    let (cf, cg) = (cross(&f.start, &f.end), cross(&g.start, &g.end));
    let l = cross(&cf, &cg);
    if l.iter().all(|x| x.sign() == 0) {
        return true;
    }
    let within = |w: &Wedge, c: &V, x: &V| {
        dot(&cross(&w.start, x), c).sign() >= 0 && dot(&cross(x, &w.end), c).sign() >= 0
    };
    let back = vscale(&l, &Q::int(-1));
    [&l, &back]
        .iter()
        .any(|x| within(f, &cf, x) && within(g, &cg, x))
}

/// A small convex cone in the void of the cone `oracle` reads, a second
/// partner of the same solid: its boundary meets the cone's only at the
/// vertex, one of its corners lies in the void, and a bound of the cone
/// lies outside it, all read exactly; `None` where the draw fails that.
fn partner_in_void(
    rng: &mut Rng,
    cone: &[Sector],
    oracle: &Oracle,
) -> Option<(Vec<Sector>, Oracle)> {
    let q = norm3([
        rng.range(-1.0, 1.0),
        rng.range(-1.0, 1.0),
        rng.range(-1.0, 1.0),
    ]);
    let r = [0.3, 0.05, 1e-3][rng.below(3)];
    let e1 = norm3(cross3(
        q,
        if q[0].abs() < 0.9 {
            [1.0, 0.0, 0.0]
        } else {
            [0.0, 1.0, 0.0]
        },
    ));
    let e2 = cross3(q, e1);
    let ring: Ring = (0..3)
        .map(|k| {
            let t = std::f64::consts::TAU * f64::from(k) / 3.0 + rng.range(-0.3, 0.3);
            let (sn, cs) = t.sin_cos();
            let d = [0, 1, 2].map(|i| q[i] + r * (cs * e1[i] + sn * e2[i]));
            (norm3(d), rng.range(0.5, 2.0))
        })
        .collect();
    let mut partner = sectors_from_ring(&ring, false);
    for s in &mut partner {
        s.face += 100;
    }
    let own = Oracle::new(&partner)?;
    let apart = oracle.class(partner[0].start_far, rng) == SideCode::Out
        && own.class(cone[0].start_far, rng) == SideCode::Out
        && !oracle
            .wedges
            .iter()
            .any(|f| own.wedges.iter().any(|g| shares_ray(f, g)));
    apart.then_some((partner, own))
}

/// **A vertex in pairs alone is read against both partners together,
/// or refuses**: `cone` and a small convex cone in its void
/// ([`partner_in_void`]), two partners of one solid at the vertex, read
/// by [`pair_classes`] for each probe edge against the exact reading of
/// their union. Where a partner reads nothing beside the other it must
/// refuse; elsewhere, a row is wrong as in [`check_cone`], and a probe
/// with no row is wrong. A layering that leaves the probe undecided and
/// keeps each pair's rows is counted, and its wrong rows with it, not
/// asserted
/// (`work/tang/pair-classes-keeps-per-pair-rows-where-its-layering-is-undecided.md`);
/// so is a wrong row of the long-probe family, whose `wedge_classes`
/// reading of the cone is known wrong (module docs).
fn check_pair(
    what: &str,
    cone: &[Sector],
    band: Band,
    rng: &mut Rng,
    counts: &mut PairCounts,
    wedge_known_wrong: bool,
) -> Vec<String> {
    let Some(oracle) = cone_oracle(cone, band) else {
        return Vec::new();
    };
    let Some((partner, _)) = partner_in_void(rng, cone, &oracle) else {
        return Vec::new();
    };
    let both: Vec<Sector> = cone.iter().chain(&partner).cloned().collect();
    let Some(union) = Oracle::new(&both) else {
        return Vec::new();
    };
    counts.placed += 1;
    let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
    let vertex = VertexKey::from(key(7000));
    let others = [bool_sectors(cone), bool_sectors(&partner)];
    let mut probes = probes_about(rng, cone);
    probes.extend(probes_about(rng, &partner));
    let mut wrong = Vec::new();
    for (k, &far) in probes.iter().enumerate() {
        let probe = probe_sector(far);
        let mut pairs = Vec::new();
        for (m, other) in others.iter().enumerate() {
            match wedge_classes(std::slice::from_ref(&probe), other, band) {
                Ok(read) => pairs.push(crate::boolean::vtxfac::PairRead {
                    partner: VertexKey::from(key(7001 + m as u64)),
                    side: None,
                    read,
                    sectors: other.clone(),
                }),
                Err(_) => break,
            }
        }
        if pairs.len() < 2 {
            counts.refused += 1;
            continue;
        }
        let unread = pairs.iter().any(|p| p.read.is_none());
        let rows = match crate::boolean::vtxfac::pair_classes(Operand::A, vertex, &pairs, band) {
            Err(BooleanError::VertexReadTwice { .. }) if unread => {
                counts.unread += 1;
                continue;
            }
            Err(_) => {
                counts.refused += 1;
                continue;
            }
            Ok(rows) if unread => {
                wrong.push(format!(
                    "{what}, pair probe {k} {far:?}: rows {rows:?} kept beside a partner that \
                     reads nothing"
                ));
                continue;
            }
            Ok(rows) => rows,
        };
        let truth = union.class(far, rng);
        let flip = least_flip(cone, far).min(least_flip(&partner, far));
        let in_band = truth == SideCode::On || flip <= band.zero();
        let kept = rows.len() > 1;
        counts.rows += rows.len();
        counts.kept += usize::from(kept);
        if rows.is_empty() {
            wrong.push(format!("{what}, pair probe {k} {far:?}: no row"));
        }
        for &(_, got) in &rows {
            let defect = match got {
                SideCode::On => (!in_band).then_some("On out of band"),
                _ if got != truth => Some("the oracle contradicts it"),
                _ => in_band.then_some("decided in band"),
            };
            match defect {
                Some(_) if kept => counts.kept_wrong += 1,
                Some(_) if wedge_known_wrong => counts.known_wrong += 1,
                Some(defect) => wrong.push(format!(
                    "{what}, pair probe {k} {far:?}: pair_classes reads {got:?}, exactly \
                     {truth:?}, flipped by a {flip:.3e} m deviation: {defect}"
                )),
                None => {}
            }
        }
    }
    wrong
}

/// What the pair oracle read at one ε: partners placed, rows read,
/// probes refused beside a partner that reads nothing, other refusals,
/// and probes whose layering kept each pair's rows, with their wrong
/// rows.
#[derive(Default)]
struct PairCounts {
    placed: usize,
    rows: usize,
    unread: usize,
    refused: usize,
    kept: usize,
    kept_wrong: usize,
    known_wrong: usize,
}

/// What one reader read at one ε, what it read nothing of or refused,
/// and, for `wedge_classes`, its readings of the long-probe family the
/// CLEAVE item knows wrong.
#[derive(Default)]
struct Counts {
    known_wrong: usize,
    read: usize,
    unread: usize,
    refused: usize,
}

/// **No decided class the polygon-cone reader gives contradicts the
/// exact oracle or lies in band**, over random stars, thin fins and
/// slivers, turned stars, and the graze, bound and long-probe families;
/// `wedge_classes` on the long-probe family is counted, not asserted
/// (module docs).
#[test]
fn the_polygon_cone_reader_never_contradicts_the_exact_oracle() {
    let mut rng = test_utils::fuzz::start("sectors_cone_fuzz");
    let cones = test_utils::fuzz::scaled(90);
    let mut wrong = Vec::new();
    let mut pair_passes = Vec::new();
    let session = Tol::witness().get().eps;
    let mut eps_set = vec![1e-9, 1e-6, 1e-12];
    if !eps_set.contains(&session) {
        eps_set.push(session);
    }
    for eps in eps_set {
        let band = Band::linear_at(Tol::witness(), eps).unwrap();
        let mut counts = [Counts::default(), Counts::default()];
        let mut read_cones = Vec::new();
        let before = wrong.len();
        for c in 0..cones {
            let hollow = rng.below(2) == 0;
            let long = c % 5 == 2;
            let (ring, probes) = if c % 5 == 0 {
                graze(&mut rng)
            } else if c % 5 == 1 {
                beside_bound(&mut rng)
            } else if long {
                dart_long(&mut rng)
            } else {
                let n = 3 + rng.below(9);
                let thin = rng.unit() < 0.3;
                let ring = match rng.below(3) {
                    0 => star(&mut rng, n, 0.1, 3.0, thin, false),
                    1 => star(&mut rng, n, -2.0, 2.0, thin, false),
                    _ => star(&mut rng, n, -1.0, 3.0, thin, true),
                };
                let Some(ring) = ring else { continue };
                (ring, Vec::new())
            };
            let cone = sectors_from_ring(&ring, hollow);
            let probes = if probes.is_empty() {
                probes_about(&mut rng, &cone)
            } else {
                probes
            };
            let what = format!(
                "ε {eps:e}, cone {c} ({} corners, hollow {hollow})",
                ring.len()
            );
            wrong.extend(check_cone(
                &what,
                &cone,
                &probes,
                band,
                &mut rng,
                &mut counts,
                long,
            ));
            read_cones.push((what, cone, long));
        }
        let [cs, wc] = &counts;
        println!(
            "[fuzz] sectors_cone_fuzz at ε {eps:e}: {} wrong; cone_side read {}, {} read \
             nothing, {} refused; wedge_classes read {}, {} read nothing, {} refused, {} of \
             the long-probe family known wrong (CLEAVE's \
             wedge-classes-reads-a-corner-flat-at-its-short-bounds-as-convex)",
            wrong.len() - before,
            cs.read,
            cs.unread,
            cs.refused,
            wc.read,
            wc.unread,
            wc.refused,
            wc.known_wrong
        );
        pair_passes.push((eps, band, read_cones));
    }
    // The pair oracle draws after every cone is read, so a seed reads
    // the same cones with it as without it.
    for (eps, band, read_cones) in pair_passes {
        let mut counts = PairCounts::default();
        let before = wrong.len();
        for (what, cone, long) in &read_cones {
            wrong.extend(check_pair(what, cone, band, &mut rng, &mut counts, *long));
        }
        println!(
            "[fuzz] sectors_cone_fuzz pairs at ε {eps:e}: {} wrong; {} partners placed, {} rows, \
             {} probes refused beside a partner that reads nothing, {} refused otherwise, {} \
             kept per pair ({} of their rows wrong; \
             pair-classes-keeps-per-pair-rows-where-its-layering-is-undecided), {} rows of the \
             long-probe family known wrong",
            wrong.len() - before,
            counts.placed,
            counts.rows,
            counts.unread,
            counts.refused,
            counts.kept,
            counts.kept_wrong,
            counts.known_wrong
        );
    }
    assert!(
        wrong.is_empty(),
        "{} readings contradict the exact oracle or decide in band; {}:\n{}",
        wrong.len(),
        test_utils::fuzz::replay(),
        wrong.join("\n")
    );
}
