//! **A gathered cone against an exact oracle**: random cones nobody
//! chose, each built as the boolean builds the other operand's cone at
//! a vertex ([`Cone::gathered`] over [`crate::boolean::sectors::fans`]),
//! read by its own reading ([`Cone::reads_tied`]) and by the boolean's
//! consumer, which publishes each own edge's class
//! ([`crate::boolean::vtxfac::cone_classes`]), each decided class
//! checked against an exact rational reading of the same cone.
//!
//! **What it certifies.** Every direction reading of the cone, `On`'s
//! sector included (the sector it names holds the direction, exactly or
//! within the band), and every class the consumer publishes for an own
//! edge, on every family; and the same of two partner cones of one
//! solid gathered into one, read against the oracle on their union.
//!
//! The cones are stars about an axis, their corners risen and fallen
//! at random, some with two corners a hair apart (a thin fin or
//! sliver), some turned off the axis; the graze family: a flat corner
//! with one face dented a hair, read by directions a hair off the flat
//! faces; the bound family: a short bound, read by directions a hair
//! past it; and the long-probe family: a dart over 1 mm or 1 cm
//! bounds, its reflex edge dented a hair, read by edges up to 1 km
//! long. Every bound is a line edge, its far point an `f64` vector, and
//! every direction a line edge too.
//!
//! **The consumer's own paths** are a second test's
//! (`the_consumers_paths_never_contradict_the_exact_oracle`): corners
//! subdivided at bisectors as the boolean pushes them (a reflex face
//! among them, read only through its bisectors), bounds and edges that
//! are curved departures, a face the vertex touches with each edge's
//! side given, and partners standing off the vertex's point by a
//! fraction of the band, at the run's `K` and at a narrow `K = 1.5`.
//! Its oracle fixes each ray where the boolean does (a direction out to
//! its reach), spelled apart from the reader's own.
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
//! `On` out of band, or `On` naming a sector whose face holds the
//! direction neither exactly nor within the band. An `On` or `None` in
//! band is the reader's answer there. `None` is counted and printed.
//!
//! It reads at ε = 1e-9, 1e-6 and 1e-12, and at the session's ε where
//! that is another, each cone that is one at that ε: a face whose
//! bounds stand off its own plane beyond the zero band is skipped, and
//! only readings checked are counted. Its stream keeps the name it had
//! as the boolean's cone reader's fuzz, `sectors_cone_fuzz`, so a seed
//! draws the cones it drew for that reader.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the reader it checks, the builder and consumer it reads
// through (the boolean's `fans` and `cone_classes`, and the reach each
// ray is fixed at), and the band it decides in.
test_utils::gated_to![
    "crates/topo/src/link.rs",
    "crates/topo/src/boolean/vtxfac.rs",
    "crates/topo/src/boolean/sectors.rs",
    "crates/topo/src/bound_side.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
];

use super::*;
use crate::boolean::SideCode;
use crate::boolean::sectors::BoolSector;
use crate::boolean::vtxfac::{Partner, cone_classes};
use crate::bound_side::Reach;
use crate::entity::HalfEdgeKey;
use geom_brep::OutwardNormal;
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

impl Wedge {
    /// Whether `q` lies on the wedge's plane within its sector.
    fn holds(&self, q: &V) -> bool {
        det(q, &self.start, &self.end).sign() == 0 && {
            let se = cross(&self.start, &self.end);
            dot(&cross(q, &self.end), &se).sign() >= 0
                && dot(&cross(&self.start, q), &se).sign() >= 0
        }
    }
}

/// The exact reading of one cone.
struct Oracle {
    wedges: Vec<Wedge>,
    reference: V,
}

impl Oracle {
    fn new(cone: &[Sector]) -> Option<Self> {
        Self::with_depth(cone, 40)
    }

