---
id: edge-length-scale-question-has-no-shared-helper
kind: issue
title: the edge-length scale question is asked per door with no shared helper, spelled three ways with three recourses
status: open
opened: 2026-10-24
---


## Where this came from

The full review of PR 5654 (CHART, `normalize-without-the-length-question-two-more-sites`), its findings S1, S5 and S6. Filed on ATREST's slate because two of the four sites below are in `census.rs`, which `work.py territory` gives to ATREST and RESTREAD. `chart_region.rs` is CHART's, and `sector_shape.rs` has no owner.

## The finding

"Is this edge's length an ordinary number?" is asked by three doors. Each spells the question itself, and none spells it the way the others do:

- `topo` `chart_region::edge_lengths` (behind `proper_crossings`) asks `!len.is_bounded() || is_underflowed_length(len, witness)`. It refuses `ChartRegionError::EdgeScale`, whose wording is the `chart_region::EDGE_SCALE_REASON` / `EDGE_SCALE_RECOURSE` pair, bounds included.
- `topo` `sector_shape::sector_shape` asks `!is_finite_length(..)` and `is_underflowed_length(..)` per chord. It refuses two variants, `SectorFault::NonFiniteChord` and `SectorFault::UnderflowedChord`. Each lane wraps them in its own words: the census edge wedge maps both to `WedgeRefusal::NoSheet`.
- `topo` `census::chord` asks `!is_finite_length(len) || is_underflowed_length(..)`. It refuses `TouchUnread::ChordScale` / `Undecided::TouchChordScale`, whose text ("rescale the model so every edge length is an ordinary number") gives no bound.

At an interval scalar, `is_finite_length` waves an enclosure reaching `∞` through, while `Real::is_bounded` refuses it. So the three doors do not even agree on which inputs refuse.

The census also spells `ChartRegionError`'s 14-arm carried or-pattern twice, once at the sweep arm and once at the validation arm. Every new `ChartRegionError` arm is two edits there, plus the samples roster, plus `validate::classify_chart_region`, plus `tests/census_g2_carrier.rs`.

## What a fix looks like (not decided)

One helper in `geom_core` or `topo` that takes the vector and answers its length or an edge-scale refusal. Use it at all three doors, with one recourse wording carrying the bounds (about 1e154 m and about 1e-162 m) and one answer at the interval scalar. Collapse the census's two or-patterns into one `fn` or `matches!` that both arms read.
