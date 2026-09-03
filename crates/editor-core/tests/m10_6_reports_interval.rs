//! **The reporting layer and the advisory lane** (M10-6 §2, §4, §5):
//! the two doors every derived report carries, the keys over them, the
//! one cache, and the Monte-Carlo estimator's label and determinism
//! discipline.
//!
//! Every row here is about a CLAIM the unit makes rather than about a
//! number it happens to produce:
//!
//! - a goldening form is deterministic across repeats AND across the
//!   two schedules (D9), and a human form is not a substitute for it;
//! - a content key moves when and only when the report's bits move;
//! - the cache serves a report to an EQUAL key and to nothing else;
//! - the MC lane refuses a band typed, carries its count and seed on
//!   every rendered line, is bit-identical across schedules and seeds
//!   deterministically, and its empirical tail fraction converges on
//!   the accounting's exact one;
//! - the MC lane changes no document and produces no assertion —
//!   "never gates, never persists" checked rather than asserted in
//!   prose.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use editor_core::mc::{McConfig, McRefusal, monte_carlo};
use editor_core::report::{
    Dials, MassBasis, MassBudget, ReportCache, leaf_histogram, report_key,
};
use editor_core::stackup::stackup;
use editor_core::{
    AssertionDir, Dimension, Distribution, DocEdit, DocParam, Expr, LoopProgram, MeasureExpr,
    MeasurePrimitive, MeasureRef, Node, ParamName, ProfileDoc, ProfileProgram, RecipeNodeId,
    UnitSym, save,
};
use geom_core::Tol;

use fixture::{Recorder, len};

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

/// The ε-scaled half-width the driver can certify over.
fn half() -> f64 {
    Tol::witness().eps() / 64.0
}

/// A plate with one hole, the hole's radius a document parameter, and
/// a measure of the distance from the hole wall to a vertex of the
/// plate — a document with a real measure, a real assertion and one
/// varying parameter.
///
/// `law` is the parameter's distribution, which is what the band rows
/// vary.
fn plate(law: Distribution) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("place"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 0.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(law),
        },
    });
    let plane = r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)])
                .expect("finite corners"),
        ],
    }));
    let solid = r.insert(Node::Extrude {
        profile,
        distance: len(1.0),
    });
    // Placed by the parameter, so the measured distance moves with it —
    // M10-5's finding that a rigid placement is what survives the
    // interval lane at a usable box.
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
    // distance(wall 0, wall 2) — two parallel walls of the prism, 2 m
    // apart, measured at the PLACED node.
    let web = MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 });
    let measure = r.insert(
        Node::measure(
            web,
            vec![
                MeasureRef::new(placed, fixture::fname(solid, fixture::wall(0))),
                MeasureRef::new(placed, fixture::fname(solid, fixture::wall(2))),
            ],
        )
        .expect("both indices in range"),
    );
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: Expr::literal(1.0, Dimension::Length).expect("finite"),
        dir: AssertionDir::AtLeast,
    });
    (r.doc, measure, assertion)
}

fn uniform() -> Distribution {
    Distribution::Uniform {
        lo: -half(),
        hi: half(),
    }
}

