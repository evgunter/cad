//! **The certified locally-valid range** — the on-demand search that
//! proves instead of sampling (`editor_core::range`).
//!
//! Everything here goes through the public doors a consumer has:
//! `admit` and `derive` for the request, `RangeSearch` and
//! `certified_range` for the answer, the answer's accessors for reading
//! it.
//!
//! The answer as a FUNCTION of a side's intervals, over lists no
//! document produces (a standing change in the evidence —
//! `work/verdict/coincidence-zone-priced-budget-at-the-floor`), is
//! `range::tests`, in the crate.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::{BTreeMap, BTreeSet};

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{BudgetKind, DriveConfig, RefusalReason};
use editor_core::range::{
    CertifiedRange, DifferenceKind, RangeBudget, RangeField, RangeRefusal, RangeSearch, RangeSeed,
    Step, Unproven, admit, certified_range, derive,
};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, EvalOptions, Evaluation, Formula, FreeVar,
    LoopProgram, Node, NodeResult, ProfileDoc, ProfileProgram, RecipeNodeId, SlotId, SpokenNode,
    StableName, VarName, evaluate,
};

use fixture::{Recorder, len, scl, tol};

fn name(n: &'static str) -> VarName {
    VarName::from_static(n)
}

/// The variable `doc` declares as `n`.
fn var(doc: &editor_core::ProfileDoc, n: &'static str) -> editor_core::VarId {
    doc.var_named(n).expect("the fixture declares it")
}

fn param(n: &'static str) -> Formula {
    Formula::named(name(n), Dimension::Length)
}

/// The search's budgets, both of them the caller's: how many times a
/// side may be halved, and how many replays it may take.
fn budget(max_depth: u32, max_replays: usize) -> RangeBudget {
    RangeBudget::new(max_depth, max_replays)
}

fn declare(r: &mut Recorder, n: &'static str, value: f64) {
    r.push(DocEdit::DeclareVar {
        name: name(n),
        def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, value)),
    });
}

fn unit_square() -> LoopProgram<Formula> {
    LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
        .expect("finite square corners")
}

/// **The slab**: a unit square extruded by a document parameter. A
/// depth is signed, and refuses at zero and in the band about it: the
/// slab's field has one boundary, a point floor at depth zero, which a
/// seed about `1.0` does not reach.
fn slab(depth: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "depth", depth);
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![unit_square()],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: param("depth"),
    });
    r.doc
}

/// **The branch fixture**: a unit square whose bottom side has a
/// vertex at its midpoint, the vertex's height a document parameter,
/// extruded.
///
/// Below zero the vertex is a convex point; past zero ([`THROUGH`]) it
/// is a notch, and the walls either side of it turn the other way:
/// every node still builds and the body is a perfectly good solid, on
/// a branch the witness's recorded decisions do not describe. That is
/// the difference between "nothing new fails", which the probe asks,
/// and "nothing decides differently", which a certificate asserts.
fn notch(height: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "height", height);
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::polygon_expr([
            [len(0.0), len(0.0)],
            [len(0.5), param("height")],
            [len(1.0), len(0.0)],
            [len(1.0), len(1.0)],
            [len(0.0), len(1.0)],
        ])],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    r.doc
}

/// The height at which [`notch`]'s vertex is collinear with its
/// neighbours.
const THROUGH: f64 = 0.0;

/// The heights below [`THROUGH`] at which two of [`notch`]'s lines
/// stop or start lying apart along one's axis, each a `segment_gap`
/// decision that changes sign (`profile::seg`'s `line_line` reads that
/// gap before any side): `(1 − √2)/2`, where `(0.5, h)–(1, 0)`'s axis
/// reads the left wall's top at its own start, and `−1/2`, where
/// `(0, 0)–(0.5, h)`'s reads the top edge's start `(1, 1)` at its own
/// start. The pair's contacts are none on either side of both.
const GAP_ZEROS: [f64; 2] = [(1.0 - std::f64::consts::SQRT_2) / 2.0, -0.5];

/// The notch's nominal height, a convex point below the square, above
/// both [`GAP_ZEROS`].
const NOMINAL: f64 = -0.125;

/// The notch's seed: from a deeper point still clear of the
/// [`GAP_ZEROS`] to just past the crossing, at offset
/// `THROUGH − NOMINAL`, so the crossing is the one boundary inside it.
const NOTCH_SEED: RangeSeed = RangeSeed {
    lo: -0.0625,
    hi: 0.15,
};

/// The same document with the extrusion distance left as a bare
/// LITERAL — a field with no name at all until the query derives one.
/// A slab whose depth slot reads a variable DEFINED by a formula over
/// the document's `depth`.
fn defined_slab() -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "depth", 1.0);
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![unit_square()],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: Formula::mul(param("depth"), scl(2.0)).expect("a length times a scalar"),
    });
    r.doc
}

fn slab_slot(depth: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![unit_square()],
        ids: Vec::new(),
        on: None,
    }));
    let e = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(depth),
    });
    (r.doc, e)
}

/// A slab with a SECOND, annotated parameter that the query is not
/// about — the document the one-field contract has to pin.
fn two_param_slab() -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "depth", 1.0);
    r.push(DocEdit::DeclareVar {
        name: name("side"),
        def: editor_core::VarDecl::Free(FreeVar::continuous_with(
            Dimension::Length,
            1.0,
            Distribution::Uniform { lo: -0.1, hi: 0.1 },
        )),
    });
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::polygon_expr([
            [len(0.0), len(0.0)],
            [param("side"), len(0.0)],
            [param("side"), param("side")],
            [len(0.0), param("side")],
        ])],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: param("depth"),
    });
    r.doc
}

/// The structural slot [`patterned`]'s node carries: the index of its
/// read of one member.
const INDEX: SlotId = SlotId::Index {
    seat: editor_core::OperandSlot::Source,
    k: 0,
};

/// A document whose one interesting slot is STRUCTURAL: a row of three
/// and a copy of its member 1, whose index ([`INDEX`]) is a count.
fn patterned() -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![unit_square()],
        ids: Vec::new(),
        on: None,
    }));
    let e = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(0.5),
    });
    let (_, row) = r.linear(e, [1.0, 0.0, 0.0], 2.0, 3);
    let read = r.member(row, 1);
    (r.doc, read)
}

fn f64_run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol(),
    )
}

fn failing(ev: &Evaluation<f64>) -> BTreeSet<RecipeNodeId> {
    ev.nodes
        .iter()
        .filter(|(_, r)| matches!(r, NodeResult::Failed(_)))
        .map(|(id, _)| *id)
        .collect()
}

