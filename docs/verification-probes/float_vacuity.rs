// Is the f64 refinement real, or vacuous? `id` returns exactly x, so a
// spec claiming the result is STRICTLY greater must be rejected by any
// non-vacuous checker.
#![allow(unused)]
extern crate flux_rs;
use flux_rs::attrs::*;

#[spec(fn(x: f64) -> f64{v: v > x})]
pub fn id(x: f64) -> f64 {
    x
}
