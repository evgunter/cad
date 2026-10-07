---
id: a-vertex-read-by-two-sector-passes-panics-instead-of-refusing
kind: issue
title: A vertex read by two sector passes trips a debug_assert panic in the boolean instead of a typed refusal
status: open
opened: 2026-10-06
priority: P1
cost: M
---


## What

Found in PR 4129's full review, on the arch-cone pose of
`nested-pierce-runs-have-no-ring-order`. The same is true on PR 4129's
merge base.

`boolean/mod.rs` (`boolean_reduce`, after the vertex-on-face passes)
holds a `debug_assert!`: "a vertex is read by two sector passes after
the first may hang a strut there". Its premise is that a vertex pierces
at most one face, and that a paired vertex pierces none. A geometry that
breaks the premise panics in a debug build, and in release it goes on
silently, reading an orbit an earlier pass has already written. The
same pose also reaches `BooleanError::SharedVertexCrossings`.

## The shape to give

Refuse typed where the premise fails, naming the vertex and its two
reads, or read every orbit before the first pass writes, as the
vertex-vertex lane does. Pin it with the arch pose.
