//! **The shell door at the symbolic tier** — the one certifying arm
//! of `AtRestPolicy::shell_door` that `at_rest_policy_tests` does not
//! instantiate (its roster is `f64`, `Probe`, `Interval`), pinned
//! through the public seam: a `Sym<f64>` answers `Some`, so the
//! driver's leaf replay can hollow the bodies it certifies. With the
//! `Sym` arm flipped to `None` and nothing else changed, nothing in
//! the tree noticed; this row does.
//!
//! The pointer itself is private to `props.rs`, so this row pins
//! presence and not identity — identity at `Sym` is the roster's to
//! add, and the census row filed on SCALAR names the gap.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Dual64, Sym};
use topo::AtRestPolicy;

#[test]
fn the_symbolic_tier_over_f64_holds_the_shell_door() {
    assert!(
        <Sym<f64> as AtRestPolicy>::shell_door().is_some(),
        "Sym<f64> wraps a certifying base, so its policy arm must hand out the shell door"
    );
}

/// The same seam, at the two ends the roster already pins — here so
/// the three answers sit in one row a reader can compare.
#[test]
fn the_seam_answers_the_door_at_f64_and_none_at_a_dual() {
    assert!(<f64 as AtRestPolicy>::shell_door().is_some());
    assert!(<Dual64 as AtRestPolicy>::shell_door().is_none());
}