    /// [`Oracle::new`] with its reference no deeper inside a face than
    /// `steps` quarterings of the first step allow: a union of two
    /// partners whose faces lie one rounding apart has a sliver between
    /// them that a deeper reference could fall in, reading the union's
    /// parity inverted.
    fn with_depth(cone: &[Sector], steps: usize) -> Option<Self> {
        let wedges: Vec<Wedge> = cone.iter().map(wedge_of).collect();
        // Just inside one face, by an exact step that halves until the
        // segment from the face's middle crosses no other face.
        for (k, w) in wedges.iter().enumerate() {
            let middle = vadd(&w.start, &w.end);
            let inner = vscale(&cross(&w.start, &w.end), &Q::int(-i64::from(w.orient)));
            let mut eta = Q {
                n: BigInt::from(1),
                d: BigInt::from(64),
            };
            for _ in 0..steps {
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
        self.wedges.iter().any(|w| w.holds(q))
    }

    fn class(&self, far: [f64; 3], rng: &mut Rng) -> SideCode {
        self.class_of(&qv(far), rng)
    }

    /// [`Oracle::class`] of an exact point.
    fn class_of(&self, q: &V, rng: &mut Rng) -> SideCode {
        let far = q.clone();
        let q = &far;
        if self.on(q) {
            return SideCode::On;
        }
        let mut reference = self.reference.clone();
        for _ in 0..50 {
            if let Some(parity) = Self::crossings_of(&self.wedges, q, &reference, None) {
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
        panic!("no reference reads {q:?}")
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
/// `hollow`, in orbit order: each sector shares its start with the
/// next one's end.
fn sectors_from_ring(ring: &[([f64; 3], f64)], hollow: bool) -> Vec<Sector> {
    let n = ring.len();
    let unit: Vec<[f64; 3]> = ring.iter().map(|&(d, _)| norm3(d)).collect();
    let far = |k: usize| unit[k].map(|x| x * ring[k].1);
    let mut sectors: Vec<_> = (0..n)
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
        .collect();
    if hollow {
        sectors.reverse();
    }
    sectors
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
                at_vertex: crate::boolean::sectors::BoolFace::Tangent(OutwardNormal::from_chart(
                    v(s.normal),
                    true,
                )),
                arm: s.arm,
                bend: crate::bound_side::FaceBend::Plane,
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
        start_reach: Reach::straight_bisector(1.0),
        end_reach: Reach::Chord {
            base: o,
            far: o + d,
        },
        face: FaceKey::from(slotmap::KeyData::from_ffi((1 << 32) | 99_999)),
        at_vertex: crate::boolean::sectors::BoolFace::Tangent(OutwardNormal::from_chart(
            Vec3::new(1.0, 0.0, 0.0),
            true,
        )),
        arm: 1.0,
        bend: crate::bound_side::FaceBend::Plane,
    }
}

/// What one probe edge read against partners of one solid at the
/// vertex: the gathered cone's own reading of its far point, and the
/// class the consumer published for the edge (`Err`, what it did
/// instead of one row).
struct Read {
    cone: Option<Material>,
    published: Result<Option<SideCode>, String>,
}

/// `far`'s probe edge read against `partners`, each one vertex's
/// sectors about the vertex itself, as the boolean reads a vertex's
/// edges ([`cone_classes`]) and as its cone reads the edge's far point.
fn read_probe(partners: &[Vec<BoolSector<f64>>], far: [f64; 3], band: Band) -> Read {
    let o = Point3::new(0.0, 0.0, 0.0);
    let partners: Vec<_> = partners.iter().map(|s| (s.clone(), o)).collect();
    read_edge(&partners, probe_sector(far), None, band)
}

/// The probe edge `probe` (one sector of the vertex at the origin,
/// whose edge it ends) read against `partners`, each one vertex's
/// sectors about where it stands, and the face the vertex touches
/// where it touches one (`touched`: its outward normal and the side the
/// edge was read on against it), as the boolean reads the edge
/// ([`cone_classes`]) and as its cone reads the edge's point at its
/// reach ([`Reach::point`]).
fn read_edge(
    partners: &[(Vec<BoolSector<f64>>, Point3<f64>)],
    probe: BoolSector<f64>,
    touched: Option<(Vec3<f64>, SideCode)>,
    band: Band,
) -> Read {
    let o = Point3::new(0.0, 0.0, 0.0);
    let corners = partners
        .iter()
        .flat_map(|(s, at)| crate::boolean::sectors::fans(s, *at));
    let gathered = Cone::gathered(o, corners, touched.map(|(n, _)| n)).expect("a cone of faces");
    let given: Vec<_> = touched
        .map(|(_, class)| {
            let side = match class {
                SideCode::In => Sign::Negative,
                SideCode::On => Sign::Zero,
                SideCode::Out => Sign::Positive,
            };
            (gathered.whole_planes()[0], side)
        })
        .into_iter()
        .collect();
    let cone = gathered.reads_tied(probe.end_reach.point(o, probe.end), &given, band);
    let key = |n: u64| VertexKey::from(slotmap::KeyData::from_ffi((1 << 32) | n));
    let partners: Vec<Partner<f64>> = partners
        .iter()
        .zip(7001..)
        .map(|((sectors, at), n)| Partner {
            vertex: key(n),
            at: *at,
            sectors: sectors.clone(),
        })
        .collect();
    let he = probe.he;
    let classes = [(he, touched.map_or(SideCode::On, |(_, class)| class))];
    let touched = touched.map(|(n, _)| (OutwardNormal::from_chart(n, true), &classes[..]));
    let published = match cone_classes(o, &[probe], &[he], &partners, touched, band) {
        Ok(Ok(rows)) => match rows.as_slice() {
            &[(h, class)] if h == he => Ok(Some(class)),
            _ => Err(format!("rows {rows:?}, not the probe edge's one")),
        },
        Ok(Err(_)) => Ok(None),
        Err(e) => Err(format!("{e:?}")),
    };
    Read { cone, published }
}

/// What a decided class is wrong by, against the exact class `truth`,
/// `in_band` where a band-sized move can flip it, `on_band` where an
/// `On` may read (the band, widened by a tie's slack).
fn defect(got: SideCode, truth: SideCode, in_band: bool, on_band: bool) -> Option<&'static str> {
    match got {
        SideCode::On => (!on_band).then_some("On out of band"),
        _ if got != truth => Some("the oracle contradicts it"),
        _ => in_band.then_some("decided in band"),
    }
}

/// `read`'s two readings against the exact class `truth`, each counted
/// in its own [`Counts`], the wrong ones described: the cone's, `On(k)`
/// wrong also where face `k` of `faces` (the cone's faces, in its
/// sectors' order, each its pieces) holds the far point neither exactly
/// nor within the band. `slack` is how much wider than the band an `On`
/// may read: a cone whose corners stand off its point by a tie reads
/// over twice the tie ([`metric::Reading::tied`]).
#[allow(clippy::too_many_arguments)]
fn judge_read(
    what: &str,
    read: Read,
    faces: &[Vec<Sector>],
    far: [f64; 3],
    truth: SideCode,
    flip: f64,
    slack: f64,
    band: Band,
    counts: &mut [Counts; 2],
) -> Vec<String> {
    let in_band = truth == SideCode::On || flip <= band.zero();
    let on_band = in_band || flip <= band.zero() + slack;
    let mut wrong = Vec::new();
    let mut say = |reader: &str, got: &dyn std::fmt::Debug, defect: &str| {
        wrong.push(format!(
            "{what}: {reader} reads {got:?}, exactly {truth:?}, flipped by a {flip:.3e} m \
             deviation: {defect}"
        ));
    };
    counts[0].read += 1;
    match read.cone {
        None => counts[0].unread += 1,
        Some(got) => {
            let class = match got {
                Material::In => SideCode::In,
                Material::Out => SideCode::Out,
                Material::On(_) | Material::Boundary => SideCode::On,
            };
            if let Some(d) = defect(class, truth, in_band, on_band) {
                say("the cone", &got, d);
            }
            if let Material::On(k) = got {
                let held = faces.get(k).is_some_and(|pieces| {
                    pieces.iter().any(|s| {
                        wedge_of(s).holds(&qv(far))
                            || least_flip(std::slice::from_ref(s), far) <= band.zero() + slack
                    })
                });
                if !held {
                    say("the cone", &got, "On a sector that does not hold it");
                }
            }
        }
    }
    counts[1].read += 1;
    match read.published {
        Ok(None) => counts[1].unread += 1,
        Err(why) => {
            counts[1].refused += 1;
            say("the published row", &why, "no one row");
        }
        Ok(Some(class)) => {
            if let Some(d) = defect(class, truth, in_band, on_band) {
                say("the published row", &class, d);
            }
        }
    }
    wrong
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
) -> Vec<String> {
    let Some(oracle) = cone_oracle(cone, band) else {
        return Vec::new();
    };
    let sectors = [bool_sectors(cone)];
    let mut wrong = Vec::new();
    for (k, &far) in probes.iter().enumerate() {
        let read = read_probe(&sectors, far, band);
        let truth = oracle.class(far, rng);
        let flip = least_flip(cone, far);
        wrong.extend(judge_read(
            &format!("{what}, probe {k} {far:?}"),
            read,
            &each(cone),
            far,
            truth,
            flip,
            0.0,
            band,
            counts,
        ));
    }
    wrong
}

/// A cone's faces, each one sector its own piece.
fn each(cone: &[Sector]) -> Vec<Vec<Sector>> {
    cone.iter().map(|s| vec![s.clone()]).collect()
}

/// A face's corner as the oracle reads it.
fn wedge_of(s: &Sector) -> Wedge {
    let (start, end) = (qv(s.start_far), qv(s.end_far));
    let orient = dot(&cross(&start, &end), &qv(s.normal)).sign();
    Wedge { start, end, orient }
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

/// How a second partner sits in the void of the first
/// ([`partner_in_void`]).
#[derive(Clone, Copy, Debug)]
enum Placed {
    /// Its boundary meets the cone's only at the vertex.
    Apart,
    /// One of its corners is a bound of the cone, the rest strictly in
    /// the void: the two touch along that ray.
    AlongARay,
    /// Each of its three corners lies on a face of the cone, inside
    /// that face's sector: every edge of it on the void's boundary.
    OnTheFaces,
}

/// Whether the void of the cone `oracle` reads is convex, exactly: every
/// bound lies on or outside every face's plane, outward being the void.
fn void_is_convex(oracle: &Oracle) -> bool {
    oracle.wedges.iter().all(|w| {
        oracle.wedges.iter().all(|b| {
            [&b.start, &b.end]
                .iter()
                .all(|x| (det(&w.start, &w.end, x).sign() * w.orient) >= 0)
        })
    })
}

/// A ring of three directions about their mean, counterclockwise seen
/// from outside, as [`sectors_from_ring`] takes a met cone's.
fn ring_about(corners: [[f64; 3]; 3], rng: &mut Rng) -> Ring {
    let q = norm3([0, 1, 2].map(|i| corners.iter().map(|c| norm3(*c)[i]).sum::<f64>()));
    let e1 = norm3(cross3(
        q,
        if q[0].abs() < 0.9 {
            [1.0, 0.0, 0.0]
        } else {
            [0.0, 1.0, 0.0]
        },
    ));
    let e2 = cross3(q, e1);
    let angle = |d: [f64; 3]| {
        let dot = |a: [f64; 3], b: [f64; 3]| (0..3).map(|i| a[i] * b[i]).sum::<f64>();
        dot(d, e2).atan2(dot(d, e1))
    };
    let mut sorted = corners;
    sorted.sort_by(|a, b| angle(*a).total_cmp(&angle(*b)));
    sorted.iter().map(|&d| (d, rng.range(0.5, 2.0))).collect()
}

/// A small convex cone in the void of the cone `oracle` reads, a second
/// partner of the same solid, placed as `how` says ([`Placed`]), all
/// read exactly; `None` where the draw fails that. The touching
/// placements are drawn in convex voids only, where a cone over
/// directions in the void's closure stays in it.
fn partner_in_void(
    rng: &mut Rng,
    cone: &[Sector],
    oracle: &Oracle,
    how: Placed,
) -> Option<(Vec<Sector>, Oracle)> {
    let ring: Ring = match how {
        Placed::Apart => {
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
            (0..3)
                .map(|k| {
                    let t = std::f64::consts::TAU * f64::from(k) / 3.0 + rng.range(-0.3, 0.3);
                    let (sn, cs) = t.sin_cos();
                    let d = [0, 1, 2].map(|i| q[i] + r * (cs * e1[i] + sn * e2[i]));
                    (norm3(d), rng.range(0.5, 2.0))
                })
                .collect()
        }
        Placed::AlongARay | Placed::OnTheFaces if !void_is_convex(oracle) => return None,
        Placed::AlongARay => {
            // The void's mean bound lies inside a convex void.
            let g = norm3([0, 1, 2].map(|i| {
                cone.iter()
                    .map(|s| norm3(s.start_far)[i] + norm3(s.end_far)[i])
                    .sum::<f64>()
            }));
            let shared = cone[rng.below(cone.len())].start_far;
            let u = norm3(shared);
            let t = [0.2, 0.5, 0.8][rng.below(3)];
            let mut inner = |k: usize| {
                let w = [0, 1, 2].map(|_| rng.range(-0.15, 0.15));
                let d = [0, 1, 2].map(|i| u[i] + t * (g[i] - u[i]) + t * w[i] * f64::from(k as u8));
                norm3(d)
            };
            let (a, b) = (inner(1), inner(2));
            if [a, b]
                .iter()
                .any(|&d| oracle.class(d, rng) != SideCode::Out)
            {
                return None;
            }
            ring_about([shared, a, b], rng)
        }
        Placed::OnTheFaces => {
            if cone.len() < 3 {
                return None;
            }
            let mut faces: Vec<usize> = (0..cone.len()).collect();
            for k in 0..3 {
                let j = k + rng.below(faces.len() - k);
                faces.swap(k, j);
            }
            let mut mid = |s: &Sector| {
                let w = rng.range(0.2, 0.8);
                let (a, b) = (norm3(s.start_far), norm3(s.end_far));
                [0, 1, 2].map(|i| w * a[i] + (1.0 - w) * b[i])
            };
            let corners = [
                mid(&cone[faces[0]]),
                mid(&cone[faces[1]]),
                mid(&cone[faces[2]]),
            ];
            let centre = [0, 1, 2].map(|i| corners.iter().map(|c| c[i]).sum::<f64>());
            if oracle.class(centre, rng) != SideCode::Out {
                return None;
            }
            ring_about(corners, rng)
        }
    };
    let corners: Vec<V> = ring.iter().map(|&(d, _)| qv(d)).collect();
    if det(&corners[0], &corners[1], &corners[2]).sign() == 0 {
        return None;
    }
    let mut partner = sectors_from_ring(&ring, false);
    for s in &mut partner {
        s.face += 100;
    }
    let own = Oracle::new(&partner)?;
    match how {
        Placed::Apart => {
            let apart = oracle.class(partner[0].start_far, rng) == SideCode::Out
                && own.class(cone[0].start_far, rng) == SideCode::Out
                && !oracle
                    .wedges
                    .iter()
                    .any(|f| own.wedges.iter().any(|g| shares_ray(f, g)));
            apart.then_some((partner, own))
        }
        // The cone's own bounds lie outside a partner in its void.
        Placed::AlongARay | Placed::OnTheFaces => cone
            .iter()
            .all(|s| own.class(s.start_far, rng) != SideCode::In)
            .then_some((partner, own)),
    }
}

/// **Two partners of one solid are read as one cone**: `cone` and a
/// small convex cone in its void ([`partner_in_void`]), two partners of
/// one solid at the vertex, gathered into one cone and read for each
/// probe edge against the exact reading of their union, as
/// [`check_cone`] reads one.
fn check_pair(
    what: &str,
    cone: &[Sector],
    band: Band,
    rng: &mut Rng,
    counts: &mut PairCounts,
) -> Vec<String> {
    let Some(oracle) = cone_oracle(cone, band) else {
        return Vec::new();
    };
    let how = if void_is_convex(&oracle) {
        [Placed::Apart, Placed::AlongARay, Placed::OnTheFaces][rng.below(3)]
    } else {
        Placed::Apart
    };
    let Some((partner, _)) = partner_in_void(rng, cone, &oracle, how) else {
        return Vec::new();
    };
    let mut probes = probes_about(rng, cone);
    probes.extend(probes_about(rng, &partner));
    check_partners(
        &format!("{what}, {how:?}"),
        cone,
        &partner,
        &probes,
        how,
        band,
        rng,
        counts,
    )
}

/// [`check_pair`]'s reading of `probes` against `cone` and `partner`
/// gathered into one cone, against the exact reading of their union.
#[allow(clippy::too_many_arguments)]
fn check_partners(
    what: &str,
    cone: &[Sector],
    partner: &[Sector],
    probes: &[[f64; 3]],
    how: Placed,
    band: Band,
    rng: &mut Rng,
    counts: &mut PairCounts,
) -> Vec<String> {
    let both: Vec<Sector> = cone.iter().chain(partner).cloned().collect();
    // Twelve tries, the step quartered from 1/64, leave the reference
    // no deeper than 1/(64·4¹¹) ≈ 3.7e-9 inside a face: far above a
    // rounding's sliver, so a face one rounding from the partner's is
    // passed over for another.
    let Some(union) = Oracle::with_depth(&both, 12) else {
        counts.no_reference += 1;
        return Vec::new();
    };
    counts.placed[how as usize] += 1;
    let partners = [bool_sectors(cone), bool_sectors(partner)];
    let mut wrong = Vec::new();
    for (k, &far) in probes.iter().enumerate() {
        let read = read_probe(&partners, far, band);
        let truth = union.class(far, rng);
        let flip = least_flip(cone, far).min(least_flip(partner, far));
        wrong.extend(judge_read(
            &format!("{what}, pair probe {k} {far:?}"),
            read,
            &each(&both),
            far,
            truth,
            flip,
            0.0,
            band,
            &mut counts.reads,
        ));
    }
    wrong
}

/// What the pair oracle read at one ε: partners placed, and each
/// reader's counts.
#[derive(Default)]
struct PairCounts {
    /// By [`Placed`]: apart, along a ray, on the faces.
    placed: [usize; 3],
    /// Placed pairs whose union no reference reads clear of a sliver,
    /// skipped.
    no_reference: usize,
    reads: [Counts; 2],
}

/// What one reader read at one ε, and what it read nothing of or
/// refused.
#[derive(Default)]
struct Counts {
    read: usize,
    unread: usize,
    refused: usize,
}

impl std::fmt::Display for Counts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "read {}, {} read nothing, {} refused",
            self.read, self.unread, self.refused
        )
    }
}

/// **No decided class a gathered cone gives, or the boolean publishes
/// through it, contradicts the exact oracle or lies in band**, over
/// random stars, thin fins and slivers, turned stars, and the graze,
/// bound and long-probe families, and over two partners of each read as
/// one cone.
#[test]
fn a_gathered_cone_never_contradicts_the_exact_oracle() {
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
            let (ring, probes) = if c % 5 == 0 {
                graze(&mut rng)
            } else if c % 5 == 1 {
                beside_bound(&mut rng)
            } else if c % 5 == 2 {
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
            ));
            read_cones.push((what, cone));
        }
        let [cone, published] = &counts;
        println!(
            "[fuzz] link_cone_fuzz at ε {eps:e}: {} wrong; the cone {cone}; the published \
             rows {published}",
            wrong.len() - before,
        );
        pair_passes.push((eps, band, read_cones));
    }
    // The pair oracle draws after every cone is read, so a seed reads
    // the same cones with it as without it.
    for (eps, band, read_cones) in pair_passes {
        let mut counts = PairCounts::default();
        let before = wrong.len();
        for (what, cone) in &read_cones {
            wrong.extend(check_pair(what, cone, band, &mut rng, &mut counts));
        }
        let [cone, published] = &counts.reads;
        println!(
            "[fuzz] link_cone_fuzz pairs at ε {eps:e}: {} wrong; partners placed {} apart, {} \
             along a ray, {} on the faces, {} with no clear reference; the cone {cone}; the \
             published rows {published}",
            wrong.len() - before,
            counts.placed[0],
            counts.placed[1],
            counts.placed[2],
            counts.no_reference,
        );
        // A union is skipped only where no face of the cone gives a
        // reference clear of the partner within the depth; no draw has
        // met one, so one that does is reported, not passed over.
        if counts.no_reference > 0 {
            wrong.push(format!(
                "ε {eps:e}: {} placed pairs had no clear reference, and were not read",
                counts.no_reference
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "{} readings contradict the exact oracle or decide in band; {}:\n{}",
        wrong.len(),
        test_utils::fuzz::replay(),
        wrong.join("\n")
    );
}

/// A cone over `ring` (directions, each at unit reach) whose exact
/// reading puts `inside` in its material (`material`) or out of it:
/// whichever winding and side of [`sectors_from_ring`] does.
fn cone_holding(ring: &[[f64; 3]], inside: [f64; 3], material: bool) -> Vec<Sector> {
    let want = if material {
        SideCode::In
    } else {
        SideCode::Out
    };
    let mut rng = Rng::from_seed(1);
    for reversed in [false, true] {
        let mut r: Ring = ring.iter().map(|&d| (d, 1.0)).collect();
        if reversed {
            r.reverse();
        }
        for hollow in [false, true] {
            let mut cone = sectors_from_ring(&r, hollow);
            if Oracle::new(&cone).is_some_and(|o| o.class(inside, &mut rng) == want) {
                if material {
                    for s in &mut cone {
                        s.face += 100;
                    }
                }
                return cone;
            }
        }
    }
    panic!("no winding of {ring:?} holds {inside:?}")
}

/// **An island whose every edge lies on its void's faces reads as one
/// cone with it**: read by [`check_partners`] against the exact oracle,
/// every probe about each reads, none wrong. The octant's island is
/// convex; the pentagon in a chevron void shares the chevron's reflex
/// edge; the thin V, its arms 1e-9 thick, is read at ε 1e-12. The
/// probes come from a fixed seed: they are the fixture's, drawn about
/// each cone rather than written out.
#[test]
fn islands_touching_every_face_of_their_voids_read_as_one_cone() {
    let band = Band::linear(Tol::witness()).unwrap();
    let k = 1e-9;
    let scenes = [
        (
            "an octant's island",
            band,
            vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            [1.0, 1.0, 1.0],
            vec![[1.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 0.0, 1.0]],
            [1.0, 1.0, 1.0],
        ),
        (
            "a pentagon in a chevron",
            band,
            vec![
                [-3.0, 2.0, 1.0],
                [0.0, -1.0, 1.0],
                [3.0, 2.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
            [0.0, -0.5, 1.0],
            vec![
                [0.0, 0.0, 1.0],
                [-1.5, 1.0, 1.0],
                [-0.5, -0.5, 1.0],
                [0.5, -0.5, 1.0],
                [1.5, 1.0, 1.0],
            ],
            [0.0, -0.3, 1.0],
        ),
        (
            "a V 1e-9 thick in a V, at ε 1e-12",
            Band::linear_at(Tol::witness(), 1e-12).unwrap(),
            vec![
                [-2.0, 2.0, 1.0],
                [0.0, 0.0, 1.0],
                [2.0, 2.0, 1.0],
                [2.0 - k, 2.0, 1.0],
                [0.0, k, 1.0],
                [-2.0 + k, 2.0, 1.0],
            ],
            [1.0, 1.0 + k / 2.0, 1.0],
            vec![
                [0.0, 0.0, 1.0],
                [2.0, 2.0, 1.0],
                [2.0 - k / 2.0, 2.0, 1.0],
                [1.8, 1.8 + k, 1.0],
                [0.0, k, 1.0],
                [-1.8, 1.8 + k, 1.0],
                [-2.0 + k / 2.0, 2.0, 1.0],
                [-2.0, 2.0, 1.0],
            ],
            [1.0, 1.0 + k / 2.0, 1.0],
        ),
    ];
    let mut rng = Rng::from_seed(0x1514);
    for (what, band, void, in_void, island, in_island) in scenes {
        let cone = cone_holding(&void, in_void, false);
        let partner = cone_holding(&island, in_island, true);
        let mut probes = probes_about(&mut rng, &cone);
        probes.extend(probes_about(&mut rng, &partner));
        let mut counts = PairCounts::default();
        let wrong = check_partners(
            what,
            &cone,
            &partner,
            &probes,
            Placed::OnTheFaces,
            band,
            &mut rng,
            &mut counts,
        );
        assert!(wrong.is_empty(), "{what}:\n{}", wrong.join("\n"));
        assert_eq!(
            counts.no_reference, 0,
            "{what}: the union has a clear reference"
        );
        let [cone, published] = &counts.reads;
        assert!(
            published.read - published.unread >= probes.len() / 2,
            "{what}: {published} of {} probes; the cone {cone}",
            probes.len(),
        );
    }
}

// ---- The consumer's own paths: subdivided corners, curved departures,
// a touched face, and partners off the vertex's point.

/// One face's corner as the boolean draws it: its pieces, in turn
/// counterclockwise about the face's outward normal, each two
/// consecutive ones sharing a ray (a subdivision bisector, with no
/// point behind it), the first piece's start and the last's end the
/// corner's bounds (each a line edge or a curved edge's departure).
struct Corner {
    pieces: Vec<Sector>,
}

/// How a corner's bounds are drawn: line edges to their far points, or
/// curved edges leaving along the same directions, each to its extent.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Bounds {
    Chord,
    Conic,
}

/// A curved edge's bend, which the cone does not read: its departure
/// tangent at its extent is its ray ([`Reach::point`]).
fn flat_bend() -> geom_brep::ConicBend<f64> {
    geom_brep::ConicBend {
        curvature: Vec3::new(0.0, 0.0, 0.0),
        bounds: geom_brep::BendBounds::Unbounded,
    }
}

/// The reach a far point `f` (off the origin) is drawn with, as a
/// corner's bound (`bound`) or a bisector between two of its pieces.
fn reach_of(f: [f64; 3], bound: Option<Bounds>) -> (Vec3<f64>, Reach<f64>) {
    let o = Point3::new(0.0, 0.0, 0.0);
    let v = Vec3::new(f[0], f[1], f[2]);
    let reach = match bound {
        Some(Bounds::Chord) => Reach::Chord {
            base: o,
            far: o + v,
        },
        Some(Bounds::Conic) => Reach::Conic {
            extent: v.norm(),
            bend: flat_bend(),
        },
        None => Reach::straight_bisector(v.norm()),
    };
    (v.normalize(), reach)
}

/// The point a reach fixes, as the oracle reads it: a line edge's far
/// point, and any other bound's direction out to its length, spelled
/// here apart from the reader's [`Reach::point`] (the same `f64`
/// operations, so the same numbers where that reading is right).
fn fixed(dir: Vec3<f64>, reach: Reach<f64>) -> [f64; 3] {
    let x = match reach {
        Reach::Chord { far, .. } => far,
        Reach::Conic { extent: l, .. } | Reach::Bisector(l, _) | Reach::Extent(l) => {
            Point3::new(0.0, 0.0, 0.0) + dir.normalize() * l
        }
    };
    [x.x, x.y, x.z]
}

/// **`corners` drawn as the boolean draws a vertex's sectors**
/// (`sectors::build_sectors`): each corner's pieces under one
/// half-edge, the end-sharing piece first, so the fan runs from the
/// last piece's start to the first piece's end ([`crate::boolean::sectors::fans`]).
/// Returns the sectors and the oracle's faces, each its pieces with
/// every ray at the point the reader fixes it at.
fn drawn(corners: &[Corner], bounds: Bounds) -> (Vec<BoolSector<f64>>, Vec<Vec<Sector>>) {
    let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
    let mut sectors = Vec::new();
    let mut faces = Vec::new();
    for (c, corner) in corners.iter().enumerate() {
        let last = corner.pieces.len() - 1;
        let mut face = Vec::new();
        for (k, s) in corner.pieces.iter().enumerate() {
            let (start, start_reach) = reach_of(s.start_far, (k == 0).then_some(bounds));
            let (end, end_reach) = reach_of(s.end_far, (k == last).then_some(bounds));
            let normal = Vec3::new(s.normal[0], s.normal[1], s.normal[2]);
            sectors.push(BoolSector {
                he: HalfEdgeKey::from(key(c as u64 + 1)),
                start,
                end,
                start_reach,
                end_reach,
                face: FaceKey::from(key(1000 + s.face)),
                at_vertex: crate::boolean::sectors::BoolFace::Tangent(OutwardNormal::from_chart(
                    normal, true,
                )),
                arm: s.arm,
                bend: crate::bound_side::FaceBend::Plane,
            });
            face.push(Sector {
                start_far: fixed(start, start_reach),
                end_far: fixed(end, end_reach),
                ..s.clone()
            });
        }
        // Pushed as `build_sectors` pushes them: the end-sharing piece
        // first.
        let n = sectors.len();
        sectors[n - corner.pieces.len()..].reverse();
        faces.push(face);
    }
    (sectors, faces)
}

/// A cone's sectors as corners, each split into up to three pieces at
/// directions inside it, at random reaches.
fn split(rng: &mut Rng, cone: &[Sector]) -> Vec<Corner> {
    cone.iter()
        .map(|s| {
            let cuts = rng.below(3);
            let mut ws: Vec<f64> = (0..cuts).map(|_| rng.range(0.15, 0.85)).collect();
            ws.sort_by(f64::total_cmp);
            let (a, b) = (norm3(s.start_far), norm3(s.end_far));
            let mut rays = vec![s.start_far];
            for w in ws {
                let l = rng.range(0.3, 2.0);
                rays.push(norm3([0, 1, 2].map(|i| (1.0 - w) * a[i] + w * b[i])).map(|x| x * l));
            }
            rays.push(s.end_far);
            Corner {
                pieces: rays
                    .windows(2)
                    .map(|r| Sector {
                        start_far: r[0],
                        end_far: r[1],
                        ..s.clone()
                    })
                    .collect(),
            }
        })
        .collect()
}

/// **A solid with a notch at a reflex corner**: its top face, on
/// `z = 0` with the material below, turns `θ` (over half a turn, under
/// a whole one) about the vertex, cut into three pieces by bisectors,
/// and two walls run down from its bounds to a ray `d` below, the
/// notch's hollow between them. One corner of the top's fan is wider
/// than half a turn, so only its bisectors read it.
fn notch(rng: &mut Rng) -> Vec<Corner> {
    let tau = std::f64::consts::TAU;
    let a0 = rng.range(0.0, tau);
    let theta = rng.range(0.56, 0.9) * tau;
    let dir = |a: f64| [a.cos(), a.sin(), 0.0];
    let u0 = dir(a0);
    let u1 = dir(a0 + theta);
    // The notch's own bisector in the top's plane, the short way round.
    let mid = dir(a0 + theta + (tau - theta) / 2.0);
    let r = rng.range(0.2, 2.0);
    let d = norm3([mid[0] * r, mid[1] * r, -1.0]);
    // One length per ray: a length drawn per coordinate turns the ray
    // off its direction, and the walls oriented by the directions then
    // bound no one material.
    let len = |rng: &mut Rng, u: [f64; 3]| {
        let l = rng.range(0.5, 2.0);
        u.map(|x| x * l)
    };
    let (f0, f1, fd) = (len(rng, u0), len(rng, u1), len(rng, d));
    let rays: Vec<[f64; 3]> = (0..4)
        .map(|k| {
            if k == 0 {
                f0
            } else if k == 3 {
                f1
            } else {
                len(rng, dir(a0 + theta * f64::from(k as u8) / 3.0))
            }
        })
        .collect();
    let top = Corner {
        pieces: rays
            .windows(2)
            .map(|w| Sector {
                start_far: w[0],
                end_far: w[1],
                normal: [0.0, 0.0, 1.0],
                arm: 0.5,
                face: 1,
            })
            .collect(),
    };
    // A wall through `x` and `fd`, its outward normal into the notch
    // (toward `toward`), its bounds counterclockwise about it.
    let wall = |x: [f64; 3], toward: [f64; 3], face: u64| {
        let mut n = norm3(cross3(x, fd));
        let into = n[0] * toward[0] + n[1] * toward[1] + n[2] * toward[2];
        if into < 0.0 {
            n = n.map(|c| -c);
        }
        let ccw = {
            let c = cross3(x, fd);
            c[0] * n[0] + c[1] * n[1] + c[2] * n[2] > 0.0
        };
        let (start_far, end_far) = if ccw { (x, fd) } else { (fd, x) };
        Corner {
            pieces: vec![Sector {
                start_far,
                end_far,
                normal: n,
                arm: 0.5,
                face,
            }],
        }
    };
    vec![top, wall(f0, u1, 2), wall(f1, u0, 3)]
}

/// **Whether `faces` bound one material**, as the exact oracle reads
/// it: just behind each piece's middle (against its outward normal) the
/// material, just in front of it not. A cone whose outward normals do
/// not agree with one material is no solid's corner, and the reader
/// and the oracle would read two materials.
fn one_material(faces: &[Vec<Sector>], oracle: &Oracle, rng: &mut Rng) -> bool {
    faces.iter().flatten().all(|s| {
        let (a, b) = (norm3(s.start_far), norm3(s.end_far));
        let at = |h: f64| [0, 1, 2].map(|i| (a[i] + b[i]) / 2.0 + h * s.normal[i]);
        oracle.class(at(-1e-4), rng) == SideCode::In && oracle.class(at(1e-4), rng) == SideCode::Out
    })
}

/// **Every notch drawn is one solid's corner**: its top face reflex,
/// its walls' outward normals into the notch, so the exact oracle reads
/// one material behind every face. The draw at `0x0a62fa00d8d51732`'s
/// fourteenth notch scaled its rays per coordinate: the top turned
/// 174.6° rather than the 203.3° drawn, its walls were oriented by the
/// directions as drawn, and the reader and the oracle read two
/// materials (CI, PR 6247).
#[test]
fn every_notch_drawn_is_one_solid() {
    let mut rng = Rng::from_seed(0x0a62_fa00_d8d5_1732);
    let band = Band::linear_at(Tol::witness(), 1e-9).unwrap();
    for k in 0..60 {
        let (_, faces) = drawn(&notch(&mut rng), Bounds::Chord);
        let pieces: Vec<Sector> = faces.iter().flatten().cloned().collect();
        let oracle = cone_oracle(&pieces, band).expect("a notch is a cone at 1e-9");
        let turn: f64 = faces[0]
            .iter()
            .map(|s| {
                let (a, b) = (norm3(s.start_far), norm3(s.end_far));
                cross3(a, b)[2].atan2(a[0] * b[0] + a[1] * b[1])
            })
            .sum();
        assert!(
            turn > std::f64::consts::PI,
            "notch {k}: its top turns {turn} rad"
        );
        assert!(
            one_material(&faces, &oracle, &mut rng),
            "notch {k}: one material"
        );
    }
}

/// The probe edge to `far` drawn as `bounds` draws a corner's bound,
/// and the point the reader fixes it at.
fn drawn_probe(far: [f64; 3], bounds: Bounds) -> (BoolSector<f64>, [f64; 3]) {
    let mut probe = probe_sector(far);
    let (end, end_reach) = reach_of(far, Some(bounds));
    probe.end = end;
    probe.end_reach = end_reach;
    (probe, fixed(end, end_reach))
}

/// What one consumer-path family read.
#[derive(Default)]
struct Family {
    reads: [Counts; 2],
    probes: usize,
}

impl Family {
    /// The probes either reader decided a class of (`In`, `Out` or
    /// `On`), at most `probes`.
    fn decided(&self) -> usize {
        let [cone, published] = &self.reads;
        (cone.read - cone.unread).min(published.read - published.unread - published.refused)
    }
}

/// `corners` (one partner at the vertex) read for each probe, drawn as
/// `bounds`, against the exact reading of the faces the reader fixes.
fn check_corners(
    what: &str,
    corners: &[Corner],
    probes: &[[f64; 3]],
    bounds: Bounds,
    band: Band,
    rng: &mut Rng,
    family: &mut Family,
) -> Vec<String> {
    let o = Point3::new(0.0, 0.0, 0.0);
    let (sectors, faces) = drawn(corners, bounds);
    let pieces: Vec<Sector> = faces.iter().flatten().cloned().collect();
    let Some(oracle) = cone_oracle(&pieces, band) else {
        return Vec::new();
    };
    if !one_material(&faces, &oracle, rng) {
        return vec![format!(
            "{what}: the drawn cone bounds no one material (a fixture defect)"
        )];
    }
    let mut wrong = Vec::new();
    for (k, &far) in probes.iter().enumerate() {
        let (probe, at) = drawn_probe(far, bounds);
        let read = read_edge(&[(sectors.clone(), o)], probe, None, band);
        family.probes += 1;
        wrong.extend(judge_read(
            &format!("{what}, probe {k} {at:?}"),
            read,
            &faces,
            at,
            oracle.class(at, rng),
            least_flip(&pieces, at),
            0.0,
            band,
            &mut family.reads,
        ));
    }
    wrong
}

/// **A vertex touching a face beside a partner**: the face's plane
/// through the vertex (outward `n`, the material behind it) and a
/// partner of the same solid, a small convex cone in front of it
/// touching the plane only at the vertex. Each probe edge is given its
/// side of the face as the pierce reads it ([`cone_classes`]'s
/// `touched`): the side of its far point, or, for an edge that leaves
/// a hair off the plane and bends back across it (a curved edge's tie
/// read one order down), the other side, the exact class then the one
/// of the far point mirrored across the plane.
fn check_touched(what: &str, band: Band, rng: &mut Rng, family: &mut Family) -> Vec<String> {
    let o = Point3::new(0.0, 0.0, 0.0);
    let n = norm3([
        rng.range(-1.0, 1.0),
        rng.range(-1.0, 1.0),
        rng.range(-1.0, 1.0),
    ]);
    let e1 = norm3(cross3(
        n,
        if n[0].abs() < 0.9 {
            [1.0, 0.0, 0.0]
        } else {
            [0.0, 1.0, 0.0]
        },
    ));
    let e2 = cross3(n, e1);
    let neg = |v: [f64; 3]| v.map(|x| -x);
    let disc: Vec<Sector> = [(e1, e2), (e2, neg(e1)), (neg(e1), neg(e2)), (neg(e2), e1)]
        .iter()
        .map(|&(start_far, end_far)| Sector {
            start_far,
            end_far,
            normal: n,
            arm: 1.0,
            face: 900,
        })
        .collect();
    // The partner about an axis in front of the plane.
    let t = rng.range(0.0, std::f64::consts::TAU);
    let tilt = rng.range(0.0, 0.8);
    let q = norm3([0, 1, 2].map(|i| n[i] + tilt * (t.cos() * e1[i] + t.sin() * e2[i])));
    let r = rng.range(0.1, 0.4);
    let (f1, f2) = {
        let f1 = norm3(cross3(q, e1));
        (f1, cross3(q, f1))
    };
    let ring: Vec<[f64; 3]> = (0..3)
        .map(|k| {
            let a = std::f64::consts::TAU * f64::from(k as u8) / 3.0 + rng.range(-0.3, 0.3);
            norm3([0, 1, 2].map(|i| q[i] + r * (a.cos() * f1[i] + a.sin() * f2[i])))
        })
        .collect();
    let dot3 = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    if ring.iter().any(|&d| dot3(d, n) < 0.05) {
        return Vec::new();
    }
    let partner = cone_holding(&ring, q, true);
    let both: Vec<Sector> = partner.iter().chain(&disc).cloned().collect();
    let (Some(union), Some(_)) = (cone_oracle(&both, band), cone_oracle(&partner, band)) else {
        return Vec::new();
    };
    let sectors = bool_sectors(&partner);
    let mut faces = each(&partner);
    faces.push(disc.clone());
    let side = |h: f64| {
        if h.abs() <= band.zero() {
            SideCode::On
        } else if h > 0.0 {
            SideCode::Out
        } else {
            SideCode::In
        }
    };
    let mut wrong = Vec::new();
    let mut read =
        |label: String, far: [f64; 3], given: SideCode, truth, flip, family: &mut Family| {
            let read = read_edge(
                &[(sectors.clone(), o)],
                probe_sector(far),
                Some((Vec3::new(n[0], n[1], n[2]), given)),
                band,
            );
            family.probes += 1;
            wrong.extend(judge_read(
                &label,
                read,
                &faces,
                far,
                truth,
                flip,
                0.0,
                band,
                &mut family.reads,
            ));
        };
    // Its far point's own side.
    let mut free = probes_about(rng, &partner);
    free.extend(probes_about(rng, &disc));
    for (k, far) in free.into_iter().enumerate() {
        let truth = union.class(far, rng);
        let flip = least_flip(&both, far);
        read(
            format!("{what}, touched probe {k} {far:?}"),
            far,
            side(dot3(far, n)),
            truth,
            flip,
            family,
        );
    }
    // Off the plane one way, given the other.
    for k in 0..12 {
        let a = rng.range(0.0, std::f64::consts::TAU);
        let l = rng.range(0.5, 2.0);
        let h = [20.0 * band.zero(), 1e-4][rng.below(2)] * [1.0, -1.0][rng.below(2)];
        let base = [0, 1, 2].map(|i| l * (a.cos() * e1[i] + a.sin() * e2[i]));
        let far = [0, 1, 2].map(|i| base[i] + h * n[i]);
        let mirror = [0, 1, 2].map(|i| base[i] - h * n[i]);
        let flip = least_flip(&partner, far).min(least_flip(&partner, mirror));
        if flip <= 4.0 * h.abs() + band.zero() {
            continue;
        }
        let truth = union.class(mirror, rng);
        read(
            format!("{what}, bent probe {k} {far:?}, given {:?}", side(-h)),
            far,
            side(-h),
            truth,
            flip,
            family,
        );
    }
    wrong
}

/// **A partner standing off the vertex's point by a fraction of the
/// band**: `cone`, its corner at `at`, read for each probe edge from the
/// origin against the exact reading of the cone at `at`, and by probes
/// at heights of a few bands over its faces' planes there. An `On` may
/// read twice the tie wider than the band ([`metric::Reading::tied`]).
fn check_offset(
    what: &str,
    cone: &[Sector],
    at: [f64; 3],
    band: Band,
    rng: &mut Rng,
    family: &mut Family,
) -> Vec<String> {
    let Some(oracle) = cone_oracle(cone, band) else {
        return Vec::new();
    };
    let apex = Point3::new(at[0], at[1], at[2]);
    let sectors: Vec<BoolSector<f64>> = bool_sectors(cone)
        .into_iter()
        .map(|mut s| {
            for r in [&mut s.start_reach, &mut s.end_reach] {
                if let Reach::Chord { far, .. } = *r {
                    *r = Reach::Chord {
                        base: apex,
                        far: apex + (far - Point3::new(0.0, 0.0, 0.0)),
                    };
                }
            }
            s
        })
        .collect();
    let tie = (at[0] * at[0] + at[1] * at[1] + at[2] * at[2]).sqrt();
    let mut probes: Vec<[f64; 3]> = probes_about(rng, cone)
        .into_iter()
        .map(|f| [0, 1, 2].map(|i| f[i] + at[i]))
        .collect();
    for _ in 0..8 {
        let s = &cone[rng.below(cone.len())];
        let (a, b) = (norm3(s.start_far), norm3(s.end_far));
        let w = rng.range(0.1, 0.9);
        let inside = norm3([0, 1, 2].map(|i| (1.0 - w) * a[i] + w * b[i]));
        let h =
            [0.5, 0.9, 1.2, 1.6, 3.5, 12.0][rng.below(6)] * band.zero() * [1.0, -1.0][rng.below(2)];
        let l = rng.range(0.5, 2.0);
        probes.push([0, 1, 2].map(|i| at[i] + inside[i] * l + s.normal[i] * h));
    }
    let mut wrong = Vec::new();
    for (k, far) in probes.into_iter().enumerate() {
        // The probe from the vertex, read about the corner's own apex.
        let rel = [0, 1, 2].map(|i| qv(far)[i].sub(&Q::of(at[i])));
        let truth = oracle.class_of(&rel, rng);
        let flip = least_flip(cone, [0, 1, 2].map(|i| far[i] - at[i]));
        let read = read_edge(&[(sectors.clone(), apex)], probe_sector(far), None, band);
        family.probes += 1;
        wrong.extend(judge_read(
            &format!("{what}, offset probe {k} {far:?}"),
            read,
            &each(cone),
            far,
            truth,
            flip,
            2.0 * tie,
            band,
            &mut family.reads,
        ));
    }
    wrong
}

/// **The consumer's own paths never contradict the exact oracle**:
/// subdivided corners (stars split at bisectors, and a notch's reflex
/// top face, whose fan only its bisectors read), curved departures
/// (every bound and probe a conic leaving along the same direction, its
/// ray fixed at its extent), a face the vertex touches beside a partner
/// (each edge given its side of it, some the other side than its far
/// point's), and a partner standing off the vertex's point by a
/// fraction of the band, at the run's `K` and at `K = 1.5`, where the
/// band's gap is narrower than twice the tie. Each family decides at
/// least a third of its probes, so a reader that reads nothing is red
/// too.
#[test]
fn the_consumers_paths_never_contradict_the_exact_oracle() {
    let mut rng = test_utils::fuzz::start("link_cone_consumer_fuzz");
    let cones = test_utils::fuzz::scaled(40);
    let mut wrong = Vec::new();
    let session = Tol::witness().get().eps;
    let mut eps_set = vec![1e-9, 1e-6, 1e-12];
    if !eps_set.contains(&session) {
        eps_set.push(session);
    }
    let names = [
        "split stars",
        "notches",
        "conic stars",
        "touched faces",
        "offset partners",
    ];
    for eps in eps_set {
        let band = Band::linear_at(Tol::witness(), eps).unwrap();
        let narrow = Band::new(eps, 1.5 * eps).unwrap();
        let mut families: [Family; 5] = Default::default();
        for c in 0..cones {
            let what = format!("ε {eps:e}, cone {c}");
            let hollow = rng.below(2) == 0;
            let n = 3 + rng.below(7);
            let thin = rng.unit() < 0.3;
            let Some(ring) = star(&mut rng, n, -1.0, 3.0, thin, true) else {
                continue;
            };
            let cone = sectors_from_ring(&ring, hollow);
            let probes = probes_about(&mut rng, &cone);
            let corners = split(&mut rng, &cone);
            wrong.extend(check_corners(
                &format!("{what}, split"),
                &corners,
                &probes,
                Bounds::Chord,
                band,
                &mut rng,
                &mut families[0],
            ));
            let corners = notch(&mut rng);
            let pieces: Vec<Sector> = corners.iter().flat_map(|c| c.pieces.clone()).collect();
            let probes = probes_about(&mut rng, &pieces);
            wrong.extend(check_corners(
                &format!("{what}, notch"),
                &corners,
                &probes,
                Bounds::Chord,
                band,
                &mut rng,
                &mut families[1],
            ));
            let corners: Vec<Corner> = cone
                .iter()
                .map(|s| Corner {
                    pieces: vec![s.clone()],
                })
                .collect();
            let probes = probes_about(&mut rng, &cone);
            wrong.extend(check_corners(
                &format!("{what}, conic"),
                &corners,
                &probes,
                Bounds::Conic,
                band,
                &mut rng,
                &mut families[2],
            ));
            wrong.extend(check_touched(
                &format!("{what}, touched"),
                band,
                &mut rng,
                &mut families[3],
            ));
            let tie = [0.3, 0.6, 0.9][rng.below(3)] * eps;
            let u = norm3([
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            ]);
            let b = if c % 2 == 0 { band } else { narrow };
            wrong.extend(check_offset(
                &format!("{what}, offset {tie:e} (escalate {:e})", b.escalate()),
                &cone,
                u.map(|x| x * tie),
                b,
                &mut rng,
                &mut families[4],
            ));
        }
        for (name, family) in names.iter().zip(&families) {
            let [cone, published] = &family.reads;
            println!(
                "[fuzz] link_cone_consumer_fuzz {name} at ε {eps:e}: the cone {cone}; the \
                 published rows {published}"
            );
            if family.decided() * 3 < family.probes {
                wrong.push(format!(
                    "ε {eps:e}, {name}: decided {} of {} probes, under a third",
                    family.decided(),
                    family.probes
                ));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "{} readings contradict the exact oracle, decide in band, or read too little; {}:\n{}",
        wrong.len(),
        test_utils::fuzz::replay(),
        wrong.join("\n")
    );
}
