//! **R1's adversarial probes for M10-6** (PR #1685 at the frozen head
//! `bf67a734`). Independent derivations, written against the unit's
//! claims rather than against its own fixtures.
//!
//! Row shapes, per `memories/test-suite-cost.md`: every row is a
//! written-down witness (a static fixture) — no sampling, no seeds.
//! Rows whose subject was a finding red at that head arrived
//! `#[ignore]`d with the finding named. **The fix pass un-ignored
//! them**: they are PINS now, and each one reds again if the fix is
//! undone. Rows marked EVIDENCE-ONLY print and assert only the shape
//! (they are not gates).
//!
//! What changed under them, for a reader who has the review beside
//! this file: the enclosure's two ends are now typed about different
//! sets (`measure::Certified`), so the two assertion arms that would
//! read the carrier's end refuse `Unevaluated { WindowSuperset }`
//! instead of answering. Rows 1 and 3-4 below therefore pass by
//! REFUSING rather than by bracketing — which is the whole content of
//! the fix, and row 2 keeps the looseness itself visible.
//!
//! What the rows attack, in order:
//!
//! 1. `min_clearance`'s enclosure over TRIMMED faces: the engine
//!    subdivides carrier WINDOWS (bounding rectangles), so a non-convex
//!    face's window covers material the face does not have. The
//!    `MinSeparation` docs claim a containment-true enclosure "so an
//!    `AtMost` assertion is sound"; the L-shaped cap below shows the
//!    bracket EXCLUDING the true separation and an `AtLeast` assertion
//!    over it reading a certified `Violated` on a pair that holds.
//! 2. The unary elaboration (D8): `min_clearance(body, body)` on one
//!    body is the selection's self-clearance.
//! 3. `VerdictVector::certifying` moves the witness-vector key of every
//!    assertion-carrying document (claim 1's "bit-identical keys" for
//!    documents without `min_clearance`).
//! 4. `report_key` omits the drive/MC config, so two different reports
//!    over one (slice, box, ε, K) tuple share a key.
//! 5. Row 1's three reds, as rows: a planted `Violated`, a planted
//!    engine refusal (priced as refused mass), a budget overrun.
//! 6. The mixed priced/forced case and a band axis that splits.
//! 7. A caption check: an enclosure that STRADDLES the bound is
//!    `Unevaluated` at the node while `worst_case.lo < bound` — the
//!    tour's "FAILS somewhere in the box" reading.
//! 8. The MC lane over a `min_clearance` document.
//! 9. An end-to-end consumer walk on a bracket (post over base), through
//!    the public doors: drive, stackup, fold, histogram, MC, budget.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::clearance::{
    ClearanceQuery, MinSepSelection, MinSeparationConfig, Selection, clearance_over, min_separation,
};
use editor_core::drive::{DriveConfig, SymbolicDials, VerdictVector, drive};
use editor_core::mc::{McConfig, monte_carlo};
use editor_core::report::{Dials, MassBasis, MassBudget, leaf_histogram, report_key};
use editor_core::stackup::stackup;
use editor_core::{
    AssertionDir, AssertionVerdict, CancelToken, CapEnd, Dimension, Distribution, DocEdit,
    DocParam, EvalOptions, Expr, LoopProgram, MeasureExpr, MeasurePrimitive, Node, NodeResult,
    ParamName, ProfileDoc, ProfileLift, ProfileProgram, RecipeNodeId, RoleSeg, SitedRef,
    UnevaluatedReason, UnitSym, ValuePayload, evaluate,
};
use geom_core::{Bounds, Tol};

use fixture::{Recorder, len};

/// The clearance engine has no lane at the symbolic identity tier
/// (ERROR-DESIGN E12; `DriveRefusal::SymbolicClearanceUnsupported`, and
/// the deviation is issue `symbolic-tier-and-clearance-engine`), so every drive over a `min_clearance`
/// document asks for the numeric-only replay BY NAME. It is a disclosed
/// limitation of the tier, not a property of these fixtures: with the
/// tier on the driver refuses this document up front rather than
/// certifying leaves whose clearance measure was never computed.
fn numeric_lane() -> DriveConfig {
    DriveConfig {
        symbolic: SymbolicDials::off(),
        ..DriveConfig::default()
    }
}

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

fn half() -> f64 {
    Tol::witness().eps() / 64.0
}

fn scalar(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite scalar")
}

fn eval_over<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    box_: Option<ParamBox>,
) -> editor_core::Evaluation<T> {
    let opts = EvalOptions {
        param_box: box_.map(std::sync::Arc::new),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness())
}

fn assertion_verdict<T: geom_core::Decide>(
    ev: &editor_core::Evaluation<T>,
    node: RecipeNodeId,
) -> AssertionVerdict<T> {
    match ev.result(node) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Assertion(a) => a.clone(),
            other => panic!("node {} is a {}", node.0, other.kind_name()),
        },
        other => panic!("assertion {} did not evaluate: {other:?}", node.0),
    }
}

fn measure_value<T: geom_core::Decide>(
    ev: &editor_core::Evaluation<T>,
    node: RecipeNodeId,
) -> Option<T> {
    match ev.result(node) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => Some(*value),
            _ => None,
        },
        _ => None,
    }
}

fn param(r: &mut Recorder, n: &str, value: f64, dist: Option<Distribution>) {
    r.push(DocEdit::SetDocParam {
        name: name(n),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: dist,
        },
    });
}

fn translate(r: &mut Recorder, input: RecipeNodeId, t: [Expr; 3]) -> RecipeNodeId {
    r.insert(Node::Transform {
        input,
        translation: t,
        rotation_axis: [scalar(0.0), scalar(0.0), scalar(1.0)],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite angle"),
    })
}

