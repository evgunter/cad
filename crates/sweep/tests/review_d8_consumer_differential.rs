//! **Review probe for #744 (Track D, D8): the ELEVEN CALL SITES, not the
//! method.**
//!
//! #744's own differential (`geom-core/tests/knot_queries_differential.rs`)
//! proves `KnotVector::interior_knots()` equals the six hand-written scans
//! it replaces. That is a claim about the *method*. It is not the claim the
//! PR actually makes, which is that **eleven call-site expressions were
//! rewritten and every consumer still produces the same answer** — a
//! `while` loop that returned on the first over-multiplicity run became
//! `.any(…)`, a `Vec` became a borrowed iterator that two sites now have to
//! `.collect()` for lifetime reasons, and one span derivation changed from
//! a linear scan to a binary search *inside* a mid-mutation ring loop.
//! Nothing in the tree drives the consumers across that rewrite.
//!
//! This file drives three of the four consumer crates through their PUBLIC
//! doors and GATES each one: the door's structural postcondition,
//! re-derived here from the returned structure with this file's own scan —
//! never by calling `interior_knots`, which is the thing under test.
//!
//! `mesh/`'s two sites are NOT reachable from this crate (they want a
//! tessellated body) and are the two pure predicate rewrites; they are
//! covered by inspection in the review, not here.
//!
//! # Seeds
//!
//! The draws VARY per run ([`test_utils::fuzz::start`]), which is the
//! shape these rows are: *for all sampled structures, the door's
//! postcondition holds*. Cutting the effort dial loses detection power
//! and can never turn a green run red, so successive runs explore new
//! ground and a failure names its seed for replay.
//!
//! They were pinned once, and the licence was real while it lasted: this
//! file also carried a bit-exact digest of every door's output, printed so
//! that the same binary run on the merge base and on the tip was a
//! byte-level differential of the whole rewrite — and a differential needs
//! both sides to see byte-identical inputs. That comparison happened at
//! the review it was built for and nothing schedules another, so the
//! digest is gone and the pin went with it: a fixture identifier for a
//! comparison nobody is making is 24 points explored forever and no claim
//! at all. Promoting the digest into a literal was the alternative and is
//! worse — it pins today's arithmetic as a target, which is not a
//! statement about whether any of these doors is right.
//!
//! No row's anti-vacuity rests on the draw: the first two count over a
//! written-down fixture set that is there on every run, and the third
//! asserts its written-down cases one by one. So nothing here is
//! anti-monotone in the sample count.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-core/src/spline/",
    "crates/geom/src/curves/",
    "crates/geom/src/curves.rs",
    "crates/sweep/src/skin.rs",
    "crates/sweep/src/loft.rs",
];

use geom::NurbsCurve3;
use geom_core::spline::compose::{CurveRingData, linear_composite};
use geom_core::spline::{KnotVector, SplineError};
use geom_core::{Point3, RingInterval};
use test_utils::fuzz;

// ---------------------------------------------------------------------
// This file's OWN interior scan — deliberately not `interior_knots()`
// ---------------------------------------------------------------------

/// Distinct interior values with multiplicities, written here so the
/// gates below do not verify the method against itself. Spelled
/// differently from all six retired copies on purpose (a fold, not a
/// while-loop and not `chunk_by`).
fn own_interior(kv: &KnotVector) -> Vec<(f64, usize)> {
    let p = kv.degree();
    let k = kv.knots();
    k[p + 1..k.len() - p - 1]
        .iter()
        .fold(Vec::<(f64, usize)>::new(), |mut acc, u| {
            if acc.last().map(|(v, _)| *v == *u) == Some(true) {
                acc.last_mut().expect("just checked non-empty").1 += 1;
            } else {
                acc.push((*u, 1));
            }
            acc
        })
}

// ---------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------

fn kv_of(degree: usize, interior: &[(f64, usize)]) -> Result<KnotVector, SplineError> {
    let mut knots = vec![0.0; degree + 1];
    for (v, m) in interior {
        knots.extend(core::iter::repeat_n(*v, *m));
    }
    knots.extend(core::iter::repeat_n(1.0, degree + 1));
    KnotVector::clamped(knots, degree)
}

