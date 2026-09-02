//! **The E6 subdivision driver** — the leaf protocol, the receipt, the
//! accounting, and the fixtures the unit exists to answer.
//!
//! Everything here goes through the public doors a consumer has:
//! `analyzed_box` for the box, `drive` for the verdict, the verdict's
//! own accessors for the answer. Nothing reaches past them.
//!
//! # The widths are in ε, and that is the unit's headline finding
//!
//! Every box below is sized as a multiple of the run's ε. That is not
//! tidiness: the certification predicates the kernel runs on a replayed
//! body are *identities* — an edge's endpoint lies ON its carrier, a
//! side plane is cosurface with its neighbour — whose margin is exactly
//! zero in real arithmetic and whose INTERVAL enclosure is `[0, c·w]`
//! for a box of width `w`, because the two sides of the identity share
//! the parameter and interval arithmetic cannot see that they cancel.
//! An enclosure that straddles the coincidence threshold is
//! indeterminate, so a leaf becomes fully definite only once its own
//! width is a fraction of ε. The fraction is FIXTURE-DEPENDENT — `c`
//! counts the parameter-dependent terms one identity accumulates — and
//! `the_certification_width_is_a_small_fraction_of_epsilon` measures
//! it rather than asserting it: `ε/8` of width on the one-extrude slab
//! here, about half that on the two-extrude chamber the review suites
//! drive.
//!
//! Refinement gets there, and the driver is correct at every step; what
//! it costs is that a MACROSCOPIC tolerance box (a ±0.05 band on a 1.0
//! nominal is `10^8 ε` wide) needs about 30 bisections on that axis to
//! reach its first certified leaf — `log2(0.05 / (ε/16))` at the
//! default ε — against a shipped per-axis depth budget of 24. On
//! such a box today's verdict is `Budget`-refused mass, priced and
//! reported — never a silent partial and never a false certificate,
//! but not yet the "2.1% of the tolerance mass has no valid build"
//! sentence either.
//!
//! This is the same widening class as **issue #1191**, observed on a
//! different family of predicates; that issue is the class's home and
//! this unit consumes it rather than fixing it. The by-shape sweep for
//! its siblings, its commands and its blind spots are recorded in this
//! unit's PR — 57 identity- and gap-shaped funnel names, of which the
//! fixtures here observe three (`carrier_endpoint_start`,
//! `carrier_endpoint_end`, `side_planes_cosurface`).
//!
//! The file's basename carries `interval` deliberately: the driver is
//! gated on that feature (there is no leaf to certify without the
//! certified scalar) and `scripts/ci-filter.py` pins the hosted lane on
//! exactly that name, so this unit's own axis is never left to the
//! sampling draw.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::UnitSym;
use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box, param_env_over};
use editor_core::drive::{
    BudgetKind, DEFAULT_MAX_DEPTH, DriveConfig, DriveRefusal, FlipEvidence, ReasonClass,
    RefusalReason, VerdictVector, drive,
};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, DocParam, EvalOptions, Expr, LoopProgram, Node,
    NodeErrorKind, NodeResult, ParamName, ParamValue, ProfileDoc, ProfileLift, ProfileProgram,
    evaluate,
};
use geom_core::{Bounds, Interval, Tol};
use profile::SketchPlane;

use fixture::{Recorder, len};

fn eps() -> f64 {
    Tol::witness().eps()
}

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

fn lit(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length literal")
}

fn param(n: &str) -> Expr {
    Expr::param(name(n), Dimension::Length)
}

fn uniform(w: f64) -> Distribution {
    Distribution::Uniform { lo: -w, hi: w }
}

fn config(max_leaves: usize) -> DriveConfig {
    DriveConfig {
        max_leaves,
        ..DriveConfig::default()
    }
}

fn unit_square() -> LoopProgram {
    LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
        .expect("finite square corners")
}

/// **The planted-flip fixture**: a square extruded by a distance that
/// is a document parameter, over a box that straddles ZERO.
///
/// The flip is real and it is not a degeneracy of the analysis: on the
/// far side the extrusion runs the other way, so
/// `extrusion_normal_component` and the side planes' cosurface tests
/// all decide the opposite sign and the body is a perfectly good solid
/// built on a different branch. That is exactly the no-flips v1 case —
/// definite, on a different verdict vector, refused as mass rather than
/// analyzed.
fn slab(nominal: f64, half: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("depth"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(uniform(half)),
        },
    });
    let xy_frame_0 = r.insert(Node::Datum(editor_core::Datum::Frame {
        origin: [0.0, 0.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Length).unwrap()),
        u: [1.0, 0.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Scalar).unwrap()),
        v: [0.0, 1.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Scalar).unwrap()),
    }));
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: xy_frame_0,
        loops: vec![unit_square()],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: param("depth"),
    });
    r.doc
}

