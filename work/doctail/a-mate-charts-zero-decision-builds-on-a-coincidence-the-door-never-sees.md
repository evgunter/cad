---
id: a-mate-charts-zero-decision-builds-on-a-coincidence-the-door-never-sees
kind: issue
title: A mate chart's Zero on a read slide direction fixes what the slide's value means, and the door never records it
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Found by `doctail/the-least-turn-and-aligned-read-exactness-off-the-nominal-at-sym`'s note pass.

## What

`mate::value::Chart::slide_direction` (the `Coord::SlideAlong` arm) decides `MATE_READ_SLIDE_WITHIN` on the read direction's part across the mate (`d·u`, or `|d × u|` beside an axis mate), levered at the arm. It admits the slide only on `Zero`.

**The `Zero` decides no contact.** On `Zero`, `slide_direction` returns `unit(part_left(u))`: the read direction's projection into what the mate leaves, so the copy stays seated whatever the residual. What the `Zero` decides is the slide value's *meaning*. The value is taken along that projection, not along `u` itself, and the two differ by `cos θ`, `θ` the residual angle that the decision called zero.

That meaning is logged to `k_stats` alone, and no coincidence row records it (`NamedCoincidence`). So the door (`coincide::prove`) never asks whether the read direction lies in what the mate leaves for every value, or only at the nominal.

`mate_chart_slides_span` is the other mate-chart predicate with a `Zero` outcome. On `Zero` it refuses (`SlidesDoNotSpan`), so nothing builds on it; it needs no row.

## Owed

Decide what a `MATE_READ_SLIDE_WITHIN` `Zero` owes, and say where:
- **Either** it becomes a recorded coincidence of the placement (the read direction's entity and the mated face), so the door proves that the slide's direction lies in what the mate leaves, or names it unproven.
- **Or** the ratified text says why a slide's value read along its projection needs no proof.

Check first which `[placement.values]` clause governs a slide along a read direction.
