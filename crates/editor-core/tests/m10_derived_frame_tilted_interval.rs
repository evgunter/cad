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
use crate::m10_8_harness::head;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{
    CancelToken, CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, EvalOptions,
    Evaluation, Expr, LoopProgram, MeridianEnd, Node, NodeResult, ParamName, ProfileDoc,
    ProfileLift, ProfileProgram, RecipeNodeId, RoleSeg, UnitSym, evaluate,
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
pub(crate) fn boss_on_tilted(half: f64, derived: bool) -> ProfileDoc {
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

// ---------------------------------------------------------------
// The reach, measured on documents PR-2 did not build — adopted from
// SYM-5's review lane R2 (`sym/5-review-r2` at `37cc12ea9`,
// `sym5_r2_probes_interval.rs`), whose ladder is the one row that
// covers both what the rule reaches and what it does not.
// ---------------------------------------------------------------

/// Which frame the parameter rides.
#[derive(Clone, Copy, Debug)]
enum Base {
    /// PR-2's own: `u = (1,0,0)`, `v = (0,1,t)`. REACHED.
    TiltV,
    /// About the other axis: `u = (1,0,t)`, `v = (0,1,0)`. NOT reached.
    TiltU,
    /// In-plane rotation, BOTH axes carry `t`: `u = (1,t,0)`,
    /// `v = (−t,1,0)`. Neither reached nor not: it certifies with the
    /// dial off as well as on, so it discriminates nothing.
    Spin,
    /// A HALF spin — only `u` carries `t`: `u = (1,t,0)`,
    /// `v = (0,1,0)`. REACHED (R1's shape: one derived rung refuses
    /// `carrier_endpoint_end` `[0, 1.43e-1]` with the dial off and
    /// certifies with it on).
    HalfSpin,
    /// Genuinely NON-unit authored axes: `u = (2,0,0)`, `v = (0,2,t)`.
    /// The datum door normalises them; reached, and no false `Zero`.
    NonUnit,
    /// **SYM-8's reviews (R1, and R2's `tilt-uv`).** BOTH axes tilt out
    /// of plane: `u = (1,0,t)`, `v = (0,1,t)`. The face normal is
    /// `(−t, −t, 1)/sqrt(1 + 2t²)`, so `n.z` is an `Inv` of a `sqrt`
    /// atom as in the tilt-`u` document but over a different `P`, and
    /// `n.x`, `n.y` carry the parameter too — the shape rule F folds,
    /// with bigger products above it.
    TiltUV,
    /// **SYM-8's review R1.** The tilt-`u` frame with `v` FLIPPED:
    /// `u = (1,0,t)`, `v = (0,−1,0)`, so the face normal is
    /// `(t, 0, −1)/sqrt(1 + t²)` and `n.z` is NEGATIVE — a `−1` over a
    /// `sqrt` atom. Rule F's predicate must DECLINE it (a negative
    /// coefficient on the only term) and the document must read the
    /// same at both dials.
    FlipZ,
    /// **SYM-8's reviews (R1, and R2's `z-touches-zero`).**
    /// `u = (1,0,0)`, `v = (0,t,1)`: the face normal is
    /// `(0,−1,t)/sqrt(1 + t²)`, so `n.z` is a BARE PARAMETER over a
    /// `sqrt` atom and its enclosure STRADDLES zero at a wide box —
    /// the case `Interval::copysign` must answer `[−1, 1]` on. Rule F
    /// must decline it.
    TiltNZ,
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
        Base::HalfSpin => (
            [scl(1.0), t.clone(), scl(0.0)],
            [scl(0.0), scl(1.0), scl(0.0)],
        ),
        Base::NonUnit => (
            [scl(2.0), scl(0.0), scl(0.0)],
            [scl(0.0), scl(2.0), t.clone()],
        ),
        Base::TiltUV => (
            [scl(1.0), scl(0.0), t.clone()],
            [scl(0.0), scl(1.0), t.clone()],
        ),
        Base::FlipZ => (
            [scl(1.0), scl(0.0), t.clone()],
            [scl(0.0), scl(-1.0), scl(0.0)],
        ),
        Base::TiltNZ => (
            [scl(1.0), scl(0.0), scl(0.0)],
            [scl(0.0), t.clone(), scl(1.0)],
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

/// One cube extruded from `base`, and the `FaceFrame` on its START
/// cap — the cap whose normal is the negation of the end cap's, so
/// `n.z` carries a negative coefficient and rule F declines it
/// (SYM-8's review R2).
fn start_cap_frame(r: &mut Recorder, base: RecipeNodeId) -> RecipeNodeId {
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
        face: fixture::fname(cube, RoleSeg::Cap(CapEnd::Start)),
        spin: ang(0.0),
    }))
}

/// A half-turn revolve of an off-axis square about the base frame's
/// `v` through its origin; the frame on its END cap.
fn revolved(r: &mut Recorder, base: RecipeNodeId) -> RecipeNodeId {
    let p = r.insert(Node::Profile(fixture::desc(
        base,
        vec![fixture::square(1.5, 0.0, 0.5)],
    )));
    let axis = r.insert(fixture::axis_in_plane(base, (0.0, 0.0), (0.0, 1.0)));
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

fn boss_on(r: &mut Recorder, on: RecipeNodeId) {
    let boss_p = r.insert(Node::Profile(fixture::desc(
        on,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.25),
    });
}

/// Where the boss sits: on the authored frame, on `n` derived frames
/// stacked over it, or on a revolved body's cap.
#[derive(Clone, Copy, Debug)]
enum Place {
    Authored,
    Derived(usize),
    Revolved,
    /// **SYM-8's review R2 (`start-cap`).** One derived frame, on the
    /// cube's START cap instead of its end: the normal is the negation,
    /// `n.z = −1/sqrt(P(t))`, which rule F's predicate declines (a
    /// negative coefficient). The document class the current predicate
    /// does NOT reach, named rather than left to inference.
    DerivedStartCap,
}

fn r2_document(half: f64, base: Base, place: Place) -> ProfileDoc {
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
    let b = base_frame(&mut r, &t, base);
    let on = match place {
        Place::Authored => b,
        Place::Derived(n) => stacked(&mut r, b, n),
        Place::Revolved => revolved(&mut r, b),
        Place::DerivedStartCap => start_cap_frame(&mut r, b),
    };
    boss_on(&mut r, on);
    r.doc
}

/// **What rule E reaches, and what it does not** (R2's e2e ladder,
/// adopted). Eight documents the unit did not build, both lifts, the
/// plain lane and the tier with the dial off and on.
///
/// The reach is the DOCUMENT's and not a class. REACHED: the tilt about
/// `v` (PR-2's own, which the gating parity row above holds), a HALF
/// spin `u = (1,t,0)`, `v = (0,1,0)` (R1's shape), non-unit authored
/// axes, and two derived frames STACKED. On the stacked document the
/// word needs its lift: under `Pinned` it certifies at BOTH dials and
/// what the rule buys is the cost (219.4 s off → 1.1 s on), while
/// under `Guided` the rule takes its refusals from four to ONE and the
/// one left is the value channel's clause-1 `Invalid` on the boss.
/// NOT reached: the tilt about `u`, where the rule turns the DEGREE
/// wall into a TERM wall (R2: the frozen `Powi` kid 606 terms at
/// degree 60 becomes 440 at degree 28, and `440² > MAX_TERMS`) behind
/// the `abs(1/sqrt(…))` and `copysign` atoms a `FaceFrame`'s `u_ref`
/// derivation mints; and a `FaceFrame` on a REVOLVED body's cap, which
/// neither dial certifies. NEITHER: the full in-plane spin, which
/// certifies at both dials.
///
/// It asserts the one thing that must hold on every document: the rule
/// never REFUSES what the dial-off tier certifies.
///
/// **This row is `#[ignore]`d, so that assertion does not GATE** — the
/// dial-off side of the stacked case is 219 s and the revolved cap 22 s
/// per lift, which is why. What gates is
/// `m10_the_tilted_derived_boss_certifies_where_its_authored_twin_does`
/// above, on the tilt-`v` document. The cheapest rung here that could
/// be un-ignored is `half-spin derived` (well under a second at both
/// dials); it is left with the rest so the ladder reads as one table,
/// and the parity row already gates a document of the same family.
#[test]
#[ignore = "evidence-only: the reach on eight documents; the stacked case is minutes with the dial off"]
fn sym5_the_reach_on_documents_the_unit_did_not_build() {
    let half = 1.0e-3;
    let cases: Vec<(&str, Base, Place)> = vec![
        ("tiltU authored", Base::TiltU, Place::Authored),
        ("tiltU derived", Base::TiltU, Place::Derived(1)),
        ("spin authored", Base::Spin, Place::Authored),
        ("spin derived", Base::Spin, Place::Derived(1)),
        ("half-spin derived", Base::HalfSpin, Place::Derived(1)),
        ("tiltV stacked-2", Base::TiltV, Place::Derived(2)),
        ("tiltV revolved-cap", Base::TiltV, Place::Revolved),
        ("non-unit authored", Base::NonUnit, Place::Authored),
        ("non-unit derived", Base::NonUnit, Place::Derived(1)),
    ];
    let only = std::env::var("CAD_SYM5_CASES").ok();
    let mut lost = Vec::new();
    for (name, base, place) in cases {
        if only
            .as_deref()
            .is_some_and(|l| !l.split(',').any(|n| n.trim() == name))
        {
            continue;
        }
        let doc = r2_document(half, base, place);
        for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
            let p = plain(&doc, lift);
            let t0 = std::time::Instant::now();
            let (off, c_off) = sym(&doc, lift, SymRules::without_rule_e(), budget());
            let d_off = t0.elapsed().as_secs_f64();
            let t1 = std::time::Instant::now();
            let (on, c_on) = sym(&doc, lift, SymRules::shipped(), budget());
            let d_on = t1.elapsed().as_secs_f64();
            println!(
                "{name:<20} {lift:?}: plain {} | off {} frozen {} sym0 {} {d_off:.1}s | on {} frozen {} sym0 {} {d_on:.1}s",
                p.len(),
                off.len(),
                c_off.frozen,
                c_off.symbolic_zero,
                on.len(),
                c_on.frozen,
                c_on.symbolic_zero
            );
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

/// The shipped set — which carries **rule F**, the manifest sign,
/// today. The SYM-8 rows below read it through this name so that the
/// day rule F leaves the shipped set they read the dial and not a
/// wish: [`SymRules::without_rule_f`] is the differential's other
/// half, and this is `shipped` with the dial asserted ON, so a rule F
/// dialled off reds here instead of turning every "F-on" column into a
/// silent copy of the "F-off" one.
fn shipped_with_rule_f() -> SymRules {
    let s = SymRules::shipped();
    assert!(
        s.manifest_sign,
        "the rows below read the shipped set as rule F's ON column"
    );
    s
}

/// How many `copysign(`/`abs(`/`sqrt(` atoms a rendered form spells,
/// and how many of its TOP-LEVEL terms carry one — arithmetic on the
/// render, which is what the census of a frozen kid's terms is.
fn atom_census(rendered: &str) -> String {
    let terms: Vec<&str> = rendered.split(" + ").collect();
    let carrying = |needle: &str| terms.iter().filter(|t| t.contains(needle)).count();
    format!(
        "copysign {} in {} terms | abs {} in {} terms | sqrt {} | terms(top) {}",
        rendered.matches("copysign(").count(),
        carrying("copysign("),
        rendered.matches("abs(").count(),
        carrying("abs("),
        rendered.matches("sqrt(").count(),
        terms.len()
    )
}

/// The DISTINCT `<name>(…)` atoms a render spells, each with its
/// argument to the render's own nesting depth — balanced on the
/// parentheses, so `copysign(1, (1) / (sqrt(…)))` comes back whole.
/// What "the fold never fires" is READ from: an atom still standing
/// in a rule-F-ON render is one the predicate declined, and its
/// argument is the form it declined.
fn distinct_atoms(rendered: &str, name: &str) -> Vec<String> {
    let needle = format!("{name}(");
    let mut out: Vec<String> = Vec::new();
    let mut from = 0;
    while let Some(i) = rendered[from..].find(&needle) {
        let start = from + i;
        let mut depth = 0usize;
        let mut end = None;
        for (j, ch) in rendered[start..].char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(start + j + ch.len_utf8());
                        break;
                    }
                }
                _ => {}
            }
        }
        // The explanation cuts a long render at a fixed width, so an
        // atom can be cut open: keep what stands and say so.
        let (atom, end) = match end {
            Some(end) => (rendered[start..end].to_owned(), end),
            None => (format!("{}…[cut]", &rendered[start..]), rendered.len()),
        };
        if !out.contains(&atom) {
            out.push(atom);
        }
        from = end;
    }
    out
}

