---
id: a-prism-bridging-two-wedges-about-a-contact-line-refuses-shared-rim
kind: issue
title: A prism bridging two wedges about a contact line refuses Naming(SharedRim NotAdjacent) on a body topo builds sound
status: closed
closed: 2026-10-13
opened: 2026-10-08
priority: P2
cost: M
---

## What

Found by the sweep of `three-solids-touching-along-one-line-refuse-their-union`
(program tang), measured on 8fd03fce and again on its branch: the same
orders refuse on both, so the topo change does not reach them.

The plate `[0,3] × [0,2] × [0,1]` and three upright prisms over
triangles from (1.5, 1) to the points 0.4 from it at the bearings
0°–50° (z 0.5–2.0), 120°–170° (z 0.47–1.7) and 30°–140° (z 0.44–1.81).
The first two touch along the vertical line through (1.5, 1); the
third bridges the gap between their wedges, crossing both.

Through editor-core's union (`crates/editor-core/tests/union_pinch_member_order.rs`'s
`sector_prism`, `crate::fixture::union_over`), the member orders
[2, 1, 3, 0] and [1, 2, 3, 0] (0 = the plate), and [1, 0, 2] and
[0, 1, 2] for the prisms alone, refuse `Naming(SharedRim { found:
NotAdjacent, .. })` (`names/emit.rs`, the `[] => Rim::NotOne(RimShare::NotAdjacent)`
arm). Topo builds those orders sound
(`crates/topo/tests/three_solids_on_one_line.rs`,
`a_third_prism_crossing_the_wedges_about_the_line_builds_or_refuses_typed`,
"both wedges crossed": 3/3′, closed-form volume, the analytic
oracle). The orders folding the bridging prism last refuse in topo
first (`work/tangtail/an-edge-crossing-two-wedges-about-a-contact-line-is-unbuilt.md`);
the other 16 (4) build through editor-core.

Unmeasured: which face pair the refusal names and why it shares no
edge. The refusing orders are those that fold the bridging prism
onto the two touching ones.

## Closed (branch `emit/naming-refusals`, 2026-10-13)

**Measured.** The refusing orders are exactly those that fold the
bridge `c` (30°–140°) right after the two wedges `a` (0°–50°) and
`b` (120°–170°). At that fold step, operand A is `a ∪ b`; the bridge
fills the gap between the wedges and is consumed there, so the result
holds a **zip-listed seam edge** on the line from z 0.5 to 1.7 whose
two faces are `a`'s 0° wall and `b`'s 170° wall. Both descend to
operand A. In `a ∪ b` those walls share no edge (each meets its own
wedge's other wall along one of the line's two coincident edges), so
`emit_topo`'s `chord_kind` asked `same_side_rim` for their rim and got
`NotAdjacent`. The refusal came through `seam_pair`, not through the
derived-chord path the row guessed.

**Fix** (`names/emit_topo.rs`, `chord_kind`). Two unmerged faces of one
operand that share no edge in it can meet in the result only where the
other operand filled the gap between them and was consumed, so only a
union produces such an edge. Around the edge, the material between the
two faces includes the gap, which neither a subtract nor an intersect
can add. The edge is now their `Seam`, sides in name order. That is the
README's union reading: an edge lying within no member edge "is the
seam of the faces it separates". It is also the name every other order
already gave: folded before either wedge, the bridge leaves the two
walls in different operands, and the edge is an ordinary cross seam.
The merged-faces arm is unchanged.
Such a seam lies along the edge of the operand that filled the gap, so
it ends at operand vertices. The group carries its one operand
(`EdgeGroup::side`), and a seam vertex read off it refuses as an
emission bug rather than taking one face of that operand as the
other side. With the bridge's z-range at (0.6, 1.5), (0.6, 1.9) and
(0.3, 1.5), every order of the prisms alone builds one table (probed,
not pinned).

**A second order-dependence the pin found** (`names/borders.rs`,
`Obstacles::record`). With the plate, order `[c, plate, a, b]` cited
neither `b`'s walls nor its cap in the bridge chord wall's left piece.
The other 17 orders cited both. Here `b`'s notch in that wall is
discarded at the last step. Its boundary along z = 1 is a piece of the
plate seam that step 2 recorded, but `a` had split that seam at step 3,
and `touches` matched only the recorded key. A chain edge's touch now
follows its split lineage (`split_from`) up to the earlier seam, so the
notch joins the plate strip's obstacle. That obstacle divides the wall,
so its walls are cited, as in the other 17 orders and as N2 reads it.

**Pinned.** `crates/editor-core/tests/union_pinch_member_order.rs`,
`a_prism_bridging_two_wedges_is_named_alike_in_every_member_order_that_builds`
(slow set; its fast subset
`a_bridge_folded_after_both_wedges_seams_their_walls` runs per PR). With
the plate, 18 of the 24 orders build. The 6 that fold the bridge last
refuse `CrossingsAlongAContactLine`
(`work/tangtail/an-edge-crossing-two-wedges-about-a-contact-line-is-unbuilt.md`),
and the test says so if that ever changes. Alone, all 6 orders build.
Two bridges filling the gap over two stretches of the line (z 0.2–0.4,
and 0.6–1.5 or 0.6–1.81) build and name alike in all 24 orders: once
one bridge is folded, the walls share a rim along its stretch, and the
other's chord off that rim is their seam too, where it used to refuse
`Emission` in 12 orders.
Every order that builds is tier-3 valid, holds one body, and publishes
one table, compared row by row by where each row's entities stand. The
edge between the walls on the line is one `Seam` of `a`'s and `b`'s
walls in every order. The editor-core suite passes with no golden
moved.

