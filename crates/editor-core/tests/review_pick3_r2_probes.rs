//! Review lane `pick3-r2` (EDIT-PICK3, PR #2786, frozen head
//! `31cbee19f`): probes that falsify the `t`-interval door's claims by
//! execution. Each row asserts what the door's own docs claim, so a
//! red row is a claim the head does not keep.
//!
//! 1. `the_certified_tie_is_decided_by_the_targets_order_through_the_early_out`
//!    — the door's contract says the certified tie is a rule about the
//!    SET and that the early-out "prunes only candidates that could not
//!    win". A narrower member of the tie whose interval reaches back
//!    below its own box entry is pruned, and the SAME two triangles
//!    answer differently depending on which target is offered first.
//! 2. `the_t_interval_encloses_the_exact_crossing_and_the_clamped_point`
//!    — exact rational arithmetic (`num-bigint`) over stressed families
//!    (near-parallel rays, far origins, `|d| ≫ 1` and `≪ 1`, tiny
//!    triangles, the `near_tangent` fixture at every `k`): does
//!    `[t_lo, t_hi]` enclose the exact crossing's parameter whenever
//!    that crossing is on the closed triangle, and the clamped point's
//!    own parameter always?
//! 3. `the_clamped_point_is_a_point_of_the_closed_triangle_to_the_bit`
//!    — the clamp's `v` bound is `fl(1 − u)`, which rounds up to `1` for
//!    a tiny `u`, so the answered point can sit outside the closed
//!    triangle by `u·|e1|`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

test_utils::gated_to![
    "crates/editor-core/src/resolve/",
    "crates/bvh/src/",
    "crates/editor-core/tests/fixture/",
];

use crate::fixture;

use bvh::{Aabb, Ray};
use editor_core::resolve::{TSpan, crossing, ray_triangle};
use editor_core::{
    CancelToken, EvalOptions, Evaluation, MeshPick, Node, PickTarget, ProfileDoc, RecipeNodeId,
    ValuePayload, pick_face,
};
use fixture::{insert, len, on_frame};
use geom_core::{Point3, Tol, Vec3};
use mesh::Mesh;
use num_bigint::BigInt;
use topo::Body;

// ---------------------------------------------------------------
// 1. The early-out prunes a member of the certified tie, and the
//    answer then depends on the targets' order.
// ---------------------------------------------------------------

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    editor_core::evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn cube(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    )
}

fn mesh_of(ev: &Evaluation<f64>, node: RecipeNodeId) -> Mesh {
    let body: &Body<f64> = match &ev.value(node).expect("extrude evaluates").payload {
        ValuePayload::Body(b) => b,
        other => panic!("extrude payload is a body, got {}", other.kind_name()),
    };
    mesh::tessellate(body, 0.1, Tol::witness()).expect("box tessellates")
}

/// A mesh carrying `tris` as one triangle per patch, under the first
/// patches' face keys of `base` (so `pick_face` names the winner
/// through the real tables). `MeshPick` copies geometry out, so the
/// index is exactly these triangles.
fn mesh_over(base: &Mesh, tris: &[[Point3<f64>; 3]]) -> Mesh {
    let mut positions = Vec::new();
    let mut patches = Vec::new();
    for (i, tri) in tris.iter().enumerate() {
        let k = positions.len() as u32;
        positions.extend_from_slice(tri);
        let mut patch = base.patches[i].clone();
        patch.triangles = vec![[k, k + 1, k + 2]];
        patches.push(patch);
    }
    Mesh {
        positions,
        patches,
        boundaries: Vec::new(),
    }
}

/// `pick.rs`'s `near_tangent` fixture: a determinant certified at
/// `k / 6` of its own bound, `u = v = 0.5` exactly, `t = 1.5`.
fn near_tangent(k: f64) -> (Ray, [Point3<f64>; 3]) {
    let zeta = 2f64.powi(-20);
    let xi = k * zeta * f64::EPSILON;
    let tri = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, zeta + xi),
        Point3::new(0.0, 1.0, 0.0),
    ];
    let ray = Ray {
        origin: Point3::new(-1.0, -1.0, 0.5 * xi - zeta),
        dir: Vec3::new(1.0, 1.0, zeta),
    };
    (ray, tri)
}

/// The same shape at `k`, scaled by `lambda` and placed so `ray` meets
/// it at `u = v = 0.5` at parameter `t_a`.
fn near_tangent_copy(k: f64, lambda: f64, t_a: f64, ray: &Ray) -> [Point3<f64>; 3] {
    let zeta = 2f64.powi(-20);
    let xi = k * zeta * f64::EPSILON;
    let e1 = Vec3::new(lambda, 0.0, lambda * (zeta + xi));
    let e2 = Vec3::new(0.0, lambda, 0.0);
    let p = ray.origin + ray.dir * t_a;
    let a = p - (e1 + e2) * 0.5;
    [a, a + e1, a + e2]
}

