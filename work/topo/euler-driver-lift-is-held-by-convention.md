---
id: euler-driver-lift-is-held-by-convention
kind: issue
title: A driver's Euler refusal reaches its error through from_driver by convention: eight Euler variants are constructible bare, and edge_join's join_one bypassed it
status: open
opened: 2026-10-13
priority: P3
cost: M
---

Found by the sweep of `fuse/join-refusals-hold-at-their-type`, which
made the Boolean's chord-join and merge lifts the only constructors of
`BooleanError::Join` and `BooleanError::Merge`.

`EulerOpError::from_driver` (`crates/topo/src/euler.rs`) is the lift a
kernel driver applies to an operator's refusal: it panics on
`EulerOpError::Argument` (a driver's own key that does not resolve is a
kernel bug) and passes every other refusal through. Each driver error
applies it in its `From<EulerOpError>`:

- `BooleanError::Euler` (`crates/topo/src/boolean/mod.rs`)
- `SplitJoinError::Euler` (`crates/topo/src/chord_join.rs`)
- `SplitReduceError::Euler` (`crates/topo/src/splitting/mod.rs`)
- `SplitFinishError::Euler` (`crates/topo/src/splitting/finish.rs`)
- `MergeCoplanarError::Op` (`crates/topo/src/merge_faces.rs`)
- `LoftError::Euler`, `ExtrudeError::Op`, `RevolveError::Op`
  (`crates/sweep/src/{loft,extrude,revolve/mod}.rs`)

and `BooleanError::CrossingInsertion { source }` and
`SplitReduceError::CrossingInsertion` apply it by hand at their one
site (`source: source.from_driver()`). Each variant holds a bare
`EulerOpError`, so
`.map_err(BooleanError::Euler)` compiles and skips the panic: an
argument miss then leaves the kernel as a typed refusal. PR 4029 put
`from_driver` at every driver wrap; `edge_join::join_one` (written
after) spelled it `.map_err(BooleanError::Euler)` twice, which the
sweep's PR turned into `?`. Nothing stops the next one.

What is owed: the wrong spelling unconstructible, as `BooleanJoin` /
`BooleanMerge` do for their lifts. One shape: a `DriverRefusal`
newtype in `euler.rs` whose field is private and whose only
constructor is `From<EulerOpError>` (applying `from_driver`), held by
each driver's `Euler`/`Op` variant, so `?` is the one spelling and a
bare variant around an `EulerOpError` does not type-check. Readers
match through an accessor.

**The panicking half is `pub` too** (ENCL's total-render sweep,
`work/encl/a-hand-built-unminted-refusal-panics-through-pub-render.md`,
2026-10-19). `EulerOpError::from_driver` (`crates/topo/src/euler.rs`)
is `pub`, and so is each `From<EulerOpError>` above, so
`BooleanError::from(EulerOpError::Argument(..))` (or
`EulerOpError::Argument(..).from_driver()`) panics on a value a caller
built. `[failure]` (D2) makes that input. A `DriverRefusal` whose
constructor is `pub(crate)` closes both: the lift stays the one
spelling inside topo, and no caller outside reaches the panic.
