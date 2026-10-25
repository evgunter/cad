---
id: full-revolve-builds-a-cavity-per-axis-gap
kind: unit
title: The full revolve builds one cavity per gap between axis runs: MultipleAxisRuns retires (S116p, Ev on PR 5680)
status: open
opened: 2026-10-25
priority: P1
cost: M
refs: [S116p]
---

Ruled by Ev on PR 5680 (2026-10-25). The design and the rows owed are
in `S116p.md`'s `## Weighed` and `## Ruled` sections, and the ratified
definition is `crates/topo/README.md`'s producer bullet. The full
revolve of a profile is `revolve(filled) − Σ revolve(pocket)` through
`topo::insert_void`, and its pockets are its hole loops plus the gaps
between its axis runs.
