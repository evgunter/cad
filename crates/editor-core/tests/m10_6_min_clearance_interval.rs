//! **The `min_clearance` measure primitive and the `min_separation`
//! engine door** (M10-6 §1; ERROR-DESIGN E3's last v1 primitive, E7's
//! engine).
//!
//! What this suite claims, in the order the rows make the claims:
//!
//! 1. At a POINT scalar the measure has no value and says so, the
//!    document still builds, and the assertion over it reports E10's
//!    third state carrying that reason.
//! 2. At the INTERVAL scalar the value IS the engine's bracket —
//!    re-derived here by calling `topo::clearance::min_separation` directly,
//!    so the row would fail if the evaluation path computed anything
//!    else.
//! 3. The bracket is BUDGET-HONEST: a smaller budget widens it and
//!    never falsifies it, and it always contains the true separation.
//! 4. A drive over a document carrying the measure certifies leaves,
//!    and the assertion holds over them.
//! 5. A reference that is not a body or a face refuses typed.
//!
//! # The fixture, and why it is the dumbbell
//!
//! M10-5's own acceptance shape: two blobs joined by a neck whose two
//! facing walls sit at `y = 0.8` and `y = 1.2`, so the true minimum
//! separation between them is exactly `0.4` — a number written into
//! the geometry rather than measured out of it, which is what lets
//! every row below compare against the truth instead of against a
//! previous run.
//!
//! The two walls share no vertex, so the pair is measured by
//! subdivision rather than read off shared topology; they are two
//! faces of ONE body, which is also the case a `min_clearance` between
//! two named faces has to handle.
//!
//! The box every row drives over is ε-scaled for the reason M10-5's
//! header states at length: above a small fraction of ε no node's
//! interval replay survives, which is issue 1191's class and this
//! unit's honest limit as much as that one's.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, SymbolicDials, drive};
use editor_core::stackup::stackup;
use editor_core::{
    AssertionRelation, AssertionVerdict, CancelToken, Dimension, Distribution, DocEdit,
    EvalOptions, Formula, FreeVar, LoopProgram, MeasurePrimitive, MeasureUnavailableAt,
    MinClearanceOperand, Node, NodeResult, ProfileDoc, ProfileProgram, RecipeNodeId, SitedRef,
    UnevaluatedReason, UnitSym, ValuePayload, VarName, evaluate,
};
use geom_core::{Bounds, Tol};
use topo::clearance::{Budgets, min_separation};

use fixture::{Recorder, len};

/// The evaluation's own band, which the engine decides its cells at.
fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).expect("the witness tolerance has a band")
}

/// The true minimum separation between the neck walls, by construction.
const NECK_GAP: f64 = 0.4;

/// The assertion's bound: comfortably under the gap, so `Holds` is the
/// answer a correct engine gives.
const BOUND: f64 = 0.3;

fn name(n: &'static str) -> VarName {
    VarName::from_static(n)
}

/// The analysis half-width, in metres — M10-5's, for M10-5's reason.
fn half() -> f64 {
    Tol::witness().eps() / 64.0
}

/// The dumbbell, its `min_clearance` measure over the two neck walls,
/// and an assertion that the neck clears [`BOUND`].
///
/// Authored through the public doors a user has: the walls by the names
/// the extrude minted, the measure as one primitive leaf, the assertion
/// as a node over it.
fn dumbbell() -> Dumbbell {
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
            LoopProgram::polygon(
                [
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
                ]
                .into_iter(),
            )
            .expect("finite corners"),
        ],
        ids: Vec::new(),
        on: None,
    }));
    let solid = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(2.0),
    });
    // A rigid translation by the document parameter: identity rotation,
    // so every stored direction passes through the interval lane
    // exactly (M10-5's finding, and the reason its fixtures are placed
    // rather than sized).
    let placed = crate::m10_5_clearance_interval::slid_by(
        &mut r,
        solid,
        solid,
        [
            Formula::named(name("place"), Dimension::Length),
            Formula::zero(Dimension::Length),
            Formula::zero(Dimension::Length),
        ],
    );
    let measured = r.measure(
        &[MeasurePrimitive::MinClearance { a: 0, b: 1 }],
        &[
            SitedRef::new(
                placed,
                fixture::fname(solid, fixture::wall(&r.doc, solid, 2)),
            ),
            SitedRef::new(
                placed,
                fixture::fname(solid, fixture::wall(&r.doc, solid, 9)),
            ),
        ],
    );
    let (measure, measure_value) = (measured.measures[0], measured.outputs[0]);
    let assertion = r.insert(Node::Assertion {
        value: fixture::read_var(&r.doc, measure_value),
        bound: len(BOUND),
        relation: AssertionRelation::AtLeast,
    });
    Dumbbell {
        doc: r.doc,
        solid,
        placed,
        measure,
        assertion,
    }
}

