//! **Differential lane: `KnotVector`'s knot-vector queries against the
//! hand-written scans they replaced.**
//!
//! Two substitutions land together, and neither is a compile error when
//! it is wrong — an off-by-one here is a wrong curve.
//!
//! 1. [`KnotVector::interior_knots`] replaces six hand-written scans of
//!    "the distinct interior knots", in three result shapes (pairs, a
//!    value list, a max-multiplicity predicate) and four spellings. All
//!    four spellings are reproduced below **verbatim as oracles** and
//!    asserted identical to the method, so the claim "one home, same
//!    answers" is measured rather than argued.
//!
//! 2. The raw knot-algebra path's span derivation, previously a linear
//!    "last index `i` with `knots[i] ≤ u`" scan, now calls the same
//!    binary search [`KnotVector::find_span`] uses. Those two functions
//!    **are not the same function**: they agree on `t ∈ [lo, hi)` and
//!    disagree everywhere else, so the sweep asserts agreement inside
//!    the half-open domain and asserts the *disagreement* outside it.
//!    A future edit that "fixed" `find_span` to match a linear scan at
//!    the domain end would fail here, which is the point.
//!
//! Merged into few tests on purpose (nextest is process-per-test, so a
//! shared generator would otherwise be rebuilt per row); every assertion
//! is labelled so the failing property reads off the message.

use geom_core::spline::KnotVector;
use test_utils::fuzz;

// ---------------------------------------------------------------------
// The retired scans, verbatim — the oracles
// ---------------------------------------------------------------------

/// `compose.rs`'s `interior_values` and `algebra.rs`'s inlined copy:
/// while-idx over the interior slice with an inner run counter.
fn retired_while_idx(kv: &KnotVector) -> Vec<(f64, usize)> {
    let p = kv.degree();
    let knots = kv.knots();
    let mut out = Vec::new();
    let mut idx = p + 1;
    while idx < knots.len() - p - 1 {
        let v = knots[idx];
        let mut m = 1;
        while idx + m < knots.len() - p - 1 && knots[idx + m] == v {
            m += 1;
        }
        out.push((v, m));
        idx += m;
    }
    out
}

/// `geom/curves/fit.rs`'s `distinct_interior`: values only, `out.last()`
/// dedup.
fn retired_last_dedup(kv: &KnotVector) -> Vec<f64> {
    let p = kv.degree();
    let knots = kv.knots();
    let mut out: Vec<f64> = Vec::new();
    for u in &knots[p + 1..knots.len() - p - 1] {
        if out.last() != Some(u) {
            out.push(*u);
        }
    }
    out
}

/// `sweep/skin.rs`'s `interior_multiplicities`: `match out.last_mut()`.
fn retired_last_mut(kv: &KnotVector) -> Vec<(f64, usize)> {
    let p = kv.degree();
    let knots = kv.knots();
    let mut out: Vec<(f64, usize)> = Vec::new();
    for u in &knots[p + 1..knots.len() - p - 1] {
        match out.last_mut() {
            Some((v, m)) if *v == *u => *m += 1,
            _ => out.push((*u, 1)),
        }
    }
    out
}

/// `mesh/chords.rs`'s and `mesh/nurbs_cert.rs`'s C¹ gate: while-i /
/// while-j over the interior slice, testing max multiplicity > p − 1.
fn retired_c1_gate(kv: &KnotVector) -> bool {
    let p = kv.degree();
    let interior = &kv.knots()[p + 1..kv.knots().len() - p - 1];
    let mut i = 0;
    while i < interior.len() {
        let mut j = i + 1;
        while j < interior.len() && interior[j] == interior[i] {
            j += 1;
        }
        if j - i > p - 1 {
            return true;
        }
        i = j;
    }
    false
}

/// `compose.rs`'s retired span derivation: the last index with
/// `knots[i] ≤ u`, `k` initialised to 0 and left there when nothing
/// matches.
fn retired_linear_span(knots: &[f64], u: f64) -> usize {
    let mut k = 0;
    for (i, kn) in knots.iter().enumerate() {
        if *kn <= u {
            k = i;
        }
    }
    k
}

