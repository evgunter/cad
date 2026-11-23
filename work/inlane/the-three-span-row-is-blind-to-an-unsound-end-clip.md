---
id: the-three-span-row-is-blind-to-an-unsound-end-clip
kind: issue
title: The S5c three-span row cannot catch an unsound clip at a span end; only the single-parameter row does
status: open
opened: 2026-10-19
priority: P3
---


Found by reviewer A's delta-read of C6 S5c (PR 5332), S2. Not taken in that PR; the INLANE orchestrator ruled it to its own item.

## Finding

`a_row_over_three_spans_encloses_every_members` (`crates/geom/src/curves/fit.rs`, tests module) stays green under three unsound clips of a span's piece:
- to the lower half of the span;
- to just above the span's low knot;
- to just below its high knot.

It catches only dropping the middle span. Its members sample the parameter's range coarsely, so none lands where an unsound end-clip would miss it. The end-clip mutants are caught today by `over_a_wide_parameter_the_fit_encloses_every_members` alone.

## Owed

The three-span row samples members at each span's ends (the member knots' bracket ends, and the parameter on each), as reviewer B's tie-break probe did. Then each of the three mutants above is red on it too.
