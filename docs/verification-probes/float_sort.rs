// Flux probe 3: can a refinement say anything about an f64 VALUE?
// This is the whole question for the interval crate.
#![allow(unused)]
extern crate flux_rs;
use flux_rs::attrs::*;

#[spec(fn(x: f64) -> f64{v: v >= x})]
pub fn pad_up(x: f64) -> f64 {
    x.next_up()
}
