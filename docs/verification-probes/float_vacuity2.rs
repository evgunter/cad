#![allow(unused)]
extern crate flux_rs;
use flux_rs::attrs::*;

// Blatantly false: claim next_down is >= its input.
#[spec(fn(x: f64) -> f64{v: v >= x})]
pub fn pad_down(x: f64) -> f64 {
    x.next_down()
}

// And the int control, which MUST be rejected.
#[spec(fn(x: i32) -> i32{v: v > x})]
pub fn id_i32(x: i32) -> i32 {
    x
}
