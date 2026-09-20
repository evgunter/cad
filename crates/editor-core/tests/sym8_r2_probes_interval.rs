//! **SYM-8 review R2 — end-to-end probes at `Sym<Interval>`** for rule F
//! (the manifest sign), on documents the unit did not build, plus the
//! pad's four re-taken.
//!
//! Every row is `#[ignore]`d evidence: it prints what happened and
//! asserts the one thing that must hold on every document (rule F never
//! REFUSES what the dial-off tier certifies), and, for the pad, the
//! counts the PR reports.
//!
//! The three documents, each a cube extruded from an authored
//! `Datum::Frame` carrying the parameter `t`, a `FaceFrame` on one of
//! its caps and a boss on that frame — the tilt-`u` construction of
//! `m10_derived_frame_tilted_interval` varied along the axis rule F
//! folds on, the cap normal's `z`:
//!
//! - `tilt-uv`: `u = (1,0,t)`, `v = (0,1,t)` — the normal is
//!   `(−t, −t, 1)/sqrt(1 + 2t²)`, so `n.z = 1/sqrt(1 + 2t²)`, an `Inv`
//!   of a `sqrt` atom the predicate calls positive (both arms fold);
//! - `start-cap`: the tilt-`u` cube with the `FaceFrame` on its START
//!   cap, whose normal is the negation — `n.z = −1/sqrt(P(t))`, a
//!   negative coefficient the predicate refuses (neither arm folds);
//! - `z-touches-zero`: `u = (1,0,0)`, `v = (0,t,1)` — the normal is
//!   `(0, −1, t)/sqrt(1 + t²)`, so `n.z = t/sqrt(1 + t²)`, a parameter at
//!   an odd power (declined), and at `half = 0.3` the box holds `t = 0`,
//!   where `n.z`'s enclosure touches zero and Duff's basis flips.
//!
//! The preamble is copied from `m10_derived_frame_tilted_interval`
//! (the class `work/sym/interval-test-preamble-is-copied-across-the-m10-files`).
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture::{self, Recorder, ang, len, scl};

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{
    CancelToken, CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, EvalOptions,
    Evaluation, Expr, Node, NodeResult, ParamName, ProfileDoc, ProfileLift, RecipeNodeId, RoleSeg,
    UnitSym, evaluate,
};
use geom_core::{Interval, SymBudget, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    }
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

fn opts(doc: &ProfileDoc, lift: ProfileLift) -> EvalOptions {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        profile_lift: lift,
        ..EvalOptions::default()
    }
}

fn plain(doc: &ProfileDoc, lift: ProfileLift) -> Vec<String> {
    let ev: Evaluation<Interval> = evaluate(
        doc,
        None,
        &CancelToken::new(),
        &opts(doc, lift),
        Tol::witness(),
    );
    failures(&ev)
}

fn sym(
    doc: &ProfileDoc,
    lift: ProfileLift,
    rules: SymRules,
) -> (Vec<String>, geom_core::SymCounts) {
    let o = opts(doc, lift);
    geom_core::sym::with_session_rules(budget(), rules, || {
        let ev: Evaluation<geom_core::Sym<Interval>> =
            evaluate(doc, None, &CancelToken::new(), &o, Tol::witness());
        failures(&ev)
    })
}

fn head(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

#[derive(Clone, Copy, Debug)]
enum Doc {
    TiltUv,
    StartCap,
    ZTouchesZero,
}

fn document(half: f64, which: Doc) -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("t"),
        value: DocParam::Continuous {
            dim: Dimension::Scalar,
            value: 0.25,
            display_unit: UnitSym::canonical_for(Dimension::Scalar),
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        },
    });
    let t = Expr::param(ParamName::new("t"), Dimension::Scalar);
    let (u, v, cap) = match which {
        Doc::TiltUv => (
            [scl(1.0), scl(0.0), t.clone()],
            [scl(0.0), scl(1.0), t.clone()],
            CapEnd::End,
        ),
        Doc::StartCap => (
            [scl(1.0), scl(0.0), t.clone()],
            [scl(0.0), scl(1.0), scl(0.0)],
            CapEnd::Start,
        ),
        Doc::ZTouchesZero => (
            [scl(1.0), scl(0.0), scl(0.0)],
            [scl(0.0), t.clone(), scl(1.0)],
            CapEnd::End,
        ),
    };
    let base: RecipeNodeId = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u,
        v,
    }));
    let p = r.insert(Node::Profile(fixture::desc(
        base,
        vec![fixture::square(0.0, 0.0, 1.0)],
    )));
    let cube = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    let on = r.insert(Node::Datum(Datum::FaceFrame {
        at: cube,
        face: fixture::fname(cube, RoleSeg::Cap(cap)),
        spin: ang(0.0),
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        on,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.25),
    });
    r.doc
}

