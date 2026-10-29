---
id: the-c0-knot-split-refuses-what-it-cannot-place
kind: issue
title: the import's C⁰-knot split refuses a face whose trim crosses the knot line more than once, or a crossed face with rings
status: open
opened: 2026-10-29
priority: P1
cost: M
refs: [import-splits-a-foreign-face-or-edge-at-each-c0-knot, tier-3-refuses-a-carrier-with-an-interior-c0-knot]
---


(FOREIGN implementer, `foreign/c0-knot-split`.)

`crates/step-import/src/knot_split.rs` splits a face chart along a crease knot's iso line where the line crosses the face (`sides`, a projection-read selection of which side each boundary sample lies on). It places the line only where it runs across the whole chart, from `Q₀` to `Q₁` (control points of the net), and parts the face's single loop at those two points: one chain on each sub-patch, or, on a chart closed across the split, one walk of the closed boundary each. A line outside the face's trimmed domain, or along its boundary, lies inside no cell and needs nothing (orchestrator's reading of "strictly inside its cell", 2026-10-29).

What still refuses `StepImportError::C0KnotUnsplit`, naming the face and the knot:

- **A trim that crosses the knot line more than once**, or crosses it away from the chart's two boundary rows. The line then parts the face into several pieces, or into pieces whose loops need outer-loop inference.
- **A crossed face with rings.**

Placing either needs the line's crossings with the trim: a curve-surface inversion (`chart::uv_of` has no NURBS arm) and the loop surgery that follows it. This is a capability gap, not a design question. Until it lands, such a file refuses typed. On the merge base it imported with the crease inside a cell, and the mesher refused it.
