---
id: a-driver-carried-operation-refusal-shows-the-user-the-drivers-recourse
kind: issue
title: A driver-carried operation refusal shows the user the driver's recourse
status: open
opened: 2026-10-04
---

(Found by PR 4029's review, finding 2.)

## What

A kernel driver (extrude, revolve, loft, blend, the split and Boolean
stages, the shell op) wraps the refusal an Euler operator gave it and
forwards it, verbatim, to the person at the GUI. Since
`EulerOpError::from_driver` (`crates/topo/src/euler.rs`, `from_driver`)
panics on an argument miss, what a driver forwards is an *operation*
refusal, and an operation refusal states its own recourse. That
recourse is addressed to whoever called the operator, which here is
the driver, not the person:

- `EulerOpError::DescriptionNotAdjacent`'s text ends "Recourse:
  describe it against the surfaces its faces wear"
  (`crates/topo/src/euler.rs:1272`, `EulerOpError::render`). Nobody
  at the GUI described the edge; the driver did.
- `ShellError::Partition` forwards it as "the thin solid around void
  … could not be partitioned out: {error}"
  (`crates/topo/src/shell.rs:588`, `impl Display for ShellError`), and
  `ShellError::Rim` the same way (`crates/topo/src/shell.rs:643`).
  `ShellError::Face` carries `ReplaceFaceError::Op` through a second
  level: "a face could not be offset inward: an edge beside the moved
  face could not be rebuilt: {error}", and "the moved faces could not
  be put onto their offset surface: {error}" for the re-chart
  (`crates/topo/src/replace_face.rs`, `impl Display for
  ReplaceFaceError`). The carried text is the operator's, "the door"
  and the "(D2 adjacency coherence)" tag included.
- `ExtrudeError::Op` forwards it as "an Euler operation refused:
  {source}" (`crates/sweep/src/extrude.rs:547`), and `BlendError::Op`,
  `RevolveError::Op`, `LoftError::Euler`, `SplitJoinError::Euler`,
  `SplitReduceError::{Euler, CrossingInsertion}`, `SplitFinishError::Euler`
  and `BooleanError::Euler` likewise.

`crates/editor-core/tests/refusal_concision_chains.rs:643`
(`DRIVER_CARRIED`, pinned by
`every_driver_carried_chain_ends_in_the_operators_own_recourse`) lists
the thirteen chains and asserts each ends in the operator's own
recourse; that row goes red when this one is fixed, and is updated
with it.

## The shape of a repair

A driver-carried operation refusal is a fact about what the driver
tried, so the recourse the person reads is the driver's, not the
operator's: either the wrapper replaces the carried recourse with one
addressed to the feature (often "There is no way through" plus the
kernel-bug report, where the driver's own construction should have
satisfied the operator), or `EulerOpError` renders without its
recourse under a driver's reading (`Reading`), and the wrapper states
its own. Which, per driver, is the unit's design question.
