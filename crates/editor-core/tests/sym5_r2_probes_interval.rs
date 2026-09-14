//! **R2's end-to-end exercise of SYM-5's rule E** (PR #2589, frozen head
//! `480704dbb`) on documents the unit did not measure: a frame tilted
//! about the OTHER axis, an in-plane rotation carrying the parameter in
//! both axes, two derived frames stacked, a `FaceFrame` on the cap of a
//! REVOLVED body, and authored axes that are genuinely NON-unit — each
//! driven through the public doors at `Sym<Interval>` with the dial on
//! and off, under both lifts. Plus the bulge-boss BEFORE table re-taken
//! under `without_rule_e`, and the tilted residual's chain rendered
//! with the rule off and on.
//!
//! The preamble is the `m10_*` family's copy
//! (`work/sym/interval-test-preamble-is-copied-across-the-m10-files`).
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture::{self, Recorder, ang, axis_in_plane, len, scl};
use crate::m10_8_harness::split_at_the_nominal;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{
    CancelToken, CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, EvalOptions,
    Evaluation, Expr, MeridianEnd, Node, NodeResult, ParamName, ProfileDoc, ProfileLift,
    RecipeNodeId, RoleSeg, UnitSym, evaluate,
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
) -> (Vec<String>, geom_core::SymCounts, f64) {
    let o = opts(doc, lift);
    let t = std::time::Instant::now();
    let (f, c) = geom_core::sym::with_session_rules(budget(), rules, || {
        let ev: Evaluation<geom_core::Sym<Interval>> =
            evaluate(doc, None, &CancelToken::new(), &o, Tol::witness());
        failures(&ev)
    });
    (f, c, t.elapsed().as_secs_f64())
}

fn head(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

fn tilt_param(r: &mut Recorder, half: f64) -> Expr {
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
    Expr::param(ParamName::new("t"), Dimension::Scalar)
}

#[derive(Clone, Copy, Debug)]
enum Base {
    /// The PR's: `u = (1,0,0)`, `v = (0,1,t)`.
    TiltV,
    /// About the other axis: `u = (1,0,t)`, `v = (0,1,0)`.
    TiltU,
    /// In-plane rotation, both axes carry `t`: `u = (1,t,0)`, `v = (−t,1,0)`.
    Spin,
    /// Genuinely NON-unit authored axes: `u = (2,0,0)`, `v = (0,2,t)`.
    NonUnit,
}

fn base_frame(r: &mut Recorder, t: &Expr, base: Base) -> RecipeNodeId {
    let (u, v) = match base {
        Base::TiltV => (
            [scl(1.0), scl(0.0), scl(0.0)],
            [scl(0.0), scl(1.0), t.clone()],
        ),
        Base::TiltU => (
            [scl(1.0), scl(0.0), t.clone()],
            [scl(0.0), scl(1.0), scl(0.0)],
        ),
        Base::Spin => (
            [scl(1.0), t.clone(), scl(0.0)],
            [Expr::neg(t.clone()), scl(1.0), scl(0.0)],
        ),
        Base::NonUnit => (
            [scl(2.0), scl(0.0), scl(0.0)],
            [scl(0.0), scl(2.0), t.clone()],
        ),
    };
    r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u,
        v,
    }))
}

/// `n` cubes stacked, each extruded from a `FaceFrame` on the last
/// one's cap; the frame the boss goes on.
fn stacked(r: &mut Recorder, base: RecipeNodeId, n: usize) -> RecipeNodeId {
    let mut on = base;
    for _ in 0..n {
        let p = r.insert(Node::Profile(fixture::desc(
            on,
            vec![fixture::square(0.0, 0.0, 1.0)],
        )));
        let cube = r.insert(Node::Extrude {
            profile: p,
            distance: len(1.0),
        });
        on = r.insert(Node::Datum(Datum::FaceFrame {
            at: cube,
            face: fixture::fname(cube, RoleSeg::Cap(CapEnd::End)),
            spin: ang(0.0),
        }));
    }
    on
}

