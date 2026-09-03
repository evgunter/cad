//! **R1's independent consumer probes for the E6 driver** (PR #1231,
//! frozen head `54a77ad9`).
//!
//! Everything here re-derives the unit's claims from the public doors
//! only, on fixtures of this reviewer's OWN construction — different
//! geometry than the PR's suite wherever the claim allows it. Rows
//! marked EVIDENCE-ONLY assert current behavior as a record for the
//! review, not as a contract.
//!
//! No fuzzing: every row is a written-down witness (static fixture),
//! per `memories/test-suite-cost.md` — no seeds anywhere.
//!
//! The basename carries `interval` so `ci-filter.py` pins the lane.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::UnitSym;
use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{
    BudgetKind, DriveConfig, MeasureAccounting, ReasonClass, RefusalReason, drive,
};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, DocParam, EvalOptions, Expr, LoopProgram, Node,
    NodeErrorKind, NodeResult, ParamName, ProfileDoc, ProfileProgram, evaluate,
};
use geom_core::Tol;

use fixture::{Recorder, len};

fn eps() -> f64 {
    Tol::witness().eps()
}

fn name(n: &str) -> ParamName {
    ParamName::new(n)
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

/// One extrude whose distance is the parameter `q` (the PR's slab, kept
/// only as the substrate for boxes this reviewer sizes differently).
fn slab_with(dist: Distribution, nominal: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("q"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(dist),
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
        distance: Expr::param(name("q"), Dimension::Length),
    });
    r.doc
}

/// **A bounded chamber**: two extrudes, distances `q` and `c - q`, so
/// the witness branch holds only for `q` inside an interval bounded on
/// BOTH sides — the geometry a containment-firing drive needs, and
/// different geometry from every fixture the PR ships.
fn bounded_chamber(c: f64, nominal: f64, half: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("q"),
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
        loops: vec![unit_square()],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: Expr::param(name("q"), Dimension::Length),
    });
    let xy_frame_2 = r.insert(Node::Datum(editor_core::Datum::Frame {
        origin: [0.0, 0.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Length).unwrap()),
        u: [1.0, 0.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Scalar).unwrap()),
        v: [0.0, 1.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Scalar).unwrap()),
    }));
    let p2 = r.insert(Node::Profile(ProfileProgram {
        plane: xy_frame_2,
        loops: vec![unit_square()],
    }));
    r.insert(Node::Extrude {
        profile: p2,
        distance: Expr::sub(len(c), Expr::param(name("q"), Dimension::Length))
            .expect("length minus length"),
    });
    r.doc
}

// ------------------------------------------------------------- claim 4
// Accounting honesty: the analyzed columns and the tail must compose to
// 1. The columns are UNCONDITIONAL masses (a leaf's mass is
// `P(offset in leaf)` under the full distribution, not conditional on
// the box), so the composition is ADDITIVE: inside + tail = 1.

/// The columns of a fully-certified drive plus the tail must total 1.
/// Exercised through the public doors with the one distribution that
/// HAS a tail (`Normal`), which no shipped fixture drives.
///
/// **RED on 54a77ad9 — the review's counterexample.** Observed:
/// certified 0.9973, tail 0.0027, `total()` 0.99730729. The columns
/// are unconditional masses but `total()` composes them as if they
/// were conditional (`t * (1 - tail) + tail`), so the total falls
/// short of 1 by `tail * (1 - tail)` whenever the tail is nonzero.
/// E2 says the tail is "an explicit additive term".
#[test]
fn a_normal_axis_drive_totals_one_including_its_tail() {
    // sigma small enough that the +/-3 sigma quantile box is narrower
    // than the certification width, so the whole box certifies and the
    // certified column is exactly the box's own mass.
    let doc = slab_with(
        Distribution::Normal {
            sigma: eps() / 100.0,
        },
        1.0,
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(64), Tol::witness()).unwrap();
    assert!(v.receipt().holds());
    assert!(!v.certified().is_empty(), "the tiny box must certify");
    let unanalyzed = v.accounting().unanalyzed.clone().unwrap();
    assert!(
        unanalyzed > 0.0,
        "a Normal axis's quantile box leaves real tail mass out"
    );
    let total = v.accounting().total().unwrap();
    assert!(
        (total - 1.0).abs() <= 1e-9,
        "certified {} + tail {} totals {}, not 1",
        v.accounting().certified.clone().unwrap(),
        unanalyzed,
        total
    );
}

