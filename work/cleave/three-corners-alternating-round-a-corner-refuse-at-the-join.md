---
id: three-corners-alternating-round-a-corner-refuse-at-the-join
kind: issue
title: Three corners touching at a point whose cuts alternate round another solid's corner refuse JoinDesync or PinchUncrossed in three ops
status: parked
opened: 2026-10-03
priority: P1
cost: M
blocked_on: [shared-vertex-crossings-that-tie-or-interleave-are-unprobed]
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
- `cube ∖ y` refuses `PinchUncrossed`. Its zips would fuse the pinch
  at the shared corner to itself, and no kept face there can cross
  between the two cones (`boolean::zip::cross_pinches`); it refused
  `Euler(SelfLoopEdge)`, from that second fusion, until
  `join/pierce-pinch-families`.

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

## Re-measured (JOIN, branch `join/reflex-corner-vertex-vertex`)

With the four-crossing pairing read in each solid's own walk order, and
started where A's runs lie on the side the op keeps of A, the three
ops refuse earlier and typed: `SharedVertexCrossings` at the cube's
corner, where the band's pair, which crosses that corner four times,
has no run clear of the other two corners' cuts either way round. This
is the fan-interleave arm of FUSE's
`shared-vertex-crossings-that-tie-or-interleave-are-unprobed`. Started at
A's first germ instead, the same ops pass the reconciliation and
refuse at the finish as before (measured), so neither start builds
them. The other three ops build as before. The pin moves to the new
refusal.

## Re-measured (JOIN, branch `join/pinch-one-vertex-per-cone-build`, PR 4139)

That PR builds `a-pinch-no-kept-face-can-cross-refuses`, which this row
was blocked on: a pinch is one vertex per cone, split before the zips,
and `PinchUncrossed` and `cross_pinches` are retired. The row does not
move. `y ∩ cube`, `cube ∪ y` and `cube ∖ y` still refuse
`SharedVertexCrossings` at the shared corner's reconciliation, before
any zip, and the other three ops build as before; the pin
(`three_corners_alternating_round_the_cube_refuse_three_ops`) passes
unchanged on the PR's head. What blocks it is FUSE's
`shared-vertex-crossings-that-tie-or-interleave-are-unprobed`, so
`blocked_on` names that row now.
