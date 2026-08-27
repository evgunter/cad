//! `Bvh::ray` held against oracles written INDEPENDENTLY of the query
//! (review lane R2, GUI-1).
//!
//! The suite in `ray.rs` checks `Bvh::ray` against `Ray::slab_enter`
//! applied item by item — a real property (tree shape never leaks into
//! the answer) but not an independent one: both sides call the same
//! float code, so a wrong slab test agrees with itself. Everything
//! here is built so that it CANNOT agree with itself:
//!
//! 1. [`exact_hit_interval`] decides "does this ray truly meet this
//!    box over `t ≥ 0`" in **exact integer arithmetic** on `i128`
//!    rationals, over geometry drawn from small integers. No `f64`
//!    arithmetic is involved in the verdict, so it is an oracle for
//!    the conservative-superset contract rather than a mirror of it.
//! 2. [`permutation_invariance`] rebuilds the same box set under a
//!    random permutation. The tree shape changes; the answer, mapped
//!    back through the permutation, must not — bit-for-bit on
//!    `t_enter`. Nothing about the slab test is assumed.
//! 3. The `t_enter`-is-a-lower-bound claim is checked against the
//!    exact rational entry parameter, not against a re-run of the
//!    same arithmetic.
//!
//! Sweep shape (per `memories/test-suite-cost.md`): counterexample
//! search, varying seed, counts on the effort dial.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use bvh::{Aabb, Bvh, Ray};
use geom_core::{Point3, Vec3};
use test_utils::{fuzz, vacuity::Exposure};

/// An exact rational `n / d` with `d > 0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Q {
    n: i128,
    d: i128,
}

impl Q {
    fn new(n: i128, d: i128) -> Self {
        if d < 0 {
            Self { n: -n, d: -d }
        } else {
            Self { n, d }
        }
    }
    fn zero() -> Self {
        Self { n: 0, d: 1 }
    }
    /// Exact ordering — cross-multiplication, both denominators
    /// positive by construction.
    fn le(self, other: Self) -> bool {
        self.n * other.d <= other.n * self.d
    }
    fn max(self, other: Self) -> Self {
        if self.le(other) { other } else { self }
    }
    fn min(self, other: Self) -> Self {
        if self.le(other) { self } else { other }
    }
    fn to_f64(self) -> f64 {
        self.n as f64 / self.d as f64
    }
}

/// The EXACT set of `t ≥ 0` at which the ray is inside the box, as a
/// closed interval `[lo, hi]`, or `None` when the ray truly misses.
///
/// Integer geometry only: every quantity here is an `i128` rational,
/// so the verdict is exact — this is the oracle the conservative
/// superset is measured against, and it shares no code with the
/// implementation.
fn exact_hit_interval(o: [i64; 3], d: [i64; 3], lo: [i64; 3], hi: [i64; 3]) -> Option<(Q, Q)> {
    // Start from the ray's own domain [0, +inf); +inf is carried as a
    // flag rather than a rational.
    let mut t_lo = Q::zero();
    let mut t_hi: Option<Q> = None; // None = +infinity
    for a in 0..3 {
        let (o, d, lo, hi) = (
            i128::from(o[a]),
            i128::from(d[a]),
            i128::from(lo[a]),
            i128::from(hi[a]),
        );
        if lo > hi {
            return None; // empty box (the crate's inverted convention)
        }
        if d == 0 {
            // Parallel to the slab: all t, or none.
            if o < lo || o > hi {
                return None;
            }
            continue;
        }
        let a0 = Q::new(lo - o, d);
        let a1 = Q::new(hi - o, d);
        let (near, far) = if a0.le(a1) { (a0, a1) } else { (a1, a0) };
        t_lo = t_lo.max(near);
        t_hi = Some(match t_hi {
            None => far,
            Some(h) => h.min(far),
        });
    }
    match t_hi {
        None => Some((t_lo, Q::new(i128::from(i64::MAX), 1))), // unbounded above
        Some(h) if t_lo.le(h) => Some((t_lo, h)),
        Some(_) => None,
    }
}

fn boxed(lo: [i64; 3], hi: [i64; 3]) -> Aabb {
    Aabb {
        min_x: lo[0] as f64,
        min_y: lo[1] as f64,
        min_z: lo[2] as f64,
        max_x: hi[0] as f64,
        max_y: hi[1] as f64,
        max_z: hi[2] as f64,
    }
}