/// The PROBE's question at one value of one parameter: does this
/// document fail anywhere the document at the nominal did not?
fn no_new_failure(doc: &editor_core::ProfileDoc, p: &'static str, value: f64) -> bool {
    let baseline = failing(&f64_run(doc));
    let moved = editor_core::apply(
        doc,
        &DocEdit::SetVarValue {
            var: name(p).into(),
            value: editor_core::FreeValue::Continuous(value),
        },
        tol(),
    )
    .expect("a value edit applies")
    .doc;
    failing(&f64_run(&moved)).is_subset(&baseline)
}

/// Every node's standing and every published name of one f64 run —
/// the two halves of "the derived witness is the document's own".
fn standings_and_names(
    ev: &Evaluation<f64>,
) -> (
    BTreeMap<RecipeNodeId, &'static str>,
    BTreeMap<RecipeNodeId, BTreeSet<StableName>>,
) {
    let standings = ev
        .nodes
        .iter()
        .map(|(id, r)| {
            (
                *id,
                match r {
                    NodeResult::Ok(_) => "ok",
                    NodeResult::Failed(_) => "failed",
                    NodeResult::Poisoned { .. } => "poisoned",
                },
            )
        })
        .collect();
    let names = ev
        .nodes
        .iter()
        .filter_map(|(id, r)| {
            r.value().map(|_| {
                let tables = crate::fixture::tables(ev, *id);
                (
                    *id,
                    tables
                        .into_iter()
                        .flat_map(|t| t.iter())
                        .map(|(n, _)| n.clone())
                        .collect(),
                )
            })
        })
        .collect();
    (standings, names)
}

fn range_of(
    doc: &editor_core::ProfileDoc,
    p: &'static str,
    seed: RangeSeed,
    config: &RangeBudget,
) -> CertifiedRange {
    certified_range(
        doc,
        &RangeField::Param(var(doc, p)),
        seed,
        config,
        None,
        &editor_core::Stop::default(),
        tol(),
    )
    .expect("the fixture has an axis and a witness that builds")
}

// ---------------------------------------------------------- the answer

/// A side with nothing to find reaches the SEED's end, never
/// "unbounded": outside the seed nothing was replayed. The slab's two
/// sides each certify whole, so the answer is the answer after two
/// replays.
#[test]
fn a_side_with_nothing_to_find_reaches_the_seeds_end() {
    let doc = slab(1.0);
    let r = range_of(&doc, "depth", RangeSeed::symmetric(0.25), &budget(24, 2048));
    for (side, edge) in [(r.lo(), -0.25), (r.hi(), 0.25)] {
        assert!(side.reached_seed_end(), "got {side:?}");
        assert_eq!(side.proven_to(), edge);
        assert!(side.not_decided().is_none());
    }
    assert_eq!(r.proven_interval(), (0.75, 1.25));
    assert_eq!(r.nominal(), 1.0);
    assert_eq!(r.seed(), RangeSeed::symmetric(0.25));
    assert_eq!(r.steps(), 2, "each side is replayed whole, once");
}

/// **A boundary past undecided ground is PROVEN DIFFERENT.** The notch
/// builds on another branch past the crossing; the ground just short of
/// it is undecided at any depth, and the side still reports the point
/// beyond, with the predicate that decided otherwise and no standing
/// moved.
#[test]
fn a_boundary_past_undecided_ground_is_proven_different() {
    let doc = notch(NOMINAL);
    let r = range_of(&doc, "height", NOTCH_SEED, &budget(24, 2048));
    assert!(r.lo().reached_seed_end(), "got {:?}", r.lo());
    let hi = r.hi();
    let different = hi
        .different()
        .unwrap_or_else(|| panic!("the branch change is proven, got {hi:?}"));
    let crossing = THROUGH - NOMINAL;
    assert!(
        0.0 < hi.proven_to() && hi.proven_to() < crossing,
        "proven to {}",
        hi.proven_to()
    );
    assert!(
        crossing < different.at() && different.at() < NOTCH_SEED.hi,
        "the difference at {} is past the crossing at {crossing}",
        different.at()
    );
    let gap = hi
        .not_decided()
        .expect("undecided ground lies between the proof and the difference");
    assert_eq!(gap.past(), hi.proven_to());
    assert!(
        !gap.more_time_helps(),
        "a finished search's frontier is at its floor: {:?}",
        gap.reason()
    );
    assert_eq!(different.kind(), DifferenceKind::BuildsDifferently);
    let named: Vec<_> = different
        .evidence()
        .verdicts
        .nodes
        .values()
        .flat_map(|d| d.flip)
        .map(|f| (f.predicate, f.from, f.to))
        .collect();
    assert!(
        named.iter().any(|(p, from, to)| *p == "path_junction_turn"
            && *from == geom_core::Sign::Positive
            && *to == geom_core::Sign::Negative),
        "the flip was not named: {named:?}"
    );
}

/// A depth too small to reach the boundary leaves the side NOT DECIDED
/// — never a bound, and not something more replays move; the same
/// document with the depth raised proves the difference and more
/// ground.
#[test]
fn a_depth_too_small_is_not_decided_and_never_a_bound() {
    let doc = notch(NOMINAL);
    let starved = range_of(&doc, "height", NOTCH_SEED, &budget(1, 2048));
    let hi = starved.hi();
    assert!(hi.end().is_none(), "a starved side has no end: {hi:?}");
    let gap = hi.not_decided().expect("the starved side is undecided");
    assert_eq!(
        *gap.reason(),
        Unproven::Floor(RefusalReason::Budget(BudgetKind::Depth { max_depth: 1 }))
    );
    assert!(!gap.more_time_helps());
    assert!(
        0.0 < hi.proven_to() && hi.proven_to() < THROUGH - NOMINAL,
        "an undecided side still carries what it proved: {}",
        hi.proven_to()
    );

    let funded = range_of(&doc, "height", NOTCH_SEED, &budget(24, 2048));
    assert!(funded.hi().different().is_some(), "got {:?}", funded.hi());
    assert!(
        funded.hi().proven_to() > hi.proven_to(),
        "more depth proves more ground"
    );
}

