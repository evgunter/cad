//! Reviewer consumer suite for GUI-1 Part A (`Bvh::ray`), R1 lane —
//! an INDEPENDENT derivation of the PR's claims (written from the
//! documented contract, not from the implementation):
//!
//! 1. Conservative superset against an exact-rational ray/box oracle
//!    on integer geometry (`exact_intersects`: i128 cross-multiplied
//!    comparisons, closed boundaries, no floating point anywhere) —
//!    a box the ray TRULY intersects is never absent from the
//!    candidates.
//! 2. `t_enter` is a lower bound on the true entry parameter,
//!    checked exactly on dyadic geometry (power-of-two directions,
//!    integer origins/bounds: every slab quotient is exactly
//!    representable, so the reference entry is exact f64).
//! 3. The documented order (ascending `t_enter` under `total_cmp`,
//!    ties by input index), no duplicate items, and bit-identical
//!    repeats — asserted on every draw of both sweeps.
//!
//! Sweeps are counterexample searches per `memories/test-suite-cost.md`
//! (varying seed via `test_utils::fuzz`, counts on the effort dial).
//!
//! 4. `overflow_prunes_a_truly_hit_box` is a PINNED COUNTEREXAMPLE
//!    (deterministic, written out in full — not a fuzz row): at
//!    adversarial magnitudes, `fl(lo - o)` overflows to −∞ before the
//!    NaN arm can see anything, the products come back +∞ on an axis
//!    whose TRUE parameter interval is moderate, and the widened
//!    `near` (≈ `f64::MAX`) empties the fold against another axis's
//!    honest `far` — the query prunes a box the ray truly intersects,
//!    refuting the conservative-superset contract as documented. RED
//!    at the reviewed head (568bda3) by design; it is the regression
//!    row for the fix.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use bvh::{Aabb, Bvh, Ray, RayCandidate};
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

fn boxed(min: [f64; 3], max: [f64; 3]) -> Aabb {
    Aabb {
        min_x: min[0],
        min_y: min[1],
        min_z: min[2],
        max_x: max[0],
        max_y: max[1],
        max_z: max[2],
    }
}

fn ray(origin: [f64; 3], dir: [f64; 3]) -> Ray {
    Ray {
        origin: Point3::new(origin[0], origin[1], origin[2]),
        dir: Vec3::new(dir[0], dir[1], dir[2]),
    }
}

/// A non-negative rational `num/den` with `den > 0`, plus `+∞`.
#[derive(Clone, Copy, Debug)]
enum Rat {
    Fin { num: i128, den: i128 },
    PosInf,
}

impl Rat {
    fn fin(num: i128, den: i128) -> Self {
        assert!(den != 0, "oracle invariant: nonzero denominator");
        if den < 0 {
            Rat::Fin {
                num: -num,
                den: -den,
            }
        } else {
            Rat::Fin { num, den }
        }
    }

    /// Exact `self <= other`.
    fn le(&self, other: &Rat) -> bool {
        match (self, other) {
            (_, Rat::PosInf) => true,
            (Rat::PosInf, Rat::Fin { .. }) => false,
            (Rat::Fin { num: a, den: b }, Rat::Fin { num: c, den: d }) => a * d <= c * b,
        }
    }

    fn max(self, other: Rat) -> Rat {
        if self.le(&other) { other } else { self }
    }

    fn min(self, other: Rat) -> Rat {
        if self.le(&other) { self } else { other }
    }
}

/// The exact oracle: does `origin + t·dir` meet the CLOSED box
/// `[lo, hi]` for some rational `t ∈ [0, ∞)`? Integer inputs, i128
/// rational arithmetic, no floating point. Independent derivation of
/// the slab test's mathematical content: intersect per-axis closed
/// parameter intervals with the domain `[0, ∞)`; a zero-direction
/// axis constrains iff the origin coordinate lies outside the closed
/// slab. Returns the exact entry parameter when nonempty.
fn exact_entry(o: [i64; 3], d: [i64; 3], lo: [i64; 3], hi: [i64; 3]) -> Option<Rat> {
    let mut t_min = Rat::fin(0, 1);
    let mut t_max = Rat::PosInf;
    for a in 0..3 {
        assert!(lo[a] <= hi[a], "oracle takes well-formed boxes");
        if d[a] == 0 {
            if o[a] < lo[a] || o[a] > hi[a] {
                return None; // parallel and strictly outside: empty
            }
            continue; // inside the closed slab for every t
        }
        let t0 = Rat::fin(i128::from(lo[a]) - i128::from(o[a]), i128::from(d[a]));
        let t1 = Rat::fin(i128::from(hi[a]) - i128::from(o[a]), i128::from(d[a]));
        let (near, far) = if t0.le(&t1) { (t0, t1) } else { (t1, t0) };
        t_min = t_min.max(near);
        t_max = t_max.min(far);
    }
    t_min.le(&t_max).then_some(t_min)
}

