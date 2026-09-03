//! **Two of M10-6's three E10 CI rows** (the third is a workflow step —
//! `ci.yml`'s `driver K-telemetry lint`, which needs a hosted k-lint
//! draw and so cannot live in a test binary).
//!
//! # Row 1 — assertion gating
//!
//! For every document carrying assertions, drive its recorded analyzed
//! box and require of EVERY assertion, over EVERY certified leaf:
//! `Holds`. A `Violated` fails the row naming the document, the
//! assertion node and the numbers; a verdict-less leaf fails it the
//! same way. And the drive's unresolved mass — refused plus tail — must
//! stay inside that document's RECORDED budget, with the budget's basis
//! (priced or forced) stated in the same table.
//!
//! **The registry is the corpus plus this file's own two documents**,
//! and the split is a deviation this unit discloses rather than hides.
//! A corpus document is a goldened artifact of eleven other suites
//! (digests, round trips, latency, the incremental probe), so
//! registering two more there to reach two node kinds would move every
//! one of those goldens for reasons none of them is about. The
//! COMPLETENESS obligation is kept where it matters: every corpus
//! document that carries an assertion must appear in [`BUDGETS`], and
//! `every_assertion_carrying_corpus_document_has_a_budget` fails when
//! one does not — so a new corpus document with an assertion cannot
//! quietly escape the row.
//!
//! # Row 2 — goldened accounting
//!
//! The serialized mass accounting of M10-3's two margin-thin fixtures,
//! bit-exact against a committed golden: the honesty metric is itself
//! regression-tested. Re-bless with `M10_6_BLESS_ACCOUNTING=1` (the
//! `m4_pr6_golden` procedure, and the file says so at its own head).
//!
//! **The goldens are keyed by ε row, and the reason is measured.**
//! Both fixtures are ε-relative — a box of `40ε` around a nominal of
//! `20ε` — and the accounting is made of MASSES, which are ratios, so
//! the VALUES are the same at every ε: the planted flip certifies
//! 0.6240 of its box at 1e-6, 1e-9 and 1e-12 alike. The BITS are not:
//! the leaf partition is `f64` arithmetic over an ε-scaled box, and
//! `0x3fe3f8000000000c` (1e-9) against `0x3fe3f7fffffffff5` (1e-6) is
//! four ulps of the same number. A bit-exact golden must therefore be
//! per-ε, and [`ACCOUNTING_GOLDENS`] carries the three rows the matrix
//! draws; a run at any other tolerance says so on the terminal and
//! compares nothing, which is the honest ε-scoped skip rather than
//! either a red over an unblessed tolerance or a silent pass.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, VerdictVector, drive};
use editor_core::report::{MassBasis, MassBudget};
use editor_core::{
    AssertionDir, AssertionVerdict, CancelToken, Dimension, Distribution, DocEdit, DocParam,
    EvalOptions, Expr, LoopProgram, MeasureExpr, MeasurePrimitive, MeasureRef, Node, NodeResult,
    ParamName, ProfileDoc, ProfileLift, ProfileProgram, RecipeNodeId, UnitSym, ValuePayload,
    evaluate,
};
use geom_core::Tol;

use fixture::{Recorder, len};

/// **The committed accounting goldens, ONE PER ε ROW** (row 2).
///
/// The masses are ratios and their VALUES are ε-invariant — both
/// fixtures are ε-relative, so scaling the box and its uniform law
/// together leaves every ratio where it was — but their BITS are not:
/// the leaf partition is computed in `f64` over an ε-scaled box, and
/// `20ε ± 40ε` does not scale exactly across three decades. Measured
/// 2026-09-03: the planted flip's certified mass is `0x3fe3f8000000000c`
/// at ε = 1e-9 and `0x3fe3f7fffffffff5` at ε = 1e-6 — the same 0.6240
/// to eleven digits, four ulps apart.
///
/// A bit-exact golden therefore has to be keyed by the ε row, and the
/// three rows are the three the matrix draws.
const ACCOUNTING_GOLDENS: &[(&str, &str, &str)] = &[
    (
        "1e-6",
        include_str!("golden/m10_6_accounting_1e-6.txt"),
        "tests/golden/m10_6_accounting_1e-6.txt",
    ),
    (
        "1e-9",
        include_str!("golden/m10_6_accounting_1e-9.txt"),
        "tests/golden/m10_6_accounting_1e-9.txt",
    ),
    (
        "1e-12",
        include_str!("golden/m10_6_accounting_1e-12.txt"),
        "tests/golden/m10_6_accounting_1e-12.txt",
    ),
];

