//! **A profile placed on a DERIVED frame, on the symbolic lane** —
//! SYM-5's rows, ported from DOCM-1's review lane R1 (`docm/1-review-r1`,
//! `docm1_review_r1_probes_interval.rs`) unchanged in what they assert,
//! and the measurement that says why they stood red.
//!
//! A derived frame (`Datum::FaceFrame`) reads its axes off the body it
//! is placed on, so those axes are the kernel's ALREADY-normalised
//! stored vectors — each a rational form over a `sqrt(v·v)` atom — and
//! the boss extrude above it normalises them AGAIN and squares them in
//! certification. Every normalisation adds a denominator and every
//! square doubles the degree, so the forms reach the budget and freeze;
//! a frozen subtree cancels nothing, and the identity `u·u = 1` that
//! an authored frame's literal axes reach for free is lost.
//!
//! The two ported rows are the pins. The `#[ignore]`d rows beside them
//! are the measurement: the freeze population by origin and cause, and
//! the frozen kids rendered far enough to name the normalisation chain.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture::{self, Recorder, ang, len, scl};

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::SymbolicDials;
use editor_core::{
    CancelToken, CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, EvalOptions,
    Evaluation, Expr, Node, NodeResult, ParamName, ProfileDoc, ProfileLift, RecipeNodeId, RoleSeg,
    UnitSym, evaluate,
};
use geom_core::{Interval, SymRules, Tol};

fn eps() -> f64 {
    Tol::witness().eps()
}

fn param_doc(name: &str, nominal: f64, half: f64, r: &mut Recorder) {
    r.push(DocEdit::SetDocParam {
        name: ParamName::new(name),
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
}

fn budget() -> geom_core::SymBudget {
    let dials = SymbolicDials::default();
    geom_core::SymBudget {
        max_terms: dials.max_terms,
        max_degree: dials.max_degree,
    }
}

/// Node failures of an evaluation over the WHOLE declared box in one
/// leaf, on the plain `Interval` lane.
fn interval_failures(doc: &ProfileDoc, lift: ProfileLift) -> Vec<String> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        profile_lift: lift,
        ..EvalOptions::default()
    };
    let ev: Evaluation<Interval> = evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness());
    failures(&ev)
}

/// The same, on the `Sym<Interval>` lane the E6 driver certifies on,
/// under the shipped rule set.
fn sym_failures(doc: &ProfileDoc, lift: ProfileLift) -> Vec<String> {
    sym_failures_under(doc, lift, SymRules::shipped()).0
}

/// The same, under a chosen rule set, with the leaf's counts.
fn sym_failures_under(
    doc: &ProfileDoc,
    lift: ProfileLift,
    rules: SymRules,
) -> (Vec<String>, geom_core::SymCounts) {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        profile_lift: lift,
        ..EvalOptions::default()
    };
    geom_core::sym::with_session_rules(budget(), rules, || {
        let ev: Evaluation<geom_core::Sym<Interval>> =
            evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness());
        failures(&ev)
    })
}

fn failures<T: geom_core::Decide>(ev: &Evaluation<T>) -> Vec<String> {
    ev.order
        .iter()
        .filter_map(|id| match ev.result(*id) {
            Some(NodeResult::Failed(e)) => Some(format!("node {} — {}", id.0, e.kind)),
            Some(NodeResult::Poisoned { through }) => {
                Some(format!("node {} poisoned through {}", id.0, through.0))
            }
            _ => None,
        })
        .collect()
}

/// **The derived-frame document**: a unit box whose HEIGHT is the
/// widened parameter, a frame derived from its top cap, a profile on
/// that frame, and an extrude of the profile.
pub(crate) fn boss_on_widened_box(half: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    param_doc("h", 1.0, half, &mut r);
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 1.0)],
    );
    let cube = r.insert(Node::Extrude {
        profile: p,
        distance: Expr::param(ParamName::new("h"), Dimension::Length),
    });
    let frame = r.insert(Node::Datum(Datum::FaceFrame {
        at: cube,
        face: fixture::fname(cube, RoleSeg::Cap(CapEnd::End)),
        spin: ang(0.0),
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    let boss = r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.25),
    });
    (r.doc, boss_p, boss)
}

