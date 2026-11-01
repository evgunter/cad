---
id: a-ring-serving-two-pierces-is-red-at-1e-12-on-main
kind: issue
title: editor-core's a_ring_serving_two_pierces_records_their_fusion_unproven is red on main at eps 1e-12
status: closed
closed: 2026-10-13
opened: 2026-10-12
priority: P1
---

## Measured

`editor-core` `coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven` (`crates/editor-core/tests/coincidence_door.rs`) fails at ε 1e-12 with CI's opt-level 1, and passes at ε 1e-6.
- NURBS build A reproduced it on main `8cd31ca1` and on PR 4551's branch, each on its own isolated target dir, so PR 4551 did not cause it.
- PR 4551's CI met it in run 38198669660 (job `test`, the eps step). The hosted log does not carry its panic text; replay locally for it.

```
CAD_TOLERANCE_EPS=1e-12 CARGO_PROFILE_TEST_OPT_LEVEL=1 cargo nextest run -p editor-core -E 'test(a_ring_serving_two_pierces_records_their_fusion_unproven)'
```

The row comes from FLUSH's closed `the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone`, which says the lint names a fusion 1e-13 off bitwise. At ε 1e-12 that 1e-13 offset is inside the tolerance, so the likely reading is that the fixture's offset does not scale with ε. This is not checked.

## Wanted

Root cause, and the fixture or the claim made ε-aware. The eps rows are gating, so this reddens every PR that seeds editor-core's ε rows. — (NURBS orchestrator)

## Panic text (NURBS build A, 2026-10-12)

The panic is byte-identical on main `8cd31ca1d4` and on PR 4551's branch, at opt-level 1 and ε 1e-12:

```
crates/editor-core/tests/coincidence_door.rs:44:28: RecipeNodeId(MintId { ordinal: 90, digest: 16108074374365535202 }) evaluated: None
```

So at 1e-12 the node does not evaluate at all, rather than recording a different proof. The breaking commit has not been bisected. — (NURBS orchestrator)

## Closed (2026-10-13, by TANG's fix PR on branch `tang/flush-ring-row-1e-12`)

The fixture was at fault, not the kernel. At ε 1e-12 the blocks' union (`ring_pinch`'s Union node) refuses `Escalated { Coincidence(Sectors, Moot) }`: the 1e-13 gap reads as ≈1.67e-12, which is in the escalate band [1e-12, 1e-11]. `common` and `cut` are then `Poisoned` through it. The "evaluated: None" came from the test helper, because `blocks::failure` maps `Poisoned` to `None`.

The row never passed at 1e-12: it fails identically at its introducing commit, e564d25b2 (PR 4728).

The fix is `pierce_gap()` = min(1e-13, ε/100). It leaves the default and 1e-6 rows building the same fixture. The test's `rows` now names a poisoned node's root failure.

Possible follow-up for FLUSH: `blocks::failure` returns `None` for `Poisoned` in every caller.

