//! **A bore taken alone is smooth at its seams, at every scalar**
//! (ONEHOME fork 259; check 4, `topo::validate`'s `one_surface_seam`).
//!
//! `r2_rounded_pad`'s bore is a circle of the parameter `bore_r`,
//! extruded. Its two seams are edges whose sides lie on one surface
//! record, one sense: interior to the cylinder, smooth by identity. The
//! whole-edge fold read that at `f64` from an exact zero sagitta, but over
//! the box the two evaluations of one surface do not cancel: it read the
//! zero as an enclosure `[0, 1.8e-9]` in band and refused both seams
//! (`SliverDihedral { check: SecondOrder }`, undecided), measured when
//! fork 259 finished a moved leaf alone at the box scalars. Check 4 now
//! reads such a seam by its record before any margin.
//!
//! The pin takes every leaf of the pad that holds such a seam alone,
//! through the one taker door (`Held::finished`, `topo::take`), at `f64`,
//! at `Interval`, and over the box at `Sym<Interval>`, and the pad
//! evaluates whole at each.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{CancelToken, EvalOptions, Evaluation, NodeResult, ProfileDoc, evaluate};
use geom_core::{Interval, Sym, SymRules, Tol};

/// The pad at its certifying width (`m10_9_pins_interval`'s
/// `r2_rounded_pad` study).
fn pad(tol: Tol) -> ProfileDoc {
    crate::m10_8_r2_probes_interval::pad(3.151e3 * tol.eps(), tol).0
}

/// Every leaf the evaluation holds that has an edge whose two sides are
/// one surface record, each taken alone: the refusals, by node.
fn seam_leaves_taken_alone<T: geom_core::Decide + topo::AtRestPolicy>(
    ev: &Evaluation<T>,
    tol: Tol,
) -> (usize, Vec<String>) {
    let mut taken = 0;
    let mut refused = Vec::new();
    for id in &ev.order {
        let Some(NodeResult::Ok(v)) = ev.result(*id) else {
            continue;
        };
        let Ok(framed) = v.payload.output_body(0) else {
            continue;
        };
        let held = framed.holding();
        if !matches!(held, editor_core::Held::Leaf(_)) {
            continue;
        }
        let body = framed.body();
        let seamed = body.edges().any(|(edge, _)| {
            topo::readback::edge_sides(body, edge)
                .is_ok_and(|sides| sides.plus.surface == sides.minus.surface)
        });
        if !seamed {
            continue;
        }
        taken += 1;
        if let Err(e) = held.finished(tol) {
            refused.push(format!("node {}: {e}", id.0));
        }
    }
    (taken, refused)
}

/// The first node an evaluation refused, as its sentence.
fn first_refusal<T: geom_core::Decide + topo::AtRestPolicy>(ev: &Evaluation<T>) -> Option<String>
where
    Evaluation<T>: Sized,
{
    ev.order.iter().find_map(|id| match ev.result(*id) {
        Some(NodeResult::Failed(e)) => Some(format!("node {}: {} — {:?}", id.0, e.kind, e.kind)),
        _ => None,
    })
}

#[test]
fn the_pads_bore_finishes_alone_at_f64() {
    let tol = Tol::witness();
    let ev: Evaluation<f64> = evaluate(
        &pad(tol),
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    assert_eq!(first_refusal(&ev), None, "the pad evaluates whole at f64");
    let (taken, refused) = seam_leaves_taken_alone(&ev, tol);
    assert!(taken > 0, "the pad holds a seamed leaf at f64");
    assert!(
        refused.is_empty(),
        "a seamed leaf taken alone refuses at f64: {refused:?}"
    );
}

#[test]
fn the_pads_bore_finishes_alone_at_interval() {
    let tol = Tol::witness();
    let ev: Evaluation<Interval> = evaluate(
        &pad(tol),
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    assert_eq!(
        first_refusal(&ev),
        None,
        "the pad evaluates whole at Interval"
    );
    let (taken, refused) = seam_leaves_taken_alone(&ev, tol);
    assert!(taken > 0, "the pad holds a seamed leaf at Interval");
    assert!(
        refused.is_empty(),
        "a seamed leaf taken alone refuses at Interval: {refused:?}"
    );
}

#[test]
fn the_pads_bore_finishes_alone_over_the_box_at_sym_interval() {
    let tol = Tol::witness();
    let doc = pad(tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    };
    let budget = geom_core::SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let refused = geom_core::sym::with_session_rules(budget, SymRules::shipped(), || {
        let ev: Evaluation<Sym<Interval>> = evaluate(&doc, None, &CancelToken::new(), &opts, tol);
        (first_refusal(&ev), seam_leaves_taken_alone(&ev, tol))
    })
    .0;
    let (refused, (taken, leaves_refused)) = refused;
    assert_eq!(
        refused, None,
        "the pad evaluates whole over the box at Sym<Interval>"
    );
    assert!(taken > 0, "the pad holds a seamed leaf at Sym<Interval>");
    assert!(
        leaves_refused.is_empty(),
        "a seamed leaf taken alone refuses over the box at Sym<Interval>: {leaves_refused:?}"
    );
}
