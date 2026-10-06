---
id: a-pinch-face-tessellation-witness-refuses-bool-join-nearest-at-eps-1e-6
kind: issue
title: "pinch_faces_tessellate refuses Escalated(bool_join_nearest) at eps 1e-6: notch307 fib117 edge psi=1.9 cp S"
status: open
opened: 2026-10-06
priority: P1
cost: M
refs: [a-planar-face-through-two-vertices-on-one-point-meshes-under-one-id, a-bar-through-a-ball-refuses-at-a-door-that-moves-with-scale]
---

## What

`sweep::all pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates`
(`crates/sweep/tests/pinch_faces_tessellate.rs`, its assertion that the
op builds a body) fails at `CAD_TOLERANCE_EPS=1e-6`. It passes at the
default eps and at 1e-12. The case is `notch307 fib117 edge psi=1.9 cp S`:

```
the op builds no body: Err(Escalated { decision: Coincidence(Join, Moot),
  diag: Indeterminate { margin: MarginDiag(Value(-5.196042020649827e-6, None)),
  band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 },
  predicate: Some("bool_join_nearest") } })
```

Measured on `fuse/one-arc-struts` (PR 3953's head, 6dfbb2cdf), with
main's `boolean/insert.rs`, `boolean/mod.rs` and `lib.rs` swapped in:
the same failure, so it is main's. Main's push CI doesn't run that
crate's eps rows. PR 3953's tier-`all` run does, and its `gate ok` is
red on this test alone (the workspace `ci` profile at 1e-6 locally:
11925 passed, this one failed).

The test last changed in `f1f230f19`. The pinch-tessellation row it
pins (`a-planar-face-through-two-vertices-on-one-point-meshes-under-one-id`,
PR 4074) is closed.

## Lead

`bool_join_nearest` ranks facing germs by chord length in absolute
metres (`a-bar-through-a-ball-refuses-at-a-door-that-moves-with-scale`).
The margin here, 5.2e-6, falls inside the 1e-6 row's escalation band
[1e-6, 1e-5). Not verified as that row's mechanism.

## Owed

Build the case at 1e-6, or refuse it typed and pin it at that row.