/// **The two-parameter fixture**: M10-P's own parametric shape — a
/// plate with a hole whose RADIUS is a document parameter, extruded by
/// a distance that is a SECOND document parameter.
///
/// Both kinds of parameter slot are live at once: the radius feeds
/// profile geometry, reachable only through the lift's guided second
/// pass, and the depth feeds a magnitude slot. Before this unit the
/// first of those reached the interval lane as a constant.
fn two_param_plate(radius: Distribution, depth: Distribution) -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("hole_r"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 0.25,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(radius),
        },
    });
    r.push(DocEdit::SetDocParam {
        name: name("depth"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 0.5,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(depth),
        },
    });
    let xy_frame_1 = r.insert(Node::Datum(editor_core::Datum::Frame {
        origin: [0.0, 0.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Length).unwrap()),
        u: [1.0, 0.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Scalar).unwrap()),
        v: [0.0, 1.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Scalar).unwrap()),
    }));
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: xy_frame_1,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)])
                .expect("finite plate corners"),
            LoopProgram::Circle {
                centre: [lit(1.0), lit(1.0)],
                radius: param("hole_r"),
            },
        ],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: param("depth"),
    });
    r.doc
}

/// **The terminal-sliver fixture**: a rigid transform whose rotation
/// AXIS length is a document parameter, over a box reaching into the
/// ambiguity band.
///
/// The axis's norm is a margin like any other. At the nominal (`20ε`)
/// it is definitely positive, so the witness builds; a leaf whose whole
/// enclosure lands strictly inside `(ε, Kε)` is deciding a quantity
/// that IS in the band, and no amount of narrowing moves it out. That
/// is PR-7's genuine semantic sliver, and the ratified answer is to
/// refuse it rather than refine it.
fn sliver_axis() -> ProfileDoc {
    let scalar = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite scalar");
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("axis"),
        value: DocParam::Continuous {
            dim: Dimension::Scalar,
            value: 20.0 * eps(),
            display_unit: UnitSym::canonical_for(Dimension::Scalar),
            distribution: Some(uniform(15.0 * eps())),
        },
    });
    let xy_frame_2 = r.insert(Node::Datum(editor_core::Datum::Frame {
        origin: [0.0, 0.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Length).unwrap()),
        u: [1.0, 0.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Scalar).unwrap()),
        v: [0.0, 1.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Scalar).unwrap()),
    }));
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: xy_frame_2,
        loops: vec![unit_square()],
    }));
    let block = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    r.insert(Node::Transform {
        input: block,
        translation: [lit(0.0), lit(0.0), lit(0.0)],
        rotation_axis: [
            scalar(0.0),
            scalar(0.0),
            Expr::param(name("axis"), Dimension::Scalar),
        ],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite angle"),
    });
    r.doc
}

// ---------------------------------------------------------------- door

/// The interval parameter door: a widened axis reaches the lane as a
/// NON-DEGENERATE interval, and a fixed one reaches it as the nominal
/// exactly.
///
/// This is the fact M10-P's seam was blocked on ("nothing reaches
/// `evaluate::<Interval>` with a non-degenerate parameter today"), so
/// it is pinned directly rather than inferred from a driver result.
#[test]
fn the_parameter_door_widens_exactly_the_declared_axes() {
    let doc = two_param_plate(uniform(0.01), uniform(0.0));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = ParamBox::of(&analyzed);
    let env = param_env_over::<Interval, _>(&doc, &box_).expect("the box binds at Interval");
    let ParamValue::Continuous { value: radius, .. } = env.bindings[&name("hole_r")] else {
        panic!("hole_r is continuous")
    };
    // The nominal is added in the scalar's own arithmetic, so the
    // enclosure rounds OUTWARD: it contains the true span and is at
    // most an ulp wider on each side, never narrower.
    assert!(radius.lo() <= 0.24 && 0.26 <= radius.hi());
    assert!(radius.hi() - radius.lo() <= 0.02 + 1e-12);
    let ParamValue::Continuous { value: depth, .. } = env.bindings[&name("depth")] else {
        panic!("depth is continuous")
    };
    // A zero-width uniform is a FIXED axis and reaches the lane as the
    // nominal, not as a hair-wide interval.
    assert_eq!((depth.lo(), depth.hi()), (0.5, 0.5));
}