fn ray_of(o: [i64; 3], d: [i64; 3]) -> Ray {
    Ray {
        origin: Point3::new(o[0] as f64, o[1] as f64, o[2] as f64),
        dir: Vec3::new(d[0] as f64, d[1] as f64, d[2] as f64),
    }
}

fn small(rng: &mut fuzz::Rng, span: i64) -> i64 {
    rng.below((2 * span + 1) as usize) as i64 - span
}

/// **The conservative-superset contract against an EXACT oracle.**
///
/// Integer boxes and integer rays, so `exact_hit_interval` decides the
/// truth in `i128`. Two directions per draw:
///
/// - every box the ray truly meets is a candidate (the contract);
/// - the reported `t_enter` never exceeds the exact entry parameter
///   (the lower-bound claim `pick_face`'s early-out rests on).
///
/// The converse — candidates that truly miss — is legal and NOT
/// asserted; the contract is one-sided.
#[test]
fn candidates_are_a_superset_of_the_exact_integer_truth() {
    let mut rng = fuzz::start("bvh::ray exact-integer superset (R2)");
    // The vacuity hazard here is real: a sweep whose rays all miss
    // every box asserts nothing and stays green. The floors below are
    // stated against `CAD_FUZZ_EFFORT=1` (measured at effort 1:
    // ~250 true hits, ~40 zero-direction draws, ~30 plane boxes).
    let mut seen = Exposure::new("bvh::ray exact-integer superset (R2)");
    for case in 0..fuzz::scaled(80) {
        let n = rng.below(24) + 1;
        let mut lo = Vec::with_capacity(n);
        let mut hi = Vec::with_capacity(n);
        for _ in 0..n {
            let mut l = [0i64; 3];
            let mut h = [0i64; 3];
            for a in 0..3 {
                let c = small(&mut rng, 20);
                // Zero extent ~1/4 of the time (the plane-box corner).
                let e = if rng.below(4) == 0 {
                    0
                } else {
                    rng.below(9) as i64
                };
                l[a] = c - e;
                h[a] = c + e;
            }
            lo.push(l);
            hi.push(h);
        }
        let boxes: Vec<Aabb> = (0..n).map(|i| boxed(lo[i], hi[i])).collect();
        let tree = Bvh::build(&boxes);

        let o = [
            small(&mut rng, 30),
            small(&mut rng, 30),
            small(&mut rng, 30),
        ];
        // Aiming matters: a uniformly random direction through a
        // sparse box field almost never hits anything, and a sweep
        // that never hits asserts nothing (the floors below caught
        // exactly that on the first draft of this row). Three
        // quarters of the draws are therefore AIMED at an integer
        // lattice point inside a randomly chosen box — which keeps
        // every quantity an exact integer, so the oracle stays exact —
        // and one quarter is a free direction, which is what reaches
        // the pruning side and the zero-direction corners.
        let d = if rng.below(4) == 0 {
            [
                if rng.below(4) == 0 {
                    0
                } else {
                    small(&mut rng, 8)
                },
                if rng.below(4) == 0 {
                    0
                } else {
                    small(&mut rng, 8)
                },
                if rng.below(4) == 0 {
                    0
                } else {
                    small(&mut rng, 8)
                },
            ]
        } else {
            let k = rng.below(n);
            let mut aim = [0i64; 3];
            for a in 0..3 {
                let span = (hi[k][a] - lo[k][a] + 1) as usize;
                aim[a] = lo[k][a] + rng.below(span) as i64;
            }
            [aim[0] - o[0], aim[1] - o[1], aim[2] - o[2]]
        };
        if d.contains(&0) {
            seen.note("draws with a zero direction component");
        }
        if (0..n).any(|i| (0..3).any(|a| lo[i][a] == hi[i][a])) {
            seen.note("draws containing a zero-extent (plane) box");
        }
        let r = ray_of(o, d);
        let got = tree.ray(&r);

        for i in 0..n {
            let Some((entry, _)) = exact_hit_interval(o, d, lo[i], hi[i]) else {
                continue; // a true miss: keeping it is legal, dropping it is too
            };
            seen.note("boxes the ray TRULY meets");
            if entry == Q::zero() {
                seen.note("true hits entering at t = 0 (origin inside)");
            }
            let cand = got.iter().find(|c| c.item == i);
            let Some(cand) = cand else {
                panic!(
                    "case {case}: box {i} {:?}..{:?} is TRULY met by ray o={o:?} d={d:?} \
                     at exact t = {}/{}, but Bvh::ray dropped it; {}",
                    lo[i],
                    hi[i],
                    entry.n,
                    entry.d,
                    fuzz::replay()
                );
            };
            // The lower-bound claim, against the exact entry: a
            // conservative t_enter is never ABOVE the truth. Compared
            // through the correctly-rounded f64 image of the exact
            // rational plus one relative ulp of slack, which is
            // strictly weaker than the claim and so cannot mask a
            // violation of any size that matters.
            let exact = entry.to_f64();
            let slack = exact.abs().mul_add(4.0 * f64::EPSILON, f64::EPSILON);
            assert!(
                cand.t_enter <= exact + slack,
                "case {case}: t_enter {} exceeds the exact entry {}/{} = {exact} for box {i}; {}",
                cand.t_enter,
                entry.n,
                entry.d,
                fuzz::replay()
            );
        }
    }
    seen.report();
    seen.require(
        "boxes the ray TRULY meets",
        40,
        &format!(
            "the sweep drew almost no true intersections, so the \
             superset contract was never actually put in scope; {}",
            fuzz::replay()
        ),
    );
    seen.require(
        "draws with a zero direction component",
        5,
        &format!(
            "the 0 x inf NaN corner was never entered; {}",
            fuzz::replay()
        ),
    );
    seen.require(
        "draws containing a zero-extent (plane) box",
        5,
        &format!("the plane-box corner was never entered; {}", fuzz::replay()),
    );
}