/// The same composition, isolated from the driver: the columns are
/// public fields, so the arithmetic can be checked directly. A verdict
/// whose inside mass is 0.9 and whose tail is 0.1 accounts for
/// everything: the total is 1.
///
/// **RED on 54a77ad9 — the review's counterexample.** Observed:
/// `total()` = 0.9*(1-0.1)+0.1 = 0.91, and `unresolved()` = 0.28
/// where refused 0.2 + tail 0.1 should unresolve 0.3.
#[test]
fn total_composes_inside_mass_and_tail_additively() {
    let acc = MeasureAccounting {
        certified: Ok(0.9),
        refused: BTreeMap::new(),
        unanalyzed: Ok(0.1),
        containment: false,
    };
    let total = acc.total().unwrap();
    assert!(
        (total - 1.0).abs() <= 1e-12,
        "inside 0.9 + tail 0.1 totals {total}"
    );
    // And `unresolved` is refused-plus-tail the same way.
    let acc2 = MeasureAccounting {
        certified: Ok(0.7),
        refused: BTreeMap::from([(ReasonClass::Budget, Ok(0.2))]),
        unanalyzed: Ok(0.1),
        containment: false,
    };
    let unresolved = acc2.unresolved().unwrap();
    assert!(
        (unresolved - 0.3).abs() <= 1e-12,
        "refused 0.2 + tail 0.1 unresolves {unresolved}"
    );
}

// ------------------------------------------------------------- claim 3
// Receipt identity on drives of this reviewer's construction.

/// `max_leaves = 0`: the root itself cannot fit, the whole frontier
/// refuses `Budget`, and the receipt still holds.
#[test]
fn a_zero_leaf_budget_refuses_the_root_and_the_receipt_holds() {
    let doc = slab_with(Distribution::Uniform { lo: -1.0, hi: 1.0 }, 20.0 * eps());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(0), Tol::witness()).unwrap();
    assert!(v.receipt().holds(), "receipt: {:?}", v.receipt());
    assert_eq!(v.receipt().certified + v.receipt().refused, 1);
    assert!(matches!(
        v.refused().first().map(|l| &l.reason),
        Some(RefusalReason::Budget(BudgetKind::Leaves { max_leaves: 0 }))
    ));
}

/// `max_leaves = 1` on a box that must split: one split happens, then
/// the two children cannot fit and refuse. Identity: 0 + 2 == 1 + 1.
#[test]
fn a_one_leaf_budget_still_covers_the_box_exactly() {
    let doc = slab_with(
        Distribution::Uniform {
            lo: -40.0 * eps(),
            hi: 40.0 * eps(),
        },
        20.0 * eps(),
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(1), Tol::witness()).unwrap();
    assert!(v.receipt().holds(), "receipt: {:?}", v.receipt());
    let budget_mass = v.accounting().refused[&ReasonClass::Budget]
        .clone()
        .unwrap();
    let certified = v.accounting().certified.clone().unwrap_or(0.0);
    assert!(
        (certified + budget_mass - 1.0).abs() <= 1e-9,
        "the refused frontier plus anything certified covers the box"
    );
}