/// A point scalar cannot carry a widened axis, and says so on every
/// node rather than quietly answering the nominal build's question.
#[test]
fn a_widened_box_at_f64_refuses_loudly() {
    let doc = two_param_plate(uniform(0.01), uniform(0.01));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = Arc::new(ParamBox::of(&analyzed));
    assert!(param_env_over::<f64, _>(&doc, &box_).is_err());
    let opts = EvalOptions {
        param_box: Some(box_),
        ..EvalOptions::default()
    };
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &opts, Tol::witness());
    assert!(!ev.order.is_empty());
    for id in &ev.order {
        assert!(
            matches!(
                ev.result(*id),
                Some(NodeResult::Failed(e)) if matches!(e.kind, NodeErrorKind::ParamBox { .. })
            ),
            "node {} did not refuse the box",
            id.0
        );
    }
}

/// The split rule as shipped: the axis of greatest RELATIVE width, ties
/// to the lowest axis index, bisected at the midpoint.
#[test]
fn the_split_rule_is_relative_width_with_a_lowest_index_tie() {
    let mut axes = BTreeMap::new();
    // Both axes span the same numbers in the root. In the sub-box `b`
    // has been narrowed, so `a` is the relatively wider one — and it is
    // ALSO the numerically wider one there, which is the case a
    // relative rule and an absolute rule agree on. The row below is the
    // one that separates them.
    axes.insert(name("a"), BoxAxis::Varying { lo: -1.0, hi: 1.0 });
    axes.insert(name("b"), BoxAxis::Varying { lo: -1.0, hi: 1.0 });
    let root = ParamBox::from_axes(axes.clone());
    axes.insert(name("b"), BoxAxis::Varying { lo: 0.0, hi: 0.1 });
    assert_eq!(ParamBox::from_axes(axes).split_axis(&root), Some(name("a")));

    // Relative vs absolute, separated: `wide`'s root axis is a hundred
    // times `narrow`'s, and the sub-box has already been bisected on
    // `wide` seven times. `wide` is still numerically wider; `narrow`
    // is relatively wider, and relative is what the rule reads.
    let mut root2 = BTreeMap::new();
    root2.insert(
        name("narrow"),
        BoxAxis::Varying {
            lo: -0.01,
            hi: 0.01,
        },
    );
    root2.insert(name("wide"), BoxAxis::Varying { lo: -1.0, hi: 1.0 });
    let root2b = ParamBox::from_axes(root2.clone());
    root2.insert(
        name("wide"),
        BoxAxis::Varying {
            lo: 0.0,
            hi: 2.0 / 128.0,
        },
    );
    assert_eq!(
        ParamBox::from_axes(root2).split_axis(&root2b),
        Some(name("narrow"))
    );

    // The tie — both axes at full relative width — goes to the lowest
    // axis index, which is name order.
    assert_eq!(root.split_axis(&root), Some(name("a")));
    let (lo, hi) = root.split(&name("a")).expect("a splits");
    assert_eq!(lo.get(&name("a")).unwrap().span(), (-1.0, 0.0));
    assert_eq!(hi.get(&name("a")).unwrap().span(), (0.0, 1.0));
    // The other axis is untouched: one axis per bisection.
    assert_eq!(lo.get(&name("b")).unwrap().span(), (-1.0, 1.0));
}

// -------------------------------------------------------------- e2e