/// **The recorded unresolved-mass budgets** (row 1): per document, the
/// most refused-plus-tail mass the row admits, and the BASIS that mass
/// is stated on.
///
/// A budget is a recorded constant, not a derived one: deriving it from
/// the run would make the row a tautology. Each entry says why it is
/// what it is.
const BUDGETS: &[Budget] = &[
    Budget {
        document: "measured_web",
        // Every parameter of the two-hole plate is FIXED (it declares
        // no distribution), so the analyzed box is a point, the drive
        // certifies it in one leaf, and there is no mass to leave
        // unresolved. Zero is the honest budget and a tight one: any
        // refusal at all fails this row.
        unresolved: 0.0,
        basis: "priced",
    },
    Budget {
        document: "min_clearance_neck",
        // The ε-scaled placement box certifies whole (issue 1191 is
        // about boxes WIDER than this), so the same reasoning gives
        // zero. The document's point is the min_clearance assertion,
        // not the mass.
        unresolved: 0.0,
        basis: "priced",
    },
    Budget {
        document: "band_placement",
        // A BAND parameter: the box is the band's own support, so the
        // drive's masses are what set theory forces rather than what a
        // measure priced — which is exactly why the basis column
        // exists. The budget is still zero because the box still
        // certifies whole; what the row checks here is that the basis
        // reads `forced`.
        unresolved: 0.0,
        basis: "forced",
    },
];

/// One row of [`BUDGETS`].
struct Budget {
    /// The document's name.
    document: &'static str,
    /// The most unresolved mass the row admits.
    unresolved: f64,
    /// `priced` or `forced` ([`MassBasis::word`]).
    basis: &'static str,
}

/// A document under row 1: its name and its recipe.
struct Entry {
    name: &'static str,
    doc: ProfileDoc,
}

/// The row's registry: every corpus document carrying an assertion,
/// then this file's own two.
fn registry() -> Vec<Entry> {
    let mut out: Vec<Entry> = corpus::documents()
        .into_iter()
        .filter(|d| carries_assertion(&d.doc))
        .map(|d| Entry {
            name: d.name,
            doc: d.doc,
        })
        .collect();
    out.push(Entry {
        name: "min_clearance_neck",
        doc: min_clearance_neck(),
    });
    out.push(Entry {
        name: "band_placement",
        doc: band_placement(),
    });
    out
}

fn carries_assertion(doc: &ProfileDoc) -> bool {
    doc.order()
        .iter()
        .any(|&id| matches!(doc.node(id), Some(Node::Assertion { .. })))
}

fn assertions_of(doc: &ProfileDoc) -> Vec<RecipeNodeId> {
    doc.order()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(Node::Assertion { .. })))
        .collect()
}

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

/// The ε-scaled half-width every parametric fixture here uses, for the
/// reason M10-5's suite header gives at length: no node's interval
/// replay survives a wider box.
fn half() -> f64 {
    Tol::witness().eps() / 64.0
}

/// A prism whose two neck walls are `0.4` apart, placed by a document
/// parameter, with a `min_clearance` measure over those two walls and
/// an assertion that the neck clears `0.3`.
///
/// The `min_clearance` entry row 1 is required to carry: its measure
/// has NO value at the f64 witness build (E10's third state), and
/// `Holds` over every certified leaf.
fn min_clearance_neck() -> ProfileDoc {
    let (doc, _) = neck_with(Distribution::Uniform {
        lo: -half(),
        hi: half(),
    });
    doc
}