/// The `f64` grid floor: a box one ulp wide cannot be bisected; the
/// refusal is typed `Budget(Resolution)` (or the box certifies whole),
/// and the receipt holds either way.
#[test]
fn an_unsplittable_box_refuses_resolution_or_certifies_and_the_receipt_holds() {
    // An axis so narrow its midpoint rounds onto an endpoint within a
    // few splits.
    let w = f64::EPSILON * 4.0;
    let doc = slab_with(Distribution::Uniform { lo: -w, hi: w }, 1.0);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(4096), Tol::witness()).unwrap();
    assert!(v.receipt().holds(), "receipt: {:?}", v.receipt());
    for leaf in v.refused() {
        assert!(
            matches!(
                leaf.reason,
                RefusalReason::Budget(BudgetKind::Resolution)
                    | RefusalReason::Budget(BudgetKind::Depth { .. })
            ),
            "unexpected refusal at the grid floor: {:?}",
            leaf.reason
        );
    }
}

// ------------------------------------------------------------- claim 5
// D9 determinism on a drive of this reviewer's construction: the
// bounded-chamber document, whose refinement pattern (two flip walls,
// a band region, budget floors) is nothing like the PR's fixture.

#[test]
fn my_own_parallel_drive_is_bit_identical_across_schedules_and_repeats() {
    let doc = bounded_chamber(60.0 * eps(), 30.0 * eps(), 100.0 * eps());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let seq = drive(&doc, &analyzed, &config(4096), Tol::witness()).unwrap();
    let par = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            parallel: true,
            ..config(4096)
        },
        Tol::witness(),
    )
    .unwrap();
    let par2 = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            parallel: true,
            ..config(4096)
        },
        Tol::witness(),
    )
    .unwrap();
    assert!(
        seq.receipt().splits > 16 && !seq.certified().is_empty() && !seq.refused().is_empty(),
        "not a vacuous comparison: {:?}",
        seq.receipt()
    );
    assert_eq!(seq.serialize(), par.serialize());
    assert_eq!(par.serialize(), par2.serialize());
    assert_eq!(seq.content_key(), par.content_key());
}

// ------------------------------------------------------------- claim 6
// Flip honesty on this reviewer's own geometry: the SECOND extrude's
// sign predicate is the one that flips at the right-hand wall, and the
// named flip says so.

#[test]
fn a_flip_on_the_far_wall_names_the_second_extrudes_predicate() {
    let doc = bounded_chamber(60.0 * eps(), 30.0 * eps(), 100.0 * eps());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(4096), Tol::witness()).unwrap();
    assert!(v.receipt().holds());
    // Both walls flip: collect every named predicate flip with the box
    // it refused on.
    let mut left_wall = false;
    let mut right_wall = false;
    for leaf in v.refused() {
        let RefusalReason::FlipCrossing { flipped } = &leaf.reason else {
            continue;
        };
        let (lo, hi) = leaf.box_.get(&name("q")).unwrap().span();
        let (alo, ahi) = (30.0 * eps() + lo, 30.0 * eps() + hi);
        // The evidence is `resolve::vdiff`'s `FlipSet`: per node, the
        // NET per-predicate sign change. The claim is unchanged — the
        // extrusion's own sign predicate flips positive-to-negative —
        // only the engine that names it is.
        for f in flipped.verdicts.nodes.values().flat_map(|d| &d.flips) {
            if f.predicate == "extrusion_normal_component"
                && f.from == geom_core::Sign::Positive
                && f.to == geom_core::Sign::Negative
            {
                // The flip must be on the side where that distance
                // really is negative: q < 0 for the first extrude,
                // q > c for the second.
                if ahi <= 0.0 {
                    left_wall = true;
                }
                if alo >= 60.0 * eps() {
                    right_wall = true;
                }
            }
        }
    }
    assert!(
        left_wall && right_wall,
        "both walls must refuse with a truthfully-located extrusion sign flip \
         (left: {left_wall}, right: {right_wall})"
    );
}

// ------------------------------------------------------------- claim 4
// Containment's POSITIVE arm, which no shipped fixture exercises: on a
// chamber bounded inside the box, every boundary leaf flips and the
// verdict reports containment.