/// **The certification width, measured rather than asserted.** The
/// largest box that certifies WHOLE — in one leaf, with no bisection —
/// is a small fraction of ε, and this row finds it by bisection
/// instead of hard-coding a power of two.
///
/// It is the load-bearing constant behind the limit row below, so it
/// is measured here and quoted there rather than stated twice. The
/// bracket is deliberately wide (`ε/4096` to `ε`): what the row pins is
/// the ORDER — a leaf goes fully definite only once its own width is a
/// fraction of the coincidence threshold, because the certification
/// predicates are checked identities whose interval enclosure widens
/// with the box. If the widening ever closes, this row fails on its
/// upper bound and the limit row below fails with it, which is the
/// point of both.
///
/// The exact fraction is FIXTURE-DEPENDENT and no single number should
/// be quoted as the kernel's: this one-extrude slab certifies at a
/// half-width of `ε/16`, while the two-extrude chamber the review
/// suites drive needs about twice the refinement. The dependence is
/// itself the finding — the constant is `c` in `[0, c·w]`, and `c`
/// counts how many parameter-dependent terms the identity accumulates.
#[test]
fn the_certification_width_is_a_small_fraction_of_epsilon() {
    let e = eps();
    let certifies_whole = |half: f64| {
        let doc = slab(1.0, half);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let v = drive(&doc, &analyzed, &config(64), Tol::witness()).unwrap();
        v.receipt().splits == 0 && v.receipt().certified == 1
    };
    // A bracket, then bisect it: `lo` certifies whole, `hi` does not.
    let (mut lo, mut hi) = (e / 4096.0, e);
    assert!(certifies_whole(lo), "the bracket's floor must certify");
    assert!(!certifies_whole(hi), "the bracket's ceiling must not");
    for _ in 0..12 {
        let mid = 0.5 * (lo + hi);
        if certifies_whole(mid) {
            lo = mid
        } else {
            hi = mid
        }
    }
    assert!(
        lo >= e / 4096.0 && hi <= e,
        "the certification half-width settled at {lo}..{hi}, outside the bracket"
    );
}

/// **The limit, pinned rather than described.** A MACROSCOPIC tolerance
/// box — the ±0.05 band on a 1.0 nominal a real study would ask for —
/// certifies nothing, and the whole of it comes back as priced `Budget`
/// mass.
///
/// **Run at the shipped per-axis depth budget**, which is the binding
/// one: reaching a certifying leaf from a half-width of 0.05 means
/// bisecting down to the width the row above measures, and
/// `log2(0.05 / (ε/16))` is about 29.6 at the default ε — call it 30
/// bisections, against a shipped [`DEFAULT_MAX_DEPTH`] of 24. The leaf
/// budget is set small here only so the row costs a second rather than
/// tens of thousands of evaluations; the depth budget is what makes
/// the answer inevitable, and it is the shipped one.
///
/// This is the honest state of the deliverable and it is a regression
/// pin in both directions: the day the certification predicates stop
/// widening with the box, this row fails and the number it is
/// asserting becomes a real answer.
///
/// **What the widening here IS, said precisely, because the obvious
/// candidate has been ruled out.** It is not a period fold enclosing
/// two integers: the floor-based folds that did that were closed
/// (issue 1191, and the fold now folds a raw difference once through a
/// window whose jump is at a half period), and this row did not move
/// when they were. What remains is the dependency problem — a
/// certification identity mentions its parameter several times, and an
/// interval evaluation cannot see that the occurrences are the same
/// number, so the enclosure grows with the box whatever the algebra
/// says. Closing THAT is a different deliverable from closing a fold,
/// and this row is the pin that says so.
#[test]
fn a_macroscopic_box_refuses_all_of_its_mass_as_budget_today() {
    let doc = slab(1.0, 0.05);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let shipped_depth = DriveConfig::default().max_depth;
    let v = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_depth: shipped_depth,
            max_leaves: 32,
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .unwrap();
    assert_eq!(shipped_depth, DEFAULT_MAX_DEPTH);
    assert!(v.receipt().holds());
    assert!(
        v.certified().is_empty(),
        "a macroscopic box certified {} leaves — the widening limit moved, and this row's \
         number is now a real answer",
        v.certified().len()
    );
    assert!(
        v.refused()
            .iter()
            .all(|l| matches!(l.reason, RefusalReason::Budget(_)))
    );
    // Priced, not silent: the refusal covers the whole box.
    let budget = v.accounting().refused[&ReasonClass::Budget]
        .clone()
        .unwrap();
    assert!((budget - 1.0).abs() <= 1e-9, "budget mass {budget}");
    assert!((v.accounting().total().unwrap() - 1.0).abs() <= 1e-9);
}

/// **The worked example's driver half**, on the two-parameter document:
/// leaves certify after real bisection, the receipt identity holds, and
/// the accounting sums to 1.
#[test]
fn the_two_parameter_drive_certifies_after_bisection_and_accounts_for_all_of_it() {
    let doc = two_param_plate(uniform(eps() / 4.0), uniform(eps() / 4.0));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(256), Tol::witness()).expect("the nominal builds");

    assert!(v.receipt().holds(), "receipt: {:?}", v.receipt());
    assert!(
        v.receipt().splits >= 1,
        "a box wider than the certification width should not certify whole: {:?}",
        v.receipt()
    );
    assert!(
        !v.certified().is_empty(),
        "the box should certify SOMETHING: {:?}",
        v.receipt()
    );
    // Every certified leaf carries the witness's own vector key — that
    // equality IS the certificate — and its per-node results.
    for leaf in v.certified() {
        assert_eq!(leaf.verdict_vector_key, v.witness_vector().key());
        assert!(!leaf.results.node_keys.is_empty());
    }
    let total = v.accounting().total().expect("no band in this fixture");
    assert!(
        (total - 1.0).abs() <= 1e-9,
        "accounting sums to {total}, not 1"
    );
    // The tail of a bounded distribution is exactly zero — reported,
    // not omitted.
    assert_eq!(v.accounting().unanalyzed, Ok(0.0));
}