/// The fixture's node ids, named rather than positional: the rows read
/// three of them and a tuple of four would be read by index.
struct Dumbbell {
    doc: ProfileDoc,
    /// The extrude that MINTED the wall names.
    solid: RecipeNodeId,
    /// The placement the measure READS them at.
    placed: RecipeNodeId,
    measure: RecipeNodeId,
    assertion: RecipeNodeId,
}

/// The neck pair, read out of an evaluation the way the engine's
/// caller does: the placed body, and the two wall faces the names
/// resolve to AT the placement.
///
/// It exists so the rows can ask the engine the same question the
/// measure asks it, without either of them going through the other.
struct Neck<'v, T: geom_core::Decide> {
    body: &'v topo::Body<T>,
    at: RecipeNodeId,
    faces: [topo::entity::FaceKey; 2],
}

impl<'v, T: geom_core::Decide> Neck<'v, T> {
    fn of(ev: &'v editor_core::Evaluation<T>, f: &Dumbbell) -> Self {
        let Some(NodeResult::Ok(value)) = ev.result(f.placed) else {
            panic!("the placed body evaluated");
        };
        let ValuePayload::Body(body) = &value.payload else {
            panic!("a placement's value is one body");
        };
        let key = |seg: u32| {
            let n = fixture::fname(f.solid, fixture::wall(&f.doc, f.solid, seg));
            let Some(editor_core::Entry::Unique(ent)) =
                ev.table_at(f.placed).expect("one table").lookup(&n)
            else {
                panic!("the wall name resolves uniquely at the placed node");
            };
            let editor_core::EntityKey::Face(k) = ent.key else {
                panic!("a wall name is a face");
            };
            k
        };
        Self {
            body: body.body(),
            at: f.placed,
            faces: [key(2), key(9)],
        }
    }

    /// One wall of the neck, as the measure resolves a face-kind
    /// reference: the placed node's body 0, and that one face.
    fn operand(&self, which: usize) -> MinClearanceOperand<'_, T> {
        MinClearanceOperand {
            at: self.at,
            index: 0,
            body: self.body,
            faces: vec![self.faces[which]],
        }
    }

    /// The engine's bracket between two walls, asked the way the
    /// measure's interval lane asks it: the sides AND the same-body
    /// question through the operands' own doors, so a row here reds when
    /// the measure's derivation of either does.
    fn ask(
        &self,
        a: usize,
        b: usize,
        config: Budgets,
    ) -> Result<topo::clearance::MinSeparation, topo::clearance::MinSeparationRefusal>
    where
        T: geom_core::CertifiedBounds,
    {
        let (a, b) = (self.operand(a), self.operand(b));
        min_separation(&a.side(), &b.side(), a.same_body(&b), band(), config)
    }

    /// The neck's bracket through this scalar's door value, as the
    /// measure asks it.
    fn bracket(&self) -> T
    where
        T: topo::AtRestPolicy,
    {
        let lane = T::clearance_lane().expect("a certifying scalar holds the engine's door");
        let (a, b) = (self.operand(0), self.operand(1));
        lane.min_separation(
            &a.side(),
            &b.side(),
            a.same_body(&b),
            band(),
            Budgets::MIN_SEPARATION,
        )
        .expect("the neck pair is admitted")
        .expect("the neck pair witnesses an upper end")
    }
}

