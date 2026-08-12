// Flux probe 2: NEGATIVE CONTROL — the same body with `p <= span`
// dropped from the precondition. Must be rejected.
#![allow(unused)]
extern crate flux_rs;
use flux_rs::attrs::*;

#[spec(fn(weights: &[f64][@n], p: usize, span: usize{span < n}) -> f64)]
pub fn eval_in_span(weights: &[f64], p: usize, span: usize) -> f64 {
    let mut acc = 0.0;
    let mut j = 0;
    while j <= p {
        let i = span - p + j;
        acc += weights[i];
        j += 1;
    }
    acc
}
