//! Review probes for EDIT-PICK3 (lane `pick3-r1`, protocol v6 dual).
//!
//! Each row here falsifies, or fails to falsify, one claim of the unit
//! `pick-door-answers-a-t-interval` by execution.
//!
//! Two of this lane's rows have moved, because they pin the door and
//! belong beside it: the early-out fixture is
//! `pick3_early_out::the_early_out_keeps_a_candidate_whose_interval_reaches_below_its_box`
//! (re-expressed through `pick_face`, not through a hand-run
//! traversal), and the clamp's retraction is
//! `pick::tests::the_clamp_is_a_retraction_onto_the_simplex_and_not_the_nearest_point`
//! (calling the real `retract_to_simplex`).//!
//! A third row is gone with the lane's Python side-channel: it printed
//! admitted spans as bit patterns for `scripts/review_pick3_r1_enclosure.py`
//! to recheck in exact rationals, and was a no-op unless `PICK3_R1_DUMP`
//! was set — so nothing in CI ran either half. The claim they made
//! together is pinned in-tree and EXECUTED by
//! `review_pick3_r2_probes::the_t_interval_encloses_the_exact_crossing_and_the_clamped_point`,
//! which does the same exact-rational recomputation in Rust over
//! stressed families, and by
//! `the_interval_encloses_an_exactly_known_crossing` below.

use bvh::Ray;
use editor_core::resolve::ray_triangle;
use geom_core::{Point3, Vec3};

/// A deterministic 64-bit xorshift, so the search is replayable.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    /// A uniform integer in `[-half, half]`.
    fn int(&mut self, half: i64) -> f64 {
        (self.next() % (2 * half as u64 + 1)) as i64 as f64 - half as f64
    }
    fn vec(&mut self, half: i64) -> Vec3<f64> {
        Vec3::new(self.int(half), self.int(half), self.int(half))
    }
}

/// **`t_span`'s interval is an enclosure of the EXACT crossing's
/// parameter** — searched over crossings whose exact answer is known
/// by construction.
///
/// Every input is a dyadic rational small enough that `a + u*e1 +
/// v*e2` and `hit - t*d` are computed without a rounding, so the ray
/// `(o, d)` meets the closed triangle at exactly the barycentrics
/// `(u, v)` and exactly the parameter `t` that built it. The claim
/// under test is `pick.rs`'s: "`[t_lo, t_hi]` encloses the parameter
/// of the true crossing whenever that crossing is a point of the
/// closed triangle, taking the corners and the ray as exact". The
/// direction `d` is drawn as `alpha*e1 + beta*e2 + w` so a large
/// share of the draws are near-tangent, which is where the bound is
/// load-bearing; `spread` puts the triangle far from the origin
/// against a small triangle.
#[test]
fn the_interval_encloses_an_exactly_known_crossing() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut admitted = 0u64;
    let mut worst = 0.0f64;
    let mut worst_tight = 0.0f64;
    let mut escapes: Vec<String> = Vec::new();
    for case in 0..400_000u64 {
        let spread = if case % 3 == 0 { 1 << 18 } else { 8 };
        let a = Point3::new(0.0, 0.0, 0.0) + rng.vec(spread);
        let e1 = rng.vec(64);
        let e2 = rng.vec(64);
        let alpha = rng.int(32);
        let beta = rng.int(32);
        let d = e1 * alpha + e2 * beta + rng.vec(2);
        // Barycentrics on a 1/1024 lattice with u + v <= 1, and a
        // parameter on the same lattice: every product below is exact.
        let u = (rng.next() % 1025) as f64 / 1024.0;
        let v = (rng.next() % (1025 - (u * 1024.0) as u64)) as f64 / 1024.0;
        let t = ((rng.next() % 1024) as f64 + 1.0) / 1024.0 * 8.0;
        let tri = [a, a + e1, a + e2];
        let hit = a + e1 * u + e2 * v;
        let origin = hit - d * t;
        // The construction is only a witness if it was exact.
        let back = origin + d * t - hit;
        if back.x != 0.0 || back.y != 0.0 || back.z != 0.0 {
            continue;
        }
        let ray = Ray { origin, dir: d };
        let Some(span) = ray_triangle(&ray, &tri) else {
            continue;
        };
        admitted += 1;
        let half = 0.5 * span.width();
        if half > 0.0 {
            worst_tight = worst_tight.max((t - span.t).abs() / half);
        }
        if !(span.t_lo <= t && t <= span.t_hi) {
            let miss = (t - span.t_hi).max(span.t_lo - t);
            worst = worst.max(miss / span.width().max(f64::MIN_POSITIVE));
            if escapes.len() < 8 {
                escapes.push(format!(
                    "case {case}: exact t = {t}, span {span:?}, miss {miss:e}, \
                     tri {tri:?}, ray o {origin:?} d {d:?}, u {u} v {v}"
                ));
            }
        }
    }
    eprintln!(
        "admitted {admitted} exactly-known crossings; worst escape/width {worst:e}; \
         the exact t reached {worst_tight:.6} of the certified half-width at its furthest"
    );
    assert!(
        escapes.is_empty(),
        "the certified interval missed the exact crossing on {} draws:\n{}",
        escapes.len(),
        escapes.join("\n")
    );
    assert!(admitted > 10_000, "the search admitted too few: {admitted}");
}
