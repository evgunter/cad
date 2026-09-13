---
id: torus-tangency-shell-floor-does-not-scale-with-k
kind: issue
title: the_clamp_floor_clears_the_torus_tangency_shell asserts a fixed floor against a shell that grows as K^(1/3) — red at CAD_AMBIGUITY_K=30 on main
status: open
opened: 2026-09-11
---


(FIX orchestrator) From the `literal-k-where-the-runs-k-belongs` lane,
PR 2346. Reported by the lane and placed here rather than repaired:
the lane's diff cannot reach it, and this program's question — whether
the suite asserts what it claims to assert — is exactly what it is.

## The defect

`crates/sweep/tests/bool3_torus_doors.rs:683`,
`the_clamp_floor_clears_the_torus_tangency_shell`, fails at
`CAD_AMBIGUITY_K=30` on `main`'s own tree (reproduced with the lane's
branch stashed):

```
the probe offset 0.001 no longer clears the tangency shell
5.147018158714216e-4 by 2×
```

`away()` is a **fixed** floor; the shell grows as K^⅓. So the row
asserts a clearance that holds at `DEFAULT_K` and at no sufficiently
large K — and **the assertion's own message predicts this**, which is
the part worth noticing: the row already knows what would break it and
still pins a constant.

## Why it is latent rather than red

No CI row runs the suite at a non-default K — measured, no workflow
mentions `AMBIGUITY_K` at all. That hole is filed separately as CIW's
`work/ciw/no-ci-row-runs-the-suite-at-a-non-default-k.md`. Until it
closes, this row cannot go red in the gate, which is why it has
survived: it is not a regression waiting to happen, it is a claim that
is already false about configurations the gate never draws.

## What it needs

The floor derived from the shell rather than pinned beside it, or the
row's claim narrowed to the K it actually holds for and said so at the
site. Both are small; the choice is about what the row is *for*, which
is this program's question rather than a passing lane's.
