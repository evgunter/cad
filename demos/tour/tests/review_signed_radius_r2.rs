//! Reviewer probe (signed radius, r2), claim 9: the cut plate the
//! boxed evaluation builds holds the bodies its members build. The
//! study box is evaluated at `Sym<Interval>`; each of nine members
//! (the box's corners and its centre) is evaluated over the degenerate
//! box at that member, at `Interval`. Every member vertex must lie in
//! some vertex enclosure of the boxed body, and the two censuses agree.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use demo_tour::plate::{RADIUS_SIGMA, SPACING_HALF_WIDTH, WEB_BOUND, cut_plate};
use pncad::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use pncad::document::{
    BooleanValue, CancelToken, EvalOptions, Evaluation, Node, NodeResult, ValuePayload, evaluate,
};
use pncad::geom_core::{Bounds, Decide, Interval, Point3, Sym, SymBudget, SymRules, Tol};

fn last_cut<T: pncad::topo::AtRestPolicy + Decide>(
    doc: &pncad::document::ProfileDoc,
    ev: &Evaluation<T>,
) -> Vec<Point3<T>>
where
    T: pncad::geom_core::Real,
{
    let id = ev
        .order
        .iter()
        .copied()
        .filter(|id| matches!(doc.node(*id), Some(Node::Subtract { .. })))
        .last()
        .expect("a subtract");
    let body = match ev.result(id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Body(b) => b.body().clone(),
            ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body.body().clone(),
            other => panic!("not a body: {other:?}"),
        },
        other => panic!("the last subtract did not build: {other:?}"),
    };
    body.vertex_points().map(|(_, p)| p).collect()
}

#[test]
fn review_r2_the_boxed_cut_plate_holds_its_members() {
    let tol = Tol::witness();
    let p = cut_plate(SPACING_HALF_WIDTH, RADIUS_SIGMA, WEB_BOUND, tol);
    let analyzed = analyzed_box(&p.doc, &AnalysisPolicy::default());
    let whole = ParamBox::of(&analyzed);
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let boxed: Vec<[(f64, f64); 3]> =
        pncad::geom_core::sym::with_session_rules(budget, SymRules::shipped(), || {
            let opts = EvalOptions {
                param_box: Some(Arc::new(whole.clone())),
                ..EvalOptions::default()
            };
            let ev: Evaluation<Sym<Interval>> =
                evaluate(&p.doc, None, &CancelToken::new(), &opts, tol);
            last_cut(&p.doc, &ev)
                .into_iter()
                .map(|q| {
                    [
                        (q.x.lo(), q.x.hi()),
                        (q.y.lo(), q.y.hi()),
                        (q.z.lo(), q.z.hi()),
                    ]
                })
                .collect()
        })
        .0;
    let varying: Vec<_> = whole
        .axes()
        .iter()
        .filter_map(|(id, a)| match a {
            BoxAxis::Varying { lo, hi } => Some((*id, *lo, *hi)),
            BoxAxis::Fixed => None,
        })
        .collect();
    let widest = boxed
        .iter()
        .flat_map(|b| b.iter().map(|(lo, hi)| hi - lo))
        .fold(0.0_f64, f64::max);
    println!(
        "{} vertices boxed; {} varying axes; widest enclosure {widest:e}",
        boxed.len(),
        varying.len()
    );
    assert!(widest < 1e-3, "the enclosures are not vacuous: {widest:e}");
    let n = varying.len();
    let mut members: Vec<Vec<f64>> = (0..(1 << n))
        .map(|bits| {
            varying
                .iter()
                .enumerate()
                .map(|(k, (_, lo, hi))| if bits >> k & 1 == 1 { *hi } else { *lo })
                .collect()
        })
        .collect();
    members.push(varying.iter().map(|(_, lo, hi)| 0.5 * (lo + hi)).collect());
    let mut worst = 0.0_f64;
    for m in members {
        let axes: BTreeMap<_, _> = whole
            .axes()
            .iter()
            .map(|(id, a)| {
                let axis = match varying.iter().position(|(v, _, _)| v == id) {
                    Some(k) => BoxAxis::Varying { lo: m[k], hi: m[k] },
                    None => *a,
                };
                (*id, axis)
            })
            .collect();
        let opts = EvalOptions {
            param_box: Some(Arc::new(ParamBox::from_axes_in(axes, whole.order()))),
            ..EvalOptions::default()
        };
        let ev: Evaluation<Interval> = evaluate(&p.doc, None, &CancelToken::new(), &opts, tol);
        let pts = last_cut(&p.doc, &ev);
        assert_eq!(pts.len(), boxed.len(), "member {m:?}: the vertex census");
        for q in pts {
            let m2 = |v: Interval| 0.5 * (v.lo() + v.hi());
            let c = [m2(q.x), m2(q.y), m2(q.z)];
            let held = boxed.iter().any(|b| {
                b.iter()
                    .zip(c)
                    .all(|((lo, hi), x)| *lo - 1e-12 <= x && x <= *hi + 1e-12)
            });
            let near = boxed
                .iter()
                .map(|b| {
                    b.iter()
                        .zip(c)
                        .map(|((lo, hi), x)| (lo - x).max(x - hi).max(0.0))
                        .fold(0.0_f64, f64::max)
                })
                .fold(f64::INFINITY, f64::min);
            worst = worst.max(near);
            assert!(
                held,
                "member {m:?}: vertex {c:?} lies in no boxed enclosure (nearest {near:e})"
            );
        }
    }
    println!("every member vertex held; worst distance outside an enclosure {worst:e}");
}