/// The same shape with a BAND on the placement — limits with no shape,
/// so the drive's masses are forced rather than priced.
fn band_placement() -> ProfileDoc {
    let (doc, _) = neck_with(Distribution::Band {
        lo: -half(),
        hi: half(),
    });
    doc
}

fn neck_with(distribution: Distribution) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("place"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 0.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(distribution),
        },
    });
    let plane = r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let profile = r.insert(Node::Profile(ProfileProgram {
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
    r.insert(Node::Assertion {
        measure,
        bound: Expr::literal(0.3, Dimension::Length).expect("finite"),
        dir: AssertionDir::AtLeast,
    });
    (r.doc, measure)
}

/// **ROW 1.** Every assertion of every registered document holds over
/// every certified leaf, and the unresolved mass stays inside the
/// recorded budget.
#[test]
fn every_registered_assertion_holds_over_the_certified_leaves_within_budget() {
    for entry in registry() {
        let budget = BUDGETS
            .iter()
            .find(|b| b.document == entry.name)
            .unwrap_or_else(|| panic!("{} has no recorded budget", entry.name));
        let analyzed = analyzed_box(&entry.doc, &AnalysisPolicy::default());
        let verdict = match drive(
            &entry.doc,
            &analyzed,
            &DriveConfig::default(),
            Tol::witness(),
        ) {
            Ok(v) => v,
            // **The degenerate document, handled rather than skipped.**
            // A document whose parameters declare no distribution has
            // no box to subdivide, and the driver says so in exactly
            // those words rather than returning a one-leaf verdict that
            // would look like an analysis. Its assertions still have to
            // hold — at the NOMINAL, which for such a document IS the
            // whole answer — and its unresolved mass is zero by
            // construction, which is what its recorded budget says.
            // `measured_web` is this case today, and it is the first
            // entry the spec names.
            Err(editor_core::DriveRefusal::NothingVaries) => {
                assert_eq!(
                    budget.unresolved, 0.0,
                    "{}: nothing varies, so no mass can be unresolved; the recorded budget \
                     of {} claims otherwise",
                    entry.name, budget.unresolved
                );
                assert_eq!(
                    MassBasis::of(&analyzed).word(),
                    budget.basis,
                    "{}: the recorded basis and the box's disagree",
                    entry.name
                );
                let ev: editor_core::Evaluation<f64> = evaluate(
                    &entry.doc,
                    None,
                    &CancelToken::new(),
                    &EvalOptions {
                        profile_lift: ProfileLift::Guided,
                        ..EvalOptions::default()
                    },
                    Tol::witness(),
                );
                for node in assertions_of(&entry.doc) {
                    let Some(NodeResult::Ok(v)) = ev.result(node) else {
                        panic!("{}: assertion node {} did not evaluate", entry.name, node.0);
                    };
                    let ValuePayload::Assertion(verdict) = &v.payload else {
                        panic!("{}: node {} is not an assertion", entry.name, node.0);
                    };
                    assert_eq!(
                        verdict.holds(),
                        Some(true),
                        "{}: assertion node {} does not hold at the nominal, which for a \
                         document with nothing varying is the whole answer: {verdict:?}",
                        entry.name,
                        node.0
                    );
                }
                continue;
            }
            Err(e) => panic!("{}: the drive refused: {e}", entry.name),
        };

        // The BUDGET, first: a document whose mass ran away is not one
        // whose assertions mean anything.
        let mass = MassBudget::of(verdict.accounting(), &analyzed);
        assert_eq!(
            mass.basis.word(),
            budget.basis,
            "{}: the recorded basis is {} and the drive's is {} — a mass that is FORCED \
             (a band's limits) may not be reported as priced\n{}",
            entry.name,
            budget.basis,
            mass.basis.word(),
            mass.render()
        );
        let unresolved = mass
            .unresolved
            .clone()
            .unwrap_or_else(|e| panic!("{}: the budget cannot be priced: {e}", entry.name));
        assert!(
            unresolved <= budget.unresolved,
            "{}: unresolved mass {unresolved} exceeds the recorded budget {}\n{}",
            entry.name,
            budget.unresolved,
            mass.render()
        );
        assert!(
            !verdict.certified().is_empty(),
            "{}: nothing certified, so no assertion was checked anywhere\n{}",
            entry.name,
            mass.render()
        );

        // Then the assertions, leaf by leaf.
        let nodes = assertions_of(&entry.doc);
        assert!(!nodes.is_empty(), "{} carries no assertion", entry.name);
        for leaf in verdict.certified() {
            let opts = EvalOptions {
                param_box: Some(std::sync::Arc::new(leaf.box_.clone())),
                profile_lift: ProfileLift::Guided,
                ..EvalOptions::default()
            };
            let ev: editor_core::Evaluation<geom_core::Interval> =
                evaluate(&entry.doc, None, &CancelToken::new(), &opts, Tol::witness());
            for &node in &nodes {
                let verdict = match ev.result(node) {
                    Some(NodeResult::Ok(v)) => match &v.payload {
                        ValuePayload::Assertion(verdict) => verdict.clone(),
                        other => panic!(
                            "{}: assertion node {} evaluated to a {}",
                            entry.name,
                            node.0,
                            other.kind_name()
                        ),
                    },
                    other => panic!(
                        "{}: assertion node {} did not evaluate over a certified leaf: {other:?}",
                        entry.name, node.0
                    ),
                };
                match &verdict {
                    AssertionVerdict::Holds { .. } => {}
                    AssertionVerdict::Violated { measured, bound } => panic!(
                        "{}: assertion node {} is VIOLATED over a certified leaf: measured \
                         {measured:?} against bound {bound:?}",
                        entry.name, node.0
                    ),
                    AssertionVerdict::Unevaluated { reason } => panic!(
                        "{}: assertion node {} has no verdict over a certified leaf: {reason}",
                        entry.name, node.0
                    ),
                }
            }
        }
    }
}

/// **ROW 1's completeness half**: a corpus document that grows an
/// assertion cannot escape the row by not being in [`BUDGETS`].
#[test]
fn every_assertion_carrying_corpus_document_has_a_budget() {
    for d in corpus::documents() {
        if !carries_assertion(&d.doc) {
            continue;
        }
        assert!(
            BUDGETS.iter().any(|b| b.document == d.name),
            "corpus document {} carries an assertion and has no recorded unresolved-mass \
             budget in this row's table — add one with the argument for the number",
            d.name
        );
    }
}

/// **ROW 2.** The serialized accounting of the two margin-thin
/// fixtures, bit-exact.
///
/// # Re-blessing
///
/// Run with `M10_6_BLESS_ACCOUNTING=1`, inspect the diff, and commit it
/// WITH the change it records. A moved golden is either a driver change
/// (the leaf partition moved) or an accounting change (the honesty
/// metric moved), and both are things a reviewer must see stated in a
/// PR body rather than absorbed in passing.
#[test]
fn the_margin_thin_accounting_is_goldened_bit_exact() {
    let eps = format!("{:e}", Tol::witness().eps());
    let Some(&(_, golden, path)) = ACCOUNTING_GOLDENS.iter().find(|(row, _, _)| *row == eps) else {
        // **A loud ε-scoped skip, and the shape issue 1342 asked for.**
        // The goldens are keyed by ε row and the matrix draws three; a
        // run at some other tolerance has no golden to compare against,
        // and inventing one from the run would make the row a
        // tautology. Say so on the terminal rather than either failing
        // over a tolerance nobody blessed or passing silently.
        eprintln!(
            "m10_6 accounting golden: no committed golden for eps={eps} (the rows are {}); \
             bless one with M10_6_BLESS_ACCOUNTING=1 if this tolerance should have one",
            ACCOUNTING_GOLDENS
                .iter()
                .map(|(r, _, _)| *r)
                .collect::<Vec<_>>()
                .join(", ")
        );
        return;
    };
    let text = accounting_text();
    if std::env::var("M10_6_BLESS_ACCOUNTING").is_ok() {
        std::fs::write(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path),
            &text,
        )
        .expect("bless writes");
        panic!(
            "accounting golden for eps={eps} re-blessed — commit the file WITH the change \
             it records, then rerun without the env var"
        );
    }
    assert_eq!(
        text, golden,
        "the refusal/tail-mass accounting drifted from its committed golden. This is the \
         honesty metric itself moving: read the diff, decide whether the driver or the \
         accounting changed, and re-bless deliberately (M10_6_BLESS_ACCOUNTING=1)."
    );
}

/// Both fixtures' budgets, serialized, in a fixed order.
fn accounting_text() -> String {
    let mut s = String::new();
    for (label, doc) in [
        ("planted_flip", planted_flip()),
        ("terminal_sliver", terminal_sliver()),
    ] {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let verdict = drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: 4096,
                ..DriveConfig::default()
            },
            Tol::witness(),
        )
        .expect("the nominal builds");
        s.push_str(&format!("== {label}\n"));
        s.push_str(&MassBudget::of(verdict.accounting(), &analyzed).serialize());
    }
    s
}

