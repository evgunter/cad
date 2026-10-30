---
id: the-c0-knot-split-refuses-what-it-cannot-place
kind: issue
title: the import's C⁰-knot split refuses a face whose trim crosses the knot line more than once, a crossed face with rings, or a face it cannot certify clear of the line
status: open
opened: 2026-10-29
priority: P1
cost: M
refs: [import-splits-a-foreign-face-or-edge-at-each-c0-knot, tier-3-refuses-a-carrier-with-an-interior-c0-knot]
---


(FOREIGN implementer, `foreign/c0-knot-split`.)

`crates/step-import/src/knot_split.rs` splits a face chart along a crease knot's iso line where the line crosses the face (`crossing_knots`: projected boundary samples are evidence that the boundary reaches a side, and a side they do not see reached is certified clear by enclosures, `clear_of`, before the line is left out). It places the line only where it runs across the whole chart, from `Q₀` to `Q₁` (control points of the net), and parts the face's single loop at those two points: one chain on each sub-patch, or, on a chart closed across the split, one walk of the closed boundary each. A line outside the face's trimmed domain, or along its boundary, lies inside no cell and needs nothing (orchestrator's reading of "strictly inside its cell", 2026-10-29).

What still refuses `StepImportError::C0KnotUnsplit`, naming the face and the knot:

- **A trim that crosses the knot line more than once**, or crosses it away from the chart's two boundary rows. The line then parts the face into several pieces, or into pieces whose loops need outer-loop inference.
- **A crossed face with rings.**
- **A face the selection cannot certify clear of the line** on the side its samples do not reach (QUAD 6013 fix lane, 2026-10-30). `clear_of` boxes each boundary edge piece against the chart's enclosure past the line, set back `4·ε_in` and inflated by `ε_in`, within a fixed budget. Axis-aligned boxes cannot part an edge that runs along the line at under `ε_in` from it unless the line is axis-aligned, so a face bounded by an edge on the knot line (one half of a surface the exporter split at the knot into two faces) is kept as crossing, and its split refuses: one chain is that edge alone and holds no corner. Certifying it needs a bound in the chart (a certified pcurve of the edge, or the edge's distance from the iso curve), not a 3-D box.

Placing either needs the line's crossings with the trim: a curve-surface inversion (`chart::uv_of` has no NURBS arm) and the loop surgery that follows it. This is a capability gap, not a design question. Until it lands, such a file refuses typed. On the merge base it imported with the crease inside a cell, and the mesher refused it.