// ---------------------------------------------------------------------
// Cases
// ---------------------------------------------------------------------

fn build(degree: usize, interior: &[(f64, usize)]) -> KnotVector {
    let mut knots = vec![0.0; degree + 1];
    for (v, m) in interior {
        knots.extend(core::iter::repeat_n(*v, *m));
    }
    knots.extend(core::iter::repeat_n(1.0, degree + 1));
    #[allow(clippy::expect_used)]
    KnotVector::clamped(knots, degree).expect("fixture is a valid clamped vector")
}

/// The written-down cases: the shapes a random draw is not guaranteed to
/// produce, so they are constructed rather than hunted (test-suite-cost
/// shape 2). Every degree from 1 to 5 appears with an empty interior, a
/// multiplicity-1 interior, and a multiplicity-`p` interior.
fn enumerated() -> Vec<(String, KnotVector)> {
    let mut out = Vec::new();
    for p in 1..=5usize {
        out.push((format!("p{p}/no interior knots"), build(p, &[])));
        out.push((format!("p{p}/one interior, mult 1"), build(p, &[(0.5, 1)])));
        out.push((format!("p{p}/one interior, mult p"), build(p, &[(0.5, p)])));
        out.push((
            format!("p{p}/two interiors, mult 1 then p"),
            build(p, &[(0.25, 1), (0.75, p)]),
        ));
        out.push((
            format!("p{p}/two interiors, mult p then 1"),
            build(p, &[(0.25, p), (0.75, 1)]),
        ));
        if p >= 2 {
            out.push((
                format!("p{p}/adjacent runs, mult p−1 each"),
                build(p, &[(0.25, p - 1), (0.5, p - 1), (0.75, p - 1)]),
            ));
        }
        // Values one ULP apart: distinct by exact f64 identity, which is
        // the rule the scan and the method both claim to use.
        let a = 0.5f64;
        let b = f64::from_bits(a.to_bits() + 1);
        out.push((
            format!("p{p}/two interiors one ULP apart"),
            build(p, &[(a, 1), (b, 1)]),
        ));
        // Many single knots: the run-detection loop's long case.
        let many: Vec<(f64, usize)> = (1..=9).map(|i| (f64::from(i) / 10.0, 1)).collect();
        out.push((format!("p{p}/nine single interiors"), build(p, &many)));
    }
    out
}

/// A valid clamped vector with a random degree, a random number of
/// distinct interior values on a coarse grid (so exact-equality runs
/// actually occur), and a random multiplicity in `1..=p` for each.
fn drawn(rng: &mut fuzz::Rng) -> KnotVector {
    let p = 1 + rng.below(5);
    let count = rng.below(7);
    let mut values: Vec<f64> = Vec::new();
    for _ in 0..count {
        // 63 interior grid points: collisions are common, which is what
        // exercises multi-copy runs built from independent draws.
        let v = f64::from(1 + rng.below(63) as u32) / 64.0;
        if !values.contains(&v) {
            values.push(v);
        }
    }
    values.sort_by(f64::total_cmp);
    let interior: Vec<(f64, usize)> = values.into_iter().map(|v| (v, 1 + rng.below(p))).collect();
    build(p, &interior)
}

/// Parameters worth locating in a vector: every knot value, every
/// midpoint between consecutive distinct knots, both domain ends, points
/// outside the domain, and NaN.
fn probes(kv: &KnotVector) -> Vec<f64> {
    let (lo, hi) = kv.domain();
    let mut out = vec![lo, hi, lo - 1.0, hi + 1.0, f64::NAN, -0.0, f64::INFINITY];
    out.extend(kv.knots().iter().copied());
    let mut distinct: Vec<f64> = kv.knots().to_vec();
    distinct.dedup();
    for w in distinct.windows(2) {
        out.push(0.5 * (w[0] + w[1]));
    }
    out
}