/// The verdict is a function of its inputs: two drives of the SAME
/// document agree bit for bit, and the parallel schedule agrees with
/// the sequential one (D9 idiom 1).
#[test]
fn the_verdict_is_bit_identical_across_repeats_and_schedules() {
    let doc = slab(20.0 * eps(), 40.0 * eps());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let seq = drive(&doc, &analyzed, &config(256), Tol::witness()).unwrap();
    let again = drive(&doc, &analyzed, &config(256), Tol::witness()).unwrap();
    let par = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            parallel: true,
            ..config(256)
        },
        Tol::witness(),
    )
    .unwrap();
    // Not a vacuous comparison: hundreds of splits and a full frontier
    // of leaves, compared row for row. It is NOT a drive with certified
    // leaves — at 256 leaves this fixture certifies none, because the
    // sign flip keeps every box indeterminate past the frontier bound
    // (the planted-flip row above pays 4096 leaves for exactly that
    // reason). What this row pins is that the same subdivision comes
    // out identical twice and under both schedules, and refusals are
    // rows like any other.
    assert!(seq.receipt().splits > 8, "{:?}", seq.receipt());
    assert!(
        seq.receipt().certified + seq.receipt().refused > 8,
        "{:?}",
        seq.receipt()
    );
    assert_eq!(seq.serialize(), again.serialize());
    assert_eq!(seq.serialize(), par.serialize());
    assert_eq!(seq.content_key(), par.content_key());
}

/// Read-only (E8): driving does not change the document, and the same
/// document value drives twice identically. The API cannot express a
/// write — `drive` takes `&Doc` — so what is left to check is that the
/// value it was handed is untouched.
#[test]
fn driving_writes_nothing() {
    let doc = slab(20.0 * eps(), 40.0 * eps());
    let before = doc.clone();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let _ = drive(&doc, &analyzed, &config(64), Tol::witness()).unwrap();
    assert!(doc.bit_eq(&before));
}

// ------------------------------------------------------- planted flip

/// **The planted flip.** A box straddling a real branch change
/// certifies on the witness side and refuses `FlipCrossing` on the far
/// side, with the flipped predicates NAMED from the vector diff.
#[test]
fn a_planted_flip_refuses_flip_crossing_and_names_what_flipped() {
    let doc = slab(20.0 * eps(), 40.0 * eps());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(4096), Tol::witness()).expect("the nominal builds");
    assert!(v.receipt().holds());
    assert!(!v.certified().is_empty(), "the witness side must certify");

    let flips: Vec<&FlipEvidence> = v
        .refused()
        .iter()
        .filter_map(|l| match &l.reason {
            RefusalReason::FlipCrossing { flipped } => Some(&**flipped),
            _ => None,
        })
        .collect();
    assert!(
        !flips.is_empty(),
        "a box across the branch change must refuse FlipCrossing; classes were {:?}",
        v.refused()
            .iter()
            .map(|l| l.reason.class())
            .collect::<Vec<_>>()
    );
    // NAMED, not merely counted — and named by `resolve::vdiff`, the
    // engine this tree declares built once, whose `VerdictFlip` carries
    // the predicate and both net signs. This is the consumer sentence
    // the unit exists to produce.
    let named: Vec<_> = flips
        .iter()
        .flat_map(|e| e.verdicts.nodes.values())
        .flat_map(|d| &d.flips)
        .map(|f| (f.predicate, f.from, f.to))
        .collect();
    assert!(
        named
            .iter()
            .any(|(p, w, l)| *p == "extrusion_normal_component"
                && *w == geom_core::Sign::Positive
                && *l == geom_core::Sign::Negative),
        "the flip was not named: {named:?}"
    );
    // Not vacuous on this fixture: every FlipCrossing here names
    // something, so the row is about evidence and not about an empty
    // set that happens to satisfy a universal.
    assert!(flips.iter().all(|e| !e.is_empty()));
    // And it is priced: the far side is refused MASS, not a silence.
    let mass = v.accounting().refused[&ReasonClass::FlipCrossing]
        .clone()
        .unwrap();
    assert!(mass > 0.0, "flip mass priced at {mass}");
}

