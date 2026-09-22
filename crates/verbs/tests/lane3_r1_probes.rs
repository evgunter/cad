//! **What the seat's general block now admits at a dual**, stated as
//! a compiler fact: `Verb::<Dual64>::run_shell` is a well-typed
//! function item (the base's second `impl` block made it an `E0599`),
//! and it is uncallable for want of its fourth argument — no route in
//! safe code produces a `ShellDoor<Dual64>` (the constructor's bound,
//! no `Default`, no `From`, a private field, no cross-scalar copy).
//! The capability the second block withheld is therefore withheld one
//! step later, at the value, and this row is where that is written
//! down rather than argued.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Dual64, Tol};
use topo::{Body, ShellDoor};
use verbs::{Verb, VerbError, VerbOut};

#[test]
fn run_shell_resolves_at_a_dual_and_wants_a_door_no_dual_can_hold() {
    let _f: fn(
        &Verb<Dual64>,
        &Body<Dual64>,
        Tol,
        ShellDoor<Dual64>,
    ) -> Result<VerbOut<Dual64>, VerbError<Dual64>> = Verb::<Dual64>::run_shell;
    assert!(
        <Dual64 as topo::AtRestPolicy>::shell_door().is_none(),
        "the one seam that answers the door answers None at a dual, so the item above has \
         no fourth argument anyone can pass"
    );
}