/// The documented order, asserted from the contract's own words:
/// ascending `t_enter` under `total_cmp`, ties by ascending item, no
/// duplicate items.
fn assert_documented_order(out: &[RayCandidate], ctx: &str) {
    for w in out.windows(2) {
        let ord = w[0]
            .t_enter
            .total_cmp(&w[1].t_enter)
            .then(w[0].item.cmp(&w[1].item));
        assert!(
            ord == std::cmp::Ordering::Less,
            "{ctx}: candidates out of documented order: {:?} then {:?}; {}",
            w[0],
            w[1],
            fuzz::replay()
        );
    }
}

/// Bit-identical repeat (determinism as documented: same query, same
/// tree, bit-identical output).
fn assert_bit_identical(a: &[RayCandidate], b: &[RayCandidate], ctx: &str) {
    assert_eq!(a.len(), b.len(), "{ctx}: repeat length; {}", fuzz::replay());
    for (x, y) in a.iter().zip(b) {
        assert_eq!(x.item, y.item, "{ctx}: repeat item; {}", fuzz::replay());
        assert_eq!(
            x.t_enter.to_bits(),
            y.t_enter.to_bits(),
            "{ctx}: repeat t_enter bits; {}",
            fuzz::replay()
        );
    }
}

/// Sweep 1 — conservative superset vs the exact oracle, on integer
/// geometry rich in the adversarial corners: zero-extent axes, zero
/// direction components, origins ON slab bounds (the `0 × ∞ → NaN`
/// trap by construction), and grazing rays along box faces.
#[test]
fn exact_oracle_hits_are_never_dropped() {
    let mut rng = fuzz::start("review r1: exact-oracle superset sweep");
    for case in 0..fuzz::scaled(40) {
        let n = rng.below(24) + 1;
        let mut int_boxes = Vec::with_capacity(n);
        let mut boxes = Vec::with_capacity(n);
        for _ in 0..n {
            let mut lo = [0i64; 3];
            let mut hi = [0i64; 3];
            for a in 0..3 {
                let c = rng.below(2001) as i64 - 1000;
                // Zero extent with probability ~1/4.
                let e = if rng.below(4) == 0 {
                    0
                } else {
                    rng.below(300) as i64
                };
                lo[a] = c - e;
                hi[a] = c + e;
            }
            int_boxes.push((lo, hi));
            boxes.push(boxed(
                [lo[0] as f64, lo[1] as f64, lo[2] as f64],
                [hi[0] as f64, hi[1] as f64, hi[2] as f64],
            ));
        }
        let tree = Bvh::build(&boxes);

        let mut o = [0i64; 3];
        let mut d = [0i64; 3];
        for a in 0..3 {
            o[a] = rng.below(4001) as i64 - 2000;
            // Zero direction with probability ~1/4; small otherwise.
            d[a] = if rng.below(4) == 0 {
                0
            } else {
                rng.below(41) as i64 - 20
            };
        }
        // With probability ~1/2, park the origin exactly on a bound of
        // a chosen box on one axis (the NaN trap / graze corner).
        if rng.below(2) == 0 {
            let k = rng.below(n);
            let a = rng.below(3);
            let (lo, hi) = int_boxes[k];
            o[a] = if rng.below(2) == 0 { lo[a] } else { hi[a] };
            if rng.below(2) == 0 {
                d[a] = 0; // graze exactly along the face plane
            }
        }
        let r = ray(
            [o[0] as f64, o[1] as f64, o[2] as f64],
            [d[0] as f64, d[1] as f64, d[2] as f64],
        );

        let out = tree.ray(&r);
        assert_documented_order(&out, &format!("case {case}"));
        assert_bit_identical(&out, &tree.ray(&r), &format!("case {case}"));

        for (item, &(lo, hi)) in int_boxes.iter().enumerate() {
            if exact_entry(o, d, lo, hi).is_some() {
                assert!(
                    out.iter().any(|c| c.item == item),
                    "case {case}: box {item} ({lo:?}..{hi:?}) is truly hit by \
                     o={o:?} d={d:?} but absent from candidates; {}",
                    fuzz::replay()
                );
            }
        }
    }
}