/// **Tree shape never leaks into the answer — checked without the
/// slab test.**
///
/// The same boxes under a random permutation build a differently
/// shaped tree (the build splits on centroid order). Mapped back
/// through the permutation, the answer must be identical — same set,
/// same `t_enter` bit pattern, and the same relative order among
/// entries whose `t_enter` ties, since the documented tie-break is
/// ascending input index.
#[test]
fn permutation_invariance_of_the_candidate_answer() {
    let mut rng = fuzz::start("bvh::ray permutation invariance (R2)");
    for case in 0..fuzz::scaled(40) {
        let n = rng.below(30) + 2;
        let boxes: Vec<Aabb> = (0..n)
            .map(|_| {
                let c = [
                    rng.range(-50.0, 50.0),
                    rng.range(-50.0, 50.0),
                    rng.range(-50.0, 50.0),
                ];
                let e = [
                    rng.range(0.0, 12.0),
                    rng.range(0.0, 12.0),
                    rng.range(0.0, 12.0),
                ];
                Aabb {
                    min_x: c[0] - e[0],
                    min_y: c[1] - e[1],
                    min_z: c[2] - e[2],
                    max_x: c[0] + e[0],
                    max_y: c[1] + e[1],
                    max_z: c[2] + e[2],
                }
            })
            .collect();
        // A duplicate box guarantees at least one exact t_enter tie,
        // so the index tie-break is exercised rather than assumed.
        let mut boxes = boxes;
        let src = rng.below(n);
        let dst = rng.below(n);
        boxes[dst] = boxes[src];

        let r = Ray {
            origin: Point3::new(
                rng.range(-90.0, 90.0),
                rng.range(-90.0, 90.0),
                rng.range(-90.0, 90.0),
            ),
            dir: Vec3::new(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            ),
        };

        // perm[new_index] = old_index
        let mut perm: Vec<usize> = (0..n).collect();
        for i in (1..n).rev() {
            perm.swap(i, rng.below(i + 1));
        }
        let permuted: Vec<Aabb> = perm.iter().map(|&o| boxes[o]).collect();

        let base = Bvh::build(&boxes).ray(&r);
        let other = Bvh::build(&permuted).ray(&r);

        // Same multiset of (old index, t_enter bits).
        let mut a: Vec<(usize, u64)> = base.iter().map(|c| (c.item, c.t_enter.to_bits())).collect();
        let mut b: Vec<(usize, u64)> = other
            .iter()
            .map(|c| (perm[c.item], c.t_enter.to_bits()))
            .collect();
        a.sort_unstable();
        b.sort_unstable();
        assert_eq!(
            a,
            b,
            "case {case}: permuting the input changed the candidate set / t_enter; {}",
            fuzz::replay()
        );

        // And the ORDER is a function of (t_enter, original index)
        // only: sorting the permuted answer by the documented key
        // reproduces the unpermuted answer exactly.
        let mut b_ord: Vec<(u64, usize)> = other
            .iter()
            .map(|c| (c.t_enter.to_bits(), perm[c.item]))
            .collect();
        b_ord.sort_unstable_by(|x, y| {
            f64::from_bits(x.0)
                .total_cmp(&f64::from_bits(y.0))
                .then(x.1.cmp(&y.1))
        });
        let a_ord: Vec<(u64, usize)> = base.iter().map(|c| (c.t_enter.to_bits(), c.item)).collect();
        assert_eq!(
            a_ord,
            b_ord,
            "case {case}: documented order is not a function of (t_enter, index); {}",
            fuzz::replay()
        );
    }
}