/// The leaf box: the one parameter at ±[`half`].
fn leaf(doc: &ProfileDoc) -> ParamBox {
    let mut axes = BTreeMap::new();
    axes.insert(
        doc.var_named("place").expect("the fixture declares it"),
        BoxAxis::Varying {
            lo: -half(),
            hi: half(),
        },
    );
    ParamBox::from_axes(axes)
}

/// Evaluates over a box at the caller's scalar, through the public door.
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

/// **Row 1**: at a point scalar there is no value, the document still
/// builds, and the assertion carries the reason.
#[test]
fn a_point_scalar_has_no_min_clearance_value_and_the_assertion_says_why() {
    let f = dumbbell();
    let (doc, measure, assertion) = (f.doc, f.measure, f.assertion);
    let ev = eval_over::<f64>(&doc, None);
    // THE DOCUMENT BUILDS. This is the half a node error would have
    // taken away: `drive`'s witness build refuses a document with any
    // failing node, so a `min_clearance` that FAILED at f64 would make
    // every document carrying one undriveable — and every f64 corpus
    // row over one red.
    assert!(
        ev.nodes.values().all(|r| matches!(r, NodeResult::Ok(_))),
        "a document carrying a min_clearance measure builds at f64"
    );
    let Some(NodeResult::Ok(v)) = ev.result(measure) else {
        panic!("the measure node evaluated");
    };
    match &v.payload {
        ValuePayload::MeasureUnavailable { reason, dim } => {
            assert_eq!(*dim, Dimension::Length);
            let MeasureUnavailableAt::NeedsEnclosure { verb, scalar, door } = reason else {
                panic!("at f64 the measure needs an enclosure, not {reason:?}");
            };
            assert_eq!(*verb, "min_clearance");
            assert_eq!(*scalar, "f64");
            assert_eq!(*door, "topo::ClearanceLane");
        }
        other => panic!("expected a typed absence, got a {}", other.kind_name()),
    }
    let Some(NodeResult::Ok(a)) = ev.result(assertion) else {
        panic!("the assertion node evaluated");
    };
    match &a.payload {
        ValuePayload::Assertion(AssertionVerdict::Unevaluated {
            reason: UnevaluatedReason::MeasureUnavailable(why),
        }) => {
            // The prose names the door rather than a number: a reader
            // is told where the answer lives.
            assert!(format!("{why}").contains("topo::ClearanceLane"));
        }
        other => panic!("expected E10's third state, got {other:?}"),
    }
    // And the verdict's own three-state accessor still says "no
    // verdict" rather than a silent pass.
    let ValuePayload::Assertion(verdict) = &a.payload else {
        unreachable!()
    };
    assert_eq!(verdict.holds(), None);
}

/// **Row 2**: at the interval scalar the value IS the engine's bracket.
///
/// Re-derived rather than compared to a recorded number: the row calls
/// `topo::clearance::min_separation` itself, over the bodies the same
/// evaluation produced, and requires the two to agree bit for bit. A
/// path that computed the measure some other way — a sampled pair, a
/// midpoint distance, a second engine — fails here.
#[test]
fn the_interval_value_is_the_engine_bracket_re_derived() {
    let f = dumbbell();
    let ev = eval_over::<geom_core::Interval>(&f.doc, Some(leaf(&f.doc)));
    let Some(NodeResult::Ok(v)) = ev.result(f.measure) else {
        panic!("the measure node evaluated at the interval scalar");
    };
    let ValuePayload::Measure { value, dim } = &v.payload else {
        panic!("expected a measured value, got a {}", v.payload.kind_name());
    };
    assert_eq!(*dim, Dimension::Length);

    // The same question, asked of the engine directly.
    let neck = Neck::of(&ev, &f);
    let direct = neck
        .ask(0, 1, Budgets::MIN_SEPARATION)
        .expect("the neck pair is admitted");
    assert_eq!(value.lo().to_bits(), direct.lo().to_bits());
    assert_eq!(
        value.hi().to_bits(),
        direct.hi().expect("an upper end").to_bits()
    );

    // And the bracket is a bracket: it contains the gap the geometry
    // was built with.
    assert!(
        value.lo() <= NECK_GAP && NECK_GAP <= value.hi(),
        "[{}, {}] must enclose the built gap {NECK_GAP}",
        value.lo(),
        value.hi()
    );
}

