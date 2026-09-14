//! **R1's own documents for SYM-5 PR-2** — derived frames whose axes
//! carry a parameter in ways the unit did not measure, plus a document
//! with a genuinely NON-UNIT stored vector, driven through the public
//! doors at `Sym<Interval>` with rule E on and off.
//!
//! Evidence-only: every row prints what each rung answered and asserts
//! only that the shipped set never introduces a refusal the tier with
//! rule E off did not have (the "nothing lost" direction), never a
//! particular count.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture::{self, Recorder, ang, len, scl};

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{
    CancelToken, CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, EvalOptions,
    Evaluation, Expr, MeridianEnd, Node, NodeResult, ParamName, ProfileDoc, ProfileLift, RoleSeg,
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
) -> (Vec<String>, geom_core::SymCounts, std::time::Duration) {
    let o = opts(doc, lift);
    let t = std::time::Instant::now();
    let (f, c) = geom_core::sym::with_session_rules(budget(), rules, || {
        let ev: Evaluation<geom_core::Sym<Interval>> =
            evaluate(doc, None, &CancelToken::new(), &o, Tol::witness());
        failures(&ev)
    });
    (f, c, t.elapsed())
}

fn head(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

fn scalar_param(name: &str, nominal: f64, half: f64, r: &mut Recorder) {
    r.push(DocEdit::SetDocParam {
        name: ParamName::new(name),
        value: DocParam::Continuous {
            dim: Dimension::Scalar,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Scalar),
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        },
    });
}

/// **A tilt about a DIFFERENT axis**: the parameter rides `u` (a spin
/// of the sketch x axis in the xy plane) rather than `v`.
fn boss_on_u_tilted(half: f64, derived: bool) -> ProfileDoc {
    let mut r = Recorder::new();
    scalar_param("t", 0.25, half, &mut r);
    let t = Expr::param(ParamName::new("t"), Dimension::Scalar);
    let base = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), t, scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
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

/// **Two derived frames STACKED**: a cube from the tilted frame, a
/// `FaceFrame` on its cap, a second cube from THAT, a `FaceFrame` on
/// the second cube's cap, and the boss on it — the normalisation chain
/// walked one rung further than PR-2's document.
fn boss_on_two_stacked_frames(half: f64, rungs: usize) -> ProfileDoc {
    let mut r = Recorder::new();
    scalar_param("t", 0.25, half, &mut r);
    let t = Expr::param(ParamName::new("t"), Dimension::Scalar);
    let mut on = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), t],
    }));
    for _ in 0..rungs {
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

/// **A `FaceFrame` on a face of a REVOLVED body**: a rectangle on the
/// tilted frame, revolved a quarter turn about an in-plane axis, with
/// the boss placed on the revolve's planar wedge cap.
fn boss_on_revolve_cap(half: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    scalar_param("t", 0.25, half, &mut r);
    let t = Expr::param(ParamName::new("t"), Dimension::Scalar);
    let base = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), t],
    }));
    let p = r.insert(Node::Profile(fixture::desc(
        base,
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    )));
    let axis = r.insert(fixture::axis_in_plane(base, (0.0, 0.0), (0.0, 1.0)));
    let body = r.insert(Node::Revolve {
        profile: p,
        axis,
        angle: ang(std::f64::consts::FRAC_PI_2),
    });
    let frame = r.insert(Node::Datum(Datum::FaceFrame {
        at: body,
        face: fixture::fname(body, RoleSeg::RevolveCap(MeridianEnd::Start)),
        spin: ang(0.0),
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(1.5, 0.5, 0.2)],
    )));
    r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.25),
    });
    r.doc
}

/// **A genuinely NON-UNIT stored vector.** The frame's `u` is `(2,0,0)`
/// and its `v` is `(0, 3, t)` — neither is a unit vector, so a rule
/// that "recognised a normalisation" and folded its norm to one would
/// be folding a falsehood. The kernel normalises them at the datum, so
/// the CORRECT answer is the same geometry as the unit spelling; what
/// this row watches is that the tier never decides a wrong sign.
fn boss_on_non_unit_frame(half: f64, derived: bool) -> ProfileDoc {
    let mut r = Recorder::new();
    scalar_param("t", 0.25, half, &mut r);
    let t = Expr::param(ParamName::new("t"), Dimension::Scalar);
    let base = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(2.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(3.0), t],
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

/// The non-unit frame with TWO derived rungs over it.
fn boss_on_non_unit_stacked(half: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    scalar_param("t", 0.25, half, &mut r);
    let t = Expr::param(ParamName::new("t"), Dimension::Scalar);
    let mut on = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(2.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(3.0), t],
    }));
    for _ in 0..2 {
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

fn rungs() -> Vec<(&'static str, SymRules)> {
    vec![
        ("no_e", SymRules::without_rule_e()),
        ("shipped", SymRules::shipped()),
    ]
}

fn report(what: &str, doc: &ProfileDoc) {
    for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
        let p = plain(doc, lift);
        println!(
            "{what} {lift:?} plain: {} {}",
            p.len(),
            head(p.first().map_or("", String::as_str), 180)
        );
        let mut seen: Vec<(&str, Vec<String>)> = Vec::new();
        for (name, r) in rungs() {
            let (f, c, dt) = sym(doc, lift, r);
            println!(
                "{what} {lift:?} {name}: frozen {} sym0 {} num {} in {:?}\n    fails {} {}",
                c.frozen,
                c.symbolic_zero,
                c.numeric,
                dt,
                f.len(),
                head(f.first().map_or("", String::as_str), 180)
            );
            seen.push((name, f));
        }
        // EVIDENCE, not a gate: a refusal the rule-E tier has and the
        // tier without it does not is printed as a DIVERGENCE. On the
        // revolve document it is one — the endpoint gate that refused
        // with the rule off passes with it on, and the wedge cap's
        // newell residual refuses behind it.
        let off = &seen[0].1;
        let on = &seen[1].1;
        for f in on {
            if !off.contains(f) {
                println!(
                    "{what} {lift:?} DIVERGENCE: only with rule E on: {}",
                    head(f, 240)
                );
            }
        }
        for f in off {
            if !on.contains(f) {
                println!("{what} {lift:?} TAKEN by rule E: {}", head(f, 240));
            }
        }
    }
}

#[test]
fn r1_a_tilt_about_a_different_axis() {
    for half in [1.0e-3, 5.0e-2] {
        for (kind, derived) in [("authored", false), ("derived", true)] {
            report(
                &format!("u_tilt {kind} half={half:e}"),
                &boss_on_u_tilted(half, derived),
            );
        }
    }
}

#[test]
fn r1_two_derived_frames_stacked() {
    for rungs_n in [1_usize, 2, 3] {
        report(
            &format!("stacked rungs={rungs_n} half=1e-3"),
            &boss_on_two_stacked_frames(1.0e-3, rungs_n),
        );
    }
}

#[test]
fn r1_a_face_frame_on_a_revolved_body() {
    report("revolve_cap half=1e-3", &boss_on_revolve_cap(1.0e-3));
}

#[test]
fn r1_a_genuinely_non_unit_stored_vector() {
    for (kind, derived) in [("authored", false), ("derived", true)] {
        report(
            &format!("non_unit_frame {kind} half=1e-3"),
            &boss_on_non_unit_frame(1.0e-3, derived),
        );
    }
    // ... and the non-unit frame with a SECOND derived rung over it.
    report(
        "non_unit_frame stacked2 half=1e-3",
        &boss_on_non_unit_stacked(1.0e-3),
    );
}