/// A curve on `kv` whose control points are a deterministic function of
/// the index, so the digest depends only on the structure and the code
/// path — never on a draw the two builds might not share.
fn curve_on(kv: KnotVector, weight_pattern: usize) -> NurbsCurve3<f64> {
    let n = kv.control_count();
    let control: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            let t = i as f64;
            Point3::new(t, 0.25 * t * t - t, (0.5 * t).sin())
        })
        .collect();
    let weights: Vec<f64> = (0..n)
        .map(|i| match weight_pattern {
            0 => 1.0,
            1 => 1.0 + 0.25 * (i % 3) as f64,
            _ => 0.5 + 0.5 * ((i % 5) + 1) as f64,
        })
        .collect();
    NurbsCurve3::new(kv, control, weights).expect("counts match by construction")
}

/// The structures every door below is driven over: written-down shapes
/// (empty interior, multiplicity 1, multiplicity `p`, mixed, adjacent
/// runs) first, then drawn ones. The written-down prefix is what every
/// floor below counts on; the draws are the search.
fn structures(rng: &mut fuzz::Rng) -> Vec<(String, KnotVector)> {
    let mut out = Vec::new();
    for p in 1..=4usize {
        for (label, interior) in [
            ("empty", vec![]),
            ("one/mult1", vec![(0.5, 1)]),
            ("one/multp", vec![(0.5, p)]),
            ("two/1-then-p", vec![(0.3, 1), (0.7, p)]),
            ("two/p-then-1", vec![(0.3, p), (0.7, 1)]),
            (
                "five/single",
                vec![(0.1, 1), (0.3, 1), (0.5, 1), (0.7, 1), (0.9, 1)],
            ),
        ] {
            if let Ok(kv) = kv_of(p, &interior) {
                out.push((format!("p{p}/{label}"), kv));
            }
        }
        if p >= 2
            && let Ok(kv) = kv_of(p, &[(0.25, p - 1), (0.5, p - 1), (0.75, p - 1)])
        {
            out.push((format!("p{p}/adjacent p-1 runs"), kv));
        }
    }
    for i in 0..fuzz::scaled(24) {
        let p = 1 + rng.below(4);
        let count = rng.below(6);
        let mut values: Vec<f64> = Vec::new();
        for _ in 0..count {
            let v = f64::from(1 + rng.below(31) as u32) / 32.0;
            if !values.contains(&v) {
                values.push(v);
            }
        }
        values.sort_by(f64::total_cmp);
        let interior: Vec<(f64, usize)> =
            values.into_iter().map(|v| (v, 1 + rng.below(p))).collect();
        if let Ok(kv) = kv_of(p, &interior) {
            out.push((format!("drawn#{i}/p{p}"), kv));
        }
    }
    out
}

// ---------------------------------------------------------------------
// Door 1 — geom-core: the RING insertion path (compose.rs)
// ---------------------------------------------------------------------