/// A half-turn revolve of an off-axis square about the base frame's
/// `v` through its origin; the frame on its END cap.
fn revolved(r: &mut Recorder, base: RecipeNodeId) -> RecipeNodeId {
    let p = r.insert(Node::Profile(fixture::desc(
        base,
        vec![fixture::square(1.5, 0.0, 0.5)],
    )));
    let axis = r.insert(axis_in_plane(base, (0.0, 0.0), (0.0, 1.0)));
    let rev = r.insert(Node::Revolve {
        profile: p,
        axis,
        angle: ang(std::f64::consts::PI),
    });
    r.insert(Node::Datum(Datum::FaceFrame {
        at: rev,
        face: fixture::fname(rev, RoleSeg::RevolveCap(MeridianEnd::End)),
        spin: ang(0.0),
    }))
}

fn boss(r: &mut Recorder, on: RecipeNodeId) {
    let boss_p = r.insert(Node::Profile(fixture::desc(
        on,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.25),
    });
}

#[derive(Clone, Copy, Debug)]
enum Place {
    Authored,
    Derived(usize),
    Revolved,
}

fn document(half: f64, base: Base, place: Place) -> ProfileDoc {
    let mut r = Recorder::new();
    let t = tilt_param(&mut r, half);
    let b = base_frame(&mut r, &t, base);
    let on = match place {
        Place::Authored => b,
        Place::Derived(n) => stacked(&mut r, b, n),
        Place::Revolved => revolved(&mut r, b),
    };
    boss(&mut r, on);
    r.doc
}

/// **The e2e ladder**: every document, both lifts, plain Interval and
/// the tier with the dial off and on. Prints; asserts only that the
/// rule never REFUSES what the dial-off tier certifies (nothing lost).
#[test]
fn r2_e2e_derived_frames_the_unit_did_not_measure() {
    let half = 1.0e-3;
    let cases: Vec<(&str, Base, Place)> = vec![
        ("tiltU authored", Base::TiltU, Place::Authored),
        ("tiltU derived", Base::TiltU, Place::Derived(1)),
        ("spin authored", Base::Spin, Place::Authored),
        ("spin derived", Base::Spin, Place::Derived(1)),
        ("tiltV stacked-2", Base::TiltV, Place::Derived(2)),
        ("tiltV revolved-cap", Base::TiltV, Place::Revolved),
        ("non-unit authored", Base::NonUnit, Place::Authored),
        ("non-unit derived", Base::NonUnit, Place::Derived(1)),
    ];
    let only = std::env::var("CAD_R2_CASES").ok();
    let mut lost = Vec::new();
    for (name, base, place) in cases {
        if only
            .as_deref()
            .is_some_and(|l| !l.split(',').any(|n| n.trim() == name))
        {
            continue;
        }
        let doc = document(half, base, place);
        for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
            let p = plain(&doc, lift);
            let (off, c_off, t_off) = sym(&doc, lift, SymRules::without_rule_e());
            let (on, c_on, t_on) = sym(&doc, lift, SymRules::shipped());
            println!(
                "{name:<20} {lift:?}: plain {} | off {} frozen {} sym0 {} {t_off:.1}s | on {} frozen {} sym0 {} {t_on:.1}s",
                p.len(),
                off.len(),
                c_off.frozen,
                c_off.symbolic_zero,
                on.len(),
                c_on.frozen,
                c_on.symbolic_zero
            );
            for f in &p {
                println!("      plain: {}", head(f, 200));
            }
            for f in &off {
                println!("      off:   {}", head(f, 200));
            }
            for f in &on {
                println!("      on:    {}", head(f, 200));
            }
            if off.is_empty() && !on.is_empty() {
                lost.push(format!("{name} {lift:?}: {on:?}"));
            }
        }
    }
    assert!(lost.is_empty(), "rule E lost a certification: {lost:?}");
}