fn entry(ray: &Ray, tri: &[Point3<f64>; 3]) -> f64 {
    ray.slab_enter(&Aabb::from_points(*tri).expect("three points box"))
        .expect("the ray enters the box")
}

/// **The early-out is not a pure optimisation of the set rule.**
///
/// `pick_face`'s contract: a candidate no other candidate PRECEDES is
/// in the certified tie, the tie is decided by width then position,
/// and the early-out "prunes only candidates that could not win" —
/// "once the smallest `t_hi` seen is strictly below a candidate's
/// `t_enter`, every remaining candidate's true parameter exceeds it
/// and nothing further can win in exact arithmetic". That was true of
/// the rounded-`t` order and is false of the width rule: a candidate
/// B whose box is entered AFTER `t_hi(A)` can still have
/// `t_lo(B) ≤ t_hi(A)` (its interval reaches back below its box), so
/// B is in the tie, and if B is narrower than A the rule answers B.
/// The traversal never tests B.
///
/// Two `near_tangent` triangles on one ray: B at `k = 38`-ish (`t =
/// 1.5`, box entered at `t = 1`, interval reaching below `1`), and A,
/// a scaled copy nearer the origin whose `t_hi` lands in
/// `[t_lo(B), t_enter(B))`. The search over `(k_a, k_b, λ, t_a)` is
/// deterministic and prints what it found. Then the REAL door:
///
/// - both triangles in one target: the walk breaks before B, answers A;
/// - B's target first, A's second: both tested, B wins the tie;
/// - A's target first, B's second: breaks before B, answers A.
///
/// The rule says B in every case, so this row asserts B — and it is
/// red on the head, twice: the pruned answer, and the answer depending
/// on the targets' order, which the same contract forbids.
#[test]
fn the_certified_tie_is_decided_by_the_targets_order_through_the_early_out() {
    let mut found = None;
    'search: for k_b in 37..=60u32 {
        let (ray, b) = near_tangent(k_b as f64);
        let Some(span_b) = ray_triangle(&ray, &b) else {
            continue;
        };
        let enter_b = entry(&ray, &b);
        if span_b.t_lo >= enter_b {
            continue;
        }
        for k_a in 37..=60u32 {
            for lambda in [1.5, 2.0, 3.0, 4.0] {
                for j in 1..64u32 {
                    let t_a = f64::from(j) / 64.0;
                    let a = near_tangent_copy(k_a as f64, lambda, t_a, &ray);
                    let Some(span_a) = ray_triangle(&ray, &a) else {
                        continue;
                    };
                    let enter_a = entry(&ray, &a);
                    if enter_a < enter_b
                        && span_a.t_hi < enter_b
                        && span_b.t_lo <= span_a.t_hi
                        && span_b.width() < span_a.width()
                    {
                        found = Some((ray, a, b, span_a, span_b, enter_a, enter_b));
                        break 'search;
                    }
                }
            }
        }
    }
    let (ray, a, b, span_a, span_b, enter_a, enter_b) =
        found.expect("the search finds a wide A whose t_hi lands between B's t_lo and B's box entry");
    println!(
        "# pick3-r2: A {a:?}\n#   span {span_a:?} width {} entry {enter_a}\n# B {b:?}\n#   span \
         {span_b:?} width {} entry {enter_b}",
        span_a.width(),
        span_b.width()
    );
    // The premises, restated as assertions so the row cannot pass on a
    // fixture that drifted.
    assert!(span_a.t_hi < enter_b, "A's upper end is below B's box entry: the walk breaks");
    assert!(
        span_b.t_lo <= span_a.t_hi && !span_a.precedes(&span_b) && !span_b.precedes(&span_a),
        "neither precedes the other: they are one certified tie"
    );
    assert!(span_b.width() < span_a.width(), "and B is the narrower claim");

    let doc = ProfileDoc::empty_derived("pick3_r2_early_out", Tol::witness());
    let (doc, node) = cube(doc);
    let ev = run(&doc);
    let base = mesh_of(&ev, node);

    let both = mesh_over(&base, &[a, b]);
    let pick_both = MeshPick::build(&both).expect("two triangles index");
    let only_a = mesh_over(&base, &[a]);
    let only_b = mesh_over(&base, &[b]);
    let pick_a = MeshPick::build(&only_a).expect("A indexes");
    let pick_b = MeshPick::build(&only_b).expect("B indexes");

    let one_target = pick_face(&ev, &[PickTarget::new(&ev, node, 0, &pick_both)], &ray)
        .expect("no error")
        .expect("a hit");
    let b_then_a = pick_face(
        &ev,
        &[
            PickTarget::new(&ev, node, 0, &pick_b),
            PickTarget::new(&ev, node, 0, &pick_a),
        ],
        &ray,
    )
    .expect("no error")
    .expect("a hit");
    let a_then_b = pick_face(
        &ev,
        &[
            PickTarget::new(&ev, node, 0, &pick_a),
            PickTarget::new(&ev, node, 0, &pick_b),
        ],
        &ray,
    )
    .expect("no error")
    .expect("a hit");
    println!(
        "# pick3-r2: one target answers t = {} ; [B, A] answers {} ; [A, B] answers {}",
        one_target.t, b_then_a.t, a_then_b.t
    );
    assert_eq!(
        b_then_a.t, span_b.t,
        "with B tested first, both survive and the narrower claim B wins (the rule's answer)"
    );
    assert_eq!(
        a_then_b.t, b_then_a.t,
        "the contract: the certified tie is decided by the candidates, not by the order the \
         targets were offered in — [A, B] answers {} and [B, A] answers {}",
        a_then_b.t, b_then_a.t
    );
    assert_eq!(
        one_target.t, span_b.t,
        "the early-out prunes only candidates that could not win: with both triangles in one \
         target the door must still answer B, but answers t = {}",
        one_target.t
    );
}

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

