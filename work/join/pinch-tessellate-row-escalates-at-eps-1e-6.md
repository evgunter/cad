---
id: pinch-tessellate-row-escalates-at-eps-1e-6
kind: issue
title: pinch_faces_tessellate's notch307 fib117 row escalates the boolean at ε = 1e-6: main is red on that row
status: open
opened: 2026-10-06
priority: P0
---


`CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep --test all -E
'test(pinch_faces_tessellate)'` fails on bare `origin/main` at
b879a7cb: `pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates`
panics at its "the op builds no body" arm on the row
`notch307 fib117 edge psi=1.9 cp S`:

```
Err(Escalated { decision: Coincidence(Join, Moot), diag: Indeterminate {
  margin: MarginDiag(Value(-5.196042020649827e-6, None)), terminal_sliver: false,
  band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 },
  predicate: Some("bool_join_nearest") } })
```

The row's join margin (≈ −5.2e-6) sits inside the ε = 1e-6 row's
ambiguity band, so the boolean escalates rather than builds. The test
arrived with the pinch-wedge work (PR 4074's item,
`a-planar-face-through-two-vertices-on-one-point-meshes-under-one-id`);
a PR that touches only `mesh` does not run sweep's 1e-6 row, so the
gate there could not see it. Seen on PR 4111's CI (run 37421965598,
the sweep `eps=1e-6` group), which touches `sweep`.

Owed: either the row's fixture is moved clear of the band at every ε
row (a fixture whose margin is ε-relative), or the row names the
escalation it takes at 1e-6 as its expected outcome there.
`crates/sweep/tests/pinch_faces_tessellate.rs` is tcost/tint ground
by territory; the row is JOIN's.