/// `linear_composite` runs `to_bezier_spans_extra`, which is where the
/// linear span scan became `find_span_in` **inside** the mutation loop.
/// This is the site where an off-by-one is a wrong curve rather than a
/// compile error, and no other suite drives it across the rewrite.
///
/// Gate: the decomposition's break list must be exactly the domain ends
/// plus every distinct interior knot value — re-derived here.
#[test]
fn the_ring_bezier_decomposition_still_breaks_where_the_structure_says() {
    let mut rng = fuzz::start("d8-consumer-compose");
    let mut checked = 0usize;
    for (name, kv) in structures(&mut rng) {
        let n = kv.control_count();
        let weights: Vec<f64> = (0..n).map(|i| 1.0 + 0.125 * (i % 4) as f64).collect();
        let coords: Vec<Vec<RingInterval>> = (0..3)
            .map(|c| {
                (0..n)
                    .map(|i| RingInterval::point((i as f64) * (1.0 + c as f64) - 2.0))
                    .collect()
            })
            .collect();
        let data = CurveRingData::new(&kv, &weights, &coords).expect("valid ring data");
        let form = linear_composite(&data, &[1.0, -2.0, 0.5], 0.75).expect("3 channels");

        let (lo, hi) = kv.domain();
        let mut want: Vec<f64> = vec![lo];
        want.extend(own_interior(&kv).iter().map(|(v, _)| *v));
        want.push(hi);
        assert_eq!(
            form.num.breaks(),
            want.as_slice(),
            "{name}: the Bezier decomposition's breaks are not the domain ends plus the \
             distinct interior knots — the raw insertion path's span derivation moved"
        );
        assert_eq!(
            form.num.breaks(),
            form.den.breaks(),
            "{name}: numerator and denominator decomposed onto different break lists"
        );
        assert_eq!(
            form.num.spans().len(),
            want.len() - 1,
            "{name}: span count does not match the break list"
        );
        for row in form.num.spans() {
            assert_eq!(
                row.len(),
                form.num.degree() + 1,
                "{name}: a Bernstein span row is not degree+1 wide"
            );
        }

        checked += 1;
    }
    assert!(
        checked >= 20,
        "the fixture set collapsed to {checked} cases"
    );
}

// ---------------------------------------------------------------------
// Door 2 — sweep: make_compatible (skin.rs, two call sites)
// ---------------------------------------------------------------------

/// `make_compatible` is the §5.3 knot-merge, and it is the one consumer
/// that calls `interior_knots()` **twice in one function**, once
/// borrowed and once `.collect()`ed. Gate: every returned section shares
/// one bit-identical knot vector, and that vector's interior
/// multiplicity at each value is the max over the inputs — re-derived
/// here from the inputs, after the degree elevation the door does first.
#[test]
fn make_compatible_still_lands_every_section_on_the_union_vector() {
    let mut rng = fuzz::start("d8-consumer-skin");
    let all = structures(&mut rng);
    let mut groups = 0usize;
    for window in all.chunks(3) {
        if window.len() < 2 {
            continue;
        }
        let sections: Vec<NurbsCurve3<f64>> = window
            .iter()
            .enumerate()
            .map(|(i, (_, kv))| curve_on(kv.clone(), i))
            .collect();
        let names: Vec<&str> = window.iter().map(|(n, _)| n.as_str()).collect();
        let same_degree = window
            .iter()
            .all(|(_, kv)| kv.degree() == window[0].1.degree());
        let out = match sweep::skin::make_compatible(&sections) {
            Ok(out) => out,
            // Refusals are a legitimate answer for MIXED degrees (the
            // elevation can exceed an internal budget). For sections
            // already at one degree the merge is pure structure and
            // must not refuse — that arm is a gate.
            Err(e) => {
                assert!(
                    !same_degree,
                    "{names:?}: make_compatible refused ({e:?}) a set already at one degree, \
                     where the merge is pure knot union"
                );
                continue;
            }
        };
        assert_eq!(
            out.len(),
            sections.len(),
            "{names:?}: section count changed"
        );
        let first = out[0].knots().clone();
        for (i, c) in out.iter().enumerate() {
            assert_eq!(
                c.knots().degree(),
                first.degree(),
                "{names:?}: section {i} has a different degree after make_compatible"
            );
            assert_eq!(
                c.knots().knots(),
                first.knots(),
                "{names:?}: section {i}'s knot vector is not bit-identical to section 0's — \
                 compatibility is exactly the claim that it is"
            );
        }
        // The union promise, from the ELEVATED inputs.
        let degree = first.degree();
        for (i, c) in sections.iter().enumerate() {
            let raise = degree - c.knots().degree();
            let elevated = if raise == 0 {
                c.clone()
            } else {
                match c.elevate_degree(raise) {
                    Ok(e) => e,
                    Err(_) => continue,
                }
            };
            for (v, m) in own_interior(elevated.knots()) {
                let got = own_interior(&first)
                    .into_iter()
                    .find(|(w, _)| *w == v)
                    .map_or(0, |(_, m)| m);
                assert!(
                    got >= m,
                    "{names:?}: input {i} has {v} at multiplicity {m}, the merged vector only \
                     reaches {got} — the union lost a section's structure"
                );
            }
        }
        groups += 1;
    }
    assert!(groups >= 5, "only {groups} section groups were compatible");
}

