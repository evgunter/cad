//! **R1 review probes for M10-1** (distributions in the document).
//!
//! Independent consumer suite: every row here derives what the PR
//! CLAIMS from the spec and the ratified design (E1/E2), not from a
//! re-reading of its diff. Written against the public surface only.
//!
//! The sampled rows follow `memories/test-suite-cost.md`: counterexample
//! searches with a varying seed, logged unconditionally, counts on the
//! `R1_EFFORT` dial, replayable via `R1_SEED`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use editor_core::{
    AnalysisPolicy, CancelToken, DEFAULT_QUANTILE_MASS, Dimension, Distribution, DocEdit, DocParam,
    DocumentId, EvalOptions, MeasureUnavailable, OffsetInterval, ParamName, ProfileDoc,
    analyzed_box, apply, box_mass, evaluate, tail_mass,
};
use geom_core::Tol;

// A tiny deterministic-per-run PRNG (SplitMix64) so the probe file
// carries no new dependency. Seed varies per run and is logged
// unconditionally; override with R1_SEED for exact replay.
struct Rng(u64);
impl Rng {
    fn from_env() -> (Self, u64) {
        let seed = std::env::var("R1_SEED")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("clock after epoch")
                    .as_nanos() as u64
            });
        println!("R1 probe seed: {seed} (replay with R1_SEED={seed})");
        (Self(seed), seed)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    /// Uniform in [0, 1).
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    /// Uniform in [lo, hi).
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }
}

