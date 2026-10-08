---
id: the-census-crossing-lane-escalates-a-line-gap-where-the-segments-lie-far-apart
kind: issue
title: The census's crossing lane escalates the two edges' line-to-line gap before it reads where the lines meet, so segments 1e-2 to 1.5 apart escalate pm_census_ee_gap
status: open
opened: 2026-10-08
priority: P0
cost: M
refs: [near-tangent-boolean-results-ship-with-an-escalated-tier-3-census]
---

## What

Found by JOIN's near-tangent census measurement (`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`, its `## Measured`), on main `047d10d5`, release.

`crates/topo/src/census.rs` `crossing_in_both_interiors` decides
`pm_census_ee_gap`, the distance between the two edges' LINES, before it
reads where the lines meet (`ee_cross_spans`). An escalated gap is pushed
whatever the spans would say. In near-tangent results the lines of two
far-apart segments pass within the band of each other:

- 42 escalations at ε = 1e-9 (tilts 1e-5 to 1e-8), 55 at ε = 1e-12 and
  17 at ε = 1e-6. Every segment pair is 9.6e-3 to 1.5 apart.
- Witness: `w345 nt e0 a8 d1e-8 pc I`, `EdgeKey(25v1)` and
  `EdgeKey(33v3)`. The gap reads 1.84e-9 (exact the same), and the
  segments are 9.85e-2 apart.
- The six `pm_census_ee_span` escalations whose exact value is in band
  are the same shape. The lines meet just past one edge's end, and the
  segments are 4.2e-8 to 2.4e-7 apart. Witness: `vee300 nt e1 a4 d1e-8 pc U`,
  `EdgeKey(20v1)`×`EdgeKey(32v5)`, 2.35e-7 apart.

The margin is exact; the line is the proxy.

Repro: `NT_DUMP=1 cargo run -p sweep --release --example near_tangent_census_probe | python3 scripts/oracles/near_tangent_census_classify.py`, with `NT_ONLY`/`NT_POSE`/`NT_D` to pick the pose and `CAD_TOLERANCE_EPS` the row. The classifier reproduces each census margin bit for bit in f64 and recomputes it at 60 digits on the same coordinates.

## The shape to give

Read the spans before the gap. A crossing parameter definitely outside an
edge's span by more than K·ε settles the pair apart, whatever the lines'
gap. Escalate only where the segments themselves come within the band.