/// **A line pair's gap is a boundary of its own.** From a nominal below
/// the first of the [`GAP_ZEROS`], the upward side is proven different
/// just past it, short of the crossing, and names `segment_gap` leaving
/// Negative: the two lines' projections go from overlapping to apart
/// there (through the zero band, which a coarse ε reaches first).
#[test]
fn a_gap_between_two_lines_is_proven_different_below_the_crossing() {
    let nominal = -0.25;
    let doc = notch(nominal);
    let r = range_of(&doc, "height", NOTCH_SEED, &budget(24, 2048));
    let hi = r.hi();
    let different = hi
        .different()
        .unwrap_or_else(|| panic!("the gap's zero is proven different, got {hi:?}"));
    // The undecided ground closes where the gap's margin enters the
    // band, which at a coarse ε lies short of its exact zero by about ε.
    let zero = GAP_ZEROS[0] - nominal;
    let resolution = NOTCH_SEED.hi / 2f64.powi(24);
    assert!(
        hi.proven_to() <= zero && zero - different.at() < 100.0 * resolution.max(tol().eps()),
        "the gap's zero at offset {zero} closes ({}, {})",
        hi.proven_to(),
        different.at()
    );
    let named: Vec<_> = different
        .evidence()
        .verdicts
        .nodes
        .values()
        .flat_map(|d| d.flip)
        .map(|f| (f.predicate, f.from, f.to))
        .collect();
    assert!(
        named.iter().any(|(p, from, to)| *p == "segment_gap"
            && *from == geom_core::Sign::Negative
            && *to != geom_core::Sign::Negative),
        "the gap's flip was not named: {named:?}"
    );
}

/// **A MEASUREMENT, and a defect's cost.** The ground between the proof
/// and the point proven different is undecided, and today it CONTAINS a
/// value at which the document does not build at all:
/// `drive::classify_replay` bisects an interval on which a node
/// definitely fails to the floor and prices it `Budget`
/// (`work/verdict/coincidence-zone-priced-budget-at-the-floor`). A
/// consumer rendering that ground as valid would be wrong, and when
/// that row lands this assertion turns over.
#[test]
fn the_undecided_ground_contains_a_value_that_does_not_build() {
    let doc = notch(NOMINAL);
    let r = range_of(&doc, "height", NOTCH_SEED, &budget(24, 2048));
    let hi = r.hi();
    let different = hi.different().expect("the hi side is proven different");
    let crossing = THROUGH - NOMINAL;
    assert!(
        hi.proven_to() < crossing && crossing < different.at(),
        "the crossing is in the undecided ground ({}, {})",
        hi.proven_to(),
        different.at()
    );
    assert!(
        !no_new_failure(&doc, "height", THROUGH),
        "at the crossing the vertex is collinear with its neighbours, which does not build"
    );
}

/// **The proof is inside the LOCALLY-VALID RANGE**, on a fixture with a
/// real boundary: every value it covers passes the sampling question
/// (does anything new fail), and the frontier stops at the driver's
/// floor short of the crossing rather than at it.
#[test]
fn the_proof_is_inside_the_locally_valid_range() {
    let doc = notch(NOMINAL);
    let r = range_of(&doc, "height", NOTCH_SEED, &budget(24, 2048));
    let (lo, hi) = r.proven_interval();
    assert!(lo < hi);
    for i in 0..=32 {
        let v = lo + (hi - lo) * f64::from(i) / 32.0;
        assert!(
            no_new_failure(&doc, "height", v),
            "the proof covers {v}, at which something new fails"
        );
    }
    // The bound is DERIVED from the two mechanisms that set the floor —
    // the run's ambiguity band and the search's resolution on this side
    // — because the gate runs three ε rows.
    let resolution = NOTCH_SEED.hi / 2f64.powi(24);
    assert!(hi < THROUGH, "the proof must not reach the crossing: {hi}");
    assert!(
        THROUGH - hi < 100.0 * resolution.max(tol().eps()),
        "the frontier is the driver's stopping point, not the boundary: {hi}"
    );
}

/// The certifying vector of the plain `f64` build at absolute value
/// `value`, built as the witness is (the derived document, the witness
/// options), less the `canonical_order_*` verdicts.
///
/// Those are left out because the leaf protocol replays the witness's
/// REPRESENTATION choices as recorded rather than deciding them afresh:
/// a `canonical_order_*` verdict picks where a loop's statement starts,
/// and a build at another value can record it differently and build the
/// same shape (`CertifiedRange::proven_offsets`,
/// `work/certread/the-leaf-protocol-pins-representation-choices-the-witness-recorded`).
fn decisions_at(doc: &ProfileDoc, value: f64) -> editor_core::VerdictVector {
    let at = editor_core::apply(
        doc,
        &DocEdit::SetVarValue {
            var: name("height").into(),
            value: editor_core::FreeValue::Continuous(value),
        },
        tol(),
    )
    .expect("a value edit")
    .doc;
    let field = RangeField::Param(var(&at, "height"));
    let derived = derive(&at, &field, RangeSeed::symmetric(1e-3), tol()).expect("admitted");
    let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
    let stop = editor_core::Stop::default();
    let built: Evaluation<f64> = evaluate(
        &derived.doc,
        None,
        &stop.cancel,
        &editor_core::drive::witness_options(&analyzed, &stop),
        tol(),
    );
    let mut vector = editor_core::drive::certifying_vector(&derived.doc, &built);
    for row in &mut vector.rows {
        row.verdicts
            .retain(|v| !v.predicate.starts_with("canonical_order_"));
    }
    vector
}

/// **Every value inside the proof decides as the witness did**: the
/// plain `f64` build's certifying vector, at the proof's two ends and at
/// points across it, is the witness's, on both fixtures with a boundary
/// either side (representation choices aside, [`decisions_at`]).
#[test]
fn every_value_inside_the_proof_decides_as_the_witness_did() {
    for (label, doc, seed) in [
        ("the notch", notch(NOMINAL), NOTCH_SEED),
        (
            "two boundaries",
            two_sided(NOMINAL),
            RangeSeed::symmetric(0.5),
        ),
    ] {
        let (_, _, r) = settle(&doc, seed, &budget(24, 1500));
        let witness = decisions_at(&doc, r.nominal());
        let (lo, hi) = r.proven_interval();
        assert!(
            lo < r.nominal() && r.nominal() < hi,
            "{label}: something is proven"
        );
        let inside = (0..=16).map(|i| lo + (hi - lo) * f64::from(i) / 16.0);
        for value in inside.chain([lo, hi]) {
            assert!(
                decisions_at(&doc, value) == witness,
                "{label}: the proof [{lo}, {hi}] covers {value}, whose build decides otherwise"
            );
        }
    }
}

