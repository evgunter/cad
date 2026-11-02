//! **R1's adversarial probes for M10-6** (PR #1685 at the frozen head
//! `bf67a734`). Independent derivations, written against the unit's
//! claims rather than against its own fixtures.
//!
//! Row shapes, per implementer-discipline §8: every row is a
//! written-down witness (a static fixture) — no sampling, no seeds.
//! Rows whose subject was a finding red at that head arrived
//! `#[ignore]`d with the finding named. **The fix pass un-ignored
//! them**: they are PINS now, and each one reds again if the fix is
//! undone. Rows marked EVIDENCE-ONLY print and assert only the shape
//! (they are not gates).
//!
//! What the rows attack, in order:
//!
//! 1. A clearance over TRIMMED faces: the engine
//!    subdivides carrier WINDOWS (bounding rectangles), so a non-convex
//!    face's window covers material the face does not have. On the
//!    L-shaped cap below a window pair straight across the notch belongs
//!    to neither face; neither assertion arm may read a verdict the faces
//!    contradict.
//! 2. The unary elaboration (D8): `clearance(body, body)` on one
//!    body. It was the selection's self-clearance over the pairs the
//!    wedge rule kept; a clearance is metric now (E7, *Self-intersection
//!    scope*), so the body meets itself and reads exactly 0, named on
//!    the shared cell.
//! 3. `drive::certifying_vector` moves the witness-vector key of every
//!    assertion-carrying document (claim 1's "bit-identical keys" for
//!    documents without a clearance).
//! 4. `report_key` omits the drive/MC config, so two different reports
//!    over one (slice, box, ε, K) tuple share a key.
//! 5. Row 1's three reds, as rows: a planted `Violated`, a planted
//!    engine refusal, a budget overrun. The planted refusal was a wall
//!    against itself (`NoAdmittedPair`), which SELFX retired: that wall
//!    reads 0 now and its `AtLeast` bound is violated. The pricing that row carried — an engine refusal no smaller
//!    box can change is refused mass under its own class — is pinned
//!    with the refusals the engine still raises, beside the classifier
//!    (`editor-core/src/drive.rs`,
//!    `a_box_independent_clearance_refusal_is_terminal_refused_mass`).
//! 6. The mixed priced/forced case and a band axis that splits.
//! 7. A caption check: an enclosure that STRADDLES the bound is
//!    `Unevaluated` at the node while `worst_case.lo < bound` — the
//!    tour's "FAILS somewhere in the box" reading.
//! 8. The MC lane over a clearance document.
//! 9. An end-to-end consumer walk on a bracket (post over base), through
//!    the public doors: drive, stackup, fold, histogram, MC, budget.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::clearance::{ClearanceQuery, Selection, clearance_over};
use editor_core::drive::{DriveConfig, VerdictVector, drive};
use editor_core::mc::{McConfig, monte_carlo};
use editor_core::report::{Dials, MassBasis, MassBudget, leaf_histogram, report_key};
use editor_core::stackup::stackup;
use editor_core::{
    AssertionRelation, AssertionVerdict, CancelToken, CapEnd, ClearanceBounds, Dimension,
    Distribution, DocEdit,
    EvalOptions, Formula, FreeVar, LoopProgram, MeasurePrimitive, Node, NodeResult, ProfileDoc,
    ProfileProgram, RecipeNodeId, RoleSeg, SitedRef, UnitSym, ValuePayload, VarName, evaluate,
};
use geom_core::Tol;
use topo::clearance::{Budgets, Side, min_separation};

use fixture::{Recorder, len};

/// The evaluation's own band, which the engine decides its cells at.
fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).expect("the witness tolerance has a band")
}

fn name(n: &'static str) -> VarName {
    VarName::from_static(n)
}

fn half() -> f64 {
    Tol::witness().eps() / 64.0
}

fn eval_over<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    box_: Option<ParamBox>,
) -> editor_core::Evaluation<T> {
    let opts = EvalOptions {
        param_box: box_.map(std::sync::Arc::new),
        ..EvalOptions::default()
    };
    evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness())
}

