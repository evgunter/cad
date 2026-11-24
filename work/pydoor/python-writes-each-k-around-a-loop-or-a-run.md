---
id: python-writes-each-k-around-a-loop-or-a-run
kind: unit
title: Python writes each k around a loop, a run or a ring
status: parked
opened: 2026-11-22
priority: P1
cost: M
parent: a-loop-cannot-repeat-inside-a-profile
blocked_on: [a-ring-closes-each-k-from-its-start-point]
---


The Python half of `work/repeat/a-loop-cannot-repeat-inside-a-profile.md`
(Ev, PR 7263). It is parked on REPEAT's ring unit, the last of the
three forms.

## Done when

- The Python door writes `each k { loop }` in a profile's loop list,
  `each k { steps }` inside a chain, and the ring
  `at(P[0]); each k from P[k] { …, to Next }`, with `k` a document
  index the caller declared. `pncad.pyi` carries the forms, and the
  binding census (`surface_census.rs`) witnesses them.
- A test builds the 6-hole plate and a ring from Python and reads back
  one profile, not a family.

## Check first: Python builds kernel paths, not programs

Python's loops are built on the kernel `profile::Path`
(`crates/pncad-py/src/py/path.rs`, `ClosedLoop`) and lifted to the
document by `LoopProgram::from_recorded`. A kernel path has no blocks:
a run built there is already N copies, and the lift would write them
out. So the door needs a builder at the document layer
(`crates/pncad/src/trace.rs` `Chain` is the existing one), or a way to
mark a recorded run as one written block. Which one is a façade design
choice. If it is not obvious from the façade's existing doctrine,
stop and ask rather than pick one.

## Ordering

PR 7251's build removes `tangent_arc_to` and `continue_to` from the
same bindings (`py/path.rs`, `py/trace.rs`, `pncad.pyi`). Build on
whichever vocabulary is on main, and use neither deleted verb in the
new tests.
