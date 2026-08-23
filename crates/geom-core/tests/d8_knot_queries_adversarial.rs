//! **Adversarial lane for D8's two knot-vector substitutions.**
//!
//! Written as an independent falsification attempt on
//! [`KnotVector::interior_knots`] and the extracted span search, and
//! deliberately *not* built from the retired scans: the oracles here are
//! re-derived from the definitions rather than copied from the code that
//! was deleted, so an error shared between a retired scan and its
//! replacement cannot hide in both sides at once.
//!
//! What this file adds over `knot_queries_differential.rs`, which builds
//! every fixture on `[0, 1]` with interior values on a 1/64 grid at
//! degrees 1–5:
//!
//! - **Domains that are not `[0, 1]`** — negative, high-magnitude,
//!   subnormal, and one whose width is a few ULP of its endpoints.
//! - **`-0.0` as an actual knot value**, including a run that mixes
//!   `-0.0` and `+0.0`: they are one run under `==` (the identity rule
//!   both the method and every retired scan claim) but two values under
//!   `total_cmp` (the ordering `compose.rs` sorts breaks with), which is
//!   the one place those two rules can disagree.
//! - **Degrees above 5**, up to a degree whose clamps dominate the
//!   vector, and the smallest-slack case `degree = 1`.
//! - **A pair-count oracle of a different shape**: for each run-leading
//!   index, count *every* interior entry equal to it, rather than
//!   walking the run. It disagrees with a run-length scan exactly when
//!   equal values are non-adjacent, which a validated vector forbids —
//!   so agreement is a check on the slice bounds, not a restatement.
//! - **The span search from its definition** (`knots[i] ≤ t <
//!   knots[i+1]`, located by exhaustive scan over `first_span
//!   ..= last_span`) rather than from the retired linear scan, plus the
//!   three documented totality exits.
//! - **The mutation sequence itself**, reconstructed from
//!   `to_bezier_spans_extra`'s loop, with the raw-slice entitlement
//!   (`clamped` still accepts the list) and the `unreachable!` guard's
//!   condition both checked *before every insertion* — including with
//!   injected extras, at multiplicity saturation, and at degree 1.
//! - **The public door**, driven with extras chosen to reach the
//!   `unreachable!` if anything can: NaN, ±∞, both domain ends, `-0.0`,
//!   values one ULP inside each end, and duplicates of existing knots.
//!
//! Merged into few tests on purpose (nextest is process-per-test);
//! every assertion carries the fixture name.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::ring_interval::RingInterval;
use geom_core::spline::KnotVector;
use geom_core::spline::compose::CurveRingData;
use geom_core::spline::compose::tensor::{SurfaceRingData, surface_curve_residual};
use test_utils::fuzz;

// ---------------------------------------------------------------------
// Oracles, re-derived rather than copied
// ---------------------------------------------------------------------

/// The break parameters `surface_curve_residual` is contracted to
/// produce: the shared domain ends, the interior knots of **both**
/// curves (the entry point merges them so every channel lands on one
/// span structure), and every caller-supplied extra STRICTLY inside the
/// domain. Deduplicated on exact `f64`, ascending. Domain ends, signed
/// zeros, non-finites, out-of-domain values and repeats of a value
/// already present contribute nothing.
fn admitted_breaks(pcurve: &KnotVector, carrier: &KnotVector, extra: &[f64]) -> Vec<f64> {
    let (lo, hi) = pcurve.domain();
    let mut b: Vec<f64> = vec![lo, hi];
    b.extend(pcurve.interior_knots().map(|(v, _)| v));
    b.extend(carrier.interior_knots().map(|(v, _)| v));
    b.extend(extra.iter().copied().filter(|&v| v > lo && v < hi));
    b.sort_by(f64::total_cmp);
    b.dedup();
    b
}

/// The distinct interior knots with multiplicities, by **counting every
/// equal interior entry** for each run-leading index. Shape-independent
/// of a run-length walk: it would disagree with one if equal values were
/// ever non-adjacent, so agreement checks the interior bounds rather
/// than restating the loop.
fn oracle_pairs(kv: &KnotVector) -> Vec<(f64, usize)> {
    let p = kv.degree();
    let k = kv.knots();
    let (first, past) = (p + 1, k.len() - p - 1);
    let mut out = Vec::new();
    for i in first..past {
        // Skip anything that is not the first entry of its run.
        if i > first && k[i] == k[i - 1] {
            continue;
        }
        let m = (first..past).filter(|&j| k[j] == k[i]).count();
        out.push((k[i], m));
    }
    out
}