/// The assertion's verdict over a certified leaf, replayed on the lane
/// the drive certified it on ([`editor_core::drive::assertion_at`]).
fn on_lane(
    doc: &ProfileDoc,
    verdict: &editor_core::drive::ParamBoxVerdict,
    leaf: &editor_core::drive::CertifiedLeaf,
    assertion: RecipeNodeId,
) -> AssertionVerdict<geom_core::Interval> {
    editor_core::drive::assertion_at(
        doc,
        assertion,
        &leaf.box_,
        verdict.symbolic(),
        Tol::witness(),
    )
    .expect("an assertion node has a verdict over a certified leaf")
}

/// The clearance bounds `relation` reads at `bound`.
fn bounds(relation: AssertionRelation, bound: f64) -> ClearanceBounds<Formula> {
    match relation {
        AssertionRelation::AtLeast => ClearanceBounds::AtLeast(len(bound)),
        AssertionRelation::AtMost => ClearanceBounds::AtMost(len(bound)),
        AssertionRelation::Equal => ClearanceBounds::Equal(len(bound)),
    }
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

fn param(r: &mut Recorder, n: &'static str, value: f64, dist: Option<Distribution>) {
    r.push(DocEdit::DeclareVar {
        name: name(n),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: dist,
        }),
    });
}

fn translate(r: &mut Recorder, input: RecipeNodeId, t: [Formula; 3]) -> RecipeNodeId {
    crate::m10_5_clearance_interval::slid_by(r, input, input, t)
}

fn prism(r: &mut Recorder, corners: &[(f64, f64)], height: f64) -> RecipeNodeId {
    let profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::polygon(corners.iter().copied()).expect("finite corners")],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: profile.into(),
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
/// over the notch at height `LIFT`, with a clearance between the L's
/// top cap and the block's underside asserted against `bound`.
fn notch(bound: f64, relation: AssertionRelation) -> (ProfileDoc, RecipeNodeId) {
    let n = notch_nodes(bound, relation);
    (n.doc, n.assertion)
}

/// [`notch`]'s nodes, named.
struct Notch {
    doc: ProfileDoc,
    assertion: RecipeNodeId,
}

fn notch_nodes(bound: f64, relation: AssertionRelation) -> Notch {
    let mut r = Recorder::new();
    let ell = prism(
        &mut r,
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
        &[(1.25, 1.25), (1.75, 1.25), (1.75, 1.75), (1.25, 1.75)],
        0.5,
    );
    // The block hovers over the notch, `1 + LIFT` up the L's own z.
    let block_at = r.against(
        ell,
        block,
        &editor_core::Frame::translation([0.0, 0.0, 1.0 + LIFT]),
    );
    let assertion = r.insert(Node::clearance(
        crate::fixture::sited_ref(fixture::fname(ell, RoleSeg::Cap(CapEnd::End))),
        SitedRef::new(block_at, fixture::fname(block, RoleSeg::Cap(CapEnd::Start))),
        bounds(relation, bound),
    ));
    Notch {
        doc: r.doc,
        assertion,
    }
}

/// The true minimum distance between the L's top cap (an L-shaped
/// region at `z = 1`) and the block's underside (a square over the
/// notch at `z = 1 + LIFT`): the block's nearest corner to the L's
/// inner corner is `OFFSET` away horizontally.
fn true_notch_clearance() -> f64 {
    (OFFSET * OFFSET + LIFT * LIFT).sqrt()
}

/// **The engine's readings stay out of the clearance node's log.** The
/// notch's sweep decides cells and stations at every level, and every
/// one of those readings runs in the unlogged scope (audit row F30), so
/// the node records none of their rows and none of their in-band
/// escalations. Breaks if the scope is removed: measured, the node then
/// holds 2140 region-reading verdicts and 297 escalations.
#[test]
fn the_notch_clearance_records_none_of_the_engines_region_readings() {
    let n = notch_nodes(0.2, AssertionRelation::AtLeast);
    let ev = eval_over::<geom_core::Interval>(&n.doc, None);
    let Some(NodeResult::Ok(v)) = ev.result(n.assertion) else {
        panic!("the clearance node evaluated");
    };
    let region = |name: &str| {
        ["cell_region_", "bool_contact_", "point_in_loop_"]
            .iter()
            .any(|door| name.starts_with(door))
    };
    let leaked: Vec<_> = v.verdicts.iter().filter(|x| region(x.predicate)).collect();
    assert!(
        leaked.is_empty() && v.escalations.is_empty(),
        "the clearance node's log holds {} region-reading verdicts and {} escalations: {:?} {:?}",
        leaked.len(),
        v.escalations.len(),
        leaked.iter().take(4).collect::<Vec<_>>(),
        v.escalations.iter().take(4).collect::<Vec<_>>()
    );
}

