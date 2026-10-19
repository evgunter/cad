---
id: sym-interval-takes-clip-to-as-the-identity
kind: issue
title: Sym<Interval> takes SpanLocate::clip_to's identity default, so a fit at Sym<Interval> loses the narrowing
status: open
opened: 2026-10-19
priority: P3
---


Found by reviewer B's narrow delta-read of C6 S5c (PR 5332), D5 (unsure). Not taken in that PR; the INLANE orchestrator's rule gives it its own item.

## Finding

`impl<T: SpanLocate> SpanLocate for Sym<T>` (`crates/geom-core/src/sym.rs`) does not override `clip_to`. So `Sym<Interval>` takes the trait's identity default (`crates/geom-core/src/spline/locate.rs`):
- that is sound, since it only loses tightness;
- but the fit's enclosure rows at `Sym<Interval>` (`crates/geom/src/curves/fit.rs`, `enclosure_piece`) are looser than at `Interval`;
- they are also looser than the `max`/`min` envelope they had between S5c's fix pass and its delta fix.

Nobody has checked whether a production path runs the fit at `Sym<Interval>`.

## Owed

- First, establish whether any production path runs the fit at `Sym<Interval>` (a sweep or loft over a symbolic document variable at an enclosure lane).
- If one does, override `clip_to` for `Sym<T>`. A narrowing is not an operation the symbol graph records today, so it needs a graph node, for example a `SymOp::Clip` beside `SymOp::Hull`, or an argument that the clipped value may keep the input's node.
- Add a row at `Sym<Interval>` that pins the width.