fn prism(r: &mut Recorder, origin: [f64; 3], corners: &[(f64, f64)], height: f64) -> RecipeNodeId {
    let plane = r.insert(fixture::frame(origin, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::polygon(corners.iter().copied()).expect("finite corners")],
    }));
    r.insert(Node::Extrude {
        profile,
        distance: len(height),
    })
}

// ----------------------------------------------------------- 1. windows

/// The gap between the block's underside and the L's top cap, and the
/// horizontal offset from the block to the L's material: the block sits
/// wholly over the NOTCH, so the nearest face point is diagonal.
const LIFT: f64 = 0.1;
const OFFSET: f64 = 0.25;

/// An L-shaped prism (notch at `x, y ∈ [1, 2]`) and a small block parked
/// over the notch at height `LIFT`. Returns the L's top cap and the
/// block's bottom cap as measure references, and the measure/assertion
/// nodes for `min_clearance(cap, underside) ≥ bound`.
fn notch(bound: f64, dir: AssertionDir) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    let ell = prism(
        &mut r,
        [0.0; 3],
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        1.0,
    );
    let block = prism(
        &mut r,
        [0.0, 0.0, 1.0 + LIFT],
        &[(1.25, 1.25), (1.75, 1.25), (1.75, 1.75), (1.25, 1.75)],
        0.5,
    );
    let measure = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::MinClearance { a: 0, b: 1 }),
            vec![
                SitedRef::at_mint(fixture::fname(ell, RoleSeg::Cap(CapEnd::End))),
                SitedRef::at_mint(fixture::fname(block, RoleSeg::Cap(CapEnd::Start))),
            ],
        )
        .expect("both indices in range"),
    );
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(bound),
        dir,
    });
    (r.doc, measure, assertion)
}

/// The true minimum distance between the L's top cap (an L-shaped
/// region at `z = 1`) and the block's underside (a square over the
/// notch at `z = 1 + LIFT`): the block's nearest corner to the L's
/// inner corner is `OFFSET` away horizontally.
fn true_notch_clearance() -> f64 {
    (OFFSET * OFFSET + LIFT * LIFT).sqrt()
}

/// **The finding, and the shape the fix gave it.** The L cap's carrier
/// window is its bounding square, which covers the notch, so the engine
/// finds a window pair straight under the block at distance `LIFT` and
/// reports it — BELOW the faces' real separation. The review wrote this
/// row as "the bracket must contain the truth", and that is not what
/// was fixed: closing it needs the trim boundary in chart coordinates
/// (`measure::WINDOW_TIGHTENING`), which is a tracker item, not this
/// unit.
///
/// What was fixed is the CLAIM. The docs said `[lo, hi]` encloses the
/// faces' separation and that this made an `AtMost` assertion sound;
/// both sentences were false. The bracket is now typed about two
/// different sets, and this row pins the half that is true and load-
/// bearing: **`lo` is a lower bound on the faces' own clearance**,
/// because the windows contain the faces and so their minimum is no
/// larger. That is the end every gate reads, and it survives any
/// amount of window slack.
///
/// The other half — `window_hi` bounding the faces from above — is
/// pinned FALSE by the row below, deliberately, so that tightening the
/// windows reds it and sends a reader back here.
#[test]
fn the_min_clearance_bracket_bounds_the_trimmed_faces_from_below() {
    let (doc, measure, _) = notch(0.2, AssertionDir::AtLeast);
    let ev = eval_over::<geom_core::Interval>(&doc, None);
    let value = measure_value(&ev, measure).expect("the measure has an interval value");
    let truth = true_notch_clearance();
    eprintln!(
        "notch: min_clearance windows ∈ [{}, {}], true face separation {truth}",
        value.lo(),
        value.hi()
    );
    assert!(
        value.lo() <= truth,
        "`lo` ({}) must bound the faces' true minimum separation {truth} from below — the \
         windows contain the faces, so their minimum cannot be the larger of the two",
        value.lo()
    );
}

/// The same fixture, read the way the review reads it: the bracket the
/// engine actually returns, printed, with its receipt. EVIDENCE-ONLY —
/// it asserts the SHAPE of the looseness (the window's `LIFT` inside
/// the bracket) so a fix that tightens windows turns it red and says
/// so, rather than letting the counterexample above silently pass.
#[test]
fn the_notch_bracket_is_the_windows_not_the_faces() {
    let (doc, measure, _) = notch(0.2, AssertionDir::AtLeast);
    let ev = eval_over::<geom_core::Interval>(&doc, None);
    let value = measure_value(&ev, measure).expect("the measure has an interval value");
    eprintln!(
        "EVIDENCE notch bracket [{}, {}] vs window distance {LIFT} vs face distance {}",
        value.lo(),
        value.hi(),
        true_notch_clearance()
    );
    assert!(
        value.hi() < true_notch_clearance(),
        "if this fails the window looseness is closed: un-ignore the row above"
    );
    // The f64 geometry puts the underside at `1.1 − 1.0`, one ulp-ish
    // above `0.1`, so the window's own distance is compared with slack.
    assert!(value.lo() <= LIFT + 1e-15 && LIFT <= value.hi());
}