/// `clearance(cap, underside) ≥ 0.2` holds on the faces (they are
/// 0.269 apart), so it never reads `Violated`.
#[test]
fn an_at_least_assertion_over_a_notch_does_not_read_violated_when_the_faces_clear_it() {
    let (doc, assertion) = notch(0.2, AssertionRelation::AtLeast);
    let ev = eval_over::<geom_core::Interval>(&doc, None);
    let verdict = assertion_verdict(&ev, assertion);
    eprintln!("notch AtLeast 0.2 (true 0.269): {verdict:?}");
    assert!(
        verdict.holds() != Some(false),
        "the faces clear 0.2 by 35%, so a certified Violated is false: {verdict:?}"
    );
}

/// And `≤ 0.15` never reads `Holds`.
#[test]
fn an_at_most_assertion_over_a_notch_does_not_read_holds_when_the_faces_exceed_it() {
    let (doc, assertion) = notch(0.15, AssertionRelation::AtMost);
    let ev = eval_over::<geom_core::Interval>(&doc, None);
    let verdict = assertion_verdict(&ev, assertion);
    eprintln!("notch AtMost 0.15 (true 0.269): {verdict:?}");
    assert!(
        verdict.holds() != Some(true),
        "the faces are 0.269 apart, so `≤ 0.15` cannot hold: {verdict:?}"
    );
}

/// **An upper bound over the notch is proved only by face points.**
/// `clearance ≤ 0.3` holds — the faces are 0.269 apart — and the pair
/// that proves it is two points of the FACES: the windows' pair straight
/// across the notch, 0.1 apart, is no pair of the faces, so a proving
/// pair closer than 0.269 sits off a trimmed face.
#[test]
fn an_upper_bound_over_a_notch_is_proved_only_by_face_points() {
    let (doc, assertion) = notch(0.3, AssertionRelation::AtMost);
    let ev = eval_over::<geom_core::Interval>(&doc, None);
    let AssertionVerdict::Clearance(report) = assertion_verdict(&ev, assertion) else {
        panic!("a clearance's verdict");
    };
    let editor_core::clearance::ClearanceVerdict::Within(w) = report.verdict() else {
        panic!("the faces are within 0.3: {}", report.render());
    };
    let (d, truth) = (w.geometry.distance, true_notch_clearance());
    assert!(
        truth - band().zero() <= d && d <= 0.3,
        "the proving pair is two face points, no closer than the faces' {truth}: {d}"
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
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
    );
    let b = prism(
        &mut r,
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
            &Side {
                body: ba.body(),
                faces: all(ba.body()),
            },
            &Side {
                body: bb.body(),
                faces: all(bb.body()),
            },
            // Body 0 of node `a` against body 0 of node `b`.
            a == b,
            band(),
            Budgets {
                max_cell_pairs: pairs,
                ..Budgets::MIN_SEPARATION
            },
        )
        .expect("admitted");
        eprintln!(
            "two blocks @ pairs={pairs}: {}",
            m.serialize().replace('\n', " | ")
        );
        let hi = m.hi().expect("a wall pair witnesses an upper end");
        assert!(
            m.lo() <= 0.7 && 0.7 <= hi,
            "budget {pairs}: [{}, {hi}]",
            m.lo()
        );
        assert!(m.receipt().holds());
    }
}

// ------------------------------------------------- 2. unary (D8)