/// The span index **from the definition**: the unique `i` in
/// `[first_span, last_span]` with `knots[i] ≤ t < knots[i+1]`, found by
/// exhaustive scan; `None` when `t` is outside `[lo, hi)` or NaN, where
/// no such `i` exists and the function's totality contract takes over.
fn oracle_span_by_definition(kv: &KnotVector, t: f64) -> Option<usize> {
    let k = kv.knots();
    let mut found = None;
    for i in kv.first_span()..=kv.last_span() {
        if k[i] <= t && t < k[i + 1] {
            assert!(
                found.is_none(),
                "two spans contain {t} — the fixture is not a valid vector"
            );
            found = Some(i);
        }
    }
    found
}

/// `compose.rs`'s retired linear span scan — kept only to state the
/// substitution claim, never as the definition of the answer.
fn retired_linear_span(knots: &[f64], u: f64) -> usize {
    let mut k = 0;
    for (i, kn) in knots.iter().enumerate() {
        if *kn <= u {
            k = i;
        }
    }
    k
}

/// `geom-brep/src/props/quad.rs`'s `raw_span`, reproduced: the other
/// raw-slice span search live in the tree. Evidence for a census note,
/// not a gate.
fn quad_raw_span(knots: &[f64], degree: usize, count: usize, t: f64) -> usize {
    let last = count.saturating_sub(1).max(degree);
    let mut span = degree;
    let mut i = degree;
    while i <= last && i + 1 < knots.len() {
        if knots[i] <= t && knots[i] < knots[i + 1] {
            span = i;
        }
        i += 1;
    }
    span.min(last)
}

// ---------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------

/// A clamped vector on `[lo, hi]` with the given interior runs, in the
/// order supplied (so a caller can build a `-0.0` / `+0.0` run that a
/// sorted draw would never produce).
fn build(degree: usize, lo: f64, hi: f64, interior: &[(f64, usize)]) -> Option<KnotVector> {
    let mut knots = vec![lo; degree + 1];
    for (v, m) in interior {
        knots.extend(core::iter::repeat_n(*v, *m));
    }
    knots.extend(core::iter::repeat_n(hi, degree + 1));
    KnotVector::clamped(knots, degree).ok()
}

/// One ULP above `x` (toward `+∞`), for finite positive-or-zero `x`.
fn ulp_up(x: f64) -> f64 {
    f64::from_bits(x.to_bits() + 1)
}

/// The domains this lane sweeps: the differential suite's `[0, 1]` plus
/// the magnitude regimes it never leaves.
fn domains() -> Vec<(&'static str, f64, f64)> {
    vec![
        ("unit", 0.0, 1.0),
        ("negative", -3.5, -0.25),
        ("straddling zero", -1.0, 1.0),
        ("high magnitude", 1.0e300, 2.0e300),
        ("tiny width at high magnitude", 1.0e6, 1.0e6 + 1.0e-9),
        ("subnormal", 1.0e-320, 9.0e-320),
        ("min positive", f64::MIN_POSITIVE, 4.0 * f64::MIN_POSITIVE),
    ]
}

/// Interior values inside `(lo, hi)` at the given count, spread by
/// bit-space interpolation so subnormal and high-magnitude domains get
/// values that are actually distinct there (a linear split of
/// `[1e-320, 9e-320]` collapses; a linear split of `[1e300, 2e300]` is
/// fine, and both are covered by walking the domain in equal steps of
/// the representable range where a linear step underflows).
fn interior_values(lo: f64, hi: f64, count: usize) -> Vec<f64> {
    let mut out = Vec::new();
    for j in 1..=count {
        #[allow(clippy::cast_precision_loss)]
        let f = j as f64 / (count + 1) as f64;
        let linear = lo + (hi - lo) * f;
        // Where a linear split does not separate (subnormals, or a
        // domain whose width is a few ULP), step in bit space instead.
        let v = if linear > lo && linear < hi {
            linear
        } else {
            let (a, b) = (lo.to_bits(), hi.to_bits());
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let step = ((b - a) as f64 * f) as u64;
            f64::from_bits(a + step.max(1))
        };
        if v > lo && v < hi && !out.contains(&v) {
            out.push(v);
        }
    }
    out
}