// ---------------------------------------------------------------------
// Door 3 — geom: the fit removal sweep + deviation_from (fit.rs)
// ---------------------------------------------------------------------

/// The `(count, degree, tol)` cases this row does not leave to the
/// draw, each asserted where it is used rather than counted up
/// afterwards: whether a fit succeeds is a property of the case, so a
/// COUNT over drawn cases would be satisfied by luck and would grow
/// anti-monotone in the sample count — the shape
/// [`test_utils::fuzz`] warns against mixing into a search. Asserting
/// per case avoids the count entirely and names the case that broke.
const WRITTEN_DOWN_FITS: [(usize, usize, f64); 5] = [
    (11, 3, 0.02),
    (16, 2, 0.05),
    (8, 4, 0.08),
    (19, 3, 0.01),
    (6, 2, 0.04),
];

/// `approximate` drives both `fit.rs` sites: the removal sweep's
/// candidate list and `deviation_from`'s union. Gate: whatever comes
/// back is a valid clamped vector whose interior multiplicities are in
/// `1..=p` — re-derived here — and the reported bound is finite and
/// within tolerance on success.
///
/// The written-down cases come first and must all fit — meaning the
/// door must not REFUSE them; it is not a claim about fit quality, and
/// it cannot become one here. `approximate` falls back to the degree-1
/// interpolant rather than refusing, so a written-down case still
/// answers `Ok` at a tolerance of `1e-13` (measured). What the
/// written-down half pins is that the door keeps answering on shapes
/// nobody drew; what the gates below pin is that whatever it answers
/// has valid structure and a bound inside the tolerance it was given.
/// The drawn cases after them are the search, and a refusal there is a
/// legitimate answer for a tolerance too tight for the degree.
#[test]
fn the_fit_removal_sweep_still_returns_valid_structure() {
    let mut rng = fuzz::start("d8-consumer-fit");
    let cases: Vec<(usize, usize, f64)> = WRITTEN_DOWN_FITS
        .iter()
        .copied()
        .chain((0..fuzz::scaled(24)).map(|_| {
            (
                6 + rng.below(14),
                2 + rng.below(3),
                0.01 * f64::from(1 + rng.below(8) as u32),
            )
        }))
        .collect();
    for (case, (count, degree, tol)) in cases.into_iter().enumerate() {
        let points: Vec<Point3<f64>> = (0..count)
            .map(|i| {
                let t = i as f64 / (count - 1) as f64;
                Point3::new(t, (3.0 * t).sin() * 0.3, t * t * 0.5)
            })
            .collect();
        let outcome = match NurbsCurve3::approximate(&points, degree, tol) {
            Ok(o) => o,
            Err(e) => {
                assert!(
                    case >= WRITTEN_DOWN_FITS.len(),
                    "written-down case {case} ({count} points, degree {degree}, tol \
                     {tol}) no longer fits at all: {e:?}"
                );
                continue;
            }
        };
        let kv = outcome.curve.knots();
        let p = kv.degree();
        assert!(
            KnotVector::clamped(kv.knots().to_vec(), p).is_ok(),
            "case {case}: approximate returned a vector that does not re-validate"
        );
        for (v, m) in own_interior(kv) {
            assert!(
                (1..=p).contains(&m),
                "case {case}: interior knot {v} came back at multiplicity {m}, outside 1..={p}"
            );
        }
        assert!(
            outcome.bound.is_finite() && outcome.bound <= tol,
            "case {case}: reported bound {} is not a success within {tol}",
            outcome.bound
        );
    }
}