/// **The wall, rendered** — one document at the named widths, both
/// lifts (`CAD_SYM8_LIFT` narrows), rule F OFF and ON: the counts,
/// the refusals, the split, and for every blocked residual its
/// enclosure, early form, atom census, the DAG below it explained to
/// `CAD_SYM12_DEPTH` levels (default 6; `CAD_SYM12_DIAL` narrows to
/// one of `F-off`/`F-on`; the `Pinned` lift on a cube
/// renders every `NumericZero` residual and at depth 6 can exhaust a
/// small box — set the depth lower there and say so), with every
/// `FROZEN` node, every `num`-sized node and the census of every
/// rendered form, and finally the DISTINCT `copysign`/`abs` atoms the
/// early form and the explanation spell, with their arguments.
fn render_wall(name: &str, base: Base, place: Place, halves: &[f64]) {
    use geom_core::sym::report::{
        ShapeOutcome, explain_depth, name_param, start_shape_report, take_shape_report,
    };
    let only_half = std::env::var("CAD_SYM8_HALF").ok();
    let depth: usize = std::env::var("CAD_SYM12_DEPTH")
        .ok()
        .and_then(|d| d.trim().parse().ok())
        .unwrap_or(6);
    if let Some(chars) = std::env::var("CAD_SYM12_CHARS")
        .ok()
        .and_then(|d| d.trim().parse().ok())
    {
        geom_core::sym::report::explain_render_chars(chars);
    }
    for &half in halves {
        if only_half
            .as_deref()
            .is_some_and(|h| h.trim() != format!("{half:e}"))
        {
            continue;
        }
        let doc = r2_document(half, base, place);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let box_ = ParamBox::of(&analyzed);
        for name_ in box_.axes().keys() {
            name_param(&name_.0);
        }
        let only_lift = std::env::var("CAD_SYM8_LIFT").ok();
        for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
            if only_lift
                .as_deref()
                .is_some_and(|l| l.trim() != format!("{lift:?}"))
            {
                continue;
            }
            let only_dial = std::env::var("CAD_SYM12_DIAL").ok();
            for (label, rules) in [
                ("F-off", SymRules::without_rule_f()),
                ("F-on", shipped_with_rule_f()),
            ] {
                if only_dial.as_deref().is_some_and(|d| d.trim() != label) {
                    continue;
                }
                let o = EvalOptions {
                    param_box: Some(Arc::new(box_.clone())),
                    profile_lift: lift,
                    ..EvalOptions::default()
                };
                start_shape_report();
                explain_depth(depth);
                let t0 = std::time::Instant::now();
                let (refusal, counts) = geom_core::sym::with_session_rules(budget(), rules, || {
                    let ev: Evaluation<geom_core::Sym<Interval>> =
                        evaluate(&doc, None, &CancelToken::new(), &o, Tol::witness());
                    failures(&ev)
                });
                let dt = t0.elapsed().as_secs_f64();
                let shapes = take_shape_report();
                explain_depth(0);
                println!("=== {name} half={half:e} {lift:?} {label} depth {depth} ({dt:.1}s)");
                println!("    counts {counts:?}");
                println!(
                    "    refusals {:?}",
                    refusal.iter().map(|r| head(r, 220)).collect::<Vec<_>>()
                );
                for (pred, row) in crate::m10_8_harness::split(&shapes) {
                    println!("    split {pred:<36} {row:?}");
                }
                // Every rendered form of the replay, blocked or not:
                // the atoms a NumericZero residual carries say whether
                // the fold fired on a decision the document DID reach.
                let mut all_atoms: Vec<String> = Vec::new();
                let mut rendered = 0usize;
                for s in &shapes {
                    if let Some(f) = &s.early_form {
                        rendered += 1;
                        for kind in ["copysign", "abs"] {
                            for a in distinct_atoms(f, kind) {
                                if !all_atoms.contains(&a) {
                                    all_atoms.push(a);
                                }
                            }
                        }
                    }
                }
                println!(
                    "    rendered {rendered} early forms; distinct copysign/abs atoms across them: {}",
                    all_atoms.len()
                );
                for a in &all_atoms {
                    println!("      atom {}", head(a, 300));
                }
                let mut seen: std::collections::BTreeSet<&str> = Default::default();
                for s in &shapes {
                    if matches!(
                        s.outcome,
                        ShapeOutcome::Indeterminate | ShapeOutcome::Invalid
                    ) && seen.insert(s.predicate)
                    {
                        println!("--- blocked {} {:?}", s.predicate, s.outcome);
                        println!("    enclosure {:?} sizes {:?}", s.enclosure, s.sizes);
                        if let Some(f) = &s.early_form {
                            println!("    early  {}", head(f, 400));
                            println!("    census {}", atom_census(f));
                        }
                        if let Some(e) = &s.explain {
                            let frozen = e.lines().filter(|l| l.contains("FROZEN")).count();
                            println!("    explain: {} lines, {frozen} FROZEN", e.lines().count());
                            let mut path_atoms: Vec<String> = Vec::new();
                            for l in e.lines() {
                                if l.contains("FROZEN") || l.contains(": num") {
                                    println!("      {}", head(l, 200));
                                } else if l.trim_start().starts_with("= ") {
                                    println!("        census {}", atom_census(l));
                                    for kind in ["copysign", "abs"] {
                                        for a in distinct_atoms(l, kind) {
                                            if !path_atoms.contains(&a) {
                                                path_atoms.push(a);
                                            }
                                        }
                                    }
                                }
                            }
                            println!(
                                "    distinct copysign/abs atoms on the explained path: {}",
                                path_atoms.len()
                            );
                            for a in &path_atoms {
                                println!("      path atom {}", head(a, 8000));
                            }
                        }
                    }
                }
            }
        }
    }
}