/// Validity is not monotone, and the search says so by stopping: the
/// far branch builds and is NOT in the proof. A search around a value
/// on that branch proves it.
#[test]
fn the_first_boundary_stops_the_proof_and_a_search_on_the_island_proves_it() {
    let doc = notch(NOMINAL);
    let r = range_of(
        &doc,
        "height",
        RangeSeed { lo: -0.25, hi: 0.5 },
        &budget(24, 2048),
    );
    assert!(r.hi().different().is_some(), "got {:?}", r.hi());
    assert!(
        r.proven_interval().1 < THROUGH,
        "the proof must stop at the crossing, not leap it"
    );
    assert!(no_new_failure(&doc, "height", 0.2), "the far branch builds");

    let island = notch(0.2);
    let on_it = range_of(
        &island,
        "height",
        RangeSeed::symmetric(0.1),
        &budget(24, 2048),
    );
    assert!(on_it.lo().reached_seed_end() && on_it.hi().reached_seed_end());
    assert_eq!(on_it.proven_offsets(), (-0.1, 0.1));
}

// ---------------------------------------------------------- the search

fn notch_search(config: &RangeBudget) -> RangeSearch {
    let doc = notch(NOMINAL);
    RangeSearch::new(
        &doc,
        &RangeField::Param(var(&doc, "height")),
        NOTCH_SEED,
        config,
        None,
        &editor_core::Stop::default(),
        tol(),
    )
    .expect("the notch is admitted and its witness builds")
}

/// The answer after `n` replays of a fresh search.
fn after(n: usize) -> CertifiedRange {
    let mut search = notch_search(&budget(24, 2048));
    for _ in 0..n {
        assert_eq!(search.step(&editor_core::Stop::default()), Step::Advanced);
    }
    assert_eq!(search.steps(), n);
    search.answer()
}

/// **Stopping after `n` steps keeps what was proven**: the answer is an
/// answer at every step, never a refusal, its proven interval only
/// grows, and the ground it has not finished with is priced as the
/// budget of the steps taken. A replay that is cancelled decides
/// nothing and leaves the answer where it stood.
#[test]
fn stopping_after_n_steps_keeps_what_was_proven() {
    let mut search = notch_search(&budget(24, 2048));
    let before = search.answer();
    assert!(before.nothing_proven(), "no replay has run: {before:?}");
    assert_eq!(before.steps(), 0);
    for side in [before.lo(), before.hi()] {
        let gap = side.not_decided().expect("nothing is decided yet");
        assert_eq!(*gap.reason(), Unproven::Unfinished { replays: 0 });
        assert!(gap.more_time_helps());
    }

    let mut last = before;
    for n in 1..=12 {
        assert_eq!(search.step(&editor_core::Stop::default()), Step::Advanced);
        let now = search.answer();
        let ((lo0, hi0), (lo1, hi1)) = (last.proven_offsets(), now.proven_offsets());
        assert!(
            lo1 <= lo0 && hi0 <= hi1,
            "step {n} shrank the proof: {:?} then {:?}",
            (lo0, hi0),
            (lo1, hi1)
        );
        last = now;
    }
    assert!(
        last.lo().reached_seed_end() && last.hi().proven_to() > 0.0,
        "twelve replays prove the low side and some of the high: {last:?}"
    );
    let gap = last.hi().not_decided().expect("the boundary is not closed");
    assert_eq!(
        *gap.reason(),
        Unproven::Unfinished { replays: 12 },
        "unfinished ground is priced under the steps taken"
    );
    assert!(gap.more_time_helps());

    let cancelled = editor_core::Stop::default();
    cancelled.cancel.cancel();
    assert!(
        matches!(search.step(&cancelled), Step::Stopped(_)),
        "a cancelled replay stops"
    );
    assert_eq!(search.answer(), last, "and decides nothing");

    // The replay cap is the same stop, taken by the search itself.
    let capped = range_of(&notch(NOMINAL), "height", NOTCH_SEED, &budget(24, 12));
    assert_eq!(capped, last);
}

/// **The answer after `n` steps is a function of the inputs and `n`**:
/// two fresh searches agree at every count, and with the answer a
/// running search gave on the way past.
#[test]
fn the_same_steps_give_the_same_answer() {
    let mut running = notch_search(&budget(24, 2048));
    for n in 0..=9 {
        let here = running.answer();
        if [0, 1, 4, 9].contains(&n) {
            assert_eq!(after(n), here, "a fresh search differs after {n} steps");
            assert_eq!(after(n), here, "and a second fresh one");
        }
        assert_eq!(running.step(&editor_core::Stop::default()), Step::Advanced);
    }
    // Where the two sides tie — a symmetric seed's two whole sides —
    // the low side goes first, in every search: nothing outside the
    // inputs breaks the tie.
    let doc = slab(1.0);
    let one_step = || {
        let mut search = RangeSearch::new(
            &doc,
            &RangeField::Param(var(&doc, "depth")),
            RangeSeed::symmetric(0.25),
            &budget(24, 2048),
            None,
            &editor_core::Stop::default(),
            tol(),
        )
        .expect("the slab is admitted");
        assert_eq!(search.step(&editor_core::Stop::default()), Step::Advanced);
        search.answer()
    };
    let first = one_step();
    assert_eq!(
        first.proven_offsets(),
        (-0.25, 0.0),
        "the low side goes first"
    );
    for again in 0..3 {
        assert_eq!(
            one_step(),
            first,
            "fresh search {again} broke the tie another way"
        );
    }
}

/// **Two boundaries, one each side**: [`notch`]'s bottom vertex
/// `(0.5, h)`, which crosses at `h = 0`, and a top vertex
/// `(0.5, 1.3 + 2h)`, which crosses the top edge at `h = −0.15`.
fn two_sided(height: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "height", height);
    let top = Formula::add(
        len(1.3),
        Formula::mul(param("height"), scl(2.0)).expect("a length times a scalar"),
    )
    .expect("a length plus a length");
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::polygon_expr([
            [len(0.0), len(0.0)],
            [len(0.5), param("height")],
            [len(1.0), len(0.0)],
            [len(1.0), len(1.0)],
            [len(0.5), top],
            [len(0.0), len(1.0)],
        ])],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    r.doc
}

/// How many replays past the last move of its answer's ends a finished
/// search may have taken: [`editor_core::range::STALL_LEVELS`]
/// halvings, two replays each, at each of a side's two ends, on both
/// sides.
const SETTLE_SLACK: usize = 2 * editor_core::range::STALL_LEVELS as usize * 2 * 2;

/// A side's ends: the proof's, and the point proven different.
type Ends = (f64, Option<f64>);

fn ends(r: &CertifiedRange) -> [Ends; 2] {
    [r.lo(), r.hi()].map(|s| (s.proven_to(), s.different().map(|d| d.at())))
}

