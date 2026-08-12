// Flux probe 1: the NURBS span-indexing pattern from
// crates/geom-curves/src/nurbs.rs::eval_in_span, whose index safety is
// today justified by a PROSE comment:
//   "Indexing justified: span valid => i = span - p + j
//    in [span - p, span] subset of [0, control_count)."
#![allow(unused)]
extern crate flux_rs;
use flux_rs::attrs::*;

#[spec(fn(weights: &[f64][@n], p: usize, span: usize{p <= span && span < n}) -> f64)]
pub fn eval_in_span(weights: &[f64], p: usize, span: usize) -> f64 {
    let mut acc = 0.0;
    let mut j = 0;
    while j <= p {
        let i = span - p + j; // must not underflow, must be < n
        acc += weights[i];
        j += 1;
    }
    acc
}
