---
id: a-ring-serving-two-pierces-does-not-evaluate-at-1e-12-on-main
kind: issue
title: coincidence_door's ring-serving-two-pierces row does not evaluate at the 1e-12 row on main
status: closed
closed: 2026-10-13
opened: 2026-10-12
priority: P1
cost: E
refs: [the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone]
---

## Measured

`editor-core` `coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven`
(added by PR 4728) fails at `CAD_TOLERANCE_EPS=1e-12`. It was confirmed in a
worktree of `origin/main` alone at `aa7af4032`. The node does not evaluate:

```text
panicked at crates/editor-core/tests/coincidence_door.rs:44:28:
RecipeNodeId(MintId { ordinal: 90, digest: 16108074374365535202 }) evaluated: None
```

It passes at the default ε and at 1e-6. The PR gate runs the 1e-12 row only
for crates a diff seeds, so it first showed on PR 4746's CI (FLUX,
run 38192091187). That diff does not touch the recipe or the pinch weld.

The row's fixture names a fusion 1e-13 off bitwise. At ε = 1e-12 that
offset is inside the band, so an evaluation that refuses there may be the
honest posture and want pinning per ε rather than fixing. The failure's
`failure(ev, node)` text is `None`, so the refusal's cause is unread; read
it before deciding.

## Closed (2026-10-13, by TANG's fix PR on branch `tang/flush-ring-row-1e-12`)

The fixture was at fault, not the kernel. At ε 1e-12 the blocks' union (`ring_pinch`'s Union node) refuses `Escalated { Coincidence(Sectors, Moot) }`: the 1e-13 gap reads as ≈1.67e-12, which is in the escalate band [1e-12, 1e-11]. `common` and `cut` are then `Poisoned` through it. The "evaluated: None" came from the test helper, because `blocks::failure` maps `Poisoned` to `None`.

The row never passed at 1e-12: it fails identically at its introducing commit, e564d25b2 (PR 4728).

The fix is `pierce_gap()` = min(1e-13, ε/100). It leaves the default and 1e-6 rows building the same fixture. The test's `rows` now names a poisoned node's root failure.

Possible follow-up for FLUSH: `blocks::failure` returns `None` for `Poisoned` in every caller.