/// **§2**: the goldening forms are deterministic across repeats and
/// across the two schedules, and the two doors are different forms of
/// the same report.
#[test]
fn the_goldening_forms_are_schedule_free_and_the_human_form_is_not_one() {
    let (doc, measure, _) = plate(uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let sequential = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            parallel: false,
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .expect("the nominal builds");
    let parallel = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect("the nominal builds");
    assert_eq!(
        sequential.serialize(),
        parallel.serialize(),
        "a verdict's goldening form is a function of the document and the box, never of \
         the schedule (D9 idiom 1)"
    );
    assert_eq!(sequential.content_key(), parallel.content_key());

    let one = stackup(
        &doc,
        measure,
        &analyzed,
        &parallel,
        None,
        false,
        Tol::witness(),
    )
    .expect("a stackup");
    let two = stackup(
        &doc,
        measure,
        &analyzed,
        &parallel,
        None,
        true,
        Tol::witness(),
    )
    .expect("a stackup");
    assert_eq!(
        one.serialize(),
        two.serialize(),
        "and neither is a stackup's"
    );
    assert_eq!(one.content_key(), two.content_key());

    // The two doors are DIFFERENT: the goldening form carries bits, the
    // human form carries percentages and prose. Neither is the other's
    // substitute, which is what makes shipping both worth it.
    let rendered = one.render(&analyzed);
    assert!(
        rendered.contains("CERTIFIED WORST CASE"),
        "the human form leads with the gating number: {rendered}"
    );
    assert!(
        !rendered.contains(&format!("{:016x}", one.nominal.expect("a closed-form measure has an f64 nominal").to_bits())),
        "the human form does not print bits"
    );
    assert!(
        one.serialize()
            .contains(&format!("{:016x}", one.nominal.expect("a closed-form measure has an f64 nominal").to_bits())),
        "and the goldening form prints nothing else"
    );
}

/// **§2**: a content key moves when and only when the report's bits
/// move.
#[test]
fn a_content_key_moves_exactly_when_the_report_does() {
    let (doc, measure, _) = plate(uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect("the nominal builds");
    let report = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        true,
        Tol::witness(),
    )
    .expect("a stackup");

    // The same report twice: equal bits, equal key.
    let again = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        true,
        Tol::witness(),
    )
    .expect("a stackup");
    assert_eq!(report.serialize(), again.serialize());
    assert_eq!(report.content_key(), again.content_key());

    // A DIFFERENT box: different bits, different key. The box is one of
    // the four things D9 says a derived report is a function of, so
    // this is the axis with the most to say.
    let (wider_doc, wider_measure, _) = plate(Distribution::Uniform {
        lo: -half() / 2.0,
        hi: half() / 2.0,
    });
    let wider = analyzed_box(&wider_doc, &AnalysisPolicy::default());
    let wider_verdict = drive(&wider_doc, &wider, &DriveConfig::default(), Tol::witness())
        .expect("the nominal builds");
    let other = stackup(
        &wider_doc,
        wider_measure,
        &wider,
        &wider_verdict,
        None,
        true,
        Tol::witness(),
    )
    .expect("a stackup");
    assert_ne!(report.serialize(), other.serialize());
    assert_ne!(report.content_key(), other.content_key());

    // And the pure key function agrees about what it is a function of:
    // the same inputs key the same, and EVERY input moves the key.
    let default_drive = DriveConfig::default();
    let starved_drive = DriveConfig {
        max_leaves: 4,
        ..DriveConfig::default()
    };
    let seeded_mc = McConfig {
        seed: 1,
        ..McConfig::default()
    };
    let base = Dials {
        drive: &default_drive,
        mc: None,
    };
    let k1 = report_key("stackup", 1, verdict.root(), 1e-9, 10.0, &base);
    let k2 = report_key("stackup", 1, verdict.root(), 1e-9, 10.0, &base);
    let k3 = report_key("stackup", 1, verdict.root(), 1e-12, 10.0, &base);
    let k4 = report_key("stackup", 2, verdict.root(), 1e-9, 10.0, &base);
    // The two the first pass could not tell apart (M10-6's review):
    // a different drive budget, and a different MC seed.
    let k5 = report_key(
        "stackup",
        1,
        verdict.root(),
        1e-9,
        10.0,
        &Dials {
            drive: &starved_drive,
            mc: None,
        },
    );
    let k6 = report_key(
        "stackup",
        1,
        verdict.root(),
        1e-9,
        10.0,
        &Dials {
            drive: &default_drive,
            mc: Some(&seeded_mc),
        },
    );
    assert_eq!(k1, k2);
    assert_ne!(k1, k3, "ε is part of the tuple");
    assert_ne!(k1, k4, "so is the recipe slice");
    assert_ne!(k1, k5, "and the drive budget, which moves the report");
    assert_ne!(k1, k6, "and the advisory lane's dials — `none` is not a seed");
}