/// **SYM-8 Phase 1.1 — the wall, named.** The tilt-`u` derived
/// document (`u = (1,0,t)`, a cube extruded from it, a `FaceFrame` on
/// its cap, the boss on that) is where SYM-5's rule E turns the DEGREE
/// wall into a TERM wall and stops: it refuses `carrier_endpoint_end`
/// identically with rule E on and off. This row renders the refused
/// residual at `explain_depth 6` at both widths and both lifts, with
/// rule F (the manifest sign) OFF and ON, and prints for every node of
/// the decision path its early form's size, whether the walk FROZE it,
/// and the census of `copysign`/`abs` atoms its render carries — so
/// "terms before / after, degree, whether it would fit the budget" is
/// read off the instrument and not eyeballed.
#[test]
#[ignore = "evidence-only: SYM-8 Phase 1.1, the tilt-U wall with and without rule F"]
fn sym8_phase1_the_tilt_u_wall_with_and_without_the_manifest_sign() {
    render_wall("tiltU derived", Base::TiltU, Place::Derived(1), &[1.0e-3, 5.0e-2]);
}

/// **SYM-12 Phase 1.1 — `tiltUV`, rendered.** Both SYM-8 reviews
/// predicted the tilt about `u` AND `v` (`n.z = 1/sqrt(1 + 2t²)`) as
/// the shape rule F folds, and the ladder above measured the rule
/// moving not one count there at either lift. This row is the render
/// that ladder did not take: the same instrument as the tilt-`u`
/// wall's, on the `tiltUV` document, so which of the three is true —
/// the fold never fires (the atoms still stand in the F-on render, and
/// their arguments say what form the predicate declined), it fires but
/// the node is frozen before it combines (no atom stands and a FROZEN
/// node sits on the path), or it fires where no decision is asked —
/// is read off the atoms and the path rather than guessed.
#[test]
#[ignore = "evidence-only: SYM-12 Phase 1.1, the tilt-UV document rendered with and without rule F"]
fn sym12_phase1_the_tilt_uv_document_rendered() {
    render_wall("tiltUV derived", Base::TiltUV, Place::Derived(1), &[1.0e-3]);
}

