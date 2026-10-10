---
id: a-vertex-read-by-two-sector-passes-panics-instead-of-refusing
kind: issue
title: A vertex read by two sector passes trips a debug_assert panic in the boolean instead of a typed refusal
status: closed
opened: 2026-10-06
priority: P1
cost: M
closed: 2026-10-07
refs: [a-vertex-read-twice-where-the-first-pass-writes-nothing-refuses]
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

## Closed (2026-10-07, TANG)

The `debug_assert!` is gone. Before the first vertex-on-face pass
writes, `vtxfac::refuse_sector_rereads` refuses
`BooleanError::VertexReadTwice` in every build. It names the operand,
the vertex, and its first two reads in pass order (`SectorRead::Pierce`
of a face, or `SectorRead::Pair` with a vertex). It refuses a vertex
that pierces a face and is read again, by a second pierce or by a pair.
A vertex in several pairs is not refused, since the vertex-vertex
passes read every pair before the first writes.

The reviewer's arch cone was not in the tree. The rows rebuild its
shape (`crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`):
three pyramids standing on their apexes at `MEET` above the plate, the
same with one, a `meeting::wedge` prism through the top beside one, and
two blocks in face contact in one body. Each row runs every op in both
operand orders at every pose of `meeting::poses`. On the merge base,
every case panicked at the assert, in debug and in the workspace's
release profile, which keeps debug assertions on. With assertions off,
the prism row hit `sectors.rs`'s `unreachable!`, and the touch-only
rows built bodies silently. Those touch-only poses refuse now: filed
`a-vertex-read-twice-where-the-first-pass-writes-nothing-refuses` (P1).
