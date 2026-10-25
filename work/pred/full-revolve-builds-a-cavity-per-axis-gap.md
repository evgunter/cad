---
id: full-revolve-builds-a-cavity-per-axis-gap
kind: unit
title: The full revolve builds one cavity per gap between axis runs: MultipleAxisRuns retires (S116p, Ev on PR 5680)
status: closed
opened: 2026-10-25
priority: P1
cost: M
refs: [S116p]
branch: pred/full-revolve-cavity-per-axis-gap
pr: 5707
closed: 2026-10-25
---

Ruled by Ev on PR 5680 (2026-10-25). The design and the rows owed are
in `S116p.md`'s `## Weighed` and `## Ruled` sections, and the ratified
definition is `crates/topo/README.md`'s producer bullet. The full
revolve of a profile is `revolve(filled) − Σ revolve(pocket)` through
`topo::insert_void`, and its pockets are its hole loops plus the gaps
between its axis runs.

## Closed (PRED orchestrator, 2026-10-25, PR 5707)

Built. A full revolve of a profile meeting the axis in k ≥ 2 runs now
builds one cavity per gap. Each pocket is the gap's stretch taken from
loop 0's single collapse, in hole-as-outer orientation, inserted
through `insert_void` with `Carried{Positive}`. `MultipleAxisRuns` is
deleted. `Revolved::cavities` is keyed by `CavityOrigin`. Every row is
checked against closed-form volumes.

A full single review returned APPROVE-WITH-FIXES with no correctness
break. Its fix pass covered:
- one swept order;
- one collapse of loop 0;
- the docs;
- the `Option` invariant and the sentinel removed;
- a naming test a name swap reds.

Filed: CARVE's
`a-near-collinear-kink-on-a-full-revolve-refuses-as-a-kernel-defect`,
which reproduces on main.
