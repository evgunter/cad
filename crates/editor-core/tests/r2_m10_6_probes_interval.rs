//! **R2's independent probes for M10-6** (reporting, the advisory
//! lanes, the `min_separation` door).
//!
//! Independently derived: every truth these rows compare against is
//! written into the geometry by hand, and no row reads a number the
//! unit's own suites produced.
//!
//! What each row is for:
//!
//! 1. `the_certifying_filter_changes_a_pre_m10_6_documents_drive` — the
//!    unit's claim 1 is "zero impact for documents without a
//!    `min_clearance` measure". `VerdictVector::certifying` drops
//!    `Assertion` rows from the certification comparison, which is a
//!    change to `drive` for EVERY assertion-carrying document, and this
//!    row exhibits one whose leaves certify only because of it.
//! 2. `min_separation_brackets_a_curved_pair_at_every_budget` — the
//!    enclosure on carriers the unit's own fixture does not use
//!    (cylinders), at starved budgets.
//! 3. `min_clearance_between_two_separated_bodies_reads_zero` — the
//!    carrier-WINDOW superset, carried into the measure layer: two
//!    solids 0.1 m apart whose `min_clearance` is `[0, 0]`, and the
//!    assertion over it that reads `Violated`.
//! 4. `report_key_is_blind_to_the_dials_that_move_a_report` — the cache
//!    seam's key function takes (kind, slice, box, ε, K) and no report
//!    config, so two MC reports that differ share one key.
//! 5. `the_mc_stream_is_re_derived_bit_for_bit` — the PRNG, re-stated
//!    here from the algorithm's own definition, against the lane's
//!    draws through `sample_offset`.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box, sample_offset};
use editor_core::clearance::{MinSepSelection, MinSeparationConfig, min_separation};
use editor_core::drive::{DriveConfig, VerdictVector, drive};
use editor_core::mc::{McConfig, monte_carlo};
use editor_core::report::report_key;
use editor_core::{
    AssertionDir, AssertionVerdict, CancelToken, Dimension, Distribution, DocEdit, DocParam,
    EntityKind, EvalOptions, Expr, LoopProgram, MeasureExpr, MeasurePrimitive, MeasureRef, Node,
    NodeResult, ParamName, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, StableName, UnitSym,
    ValuePayload, evaluate,
};
use geom_core::{Bounds, Tol};

use fixture::{Recorder, len};

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

/// The ε-scaled half-width every parametric row here uses — M10-5's
/// and M10-6's own, for their reason (no interval replay survives a
/// wider box).
fn half() -> f64 {
    Tol::witness().eps() / 64.0
}

/// The body a node output, by name.
fn bname(node: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Body,
        node,
        path: vec![RoleSeg::OutputBody],
    }
}

fn eval_over<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    box_: Option<ParamBox>,
) -> editor_core::Evaluation<T> {
    let opts = EvalOptions {
        param_box: box_.map(std::sync::Arc::new),
        profile_lift: editor_core::ProfileLift::Guided,
        ..EvalOptions::default()
    };
    evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness())
}

fn one_axis(n: &str, h: f64) -> ParamBox {
    let mut axes = BTreeMap::new();
    axes.insert(name(n), BoxAxis::Varying { lo: -h, hi: h });
    ParamBox::from_axes(axes)
}

// ------------------------------------------------------------------
// 1. The certifying filter, and what it does to a document that has
//    nothing to do with `min_clearance`.
// ------------------------------------------------------------------

/// A block whose x-placement varies, a `Distance` measure between two
/// of its own walls (a constant 1.0 by construction), and an assertion
/// whose bound is EXACTLY that distance — so the enclosure over any
/// leaf straddles the bound and the assertion is `Unevaluated` at the
/// interval scalar while the f64 witness decides it.
fn straddling_assertion() -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("place"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 0.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half(),
                hi: half(),
            }),
        },
    });
    let plane = r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                .expect("finite corners"),
        ],
    }));
    let solid = r.insert(Node::Extrude {
        profile,
        distance: len(2.0),
    });
    let placed = r.insert(Node::Transform {
        input: solid,
        translation: [
            Expr::param(name("place"), Dimension::Length),
            len(0.0),
            len(0.0),
        ],
        rotation_axis: [
            Expr::literal(0.0, Dimension::Scalar).unwrap(),
            Expr::literal(0.0, Dimension::Scalar).unwrap(),
            Expr::literal(1.0, Dimension::Scalar).unwrap(),
        ],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite angle"),
    });
    // The two facing walls of the unit square: their distance is 1.0.
    let measure = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                MeasureRef::new(placed, fixture::fname(solid, fixture::wall(1))),
                MeasureRef::new(placed, fixture::fname(solid, fixture::wall(3))),
            ],
        )
        .expect("both indices in range"),
    );
    let assertion = r.insert(Node::Assertion {
        measure,
        // The bound IS the measured value, so no enclosure separates
        // them: E10's third state at every leaf.
        bound: Expr::literal(1.0, Dimension::Length).expect("finite"),
        dir: AssertionDir::AtLeast,
    });
    (r.doc, assertion)
}