/// **Finding (MAJOR, the consequence):** `min_clearance(cap, underside)
/// ≥ 0.2` HOLDS on the faces (they are 0.269 apart) and the assertion
/// reads a certified `Violated`, because the bracket's `hi` is the
/// window's 0.1. A CI row 1 over this document reds a true assertion.
#[test]
fn an_at_least_assertion_over_a_notch_does_not_read_violated_when_the_faces_clear_it() {
    let (doc, _, assertion) = notch(0.2, AssertionDir::AtLeast);
    let ev = eval_over::<geom_core::Interval>(&doc, None);
    let verdict = assertion_verdict(&ev, assertion);
    eprintln!("notch AtLeast 0.2 (true 0.269): {verdict:?}");
    assert!(
        !matches!(verdict, AssertionVerdict::Violated { .. }),
        "the faces clear 0.2 by 35%, so a certified Violated is false: {verdict:?}"
    );
}

/// **And `AtMost` reads a false `Holds`** — the direction the
/// `MinSeparation` docs say the containment-true reading makes sound.
#[test]
fn an_at_most_assertion_over_a_notch_does_not_read_holds_when_the_faces_exceed_it() {
    let (doc, _, assertion) = notch(0.15, AssertionDir::AtMost);
    let ev = eval_over::<geom_core::Interval>(&doc, None);
    let verdict = assertion_verdict(&ev, assertion);
    eprintln!("notch AtMost 0.15 (true 0.269): {verdict:?}");
    assert!(
        !matches!(verdict, AssertionVerdict::Holds { .. }),
        "the faces are 0.269 apart, so `≤ 0.15` cannot hold: {verdict:?}"
    );
}

/// A CONVEX pair (two rectangular walls) is exactly right: the bracket
/// encloses the built gap at every budget, including a starved one.
/// This is the case the unit's own suite covers, re-derived on a
/// two-body fixture.
#[test]
fn a_convex_pair_brackets_the_built_gap_at_every_budget() {
    let mut r = Recorder::new();
    let a = prism(
        &mut r,
        [0.0; 3],
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
    );
    let b = prism(
        &mut r,
        [0.0; 3],
        &[(1.7, 0.0), (2.7, 0.0), (2.7, 1.0), (1.7, 1.0)],
        1.0,
    );
    let ev = eval_over::<geom_core::Interval>(&r.doc, None);
    let body = |n: RecipeNodeId| match ev.result(n) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Body(body) => body,
            _ => panic!("a body"),
        },
        _ => panic!("evaluated"),
    };
    let (ba, bb) = (body(a), body(b));
    let all = |body: &topo::Body<geom_core::Interval>| body.faces().map(|(k, _)| k).collect();
    for pairs in [0usize, 1, 3, 64, 512] {
        let m = min_separation(
            &MinSepSelection {
                at: a,
                index: 0,
                body: ba,
                faces: all(ba),
            },
            &MinSepSelection {
                at: b,
                index: 0,
                body: bb,
                faces: all(bb),
            },
            MinSeparationConfig {
                max_cell_pairs: pairs,
                ..MinSeparationConfig::default()
            },
        )
        .expect("admitted");
        eprintln!(
            "two blocks @ pairs={pairs}: {}",
            m.serialize().replace('\n', " | ")
        );
        assert!(
            m.lo() <= 0.7 && 0.7 <= m.window_hi(),
            "budget {pairs}: [{}, {}]",
            m.lo(),
            m.window_hi()
        );
        assert!(m.receipt().holds());
    }
}

// ------------------------------------------------- 2. unary (D8)

/// The dumbbell of the unit's own suite, but measured BODY against
/// BODY: `min_clearance(sel, sel)` is E7's unary shape, and on one body
/// it is the self-clearance over every non-adjacent face pair — the
/// neck's 0.4.
#[test]
fn min_clearance_of_a_body_against_itself_is_the_selections_self_clearance() {
    let mut r = Recorder::new();
    let solid = prism(
        &mut r,
        [0.0; 3],
        &[
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
        ],
        2.0,
    );
    let ev = eval_over::<geom_core::Interval>(&r.doc, None);
    let Some(NodeResult::Ok(v)) = ev.result(solid) else {
        panic!("evaluated")
    };
    let ValuePayload::Body(body) = &v.payload else {
        panic!("a body")
    };
    let faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
    let sel = || MinSepSelection {
        at: solid,
        index: 0,
        body,
        faces: faces.clone(),
    };
    let m = min_separation(&sel(), &sel(), MinSeparationConfig::default()).expect("admitted");
    eprintln!(
        "dumbbell self-clearance: {}",
        m.serialize().replace('\n', " | ")
    );
    assert!(
        m.lo() <= 0.4 && 0.4 <= m.window_hi(),
        "[{}, {}]",
        m.lo(),
        m.window_hi()
    );
    // Tight enough to be the neck and not the outer walls (5 apart).
    assert!(m.window_hi() < 1.0);
}

// ------------------------------------ 3. the certifying vector

/// A plate whose web is `distance(wall 0, wall 2) = 2`, placed by a
/// uniform parameter, with an assertion `web ≥ bound`. No
/// `min_clearance` anywhere.
fn web_plate(bound: f64, law: Distribution) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    param(&mut r, "place", 0.0, Some(law));
    let solid = prism(
        &mut r,
        [0.0; 3],
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        1.0,
    );
    let placed = translate(
        &mut r,
        solid,
        [
            Expr::param(name("place"), Dimension::Length),
            len(0.0),
            len(0.0),
        ],
    );
    let measure = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                SitedRef::new(placed, fixture::fname(solid, fixture::wall(0))),
                SitedRef::new(placed, fixture::fname(solid, fixture::wall(2))),
            ],
        )
        .expect("in range"),
    );
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(bound),
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