/// The written-down cases: every shape a draw is not guaranteed to
/// produce (test-suite-cost shape 2 — construct, do not hunt).
fn enumerated() -> Vec<(String, KnotVector)> {
    let mut out = Vec::new();
    let mut push = |name: String, kv: Option<KnotVector>| {
        if let Some(kv) = kv {
            out.push((name, kv));
        }
    };

    for (dn, lo, hi) in domains() {
        // Degree 1 is where `len ≥ 2(degree + 1)` has the least slack;
        // 6, 9 and 17 are above everything the differential suite draws.
        for p in [1usize, 2, 3, 6, 9, 17] {
            let vals = interior_values(lo, hi, 3);
            if vals.len() < 3 {
                continue;
            }
            let (a, b, c) = (vals[0], vals[1], vals[2]);
            push(format!("{dn}/p{p}/empty interior"), build(p, lo, hi, &[]));
            push(
                format!("{dn}/p{p}/single mult 1"),
                build(p, lo, hi, &[(b, 1)]),
            );
            push(
                format!("{dn}/p{p}/single mult p"),
                build(p, lo, hi, &[(b, p)]),
            );
            if p >= 2 {
                push(
                    format!("{dn}/p{p}/single mult p−1"),
                    build(p, lo, hi, &[(b, p - 1)]),
                );
                push(
                    format!("{dn}/p{p}/runs p−1, p, p−1"),
                    build(p, lo, hi, &[(a, p - 1), (b, p), (c, p - 1)]),
                );
            }
            push(
                format!("{dn}/p{p}/three singles"),
                build(p, lo, hi, &[(a, 1), (b, 1), (c, 1)]),
            );
            // One ULP apart: distinct under the exact-identity rule, and
            // the tightest case the run detection can face.
            let u1 = ulp_up(b);
            let u2 = ulp_up(u1);
            if u2 < hi {
                push(
                    format!("{dn}/p{p}/two interiors one ULP apart"),
                    build(p, lo, hi, &[(b, 1), (u1, 1)]),
                );
                push(
                    format!("{dn}/p{p}/three consecutive ULPs, mult 1"),
                    build(p, lo, hi, &[(b, 1), (u1, 1), (u2, 1)]),
                );
            }
            // An interior value one ULP inside each clamp — the closest
            // an interior knot can get to the domain end while the
            // "strictly inside" promise still has to hold.
            let just_above_lo = ulp_up(lo);
            if just_above_lo < hi {
                push(
                    format!("{dn}/p{p}/interior one ULP above lo"),
                    build(p, lo, hi, &[(just_above_lo, 1)]),
                );
            }
        }
    }

    // `-0.0` as a knot value, on a domain that admits it. `-0.0 == 0.0`,
    // so a `-0.0` run followed by a `+0.0` run is ONE run under the
    // identity rule the method and every retired scan use — and two
    // values under `total_cmp`. The vector is valid: the ordering check
    // is `<`, and `0.0 < -0.0` is false.
    for p in [1usize, 2, 3, 5] {
        push(
            format!("signed zero/p{p}/interior −0.0 alone"),
            build(p, -1.0, 1.0, &[(-0.0, 1)]),
        );
        if p >= 2 {
            push(
                format!("signed zero/p{p}/−0.0 then +0.0, one run"),
                build(p, -1.0, 1.0, &[(-0.0, 1), (0.0, 1)]),
            );
            push(
                format!("signed zero/p{p}/+0.0 then −0.0, one run"),
                build(p, -1.0, 1.0, &[(0.0, 1), (-0.0, 1)]),
            );
        }
        push(
            format!("signed zero/p{p}/interior −0.0 at full multiplicity"),
            build(p, -1.0, 1.0, &[(-0.0, p)]),
        );
    }

    // A domain END that is `-0.0`. `clamped` accepts it (the end run is
    // exact under `==`, and every interior value differs from `-0.0`),
    // and it is the sharpest form of the `u == lo` / `u == hi` case the
    // span substitution rests on: the probe `+0.0` compares `==` to the
    // endpoint but is a different bit pattern, and `find_span`'s guard
    // is spelled `!(t > knots[p])`.
    push(
        "signed zero/p3/lo is −0.0".to_string(),
        build(3, -0.0, 1.0, &[(0.5, 2)]),
    );
    push(
        "signed zero/p3/hi is −0.0".to_string(),
        build(3, -1.0, -0.0, &[(-0.5, 2)]),
    );
    push(
        "signed zero/p1/lo is −0.0, least slack".to_string(),
        build(1, -0.0, 1.0, &[(0.5, 1)]),
    );

    // Hand-built extremes the generator above cannot express.
    //
    // `[-0.0, +0.0, -0.0]` is a NON-DECREASING sequence (`0.0 < -0.0`
    // is false, so the ordering check passes) whose three entries are
    // all `==` each other: one run of three under the identity rule,
    // and three distinct values under `total_cmp`. It is the sharpest
    // case for "which representative does the run report".
    push(
        "signed zero/p3/−0.0, +0.0, −0.0 — one run, three total_cmp values".to_string(),
        build(3, -1.0, 1.0, &[(-0.0, 1), (0.0, 1), (-0.0, 1)]),
    );
    // The widest domain the type admits, and a degree whose clamps
    // dominate the vector.
    push(
        "extreme/p2/full f64 span".to_string(),
        build(
            2,
            -f64::MAX,
            f64::MAX,
            &[(-1.0e300, 1), (0.0, 2), (1.0e300, 1)],
        ),
    );
    push(
        "extreme/p1/full f64 span, least slack".to_string(),
        build(1, -f64::MAX, f64::MAX, &[(0.0, 1)]),
    );
    push(
        "extreme/p64/clamps dominate".to_string(),
        build(64, 0.0, 1.0, &[(0.25, 64), (0.5, 1), (0.75, 63)]),
    );
    push(
        "extreme/p64/empty interior".to_string(),
        build(64, 0.0, 1.0, &[]),
    );
    // Subnormal knots, written out rather than interpolated.
    push(
        "extreme/p3/subnormal interior".to_string(),
        build(
            3,
            0.0,
            f64::MIN_POSITIVE,
            &[
                (f64::from_bits(1), 2),
                (f64::from_bits(2), 1),
                (f64::from_bits(3), 3),
            ],
        ),
    );
    out
}