/// **SYM-8 Phase 1.1's ladder** — the tilt-`u` derived document and its
/// authored twin at both widths and both lifts, with rule F off and on,
/// counts and first refusal only (no shape report, so it is affordable
/// where `sym8_phase1_the_tilt_u_wall_with_and_without_the_manifest_sign`
/// is not: that row renders every blocked residual to six levels, which
/// on the `Pinned` lift exhausts the measuring box's memory at BOTH
/// dials).
#[test]
#[ignore = "evidence-only: SYM-8 Phase 1.1, the tilt-U ladder at both widths and both lifts"]
fn sym8_phase1_the_tilt_u_ladder() {
    for half in [1.0e-3, 5.0e-2] {
        for (kind, place) in [
            ("authored", Place::Authored),
            ("derived", Place::Derived(1)),
        ] {
            let doc = r2_document(half, Base::TiltU, place);
            for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
                let p = plain(&doc, lift);
                println!(
                    "tiltU {kind} half={half:e} {lift:?} plain: {} {}",
                    p.len(),
                    head(p.first().map_or("", String::as_str), 200)
                );
                for (label, rules) in [
                    ("F-off", SymRules::without_rule_f()),
                    ("F-on", shipped_with_rule_f()),
                ] {
                    let t = std::time::Instant::now();
                    let (f, c) = sym(&doc, lift, rules, budget());
                    println!(
                        "tiltU {kind} half={half:e} {lift:?} {label}: frozen {} sym0 {} num {} in {:.1}s\n  fails {} {}",
                        c.frozen,
                        c.symbolic_zero,
                        c.numeric,
                        t.elapsed().as_secs_f64(),
                        f.len(),
                        head(f.first().map_or("", String::as_str), 200)
                    );
                }
            }
        }
    }
}