/// The documented index tie-break, at scale — **EVIDENCE, not a gate
/// on the tie-break**.
///
/// Stated plainly because the attempt failed: deleting
/// `.then(a.item.cmp(&b.item))` from `Bvh::ray`'s comparator leaves
/// THIS row green, and leaves `ray.rs`'s order row green, and leaves
/// the sweep green on roughly a third of seeds (measured: 3 red of 5
/// mutant runs across two constructions). The cause is structural —
/// the traversal is left-child-first over an items array the build
/// already ordered, so the pre-sort sequence is normally *already*
/// index-ascending inside each tie group and an unstable sort has
/// nothing to disturb. No deterministic construction found here
/// reaches the mutant.
///
/// What the row does carry: 200 boxes in two exact tie groups of 100,
/// with the near group last in input order, come back grouped by
/// `t_enter` and index-ascending within each group. That is the
/// contract's observable shape; the falsifying row for the comparator
/// itself is a review finding, not something this file supplies.
#[test]
fn interleaved_ties_come_back_in_ascending_index_order() {
    let near = |k: usize| Aabb {
        min_x: if k >= 100 { 1.0 } else { 3.0 },
        min_y: -1.0,
        min_z: -1.0,
        max_x: if k >= 100 { 2.0 } else { 4.0 },
        max_y: 1.0,
        max_z: 1.0,
    };
    let boxes: Vec<Aabb> = (0..200).map(near).collect();
    let tree = Bvh::build(&boxes);
    let out = tree.ray(&Ray {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    });
    assert_eq!(out.len(), 200, "every box is on the ray");
    let nearer: Vec<usize> = out.iter().take(100).map(|c| c.item).collect();
    let farther: Vec<usize> = out.iter().skip(100).map(|c| c.item).collect();
    assert_eq!(
        nearer,
        (100..200).collect::<Vec<_>>(),
        "the nearer tie group ascends by input index"
    );
    assert_eq!(
        farther,
        (0..100).collect::<Vec<_>>(),
        "the farther tie group ascends by input index"
    );
    assert!(
        out[0].t_enter.to_bits() == out[99].t_enter.to_bits()
            && out[100].t_enter.to_bits() == out[199].t_enter.to_bits(),
        "each group ties bit-exactly, so only the index can order it"
    );
}

/// Identical boxes tie bit-exactly and come back index-ascending.
#[test]
fn identical_boxes_come_back_in_ascending_index_order() {
    // 64 copies of one box: every t_enter ties exactly, so the ONLY
    // thing that can order the answer is the index tie-break.
    let boxes = vec![
        Aabb {
            min_x: 1.0,
            min_y: -1.0,
            min_z: -1.0,
            max_x: 2.0,
            max_y: 1.0,
            max_z: 1.0,
        };
        64
    ];
    let tree = Bvh::build(&boxes);
    let out = tree.ray(&Ray {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    });
    let items: Vec<usize> = out.iter().map(|c| c.item).collect();
    assert_eq!(items, (0..64).collect::<Vec<_>>(), "ties ascend by index");
    let first = out[0].t_enter;
    assert!(
        out.iter().all(|c| c.t_enter.to_bits() == first.to_bits()),
        "identical boxes tie bit-exactly"
    );
}