#[test]
fn containment_fires_when_the_chamber_is_bounded_inside_the_box() {
    let doc = bounded_chamber(60.0 * eps(), 30.0 * eps(), 100.0 * eps());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(4096), Tol::witness()).unwrap();
    assert!(v.receipt().holds());
    // Re-derive the predicate from the shipped leaves first, so a
    // failure distinguishes "the driver disagrees with its own
    // definition" from "this fixture did not corner the chamber".
    let root = v.root();
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
        !boundary_certified && saw && all_flips,
        "the shipped bool must agree with its own definition"
    );
    assert!(
        v.accounting().containment,
        "a chamber bounded inside the box must report containment \
         (boundary_certified: {boundary_certified}, saw: {saw}, all_flips: {all_flips})"
    );
}

// ------------------------------------------------------------- claim 4
// Band pricing: the Uniform-for-Band swap pinned INDEPENDENTLY — not
// just receipts and witness keys but the full leaf partition, box by
// box, class by class.

#[test]
fn the_band_and_uniform_drives_ship_the_same_leaf_partition() {
    let w = 40.0 * eps();
    let banded = slab_with(Distribution::Band { lo: -w, hi: w }, 20.0 * eps());
    let uniform = slab_with(Distribution::Uniform { lo: -w, hi: w }, 20.0 * eps());
    let vb = drive(
        &banded,
        &analyzed_box(&banded, &AnalysisPolicy::default()),
        &config(4096),
        Tol::witness(),
    )
    .unwrap();
    let vu = drive(
        &uniform,
        &analyzed_box(&uniform, &AnalysisPolicy::default()),
        &config(4096),
        Tol::witness(),
    )
    .unwrap();
    let boxes = |v: &editor_core::ParamBoxVerdict| {
        let c: Vec<ParamBox> = v.certified().iter().map(|l| l.box_.clone()).collect();
        let r: Vec<(ParamBox, ReasonClass)> = v
            .refused()
            .iter()
            .map(|l| (l.box_.clone(), l.reason.class()))
            .collect();
        (c, r)
    };
    assert_eq!(boxes(&vb), boxes(&vu), "certification is measure-free");
    assert_eq!(vb.witness_vector().key(), vu.witness_vector().key());
    // The band's accounting refuses typed, naming the parameter; the
    // uniform's prices.
    assert!(vb.accounting().certified.is_err());
    assert!(vu.accounting().certified.is_ok());
}

// ------------------------------------------------------------- claim 8
// The wrapped-escalation arm (reported deviation 3): a box reaching
// into the ambiguity band on the extrusion distance — whose escalation
// is wrapped in `ExtrudeError` — must never certify a leaf that sits
// wholly inside the band. The honest floor is `Budget`.

#[test]
fn a_wrapped_escalation_never_certifies_inside_the_band() {
    // Box [5eps, 35eps]: reaches into (eps, 10eps) but not across zero.
    let doc = slab_with(
        Distribution::Uniform {
            lo: -15.0 * eps(),
            hi: 15.0 * eps(),
        },
        20.0 * eps(),
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(&doc, &analyzed, &config(4096), Tol::witness()).unwrap();
    assert!(v.receipt().holds());
    assert!(!v.certified().is_empty(), "the definite side must certify");
    // No certified leaf's absolute distance interval may sit wholly
    // inside the band (eps, K*eps), K = 10: there the extrusion is
    // genuinely undecidable and a certificate would be false.
    for leaf in v.certified() {
        let (lo, hi) = leaf.box_.get(&name("q")).unwrap().span();
        let (alo, ahi) = (20.0 * eps() + lo, 20.0 * eps() + hi);
        assert!(
            !(alo > eps() && ahi < 10.0 * eps()),
            "a leaf wholly inside the band certified: [{alo:e}, {ahi:e}]"
        );
    }
    // And the band region's mass is refused SOMEHOW — typed, priced.
    assert!(
        v.accounting()
            .refused
            .values()
            .any(|m| *m.as_ref().unwrap() > 0.0),
        "the band region must be refused mass"
    );
}

// ---------------------------------------------------------- claim 1
// The door's value-degenerate boundary. EVIDENCE-ONLY: a box whose
// axis spans [-0.0, +0.0] is a point in value but is refused at f64,
// because the door compares bits (documented on `AxisScalar for f64`).
// Recorded here so the review can cite the behavior, not to freeze it.

#[test]
fn evidence_only_a_negative_zero_axis_refuses_at_f64() {
    let doc = slab_with(Distribution::Uniform { lo: -1.0, hi: 1.0 }, 1.0);
    let mut axes = BTreeMap::new();
    axes.insert(name("q"), BoxAxis::Varying { lo: -0.0, hi: 0.0 });
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::from_axes(axes))),
        ..EvalOptions::default()
    };
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &opts, Tol::witness());
    assert!(ev.order.iter().all(|id| matches!(
        ev.result(*id),
        Some(NodeResult::Failed(e)) if matches!(e.kind, NodeErrorKind::ParamBox { .. })
    )));
}

