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
//!    re-derived here by calling `clearance::min_separation` directly,
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
//! The two walls share no vertex, so the wedge rule admits the pair;
//! they are two faces of ONE body, which is also the case a
//! `min_clearance` between two named faces has to handle.
//!
//! The box every row drives over is ε-scaled for the reason M10-5's
//! header states at length: above a small fraction of ε no node's
//! interval replay survives, which is issue 1191's class and this
//! unit's honest limit as much as that one's.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::clearance::{MinSepSelection, MinSeparationConfig, min_separation};
use editor_core::drive::{DriveConfig, drive, SymbolicDials};
use editor_core::stackup::stackup;
use editor_core::{
    AssertionDir, AssertionVerdict, CancelToken, Dimension, Distribution, DocEdit, DocParam,
    EvalOptions, Expr, LoopProgram, MeasureExpr, MeasurePrimitive, MeasureRef,
    MeasureUnavailableAt, Node, NodeErrorKind, NodeResult, ParamName, ProfileDoc, ProfileProgram,
    RecipeNodeId, UnevaluatedReason, UnitSym, ValuePayload, evaluate,
};
use geom_core::{Bounds, Tol};

use fixture::{Recorder, len};

/// The clearance engine has no lane at the symbolic identity tier
/// (ERROR-DESIGN E12; `DriveRefusal::SymbolicClearanceUnsupported`, and
/// the deviation is issue 1276), so every drive over a `min_clearance`
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


/// The true minimum separation between the neck walls, by construction.
const NECK_GAP: f64 = 0.4;

/// The assertion's bound: comfortably under the gap, so `Holds` is the
/// answer a correct engine gives.
const BOUND: f64 = 0.3;

fn name(n: &str) -> ParamName {
    ParamName::new(n)
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
    }));
    let solid = r.insert(Node::Extrude {
        profile,
        distance: len(2.0),
    });
    // A rigid translation by the document parameter: identity rotation,
    // so every stored direction passes through the interval lane
    // exactly (M10-5's finding, and the reason its fixtures are placed
    // rather than sized).
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
    let measure = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::MinClearance { a: 0, b: 1 }),
            vec![
                MeasureRef::new(placed, fixture::fname(solid, fixture::wall(2))),
                MeasureRef::new(placed, fixture::fname(solid, fixture::wall(9))),
            ],
        )
        .expect("both indices in range"),
    );
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: Expr::literal(BOUND, Dimension::Length).expect("finite"),
        dir: AssertionDir::AtLeast,
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
    /// The transform the measure READS them at.
    placed: RecipeNodeId,
    measure: RecipeNodeId,
    assertion: RecipeNodeId,
}

/// The neck pair, read out of an evaluation the way the engine's
/// caller does: the placed body, and the two wall faces the names
/// resolve to AT the transform.
///
/// It exists so the rows can ask the engine the same question the
/// measure asks it, without either of them going through the other.
struct Neck<'v> {
    body: &'v topo::Body<geom_core::Interval>,
    at: RecipeNodeId,
    faces: [topo::entity::FaceKey; 2],
}

impl<'v> Neck<'v> {
    fn of(ev: &'v editor_core::Evaluation<geom_core::Interval>, f: &Dumbbell) -> Self {
        let Some(NodeResult::Ok(value)) = ev.result(f.placed) else {
            panic!("the placed body evaluated");
        };
        let ValuePayload::Body(body) = &value.payload else {
            panic!("a transform's value is one body");
        };
        let key = |seg: u32| {
            let n = fixture::fname(f.solid, fixture::wall(seg));
            let Some(editor_core::Entry::Unique(ent)) = value.name_table.lookup(&n) else {
                panic!("the wall name resolves uniquely at the placed node");
            };
            let editor_core::EntityKey::Face(k) = ent.key else {
                panic!("a wall name is a face");
            };
            k
        };
        Self {
            body,
            at: f.placed,
            faces: [key(2), key(9)],
        }
    }

    fn side(&self, which: usize) -> MinSepSelection<'_> {
        MinSepSelection {
            at: self.at,
            index: 0,
            body: self.body,
            faces: vec![self.faces[which]],
        }
    }
}

