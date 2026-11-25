//! The runner's speed reference (`speed_reference`), as the one ignored
//! test `scripts/fast_set_alone.py` times alone in every re-time pass.
//!
//! It asserts nothing, so it is `#[ignore]`d: no suite runs it, and only
//! the re-time passes ask for it, by name and with `--run-ignored all`.
//! FROZEN, like the library: an edit here moves the nominal time.

use std::hint::black_box;
use std::ops::{Add, Div, Mul, Sub};

use speed_reference::Scalar;

/// A scalar defined here, so the generic half is built in this binary,
/// as the kernel's generic code at the suites' scalars is.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Local(f64);

impl Add for Local {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Local(self.0 + o.0)
    }
}

impl Sub for Local {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Local(self.0 - o.0)
    }
}

impl Mul for Local {
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        Local(self.0 * o.0)
    }
}

impl Div for Local {
    type Output = Self;
    fn div(self, o: Self) -> Self {
        Local(self.0 / o.0)
    }
}

impl Scalar for Local {
    fn of(x: f64) -> Self {
        Local(x)
    }
    fn get(self) -> f64 {
        self.0
    }
}

const SHARED_ROUNDS: u32 = 23_000;
const LOCAL_ROUNDS: u32 = 2_000;

#[test]
#[ignore = "the runner's speed reference: scripts/fast_set_alone.py times it alone; it asserts nothing"]
fn the_runners_speed_reference() {
    black_box(speed_reference::shared(black_box(SHARED_ROUNDS)));
    black_box(speed_reference::run::<Local>(black_box(LOCAL_ROUNDS)));
}