/// A drawn valid vector: random degree, random domain regime, random
/// interior structure on a grid coarse enough that equal values arise
/// from independent draws.
fn drawn(rng: &mut fuzz::Rng) -> KnotVector {
    let doms = domains();
    loop {
        let p = 1 + rng.below(9);
        let (_, lo, hi) = doms[rng.below(doms.len())];
        let count = rng.below(6);
        let grid = interior_values(lo, hi, 31);
        let mut values: Vec<f64> = Vec::new();
        for _ in 0..count {
            if grid.is_empty() {
                break;
            }
            let v = grid[rng.below(grid.len())];
            if !values.contains(&v) {
                values.push(v);
            }
        }
        values.sort_by(f64::total_cmp);
        let interior: Vec<(f64, usize)> =
            values.into_iter().map(|v| (v, 1 + rng.below(p))).collect();
        if let Some(kv) = build(p, lo, hi, &interior) {
            return kv;
        }
    }
}

/// Parameters worth locating: every knot, every midpoint and every ULP
/// neighbour of a knot, both domain ends, outside both ends, the signed
/// zeros, the infinities and NaN.
fn probes(kv: &KnotVector) -> Vec<f64> {
    let (lo, hi) = kv.domain();
    let mut out = vec![
        lo,
        hi,
        f64::NAN,
        -0.0,
        0.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MIN,
        f64::MAX,
    ];
    for k in kv.knots() {
        out.push(*k);
        out.push(f64::from_bits(k.to_bits().wrapping_add(1)));
        out.push(f64::from_bits(k.to_bits().wrapping_sub(1)));
    }
    let mut distinct: Vec<f64> = kv.knots().to_vec();
    distinct.dedup();
    for w in distinct.windows(2) {
        let mid = 0.5 * w[0] + 0.5 * w[1];
        out.push(mid);
    }
    out
}

// ---------------------------------------------------------------------
// C1 / C4: the query
// ---------------------------------------------------------------------