/// **SYM-8's GATING tilt-`u` row — the acceptance the unit owes.** The
/// spec asked for the tilt-U parity row green at the widths the twin
/// certifies at, or the width it stops at pinned BY NAME with the
/// reason. It stops, so this row pins the stop.
///
/// On the tilt-`u` derived document (`u = (1,0,t)`, a cube extruded
/// from it, a `FaceFrame` on its cap, the boss on that) at
/// `half = 1e-3` under `Guided`:
///
/// - with rule F SHUT the boss refuses `carrier_endpoint_end`, whose
///   split is 24/0/0/1 — the residual is a `sqrt` over a FROZEN
///   `Powi ^2` whose kid is 440 terms at degree 27 and `440² >
///   MAX_TERMS`;
/// - with rule F ON that square is built, `carrier_endpoint_end` is
///   33/0/0/0 — every decision a THEOREM — and the document's refusal
///   MOVES, to a `newell_plane_residual` straddle the tier does not
///   prove.
///
/// Both refusals are asserted by name, so the day either moves this
/// reds and says which. The remaining wall is
/// `work/sym/the-tilt-u-newell-residual-is-the-next-wall`; when it is
/// answered, the second half of this row fails and the width joins the
/// parity list.
///
/// Cost: two evaluations of a small document, well under a second each
/// in release and about four seconds in the test profile — the split
/// is read from the shape report, which is what the second of those
/// pays for.
#[test]
fn m10_the_tilt_u_derived_boss_stops_on_the_newell_residual_and_names_it() {
    use geom_core::sym::report::{start_shape_report, take_shape_report};
    let doc = r2_document(1.0e-3, Base::TiltU, Place::Derived(1));
    let mut seen = Vec::new();
    for (label, rules) in [
        ("F-off", SymRules::without_rule_f()),
        ("F-on", shipped_with_rule_f()),
    ] {
        start_shape_report();
        let (fails, counts) = sym(&doc, ProfileLift::Guided, rules, budget());
        let shapes = take_shape_report();
        let split = crate::m10_8_harness::split(&shapes);
        let row = |p: &str| split.get(p).copied().unwrap_or([0; 4]);
        println!(
            "tiltU derived Guided 1e-3 {label}: {counts:?}\n  carrier_endpoint_end {:?} \
             newell_plane_residual {:?}\n  fails {} {}",
            row("carrier_endpoint_end"),
            row("newell_plane_residual"),
            fails.len(),
            head(fails.first().map_or("", String::as_str), 200)
        );
        seen.push((row("carrier_endpoint_end"), fails));
    }
    let (off_split, off_fails) = &seen[0];
    let (on_split, on_fails) = &seen[1];
    assert_eq!(
        *off_split,
        [24, 0, 0, 1],
        "with rule F shut the carrier endpoint is one decision short of the theorem"
    );
    assert_eq!(
        off_fails.len(),
        1,
        "and the document refuses: {off_fails:?}"
    );
    assert!(
        off_fails[0].contains("carrier_endpoint_end"),
        "the wall rule F is measured against is the carrier endpoint: {off_fails:?}"
    );
    assert_eq!(
        *on_split,
        [33, 0, 0, 0],
        "rule F takes the whole predicate: every decision a theorem"
    );
    assert_eq!(
        on_fails.len(),
        1,
        "the document still refuses: {on_fails:?}"
    );
    assert!(
        on_fails[0].contains("newell_plane_residual"),
        "and the wall it stops at now is the newell residual, by name \
         (`work/sym/the-tilt-u-newell-residual-is-the-next-wall`): {on_fails:?}"
    );
}