/// The dumbbell of the unit's own suite, but measured BODY against
/// BODY: `clearance(sel, sel)` is E7's unary shape. It used to drop
/// every pair that shared a vertex and read the neck's 0.4; a
/// clearance is metric now (E7, *Self-intersection scope*), so the
/// body meets itself and reads exactly 0, named on the cell the scopes
/// share. Whether the body is embedded is the injectivity question,
/// `self_intersection`.
#[test]
fn min_clearance_of_a_body_against_itself_meets_itself_and_reads_zero() {
    let mut r = Recorder::new();
    let solid = prism(
        &mut r,
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
    let faces: Vec<_> = body.body().faces().map(|(k, _)| k).collect();
    let sel = || Side {
        body: body.body(),
        faces: faces.clone(),
    };
    let m =
        min_separation(&sel(), &sel(), true, band(), Budgets::MIN_SEPARATION).expect("admitted");
    eprintln!(
        "dumbbell self-clearance: {}",
        m.serialize().replace('\n', " | ")
    );
    assert_eq!((m.lo(), m.hi()), (0.0, Some(0.0)), "{}", m.render());
    assert!(
        m.meets().is_some(),
        "named on the shared cell: {}",
        m.render()
    );
}

// ------------------------------------ 3. the certifying vector

/// A plate whose web is `distance(wall 0, wall 2) = 2`, placed by a
/// uniform parameter, with an assertion `web ≥ bound`. No
/// clearance anywhere.
fn web_plate(bound: f64, law: Distribution) -> (ProfileDoc, editor_core::VarId, RecipeNodeId) {
    web_across(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        bound,
        law,
    )
}

/// [`web_plate`] over the prism on `outline`: the web is the distance
/// between its walls 0 and 2.
fn web_across(
    outline: &[(f64, f64)],
    bound: f64,
    law: Distribution,
) -> (ProfileDoc, editor_core::VarId, RecipeNodeId) {
    let mut r = Recorder::new();
    param(&mut r, "place", 0.0, Some(law));
    let solid = prism(&mut r, outline, 1.0);
    let placed = translate(
        &mut r,
        solid,
        [
            Formula::named(name("place"), Dimension::Length),
            len(0.0),
            len(0.0),
        ],
    );
    let measure = r
        .measure(
            &[MeasurePrimitive::Distance { a: 0, b: 1 }],
            &[
                SitedRef::new(
                    placed,
                    fixture::fname(solid, fixture::wall(&r.doc, solid, 0)),
                ),
                SitedRef::new(
                    placed,
                    fixture::fname(solid, fixture::wall(&r.doc, solid, 2)),
                ),
            ],
        )
        .outputs[0];
    let assertion = r.insert(Node::Assertion { claim: editor_core::Claim::Value {
        value: fixture::read_var(&r.doc, measure),
        bound: len(bound),
        relation: AssertionRelation::AtLeast,
    } });
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
/// clearance do not key their drive bit-identically.
#[test]
fn the_shipped_witness_vector_drops_the_assertion_row_for_any_assertion_document() {
    let (doc, _, _) = web_plate(1.0, uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness()).expect("builds");
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

/// The behaviour the change buys: a bound INSIDE the leaf's enclosure,
/// by ulps, reads `Holds` over a leaf that is CERTIFIED (before this
/// head the differing row refused the leaf as a flip), while the
/// stackup's worst case, read as numbers, straddles the bound. EVIDENCE
/// for the caption check in §7 too.
///
/// The web runs between the slanted walls of a parallelogram, the
/// lines `y = x` and `y = x + 2`, so it is `√2` and no enclosure of it
/// is a point; the bound is the `f64` nearest `√2`.
#[test]
fn a_bound_straddled_within_the_band_reads_holds_while_the_stackup_reads_under() {
    let bound = std::f64::consts::SQRT_2;
    let (doc, measure, assertion) = web_across(
        &[(0.0, 0.0), (1.0, 1.0), (1.0, 3.0), (0.0, 2.0)],
        bound,
        uniform(),
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness()).expect("builds");
    assert!(
        !verdict.certified().is_empty(),
        "the ε-scaled box certifies"
    );
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("{}", budget.render(&doc));
    let mut seen = Vec::new();
    for leaf in verdict.certified() {
        seen.push(on_lane(&doc, &verdict, leaf, assertion));
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
    let report = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        true,
        None,
        Tol::witness(),
    )
    .expect("a stackup");
    eprintln!("{}", report.render(&doc, &analyzed));
    // The tour's stop-2 sentence — "worst_case.lo < bound ⇒ the
    // requirement FAILS somewhere in the box" — is what a reader would
    // print here; the node itself says the requirement HOLDS.
    assert!(
        report.worst_case.lo < bound && bound < report.worst_case.hi,
        "the stackup straddles the bound: {:?}",
        report.worst_case
    );
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
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]).expect("square"),
        ],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: Formula::named(name("depth"), Dimension::Length),
    });
    let analyzed = analyzed_box(&r.doc, &AnalysisPolicy::default());
    let starved = drive(
        &r.doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 4,
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .expect("builds");
    let full = drive(
        &r.doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 4096,
            ..DriveConfig::default()
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
        ..DriveConfig::default()
    };
    let full_cfg = DriveConfig {
        max_leaves: 4096,
        ..DriveConfig::default()
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
    neck_related(bound, wall_b, law, AssertionRelation::AtLeast)
}

/// The same, with the assertion's relation chosen.
fn neck_related(
    bound: f64,
    wall_b: u32,
    law: Distribution,
    relation: AssertionRelation,
) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    param(&mut r, "place", 0.0, Some(law));
    let solid = prism(
        &mut r,
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
            Formula::named(name("place"), Dimension::Length),
            len(0.0),
            len(0.0),
        ],
    );
    let assertion = r.insert(Node::clearance(
        SitedRef::new(placed, fixture::fname(solid, fixture::wall(&r.doc, solid, 2))),
        SitedRef::new(
            placed,
            fixture::fname(solid, fixture::wall(&r.doc, solid, wall_b)),
        ),
        bounds(relation, bound),
    ));
    (r.doc, assertion)
}