fn check_query(name: &str, kv: &KnotVector) {
    let p = kv.degree();
    let mine: Vec<(f64, usize)> = kv.interior_knots().collect();
    let oracle = oracle_pairs(kv);

    // Bitwise, not `==`: `-0.0` and `+0.0` compare equal, so an
    // `assert_eq!` on `f64` would not notice a run reported under the
    // wrong representative.
    assert_eq!(
        mine.len(),
        oracle.len(),
        "{name}: interior_knots yielded {} runs, the count oracle {}",
        mine.len(),
        oracle.len()
    );
    for (i, (a, b)) in mine.iter().zip(oracle.iter()).enumerate() {
        assert_eq!(
            a.0.to_bits(),
            b.0.to_bits(),
            "{name}: run {i} value {} is not bitwise the oracle's {}",
            a.0,
            b.0
        );
        assert_eq!(
            a.1, b.1,
            "{name}: run {i} at {} has multiplicity {} but the oracle says {}",
            a.0, a.1, b.1
        );
    }

    // The two facts the docs promise consumers.
    let (lo, hi) = kv.domain();
    for (v, m) in &mine {
        assert!(
            *v > lo && *v < hi,
            "{name}: {v} is not strictly inside [{lo}, {hi}]"
        );
        assert!(
            (1..=p).contains(m),
            "{name}: multiplicity {m} at {v} is outside 1..={p}"
        );
    }
    for w in mine.windows(2) {
        assert!(
            w[0].0 < w[1].0,
            "{name}: not strictly ascending at {} / {}",
            w[0].0,
            w[1].0
        );
    }

    // Total interior length: the runs must partition the interior slice,
    // which is the off-by-one at either bound the method could hide.
    let total: usize = mine.iter().map(|(_, m)| *m).sum();
    assert_eq!(
        total,
        kv.knots().len() - 2 * (p + 1),
        "{name}: the runs cover {total} knots, the interior slice holds {}",
        kv.knots().len() - 2 * (p + 1)
    );

    // `multiplicity_of` counts the WHOLE vector; for an interior value
    // it must still agree, which is the fact `fit.rs`'s retired
    // per-value re-scan silently depended on.
    for (v, m) in &mine {
        assert_eq!(
            kv.multiplicity_of(*v).map(|(s, _)| s),
            Some(*m),
            "{name}: multiplicity_of({v}) disagrees with the interior run"
        );
    }

    // C4: the two `mesh/` gates. The predicate, its sense, and its
    // vacuous case.
    if p >= 1 {
        let max_m = mine.iter().map(|(_, m)| *m).max().unwrap_or(0);
        let gate = kv.interior_knots().any(|(_, m)| m > p - 1);
        assert_eq!(
            gate,
            max_m > p - 1,
            "{name}: the C¹ gate says {gate} but max multiplicity is {max_m} against p−1 = {}",
            p - 1
        );
        if mine.is_empty() {
            assert!(
                !gate,
                "{name}: the C¹ gate must be false on an empty interior"
            );
        }
        // Sense: a vector with a mult-p run must REFUSE, one capped at
        // p−1 must PASS. Checked here rather than asserted about the
        // fixture, so a flipped predicate cannot read as green.
        if max_m == p {
            assert!(gate, "{name}: multiplicity p must trip the C¹ gate");
        }
        if p >= 2 && max_m == p - 1 {
            assert!(!gate, "{name}: multiplicity p−1 must NOT trip the C¹ gate");
        }
    }
    // The degree-1 branch `nurbs_cert.rs` keeps separate.
    assert_eq!(
        kv.interior_knots().next().is_none(),
        mine.is_empty(),
        "{name}: the emptiness test disagrees with the collected runs"
    );
}

// ---------------------------------------------------------------------
// C2: the span search
// ---------------------------------------------------------------------