/// Steps a search to its end, and says how many replays it took and at
/// which replay its ends last moved.
fn settle(
    doc: &ProfileDoc,
    seed: RangeSeed,
    budget: &RangeBudget,
) -> (usize, usize, CertifiedRange) {
    let stop = editor_core::Stop::default();
    let mut search = RangeSearch::new(
        doc,
        &RangeField::Param(var(doc, "height")),
        seed,
        budget,
        None,
        &stop,
        tol(),
    )
    .expect("the fixture is admitted and its witness builds");
    let (mut last, mut moved) = (ends(&search.answer()), 0);
    while search.step(&stop) == Step::Advanced {
        let now = ends(&search.answer());
        if now != last {
            (last, moved) = (now, search.steps());
        }
    }
    (search.steps(), moved, search.answer())
}

/// **The search finishes soon after its answer stops moving, at any
/// depth**: on the notch at depth 48 and on two boundaries at the
/// default depth, it reports [`Step::Finished`] well short of a cap ten
/// times what it needs, within [`SETTLE_SLACK`] replays of the last
/// move of either side's ends, and says more time will not help.
#[test]
fn the_search_finishes_soon_after_its_answer_stops_moving() {
    for (label, doc, seed, depth) in [
        ("the notch", notch(NOMINAL), NOTCH_SEED, 48),
        (
            "two boundaries",
            two_sided(NOMINAL),
            RangeSeed::symmetric(0.5),
            editor_core::drive::DEFAULT_MAX_DEPTH,
        ),
    ] {
        let cap = 1500;
        let (steps, moved, answer) = settle(&doc, seed, &budget(depth, cap));
        assert!(
            steps < cap && steps <= moved + SETTLE_SLACK,
            "{label} at depth {depth}: {steps} replays, its ends last moved at replay {moved}"
        );
        for side in [answer.lo(), answer.hi()] {
            assert!(
                side.not_decided().is_none_or(|gap| !gap.more_time_helps()),
                "{label}: a finished search has nothing left to do: {side:?}"
            );
        }
    }
}

/// **One side does not starve the other.** On two boundaries at a depth
/// far past what either needs, a cap that the depth-24 search finishes
/// under proves the high side at least as far as that search does: work
/// is taken widest first, so the low side's deep refinement waits for
/// the high side's shallow one.
#[test]
fn one_side_does_not_starve_the_other() {
    let doc = two_sided(NOMINAL);
    let seed = RangeSeed::symmetric(0.5);
    let (steps, _, shallow) = settle(&doc, seed, &budget(24, 1500));
    let (_, _, deep) = settle(&doc, seed, &budget(64, steps));
    assert!(
        deep.hi().proven_to() >= shallow.hi().proven_to() && deep.lo().proven_to() < 0.0,
        "under {steps} replays at depth 64 the sides reached {:?}, at depth 24 {:?}",
        deep.proven_offsets(),
        shallow.proven_offsets()
    );
}

/// How many halvings of the plate's seed [`plate_seed`] the row below
/// allows: enough to reach a width that certifies with room to spare.
const PLATE_DEPTH: u32 = 12;

/// The replay cap both searches of the plate row are held to.
const PLATE_CAP: usize = 2 * PLATE_DEPTH as usize + 2;

/// The corpus plate, its hole radius, and a one-sided seed `32 ε` up.
fn plate() -> (ProfileDoc, RangeField, RangeSeed) {
    let doc = crate::corpus::plate_param::document().doc;
    let field = RangeField::Param(var(&doc, crate::corpus::plate_param::HOLE_R));
    let seed = RangeSeed {
        lo: 0.0,
        hi: 32.0 * tol().eps(),
    };
    (doc, field, seed)
}

/// **The directed search proves a non-empty interval on the corpus
/// plate's `hole_r` within a replay cap under which the level-by-level
/// drive certifies nothing.** The plate certifies only at about `ε/4`
/// (an arc rim's endpoint pinning), which is 2⁷ halvings of this seed:
/// about 2·7 replays towards the current value, and 2⁷ leaves across
/// the whole seed.
#[test]
fn the_directed_search_proves_ground_on_the_plate_the_level_drive_cannot() {
    let (doc, field, seed) = plate();
    let config = budget(PLATE_DEPTH, PLATE_CAP);
    let stop = editor_core::Stop::default();

    let mut search =
        RangeSearch::new(&doc, &field, seed, &config, None, &stop, tol()).expect("admitted");
    while search.answer().nothing_proven() && search.step(&stop) == Step::Advanced {}
    let proven = search.answer();
    assert!(
        proven.hi().proven_to() > 0.0,
        "the directed search proved nothing in {} replays: {proven:?}. While the leaf protocol \
         refuses the plate's narrow leaves as a flip that names nothing, this row is held by \
         work/boxfinish/a-witness-records-curved-premise-checks-the-interval-replay-skips-so-no-\
         curved-leaf-certifies",
        search.steps()
    );
    assert!(search.steps() <= PLATE_CAP);
    assert!(proven.lo().reached_seed_end(), "the low side has no width");

    // The level-by-level drive over the same seed, under the same cap.
    let derived = derive(&doc, &field, seed, tol()).expect("admitted");
    let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
    let level = DriveConfig {
        max_depth: PLATE_DEPTH,
        max_leaves: PLATE_CAP,
        ..DriveConfig::default()
    };
    let verdict = editor_core::drive::drive(&derived.doc, &analyzed, &level, &stop, tol())
        .expect("the plate builds");
    assert!(
        verdict.certified().is_empty(),
        "the level drive certified {} of {} leaves under the cap, so the cap separates nothing",
        verdict.certified().len(),
        verdict.certified().len() + verdict.refused().len()
    );
}

/// **The plate's narrow `hole_r` leaves certify against their witness**:
/// over `[0, ε/4]` and `[0, ε/8]` the one leaf of a depth-0 drive takes
/// every decision the `f64` witness took, check for check, so its
/// verdict vector is the witness's. The plate's holes are cylinder
/// walls, whose closed-form premises each lane logs where its role walk
/// decides them and nowhere else.
///
/// It says that the drive certifies and no more: what the certificate
/// is worth at the leaf's members is `witness_premises_rows`'
/// `members_of_the_plates_certified_leaf_build_as_the_witness_did`.
///
/// Red where the `f64` lane's interval re-read of a curved face runs on
/// the log (`topo::props`' `lifted_closed_form` without its `unlogged`
/// scope): the witness records each premise once more than the leaf,
/// and the leaf is refused `flip_crossing` with no turn named.
#[test]
fn the_plates_narrow_hole_radius_leaves_certify_against_their_witness() {
    let (doc, field, _) = plate();
    let stop = editor_core::Stop::default();
    for narrow in [4.0, 8.0] {
        let seed = RangeSeed {
            lo: 0.0,
            hi: tol().eps() / narrow,
        };
        let derived = derive(&doc, &field, seed, tol()).expect("admitted");
        let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
        let level = DriveConfig {
            max_depth: 0,
            ..DriveConfig::default()
        };
        let verdict = editor_core::drive::drive(&derived.doc, &analyzed, &level, &stop, tol())
            .expect("the plate builds");
        assert!(
            verdict.certified().len() == 1 && verdict.refused().is_empty(),
            "over [0, ε/{narrow}] the plate's one leaf does not certify: refused {:?}",
            verdict
                .refused()
                .iter()
                .map(|leaf| leaf.reason.class().name())
                .collect::<Vec<_>>()
        );
    }
}

