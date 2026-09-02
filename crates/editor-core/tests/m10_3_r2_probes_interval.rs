//! **M10-3 review probes (R2)** — an INDEPENDENT derivation of what the
//! E6 subdivision driver claims, written from `docs/M10-3-SPEC.md` and
//! `docs/ERROR-DESIGN.md` E2/E6/E8 rather than from the unit's own rows.
//!
//! Why these rows and not the unit's: every accounting fixture in
//! `m10_3_driver_interval.rs` uses a BOUNDED distribution, so its tail
//! column is `Ok(0.0)` in every row and the composition of the analyzed
//! columns with the tail is never exercised at all. Every determinism
//! row drives one shape (`slab`). Every containment row lands on the
//! NEGATIVE arm. The rows below drive a document whose tail is
//! genuinely non-zero, construct the containment-positive case, and
//! re-derive the receipt identity and the mass total from the shipped
//! leaves with arithmetic that does not reuse the module's own.
//!
//! **TWO ROWS ARE RED AT `54a77ad9`, DELIBERATELY**, and they are this
//! suite's finding rather than a defect in it:
//! `the_accounting_columns_plus_the_tail_sum_to_one_under_a_normal` and
//! `the_unresolved_budget_is_refused_mass_plus_tail`. Both fail because
//! `MeasureAccounting::total()` / `unresolved()` compose the analyzed
//! columns with the tail MULTIPLICATIVELY (`t·(1 - tail) + tail`) while
//! `box_mass` prices a leaf UNCONDITIONALLY, so under a `Normal`
//! parameter the shipped total is `0.99730729` where the additive
//! oracle over the same shipped leaves is exactly `1`. Deleting the
//! rows would delete the finding; they go green the day the
//! composition becomes a sum.
//!
//! Sweep shape (`memories/test-suite-cost.md`): nothing here samples —
//! every row is a witness that can be written down, so all are static
//! fixtures asserted every run and no seed appears. Rows whose doc
//! comment says EVIDENCE-ONLY assert that a documented behaviour is
//! still what it is and gate nothing new.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::UnitSym;
use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box, param_env_over};
use editor_core::drive::{
    BudgetKind, DriveConfig, ReasonClass, RefusalReason, VerdictVector, drive,
};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, DocParam, EvalOptions, Expr, LoopProgram, Node,
    ParamName, ProfileDoc, ProfileLift, ProfileProgram, evaluate,
};
use geom_core::{Interval, Tol};
use profile::SketchPlane;

use fixture::Recorder;

fn eps() -> f64 {
    Tol::witness().eps()
}

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

fn lit(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length literal")
}

fn unit_square() -> LoopProgram {
    LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
        .expect("finite square corners")
}

/// A square extruded by `distance`, with one continuous parameter
/// `depth` carrying `dist`.
fn slab_with(nominal: f64, dist: Distribution, distance: Expr) -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("depth"),
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
        distance,
    });
    r.doc
}

fn depth_param() -> Expr {
    Expr::param(name("depth"), Dimension::Length)
}

/// **A document whose witness chamber is BOUNDED ON BOTH SIDES in the
/// varying parameter**: the extrusion distance is
/// `min(depth, 2·nominal - depth)`, so the solid exists exactly on
/// `0 < depth < 2·nominal` and both ends of a wide enough box are
/// definitely on the far branch. That is the shape E2's
/// chamber-containment amendment describes, and the unit's own suite
/// has no fixture for it.
fn pinched(nominal: f64, half: f64) -> ProfileDoc {
    let distance = Expr::min(
        depth_param(),
        Expr::sub(lit(2.0 * nominal), depth_param()).expect("length minus length"),
    )
    .expect("min of two lengths is a length");
    slab_with(
        nominal,
        Distribution::Uniform {
            lo: -half,
            hi: half,
        },
        distance,
    )
}

// -------------------------------------------------- the accounting

