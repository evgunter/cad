---
id: pinch-notch307-pierce-escalates-at-eps-1e-6
kind: issue
title: pinch_faces_tessellate's notch307 fib117 psi=1.9 pose escalates bool_join_nearest at eps 1e-6 on main
status: open
opened: 2026-10-06
priority: P1
cost: M
---

## What

`sweep::all pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates`
fails at `CAD_TOLERANCE_EPS=1e-6` on `origin/main` at 62bdd557:

```
notch307 fib117 edge psi=1.9 cp S: the op builds no body: Err(Escalated {
  decision: Coincidence(Join, Moot), diag: Indeterminate { margin:
  MarginDiag(Value(-5.196042020649827e-6, None)), band: Band { zero: 1e-6,
  escalate: 1e-5 }, predicate: Some("bool_join_nearest") } })
```

The row panics at `crates/sweep/tests/pinch_faces_tessellate.rs` (the
pose loop's "the op builds no body" assertion, around line 337).

The row passed at 1e-6 locally at `930c880a`, the tree before these
merges. Between that tree and 62bdd557, main took `443f33b7` (Merge
PR #3954, `tang/pierce-strut-at-a-pinch`) and `361a6a9f` (Merge PR
#4096, `cleave/pierce-strut-after`). Those two merges are the latest
change to the boolean's strut and pierce ground, which makes them the
suspects. The failure reproduces identically on main alone, without
the BAND branch that found it (PR #4119's CI, `test` job, 1e-6 row).

## What it wants

Bisect between the two merges. Then do one of: fix the pose's join
margin, or pin it as a typed escalation at the loose row. The margin
is −5.2e-6 against a 1e-6 band, so it decides definitely at default
eps and escalates only once the band is widened.