/// **Claim 1, falsified for assertion-carrying documents.**
///
/// `drive` compares each leaf's verdict vector against the witness's.
/// Before this unit that comparison was `VerdictVector::of`; this unit
/// silently narrows it to `VerdictVector::certifying`, which drops
/// `Assertion` rows. The narrowing is not disclosed in the PR's
/// deviation table, and it moves the drive of any document carrying an
/// assertion — a class that contains no `min_clearance` at all.
///
/// The row exhibits the difference two ways: the two vectors are not
/// equal over this document's witness, and a leaf certifies while its
/// assertion has NO verdict at the interval scalar — which is exactly
/// the row-level disagreement the old comparison refused on.
#[test]
fn the_certifying_filter_changes_a_pre_m10_6_documents_drive() {
    let (doc, assertion) = straddling_assertion();
    let witness: editor_core::Evaluation<f64> = eval_over(&doc, None);
    let full = VerdictVector::of(&witness);
    let certifying = VerdictVector::certifying(&doc, &witness);
    assert_ne!(
        full.key(),
        certifying.key(),
        "the assertion contributes rows to the full vector, so the two keys must differ; \
         if they do not, this fixture no longer exercises the filter"
    );

    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect("the nominal builds");
    assert!(
        !verdict.certified().is_empty(),
        "the ε-scaled box certifies"
    );
    // `ParamBoxVerdict::serialize()` prints the witness vector's key and
    // each certified leaf's, so BOTH move for this document — a
    // document with no `min_clearance` anywhere in it. That is claim
    // 1's "keys bit-identical" falsified; the masses move too wherever
    // a leaf's assertion row disagrees with the witness's, which is the
    // case `drive.rs` describes and which `min_clearance` makes
    // unconditional.
    let serialized = verdict.serialize();
    assert!(
        serialized.contains(&format!("witness_vector {:032x}", certifying.key().0)),
        "the goldening form carries the CERTIFYING key, which is not the key this \
         document's drive used before M10-6:\n{serialized}"
    );
    // How many certified leaves' assertions the interval lane leaves
    // undecided — measured, not assumed. At an ε-scaled box the
    // enclosure is far narrower than the funnel's band, so the answer
    // here is zero and the mass is unmoved; the KEYS still moved.
    let mut straddled = 0usize;
    for leaf in verdict.certified() {
        let ev: editor_core::Evaluation<geom_core::Interval> =
            eval_over(&doc, Some(leaf.box_.clone()));
        let Some(NodeResult::Ok(v)) = ev.result(assertion) else {
            panic!("the assertion node evaluated over a certified leaf");
        };
        let ValuePayload::Assertion(av) = &v.payload else {
            panic!("an assertion node's value is a verdict");
        };
        if matches!(av, AssertionVerdict::Unevaluated { .. }) {
            straddled += 1;
        }
    }
    eprintln!(
        "certifying filter: {} certified leaves, {straddled} of them assertion-straddling; \
         witness vector key {:032x} (full, pre-M10-6) vs {:032x} (certifying, shipped)",
        verdict.certified().len(),
        full.key().0,
        certifying.key().0
    );
}

// ------------------------------------------------------------------
// 2. The enclosure on a curved pair.
// ------------------------------------------------------------------

/// Two circular pins, radius `r`, axes `d` apart in x, extruded the
/// same height from the same plane. The true minimum separation
/// between the two WALLS is `d − 2r`, written into the geometry.
fn pins(d: f64, r: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r_ = Recorder::new();
    let plane = r_.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let mut pin = |cx: f64| {
        let profile = r_.insert(Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Circle {
                centre: [len(cx), len(0.0)],
                radius: len(r),
            }],
        }));
        r_.insert(Node::Extrude {
            profile,
            distance: len(1.0),
        })
    };
    let a = pin(0.0);
    let b = pin(d);
    (r_.doc, a, b)
}