fn check_span(name: &str, kv: &KnotVector) {
    let knots = kv.knots();
    let (lo, hi) = kv.domain();
    let (first, last) = (kv.first_span(), kv.last_span());

    for t in probes(kv) {
        let got = kv.find_span(t);
        match oracle_span_by_definition(kv, t) {
            Some(want) => assert_eq!(
                got, want,
                "{name}: find_span({t}) = {got}, the definition says {want}"
            ),
            None => {
                // Outside `[lo, hi)`, or NaN: the three documented exits.
                let want = if t.is_nan() || t < lo { first } else { last };
                assert!(
                    t.is_nan() || t < lo || t >= hi,
                    "{name}: {t} is in [lo, hi) yet no span contains it"
                );
                assert_eq!(
                    got, want,
                    "{name}: find_span({t}) = {got}, the totality contract says {want}"
                );
            }
        }
        // The substitution claim itself: agreement on `[lo, hi)`, and
        // the documented disagreement outside — including at `u == lo`,
        // which the retired scan reached only through its clamp run.
        let linear = retired_linear_span(knots, t);
        if t >= lo && t < hi {
            assert_eq!(
                got, linear,
                "{name}: at {t} in [lo, hi) the binary search says {got}, the retired \
                 linear scan {linear} — the substitution is unsound if these ever differ"
            );
        } else if t >= hi {
            assert_eq!(
                linear,
                knots.len() - 1,
                "{name}: at {t} the retired scan must walk into the trailing clamp"
            );
            assert_ne!(got, linear, "{name}: at {t} the two must differ");
        } else {
            assert_eq!(
                linear, 0,
                "{name}: below the domain (or NaN) the retired scan returns its initialiser"
            );
        }
        // Evidence, not a gate: `quad.rs`'s raw-slice span
        // search is the same function as `find_span` on a valid clamped
        // vector, at every probe including the totality exits.
        assert_eq!(
            quad_raw_span(knots, kv.degree(), kv.control_count(), t),
            got,
            "{name}: quad.rs's raw_span disagrees with find_span at {t} — if this ever \
             fires the census note about it is wrong"
        );
        // `span_at` must never land on an empty span; `find_span`'s
        // tie-break is the whole reason the raw insertion path can index
        // `knots[k+1]`.
        assert!(
            kv.span_is_nonempty(got),
            "{name}: find_span({t}) = {got} is an EMPTY span"
        );
        assert!(
            (first..=last).contains(&got),
            "{name}: find_span({t}) = {got} is outside [{first}, {last}]"
        );
    }
}

#[test]
fn interior_knots_and_the_span_search_against_definitional_oracles() {
    let cases = enumerated();
    assert!(
        cases.len() > 100,
        "the enumeration collapsed to {} cases — a builder change has silenced it",
        cases.len()
    );
    // Anti-vacuity: `build` returns `None` for a shape `clamped`
    // refuses, so a change to the builder could silently empty a whole
    // magnitude regime. Each regime's presence is asserted, not hoped
    // for, and the census printed as evidence.
    {
        use std::collections::BTreeMap;
        let mut per: BTreeMap<String, usize> = BTreeMap::new();
        for (n, _) in &cases {
            *per.entry(n.split('/').next().unwrap().to_string())
                .or_default() += 1;
        }
        println!(
            "[evidence] enumerated fixtures: {} total, by regime {per:?}",
            cases.len()
        );
        for regime in [
            "unit",
            "negative",
            "straddling zero",
            "high magnitude",
            "tiny width at high magnitude",
            "subnormal",
            "min positive",
            "signed zero",
            "extreme",
        ] {
            assert!(
                per.get(regime).copied().unwrap_or(0) >= 5,
                "regime {regime:?} contributed {:?} fixtures — the builder has silenced it",
                per.get(regime)
            );
        }
    }
    for (name, kv) in &cases {
        check_query(name, kv);
        check_span(name, kv);
    }

    let mut rng = fuzz::start("d8-adversarial-knot-queries");
    for i in 0..fuzz::scaled(300) {
        let kv = drawn(&mut rng);
        let name = format!(
            "drawn#{i} (p{} len{}) [{}]",
            kv.degree(),
            kv.knots().len(),
            fuzz::replay()
        );
        check_query(&name, &kv);
        check_span(&name, &kv);
    }
}

// ---------------------------------------------------------------------
// C3 / C7: the mutation sequence, and the guard it runs under
// ---------------------------------------------------------------------