/// The leaf box: the one parameter at ±[`half`].
fn leaf() -> ParamBox {
    let mut axes = BTreeMap::new();
    axes.insert(
        name("place"),
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
        profile_lift: editor_core::ProfileLift::Guided,
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
            let MeasureUnavailableAt::NeedsEnclosure { verb, scalar, door } = reason;
            assert_eq!(*verb, "min_clearance");
            assert_eq!(*scalar, "f64");
            assert_eq!(*door, "clearance::min_separation");
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
            assert!(format!("{why}").contains("clearance::min_separation"));
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
/// `clearance::min_separation` itself, over the bodies the same
/// evaluation produced, and requires the two to agree bit for bit. A
/// path that computed the measure some other way — a sampled pair, a
/// midpoint distance, a second engine — fails here.
#[test]
fn the_interval_value_is_the_engine_bracket_re_derived() {
    let f = dumbbell();
    let ev = eval_over::<geom_core::Interval>(&f.doc, Some(leaf()));
    let Some(NodeResult::Ok(v)) = ev.result(f.measure) else {
        panic!("the measure node evaluated at the interval scalar");
    };
    let ValuePayload::Measure { value, dim } = &v.payload else {
        panic!("expected a measured value, got a {}", v.payload.kind_name());
    };
    assert_eq!(*dim, Dimension::Length);

    // The same question, asked of the engine directly.
    let neck = Neck::of(&ev, &f);
    let direct = min_separation(&neck.side(0), &neck.side(1), MinSeparationConfig::default())
        .expect("the neck pair is admitted");
    assert_eq!(value.lo().to_bits(), direct.lo().to_bits());
    assert_eq!(value.hi().to_bits(), direct.window_hi().to_bits());

    // And the bracket is a bracket: it contains the gap the geometry
    // was built with.
    assert!(
        value.lo() <= NECK_GAP && NECK_GAP <= value.hi(),
        "[{}, {}] must enclose the built gap {NECK_GAP}",
        value.lo(),
        value.hi()
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
    let ev = eval_over::<geom_core::Interval>(&f.doc, Some(leaf()));
    let neck = Neck::of(&ev, &f);
    let mut widths = Vec::new();
    for pairs in [2usize, 32, 512] {
        let m = min_separation(
            &neck.side(0),
            &neck.side(1),
            MinSeparationConfig {
                max_cell_pairs: pairs,
                ..MinSeparationConfig::default()
            },
        )
        .expect("the neck pair is admitted");
        assert!(
            m.lo() <= NECK_GAP && NECK_GAP <= m.window_hi(),
            "budget {pairs}: [{}, {}] must enclose {NECK_GAP}",
            m.lo(),
            m.window_hi()
        );
        assert!(
            m.lo() <= m.window_hi(),
            "budget {pairs}: a bracket is ordered"
        );
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
            m.window_hi(),
            m.window_hi() - m.lo(),
            m.receipt()
        );
        widths.push(m.window_hi() - m.lo());
        // The goldening form is exact bits and is stable across
        // repeats: the same query twice is the same text.
        let again = min_separation(
            &neck.side(0),
            &neck.side(1),
            MinSeparationConfig {
                max_cell_pairs: pairs,
                ..MinSeparationConfig::default()
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
    let verdict = drive(&doc, &analyzed, &numeric_lane(), Tol::witness())
        .expect("the witness build succeeds");
    assert!(
        !verdict.certified().is_empty(),
        "the ε-scaled box certifies at least one leaf; refused: {}",
        verdict.refused().len()
    );
    for leaf in verdict.certified() {
        let ev = eval_over::<geom_core::Interval>(&doc, Some(leaf.box_.clone()));
        let Some(NodeResult::Ok(a)) = ev.result(assertion) else {
            panic!("the assertion evaluated over a certified leaf");
        };
        let ValuePayload::Assertion(verdict) = &a.payload else {
            panic!("an assertion node carries a verdict");
        };
        assert_eq!(
            verdict.holds(),
            Some(true),
            "the neck clears {BOUND} over every certified leaf: {verdict:?}"
        );
    }
}

/// **Row 4b**: a pairing the wedge rule empties refuses typed.
///
/// One face against ITSELF: a face is never at a distance from itself,
/// so the only candidate is dropped and there is no separation to
/// enclose. The minimum over an empty set is `+∞`, and reporting that
/// as a clearance would be a number no geometry backs.
#[test]
fn a_pairing_the_wedge_rule_empties_refuses_typed() {
    let f = dumbbell();
    let ev = eval_over::<geom_core::Interval>(&f.doc, Some(leaf()));
    let neck = Neck::of(&ev, &f);
    match min_separation(&neck.side(0), &neck.side(0), MinSeparationConfig::default()) {
        Err(r) => assert_eq!(r.name(), "no_admitted_pair", "{r:?}"),
        Ok(m) => panic!("a face against itself has no pair: {}", m.serialize()),
    }
}

/// **Row 5**: a reference that names neither a body nor a face refuses
/// typed, naming what it found.
#[test]
fn a_selection_that_is_not_a_body_or_a_face_refuses_typed() {
    let mut r = Recorder::new();
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
        distance: len(1.0),
    });
    let measure = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::MinClearance { a: 0, b: 1 }),
            vec![
                // A real EDGE name — the extrude's own lateral edge at
                // profile vertex 0 — so the reference resolves and the
                // refusal is about its KIND rather than about a name
                // that names nothing.
                MeasureRef::at_mint(fixture::prism_edges(solid, 4).remove(2)),
                MeasureRef::at_mint(fixture::fname(solid, fixture::wall(2))),
            ],
        )
        .expect("both indices in range"),
    );
    let ev = eval_over::<geom_core::Interval>(&r.doc, None);
    let Some(NodeResult::Failed(err)) = ev.result(measure) else {
        panic!("an edge is not a selection, so the measure refuses");
    };
    assert!(
        matches!(
            err.kind,
            NodeErrorKind::MeasureSelectionKind {
                verb: "min_clearance",
                found: "an edge"
            }
        ),
        "typed, naming what it found: {}",
        err.kind
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
    let verdict = drive(&f.doc, &analyzed, &numeric_lane(), Tol::witness())
        .expect("the nominal builds");
    assert!(!verdict.certified().is_empty(), "the box certifies");
    let report = stackup(
        &f.doc,
        f.measure,
        &analyzed,
        &verdict,
        None,
        true,
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
                said.contains("clearance::min_separation"),
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
    let human = report.render(&analyzed);
    assert!(
        human.contains("UNAVAILABLE") && human.contains("still gates"),
        "the human form says the column forfeited AND that the gate stands: {human}"
    );
}
