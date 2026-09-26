//! Review lane `pick3-r2` (EDIT-PICK3, PR #2786, frozen head
//! `31cbee19f`): probes that falsify the `t`-interval door's claims by
//! execution. Each row asserts what the door's own docs claim, so a
//! red row is a claim the head does not keep.
//!
//! Two of this lane's rows have moved, because they pin the door and
//! belong beside it: the early-out fixture (a narrower member of the
//! certified tie, pruned, with the answer then depending on which
//! target was offered first) is now
//! `pick3_early_out::the_certified_tie_is_decided_by_the_candidates_and_not_the_targets_order`,
//! and the clamp's exactness is
//! `pick::tests::the_retraction_keeps_u_plus_v_at_most_one_exactly`,
//! calling the real `retract_to_simplex` rather than a restatement of
//! it.
//!
//! 1. `the_t_interval_encloses_the_exact_crossing_and_the_clamped_point`
//!    — exact rational arithmetic (`num-bigint`) over stressed families
//!    (near-parallel rays, far origins, `|d| ≫ 1` and `≪ 1`, tiny
//!    triangles, the `near_tangent` fixture at every `k`): does
//!    `[t_lo, t_hi]` enclose the exact crossing's parameter whenever
//!    that crossing is on the closed triangle, and the clamped point's
//!    own parameter always?

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

test_utils::gated_to![
    "crates/editor-core/src/resolve/",
    "crates/bvh/src/",
    "crates/editor-core/tests/fixture/pick.rs",
];

use crate::fixture::pick::{aimed, near_tangent};
use bvh::Ray;
use editor_core::resolve::{crossing, ray_triangle};
use geom_core::{Point3, Vec3};
use num_bigint::BigInt;

// ---------------------------------------------------------------
// 2. Exact enclosure.
// ---------------------------------------------------------------

/// Every `f64` here is read as an exact dyadic rational over `2^SHIFT`.
const SHIFT: i64 = 400;

fn big(x: f64) -> BigInt {
    assert!(x.is_finite(), "the probe reads finite values only, got {x}");
    if x == 0.0 {
        return BigInt::from(0);
    }
    let bits = x.to_bits();
    let neg = bits >> 63 == 1;
    let exp = ((bits >> 52) & 0x7ff) as i64;
    let frac = bits & ((1u64 << 52) - 1);
    let (mant, e) = if exp == 0 {
        (frac, -1074i64)
    } else {
        (frac | (1u64 << 52), exp - 1075)
    };
    let sh = e + SHIFT;
    assert!(sh >= 0, "{x:e} is below the probe's dyadic grid");
    let m = BigInt::from(mant) << (sh as usize);
    if neg { -m } else { m }
}

type V = [BigInt; 3];

fn vb(p: Point3<f64>) -> V {
    [big(p.x), big(p.y), big(p.z)]
}
fn vv(v: Vec3<f64>) -> V {
    [big(v.x), big(v.y), big(v.z)]
}
fn sub(a: &V, b: &V) -> V {
    [&a[0] - &b[0], &a[1] - &b[1], &a[2] - &b[2]]
}
fn add(a: &V, b: &V) -> V {
    [&a[0] + &b[0], &a[1] + &b[1], &a[2] + &b[2]]
}
fn cross(a: &V, b: &V) -> V {
    [
        &a[1] * &b[2] - &a[2] * &b[1],
        &a[2] * &b[0] - &a[0] * &b[2],
        &a[0] * &b[1] - &a[1] * &b[0],
    ]
}
fn dot(a: &V, b: &V) -> BigInt {
    &a[0] * &b[0] + &a[1] * &b[1] + &a[2] * &b[2]
}
fn scale(a: &V, k: &BigInt) -> V {
    [&a[0] * k, &a[1] * k, &a[2] * k]
}

/// `x ≤ n / d`, exactly, for `d > 0`.
fn le_frac(x: f64, n: &BigInt, d: &BigInt) -> bool {
    &big(x) * d <= (n.clone() << (SHIFT as usize))
}
/// `x ≥ n / d`, exactly, for `d > 0`.
fn ge_frac(x: f64, n: &BigInt, d: &BigInt) -> bool {
    &big(x) * d >= (n.clone() << (SHIFT as usize))
}

/// The exact crossing of `ray` with the plane of `tri`: whether it is
/// a point of the CLOSED triangle, and its parameter `nt / dt`
/// (`dt > 0`).
struct Exact {
    inside: bool,
    nt: BigInt,
    dt: BigInt,
}