/// Planted `Violated`: `clearance ≤ 0.3` over a neck that is `0.4`
/// apart reads `Violated` over a certified leaf — the red row 1
/// promises — every pair certified clear of the bound.
#[test]
fn a_planted_violated_reads_violated_over_a_certified_leaf() {
    let (doc, assertion) = neck_related(0.3, 9, uniform(), AssertionRelation::AtMost);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness()).expect("builds");
    assert!(!verdict.certified().is_empty());
    let v = on_lane(&doc, &verdict, &verdict.certified()[0], assertion);
    assert_eq!(v.holds(), Some(false), "{v:?}");
}

/// **The review's own `AtLeast 0.5` fixture reads `Violated`**, by a
/// pair of the faces: the neck is `0.4` apart.
#[test]
fn the_at_least_arm_reads_violated_off_the_faces_upper_end() {
    let (doc, assertion) = neck(0.5, 9, uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness()).expect("builds");
    let v = on_lane(&doc, &verdict, &verdict.certified()[0], assertion);
    assert_eq!(v.holds(), Some(false), "{v:?}");
}

/// **`=` reads both bounds.** Over the 0.4 neck on a certified leaf,
/// `= 0.3` is violated by every pair clear of it and `= 0.5` by a pair
/// closer. Breaks if `Equal` reads one bound only (one of the two holds
/// or refuses).
#[test]
fn equal_over_a_clearance_is_violated_off_either_bound() {
    let at = |bound| {
        let (doc, assertion) = neck_related(bound, 9, uniform(), AssertionRelation::Equal);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let verdict =
            drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness()).expect("builds");
        on_lane(&doc, &verdict, &verdict.certified()[0], assertion)
    };
    for bound in [0.3, 0.5] {
        let v = at(bound);
        assert_eq!(v.holds(), Some(false), "= {bound}: {v:?}");
    }
}

/// **A wall against itself.** A clearance is metric: the wall meets
/// itself at distance 0, so the drive certifies the leaf and the
/// `AtLeast 0.3` bound is violated by the meeting.
#[test]
fn a_wall_against_itself_reads_zero_and_violates_its_at_least_bound() {
    let (doc, assertion) = neck(0.3, 2, uniform());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 64,
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .expect("the f64 witness builds: no engine runs at a point scalar");
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("{}", budget.render(&doc));
    let leaf = verdict
        .certified()
        .first()
        .expect("the assertion builds, so the leaf certifies");
    let ev = eval_over::<geom_core::Interval>(&doc, Some(leaf.box_.clone()));
    let v = assertion_verdict(&ev, assertion);
    assert_eq!(v.holds(), Some(false), "{v:?}");
}