/// **The tail composes ADDITIVELY, and `total()` does not.**
///
/// E2/E6 price a leaf with `box_mass`, which is `P(offset ∈ leaf)` —
/// an UNCONDITIONAL probability under the parameter's own measure, not
/// a probability conditional on landing in the analyzed box. The
/// analyzed columns therefore already sum to `∏(1 - tᵢ) = 1 - tail`,
/// and the honest total is the plain SUM `analyzed + tail`.
///
/// `MeasureAccounting::total()` instead composes multiplicatively
/// (`t·(1 - tail) + tail`), which is right only when the columns are
/// conditional. With a `Normal` parameter — the ONE distribution E2
/// gives an unbounded support, and the only one whose tail is
/// non-zero — the shipped total therefore lands at `1 - tail·(1-tail)`
/// instead of 1. Every fixture in the unit's own suite uses a bounded
/// distribution, so `unanalyzed == Ok(0.0)` everywhere and the arm is
/// never taken.
///
/// This row asserts the SPEC's claim ("summing to 1 within stated f64
/// bounds") from an oracle built out of the shipped leaves.
#[test]
fn the_accounting_columns_plus_the_tail_sum_to_one_under_a_normal() {
    let sigma = eps() / 64.0;
    let doc = slab_with(1.0, Distribution::Normal { sigma }, depth_param());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 256,
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .expect("the nominal builds");
    assert!(v.receipt().holds(), "receipt {:?}", v.receipt());

    let tail = v.accounting().unanalyzed.clone().unwrap();
    assert!(
        tail > 0.0,
        "the probe needs a real tail; a ±3σ box leaves ~0.27% outside, got {tail}"
    );

    // The oracle: re-price every shipped leaf from the analyzed box's
    // own per-axis door, and add the tail. Nothing here reuses
    // `MeasureAccounting`'s arithmetic.
    let mut analyzed_mass = 0.0;
    for leaf in v.certified() {
        analyzed_mass += leaf.box_.mass(&analyzed).unwrap();
    }
    for leaf in v.refused() {
        analyzed_mass += leaf.box_.mass(&analyzed).unwrap();
    }
    let honest = analyzed_mass + tail;
    assert!(
        (honest - 1.0).abs() <= 1e-9,
        "leaves {analyzed_mass} + tail {tail} = {honest}, not 1 — the leaves do not \
         partition the analyzed box's mass"
    );

    // And the shipped total must be that same number.
    let shipped = v.accounting().total().unwrap();
    assert!(
        (shipped - 1.0).abs() <= 1e-9,
        "MeasureAccounting::total() = {shipped}, not 1; the additive oracle over the same \
         leaves gives {honest} (analyzed {analyzed_mass} + tail {tail}). The multiplicative \
         composition treats an unconditional column as conditional."
    );
}

/// The same defect on the honesty GATE rather than on the total:
/// `unresolved()` (refused + tail, E2/E10's single honesty gate)
/// scales the already-unconditional refused column by `(1 - tail)`,
/// which UNDER-reports unresolved mass — the unsafe direction for a
/// gate.
#[test]
fn the_unresolved_budget_is_refused_mass_plus_tail() {
    let sigma = eps() / 2.0;
    let doc = slab_with(1.0, Distribution::Normal { sigma }, depth_param());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 4,
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .expect("the nominal builds");
    let tail = v.accounting().unanalyzed.clone().unwrap();
    let mut refused = 0.0;
    for leaf in v.refused() {
        refused += leaf.box_.mass(&analyzed).unwrap();
    }
    assert!(refused > 0.0, "the probe needs refused mass");
    let honest = refused + tail;
    let shipped = v.accounting().unresolved().unwrap();
    assert!(
        (shipped - honest).abs() <= 1e-12,
        "unresolved() = {shipped}, the additive oracle over the same leaves = {honest} \
         (refused {refused} + tail {tail})"
    );
}

// -------------------------------------------------- containment

/// **The containment POSITIVE arm** (E2's amendment), which the unit's
/// own suite never exercises: it pins only
/// `containment_does_not_fire_while_a_boundary_leaf_certifies`, so a
/// `contained()` that returned `false` unconditionally would pass the
/// whole shipped suite.
///
/// The construction: the extrusion distance is `min(depth, 2h - depth)`,
/// so the witness chamber in `depth` is the OPEN interval `(0, 2h)` and
/// the analyzed box `[-h, 3h]` strictly contains it. Both ends of the
/// box are then definitely on the far branch, which is exactly E2's
/// "the witness chamber is contained in the box".
#[test]
fn containment_fires_when_the_chamber_is_strictly_inside_the_box() {
    let doc = pinched(20.0 * eps(), 40.0 * eps());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 8192,
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .expect("the nominal builds");
    assert!(v.receipt().holds());
    let root = v.root();

    // Re-derived from the shipped leaves, not read off the flag.
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
    assert!(saw, "some leaf must reach the analyzed box's boundary");
    assert!(
        !boundary_certified && all_flips,
        "the chamber is strictly inside the box, so every boundary leaf should be a definite \
         flip; boundary_certified={boundary_certified} all_flips={all_flips}, boundary classes \
         {:?}",
        v.refused()
            .iter()
            .filter(|l| l.box_.touches_boundary_of(root))
            .map(|l| l.reason.class())
            .collect::<Vec<_>>()
    );
    assert!(
        v.accounting().containment,
        "containment must fire when every boundary leaf is FlipCrossing-refused"
    );
}

