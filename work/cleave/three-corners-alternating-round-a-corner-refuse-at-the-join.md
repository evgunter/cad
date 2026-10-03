---
id: three-corners-alternating-round-a-corner-refuse-at-the-join
kind: issue
title: "Three corners touching at a point whose cuts alternate round another solid's corner refuse JoinDesync or Euler(SelfLoopEdge) in three ops"
status: open
opened: 2026-10-03
priority: P1
cost: M
---

## What

Three trihedral corners at the origin (`corner_prism`, scaled 0.4)
folded into one body `y` by unions, touching only there, against the
unit cube's corner at the origin. One corner is a band that meets the
cube corner's boundary in two dangling segments, and the other two lie
in the gaps between, so the cuts alternate round the cube's corner.
The shared corner's runs reconcile (every planned run is clear of the
others' cuts). Then:

- `y ∪ cube`, `y ∖ cube` and `cube ∩ y` build, pass 3′, and agree
  (∖ + ∩ = y; ∪ = 1 + ∖);
- `y ∩ cube` and `cube ∪ y` refuse
  `JoinDesync { what: "conflicting seam vertex correspondence" }`;
- `cube ∖ y` refuses `Euler(SelfLoopEdge)`.

The same on main before
`work/fuse/shared-vertex-crossings-that-tie-or-interleave-are-unprobed.md`'s
PR (measured). Pinned by
`three_corners_alternating_round_the_cube_refuse_three_ops`
(`crates/topo/tests/union_flush_onto_edge_contact.rs`), which carries
the rays.

## Owed

Trace the seam correspondence (`finish.rs`) and the zip's self-loop
for a shared corner holding three pairs' null edges, two of them one
pair's struts; build every op at volumes checked outside the kernel.