/// **SYM-12 Phase 1.3 — the one-sided documents, per lift, with and
/// without rule F, under whatever the process's manifest-sign arm is**
/// (the negative arm is a process dial while it is hand-planted; this
/// row is run once without it and once with it, and the two prints
/// are the table). Per document and lift: the counts, the first
/// refusal by name, the split of every predicate that refused, and
/// the widest half-width that certifies whole — a log bisection of
/// `half` between `1e-4` and `5e-2` in eight steps, `(certifies,
/// refuses)`, `NaN` when even `1e-4` refuses. `CAD_SYM12_CASES` names
/// a comma-separated subset; `CAD_SYM8_LIFT` narrows the lift.
#[test]
#[ignore = "evidence-only: SYM-12 Phase 1.3, the one-sided documents under the manifest-sign arms"]
fn sym12_phase1_the_one_sided_documents_ladder() {
    use geom_core::sym::report::{start_shape_report, take_shape_report};
    let only = std::env::var("CAD_SYM12_CASES").ok();
    let only_lift = std::env::var("CAD_SYM8_LIFT").ok();
    for (name, base, place) in [
        ("tiltU start-cap", Base::TiltU, Place::DerivedStartCap),
        ("flipZ derived", Base::FlipZ, Place::Derived(1)),
        ("tiltU derived", Base::TiltU, Place::Derived(1)),
        ("tiltUV derived", Base::TiltUV, Place::Derived(1)),
        ("tiltNZ derived", Base::TiltNZ, Place::Derived(1)),
        ("tiltV derived", Base::TiltV, Place::Derived(1)),
    ] {
        if only
            .as_deref()
            .is_some_and(|l| !l.split(',').any(|n| n.trim() == name))
        {
            continue;
        }
        for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
            if only_lift
                .as_deref()
                .is_some_and(|l| l.trim() != format!("{lift:?}"))
            {
                continue;
            }
            for (label, rules) in [
                ("F-off", SymRules::without_rule_f()),
                ("F-on ", shipped_with_rule_f()),
            ] {
                let doc = r2_document(1.0e-3, base, place);
                start_shape_report();
                let t0 = std::time::Instant::now();
                let (f, c) = sym(&doc, lift, rules, budget());
                let dt = t0.elapsed().as_secs_f64();
                let shapes = take_shape_report();
                let split = crate::m10_8_harness::split(&shapes);
                println!(
                    "{name} half=1e-3 {lift:?} {label}: sym0 {} reg {} gated {} num {} frozen {} \
                     in {dt:.1}s\n  fails {} {}",
                    c.symbolic_zero,
                    c.registered,
                    c.sign_gated,
                    c.numeric,
                    c.frozen,
                    f.len(),
                    head(f.first().map_or("", String::as_str), 200)
                );
                let mut named: Vec<&str> = Vec::new();
                for (pred, row) in &split {
                    if row[3] > 0 && f.iter().any(|r| r.contains(pred)) {
                        named.push(pred);
                        println!("  refused split {pred:<32} {row:?}");
                    }
                }
                // The widest certifying half-width under this lift and
                // arm: cheap on these cubes (a replay is well under a
                // second each way with the report off).
                let certifies = |h: f64| sym(&r2_document(h, base, place), lift, rules, budget()).0.is_empty();
                let (mut lo, mut hi) = (1.0e-4_f64, 5.0e-2_f64);
                let ceiling = if !certifies(lo) {
                    (f64::NAN, lo)
                } else if certifies(hi) {
                    (hi, f64::INFINITY)
                } else {
                    for _ in 0..8 {
                        let mid = (0.5 * (lo.ln() + hi.ln())).exp();
                        if certifies(mid) {
                            lo = mid;
                        } else {
                            hi = mid;
                        }
                    }
                    (lo, hi)
                };
                println!(
                    "  half-width ceiling: certifies {:.4e}, refuses {:.4e}",
                    ceiling.0, ceiling.1
                );
            }
        }
    }
}