/// **Finding (undisclosed in the PR body):** the verdict's shipped
/// witness vector is `certifying`, not `of`, so for EVERY
/// assertion-carrying document its key differs from the full vector's —
/// which is what the merge base serialized. Documents without a
/// `min_clearance` measure do not key their drive bit-identically.
#[test]
fn the_shipped_witness_vector_drops_the_assertion_row_for_any_assertion_document() {
    let (doc, _, _) = web_plate(1.0, uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &numeric_lane(), Tol::witness()).expect("builds");
    let witness = eval_over::<f64>(&doc, None);
    let full = VerdictVector::of(&witness);
    let shipped = verdict.witness_vector();
    eprintln!(
        "full vector rows={} shipped rows={}",
        full.rows.len(),
        shipped.rows.len()
    );
    assert_ne!(
        full.key(),
        shipped.key(),
        "the assertion row is in the full vector and not in the shipped one"
    );
    assert_eq!(full.rows.len(), shipped.rows.len() + 1);
    assert!(
        verdict
            .serialize()
            .contains(&format!("{:032x}", shipped.key().0))
            || !verdict
                .serialize()
                .contains(&format!("{:032x}", full.key().0)),
        "the goldening form carries the certifying key, never the full one"
    );
}

/// The behaviour the change buys: a bound INSIDE the leaf's enclosure
/// leaves the assertion `Unevaluated` over a leaf that is nonetheless
/// CERTIFIED (before this head the differing row refused the leaf as a
/// flip). EVIDENCE for the caption check in §7 too.
#[test]
fn a_bound_straddled_within_the_band_reads_holds_while_the_stackup_reads_under() {
    let (doc, measure, assertion) = web_plate(2.0, uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &numeric_lane(), Tol::witness()).expect("builds");
    assert!(
        !verdict.certified().is_empty(),
        "the ε-scaled box certifies"
    );
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("{}", budget.render());
    let mut seen = Vec::new();
    for leaf in verdict.certified() {
        let ev = eval_over::<geom_core::Interval>(&doc, Some(leaf.box_.clone()));
        seen.push(assertion_verdict(&ev, assertion));
    }
    // The enclosure straddles the bound by a few ulps — inside the
    // coincidence band `(−ε, ε)` — so the funnel classifies the margin
    // ZERO and the non-strict relation HOLDS. An `Unevaluated` would
    // need a straddle wider than ε, which an ε/64 box never produces.
    eprintln!("straddling by ulps: {seen:?}");
    assert!(
        seen.iter()
            .all(|v| matches!(v, AssertionVerdict::Holds { .. })),
        "a straddle inside the band reads Holds: {seen:?}"
    );
    let _ = UnevaluatedReason::Indeterminate;
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
    eprintln!("{}", report.render(&analyzed));
    // The tour's stop-2 sentence — "worst_case.lo < bound ⇒ the
    // requirement FAILS somewhere in the box" — is what a reader would
    // print here; the node itself says it has NO verdict.
    assert!(report.worst_case.lo < 2.0 && 2.0 < report.worst_case.hi);
}

// ------------------------------------------- 4. the cache key

/// **The finding, and its fix.** `report_key` was the tuple
/// (kind, slice, box, ε, K) and nothing else, so two drives of one
/// document over one box under two BUDGETS — different verdicts,
/// different goldening forms — keyed the same, and a `ReportCache`
/// keyed on it (its documented purpose) would have served the starved
/// verdict for the full one.
///
/// The dials are in the key now (`report::Dials`), so this row is a
/// PIN the other way round: the two budgets must key differently, and
/// the key must still be a pure function of what it is handed.
#[test]
fn report_key_tells_two_budgets_apart() {
    let eps = Tol::witness().eps();
    let mut r = Recorder::new();
    param(
        &mut r,
        "depth",
        20.0 * eps,
        Some(Distribution::Uniform {
            lo: -40.0 * eps,
            hi: 40.0 * eps,
        }),
    );
    let plane = r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]).expect("square"),
        ],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: Expr::param(name("depth"), Dimension::Length),
    });
    let analyzed = analyzed_box(&r.doc, &AnalysisPolicy::default());
    let starved = drive(
        &r.doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 4,
            ..numeric_lane()
        },
        Tol::witness(),
    )
    .expect("builds");
    let full = drive(
        &r.doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 4096,
            ..numeric_lane()
        },
        Tol::witness(),
    )
    .expect("builds");
    assert_ne!(
        starved.serialize(),
        full.serialize(),
        "the budget moved the verdict"
    );
    let slice = 7u128;
    let starved_cfg = DriveConfig {
        max_leaves: 4,
        ..numeric_lane()
    };
    let full_cfg = DriveConfig {
        max_leaves: 4096,
        ..numeric_lane()
    };
    let k_starved = report_key(
        "verdict",
        slice,
        starved.root(),
        eps,
        Tol::witness().k(),
        &Dials {
            drive: &starved_cfg,
            mc: None,
        },
    );
    let k_full = report_key(
        "verdict",
        slice,
        full.root(),
        eps,
        Tol::witness().k(),
        &Dials {
            drive: &full_cfg,
            mc: None,
        },
    );
    // **The finding, fixed and pinned the other way round.** The review
    // wrote this row as `assert_eq!` — the two budgets produced
    // different verdicts and the SAME key, which is the collision a
    // content key exists to make impossible. The dials are in the key
    // now, so the row asserts what it was always about: a key that can
    // tell them apart.
    assert_ne!(
        k_starved, k_full,
        "different budgets produce different reports, so they must not share a key"
    );
    // And the tuple ALONE still cannot: same dials, same key, whatever
    // the verdict was. That half is what makes the row about the dials
    // rather than about the box.
    assert_eq!(
        report_key(
            "verdict",
            slice,
            starved.root(),
            eps,
            Tol::witness().k(),
            &Dials {
                drive: &starved_cfg,
                mc: None,
            },
        ),
        k_starved,
        "the key is a pure function of its inputs"
    );
    eprintln!(
        "report_key over budgets 4 and 4096: {k_starved:?} vs {k_full:?}; verdict keys \
         {:?} vs {:?}",
        starved.content_key(),
        full.content_key()
    );
}