/// **The three documents through the public doors**, at `half` in
/// `{1e-3, 5e-2}` (and `0.3` for the touching-zero document), both
/// lifts, the plain lane and the tier with rule F off and on.
/// `CAD_SYM8_R2_DOC` names one of `tilt-uv | start-cap | z-touches-zero`.
#[test]
#[ignore = "evidence-only: R2's e2e ladder for rule F on documents the unit did not build"]
fn sym8_r2_the_reach_on_three_documents_the_unit_did_not_build() {
    let only = std::env::var("CAD_SYM8_R2_DOC").ok();
    let cases: [(&str, Doc, &[f64]); 3] = [
        ("tilt-uv", Doc::TiltUv, &[1.0e-3, 5.0e-2]),
        ("start-cap", Doc::StartCap, &[1.0e-3, 5.0e-2]),
        ("z-touches-zero", Doc::ZTouchesZero, &[1.0e-3, 3.0e-1]),
    ];
    let mut lost: Vec<String> = Vec::new();
    for (name, which, halves) in cases {
        if only.as_deref().is_some_and(|o| o.trim() != name) {
            continue;
        }
        for &half in halves {
            let doc = document(half, which);
            for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
                let p = plain(&doc, lift);
                println!(
                    "{name} half={half:e} {lift:?} plain: {} {}",
                    p.len(),
                    head(p.first().map_or("", String::as_str), 200)
                );
                let mut off_ok = false;
                for (label, rules) in [
                    ("F-off", SymRules::without_rule_f()),
                    ("F-on ", SymRules::shipped()),
                ] {
                    let t0 = std::time::Instant::now();
                    let (f, c) = sym(&doc, lift, rules);
                    println!(
                        "{name} half={half:e} {lift:?} {label}: frozen {} sym0 {} gated {} reg {} num {} in {:.1}s\n  fails {} {}",
                        c.frozen,
                        c.symbolic_zero,
                        c.sign_gated,
                        c.registered,
                        c.numeric,
                        t0.elapsed().as_secs_f64(),
                        f.len(),
                        head(f.first().map_or("", String::as_str), 220)
                    );
                    if label.trim() == "F-off" {
                        off_ok = f.is_empty();
                    } else if off_ok && !f.is_empty() {
                        lost.push(format!("{name} half={half:e} {lift:?}: {f:?}"));
                    }
                }
            }
        }
    }
    assert!(
        lost.is_empty(),
        "rule F refused what the dial-off tier certified: {lost:?}"
    );
}

/// **The pad's four, re-taken**: R2's rounded pad at the scale the pins
/// say it certifies whole at (`m10_9_pins_interval::measured_studies`,
/// `2.083e3 · ε`), replayed over its analyzed box under `Guided`, rule
/// F off and on. The PR reports `858 / 104 / 991` → `854 / 128 / 971`
/// with `frozen` 2750 either way; this row prints both and asserts them.
#[test]
#[ignore = "evidence-only: the pad's four symbolic_zero -> registered, re-taken (one heavy row)"]
fn sym8_r2_the_pads_four_re_taken() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let doc = crate::m10_8_r2_probes_interval::pad(2.083e3 * eps, tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = ParamBox::of(&analyzed);
    let o = EvalOptions {
        param_box: Some(Arc::new(box_)),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    let mut got = Vec::new();
    for (label, rules) in [
        ("F-off", SymRules::without_rule_f()),
        ("F-on ", SymRules::shipped()),
    ] {
        let t0 = std::time::Instant::now();
        let (f, c) = geom_core::sym::with_session_rules(budget(), rules, || {
            let ev: Evaluation<geom_core::Sym<Interval>> =
                evaluate(&doc, None, &CancelToken::new(), &o, tol);
            failures(&ev)
        });
        println!(
            "pad x2.083e3·eps Guided {label}: {c:?} in {:.1}s; fails {}",
            t0.elapsed().as_secs_f64(),
            f.len()
        );
        assert!(
            f.is_empty(),
            "{label}: the pad certifies whole at this scale: {f:?}"
        );
        got.push((c.symbolic_zero, c.registered, c.numeric, c.frozen));
    }
    assert_eq!(
        got[0],
        (858, 104, 991, 2750),
        "rule F off: the PR's numbers"
    );
    assert_eq!(got[1], (854, 128, 971, 2750), "rule F on: the PR's numbers");
}