/// A box wholly inside the witness chamber certifies whole — the
/// near-flip control for the row above, and the reason the row above is
/// about a flip rather than about a wide box.
#[test]
fn a_box_inside_the_witness_chamber_certifies_whole() {
    let doc = slab(1.0, eps() / 16.0);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(64), Tol::witness()).unwrap();
    assert!(v.receipt().holds());
    assert!(
        v.refused().is_empty(),
        "a box well inside one chamber should not refuse: {:?}",
        v.refused().iter().map(|l| &l.reason).collect::<Vec<_>>()
    );
    let certified = v.accounting().certified.clone().unwrap();
    assert!(
        (certified - 1.0).abs() <= 1e-9,
        "certified mass {certified}"
    );
}

/// Certification is EXACT verdict comparison, not a width heuristic.
///
/// The mutation: perturb the witness vector — one sign flipped, or the
/// rows reordered — and the identity a certified leaf is checked
/// against must change. A width-based certifier would not notice
/// either edit.
#[test]
fn certification_reads_the_vector_and_nothing_else() {
    let doc = slab(1.0, eps() / 16.0);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(64), Tol::witness()).unwrap();
    let witness = v.witness_vector().clone();
    assert!(
        witness.rows.iter().any(|r| !r.verdicts.is_empty()),
        "the witness build decided something"
    );

    let mut mutated = witness.clone();
    let row = mutated
        .rows
        .iter_mut()
        .find(|r| !r.verdicts.is_empty())
        .unwrap();
    row.verdicts[0].sign = row.verdicts[0].sign.flip();
    assert_ne!(mutated, witness);
    assert_ne!(mutated.key(), witness.key());

    // The key is a function of the WHOLE vector, order included: two
    // builds that made the same decisions in a different order are not
    // the same build.
    let mut reordered = witness.clone();
    reordered.rows.reverse();
    assert_ne!(reordered.key(), witness.key());

    // And the CERTIFIER's use of it, not merely the key's sensitivity:
    // every certified leaf carries the witness's key, and the mutated
    // vector's key is one no certified leaf carries — so a drive whose
    // witness had made that one different decision could not have
    // certified these leaves.
    for leaf in v.certified() {
        assert_eq!(leaf.verdict_vector_key, witness.key());
        assert_ne!(leaf.verdict_vector_key, mutated.key());
    }
    // The strict comparison is what gates, and it is strictly stronger
    // than the population engine that NAMES flips: a reordering the
    // populations cannot see moves this key.
    assert_eq!(v.witness_vector().key(), witness.key());
}

// ------------------------------------------------------------ slivers

/// **Terminal slivers refuse, and are never refined** (the ratified
/// PR-7 semantics): a leaf whose deciding enclosure sits wholly inside
/// `(ε, Kε)` is refused naming the predicate, and the driver does not
/// spend a single further bisection on it.
#[test]
fn a_terminal_sliver_refuses_naming_its_predicate_and_is_not_refined() {
    let doc = sliver_axis();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(4096), Tol::witness()).expect("the nominal builds");
    assert!(v.receipt().holds());
    let slivers: Vec<_> = v
        .refused()
        .iter()
        .filter_map(|l| match &l.reason {
            RefusalReason::SliverTerminal { predicate } => Some(*predicate),
            _ => None,
        })
        .collect();
    assert_eq!(
        slivers.first(),
        Some(&"eval_direction_norm"),
        "classes were {:?}",
        v.refused()
            .iter()
            .map(|l| l.reason.class())
            .collect::<Vec<_>>()
    );
    // Never refined: the sliver band is a sixth of this box, and a
    // driver that kept bisecting it would need thousands of leaves to
    // reach the depth budget. A few dozen is the signature of stopping.
    assert!(
        v.receipt().certified + v.receipt().refused < 128,
        "the sliver region was refined rather than refused: {:?}",
        v.receipt()
    );
    let mass = v.accounting().refused[&ReasonClass::SliverTerminal]
        .clone()
        .unwrap();
    assert!(mass > 0.0, "sliver mass priced at {mass}");
}

// ------------------------------------------------------------ budgets

