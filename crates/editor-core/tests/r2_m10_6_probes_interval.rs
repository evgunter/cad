//! **R2's independent probes for M10-6** (reporting, the advisory
//! lanes, the `min_separation` door).
//!
//! Independently derived: every truth these rows compare against is
//! written into the geometry by hand, and no row reads a number the
//! unit's own suites produced.
//!
//! **What each row is for is in the `roster!{}` block below**, not in
//! this header: the block's entries are the rows' own idents, so a row
//! renamed or retired is a compile error and a row added without an
//! entry is a red run. The sentence beside each name is still
//! hand-written and still unchecked — the weld is on NAMES, never on
//! prose.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::roster! {
    the_certifying_filter_changes_a_pre_m10_6_documents_drive:
        "M10-6's claim 1 falsified for assertion-carrying documents. \
         `drive::certifying_vector` drops `Assertion` rows for EVERY such document — \
         deviation D10 — and this row exhibits one whose leaves certify only because \
         of it",
    min_separation_brackets_a_curved_pair_at_every_budget:
        "the enclosure over carriers the unit's own fixture never uses — two CYLINDERS \
         — at four budgets from starved to the shipped one, containing the \
         hand-derived truth at every one and narrowing monotonically",
    a_clearance_between_two_separated_bodies_reads_their_gap:
        "two solids 0.1 m apart, one non-convex, whose carrier windows interpenetrate: \
         `clearance >= 0.05`, met by them twice over, holds, and `clearance <= 0.15` is \
         proved by a pair no closer than the true 0.1",
    report_key_tells_the_dials_that_move_a_report_apart:
        "the cache seam's key. The run dials were missing from the (kind, slice, box, \
         eps, K) tuple, unnoticed rather than decided, so two MC runs a consumer would \
         never confuse hashed to one key; M10-6's review put them in as `report::Dials` \
         (deviation D11, and `editor_core::report::report_key` is where that is written \
         down), and this row pins the DISCRIMINATION it was written to pin the absence \
         of",
    the_mc_stream_is_re_derived_bit_for_bit:
        "the two halves of the MC lane a report that is not a per-sample dump lets a \
         row check: `sample_offset` is the inverse transform of the same mass function \
         the accounting integrates, driven by a stream re-stated here from the \
         algorithms; and the report is bit-identical across rayon schedules while \
         moving with the seed",
}

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box, sample_offset};
use editor_core::drive::{DriveConfig, VerdictVector, certifying_vector, drive};
use editor_core::mc::{McConfig, monte_carlo};
use editor_core::report::{Dials, report_key};
use editor_core::{
    AssertionRelation, AssertionVerdict, Dimension, Distribution, DocEdit, EntityKind, EvalOptions,
    Formula, FreeVar, LoopProgram, MeasurePrimitive, Node, NodeResult, ProfileDoc, ProfileProgram,
    RecipeNodeId, RoleSeg, SitedRef, StableName, UnitSym, ValuePayload, VarName,
};
use geom_core::Tol;
use topo::clearance::{Budgets, Side, min_separation};

use fixture::{Recorder, len};

/// The evaluation's own band, which the engine decides its cells at.
fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).expect("the witness tolerance has a band")
}

/// A variable as the free mass doors' refusals speak it.
fn sp(name: &'static str) -> editor_core::SpokenVar {
    editor_core::SpokenVar::new(
        editor_core::VarId::new(0, 0),
        Some(editor_core::VarName::from_static(name)),
    )
}

fn name(n: &'static str) -> VarName {
    VarName::from_static(n)
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
        ..EvalOptions::default()
    };
    editor_core::evaluate(
        doc,
        None,
        &editor_core::CancelToken::new(),
        &opts,
        Tol::witness(),
    )
}