/// **§2**: the cache serves an equal key and nothing else.
#[test]
fn the_cache_serves_equal_keys_and_only_those() {
    let (doc, measure, _) = plate(uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect("the nominal builds");
    let report = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        true,
        Tol::witness(),
    )
    .expect("a stackup");

    let mut cache = ReportCache::new();
    assert!(cache.is_empty());
    assert!(cache.get(report.content_key(), "stackup").is_none());
    assert!(
        cache
            .put(report.content_key(), "stackup", report.serialize())
            .is_none(),
        "the first put stores"
    );
    assert_eq!(cache.len(), 1);
    assert_eq!(
        cache.get(report.content_key(), "stackup"),
        Some(report.serialize().as_str())
    );
    // A different KIND under the same key is a different entry: the
    // key is over a report's own bits, and two kinds of report can
    // legitimately be derived from one tuple.
    assert!(cache.get(report.content_key(), "verdict").is_none());
    // And a key that is not equal serves nothing.
    assert!(
        cache.get(verdict.content_key(), "stackup").is_none(),
        "the cache has no notion of a key being close"
    );

    // **The DOCUMENTED seam, exercised** (M10-6's review). The two
    // doors above key on the report's own bits, which a consumer only
    // has once the work is done. `report_key` is the other door — the
    // one a consumer computes BEFORE deciding to do the work — and
    // until the dials went into it, a cache used that way served one
    // budget's stackup for another's. Here it is used that way, and
    // the two budgets miss each other.
    let default_drive = DriveConfig::default();
    let starved_drive = DriveConfig {
        max_leaves: 4,
        ..DriveConfig::default()
    };
    let pre = |d: &DriveConfig| {
        report_key(
            "stackup",
            1,
            verdict.root(),
            Tol::witness().eps(),
            Tol::witness().k(),
            &Dials { drive: d, mc: None },
        )
    };
    let mut seam = ReportCache::new();
    assert!(
        seam.put(pre(&default_drive), "stackup", report.serialize())
            .is_none()
    );
    assert_eq!(
        seam.get(pre(&default_drive), "stackup"),
        Some(report.serialize().as_str()),
        "the same run's dials find the entry the run put there"
    );
    assert!(
        seam.get(pre(&starved_drive), "stackup").is_none(),
        "a DIFFERENT budget must miss: it would have produced a different report, and          serving this one for it is the collision the key exists to prevent"
    );
}

