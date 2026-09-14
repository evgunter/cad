//! **A derived frame whose AXES carry the parameter** — SYM-5's
//! second document class, and the one the item
//! `derived-frame-placement-freezes-on-the-symbolic-lane` was filed
//! on. DOCM's own document (`m10_derived_frame_interval`) is a pure
//! z-TRANSLATION: every normalised quantity in it is constant in the
//! parameter, so rule A0 folds the whole normalisation chain and no
//! degree mechanism is left to measure. Tilt the frame so its AXES
//! carry the parameter and the `sqrt(S)` over the stored unit vector
//! is a sqrt of a NON-constant form that no constant fold reaches, the
//! re-normalised forms freeze on DEGREE, and the identity is not
//! reached. (Widening the PROFILE instead of the height is NOT a third
//! case: A0 clears it — the width row below.)
//!
//! Every row here is EVIDENCE-ONLY and `#[ignore]`d: each prints a
//! ladder, a profile or a budget sweep and asserts nothing about the
//! numbers. The readings they take are the record; re-take them by
//! re-running the row. Phase 2 of SYM-5 runs on these documents (PR-2)
//! and is where a pin over them belongs.
//!
//! **Adopted from SYM-5's review lane**, branch `sym/5-review` at
//! `a3e2b1b46` (`crates/editor-core/tests/sym5_review_probes_interval.rs`),
//! which built the tilted and width-parameter documents and found the
//! item's mechanism alive on them. The two height-document rows of
//! that file live beside the height document, in
//! `m10_derived_frame_interval`.
//!
//! The preamble this file shares with its siblings — `param_doc`,
//! `failures`, the `budget()` constants — is copied across the `m10_*`
//! family; the class is
//! `work/sym/interval-test-preamble-is-copied-across-the-m10-files`.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture::{self, Recorder, ang, len, scl};

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{
    CancelToken, CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, EvalOptions,
    Evaluation, Expr, LoopProgram, Node, NodeResult, ParamName, ProfileDoc, ProfileLift,
    ProfileProgram, RoleSeg, UnitSym, evaluate,
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
    budget: SymBudget,
) -> (Vec<String>, geom_core::SymCounts) {
    let o = opts(doc, lift);
    geom_core::sym::with_session_rules(budget, rules, || {
        let ev: Evaluation<geom_core::Sym<Interval>> =
            evaluate(doc, None, &CancelToken::new(), &o, Tol::witness());
        failures(&ev)
    })
}

fn head(s: &str, n: usize) -> String {
    let cut: String = s.chars().take(n).collect();
    cut
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

/// A box whose WIDTH (not height) is the widened parameter. Measured
/// NOT to be a third case: A0 clears it (frozen 0, no refusal on either
/// lane at either width) — its `u`, `v` come out exact.
fn boss_on_widened_width_box(half: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    param_doc("w", 0.5, half, &mut r);
    let plane = r.insert(fixture::frame(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
    ));
    let w = Expr::param(ParamName::new("w"), Dimension::Length);
    let neg_w = Expr::neg(w.clone());
    let p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::polygon_expr([
            [neg_w.clone(), len(-0.5)],
            [w.clone(), len(-0.5)],
            [w, len(0.5)],
            [neg_w, len(0.5)],
        ])],
    }));
    let cube = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    let frame = r.insert(Node::Datum(Datum::FaceFrame {
        at: cube,
        face: fixture::fname(cube, RoleSeg::Cap(CapEnd::End)),
        spin: ang(0.0),
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(0.0, 0.0, 0.25)],
    )));
    r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.25),
    });
    r.doc
}

