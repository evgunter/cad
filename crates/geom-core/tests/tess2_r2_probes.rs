//! TESS-2 reviewer (r2) probes on the ring applier.

use geom_core::ring_interval::RingInterval;
use geom_core::spline::KnotVector;
use geom_core::spline::algebra::refine_plan_homogeneous;

fn chain(kv: &KnotVector, add: &[f64], coeffs: &[f64]) -> Vec<RingInterval> {
    let plans = refine_plan_homogeneous(kv, add).unwrap();
    let mut out: Vec<RingInterval> = coeffs.iter().copied().map(RingInterval::point).collect();
    for p in &plans {
        out = p.apply_ring(&out);
    }
    out
}

/// The row `the_ring_applier_stays_in_step_and_inside_the_hull` claims
/// "every slot lies inside the hull of the whole input ... the convex
/// form cannot [bulge]". With EQUAL adjacent coefficients the two
/// outward-rounded ratios sum above 1, so the slot leaves a degenerate
/// hull. Not a soundness defect (the enclosure still contains the truth).
#[test]
fn r2_constant_coefficients_leave_the_hull() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let add: Vec<f64> = (1..3).map(|k| k as f64 / 3.0).collect();
    for c in [1.0f64, 0.5, -3.0] {
        let out = chain(&kv, &add, &[c; 3]);
        let mut left = 0;
        for (i, r) in out.iter().enumerate() {
            let inside = r.lo() >= c.min(c) && r.hi() <= c;
            if !inside {
                left += 1;
                println!("c={c}: slot {i} = [{:.17e}, {:.17e}] leaves hull [{c}, {c}]", r.lo(), r.hi());
            }
        }
        println!("c={c}: {left} of {} slots left the degenerate hull", out.len());
    }
}

/// A knot one ulp from an existing one (`refine-dir-hairline`): the
/// ring ratios stay finite and the chain stays sound-shaped (no poison);
/// print the widths.
#[test]
fn r2_hairline_insertion_is_total() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let u = 0.5f64.next_up();
    let out = chain(&kv, &[u, 0.25], &[0.0, 1.0, -2.0, 3.0]);
    for (i, r) in out.iter().enumerate() {
        println!("slot {i}: poison={} [{:.17e}, {:.17e}] width {:e}", r.is_poison(), r.lo(), r.hi(), r.width());
        assert!(!r.is_poison());
    }
    // Repeated insertion at an EXISTING interior knot up to the C1 budget.
    let out2 = chain(&kv, &[0.5], &[0.0, 1.0, -2.0, 3.0]);
    println!("repeat at 0.5 (mult 1 -> 2 = p): {} slots, poison={}", out2.len(), out2.iter().any(|r| r.is_poison()));
    let over = refine_plan_homogeneous(&kv, &[0.5, 0.5]);
    println!("repeat at 0.5 twice (over the degree budget): {:?}", over.map(|p| p.len()));
}