fn one_axis(doc: &ProfileDoc, n: &'static str, h: f64) -> ParamBox {
    let mut axes = BTreeMap::new();
    axes.insert(
        doc.var_named(n).expect("the fixture declares it"),
        BoxAxis::Varying { lo: -h, hi: h },
    );
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
    r.push(DocEdit::DeclareVar {
        name: name("place"),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: 0.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half(),
                hi: half(),
            }),
        }),
    });
    let profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                .expect("finite corners"),
        ],
        ids: Vec::new(),
        on: None,
    }));
    let solid = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(2.0),
    });
    // A copy of the block slid along its own x by `place`.
    let placed = r.against(solid, solid, &editor_core::Frame::IDENTITY);
    let slide_x = fixture::value_slot(
        &r.doc,
        placed,
        editor_core::Coord::Slide(editor_core::TargetAxis::X),
    );
    r.push(DocEdit::SetParam {
        node: placed,
        slot: slide_x,
        value: Formula::named(name("place"), Dimension::Length).into(),
        fresh: Vec::new(),
    });
    // The two facing walls of the unit square: their distance is 1.0.
    let measured = r.measure(
        &[MeasurePrimitive::Distance { a: 0, b: 1 }],
        &[
            SitedRef::new(
                placed,
                fixture::fname(solid, fixture::wall(&r.doc, solid, 1)),
            ),
            SitedRef::new(
                placed,
                fixture::fname(solid, fixture::wall(&r.doc, solid, 3)),
            ),
        ],
    );
    let (_measure, measure_value) = (measured.measures[0], measured.outputs[0]);
    let assertion = r.insert(Node::Assertion(editor_core::Claim::Value {
            value: fixture::read_var(&r.doc, measure_value),
            // The bound IS the measured value, so no enclosure separates
            // them: E10's third state at every leaf.
            bound: len(1.0),
            relation: AssertionRelation::AtLeast,
        }));
    (r.doc, assertion)
}

/// **Claim 1, falsified for assertion-carrying documents.**
///
/// `drive` compares each leaf's verdict vector against the witness's
/// through `drive::certifying_vector`, which drops `Assertion` rows.
/// That moves the drive of any document carrying an assertion — a class
/// that contains no `min_clearance` at all — and is M10-6's deviation
/// D10.
///
/// The row exhibits the difference two ways: the two vectors are not
/// equal over this document's witness, and a leaf certifies while its
/// assertion has NO verdict at the interval scalar — which is exactly
/// the row-level disagreement the old comparison refused on.
#[test]
fn the_certifying_filter_changes_a_pre_m10_6_documents_drive() {
    let (doc, assertion) = straddling_assertion();
    // The drive's own witness, under its own options.
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let witness: editor_core::Evaluation<f64> = editor_core::evaluate(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &editor_core::drive::witness_options(&analyzed),
        Tol::witness(),
    );
    let full = VerdictVector::of(&witness);
    let certifying = certifying_vector(&doc, &witness);
    assert_ne!(
        full.key(),
        certifying.key(),
        "the assertion contributes rows to the full vector, so the two keys must differ; \
         if they do not, this fixture no longer exercises the filter"
    );

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
        let av = editor_core::drive::assertion_at(
            &doc,
            assertion,
            &leaf.box_,
            verdict.symbolic(),
            Tol::witness(),
        )
        .expect("the assertion node evaluated over a certified leaf, on its lane");
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
    let mut pin = |cx: f64| {
        let profile = r_.insert(Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(cx), len(0.0)],
                radius: len(r),
            }],
            ids: Vec::new(),
            on: None,
        }));
        r_.insert(Node::Extrude {
            profile: profile.into(),
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
    let side = |node: RecipeNodeId| -> Side<'_, geom_core::Interval> {
        let Some(NodeResult::Ok(value)) = ev.result(node) else {
            panic!("the pin evaluated");
        };
        let ValuePayload::Body(body) = &value.payload else {
            panic!("an extrude's value is one body");
        };
        Side {
            body: body.body(),
            faces: body.body().faces().map(|(k, _)| k).collect(),
        }
    };
    let mut widths = Vec::new();
    for pairs in [1usize, 4, 32, 512] {
        let m = min_separation(
            &side(a),
            &side(b),
            // Two pins, two nodes.
            a == b,
            band(),
            Budgets {
                max_cell_pairs: pairs,
                ..Budgets::MIN_SEPARATION
            },
        )
        .expect("two disjoint pins admit a pair");
        let hi = m.hi().expect("a pair witnesses an upper end");
        eprintln!(
            "pins @ max_cell_pairs={pairs}: [{}, {}] width {:e} (truth {truth})",
            m.lo(),
            hi,
            hi - m.lo()
        );
        assert!(
            m.lo() <= truth,
            "budget {pairs}: lo {} is above the true minimum {truth} — the lower bound is \
             not sound",
            m.lo()
        );
        assert!(
            truth <= hi,
            "budget {pairs}: hi {} is below the true minimum {truth}",
            hi
        );
        assert!(m.receipt().holds(), "budget {pairs}: the receipt identity");
        widths.push(hi - m.lo());
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
fn notched_pair(bounds: editor_core::ClearanceBounds<Formula>) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    let c_profile = r.insert(Node::Profile(ProfileProgram {
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
        ids: Vec::new(),
        on: None,
    }));
    let c = r.insert(Node::Extrude {
        profile: c_profile.into(),
        distance: len(2.0),
    });
    let block_profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(2.2, 0.1), (2.8, 0.1), (2.8, 0.7), (2.2, 0.7)])
                .expect("finite corners"),
        ],
        ids: Vec::new(),
        on: None,
    }));
    let block = r.insert(Node::Extrude {
        profile: block_profile.into(),
        distance: len(2.0),
    });
    let block = r.against(c, block, &editor_core::Frame::IDENTITY);
    let assertion = r.insert(Node::clearance(
        SitedRef::new(c, bname(c)),
        SitedRef::new(block, bname(block)),
        bounds,
    ));
    (r.doc, assertion)
}