// ------------------------------------------ 5. row 1's three reds

/// The unit's neck with the bound / the pairing / the box as arguments.
fn neck(bound: f64, wall_b: u32, law: Distribution) -> (ProfileDoc, RecipeNodeId) {
    neck_dir(bound, wall_b, law, AssertionDir::AtLeast)
}

/// The same, with the assertion's direction chosen — which the fix
/// pass made load-bearing: `min_clearance` reaches `Violated` only
/// through `AtMost` now (`measure::Certified`), because the `AtLeast`
/// arm that would read the carrier's upper end refuses instead.
fn neck_dir(
    bound: f64,
    wall_b: u32,
    law: Distribution,
    dir: AssertionDir,
) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    param(&mut r, "place", 0.0, Some(law));
    let solid = prism(
        &mut r,
        [0.0; 3],
        &[
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
        ],
        2.0,
    );
    let placed = translate(
        &mut r,
        solid,
        [
            Expr::param(name("place"), Dimension::Length),
            len(0.0),
            len(0.0),
        ],
    );
    let measure = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::MinClearance { a: 0, b: 1 }),
            vec![
                SitedRef::new(placed, fixture::fname(solid, fixture::wall(2))),
                SitedRef::new(placed, fixture::fname(solid, fixture::wall(wall_b))),
            ],
        )
        .expect("in range"),
    );
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(bound),
        dir,
    });
    (r.doc, assertion)
}

/// Planted `Violated`: `min_clearance ≤ 0.3` over a neck that is `0.4`
/// apart reads `Violated` over a certified leaf — the red row 1
/// promises.
///
/// **`AtMost`, not `AtLeast`, and that is the fix showing through.**
/// The verdict is read off `lo` (`lo > 0.3`), the end certified for the
/// faces, so it is sound and it survives. The row as the review wrote
/// it planted the violation through `AtLeast 0.5`, which reaches
/// `Violated` only by reading the carrier's upper end — the false
/// certification the two rows above pin. That arm now refuses, so
/// planting through it would be planting a refusal.
#[test]
fn a_planted_violated_reads_violated_over_a_certified_leaf() {
    let (doc, assertion) = neck_dir(0.3, 9, uniform(), AssertionDir::AtMost);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &numeric_lane(), Tol::witness()).expect("builds");
    assert!(!verdict.certified().is_empty());
    let leaf = &verdict.certified()[0];
    let ev = eval_over::<geom_core::Interval>(&doc, Some(leaf.box_.clone()));
    let v = assertion_verdict(&ev, assertion);
    assert!(matches!(v, AssertionVerdict::Violated { .. }), "{v:?}");
}

/// **And the arm that no longer decides says so rather than going
/// quiet**: the review's own `AtLeast 0.5` fixture, which used to read
/// a certified `Violated` off the windows, now refuses typed and names
/// both the endpoint and the tracker item that retires the refusal.
#[test]
fn the_at_least_arm_that_read_the_carriers_end_now_refuses_by_name() {
    let (doc, assertion) = neck(0.5, 9, uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &numeric_lane(), Tol::witness()).expect("builds");
    let leaf = &verdict.certified()[0];
    let ev = eval_over::<geom_core::Interval>(&doc, Some(leaf.box_.clone()));
    match assertion_verdict(&ev, assertion) {
        AssertionVerdict::Unevaluated {
            reason:
                UnevaluatedReason::WindowSuperset {
                    verb,
                    endpoint,
                    recourse,
                },
        } => {
            assert_eq!((verb, endpoint), ("min_clearance", "upper"));
            assert_eq!(recourse, editor_core::WINDOW_TIGHTENING);
        }
        other => panic!("expected a typed window-superset refusal, got {other:?}"),
    }
}

/// Planted engine refusal (a wall against itself, `NoAdmittedPair`):
/// the f64 witness still builds (no engine call at a point scalar), the
/// interval replay fails the measure node, the leaf is refused, and the
/// unresolved mass overruns a zero budget — the row reds through the
/// budget, naming the mass.
#[test]
fn a_planted_engine_refusal_becomes_refused_mass_that_overruns_a_zero_budget() {
    let (doc, _) = neck(0.3, 2, uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    // CAPPED: a box-independent engine refusal (`NoAdmittedPair`) fails
    // the measure node on every leaf, and `classify` answers any other
    // node failure with `Bisect` — so the default 65,536-leaf budget is
    // spent bisecting a refusal that no subdivision can change, and the
    // accounting names `budget` rather than the engine's own class.
    let verdict = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 64,
            ..numeric_lane()
        },
        Tol::witness(),
    )
    .expect("the f64 witness builds: no engine runs at a point scalar");
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("{}", budget.render());
    let unresolved = budget.unresolved.clone().expect("priced");
    assert!(unresolved > 0.0, "the refused leaf is priced: {unresolved}");
    assert!(verdict.certified().is_empty());
}

/// Budget overrun: a box the widening class refuses (issue 1191) leaves
/// unresolved mass, and a recorded budget of zero reds it.
#[test]
fn a_wide_box_overruns_a_zero_budget() {
    let (doc, _) = neck(
        0.3,
        9,
        Distribution::Uniform {
            lo: -1e-3,
            hi: 1e-3,
        },
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 64,
            ..numeric_lane()
        },
        Tol::witness(),
    )
    .expect("builds");
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("{}", budget.render());
    assert!(budget.unresolved.clone().expect("priced") > 0.0);
}

