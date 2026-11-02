//! **A clearance is an assertion decided by the sweep** (ERROR-DESIGN
//! E3's clearance amendment, E7; DESIGN `[assertions]`).
//!
//! What this suite claims, in the order the rows make the claims:
//!
//! 1. A lower bound the neck violates is `Violated` with a pair of face
//!    points closer than it, verified at `f64`.
//! 2. An upper bound the neck meets is proved by a pair within it, and
//!    one it exceeds is `Exceeded`: every pair certified clear of it.
//! 3. A band, and `= c`, are ONE sweep: the receipt counts each face
//!    pair once, not once per bound.
//! 4. A bound that is not a distance refuses typed — the verdict's third
//!    state, never a poisoned node.
//! 5. At a point scalar the assertion reads that third state, naming the
//!    certifying lane, and the document still builds.
//! 6. Over a tolerance box, `drive::assertion_at` answers per certified
//!    leaf, each pair stood at the leaf's midpoint.
//!
//! # The fixture, and why it is the dumbbell
//!
//! Two blobs joined by a neck whose two facing walls sit at `y = 0.8`
//! and `y = 1.2`, so the true clearance between them is exactly `0.4` —
//! a number written into the geometry rather than measured out of it.
//! The two walls share no vertex, so the pair is decided by subdivision
//! rather than read off shared topology; they are two faces of ONE body.
//!
//! The box the drive row runs over is ε-scaled for the reason M10-5's
//! header states: above a small fraction of ε no node's interval replay
//! survives (issue 1191's class).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::clearance::{ClearanceRefusal, ClearanceReport, ClearanceVerdict};
use editor_core::drive::{DriveConfig, drive};
use editor_core::{
    AssertionVerdict, CancelToken, ClearanceBounds, Dimension, Distribution, DocEdit, EvalOptions,
    Formula, FreeVar, LoopProgram, Node, NodeResult, ProfileDoc, ProfileProgram, RecipeNodeId,
    SitedRef, UnitSym, ValuePayload, VarName, evaluate,
};
use geom_core::Tol;

use fixture::{Recorder, len};

/// The true clearance between the neck walls, by construction.
const NECK_GAP: f64 = 0.4;

fn name(n: &'static str) -> VarName {
    VarName::from_static(n)
}

/// The analysis half-width, in metres — M10-5's, for M10-5's reason.
fn half() -> f64 {
    Tol::witness().eps() / 64.0
}

/// The dumbbell, and one clearance assertion over its two neck walls
/// per entry of `bounds`, in order.
///
/// Authored through the public doors a user has: the walls by the names
/// the extrude minted, the requirement as one node over them.
fn dumbbell(bounds: &[ClearanceBounds<f64>]) -> Dumbbell {
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
    let wall = |r: &Recorder, seg: u32| {
        SitedRef::new(placed, fixture::fname(solid, fixture::wall(&r.doc, solid, seg)))
    };
    let assertions = bounds
        .iter()
        .map(|b| {
            let (a, w) = (wall(&r, 2), wall(&r, 9));
            let b = b.try_map(|&c| Ok::<_, core::convert::Infallible>(len(c)));
            let Ok(b) = b;
            r.insert(Node::clearance(a, w, b))
        })
        .collect();
    Dumbbell {
        doc: r.doc,
        assertions,
    }
}