fn exact(ray: &Ray, tri: &[Point3<f64>; 3]) -> Option<Exact> {
    let zero = BigInt::from(0);
    let (a, b, c) = (vb(tri[0]), vb(tri[1]), vb(tri[2]));
    let (o, d) = (vb(ray.origin), vv(ray.dir));
    let e1 = sub(&b, &a);
    let e2 = sub(&c, &a);
    let s = sub(&o, &a);
    let p = cross(&d, &e2);
    let det = dot(&e1, &p);
    if det == zero {
        return None;
    }
    let nu = dot(&s, &p);
    let q = cross(&s, &e1);
    let nv = dot(&d, &q);
    let (nu_s, nv_s, ad) = if det > zero {
        (nu.clone(), nv.clone(), det.clone())
    } else {
        (-nu.clone(), -nv.clone(), -det.clone())
    };
    let inside = nu_s >= zero && nv_s >= zero && &nu_s + &nv_s <= ad;
    // The crossing point times `det`: det·(a − o) + nu·e1 + nv·e2.
    let pn = add(
        &add(&scale(&sub(&a, &o), &det), &scale(&e1, &nu)),
        &scale(&e2, &nv),
    );
    let mut nt = dot(&pn, &d);
    let mut dt = &det * dot(&d, &d);
    if dt < zero {
        nt = -nt;
        dt = -dt;
    }
    Some(Exact { inside, nt, dt })
}

/// The exact parameter of `a + u·e1 + v·e2` along the ray for the
/// `f64` barycentrics the door clamped to, and whether that point is a
/// point of the closed triangle exactly.
fn exact_clamped(ray: &Ray, tri: &[Point3<f64>; 3], u: f64, v: f64) -> (BigInt, BigInt, bool) {
    let (a, b, c) = (vb(tri[0]), vb(tri[1]), vb(tri[2]));
    let (o, d) = (vb(ray.origin), vv(ray.dir));
    let e1 = sub(&b, &a);
    let e2 = sub(&c, &a);
    let one = BigInt::from(1) << (SHIFT as usize);
    let (bu, bv) = (big(u), big(v));
    let on_triangle = bu >= BigInt::from(0) && bv >= BigInt::from(0) && &bu + &bv <= one;
    let pn = add(
        &add(&scale(&sub(&a, &o), &one), &scale(&e1, &bu)),
        &scale(&e2, &bv),
    );
    let nt = dot(&pn, &d);
    let dt = &one * dot(&d, &d);
    (nt, dt, on_triangle)
}

/// `retract_to_simplex`, restated over [`crossing`]'s barycentrics so
/// the clamped point's own parameter can be checked exactly. Kept in
/// step with the door by hand; what pins the map itself is
/// `pick::tests::the_retraction_keeps_u_plus_v_at_most_one_exactly`,
/// which calls it.
fn clamped(ray: &Ray, tri: &[Point3<f64>; 3]) -> Option<(f64, f64)> {
    let [(u, _), (v, _), _] = crossing(ray, tri)?.barycentrics;
    let u = u.clamp(0.0, 1.0);
    let top = 1.0 - u;
    let top = if 1.0 - top < u { top.next_down() } else { top };
    Some((u, v.clamp(0.0, top)))
}

#[derive(Default, Debug)]
struct Tally {
    drawn: usize,
    admitted: usize,
    /// Admitted with an exact determinant of zero (the door certified
    /// a sign the exact arithmetic does not have).
    exact_parallel: usize,
    /// Admitted, exact crossing on the closed triangle.
    inside: usize,
    /// … and its exact parameter outside `[t_lo, t_hi]`.
    inside_escaped: usize,
    /// Admitted, exact crossing NOT on the closed triangle (a graze
    /// admitted by rounding).
    outside: usize,
    /// … and its exact parameter outside `[t_lo, t_hi]` all the same.
    outside_escaped: usize,
    /// The clamped point's own exact parameter outside the interval.
    clamped_escaped: usize,
    /// The clamped point not a point of the closed triangle, exactly.
    clamped_off_triangle: usize,
    /// max over inside-admitted of `|t* − t| / half`: how much of the
    /// certified half-width the exact crossing uses.
    tightness: f64,
    examples: Vec<String>,
}