fn effort() -> usize {
    std::env::var("R1_EFFORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
}

fn p(name: &str) -> ParamName {
    ParamName::new(name)
}

/// An independent oracle for `P(lo <= X <= hi)`, X ~ N(0, sigma²):
/// adaptive Simpson integration of the density — shares no code with
/// the implementation's `libm::erf` route.
fn normal_mass_oracle(sigma: f64, lo: f64, hi: f64) -> f64 {
    let density = |x: f64| {
        let t = x / sigma;
        (-0.5 * t * t).exp() / (sigma * (2.0 * std::f64::consts::PI).sqrt())
    };
    // Clip to ±12σ: the mass beyond is < 1e-32, far below the 1e-9
    // comparisons the consumers below make.
    let a = lo.max(-12.0 * sigma);
    let b = hi.min(12.0 * sigma);
    if a >= b {
        return 0.0;
    }
    let n = 20_000; // even; Simpson error ~ (b-a)^5 / n^4, ample here
    let h = (b - a) / n as f64;
    let mut sum = density(a) + density(b);
    for i in 1..n {
        let x = a + i as f64 * h;
        sum += density(x) * if i % 2 == 1 { 4.0 } else { 2.0 };
    }
    sum * h / 3.0
}

/// CLAIM 4 (accounting honesty), independently derived: for sampled
/// priceable forms and sampled boxes, `box_mass` matches an
/// integration oracle, stays in `[0, 1]`, and complements sum to 1.
#[test]
fn sampled_masses_match_an_independent_integration_oracle() {
    let (mut rng, seed) = Rng::from_env();
    let rounds = 40 * effort();
    for round in 0..rounds {
        let sigma = rng.range(1e-4, 2.0);
        let (slo, shi) = (-rng.range(0.0, 3.0 * sigma), rng.range(0.0, 3.0 * sigma));
        let (a, b) = {
            let x = rng.range(-4.0 * sigma, 4.0 * sigma);
            let y = rng.range(-4.0 * sigma, 4.0 * sigma);
            (x.min(y), x.max(y))
        };
        let ctx = format!("seed {seed} round {round}: sigma {sigma}, sub ({a}, {b})");

        // Normal against the oracle.
        let got = box_mass(&p("x"), &Distribution::Normal { sigma }, (a, b)).expect("priceable");
        let want = normal_mass_oracle(sigma, a, b);
        assert!((got - want).abs() < 1e-9, "{ctx}: normal {got} vs {want}");

        // TruncatedNormal against the renormalized oracle.
        let t = Distribution::TruncatedNormal {
            sigma,
            lo: slo,
            hi: shi,
        };
        if shi > slo {
            let got = box_mass(&p("x"), &t, (a, b)).expect("priceable");
            let denom = normal_mass_oracle(sigma, slo, shi);
            let numer = normal_mass_oracle(sigma, a.max(slo), b.min(shi)).max(0.0);
            let want = if a.max(slo) < b.min(shi) {
                numer / denom
            } else {
                0.0
            };
            assert!(
                (got - want).abs() < 1e-8,
                "{ctx}: truncated ({slo}, {shi}) {got} vs {want}"
            );
        }

        // Uniform against the exact ratio.
        let u = Distribution::Uniform { lo: slo, hi: shi };
        if shi > slo {
            let got = box_mass(&p("x"), &u, (a, b)).expect("priceable");
            let overlap = (b.min(shi) - a.max(slo)).max(0.0);
            let want = overlap / (shi - slo);
            assert!(
                (got - want).abs() < 1e-12,
                "{ctx}: uniform ({slo}, {shi}) {got} vs {want}"
            );
        }

        // Complements: mass(sub) + mass(left of sub) + mass(right of
        // sub) = 1 for every priceable form (BIG covers any support
        // drawn above; the normal's residue beyond it is < 1e-300).
        const BIG: f64 = 1e9;
        for dist in [
            Distribution::Normal { sigma },
            Distribution::Uniform { lo: slo, hi: shi },
            t,
        ] {
            if matches!(
                dist,
                Distribution::Uniform { lo, hi } | Distribution::TruncatedNormal { lo, hi, .. }
                if hi <= lo
            ) {
                continue;
            }
            let inside = box_mass(&p("x"), &dist, (a, b)).expect("priceable");
            let left = box_mass(&p("x"), &dist, (-BIG, a)).expect("priceable");
            let right = box_mass(&p("x"), &dist, (b, BIG)).expect("priceable");
            // The seam points a and b are each counted twice by the
            // three closed intervals; a continuous density puts zero
            // mass on a point, so the sum is still 1.
            assert!(
                (inside + left + right - 1.0).abs() < 1e-9,
                "{ctx}: {dist:?} partition {inside} + {left} + {right}"
            );
        }
    }
}

/// CLAIM 4: `TruncatedNormal`'s tail on its own support is EXACTLY
/// zero — `==`, not "small" — across sampled supports and sigmas.
#[test]
fn truncated_normal_tail_is_exactly_zero_on_its_own_support() {
    let (mut rng, seed) = Rng::from_env();
    for round in 0..(50 * effort()) {
        let sigma = rng.range(1e-6, 10.0);
        let lo = -rng.range(0.0, 5.0 * sigma);
        let hi = rng.range(f64::MIN_POSITIVE, 5.0 * sigma);
        let dist = Distribution::TruncatedNormal { sigma, lo, hi };
        assert_eq!(
            tail_mass(&p("t"), &dist, &OffsetInterval { lo, hi }),
            Ok(0.0),
            "seed {seed} round {round}: sigma {sigma} support ({lo}, {hi})"
        );
    }
}

/// CLAIM 4: the Band answers ONLY on the overlap topologies where
/// every measure consistent with it agrees, enumerated exhaustively —
/// an enumeration, not a fuzz (`test-suite-cost.md`).
#[test]
fn band_answers_are_exactly_the_measure_free_ones() {
    let band = Distribution::Band { lo: -1.0, hi: 1.0 };
    let cases: [(f64, f64, Option<f64>); 9] = [
        (-3.0, -2.0, Some(0.0)), // disjoint left
        (2.0, 3.0, Some(0.0)),   // disjoint right
        (-3.0, 3.0, Some(1.0)),  // strictly covers
        (-1.0, 1.0, Some(1.0)),  // exactly the support
        (-3.0, -1.0, None),      // touches at lo: an atom could sit there
        (1.0, 3.0, None),        // touches at hi
        (-3.0, 0.0, None),       // partial from the left
        (0.0, 3.0, None),        // partial from the right
        (-0.5, 0.5, None),       // strictly inside
    ];
    for (a, b, expected) in cases {
        let got = box_mass(&p("bore"), &band, (a, b));
        match expected {
            Some(v) => assert_eq!(got, Ok(v), "({a}, {b})"),
            None => assert_eq!(
                got,
                Err(MeasureUnavailable::BandHasNoMeasure { param: p("bore") }),
                "({a}, {b}) is shape-dependent and must refuse"
            ),
        }
        // tail_mass agrees door-for-door where the interval is one an
        // analyzed box can be (lo <= 0 <= hi).
        if a <= 0.0 && b >= 0.0 {
            let tail = tail_mass(&p("bore"), &band, &OffsetInterval { lo: a, hi: b });
            match expected {
                Some(v) => assert_eq!(tail, Ok(1.0 - v), "tail over ({a}, {b})"),
                None => assert!(tail.is_err(), "tail over ({a}, {b}) must refuse"),
            }
        }
    }
}

/// CLAIM re the bisection: deterministic (bit-identical across calls),
/// monotone in the requested mass, and the box it draws actually holds
/// at least (within an ulp's worth of scaling) the requested mass.
#[test]
fn the_quantile_box_is_deterministic_monotone_and_covers_its_mass() {
    let (mut rng, seed) = Rng::from_env();
    let dist = Distribution::Normal { sigma: 1.0 };
    let doc = {
        let doc = ProfileDoc::empty(DocumentId::derive("r1-quantile"), Tol::witness());
        apply(
            &doc,
            &DocEdit::SetDocParam {
                name: p("n"),
                value: DocParam::continuous_with(Dimension::Length, 0.0, dist),
            },
            Tol::witness(),
        )
        .expect("applies")
        .doc
    };
    let hi_for = |mass: f64| {
        let policy = AnalysisPolicy::new(mass).expect("valid mass");
        analyzed_box(&doc, &policy)
            .get(&p("n"))
            .expect("axis")
            .offsets
            .hi
    };
    let mut masses: Vec<f64> = (0..(20 * effort()))
        .map(|_| rng.range(1e-6, 1.0 - 1e-12))
        .collect();
    masses.push(DEFAULT_QUANTILE_MASS);
    masses.sort_by(f64::total_cmp);
    let mut prev: Option<(f64, f64)> = None;
    for &mass in &masses {
        let z1 = hi_for(mass);
        let z2 = hi_for(mass);
        assert_eq!(
            z1.to_bits(),
            z2.to_bits(),
            "seed {seed}: same request, same box, bit for bit (mass {mass})"
        );
        let covered = box_mass(&p("n"), &dist, (-z1, z1)).expect("priced");
        assert!(
            covered >= mass - 1e-12,
            "seed {seed}: box for mass {mass} covers only {covered}"
        );
        if let Some((pm, pz)) = prev {
            assert!(
                z1 >= pz,
                "seed {seed}: monotone: mass {pm} -> {pz}, mass {mass} -> {z1}"
            );
        }
        prev = Some((mass, z1));
    }
}

/// CLAIM 5: `bit_eq` and the document diff both see the new field —
/// every pair of DIFFERENT annotations (all four forms, `None`
/// included, both orders) on an otherwise identical parameter is
/// unequal and diffs as a param change.
#[test]
fn every_annotation_pair_is_visible_to_bit_eq_and_diff() {
    let annotations: [Option<Distribution>; 6] = [
        None,
        Some(Distribution::Band { lo: -0.1, hi: 0.1 }),
        Some(Distribution::Uniform { lo: -0.1, hi: 0.1 }),
        Some(Distribution::Normal { sigma: 0.1 }),
        Some(Distribution::TruncatedNormal {
            sigma: 0.1,
            lo: -0.1,
            hi: 0.1,
        }),
        // Same form as [3], different bits: the depth beyond the tag.
        Some(Distribution::Normal { sigma: 0.2 }),
    ];
    let doc_for = |ann: Option<Distribution>| {
        let doc = ProfileDoc::empty(DocumentId::derive("r1-bit-eq"), Tol::witness());
        let value = match ann {
            None => DocParam::continuous(Dimension::Length, 1.0),
            Some(d) => DocParam::continuous_with(Dimension::Length, 1.0, d),
        };
        apply(
            &doc,
            &DocEdit::SetDocParam {
                name: p("q"),
                value,
            },
            Tol::witness(),
        )
        .expect("applies")
        .doc
    };
    for (i, a) in annotations.iter().enumerate() {
        for (j, b) in annotations.iter().enumerate() {
            let (da, db) = (doc_for(*a), doc_for(*b));
            let same = i == j;
            assert_eq!(
                da.bit_eq(&db),
                same,
                "annotations {a:?} vs {b:?}: bit_eq must be {same}"
            );
            let diff = da.diff(&db);
            assert_eq!(
                diff.params,
                if same { vec![] } else { vec![p("q")] },
                "annotations {a:?} vs {b:?}: diff"
            );
        }
    }
}

/// CLAIM 1 (zero evaluation impact), the memo half: a
/// DISTRIBUTION-ONLY edit to the corpus's parametric document reuses
/// EVERY node of the prior evaluation — it invalidates nothing.
#[test]
fn a_distribution_only_edit_invalidates_no_memoized_evaluation() {
    let d = corpus::plate_param::document();
    let full = corpus::eval::<f64>(&d.doc);
    assert!(corpus::failures(&full).is_empty(), "corpus doc is green");
    let annotated = apply(
        &d.doc,
        &DocEdit::SetDocParam {
            name: p(corpus::plate_param::HOLE_R),
            value: DocParam::continuous_with(
                Dimension::Length,
                corpus::plate_param::HOLE_R_VALUE,
                Distribution::Normal { sigma: 1e-4 },
            ),
        },
        Tol::witness(),
    )
    .expect("a distribution-only edit applies")
    .doc;
    let after = evaluate::<f64>(
        &annotated,
        Some(&full),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(corpus::failures(&after).is_empty(), "still green");
    assert_eq!(
        (after.recomputed, after.reused),
        (0, d.doc.len()),
        "a distribution edit recomputes NOTHING: same nominal, same keys"
    );
    // And the headline geometry is bit-identical.
    let result = d.result.expect("plate_param has a headline body");
    let (before_body, after_body) = (
        corpus::body_of(&full, result),
        corpus::body_of(&after, result),
    );
    let (mb, ma) = (
        topo::mass_properties(before_body, Tol::witness()).expect("props"),
        topo::mass_properties(after_body, Tol::witness()).expect("props"),
    );
    assert_eq!(mb.volume.to_bits(), ma.volume.to_bits());
    assert_eq!(mb.surface_area.to_bits(), ma.surface_area.to_bits());
}

/// CLAIM 1, the environment half at BOTH ends of a doc's life: the
/// param env never differs, and a distribution-only edit round-tripped
/// through persistence still evaluates to the same environment.
#[test]
fn the_param_env_is_blind_to_annotations_even_after_a_round_trip() {
    let plain = {
        let doc = ProfileDoc::empty(DocumentId::derive("r1-env"), Tol::witness());
        apply(
            &doc,
            &DocEdit::SetDocParam {
                name: p("d"),
                value: DocParam::continuous(Dimension::Angle, 0.25),
            },
            Tol::witness(),
        )
        .expect("applies")
        .doc
    };
    let annotated = {
        let doc = ProfileDoc::empty(DocumentId::derive("r1-env"), Tol::witness());
        apply(
            &doc,
            &DocEdit::SetDocParam {
                name: p("d"),
                value: DocParam::continuous_with(
                    Dimension::Angle,
                    0.25,
                    Distribution::Uniform {
                        lo: -0.01,
                        hi: 0.02,
                    },
                ),
            },
            Tol::witness(),
        )
        .expect("applies")
        .doc
    };
    assert_eq!(
        plain.param_env::<f64>().bindings,
        annotated.param_env::<f64>().bindings
    );
    let text = editor_core::save(&annotated, &[], Tol::witness()).expect("saves");
    let back = editor_core::load(&text, Tol::witness()).expect("loads").doc;
    assert_eq!(
        plain.param_env::<f64>().bindings,
        back.param_env::<f64>().bindings,
        "the annotation survives the file, the environment never sees it"
    );
}

/// CLAIM 3 hardening: `analyzed_box` + `tail_mass` compose over a
/// MIXED document (all four forms + fixed + count) without a refusal
/// anywhere a measure exists, and the box's axes are exactly the
/// continuous params.
#[test]
fn a_mixed_document_analyzes_end_to_end() {
    let mut doc = ProfileDoc::empty(DocumentId::derive("r1-mixed"), Tol::witness());
    let sets: [(&str, DocParam); 6] = [
        (
            "band",
            DocParam::continuous_with(
                Dimension::Length,
                1.0,
                Distribution::Band { lo: -0.1, hi: 0.1 },
            ),
        ),
        (
            "uniform",
            DocParam::continuous_with(
                Dimension::Length,
                2.0,
                Distribution::Uniform { lo: -0.2, hi: 0.05 },
            ),
        ),
        (
            "normal",
            DocParam::continuous_with(Dimension::Scalar, 0.5, Distribution::Normal { sigma: 0.01 }),
        ),
        (
            "truncated",
            DocParam::continuous_with(
                Dimension::Angle,
                0.1,
                Distribution::TruncatedNormal {
                    sigma: 0.05,
                    lo: -0.1,
                    hi: 0.1,
                },
            ),
        ),
        ("fixed", DocParam::continuous(Dimension::Length, 3.0)),
        ("count", DocParam::Count { value: 6 }),
    ];
    for (name, value) in sets {
        doc = apply(
            &doc,
            &DocEdit::SetDocParam {
                name: p(name),
                value,
            },
            Tol::witness(),
        )
        .expect("applies")
        .doc;
    }
    let b = analyzed_box(&doc, &AnalysisPolicy::default());
    assert_eq!(b.params().len(), 5, "five continuous axes, no count");
    assert_eq!(b.varying().count(), 4, "four declared variable");
    for (name, axis) in b.params() {
        let Some(dist) = axis.distribution else {
            assert!(axis.offsets.is_fixed());
            continue;
        };
        let tail = tail_mass(name, &dist, &axis.offsets).expect("every analyzed box is priceable");
        match dist {
            Distribution::Normal { .. } => assert!(tail > 0.0 && tail < 0.005),
            _ => assert_eq!(tail, 0.0, "{name:?}: bounded forms leave nothing out"),
        }
    }
}
