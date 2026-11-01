---
id: a-slide-value-doubles-its-translations-width
kind: issue
title: A slide value doubles its translation's width on a box lane
status: closed
opened: 2026-10-18
priority: P3
cost: M
closed: 2026-10-23
refs: [5613]
---

`value::settle` sets a slide as `gap = v.by − chart.slide(&m, axis)` and then translates the folded map by `gap` along the axis (`crates/editor-core/src/mate/value.rs:279-284`; `chart.slide` at `:157-159` reads `(m.translation − origin)·axis`). On a box lane, `gap` carries the map translation's width, and the translation then adds it again, so the result is about twice the width the value and the target justify. The INTENT stage 3 D m10 lane measured it. It is the translational twin of the spin's width, which D fixed by setting the turn from the two charts' frames (`Chart::turned`).

**Done when** the slide sets the translation's component along the axis directly (`origin·axis + v.by`), not by adding a difference, on the lanes where `SolveScalar::spins_by_frames` holds. Add a row that measures a slid copy's translation width at Interval against the value's own. If D′ rewrites the chart, it absorbs this.

## Closed

Closed by D′2 (`a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space`, PR 5613, 2026-10-23). The chart sets a slide's translation component directly where the chart is on the box lanes' axes, not by adding a difference. Row: `intent_dprime2_mates_read_geometry::a_slide_value_sets_its_translation_with_no_width` (the slid copy's translation width at Interval equals the value's).
