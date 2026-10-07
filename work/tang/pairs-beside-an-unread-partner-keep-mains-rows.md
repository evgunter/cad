---
id: pairs-beside-an-unread-partner-keep-mains-rows
kind: issue
title: A vertex in pairs alone beside a partner no corner reading reads keeps main's per-pair rows, wrong or missing where that partner holds the edge
status: open
opened: 2026-10-07
priority: P2
cost: M
refs: [a-vertex-read-twice-where-the-first-pass-writes-nothing-refuses]
---


Filed by PR 4256's third review (m-2).

## What

`vtxfac::pair_classes` layers a vertex's partners where every one reads
as a convex cone or its complement (`sectors::wedge_classes`). Where one
reads neither way, the edge classes fall back to each pair's own rows,
which is main's behaviour. Two examples of a partner that reads neither
way: a dart's apex (a reflex edge), and a near-flat non-convex
quadrilateral void.

Those rows read each partner alone. So they are wrong where the unread
partner holds the edge, and missing where it is the only reading. These
are naming rows (`BooleanReduction::edge_classes`), not the body. The
material is right.

## Witnesses

At every pose of `meeting::poses`, ε = 1e-9:
- **Wrong rows.** "inside a bare dart beside a bare arch"
  (`crates/topo/tests/a_vertex_read_again_classes_every_edge.rs`,
  `Expect::PairsOwn`). A pyramid inside a dart's cone is paired with an
  arch and the dart, apart, in one body. The arch's rows put the edges
  Out, but they lie in the dart. That is 90 wrong rows over the review's
  probes, the same as main.
- **Missing rows.** "near-flat quad void -1e-3 buried, island" with
  probe `hang_over`, `x ∩ y` and `y ∩ x`. These raise `Emission` "a
  crossing's edge has no piece the boolean classified at the crossing",
  as on main.

## The shape to give

Read a non-convex partner. Either cut the cone into convex pieces, by
the planes of its faces at its reflex edges, and layer the pieces. Or
read an edge's membership of the polygon cone directly. Then layering
needs no fallback, and these rows match the germ.