/// `to_bezier_spans_extra`'s loop, reconstructed on knots alone: the
/// merge, the sort, and one insertion per missing multiplicity. Before
/// each insertion it asserts the two things the PR moved from "held by
/// the type" to "held by convention" — the list is still a valid clamped
/// vector for `p` (the raw-slice entitlement), and `u` still satisfies
/// the `unreachable!` guard's condition.
fn simulate_decomposition(name: &str, kv: &KnotVector, extra: &[f64]) {
    let p = kv.degree();
    let (lo, hi) = kv.domain();
    let mut knots = kv.knots().to_vec();
    let mut interior: Vec<(f64, usize)> = kv.interior_knots().collect();
    for &v in extra {
        if v > lo && v < hi && !interior.iter().any(|(w, _)| *w == v) {
            interior.push((v, 0));
        }
    }
    interior.sort_by(|a, b| a.0.total_cmp(&b.0));

    let mut inserted = 0usize;
    for (v, m) in &interior {
        for step in *m..p {
            // The entitlement `span_offset_in` now takes on trust.
            let cur = KnotVector::clamped(knots.clone(), p).unwrap_or_else(|e| {
                panic!("{name}: before step {step} at {v} the raw list is not clamped: {e}")
            });
            // The `unreachable!`'s condition, verbatim.
            assert!(
                *v > knots[p] && *v < knots[knots.len() - p - 1],
                "{name}: step {step} would reach insert_once_ring's unreachable! at {v}"
            );
            let k = cur.find_span(*v);
            assert_eq!(
                k,
                retired_linear_span(&knots, *v),
                "{name}: step {step} at {v}: the binary search left the linear scan"
            );
            // The clamps stay untouched, which is what keeps `knots[p]`
            // and `knots[len − p − 1]` meaning `lo` and `hi` next round.
            assert!(
                k >= p && k < knots.len() - p - 1,
                "{name}: step {step} at {v} would insert at {} , inside a clamp run",
                k + 1
            );
            knots.insert(k + 1, *v);
            inserted += 1;
        }
    }
    let final_kv = KnotVector::clamped(knots.clone(), p)
        .unwrap_or_else(|e| panic!("{name}: the decomposed list is not clamped: {e}"));
    assert_eq!(
        knots[p], lo,
        "{name}: the leading clamp moved during decomposition"
    );
    assert_eq!(
        knots[knots.len() - p - 1],
        hi,
        "{name}: the trailing clamp moved during decomposition"
    );
    // Every interior value ends at full multiplicity `p`, which is what
    // makes the span chunks Bézier.
    for (v, m) in final_kv.interior_knots() {
        assert_eq!(
            m, p,
            "{name}: {v} ended at multiplicity {m}, not the degree {p}"
        );
    }
    assert_eq!(
        knots.len(),
        kv.knots().len() + inserted,
        "{name}: insertion count does not match the length growth"
    );
}

#[test]
fn the_raw_slice_entitlement_survives_the_whole_mutation_sequence() {
    let mut cases: Vec<(String, KnotVector, Vec<f64>)> = Vec::new();
    for (name, kv) in enumerated() {
        let (lo, hi) = kv.domain();
        // Extras chosen to break the filter if it can be broken: the
        // ends themselves, one ULP inside each, both signed zeros, the
        // non-finites, and a duplicate of an existing interior knot.
        let mut extra = vec![
            lo,
            hi,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            -0.0,
            0.0,
            f64::from_bits(lo.to_bits().wrapping_add(1)),
            f64::from_bits(hi.to_bits().wrapping_sub(1)),
        ];
        if let Some((v, _)) = kv.interior_knots().next() {
            extra.push(v);
            extra.push(-v);
        }
        cases.push((format!("{name}/no extras"), kv.clone(), Vec::new()));
        cases.push((format!("{name}/adversarial extras"), kv, extra));
    }
    for (name, kv, extra) in &cases {
        simulate_decomposition(name, kv, extra);
    }

    // Drawn vectors with drawn extras, including values that land
    // exactly on existing knots.
    let mut rng = fuzz::start("d8-adversarial-mutation-sequence");
    for i in 0..fuzz::scaled(120) {
        let kv = drawn(&mut rng);
        let (lo, hi) = kv.domain();
        let mut extra: Vec<f64> = Vec::new();
        for _ in 0..rng.below(5) {
            let pick = rng.below(4);
            extra.push(match pick {
                0 => rng.range(lo, hi),
                1 => kv.knots()[rng.below(kv.knots().len())],
                2 => rng.range(lo - (hi - lo), hi + (hi - lo)),
                _ => f64::NAN,
            });
        }
        simulate_decomposition(&format!("drawn#{i} [{}]", fuzz::replay()), &kv, &extra);
    }
}

// ---------------------------------------------------------------------
// C7: the public door
// ---------------------------------------------------------------------

fn lift(v: &[f64]) -> Vec<RingInterval> {
    v.iter().map(|x| RingInterval::point(*x)).collect()
}