/// M10-3's planted-flip fixture, re-derived: a square extruded by a
/// parameter whose box straddles zero, so the far side is a definite
/// different branch and refuses as `FlipCrossing` mass.
fn planted_flip() -> ProfileDoc {
    let eps = Tol::witness().eps();
    slab(20.0 * eps, 40.0 * eps)
}

fn slab(nominal: f64, half: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("depth"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        },
    });
    let plane = r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                .expect("finite square corners"),
        ],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: Expr::param(name("depth"), Dimension::Length),
    });
    r.doc
}

/// M10-3's terminal-sliver fixture, re-derived: a rigid transform whose
/// rotation AXIS length is a parameter reaching into the ambiguity
/// band, where refinement provably cannot move the enclosure out.
fn terminal_sliver() -> ProfileDoc {
    let eps = Tol::witness().eps();
    let scalar = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite scalar");
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("axis"),
        value: DocParam::Continuous {
            dim: Dimension::Scalar,
            value: 20.0 * eps,
            display_unit: UnitSym::canonical_for(Dimension::Scalar),
            distribution: Some(Distribution::Uniform {
                lo: -15.0 * eps,
                hi: 15.0 * eps,
            }),
        },
    });
    let plane = r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                .expect("finite square corners"),
        ],
    }));
    let block = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    r.insert(Node::Transform {
        input: block,
        translation: [len(0.0), len(0.0), len(0.0)],
        rotation_axis: [
            scalar(0.0),
            scalar(0.0),
            Expr::param(name("axis"), Dimension::Scalar),
        ],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite angle"),
    });
    r.doc
}

