---
id: a-mate-charts-zero-decision-builds-on-a-coincidence-the-door-never-sees
kind: issue
title: A mate chart's Zero on a read slide direction builds on a coincidence the door never records
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Found by `doctail/the-least-turn-and-aligned-read-exactness-off-the-nominal-at-sym`'s note pass.

## What

`mate::value::Chart::slide_direction` (the `Coord::SlideAlong` arm) decides `MATE_READ_SLIDE_WITHIN` on the read direction's part across the mate (`d·u`, or `|d × u|` beside an axis mate), levered at the arm. It admits the slide only on `Zero`. That `Zero` is a coincidence the build stands on: a direction read off a body lies in the mated plane.

It is logged to `k_stats` alone, never as a recorded coincidence row (`NamedCoincidence`). So the door (`coincide::prove`) never sees it: it is neither proved nor reported unproven, and a copy placed along such a slide moves as though the coincidence were exact.

`mate_chart_slides_span` is the other mate-chart predicate with a `Zero` outcome. On `Zero` it refuses (`SlidesDoNotSpan`), so nothing builds on it; it needs no row.

## Owed

Either `MATE_READ_SLIDE_WITHIN`'s `Zero` becomes a recorded coincidence of the placement (cells: the read direction's entity and the mated face), so the door proves it or names it, or the ratified text says why a mate's admission is no coincidence. Check first which `[placement.values]` clause governs it.