/// A ray lying exactly IN a box face plane, travelling along it, and
/// a ray along a box EDGE: both are closed-boundary hits, so the box
/// stays a candidate. (`Aabb::overlaps` is closed; the ray query
/// matches it.)
#[test]
fn rays_in_a_face_plane_and_along_an_edge_stay_candidates() {
    let unit = Aabb {
        min_x: 0.0,
        min_y: 0.0,
        min_z: 0.0,
        max_x: 1.0,
        max_y: 1.0,
        max_z: 1.0,
    };
    let tree = Bvh::build(&[unit]);
    // In the z = 1 face plane, travelling +x, entering at x = 0.
    let in_face = Ray {
        origin: Point3::new(-3.0, 0.5, 1.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    };
    let out = tree.ray(&in_face);
    assert_eq!(out.len(), 1, "a ray in a face plane still meets the box");
    assert!(
        out[0].t_enter <= 3.0,
        "t_enter {} is a lower bound",
        out[0].t_enter
    );
    // Along the edge x = 1, z = 1, travelling +y.
    let along_edge = Ray {
        origin: Point3::new(1.0, -3.0, 1.0),
        dir: Vec3::new(0.0, 1.0, 0.0),
    };
    let out = tree.ray(&along_edge);
    assert_eq!(out.len(), 1, "a ray along an edge still meets the box");
    assert!(
        out[0].t_enter <= 3.0,
        "t_enter {} is a lower bound",
        out[0].t_enter
    );
    // Grazing a single corner.
    let corner = Ray {
        origin: Point3::new(1.0, 1.0, -3.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    };
    assert_eq!(tree.ray(&corner).len(), 1, "a corner graze is a candidate");
}

/// The empty tree and an empty candidate answer are not an error.
///
/// EVIDENCE ROW, not a gate on new behaviour: it records that the
/// degenerate inputs a viewport will hand this query (nothing
/// displayed; a ray into empty space) answer with an empty vector
/// rather than a panic.
#[test]
fn empty_inputs_answer_empty() {
    let tree = Bvh::build(&[]);
    assert!(
        tree.ray(&Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 0.0, 0.0),
        })
        .is_empty()
    );
    let tree = Bvh::build(&[Aabb {
        min_x: 10.0,
        min_y: 10.0,
        min_z: 10.0,
        max_x: 11.0,
        max_y: 11.0,
        max_z: 11.0,
    }]);
    assert!(
        tree.ray(&Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(-1.0, 0.0, 0.0),
        })
        .is_empty()
    );
}

/// **The conservative-superset contract's undocumented precondition.**
///
/// `Ray::slab_enter` computes `(bound − origin) · (1/d)` in `f64`.
/// The doc-comment's corner analysis covers NaN, zero directions,
/// zero extents and rounding, but not IEEE **overflow**: when
/// `bound − origin` overflows to `±∞` while the exact quotient is
/// finite, the axis reports `near = far = ±∞` and the fold prunes a
/// box the ray truly meets.
///
/// The witness below is exact and reproducible: with
/// `o = (1.7e308, 0, 0)`, `d = (−1e300, 1, 0)` and the box
/// `[−1.7e308, −1.6e308] × [0, 1e9] × [−1, 1]`, the ray is inside the
/// box at `t = 3.35e8` (`x = −1.65e308`, `y = 3.35e8`, `z = 0`), yet
/// the query returns nothing.
///
/// IGNORED, not red: this is a review finding, not an agreed contract
/// change. Un-ignore it if the fix pass decides the contract holds at
/// all finite magnitudes; delete it if the fix pass instead scopes the
/// doc-comment to inputs where `bound − origin` and `1/d` stay in the
/// normal range.
#[test]
#[ignore = "R2 review finding: overflow in (bound - origin) prunes a true intersection"]
fn overflow_in_the_slab_subtraction_drops_a_true_intersection() {
    let b = Aabb {
        min_x: -1.7e308,
        min_y: 0.0,
        min_z: -1.0,
        max_x: -1.6e308,
        max_y: 1e9,
        max_z: 1.0,
    };
    let tree = Bvh::build(&[b]);
    let r = Ray {
        origin: Point3::new(1.7e308, 0.0, 0.0),
        dir: Vec3::new(-1e300, 1.0, 0.0),
    };
    // The witness point, evaluated without overflowing the product.
    let t = 3.35e8f64;
    let half = -1e300 * (t * 0.5);
    let x = (1.7e308 + half) + half;
    assert!(
        x >= b.min_x && x <= b.max_x,
        "x({t}) = {x} is inside the box"
    );
    assert!(t >= b.min_y && t <= b.max_y, "y({t}) is inside the box");
    assert_eq!(
        tree.ray(&r).len(),
        1,
        "a truly met box must stay a candidate"
    );
}