/// The door's clamp, restated (two lines of `ray_triangle`) so the
/// clamped point's own parameter can be checked exactly.
fn clamped(ray: &Ray, tri: &[Point3<f64>; 3]) -> Option<(f64, f64)> {
    let [(u, _), (v, _), _] = crossing(ray, tri)?.barycentrics;
    let u = u.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0 - u);
    Some((u, v))
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
    assert!(k.abs() < 2f64.powi(53), "snap: {x} at 2^-{bits} does not fit a mantissa");
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

fn aimed(target: Point3<f64>, dir: Vec3<f64>, reach: f64, origin_bits: i32) -> Ray {
    Ray {
        origin: snap3(target - dir * reach, origin_bits),
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
#[test]
fn the_t_interval_encloses_the_exact_crossing_and_the_clamped_point() {
    let mut r = Rng(0x9E37_79B9_7F4A_7C15);
    let mut tallies: Vec<(&str, Tally)> = Vec::new();

    let mut family = |name: &'static str, n: usize, mut draw: Box<dyn FnMut(&mut Rng) -> (Ray, [Point3<f64>; 3])>| {
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
            (aimed(target, dir, r.range(0.5, 8.0), 30), tri)
        }),
    );
    family(
        "near-parallel",
        12000,
        Box::new(|r| {
            let tri = tri_general(r);
            let target = interior(r, &tri);
            let dir = near_parallel_dir(r, &tri);
            (aimed(target, dir, r.range(0.5, 8.0), 30), tri)
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
            (aimed(target, dir, reach, 20), tri)
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
            let ray = aimed(target, dir, r.range(0.5, 8.0), 30);
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
            (aimed(target, dir, r.range(0.5, 1000.0), 30), tri)
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
                    let ray2 = Ray {
                        origin: target - ray.dir,
                        dir: ray.dir,
                    };
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

// ---------------------------------------------------------------
// 3. The clamp's own rounding.
// ---------------------------------------------------------------

/// **The clamped point is claimed to be a point OF the closed triangle
/// ("to the bit", `every_admitted_hit_is_placed_on_the_closed_triangle`).**
/// The `v` bound is `fl(1 − u)`, which rounds to `1` for `u ≤ 2⁻⁵⁴`
/// (half-even); so at `u = 2⁻⁵⁴`, `v = 1` — admitted, since
/// `fl(u + v) = 1` — the door's point is `a + 2⁻⁵⁴·e1 + e2`, outside
/// the closed triangle by `2⁻⁵⁴·|e1|`. The existing row cannot see it
/// because its membership check is itself `f64` (`bu + bv <= 1.0`
/// rounds back to `1`).
#[test]
fn the_clamped_point_is_a_point_of_the_closed_triangle_to_the_bit() {
    let tri = [
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(5.0, 1.0, 1.0),
        Point3::new(1.0, 5.0, 1.0),
    ];
    let ray = Ray {
        origin: Point3::new(1.0 + 2f64.powi(-52), 5.0, 3.0),
        dir: Vec3::new(0.0, 0.0, -1.0),
    };
    let span = ray_triangle(&ray, &tri).expect("u = 2^-54, v = 1 is admitted");
    let (u, v) = clamped(&ray, &tri).expect("a crossing");
    println!("# pick3-r2 clamp: span {span:?}, clamped (u, v) = ({u:e}, {v})");
    assert_eq!((u, v), (2f64.powi(-54), 1.0), "the fixture's barycentrics");
    let (_, _, on) = exact_clamped(&ray, &tri, u, v);
    assert!(
        on,
        "the answered point a + {u:e}·e1 + {v}·e2 is a point of the closed triangle exactly"
    );
}
