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
//! draws — precisely `ci-filter.py`'s `EPS_ROWS`, so every point the
//! gate can draw has one. A run at any other tolerance FAILS, naming
//! the blessed rows and the bless command: the arm is unreachable on
//! any drawn point, and what it catches is the matrix growing a row
//! while this table does not. (It used to print a note and return
//! green, which is a row that reports nothing and passes — the shape
//! this tree refuses everywhere else.)
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, SymbolicDials, VerdictVector, drive};
use editor_core::report::{MassBasis, MassBudget};
use editor_core::{
    AssertionDir, AssertionVerdict, CancelToken, Dimension, Distribution, DocEdit, DocParam,
    EvalOptions, Expr, LoopProgram, MeasureExpr, MeasurePrimitive, Node, NodeResult, ParamName,
    ProfileDoc, ProfileLift, ProfileProgram, RecipeNodeId, SitedRef, UnitSym, ValuePayload,
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
        document: "distributed_plate",
        // **The entry MINOR-2 asked for**: a document that actually
        // DRIVES. Every other row here either takes the `NothingVaries`
        // path (`measured_web`, which declares no distribution at all)
        // or certifies its whole box in a single leaf, so all three
        // recorded budgets were 0.0 and none of them could
        // discriminate — a budget of zero passes at `max_leaves: 1`.
        // This is the two-hole plate at the tour's own ε-scaled
        // tolerances: three varying axes and a real subdivision.
        //
        // The number is the analyzed box's own EXCLUDED TAIL. The two
        // radii are normals, so the ±3σ box leaves mass outside on
        // each axis, and the product measure over the three leaves
        // 0.5393% unresolved even when every leaf certifies. Recorded
        // rather than derived at run time — deriving it would make the
        // row a tautology — and tight enough to bite: it is the tail
        // alone, so a drive that refused any leaf at all exceeds it.
        // MEASURED at 0.005392710000000176; recorded one significant
        // figure above so a last-ulp move in the quantile arithmetic
        // is not a red, and a refused leaf still is.
        unresolved: 0.0054,
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
    out.push(Entry {
        name: "distributed_plate",
        doc: distributed_plate(),
    });
    out
}

/// The two-hole plate at the tour's stop-2 tolerances — the one
/// registered document whose drive actually subdivides (MINOR-2).
///
/// Authored here rather than imported from `demos/tour`, which is a
/// detached workspace the kernel must not depend on: the SHAPE is the
/// worked example's and the numbers are stop 2's.
fn distributed_plate() -> ProfileDoc {
    const SPACING: f64 = 3.1e-3;
    const RADIUS: f64 = 1.25e-3;
    let spread = half();
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("half_spacing"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: SPACING / 2.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -0.05 * spread,
                hi: 0.05 * spread,
            }),
        },
    });
    for n in ["hole_a_r", "hole_b_r"] {
        r.push(DocEdit::SetDocParam {
            name: name(n),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: RADIUS,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(Distribution::Normal {
                    sigma: 0.2 * spread,
                }),
            },
        });
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
            .expect("finite plate corners"),
        ],
    }));
    let _plate = r.insert(Node::Extrude {
        profile: plate_p,
        distance: len(1.0e-3),
    });
    let hs = Expr::param(name("half_spacing"), Dimension::Length);
    let hole_a_p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Circle {
            centre: [Expr::neg(hs.clone()), len(0.0)],
            radius: Expr::param(name("hole_a_r"), Dimension::Length),
        }],
    }));
    let hole_a = r.insert(Node::Extrude {
        profile: hole_a_p,
        distance: len(1.0e-3),
    });
    let hole_b_p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Circle {
            centre: [hs, len(0.0)],
            radius: Expr::param(name("hole_b_r"), Dimension::Length),
        }],
    }));
    let hole_b = r.insert(Node::Extrude {
        profile: hole_b_p,
        distance: len(1.0e-3),
    });
    let ev = evaluate::<f64>(
        &r.doc,
        None,
        &CancelToken::new(),
        &EvalOptions {
            profile_lift: ProfileLift::Guided,
            ..EvalOptions::default()
        },
        Tol::witness(),
    );
    let wall = |node: RecipeNodeId| {
        let mut faces = editor_core::select_where(
            &ev,
            node,
            &editor_core::Selector::of(editor_core::NamePat::of_kind(
                editor_core::EntityKind::Face,
            )),
            &[editor_core::GeomPred::SurfaceKind(
                editor_core::SurfaceKindSet::just(geom_brep::SurfaceKind::Cylinder),
            )],
            &r.doc.param_env::<f64>(),
            Tol::witness(),
        )
        .expect("the hole wall is an exact atom");
        faces.sort();
        SitedRef::new(node, faces.remove(0))
    };
    let refs = vec![wall(hole_a), wall(hole_b)];
    let radius_of = |n: &str| MeasureExpr::value(Expr::param(name(n), Dimension::Length));
    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::add(radius_of("hole_a_r"), radius_of("hole_b_r")).expect("L + L"),
    )
    .expect("L - L");
    let measure = r.insert(Node::measure(web, refs).expect("indices in range"));
    // A bound the run can DECIDE: a decade past the escalation
    // threshold below the nominal web, so the verdict is a plain
    // `Holds` rather than a band-coincident one. Row 1 is about the
    // verdict being taken and holding, not about the band.
    r.insert(Node::Assertion {
        measure,
        bound: Expr::literal(
            SPACING - 2.0 * RADIUS - 100.0 * Tol::witness().eps(),
            Dimension::Length,
        )
        .expect("finite"),
        dir: AssertionDir::AtLeast,
    });
    r.doc
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
                SitedRef::new(placed, fixture::fname(solid, fixture::wall(2))),
                SitedRef::new(placed, fixture::fname(solid, fixture::wall(9))),
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