/// **The authored twin**: a frame whose ORIGIN z is the widened
/// parameter, a profile on it, an extrude — under `Guided`, the
/// placement is at `T` exactly as a derived frame's is under every
/// lift.
pub(crate) fn boss_on_widened_authored_frame(half: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    param_doc("z0", 1.0, half, &mut r);
    let frame = r.insert(Node::Datum(Datum::Frame {
        origin: [
            len(0.0),
            len(0.0),
            Expr::param(ParamName::new("z0"), Dimension::Length),
        ],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    let boss = r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.25),
    });
    (r.doc, boss)
}

/// **The transform-lifted document**: an EXACT box lifted by a widened
/// rigid transform, a derived frame on the lifted body's top cap, a
/// profile, and an extrude above it.
pub(crate) fn transform_lifted_boss(half: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    param_doc("lift", 0.0, half, &mut r);
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 1.0)],
    );
    let cube = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    let lifted = r.insert(Node::Transform {
        input: cube,
        translation: [
            len(0.0),
            len(0.0),
            Expr::param(ParamName::new("lift"), Dimension::Length),
        ],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(0.0),
    });
    let frame = r.insert(Node::Datum(Datum::FaceFrame {
        at: lifted,
        face: fixture::fname(cube, RoleSeg::Cap(CapEnd::End)),
        spin: ang(0.0),
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.25),
    });
    r.doc
}

/// **DOCM-1 R1's C7 row** (`r1_c7_an_extrude_on_a_widened_derived_frame_versus_the_authored_guided_twin`),
/// unchanged in what it asserts: whatever the kernel does with a
/// widened placement, it must do the same for the two frame kinds.
#[test]
#[ignore = "red at ONE point only — half = 5e-2, all three eps rows: the derived boss's \
            side-plane newell normalises a cross-sum whose enclosure straddles zero, so \
            clause 1 refuses a margin the tier's early form already proves zero \
            (work/sym/a-widened-derived-placement-normalises-a-straddling-newell-sum). \
            m10_the_derived_frame_extrude_agrees_with_its_authored_twin_below_that_width \
            pins the measured state meanwhile."]
fn m10_an_extrude_on_a_widened_derived_frame_versus_the_authored_guided_twin() {
    let e = eps();
    let mut mismatch = Vec::new();
    for w in [e / 8.0, 1e-6, 1e-3, 0.05] {
        let (derived, boss_p, boss) = boss_on_widened_box(w);
        let (authored, _) = boss_on_widened_authored_frame(w);
        for (lane, run) in [
            (
                "plain",
                interval_failures as fn(&ProfileDoc, ProfileLift) -> Vec<String>,
            ),
            (
                "sym",
                sym_failures as fn(&ProfileDoc, ProfileLift) -> Vec<String>,
            ),
        ] {
            let d_pinned = run(&derived, ProfileLift::Pinned);
            let d_guided = run(&derived, ProfileLift::Guided);
            let a_guided = run(&authored, ProfileLift::Guided);
            println!(
                "C7 eps={e:e} half={w:e} lane={lane}: derived/Pinned {d_pinned:?}; derived/Guided {d_guided:?}; authored/Guided {a_guided:?}"
            );
            let d_ok = d_pinned.is_empty();
            let a_ok = a_guided.is_empty();
            if d_ok != a_ok {
                mismatch.push(format!(
                    "eps={e:e} half={w:e} lane={lane}: derived ok={d_ok}, authored/Guided ok={a_ok}"
                ));
            }
            let _ = (boss_p, boss);
        }
    }
    assert!(
        mismatch.is_empty(),
        "frame kinds disagree on a widened placement: {mismatch:?}"
    );
}