// ------------------------------------------------------------ e2e walk
// EVIDENCE-ONLY, run by hand (`--ignored --nocapture`): the consumer
// exercise the review brief requires. A two-parameter document driven
// at several widths; the receipt, the mass table, and the serialized
// verdict read as a consumer would read them.

/// R1's consumer walk. Not an assertion row; it prints what a consumer
/// sees.
#[test]
#[ignore = "R1 evidence: prints the consumer's view at several widths"]
fn evidence_only_e2e_consumer_walk() {
    let mk = |half_r: f64, half_d: f64| {
        let mut r = Recorder::new();
        r.push(DocEdit::SetDocParam {
            name: name("hole_r"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.25,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(Distribution::Uniform {
                    lo: -half_r,
                    hi: half_r,
                }),
            },
        });
        r.push(DocEdit::SetDocParam {
            name: name("depth"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.5,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(Distribution::Normal { sigma: half_d }),
            },
        });
        let xy_frame_3 = r.insert(Node::Datum(editor_core::Datum::Frame {
            origin: [0.0, 0.0, 0.0]
                .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Length).unwrap()),
            u: [1.0, 0.0, 0.0]
                .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Scalar).unwrap()),
            v: [0.0, 1.0, 0.0]
                .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Scalar).unwrap()),
        }));
        let p = r.insert(Node::Profile(ProfileProgram {
            plane: xy_frame_3,
            loops: vec![
                LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)])
                    .expect("finite plate corners"),
                LoopProgram::Circle {
                    centre: [len(1.0), len(1.0)],
                    radius: Expr::param(name("hole_r"), Dimension::Length),
                },
            ],
        }));
        r.insert(Node::Extrude {
            profile: p,
            distance: Expr::param(name("depth"), Dimension::Length),
        });
        r.doc
    };
    for (label, half_r, half_d) in [
        ("eps/8 box", eps() / 16.0, eps() / 48.0),
        ("2eps box", eps(), eps() / 3.0),
        ("50eps box", 25.0 * eps(), 8.0 * eps()),
        ("macroscopic 1e-3", 5e-4, 1.6e-4),
    ] {
        let doc = mk(half_r, half_d);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let v = drive(&doc, &analyzed, &config(512), Tol::witness()).unwrap();
        println!("== {label}: receipt {:?}", v.receipt());
        println!("   containment {}", v.accounting().containment);
        println!(
            "   certified {:?}  unanalyzed {:?}  total {:?}  unresolved {:?}",
            v.accounting().certified,
            v.accounting().unanalyzed,
            v.accounting().total(),
            v.accounting().unresolved()
        );
        for (class, m) in &v.accounting().refused {
            println!("   refused {:?} {:?}", class, m);
        }
        let s = v.serialize();
        println!("   serialize: {} bytes, first lines:", s.len());
        for line in s.lines().take(3) {
            println!("     {line}");
        }
    }
}