/// The document and its assertions' ids, in the order they were asked.
struct Dumbbell {
    doc: ProfileDoc,
    assertions: Vec<RecipeNodeId>,
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

/// The clearance report an evaluation holds for `assertion`.
fn report_of<T: editor_core::EvalScalar>(
    ev: &editor_core::Evaluation<T>,
    assertion: RecipeNodeId,
) -> ClearanceReport {
    let Some(NodeResult::Ok(value)) = ev.result(assertion) else {
        panic!("the assertion evaluated: {:?}", ev.result(assertion));
    };
    let ValuePayload::Assertion(AssertionVerdict::Clearance(report)) = &value.payload else {
        panic!("a clearance assertion's value is the sweep's report");
    };
    (**report).clone()
}

/// The report of one assertion of `bounds` over the dumbbell, at
/// `Interval` over no box.
fn nominal(bounds: ClearanceBounds<f64>) -> ClearanceReport {
    let f = dumbbell(&[bounds]);
    report_of(&eval_over::<geom_core::Interval>(&f.doc, None), f.assertions[0])
}

/// **Row 1**: `clearance ≥ 0.5` over walls 0.4 apart is violated, and
/// the pair it names is two face points the `f64` rebuild puts closer
/// than the bound — at the gap itself, the walls being parallel.
#[test]
fn a_lower_bound_the_neck_breaks_is_violated_by_a_verified_pair() {
    let report = nominal(ClearanceBounds::AtLeast(0.5));
    let ClearanceVerdict::Violated(w) = report.verdict() else {
        panic!("0.5 above a 0.4 gap is violated: {}", report.render());
    };
    let d = w.geometry.distance;
    assert!(d < 0.5, "the pair is closer than the bound: {d}");
    assert!(
        (d - NECK_GAP).abs() < 1e-9,
        "two points of parallel walls 0.4 apart are 0.4 apart: {d}"
    );
    let span = w.geometry.b_point - w.geometry.a_point;
    assert!(
        (span.norm() - d).abs() < 1e-12,
        "the distance is the points' own: {span:?} against {d}"
    );
    assert!(
        w.param.offsets.is_empty(),
        "over no box the pair stands at the nominal: {:?}",
        w.param.offsets
    );
    assert!(report.receipt().holds(), "{:?}", report.receipt());
    // The control: under the gap, the same pair is clear.
    assert_eq!(
        nominal(ClearanceBounds::AtLeast(0.3)).verdict(),
        &ClearanceVerdict::Holds,
        "0.3 under a 0.4 gap holds"
    );
}

/// **Row 2**: `clearance ≤ 0.5` is proved by a pair within it, and
/// `clearance ≤ 0.3` is exceeded — no pair of the walls is within it.
#[test]
fn an_upper_bound_is_proved_by_a_pair_and_exceeded_by_none() {
    let report = nominal(ClearanceBounds::AtMost(0.5));
    let ClearanceVerdict::Within(w) = report.verdict() else {
        panic!("a 0.4 gap is within 0.5: {}", report.render());
    };
    let d = w.geometry.distance;
    assert!(d <= 0.5, "the proving pair is within the bound: {d}");
    assert!(report.receipt().holds(), "{:?}", report.receipt());
    let exceeded = nominal(ClearanceBounds::AtMost(0.3));
    assert_eq!(
        exceeded.verdict(),
        &ClearanceVerdict::Exceeded,
        "a 0.4 gap exceeds 0.3: {}",
        exceeded.render()
    );
    assert_eq!(
        exceeded.receipt().within,
        0,
        "no cell pair lies within an exceeded bound"
    );
}

/// **Row 3**: a band and `= c` are one sweep. Asked as one assertion,
/// the receipt counts each candidate face pair once — two searches
/// would count each twice — and the band's answer is both bounds'.
#[test]
fn a_band_and_an_equality_run_one_fused_sweep() {
    let lower = nominal(ClearanceBounds::AtLeast(0.3));
    let upper = nominal(ClearanceBounds::AtMost(0.5));
    let band = nominal(ClearanceBounds::Within { lo: 0.3, hi: 0.5 });
    let ClearanceVerdict::Within(w) = band.verdict() else {
        panic!("0.3 ≤ 0.4 ≤ 0.5 holds: {}", band.render());
    };
    assert!((0.3..=0.5).contains(&w.geometry.distance), "{}", band.render());
    assert_eq!(
        band.receipt().candidates,
        lower.receipt().candidates,
        "the band examines each face pair once, as one bound does"
    );
    assert_eq!(lower.receipt().candidates, upper.receipt().candidates);
    assert!(
        band.receipt().splits <= lower.receipt().splits.max(upper.receipt().splits),
        "one subdivision serves both bounds: band {:?}, lower {:?}, upper {:?}",
        band.receipt(),
        lower.receipt(),
        upper.receipt()
    );
    // `= c` is the band with `lo = hi = c`: off the gap either way it is
    // violated, by a pair below or by every pair above.
    let below = nominal(ClearanceBounds::Equal(0.45));
    let ClearanceVerdict::Violated(_) = below.verdict() else {
        panic!("= 0.45 over a 0.4 gap has a pair closer: {}", below.render());
    };
    assert_eq!(below.receipt().candidates, lower.receipt().candidates);
    assert_eq!(
        nominal(ClearanceBounds::Equal(0.35)).verdict(),
        &ClearanceVerdict::Exceeded,
        "= 0.35 over a 0.4 gap has no pair within it"
    );
}

/// **Row 4**: a negative bound is not a distance. The assertion answers
/// the third state, typed, and its node is no failure.
#[test]
fn a_bound_that_is_not_a_distance_refuses_as_the_third_state() {
    let report = nominal(ClearanceBounds::AtLeast(-0.1));
    let ClearanceVerdict::Refused(ClearanceRefusal::NotADistance { c }) = report.verdict() else {
        panic!("a negative bound refuses: {}", report.render());
    };
    assert_eq!(*c, -0.1);
    assert_eq!(report.verdict().holds(), None, "the third state is no pass");
    assert_eq!(report.receipt().candidates, 0, "nothing was swept");
}

/// **Row 5**: at `f64` the document builds and the assertion reads the
/// third state, naming the lane that certifies it.
#[test]
fn a_point_scalar_reads_the_third_state_naming_the_certifying_lane() {
    let f = dumbbell(&[ClearanceBounds::AtLeast(0.3)]);
    let ev = eval_over::<f64>(&f.doc, None);
    let failed: Vec<_> = f
        .doc
        .ids()
        .iter()
        .filter_map(|&id| ev.node_error(id))
        .collect();
    assert!(failed.is_empty(), "the document builds at f64: {failed:?}");
    let report = report_of(&ev, f.assertions[0]);
    let ClearanceVerdict::Refused(refusal @ ClearanceRefusal::NeedsCertifyingLane { scalar }) =
        report.verdict()
    else {
        panic!("f64 cannot certify: {}", report.render());
    };
    assert_eq!(*scalar, "f64");
    let said = refusal.to_string();
    assert!(
        said.contains("Interval") && said.contains("Recourse"),
        "the refusal names the certifying lane: {said}"
    );
}

/// **Row 6**: the drive's per-leaf answer. Over the ε-scaled box the
/// drive certifies leaves, `drive::assertion_at` answers the same
/// sweep over each, and each verified pair stands at its leaf's
/// midpoint.
#[test]
fn the_drive_answers_each_certified_leaf_through_the_assertion() {
    let f = dumbbell(&[ClearanceBounds::AtLeast(0.3), ClearanceBounds::AtLeast(0.5)]);
    let analyzed = analyzed_box(&f.doc, &AnalysisPolicy::default());
    let verdict = drive(&f.doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect("the witness build succeeds");
    assert!(
        !verdict.certified().is_empty(),
        "the ε-scaled box certifies at least one leaf; refused: {}",
        verdict.refused().len()
    );
    let place = f.doc.var_named("place").expect("the fixture declares it");
    for leaf in verdict.certified() {
        let at = |assertion| {
            match editor_core::drive::assertion_at(
                &f.doc,
                assertion,
                &leaf.box_,
                verdict.symbolic(),
                Tol::witness(),
            ) {
                Some(AssertionVerdict::Clearance(report)) => *report,
                other => panic!("a clearance's verdict over a leaf: {other:?}"),
            }
        };
        let held = at(f.assertions[0]);
        assert_eq!(held.verdict(), &ClearanceVerdict::Holds, "{}", held.render());
        let broken = at(f.assertions[1]);
        let ClearanceVerdict::Violated(w) = broken.verdict() else {
            panic!("0.5 is violated over every leaf: {}", broken.render());
        };
        let mid = leaf.box_.get(place).expect("the leaf varies `place`").midpoint();
        assert_eq!(
            w.param.offsets,
            vec![(place, mid)],
            "the pair stands at the leaf's midpoint"
        );
    }
}
