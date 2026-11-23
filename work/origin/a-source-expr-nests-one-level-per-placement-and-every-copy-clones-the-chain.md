---
id: a-source-expr-nests-one-level-per-placement-and-every-copy-clones-the-chain
kind: issue
title: topo: SourceExpr::Placed nests one level per placement and a body's origin tables are cloned whole per level, so evaluation time grows with the square of nesting depth and every derived walk recurses
status: open
opened: 2026-09-30
priority: P2
cost: M
---

(Filed by EDIT's `edit/part-depth-bound` fix pass, which measured why
evaluation at the part-nesting bound is slow.)

## What

`topo::source::SourceExpr` (`crates/topo/src/source.rs`) records a
placement as `Placed { node, instance, inner: Box<SourceExpr> }`, so a
description placed D times carries a chain D boxes deep. A part nested
D documents deep is placed once per document (the identity fast path
included), and a chain of patterns wraps once per copy. The body's
origin tables (`SecondaryMap<CurveKey | PointKey | SurfaceKey,
GeomOrigin>`) hold one such chain per description, with no sharing, and
a body is cloned whole on the way up through each level. So each level
copies and drops chains as deep as the level: the work is quadratic in
nesting depth.

The derived `Clone`, `Drop`, `PartialEq`, `Ord`, `Hash` and `Debug` of
`SourceExpr` each recurse once per level. At the part bound (1024
documents) that fits the smallest stack a door runs on (the at-bound row
below passes on a 1 MiB thread in dev); whether a pattern chain reaches
a depth that does not, and where, is not measured here.

## Evidence

A chain of D documents over a leaf (a block with a boss unioned onto
it), each instantiating the one below, evaluated from the top (dev
profile, one thread):

| D | evaluate |
|---|---|
| 64 | 0.18 s |
| 128 | 0.46 s |
| 256 | 1.42 s |
| 512 | 5.0 s |
| 1024 | 15 s (the review's measurement, and `editor-core::all part_depth_bound::the_refused_chain_flattened_by_one_level_evaluates_at_the_bound`'s 17 s) |

Under `valgrind --tool=callgrind` at D = 160, of 2.75·10⁹ instructions:
`SourceExpr::clone` (inclusive, outermost call) 25.6 %, dropping
`GeomOrigin` 16.1 %, cloning the `Vec<Slot<GeomOrigin>>` of the origin
maps 9.9 %, and malloc/free most of the rest. `StableName`'s walks, the
other thing that nests one level per document
(`work/edit/a-stable-name-nests-one-level-per-copy-and-every-walk-over-it-recurses.md`),
do not appear in the profile.

## What would close it

Placement chains that share their tails (an `Arc` inner), or a flat
representation, so a level costs what it adds; a `Drop` and the other
walks that do not recurse, or a bound every walk fits. A row that
evaluates the part chain at the bound in time linear in its depth.