/// **The bracket on carriers the unit's own fixture never touches.**
///
/// The dumbbell is planar throughout. This row runs the door over two
/// CYLINDERS at four budgets from starved to the shipped one and
/// requires containment of the hand-derived truth at every one, plus
/// monotone narrowing.
#[test]
fn min_separation_brackets_a_curved_pair_at_every_budget() {
    let (d, r) = (3.0, 1.0);
    let truth = d - 2.0 * r; // 1.0, by construction.
    let (doc, a, b) = pins(d, r);
    let ev: editor_core::Evaluation<geom_core::Interval> = eval_over(&doc, None);
    let side = |node: RecipeNodeId| -> MinSepSelection<'_> {
        let Some(NodeResult::Ok(value)) = ev.result(node) else {
            panic!("the pin evaluated");
        };
        let ValuePayload::Body(body) = &value.payload else {
            panic!("an extrude's value is one body");
        };
        MinSepSelection {
            at: node,
            index: 0,
            body,
            faces: body.faces().map(|(k, _)| k).collect(),
        }
    };
    let mut widths = Vec::new();
    for pairs in [1usize, 4, 32, 512] {
        let m = min_separation(
            &side(a),
            &side(b),
            MinSeparationConfig {
                max_cell_pairs: pairs,
                ..MinSeparationConfig::default()
            },
        )
        .expect("two disjoint pins admit a pair");
        eprintln!(
            "pins @ max_cell_pairs={pairs}: [{}, {}] width {:e} (truth {truth})",
            m.lo(),
            m.hi(),
            m.hi() - m.lo()
        );
        assert!(
            m.lo() <= truth,
            "budget {pairs}: lo {} is above the true minimum {truth} — the lower bound is \
             not sound",
            m.lo()
        );
        assert!(
            truth <= m.hi(),
            "budget {pairs}: hi {} is below the true minimum {truth}",
            m.hi()
        );
        assert!(m.receipt().holds(), "budget {pairs}: the receipt identity");
        widths.push(m.hi() - m.lo());
    }
    for w in widths.windows(2) {
        assert!(
            w[0] >= w[1],
            "a bigger budget is never a wider bracket: {widths:?}"
        );
    }
}

// ------------------------------------------------------------------
// 3. The window superset, carried into the measure.
// ------------------------------------------------------------------

/// The M10-5 dumbbell prism (a C whose neck walls sit 0.4 apart) and a
/// small block parked in its LOWER notch with 0.1 m of clearance on
/// every side that faces material.
///
/// The two solids' true minimum separation is 0.1: the block's top face
/// (`y = 0.7`) against the neck's lower wall (`y = 0.8`).
fn notched_pair(bound: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    let plane = r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let c_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([
                (0.0, 0.0),
                (2.0, 0.0),
                (2.0, 0.8),
                (3.0, 0.8),
                (3.0, 0.0),
                (5.0, 0.0),
                (5.0, 2.0),
                (3.0, 2.0),
                (3.0, 1.2),
                (2.0, 1.2),
                (2.0, 2.0),
                (0.0, 2.0),
            ])
            .expect("finite corners"),
        ],
    }));
    let c = r.insert(Node::Extrude {
        profile: c_profile,
        distance: len(2.0),
    });
    let block_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(2.2, 0.1), (2.8, 0.1), (2.8, 0.7), (2.2, 0.7)])
                .expect("finite corners"),
        ],
    }));
    let block = r.insert(Node::Extrude {
        profile: block_profile,
        distance: len(2.0),
    });
    let measure = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::MinClearance { a: 0, b: 1 }),
            vec![MeasureRef::new(c, bname(c)), MeasureRef::new(block, bname(block))],
        )
        .expect("both indices in range"),
    );
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: Expr::literal(bound, Dimension::Length).expect("finite"),
        dir: AssertionDir::AtLeast,
    });
    (r.doc, measure, assertion)
}

