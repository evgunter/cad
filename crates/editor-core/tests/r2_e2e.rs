//! **R2's end-to-end consumer exercise** (review probe, M10-P).
//!
//! The charter asks: author a parameterized profile document through
//! the public doors, evaluate at `Interval` with the lift ON over a
//! genuinely wide parameter, and report what certifies, what aborts
//! typed, and the friction. This file is that exercise, written as a
//! consumer would write it — only items `editor_core` and `profile`
//! export, no crate internals — and it reports rather than asserts,
//! because what it found is a statement about the doors.
//!
//! The document is a bracket with a PARAMETRIC FILLET RADIUS, so the
//! parameter feeds a discrete decision (the fit signs) and not only a
//! coordinate: the plate the unit's own rows use resolves no fillet
//! anywhere, and so has no consumed decision for a width to threaten.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{
    CancelToken, Dimension, Doc, DocEdit, DocParam, EvalOptions, Expr, LoopProgram, Node,
    NodeResult, ParamName, ProfileLift, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget,
    ValuePayload, apply, evaluate,
};
use geom_core::Tol;
use profile::SketchPlane;

const R: &str = "fillet_r";
const R_VALUE: f64 = 0.5;

fn lit(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite literal")
}

fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).expect("finite angle literal")
}

fn radius() -> Expr {
    Expr::param(ParamName::new(R), Dimension::Length)
}

/// A four-corner diamond, every corner filleted at the SHARED
/// parameter — so one `SetDocParam` moves four fillet resolutions and
/// eight fit-sign decisions at once.
fn bracket_profile() -> ProfileProgram {
    use std::f64::consts::{FRAC_PI_2, PI};
    ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At([lit(1.5), lit(0.0)]),
            ProgramStep::Angle(ang(0.0)),
            ProgramStep::Fillet(radius()),
            ProgramStep::At([lit(3.0), lit(1.5)]),
            ProgramStep::Angle(ang(FRAC_PI_2)),
            ProgramStep::Fillet(radius()),
            ProgramStep::At([lit(1.5), lit(3.0)]),
            ProgramStep::Angle(ang(PI)),
            ProgramStep::Fillet(radius()),
            ProgramStep::At([lit(0.0), lit(1.5)]),
            ProgramStep::Angle(ang(-FRAC_PI_2)),
            ProgramStep::Fillet(radius()),
            ProgramStep::CloseTo,
        ])],
    }
}

/// The document: the parametric bracket, extruded.
fn document() -> (Doc<ProfileProgram>, editor_core::RecipeNodeId) {
    let mut doc =
        Doc::<ProfileProgram>::empty(editor_core::DocumentId::derive("r2-e2e"), Tol::witness());
    let mut edit = |doc: &Doc<ProfileProgram>, e: DocEdit<ProfileProgram>| {
        apply(doc, &e, Tol::witness()).expect("the authoring door accepts")
    };
    doc = edit(
        &doc,
        DocEdit::SetDocParam {
            name: ParamName::new(R),
            value: DocParam::continuous(Dimension::Length, R_VALUE),
        },
    )
    .doc;
    let applied = edit(
        &doc,
        DocEdit::InsertNode {
            node: Node::Profile(bracket_profile()),
        },
    );
    let pid = applied.record.minted.expect("an id came back");
    doc = applied.doc;
    doc = edit(
        &doc,
        DocEdit::InsertNode {
            node: Node::Extrude {
                profile: pid,
                distance: lit(0.5),
            },
        },
    )
    .doc;
    (doc, pid)
}

fn opts(lift: ProfileLift) -> EvalOptions {
    EvalOptions {
        profile_lift: lift,
        ..EvalOptions::default()
    }
}