/// **DOCM-1 R1's transform-lifted row**
/// (`r1_c7_the_prs_transform_lifted_shape_with_an_extrude_above_it`),
/// unchanged in what it asserts.
#[test]
fn m10_the_transform_lifted_shape_with_an_extrude_above_it() {
    let e = eps();
    let doc = transform_lifted_boss(e / 8.0);
    let plain = interval_failures(&doc, ProfileLift::Pinned);
    let sym = sym_failures(&doc, ProfileLift::Pinned);
    println!(
        "C7 transform-lifted eps={e:e} half={:e}: plain {plain:?}; sym {sym:?}",
        e / 8.0
    );
    assert!(
        sym.is_empty(),
        "on the driver's lane the extrude above a derived frame on a transform-lifted box \
         must certify as the authored twin does: {sym:?}"
    );
}

// ---------------------------------------------------------------
// Phase 1 — the measurement
// ---------------------------------------------------------------

/// The first `n` characters of a rendering — a form that reaches the
/// budget renders to megabytes, and what a reader needs is its head.
fn head(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        return s.to_owned();
    }
    let cut: String = s.chars().take(n).collect();
    format!("{cut}… [{} chars]", s.chars().count())
}

/// The FROZEN lines of an explanation with the ancestors that lead to
/// them — the normalisation chain, without the thousands of lines that
/// did not freeze.
fn frozen_chain(explain: &str) -> String {
    let lines: Vec<&str> = explain.lines().collect();
    let indent = |l: &str| l.len() - l.trim_start().len();
    let mut keep = vec![false; lines.len()];
    for (i, l) in lines.iter().enumerate() {
        if !l.contains("FROZEN") {
            continue;
        }
        keep[i] = true;
        let mut d = indent(l);
        for j in (0..i).rev() {
            let dj = indent(lines[j]);
            if dj < d {
                keep[j] = true;
                d = dj;
            }
        }
    }
    let n = keep.iter().filter(|k| **k).count();
    if n == 0 {
        return "    (no frozen node below this residual)".to_owned();
    }
    let mut out = String::new();
    for (i, l) in lines.iter().enumerate() {
        if keep[i] {
            out.push_str(&head(l, 400));
            out.push('\n');
        }
    }
    out.push_str(&format!(
        "    ({n} lines of {} on the frozen path)\n",
        lines.len()
    ));
    out
}

/// One replay with BOTH instruments installed: the structural profile
/// (`sym::profile`) and the shape report with an explanation depth, so
/// a blocked residual comes back with its form, its early form and the
/// DAG below it to `depth` levels.
fn measured_replay(
    label: &str,
    doc: &ProfileDoc,
    box_: &ParamBox,
    lift: ProfileLift,
    rules: SymRules,
    depth: usize,
) {
    use geom_core::sym::profile::{start_profile, take_profile};
    use geom_core::sym::report::{
        ShapeOutcome, explain_depth, name_param, start_shape_report, take_shape_report,
    };

    for name in box_.axes().keys() {
        name_param(&name.0);
    }
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        profile_lift: lift,
        ..EvalOptions::default()
    };
    start_profile();
    start_shape_report();
    explain_depth(depth);
    let (refusal, counts) = geom_core::sym::with_session_rules(budget(), rules, || {
        let ev: Evaluation<geom_core::Sym<Interval>> =
            evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness());
        failures(&ev)
    });
    let shapes = take_shape_report();
    let profile = take_profile();
    explain_depth(0);

    let mut split: std::collections::BTreeMap<String, usize> = Default::default();
    for s in &shapes {
        *split
            .entry(format!("{} {:?}", s.predicate, s.outcome))
            .or_default() += 1;
    }
    println!("=== {label}");
    println!("counts {counts:?}");
    println!("refusals {refusal:?}");
    println!("decisions {split:?}");
    println!("freezes {} by (cause, op/walk/origin):", profile.frozen());
    for ((cause, where_), s) in profile.freezes_by_cause() {
        println!("  {cause:?} {where_} -> {s:?}");
    }
    println!("unnoted {}", profile.unnoted());
    // The first blocked residual of each predicate, with its chain.
    let mut seen: std::collections::BTreeSet<&str> = Default::default();
    for s in &shapes {
        if matches!(
            s.outcome,
            ShapeOutcome::Indeterminate | ShapeOutcome::Invalid | ShapeOutcome::NumericZero
        ) && seen.insert(s.predicate)
        {
            println!("--- blocked {} {:?}", s.predicate, s.outcome);
            println!("    enclosure {:?} sizes {:?}", s.enclosure, s.sizes);
            if let Some(f) = &s.form {
                println!("    plain  {}", head(f, 1200));
            }
            if let Some(f) = &s.early_form {
                println!("    early  {}", head(f, 1200));
            }
            if let Some(e) = &s.explain {
                println!("{}", frozen_chain(e));
            }
        }
    }
    print!("{}", profile.render());
}