// ------------------------------------------- 6. priced vs forced

/// Two varying parameters, one uniform and one BAND: the basis is
/// forced and names only the band; when the drive splits the band axis
/// the sub-box masses refuse typed, and the rendering says so.
#[test]
fn a_mixed_document_is_forced_by_its_band_alone_and_split_band_masses_refuse_typed() {
    let mut r = Recorder::new();
    param(&mut r, "place", 0.0, Some(uniform()));
    param(
        &mut r,
        "lift",
        0.0,
        Some(Distribution::Band {
            lo: -half(),
            hi: half(),
        }),
    );
    let solid = prism(
        &mut r,
        [0.0; 3],
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        1.0,
    );
    let placed = translate(
        &mut r,
        solid,
        [
            Expr::param(name("place"), Dimension::Length),
            len(0.0),
            Expr::param(name("lift"), Dimension::Length),
        ],
    );
    let measure = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                SitedRef::new(placed, fixture::fname(solid, fixture::wall(0))),
                SitedRef::new(placed, fixture::fname(solid, fixture::wall(2))),
            ],
        )
        .expect("in range"),
    );
    r.insert(Node::Assertion {
        measure,
        bound: len(1.0),
        dir: AssertionDir::AtLeast,
    });
    let analyzed = analyzed_box(&r.doc, &AnalysisPolicy::default());
    match MassBasis::of(&analyzed) {
        MassBasis::Forced { by } => assert_eq!(by, vec![name("lift")]),
        other => panic!("{other:?}"),
    }
    let verdict = drive(&r.doc, &analyzed, &numeric_lane(), Tol::witness()).expect("builds");
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("mixed:\n{}\n{}", budget.render(), budget.serialize());
    assert!(budget.render().contains("FORCED, not priced: lift"));
    // Now a band-only box that the drive must SPLIT: a starved split on
    // a band axis prices nothing, typed.
    let mut sub = BTreeMap::new();
    sub.insert(
        name("lift"),
        BoxAxis::Varying {
            lo: -half(),
            hi: 0.0,
        },
    );
    let piece = ParamBox::from_axes(sub);
    match piece.mass(&analyzed) {
        Err(e) => eprintln!("a half of the band axis: {e}"),
        Ok(m) => panic!("a band sub-box has no mass: {m}"),
    }
}

// ---------------------------------------------- 8. MC over min_clearance

/// The MC lane over a `min_clearance` document: every sample is
/// unmeasured and undecided, the fraction is `None`, and the rendering
/// says so with its count and seed — an honest nothing.
#[test]
fn the_mc_lane_over_a_min_clearance_document_decides_nothing_and_says_so() {
    let (doc, _) = neck(0.3, 9, uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let report = monte_carlo(
        &doc,
        &analyzed,
        &McConfig {
            samples: 16,
            ..McConfig::default()
        },
        Tol::witness(),
    )
    .expect("replays");
    eprintln!("{}", report.render());
    assert_eq!(report.measures[0].unmeasured, 16);
    assert_eq!(report.assertions[0].unevaluated, 16);
    assert!(report.assertions[0].violation_fraction().is_none());
    assert!(report.render().contains("no sample decided"));
}

// --------------------------------------------- 9. the consumer walk

/// A bracket: a base slab and a post parked above it, the post placed
/// by `offset` (uniform) and lifted by `lift` (normal). Two measures —
/// the web between the post's wall and the base's wall, and the
/// `min_clearance` between the two bodies — each with an assertion.
fn bracket(
    offset_law: Distribution,
    lift_law: Distribution,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
) {
    let mut r = Recorder::new();
    param(&mut r, "offset", 0.0, Some(offset_law));
    param(&mut r, "lift", 0.2, Some(lift_law));
    let base = prism(
        &mut r,
        [0.0; 3],
        &[(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (0.0, 1.0)],
        1.0,
    );
    let post_solid = prism(
        &mut r,
        [0.0; 3],
        &[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)],
        2.0,
    );
    let post = translate(
        &mut r,
        post_solid,
        [
            Expr::param(name("offset"), Dimension::Length),
            len(0.0),
            Expr::sub(
                len(1.0),
                Expr::neg(Expr::param(name("lift"), Dimension::Length)),
            )
            .expect("length"),
        ],
    );
    // web = distance(post's x=1 wall, base's x=0 wall) = 1 + offset.
    let web = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                SitedRef::new(post, fixture::fname(post_solid, fixture::wall(3))),
                SitedRef::at_mint(fixture::fname(base, fixture::wall(3))),
            ],
        )
        .expect("in range"),
    );
    let web_ok = r.insert(Node::Assertion {
        measure: web,
        bound: len(0.9),
        dir: AssertionDir::AtLeast,
    });
    let clearance = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::MinClearance { a: 0, b: 1 }),
            vec![
                SitedRef::new(
                    post,
                    editor_core::StableName {
                        kind: editor_core::EntityKind::Body,
                        node: post_solid,
                        path: vec![RoleSeg::OutputBody],
                    },
                ),
                SitedRef::at_mint(editor_core::StableName {
                    kind: editor_core::EntityKind::Body,
                    node: base,
                    path: vec![RoleSeg::OutputBody],
                }),
            ],
        )
        .expect("in range"),
    );
    let clear_ok = r.insert(Node::Assertion {
        measure: clearance,
        bound: len(0.1),
        dir: AssertionDir::AtLeast,
    });
    (r.doc, web, web_ok, clearance, clear_ok)
}