/// **The exercise.** What a consumer can actually reach.
#[cfg(feature = "interval")]
#[test]
fn r2_e2e_a_parametric_fillet_document_under_the_lift() {
    use geom_core::{Bounds, Interval};
    let (doc, pid) = document();

    // 1. f64, both settings. The fence's own claim, on a document with
    //    a parametric FILLET rather than a parametric circle.
    for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
        let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &opts(lift), Tol::witness());
        let bad: Vec<String> = ev
            .nodes
            .values()
            .filter_map(|r| r.error().map(ToString::to_string))
            .collect();
        println!(
            "R2-E2E f64 {lift:?}: {} nodes, errors={bad:?}",
            ev.nodes.len()
        );
    }

    // 2. Interval, both settings — and the WIDTH of what comes back,
    //    which is the whole point of turning the lift on.
    let widths = |lift| -> (usize, f64) {
        let ev = evaluate::<Interval>(&doc, None, &CancelToken::new(), &opts(lift), Tol::witness());
        let mut n = 0;
        let mut worst = 0.0_f64;
        for r in ev.nodes.values() {
            if let NodeResult::Ok(v) = r
                && let ValuePayload::Body(b) = &v.payload
            {
                for (_, p) in b.points() {
                    n += 1;
                    for c in [p.x, p.y, p.z] {
                        worst = worst.max(c.hi() - c.lo());
                    }
                }
            }
        }
        (n, worst)
    };
    let pinned = widths(ProfileLift::Pinned);
    let guided = widths(ProfileLift::Guided);
    println!(
        "R2-E2E interval: pinned {} pts worst width {:e}; guided {} pts worst width {:e}",
        pinned.0, pinned.1, guided.0, guided.1
    );

    // 3. Can a consumer put a WIDE parameter into a document evaluation
    //    at all? `Doc::param_env` embeds every DocParam through
    //    `from_f64`, and `evaluate` takes no environment — so the
    //    answer is recorded here rather than argued.
    let env = doc.param_env::<Interval>();
    for (name, v) in &env.bindings {
        if let editor_core::ParamValue::Continuous { value, .. } = v {
            println!(
                "R2-E2E document env: {name:?} = [{:e}, {:e}] (width {:e})",
                value.lo(),
                value.hi(),
                value.hi() - value.lo()
            );
        }
    }

    // 4. The widest box a consumer CAN drive: hand-assembling the
    //    ladder from the public profile doors. This is the friction
    //    the charter asks about — five public calls and a hand-built
    //    `SketchPlane<Interval>` to do what `evaluate` will not.
    let Some(Node::Profile(program)) = doc.node(pid) else {
        panic!("the profile node is a profile node")
    };
    let nominal = program
        .resolve(&doc.param_env::<f64>())
        .expect("the nominal resolves");
    let mut records = Vec::new();
    for steps in &nominal {
        let (_, rec) = profile::replay_recording(steps, Tol::witness()).expect("records");
        records.push(rec);
    }
    for (lo, hi) in [
        (R_VALUE, R_VALUE),
        (0.49, 0.51),
        (0.4, 0.6),
        (0.05, 1.0),
        (1e-6, 1.45),
    ] {
        let mut env = doc.param_env::<Interval>();
        env.bindings.insert(
            ParamName::new(R),
            editor_core::ParamValue::Continuous {
                dim: Dimension::Length,
                value: Interval::from_bounds(lo, hi),
            },
        );
        let resolved = match program.resolve(&env) {
            Ok(r) => r,
            Err((slot, e)) => {
                println!("R2-E2E box [{lo:e},{hi:e}]: resolve refused at {slot:?}: {e}");
                continue;
            }
        };
        let mut loops = Vec::new();
        let mut wall = None;
        for (li, steps) in resolved.iter().enumerate() {
            match profile::replay_guided(steps, &records[li], Tol::witness()) {
                Ok(lp) => loops.push(lp),
                Err(e) => {
                    wall = Some(format!(
                        "guided REPLAY loop {li} step {}: {:?}",
                        e.step, e.kind
                    ));
                    break;
                }
            }
        }
        if let Some(w) = wall {
            println!("R2-E2E box [{lo:e},{hi:e}]: {w}");
            continue;
        }
        println!(
            "R2-E2E box [{lo:e},{hi:e}]: guided replay CERTIFIED, {} loops",
            loops.len()
        );
    }
}

/// The `Dual64` half: does the lift run through `evaluate` at a dual,
/// and does anything actually carry a tangent?
#[test]
fn r2_e2e_dual_through_the_evaluation_door() {
    use geom_core::Dual64;
    let (doc, _) = document();
    for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
        let ev = evaluate::<Dual64>(&doc, None, &CancelToken::new(), &opts(lift), Tol::witness());
        let mut pts = 0;
        let mut nonzero_tangent = 0;
        let mut errs = Vec::new();
        for r in ev.nodes.values() {
            if let Some(e) = r.error() {
                errs.push(e.to_string());
            }
            if let NodeResult::Ok(v) = r
                && let ValuePayload::Body(b) = &v.payload
            {
                for (_, p) in b.points() {
                    pts += 1;
                    for c in [p.x, p.y, p.z] {
                        if c.deriv != 0.0 {
                            nonzero_tangent += 1;
                        }
                    }
                }
            }
        }
        println!(
            "R2-E2E dual {lift:?}: {pts} body points, {nonzero_tangent} nonzero tangent \
             coordinates, errors={errs:?}"
        );
    }
}

/// **The two-ladder consequence, measured.** `wire_profile`'s payload
/// under the lift is the `T`-valued validated profile; `section_of`'s
/// is the f64 replay, with the second pass run only as a GATE. So a
/// LOFT's geometry cannot widen under the lift, while an EXTRUDE's
/// does. Both are claimed as "parity"; this row records the
/// difference a consumer sees.
#[cfg(feature = "interval")]
#[test]
fn r2_e2e_the_loft_ladder_does_not_widen_where_the_extrude_ladder_does() {
    use geom_core::{Bounds, Interval};
    let worst = |doc: &Doc<ProfileProgram>, lift| -> f64 {
        let ev = evaluate::<Interval>(doc, None, &CancelToken::new(), &opts(lift), Tol::witness());
        let mut w = 0.0_f64;
        for r in ev.nodes.values() {
            if let NodeResult::Ok(v) = r
                && let ValuePayload::Body(b) = &v.payload
            {
                for (_, p) in b.points() {
                    for c in [p.x, p.y, p.z] {
                        w = w.max(c.hi() - c.lo());
                    }
                }
            }
        }
        w
    };
    let (bracket, _) = document();
    println!(
        "R2-E2E extrude ladder: pinned {:e} -> guided {:e}",
        worst(&bracket, ProfileLift::Pinned),
        worst(&bracket, ProfileLift::Guided)
    );
    let loft = corpus::loft_prism::document();
    println!(
        "R2-E2E loft ladder:    pinned {:e} -> guided {:e}",
        worst(&loft.doc, ProfileLift::Pinned),
        worst(&loft.doc, ProfileLift::Guided)
    );
}