/// **The priced-vs-forced type, at the row that needs it** (M10-6 §2,
/// the M10-1 adjudication's obligation).
///
/// A band-only document's budget must not read as priced. The two
/// fixtures differ in exactly one thing — the placement's law — so the
/// difference in the report is the basis and nothing else.
#[test]
fn a_band_only_documents_budget_reads_forced_and_a_uniform_ones_priced() {
    let priced = analyzed_box(&min_clearance_neck(), &AnalysisPolicy::default());
    let forced = analyzed_box(&band_placement(), &AnalysisPolicy::default());
    assert_eq!(MassBasis::of(&priced), MassBasis::Priced);
    match MassBasis::of(&forced) {
        MassBasis::Forced { by } => assert_eq!(by, vec![name("place")]),
        other => panic!("a band parameter forces the basis: {other:?}"),
    }
    // And the RENDERING says so in words, which is what a consumer
    // actually meets.
    let analyzed = forced;
    let verdict = drive(
        &band_placement(),
        &analyzed,
        &DriveConfig::default(),
        Tol::witness(),
    )
    .expect("the nominal builds");
    let rendered = MassBudget::of(verdict.accounting(), &analyzed).render();
    assert!(
        rendered.contains("FORCED, not priced"),
        "the rendering must not let a forced mass read as a priced one: {rendered}"
    );
}