/// **A wide box over a copy certifies whole.** The neck's placed prism
/// is its source's body under a frame the box moves (D10
/// `[placement.frame]`), so no copy is rebuilt over the box and nothing
/// widens past the band: the box that issue 1191's class refused while
/// the copy was rebuilt leaves no unresolved mass.
///
/// *Breaks if* a copy is rebuilt over the box again (the leaf refuses
/// and its mass is unresolved).
#[test]
fn a_wide_box_over_a_copy_certifies_whole() {
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
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .expect("builds");
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("{}", budget.render(&doc));
    assert_eq!(budget.unresolved.clone().expect("priced"), 0.0);
    assert!(!verdict.certified().is_empty());
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
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        1.0,
    );
    let placed = translate(
        &mut r,
        solid,
        [
            Formula::named(name("place"), Dimension::Length),
            len(0.0),
            Formula::named(name("lift"), Dimension::Length),
        ],
    );
    let measure = r
        .measure(
            &[MeasurePrimitive::Distance { a: 0, b: 1 }],
            &[
                SitedRef::new(
                    placed,
                    fixture::fname(solid, fixture::wall(&r.doc, solid, 0)),
                ),
                SitedRef::new(
                    placed,
                    fixture::fname(solid, fixture::wall(&r.doc, solid, 2)),
                ),
            ],
        )
        .outputs[0];
    r.insert(Node::Assertion { claim: editor_core::Claim::Value {
        value: fixture::read_var(&r.doc, measure),
        bound: len(1.0),
        relation: AssertionRelation::AtLeast,
    } });
    let analyzed = analyzed_box(&r.doc, &AnalysisPolicy::default());
    match MassBasis::of(&analyzed) {
        MassBasis::Forced { by } => {
            assert_eq!(by, vec![r.doc.var_named("lift").expect("declared")]);
        }
        other => panic!("{other:?}"),
    }
    let verdict =
        drive(&r.doc, &analyzed, &DriveConfig::default(), Tol::witness()).expect("builds");
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("mixed:\n{}\n{}", budget.render(&r.doc), budget.serialize());
    assert!(budget.render(&r.doc).contains("FORCED, not priced: lift"));
    // Now a band-only box that the drive must SPLIT: a starved split on
    // a band axis prices nothing, typed.
    let mut sub = BTreeMap::new();
    sub.insert(
        r.doc.var_named("lift").expect("declared"),
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

// ---------------------------------------------- 8. MC over a clearance

/// The MC lane over a clearance document: every sample is an `f64`
/// build, where a clearance reads its third state, so every sample is
/// undecided, the fraction is `None`, and the rendering says so with its
/// count and seed — an honest nothing.
#[test]
fn the_mc_lane_over_a_clearance_document_decides_nothing_and_says_so() {
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
    eprintln!("{}", report.render(&doc));
    assert!(report.measures.is_empty(), "no node holds a clearance value");
    assert_eq!(report.assertions[0].unevaluated, 16);
    assert!(report.assertions[0].violation_fraction().is_none());
    assert!(report.render(&doc).contains("no sample decided"));
}

// --------------------------------------------- 9. the consumer walk

/// A bracket: a base slab and a post parked above it, the post placed
/// by `offset` (uniform) and lifted by `lift` (normal). Two measures —
/// the web between the post's wall and the base's wall, and the
/// `min_clearance` between the two bodies — each with an assertion.
fn bracket(
    offset_law: Distribution,
    lift_law: Distribution,
) -> (ProfileDoc, editor_core::VarId, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    param(&mut r, "offset", 0.0, Some(offset_law));
    param(&mut r, "lift", 0.2, Some(lift_law));
    let base = prism(
        &mut r,
        &[(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (0.0, 1.0)],
        1.0,
    );
    let post_solid = prism(
        &mut r,
        &[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)],
        2.0,
    );
    let post = crate::m10_5_clearance_interval::slid_by(
        &mut r,
        base,
        post_solid,
        [
            Formula::named(name("offset"), Dimension::Length),
            Formula::zero(Dimension::Length),
            Formula::sub(
                len(1.0),
                Formula::neg(Formula::named(name("lift"), Dimension::Length))
                    .expect("a shallow negation"),
            )
            .expect("length"),
        ],
    );
    // web = distance(post's x=1 wall, base's x=0 wall) = 1 + offset.
    let web = r
        .measure(
            &[MeasurePrimitive::Distance { a: 0, b: 1 }],
            &[
                SitedRef::new(
                    post,
                    fixture::fname(post_solid, fixture::wall(&r.doc, post_solid, 3)),
                ),
                crate::fixture::sited_ref(fixture::fname(base, fixture::wall(&r.doc, base, 3))),
            ],
        )
        .outputs[0];
    let web_ok = r.insert(Node::Assertion { claim: editor_core::Claim::Value {
        value: fixture::read_var(&r.doc, web),
        bound: len(0.9),
        relation: AssertionRelation::AtLeast,
    } });
    let clear_ok = r.insert(Node::clearance(
        SitedRef::new(
            post,
            editor_core::StableName {
                kind: editor_core::EntityKind::Body,
                node: post_solid,
                path: vec![RoleSeg::OutputBody],
            },
        ),
        crate::fixture::sited_ref(editor_core::StableName {
            kind: editor_core::EntityKind::Body,
            node: base,
            path: vec![RoleSeg::OutputBody],
        }),
        ClearanceBounds::AtLeast(len(0.1)),
    ));
    (r.doc, web, web_ok, clear_ok)
}

/// The walk a consumer takes today, rendered as a consumer would read
/// it. EVIDENCE-ONLY beyond the shape assertions; the friction is in
/// the review report.
#[test]
fn the_bracket_walk_through_the_public_doors() {
    let tol = Tol::witness();
    let (doc, web, web_ok, clear_ok) = bracket(
        uniform(),
        Distribution::Normal {
            sigma: half() / 3.0,
        },
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), tol).expect("builds");
    eprintln!("== drive\n{}", verdict.render(&doc, &analyzed));
    assert!(
        !verdict.certified().is_empty(),
        "the bracket certifies at ε/64"
    );

    eprintln!("== stackup(web)");
    let report = stackup(&doc, web, &analyzed, &verdict, None, true, None, tol).expect("a stackup");
    eprintln!("{}", report.render(&doc, &analyzed));
    assert!(report.worst_case.lo <= 1.0 && 1.0 <= report.worst_case.hi);

    eprintln!("== fold(post vs base ≥ 0.1)");
    let fold = clearance_over(
        &doc,
        &analyzed,
        &verdict,
        &Selection::body_of(verdict_post(&doc)),
        &Selection::body_of(verdict_base(&doc)),
        &ClearanceQuery { bound: editor_core::clearance::ClearanceBound::at_least(0.1), ..ClearanceQuery::strictly_positive(tol) },
    );
    // **§2's two doors and the key, on the three reports that shipped
    // with no row at all** (R2's MINOR-10): `LeafFold`'s trio,
    // `ClearanceReport`'s — which had no `content_key` until the fix
    // pass, though claim 3 names it.
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
    let report = crate::fixture::clearance(
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
    eprintln!("{}", h.render(&doc));
    eprintln!("== assertions over the certified leaves");
    for leaf in verdict.certified() {
        let (a, b) = (
            on_lane(&doc, &verdict, leaf, web_ok),
            on_lane(&doc, &verdict, leaf, clear_ok),
        );
        assert_eq!(a.holds(), Some(true), "{a:?}");
        assert_eq!(b.holds(), Some(true), "{b:?}");
    }

    eprintln!("== mc");
    let mc = monte_carlo(&doc, &analyzed, &McConfig::default(), tol).expect("replays");
    eprintln!("{}", mc.render(&doc));
    assert_eq!(mc.measures.len(), 1, "the web; a clearance holds no value");

    eprintln!("== budget");
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("{}", budget.render(&doc));
    assert_eq!(budget.basis, MassBasis::Priced);

    // The same bracket with a BAND on the offset: the budget reads
    // forced, the MC lane refuses typed, and the stackup's rss is
    // unavailable.
    let (doc, web, _, _) = bracket(
        Distribution::Band {
            lo: -half(),
            hi: half(),
        },
        Distribution::Normal {
            sigma: half() / 3.0,
        },
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), tol).expect("builds");
    let budget = MassBudget::of(verdict.accounting(), &analyzed);
    eprintln!("== band budget\n{}", budget.render(&doc));
    assert!(matches!(budget.basis, MassBasis::Forced { .. }));
    assert!(monte_carlo(&doc, &analyzed, &McConfig::default(), tol).is_err());
    if let Ok(report) = stackup(&doc, web, &analyzed, &verdict, None, true, None, tol) {
        eprintln!("== band stackup\n{}", report.render(&doc, &analyzed));
    }
}