/// Budgets refuse typed and PRICED, and the receipt still holds: a
/// drive that runs out of leaves refuses the frontier it did not
/// examine rather than keeping a partial answer.
#[test]
fn an_exhausted_leaf_budget_refuses_typed_and_priced() {
    let doc = slab(20.0 * eps(), 40.0 * eps());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(4), Tol::witness()).unwrap();
    assert!(v.receipt().holds(), "receipt: {:?}", v.receipt());
    assert!(
        v.refused()
            .iter()
            .any(|l| matches!(l.reason, RefusalReason::Budget(BudgetKind::Leaves { .. }))),
        "the tiny budget should have fired"
    );
    let priced = v.accounting().refused[&ReasonClass::Budget]
        .clone()
        .unwrap();
    assert!(priced > 0.0, "budget mass priced at {priced}");
    let total = v.accounting().total().unwrap();
    assert!((total - 1.0).abs() <= 1e-9, "accounting sums to {total}");
}

/// A depth budget of zero cannot split at all, so the whole box lands
/// in ONE leaf — typed, never a silent certification.
#[test]
fn an_exhausted_depth_budget_refuses_the_whole_box() {
    let doc = slab(20.0 * eps(), 40.0 * eps());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_depth: 0,
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .unwrap();
    assert!(v.receipt().holds());
    assert_eq!(v.receipt().splits, 0);
    assert_eq!(v.receipt().certified + v.receipt().refused, 1);
    assert!(matches!(
        v.refused().first().map(|l| &l.reason),
        Some(RefusalReason::Budget(BudgetKind::Depth { max_depth: 0 }))
    ));
    // WHAT THE DEPTH BUDGET IS PER: the CHOSEN axis. A box whose split
    // axis has spent its budget refuses even if another varying axis
    // still has budget left — the driver does not fall through to a
    // second-choice axis. Benign under the relative-width rule, which
    // picks the widest axis relative to the root and therefore spends
    // the axes evenly: an axis reaches the bound only once every axis
    // is within one bisection of it. Stated because it is a real
    // difference from "the box refuses when every axis is exhausted",
    // and because falling through would make the split rule
    // non-deterministic in the budget's shadow.
}

// ------------------------------------------------------- band pricing

/// **Certification is measure-free; pricing is not.** With a `Band`
/// parameter varying, leaves certify and refuse exactly as they would
/// otherwise, and the ACCOUNTING columns refuse typed, naming the band.
#[test]
fn a_band_parameter_certifies_normally_and_prices_nothing() {
    let w = eps() / 4.0;
    let banded = two_param_plate(Distribution::Band { lo: -w, hi: w }, uniform(w));
    let priced = two_param_plate(uniform(w), uniform(w));
    let analyzed = analyzed_box(&banded, &AnalysisPolicy::default());
    let v = drive(&banded, &analyzed, &config(256), Tol::witness()).expect("the nominal builds");
    assert!(v.receipt().holds());
    assert!(
        !v.certified().is_empty(),
        "a band must not stop certification"
    );
    // Certification is UNCHANGED by the band: the same document with a
    // uniform in place of the band produces the same leaf structure.
    let u = drive(
        &priced,
        &analyzed_box(&priced, &AnalysisPolicy::default()),
        &config(256),
        Tol::witness(),
    )
    .unwrap();
    assert_eq!(v.receipt(), u.receipt());
    assert_eq!(v.witness_vector().key(), u.witness_vector().key());

    // The pricing, and only the pricing, refuses — naming the band.
    assert_eq!(
        v.accounting()
            .certified
            .clone()
            .expect_err("a band prices nothing"),
        editor_core::MeasureUnavailable::BandHasNoMeasure {
            param: name("hole_r")
        }
    );
    assert!(v.accounting().total().is_err());
    assert!(u.accounting().total().is_ok());
    // The tail column is still answerable: a band's support is bounded,
    // and zero-outside is the one answer every measure on it agrees on.
    assert_eq!(v.accounting().unanalyzed, Ok(0.0));
}

// ------------------------------------------------------- containment