fn check(name: &str, ray: &Ray, tri: &[Point3<f64>; 3], tally: &mut Tally) {
    tally.drawn += 1;
    let Some(span) = ray_triangle(ray, tri) else {
        return;
    };
    tally.admitted += 1;
    let (u, v) = clamped(ray, tri).expect("an admitted candidate has a crossing");
    let (cn, cd, on) = exact_clamped(ray, tri, u, v);
    if !on {
        tally.clamped_off_triangle += 1;
    }
    if !(le_frac(span.t_lo, &cn, &cd) && ge_frac(span.t_hi, &cn, &cd)) {
        tally.clamped_escaped += 1;
        if tally.examples.len() < 12 {
            tally.examples.push(format!(
                "{name}: CLAMPED POINT ESCAPES {span:?} ray {ray:?} tri {tri:?} (u, v) = ({u}, {v})"
            ));
        }
    }
    let Some(ex) = exact(ray, tri) else {
        tally.exact_parallel += 1;
        return;
    };
    let enclosed = le_frac(span.t_lo, &ex.nt, &ex.dt) && ge_frac(span.t_hi, &ex.nt, &ex.dt);
    if ex.inside {
        tally.inside += 1;
        // Tightness in f64 (a report, not an assertion).
        let t_star = {
            // nt/dt to f64 through a 60-bit-ish quotient.
            let q = (&ex.nt << 64usize) / &ex.dt;
            let qf: f64 = q.to_string().parse::<f64>().unwrap_or(f64::NAN);
            qf / 2f64.powi(64)
        };
        let half = 0.5 * span.width();
        if half > 0.0 {
            tally.tightness = tally.tightness.max((t_star - span.t).abs() / half);
        }
        if !enclosed {
            tally.inside_escaped += 1;
            if tally.examples.len() < 12 {
                tally.examples.push(format!(
                    "{name}: ESCAPE exact t* = {t_star:e} span {span:?} ray {ray:?} tri {tri:?}"
                ));
            }
        }
    } else {
        tally.outside += 1;
        if !enclosed {
            tally.outside_escaped += 1;
        }
    }
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }
}

/// `x` rounded to the grid `2^-bits`, exact in `f64` (asserted).
fn snap(x: f64, bits: i32) -> f64 {
    let s = 2f64.powi(bits);
    let k = (x * s).round();
    assert!(
        k.abs() < 2f64.powi(53),
        "snap: {x} at 2^-{bits} does not fit a mantissa"
    );
    k / s
}
fn snap3(p: Point3<f64>, bits: i32) -> Point3<f64> {
    Point3::new(snap(p.x, bits), snap(p.y, bits), snap(p.z, bits))
}
fn snapv(v: Vec3<f64>, bits: i32) -> Vec3<f64> {
    Vec3::new(snap(v.x, bits), snap(v.y, bits), snap(v.z, bits))
}

fn tri_general(r: &mut Rng) -> [Point3<f64>; 3] {
    let mut pt = || {
        Point3::new(
            snap(r.range(-8.0, 8.0), 20),
            snap(r.range(-8.0, 8.0), 20),
            snap(r.range(-8.0, 8.0), 20),
        )
    };
    [pt(), pt(), pt()]
}

fn tri_tiny(r: &mut Rng) -> [Point3<f64>; 3] {
    let c = Point3::new(
        snap(r.range(-4.0, 4.0), 20),
        snap(r.range(-4.0, 4.0), 20),
        snap(r.range(-4.0, 4.0), 20),
    );
    let mut pt = || {
        Point3::new(
            snap(c.x + r.range(-1e-6, 1e-6), 48),
            snap(c.y + r.range(-1e-6, 1e-6), 48),
            snap(c.z + r.range(-1e-6, 1e-6), 48),
        )
    };
    [pt(), pt(), pt()]
}

/// A point of the open triangle, away from its boundary.
fn interior(r: &mut Rng, tri: &[Point3<f64>; 3]) -> Point3<f64> {
    let u = r.range(0.05, 0.8);
    let v = r.range(0.05, 0.9 - u);
    tri[0] + (tri[1] - tri[0]) * u + (tri[2] - tri[0]) * v
}

fn transversal_dir(r: &mut Rng) -> Vec3<f64> {
    snapv(
        Vec3::new(r.range(-1.0, 1.0), r.range(-1.0, 1.0), r.range(-1.0, 1.0)) * r.range(0.5, 2.0),
        30,
    )
}

fn near_parallel_dir(r: &mut Rng, tri: &[Point3<f64>; 3]) -> Vec3<f64> {
    let e1 = tri[1] - tri[0];
    let e2 = tri[2] - tri[0];
    let n = e1.cross(e2);
    let n = n * (1.0 / n.norm().max(f64::MIN_POSITIVE));
    let inplane = e1 * r.range(-1.0, 1.0) + e2 * r.range(-1.0, 1.0);
    let scale = inplane.norm().max(1e-3);
    snapv(inplane + n * (scale * 10f64.powf(r.range(-9.0, -2.0))), 40)
}

/// [`aimed`] with its origin rounded to the grid `2^-origin_bits`
/// ([`snap3`]).
fn aimed_snapped(target: Point3<f64>, dir: Vec3<f64>, reach: f64, origin_bits: i32) -> Ray {
    let ray = aimed(target, dir, reach);
    Ray {
        origin: snap3(ray.origin, origin_bits),
        dir,
    }
}