/// **The engine's door at the symbolic scalar.** The leaf replay's
/// `Body<Sym<Interval>>` goes to the same door value the interval lane
/// holds, and the bracket comes back as two computed values hulled:
/// bit-identical to the interval run's numbers, and a fresh unknown,
/// so two asks of one neck are two values the tier cannot prove equal.
/// A re-entry through `from_f64` would mint the same literal for each
/// end of both asks, so the two hulls would be one node.
#[test]
fn the_engines_door_runs_at_the_symbolic_scalar_and_re_enters_computed() {
    use geom_core::{Interval, Sym};
    let f = dumbbell();
    let budget = geom_core::SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let numeric = {
        let ev = eval_over::<Interval>(&f.doc, Some(leaf(&f.doc)));
        let neck = Neck::of(&ev, &f);
        let direct = neck
            .ask(0, 1, Budgets::MIN_SEPARATION)
            .expect("the neck pair is admitted")
            .enclosure()
            .expect("an upper end");
        let lane = neck.bracket();
        assert_eq!(
            lane.repr_bits(),
            direct.repr_bits(),
            "at Interval the door's re-entry is the engine's enclosure, bit for bit"
        );
        lane
    };
    let ((first, second), _) = geom_core::sym::with_session(budget, || {
        let ev = eval_over::<Sym<Interval>>(&f.doc, Some(leaf(&f.doc)));
        let neck = Neck::of(&ev, &f);
        (neck.bracket(), neck.bracket())
    });
    for (n, v) in [("first", first), ("second", second)] {
        assert_eq!(
            v.value.repr_bits(),
            numeric.repr_bits(),
            "{n} ask: the symbolic bracket's numbers are the interval run's"
        );
    }
    assert_ne!(
        first.node(),
        second.node(),
        "two asks with equal brackets are two unknowns, never one node"
    );
}

/// **Row 3**: the budget narrows the bracket and never falsifies it.
///
/// Three budgets over one fixture. Every bracket contains the truth,
/// the receipt identity holds at every one of them, and a bigger budget
/// is never a wider answer — which is the whole content of "honest at
/// any budget".
#[test]
fn the_budget_narrows_the_bracket_and_never_falsifies_it() {
    let f = dumbbell();
    let ev = eval_over::<geom_core::Interval>(&f.doc, Some(leaf(&f.doc)));
    let neck = Neck::of(&ev, &f);
    let mut widths = Vec::new();
    for pairs in [2usize, 32, 512] {
        let m = neck
            .ask(
                0,
                1,
                Budgets {
                    max_cell_pairs: pairs,
                    ..Budgets::MIN_SEPARATION
                },
            )
            .expect("the neck pair is admitted");
        let hi = m.hi().expect("a pair witnesses an upper end");
        assert!(
            m.lo() <= NECK_GAP && NECK_GAP <= hi,
            "budget {pairs}: [{}, {}] must enclose {NECK_GAP}",
            m.lo(),
            hi
        );
        assert!(m.lo() <= hi, "budget {pairs}: a bracket is ordered");
        assert!(
            m.receipt().holds(),
            "budget {pairs}: the receipt identity holds"
        );
        // The measured bracket at each budget, printed rather than
        // pinned: these are facts about this fixture on this kernel,
        // and a pinned width would be a baseline nothing re-takes.
        eprintln!(
            "min_separation @ max_cell_pairs={pairs}: [{}, {}] (width {:e}), receipt {:?}",
            m.lo(),
            hi,
            hi - m.lo(),
            m.receipt()
        );
        widths.push(hi - m.lo());
        // The goldening form is exact bits and is stable across
        // repeats: the same query twice is the same text.
        let again = neck
            .ask(
                0,
                1,
                Budgets {
                    max_cell_pairs: pairs,
                    ..Budgets::MIN_SEPARATION
                },
            )
            .expect("the neck pair is admitted");
        assert_eq!(m.serialize(), again.serialize());
    }
    assert!(
        widths[0] >= widths[1] && widths[1] >= widths[2],
        "a bigger budget is never a wider bracket: {widths:?}"
    );
}

