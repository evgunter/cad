---
id: a-rows-zero-slides-across-a-read-refuse-once-an-edit-turns-the-read-into-their-plane
kind: issue
title: a row along a read direction writes its zero slides across the direction as read at write time, so an edit that turns the read into their plane refuses the row; and Placed::of beside a mate has no across
status: open
opened: 2026-11-02
priority: P3
cost: M
---


Found by the PR 6394 dual review (R2 S4), filed by its fix pass. Traced, not run.

- `pncad::pattern::linear_pattern` writes the row's other slides as the constant `0` along the target axes `across` names. The viewer chooses them (`DocSession::picked_heading`, `pncad::document::slides_across`) from the read direction **as it evaluates at the commit**.
- The read is live, so an edit that turns the picked edge into the plane of those two axes makes the three slides dependent. The fold then refuses the row with `NoChartWhy::ReadSlidesDoNotSpan` (`mate/value.rs`, `unmated_motion`), although the row's own value moves nothing.
- `Placed::of` beside a mate takes no `across` at all: a `SlideAlong` beside a direction mate gets its zero slides from the target axes in order.

**Owed.** A decision on whether zero-valued slides beside a read should follow the read (re-chosen at evaluation, which is a value whose direction no one wrote) or the refusal should offer the axes to write. Either is a `[placement.values]` reading, so it may need designers.
