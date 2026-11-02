---
id: a-one-loop-cylinder-pair-refuses-winds-unseparated-at-coarse-eps
kind: issue
title: A one-loop cylinder pair (e 0.9, r 1 and 0.2) refuses Winds(Unseparated) when carried at eps 1e-6; the cause is unmeasured
status: open
opened: 2026-10-31
priority: P2
cost: M
---


Filed by the SECTARMS orchestrator (2026-10-31) from the cylinder unit's
stage-1 fix pass (PR 6139).

## What

Carrying the one-loop crossing pose (e = 0.9, r₁ = 1, r₂ = 0.2) refuses
`Winds(Unseparated)` at ε 1e-6 and carries at 1e-9 and 1e-12. The lane
read this as a coarse-ε limit and moved its ball row to the two-loop pose,
which carries at all three ε rows.

That reading is unmeasured, and a refusal's text is not its cause
(`memories/refusal-text-is-not-cause.md`).

## Done when

The refusal is measured at its raising site. It is then fixed, or shown to
be a precision limit with a lever, or pinned as what it is. The one-loop
ball row then rejoins `cylinder_pair_section`.