/// Drives the one public entry point that forwards caller-supplied
/// break parameters into `to_bezier_spans_extra`, with extras chosen to
/// reach `insert_once_ring`'s `unreachable!` if any input can. A panic
/// here IS the finding; the residual value is not the subject.
#[test]
fn caller_supplied_break_parameters_cannot_reach_the_unreachable() {
    // A bilinear-in-u, quadratic-in-v wall; a cubic pcurve and carrier on
    // one shared domain (the entry point refuses unequal domains).
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv_s = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let sw = vec![1.0; 8];
    let sx: Vec<Vec<RingInterval>> = vec![
        lift(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0]),
        lift(&[0.0, 0.4, 0.9, 0.0, 0.4, 0.9, 0.1, 0.2]),
        lift(&[0.0, 0.2, 0.0, 0.1, 0.3, 0.1, 0.0, 0.1]),
    ];

    // Interior knots the carrier owns and no `ck` below does, so the
    // merge of the two sets is visible in the result's break list.
    let ckc =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.75, 1.0, 1.0, 1.0, 1.0], 3).unwrap();

    for (dname, ck) in [
        (
            "single span",
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap(),
        ),
        (
            "one interior at mult 1",
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap(),
        ),
        (
            "interior at full multiplicity",
            KnotVector::clamped(
                vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0],
                3,
            )
            .unwrap(),
        ),
    ] {
        let n = ck.control_count();
        #[allow(clippy::cast_precision_loss)]
        let ramp =
            |scale: f64| -> Vec<f64> { (0..n).map(|i| scale * i as f64 / n as f64).collect() };
        let pw = vec![1.0; n];
        let px = vec![lift(&ramp(0.9)), lift(&ramp(0.8))];
        // The carrier carries a DIFFERENT interior knot set on the same
        // domain, disjoint from every `ck` above. The entry point merges
        // the two sets, so a fixture sharing one vector between the two
        // curves makes half that merge unobservable: each side re-adds
        // its own knots downstream, and dropping the merge changes
        // nothing anyone can see.
        let nc = ckc.control_count();
        #[allow(clippy::cast_precision_loss)]
        let cramp =
            |scale: f64| -> Vec<f64> { (0..nc).map(|i| scale * i as f64 / nc as f64).collect() };
        let cw = vec![1.0; nc];
        let cx = vec![lift(&cramp(1.0)), lift(&cramp(0.5)), lift(&cramp(0.3))];
        let s = SurfaceRingData::new(&ku, &kv_s, &sw, &sx).unwrap();
        let pd = CurveRingData::new(&ck, &pw, &px).unwrap();
        let cd = CurveRingData::new(&ckc, &cw, &cx).unwrap();

        let one_ulp_in = f64::from_bits(1.0f64.to_bits() - 1);
        let extras: Vec<(&str, Vec<f64>)> = vec![
            ("empty", vec![]),
            ("both domain ends", vec![0.0, 1.0]),
            ("signed zeros", vec![-0.0, 0.0]),
            (
                "non-finite",
                vec![f64::NAN, f64::INFINITY, f64::NEG_INFINITY],
            ),
            ("outside", vec![-1.0, 2.0, f64::MIN, f64::MAX]),
            (
                "one ULP inside each end",
                vec![f64::from_bits(1u64), one_ulp_in],
            ),
            ("duplicates of an existing knot", vec![0.5, 0.5, 0.5]),
            (
                "everything at once",
                vec![
                    0.0,
                    1.0,
                    -0.0,
                    f64::NAN,
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                    -1.0,
                    2.0,
                    0.5,
                    0.5,
                    f64::from_bits(1u64),
                    one_ulp_in,
                    0.25,
                    0.75,
                ],
            ),
        ];
        for (ename, extra) in &extras {
            let out = surface_curve_residual(&s, &pd, &cd, extra).unwrap_or_else(|e| {
                panic!("{dname}/{ename}: the entry point refused a legal call: {e:?}")
            });
            // WHERE the spans were placed, not merely that a number
            // came back: an extra that mislocated a span, or that was
            // let through when it should have been filtered, moves this
            // list while leaving the bound finite.
            assert_eq!(
                out.breaks(),
                admitted_breaks(&ck, &ckc, extra),
                "{dname}/{ename}: the extras landed a different break structure"
            );
            // The bound must still be a real number: an extra that
            // corrupted the break structure would show up as poison.
            let sup = out.sup_bound();
            assert!(
                sup.is_finite(),
                "{dname}/{ename}: sup bound is {sup} — the extras corrupted the decomposition"
            );
        }
    }
}