fn ladder() -> Vec<(&'static str, SymRules)> {
    let n = SymRules::none();
    vec![
        ("none", n),
        (
            "A0",
            SymRules {
                const_fold: true,
                ..n
            },
        ),
        (
            "A_only",
            SymRules {
                sqrt_square: true,
                ..n
            },
        ),
        ("shipped", SymRules::shipped()),
    ]
}
#[test]
#[ignore = "evidence-only: adopted SYM-5 review probe; Phase 2 (PR-2) is where these become pins"]
fn sym5_tilted_width_parameter_ladder() {
    for half in [1.0e-3, 5.0e-2] {
        let doc = boss_on_widened_width_box(half);
        let p = plain(&doc, ProfileLift::Pinned);
        println!(
            "width half={half:e} plain: {} {}",
            p.len(),
            head(p.first().map_or("", String::as_str), 300)
        );
        for (name, rules) in ladder() {
            let (f, c) = sym(&doc, ProfileLift::Pinned, rules, budget());
            println!(
                "width half={half:e} {name}: {c:?}\n  fails {} {}",
                f.len(),
                head(f.first().map_or("", String::as_str), 300)
            );
        }
    }
}
/// A frame whose `v` axis carries a SCALAR parameter `t` (`v = (0, 1, t)`),
/// so the frame normal is `(0, −t, 1)/sqrt(1 + t²)` — a re-normalised
/// unit vector over a NON-constant `sqrt` form that no constant fold can
/// reach. `derived`: a cube extruded from that frame, a `FaceFrame` on
/// its cap, the boss on it (the stored unit vectors re-normalised).
/// Otherwise the boss sits directly on the tilted authored frame.
fn boss_on_tilted(half: f64, derived: bool) -> ProfileDoc {
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
    let base = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), t],
    }));
    let on = if derived {
        let p = r.insert(Node::Profile(fixture::desc(
            base,
            vec![fixture::square(0.0, 0.0, 1.0)],
        )));
        let cube = r.insert(Node::Extrude {
            profile: p,
            distance: len(1.0),
        });
        r.insert(Node::Datum(Datum::FaceFrame {
            at: cube,
            face: fixture::fname(cube, RoleSeg::Cap(CapEnd::End)),
            spin: ang(0.0),
        }))
    } else {
        base
    };
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

#[test]
#[ignore = "evidence-only: adopted SYM-5 review probe; Phase 2 (PR-2) is where these become pins"]
fn sym5_tilted_frame_ladder() {
    for half in [1.0e-3, 5.0e-2] {
        for (kind, derived) in [("authored", false), ("derived", true)] {
            let doc = boss_on_tilted(half, derived);
            for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
                let p = plain(&doc, lift);
                println!(
                    "tilted {kind} half={half:e} {lift:?} plain: {} {}",
                    p.len(),
                    head(p.first().map_or("", String::as_str), 260)
                );
                for (name, rules) in ladder() {
                    let (f, c) = sym(&doc, lift, rules, budget());
                    println!(
                        "tilted {kind} half={half:e} {lift:?} {name}: frozen {} sym0 {} num {}\n  fails {} {}",
                        c.frozen,
                        c.symbolic_zero,
                        c.numeric,
                        f.len(),
                        head(f.first().map_or("", String::as_str), 260)
                    );
                }
            }
        }
    }
}

/// The tilted derived/Guided refusal, profiled: freezes by cause and
/// the first blocked residual of each predicate with its early form
/// and the FROZEN nodes on its path (SYM-1's instrument, as
/// `m10_derived_frame_interval::measured_replay` uses it).
#[test]
#[ignore = "evidence-only: adopted SYM-5 review probe; Phase 2 (PR-2) is where these become pins"]
fn sym5_tilted_derived_guided_profiled() {
    use geom_core::sym::profile::{start_profile, take_profile};
    use geom_core::sym::report::{
        ShapeOutcome, explain_depth, name_param, start_shape_report, take_shape_report,
    };
    let doc = boss_on_tilted(1.0e-3, true);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = ParamBox::of(&analyzed);
    for name in box_.axes().keys() {
        name_param(&name.0);
    }
    for (label, rules) in [
        ("shipped", SymRules::shipped()),
        (
            "A0",
            SymRules {
                const_fold: true,
                ..SymRules::none()
            },
        ),
    ] {
        let o = EvalOptions {
            param_box: Some(Arc::new(box_.clone())),
            profile_lift: ProfileLift::Guided,
            ..EvalOptions::default()
        };
        start_profile();
        start_shape_report();
        explain_depth(6);
        let (refusal, counts) = geom_core::sym::with_session_rules(budget(), rules, || {
            let ev: Evaluation<geom_core::Sym<Interval>> =
                evaluate(&doc, None, &CancelToken::new(), &o, Tol::witness());
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
        println!("=== tilted derived Guided 1e-3 {label}");
        println!("counts {counts:?}");
        println!(
            "refusals {:?}",
            refusal.iter().map(|r| head(r, 200)).collect::<Vec<_>>()
        );
        println!("decisions {split:?}");
        println!("freezes {} by (cause, op/walk/origin):", profile.frozen());
        for ((cause, where_), s) in profile.freezes_by_cause() {
            println!("  {cause:?} {where_} -> {s:?}");
        }
        print!("{}", profile.render());
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
                    println!("    plain  {}", head(f, 500));
                }
                if let Some(f) = &s.early_form {
                    println!("    early  {}", head(f, 500));
                }
                if let Some(e) = &s.explain {
                    let n = e.lines().filter(|l| l.contains("FROZEN")).count();
                    println!("    explain: {} lines, {n} FROZEN", e.lines().count());
                    for l in e.lines() {
                        println!("      {}", head(l, 300));
                    }
                }
            }
        }
    }
}