/// **Row 4**: the drive certifies leaves over a document carrying the
/// measure, and the assertion holds over them.
///
/// This is the row the certification filter exists for: the assertion
/// reads `Unevaluated` at the f64 witness (no value there) and `Holds`
/// over every certified leaf, and those two rows differing is what used
/// to refuse every leaf as a topology flip.
#[test]
fn the_drive_certifies_and_the_assertion_holds_over_the_certified_leaves() {
    let f = dumbbell();
    let (doc, assertion) = (f.doc, f.assertion);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect("the witness build succeeds");
    assert!(
        !verdict.certified().is_empty(),
        "the ε-scaled box certifies at least one leaf; refused: {}",
        verdict.refused().len()
    );
    for leaf in verdict.certified() {
        let verdict = editor_core::drive::assertion_at(
            &doc,
            assertion,
            &leaf.box_,
            verdict.symbolic(),
            Tol::witness(),
        )
        .expect("the assertion evaluated over a certified leaf, on its lane");
        assert_eq!(
            verdict.holds(),
            Some(true),
            "the neck clears {BOUND} over every certified leaf: {verdict:?}"
        );
    }
}

/// **The symbolic tier drives a clearance measure**: the dumbbell
/// certifies with the tier on, and the certified worst case of its
/// `min_clearance` is bit-identical to the numeric run's. The leaf
/// replays at `Sym<Interval>` hand the engine's door that body, and the
/// tier changes how a margin decides, never a bracket.
#[test]
fn the_symbolic_tier_certifies_a_clearance_measure_bit_identical_to_the_numeric_run() {
    let f = dumbbell();
    let analyzed = analyzed_box(&f.doc, &AnalysisPolicy::default());
    let worst = |symbolic: SymbolicDials| {
        let config = DriveConfig {
            symbolic,
            ..DriveConfig::default()
        };
        let verdict =
            drive(&f.doc, &analyzed, &config, Tol::witness()).expect("the witness build succeeds");
        assert!(
            !verdict.certified().is_empty(),
            "the ε-scaled box certifies at least one leaf (tier on: {}); refused: {}",
            symbolic.enabled,
            verdict.refused().len()
        );
        let report = stackup(
            &f.doc,
            crate::fixture::output(&f.doc, f.measure),
            &analyzed,
            &verdict,
            None,
            true,
            None,
            Tol::witness(),
        )
        .expect("the report builds");
        let w = report.worst_case;
        (
            verdict.certified().len(),
            w.leaves,
            w.lo.to_bits(),
            w.hi.to_bits(),
        )
    };
    let on = worst(SymbolicDials::default());
    assert!(on.1 > 0, "the tier-on worst case reads certified leaves");
    assert_eq!(
        on,
        worst(SymbolicDials::off()),
        "(certified leaves, worst-case leaves, lo bits, hi bits) with the tier on, against off"
    );
}

/// **Row 4b**: two scopes that meet read 0, named on the cell they
/// share.
///
/// One face against ITSELF: the two scopes share that face, so the
/// minimum separation is 0, read off topology, and the bracket is
/// `[0, 0]` with the face as its witness. A metric door has no pairing
/// rule: a scope that meets another is at 0 from it.
///
/// The row is also what holds the measure's same-body derivation
/// (`MinClearanceOperand::same_body`): the meeting is read only between
/// two scopes of one body, and a derivation that answered `false` here
/// would subdivide the face against itself instead.
#[test]
fn two_scopes_that_meet_read_zero_on_the_shared_cell() {
    let f = dumbbell();
    let ev = eval_over::<geom_core::Interval>(&f.doc, Some(leaf(&f.doc)));
    let neck = Neck::of(&ev, &f);
    let m = neck
        .ask(0, 0, Budgets::MIN_SEPARATION)
        .expect("a face against itself is at 0");
    assert_eq!((m.lo(), m.hi()), (0.0, Some(0.0)), "{}", m.serialize());
    let meets = m.meets().expect("the scopes meet");
    assert_eq!(meets.a, meets.b, "on the one face: {meets:?}");
    assert!(
        matches!(meets.shared, topo::clearance::SharedCell::Face(f) if f == meets.a),
        "{meets:?}"
    );
}