/// **The bulge boss's BEFORE table, re-taken under `without_rule_e`**:
/// the PR's "63/0/0/27 → 81/0/0/9" claims a before that no pinned row
/// holds any more; this row holds it.
#[test]
fn r2_the_bulge_boss_before_table_under_without_rule_e() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_10_r1_probes_interval::segment_boss(1.0, tol);
    let table = split_at_the_nominal(&doc, SymRules::without_rule_e(), tol);
    for (k, v) in &table {
        println!("   {k}: {v:?}");
    }
    // Only the rows the PR names are compared (`assert_split` wants the
    // whole table).
    for (pred, want) in [
        ("carrier_matches_mapped_source", [72, 0, 48, 6]),
        ("carrier_on_surface_2", [63, 0, 0, 27]),
        ("witness_on_surface_2", [7, 0, 0, 3]),
        ("carrier_on_surface_1", [81, 0, 0, 9]),
        ("witness_on_surface_1", [9, 0, 0, 1]),
    ] {
        assert_eq!(
            table.get(pred).copied(),
            Some(want),
            "{pred} with rule E off"
        );
    }
}

/// **The tilted residual's chain, rule E off and on** (claim 3): the
/// blocked shapes' early forms at `explain_depth 6`, and the decision
/// split, on the PR's own document.
#[test]
fn r2_the_tilted_residual_rendered_off_and_on() {
    use geom_core::sym::report::{
        ShapeOutcome, explain_depth, name_param, start_shape_report, take_shape_report,
    };
    let base = match std::env::var("CAD_R2_BASE").as_deref() {
        Ok("tiltU") => Base::TiltU,
        Ok("spin") => Base::Spin,
        Ok("nonunit") => Base::NonUnit,
        _ => Base::TiltV,
    };
    let half: f64 = std::env::var("CAD_R2_HALF")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1.0e-3);
    let doc = document(half, base, Place::Derived(1));
    println!("=== rendering {base:?} derived(1) Guided half={half:e}");
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = ParamBox::of(&analyzed);
    for name in box_.axes().keys() {
        name_param(&name.0);
    }
    for (label, rules) in [
        ("without_rule_e", SymRules::without_rule_e()),
        ("shipped", SymRules::shipped()),
    ] {
        let o = EvalOptions {
            param_box: Some(Arc::new(box_.clone())),
            profile_lift: ProfileLift::Guided,
            ..EvalOptions::default()
        };
        start_shape_report();
        explain_depth(6);
        let (refusal, counts) = geom_core::sym::with_session_rules(budget(), rules, || {
            let ev: Evaluation<geom_core::Sym<Interval>> =
                evaluate(&doc, None, &CancelToken::new(), &o, Tol::witness());
            failures(&ev)
        });
        let shapes = take_shape_report();
        explain_depth(0);
        let mut split: std::collections::BTreeMap<String, usize> = Default::default();
        for s in &shapes {
            *split
                .entry(format!("{} {:?}", s.predicate, s.outcome))
                .or_default() += 1;
        }
        println!("=== {base:?} derived Guided half={half:e} {label}: counts {counts:?}");
        println!(
            "refusals {:?}",
            refusal.iter().map(|r| head(r, 200)).collect::<Vec<_>>()
        );
        println!("decisions {split:?}");
        let mut seen: std::collections::BTreeSet<&str> = Default::default();
        for s in &shapes {
            if matches!(
                s.outcome,
                ShapeOutcome::Indeterminate | ShapeOutcome::Invalid | ShapeOutcome::NumericZero
            ) && seen.insert(s.predicate)
            {
                println!("--- blocked {} {:?}", s.predicate, s.outcome);
                println!("    enclosure {:?} sizes {:?}", s.enclosure, s.sizes);
                if let Some(f) = &s.early_form {
                    println!("    early  {}", head(f, 600));
                }
                if let Some(e) = &s.explain {
                    let n = e.lines().filter(|l| l.contains("FROZEN")).count();
                    println!("    explain: {} lines, {n} FROZEN", e.lines().count());
                    for l in e.lines().take(40) {
                        println!("      {}", head(l, 220));
                    }
                }
            }
        }
    }
}
