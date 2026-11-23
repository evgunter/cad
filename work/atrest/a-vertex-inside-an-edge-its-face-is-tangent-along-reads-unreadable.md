---
id: a-vertex-inside-an-edge-its-face-is-tangent-along-reads-unreadable
kind: issue
title: A vertex resting inside an edge whose face is tangent to the vertex's faces along that edge reads its self-touch Unreadable
status: open
priority: P2
cost: M
opened: 2026-11-23
---

Found by the LATHE wrap-edge-station lane (2026-11-23, branch
`lathe/wrap-edge-station`), measured there and on its base
`lathe/full-revolve-one-band`.

## What

`B ∖ A`, `A` the one-segment extruded cylinder `r = 0.3`, `z ∈ [0, 1]`,
inside `B`, `r = 0.5` about `(−0.2, 0)`, `z ∈ [−0.5, 1]`: the walls are
tangent along the ruling through `(0.3, 0)`, the hole's floor rim
touches `B`'s outer wall at `(0.3, 0, 0)`, and `B`'s seam lies on that
ruling. The result is one maximal body: `B`'s seam one edge from
`z = 1` to `z = −0.5`, `A`'s rim vertex resting inside it (record
`(vertex, edge)`), `A`'s seam lying on its upper part (record
`(edge, edge)`). Tier 3′ refuses it undecided:
`BoundaryCrossesItselfUndecided { witness: "(0.3, 0.0, 0.0)", why: Curved(Unreadable) }`.
Volume, tier 2, the certificate and the operand check all pass.

The reading: the planar link declines (curved faces), and the tangent
cones' first-order hold (`crates/topo/src/link.rs`, the probe loop
that sets `notes.coincident`) finds two curved sectors lying on each
other along a ray. That is `B`'s wall, read along its edge on both
sides (`census::tangent_sectors`' edge fans), on `A`'s hole wall,
which is tangent to it along the ruling. It answers `Unreadable`.

## Two witnesses, one gap

- **Already undecided before the wrap-edge fix**: the same cell with
  `B` built from two arcs, so that its seam is an edge between two
  distinct wall faces. The join has always taken that seam's station
  at `z = 0`, and tier 3′ reads the identical finding there. Measured
  on LATHE's base `lathe/full-revolve-one-band`, which builds this
  extrusion as main does.
- **Newly reached**: the one-segment `B`, whose seam is its one wall's
  wrap edge. It read SOUND only because its station stayed unjoined:
  the join's predicate refused a station on a wrap edge
  (`work/lathe/a-station-on-a-wrap-edge-is-not-joinable-and-passes-check-11.md`),
  so the touch read vertex/vertex at two corners of a non-maximal
  body. PR 7532 makes the join take it. So a class of extruded-cylinder
  internal tangencies, a seam on the tangent ruling with a rim vertex
  touching it, now reaches this gap where it used to build a
  non-canonical body read SOUND.

## Witness

`sweep` `a_closed_conic_in_the_partners_face::a_rim_flush_in_its_partners_top_tangent_inside_builds_or_meets_a_pinned_limit`,
cells `A's vertex at 0, B's at 0`, `B ∖ A`, at rest and turned, pinned
there as this limit.

## Done when

Those cells read SOUND, and their pin
(`sound_or_unread_self_touch`) goes. The two-arc `B` cell reads SOUND
too, though no row pins it yet.