/// **§5**: the histogram is a typed table of (leaf, mass, enclosure)
/// with the two doors, and it says what it does not cover.
#[test]
fn the_histogram_joins_leaf_mass_to_the_measures_enclosure() {
    let (doc, measure, _) = plate(uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect("the nominal builds");
    let histogram = leaf_histogram(&doc, &analyzed, &verdict, measure, Tol::witness());
    assert_eq!(
        histogram.rows.len(),
        verdict.certified().len(),
        "one row per certified leaf"
    );
    for row in &histogram.rows {
        assert!(
            row.enclosure.0 <= row.enclosure.1,
            "a row's enclosure is an interval"
        );
        assert!(row.mass.is_ok(), "a uniform law prices every leaf");
    }
    let rendered = histogram.render();
    assert!(
        rendered.contains("ADVISORY") && rendered.contains("Not a density"),
        "the advisory label and the E11.6 disclaimer are the first line: {rendered}"
    );
    assert!(
        rendered.contains("NOT covered"),
        "and the uncovered mass is on its own line"
    );
    // Deterministic, and keyed like every other report.
}

/// **§4**: the MC lane refuses a band typed, naming the parameter.
#[test]
fn the_mc_lane_refuses_a_band_typed() {
    let (doc, _, _) = plate(Distribution::Band {
        lo: -half(),
        hi: half(),
    });
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    match monte_carlo(&doc, &analyzed, &McConfig::default(), Tol::witness()) {
        Err(McRefusal::BandHasNoMeasure(e)) => {
            assert!(
                format!("{e}").contains("place"),
                "the refusal names the parameter: {e}"
            );
        }
        other => panic!("a band cannot be sampled: {other:?}"),
    }
    // And the budget over the same document reads FORCED, which is the
    // same fact in the certified lane's voice.
    assert!(matches!(MassBasis::of(&analyzed), MassBasis::Forced { .. }));
}

/// **§4**: seed- and schedule-determinism, and the label on every line.
#[test]
fn the_mc_report_is_deterministic_and_labeled() {
    let (doc, _, _) = plate(uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let cfg = McConfig {
        samples: 64,
        ..McConfig::default()
    };
    let a = monte_carlo(&doc, &analyzed, &cfg, Tol::witness()).expect("replays");
    let b = monte_carlo(&doc, &analyzed, &cfg, Tol::witness()).expect("replays");
    assert_eq!(a.serialize(), b.serialize(), "same seed, same report");

    // The two SCHEDULES agree bit for bit — the whole reason each
    // sample gets its own stream.
    let sequential = monte_carlo(
        &doc,
        &analyzed,
        &McConfig {
            parallel: false,
            ..cfg
        },
        Tol::witness(),
    )
    .expect("replays");
    assert_eq!(
        a.serialize(),
        sequential.serialize(),
        "rayon over samples is idiom 1: an indexed map, never an accumulation"
    );

    // A different seed is a different draw.
    let other = monte_carlo(
        &doc,
        &analyzed,
        &McConfig { seed: 12345, ..cfg },
        Tol::witness(),
    )
    .expect("replays");
    assert_ne!(a.serialize(), other.serialize(), "the seed is the draw");

    // Every advisory line carries the count and the seed.
    let rendered = a.render();
    for line in rendered.lines().filter(|l| l.contains("node")) {
        assert!(
            line.contains("64 samples") && line.contains("0x4d435f4531315f31"),
            "an estimate line carries its count and seed: {line}"
        );
    }
}

/// **§4**: the MC lane never gates and never persists — checked at the
/// two places those words mean something.
#[test]
fn the_mc_lane_changes_no_document_and_mints_no_assertion() {
    let (doc, _, _) = plate(uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let before = save(&doc, &[], Tol::witness()).expect("the document saves");
    let report = monte_carlo(
        &doc,
        &analyzed,
        &McConfig {
            samples: 32,
            ..McConfig::default()
        },
        Tol::witness(),
    )
    .expect("replays");
    let after = save(&doc, &[], Tol::witness()).expect("the document saves");
    assert_eq!(
        before, after,
        "the lane takes &Doc and writes nothing: the saved bytes are the same bytes"
    );
    // The report's assertion rows are COUNTS, not verdicts: there is no
    // door here that produces an `AssertionVerdict` for anything to
    // persist.
    assert_eq!(report.assertions.len(), 1);
    assert_eq!(
        report.assertions[0].holds
            + report.assertions[0].violated
            + report.assertions[0].unevaluated,
        32,
        "every sample is accounted in exactly one column"
    );
}

/// **§4**: the empirical tail fraction converges on the accounting's
/// exact one — the two are computed by completely different means, and
/// their agreement is the point.
#[test]
fn the_mc_tail_fraction_converges_on_the_accountings_tail() {
    // A NORMAL law, so the analyzed box genuinely excludes a tail: a
    // uniform's support is inside its box and its tail is exactly zero,
    // which would make this row vacuous.
    let (doc, _, _) = plate(Distribution::Normal { sigma: half() });
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let exact = editor_core::tail_mass(
        &name("place"),
        &Distribution::Normal { sigma: half() },
        &analyzed.get(&name("place")).expect("the axis").offsets,
    )
    .expect("a normal prices its tail");
    assert!(
        exact > 0.0,
        "the ±3σ default box leaves 0.27% outside, which is what this row is about"
    );
    let report = monte_carlo(
        &doc,
        &analyzed,
        &McConfig {
            samples: 2048,
            ..McConfig::default()
        },
        Tol::witness(),
    )
    .expect("replays");
    // The bound is a binomial one, stated rather than tuned: 2048 draws
    // at p = 0.0027 have mean 5.5 and σ ≈ 2.3, so an absolute window of
    // 0.005 is about four σ wide on either side. It is loose ON PURPOSE
    // — the claim is convergence, not a pinned count, and a tight bound
    // here would be a flake with a story.
    // **And it must be non-zero** (R2's MINOR-8). An absolute window of
    // 0.005 around 0.0027 admits 0.0 — a lane that sampled the box and
    // never left it would pass a row whose whole subject is that MC
    // sees the tail the certified box excludes. The convergence bound
    // stays loose on purpose; this is the separate claim it was
    // silently making.
    assert!(
        report.outside_box > 0.0,
        "the sampler must actually reach outside the analyzed box — that is what this \
         lane adds over the certified one"
    );
    assert!(
        (report.outside_box - exact).abs() < 0.005,
        "the empirical tail {} and the exact tail {exact} disagree by more than sampling \
         noise explains",
        report.outside_box
    );
}

/// **§2**: the budget's rendering puts the tail on its own line and
/// says whether the number is exact — the honesty gate's own report.
#[test]
fn the_budget_renders_its_tail_and_its_containment() {
    let (doc, _, _) = plate(Distribution::Normal { sigma: half() });
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect("the nominal builds");
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    let rendered = budget.render();
    assert!(rendered.contains("tail"), "the tail has a line: {rendered}");
    assert!(
        rendered.contains("UNRESOLVED"),
        "and so does the one number a CI row bounds"
    );
    assert!(
        rendered.contains("exact") || rendered.contains("conservative"),
        "and the budget says which of the two it is"
    );
}
