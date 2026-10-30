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
- **A face the selection cannot certify clear of the line** on the side its samples do not reach (QUAD 6013 fix lane, 2026-10-30). `clear_of` boxes each boundary edge piece against the chart's enclosure past the line, set back `ε_in` and inflated by `ε_in`, within a fixed budget. A piece within `ε_in` of the line counts as on it. So does a whole edge whose hull lies within `ε_in` of a straight iso line (`on_the_line`). What stays uncertified is a long edge running along a **curved** knot line within `ε_in` of it, such as one half of a curved surface the exporter split at the knot into two faces. That face is kept as crossing, and its split refuses, because one chain is that edge alone and holds no corner. Certifying it needs the edge's distance from the iso curve bounded along its length (a certified pcurve, or a curve-curve Hausdorff bound). On the committed corpus at the fix (93 imports, `tier_gate::every_corpus_import_passes_the_shared_gate`), no knot reached certification at all.

Placing either needs the line's crossings with the trim: a curve-surface inversion (`chart::uv_of` has no NURBS arm) and the loop surgery that follows it. This is a capability gap, not a design question. Until it lands, such a file refuses typed. On the merge base it imported with the crease inside a cell, and the mesher refused it.
