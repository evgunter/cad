---
id: crossings-of-a-union-seam-of-several-curves-tie
kind: issue
title: A union's crossings of one seam that is several curves tie: rank_along_seam ranks only a seam whose pieces lie on one line
status: open
opened: 2026-10-01
priority: P3
cost: M
refs: [edge-pieces-are-named-by-their-ends]
---

## What

A union's vertex cites a union seam by its head (N2), so where one face
crosses a seam several times the crossings share a name and N2 ranks
them along the crossed edge. `emit_union::rank_along_seam`
(`crates/editor-core/src/names/emit_union.rs:893`) ranks them only where
the seam's pieces lie on one straight line; a seam of several curves
(the two lines a cylinder lying across a plate makes, a circle cut in
pieces) has no one carrier, and the crossings tie (line 952).

## The question

What "the crossed edge" is for a union seam that is several curves: the
piece each crossing ends, which a crossing is the end of two of; or a
tie, which this records. No fixture reaches it; the area-overlap
fixtures of `wire_legal_union_refusals` reach the one-line case and
rank.
