---
id: ring-path-spline-crossings-read-a-meet-fault-as-saying-nothing
kind: issue
title: ring_path: spline_crossings reads a meeting's limit, missing pose and kernel fault alike as a path that says nothing
status: dispatched
opened: 2026-10-29
priority: P1
cost: E
---


Found by the §5 sweep of `splitdoor/roots-unsettled-split`.

`crates/topo/src/ring_path.rs` `spline_crossings` reads a refused
`meet` as `MeetFault::Escalated(diag) => Err(diag)` and every other
fault, `Touch(_)`, `Unsettled(_)`, `Unsupported`, `Desync` and
`Contradiction`, as `Ok(None)`: the path says nothing, and the caller moves on. A touch in band
(the path is the kernel's probe), a limit or a missing pose may
honestly say nothing. A `Desync` (a carrier of a
kind the arm does not read) or a `Contradiction` (two readings that
contradict, now including a root finder's `RootFault`) is the
kernel's defect, and it is dropped silently.

The repair is a kernel-defect ending for those two, with a row; the
germtail row `ring-path-reads-curve-meetings-and-one-curve-outside-meet`
is the neighbouring reader.
