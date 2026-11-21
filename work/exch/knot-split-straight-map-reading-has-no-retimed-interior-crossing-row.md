---
id: knot-split-straight-map-reading-has-no-retimed-interior-crossing-row
kind: issue
title: knot_split reads a straight knot line through its chord map, and no fixture crosses a re-timed straight knot line inside it
status: open
opened: 2026-11-20
priority: P3
cost: M
---


Found by EXCH unit B's fix pass (PR 7170, the dual review's "unguarded
mechanisms"). `crates/step-import/src/knot_split.rs`, `split_face_once`
(the `straight` reading near `to_w`): an iso at a crease knot that reads
as the straight MAP (`recognize_curve::straight_map`) is read as a
`Line`, and a crossing found on it is carried back to the iso's
parameter through the affine chord map. The map is owed there, not the
locus, and only an INTERIOR crossing reads it: a crossing at the line's
ends is exact under any timing.

No committed fixture has a straight knot line re-timed along itself
with an edge crossing it inside. Measured: `ridge_u` remade quadratic
across the ridge with the middle row off its Greville position imports
the same whether the split reads `straight_map` or the locus-only
`try_line` (its crossings are at the line's ends), so it does not
guard the reading. A `c_prism`-class fixture — a trim crossing a
straight, re-timed knot line inside it — would.

What would close it: that fixture, pinned, red with the reading
replaced by `try_line`.
