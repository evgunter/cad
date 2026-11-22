//! **What the seat's general block admits at a dual**, as a compiler
//! fact: `Shell::<Dual64>::run_shell` is a well-typed function item
//! whose fourth argument is a type with no values, so no call of it
//! can be written.
//!
//! The door a scalar holds is [`topo::AtRestPolicy::ShellDoor`]:
//! `topo::ShellDoor` where the scalar certifies, and at a dual
//! [`geom_core::NoLane`]. `topo::ShellDoor<Dual64>` is not a type at
//! all (the `compile_fail` doctest on `topo::ShellDoor`).

use geom_core::{Dual64, NoLane, Tol};
use verbs::{Shell, VerbError, VerbOut};

/// The seat's shell door at a dual, as a function item.
type RunShellAtDual = fn(
    &Shell<Dual64>,
    &topo::Replay<Dual64>,
    Tol,
    NoLane,
) -> Result<VerbOut<Dual64>, VerbError<Dual64>>;

/// The red is a compile error: the coercion holds only while the
/// fourth argument is the uninhabited type.
#[test]
fn run_shell_resolves_at_a_dual_and_wants_a_door_with_no_values() {
    let _f: RunShellAtDual = Shell::<Dual64>::run_shell;
}