/// **The per-replay cost on the corpus documents**, printed: the
/// measurement `range.rs`'s module docs record. Asserts nothing, so it
/// is not a gate:
/// `cargo nextest run -p editor-core --test all -E
/// 'test(=docm9_range::the_per_replay_cost_on_the_corpus)' --run-ignored
/// only --no-capture`.
#[test]
#[ignore = "a measurement, printed; run it by name"]
fn the_per_replay_cost_on_the_corpus() {
    let stop = editor_core::Stop::default();
    let die = crate::corpus::die::document().doc;
    let cube = die
        .ids()
        .into_iter()
        .find(|&id| matches!(die.node(id), Some(Node::Extrude { .. })))
        .expect("the die extrudes its cube");
    let (plate, hole_r, _) = plate();
    let rows = [
        ("plate_param", "hole_r", plate, hole_r),
        (
            "die",
            "pip_depth",
            die.clone(),
            RangeField::Param(var(&die, "pip_depth")),
        ),
        (
            "die",
            "the cube's Distance slot",
            die,
            RangeField::Slot {
                node: cube,
                slot: SlotId::Distance,
            },
        ),
    ];
    for (name, label, doc, field) in rows {
        let seed = RangeSeed::of(&doc, &field).expect("a seed");
        let t = std::time::Instant::now();
        let mut search = RangeSearch::new(&doc, &field, seed, &budget(64, 8), None, &stop, tol())
            .expect("admitted");
        let witness = t.elapsed();
        let mut times = Vec::new();
        loop {
            let t = std::time::Instant::now();
            if search.step(&stop) != Step::Advanced {
                break;
            }
            times.push(t.elapsed().as_secs_f64());
        }
        println!(
            "{name} ({} nodes) | {label} | seed {seed:?} | witness {witness:?} | replays {times:.2?} | proven {:?}",
            doc.len(),
            search.answer().proven_offsets()
        );
    }
}

// ----------------------------------------------------------- admission

/// **The admission door runs no geometry and says what the search would
/// refuse**: handed the caller's own build of a document that does not
/// build, it refuses as the witness would, naming the node; handed
/// none, or a build that stands, it admits.
#[test]
fn admission_refuses_a_document_that_does_not_build_from_the_callers_build() {
    let broken = slab(0.0);
    let field = RangeField::Param(var(&broken, "depth"));
    let seed = RangeSeed::symmetric(0.25);
    let built = f64_run(&broken);
    assert!(
        !failing(&built).is_empty(),
        "a depth of zero does not build"
    );
    let Err(RangeRefusal::Drive(refusal)) = admit(&broken, &field, seed, Some(&built)) else {
        panic!("a build with a failed node is not admitted");
    };
    assert!(
        matches!(
            *refusal,
            editor_core::drive::DriveRefusal::WitnessDoesNotBuild { .. }
        ),
        "got {refusal:?}"
    );
    // The search's own witness says the same thing.
    let searched = RangeSearch::new(
        &broken,
        &field,
        seed,
        &budget(4, 8),
        None,
        &editor_core::Stop::default(),
        tol(),
    )
    .map(|_| ())
    .expect_err("the witness does not build");
    assert_eq!(searched, RangeRefusal::Drive(refusal));
    // Without the build the door cannot know, and with a standing one
    // it admits.
    let admitted = admit(&broken, &field, seed, None).expect("the request itself is sound");
    assert_eq!(admitted.axis, var(&broken, "depth"));
    let sound = slab(1.0);
    let field = RangeField::Param(var(&sound, "depth"));
    let admitted =
        admit(&sound, &field, seed, Some(&f64_run(&sound))).expect("a standing build admits");
    assert_eq!(admitted.nominal, 1.0);
}

/// **The seed this module chooses** reaches from zero to twice the
/// current value, and one unit either way for a field at zero.
#[test]
fn the_chosen_seed_spans_zero_to_twice_the_current_value() {
    for (value, half) in [(0.25, 0.25), (-4.0, 4.0), (0.0, 1.0)] {
        let doc = slab(value);
        let seed = RangeSeed::of(&doc, &RangeField::Param(var(&doc, "depth"))).expect("a seed");
        assert_eq!(seed, RangeSeed::symmetric(half), "at {value}");
    }
    let (doc, node) = slab_slot(1.5);
    let field = RangeField::Slot {
        node,
        slot: SlotId::Distance,
    };
    let seed = RangeSeed::of(&doc, &field).expect("a seed");
    assert_eq!(seed, RangeSeed::symmetric(1.5));
    assert_eq!(
        RangeSeed::of(&doc, &RangeField::Slot { node, slot: INDEX }),
        Err(RangeRefusal::UnknownSlot {
            node: doc.spoken(node),
            slot: INDEX
        })
    );
}

// ------------------------------------------------- purity and identity

/// **A5.** The query writes nothing: the input document is
/// bit-identical and serializes identically after a range is taken.
#[test]
fn the_query_does_not_touch_the_input_document() {
    let (doc, node) = slab_slot(1.0);
    let before = doc.clone();
    let before_text = editor_core::persist::save(&doc, tol()).expect("the fixture saves");
    let _ = certified_range(
        &doc,
        &RangeField::Slot {
            node,
            slot: SlotId::Distance,
        },
        RangeSeed::symmetric(0.25),
        &budget(24, 2048),
        None,
        &editor_core::Stop::default(),
        tol(),
    )
    .expect("the slot widens");
    assert!(doc.bit_eq(&before));
    assert_eq!(
        editor_core::persist::save(&doc, tol()).expect("the fixture saves"),
        before_text
    );
}

/// **A5.** The derived document's f64 build IS the input's: the
/// synthetic parameter enters the environment through the same
/// `from_f64` the literal did, so every node's standing and every
/// published name match.
#[test]
fn the_derived_witness_is_the_documents_own_build() {
    let (doc, node) = slab_slot(1.0);
    let derived = derive(
        &doc,
        &RangeField::Slot {
            node,
            slot: SlotId::Distance,
        },
        RangeSeed::symmetric(0.25),
        tol(),
    )
    .expect("the slot widens");
    let (mine, theirs) = (
        standings_and_names(&f64_run(&doc)),
        standings_and_names(&f64_run(&derived.doc)),
    );
    assert_eq!(mine.0, theirs.0, "a node's standing moved");
    assert_eq!(mine.1, theirs.1, "a published name moved");
    assert!(!mine.0.is_empty());
}