/// The lane a registered document is driven on.
///
/// The symbolic identity tier (ERROR-DESIGN E12) is the shipped default
/// and every document here takes it — except the ones whose measure is
/// a `min_clearance`, whose engine has no lane at the tier
/// (`DriveRefusal::SymbolicClearanceUnsupported`; the deviation is issue
/// `symbolic-tier-and-clearance-engine`). Those fall back to the numeric-only replay, which keeps their
/// recorded budgets real rows rather than skipped ones.
///
/// The fallback is driven by the DRIVER'S OWN refusal rather than by a
/// list of document names here: a name list would go stale the first
/// time a fixture grew a clearance measure, and the refusal is exactly
/// the fact being reacted to.
fn drive_registered(
    doc: &ProfileDoc,
    analyzed: &editor_core::analysis::AnalyzedBox,
    tol: Tol,
) -> Result<editor_core::drive::ParamBoxVerdict, editor_core::DriveRefusal> {
    match drive(doc, analyzed, &DriveConfig::default(), tol) {
        Err(editor_core::DriveRefusal::SymbolicClearanceUnsupported { .. }) => drive(
            doc,
            analyzed,
            &DriveConfig {
                symbolic: SymbolicDials::off(),
                ..DriveConfig::default()
            },
            tol,
        ),
        other => other,
    }
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
        let verdict = match drive_registered(&entry.doc, &analyzed, Tol::witness()) {
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
        // **An unblessed ε FAILS** (M10-6's fix pass; R2's MINOR-11).
        //
        // The first pass printed a note and returned — green, in 0.00
        // s, on a tolerance nothing had ever been compared at. That is
        // the shape this repository refuses everywhere else: a row that
        // reports nothing and passes is indistinguishable from a row
        // that checked something, and the sampled matrix is exactly
        // where nobody is watching which point was drawn.
        //
        // It costs nothing to fail here, because
        // [`ACCOUNTING_GOLDENS`] holds precisely `ci-filter.py`'s
        // `EPS_ROWS`: every point the matrix can draw HAS a golden, so
        // this arm is unreachable on any run the gate takes. What it
        // catches is the matrix growing an ε row while the goldens do
        // not — the drift the silent return would have hidden for as
        // long as nobody read a log.
        panic!(
            "no committed accounting golden for eps={eps}. The blessed rows are {}, which \
             are ci-filter.py's EPS_ROWS — so either the matrix grew a row and this table \
             did not, or this run is at a tolerance the gate does not draw. Bless it with \
             M10_6_BLESS_ACCOUNTING=1 and commit the file WITH the change it records.",
            ACCOUNTING_GOLDENS
                .iter()
                .map(|(r, _, _)| *r)
                .collect::<Vec<_>>()
                .join(", ")
        );
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

/// **M10-3's own fixtures, not copies of them** (M10-6's fix pass;
/// R2's MINOR-14).
///
/// This row goldens the ACCOUNTING of M10-3's planted-flip and
/// terminal-sliver documents. It used to re-derive both here, which
/// meant the golden could go on passing while the documents it claims
/// to be about drifted away from it — the two would simply have become
/// different fixtures wearing the same names. They are imported now,
/// so there is one home and a change to either reds this golden, which
/// is exactly what a golden about someone else's fixture is for.
fn planted_flip() -> ProfileDoc {
    let eps = Tol::witness().eps();
    crate::m10_3_driver_interval::slab(20.0 * eps, 40.0 * eps)
}

fn terminal_sliver() -> ProfileDoc {
    crate::m10_3_driver_interval::sliver_axis()
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
    // The band fixture is a `min_clearance` document (its measure is
    // the neck's), so it takes the numeric lane — see
    // [`drive_registered`].
    let verdict =
        drive_registered(&band_placement(), &analyzed, Tol::witness()).expect("the nominal builds");
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
                SitedRef::at_mint(fixture::fname(solid, fixture::wall(0))),
                SitedRef::at_mint(fixture::fname(solid, fixture::wall(2))),
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