// ------------------------ the certifying filter's key move, goldened

/// **The witness-vector key of an assertion-carrying document, pinned**
/// (M10-6 deviation D10; both reviews' MAJOR).
///
/// `VerdictVector::certifying` drops `Assertion` rows from the
/// certification comparison, and that MOVES the `verdict_vector_key`
/// every certified leaf carries — for every document with an
/// assertion, including ones with no `min_clearance` anywhere. The
/// change is deliberate and argued at `certifying`; what it was
/// missing is a pin, so the move was invisible in review and a FUTURE
/// move would be too.
///
/// This is that pin: the two keys, bit-exact, side by side. `full` is
/// what the pre-M10-6 comparison produced; `certifying` is what ships.
/// They must differ — that IS the change — and each must be stable.
///
/// # Re-blessing
///
/// A diff here means the certification comparison's INPUTS moved:
/// either the vector's contents (a node kind's verdict rows changed)
/// or the filter (a new node kind excused from certification). Both
/// are decisions a PR body has to state. Re-bless with
/// `M10_6_BLESS_CERT_KEYS=1`, read the diff, and say in the PR which
/// of the two it was.
#[test]
fn the_certifying_filter_moves_the_witness_key_and_the_move_is_goldened() {
    let doc = min_clearance_neck();
    let plain = plain_distance_doc();
    let mut text = String::new();
    for (label, d) in [("min_clearance_neck", &doc), ("plain_distance", &plain)] {
        let ev = evaluate::<f64>(
            d,
            None,
            &CancelToken::new(),
            &EvalOptions {
                profile_lift: ProfileLift::Guided,
                ..EvalOptions::default()
            },
            Tol::witness(),
        );
        let full = VerdictVector::of(&ev);
        let certifying = VerdictVector::certifying(d, &ev);
        assert_ne!(
            full.key().0,
            certifying.key().0,
            "{label}: the filter must MOVE the key — if these agree the document grew no \
             assertion row and the fixture stopped testing the thing"
        );
        use core::fmt::Write as _;
        let _ = writeln!(
            text,
            "{label} full={:032x} certifying={:032x} dropped={}",
            full.key().0,
            certifying.key().0,
            full.rows.len() - certifying.rows.len()
        );
    }
    let path = "tests/golden/m10_6_certifying_keys.txt";
    if std::env::var("M10_6_BLESS_CERT_KEYS").is_ok() {
        std::fs::write(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path),
            &text,
        )
        .expect("bless writes");
        panic!("certifying-key golden re-blessed — commit it WITH the change it records");
    }
    assert_eq!(
        text,
        include_str!("golden/m10_6_certifying_keys.txt"),
        "the certification comparison's key moved. Read the diff: either the verdict rows \
         changed or the filter did, and a PR body has to say which (M10_6_BLESS_CERT_KEYS=1)."
    );
}

/// A document with an assertion over a PLAIN `Distance` measure — no
/// `min_clearance` anywhere. It is here because the review's point was
/// that the key move is not confined to this unit's new primitive: it
/// reaches every assertion-carrying document in the tree.
fn plain_distance_doc() -> ProfileDoc {
    let mut r = Recorder::new();
    let plane = r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)])
                .expect("finite corners"),
        ],
    }));
    let solid = r.insert(Node::Extrude {
        profile,
        distance: len(1.0),
    });
    let measure = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                MeasureRef::at_mint(fixture::fname(solid, fixture::wall(0))),
                MeasureRef::at_mint(fixture::fname(solid, fixture::wall(2))),
            ],
        )
        .expect("indices in range"),
    );
    r.insert(Node::Assertion {
        measure,
        bound: Expr::literal(0.5, Dimension::Length).expect("finite"),
        dir: AssertionDir::AtLeast,
    });
    r.doc
}