/// Sweep 2 — `t_enter` lower-bounds the true entry, checked EXACTLY:
/// power-of-two directions and integer origins/bounds make every true
/// slab quotient exactly representable, so the reference entry
/// (computed in exact f64 arithmetic by construction) is the true
/// rational entry and the comparison is exact.
#[test]
fn t_enter_lower_bounds_the_exact_entry_on_dyadic_geometry() {
    let mut rng = fuzz::start("review r1: dyadic t_enter lower-bound sweep");
    for case in 0..fuzz::scaled(40) {
        let n = rng.below(16) + 1;
        let mut int_boxes = Vec::with_capacity(n);
        let mut boxes = Vec::with_capacity(n);
        for _ in 0..n {
            let mut lo = [0i64; 3];
            let mut hi = [0i64; 3];
            for a in 0..3 {
                let c = rng.below(513) as i64 - 256;
                let e = if rng.below(4) == 0 {
                    0
                } else {
                    rng.below(65) as i64
                };
                lo[a] = c - e;
                hi[a] = c + e;
            }
            int_boxes.push((lo, hi));
            boxes.push(boxed(
                [lo[0] as f64, lo[1] as f64, lo[2] as f64],
                [hi[0] as f64, hi[1] as f64, hi[2] as f64],
            ));
        }
        let tree = Bvh::build(&boxes);

        let mut o = [0i64; 3];
        let mut d = [0i64; 3];
        for a in 0..3 {
            o[a] = rng.below(1025) as i64 - 512;
            // Direction: 0 or ±2^j (j ≤ 4) — every quotient dyadic.
            d[a] = if rng.below(4) == 0 {
                0
            } else {
                let mag = 1i64 << rng.below(5);
                if rng.below(2) == 0 { mag } else { -mag }
            };
        }
        let r = ray(
            [o[0] as f64, o[1] as f64, o[2] as f64],
            [d[0] as f64, d[1] as f64, d[2] as f64],
        );

        let out = tree.ray(&r);
        assert_documented_order(&out, &format!("case {case}"));

        for (item, &(lo, hi)) in int_boxes.iter().enumerate() {
            // The exact entry, in f64 arithmetic that is exact by
            // construction (integer subtractions < 2^53; divisions by
            // powers of two): max(0, per-axis near) over constraining
            // axes; the closed d = 0 inside-slab case constrains
            // nothing.
            let mut t_min = 0.0f64;
            let mut t_max = f64::INFINITY;
            let mut empty = false;
            for a in 0..3 {
                if d[a] == 0 {
                    if o[a] < lo[a] || o[a] > hi[a] {
                        empty = true;
                    }
                    continue;
                }
                let t0 = (lo[a] - o[a]) as f64 / d[a] as f64;
                let t1 = (hi[a] - o[a]) as f64 / d[a] as f64;
                let (near, far) = if t0 <= t1 { (t0, t1) } else { (t1, t0) };
                t_min = t_min.max(near);
                t_max = t_max.min(far);
            }
            if empty || t_min > t_max {
                continue; // no true intersection: nothing to bound
            }
            let cand = out.iter().find(|c| c.item == item);
            let cand = cand.unwrap_or_else(|| {
                panic!(
                    "case {case}: truly-hit dyadic box {item} absent; {}",
                    fuzz::replay()
                )
            });
            assert!(
                cand.t_enter <= t_min,
                "case {case}: t_enter {} exceeds the exact entry {} for box \
                 {item} ({lo:?}..{hi:?}, o={o:?} d={d:?}); {}",
                cand.t_enter,
                t_min,
                fuzz::replay()
            );
        }
    }
}

/// PINNED COUNTEREXAMPLE (deterministic; see the file header). The
/// x-axis subtraction `lo − o = −1.5e308 − 1.7e308` overflows to −∞,
/// `(−∞)·inv` with `inv < 0` gives `near = far = +∞` (no NaN, so the
/// skip arm never fires), widening turns `near` into ≈ `f64::MAX`,
/// and the y-axis's honest `far ≈ 1.6` empties the fold — yet the
/// TRUE x-interval is ≈ [1.588, 1.882] and the ray genuinely passes
/// through the box at t ≈ 1.59. The contract ("must never answer
/// 'disjoint' for a box the ray truly intersects") is violated.
#[test]
fn overflow_prunes_a_truly_hit_box() {
    let b = boxed([-1.5e308, 0.5, -1.0], [-1.0e308, 1.6, 1.0]);
    let r = ray([1.7e308, 0.0, 0.0], [-1.7e308, 1.0, 0.0]);
    // The true intersection, derived without any overflowing
    // subtraction: x enters at (1.7e308 − 1.0e308)/1.7e308 ≈ 0.41 …
    // no — parameterize: x(t) = 1.7e308 · (1 − t), so x ∈ [−1.5e308,
    // −1.0e308] for t ∈ [2.7/1.7, 3.2/1.7] ≈ [1.588, 1.882]; y(t) = t
    // ∈ [0.5, 1.6]; z(t) = 0 ∈ [−1, 1]. Intersection over t:
    // [1.588…, 1.6] — nonempty, forward. Sanity-check it in exact
    // integer arithmetic scaled by 1e306: x(t)·1e−306 = 170(1 − t),
    // bounds [−150, −100] ⇒ t ∈ [270/170, 320/170].
    // (270/170 ≈ 1.588 < 1.6: the true interval is nonempty.)
    let tree = Bvh::build(&[b]);
    assert_eq!(
        tree.ray(&r).len(),
        1,
        "a truly intersected box must stay a candidate (conservative \
         superset) — the overflowed x slab wrongly pruned it"
    );
    // The same violation through the single-box door, for locality.
    assert!(
        r.slab_enter(&b).is_some(),
        "slab_enter answered 'definitely disjoint' for a box the ray \
         truly intersects (intermediate overflow in lo − o)"
    );
}
