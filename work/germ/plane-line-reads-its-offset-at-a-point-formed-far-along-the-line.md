---
id: plane-line-reads-its-offset-at-a-point-formed-far-along-the-line
kind: issue
title: plane_line forms its midpoint as line.origin + line.dir·mid before reading the plane offset, a formula loss on stored data where the line's origin lies far from its run
status: open
opened: 2026-11-22
priority: P2
cost: E
---


Filed by GERM's `germ/plane-pose-far-frame` lane (PR 7411) from its
FULL review.

## Finding

`curve_face::pose::plane_line` (`crates/geom-brep/src/curve_face/pose.rs`,
`plane_line`, the `at_mid` line) reads the line's offset at its run's
middle as `geom::plane_offset(origin, normal, line.origin + line.dir·mid)`.
PR 7411 posed `plane_offset` itself well (`Real::difference_dot`), but the
point `line.origin + line.dir·mid` is formed first, in plain arithmetic:
where the line's stored origin lies far from its run (`|mid|` large),
each coordinate rounds at `ulp(|line.origin| + |mid|)`. The stored
`origin`, `dir` and `mid` determine an exact offset
`n·(origin − o) + mid·(n·dir)`, so on the PR's own premise (stored data
exact) this is a formula loss. The plane door's parallel `offset`
(`curve_face::plane_crossing`, `SpanPlaneMeet::Parallel`) forms the same
point.

## Settles when

The offset at the middle is formed from the stored terms without
forming the point: `n·(origin − o)` through `difference_dot`, plus
`mid·(n·dir)` with its product error-free. Measured by a far-origin row
at ε 1e-12 against an exact oracle.
