//! R2 head capability probe: every route to a `ShellDoor<Dual64>`,
//! and whether the seat's shell door can be NAMED at a dual.
#![allow(dead_code, unused)]

use geom_core::{Dual64, Tol};
use topo::{AtRestPolicy, Body, ShellDoor};
use verbs::Verb;

// ROUTE 0 — the signature. At the merge base this was E0599; at the
// head it type-checks, and only the ARGUMENT is unobtainable.
fn route0_names_the_seat_door(v: &Verb<Dual64>, b: &Body<Dual64>, tol: Tol, door: ShellDoor<Dual64>) {
    let _ = v.run_shell(b, tol, door);
}

// ROUTE 1 — the constructor. Expected: E0599.
// fn route1() -> ShellDoor<Dual64> { ShellDoor::<Dual64>::certified() }

// ROUTE 2 — the policy seam, unwrapped. Compiles; `None` at run time.
fn route2() -> Option<ShellDoor<Dual64>> {
    <Dual64 as AtRestPolicy>::shell_door()
}

// ROUTE 3 — a struct literal from outside `topo`. Expected: E0451/E0560.
// fn route3() -> ShellDoor<Dual64> { ShellDoor { open: topo::shell_open::<Dual64> } }

// ROUTE 4 — `Default`. Expected: E0277/E0599.
// fn route4() -> ShellDoor<Dual64> { Default::default() }

// ROUTE 5 — `From`/`Into` from the f64 door. Expected: E0277.
// fn route5(d: ShellDoor<f64>) -> ShellDoor<Dual64> { d.into() }

// ROUTE 6 — a looser generic helper that hands one back at plain
// `Decide`. Expected: E0599 inside the helper.
// fn route6<T: geom_core::predicate::Decide>() -> ShellDoor<T> { ShellDoor::certified() }

// ROUTE 7 — Copy/Clone across scalars. Expected: E0308.
// fn route7(d: ShellDoor<f64>) -> ShellDoor<Dual64> { d }

#[test]
fn r2_the_dual_seam_answers_none_at_run_time() {
    assert!(route2().is_none(), "the dual policy arm hands out no door");
}