/// **Row 5**: a reference that names neither a body nor a face refuses
/// at the door, as the seat's kind (FORK-VTX): a `min_clearance`
/// reference reads a `Body` or a `Face`, and a selection's kind is fixed
/// when it is minted, so the edge's selection is refused before any
/// evaluation sees it.
#[test]
fn a_selection_that_is_not_a_body_or_a_face_refuses_at_the_door() {
    let mut r = Recorder::new();
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
        distance: len(1.0),
    });
    // A real EDGE name — the extrude's own lateral edge at profile
    // vertex 0 — so the refusal is about its KIND rather than about a
    // name that names nothing.
    let edge = crate::fixture::sited_ref(fixture::prism_edges(&r.doc, solid, 4).remove(2));
    let face = crate::fixture::sited_ref(fixture::fname(solid, fixture::wall(&r.doc, solid, 2)));
    let refused = editor_core::measure(
        &r.doc,
        &[MeasurePrimitive::MinClearance { a: edge, b: face }],
        Tol::witness(),
    );
    let Err(err) = refused else {
        panic!("an edge is no min_clearance reference, so the insert refuses");
    };
    assert!(
        matches!(
            &err,
            editor_core::EditError::SlotVarKind {
                found: editor_core::VarKind::Edge,
                expected: editor_core::SlotKind::Measured(editor_core::MeasureVerb::MinClearance),
                ..
            }
        ),
        "typed, naming what it found: {err:?}"
    );
    assert!(
        err.to_string().contains("a body or a face"),
        "the refusal says what the reference admits: {err}"
    );
}

/// **§1 + E9: a stackup over a `min_clearance` BUILDS, with the
/// advisory columns forfeited** (M10-6's review, R2's MINOR-3).
///
/// The measure has no f64 value by construction, so the nominal, the
/// per-parameter table and the RSS — all advisory — have nothing to
/// report. The certified worst case does: it is the hull of the
/// measure's enclosures over the certified leaves, which is exactly
/// what the histogram already tabulates. E9's rule is that a degraded
/// advisory column forfeits and the gate stays, and this row is that
/// rule as a test: before it, the whole report refused
/// `MeasureRefusedAtNominal` because an advisory number was missing.
#[test]
fn a_stackup_over_a_min_clearance_forfeits_its_advisory_columns_and_still_gates() {
    let f = dumbbell();
    let analyzed = analyzed_box(&f.doc, &AnalysisPolicy::default());
    let verdict = drive(&f.doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect("the nominal builds");
    assert!(!verdict.certified().is_empty(), "the box certifies");
    let report = stackup(
        &f.doc,
        crate::fixture::output(&f.doc, f.measure),
        &analyzed,
        &verdict,
        None,
        true,
        None,
        Tol::witness(),
    )
    .expect("the report builds even though the nominal cannot");

    // The gating column is there and encloses the built gap.
    assert!(
        report.worst_case.lo <= NECK_GAP && NECK_GAP <= report.worst_case.hi,
        "the certified worst case [{}, {}] must enclose the built gap {NECK_GAP}",
        report.worst_case.lo,
        report.worst_case.hi
    );
    assert!(report.worst_case.leaves > 0);

    // And the advisory column forfeits BY NAME rather than silently.
    match report.nominal {
        Err(why) => {
            assert_eq!(why.verb(), "min_clearance");
            let said = why.to_string();
            assert!(
                said.contains("topo::ClearanceLane"),
                "the forfeit names the door that can answer: {said}"
            );
        }
        Ok(v) => panic!("a min_clearance has no f64 nominal, yet one arrived: {v}"),
    }

    // Both forms say so on their own face — the goldening one so a
    // diff shows it, the human one so a reader is not left to infer a
    // missing line.
    assert!(
        report
            .serialize()
            .contains("nominal=unavailable:min_clearance"),
        "the goldening form records the forfeit: {}",
        report.serialize()
    );
    let human = report.render(&f.doc, &analyzed);
    assert!(
        human.contains("UNAVAILABLE") && human.contains("still gates"),
        "the human form says the column forfeited AND that the gate stands: {human}"
    );
}