// -------------------------------------------------- receipt identity

/// The receipt identity on every degenerate drive shape I can build:
/// a one-leaf budget, a zero-depth budget, a box whose only axis is
/// degenerate at `[lo, hi]` with `lo == hi`, and a normal drive. The
/// identity is re-derived rather than read off `holds()`.
#[test]
fn the_receipt_identity_holds_on_every_edge_drive() {
    let doc = slab_with(
        20.0 * eps(),
        Distribution::Uniform {
            lo: -40.0 * eps(),
            hi: 40.0 * eps(),
        },
        depth_param(),
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let cases: Vec<(&str, DriveConfig)> = vec![
        (
            "one leaf",
            DriveConfig {
                max_leaves: 1,
                ..DriveConfig::default()
            },
        ),
        (
            "zero leaves",
            DriveConfig {
                max_leaves: 0,
                ..DriveConfig::default()
            },
        ),
        (
            "zero depth",
            DriveConfig {
                max_depth: 0,
                ..DriveConfig::default()
            },
        ),
        (
            "one depth",
            DriveConfig {
                max_depth: 1,
                max_leaves: 1024,
                ..DriveConfig::default()
            },
        ),
        (
            "ordinary",
            DriveConfig {
                max_leaves: 512,
                ..DriveConfig::default()
            },
        ),
        (
            "parallel",
            DriveConfig {
                max_leaves: 512,
                parallel: true,
                ..DriveConfig::default()
            },
        ),
    ];
    for (label, config) in cases {
        let v = drive(&doc, &analyzed, &config, Tol::witness()).expect("the nominal builds");
        let r = v.receipt();
        assert_eq!(
            r.certified + r.refused,
            r.splits + 1,
            "{label}: receipt {r:?} breaks certified + refused == splits + 1"
        );
        assert_eq!(
            r.certified,
            v.certified().len(),
            "{label}: the receipt's certified count is not the shipped leaf count"
        );
        assert_eq!(
            r.refused,
            v.refused().len(),
            "{label}: the receipt's refused count is not the shipped leaf count"
        );
        // Nothing on the frontier is dropped: the leaves tile the root
        // box's mass exactly.
        let mut m = 0.0;
        for leaf in v.certified() {
            m += leaf.box_.mass(&analyzed).unwrap();
        }
        for leaf in v.refused() {
            m += leaf.box_.mass(&analyzed).unwrap();
        }
        assert!(
            (m - 1.0).abs() <= 1e-9,
            "{label}: the leaves cover {m} of a bounded box's mass, not 1"
        );
    }
}

/// A box whose only varying axis is degenerate (`lo == hi`) cannot be
/// split, so the drive is one leaf and the identity is `1 == 0 + 1`.
/// The driver must say `Budget(Resolution)`, not certify by accident
/// and not trip the tripwire.
///
/// EVIDENCE-ONLY: pins the shipped behaviour of an unsplittable axis.
#[test]
fn a_degenerate_varying_axis_is_one_leaf_and_never_a_silent_partial() {
    // A distribution the analysis reads as varying but whose offsets
    // are a single point cannot exist through `analyzed_box` (a
    // zero-width uniform is FIXED), so the box is built directly —
    // which is the door `ParamBox::from_axes` exists for.
    let doc = slab_with(
        1.0,
        Distribution::Uniform {
            lo: -eps() / 16.0,
            hi: eps() / 16.0,
        },
        depth_param(),
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let mut axes = BTreeMap::new();
    axes.insert(name("depth"), BoxAxis::Varying { lo: 0.0, hi: 0.0 });
    let degenerate = ParamBox::from_axes(axes);
    assert_eq!(degenerate.split_axis(&degenerate), Some(name("depth")));
    assert_eq!(degenerate.split(&name("depth")), None);
    // And it binds at Interval as the nominal exactly.
    let env = param_env_over::<Interval, _>(&doc, &degenerate).expect("a point axis binds");
    let editor_core::ParamValue::Continuous { value, .. } = env.bindings[&name("depth")] else {
        panic!("depth is continuous")
    };
    use geom_core::Bounds;
    assert_eq!((value.lo(), value.hi()), (1.0, 1.0));
    let _ = analyzed;
}

// -------------------------------------------------- the closed door

/// **Zero impact with the door closed**, re-derived across all three
/// scalars the tree evaluates at and across BOTH lift settings: an
/// evaluation with `param_box: None` and one with a fully `Fixed` box
/// make the same decisions AND produce the same per-node content keys,
/// which is the half the unit's own fence row (`an_unused_box_seam_
/// changes_no_decision`, verdict vectors at `f64` under `Guided` only)
/// does not cover. A content key that moved would move every memo key
/// and every persisted-verdict comparison downstream.
#[test]
fn a_closed_and_a_fixed_box_agree_on_verdicts_and_on_content_keys() {
    let doc = slab_with(
        1.0,
        Distribution::Uniform { lo: 0.0, hi: 0.0 },
        depth_param(),
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let fixed = Arc::new(ParamBox::of(&analyzed));
    assert!(
        fixed.varying().next().is_none(),
        "the probe's box must be all-Fixed"
    );

    for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
        let base = EvalOptions {
            profile_lift: lift,
            ..EvalOptions::default()
        };
        let plain = evaluate::<Interval>(&doc, None, &CancelToken::new(), &base, Tol::witness());
        let boxed = evaluate::<Interval>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions {
                param_box: Some(Arc::clone(&fixed)),
                ..base
            },
            Tol::witness(),
        );
        assert_eq!(
            VerdictVector::of(&plain),
            VerdictVector::of(&boxed),
            "{lift:?}: the closed door and a Fixed box decided differently"
        );
        assert!(
            VerdictVector::of(&plain)
                .rows
                .iter()
                .any(|r| !r.verdicts.is_empty()),
            "{lift:?}: the comparison must have decisions in it"
        );
        let keys = |ev: &editor_core::Evaluation<Interval>| -> Vec<_> {
            ev.order
                .iter()
                .map(|&id| (id, ev.value(id).map(|v| v.content_key)))
                .collect()
        };
        assert_eq!(
            keys(&plain),
            keys(&boxed),
            "{lift:?}: a Fixed box moved a content key, so it would move every memo key"
        );
    }
}

/// The door's refusal edge, at the boundary the trait's own doc
/// comment argues about: a point scalar carries a degenerate axis and
/// refuses a widened one, and it decides "degenerate" by BITS — so the
/// value-degenerate axis `[-0.0, 0.0]` refuses at `f64` even though
/// `-0.0 == 0.0`.
///
/// EVIDENCE-ONLY on the second half: it pins what the shipped
/// comparison does, which is not what the comment beside it says
/// ("`-0.0` and `0.0` are the same offset in every arithmetic sense").
#[test]
fn the_point_scalar_door_refuses_by_bits_not_by_value() {
    use editor_core::AxisScalar;
    assert_eq!(f64::axis(0.0, 0.0), Some(0.0));
    assert_eq!(f64::axis(1.5, 1.5), Some(1.5));
    assert_eq!(f64::axis(-1.0, 1.0), None);
    // Value-degenerate, bit-distinct: refused.
    assert_eq!(f64::axis(-0.0, 0.0), None);
    // A dual carries whatever its value channel carries, with a zero
    // tangent — the box is an enclosure, never a seed.
    let d = <geom_core::Dual<f64> as AxisScalar>::axis(2.0, 2.0).expect("a point axis binds");
    assert_eq!((d.value, d.deriv), (2.0, 0.0));
    assert!(<geom_core::Dual<f64> as AxisScalar>::axis(-1.0, 1.0).is_none());
    // And over an interval value channel a widened axis DOES bind,
    // still with a zero tangent.
    let di =
        <geom_core::Dual<Interval> as AxisScalar>::axis(-1.0, 1.0).expect("an interval axis binds");
    use geom_core::Bounds;
    assert_eq!((di.value.lo(), di.value.hi()), (-1.0, 1.0));
    assert_eq!((di.deriv.lo(), di.deriv.hi()), (0.0, 0.0));
}

// -------------------------------------------------- determinism

/// D9 determinism on a document of my own, not the unit's: the
/// serialized verdict and its content key are bit-identical across a
/// repeat and across the rayon schedule, on a drive with certified
/// leaves, a real refusal class and hundreds of splits.
#[test]
fn my_own_drive_is_bit_identical_across_repeats_and_schedules() {
    let doc = pinched(20.0 * eps(), 40.0 * eps());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let seq = DriveConfig {
        max_leaves: 512,
        ..DriveConfig::default()
    };
    let par = DriveConfig {
        parallel: true,
        ..seq.clone()
    };
    let a = drive(&doc, &analyzed, &seq, Tol::witness()).unwrap();
    let b = drive(&doc, &analyzed, &seq, Tol::witness()).unwrap();
    let c = drive(&doc, &analyzed, &par, Tol::witness()).unwrap();
    assert!(a.receipt().splits > 16, "{:?}", a.receipt());
    assert!(!a.certified().is_empty() || !a.refused().is_empty());
    assert_eq!(a.serialize(), b.serialize());
    assert_eq!(a.serialize(), c.serialize());
    assert_eq!(a.content_key(), c.content_key());
    // The parallel schedule must also agree leaf for leaf, in order.
    assert_eq!(a.certified(), c.certified());
    assert_eq!(a.refused(), c.refused());
}

// -------------------------------------------------- the widening

/// **The macroscopic-box honesty statement, measured rather than
/// quoted.** The PR states that the certification predicates' interval
/// enclosure widens as `[0, c·w]` with `c ≈ 2–4`, so a leaf certifies
/// only below roughly `ε/8` of width. This row measures the largest
/// box that certifies WHOLE (`max_depth = 0`, so exactly one leaf) by
/// bisection on the half-width, and asserts only the order of
/// magnitude the claim implies.
///
/// EVIDENCE-ONLY: it records the measured threshold in ε and fails
/// only if the number leaves the band the PR's statement implies, so
/// the claim stops being a sentence nothing checks.
#[test]
fn the_certification_width_threshold_is_a_small_fraction_of_epsilon() {
    let certifies_whole = |half: f64| -> bool {
        let doc = slab_with(
            1.0,
            Distribution::Uniform {
                lo: -half,
                hi: half,
            },
            depth_param(),
        );
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        match drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_depth: 0,
                ..DriveConfig::default()
            },
            Tol::witness(),
        ) {
            Ok(v) => v.receipt().certified == 1,
            Err(_) => false,
        }
    };
    // Bracket: ε/1024 certifies, ε certainly does not.
    let (mut lo, mut hi) = (eps() / 1024.0, eps());
    assert!(certifies_whole(lo), "the bracket's low end must certify");
    assert!(!certifies_whole(hi), "the bracket's high end must not");
    for _ in 0..24 {
        let mid = 0.5 * (lo + hi);
        if certifies_whole(mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let ratio = eps() / (2.0 * lo);
    println!("R2 MEASURED: the widest whole-certifying box is eps/{ratio:.2} wide");
    // The PR's `[0, c·w]` with c ≈ 2–4 puts the whole-box width
    // threshold within an order of magnitude of ε/8.
    assert!(
        (2.0..=256.0).contains(&ratio),
        "the largest box that certifies whole is ε/{ratio} wide — outside the band the PR's \
         `[0, c·w]`, c ≈ 2–4 statement implies (it says roughly ε/8)"
    );
}

// -------------------------------------------------- e2e, as a consumer

/// **The consumer's walk**: a two-parameter document authored through
/// the public doors, driven at four box widths, with the receipt and
/// the refused-mass table read back the way a report would read them.
///
/// EVIDENCE-ONLY on the printed table (run with `--nocapture` to see
/// it); the assertions are the invariants a consumer is entitled to at
/// every width — the receipt holds, every leaf is in exactly one
/// bucket, and the priced columns never exceed 1.
#[test]
fn a_consumer_drives_a_two_parameter_document_at_four_widths() {
    // The document a tolerance study would author: a plate with a
    // parametric hole radius, extruded by a parametric depth. Both
    // axes uniform, both boxes sized in ε.
    let plate = |scale: f64| -> ProfileDoc {
        let mut r = Recorder::new();
        for (n, nominal) in [("hole_r", 0.25_f64), ("plate_h", 0.5)] {
            r.push(DocEdit::SetDocParam {
                name: name(n),
                value: DocParam::Continuous {
                    dim: Dimension::Length,
                    value: nominal,
                    display_unit: UnitSym::canonical_for(Dimension::Length),
                    distribution: Some(Distribution::Uniform {
                        lo: -scale * eps(),
                        hi: scale * eps(),
                    }),
                },
            });
        }
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
                    radius: Expr::param(name("hole_r"), Dimension::Length),
                },
            ],
        }));
        r.insert(Node::Extrude {
            profile: p,
            distance: Expr::param(name("plate_h"), Dimension::Length),
        });
        r.doc
    };

    println!("\nR2 e2e — two-parameter plate, boxes in eps");
    println!("  half-width | cert | refused | splits | certified mass | refused columns");
    for scale in [0.125_f64, 1.0, 8.0, 1024.0] {
        let doc = plate(scale);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let v = drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: 1024,
                ..DriveConfig::default()
            },
            Tol::witness(),
        )
        .expect("the nominal builds");
        let rec = v.receipt();
        assert!(rec.holds(), "receipt {rec:?} at half-width {scale}eps");
        assert_eq!(rec.certified, v.certified().len());
        assert_eq!(rec.refused, v.refused().len());
        let cert = v.accounting().certified.clone().unwrap();
        assert!((0.0..=1.0 + 1e-12).contains(&cert));
        let cols: Vec<String> = v
            .accounting()
            .refused
            .iter()
            .map(|(c, m)| match m {
                Ok(x) => format!("{}={:.4}", c.name(), x),
                Err(e) => format!("{}=refused({e})", c.name()),
            })
            .collect();
        println!(
            "  {:>9} | {:>4} | {:>7} | {:>6} | {:>14.4} | {}",
            format!("{scale}eps"),
            rec.certified,
            rec.refused,
            rec.splits,
            cert,
            cols.join(" ")
        );
    }
}