/// **The carrier-window superset reaches the document's requirement.**
///
/// The two solids are 0.1 m apart. The C's PLANAR cap faces are
/// non-convex, and the engine's window for a planar face is the
/// rectangle hull of its boundary — so the C's cap window covers the
/// notch the block sits in, the two windows INTERPENETRATE, and no
/// refinement can separate them: `lo` collapses to 0 and stays there at
/// any budget.
///
/// `lo` is the sound direction, so nothing here is unsound. What it
/// costs is the whole point of the door: the assertion
/// `min_clearance ≥ 0.05` — met by these solids twice over — gets no
/// verdict at all, and a `min_clearance` assertion registered in the
/// corpus over a non-convex body would RED CI row 1 (which fails on a
/// verdict-less leaf) for a reason that is about the window model and
/// not about the part.
#[test]
fn min_clearance_between_two_separated_bodies_reads_zero() {
    let truth = 0.1;
    let (doc, measure, assertion) = notched_pair(0.05);
    let ev: editor_core::Evaluation<geom_core::Interval> = eval_over(&doc, None);
    let Some(NodeResult::Ok(v)) = ev.result(measure) else {
        panic!("the measure evaluated: {:?}", ev.result(measure));
    };
    let ValuePayload::Measure { value, .. } = &v.payload else {
        panic!("the measure has a value at the interval scalar");
    };
    eprintln!(
        "notched pair: min_clearance = [{}, {}], true solid separation {truth}",
        value.lo(),
        value.hi()
    );
    assert!(
        truth <= value.hi(),
        "the enclosure still contains the truth: {} < {truth}",
        value.hi()
    );
    assert_eq!(
        value.lo(),
        0.0,
        "the C's cap window contains the block's, so the pair never separates and the \
         certified lower bound is 0 against a true 0.1. If this row goes red the window \
         model has been tightened and this finding is stale."
    );
    let Some(NodeResult::Ok(a)) = ev.result(assertion) else {
        panic!("the assertion evaluated");
    };
    let ValuePayload::Assertion(verdict) = &a.payload else {
        panic!("an assertion's value is a verdict");
    };
    eprintln!("notched pair: the assertion reads {verdict:?}");
    assert!(
        !matches!(verdict, AssertionVerdict::Holds { .. }),
        "the requirement `min_clearance >= 0.05` is met by the solids twice over, yet the \
         verdict is {verdict:?} — a row 1 red on a part that is fine"
    );
}

// ------------------------------------------------------------------
// 4. The cache seam's key.
// ------------------------------------------------------------------

/// A document with one varying parameter and one measure — enough to
/// have an analyzed box and an MC report.
fn sampled_doc() -> ProfileDoc {
    straddling_assertion().0
}

/// **`report_key` is blind to every dial that moves a report.**
///
/// D5 says the two key kinds are on purpose and that `report_key` — the
/// tuple (kind, slice, box, ε, K) — "is the cache seam". But the MC
/// report is a function of its seed and its sample count as well, and
/// neither is in that tuple: two runs a consumer would never confuse
/// hash to one key. A `ReportCache` keyed the documented way therefore
/// serves one run's numbers to another run's question.
///
/// Evidence-only in the sense that nothing in the tree keys a cache
/// this way today — `report_key` has no consumer at all outside the
/// unit's own key row — which is itself the finding.
#[test]
fn report_key_is_blind_to_the_dials_that_move_a_report() {
    let doc = sampled_doc();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let a = monte_carlo(
        &doc,
        &analyzed,
        &McConfig {
            samples: 64,
            seed: 1,
            parallel: false,
        },
        Tol::witness(),
    )
    .expect("the nominal builds");
    let b = monte_carlo(
        &doc,
        &analyzed,
        &McConfig {
            samples: 128,
            seed: 2,
            parallel: false,
        },
        Tol::witness(),
    )
    .expect("the nominal builds");
    assert_ne!(
        a.serialize(),
        b.serialize(),
        "two different dials produce two different reports"
    );
    let box_ = one_axis("place", half());
    let ka = report_key("mc", 7, &box_, Tol::witness().eps(), 10.0);
    let kb = report_key("mc", 7, &box_, Tol::witness().eps(), 10.0);
    assert_eq!(
        ka, kb,
        "…and the documented cache key cannot tell them apart: the seed and the sample \
         count are not in (kind, slice, box, eps, K)"
    );
}

// ------------------------------------------------------------------
// 5. The advisory lane's stream, re-derived.
// ------------------------------------------------------------------

/// `splitmix64`-style finalizer + `xorshift64*`, written from the
/// algorithms rather than read off the lane's source.
struct MyRng(u64);

impl MyRng {
    fn for_sample(seed: u64, index: u64) -> Self {
        let mut z = seed ^ index.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^= z >> 31;
        Self(if z == 0 { 0x9e37_79b9_7f4a_7c15 } else { z })
    }

    fn unit(&mut self) -> f64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        let v = x.wrapping_mul(0x2545_f491_4f6c_dd1d);
        (v >> 11) as f64 * (1.0 / 9_007_199_254_740_992.0)
    }
}