/// **C3.** A slot widens through the free variable it reads (VR4):
/// the axis is the slot's own id, its nominal the written value's
/// bits, and the derived document names nothing, declares nothing and
/// rewrites no slot (`range-synthetic-name-mints-a-name`).
#[test]
fn the_slot_widens_its_own_variable() {
    let literal = 1.0_f64 + 2.0_f64.powi(-40);
    let (doc, node) = slab_slot(literal);
    let derived = derive(
        &doc,
        &RangeField::Slot {
            node,
            slot: SlotId::Distance,
        },
        RangeSeed::symmetric(0.25),
        tol(),
    )
    .expect("the slot widens");
    assert_eq!(derived.nominal.to_bits(), literal.to_bits());
    assert_eq!(
        Some(derived.axis),
        doc.slot(node, SlotId::Distance),
        "the axis is the variable the slot already reads"
    );
    let Some(FreeVar::Continuous { value, dim, .. }) = derived.doc.free(derived.axis) else {
        panic!("the slot reads a free continuous variable");
    };
    assert_eq!(value.to_bits(), literal.to_bits());
    assert_eq!(*dim, Dimension::Length);
    assert_eq!(
        derived.doc.slot(node, SlotId::Distance),
        doc.slot(node, SlotId::Distance),
        "the slot is not rewritten"
    );
    assert_eq!(
        derived.doc.var_names(),
        doc.var_names(),
        "no name is minted"
    );
    assert_eq!(
        derived.doc.var_ids(),
        doc.var_ids(),
        "no variable is declared"
    );
}

/// **A6.** A parameter field boxes directly: no rewrite, no synthetic
/// name, and the same four arms.
#[test]
fn a_parameter_field_boxes_directly() {
    let doc = slab(1.0);
    let derived = derive(
        &doc,
        &RangeField::Param(var(&doc, "depth")),
        RangeSeed::symmetric(0.25),
        tol(),
    )
    .expect("the parameter boxes");
    assert_eq!(derived.axis, var(&doc, "depth"));
    assert_eq!(derived.doc.vars().len(), doc.vars().len());
    assert_eq!(derived.doc.ids(), doc.ids());
    assert_eq!(derived.nominal, 1.0);
    let r = range_of(&doc, "depth", RangeSeed::symmetric(0.25), &budget(24, 2048));
    assert!(r.lo().reached_seed_end());
}

/// **A6.** A slot the rewrite cannot name refuses TYPED, at the door,
/// before anything is driven.
#[test]
fn a_slot_the_rewrite_cannot_name_refuses_typed() {
    let seed = RangeSeed::symmetric(0.25);
    let (doc, pattern) = patterned();
    assert_eq!(
        derive(
            &doc,
            &RangeField::Slot {
                node: pattern,
                slot: INDEX,
            },
            seed,
            tol()
        ),
        Err(RangeRefusal::StructuralSlot {
            node: doc.spoken(pattern),
            slot: INDEX
        })
    );
    // A slot reading a defined variable has no interval of its own:
    // the query refuses, naming the variables to certify instead.
    let driven = defined_slab();
    let extrude = *driven.ids().last().expect("the extrude is last");
    assert_eq!(
        derive(
            &driven,
            &RangeField::Slot {
                node: extrude,
                slot: SlotId::Distance
            },
            seed,
            tol()
        ),
        Err(RangeRefusal::SlotIsDefined {
            node: driven.spoken(extrude),
            slot: SlotId::Distance
        })
    );
    // A slot the node does not carry, and a node the document does not
    // carry.
    assert_eq!(
        derive(
            &driven,
            &RangeField::Slot {
                node: extrude,
                slot: SlotId::Radius
            },
            seed,
            tol()
        ),
        Err(RangeRefusal::UnknownSlot {
            node: driven.spoken(extrude),
            slot: SlotId::Radius
        })
    );
    let ghost = RecipeNodeId::new(0, 9999);
    assert_eq!(
        derive(
            &driven,
            &RangeField::Slot {
                node: ghost,
                slot: SlotId::Distance
            },
            seed,
            tol()
        ),
        Err(RangeRefusal::UnknownNode {
            node: SpokenNode::absent(ghost)
        })
    );
    assert_eq!(
        derive(
            &driven,
            &RangeField::Param(editor_core::VarId::new(0, 0)),
            seed,
            tol()
        ),
        Err(RangeRefusal::NotAContinuousParam {
            param: editor_core::SpokenVar::new(editor_core::VarId::new(0, 0), None)
        })
    );
}

/// **The door's read order**: whether THIS node carries the slot is
/// asked before what the slot id means in the vocabulary.
///
/// An index slot is structural everywhere, so a door that tested that
/// first told a caller "the index slot of node N is structural" about a
/// node with no such slot at all — the vocabulary's fact where the
/// caller asked about this document's.
#[test]
fn a_structural_slot_on_a_node_that_has_none_is_an_unknown_slot() {
    let (doc, node) = slab_slot(1.0);
    assert!(
        doc.node(node).and_then(|n| n.expr(INDEX)).is_none(),
        "an extrude reads no member"
    );
    assert_eq!(
        derive(
            &doc,
            &RangeField::Slot { node, slot: INDEX },
            RangeSeed::symmetric(0.25),
            tol()
        ),
        Err(RangeRefusal::UnknownSlot {
            node: doc.spoken(node),
            slot: INDEX
        })
    );
    // And a node that DOES carry it still refuses structural — the
    // reorder narrows the first door, it does not open the second.
    let (patterned_doc, pattern) = patterned();
    assert!(
        patterned_doc
            .node(pattern)
            .and_then(|n| n.expr(INDEX))
            .is_some()
    );
    assert_eq!(
        derive(
            &patterned_doc,
            &RangeField::Slot {
                node: pattern,
                slot: INDEX
            },
            RangeSeed::symmetric(0.25),
            tol()
        ),
        Err(RangeRefusal::StructuralSlot {
            node: patterned_doc.spoken(pattern),
            slot: INDEX
        })
    );
}