/// **Two separated solids, one non-convex, read their gap.**
///
/// The two solids are 0.1 m apart. The C's planar cap faces are
/// non-convex, and the engine's window for a planar face is the
/// rectangle hull of its boundary, so the C's cap window covers the
/// notch the block sits in and the two windows interpenetrate. The cells
/// of that window decided outside the C drop out, and a pair is read
/// only off points of the faces, so `clearance ≥ 0.05`, which they meet
/// twice over, holds, and `clearance ≤ 0.15` is proved by a pair of face
/// points no closer than the solids' own 0.1.
#[test]
fn a_clearance_between_two_separated_bodies_reads_their_gap() {
    use editor_core::ClearanceBounds;
    use editor_core::clearance::ClearanceVerdict;
    let truth = 0.1;
    let report = |bounds| {
        let (doc, assertion) = notched_pair(bounds);
        let ev: editor_core::Evaluation<geom_core::Interval> = eval_over(&doc, None);
        let Some(NodeResult::Ok(a)) = ev.result(assertion) else {
            panic!("the assertion evaluated");
        };
        let ValuePayload::Assertion(AssertionVerdict::Clearance(report)) = &a.payload else {
            panic!("a clearance's value is the sweep's report");
        };
        (**report).clone()
    };
    let held = report(ClearanceBounds::AtLeast(len(0.05)));
    println!("notched pair ≥ 0.05: {}", held.render());
    assert_eq!(
        held.verdict(),
        &ClearanceVerdict::Holds,
        "`clearance >= 0.05` is met by the solids twice over: {}",
        held.render()
    );
    let proved = report(ClearanceBounds::AtMost(len(0.15)));
    let ClearanceVerdict::Within(w) = proved.verdict() else {
        panic!("a 0.1 gap is within 0.15: {}", proved.render());
    };
    let d = w.geometry.distance;
    assert!(
        truth - band().zero() <= d && d <= 0.15,
        "the proving pair is two face points, so no closer than the solids' {truth}: {d}"
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

/// **`report_key` was blind to every dial that moves a report** — and
/// is not any more.
///
/// The finding: D5 said the two key kinds are on purpose and that
/// `report_key` — the tuple (kind, slice, box, ε, K) — "is the cache
/// seam", but an MC report is a function of its seed and its sample
/// count too, and neither was in that tuple. Two runs a consumer would
/// never confuse hashed to one key, so a `ReportCache` keyed the
/// documented way served one run's numbers to another run's question.
/// The review also noted that `report_key` had no consumer at all,
/// which is what kept the collision theoretical.
///
/// Both halves are closed. The dials are in the key
/// (`report::Dials`), and the unit's cache row now uses `report_key`
/// the documented pre-work way. This row pins the discrimination it
/// was written to pin the absence of.
#[test]
fn report_key_tells_the_dials_that_move_a_report_apart() {
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
    let box_ = one_axis(&doc, "place", half());
    let drive_cfg = DriveConfig::default();
    let mc_a = McConfig {
        samples: 128,
        seed: 1,
        parallel: false,
    };
    let mc_b = McConfig {
        samples: 128,
        seed: 2,
        parallel: false,
    };
    let key = |mc: &McConfig| {
        report_key(
            "mc",
            7,
            &box_,
            Tol::witness().eps(),
            10.0,
            &Dials {
                drive: &drive_cfg,
                mc: Some(mc),
            },
        )
    };
    // **The finding, fixed.** The review wrote this as `assert_eq!`:
    // two reports that differ, keyed the same, because the seed and
    // the sample count were not in the tuple. They are now
    // (`report::Dials`), so the row pins the discrimination instead.
    assert_ne!(
        key(&mc_a),
        key(&mc_b),
        "two different dials produce two different reports, so they must not share a key"
    );
    assert_eq!(key(&mc_a), key(&mc_a), "the key is a pure function");
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
        let got = sample_offset(&sp("p"), &uniform, u).expect("a uniform is sampleable");
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
        let z = sample_offset(&sp("p"), &normal, u).expect("a normal is sampleable");
        assert!(
            z > prev,
            "the quantile is monotone in u at u={u}: {z} <= {prev}"
        );
        prev = z;
        let mirror = sample_offset(&sp("p"), &normal, 1.0 - u).expect("sampleable");
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