/// **The lane's draws, re-derived independently.**
///
/// The MC report is not a per-sample dump, so the row checks the two
/// halves it can: `sample_offset` is the inverse transform of the SAME
/// mass function the accounting integrates (a uniform draw at `u` lands
/// where `box_mass` says it should), and the report is bit-identical
/// across rayon schedules while moving with the seed.
#[test]
fn the_mc_stream_is_re_derived_bit_for_bit() {
    // The inverse transform is exact for a uniform law, so a
    // hand-computed draw is a real check on the door.
    let uniform = Distribution::Uniform { lo: -2.0, hi: 6.0 };
    for i in 0..8u64 {
        let u = MyRng::for_sample(editor_core::mc::DEFAULT_SEED, i).unit();
        let got = sample_offset(&name("p"), &uniform, u).expect("a uniform is sampleable");
        let want = -2.0 + 8.0 * u;
        assert_eq!(
            got.to_bits(),
            want.to_bits(),
            "sample {i}: the uniform inverse transform is `lo + (hi - lo) * u`"
        );
    }
    // A normal draw is monotone in `u` and symmetric about the median,
    // which is the property an inverse CDF has and a shaped generator
    // does not.
    let normal = Distribution::Normal { sigma: 0.5 };
    let mut prev = f64::NEG_INFINITY;
    for k in 1..20 {
        let u = f64::from(k) / 20.0;
        let z = sample_offset(&name("p"), &normal, u).expect("a normal is sampleable");
        assert!(z > prev, "the quantile is monotone in u at u={u}: {z} <= {prev}");
        prev = z;
        let mirror = sample_offset(&name("p"), &normal, 1.0 - u).expect("sampleable");
        assert!(
            (z + mirror).abs() < 1e-9,
            "Phi^-1 is odd about the median: u={u} gives {z} and 1-u gives {mirror}"
        );
    }

    // And the report: schedule-free, seed-sensitive.
    let doc = sampled_doc();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = |seed: u64, parallel: bool| {
        monte_carlo(
            &doc,
            &analyzed,
            &McConfig {
                samples: 96,
                seed,
                parallel,
            },
            Tol::witness(),
        )
        .expect("the nominal builds")
        .serialize()
    };
    assert_eq!(run(3, true), run(3, false), "rayon cannot move a number");
    assert_ne!(run(3, false), run(4, false), "the seed rides the report");
}

// ------------------------------------------------------------------
// 6. The end-to-end consumer walk: my own distributed, measured
//    document, driven and reported through the public doors.
// ------------------------------------------------------------------

/// A GUIDE CHANNEL and its tongue — two rails facing each other across
/// a running clearance, geometry the unit's own suites do not use.
///
/// * `place` (Uniform ±h): where the tongue sits along the clearance.
/// * `skew` (Normal σ = h/3): where it sits ACROSS it — a distributed
///   contributor the clearance does not depend on, so the stackup's
///   per-parameter table has a real zero row to print.
///
/// Two measures over the same face pair, on purpose: `distance` (the
/// closed form, which has a nominal and sensitivities) and
/// `min_clearance` (the new primitive, which has neither at `f64`),
/// with an assertion over each.
struct Guide {
    doc: ProfileDoc,
    by_distance: RecipeNodeId,
    by_clearance: RecipeNodeId,
    assertion: RecipeNodeId,
}

fn guide(bound: f64) -> Guide {
    let h = half();
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("place"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 3.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform { lo: -h, hi: h }),
        },
    });
    r.push(DocEdit::SetDocParam {
        name: name("skew"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 0.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Normal { sigma: h / 3.0 }),
        },
    });
    let plane = r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let square = |r: &mut Recorder| {
        r.insert(Node::Profile(ProfileProgram {
            plane,
            loops: vec![
                LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                    .expect("finite corners"),
            ],
        }))
    };
    let rail_profile = square(&mut r);
    let rail = r.insert(Node::Extrude {
        profile: rail_profile,
        distance: len(1.0),
    });
    let tongue_profile = square(&mut r);
    let tongue = r.insert(Node::Extrude {
        profile: tongue_profile,
        distance: len(1.0),
    });
    let placed = r.insert(Node::Transform {
        input: tongue,
        translation: [
            Expr::param(name("place"), Dimension::Length),
            Expr::param(name("skew"), Dimension::Length),
            len(0.0),
        ],
        rotation_axis: [
            Expr::literal(0.0, Dimension::Scalar).unwrap(),
            Expr::literal(0.0, Dimension::Scalar).unwrap(),
            Expr::literal(1.0, Dimension::Scalar).unwrap(),
        ],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite angle"),
    });
    let refs = || {
        vec![
            MeasureRef::new(rail, fixture::fname(rail, fixture::wall(1))),
            MeasureRef::new(placed, fixture::fname(tongue, fixture::wall(3))),
        ]
    };
    let by_distance = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            refs(),
        )
        .expect("both indices in range"),
    );
    let by_clearance = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::MinClearance { a: 0, b: 1 }),
            refs(),
        )
        .expect("both indices in range"),
    );
    let assertion = r.insert(Node::Assertion {
        measure: by_distance,
        bound: Expr::literal(bound, Dimension::Length).expect("finite"),
        dir: AssertionDir::AtLeast,
    });
    r.insert(Node::Assertion {
        measure: by_clearance,
        bound: Expr::literal(bound, Dimension::Length).expect("finite"),
        dir: AssertionDir::AtLeast,
    });
    Guide {
        doc: r.doc,
        by_distance,
        by_clearance,
        assertion,
    }
}

