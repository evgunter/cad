//! **What the seat's general block admits at a dual**, as a compiler
//! fact: `Verb::<Dual64>::run_shell` is a well-typed function item,
//! and it is uncallable for want of its fourth argument — the guard
//! is the door VALUE's provenance and not the method's signature.
//!
//! No route in safe code produces a `ShellDoor<Dual64>`. The
//! constructor is held executably, by the `compile_fail,E0599`
//! doctest on `topo::ShellDoor`; the others are closed by the
//! compiler when written out — `Default` and `.into()` from a
//! `ShellDoor<f64>` want an `impl` nobody wrote (E0277), a struct
//! literal outside `topo` cannot name the private field (E0451), the
//! `f64` door is a different type (E0308), and a helper generic in
//! plain `Decide` cannot reach the constructor's bound (E0277). What
//! remains is the one seam that hands the value out,
//! [`topo::AtRestPolicy::shell_door`], and the row below reads it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Dual64, Tol};
use topo::{Body, ShellDoor};
use verbs::{Verb, VerbError, VerbOut};

/// The seat's shell door at a dual, as a function item: naming the
/// type is the assertion, and the fourth argument is the one nobody
/// can supply.
type RunShellAtDual = fn(
    &Verb<Dual64>,
    &Body<Dual64>,
    Tol,
    ShellDoor<Dual64>,
) -> Result<VerbOut<Dual64>, VerbError<Dual64>>;

#[test]
fn run_shell_resolves_at_a_dual_and_wants_a_door_no_dual_can_hold() {
    let _f: RunShellAtDual = Verb::<Dual64>::run_shell;
    assert!(
        <Dual64 as topo::AtRestPolicy>::shell_door().is_none(),
        "the one seam that answers the door answers None at a dual, so the item above has \
         no fourth argument anyone can pass"
    );
}