/// A profile step argument — whose label, `loop L step S · <arg>`, is
/// prose for a person and no identifier — widens like any other
/// continuous slot: through the variable it reads.
#[test]
fn a_profile_step_argument_widens() {
    let mut r = Recorder::new();
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![unit_square()],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    let doc = r.doc;
    let profile = doc.node(p).expect("the profile");
    let slot = profile
        .slots()
        .into_iter()
        .find(|s| matches!(s, SlotId::Profile { .. }) && doc.slot_value(p, *s).is_some())
        .expect("the square carries a literal step argument");
    assert!(
        VarName::new(slot.label().replace(' ', "_")).is_err(),
        "the fixture's premise: the label {:?} is not an identifier",
        slot.label()
    );
    let derived = derive(
        &doc,
        &RangeField::Slot { node: p, slot },
        RangeSeed::symmetric(0.1),
        tol(),
    )
    .unwrap_or_else(|e| panic!("the {} slot widens: {e}", slot.label()));
    assert_eq!(
        Some(derived.axis),
        doc.slot(p, slot),
        "the slot widens through its own variable"
    );
    assert_eq!(
        derived.doc.var_names(),
        doc.var_names(),
        "no name is minted"
    );
}

/// **The two guard arms**, which the doors cannot reach: each renders
/// both facts its payload carries, and ends as a kernel defect does,
/// since neither is anything the caller wrote.
#[test]
fn the_guard_refusals_render_what_they_carry() {
    let axis = RangeRefusal::SeedIsNotTheAnalyzedAxis {
        analyzed: (-0.5, 0.5),
        asked: (-0.25, 0.75),
    }
    .to_string();
    for fragment in ["-0.5", "0.5", "-0.25", "0.75"] {
        assert!(axis.contains(fragment), "{axis:?} omits {fragment}");
    }
    let axes = RangeRefusal::MoreThanOneAxisVaries { varying: 3 }.to_string();
    assert!(axes.contains('3'), "{axes:?} omits the count");
    for text in [&axis, &axes] {
        assert!(
            text.ends_with(geom_core::KERNEL_DEFECT_ENDING),
            "{text:?} does not end as a kernel defect"
        );
    }
}

/// **The answer carries its condition.** The derivation holds every
/// other annotated parameter at its nominal, so the names it pinned
/// ride on the certificate for a consumer to state.
#[test]
fn the_answer_names_the_parameters_it_pinned() {
    let doc = two_param_slab();
    let r = range_of(&doc, "depth", RangeSeed::symmetric(0.1), &budget(24, 64));
    assert_eq!(r.pinned(), [var(&doc, "side")]);
    // The axis itself is never pinned: it did not lose a spread, it
    // was given one.
    assert!(!r.pinned().contains(&var(&doc, "depth")));
    // A document with no other spread to drop pins nothing, which is
    // not the same claim as "nothing was checked".
    let plain = range_of(
        &slab(1.0),
        "depth",
        RangeSeed::symmetric(0.1),
        &budget(24, 64),
    );
    assert!(plain.pinned().is_empty());
}

/// A seed that does not bracket the field's current value refuses: the
/// witness is taken there, so a range around some other point is a
/// range of a different document.
#[test]
fn a_seed_that_does_not_bracket_the_nominal_refuses() {
    let doc = slab(1.0);
    for (lo, hi) in [
        (0.1, 0.5),
        (-0.5, -0.1),
        (0.0, 0.0),
        (f64::NAN, 1.0),
        (-1.0, f64::INFINITY),
    ] {
        // Compared on BITS, because a NaN seed is one of the cases and
        // `NaN != NaN` would make the equality pass for the wrong
        // reason — or, here, fail for one.
        let Err(RangeRefusal::SeedNotABracket { lo: glo, hi: ghi }) = derive(
            &doc,
            &RangeField::Param(var(&doc, "depth")),
            RangeSeed { lo, hi },
            tol(),
        ) else {
            panic!("seed [{lo}, {hi}] must refuse");
        };
        assert_eq!((glo.to_bits(), ghi.to_bits()), (lo.to_bits(), hi.to_bits()));
    }
}

/// **`RangeSeed::symmetric` is infallible, and the invariant still
/// holds at one door.** A negative, zero or non-finite half-width
/// builds a seed the derivation refuses, naming the offsets it was
/// handed — which is why the constructor does not refuse a second
/// time under a second spelling.
#[test]
fn a_symmetric_seed_of_a_bad_half_width_refuses_at_the_derivation() {
    let doc = slab(1.0);
    for w in [-1.0, 0.0, f64::NAN, f64::INFINITY] {
        let seed = RangeSeed::symmetric(w);
        let out = derive(&doc, &RangeField::Param(var(&doc, "depth")), seed, tol());
        assert!(
            matches!(out, Err(RangeRefusal::SeedNotABracket { .. })),
            "symmetric({w}) must refuse at the door, got {out:?}"
        );
    }
    assert_eq!(
        RangeSeed::symmetric(0.25),
        RangeSeed {
            lo: -0.25,
            hi: 0.25
        }
    );
}

// ------------------------------------------- the seed reaches the axis

/// The seed IS the analyzed axis, bit for bit — the stop-clause check
/// the query makes before it drives anything.
#[test]
fn the_seed_reaches_the_driver_as_the_analyzed_axis() {
    let doc = slab(1.0);
    let seed = RangeSeed { lo: -0.3, hi: 0.7 };
    let derived = derive(&doc, &RangeField::Param(var(&doc, "depth")), seed, tol())
        .expect("the parameter boxes");
    let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
    let axis = analyzed.get(derived.axis).expect("the axis is there");
    assert_eq!(axis.offsets.lo.to_bits(), seed.lo.to_bits());
    assert_eq!(axis.offsets.hi.to_bits(), seed.hi.to_bits());
    assert_eq!(
        axis.distribution,
        Some(Distribution::Band {
            lo: seed.lo,
            hi: seed.hi
        })
    );
}

/// The query's contract is ONE field: every other parameter of the
/// derived document is pinned at its nominal, whatever the input
/// declared.
#[test]
fn every_other_parameter_is_pinned() {
    let doc = two_param_slab();
    assert_eq!(
        analyzed_box(&doc, &AnalysisPolicy::default())
            .varying()
            .count(),
        1,
        "the fixture's OTHER parameter is the annotated one"
    );
    let derived = derive(
        &doc,
        &RangeField::Param(var(&doc, "depth")),
        RangeSeed::symmetric(0.1),
        tol(),
    )
    .expect("the parameter boxes");
    let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
    let varying: Vec<editor_core::VarId> = analyzed.varying().map(|(n, _)| n).collect();
    assert_eq!(varying, vec![var(&doc, "depth")]);
    assert!(
        matches!(
            derived.doc.free_named("side"),
            Some(FreeVar::Continuous {
                distribution: None,
                ..
            })
        ),
        "the other parameter keeps its value and loses its spread"
    );
}