/// **Chamber containment** (E2's amendment) is a free predicate on the
/// leaf set, and it says what it says: it does NOT fire while a leaf
/// touching the analyzed box's boundary certifies, and it agrees with
/// its own definition re-derived from the shipped leaves.
#[test]
fn containment_does_not_fire_while_a_boundary_leaf_certifies() {
    let doc = slab(20.0 * eps(), 40.0 * eps());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(4096), Tol::witness()).unwrap();
    let root = v.root();
    assert!(
        v.certified()
            .iter()
            .any(|l| l.box_.touches_boundary_of(root)),
        "this box's witness side reaches its own boundary"
    );
    assert!(!v.accounting().containment);

    // Re-derived from the shipped leaves rather than trusted: every
    // boundary leaf is FlipCrossing-refused, and at least one leaf
    // touches the boundary.
    let boundary_certified = v
        .certified()
        .iter()
        .any(|l| l.box_.touches_boundary_of(root));
    let mut saw = false;
    let mut all_flips = true;
    for leaf in v.refused() {
        if leaf.box_.touches_boundary_of(root) {
            saw = true;
            all_flips &= matches!(leaf.reason, RefusalReason::FlipCrossing { .. });
        }
    }
    assert_eq!(
        v.accounting().containment,
        !boundary_certified && saw && all_flips
    );
}

// ------------------------------------------------------- preconditions

/// A document with no varying axis has no box to subdivide, and the
/// driver says so rather than returning a one-leaf verdict that looks
/// like an analysis.
#[test]
fn a_document_with_nothing_varying_refuses_up_front() {
    let doc = slab(1.0, 0.0);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    assert_eq!(
        drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness()),
        Err(DriveRefusal::NothingVaries)
    );
}

/// A witness that does not build has no branch for leaves to be
/// certified against, and the driver refuses BEFORE subdividing —
/// which is also what keeps `Infeasible` unreachable rather than
/// becoming the drawer this case falls into.
#[test]
fn a_witness_that_does_not_build_refuses_up_front() {
    // A nominal extrusion depth INSIDE the ambiguity band: the f64
    // build itself cannot classify it.
    let doc = slab(5.0 * eps(), eps());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let err = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect_err("a sliver-thin nominal does not build");
    assert!(matches!(err, DriveRefusal::WitnessDoesNotBuild { .. }));
}

// ------------------------------------------------------------ the fence

/// **The box seam is inert when unused.** An evaluation with no box and
/// one with a fully FIXED box make the same decisions, so the door this
/// unit opened added none.
#[test]
fn an_unused_box_seam_changes_no_decision() {
    let doc = slab(1.0, 0.0);
    let opts = EvalOptions {
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    let plain = evaluate::<f64>(&doc, None, &CancelToken::new(), &opts, Tol::witness());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let boxed = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions {
            param_box: Some(Arc::new(ParamBox::of(&analyzed))),
            ..opts
        },
        Tol::witness(),
    );
    assert_eq!(VerdictVector::of(&plain), VerdictVector::of(&boxed));
    assert!(
        VerdictVector::of(&plain)
            .rows
            .iter()
            .any(|r| !r.verdicts.is_empty()),
        "the comparison must have decisions in it to compare"
    );
}

// ---------------------------------------------------------------------
// R1 review probe (CERT-4): what the driver's widening is actually made
// of. A period-fold widening is CONSTANT (~a period) whatever the box;
// a dependency-problem widening SCALES with the box. This row drives the
// certification predicate across decades of half-width and reports
// which shape the measurement has.
// ---------------------------------------------------------------------
#[test]
fn cert4r1_the_driver_widening_scales_with_the_box_not_with_a_period() {
    let e = eps();
    let mut certified_any = false;
    let mut refused_any = false;
    for k in [1.0f64, 1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6] {
        let half = e * k;
        let doc = slab(1.0, half);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let v = drive(&doc, &analyzed, &config(64), Tol::witness()).unwrap();
        let r = v.receipt();
        println!(
            "cert4r1 half={half:e} (eps*{k:e}): splits {} certified {} refused {}",
            r.splits,
            r.certified,
            v.refused().len()
        );
        if r.certified > 0 {
            certified_any = true;
        } else {
            refused_any = true;
        }
    }
    // The discriminator: a floor-straddle widening would be period-wide
    // at EVERY half-width and would never certify. It certifies once the
    // box is small enough, so the widening is a function of the box.
    let _ = refused_any;
    // Measured: every half-width at or below eps certifies (whole, with
    // no splits, below eps/10), while the macroscopic box of the pin row
    // above (half = 0.05, ~5e7 eps) certifies nothing. The widening is a
    // function of the box, which is what a dependency problem looks like
    // and is not what a floor straddle looks like — a period-wide
    // enclosure would refuse at every half-width here.
    assert!(
        certified_any,
        "nothing certified at any half-width — the widening is box-independent, \
         which would be the period-fold shape the PR says it is not"
    );
}