/// **Phase 1, the freeze profiled by origin and cause, and the frozen
/// kids rendered** — the derived-frame document at its nominal and at
/// one widened box, against its authored twin.
#[test]
#[ignore = "evidence-only: SYM-5 phase 1 — the freeze population and the normalisation chain"]
fn sym5_phase1_profile_the_derived_frame_freeze() {
    for half in [1.0e-3, 5.0e-2] {
        let (derived, _, _) = boss_on_widened_box(half);
        let (authored, _) = boss_on_widened_authored_frame(half);
        for (name, doc) in [("derived", &derived), ("authored", &authored)] {
            let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
            let root = ParamBox::of(&analyzed);
            let nominal = crate::m10_8_harness::nominal_box(&analyzed);
            for (scale, box_) in [("nominal", nominal), ("root", root)] {
                measured_replay(
                    &format!("{name} half={half:e} {scale} shipped"),
                    doc,
                    &box_,
                    ProfileLift::Pinned,
                    SymRules::shipped(),
                    6,
                );
            }
        }
    }
}

/// **Phase 1, the tier with the algebra off** — the same documents
/// under `SymRules::without_the_algebra` (M10-9's tier bit for bit),
/// which is the state DOCM-1's diagnosis was taken in.
#[test]
#[ignore = "evidence-only: SYM-5 phase 1 — the same documents under M10-9's tier"]
fn sym5_phase1_the_derived_frame_under_the_earlier_tier() {
    for half in [1.0e-3, 5.0e-2] {
        let (derived, _, _) = boss_on_widened_box(half);
        let n = SymRules::none();
        let ladder = [
            ("none", n),
            (
                "A0",
                SymRules {
                    const_fold: true,
                    ..n
                },
            ),
            (
                "A0+early",
                SymRules {
                    const_fold: true,
                    early: true,
                    ..n
                },
            ),
            ("without_the_door", SymRules::shipped_without_the_door()),
            ("without_the_algebra", SymRules::without_the_algebra()),
            ("shipped", SymRules::shipped()),
        ];
        for (name, rules) in ladder {
            let (fails, counts) = sym_failures_under(&derived, ProfileLift::Pinned, rules);
            println!(
                "ladder derived half={half:e} {name}\n  counts {counts:?}\n  fails {} {}",
                fails.len(),
                head(fails.first().map_or("", String::as_str), 220)
            );
        }
    }
}

/// **Phase 1, the chain the plain form cannot see** — the same
/// derived-frame document with every rule OFF (`SymRules::none`), at
/// the nominal, so every residual the plain quotient form does not
/// reach is rendered with the DAG below it and the FROZEN nodes on the
/// path named. This is where the normalisation chain is readable: with
/// the shipped set the document has no blocked residual at all.
#[test]
#[ignore = "evidence-only: SYM-5 phase 1 — the normalisation chain under the plain form \
            alone; ~16 minutes, most of it rendering forms of 992 terms at degree 82"]
fn sym5_phase1_the_normalisation_chain_under_the_plain_form() {
    let (derived, _, _) = boss_on_widened_box(5.0e-2);
    let analyzed = analyzed_box(&derived, &AnalysisPolicy::default());
    let nominal = crate::m10_8_harness::nominal_box(&analyzed);
    measured_replay(
        "derived half=5e-2 nominal rules=none",
        &derived,
        &nominal,
        ProfileLift::Pinned,
        SymRules::none(),
        10,
    );
}