fn node_named(doc: &editor_core::ProfileDoc, pick: usize) -> RecipeNodeId {
    let mut placements: Vec<RecipeNodeId> = doc
        .ids()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(Node::Place { .. })))
        .collect();
    let mut extrudes: Vec<RecipeNodeId> = doc
        .ids()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(Node::Extrude { .. })))
        .collect();
    if pick == 0 {
        placements.remove(0)
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
    let plate_p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([
                (-4.0e-3, -2.0e-3),
                (4.0e-3, -2.0e-3),
                (4.0e-3, 2.0e-3),
                (-4.0e-3, 2.0e-3),
            ])
            .expect("plate"),
        ],
        ids: Vec::new(),
        on: None,
    }));
    let plate = r.insert(Node::Extrude {
        profile: plate_p.into(),
        distance: len(1.0e-3),
    });
    let hole = |r: &mut Recorder, centre: Formula, radius: &'static str| {
        let p = r.insert(Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [centre, len(0.0)],
                radius: Formula::named(name(radius), Dimension::Length),
            }],
            ids: Vec::new(),
            on: None,
        }));
        r.insert(Node::Extrude {
            profile: p.into(),
            distance: len(1.0e-3),
        })
    };
    let hole_a = hole(
        &mut r,
        Formula::sub(
            len(0.0),
            Formula::named(name("half_spacing"), Dimension::Length),
        )
        .expect("length"),
        "hole_a_r",
    );
    let hole_b = hole(
        &mut r,
        Formula::named(name("half_spacing"), Dimension::Length),
        "hole_b_r",
    );
    // Each hole joins the plate's space where it is drawn.
    let hole_a = r.against(plate, hole_a, &editor_core::Frame::IDENTITY);
    let hole_b = r.against(plate, hole_b, &editor_core::Frame::IDENTITY);
    let ev = eval_over::<f64>(&r.doc, None);
    let wall = |node: RecipeNodeId| {
        let mut faces = select_where(
            &ev,
            node,
            &Selector::of(NamePat::of_kind(EntityKind::Face)),
            &[GeomPred::SurfaceKind(SurfaceKindSet::just(
                geom::SurfaceKind::Cylinder,
            ))],
            &r.doc.var_env::<f64>(),
            tol,
        )
        .expect("exact atom");
        faces.sort();
        SitedRef::new(node, faces.remove(0))
    };
    let refs = vec![wall(hole_a), wall(hole_b)];
    let distance = r
        .measure(&[MeasurePrimitive::Distance { a: 0, b: 1 }], &refs)
        .outputs[0];
    let radius_of = |n: &'static str| Formula::named(name(n), Dimension::Length);
    let web = Formula::sub(
        r.len_of(distance),
        Formula::add(radius_of("hole_a_r"), radius_of("hole_b_r")).expect("L + L"),
    )
    .expect("L − L");
    let measure = r.define("measure", web);
    let assertion = r.insert(Node::Assertion { claim: editor_core::Claim::Value {
        value: fixture::read_var(&r.doc, measure),
        bound: len(bound),
        relation: AssertionRelation::AtLeast,
    } });

    let analyzed = analyzed_box(&r.doc, &AnalysisPolicy::default());
    let verdict = drive(&r.doc, &analyzed, &DriveConfig::default(), tol).expect("builds");
    assert!(!verdict.certified().is_empty());
    let report =
        stackup(&r.doc, measure, &analyzed, &verdict, None, true, None, tol).expect("stackup");
    eprintln!(
        "stop 2: worst_case [{:e}, {:e}] bound {bound:e} (lo − bound = {:e}, ε = {:e})",
        report.worst_case.lo,
        report.worst_case.hi,
        report.worst_case.lo - bound,
        tol.eps()
    );
    assert!(report.worst_case.lo < bound, "the caption's premise");
    for leaf in verdict.certified() {
        let v = on_lane(&r.doc, &verdict, leaf, assertion);
        eprintln!("stop 2: the assertion node over the certified leaf: {v:?}");
        assert!(
            matches!(v, AssertionVerdict::Holds { .. }),
            "the straddle is inside the coincidence band, so the node says HOLDS: {v:?}"
        );
    }
}