/// **The documents the two reviews built and could not run** (R1's
/// `r1_sym8_three_documents_the_unit_did_not_measure`, R2's `tilt-uv`,
/// `start-cap` and `z-touches-zero`), adopted here as one ladder
/// rather than a second file with a copied preamble.
///
/// - `TiltUV` — a tilt about `u` AND `v`; `n.z = 1/sqrt(1 + 2t²)`, the
///   shape rule F folds, on a document whose `n.x` and `n.y` carry the
///   parameter too.
/// - `FlipZ` — the same tilt with `v` flipped, so `n.z` is NEGATIVE;
///   the predicate must decline and the document read the same at both
///   dials.
/// - `TiltNZ` — `n.z` a bare parameter over a `sqrt` atom, whose
///   enclosure straddles zero at the wide box; the predicate must
///   decline.
/// - `TiltU` on the START cap — `n.z = −1/sqrt(P(t))`, the
///   document class the current predicate does not reach.
///
/// Written by the reviews, not run by them (neither box could link the
/// `editor-core` interval binary); run once by this fix pass. It
/// asserts the one thing that must hold everywhere — rule F never
/// REFUSES what the dial-off tier certifies — and prints the rest.
///
/// **What it showed, 2026-09-21, dev build** (`SymCounts` as
/// `sym0 / numeric / frozen`, F off → on): rule F moves NOT ONE COUNT
/// on any of the four, at either lift.
///
/// | document | `Pinned` | `Guided` |
/// | --- | --- | --- |
/// | tiltUV derived | 576/746/291 both dials, certifies | 525/308/37 both dials, refuses `carrier_endpoint_end` |
/// | flipZ derived | 754/568/1270 both dials, certifies | 525/308/37 both dials, refuses `carrier_endpoint_end` |
/// | tiltNZ derived, `half = 1e-3` | 876/446/1269 both, certifies | 956/526/1467 both, certifies |
/// | tiltNZ derived, `half = 3e-1` | 876/446/1269 both, certifies | 142/56/0 both, four refusals, the first a clause-1 INVALID newell margin |
/// | tiltU start-cap | 768/554/1270 both dials, certifies | 573/316/398 both dials, refuses `carrier_endpoint_end` |
///
/// Three of those are the reviews' own predictions confirmed by
/// execution: `flipZ`, `tiltNZ` and the START cap carry an `n.z` the
/// predicate DECLINES (a negative coefficient, or a bare parameter at
/// an odd power), and `flipZ`'s `Pinned` reading is the tilt-`u`
/// document's F-OFF reading to the digit — the same document with the
/// fold declined, which is what "the reach is one-sided" means at the
/// document scale.
///
/// **`tiltUV` is NOT one of them, and that is a finding.** Both reviews
/// predicted it as the shape rule F folds (`n.z = 1/sqrt(1 + 2t²)`, an
/// `Inv` of a `sqrt` atom). Measured, rule F moves nothing there at
/// either lift. Why is NOT measured here — the fold either never fires
/// on that document's `n.z` form or fires without reaching a decision —
/// and saying which needs the render this row does not take. Recorded
/// rather than explained.
#[test]
#[ignore = "evidence-only: the reviews' e2e ladder, four documents x two lifts x two dials"]
fn sym8_the_reviews_documents_the_unit_did_not_measure() {
    let mut lost: Vec<String> = Vec::new();
    for (name, base, place, half) in [
        ("tiltUV derived", Base::TiltUV, Place::Derived(1), 1.0e-3),
        ("flipZ derived", Base::FlipZ, Place::Derived(1), 1.0e-3),
        (
            "tiltNZ derived narrow",
            Base::TiltNZ,
            Place::Derived(1),
            1.0e-3,
        ),
        (
            "tiltNZ derived wide",
            Base::TiltNZ,
            Place::Derived(1),
            3.0e-1,
        ),
        (
            "tiltU start-cap",
            Base::TiltU,
            Place::DerivedStartCap,
            1.0e-3,
        ),
    ] {
        let doc = r2_document(half, base, place);
        for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
            let p = plain(&doc, lift);
            println!(
                "{name} half={half:e} {lift:?} plain: {} {}",
                p.len(),
                head(p.first().map_or("", String::as_str), 160)
            );
            let mut off_ok = false;
            for (label, rules) in [
                ("F-off", SymRules::without_rule_f()),
                ("F-on ", shipped_with_rule_f()),
            ] {
                let t0 = std::time::Instant::now();
                let (f, c) = sym(&doc, lift, rules, budget());
                println!(
                    "{name} half={half:e} {lift:?} {label}: sym0 {} reg {} gated {} num {} \
                     frozen {} in {:.1}s\n  fails {} {}",
                    c.symbolic_zero,
                    c.registered,
                    c.sign_gated,
                    c.numeric,
                    c.frozen,
                    t0.elapsed().as_secs_f64(),
                    f.len(),
                    head(f.first().map_or("", String::as_str), 200)
                );
                if label.trim() == "F-off" {
                    off_ok = f.is_empty();
                } else if off_ok && !f.is_empty() {
                    lost.push(format!("{name} half={half:e} {lift:?}: {f:?}"));
                }
            }
        }
    }
    assert!(
        lost.is_empty(),
        "rule F refused what the dial-off tier certified: {lost:?}"
    );
}
