---
id: graft-stages-into-a-fresh-body-and-commits-on-success
kind: issue
title: instance's graft leaves a refused destination partly written; it stages into a fresh body and commits only on success (S14, Ev, PR 4006)
status: open
opened: 2026-10-04
priority: P1
cost: M
---


(TOPO orchestrator, from Ev's ruling on PR 4006, 2026-10-04: "closing s14
and making this panic is the right end state".)

## What

`topo::instance`'s graft (`graft_disjoint_all_keyed` and its siblings,
`crates/topo/src/instance.rs`, docs around :149) writes the destination as
it goes. A `JoinDesync` raised mid-transplant leaves `dst` "partially
written … spent, never resumable" (an empty destination solid is
`SolidWithoutShells`, a tier-1 error). A caller that discards the `Err` and
keeps the body holds a tier-1-invalid body through its own misuse. This is
the one door outside D9's "every public mutation path preserves tier 1".

## Build

Stage the transplant into a fresh body and commit it into `dst` only on
success, the shape `merge_coplanar_faces` already uses (D2 row 0: make the
partly written state unrepresentable). Every `Err` leaves `dst`
deep-unchanged. Retire the "spent, never resumable" docs and any caller code
that relies on it. `docs/DESIGN.md` D9 already states the ruled property.

## Why first

`stale-key-and-not-same-edge-answer-for-a-callers-key-and-a-torn-body`
converts every torn-body refusal into `unreachable!` (row 4). Until the
graft stages, a caller who keeps a refused graft destination would trip
that panic through misuse rather than a kernel bug.
