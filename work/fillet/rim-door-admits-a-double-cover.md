---
id: rim-door-admits-a-double-cover
kind: issue
title: rim_of admits a double cover: the chain test is topological and does not detect overlap
status: open
opened: 2026-09-05
---

## The shape

`topo::query::rim_of` (`crates/topo/src/query.rs`) decides "these arcs
are one rim" with a TOPOLOGICAL test: the arcs that matched the seed's
circle and support pair must form one closed chain on shared vertices,
walked from the seed and returning to it having consumed every matched
arc (`order_rim`, `crates/topo/src/query.rs`). That test does not look
at the circle at all once the carriers have matched, so **arcs that
cover one part of the circle twice and another part not at all still
chain, and the door answers them as a rim.**

## The instance, executed

`crates/topo/tests/rim_of_r1_probes.rs::a_double_cover_of_half_the_circle_is_answered_as_a_rim`
(R1's review probe, adopted on PR 1821). Two arcs, both stated on ONE
`Curve3::Circle` value — so `center`, `radius` and `axis` are bit-equal
by construction and the match is not the interesting part — both running
`V0 → V1` over `t ∈ (0, π)`, i.e. both covering the upper half. The walk
closes on the second step with both consumed, and `rim_of` returns
`Ok([first, second])`. The lower half is bare and no refusal fires.

The row also prints the tier-3 verdict for the body it builds, which is
the other half of the picture: what refuses such a body is the
conventional specs at `validate_geometric`, not this door.

## Why it was left

Detecting overlap needs a PARAMETRIC test, and a rim's arcs are minted
one per chart with a seam each: their stored parameter intervals are each
stated in their own frame (Phase 1 on PR 1821 measured `u_ref` disagreeing
on every seam-split revolve rim and on extrude's hole rims). Comparing
them across arcs therefore needs a decided comparison — a band, a margin
— which `rim_of` is specified not to have (`docs/FILLET-RIM-SPEC.md`,
"Constraints, binding": no band, no margin, no sampled geometry).

So this is a real gap with a real reason, and the honest disposition is
a named issue rather than a widened door. It is stated in three places
so no reader meets the door without it: the `rim_of` doc, the
`RimError::NotOneRim` doc, and `order_rim`'s own comment.

## What would close it

Either a decided covering test in a NUMERIC unit of its own (with the
funnel site and the margin the exact door may not have), or a producer-
side argument that no door in this tree can mint two arcs of one circle
between one surface pair that overlap — which would make the gap
unreachable rather than merely unrefused. The second is the cheaper
claim and nobody has made it; the probe above builds its body through
the public Euler doors, so it is not obviously false.