/// **The enclosure, exactly.** For every admitted candidate whose
/// exact crossing is a point of the closed triangle, `[t_lo, t_hi]`
/// contains the exact parameter; and for every admitted candidate at
/// all, it contains the exact parameter of the clamped point the door
/// projected (that is the `PROJECTION_ERROR_UNITS` half of the claim
/// alone). Families: general position, near-parallel rays down to
/// `1e-9` of the normal, origins `2^10`–`2^22` away, directions scaled
/// by `2^±40`, triangles `1e-6` across under far origins, and the
/// `near_tangent` fixture at every `k` from `37` to `400` with the ray
/// through `(1/2, 1/2)` and through a lattice of interior points.
/// One family's draw: a fresh `(ray, triangle)` from the lane's own
/// deterministic generator.
type Draw = Box<dyn FnMut(&mut Rng) -> (Ray, [Point3<f64>; 3])>;

#[test]
fn the_t_interval_encloses_the_exact_crossing_and_the_clamped_point() {
    let mut r = Rng(0x9E37_79B9_7F4A_7C15);
    let mut tallies: Vec<(&str, Tally)> = Vec::new();

    let mut family = |name: &'static str, n: usize, mut draw: Draw| {
        let mut t = Tally::default();
        for _ in 0..n {
            let (ray, tri) = draw(&mut r);
            check(name, &ray, &tri, &mut t);
        }
        tallies.push((name, t));
    };

    family(
        "transversal",
        6000,
        Box::new(|r| {
            let tri = tri_general(r);
            let target = interior(r, &tri);
            let dir = transversal_dir(r);
            (aimed_snapped(target, dir, r.range(0.5, 8.0), 30), tri)
        }),
    );
    family(
        "near-parallel",
        12000,
        Box::new(|r| {
            let tri = tri_general(r);
            let target = interior(r, &tri);
            let dir = near_parallel_dir(r, &tri);
            (aimed_snapped(target, dir, r.range(0.5, 8.0), 30), tri)
        }),
    );
    family(
        "far origin",
        6000,
        Box::new(|r| {
            let tri = tri_general(r);
            let target = interior(r, &tri);
            let dir = if r.unit() < 0.5 {
                transversal_dir(r)
            } else {
                near_parallel_dir(r, &tri)
            };
            let reach = 2f64.powf(r.range(10.0, 22.0));
            (aimed_snapped(target, dir, reach, 20), tri)
        }),
    );
    family(
        "|d| scaled by 2^±40",
        6000,
        Box::new(|r| {
            let tri = tri_general(r);
            let target = interior(r, &tri);
            let dir = if r.unit() < 0.5 {
                transversal_dir(r)
            } else {
                near_parallel_dir(r, &tri)
            };
            let ray = aimed_snapped(target, dir, r.range(0.5, 8.0), 30);
            let k = if r.unit() < 0.5 { 40 } else { -40 };
            (
                Ray {
                    origin: ray.origin,
                    dir: ray.dir * 2f64.powi(k),
                },
                tri,
            )
        }),
    );
    family(
        "tiny triangle, far origin",
        6000,
        Box::new(|r| {
            let tri = tri_tiny(r);
            let target = interior(r, &tri);
            let dir = if r.unit() < 0.5 {
                transversal_dir(r)
            } else {
                near_parallel_dir(r, &tri)
            };
            (aimed_snapped(target, dir, r.range(0.5, 1000.0), 30), tri)
        }),
    );
    {
        let mut t = Tally::default();
        for k in 37..=400u32 {
            let (ray, tri) = near_tangent(k as f64);
            check("near_tangent", &ray, &tri, &mut t);
            let e1 = tri[1] - tri[0];
            let e2 = tri[2] - tri[0];
            for i in 1..8u32 {
                for j in 1..(8 - i) {
                    let target = tri[0] + e1 * (f64::from(i) / 8.0) + e2 * (f64::from(j) / 8.0);
                    let ray2 = aimed(target, ray.dir, 1.0);
                    check("near_tangent interior", &ray2, &tri, &mut t);
                }
            }
        }
        tallies.push(("near_tangent", t));
    }

    let mut inside_escapes = 0;
    let mut clamped_escapes = 0;
    for (name, t) in &tallies {
        println!("# pick3-r2 enclosure, {name}: {t:#?}");
        inside_escapes += t.inside_escaped;
        clamped_escapes += t.clamped_escaped;
    }
    assert_eq!(
        clamped_escapes, 0,
        "the projection's own rounding bound is violated: the clamped point's exact parameter \
         escapes [t_lo, t_hi]"
    );
    assert_eq!(
        inside_escapes, 0,
        "an exact crossing on the closed triangle has its parameter outside [t_lo, t_hi]"
    );
}
