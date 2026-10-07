---
id: an-end-touch-crossing-has-no-witness-once-edges-join
kind: issue
title: The end-touch crossing (an edge lying in a face that ends in it) has no test witness since the output stage joins the area-overlap fixture's vertices away
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [a-second-crossing-by-one-face-renames-the-first-and-its-pieces]
---


## What

N2 reads a crossing's sense with "a side of the vertex with no portion of
the edge counts as outside", which is what names an end-touch: an edge
lying in a face of the other operand and ending in it. The ruling on
`a-second-crossing-by-one-face-renames-the-first-and-its-pieces` asked
for a row on the area-overlap union's in-face vertices
(`wire_legal_union_refusals::area_overlap_fixture`, `s ∪ big`'s seam ending
at x = 0.2 and 0.4 inside `a`'s wall), which were that case.

Since the output stage joins collinear edges at valence-two vertices
(`BooleanNaming::edge_joins`), no member order of either area-overlap
fixture publishes a vertex there (measured over every order of the three-
and four-member fixtures), so the row has nothing to read. Reading the
no-portion side as inside instead (`emit_topo::senses_at`, `one`) fails no
test of `editor-core`'s naming, union, split, resolve and corpus suites:
the branch has no witness.

## Next

Find a union or boolean that publishes an end-touch crossing — an edge
ending inside a face it lies in, at a vertex of degree three or more —
and pin its two senses, mutation-checked against that branch.