/// The walk a consumer takes today, rendered as a consumer would read
/// it. EVIDENCE-ONLY beyond the shape assertions; the friction is in
/// the review report.
#[test]
fn the_bracket_walk_through_the_public_doors() {
    let tol = Tol::witness();
    let (doc, web, web_ok, clearance, clear_ok) = bracket(
        uniform(),
        Distribution::Normal {
            sigma: half() / 3.0,
        },
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &numeric_lane(), tol).expect("builds");
    eprintln!("== drive\n{}", verdict.render(&analyzed));
    assert!(
        !verdict.certified().is_empty(),
        "the bracket certifies at ε/64"
    );

    eprintln!("== stackup(web)");
    let report = stackup(&doc, web, &analyzed, &verdict, None, true, tol).expect("a stackup");
    eprintln!("{}", report.render(&analyzed));
    assert!(report.worst_case.lo <= 1.0 && 1.0 <= report.worst_case.hi);

    // **The walk's finding, and what the fix pass did with it.** This
    // used to refuse `MeasureRefusedAtNominal` — the whole report
    // withheld because an ADVISORY column (the f64 nominal) cannot
    // exist for a `min_clearance`. E9 says a degraded advisory column
    // forfeits and the gate stays, so the report builds now: the
    // certified worst case is there and gates, and the nominal
    // forfeits by name.
    eprintln!("== stackup(min_clearance)");
    let clearance_report =
        stackup(&doc, clearance, &analyzed, &verdict, None, true, tol).expect("a stackup");
    eprintln!("{}", clearance_report.render(&analyzed));
    assert!(
        clearance_report.worst_case.leaves > 0,
        "the gating column is built from the certified leaves"
    );
    assert!(
        clearance_report.nominal.is_err(),
        "…and the advisory nominal forfeits, because a point scalar has no enclosure"
    );

    eprintln!("== fold(post vs base ≥ 0.1)");
    let fold = clearance_over(
        &doc,
        &analyzed,
        &verdict,
        &Selection::body_of(verdict_post(&doc)),
        &Selection::body_of(verdict_base(&doc)),
        &ClearanceQuery::at_least(0.1, tol),
    );
    // **§2's two doors and the key, on the three reports that shipped
    // with no row at all** (R2's MINOR-10): `LeafFold`'s trio,
    // `ClearanceReport`'s — which had no `content_key` until the fix
    // pass, though claim 3 names it — and `MinSeparation::render`.
    let fold_gold = fold.serialize();
    let fold_human = fold.render();
    eprintln!("{fold_human}");
    assert_ne!(
        fold_gold, fold_human,
        "the goldening form and the human form are not each other"
    );
    assert!(
        fold_human.contains('%'),
        "the human form reports masses as percentages: {fold_human}"
    );
    assert_eq!(
        fold.content_key(),
        editor_core::eval::key_of(0xED, &fold_gold),
        "the fold's key is its tag over its own serialized bits, and nothing else"
    );
    // And the per-leaf report the fold is made of, through its own
    // single-leaf door.
    let leaf_box = verdict.certified()[0].box_.clone();
    let report = editor_core::clearance::clearance(
        &doc,
        &leaf_box,
        &Selection::body_of(verdict_post(&doc)),
        &Selection::body_of(verdict_base(&doc)),
        0.1,
        tol,
    );
    let gold = report.serialize();
    let human = report.render();
    assert_ne!(gold, human, "and the same for a per-leaf clearance report");
    assert_eq!(
        report.content_key(),
        editor_core::eval::key_of(0xEE, &gold),
        "ClearanceReport's key, added in the fix pass — claim 3 named it and it did not exist"
    );

    eprintln!("== histogram(web)");
    let h = leaf_histogram(&doc, &analyzed, &verdict, web, tol);
    eprintln!("{}", h.render());
    eprintln!("== histogram(min_clearance)");
    let hc = leaf_histogram(&doc, &analyzed, &verdict, clearance, tol);
    eprintln!("{}", hc.render());
    assert_eq!(hc.rows.len(), verdict.certified().len());
    for row in &hc.rows {
        assert!(
            row.enclosure.0 <= 0.2 && 0.2 <= row.enclosure.1,
            "{:?}",
            row.enclosure
        );
    }

    eprintln!("== assertions over the certified leaves");
    for leaf in verdict.certified() {
        let ev = eval_over::<geom_core::Interval>(&doc, Some(leaf.box_.clone()));
        let (a, b) = (
            assertion_verdict(&ev, web_ok),
            assertion_verdict(&ev, clear_ok),
        );
        assert_eq!(a.holds(), Some(true), "{a:?}");
        assert_eq!(b.holds(), Some(true), "{b:?}");
    }

    eprintln!("== mc");
    let mc = monte_carlo(&doc, &analyzed, &McConfig::default(), tol).expect("replays");
    eprintln!("{}", mc.render());
    assert_eq!(mc.measures.len(), 2);
    assert_eq!(mc.measures[1].unmeasured, mc.samples);

    eprintln!("== budget");
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("{}", budget.render());
    assert_eq!(budget.basis, MassBasis::Priced);

    // The same bracket with a BAND on the offset: the budget reads
    // forced, the MC lane refuses typed, and the stackup's rss is
    // unavailable.
    let (doc, web, _, _, _) = bracket(
        Distribution::Band {
            lo: -half(),
            hi: half(),
        },
        Distribution::Normal {
            sigma: half() / 3.0,
        },
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &numeric_lane(), tol).expect("builds");
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("== band budget\n{}", budget.render());
    assert!(matches!(budget.basis, MassBasis::Forced { .. }));
    assert!(monte_carlo(&doc, &analyzed, &McConfig::default(), tol).is_err());
    if let Ok(report) = stackup(&doc, web, &analyzed, &verdict, None, true, tol) {
        eprintln!("== band stackup\n{}", report.render(&analyzed));
    }
}