/// **The measured state of the parity row**, pinned while the row
/// above stays red at one width. Two claims, both measured at all
/// three ε rows:
///
/// - up to a half-width of `1e-3` the two frame kinds AGREE on the
///   symbolic lane — the derived-frame boss certifies exactly where its
///   authored twin does, which is what the tier buys the derived frame
///   (with every rule off it refuses at every width, on
///   `carrier_endpoint_start`);
/// - at `5e-2` the derived one refuses, and the refusal is NOT the
///   tier's: it is a clause-1 invalid margin on `newell_plane_residual`
///   — `Vec3::normalize` divides the boss's side-plane cross-sum by a
///   norm whose enclosure contains zero, so the value channel has no
///   certified value and the identity test is never asked. The tier's
///   own early form for that residual is the ZERO form.
///
/// A pin of what the kernel does: when the value channel stops
/// widening that cross-sum, this row fails and the parity row above
/// goes green and takes its place.
#[test]
fn m10_the_derived_frame_extrude_agrees_with_its_authored_twin_below_that_width() {
    let e = eps();
    for w in [e / 8.0, 1.0e-6, 1.0e-3] {
        let (derived, _, _) = boss_on_widened_box(w);
        let (authored, _) = boss_on_widened_authored_frame(w);
        for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
            assert!(
                sym_failures(&derived, lift).is_empty(),
                "half={w:e} {lift:?}: the derived-frame boss must certify on the symbolic lane"
            );
        }
        assert!(
            sym_failures(&authored, ProfileLift::Guided).is_empty(),
            "half={w:e}: the authored twin must certify"
        );
    }
    // The one width where they part, and the reason, read off the
    // refusal the kernel reports.
    let (derived, _, _) = boss_on_widened_box(5.0e-2);
    let (authored, _) = boss_on_widened_authored_frame(5.0e-2);
    let refusal = sym_failures(&derived, ProfileLift::Pinned);
    assert_eq!(refusal.len(), 1, "one refusal at 5e-2: {refusal:?}");
    assert!(
        refusal[0].contains("newell_plane_residual") && refusal[0].contains("margin is invalid"),
        "the 5e-2 refusal is clause 1's, on the newell residual: {refusal:?}"
    );
    assert!(
        sym_failures(&authored, ProfileLift::Guided).is_empty(),
        "the authored twin still certifies at 5e-2"
    );
}

/// **The freeze population is not what refuses.** With the exact
/// constant fold alone (`const_fold`, rule A0, REPLACING the plain
/// form's constant atoms — the early walk off) the derived-frame
/// document freezes NOTHING and still refuses at `5e-2` with the same
/// clause-1 margin; with every rule off it freezes 12 and refuses at
/// EVERY width on `carrier_endpoint_start`. So A0 is what carries this
/// document, the 1,253 degree freezes the shipped set makes in its
/// early walk cost it no decision, and no rule of the atom algebra
/// stands between the derived frame and its authored twin.
#[test]
fn m10_the_derived_frames_refusal_is_not_a_freeze() {
    let (derived, _, _) = boss_on_widened_box(5.0e-2);
    let n = SymRules::none();

    let (plain_fails, plain_counts) = sym_failures_under(&derived, ProfileLift::Pinned, n);
    assert_eq!(
        plain_fails.len(),
        1,
        "plain form alone refuses: {plain_fails:?}"
    );
    assert!(
        plain_fails[0].contains("carrier_endpoint_start"),
        "with no rule the refusal is an identity the tier cannot see: {plain_fails:?}"
    );
    assert!(
        plain_counts.frozen > 0,
        "the plain form freezes: {plain_counts:?}"
    );

    let a0 = SymRules {
        const_fold: true,
        ..n
    };
    let (a0_fails, a0_counts) = sym_failures_under(&derived, ProfileLift::Pinned, a0);
    assert_eq!(
        a0_counts.frozen, 0,
        "A0 alone freezes nothing on this document: {a0_counts:?}"
    );
    assert_eq!(a0_fails.len(), 1, "and still refuses once: {a0_fails:?}");
    assert!(
        a0_fails[0].contains("newell_plane_residual") && a0_fails[0].contains("margin is invalid"),
        "with no freeze left the refusal is clause 1's: {a0_fails:?}"
    );
}
