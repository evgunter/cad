---
id: three-wedges-meeting-at-a-point-on-a-face-refuse-in-every-member-order
kind: issue
title: Three wedges meeting at one point of a plate's top refuse in every member order, with three different refusals
status: closed
opened: 2026-10-02
priority: P1
cost: M
closed: 2026-10-06
branch: tang/holes-meeting-at-a-vertex
---


## What

The plate `[0,3] × [0,2] × [0,1]` with three triangular prisms standing on
it, z ∈ (0.5, 2.0). Their plans are the sectors 0°–60°, 120°–180° and
240°–300° of radius 0.4 about (1.5, 1.0). The prisms touch pairwise only
along the vertical line through (1.5, 1.0), and their footprints on the
plate's top are three holes meeting at one vertex. The union refuses in
all 24 member orders, and which refusal depends on the order:

- `Join(RingHomingAmbiguous)` when two prisms come before the plate
  (`crates/topo/src/chord_join.rs`, `rehome_rings`);
- `JoinDesync { "ring-run winding is degenerate (zero enclosed area)" }`
  (`crates/topo/src/boolean/join.rs`, `ring_run_ccw`);
- `ClassificationInvariant { "two distinct same-solid bounds share one
  ray (degenerate operand)" }`.

The counts were the same before and after the branch
`tang/pinch-union-order`. That branch welds two pierces that meet on one
kept face (`crates/topo/src/boolean/finish.rs`, `weld_pinches`), which
covers two holes meeting at a corner. The wedges never reach the weld:
every order refuses earlier, in the join, and which step raises each of
the three refusals was not traced. What is missing is building a face
whose holes pass through one vertex, in any order. The probe
was a scratch test over these members (`block` and an `on_frame` prism in
`crates/editor-core/tests/docm7_union_declare.rs`'s helpers), run in
every member order through `fixture::union_over`.

## Review tier

SINGLE, FULL: a vertex shared by several holes is new topology for the
join; one full review.

## Closed (2026-10-06, TANG)

Measured on 443f33b7. Under the D10 hold, the item's own fixture is
two problems, neither of them the face:

- **The 60° sectors are flush.** The lateral faces at 0° and 180°
  are coplanar and face the same way, and so are 60°/240° and
  120°/300°. Every order refuses `UndeclaredCoincidence` before the
  join, in topo and editor-core alike. The three refusals the item names came
  from a declared run, which the hold puts out of reach.
- **Upright prisms share one contact line.** With 50° sectors there
  is no coincidence left, only three solids touching along the
  vertical line through the vertex. The third prism's step refuses in
  `recl_edges`. With the plate last, the join desyncs on a pinch face. The
  three prisms alone refuse naming. That is contact, and it is filed and parked
  as `three-solids-touching-along-one-line-refuse-their-union`.

The face's own class shows once the prisms lean apart along axes
tilted out of their footprints, so they meet only at the vertex.
Folding three or more of them before the plate makes their shared
vertex pierce the top with one Out run per prism. Two fixes:

- **The ring struts hang in angular order.**
  `vtxfac::classify_vertex_on_face` refused three or more runs
  (`PierceRunsUnordered`; this also closes
  `ring-struts-of-three-or-more-runs-hang-in-run-order`). Now it hangs
  the ring struts clockwise about the pierced face's outward normal
  (`vtxfac::ring_order`), and each strut faces its germs from the next
  run's start germ.
- **The crossing vertex is a junction.** With the struts fixed,
  editor-core's union refused one step earlier, in naming
  (`names/emit_topo.rs`, `name_boolean_vertices`). The wedges' axes
  cross at the vertex, which has two A edges, one B edge and four seam
  lines, and no arm named that. The seam-junction arm now takes any
  vertex on two or more seam lines with any number of operand edges
  except exactly one. Its name is the set of lines, which straight
  lines make unique.

Rows (`crates/topo/tests/holes_meeting_at_a_vertex.rs` and
`crates/editor-core/tests/union_pinch_member_order.rs`):

- every member order of two, three and four wedges;
- three wedges on one side, which leave the top a reflex sector at
  the vertex;
- an L-shaped hole with its reflex corner at the vertex, beside two
  wedges.

Each order asserts its counts, the closed-form volume, tiers 3 and 3′,
one vertex at the meeting point, and the first order's body by
geometry. Topo also asserts that every face's corners at one point are
disjoint, and editor-core asserts `check_mesh`.
On 443f33b7:

- topo refuses `PierceRunsUnordered` in 6 of 24 orders for each
  three-hole fixture and 48 of 120 for four wedges;
- editor-core refuses `Naming(Emission)` at the wedges-only step;
- the two-wedge fixture builds there and is the control.

The bare tier-3 gap the corner check exposes is filed as
`work/restfront/tier-3-passes-a-face-whose-loop-crosses-itself-at-a-repeated-vertex.md`.