fn node_named(doc: &ProfileDoc, pick: usize) -> RecipeNodeId {
    let mut transforms: Vec<RecipeNodeId> = doc
        .order()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(Node::Transform { .. })))
        .collect();
    let mut extrudes: Vec<RecipeNodeId> = doc
        .order()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(Node::Extrude { .. })))
        .collect();
    if pick == 0 {
        transforms.remove(0)
    } else {
        extrudes.remove(0)
    }
}

fn verdict_post(doc: &ProfileDoc) -> RecipeNodeId {
    node_named(doc, 0)
}

fn verdict_base(doc: &ProfileDoc) -> RecipeNodeId {
    node_named(doc, 1)
}

// ------------------------------- 7b. the tour's stop 2, verdict read back

/// The tour's stop-2 plate (ERROR-DESIGN's worked example at the
/// certifiable box), authored with editor-core's doors exactly as
/// `demos/tour/src/tolerance.rs` authors it, and then the one thing the
/// cell never prints: the Assertion node's own verdict over each
/// certified leaf. The caption says the certified worst case "FAILS
/// somewhere in the box: this is the number that gates"; row 1 gates on
/// the node's verdict.
#[test]
fn the_tours_stop_two_assertion_reads_holds_where_the_caption_says_fails() {
    use editor_core::{EntityKind, GeomPred, NamePat, Selector, SurfaceKindSet, select_where};
    let tol = Tol::witness();
    const SPACING: f64 = 3.1e-3;
    const RADIUS: f64 = 1.25e-3;
    const WEB: f64 = SPACING - 2.0 * RADIUS;
    let spread = tol.eps() / 64.0;
    let (half_width, sigma) = (0.05 * spread, 0.2 * spread);
    let worst = 2.0 * half_width + 2.0 * (3.0 * sigma);
    let rss3 = 3.0 * ((2.0 * half_width / 3.0_f64.sqrt()).powi(2) + 2.0 * sigma.powi(2)).sqrt();
    let bound = WEB - 0.5 * (worst + rss3);

    let mut r = Recorder::new();
    param(
        &mut r,
        "half_spacing",
        SPACING / 2.0,
        Some(Distribution::Uniform {
            lo: -half_width,
            hi: half_width,
        }),
    );
    for n in ["hole_a_r", "hole_b_r"] {
        param(&mut r, n, RADIUS, Some(Distribution::Normal { sigma }));
    }
    let plane = r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let plate_p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([
                (-4.0e-3, -2.0e-3),
                (4.0e-3, -2.0e-3),
                (4.0e-3, 2.0e-3),
                (-4.0e-3, 2.0e-3),
            ])
            .expect("plate"),
        ],
    }));
    let _plate = r.insert(Node::Extrude {
        profile: plate_p,
        distance: len(1.0e-3),
    });
    let hole = |r: &mut Recorder, centre: Expr, radius: &str| {
        let p = r.insert(Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Circle {
                centre: [centre, len(0.0)],
                radius: Expr::param(name(radius), Dimension::Length),
            }],
        }));
        r.insert(Node::Extrude {
            profile: p,
            distance: len(1.0e-3),
        })
    };
    let hole_a = hole(
        &mut r,
        Expr::sub(
            len(0.0),
            Expr::param(name("half_spacing"), Dimension::Length),
        )
        .expect("length"),
        "hole_a_r",
    );
    let hole_b = hole(
        &mut r,
        Expr::param(name("half_spacing"), Dimension::Length),
        "hole_b_r",
    );
    let ev = eval_over::<f64>(&r.doc, None);
    let wall = |node: RecipeNodeId| {
        let mut faces = select_where(
            &ev,
            node,
            &Selector::of(NamePat::of_kind(EntityKind::Face)),
            &[GeomPred::SurfaceKind(SurfaceKindSet::just(
                geom_brep::SurfaceKind::Cylinder,
            ))],
            &r.doc.param_env::<f64>(),
            tol,
        )
        .expect("exact atom");
        faces.sort();
        SitedRef::new(node, faces.remove(0))
    };
    let refs = vec![wall(hole_a), wall(hole_b)];
    let radius_of = |n: &str| MeasureExpr::value(Expr::param(name(n), Dimension::Length));
    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::add(radius_of("hole_a_r"), radius_of("hole_b_r")).expect("L + L"),
    )
    .expect("L − L");
    let measure = r.insert(Node::measure(web, refs).expect("in range"));
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(bound),
        dir: AssertionDir::AtLeast,
    });

    let analyzed = analyzed_box(&r.doc, &AnalysisPolicy::default());
    let verdict = drive(&r.doc, &analyzed, &numeric_lane(), tol).expect("builds");
    assert!(!verdict.certified().is_empty());
    let report = stackup(&r.doc, measure, &analyzed, &verdict, None, true, tol).expect("stackup");
    eprintln!(
        "stop 2: worst_case [{:e}, {:e}] bound {bound:e} (lo − bound = {:e}, ε = {:e})",
        report.worst_case.lo,
        report.worst_case.hi,
        report.worst_case.lo - bound,
        tol.eps()
    );
    assert!(report.worst_case.lo < bound, "the caption's premise");
    for leaf in verdict.certified() {
        let lev = eval_over::<geom_core::Interval>(&r.doc, Some(leaf.box_.clone()));
        let v = assertion_verdict(&lev, assertion);
        eprintln!("stop 2: the assertion node over the certified leaf: {v:?}");
        assert!(
            matches!(v, AssertionVerdict::Holds { .. }),
            "the straddle is inside the coincidence band, so the node says HOLDS: {v:?}"
        );
    }
}