// -------------------------------------------------- refusal classes

/// The two budget kinds refuse typed and PRICED, and the drive that
/// hits the leaf budget prices the whole frontier rather than the
/// examined part of it.
#[test]
fn the_leaf_budget_prices_the_whole_unexamined_frontier() {
    let doc = slab_with(
        20.0 * eps(),
        Distribution::Uniform {
            lo: -40.0 * eps(),
            hi: 40.0 * eps(),
        },
        depth_param(),
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let v = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 3,
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .unwrap();
    let unexamined: Vec<_> = v
        .refused()
        .iter()
        .filter(|l| {
            matches!(
                l.reason,
                RefusalReason::Budget(BudgetKind::Leaves { max_leaves: 3 })
            )
        })
        .collect();
    assert!(!unexamined.is_empty(), "the frontier must be priced");
    let priced: f64 = unexamined
        .iter()
        .map(|l| l.box_.mass(&analyzed).unwrap())
        .sum();
    assert!(
        priced > 0.0,
        "the unexamined frontier carries {priced} of the box's mass"
    );
    let column = v.accounting().refused[&ReasonClass::Budget]
        .clone()
        .unwrap();
    assert!(
        column >= priced - 1e-12,
        "the Budget column {column} is under the {priced} the unexamined frontier alone carries"
    );
    assert!(v.receipt().holds());
}

/// **`Bifurcation` and `Infeasible` are unreachable in v1**, and this
/// row is the grep made mechanical: no shipped source outside the two
/// variants' own definitions constructs either.
///
/// EVIDENCE-ONLY: a source-text scan, which is what "no machinery
/// invents a way to reach them" can be checked as.
#[test]
fn nothing_constructs_the_two_unreachable_refusals() {
    // THROUGH THE SHARED LEXER, not a hand-rolled one: comments and
    // string literals are blanked by `test_utils::source::code_only`,
    // so a doc comment naming `RefusalReason::Infeasible` — and this
    // module's own docs do — cannot be read as a construction. The
    // reader census's `Shared` disposition is the destination for every
    // site that reads Rust source, and a `//`-prefix filter is the
    // hand-rolled reader it exists to retire.
    let src = test_utils::source::code_only(include_str!("../src/drive.rs"));
    // A CONSTRUCTION, as opposed to a declaration or a match pattern:
    // the variant's name in VALUE position — pushed onto the refused
    // list, assigned to a `reason:` field, or returned as a verdict.
    for (i, line) in src.lines().enumerate() {
        let l = line.trim_start();
        for v in ["Infeasible", "Bifurcation"] {
            let mentions = l.contains(&format!("RefusalReason::{v}"));
            let constructs = mentions
                && (l.contains("reason:")
                    || l.contains("Refused(")
                    || l.contains("push(")
                    || l.contains("return "));
            assert!(
                !constructs,
                "drive.rs:{} constructs the v1-unreachable {v}: {line}",
                i + 1
            );
        }
    }
}