fn check_one(name: &str, kv: &KnotVector) {
    let p = kv.degree();
    let mine: Vec<(f64, usize)> = kv.interior_knots().collect();

    assert_eq!(
        mine,
        retired_while_idx(kv),
        "{name}: interior_knots disagrees with the retired while-idx scan"
    );
    assert_eq!(
        mine,
        retired_last_mut(kv),
        "{name}: interior_knots disagrees with the retired last_mut scan"
    );
    let values: Vec<f64> = kv.interior_knots().map(|(v, _)| v).collect();
    assert_eq!(
        values,
        retired_last_dedup(kv),
        "{name}: the value-only shape disagrees with the retired dedup scan"
    );
    if p >= 2 {
        assert_eq!(
            kv.interior_knots().any(|(_, m)| m > p - 1),
            retired_c1_gate(kv),
            "{name}: the C¹ predicate disagrees with the retired while-i/while-j gate"
        );
    }

    // The two invariants the method's docs promise its consumers.
    let (lo, hi) = kv.domain();
    for (v, m) in &mine {
        assert!(
            *v > lo && *v < hi,
            "{name}: interior_knots yielded {v}, not strictly inside [{lo}, {hi}]"
        );
        assert!(
            *m >= 1 && *m <= p,
            "{name}: interior_knots yielded multiplicity {m}, outside 1..={p}"
        );
    }
    // Ascending and distinct.
    for w in mine.windows(2) {
        assert!(
            w[0].0 < w[1].0,
            "{name}: interior_knots is not strictly ascending at {} / {}",
            w[0].0,
            w[1].0
        );
    }
    // Agreement with the accessor that needed the value first — the one
    // the type used to offer instead of this.
    for (v, m) in &mine {
        assert_eq!(
            kv.multiplicity_of(*v).map(|(s, _)| s),
            Some(*m),
            "{name}: multiplicity_of({v}) disagrees with interior_knots"
        );
    }
}

/// The span substitution, on one vector: agreement inside the half-open
/// domain, and the DOCUMENTED disagreement outside it.
fn check_span(name: &str, kv: &KnotVector) {
    let knots = kv.knots();
    let (lo, hi) = kv.domain();
    let (first, last) = (kv.first_span(), kv.last_span());

    for t in probes(kv) {
        let binary = kv.find_span(t);
        let linear = retired_linear_span(knots, t);
        if t >= lo && t < hi {
            assert_eq!(
                binary, linear,
                "{name}: find_span({t}) = {binary} but the retired linear scan says {linear} \
                 — inside [lo, hi) they must agree, which is what makes the substitution sound"
            );
        } else if t >= hi {
            // The precondition's whole content: past the domain end the
            // linear scan walks into the trailing clamp.
            assert_eq!(
                binary, last,
                "{name}: find_span({t}) at/above the domain end must be the last span"
            );
            assert_eq!(
                linear,
                knots.len() - 1,
                "{name}: the retired scan at/above the domain end must be len − 1"
            );
            assert_ne!(
                binary, linear,
                "{name}: at/above the domain end the two must DIFFER — if they ever agree, \
                 the substitution's stated precondition has become false"
            );
        } else {
            // Below the domain, and NaN: the linear scan returns its
            // initialiser, not a located span.
            assert_eq!(
                binary, first,
                "{name}: find_span({t}) below the domain (or NaN) must be the first span"
            );
            assert_eq!(
                linear, 0,
                "{name}: the retired scan below the domain (or NaN) returns its initialiser"
            );
        }
    }
}

#[test]
fn interior_knots_and_the_span_search_match_the_scans_they_replaced() {
    for (name, kv) in enumerated() {
        check_one(&name, &kv);
        check_span(&name, &kv);
    }

    let mut rng = fuzz::start("knot-queries-differential");
    for i in 0..fuzz::scaled(400) {
        let kv = drawn(&mut rng);
        let name = format!("drawn#{i} (p{} len{})", kv.degree(), kv.knots().len());
        check_one(&name, &kv);
        check_span(&name, &kv);
    }
}