/// The tilted derived/Guided refusal under the shipped set with the
/// budget raised: if degree is what stands between the derived frame
/// and the theorem, a larger budget certifies it.
#[test]
#[ignore = "evidence-only: adopted SYM-5 review probe; Phase 2 (PR-2) is where these become pins"]
fn sym5_tilted_derived_guided_budget_ladder() {
    let doc = boss_on_tilted(1.0e-3, true);
    for (label, b) in [
        ("default", budget()),
        (
            "512/16384",
            SymBudget {
                max_terms: 16384,
                max_degree: 512,
            },
        ),
        (
            "4096/65536",
            SymBudget {
                max_terms: 65536,
                max_degree: 4096,
            },
        ),
    ] {
        let t = std::time::Instant::now();
        let (f, c) = sym(&doc, ProfileLift::Guided, SymRules::shipped(), b);
        println!(
            "tilted derived Guided 1e-3 shipped budget {label}: frozen {} sym0 {} num {} in {:?}\n  fails {} {}",
            c.frozen,
            c.symbolic_zero,
            c.numeric,
            t.elapsed(),
            f.len(),
            head(f.first().map_or("", String::as_str), 260)
        );
    }
}

/// **SYM-5's pin: the derived boss on a tilted frame certifies where
/// its authored twin does.** The acceptance row of the unit, under
/// BOTH `ProfileLift`s, at the widths the twin certifies at on the
/// symbolic lane.
///
/// What carries it is rule E ([`SymRules::common_factor`], the
/// quotient's common factor): with the rule off — `without_rule_e`,
/// M10-10's tier bit for bit — the derived boss refuses under `Guided`
/// on `newell_plane_residual` while the twin certifies, and that
/// asymmetry is asserted here too, so the pin says what the rule is
/// for and not only that it works.
///
/// At a half-width of `5e-2` the derived boss still refuses, and the
/// refusal is NOT the tier's: it is the clause-1 `Invalid` margin
/// `work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum`
/// carries, the same one PR-1 measured on the translated document.
/// That is asserted by name below — it is a defect pinned as the
/// current behaviour, and when that row is answered this assertion
/// fails and the width joins the parity list above it.
#[test]
fn m10_the_tilted_derived_boss_certifies_where_its_authored_twin_does() {
    let eps = Tol::witness().eps();
    for half in [eps / 8.0, 1.0e-3] {
        for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
            let authored = sym(
                &boss_on_tilted(half, false),
                lift,
                SymRules::shipped(),
                budget(),
            )
            .0;
            let derived = sym(
                &boss_on_tilted(half, true),
                lift,
                SymRules::shipped(),
                budget(),
            )
            .0;
            assert!(
                authored.is_empty(),
                "half={half:e} {lift:?}: the authored twin certifies: {authored:?}"
            );
            assert!(
                derived.is_empty(),
                "half={half:e} {lift:?}: the derived boss must certify where its twin does — \
                 this is rule E's acceptance row: {derived:?}"
            );
        }
    }

    // Rule E is what carries it: with the rule off the derived boss
    // refuses under Guided where the twin certifies.
    let off = SymRules::without_rule_e();
    let (derived_off, _) = sym(
        &boss_on_tilted(1.0e-3, true),
        ProfileLift::Guided,
        off,
        budget(),
    );
    let (authored_off, _) = sym(
        &boss_on_tilted(1.0e-3, false),
        ProfileLift::Guided,
        off,
        budget(),
    );
    assert!(
        authored_off.is_empty(),
        "the twin certifies with rule E off too: {authored_off:?}"
    );
    assert_eq!(
        derived_off.len(),
        1,
        "without rule E the derived boss refuses: {derived_off:?}"
    );
    assert!(
        derived_off[0].contains("newell_plane_residual"),
        "and it refuses on the side plane's newell residual: {derived_off:?}"
    );

    // The one width still refused, and the reason, read off the kernel.
    let (wide, _) = sym(
        &boss_on_tilted(5.0e-2, true),
        ProfileLift::Guided,
        SymRules::shipped(),
        budget(),
    );
    assert_eq!(wide.len(), 1, "one refusal at 5e-2: {wide:?}");
    assert!(
        wide[0].contains("newell_plane_residual") && wide[0].contains("margin is invalid"),
        "THIS PINS A DEFECT AS THE CURRENT BEHAVIOUR, NOT A DESIRED ONE: at 5e-2 the tier has \
         done its work and clause 1 refuses first — newell normalises a cross-sum whose \
         enclosure contains zero \
         (work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum). When that \
         row is answered this assertion FAILS and 5e-2 joins the parity list above: {wide:?}"
    );
}
