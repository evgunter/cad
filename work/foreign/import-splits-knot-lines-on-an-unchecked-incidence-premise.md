---
id: import-splits-knot-lines-on-an-unchecked-incidence-premise
kind: issue
title: the C⁰-knot split's side certificate assumes every boundary edge lies within ε_in of its face's chart, which nothing in step-import checks before the split
status: open
opened: 2026-10-30
priority: P3
cost: M
refs: [import-splits-a-foreign-face-or-edge-at-each-c0-knot, the-c0-knot-split-refuses-what-it-cannot-place]
---


(QUAD 6013 fix lane; the delta review's N3 on PR 6013.)

`crates/step-import/src/knot_split.rs` `clear_of` certifies that a face's boundary keeps clear of the chart past a knot line by inflating the region's enclosure by `ε_in`. That inflation is sound only if every boundary edge lies within `ε_in` of the face's chart, so that a point farther than `ε_in` from the region has no foot in it. Nothing in step-import checks that premise before `split_c0_knots` runs. The file states it, and tier 3 certifies each edge on its faces only at rest, after the split has already selected.

What happens if the premise fails: an edge off its chart by more than `ε_in` can have its foot past the line while the edge itself keeps clear of the inflated region. The line is then left out, and the face keeps a crease in its cell. Whether tier 3's incidence certificate refuses every such body at rest, so that nothing ships silently, is not established.

Owed, either:
- a cheap pre-split bound on each boundary edge's distance from its chart (a certified incidence reading at the edge's own sample density, or the adopted edge's own certificate if one exists by then), used as the inflation; or
- a shown and pinned argument that tier 3 refuses every body whose edge exceeds the premise.