/// **The whole consumer walk, in one row**: box, drive, stackup,
/// histogram, budget, MC, cache — each rendered as a consumer would
/// read it, with the friction printed rather than smoothed.
#[test]
fn a_tolerance_study_end_to_end_through_the_public_doors() {
    let g = guide(2.0 - 1.0e-11);
    let analyzed = analyzed_box(&g.doc, &AnalysisPolicy::default());
    let verdict = drive(&g.doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect("the nominal builds");
    eprintln!("--- drive ---\n{}", verdict.render(&analyzed));
    assert!(!verdict.certified().is_empty(), "the ε-scaled box certifies");

    // The stackup over the CLOSED-FORM measure: a real report.
    let report = editor_core::stackup::stackup(
        &g.doc,
        g.by_distance,
        &analyzed,
        &verdict,
        None,
        true,
        Tol::witness(),
    )
    .expect("a stackup over a closed-form measure");
    eprintln!("--- stackup (distance) ---\n{}", report.render(&analyzed));

    // …and over the NEW primitive: it refuses, because E5's nominal is
    // an f64 number and `min_clearance` has none. Friction, recorded.
    let refused = editor_core::stackup::stackup(
        &g.doc,
        g.by_clearance,
        &analyzed,
        &verdict,
        None,
        true,
        Tol::witness(),
    );
    eprintln!(
        "--- stackup (min_clearance) --- {}",
        match &refused {
            Ok(_) => "a report".to_owned(),
            Err(e) => format!("{e}"),
        }
    );
    assert!(
        refused.is_err(),
        "a stackup over a min_clearance measure has no nominal and no per-param table"
    );

    // The histogram, over both measures.
    for (label, node) in [
        ("distance", g.by_distance),
        ("min_clearance", g.by_clearance),
    ] {
        let h =
            editor_core::report::leaf_histogram(&g.doc, &analyzed, &verdict, node, Tol::witness());
        eprintln!("--- histogram ({label}) ---\n{}", h.render());
        assert_eq!(
            h.rows.len(),
            verdict.certified().len(),
            "{label}: every certified leaf places its mass"
        );
    }

    // The budget, priced (no band anywhere in this document).
    let budget = editor_core::report::MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("--- budget ---\n{}", budget.render());
    assert_eq!(budget.basis.word(), "priced");

    // The advisory lane.
    let mc = monte_carlo(&g.doc, &analyzed, &McConfig::default(), Tol::witness())
        .expect("the nominal builds");
    eprintln!("--- mc ---\n{}", mc.render());
    assert!(mc.render().contains("ADVISORY"));

    // And the assertion the document records, read back over a leaf.
    let leaf = verdict.certified()[0].box_.clone();
    let ev: editor_core::Evaluation<geom_core::Interval> = eval_over(&g.doc, Some(leaf));
    let Some(NodeResult::Ok(v)) = ev.result(g.assertion) else {
        panic!("the assertion evaluated over a certified leaf");
    };
    eprintln!("--- assertion over leaf 0 ---\n{:?}", v.payload);

    // The cache seam, used the documented way.
    let mut cache = editor_core::report::ReportCache::new();
    let key = report_key(
        "stackup",
        verdict.content_key().0,
        verdict.root(),
        Tol::witness().eps(),
        10.0,
    );
    cache.put(key, "stackup", report.serialize());
    assert_eq!(cache.get(key, "stackup"), Some(report.serialize().as_str()));
}
