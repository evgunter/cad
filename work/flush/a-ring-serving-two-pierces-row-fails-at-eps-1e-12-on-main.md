---
id: a-ring-serving-two-pierces-row-fails-at-eps-1e-12-on-main
kind: issue
title: coincidence_door's ring-serving-two-pierces row evaluates no value at eps 1e-12 on main
status: open
opened: 2026-10-12
priority: P1
cost: E
---


Found by the FLUX cone-tail unit (PR 4744), whose CI took
editor-core's 1e-12 row through its merge with main. It reproduces with
main's props sources (`geom-brep/src/props/curved.rs`,
`implicit.rs`) on a branch merged with main at `ca80f91d0a`; the
branch's other changes are tests, docs and work rows:

```text
CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p editor-core \
  -E 'test(a_ring_serving_two_pierces_records_their_fusion_unproven)'
```

fails at `crates/editor-core/tests/coincidence_door.rs:44`:

```text
RecipeNodeId(MintId { ordinal: 90, digest: 16108074374365535202 })
evaluated: None
```

It passes at ε = 1e-9. The row landed with `e564d25b23` (PR 4728 fix
pass). Why the node evaluates to nothing at 1e-12 is unmeasured.
