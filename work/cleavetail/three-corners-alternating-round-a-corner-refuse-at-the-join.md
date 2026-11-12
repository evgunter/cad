---
id: three-corners-alternating-round-a-corner-refuse-at-the-join
kind: issue
title: Three corners touching at a point whose cuts alternate round another solid's corner refuse JoinDesync or PinchUncrossed in three ops
status: closed
opened: 2026-10-03
closed: 2026-11-11
priority: P1
cost: M
pr: 6832
refs: [shared-vertex-crossings-that-tie-or-interleave-are-unprobed]
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
`work/gauge/shared-vertex-crossings-that-tie-or-interleave-are-unprobed.md`'s
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

- 2026-11-03 — moved from CLEAVE to CLEAVETAIL by `git mv` when CLEAVE closed, id and body unchanged (CLEAVE had no dispatchable row above P2 left, and CLEAVETAIL is its own tail on the same ground; Ev, in chat).

## Re-measured (GAUGE, lane `gauge-svc`, 2026-11-11)

On main `0f42d98351` the pin passes. Its three refusals are the laminar
interleave class of
`work/gauge/shared-vertex-crossings-that-tie-or-interleave-are-unprobed.md`:
the band's pair is paired along both of its Out arcs, and the second
and third corners sit on one each. Re-planned with the other pairing
start (a local experiment), `cube ∖ y` builds: volume 0.97142669 =
1 − `cube ∩ y`, and 3′ and tier 2 pass. `y ∩ cube` and `cube ∪ y`
refuse `JoinDesync { what: "conflicting seam vertex correspondence" }`.
Which pairing the shared corner should take is now that row's design
question.

## Closed (GAUGE, lane `gauge-svc`, PR 6832, 2026-11-11)

Every op builds. Where the band's pair's start on the side the op
keeps lays it along the cube's arcs outside the band, which the other
two corners' cuts lie on, the pairs at the cube's corner take the
start that clears them, chosen together
(`shared-vertex-crossings-that-tie-or-interleave-are-unprobed`, its
`## Decided, revised` section; `insert::reconcile_shared`). The finish
takes the seam correspondence as the pair records give it. The pin is renamed and re-pinned to the bodies:
`three_corners_alternating_round_the_cube_build_every_op`
(`crates/topo/tests/union_flush_onto_edge_contact.rs`) asserts every op
builds and passes tier 2, 3′ and the certificate, at `cube ∪ y` =
1.0336378, `cube ∩ y` = 0.0285733 and `cube ∖ y` = 0.9714267, each op
alike in both operand orders, and `y ∖ cube + ∩` = `y`. It runs again
with every reduction's vertex pairs reversed
(`test_support::with_vertex_pairs_reversed`). Red when the other
start is never taken (`SharedVertexCrossings` in `y ∪ cube`) and when
the finish's vertex-set check is restored (`JoinDesync`).
